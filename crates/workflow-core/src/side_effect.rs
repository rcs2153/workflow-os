use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    ActorId, AdapterId, AdapterKind, ApprovalDecisionKind, ApprovalRequest, CorrelationId,
    IdempotencyKey, IntegrationId, RedactionMetadata, SchemaVersion, SideEffectWorkflowEvent,
    SideEffectWorkflowEventDefinition, SkillId, SkillVersion, SpecContentHash, StepId, Timestamp,
    WorkflowId, WorkflowOsError, WorkflowRun, WorkflowRunEventKind, WorkflowRunId, WorkflowVersion,
};

use crate::state::SideEffectRecordStore;

const SIDE_EFFECT_ID_MAX_BYTES: usize = 128;
const SIDE_EFFECT_REFERENCE_MAX_BYTES: usize = 256;
const SIDE_EFFECT_SUMMARY_MAX_BYTES: usize = 1_000;
const SIDE_EFFECT_REASON_MAX_BYTES: usize = 128;
const SIDE_EFFECT_REASON_MAX_COUNT: usize = 16;
const SIDE_EFFECT_REFERENCE_MAX_COUNT: usize = 64;
const SIDE_EFFECT_REDACTION_FIELD_MAX_BYTES: usize = 128;
const SIDE_EFFECT_REDACTION_REASON_MAX_BYTES: usize = 512;
const SIDE_EFFECT_REDACTION_MAX_ENTRIES: usize = 64;

static NEXT_SIDE_EFFECT_ID: AtomicU64 = AtomicU64::new(1);

/// Identifier for one governed side-effect record.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct SideEffectId(String);

impl SideEffectId {
    /// Generates a new side-effect ID.
    #[must_use]
    pub fn generate() -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let counter = NEXT_SIDE_EFFECT_ID.fetch_add(1, Ordering::Relaxed);
        Self(format!("side-effect-{timestamp}-{counter}"))
    }

    /// Creates a validated side-effect ID.
    ///
    /// # Errors
    ///
    /// Returns an error when the ID is empty, too long, invalid, or secret-like.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        validate_identifier("side-effect id", &value)?;
        Ok(Self(value))
    }

    /// Returns the side-effect ID as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SideEffectId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Debug for SideEffectId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("SideEffectId")
            .field(&"[REDACTED]")
            .finish()
    }
}

impl From<SideEffectId> for String {
    fn from(value: SideEffectId) -> Self {
        value.0
    }
}

impl TryFrom<String> for SideEffectId {
    type Error = WorkflowOsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for SideEffectId {
    type Err = WorkflowOsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Lifecycle state of a governed side-effect record.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideEffectLifecycleState {
    /// The side effect was proposed but not attempted.
    Proposed,
    /// The side effect was attempted after authority checks allowed it.
    Attempted,
    /// The side effect completed and has an outcome reference.
    Completed,
    /// The side effect was denied before attempt.
    Denied,
    /// The side effect was intentionally skipped.
    Skipped,
    /// The side effect was attempted and failed.
    Failed,
}

/// Domain-neutral target kind for a future side-effect.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideEffectTargetKind {
    /// External resource managed by a provider.
    ExternalResource,
    /// Resource addressed through an adapter.
    AdapterResource,
    /// Workflow-owned resource.
    WorkflowResource,
    /// Local resource.
    LocalResource,
    /// Provider operation reference.
    ProviderOperation,
    /// Unknown target kind. Valid records fail closed when this is used.
    Unknown,
}

/// Bounded, non-payload reference to the target of a side-effect.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct SideEffectTargetReference {
    kind: SideEffectTargetKind,
    reference: String,
}

impl SideEffectTargetReference {
    /// Creates a validated side-effect target reference.
    ///
    /// # Errors
    ///
    /// Returns an error when the reference is empty, too long, invalid, or secret-like.
    pub fn new(
        kind: SideEffectTargetKind,
        reference: impl Into<String>,
    ) -> Result<Self, WorkflowOsError> {
        let target = Self {
            kind,
            reference: reference.into(),
        };
        target.validate()?;
        Ok(target)
    }

    /// Validates the target reference.
    ///
    /// # Errors
    ///
    /// Returns an error when the target kind or reference is invalid.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        if self.kind == SideEffectTargetKind::Unknown {
            return Err(validation_error(
                "side_effect.target.unknown",
                "side-effect target kind must be known",
            ));
        }
        validate_reference("side-effect target reference", &self.reference)
    }

    /// Returns the target kind.
    #[must_use]
    pub const fn kind(&self) -> SideEffectTargetKind {
        self.kind
    }

    /// Returns the bounded target reference.
    #[must_use]
    pub fn reference(&self) -> &str {
        &self.reference
    }
}

impl fmt::Debug for SideEffectTargetReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SideEffectTargetReference")
            .field("kind", &self.kind)
            .field("reference", &"[REDACTED]")
            .finish()
    }
}

impl<'de> Deserialize<'de> for SideEffectTargetReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            kind: SideEffectTargetKind,
            reference: String,
        }

        let wire = Wire::deserialize(deserializer)?;
        Self::new(wire.kind, wire.reference).map_err(serde::de::Error::custom)
    }
}

/// Domain-neutral side-effect capability vocabulary.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideEffectCapability {
    /// External write.
    ExternalWrite,
    /// Local write.
    LocalWrite,
    /// Adapter-mediated write.
    AdapterWrite,
    /// GitHub write.
    GitHubWrite,
    /// Jira write.
    JiraWrite,
    /// CI write.
    CiWrite,
    /// Workflow dispatch.
    WorkflowDispatch,
    /// CI rerun.
    CiRerun,
    /// Unknown capability. Unsafe lifecycle states fail closed when this is used.
    Unknown,
}

/// Authority decision for a side-effect.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideEffectAuthorityDecision {
    /// Authority has not been evaluated.
    NotEvaluated,
    /// Policy allowed the action.
    AllowedByPolicy,
    /// Policy requires human approval before attempt.
    RequiresApproval,
    /// Human approval was granted.
    ApprovedByHuman,
    /// Policy denied the action.
    DeniedByPolicy,
    /// Human approval denied the action.
    DeniedByApproval,
    /// Capability policy denied the action.
    DeniedByCapability,
    /// Kill switch denied the action.
    DeniedByKillSwitch,
    /// Validation denied the action.
    DeniedByValidation,
    /// Capability or execution path is unsupported.
    Unsupported,
}

impl SideEffectAuthorityDecision {
    const fn allows_attempt(self) -> bool {
        matches!(self, Self::AllowedByPolicy | Self::ApprovedByHuman)
    }

    const fn denies_or_unsupported(self) -> bool {
        matches!(
            self,
            Self::DeniedByPolicy
                | Self::DeniedByApproval
                | Self::DeniedByCapability
                | Self::DeniedByKillSwitch
                | Self::DeniedByValidation
                | Self::Unsupported
        )
    }
}

/// Authority context for a side-effect record.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SideEffectAuthority {
    /// Decision describing whether the side effect may proceed.
    pub decision: SideEffectAuthorityDecision,
    /// Stable policy references where available.
    pub policy_references: Vec<SideEffectReference>,
    /// Stable approval references where available.
    pub approval_references: Vec<SideEffectReference>,
}

impl SideEffectAuthority {
    /// Creates a validated authority context.
    ///
    /// # Errors
    ///
    /// Returns an error when references are invalid or duplicated.
    pub fn new(
        decision: SideEffectAuthorityDecision,
        policy_references: Vec<SideEffectReference>,
        approval_references: Vec<SideEffectReference>,
    ) -> Result<Self, WorkflowOsError> {
        let authority = Self {
            decision,
            policy_references,
            approval_references,
        };
        authority.validate()?;
        Ok(authority)
    }

    /// Validates authority references.
    ///
    /// # Errors
    ///
    /// Returns an error when references are invalid or duplicated.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_references(&self.policy_references)?;
        validate_references(&self.approval_references)
    }
}

/// Scope for a side-effect idempotency binding.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideEffectIdempotencyScope {
    /// Binding is scoped to a run.
    Run,
    /// Binding is scoped to a workflow step.
    Step,
    /// Binding is scoped to an adapter.
    Adapter,
    /// Binding is scoped to an integration.
    Integration,
}

/// Non-payload outcome reference for completed, failed, or duplicate handling.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct SideEffectOutcomeReference {
    kind: SideEffectOutcomeReferenceKind,
    reference: String,
}

impl SideEffectOutcomeReference {
    /// Creates a validated outcome reference.
    ///
    /// # Errors
    ///
    /// Returns an error when the reference is empty, too long, invalid, or secret-like.
    pub fn new(
        kind: SideEffectOutcomeReferenceKind,
        reference: impl Into<String>,
    ) -> Result<Self, WorkflowOsError> {
        let outcome = Self {
            kind,
            reference: reference.into(),
        };
        outcome.validate()?;
        Ok(outcome)
    }

    /// Validates the outcome reference.
    ///
    /// # Errors
    ///
    /// Returns an error when the reference is invalid.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_reference("side-effect outcome reference", &self.reference)
    }

    /// Returns the outcome reference kind.
    #[must_use]
    pub const fn kind(&self) -> SideEffectOutcomeReferenceKind {
        self.kind
    }

    /// Returns the bounded outcome reference.
    #[must_use]
    pub fn reference(&self) -> &str {
        &self.reference
    }
}

impl fmt::Debug for SideEffectOutcomeReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SideEffectOutcomeReference")
            .field("kind", &self.kind)
            .field("reference", &"[REDACTED]")
            .finish()
    }
}

impl<'de> Deserialize<'de> for SideEffectOutcomeReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            kind: SideEffectOutcomeReferenceKind,
            reference: String,
        }

        let wire = Wire::deserialize(deserializer)?;
        Self::new(wire.kind, wire.reference).map_err(serde::de::Error::custom)
    }
}

/// Kind of non-payload outcome reference.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideEffectOutcomeReferenceKind {
    /// Completed outcome.
    Outcome,
    /// Failure classification or reference.
    Failure,
    /// Duplicate idempotency outcome.
    Duplicate,
}

/// Idempotency binding for a side-effect record.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SideEffectIdempotencyBinding {
    key: IdempotencyKey,
    scope: SideEffectIdempotencyScope,
    prior_side_effect_id: Option<SideEffectId>,
    outcome_reference: Option<SideEffectOutcomeReference>,
}

impl SideEffectIdempotencyBinding {
    /// Creates a validated idempotency binding.
    ///
    /// # Errors
    ///
    /// Returns an error when nested references are invalid.
    pub fn new(
        key: IdempotencyKey,
        scope: SideEffectIdempotencyScope,
        prior_side_effect_id: Option<SideEffectId>,
        outcome_reference: Option<SideEffectOutcomeReference>,
    ) -> Result<Self, WorkflowOsError> {
        let binding = Self {
            key,
            scope,
            prior_side_effect_id,
            outcome_reference,
        };
        binding.validate()?;
        Ok(binding)
    }

    /// Validates the binding.
    ///
    /// # Errors
    ///
    /// Returns an error when nested references are invalid.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        if let Some(outcome_reference) = &self.outcome_reference {
            outcome_reference.validate()?;
        }
        Ok(())
    }

    /// Returns the idempotency key.
    #[must_use]
    pub const fn key(&self) -> &IdempotencyKey {
        &self.key
    }

    /// Returns the idempotency scope.
    #[must_use]
    pub const fn scope(&self) -> SideEffectIdempotencyScope {
        self.scope
    }
}

impl<'de> Deserialize<'de> for SideEffectIdempotencyBinding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            key: IdempotencyKey,
            scope: SideEffectIdempotencyScope,
            prior_side_effect_id: Option<SideEffectId>,
            outcome_reference: Option<SideEffectOutcomeReference>,
        }

        let wire = Wire::deserialize(deserializer)?;
        Self::new(
            wire.key,
            wire.scope,
            wire.prior_side_effect_id,
            wire.outcome_reference,
        )
        .map_err(serde::de::Error::custom)
    }
}

/// Stable reference kinds a side-effect record may cite.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideEffectReferenceKind {
    /// Workflow event ID.
    WorkflowEvent,
    /// Audit event ID.
    AuditEvent,
    /// Policy decision or policy event reference.
    PolicyDecision,
    /// Approval decision reference.
    ApprovalDecision,
    /// Adapter telemetry record reference.
    AdapterTelemetry,
    /// `EvidenceReference` ID.
    EvidenceReference,
    /// `WorkReport` ID.
    WorkReport,
    /// Local check result reference.
    LocalCheckResult,
    /// Typed handoff ID.
    TypedHandoff,
}

/// Bounded stable reference associated with a side-effect record.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
pub struct SideEffectReference {
    kind: SideEffectReferenceKind,
    reference: String,
}

impl SideEffectReference {
    /// Creates a validated side-effect reference.
    ///
    /// # Errors
    ///
    /// Returns an error when the reference is empty, too long, invalid, or secret-like.
    pub fn new(
        kind: SideEffectReferenceKind,
        reference: impl Into<String>,
    ) -> Result<Self, WorkflowOsError> {
        let reference = Self {
            kind,
            reference: reference.into(),
        };
        reference.validate()?;
        Ok(reference)
    }

    /// Validates the side-effect reference.
    ///
    /// # Errors
    ///
    /// Returns an error when the reference is invalid.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_reference("side-effect reference", &self.reference)
    }

    /// Returns the reference kind.
    #[must_use]
    pub const fn kind(&self) -> SideEffectReferenceKind {
        self.kind
    }

    /// Returns the bounded reference text.
    #[must_use]
    pub fn reference(&self) -> &str {
        &self.reference
    }
}

impl fmt::Debug for SideEffectReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SideEffectReference")
            .field("kind", &self.kind)
            .field("reference", &"[REDACTED]")
            .finish()
    }
}

impl<'de> Deserialize<'de> for SideEffectReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            kind: SideEffectReferenceKind,
            reference: String,
        }

        let wire = Wire::deserialize(deserializer)?;
        Self::new(wire.kind, wire.reference).map_err(serde::de::Error::custom)
    }
}

/// Sensitivity classification for a side-effect record.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideEffectSensitivity {
    /// Public metadata.
    Public,
    /// Internal metadata.
    Internal,
    /// Confidential metadata.
    Confidential,
    /// Regulated metadata.
    Regulated,
    /// Secret side-effect metadata. Values still must not store secrets.
    Secret,
    /// Unknown sensitivity, treated conservatively.
    Unknown,
}

impl SideEffectSensitivity {
    /// Conservative default for side-effect records.
    #[must_use]
    pub const fn conservative_default() -> Self {
        Self::Confidential
    }
}

/// Domain-neutral model record for a governed side-effect boundary.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct SideEffectRecord {
    side_effect_id: SideEffectId,
    lifecycle_state: SideEffectLifecycleState,
    target: SideEffectTargetReference,
    capability: SideEffectCapability,
    authority: SideEffectAuthority,
    actor: Option<ActorId>,
    system_actor: Option<ActorId>,
    workflow_id: WorkflowId,
    workflow_version: WorkflowVersion,
    schema_version: SchemaVersion,
    spec_hash: SpecContentHash,
    run_id: WorkflowRunId,
    step_id: Option<StepId>,
    skill_id: Option<SkillId>,
    skill_version: Option<SkillVersion>,
    adapter_id: Option<AdapterId>,
    adapter_kind: Option<AdapterKind>,
    integration_id: Option<IntegrationId>,
    idempotency: SideEffectIdempotencyBinding,
    references: Vec<SideEffectReference>,
    outcome_reference: Option<SideEffectOutcomeReference>,
    created_at: Timestamp,
    updated_at: Option<Timestamp>,
    correlation_id: Option<CorrelationId>,
    summary: Option<String>,
    reason_codes: Vec<String>,
    sensitivity: SideEffectSensitivity,
    redaction: RedactionMetadata,
}

/// Input fields for constructing a validated `SideEffectRecord`.
pub struct SideEffectRecordDefinition {
    /// Side-effect ID.
    pub side_effect_id: SideEffectId,
    /// Lifecycle state.
    pub lifecycle_state: SideEffectLifecycleState,
    /// Target reference.
    pub target: SideEffectTargetReference,
    /// Requested capability.
    pub capability: SideEffectCapability,
    /// Authority context.
    pub authority: SideEffectAuthority,
    /// Human or delegated actor where available.
    pub actor: Option<ActorId>,
    /// System actor where available.
    pub system_actor: Option<ActorId>,
    /// Workflow ID.
    pub workflow_id: WorkflowId,
    /// Workflow version.
    pub workflow_version: WorkflowVersion,
    /// Schema version.
    pub schema_version: SchemaVersion,
    /// Workflow spec content hash.
    pub spec_hash: SpecContentHash,
    /// Workflow run ID.
    pub run_id: WorkflowRunId,
    /// Step ID where available.
    pub step_id: Option<StepId>,
    /// Skill ID where available.
    pub skill_id: Option<SkillId>,
    /// Skill version where available.
    pub skill_version: Option<SkillVersion>,
    /// Adapter ID where available.
    pub adapter_id: Option<AdapterId>,
    /// Adapter kind where available.
    pub adapter_kind: Option<AdapterKind>,
    /// Integration ID where available.
    pub integration_id: Option<IntegrationId>,
    /// Idempotency binding.
    pub idempotency: SideEffectIdempotencyBinding,
    /// Stable references to related governance records.
    pub references: Vec<SideEffectReference>,
    /// Non-payload outcome or failure reference where applicable.
    pub outcome_reference: Option<SideEffectOutcomeReference>,
    /// Created timestamp.
    pub created_at: Timestamp,
    /// Updated/finalized timestamp where applicable.
    pub updated_at: Option<Timestamp>,
    /// Correlation ID where available.
    pub correlation_id: Option<CorrelationId>,
    /// Optional bounded non-secret summary.
    pub summary: Option<String>,
    /// Stable non-secret reason codes.
    pub reason_codes: Vec<String>,
    /// Sensitivity classification.
    pub sensitivity: SideEffectSensitivity,
    /// Redaction metadata.
    pub redaction: RedactionMetadata,
}

/// Explicit input for validating `SideEffect` approval authority linkage.
///
/// This helper input is reference-only. It validates already-loaded
/// `SideEffect` records against an already-existing workflow run. It does not
/// create approvals, create records, append events, emit audit records, write
/// artifacts, call adapters, or execute side effects.
#[derive(Clone, Copy)]
pub struct SideEffectApprovalLinkageInput<'a> {
    /// Workflow run whose event history is the source of approval truth.
    pub run: &'a WorkflowRun,
    /// Already-loaded `SideEffect` records to validate.
    pub side_effect_records: &'a [SideEffectRecord],
    /// Whether `RequiresApproval` records must cite an approval request.
    pub require_approval_references_for_requires_approval: bool,
    /// Whether `ApprovedByHuman` and `DeniedByApproval` records require a decision.
    pub require_decision_for_approved_or_denied: bool,
}

/// Explicit input for transitioning a proposed side-effect record to attempted.
///
/// This input is pure/local. It does not call providers, append workflow events,
/// write stores, or execute a side effect.
pub struct SideEffectAttemptTransitionInput<'a> {
    /// Prior proposed side-effect record.
    pub prior_record: &'a SideEffectRecord,
    /// Timestamp for the attempted transition.
    pub transitioned_at: Timestamp,
    /// Optional bounded non-secret transition summary.
    pub summary: Option<String>,
    /// Stable non-secret references to add to the transition record.
    pub additional_references: Vec<SideEffectReference>,
    /// Count of evidence references associated elsewhere.
    pub evidence_reference_count: u32,
}

/// Explicit input for transitioning an attempted side-effect record to completed.
///
/// This input is pure/local. It does not call providers, append workflow events,
/// write stores, or execute a side effect.
pub struct SideEffectCompleteTransitionInput<'a> {
    /// Prior attempted side-effect record.
    pub prior_record: &'a SideEffectRecord,
    /// Timestamp for the completed transition.
    pub transitioned_at: Timestamp,
    /// Stable, bounded provider or adapter outcome reference.
    pub outcome_reference: SideEffectOutcomeReference,
    /// Optional bounded non-secret transition summary.
    pub summary: Option<String>,
    /// Stable non-secret references to add to the transition record.
    pub additional_references: Vec<SideEffectReference>,
    /// Count of evidence references associated elsewhere.
    pub evidence_reference_count: u32,
}

/// Explicit input for transitioning an attempted side-effect record to failed.
///
/// This input is pure/local. It does not call providers, append workflow events,
/// write stores, or execute a side effect.
pub struct SideEffectFailTransitionInput<'a> {
    /// Prior attempted side-effect record.
    pub prior_record: &'a SideEffectRecord,
    /// Timestamp for the failed transition.
    pub transitioned_at: Timestamp,
    /// Optional stable, bounded provider or adapter failure reference.
    pub outcome_reference: Option<SideEffectOutcomeReference>,
    /// Stable non-secret failure reason codes.
    pub reason_codes: Vec<String>,
    /// Optional bounded non-secret transition summary.
    pub summary: Option<String>,
    /// Stable non-secret references to add to the transition record.
    pub additional_references: Vec<SideEffectReference>,
    /// Count of evidence references associated elsewhere.
    pub evidence_reference_count: u32,
}

/// Explicit store-backed input for transitioning a proposed side-effect record to attempted.
pub struct SideEffectAttemptTransitionStoreInput<'a> {
    /// Stable side-effect record identifier to load from the store.
    pub side_effect_id: &'a SideEffectId,
    /// Timestamp for the attempted transition.
    pub transitioned_at: Timestamp,
    /// Optional bounded non-secret transition summary.
    pub summary: Option<String>,
    /// Stable non-secret references to add to the transition record.
    pub additional_references: Vec<SideEffectReference>,
    /// Count of evidence references associated elsewhere.
    pub evidence_reference_count: u32,
}

/// Explicit store-backed input for transitioning an attempted side-effect record to completed.
pub struct SideEffectCompleteTransitionStoreInput<'a> {
    /// Stable side-effect record identifier to load from the store.
    pub side_effect_id: &'a SideEffectId,
    /// Timestamp for the completed transition.
    pub transitioned_at: Timestamp,
    /// Stable, bounded provider or adapter outcome reference.
    pub outcome_reference: SideEffectOutcomeReference,
    /// Optional bounded non-secret transition summary.
    pub summary: Option<String>,
    /// Stable non-secret references to add to the transition record.
    pub additional_references: Vec<SideEffectReference>,
    /// Count of evidence references associated elsewhere.
    pub evidence_reference_count: u32,
}

/// Explicit store-backed input for transitioning an attempted side-effect record to failed.
pub struct SideEffectFailTransitionStoreInput<'a> {
    /// Stable side-effect record identifier to load from the store.
    pub side_effect_id: &'a SideEffectId,
    /// Timestamp for the failed transition.
    pub transitioned_at: Timestamp,
    /// Optional stable, bounded provider or adapter failure reference.
    pub outcome_reference: Option<SideEffectOutcomeReference>,
    /// Stable non-secret failure reason codes.
    pub reason_codes: Vec<String>,
    /// Optional bounded non-secret transition summary.
    pub summary: Option<String>,
    /// Stable non-secret references to add to the transition record.
    pub additional_references: Vec<SideEffectReference>,
    /// Count of evidence references associated elsewhere.
    pub evidence_reference_count: u32,
}

/// Pure result of a side-effect lifecycle transition.
pub struct SideEffectLifecycleTransitionResult {
    record: SideEffectRecord,
    event: SideEffectWorkflowEvent,
}

impl SideEffectLifecycleTransitionResult {
    /// Returns the transitioned side-effect record.
    #[must_use]
    pub const fn record(&self) -> &SideEffectRecord {
        &self.record
    }

    /// Returns the reference-only workflow event payload for a caller to append later.
    #[must_use]
    pub const fn event(&self) -> &SideEffectWorkflowEvent {
        &self.event
    }

    /// Consumes the result into the transitioned record and event payload.
    #[must_use]
    pub fn into_parts(self) -> (SideEffectRecord, SideEffectWorkflowEvent) {
        (self.record, self.event)
    }
}

impl fmt::Debug for SideEffectLifecycleTransitionResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SideEffectLifecycleTransitionResult")
            .field("lifecycle_state", &self.record.lifecycle_state())
            .field(
                "has_outcome_reference",
                &self.record.outcome_reference().is_some(),
            )
            .field("reference_count", &self.record.references().len())
            .field(
                "evidence_reference_count",
                &self.event.evidence_reference_count(),
            )
            .field(
                "outcome_reference_count",
                &self.event.outcome_reference_count(),
            )
            .finish()
    }
}

impl fmt::Debug for SideEffectApprovalLinkageInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SideEffectApprovalLinkageInput")
            .field("run", &"[REDACTED]")
            .field("side_effect_record_count", &self.side_effect_records.len())
            .field(
                "require_approval_references_for_requires_approval",
                &self.require_approval_references_for_requires_approval,
            )
            .field(
                "require_decision_for_approved_or_denied",
                &self.require_decision_for_approved_or_denied,
            )
            .finish()
    }
}

/// Explicit input for store-backed `SideEffect` approval authority linkage.
///
/// This helper input is validation-only. It reads already-persisted
/// `SideEffect` records through the supplied store and then delegates approval
/// linkage validation to `validate_side_effect_approval_linkage(...)`. It does
/// not create records, append events, emit audit records, write artifacts, call
/// adapters, or execute side effects.
#[derive(Clone, Copy)]
pub struct SideEffectApprovalLinkageFromStoreInput<'a> {
    /// Workflow run whose event history is the source of approval truth.
    pub run: &'a WorkflowRun,
    /// Explicit side-effect IDs to load.
    pub side_effect_ids: &'a [SideEffectId],
    /// Store load mode.
    pub load_mode: SideEffectApprovalLinkageStoreLoadMode,
    /// Missing explicit record policy.
    pub missing_record_policy: SideEffectMissingRecordPolicy,
    /// Whether `RequiresApproval` records must cite an approval request.
    pub require_approval_references_for_requires_approval: bool,
    /// Whether `ApprovedByHuman` and `DeniedByApproval` records require a decision.
    pub require_decision_for_approved_or_denied: bool,
}

/// Store load mode for store-backed `SideEffect` approval linkage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SideEffectApprovalLinkageStoreLoadMode {
    /// Load only explicitly supplied `SideEffectId` values.
    ExplicitIds,
    /// Load all records matching the supplied run identity.
    AllRecordsForRun,
    /// Load explicit IDs and all records matching the supplied run identity.
    ExplicitIdsAndAllRecordsForRun,
}

impl SideEffectApprovalLinkageStoreLoadMode {
    const fn includes_explicit_ids(self) -> bool {
        matches!(
            self,
            Self::ExplicitIds | Self::ExplicitIdsAndAllRecordsForRun
        )
    }

    const fn includes_all_records_for_run(self) -> bool {
        matches!(
            self,
            Self::AllRecordsForRun | Self::ExplicitIdsAndAllRecordsForRun
        )
    }
}

/// Missing explicit record policy for store-backed `SideEffect` approval linkage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SideEffectMissingRecordPolicy {
    /// Every explicit `SideEffectId` must resolve to a record.
    RequireAll,
    /// Missing explicit `SideEffectId` values are counted and ignored.
    CountMissing,
}

impl SideEffectMissingRecordPolicy {
    const fn requires_all(self) -> bool {
        matches!(self, Self::RequireAll)
    }
}

impl fmt::Debug for SideEffectApprovalLinkageFromStoreInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SideEffectApprovalLinkageFromStoreInput")
            .field("run", &"[REDACTED]")
            .field("side_effect_id_count", &self.side_effect_ids.len())
            .field("load_mode", &self.load_mode)
            .field("missing_record_policy", &self.missing_record_policy)
            .field(
                "require_approval_references_for_requires_approval",
                &self.require_approval_references_for_requires_approval,
            )
            .field(
                "require_decision_for_approved_or_denied",
                &self.require_decision_for_approved_or_denied,
            )
            .finish()
    }
}

/// Bounded result for `SideEffect` approval authority linkage validation.
///
/// Counts intentionally avoid exposing approval IDs, `SideEffect` IDs, run IDs,
/// target references, reasons, summaries, or other caller-supplied values.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct SideEffectApprovalLinkageResult {
    side_effect_records: usize,
    approval_references: usize,
    linked_approval_references: usize,
    duplicate_approval_references: usize,
}

/// Bounded result for store-backed `SideEffect` approval authority linkage.
///
/// Counts intentionally avoid exposing store paths, approval IDs,
/// `SideEffect` IDs, run IDs, target references, reasons, summaries, or other
/// caller-supplied values.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct SideEffectApprovalLinkageFromStoreResult {
    explicit_side_effect_ids: usize,
    loaded_side_effect_records: usize,
    missing_side_effect_records: usize,
    duplicate_side_effect_ids: usize,
    approval_linkage: SideEffectApprovalLinkageResult,
}

impl SideEffectApprovalLinkageFromStoreResult {
    /// Returns the explicit side-effect ID count supplied by the caller.
    #[must_use]
    pub const fn explicit_side_effect_id_count(&self) -> usize {
        self.explicit_side_effect_ids
    }

    /// Returns the unique loaded side-effect record count.
    #[must_use]
    pub const fn loaded_side_effect_record_count(&self) -> usize {
        self.loaded_side_effect_records
    }

    /// Returns the explicit side-effect ID count that did not resolve.
    #[must_use]
    pub const fn missing_side_effect_record_count(&self) -> usize {
        self.missing_side_effect_records
    }

    /// Returns the duplicate explicit side-effect ID count.
    #[must_use]
    pub const fn duplicate_side_effect_id_count(&self) -> usize {
        self.duplicate_side_effect_ids
    }

    /// Returns the `SideEffect` record count inspected by approval linkage.
    #[must_use]
    pub const fn approval_linkage_side_effect_record_count(&self) -> usize {
        self.approval_linkage.side_effect_record_count()
    }

    /// Returns the approval reference count inspected by approval linkage.
    #[must_use]
    pub const fn approval_reference_count(&self) -> usize {
        self.approval_linkage.approval_reference_count()
    }

    /// Returns the approval reference count resolved by approval linkage.
    #[must_use]
    pub const fn linked_approval_reference_count(&self) -> usize {
        self.approval_linkage.linked_approval_reference_count()
    }

    /// Returns the duplicate approval reference count from approval linkage.
    #[must_use]
    pub const fn duplicate_approval_reference_count(&self) -> usize {
        self.approval_linkage.duplicate_approval_reference_count()
    }
}

impl fmt::Debug for SideEffectApprovalLinkageFromStoreResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SideEffectApprovalLinkageFromStoreResult")
            .field(
                "explicit_side_effect_id_count",
                &self.explicit_side_effect_ids,
            )
            .field(
                "loaded_side_effect_record_count",
                &self.loaded_side_effect_records,
            )
            .field(
                "missing_side_effect_record_count",
                &self.missing_side_effect_records,
            )
            .field(
                "duplicate_side_effect_id_count",
                &self.duplicate_side_effect_ids,
            )
            .field(
                "approval_linkage_side_effect_record_count",
                &self.approval_linkage.side_effect_record_count(),
            )
            .field(
                "approval_reference_count",
                &self.approval_linkage.approval_reference_count(),
            )
            .field(
                "linked_approval_reference_count",
                &self.approval_linkage.linked_approval_reference_count(),
            )
            .field(
                "duplicate_approval_reference_count",
                &self.approval_linkage.duplicate_approval_reference_count(),
            )
            .finish()
    }
}

impl SideEffectApprovalLinkageResult {
    /// Returns the `SideEffect` record count inspected.
    #[must_use]
    pub const fn side_effect_record_count(&self) -> usize {
        self.side_effect_records
    }

    /// Returns the total approval reference count inspected.
    #[must_use]
    pub const fn approval_reference_count(&self) -> usize {
        self.approval_references
    }

    /// Returns the count of approval references that resolved to run events.
    #[must_use]
    pub const fn linked_approval_reference_count(&self) -> usize {
        self.linked_approval_references
    }

    /// Returns duplicate approval references across all inspected records.
    #[must_use]
    pub const fn duplicate_approval_reference_count(&self) -> usize {
        self.duplicate_approval_references
    }
}

impl fmt::Debug for SideEffectApprovalLinkageResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SideEffectApprovalLinkageResult")
            .field("side_effect_record_count", &self.side_effect_records)
            .field("approval_reference_count", &self.approval_references)
            .field(
                "linked_approval_reference_count",
                &self.linked_approval_references,
            )
            .field(
                "duplicate_approval_reference_count",
                &self.duplicate_approval_references,
            )
            .finish()
    }
}

impl SideEffectRecord {
    /// Creates a validated side-effect record.
    ///
    /// # Errors
    ///
    /// Returns an error when required fields, lifecycle/authority compatibility,
    /// references, bounded text, or redaction metadata are invalid.
    pub fn new(definition: SideEffectRecordDefinition) -> Result<Self, WorkflowOsError> {
        let record = Self {
            side_effect_id: definition.side_effect_id,
            lifecycle_state: definition.lifecycle_state,
            target: definition.target,
            capability: definition.capability,
            authority: definition.authority,
            actor: definition.actor,
            system_actor: definition.system_actor,
            workflow_id: definition.workflow_id,
            workflow_version: definition.workflow_version,
            schema_version: definition.schema_version,
            spec_hash: definition.spec_hash,
            run_id: definition.run_id,
            step_id: definition.step_id,
            skill_id: definition.skill_id,
            skill_version: definition.skill_version,
            adapter_id: definition.adapter_id,
            adapter_kind: definition.adapter_kind,
            integration_id: definition.integration_id,
            idempotency: definition.idempotency,
            references: definition.references,
            outcome_reference: definition.outcome_reference,
            created_at: definition.created_at,
            updated_at: definition.updated_at,
            correlation_id: definition.correlation_id,
            summary: definition.summary,
            reason_codes: definition.reason_codes,
            sensitivity: definition.sensitivity,
            redaction: definition.redaction,
        };
        record.validate()?;
        Ok(record)
    }

    /// Validates the side-effect record.
    ///
    /// # Errors
    ///
    /// Returns an error when the record violates model invariants.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        self.target.validate()?;
        self.authority.validate()?;
        self.idempotency.validate()?;
        validate_references(&self.references)?;
        validate_actor_boundary(self.actor.as_ref(), self.system_actor.as_ref())?;
        validate_lifecycle_authority(self.lifecycle_state, self.authority.decision)?;
        validate_lifecycle_capability(self.lifecycle_state, self.capability)?;
        validate_lifecycle_outcome(
            self.lifecycle_state,
            self.outcome_reference.as_ref(),
            &self.reason_codes,
        )?;
        if let Some(outcome_reference) = &self.outcome_reference {
            outcome_reference.validate()?;
        }
        if let Some(summary) = &self.summary {
            validate_summary("side-effect summary", summary)?;
        }
        validate_reason_codes(&self.reason_codes)?;
        validate_redaction_metadata(&self.redaction)
    }

    /// Returns the side-effect ID.
    #[must_use]
    pub const fn side_effect_id(&self) -> &SideEffectId {
        &self.side_effect_id
    }

    /// Returns the lifecycle state.
    #[must_use]
    pub const fn lifecycle_state(&self) -> SideEffectLifecycleState {
        self.lifecycle_state
    }

    /// Returns the target reference.
    #[must_use]
    pub const fn target(&self) -> &SideEffectTargetReference {
        &self.target
    }

    /// Returns the workflow ID associated with the record.
    #[must_use]
    pub const fn workflow_id(&self) -> &WorkflowId {
        &self.workflow_id
    }

    /// Returns the workflow version associated with the record.
    #[must_use]
    pub const fn workflow_version(&self) -> &WorkflowVersion {
        &self.workflow_version
    }

    /// Returns the schema version associated with the record.
    #[must_use]
    pub const fn schema_version(&self) -> &SchemaVersion {
        &self.schema_version
    }

    /// Returns the workflow spec content hash associated with the record.
    #[must_use]
    pub const fn spec_hash(&self) -> &SpecContentHash {
        &self.spec_hash
    }

    /// Returns the workflow run ID associated with the record.
    #[must_use]
    pub const fn run_id(&self) -> &WorkflowRunId {
        &self.run_id
    }

    /// Returns the optional step ID.
    #[must_use]
    pub const fn step_id(&self) -> Option<&StepId> {
        self.step_id.as_ref()
    }

    /// Returns the optional skill ID.
    #[must_use]
    pub const fn skill_id(&self) -> Option<&SkillId> {
        self.skill_id.as_ref()
    }

    /// Returns the optional skill version.
    #[must_use]
    pub const fn skill_version(&self) -> Option<&SkillVersion> {
        self.skill_version.as_ref()
    }

    /// Returns the requested capability.
    #[must_use]
    pub const fn capability(&self) -> SideEffectCapability {
        self.capability
    }

    /// Returns the idempotency binding.
    #[must_use]
    pub const fn idempotency(&self) -> &SideEffectIdempotencyBinding {
        &self.idempotency
    }

    /// Returns the authority context.
    #[must_use]
    pub const fn authority(&self) -> &SideEffectAuthority {
        &self.authority
    }

    /// Returns the creation timestamp.
    #[must_use]
    pub const fn created_at(&self) -> Timestamp {
        self.created_at
    }

    /// Returns the optional correlation ID.
    #[must_use]
    pub const fn correlation_id(&self) -> Option<&CorrelationId> {
        self.correlation_id.as_ref()
    }

    /// Returns stable related references.
    #[must_use]
    pub fn references(&self) -> &[SideEffectReference] {
        &self.references
    }

    /// Returns the outcome reference where available.
    #[must_use]
    pub const fn outcome_reference(&self) -> Option<&SideEffectOutcomeReference> {
        self.outcome_reference.as_ref()
    }

    /// Returns the bounded summary.
    #[must_use]
    pub fn summary(&self) -> Option<&str> {
        self.summary.as_deref()
    }

    /// Returns stable reason codes.
    #[must_use]
    pub fn reason_codes(&self) -> &[String] {
        &self.reason_codes
    }

    /// Returns sensitivity classification.
    #[must_use]
    pub const fn sensitivity(&self) -> SideEffectSensitivity {
        self.sensitivity
    }

    /// Returns redaction metadata.
    #[must_use]
    pub const fn redaction(&self) -> &RedactionMetadata {
        &self.redaction
    }
}

/// Transitions a proposed side-effect record to attempted.
///
/// This pure helper returns a validated transitioned `SideEffectRecord` and a
/// reference-only workflow event payload for a caller to append later. It does
/// not call providers, append events, write stores, or execute a side effect.
///
/// # Errors
///
/// Returns a stable non-leaking error when the prior record is invalid, the
/// prior lifecycle state is not `Proposed`, authority is not allowed,
/// references are invalid, or the transitioned record/event is invalid.
pub fn transition_side_effect_to_attempted(
    input: SideEffectAttemptTransitionInput<'_>,
) -> Result<SideEffectLifecycleTransitionResult, WorkflowOsError> {
    input
        .prior_record
        .validate()
        .map_err(|_| transition_error("prior_invalid", "prior SideEffect record is invalid"))?;
    require_prior_lifecycle(
        input.prior_record,
        SideEffectLifecycleState::Proposed,
        "attempted transitions require a proposed prior record",
    )?;

    transition_record(
        input.prior_record,
        SideEffectLifecycleState::Attempted,
        input.transitioned_at,
        None,
        Vec::new(),
        input.summary,
        input.additional_references,
        input.evidence_reference_count,
    )
}

/// Transitions an attempted side-effect record to completed.
///
/// This pure helper returns a validated transitioned `SideEffectRecord` and a
/// reference-only workflow event payload for a caller to append later. It does
/// not call providers, append events, write stores, or execute a side effect.
///
/// # Errors
///
/// Returns a stable non-leaking error when the prior record is invalid, the
/// prior lifecycle state is not `Attempted`, the outcome reference is invalid,
/// references are invalid, or the transitioned record/event is invalid.
pub fn transition_side_effect_to_completed(
    input: SideEffectCompleteTransitionInput<'_>,
) -> Result<SideEffectLifecycleTransitionResult, WorkflowOsError> {
    input
        .prior_record
        .validate()
        .map_err(|_| transition_error("prior_invalid", "prior SideEffect record is invalid"))?;
    require_prior_lifecycle(
        input.prior_record,
        SideEffectLifecycleState::Attempted,
        "completed transitions require an attempted prior record",
    )?;
    input.outcome_reference.validate()?;

    transition_record(
        input.prior_record,
        SideEffectLifecycleState::Completed,
        input.transitioned_at,
        Some(input.outcome_reference),
        Vec::new(),
        input.summary,
        input.additional_references,
        input.evidence_reference_count,
    )
}

/// Transitions an attempted side-effect record to failed.
///
/// This pure helper returns a validated transitioned `SideEffectRecord` and a
/// reference-only workflow event payload for a caller to append later. It does
/// not call providers, append events, write stores, or execute a side effect.
///
/// # Errors
///
/// Returns a stable non-leaking error when the prior record is invalid, the
/// prior lifecycle state is not `Attempted`, the failure reference/reasons are
/// invalid or missing, references are invalid, or the transitioned record/event
/// is invalid.
pub fn transition_side_effect_to_failed(
    input: SideEffectFailTransitionInput<'_>,
) -> Result<SideEffectLifecycleTransitionResult, WorkflowOsError> {
    input
        .prior_record
        .validate()
        .map_err(|_| transition_error("prior_invalid", "prior SideEffect record is invalid"))?;
    require_prior_lifecycle(
        input.prior_record,
        SideEffectLifecycleState::Attempted,
        "failed transitions require an attempted prior record",
    )?;
    if let Some(outcome_reference) = &input.outcome_reference {
        outcome_reference.validate()?;
    }

    transition_record(
        input.prior_record,
        SideEffectLifecycleState::Failed,
        input.transitioned_at,
        input.outcome_reference,
        input.reason_codes,
        input.summary,
        input.additional_references,
        input.evidence_reference_count,
    )
}

/// Loads a proposed side-effect record from a store, transitions it to attempted,
/// writes the transitioned record, and returns the transitioned record plus a
/// reference-only workflow event payload.
///
/// This helper does not call providers, append workflow events, mutate workflow
/// runs, emit audit records, write report artifacts, or expose CLI behavior.
///
/// # Errors
///
/// Returns a stable non-leaking error when the prior record is missing,
/// unreadable, invalid, in the wrong lifecycle state, rejected by transition
/// validation, or cannot be durably updated in the store.
pub fn transition_side_effect_to_attempted_in_store(
    store: &impl SideEffectRecordStore,
    input: SideEffectAttemptTransitionStoreInput<'_>,
) -> Result<SideEffectLifecycleTransitionResult, WorkflowOsError> {
    let prior_record = load_transition_prior(store, input.side_effect_id)?;
    let result = transition_side_effect_to_attempted(SideEffectAttemptTransitionInput {
        prior_record: &prior_record,
        transitioned_at: input.transitioned_at,
        summary: input.summary,
        additional_references: input.additional_references,
        evidence_reference_count: input.evidence_reference_count,
    })?;
    store_transition_record(store, result.record())?;
    Ok(result)
}

/// Loads an attempted side-effect record from a store, transitions it to completed,
/// writes the transitioned record, and returns the transitioned record plus a
/// reference-only workflow event payload.
///
/// This helper does not call providers, append workflow events, mutate workflow
/// runs, emit audit records, write report artifacts, or expose CLI behavior.
///
/// # Errors
///
/// Returns a stable non-leaking error when the prior record is missing,
/// unreadable, invalid, in the wrong lifecycle state, rejected by transition
/// validation, or cannot be durably updated in the store.
pub fn transition_side_effect_to_completed_in_store(
    store: &impl SideEffectRecordStore,
    input: SideEffectCompleteTransitionStoreInput<'_>,
) -> Result<SideEffectLifecycleTransitionResult, WorkflowOsError> {
    let prior_record = load_transition_prior(store, input.side_effect_id)?;
    let result = transition_side_effect_to_completed(SideEffectCompleteTransitionInput {
        prior_record: &prior_record,
        transitioned_at: input.transitioned_at,
        outcome_reference: input.outcome_reference,
        summary: input.summary,
        additional_references: input.additional_references,
        evidence_reference_count: input.evidence_reference_count,
    })?;
    store_transition_record(store, result.record())?;
    Ok(result)
}

/// Loads an attempted side-effect record from a store, transitions it to failed,
/// writes the transitioned record, and returns the transitioned record plus a
/// reference-only workflow event payload.
///
/// This helper does not call providers, append workflow events, mutate workflow
/// runs, emit audit records, write report artifacts, or expose CLI behavior.
///
/// # Errors
///
/// Returns a stable non-leaking error when the prior record is missing,
/// unreadable, invalid, in the wrong lifecycle state, rejected by transition
/// validation, or cannot be durably updated in the store.
pub fn transition_side_effect_to_failed_in_store(
    store: &impl SideEffectRecordStore,
    input: SideEffectFailTransitionStoreInput<'_>,
) -> Result<SideEffectLifecycleTransitionResult, WorkflowOsError> {
    let prior_record = load_transition_prior(store, input.side_effect_id)?;
    let result = transition_side_effect_to_failed(SideEffectFailTransitionInput {
        prior_record: &prior_record,
        transitioned_at: input.transitioned_at,
        outcome_reference: input.outcome_reference,
        reason_codes: input.reason_codes,
        summary: input.summary,
        additional_references: input.additional_references,
        evidence_reference_count: input.evidence_reference_count,
    })?;
    store_transition_record(store, result.record())?;
    Ok(result)
}

/// Validates `SideEffect` approval authority references against workflow approval events.
///
/// This is a validation-only linkage helper. It does not mutate the run,
/// append events, create approval decisions, create `SideEffect` records, write
/// artifacts, call adapters, or execute side effects.
///
/// # Errors
///
/// Returns a stable, non-leaking error when the run cannot be trusted, a
/// record is invalid, identities do not match, approval references are missing,
/// decision state does not match authority, or step/skill linkage is
/// inconsistent.
pub fn validate_side_effect_approval_linkage(
    input: SideEffectApprovalLinkageInput<'_>,
) -> Result<SideEffectApprovalLinkageResult, WorkflowOsError> {
    validate_linkage_run(input.run)?;
    let approvals = approval_index(input.run)?;
    let identity = &input.run.snapshot.identity;
    let mut seen_references = BTreeSet::new();
    let mut duplicate_approval_references = 0usize;
    let mut approval_references = 0usize;
    let mut linked_approval_references = 0usize;

    for record in input.side_effect_records {
        record.validate().map_err(|_| {
            approval_linkage_error("record_invalid", "SideEffect record is invalid")
        })?;
        validate_record_matches_run(record, identity)?;

        let references = &record.authority.approval_references;
        if references.is_empty() {
            match record.authority.decision {
                SideEffectAuthorityDecision::RequiresApproval
                    if input.require_approval_references_for_requires_approval =>
                {
                    return Err(approval_linkage_error(
                        "approval_missing",
                        "required approval reference is missing",
                    ));
                }
                SideEffectAuthorityDecision::ApprovedByHuman
                | SideEffectAuthorityDecision::DeniedByApproval
                    if input.require_decision_for_approved_or_denied =>
                {
                    return Err(approval_linkage_error(
                        "decision_missing",
                        "required approval decision reference is missing",
                    ));
                }
                _ => {}
            }
        }

        for reference in references {
            approval_references += 1;
            if !seen_references.insert((reference.kind(), reference.reference().to_owned())) {
                duplicate_approval_references += 1;
            }
            if reference.kind() != SideEffectReferenceKind::ApprovalDecision {
                return Err(approval_linkage_error(
                    "approval_missing",
                    "approval reference is not resolvable",
                ));
            }
            let approval = approvals.get(reference.reference()).ok_or_else(|| {
                approval_linkage_error("approval_missing", "approval reference is missing")
            })?;

            validate_approval_identity(&approval.request, identity)?;
            validate_record_matches_approval(record, &approval.request)?;
            validate_authority_matches_approval(record.authority.decision, approval)?;
            linked_approval_references += 1;
        }
    }

    Ok(SideEffectApprovalLinkageResult {
        side_effect_records: input.side_effect_records.len(),
        approval_references,
        linked_approval_references,
        duplicate_approval_references,
    })
}

/// Validates store-backed `SideEffect` approval authority references against workflow approval events.
///
/// This is an explicit composition helper. It loads already-persisted
/// `SideEffect` records from the supplied store and then delegates to
/// `validate_side_effect_approval_linkage(...)`. It does not mutate workflow
/// state, append events, write records, write artifacts, call adapters, or
/// execute side effects.
///
/// # Errors
///
/// Returns a stable, non-leaking error when no store source is selected, the
/// run cannot be trusted, store reads fail, required records are missing,
/// loaded records are corrupt, identities do not match, or approval linkage
/// validation fails.
pub fn validate_side_effect_approval_linkage_from_store(
    store: &impl SideEffectRecordStore,
    input: SideEffectApprovalLinkageFromStoreInput<'_>,
) -> Result<SideEffectApprovalLinkageFromStoreResult, WorkflowOsError> {
    if input.side_effect_ids.is_empty() && !input.load_mode.includes_all_records_for_run() {
        return Err(approval_linkage_error(
            "invalid_input",
            "no side-effect record source selected",
        ));
    }

    validate_linkage_run(input.run)?;
    let identity = &input.run.snapshot.identity;
    let mut unique_ids = BTreeSet::new();
    let mut duplicate_side_effect_ids = 0usize;
    let mut missing_side_effect_records = 0usize;
    let mut records = BTreeMap::<SideEffectId, SideEffectRecord>::new();

    if input.load_mode.includes_explicit_ids() {
        for side_effect_id in input.side_effect_ids {
            if !unique_ids.insert(side_effect_id.clone()) {
                duplicate_side_effect_ids += 1;
                continue;
            }

            match store
                .read_side_effect_record(side_effect_id)
                .map_err(|_| approval_linkage_store_error())?
            {
                Some(record) => {
                    validate_store_loaded_side_effect_record(&record)?;
                    records.insert(record.side_effect_id().clone(), record);
                }
                None if input.missing_record_policy.requires_all() => {
                    return Err(approval_linkage_error(
                        "record_missing",
                        "required side-effect record is missing",
                    ));
                }
                None => {
                    missing_side_effect_records += 1;
                }
            }
        }
    }

    if input.load_mode.includes_all_records_for_run() {
        for record in store
            .list_side_effect_records_for_workflow_run(&identity.workflow_id, &identity.run_id)
            .map_err(|_| approval_linkage_store_error())?
        {
            validate_store_loaded_side_effect_record(&record)?;
            records.insert(record.side_effect_id().clone(), record);
        }
    }

    let side_effect_records = records.into_values().collect::<Vec<_>>();
    let approval_linkage = validate_side_effect_approval_linkage(SideEffectApprovalLinkageInput {
        run: input.run,
        side_effect_records: &side_effect_records,
        require_approval_references_for_requires_approval: input
            .require_approval_references_for_requires_approval,
        require_decision_for_approved_or_denied: input.require_decision_for_approved_or_denied,
    })?;

    Ok(SideEffectApprovalLinkageFromStoreResult {
        explicit_side_effect_ids: input.side_effect_ids.len(),
        loaded_side_effect_records: side_effect_records.len(),
        missing_side_effect_records,
        duplicate_side_effect_ids,
        approval_linkage,
    })
}

impl fmt::Debug for SideEffectRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SideEffectRecord")
            .field("side_effect_id", &self.side_effect_id)
            .field("lifecycle_state", &self.lifecycle_state)
            .field("target", &self.target)
            .field("capability", &self.capability)
            .field("authority_decision", &self.authority.decision)
            .field("actor", &self.actor.as_ref().map(|_| "[REDACTED]"))
            .field(
                "system_actor",
                &self.system_actor.as_ref().map(|_| "[REDACTED]"),
            )
            .field("workflow_id", &"[REDACTED]")
            .field("workflow_version", &"[REDACTED]")
            .field("schema_version", &self.schema_version)
            .field("spec_hash", &"[REDACTED]")
            .field("run_id", &"[REDACTED]")
            .field("step_id", &self.step_id.as_ref().map(|_| "[REDACTED]"))
            .field("skill_id", &self.skill_id.as_ref().map(|_| "[REDACTED]"))
            .field(
                "skill_version",
                &self.skill_version.as_ref().map(|_| "[REDACTED]"),
            )
            .field(
                "adapter_id",
                &self.adapter_id.as_ref().map(|_| "[REDACTED]"),
            )
            .field("adapter_kind", &self.adapter_kind)
            .field(
                "integration_id",
                &self.integration_id.as_ref().map(|_| "[REDACTED]"),
            )
            .field("idempotency_scope", &self.idempotency.scope())
            .field("reference_count", &self.references.len())
            .field("has_outcome_reference", &self.outcome_reference.is_some())
            .field("created_at", &self.created_at)
            .field("updated_at", &self.updated_at)
            .field(
                "correlation_id",
                &self.correlation_id.as_ref().map(|_| "[REDACTED]"),
            )
            .field("has_summary", &self.summary.is_some())
            .field("reason_code_count", &self.reason_codes.len())
            .field("sensitivity", &self.sensitivity)
            .field(
                "redaction",
                &RedactedRedactionMetadataDebug(&self.redaction),
            )
            .finish()
    }
}

struct ApprovalLink {
    request: ApprovalRequest,
    decision: Option<ApprovalDecisionKind>,
}

fn validate_linkage_run(run: &WorkflowRun) -> Result<(), WorkflowOsError> {
    let rehydrated = WorkflowRun::rehydrate(&run.events)
        .map_err(|_| approval_linkage_error("run_invalid", "workflow run is invalid"))?;
    if rehydrated.snapshot != run.snapshot {
        return Err(approval_linkage_error(
            "run_invalid",
            "workflow run snapshot does not match event history",
        ));
    }
    Ok(())
}

fn approval_index(run: &WorkflowRun) -> Result<BTreeMap<String, ApprovalLink>, WorkflowOsError> {
    let mut approvals = BTreeMap::<String, ApprovalLink>::new();
    for event in &run.events {
        match &event.kind {
            WorkflowRunEventKind::ApprovalRequested(request) => {
                approvals.insert(
                    request.approval_id.clone(),
                    ApprovalLink {
                        request: (**request).clone(),
                        decision: request.decision.as_ref().map(|decision| decision.decision),
                    },
                );
            }
            WorkflowRunEventKind::ApprovalGranted(decision)
            | WorkflowRunEventKind::ApprovalDenied(decision) => {
                if let Some(link) = approvals.get_mut(&decision.approval_id) {
                    link.decision = Some(decision.decision);
                } else {
                    return Err(approval_linkage_error(
                        "approval_missing",
                        "approval decision has no request",
                    ));
                }
            }
            _ => {}
        }
    }
    Ok(approvals)
}

#[allow(clippy::too_many_arguments)]
fn transition_record(
    prior_record: &SideEffectRecord,
    lifecycle_state: SideEffectLifecycleState,
    transitioned_at: Timestamp,
    outcome_reference: Option<SideEffectOutcomeReference>,
    reason_codes: Vec<String>,
    summary: Option<String>,
    additional_references: Vec<SideEffectReference>,
    evidence_reference_count: u32,
) -> Result<SideEffectLifecycleTransitionResult, WorkflowOsError> {
    let mut references = prior_record.references.clone();
    references.extend(additional_references);

    let record = SideEffectRecord::new(SideEffectRecordDefinition {
        side_effect_id: prior_record.side_effect_id.clone(),
        lifecycle_state,
        target: prior_record.target.clone(),
        capability: prior_record.capability,
        authority: prior_record.authority.clone(),
        actor: prior_record.actor.clone(),
        system_actor: prior_record.system_actor.clone(),
        workflow_id: prior_record.workflow_id.clone(),
        workflow_version: prior_record.workflow_version.clone(),
        schema_version: prior_record.schema_version.clone(),
        spec_hash: prior_record.spec_hash.clone(),
        run_id: prior_record.run_id.clone(),
        step_id: prior_record.step_id.clone(),
        skill_id: prior_record.skill_id.clone(),
        skill_version: prior_record.skill_version.clone(),
        adapter_id: prior_record.adapter_id.clone(),
        adapter_kind: prior_record.adapter_kind.clone(),
        integration_id: prior_record.integration_id.clone(),
        idempotency: prior_record.idempotency.clone(),
        references,
        outcome_reference,
        created_at: prior_record.created_at,
        updated_at: Some(transitioned_at),
        correlation_id: prior_record.correlation_id.clone(),
        summary: summary.or_else(|| prior_record.summary.clone()),
        reason_codes,
        sensitivity: prior_record.sensitivity,
        redaction: prior_record.redaction.clone(),
    })?;

    let event = transition_event(&record, evidence_reference_count)?;

    Ok(SideEffectLifecycleTransitionResult { record, event })
}

fn transition_event(
    record: &SideEffectRecord,
    evidence_reference_count: u32,
) -> Result<SideEffectWorkflowEvent, WorkflowOsError> {
    SideEffectWorkflowEvent::new(SideEffectWorkflowEventDefinition {
        side_effect_id: record.side_effect_id.clone(),
        lifecycle_state: record.lifecycle_state,
        step_id: record.step_id.clone(),
        skill_id: record.skill_id.clone(),
        skill_version: record.skill_version.clone(),
        correlation_id: record.correlation_id.clone(),
        references: record.references.clone(),
        evidence_reference_count,
        outcome_reference_count: u32::from(record.outcome_reference.is_some()),
        redaction: record.redaction.clone(),
        sensitivity: record.sensitivity,
    })
}

fn load_transition_prior(
    store: &impl SideEffectRecordStore,
    side_effect_id: &SideEffectId,
) -> Result<SideEffectRecord, WorkflowOsError> {
    store
        .read_side_effect_record(side_effect_id)
        .map_err(|_| {
            transition_error(
                "store_read_failed",
                "side-effect transition prior record could not be read",
            )
        })?
        .ok_or_else(|| {
            transition_error(
                "prior_missing",
                "side-effect transition prior record is missing",
            )
        })
}

fn store_transition_record(
    store: &impl SideEffectRecordStore,
    record: &SideEffectRecord,
) -> Result<(), WorkflowOsError> {
    store.update_side_effect_record(record).map_err(|_| {
        transition_error(
            "store_write_failed",
            "side-effect transition record could not be written",
        )
    })
}

fn require_prior_lifecycle(
    prior_record: &SideEffectRecord,
    expected: SideEffectLifecycleState,
    message: &'static str,
) -> Result<(), WorkflowOsError> {
    if prior_record.lifecycle_state != expected {
        return Err(transition_error("invalid_prior_state", message));
    }
    Ok(())
}

fn validate_record_matches_run(
    record: &SideEffectRecord,
    identity: &crate::WorkflowRunIdentity,
) -> Result<(), WorkflowOsError> {
    if record.workflow_id() != &identity.workflow_id
        || record.workflow_version() != &identity.workflow_version
        || record.schema_version() != &identity.schema_version
        || record.spec_hash() != &identity.spec_content_hash
        || record.run_id() != &identity.run_id
    {
        return Err(approval_linkage_error(
            "identity_mismatch",
            "SideEffect record identity does not match workflow run",
        ));
    }
    Ok(())
}

fn validate_approval_identity(
    approval: &ApprovalRequest,
    identity: &crate::WorkflowRunIdentity,
) -> Result<(), WorkflowOsError> {
    if approval.workflow_id != identity.workflow_id
        || approval.workflow_version != identity.workflow_version
        || approval.schema_version != identity.schema_version
        || approval.spec_content_hash != identity.spec_content_hash
        || approval.run_id != identity.run_id
    {
        return Err(approval_linkage_error(
            "identity_mismatch",
            "approval identity does not match workflow run",
        ));
    }
    Ok(())
}

fn validate_record_matches_approval(
    record: &SideEffectRecord,
    approval: &ApprovalRequest,
) -> Result<(), WorkflowOsError> {
    if let Some(step_id) = &record.step_id {
        if step_id != &approval.step_id {
            return Err(approval_linkage_error(
                "step_mismatch",
                "SideEffect step does not match approval request",
            ));
        }
    }
    if let Some(skill_id) = &record.skill_id {
        if skill_id != &approval.skill_id {
            return Err(approval_linkage_error(
                "skill_mismatch",
                "SideEffect skill does not match approval request",
            ));
        }
    }
    if let Some(skill_version) = &record.skill_version {
        if skill_version != &approval.skill_version {
            return Err(approval_linkage_error(
                "skill_mismatch",
                "SideEffect skill version does not match approval request",
            ));
        }
    }
    Ok(())
}

fn validate_authority_matches_approval(
    decision: SideEffectAuthorityDecision,
    approval: &ApprovalLink,
) -> Result<(), WorkflowOsError> {
    match decision {
        SideEffectAuthorityDecision::ApprovedByHuman => match approval.decision {
            Some(ApprovalDecisionKind::Granted) => Ok(()),
            Some(ApprovalDecisionKind::Denied) => Err(approval_linkage_error(
                "decision_kind_mismatch",
                "approval decision does not match SideEffect authority",
            )),
            None => Err(approval_linkage_error(
                "decision_missing",
                "approval decision is missing",
            )),
        },
        SideEffectAuthorityDecision::DeniedByApproval => match approval.decision {
            Some(ApprovalDecisionKind::Denied) => Ok(()),
            Some(ApprovalDecisionKind::Granted) => Err(approval_linkage_error(
                "decision_kind_mismatch",
                "approval decision does not match SideEffect authority",
            )),
            None => Err(approval_linkage_error(
                "decision_missing",
                "approval decision is missing",
            )),
        },
        _ => Ok(()),
    }
}

impl<'de> Deserialize<'de> for SideEffectRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            side_effect_id: SideEffectId,
            lifecycle_state: SideEffectLifecycleState,
            target: SideEffectTargetReference,
            capability: SideEffectCapability,
            authority: SideEffectAuthority,
            actor: Option<ActorId>,
            system_actor: Option<ActorId>,
            workflow_id: WorkflowId,
            workflow_version: WorkflowVersion,
            schema_version: SchemaVersion,
            spec_hash: SpecContentHash,
            run_id: WorkflowRunId,
            step_id: Option<StepId>,
            skill_id: Option<SkillId>,
            skill_version: Option<SkillVersion>,
            adapter_id: Option<AdapterId>,
            adapter_kind: Option<AdapterKind>,
            integration_id: Option<IntegrationId>,
            idempotency: SideEffectIdempotencyBinding,
            references: Vec<SideEffectReference>,
            outcome_reference: Option<SideEffectOutcomeReference>,
            created_at: Timestamp,
            updated_at: Option<Timestamp>,
            correlation_id: Option<CorrelationId>,
            summary: Option<String>,
            reason_codes: Vec<String>,
            sensitivity: SideEffectSensitivity,
            redaction: RedactionMetadata,
        }

        let wire = Wire::deserialize(deserializer)?;
        Self::new(SideEffectRecordDefinition {
            side_effect_id: wire.side_effect_id,
            lifecycle_state: wire.lifecycle_state,
            target: wire.target,
            capability: wire.capability,
            authority: wire.authority,
            actor: wire.actor,
            system_actor: wire.system_actor,
            workflow_id: wire.workflow_id,
            workflow_version: wire.workflow_version,
            schema_version: wire.schema_version,
            spec_hash: wire.spec_hash,
            run_id: wire.run_id,
            step_id: wire.step_id,
            skill_id: wire.skill_id,
            skill_version: wire.skill_version,
            adapter_id: wire.adapter_id,
            adapter_kind: wire.adapter_kind,
            integration_id: wire.integration_id,
            idempotency: wire.idempotency,
            references: wire.references,
            outcome_reference: wire.outcome_reference,
            created_at: wire.created_at,
            updated_at: wire.updated_at,
            correlation_id: wire.correlation_id,
            summary: wire.summary,
            reason_codes: wire.reason_codes,
            sensitivity: wire.sensitivity,
            redaction: wire.redaction,
        })
        .map_err(serde::de::Error::custom)
    }
}

fn validate_actor_boundary(
    actor: Option<&ActorId>,
    system_actor: Option<&ActorId>,
) -> Result<(), WorkflowOsError> {
    if actor.is_none() && system_actor.is_none() {
        return Err(validation_error(
            "side_effect.actor.required",
            "side-effect records require an actor or system actor",
        ));
    }
    Ok(())
}

fn validate_lifecycle_authority(
    lifecycle_state: SideEffectLifecycleState,
    decision: SideEffectAuthorityDecision,
) -> Result<(), WorkflowOsError> {
    match lifecycle_state {
        SideEffectLifecycleState::Attempted
        | SideEffectLifecycleState::Completed
        | SideEffectLifecycleState::Failed => {
            if !decision.allows_attempt() {
                return Err(validation_error(
                    "side_effect.authority.not_allowed",
                    "attempted, completed, and failed side-effect records require allowed authority",
                ));
            }
        }
        SideEffectLifecycleState::Denied => {
            if !decision.denies_or_unsupported() {
                return Err(validation_error(
                    "side_effect.authority.denied_required",
                    "denied side-effect records require denied or unsupported authority",
                ));
            }
        }
        SideEffectLifecycleState::Proposed | SideEffectLifecycleState::Skipped => {}
    }
    Ok(())
}

fn validate_lifecycle_capability(
    lifecycle_state: SideEffectLifecycleState,
    capability: SideEffectCapability,
) -> Result<(), WorkflowOsError> {
    if matches!(
        lifecycle_state,
        SideEffectLifecycleState::Attempted
            | SideEffectLifecycleState::Completed
            | SideEffectLifecycleState::Failed
    ) && capability == SideEffectCapability::Unknown
    {
        return Err(validation_error(
            "side_effect.capability.unknown",
            "unknown side-effect capability cannot be attempted, completed, or failed",
        ));
    }
    Ok(())
}

fn validate_lifecycle_outcome(
    lifecycle_state: SideEffectLifecycleState,
    outcome_reference: Option<&SideEffectOutcomeReference>,
    reason_codes: &[String],
) -> Result<(), WorkflowOsError> {
    match lifecycle_state {
        SideEffectLifecycleState::Completed => {
            if outcome_reference.is_none() {
                return Err(validation_error(
                    "side_effect.outcome.required",
                    "completed side-effect records require an outcome reference",
                ));
            }
        }
        SideEffectLifecycleState::Denied | SideEffectLifecycleState::Skipped => {
            if reason_codes.is_empty() {
                return Err(validation_error(
                    "side_effect.reason.required",
                    "denied and skipped side-effect records require stable reason codes",
                ));
            }
        }
        SideEffectLifecycleState::Failed => {
            if outcome_reference.is_none() && reason_codes.is_empty() {
                return Err(validation_error(
                    "side_effect.failure_reference.required",
                    "failed side-effect records require a failure reference or stable reason code",
                ));
            }
        }
        SideEffectLifecycleState::Proposed | SideEffectLifecycleState::Attempted => {}
    }
    Ok(())
}

fn validate_references(references: &[SideEffectReference]) -> Result<(), WorkflowOsError> {
    if references.len() > SIDE_EFFECT_REFERENCE_MAX_COUNT {
        return Err(validation_error(
            "side_effect.reference.too_many",
            "side-effect records include too many references",
        ));
    }

    let mut seen = BTreeSet::new();
    for reference in references {
        reference.validate()?;
        if !seen.insert((reference.kind(), reference.reference().to_owned())) {
            return Err(validation_error(
                "side_effect.reference.duplicate",
                "side-effect records cannot repeat references",
            ));
        }
    }
    Ok(())
}

fn validate_reason_codes(reason_codes: &[String]) -> Result<(), WorkflowOsError> {
    if reason_codes.len() > SIDE_EFFECT_REASON_MAX_COUNT {
        return Err(validation_error(
            "side_effect.reason.too_many",
            "side-effect records include too many reason codes",
        ));
    }

    let mut seen = BTreeSet::new();
    for reason_code in reason_codes {
        validate_reason_code(reason_code)?;
        if !seen.insert(reason_code) {
            return Err(validation_error(
                "side_effect.reason.duplicate",
                "side-effect records cannot repeat reason codes",
            ));
        }
    }
    Ok(())
}

fn validate_identifier(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "side_effect.identifier.empty",
            format!("{type_name} cannot be empty"),
        ));
    }

    if value.len() > SIDE_EFFECT_ID_MAX_BYTES {
        return Err(validation_error(
            "side_effect.identifier.too_long",
            format!("{type_name} cannot exceed {SIDE_EFFECT_ID_MAX_BYTES} bytes"),
        ));
    }

    let is_valid = value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/'));

    if !is_valid {
        return Err(validation_error(
            "side_effect.identifier.invalid_character",
            format!("{type_name} contains an invalid character"),
        ));
    }

    validate_not_secret_like(type_name, value)
}

fn validate_reference(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "side_effect.reference.empty",
            format!("{type_name} cannot be empty"),
        ));
    }

    if value.len() > SIDE_EFFECT_REFERENCE_MAX_BYTES {
        return Err(validation_error(
            "side_effect.reference.too_long",
            format!("{type_name} cannot exceed {SIDE_EFFECT_REFERENCE_MAX_BYTES} bytes"),
        ));
    }

    validate_not_secret_like(type_name, value)
}

fn validate_summary(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "side_effect.summary.empty",
            format!("{type_name} cannot be empty"),
        ));
    }

    if value.len() > SIDE_EFFECT_SUMMARY_MAX_BYTES {
        return Err(validation_error(
            "side_effect.summary.too_long",
            format!("{type_name} cannot exceed {SIDE_EFFECT_SUMMARY_MAX_BYTES} bytes"),
        ));
    }

    validate_not_secret_like(type_name, value)
}

fn validate_reason_code(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "side_effect.reason.empty",
            "side-effect reason code cannot be empty",
        ));
    }

    if value.len() > SIDE_EFFECT_REASON_MAX_BYTES {
        return Err(validation_error(
            "side_effect.reason.too_long",
            format!("side-effect reason code cannot exceed {SIDE_EFFECT_REASON_MAX_BYTES} bytes"),
        ));
    }

    validate_identifier("side-effect reason code", value)
}

fn validate_redaction_metadata(redaction: &RedactionMetadata) -> Result<(), WorkflowOsError> {
    if redaction.redacted_fields.len() > SIDE_EFFECT_REDACTION_MAX_ENTRIES {
        return Err(validation_error(
            "side_effect.redaction.too_many_fields",
            "side-effect redaction metadata contains too many fields",
        ));
    }

    if redaction.field_states.len() > SIDE_EFFECT_REDACTION_MAX_ENTRIES {
        return Err(validation_error(
            "side_effect.redaction.too_many_states",
            "side-effect redaction metadata contains too many field states",
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
            "side_effect.redaction.field.empty",
            "side-effect redaction field cannot be empty",
        ));
    }

    if value.len() > SIDE_EFFECT_REDACTION_FIELD_MAX_BYTES {
        return Err(validation_error(
            "side_effect.redaction.field.too_long",
            format!(
                "side-effect redaction field cannot exceed {SIDE_EFFECT_REDACTION_FIELD_MAX_BYTES} bytes"
            ),
        ));
    }

    validate_not_secret_like("side-effect redaction field", value)
}

fn validate_redaction_reason(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "side_effect.redaction.reason.empty",
            "side-effect redaction reason cannot be empty",
        ));
    }

    if value.len() > SIDE_EFFECT_REDACTION_REASON_MAX_BYTES {
        return Err(validation_error(
            "side_effect.redaction.reason.too_long",
            format!(
                "side-effect redaction reason cannot exceed {SIDE_EFFECT_REDACTION_REASON_MAX_BYTES} bytes"
            ),
        ));
    }

    validate_not_secret_like("side-effect redaction reason", value)
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
            "side_effect.secret_like_value",
            format!("{type_name} contains sensitive-looking text"),
        ));
    }

    Ok(())
}

fn validation_error(code: &'static str, message: impl Into<String>) -> WorkflowOsError {
    WorkflowOsError::validation(code, message)
}

fn approval_linkage_error(suffix: &'static str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(format!("side_effect_approval_linkage.{suffix}"), message)
}

fn approval_linkage_store_error() -> WorkflowOsError {
    approval_linkage_error("store_read_failed", "side-effect record store read failed")
}

fn transition_error(suffix: &'static str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(format!("side_effect.transition.{suffix}"), message)
}

fn validate_store_loaded_side_effect_record(
    record: &SideEffectRecord,
) -> Result<(), WorkflowOsError> {
    record.validate().map_err(|_| {
        approval_linkage_error("record_corrupt", "stored SideEffect record is invalid")
    })
}

struct RedactedRedactionMetadataDebug<'a>(&'a RedactionMetadata);

impl fmt::Debug for RedactedRedactionMetadataDebug<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RedactionMetadata")
            .field("redacted_field_count", &self.0.redacted_fields.len())
            .field("field_state_count", &self.0.field_states.len())
            .finish()
    }
}
