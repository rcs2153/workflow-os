use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    validate_approval_presentation_approval_id, ActorId, AdapterRuntimeAuditRecord,
    AdapterRuntimeObservabilityRecord, ApprovalPresentationId, ApprovalPresentationRecord,
    ApprovalRequest, EventId, EventSequenceNumber, IdempotencyKey, PolicyAuditRecord, ProjectId,
    SideEffectId, SideEffectLifecycleState, SideEffectRecord, WorkReportArtifactRecord,
    WorkReportId, WorkflowId, WorkflowOsError, WorkflowOsErrorKind, WorkflowRun, WorkflowRunEvent,
    WorkflowRunId, WorkflowRunSnapshot,
};

/// Durable event log contract.
pub trait EventLogStore {
    /// Appends one event to the durable event log after validating it against
    /// current durable event history.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the event ID or run sequence number is
    /// duplicated, the sequence is non-contiguous, immutable run identity does
    /// not match, the state transition is invalid, storage is corrupt, or the
    /// backend cannot durably write.
    fn append_event(&self, event: &WorkflowRunEvent) -> Result<(), WorkflowOsError>;

    /// Reads all events for a run ordered by sequence number.
    ///
    /// # Errors
    ///
    /// Returns a structured error when stored event data cannot be read or decoded.
    fn read_events(&self, run_id: &WorkflowRunId)
        -> Result<Vec<WorkflowRunEvent>, WorkflowOsError>;
}

/// Durable run snapshot projection contract.
pub trait RunSnapshotStore {
    /// Saves a snapshot projection.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the backend cannot durably write.
    fn save_snapshot(&self, snapshot: &WorkflowRunSnapshot) -> Result<(), WorkflowOsError>;

    /// Loads a snapshot projection, if present.
    ///
    /// # Errors
    ///
    /// Returns a structured error when stored snapshot data is corrupt.
    fn load_snapshot(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<Option<WorkflowRunSnapshot>, WorkflowOsError>;
}

/// Durable idempotency contract.
pub trait IdempotencyStore {
    /// Records an idempotency result if the key has not been seen before.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the backend cannot durably write or read.
    fn record_idempotency_result(
        &self,
        key: &IdempotencyKey,
        result: IdempotencyResult,
    ) -> Result<IdempotencyWrite, WorkflowOsError>;
}

/// Durable local lock contract.
pub trait LockStore {
    /// Acquires a lock for a caller.
    ///
    /// # Errors
    ///
    /// Returns `state.lock_contended` when the lock is already held.
    fn acquire_lock(&self, key: &str, owner: &ActorId) -> Result<LockLease, WorkflowOsError>;

    /// Releases a lock.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the lock cannot be released.
    fn release_lock(&self, lease: &LockLease) -> Result<(), WorkflowOsError>;
}

/// Approval projection contract. Approval truth remains in the event log.
pub trait ApprovalStore {
    /// Saves an approval request projection.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the backend cannot durably write.
    fn save_approval_request(&self, request: &ApprovalRequest) -> Result<(), WorkflowOsError>;

    /// Loads an approval request projection.
    ///
    /// # Errors
    ///
    /// Returns a structured error when stored approval data is corrupt.
    fn load_approval_request(
        &self,
        approval_id: &str,
    ) -> Result<Option<ApprovalRequest>, WorkflowOsError>;

    /// Deletes an approval request projection. The event log remains authoritative.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the backend cannot remove the projection.
    fn delete_approval_request(&self, approval_id: &str) -> Result<(), WorkflowOsError>;
}

/// Local project-state metadata contract.
pub trait ProjectStateStore {
    /// Saves non-secret project metadata.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the backend cannot durably write.
    fn save_project_state(&self, state: &ProjectStateRecord) -> Result<(), WorkflowOsError>;

    /// Loads non-secret project metadata.
    ///
    /// # Errors
    ///
    /// Returns a structured error when stored metadata is corrupt.
    fn load_project_state(
        &self,
        project_id: &ProjectId,
    ) -> Result<Option<ProjectStateRecord>, WorkflowOsError>;
}

/// Durable policy audit contract.
pub trait PolicyAuditStore {
    /// Appends a durable policy audit record.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the audit record is duplicated or cannot
    /// be durably written.
    fn append_policy_audit_record(&self, record: &PolicyAuditRecord)
        -> Result<(), WorkflowOsError>;

    /// Reads durable policy audit records.
    ///
    /// # Errors
    ///
    /// Returns a structured error when stored audit data is corrupt.
    fn read_policy_audit_records(&self) -> Result<Vec<PolicyAuditRecord>, WorkflowOsError>;
}

/// Durable adapter telemetry contract for scoped read-only example mappings.
pub trait AdapterTelemetryStore {
    /// Appends one adapter runtime audit telemetry record.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the telemetry ID is duplicated or cannot
    /// be durably written.
    fn append_adapter_audit_record(
        &self,
        record: &AdapterRuntimeAuditRecord,
    ) -> Result<(), WorkflowOsError>;

    /// Reads adapter runtime audit telemetry for a workflow run.
    ///
    /// # Errors
    ///
    /// Returns a structured error when stored telemetry is corrupt.
    fn read_adapter_audit_records(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<Vec<AdapterRuntimeAuditRecord>, WorkflowOsError>;

    /// Appends one adapter runtime observability telemetry record.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the telemetry ID is duplicated or cannot
    /// be durably written.
    fn append_adapter_observability_record(
        &self,
        record: &AdapterRuntimeObservabilityRecord,
    ) -> Result<(), WorkflowOsError>;

    /// Reads adapter runtime observability telemetry for a workflow run.
    ///
    /// # Errors
    ///
    /// Returns a structured error when stored telemetry is corrupt.
    fn read_adapter_observability_records(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<Vec<AdapterRuntimeObservabilityRecord>, WorkflowOsError>;
}

/// Durable work report artifact contract.
pub trait WorkReportArtifactStore {
    /// Writes one validated work report artifact.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the artifact is invalid, a duplicate
    /// report artifact already exists, or local storage cannot durably write.
    fn write_work_report_artifact(
        &self,
        artifact: &WorkReportArtifactRecord,
    ) -> Result<(), WorkflowOsError>;

    /// Reads one work report artifact for a run.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the artifact cannot be read or fails
    /// validation.
    fn read_work_report_artifact(
        &self,
        run_id: &WorkflowRunId,
        report_id: &WorkReportId,
    ) -> Result<Option<WorkReportArtifactRecord>, WorkflowOsError>;

    /// Lists work report artifacts for a run.
    ///
    /// # Errors
    ///
    /// Returns a structured error when artifact records cannot be read or fail
    /// validation.
    fn list_work_report_artifacts(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<Vec<WorkReportArtifactRecord>, WorkflowOsError>;
}

/// Durable `SideEffect` record contract.
pub trait SideEffectRecordStore {
    /// Writes one validated `SideEffect` record.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the record is invalid, the side-effect ID
    /// is duplicated, the run identity conflicts with existing records, or local
    /// storage cannot durably write.
    fn write_side_effect_record(&self, record: &SideEffectRecord) -> Result<(), WorkflowOsError>;

    /// Replaces one existing validated `SideEffect` record with the same ID.
    ///
    /// This is intended for reviewed lifecycle transitions only. It must not
    /// create a missing record, change immutable workflow/run identity, or be
    /// used to bypass transition validation.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the existing record is missing, the new
    /// record is invalid, immutable identity does not match, or local storage
    /// cannot durably replace the record.
    fn update_side_effect_record(&self, _record: &SideEffectRecord) -> Result<(), WorkflowOsError> {
        Err(WorkflowOsError::validation(
            "side_effect_record.update.unsupported",
            "side-effect record updates are unsupported by this store",
        ))
    }

    /// Reads one `SideEffect` record by ID.
    ///
    /// # Errors
    ///
    /// Returns a structured error when stored data cannot be read or fails
    /// validation.
    fn read_side_effect_record(
        &self,
        side_effect_id: &SideEffectId,
    ) -> Result<Option<SideEffectRecord>, WorkflowOsError>;

    /// Lists `SideEffect` records for one run.
    ///
    /// # Errors
    ///
    /// Returns a structured error when stored data cannot be read or fails
    /// validation.
    fn list_side_effect_records(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<Vec<SideEffectRecord>, WorkflowOsError>;

    /// Lists `SideEffect` records for one workflow/run identity.
    ///
    /// # Errors
    ///
    /// Returns a structured error when stored data cannot be read, fails
    /// validation, or does not match the requested workflow/run identity.
    fn list_side_effect_records_for_workflow_run(
        &self,
        workflow_id: &WorkflowId,
        run_id: &WorkflowRunId,
    ) -> Result<Vec<SideEffectRecord>, WorkflowOsError>;
}

/// Durable approval-presentation proof record contract.
pub trait ApprovalPresentationRecordStore {
    /// Writes one validated approval-presentation proof record.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the record is invalid, the presentation
    /// ID is duplicated, the run identity conflicts with existing records, or
    /// local storage cannot durably write.
    fn write_approval_presentation_record(
        &self,
        record: &ApprovalPresentationRecord,
    ) -> Result<(), WorkflowOsError>;

    /// Reads one approval-presentation proof record by presentation ID.
    ///
    /// # Errors
    ///
    /// Returns a structured error when stored data cannot be read or fails
    /// validation.
    fn read_approval_presentation_record(
        &self,
        presentation_id: &ApprovalPresentationId,
    ) -> Result<Option<ApprovalPresentationRecord>, WorkflowOsError>;

    /// Lists approval-presentation proof records for one run.
    ///
    /// # Errors
    ///
    /// Returns a structured error when stored data cannot be read or fails
    /// validation.
    fn list_approval_presentation_records(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<Vec<ApprovalPresentationRecord>, WorkflowOsError>;

    /// Lists approval-presentation proof records for one run and approval ID.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the approval ID is invalid, stored data
    /// cannot be read, or stored data fails validation.
    fn list_approval_presentation_records_for_approval(
        &self,
        run_id: &WorkflowRunId,
        approval_id: &str,
    ) -> Result<Vec<ApprovalPresentationRecord>, WorkflowOsError>;
}

/// Aggregate state backend contract.
pub trait StateBackend:
    EventLogStore
    + RunSnapshotStore
    + IdempotencyStore
    + LockStore
    + ApprovalStore
    + ApprovalPresentationRecordStore
    + ProjectStateStore
    + PolicyAuditStore
    + AdapterTelemetryStore
{
    /// Runs a backend health check.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the backend cannot check health.
    fn health_check(&self) -> Result<BackendHealthCheck, WorkflowOsError>;

    /// Rehydrates a workflow run from durable events.
    ///
    /// # Errors
    ///
    /// Returns a structured error when events cannot be read or replayed.
    fn rehydrate_run(&self, run_id: &WorkflowRunId) -> Result<WorkflowRun, WorkflowOsError> {
        let events = self.read_events(run_id)?;
        WorkflowRun::rehydrate(&events)
    }
}

/// Backend health check result.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BackendHealthCheck {
    /// Whether the backend is available.
    pub healthy: bool,
    /// Human-readable backend kind.
    pub backend: String,
    /// Non-secret status message.
    pub message: String,
}

/// Result stored for an idempotency key.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct IdempotencyResult {
    /// Non-secret result reference or summary.
    pub result_ref: String,
}

/// Result of an idempotency write.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum IdempotencyWrite {
    /// The key was written for the first time.
    FirstWrite(IdempotencyResult),
    /// The key already existed and returned the stored result.
    Duplicate(IdempotencyResult),
}

/// Local lock lease.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LockLease {
    /// Lock key.
    pub key: String,
    /// Lock owner.
    pub owner: ActorId,
}

/// Non-secret local project state metadata.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProjectStateRecord {
    /// Project ID.
    pub project_id: ProjectId,
    /// Non-secret metadata summary.
    pub metadata: String,
}

/// Local filesystem development backend.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalStateBackend {
    root: PathBuf,
}

/// Severity of a local state inspection issue.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalStateIssueSeverity {
    /// The state is inconsistent or unreadable enough that operators should
    /// treat the backend as unhealthy.
    Error,
    /// The state is recoverable or informational but should still be reviewed.
    Warning,
}

impl LocalStateIssueSeverity {
    /// Stable lowercase label for CLI and JSON output.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
        }
    }
}

/// One finding from read-only local state inspection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalStateIssue {
    /// Issue severity.
    pub severity: LocalStateIssueSeverity,
    /// Stable diagnostic code.
    pub code: String,
    /// Human-readable message.
    pub message: String,
    /// Local state file related to the issue, when known.
    pub path: Option<PathBuf>,
    /// Workflow run related to the issue, when known.
    pub run_id: Option<WorkflowRunId>,
    /// Event sequence related to the issue, when known.
    pub sequence_number: Option<EventSequenceNumber>,
    /// Event ID related to the issue, when known.
    pub event_id: Option<EventId>,
}

impl LocalStateIssue {
    fn new(
        severity: LocalStateIssueSeverity,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity,
            code: code.into(),
            message: message.into(),
            path: None,
            run_id: None,
            sequence_number: None,
            event_id: None,
        }
    }

    fn with_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.path = Some(path.into());
        self
    }

    fn with_run_id(mut self, run_id: WorkflowRunId) -> Self {
        self.run_id = Some(run_id);
        self
    }

    fn with_sequence_number(mut self, sequence_number: EventSequenceNumber) -> Self {
        self.sequence_number = Some(sequence_number);
        self
    }

    fn with_event_id(mut self, event_id: EventId) -> Self {
        self.event_id = Some(event_id);
        self
    }
}

/// Read-only inspection report for a local state backend.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalStateInspection {
    /// Whether no error-severity issues were found.
    pub healthy: bool,
    /// Backend implementation label.
    pub backend: String,
    /// State root inspected.
    pub root: PathBuf,
    /// Issues found during inspection.
    pub issues: Vec<LocalStateIssue>,
}

impl LocalStateBackend {
    /// Creates a local filesystem backend rooted at `root`.
    ///
    /// # Errors
    ///
    /// Returns an error when required directories cannot be created.
    pub fn new(root: impl Into<PathBuf>) -> Result<Self, WorkflowOsError> {
        let backend = Self { root: root.into() };
        backend.ensure_layout()?;
        Ok(backend)
    }

    /// Creates a local filesystem backend handle without creating directories.
    ///
    /// This is intended for read-only inspection commands.
    #[must_use]
    pub fn for_inspection(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Returns the backend root.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    fn ensure_layout(&self) -> Result<(), WorkflowOsError> {
        for directory in [
            self.events_dir(),
            self.event_ids_dir(),
            self.snapshots_dir(),
            self.idempotency_dir(),
            self.locks_dir(),
            self.approvals_dir(),
            self.projects_dir(),
            self.policy_audit_dir(),
            self.adapter_audit_dir(),
            self.adapter_observability_dir(),
            self.work_reports_dir(),
            self.side_effect_records_dir(),
            self.side_effect_ids_dir(),
            self.approval_presentation_records_dir(),
            self.approval_presentation_ids_dir(),
        ] {
            fs::create_dir_all(&directory).map_err(|error| {
                state_error(
                    "state.local.mkdir",
                    format!(
                        "failed to create state directory {}: {error}",
                        directory.display()
                    ),
                )
            })?;
        }
        Ok(())
    }

    fn events_dir(&self) -> PathBuf {
        self.root.join("events")
    }

    fn event_ids_dir(&self) -> PathBuf {
        self.root.join("event_ids")
    }

    fn snapshots_dir(&self) -> PathBuf {
        self.root.join("snapshots")
    }

    fn idempotency_dir(&self) -> PathBuf {
        self.root.join("idempotency")
    }

    fn locks_dir(&self) -> PathBuf {
        self.root.join("locks")
    }

    fn approvals_dir(&self) -> PathBuf {
        self.root.join("approvals")
    }

    fn projects_dir(&self) -> PathBuf {
        self.root.join("projects")
    }

    fn policy_audit_dir(&self) -> PathBuf {
        self.root.join("policy_audit")
    }

    fn adapter_audit_dir(&self) -> PathBuf {
        self.root.join("adapter_audit")
    }

    fn adapter_observability_dir(&self) -> PathBuf {
        self.root.join("adapter_observability")
    }

    fn work_reports_dir(&self) -> PathBuf {
        self.root.join("work_reports")
    }

    fn side_effect_records_dir(&self) -> PathBuf {
        self.root.join("side_effects").join("records")
    }

    fn side_effect_ids_dir(&self) -> PathBuf {
        self.root.join("side_effects").join("ids")
    }

    fn approval_presentation_records_dir(&self) -> PathBuf {
        self.root.join("approval_presentations").join("records")
    }

    fn approval_presentation_ids_dir(&self) -> PathBuf {
        self.root.join("approval_presentations").join("ids")
    }

    fn adapter_audit_run_dir(&self, run_id: &WorkflowRunId) -> PathBuf {
        self.adapter_audit_dir().join(encode_key(run_id.as_str()))
    }

    fn adapter_observability_run_dir(&self, run_id: &WorkflowRunId) -> PathBuf {
        self.adapter_observability_dir()
            .join(encode_key(run_id.as_str()))
    }

    fn work_report_run_dir(&self, run_id: &WorkflowRunId) -> PathBuf {
        self.work_reports_dir().join(encode_key(run_id.as_str()))
    }

    fn work_report_artifact_path(
        &self,
        run_id: &WorkflowRunId,
        report_id: &WorkReportId,
    ) -> PathBuf {
        self.work_report_run_dir(run_id)
            .join(format!("{}.json", encode_key(report_id.as_str())))
    }

    fn side_effect_run_dir(&self, run_id: &WorkflowRunId) -> PathBuf {
        self.side_effect_records_dir()
            .join(encode_key(run_id.as_str()))
    }

    fn side_effect_record_path(
        &self,
        run_id: &WorkflowRunId,
        side_effect_id: &SideEffectId,
    ) -> PathBuf {
        self.side_effect_run_dir(run_id)
            .join(format!("{}.json", encode_key(side_effect_id.as_str())))
    }

    fn side_effect_id_path(&self, side_effect_id: &SideEffectId) -> PathBuf {
        self.side_effect_ids_dir()
            .join(format!("{}.json", encode_key(side_effect_id.as_str())))
    }

    fn approval_presentation_run_dir(&self, run_id: &WorkflowRunId) -> PathBuf {
        self.approval_presentation_records_dir()
            .join(encode_key(run_id.as_str()))
    }

    fn approval_presentation_record_path(
        &self,
        run_id: &WorkflowRunId,
        presentation_id: &ApprovalPresentationId,
    ) -> PathBuf {
        self.approval_presentation_run_dir(run_id)
            .join(format!("{}.json", encode_key(presentation_id.as_str())))
    }

    fn approval_presentation_id_path(&self, presentation_id: &ApprovalPresentationId) -> PathBuf {
        self.approval_presentation_ids_dir()
            .join(format!("{}.json", encode_key(presentation_id.as_str())))
    }

    fn run_events_dir(&self, run_id: &WorkflowRunId) -> PathBuf {
        self.events_dir().join(encode_key(run_id.as_str()))
    }

    fn event_path(&self, event: &WorkflowRunEvent) -> PathBuf {
        self.event_sequence_path(&event.run_id, event.sequence_number)
    }

    fn event_sequence_path(
        &self,
        run_id: &WorkflowRunId,
        sequence_number: EventSequenceNumber,
    ) -> PathBuf {
        self.run_events_dir(run_id)
            .join(format!("{:020}.json", sequence_number.get()))
    }

    fn event_id_path(&self, event_id: &EventId) -> PathBuf {
        self.event_ids_dir()
            .join(format!("{}.json", encode_key(event_id.as_str())))
    }

    fn validate_event_index_consistency(&self) -> Result<(), WorkflowOsError> {
        if self.events_dir().exists() {
            for run_entry in fs::read_dir(self.events_dir()).map_err(|error| {
                state_error(
                    "state.local.read_dir",
                    format!("failed to read events directory: {error}"),
                )
            })? {
                let run_entry = run_entry.map_err(|error| {
                    state_error(
                        "state.local.read_dir_entry",
                        format!("failed to inspect events directory entry: {error}"),
                    )
                })?;
                let run_dir = run_entry.path();
                if run_dir.is_dir() {
                    for event_path in json_files_in_dir(&run_dir)? {
                        let event: WorkflowRunEvent = read_json(&event_path)?;
                        self.validate_event_has_index(&event)?;
                    }
                }
            }
        }

        if self.event_ids_dir().exists() {
            for index_path in json_files_in_dir(&self.event_ids_dir())? {
                let record: EventIdRecord = read_json(&index_path)?;
                let event_path = self.event_sequence_path(&record.run_id, record.sequence_number);
                if !event_path.exists() {
                    return Err(state_error(
                        "state.event_index.dangling",
                        format!(
                            "event ID index {} points to missing event file {}",
                            index_path.display(),
                            event_path.display()
                        ),
                    ));
                }
                let event: WorkflowRunEvent = read_json(&event_path)?;
                if self.event_id_path(&event.event_id) != index_path {
                    return Err(state_error(
                        "state.event_index.mismatch",
                        format!(
                            "event ID index {} does not match event {}",
                            index_path.display(),
                            event.event_id
                        ),
                    ));
                }
            }
        }

        Ok(())
    }

    fn validate_event_has_index(&self, event: &WorkflowRunEvent) -> Result<(), WorkflowOsError> {
        let index_path = self.event_id_path(&event.event_id);
        if !index_path.exists() {
            return Err(state_error(
                "state.event_index.missing",
                format!(
                    "event {} for run {} sequence {} is missing event ID index {}",
                    event.event_id,
                    event.run_id,
                    event.sequence_number,
                    index_path.display()
                ),
            ));
        }
        let record: EventIdRecord = read_json(&index_path)?;
        if record.run_id != event.run_id || record.sequence_number != event.sequence_number {
            return Err(state_error(
                "state.event_index.mismatch",
                format!(
                    "event ID index {} points to run {} sequence {}, expected run {} sequence {}",
                    index_path.display(),
                    record.run_id,
                    record.sequence_number,
                    event.run_id,
                    event.sequence_number
                ),
            ));
        }
        Ok(())
    }

    fn with_local_lock<T>(
        &self,
        lock_key: &str,
        owner: &ActorId,
        operation: impl FnOnce() -> Result<T, WorkflowOsError>,
    ) -> Result<T, WorkflowOsError> {
        let lease = self.acquire_lock(lock_key, owner)?;
        let result = operation();
        let release = self.release_lock(&lease);
        match (result, release) {
            (Ok(value), Ok(())) => Ok(value),
            (Err(error), _) | (Ok(_), Err(error)) => Err(error),
        }
    }

    /// Performs a read-only local state inspection.
    ///
    /// The inspection does not create directories, write probe files, repair
    /// indexes, or delete state. It reports event/index drift, corrupt event
    /// files, rehydration failures, and approval projection drift that can be
    /// detected from existing files.
    #[must_use]
    pub fn inspect_state(&self) -> LocalStateInspection {
        let mut issues = Vec::new();
        let mut events_by_run: BTreeMap<WorkflowRunId, Vec<WorkflowRunEvent>> = BTreeMap::new();
        let mut event_backed_approvals: BTreeMap<String, ApprovalRequest> = BTreeMap::new();

        self.inspect_event_files(&mut issues, &mut events_by_run);
        self.inspect_event_indexes(&mut issues);
        Self::inspect_rehydration(&mut issues, &events_by_run, &mut event_backed_approvals);
        self.inspect_approval_projections(&mut issues, &event_backed_approvals);

        let healthy = issues
            .iter()
            .all(|issue| issue.severity != LocalStateIssueSeverity::Error);
        LocalStateInspection {
            healthy,
            backend: "local_filesystem".to_owned(),
            root: self.root.clone(),
            issues,
        }
    }

    fn inspect_event_files(
        &self,
        issues: &mut Vec<LocalStateIssue>,
        events_by_run: &mut BTreeMap<WorkflowRunId, Vec<WorkflowRunEvent>>,
    ) {
        let events_dir = self.events_dir();
        if !events_dir.exists() {
            return;
        }
        let run_entries = match fs::read_dir(&events_dir) {
            Ok(entries) => entries,
            Err(error) => {
                issues.push(
                    LocalStateIssue::new(
                        LocalStateIssueSeverity::Error,
                        "state.local.read_dir",
                        format!(
                            "failed to read events directory {}: {error}",
                            events_dir.display()
                        ),
                    )
                    .with_path(events_dir),
                );
                return;
            }
        };
        for entry in run_entries {
            let Ok(entry) = entry else {
                issues.push(LocalStateIssue::new(
                    LocalStateIssueSeverity::Error,
                    "state.local.read_dir_entry",
                    "failed to inspect events directory entry",
                ));
                continue;
            };
            let run_dir = entry.path();
            if !run_dir.is_dir() {
                continue;
            }
            for event_path in inspection_json_files(&run_dir, issues) {
                match read_json::<WorkflowRunEvent>(&event_path) {
                    Ok(event) => {
                        if !self.event_id_path(&event.event_id).exists() {
                            issues.push(
                                LocalStateIssue::new(
                                    LocalStateIssueSeverity::Error,
                                    "state.event_index.missing",
                                    format!(
                                    "event {} for run {} sequence {} is missing its event ID index",
                                    event.event_id, event.run_id, event.sequence_number
                                ),
                                )
                                .with_path(event_path.clone())
                                .with_run_id(event.run_id.clone())
                                .with_sequence_number(event.sequence_number)
                                .with_event_id(event.event_id.clone()),
                            );
                        }
                        events_by_run
                            .entry(event.run_id.clone())
                            .or_default()
                            .push(event);
                    }
                    Err(error) => issues.push(
                        LocalStateIssue::new(
                            LocalStateIssueSeverity::Error,
                            error.code().to_owned(),
                            error.message().to_owned(),
                        )
                        .with_path(event_path),
                    ),
                }
            }
        }
    }

    fn inspect_event_indexes(&self, issues: &mut Vec<LocalStateIssue>) {
        let event_ids_dir = self.event_ids_dir();
        if !event_ids_dir.exists() {
            return;
        }
        for index_path in inspection_json_files(&event_ids_dir, issues) {
            let record = match read_json::<EventIdRecord>(&index_path) {
                Ok(record) => record,
                Err(error) => {
                    issues.push(
                        LocalStateIssue::new(
                            LocalStateIssueSeverity::Error,
                            error.code().to_owned(),
                            error.message().to_owned(),
                        )
                        .with_path(index_path),
                    );
                    continue;
                }
            };
            let event_path = self.event_sequence_path(&record.run_id, record.sequence_number);
            if !event_path.exists() {
                issues.push(
                    LocalStateIssue::new(
                        LocalStateIssueSeverity::Error,
                        "state.event_index.dangling",
                        format!(
                            "event ID index {} points to missing event file {}",
                            index_path.display(),
                            event_path.display()
                        ),
                    )
                    .with_path(index_path)
                    .with_run_id(record.run_id)
                    .with_sequence_number(record.sequence_number),
                );
                continue;
            }
            let Ok(event) = read_json::<WorkflowRunEvent>(&event_path) else {
                continue;
            };
            if self.event_id_path(&event.event_id) != index_path
                || event.run_id != record.run_id
                || event.sequence_number != record.sequence_number
            {
                issues.push(
                    LocalStateIssue::new(
                        LocalStateIssueSeverity::Error,
                        "state.event_index.mismatch",
                        format!(
                            "event ID index {} does not match event {} at {}",
                            index_path.display(),
                            event.event_id,
                            event_path.display()
                        ),
                    )
                    .with_path(index_path)
                    .with_run_id(record.run_id)
                    .with_sequence_number(record.sequence_number)
                    .with_event_id(event.event_id),
                );
            }
        }
    }

    fn inspect_rehydration(
        issues: &mut Vec<LocalStateIssue>,
        events_by_run: &BTreeMap<WorkflowRunId, Vec<WorkflowRunEvent>>,
        event_backed_approvals: &mut BTreeMap<String, ApprovalRequest>,
    ) {
        for (run_id, events) in events_by_run {
            let mut ordered = events.clone();
            ordered.sort_by_key(|event| event.sequence_number);
            match WorkflowRun::rehydrate(&ordered) {
                Ok(run) => {
                    for approval in run.snapshot.approval_requests {
                        event_backed_approvals.insert(approval.approval_id.clone(), approval);
                    }
                }
                Err(error) => issues.push(
                    LocalStateIssue::new(
                        LocalStateIssueSeverity::Error,
                        "state.rehydration.failed",
                        format!("failed to rehydrate run {run_id}: {}", error.message()),
                    )
                    .with_run_id(run_id.clone()),
                ),
            }
        }
    }

    fn inspect_approval_projections(
        &self,
        issues: &mut Vec<LocalStateIssue>,
        event_backed_approvals: &BTreeMap<String, ApprovalRequest>,
    ) {
        let approvals_dir = self.approvals_dir();
        if !approvals_dir.exists() {
            for approval in event_backed_approvals.values() {
                if approval.decision.is_none() {
                    issues.push(
                        LocalStateIssue::new(
                            LocalStateIssueSeverity::Warning,
                            "state.approval_projection.missing",
                            format!(
                                "pending approval {} has no local approval projection",
                                approval.approval_id
                            ),
                        )
                        .with_path(approvals_dir.clone())
                        .with_run_id(approval.run_id.clone()),
                    );
                }
            }
            return;
        }
        let mut projection_ids = BTreeSet::new();
        for path in inspection_json_files(&approvals_dir, issues) {
            match read_json::<ApprovalRequest>(&path) {
                Ok(projection) => {
                    projection_ids.insert(projection.approval_id.clone());
                    match event_backed_approvals.get(&projection.approval_id) {
                        Some(event_backed) if event_backed.run_id == projection.run_id => {}
                        Some(event_backed) => issues.push(
                            LocalStateIssue::new(
                                LocalStateIssueSeverity::Error,
                                "state.approval_projection.mismatch",
                                format!(
                                "approval projection {} references run {}, but event history references run {}",
                                projection.approval_id, projection.run_id, event_backed.run_id
                            ),
                            )
                            .with_path(path)
                            .with_run_id(projection.run_id),
                        ),
                        None => issues.push(
                            LocalStateIssue::new(
                                LocalStateIssueSeverity::Error,
                                "state.approval_projection.orphaned",
                                format!(
                                "approval projection {} has no matching approval request event",
                                projection.approval_id
                            ),
                            )
                            .with_path(path)
                            .with_run_id(projection.run_id),
                        ),
                    }
                }
                Err(error) => issues.push(
                    LocalStateIssue::new(
                        LocalStateIssueSeverity::Error,
                        error.code().to_owned(),
                        error.message().to_owned(),
                    )
                    .with_path(path),
                ),
            }
        }
        for approval in event_backed_approvals.values() {
            if approval.decision.is_none() && !projection_ids.contains(&approval.approval_id) {
                issues.push(
                    LocalStateIssue::new(
                        LocalStateIssueSeverity::Warning,
                        "state.approval_projection.missing",
                        format!(
                            "pending approval {} has no local approval projection",
                            approval.approval_id
                        ),
                    )
                    .with_path(
                        approvals_dir.join(format!("{}.json", encode_key(&approval.approval_id))),
                    )
                    .with_run_id(approval.run_id.clone()),
                );
            }
        }
    }
}

impl EventLogStore for LocalStateBackend {
    fn append_event(&self, event: &WorkflowRunEvent) -> Result<(), WorkflowOsError> {
        let owner = match &event.actor {
            Some(actor) => actor.clone(),
            None => ActorId::new("system").map_err(|error| {
                state_error(
                    "state.actor.invalid",
                    format!("failed to build system lock actor: {error}"),
                )
            })?,
        };
        self.with_local_lock("event-log", &owner, || {
            self.ensure_layout()?;
            self.validate_event_index_consistency()?;
            let run_dir = self.run_events_dir(&event.run_id);
            fs::create_dir_all(&run_dir).map_err(|error| {
                state_error(
                    "state.local.mkdir",
                    format!("failed to create run event directory: {error}"),
                )
            })?;

            let event_path = self.event_path(event);
            let event_id_path = self.event_id_path(&event.event_id);
            if event_id_path.exists() {
                return Err(state_error(
                    "state.event.duplicate_id",
                    format!("duplicate event id {}", event.event_id),
                ));
            }
            if event_path.exists() {
                return Err(state_error(
                    "state.event.duplicate_sequence",
                    format!(
                        "duplicate sequence {} for run {}",
                        event.sequence_number, event.run_id
                    ),
                ));
            }

            let existing_events = self.read_events(&event.run_id)?;
            validate_append_against_history(&existing_events, event)?;
            write_json_create_new_atomic(
                &event_id_path,
                &EventIdRecord {
                    run_id: event.run_id.clone(),
                    sequence_number: event.sequence_number,
                },
            )?;
            if let Err(error) = write_json_create_new_atomic(&event_path, event) {
                let _ = fs::remove_file(&event_id_path);
                return Err(error);
            }
            Ok(())
        })
    }

    fn read_events(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<Vec<WorkflowRunEvent>, WorkflowOsError> {
        let run_dir = self.run_events_dir(run_id);
        if !run_dir.exists() {
            return Ok(Vec::new());
        }
        let mut paths = json_files_in_dir(&run_dir)?;
        paths.sort();
        let events = paths
            .iter()
            .map(|path| read_json(path))
            .collect::<Result<Vec<_>, _>>()?;
        for event in &events {
            self.validate_event_has_index(event)?;
        }
        Ok(events)
    }
}

impl RunSnapshotStore for LocalStateBackend {
    fn save_snapshot(&self, snapshot: &WorkflowRunSnapshot) -> Result<(), WorkflowOsError> {
        self.ensure_layout()?;
        write_json_replace(
            &self.snapshots_dir().join(format!(
                "{}.json",
                encode_key(snapshot.identity.run_id.as_str())
            )),
            snapshot,
        )
    }

    fn load_snapshot(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<Option<WorkflowRunSnapshot>, WorkflowOsError> {
        read_optional_json(
            &self
                .snapshots_dir()
                .join(format!("{}.json", encode_key(run_id.as_str()))),
        )
    }
}

impl IdempotencyStore for LocalStateBackend {
    fn record_idempotency_result(
        &self,
        key: &IdempotencyKey,
        result: IdempotencyResult,
    ) -> Result<IdempotencyWrite, WorkflowOsError> {
        self.ensure_layout()?;
        let path = self
            .idempotency_dir()
            .join(format!("{}.json", encode_key(key.as_str())));
        if path.exists() {
            return Ok(IdempotencyWrite::Duplicate(read_json(&path)?));
        }
        match write_json_create_new(&path, &result) {
            Ok(()) => Ok(IdempotencyWrite::FirstWrite(result)),
            Err(error) if error.code() == "state.local.exists" => {
                Ok(IdempotencyWrite::Duplicate(read_json(&path)?))
            }
            Err(error) => Err(error),
        }
    }
}

impl LockStore for LocalStateBackend {
    fn acquire_lock(&self, key: &str, owner: &ActorId) -> Result<LockLease, WorkflowOsError> {
        self.ensure_layout()?;
        let path = self.locks_dir().join(encode_key(key));
        match fs::create_dir(&path) {
            Ok(()) => {
                write_json_replace(
                    &path.join("owner.json"),
                    &LockLease {
                        key: key.to_owned(),
                        owner: owner.clone(),
                    },
                )?;
                Ok(LockLease {
                    key: key.to_owned(),
                    owner: owner.clone(),
                })
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => Err(state_error(
                "state.lock_contended",
                format!("lock {key} is already held"),
            )),
            Err(error) => Err(state_error(
                "state.lock.acquire",
                format!("failed to acquire lock {key}: {error}"),
            )),
        }
    }

    fn release_lock(&self, lease: &LockLease) -> Result<(), WorkflowOsError> {
        let path = self.locks_dir().join(encode_key(&lease.key));
        if !path.exists() {
            return Ok(());
        }
        fs::remove_dir_all(&path).map_err(|error| {
            state_error(
                "state.lock.release",
                format!("failed to release lock {}: {error}", lease.key),
            )
        })
    }
}

impl ApprovalStore for LocalStateBackend {
    fn save_approval_request(&self, request: &ApprovalRequest) -> Result<(), WorkflowOsError> {
        self.ensure_layout()?;
        write_json_replace(
            &self
                .approvals_dir()
                .join(format!("{}.json", encode_key(&request.approval_id))),
            request,
        )
    }

    fn load_approval_request(
        &self,
        approval_id: &str,
    ) -> Result<Option<ApprovalRequest>, WorkflowOsError> {
        read_optional_json(
            &self
                .approvals_dir()
                .join(format!("{}.json", encode_key(approval_id))),
        )
    }

    fn delete_approval_request(&self, approval_id: &str) -> Result<(), WorkflowOsError> {
        let path = self
            .approvals_dir()
            .join(format!("{}.json", encode_key(approval_id)));
        match fs::remove_file(&path) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(state_error(
                "state.approval.delete",
                format!(
                    "failed to delete approval projection {}: {error}",
                    path.display()
                ),
            )),
        }
    }
}

impl ProjectStateStore for LocalStateBackend {
    fn save_project_state(&self, state: &ProjectStateRecord) -> Result<(), WorkflowOsError> {
        self.ensure_layout()?;
        write_json_replace(
            &self
                .projects_dir()
                .join(format!("{}.json", encode_key(state.project_id.as_str()))),
            state,
        )
    }

    fn load_project_state(
        &self,
        project_id: &ProjectId,
    ) -> Result<Option<ProjectStateRecord>, WorkflowOsError> {
        read_optional_json(
            &self
                .projects_dir()
                .join(format!("{}.json", encode_key(project_id.as_str()))),
        )
    }
}

impl PolicyAuditStore for LocalStateBackend {
    fn append_policy_audit_record(
        &self,
        record: &PolicyAuditRecord,
    ) -> Result<(), WorkflowOsError> {
        self.ensure_layout()?;
        write_json_create_new_atomic(
            &self
                .policy_audit_dir()
                .join(format!("{}.json", encode_key(record.audit_id.as_str()))),
            record,
        )
    }

    fn read_policy_audit_records(&self) -> Result<Vec<PolicyAuditRecord>, WorkflowOsError> {
        self.ensure_layout()?;
        let records = json_files_in_dir(&self.policy_audit_dir())?
            .iter()
            .map(|path| read_json(path))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(records)
    }
}

impl AdapterTelemetryStore for LocalStateBackend {
    fn append_adapter_audit_record(
        &self,
        record: &AdapterRuntimeAuditRecord,
    ) -> Result<(), WorkflowOsError> {
        self.ensure_layout()?;
        let Some(run_id) = &record.workflow_run_id else {
            return Err(state_error(
                "state.adapter_audit.run_id_required",
                "adapter audit telemetry requires workflow run ID for local persistence",
            ));
        };
        let directory = self.adapter_audit_run_dir(run_id);
        fs::create_dir_all(&directory).map_err(|error| {
            state_error(
                "state.local.mkdir",
                format!(
                    "failed to create adapter audit directory {}: {error}",
                    directory.display()
                ),
            )
        })?;
        write_json_create_new_atomic(
            &directory.join(format!("{}.json", encode_key(record.telemetry_id.as_str()))),
            record,
        )
    }

    fn read_adapter_audit_records(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<Vec<AdapterRuntimeAuditRecord>, WorkflowOsError> {
        self.ensure_layout()?;
        let directory = self.adapter_audit_run_dir(run_id);
        if !directory.exists() {
            return Ok(Vec::new());
        }
        let mut records = json_files_in_dir(&directory)?
            .iter()
            .map(|path| read_json(path))
            .collect::<Result<Vec<_>, _>>()?;
        records.sort_by_key(|record: &AdapterRuntimeAuditRecord| record.timestamp);
        Ok(records)
    }

    fn append_adapter_observability_record(
        &self,
        record: &AdapterRuntimeObservabilityRecord,
    ) -> Result<(), WorkflowOsError> {
        self.ensure_layout()?;
        let Some(run_id) = &record.workflow_run_id else {
            return Err(state_error(
                "state.adapter_observability.run_id_required",
                "adapter observability telemetry requires workflow run ID for local persistence",
            ));
        };
        let directory = self.adapter_observability_run_dir(run_id);
        fs::create_dir_all(&directory).map_err(|error| {
            state_error(
                "state.local.mkdir",
                format!(
                    "failed to create adapter observability directory {}: {error}",
                    directory.display()
                ),
            )
        })?;
        write_json_create_new_atomic(
            &directory.join(format!("{}.json", encode_key(record.telemetry_id.as_str()))),
            record,
        )
    }

    fn read_adapter_observability_records(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<Vec<AdapterRuntimeObservabilityRecord>, WorkflowOsError> {
        self.ensure_layout()?;
        let directory = self.adapter_observability_run_dir(run_id);
        if !directory.exists() {
            return Ok(Vec::new());
        }
        let mut records = json_files_in_dir(&directory)?
            .iter()
            .map(|path| read_json(path))
            .collect::<Result<Vec<_>, _>>()?;
        records.sort_by_key(|record: &AdapterRuntimeObservabilityRecord| record.timestamp);
        Ok(records)
    }
}

impl WorkReportArtifactStore for LocalStateBackend {
    fn write_work_report_artifact(
        &self,
        artifact: &WorkReportArtifactRecord,
    ) -> Result<(), WorkflowOsError> {
        artifact.validate()?;
        self.ensure_layout()?;
        let path = self.work_report_artifact_path(artifact.run_id(), artifact.report_id());
        match write_json_create_new_atomic(&path, artifact) {
            Ok(()) => Ok(()),
            Err(error) if error.code() == "state.local.exists" => Err(state_error(
                "work_report_artifact.write.duplicate",
                "work report artifact already exists",
            )),
            Err(_) => Err(state_error(
                "work_report_artifact.write.failed",
                "failed to write work report artifact",
            )),
        }
    }

    fn read_work_report_artifact(
        &self,
        run_id: &WorkflowRunId,
        report_id: &WorkReportId,
    ) -> Result<Option<WorkReportArtifactRecord>, WorkflowOsError> {
        self.ensure_layout()?;
        let path = self.work_report_artifact_path(run_id, report_id);
        if !path.exists() {
            return Ok(None);
        }
        read_json::<WorkReportArtifactRecord>(&path)
            .map(Some)
            .map_err(|_| {
                state_error(
                    "work_report_artifact.read.corrupt",
                    "work report artifact could not be read or validated",
                )
            })
    }

    fn list_work_report_artifacts(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<Vec<WorkReportArtifactRecord>, WorkflowOsError> {
        self.ensure_layout()?;
        let directory = self.work_report_run_dir(run_id);
        if !directory.exists() {
            return Ok(Vec::new());
        }
        json_files_in_dir(&directory)?
            .iter()
            .map(|path| {
                read_json::<WorkReportArtifactRecord>(path).map_err(|_| {
                    state_error(
                        "work_report_artifact.read.corrupt",
                        "work report artifact could not be read or validated",
                    )
                })
            })
            .collect()
    }
}

impl SideEffectRecordStore for LocalStateBackend {
    fn write_side_effect_record(&self, record: &SideEffectRecord) -> Result<(), WorkflowOsError> {
        record.validate()?;
        self.ensure_layout()?;
        for existing in self.list_side_effect_records(record.run_id())? {
            if !same_side_effect_run_identity(&existing, record) {
                return Err(state_error(
                    "side_effect_record.write.identity_mismatch",
                    "side-effect record workflow/run identity conflicts with existing records",
                ));
            }
        }
        let record_path = self.side_effect_record_path(record.run_id(), record.side_effect_id());
        let id_path = self.side_effect_id_path(record.side_effect_id());
        if id_path.exists() {
            return Err(state_error(
                "side_effect_record.write.duplicate",
                "side-effect record already exists",
            ));
        }
        let index = SideEffectIdRecord {
            run_id: record.run_id().clone(),
        };
        match write_json_create_new_atomic(&id_path, &index) {
            Ok(()) => {}
            Err(error) if error.code() == "state.local.exists" => {
                return Err(state_error(
                    "side_effect_record.write.duplicate",
                    "side-effect record already exists",
                ));
            }
            Err(_) => {
                return Err(state_error(
                    "side_effect_record.write.failed",
                    "failed to write side-effect record",
                ));
            }
        }
        if write_json_create_new_atomic(&record_path, record).is_err() {
            let _ = fs::remove_file(&id_path);
            return Err(state_error(
                "side_effect_record.write.failed",
                "failed to write side-effect record",
            ));
        }
        Ok(())
    }

    fn update_side_effect_record(&self, record: &SideEffectRecord) -> Result<(), WorkflowOsError> {
        record.validate()?;
        self.ensure_layout()?;
        let existing = self
            .read_side_effect_record(record.side_effect_id())?
            .ok_or_else(|| {
                state_error(
                    "side_effect_record.update.missing",
                    "side-effect record does not exist",
                )
            })?;
        if !same_side_effect_run_identity(&existing, record) {
            return Err(state_error(
                "side_effect_record.update.identity_mismatch",
                "side-effect record workflow/run identity conflicts with existing record",
            ));
        }
        if !is_allowed_side_effect_lifecycle_update(&existing, record) {
            return Err(state_error(
                "side_effect_record.update.invalid_lifecycle_transition",
                "side-effect record update lifecycle transition is not supported",
            ));
        }
        let record_path = self.side_effect_record_path(record.run_id(), record.side_effect_id());
        write_json_replace(&record_path, record).map_err(|_| {
            state_error(
                "side_effect_record.update.failed",
                "failed to update side-effect record",
            )
        })
    }

    fn read_side_effect_record(
        &self,
        side_effect_id: &SideEffectId,
    ) -> Result<Option<SideEffectRecord>, WorkflowOsError> {
        self.ensure_layout()?;
        let id_path = self.side_effect_id_path(side_effect_id);
        if !id_path.exists() {
            return Ok(None);
        }
        let index = read_json::<SideEffectIdRecord>(&id_path).map_err(|_| {
            state_error(
                "side_effect_record.read.corrupt",
                "side-effect record could not be read or validated",
            )
        })?;
        let record_path = self.side_effect_record_path(&index.run_id, side_effect_id);
        let record = read_json::<SideEffectRecord>(&record_path).map_err(|_| {
            state_error(
                "side_effect_record.read.corrupt",
                "side-effect record could not be read or validated",
            )
        })?;
        if record.side_effect_id() != side_effect_id || record.run_id() != &index.run_id {
            return Err(state_error(
                "side_effect_record.read.corrupt",
                "side-effect record could not be read or validated",
            ));
        }
        Ok(Some(record))
    }

    fn list_side_effect_records(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<Vec<SideEffectRecord>, WorkflowOsError> {
        self.ensure_layout()?;
        let directory = self.side_effect_run_dir(run_id);
        if !directory.exists() {
            return Ok(Vec::new());
        }
        let paths = json_files_in_dir(&directory).map_err(|_| {
            state_error(
                "side_effect_record.read.failed",
                "failed to list side-effect records",
            )
        })?;
        let mut records = paths
            .iter()
            .map(|path| {
                let record = read_json::<SideEffectRecord>(path).map_err(|_| {
                    state_error(
                        "side_effect_record.read.corrupt",
                        "side-effect record could not be read or validated",
                    )
                })?;
                if record.run_id() != run_id {
                    return Err(state_error(
                        "side_effect_record.read.corrupt",
                        "side-effect record could not be read or validated",
                    ));
                }
                Ok(record)
            })
            .collect::<Result<Vec<_>, _>>()?;
        records.sort_by(|left, right| left.side_effect_id().cmp(right.side_effect_id()));
        Ok(records)
    }

    fn list_side_effect_records_for_workflow_run(
        &self,
        workflow_id: &WorkflowId,
        run_id: &WorkflowRunId,
    ) -> Result<Vec<SideEffectRecord>, WorkflowOsError> {
        let records = self.list_side_effect_records(run_id)?;
        if records
            .iter()
            .any(|record| record.workflow_id() != workflow_id)
            || records
                .windows(2)
                .any(|pair| !same_side_effect_run_identity(&pair[0], &pair[1]))
        {
            return Err(state_error(
                "side_effect_record.read.identity_mismatch",
                "side-effect record workflow/run identity does not match requested identity",
            ));
        }
        Ok(records)
    }
}

impl ApprovalPresentationRecordStore for LocalStateBackend {
    fn write_approval_presentation_record(
        &self,
        record: &ApprovalPresentationRecord,
    ) -> Result<(), WorkflowOsError> {
        self.ensure_layout()?;
        for existing in self.list_approval_presentation_records(record.run_id())? {
            if !same_approval_presentation_run_identity(&existing, record) {
                return Err(state_error(
                    "approval_presentation_record.write.identity_mismatch",
                    "approval-presentation record workflow/run identity conflicts with existing records",
                ));
            }
        }
        let record_path =
            self.approval_presentation_record_path(record.run_id(), record.presentation_id());
        let id_path = self.approval_presentation_id_path(record.presentation_id());
        if id_path.exists() {
            return Err(state_error(
                "approval_presentation_record.write.duplicate",
                "approval-presentation record already exists",
            ));
        }
        let index = ApprovalPresentationIdRecord {
            run_id: record.run_id().clone(),
        };
        match write_json_create_new_atomic(&id_path, &index) {
            Ok(()) => {}
            Err(error) if error.code() == "state.local.exists" => {
                return Err(state_error(
                    "approval_presentation_record.write.duplicate",
                    "approval-presentation record already exists",
                ));
            }
            Err(_) => {
                return Err(state_error(
                    "approval_presentation_record.write.failed",
                    "failed to write approval-presentation record",
                ));
            }
        }
        if write_json_create_new_atomic(&record_path, record).is_err() {
            let _ = fs::remove_file(&id_path);
            return Err(state_error(
                "approval_presentation_record.write.failed",
                "failed to write approval-presentation record",
            ));
        }
        Ok(())
    }

    fn read_approval_presentation_record(
        &self,
        presentation_id: &ApprovalPresentationId,
    ) -> Result<Option<ApprovalPresentationRecord>, WorkflowOsError> {
        self.ensure_layout()?;
        let id_path = self.approval_presentation_id_path(presentation_id);
        if !id_path.exists() {
            return Ok(None);
        }
        let index = read_json::<ApprovalPresentationIdRecord>(&id_path).map_err(|_| {
            state_error(
                "approval_presentation_record.read.corrupt",
                "approval-presentation record could not be read or validated",
            )
        })?;
        let record_path = self.approval_presentation_record_path(&index.run_id, presentation_id);
        let record = read_json::<ApprovalPresentationRecord>(&record_path).map_err(|_| {
            state_error(
                "approval_presentation_record.read.corrupt",
                "approval-presentation record could not be read or validated",
            )
        })?;
        if record.presentation_id() != presentation_id || record.run_id() != &index.run_id {
            return Err(state_error(
                "approval_presentation_record.read.corrupt",
                "approval-presentation record could not be read or validated",
            ));
        }
        Ok(Some(record))
    }

    fn list_approval_presentation_records(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<Vec<ApprovalPresentationRecord>, WorkflowOsError> {
        self.ensure_layout()?;
        let directory = self.approval_presentation_run_dir(run_id);
        if !directory.exists() {
            return Ok(Vec::new());
        }
        let paths = json_files_in_dir(&directory).map_err(|_| {
            state_error(
                "approval_presentation_record.read.failed",
                "failed to list approval-presentation records",
            )
        })?;
        let mut records = paths
            .iter()
            .map(|path| {
                let record = read_json::<ApprovalPresentationRecord>(path).map_err(|_| {
                    state_error(
                        "approval_presentation_record.read.corrupt",
                        "approval-presentation record could not be read or validated",
                    )
                })?;
                if record.run_id() != run_id {
                    return Err(state_error(
                        "approval_presentation_record.read.corrupt",
                        "approval-presentation record could not be read or validated",
                    ));
                }
                Ok(record)
            })
            .collect::<Result<Vec<_>, _>>()?;
        records.sort_by(|left, right| left.presentation_id().cmp(right.presentation_id()));
        Ok(records)
    }

    fn list_approval_presentation_records_for_approval(
        &self,
        run_id: &WorkflowRunId,
        approval_id: &str,
    ) -> Result<Vec<ApprovalPresentationRecord>, WorkflowOsError> {
        validate_approval_presentation_approval_id(approval_id)?;
        let records = self.list_approval_presentation_records(run_id)?;
        if records
            .windows(2)
            .any(|pair| !same_approval_presentation_run_identity(&pair[0], &pair[1]))
        {
            return Err(state_error(
                "approval_presentation_record.read.identity_mismatch",
                "approval-presentation record workflow/run identity does not match requested identity",
            ));
        }
        Ok(records
            .into_iter()
            .filter(|record| record.approval_id() == approval_id)
            .collect())
    }
}

impl StateBackend for LocalStateBackend {
    fn health_check(&self) -> Result<BackendHealthCheck, WorkflowOsError> {
        self.ensure_layout()?;
        if let Err(error) = self.validate_event_index_consistency() {
            return Ok(BackendHealthCheck {
                healthy: false,
                backend: "local_filesystem".to_owned(),
                message: format!(
                    "local state backend event log/index consistency check failed: {}",
                    error.message()
                ),
            });
        }
        let probe = self.root.join(".healthcheck");
        write_json_replace(
            &probe,
            &BackendHealthCheck {
                healthy: true,
                backend: "local_filesystem".to_owned(),
                message: "local state backend is writable".to_owned(),
            },
        )?;
        let _ = fs::remove_file(probe);
        Ok(BackendHealthCheck {
            healthy: true,
            backend: "local_filesystem".to_owned(),
            message: "local state backend is writable".to_owned(),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct EventIdRecord {
    run_id: WorkflowRunId,
    sequence_number: EventSequenceNumber,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct SideEffectIdRecord {
    run_id: WorkflowRunId,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct ApprovalPresentationIdRecord {
    run_id: WorkflowRunId,
}

fn same_side_effect_run_identity(left: &SideEffectRecord, right: &SideEffectRecord) -> bool {
    left.run_id() == right.run_id()
        && left.workflow_id() == right.workflow_id()
        && left.workflow_version() == right.workflow_version()
        && left.schema_version() == right.schema_version()
        && left.spec_hash() == right.spec_hash()
}

fn same_approval_presentation_run_identity(
    left: &ApprovalPresentationRecord,
    right: &ApprovalPresentationRecord,
) -> bool {
    left.run_id() == right.run_id()
        && left.workflow_id() == right.workflow_id()
        && left.workflow_version() == right.workflow_version()
        && left.schema_version() == right.schema_version()
}

fn is_allowed_side_effect_lifecycle_update(
    existing: &SideEffectRecord,
    next: &SideEffectRecord,
) -> bool {
    matches!(
        (existing.lifecycle_state(), next.lifecycle_state()),
        (
            SideEffectLifecycleState::Proposed,
            SideEffectLifecycleState::Attempted
        ) | (
            SideEffectLifecycleState::Attempted,
            SideEffectLifecycleState::Completed | SideEffectLifecycleState::Failed
        )
    )
}

fn write_json_create_new<T>(path: &Path, value: &T) -> Result<(), WorkflowOsError>
where
    T: Serialize,
{
    write_json_create_new_atomic(path, value)
}

fn write_json_create_new_atomic<T>(path: &Path, value: &T) -> Result<(), WorkflowOsError>
where
    T: Serialize,
{
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            state_error(
                "state.local.mkdir",
                format!(
                    "failed to create parent directory {}: {error}",
                    parent.display()
                ),
            )
        })?;
    }
    if path.exists() {
        return Err(state_error(
            "state.local.exists",
            format!("state file already exists: {}", path.display()),
        ));
    }
    let temp_path = unique_temp_path(path);
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temp_path)
        .map_err(|error| {
            if error.kind() == std::io::ErrorKind::AlreadyExists {
                state_error(
                    "state.local.exists",
                    format!(
                        "temporary state file already exists: {}",
                        temp_path.display()
                    ),
                )
            } else {
                state_error(
                    "state.local.write",
                    format!(
                        "failed to create temporary state file {}: {error}",
                        temp_path.display()
                    ),
                )
            }
        })?;
    let bytes = serde_json::to_vec_pretty(value).map_err(|error| {
        state_error(
            "state.json.serialize",
            format!("failed to serialize state JSON: {error}"),
        )
    })?;
    file.write_all(&bytes).map_err(|error| {
        state_error(
            "state.local.write",
            format!(
                "failed to write temporary state file {}: {error}",
                temp_path.display()
            ),
        )
    })?;
    file.sync_all().map_err(|error| {
        state_error(
            "state.local.sync",
            format!(
                "failed to sync temporary state file {}: {error}",
                temp_path.display()
            ),
        )
    })?;
    drop(file);

    match fs::hard_link(&temp_path, path) {
        Ok(()) => {
            let _ = fs::remove_file(&temp_path);
            sync_parent_dir(path)?;
            Ok(())
        }
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            let _ = fs::remove_file(&temp_path);
            Err(state_error(
                "state.local.exists",
                format!("state file already exists: {}", path.display()),
            ))
        }
        Err(error) => {
            let _ = fs::remove_file(&temp_path);
            Err(state_error(
                "state.local.link",
                format!(
                    "failed to atomically publish state file {} from {}: {error}",
                    path.display(),
                    temp_path.display()
                ),
            ))
        }
    }
}

fn write_json_replace<T>(path: &Path, value: &T) -> Result<(), WorkflowOsError>
where
    T: Serialize,
{
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            state_error(
                "state.local.mkdir",
                format!(
                    "failed to create parent directory {}: {error}",
                    parent.display()
                ),
            )
        })?;
    }
    let temp_path = path.with_extension("tmp");
    {
        let mut file = File::create(&temp_path).map_err(|error| {
            state_error(
                "state.local.write",
                format!(
                    "failed to create temp state file {}: {error}",
                    temp_path.display()
                ),
            )
        })?;
        let bytes = serde_json::to_vec_pretty(value).map_err(|error| {
            state_error(
                "state.json.serialize",
                format!("failed to serialize state JSON: {error}"),
            )
        })?;
        file.write_all(&bytes).map_err(|error| {
            state_error(
                "state.local.write",
                format!(
                    "failed to write temp state file {}: {error}",
                    temp_path.display()
                ),
            )
        })?;
        file.sync_all().map_err(|error| {
            state_error(
                "state.local.sync",
                format!(
                    "failed to sync temp state file {}: {error}",
                    temp_path.display()
                ),
            )
        })?;
    }
    fs::rename(&temp_path, path).map_err(|error| {
        state_error(
            "state.local.rename",
            format!(
                "failed to replace state file {} with {}: {error}",
                path.display(),
                temp_path.display()
            ),
        )
    })?;
    sync_parent_dir(path)
}

fn json_files_in_dir(directory: &Path) -> Result<Vec<PathBuf>, WorkflowOsError> {
    let mut paths = Vec::new();
    for entry in fs::read_dir(directory).map_err(|error| {
        state_error(
            "state.local.read_dir",
            format!("failed to read directory {}: {error}", directory.display()),
        )
    })? {
        let entry = entry.map_err(|error| {
            state_error(
                "state.local.read_dir_entry",
                format!("failed to inspect directory entry: {error}"),
            )
        })?;
        let path = entry.path();
        if path.is_file()
            && path
                .extension()
                .is_some_and(|extension| extension == "json")
        {
            paths.push(path);
        }
    }
    paths.sort();
    Ok(paths)
}

fn inspection_json_files(directory: &Path, issues: &mut Vec<LocalStateIssue>) -> Vec<PathBuf> {
    match json_files_in_dir(directory) {
        Ok(paths) => paths,
        Err(error) => {
            issues.push(
                LocalStateIssue::new(
                    LocalStateIssueSeverity::Error,
                    error.code().to_owned(),
                    error.message().to_owned(),
                )
                .with_path(directory.to_path_buf()),
            );
            Vec::new()
        }
    }
}

fn unique_temp_path(path: &Path) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos());
    path.with_file_name(format!(
        ".workflow-os-tmp-{}-{}.tmp",
        std::process::id(),
        unique
    ))
}

fn sync_parent_dir(path: &Path) -> Result<(), WorkflowOsError> {
    if let Some(parent) = path.parent() {
        let directory = File::open(parent).map_err(|error| {
            state_error(
                "state.local.sync",
                format!(
                    "failed to open parent directory {} for sync: {error}",
                    parent.display()
                ),
            )
        })?;
        directory.sync_all().map_err(|error| {
            state_error(
                "state.local.sync",
                format!(
                    "failed to sync parent directory {}: {error}",
                    parent.display()
                ),
            )
        })?;
    }
    Ok(())
}

fn read_optional_json<T>(path: &Path) -> Result<Option<T>, WorkflowOsError>
where
    T: for<'de> Deserialize<'de>,
{
    if !path.exists() {
        return Ok(None);
    }
    read_json(path).map(Some)
}

fn read_json<T>(path: &Path) -> Result<T, WorkflowOsError>
where
    T: for<'de> Deserialize<'de>,
{
    let mut text = String::new();
    File::open(path)
        .map_err(|error| {
            state_error(
                "state.local.read",
                format!("failed to open state file {}: {error}", path.display()),
            )
        })?
        .read_to_string(&mut text)
        .map_err(|error| {
            state_error(
                "state.local.read",
                format!("failed to read state file {}: {error}", path.display()),
            )
        })?;
    serde_json::from_str(&text).map_err(|error| {
        state_error(
            "state.corrupt",
            format!("failed to decode state file {}: {error}", path.display()),
        )
    })
}

fn encode_key(value: &str) -> String {
    let digest = Sha256::digest(value.as_bytes());
    let mut output = String::with_capacity(digest.len() * 2);
    for byte in digest {
        let _ = write!(output, "{byte:02x}");
    }
    output
}

fn state_error(code: impl Into<String>, message: impl Into<String>) -> WorkflowOsError {
    WorkflowOsError::new(WorkflowOsErrorKind::InvalidState, code, message)
}

fn validate_append_against_history(
    existing_events: &[WorkflowRunEvent],
    event: &WorkflowRunEvent,
) -> Result<(), WorkflowOsError> {
    if existing_events.is_empty() {
        WorkflowRun::rehydrate(std::slice::from_ref(event))?;
        return Ok(());
    }

    let mut run = WorkflowRun::rehydrate(existing_events)?;
    run.append_event(event.clone())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]

    use std::cell::RefCell;
    use std::collections::{BTreeMap, BTreeSet};
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::*;
    use crate::{
        compute_approval_presentation_content_hash, ApprovalPresentationChannel,
        ApprovalPresentationRecordDefinition, ApprovalPresentationSensitivity, CorrelationId,
        EventSequenceNumber, RedactionDisposition, RedactionFieldState, RedactionMetadata,
        RunRehydration, SchemaVersion, SideEffectAuthority, SideEffectAuthorityDecision,
        SideEffectCapability, SideEffectIdempotencyBinding, SideEffectIdempotencyScope,
        SideEffectLifecycleState, SideEffectOutcomeReference, SideEffectOutcomeReferenceKind,
        SideEffectRecordDefinition, SideEffectReference, SideEffectReferenceKind,
        SideEffectSensitivity, SideEffectTargetKind, SideEffectTargetReference, SpecContentHash,
        StepId, Timestamp, WorkflowId, WorkflowRunEventKind, WorkflowRunStatus, WorkflowVersion,
    };

    static NEXT_TEST_BACKEND: AtomicU64 = AtomicU64::new(1);

    #[derive(Default)]
    struct InMemoryStateBackend {
        events: RefCell<BTreeMap<WorkflowRunId, Vec<WorkflowRunEvent>>>,
        event_ids: RefCell<BTreeSet<EventId>>,
        snapshots: RefCell<BTreeMap<WorkflowRunId, WorkflowRunSnapshot>>,
        idempotency: RefCell<BTreeMap<IdempotencyKey, IdempotencyResult>>,
        locks: RefCell<BTreeMap<String, ActorId>>,
        approvals: RefCell<BTreeMap<String, ApprovalRequest>>,
        projects: RefCell<BTreeMap<ProjectId, ProjectStateRecord>>,
        policy_audit: RefCell<BTreeMap<EventId, PolicyAuditRecord>>,
        adapter_audit: RefCell<BTreeMap<EventId, AdapterRuntimeAuditRecord>>,
        adapter_observability: RefCell<BTreeMap<EventId, AdapterRuntimeObservabilityRecord>>,
        side_effect_records: RefCell<BTreeMap<SideEffectId, SideEffectRecord>>,
        approval_presentation_records:
            RefCell<BTreeMap<ApprovalPresentationId, ApprovalPresentationRecord>>,
    }

    impl EventLogStore for InMemoryStateBackend {
        fn append_event(&self, event: &WorkflowRunEvent) -> Result<(), WorkflowOsError> {
            if self.event_ids.borrow().contains(&event.event_id) {
                return Err(state_error(
                    "state.event.duplicate_id",
                    format!("duplicate event id {}", event.event_id),
                ));
            }
            let mut events = self.events.borrow_mut();
            let run_events = events.entry(event.run_id.clone()).or_default();
            if run_events
                .iter()
                .any(|stored| stored.sequence_number == event.sequence_number)
            {
                return Err(state_error(
                    "state.event.duplicate_sequence",
                    format!(
                        "duplicate sequence {} for run {}",
                        event.sequence_number, event.run_id
                    ),
                ));
            }
            validate_append_against_history(run_events, event)?;
            run_events.push(event.clone());
            run_events.sort_by_key(|event| event.sequence_number);
            self.event_ids.borrow_mut().insert(event.event_id.clone());
            Ok(())
        }

        fn read_events(
            &self,
            run_id: &WorkflowRunId,
        ) -> Result<Vec<WorkflowRunEvent>, WorkflowOsError> {
            Ok(self
                .events
                .borrow()
                .get(run_id)
                .cloned()
                .unwrap_or_default())
        }
    }

    impl RunSnapshotStore for InMemoryStateBackend {
        fn save_snapshot(&self, snapshot: &WorkflowRunSnapshot) -> Result<(), WorkflowOsError> {
            self.snapshots
                .borrow_mut()
                .insert(snapshot.identity.run_id.clone(), snapshot.clone());
            Ok(())
        }

        fn load_snapshot(
            &self,
            run_id: &WorkflowRunId,
        ) -> Result<Option<WorkflowRunSnapshot>, WorkflowOsError> {
            Ok(self.snapshots.borrow().get(run_id).cloned())
        }
    }

    impl IdempotencyStore for InMemoryStateBackend {
        fn record_idempotency_result(
            &self,
            key: &IdempotencyKey,
            result: IdempotencyResult,
        ) -> Result<IdempotencyWrite, WorkflowOsError> {
            let mut idempotency = self.idempotency.borrow_mut();
            if let Some(existing) = idempotency.get(key) {
                return Ok(IdempotencyWrite::Duplicate(existing.clone()));
            }
            idempotency.insert(key.clone(), result.clone());
            Ok(IdempotencyWrite::FirstWrite(result))
        }
    }

    impl LockStore for InMemoryStateBackend {
        fn acquire_lock(&self, key: &str, owner: &ActorId) -> Result<LockLease, WorkflowOsError> {
            let mut locks = self.locks.borrow_mut();
            if locks.contains_key(key) {
                return Err(state_error(
                    "state.lock_contended",
                    format!("lock {key} is already held"),
                ));
            }
            locks.insert(key.to_owned(), owner.clone());
            Ok(LockLease {
                key: key.to_owned(),
                owner: owner.clone(),
            })
        }

        fn release_lock(&self, lease: &LockLease) -> Result<(), WorkflowOsError> {
            self.locks.borrow_mut().remove(&lease.key);
            Ok(())
        }
    }

    impl ApprovalStore for InMemoryStateBackend {
        fn save_approval_request(&self, request: &ApprovalRequest) -> Result<(), WorkflowOsError> {
            self.approvals
                .borrow_mut()
                .insert(request.approval_id.clone(), request.clone());
            Ok(())
        }

        fn load_approval_request(
            &self,
            approval_id: &str,
        ) -> Result<Option<ApprovalRequest>, WorkflowOsError> {
            Ok(self.approvals.borrow().get(approval_id).cloned())
        }

        fn delete_approval_request(&self, approval_id: &str) -> Result<(), WorkflowOsError> {
            self.approvals.borrow_mut().remove(approval_id);
            Ok(())
        }
    }

    impl ProjectStateStore for InMemoryStateBackend {
        fn save_project_state(&self, state: &ProjectStateRecord) -> Result<(), WorkflowOsError> {
            self.projects
                .borrow_mut()
                .insert(state.project_id.clone(), state.clone());
            Ok(())
        }

        fn load_project_state(
            &self,
            project_id: &ProjectId,
        ) -> Result<Option<ProjectStateRecord>, WorkflowOsError> {
            Ok(self.projects.borrow().get(project_id).cloned())
        }
    }

    impl PolicyAuditStore for InMemoryStateBackend {
        fn append_policy_audit_record(
            &self,
            record: &PolicyAuditRecord,
        ) -> Result<(), WorkflowOsError> {
            let mut records = self.policy_audit.borrow_mut();
            if records.contains_key(&record.audit_id) {
                return Err(state_error(
                    "state.policy_audit.duplicate_id",
                    format!("duplicate policy audit id {}", record.audit_id),
                ));
            }
            records.insert(record.audit_id.clone(), record.clone());
            Ok(())
        }

        fn read_policy_audit_records(&self) -> Result<Vec<PolicyAuditRecord>, WorkflowOsError> {
            Ok(self.policy_audit.borrow().values().cloned().collect())
        }
    }

    impl AdapterTelemetryStore for InMemoryStateBackend {
        fn append_adapter_audit_record(
            &self,
            record: &AdapterRuntimeAuditRecord,
        ) -> Result<(), WorkflowOsError> {
            let mut records = self.adapter_audit.borrow_mut();
            if records.contains_key(&record.telemetry_id) {
                return Err(state_error(
                    "state.adapter_audit.duplicate_id",
                    format!("duplicate adapter audit id {}", record.telemetry_id),
                ));
            }
            records.insert(record.telemetry_id.clone(), record.clone());
            Ok(())
        }

        fn read_adapter_audit_records(
            &self,
            run_id: &WorkflowRunId,
        ) -> Result<Vec<AdapterRuntimeAuditRecord>, WorkflowOsError> {
            Ok(self
                .adapter_audit
                .borrow()
                .values()
                .filter(|record| record.workflow_run_id.as_ref() == Some(run_id))
                .cloned()
                .collect())
        }

        fn append_adapter_observability_record(
            &self,
            record: &AdapterRuntimeObservabilityRecord,
        ) -> Result<(), WorkflowOsError> {
            let mut records = self.adapter_observability.borrow_mut();
            if records.contains_key(&record.telemetry_id) {
                return Err(state_error(
                    "state.adapter_observability.duplicate_id",
                    format!("duplicate adapter observability id {}", record.telemetry_id),
                ));
            }
            records.insert(record.telemetry_id.clone(), record.clone());
            Ok(())
        }

        fn read_adapter_observability_records(
            &self,
            run_id: &WorkflowRunId,
        ) -> Result<Vec<AdapterRuntimeObservabilityRecord>, WorkflowOsError> {
            Ok(self
                .adapter_observability
                .borrow()
                .values()
                .filter(|record| record.workflow_run_id.as_ref() == Some(run_id))
                .cloned()
                .collect())
        }
    }

    impl SideEffectRecordStore for InMemoryStateBackend {
        fn write_side_effect_record(
            &self,
            record: &SideEffectRecord,
        ) -> Result<(), WorkflowOsError> {
            record.validate()?;
            let mut records = self.side_effect_records.borrow_mut();
            if records.contains_key(record.side_effect_id()) {
                return Err(state_error(
                    "side_effect_record.write.duplicate",
                    "side-effect record already exists",
                ));
            }
            if records.values().any(|existing| {
                existing.run_id() == record.run_id()
                    && !same_side_effect_run_identity(existing, record)
            }) {
                return Err(state_error(
                    "side_effect_record.write.identity_mismatch",
                    "side-effect record workflow/run identity conflicts with existing records",
                ));
            }
            records.insert(record.side_effect_id().clone(), record.clone());
            Ok(())
        }

        fn update_side_effect_record(
            &self,
            record: &SideEffectRecord,
        ) -> Result<(), WorkflowOsError> {
            record.validate()?;
            let mut records = self.side_effect_records.borrow_mut();
            let existing = records.get(record.side_effect_id()).ok_or_else(|| {
                state_error(
                    "side_effect_record.update.missing",
                    "side-effect record does not exist",
                )
            })?;
            if !same_side_effect_run_identity(existing, record) {
                return Err(state_error(
                    "side_effect_record.update.identity_mismatch",
                    "side-effect record workflow/run identity conflicts with existing record",
                ));
            }
            if !is_allowed_side_effect_lifecycle_update(existing, record) {
                return Err(state_error(
                    "side_effect_record.update.invalid_lifecycle_transition",
                    "side-effect record update lifecycle transition is not supported",
                ));
            }
            records.insert(record.side_effect_id().clone(), record.clone());
            Ok(())
        }

        fn read_side_effect_record(
            &self,
            side_effect_id: &SideEffectId,
        ) -> Result<Option<SideEffectRecord>, WorkflowOsError> {
            Ok(self
                .side_effect_records
                .borrow()
                .get(side_effect_id)
                .cloned())
        }

        fn list_side_effect_records(
            &self,
            run_id: &WorkflowRunId,
        ) -> Result<Vec<SideEffectRecord>, WorkflowOsError> {
            let mut records = self
                .side_effect_records
                .borrow()
                .values()
                .filter(|record| record.run_id() == run_id)
                .cloned()
                .collect::<Vec<_>>();
            records.sort_by(|left, right| left.side_effect_id().cmp(right.side_effect_id()));
            Ok(records)
        }

        fn list_side_effect_records_for_workflow_run(
            &self,
            workflow_id: &WorkflowId,
            run_id: &WorkflowRunId,
        ) -> Result<Vec<SideEffectRecord>, WorkflowOsError> {
            let records = self.list_side_effect_records(run_id)?;
            if records
                .iter()
                .any(|record| record.workflow_id() != workflow_id)
                || records
                    .windows(2)
                    .any(|pair| !same_side_effect_run_identity(&pair[0], &pair[1]))
            {
                return Err(state_error(
                    "side_effect_record.read.identity_mismatch",
                    "side-effect record workflow/run identity does not match requested identity",
                ));
            }
            Ok(records)
        }
    }

    impl ApprovalPresentationRecordStore for InMemoryStateBackend {
        fn write_approval_presentation_record(
            &self,
            record: &ApprovalPresentationRecord,
        ) -> Result<(), WorkflowOsError> {
            let mut records = self.approval_presentation_records.borrow_mut();
            if records.contains_key(record.presentation_id()) {
                return Err(state_error(
                    "approval_presentation_record.write.duplicate",
                    "approval-presentation record already exists",
                ));
            }
            if records.values().any(|existing| {
                existing.run_id() == record.run_id()
                    && !same_approval_presentation_run_identity(existing, record)
            }) {
                return Err(state_error(
                    "approval_presentation_record.write.identity_mismatch",
                    "approval-presentation record workflow/run identity conflicts with existing records",
                ));
            }
            records.insert(record.presentation_id().clone(), record.clone());
            Ok(())
        }

        fn read_approval_presentation_record(
            &self,
            presentation_id: &ApprovalPresentationId,
        ) -> Result<Option<ApprovalPresentationRecord>, WorkflowOsError> {
            Ok(self
                .approval_presentation_records
                .borrow()
                .get(presentation_id)
                .cloned())
        }

        fn list_approval_presentation_records(
            &self,
            run_id: &WorkflowRunId,
        ) -> Result<Vec<ApprovalPresentationRecord>, WorkflowOsError> {
            let mut records = self
                .approval_presentation_records
                .borrow()
                .values()
                .filter(|record| record.run_id() == run_id)
                .cloned()
                .collect::<Vec<_>>();
            records.sort_by(|left, right| left.presentation_id().cmp(right.presentation_id()));
            Ok(records)
        }

        fn list_approval_presentation_records_for_approval(
            &self,
            run_id: &WorkflowRunId,
            approval_id: &str,
        ) -> Result<Vec<ApprovalPresentationRecord>, WorkflowOsError> {
            validate_approval_presentation_approval_id(approval_id)?;
            let records = self.list_approval_presentation_records(run_id)?;
            if records
                .windows(2)
                .any(|pair| !same_approval_presentation_run_identity(&pair[0], &pair[1]))
            {
                return Err(state_error(
                    "approval_presentation_record.read.identity_mismatch",
                    "approval-presentation record workflow/run identity does not match requested identity",
                ));
            }
            Ok(records
                .into_iter()
                .filter(|record| record.approval_id() == approval_id)
                .collect())
        }
    }

    impl StateBackend for InMemoryStateBackend {
        fn health_check(&self) -> Result<BackendHealthCheck, WorkflowOsError> {
            Ok(BackendHealthCheck {
                healthy: true,
                backend: "in_memory_test".to_owned(),
                message: "in-memory test backend is available".to_owned(),
            })
        }
    }

    #[derive(Clone)]
    struct Fixture {
        id: u64,
        run_id: WorkflowRunId,
        workflow_id: WorkflowId,
        schema_version: SchemaVersion,
        workflow_version: WorkflowVersion,
        spec_hash: SpecContentHash,
    }

    impl Fixture {
        fn new() -> Self {
            let id = NEXT_TEST_BACKEND.fetch_add(1, Ordering::Relaxed);
            Self {
                id,
                run_id: WorkflowRunId::new(format!("run-state-{id}")).expect("run id"),
                workflow_id: WorkflowId::new(format!("workflow/state-{id}")).expect("workflow id"),
                schema_version: SchemaVersion::new("workflowos.dev/v0").expect("schema version"),
                workflow_version: WorkflowVersion::new("v0").expect("workflow version"),
                spec_hash: SpecContentHash::from_text(&format!("state backend spec {id}")),
            }
        }

        fn event(&self, sequence: u64, kind: WorkflowRunEventKind) -> WorkflowRunEvent {
            WorkflowRunEvent {
                sequence_number: EventSequenceNumber::new(sequence).expect("sequence"),
                event_id: EventId::new(format!("event-{}-{sequence}", self.id)).expect("event id"),
                timestamp: crate::Timestamp::parse_rfc3339("2026-01-01T00:00:00Z")
                    .expect("timestamp"),
                run_id: self.run_id.clone(),
                workflow_id: self.workflow_id.clone(),
                schema_version: self.schema_version.clone(),
                workflow_version: self.workflow_version.clone(),
                spec_content_hash: self.spec_hash.clone(),
                correlation_id: Some(CorrelationId::new("correlation-state").expect("correlation")),
                actor: Some(ActorId::new("system").expect("actor")),
                idempotency_key: None,
                kind,
            }
        }

        fn created(&self) -> WorkflowRunEvent {
            self.event(
                1,
                WorkflowRunEventKind::RunCreated {
                    summary: None,
                    immutable_run_bundle: None,
                },
            )
        }

        fn validated(&self) -> WorkflowRunEvent {
            self.event(2, WorkflowRunEventKind::RunValidated)
        }

        fn side_effect_record(&self, suffix: &str, workflow_id: WorkflowId) -> SideEffectRecord {
            self.side_effect_record_with_identity(
                suffix,
                workflow_id,
                self.workflow_version.clone(),
                self.schema_version.clone(),
                self.spec_hash.clone(),
            )
        }

        fn side_effect_record_with_identity(
            &self,
            suffix: &str,
            workflow_id: WorkflowId,
            workflow_version: WorkflowVersion,
            schema_version: SchemaVersion,
            spec_hash: SpecContentHash,
        ) -> SideEffectRecord {
            self.side_effect_record_with_identity_and_lifecycle(
                suffix,
                workflow_id,
                workflow_version,
                schema_version,
                spec_hash,
                SideEffectLifecycleState::Proposed,
            )
        }

        fn side_effect_record_with_lifecycle(
            &self,
            suffix: &str,
            lifecycle_state: SideEffectLifecycleState,
        ) -> SideEffectRecord {
            self.side_effect_record_with_identity_and_lifecycle(
                suffix,
                self.workflow_id.clone(),
                self.workflow_version.clone(),
                self.schema_version.clone(),
                self.spec_hash.clone(),
                lifecycle_state,
            )
        }

        fn side_effect_record_with_identity_and_lifecycle(
            &self,
            suffix: &str,
            workflow_id: WorkflowId,
            workflow_version: WorkflowVersion,
            schema_version: SchemaVersion,
            spec_hash: SpecContentHash,
            lifecycle_state: SideEffectLifecycleState,
        ) -> SideEffectRecord {
            let parts = self.side_effect_lifecycle_parts(suffix, lifecycle_state);
            SideEffectRecord::new(SideEffectRecordDefinition {
                side_effect_id: SideEffectId::new(format!("side-effect-{}-{suffix}", self.id))
                    .expect("side-effect id"),
                lifecycle_state,
                target: SideEffectTargetReference::new(
                    SideEffectTargetKind::WorkflowResource,
                    format!("workflow/{}/side-effect/{suffix}", self.id),
                )
                .expect("target reference"),
                capability: SideEffectCapability::ExternalWrite,
                authority: SideEffectAuthority::new(
                    parts.authority_decision,
                    vec![SideEffectReference::new(
                        SideEffectReferenceKind::PolicyDecision,
                        format!("policy/{}/side-effect/{suffix}", self.id),
                    )
                    .expect("policy reference")],
                    parts.approval_references,
                )
                .expect("authority"),
                actor: Some(ActorId::new("system/state-test").expect("actor")),
                system_actor: None,
                workflow_id,
                workflow_version,
                schema_version,
                spec_hash,
                run_id: self.run_id.clone(),
                step_id: None,
                skill_id: None,
                skill_version: None,
                adapter_id: None,
                adapter_kind: None,
                integration_id: None,
                idempotency: SideEffectIdempotencyBinding::new(
                    IdempotencyKey::new(format!("side-effect/state/{}/{}", self.id, suffix))
                        .expect("idempotency key"),
                    SideEffectIdempotencyScope::Run,
                    None,
                    None,
                )
                .expect("idempotency binding"),
                references: Vec::new(),
                outcome_reference: parts.outcome_reference,
                created_at: Timestamp::parse_rfc3339("2026-01-01T00:00:00Z").expect("timestamp"),
                updated_at: None,
                correlation_id: Some(
                    CorrelationId::new(format!("correlation-side-effect-{suffix}"))
                        .expect("correlation"),
                ),
                summary: Some("bounded side-effect proposal".to_owned()),
                reason_codes: parts.reason_codes,
                sensitivity: SideEffectSensitivity::Confidential,
                redaction: crate::RedactionMetadata::empty(),
            })
            .expect("valid side-effect record")
        }

        fn approval_presentation_record(
            &self,
            suffix: &str,
            approval_id: &str,
        ) -> ApprovalPresentationRecord {
            let strict_non_goals = vec!["no approval behavior changes".to_owned()];
            let touched_surfaces = vec!["approval presentation persistence".to_owned()];
            let validation_expectations = vec!["cargo test -p workflow-core".to_owned()];
            let channel = ApprovalPresentationChannel::Terminal;
            let sensitivity = ApprovalPresentationSensitivity::Internal;
            let presentation_id =
                ApprovalPresentationId::new(format!("presentation-{}-{suffix}", self.id))
                    .expect("presentation id");
            let step_id = StepId::new("approval-presentation-store").expect("step id");
            let hash = compute_approval_presentation_content_hash(
                &self.run_id,
                approval_id,
                &self.workflow_id,
                Some(&self.workflow_version),
                Some(&self.schema_version),
                Some(&step_id),
                "approve persistence helper",
                "persist approval presentation proof",
                "local persistence helper only",
                &strict_non_goals,
                &touched_surfaces,
                &validation_expectations,
                "store proof before enforcement",
                "run validation",
                &channel,
                sensitivity,
            )
            .expect("content hash");
            ApprovalPresentationRecord::new(ApprovalPresentationRecordDefinition {
                presentation_id,
                run_id: self.run_id.clone(),
                approval_id: approval_id.to_owned(),
                workflow_id: self.workflow_id.clone(),
                workflow_version: Some(self.workflow_version.clone()),
                schema_version: Some(self.schema_version.clone()),
                step_id: Some(step_id),
                requested_action: "approve persistence helper".to_owned(),
                work_summary: "persist approval presentation proof".to_owned(),
                approved_scope: "local persistence helper only".to_owned(),
                strict_non_goals,
                expected_touched_surfaces: touched_surfaces,
                validation_expectations,
                why_now: "store proof before enforcement".to_owned(),
                next_action: "run validation".to_owned(),
                presented_at: Timestamp::parse_rfc3339("2026-01-01T00:00:00Z").expect("timestamp"),
                presented_by: ActorId::new("system/state-test").expect("actor"),
                channel,
                content_hash: hash,
                redaction: RedactionMetadata {
                    redacted_fields: vec!["approval_handoff".to_owned()],
                    field_states: vec![RedactionFieldState {
                        field: "approval_handoff".to_owned(),
                        disposition: RedactionDisposition::ReferenceOnly,
                        reason: "bounded approval presentation proof".to_owned(),
                    }],
                },
                sensitivity,
            })
            .expect("approval presentation record")
        }

        fn side_effect_lifecycle_parts(
            &self,
            suffix: &str,
            lifecycle_state: SideEffectLifecycleState,
        ) -> SideEffectLifecycleParts {
            match lifecycle_state {
                SideEffectLifecycleState::Proposed => SideEffectLifecycleParts::new(
                    SideEffectAuthorityDecision::NotEvaluated,
                    Vec::new(),
                    None,
                    Vec::new(),
                ),
                SideEffectLifecycleState::Attempted => SideEffectLifecycleParts::new(
                    SideEffectAuthorityDecision::AllowedByPolicy,
                    Vec::new(),
                    None,
                    Vec::new(),
                ),
                SideEffectLifecycleState::Completed => SideEffectLifecycleParts::new(
                    SideEffectAuthorityDecision::ApprovedByHuman,
                    vec![SideEffectReference::new(
                        SideEffectReferenceKind::ApprovalDecision,
                        format!("approval/{}/side-effect/{suffix}", self.id),
                    )
                    .expect("approval reference")],
                    Some(
                        SideEffectOutcomeReference::new(
                            SideEffectOutcomeReferenceKind::Outcome,
                            format!("outcome/{}/side-effect/{suffix}", self.id),
                        )
                        .expect("outcome reference"),
                    ),
                    Vec::new(),
                ),
                SideEffectLifecycleState::Failed => SideEffectLifecycleParts::new(
                    SideEffectAuthorityDecision::AllowedByPolicy,
                    Vec::new(),
                    Some(
                        SideEffectOutcomeReference::new(
                            SideEffectOutcomeReferenceKind::Failure,
                            format!("failure/{}/side-effect/{suffix}", self.id),
                        )
                        .expect("failure reference"),
                    ),
                    vec!["provider.failed".to_owned()],
                ),
                SideEffectLifecycleState::Denied | SideEffectLifecycleState::Skipped => {
                    SideEffectLifecycleParts::new(
                        SideEffectAuthorityDecision::DeniedByPolicy,
                        Vec::new(),
                        None,
                        vec!["policy.denied".to_owned()],
                    )
                }
            }
        }
    }

    struct SideEffectLifecycleParts {
        authority_decision: SideEffectAuthorityDecision,
        approval_references: Vec<SideEffectReference>,
        outcome_reference: Option<SideEffectOutcomeReference>,
        reason_codes: Vec<String>,
    }

    impl SideEffectLifecycleParts {
        fn new(
            authority_decision: SideEffectAuthorityDecision,
            approval_references: Vec<SideEffectReference>,
            outcome_reference: Option<SideEffectOutcomeReference>,
            reason_codes: Vec<String>,
        ) -> Self {
            Self {
                authority_decision,
                approval_references,
                outcome_reference,
                reason_codes,
            }
        }
    }

    fn contract_append_and_read_events(backend: &impl StateBackend) {
        let fixture = Fixture::new();
        backend
            .append_event(&fixture.created())
            .expect("append created");
        backend
            .append_event(&fixture.validated())
            .expect("append validated");

        let events = backend.read_events(&fixture.run_id).expect("read events");

        assert_eq!(events.len(), 2);
        assert_eq!(events[0].sequence_number.get(), 1);
        assert_eq!(events[1].sequence_number.get(), 2);
    }

    fn contract_reject_duplicate_event_id(backend: &impl StateBackend) {
        let fixture = Fixture::new();
        let first = fixture.created();
        let mut duplicate = fixture.validated();
        duplicate.event_id = first.event_id.clone();

        backend.append_event(&first).expect("append first");
        let error = backend
            .append_event(&duplicate)
            .expect_err("duplicate event id is rejected");

        assert_eq!(error.code(), "state.event.duplicate_id");
    }

    fn contract_reject_duplicate_sequence(backend: &impl StateBackend) {
        let fixture = Fixture::new();
        let first = fixture.created();
        let mut duplicate_sequence = fixture.event(1, WorkflowRunEventKind::RunValidated);
        duplicate_sequence.event_id =
            EventId::new(format!("event-{}-duplicate", fixture.id)).expect("event id");

        backend.append_event(&first).expect("append first");
        let error = backend
            .append_event(&duplicate_sequence)
            .expect_err("duplicate sequence is rejected");

        assert_eq!(error.code(), "state.event.duplicate_sequence");
    }

    fn contract_rehydrate_from_backend(backend: &impl StateBackend) {
        let fixture = Fixture::new();
        backend
            .append_event(&fixture.created())
            .expect("append created");
        backend
            .append_event(&fixture.validated())
            .expect("append validated");

        let run = backend
            .rehydrate_run(&fixture.run_id)
            .expect("rehydrates from backend");

        assert_eq!(run.snapshot.status, WorkflowRunStatus::Validated);
    }

    fn contract_reject_non_contiguous_sequence(backend: &impl StateBackend) {
        let fixture = Fixture::new();
        backend
            .append_event(&fixture.created())
            .expect("append created");
        let skipped_sequence = fixture.event(3, WorkflowRunEventKind::RunValidated);

        let error = backend
            .append_event(&skipped_sequence)
            .expect_err("non-contiguous sequence is rejected before write");

        assert_eq!(error.code(), "runtime.sequence.non_contiguous");
        assert_eq!(
            backend
                .read_events(&fixture.run_id)
                .expect("read events")
                .len(),
            1
        );
    }

    fn contract_reject_invalid_transition(backend: &impl StateBackend) {
        let fixture = Fixture::new();
        backend
            .append_event(&fixture.created())
            .expect("append created");

        let error = backend
            .append_event(&fixture.event(2, WorkflowRunEventKind::RunStarted))
            .expect_err("invalid transition is rejected before write");

        assert_eq!(error.code(), "runtime.transition.invalid");
        assert_eq!(
            backend
                .read_events(&fixture.run_id)
                .expect("read events")
                .len(),
            1
        );
    }

    fn contract_reject_terminal_state_mutation(backend: &impl StateBackend) {
        let fixture = Fixture::new();
        for event in [
            fixture.created(),
            fixture.validated(),
            fixture.event(3, WorkflowRunEventKind::RunStarted),
            fixture.event(4, WorkflowRunEventKind::RunCompleted),
        ] {
            backend.append_event(&event).expect("append valid event");
        }

        let error = backend
            .append_event(&fixture.event(
                5,
                WorkflowRunEventKind::StepScheduled {
                    step_id: crate::StepId::new("after-terminal").expect("step id"),
                },
            ))
            .expect_err("terminal mutation is rejected before write");

        assert_eq!(error.code(), "runtime.transition.invalid");
        assert_eq!(
            backend
                .read_events(&fixture.run_id)
                .expect("read events")
                .len(),
            4
        );
    }

    fn contract_reject_mismatched_workflow_id(backend: &impl StateBackend) {
        let fixture = Fixture::new();
        backend
            .append_event(&fixture.created())
            .expect("append created");
        let mut event = fixture.validated();
        event.workflow_id = WorkflowId::new("workflow/other").expect("workflow id");

        let error = backend
            .append_event(&event)
            .expect_err("workflow id mismatch is rejected before write");

        assert_eq!(error.code(), "runtime.identity.mismatch");
    }

    fn contract_reject_mismatched_workflow_version(backend: &impl StateBackend) {
        let fixture = Fixture::new();
        backend
            .append_event(&fixture.created())
            .expect("append created");
        let mut event = fixture.validated();
        event.workflow_version = WorkflowVersion::new("v1").expect("workflow version");

        let error = backend
            .append_event(&event)
            .expect_err("workflow version mismatch is rejected before write");

        assert_eq!(error.code(), "runtime.identity.mismatch");
    }

    fn contract_reject_mismatched_schema_version(backend: &impl StateBackend) {
        let fixture = Fixture::new();
        backend
            .append_event(&fixture.created())
            .expect("append created");
        let mut event = fixture.validated();
        event.schema_version = SchemaVersion::new("workflowos.dev/v1").expect("schema version");

        let error = backend
            .append_event(&event)
            .expect_err("schema version mismatch is rejected before write");

        assert_eq!(error.code(), "runtime.identity.mismatch");
    }

    fn contract_reject_mismatched_spec_hash(backend: &impl StateBackend) {
        let fixture = Fixture::new();
        backend
            .append_event(&fixture.created())
            .expect("append created");
        let mut event = fixture.validated();
        event.spec_content_hash = SpecContentHash::from_text("different spec");

        let error = backend
            .append_event(&event)
            .expect_err("spec hash mismatch is rejected before write");

        assert_eq!(error.code(), "runtime.identity.mismatch");
    }

    fn contract_idempotency_first_write_wins(backend: &impl StateBackend) {
        let key = IdempotencyKey::new("idem/state").expect("idempotency key");
        let first = backend
            .record_idempotency_result(
                &key,
                IdempotencyResult {
                    result_ref: "first".to_owned(),
                },
            )
            .expect("first idempotency write");
        let second = backend
            .record_idempotency_result(
                &key,
                IdempotencyResult {
                    result_ref: "second".to_owned(),
                },
            )
            .expect("duplicate idempotency write");

        assert_eq!(
            first,
            IdempotencyWrite::FirstWrite(IdempotencyResult {
                result_ref: "first".to_owned()
            })
        );
        assert_eq!(
            second,
            IdempotencyWrite::Duplicate(IdempotencyResult {
                result_ref: "first".to_owned()
            })
        );
    }

    fn contract_lock_acquire_release_and_contention(backend: &impl StateBackend) {
        let owner = ActorId::new("worker").expect("actor");
        let lease = backend
            .acquire_lock("run/run-state", &owner)
            .expect("lock acquired");
        let contention = backend
            .acquire_lock("run/run-state", &owner)
            .expect_err("contention rejected");
        assert_eq!(contention.code(), "state.lock_contended");

        backend.release_lock(&lease).expect("lock released");
        let reacquired = backend
            .acquire_lock("run/run-state", &owner)
            .expect("lock reacquired");
        backend.release_lock(&reacquired).expect("lock released");
    }

    fn contract_health_check(backend: &impl StateBackend) {
        let health = backend.health_check().expect("health check");

        assert!(health.healthy);
    }

    fn contract_snapshot_projection(backend: &impl StateBackend) {
        let fixture = Fixture::new();
        let snapshot = RunRehydration::rehydrate(&[fixture.created()]).expect("snapshot");
        backend.save_snapshot(&snapshot).expect("save snapshot");

        let loaded = backend
            .load_snapshot(&fixture.run_id)
            .expect("load snapshot")
            .expect("snapshot exists");

        assert_eq!(loaded.identity.run_id, fixture.run_id);
    }

    fn contract_policy_audit_append_and_read(backend: &impl StateBackend) {
        let fixture = Fixture::new();
        let record = PolicyAuditRecord {
            audit_id: EventId::generate(),
            timestamp: Timestamp::parse_rfc3339("2026-01-01T00:00:00Z").expect("timestamp"),
            scope: crate::PolicyAuditScope::PreRun,
            workflow_event_id: None,
            action: crate::Action::StartWorkflow,
            capabilities: vec![crate::Capability::LocalRead, crate::Capability::AuditWrite],
            allowed: true,
            requires_approval: false,
            reason_codes: vec!["policy.allow.default_conservative".to_owned()],
            violations: Vec::new(),
            actor: Some(ActorId::new("system/test").expect("actor")),
            workflow_id: Some(fixture.workflow_id.clone()),
            schema_version: Some(fixture.schema_version.clone()),
            workflow_version: Some(fixture.workflow_version.clone()),
            workflow_run_id: Some(fixture.run_id.clone()),
            spec_hash: Some(fixture.spec_hash.clone()),
            step_id: None,
            skill_id: None,
            correlation_id: Some(CorrelationId::new("correlation/state").expect("correlation")),
            idempotency_key: None,
            redaction: crate::RedactionMetadata::empty(),
            policy_context: "allow; action=StartWorkflow".to_owned(),
            source_component: "workflow-core.state-test".to_owned(),
        };

        backend
            .append_policy_audit_record(&record)
            .expect("append policy audit");
        let records = backend
            .read_policy_audit_records()
            .expect("read policy audit");

        assert!(records.iter().any(|stored| stored == &record));
    }

    fn contract_side_effect_record_write_read_and_list(backend: &impl SideEffectRecordStore) {
        let fixture = Fixture::new();
        let first = fixture.side_effect_record("a", fixture.workflow_id.clone());
        let second = fixture.side_effect_record("b", fixture.workflow_id.clone());

        backend
            .write_side_effect_record(&second)
            .expect("second record written");
        backend
            .write_side_effect_record(&first)
            .expect("first record written");
        let read = backend
            .read_side_effect_record(first.side_effect_id())
            .expect("record read")
            .expect("record exists");
        let listed = backend
            .list_side_effect_records(&fixture.run_id)
            .expect("records listed");
        let workflow_listed = backend
            .list_side_effect_records_for_workflow_run(&fixture.workflow_id, &fixture.run_id)
            .expect("workflow records listed");

        assert_eq!(read, first);
        assert_eq!(listed, vec![first.clone(), second.clone()]);
        assert_eq!(workflow_listed, vec![first, second]);
    }

    fn contract_side_effect_record_rejects_duplicate_id(backend: &impl SideEffectRecordStore) {
        let fixture = Fixture::new();
        let record = fixture.side_effect_record("duplicate", fixture.workflow_id.clone());
        backend
            .write_side_effect_record(&record)
            .expect("record written");

        let error = backend
            .write_side_effect_record(&record)
            .expect_err("duplicate record rejected");

        assert_eq!(error.code(), "side_effect_record.write.duplicate");
        assert!(!error.to_string().contains(record.side_effect_id().as_str()));
        assert!(!error.to_string().contains(fixture.run_id.as_str()));
    }

    fn contract_side_effect_record_rejects_workflow_identity_mismatch(
        backend: &impl SideEffectRecordStore,
    ) {
        let fixture = Fixture::new();
        let record = fixture.side_effect_record("identity", fixture.workflow_id.clone());
        let mismatched = fixture.side_effect_record(
            "identity-mismatch",
            WorkflowId::new("workflow/other-state").expect("workflow id"),
        );
        backend
            .write_side_effect_record(&record)
            .expect("record written");

        let error = backend
            .write_side_effect_record(&mismatched)
            .expect_err("identity mismatch rejected");

        assert_eq!(error.code(), "side_effect_record.write.identity_mismatch");
        assert!(!error.to_string().contains("workflow/other-state"));
        assert!(!error.to_string().contains(fixture.run_id.as_str()));
    }

    fn contract_side_effect_record_rejects_workflow_version_mismatch(
        backend: &impl SideEffectRecordStore,
    ) {
        let fixture = Fixture::new();
        let record = fixture.side_effect_record("version", fixture.workflow_id.clone());
        let mismatched = fixture.side_effect_record_with_identity(
            "version-mismatch",
            fixture.workflow_id.clone(),
            WorkflowVersion::new("v1").expect("workflow version"),
            fixture.schema_version.clone(),
            fixture.spec_hash.clone(),
        );
        backend
            .write_side_effect_record(&record)
            .expect("record written");

        let error = backend
            .write_side_effect_record(&mismatched)
            .expect_err("workflow version mismatch rejected");

        assert_eq!(error.code(), "side_effect_record.write.identity_mismatch");
        assert!(!error.to_string().contains("v1"));
        assert!(!error.to_string().contains(fixture.run_id.as_str()));
    }

    fn contract_side_effect_record_rejects_schema_version_mismatch(
        backend: &impl SideEffectRecordStore,
    ) {
        let fixture = Fixture::new();
        let record = fixture.side_effect_record("schema", fixture.workflow_id.clone());
        let mismatched = fixture.side_effect_record_with_identity(
            "schema-mismatch",
            fixture.workflow_id.clone(),
            fixture.workflow_version.clone(),
            SchemaVersion::new("workflowos.dev/v1").expect("schema version"),
            fixture.spec_hash.clone(),
        );
        backend
            .write_side_effect_record(&record)
            .expect("record written");

        let error = backend
            .write_side_effect_record(&mismatched)
            .expect_err("schema version mismatch rejected");

        assert_eq!(error.code(), "side_effect_record.write.identity_mismatch");
        assert!(!error.to_string().contains("workflowos.dev/v1"));
        assert!(!error.to_string().contains(fixture.run_id.as_str()));
    }

    fn contract_side_effect_record_rejects_spec_hash_mismatch(
        backend: &impl SideEffectRecordStore,
    ) {
        let fixture = Fixture::new();
        let record = fixture.side_effect_record("spec", fixture.workflow_id.clone());
        let mismatched_hash = SpecContentHash::from_text("mismatched side-effect store spec");
        let mismatched = fixture.side_effect_record_with_identity(
            "spec-mismatch",
            fixture.workflow_id.clone(),
            fixture.workflow_version.clone(),
            fixture.schema_version.clone(),
            mismatched_hash.clone(),
        );
        backend
            .write_side_effect_record(&record)
            .expect("record written");

        let error = backend
            .write_side_effect_record(&mismatched)
            .expect_err("spec hash mismatch rejected");

        assert_eq!(error.code(), "side_effect_record.write.identity_mismatch");
        assert!(!error.to_string().contains(mismatched_hash.as_str()));
        assert!(!error.to_string().contains(fixture.run_id.as_str()));
    }

    fn contract_side_effect_record_update_accepts_allowed_lifecycle_transition(
        backend: &impl SideEffectRecordStore,
    ) {
        let fixture = Fixture::new();
        let proposed = fixture.side_effect_record_with_lifecycle(
            "allowed-update",
            SideEffectLifecycleState::Proposed,
        );
        let attempted = fixture.side_effect_record_with_lifecycle(
            "allowed-update",
            SideEffectLifecycleState::Attempted,
        );

        backend
            .write_side_effect_record(&proposed)
            .expect("proposed record written");
        backend
            .update_side_effect_record(&attempted)
            .expect("allowed lifecycle update succeeds");
        let stored = backend
            .read_side_effect_record(proposed.side_effect_id())
            .expect("record read")
            .expect("record exists");

        assert_eq!(
            stored.lifecycle_state(),
            SideEffectLifecycleState::Attempted
        );
    }

    fn contract_side_effect_record_update_rejects_invalid_lifecycle_transition(
        backend: &impl SideEffectRecordStore,
    ) {
        let fixture = Fixture::new();
        let proposed = fixture.side_effect_record_with_lifecycle(
            "invalid-update",
            SideEffectLifecycleState::Proposed,
        );
        let completed = fixture.side_effect_record_with_lifecycle(
            "invalid-update",
            SideEffectLifecycleState::Completed,
        );

        backend
            .write_side_effect_record(&proposed)
            .expect("proposed record written");
        let error = backend
            .update_side_effect_record(&completed)
            .expect_err("invalid lifecycle update rejected");

        assert_eq!(
            error.code(),
            "side_effect_record.update.invalid_lifecycle_transition"
        );
        assert!(!error
            .to_string()
            .contains(proposed.side_effect_id().as_str()));
        assert!(!error.to_string().contains(fixture.run_id.as_str()));
    }

    fn contract_side_effect_record_update_rejects_same_state_replace(
        backend: &impl SideEffectRecordStore,
    ) {
        let fixture = Fixture::new();
        let attempted = fixture.side_effect_record_with_lifecycle(
            "same-state-update",
            SideEffectLifecycleState::Attempted,
        );
        let replacement = fixture.side_effect_record_with_lifecycle(
            "same-state-update",
            SideEffectLifecycleState::Attempted,
        );

        backend
            .write_side_effect_record(&attempted)
            .expect("attempted record written");
        let error = backend
            .update_side_effect_record(&replacement)
            .expect_err("same-state replacement rejected");

        assert_eq!(
            error.code(),
            "side_effect_record.update.invalid_lifecycle_transition"
        );
        assert!(!error
            .to_string()
            .contains(attempted.side_effect_id().as_str()));
        assert!(!error.to_string().contains(fixture.run_id.as_str()));
    }

    fn run_backend_contract(backend: &impl StateBackend) {
        contract_append_and_read_events(backend);
        contract_reject_duplicate_event_id(backend);
        contract_reject_duplicate_sequence(backend);
        contract_rehydrate_from_backend(backend);
        contract_reject_non_contiguous_sequence(backend);
        contract_reject_invalid_transition(backend);
        contract_reject_terminal_state_mutation(backend);
        contract_reject_mismatched_workflow_id(backend);
        contract_reject_mismatched_workflow_version(backend);
        contract_reject_mismatched_schema_version(backend);
        contract_reject_mismatched_spec_hash(backend);
        contract_idempotency_first_write_wins(backend);
        contract_lock_acquire_release_and_contention(backend);
        contract_health_check(backend);
        contract_snapshot_projection(backend);
        contract_policy_audit_append_and_read(backend);
    }

    fn run_side_effect_record_store_contract(backend: &impl SideEffectRecordStore) {
        contract_side_effect_record_write_read_and_list(backend);
        contract_side_effect_record_rejects_duplicate_id(backend);
        contract_side_effect_record_rejects_workflow_identity_mismatch(backend);
        contract_side_effect_record_rejects_workflow_version_mismatch(backend);
        contract_side_effect_record_rejects_schema_version_mismatch(backend);
        contract_side_effect_record_rejects_spec_hash_mismatch(backend);
        contract_side_effect_record_update_accepts_allowed_lifecycle_transition(backend);
        contract_side_effect_record_update_rejects_invalid_lifecycle_transition(backend);
        contract_side_effect_record_update_rejects_same_state_replace(backend);
    }

    fn run_approval_presentation_record_store_contract(
        backend: &impl ApprovalPresentationRecordStore,
    ) {
        let fixture = Fixture::new();
        let first = fixture.approval_presentation_record("a", "approval/run-a/planning");
        let second = fixture.approval_presentation_record("b", "approval/run-a/planning");

        backend
            .write_approval_presentation_record(&second)
            .expect("second record written");
        backend
            .write_approval_presentation_record(&first)
            .expect("first record written");
        let read = backend
            .read_approval_presentation_record(first.presentation_id())
            .expect("record read")
            .expect("record exists");
        let listed = backend
            .list_approval_presentation_records(&fixture.run_id)
            .expect("records listed");
        let approval_listed = backend
            .list_approval_presentation_records_for_approval(
                &fixture.run_id,
                "approval/run-a/planning",
            )
            .expect("approval records listed");

        assert_eq!(read, first);
        assert_eq!(listed, vec![first.clone(), second.clone()]);
        assert_eq!(approval_listed, vec![first, second]);
    }

    fn local_backend() -> LocalStateBackend {
        let id = NEXT_TEST_BACKEND.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "workflow-os-state-backend-{}-{id}",
            std::process::id()
        ));
        if root.exists() {
            fs::remove_dir_all(&root).expect("stale backend cleanup");
        }
        LocalStateBackend::new(root).expect("local backend")
    }

    #[test]
    fn backend_contract_passes_for_in_memory_backend() {
        let backend = InMemoryStateBackend::default();
        run_backend_contract(&backend);
        run_side_effect_record_store_contract(&backend);
        run_approval_presentation_record_store_contract(&backend);
    }

    #[test]
    fn backend_contract_passes_for_local_backend() {
        let backend = local_backend();
        let root = backend.root().to_path_buf();
        run_backend_contract(&backend);
        run_side_effect_record_store_contract(&backend);
        run_approval_presentation_record_store_contract(&backend);
        fs::remove_dir_all(root).expect("cleanup local backend");
    }

    #[test]
    fn corrupted_local_state_returns_clear_error() {
        let backend = local_backend();
        let fixture = Fixture::new();
        let run_dir = backend.run_events_dir(&fixture.run_id);
        fs::create_dir_all(&run_dir).expect("run dir");
        fs::write(run_dir.join("00000000000000000001.json"), "{not json")
            .expect("corrupt event file");

        let error = backend
            .read_events(&fixture.run_id)
            .expect_err("corruption is reported");

        assert_eq!(error.code(), "state.corrupt");
        fs::remove_dir_all(backend.root()).expect("cleanup local backend");
    }

    #[test]
    fn legacy_event_without_schema_version_returns_clear_error() {
        let backend = local_backend();
        let fixture = Fixture::new();
        let run_dir = backend.run_events_dir(&fixture.run_id);
        fs::create_dir_all(&run_dir).expect("run dir");
        fs::write(
            run_dir.join("00000000000000000001.json"),
            format!(
                r#"{{
  "sequence_number": 1,
  "event_id": "event-legacy-state",
  "timestamp": "2026-01-01T00:00:00Z",
  "run_id": "{}",
  "workflow_id": "{}",
  "workflow_version": "{}",
  "spec_content_hash": "{}",
  "correlation_id": "correlation-state",
  "actor": "system",
  "idempotency_key": null,
  "kind": {{
    "kind": "RunCreated",
    "summary": null
  }}
}}"#,
                fixture.run_id,
                fixture.workflow_id,
                fixture.workflow_version,
                fixture.spec_hash.as_str()
            ),
        )
        .expect("legacy event file");

        let error = backend
            .read_events(&fixture.run_id)
            .expect_err("legacy event is reported");

        assert_eq!(error.code(), "state.corrupt");
        assert!(error.message().contains("schema_version"));
        fs::remove_dir_all(backend.root()).expect("cleanup local backend");
    }

    #[test]
    fn health_check_reports_missing_event_id_index_entry() {
        let backend = local_backend();
        let fixture = Fixture::new();
        let event = fixture.created();
        backend.append_event(&event).expect("append event");
        fs::remove_file(backend.event_id_path(&event.event_id)).expect("remove event id index");

        let health = backend.health_check().expect("health check reports");

        assert!(!health.healthy);
        assert!(health.message.contains("missing event ID index"));
        fs::remove_dir_all(backend.root()).expect("cleanup local backend");
    }

    #[test]
    fn event_file_without_index_fails_deterministically() {
        let backend = local_backend();
        let fixture = Fixture::new();
        let event = fixture.created();
        let run_dir = backend.run_events_dir(&fixture.run_id);
        fs::create_dir_all(&run_dir).expect("run dir");
        write_json_create_new_atomic(&backend.event_path(&event), &event).expect("write event");

        let read_error = backend
            .read_events(&fixture.run_id)
            .expect_err("missing index is rejected");
        let append_error = backend
            .append_event(&fixture.validated())
            .expect_err("append refuses inconsistent event log");

        assert_eq!(read_error.code(), "state.event_index.missing");
        assert_eq!(append_error.code(), "state.event_index.missing");
        fs::remove_dir_all(backend.root()).expect("cleanup local backend");
    }

    #[test]
    fn local_side_effect_record_corrupt_read_fails_without_leaking_payload() {
        let backend = local_backend();
        let fixture = Fixture::new();
        let record = fixture.side_effect_record("corrupt", fixture.workflow_id.clone());
        backend
            .write_side_effect_record(&record)
            .expect("record written");
        let path = backend.side_effect_record_path(record.run_id(), record.side_effect_id());
        fs::write(&path, r#"{"target":"sk-side-effect-secret"}"#).expect("corrupt record");

        let error = backend
            .read_side_effect_record(record.side_effect_id())
            .expect_err("corrupt record rejected");

        assert_eq!(error.code(), "side_effect_record.read.corrupt");
        assert!(!error.to_string().contains("sk-side-effect-secret"));
        assert!(!error.to_string().contains(record.side_effect_id().as_str()));
        assert!(!error.to_string().contains(fixture.run_id.as_str()));
        fs::remove_dir_all(backend.root()).expect("cleanup local backend");
    }

    #[test]
    fn local_side_effect_record_write_does_not_mutate_runtime_state() {
        let backend = local_backend();
        let fixture = Fixture::new();
        backend
            .append_event(&fixture.created())
            .expect("append created");
        let events_before = backend.read_events(&fixture.run_id).expect("events before");
        let snapshot_before = backend
            .load_snapshot(&fixture.run_id)
            .expect("snapshot before");
        let record = fixture.side_effect_record("no-runtime-mutation", fixture.workflow_id.clone());

        backend
            .write_side_effect_record(&record)
            .expect("record written");

        let events_after = backend.read_events(&fixture.run_id).expect("events after");
        let snapshot_after = backend
            .load_snapshot(&fixture.run_id)
            .expect("snapshot after");

        assert_eq!(events_before, events_after);
        assert_eq!(snapshot_before, snapshot_after);
        fs::remove_dir_all(backend.root()).expect("cleanup local backend");
    }

    #[test]
    fn local_side_effect_record_list_rejects_full_identity_mismatch_without_leaking() {
        let backend = local_backend();
        let fixture = Fixture::new();
        let record = fixture.side_effect_record("identity-list", fixture.workflow_id.clone());
        let mismatched = fixture.side_effect_record_with_identity(
            "identity-list-mismatch",
            fixture.workflow_id.clone(),
            WorkflowVersion::new("v99").expect("workflow version"),
            fixture.schema_version.clone(),
            fixture.spec_hash.clone(),
        );
        backend
            .write_side_effect_record(&record)
            .expect("record written");
        write_json_create_new_atomic(
            &backend.side_effect_record_path(mismatched.run_id(), mismatched.side_effect_id()),
            &mismatched,
        )
        .expect("bypass store write for corrupt bucket fixture");

        let error = backend
            .list_side_effect_records_for_workflow_run(&fixture.workflow_id, &fixture.run_id)
            .expect_err("identity mismatch rejected");

        assert_eq!(error.code(), "side_effect_record.read.identity_mismatch");
        assert!(!error.to_string().contains("v99"));
        assert!(!error.to_string().contains(fixture.run_id.as_str()));
        fs::remove_dir_all(backend.root()).expect("cleanup local backend");
    }

    #[test]
    fn health_check_reports_index_entry_without_event_file() {
        let backend = local_backend();
        let fixture = Fixture::new();
        let event = fixture.created();
        backend.append_event(&event).expect("append event");
        fs::remove_file(backend.event_path(&event)).expect("remove event file");

        let health = backend.health_check().expect("health check reports");
        let append_error = backend
            .append_event(&fixture.validated())
            .expect_err("append refuses dangling index");

        assert!(!health.healthy);
        assert!(health.message.contains("points to missing event file"));
        assert_eq!(append_error.code(), "state.event_index.dangling");
        fs::remove_dir_all(backend.root()).expect("cleanup local backend");
    }

    #[test]
    fn rehydration_fails_clearly_on_corrupt_local_event_stream() {
        let backend = local_backend();
        let fixture = Fixture::new();
        let run_dir = backend.run_events_dir(&fixture.run_id);
        fs::create_dir_all(&run_dir).expect("run dir");
        fs::write(run_dir.join("00000000000000000001.json"), "{not json")
            .expect("corrupt event file");

        let error = backend
            .rehydrate_run(&fixture.run_id)
            .expect_err("corrupt stream fails rehydration");

        assert_eq!(error.code(), "state.corrupt");
        fs::remove_dir_all(backend.root()).expect("cleanup local backend");
    }
}
