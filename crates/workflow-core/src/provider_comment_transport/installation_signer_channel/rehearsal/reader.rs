use std::io::Read;

use super::*;
use crate::provider_comment_transport::installation_exchange::reader::ReaderSignerRequest;
use crate::provider_comment_transport::installation_signer_channel::reader_signer::{
    ReaderSignerPeerPolicy, RecoveryReaderSigner, RecoveryReaderSignerReceiver,
};
use crate::SpecContentHash;

const READER_POLICY: ReaderSignerPeerPolicy = ReaderSignerPeerPolicy {
    core_uid: POLICY.exchange_uid,
    reader_signer_uid: 61_025,
    writer_signer_uid: POLICY.signer_uid,
    harness_uids: POLICY.harness_uids,
};
const INVENTORY_LIMIT: usize = 64 * 1024;

pub(crate) fn serve_operator_inventory(
    review: &SpecContentHash,
    deadline: Timestamp,
    inventory: &serde_json::Value,
) {
    use super::super::{socket_peer_uid, write_frame};
    assert_isolated(READER_POLICY.reader_signer_uid);
    let root = fixture_root();
    let assertion = bind_socket(&root.join("reader-socket/assertion"));
    let inventory_socket = bind_socket(&root.join("reader-socket/inventory"));
    let mut recovery = None;
    RecoveryReaderSignerReceiver::authenticate(accept_socket(&assertion), &READER_POLICY)
        .expect("exact Core peer")
        .serve_once(review, deadline, |request| {
            recovery = Some(request.recovery.clone());
            Some(("fixture.reader.assertion".to_owned(), request.deadline))
        })
        .expect("operation-bound synthetic signer");
    let mut stream = accept_socket(&inventory_socket);
    assert_eq!(
        socket_peer_uid(&stream).expect("inventory peer"),
        READER_POLICY.core_uid
    );
    let bytes = serde_json::to_vec(&(recovery.expect("signed recovery"), inventory))
        .expect("synthetic inventory packet");
    write_frame(
        &mut stream,
        &bytes,
        INVENTORY_LIMIT,
        Instant::now() + Duration::from_secs(5),
    )
    .expect("bounded inventory");
}

pub(crate) fn read_operator_inventory(request: &ReaderSignerRequest) -> Option<Vec<u8>> {
    use super::super::{read_frame, socket_peer_uid};
    let root = fixture_root();
    let signer = RecoveryReaderSigner::authenticate(
        connect_socket(&root.join("reader-socket/assertion")),
        &READER_POLICY,
    )
    .ok()?;
    signer.request(request).ok()?.into_reader_parts(request)?;
    let mut stream = connect_socket(&root.join("reader-socket/inventory"));
    if socket_peer_uid(&stream).ok()? != READER_POLICY.reader_signer_uid {
        return None;
    }
    let bytes = read_frame(
        &mut stream,
        INVENTORY_LIMIT,
        Instant::now() + Duration::from_secs(5),
    )
    .ok()?;
    let (recovery, inventory): (SpecContentHash, serde_json::Value) =
        serde_json::from_slice(&bytes).ok()?;
    if recovery != request.recovery {
        return None;
    }
    serde_json::to_vec(&inventory).ok()
}

fn access_review() -> SpecContentHash {
    SpecContentHash::from_text("hosted reader scope rehearsal")
}

fn recovery() -> SpecContentHash {
    SpecContentHash::from_text("hosted exact pending recovery rehearsal")
}

fn after(seconds: i64) -> Timestamp {
    Timestamp::from_offset_date_time(
        Timestamp::now_utc().as_offset_date_time() + time::Duration::seconds(seconds),
    )
}

#[test]
#[ignore = "requires the hosted signer rehearsal launcher"]
fn rehearsal_reader_signer_process() {
    assert_isolated(READER_POLICY.reader_signer_uid);
    let root = fixture_root();
    assert_file_inaccessible(&root.join("core-private/authority"));
    assert_file_inaccessible(&root.join("signer-private/custody"));
    assert!(fs::File::open(root.join("reader-private/custody")).is_ok());
    let callback = root.join("reader-private/callback");
    let listener = bind_socket(&root.join("reader-socket/assertion"));
    assert!(
        RecoveryReaderSignerReceiver::authenticate(accept_socket(&listener), &READER_POLICY)
            .is_err()
    );
    let serve = || {
        RecoveryReaderSignerReceiver::authenticate(accept_socket(&listener), &READER_POLICY)
            .expect("exact Core peer")
            .serve_once(&access_review(), after(30), |request| {
                assert_eq!(request.recovery, recovery());
                OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(&callback)
                    .expect("one synthetic reader signing callback");
                Some(("fixture.reader.assertion".to_owned(), request.deadline))
            })
    };
    assert!(serve().is_err());
    assert!(!callback.exists());
    serve().expect("scoped reader signing");
    assert!(callback.is_file());
}

#[test]
#[ignore = "requires the hosted signer rehearsal launcher"]
fn rehearsal_reader_core_process() {
    assert_isolated(READER_POLICY.core_uid);
    let root = fixture_root();
    assert_file_inaccessible(&root.join("reader-private/custody"));
    let path = root.join("reader-socket/assertion");
    for (review, expected) in [
        (SpecContentHash::from_text("wrong review"), false),
        (access_review(), true),
    ] {
        let channel = RecoveryReaderSigner::authenticate(connect_socket(&path), &READER_POLICY)
            .expect("exact reader signer peer");
        let request = ReaderSignerRequest::new(recovery(), review, after(15));
        let result = channel.request(&request);
        if expected {
            assert!(result
                .expect("exact reader assertion")
                .into_reader_parts(&request)
                .is_some());
        } else {
            assert!(result.is_err());
        }
    }
}

#[test]
#[ignore = "requires the hosted signer rehearsal launcher"]
fn rehearsal_reader_writer_probe() {
    assert_isolated(READER_POLICY.writer_signer_uid);
    let root = fixture_root();
    assert_file_inaccessible(&root.join("reader-private/custody"));
    let path = root.join("reader-socket/assertion");
    let mut stream = connect_socket(&path);
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .expect("bounded probe");
    assert_eq!(
        stream
            .read(&mut [0])
            .expect("wrong peer closed without signing"),
        0
    );
    assert_eq!(
        fs::remove_file(path).err().map(|e| e.kind()),
        Some(ErrorKind::PermissionDenied)
    );
}

#[test]
#[ignore = "requires the hosted signer rehearsal launcher"]
fn rehearsal_reader_harness_probe() {
    assert_isolated(READER_POLICY.harness_uids[0]);
    let root = fixture_root();
    assert_file_inaccessible(&root.join("reader-private/custody"));
    let path = root.join("reader-socket/assertion");
    assert_eq!(
        UnixStream::connect(&path).err().map(|e| e.kind()),
        Some(ErrorKind::PermissionDenied)
    );
    assert_eq!(
        fs::remove_file(path).err().map(|e| e.kind()),
        Some(ErrorKind::PermissionDenied)
    );
}
