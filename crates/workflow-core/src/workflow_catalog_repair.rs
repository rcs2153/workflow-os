use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    ActorId, ApprovalReferenceId, EventId, EvidenceReferenceId, RedactionDisposition,
    RedactionFieldState, RedactionMetadata, Timestamp, ValidationReferenceId, WorkReportId,
    WorkReportSensitivity, WorkflowCatalogConflict, WorkflowCatalogConflictKind,
    WorkflowCatalogConflictSource, WorkflowCatalogIndex, WorkflowId, WorkflowOsError,
};

const REVIEW_REFERENCE_MAX_COUNT: usize = 32;

/// Stable identifier for a workflow catalog repair proposal.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
pub struct WorkflowCatalogRepairProposalId(String);

impl WorkflowCatalogRepairProposalId {
    /// Creates a bounded repair proposal id.
    ///
    /// # Errors
    ///
    /// Returns an error when the id is empty, too long, unsafe, or secret-like.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        validate_identifier("workflow catalog repair proposal id", &value)?;
        Ok(Self(value))
    }

    /// Returns the proposal id as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for WorkflowCatalogRepairProposalId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Debug for WorkflowCatalogRepairProposalId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("WorkflowCatalogRepairProposalId")
            .field(&self.0)
            .finish()
    }
}

impl<'de> Deserialize<'de> for WorkflowCatalogRepairProposalId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

/// Non-mutating action kind proposed by catalog repair planning.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowCatalogRepairActionKind {
    /// A future apply mode may create a missing active workflow catalog record.
    CreateMissingCatalogRecord,
    /// Metadata is missing but requires explicit maintainer-supplied values.
    UpdateCatalogRecordMetadata,
    /// Active workflow file and catalog sidecar disagree.
    ReviewCatalogRecordMismatch,
    /// Archive sidecar and archived draft state disagree.
    ReviewArchiveRecordMismatch,
    /// A persisted stewardship decision no longer matches its draft.
    ReviewStaleStewardshipDecision,
    /// Duplicate active workflow identity or path requires manual review.
    ReviewDuplicateActiveWorkflow,
    /// Catalog store state needs manual cleanup outside the first slice.
    RequiresCatalogStoreCleanup,
    /// No automatic repair is available in the first slice.
    NoAutomaticRepairAvailable,
}

/// Deterministic, non-mutating repair proposal derived from catalog conflicts.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct WorkflowCatalogRepairProposal {
    proposal_id: WorkflowCatalogRepairProposalId,
    conflict_kind: WorkflowCatalogConflictKind,
    conflict_source: WorkflowCatalogConflictSource,
    workflow_id: Option<WorkflowId>,
    source_reference: String,
    action_kind: WorkflowCatalogRepairActionKind,
    safe_for_future_apply: bool,
    human_review_required: bool,
    summary: String,
    sensitivity: WorkReportSensitivity,
    redaction: RedactionMetadata,
}

impl WorkflowCatalogRepairProposal {
    fn new(
        proposal_id: WorkflowCatalogRepairProposalId,
        conflict: &WorkflowCatalogConflict,
        action_kind: WorkflowCatalogRepairActionKind,
        safe_for_future_apply: bool,
        human_review_required: bool,
        summary: impl Into<String>,
    ) -> Result<Self, WorkflowOsError> {
        let summary = summary.into();
        validate_action_posture(action_kind, safe_for_future_apply, human_review_required)?;
        validate_reference_text("workflow catalog repair proposal summary", &summary)?;
        validate_reference_text(
            "workflow catalog repair proposal source reference",
            conflict.source_reference(),
        )?;
        Ok(Self {
            proposal_id,
            conflict_kind: conflict.kind(),
            conflict_source: conflict.source(),
            workflow_id: conflict.workflow_id().cloned(),
            source_reference: conflict.source_reference().to_owned(),
            action_kind,
            safe_for_future_apply,
            human_review_required,
            summary,
            sensitivity: WorkReportSensitivity::Confidential,
            redaction: repair_redaction_metadata(),
        })
    }

    /// Returns the stable proposal id.
    #[must_use]
    pub const fn proposal_id(&self) -> &WorkflowCatalogRepairProposalId {
        &self.proposal_id
    }

    /// Returns the source conflict kind.
    #[must_use]
    pub const fn conflict_kind(&self) -> WorkflowCatalogConflictKind {
        self.conflict_kind
    }

    /// Returns the conflict source category.
    #[must_use]
    pub const fn conflict_source(&self) -> WorkflowCatalogConflictSource {
        self.conflict_source
    }

    /// Returns the workflow id associated with this proposal, if known.
    #[must_use]
    pub const fn workflow_id(&self) -> Option<&WorkflowId> {
        self.workflow_id.as_ref()
    }

    /// Returns the bounded source reference.
    #[must_use]
    pub fn source_reference(&self) -> &str {
        &self.source_reference
    }

    /// Returns the proposed action kind.
    #[must_use]
    pub const fn action_kind(&self) -> WorkflowCatalogRepairActionKind {
        self.action_kind
    }

    /// Returns whether a future apply mode could safely automate this class.
    #[must_use]
    pub const fn safe_for_future_apply(&self) -> bool {
        self.safe_for_future_apply
    }

    /// Returns whether human review is required.
    #[must_use]
    pub const fn human_review_required(&self) -> bool {
        self.human_review_required
    }

    /// Returns a bounded proposal summary.
    #[must_use]
    pub fn summary(&self) -> &str {
        &self.summary
    }

    /// Returns the proposal sensitivity.
    #[must_use]
    pub const fn sensitivity(&self) -> WorkReportSensitivity {
        self.sensitivity
    }

    /// Returns redaction metadata for this proposal.
    #[must_use]
    pub const fn redaction(&self) -> &RedactionMetadata {
        &self.redaction
    }
}

impl<'de> Deserialize<'de> for WorkflowCatalogRepairProposal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            proposal_id: WorkflowCatalogRepairProposalId,
            conflict_kind: WorkflowCatalogConflictKind,
            conflict_source: WorkflowCatalogConflictSource,
            workflow_id: Option<WorkflowId>,
            source_reference: String,
            action_kind: WorkflowCatalogRepairActionKind,
            safe_for_future_apply: bool,
            human_review_required: bool,
            summary: String,
            sensitivity: WorkReportSensitivity,
            redaction: RedactionMetadata,
        }

        let wire = Wire::deserialize(deserializer)?;
        validate_reference_text(
            "workflow catalog repair proposal source reference",
            &wire.source_reference,
        )
        .map_err(serde::de::Error::custom)?;
        validate_reference_text("workflow catalog repair proposal summary", &wire.summary)
            .map_err(serde::de::Error::custom)?;
        validate_redaction_metadata(&wire.redaction).map_err(serde::de::Error::custom)?;
        validate_action_posture(
            wire.action_kind,
            wire.safe_for_future_apply,
            wire.human_review_required,
        )
        .map_err(serde::de::Error::custom)?;
        Ok(Self {
            proposal_id: wire.proposal_id,
            conflict_kind: wire.conflict_kind,
            conflict_source: wire.conflict_source,
            workflow_id: wire.workflow_id,
            source_reference: wire.source_reference,
            action_kind: wire.action_kind,
            safe_for_future_apply: wire.safe_for_future_apply,
            human_review_required: wire.human_review_required,
            summary: wire.summary,
            sensitivity: wire.sensitivity,
            redaction: wire.redaction,
        })
    }
}

impl fmt::Debug for WorkflowCatalogRepairProposal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkflowCatalogRepairProposal")
            .field("proposal_id", &self.proposal_id)
            .field("conflict_kind", &self.conflict_kind)
            .field("conflict_source", &self.conflict_source)
            .field("workflow_id", &self.workflow_id)
            .field("source_reference", &self.source_reference)
            .field("action_kind", &self.action_kind)
            .field("safe_for_future_apply", &self.safe_for_future_apply)
            .field("human_review_required", &self.human_review_required)
            .field("summary", &self.summary)
            .field("sensitivity", &self.sensitivity)
            .field(
                "redaction",
                &RedactedRedactionMetadataDebug(&self.redaction),
            )
            .finish()
    }
}

/// Derives deterministic, non-mutating catalog repair proposals from a catalog
/// index.
///
/// # Errors
///
/// Returns an error if proposal construction fails validation.
pub fn propose_workflow_catalog_repairs(
    index: &WorkflowCatalogIndex,
) -> Result<Vec<WorkflowCatalogRepairProposal>, WorkflowOsError> {
    index
        .conflicts()
        .iter()
        .enumerate()
        .map(|(index, conflict)| proposal_from_conflict(index, conflict))
        .collect()
}

/// Stable identifier for a workflow catalog repair proposal review decision.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
pub struct WorkflowCatalogRepairProposalReviewId(String);

impl WorkflowCatalogRepairProposalReviewId {
    /// Creates a bounded repair proposal review id.
    ///
    /// # Errors
    ///
    /// Returns an error when the id is empty, too long, unsafe, or secret-like.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        validate_identifier("workflow catalog repair proposal review id", &value)?;
        Ok(Self(value))
    }

    /// Returns the review id as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for WorkflowCatalogRepairProposalReviewId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Debug for WorkflowCatalogRepairProposalReviewId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("WorkflowCatalogRepairProposalReviewId")
            .field(&self.0)
            .finish()
    }
}

impl<'de> Deserialize<'de> for WorkflowCatalogRepairProposalReviewId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

/// Maintainer decision recorded against a repair proposal.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowCatalogRepairProposalDecisionKind {
    /// The proposal may be considered by a future apply-mode planner.
    ApprovedForFutureApplyPlanning,
    /// The proposal is rejected.
    Rejected,
    /// The proposal is deferred for later review.
    Deferred,
    /// Catalog sidecar state requires manual review before reuse.
    RequiresManualCatalogReview,
    /// Workflow file state requires manual review before reuse.
    RequiresManualWorkflowReview,
    /// A fresh dry-run is required before the decision can be reused.
    RequiresNewDryRun,
}

/// Explicit input for constructing a repair proposal review record.
pub struct WorkflowCatalogRepairProposalReviewInput<'a> {
    /// Stable review id supplied by the caller.
    pub review_id: WorkflowCatalogRepairProposalReviewId,
    /// Typed repair proposal being reviewed.
    pub proposal: &'a WorkflowCatalogRepairProposal,
    /// Maintainer or steward actor making the decision.
    pub reviewer: ActorId,
    /// Bounded reviewer rationale.
    pub reason: String,
    /// Review decision.
    pub decision_kind: WorkflowCatalogRepairProposalDecisionKind,
    /// Time the decision was reviewed.
    pub reviewed_at: Timestamp,
    /// Optional approval references, when available.
    pub approval_references: Vec<ApprovalReferenceId>,
    /// Optional policy decision event references, when available.
    pub policy_decision_references: Vec<EventId>,
    /// Optional evidence references, cited by stable id only.
    pub evidence_references: Vec<EvidenceReferenceId>,
    /// Optional validation references, cited by stable id only.
    pub validation_references: Vec<ValidationReferenceId>,
    /// Optional `WorkReport` references, cited by stable id only.
    pub work_report_references: Vec<WorkReportId>,
    /// Review sensitivity.
    pub sensitivity: WorkReportSensitivity,
    /// Redaction metadata for the review record.
    pub redaction: RedactionMetadata,
}

/// In-memory maintainer review decision for a repair proposal.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct WorkflowCatalogRepairProposalReview {
    review_id: WorkflowCatalogRepairProposalReviewId,
    proposal_id: WorkflowCatalogRepairProposalId,
    proposal_action_kind: WorkflowCatalogRepairActionKind,
    proposal_conflict_kind: WorkflowCatalogConflictKind,
    proposal_conflict_source: WorkflowCatalogConflictSource,
    workflow_id: Option<WorkflowId>,
    source_reference: String,
    reviewer: ActorId,
    reason: String,
    decision_kind: WorkflowCatalogRepairProposalDecisionKind,
    reviewed_at: Timestamp,
    approval_references: Vec<ApprovalReferenceId>,
    policy_decision_references: Vec<EventId>,
    evidence_references: Vec<EvidenceReferenceId>,
    validation_references: Vec<ValidationReferenceId>,
    work_report_references: Vec<WorkReportId>,
    sensitivity: WorkReportSensitivity,
    redaction: RedactionMetadata,
}

impl WorkflowCatalogRepairProposalReview {
    fn from_input(
        input: WorkflowCatalogRepairProposalReviewInput<'_>,
    ) -> Result<Self, WorkflowOsError> {
        validate_reference_text(
            "workflow catalog repair proposal review reason",
            &input.reason,
        )?;
        validate_reference_count(
            "workflow catalog repair proposal approval references",
            input.approval_references.len(),
        )?;
        validate_reference_count(
            "workflow catalog repair proposal policy decision references",
            input.policy_decision_references.len(),
        )?;
        validate_reference_count(
            "workflow catalog repair proposal evidence references",
            input.evidence_references.len(),
        )?;
        validate_reference_count(
            "workflow catalog repair proposal validation references",
            input.validation_references.len(),
        )?;
        validate_reference_count(
            "workflow catalog repair proposal work report references",
            input.work_report_references.len(),
        )?;
        validate_redaction_metadata(&input.redaction)?;

        Ok(Self {
            review_id: input.review_id,
            proposal_id: input.proposal.proposal_id().clone(),
            proposal_action_kind: input.proposal.action_kind(),
            proposal_conflict_kind: input.proposal.conflict_kind(),
            proposal_conflict_source: input.proposal.conflict_source(),
            workflow_id: input.proposal.workflow_id().cloned(),
            source_reference: input.proposal.source_reference().to_owned(),
            reviewer: input.reviewer,
            reason: input.reason,
            decision_kind: input.decision_kind,
            reviewed_at: input.reviewed_at,
            approval_references: input.approval_references,
            policy_decision_references: input.policy_decision_references,
            evidence_references: input.evidence_references,
            validation_references: input.validation_references,
            work_report_references: input.work_report_references,
            sensitivity: input.sensitivity,
            redaction: input.redaction,
        })
    }

    /// Returns the stable review id.
    #[must_use]
    pub const fn review_id(&self) -> &WorkflowCatalogRepairProposalReviewId {
        &self.review_id
    }

    /// Returns the reviewed proposal id.
    #[must_use]
    pub const fn proposal_id(&self) -> &WorkflowCatalogRepairProposalId {
        &self.proposal_id
    }

    /// Returns the reviewed proposal action kind.
    #[must_use]
    pub const fn proposal_action_kind(&self) -> WorkflowCatalogRepairActionKind {
        self.proposal_action_kind
    }

    /// Returns the reviewed proposal conflict kind.
    #[must_use]
    pub const fn proposal_conflict_kind(&self) -> WorkflowCatalogConflictKind {
        self.proposal_conflict_kind
    }

    /// Returns the reviewed proposal conflict source.
    #[must_use]
    pub const fn proposal_conflict_source(&self) -> WorkflowCatalogConflictSource {
        self.proposal_conflict_source
    }

    /// Returns the reviewed workflow id, if known.
    #[must_use]
    pub const fn workflow_id(&self) -> Option<&WorkflowId> {
        self.workflow_id.as_ref()
    }

    /// Returns the bounded source reference.
    #[must_use]
    pub fn source_reference(&self) -> &str {
        &self.source_reference
    }

    /// Returns the reviewer actor.
    #[must_use]
    pub const fn reviewer(&self) -> &ActorId {
        &self.reviewer
    }

    /// Returns the bounded reviewer reason.
    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }

    /// Returns the review decision kind.
    #[must_use]
    pub const fn decision_kind(&self) -> WorkflowCatalogRepairProposalDecisionKind {
        self.decision_kind
    }

    /// Returns the review timestamp.
    #[must_use]
    pub const fn reviewed_at(&self) -> Timestamp {
        self.reviewed_at
    }

    /// Returns optional approval references.
    #[must_use]
    pub fn approval_references(&self) -> &[ApprovalReferenceId] {
        &self.approval_references
    }

    /// Returns optional policy decision references.
    #[must_use]
    pub fn policy_decision_references(&self) -> &[EventId] {
        &self.policy_decision_references
    }

    /// Returns optional evidence references.
    #[must_use]
    pub fn evidence_references(&self) -> &[EvidenceReferenceId] {
        &self.evidence_references
    }

    /// Returns optional validation references.
    #[must_use]
    pub fn validation_references(&self) -> &[ValidationReferenceId] {
        &self.validation_references
    }

    /// Returns optional `WorkReport` references.
    #[must_use]
    pub fn work_report_references(&self) -> &[WorkReportId] {
        &self.work_report_references
    }

    /// Returns review sensitivity.
    #[must_use]
    pub const fn sensitivity(&self) -> WorkReportSensitivity {
        self.sensitivity
    }

    /// Returns redaction metadata.
    #[must_use]
    pub const fn redaction(&self) -> &RedactionMetadata {
        &self.redaction
    }

    /// Returns whether this review still matches the supplied proposal identity.
    #[must_use]
    pub fn matches_proposal_identity(&self, proposal: &WorkflowCatalogRepairProposal) -> bool {
        self.proposal_id == *proposal.proposal_id()
            && self.proposal_action_kind == proposal.action_kind()
            && self.proposal_conflict_kind == proposal.conflict_kind()
            && self.proposal_conflict_source == proposal.conflict_source()
            && self.workflow_id.as_ref() == proposal.workflow_id()
            && self.source_reference == proposal.source_reference()
    }
}

impl<'de> Deserialize<'de> for WorkflowCatalogRepairProposalReview {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            review_id: WorkflowCatalogRepairProposalReviewId,
            proposal_id: WorkflowCatalogRepairProposalId,
            proposal_action_kind: WorkflowCatalogRepairActionKind,
            proposal_conflict_kind: WorkflowCatalogConflictKind,
            proposal_conflict_source: WorkflowCatalogConflictSource,
            workflow_id: Option<WorkflowId>,
            source_reference: String,
            reviewer: ActorId,
            reason: String,
            decision_kind: WorkflowCatalogRepairProposalDecisionKind,
            reviewed_at: Timestamp,
            approval_references: Vec<ApprovalReferenceId>,
            policy_decision_references: Vec<EventId>,
            evidence_references: Vec<EvidenceReferenceId>,
            validation_references: Vec<ValidationReferenceId>,
            work_report_references: Vec<WorkReportId>,
            sensitivity: WorkReportSensitivity,
            redaction: RedactionMetadata,
        }

        let wire = Wire::deserialize(deserializer)?;
        validate_reference_text(
            "workflow catalog repair proposal review reason",
            &wire.reason,
        )
        .map_err(serde::de::Error::custom)?;
        validate_reference_text(
            "workflow catalog repair proposal source reference",
            &wire.source_reference,
        )
        .map_err(serde::de::Error::custom)?;
        validate_reference_count(
            "workflow catalog repair proposal approval references",
            wire.approval_references.len(),
        )
        .map_err(serde::de::Error::custom)?;
        validate_reference_count(
            "workflow catalog repair proposal policy decision references",
            wire.policy_decision_references.len(),
        )
        .map_err(serde::de::Error::custom)?;
        validate_reference_count(
            "workflow catalog repair proposal evidence references",
            wire.evidence_references.len(),
        )
        .map_err(serde::de::Error::custom)?;
        validate_reference_count(
            "workflow catalog repair proposal validation references",
            wire.validation_references.len(),
        )
        .map_err(serde::de::Error::custom)?;
        validate_reference_count(
            "workflow catalog repair proposal work report references",
            wire.work_report_references.len(),
        )
        .map_err(serde::de::Error::custom)?;
        validate_redaction_metadata(&wire.redaction).map_err(serde::de::Error::custom)?;

        Ok(Self {
            review_id: wire.review_id,
            proposal_id: wire.proposal_id,
            proposal_action_kind: wire.proposal_action_kind,
            proposal_conflict_kind: wire.proposal_conflict_kind,
            proposal_conflict_source: wire.proposal_conflict_source,
            workflow_id: wire.workflow_id,
            source_reference: wire.source_reference,
            reviewer: wire.reviewer,
            reason: wire.reason,
            decision_kind: wire.decision_kind,
            reviewed_at: wire.reviewed_at,
            approval_references: wire.approval_references,
            policy_decision_references: wire.policy_decision_references,
            evidence_references: wire.evidence_references,
            validation_references: wire.validation_references,
            work_report_references: wire.work_report_references,
            sensitivity: wire.sensitivity,
            redaction: wire.redaction,
        })
    }
}

impl fmt::Debug for WorkflowCatalogRepairProposalReview {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkflowCatalogRepairProposalReview")
            .field("review_id", &self.review_id)
            .field("proposal_id", &self.proposal_id)
            .field("proposal_action_kind", &self.proposal_action_kind)
            .field("proposal_conflict_kind", &self.proposal_conflict_kind)
            .field("proposal_conflict_source", &self.proposal_conflict_source)
            .field("workflow_id", &self.workflow_id)
            .field("source_reference", &"[REDACTED]")
            .field("reviewer", &self.reviewer)
            .field("reason", &"[REDACTED]")
            .field("decision_kind", &self.decision_kind)
            .field("reviewed_at", &self.reviewed_at)
            .field("approval_references_count", &self.approval_references.len())
            .field(
                "policy_decision_references_count",
                &self.policy_decision_references.len(),
            )
            .field("evidence_references_count", &self.evidence_references.len())
            .field(
                "validation_references_count",
                &self.validation_references.len(),
            )
            .field(
                "work_report_references_count",
                &self.work_report_references.len(),
            )
            .field("sensitivity", &self.sensitivity)
            .field(
                "redaction",
                &RedactedRedactionMetadataDebug(&self.redaction),
            )
            .finish()
    }
}

/// Constructs a bounded in-memory review decision from an explicit proposal.
///
/// # Errors
///
/// Returns an error when review metadata is invalid, unbounded, or
/// secret-like.
pub fn review_workflow_catalog_repair_proposal(
    input: WorkflowCatalogRepairProposalReviewInput<'_>,
) -> Result<WorkflowCatalogRepairProposalReview, WorkflowOsError> {
    WorkflowCatalogRepairProposalReview::from_input(input)
}

/// Validates that a repair proposal review still matches a fresh proposal.
///
/// # Errors
///
/// Returns an error when the reviewed proposal identity is stale.
pub fn validate_workflow_catalog_repair_proposal_review_matches(
    review: &WorkflowCatalogRepairProposalReview,
    proposal: &WorkflowCatalogRepairProposal,
) -> Result<(), WorkflowOsError> {
    if review.matches_proposal_identity(proposal) {
        Ok(())
    } else {
        Err(WorkflowOsError::validation(
            "workflow_catalog_repair.review.stale_proposal",
            "workflow catalog repair proposal review no longer matches the proposal identity",
        ))
    }
}

fn proposal_from_conflict(
    index: usize,
    conflict: &WorkflowCatalogConflict,
) -> Result<WorkflowCatalogRepairProposal, WorkflowOsError> {
    let proposal_id =
        WorkflowCatalogRepairProposalId::new(format!("catalog-repair/proposal-{:04}", index + 1))?;
    let (action_kind, safe_for_future_apply, human_review_required, summary) =
        classify_conflict(conflict.kind());
    WorkflowCatalogRepairProposal::new(
        proposal_id,
        conflict,
        action_kind,
        safe_for_future_apply,
        human_review_required,
        summary,
    )
}

const fn classify_conflict(
    kind: WorkflowCatalogConflictKind,
) -> (WorkflowCatalogRepairActionKind, bool, bool, &'static str) {
    match kind {
        WorkflowCatalogConflictKind::ActiveWorkflowMissingCatalogRecord => (
            WorkflowCatalogRepairActionKind::CreateMissingCatalogRecord,
            true,
            true,
            "active workflow is missing a catalog sidecar; future apply may create one after validation",
        ),
        WorkflowCatalogConflictKind::MissingOwner
        | WorkflowCatalogConflictKind::MissingEscalationContact
        | WorkflowCatalogConflictKind::MissingLatestStewardshipDecision
        | WorkflowCatalogConflictKind::MissingSideEffectPosture => (
            WorkflowCatalogRepairActionKind::UpdateCatalogRecordMetadata,
            false,
            true,
            "catalog metadata is incomplete and requires explicit maintainer review",
        ),
        WorkflowCatalogConflictKind::CatalogActiveMissingWorkflowFile
        | WorkflowCatalogConflictKind::CatalogActivePathMismatch
        | WorkflowCatalogConflictKind::CatalogActiveHashMismatch => (
            WorkflowCatalogRepairActionKind::ReviewCatalogRecordMismatch,
            false,
            true,
            "catalog sidecar and active workflow state disagree; no automatic repair is available",
        ),
        WorkflowCatalogConflictKind::DraftStewardshipHashMismatch => (
            WorkflowCatalogRepairActionKind::ReviewStaleStewardshipDecision,
            false,
            true,
            "stewardship decision hash is stale and must be reviewed before reuse",
        ),
        WorkflowCatalogConflictKind::ArchiveRecordMissingArchivedDraft
        | WorkflowCatalogConflictKind::ArchivePathMismatch
        | WorkflowCatalogConflictKind::ArchiveHashMismatch => (
            WorkflowCatalogRepairActionKind::ReviewArchiveRecordMismatch,
            false,
            true,
            "archive sidecar and archived draft state disagree; no automatic repair is available",
        ),
        WorkflowCatalogConflictKind::DuplicateActiveWorkflowId
        | WorkflowCatalogConflictKind::DuplicateActiveWorkflowPath => (
            WorkflowCatalogRepairActionKind::ReviewDuplicateActiveWorkflow,
            false,
            true,
            "duplicate active workflow identity or path requires manual review",
        ),
    }
}

fn validate_action_posture(
    action_kind: WorkflowCatalogRepairActionKind,
    safe_for_future_apply: bool,
    human_review_required: bool,
) -> Result<(), WorkflowOsError> {
    if !human_review_required {
        return Err(WorkflowOsError::validation(
            "workflow_catalog_repair.posture.invalid",
            "workflow catalog repair proposals require human review",
        ));
    }

    if safe_for_future_apply
        && !matches!(
            action_kind,
            WorkflowCatalogRepairActionKind::CreateMissingCatalogRecord
        )
    {
        return Err(WorkflowOsError::validation(
            "workflow_catalog_repair.posture.invalid",
            "workflow catalog repair proposal apply posture is not valid for this action",
        ));
    }

    Ok(())
}

fn repair_redaction_metadata() -> RedactionMetadata {
    RedactionMetadata {
        redacted_fields: Vec::new(),
        field_states: vec![RedactionFieldState {
            field: "workflow_catalog_repair_proposal".to_owned(),
            disposition: RedactionDisposition::ReferenceOnly,
            reason: "proposal stores bounded ids, paths, hashes, and posture only".to_owned(),
        }],
    }
}

fn validate_identifier(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(WorkflowOsError::validation(
            "workflow_catalog_repair.identifier.empty",
            format!("{type_name} must not be empty"),
        ));
    }
    if value.len() > 128 {
        return Err(WorkflowOsError::validation(
            "workflow_catalog_repair.identifier.too_long",
            format!("{type_name} is too long"),
        ));
    }
    if value.starts_with('/') || value.contains("..") {
        return Err(WorkflowOsError::validation(
            "workflow_catalog_repair.identifier.unsafe",
            format!("{type_name} must be repository-relative and safe"),
        ));
    }
    if !value
        .chars()
        .all(|character| character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '/'))
    {
        return Err(WorkflowOsError::validation(
            "workflow_catalog_repair.identifier.invalid_character",
            format!("{type_name} contains an unsupported character"),
        ));
    }
    validate_not_secret_like(type_name, value)
}

fn validate_reference_text(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(WorkflowOsError::validation(
            "workflow_catalog_repair.reference.empty",
            format!("{type_name} must not be empty"),
        ));
    }
    if value.len() > 256 {
        return Err(WorkflowOsError::validation(
            "workflow_catalog_repair.reference.too_long",
            format!("{type_name} is too long"),
        ));
    }
    if value.starts_with('/') || value.contains("..") {
        return Err(WorkflowOsError::validation(
            "workflow_catalog_repair.reference.unsafe",
            format!("{type_name} must be repository-relative and safe"),
        ));
    }
    validate_not_secret_like(type_name, value)
}

fn validate_redaction_metadata(redaction: &RedactionMetadata) -> Result<(), WorkflowOsError> {
    for field in &redaction.redacted_fields {
        validate_reference_text("workflow catalog repair redaction field", field)?;
    }
    for state in &redaction.field_states {
        validate_reference_text("workflow catalog repair redaction field", &state.field)?;
        validate_reference_text("workflow catalog repair redaction reason", &state.reason)?;
    }
    Ok(())
}

fn validate_reference_count(type_name: &'static str, len: usize) -> Result<(), WorkflowOsError> {
    if len > REVIEW_REFERENCE_MAX_COUNT {
        return Err(WorkflowOsError::validation(
            "workflow_catalog_repair.reference.too_many",
            format!("{type_name} has too many entries"),
        ));
    }
    Ok(())
}

fn validate_not_secret_like(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    let lowercase = value.to_ascii_lowercase();
    let is_secret_like = lowercase.contains("authorization")
        || lowercase.contains("bearer ")
        || lowercase.contains("private_key")
        || lowercase.contains("token")
        || lowercase.contains("secret")
        || lowercase.contains("password");

    if is_secret_like {
        return Err(WorkflowOsError::validation(
            "workflow_catalog_repair.secret_like_value",
            format!("{type_name} must not contain secret-like values"),
        ));
    }
    Ok(())
}

struct RedactedRedactionMetadataDebug<'a>(&'a RedactionMetadata);

impl fmt::Debug for RedactedRedactionMetadataDebug<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RedactionMetadata")
            .field("redacted_fields_count", &self.0.redacted_fields.len())
            .field("field_states_count", &self.0.field_states.len())
            .finish()
    }
}
