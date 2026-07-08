use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::{Component, Path};

use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    SchemaVersion, SpecContentHash, WorkflowArchiveRecord, WorkflowCatalogRecord, WorkflowId,
    WorkflowLifecycleStatus, WorkflowOsError, WorkflowStewardshipDecisionKind,
    WorkflowStewardshipRecord,
};

const INDEX_PATH_MAX_BYTES: usize = 192;
const INDEX_TEXT_MAX_BYTES: usize = 128;

/// Loader-visible active workflow summary supplied to the catalog index helper.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct WorkflowCatalogActiveWorkflowSummary {
    workflow_id: WorkflowId,
    workflow_path: String,
    workflow_content_hash: SpecContentHash,
    schema_version: SchemaVersion,
}

impl WorkflowCatalogActiveWorkflowSummary {
    /// Creates a validated active workflow summary.
    ///
    /// # Errors
    ///
    /// Returns an error when the supplied path is not safe to represent.
    pub fn new(
        workflow_id: WorkflowId,
        workflow_path: impl Into<String>,
        workflow_content_hash: SpecContentHash,
        schema_version: SchemaVersion,
    ) -> Result<Self, WorkflowOsError> {
        let workflow_path = workflow_path.into();
        validate_relative_path(
            "workflow catalog index active workflow path",
            &workflow_path,
        )?;
        Ok(Self {
            workflow_id,
            workflow_path,
            workflow_content_hash,
            schema_version,
        })
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

    /// Returns the schema version.
    #[must_use]
    pub const fn schema_version(&self) -> &SchemaVersion {
        &self.schema_version
    }
}

impl<'de> Deserialize<'de> for WorkflowCatalogActiveWorkflowSummary {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            workflow_id: WorkflowId,
            workflow_path: String,
            workflow_content_hash: SpecContentHash,
            schema_version: SchemaVersion,
        }

        let wire = Wire::deserialize(deserializer)?;
        Self::new(
            wire.workflow_id,
            wire.workflow_path,
            wire.workflow_content_hash,
            wire.schema_version,
        )
        .map_err(serde::de::Error::custom)
    }
}

impl fmt::Debug for WorkflowCatalogActiveWorkflowSummary {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkflowCatalogActiveWorkflowSummary")
            .field("workflow_id", &self.workflow_id)
            .field("workflow_path", &self.workflow_path)
            .field("workflow_content_hash", &self.workflow_content_hash)
            .field("schema_version", &self.schema_version)
            .finish()
    }
}

/// Inactive workflow draft summary supplied to the catalog index helper.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct WorkflowCatalogDraftSummary {
    workflow_id: WorkflowId,
    draft_path: String,
    draft_content_hash: SpecContentHash,
    draft_status: Option<String>,
}

impl WorkflowCatalogDraftSummary {
    /// Creates a validated workflow draft summary.
    ///
    /// # Errors
    ///
    /// Returns an error when the path or optional status is unsafe.
    pub fn new(
        workflow_id: WorkflowId,
        draft_path: impl Into<String>,
        draft_content_hash: SpecContentHash,
        draft_status: Option<String>,
    ) -> Result<Self, WorkflowOsError> {
        let draft_path = draft_path.into();
        validate_relative_path("workflow catalog index draft path", &draft_path)?;
        validate_optional_reference_text(
            "workflow catalog index draft status",
            draft_status.as_ref(),
        )?;
        Ok(Self {
            workflow_id,
            draft_path,
            draft_content_hash,
            draft_status,
        })
    }

    /// Returns the workflow id.
    #[must_use]
    pub const fn workflow_id(&self) -> &WorkflowId {
        &self.workflow_id
    }

    /// Returns the repository-relative draft path.
    #[must_use]
    pub fn draft_path(&self) -> &str {
        &self.draft_path
    }

    /// Returns the draft content hash.
    #[must_use]
    pub const fn draft_content_hash(&self) -> &SpecContentHash {
        &self.draft_content_hash
    }

    /// Returns the draft status code, if supplied.
    #[must_use]
    pub fn draft_status(&self) -> Option<&str> {
        self.draft_status.as_deref()
    }
}

impl<'de> Deserialize<'de> for WorkflowCatalogDraftSummary {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            workflow_id: WorkflowId,
            draft_path: String,
            draft_content_hash: SpecContentHash,
            draft_status: Option<String>,
        }

        let wire = Wire::deserialize(deserializer)?;
        Self::new(
            wire.workflow_id,
            wire.draft_path,
            wire.draft_content_hash,
            wire.draft_status,
        )
        .map_err(serde::de::Error::custom)
    }
}

impl fmt::Debug for WorkflowCatalogDraftSummary {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkflowCatalogDraftSummary")
            .field("workflow_id", &self.workflow_id)
            .field("draft_path", &self.draft_path)
            .field("draft_content_hash", &self.draft_content_hash)
            .field("draft_status", &self.draft_status)
            .finish()
    }
}

/// Archived workflow draft summary supplied to the catalog index helper.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct WorkflowCatalogArchivedDraftSummary {
    workflow_id: WorkflowId,
    original_draft_path: String,
    archive_path: String,
    draft_content_hash: SpecContentHash,
}

impl WorkflowCatalogArchivedDraftSummary {
    /// Creates a validated archived draft summary.
    ///
    /// # Errors
    ///
    /// Returns an error when either path is unsafe.
    pub fn new(
        workflow_id: WorkflowId,
        original_draft_path: impl Into<String>,
        archive_path: impl Into<String>,
        draft_content_hash: SpecContentHash,
    ) -> Result<Self, WorkflowOsError> {
        let original_draft_path = original_draft_path.into();
        let archive_path = archive_path.into();
        validate_relative_path(
            "workflow catalog index original draft path",
            &original_draft_path,
        )?;
        validate_relative_path("workflow catalog index archive path", &archive_path)?;
        Ok(Self {
            workflow_id,
            original_draft_path,
            archive_path,
            draft_content_hash,
        })
    }

    /// Returns the workflow id.
    #[must_use]
    pub const fn workflow_id(&self) -> &WorkflowId {
        &self.workflow_id
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

    /// Returns the archived draft content hash.
    #[must_use]
    pub const fn draft_content_hash(&self) -> &SpecContentHash {
        &self.draft_content_hash
    }
}

impl<'de> Deserialize<'de> for WorkflowCatalogArchivedDraftSummary {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            workflow_id: WorkflowId,
            original_draft_path: String,
            archive_path: String,
            draft_content_hash: SpecContentHash,
        }

        let wire = Wire::deserialize(deserializer)?;
        Self::new(
            wire.workflow_id,
            wire.original_draft_path,
            wire.archive_path,
            wire.draft_content_hash,
        )
        .map_err(serde::de::Error::custom)
    }
}

impl fmt::Debug for WorkflowCatalogArchivedDraftSummary {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkflowCatalogArchivedDraftSummary")
            .field("workflow_id", &self.workflow_id)
            .field("original_draft_path", &self.original_draft_path)
            .field("archive_path", &self.archive_path)
            .field("draft_content_hash", &self.draft_content_hash)
            .finish()
    }
}

/// Explicit input for pure workflow catalog indexing.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorkflowCatalogIndexInput {
    active_workflows: Vec<WorkflowCatalogActiveWorkflowSummary>,
    drafts: Vec<WorkflowCatalogDraftSummary>,
    archived_drafts: Vec<WorkflowCatalogArchivedDraftSummary>,
    catalog_records: Vec<WorkflowCatalogRecord>,
    stewardship_records: Vec<WorkflowStewardshipRecord>,
    archive_records: Vec<WorkflowArchiveRecord>,
    require_catalog_records_for_active_workflows: bool,
}

impl WorkflowCatalogIndexInput {
    /// Creates empty index input.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets active workflow summaries.
    #[must_use]
    pub fn with_active_workflows(
        mut self,
        active_workflows: Vec<WorkflowCatalogActiveWorkflowSummary>,
    ) -> Self {
        self.active_workflows = active_workflows;
        self
    }

    /// Sets draft summaries.
    #[must_use]
    pub fn with_drafts(mut self, drafts: Vec<WorkflowCatalogDraftSummary>) -> Self {
        self.drafts = drafts;
        self
    }

    /// Sets archived draft summaries.
    #[must_use]
    pub fn with_archived_drafts(
        mut self,
        archived_drafts: Vec<WorkflowCatalogArchivedDraftSummary>,
    ) -> Self {
        self.archived_drafts = archived_drafts;
        self
    }

    /// Sets catalog records.
    #[must_use]
    pub fn with_catalog_records(mut self, catalog_records: Vec<WorkflowCatalogRecord>) -> Self {
        self.catalog_records = catalog_records;
        self
    }

    /// Sets stewardship records.
    #[must_use]
    pub fn with_stewardship_records(
        mut self,
        stewardship_records: Vec<WorkflowStewardshipRecord>,
    ) -> Self {
        self.stewardship_records = stewardship_records;
        self
    }

    /// Sets archive records.
    #[must_use]
    pub fn with_archive_records(mut self, archive_records: Vec<WorkflowArchiveRecord>) -> Self {
        self.archive_records = archive_records;
        self
    }

    /// Requires active workflow files to have matching catalog records.
    #[must_use]
    pub const fn require_catalog_records_for_active_workflows(mut self, required: bool) -> Self {
        self.require_catalog_records_for_active_workflows = required;
        self
    }
}

/// Deterministic severity for a catalog conflict disclosure.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowCatalogConflictSeverity {
    /// Deterministic mismatch that should block future command integration.
    Blocker,
    /// Missing or overlapping metadata that should be reviewed.
    Warning,
    /// Bounded inventory fact.
    Info,
}

/// Deterministic catalog conflict kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowCatalogConflictKind {
    /// More than one active workflow summary uses the same workflow id.
    DuplicateActiveWorkflowId,
    /// More than one active workflow summary uses the same path.
    DuplicateActiveWorkflowPath,
    /// An active workflow file has no matching catalog record.
    ActiveWorkflowMissingCatalogRecord,
    /// An active catalog record references no active workflow file.
    CatalogActiveMissingWorkflowFile,
    /// Catalog path differs from the active workflow path for the same id.
    CatalogActivePathMismatch,
    /// Catalog hash differs from the active workflow hash for the same id.
    CatalogActiveHashMismatch,
    /// Draft summary hash differs from an associated stewardship candidate hash.
    DraftStewardshipHashMismatch,
    /// Archive record references no archived draft summary.
    ArchiveRecordMissingArchivedDraft,
    /// Archive record path differs from the archived draft summary.
    ArchivePathMismatch,
    /// Archive record hash differs from the archived draft summary.
    ArchiveHashMismatch,
    /// Catalog record is missing an owner.
    MissingOwner,
    /// Catalog record is missing an escalation contact.
    MissingEscalationContact,
    /// Catalog record is missing a stewardship decision reference.
    MissingLatestStewardshipDecision,
    /// Catalog record is missing side-effect posture.
    MissingSideEffectPosture,
}

/// Bounded source category for a catalog conflict disclosure.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowCatalogConflictSource {
    /// Active workflow summary.
    ActiveWorkflow,
    /// Draft summary.
    Draft,
    /// Archived draft summary.
    ArchivedDraft,
    /// Catalog record.
    CatalogRecord,
    /// Stewardship record.
    StewardshipRecord,
    /// Archive record.
    ArchiveRecord,
}

/// Deterministic, non-mutating catalog conflict disclosure.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct WorkflowCatalogConflict {
    severity: WorkflowCatalogConflictSeverity,
    kind: WorkflowCatalogConflictKind,
    workflow_id: Option<WorkflowId>,
    source: WorkflowCatalogConflictSource,
    source_reference: String,
}

impl WorkflowCatalogConflict {
    fn new(
        severity: WorkflowCatalogConflictSeverity,
        kind: WorkflowCatalogConflictKind,
        workflow_id: Option<WorkflowId>,
        source: WorkflowCatalogConflictSource,
        source_reference: impl Into<String>,
    ) -> Result<Self, WorkflowOsError> {
        let source_reference = source_reference.into();
        validate_reference_text(
            "workflow catalog conflict source reference",
            &source_reference,
        )?;
        Ok(Self {
            severity,
            kind,
            workflow_id,
            source,
            source_reference,
        })
    }

    /// Returns the conflict severity.
    #[must_use]
    pub const fn severity(&self) -> WorkflowCatalogConflictSeverity {
        self.severity
    }

    /// Returns the conflict kind.
    #[must_use]
    pub const fn kind(&self) -> WorkflowCatalogConflictKind {
        self.kind
    }

    /// Returns the workflow id associated with this conflict, if known.
    #[must_use]
    pub const fn workflow_id(&self) -> Option<&WorkflowId> {
        self.workflow_id.as_ref()
    }

    /// Returns the source category.
    #[must_use]
    pub const fn source(&self) -> WorkflowCatalogConflictSource {
        self.source
    }

    /// Returns a bounded source reference.
    #[must_use]
    pub fn source_reference(&self) -> &str {
        &self.source_reference
    }
}

impl<'de> Deserialize<'de> for WorkflowCatalogConflict {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            severity: WorkflowCatalogConflictSeverity,
            kind: WorkflowCatalogConflictKind,
            workflow_id: Option<WorkflowId>,
            source: WorkflowCatalogConflictSource,
            source_reference: String,
        }

        let wire = Wire::deserialize(deserializer)?;
        Self::new(
            wire.severity,
            wire.kind,
            wire.workflow_id,
            wire.source,
            wire.source_reference,
        )
        .map_err(serde::de::Error::custom)
    }
}

impl fmt::Debug for WorkflowCatalogConflict {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkflowCatalogConflict")
            .field("severity", &self.severity)
            .field("kind", &self.kind)
            .field("workflow_id", &self.workflow_id)
            .field("source", &self.source)
            .field("source_reference", &"[REDACTED]")
            .finish()
    }
}

/// Pure, in-memory workflow catalog index.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct WorkflowCatalogIndex {
    active_workflows: Vec<WorkflowCatalogActiveWorkflowSummary>,
    drafts: Vec<WorkflowCatalogDraftSummary>,
    archived_drafts: Vec<WorkflowCatalogArchivedDraftSummary>,
    catalog_records: Vec<WorkflowCatalogRecord>,
    stewardship_records: Vec<WorkflowStewardshipRecord>,
    archive_records: Vec<WorkflowArchiveRecord>,
    conflicts: Vec<WorkflowCatalogConflict>,
}

impl WorkflowCatalogIndex {
    /// Returns active workflow summaries in deterministic order.
    #[must_use]
    pub fn active_workflows(&self) -> &[WorkflowCatalogActiveWorkflowSummary] {
        &self.active_workflows
    }

    /// Returns draft summaries in deterministic order.
    #[must_use]
    pub fn drafts(&self) -> &[WorkflowCatalogDraftSummary] {
        &self.drafts
    }

    /// Returns archived draft summaries in deterministic order.
    #[must_use]
    pub fn archived_drafts(&self) -> &[WorkflowCatalogArchivedDraftSummary] {
        &self.archived_drafts
    }

    /// Returns catalog records in deterministic order.
    #[must_use]
    pub fn catalog_records(&self) -> &[WorkflowCatalogRecord] {
        &self.catalog_records
    }

    /// Returns stewardship records in deterministic order.
    #[must_use]
    pub fn stewardship_records(&self) -> &[WorkflowStewardshipRecord] {
        &self.stewardship_records
    }

    /// Returns archive records in deterministic order.
    #[must_use]
    pub fn archive_records(&self) -> &[WorkflowArchiveRecord] {
        &self.archive_records
    }

    /// Returns conflict disclosures in deterministic order.
    #[must_use]
    pub fn conflicts(&self) -> &[WorkflowCatalogConflict] {
        &self.conflicts
    }

    /// Returns the number of conflicts with the given severity.
    #[must_use]
    pub fn conflict_count_by_severity(&self, severity: WorkflowCatalogConflictSeverity) -> usize {
        self.conflicts
            .iter()
            .filter(|conflict| conflict.severity == severity)
            .count()
    }
}

impl fmt::Debug for WorkflowCatalogIndex {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkflowCatalogIndex")
            .field("active_workflow_count", &self.active_workflows.len())
            .field("draft_count", &self.drafts.len())
            .field("archived_draft_count", &self.archived_drafts.len())
            .field("catalog_record_count", &self.catalog_records.len())
            .field("stewardship_record_count", &self.stewardship_records.len())
            .field("archive_record_count", &self.archive_records.len())
            .field("conflict_count", &self.conflicts.len())
            .finish()
    }
}

/// Builds a deterministic in-memory workflow catalog index and conflict set.
///
/// # Errors
///
/// Returns an error only when supplied input cannot be represented safely.
pub fn build_workflow_catalog_index(
    input: WorkflowCatalogIndexInput,
) -> Result<WorkflowCatalogIndex, WorkflowOsError> {
    let mut active_workflows = input.active_workflows;
    let mut drafts = input.drafts;
    let mut archived_drafts = input.archived_drafts;
    let mut catalog_records = input.catalog_records;
    let mut stewardship_records = input.stewardship_records;
    let mut archive_records = input.archive_records;

    active_workflows.sort_by(|left, right| {
        (left.workflow_id.as_str(), left.workflow_path.as_str())
            .cmp(&(right.workflow_id.as_str(), right.workflow_path.as_str()))
    });
    drafts.sort_by(|left, right| {
        (left.workflow_id.as_str(), left.draft_path.as_str())
            .cmp(&(right.workflow_id.as_str(), right.draft_path.as_str()))
    });
    archived_drafts.sort_by(|left, right| {
        (left.workflow_id.as_str(), left.archive_path.as_str())
            .cmp(&(right.workflow_id.as_str(), right.archive_path.as_str()))
    });
    catalog_records.sort_by(|left, right| left.record_id().cmp(right.record_id()));
    stewardship_records.sort_by(|left, right| left.decision_id().cmp(right.decision_id()));
    archive_records.sort_by(|left, right| left.archive_record_id().cmp(right.archive_record_id()));

    let mut conflicts = Vec::new();
    detect_duplicate_active_workflows(&active_workflows, &mut conflicts)?;
    detect_catalog_conflicts(
        &active_workflows,
        &catalog_records,
        input.require_catalog_records_for_active_workflows,
        &mut conflicts,
    )?;
    detect_stewardship_conflicts(&drafts, &stewardship_records, &mut conflicts)?;
    detect_archive_conflicts(&archived_drafts, &archive_records, &mut conflicts)?;
    conflicts.sort_by(compare_conflicts);

    Ok(WorkflowCatalogIndex {
        active_workflows,
        drafts,
        archived_drafts,
        catalog_records,
        stewardship_records,
        archive_records,
        conflicts,
    })
}

fn detect_duplicate_active_workflows(
    active_workflows: &[WorkflowCatalogActiveWorkflowSummary],
    conflicts: &mut Vec<WorkflowCatalogConflict>,
) -> Result<(), WorkflowOsError> {
    let mut seen_ids = BTreeSet::new();
    let mut duplicate_ids = BTreeSet::new();
    let mut seen_paths = BTreeSet::new();
    let mut duplicate_paths = BTreeSet::new();

    for active in active_workflows {
        if !seen_ids.insert(active.workflow_id.clone()) {
            duplicate_ids.insert(active.workflow_id.clone());
        }
        if !seen_paths.insert(active.workflow_path.clone()) {
            duplicate_paths.insert(active.workflow_path.clone());
        }
    }

    for workflow_id in duplicate_ids {
        conflicts.push(WorkflowCatalogConflict::new(
            WorkflowCatalogConflictSeverity::Blocker,
            WorkflowCatalogConflictKind::DuplicateActiveWorkflowId,
            Some(workflow_id.clone()),
            WorkflowCatalogConflictSource::ActiveWorkflow,
            workflow_id.as_str(),
        )?);
    }

    for workflow_path in duplicate_paths {
        conflicts.push(WorkflowCatalogConflict::new(
            WorkflowCatalogConflictSeverity::Blocker,
            WorkflowCatalogConflictKind::DuplicateActiveWorkflowPath,
            None,
            WorkflowCatalogConflictSource::ActiveWorkflow,
            workflow_path,
        )?);
    }

    Ok(())
}

fn detect_catalog_conflicts(
    active_workflows: &[WorkflowCatalogActiveWorkflowSummary],
    catalog_records: &[WorkflowCatalogRecord],
    require_catalog_records_for_active_workflows: bool,
    conflicts: &mut Vec<WorkflowCatalogConflict>,
) -> Result<(), WorkflowOsError> {
    let active_by_workflow_id: BTreeMap<_, _> = active_workflows
        .iter()
        .map(|active| (active.workflow_id.clone(), active))
        .collect();
    let catalog_by_workflow_id: BTreeMap<_, _> = catalog_records
        .iter()
        .map(|record| (record.workflow_id().clone(), record))
        .collect();

    for active in active_workflows {
        if !catalog_by_workflow_id.contains_key(&active.workflow_id) {
            let severity = if require_catalog_records_for_active_workflows {
                WorkflowCatalogConflictSeverity::Blocker
            } else {
                WorkflowCatalogConflictSeverity::Warning
            };
            conflicts.push(WorkflowCatalogConflict::new(
                severity,
                WorkflowCatalogConflictKind::ActiveWorkflowMissingCatalogRecord,
                Some(active.workflow_id.clone()),
                WorkflowCatalogConflictSource::ActiveWorkflow,
                active.workflow_path.clone(),
            )?);
        }
    }

    for record in catalog_records {
        if record.lifecycle_status() == WorkflowLifecycleStatus::Active {
            if let Some(active) = active_by_workflow_id.get(record.workflow_id()) {
                if active.workflow_path() != record.workflow_path() {
                    conflicts.push(WorkflowCatalogConflict::new(
                        WorkflowCatalogConflictSeverity::Blocker,
                        WorkflowCatalogConflictKind::CatalogActivePathMismatch,
                        Some(record.workflow_id().clone()),
                        WorkflowCatalogConflictSource::CatalogRecord,
                        record.record_id().as_str(),
                    )?);
                }
                if active.workflow_content_hash() != record.workflow_content_hash() {
                    conflicts.push(WorkflowCatalogConflict::new(
                        WorkflowCatalogConflictSeverity::Blocker,
                        WorkflowCatalogConflictKind::CatalogActiveHashMismatch,
                        Some(record.workflow_id().clone()),
                        WorkflowCatalogConflictSource::CatalogRecord,
                        record.record_id().as_str(),
                    )?);
                }
            } else {
                conflicts.push(WorkflowCatalogConflict::new(
                    WorkflowCatalogConflictSeverity::Blocker,
                    WorkflowCatalogConflictKind::CatalogActiveMissingWorkflowFile,
                    Some(record.workflow_id().clone()),
                    WorkflowCatalogConflictSource::CatalogRecord,
                    record.record_id().as_str(),
                )?);
            }
        }

        if record.owner().is_none() {
            conflicts.push(WorkflowCatalogConflict::new(
                WorkflowCatalogConflictSeverity::Warning,
                WorkflowCatalogConflictKind::MissingOwner,
                Some(record.workflow_id().clone()),
                WorkflowCatalogConflictSource::CatalogRecord,
                record.record_id().as_str(),
            )?);
        }

        if record.escalation_contact().is_none() {
            conflicts.push(WorkflowCatalogConflict::new(
                WorkflowCatalogConflictSeverity::Warning,
                WorkflowCatalogConflictKind::MissingEscalationContact,
                Some(record.workflow_id().clone()),
                WorkflowCatalogConflictSource::CatalogRecord,
                record.record_id().as_str(),
            )?);
        }

        if record.latest_stewardship_decision_id().is_none() {
            conflicts.push(WorkflowCatalogConflict::new(
                WorkflowCatalogConflictSeverity::Warning,
                WorkflowCatalogConflictKind::MissingLatestStewardshipDecision,
                Some(record.workflow_id().clone()),
                WorkflowCatalogConflictSource::CatalogRecord,
                record.record_id().as_str(),
            )?);
        }

        if record.side_effect_posture().is_none() {
            conflicts.push(WorkflowCatalogConflict::new(
                WorkflowCatalogConflictSeverity::Warning,
                WorkflowCatalogConflictKind::MissingSideEffectPosture,
                Some(record.workflow_id().clone()),
                WorkflowCatalogConflictSource::CatalogRecord,
                record.record_id().as_str(),
            )?);
        }
    }

    Ok(())
}

fn detect_stewardship_conflicts(
    drafts: &[WorkflowCatalogDraftSummary],
    stewardship_records: &[WorkflowStewardshipRecord],
    conflicts: &mut Vec<WorkflowCatalogConflict>,
) -> Result<(), WorkflowOsError> {
    let drafts_by_path: BTreeMap<_, _> = drafts
        .iter()
        .map(|draft| (draft.draft_path.as_str(), draft))
        .collect();

    for record in stewardship_records {
        if matches!(
            record.decision_kind(),
            WorkflowStewardshipDecisionKind::ApprovedForPromotion
                | WorkflowStewardshipDecisionKind::Promoted
        ) {
            if let Some(draft_path) = record.draft_path() {
                if let Some(draft) = drafts_by_path.get(draft_path) {
                    if draft.draft_content_hash() != record.candidate_content_hash() {
                        conflicts.push(WorkflowCatalogConflict::new(
                            WorkflowCatalogConflictSeverity::Blocker,
                            WorkflowCatalogConflictKind::DraftStewardshipHashMismatch,
                            Some(record.workflow_id().clone()),
                            WorkflowCatalogConflictSource::StewardshipRecord,
                            record.decision_id().as_str(),
                        )?);
                    }
                }
            }
        }
    }

    Ok(())
}

fn detect_archive_conflicts(
    archived_drafts: &[WorkflowCatalogArchivedDraftSummary],
    archive_records: &[WorkflowArchiveRecord],
    conflicts: &mut Vec<WorkflowCatalogConflict>,
) -> Result<(), WorkflowOsError> {
    let archived_by_path: BTreeMap<_, _> = archived_drafts
        .iter()
        .map(|archived| (archived.archive_path.as_str(), archived))
        .collect();

    for record in archive_records {
        if let Some(archived) = archived_by_path.get(record.archive_path()) {
            if archived.original_draft_path() != record.original_draft_path() {
                conflicts.push(WorkflowCatalogConflict::new(
                    WorkflowCatalogConflictSeverity::Blocker,
                    WorkflowCatalogConflictKind::ArchivePathMismatch,
                    Some(record.workflow_id().clone()),
                    WorkflowCatalogConflictSource::ArchiveRecord,
                    record.archive_record_id().as_str(),
                )?);
            }
            if archived.draft_content_hash() != record.draft_content_hash() {
                conflicts.push(WorkflowCatalogConflict::new(
                    WorkflowCatalogConflictSeverity::Blocker,
                    WorkflowCatalogConflictKind::ArchiveHashMismatch,
                    Some(record.workflow_id().clone()),
                    WorkflowCatalogConflictSource::ArchiveRecord,
                    record.archive_record_id().as_str(),
                )?);
            }
        } else {
            conflicts.push(WorkflowCatalogConflict::new(
                WorkflowCatalogConflictSeverity::Blocker,
                WorkflowCatalogConflictKind::ArchiveRecordMissingArchivedDraft,
                Some(record.workflow_id().clone()),
                WorkflowCatalogConflictSource::ArchiveRecord,
                record.archive_record_id().as_str(),
            )?);
        }
    }

    Ok(())
}

fn compare_conflicts(
    left: &WorkflowCatalogConflict,
    right: &WorkflowCatalogConflict,
) -> std::cmp::Ordering {
    (
        severity_rank(left.severity),
        conflict_kind_rank(left.kind),
        left.workflow_id.as_ref().map(WorkflowId::as_str),
        source_rank(left.source),
        left.source_reference.as_str(),
    )
        .cmp(&(
            severity_rank(right.severity),
            conflict_kind_rank(right.kind),
            right.workflow_id.as_ref().map(WorkflowId::as_str),
            source_rank(right.source),
            right.source_reference.as_str(),
        ))
}

const fn severity_rank(severity: WorkflowCatalogConflictSeverity) -> u8 {
    match severity {
        WorkflowCatalogConflictSeverity::Blocker => 0,
        WorkflowCatalogConflictSeverity::Warning => 1,
        WorkflowCatalogConflictSeverity::Info => 2,
    }
}

const fn conflict_kind_rank(kind: WorkflowCatalogConflictKind) -> u8 {
    match kind {
        WorkflowCatalogConflictKind::DuplicateActiveWorkflowId => 0,
        WorkflowCatalogConflictKind::DuplicateActiveWorkflowPath => 1,
        WorkflowCatalogConflictKind::ActiveWorkflowMissingCatalogRecord => 2,
        WorkflowCatalogConflictKind::CatalogActiveMissingWorkflowFile => 3,
        WorkflowCatalogConflictKind::CatalogActivePathMismatch => 4,
        WorkflowCatalogConflictKind::CatalogActiveHashMismatch => 5,
        WorkflowCatalogConflictKind::DraftStewardshipHashMismatch => 6,
        WorkflowCatalogConflictKind::ArchiveRecordMissingArchivedDraft => 7,
        WorkflowCatalogConflictKind::ArchivePathMismatch => 8,
        WorkflowCatalogConflictKind::ArchiveHashMismatch => 9,
        WorkflowCatalogConflictKind::MissingOwner => 10,
        WorkflowCatalogConflictKind::MissingEscalationContact => 11,
        WorkflowCatalogConflictKind::MissingLatestStewardshipDecision => 12,
        WorkflowCatalogConflictKind::MissingSideEffectPosture => 13,
    }
}

const fn source_rank(source: WorkflowCatalogConflictSource) -> u8 {
    match source {
        WorkflowCatalogConflictSource::ActiveWorkflow => 0,
        WorkflowCatalogConflictSource::Draft => 1,
        WorkflowCatalogConflictSource::ArchivedDraft => 2,
        WorkflowCatalogConflictSource::CatalogRecord => 3,
        WorkflowCatalogConflictSource::StewardshipRecord => 4,
        WorkflowCatalogConflictSource::ArchiveRecord => 5,
    }
}

fn validate_relative_path(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "workflow_catalog_index.path.empty",
            format!("{type_name} cannot be empty"),
        ));
    }

    if value.len() > INDEX_PATH_MAX_BYTES {
        return Err(validation_error(
            "workflow_catalog_index.path.too_long",
            format!("{type_name} cannot exceed {INDEX_PATH_MAX_BYTES} bytes"),
        ));
    }

    validate_not_secret_like(type_name, value)?;

    let path = Path::new(value);
    if path.is_absolute() {
        return Err(validation_error(
            "workflow_catalog_index.path.absolute",
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
                    "workflow_catalog_index.path.unsafe",
                    format!("{type_name} contains an unsafe path component"),
                ));
            }
        }
    }

    if !saw_normal {
        return Err(validation_error(
            "workflow_catalog_index.path.empty",
            format!("{type_name} cannot be empty"),
        ));
    }

    Ok(())
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

fn validate_reference_text(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "workflow_catalog_index.reference.empty",
            format!("{type_name} cannot be empty"),
        ));
    }

    if value.len() > INDEX_TEXT_MAX_BYTES {
        return Err(validation_error(
            "workflow_catalog_index.reference.too_long",
            format!("{type_name} cannot exceed {INDEX_TEXT_MAX_BYTES} bytes"),
        ));
    }

    validate_not_secret_like(type_name, value)
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
            "workflow_catalog_index.secret_like_value",
            format!("{type_name} contains sensitive-looking text"),
        ));
    }

    Ok(())
}

fn validation_error(code: &'static str, message: impl Into<String>) -> WorkflowOsError {
    WorkflowOsError::validation(code, message)
}
