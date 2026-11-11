use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

use serde::Deserialize;

use super::{
    bounded_bytes, denied, executable_digest, linux_isolation, matches_identity,
    only_standard_descriptors, protected_file, unprotected,
};
use crate::protected_launch::ProtectedLaunchInspection;
use crate::{SpecContentHash, WorkflowOsError};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    version: String,
    executable_sha256: SpecContentHash,
    role: Role,
    identities: Identities,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum Role {
    Core,
    Signer,
    Reader,
    Receiver,
    Harness,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Identity {
    uid: u32,
    gid: u32,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Identities {
    core: Identity,
    signer: Identity,
    reader: Identity,
    receiver: Identity,
    harness: Identity,
}

fn invalid_manifest() -> WorkflowOsError {
    denied(
        "protected_launch.manifest.invalid",
        "launch manifest is invalid or contains unsupported fields",
    )
}

impl Manifest {
    fn parse(bytes: &[u8]) -> Result<Self, WorkflowOsError> {
        let manifest: Self = serde_json::from_slice(bytes).map_err(|_| invalid_manifest())?;
        let roles = &manifest.identities;
        let identities = [
            &roles.core,
            &roles.signer,
            &roles.reader,
            &roles.receiver,
            &roles.harness,
        ];
        if manifest.version != "workflow-os/protected-launch/v1"
            || identities.iter().enumerate().any(|(i, identity)| {
                identity.uid == 0
                    || identity.gid == 0
                    || identity.uid == u32::MAX
                    || identity.gid == u32::MAX
                    || identities[..i]
                        .iter()
                        .any(|prior| prior.uid == identity.uid)
            })
        {
            return Err(invalid_manifest());
        }
        Ok(manifest)
    }

    fn selected(&self) -> (&'static str, &Identity) {
        let roles = &self.identities;
        match self.role {
            Role::Core => ("core", &roles.core),
            Role::Signer => ("signer", &roles.signer),
            Role::Reader => ("reader", &roles.reader),
            Role::Receiver => ("receiver", &roles.receiver),
            Role::Harness => ("harness", &roles.harness),
        }
    }
}

pub(in crate::protected_launch) fn inspect(
    manifest: &Path,
    expected: &SpecContentHash,
) -> Result<ProtectedLaunchInspection, WorkflowOsError> {
    if nix::unistd::getuid().is_root() || nix::unistd::geteuid().is_root() {
        return Err(denied(
            "protected_launch.identity.root",
            "root launch is not permitted",
        ));
    }
    if !only_standard_descriptors() {
        return Err(denied(
            "protected_launch.descriptors.unverified",
            "inherited descriptors are not permitted",
        ));
    }
    if !linux_isolation() {
        return Err(denied(
            "protected_launch.isolation.unverified",
            "required Linux isolation was not verified",
        ));
    }
    let bytes = bounded_bytes(protected_file(manifest, 0o444, false)?, 4096)?;
    let manifest_sha256 = SpecContentHash::from_bytes(&bytes);
    if &manifest_sha256 != expected {
        return Err(denied(
            "protected_launch.manifest.digest_mismatch",
            "manifest does not match the reviewed digest",
        ));
    }
    let config = Manifest::parse(&bytes)?;
    let (role, identity) = config.selected();
    if !matches_identity(identity.uid, identity.gid) {
        return Err(denied(
            "protected_launch.identity.mismatch",
            "current process does not match the selected role",
        ));
    }
    let directory = manifest.parent().ok_or_else(unprotected)?;
    let executable = protected_file(&directory.join("workflow-os"), 0o555, false)?;
    let installed = executable.metadata().map_err(|_| unprotected())?;
    let running = fs::metadata("/proc/self/exe").map_err(|_| unprotected())?;
    let wrong_executable = || {
        denied(
            "protected_launch.executable.mismatch",
            "running executable does not match the reviewed launch",
        )
    };
    if running.dev() != installed.dev() || running.ino() != installed.ino() {
        return Err(wrong_executable());
    }
    let executable_sha256 = executable_digest(executable)?;
    if executable_sha256 != config.executable_sha256 {
        return Err(wrong_executable());
    }
    Ok(ProtectedLaunchInspection {
        status: "verified_no_dispatch",
        role,
        uid: identity.uid,
        gid: identity.gid,
        manifest_sha256,
        executable_sha256,
        unverified: [
            "deployment",
            "custody",
            "ingress",
            "trusted_time",
            "provider_scope",
        ],
    })
}
