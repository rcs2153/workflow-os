use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    ActorId, AdapterId, CorrelationId, IdempotencyKey, RedactionDisposition, RedactionFieldState,
    RedactionMetadata, SchemaVersion, SpecContentHash, Timestamp, WorkflowId, WorkflowOsError,
    WorkflowOsErrorKind, WorkflowRunId, WorkflowVersion,
};

/// Symbolic adapter kind for future integration implementations.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AdapterKind {
    /// GitHub adapter kind.
    GitHub,
    /// Jira adapter kind.
    Jira,
    /// Future CI adapter kind.
    Ci,
    /// Local deterministic adapter-like boundary.
    Local,
    /// Future generic HTTP adapter kind.
    GenericHttp,
    /// Unknown adapter kind. Unknown kinds must fail closed.
    Unknown(String),
}

/// Capability an adapter operation requires.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterCapability {
    /// Read from GitHub.
    #[serde(rename = "github.read")]
    GitHubRead,
    /// Write to GitHub. Unsupported in Phase 2.
    #[serde(rename = "github.write")]
    GitHubWrite,
    /// Read from Jira.
    #[serde(rename = "jira.read")]
    JiraRead,
    /// Write to Jira. Unsupported in Phase 2.
    #[serde(rename = "jira.write")]
    JiraWrite,
    /// Read from a CI system.
    #[serde(rename = "ci.read")]
    CiRead,
    /// Write to a CI system. Unsupported in Phase 2.
    #[serde(rename = "ci.write")]
    CiWrite,
    /// Rerun CI. Unsupported in Phase 2.
    #[serde(rename = "ci.rerun")]
    CiRerun,
    /// Generic adapter write. Unsupported in Phase 2.
    #[serde(rename = "adapter.write")]
    AdapterWrite,
    /// Read from an external system.
    ExternalRead,
    /// Write to an external system.
    ExternalWrite,
    /// Read local deterministic data.
    LocalRead,
    /// Write local deterministic data.
    LocalWrite,
    /// Subscribe to or poll external events.
    EventSource,
    /// Discover adapter capabilities.
    CapabilityDiscovery,
    /// Run a plan without performing side effects.
    DryRun,
    /// Unknown capability. Unknown capabilities must fail closed.
    Unknown(String),
}

/// Adapter action requested by the runtime.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AdapterAction {
    /// Stable action name.
    pub name: String,
    /// Whether the action may create external side effects.
    pub side_effecting: bool,
    /// Capabilities required to perform the action.
    pub required_capabilities: Vec<AdapterCapability>,
}

impl AdapterAction {
    /// Creates a read-only adapter action.
    #[must_use]
    pub fn read(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            side_effecting: false,
            required_capabilities: vec![AdapterCapability::ExternalRead],
        }
    }

    /// Creates a write-capable adapter action.
    #[must_use]
    pub fn write(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            side_effecting: true,
            required_capabilities: vec![AdapterCapability::ExternalWrite],
        }
    }
}

/// Phase-specific adapter execution mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AdapterOperationMode {
    /// Static fixture-backed adapter behavior.
    Fixture,
    /// Mock adapter behavior for tests.
    Mock,
    /// Local non-network adapter behavior.
    Local,
    /// Live read-only calls to an external service.
    LiveReadOnly,
    /// Live write-capable mode. Defined for future contracts but denied in Phase 2.
    LiveWriteCapable,
}

/// Immutable workflow/run identity attached to run-scoped adapter requests.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AdapterRunScope {
    /// Workflow run ID.
    pub workflow_run_id: WorkflowRunId,
    /// Workflow ID.
    pub workflow_id: WorkflowId,
    /// Workflow version.
    pub workflow_version: WorkflowVersion,
    /// Workflow spec schema version.
    pub schema_version: SchemaVersion,
    /// Workflow spec content hash.
    pub spec_hash: SpecContentHash,
}

/// Adapter request redaction policy.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AdapterRedactionPolicy {
    /// Strategy for request and response payload handling.
    pub strategy: AdapterRedactionStrategy,
    /// Fields or paths that must not be recorded raw.
    #[serde(default)]
    pub sensitive_fields: Vec<String>,
}

/// Adapter timeout policy preserved at the contract boundary.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AdapterTimeoutPolicy {
    /// Timeout duration in milliseconds.
    pub timeout_ms: u64,
}

/// Response size metadata for adapter summaries.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct AdapterResponseSize {
    /// Approximate byte size of the original response, if known.
    pub original_bytes: Option<u64>,
    /// Approximate byte size of the stored summary.
    pub stored_summary_bytes: u64,
    /// Whether the stored summary was truncated.
    pub truncated: bool,
}

/// Adapter response status.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterResponseStatus {
    /// Operation succeeded.
    Success,
    /// Operation failed.
    Failure,
}

/// Result of policy pre-check before adapter invocation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AdapterPolicyPrecheck {
    /// Policy allowed the action.
    Allowed {
        /// Where this policy pre-check came from.
        provenance: AdapterPolicyPrecheckProvenance,
        /// Stable policy reason codes.
        reason_codes: Vec<String>,
    },
    /// A human approval decision allowed the action.
    ApprovalGranted {
        /// Approval request identifier.
        approval_id: String,
        /// Where this approval pre-check came from.
        provenance: AdapterPolicyPrecheckProvenance,
        /// Stable policy reason codes.
        reason_codes: Vec<String>,
    },
    /// No successful policy or approval pre-check exists.
    Missing,
    /// Policy explicitly denied the action.
    Denied {
        /// Where this policy pre-check came from.
        provenance: AdapterPolicyPrecheckProvenance,
        /// Stable denial reason codes.
        reason_codes: Vec<String>,
    },
}

impl AdapterPolicyPrecheck {
    /// Creates an allowed pre-check produced by runtime policy evaluation.
    #[must_use]
    pub fn runtime_allowed(reason_codes: Vec<String>) -> Self {
        Self::Allowed {
            provenance: AdapterPolicyPrecheckProvenance::RuntimePolicy,
            reason_codes,
        }
    }

    /// Creates an allowed pre-check for fixture or test-only adapter paths.
    #[must_use]
    pub fn fixture_test_allowed(reason_codes: Vec<String>) -> Self {
        Self::Allowed {
            provenance: AdapterPolicyPrecheckProvenance::FixtureTest,
            reason_codes,
        }
    }

    /// Creates an approval-granted pre-check produced by runtime approval handling.
    #[must_use]
    pub fn approval_granted(approval_id: String, reason_codes: Vec<String>) -> Self {
        Self::ApprovalGranted {
            approval_id,
            provenance: AdapterPolicyPrecheckProvenance::ApprovalDecision,
            reason_codes,
        }
    }

    /// Creates a denied pre-check produced by runtime policy evaluation.
    #[must_use]
    pub fn runtime_denied(reason_codes: Vec<String>) -> Self {
        Self::Denied {
            provenance: AdapterPolicyPrecheckProvenance::RuntimePolicy,
            reason_codes,
        }
    }

    /// Creates a denied pre-check for fixture or test-only adapter paths.
    #[must_use]
    pub fn fixture_test_denied(reason_codes: Vec<String>) -> Self {
        Self::Denied {
            provenance: AdapterPolicyPrecheckProvenance::FixtureTest,
            reason_codes,
        }
    }

    fn allows_operation(&self) -> bool {
        matches!(self, Self::Allowed { .. } | Self::ApprovalGranted { .. })
    }
}

/// Provenance for adapter policy pre-checks.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterPolicyPrecheckProvenance {
    /// Runtime policy engine evaluated and allowed or denied the operation.
    RuntimePolicy,
    /// Runtime approval decision authorized the operation.
    ApprovalDecision,
    /// Fixture or test-only path authorized the operation.
    FixtureTest,
}

/// Strategy for adapter idempotency.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterIdempotencyStrategy {
    /// Runtime supplied a concrete idempotency key.
    RuntimeKey,
    /// Adapter derives an idempotency key from non-secret request references.
    AdapterDerived,
    /// Operation is read-only and does not require idempotency.
    NotRequiredForReadOnly,
}

/// Strategy for redacting adapter inputs and outputs.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterRedactionStrategy {
    /// Store only references to sensitive payloads.
    ReferenceOnly,
    /// Store non-sensitive summaries.
    SummaryOnly,
    /// Fully redact payload fields.
    Full,
}

/// Request passed to an adapter implementation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AdapterRequest {
    /// Adapter identifier.
    pub adapter_id: AdapterId,
    /// Symbolic adapter kind.
    pub adapter_kind: AdapterKind,
    /// Requested action.
    pub action: AdapterAction,
    /// Primary declared capability for this request.
    pub capability: AdapterCapability,
    /// Fixture, mock, local, live read-only, or future live write-capable mode.
    pub operation_mode: AdapterOperationMode,
    /// Correlation ID propagated from runtime.
    pub correlation_id: CorrelationId,
    /// Actor or system actor responsible for the request.
    pub actor: ActorId,
    /// Workflow/run identity when the adapter call is run-scoped.
    pub run_scope: Option<AdapterRunScope>,
    /// Idempotency key for side-effecting actions.
    pub idempotency_key: Option<IdempotencyKey>,
    /// Non-secret input reference.
    pub input_reference: Option<String>,
    /// Declared idempotency strategy.
    pub idempotency_strategy: AdapterIdempotencyStrategy,
    /// Declared redaction policy.
    pub redaction_policy: AdapterRedactionPolicy,
    /// Timeout policy for this adapter request.
    pub timeout_policy: AdapterTimeoutPolicy,
    /// Non-secret request metadata.
    #[serde(default)]
    pub metadata: BTreeMap<String, String>,
    /// Policy or approval result captured before invocation.
    pub policy_precheck: AdapterPolicyPrecheck,
}

impl AdapterRequest {
    /// Returns true when this request may cause an external side effect.
    #[must_use]
    pub fn is_side_effecting(&self) -> bool {
        self.action.side_effecting
            || self
                .action
                .required_capabilities
                .iter()
                .any(AdapterCapability::is_write_capability)
            || self.capability.is_write_capability()
    }

    /// Validates safety preconditions before an adapter operation may run.
    ///
    /// # Errors
    ///
    /// Returns an error when required metadata is missing, read capability is
    /// absent, a write is attempted during Phase 2, or policy is missing.
    pub fn validate_preconditions(&self) -> Result<(), AdapterError> {
        if matches!(self.operation_mode, AdapterOperationMode::LiveWriteCapable) {
            return Err(AdapterError::new(
                AdapterErrorKind::PolicyDenied,
                "adapter.phase2.write_mode_denied",
                "live write-capable adapter mode is not enabled in Phase 2",
            ));
        }

        if self.actor.as_str().is_empty() {
            return Err(AdapterError::new(
                AdapterErrorKind::ValidationFailure,
                "adapter.actor.required",
                "adapter requests require an actor or system actor",
            ));
        }

        if self.timeout_policy.timeout_ms == 0 {
            return Err(AdapterError::new(
                AdapterErrorKind::ValidationFailure,
                "adapter.timeout.required",
                "adapter requests require a non-zero timeout policy",
            ));
        }

        if self
            .action
            .required_capabilities
            .iter()
            .chain(std::iter::once(&self.capability))
            .any(|capability| matches!(capability, AdapterCapability::Unknown(_)))
        {
            return Err(AdapterError::new(
                AdapterErrorKind::PermissionFailure,
                "adapter.capability.unknown",
                "unknown adapter capability is denied",
            ));
        }

        if !self.capability.is_read_capability() && !self.capability.is_write_capability() {
            return Err(AdapterError::new(
                AdapterErrorKind::PermissionFailure,
                "adapter.capability.read_required",
                "read-only adapter actions require a declared read capability",
            ));
        }

        if self.action.required_capabilities.is_empty()
            || !self
                .action
                .required_capabilities
                .iter()
                .any(AdapterCapability::is_read_capability)
                && !self
                    .action
                    .required_capabilities
                    .iter()
                    .any(AdapterCapability::is_write_capability)
        {
            return Err(AdapterError::new(
                AdapterErrorKind::PermissionFailure,
                "adapter.capability.required",
                "adapter actions require at least one declared read or write capability",
            ));
        }

        if self.is_side_effecting() {
            return Err(AdapterError::new(
                AdapterErrorKind::PolicyDenied,
                "adapter.phase2.write_denied",
                "write-capable adapter actions are denied in Phase 2",
            ));
        }

        if !self.policy_precheck.allows_operation() {
            return Err(AdapterError::new(
                AdapterErrorKind::PolicyDenied,
                "adapter.policy.required",
                "adapter actions require policy allow or approval before invocation",
            ));
        }

        if self.idempotency_key.is_none()
            && self.idempotency_strategy != AdapterIdempotencyStrategy::NotRequiredForReadOnly
        {
            return Err(AdapterError::new(
                AdapterErrorKind::ValidationFailure,
                "adapter.idempotency.strategy_mismatch",
                "adapter idempotency strategy requires an idempotency key",
            ));
        }
        Ok(())
    }
}

impl AdapterCapability {
    /// Returns true for read-only adapter capabilities.
    #[must_use]
    pub const fn is_read_capability(&self) -> bool {
        matches!(
            self,
            Self::GitHubRead | Self::JiraRead | Self::CiRead | Self::ExternalRead | Self::LocalRead
        )
    }

    /// Returns true for write-capable adapter capabilities.
    #[must_use]
    pub const fn is_write_capability(&self) -> bool {
        matches!(
            self,
            Self::GitHubWrite
                | Self::JiraWrite
                | Self::CiWrite
                | Self::CiRerun
                | Self::AdapterWrite
                | Self::ExternalWrite
                | Self::LocalWrite
        )
    }
}

/// Adapter response safe for audit by default.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AdapterResponse {
    /// Adapter identifier.
    pub adapter_id: AdapterId,
    /// Action name.
    pub action: String,
    /// Success or failure status.
    pub status: AdapterResponseStatus,
    /// Non-sensitive response summary.
    pub summary: String,
    /// External object references.
    pub external_references: Vec<String>,
    /// Redaction metadata describing hidden fields.
    pub redaction: RedactionMetadata,
    /// Response size metadata.
    pub size: AdapterResponseSize,
    /// Correlation ID propagated from request.
    pub correlation_id: CorrelationId,
    /// Duration in milliseconds.
    pub duration_ms: u64,
    /// Non-secret warnings.
    pub warnings: Vec<String>,
}

impl AdapterResponse {
    /// Creates a response while redacting sensitive-looking summary text.
    #[must_use]
    pub fn redacted_summary(
        adapter_id: AdapterId,
        action: impl Into<String>,
        correlation_id: CorrelationId,
        summary: impl Into<String>,
        external_references: Vec<String>,
        duration_ms: u64,
    ) -> Self {
        let summary = summary.into();
        let stored_summary_bytes = summary.len() as u64;
        let (summary, redaction) = redact_text(summary);
        Self {
            adapter_id,
            action: action.into(),
            status: AdapterResponseStatus::Success,
            summary,
            external_references,
            redaction,
            size: AdapterResponseSize {
                original_bytes: None,
                stored_summary_bytes,
                truncated: false,
            },
            correlation_id,
            duration_ms,
            warnings: Vec::new(),
        }
    }
}

/// Adapter failure classification.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterErrorKind {
    /// Authentication failed.
    AuthFailure,
    /// Permission denied.
    PermissionFailure,
    /// Target was not found.
    NotFound,
    /// Rate limit.
    RateLimited,
    /// Operation timed out.
    Timeout,
    /// Request failed validation.
    ValidationFailure,
    /// Adapter response was malformed.
    MalformedResponse,
    /// Transient network failure.
    TransientNetworkFailure,
    /// Operation is unsupported.
    UnsupportedOperation,
    /// Policy denied the operation.
    PolicyDenied,
    /// Unknown failure.
    Unknown,
}

/// Structured adapter error.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AdapterError {
    /// Adapter error kind.
    pub kind: AdapterErrorKind,
    /// Stable code.
    pub code: String,
    /// Non-secret message.
    pub message: String,
}

impl AdapterError {
    /// Creates a structured adapter error.
    #[must_use]
    pub fn new(
        kind: AdapterErrorKind,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            code: code.into(),
            message: message.into(),
        }
    }
}

impl From<AdapterError> for WorkflowOsError {
    fn from(error: AdapterError) -> Self {
        WorkflowOsError::new(WorkflowOsErrorKind::InvalidState, error.code, error.message)
    }
}

/// Event emitted by an event-source adapter.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AdapterEvent {
    /// Adapter ID.
    pub adapter_id: AdapterId,
    /// Event kind from the adapter boundary.
    pub event_kind: String,
    /// Non-secret external reference.
    pub external_reference: Option<String>,
    /// Correlation ID for runtime linkage.
    pub correlation_id: CorrelationId,
    /// Event timestamp.
    pub observed_at: Timestamp,
}

/// Adapter health result.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AdapterHealth {
    /// Adapter ID.
    pub adapter_id: AdapterId,
    /// Adapter kind.
    pub adapter_kind: AdapterKind,
    /// Fixture, mock, local, live read-only, or future live write-capable mode.
    pub operation_mode: AdapterOperationMode,
    /// Whether the adapter has required configuration.
    pub configured: bool,
    /// Whether the adapter is reachable, if reachability can be tested.
    pub reachable: Option<bool>,
    /// Whether credentials are present without exposing credential values.
    pub credential_present: bool,
    /// Last health check timestamp.
    pub last_checked_at: Timestamp,
    /// Non-secret warnings.
    pub warnings: Vec<String>,
}

/// Record of an adapter invocation suitable for audit projections.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AdapterInvocationRecord {
    /// Adapter ID.
    pub adapter_id: AdapterId,
    /// Adapter kind.
    pub adapter_kind: AdapterKind,
    /// Operation mode.
    pub operation_mode: AdapterOperationMode,
    /// Action name.
    pub action: String,
    /// Required capability.
    pub capability: AdapterCapability,
    /// Actor or system actor.
    pub actor: ActorId,
    /// Workflow/run identity when scoped to a run.
    pub run_scope: Option<AdapterRunScope>,
    /// Success or failure status.
    pub status: AdapterResponseStatus,
    /// Correlation ID.
    pub correlation_id: CorrelationId,
    /// Idempotency key where side effects are possible.
    pub idempotency_key: Option<IdempotencyKey>,
    /// Non-secret input reference.
    pub input_reference: Option<String>,
    /// Non-secret output reference.
    pub output_reference: Option<String>,
    /// Classified adapter error for failures.
    pub error_kind: Option<AdapterErrorKind>,
    /// Non-secret warnings.
    pub warnings: Vec<String>,
    /// Duration in milliseconds.
    pub duration_ms: u64,
    /// Redaction metadata.
    pub redaction: RedactionMetadata,
    /// Timestamp.
    pub invoked_at: Timestamp,
}

impl AdapterInvocationRecord {
    /// Builds an audit-safe invocation record from a successful response.
    #[must_use]
    pub fn from_response(
        request: &AdapterRequest,
        response: &AdapterResponse,
        invoked_at: Timestamp,
    ) -> Self {
        Self {
            adapter_id: request.adapter_id.clone(),
            adapter_kind: request.adapter_kind.clone(),
            operation_mode: request.operation_mode,
            action: request.action.name.clone(),
            capability: request.capability.clone(),
            actor: request.actor.clone(),
            run_scope: request.run_scope.clone(),
            status: response.status,
            correlation_id: request.correlation_id.clone(),
            idempotency_key: request.idempotency_key.clone(),
            input_reference: request.input_reference.clone(),
            output_reference: response.external_references.first().cloned(),
            error_kind: None,
            warnings: response.warnings.clone(),
            duration_ms: response.duration_ms,
            redaction: response.redaction.clone(),
            invoked_at,
        }
    }

    /// Builds an audit-safe invocation record from a failed adapter call.
    #[must_use]
    pub fn from_error(
        request: &AdapterRequest,
        error: &AdapterError,
        duration_ms: u64,
        invoked_at: Timestamp,
    ) -> Self {
        Self {
            adapter_id: request.adapter_id.clone(),
            adapter_kind: request.adapter_kind.clone(),
            operation_mode: request.operation_mode,
            action: request.action.name.clone(),
            capability: request.capability.clone(),
            actor: request.actor.clone(),
            run_scope: request.run_scope.clone(),
            status: AdapterResponseStatus::Failure,
            correlation_id: request.correlation_id.clone(),
            idempotency_key: request.idempotency_key.clone(),
            input_reference: request.input_reference.clone(),
            output_reference: None,
            error_kind: Some(error.kind),
            warnings: Vec::new(),
            duration_ms,
            redaction: RedactionMetadata::empty(),
            invoked_at,
        }
    }
}

/// Adapter observability signal derived from an invocation record.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AdapterObservabilityRecord {
    /// Adapter ID.
    pub adapter_id: AdapterId,
    /// Adapter kind.
    pub adapter_kind: AdapterKind,
    /// Operation mode.
    pub operation_mode: AdapterOperationMode,
    /// Action name.
    pub action: String,
    /// Success or failure status.
    pub status: AdapterResponseStatus,
    /// Correlation ID.
    pub correlation_id: CorrelationId,
    /// Duration in milliseconds.
    pub duration_ms: u64,
    /// Classified error for failures.
    pub error_kind: Option<AdapterErrorKind>,
    /// Non-secret attributes.
    pub attributes: BTreeMap<String, String>,
}

impl AdapterObservabilityRecord {
    /// Builds an adapter observability record from an audit invocation record.
    #[must_use]
    pub fn from_invocation(record: &AdapterInvocationRecord) -> Self {
        let mut attributes = BTreeMap::new();
        attributes.insert("capability".to_owned(), format!("{:?}", record.capability));
        attributes.insert("actor".to_owned(), record.actor.to_string());
        if let Some(run_scope) = &record.run_scope {
            attributes.insert("workflow_id".to_owned(), run_scope.workflow_id.to_string());
            attributes.insert(
                "workflow_version".to_owned(),
                run_scope.workflow_version.to_string(),
            );
            attributes.insert(
                "schema_version".to_owned(),
                run_scope.schema_version.to_string(),
            );
        }

        Self {
            adapter_id: record.adapter_id.clone(),
            adapter_kind: record.adapter_kind.clone(),
            operation_mode: record.operation_mode,
            action: record.action.clone(),
            status: record.status,
            correlation_id: record.correlation_id.clone(),
            duration_ms: record.duration_ms,
            error_kind: record.error_kind,
            attributes,
        }
    }
}

/// Read-only adapter operation.
pub trait AdapterReadOperation {
    /// Performs a read-only adapter operation.
    ///
    /// # Errors
    ///
    /// Returns an adapter error when the operation cannot be completed.
    fn read(&self, request: &AdapterRequest) -> Result<AdapterResponse, AdapterError>;
}

/// Write-capable adapter operation.
pub trait AdapterWriteOperation {
    /// Performs a write-capable adapter operation after preconditions pass.
    ///
    /// # Errors
    ///
    /// Returns an adapter error when preconditions fail or the operation cannot
    /// be completed.
    fn write(&self, request: &AdapterRequest) -> Result<AdapterResponse, AdapterError>;
}

/// Event-source adapter operation.
pub trait AdapterEventSource {
    /// Polls or reads non-secret adapter events.
    ///
    /// # Errors
    ///
    /// Returns an adapter error when events cannot be read.
    fn read_events(&self, request: &AdapterRequest) -> Result<Vec<AdapterEvent>, AdapterError>;
}

/// Adapter health check operation.
pub trait AdapterHealthCheck {
    /// Checks adapter health without performing side effects.
    ///
    /// # Errors
    ///
    /// Returns an adapter error when health cannot be determined.
    fn health(&self) -> Result<AdapterHealth, AdapterError>;
}

/// Adapter capability discovery operation.
pub trait AdapterCapabilityDiscovery {
    /// Returns capabilities supported by the adapter.
    fn capabilities(&self) -> Vec<AdapterCapability>;
}

/// Adapter dry-run or plan operation.
pub trait AdapterDryRun {
    /// Plans an adapter action without performing side effects.
    ///
    /// # Errors
    ///
    /// Returns an adapter error when the plan cannot be produced.
    fn dry_run(&self, request: &AdapterRequest) -> Result<AdapterResponse, AdapterError>;
}

fn redact_text(value: String) -> (String, RedactionMetadata) {
    let lower = value.to_ascii_lowercase();
    if lower.contains("secret")
        || lower.contains("token")
        || lower.contains("password")
        || lower.contains("credential")
        || lower.contains("api_key")
    {
        (
            "[REDACTED]".to_owned(),
            RedactionMetadata {
                redacted_fields: vec!["summary".to_owned()],
                field_states: vec![RedactionFieldState {
                    field: "summary".to_owned(),
                    disposition: RedactionDisposition::Redacted,
                    reason: "sensitive-looking adapter summary was redacted".to_owned(),
                }],
            },
        )
    } else {
        (value, RedactionMetadata::empty())
    }
}
