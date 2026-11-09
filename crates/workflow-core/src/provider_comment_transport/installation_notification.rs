//! Offline verification only. Receiver trust must come from a reviewed host.
#![allow(dead_code)] // No observer registration or delivery route is enabled.

use std::{borrow::Cow, io::Read, num::NonZeroU64};

use ring::signature::KeyPair;
use ring::{hmac, signature};
use serde::{Deserialize, Serialize};

use crate::authorized_execution_continuity_state::internal::TrustedTimeObservation;
use crate::provider_comment_observation::{
    encode_marked_comment, MARKER_NAMESPACE, MAX_RAW_BODY_BYTES,
};
use crate::{GitHubPullRequestCommentTarget, SpecContentHash, Timestamp};

#[cfg(any(target_os = "linux", target_os = "macos"))]
mod custody;
#[cfg(any(target_os = "linux", target_os = "macos"))]
mod handoff;

const MAX_NOTIFICATION_BYTES: usize = 256 * 1024;
const RECEIPT_DOMAIN: &str = "workflow-os/installation-comment-receiver-receipt/v1";

/// Supplied by trusted Core composition, not decoded from a request.
#[derive(Clone, Copy)]
pub(crate) struct ReviewedCreationReceiver<'a> {
    pub(crate) installation: NonZeroU64,
    pub(crate) repository: NonZeroU64,
    pub(crate) writer_scope: &'a SpecContentHash,
    pub(crate) github_hmac: &'a [u8; 32],
    pub(crate) attestation_key: &'a [u8; 32],
    pub(crate) webhook_verifier_generation: Option<&'a SpecContentHash>,
}

#[derive(Clone, Copy)]
pub(crate) struct PreparedCreation<'a> {
    pub(crate) reservation: &'a SpecContentHash,
    pub(crate) writer_scope: &'a SpecContentHash,
    pub(crate) target: &'a GitHubPullRequestCommentTarget,
    pub(crate) repository: NonZeroU64,
    pub(crate) writer: NonZeroU64,
    pub(crate) approved_body: &'a str,
    pub(crate) marker_commitment: &'a SpecContentHash,
    pub(crate) body_commitment: &'a SpecContentHash,
    pub(crate) preparation_time: &'a TrustedTimeObservation,
    pub(crate) checked_at: &'a TrustedTimeObservation,
    pub(crate) deadline: Timestamp,
    pub(crate) observer_profile: Option<&'a super::InstallationObserverProfile>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct CreationReceipt {
    pub(crate) receiving_installation: NonZeroU64,
    pub(crate) repository: NonZeroU64,
    pub(crate) payload_digest: SpecContentHash,
    pub(crate) earliest: TrustedTimeObservation,
    pub(crate) latest: TrustedTimeObservation,
}

impl CreationReceipt {
    pub(crate) fn signing_bytes(&self) -> Option<Vec<u8>> {
        serde_json::to_vec(&(RECEIPT_DOMAIN, self)).ok()
    }
}

#[derive(Clone, Copy)]
pub(crate) struct CreationNotification<'a> {
    pub(crate) body: &'a [u8],
    pub(crate) github_signature: &'a str,
    pub(crate) receipt: &'a CreationReceipt,
    pub(crate) receipt_signature: &'a [u8; 64],
}

/// Private receiver-side intake; the trusted host owns the clock and custody.
pub(crate) struct CreationReceiptAttester<'a> {
    profile: &'a super::InstallationObserverProfile,
    repository: NonZeroU64,
    target: &'a GitHubPullRequestCommentTarget,
    deadline: Timestamp,
    verifier: hmac::Key,
    signer: &'a signature::Ed25519KeyPair,
}

pub(crate) struct AttestedCreationNotification<'a> {
    body: Vec<u8>,
    github_signature: &'a str,
    receipt: CreationReceipt,
    receipt_signature: [u8; 64],
}

impl AttestedCreationNotification<'_> {
    pub(crate) fn notification(&self) -> CreationNotification<'_> {
        CreationNotification {
            body: &self.body,
            github_signature: self.github_signature,
            receipt: &self.receipt,
            receipt_signature: &self.receipt_signature,
        }
    }
}

impl<'a> CreationReceiptAttester<'a> {
    pub(crate) fn new(
        profile: &'a super::InstallationObserverProfile,
        repository: NonZeroU64,
        target: &'a GitHubPullRequestCommentTarget,
        deadline: Timestamp,
        webhook_material: &[u8; 32],
        signer: &'a signature::Ed25519KeyPair,
    ) -> Option<Self> {
        target.validate().ok()?;
        let verifier = hmac::Key::new(hmac::HMAC_SHA256, webhook_material);
        if signer.public_key().as_ref() != profile.attestation_key
            || !profile.matches_webhook_material(&verifier)
        {
            return None;
        }
        Some(Self {
            profile,
            repository,
            target,
            deadline,
            verifier,
            signer,
        })
    }

    pub(crate) fn capture<'header>(
        &self,
        body: impl Read,
        github_signature: &'header str,
        mut observe: impl FnMut() -> Option<TrustedTimeObservation>,
    ) -> Option<AttestedCreationNotification<'header>> {
        let mac = decode_signature(github_signature)?;
        let earliest = observe()?;
        if earliest.observed_at() >= self.deadline {
            return None;
        }
        let mut bytes = Vec::new();
        body.take(MAX_NOTIFICATION_BYTES as u64 + 1)
            .read_to_end(&mut bytes)
            .ok()?;
        let latest = observe()?;
        if bytes.len() > MAX_NOTIFICATION_BYTES
            || !same_time_domain(&earliest, &latest)
            || latest.observed_at() < earliest.observed_at()
            || latest.observed_at() >= self.deadline
        {
            return None;
        }
        hmac::verify(&self.verifier, &bytes, &mac).ok()?;
        let event: Notification<'_> = serde_json::from_slice(&bytes).ok()?;
        if !event.matches_scope(
            self.profile.receiver_installation,
            self.repository,
            self.target,
        ) {
            return None;
        }
        let receipt = CreationReceipt {
            receiving_installation: self.profile.receiver_installation,
            repository: self.repository,
            payload_digest: SpecContentHash::from_bytes(&bytes),
            earliest,
            latest,
        };
        let receipt_signature = self
            .signer
            .sign(&receipt.signing_bytes()?)
            .as_ref()
            .try_into()
            .ok()?;
        Some(AttestedCreationNotification {
            body: bytes,
            github_signature,
            receipt,
            receipt_signature,
        })
    }
}

/// A current candidate only. No completeness, outcome, or dispatch authority.
#[derive(Debug, Eq, PartialEq)]
pub(crate) struct CreationCandidate {
    reservation: SpecContentHash,
    payload_digest: SpecContentHash,
    comment_id: NonZeroU64,
    body_commitment: SpecContentHash,
    marker_commitment: SpecContentHash,
    receipt_commitment: SpecContentHash,
    received_latest: Timestamp,
    observer_profile: Option<SpecContentHash>,
}

impl CreationCandidate {
    pub(crate) const fn received_latest(&self) -> Timestamp {
        self.received_latest
    }
    pub(crate) fn commitments(&self) -> (&SpecContentHash, &SpecContentHash) {
        (&self.payload_digest, &self.receipt_commitment)
    }
    pub(crate) fn prepared_comment_id(
        &self,
        reservation: &SpecContentHash,
        body: &SpecContentHash,
        marker: &SpecContentHash,
        observer_profile: Option<&super::InstallationObserverProfile>,
    ) -> Option<NonZeroU64> {
        let expected_profile = match observer_profile {
            Some(profile) => Some(profile.commitment()?),
            None => None,
        };
        (&self.reservation == reservation
            && &self.body_commitment == body
            && &self.marker_commitment == marker
            && self.observer_profile == expected_profile)
            .then_some(self.comment_id)
    }
}

#[derive(Deserialize)]
struct Notification<'a> {
    action: &'a str,
    installation: Identity,
    #[serde(borrow)]
    repository: Repository<'a>,
    #[serde(borrow)]
    issue: Issue<'a>,
    #[serde(borrow)]
    comment: Comment<'a>,
}

#[derive(Deserialize)]
struct Identity {
    id: NonZeroU64,
}

#[derive(Deserialize)]
struct Repository<'a> {
    id: NonZeroU64,
    full_name: &'a str,
}

#[derive(Deserialize)]
struct Issue<'a> {
    number: NonZeroU64,
    #[serde(borrow)]
    pull_request: PullRequest<'a>,
}

#[derive(Deserialize)]
struct PullRequest<'a> {
    url: &'a str,
}

#[derive(Deserialize)]
struct Comment<'a> {
    id: NonZeroU64,
    issue_url: &'a str,
    url: &'a str,
    user: Identity,
    #[serde(borrow)]
    body: Cow<'a, str>,
}

impl Notification<'_> {
    fn matches_scope(
        &self,
        installation: NonZeroU64,
        repository: NonZeroU64,
        target: &GitHubPullRequestCommentTarget,
    ) -> bool {
        let repo = format!("{}/{}", target.owner(), target.repository());
        let api = format!("https://api.github.com/repos/{repo}");
        let number = target.pull_request_number();
        self.action == "created"
            && self.installation.id == installation
            && self.repository.id == repository
            && self.repository.full_name == repo
            && self.issue.number.get() == number
            && self.issue.pull_request.url == format!("{api}/pulls/{number}")
            && self.comment.issue_url == format!("{api}/issues/{number}")
            && self.comment.url == format!("{api}/issues/comments/{}", self.comment.id)
    }
}

pub(crate) fn verify_creation_notification(
    expected: PreparedCreation<'_>,
    receiver: ReviewedCreationReceiver<'_>,
    input: CreationNotification<'_>,
) -> Option<CreationCandidate> {
    let receipt = input.receipt;
    if input.body.len() > MAX_NOTIFICATION_BYTES
        || receiver.writer_scope != expected.writer_scope
        || receiver.repository != expected.repository
        || receipt.repository != receiver.repository
        || receipt.receiving_installation != receiver.installation
        || !valid_receipt_time(&expected, receipt)
        || receipt.payload_digest != SpecContentHash::from_bytes(input.body)
        || expected.observer_profile.is_some_and(|profile| {
            receiver.installation != profile.receiver_installation
                || receiver.attestation_key != &profile.attestation_key
                || receiver.webhook_verifier_generation
                    != Some(&profile.webhook_verifier_generation)
        })
    {
        return None;
    }
    let receipt_bytes = receipt.signing_bytes()?;
    signature::UnparsedPublicKey::new(&signature::ED25519, receiver.attestation_key)
        .verify(&receipt_bytes, input.receipt_signature)
        .ok()?;
    let receipt_commitment = SpecContentHash::from_bytes(
        serde_json::to_vec(&(
            "workflow-os/verified-receiver-receipt/v1",
            receiver.attestation_key,
            &receipt_bytes,
        ))
        .ok()?,
    );
    let mac = hmac::Key::new(hmac::HMAC_SHA256, receiver.github_hmac);
    if expected
        .observer_profile
        .is_some_and(|profile| !profile.matches_webhook_material(&mac))
    {
        return None;
    }
    hmac::verify(&mac, input.body, &decode_signature(input.github_signature)?).ok()?;
    let event: Notification<'_> = serde_json::from_slice(input.body).ok()?;
    let observer_profile = match expected.observer_profile {
        Some(profile) => Some(profile.commitment()?),
        None => None,
    };
    (event.matches_scope(receiver.installation, expected.repository, expected.target)
        && event.comment.user.id == expected.writer
        && valid_body(&expected, &event.comment.body))
    .then(|| CreationCandidate {
        reservation: expected.reservation.clone(),
        payload_digest: receipt.payload_digest.clone(),
        comment_id: event.comment.id,
        body_commitment: expected.body_commitment.clone(),
        marker_commitment: expected.marker_commitment.clone(),
        receipt_commitment,
        received_latest: receipt.latest.observed_at(),
        observer_profile,
    })
}

fn valid_receipt_time(expected: &PreparedCreation<'_>, receipt: &CreationReceipt) -> bool {
    let prepared = expected.preparation_time;
    [&receipt.earliest, &receipt.latest, expected.checked_at]
        .into_iter()
        .all(|time| same_time_domain(time, prepared))
        && prepared.observed_at() <= receipt.earliest.observed_at()
        && receipt.earliest.observed_at() <= receipt.latest.observed_at()
        && receipt.latest.observed_at() < expected.deadline
        && receipt.latest.observed_at() <= expected.checked_at.observed_at()
}

fn same_time_domain(first: &TrustedTimeObservation, second: &TrustedTimeObservation) -> bool {
    first.source() == second.source()
        && first.epoch_id() == second.epoch_id()
        && first.provenance_commitment() == second.provenance_commitment()
}

fn valid_body(expected: &PreparedCreation<'_>, body: &str) -> bool {
    if body.len() > MAX_RAW_BODY_BYTES
        || SpecContentHash::from_text(body) != *expected.body_commitment
    {
        return false;
    }
    let marker = body
        .strip_prefix(expected.approved_body)
        .and_then(|suffix| suffix.strip_prefix("\n\n<!-- "))
        .and_then(|suffix| suffix.strip_suffix(" -->"));
    marker.is_some_and(|marker| {
        marker.starts_with(MARKER_NAMESPACE)
            && SpecContentHash::from_text(marker) == *expected.marker_commitment
            && encode_marked_comment(expected.approved_body, marker).as_deref() == Some(body)
    })
}

fn decode_signature(header: &str) -> Option<[u8; 32]> {
    let hex = header.strip_prefix("sha256=")?;
    if hex.len() != 64 || !hex.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return None;
    }
    let mut result = [0; 32];
    for (index, out) in result.iter_mut().enumerate() {
        *out = u8::from_str_radix(&hex[index * 2..index * 2 + 2], 16).ok()?;
    }
    Some(result)
}

#[cfg(test)]
#[allow(clippy::expect_used)]
pub(crate) mod tests {
    use super::*;
    use ring::signature::KeyPair;

    pub(crate) fn fixture_observer_profile() -> super::super::InstallationObserverProfile {
        let signer =
            signature::Ed25519KeyPair::from_seed_unchecked(&[7; 32]).expect("fixture signing key");
        let mut profile = super::super::InstallationObserverProfile {
            receiver_installation: NonZeroU64::new(91).expect("receiver installation"),
            attestation_key: signer
                .public_key()
                .as_ref()
                .try_into()
                .expect("fixture public key"),
            webhook_verifier_generation: SpecContentHash::from_text("fixture webhook generation 1"),
            webhook_material_tag: None,
            reader: NonZeroU64::new(777).expect("reader id"),
            reader_access_review: SpecContentHash::from_text(
                "fixture independent repository reader",
            ),
        };
        profile.bind_webhook_material(&[12; 32]);
        profile
    }
    use serde_json::{json, Value};
    use std::fmt::Write;

    pub(crate) fn github_header(key: &[u8; 32], payload: &[u8]) -> String {
        let key = hmac::Key::new(hmac::HMAC_SHA256, key);
        let mut header = String::from("sha256=");
        for byte in hmac::sign(&key, payload).as_ref() {
            write!(header, "{byte:02x}").expect("format signature");
        }
        header
    }

    #[test]
    fn comment_webhook_material_tags_bind_the_generation_and_preserve_legacy_encoding() {
        let mut profile = fixture_observer_profile();
        let verifier = hmac::Key::new(hmac::HMAC_SHA256, &[12; 32]);
        assert!(profile.matches_webhook_material(&verifier));
        let generation = profile.webhook_verifier_generation.clone();
        profile.webhook_verifier_generation = SpecContentHash::from_text("another generation");
        assert!(!profile.matches_webhook_material(&verifier));
        profile.webhook_verifier_generation = generation;
        let bound_commitment = profile.commitment();
        profile.webhook_material_tag = None;
        let legacy = serde_json::to_string(&profile).expect("legacy profile");
        assert!(!legacy.contains("webhook_material_tag"));
        let decoded: super::super::InstallationObserverProfile =
            serde_json::from_str(&legacy).expect("read legacy profile");
        assert_eq!(decoded.commitment(), profile.commitment());
        assert_ne!(decoded.commitment(), bound_commitment);
        assert!(!decoded.matches_webhook_material(&verifier));
    }

    fn provider_payload(body: &str) -> Value {
        json!({
            "action": "created", "installation": {"id": 91},
            "repository": {"id": 123, "full_name": "example/sandbox"},
            "issue": {"number": 7, "pull_request": {"url": "https://api.github.com/repos/example/sandbox/pulls/7"}},
            "comment": {"id": 22, "user": {"id": 456}, "body": body,
                "url": "https://api.github.com/repos/example/sandbox/issues/comments/22",
                "issue_url": "https://api.github.com/repos/example/sandbox/issues/7"},
            "sender": {"id": 456},
        })
    }

    pub(crate) fn exercise_prepared_notification(
        body: &str,
        writer_scope: &SpecContentHash,
        earliest: &TrustedTimeObservation,
        latest: &TrustedTimeObservation,
        deadline: Timestamp,
        verify: impl Fn(
            ReviewedCreationReceiver<'_>,
            CreationNotification<'_>,
        ) -> Option<CreationCandidate>,
        exercise: impl FnOnce(
            ReviewedCreationReceiver<'_>,
            CreationNotification<'_>,
            &CreationCandidate,
        ),
    ) -> CreationCandidate {
        let signer =
            signature::Ed25519KeyPair::from_seed_unchecked(&[7; 32]).expect("fixture signing key");
        let profile = fixture_observer_profile();
        let receiver = ReviewedCreationReceiver {
            installation: NonZeroU64::new(91).expect("independent receiving installation"),
            repository: NonZeroU64::new(123).expect("repository id"),
            writer_scope,
            github_hmac: &[12; 32],
            attestation_key: signer
                .public_key()
                .as_ref()
                .try_into()
                .expect("fixture public key"),
            webhook_verifier_generation: Some(&profile.webhook_verifier_generation),
        };
        let payload = provider_payload(body);
        assert_receipt_authentication(&payload, receiver, &signer, earliest, latest, &verify);
        let check = |event: &Value,
                     receiver: ReviewedCreationReceiver<'_>,
                     earliest: &TrustedTimeObservation,
                     latest: &TrustedTimeObservation| {
            with_signed_notification(event, receiver, &signer, earliest, latest, &verify)
        };
        let candidate =
            check(&payload, receiver, earliest, latest).expect("exact creation candidate");
        assert_eq!(candidate.comment_id.get(), 22);
        assert_eq!(
            check(&payload, receiver, earliest, latest).as_ref(),
            Some(&candidate)
        );
        let at_deadline =
            crate::authorized_execution_continuity_state::internal::trusted_time_observation(
                deadline,
                latest.source(),
                latest.provenance_commitment().clone(),
                latest.epoch_id().clone(),
            );
        assert!(check(&payload, receiver, earliest, &at_deadline).is_none());
        let other_epoch = crate::authorized_execution_continuity_state::internal::trusted_time_observation(
            latest.observed_at(), latest.source(), latest.provenance_commitment().clone(),
            crate::authorized_execution_continuity_state::internal::ContinuityTrustedTimeEpochId::new("epoch/other-receiver").expect("different epoch"),
        );
        assert!(check(&payload, receiver, earliest, &other_epoch).is_none());
        let wrong_scope = SpecContentHash::from_text("wrong writer scope");
        for wrong in [
            ReviewedCreationReceiver {
                writer_scope: &wrong_scope,
                ..receiver
            },
            ReviewedCreationReceiver {
                installation: NonZeroU64::new(44).expect("writer installation"),
                ..receiver
            },
            ReviewedCreationReceiver {
                github_hmac: &[13; 32],
                ..receiver
            },
            ReviewedCreationReceiver {
                attestation_key: &[0; 32],
                ..receiver
            },
        ] {
            assert!(check(&payload, wrong, earliest, latest).is_none());
        }
        for (path, wrong) in [
            ("/action", json!("edited")),
            ("/installation/id", json!(44)),
            ("/repository/id", json!(124)),
            ("/issue/number", json!(8)),
            ("/issue/pull_request", Value::Null),
            ("/comment/user/id", json!(457)),
            ("/comment/body", json!("other body")),
        ] {
            let mut altered = payload.clone();
            *altered.pointer_mut(path).expect("fixture field") = wrong;
            assert!(check(&altered, receiver, earliest, latest).is_none());
        }
        with_captured_notification(
            payload,
            receiver,
            &signer,
            &profile,
            deadline,
            [earliest, latest, &at_deadline, &other_epoch],
            |notification| {
                assert_eq!(verify(receiver, notification).as_ref(), Some(&candidate));
                exercise(receiver, notification, &candidate);
            },
        );
        candidate
    }

    fn with_captured_notification(
        payload: Value,
        receiver: ReviewedCreationReceiver<'_>,
        signer: &signature::Ed25519KeyPair,
        profile: &super::super::InstallationObserverProfile,
        deadline: Timestamp,
        [earliest, latest, at_deadline, other_epoch]: [&TrustedTimeObservation; 4],
        consume: impl FnOnce(CreationNotification<'_>),
    ) {
        let target =
            GitHubPullRequestCommentTarget::new("example", "sandbox", 7).expect("fixture target");
        let direct_fixture = || {
            CreationReceiptAttester::new(
                profile,
                receiver.repository,
                &target,
                deadline,
                receiver.github_hmac,
                signer,
            )
            .expect("exact receiver custody configuration")
        };
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        let custody =
            (!nix::unistd::geteuid().is_root()).then(|| custody::tests::fixture_custody(profile));
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        let attester = custody.as_ref().map_or_else(direct_fixture, |material| {
            material
                .attester(receiver.repository, &target, deadline)
                .expect("attester from exact receiver custody")
        });
        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        let attester = direct_fixture();
        assert!(CreationReceiptAttester::new(
            profile,
            receiver.repository,
            &target,
            deadline,
            &[13; 32],
            signer
        )
        .is_none());
        let alternate_signer =
            signature::Ed25519KeyPair::from_seed_unchecked(&[9; 32]).expect("other fixture signer");
        assert!(CreationReceiptAttester::new(
            profile,
            receiver.repository,
            &target,
            deadline,
            receiver.github_hmac,
            &alternate_signer
        )
        .is_none());
        let bytes = serde_json::to_vec(&payload).expect("exact body");
        let header = github_header(receiver.github_hmac, &bytes);
        let bad_header = github_header(&[13; 32], &bytes);
        let mut wrong_target = payload;
        wrong_target["issue"]["number"] = json!(8);
        let wrong_target = serde_json::to_vec(&wrong_target).expect("other target bytes");
        let target_header = github_header(receiver.github_hmac, &wrong_target);
        let mut oversized = bytes.clone();
        oversized.resize(MAX_NOTIFICATION_BYTES + 1, b' ');
        let oversized_header = github_header(receiver.github_hmac, &oversized);
        for (body, header, samples) in [
            (bytes.as_slice(), bad_header.as_str(), [earliest, latest]),
            (&wrong_target, target_header.as_str(), [earliest, latest]),
            (&oversized, oversized_header.as_str(), [earliest, latest]),
            (&bytes, header.as_str(), [earliest, at_deadline]),
            (&bytes, header.as_str(), [earliest, other_epoch]),
            (&bytes, header.as_str(), [latest, earliest]),
        ] {
            let mut samples = samples.into_iter();
            assert!(attester
                .capture(body, header, || samples.next().cloned())
                .is_none());
        }
        let mut samples = [earliest, latest].into_iter();
        let captured = attester
            .capture(bytes.as_slice(), &header, || samples.next().cloned())
            .expect("capture and attest original notification bytes");
        assert_eq!(captured.notification().body, bytes);
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        let captured = handoff::tests::transfer_fixture(captured);
        consume(captured.notification());
    }

    fn with_signed_notification<T>(
        event: &Value,
        receiver: ReviewedCreationReceiver<'_>,
        signer: &signature::Ed25519KeyPair,
        earliest: &TrustedTimeObservation,
        latest: &TrustedTimeObservation,
        consume: impl FnOnce(ReviewedCreationReceiver<'_>, CreationNotification<'_>) -> T,
    ) -> T {
        let bytes = serde_json::to_vec(event).expect("provider-shaped bytes");
        let receipt = CreationReceipt {
            receiving_installation: receiver.installation,
            repository: receiver.repository,
            payload_digest: SpecContentHash::from_bytes(&bytes),
            earliest: earliest.clone(),
            latest: latest.clone(),
        };
        let sig = signer.sign(&receipt.signing_bytes().expect("canonical receipt"));
        consume(
            receiver,
            CreationNotification {
                body: &bytes,
                github_signature: &github_header(&[12; 32], &bytes),
                receipt: &receipt,
                receipt_signature: sig.as_ref().try_into().expect("fixture signature"),
            },
        )
    }

    fn assert_receipt_authentication(
        payload: &Value,
        receiver: ReviewedCreationReceiver<'_>,
        signer: &signature::Ed25519KeyPair,
        earliest: &TrustedTimeObservation,
        latest: &TrustedTimeObservation,
        verify: impl Fn(
            ReviewedCreationReceiver<'_>,
            CreationNotification<'_>,
        ) -> Option<CreationCandidate>,
    ) {
        let bytes = serde_json::to_vec(payload).expect("exact signed body");
        let mut receipt = CreationReceipt {
            receiving_installation: receiver.installation,
            repository: receiver.repository,
            payload_digest: SpecContentHash::from_bytes(&bytes),
            earliest: earliest.clone(),
            latest: latest.clone(),
        };
        let attestation = signer.sign(&receipt.signing_bytes().expect("signed receipt"));
        let sig = attestation.as_ref().try_into().expect("signature length");
        let check = |body: &[u8], receipt: &CreationReceipt| {
            verify(
                receiver,
                CreationNotification {
                    body,
                    github_signature: &github_header(receiver.github_hmac, body),
                    receipt,
                    receipt_signature: sig,
                },
            )
        };
        let mut swapped = bytes.clone();
        swapped.push(b' ');
        assert!(
            check(&swapped, &receipt).is_none(),
            "fresh provider MAC cannot reuse an old receipt"
        );
        receipt.earliest = latest.clone();
        receipt.latest = latest.clone();
        assert!(
            check(&bytes, &receipt).is_none(),
            "receipt timing is authenticated"
        );
    }
}
