use std::os::unix::net::UnixStream;
use std::time::Instant;

use serde::{Deserialize, Serialize};

use super::{
    authenticate_peer, channel_denied, read_frame, remaining, write_frame,
    ReceivedInstallationSignerRequest, SignerPeerPolicy, EXCHANGE_TIMEOUT,
};
use crate::sqlite_state::InstallationSignerAdmission;
use crate::{Timestamp, WorkflowOsError};

const DECISION_VERSION: u8 = 1;
const MAX_DECISION_FRAME: usize = 2048;
const CHALLENGE_SIZE: usize = 32;

// The trusted launcher supplies a second connection, not the exchange socket.
pub(crate) struct InstallationSignerDecisionChannel(UnixStream);

pub(crate) struct InstallationSignerDecisionReceiver(UnixStream);

pub(crate) struct SignerDecisionPermit(Timestamp);

impl SignerDecisionPermit {
    pub(super) const fn admitted_at(&self) -> Timestamp {
        self.0
    }
}

#[derive(Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct DecisionQuery {
    version: u8,
    challenge: [u8; CHALLENGE_SIZE],
    request: ReceivedInstallationSignerRequest,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct DecisionReply {
    query: DecisionQuery,
    admitted_at: Timestamp,
}

impl InstallationSignerDecisionChannel {
    pub(crate) fn authenticate(
        stream: UnixStream,
        policy: &SignerPeerPolicy,
    ) -> Result<Self, WorkflowOsError> {
        authenticate_peer(&stream, policy, policy.signer_uid, policy.exchange_uid)?;
        Ok(Self(stream))
    }

    #[cfg(test)]
    pub(crate) fn from_fixture_stream(stream: UnixStream) -> Self {
        Self(stream)
    }

    pub(super) fn request(
        mut self,
        request: &ReceivedInstallationSignerRequest,
        stop: Instant,
    ) -> Result<SignerDecisionPermit, WorkflowOsError> {
        let mut challenge = [0; CHALLENGE_SIZE];
        getrandom::getrandom(&mut challenge).map_err(|_| channel_denied())?;
        if challenge == [0; CHALLENGE_SIZE] {
            return Err(channel_denied());
        }
        let requested_at = Timestamp::now_utc();
        if requested_at >= request.deadline {
            return Err(channel_denied());
        }
        let query = DecisionQuery {
            version: DECISION_VERSION,
            challenge,
            request: request.clone(),
        };
        self.0.set_nonblocking(true).map_err(|_| channel_denied())?;
        let payload = serde_json::to_vec(&query).map_err(|_| channel_denied())?;
        write_frame(&mut self.0, &payload, MAX_DECISION_FRAME, stop)?;
        let reply: DecisionReply =
            serde_json::from_slice(&read_frame(&mut self.0, MAX_DECISION_FRAME, stop)?)
                .map_err(|_| channel_denied())?;
        let received_at = Timestamp::now_utc();
        if reply.query != query
            || reply.admitted_at < requested_at
            || reply.admitted_at > received_at
            || received_at >= request.deadline
        {
            return Err(channel_denied());
        }
        Ok(SignerDecisionPermit(reply.admitted_at))
    }
}

impl InstallationSignerDecisionReceiver {
    pub(crate) fn authenticate(
        stream: UnixStream,
        policy: &SignerPeerPolicy,
    ) -> Result<Self, WorkflowOsError> {
        authenticate_peer(&stream, policy, policy.exchange_uid, policy.signer_uid)?;
        Ok(Self(stream))
    }

    #[cfg(test)]
    pub(crate) fn from_fixture_stream(stream: UnixStream) -> Self {
        Self(stream)
    }

    pub(crate) fn serve_once(
        mut self,
        admit: impl FnOnce(
            &ReceivedInstallationSignerRequest,
        ) -> Result<InstallationSignerAdmission, WorkflowOsError>,
    ) -> Result<(), WorkflowOsError> {
        let stop = Instant::now()
            .checked_add(EXCHANGE_TIMEOUT)
            .ok_or_else(channel_denied)?;
        self.0.set_nonblocking(true).map_err(|_| channel_denied())?;
        let query: DecisionQuery =
            serde_json::from_slice(&read_frame(&mut self.0, MAX_DECISION_FRAME, stop)?)
                .map_err(|_| channel_denied())?;
        let received_at = Timestamp::now_utc();
        if query.version != DECISION_VERSION
            || query.challenge == [0; CHALLENGE_SIZE]
            || !query.request.has_current_version()
            || !query
                .request
                .operation
                .starts_with("dispatch/installation/")
            || received_at >= query.request.deadline
        {
            return Err(channel_denied());
        }
        remaining(stop)?;
        let permit = admit(&query.request).map_err(|_| channel_denied())?;
        let admitted_at = permit
            .admitted_at(&query.request)
            .ok_or_else(channel_denied)?;
        let now = Timestamp::now_utc();
        if admitted_at < received_at || admitted_at > now || now >= query.request.deadline {
            return Err(channel_denied());
        }
        let reply = DecisionReply { query, admitted_at };
        let payload = serde_json::to_vec(&reply).map_err(|_| channel_denied())?;
        write_frame(&mut self.0, &payload, MAX_DECISION_FRAME, stop)
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use crate::SpecContentHash;
    use serde_json::json;
    use std::io::Read;

    fn stop() -> Instant {
        Instant::now()
            .checked_add(EXCHANGE_TIMEOUT)
            .expect("deadline")
    }

    fn request() -> ReceivedInstallationSignerRequest {
        serde_json::from_value(json!({
            "version": 1,
            "operation": "dispatch/installation/fixture",
            "claim": SpecContentHash::from_text("fixture claim"),
            "scope": SpecContentHash::from_text("fixture scope"),
            "deadline": Timestamp::from_offset_date_time(
                Timestamp::now_utc().as_offset_date_time() + time::Duration::minutes(1),
            ),
        }))
        .expect("wire request")
    }

    fn try_reply(
        request: &ReceivedInstallationSignerRequest,
        reply: impl FnOnce(DecisionReply) -> Option<Vec<u8>> + Send,
    ) -> Result<SignerDecisionPermit, WorkflowOsError> {
        let (signer, mut core) = UnixStream::pair().expect("decision pair");
        std::thread::scope(|threads| {
            let worker = threads.spawn(move || {
                core.set_nonblocking(true).expect("nonblocking");
                let stop = stop();
                let frame = read_frame(&mut core, MAX_DECISION_FRAME, stop).expect("query frame");
                let query = serde_json::from_slice(&frame).expect("query");
                if let Some(payload) = reply(DecisionReply {
                    query,
                    admitted_at: Timestamp::now_utc(),
                }) {
                    write_frame(&mut core, &payload, MAX_DECISION_FRAME, stop).expect("reply");
                }
            });
            let result = InstallationSignerDecisionChannel::from_fixture_stream(signer)
                .request(request, stop());
            worker.join().expect("decision worker");
            result
        })
    }

    #[test]
    fn installation_decision_rejects_untrusted_peers_at_both_ends() {
        let uid = nix::unistd::geteuid().as_raw();
        let other = if uid == 1 { 2 } else { 1 };
        for is_signer in [true, false] {
            let (endpoint, mut peer) = UnixStream::pair().expect("decision pair");
            let (signer_uid, exchange_uid) = if is_signer {
                (uid, other)
            } else {
                (other, uid)
            };
            let policy = SignerPeerPolicy {
                signer_uid,
                exchange_uid,
                harness_uids: [3, 4],
            };
            if is_signer {
                assert!(
                    InstallationSignerDecisionChannel::authenticate(endpoint, &policy).is_err()
                );
            } else {
                assert!(
                    InstallationSignerDecisionReceiver::authenticate(endpoint, &policy).is_err()
                );
            }
            assert_eq!(peer.read(&mut [0]).expect("peer closed without data"), 0);
        }
    }

    #[test]
    fn installation_decision_rejects_mismatch_staleness_and_replay() {
        let request = request();
        for field in ["challenge", "claim", "scope", "admitted_at"] {
            assert!(
                try_reply(&request, |reply| {
                    let mut payload = serde_json::to_value(reply).expect("reply JSON");
                    let changed = SpecContentHash::from_text("different value");
                    match field {
                        "challenge" => {
                            payload["query"]["challenge"][0] = json!(
                                payload["query"]["challenge"][0]
                                    .as_u64()
                                    .expect("challenge byte")
                                    ^ 1
                            );
                        }
                        "admitted_at" => {
                            payload["admitted_at"] = json!(Timestamp::from_offset_date_time(
                                Timestamp::now_utc().as_offset_date_time()
                                    - time::Duration::minutes(1),
                            ));
                        }
                        _ => payload["query"]["request"][field] = json!(changed),
                    }
                    Some(serde_json::to_vec(&payload).expect("altered reply"))
                })
                .is_err(),
                "{field} must not produce a permit"
            );
        }

        let mut first = Vec::new();
        assert!(try_reply(&request, |reply| {
            first = serde_json::to_vec(&reply).expect("fresh reply");
            Some(first.clone())
        })
        .is_ok());
        assert!(try_reply(&request, |mut reply| {
            let old: DecisionReply = serde_json::from_slice(&first).expect("captured reply");
            // Refresh the timestamp to isolate the challenge check from staleness.
            reply.query = old.query;
            Some(serde_json::to_vec(&reply).expect("replayed query"))
        })
        .is_err());
        assert!(try_reply(&request, |_| None).is_err());
    }

    #[test]
    fn installation_decision_core_refusal_never_produces_a_permit() {
        let (signer, core) = UnixStream::pair().expect("decision pair");
        std::thread::scope(|threads| {
            let worker = threads.spawn(|| {
                InstallationSignerDecisionReceiver::from_fixture_stream(core)
                    .serve_once(|_| Err(channel_denied()))
            });
            let result = InstallationSignerDecisionChannel::from_fixture_stream(signer)
                .request(&request(), stop());
            assert!(worker.join().expect("decision worker").is_err());
            assert!(result.is_err());
        });
    }
}
