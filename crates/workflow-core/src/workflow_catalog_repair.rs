use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    RedactionDisposition, RedactionFieldState, RedactionMetadata, WorkReportSensitivity,
    WorkflowCatalogConflict, WorkflowCatalogConflictKind, WorkflowCatalogConflictSource,
    WorkflowCatalogIndex, WorkflowId, WorkflowOsError,
};

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
