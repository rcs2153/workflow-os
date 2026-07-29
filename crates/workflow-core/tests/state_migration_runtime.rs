#![allow(clippy::expect_used)]

//! Operational guarded filesystem-to-SQLite migration tests.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use workflow_core::{
    ActorId, CorrelationId, EventId, EventLogStore, EventSequenceNumber,
    FilesystemToSqliteMigrationInput, IdempotencyKey, IdempotencyResult, IdempotencyStore,
    IdempotencyWrite, LocalStateBackend, SchemaVersion, SpecContentHash, SqliteStateBackend,
    StateMigrationDestinationId, StateMigrationId, StateMigrationPlan, Timestamp, WorkflowId,
    WorkflowRunEvent, WorkflowRunEventKind, WorkflowRunId, WorkflowVersion,
};

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(1);

struct Fixture {
    root: PathBuf,
    database: PathBuf,
}

impl Fixture {
    fn new(label: &str) -> Self {
        let id = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "workflow-os-operational-migration-{label}-{}-{id}",
            std::process::id()
        ));
        let database = root.with_extension("sqlite3");
        let _ = fs::remove_dir_all(&root);
        cleanup_database(&database);
        Self { root, database }
    }

    fn source(&self) -> LocalStateBackend {
        LocalStateBackend::new(&self.root).expect("source backend")
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
        cleanup_database(&self.database);
    }
}

fn event(sequence: u64) -> WorkflowRunEvent {
    WorkflowRunEvent {
        sequence_number: EventSequenceNumber::new(sequence).expect("sequence"),
        event_id: EventId::new(format!("event-operational-migration-{sequence}"))
            .expect("event id"),
        timestamp: Timestamp::parse_rfc3339("2026-07-29T00:00:00Z").expect("timestamp"),
        run_id: WorkflowRunId::new("run-operational-migration").expect("run id"),
        workflow_id: WorkflowId::new("workflow/operational-migration").expect("workflow id"),
        schema_version: SchemaVersion::new("workflowos.dev/v0").expect("schema version"),
        workflow_version: WorkflowVersion::new("v0").expect("workflow version"),
        spec_content_hash: SpecContentHash::from_text("operational migration fixture"),
        correlation_id: Some(
            CorrelationId::new("correlation-operational-migration").expect("correlation"),
        ),
        actor: Some(ActorId::new("system/operational-migration").expect("actor")),
        idempotency_key: None,
        kind: if sequence == 1 {
            WorkflowRunEventKind::RunCreated {
                summary: None,
                immutable_run_bundle: None,
            }
        } else {
            WorkflowRunEventKind::RunValidated
        },
    }
}

fn plan(source: &LocalStateBackend) -> StateMigrationPlan {
    StateMigrationPlan::new(
        StateMigrationId::new("migration/operational-v1").expect("migration id"),
        &source
            .inspect_migration_inventory()
            .expect("compatible inventory"),
        StateMigrationDestinationId::new("sqlite/operational-staging").expect("destination id"),
        SqliteStateBackend::adapter_schema_version(),
    )
    .expect("migration plan")
}

fn stage(
    source: &LocalStateBackend,
    plan: &StateMigrationPlan,
    database: &Path,
) -> workflow_core::StateMigrationVerificationReceipt {
    SqliteStateBackend::stage_filesystem_migration(FilesystemToSqliteMigrationInput {
        source,
        plan,
        destination_path: database.to_path_buf(),
        verified_by: ActorId::new("user/migration-operator").expect("actor"),
        verified_at: Timestamp::parse_rfc3339("2026-07-29T00:30:00Z").expect("timestamp"),
        incompatible_older_writers_stopped: true,
    })
    .expect("verified inactive staging")
}

#[test]
fn guarded_migration_is_inactive_until_exact_receipt_activation() {
    let fixture = Fixture::new("activate");
    let source = fixture.source();
    source.append_event(&event(1)).expect("created");
    source.append_event(&event(2)).expect("validated");
    let key = IdempotencyKey::new("migration/idempotency").expect("key");
    source
        .record_idempotency_result(
            &key,
            IdempotencyResult {
                result_ref: "result/migrated".to_owned(),
            },
        )
        .expect("idempotency");
    let plan = plan(&source);

    let receipt = stage(&source, &plan, &fixture.database);
    let inactive = SqliteStateBackend::open(&fixture.database)
        .expect_err("ordinary runtime cannot open inactive staging");
    assert_eq!(inactive.code(), "state.sqlite.schema.recovery_required");
    assert_eq!(
        source
            .read_events(&event(1).run_id)
            .expect("source remains readable")
            .len(),
        2
    );

    let backend = SqliteStateBackend::activate_verified_migration(&fixture.database, &receipt)
        .expect("explicit activation");
    assert_eq!(
        backend
            .read_events(&event(1).run_id)
            .expect("migrated events"),
        vec![event(1), event(2)]
    );
    assert_eq!(
        backend
            .record_idempotency_result(
                &key,
                IdempotencyResult {
                    result_ref: "different".to_owned(),
                },
            )
            .expect("idempotency replay"),
        IdempotencyWrite::Duplicate(IdempotencyResult {
            result_ref: "result/migrated".to_owned(),
        })
    );
}

#[test]
fn exact_verified_staging_resumes_without_duplicate_import() {
    let fixture = Fixture::new("resume");
    let source = fixture.source();
    source.append_event(&event(1)).expect("created");
    source.append_event(&event(2)).expect("validated");
    let plan = plan(&source);

    let first = stage(&source, &plan, &fixture.database);
    let resumed = stage(&source, &plan, &fixture.database);

    assert_eq!(resumed, first);
    assert!(format!("{resumed:?}").contains("<redacted>"));
    assert!(!format!("{resumed:?}").contains(fixture.database.to_string_lossy().as_ref()));
}

#[test]
fn changed_source_and_mismatched_receipt_fail_closed_without_leakage() {
    let fixture = Fixture::new("fail-closed");
    let source = fixture.source();
    source.append_event(&event(1)).expect("created");
    let stale_plan = plan(&source);
    source.append_event(&event(2)).expect("validated");
    let changed =
        SqliteStateBackend::stage_filesystem_migration(FilesystemToSqliteMigrationInput {
            source: &source,
            plan: &stale_plan,
            destination_path: fixture.database.clone(),
            verified_by: ActorId::new("user/migration-operator").expect("actor"),
            verified_at: Timestamp::parse_rfc3339("2026-07-29T00:30:00Z").expect("timestamp"),
            incompatible_older_writers_stopped: true,
        })
        .expect_err("changed source rejected");
    assert_eq!(changed.code(), "state.migration.source.changed");
    assert!(!fixture.database.exists());

    let current_plan = plan(&source);
    let receipt = stage(&source, &current_plan, &fixture.database);
    let other = Fixture::new("other-receipt");
    let other_source = other.source();
    let other_plan = plan(&other_source);
    let other_receipt = stage(&other_source, &other_plan, &other.database);
    let mismatch =
        SqliteStateBackend::activate_verified_migration(&fixture.database, &other_receipt)
            .expect_err("mismatched receipt rejected");

    assert_eq!(
        mismatch.code(),
        "state.migration.activation.receipt_mismatch"
    );
    assert!(!mismatch
        .to_string()
        .contains(fixture.database.to_string_lossy().as_ref()));
    assert!(SqliteStateBackend::open(&fixture.database).is_err());
    assert_ne!(receipt, other_receipt);
}

#[test]
fn older_writer_assertion_is_required_and_errors_are_non_leaking() {
    let fixture = Fixture::new("writer-assertion");
    let source = fixture.source();
    let plan = plan(&source);
    let error = SqliteStateBackend::stage_filesystem_migration(FilesystemToSqliteMigrationInput {
        source: &source,
        plan: &plan,
        destination_path: fixture.database.clone(),
        verified_by: ActorId::new("user/migration-operator").expect("actor"),
        verified_at: Timestamp::parse_rfc3339("2026-07-29T00:30:00Z").expect("timestamp"),
        incompatible_older_writers_stopped: false,
    })
    .expect_err("assertion required");

    assert_eq!(error.code(), "state.migration.writer.compatibility.invalid");
    assert!(!error
        .to_string()
        .contains(fixture.database.to_string_lossy().as_ref()));
}

fn cleanup_database(path: &Path) {
    for candidate in [
        path.to_path_buf(),
        PathBuf::from(format!("{}-wal", path.display())),
        PathBuf::from(format!("{}-shm", path.display())),
    ] {
        let _ = fs::remove_file(candidate);
    }
}
