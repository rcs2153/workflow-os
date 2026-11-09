//! Private, read-only recovery signing, separate from writer admission.
use super::{
    channel_denied, read_frame, remaining, socket_peer_uid, valid_app_assertion,
    valid_assertion_window, write_frame, Instant, UnixStream, EXCHANGE_TIMEOUT, MAX_REQUEST,
    MAX_RESPONSE,
};
use crate::provider_comment_transport::installation_exchange::reader::ReaderSignerRequest;
use crate::{SpecContentHash, Timestamp, WorkflowOsError};
use serde::{Deserialize, Serialize};

pub(crate) struct ReaderSignerPeerPolicy {
    pub(crate) core_uid: u32,
    pub(crate) reader_signer_uid: u32,
    pub(crate) writer_signer_uid: u32,
    pub(crate) harness_uids: [u32; 2],
}

impl ReaderSignerPeerPolicy {
    fn authenticate(
        &self,
        stream: &UnixStream,
        local: u32,
        peer: u32,
    ) -> Result<(), WorkflowOsError> {
        let [first, second] = self.harness_uids;
        let roles = [
            self.core_uid,
            self.reader_signer_uid,
            self.writer_signer_uid,
            first,
            second,
        ];
        if roles
            .iter()
            .enumerate()
            .any(|(i, uid)| *uid == 0 || roles[..i].contains(uid))
            || nix::unistd::geteuid().as_raw() != local
            || socket_peer_uid(stream)? != peer
        {
            return Err(channel_denied());
        }
        Ok(())
    }
}

pub(crate) struct RecoveryReaderSigner(UnixStream);
pub(crate) struct RecoveryReaderSignerReceiver(UnixStream);

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct ReaderSignerResponse {
    request: ReaderSignerRequest,
    issued_at: Timestamp,
    expires_at: Timestamp,
    assertion: String,
}

pub(crate) struct ReaderInstallationAssertion(ReaderSignerResponse);

impl ReaderInstallationAssertion {
    pub(in crate::provider_comment_transport) fn into_reader_parts(
        self,
        request: &ReaderSignerRequest,
    ) -> Option<(String, Timestamp)> {
        (&self.0.request == request && Timestamp::now_utc() < self.0.expires_at)
            .then_some((self.0.assertion, self.0.expires_at))
    }
}

impl RecoveryReaderSigner {
    pub(crate) fn authenticate(
        stream: UnixStream,
        policy: &ReaderSignerPeerPolicy,
    ) -> Result<Self, WorkflowOsError> {
        policy.authenticate(&stream, policy.core_uid, policy.reader_signer_uid)?;
        Ok(Self(stream))
    }

    #[cfg(test)]
    pub(crate) fn from_fixture_stream(stream: UnixStream) -> Self {
        Self(stream)
    }

    pub(crate) fn request(
        mut self,
        request: &ReaderSignerRequest,
    ) -> Result<ReaderInstallationAssertion, WorkflowOsError> {
        let started = Timestamp::now_utc();
        if started >= request.deadline {
            return Err(channel_denied());
        }
        let stop = configure(&self.0)?;
        let bytes = serde_json::to_vec(request).map_err(|_| channel_denied())?;
        write_frame(&mut self.0, &bytes, MAX_REQUEST, stop)?;
        let bytes = read_frame(&mut self.0, MAX_RESPONSE, stop)?;
        let response: ReaderSignerResponse =
            serde_json::from_slice(&bytes).map_err(|_| channel_denied())?;
        let finished = Timestamp::now_utc();
        if &response.request != request
            || response.issued_at < started
            || response.issued_at > finished
            || response.expires_at <= finished
            || !valid_assertion_window(response.issued_at, response.expires_at, request.deadline)
            || !valid_app_assertion(&response.assertion)
        {
            return Err(channel_denied());
        }
        Ok(ReaderInstallationAssertion(response))
    }
}

impl RecoveryReaderSignerReceiver {
    pub(crate) fn authenticate(
        stream: UnixStream,
        policy: &ReaderSignerPeerPolicy,
    ) -> Result<Self, WorkflowOsError> {
        policy.authenticate(&stream, policy.reader_signer_uid, policy.core_uid)?;
        Ok(Self(stream))
    }

    #[cfg(test)]
    pub(crate) fn from_fixture_stream(stream: UnixStream) -> Self {
        Self(stream)
    }

    pub(crate) fn serve_once(
        mut self,
        reviewed_access: &SpecContentHash,
        review_deadline: Timestamp,
        sign: impl FnOnce(&ReaderSignerRequest) -> Option<(String, Timestamp)>,
    ) -> Result<(), WorkflowOsError> {
        let stop = configure(&self.0)?;
        let bytes = read_frame(&mut self.0, MAX_REQUEST, stop)?;
        let request: ReaderSignerRequest =
            serde_json::from_slice(&bytes).map_err(|_| channel_denied())?;
        let before = Timestamp::now_utc();
        if &request.review != reviewed_access
            || before >= request.deadline
            || request.deadline > review_deadline
        {
            return Err(channel_denied());
        }
        remaining(stop)?;
        let (assertion, expires_at) = sign(&request).ok_or_else(channel_denied)?;
        let issued_at = Timestamp::now_utc();
        if issued_at < before
            || !valid_assertion_window(issued_at, expires_at, request.deadline)
            || !valid_app_assertion(&assertion)
        {
            return Err(channel_denied());
        }
        let response = ReaderSignerResponse {
            request,
            issued_at,
            expires_at,
            assertion,
        };
        let bytes = serde_json::to_vec(&response).map_err(|_| channel_denied())?;
        write_frame(&mut self.0, &bytes, MAX_RESPONSE, stop)
    }
}

fn configure(stream: &UnixStream) -> Result<Instant, WorkflowOsError> {
    let stop = Instant::now()
        .checked_add(EXCHANGE_TIMEOUT)
        .ok_or_else(channel_denied)?;
    stream.set_nonblocking(true).map_err(|_| channel_denied())?;
    Ok(stop)
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;

    const ASSERTION: &str = "reader.fixture.assertion";

    fn request() -> ReaderSignerRequest {
        ReaderSignerRequest::new(
            SpecContentHash::from_text("pending recovery"),
            SpecContentHash::from_text("reviewed reader"),
            Timestamp::from_offset_date_time(
                Timestamp::now_utc().as_offset_date_time() + time::Duration::seconds(20),
            ),
        )
    }

    #[test]
    fn installation_reader_signer_rejects_wrong_peers_and_role_aliases() {
        let core_uid = nix::unistd::geteuid().as_raw();
        let mut policy = ReaderSignerPeerPolicy {
            core_uid,
            reader_signer_uid: core_uid.wrapping_add(1),
            writer_signer_uid: core_uid.wrapping_add(2),
            harness_uids: [core_uid.wrapping_add(3), core_uid.wrapping_add(4)],
        };
        let (client, server) = UnixStream::pair().expect("socket pair");
        assert!(RecoveryReaderSigner::authenticate(client, &policy).is_err());
        assert!(RecoveryReaderSignerReceiver::authenticate(server, &policy).is_err());
        policy.reader_signer_uid = policy.writer_signer_uid;
        let (client, _server) = UnixStream::pair().expect("socket pair");
        assert!(policy.authenticate(&client, core_uid, core_uid).is_err());
    }

    #[test]
    fn installation_reader_signer_rejects_mismatched_review_deadline_and_writer_request() {
        let request = request();
        let other = SpecContentHash::from_text("another review");
        for (review, deadline) in [
            (&other, request.deadline),
            (&request.review, Timestamp::now_utc()),
        ] {
            let (client, peer) = UnixStream::pair().expect("socket pair");
            std::thread::scope(|threads| {
                let worker = threads.spawn(|| {
                    let mut called = false;
                    let result = RecoveryReaderSignerReceiver::from_fixture_stream(peer)
                        .serve_once(review, deadline, |_| {
                            called = true;
                            None
                        });
                    assert!(!called);
                    result
                });
                assert!(RecoveryReaderSigner::from_fixture_stream(client)
                    .request(&request)
                    .is_err());
                assert!(worker.join().expect("reader peer").is_err());
            });
        }
        let (mut client, peer) = UnixStream::pair().expect("socket pair");
        let mut value = serde_json::to_value(&request).expect("request JSON");
        value["purpose"] = serde_json::json!("dispatch/installation/fixture");
        let stop = configure(&client).expect("deadline");
        write_frame(
            &mut client,
            &serde_json::to_vec(&value).expect("request payload"),
            MAX_REQUEST,
            stop,
        )
        .expect("fixture frame");
        let mut called = false;
        assert!(RecoveryReaderSignerReceiver::from_fixture_stream(peer)
            .serve_once(&request.review, request.deadline, |_| {
                called = true;
                None
            })
            .is_err());
        assert!(!called);
    }

    #[test]
    fn installation_reader_signer_rejects_rebound_and_expired_responses() {
        let request = request();
        for drift in [true, false] {
            let (client, mut peer) = UnixStream::pair().expect("socket pair");
            std::thread::scope(|threads| {
                let worker = threads.spawn(|| {
                    let stop = configure(&peer).expect("deadline");
                    let mut received: ReaderSignerRequest = serde_json::from_slice(
                        &read_frame(&mut peer, MAX_REQUEST, stop).expect("request frame"),
                    )
                    .expect("reader request");
                    let issued_at = Timestamp::now_utc();
                    if drift {
                        received.recovery = SpecContentHash::from_text("other recovery");
                    }
                    let response = ReaderSignerResponse {
                        request: received,
                        issued_at,
                        expires_at: if drift { request.deadline } else { issued_at },
                        assertion: ASSERTION.to_owned(),
                    };
                    write_frame(
                        &mut peer,
                        &serde_json::to_vec(&response).expect("response"),
                        MAX_RESPONSE,
                        stop,
                    )
                    .expect("response frame");
                });
                assert!(RecoveryReaderSigner::from_fixture_stream(client)
                    .request(&request)
                    .is_err());
                worker.join().expect("reader peer");
            });
        }
    }
}
