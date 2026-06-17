use std::collections::BTreeSet;
use std::fmt;
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    ActorId, AdapterId, AdapterKind, CorrelationId, IdempotencyKey, IntegrationId,
    RedactionMetadata, SchemaVersion, SkillId, SkillVersion, SpecContentHash, StepId, Timestamp,
    WorkflowId, WorkflowOsError, WorkflowRunId, WorkflowVersion,
};

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

    /// Returns the requested capability.
    #[must_use]
    pub const fn capability(&self) -> SideEffectCapability {
        self.capability
    }

    /// Returns the authority context.
    #[must_use]
    pub const fn authority(&self) -> &SideEffectAuthority {
        &self.authority
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
