use std::collections::BTreeSet;
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize};

use crate::{WorkReportCitationKind, WorkflowOsError};

const LOCAL_CHECK_ID_MAX_BYTES: usize = 128;
const LOCAL_CHECK_ARG_MAX_BYTES: usize = 256;
const LOCAL_CHECK_ARG_MAX_COUNT: usize = 32;
const LOCAL_CHECK_ENV_MAX_COUNT: usize = 16;
const LOCAL_CHECK_OUTPUT_MAX_BYTES: usize = 64 * 1024;
const LOCAL_CHECK_TIMEOUT_MAX_SECONDS: u32 = 30 * 60;

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

fn validate_not_secret_like(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    let lowercase = value.to_ascii_lowercase();
    let is_secret_like = lowercase.contains("authorization")
        || lowercase.contains("bearer")
        || lowercase.contains("private_key")
        || lowercase.contains("private-key")
        || lowercase.contains("api_token")
        || lowercase.contains("api-token")
        || lowercase.contains("secret")
        || lowercase.contains("token");

    if is_secret_like {
        return Err(validation_error(
            "local_check.secret_like_value",
            format!("{type_name} contains sensitive-looking text"),
        ));
    }

    Ok(())
}

fn validation_error(code: &'static str, message: impl Into<String>) -> WorkflowOsError {
    WorkflowOsError::validation(code, message)
}
