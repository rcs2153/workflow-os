//! Local, no-dispatch inspection of a protected launch on Linux.

use std::path::Path;

use serde::Serialize;

use crate::{SpecContentHash, WorkflowOsError};

#[cfg(any(target_os = "linux", all(test, target_os = "macos")))]
pub(crate) mod unix;

/// Verified facts about the current process, not deployment or admission proof.
#[derive(Debug, Serialize)]
pub struct ProtectedLaunchInspection {
    status: &'static str,
    role: &'static str,
    uid: u32,
    gid: u32,
    manifest_sha256: SpecContentHash,
    executable_sha256: SpecContentHash,
    unverified: [&'static str; 5],
}

/// Socket credentials observed at connection establishment, not authority.
#[derive(Debug, Serialize)]
pub struct ProtectedLaunchPeerInspection {
    status: &'static str,
    launch: ProtectedLaunchInspection,
    peer_role: &'static str,
    peer_uid: u32,
    peer_gid: u32,
}

/// Verifies the reviewed manifest, installed binary, and current Linux launch.
///
/// This command cannot open custody, contact a provider or authorize dispatch.
///
/// # Errors
///
/// Fails closed when the platform, manifest, file protection or process
/// isolation cannot be verified. Diagnostics do not echo paths or inputs.
pub fn inspect_protected_launch(
    manifest: &Path,
    expected: &SpecContentHash,
) -> Result<ProtectedLaunchInspection, WorkflowOsError> {
    #[cfg(target_os = "linux")]
    {
        unix::inspect(manifest, expected)
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (manifest, expected);
        Err(WorkflowOsError::invalid_state(
            "protected_launch.platform.unsupported",
            "protected-launch preflight requires Linux isolation evidence",
        ))
    }
}

/// Verifies a protected Linux launch and the Core/signer peer on standard input.
///
/// The caller supplies an already-connected Unix stream. Only its kernel
/// credentials are inspected; no payload is exchanged or authority returned.
///
/// # Errors
///
/// Fails closed without a verified v2 launch, a supported role pair, and
/// matching peer credentials on the connected stream.
pub fn inspect_protected_launch_peer(
    manifest: &Path,
    expected: &SpecContentHash,
) -> Result<ProtectedLaunchPeerInspection, WorkflowOsError> {
    #[cfg(target_os = "linux")]
    {
        unix::inspect_peer(manifest, expected)
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (manifest, expected);
        Err(WorkflowOsError::invalid_state(
            "protected_launch.platform.unsupported",
            "protected-launch preflight requires Linux isolation evidence",
        ))
    }
}
