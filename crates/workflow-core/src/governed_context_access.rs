use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    ActorId, ApprovalReferenceId, CapabilityReference, CapabilityResolution,
    CapabilityResolutionPosture, CapabilityResolutionReason, CapabilityResourceKind,
    CapabilityResourceScope, EventId, EvidenceReferenceId, HarnessContractId, RedactionMetadata,
    SideEffectId, StepId, Timestamp, TypedHandoffId, ValidationReferenceId, WorkReportId,
    WorkReportSensitivity, WorkflowId, WorkflowOsError, WorkflowRunId,
};

const REDACTION_FIELD_MAX_BYTES: usize = 128;
const REDACTION_REASON_MAX_BYTES: usize = 512;
const REDACTION_MAX_ENTRIES: usize = 64;
const REFERENCE_ONLY_CAPABILITY: &str = "context.reference.view";
const BOUNDED_METADATA_CAPABILITY: &str = "context.metadata.view";

/// Fixed first-slice kinds of stable references eligible for context projection.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernedContextReferenceKind {
    /// `EvidenceReference` identity.
    EvidenceReference,
    /// Workflow event identity.
    WorkflowEvent,
    /// Audit event identity.
    AuditEvent,
    /// Validation diagnostic identity.
    ValidationDiagnostic,
    /// Approval decision identity.
    ApprovalDecision,
    /// Policy decision event identity.
    PolicyDecision,
    /// Governed `SideEffect` identity.
    SideEffect,
    /// Typed handoff identity.
    TypedHandoff,
    /// `WorkReport` identity.
    WorkReport,
}

impl<'de> Deserialize<'de> for GovernedContextReferenceKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "evidence_reference" => Ok(Self::EvidenceReference),
            "workflow_event" => Ok(Self::WorkflowEvent),
            "audit_event" => Ok(Self::AuditEvent),
            "validation_diagnostic" => Ok(Self::ValidationDiagnostic),
            "approval_decision" => Ok(Self::ApprovalDecision),
            "policy_decision" => Ok(Self::PolicyDecision),
            "side_effect" => Ok(Self::SideEffect),
            "typed_handoff" => Ok(Self::TypedHandoff),
            "work_report" => Ok(Self::WorkReport),
            _ => Err(serde::de::Error::custom(validation_error(
                "governed_context.reference.kind_invalid",
                "governed context reference kind is invalid",
            ))),
        }
    }
}

impl GovernedContextReferenceKind {
    fn canonical_name(self) -> &'static str {
        match self {
            Self::EvidenceReference => "evidence-reference",
            Self::WorkflowEvent => "workflow-event",
            Self::AuditEvent => "audit-event",
            Self::ValidationDiagnostic => "validation-diagnostic",
            Self::ApprovalDecision => "approval-decision",
            Self::PolicyDecision => "policy-decision",
            Self::SideEffect => "side-effect",
            Self::TypedHandoff => "typed-handoff",
            Self::WorkReport => "work-report",
        }
    }
}

/// Typed stable reference target. No generic-string escape hatch is provided.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(tag = "kind", content = "id", rename_all = "snake_case")]
pub enum GovernedContextReferenceTarget {
    /// `EvidenceReference` identity.
    EvidenceReference(EvidenceReferenceId),
    /// Workflow event identity.
    WorkflowEvent(EventId),
    /// Audit event identity.
    AuditEvent(EventId),
    /// Validation diagnostic identity.
    ValidationDiagnostic(ValidationReferenceId),
    /// Approval decision identity.
    ApprovalDecision(ApprovalReferenceId),
    /// Policy decision event identity.
    PolicyDecision(EventId),
    /// Governed `SideEffect` identity.
    SideEffect(SideEffectId),
    /// Typed handoff identity.
    TypedHandoff(TypedHandoffId),
    /// `WorkReport` identity.
    WorkReport(WorkReportId),
}

impl<'de> Deserialize<'de> for GovernedContextReferenceTarget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            kind: GovernedContextReferenceKind,
            id: String,
        }

        let wire = Wire::deserialize(deserializer)?;
        let target = match wire.kind {
            GovernedContextReferenceKind::EvidenceReference => {
                EvidenceReferenceId::new(wire.id).map(Self::EvidenceReference)
            }
            GovernedContextReferenceKind::WorkflowEvent => {
                EventId::new(wire.id).map(Self::WorkflowEvent)
            }
            GovernedContextReferenceKind::AuditEvent => EventId::new(wire.id).map(Self::AuditEvent),
            GovernedContextReferenceKind::ValidationDiagnostic => {
                ValidationReferenceId::new(wire.id).map(Self::ValidationDiagnostic)
            }
            GovernedContextReferenceKind::ApprovalDecision => {
                ApprovalReferenceId::new(wire.id).map(Self::ApprovalDecision)
            }
            GovernedContextReferenceKind::PolicyDecision => {
                EventId::new(wire.id).map(Self::PolicyDecision)
            }
            GovernedContextReferenceKind::SideEffect => {
                SideEffectId::new(wire.id).map(Self::SideEffect)
            }
            GovernedContextReferenceKind::TypedHandoff => {
                TypedHandoffId::new(wire.id).map(Self::TypedHandoff)
            }
            GovernedContextReferenceKind::WorkReport => {
                WorkReportId::new(wire.id).map(Self::WorkReport)
            }
        };
        target.map_err(serde::de::Error::custom)
    }
}

impl GovernedContextReferenceTarget {
    /// Returns the typed target kind.
    #[must_use]
    pub const fn kind(&self) -> GovernedContextReferenceKind {
        match self {
            Self::EvidenceReference(_) => GovernedContextReferenceKind::EvidenceReference,
            Self::WorkflowEvent(_) => GovernedContextReferenceKind::WorkflowEvent,
            Self::AuditEvent(_) => GovernedContextReferenceKind::AuditEvent,
            Self::ValidationDiagnostic(_) => GovernedContextReferenceKind::ValidationDiagnostic,
            Self::ApprovalDecision(_) => GovernedContextReferenceKind::ApprovalDecision,
            Self::PolicyDecision(_) => GovernedContextReferenceKind::PolicyDecision,
            Self::SideEffect(_) => GovernedContextReferenceKind::SideEffect,
            Self::TypedHandoff(_) => GovernedContextReferenceKind::TypedHandoff,
            Self::WorkReport(_) => GovernedContextReferenceKind::WorkReport,
        }
    }

    fn stable_id(&self) -> &str {
        match self {
            Self::EvidenceReference(value) => value.as_str(),
            Self::WorkflowEvent(value) | Self::AuditEvent(value) | Self::PolicyDecision(value) => {
                value.as_str()
            }
            Self::ValidationDiagnostic(value) => value.as_str(),
            Self::ApprovalDecision(value) => value.as_str(),
            Self::SideEffect(value) => value.as_str(),
            Self::TypedHandoff(value) => value.as_str(),
            Self::WorkReport(value) => value.as_str(),
        }
    }
}

impl fmt::Debug for GovernedContextReferenceTarget {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GovernedContextReferenceTarget")
            .field("kind", &self.kind())
            .field("id", &"[REDACTED]")
            .finish()
    }
}

/// Current declared availability of a stable context target.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernedContextAvailability {
    /// The stable target is declared available.
    Available,
    /// The stable target is declared unavailable.
    Unavailable,
    /// Current availability is unknown.
    Unknown,
}

impl<'de> Deserialize<'de> for GovernedContextAvailability {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "available" => Ok(Self::Available),
            "unavailable" => Ok(Self::Unavailable),
            "unknown" => Ok(Self::Unknown),
            _ => Err(serde::de::Error::custom(validation_error(
                "governed_context.reference.availability_invalid",
                "governed context availability is invalid",
            ))),
        }
    }
}

/// Positive context-access level supported by the first model slice.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernedContextAccessLevel {
    /// Expose only typed stable identity.
    ReferenceOnly,
    /// Expose stable identity plus the fixed bounded metadata record.
    BoundedMetadata,
}

impl<'de> Deserialize<'de> for GovernedContextAccessLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "reference_only" => Ok(Self::ReferenceOnly),
            "bounded_metadata" => Ok(Self::BoundedMetadata),
            _ => Err(serde::de::Error::custom(validation_error(
                "governed_context.access_level.invalid",
                "governed context access level is invalid",
            ))),
        }
    }
}

impl GovernedContextAccessLevel {
    /// Returns the fixed Core-owned capability required by this access level.
    ///
    /// # Errors
    ///
    /// Returns a validation error only if the Core-owned constant is invalid.
    pub fn required_capability(self) -> Result<CapabilityReference, WorkflowOsError> {
        CapabilityReference::new(match self {
            Self::ReferenceOnly => REFERENCE_ONLY_CAPABILITY,
            Self::BoundedMetadata => BOUNDED_METADATA_CAPABILITY,
        })
    }
}

/// Validated payload-free context reference and its declared posture.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct GovernedContextReference {
    target: GovernedContextReferenceTarget,
    sensitivity: WorkReportSensitivity,
    availability: GovernedContextAvailability,
    redaction: RedactionMetadata,
}

impl GovernedContextReference {
    /// Creates a validated payload-free context reference.
    ///
    /// # Errors
    ///
    /// Returns a stable error for unknown sensitivity, secret-like identity,
    /// or unsafe redaction metadata.
    pub fn new(
        target: GovernedContextReferenceTarget,
        sensitivity: WorkReportSensitivity,
        availability: GovernedContextAvailability,
        redaction: RedactionMetadata,
    ) -> Result<Self, WorkflowOsError> {
        let value = Self {
            target,
            sensitivity,
            availability,
            redaction,
        };
        value.validate()?;
        Ok(value)
    }

    /// Validates the bounded reference posture.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking validation error.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        if self.sensitivity == WorkReportSensitivity::Unknown {
            return Err(validation_error(
                "governed_context.reference.sensitivity_unknown",
                "governed context reference requires known sensitivity",
            ));
        }
        validate_not_secret_like(self.target.stable_id())?;
        validate_redaction_metadata(&self.redaction)
    }

    /// Returns the typed stable target.
    #[must_use]
    pub const fn target(&self) -> &GovernedContextReferenceTarget {
        &self.target
    }

    /// Returns the typed target kind.
    #[must_use]
    pub const fn kind(&self) -> GovernedContextReferenceKind {
        self.target.kind()
    }

    /// Returns the declared sensitivity.
    #[must_use]
    pub const fn sensitivity(&self) -> WorkReportSensitivity {
        self.sensitivity
    }

    /// Returns the declared target availability.
    #[must_use]
    pub const fn availability(&self) -> GovernedContextAvailability {
        self.availability
    }

    /// Derives the exact Core-owned capability resource for this target.
    ///
    /// # Errors
    ///
    /// Returns a stable validation error if the derived resource is invalid.
    pub fn capability_resource(&self) -> Result<CapabilityResourceScope, WorkflowOsError> {
        CapabilityResourceScope::new(
            CapabilityResourceKind::ContextReference,
            format!(
                "{}/{}",
                self.kind().canonical_name(),
                self.target.stable_id()
            ),
        )
    }
}

impl fmt::Debug for GovernedContextReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GovernedContextReference")
            .field("target", &self.target)
            .field("sensitivity", &self.sensitivity)
            .field("availability", &self.availability)
            .field(
                "redaction",
                &RedactedRedactionMetadataDebug(&self.redaction),
            )
            .finish()
    }
}

impl<'de> Deserialize<'de> for GovernedContextReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            target: GovernedContextReferenceTarget,
            sensitivity: WorkReportSensitivity,
            availability: GovernedContextAvailability,
            redaction: RedactionMetadata,
        }

        let wire = Wire::deserialize(deserializer)?;
        Self::new(
            wire.target,
            wire.sensitivity,
            wire.availability,
            wire.redaction,
        )
        .map_err(serde::de::Error::custom)
    }
}

/// One complete evaluated target and exact source authority resolution.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct GovernedContextProjectionCandidate {
    reference: GovernedContextReference,
    availability_observed_at: Timestamp,
    requested_access_level: GovernedContextAccessLevel,
    source_resolution: CapabilityResolution,
}

impl GovernedContextProjectionCandidate {
    /// Creates a complete evaluated projection candidate.
    ///
    /// # Errors
    ///
    /// Returns a stable error when capability, resource, sensitivity, or time
    /// does not match the context reference.
    pub fn new(
        reference: GovernedContextReference,
        availability_observed_at: Timestamp,
        requested_access_level: GovernedContextAccessLevel,
        source_resolution: CapabilityResolution,
    ) -> Result<Self, WorkflowOsError> {
        let candidate = Self {
            reference,
            availability_observed_at,
            requested_access_level,
            source_resolution,
        };
        candidate.validate()?;
        Ok(candidate)
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        self.reference.validate()?;
        self.source_resolution.validate()?;
        if self.availability_observed_at > self.source_resolution.evaluated_at() {
            return Err(validation_error(
                "governed_context.candidate.availability_in_future",
                "context availability observation cannot follow authority evaluation",
            ));
        }
        let expected_capability = self.requested_access_level.required_capability()?;
        let expected_resource = self.reference.capability_resource()?;
        let context = self.source_resolution.context();
        if context.capability() != &expected_capability
            || context.resource() != &expected_resource
            || context.requested_sensitivity() != self.reference.sensitivity()
        {
            return Err(validation_error(
                "governed_context.candidate.authority_mismatch",
                "context candidate requires exact capability, resource, and sensitivity authority",
            ));
        }
        Ok(())
    }

    /// Returns the payload-free context reference.
    #[must_use]
    pub const fn reference(&self) -> &GovernedContextReference {
        &self.reference
    }

    /// Returns the requested access level.
    #[must_use]
    pub const fn requested_access_level(&self) -> GovernedContextAccessLevel {
        self.requested_access_level
    }

    /// Returns the exact source authority resolution.
    #[must_use]
    pub const fn source_resolution(&self) -> &CapabilityResolution {
        &self.source_resolution
    }
}

impl fmt::Debug for GovernedContextProjectionCandidate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GovernedContextProjectionCandidate")
            .field("reference", &self.reference)
            .field("availability_observed_at", &self.availability_observed_at)
            .field("requested_access_level", &self.requested_access_level)
            .field("source_resolution", &self.source_resolution)
            .finish()
    }
}

impl<'de> Deserialize<'de> for GovernedContextProjectionCandidate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            reference: GovernedContextReference,
            availability_observed_at: Timestamp,
            requested_access_level: GovernedContextAccessLevel,
            source_resolution: CapabilityResolution,
        }

        let wire = Wire::deserialize(deserializer)?;
        Self::new(
            wire.reference,
            wire.availability_observed_at,
            wire.requested_access_level,
            wire.source_resolution,
        )
        .map_err(serde::de::Error::custom)
    }
}

/// Fixed safe metadata exposed only for bounded-metadata access.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GovernedContextBoundedMetadata {
    target_kind: GovernedContextReferenceKind,
    declared_sensitivity: WorkReportSensitivity,
    availability_observed_at: Timestamp,
}

impl GovernedContextBoundedMetadata {
    /// Returns the typed target kind.
    #[must_use]
    pub const fn target_kind(&self) -> GovernedContextReferenceKind {
        self.target_kind
    }

    /// Returns the declared target sensitivity.
    #[must_use]
    pub const fn declared_sensitivity(&self) -> WorkReportSensitivity {
        self.declared_sensitivity
    }

    /// Returns when availability was observed.
    #[must_use]
    pub const fn availability_observed_at(&self) -> Timestamp {
        self.availability_observed_at
    }
}

/// One authorized payload-free context entry.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct GovernedContextProjectionEntry {
    reference: GovernedContextReference,
    access_level: GovernedContextAccessLevel,
    metadata: Option<GovernedContextBoundedMetadata>,
    source_resolution: CapabilityResolution,
}

impl GovernedContextProjectionEntry {
    fn validate(&self) -> Result<(), WorkflowOsError> {
        self.reference.validate()?;
        self.source_resolution.validate()?;
        let expected_capability = self.access_level.required_capability()?;
        let expected_resource = self.reference.capability_resource()?;
        let context = self.source_resolution.context();
        if self.source_resolution.posture() != CapabilityResolutionPosture::Authorized
            || self.reference.availability() != GovernedContextAvailability::Available
            || context.capability() != &expected_capability
            || context.resource() != &expected_resource
            || context.requested_sensitivity() != self.reference.sensitivity()
        {
            return Err(validation_error(
                "governed_context.entry.authority_mismatch",
                "context projection entry requires exact authorized capability",
            ));
        }
        match (self.access_level, self.metadata) {
            (GovernedContextAccessLevel::ReferenceOnly, None) => Ok(()),
            (GovernedContextAccessLevel::BoundedMetadata, Some(metadata))
                if metadata.target_kind == self.reference.kind()
                    && metadata.declared_sensitivity == self.reference.sensitivity()
                    && metadata.availability_observed_at
                        <= self.source_resolution.evaluated_at() =>
            {
                Ok(())
            }
            _ => Err(validation_error(
                "governed_context.entry.metadata_inconsistent",
                "context projection entry metadata is inconsistent",
            )),
        }
    }

    /// Returns the authorized stable reference.
    #[must_use]
    pub const fn reference(&self) -> &GovernedContextReference {
        &self.reference
    }

    /// Returns the granted access level.
    #[must_use]
    pub const fn access_level(&self) -> GovernedContextAccessLevel {
        self.access_level
    }

    /// Returns fixed safe metadata when bounded-metadata access was granted.
    #[must_use]
    pub const fn metadata(&self) -> Option<&GovernedContextBoundedMetadata> {
        self.metadata.as_ref()
    }
}

impl<'de> Deserialize<'de> for GovernedContextProjectionEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            reference: GovernedContextReference,
            access_level: GovernedContextAccessLevel,
            metadata: Option<GovernedContextBoundedMetadata>,
            source_resolution: CapabilityResolution,
        }

        let wire = Wire::deserialize(deserializer)?;
        let entry = Self {
            reference: wire.reference,
            access_level: wire.access_level,
            metadata: wire.metadata,
            source_resolution: wire.source_resolution,
        };
        entry.validate().map_err(serde::de::Error::custom)?;
        Ok(entry)
    }
}

impl fmt::Debug for GovernedContextProjectionEntry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GovernedContextProjectionEntry")
            .field("reference", &self.reference)
            .field("access_level", &self.access_level)
            .field("metadata", &self.metadata)
            .field("source_resolution", &self.source_resolution)
            .finish()
    }
}

/// Stable bounded reason that an evaluated candidate was not projected.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernedContextProjectionGapReason {
    /// The target was declared unavailable.
    Unavailable,
    /// Current target availability is unknown.
    UnknownAvailability,
    /// No exact current authority authorizes the target and access level.
    NoMatchingAuthority,
    /// Independent policy evaluation remains required.
    IndependentPolicyEvaluationRequired,
    /// Independent approval evaluation remains required.
    IndependentApprovalEvaluationRequired,
    /// Independent evidence or check evaluation remains required.
    IndependentEvidenceOrCheckEvaluationRequired,
    /// The target exceeds the projection sensitivity ceiling.
    SensitivityCeilingExceeded,
    /// The requested access level is unavailable or unsupported.
    AccessLevelNotAuthorized,
}

impl<'de> Deserialize<'de> for GovernedContextProjectionGapReason {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "unavailable" => Ok(Self::Unavailable),
            "unknown_availability" => Ok(Self::UnknownAvailability),
            "no_matching_authority" => Ok(Self::NoMatchingAuthority),
            "independent_policy_evaluation_required" => {
                Ok(Self::IndependentPolicyEvaluationRequired)
            }
            "independent_approval_evaluation_required" => {
                Ok(Self::IndependentApprovalEvaluationRequired)
            }
            "independent_evidence_or_check_evaluation_required" => {
                Ok(Self::IndependentEvidenceOrCheckEvaluationRequired)
            }
            "sensitivity_ceiling_exceeded" => Ok(Self::SensitivityCeilingExceeded),
            "access_level_not_authorized" => Ok(Self::AccessLevelNotAuthorized),
            _ => Err(serde::de::Error::custom(validation_error(
                "governed_context.gap.reason_invalid",
                "governed context gap reason is invalid",
            ))),
        }
    }
}

/// Payload-free explanation for one unprojected candidate.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GovernedContextProjectionGap {
    target_kind: GovernedContextReferenceKind,
    reason: GovernedContextProjectionGapReason,
}

impl GovernedContextProjectionGap {
    /// Returns the target kind without exposing the rejected identity.
    #[must_use]
    pub const fn target_kind(&self) -> GovernedContextReferenceKind {
        self.target_kind
    }

    /// Returns the bounded gap reason.
    #[must_use]
    pub const fn reason(&self) -> GovernedContextProjectionGapReason {
        self.reason
    }
}

/// Explicit borrowed inputs for pure step-scoped context projection.
pub struct GovernedContextProjectionInput<'a> {
    /// Actor receiving the projection.
    pub actor: &'a ActorId,
    /// Workflow boundary.
    pub workflow_id: &'a WorkflowId,
    /// Exact run boundary.
    pub run_id: &'a WorkflowRunId,
    /// Exact step boundary.
    pub step_id: &'a StepId,
    /// Optional harness-contract boundary.
    pub harness_contract_id: Option<&'a HarnessContractId>,
    /// Exact shared projection timestamp.
    pub projected_at: Timestamp,
    /// Maximum sensitivity this projection may expose.
    pub maximum_allowed_sensitivity: WorkReportSensitivity,
    /// Access level requested for every complete evaluated candidate.
    pub requested_access_level: GovernedContextAccessLevel,
    /// Complete evaluated candidate set.
    pub candidates: &'a [GovernedContextProjectionCandidate],
    /// Required bounded redaction posture.
    pub redaction: &'a RedactionMetadata,
}

impl fmt::Debug for GovernedContextProjectionInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GovernedContextProjectionInput")
            .field("actor", &"[REDACTED]")
            .field("workflow_id", &"[REDACTED]")
            .field("run_id", &"[REDACTED]")
            .field("step_id", &"[REDACTED]")
            .field(
                "harness_contract_id",
                &self.harness_contract_id.map(|_| "[REDACTED]"),
            )
            .field("projected_at", &self.projected_at)
            .field(
                "maximum_allowed_sensitivity",
                &self.maximum_allowed_sensitivity,
            )
            .field("requested_access_level", &self.requested_access_level)
            .field("candidates", &self.candidates.len())
            .field("redaction", &RedactedRedactionMetadataDebug(self.redaction))
            .finish()
    }
}

/// Deterministic payload-free context projection for one exact step.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct GovernedContextProjection {
    actor: ActorId,
    workflow_id: WorkflowId,
    run_id: WorkflowRunId,
    step_id: StepId,
    harness_contract_id: Option<HarnessContractId>,
    projected_at: Timestamp,
    maximum_allowed_sensitivity: WorkReportSensitivity,
    requested_access_level: GovernedContextAccessLevel,
    candidates: Vec<GovernedContextProjectionCandidate>,
    entries: Vec<GovernedContextProjectionEntry>,
    gaps: Vec<GovernedContextProjectionGap>,
    redaction: RedactionMetadata,
}

impl GovernedContextProjection {
    /// Validates exact scope, complete-candidate retention, and deterministic derivation.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when a serialized projection omits,
    /// substitutes, reorders, or inconsistently derives candidates, entries, or gaps.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_redaction_metadata(&self.redaction)?;
        if self.maximum_allowed_sensitivity == WorkReportSensitivity::Unknown {
            return Err(validation_error(
                "governed_context.projection.sensitivity_unknown",
                "context projection requires known maximum sensitivity",
            ));
        }
        validate_candidates(
            &self.candidates,
            &ProjectionScope {
                actor: &self.actor,
                workflow_id: &self.workflow_id,
                run_id: &self.run_id,
                step_id: &self.step_id,
                harness_contract_id: self.harness_contract_id.as_ref(),
                projected_at: self.projected_at,
                requested_access_level: self.requested_access_level,
            },
        )?;
        let (expected_entries, expected_gaps) =
            derive_projection(&self.candidates, self.maximum_allowed_sensitivity);
        if self.entries != expected_entries || self.gaps != expected_gaps {
            return Err(validation_error(
                "governed_context.projection.derivation_inconsistent",
                "context projection entries and gaps must exactly match retained candidates",
            ));
        }
        for entry in &self.entries {
            entry.validate()?;
        }
        Ok(())
    }

    /// Returns the actor receiving the projection.
    #[must_use]
    pub const fn actor(&self) -> &ActorId {
        &self.actor
    }

    /// Returns the exact workflow boundary.
    #[must_use]
    pub const fn workflow_id(&self) -> &WorkflowId {
        &self.workflow_id
    }

    /// Returns the exact run boundary.
    #[must_use]
    pub const fn run_id(&self) -> &WorkflowRunId {
        &self.run_id
    }

    /// Returns the exact step boundary.
    #[must_use]
    pub const fn step_id(&self) -> &StepId {
        &self.step_id
    }

    /// Returns the optional harness-contract boundary.
    #[must_use]
    pub const fn harness_contract_id(&self) -> Option<&HarnessContractId> {
        self.harness_contract_id.as_ref()
    }

    /// Returns the shared projection timestamp.
    #[must_use]
    pub const fn projected_at(&self) -> Timestamp {
        self.projected_at
    }

    /// Returns the projection sensitivity ceiling.
    #[must_use]
    pub const fn maximum_allowed_sensitivity(&self) -> WorkReportSensitivity {
        self.maximum_allowed_sensitivity
    }

    /// Returns the access level evaluated by this projection.
    #[must_use]
    pub const fn requested_access_level(&self) -> GovernedContextAccessLevel {
        self.requested_access_level
    }

    /// Returns the complete ordered evaluated candidate set.
    #[must_use]
    pub fn candidates(&self) -> &[GovernedContextProjectionCandidate] {
        &self.candidates
    }

    /// Returns authorized payload-free entries.
    #[must_use]
    pub fn entries(&self) -> &[GovernedContextProjectionEntry] {
        &self.entries
    }

    /// Returns bounded gaps for unprojected candidates.
    #[must_use]
    pub fn gaps(&self) -> &[GovernedContextProjectionGap] {
        &self.gaps
    }
}

impl fmt::Debug for GovernedContextProjection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GovernedContextProjection")
            .field("actor", &"[REDACTED]")
            .field("workflow_id", &"[REDACTED]")
            .field("run_id", &"[REDACTED]")
            .field("step_id", &"[REDACTED]")
            .field(
                "harness_contract_id",
                &self.harness_contract_id.as_ref().map(|_| "[REDACTED]"),
            )
            .field("projected_at", &self.projected_at)
            .field(
                "maximum_allowed_sensitivity",
                &self.maximum_allowed_sensitivity,
            )
            .field("requested_access_level", &self.requested_access_level)
            .field("candidates", &self.candidates.len())
            .field("entries", &self.entries.len())
            .field("gaps", &self.gaps)
            .field(
                "redaction",
                &RedactedRedactionMetadataDebug(&self.redaction),
            )
            .finish()
    }
}

impl<'de> Deserialize<'de> for GovernedContextProjection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            actor: ActorId,
            workflow_id: WorkflowId,
            run_id: WorkflowRunId,
            step_id: StepId,
            harness_contract_id: Option<HarnessContractId>,
            projected_at: Timestamp,
            maximum_allowed_sensitivity: WorkReportSensitivity,
            requested_access_level: GovernedContextAccessLevel,
            candidates: Vec<GovernedContextProjectionCandidate>,
            entries: Vec<GovernedContextProjectionEntry>,
            gaps: Vec<GovernedContextProjectionGap>,
            redaction: RedactionMetadata,
        }

        let wire = Wire::deserialize(deserializer)?;
        let projection = Self {
            actor: wire.actor,
            workflow_id: wire.workflow_id,
            run_id: wire.run_id,
            step_id: wire.step_id,
            harness_contract_id: wire.harness_contract_id,
            projected_at: wire.projected_at,
            maximum_allowed_sensitivity: wire.maximum_allowed_sensitivity,
            requested_access_level: wire.requested_access_level,
            candidates: wire.candidates,
            entries: wire.entries,
            gaps: wire.gaps,
            redaction: wire.redaction,
        };
        projection.validate().map_err(serde::de::Error::custom)?;
        Ok(projection)
    }
}

/// Projects only authorized stable references for one exact step.
///
/// This helper is pure and payload-free. It does not dereference targets,
/// inspect repositories, contact stores or providers, mutate state, emit
/// events, or persist results.
///
/// # Errors
///
/// Returns a stable non-leaking error for unsafe metadata, incomplete
/// candidates, wrong context, stale resolution time, duplicate targets, or
/// mismatched capability authority.
pub fn project_step_scoped_context(
    input: &GovernedContextProjectionInput<'_>,
) -> Result<GovernedContextProjection, WorkflowOsError> {
    validate_redaction_metadata(input.redaction)?;
    if input.maximum_allowed_sensitivity == WorkReportSensitivity::Unknown {
        return Err(validation_error(
            "governed_context.projection.sensitivity_unknown",
            "context projection requires known maximum sensitivity",
        ));
    }
    let mut candidates = input.candidates.to_vec();
    candidates.sort_by_key(candidate_key);
    validate_candidates(
        &candidates,
        &ProjectionScope {
            actor: input.actor,
            workflow_id: input.workflow_id,
            run_id: input.run_id,
            step_id: input.step_id,
            harness_contract_id: input.harness_contract_id,
            projected_at: input.projected_at,
            requested_access_level: input.requested_access_level,
        },
    )?;
    let (entries, gaps) = derive_projection(&candidates, input.maximum_allowed_sensitivity);
    let projection = GovernedContextProjection {
        actor: input.actor.clone(),
        workflow_id: input.workflow_id.clone(),
        run_id: input.run_id.clone(),
        step_id: input.step_id.clone(),
        harness_contract_id: input.harness_contract_id.cloned(),
        projected_at: input.projected_at,
        maximum_allowed_sensitivity: input.maximum_allowed_sensitivity,
        requested_access_level: input.requested_access_level,
        candidates,
        entries,
        gaps,
        redaction: input.redaction.clone(),
    };
    projection.validate()?;
    Ok(projection)
}

struct ProjectionScope<'a> {
    actor: &'a ActorId,
    workflow_id: &'a WorkflowId,
    run_id: &'a WorkflowRunId,
    step_id: &'a StepId,
    harness_contract_id: Option<&'a HarnessContractId>,
    projected_at: Timestamp,
    requested_access_level: GovernedContextAccessLevel,
}

fn validate_candidates(
    candidates: &[GovernedContextProjectionCandidate],
    scope: &ProjectionScope<'_>,
) -> Result<(), WorkflowOsError> {
    let mut seen = BTreeSet::new();
    let mut previous = None;
    for candidate in candidates {
        candidate.validate()?;
        if candidate.requested_access_level != scope.requested_access_level {
            return Err(validation_error(
                "governed_context.projection.access_level_mismatch",
                "all context candidates must use the projection access level",
            ));
        }
        let resolution = &candidate.source_resolution;
        let context = resolution.context();
        if context.actor() != scope.actor
            || context.workflow_id() != scope.workflow_id
            || context.run_id() != scope.run_id
            || context.step_id() != scope.step_id
            || context.harness_contract_id() != scope.harness_contract_id
            || resolution.evaluated_at() != scope.projected_at
        {
            return Err(validation_error(
                "governed_context.projection.context_mismatch",
                "context projection requires exact fresh same-step authority",
            ));
        }
        let key = candidate_key(candidate);
        if !seen.insert(key.clone()) {
            return Err(validation_error(
                "governed_context.projection.duplicate_candidate",
                "context projection cannot accept duplicate targets",
            ));
        }
        if previous.as_ref().is_some_and(|value| value >= &key) {
            return Err(validation_error(
                "governed_context.projection.candidates_unordered",
                "retained context candidates must be unique and ordered",
            ));
        }
        previous = Some(key);
    }
    Ok(())
}

fn candidate_key(candidate: &GovernedContextProjectionCandidate) -> String {
    format!(
        "{}/{}",
        candidate.reference.kind().canonical_name(),
        candidate.reference.target.stable_id()
    )
}

fn derive_projection(
    candidates: &[GovernedContextProjectionCandidate],
    sensitivity_ceiling: WorkReportSensitivity,
) -> (
    Vec<GovernedContextProjectionEntry>,
    Vec<GovernedContextProjectionGap>,
) {
    let mut entries = Vec::new();
    let mut gaps = Vec::new();
    for candidate in candidates {
        let reason = gap_reason(candidate, sensitivity_ceiling);
        if let Some(reason) = reason {
            gaps.push(GovernedContextProjectionGap {
                target_kind: candidate.reference.kind(),
                reason,
            });
            continue;
        }
        let metadata = match candidate.requested_access_level {
            GovernedContextAccessLevel::ReferenceOnly => None,
            GovernedContextAccessLevel::BoundedMetadata => Some(GovernedContextBoundedMetadata {
                target_kind: candidate.reference.kind(),
                declared_sensitivity: candidate.reference.sensitivity(),
                availability_observed_at: candidate.availability_observed_at,
            }),
        };
        entries.push(GovernedContextProjectionEntry {
            reference: candidate.reference.clone(),
            access_level: candidate.requested_access_level,
            metadata,
            source_resolution: candidate.source_resolution.clone(),
        });
    }
    (entries, gaps)
}

fn gap_reason(
    candidate: &GovernedContextProjectionCandidate,
    sensitivity_ceiling: WorkReportSensitivity,
) -> Option<GovernedContextProjectionGapReason> {
    match candidate.reference.availability() {
        GovernedContextAvailability::Unavailable => {
            return Some(GovernedContextProjectionGapReason::Unavailable);
        }
        GovernedContextAvailability::Unknown => {
            return Some(GovernedContextProjectionGapReason::UnknownAvailability);
        }
        GovernedContextAvailability::Available => {}
    }
    if candidate.reference.sensitivity() > sensitivity_ceiling {
        return Some(GovernedContextProjectionGapReason::SensitivityCeilingExceeded);
    }
    match candidate.source_resolution.posture() {
        CapabilityResolutionPosture::Authorized => None,
        CapabilityResolutionPosture::RequiresIndependentEvaluation => {
            if candidate
                .source_resolution
                .reasons()
                .contains(&CapabilityResolutionReason::PolicyEvaluationRequired)
            {
                Some(GovernedContextProjectionGapReason::IndependentPolicyEvaluationRequired)
            } else if candidate
                .source_resolution
                .reasons()
                .contains(&CapabilityResolutionReason::ApprovalEvaluationRequired)
            {
                Some(GovernedContextProjectionGapReason::IndependentApprovalEvaluationRequired)
            } else {
                Some(
                    GovernedContextProjectionGapReason::IndependentEvidenceOrCheckEvaluationRequired,
                )
            }
        }
        CapabilityResolutionPosture::NotAuthorized => {
            if candidate.source_resolution.reasons().iter().any(|reason| {
                matches!(
                    reason,
                    CapabilityResolutionReason::CapabilityNotConnected
                        | CapabilityResolutionReason::CapabilityUnsupported
                        | CapabilityResolutionReason::CapabilityAvailabilityUnknown
                )
            }) {
                Some(GovernedContextProjectionGapReason::AccessLevelNotAuthorized)
            } else {
                Some(GovernedContextProjectionGapReason::NoMatchingAuthority)
            }
        }
    }
}

fn validate_redaction_metadata(redaction: &RedactionMetadata) -> Result<(), WorkflowOsError> {
    if redaction.redacted_fields.len() > REDACTION_MAX_ENTRIES
        || redaction.field_states.len() > REDACTION_MAX_ENTRIES
    {
        return Err(validation_error(
            "governed_context.redaction.too_many_entries",
            "context redaction metadata contains too many entries",
        ));
    }
    for field in &redaction.redacted_fields {
        validate_redaction_field(field)?;
    }
    for state in &redaction.field_states {
        validate_redaction_field(&state.field)?;
        if state.reason.is_empty() || state.reason.len() > REDACTION_REASON_MAX_BYTES {
            return Err(validation_error(
                "governed_context.redaction.reason_invalid",
                "context redaction reason is invalid",
            ));
        }
        validate_not_secret_like(&state.reason)?;
    }
    Ok(())
}

fn validate_redaction_field(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() || value.len() > REDACTION_FIELD_MAX_BYTES {
        return Err(validation_error(
            "governed_context.redaction.field_invalid",
            "context redaction field is invalid",
        ));
    }
    validate_not_secret_like(value)
}

fn validate_not_secret_like(value: &str) -> Result<(), WorkflowOsError> {
    let lowercase = value.to_ascii_lowercase();
    if lowercase.contains("authorization")
        || lowercase.contains("bearer")
        || lowercase.contains("private_key")
        || lowercase.contains("private-key")
        || lowercase.contains("api_token")
        || lowercase.contains("api-token")
        || lowercase.contains("secret")
        || lowercase.contains("token")
    {
        return Err(validation_error(
            "governed_context.secret_like_value",
            "governed context value contains sensitive-looking text",
        ));
    }
    Ok(())
}

fn validation_error(code: &'static str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(code, message)
}

struct RedactedRedactionMetadataDebug<'a>(&'a RedactionMetadata);

impl fmt::Debug for RedactedRedactionMetadataDebug<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RedactionMetadata")
            .field("redacted_fields", &self.0.redacted_fields.len())
            .field("field_states", &self.0.field_states.len())
            .finish()
    }
}
