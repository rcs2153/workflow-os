use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    EventId, RedactionMetadata, SkillHandler, SkillInput, SkillOutput, WorkReportCitationKind,
    WorkReportSensitivity, WorkflowId, WorkflowOsError, WorkflowOsErrorKind, WorkflowRunId,
};

const LOCAL_CHECK_ID_MAX_BYTES: usize = 128;
const LOCAL_CHECK_ARG_MAX_BYTES: usize = 256;
const LOCAL_CHECK_ARG_MAX_COUNT: usize = 32;
const LOCAL_CHECK_ENV_MAX_COUNT: usize = 16;
const LOCAL_CHECK_OUTPUT_MAX_BYTES: usize = 64 * 1024;
const LOCAL_CHECK_TIMEOUT_MAX_SECONDS: u32 = 30 * 60;
const LOCAL_CHECK_REDACTION_FIELD_MAX_BYTES: usize = 128;
const LOCAL_CHECK_REDACTION_REASON_MAX_BYTES: usize = 256;

/// Identifier for an allowlisted local validation/check command contract.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct LocalCheckCommandId(String);

impl LocalCheckCommandId {
    /// Creates a validated local check command ID.
    ///
    /// # Errors
    ///
    /// Returns an error when the ID is empty, too long, contains invalid
    /// characters, or looks secret-like.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        validate_identifier("LocalCheckCommandId", &value)?;
        Ok(Self(value))
    }

    /// Returns the ID as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Stable identifier for a citeable local check result reference.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct LocalCheckResultId(String);

impl LocalCheckResultId {
    /// Creates a validated local check result ID.
    ///
    /// # Errors
    ///
    /// Returns an error when the ID is empty, too long, contains invalid
    /// characters, or looks secret-like.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        validate_identifier("LocalCheckResultId", &value)?;
        Ok(Self(value))
    }

    /// Returns the ID as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for LocalCheckResultId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Debug for LocalCheckResultId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("LocalCheckResultId")
            .field(&"[REDACTED]")
            .finish()
    }
}

impl From<LocalCheckResultId> for String {
    fn from(value: LocalCheckResultId) -> Self {
        value.0
    }
}

impl TryFrom<String> for LocalCheckResultId {
    type Error = WorkflowOsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for LocalCheckResultId {
    type Err = WorkflowOsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl fmt::Display for LocalCheckCommandId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Debug for LocalCheckCommandId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("LocalCheckCommandId")
            .field(&"[REDACTED]")
            .finish()
    }
}

impl From<LocalCheckCommandId> for String {
    fn from(value: LocalCheckCommandId) -> Self {
        value.0
    }
}

impl TryFrom<String> for LocalCheckCommandId {
    type Error = WorkflowOsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for LocalCheckCommandId {
    type Err = WorkflowOsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Allowlisted local validation/check command vocabulary.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalCheckCommandKind {
    /// `workflow-os validate` for the self-governance dogfood project.
    WorkflowOsValidateDogfood,
    /// `npm run check:docs`.
    DocsCheck,
    /// `cargo fmt --all --check`.
    CargoFmtCheck,
    /// `cargo clippy --workspace --all-targets -- -D warnings`.
    CargoClippyWorkspace,
    /// `cargo test --workspace`.
    CargoTestWorkspace,
    /// `npm run check:ts`.
    TypeScriptCheck,
    /// `npm run check:contracts`.
    ContractCheck,
    /// `npm run check:integrations`.
    IntegrationCheck,
}

struct LocalCheckCommandTemplate {
    executable: &'static str,
    arguments: &'static [&'static str],
}

impl LocalCheckCommandKind {
    fn template(self) -> LocalCheckCommandTemplate {
        match self {
            Self::WorkflowOsValidateDogfood => LocalCheckCommandTemplate {
                executable: "workflow-os",
                arguments: &[
                    "--project-dir",
                    "dogfood/workflow-os-self-governance",
                    "validate",
                ],
            },
            Self::DocsCheck => LocalCheckCommandTemplate {
                executable: "npm",
                arguments: &["run", "check:docs"],
            },
            Self::CargoFmtCheck => LocalCheckCommandTemplate {
                executable: "cargo",
                arguments: &["fmt", "--all", "--check"],
            },
            Self::CargoClippyWorkspace => LocalCheckCommandTemplate {
                executable: "cargo",
                arguments: &[
                    "clippy",
                    "--workspace",
                    "--all-targets",
                    "--",
                    "-D",
                    "warnings",
                ],
            },
            Self::CargoTestWorkspace => LocalCheckCommandTemplate {
                executable: "cargo",
                arguments: &["test", "--workspace"],
            },
            Self::TypeScriptCheck => LocalCheckCommandTemplate {
                executable: "npm",
                arguments: &["run", "check:ts"],
            },
            Self::ContractCheck => LocalCheckCommandTemplate {
                executable: "npm",
                arguments: &["run", "check:contracts"],
            },
            Self::IntegrationCheck => LocalCheckCommandTemplate {
                executable: "npm",
                arguments: &["run", "check:integrations"],
            },
        }
    }

    fn output_name(self) -> &'static str {
        match self {
            Self::WorkflowOsValidateDogfood => "workflow_os_validate_dogfood",
            Self::DocsCheck => "docs_check",
            Self::CargoFmtCheck => "cargo_fmt_check",
            Self::CargoClippyWorkspace => "cargo_clippy_workspace",
            Self::CargoTestWorkspace => "cargo_test_workspace",
            Self::TypeScriptCheck => "typescript_check",
            Self::ContractCheck => "contract_check",
            Self::IntegrationCheck => "integration_check",
        }
    }
}

/// Execution posture for a local check command contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalCheckExecutionPosture {
    /// Model-only contract; no execution is authorized.
    ModelOnly,
    /// Future execution may be allowed only through an explicit reviewed handler.
    AllowlistedHandlerOnly,
}

/// Working directory policy for a local check command contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalCheckWorkingDirectoryPolicy {
    /// Run from the repository root.
    RepositoryRoot,
    /// Run from the self-governance dogfood project directory.
    DogfoodProjectRoot,
}

/// Environment variable policy for a local check command contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalCheckEnvironmentPolicy {
    /// Use a minimal environment and remove secret-bearing variables.
    SanitizedMinimal,
    /// Use only explicitly allowlisted non-secret variables.
    ExplicitAllowlistOnly,
}

/// Network policy for a local check command contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalCheckNetworkPolicy {
    /// No network access is expected or allowed.
    Disabled,
}

/// Side-effect classification for a local check command contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalCheckSideEffectClass {
    /// No repository source writes are expected.
    NoSourceWrites,
    /// Build or cache writes may occur in declared output directories.
    BuildOrCacheWrites,
    /// Side effects are not sufficiently classified for execution.
    Unclassified,
}

/// Output capture policy for a future local check handler.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LocalCheckOutputCapturePolicy {
    /// Maximum captured stdout bytes.
    pub stdout_max_bytes: usize,
    /// Maximum captured stderr bytes.
    pub stderr_max_bytes: usize,
    /// Whether full raw output may be persisted.
    pub persist_raw_output: bool,
}

impl LocalCheckOutputCapturePolicy {
    /// Creates a bounded output capture policy.
    #[must_use]
    pub const fn bounded(stdout_max_bytes: usize, stderr_max_bytes: usize) -> Self {
        Self {
            stdout_max_bytes,
            stderr_max_bytes,
            persist_raw_output: false,
        }
    }

    /// Validates the output capture policy.
    ///
    /// # Errors
    ///
    /// Returns an error when output bounds are zero, too large, or raw output
    /// persistence is requested.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        if self.stdout_max_bytes == 0 || self.stderr_max_bytes == 0 {
            return Err(validation_error(
                "local_check.output.bound_required",
                "local check output capture bounds must be non-zero",
            ));
        }

        if self.stdout_max_bytes > LOCAL_CHECK_OUTPUT_MAX_BYTES
            || self.stderr_max_bytes > LOCAL_CHECK_OUTPUT_MAX_BYTES
        {
            return Err(validation_error(
                "local_check.output.bound_too_large",
                "local check output capture bounds exceed the supported maximum",
            ));
        }

        if self.persist_raw_output {
            return Err(validation_error(
                "local_check.output.raw_persistence_unsupported",
                "local check contracts cannot persist raw command output",
            ));
        }

        Ok(())
    }
}

/// Redaction policy for future local check output and errors.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalCheckRedactionPolicy {
    /// Capture bounded redacted summaries only.
    BoundedRedactedSummary,
}

/// Result status vocabulary for future local check results.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalCheckResultStatus {
    /// The check passed.
    Passed,
    /// The check failed.
    Failed,
    /// The check timed out.
    TimedOut,
    /// The check was skipped.
    Skipped,
    /// The check was not available.
    NotAvailable,
    /// The check hit an internal error.
    InternalError,
    /// Policy denied check execution.
    PolicyDenied,
    /// Redaction failed.
    RedactionFailed,
}

impl fmt::Display for LocalCheckResultStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::TimedOut => "timed_out",
            Self::Skipped => "skipped",
            Self::NotAvailable => "not_available",
            Self::InternalError => "internal_error",
            Self::PolicyDenied => "policy_denied",
            Self::RedactionFailed => "redaction_failed",
        };
        formatter.write_str(value)
    }
}

/// Domain-neutral local validation/check command contract.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct LocalCheckCommandContract {
    command_id: LocalCheckCommandId,
    command_kind: LocalCheckCommandKind,
    execution_posture: LocalCheckExecutionPosture,
    executable: String,
    arguments: Vec<String>,
    working_directory_policy: LocalCheckWorkingDirectoryPolicy,
    environment_policy: LocalCheckEnvironmentPolicy,
    allowed_environment_variables: Vec<String>,
    network_policy: LocalCheckNetworkPolicy,
    timeout_seconds: u32,
    side_effect_class: LocalCheckSideEffectClass,
    permitted_output_directories: Vec<String>,
    output_capture: LocalCheckOutputCapturePolicy,
    redaction_policy: LocalCheckRedactionPolicy,
    citation_kinds: Vec<WorkReportCitationKind>,
}

/// Input fields for constructing a validated local check command contract.
pub struct LocalCheckCommandContractDefinition {
    /// Command contract ID.
    pub command_id: LocalCheckCommandId,
    /// Allowlisted command kind.
    pub command_kind: LocalCheckCommandKind,
    /// Execution posture.
    pub execution_posture: LocalCheckExecutionPosture,
    /// Executable name or repository-relative executable path.
    pub executable: String,
    /// Fixed argument vector.
    pub arguments: Vec<String>,
    /// Working directory policy.
    pub working_directory_policy: LocalCheckWorkingDirectoryPolicy,
    /// Environment policy.
    pub environment_policy: LocalCheckEnvironmentPolicy,
    /// Explicitly allowed non-secret environment variable names.
    pub allowed_environment_variables: Vec<String>,
    /// Network policy.
    pub network_policy: LocalCheckNetworkPolicy,
    /// Timeout in seconds.
    pub timeout_seconds: u32,
    /// Side-effect classification.
    pub side_effect_class: LocalCheckSideEffectClass,
    /// Permitted output/cache directories.
    pub permitted_output_directories: Vec<String>,
    /// Output capture policy.
    pub output_capture: LocalCheckOutputCapturePolicy,
    /// Redaction policy.
    pub redaction_policy: LocalCheckRedactionPolicy,
    /// Citation kinds this check may later feed into reports.
    pub citation_kinds: Vec<WorkReportCitationKind>,
}

impl LocalCheckCommandContract {
    /// Creates a validated local check command contract.
    ///
    /// # Errors
    ///
    /// Returns an error when the contract uses arbitrary shell text, unbounded
    /// output capture, secret-like fields, duplicate citation kinds, or an
    /// execution posture that would authorize command execution prematurely.
    pub fn new(definition: LocalCheckCommandContractDefinition) -> Result<Self, WorkflowOsError> {
        let contract = Self {
            command_id: definition.command_id,
            command_kind: definition.command_kind,
            execution_posture: definition.execution_posture,
            executable: definition.executable,
            arguments: definition.arguments,
            working_directory_policy: definition.working_directory_policy,
            environment_policy: definition.environment_policy,
            allowed_environment_variables: definition.allowed_environment_variables,
            network_policy: definition.network_policy,
            timeout_seconds: definition.timeout_seconds,
            side_effect_class: definition.side_effect_class,
            permitted_output_directories: definition.permitted_output_directories,
            output_capture: definition.output_capture,
            redaction_policy: definition.redaction_policy,
            citation_kinds: definition.citation_kinds,
        };
        contract.validate()?;
        Ok(contract)
    }

    /// Creates the planned model-only dogfood validation command contract.
    ///
    /// # Errors
    ///
    /// Returns an error only if the built-in definition violates model
    /// validation.
    pub fn dogfood_validate_model_only() -> Result<Self, WorkflowOsError> {
        Self::new(LocalCheckCommandContractDefinition {
            command_id: LocalCheckCommandId::new("local-check/dogfood-validate")?,
            command_kind: LocalCheckCommandKind::WorkflowOsValidateDogfood,
            execution_posture: LocalCheckExecutionPosture::ModelOnly,
            executable: "workflow-os".to_owned(),
            arguments: vec![
                "--project-dir".to_owned(),
                "dogfood/workflow-os-self-governance".to_owned(),
                "validate".to_owned(),
            ],
            working_directory_policy: LocalCheckWorkingDirectoryPolicy::RepositoryRoot,
            environment_policy: LocalCheckEnvironmentPolicy::SanitizedMinimal,
            allowed_environment_variables: Vec::new(),
            network_policy: LocalCheckNetworkPolicy::Disabled,
            timeout_seconds: 120,
            side_effect_class: LocalCheckSideEffectClass::NoSourceWrites,
            permitted_output_directories: Vec::new(),
            output_capture: LocalCheckOutputCapturePolicy::bounded(16 * 1024, 16 * 1024),
            redaction_policy: LocalCheckRedactionPolicy::BoundedRedactedSummary,
            citation_kinds: vec![
                WorkReportCitationKind::ValidationDiagnostic,
                WorkReportCitationKind::WorkflowEvent,
                WorkReportCitationKind::AuditEvent,
            ],
        })
    }

    /// Creates the planned model-only docs check command contract.
    ///
    /// # Errors
    ///
    /// Returns an error only if the built-in definition violates model
    /// validation.
    pub fn docs_check_model_only() -> Result<Self, WorkflowOsError> {
        Self::new(LocalCheckCommandContractDefinition {
            command_id: LocalCheckCommandId::new("local-check/docs")?,
            command_kind: LocalCheckCommandKind::DocsCheck,
            execution_posture: LocalCheckExecutionPosture::ModelOnly,
            executable: "npm".to_owned(),
            arguments: vec!["run".to_owned(), "check:docs".to_owned()],
            working_directory_policy: LocalCheckWorkingDirectoryPolicy::RepositoryRoot,
            environment_policy: LocalCheckEnvironmentPolicy::SanitizedMinimal,
            allowed_environment_variables: vec!["NPM_CONFIG_CACHE".to_owned()],
            network_policy: LocalCheckNetworkPolicy::Disabled,
            timeout_seconds: 120,
            side_effect_class: LocalCheckSideEffectClass::NoSourceWrites,
            permitted_output_directories: Vec::new(),
            output_capture: LocalCheckOutputCapturePolicy::bounded(16 * 1024, 16 * 1024),
            redaction_policy: LocalCheckRedactionPolicy::BoundedRedactedSummary,
            citation_kinds: vec![
                WorkReportCitationKind::ValidationDiagnostic,
                WorkReportCitationKind::WorkflowEvent,
                WorkReportCitationKind::AuditEvent,
            ],
        })
    }

    /// Validates the contract.
    ///
    /// # Errors
    ///
    /// Returns stable validation errors without echoing caller-supplied command,
    /// argument, environment, path, or output text.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        if self.execution_posture != LocalCheckExecutionPosture::ModelOnly {
            return Err(validation_error(
                "local_check.execution.deferred",
                "local check command execution remains deferred",
            ));
        }

        validate_command_token("local check executable", &self.executable)?;
        validate_arguments(&self.arguments)?;
        validate_command_template(self.command_kind, &self.executable, &self.arguments)?;
        validate_environment_variables(&self.allowed_environment_variables)?;
        validate_output_directories(&self.permitted_output_directories)?;
        validate_timeout(self.timeout_seconds)?;
        self.output_capture.validate()?;
        validate_citation_kinds(&self.citation_kinds)?;

        if self.side_effect_class == LocalCheckSideEffectClass::Unclassified {
            return Err(validation_error(
                "local_check.side_effect.unclassified",
                "local check side effects must be classified before execution is considered",
            ));
        }

        Ok(())
    }

    /// Returns the command contract ID.
    #[must_use]
    pub const fn command_id(&self) -> &LocalCheckCommandId {
        &self.command_id
    }

    /// Returns the allowlisted command kind.
    #[must_use]
    pub const fn command_kind(&self) -> LocalCheckCommandKind {
        self.command_kind
    }

    /// Returns the execution posture.
    #[must_use]
    pub const fn execution_posture(&self) -> LocalCheckExecutionPosture {
        self.execution_posture
    }

    /// Returns the executable token.
    #[must_use]
    pub fn executable(&self) -> &str {
        &self.executable
    }

    /// Returns the fixed argument vector.
    #[must_use]
    pub fn arguments(&self) -> &[String] {
        &self.arguments
    }

    /// Returns the working-directory policy.
    #[must_use]
    pub const fn working_directory_policy(&self) -> LocalCheckWorkingDirectoryPolicy {
        self.working_directory_policy
    }

    /// Returns the environment policy.
    #[must_use]
    pub const fn environment_policy(&self) -> LocalCheckEnvironmentPolicy {
        self.environment_policy
    }

    /// Returns explicitly allowed non-secret environment variable names.
    #[must_use]
    pub fn allowed_environment_variables(&self) -> &[String] {
        &self.allowed_environment_variables
    }

    /// Returns the network policy.
    #[must_use]
    pub const fn network_policy(&self) -> LocalCheckNetworkPolicy {
        self.network_policy
    }

    /// Returns the timeout in seconds.
    #[must_use]
    pub const fn timeout_seconds(&self) -> u32 {
        self.timeout_seconds
    }

    /// Returns the side-effect classification.
    #[must_use]
    pub const fn side_effect_class(&self) -> LocalCheckSideEffectClass {
        self.side_effect_class
    }

    /// Returns the output capture policy.
    #[must_use]
    pub const fn output_capture(&self) -> &LocalCheckOutputCapturePolicy {
        &self.output_capture
    }

    /// Returns the citation kinds.
    #[must_use]
    pub fn citation_kinds(&self) -> &[WorkReportCitationKind] {
        &self.citation_kinds
    }
}

/// Test-only local check handler for `WorkflowOsValidateDogfood`.
///
/// This handler is never registered by default. It exists to prove the local
/// check handler boundary in focused tests before production check execution,
/// CLI exposure, workflow schema fields, or automatic dogfood execution are
/// considered.
#[derive(Clone)]
pub struct TestOnlyWorkflowOsValidateDogfoodHandler {
    contract: LocalCheckCommandContract,
    workflow_os_binary: PathBuf,
    repository_root: PathBuf,
    process_runner: Arc<dyn LocalCheckProcessRunner>,
}

impl TestOnlyWorkflowOsValidateDogfoodHandler {
    /// Creates a test-only handler for the dogfood validation command.
    ///
    /// # Errors
    ///
    /// Returns an error when the contract is not the canonical dogfood
    /// validation contract or when required local paths are missing.
    pub fn new(
        contract: LocalCheckCommandContract,
        workflow_os_binary: PathBuf,
        repository_root: PathBuf,
    ) -> Result<Self, WorkflowOsError> {
        Self::new_with_process_runner(
            contract,
            workflow_os_binary,
            repository_root,
            Arc::new(StdLocalCheckProcessRunner),
        )
    }

    /// Creates a test-only handler with an injected process runner.
    ///
    /// This constructor exists so tests can deterministically exercise failure,
    /// timeout, and redaction behavior without broadening production command
    /// execution.
    ///
    /// # Errors
    ///
    /// Returns an error when the contract is not the canonical dogfood
    /// validation contract or when required local paths are missing.
    pub fn new_with_process_runner(
        contract: LocalCheckCommandContract,
        workflow_os_binary: PathBuf,
        repository_root: PathBuf,
        process_runner: Arc<dyn LocalCheckProcessRunner>,
    ) -> Result<Self, WorkflowOsError> {
        contract.validate()?;
        if contract.command_kind() != LocalCheckCommandKind::WorkflowOsValidateDogfood {
            return Err(local_check_error(
                WorkflowOsErrorKind::Validation,
                "local_check.handler.unsupported_kind",
                "test-only local check handler supports only dogfood validation",
            ));
        }
        if contract.working_directory_policy() != LocalCheckWorkingDirectoryPolicy::RepositoryRoot {
            return Err(local_check_error(
                WorkflowOsErrorKind::Validation,
                "local_check.handler.working_directory_unsupported",
                "test-only local check handler requires repository-root working directory policy",
            ));
        }
        if contract.network_policy() != LocalCheckNetworkPolicy::Disabled {
            return Err(local_check_error(
                WorkflowOsErrorKind::Validation,
                "local_check.handler.network_unsupported",
                "test-only local check handler requires disabled network policy",
            ));
        }
        if contract.side_effect_class() != LocalCheckSideEffectClass::NoSourceWrites {
            return Err(local_check_error(
                WorkflowOsErrorKind::Validation,
                "local_check.handler.side_effect_unsupported",
                "test-only local check handler requires no-source-writes classification",
            ));
        }
        if !workflow_os_binary.is_file() {
            return Err(local_check_error(
                WorkflowOsErrorKind::Validation,
                "local_check.handler.binary_missing",
                "test-only local check handler requires an existing workflow-os binary",
            ));
        }
        if !repository_root.join("Cargo.toml").is_file()
            || !repository_root
                .join("dogfood/workflow-os-self-governance")
                .is_dir()
        {
            return Err(local_check_error(
                WorkflowOsErrorKind::Validation,
                "local_check.handler.repository_root_invalid",
                "test-only local check handler requires the Workflow OS repository root",
            ));
        }

        Ok(Self {
            contract,
            workflow_os_binary,
            repository_root,
            process_runner,
        })
    }
}

impl fmt::Debug for TestOnlyWorkflowOsValidateDogfoodHandler {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TestOnlyWorkflowOsValidateDogfoodHandler")
            .field("command_kind", &self.contract.command_kind())
            .field("workflow_os_binary", &"[REDACTED]")
            .field("repository_root", &"[REDACTED]")
            .field("process_runner", &"[REDACTED]")
            .finish()
    }
}

impl SkillHandler for TestOnlyWorkflowOsValidateDogfoodHandler {
    fn invoke(&self, _input: SkillInput) -> Result<SkillOutput, WorkflowOsError> {
        self.contract.validate()?;

        let request = LocalCheckProcessRequest::new(
            self.workflow_os_binary.clone(),
            self.contract.arguments().to_vec(),
            self.repository_root.clone(),
            sanitized_environment()?,
            Duration::from_secs(u64::from(self.contract.timeout_seconds())),
        )?;
        let output = self.process_runner.run(&request)?;
        let result = LocalCheckResult::from_process_output(&self.contract, &output)?;

        Ok(result.to_skill_output())
    }
}

/// Explicit local check handler for `DocsCheck`.
///
/// This handler is never registered by default. It exists to prove the first
/// production-shaped non-dogfood local check boundary before default
/// registration, CLI exposure, workflow schema fields, evidence attachment, or
/// automatic check execution are considered.
#[derive(Clone)]
pub struct DocsCheckLocalHandler {
    contract: LocalCheckCommandContract,
    npm_executable: PathBuf,
    repository_root: PathBuf,
    npm_cache_directory: Option<PathBuf>,
    process_runner: Arc<dyn LocalCheckProcessRunner>,
}

impl DocsCheckLocalHandler {
    /// Creates an explicit local handler for the docs check command.
    ///
    /// # Errors
    ///
    /// Returns an error when the contract is not the canonical docs check
    /// contract or when required local paths are missing.
    pub fn new(
        contract: LocalCheckCommandContract,
        npm_executable: PathBuf,
        repository_root: PathBuf,
        npm_cache_directory: Option<PathBuf>,
    ) -> Result<Self, WorkflowOsError> {
        Self::new_with_process_runner(
            contract,
            npm_executable,
            repository_root,
            npm_cache_directory,
            Arc::new(StdLocalCheckProcessRunner),
        )
    }

    /// Creates an explicit docs check handler with an injected process runner.
    ///
    /// This constructor exists so tests can deterministically exercise docs
    /// check success, failure, timeout, environment, and redaction behavior
    /// without making npm execution ambient or default.
    ///
    /// # Errors
    ///
    /// Returns an error when the contract is not the canonical docs check
    /// contract or when required local paths are missing.
    pub fn new_with_process_runner(
        contract: LocalCheckCommandContract,
        npm_executable: PathBuf,
        repository_root: PathBuf,
        npm_cache_directory: Option<PathBuf>,
        process_runner: Arc<dyn LocalCheckProcessRunner>,
    ) -> Result<Self, WorkflowOsError> {
        contract.validate()?;
        if contract.command_kind() != LocalCheckCommandKind::DocsCheck {
            return Err(local_check_error(
                WorkflowOsErrorKind::Validation,
                "local_check.handler.unsupported_kind",
                "docs check local handler supports only docs check",
            ));
        }
        if contract.working_directory_policy() != LocalCheckWorkingDirectoryPolicy::RepositoryRoot {
            return Err(local_check_error(
                WorkflowOsErrorKind::Validation,
                "local_check.handler.working_directory_unsupported",
                "docs check local handler requires repository-root working directory policy",
            ));
        }
        if contract.environment_policy() != LocalCheckEnvironmentPolicy::SanitizedMinimal {
            return Err(local_check_error(
                WorkflowOsErrorKind::Validation,
                "local_check.handler.environment_unsupported",
                "docs check local handler requires sanitized minimal environment policy",
            ));
        }
        if contract.network_policy() != LocalCheckNetworkPolicy::Disabled {
            return Err(local_check_error(
                WorkflowOsErrorKind::Validation,
                "local_check.handler.network_unsupported",
                "docs check local handler requires disabled network policy",
            ));
        }
        if contract.side_effect_class() != LocalCheckSideEffectClass::NoSourceWrites {
            return Err(local_check_error(
                WorkflowOsErrorKind::Validation,
                "local_check.handler.side_effect_unsupported",
                "docs check local handler requires no-source-writes classification",
            ));
        }
        if !npm_executable.is_file() {
            return Err(local_check_error(
                WorkflowOsErrorKind::Validation,
                "local_check.handler.npm_missing",
                "docs check local handler requires an existing npm executable",
            ));
        }
        if !repository_root.join("package.json").is_file()
            || !repository_root.join("scripts/check-docs.mjs").is_file()
        {
            return Err(local_check_error(
                WorkflowOsErrorKind::Validation,
                "local_check.handler.repository_root_invalid",
                "docs check local handler requires the Workflow OS repository root",
            ));
        }

        let _ = docs_check_environment(npm_cache_directory.as_deref())?;

        Ok(Self {
            contract,
            npm_executable,
            repository_root,
            npm_cache_directory,
            process_runner,
        })
    }
}

impl fmt::Debug for DocsCheckLocalHandler {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DocsCheckLocalHandler")
            .field("command_kind", &self.contract.command_kind())
            .field("npm_executable", &"[REDACTED]")
            .field("repository_root", &"[REDACTED]")
            .field("npm_cache_directory", &"[REDACTED]")
            .field("process_runner", &"[REDACTED]")
            .finish()
    }
}

impl SkillHandler for DocsCheckLocalHandler {
    fn invoke(&self, _input: SkillInput) -> Result<SkillOutput, WorkflowOsError> {
        self.contract.validate()?;

        let request = LocalCheckProcessRequest::new(
            self.npm_executable.clone(),
            self.contract.arguments().to_vec(),
            self.repository_root.clone(),
            docs_check_environment(self.npm_cache_directory.as_deref())?,
            Duration::from_secs(u64::from(self.contract.timeout_seconds())),
        )?;
        let output = self.process_runner.run(&request)?;
        let result = LocalCheckResult::from_process_output(&self.contract, &output)?;

        Ok(result.to_skill_output())
    }
}

/// Backward-compatible alias for tests that still use the original
/// test-scoped name. New code should prefer [`DocsCheckLocalHandler`].
pub type TestOnlyDocsCheckHandler = DocsCheckLocalHandler;

/// Validated local check result summary.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct LocalCheckResult {
    command_id: LocalCheckCommandId,
    command_kind: LocalCheckCommandKind,
    status: LocalCheckResultStatus,
    exit_code: Option<i32>,
    duration_ms: u64,
    stdout_summary: String,
    stderr_summary: String,
    stdout_truncated: bool,
    stderr_truncated: bool,
    error_code: Option<String>,
}

/// Input fields for constructing a validated local check result.
pub struct LocalCheckResultDefinition {
    /// Command contract ID.
    pub command_id: LocalCheckCommandId,
    /// Allowlisted command kind.
    pub command_kind: LocalCheckCommandKind,
    /// Check result status.
    pub status: LocalCheckResultStatus,
    /// Process exit code, if available.
    pub exit_code: Option<i32>,
    /// Duration in milliseconds.
    pub duration_ms: u64,
    /// Bounded stdout summary.
    pub stdout_summary: String,
    /// Bounded stderr summary.
    pub stderr_summary: String,
    /// Whether stdout was truncated.
    pub stdout_truncated: bool,
    /// Whether stderr was truncated.
    pub stderr_truncated: bool,
    /// Stable internal error code, if the result represents an internal failure.
    pub error_code: Option<String>,
}

impl LocalCheckResult {
    /// Creates a validated local check result.
    ///
    /// # Errors
    ///
    /// Returns an error when summaries are too large, contain sensitive-looking
    /// text, or the error code is not stable and bounded.
    pub fn new(definition: LocalCheckResultDefinition) -> Result<Self, WorkflowOsError> {
        let result = Self {
            command_id: definition.command_id,
            command_kind: definition.command_kind,
            status: definition.status,
            exit_code: definition.exit_code,
            duration_ms: definition.duration_ms,
            stdout_summary: definition.stdout_summary,
            stderr_summary: definition.stderr_summary,
            stdout_truncated: definition.stdout_truncated,
            stderr_truncated: definition.stderr_truncated,
            error_code: definition.error_code,
        };
        result.validate()?;
        Ok(result)
    }

    fn from_process_output(
        contract: &LocalCheckCommandContract,
        output: &LocalCheckProcessOutput,
    ) -> Result<Self, WorkflowOsError> {
        let stdout = bounded_redacted_output(
            output.stdout.as_slice(),
            contract.output_capture().stdout_max_bytes,
            "stdout",
        )?;
        let stderr = bounded_redacted_output(
            output.stderr.as_slice(),
            contract.output_capture().stderr_max_bytes,
            "stderr",
        )?;
        let (status, error_code) = if output.timed_out {
            (
                LocalCheckResultStatus::TimedOut,
                Some("local_check.handler.timed_out".to_owned()),
            )
        } else if output.success {
            (LocalCheckResultStatus::Passed, None)
        } else {
            (LocalCheckResultStatus::Failed, None)
        };

        Self::new(LocalCheckResultDefinition {
            command_id: contract.command_id().clone(),
            command_kind: contract.command_kind(),
            status,
            exit_code: output.exit_code,
            duration_ms: output.duration_ms,
            stdout_summary: stdout.summary,
            stderr_summary: stderr.summary,
            stdout_truncated: stdout.truncated,
            stderr_truncated: stderr.truncated,
            error_code,
        })
    }

    /// Validates this result.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when the result is invalid.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_result_summary("local check stdout summary", &self.stdout_summary)?;
        validate_result_summary("local check stderr summary", &self.stderr_summary)?;
        if let Some(error_code) = &self.error_code {
            validate_identifier("local check result error code", error_code)?;
        }
        Ok(())
    }

    /// Returns the command contract ID.
    #[must_use]
    pub const fn command_id(&self) -> &LocalCheckCommandId {
        &self.command_id
    }

    /// Returns the command kind.
    #[must_use]
    pub const fn command_kind(&self) -> LocalCheckCommandKind {
        self.command_kind
    }

    /// Returns the result status.
    #[must_use]
    pub const fn status(&self) -> LocalCheckResultStatus {
        self.status
    }

    /// Returns the exit code, if available.
    #[must_use]
    pub const fn exit_code(&self) -> Option<i32> {
        self.exit_code
    }

    /// Returns duration in milliseconds.
    #[must_use]
    pub const fn duration_ms(&self) -> u64 {
        self.duration_ms
    }

    /// Returns the bounded stdout summary.
    #[must_use]
    pub fn stdout_summary(&self) -> &str {
        &self.stdout_summary
    }

    /// Returns the bounded stderr summary.
    #[must_use]
    pub fn stderr_summary(&self) -> &str {
        &self.stderr_summary
    }

    /// Returns whether stdout was truncated.
    #[must_use]
    pub const fn stdout_truncated(&self) -> bool {
        self.stdout_truncated
    }

    /// Returns whether stderr was truncated.
    #[must_use]
    pub const fn stderr_truncated(&self) -> bool {
        self.stderr_truncated
    }

    /// Returns a stable error code, if present.
    #[must_use]
    pub fn error_code(&self) -> Option<&str> {
        self.error_code.as_deref()
    }

    fn to_skill_output(&self) -> SkillOutput {
        let mut values = BTreeMap::new();
        values.insert("summary".to_owned(), "local check completed".to_owned());
        values.insert("local_check_status".to_owned(), self.status.to_string());
        values.insert(
            "local_check_kind".to_owned(),
            self.command_kind.output_name().to_owned(),
        );
        values.insert(
            "exit_code".to_owned(),
            self.exit_code
                .map_or_else(|| "not_available".to_owned(), |code| code.to_string()),
        );
        values.insert("duration_ms".to_owned(), self.duration_ms.to_string());
        values.insert("stdout_summary".to_owned(), self.stdout_summary.clone());
        values.insert("stderr_summary".to_owned(), self.stderr_summary.clone());
        values.insert(
            "stdout_truncated".to_owned(),
            self.stdout_truncated.to_string(),
        );
        values.insert(
            "stderr_truncated".to_owned(),
            self.stderr_truncated.to_string(),
        );
        if let Some(error_code) = &self.error_code {
            values.insert("error_code".to_owned(), error_code.clone());
        }
        let output_ref = Some(format!(
            "local-check-result/{}/{}",
            self.command_id, self.status
        ));
        SkillOutput::new(values, output_ref)
    }
}

impl fmt::Debug for LocalCheckResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalCheckResult")
            .field("command_id", &self.command_id)
            .field("command_kind", &self.command_kind)
            .field("status", &self.status)
            .field("exit_code", &self.exit_code)
            .field("duration_ms", &self.duration_ms)
            .field("stdout_summary", &"[REDACTED]")
            .field("stderr_summary", &"[REDACTED]")
            .field("stdout_truncated", &self.stdout_truncated)
            .field("stderr_truncated", &self.stderr_truncated)
            .field("error_code", &self.error_code)
            .finish()
    }
}

impl<'de> Deserialize<'de> for LocalCheckResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct LocalCheckResultWire {
            command_id: LocalCheckCommandId,
            command_kind: LocalCheckCommandKind,
            status: LocalCheckResultStatus,
            exit_code: Option<i32>,
            duration_ms: u64,
            stdout_summary: String,
            stderr_summary: String,
            stdout_truncated: bool,
            stderr_truncated: bool,
            error_code: Option<String>,
        }

        let wire = LocalCheckResultWire::deserialize(deserializer)?;
        Self::new(LocalCheckResultDefinition {
            command_id: wire.command_id,
            command_kind: wire.command_kind,
            status: wire.status,
            exit_code: wire.exit_code,
            duration_ms: wire.duration_ms,
            stdout_summary: wire.stdout_summary,
            stderr_summary: wire.stderr_summary,
            stdout_truncated: wire.stdout_truncated,
            stderr_truncated: wire.stderr_truncated,
            error_code: wire.error_code,
        })
        .map_err(serde::de::Error::custom)
    }
}

/// Stable, bounded reference to a local check result.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct LocalCheckResultReference {
    result_id: LocalCheckResultId,
    command_id: LocalCheckCommandId,
    command_kind: LocalCheckCommandKind,
    status: LocalCheckResultStatus,
    workflow_id: WorkflowId,
    run_id: WorkflowRunId,
    workflow_event_id: Option<EventId>,
    audit_event_id: Option<EventId>,
    output_reference: Option<String>,
    redaction: RedactionMetadata,
    sensitivity: WorkReportSensitivity,
}

/// Input fields for constructing a validated local check result reference.
pub struct LocalCheckResultReferenceDefinition {
    /// Stable local check result reference ID.
    pub result_id: LocalCheckResultId,
    /// Command contract ID.
    pub command_id: LocalCheckCommandId,
    /// Allowlisted command kind.
    pub command_kind: LocalCheckCommandKind,
    /// Check result status.
    pub status: LocalCheckResultStatus,
    /// Workflow ID associated with the result.
    pub workflow_id: WorkflowId,
    /// Workflow run ID associated with the result.
    pub run_id: WorkflowRunId,
    /// Optional workflow event ID associated with the result.
    pub workflow_event_id: Option<EventId>,
    /// Optional audit event ID associated with the result.
    pub audit_event_id: Option<EventId>,
    /// Optional stable output reference.
    pub output_reference: Option<String>,
    /// Redaction metadata for reference fields.
    pub redaction: RedactionMetadata,
    /// Sensitivity classification.
    pub sensitivity: WorkReportSensitivity,
}

impl LocalCheckResultReference {
    /// Creates a validated local check result reference.
    ///
    /// # Errors
    ///
    /// Returns an error when optional reference text or redaction metadata is
    /// unbounded or secret-like.
    pub fn new(definition: LocalCheckResultReferenceDefinition) -> Result<Self, WorkflowOsError> {
        let reference = Self {
            result_id: definition.result_id,
            command_id: definition.command_id,
            command_kind: definition.command_kind,
            status: definition.status,
            workflow_id: definition.workflow_id,
            run_id: definition.run_id,
            workflow_event_id: definition.workflow_event_id,
            audit_event_id: definition.audit_event_id,
            output_reference: definition.output_reference,
            redaction: definition.redaction,
            sensitivity: definition.sensitivity,
        };
        reference.validate()?;
        Ok(reference)
    }

    /// Creates a local check result reference from an existing result plus
    /// explicit workflow context.
    ///
    /// # Errors
    ///
    /// Returns an error when the reference fields are invalid.
    #[allow(clippy::too_many_arguments)]
    pub fn from_result(
        result_id: LocalCheckResultId,
        result: &LocalCheckResult,
        workflow_id: WorkflowId,
        run_id: WorkflowRunId,
        workflow_event_id: Option<EventId>,
        audit_event_id: Option<EventId>,
        output_reference: Option<String>,
        redaction: RedactionMetadata,
        sensitivity: WorkReportSensitivity,
    ) -> Result<Self, WorkflowOsError> {
        Self::new(LocalCheckResultReferenceDefinition {
            result_id,
            command_id: result.command_id().clone(),
            command_kind: result.command_kind(),
            status: result.status(),
            workflow_id,
            run_id,
            workflow_event_id,
            audit_event_id,
            output_reference,
            redaction,
            sensitivity,
        })
    }

    /// Validates this reference.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when the reference is invalid.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        if let Some(output_reference) = &self.output_reference {
            validate_identifier("local check result output reference", output_reference)?;
        }
        validate_reference_redaction_metadata(&self.redaction)?;
        Ok(())
    }

    /// Returns the local check result reference ID.
    #[must_use]
    pub const fn result_id(&self) -> &LocalCheckResultId {
        &self.result_id
    }

    /// Returns the command contract ID.
    #[must_use]
    pub const fn command_id(&self) -> &LocalCheckCommandId {
        &self.command_id
    }

    /// Returns the command kind.
    #[must_use]
    pub const fn command_kind(&self) -> LocalCheckCommandKind {
        self.command_kind
    }

    /// Returns the check result status.
    #[must_use]
    pub const fn status(&self) -> LocalCheckResultStatus {
        self.status
    }

    /// Returns the workflow ID.
    #[must_use]
    pub const fn workflow_id(&self) -> &WorkflowId {
        &self.workflow_id
    }

    /// Returns the workflow run ID.
    #[must_use]
    pub const fn run_id(&self) -> &WorkflowRunId {
        &self.run_id
    }

    /// Returns the workflow event ID, if available.
    #[must_use]
    pub const fn workflow_event_id(&self) -> Option<&EventId> {
        self.workflow_event_id.as_ref()
    }

    /// Returns the audit event ID, if available.
    #[must_use]
    pub const fn audit_event_id(&self) -> Option<&EventId> {
        self.audit_event_id.as_ref()
    }

    /// Returns the stable output reference, if available.
    #[must_use]
    pub fn output_reference(&self) -> Option<&str> {
        self.output_reference.as_deref()
    }

    /// Returns the sensitivity classification.
    #[must_use]
    pub const fn sensitivity(&self) -> WorkReportSensitivity {
        self.sensitivity
    }
}

impl fmt::Debug for LocalCheckResultReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalCheckResultReference")
            .field("result_id", &"[REDACTED]")
            .field("command_id", &"[REDACTED]")
            .field("command_kind", &self.command_kind)
            .field("status", &self.status)
            .field("workflow_id", &"[REDACTED]")
            .field("run_id", &"[REDACTED]")
            .field(
                "workflow_event_id",
                &self.workflow_event_id.as_ref().map(|_| "[REDACTED]"),
            )
            .field(
                "audit_event_id",
                &self.audit_event_id.as_ref().map(|_| "[REDACTED]"),
            )
            .field(
                "output_reference",
                &self.output_reference.as_ref().map(|_| "[REDACTED]"),
            )
            .field("redaction", &"[REDACTED]")
            .field("sensitivity", &self.sensitivity)
            .finish()
    }
}

impl<'de> Deserialize<'de> for LocalCheckResultReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct LocalCheckResultReferenceWire {
            result_id: LocalCheckResultId,
            command_id: LocalCheckCommandId,
            command_kind: LocalCheckCommandKind,
            status: LocalCheckResultStatus,
            workflow_id: WorkflowId,
            run_id: WorkflowRunId,
            workflow_event_id: Option<EventId>,
            audit_event_id: Option<EventId>,
            output_reference: Option<String>,
            redaction: RedactionMetadata,
            sensitivity: WorkReportSensitivity,
        }

        let wire = LocalCheckResultReferenceWire::deserialize(deserializer)?;
        Self::new(LocalCheckResultReferenceDefinition {
            result_id: wire.result_id,
            command_id: wire.command_id,
            command_kind: wire.command_kind,
            status: wire.status,
            workflow_id: wire.workflow_id,
            run_id: wire.run_id,
            workflow_event_id: wire.workflow_event_id,
            audit_event_id: wire.audit_event_id,
            output_reference: wire.output_reference,
            redaction: wire.redaction,
            sensitivity: wire.sensitivity,
        })
        .map_err(serde::de::Error::custom)
    }
}

/// Process execution request for a local check handler.
#[derive(Clone, Eq, PartialEq)]
pub struct LocalCheckProcessRequest {
    executable: PathBuf,
    arguments: Vec<String>,
    working_directory: PathBuf,
    environment: BTreeMap<String, String>,
    timeout: Duration,
}

impl LocalCheckProcessRequest {
    /// Creates a validated local check process request.
    ///
    /// # Errors
    ///
    /// Returns an error when command tokens or environment entries are unsafe.
    pub fn new(
        executable: PathBuf,
        arguments: Vec<String>,
        working_directory: PathBuf,
        environment: BTreeMap<String, String>,
        timeout: Duration,
    ) -> Result<Self, WorkflowOsError> {
        let executable_text = executable.to_string_lossy();
        if executable_text.is_empty() {
            return Err(local_check_error(
                WorkflowOsErrorKind::Validation,
                "local_check.process.executable_required",
                "local check process executable is required",
            ));
        }
        validate_arguments(&arguments)?;
        validate_process_environment(&environment)?;
        if timeout.is_zero() {
            return Err(local_check_error(
                WorkflowOsErrorKind::Validation,
                "local_check.process.timeout_required",
                "local check process timeout is required",
            ));
        }

        Ok(Self {
            executable,
            arguments,
            working_directory,
            environment,
            timeout,
        })
    }

    /// Returns the executable path.
    #[must_use]
    pub fn executable(&self) -> &Path {
        &self.executable
    }

    /// Returns the fixed argument vector.
    #[must_use]
    pub fn arguments(&self) -> &[String] {
        &self.arguments
    }

    /// Returns the working directory.
    #[must_use]
    pub fn working_directory(&self) -> &Path {
        &self.working_directory
    }

    /// Returns the sanitized environment map.
    #[must_use]
    pub fn environment(&self) -> &BTreeMap<String, String> {
        &self.environment
    }

    /// Returns the timeout.
    #[must_use]
    pub const fn timeout(&self) -> Duration {
        self.timeout
    }
}

impl fmt::Debug for LocalCheckProcessRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalCheckProcessRequest")
            .field("executable", &"[REDACTED]")
            .field("argument_count", &self.arguments.len())
            .field("working_directory", &"[REDACTED]")
            .field("environment_key_count", &self.environment.len())
            .field("timeout", &self.timeout)
            .finish()
    }
}

/// Bounded process output for a local check handler.
#[derive(Clone, Eq, PartialEq)]
pub struct LocalCheckProcessOutput {
    exit_code: Option<i32>,
    success: bool,
    duration_ms: u64,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
    timed_out: bool,
}

impl LocalCheckProcessOutput {
    /// Creates process output for a completed process.
    #[must_use]
    pub fn completed(
        exit_code: Option<i32>,
        success: bool,
        duration_ms: u64,
        stdout: Vec<u8>,
        stderr: Vec<u8>,
    ) -> Self {
        Self {
            exit_code,
            success,
            duration_ms,
            stdout,
            stderr,
            timed_out: false,
        }
    }

    /// Creates process output for a timed-out process.
    #[must_use]
    pub fn timed_out(duration_ms: u64, stdout: Vec<u8>, stderr: Vec<u8>) -> Self {
        Self {
            exit_code: None,
            success: false,
            duration_ms,
            stdout,
            stderr,
            timed_out: true,
        }
    }
}

impl fmt::Debug for LocalCheckProcessOutput {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalCheckProcessOutput")
            .field("exit_code", &self.exit_code)
            .field("success", &self.success)
            .field("duration_ms", &self.duration_ms)
            .field("stdout_byte_count", &self.stdout.len())
            .field("stderr_byte_count", &self.stderr.len())
            .field("stdout", &"[REDACTED]")
            .field("stderr", &"[REDACTED]")
            .field("timed_out", &self.timed_out)
            .finish()
    }
}

/// Process runner boundary for local check handlers.
pub trait LocalCheckProcessRunner: Send + Sync + fmt::Debug {
    /// Runs one local check process request.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error if the process cannot be started or
    /// observed.
    fn run(
        &self,
        request: &LocalCheckProcessRequest,
    ) -> Result<LocalCheckProcessOutput, WorkflowOsError>;
}

#[derive(Debug)]
struct StdLocalCheckProcessRunner;

impl LocalCheckProcessRunner for StdLocalCheckProcessRunner {
    fn run(
        &self,
        request: &LocalCheckProcessRequest,
    ) -> Result<LocalCheckProcessOutput, WorkflowOsError> {
        run_process_with_timeout(request)
    }
}

struct BoundedOutput {
    summary: String,
    truncated: bool,
}

fn bounded_redacted_output(
    bytes: &[u8],
    max_bytes: usize,
    stream_name: &'static str,
) -> Result<BoundedOutput, WorkflowOsError> {
    let take = bytes.len().min(max_bytes);
    let mut summary = String::from_utf8_lossy(&bytes[..take]).into_owned();
    summary = summary.replace('\0', "");
    let truncated = bytes.len() > max_bytes;
    if looks_secret_like(&summary) {
        return Err(local_check_error(
            WorkflowOsErrorKind::Validation,
            "local_check.output.secret_like",
            format!("test-only local check {stream_name} output contains sensitive-looking text"),
        ));
    }
    Ok(BoundedOutput { summary, truncated })
}

fn run_process_with_timeout(
    request: &LocalCheckProcessRequest,
) -> Result<LocalCheckProcessOutput, WorkflowOsError> {
    let started_at = Instant::now();
    let mut child = Command::new(request.executable())
        .args(request.arguments())
        .current_dir(request.working_directory())
        .env_clear()
        .envs(request.environment())
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|_error| {
            local_check_error(
                WorkflowOsErrorKind::Internal,
                "local_check.handler.process_failed",
                "test-only local check process failed",
            )
        })?;
    loop {
        if child
            .try_wait()
            .map_err(|_error| {
                local_check_error(
                    WorkflowOsErrorKind::Internal,
                    "local_check.handler.process_failed",
                    "test-only local check process failed",
                )
            })?
            .is_some()
        {
            let duration_ms = duration_millis(started_at.elapsed());
            let output = child.wait_with_output().map_err(|_error| {
                local_check_error(
                    WorkflowOsErrorKind::Internal,
                    "local_check.handler.process_failed",
                    "test-only local check process failed",
                )
            })?;
            return Ok(LocalCheckProcessOutput::completed(
                output.status.code(),
                output.status.success(),
                duration_ms,
                output.stdout,
                output.stderr,
            ));
        }
        if started_at.elapsed() >= request.timeout() {
            let _ = child.kill();
            let duration_ms = duration_millis(started_at.elapsed());
            let output = child.wait_with_output().map_err(|_error| {
                local_check_error(
                    WorkflowOsErrorKind::Internal,
                    "local_check.handler.process_failed",
                    "test-only local check process failed",
                )
            })?;
            return Ok(LocalCheckProcessOutput::timed_out(
                duration_ms,
                output.stdout,
                output.stderr,
            ));
        }
        thread::sleep(Duration::from_millis(10));
    }
}

fn sanitized_environment() -> Result<BTreeMap<String, String>, WorkflowOsError> {
    let mut environment = BTreeMap::new();
    environment.insert(
        "PATH".to_owned(),
        "/usr/bin:/bin:/usr/sbin:/sbin".to_owned(),
    );
    validate_process_environment(&environment)?;
    Ok(environment)
}

fn docs_check_environment(
    npm_cache_directory: Option<&Path>,
) -> Result<BTreeMap<String, String>, WorkflowOsError> {
    let mut environment = sanitized_environment()?;
    if let Some(cache_directory) = npm_cache_directory {
        environment.insert(
            "NPM_CONFIG_CACHE".to_owned(),
            cache_directory.to_string_lossy().into_owned(),
        );
    }
    validate_process_environment(&environment)?;
    Ok(environment)
}

fn duration_millis(duration: Duration) -> u64 {
    u64::try_from(duration.as_millis()).unwrap_or(u64::MAX)
}

impl fmt::Debug for LocalCheckCommandContract {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalCheckCommandContract")
            .field("command_id", &self.command_id)
            .field("command_kind", &self.command_kind)
            .field("execution_posture", &self.execution_posture)
            .field("executable", &"[REDACTED]")
            .field("argument_count", &self.arguments.len())
            .field("working_directory_policy", &self.working_directory_policy)
            .field("environment_policy", &self.environment_policy)
            .field(
                "allowed_environment_variable_count",
                &self.allowed_environment_variables.len(),
            )
            .field("network_policy", &self.network_policy)
            .field("timeout_seconds", &self.timeout_seconds)
            .field("side_effect_class", &self.side_effect_class)
            .field(
                "permitted_output_directory_count",
                &self.permitted_output_directories.len(),
            )
            .field("output_capture", &self.output_capture)
            .field("redaction_policy", &self.redaction_policy)
            .field("citation_kind_count", &self.citation_kinds.len())
            .finish()
    }
}

impl<'de> Deserialize<'de> for LocalCheckCommandContract {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct LocalCheckCommandContractWire {
            command_id: LocalCheckCommandId,
            command_kind: LocalCheckCommandKind,
            execution_posture: LocalCheckExecutionPosture,
            executable: String,
            arguments: Vec<String>,
            working_directory_policy: LocalCheckWorkingDirectoryPolicy,
            environment_policy: LocalCheckEnvironmentPolicy,
            allowed_environment_variables: Vec<String>,
            network_policy: LocalCheckNetworkPolicy,
            timeout_seconds: u32,
            side_effect_class: LocalCheckSideEffectClass,
            permitted_output_directories: Vec<String>,
            output_capture: LocalCheckOutputCapturePolicy,
            redaction_policy: LocalCheckRedactionPolicy,
            citation_kinds: Vec<WorkReportCitationKind>,
        }

        let wire = LocalCheckCommandContractWire::deserialize(deserializer)?;
        Self::new(LocalCheckCommandContractDefinition {
            command_id: wire.command_id,
            command_kind: wire.command_kind,
            execution_posture: wire.execution_posture,
            executable: wire.executable,
            arguments: wire.arguments,
            working_directory_policy: wire.working_directory_policy,
            environment_policy: wire.environment_policy,
            allowed_environment_variables: wire.allowed_environment_variables,
            network_policy: wire.network_policy,
            timeout_seconds: wire.timeout_seconds,
            side_effect_class: wire.side_effect_class,
            permitted_output_directories: wire.permitted_output_directories,
            output_capture: wire.output_capture,
            redaction_policy: wire.redaction_policy,
            citation_kinds: wire.citation_kinds,
        })
        .map_err(serde::de::Error::custom)
    }
}

fn validate_identifier(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "local_check.identifier.empty",
            format!("{type_name} cannot be empty"),
        ));
    }

    if value.len() > LOCAL_CHECK_ID_MAX_BYTES {
        return Err(validation_error(
            "local_check.identifier.too_long",
            format!("{type_name} cannot exceed {LOCAL_CHECK_ID_MAX_BYTES} bytes"),
        ));
    }

    let is_valid = value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/'));

    if !is_valid {
        return Err(validation_error(
            "local_check.identifier.invalid_character",
            format!("{type_name} contains an invalid character"),
        ));
    }

    validate_not_secret_like(type_name, value)
}

fn validate_arguments(arguments: &[String]) -> Result<(), WorkflowOsError> {
    if arguments.len() > LOCAL_CHECK_ARG_MAX_COUNT {
        return Err(validation_error(
            "local_check.arguments.too_many",
            "local check command contracts include too many arguments",
        ));
    }

    for argument in arguments {
        validate_command_token("local check argument", argument)?;
    }

    Ok(())
}

fn validate_command_template(
    command_kind: LocalCheckCommandKind,
    executable: &str,
    arguments: &[String],
) -> Result<(), WorkflowOsError> {
    let template = command_kind.template();
    let matches_template = executable == template.executable
        && arguments.len() == template.arguments.len()
        && arguments
            .iter()
            .map(String::as_str)
            .eq(template.arguments.iter().copied());

    if !matches_template {
        return Err(validation_error(
            "local_check.command_template.mismatch",
            "local check command kind must match its canonical executable and argument template",
        ));
    }

    Ok(())
}

fn validate_command_token(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "local_check.command_token.empty",
            format!("{type_name} cannot be empty"),
        ));
    }

    if value.len() > LOCAL_CHECK_ARG_MAX_BYTES {
        return Err(validation_error(
            "local_check.command_token.too_long",
            format!("{type_name} cannot exceed {LOCAL_CHECK_ARG_MAX_BYTES} bytes"),
        ));
    }

    if value.bytes().any(is_shell_metacharacter) {
        return Err(validation_error(
            "local_check.command_token.shell_metacharacter",
            format!("{type_name} contains shell metacharacters"),
        ));
    }

    validate_not_secret_like(type_name, value)
}

fn is_shell_metacharacter(byte: u8) -> bool {
    matches!(
        byte,
        b'|' | b'&'
            | b';'
            | b'>'
            | b'<'
            | b'`'
            | b'$'
            | b'('
            | b')'
            | b'{'
            | b'}'
            | b'*'
            | b'?'
            | b'!'
            | b'~'
            | b'\''
            | b'"'
            | b'\\'
            | b' '
            | b'\t'
            | b'\n'
            | b'\r'
    )
}

fn validate_environment_variables(values: &[String]) -> Result<(), WorkflowOsError> {
    if values.len() > LOCAL_CHECK_ENV_MAX_COUNT {
        return Err(validation_error(
            "local_check.environment.too_many",
            "local check command contracts allow too many environment variables",
        ));
    }

    for value in values {
        validate_environment_variable_name(value)?;
    }

    Ok(())
}

fn validate_process_environment(
    environment: &BTreeMap<String, String>,
) -> Result<(), WorkflowOsError> {
    if environment.len() > LOCAL_CHECK_ENV_MAX_COUNT {
        return Err(validation_error(
            "local_check.environment.too_many",
            "local check process environment includes too many variables",
        ));
    }

    for (key, value) in environment {
        validate_environment_variable_name(key)?;
        if value.len() > LOCAL_CHECK_ARG_MAX_BYTES {
            return Err(validation_error(
                "local_check.environment.value_too_long",
                "local check process environment value exceeds the supported maximum",
            ));
        }
        validate_not_secret_like("local check process environment value", value)?;
    }

    Ok(())
}

fn validate_environment_variable_name(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "local_check.environment.empty",
            "local check environment variable name cannot be empty",
        ));
    }

    let is_valid = value.bytes().enumerate().all(|(index, byte)| {
        if index == 0 {
            byte.is_ascii_alphabetic() || byte == b'_'
        } else {
            byte.is_ascii_alphanumeric() || byte == b'_'
        }
    });

    if !is_valid {
        return Err(validation_error(
            "local_check.environment.invalid",
            "local check environment variable name is invalid",
        ));
    }

    validate_not_secret_like("local check environment variable", value)
}

fn validate_result_summary(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if value.len() > LOCAL_CHECK_OUTPUT_MAX_BYTES {
        return Err(validation_error(
            "local_check.result.summary_too_large",
            format!("{type_name} exceeds the supported maximum"),
        ));
    }

    validate_not_secret_like(type_name, value)
}

fn validate_output_directories(values: &[String]) -> Result<(), WorkflowOsError> {
    for value in values {
        validate_command_token("local check output directory", value)?;
        if value.starts_with('/') || value.contains("..") {
            return Err(validation_error(
                "local_check.output_directory.invalid",
                "local check output directories must be relative safe paths",
            ));
        }
    }

    Ok(())
}

fn validate_timeout(timeout_seconds: u32) -> Result<(), WorkflowOsError> {
    if timeout_seconds == 0 {
        return Err(validation_error(
            "local_check.timeout.required",
            "local check command contracts require a timeout",
        ));
    }

    if timeout_seconds > LOCAL_CHECK_TIMEOUT_MAX_SECONDS {
        return Err(validation_error(
            "local_check.timeout.too_large",
            "local check timeout exceeds the supported maximum",
        ));
    }

    Ok(())
}

fn validate_citation_kinds(
    citation_kinds: &[WorkReportCitationKind],
) -> Result<(), WorkflowOsError> {
    let mut seen = BTreeSet::new();
    for kind in citation_kinds {
        if !seen.insert(*kind) {
            return Err(validation_error(
                "local_check.citation.duplicate",
                "local check command contracts cannot repeat citation kinds",
            ));
        }
    }
    Ok(())
}

fn validate_reference_redaction_metadata(
    redaction: &RedactionMetadata,
) -> Result<(), WorkflowOsError> {
    if redaction.redacted_fields.len() > LOCAL_CHECK_ENV_MAX_COUNT
        || redaction.field_states.len() > LOCAL_CHECK_ENV_MAX_COUNT
    {
        return Err(validation_error(
            "local_check_result_reference.redaction.too_many",
            "local check result reference redaction metadata has too many entries",
        ));
    }

    for field in &redaction.redacted_fields {
        validate_redaction_field(field)?;
    }

    for state in &redaction.field_states {
        validate_redaction_field(&state.field)?;
        validate_redaction_reason(&state.reason)?;
    }

    Ok(())
}

fn validate_redaction_field(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "local_check_result_reference.redaction.field_empty",
            "local check result reference redaction field cannot be empty",
        ));
    }

    if value.len() > LOCAL_CHECK_REDACTION_FIELD_MAX_BYTES {
        return Err(validation_error(
            "local_check_result_reference.redaction.field_too_long",
            "local check result reference redaction field exceeds the supported maximum",
        ));
    }

    validate_identifier("local check result reference redaction field", value)
}

fn validate_redaction_reason(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "local_check_result_reference.redaction.reason_empty",
            "local check result reference redaction reason cannot be empty",
        ));
    }

    if value.len() > LOCAL_CHECK_REDACTION_REASON_MAX_BYTES {
        return Err(validation_error(
            "local_check_result_reference.redaction.reason_too_long",
            "local check result reference redaction reason exceeds the supported maximum",
        ));
    }

    validate_not_secret_like("local check result reference redaction reason", value)
}

fn validate_not_secret_like(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if looks_secret_like(value) {
        return Err(validation_error(
            "local_check.secret_like_value",
            format!("{type_name} contains sensitive-looking text"),
        ));
    }

    Ok(())
}

fn looks_secret_like(value: &str) -> bool {
    let lowercase = value.to_ascii_lowercase();
    lowercase.contains("authorization")
        || lowercase.contains("bearer")
        || lowercase.contains("private_key")
        || lowercase.contains("private-key")
        || lowercase.contains("api_token")
        || lowercase.contains("api-token")
        || lowercase.contains("secret")
        || lowercase.contains("token")
}

fn local_check_error(
    kind: WorkflowOsErrorKind,
    code: &'static str,
    message: impl Into<String>,
) -> WorkflowOsError {
    WorkflowOsError::new(kind, code, message)
}

fn validation_error(code: &'static str, message: impl Into<String>) -> WorkflowOsError {
    WorkflowOsError::validation(code, message)
}
