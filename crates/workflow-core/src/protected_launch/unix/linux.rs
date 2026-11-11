use std::fs::{self, File};
use std::io::Read as _;
use std::os::fd::AsRawFd;
use std::os::unix::fs::{FileTypeExt, MetadataExt};
use std::path::Path;

use nix::sys::socket::{getpeername, getsockname, getsockopt, sockopt, SockType, UnixAddr};
use serde::Deserialize;

use super::{
    bounded_bytes, denied, executable_digest, linux_isolation, matches_identity,
    only_launch_descriptors, only_standard_descriptors, protected_file, protected_parents,
    unprotected,
};
use crate::protected_launch::{
    ProtectedLaunchInspection, ProtectedLaunchPeerInspection, ProtectedReceiverDescriptorInspection,
};
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
    verify_launch(manifest, expected, only_standard_descriptors).map(|(launch, _, _)| launch)
}

const RECEIVER_DESCRIPTORS: [u32; 2] = [3, 4];

pub(in crate::protected_launch) fn inspect_receiver_descriptors(
    manifest: &Path,
    expected: &SpecContentHash,
) -> Result<ProtectedReceiverDescriptorInspection, WorkflowOsError> {
    let inventory = || only_launch_descriptors(&RECEIVER_DESCRIPTORS);
    let (launch, role, config) = verify_launch(manifest, expected, inventory)?;
    if !matches!((role, config), (Role::Receiver, Manifest::Shared { .. })) {
        return Err(denied(
            "protected_launch.receiver.role_unsupported",
            "receiver descriptor inspection requires a verified shared receiver launch",
        ));
    }
    let invalid = || {
        denied(
            "protected_launch.receiver.descriptors_unverified",
            "the inherited receiver descriptor set was not verified",
        )
    };
    let [first, second] = RECEIVER_DESCRIPTORS.map(|fd| receiver_descriptor(fd, launch.uid));
    let (first, second) = (first.ok_or_else(invalid)?, second.ok_or_else(invalid)?);
    if first == second
        || (0..=2).any(|fd| {
            !fs::metadata(format!("/proc/self/fd/{fd}"))
                .is_ok_and(|meta| ![first, second].contains(&(meta.dev(), meta.ino())))
        })
        || !inventory()
    {
        return Err(invalid());
    }
    Ok(ProtectedReceiverDescriptorInspection {
        status: "receiver_descriptor_metadata_verified_no_dispatch",
        launch,
        descriptor_count: 2,
    })
}

fn receiver_descriptor(fd: u32, uid: u32) -> Option<(u64, u64)> {
    let path = format!("/proc/self/fd/{fd}");
    let fingerprint = |meta: fs::Metadata| {
        (meta.is_file()
            && meta.uid() == uid
            && meta.mode() & 0o7777 == 0o400
            && meta.nlink() == 1
            && meta.len() == 32)
            .then_some((meta.dev(), meta.ino(), meta.ctime(), meta.ctime_nsec()))
    };
    let before = fingerprint(fs::metadata(&path).ok()?)?;
    // Read kernel-generated fdinfo, never open or read through the supplied file.
    let mut info = String::new();
    File::open(format!("/proc/self/fdinfo/{fd}"))
        .ok()?
        .take(4097)
        .read_to_string(&mut info)
        .ok()?;
    if info.len() > 4096 || !receiver_read_only_flags(&info) {
        return None;
    }
    (fingerprint(fs::metadata(path).ok()?)? == before).then_some((before.0, before.1))
}

fn receiver_read_only_flags(info: &str) -> bool {
    let mut fields = info.lines().filter_map(|line| line.strip_prefix("flags:"));
    let Some(flags) = fields
        .next()
        .and_then(|value| i32::from_str_radix(value.trim(), 8).ok())
    else {
        return false;
    };
    fields.next().is_none() && flags >= 0 && flags & (nix::libc::O_ACCMODE | nix::libc::O_PATH) == 0
}

#[cfg(test)]
#[test]
fn descriptor_flags_require_exactly_one_read_only_field() {
    assert!(receiver_read_only_flags("pos:\t0\nflags:\t0100000\n"));
    assert!(!receiver_read_only_flags(&format!(
        "flags:\t{:o}\n",
        nix::libc::O_PATH
    )));
    for invalid in [
        "pos:\t0\n",
        "flags:\t0100001\n",
        "flags:\t0100002\n",
        "flags:\t0100003\n",
        "flags:\t-100\n",
        "flags:\t9\n",
        "flags:\t0\nflags:\t0\n",
    ] {
        assert!(!receiver_read_only_flags(invalid), "{invalid:?}");
    }
}

pub(in crate::protected_launch) fn inspect_peer(
    manifest: &Path,
    expected: &SpecContentHash,
    named_endpoint: bool,
) -> Result<ProtectedLaunchPeerInspection, WorkflowOsError> {
    let (launch, local_role, config) =
        verify_launch(manifest, expected, only_standard_descriptors)?;
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
    if getsockopt(&socket, sockopt::SockType).ok() != Some(SockType::Stream) {
        return Err(unverified());
    }
    let peer_address = getpeername::<UnixAddr>(socket.as_raw_fd()).map_err(|_| unverified())?;
    let peer = getsockopt(&socket, sockopt::PeerCredentials).map_err(|_| unverified())?;
    let expected_peer = identities.for_role(peer_role);
    if peer.pid() <= 0 || peer.uid() != expected_peer.uid || peer.gid() != expected_peer.gid {
        return Err(denied(
            "protected_launch.peer.identity_mismatch",
            "connected peer does not match the reviewed counterpart",
        ));
    }
    let status = if named_endpoint {
        verify_named_endpoint(manifest, &identities, local_role, &socket, peer_address)?;
        "endpoint_verified_no_dispatch"
    } else {
        "peer_verified_no_dispatch"
    };
    Ok(ProtectedLaunchPeerInspection {
        status,
        launch,
        peer_role: peer_role.label(),
        peer_uid: peer.uid(),
        peer_gid: peer.gid(),
    })
}

fn verify_named_endpoint(
    manifest: &Path,
    identities: &Identities,
    role: Role,
    socket: &std::io::Stdin,
    peer_address: UnixAddr,
) -> Result<(), WorkflowOsError> {
    let unverified = || {
        denied(
            "protected_launch.endpoint.unverified",
            "connected endpoint does not match the protected launch pathname or access policy",
        )
    };
    let core_gid = identities.core.gid;
    if identities
        .all()
        .iter()
        .any(|(role, identity)| !matches!(role, Role::Core) && identity.gid == core_gid)
    {
        return Err(unverified());
    }
    let path = manifest
        .parent()
        .ok_or_else(unverified)?
        .join("peer/signer.sock");
    protected_parents(&path, false).map_err(|_| unverified())?;
    let metadata = || {
        let file = fs::symlink_metadata(&path).map_err(|_| unverified())?;
        if !file.file_type().is_socket()
            || file.uid() != 0
            || file.gid() != core_gid
            || file.nlink() != 1
            || file.mode() & 0o7777 != 0o620
        {
            return Err(unverified());
        }
        Ok((file.dev(), file.ino()))
    };
    let before = metadata()?;
    let address = match role {
        Role::Core => peer_address,
        Role::Signer => getsockname::<UnixAddr>(socket.as_raw_fd()).map_err(|_| unverified())?,
        _ => return Err(peer_role_unsupported()),
    };
    if address.path().map(Path::as_os_str) != Some(path.as_os_str()) || metadata()? != before {
        return Err(unverified());
    }
    Ok(())
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
    descriptors_verified: impl FnOnce() -> bool,
) -> Result<(ProtectedLaunchInspection, Role, Manifest), WorkflowOsError> {
    if nix::unistd::getuid().is_root() || nix::unistd::geteuid().is_root() {
        return Err(denied(
            "protected_launch.identity.root",
            "root launch is not permitted",
        ));
    }
    if !descriptors_verified() {
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
