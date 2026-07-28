use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use rusqlite::{
    params, Connection, Error as SqliteError, ErrorCode, OptionalExtension, TransactionBehavior,
};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::state::{
    is_allowed_side_effect_lifecycle_update, same_approval_presentation_run_identity,
    same_side_effect_run_identity, state_error, validate_append_against_history,
};
use crate::{
    validate_approval_presentation_approval_id, ActorId, AdapterRuntimeAuditRecord,
    AdapterRuntimeObservabilityRecord, AdapterTelemetryStore, ApprovalPresentationId,
    ApprovalPresentationRecord, ApprovalPresentationRecordStore, ApprovalRequest, ApprovalStore,
    BackendHealthCheck, DurableLeaseSemantics, DurableStateBackendKind, DurableStateCapability,
    DurableStateContractProvider, DurableStateContractVersion, DurableStateSchemaMetadata,
    DurableStateSchemaPosture, DurableStateSemanticContract, DurableStateSupport,
    DurableStateTransactionKind, DurableStateTransactionSupport, EventLogStore, IdempotencyKey,
    IdempotencyResult, IdempotencyStore, IdempotencyWrite, LockLease, LockStore, PolicyAuditRecord,
    PolicyAuditStore, ProjectId, ProjectStateRecord, ProjectStateStore, RunSnapshotStore,
    SideEffectId, SideEffectRecord, SideEffectRecordStore, StateBackend, WorkReportArtifactRecord,
    WorkReportArtifactStore, WorkReportId, WorkflowId, WorkflowOsError, WorkflowRunEvent,
    WorkflowRunId, WorkflowRunSnapshot,
};

const ADAPTER_SCHEMA_VERSION: u32 = 1;
const SCHEMA_CHECKSUM: &str = "workflow-os-sqlite-state-v1";
const DEFAULT_BUSY_TIMEOUT: Duration = Duration::from_secs(2);

const SCHEMA: &str = r"
CREATE TABLE schema_metadata (
    singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
    schema_version INTEGER NOT NULL,
    migration_state TEXT NOT NULL,
    checksum TEXT NOT NULL
);
CREATE TABLE events (
    event_id TEXT NOT NULL UNIQUE,
    run_id TEXT NOT NULL,
    sequence_number INTEGER NOT NULL CHECK (sequence_number > 0),
    payload TEXT NOT NULL,
    PRIMARY KEY (run_id, sequence_number)
);
CREATE TABLE snapshots (
    run_id TEXT PRIMARY KEY,
    payload TEXT NOT NULL
);
CREATE TABLE idempotency_results (
    idempotency_key TEXT PRIMARY KEY,
    payload TEXT NOT NULL
);
CREATE TABLE locks (
    lock_key TEXT PRIMARY KEY,
    owner TEXT NOT NULL
);
CREATE TABLE approvals (
    approval_id TEXT PRIMARY KEY,
    payload TEXT NOT NULL
);
CREATE TABLE approval_presentations (
    presentation_id TEXT PRIMARY KEY,
    run_id TEXT NOT NULL,
    approval_id TEXT NOT NULL,
    payload TEXT NOT NULL
);
CREATE INDEX approval_presentations_run
    ON approval_presentations (run_id, presentation_id);
CREATE INDEX approval_presentations_approval
    ON approval_presentations (run_id, approval_id, presentation_id);
CREATE TABLE projects (
    project_id TEXT PRIMARY KEY,
    payload TEXT NOT NULL
);
CREATE TABLE policy_audit (
    audit_id TEXT PRIMARY KEY,
    sort_timestamp TEXT NOT NULL,
    payload TEXT NOT NULL
);
CREATE TABLE adapter_audit (
    telemetry_id TEXT PRIMARY KEY,
    run_id TEXT NOT NULL,
    sort_timestamp TEXT NOT NULL,
    payload TEXT NOT NULL
);
CREATE INDEX adapter_audit_run
    ON adapter_audit (run_id, sort_timestamp, telemetry_id);
CREATE TABLE adapter_observability (
    telemetry_id TEXT PRIMARY KEY,
    run_id TEXT NOT NULL,
    sort_timestamp TEXT NOT NULL,
    payload TEXT NOT NULL
);
CREATE INDEX adapter_observability_run
    ON adapter_observability (run_id, sort_timestamp, telemetry_id);
CREATE TABLE work_report_artifacts (
    run_id TEXT NOT NULL,
    report_id TEXT NOT NULL,
    payload TEXT NOT NULL,
    PRIMARY KEY (run_id, report_id)
);
CREATE TABLE side_effect_records (
    side_effect_id TEXT PRIMARY KEY,
    run_id TEXT NOT NULL,
    workflow_id TEXT NOT NULL,
    payload TEXT NOT NULL
);
CREATE INDEX side_effect_records_run
    ON side_effect_records (run_id, side_effect_id);
";

/// Opt-in embedded `SQLite` durable-state backend.
///
/// The adapter opens a fresh connection for each operation, uses WAL and full
/// synchronous durability, and stores canonical validated JSON envelopes.
/// It is local-only and is not selected automatically by the runtime or CLI.
#[derive(Clone, Eq, PartialEq)]
pub struct SqliteStateBackend {
    database_path: PathBuf,
    busy_timeout: Duration,
}

impl fmt::Debug for SqliteStateBackend {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SqliteStateBackend")
            .field("backend", &"embedded_sqlite")
            .field("adapter_schema_version", &ADAPTER_SCHEMA_VERSION)
            .finish_non_exhaustive()
    }
}

impl SqliteStateBackend {
    /// Opens or creates an opt-in local `SQLite` state database.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when the parent directory, database,
    /// durability configuration, or managed schema cannot be prepared.
    pub fn open(database_path: impl Into<PathBuf>) -> Result<Self, WorkflowOsError> {
        Self::open_with_busy_timeout(database_path, DEFAULT_BUSY_TIMEOUT)
    }

    /// Opens an adapter with an explicit bounded busy timeout.
    ///
    /// This constructor exists for deterministic contention tests and local
    /// operator tuning. It does not create a shared-worker lease guarantee.
    ///
    /// # Errors
    ///
    /// Returns a stable error when the timeout is zero or the backend cannot be
    /// prepared.
    pub fn open_with_busy_timeout(
        database_path: impl Into<PathBuf>,
        busy_timeout: Duration,
    ) -> Result<Self, WorkflowOsError> {
        if busy_timeout.is_zero() {
            return Err(sqlite_state_error(
                "configuration.invalid",
                "SQLite state configuration is invalid",
            ));
        }
        let database_path = database_path.into();
        if let Some(parent) = database_path.parent() {
            fs::create_dir_all(parent).map_err(|_| {
                sqlite_state_error("open.failed", "SQLite state database could not be prepared")
            })?;
        }
        let backend = Self {
            database_path,
            busy_timeout,
        };
        let mut connection = backend.connection()?;
        Self::prepare_schema(&mut connection)?;
        Ok(backend)
    }

    fn connection(&self) -> Result<Connection, WorkflowOsError> {
        let connection = Connection::open(&self.database_path).map_err(|error| {
            map_sqlite_error(
                error,
                "open.failed",
                "SQLite state database could not be opened",
            )
        })?;
        connection
            .busy_timeout(self.busy_timeout)
            .map_err(|error| {
                map_sqlite_error(
                    error,
                    "configuration.failed",
                    "SQLite state durability configuration failed",
                )
            })?;
        connection
            .pragma_update(None, "foreign_keys", true)
            .and_then(|()| connection.pragma_update(None, "journal_mode", "WAL"))
            .and_then(|()| connection.pragma_update(None, "synchronous", "FULL"))
            .map_err(|error| {
                map_sqlite_error(
                    error,
                    "configuration.failed",
                    "SQLite state durability configuration failed",
                )
            })?;
        Ok(connection)
    }

    fn prepare_schema(connection: &mut Connection) -> Result<(), WorkflowOsError> {
        let version: u32 = connection
            .pragma_query_value(None, "user_version", |row| row.get(0))
            .map_err(|error| {
                map_sqlite_error(
                    error,
                    "schema.read_failed",
                    "SQLite state schema metadata could not be read",
                )
            })?;
        match version {
            0 => {
                let transaction = connection
                    .transaction_with_behavior(TransactionBehavior::Immediate)
                    .map_err(|error| {
                        map_sqlite_error(
                            error,
                            "schema.initialize_failed",
                            "SQLite state schema could not be initialized",
                        )
                    })?;
                transaction.execute_batch(SCHEMA).map_err(|error| {
                    map_sqlite_error(
                        error,
                        "schema.initialize_failed",
                        "SQLite state schema could not be initialized",
                    )
                })?;
                transaction
                    .execute(
                        "INSERT INTO schema_metadata
                         (singleton, schema_version, migration_state, checksum)
                         VALUES (1, ?1, 'ready', ?2)",
                        params![ADAPTER_SCHEMA_VERSION, SCHEMA_CHECKSUM],
                    )
                    .map_err(|error| {
                        map_sqlite_error(
                            error,
                            "schema.initialize_failed",
                            "SQLite state schema could not be initialized",
                        )
                    })?;
                transaction
                    .pragma_update(None, "user_version", ADAPTER_SCHEMA_VERSION)
                    .map_err(|error| {
                        map_sqlite_error(
                            error,
                            "schema.initialize_failed",
                            "SQLite state schema could not be initialized",
                        )
                    })?;
                transaction.commit().map_err(|error| {
                    map_sqlite_error(
                        error,
                        "schema.initialize_failed",
                        "SQLite state schema could not be initialized",
                    )
                })?;
                Ok(())
            }
            ADAPTER_SCHEMA_VERSION => validate_schema_metadata(connection),
            _ => Err(sqlite_state_error(
                "schema.incompatible",
                "SQLite state schema version is not supported",
            )),
        }
    }

    fn read_events_with_connection(
        connection: &Connection,
        run_id: &WorkflowRunId,
    ) -> Result<Vec<WorkflowRunEvent>, WorkflowOsError> {
        let mut statement = connection
            .prepare(
                "SELECT event_id, run_id, sequence_number, payload FROM events
                 WHERE run_id = ?1 ORDER BY sequence_number ASC",
            )
            .map_err(|error| {
                map_sqlite_error(
                    error,
                    "read.failed",
                    "SQLite state records could not be read",
                )
            })?;
        let rows = statement
            .query_map(params![run_id.as_str()], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, i64>(2)?,
                    row.get::<_, String>(3)?,
                ))
            })
            .map_err(|error| {
                map_sqlite_error(
                    error,
                    "read.failed",
                    "SQLite state records could not be read",
                )
            })?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| {
                map_sqlite_error(
                    error,
                    "read.failed",
                    "SQLite state records could not be read",
                )
            })?;
        rows.into_iter()
            .map(|(event_id, relational_run_id, sequence_number, payload)| {
                let event: WorkflowRunEvent = decode_json(&payload, "event")?;
                if event.event_id.as_str() != event_id
                    || event.run_id.as_str() != relational_run_id
                    || event.run_id != *run_id
                    || i64::try_from(event.sequence_number.get()).ok() != Some(sequence_number)
                {
                    return Err(sqlite_state_error(
                        "record.identity_mismatch",
                        "SQLite state record identity does not match its relational index",
                    ));
                }
                Ok(event)
            })
            .collect()
    }

    fn validate_all_payloads(connection: &Connection) -> Result<(), WorkflowOsError> {
        validate_table_payloads::<WorkflowRunEvent>(connection, "events")?;
        validate_table_payloads::<WorkflowRunSnapshot>(connection, "snapshots")?;
        validate_table_payloads::<IdempotencyResult>(connection, "idempotency_results")?;
        validate_table_payloads::<ApprovalRequest>(connection, "approvals")?;
        validate_table_payloads::<ApprovalPresentationRecord>(
            connection,
            "approval_presentations",
        )?;
        validate_table_payloads::<ProjectStateRecord>(connection, "projects")?;
        validate_table_payloads::<PolicyAuditRecord>(connection, "policy_audit")?;
        validate_table_payloads::<AdapterRuntimeAuditRecord>(connection, "adapter_audit")?;
        validate_table_payloads::<AdapterRuntimeObservabilityRecord>(
            connection,
            "adapter_observability",
        )?;
        validate_table_payloads::<WorkReportArtifactRecord>(connection, "work_report_artifacts")?;
        validate_table_payloads::<SideEffectRecord>(connection, "side_effect_records")
    }

    fn validate_relational_identities(connection: &Connection) -> Result<(), WorkflowOsError> {
        validate_event_identities(connection)?;
        validate_single_identity_table::<WorkflowRunSnapshot>(
            connection,
            "snapshots",
            "run_id",
            "snapshot",
            |snapshot, run_id| snapshot.identity.run_id.as_str() == run_id,
        )?;
        validate_single_identity_table::<ApprovalRequest>(
            connection,
            "approvals",
            "approval_id",
            "approval request",
            |request, approval_id| request.approval_id == approval_id,
        )?;
        validate_triple_identity_table::<ApprovalPresentationRecord>(
            connection,
            "approval_presentations",
            ["presentation_id", "run_id", "approval_id"],
            "approval presentation",
            |record, presentation_id, run_id, approval_id| {
                record.presentation_id().as_str() == presentation_id
                    && record.run_id().as_str() == run_id
                    && record.approval_id() == approval_id
            },
        )?;
        validate_single_identity_table::<ProjectStateRecord>(
            connection,
            "projects",
            "project_id",
            "project state",
            |state, project_id| state.project_id.as_str() == project_id,
        )?;
        validate_single_identity_table::<PolicyAuditRecord>(
            connection,
            "policy_audit",
            "audit_id",
            "policy audit",
            |record, audit_id| record.audit_id.as_str() == audit_id,
        )?;
        validate_double_identity_table::<AdapterRuntimeAuditRecord>(
            connection,
            "adapter_audit",
            ["telemetry_id", "run_id"],
            "adapter audit",
            |record, telemetry_id, run_id| {
                record.telemetry_id.as_str() == telemetry_id
                    && record.workflow_run_id.as_ref().map(WorkflowRunId::as_str) == Some(run_id)
            },
        )?;
        validate_double_identity_table::<AdapterRuntimeObservabilityRecord>(
            connection,
            "adapter_observability",
            ["telemetry_id", "run_id"],
            "adapter observability",
            |record, telemetry_id, run_id| {
                record.telemetry_id.as_str() == telemetry_id
                    && record.workflow_run_id.as_ref().map(WorkflowRunId::as_str) == Some(run_id)
            },
        )?;
        validate_double_identity_table::<WorkReportArtifactRecord>(
            connection,
            "work_report_artifacts",
            ["run_id", "report_id"],
            "work report artifact",
            |record, run_id, report_id| {
                record.run_id().as_str() == run_id && record.report_id().as_str() == report_id
            },
        )?;
        validate_triple_identity_table::<SideEffectRecord>(
            connection,
            "side_effect_records",
            ["side_effect_id", "run_id", "workflow_id"],
            "side-effect record",
            |record, side_effect_id, run_id, workflow_id| {
                record.side_effect_id().as_str() == side_effect_id
                    && record.run_id().as_str() == run_id
                    && record.workflow_id().as_str() == workflow_id
            },
        )
    }
}

impl EventLogStore for SqliteStateBackend {
    fn append_event(&self, event: &WorkflowRunEvent) -> Result<(), WorkflowOsError> {
        let sequence_number = i64::try_from(event.sequence_number.get()).map_err(|_| {
            sqlite_state_error(
                "event.sequence_invalid",
                "SQLite event sequence is outside the supported range",
            )
        })?;
        let mut connection = self.connection()?;
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|error| {
                map_sqlite_error(
                    error,
                    "write.failed",
                    "SQLite event transaction could not start",
                )
            })?;
        if row_exists(
            &transaction,
            "SELECT 1 FROM events WHERE event_id = ?1",
            event.event_id.as_str(),
        )? {
            return Err(state_error(
                "state.event.duplicate_id",
                "duplicate event ID",
            ));
        }
        if transaction
            .query_row(
                "SELECT 1 FROM events WHERE run_id = ?1 AND sequence_number = ?2",
                params![event.run_id.as_str(), sequence_number],
                |_| Ok(()),
            )
            .optional()
            .map_err(|error| {
                map_sqlite_error(
                    error,
                    "read.failed",
                    "SQLite event history could not be read",
                )
            })?
            .is_some()
        {
            return Err(state_error(
                "state.event.duplicate_sequence",
                "duplicate event sequence",
            ));
        }
        let history = Self::read_events_with_connection(&transaction, &event.run_id)?;
        validate_append_against_history(&history, event)?;
        let payload = encode_json(event, "event")?;
        transaction
            .execute(
                "INSERT INTO events
                 (event_id, run_id, sequence_number, payload)
                 VALUES (?1, ?2, ?3, ?4)",
                params![
                    event.event_id.as_str(),
                    event.run_id.as_str(),
                    sequence_number,
                    payload
                ],
            )
            .map_err(|error| {
                map_sqlite_error(error, "write.failed", "SQLite event could not be written")
            })?;
        transaction.commit().map_err(|error| {
            map_sqlite_error(
                error,
                "write.failed",
                "SQLite event transaction could not commit",
            )
        })
    }

    fn read_events(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<Vec<WorkflowRunEvent>, WorkflowOsError> {
        Self::read_events_with_connection(&self.connection()?, run_id)
    }
}

impl RunSnapshotStore for SqliteStateBackend {
    fn save_snapshot(&self, snapshot: &WorkflowRunSnapshot) -> Result<(), WorkflowOsError> {
        upsert_payload(
            &self.connection()?,
            "snapshots",
            "run_id",
            snapshot.identity.run_id.as_str(),
            &encode_json(snapshot, "snapshot")?,
        )
    }

    fn load_snapshot(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<Option<WorkflowRunSnapshot>, WorkflowOsError> {
        let snapshot: Option<WorkflowRunSnapshot> = read_optional_payload(
            &self.connection()?,
            "snapshots",
            "run_id",
            run_id.as_str(),
            "snapshot",
        )?;
        if snapshot
            .as_ref()
            .is_some_and(|snapshot| snapshot.identity.run_id != *run_id)
        {
            return Err(sqlite_state_error(
                "record.identity_mismatch",
                "SQLite state record identity does not match its relational index",
            ));
        }
        Ok(snapshot)
    }
}

impl IdempotencyStore for SqliteStateBackend {
    fn record_idempotency_result(
        &self,
        key: &IdempotencyKey,
        result: IdempotencyResult,
    ) -> Result<IdempotencyWrite, WorkflowOsError> {
        let mut connection = self.connection()?;
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|error| {
                map_sqlite_error(
                    error,
                    "write.failed",
                    "SQLite idempotency transaction could not start",
                )
            })?;
        let existing = transaction
            .query_row(
                "SELECT payload FROM idempotency_results WHERE idempotency_key = ?1",
                params![key.as_str()],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .map_err(|error| {
                map_sqlite_error(
                    error,
                    "read.failed",
                    "SQLite idempotency record could not be read",
                )
            })?;
        if let Some(payload) = existing {
            return Ok(IdempotencyWrite::Duplicate(decode_json(
                &payload,
                "idempotency result",
            )?));
        }
        transaction
            .execute(
                "INSERT INTO idempotency_results (idempotency_key, payload)
                 VALUES (?1, ?2)",
                params![key.as_str(), encode_json(&result, "idempotency result")?],
            )
            .map_err(|error| {
                map_sqlite_error(
                    error,
                    "write.failed",
                    "SQLite idempotency record could not be written",
                )
            })?;
        transaction.commit().map_err(|error| {
            map_sqlite_error(
                error,
                "write.failed",
                "SQLite idempotency transaction could not commit",
            )
        })?;
        Ok(IdempotencyWrite::FirstWrite(result))
    }
}

impl LockStore for SqliteStateBackend {
    fn acquire_lock(&self, key: &str, owner: &ActorId) -> Result<LockLease, WorkflowOsError> {
        let connection = self.connection()?;
        match connection.execute(
            "INSERT INTO locks (lock_key, owner) VALUES (?1, ?2)",
            params![key, owner.as_str()],
        ) {
            Ok(_) => Ok(LockLease {
                key: key.to_owned(),
                owner: owner.clone(),
            }),
            Err(error) if is_constraint_error(&error) => Err(state_error(
                "state.lock_contended",
                "SQLite state lock is already held",
            )),
            Err(error) => Err(map_sqlite_error(
                error,
                "write.failed",
                "SQLite state lock could not be acquired",
            )),
        }
    }

    fn release_lock(&self, lease: &LockLease) -> Result<(), WorkflowOsError> {
        let connection = self.connection()?;
        let removed = connection
            .execute(
                "DELETE FROM locks WHERE lock_key = ?1 AND owner = ?2",
                params![lease.key, lease.owner.as_str()],
            )
            .map_err(|error| {
                map_sqlite_error(
                    error,
                    "write.failed",
                    "SQLite state lock could not be released",
                )
            })?;
        if removed == 0
            && row_exists(
                &connection,
                "SELECT 1 FROM locks WHERE lock_key = ?1",
                &lease.key,
            )?
        {
            return Err(state_error(
                "state.lock.owner_mismatch",
                "SQLite state lock is owned by a different actor",
            ));
        }
        Ok(())
    }
}

impl ApprovalStore for SqliteStateBackend {
    fn save_approval_request(&self, request: &ApprovalRequest) -> Result<(), WorkflowOsError> {
        upsert_payload(
            &self.connection()?,
            "approvals",
            "approval_id",
            &request.approval_id,
            &encode_json(request, "approval request")?,
        )
    }

    fn load_approval_request(
        &self,
        approval_id: &str,
    ) -> Result<Option<ApprovalRequest>, WorkflowOsError> {
        let request: Option<ApprovalRequest> = read_optional_payload(
            &self.connection()?,
            "approvals",
            "approval_id",
            approval_id,
            "approval request",
        )?;
        if request
            .as_ref()
            .is_some_and(|request| request.approval_id != approval_id)
        {
            return Err(sqlite_state_error(
                "record.identity_mismatch",
                "SQLite state record identity does not match its relational index",
            ));
        }
        Ok(request)
    }

    fn delete_approval_request(&self, approval_id: &str) -> Result<(), WorkflowOsError> {
        self.connection()?
            .execute(
                "DELETE FROM approvals WHERE approval_id = ?1",
                params![approval_id],
            )
            .map(|_| ())
            .map_err(|error| {
                map_sqlite_error(
                    error,
                    "write.failed",
                    "SQLite approval projection could not be deleted",
                )
            })
    }
}

impl ApprovalPresentationRecordStore for SqliteStateBackend {
    fn write_approval_presentation_record(
        &self,
        record: &ApprovalPresentationRecord,
    ) -> Result<(), WorkflowOsError> {
        let mut connection = self.connection()?;
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|error| {
                map_sqlite_error(
                    error,
                    "write.failed",
                    "SQLite presentation transaction could not start",
                )
            })?;
        let existing = read_payloads::<ApprovalPresentationRecord>(
            &transaction,
            "SELECT payload FROM approval_presentations
             WHERE run_id = ?1 ORDER BY presentation_id",
            record.run_id().as_str(),
            "approval presentation",
        )?;
        if existing
            .iter()
            .any(|item| !same_approval_presentation_run_identity(item, record))
        {
            return Err(state_error(
                "approval_presentation_record.write.identity_mismatch",
                "approval-presentation record workflow/run identity conflicts with existing records",
            ));
        }
        let result = transaction.execute(
            "INSERT INTO approval_presentations
             (presentation_id, run_id, approval_id, payload)
             VALUES (?1, ?2, ?3, ?4)",
            params![
                record.presentation_id().as_str(),
                record.run_id().as_str(),
                record.approval_id(),
                encode_json(record, "approval presentation")?
            ],
        );
        match result {
            Ok(_) => {}
            Err(error) if is_constraint_error(&error) => {
                return Err(state_error(
                    "approval_presentation_record.write.duplicate",
                    "approval-presentation record already exists",
                ));
            }
            Err(error) => {
                return Err(map_sqlite_error(
                    error,
                    "write.failed",
                    "SQLite approval-presentation record could not be written",
                ));
            }
        }
        transaction.commit().map_err(|error| {
            map_sqlite_error(
                error,
                "write.failed",
                "SQLite presentation transaction could not commit",
            )
        })
    }

    fn read_approval_presentation_record(
        &self,
        presentation_id: &ApprovalPresentationId,
    ) -> Result<Option<ApprovalPresentationRecord>, WorkflowOsError> {
        let record: Option<ApprovalPresentationRecord> = read_optional_payload(
            &self.connection()?,
            "approval_presentations",
            "presentation_id",
            presentation_id.as_str(),
            "approval presentation",
        )?;
        if record
            .as_ref()
            .is_some_and(|record| record.presentation_id() != presentation_id)
        {
            return Err(sqlite_state_error(
                "record.identity_mismatch",
                "SQLite state record identity does not match its relational index",
            ));
        }
        Ok(record)
    }

    fn list_approval_presentation_records(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<Vec<ApprovalPresentationRecord>, WorkflowOsError> {
        let records: Vec<ApprovalPresentationRecord> = read_payloads(
            &self.connection()?,
            "SELECT payload FROM approval_presentations
             WHERE run_id = ?1 ORDER BY presentation_id",
            run_id.as_str(),
            "approval presentation",
        )?;
        if records.iter().any(|record| record.run_id() != run_id) {
            return Err(sqlite_state_error(
                "record.identity_mismatch",
                "SQLite state record identity does not match its relational index",
            ));
        }
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

impl ProjectStateStore for SqliteStateBackend {
    fn save_project_state(&self, state: &ProjectStateRecord) -> Result<(), WorkflowOsError> {
        upsert_payload(
            &self.connection()?,
            "projects",
            "project_id",
            state.project_id.as_str(),
            &encode_json(state, "project state")?,
        )
    }

    fn load_project_state(
        &self,
        project_id: &ProjectId,
    ) -> Result<Option<ProjectStateRecord>, WorkflowOsError> {
        let state: Option<ProjectStateRecord> = read_optional_payload(
            &self.connection()?,
            "projects",
            "project_id",
            project_id.as_str(),
            "project state",
        )?;
        if state
            .as_ref()
            .is_some_and(|state| state.project_id != *project_id)
        {
            return Err(sqlite_state_error(
                "record.identity_mismatch",
                "SQLite state record identity does not match its relational index",
            ));
        }
        Ok(state)
    }
}

impl PolicyAuditStore for SqliteStateBackend {
    fn append_policy_audit_record(
        &self,
        record: &PolicyAuditRecord,
    ) -> Result<(), WorkflowOsError> {
        let timestamp = record.timestamp.to_string();
        insert_ordered_payload(
            &self.connection()?,
            OrderedPayloadInsert {
                table: "policy_audit",
                id_column: "audit_id",
                id: record.audit_id.as_str(),
                run_id: "",
                sort_timestamp: &timestamp,
                payload: &encode_json(record, "policy audit")?,
                kind: "policy audit",
            },
        )
    }

    fn read_policy_audit_records(&self) -> Result<Vec<PolicyAuditRecord>, WorkflowOsError> {
        read_all_payloads(
            &self.connection()?,
            "SELECT payload FROM policy_audit ORDER BY sort_timestamp, audit_id",
            "policy audit",
        )
    }
}

impl AdapterTelemetryStore for SqliteStateBackend {
    fn append_adapter_audit_record(
        &self,
        record: &AdapterRuntimeAuditRecord,
    ) -> Result<(), WorkflowOsError> {
        let run_id = record.workflow_run_id.as_ref().ok_or_else(|| {
            state_error(
                "state.adapter_audit.run_id_required",
                "adapter audit telemetry requires workflow run ID for SQLite persistence",
            )
        })?;
        let timestamp = record.timestamp.to_string();
        insert_ordered_payload(
            &self.connection()?,
            OrderedPayloadInsert {
                table: "adapter_audit",
                id_column: "telemetry_id",
                id: record.telemetry_id.as_str(),
                run_id: run_id.as_str(),
                sort_timestamp: &timestamp,
                payload: &encode_json(record, "adapter audit")?,
                kind: "adapter audit",
            },
        )
    }

    fn read_adapter_audit_records(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<Vec<AdapterRuntimeAuditRecord>, WorkflowOsError> {
        read_payloads(
            &self.connection()?,
            "SELECT payload FROM adapter_audit
             WHERE run_id = ?1 ORDER BY sort_timestamp, telemetry_id",
            run_id.as_str(),
            "adapter audit",
        )
    }

    fn append_adapter_observability_record(
        &self,
        record: &AdapterRuntimeObservabilityRecord,
    ) -> Result<(), WorkflowOsError> {
        let run_id = record.workflow_run_id.as_ref().ok_or_else(|| {
            state_error(
                "state.adapter_observability.run_id_required",
                "adapter observability telemetry requires workflow run ID for SQLite persistence",
            )
        })?;
        let timestamp = record.timestamp.to_string();
        insert_ordered_payload(
            &self.connection()?,
            OrderedPayloadInsert {
                table: "adapter_observability",
                id_column: "telemetry_id",
                id: record.telemetry_id.as_str(),
                run_id: run_id.as_str(),
                sort_timestamp: &timestamp,
                payload: &encode_json(record, "adapter observability")?,
                kind: "adapter observability",
            },
        )
    }

    fn read_adapter_observability_records(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<Vec<AdapterRuntimeObservabilityRecord>, WorkflowOsError> {
        read_payloads(
            &self.connection()?,
            "SELECT payload FROM adapter_observability
             WHERE run_id = ?1 ORDER BY sort_timestamp, telemetry_id",
            run_id.as_str(),
            "adapter observability",
        )
    }
}

impl WorkReportArtifactStore for SqliteStateBackend {
    fn write_work_report_artifact(
        &self,
        artifact: &WorkReportArtifactRecord,
    ) -> Result<(), WorkflowOsError> {
        artifact.validate()?;
        let result = self.connection()?.execute(
            "INSERT INTO work_report_artifacts (run_id, report_id, payload)
             VALUES (?1, ?2, ?3)",
            params![
                artifact.run_id().as_str(),
                artifact.report_id().as_str(),
                encode_json(artifact, "work report artifact")?
            ],
        );
        match result {
            Ok(_) => Ok(()),
            Err(error) if is_constraint_error(&error) => Err(state_error(
                "work_report_artifact.write.duplicate",
                "work report artifact already exists",
            )),
            Err(error) => Err(map_sqlite_error(
                error,
                "write.failed",
                "SQLite work report artifact could not be written",
            )),
        }
    }

    fn read_work_report_artifact(
        &self,
        run_id: &WorkflowRunId,
        report_id: &WorkReportId,
    ) -> Result<Option<WorkReportArtifactRecord>, WorkflowOsError> {
        let payload = self
            .connection()?
            .query_row(
                "SELECT payload FROM work_report_artifacts
                 WHERE run_id = ?1 AND report_id = ?2",
                params![run_id.as_str(), report_id.as_str()],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .map_err(|error| {
                map_sqlite_error(
                    error,
                    "read.failed",
                    "SQLite work report artifact could not be read",
                )
            })?;
        let artifact: Option<WorkReportArtifactRecord> = payload
            .map(|payload| decode_json(&payload, "work report artifact"))
            .transpose()?;
        if artifact.as_ref().is_some_and(|artifact| {
            artifact.run_id() != run_id || artifact.report_id() != report_id
        }) {
            return Err(sqlite_state_error(
                "record.identity_mismatch",
                "SQLite state record identity does not match its relational index",
            ));
        }
        Ok(artifact)
    }

    fn list_work_report_artifacts(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<Vec<WorkReportArtifactRecord>, WorkflowOsError> {
        let artifacts: Vec<WorkReportArtifactRecord> = read_payloads(
            &self.connection()?,
            "SELECT payload FROM work_report_artifacts
             WHERE run_id = ?1 ORDER BY report_id",
            run_id.as_str(),
            "work report artifact",
        )?;
        if artifacts.iter().any(|artifact| artifact.run_id() != run_id) {
            return Err(sqlite_state_error(
                "record.identity_mismatch",
                "SQLite state record identity does not match its relational index",
            ));
        }
        Ok(artifacts)
    }
}

impl SideEffectRecordStore for SqliteStateBackend {
    fn write_side_effect_record(&self, record: &SideEffectRecord) -> Result<(), WorkflowOsError> {
        record.validate()?;
        let mut connection = self.connection()?;
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|error| {
                map_sqlite_error(
                    error,
                    "write.failed",
                    "SQLite side-effect transaction could not start",
                )
            })?;
        let existing = read_payloads::<SideEffectRecord>(
            &transaction,
            "SELECT payload FROM side_effect_records
             WHERE run_id = ?1 ORDER BY side_effect_id",
            record.run_id().as_str(),
            "side-effect record",
        )?;
        if existing
            .iter()
            .any(|item| !same_side_effect_run_identity(item, record))
        {
            return Err(state_error(
                "side_effect_record.write.identity_mismatch",
                "side-effect record workflow/run identity conflicts with existing records",
            ));
        }
        let result = transaction.execute(
            "INSERT INTO side_effect_records
             (side_effect_id, run_id, workflow_id, payload)
             VALUES (?1, ?2, ?3, ?4)",
            params![
                record.side_effect_id().as_str(),
                record.run_id().as_str(),
                record.workflow_id().as_str(),
                encode_json(record, "side-effect record")?
            ],
        );
        match result {
            Ok(_) => {}
            Err(error) if is_constraint_error(&error) => {
                return Err(state_error(
                    "side_effect_record.write.duplicate",
                    "side-effect record already exists",
                ));
            }
            Err(error) => {
                return Err(map_sqlite_error(
                    error,
                    "write.failed",
                    "SQLite side-effect record could not be written",
                ));
            }
        }
        transaction.commit().map_err(|error| {
            map_sqlite_error(
                error,
                "write.failed",
                "SQLite side-effect transaction could not commit",
            )
        })
    }

    fn update_side_effect_record(&self, record: &SideEffectRecord) -> Result<(), WorkflowOsError> {
        record.validate()?;
        let mut connection = self.connection()?;
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|error| {
                map_sqlite_error(
                    error,
                    "write.failed",
                    "SQLite side-effect transaction could not start",
                )
            })?;
        let payload = transaction
            .query_row(
                "SELECT payload FROM side_effect_records WHERE side_effect_id = ?1",
                params![record.side_effect_id().as_str()],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .map_err(|error| {
                map_sqlite_error(
                    error,
                    "read.failed",
                    "SQLite side-effect record could not be read",
                )
            })?
            .ok_or_else(|| {
                state_error(
                    "side_effect_record.update.missing",
                    "side-effect record does not exist",
                )
            })?;
        let existing: SideEffectRecord = decode_json(&payload, "side-effect record")?;
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
        transaction
            .execute(
                "UPDATE side_effect_records SET payload = ?2 WHERE side_effect_id = ?1",
                params![
                    record.side_effect_id().as_str(),
                    encode_json(record, "side-effect record")?
                ],
            )
            .map_err(|error| {
                map_sqlite_error(
                    error,
                    "write.failed",
                    "SQLite side-effect record could not be updated",
                )
            })?;
        transaction.commit().map_err(|error| {
            map_sqlite_error(
                error,
                "write.failed",
                "SQLite side-effect transaction could not commit",
            )
        })
    }

    fn read_side_effect_record(
        &self,
        side_effect_id: &SideEffectId,
    ) -> Result<Option<SideEffectRecord>, WorkflowOsError> {
        let record: Option<SideEffectRecord> = read_optional_payload(
            &self.connection()?,
            "side_effect_records",
            "side_effect_id",
            side_effect_id.as_str(),
            "side-effect record",
        )?;
        if record
            .as_ref()
            .is_some_and(|record| record.side_effect_id() != side_effect_id)
        {
            return Err(sqlite_state_error(
                "record.identity_mismatch",
                "SQLite state record identity does not match its relational index",
            ));
        }
        Ok(record)
    }

    fn list_side_effect_records(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<Vec<SideEffectRecord>, WorkflowOsError> {
        let records: Vec<SideEffectRecord> = read_payloads(
            &self.connection()?,
            "SELECT payload FROM side_effect_records
             WHERE run_id = ?1 ORDER BY side_effect_id",
            run_id.as_str(),
            "side-effect record",
        )?;
        if records.iter().any(|record| record.run_id() != run_id) {
            return Err(sqlite_state_error(
                "record.identity_mismatch",
                "SQLite state record identity does not match its relational index",
            ));
        }
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

impl StateBackend for SqliteStateBackend {
    fn health_check(&self) -> Result<BackendHealthCheck, WorkflowOsError> {
        let connection = self.connection()?;
        validate_schema_metadata(&connection)?;
        let integrity: String = connection
            .pragma_query_value(None, "quick_check", |row| row.get(0))
            .map_err(|error| {
                map_sqlite_error(
                    error,
                    "health.failed",
                    "SQLite state integrity check could not run",
                )
            })?;
        if integrity != "ok"
            || Self::validate_all_payloads(&connection).is_err()
            || Self::validate_relational_identities(&connection).is_err()
        {
            return Ok(BackendHealthCheck {
                healthy: false,
                backend: "embedded_sqlite".to_owned(),
                message: "SQLite state integrity validation failed".to_owned(),
            });
        }
        Ok(BackendHealthCheck {
            healthy: true,
            backend: "embedded_sqlite".to_owned(),
            message: "SQLite state backend is ready".to_owned(),
        })
    }
}

impl DurableStateContractProvider for SqliteStateBackend {
    fn durable_state_contract(&self) -> Result<DurableStateSemanticContract, WorkflowOsError> {
        DurableStateSemanticContract::new(
            DurableStateContractVersion::V1,
            DurableStateBackendKind::EmbeddedSqlite,
            vec![
                DurableStateCapability::OrderedEventAppend,
                DurableStateCapability::ImmutableRunIdentityValidation,
                DurableStateCapability::IdempotencyReplay,
                DurableStateCapability::ProcessLocalExclusiveLock,
            ],
            DurableStateTransactionKind::all()
                .iter()
                .copied()
                .map(|kind| {
                    DurableStateTransactionSupport::new(
                        kind,
                        if kind == DurableStateTransactionKind::AppendRunEvent {
                            DurableStateSupport::Supported
                        } else {
                            DurableStateSupport::Unsupported
                        },
                    )
                })
                .collect(),
            DurableLeaseSemantics::ProcessLocalUnfenced,
            DurableStateSchemaMetadata::managed(
                ADAPTER_SCHEMA_VERSION,
                DurableStateSchemaPosture::Ready,
            )?,
        )
    }
}

fn validate_schema_metadata(connection: &Connection) -> Result<(), WorkflowOsError> {
    let metadata = connection
        .query_row(
            "SELECT schema_version, migration_state, checksum
             FROM schema_metadata WHERE singleton = 1",
            [],
            |row| {
                Ok((
                    row.get::<_, u32>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                ))
            },
        )
        .optional()
        .map_err(|error| {
            map_sqlite_error(
                error,
                "schema.read_failed",
                "SQLite state schema metadata could not be read",
            )
        })?;
    match metadata {
        Some((ADAPTER_SCHEMA_VERSION, state, checksum))
            if state == "ready" && checksum == SCHEMA_CHECKSUM =>
        {
            Ok(())
        }
        Some((version, _, _)) if version > ADAPTER_SCHEMA_VERSION => Err(sqlite_state_error(
            "schema.incompatible",
            "SQLite state schema version is not supported",
        )),
        _ => Err(sqlite_state_error(
            "schema.recovery_required",
            "SQLite state schema requires operator recovery",
        )),
    }
}

fn encode_json<T: Serialize>(value: &T, kind: &str) -> Result<String, WorkflowOsError> {
    serde_json::to_string(value).map_err(|_| {
        sqlite_state_error(
            "serialize.failed",
            format!("SQLite {kind} could not be encoded"),
        )
    })
}

fn decode_json<T: DeserializeOwned>(payload: &str, kind: &str) -> Result<T, WorkflowOsError> {
    serde_json::from_str(payload).map_err(|_| {
        sqlite_state_error("record.corrupt", format!("SQLite {kind} record is corrupt"))
    })
}

fn row_exists(connection: &Connection, query: &str, value: &str) -> Result<bool, WorkflowOsError> {
    connection
        .query_row(query, params![value], |_| Ok(()))
        .optional()
        .map(|value| value.is_some())
        .map_err(|error| {
            map_sqlite_error(
                error,
                "read.failed",
                "SQLite state record could not be read",
            )
        })
}

fn upsert_payload(
    connection: &Connection,
    table: &str,
    key_column: &str,
    key: &str,
    payload: &str,
) -> Result<(), WorkflowOsError> {
    let query = format!(
        "INSERT INTO {table} ({key_column}, payload) VALUES (?1, ?2)
         ON CONFLICT({key_column}) DO UPDATE SET payload = excluded.payload"
    );
    connection
        .execute(&query, params![key, payload])
        .map(|_| ())
        .map_err(|error| {
            map_sqlite_error(
                error,
                "write.failed",
                "SQLite state record could not be written",
            )
        })
}

fn read_optional_payload<T: DeserializeOwned>(
    connection: &Connection,
    table: &str,
    key_column: &str,
    key: &str,
    kind: &str,
) -> Result<Option<T>, WorkflowOsError> {
    let query = format!("SELECT payload FROM {table} WHERE {key_column} = ?1");
    let payload = connection
        .query_row(&query, params![key], |row| row.get::<_, String>(0))
        .optional()
        .map_err(|error| {
            map_sqlite_error(
                error,
                "read.failed",
                "SQLite state record could not be read",
            )
        })?;
    payload
        .map(|payload| decode_json(&payload, kind))
        .transpose()
}

fn read_payloads<T: DeserializeOwned>(
    connection: &Connection,
    query: &str,
    value: &str,
    kind: &str,
) -> Result<Vec<T>, WorkflowOsError> {
    let mut statement = connection.prepare(query).map_err(|error| {
        map_sqlite_error(
            error,
            "read.failed",
            "SQLite state records could not be read",
        )
    })?;
    let payloads = statement
        .query_map(params![value], |row| row.get::<_, String>(0))
        .map_err(|error| {
            map_sqlite_error(
                error,
                "read.failed",
                "SQLite state records could not be read",
            )
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| {
            map_sqlite_error(
                error,
                "read.failed",
                "SQLite state records could not be read",
            )
        })?;
    payloads
        .into_iter()
        .map(|payload| decode_json(&payload, kind))
        .collect()
}

fn read_all_payloads<T: DeserializeOwned>(
    connection: &Connection,
    query: &str,
    kind: &str,
) -> Result<Vec<T>, WorkflowOsError> {
    let mut statement = connection.prepare(query).map_err(|error| {
        map_sqlite_error(
            error,
            "read.failed",
            "SQLite state records could not be read",
        )
    })?;
    let payloads = statement
        .query_map([], |row| row.get::<_, String>(0))
        .map_err(|error| {
            map_sqlite_error(
                error,
                "read.failed",
                "SQLite state records could not be read",
            )
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| {
            map_sqlite_error(
                error,
                "read.failed",
                "SQLite state records could not be read",
            )
        })?;
    payloads
        .into_iter()
        .map(|payload| decode_json(&payload, kind))
        .collect()
}

#[derive(Clone, Copy)]
struct OrderedPayloadInsert<'a> {
    table: &'a str,
    id_column: &'a str,
    id: &'a str,
    run_id: &'a str,
    sort_timestamp: &'a str,
    payload: &'a str,
    kind: &'a str,
}

fn insert_ordered_payload(
    connection: &Connection,
    insert: OrderedPayloadInsert<'_>,
) -> Result<(), WorkflowOsError> {
    let (query, values) = if insert.run_id.is_empty() {
        (
            format!(
                "INSERT INTO {} ({}, sort_timestamp, payload)
                 VALUES (?1, ?2, ?3)",
                insert.table, insert.id_column
            ),
            vec![insert.id, insert.sort_timestamp, insert.payload],
        )
    } else {
        (
            format!(
                "INSERT INTO {} ({}, run_id, sort_timestamp, payload)
                 VALUES (?1, ?2, ?3, ?4)",
                insert.table, insert.id_column
            ),
            vec![
                insert.id,
                insert.run_id,
                insert.sort_timestamp,
                insert.payload,
            ],
        )
    };
    let result = connection.execute(&query, rusqlite::params_from_iter(values));
    match result {
        Ok(_) => Ok(()),
        Err(error) if is_constraint_error(&error) => Err(sqlite_state_error(
            "record.duplicate",
            format!("SQLite {} record already exists", insert.kind),
        )),
        Err(error) => Err(map_sqlite_error(
            error,
            "write.failed",
            "SQLite state record could not be written",
        )),
    }
}

fn validate_table_payloads<T: DeserializeOwned>(
    connection: &Connection,
    table: &str,
) -> Result<(), WorkflowOsError> {
    let query = format!("SELECT payload FROM {table}");
    let _: Vec<T> = read_all_payloads(connection, &query, "state")?;
    Ok(())
}

fn validate_event_identities(connection: &Connection) -> Result<(), WorkflowOsError> {
    let mut statement = connection
        .prepare("SELECT event_id, run_id, sequence_number, payload FROM events")
        .map_err(|error| relational_identity_error(error, "event"))?;
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, String>(3)?,
            ))
        })
        .map_err(|error| relational_identity_error(error, "event"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| relational_identity_error(error, "event"))?;
    for (event_id, run_id, sequence_number, payload) in rows {
        let event: WorkflowRunEvent = decode_json(&payload, "event")?;
        if event.event_id.as_str() != event_id
            || event.run_id.as_str() != run_id
            || i64::try_from(event.sequence_number.get()) != Ok(sequence_number)
        {
            return Err(relational_identity_mismatch("event"));
        }
    }
    Ok(())
}

fn validate_single_identity_table<T>(
    connection: &Connection,
    table: &str,
    key_column: &str,
    kind: &str,
    identity_matches: impl Fn(&T, &str) -> bool,
) -> Result<(), WorkflowOsError>
where
    T: DeserializeOwned,
{
    let query = format!("SELECT {key_column}, payload FROM {table}");
    let mut statement = connection
        .prepare(&query)
        .map_err(|error| relational_identity_error(error, kind))?;
    let rows = statement
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(|error| relational_identity_error(error, kind))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| relational_identity_error(error, kind))?;
    for (key, payload) in rows {
        let record: T = decode_json(&payload, kind)?;
        if !identity_matches(&record, &key) {
            return Err(relational_identity_mismatch(kind));
        }
    }
    Ok(())
}

fn validate_double_identity_table<T>(
    connection: &Connection,
    table: &str,
    columns: [&str; 2],
    kind: &str,
    identity_matches: impl Fn(&T, &str, &str) -> bool,
) -> Result<(), WorkflowOsError>
where
    T: DeserializeOwned,
{
    let query = format!(
        "SELECT {}, {}, payload FROM {table}",
        columns[0], columns[1]
    );
    let mut statement = connection
        .prepare(&query)
        .map_err(|error| relational_identity_error(error, kind))?;
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })
        .map_err(|error| relational_identity_error(error, kind))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| relational_identity_error(error, kind))?;
    for (first, second, payload) in rows {
        let record: T = decode_json(&payload, kind)?;
        if !identity_matches(&record, &first, &second) {
            return Err(relational_identity_mismatch(kind));
        }
    }
    Ok(())
}

fn validate_triple_identity_table<T>(
    connection: &Connection,
    table: &str,
    columns: [&str; 3],
    kind: &str,
    identity_matches: impl Fn(&T, &str, &str, &str) -> bool,
) -> Result<(), WorkflowOsError>
where
    T: DeserializeOwned,
{
    let query = format!(
        "SELECT {}, {}, {}, payload FROM {table}",
        columns[0], columns[1], columns[2]
    );
    let mut statement = connection
        .prepare(&query)
        .map_err(|error| relational_identity_error(error, kind))?;
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        })
        .map_err(|error| relational_identity_error(error, kind))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| relational_identity_error(error, kind))?;
    for (first, second, third, payload) in rows {
        let record: T = decode_json(&payload, kind)?;
        if !identity_matches(&record, &first, &second, &third) {
            return Err(relational_identity_mismatch(kind));
        }
    }
    Ok(())
}

fn relational_identity_error(error: SqliteError, kind: &str) -> WorkflowOsError {
    map_sqlite_error(
        error,
        "read.failed",
        &format!("SQLite {kind} identity could not be read"),
    )
}

fn relational_identity_mismatch(kind: &str) -> WorkflowOsError {
    sqlite_state_error(
        "record.identity_mismatch",
        format!("SQLite {kind} relational identity does not match its canonical record"),
    )
}

fn is_constraint_error(error: &SqliteError) -> bool {
    matches!(
        error,
        SqliteError::SqliteFailure(inner, _)
            if inner.code == ErrorCode::ConstraintViolation
    )
}

fn map_sqlite_error(error: SqliteError, suffix: &str, message: &str) -> WorkflowOsError {
    let is_busy = matches!(
        &error,
        SqliteError::SqliteFailure(inner, _)
            if matches!(inner.code, ErrorCode::DatabaseBusy | ErrorCode::DatabaseLocked)
    );
    drop(error);
    if is_busy {
        return sqlite_state_error(
            "busy",
            "SQLite state database is busy; retry after reading current state",
        );
    }
    sqlite_state_error(suffix, message)
}

fn sqlite_state_error(suffix: &str, message: impl Into<String>) -> WorkflowOsError {
    state_error(format!("state.sqlite.{suffix}"), message)
}
