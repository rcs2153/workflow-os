use std::collections::BTreeSet;
use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use rusqlite::{
    params, Connection, Error as SqliteError, ErrorCode, OpenFlags, OptionalExtension,
    TransactionBehavior,
};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::state::{
    is_allowed_side_effect_lifecycle_update, same_approval_presentation_run_identity,
    same_side_effect_run_identity, state_error, validate_append_against_history,
    LocalStateMigrationExport,
};
use crate::{
    validate_approval_presentation_approval_id,
    validate_work_report_artifact_side_effect_integrity, ActorId, AdapterRuntimeAuditRecord,
    AdapterRuntimeObservabilityRecord, AdapterTelemetryStore, ApprovalPresentationId,
    ApprovalPresentationRecord, ApprovalPresentationRecordStore, ApprovalRequest, ApprovalStore,
    BackendHealthCheck, DurableLeaseSemantics, DurableStateBackendKind, DurableStateCapability,
    DurableStateContractProvider, DurableStateContractVersion, DurableStateSchemaMetadata,
    DurableStateSchemaPosture, DurableStateSemanticContract, DurableStateSupport,
    DurableStateTransactionKind, DurableStateTransactionSupport, EventLogStore, IdempotencyKey,
    IdempotencyResult, IdempotencyStore, IdempotencyWrite, LockLease, LockStore, PolicyAuditRecord,
    PolicyAuditStore, ProjectId, ProjectStateRecord, ProjectStateStore, RunSnapshotStore,
    SideEffectId, SideEffectRecord, SideEffectRecordStore, StateBackend, StateMigrationAttempt,
    StateMigrationDigest, StateMigrationImporterTransactionVersion, StateMigrationPlan,
    StateMigrationRecordCount, StateMigrationWriterCompatibility,
    StateMigrationWriterProtocolVersion, Timestamp, WorkReportArtifactRecord,
    WorkReportArtifactSideEffectIntegrityInput, WorkReportArtifactStore, WorkReportId, WorkflowId,
    WorkflowOsError, WorkflowRun, WorkflowRunEvent, WorkflowRunId, WorkflowRunSnapshot,
};

const ADAPTER_SCHEMA_VERSION: u32 = 2;
const PREVIOUS_ADAPTER_SCHEMA_VERSION: u32 = 1;
const PREVIOUS_SCHEMA_CHECKSUM: &str = "workflow-os-sqlite-state-v1";
const PREVIOUS_SCHEMA_MANIFEST_DIGEST: &str =
    "8a35e6cb79e4908f93738e4e7320ca177a93643d7e8411b640bac81ec3c3ff96";
const SCHEMA_CHECKSUM: &str =
    "sha256:2a4c27713b3637989cfafce0ba68bb8444293edd6ce2557affc218ea13b7b1a5";
const SCHEMA_MANIFEST_DIGEST: &str =
    "2a4c27713b3637989cfafce0ba68bb8444293edd6ce2557affc218ea13b7b1a5";
const DEFAULT_BUSY_TIMEOUT: Duration = Duration::from_secs(2);

const BASE_SCHEMA: &str = r"
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
CREATE TABLE migration_metadata (
    singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
    attempt_fingerprint TEXT NOT NULL,
    plan_fingerprint TEXT NOT NULL,
    source_fingerprint TEXT NOT NULL,
    destination_id TEXT NOT NULL,
    destination_content_digest TEXT,
    verification_receipt TEXT
);
";

const CONTINUITY_SCHEMA_V2: &str = include_str!("sqlite_continuity_schema_v2.sql");
const CONTINUITY_CLOCK_PROVENANCE: &str =
    "77efdb5ae4c8696d8573d816a52dce594793b1749471a98cc58a85fc8129e50f";
const CONTINUITY_CLOCK_EPOCH: &str = "epoch/sqlite-local-live-state/1";

mod continuity_codec;
mod continuity_store;

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

/// Explicit inputs for one guarded filesystem-to-SQLite staging migration.
pub struct FilesystemToSqliteMigrationInput<'a> {
    /// Filesystem source backend. It remains unchanged.
    pub source: &'a crate::LocalStateBackend,
    /// Immutable plan created from a prior compatible inventory.
    pub plan: &'a StateMigrationPlan,
    /// New or exactly resumable inactive `SQLite` staging database.
    pub destination_path: PathBuf,
    /// Actor responsible for the verification decision.
    pub verified_by: ActorId,
    /// Verification timestamp supplied by the caller.
    pub verified_at: Timestamp,
    /// Explicit assertion that incompatible older writers are stopped.
    pub incompatible_older_writers_stopped: bool,
}

impl fmt::Debug for FilesystemToSqliteMigrationInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FilesystemToSqliteMigrationInput")
            .field("source", &"[redacted]")
            .field("plan", &"[redacted]")
            .field("destination_path", &"[redacted]")
            .field("verified_by", &"[redacted]")
            .field("verified_at", &self.verified_at)
            .field(
                "incompatible_older_writers_stopped",
                &self.incompatible_older_writers_stopped,
            )
            .finish()
    }
}

/// Payload-free proof that one exact `SQLite` staging destination was verified.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StateMigrationVerificationReceipt {
    attempt_fingerprint: StateMigrationDigest,
    plan_fingerprint: StateMigrationDigest,
    source_fingerprint: StateMigrationDigest,
    destination_content_digest: StateMigrationDigest,
    destination_id: crate::StateMigrationDestinationId,
    record_counts: Vec<StateMigrationRecordCount>,
    verified_at: Timestamp,
    verified_by: ActorId,
    companion_state_retained: bool,
}

impl StateMigrationVerificationReceipt {
    /// Returns the exact migration-attempt fingerprint.
    #[must_use]
    pub const fn attempt_fingerprint(&self) -> &StateMigrationDigest {
        &self.attempt_fingerprint
    }

    /// Returns the verified destination content digest.
    #[must_use]
    pub const fn destination_content_digest(&self) -> &StateMigrationDigest {
        &self.destination_content_digest
    }

    /// Returns canonical and projection family counts from the source inventory.
    #[must_use]
    pub fn record_counts(&self) -> &[StateMigrationRecordCount] {
        &self.record_counts
    }
}

impl fmt::Debug for StateMigrationVerificationReceipt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("StateMigrationVerificationReceipt")
            .field("attempt_fingerprint", &"<redacted>")
            .field("plan_fingerprint", &"<redacted>")
            .field("source_fingerprint", &"<redacted>")
            .field("destination_content_digest", &"<redacted>")
            .field("destination_id", &"<redacted>")
            .field("record_counts", &self.record_counts)
            .field("verified_at", &self.verified_at)
            .field("verified_by", &"<redacted>")
            .field("companion_state_retained", &self.companion_state_retained)
            .finish()
    }
}

const MIGRATION_STATE_IMPORTING_EMPTY: &str = "importing_empty";
const MIGRATION_STATE_IMPORTED_UNVERIFIED: &str = "imported_unverified";
const MIGRATION_STATE_VERIFIED_INACTIVE: &str = "verified_inactive";
const MIGRATION_STATE_READY: &str = "ready";

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
    /// Returns the current embedded adapter schema version.
    #[must_use]
    pub const fn adapter_schema_version() -> u32 {
        ADAPTER_SCHEMA_VERSION
    }

    /// Explicitly upgrades one exact ready V1 database to the additive V2
    /// continuity schema. Ordinary `open` never performs this upgrade.
    ///
    /// The operation is idempotent for an already-valid V2 database and
    /// atomic for V1. It does not select `SQLite` as the runtime backend or
    /// enable continuity operations by itself.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error for unknown, incomplete, staged, or
    /// checksum-mismatched databases.
    pub fn upgrade_authorized_execution_continuity_v1_to_v2(
        database_path: impl Into<PathBuf>,
    ) -> Result<Self, WorkflowOsError> {
        let backend = Self {
            database_path: database_path.into(),
            busy_timeout: DEFAULT_BUSY_TIMEOUT,
        };
        let mut connection = backend.existing_connection()?;
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|error| {
                map_sqlite_error(
                    error,
                    "schema.upgrade_failed",
                    "SQLite state schema upgrade could not start",
                )
            })?;
        let version: u32 = transaction
            .pragma_query_value(None, "user_version", |row| row.get(0))
            .map_err(|error| {
                map_sqlite_error(
                    error,
                    "schema.read_failed",
                    "SQLite state schema metadata could not be read",
                )
            })?;
        if version == ADAPTER_SCHEMA_VERSION {
            validate_schema_metadata(&transaction)?;
            transaction.commit().map_err(|error| {
                map_sqlite_error(
                    error,
                    "schema.upgrade_failed",
                    "SQLite state schema upgrade could not commit",
                )
            })?;
            return Ok(backend);
        }
        if version != PREVIOUS_ADAPTER_SCHEMA_VERSION {
            return Err(sqlite_state_error(
                "schema.incompatible",
                "SQLite state schema version is not supported",
            ));
        }
        validate_v1_upgrade_eligibility(&transaction)?;
        transaction
            .execute_batch(CONTINUITY_SCHEMA_V2)
            .and_then(|()| {
                initialize_continuity_trusted_time(&transaction)?;
                transaction.execute(
                    "UPDATE schema_metadata
                     SET schema_version = ?1, checksum = ?2
                     WHERE singleton = 1",
                    params![ADAPTER_SCHEMA_VERSION, SCHEMA_CHECKSUM],
                )?;
                Ok(())
            })
            .and_then(|()| transaction.pragma_update(None, "user_version", ADAPTER_SCHEMA_VERSION))
            .map_err(|error| {
                map_sqlite_error(
                    error,
                    "schema.upgrade_failed",
                    "SQLite state schema upgrade failed",
                )
            })?;
        validate_schema_metadata(&transaction)?;
        transaction.commit().map_err(|error| {
            map_sqlite_error(
                error,
                "schema.upgrade_failed",
                "SQLite state schema upgrade could not commit",
            )
        })?;
        Ok(backend)
    }

    /// Imports one guarded filesystem source into inactive `SQLite` staging.
    ///
    /// The source remains under the exclusive cooperating-writer guard through
    /// export, one-transaction import, source recheck, destination verification,
    /// and receipt persistence. The returned receipt does not activate `SQLite`.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking migration error for stale plans,
    /// incompatible writers, existing unknown destinations, import failure, or
    /// failed verification.
    pub fn stage_filesystem_migration(
        input: FilesystemToSqliteMigrationInput<'_>,
    ) -> Result<StateMigrationVerificationReceipt, WorkflowOsError> {
        if input.plan.destination().adapter_schema_version() != ADAPTER_SCHEMA_VERSION {
            return Err(migration_runtime_error(
                "destination.schema_incompatible",
                "state migration destination schema is incompatible",
            ));
        }
        let guard = input.source.try_acquire_exclusive_migration_guard()?;
        let inventory = guard.inspect_migration_inventory()?;
        let source_fingerprint = inventory.source_fingerprint().ok_or_else(|| {
            migration_runtime_error(
                "source.fingerprint_missing",
                "state migration source fingerprint is unavailable",
            )
        })?;
        if source_fingerprint != input.plan.source().source_fingerprint() {
            return Err(migration_runtime_error(
                "source.changed",
                "state migration source changed after planning",
            ));
        }

        let capability = guard.capability();
        let compatibility = StateMigrationWriterCompatibility::assess(
            input.plan.source().backend_kind(),
            Some(StateMigrationWriterProtocolVersion::V1),
            &capability,
            input.incompatible_older_writers_stopped,
        );
        let attempt = StateMigrationAttempt::new(
            input.plan,
            &capability,
            &compatibility,
            StateMigrationImporterTransactionVersion::V1,
        )?;
        let export = guard.export_migration_records()?;

        let backend = Self {
            database_path: input.destination_path,
            busy_timeout: DEFAULT_BUSY_TIMEOUT,
        };
        let mut connection = backend.migration_connection()?;
        let state = Self::prepare_or_resume_staging(&mut connection, input.plan, &attempt)?;
        if state == MIGRATION_STATE_IMPORTING_EMPTY {
            Self::import_export(&mut connection, &export)?;
        } else if state == MIGRATION_STATE_VERIFIED_INACTIVE {
            return Self::read_verified_receipt(&connection, &attempt);
        } else if state != MIGRATION_STATE_IMPORTED_UNVERIFIED {
            return Err(migration_runtime_error(
                "destination.state.unknown",
                "state migration destination requires explicit recovery",
            ));
        }

        let rechecked = guard.inspect_migration_inventory()?;
        if rechecked.source_fingerprint() != Some(attempt.source_fingerprint()) {
            return Err(migration_runtime_error(
                "source.changed",
                "state migration source changed during import",
            ));
        }
        let destination_content_digest = Self::verify_staging(&connection, &export)?;
        let receipt = StateMigrationVerificationReceipt {
            attempt_fingerprint: attempt.attempt_fingerprint().clone(),
            plan_fingerprint: attempt.plan_fingerprint().clone(),
            source_fingerprint: attempt.source_fingerprint().clone(),
            destination_content_digest,
            destination_id: attempt.destination_id().clone(),
            record_counts: export.inventory.record_counts().to_vec(),
            verified_at: input.verified_at,
            verified_by: input.verified_by,
            companion_state_retained: true,
        };
        Self::persist_verified_receipt(&mut connection, &receipt)?;
        Ok(receipt)
    }

    /// Activates one exact verified inactive staging database.
    ///
    /// Activation changes only the destination metadata from
    /// `verified_inactive` to `ready`. It does not select the backend globally,
    /// delete the filesystem source, or perform external writes.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when the receipt does not exactly
    /// match persisted verification state or staging is not inactive.
    pub fn activate_verified_migration(
        database_path: impl Into<PathBuf>,
        receipt: &StateMigrationVerificationReceipt,
    ) -> Result<Self, WorkflowOsError> {
        let backend = Self {
            database_path: database_path.into(),
            busy_timeout: DEFAULT_BUSY_TIMEOUT,
        };
        let mut connection = backend.migration_connection()?;
        let persisted = Self::read_receipt_payload(&connection)?;
        if persisted != *receipt {
            return Err(migration_runtime_error(
                "activation.receipt_mismatch",
                "state migration activation receipt does not match",
            ));
        }
        let state = read_migration_state(&connection)?;
        if state != MIGRATION_STATE_VERIFIED_INACTIVE {
            return Err(migration_runtime_error(
                "activation.state_invalid",
                "state migration destination is not verified inactive",
            ));
        }
        let current_digest = Self::destination_content_digest(&connection)?;
        if current_digest != receipt.destination_content_digest {
            return Err(migration_runtime_error(
                "activation.destination_changed",
                "state migration destination changed after verification",
            ));
        }
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|error| {
                map_sqlite_error(
                    error,
                    "state.migration.activation.failed",
                    "state migration activation transaction could not start",
                )
            })?;
        validate_schema_manifest(&transaction, SCHEMA_MANIFEST_DIGEST).map_err(|_| {
            migration_runtime_error(
                "activation.schema_invalid",
                "state migration destination schema is invalid",
            )
        })?;
        validate_continuity_security_state(&transaction).map_err(|_| {
            migration_runtime_error(
                "activation.schema_invalid",
                "state migration destination schema is invalid",
            )
        })?;
        let locked_digest = Self::destination_content_digest(&transaction)?;
        if locked_digest != receipt.destination_content_digest {
            return Err(migration_runtime_error(
                "activation.destination_changed",
                "state migration destination changed after verification",
            ));
        }
        transaction
            .execute(
                "UPDATE schema_metadata SET migration_state = ?1 WHERE singleton = 1",
                params![MIGRATION_STATE_READY],
            )
            .map_err(|error| {
                map_sqlite_error(
                    error,
                    "state.migration.activation.failed",
                    "state migration destination could not be activated",
                )
            })?;
        transaction.commit().map_err(|error| {
            map_sqlite_error(
                error,
                "state.migration.activation.failed",
                "state migration activation could not commit",
            )
        })?;
        Self::open(backend.database_path)
    }

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

    fn migration_connection(&self) -> Result<Connection, WorkflowOsError> {
        if let Some(parent) = self.database_path.parent() {
            fs::create_dir_all(parent).map_err(|_| {
                migration_runtime_error(
                    "destination.prepare_failed",
                    "state migration destination could not be prepared",
                )
            })?;
        }
        self.connection()
    }

    #[allow(clippy::too_many_lines)]
    fn prepare_or_resume_staging(
        connection: &mut Connection,
        plan: &StateMigrationPlan,
        attempt: &StateMigrationAttempt,
    ) -> Result<String, WorkflowOsError> {
        let version: u32 = connection
            .pragma_query_value(None, "user_version", |row| row.get(0))
            .map_err(|_| {
                migration_runtime_error(
                    "destination.read_failed",
                    "state migration destination metadata could not be read",
                )
            })?;
        if version == 0 {
            let transaction = connection
                .transaction_with_behavior(TransactionBehavior::Immediate)
                .map_err(|_| {
                    migration_runtime_error(
                        "destination.initialize_failed",
                        "state migration destination could not be initialized",
                    )
                })?;
            let locked_version: u32 = transaction
                .pragma_query_value(None, "user_version", |row| row.get(0))
                .map_err(|_| {
                    migration_runtime_error(
                        "destination.read_failed",
                        "state migration destination metadata could not be read",
                    )
                })?;
            let object_count = schema_object_count(&transaction).map_err(|_| {
                migration_runtime_error(
                    "destination.read_failed",
                    "state migration destination metadata could not be read",
                )
            })?;
            if locked_version != 0 || object_count != 0 {
                return Err(migration_runtime_error(
                    "destination.not_empty",
                    "state migration destination is not empty",
                ));
            }
            transaction.execute_batch(BASE_SCHEMA).map_err(|_| {
                migration_runtime_error(
                    "destination.initialize_failed",
                    "state migration destination could not be initialized",
                )
            })?;
            transaction
                .execute_batch(CONTINUITY_SCHEMA_V2)
                .map_err(|_| {
                    migration_runtime_error(
                        "destination.initialize_failed",
                        "state migration destination could not be initialized",
                    )
                })?;
            initialize_continuity_trusted_time(&transaction).map_err(|_| {
                migration_runtime_error(
                    "destination.initialize_failed",
                    "state migration destination could not be initialized",
                )
            })?;
            transaction
                .execute(
                    "INSERT INTO schema_metadata
                     (singleton, schema_version, migration_state, checksum)
                     VALUES (1, ?1, ?2, ?3)",
                    params![
                        ADAPTER_SCHEMA_VERSION,
                        MIGRATION_STATE_IMPORTING_EMPTY,
                        SCHEMA_CHECKSUM
                    ],
                )
                .and_then(|_| {
                    transaction.execute(
                        "INSERT INTO migration_metadata
                         (singleton, attempt_fingerprint, plan_fingerprint,
                          source_fingerprint, destination_id)
                         VALUES (1, ?1, ?2, ?3, ?4)",
                        params![
                            attempt.attempt_fingerprint().as_str(),
                            attempt.plan_fingerprint().as_str(),
                            attempt.source_fingerprint().as_str(),
                            plan.destination().destination_id().as_str(),
                        ],
                    )
                })
                .map_err(|_| {
                    migration_runtime_error(
                        "destination.initialize_failed",
                        "state migration destination could not be initialized",
                    )
                })?;
            transaction
                .pragma_update(None, "user_version", ADAPTER_SCHEMA_VERSION)
                .map_err(|_| {
                    migration_runtime_error(
                        "destination.initialize_failed",
                        "state migration destination could not be initialized",
                    )
                })?;
            validate_schema_manifest(&transaction, SCHEMA_MANIFEST_DIGEST).map_err(|_| {
                migration_runtime_error(
                    "destination.initialize_failed",
                    "state migration destination could not be initialized",
                )
            })?;
            validate_continuity_security_state(&transaction).map_err(|_| {
                migration_runtime_error(
                    "destination.initialize_failed",
                    "state migration destination could not be initialized",
                )
            })?;
            transaction.commit().map_err(|_| {
                migration_runtime_error(
                    "destination.initialize_failed",
                    "state migration destination could not be initialized",
                )
            })?;
            return Ok(MIGRATION_STATE_IMPORTING_EMPTY.to_owned());
        }
        if version != ADAPTER_SCHEMA_VERSION {
            return Err(migration_runtime_error(
                "destination.schema_incompatible",
                "state migration destination schema is incompatible",
            ));
        }
        validate_schema_manifest(connection, SCHEMA_MANIFEST_DIGEST).map_err(|_| {
            migration_runtime_error(
                "destination.schema_incompatible",
                "state migration destination schema is incompatible",
            )
        })?;
        validate_continuity_security_state(connection).map_err(|_| {
            migration_runtime_error(
                "destination.schema_incompatible",
                "state migration destination schema is incompatible",
            )
        })?;
        let binding = connection
            .query_row(
                "SELECT attempt_fingerprint, plan_fingerprint, source_fingerprint,
                        destination_id
                 FROM migration_metadata WHERE singleton = 1",
                [],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?,
                    ))
                },
            )
            .optional()
            .map_err(|_| {
                migration_runtime_error(
                    "destination.read_failed",
                    "state migration destination metadata could not be read",
                )
            })?;
        let expected = (
            attempt.attempt_fingerprint().as_str(),
            attempt.plan_fingerprint().as_str(),
            attempt.source_fingerprint().as_str(),
            plan.destination().destination_id().as_str(),
        );
        if binding.as_ref().map(|value| {
            (
                value.0.as_str(),
                value.1.as_str(),
                value.2.as_str(),
                value.3.as_str(),
            )
        }) != Some(expected)
        {
            return Err(migration_runtime_error(
                "destination.binding_mismatch",
                "state migration destination belongs to another attempt",
            ));
        }
        read_migration_state(connection)
    }

    fn import_export(
        connection: &mut Connection,
        export: &LocalStateMigrationExport,
    ) -> Result<(), WorkflowOsError> {
        Self::import_export_with_failure(connection, export, false)
    }

    #[allow(clippy::too_many_lines)]
    fn import_export_with_failure(
        connection: &mut Connection,
        export: &LocalStateMigrationExport,
        fail_after_events: bool,
    ) -> Result<(), WorkflowOsError> {
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| {
                migration_runtime_error(
                    "transaction.start_failed",
                    "state migration import transaction could not start",
                )
            })?;
        if read_migration_state(&transaction)? != MIGRATION_STATE_IMPORTING_EMPTY {
            return Err(migration_runtime_error(
                "transaction.state_invalid",
                "state migration destination is not empty staging",
            ));
        }
        validate_export_referential_integrity(export)?;

        let mut events_by_run =
            std::collections::BTreeMap::<WorkflowRunId, Vec<WorkflowRunEvent>>::new();
        for event in &export.events {
            let sequence = i64::try_from(event.sequence_number.get()).map_err(|_| {
                migration_runtime_error(
                    "transaction.record_invalid",
                    "state migration record is invalid",
                )
            })?;
            transaction
                .execute(
                    "INSERT INTO events
                     (event_id, run_id, sequence_number, payload)
                     VALUES (?1, ?2, ?3, ?4)",
                    params![
                        event.event_id.as_str(),
                        event.run_id.as_str(),
                        sequence,
                        encode_json(event, "migration event")?
                    ],
                )
                .map_err(|_| import_failed())?;
            events_by_run
                .entry(event.run_id.clone())
                .or_default()
                .push(event.clone());
        }
        if fail_after_events {
            return Err(import_failed());
        }

        for (run_id, events) in events_by_run {
            let run = WorkflowRun::rehydrate(&events).map_err(|_| import_failed())?;
            transaction
                .execute(
                    "INSERT INTO snapshots (run_id, payload) VALUES (?1, ?2)",
                    params![
                        run_id.as_str(),
                        encode_json(&run.snapshot, "migration snapshot")?
                    ],
                )
                .map_err(|_| import_failed())?;
            for approval in run
                .snapshot
                .approval_requests
                .iter()
                .filter(|approval| approval.decision.is_none())
            {
                transaction
                    .execute(
                        "INSERT INTO approvals (approval_id, payload) VALUES (?1, ?2)",
                        params![
                            approval.approval_id,
                            encode_json(approval, "migration approval")?
                        ],
                    )
                    .map_err(|_| import_failed())?;
            }
        }

        for (storage_key, result) in &export.idempotency_results {
            transaction
                .execute(
                    "INSERT INTO idempotency_results (idempotency_key, payload)
                     VALUES (?1, ?2)",
                    params![
                        storage_key,
                        encode_json(result, "migration idempotency result")?
                    ],
                )
                .map_err(|_| import_failed())?;
        }
        for record in &export.approval_presentations {
            transaction
                .execute(
                    "INSERT INTO approval_presentations
                     (presentation_id, run_id, approval_id, payload)
                     VALUES (?1, ?2, ?3, ?4)",
                    params![
                        record.presentation_id().as_str(),
                        record.run_id().as_str(),
                        record.approval_id(),
                        encode_json(record, "migration approval presentation")?
                    ],
                )
                .map_err(|_| import_failed())?;
        }
        for record in &export.projects {
            transaction
                .execute(
                    "INSERT INTO projects (project_id, payload) VALUES (?1, ?2)",
                    params![
                        record.project_id.as_str(),
                        encode_json(record, "migration project")?
                    ],
                )
                .map_err(|_| import_failed())?;
        }
        for record in &export.policy_audit {
            transaction
                .execute(
                    "INSERT INTO policy_audit
                     (audit_id, sort_timestamp, payload) VALUES (?1, ?2, ?3)",
                    params![
                        record.audit_id.as_str(),
                        record.timestamp.to_string(),
                        encode_json(record, "migration policy audit")?
                    ],
                )
                .map_err(|_| import_failed())?;
        }
        for record in &export.adapter_audit {
            let run_id = record.workflow_run_id.as_ref().ok_or_else(import_failed)?;
            transaction
                .execute(
                    "INSERT INTO adapter_audit
                     (telemetry_id, run_id, sort_timestamp, payload)
                     VALUES (?1, ?2, ?3, ?4)",
                    params![
                        record.telemetry_id.as_str(),
                        run_id.as_str(),
                        record.timestamp.to_string(),
                        encode_json(record, "migration adapter audit")?
                    ],
                )
                .map_err(|_| import_failed())?;
        }
        for record in &export.adapter_observability {
            let run_id = record.workflow_run_id.as_ref().ok_or_else(import_failed)?;
            transaction
                .execute(
                    "INSERT INTO adapter_observability
                     (telemetry_id, run_id, sort_timestamp, payload)
                     VALUES (?1, ?2, ?3, ?4)",
                    params![
                        record.telemetry_id.as_str(),
                        run_id.as_str(),
                        record.timestamp.to_string(),
                        encode_json(record, "migration adapter observability")?
                    ],
                )
                .map_err(|_| import_failed())?;
        }
        for record in &export.side_effects {
            record.validate().map_err(|_| import_failed())?;
            transaction
                .execute(
                    "INSERT INTO side_effect_records
                     (side_effect_id, run_id, workflow_id, payload)
                     VALUES (?1, ?2, ?3, ?4)",
                    params![
                        record.side_effect_id().as_str(),
                        record.run_id().as_str(),
                        record.workflow_id().as_str(),
                        encode_json(record, "migration side effect")?
                    ],
                )
                .map_err(|_| import_failed())?;
        }
        for record in &export.work_reports {
            record.validate().map_err(|_| import_failed())?;
            transaction
                .execute(
                    "INSERT INTO work_report_artifacts
                     (run_id, report_id, payload) VALUES (?1, ?2, ?3)",
                    params![
                        record.run_id().as_str(),
                        record.report_id().as_str(),
                        encode_json(record, "migration work report")?
                    ],
                )
                .map_err(|_| import_failed())?;
        }
        transaction
            .execute(
                "UPDATE schema_metadata SET migration_state = ?1 WHERE singleton = 1",
                params![MIGRATION_STATE_IMPORTED_UNVERIFIED],
            )
            .map_err(|_| import_failed())?;
        transaction.commit().map_err(|_| import_failed())
    }

    fn verify_staging(
        connection: &Connection,
        export: &LocalStateMigrationExport,
    ) -> Result<StateMigrationDigest, WorkflowOsError> {
        let quick_check: String = connection
            .query_row("PRAGMA quick_check", [], |row| row.get(0))
            .map_err(|_| verification_failed())?;
        if quick_check != "ok" {
            return Err(verification_failed());
        }
        let expected = [
            ("events", export.events.len()),
            ("idempotency_results", export.idempotency_results.len()),
            (
                "approval_presentations",
                export.approval_presentations.len(),
            ),
            ("projects", export.projects.len()),
            ("policy_audit", export.policy_audit.len()),
            ("adapter_audit", export.adapter_audit.len()),
            ("adapter_observability", export.adapter_observability.len()),
            ("work_report_artifacts", export.work_reports.len()),
            ("side_effect_records", export.side_effects.len()),
        ];
        for (table, count) in expected {
            if table_count(connection, table)? != count {
                return Err(verification_failed());
            }
        }
        if table_count(connection, "locks")? != 0 {
            return Err(verification_failed());
        }
        if migration_export_canonical_digest(export)?
            != Self::destination_canonical_content_digest(connection)?
        {
            return Err(verification_failed());
        }
        Self::destination_content_digest(connection)
    }

    fn destination_canonical_content_digest(
        connection: &Connection,
    ) -> Result<StateMigrationDigest, WorkflowOsError> {
        let mut hasher = Sha256::new();
        hash_migration_family(
            &mut hasher,
            "events",
            read_migration_payloads(connection, "SELECT payload FROM events")?,
        );

        let mut statement = connection
            .prepare("SELECT idempotency_key, payload FROM idempotency_results")
            .map_err(|_| verification_failed())?;
        let rows = statement
            .query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })
            .map_err(|_| verification_failed())?;
        let mut idempotency = Vec::new();
        for row in rows {
            let (storage_key, payload) = row.map_err(|_| verification_failed())?;
            idempotency.push(format!("{storage_key}:{payload}"));
        }
        hash_migration_family(&mut hasher, "idempotency_results", idempotency);
        for (family, query) in [
            (
                "approval_presentations",
                "SELECT payload FROM approval_presentations",
            ),
            ("projects", "SELECT payload FROM projects"),
            ("policy_audit", "SELECT payload FROM policy_audit"),
            ("adapter_audit", "SELECT payload FROM adapter_audit"),
            (
                "adapter_observability",
                "SELECT payload FROM adapter_observability",
            ),
            (
                "work_report_artifacts",
                "SELECT payload FROM work_report_artifacts",
            ),
            (
                "side_effect_records",
                "SELECT payload FROM side_effect_records",
            ),
        ] {
            hash_migration_family(
                &mut hasher,
                family,
                read_migration_payloads(connection, query)?,
            );
        }
        Ok(StateMigrationDigest::from_hasher(hasher))
    }

    fn destination_content_digest(
        connection: &Connection,
    ) -> Result<StateMigrationDigest, WorkflowOsError> {
        let queries = [
            ("events", "SELECT payload FROM events ORDER BY run_id, sequence_number"),
            ("snapshots", "SELECT payload FROM snapshots ORDER BY run_id"),
            (
                "idempotency_results",
                "SELECT idempotency_key || ':' || payload FROM idempotency_results ORDER BY idempotency_key",
            ),
            ("approvals", "SELECT payload FROM approvals ORDER BY approval_id"),
            (
                "approval_presentations",
                "SELECT payload FROM approval_presentations ORDER BY presentation_id",
            ),
            ("projects", "SELECT payload FROM projects ORDER BY project_id"),
            (
                "policy_audit",
                "SELECT payload FROM policy_audit ORDER BY sort_timestamp, audit_id",
            ),
            (
                "adapter_audit",
                "SELECT payload FROM adapter_audit ORDER BY sort_timestamp, telemetry_id",
            ),
            (
                "adapter_observability",
                "SELECT payload FROM adapter_observability ORDER BY sort_timestamp, telemetry_id",
            ),
            (
                "work_report_artifacts",
                "SELECT payload FROM work_report_artifacts ORDER BY run_id, report_id",
            ),
            (
                "side_effect_records",
                "SELECT payload FROM side_effect_records ORDER BY side_effect_id",
            ),
        ];
        let mut hasher = Sha256::new();
        for (table, query) in queries {
            hash_migration_frame(&mut hasher, table.as_bytes());
            let mut statement = connection
                .prepare(query)
                .map_err(|_| verification_failed())?;
            let rows = statement
                .query_map([], |row| row.get::<_, String>(0))
                .map_err(|_| verification_failed())?;
            for row in rows {
                hash_migration_frame(
                    &mut hasher,
                    row.map_err(|_| verification_failed())?.as_bytes(),
                );
            }
        }
        Ok(StateMigrationDigest::from_hasher(hasher))
    }

    fn persist_verified_receipt(
        connection: &mut Connection,
        receipt: &StateMigrationVerificationReceipt,
    ) -> Result<(), WorkflowOsError> {
        let payload = encode_json(receipt, "migration verification receipt")?;
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| verification_failed())?;
        transaction
            .execute(
                "UPDATE migration_metadata
                 SET destination_content_digest = ?1, verification_receipt = ?2
                 WHERE singleton = 1",
                params![receipt.destination_content_digest.as_str(), payload],
            )
            .and_then(|_| {
                transaction.execute(
                    "UPDATE schema_metadata SET migration_state = ?1 WHERE singleton = 1",
                    params![MIGRATION_STATE_VERIFIED_INACTIVE],
                )
            })
            .map_err(|_| verification_failed())?;
        transaction.commit().map_err(|_| verification_failed())
    }

    fn read_verified_receipt(
        connection: &Connection,
        attempt: &StateMigrationAttempt,
    ) -> Result<StateMigrationVerificationReceipt, WorkflowOsError> {
        let receipt = Self::read_receipt_payload(connection)?;
        if receipt.attempt_fingerprint != *attempt.attempt_fingerprint()
            || receipt.plan_fingerprint != *attempt.plan_fingerprint()
            || receipt.source_fingerprint != *attempt.source_fingerprint()
            || receipt.destination_id != *attempt.destination_id()
        {
            return Err(migration_runtime_error(
                "destination.binding_mismatch",
                "state migration destination belongs to another attempt",
            ));
        }
        Ok(receipt)
    }

    fn read_receipt_payload(
        connection: &Connection,
    ) -> Result<StateMigrationVerificationReceipt, WorkflowOsError> {
        let payload = connection
            .query_row(
                "SELECT verification_receipt FROM migration_metadata WHERE singleton = 1",
                [],
                |row| row.get::<_, Option<String>>(0),
            )
            .optional()
            .map_err(|_| verification_failed())?
            .flatten()
            .ok_or_else(verification_failed)?;
        decode_json(&payload, "migration verification receipt").map_err(|_| verification_failed())
    }

    fn connection(&self) -> Result<Connection, WorkflowOsError> {
        let connection = Connection::open(&self.database_path).map_err(|error| {
            map_sqlite_error(
                error,
                "open.failed",
                "SQLite state database could not be opened",
            )
        })?;
        self.configure_connection(connection)
    }

    fn existing_connection(&self) -> Result<Connection, WorkflowOsError> {
        let connection = Connection::open_with_flags(
            &self.database_path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_URI,
        )
        .map_err(|error| {
            map_sqlite_error(
                error,
                "open.failed",
                "SQLite state database could not be opened",
            )
        })?;
        self.configure_connection(connection)
    }

    fn configure_connection(&self, connection: Connection) -> Result<Connection, WorkflowOsError> {
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

    #[allow(clippy::too_many_lines)]
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
                let locked_version: u32 = transaction
                    .pragma_query_value(None, "user_version", |row| row.get(0))
                    .map_err(|error| {
                        map_sqlite_error(
                            error,
                            "schema.read_failed",
                            "SQLite state schema metadata could not be read",
                        )
                    })?;
                if locked_version == ADAPTER_SCHEMA_VERSION {
                    validate_schema_metadata(&transaction)?;
                    transaction.commit().map_err(|error| {
                        map_sqlite_error(
                            error,
                            "schema.initialize_failed",
                            "SQLite state schema could not be initialized",
                        )
                    })?;
                    return Ok(());
                }
                if locked_version != 0 || schema_object_count(&transaction)? != 0 {
                    return Err(sqlite_state_error(
                        "schema.nonempty_unmanaged",
                        "SQLite state database is not an empty managed database",
                    ));
                }
                transaction.execute_batch(BASE_SCHEMA).map_err(|error| {
                    map_sqlite_error(
                        error,
                        "schema.initialize_failed",
                        "SQLite state schema could not be initialized",
                    )
                })?;
                transaction
                    .execute_batch(CONTINUITY_SCHEMA_V2)
                    .map_err(|error| {
                        map_sqlite_error(
                            error,
                            "schema.initialize_failed",
                            "SQLite state schema could not be initialized",
                        )
                    })?;
                initialize_continuity_trusted_time(&transaction).map_err(|error| {
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
                validate_schema_metadata(&transaction)?;
                transaction.commit().map_err(|error| {
                    map_sqlite_error(
                        error,
                        "schema.initialize_failed",
                        "SQLite state schema could not be initialized",
                    )
                })?;
                Ok(())
            }
            PREVIOUS_ADAPTER_SCHEMA_VERSION => {
                validate_v1_upgrade_eligibility(connection)?;
                Err(sqlite_state_error(
                    "schema.upgrade_required",
                    "SQLite state schema requires an explicit continuity upgrade",
                ))
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
        let storage_key = sqlite_idempotency_storage_key(key);
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
                params![storage_key],
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
                params![storage_key, encode_json(&result, "idempotency result")?],
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

fn initialize_continuity_trusted_time(connection: &Connection) -> Result<(), rusqlite::Error> {
    connection.execute(
        "INSERT INTO continuity_trusted_time
         (singleton_id, source_kind, provenance_commitment, epoch_id,
          observed_seconds, observed_nanos, posture, eligibility, revision)
         VALUES (1, 'core_injected_clock_v1', ?1, ?2,
                 NULL, NULL, 'unobserved', 'live_state_eligible', 1)",
        params![CONTINUITY_CLOCK_PROVENANCE, CONTINUITY_CLOCK_EPOCH],
    )?;
    Ok(())
}

fn validate_v1_upgrade_eligibility(connection: &Connection) -> Result<(), WorkflowOsError> {
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
        .map_err(|_| schema_recovery_required())?;
    if metadata
        != Some((
            PREVIOUS_ADAPTER_SCHEMA_VERSION,
            "ready".to_owned(),
            PREVIOUS_SCHEMA_CHECKSUM.to_owned(),
        ))
    {
        return Err(schema_recovery_required());
    }
    validate_schema_manifest(connection, PREVIOUS_SCHEMA_MANIFEST_DIGEST)
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
            validate_schema_manifest(connection, SCHEMA_MANIFEST_DIGEST)?;
            validate_continuity_security_state(connection)
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

fn validate_schema_manifest(
    connection: &Connection,
    expected_digest: &str,
) -> Result<(), WorkflowOsError> {
    let mut statement = connection
        .prepare(
            "SELECT type, name, COALESCE(sql, '') FROM sqlite_master
             WHERE name NOT LIKE 'sqlite_%' ORDER BY type, name",
        )
        .map_err(|_| schema_recovery_required())?;
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })
        .map_err(|_| schema_recovery_required())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| schema_recovery_required())?;
    let manifest = rows
        .into_iter()
        .map(|(kind, name, sql)| format!("{kind}|{name}|{sql}"))
        .collect::<Vec<_>>()
        .join("\n");
    let digest = format!("{:x}", Sha256::digest(manifest.as_bytes()));
    if digest != expected_digest {
        return Err(schema_recovery_required());
    }
    Ok(())
}

fn schema_object_count(connection: &Connection) -> Result<u32, WorkflowOsError> {
    connection
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE name NOT LIKE 'sqlite_%'",
            [],
            |row| row.get(0),
        )
        .map_err(|error| {
            map_sqlite_error(
                error,
                "schema.read_failed",
                "SQLite state schema metadata could not be read",
            )
        })
}

fn validate_continuity_security_state(connection: &Connection) -> Result<(), WorkflowOsError> {
    let singleton_count: u32 = connection
        .query_row(
            "SELECT COUNT(*) FROM continuity_trusted_time
             WHERE singleton_id = 1
               AND source_kind = 'core_injected_clock_v1'
               AND provenance_commitment = ?1
               AND epoch_id = ?2
               AND revision > 0
               AND ((posture = 'unobserved'
                     AND observed_seconds IS NULL
                     AND observed_nanos IS NULL
                     AND eligibility IN ('live_state_eligible','restore_unverified'))
                    OR (posture = 'healthy'
                        AND observed_seconds IS NOT NULL
                        AND observed_nanos IS NOT NULL
                        AND eligibility IN ('live_state_eligible','restore_unverified'))
                    OR (posture = 'quarantined'
                        AND observed_seconds IS NOT NULL
                        AND observed_nanos IS NOT NULL
                        AND eligibility = 'quarantined'))",
            params![CONTINUITY_CLOCK_PROVENANCE, CONTINUITY_CLOCK_EPOCH],
            |row| row.get(0),
        )
        .map_err(|_| schema_recovery_required())?;
    if singleton_count != 1 {
        return Err(schema_recovery_required());
    }
    let foreign_key_violation = connection
        .prepare("PRAGMA foreign_key_check")
        .and_then(|mut statement| statement.exists([]))
        .map_err(|_| schema_recovery_required())?;
    if foreign_key_violation {
        return Err(schema_recovery_required());
    }
    Ok(())
}

fn schema_recovery_required() -> WorkflowOsError {
    sqlite_state_error(
        "schema.recovery_required",
        "SQLite state schema requires operator recovery",
    )
}

fn read_migration_state(connection: &Connection) -> Result<String, WorkflowOsError> {
    connection
        .query_row(
            "SELECT migration_state FROM schema_metadata WHERE singleton = 1",
            [],
            |row| row.get(0),
        )
        .map_err(|_| {
            migration_runtime_error(
                "destination.read_failed",
                "state migration destination metadata could not be read",
            )
        })
}

fn table_count(connection: &Connection, table: &str) -> Result<usize, WorkflowOsError> {
    let query = format!("SELECT COUNT(*) FROM {table}");
    let count = connection
        .query_row(&query, [], |row| row.get::<_, i64>(0))
        .map_err(|_| verification_failed())?;
    usize::try_from(count).map_err(|_| verification_failed())
}

fn validate_export_referential_integrity(
    export: &LocalStateMigrationExport,
) -> Result<(), WorkflowOsError> {
    let run_ids = export
        .events
        .iter()
        .map(|event| event.run_id.clone())
        .collect::<BTreeSet<_>>();
    let has_run = |run_id: &WorkflowRunId| run_ids.contains(run_id);

    if export
        .approval_presentations
        .iter()
        .any(|record| !has_run(record.run_id()))
        || export
            .adapter_audit
            .iter()
            .any(|record| match record.workflow_run_id.as_ref() {
                Some(run_id) => !has_run(run_id),
                None => true,
            })
        || export
            .adapter_observability
            .iter()
            .any(|record| match record.workflow_run_id.as_ref() {
                Some(run_id) => !has_run(run_id),
                None => true,
            })
        || export
            .side_effects
            .iter()
            .any(|record| !has_run(record.run_id()))
        || export
            .work_reports
            .iter()
            .any(|record| !has_run(record.run_id()))
    {
        return Err(import_failed());
    }

    let store = MigrationSideEffectRecordStore {
        records: &export.side_effects,
    };
    for artifact in &export.work_reports {
        validate_work_report_artifact_side_effect_integrity(
            &store,
            WorkReportArtifactSideEffectIntegrityInput {
                artifact,
                require_all_side_effect_citations: true,
            },
        )
        .map_err(|_| import_failed())?;
    }
    Ok(())
}

struct MigrationSideEffectRecordStore<'a> {
    records: &'a [SideEffectRecord],
}

impl SideEffectRecordStore for MigrationSideEffectRecordStore<'_> {
    fn write_side_effect_record(&self, _record: &SideEffectRecord) -> Result<(), WorkflowOsError> {
        Err(import_failed())
    }

    fn read_side_effect_record(
        &self,
        side_effect_id: &SideEffectId,
    ) -> Result<Option<SideEffectRecord>, WorkflowOsError> {
        Ok(self
            .records
            .iter()
            .find(|record| record.side_effect_id() == side_effect_id)
            .cloned())
    }

    fn list_side_effect_records(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<Vec<SideEffectRecord>, WorkflowOsError> {
        Ok(self
            .records
            .iter()
            .filter(|record| record.run_id() == run_id)
            .cloned()
            .collect())
    }

    fn list_side_effect_records_for_workflow_run(
        &self,
        workflow_id: &WorkflowId,
        run_id: &WorkflowRunId,
    ) -> Result<Vec<SideEffectRecord>, WorkflowOsError> {
        Ok(self
            .records
            .iter()
            .filter(|record| record.workflow_id() == workflow_id && record.run_id() == run_id)
            .cloned()
            .collect())
    }
}

fn migration_export_canonical_digest(
    export: &LocalStateMigrationExport,
) -> Result<StateMigrationDigest, WorkflowOsError> {
    let mut hasher = Sha256::new();
    hash_serialized_migration_family(&mut hasher, "events", &export.events)?;
    hash_migration_family(
        &mut hasher,
        "idempotency_results",
        export
            .idempotency_results
            .iter()
            .map(|(storage_key, result)| {
                encode_json(result, "migration idempotency result")
                    .map(|payload| format!("{storage_key}:{payload}"))
            })
            .collect::<Result<Vec<_>, _>>()?,
    );
    hash_serialized_migration_family(
        &mut hasher,
        "approval_presentations",
        &export.approval_presentations,
    )?;
    hash_serialized_migration_family(&mut hasher, "projects", &export.projects)?;
    hash_serialized_migration_family(&mut hasher, "policy_audit", &export.policy_audit)?;
    hash_serialized_migration_family(&mut hasher, "adapter_audit", &export.adapter_audit)?;
    hash_serialized_migration_family(
        &mut hasher,
        "adapter_observability",
        &export.adapter_observability,
    )?;
    hash_serialized_migration_family(&mut hasher, "work_report_artifacts", &export.work_reports)?;
    hash_serialized_migration_family(&mut hasher, "side_effect_records", &export.side_effects)?;
    Ok(StateMigrationDigest::from_hasher(hasher))
}

fn hash_serialized_migration_family<T: Serialize>(
    hasher: &mut Sha256,
    family: &str,
    values: &[T],
) -> Result<(), WorkflowOsError> {
    let payloads = values
        .iter()
        .map(|value| encode_json(value, "migration verification record"))
        .collect::<Result<Vec<_>, _>>()?;
    hash_migration_family(hasher, family, payloads);
    Ok(())
}

fn hash_migration_family(hasher: &mut Sha256, family: &str, mut values: Vec<String>) {
    values.sort();
    hash_migration_frame(hasher, family.as_bytes());
    for value in values {
        hash_migration_frame(hasher, value.as_bytes());
    }
}

fn read_migration_payloads(
    connection: &Connection,
    query: &str,
) -> Result<Vec<String>, WorkflowOsError> {
    let mut statement = connection
        .prepare(query)
        .map_err(|_| verification_failed())?;
    let rows = statement
        .query_map([], |row| row.get::<_, String>(0))
        .map_err(|_| verification_failed())?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|_| verification_failed())
}

fn hash_migration_frame(hasher: &mut Sha256, value: &[u8]) {
    hasher.update((value.len() as u64).to_be_bytes());
    hasher.update(value);
}

fn sqlite_idempotency_storage_key(key: &IdempotencyKey) -> String {
    format!("{:x}", Sha256::digest(key.as_str().as_bytes()))
}

fn import_failed() -> WorkflowOsError {
    migration_runtime_error(
        "transaction.import_failed",
        "state migration import transaction failed",
    )
}

fn verification_failed() -> WorkflowOsError {
    migration_runtime_error(
        "verification.failed",
        "state migration destination verification failed",
    )
}

fn migration_runtime_error(suffix: &str, message: &str) -> WorkflowOsError {
    WorkflowOsError::new(
        crate::WorkflowOsErrorKind::InvalidState,
        format!("state.migration.{suffix}"),
        message,
    )
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

#[cfg(test)]
#[allow(clippy::expect_used)]
mod migration_transaction_tests {
    use std::fs;

    use super::*;
    use crate::{
        CorrelationId, EventId, EventSequenceNumber, SchemaVersion, SpecContentHash,
        StateMigrationDestinationId, StateMigrationId, StateMigrationWriterGuardCapability,
        WorkflowRunEventKind, WorkflowVersion,
    };

    fn migration_event() -> WorkflowRunEvent {
        WorkflowRunEvent {
            sequence_number: EventSequenceNumber::new(1).expect("sequence"),
            event_id: EventId::new("event-atomic-migration").expect("event id"),
            timestamp: Timestamp::parse_rfc3339("2026-07-29T00:00:00Z").expect("timestamp"),
            run_id: WorkflowRunId::new("run-atomic-migration").expect("run id"),
            workflow_id: WorkflowId::new("workflow/atomic-migration").expect("workflow id"),
            schema_version: SchemaVersion::new("workflowos.dev/v0").expect("schema"),
            workflow_version: WorkflowVersion::new("v0").expect("version"),
            spec_content_hash: SpecContentHash::from_text("atomic migration"),
            correlation_id: Some(
                CorrelationId::new("correlation-atomic-migration").expect("correlation"),
            ),
            actor: Some(ActorId::new("system/atomic-migration").expect("actor")),
            idempotency_key: None,
            kind: WorkflowRunEventKind::RunCreated {
                summary: None,
                immutable_run_bundle: None,
            },
        }
    }

    #[test]
    fn injected_precommit_failure_rolls_back_all_imported_rows() {
        let base = std::env::temp_dir().join(format!(
            "workflow-os-sqlite-import-rollback-{}",
            std::process::id()
        ));
        let source_path = base.join("source");
        let database_path = base.join("staging.sqlite3");
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(&base).expect("fixture root");
        let source = crate::LocalStateBackend::new(&source_path).expect("source");
        source
            .append_event(&migration_event())
            .expect("source event");
        let guard = source
            .try_acquire_exclusive_migration_guard()
            .expect("exclusive guard");
        let inventory = guard.inspect_migration_inventory().expect("inventory");
        let plan = StateMigrationPlan::new(
            StateMigrationId::new("migration/atomic-rollback").expect("migration id"),
            &inventory,
            StateMigrationDestinationId::new("sqlite/atomic-rollback").expect("destination id"),
            ADAPTER_SCHEMA_VERSION,
        )
        .expect("plan");
        let capability = StateMigrationWriterGuardCapability::local_filesystem_v1();
        let compatibility = StateMigrationWriterCompatibility::assess(
            crate::DurableStateBackendKind::LocalFilesystemPreview,
            Some(StateMigrationWriterProtocolVersion::V1),
            &capability,
            true,
        );
        let attempt = StateMigrationAttempt::new(
            &plan,
            &capability,
            &compatibility,
            StateMigrationImporterTransactionVersion::V1,
        )
        .expect("attempt");
        let export = guard.export_migration_records().expect("export");
        let backend = SqliteStateBackend {
            database_path: database_path.clone(),
            busy_timeout: DEFAULT_BUSY_TIMEOUT,
        };
        let mut connection = backend.migration_connection().expect("connection");
        SqliteStateBackend::prepare_or_resume_staging(&mut connection, &plan, &attempt)
            .expect("staging");

        let error = SqliteStateBackend::import_export_with_failure(&mut connection, &export, true)
            .expect_err("injected failure");

        assert_eq!(error.code(), "state.migration.transaction.import_failed");
        assert_eq!(table_count(&connection, "events").expect("event count"), 0);
        assert_eq!(
            read_migration_state(&connection).expect("state"),
            MIGRATION_STATE_IMPORTING_EMPTY
        );
        drop(guard);
        drop(connection);
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn canonical_verification_rejects_same_count_payload_tampering() {
        let base = std::env::temp_dir().join(format!(
            "workflow-os-sqlite-import-verification-{}",
            std::process::id()
        ));
        let source_path = base.join("source");
        let database_path = base.join("staging.sqlite3");
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(&base).expect("fixture root");
        let source = crate::LocalStateBackend::new(&source_path).expect("source");
        source
            .append_event(&migration_event())
            .expect("source event");
        let guard = source
            .try_acquire_exclusive_migration_guard()
            .expect("exclusive guard");
        let inventory = guard.inspect_migration_inventory().expect("inventory");
        let plan = StateMigrationPlan::new(
            StateMigrationId::new("migration/canonical-tamper").expect("migration id"),
            &inventory,
            StateMigrationDestinationId::new("sqlite/canonical-tamper").expect("destination id"),
            ADAPTER_SCHEMA_VERSION,
        )
        .expect("plan");
        let capability = StateMigrationWriterGuardCapability::local_filesystem_v1();
        let compatibility = StateMigrationWriterCompatibility::assess(
            crate::DurableStateBackendKind::LocalFilesystemPreview,
            Some(StateMigrationWriterProtocolVersion::V1),
            &capability,
            true,
        );
        let attempt = StateMigrationAttempt::new(
            &plan,
            &capability,
            &compatibility,
            StateMigrationImporterTransactionVersion::V1,
        )
        .expect("attempt");
        let export = guard.export_migration_records().expect("export");
        let backend = SqliteStateBackend {
            database_path: database_path.clone(),
            busy_timeout: DEFAULT_BUSY_TIMEOUT,
        };
        let mut connection = backend.migration_connection().expect("connection");
        SqliteStateBackend::prepare_or_resume_staging(&mut connection, &plan, &attempt)
            .expect("staging");
        SqliteStateBackend::import_export(&mut connection, &export).expect("import");
        connection
            .execute("UPDATE events SET payload = '{}'", [])
            .expect("tamper");

        let error = SqliteStateBackend::verify_staging(&connection, &export)
            .expect_err("same-count tampering must fail verification");

        assert_eq!(error.code(), "state.migration.verification.failed");
        assert_eq!(
            read_migration_state(&connection).expect("state"),
            MIGRATION_STATE_IMPORTED_UNVERIFIED
        );
        drop(guard);
        drop(connection);
        let _ = fs::remove_dir_all(&base);
    }
}
