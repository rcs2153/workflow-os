use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    ActorId, CapabilityReference, CorrelationId, EscalationRecord, EventId, EventSequenceNumber,
    FailureClass, FailureRecord, IdempotencyKey, ImmutableRunBundleId, ImmutableRunBundleVersion,
    SchemaVersion, SideEffectId, SkillAttemptId, SkillId, SkillInvocation, SkillInvocationAttempt,
    SkillInvocationId, SkillVersion, SpecContentHash, StepId, Timestamp, WorkflowId,
    WorkflowOsError, WorkflowRun, WorkflowRunEvent, WorkflowRunEventKind, WorkflowRunId,
    WorkflowRunStatus, WorkflowVersion,
};

const IDENTIFIER_MAX_BYTES: usize = 128;
const REFERENCE_MAX_BYTES: usize = 256;
const REFERENCE_MAX_COUNT: usize = 256;
const MAX_TIMEOUT_SECONDS: u64 = 86_400;
const MAX_OUTPUT_BYTES: u64 = 64 * 1024 * 1024;

macro_rules! hosted_id {
    ($name:ident, $label:literal, $code:literal) => {
        #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        #[doc = concat!("Validated ", $label, ".")]
        pub struct $name(String);

        impl $name {
            #[doc = concat!("Creates a validated ", $label, ".")]
            ///
            /// # Errors
            ///
            /// Returns a stable validation error when the value is malformed or secret-like.
            pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
                let value = value.into();
                validate_identifier(&value, IDENTIFIER_MAX_BYTES, $code, $label)?;
                Ok(Self(value))
            }

            #[doc = concat!("Returns the ", $label, " text.")]
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter
                    .debug_tuple(stringify!($name))
                    .field(&"[REDACTED]")
                    .finish()
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = WorkflowOsError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
    };
}

hosted_id!(
    HostedCatalogEntryId,
    "hosted catalog entry id",
    "hosted.catalog_entry_id.invalid"
);
hosted_id!(
    HostedWorkItemId,
    "hosted work item id",
    "hosted.work_item_id.invalid"
);
hosted_id!(
    HostedExecutionProviderId,
    "hosted execution provider id",
    "hosted.execution_provider_id.invalid"
);
hosted_id!(
    HostedExecutionProviderVersion,
    "hosted execution provider version",
    "hosted.execution_provider_version.invalid"
);
hosted_id!(
    HostedExecutionId,
    "hosted execution id",
    "hosted.execution_id.invalid"
);
hosted_id!(
    HostedExecutionPolicyId,
    "hosted execution policy id",
    "hosted.execution_policy_id.invalid"
);

/// Deterministic fingerprint of one canonical hosted execution request.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct HostedExecutionRequestFingerprint(SpecContentHash);

impl HostedExecutionRequestFingerprint {
    /// Returns the lowercase SHA-256 fingerprint.
    #[must_use]
    pub const fn as_hash(&self) -> &SpecContentHash {
        &self.0
    }
}

impl fmt::Debug for HostedExecutionRequestFingerprint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("HostedExecutionRequestFingerprint")
            .field(&"[REDACTED]")
            .finish()
    }
}

/// Kind of a stable payload-free hosted execution reference.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostedExecutionReferenceKind {
    /// Input artifact made available to the provider.
    Input,
    /// Output artifact produced by the provider.
    Artifact,
    /// Provider or sandbox log collection.
    Log,
    /// Denied action collection.
    DeniedAction,
    /// Provider telemetry collection.
    Telemetry,
    /// Access material resolved outside Core.
    AccessMaterial,
    /// Provider-side `SideEffect` reconciliation record.
    SideEffectReconciliation,
}

/// Stable payload-free reference passed across the hosted execution boundary.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "HostedExecutionReferenceWire")]
pub struct HostedExecutionReference {
    kind: HostedExecutionReferenceKind,
    value: String,
}

#[derive(Deserialize)]
struct HostedExecutionReferenceWire {
    kind: HostedExecutionReferenceKind,
    value: String,
}

impl HostedExecutionReference {
    /// Creates a validated stable reference.
    ///
    /// # Errors
    ///
    /// Rejects empty, oversized, malformed, path-like, URL-like, or secret-like values.
    pub fn new(
        kind: HostedExecutionReferenceKind,
        value: impl Into<String>,
    ) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        validate_reference(&value)?;
        Ok(Self { kind, value })
    }

    /// Returns the reference kind.
    #[must_use]
    pub const fn kind(&self) -> HostedExecutionReferenceKind {
        self.kind
    }

    /// Returns the stable reference text.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl TryFrom<HostedExecutionReferenceWire> for HostedExecutionReference {
    type Error = WorkflowOsError;

    fn try_from(value: HostedExecutionReferenceWire) -> Result<Self, Self::Error> {
        Self::new(value.kind, value.value)
    }
}

impl fmt::Debug for HostedExecutionReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HostedExecutionReference")
            .field("kind", &self.kind)
            .field("value", &"[REDACTED]")
            .finish()
    }
}

/// Bounded timeout and output allowance for one provider invocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "HostedExecutionBudgetWire")]
pub struct HostedExecutionBudget {
    timeout_seconds: u64,
    max_output_bytes: u64,
}

#[derive(Clone, Copy, Deserialize)]
struct HostedExecutionBudgetWire {
    timeout_seconds: u64,
    max_output_bytes: u64,
}

impl HostedExecutionBudget {
    /// Creates a bounded execution budget.
    ///
    /// # Errors
    ///
    /// Rejects zero or excessive timeout and output limits.
    pub fn new(timeout_seconds: u64, max_output_bytes: u64) -> Result<Self, WorkflowOsError> {
        if timeout_seconds == 0 || timeout_seconds > MAX_TIMEOUT_SECONDS {
            return Err(WorkflowOsError::validation(
                "hosted.execution_budget.timeout.invalid",
                "hosted execution timeout is invalid",
            ));
        }
        if max_output_bytes == 0 || max_output_bytes > MAX_OUTPUT_BYTES {
            return Err(WorkflowOsError::validation(
                "hosted.execution_budget.output.invalid",
                "hosted execution output limit is invalid",
            ));
        }
        Ok(Self {
            timeout_seconds,
            max_output_bytes,
        })
    }

    /// Returns the timeout in seconds.
    #[must_use]
    pub const fn timeout_seconds(self) -> u64 {
        self.timeout_seconds
    }

    /// Returns the maximum provider output size.
    #[must_use]
    pub const fn max_output_bytes(self) -> u64 {
        self.max_output_bytes
    }
}

impl TryFrom<HostedExecutionBudgetWire> for HostedExecutionBudget {
    type Error = WorkflowOsError;

    fn try_from(value: HostedExecutionBudgetWire) -> Result<Self, Self::Error> {
        Self::new(value.timeout_seconds, value.max_output_bytes)
    }
}

/// Exact immutable execution policy binding supplied to a provider.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct HostedExecutionPolicyBinding {
    policy_id: HostedExecutionPolicyId,
    policy_hash: SpecContentHash,
}

impl HostedExecutionPolicyBinding {
    /// Creates an exact policy binding.
    #[must_use]
    pub const fn new(policy_id: HostedExecutionPolicyId, policy_hash: SpecContentHash) -> Self {
        Self {
            policy_id,
            policy_hash,
        }
    }

    /// Returns the policy identity.
    #[must_use]
    pub const fn policy_id(&self) -> &HostedExecutionPolicyId {
        &self.policy_id
    }

    /// Returns the canonical policy hash.
    #[must_use]
    pub const fn policy_hash(&self) -> &SpecContentHash {
        &self.policy_hash
    }
}

impl fmt::Debug for HostedExecutionPolicyBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HostedExecutionPolicyBinding")
            .field("policy_id", &"[REDACTED]")
            .field("policy_hash", &"[REDACTED]")
            .finish()
    }
}

/// Payload-free request for one already-authorized hosted execution.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "HostedExecutionRequestWire")]
pub struct HostedExecutionRequest {
    run_id: WorkflowRunId,
    workflow_id: WorkflowId,
    workflow_version: WorkflowVersion,
    schema_version: SchemaVersion,
    step_id: StepId,
    bundle_id: ImmutableRunBundleId,
    bundle_version: ImmutableRunBundleVersion,
    bundle_root_hash: SpecContentHash,
    input_references: Vec<HostedExecutionReference>,
    authorized_capabilities: Vec<CapabilityReference>,
    approved_side_effects: Vec<SideEffectId>,
    policy: HostedExecutionPolicyBinding,
    budget: HostedExecutionBudget,
    correlation_id: CorrelationId,
    idempotency_key: IdempotencyKey,
    access_material_references: Vec<HostedExecutionReference>,
}

#[derive(Deserialize)]
struct HostedExecutionRequestWire {
    run_id: WorkflowRunId,
    workflow_id: WorkflowId,
    workflow_version: WorkflowVersion,
    schema_version: SchemaVersion,
    step_id: StepId,
    bundle_id: ImmutableRunBundleId,
    bundle_version: ImmutableRunBundleVersion,
    bundle_root_hash: SpecContentHash,
    input_references: Vec<HostedExecutionReference>,
    authorized_capabilities: Vec<CapabilityReference>,
    approved_side_effects: Vec<SideEffectId>,
    policy: HostedExecutionPolicyBinding,
    budget: HostedExecutionBudget,
    correlation_id: CorrelationId,
    idempotency_key: IdempotencyKey,
    access_material_references: Vec<HostedExecutionReference>,
}

impl HostedExecutionRequest {
    /// Creates and validates an exact hosted execution request.
    ///
    /// # Errors
    ///
    /// Rejects duplicate, excessive, or incorrectly classified references.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        run_id: WorkflowRunId,
        workflow_id: WorkflowId,
        workflow_version: WorkflowVersion,
        schema_version: SchemaVersion,
        step_id: StepId,
        bundle_id: ImmutableRunBundleId,
        bundle_version: ImmutableRunBundleVersion,
        bundle_root_hash: SpecContentHash,
        mut input_references: Vec<HostedExecutionReference>,
        mut authorized_capabilities: Vec<CapabilityReference>,
        mut approved_side_effects: Vec<SideEffectId>,
        policy: HostedExecutionPolicyBinding,
        budget: HostedExecutionBudget,
        correlation_id: CorrelationId,
        idempotency_key: IdempotencyKey,
        mut access_material_references: Vec<HostedExecutionReference>,
    ) -> Result<Self, WorkflowOsError> {
        validate_reference_kinds(
            &input_references,
            &[HostedExecutionReferenceKind::Input],
            "hosted.execution_request.input_references.invalid",
        )?;
        validate_reference_kinds(
            &access_material_references,
            &[HostedExecutionReferenceKind::AccessMaterial],
            "hosted.execution_request.access_material_references.invalid",
        )?;
        canonicalize_unique(
            &mut input_references,
            "hosted.execution_request.reference.duplicate",
        )?;
        canonicalize_unique(
            &mut access_material_references,
            "hosted.execution_request.reference.duplicate",
        )?;
        canonicalize_unique(
            &mut authorized_capabilities,
            "hosted.execution_request.capability.duplicate",
        )?;
        canonicalize_unique(
            &mut approved_side_effects,
            "hosted.execution_request.side_effect.duplicate",
        )?;
        Ok(Self {
            run_id,
            workflow_id,
            workflow_version,
            schema_version,
            step_id,
            bundle_id,
            bundle_version,
            bundle_root_hash,
            input_references,
            authorized_capabilities,
            approved_side_effects,
            policy,
            budget,
            correlation_id,
            idempotency_key,
            access_material_references,
        })
    }

    /// Returns the workflow run identity.
    #[must_use]
    pub const fn run_id(&self) -> &WorkflowRunId {
        &self.run_id
    }

    /// Returns the workflow identity.
    #[must_use]
    pub const fn workflow_id(&self) -> &WorkflowId {
        &self.workflow_id
    }

    /// Returns the workflow version.
    #[must_use]
    pub const fn workflow_version(&self) -> &WorkflowVersion {
        &self.workflow_version
    }

    /// Returns the schema version.
    #[must_use]
    pub const fn schema_version(&self) -> &SchemaVersion {
        &self.schema_version
    }

    /// Returns the exact step identity.
    #[must_use]
    pub const fn step_id(&self) -> &StepId {
        &self.step_id
    }

    /// Returns the immutable bundle identity.
    #[must_use]
    pub const fn bundle_id(&self) -> &ImmutableRunBundleId {
        &self.bundle_id
    }

    /// Returns the immutable bundle model version.
    #[must_use]
    pub const fn bundle_version(&self) -> &ImmutableRunBundleVersion {
        &self.bundle_version
    }

    /// Returns the immutable bundle integrity root.
    #[must_use]
    pub const fn bundle_root_hash(&self) -> &SpecContentHash {
        &self.bundle_root_hash
    }

    /// Returns input references.
    #[must_use]
    pub fn input_references(&self) -> &[HostedExecutionReference] {
        &self.input_references
    }

    /// Returns authorized capabilities.
    #[must_use]
    pub fn authorized_capabilities(&self) -> &[CapabilityReference] {
        &self.authorized_capabilities
    }

    /// Returns approved `SideEffect` identities.
    #[must_use]
    pub fn approved_side_effects(&self) -> &[SideEffectId] {
        &self.approved_side_effects
    }

    /// Returns the exact execution policy binding.
    #[must_use]
    pub const fn policy(&self) -> &HostedExecutionPolicyBinding {
        &self.policy
    }

    /// Returns the bounded execution budget.
    #[must_use]
    pub const fn budget(&self) -> HostedExecutionBudget {
        self.budget
    }

    /// Returns the correlation identity.
    #[must_use]
    pub const fn correlation_id(&self) -> &CorrelationId {
        &self.correlation_id
    }

    /// Returns the idempotency identity.
    #[must_use]
    pub const fn idempotency_key(&self) -> &IdempotencyKey {
        &self.idempotency_key
    }

    /// Returns opaque access-material references.
    #[must_use]
    pub fn access_material_references(&self) -> &[HostedExecutionReference] {
        &self.access_material_references
    }

    /// Returns a deterministic fingerprint over the exact canonical request.
    #[must_use]
    pub fn fingerprint(&self) -> HostedExecutionRequestFingerprint {
        HostedExecutionRequestFingerprint(SpecContentHash::from_bytes(
            canonical_execution_request_bytes(self),
        ))
    }
}

impl TryFrom<HostedExecutionRequestWire> for HostedExecutionRequest {
    type Error = WorkflowOsError;

    fn try_from(value: HostedExecutionRequestWire) -> Result<Self, Self::Error> {
        Self::new(
            value.run_id,
            value.workflow_id,
            value.workflow_version,
            value.schema_version,
            value.step_id,
            value.bundle_id,
            value.bundle_version,
            value.bundle_root_hash,
            value.input_references,
            value.authorized_capabilities,
            value.approved_side_effects,
            value.policy,
            value.budget,
            value.correlation_id,
            value.idempotency_key,
            value.access_material_references,
        )
    }
}

impl fmt::Debug for HostedExecutionRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HostedExecutionRequest")
            .field("identity", &"[REDACTED]")
            .field("input_reference_count", &self.input_references.len())
            .field("capability_count", &self.authorized_capabilities.len())
            .field(
                "approved_side_effect_count",
                &self.approved_side_effects.len(),
            )
            .field(
                "access_material_reference_count",
                &self.access_material_references.len(),
            )
            .field("budget", &self.budget)
            .finish_non_exhaustive()
    }
}

/// Terminal posture reported by a hosted execution provider.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostedExecutionStatus {
    /// Provider completed the requested work successfully.
    Completed,
    /// Provider completed with a bounded failure.
    Failed,
    /// Provider confirmed cancellation.
    Canceled,
    /// Provider outcome cannot be established safely.
    Ambiguous,
}

/// Stable provider-owned failure classification.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostedExecutionErrorCategory {
    /// Provider could not provision the execution environment.
    Provisioning,
    /// Execution policy could not be applied exactly.
    Policy,
    /// Provider execution timed out.
    Timeout,
    /// Requested operation was denied by the runtime.
    Denied,
    /// Provider transport became unavailable.
    Transport,
    /// Provider returned an invalid or unsupported response.
    Protocol,
    /// Outcome is ambiguous and requires reconciliation.
    Ambiguous,
}

/// Whether a failed provider invocation can prove that execution did not start.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostedExecutionAttemptPosture {
    /// The provider proves that no execution was started.
    NotStarted,
    /// Execution may have started and requires reconciliation before retry.
    MayHaveStarted,
}

/// Durable lifecycle posture for one exact hosted provider invocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostedExecutionAttemptStatus {
    /// The invocation identity and provider binding are durable, but no call has started.
    Prepared,
    /// The provider call may have started.
    Invoking,
    /// Another provider call is blocked until the prior invocation is reconciled.
    ReconciliationRequired,
    /// An exactly bound terminal receipt has been committed.
    Terminal,
}

impl HostedExecutionAttemptStatus {
    /// Returns the stable `PostgreSQL` discovery key for this status.
    #[must_use]
    pub const fn storage_key(self) -> &'static str {
        match self {
            Self::Prepared => "prepared",
            Self::Invoking => "invoking",
            Self::ReconciliationRequired => "reconciliation_required",
            Self::Terminal => "terminal",
        }
    }
}

/// Durable, payload-free posture for one exact hosted provider invocation.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "HostedExecutionAttemptWire")]
pub struct HostedExecutionAttempt {
    execution_id: HostedExecutionId,
    work_item_id: HostedWorkItemId,
    request_fingerprint: HostedExecutionRequestFingerprint,
    provider_id: HostedExecutionProviderId,
    provider_version: HostedExecutionProviderVersion,
    provider_configuration_hash: SpecContentHash,
    status: HostedExecutionAttemptStatus,
    prepared_at: Timestamp,
    updated_at: Timestamp,
    terminal_status: Option<HostedExecutionStatus>,
}

#[derive(Deserialize)]
struct HostedExecutionAttemptWire {
    execution_id: HostedExecutionId,
    work_item_id: HostedWorkItemId,
    request_fingerprint: HostedExecutionRequestFingerprint,
    provider_id: HostedExecutionProviderId,
    provider_version: HostedExecutionProviderVersion,
    provider_configuration_hash: SpecContentHash,
    status: HostedExecutionAttemptStatus,
    prepared_at: Timestamp,
    updated_at: Timestamp,
    terminal_status: Option<HostedExecutionStatus>,
}

impl HostedExecutionAttempt {
    /// Creates one durable attempt before provider invocation.
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn prepared(
        execution_id: HostedExecutionId,
        work_item_id: HostedWorkItemId,
        request_fingerprint: HostedExecutionRequestFingerprint,
        provider_id: HostedExecutionProviderId,
        provider_version: HostedExecutionProviderVersion,
        provider_configuration_hash: SpecContentHash,
        prepared_at: Timestamp,
    ) -> Self {
        Self {
            execution_id,
            work_item_id,
            request_fingerprint,
            provider_id,
            provider_version,
            provider_configuration_hash,
            status: HostedExecutionAttemptStatus::Prepared,
            prepared_at,
            updated_at: prepared_at,
            terminal_status: None,
        }
    }

    /// Marks that the exact provider invocation may have started.
    ///
    /// # Errors
    ///
    /// Rejects repeated, terminal, or time-regressing transitions.
    pub fn mark_invoking(&self, updated_at: Timestamp) -> Result<Self, WorkflowOsError> {
        self.transition(HostedExecutionAttemptStatus::Invoking, updated_at, None)
    }

    /// Blocks another invocation until the exact attempt is reconciled.
    ///
    /// # Errors
    ///
    /// Rejects attempts that were not invoking and time-regressing transitions.
    pub fn require_reconciliation(&self, updated_at: Timestamp) -> Result<Self, WorkflowOsError> {
        self.transition(
            HostedExecutionAttemptStatus::ReconciliationRequired,
            updated_at,
            None,
        )
    }

    /// Commits the exactly bound terminal receipt posture.
    ///
    /// # Errors
    ///
    /// Rejects non-invoking attempts, receipt substitution, and time regression.
    pub fn mark_terminal(&self, receipt: &HostedExecutionReceipt) -> Result<Self, WorkflowOsError> {
        receipt.validate_for_attempt(self)?;
        self.transition(
            HostedExecutionAttemptStatus::Terminal,
            receipt.terminal_at(),
            Some(receipt.status()),
        )
    }

    fn transition(
        &self,
        target: HostedExecutionAttemptStatus,
        updated_at: Timestamp,
        terminal_status: Option<HostedExecutionStatus>,
    ) -> Result<Self, WorkflowOsError> {
        if updated_at < self.updated_at {
            return Err(WorkflowOsError::invalid_state(
                "hosted.execution_attempt.timestamp.regressed",
                "hosted execution attempt timestamp cannot regress",
            ));
        }
        let allowed = matches!(
            (self.status, target),
            (
                HostedExecutionAttemptStatus::Prepared,
                HostedExecutionAttemptStatus::Invoking
            ) | (
                HostedExecutionAttemptStatus::Invoking,
                HostedExecutionAttemptStatus::ReconciliationRequired
                    | HostedExecutionAttemptStatus::Terminal
            ) | (
                HostedExecutionAttemptStatus::ReconciliationRequired,
                HostedExecutionAttemptStatus::Terminal
            )
        );
        if !allowed
            || (target == HostedExecutionAttemptStatus::Terminal) != terminal_status.is_some()
        {
            return Err(WorkflowOsError::invalid_state(
                "hosted.execution_attempt.transition.invalid",
                "hosted execution attempt transition is invalid",
            ));
        }
        let mut next = self.clone();
        next.status = target;
        next.updated_at = updated_at;
        next.terminal_status = terminal_status;
        Ok(next)
    }

    /// Returns the durable provider invocation identity.
    #[must_use]
    pub const fn execution_id(&self) -> &HostedExecutionId {
        &self.execution_id
    }

    /// Returns the owning work-item identity.
    #[must_use]
    pub const fn work_item_id(&self) -> &HostedWorkItemId {
        &self.work_item_id
    }

    /// Returns the exact request fingerprint.
    #[must_use]
    pub const fn request_fingerprint(&self) -> &HostedExecutionRequestFingerprint {
        &self.request_fingerprint
    }

    /// Returns the exact provider identity.
    #[must_use]
    pub const fn provider_id(&self) -> &HostedExecutionProviderId {
        &self.provider_id
    }

    /// Returns the exact provider implementation version.
    #[must_use]
    pub const fn provider_version(&self) -> &HostedExecutionProviderVersion {
        &self.provider_version
    }

    /// Returns the exact provider configuration hash.
    #[must_use]
    pub const fn provider_configuration_hash(&self) -> &SpecContentHash {
        &self.provider_configuration_hash
    }

    /// Returns the durable attempt status.
    #[must_use]
    pub const fn status(&self) -> HostedExecutionAttemptStatus {
        self.status
    }

    /// Returns when the invocation identity became durable.
    #[must_use]
    pub const fn prepared_at(&self) -> Timestamp {
        self.prepared_at
    }

    /// Returns the last durable transition time.
    #[must_use]
    pub const fn updated_at(&self) -> Timestamp {
        self.updated_at
    }

    /// Returns the terminal provider status when committed.
    #[must_use]
    pub const fn terminal_status(&self) -> Option<HostedExecutionStatus> {
        self.terminal_status
    }
}

impl TryFrom<HostedExecutionAttemptWire> for HostedExecutionAttempt {
    type Error = WorkflowOsError;

    fn try_from(value: HostedExecutionAttemptWire) -> Result<Self, Self::Error> {
        if value.updated_at < value.prepared_at
            || (value.status == HostedExecutionAttemptStatus::Terminal)
                != value.terminal_status.is_some()
        {
            return Err(WorkflowOsError::validation(
                "hosted.execution_attempt.serialized_state.invalid",
                "serialized hosted execution attempt state is invalid",
            ));
        }
        Ok(Self {
            execution_id: value.execution_id,
            work_item_id: value.work_item_id,
            request_fingerprint: value.request_fingerprint,
            provider_id: value.provider_id,
            provider_version: value.provider_version,
            provider_configuration_hash: value.provider_configuration_hash,
            status: value.status,
            prepared_at: value.prepared_at,
            updated_at: value.updated_at,
            terminal_status: value.terminal_status,
        })
    }
}

impl fmt::Debug for HostedExecutionAttempt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HostedExecutionAttempt")
            .field("identity", &"[REDACTED]")
            .field("status", &self.status)
            .field("prepared_at", &self.prepared_at)
            .field("updated_at", &self.updated_at)
            .field("terminal_status", &self.terminal_status)
            .finish_non_exhaustive()
    }
}

/// Structured, non-leaking provider invocation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HostedExecutionInvocationError {
    category: HostedExecutionErrorCategory,
    attempt_posture: HostedExecutionAttemptPosture,
}

impl HostedExecutionInvocationError {
    /// Creates a bounded provider invocation error.
    #[must_use]
    pub const fn new(
        category: HostedExecutionErrorCategory,
        attempt_posture: HostedExecutionAttemptPosture,
    ) -> Self {
        Self {
            category,
            attempt_posture,
        }
    }

    /// Returns the stable failure category.
    #[must_use]
    pub const fn category(self) -> HostedExecutionErrorCategory {
        self.category
    }

    /// Returns whether execution may have started.
    #[must_use]
    pub const fn attempt_posture(self) -> HostedExecutionAttemptPosture {
        self.attempt_posture
    }

    /// Maps the provider failure to a stable Core error.
    #[must_use]
    pub fn into_workflow_error(self) -> WorkflowOsError {
        let code = match self.attempt_posture {
            HostedExecutionAttemptPosture::NotStarted => {
                "hosted.execution_provider.invocation.not_started"
            }
            HostedExecutionAttemptPosture::MayHaveStarted => {
                "hosted.execution_provider.invocation.ambiguous"
            }
        };
        WorkflowOsError::invalid_state(code, "hosted execution provider invocation failed")
    }
}

/// Payload-free terminal receipt returned by a hosted execution provider.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "HostedExecutionReceiptWire")]
pub struct HostedExecutionReceipt {
    execution_id: HostedExecutionId,
    provider_id: HostedExecutionProviderId,
    provider_version: HostedExecutionProviderVersion,
    provider_configuration_hash: SpecContentHash,
    request_fingerprint: HostedExecutionRequestFingerprint,
    environment_reference: HostedExecutionReference,
    policy_hash: SpecContentHash,
    started_at: Timestamp,
    terminal_at: Timestamp,
    status: HostedExecutionStatus,
    error_category: Option<HostedExecutionErrorCategory>,
    exit_status: Option<i32>,
    references: Vec<HostedExecutionReference>,
}

#[derive(Deserialize)]
struct HostedExecutionReceiptWire {
    execution_id: HostedExecutionId,
    provider_id: HostedExecutionProviderId,
    provider_version: HostedExecutionProviderVersion,
    provider_configuration_hash: SpecContentHash,
    request_fingerprint: HostedExecutionRequestFingerprint,
    environment_reference: HostedExecutionReference,
    policy_hash: SpecContentHash,
    started_at: Timestamp,
    terminal_at: Timestamp,
    status: HostedExecutionStatus,
    error_category: Option<HostedExecutionErrorCategory>,
    exit_status: Option<i32>,
    references: Vec<HostedExecutionReference>,
}

impl HostedExecutionReceipt {
    /// Creates and validates a terminal provider receipt.
    ///
    /// # Errors
    ///
    /// Rejects invalid time, terminal, environment, and reference posture.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        execution_id: HostedExecutionId,
        provider_id: HostedExecutionProviderId,
        provider_version: HostedExecutionProviderVersion,
        provider_configuration_hash: SpecContentHash,
        request_fingerprint: HostedExecutionRequestFingerprint,
        environment_reference: HostedExecutionReference,
        policy_hash: SpecContentHash,
        started_at: Timestamp,
        terminal_at: Timestamp,
        status: HostedExecutionStatus,
        error_category: Option<HostedExecutionErrorCategory>,
        exit_status: Option<i32>,
        mut references: Vec<HostedExecutionReference>,
    ) -> Result<Self, WorkflowOsError> {
        if environment_reference.kind != HostedExecutionReferenceKind::Telemetry {
            return Err(WorkflowOsError::validation(
                "hosted.execution_receipt.environment_reference.invalid",
                "hosted execution environment reference is invalid",
            ));
        }
        if terminal_at < started_at {
            return Err(WorkflowOsError::validation(
                "hosted.execution_receipt.time.invalid",
                "hosted execution receipt time range is invalid",
            ));
        }
        if status == HostedExecutionStatus::Completed && error_category.is_some() {
            return Err(WorkflowOsError::validation(
                "hosted.execution_receipt.error_posture.invalid",
                "completed hosted execution cannot include an error category",
            ));
        }
        if status != HostedExecutionStatus::Completed && error_category.is_none() {
            return Err(WorkflowOsError::validation(
                "hosted.execution_receipt.error_posture.invalid",
                "non-completed hosted execution requires an error category",
            ));
        }
        if status == HostedExecutionStatus::Completed && exit_status != Some(0) {
            return Err(WorkflowOsError::validation(
                "hosted.execution_receipt.exit_status.invalid",
                "completed hosted execution requires a zero exit status",
            ));
        }
        validate_reference_kinds(
            &references,
            &[
                HostedExecutionReferenceKind::Artifact,
                HostedExecutionReferenceKind::Log,
                HostedExecutionReferenceKind::DeniedAction,
                HostedExecutionReferenceKind::Telemetry,
                HostedExecutionReferenceKind::SideEffectReconciliation,
            ],
            "hosted.execution_receipt.reference.invalid",
        )?;
        canonicalize_unique(
            &mut references,
            "hosted.execution_receipt.reference.duplicate",
        )?;
        Ok(Self {
            execution_id,
            provider_id,
            provider_version,
            provider_configuration_hash,
            request_fingerprint,
            environment_reference,
            policy_hash,
            started_at,
            terminal_at,
            status,
            error_category,
            exit_status,
            references,
        })
    }

    /// Returns the provider execution identity.
    #[must_use]
    pub const fn execution_id(&self) -> &HostedExecutionId {
        &self.execution_id
    }

    /// Returns the provider identity.
    #[must_use]
    pub const fn provider_id(&self) -> &HostedExecutionProviderId {
        &self.provider_id
    }

    /// Returns the provider implementation version.
    #[must_use]
    pub const fn provider_version(&self) -> &HostedExecutionProviderVersion {
        &self.provider_version
    }

    /// Returns the provider configuration hash.
    #[must_use]
    pub const fn provider_configuration_hash(&self) -> &SpecContentHash {
        &self.provider_configuration_hash
    }

    /// Returns the exact request fingerprint acknowledged by the provider.
    #[must_use]
    pub const fn request_fingerprint(&self) -> &HostedExecutionRequestFingerprint {
        &self.request_fingerprint
    }

    /// Returns the execution-environment reference.
    #[must_use]
    pub const fn environment_reference(&self) -> &HostedExecutionReference {
        &self.environment_reference
    }

    /// Returns the applied execution policy hash.
    #[must_use]
    pub const fn policy_hash(&self) -> &SpecContentHash {
        &self.policy_hash
    }

    /// Returns the provider start timestamp.
    #[must_use]
    pub const fn started_at(&self) -> Timestamp {
        self.started_at
    }

    /// Returns the provider terminal timestamp.
    #[must_use]
    pub const fn terminal_at(&self) -> Timestamp {
        self.terminal_at
    }

    /// Returns the terminal execution status.
    #[must_use]
    pub const fn status(&self) -> HostedExecutionStatus {
        self.status
    }

    /// Returns the optional bounded error category.
    #[must_use]
    pub const fn error_category(&self) -> Option<HostedExecutionErrorCategory> {
        self.error_category
    }

    /// Returns the optional provider exit status.
    #[must_use]
    pub const fn exit_status(&self) -> Option<i32> {
        self.exit_status
    }

    /// Returns stable output references.
    #[must_use]
    pub fn references(&self) -> &[HostedExecutionReference] {
        &self.references
    }

    /// Validates this receipt against the exact request and provider boundary.
    ///
    /// # Errors
    ///
    /// Rejects mismatched request, policy, provider identity, version, or
    /// provider configuration.
    pub fn validate_for(
        &self,
        request: &HostedExecutionRequest,
        provider: &dyn HostedExecutionProvider,
    ) -> Result<(), WorkflowOsError> {
        if self.provider_id != *provider.provider_id()
            || self.provider_version != *provider.provider_version()
            || self.provider_configuration_hash != *provider.configuration_hash()
            || self.request_fingerprint != request.fingerprint()
            || self.policy_hash != *request.policy().policy_hash()
        {
            return Err(WorkflowOsError::invalid_state(
                "hosted.execution_receipt.binding.invalid",
                "hosted execution receipt binding is invalid",
            ));
        }
        Ok(())
    }

    /// Validates the request and policy portion of this receipt binding.
    ///
    /// Provider identity remains validated separately against the durable
    /// invocation attempt before terminal commit.
    ///
    /// # Errors
    ///
    /// Rejects a substituted request or policy binding.
    pub fn validate_for_request(
        &self,
        request: &HostedExecutionRequest,
    ) -> Result<(), WorkflowOsError> {
        if self.request_fingerprint != request.fingerprint()
            || self.policy_hash != *request.policy().policy_hash()
        {
            return Err(WorkflowOsError::invalid_state(
                "hosted.execution_receipt.request_binding.invalid",
                "hosted execution receipt request binding is invalid",
            ));
        }
        Ok(())
    }

    /// Validates this receipt against one durable invocation attempt.
    ///
    /// # Errors
    ///
    /// Rejects substituted invocation, request, provider, or configuration
    /// identity without including those values in the error.
    pub fn validate_for_attempt(
        &self,
        attempt: &HostedExecutionAttempt,
    ) -> Result<(), WorkflowOsError> {
        if self.execution_id != *attempt.execution_id()
            || self.provider_id != *attempt.provider_id()
            || self.provider_version != *attempt.provider_version()
            || self.provider_configuration_hash != *attempt.provider_configuration_hash()
            || self.request_fingerprint != *attempt.request_fingerprint()
        {
            return Err(WorkflowOsError::invalid_state(
                "hosted.execution_receipt.attempt_binding.invalid",
                "hosted execution receipt attempt binding is invalid",
            ));
        }
        Ok(())
    }
}

impl TryFrom<HostedExecutionReceiptWire> for HostedExecutionReceipt {
    type Error = WorkflowOsError;

    fn try_from(value: HostedExecutionReceiptWire) -> Result<Self, Self::Error> {
        Self::new(
            value.execution_id,
            value.provider_id,
            value.provider_version,
            value.provider_configuration_hash,
            value.request_fingerprint,
            value.environment_reference,
            value.policy_hash,
            value.started_at,
            value.terminal_at,
            value.status,
            value.error_category,
            value.exit_status,
            value.references,
        )
    }
}

impl fmt::Debug for HostedExecutionReceipt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HostedExecutionReceipt")
            .field("identity", &"[REDACTED]")
            .field("status", &self.status)
            .field("error_category", &self.error_category)
            .field("exit_status", &self.exit_status)
            .field("reference_count", &self.references.len())
            .finish_non_exhaustive()
    }
}

/// Injected hosted execution boundary.
pub trait HostedExecutionProvider: Send + Sync {
    /// Returns the stable provider identity.
    fn provider_id(&self) -> &HostedExecutionProviderId;

    /// Returns the stable provider implementation version.
    fn provider_version(&self) -> &HostedExecutionProviderVersion;

    /// Returns the exact non-secret provider configuration hash.
    fn configuration_hash(&self) -> &SpecContentHash;

    /// Executes one already-authorized payload-free request.
    ///
    /// # Errors
    ///
    /// Returns a structured non-leaking error when no valid receipt can be produced.
    fn execute(
        &self,
        request: &HostedExecutionRequest,
    ) -> Result<HostedExecutionReceipt, HostedExecutionInvocationError>;
}

/// Invokes a provider and validates its receipt against the exact request.
///
/// # Errors
///
/// Returns the provider's bounded failure posture, or an ambiguous protocol
/// failure when the provider returns a receipt that does not bind to the
/// request and configured provider.
pub fn invoke_hosted_execution_provider(
    provider: &dyn HostedExecutionProvider,
    request: &HostedExecutionRequest,
) -> Result<HostedExecutionReceipt, HostedExecutionInvocationError> {
    let receipt = provider.execute(request)?;
    receipt.validate_for(request, provider).map_err(|_| {
        HostedExecutionInvocationError::new(
            HostedExecutionErrorCategory::Protocol,
            HostedExecutionAttemptPosture::MayHaveStarted,
        )
    })?;
    Ok(receipt)
}

/// Durable hosted worker work-item status.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostedWorkItemStatus {
    /// Work is eligible for a fenced worker claim.
    Queued,
    /// One fenced worker is processing the work.
    Running,
    /// Core paused the run for an approval decision.
    WaitingForApproval,
    /// Governed work completed.
    Completed,
    /// Governed work failed.
    Failed,
    /// Governed work was canceled.
    Canceled,
    /// Outcome is ambiguous and requires operator reconciliation.
    Ambiguous,
}

/// Core-owned atomic dispatch projection for one scheduled hosted invocation.
#[derive(Clone, Eq, PartialEq)]
pub struct HostedSkillDispatch {
    work_item: HostedWorkItem,
    invocation_requested: WorkflowRunEvent,
    invocation_started: WorkflowRunEvent,
}

impl HostedSkillDispatch {
    /// Builds the exact workflow events paired with a queued hosted work item.
    ///
    /// The current alpha accepts one already-scheduled invocation only. It does
    /// not infer a step, skill, or work-item identity from provider input.
    ///
    /// # Errors
    ///
    /// Fails closed when the run, immutable bundle, scheduled step, invocation,
    /// attempt, or work-item binding is inconsistent.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        run: &WorkflowRun,
        work_item: HostedWorkItem,
        invocation_id: SkillInvocationId,
        skill_id: SkillId,
        skill_version: SkillVersion,
        attempt_id: SkillAttemptId,
        requested_at: Timestamp,
    ) -> Result<Self, WorkflowOsError> {
        validate_dispatch_run_binding(run, &work_item)?;
        let step_id = work_item.execution_request().step_id().clone();
        let scheduled_step = run.events.iter().rev().find_map(|event| match &event.kind {
            WorkflowRunEventKind::StepScheduled { step_id } => Some(step_id),
            _ => None,
        });
        if scheduled_step != Some(&step_id)
            || run.events.iter().any(|event| {
                matches!(
                    &event.kind,
                    WorkflowRunEventKind::SkillInvocationRequested(invocation)
                        if invocation.step_id == step_id
                )
            })
        {
            return Err(hosted_projection_error(
                "hosted.dispatch.scheduled_invocation.invalid",
                "hosted dispatch requires one unconsumed authoritative scheduled step",
            ));
        }

        let requested_key = work_item.idempotency_key().clone();
        let attempt_key = hosted_attempt_idempotency_key(&requested_key)?;
        let invocation_requested = hosted_event(
            run,
            work_item.requested_by().clone(),
            work_item.correlation_id().clone(),
            requested_at,
            run.snapshot.last_sequence_number.next(),
            "dispatch-requested",
            work_item.work_item_id(),
            Some(requested_key.clone()),
            WorkflowRunEventKind::SkillInvocationRequested(SkillInvocation {
                invocation_id: invocation_id.clone(),
                step_id: step_id.clone(),
                skill_id: skill_id.clone(),
                skill_version: skill_version.clone(),
                idempotency_key: Some(requested_key),
                attempts: Vec::new(),
            }),
        )?;
        let invocation_started = hosted_event(
            run,
            work_item.requested_by().clone(),
            work_item.correlation_id().clone(),
            requested_at,
            invocation_requested.sequence_number.next(),
            "dispatch-started",
            work_item.work_item_id(),
            Some(attempt_key),
            WorkflowRunEventKind::SkillInvocationStarted(SkillInvocationAttempt {
                invocation_id,
                attempt_id,
                step_id,
                skill_id,
                skill_version,
                attempt_number: 1,
            }),
        )?;

        let mut events = run.events.clone();
        events.push(invocation_requested.clone());
        events.push(invocation_started.clone());
        let projected = WorkflowRun::rehydrate(&events)?;
        if projected.snapshot.status != WorkflowRunStatus::Running {
            return Err(hosted_projection_error(
                "hosted.dispatch.projection.invalid",
                "hosted dispatch did not preserve a running workflow projection",
            ));
        }

        Ok(Self {
            work_item,
            invocation_requested,
            invocation_started,
        })
    }

    /// Returns the queued hosted work item.
    #[must_use]
    pub const fn work_item(&self) -> &HostedWorkItem {
        &self.work_item
    }

    /// Returns the authoritative invocation-request event.
    #[must_use]
    pub const fn invocation_requested(&self) -> &WorkflowRunEvent {
        &self.invocation_requested
    }

    /// Returns the authoritative invocation-start event.
    #[must_use]
    pub const fn invocation_started(&self) -> &WorkflowRunEvent {
        &self.invocation_started
    }
}

impl fmt::Debug for HostedSkillDispatch {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HostedSkillDispatch")
            .field("work_item", &"[REDACTED]")
            .field("event_count", &2)
            .finish()
    }
}

/// Core-owned terminal projection paired with an exactly bound provider receipt.
#[derive(Clone, Eq, PartialEq)]
pub struct HostedTerminalResultProjection {
    receipt: HostedExecutionReceipt,
    events: Vec<WorkflowRunEvent>,
    projected_run: WorkflowRun,
}

impl HostedTerminalResultProjection {
    /// Creates terminal workflow events from one exactly bound hosted receipt.
    ///
    /// The alpha projection is intentionally single-step. Completion may not
    /// silently advance another workflow step.
    ///
    /// # Errors
    ///
    /// Fails closed on binding, invocation, status, or projection mismatch.
    pub fn new(
        run: &WorkflowRun,
        work_item: &HostedWorkItem,
        receipt: HostedExecutionReceipt,
        actor: ActorId,
    ) -> Result<Self, WorkflowOsError> {
        validate_dispatch_run_binding(run, work_item)?;
        receipt
            .validate_for_request(work_item.execution_request())
            .map_err(|_| {
                hosted_projection_error(
                    "hosted.result.receipt_binding.invalid",
                    "hosted terminal receipt binding is invalid",
                )
            })?;
        if receipt.status() == HostedExecutionStatus::Ambiguous {
            return Self::from_ambiguous_receipt(run, work_item, receipt, actor);
        }
        let (invocation_id, step_id, skill_id, skill_version) =
            pending_hosted_invocation(run, work_item)?;
        let idempotency_key = hosted_result_idempotency_key(
            work_item.idempotency_key(),
            receipt.execution_id(),
            receipt.status(),
        )?;
        let first_kind = match receipt.status() {
            HostedExecutionStatus::Completed => WorkflowRunEventKind::SkillInvocationSucceeded {
                invocation_id,
                step_id,
                skill_id,
                skill_version,
                output_ref: Some(format!(
                    "hosted-receipt/{}",
                    receipt.execution_id().as_str()
                )),
            },
            HostedExecutionStatus::Failed | HostedExecutionStatus::Canceled => {
                WorkflowRunEventKind::SkillInvocationFailed {
                    invocation_id,
                    step_id,
                    skill_id,
                    skill_version,
                    failure: hosted_terminal_failure(receipt.status()),
                }
            }
            HostedExecutionStatus::Ambiguous => unreachable!("handled above"),
        };
        let first = hosted_event(
            run,
            actor.clone(),
            work_item.correlation_id().clone(),
            receipt.terminal_at(),
            run.snapshot.last_sequence_number.next(),
            "result-invocation",
            work_item.work_item_id(),
            Some(idempotency_key),
            first_kind,
        )?;
        let terminal_kind = match receipt.status() {
            HostedExecutionStatus::Completed => WorkflowRunEventKind::RunCompleted,
            HostedExecutionStatus::Failed => {
                WorkflowRunEventKind::RunFailed(hosted_terminal_failure(receipt.status()))
            }
            HostedExecutionStatus::Canceled => {
                WorkflowRunEventKind::RunCanceled(crate::CancellationRecord {
                    run_id: run.snapshot.identity.run_id.clone(),
                    reason: "hosted execution was canceled".to_owned(),
                    actor: actor.clone(),
                    canceled_at: receipt.terminal_at(),
                    correlation_id: work_item.correlation_id().clone(),
                })
            }
            HostedExecutionStatus::Ambiguous => unreachable!("handled above"),
        };
        let terminal = hosted_event(
            run,
            actor,
            work_item.correlation_id().clone(),
            receipt.terminal_at(),
            first.sequence_number.next(),
            "result-terminal",
            work_item.work_item_id(),
            None,
            terminal_kind,
        )?;
        let events = vec![first, terminal];
        let mut complete_history = run.events.clone();
        complete_history.extend(events.clone());
        let projected_run = WorkflowRun::rehydrate(&complete_history)?;
        if !projected_run.snapshot.status.is_terminal() {
            return Err(hosted_projection_error(
                "hosted.result.projection.invalid",
                "hosted result did not produce a terminal workflow projection",
            ));
        }
        Ok(Self {
            receipt,
            events,
            projected_run,
        })
    }

    fn from_ambiguous_receipt(
        run: &WorkflowRun,
        work_item: &HostedWorkItem,
        receipt: HostedExecutionReceipt,
        actor: ActorId,
    ) -> Result<Self, WorkflowOsError> {
        let (_, step_id, skill_id, skill_version) = pending_hosted_invocation(run, work_item)?;
        let idempotency_key = hosted_result_idempotency_key(
            work_item.idempotency_key(),
            receipt.execution_id(),
            receipt.status(),
        )?;
        let escalation = hosted_reconciliation_event(
            run,
            work_item,
            actor,
            receipt.terminal_at(),
            step_id,
            skill_id,
            skill_version,
            Some(idempotency_key),
            "receipt-reconciliation",
        )?;
        let events = vec![escalation];
        let mut complete_history = run.events.clone();
        complete_history.extend(events.clone());
        let projected_run = WorkflowRun::rehydrate(&complete_history)?;
        if projected_run.snapshot.status != WorkflowRunStatus::Escalated {
            return Err(hosted_projection_error(
                "hosted.result.projection.invalid",
                "ambiguous hosted result did not produce an escalated workflow projection",
            ));
        }
        Ok(Self {
            receipt,
            events,
            projected_run,
        })
    }

    /// Returns the exactly bound provider receipt.
    #[must_use]
    pub const fn receipt(&self) -> &HostedExecutionReceipt {
        &self.receipt
    }

    /// Returns the terminal workflow events in append order.
    #[must_use]
    pub fn events(&self) -> &[WorkflowRunEvent] {
        &self.events
    }

    /// Returns the resulting authoritative workflow run.
    #[must_use]
    pub const fn projected_run(&self) -> &WorkflowRun {
        &self.projected_run
    }
}

impl fmt::Debug for HostedTerminalResultProjection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HostedTerminalResultProjection")
            .field("receipt", &"[REDACTED]")
            .field("event_count", &self.events.len())
            .field("terminal_status", &self.projected_run.snapshot.status)
            .finish()
    }
}

/// Provider outcome without a receipt that can be projected authoritatively.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostedUnreceiptedOutcome {
    /// Core rejected the request before any provider action started.
    RejectedBeforeStart,
    /// The provider may have started and operator reconciliation is required.
    ReconciliationRequired,
}

/// Core-owned projection for a provider outcome that has no valid receipt.
#[derive(Clone, Eq, PartialEq)]
pub struct HostedUnreceiptedResultProjection {
    outcome: HostedUnreceiptedOutcome,
    events: Vec<WorkflowRunEvent>,
    projected_run: WorkflowRun,
}

impl HostedUnreceiptedResultProjection {
    /// Creates authoritative failure or escalation events without fabricating
    /// a provider receipt.
    ///
    /// # Errors
    ///
    /// Fails closed on run, work-item, invocation, or projection mismatch.
    pub fn new(
        run: &WorkflowRun,
        work_item: &HostedWorkItem,
        outcome: HostedUnreceiptedOutcome,
        actor: ActorId,
        occurred_at: Timestamp,
    ) -> Result<Self, WorkflowOsError> {
        validate_dispatch_run_binding(run, work_item)?;
        let (invocation_id, step_id, skill_id, skill_version) =
            pending_hosted_invocation(run, work_item)?;
        let idempotency_key = hosted_derived_idempotency_key(
            "unreceipted-result",
            work_item.idempotency_key().as_str(),
            Some(match outcome {
                HostedUnreceiptedOutcome::RejectedBeforeStart => "rejected-before-start",
                HostedUnreceiptedOutcome::ReconciliationRequired => "reconciliation-required",
            }),
        )?;
        let events = match outcome {
            HostedUnreceiptedOutcome::RejectedBeforeStart => {
                let failure = hosted_rejected_before_start_failure();
                let invocation_failed = hosted_event(
                    run,
                    actor.clone(),
                    work_item.correlation_id().clone(),
                    occurred_at,
                    run.snapshot.last_sequence_number.next(),
                    "rejected-invocation",
                    work_item.work_item_id(),
                    Some(idempotency_key),
                    WorkflowRunEventKind::SkillInvocationFailed {
                        invocation_id,
                        step_id,
                        skill_id,
                        skill_version,
                        failure: failure.clone(),
                    },
                )?;
                let run_failed = hosted_event(
                    run,
                    actor,
                    work_item.correlation_id().clone(),
                    occurred_at,
                    invocation_failed.sequence_number.next(),
                    "rejected-terminal",
                    work_item.work_item_id(),
                    None,
                    WorkflowRunEventKind::RunFailed(failure),
                )?;
                vec![invocation_failed, run_failed]
            }
            HostedUnreceiptedOutcome::ReconciliationRequired => {
                vec![hosted_reconciliation_event(
                    run,
                    work_item,
                    actor,
                    occurred_at,
                    step_id,
                    skill_id,
                    skill_version,
                    Some(idempotency_key),
                    "invocation-reconciliation",
                )?]
            }
        };
        let mut complete_history = run.events.clone();
        complete_history.extend(events.clone());
        let projected_run = WorkflowRun::rehydrate(&complete_history)?;
        let expected_status = match outcome {
            HostedUnreceiptedOutcome::RejectedBeforeStart => WorkflowRunStatus::Failed,
            HostedUnreceiptedOutcome::ReconciliationRequired => WorkflowRunStatus::Escalated,
        };
        if projected_run.snapshot.status != expected_status {
            return Err(hosted_projection_error(
                "hosted.unreceipted_result.projection.invalid",
                "unreceipted hosted result produced an invalid workflow projection",
            ));
        }
        Ok(Self {
            outcome,
            events,
            projected_run,
        })
    }

    /// Returns the provider outcome represented by this projection.
    #[must_use]
    pub const fn outcome(&self) -> HostedUnreceiptedOutcome {
        self.outcome
    }

    /// Returns the authoritative workflow events in append order.
    #[must_use]
    pub fn events(&self) -> &[WorkflowRunEvent] {
        &self.events
    }

    /// Returns the resulting authoritative workflow run.
    #[must_use]
    pub const fn projected_run(&self) -> &WorkflowRun {
        &self.projected_run
    }
}

impl fmt::Debug for HostedUnreceiptedResultProjection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HostedUnreceiptedResultProjection")
            .field("outcome", &self.outcome)
            .field("event_count", &self.events.len())
            .field("projected_status", &self.projected_run.snapshot.status)
            .finish()
    }
}

impl HostedWorkItemStatus {
    /// Returns the stable `PostgreSQL` discovery key for this status.
    #[must_use]
    pub const fn storage_key(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Running => "running",
            Self::WaitingForApproval => "waiting_for_approval",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Canceled => "canceled",
            Self::Ambiguous => "ambiguous",
        }
    }
}

/// Durable, payload-free hosted worker item.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "HostedWorkItemWire")]
pub struct HostedWorkItem {
    work_item_id: HostedWorkItemId,
    catalog_entry_id: HostedCatalogEntryId,
    run_id: WorkflowRunId,
    workflow_id: WorkflowId,
    bundle_id: ImmutableRunBundleId,
    bundle_version: ImmutableRunBundleVersion,
    bundle_root_hash: SpecContentHash,
    requested_by: ActorId,
    correlation_id: CorrelationId,
    idempotency_key: IdempotencyKey,
    execution_request: HostedExecutionRequest,
    status: HostedWorkItemStatus,
    queued_at: Timestamp,
    updated_at: Timestamp,
    attempt_count: u32,
}

#[derive(Deserialize)]
struct HostedWorkItemWire {
    work_item_id: HostedWorkItemId,
    catalog_entry_id: HostedCatalogEntryId,
    run_id: WorkflowRunId,
    workflow_id: WorkflowId,
    bundle_id: ImmutableRunBundleId,
    bundle_version: ImmutableRunBundleVersion,
    bundle_root_hash: SpecContentHash,
    requested_by: ActorId,
    correlation_id: CorrelationId,
    idempotency_key: IdempotencyKey,
    execution_request: HostedExecutionRequest,
    status: HostedWorkItemStatus,
    queued_at: Timestamp,
    updated_at: Timestamp,
    attempt_count: u32,
}

impl HostedWorkItem {
    /// Creates a queued hosted work item.
    #[allow(clippy::too_many_arguments)]
    ///
    /// # Errors
    ///
    /// Rejects an execution request that does not bind to the exact work item.
    pub fn queued(
        work_item_id: HostedWorkItemId,
        catalog_entry_id: HostedCatalogEntryId,
        run_id: WorkflowRunId,
        workflow_id: WorkflowId,
        bundle_id: ImmutableRunBundleId,
        bundle_version: ImmutableRunBundleVersion,
        bundle_root_hash: SpecContentHash,
        requested_by: ActorId,
        correlation_id: CorrelationId,
        idempotency_key: IdempotencyKey,
        execution_request: HostedExecutionRequest,
        queued_at: Timestamp,
    ) -> Result<Self, WorkflowOsError> {
        validate_work_item_execution_request(
            &run_id,
            &workflow_id,
            &bundle_id,
            &bundle_version,
            &bundle_root_hash,
            &correlation_id,
            &idempotency_key,
            &execution_request,
        )?;
        Ok(Self {
            work_item_id,
            catalog_entry_id,
            run_id,
            workflow_id,
            bundle_id,
            bundle_version,
            bundle_root_hash,
            requested_by,
            correlation_id,
            idempotency_key,
            execution_request,
            status: HostedWorkItemStatus::Queued,
            queued_at,
            updated_at: queued_at,
            attempt_count: 0,
        })
    }

    /// Applies one deterministic work-item transition.
    ///
    /// # Errors
    ///
    /// Rejects illegal transitions and regressing timestamps.
    pub fn transition(
        &self,
        target: HostedWorkItemStatus,
        updated_at: Timestamp,
    ) -> Result<Self, WorkflowOsError> {
        if updated_at < self.updated_at {
            return Err(WorkflowOsError::invalid_state(
                "hosted.work_item.timestamp.regressed",
                "hosted work item timestamp cannot regress",
            ));
        }
        if !allowed_work_item_transition(self.status, target) {
            return Err(WorkflowOsError::invalid_state(
                "hosted.work_item.transition.invalid",
                "hosted work item transition is invalid",
            ));
        }
        let mut next = self.clone();
        next.status = target;
        next.updated_at = updated_at;
        if self.status == HostedWorkItemStatus::Queued && target == HostedWorkItemStatus::Running {
            next.attempt_count = self.attempt_count.checked_add(1).ok_or_else(|| {
                WorkflowOsError::invalid_state(
                    "hosted.work_item.attempt.exhausted",
                    "hosted work item attempt count is exhausted",
                )
            })?;
        }
        Ok(next)
    }

    /// Reclaims a running item after its prior worker lease expired.
    ///
    /// # Errors
    ///
    /// Rejects non-running items, regressing timestamps, and exhausted
    /// attempt counters.
    pub fn reclaim(&self, updated_at: Timestamp) -> Result<Self, WorkflowOsError> {
        if self.status != HostedWorkItemStatus::Running {
            return Err(WorkflowOsError::invalid_state(
                "hosted.work_item.reclaim_status.invalid",
                "only a running hosted work item can be reclaimed",
            ));
        }
        if updated_at < self.updated_at {
            return Err(WorkflowOsError::invalid_state(
                "hosted.work_item.timestamp.regressed",
                "hosted work item timestamp cannot regress",
            ));
        }
        let mut next = self.clone();
        next.updated_at = updated_at;
        next.attempt_count = self.attempt_count.checked_add(1).ok_or_else(|| {
            WorkflowOsError::invalid_state(
                "hosted.work_item.attempt.exhausted",
                "hosted work item attempt count is exhausted",
            )
        })?;
        Ok(next)
    }

    /// Returns the work-item identity.
    #[must_use]
    pub const fn work_item_id(&self) -> &HostedWorkItemId {
        &self.work_item_id
    }

    /// Returns the deployment catalog identity.
    #[must_use]
    pub const fn catalog_entry_id(&self) -> &HostedCatalogEntryId {
        &self.catalog_entry_id
    }

    /// Returns the run identity.
    #[must_use]
    pub const fn run_id(&self) -> &WorkflowRunId {
        &self.run_id
    }

    /// Returns the workflow identity.
    #[must_use]
    pub const fn workflow_id(&self) -> &WorkflowId {
        &self.workflow_id
    }

    /// Returns the immutable bundle identity.
    #[must_use]
    pub const fn bundle_id(&self) -> &ImmutableRunBundleId {
        &self.bundle_id
    }

    /// Returns the immutable bundle version.
    #[must_use]
    pub const fn bundle_version(&self) -> &ImmutableRunBundleVersion {
        &self.bundle_version
    }

    /// Returns the immutable bundle integrity root.
    #[must_use]
    pub const fn bundle_root_hash(&self) -> &SpecContentHash {
        &self.bundle_root_hash
    }

    /// Returns the requesting actor.
    #[must_use]
    pub const fn requested_by(&self) -> &ActorId {
        &self.requested_by
    }

    /// Returns the correlation identity.
    #[must_use]
    pub const fn correlation_id(&self) -> &CorrelationId {
        &self.correlation_id
    }

    /// Returns the idempotency identity.
    #[must_use]
    pub const fn idempotency_key(&self) -> &IdempotencyKey {
        &self.idempotency_key
    }

    /// Returns the exact provider request bound to this work item.
    #[must_use]
    pub const fn execution_request(&self) -> &HostedExecutionRequest {
        &self.execution_request
    }

    /// Returns the current work-item status.
    #[must_use]
    pub const fn status(&self) -> HostedWorkItemStatus {
        self.status
    }

    /// Returns the queue timestamp.
    #[must_use]
    pub const fn queued_at(&self) -> Timestamp {
        self.queued_at
    }

    /// Returns the last transition timestamp.
    #[must_use]
    pub const fn updated_at(&self) -> Timestamp {
        self.updated_at
    }

    /// Returns the number of worker attempts.
    #[must_use]
    pub const fn attempt_count(&self) -> u32 {
        self.attempt_count
    }
}

impl TryFrom<HostedWorkItemWire> for HostedWorkItem {
    type Error = WorkflowOsError;

    fn try_from(value: HostedWorkItemWire) -> Result<Self, Self::Error> {
        if value.updated_at < value.queued_at
            || (value.status == HostedWorkItemStatus::Queued && value.attempt_count != 0)
            || (matches!(
                value.status,
                HostedWorkItemStatus::Running
                    | HostedWorkItemStatus::WaitingForApproval
                    | HostedWorkItemStatus::Completed
                    | HostedWorkItemStatus::Failed
                    | HostedWorkItemStatus::Ambiguous
            ) && value.attempt_count == 0)
        {
            return Err(WorkflowOsError::validation(
                "hosted.work_item.serialized_state.invalid",
                "serialized hosted work item state is invalid",
            ));
        }
        validate_work_item_execution_request(
            &value.run_id,
            &value.workflow_id,
            &value.bundle_id,
            &value.bundle_version,
            &value.bundle_root_hash,
            &value.correlation_id,
            &value.idempotency_key,
            &value.execution_request,
        )?;
        Ok(Self {
            work_item_id: value.work_item_id,
            catalog_entry_id: value.catalog_entry_id,
            run_id: value.run_id,
            workflow_id: value.workflow_id,
            bundle_id: value.bundle_id,
            bundle_version: value.bundle_version,
            bundle_root_hash: value.bundle_root_hash,
            requested_by: value.requested_by,
            correlation_id: value.correlation_id,
            idempotency_key: value.idempotency_key,
            execution_request: value.execution_request,
            status: value.status,
            queued_at: value.queued_at,
            updated_at: value.updated_at,
            attempt_count: value.attempt_count,
        })
    }
}

impl fmt::Debug for HostedWorkItem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HostedWorkItem")
            .field("identity", &"[REDACTED]")
            .field("status", &self.status)
            .field("queued_at", &self.queued_at)
            .field("updated_at", &self.updated_at)
            .field("attempt_count", &self.attempt_count)
            .finish_non_exhaustive()
    }
}

#[allow(clippy::too_many_arguments)]
fn validate_work_item_execution_request(
    run_id: &WorkflowRunId,
    workflow_id: &WorkflowId,
    bundle_id: &ImmutableRunBundleId,
    bundle_version: &ImmutableRunBundleVersion,
    bundle_root_hash: &SpecContentHash,
    correlation_id: &CorrelationId,
    idempotency_key: &IdempotencyKey,
    execution_request: &HostedExecutionRequest,
) -> Result<(), WorkflowOsError> {
    if execution_request.run_id() != run_id
        || execution_request.workflow_id() != workflow_id
        || execution_request.bundle_id() != bundle_id
        || execution_request.bundle_version() != bundle_version
        || execution_request.bundle_root_hash() != bundle_root_hash
        || execution_request.correlation_id() != correlation_id
        || execution_request.idempotency_key() != idempotency_key
    {
        return Err(WorkflowOsError::validation(
            "hosted.work_item.execution_request.invalid",
            "hosted work item execution request binding is invalid",
        ));
    }
    Ok(())
}

fn allowed_work_item_transition(
    current: HostedWorkItemStatus,
    target: HostedWorkItemStatus,
) -> bool {
    matches!(
        (current, target),
        (
            HostedWorkItemStatus::Queued,
            HostedWorkItemStatus::Running | HostedWorkItemStatus::Canceled
        ) | (
            HostedWorkItemStatus::Running,
            HostedWorkItemStatus::WaitingForApproval
                | HostedWorkItemStatus::Completed
                | HostedWorkItemStatus::Failed
                | HostedWorkItemStatus::Canceled
                | HostedWorkItemStatus::Ambiguous
        ) | (
            HostedWorkItemStatus::WaitingForApproval,
            HostedWorkItemStatus::Queued
                | HostedWorkItemStatus::Failed
                | HostedWorkItemStatus::Canceled
        )
    )
}

fn validate_dispatch_run_binding(
    run: &WorkflowRun,
    work_item: &HostedWorkItem,
) -> Result<(), WorkflowOsError> {
    let identity = &run.snapshot.identity;
    let bundle = identity.immutable_run_bundle.as_ref().ok_or_else(|| {
        hosted_projection_error(
            "hosted.dispatch.bundle_binding.missing",
            "hosted dispatch requires an immutable run bundle binding",
        )
    })?;
    if run.snapshot.status != WorkflowRunStatus::Running
        || identity.run_id != *work_item.run_id()
        || identity.workflow_id != *work_item.workflow_id()
        || identity.workflow_version != *work_item.execution_request().workflow_version()
        || identity.schema_version != *work_item.execution_request().schema_version()
        || bundle.bundle_id() != work_item.bundle_id()
        || bundle.bundle_version() != work_item.bundle_version()
        || bundle.root_hash() != work_item.bundle_root_hash()
    {
        return Err(hosted_projection_error(
            "hosted.dispatch.run_binding.invalid",
            "hosted dispatch does not match the authoritative running workflow",
        ));
    }
    Ok(())
}

fn pending_hosted_invocation(
    run: &WorkflowRun,
    work_item: &HostedWorkItem,
) -> Result<(SkillInvocationId, StepId, SkillId, SkillVersion), WorkflowOsError> {
    let step_id = work_item.execution_request().step_id();
    let requested = run.events.iter().rev().find_map(|event| match &event.kind {
        WorkflowRunEventKind::SkillInvocationRequested(invocation)
            if &invocation.step_id == step_id
                && invocation.idempotency_key.as_ref() == Some(work_item.idempotency_key()) =>
        {
            Some(invocation)
        }
        _ => None,
    });
    let Some(requested) = requested else {
        return Err(hosted_projection_error(
            "hosted.result.invocation.missing",
            "hosted result requires an exactly bound pending invocation",
        ));
    };
    let started = run.events.iter().any(|event| {
        matches!(
            &event.kind,
            WorkflowRunEventKind::SkillInvocationStarted(attempt)
                if attempt.invocation_id == requested.invocation_id
                    && attempt.step_id == requested.step_id
                    && attempt.skill_id == requested.skill_id
                    && attempt.skill_version == requested.skill_version
                    && attempt.attempt_number == 1
        )
    });
    let terminal = run.events.iter().any(|event| match &event.kind {
        WorkflowRunEventKind::SkillInvocationSucceeded { invocation_id, .. }
        | WorkflowRunEventKind::SkillInvocationFailed { invocation_id, .. } => {
            invocation_id == &requested.invocation_id
        }
        _ => false,
    });
    if !started || terminal {
        return Err(hosted_projection_error(
            "hosted.result.invocation.invalid",
            "hosted result invocation posture is invalid",
        ));
    }
    Ok((
        requested.invocation_id.clone(),
        requested.step_id.clone(),
        requested.skill_id.clone(),
        requested.skill_version.clone(),
    ))
}

#[allow(clippy::too_many_arguments)]
fn hosted_event(
    run: &WorkflowRun,
    actor: ActorId,
    correlation_id: CorrelationId,
    timestamp: Timestamp,
    sequence_number: EventSequenceNumber,
    event_label: &'static str,
    work_item_id: &HostedWorkItemId,
    idempotency_key: Option<IdempotencyKey>,
    kind: WorkflowRunEventKind,
) -> Result<WorkflowRunEvent, WorkflowOsError> {
    let mut hasher = Sha256::new();
    hash_hosted_projection_field(&mut hasher, "run", run.snapshot.identity.run_id.as_str());
    hash_hosted_projection_field(&mut hasher, "work_item", work_item_id.as_str());
    hash_hosted_projection_field(&mut hasher, "event", event_label);
    let event_id = EventId::new(format!("hosted-{}", hosted_hex_digest(hasher.finalize())))
        .map_err(|_| {
            hosted_projection_error(
                "hosted.projection.event_id.invalid",
                "hosted projection event identity is invalid",
            )
        })?;
    Ok(WorkflowRunEvent {
        sequence_number,
        event_id,
        timestamp,
        run_id: run.snapshot.identity.run_id.clone(),
        workflow_id: run.snapshot.identity.workflow_id.clone(),
        schema_version: run.snapshot.identity.schema_version.clone(),
        workflow_version: run.snapshot.identity.workflow_version.clone(),
        spec_content_hash: run.snapshot.identity.spec_content_hash.clone(),
        correlation_id: Some(correlation_id),
        actor: Some(actor),
        idempotency_key,
        kind,
    })
}

fn hosted_attempt_idempotency_key(
    invocation_key: &IdempotencyKey,
) -> Result<IdempotencyKey, WorkflowOsError> {
    hosted_derived_idempotency_key("attempt", invocation_key.as_str(), None)
}

fn hosted_result_idempotency_key(
    invocation_key: &IdempotencyKey,
    execution_id: &HostedExecutionId,
    status: HostedExecutionStatus,
) -> Result<IdempotencyKey, WorkflowOsError> {
    hosted_derived_idempotency_key(
        "result",
        invocation_key.as_str(),
        Some(&format!("{}:{status:?}", execution_id.as_str())),
    )
}

fn hosted_derived_idempotency_key(
    label: &'static str,
    value: &str,
    extra: Option<&str>,
) -> Result<IdempotencyKey, WorkflowOsError> {
    let mut hasher = Sha256::new();
    hash_hosted_projection_field(&mut hasher, "label", label);
    hash_hosted_projection_field(&mut hasher, "value", value);
    if let Some(extra) = extra {
        hash_hosted_projection_field(&mut hasher, "extra", extra);
    }
    IdempotencyKey::new(format!("hosted-{}", hosted_hex_digest(hasher.finalize()))).map_err(|_| {
        hosted_projection_error(
            "hosted.projection.idempotency.invalid",
            "hosted projection idempotency identity is invalid",
        )
    })
}

fn hash_hosted_projection_field(hasher: &mut Sha256, label: &str, value: &str) {
    hasher.update((label.len() as u64).to_be_bytes());
    hasher.update(label.as_bytes());
    hasher.update((value.len() as u64).to_be_bytes());
    hasher.update(value.as_bytes());
}

fn hosted_hex_digest(bytes: impl AsRef<[u8]>) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let bytes = bytes.as_ref();
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(HEX[(byte >> 4) as usize] as char);
        encoded.push(HEX[(byte & 0x0f) as usize] as char);
    }
    encoded
}

fn hosted_terminal_failure(status: HostedExecutionStatus) -> FailureRecord {
    let (code, message, failure_class) = match status {
        HostedExecutionStatus::Completed => (
            "hosted.execution.completed",
            "hosted execution completed",
            FailureClass::Unknown,
        ),
        HostedExecutionStatus::Failed => (
            "hosted.execution.failed",
            "hosted execution failed",
            FailureClass::Permanent,
        ),
        HostedExecutionStatus::Canceled => (
            "hosted.execution.canceled",
            "hosted execution was canceled",
            FailureClass::Canceled,
        ),
        HostedExecutionStatus::Ambiguous => (
            "hosted.execution.ambiguous",
            "hosted execution outcome requires reconciliation",
            FailureClass::Unknown,
        ),
    };
    FailureRecord {
        code: code.to_owned(),
        message: message.to_owned(),
        failure_class,
    }
}

fn hosted_rejected_before_start_failure() -> FailureRecord {
    FailureRecord {
        code: "hosted.execution.rejected_before_start".to_owned(),
        message: "hosted execution request was rejected before provider start".to_owned(),
        failure_class: FailureClass::Permanent,
    }
}

#[allow(clippy::too_many_arguments)]
fn hosted_reconciliation_event(
    run: &WorkflowRun,
    work_item: &HostedWorkItem,
    actor: ActorId,
    occurred_at: Timestamp,
    step_id: StepId,
    skill_id: SkillId,
    skill_version: SkillVersion,
    idempotency_key: Option<IdempotencyKey>,
    event_label: &'static str,
) -> Result<WorkflowRunEvent, WorkflowOsError> {
    let mut hasher = Sha256::new();
    hash_hosted_projection_field(&mut hasher, "run", run.snapshot.identity.run_id.as_str());
    hash_hosted_projection_field(&mut hasher, "work_item", work_item.work_item_id().as_str());
    hash_hosted_projection_field(&mut hasher, "kind", "reconciliation");
    let escalation_id = format!(
        "hosted-reconciliation-{}",
        hosted_hex_digest(hasher.finalize())
    );
    hosted_event(
        run,
        actor,
        work_item.correlation_id().clone(),
        occurred_at,
        run.snapshot.last_sequence_number.next(),
        event_label,
        work_item.work_item_id(),
        idempotency_key,
        WorkflowRunEventKind::EscalationTriggered(EscalationRecord {
            escalation_id,
            run_id: run.snapshot.identity.run_id.clone(),
            step_id: Some(step_id),
            skill_id: Some(skill_id),
            skill_version: Some(skill_version),
            attempts: work_item.attempt_count(),
            last_error: "hosted provider outcome is ambiguous".to_owned(),
            failure_class: FailureClass::Unknown,
            suggested_next_action: "inspect the provider outcome before retry".to_owned(),
            reason: "hosted provider reconciliation is required".to_owned(),
            contact: None,
        }),
    )
}

fn hosted_projection_error(code: &'static str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::invalid_state(code, message)
}

fn canonicalize_unique<T: Ord>(
    values: &mut [T],
    duplicate_code: &'static str,
) -> Result<(), WorkflowOsError> {
    if values.len() > REFERENCE_MAX_COUNT {
        return Err(WorkflowOsError::validation(
            "hosted.reference.count.invalid",
            "hosted reference count is invalid",
        ));
    }
    values.sort();
    if values.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(WorkflowOsError::validation(
            duplicate_code,
            "hosted request contains a duplicate reference",
        ));
    }
    Ok(())
}

fn validate_reference_kinds(
    references: &[HostedExecutionReference],
    allowed: &[HostedExecutionReferenceKind],
    code: &'static str,
) -> Result<(), WorkflowOsError> {
    let allowed = allowed.iter().copied().collect::<BTreeSet<_>>();
    if references
        .iter()
        .any(|reference| !allowed.contains(&reference.kind))
    {
        return Err(WorkflowOsError::validation(
            code,
            "hosted execution reference kind is invalid",
        ));
    }
    Ok(())
}

fn validate_reference(value: &str) -> Result<(), WorkflowOsError> {
    validate_identifier(
        value,
        REFERENCE_MAX_BYTES,
        "hosted.execution_reference.invalid",
        "hosted execution reference",
    )?;
    if value.starts_with('/')
        || value.contains("..")
        || value.contains("://")
        || value.contains('\\')
    {
        return Err(WorkflowOsError::validation(
            "hosted.execution_reference.invalid",
            "hosted execution reference is invalid",
        ));
    }
    Ok(())
}

fn validate_identifier(
    value: &str,
    max_bytes: usize,
    code: &'static str,
    message_label: &'static str,
) -> Result<(), WorkflowOsError> {
    if value.is_empty()
        || value.len() > max_bytes
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'/' | b'-' | b'_' | b'.'))
        || looks_secret_like(value)
    {
        return Err(WorkflowOsError::validation(
            code,
            format!("{message_label} is invalid"),
        ));
    }
    Ok(())
}

fn canonical_execution_request_bytes(request: &HostedExecutionRequest) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hash_component(&mut hasher, b"workflow-os.hosted-execution-request.v1");
    hash_component(&mut hasher, request.run_id.as_str().as_bytes());
    hash_component(&mut hasher, request.workflow_id.as_str().as_bytes());
    hash_component(&mut hasher, request.workflow_version.as_str().as_bytes());
    hash_component(&mut hasher, request.schema_version.as_str().as_bytes());
    hash_component(&mut hasher, request.step_id.as_str().as_bytes());
    hash_component(&mut hasher, request.bundle_id.as_str().as_bytes());
    hash_component(&mut hasher, request.bundle_version.as_str().as_bytes());
    hash_component(&mut hasher, request.bundle_root_hash.as_str().as_bytes());
    for reference in &request.input_references {
        hash_component(&mut hasher, &[reference_kind_tag(reference.kind)]);
        hash_component(&mut hasher, reference.value.as_bytes());
    }
    for capability in &request.authorized_capabilities {
        hash_component(&mut hasher, capability.as_str().as_bytes());
    }
    for side_effect in &request.approved_side_effects {
        hash_component(&mut hasher, side_effect.as_str().as_bytes());
    }
    hash_component(&mut hasher, request.policy.policy_id.as_str().as_bytes());
    hash_component(&mut hasher, request.policy.policy_hash.as_str().as_bytes());
    hash_component(&mut hasher, &request.budget.timeout_seconds.to_be_bytes());
    hash_component(&mut hasher, &request.budget.max_output_bytes.to_be_bytes());
    hash_component(&mut hasher, request.correlation_id.as_str().as_bytes());
    hash_component(&mut hasher, request.idempotency_key.as_str().as_bytes());
    for reference in &request.access_material_references {
        hash_component(&mut hasher, &[reference_kind_tag(reference.kind)]);
        hash_component(&mut hasher, reference.value.as_bytes());
    }
    hasher.finalize().to_vec()
}

fn hash_component(hasher: &mut Sha256, value: &[u8]) {
    hasher.update(value.len().to_be_bytes());
    hasher.update(value);
}

const fn reference_kind_tag(kind: HostedExecutionReferenceKind) -> u8 {
    match kind {
        HostedExecutionReferenceKind::Input => 0,
        HostedExecutionReferenceKind::Artifact => 1,
        HostedExecutionReferenceKind::Log => 2,
        HostedExecutionReferenceKind::DeniedAction => 3,
        HostedExecutionReferenceKind::Telemetry => 4,
        HostedExecutionReferenceKind::AccessMaterial => 5,
        HostedExecutionReferenceKind::SideEffectReconciliation => 6,
    }
}

fn looks_secret_like(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    [
        "authorization",
        "bearer",
        "password",
        "private_key",
        "private-key",
        "api_key",
        "api-key",
        "secret",
        "token",
        "credential",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::panic)]

    use super::*;

    fn timestamp(value: &str) -> Timestamp {
        Timestamp::parse_rfc3339(value).unwrap_or_else(|error| panic!("{error}"))
    }

    fn request() -> HostedExecutionRequest {
        HostedExecutionRequest::new(
            WorkflowRunId::new("run-hosted-1").unwrap_or_else(|error| panic!("{error}")),
            WorkflowId::new("hosted/example").unwrap_or_else(|error| panic!("{error}")),
            WorkflowVersion::new("v1").unwrap_or_else(|error| panic!("{error}")),
            SchemaVersion::new("workflowos.dev/v0").unwrap_or_else(|error| panic!("{error}")),
            StepId::new("verify").unwrap_or_else(|error| panic!("{error}")),
            ImmutableRunBundleId::new("bundle-hosted-1").unwrap_or_else(|error| panic!("{error}")),
            ImmutableRunBundleVersion::new("v1").unwrap_or_else(|error| panic!("{error}")),
            SpecContentHash::from_text("bundle"),
            vec![HostedExecutionReference::new(
                HostedExecutionReferenceKind::Input,
                "artifact/input-1",
            )
            .unwrap_or_else(|error| panic!("{error}"))],
            vec![CapabilityReference::new("repository.read")
                .unwrap_or_else(|error| panic!("{error}"))],
            Vec::new(),
            HostedExecutionPolicyBinding::new(
                HostedExecutionPolicyId::new("policy/no-write")
                    .unwrap_or_else(|error| panic!("{error}")),
                SpecContentHash::from_text("policy"),
            ),
            HostedExecutionBudget::new(300, 1024).unwrap_or_else(|error| panic!("{error}")),
            CorrelationId::new("correlation-hosted-1").unwrap_or_else(|error| panic!("{error}")),
            IdempotencyKey::new("hosted-request-1").unwrap_or_else(|error| panic!("{error}")),
            Vec::new(),
        )
        .unwrap_or_else(|error| panic!("{error}"))
    }

    fn dispatch_run() -> WorkflowRun {
        let request = request();
        let binding: crate::ImmutableRunBundleBinding = serde_json::from_value(serde_json::json!({
            "bundle_id": request.bundle_id().as_str(),
            "bundle_version": request.bundle_version().as_str(),
            "root_hash": request.bundle_root_hash().as_str(),
        }))
        .unwrap_or_else(|error| panic!("binding: {error}"));
        let created = WorkflowRunEvent {
            sequence_number: EventSequenceNumber::first(),
            event_id: EventId::new("event-hosted-created")
                .unwrap_or_else(|error| panic!("{error}")),
            timestamp: timestamp("2026-07-29T00:00:00Z"),
            run_id: request.run_id().clone(),
            workflow_id: request.workflow_id().clone(),
            schema_version: request.schema_version().clone(),
            workflow_version: request.workflow_version().clone(),
            spec_content_hash: SpecContentHash::from_text("workflow"),
            correlation_id: Some(request.correlation_id().clone()),
            actor: Some(
                ActorId::new("system/hosted-test").unwrap_or_else(|error| panic!("{error}")),
            ),
            idempotency_key: None,
            kind: WorkflowRunEventKind::RunCreated {
                summary: None,
                immutable_run_bundle: Some(binding),
            },
        };
        let event = |sequence: u64, suffix: &str, kind: WorkflowRunEventKind| WorkflowRunEvent {
            sequence_number: EventSequenceNumber::new(sequence)
                .unwrap_or_else(|error| panic!("{error}")),
            event_id: EventId::new(format!("event-hosted-{suffix}"))
                .unwrap_or_else(|error| panic!("{error}")),
            timestamp: timestamp("2026-07-29T00:00:00Z"),
            run_id: created.run_id.clone(),
            workflow_id: created.workflow_id.clone(),
            schema_version: created.schema_version.clone(),
            workflow_version: created.workflow_version.clone(),
            spec_content_hash: created.spec_content_hash.clone(),
            correlation_id: created.correlation_id.clone(),
            actor: created.actor.clone(),
            idempotency_key: None,
            kind,
        };
        WorkflowRun::rehydrate(&[
            created.clone(),
            event(2, "validated", WorkflowRunEventKind::RunValidated),
            event(3, "started", WorkflowRunEventKind::RunStarted),
            event(
                4,
                "scheduled",
                WorkflowRunEventKind::StepScheduled {
                    step_id: request.step_id().clone(),
                },
            ),
        ])
        .unwrap_or_else(|error| panic!("run: {error}"))
    }

    fn work_item() -> HostedWorkItem {
        let request = request();
        HostedWorkItem::queued(
            HostedWorkItemId::new("work-item-hosted-1").unwrap_or_else(|error| panic!("{error}")),
            HostedCatalogEntryId::new("catalog/example").unwrap_or_else(|error| panic!("{error}")),
            request.run_id().clone(),
            request.workflow_id().clone(),
            request.bundle_id().clone(),
            request.bundle_version().clone(),
            request.bundle_root_hash().clone(),
            ActorId::new("user/maintainer").unwrap_or_else(|error| panic!("{error}")),
            request.correlation_id().clone(),
            request.idempotency_key().clone(),
            request,
            timestamp("2026-07-29T00:00:00Z"),
        )
        .unwrap_or_else(|error| panic!("{error}"))
    }

    fn completed_receipt(request: &HostedExecutionRequest) -> HostedExecutionReceipt {
        HostedExecutionReceipt::new(
            HostedExecutionId::new("execution-projection-1")
                .unwrap_or_else(|error| panic!("{error}")),
            HostedExecutionProviderId::new("provider/no-write")
                .unwrap_or_else(|error| panic!("{error}")),
            HostedExecutionProviderVersion::new("v1").unwrap_or_else(|error| panic!("{error}")),
            SpecContentHash::from_text("provider-configuration"),
            request.fingerprint(),
            HostedExecutionReference::new(
                HostedExecutionReferenceKind::Telemetry,
                "environment/projection-1",
            )
            .unwrap_or_else(|error| panic!("{error}")),
            request.policy().policy_hash().clone(),
            timestamp("2026-07-29T00:00:01Z"),
            timestamp("2026-07-29T00:00:02Z"),
            HostedExecutionStatus::Completed,
            None,
            Some(0),
            Vec::new(),
        )
        .unwrap_or_else(|error| panic!("{error}"))
    }

    fn dispatched_run_and_item() -> (WorkflowRun, HostedWorkItem) {
        let run = dispatch_run();
        let item = work_item();
        let dispatch = HostedSkillDispatch::new(
            &run,
            item.clone(),
            SkillInvocationId::new("invocation-hosted-result")
                .unwrap_or_else(|error| panic!("{error}")),
            SkillId::new("hosted/no-write").unwrap_or_else(|error| panic!("{error}")),
            SkillVersion::new("v1").unwrap_or_else(|error| panic!("{error}")),
            SkillAttemptId::new("attempt-hosted-result").unwrap_or_else(|error| panic!("{error}")),
            timestamp("2026-07-29T00:00:01Z"),
        )
        .unwrap_or_else(|error| panic!("{error}"));
        let mut events = run.events.clone();
        events.push(dispatch.invocation_requested().clone());
        events.push(dispatch.invocation_started().clone());
        let running = item
            .transition(
                HostedWorkItemStatus::Running,
                timestamp("2026-07-29T00:00:01Z"),
            )
            .unwrap_or_else(|error| panic!("{error}"));
        (
            WorkflowRun::rehydrate(&events).unwrap_or_else(|error| panic!("{error}")),
            running,
        )
    }

    #[test]
    fn hosted_request_is_payload_free_and_round_trips() {
        let request = request();
        let json =
            serde_json::to_string(&request).unwrap_or_else(|error| panic!("serialize: {error}"));
        assert!(!json.contains("raw_payload"));
        assert!(!json.contains("command_output"));
        let decoded: HostedExecutionRequest =
            serde_json::from_str(&json).unwrap_or_else(|error| panic!("deserialize: {error}"));
        assert_eq!(decoded, request);
    }

    #[test]
    fn hosted_reference_rejects_paths_urls_and_secrets_without_leaking() {
        for value in [
            "/private/repo",
            "https://example.com",
            "artifact/../secret",
            "token-value",
        ] {
            let error = HostedExecutionReference::new(HostedExecutionReferenceKind::Input, value)
                .expect_err("unsafe reference must fail");
            assert_eq!(error.code(), "hosted.execution_reference.invalid");
            assert!(!error.to_string().contains(value));
        }
    }

    #[test]
    fn hosted_request_rejects_duplicate_capabilities() {
        let mut request = request();
        request
            .authorized_capabilities
            .push(request.authorized_capabilities[0].clone());
        let json =
            serde_json::to_string(&request).unwrap_or_else(|error| panic!("serialize: {error}"));
        let error =
            serde_json::from_str::<HostedExecutionRequest>(&json).expect_err("duplicate must fail");
        assert!(error.to_string().contains("duplicate reference"));
    }

    #[test]
    fn hosted_receipt_requires_exact_terminal_posture() {
        let request = request();
        let error = HostedExecutionReceipt::new(
            HostedExecutionId::new("execution-1").unwrap_or_else(|error| panic!("{error}")),
            HostedExecutionProviderId::new("provider/no-write")
                .unwrap_or_else(|error| panic!("{error}")),
            HostedExecutionProviderVersion::new("v1").unwrap_or_else(|error| panic!("{error}")),
            SpecContentHash::from_text("provider-configuration"),
            request.fingerprint(),
            HostedExecutionReference::new(
                HostedExecutionReferenceKind::Telemetry,
                "environment/sandbox-1",
            )
            .unwrap_or_else(|error| panic!("{error}")),
            SpecContentHash::from_text("policy"),
            timestamp("2026-07-29T00:00:00Z"),
            timestamp("2026-07-29T00:00:01Z"),
            HostedExecutionStatus::Completed,
            Some(HostedExecutionErrorCategory::Protocol),
            Some(0),
            Vec::new(),
        )
        .expect_err("completed receipt cannot carry an error");
        assert_eq!(
            error.code(),
            "hosted.execution_receipt.error_posture.invalid"
        );
    }

    #[test]
    fn hosted_attempt_requires_durable_pre_invocation_and_exact_receipt_binding() {
        let request = request();
        let provider = provider(true);
        let prepared = HostedExecutionAttempt::prepared(
            HostedExecutionId::new("execution-attempt-1").unwrap_or_else(|error| panic!("{error}")),
            HostedWorkItemId::new("work-item-attempt-1").unwrap_or_else(|error| panic!("{error}")),
            request.fingerprint(),
            provider.provider_id.clone(),
            provider.provider_version.clone(),
            provider.configuration_hash.clone(),
            timestamp("2026-07-29T00:00:00Z"),
        );
        assert_eq!(prepared.status(), HostedExecutionAttemptStatus::Prepared);
        let invoking = prepared
            .mark_invoking(timestamp("2026-07-29T00:00:01Z"))
            .unwrap_or_else(|error| panic!("{error}"));
        let reconciling = invoking
            .require_reconciliation(timestamp("2026-07-29T00:00:02Z"))
            .unwrap_or_else(|error| panic!("{error}"));
        let receipt = HostedExecutionReceipt::new(
            prepared.execution_id().clone(),
            provider.provider_id.clone(),
            provider.provider_version.clone(),
            provider.configuration_hash.clone(),
            request.fingerprint(),
            HostedExecutionReference::new(
                HostedExecutionReferenceKind::Telemetry,
                "environment/attempt-1",
            )
            .unwrap_or_else(|error| panic!("{error}")),
            request.policy().policy_hash().clone(),
            timestamp("2026-07-29T00:00:01Z"),
            timestamp("2026-07-29T00:00:03Z"),
            HostedExecutionStatus::Completed,
            None,
            Some(0),
            Vec::new(),
        )
        .unwrap_or_else(|error| panic!("{error}"));
        let terminal = reconciling
            .mark_terminal(&receipt)
            .unwrap_or_else(|error| panic!("{error}"));
        assert_eq!(terminal.status(), HostedExecutionAttemptStatus::Terminal);
        assert_eq!(
            terminal.terminal_status(),
            Some(HostedExecutionStatus::Completed)
        );

        let replay_error = terminal
            .mark_invoking(timestamp("2026-07-29T00:00:04Z"))
            .expect_err("terminal attempts cannot invoke again");
        assert_eq!(
            replay_error.code(),
            "hosted.execution_attempt.transition.invalid"
        );
    }

    #[test]
    fn hosted_attempt_rejects_substituted_receipt_without_leaking_identity() {
        let request = request();
        let provider = provider(true);
        let attempt = HostedExecutionAttempt::prepared(
            HostedExecutionId::new("execution-attempt-private")
                .unwrap_or_else(|error| panic!("{error}")),
            HostedWorkItemId::new("work-item-attempt-private")
                .unwrap_or_else(|error| panic!("{error}")),
            request.fingerprint(),
            provider.provider_id.clone(),
            provider.provider_version.clone(),
            provider.configuration_hash.clone(),
            timestamp("2026-07-29T00:00:00Z"),
        )
        .mark_invoking(timestamp("2026-07-29T00:00:01Z"))
        .unwrap_or_else(|error| panic!("{error}"));
        let substituted = HostedExecutionReceipt::new(
            HostedExecutionId::new("execution-other-private")
                .unwrap_or_else(|error| panic!("{error}")),
            provider.provider_id.clone(),
            provider.provider_version.clone(),
            provider.configuration_hash.clone(),
            request.fingerprint(),
            HostedExecutionReference::new(
                HostedExecutionReferenceKind::Telemetry,
                "environment/attempt-2",
            )
            .unwrap_or_else(|error| panic!("{error}")),
            request.policy().policy_hash().clone(),
            timestamp("2026-07-29T00:00:01Z"),
            timestamp("2026-07-29T00:00:02Z"),
            HostedExecutionStatus::Completed,
            None,
            Some(0),
            Vec::new(),
        )
        .unwrap_or_else(|error| panic!("{error}"));
        let error = attempt
            .mark_terminal(&substituted)
            .expect_err("substituted receipt must fail");
        assert_eq!(
            error.code(),
            "hosted.execution_receipt.attempt_binding.invalid"
        );
        assert!(!error.to_string().contains("private"));
        assert!(!format!("{attempt:?}").contains("private"));
    }

    struct NoWriteProvider {
        provider_id: HostedExecutionProviderId,
        provider_version: HostedExecutionProviderVersion,
        configuration_hash: SpecContentHash,
        bind_request: bool,
    }

    impl HostedExecutionProvider for NoWriteProvider {
        fn provider_id(&self) -> &HostedExecutionProviderId {
            &self.provider_id
        }

        fn provider_version(&self) -> &HostedExecutionProviderVersion {
            &self.provider_version
        }

        fn configuration_hash(&self) -> &SpecContentHash {
            &self.configuration_hash
        }

        fn execute(
            &self,
            request: &HostedExecutionRequest,
        ) -> Result<HostedExecutionReceipt, HostedExecutionInvocationError> {
            let request_fingerprint = if self.bind_request {
                request.fingerprint()
            } else {
                HostedExecutionRequestFingerprint(SpecContentHash::from_text("other-request"))
            };
            HostedExecutionReceipt::new(
                HostedExecutionId::new("execution-1").unwrap_or_else(|error| panic!("{error}")),
                self.provider_id.clone(),
                self.provider_version.clone(),
                self.configuration_hash.clone(),
                request_fingerprint,
                HostedExecutionReference::new(
                    HostedExecutionReferenceKind::Telemetry,
                    "environment/no-write-1",
                )
                .unwrap_or_else(|error| panic!("{error}")),
                request.policy().policy_hash().clone(),
                timestamp("2026-07-29T00:00:00Z"),
                timestamp("2026-07-29T00:00:01Z"),
                HostedExecutionStatus::Completed,
                None,
                Some(0),
                Vec::new(),
            )
            .map_err(|_| {
                HostedExecutionInvocationError::new(
                    HostedExecutionErrorCategory::Protocol,
                    HostedExecutionAttemptPosture::MayHaveStarted,
                )
            })
        }
    }

    fn provider(bind_request: bool) -> NoWriteProvider {
        NoWriteProvider {
            provider_id: HostedExecutionProviderId::new("provider/no-write")
                .unwrap_or_else(|error| panic!("{error}")),
            provider_version: HostedExecutionProviderVersion::new("v1")
                .unwrap_or_else(|error| panic!("{error}")),
            configuration_hash: SpecContentHash::from_text("provider-configuration"),
            bind_request,
        }
    }

    #[test]
    fn provider_receipt_binds_exact_request_and_configuration() {
        let request = request();
        let receipt = invoke_hosted_execution_provider(&provider(true), &request)
            .unwrap_or_else(|error| panic!("{error:?}"));
        assert_eq!(receipt.request_fingerprint(), &request.fingerprint());
        assert_eq!(receipt.policy_hash(), request.policy().policy_hash());
    }

    #[test]
    fn mismatched_provider_receipt_becomes_ambiguous() {
        let request = request();
        let error = invoke_hosted_execution_provider(&provider(false), &request)
            .expect_err("mismatched receipt must fail");
        assert_eq!(error.category(), HostedExecutionErrorCategory::Protocol);
        assert_eq!(
            error.attempt_posture(),
            HostedExecutionAttemptPosture::MayHaveStarted
        );
        assert_eq!(
            error.into_workflow_error().code(),
            "hosted.execution_provider.invocation.ambiguous"
        );
    }

    #[test]
    fn request_fingerprint_changes_with_governed_input() {
        let first = request();
        let mut second = request();
        second.budget =
            HostedExecutionBudget::new(301, 1024).unwrap_or_else(|error| panic!("{error}"));
        assert_ne!(first.fingerprint(), second.fingerprint());
    }

    #[test]
    fn hosted_work_item_transitions_are_deterministic() {
        let execution_request = request();
        let queued = HostedWorkItem::queued(
            HostedWorkItemId::new("work-item-1").unwrap_or_else(|error| panic!("{error}")),
            HostedCatalogEntryId::new("catalog/example").unwrap_or_else(|error| panic!("{error}")),
            WorkflowRunId::new("run-hosted-1").unwrap_or_else(|error| panic!("{error}")),
            WorkflowId::new("hosted/example").unwrap_or_else(|error| panic!("{error}")),
            ImmutableRunBundleId::new("bundle-hosted-1").unwrap_or_else(|error| panic!("{error}")),
            ImmutableRunBundleVersion::new("v1").unwrap_or_else(|error| panic!("{error}")),
            SpecContentHash::from_text("bundle"),
            ActorId::new("user/maintainer").unwrap_or_else(|error| panic!("{error}")),
            CorrelationId::new("correlation-hosted-1").unwrap_or_else(|error| panic!("{error}")),
            IdempotencyKey::new("hosted-request-1").unwrap_or_else(|error| panic!("{error}")),
            execution_request,
            timestamp("2026-07-29T00:00:00Z"),
        )
        .unwrap_or_else(|error| panic!("{error}"));
        let running = queued
            .transition(
                HostedWorkItemStatus::Running,
                timestamp("2026-07-29T00:00:01Z"),
            )
            .unwrap_or_else(|error| panic!("{error}"));
        assert_eq!(running.attempt_count(), 1);
        let completed = running
            .transition(
                HostedWorkItemStatus::Completed,
                timestamp("2026-07-29T00:00:02Z"),
            )
            .unwrap_or_else(|error| panic!("{error}"));
        assert_eq!(completed.status(), HostedWorkItemStatus::Completed);
        assert!(completed
            .transition(
                HostedWorkItemStatus::Running,
                timestamp("2026-07-29T00:00:03Z")
            )
            .is_err());
        let reclaimed = running
            .reclaim(timestamp("2026-07-29T00:00:03Z"))
            .unwrap_or_else(|error| panic!("{error}"));
        assert_eq!(reclaimed.status(), HostedWorkItemStatus::Running);
        assert_eq!(reclaimed.attempt_count(), 2);
    }

    #[test]
    fn hosted_dispatch_and_terminal_receipt_project_authoritative_run_events() {
        let run = dispatch_run();
        let item = work_item();
        let dispatch = HostedSkillDispatch::new(
            &run,
            item.clone(),
            SkillInvocationId::new("invocation-hosted-1").unwrap_or_else(|error| panic!("{error}")),
            SkillId::new("hosted/no-write").unwrap_or_else(|error| panic!("{error}")),
            SkillVersion::new("v1").unwrap_or_else(|error| panic!("{error}")),
            SkillAttemptId::new("attempt-hosted-1").unwrap_or_else(|error| panic!("{error}")),
            timestamp("2026-07-29T00:00:01Z"),
        )
        .unwrap_or_else(|error| panic!("{error}"));
        let mut dispatched_events = run.events.clone();
        dispatched_events.push(dispatch.invocation_requested().clone());
        dispatched_events.push(dispatch.invocation_started().clone());
        let dispatched_run =
            WorkflowRun::rehydrate(&dispatched_events).unwrap_or_else(|error| panic!("{error}"));
        assert_eq!(dispatched_run.snapshot.status, WorkflowRunStatus::Running);

        let receipt = completed_receipt(item.execution_request());
        let projection = HostedTerminalResultProjection::new(
            &dispatched_run,
            &item,
            receipt,
            ActorId::new("worker/hosted-test").unwrap_or_else(|error| panic!("{error}")),
        )
        .unwrap_or_else(|error| panic!("{error}"));
        assert_eq!(
            projection.projected_run().snapshot.status,
            WorkflowRunStatus::Completed
        );
        assert!(matches!(
            projection.events()[0].kind(),
            crate::WorkflowRunEventKindName::SkillInvocationSucceeded
        ));
        assert!(matches!(
            projection.events()[1].kind(),
            crate::WorkflowRunEventKindName::RunCompleted
        ));
        assert!(!format!("{dispatch:?}").contains("work-item-hosted-1"));
        assert!(!format!("{projection:?}").contains("execution-projection-1"));
    }

    #[test]
    fn hosted_pre_start_rejection_projects_authoritative_failure_without_receipt() {
        let (run, item) = dispatched_run_and_item();
        let projection = HostedUnreceiptedResultProjection::new(
            &run,
            &item,
            HostedUnreceiptedOutcome::RejectedBeforeStart,
            ActorId::new("worker/hosted-test").unwrap_or_else(|error| panic!("{error}")),
            timestamp("2026-07-29T00:00:02Z"),
        )
        .unwrap_or_else(|error| panic!("{error}"));
        assert_eq!(
            projection.projected_run().snapshot.status,
            WorkflowRunStatus::Failed
        );
        assert_eq!(projection.events().len(), 2);
        assert!(matches!(
            projection.events()[0].kind(),
            crate::WorkflowRunEventKindName::SkillInvocationFailed
        ));
        assert!(matches!(
            projection.events()[1].kind(),
            crate::WorkflowRunEventKindName::RunFailed
        ));
        assert!(!format!("{projection:?}").contains("work-item-hosted-1"));
    }

    #[test]
    fn hosted_provider_uncertainty_projects_reconciliation_escalation() {
        let (run, item) = dispatched_run_and_item();
        let projection = HostedUnreceiptedResultProjection::new(
            &run,
            &item,
            HostedUnreceiptedOutcome::ReconciliationRequired,
            ActorId::new("worker/hosted-test").unwrap_or_else(|error| panic!("{error}")),
            timestamp("2026-07-29T00:00:02Z"),
        )
        .unwrap_or_else(|error| panic!("{error}"));
        assert_eq!(
            projection.projected_run().snapshot.status,
            WorkflowRunStatus::Escalated
        );
        assert_eq!(projection.events().len(), 1);
        assert!(matches!(
            projection.events()[0].kind(),
            crate::WorkflowRunEventKindName::EscalationTriggered
        ));
        assert_eq!(
            projection.projected_run().snapshot.escalations[0].attempts,
            1
        );
        assert!(!format!("{projection:?}").contains("work-item-hosted-1"));
    }

    #[test]
    fn hosted_ambiguous_receipt_projects_escalation_not_failure() {
        let (run, item) = dispatched_run_and_item();
        let completed = completed_receipt(item.execution_request());
        let ambiguous = HostedExecutionReceipt::new(
            completed.execution_id().clone(),
            completed.provider_id().clone(),
            completed.provider_version().clone(),
            completed.provider_configuration_hash().clone(),
            item.execution_request().fingerprint(),
            completed.environment_reference().clone(),
            item.execution_request().policy().policy_hash().clone(),
            completed.started_at(),
            completed.terminal_at(),
            HostedExecutionStatus::Ambiguous,
            Some(HostedExecutionErrorCategory::Ambiguous),
            None,
            Vec::new(),
        )
        .unwrap_or_else(|error| panic!("{error}"));
        let projection = HostedTerminalResultProjection::new(
            &run,
            &item,
            ambiguous,
            ActorId::new("worker/hosted-test").unwrap_or_else(|error| panic!("{error}")),
        )
        .unwrap_or_else(|error| panic!("{error}"));
        assert_eq!(
            projection.projected_run().snapshot.status,
            WorkflowRunStatus::Escalated
        );
        assert_eq!(projection.events().len(), 1);
        assert!(matches!(
            projection.events()[0].kind(),
            crate::WorkflowRunEventKindName::EscalationTriggered
        ));
        assert_eq!(
            projection.projected_run().snapshot.escalations[0].attempts,
            1
        );
    }

    #[test]
    fn hosted_projection_rejects_substituted_receipt_without_leaking() {
        let run = dispatch_run();
        let item = work_item();
        let dispatch = HostedSkillDispatch::new(
            &run,
            item.clone(),
            SkillInvocationId::new("invocation-hosted-private")
                .unwrap_or_else(|error| panic!("{error}")),
            SkillId::new("hosted/no-write").unwrap_or_else(|error| panic!("{error}")),
            SkillVersion::new("v1").unwrap_or_else(|error| panic!("{error}")),
            SkillAttemptId::new("attempt-hosted-private").unwrap_or_else(|error| panic!("{error}")),
            timestamp("2026-07-29T00:00:01Z"),
        )
        .unwrap_or_else(|error| panic!("{error}"));
        let mut events = run.events.clone();
        events.push(dispatch.invocation_requested().clone());
        events.push(dispatch.invocation_started().clone());
        let dispatched_run =
            WorkflowRun::rehydrate(&events).unwrap_or_else(|error| panic!("{error}"));
        let other_request = HostedExecutionRequest::new(
            item.execution_request().run_id().clone(),
            item.execution_request().workflow_id().clone(),
            item.execution_request().workflow_version().clone(),
            item.execution_request().schema_version().clone(),
            item.execution_request().step_id().clone(),
            item.execution_request().bundle_id().clone(),
            item.execution_request().bundle_version().clone(),
            item.execution_request().bundle_root_hash().clone(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            item.execution_request().policy().clone(),
            HostedExecutionBudget::new(301, 1024).unwrap_or_else(|error| panic!("{error}")),
            item.execution_request().correlation_id().clone(),
            item.execution_request().idempotency_key().clone(),
            Vec::new(),
        )
        .unwrap_or_else(|error| panic!("{error}"));
        let substituted = completed_receipt(&other_request);
        let error = HostedTerminalResultProjection::new(
            &dispatched_run,
            &item,
            substituted,
            ActorId::new("worker/private").unwrap_or_else(|error| panic!("{error}")),
        )
        .expect_err("substituted receipt must fail");
        assert_eq!(error.code(), "hosted.result.receipt_binding.invalid");
        assert!(!error.to_string().contains("private"));
    }

    #[test]
    fn debug_output_redacts_hosted_identity() {
        let debug = format!("{:?}", request());
        assert!(!debug.contains("run-hosted-1"));
        assert!(!debug.contains("artifact/input-1"));
        assert!(debug.contains("[REDACTED]"));
    }
}
