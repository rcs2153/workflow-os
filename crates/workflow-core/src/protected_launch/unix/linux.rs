use std::fs;
use std::os::fd::AsRawFd;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

use nix::sys::socket::{getpeername, getsockopt, sockopt, SockType, UnixAddr};
use serde::Deserialize;

use super::{
    bounded_bytes, denied, executable_digest, linux_isolation, matches_identity,
    only_standard_descriptors, protected_file, unprotected,
};
use crate::protected_launch::{ProtectedLaunchInspection, ProtectedLaunchPeerInspection};
use crate::{SpecContentHash, WorkflowOsError};

#[derive(Deserialize)]
#[serde(tag = "version", deny_unknown_fields)]
enum Manifest {
    #[serde(rename = "workflow-os/protected-launch/v1")]
    Selected {
        executable_sha256: SpecContentHash,
        role: Role,
        identities: Identities,
    },
    #[serde(rename = "workflow-os/protected-launch/v2")]
    Shared {
        executable_sha256: SpecContentHash,
        identities: Identities,
    },
}

#[derive(Clone, Copy, Deserialize)]
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
        let identities = manifest.identities().all();
        if identities.iter().enumerate().any(|(i, (_, identity))| {
            identity.uid == 0
                || identity.gid == 0
                || identity.uid == u32::MAX
                || identity.gid == u32::MAX
                || identities[..i]
                    .iter()
                    .any(|(_, prior)| prior.uid == identity.uid)
        }) {
            return Err(invalid_manifest());
        }
        Ok(manifest)
    }

    fn identities(&self) -> &Identities {
        match self {
            Self::Selected { identities, .. } | Self::Shared { identities, .. } => identities,
        }
    }

    fn executable_sha256(&self) -> &SpecContentHash {
        match self {
            Self::Selected {
                executable_sha256, ..
            }
            | Self::Shared {
                executable_sha256, ..
            } => executable_sha256,
        }
    }

    fn current_role(&self) -> Option<(Role, &Identity)> {
        let matches =
            |(_, identity): &(Role, &Identity)| matches_identity(identity.uid, identity.gid);
        match self {
            Self::Selected {
                role, identities, ..
            } => Some((*role, identities.for_role(*role))).filter(matches),
            Self::Shared { identities, .. } => identities.all().into_iter().find(matches),
        }
    }
}

impl Identities {
    fn for_role(&self, role: Role) -> &Identity {
        match role {
            Role::Core => &self.core,
            Role::Signer => &self.signer,
            Role::Reader => &self.reader,
            Role::Receiver => &self.receiver,
            Role::Harness => &self.harness,
        }
    }

    fn all(&self) -> [(Role, &Identity); 5] {
        [
            Role::Core,
            Role::Signer,
            Role::Reader,
            Role::Receiver,
            Role::Harness,
        ]
        .map(|role| (role, self.for_role(role)))
    }
}

impl Role {
    const fn label(self) -> &'static str {
        match self {
            Self::Core => "core",
            Self::Signer => "signer",
            Self::Reader => "reader",
            Self::Receiver => "receiver",
            Self::Harness => "harness",
        }
    }
}

pub(in crate::protected_launch) fn inspect(
    manifest: &Path,
    expected: &SpecContentHash,
) -> Result<ProtectedLaunchInspection, WorkflowOsError> {
    verify_launch(manifest, expected).map(|(launch, _, _)| launch)
}

pub(in crate::protected_launch) fn inspect_peer(
    manifest: &Path,
    expected: &SpecContentHash,
) -> Result<ProtectedLaunchPeerInspection, WorkflowOsError> {
    let (launch, local_role, config) = verify_launch(manifest, expected)?;
    let (Manifest::Shared { identities, .. }, peer_role) = (
        config,
        match local_role {
            Role::Core => Role::Signer,
            Role::Signer => Role::Core,
            _ => return Err(peer_role_unsupported()),
        },
    ) else {
        return Err(peer_role_unsupported());
    };

    let socket = std::io::stdin();
    let unverified = || {
        denied(
            "protected_launch.peer.transport_unverified",
            "standard input is not a verified connected Unix stream",
        )
    };
    if getsockopt(&socket, sockopt::SockType).ok() != Some(SockType::Stream)
        || getpeername::<UnixAddr>(socket.as_raw_fd()).is_err()
    {
        return Err(unverified());
    }
    let peer = getsockopt(&socket, sockopt::PeerCredentials).map_err(|_| unverified())?;
    let expected_peer = identities.for_role(peer_role);
    if peer.pid() <= 0 || peer.uid() != expected_peer.uid || peer.gid() != expected_peer.gid {
        return Err(denied(
            "protected_launch.peer.identity_mismatch",
            "connected peer does not match the reviewed counterpart",
        ));
    }
    Ok(ProtectedLaunchPeerInspection {
        status: "peer_verified_no_dispatch",
        launch,
        peer_role: peer_role.label(),
        peer_uid: peer.uid(),
        peer_gid: peer.gid(),
    })
}

fn peer_role_unsupported() -> WorkflowOsError {
    denied(
        "protected_launch.peer.role_unsupported",
        "peer preflight requires a shared launch and the Core/signer pair",
    )
}

fn verify_launch(
    manifest: &Path,
    expected: &SpecContentHash,
) -> Result<(ProtectedLaunchInspection, Role, Manifest), WorkflowOsError> {
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
    let (role, identity) = config.current_role().ok_or_else(|| {
        denied(
            "protected_launch.identity.mismatch",
            "current process does not match the selected role",
        )
    })?;
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
    if &executable_sha256 != config.executable_sha256() {
        return Err(wrong_executable());
    }
    Ok((
        ProtectedLaunchInspection {
            status: "verified_no_dispatch",
            role: role.label(),
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
        },
        role,
        config,
    ))
}
