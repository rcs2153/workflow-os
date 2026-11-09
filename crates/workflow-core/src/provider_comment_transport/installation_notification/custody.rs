//! Material intake from reviewed host descriptors, not caller-supplied paths.
use std::fs::{File, Metadata};
use std::os::unix::fs::{FileExt, MetadataExt};

use super::{CreationReceiptAttester, GitHubPullRequestCommentTarget, NonZeroU64, Timestamp};
use crate::provider_comment_transport::InstallationObserverProfile;
use ring::{
    hmac,
    signature::{Ed25519KeyPair, KeyPair},
};

pub(super) struct ReceiverCustody<'a> {
    profile: &'a InstallationObserverProfile,
    webhook: [u8; 32],
    signer: Ed25519KeyPair,
}

impl<'a> ReceiverCustody<'a> {
    pub(super) fn from_descriptors(
        reviewed_uid: u32,
        profile: &'a InstallationObserverProfile,
        webhook: &File,
        signer_seed: &File,
    ) -> Option<Self> {
        if reviewed_uid == 0 || nix::unistd::geteuid().as_raw() != reviewed_uid {
            return None;
        }
        let (webhook, webhook_file) = read_material(webhook, reviewed_uid)?;
        let (seed, signer_file) = read_material(signer_seed, reviewed_uid)?;
        if webhook_file == signer_file || webhook == seed {
            return None;
        }
        let signer = Ed25519KeyPair::from_seed_unchecked(&seed).ok()?;
        let verifier = hmac::Key::new(hmac::HMAC_SHA256, &webhook);
        if !profile.matches_webhook_material(&verifier)
            || signer.public_key().as_ref() != profile.attestation_key
        {
            return None;
        }
        Some(Self {
            profile,
            webhook,
            signer,
        })
    }

    pub(super) fn attester<'b>(
        &'b self,
        repository: NonZeroU64,
        target: &'b GitHubPullRequestCommentTarget,
        deadline: Timestamp,
    ) -> Option<CreationReceiptAttester<'b>> {
        CreationReceiptAttester::new(
            self.profile,
            repository,
            target,
            deadline,
            &self.webhook,
            &self.signer,
        )
    }
}

fn protected(meta: &Metadata, owner: u32) -> bool {
    meta.is_file()
        && meta.uid() == owner
        && meta.mode() & 0o7777 == 0o400
        && meta.nlink() == 1
        && meta.len() == 32
}

fn read_material(file: &File, owner: u32) -> Option<([u8; 32], (u64, u64))> {
    let before = file.metadata().ok()?;
    if !protected(&before, owner) {
        return None;
    }
    let mut bytes = [0; 32];
    file.read_exact_at(&mut bytes, 0).ok()?;
    let after = file.metadata().ok()?;
    if !protected(&after, owner)
        || before.modified().ok()? != after.modified().ok()?
        || (before.ctime(), before.ctime_nsec()) != (after.ctime(), after.ctime_nsec())
    {
        return None;
    }
    Some((bytes, (after.dev(), after.ino())))
}

#[cfg(test)]
#[allow(clippy::expect_used)]
pub(super) mod tests {
    use super::*;
    use std::{
        fs,
        os::unix::fs::{DirBuilderExt, PermissionsExt},
        path::{Path, PathBuf},
    };

    struct FixtureRoot(PathBuf);

    impl FixtureRoot {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "workflow-os-receiver-{}",
                crate::WorkflowRunId::generate().as_str()
            ));
            fs::DirBuilder::new()
                .mode(0o700)
                .create(&path)
                .expect("private fixture directory");
            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for FixtureRoot {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    pub(in crate::provider_comment_transport::installation_notification) fn material(
        path: &Path,
        bytes: &[u8],
        mode: u32,
    ) -> File {
        use std::{io::Write, os::unix::fs::OpenOptionsExt};
        let mut file = fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .mode(0o600)
            .open(path)
            .expect("synthetic material file");
        file.write_all(bytes).expect("synthetic material");
        file.set_permissions(fs::Permissions::from_mode(mode))
            .expect("fixture mode");
        drop(file);
        File::open(path).expect("read-only fixture descriptor")
    }

    pub(in crate::provider_comment_transport::installation_notification) fn fixture_custody(
        profile: &InstallationObserverProfile,
    ) -> ReceiverCustody<'_> {
        let root = FixtureRoot::new();
        let webhook = material(&root.path().join("webhook"), &[12; 32], 0o400);
        let signer = material(&root.path().join("signer"), &[7; 32], 0o400);
        ReceiverCustody::from_descriptors(
            nix::unistd::geteuid().as_raw(),
            profile,
            &webhook,
            &signer,
        )
        .expect("exact fixture custody")
    }

    #[test]
    fn installation_receiver_custody_rejects_unreviewed_descriptors_and_material() {
        let uid = nix::unistd::geteuid().as_raw();
        let profile = super::super::tests::fixture_observer_profile();
        let root = FixtureRoot::new();
        let path = |name| root.path().join(name);
        let webhook = material(&path("webhook"), &[12; 32], 0o400);
        let signer = material(&path("signer"), &[7; 32], 0o400);
        assert!(ReceiverCustody::from_descriptors(0, &profile, &webhook, &signer).is_none());
        if uid == 0 {
            return;
        }
        assert!(ReceiverCustody::from_descriptors(uid, &profile, &webhook, &signer).is_some());
        assert!(ReceiverCustody::from_descriptors(
            uid.wrapping_add(1),
            &profile,
            &webhook,
            &signer
        )
        .is_none());
        let directory = File::open(root.path()).expect("directory descriptor");
        for bad in [
            material(&path("broad"), &[12; 32], 0o440),
            material(&path("writable"), &[12; 32], 0o600),
            material(&path("short"), &[12; 31], 0o400),
            material(&path("long"), &[12; 33], 0o400),
            material(&path("wrong"), &[13; 32], 0o400),
            directory,
        ] {
            assert!(ReceiverCustody::from_descriptors(uid, &profile, &bad, &signer).is_none());
        }
        let wrong_signer = material(&path("wrong-signer"), &[9; 32], 0o400);
        assert!(
            ReceiverCustody::from_descriptors(uid, &profile, &webhook, &wrong_signer).is_none()
        );
        assert!(ReceiverCustody::from_descriptors(uid, &profile, &webhook, &webhook).is_none());
        fs::hard_link(path("webhook"), path("alias")).expect("fixture hard link");
        assert!(ReceiverCustody::from_descriptors(uid, &profile, &webhook, &signer).is_none());
    }
}
