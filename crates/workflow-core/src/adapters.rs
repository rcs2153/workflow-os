use serde::{Deserialize, Serialize};

use crate::{
    AdapterId, CorrelationId, IdempotencyKey, RedactionDisposition, RedactionFieldState,
    RedactionMetadata, Timestamp, WorkflowOsError, WorkflowOsErrorKind,
};

/// Symbolic adapter kind for future integration implementations.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AdapterKind {
    /// Future GitHub adapter kind.
    GitHub,
    /// Future Jira adapter kind.
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

/// Result of policy pre-check before adapter invocation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AdapterPolicyPrecheck {
    /// Policy allowed the action.
    Allowed {
        /// Stable policy reason codes.
        reason_codes: Vec<String>,
    },
    /// A human approval decision allowed the action.
    ApprovalGranted {
        /// Approval request identifier.
        approval_id: String,
        /// Stable policy reason codes.
        reason_codes: Vec<String>,
    },
    /// No successful policy or approval pre-check exists.
    Missing,
    /// Policy explicitly denied the action.
    Denied {
        /// Stable denial reason codes.
        reason_codes: Vec<String>,
    },
}

impl AdapterPolicyPrecheck {
    fn allows_side_effect(&self) -> bool {
        matches!(self, Self::Allowed { .. } | Self::ApprovalGranted { .. })
    }
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
    /// Correlation ID propagated from runtime.
    pub correlation_id: CorrelationId,
    /// Idempotency key for side-effecting actions.
    pub idempotency_key: Option<IdempotencyKey>,
    /// Non-secret input reference.
    pub input_reference: Option<String>,
    /// Declared idempotency strategy.
    pub idempotency_strategy: AdapterIdempotencyStrategy,
    /// Declared redaction strategy.
    pub redaction_strategy: AdapterRedactionStrategy,
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
                .any(|capability| capability == &AdapterCapability::ExternalWrite)
    }

    /// Validates safety preconditions before an adapter operation may run.
    ///
    /// # Errors
    ///
    /// Returns an error when an external write lacks capability, idempotency, or
    /// policy/approval permission.
    pub fn validate_preconditions(&self) -> Result<(), AdapterError> {
        if self
            .action
            .required_capabilities
            .iter()
            .any(|capability| matches!(capability, AdapterCapability::Unknown(_)))
        {
            return Err(AdapterError::new(
                AdapterErrorKind::PermissionFailure,
                "adapter.capability.unknown",
                "unknown adapter capability is denied",
            ));
        }

        if self.is_side_effecting() {
            if !self
                .action
                .required_capabilities
                .contains(&AdapterCapability::ExternalWrite)
            {
                return Err(AdapterError::new(
                    AdapterErrorKind::PermissionFailure,
                    "adapter.capability.external_write_missing",
                    "external writes require declared external_write capability",
                ));
            }
            if self.idempotency_key.is_none() {
                return Err(AdapterError::new(
                    AdapterErrorKind::ValidationFailure,
                    "adapter.idempotency.required",
                    "side-effecting adapter actions require an idempotency key",
                ));
            }
            if !self.policy_precheck.allows_side_effect() {
                return Err(AdapterError::new(
                    AdapterErrorKind::PermissionFailure,
                    "adapter.policy.required",
                    "side-effecting adapter actions require policy allow or approval",
                ));
            }
        }
        Ok(())
    }
}

/// Adapter response safe for audit by default.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AdapterResponse {
    /// Non-sensitive response summary.
    pub summary: String,
    /// External object reference.
    pub external_reference: Option<String>,
    /// Redaction metadata describing hidden fields.
    pub redaction: RedactionMetadata,
}

impl AdapterResponse {
    /// Creates a response while redacting sensitive-looking summary text.
    #[must_use]
    pub fn redacted_summary(
        summary: impl Into<String>,
        external_reference: Option<String>,
    ) -> Self {
        let summary = summary.into();
        let (summary, redaction) = redact_text(summary);
        Self {
            summary,
            external_reference,
            redaction,
        }
    }
}

/// Adapter failure classification.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterErrorKind {
    /// Rate limit.
    RateLimit,
    /// Authentication failed.
    AuthFailure,
    /// Permission denied.
    PermissionFailure,
    /// Target was not found.
    NotFound,
    /// Request failed validation.
    ValidationFailure,
    /// Transient failure.
    TransientFailure,
    /// Unknown failure.
    UnknownFailure,
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
    /// Whether the adapter boundary is healthy.
    pub healthy: bool,
    /// Non-secret status message.
    pub message: String,
}

/// Record of an adapter invocation suitable for audit projections.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AdapterInvocationRecord {
    /// Adapter ID.
    pub adapter_id: AdapterId,
    /// Adapter kind.
    pub adapter_kind: AdapterKind,
    /// Action name.
    pub action: String,
    /// Correlation ID.
    pub correlation_id: CorrelationId,
    /// Idempotency key where side effects are possible.
    pub idempotency_key: Option<IdempotencyKey>,
    /// Non-secret input reference.
    pub input_reference: Option<String>,
    /// Non-secret output reference.
    pub output_reference: Option<String>,
    /// Redaction metadata.
    pub redaction: RedactionMetadata,
    /// Timestamp.
    pub invoked_at: Timestamp,
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
