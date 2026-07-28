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
        Some(1),
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
        .pragma_update(None, "user_version", 2)
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
        .pragma_update(None, "user_version", 1)
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
