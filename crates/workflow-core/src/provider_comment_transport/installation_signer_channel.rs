use std::io::{ErrorKind, Read, Write};
use std::net::Shutdown;
use std::os::fd::AsFd;
use std::os::unix::net::UnixStream;
use std::time::{Duration, Instant};

use nix::poll::{poll, PollFd, PollFlags, PollTimeout};
use nix::sys::socket::getsockopt;
use serde::{Deserialize, Serialize};

use super::installation_exchange::valid_app_assertion;
use crate::{SpecContentHash, Timestamp, WorkflowOsError};

pub(crate) mod decision;
use decision::{InstallationSignerDecisionChannel, SignerDecisionPermit};

const VERSION: u8 = 1;
const MAX_REQUEST: usize = 1024;
const MAX_RESPONSE: usize = 10_240;
const EXCHANGE_TIMEOUT: Duration = Duration::from_secs(5);
const MAX_ASSERTION_LIFETIME: time::Duration = time::Duration::minutes(2);

// A trusted launcher owns the connected descriptor and these configured UIDs.
pub(crate) struct SignerPeerPolicy {
    pub(crate) signer_uid: u32,
    pub(crate) exchange_uid: u32,
    pub(crate) harness_uids: [u32; 2],
}

pub(crate) struct InstallationSignerChannel {
    stream: UnixStream,
}

pub(crate) struct InstallationSignerReceiver {
    stream: UnixStream,
}

#[derive(Serialize)]
pub(crate) struct InstallationSignerRequest<'a> {
    version: u8,
    operation: &'a str,
    claim: &'a SpecContentHash,
    scope: &'a SpecContentHash,
    deadline: Timestamp,
}

impl<'a> InstallationSignerRequest<'a> {
    pub(crate) const fn new(
        operation: &'a str,
        claim: &'a SpecContentHash,
        scope: &'a SpecContentHash,
        deadline: Timestamp,
    ) -> Self {
        Self {
            version: VERSION,
            operation,
            claim,
            scope,
            deadline,
        }
    }
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReceivedInstallationSignerRequest {
    version: u8,
    pub(crate) operation: String,
    pub(crate) claim: SpecContentHash,
    pub(crate) scope: SpecContentHash,
    pub(crate) deadline: Timestamp,
}

impl ReceivedInstallationSignerRequest {
    pub(crate) const fn has_current_version(&self) -> bool {
        self.version == VERSION
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct SignerResponse {
    version: u8,
    claim: SpecContentHash,
    scope: SpecContentHash,
    issued_at: Timestamp,
    expires_at: Timestamp,
    assertion: String,
}

// Neither serializable nor printable. Only the fixed-origin exchange reads it.
pub(crate) struct InstallationAppAssertion {
    assertion: String,
    issued_at: Timestamp,
    expires_at: Timestamp,
}

impl InstallationAppAssertion {
    pub(crate) const fn issued_at(&self) -> Timestamp {
        self.issued_at
    }

    pub(super) fn into_parts(self) -> (String, Timestamp) {
        (self.assertion, self.expires_at)
    }
}

impl InstallationSignerChannel {
    #[cfg(test)]
    pub(crate) fn from_fixture_stream(stream: UnixStream) -> Self {
        Self { stream }
    }

    pub(crate) fn authenticate(
        stream: UnixStream,
        policy: &SignerPeerPolicy,
    ) -> Result<Self, WorkflowOsError> {
        authenticate_peer(&stream, policy, policy.exchange_uid, policy.signer_uid)?;
        Ok(Self { stream })
    }

    pub(crate) fn request(
        self,
        request: &InstallationSignerRequest<'_>,
    ) -> Result<InstallationAppAssertion, WorkflowOsError> {
        exchange(self.stream, request, Timestamp::now_utc, EXCHANGE_TIMEOUT)
    }
}

impl InstallationSignerReceiver {
    #[cfg(test)]
    pub(crate) fn from_fixture_stream(stream: UnixStream) -> Self {
        Self { stream }
    }

    pub(crate) fn authenticate(
        stream: UnixStream,
        policy: &SignerPeerPolicy,
    ) -> Result<Self, WorkflowOsError> {
        authenticate_peer(&stream, policy, policy.signer_uid, policy.exchange_uid)?;
        Ok(Self { stream })
    }

    pub(crate) fn serve_once_via_core(
        mut self,
        decision: InstallationSignerDecisionChannel,
        sign: impl FnOnce(SignerDecisionPermit) -> Option<(String, Timestamp)>,
    ) -> Result<(), WorkflowOsError> {
        let stop = Instant::now()
            .checked_add(EXCHANGE_TIMEOUT)
            .ok_or_else(channel_denied)?;
        self.stream
            .set_nonblocking(true)
            .map_err(|_| channel_denied())?;
        let payload = read_frame(&mut self.stream, MAX_REQUEST, stop)?;
        let request: ReceivedInstallationSignerRequest =
            serde_json::from_slice(&payload).map_err(|_| channel_denied())?;
        if !request.has_current_version()
            || !request.operation.starts_with("dispatch/installation/")
            || Timestamp::now_utc() >= request.deadline
        {
            return Err(channel_denied());
        }
        let permit = decision.request(&request, stop)?;
        let admitted_at = permit.admitted_at();
        let before_sign = Timestamp::now_utc();
        remaining(stop)?;
        if before_sign < admitted_at || before_sign >= request.deadline {
            return Err(channel_denied());
        }
        let (assertion, expires_at) = sign(permit).ok_or_else(channel_denied)?;
        let issued_at = Timestamp::now_utc();
        if issued_at < before_sign
            || !valid_assertion_window(issued_at, expires_at, request.deadline)
            || !valid_app_assertion(&assertion)
        {
            return Err(channel_denied());
        }
        let response = SignerResponse {
            version: VERSION,
            claim: request.claim,
            scope: request.scope,
            issued_at,
            expires_at,
            assertion,
        };
        let payload = serde_json::to_vec(&response).map_err(|_| channel_denied())?;
        write_frame(&mut self.stream, &payload, MAX_RESPONSE, stop)
    }
}

fn authenticate_peer(
    stream: &UnixStream,
    policy: &SignerPeerPolicy,
    local_uid: u32,
    peer_uid: u32,
) -> Result<(), WorkflowOsError> {
    if policy.signer_uid == 0
        || policy.exchange_uid == 0
        || policy.signer_uid == policy.exchange_uid
        || policy
            .harness_uids
            .iter()
            .any(|uid| *uid == 0 || *uid == policy.signer_uid || *uid == policy.exchange_uid)
        || nix::unistd::geteuid().as_raw() != local_uid
        || socket_peer_uid(stream)? != peer_uid
    {
        return Err(channel_denied());
    }
    Ok(())
}

fn socket_peer_uid(stream: &UnixStream) -> Result<u32, WorkflowOsError> {
    #[cfg(target_os = "linux")]
    let cred = getsockopt(stream, nix::sys::socket::sockopt::PeerCredentials);
    #[cfg(target_os = "macos")]
    let cred = getsockopt(stream, nix::sys::socket::sockopt::LocalPeerCred);
    cred.map(|cred| cred.uid()).map_err(|_| channel_denied())
}

fn exchange(
    mut stream: UnixStream,
    request: &InstallationSignerRequest<'_>,
    mut now: impl FnMut() -> Timestamp,
    timeout: Duration,
) -> Result<InstallationAppAssertion, WorkflowOsError> {
    let started = now();
    if started >= request.deadline {
        return Err(channel_denied());
    }
    let stop = Instant::now()
        .checked_add(timeout)
        .ok_or_else(channel_denied)?;
    stream.set_nonblocking(true).map_err(|_| channel_denied())?;
    let payload = serde_json::to_vec(request).map_err(|_| channel_denied())?;
    write_frame(&mut stream, &payload, MAX_REQUEST, stop)?;
    let payload = read_frame(&mut stream, MAX_RESPONSE, stop)?;
    let response: SignerResponse =
        serde_json::from_slice(&payload).map_err(|_| channel_denied())?;
    let finished = now();
    if response.version != VERSION
        || &response.claim != request.claim
        || &response.scope != request.scope
        || response.issued_at < started
        || response.issued_at > finished
        || response.expires_at <= finished
        || !valid_assertion_window(response.issued_at, response.expires_at, request.deadline)
        || !valid_app_assertion(&response.assertion)
    {
        return Err(channel_denied());
    }
    Ok(InstallationAppAssertion {
        assertion: response.assertion,
        issued_at: response.issued_at,
        expires_at: response.expires_at,
    })
}

fn valid_assertion_window(issued: Timestamp, expires: Timestamp, deadline: Timestamp) -> bool {
    issued < expires
        && expires <= deadline
        && expires.as_offset_date_time() - issued.as_offset_date_time() <= MAX_ASSERTION_LIFETIME
}

fn write_frame(
    stream: &mut UnixStream,
    payload: &[u8],
    limit: usize,
    stop: Instant,
) -> Result<(), WorkflowOsError> {
    if !(2..=limit).contains(&payload.len()) {
        return Err(channel_denied());
    }
    let size = u32::try_from(payload.len()).map_err(|_| channel_denied())?;
    write_bounded(stream, &size.to_be_bytes(), stop)?;
    write_bounded(stream, payload, stop)?;
    stream
        .shutdown(Shutdown::Write)
        .map_err(|_| channel_denied())
}

fn read_frame(
    stream: &mut UnixStream,
    limit: usize,
    stop: Instant,
) -> Result<Vec<u8>, WorkflowOsError> {
    let mut size = [0; 4];
    read_bounded(stream, &mut size, stop)?;
    let size = usize::try_from(u32::from_be_bytes(size)).map_err(|_| channel_denied())?;
    if !(2..=limit).contains(&size) {
        return Err(channel_denied());
    }
    let mut payload = vec![0; size];
    read_bounded(stream, &mut payload, stop)?;
    if read_once(stream, &mut [0], stop)? != 0 {
        return Err(channel_denied());
    }
    remaining(stop)?;
    Ok(payload)
}

fn remaining(stop: Instant) -> Result<Duration, WorkflowOsError> {
    let time = stop.saturating_duration_since(Instant::now());
    if time.is_zero() {
        Err(channel_denied())
    } else {
        Ok(time)
    }
}

fn wait_ready(stream: &UnixStream, flags: PollFlags, stop: Instant) -> Result<(), WorkflowOsError> {
    let timeout = PollTimeout::try_from(remaining(stop)?.as_millis().saturating_add(1))
        .map_err(|_| channel_denied())?;
    let mut fds = [PollFd::new(stream.as_fd(), flags)];
    if poll(&mut fds, timeout).map_err(|_| channel_denied())? == 0 {
        return Err(channel_denied());
    }
    remaining(stop)?;
    Ok(())
}

fn write_bounded(
    stream: &mut UnixStream,
    mut bytes: &[u8],
    stop: Instant,
) -> Result<(), WorkflowOsError> {
    while !bytes.is_empty() {
        wait_ready(stream, PollFlags::POLLOUT, stop)?;
        match stream.write(bytes) {
            Ok(n) if n > 0 => bytes = &bytes[n..],
            Err(error) if error.kind() == ErrorKind::WouldBlock => {}
            _ => return Err(channel_denied()),
        }
    }
    Ok(())
}

fn read_bounded(
    stream: &mut UnixStream,
    mut bytes: &mut [u8],
    stop: Instant,
) -> Result<(), WorkflowOsError> {
    while !bytes.is_empty() {
        let read = read_once(stream, bytes, stop)?;
        if read == 0 {
            return Err(channel_denied());
        }
        bytes = &mut bytes[read..];
    }
    Ok(())
}

fn read_once(
    stream: &mut UnixStream,
    bytes: &mut [u8],
    stop: Instant,
) -> Result<usize, WorkflowOsError> {
    loop {
        wait_ready(stream, PollFlags::POLLIN, stop)?;
        match stream.read(bytes) {
            Ok(n) => return Ok(n),
            Err(error) if error.kind() == ErrorKind::WouldBlock => {}
            Err(_) => return Err(channel_denied()),
        }
    }
}

fn channel_denied() -> WorkflowOsError {
    WorkflowOsError::security(
        "installation_signer.channel_denied",
        "installation signer channel could not be verified",
    )
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use serde_json::{json, Value};

    const ASSERTION: &str = "fixture-header.fixture-payload.fixture-signature";

    fn at(seconds: i64) -> Timestamp {
        Timestamp::from_offset_date_time(
            time::OffsetDateTime::from_unix_timestamp(seconds).expect("timestamp"),
        )
    }

    fn fixture_exchange(
        response: impl FnOnce(Value) -> Vec<u8> + Send,
    ) -> Result<InstallationAppAssertion, WorkflowOsError> {
        let (client, peer) = UnixStream::pair().expect("disposable socket pair");
        let claim = SpecContentHash::from_text("committed claim");
        let scope = SpecContentHash::from_text("reviewed scope");
        let request = InstallationSignerRequest::new(
            "dispatch/installation/fixture",
            &claim,
            &scope,
            at(200),
        );
        std::thread::scope(|threads| {
            let worker = threads.spawn(|| {
                let mut peer = peer;
                let mut request = Vec::new();
                peer.read_to_end(&mut request).expect("one half-closed request");
                let (size, payload) = request.split_at(4);
                assert_eq!(u32::from_be_bytes(size.try_into().expect("length")) as usize, payload.len());
                let actual: Value = serde_json::from_slice(payload).expect("request frame");
                assert_eq!(actual, json!({"version":1,"operation":"dispatch/installation/fixture","claim":claim,"scope":scope,"deadline":at(200)}));
                let bytes = response(json!({"version":1,"claim":claim,"scope":scope,"issued_at":at(101),"expires_at":at(150),"assertion":ASSERTION}));
                peer.write_all(&bytes).expect("fixture reply");
            });
            let mut samples = [at(100), at(102)].into_iter();
            let result = exchange(
                client,
                &request,
                || samples.next().expect("clock sample"),
                Duration::from_secs(2),
            );
            worker.join().expect("fixture peer");
            result
        })
    }

    fn frame(response: &Value) -> Vec<u8> {
        let payload = serde_json::to_vec(response).expect("fixture JSON");
        let mut frame = u32::try_from(payload.len())
            .expect("bounded frame")
            .to_be_bytes()
            .to_vec();
        frame.extend(payload);
        frame
    }

    #[test]
    fn installation_signer_rejects_wrong_os_peer_before_sending() {
        let (client, mut peer) = UnixStream::pair().expect("socket pair");
        let uid = nix::unistd::geteuid().as_raw();
        assert_eq!(socket_peer_uid(&client).expect("kernel peer UID"), uid);
        let other_uid = if uid == 1 { 2 } else { 1 };
        let policy = SignerPeerPolicy {
            signer_uid: other_uid,
            exchange_uid: uid,
            harness_uids: [3, 4],
        };
        assert!(InstallationSignerChannel::authenticate(client, &policy).is_err());
        assert_eq!(
            peer.read(&mut [0])
                .expect("no request before peer verification"),
            0
        );
        let (client, _) = UnixStream::pair().expect("socket pair");
        let shared = SignerPeerPolicy {
            signer_uid: uid,
            exchange_uid: uid,
            harness_uids: [uid, uid],
        };
        assert!(InstallationSignerChannel::authenticate(client, &shared).is_err());
    }

    #[test]
    fn installation_signer_receiver_rejects_wrong_os_peer_and_malformed_frames() {
        let (peer, receiver) = UnixStream::pair().expect("socket pair");
        let uid = nix::unistd::geteuid().as_raw();
        let policy = SignerPeerPolicy {
            signer_uid: uid,
            exchange_uid: if uid == 1 { 2 } else { 1 },
            harness_uids: [3, 4],
        };
        assert!(InstallationSignerReceiver::authenticate(receiver, &policy).is_err());
        drop(peer);

        for (size, payload) in [(u32::MAX, &[][..]), (2, b"{}x"), (4, b"{}"), (2, b"{}")] {
            let (mut peer, receiver) = UnixStream::pair().expect("socket pair");
            peer.write_all(&size.to_be_bytes()).expect("size");
            peer.write_all(payload).expect("payload");
            peer.shutdown(Shutdown::Write).expect("single request");
            let (decision_client, mut decision_peer) = UnixStream::pair().expect("decision pair");
            assert!(InstallationSignerReceiver::from_fixture_stream(receiver)
                .serve_once_via_core(
                    InstallationSignerDecisionChannel::from_fixture_stream(decision_client),
                    |_| None,
                )
                .is_err());
            assert_eq!(decision_peer.read(&mut [0]).expect("decision EOF"), 0);
        }
    }

    #[test]
    fn installation_signer_accepts_only_exact_fresh_bounded_response() {
        let assertion = fixture_exchange(|reply| frame(&reply)).expect("matching response");
        assert_eq!(assertion.into_parts(), (ASSERTION.to_owned(), at(150)));
        for (field, value) in [
            ("claim", json!(SpecContentHash::from_text("another claim"))),
            ("scope", json!(SpecContentHash::from_text("another scope"))),
            ("version", json!(2)),
            ("issued_at", json!(at(99))),
            ("issued_at", json!(at(103))),
            ("expires_at", json!(at(102))),
            ("expires_at", json!(at(201))),
            ("assertion", json!("malformed fixture")),
            ("extra", json!(true)),
        ] {
            let result = fixture_exchange(move |mut reply| {
                reply[field] = value;
                frame(&reply)
            });
            let error = result.err().expect("invalid reply must be denied");
            assert_eq!(error.code(), "installation_signer.channel_denied");
            assert!(!error.to_string().contains(ASSERTION));
        }
    }

    #[test]
    fn installation_signer_rejects_truncated_overlong_or_second_frames() {
        assert!(fixture_exchange(|_| Vec::new()).is_err());
        assert!(fixture_exchange(|_| u32::MAX.to_be_bytes().to_vec()).is_err());
        assert!(fixture_exchange(|reply| {
            let mut bytes = frame(&reply);
            bytes.pop();
            bytes
        })
        .is_err());
        assert!(fixture_exchange(|reply| {
            let mut bytes = frame(&reply);
            bytes.push(b'X');
            bytes
        })
        .is_err());
    }

    #[test]
    fn installation_signer_deadline_does_not_retry_a_silent_peer() {
        let (client, mut peer) = UnixStream::pair().expect("socket pair");
        let claim = SpecContentHash::from_text("claim");
        let scope = SpecContentHash::from_text("scope");
        let request = InstallationSignerRequest::new(
            "dispatch/installation/fixture",
            &claim,
            &scope,
            at(200),
        );
        assert!(exchange(client, &request, || at(100), Duration::from_millis(50)).is_err());
        let mut data = Vec::new();
        peer.read_to_end(&mut data).expect("only one request frame");
        let (size, payload) = data.split_at(4);
        assert_eq!(
            u32::from_be_bytes(size.try_into().expect("length")) as usize,
            payload.len()
        );
    }
}
