#![allow(clippy::expect_used)]

//! Focused conformance and failure-boundary tests for the opt-in `SQLite` backend.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Barrier};
use std::thread;

use rusqlite::{params, Connection};
use workflow_core::{
    run_durable_state_conformance, ActorId, CorrelationId, DurableStateBackendKind,
    DurableStateCapability, DurableStateConformanceFixture, DurableStateConformanceOutcome,
    DurableStateSchemaPosture, DurableStateSupport, DurableStateTransactionKind, EventId,
    EventLogStore, EventSequenceNumber, IdempotencyKey, SchemaVersion, SpecContentHash,
    SqliteStateBackend, StateBackend, Timestamp, WorkflowId, WorkflowRunEvent,
    WorkflowRunEventKind, WorkflowRunId, WorkflowVersion,
};

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(1);

struct Fixture {
    path: PathBuf,
    backend: SqliteStateBackend,
    created: WorkflowRunEvent,
    validated: WorkflowRunEvent,
}

impl Fixture {
    fn new() -> Self {
        let id = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "workflow-os-sqlite-state-{}-{id}.sqlite3",
            std::process::id()
        ));
        cleanup_database(&path);

        let run_id = WorkflowRunId::new(format!("run-sqlite-contract-{id}")).expect("run id");
        let workflow_id =
            WorkflowId::new(format!("workflow/sqlite-contract-{id}")).expect("workflow id");
        let schema_version = SchemaVersion::new("workflowos.dev/v0").expect("schema version");
        let workflow_version = WorkflowVersion::new("v0").expect("workflow version");
        let spec_content_hash = SpecContentHash::from_text("sqlite state conformance fixture");
        let event = |sequence, event_id: &str, kind| WorkflowRunEvent {
            sequence_number: EventSequenceNumber::new(sequence).expect("sequence"),
            event_id: EventId::new(format!("event-sqlite-{id}-{event_id}")).expect("event id"),
            timestamp: Timestamp::parse_rfc3339("2026-01-01T00:00:00Z").expect("timestamp"),
            run_id: run_id.clone(),
            workflow_id: workflow_id.clone(),
            schema_version: schema_version.clone(),
            workflow_version: workflow_version.clone(),
            spec_content_hash: spec_content_hash.clone(),
            correlation_id: Some(
                CorrelationId::new(format!("correlation-sqlite-{id}")).expect("correlation"),
            ),
            actor: Some(ActorId::new("system/sqlite-conformance").expect("actor")),
            idempotency_key: None,
            kind,
        };

        Self {
            backend: SqliteStateBackend::open(&path).expect("SQLite backend"),
            path,
            created: event(
                1,
                "created",
                WorkflowRunEventKind::RunCreated {
                    summary: None,
                    immutable_run_bundle: None,
                },
            ),
            validated: event(2, "validated", WorkflowRunEventKind::RunValidated),
        }
    }

    fn conformance_fixture(&self) -> DurableStateConformanceFixture {
        DurableStateConformanceFixture::new(
            self.created.clone(),
            self.validated.clone(),
            IdempotencyKey::new(format!(
                "sqlite-conformance/{}",
                self.created.run_id.as_str()
            ))
            .expect("idempotency key"),
            ActorId::new("worker/sqlite-conformance").expect("actor"),
        )
        .expect("fixture")
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        cleanup_database(&self.path);
    }
}

#[test]
fn sqlite_backend_passes_common_conformance_without_overclaiming() {
    let fixture = Fixture::new();

    let report = run_durable_state_conformance(&fixture.backend, &fixture.conformance_fixture())
        .expect("SQLite conformance passes");
    let contract = report.contract();

    assert_eq!(
        contract.backend_kind(),
        DurableStateBackendKind::EmbeddedSqlite
    );
    assert_eq!(
        contract.schema().adapter_schema_version(),
        Some(2),
        "SQLite schema version is explicit"
    );
    assert_eq!(
        contract.schema().posture(),
        DurableStateSchemaPosture::Ready
    );
    assert_eq!(
        contract.transaction_support(DurableStateTransactionKind::AppendRunEvent),
        DurableStateSupport::Supported
    );
    assert!(DurableStateTransactionKind::all()
        .iter()
        .copied()
        .filter(|kind| *kind != DurableStateTransactionKind::AppendRunEvent)
        .all(|kind| contract.transaction_support(kind) == DurableStateSupport::Unsupported));
    assert!(contract.supports_capability(DurableStateCapability::OrderedEventAppend));
    assert!(!contract.supports_capability(DurableStateCapability::SharedWorkerConcurrency));
    assert!(!contract.supports_capability(DurableStateCapability::ManagedSchemaMigration));

    assert_eq!(report.results().len(), 22);
    assert_eq!(
        report
            .results()
            .iter()
            .filter(|result| result.outcome() == DurableStateConformanceOutcome::Passed)
            .count(),
        10
    );
    assert_eq!(
        report
            .results()
            .iter()
            .filter(|result| result.outcome() == DurableStateConformanceOutcome::Unsupported)
            .count(),
        12
    );
}

#[test]
fn sqlite_backend_reopens_with_ordered_events_and_wal_posture() {
    let fixture = Fixture::new();
    fixture
        .backend
        .append_event(&fixture.created)
        .expect("created event");
    fixture
        .backend
        .append_event(&fixture.validated)
        .expect("validated event");

    let reopened = SqliteStateBackend::open(&fixture.path).expect("reopen backend");
    let events = reopened
        .read_events(&fixture.created.run_id)
        .expect("read reopened events");
    let health = reopened.health_check().expect("health check");
    let connection = Connection::open(&fixture.path).expect("inspect SQLite metadata");
    let journal_mode: String = connection
        .query_row("PRAGMA journal_mode", [], |row| row.get(0))
        .expect("journal mode");

    assert_eq!(
        events,
        vec![fixture.created.clone(), fixture.validated.clone()]
    );
    assert!(health.healthy);
    assert_eq!(health.backend, "embedded_sqlite");
    assert_eq!(journal_mode, "wal");
}

#[test]
fn sqlite_backend_serializes_competing_event_appends() {
    let fixture = Fixture::new();
    fixture
        .backend
        .append_event(&fixture.created)
        .expect("created event");

    let first_backend = fixture.backend.clone();
    let second_backend = SqliteStateBackend::open(&fixture.path).expect("second connection");
    let mut first_event = fixture.validated.clone();
    first_event.event_id = EventId::new("event-sqlite-concurrent-first").expect("event id");
    let mut second_event = fixture.validated.clone();
    second_event.event_id = EventId::new("event-sqlite-concurrent-second").expect("event id");
    let barrier = Arc::new(Barrier::new(3));

    let first = spawn_append(first_backend, first_event, Arc::clone(&barrier));
    let second = spawn_append(second_backend, second_event, Arc::clone(&barrier));
    barrier.wait();
    let results = [
        first.join().expect("first append"),
        second.join().expect("second append"),
    ];

    assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
    let error = results
        .iter()
        .find_map(|result| result.as_ref().err())
        .expect("one competing append fails");
    assert_eq!(error.code(), "state.event.duplicate_sequence");
    assert_eq!(
        fixture
            .backend
            .read_events(&fixture.created.run_id)
            .expect("ordered history")
            .len(),
        2
    );
}

#[test]
fn sqlite_backend_rejects_newer_and_incomplete_schema_without_leakage() {
    let fixture = Fixture::new();
    let secret = "secret-schema-token-marker";
    let connection = Connection::open(&fixture.path).expect("open fixture database");
    connection
        .pragma_update(None, "user_version", 3)
        .expect("set newer schema");
    drop(connection);

    let newer = SqliteStateBackend::open(&fixture.path).expect_err("newer schema rejected");
    assert_eq!(newer.code(), "state.sqlite.schema.incompatible");
    assert!(!newer.to_string().contains(secret));
    assert!(!newer
        .to_string()
        .contains(fixture.path.to_string_lossy().as_ref()));

    let connection = Connection::open(&fixture.path).expect("open fixture database");
    connection
        .pragma_update(None, "user_version", 2)
        .expect("restore schema version");
    connection
        .execute(
            "UPDATE schema_metadata SET migration_state = ?1 WHERE singleton = 1",
            params![secret],
        )
        .expect("mark incomplete schema");
    drop(connection);

    let incomplete =
        SqliteStateBackend::open(&fixture.path).expect_err("incomplete schema rejected");
    assert_eq!(incomplete.code(), "state.sqlite.schema.recovery_required");
    assert!(!incomplete.to_string().contains(secret));
    assert!(!incomplete
        .to_string()
        .contains(fixture.path.to_string_lossy().as_ref()));
}

#[test]
fn sqlite_backend_requires_and_performs_explicit_v1_to_v2_upgrade() {
    let fixture = Fixture::new();
    fixture
        .backend
        .append_event(&fixture.created)
        .expect("seed V1-compatible event");
    downgrade_fixture_to_v1(&fixture.path);

    let required = SqliteStateBackend::open(&fixture.path).expect_err("upgrade is explicit");
    assert_eq!(required.code(), "state.sqlite.schema.upgrade_required");

    let upgraded =
        SqliteStateBackend::upgrade_authorized_execution_continuity_v1_to_v2(&fixture.path)
            .expect("upgrade exact V1 database");
    let reopened = SqliteStateBackend::open(&fixture.path).expect("reopen upgraded database");
    assert_eq!(
        reopened
            .read_events(&fixture.created.run_id)
            .expect("preserved event"),
        vec![fixture.created.clone()]
    );
    let connection = Connection::open(&fixture.path).expect("inspect upgraded database");
    let version: u32 = connection
        .pragma_query_value(None, "user_version", |row| row.get(0))
        .expect("schema version");
    let trusted_time_rows: u32 = connection
        .query_row(
            "SELECT COUNT(*) FROM continuity_trusted_time
             WHERE singleton_id = 1 AND posture = 'unobserved'
               AND eligibility = 'live_state_eligible'",
            [],
            |row| row.get(0),
        )
        .expect("trusted time singleton");
    assert_eq!(version, 2);
    assert_eq!(trusted_time_rows, 1);

    SqliteStateBackend::upgrade_authorized_execution_continuity_v1_to_v2(&fixture.path)
        .expect("upgrade is idempotent for exact V2");
    assert_eq!(
        upgraded
            .read_events(&fixture.created.run_id)
            .expect("upgraded handle remains valid"),
        vec![fixture.created.clone()]
    );
}

#[test]
fn sqlite_backend_serializes_concurrent_v1_to_v2_upgraders() {
    let fixture = Fixture::new();
    fixture
        .backend
        .append_event(&fixture.created)
        .expect("seed V1-compatible event");
    downgrade_fixture_to_v1(&fixture.path);

    let barrier = Arc::new(Barrier::new(3));
    let handles = (0..2)
        .map(|_| {
            let path = fixture.path.clone();
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                barrier.wait();
                SqliteStateBackend::upgrade_authorized_execution_continuity_v1_to_v2(path)
            })
        })
        .collect::<Vec<_>>();
    barrier.wait();

    for handle in handles {
        handle
            .join()
            .expect("upgrader thread")
            .expect("concurrent upgrader converges on exact V2");
    }

    let reopened = SqliteStateBackend::open(&fixture.path).expect("reopen upgraded database");
    assert_eq!(
        reopened
            .read_events(&fixture.created.run_id)
            .expect("preserved event"),
        vec![fixture.created.clone()]
    );
    let connection = Connection::open(&fixture.path).expect("inspect upgraded database");
    let version: u32 = connection
        .pragma_query_value(None, "user_version", |row| row.get(0))
        .expect("schema version");
    let trusted_time_rows: u32 = connection
        .query_row(
            "SELECT COUNT(*) FROM continuity_trusted_time WHERE singleton_id = 1",
            [],
            |row| row.get(0),
        )
        .expect("trusted time singleton");
    assert_eq!(version, 2);
    assert_eq!(trusted_time_rows, 1);
}

#[test]
fn sqlite_backend_v1_upgrade_fails_closed_and_rolls_back() {
    let fixture = Fixture::new();
    downgrade_fixture_to_v1(&fixture.path);
    let secret = "secret-v1-checksum-marker";
    let connection = Connection::open(&fixture.path).expect("open V1 fixture");
    connection
        .execute(
            "UPDATE schema_metadata SET checksum = ?1 WHERE singleton = 1",
            params![secret],
        )
        .expect("corrupt V1 checksum");
    drop(connection);

    let error = SqliteStateBackend::upgrade_authorized_execution_continuity_v1_to_v2(&fixture.path)
        .expect_err("mismatched V1 is rejected");
    assert_eq!(error.code(), "state.sqlite.schema.recovery_required");
    assert!(!error.to_string().contains(secret));
    assert!(!error
        .to_string()
        .contains(fixture.path.to_string_lossy().as_ref()));

    let connection = Connection::open(&fixture.path).expect("inspect rollback");
    let version: u32 = connection
        .pragma_query_value(None, "user_version", |row| row.get(0))
        .expect("schema version remains V1");
    let continuity_table_count: u32 = connection
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master
             WHERE type = 'table' AND name = 'continuity_windows'",
            [],
            |row| row.get(0),
        )
        .expect("continuity table absence");
    assert_eq!(version, 1);
    assert_eq!(continuity_table_count, 0);
}

#[test]
fn sqlite_backend_upgrade_does_not_create_a_missing_database() {
    let id = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "workflow-os-sqlite-missing-upgrade-{}-{id}.sqlite3",
        std::process::id()
    ));
    cleanup_database(&path);

    let error = SqliteStateBackend::upgrade_authorized_execution_continuity_v1_to_v2(&path)
        .expect_err("missing upgrade target is rejected");
    assert_eq!(error.code(), "state.sqlite.open.failed");
    assert!(!path.exists());
}

#[test]
fn sqlite_backend_rejects_nonempty_unmanaged_version_zero_database() {
    let id = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "workflow-os-sqlite-unmanaged-{}-{id}.sqlite3",
        std::process::id()
    ));
    cleanup_database(&path);
    let connection = Connection::open(&path).expect("create unmanaged database");
    connection
        .execute("CREATE TABLE unrelated_data (id INTEGER PRIMARY KEY)", [])
        .expect("create unrelated object");
    drop(connection);

    let error = SqliteStateBackend::open(&path).expect_err("unmanaged database rejected");
    assert_eq!(error.code(), "state.sqlite.schema.nonempty_unmanaged");
    let connection = Connection::open(&path).expect("inspect unmanaged database");
    let unrelated_rows: u32 = connection
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master
             WHERE type = 'table' AND name = 'unrelated_data'",
            [],
            |row| row.get(0),
        )
        .expect("unrelated object retained");
    assert_eq!(unrelated_rows, 1);
    drop(connection);
    cleanup_database(&path);
}

#[test]
fn sqlite_backend_rejects_physical_v2_schema_drift() {
    let fixture = Fixture::new();
    let connection = Connection::open(&fixture.path).expect("open fixture database");
    connection
        .execute("DROP INDEX continuity_one_active_window", [])
        .expect("remove required index");
    drop(connection);

    let error = SqliteStateBackend::open(&fixture.path).expect_err("schema drift rejected");
    assert_eq!(error.code(), "state.sqlite.schema.recovery_required");
    assert!(!error
        .to_string()
        .contains(fixture.path.to_string_lossy().as_ref()));
}

#[test]
fn sqlite_backend_only_advertises_upgrade_for_exact_ready_v1() {
    let fixture = Fixture::new();
    downgrade_fixture_to_v1(&fixture.path);
    let connection = Connection::open(&fixture.path).expect("open V1 fixture");
    connection
        .execute("DROP INDEX adapter_audit_run", [])
        .expect("remove V1 index");
    drop(connection);

    let error = SqliteStateBackend::open(&fixture.path).expect_err("partial V1 rejected");
    assert_eq!(error.code(), "state.sqlite.schema.recovery_required");
}

#[test]
fn sqlite_backend_reopens_managed_restore_as_mutation_ineligible() {
    let fixture = Fixture::new();
    let connection = Connection::open(&fixture.path).expect("open fixture database");
    connection
        .execute(
            "UPDATE continuity_trusted_time
             SET eligibility = 'restore_unverified' WHERE singleton_id = 1",
            [],
        )
        .expect("mark managed restore posture");
    drop(connection);

    SqliteStateBackend::open(&fixture.path).expect("restore posture remains inspectable");
}

#[test]
fn sqlite_backend_detects_corrupt_payload_without_echoing_it() {
    let fixture = Fixture::new();
    fixture
        .backend
        .append_event(&fixture.created)
        .expect("created event");
    let secret = "secret-provider-payload-marker";
    let connection = Connection::open(&fixture.path).expect("open fixture database");
    connection
        .execute(
            "UPDATE events SET payload = ?1 WHERE event_id = ?2",
            params![secret, fixture.created.event_id.as_str()],
        )
        .expect("corrupt payload");
    drop(connection);

    let read_error = fixture
        .backend
        .read_events(&fixture.created.run_id)
        .expect_err("corrupt payload rejected");
    let health = fixture.backend.health_check().expect("health result");

    assert_eq!(read_error.code(), "state.sqlite.record.corrupt");
    assert!(!read_error.to_string().contains(secret));
    assert!(!health.healthy);
    assert!(!format!("{:?}", fixture.backend).contains(secret));
    assert!(!format!("{:?}", fixture.backend).contains(fixture.path.to_string_lossy().as_ref()));
}

#[test]
fn sqlite_backend_health_rejects_relational_identity_drift() {
    let fixture = Fixture::new();
    fixture
        .backend
        .append_event(&fixture.created)
        .expect("created event");
    let drifted_run_id = WorkflowRunId::new("run-sqlite-relational-drift").expect("drifted run id");
    let connection = Connection::open(&fixture.path).expect("open fixture database");
    connection
        .execute(
            "UPDATE events SET run_id = ?1 WHERE event_id = ?2",
            params![drifted_run_id.as_str(), fixture.created.event_id.as_str()],
        )
        .expect("corrupt relational identity");
    drop(connection);

    let read_error = fixture
        .backend
        .read_events(&drifted_run_id)
        .expect_err("relational identity drift rejected during authoritative read");
    let health = fixture.backend.health_check().expect("health result");

    assert_eq!(read_error.code(), "state.sqlite.record.identity_mismatch");
    assert!(!health.healthy);
    assert!(!format!("{health:?}").contains(fixture.path.to_string_lossy().as_ref()));
}

fn spawn_append(
    backend: SqliteStateBackend,
    event: WorkflowRunEvent,
    barrier: Arc<Barrier>,
) -> thread::JoinHandle<Result<(), workflow_core::WorkflowOsError>> {
    thread::spawn(move || {
        barrier.wait();
        backend.append_event(&event)
    })
}

fn cleanup_database(path: &Path) {
    for candidate in [
        path.to_path_buf(),
        PathBuf::from(format!("{}-wal", path.display())),
        PathBuf::from(format!("{}-shm", path.display())),
    ] {
        if candidate.exists() {
            fs::remove_file(candidate).expect("fixture cleanup");
        }
    }
}

fn downgrade_fixture_to_v1(path: &Path) {
    let connection = Connection::open(path).expect("open fixture for V1 downgrade");
    connection
        .execute_batch(
            "PRAGMA foreign_keys = OFF;
             DROP TABLE continuity_operations;
             DROP TABLE continuity_directives;
             DROP TABLE continuity_waits;
             DROP TABLE continuity_yields;
             DROP TABLE continuity_attempts;
             DROP TABLE continuity_windows;
             DROP TABLE continuity_trusted_time;
             UPDATE schema_metadata
             SET schema_version = 1,
                 migration_state = 'ready',
                 checksum = 'workflow-os-sqlite-state-v1'
             WHERE singleton = 1;
             PRAGMA user_version = 1;",
        )
        .expect("construct exact V1 fixture");
}
