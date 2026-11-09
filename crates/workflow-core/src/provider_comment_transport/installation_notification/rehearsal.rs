#![allow(clippy::expect_used)]

use std::{fs, io::ErrorKind, os::unix::net::UnixStream, time::Duration};

use super::custody::{tests::material, ReceiverCustody};
use super::handoff::{ReceiptReceiver, ReceiptSender, ReceiverPeerPolicy};
use super::tests::{fixture_observer_profile, github_header, provider_payload};
use super::*;
use crate::authorized_execution_continuity_state::internal::{
    trusted_time_observation, ContinuityTrustedTimeEpochId, TrustedTimeSourceKind,
};
use crate::provider_comment_transport::installation_signer_channel::rehearsal::{
    accept_socket, assert_file_inaccessible, assert_isolated, bind_socket, connect_socket,
    fixture_root, POLICY,
};

const RECEIVER: ReceiverPeerPolicy = ReceiverPeerPolicy {
    core_uid: POLICY.exchange_uid,
    receiver_uid: 61_026,
    excluded_uids: [
        POLICY.signer_uid,
        61_025,
        POLICY.harness_uids[0],
        POLICY.harness_uids[1],
    ],
};
const BODY: &str = "Synthetic hosted recovery comment";

fn time_at(seconds: i64) -> TrustedTimeObservation {
    trusted_time_observation(
        Timestamp::from_offset_date_time(
            time::OffsetDateTime::from_unix_timestamp(seconds).expect("fixture timestamp"),
        ),
        TrustedTimeSourceKind::CoreInjectedClockV1,
        SpecContentHash::from_text("synthetic receiver clock"),
        ContinuityTrustedTimeEpochId::new("epoch/receiver-rehearsal").expect("fixture epoch"),
    )
}

fn target() -> GitHubPullRequestCommentTarget {
    GitHubPullRequestCommentTarget::new("example", "sandbox", 7).expect("fixture target")
}

fn repository() -> NonZeroU64 {
    NonZeroU64::new(123).expect("fixture repository")
}

fn marked_body() -> (String, String) {
    let marker = format!("{MARKER_NAMESPACE}{}", "a".repeat(64));
    let full_body = encode_marked_comment(BODY, &marker).expect("exact marked fixture body");
    (marker, full_body)
}

#[test]
#[ignore = "requires the hosted signer rehearsal launcher"]
fn rehearsal_notification_receiver() {
    assert_isolated(RECEIVER.receiver_uid);
    let root = fixture_root();
    for file in [
        "core-private/authority",
        "signer-private/custody",
        "reader-private/custody",
    ] {
        assert_file_inaccessible(&root.join(file));
    }
    let webhook = material(&root.join("receiver-private/webhook"), &[12; 32], 0o400);
    let seed = material(&root.join("receiver-private/receipt"), &[7; 32], 0o400);
    let profile = fixture_observer_profile();
    let custody =
        ReceiverCustody::from_descriptors(RECEIVER.receiver_uid, &profile, &webhook, &seed)
            .expect("isolated receiver custody");
    let target = target();
    let attester = custody
        .attester(repository(), &target, time_at(30).observed_at())
        .expect("reviewed receiver");
    let listener = bind_socket(&root.join("receiver-socket/receipt"));
    assert!(ReceiptSender::authenticate(accept_socket(&listener), &RECEIVER).is_none());
    let sender =
        ReceiptSender::authenticate(accept_socket(&listener), &RECEIVER).expect("exact Core peer");
    let (_, marked) = marked_body();
    let raw = serde_json::to_vec(&provider_payload(&marked)).expect("original fixture bytes");
    let mac = github_header(&[12; 32], &raw);
    let mut observed = [time_at(20), time_at(21)].into_iter();
    let packet = attester
        .capture(raw.as_slice(), &mac, || observed.next())
        .expect("authenticated receipt");
    sender.send(packet).expect("one bounded receipt");
}

#[test]
#[ignore = "requires the hosted signer rehearsal launcher"]
fn rehearsal_notification_core() {
    assert_isolated(RECEIVER.core_uid);
    let root = fixture_root();
    let incoming = ReceiptReceiver::authenticate(
        connect_socket(&root.join("receiver-socket/receipt")),
        &RECEIVER,
    )
    .expect("exact receiver peer")
    .receive()
    .expect("one bounded receipt");
    assert_receiver_custody_denied();
    let profile = fixture_observer_profile();
    let writer_scope = SpecContentHash::from_text("reviewed fixture writer scope");
    let (marker, full_body) = marked_body();
    let receiver = ReviewedCreationReceiver {
        installation: profile.receiver_installation,
        repository: repository(),
        writer_scope: &writer_scope,
        github_hmac: &[12; 32],
        attestation_key: &profile.attestation_key,
        webhook_verifier_generation: Some(&profile.webhook_verifier_generation),
    };
    let expected = PreparedCreation {
        reservation: &SpecContentHash::from_text("reviewed fixture reservation"),
        writer_scope: &writer_scope,
        target: &target(),
        repository: repository(),
        writer: NonZeroU64::new(456).expect("fixture writer"),
        approved_body: BODY,
        marker_commitment: &SpecContentHash::from_text(&marker),
        body_commitment: &SpecContentHash::from_text(&full_body),
        preparation_time: &time_at(19),
        checked_at: &time_at(22),
        deadline: time_at(30).observed_at(),
        observer_profile: Some(&profile),
    };
    let candidate = verify_creation_notification(expected, receiver, incoming.notification())
        .expect("independent Core verification");
    assert_eq!(candidate.comment_id.get(), 22);
    assert!(verify_creation_notification(
        PreparedCreation {
            writer: NonZeroU64::MIN,
            ..expected
        },
        receiver,
        incoming.notification()
    )
    .is_none());
}

fn assert_receiver_custody_denied() {
    let root = fixture_root();
    for path in ["receiver-private/webhook", "receiver-private/receipt"] {
        assert_file_inaccessible(&root.join(path));
    }
}

#[test]
#[ignore = "requires the hosted signer rehearsal launcher"]
fn rehearsal_notification_writer_probe() {
    assert_isolated(POLICY.signer_uid);
    let path = fixture_root().join("receiver-socket/receipt");
    let mut peer = connect_socket(&path);
    assert_receiver_custody_denied();
    peer.set_read_timeout(Some(Duration::from_secs(5)))
        .expect("bounded denial probe");
    assert_eq!(
        peer.read(&mut [0]).expect("unauthorized connection closed"),
        0
    );
    assert_eq!(
        fs::remove_file(path).err().map(|error| error.kind()),
        Some(ErrorKind::PermissionDenied)
    );
}

#[test]
#[ignore = "requires the hosted signer rehearsal launcher"]
fn rehearsal_notification_harness_probe() {
    assert_isolated(POLICY.harness_uids[0]);
    assert_receiver_custody_denied();
    let path = fixture_root().join("receiver-socket/receipt");
    assert_eq!(
        UnixStream::connect(&path).err().map(|error| error.kind()),
        Some(ErrorKind::PermissionDenied)
    );
    assert_eq!(
        fs::remove_file(path).err().map(|error| error.kind()),
        Some(ErrorKind::PermissionDenied)
    );
}
