//! Version-pinned `OpenShell` CLI compatibility boundary.
//!
//! This transport intentionally covers only machine-readable lifecycle and
//! effective-policy inspection available in `OpenShell` v0.0.101. It does not
//! implement [`crate::OpenShellNoWriteClient`]: the pinned CLI does not expose
//! the driver-observed immutable image identity, complete OCSF observations,
//! or machine-readable cleanup confirmation required by that contract.

#[cfg(test)]
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use serde::Deserialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use workflow_core::{
    HostedExecutionAttemptPosture, HostedExecutionErrorCategory, HostedExecutionInvocationError,
    HostedExecutionPolicyRevision, SpecContentHash, WorkflowOsError,
};

/// Reviewed `OpenShell` release consumed by this compatibility boundary.
pub const OPENSHELL_CLI_VERSION: &str = "0.0.101";

/// Reviewed upstream commit for [`OPENSHELL_CLI_VERSION`].
pub const OPENSHELL_UPSTREAM_COMMIT: &str = "8ddd98c3dff62619a3963f99ba1e055b67650e72";

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
const MAX_STREAM_BYTES: usize = 256 * 1024;

/// Explicit, immutable `OpenShell` CLI configuration.
#[derive(Clone, Eq, PartialEq)]
pub struct OpenShellCliTransportConfig {
    binary_path: PathBuf,
    expected_binary_digest: SpecContentHash,
    workspace: String,
    pinned_image: String,
}

impl OpenShellCliTransportConfig {
    /// Creates one pinned CLI configuration.
    ///
    /// # Errors
    ///
    /// Rejects a relative binary, malformed workspace, or mutable image.
    pub fn new(
        binary_path: PathBuf,
        expected_binary_digest: SpecContentHash,
        workspace: impl Into<String>,
        pinned_image: impl Into<String>,
    ) -> Result<Self, WorkflowOsError> {
        let workspace = workspace.into();
        let pinned_image = pinned_image.into();
        if !binary_path.is_absolute() {
            return Err(configuration_error());
        }
        validate_name(&workspace)?;
        validate_pinned_image(&pinned_image)?;
        Ok(Self {
            binary_path,
            expected_binary_digest,
            workspace,
            pinned_image,
        })
    }
}

impl fmt::Debug for OpenShellCliTransportConfig {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OpenShellCliTransportConfig")
            .field("binary_path", &"[REDACTED]")
            .field("expected_binary_digest", &"[REDACTED]")
            .field("workspace", &"[REDACTED]")
            .field("pinned_image", &"[REDACTED]")
            .field("expected_version", &OPENSHELL_CLI_VERSION)
            .finish()
    }
}

/// Bounded identity and lifecycle fields from `OpenShell` structured output.
#[derive(Clone, Eq, PartialEq)]
pub struct OpenShellCliSandboxState {
    id: String,
    name: String,
    phase: String,
    resource_version: u64,
    current_policy_version: u32,
    revision: Option<u32>,
    policy_source: Option<String>,
}

impl OpenShellCliSandboxState {
    /// Returns the stable `OpenShell` sandbox ID.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the current lifecycle phase.
    #[must_use]
    pub fn phase(&self) -> &str {
        &self.phase
    }
}

impl fmt::Debug for OpenShellCliSandboxState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OpenShellCliSandboxState")
            .field("identity", &"[REDACTED]")
            .field("phase", &self.phase)
            .field("current_policy_version", &self.current_policy_version)
            .field("revision", &self.revision)
            .field("policy_source", &self.policy_source)
            .finish_non_exhaustive()
    }
}

/// Effective `OpenShell` policy binding from machine-readable CLI output.
#[derive(Clone, Eq, PartialEq)]
pub struct OpenShellCliEffectivePolicy {
    version: u32,
    revision: HostedExecutionPolicyRevision,
    provider_hash: String,
    canonical_policy_hash: SpecContentHash,
    config_revision: u64,
    policy_source: String,
}

impl OpenShellCliEffectivePolicy {
    /// Returns the loaded policy revision.
    #[must_use]
    pub const fn revision(&self) -> &HostedExecutionPolicyRevision {
        &self.revision
    }

    /// Returns the numeric loaded policy version reported by `OpenShell`.
    #[must_use]
    pub const fn version(&self) -> u32 {
        self.version
    }

    /// Returns the canonical digest of the full effective policy payload.
    #[must_use]
    pub const fn canonical_policy_hash(&self) -> &SpecContentHash {
        &self.canonical_policy_hash
    }
}

impl fmt::Debug for OpenShellCliEffectivePolicy {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OpenShellCliEffectivePolicy")
            .field("revision", &self.revision)
            .field("provider_hash", &"[REDACTED]")
            .field("canonical_policy_hash", &"[REDACTED]")
            .field("config_revision", &self.config_revision)
            .field("policy_source", &self.policy_source)
            .finish_non_exhaustive()
    }
}

/// Drift-detecting reconciliation of reviewed `OpenShell` CLI observations.
///
/// This is compatibility data, not atomic runtime attestation. The pinned CLI
/// does not expose one atomic snapshot or all facts required by
/// [`crate::OpenShellNoWriteClient`].
#[derive(Clone, Eq, PartialEq)]
pub struct OpenShellCliReconciledSnapshot {
    sandbox: OpenShellCliSandboxState,
    effective_policy: OpenShellCliEffectivePolicy,
}

impl OpenShellCliReconciledSnapshot {
    /// Returns the stable sandbox ID from the reconciled observations.
    #[must_use]
    pub fn sandbox_id(&self) -> &str {
        self.sandbox.id()
    }

    /// Returns the reconciled effective policy revision.
    #[must_use]
    pub const fn policy_revision(&self) -> &HostedExecutionPolicyRevision {
        self.effective_policy.revision()
    }

    /// Returns the canonical digest of the full effective policy payload.
    #[must_use]
    pub const fn canonical_policy_hash(&self) -> &SpecContentHash {
        self.effective_policy.canonical_policy_hash()
    }
}

impl fmt::Debug for OpenShellCliReconciledSnapshot {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OpenShellCliReconciledSnapshot")
            .field("identity", &"[REDACTED]")
            .field("phase", &self.sandbox.phase)
            .field("policy_revision", &self.effective_policy.revision)
            .field("policy_hash", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

/// Pinned subprocess transport for `OpenShell` structured CLI operations.
pub struct OpenShellCliTransport {
    config: OpenShellCliTransportConfig,
    runner: Arc<dyn OpenShellCommandRunner>,
}

impl OpenShellCliTransport {
    /// Creates a transport backed by the local process boundary.
    #[must_use]
    pub fn new(config: OpenShellCliTransportConfig) -> Self {
        Self {
            config,
            runner: Arc::new(ProcessOpenShellCommandRunner),
        }
    }

    #[cfg(test)]
    fn with_runner(
        config: OpenShellCliTransportConfig,
        runner: Arc<dyn OpenShellCommandRunner>,
    ) -> Self {
        Self { config, runner }
    }

    /// Verifies the exact reviewed `OpenShell` CLI version.
    ///
    /// # Errors
    ///
    /// Fails closed on process, timeout, output-bound, or version mismatch.
    pub fn verify_version(&self) -> Result<(), HostedExecutionInvocationError> {
        let output = self.run(&[OsString::from("--version")])?;
        let text = std::str::from_utf8(&output.stdout)
            .map_err(|_| protocol_error(HostedExecutionAttemptPosture::MayHaveStarted))?;
        if text.trim() != format!("openshell {OPENSHELL_CLI_VERSION}") {
            return Err(protocol_error(
                HostedExecutionAttemptPosture::MayHaveStarted,
            ));
        }
        Ok(())
    }

    /// Creates a sandbox from an absolute policy path and immutable image.
    ///
    /// # Errors
    ///
    /// Fails closed on invalid input or unsupported structured output.
    pub fn create_sandbox(
        &self,
        name: &str,
        policy_path: &Path,
    ) -> Result<OpenShellCliSandboxState, HostedExecutionInvocationError> {
        validate_runtime_name(name)?;
        if !policy_path.is_absolute() {
            return Err(protocol_error(HostedExecutionAttemptPosture::NotStarted));
        }
        self.verify_version()?;
        let output = self.run(&[
            OsString::from("--workspace"),
            OsString::from(&self.config.workspace),
            OsString::from("sandbox"),
            OsString::from("create"),
            OsString::from("--name"),
            OsString::from(name),
            OsString::from("--from"),
            OsString::from(&self.config.pinned_image),
            OsString::from("--policy"),
            policy_path.as_os_str().to_os_string(),
            OsString::from("--no-auto-providers"),
            OsString::from("--approval-mode"),
            OsString::from("manual"),
            OsString::from("--output"),
            OsString::from("json"),
        ])?;
        parse_sandbox_state(&output.stdout, name, false)
    }

    /// Reads one sandbox through the pinned structured output shape.
    ///
    /// # Errors
    ///
    /// Fails closed on invalid identity or unsupported response shape.
    pub fn inspect_sandbox(
        &self,
        name: &str,
    ) -> Result<OpenShellCliSandboxState, HostedExecutionInvocationError> {
        validate_runtime_name(name)?;
        self.verify_version()?;
        let output = self.run(&[
            OsString::from("--workspace"),
            OsString::from(&self.config.workspace),
            OsString::from("sandbox"),
            OsString::from("get"),
            OsString::from(name),
            OsString::from("--output"),
            OsString::from("json"),
        ])?;
        parse_sandbox_state(&output.stdout, name, true)
    }

    /// Reads the full effective policy through structured output.
    ///
    /// # Errors
    ///
    /// Fails closed unless the policy is effective, loaded, and complete.
    pub fn inspect_effective_policy(
        &self,
        name: &str,
    ) -> Result<OpenShellCliEffectivePolicy, HostedExecutionInvocationError> {
        validate_runtime_name(name)?;
        self.verify_version()?;
        let output = self.run(&[
            OsString::from("--workspace"),
            OsString::from(&self.config.workspace),
            OsString::from("policy"),
            OsString::from("get"),
            OsString::from(name),
            OsString::from("--full"),
            OsString::from("--output"),
            OsString::from("json"),
        ])?;
        parse_effective_policy(&output.stdout, name)
    }

    /// Reconciles detailed sandbox state around one effective-policy read.
    ///
    /// This detects state or policy drift visible through the pinned CLI. It
    /// does not claim an atomic snapshot and is not execution attestation.
    ///
    /// # Errors
    ///
    /// Fails closed when the before/after sandbox observations differ or the
    /// policy version/source does not match the sandbox detail.
    pub fn inspect_reconciled_sandbox(
        &self,
        name: &str,
    ) -> Result<OpenShellCliReconciledSnapshot, HostedExecutionInvocationError> {
        let before = self.inspect_sandbox(name)?;
        let effective_policy = self.inspect_effective_policy(name)?;
        let after = self.inspect_sandbox(name)?;
        if before != after
            || after.current_policy_version != effective_policy.version
            || after.revision != Some(effective_policy.version)
            || after.policy_source.as_deref() != Some(effective_policy.policy_source.as_str())
        {
            return Err(protocol_error(
                HostedExecutionAttemptPosture::MayHaveStarted,
            ));
        }
        Ok(OpenShellCliReconciledSnapshot {
            sandbox: after,
            effective_policy,
        })
    }

    fn run<const N: usize>(
        &self,
        args: &[OsString; N],
    ) -> Result<CommandOutput, HostedExecutionInvocationError> {
        self.verify_binary_digest(HostedExecutionAttemptPosture::NotStarted)?;
        let output = self.runner.run(
            &self.config.binary_path,
            args,
            DEFAULT_TIMEOUT,
            MAX_STREAM_BYTES,
        )?;
        self.verify_binary_digest(HostedExecutionAttemptPosture::MayHaveStarted)?;
        if !output.stderr.is_empty() {
            return Err(protocol_error(
                HostedExecutionAttemptPosture::MayHaveStarted,
            ));
        }
        Ok(output)
    }

    fn verify_binary_digest(
        &self,
        posture: HostedExecutionAttemptPosture,
    ) -> Result<(), HostedExecutionInvocationError> {
        let observed = hash_file(&self.config.binary_path, posture)?;
        if observed != self.config.expected_binary_digest {
            return Err(protocol_error(posture));
        }
        Ok(())
    }
}

impl fmt::Debug for OpenShellCliTransport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OpenShellCliTransport")
            .field("config", &self.config)
            .finish_non_exhaustive()
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SandboxWire {
    id: String,
    name: String,
    workspace: String,
    labels: serde_json::Map<String, Value>,
    annotations: serde_json::Map<String, Value>,
    resource_version: u64,
    created_at: String,
    phase: String,
    current_policy_version: u32,
    #[serde(default)]
    policy_source: Option<String>,
    #[serde(default)]
    revision: Option<u32>,
    #[serde(default)]
    policy: Option<Value>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EffectivePolicyWire {
    scope: String,
    sandbox: String,
    version: u32,
    active_version: u32,
    hash: String,
    status: String,
    config_revision: u64,
    policy_source: String,
    #[serde(default)]
    global_policy_version: Option<u32>,
    policy: Value,
}

fn parse_sandbox_state(
    bytes: &[u8],
    expected_name: &str,
    require_detail: bool,
) -> Result<OpenShellCliSandboxState, HostedExecutionInvocationError> {
    let wire: SandboxWire = serde_json::from_slice(bytes)
        .map_err(|_| protocol_error(HostedExecutionAttemptPosture::MayHaveStarted))?;
    if wire.id.is_empty()
        || wire.name != expected_name
        || wire.workspace.is_empty()
        || wire.phase.is_empty()
        || wire.resource_version == 0
        || wire.created_at.is_empty()
        || !wire.labels.values().all(Value::is_string)
        || !wire.annotations.values().all(Value::is_string)
        || (require_detail
            && (!matches!(wire.policy_source.as_deref(), Some("sandbox" | "global"))
                || wire.revision != Some(wire.current_policy_version)
                || wire.policy.is_none()))
    {
        return Err(protocol_error(
            HostedExecutionAttemptPosture::MayHaveStarted,
        ));
    }
    Ok(OpenShellCliSandboxState {
        id: wire.id,
        name: wire.name,
        phase: wire.phase,
        resource_version: wire.resource_version,
        current_policy_version: wire.current_policy_version,
        revision: wire.revision,
        policy_source: wire.policy_source,
    })
}

fn parse_effective_policy(
    bytes: &[u8],
    expected_name: &str,
) -> Result<OpenShellCliEffectivePolicy, HostedExecutionInvocationError> {
    let wire: EffectivePolicyWire = serde_json::from_slice(bytes)
        .map_err(|_| protocol_error(HostedExecutionAttemptPosture::MayHaveStarted))?;
    if wire.scope != "sandbox"
        || wire.sandbox != expected_name
        || wire.status != "effective"
        || wire.version == 0
        || wire.version != wire.active_version
        || wire.hash.is_empty()
        || wire.config_revision == 0
        || !matches!(wire.policy_source.as_str(), "sandbox" | "global")
        || wire.policy.is_null()
        || (wire.policy_source == "global" && wire.global_policy_version != Some(wire.version))
    {
        return Err(protocol_error(
            HostedExecutionAttemptPosture::MayHaveStarted,
        ));
    }
    let policy_bytes = serde_json::to_vec(&wire.policy)
        .map_err(|_| protocol_error(HostedExecutionAttemptPosture::MayHaveStarted))?;
    let revision = HostedExecutionPolicyRevision::new(format!("revision/{}", wire.version))
        .map_err(|_| protocol_error(HostedExecutionAttemptPosture::MayHaveStarted))?;
    Ok(OpenShellCliEffectivePolicy {
        version: wire.version,
        revision,
        provider_hash: wire.hash,
        canonical_policy_hash: SpecContentHash::from_bytes(policy_bytes),
        config_revision: wire.config_revision,
        policy_source: wire.policy_source,
    })
}

fn hash_file(
    path: &Path,
    posture: HostedExecutionAttemptPosture,
) -> Result<SpecContentHash, HostedExecutionInvocationError> {
    let mut file = File::open(path).map_err(|_| transport_error(posture))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 16 * 1024];
    loop {
        let read = file
            .read(&mut buffer)
            .map_err(|_| transport_error(posture))?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    SpecContentHash::new(format!("{:x}", hasher.finalize())).map_err(|_| protocol_error(posture))
}

fn validate_name(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty()
        || value.len() > 63
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
    {
        return Err(configuration_error());
    }
    Ok(())
}

fn validate_runtime_name(value: &str) -> Result<(), HostedExecutionInvocationError> {
    validate_name(value).map_err(|_| protocol_error(HostedExecutionAttemptPosture::NotStarted))
}

fn validate_pinned_image(value: &str) -> Result<(), WorkflowOsError> {
    let Some((repository, digest)) = value.rsplit_once("@sha256:") else {
        return Err(configuration_error());
    };
    if repository.is_empty()
        || repository.len() > 256
        || repository.bytes().any(|byte| byte.is_ascii_whitespace())
        || SpecContentHash::new(digest).is_err()
    {
        return Err(configuration_error());
    }
    Ok(())
}

fn configuration_error() -> WorkflowOsError {
    WorkflowOsError::validation(
        "hosted.openshell.cli.configuration.invalid",
        "OpenShell CLI configuration is invalid",
    )
}

const fn protocol_error(posture: HostedExecutionAttemptPosture) -> HostedExecutionInvocationError {
    HostedExecutionInvocationError::new(HostedExecutionErrorCategory::Protocol, posture)
}

trait OpenShellCommandRunner: Send + Sync {
    fn run(
        &self,
        binary: &Path,
        args: &[OsString],
        timeout: Duration,
        max_stream_bytes: usize,
    ) -> Result<CommandOutput, HostedExecutionInvocationError>;
}

struct ProcessOpenShellCommandRunner;

impl OpenShellCommandRunner for ProcessOpenShellCommandRunner {
    fn run(
        &self,
        binary: &Path,
        args: &[OsString],
        timeout: Duration,
        max_stream_bytes: usize,
    ) -> Result<CommandOutput, HostedExecutionInvocationError> {
        let mut child = Command::new(binary)
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|_| transport_error(HostedExecutionAttemptPosture::NotStarted))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| transport_error(HostedExecutionAttemptPosture::MayHaveStarted))?;
        let stderr = child
            .stderr
            .take()
            .ok_or_else(|| transport_error(HostedExecutionAttemptPosture::MayHaveStarted))?;
        let stdout_reader = read_bounded(stdout, max_stream_bytes);
        let stderr_reader = read_bounded(stderr, max_stream_bytes);
        let deadline = Instant::now() + timeout;
        let status = loop {
            match child.try_wait() {
                Ok(Some(status)) => break status,
                Ok(None) if Instant::now() < deadline => thread::sleep(Duration::from_millis(10)),
                Ok(None) => {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err(HostedExecutionInvocationError::new(
                        HostedExecutionErrorCategory::Timeout,
                        HostedExecutionAttemptPosture::MayHaveStarted,
                    ));
                }
                Err(_) => {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err(transport_error(
                        HostedExecutionAttemptPosture::MayHaveStarted,
                    ));
                }
            }
        };
        let stdout = join_bounded(stdout_reader, max_stream_bytes)?;
        let stderr = join_bounded(stderr_reader, max_stream_bytes)?;
        if !status.success() {
            return Err(transport_error(
                HostedExecutionAttemptPosture::MayHaveStarted,
            ));
        }
        Ok(CommandOutput { stdout, stderr })
    }
}

fn read_bounded(
    mut stream: impl Read + Send + 'static,
    max_bytes: usize,
) -> thread::JoinHandle<io::Result<Vec<u8>>> {
    thread::spawn(move || {
        let mut bytes = Vec::with_capacity(max_bytes.min(8192));
        stream
            .by_ref()
            .take((max_bytes + 1) as u64)
            .read_to_end(&mut bytes)?;
        Ok(bytes)
    })
}

fn join_bounded(
    handle: thread::JoinHandle<io::Result<Vec<u8>>>,
    max_bytes: usize,
) -> Result<Vec<u8>, HostedExecutionInvocationError> {
    let bytes = handle
        .join()
        .map_err(|_| transport_error(HostedExecutionAttemptPosture::MayHaveStarted))?
        .map_err(|_| transport_error(HostedExecutionAttemptPosture::MayHaveStarted))?;
    if bytes.len() > max_bytes {
        return Err(protocol_error(
            HostedExecutionAttemptPosture::MayHaveStarted,
        ));
    }
    Ok(bytes)
}

const fn transport_error(posture: HostedExecutionAttemptPosture) -> HostedExecutionInvocationError {
    HostedExecutionInvocationError::new(HostedExecutionErrorCategory::Transport, posture)
}

struct CommandOutput {
    stdout: Vec<u8>,
    #[allow(dead_code)]
    stderr: Vec<u8>,
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod tests {
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::Mutex;

    use super::*;

    const CREATE_JSON: &str = r#"{
        "id":"sandbox-id-1","name":"workflow-os-proof","workspace":"default",
        "labels":{},"annotations":{},"resource_version":1,
        "created_at":"2026-08-08T00:00:00Z","phase":"ready",
        "current_policy_version":7
    }"#;
    const DETAIL_JSON: &str = r#"{
        "id":"sandbox-id-1","name":"workflow-os-proof","workspace":"default",
        "labels":{},"annotations":{},"resource_version":2,
        "created_at":"2026-08-08T00:00:00Z","phase":"ready",
        "current_policy_version":7,"policy_source":"sandbox","revision":7,
        "policy":{"filesystem":{"read_only":["/workspace"]}}
    }"#;
    const POLICY_JSON: &str = r#"{
        "scope":"sandbox","sandbox":"workflow-os-proof","version":7,
        "active_version":7,"hash":"upstream-policy-hash","status":"effective",
        "config_revision":12,"policy_source":"sandbox",
        "policy":{"filesystem":{"read_only":["/workspace"]},"network":{"default":"deny"}}
    }"#;
    const TEST_BINARY_BYTES: &[u8] = b"reviewed-openshell-test-binary";
    static NEXT_BINARY_ID: AtomicU64 = AtomicU64::new(1);

    struct ScriptedRunner {
        outputs: Mutex<Vec<CommandOutput>>,
        calls: Mutex<Vec<Vec<OsString>>>,
    }

    struct MutatingRunner;

    struct RemovingRunner;

    impl OpenShellCommandRunner for MutatingRunner {
        fn run(
            &self,
            binary: &Path,
            args: &[OsString],
            _timeout: Duration,
            _max_stream_bytes: usize,
        ) -> Result<CommandOutput, HostedExecutionInvocationError> {
            if args.len() == 1 && args[0] == "--version" {
                return Ok(CommandOutput {
                    stdout: b"openshell 0.0.101\n".to_vec(),
                    stderr: Vec::new(),
                });
            }
            std::fs::write(binary, b"changed-openshell-test-binary")
                .unwrap_or_else(|error| panic!("failed to mutate test binary: {error}"));
            Ok(CommandOutput {
                stdout: CREATE_JSON.as_bytes().to_vec(),
                stderr: Vec::new(),
            })
        }
    }

    impl OpenShellCommandRunner for RemovingRunner {
        fn run(
            &self,
            binary: &Path,
            args: &[OsString],
            _timeout: Duration,
            _max_stream_bytes: usize,
        ) -> Result<CommandOutput, HostedExecutionInvocationError> {
            if args.len() == 1 && args[0] == "--version" {
                return Ok(CommandOutput {
                    stdout: b"openshell 0.0.101\n".to_vec(),
                    stderr: Vec::new(),
                });
            }
            std::fs::remove_file(binary)
                .unwrap_or_else(|error| panic!("failed to remove test binary: {error}"));
            Ok(CommandOutput {
                stdout: CREATE_JSON.as_bytes().to_vec(),
                stderr: Vec::new(),
            })
        }
    }

    impl ScriptedRunner {
        fn new(outputs: &[&str]) -> Self {
            Self {
                outputs: Mutex::new(
                    outputs
                        .iter()
                        .rev()
                        .map(|stdout| CommandOutput {
                            stdout: stdout.as_bytes().to_vec(),
                            stderr: Vec::new(),
                        })
                        .collect(),
                ),
                calls: Mutex::new(Vec::new()),
            }
        }

        fn new_with_stderr(outputs: &[&str], stderr: &[&str]) -> Self {
            assert_eq!(outputs.len(), stderr.len());
            Self {
                outputs: Mutex::new(
                    outputs
                        .iter()
                        .zip(stderr)
                        .rev()
                        .map(|(stdout, stderr)| CommandOutput {
                            stdout: stdout.as_bytes().to_vec(),
                            stderr: stderr.as_bytes().to_vec(),
                        })
                        .collect(),
                ),
                calls: Mutex::new(Vec::new()),
            }
        }
    }

    impl OpenShellCommandRunner for ScriptedRunner {
        fn run(
            &self,
            _binary: &Path,
            args: &[OsString],
            _timeout: Duration,
            _max_stream_bytes: usize,
        ) -> Result<CommandOutput, HostedExecutionInvocationError> {
            self.calls
                .lock()
                .unwrap_or_else(|error| panic!("{error}"))
                .push(args.to_vec());
            let output = self
                .outputs
                .lock()
                .unwrap_or_else(|error| panic!("{error}"))
                .pop()
                .unwrap_or_else(|| panic!("missing scripted output"));
            Ok(output)
        }
    }

    fn test_binary_path(label: &str) -> PathBuf {
        let id = NEXT_BINARY_ID.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "workflow-os-openshell-{label}-{}-{id}",
            std::process::id()
        ));
        std::fs::write(&path, TEST_BINARY_BYTES)
            .unwrap_or_else(|error| panic!("failed to write test binary: {error}"));
        path
    }

    fn config() -> OpenShellCliTransportConfig {
        OpenShellCliTransportConfig::new(
            test_binary_path("reviewed"),
            SpecContentHash::from_bytes(TEST_BINARY_BYTES),
            "default",
            format!("registry.example/openshell@sha256:{}", "a".repeat(64)),
        )
        .unwrap_or_else(|error| panic!("{error}"))
    }

    #[test]
    fn pinned_transport_parses_reviewed_response_shapes() {
        let runner = Arc::new(ScriptedRunner::new(&[
            "openshell 0.0.101\n",
            CREATE_JSON,
            "openshell 0.0.101\n",
            DETAIL_JSON,
            "openshell 0.0.101\n",
            POLICY_JSON,
        ]));
        let transport = OpenShellCliTransport::with_runner(config(), runner.clone());

        let created = transport
            .create_sandbox("workflow-os-proof", Path::new("/tmp/policy.yml"))
            .unwrap_or_else(|error| panic!("{error:?}"));
        let inspected = transport
            .inspect_sandbox("workflow-os-proof")
            .unwrap_or_else(|error| panic!("{error:?}"));
        let policy = transport
            .inspect_effective_policy("workflow-os-proof")
            .unwrap_or_else(|error| panic!("{error:?}"));

        assert_eq!(created.id(), "sandbox-id-1");
        assert_eq!(inspected.phase(), "ready");
        assert_eq!(policy.revision().as_str(), "revision/7");
        let calls = runner
            .calls
            .lock()
            .unwrap_or_else(|error| panic!("{error}"));
        assert_eq!(calls.len(), 6);
        assert_eq!(calls[0], [OsString::from("--version")]);
        assert_eq!(calls[2], [OsString::from("--version")]);
        assert_eq!(calls[4], [OsString::from("--version")]);
        let policy_args: Vec<&OsStr> = calls[5].iter().map(OsString::as_os_str).collect();
        assert!(policy_args
            .windows(3)
            .any(|args| { args == ["policy", "get", "workflow-os-proof"] }));
        assert!(!policy_args.iter().any(|arg| *arg == "--effective"));
        assert!(!policy_args.iter().any(|arg| *arg == "--sandbox"));
    }

    #[test]
    fn create_uses_fixed_no_provider_manual_approval_arguments() {
        let runner = Arc::new(ScriptedRunner::new(&["openshell 0.0.101\n", CREATE_JSON]));
        let transport = OpenShellCliTransport::with_runner(config(), runner.clone());
        transport
            .create_sandbox("workflow-os-proof", Path::new("/tmp/policy.yml"))
            .unwrap_or_else(|error| panic!("{error:?}"));
        let calls = runner
            .calls
            .lock()
            .unwrap_or_else(|error| panic!("{error}"));
        let args: Vec<&OsStr> = calls[1].iter().map(OsString::as_os_str).collect();
        assert!(args
            .windows(2)
            .any(|pair| pair == ["--approval-mode", "manual"]));
        assert!(args.iter().any(|arg| *arg == "--no-auto-providers"));
        assert!(!args.iter().any(|arg| *arg == "--provider"));
        assert!(!args.iter().any(|arg| *arg == "--env"));
    }

    #[test]
    fn mutable_image_and_relative_binary_are_rejected_without_leakage() {
        let marker = "token-private-marker";
        let error = OpenShellCliTransportConfig::new(
            PathBuf::from(marker),
            SpecContentHash::from_bytes(TEST_BINARY_BYTES),
            "default",
            format!("registry.example/image:latest-{marker}"),
        )
        .expect_err("mutable configuration should fail");
        assert_eq!(error.code(), "hosted.openshell.cli.configuration.invalid");
        assert!(!format!("{error:?}").contains(marker));
    }

    #[test]
    fn version_mismatch_blocks_operation_before_sandbox_creation() {
        let runner = Arc::new(ScriptedRunner::new(&["openshell 0.0.102\n"]));
        let transport = OpenShellCliTransport::with_runner(config(), runner.clone());
        let error = transport
            .create_sandbox("workflow-os-proof", Path::new("/tmp/policy.yml"))
            .expect_err("unreviewed version should fail");
        assert_eq!(error.category(), HostedExecutionErrorCategory::Protocol);
        assert_eq!(
            error.attempt_posture(),
            HostedExecutionAttemptPosture::MayHaveStarted
        );
        assert_eq!(runner.calls.lock().map_or(0, |calls| calls.len()), 1);
    }

    #[test]
    fn binary_digest_mismatch_blocks_before_invocation_without_leaking() {
        let marker = "private-binary-marker";
        let path = test_binary_path(marker);
        let config = OpenShellCliTransportConfig::new(
            path,
            SpecContentHash::from_bytes(b"different-reviewed-binary"),
            "default",
            format!("registry.example/image@sha256:{}", "a".repeat(64)),
        )
        .unwrap_or_else(|error| panic!("{error}"));
        let runner = Arc::new(ScriptedRunner::new(&["openshell 0.0.101\n"]));
        let transport = OpenShellCliTransport::with_runner(config, runner.clone());

        let error = transport
            .verify_version()
            .expect_err("unexpected binary digest should fail");

        assert_eq!(error.category(), HostedExecutionErrorCategory::Protocol);
        assert_eq!(
            error.attempt_posture(),
            HostedExecutionAttemptPosture::NotStarted
        );
        assert_eq!(runner.calls.lock().map_or(0, |calls| calls.len()), 0);
        assert!(!format!("{error:?}").contains(marker));
    }

    #[test]
    fn binary_change_during_invocation_fails_closed() {
        let transport = OpenShellCliTransport::with_runner(config(), Arc::new(MutatingRunner));

        let error = transport
            .create_sandbox("workflow-os-proof", Path::new("/tmp/policy.yml"))
            .expect_err("binary replacement should fail after invocation");

        assert_eq!(error.category(), HostedExecutionErrorCategory::Protocol);
        assert_eq!(
            error.attempt_posture(),
            HostedExecutionAttemptPosture::MayHaveStarted
        );
    }

    #[test]
    fn binary_read_failure_after_invocation_requires_reconciliation() {
        let transport = OpenShellCliTransport::with_runner(config(), Arc::new(RemovingRunner));

        let error = transport
            .create_sandbox("workflow-os-proof", Path::new("/tmp/policy.yml"))
            .expect_err("missing binary after invocation should fail");

        assert_eq!(error.category(), HostedExecutionErrorCategory::Transport);
        assert_eq!(
            error.attempt_posture(),
            HostedExecutionAttemptPosture::MayHaveStarted
        );
    }

    #[test]
    fn successful_stderr_fails_closed_without_copying_warning() {
        let marker = "provider-warning-private-marker";
        let runner = Arc::new(ScriptedRunner::new_with_stderr(
            &["openshell 0.0.101\n", CREATE_JSON],
            &["", marker],
        ));
        let transport = OpenShellCliTransport::with_runner(config(), runner.clone());

        let error = transport
            .create_sandbox("workflow-os-proof", Path::new("/tmp/policy.yml"))
            .expect_err("successful stderr should require review");

        assert_eq!(error.category(), HostedExecutionErrorCategory::Protocol);
        assert_eq!(
            error.attempt_posture(),
            HostedExecutionAttemptPosture::MayHaveStarted
        );
        assert_eq!(runner.calls.lock().map_or(0, |calls| calls.len()), 2);
        assert!(!format!("{error:?}").contains(marker));
    }

    #[test]
    fn static_input_failure_proves_operation_not_started() {
        let runner = Arc::new(ScriptedRunner::new(&[]));
        let transport = OpenShellCliTransport::with_runner(config(), runner.clone());

        let error = transport
            .create_sandbox("invalid sandbox name", Path::new("relative-policy.yml"))
            .expect_err("invalid static input should fail before invocation");

        assert_eq!(
            error.attempt_posture(),
            HostedExecutionAttemptPosture::NotStarted
        );
        assert_eq!(runner.calls.lock().map_or(0, |calls| calls.len()), 0);
    }

    #[test]
    fn malformed_create_output_requires_reconciliation() {
        let runner = Arc::new(ScriptedRunner::new(&["openshell 0.0.101\n", "{}"]));
        let transport = OpenShellCliTransport::with_runner(config(), runner);

        let error = transport
            .create_sandbox("workflow-os-proof", Path::new("/tmp/policy.yml"))
            .expect_err("malformed create output should fail");

        assert_eq!(error.category(), HostedExecutionErrorCategory::Protocol);
        assert_eq!(
            error.attempt_posture(),
            HostedExecutionAttemptPosture::MayHaveStarted
        );
    }

    #[test]
    fn output_bound_failure_requires_reconciliation() {
        let exact = read_bounded(std::io::Cursor::new(vec![b'x'; 8]), 8);
        assert_eq!(
            join_bounded(exact, 8).unwrap_or_else(|error| panic!("{error:?}")),
            vec![b'x'; 8]
        );
        let reader = read_bounded(std::io::Cursor::new(vec![b'x'; 9]), 8);

        let error = join_bounded(reader, 8).expect_err("oversized output should fail");

        assert_eq!(error.category(), HostedExecutionErrorCategory::Protocol);
        assert_eq!(
            error.attempt_posture(),
            HostedExecutionAttemptPosture::MayHaveStarted
        );
    }

    #[test]
    fn unknown_or_incomplete_security_response_fails_closed() {
        let unknown = DETAIL_JSON.replace(
            "\"current_policy_version\":7",
            "\"current_policy_version\":7,\"degraded\":true",
        );
        let runner = Arc::new(ScriptedRunner::new(&["openshell 0.0.101\n", &unknown]));
        let transport = OpenShellCliTransport::with_runner(config(), runner);
        let error = transport
            .inspect_sandbox("workflow-os-proof")
            .expect_err("unknown security field should fail");
        assert_eq!(error.category(), HostedExecutionErrorCategory::Protocol);
        assert_eq!(
            error.attempt_posture(),
            HostedExecutionAttemptPosture::MayHaveStarted
        );
    }

    #[test]
    fn detailed_sandbox_policy_revision_and_source_must_be_coherent() {
        let mismatched = DETAIL_JSON.replace("\"revision\":7", "\"revision\":6");
        let runner = Arc::new(ScriptedRunner::new(&["openshell 0.0.101\n", &mismatched]));
        let transport = OpenShellCliTransport::with_runner(config(), runner);
        let error = transport
            .inspect_sandbox("workflow-os-proof")
            .expect_err("mismatched detailed policy revision should fail");
        assert_eq!(error.category(), HostedExecutionErrorCategory::Protocol);

        let unsupported = DETAIL_JSON.replace(
            "\"policy_source\":\"sandbox\"",
            "\"policy_source\":\"unspecified\"",
        );
        let runner = Arc::new(ScriptedRunner::new(&["openshell 0.0.101\n", &unsupported]));
        let transport = OpenShellCliTransport::with_runner(config(), runner);
        let error = transport
            .inspect_sandbox("workflow-os-proof")
            .expect_err("unsupported policy source should fail");
        assert_eq!(error.category(), HostedExecutionErrorCategory::Protocol);
    }

    #[test]
    fn effective_policy_identity_and_loaded_version_must_match() {
        let mismatched = POLICY_JSON.replace("\"active_version\":7", "\"active_version\":6");
        let runner = Arc::new(ScriptedRunner::new(&["openshell 0.0.101\n", &mismatched]));
        let transport = OpenShellCliTransport::with_runner(config(), runner);
        let error = transport
            .inspect_effective_policy("workflow-os-proof")
            .expect_err("stale effective policy should fail");
        assert_eq!(error.category(), HostedExecutionErrorCategory::Protocol);
    }

    #[test]
    fn reconciled_snapshot_requires_stable_matching_observations() {
        let runner = Arc::new(ScriptedRunner::new(&[
            "openshell 0.0.101\n",
            DETAIL_JSON,
            "openshell 0.0.101\n",
            POLICY_JSON,
            "openshell 0.0.101\n",
            DETAIL_JSON,
        ]));
        let transport = OpenShellCliTransport::with_runner(config(), runner);

        let snapshot = transport
            .inspect_reconciled_sandbox("workflow-os-proof")
            .unwrap_or_else(|error| panic!("{error:?}"));

        assert_eq!(snapshot.sandbox_id(), "sandbox-id-1");
        assert_eq!(snapshot.policy_revision().as_str(), "revision/7");
        assert!(!format!("{snapshot:?}").contains("sandbox-id-1"));
    }

    #[test]
    fn reconciled_snapshot_rejects_observable_sandbox_drift() {
        let changed = DETAIL_JSON.replace("\"resource_version\":2", "\"resource_version\":3");
        let runner = Arc::new(ScriptedRunner::new(&[
            "openshell 0.0.101\n",
            DETAIL_JSON,
            "openshell 0.0.101\n",
            POLICY_JSON,
            "openshell 0.0.101\n",
            &changed,
        ]));
        let transport = OpenShellCliTransport::with_runner(config(), runner);

        let error = transport
            .inspect_reconciled_sandbox("workflow-os-proof")
            .expect_err("sandbox drift should fail reconciliation");

        assert_eq!(error.category(), HostedExecutionErrorCategory::Protocol);
        assert_eq!(
            error.attempt_posture(),
            HostedExecutionAttemptPosture::MayHaveStarted
        );
    }

    #[test]
    fn debug_redacts_transport_configuration_and_response_identity() {
        let marker = "private-marker";
        let binary_path = test_binary_path(marker);
        let config = OpenShellCliTransportConfig::new(
            binary_path,
            SpecContentHash::from_bytes(TEST_BINARY_BYTES),
            "default",
            format!("registry.example/{marker}@sha256:{}", "b".repeat(64)),
        )
        .unwrap_or_else(|error| panic!("{error}"));
        let transport = OpenShellCliTransport::with_runner(
            config,
            Arc::new(ScriptedRunner::new(&["openshell 0.0.101\n", CREATE_JSON])),
        );
        let state = transport
            .create_sandbox("workflow-os-proof", Path::new("/tmp/policy.yml"))
            .unwrap_or_else(|error| panic!("{error:?}"));
        assert!(!format!("{transport:?}{state:?}").contains(marker));
        assert!(!format!("{state:?}").contains("sandbox-id-1"));
    }
}
