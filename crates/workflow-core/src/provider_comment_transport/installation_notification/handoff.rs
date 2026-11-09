//! One bounded packet from the reviewed receiver, never an authorization.
use std::{
    os::unix::net::UnixStream,
    time::{Duration, Instant},
};

use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::{Deserialize, Serialize};

use super::{
    decode_signature, AttestedCreationNotification, CreationNotification, CreationReceipt,
    MAX_NOTIFICATION_BYTES,
};
use crate::provider_comment_transport::installation_signer_channel::{
    read_frame, socket_peer_uid, write_frame,
};
use crate::SpecContentHash;

const MAX_PACKET_BYTES: usize = 360 * 1024;
const PACKET_TIMEOUT: Duration = Duration::from_secs(5);

pub(super) struct ReceiverPeerPolicy {
    pub(super) core_uid: u32,
    pub(super) receiver_uid: u32,
    pub(super) excluded_uids: [u32; 4],
}

impl ReceiverPeerPolicy {
    fn authenticates(&self, stream: &UnixStream, local: u32, peer: u32) -> bool {
        self.core_uid != 0
            && self.receiver_uid != 0
            && self.core_uid != self.receiver_uid
            && !self.excluded_uids.contains(&self.core_uid)
            && !self.excluded_uids.contains(&self.receiver_uid)
            && nix::unistd::geteuid().as_raw() == local
            && socket_peer_uid(stream).ok() == Some(peer)
    }
}

pub(super) struct ReceiptSender(UnixStream);
pub(super) struct ReceiptReceiver(UnixStream);

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct WireReceipt {
    kind: PacketKind,
    body: String,
    github_signature: String,
    receipt: CreationReceipt,
    receipt_signature: String,
}

#[derive(Deserialize, Serialize)]
enum PacketKind {
    #[serde(rename = "workflow-os/receiver-notification/v1")]
    ReceiverNotification,
}

pub(super) struct ReceivedNotification {
    body: Vec<u8>,
    github_signature: String,
    receipt: CreationReceipt,
    receipt_signature: [u8; 64],
}

impl ReceivedNotification {
    pub(super) fn notification(&self) -> CreationNotification<'_> {
        CreationNotification {
            body: &self.body,
            github_signature: &self.github_signature,
            receipt: &self.receipt,
            receipt_signature: &self.receipt_signature,
        }
    }
}

impl ReceiptSender {
    pub(super) fn authenticate(stream: UnixStream, policy: &ReceiverPeerPolicy) -> Option<Self> {
        policy
            .authenticates(&stream, policy.receiver_uid, policy.core_uid)
            .then_some(Self(stream))
    }

    pub(super) fn send(mut self, captured: AttestedCreationNotification<'_>) -> Option<()> {
        let stop = configure(&self.0)?;
        let packet = WireReceipt {
            kind: PacketKind::ReceiverNotification,
            body: STANDARD.encode(captured.body),
            github_signature: captured.github_signature.to_owned(),
            receipt: captured.receipt,
            receipt_signature: STANDARD.encode(captured.receipt_signature),
        };
        write_frame(
            &mut self.0,
            &serde_json::to_vec(&packet).ok()?,
            MAX_PACKET_BYTES,
            stop,
        )
        .ok()
    }
}

impl ReceiptReceiver {
    pub(super) fn authenticate(stream: UnixStream, policy: &ReceiverPeerPolicy) -> Option<Self> {
        policy
            .authenticates(&stream, policy.core_uid, policy.receiver_uid)
            .then_some(Self(stream))
    }

    pub(super) fn receive(mut self) -> Option<ReceivedNotification> {
        let stop = configure(&self.0)?;
        let bytes = read_frame(&mut self.0, MAX_PACKET_BYTES, stop).ok()?;
        decode_packet(&bytes)
    }
}

fn decode_packet(bytes: &[u8]) -> Option<ReceivedNotification> {
    let packet: WireReceipt = serde_json::from_slice(bytes).ok()?;
    decode_signature(&packet.github_signature)?;
    let body = STANDARD.decode(packet.body).ok()?;
    if body.len() > MAX_NOTIFICATION_BYTES
        || packet.receipt.payload_digest != SpecContentHash::from_bytes(&body)
    {
        return None;
    }
    Some(ReceivedNotification {
        body,
        github_signature: packet.github_signature,
        receipt: packet.receipt,
        receipt_signature: STANDARD
            .decode(packet.receipt_signature)
            .ok()?
            .try_into()
            .ok()?,
    })
}

fn configure(stream: &UnixStream) -> Option<Instant> {
    let stop = Instant::now().checked_add(PACKET_TIMEOUT)?;
    stream.set_nonblocking(true).ok()?;
    Some(stop)
}

#[cfg(test)]
#[allow(clippy::expect_used)]
pub(super) mod tests {
    use super::*;
    use std::io::Write;

    pub(in crate::provider_comment_transport::installation_notification) fn transfer_fixture(
        captured: AttestedCreationNotification<'_>,
    ) -> ReceivedNotification {
        let (sender, receiver) = UnixStream::pair().expect("disposable receiver channel");
        std::thread::scope(|threads| {
            let writer = threads.spawn(|| ReceiptSender(sender).send(captured));
            let packet = ReceiptReceiver(receiver)
                .receive()
                .expect("bounded receipt packet");
            writer
                .join()
                .expect("fixture receipt sender")
                .expect("write exact receipt");
            let mut altered = packet.body.clone();
            altered.push(b' ');
            let wire = serde_json::json!({
                "kind": "workflow-os/receiver-notification/v1",
                "body": STANDARD.encode(altered),
                "github_signature": packet.github_signature,
                "receipt": packet.receipt,
                "receipt_signature": STANDARD.encode(packet.receipt_signature),
            });
            assert!(
                decode_packet(&serde_json::to_vec(&wire).expect("altered fixture packet"))
                    .is_none()
            );
            packet
        })
    }

    #[test]
    fn installation_notification_handoff_rejects_wrong_peers_and_frames() {
        let uid = nix::unistd::geteuid().as_raw();
        let mut policy = ReceiverPeerPolicy {
            core_uid: uid,
            receiver_uid: uid.wrapping_add(1),
            excluded_uids: [0; 4],
        };
        let (sender, receiver) = UnixStream::pair().expect("fixture sockets");
        assert!(ReceiptSender::authenticate(sender, &policy).is_none());
        assert!(ReceiptReceiver::authenticate(receiver, &policy).is_none());
        policy.receiver_uid = uid;
        let (sender, receiver) = UnixStream::pair().expect("fixture sockets");
        assert!(ReceiptSender::authenticate(sender, &policy).is_none());
        assert!(ReceiptReceiver::authenticate(receiver, &policy).is_none());
        for (size, payload) in [
            (u32::MAX, &[][..]),
            (3, b"{}"),
            (2, b"{}x"),
            (2, b"{}"),
            (2, b"[]"),
        ] {
            let (mut sender, receiver) = UnixStream::pair().expect("fixture sockets");
            sender.write_all(&size.to_be_bytes()).expect("frame size");
            sender.write_all(payload).expect("frame payload");
            drop(sender);
            assert!(ReceiptReceiver(receiver).receive().is_none());
        }
    }
}
