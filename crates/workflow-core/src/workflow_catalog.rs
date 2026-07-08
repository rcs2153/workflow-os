use std::fmt;
use std::path::{Component, Path};
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    ActorId, ApprovalReferenceId, EventId, EvidenceReferenceId, RedactionMetadata, SchemaVersion,
    SpecContentHash, Timestamp, ValidationReferenceId, WorkReportId, WorkReportSensitivity,
    WorkflowId, WorkflowOsError,
};

const CATALOG_ID_MAX_BYTES: usize = 128;
const CATALOG_PATH_MAX_BYTES: usize = 192;
const CATALOG_TEXT_MAX_BYTES: usize = 512;
const CATALOG_REFERENCE_MAX_COUNT: usize = 64;
const CATALOG_REDACTION_FIELD_MAX_BYTES: usize = 128;
const CATALOG_REDACTION_REASON_MAX_BYTES: usize = 512;
const CATALOG_REDACTION_MAX_ENTRIES: usize = 64;

/// Identifier for a workflow catalog record.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct WorkflowCatalogRecordId(String);

impl WorkflowCatalogRecordId {
    /// Creates a validated workflow catalog record id.
    ///
    /// # Errors
    ///
    /// Returns an error when the id is empty, too long, contains invalid
    /// characters, or looks like a secret.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        validate_identifier("WorkflowCatalogRecordId", &value)?;
        Ok(Self(value))
    }

    /// Returns the id as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for WorkflowCatalogRecordId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Debug for WorkflowCatalogRecordId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("WorkflowCatalogRecordId")
            .field(&"[REDACTED]")
            .finish()
    }
}

impl From<WorkflowCatalogRecordId> for String {
    fn from(value: WorkflowCatalogRecordId) -> Self {
        value.0
    }
}

impl TryFrom<String> for WorkflowCatalogRecordId {
    type Error = WorkflowOsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for WorkflowCatalogRecordId {
    type Err = WorkflowOsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Identifier for a workflow stewardship decision.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct WorkflowStewardshipDecisionId(String);

impl WorkflowStewardshipDecisionId {
    /// Creates a validated workflow stewardship decision id.
    ///
    /// # Errors
    ///
    /// Returns an error when the id is empty, too long, contains invalid
    /// characters, or looks like a secret.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        validate_identifier("WorkflowStewardshipDecisionId", &value)?;
        Ok(Self(value))
    }

    /// Returns the id as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for WorkflowStewardshipDecisionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Debug for WorkflowStewardshipDecisionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("WorkflowStewardshipDecisionId")
            .field(&"[REDACTED]")
            .finish()
    }
}

impl From<WorkflowStewardshipDecisionId> for String {
    fn from(value: WorkflowStewardshipDecisionId) -> Self {
        value.0
    }
}

impl TryFrom<String> for WorkflowStewardshipDecisionId {
    type Error = WorkflowOsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for WorkflowStewardshipDecisionId {
    type Err = WorkflowOsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Identifier for workflow archive metadata.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct WorkflowArchiveRecordId(String);

impl WorkflowArchiveRecordId {
    /// Creates a validated workflow archive record id.
    ///
    /// # Errors
    ///
    /// Returns an error when the id is empty, too long, contains invalid
    /// characters, or looks like a secret.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        validate_identifier("WorkflowArchiveRecordId", &value)?;
        Ok(Self(value))
    }

    /// Returns the id as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for WorkflowArchiveRecordId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Debug for WorkflowArchiveRecordId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("WorkflowArchiveRecordId")
            .field(&"[REDACTED]")
            .finish()
    }
}

impl From<WorkflowArchiveRecordId> for String {
    fn from(value: WorkflowArchiveRecordId) -> Self {
        value.0
    }
}

impl TryFrom<String> for WorkflowArchiveRecordId {
    type Error = WorkflowOsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for WorkflowArchiveRecordId {
    type Err = WorkflowOsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Lifecycle status for a workflow catalog record.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowLifecycleStatus {
    /// Draft exists but is not active.
    Draft,
    /// Draft or workflow is waiting for review.
    ReviewPending,
    /// Steward approval exists but active placement may be separate.
    Approved,
    /// Workflow is active and loader-visible.
    Active,
    /// Workflow or draft has been superseded by a newer active definition.
    Superseded,
    /// Workflow or draft has been archived.
    Archived,
    /// Workflow is deprecated but still represented.
    Deprecated,
    /// Workflow proposal was rejected.
    Rejected,
}

/// Bounded steward decision kind for workflow authoring.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowStewardshipDecisionKind {
    /// Draft was created.
    DraftCreated,
    /// Review was requested.
    ReviewRequested,
    /// Draft was approved for a separate promotion step.
    ApprovedForPromotion,
    /// Draft was rejected.
    Rejected,
    /// Draft needs changes before promotion.
    NeedsChanges,
    /// Draft was promoted to an active workflow file.
    Promoted,
    /// Draft was archived.
    Archived,
    /// Draft or workflow was superseded.
    Superseded,
}

/// Definition used to construct and deserialize workflow catalog records.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct WorkflowCatalogRecordDefinition {
    /// Catalog record id.
    pub record_id: WorkflowCatalogRecordId,
    /// Workflow id represented by the catalog record.
    pub workflow_id: WorkflowId,
    /// Repository-relative active workflow path.
    pub workflow_path: String,
    /// Active workflow content hash.
    pub workflow_content_hash: SpecContentHash,
    /// Workflow schema version.
    pub schema_version: SchemaVersion,
    /// Catalog lifecycle status.
    pub lifecycle_status: WorkflowLifecycleStatus,
    /// Source recommendation id, if any.
    pub source_recommendation_id: Option<String>,
    /// Source draft path, if any.
    pub source_draft_path: Option<String>,
    /// Archived draft path, if any.
    pub archived_draft_path: Option<String>,
    /// Current owner, if known.
    pub owner: Option<ActorId>,
    /// Current escalation contact, if known.
    pub escalation_contact: Option<ActorId>,
    /// Bounded authority scope summary.
    pub authority_scope: Option<String>,
    /// Bounded evidence/check/report posture summary.
    pub evidence_check_report_posture: Option<String>,
    /// Bounded side-effect posture summary.
    pub side_effect_posture: Option<String>,
    /// Latest stewardship decision id, if any.
    pub latest_stewardship_decision_id: Option<WorkflowStewardshipDecisionId>,
    /// Latest promotion decision id, if any.
    pub latest_promotion_decision_id: Option<WorkflowStewardshipDecisionId>,
    /// Latest archive record id, if any.
    pub latest_archive_record_id: Option<WorkflowArchiveRecordId>,
    /// Creation timestamp.
    pub created_at: Timestamp,
    /// Last updated timestamp.
    pub updated_at: Timestamp,
    /// Catalog record sensitivity.
    pub sensitivity: WorkReportSensitivity,
    /// Redaction metadata for bounded catalog fields.
    pub redaction: RedactionMetadata,
}

/// Domain-neutral workflow catalog record.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct WorkflowCatalogRecord {
    record_id: WorkflowCatalogRecordId,
    workflow_id: WorkflowId,
    workflow_path: String,
    workflow_content_hash: SpecContentHash,
    schema_version: SchemaVersion,
    lifecycle_status: WorkflowLifecycleStatus,
    source_recommendation_id: Option<String>,
    source_draft_path: Option<String>,
    archived_draft_path: Option<String>,
    owner: Option<ActorId>,
    escalation_contact: Option<ActorId>,
    authority_scope: Option<String>,
    evidence_check_report_posture: Option<String>,
    side_effect_posture: Option<String>,
    latest_stewardship_decision_id: Option<WorkflowStewardshipDecisionId>,
    latest_promotion_decision_id: Option<WorkflowStewardshipDecisionId>,
    latest_archive_record_id: Option<WorkflowArchiveRecordId>,
    created_at: Timestamp,
    updated_at: Timestamp,
    sensitivity: WorkReportSensitivity,
    redaction: RedactionMetadata,
}

impl WorkflowCatalogRecord {
    /// Creates a validated workflow catalog record.
    ///
    /// # Errors
    ///
    /// Returns an error when required identity, paths, bounded summaries, or
    /// redaction metadata are invalid.
    pub fn new(definition: WorkflowCatalogRecordDefinition) -> Result<Self, WorkflowOsError> {
        validate_relative_path("workflow catalog workflow path", &definition.workflow_path)?;
        validate_optional_path(
            "workflow catalog source draft path",
            definition.source_draft_path.as_ref(),
        )?;
        validate_optional_path(
            "workflow catalog archived draft path",
            definition.archived_draft_path.as_ref(),
        )?;
        validate_optional_reference_text(
            "workflow catalog source recommendation id",
            definition.source_recommendation_id.as_ref(),
        )?;
        validate_optional_text(
            "workflow catalog authority scope",
            definition.authority_scope.as_ref(),
        )?;
        validate_optional_text(
            "workflow catalog evidence/check/report posture",
            definition.evidence_check_report_posture.as_ref(),
        )?;
        validate_optional_text(
            "workflow catalog side-effect posture",
            definition.side_effect_posture.as_ref(),
        )?;
        validate_catalog_redaction_metadata(&definition.redaction)?;

        Ok(Self {
            record_id: definition.record_id,
            workflow_id: definition.workflow_id,
            workflow_path: definition.workflow_path,
            workflow_content_hash: definition.workflow_content_hash,
            schema_version: definition.schema_version,
            lifecycle_status: definition.lifecycle_status,
            source_recommendation_id: definition.source_recommendation_id,
            source_draft_path: definition.source_draft_path,
            archived_draft_path: definition.archived_draft_path,
            owner: definition.owner,
            escalation_contact: definition.escalation_contact,
            authority_scope: definition.authority_scope,
            evidence_check_report_posture: definition.evidence_check_report_posture,
            side_effect_posture: definition.side_effect_posture,
            latest_stewardship_decision_id: definition.latest_stewardship_decision_id,
            latest_promotion_decision_id: definition.latest_promotion_decision_id,
            latest_archive_record_id: definition.latest_archive_record_id,
            created_at: definition.created_at,
            updated_at: definition.updated_at,
            sensitivity: definition.sensitivity,
            redaction: definition.redaction,
        })
    }

    /// Validates this catalog record.
    ///
    /// # Errors
    ///
    /// Returns an error when record fields no longer satisfy model rules.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_relative_path("workflow catalog workflow path", &self.workflow_path)?;
        validate_optional_path(
            "workflow catalog source draft path",
            self.source_draft_path.as_ref(),
        )?;
        validate_optional_path(
            "workflow catalog archived draft path",
            self.archived_draft_path.as_ref(),
        )?;
        validate_optional_reference_text(
            "workflow catalog source recommendation id",
            self.source_recommendation_id.as_ref(),
        )?;
        validate_optional_text(
            "workflow catalog authority scope",
            self.authority_scope.as_ref(),
        )?;
        validate_optional_text(
            "workflow catalog evidence/check/report posture",
            self.evidence_check_report_posture.as_ref(),
        )?;
        validate_optional_text(
            "workflow catalog side-effect posture",
            self.side_effect_posture.as_ref(),
        )?;
        validate_catalog_redaction_metadata(&self.redaction)
    }

    /// Returns the catalog record id.
    #[must_use]
    pub const fn record_id(&self) -> &WorkflowCatalogRecordId {
        &self.record_id
    }

    /// Returns the workflow id.
    #[must_use]
    pub const fn workflow_id(&self) -> &WorkflowId {
        &self.workflow_id
    }

    /// Returns the repository-relative workflow path.
    #[must_use]
    pub fn workflow_path(&self) -> &str {
        &self.workflow_path
    }

    /// Returns the workflow content hash.
    #[must_use]
    pub const fn workflow_content_hash(&self) -> &SpecContentHash {
        &self.workflow_content_hash
    }

    /// Returns the lifecycle status.
    #[must_use]
    pub const fn lifecycle_status(&self) -> WorkflowLifecycleStatus {
        self.lifecycle_status
    }

    /// Returns catalog sensitivity.
    #[must_use]
    pub const fn sensitivity(&self) -> WorkReportSensitivity {
        self.sensitivity
    }

    /// Returns redaction metadata.
    #[must_use]
    pub const fn redaction(&self) -> &RedactionMetadata {
        &self.redaction
    }
}

impl fmt::Debug for WorkflowCatalogRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkflowCatalogRecord")
            .field("record_id", &self.record_id)
            .field("workflow_id", &self.workflow_id)
            .field("workflow_path", &self.workflow_path)
            .field("workflow_content_hash", &self.workflow_content_hash)
            .field("schema_version", &self.schema_version)
            .field("lifecycle_status", &self.lifecycle_status)
            .field("source_recommendation_id", &"[REDACTED]")
            .field("source_draft_path", &self.source_draft_path)
            .field("archived_draft_path", &self.archived_draft_path)
            .field("owner", &self.owner)
            .field("escalation_contact", &self.escalation_contact)
            .field("authority_scope", &"[REDACTED]")
            .field("evidence_check_report_posture", &"[REDACTED]")
            .field("side_effect_posture", &"[REDACTED]")
            .field(
                "latest_stewardship_decision_id",
                &self.latest_stewardship_decision_id,
            )
            .field(
                "latest_promotion_decision_id",
                &self.latest_promotion_decision_id,
            )
            .field("latest_archive_record_id", &self.latest_archive_record_id)
            .field("created_at", &self.created_at)
            .field("updated_at", &self.updated_at)
            .field("sensitivity", &self.sensitivity)
            .field(
                "redaction",
                &RedactedRedactionMetadataDebug(&self.redaction),
            )
            .finish()
    }
}

impl<'de> Deserialize<'de> for WorkflowCatalogRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let definition = WorkflowCatalogRecordDefinition::deserialize(deserializer)?;
        Self::new(definition).map_err(serde::de::Error::custom)
    }
}

/// Definition used to construct and deserialize stewardship records.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct WorkflowStewardshipRecordDefinition {
    /// Stewardship decision id.
    pub decision_id: WorkflowStewardshipDecisionId,
    /// Stewardship decision kind.
    pub decision_kind: WorkflowStewardshipDecisionKind,
    /// Workflow id.
    pub workflow_id: WorkflowId,
    /// Draft path, if relevant.
    pub draft_path: Option<String>,
    /// Active workflow path, if relevant.
    pub active_workflow_path: Option<String>,
    /// Archive path, if relevant.
    pub archive_path: Option<String>,
    /// Candidate content hash.
    pub candidate_content_hash: SpecContentHash,
    /// Active content hash, if relevant.
    pub active_content_hash: Option<SpecContentHash>,
    /// Reviewer or steward actor.
    pub reviewer: ActorId,
    /// Decision timestamp.
    pub decided_at: Timestamp,
    /// Bounded non-secret reason summary.
    pub reason_summary: Option<String>,
    /// Preflight validation reference.
    pub preflight_reference: Option<ValidationReferenceId>,
    /// Steward review validation reference.
    pub steward_review_reference: Option<ValidationReferenceId>,
    /// Evidence references considered.
    pub evidence_references: Vec<EvidenceReferenceId>,
    /// Approval references considered.
    pub approval_references: Vec<ApprovalReferenceId>,
    /// Policy decision event references considered.
    pub policy_decision_references: Vec<EventId>,
    /// Validation diagnostic references considered.
    pub validation_references: Vec<ValidationReferenceId>,
    /// Work report references considered.
    pub work_report_references: Vec<WorkReportId>,
    /// Bounded known limitations.
    pub known_limitations: Vec<String>,
    /// Bounded strict non-goals.
    pub strict_non_goals: Vec<String>,
    /// Stewardship record sensitivity.
    pub sensitivity: WorkReportSensitivity,
    /// Redaction metadata.
    pub redaction: RedactionMetadata,
}

/// Durable-model shape for workflow stewardship decisions.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct WorkflowStewardshipRecord {
    decision_id: WorkflowStewardshipDecisionId,
    decision_kind: WorkflowStewardshipDecisionKind,
    workflow_id: WorkflowId,
    draft_path: Option<String>,
    active_workflow_path: Option<String>,
    archive_path: Option<String>,
    candidate_content_hash: SpecContentHash,
    active_content_hash: Option<SpecContentHash>,
    reviewer: ActorId,
    decided_at: Timestamp,
    reason_summary: Option<String>,
    preflight_reference: Option<ValidationReferenceId>,
    steward_review_reference: Option<ValidationReferenceId>,
    evidence_references: Vec<EvidenceReferenceId>,
    approval_references: Vec<ApprovalReferenceId>,
    policy_decision_references: Vec<EventId>,
    validation_references: Vec<ValidationReferenceId>,
    work_report_references: Vec<WorkReportId>,
    known_limitations: Vec<String>,
    strict_non_goals: Vec<String>,
    sensitivity: WorkReportSensitivity,
    redaction: RedactionMetadata,
}

impl WorkflowStewardshipRecord {
    /// Creates a validated stewardship record.
    ///
    /// # Errors
    ///
    /// Returns an error when paths, bounded summaries, references, or
    /// redaction metadata are invalid.
    pub fn new(definition: WorkflowStewardshipRecordDefinition) -> Result<Self, WorkflowOsError> {
        validate_optional_path(
            "workflow stewardship draft path",
            definition.draft_path.as_ref(),
        )?;
        validate_optional_path(
            "workflow stewardship active workflow path",
            definition.active_workflow_path.as_ref(),
        )?;
        validate_optional_path(
            "workflow stewardship archive path",
            definition.archive_path.as_ref(),
        )?;
        validate_optional_text(
            "workflow stewardship reason summary",
            definition.reason_summary.as_ref(),
        )?;
        validate_text_list(
            "workflow stewardship known limitation",
            &definition.known_limitations,
        )?;
        validate_text_list(
            "workflow stewardship strict non-goal",
            &definition.strict_non_goals,
        )?;
        validate_reference_counts(
            definition.evidence_references.len(),
            "workflow stewardship evidence references",
        )?;
        validate_reference_counts(
            definition.approval_references.len(),
            "workflow stewardship approval references",
        )?;
        validate_reference_counts(
            definition.policy_decision_references.len(),
            "workflow stewardship policy decision references",
        )?;
        validate_reference_counts(
            definition.validation_references.len(),
            "workflow stewardship validation references",
        )?;
        validate_reference_counts(
            definition.work_report_references.len(),
            "workflow stewardship work report references",
        )?;
        validate_catalog_redaction_metadata(&definition.redaction)?;

        Ok(Self {
            decision_id: definition.decision_id,
            decision_kind: definition.decision_kind,
            workflow_id: definition.workflow_id,
            draft_path: definition.draft_path,
            active_workflow_path: definition.active_workflow_path,
            archive_path: definition.archive_path,
            candidate_content_hash: definition.candidate_content_hash,
            active_content_hash: definition.active_content_hash,
            reviewer: definition.reviewer,
            decided_at: definition.decided_at,
            reason_summary: definition.reason_summary,
            preflight_reference: definition.preflight_reference,
            steward_review_reference: definition.steward_review_reference,
            evidence_references: definition.evidence_references,
            approval_references: definition.approval_references,
            policy_decision_references: definition.policy_decision_references,
            validation_references: definition.validation_references,
            work_report_references: definition.work_report_references,
            known_limitations: definition.known_limitations,
            strict_non_goals: definition.strict_non_goals,
            sensitivity: definition.sensitivity,
            redaction: definition.redaction,
        })
    }

    /// Returns the stewardship decision id.
    #[must_use]
    pub const fn decision_id(&self) -> &WorkflowStewardshipDecisionId {
        &self.decision_id
    }

    /// Returns the stewardship decision kind.
    #[must_use]
    pub const fn decision_kind(&self) -> WorkflowStewardshipDecisionKind {
        self.decision_kind
    }

    /// Returns the workflow id.
    #[must_use]
    pub const fn workflow_id(&self) -> &WorkflowId {
        &self.workflow_id
    }

    /// Returns the candidate content hash.
    #[must_use]
    pub const fn candidate_content_hash(&self) -> &SpecContentHash {
        &self.candidate_content_hash
    }

    /// Returns record sensitivity.
    #[must_use]
    pub const fn sensitivity(&self) -> WorkReportSensitivity {
        self.sensitivity
    }
}

impl fmt::Debug for WorkflowStewardshipRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkflowStewardshipRecord")
            .field("decision_id", &self.decision_id)
            .field("decision_kind", &self.decision_kind)
            .field("workflow_id", &self.workflow_id)
            .field("draft_path", &self.draft_path)
            .field("active_workflow_path", &self.active_workflow_path)
            .field("archive_path", &self.archive_path)
            .field("candidate_content_hash", &self.candidate_content_hash)
            .field("active_content_hash", &self.active_content_hash)
            .field("reviewer", &self.reviewer)
            .field("decided_at", &self.decided_at)
            .field("reason_summary", &"[REDACTED]")
            .field("preflight_reference", &self.preflight_reference)
            .field("steward_review_reference", &self.steward_review_reference)
            .field("evidence_reference_count", &self.evidence_references.len())
            .field("approval_reference_count", &self.approval_references.len())
            .field(
                "policy_decision_reference_count",
                &self.policy_decision_references.len(),
            )
            .field(
                "validation_reference_count",
                &self.validation_references.len(),
            )
            .field(
                "work_report_reference_count",
                &self.work_report_references.len(),
            )
            .field("known_limitation_count", &self.known_limitations.len())
            .field("strict_non_goal_count", &self.strict_non_goals.len())
            .field("sensitivity", &self.sensitivity)
            .field(
                "redaction",
                &RedactedRedactionMetadataDebug(&self.redaction),
            )
            .finish()
    }
}

impl<'de> Deserialize<'de> for WorkflowStewardshipRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let definition = WorkflowStewardshipRecordDefinition::deserialize(deserializer)?;
        Self::new(definition).map_err(serde::de::Error::custom)
    }
}

/// Definition used to construct and deserialize archive records.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct WorkflowArchiveRecordDefinition {
    /// Archive record id.
    pub archive_record_id: WorkflowArchiveRecordId,
    /// Original draft path.
    pub original_draft_path: String,
    /// Archive path.
    pub archive_path: String,
    /// Workflow id.
    pub workflow_id: WorkflowId,
    /// Draft content hash.
    pub draft_content_hash: SpecContentHash,
    /// Matching active workflow path, if any.
    pub active_workflow_path: Option<String>,
    /// Matching active workflow content hash, if any.
    pub active_workflow_content_hash: Option<SpecContentHash>,
    /// Prior draft status code.
    pub prior_draft_status: String,
    /// Archive actor.
    pub archive_actor: ActorId,
    /// Bounded archive reason summary.
    pub archive_reason_summary: Option<String>,
    /// Archive timestamp.
    pub archived_at: Timestamp,
    /// Validation reference for the archive action, if available.
    pub validation_reference: Option<ValidationReferenceId>,
    /// Related stewardship decision, if available.
    pub stewardship_decision_id: Option<WorkflowStewardshipDecisionId>,
    /// Archive record sensitivity.
    pub sensitivity: WorkReportSensitivity,
    /// Redaction metadata.
    pub redaction: RedactionMetadata,
}

/// Model-only archive metadata record.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct WorkflowArchiveRecord {
    archive_record_id: WorkflowArchiveRecordId,
    original_draft_path: String,
    archive_path: String,
    workflow_id: WorkflowId,
    draft_content_hash: SpecContentHash,
    active_workflow_path: Option<String>,
    active_workflow_content_hash: Option<SpecContentHash>,
    prior_draft_status: String,
    archive_actor: ActorId,
    archive_reason_summary: Option<String>,
    archived_at: Timestamp,
    validation_reference: Option<ValidationReferenceId>,
    stewardship_decision_id: Option<WorkflowStewardshipDecisionId>,
    sensitivity: WorkReportSensitivity,
    redaction: RedactionMetadata,
}

impl WorkflowArchiveRecord {
    /// Creates a validated archive metadata record.
    ///
    /// # Errors
    ///
    /// Returns an error when paths, status, bounded summaries, or redaction
    /// metadata are invalid.
    pub fn new(definition: WorkflowArchiveRecordDefinition) -> Result<Self, WorkflowOsError> {
        validate_relative_path(
            "workflow archive original draft path",
            &definition.original_draft_path,
        )?;
        validate_relative_path("workflow archive archive path", &definition.archive_path)?;
        validate_optional_path(
            "workflow archive active workflow path",
            definition.active_workflow_path.as_ref(),
        )?;
        validate_reference_text(
            "workflow archive prior draft status",
            &definition.prior_draft_status,
        )?;
        validate_optional_text(
            "workflow archive reason summary",
            definition.archive_reason_summary.as_ref(),
        )?;
        validate_catalog_redaction_metadata(&definition.redaction)?;

        Ok(Self {
            archive_record_id: definition.archive_record_id,
            original_draft_path: definition.original_draft_path,
            archive_path: definition.archive_path,
            workflow_id: definition.workflow_id,
            draft_content_hash: definition.draft_content_hash,
            active_workflow_path: definition.active_workflow_path,
            active_workflow_content_hash: definition.active_workflow_content_hash,
            prior_draft_status: definition.prior_draft_status,
            archive_actor: definition.archive_actor,
            archive_reason_summary: definition.archive_reason_summary,
            archived_at: definition.archived_at,
            validation_reference: definition.validation_reference,
            stewardship_decision_id: definition.stewardship_decision_id,
            sensitivity: definition.sensitivity,
            redaction: definition.redaction,
        })
    }

    /// Returns the archive record id.
    #[must_use]
    pub const fn archive_record_id(&self) -> &WorkflowArchiveRecordId {
        &self.archive_record_id
    }

    /// Returns the original draft path.
    #[must_use]
    pub fn original_draft_path(&self) -> &str {
        &self.original_draft_path
    }

    /// Returns the archive path.
    #[must_use]
    pub fn archive_path(&self) -> &str {
        &self.archive_path
    }
}

impl fmt::Debug for WorkflowArchiveRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkflowArchiveRecord")
            .field("archive_record_id", &self.archive_record_id)
            .field("original_draft_path", &self.original_draft_path)
            .field("archive_path", &self.archive_path)
            .field("workflow_id", &self.workflow_id)
            .field("draft_content_hash", &self.draft_content_hash)
            .field("active_workflow_path", &self.active_workflow_path)
            .field(
                "active_workflow_content_hash",
                &self.active_workflow_content_hash,
            )
            .field("prior_draft_status", &self.prior_draft_status)
            .field("archive_actor", &self.archive_actor)
            .field("archive_reason_summary", &"[REDACTED]")
            .field("archived_at", &self.archived_at)
            .field("validation_reference", &self.validation_reference)
            .field("stewardship_decision_id", &self.stewardship_decision_id)
            .field("sensitivity", &self.sensitivity)
            .field(
                "redaction",
                &RedactedRedactionMetadataDebug(&self.redaction),
            )
            .finish()
    }
}

impl<'de> Deserialize<'de> for WorkflowArchiveRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let definition = WorkflowArchiveRecordDefinition::deserialize(deserializer)?;
        Self::new(definition).map_err(serde::de::Error::custom)
    }
}

fn validate_identifier(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "workflow_catalog.identifier.empty",
            format!("{type_name} cannot be empty"),
        ));
    }

    if value.len() > CATALOG_ID_MAX_BYTES {
        return Err(validation_error(
            "workflow_catalog.identifier.too_long",
            format!("{type_name} cannot exceed {CATALOG_ID_MAX_BYTES} bytes"),
        ));
    }

    let is_valid = value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/'));

    if !is_valid {
        return Err(validation_error(
            "workflow_catalog.identifier.invalid_character",
            format!("{type_name} contains an invalid character"),
        ));
    }

    validate_not_secret_like(type_name, value)
}

fn validate_relative_path(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "workflow_catalog.path.empty",
            format!("{type_name} cannot be empty"),
        ));
    }

    if value.len() > CATALOG_PATH_MAX_BYTES {
        return Err(validation_error(
            "workflow_catalog.path.too_long",
            format!("{type_name} cannot exceed {CATALOG_PATH_MAX_BYTES} bytes"),
        ));
    }

    validate_not_secret_like(type_name, value)?;

    let path = Path::new(value);
    if path.is_absolute() {
        return Err(validation_error(
            "workflow_catalog.path.absolute",
            format!("{type_name} must be repository-relative"),
        ));
    }

    let mut saw_normal = false;
    for component in path.components() {
        match component {
            Component::Normal(_) => saw_normal = true,
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err(validation_error(
                    "workflow_catalog.path.unsafe",
                    format!("{type_name} contains an unsafe path component"),
                ));
            }
        }
    }

    if !saw_normal {
        return Err(validation_error(
            "workflow_catalog.path.empty",
            format!("{type_name} cannot be empty"),
        ));
    }

    Ok(())
}

fn validate_optional_path(
    type_name: &'static str,
    value: Option<&String>,
) -> Result<(), WorkflowOsError> {
    if let Some(value) = value {
        validate_relative_path(type_name, value)?;
    }
    Ok(())
}

fn validate_reference_text(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "workflow_catalog.reference.empty",
            format!("{type_name} cannot be empty"),
        ));
    }

    if value.len() > CATALOG_TEXT_MAX_BYTES {
        return Err(validation_error(
            "workflow_catalog.reference.too_long",
            format!("{type_name} cannot exceed {CATALOG_TEXT_MAX_BYTES} bytes"),
        ));
    }

    validate_not_secret_like(type_name, value)
}

fn validate_optional_reference_text(
    type_name: &'static str,
    value: Option<&String>,
) -> Result<(), WorkflowOsError> {
    if let Some(value) = value {
        validate_reference_text(type_name, value)?;
    }
    Ok(())
}

fn validate_optional_text(
    type_name: &'static str,
    value: Option<&String>,
) -> Result<(), WorkflowOsError> {
    if let Some(value) = value {
        validate_bounded_text(type_name, value)?;
    }
    Ok(())
}

fn validate_text_list(type_name: &'static str, values: &[String]) -> Result<(), WorkflowOsError> {
    validate_reference_counts(values.len(), type_name)?;
    for value in values {
        validate_bounded_text(type_name, value)?;
    }
    Ok(())
}

fn validate_bounded_text(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "workflow_catalog.text.empty",
            format!("{type_name} cannot be empty"),
        ));
    }

    if value.len() > CATALOG_TEXT_MAX_BYTES {
        return Err(validation_error(
            "workflow_catalog.text.too_long",
            format!("{type_name} cannot exceed {CATALOG_TEXT_MAX_BYTES} bytes"),
        ));
    }

    validate_not_secret_like(type_name, value)
}

fn validate_reference_counts(count: usize, type_name: &'static str) -> Result<(), WorkflowOsError> {
    if count > CATALOG_REFERENCE_MAX_COUNT {
        return Err(validation_error(
            "workflow_catalog.references.too_many",
            format!("{type_name} cannot exceed {CATALOG_REFERENCE_MAX_COUNT} entries"),
        ));
    }
    Ok(())
}

fn validate_catalog_redaction_metadata(
    redaction: &RedactionMetadata,
) -> Result<(), WorkflowOsError> {
    if redaction.redacted_fields.len() > CATALOG_REDACTION_MAX_ENTRIES {
        return Err(validation_error(
            "workflow_catalog.redaction.too_many_fields",
            "workflow catalog redaction metadata contains too many fields",
        ));
    }

    if redaction.field_states.len() > CATALOG_REDACTION_MAX_ENTRIES {
        return Err(validation_error(
            "workflow_catalog.redaction.too_many_states",
            "workflow catalog redaction metadata contains too many field states",
        ));
    }

    for field in &redaction.redacted_fields {
        validate_redaction_field_name(field)?;
    }

    for state in &redaction.field_states {
        validate_redaction_field_name(&state.field)?;
        validate_redaction_reason(&state.reason)?;
    }

    Ok(())
}

fn validate_redaction_field_name(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "workflow_catalog.redaction.field.empty",
            "workflow catalog redaction field cannot be empty",
        ));
    }

    if value.len() > CATALOG_REDACTION_FIELD_MAX_BYTES {
        return Err(validation_error(
            "workflow_catalog.redaction.field.too_long",
            format!(
                "workflow catalog redaction field cannot exceed {CATALOG_REDACTION_FIELD_MAX_BYTES} bytes"
            ),
        ));
    }

    validate_not_secret_like("workflow catalog redaction field", value)
}

fn validate_redaction_reason(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "workflow_catalog.redaction.reason.empty",
            "workflow catalog redaction reason cannot be empty",
        ));
    }

    if value.len() > CATALOG_REDACTION_REASON_MAX_BYTES {
        return Err(validation_error(
            "workflow_catalog.redaction.reason.too_long",
            format!(
                "workflow catalog redaction reason cannot exceed {CATALOG_REDACTION_REASON_MAX_BYTES} bytes"
            ),
        ));
    }

    validate_not_secret_like("workflow catalog redaction reason", value)
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
            "workflow_catalog.secret_like_value",
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
