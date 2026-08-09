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
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use serde::Deserialize;
use serde_json::Value;
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
        let text = std::str::from_utf8(&output.stdout).map_err(|_| protocol_error())?;
        if text.trim() != format!("openshell {OPENSHELL_CLI_VERSION}") {
            return Err(protocol_error());
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
        self.verify_version()?;
        validate_runtime_name(name)?;
        if !policy_path.is_absolute() {
            return Err(protocol_error());
        }
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
        self.verify_version()?;
        validate_runtime_name(name)?;
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
        self.verify_version()?;
        validate_runtime_name(name)?;
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

    fn run<const N: usize>(
        &self,
        args: &[OsString; N],
    ) -> Result<CommandOutput, HostedExecutionInvocationError> {
        self.runner.run(
            &self.config.binary_path,
            args,
            DEFAULT_TIMEOUT,
            MAX_STREAM_BYTES,
        )
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
    let wire: SandboxWire = serde_json::from_slice(bytes).map_err(|_| protocol_error())?;
    if wire.id.is_empty()
        || wire.name != expected_name
        || wire.workspace.is_empty()
        || wire.phase.is_empty()
        || wire.resource_version == 0
        || wire.created_at.is_empty()
        || !wire.labels.values().all(Value::is_string)
        || !wire.annotations.values().all(Value::is_string)
        || (require_detail
            && (wire.policy_source.is_none() || wire.revision.is_none() || wire.policy.is_none()))
    {
        return Err(protocol_error());
    }
    Ok(OpenShellCliSandboxState {
        id: wire.id,
        name: wire.name,
        phase: wire.phase,
        current_policy_version: wire.current_policy_version,
        revision: wire.revision,
        policy_source: wire.policy_source,
    })
}

fn parse_effective_policy(
    bytes: &[u8],
    expected_name: &str,
) -> Result<OpenShellCliEffectivePolicy, HostedExecutionInvocationError> {
    let wire: EffectivePolicyWire = serde_json::from_slice(bytes).map_err(|_| protocol_error())?;
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
        return Err(protocol_error());
    }
    let policy_bytes = serde_json::to_vec(&wire.policy).map_err(|_| protocol_error())?;
    let revision = HostedExecutionPolicyRevision::new(format!("revision/{}", wire.version))
        .map_err(|_| protocol_error())?;
    Ok(OpenShellCliEffectivePolicy {
        revision,
        provider_hash: wire.hash,
        canonical_policy_hash: SpecContentHash::from_bytes(policy_bytes),
        config_revision: wire.config_revision,
        policy_source: wire.policy_source,
    })
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
    validate_name(value).map_err(|_| protocol_error())
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

const fn protocol_error() -> HostedExecutionInvocationError {
    HostedExecutionInvocationError::new(
        HostedExecutionErrorCategory::Protocol,
        HostedExecutionAttemptPosture::NotStarted,
    )
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
        let stdout = join_bounded(stdout_reader)?;
        let stderr = join_bounded(stderr_reader)?;
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
) -> Result<Vec<u8>, HostedExecutionInvocationError> {
    let bytes = handle
        .join()
        .map_err(|_| transport_error(HostedExecutionAttemptPosture::MayHaveStarted))?
        .map_err(|_| transport_error(HostedExecutionAttemptPosture::MayHaveStarted))?;
    if bytes.len() > MAX_STREAM_BYTES {
        return Err(protocol_error());
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

    struct ScriptedRunner {
        outputs: Mutex<Vec<Vec<u8>>>,
        calls: Mutex<Vec<Vec<OsString>>>,
    }

    impl ScriptedRunner {
        fn new(outputs: &[&str]) -> Self {
            Self {
                outputs: Mutex::new(
                    outputs
                        .iter()
                        .rev()
                        .map(|output| output.as_bytes().to_vec())
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
            let stdout = self
                .outputs
                .lock()
                .unwrap_or_else(|error| panic!("{error}"))
                .pop()
                .unwrap_or_else(|| panic!("missing scripted output"));
            Ok(CommandOutput {
                stdout,
                stderr: Vec::new(),
            })
        }
    }

    fn config() -> OpenShellCliTransportConfig {
        OpenShellCliTransportConfig::new(
            PathBuf::from("/opt/openshell/bin/openshell"),
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
        assert_eq!(runner.calls.lock().map_or(0, |calls| calls.len()), 1);
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
    fn debug_redacts_transport_configuration_and_response_identity() {
        let marker = "private-marker";
        let config = OpenShellCliTransportConfig::new(
            PathBuf::from(format!("/private/{marker}/openshell")),
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
