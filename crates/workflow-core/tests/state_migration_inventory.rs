#![allow(clippy::expect_used)]

//! Read-only filesystem state migration inventory contract tests.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use sha2::{Digest, Sha256};
use workflow_core::{
    ActorId, CorrelationId, EventId, EventLogStore, EventSequenceNumber, IdempotencyKey,
    IdempotencyResult, IdempotencyStore, LocalStateBackend, LockStore, SchemaVersion,
    SpecContentHash, StateMigrationDigest, StateMigrationDisposition, StateMigrationFindingCode,
    StateMigrationFindingSeverity, StateMigrationInventory, StateMigrationRecordCount,
    StateMigrationRecordFamily, Timestamp, WorkflowId, WorkflowRunEvent, WorkflowRunEventKind,
    WorkflowRunId, WorkflowVersion,
};

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(1);

struct Fixture {
    root: PathBuf,
}

impl Fixture {
    fn new(label: &str) -> Self {
        let id = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "workflow-os-state-migration-{label}-{}-{id}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);
        Self { root }
    }

    fn backend(&self) -> LocalStateBackend {
        LocalStateBackend::new(&self.root).expect("local state backend")
    }

    fn inspection_backend(&self) -> LocalStateBackend {
        LocalStateBackend::for_inspection(&self.root)
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn event(sequence: u64) -> WorkflowRunEvent {
    WorkflowRunEvent {
        sequence_number: EventSequenceNumber::new(sequence).expect("sequence"),
        event_id: EventId::new(format!("event-migration-{sequence}")).expect("event id"),
        timestamp: Timestamp::parse_rfc3339("2026-07-28T00:00:00Z").expect("timestamp"),
        run_id: WorkflowRunId::new("run-migration-inventory").expect("run id"),
        workflow_id: WorkflowId::new("workflow/migration-inventory").expect("workflow id"),
        schema_version: SchemaVersion::new("workflowos.dev/v0").expect("schema version"),
        workflow_version: WorkflowVersion::new("v0").expect("workflow version"),
        spec_content_hash: SpecContentHash::from_text("migration inventory fixture"),
        correlation_id: Some(
            CorrelationId::new("correlation-migration-inventory").expect("correlation"),
        ),
        actor: Some(ActorId::new("system/migration-inventory").expect("actor")),
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

fn populate_known_state(backend: &LocalStateBackend) {
    backend.append_event(&event(1)).expect("event appended");
    backend
        .record_idempotency_result(
            &IdempotencyKey::new("migration/inventory").expect("idempotency key"),
            IdempotencyResult {
                result_ref: "result/migration-inventory".to_owned(),
            },
        )
        .expect("idempotency recorded");
}

fn hash(value: &str) -> String {
    format!("{:x}", Sha256::digest(value.as_bytes()))
}

fn tree_paths(root: &Path) -> Vec<PathBuf> {
    fn visit(root: &Path, paths: &mut Vec<PathBuf>) {
        let Ok(entries) = fs::read_dir(root) else {
            return;
        };
        for entry in entries {
            let path = entry.expect("directory entry").path();
            paths.push(path.clone());
            if path.is_dir() {
                visit(&path, paths);
            }
        }
    }

    let mut paths = Vec::new();
    visit(root, &mut paths);
    paths.sort();
    paths
}

#[test]
fn absent_source_is_inspected_without_creating_files() {
    let fixture = Fixture::new("absent");
    assert!(!fixture.root.exists());

    let inventory = fixture
        .inspection_backend()
        .inspect_migration_inventory()
        .expect("inventory");

    assert!(inventory.is_empty());
    assert!(inventory.is_healthy());
    assert!(inventory.is_migration_compatible());
    assert!(inventory.source_fingerprint().is_some());
    assert_eq!(
        inventory.record_counts().len(),
        StateMigrationRecordFamily::all().len()
    );
    assert!(!fixture.root.exists(), "inspection must remain read-only");
}

#[test]
fn known_state_has_complete_classification_and_path_independent_fingerprint() {
    let first = Fixture::new("known-first");
    let second = Fixture::new("known-second");
    let first_backend = first.backend();
    let second_backend = second.backend();
    populate_known_state(&first_backend);
    populate_known_state(&second_backend);

    let first_inventory = first_backend
        .inspect_migration_inventory()
        .expect("first inventory");
    let second_inventory = second_backend
        .inspect_migration_inventory()
        .expect("second inventory");

    assert!(
        first_inventory.is_migration_compatible(),
        "findings: {:?}",
        first_inventory.findings()
    );
    assert_eq!(
        first_inventory.source_fingerprint(),
        second_inventory.source_fingerprint(),
        "source paths must not affect semantic identity"
    );
    assert_eq!(
        first_inventory
            .record_count(StateMigrationRecordFamily::WorkflowEvents)
            .expect("events")
            .count(),
        1
    );
    assert_eq!(
        first_inventory
            .record_count(StateMigrationRecordFamily::IdempotencyResults)
            .expect("idempotency")
            .count(),
        1
    );
    for family in StateMigrationRecordFamily::all() {
        assert_eq!(
            first_inventory
                .record_count(*family)
                .expect("all families represented")
                .disposition(),
            family.disposition()
        );
    }
}

#[test]
fn canonical_idempotency_storage_identity_changes_source_fingerprint() {
    let first = Fixture::new("idempotency-first");
    let second = Fixture::new("idempotency-second");
    let first_backend = first.backend();
    let second_backend = second.backend();
    let result = IdempotencyResult {
        result_ref: "result/shared".to_owned(),
    };
    first_backend
        .record_idempotency_result(
            &IdempotencyKey::new("migration/first-key").expect("first key"),
            result.clone(),
        )
        .expect("first result");
    second_backend
        .record_idempotency_result(
            &IdempotencyKey::new("migration/second-key").expect("second key"),
            result,
        )
        .expect("second result");

    let first_inventory = first_backend
        .inspect_migration_inventory()
        .expect("first inventory");
    let second_inventory = second_backend
        .inspect_migration_inventory()
        .expect("second inventory");

    assert_ne!(
        first_inventory.source_fingerprint(),
        second_inventory.source_fingerprint(),
        "canonical idempotency identity must affect the source fingerprint"
    );
}

#[test]
fn inspection_does_not_modify_source_or_create_destination_state() {
    let fixture = Fixture::new("read-only");
    let backend = fixture.backend();
    populate_known_state(&backend);
    let before = tree_paths(&fixture.root);

    backend
        .inspect_migration_inventory()
        .expect("read-only inventory");

    assert_eq!(tree_paths(&fixture.root), before);
    assert!(!fixture.root.join("workflow-os.sqlite3").exists());
}

#[test]
fn unknown_empty_directory_warns_but_unknown_nonempty_state_blocks() {
    let warning_fixture = Fixture::new("unknown-empty");
    let warning_backend = warning_fixture.backend();
    fs::create_dir(warning_fixture.root.join("future-family")).expect("empty unknown directory");
    let warning_inventory = warning_backend
        .inspect_migration_inventory()
        .expect("warning inventory");

    assert!(warning_inventory.is_migration_compatible());
    assert!(warning_inventory.source_fingerprint().is_some());
    assert!(warning_inventory.findings().iter().any(|finding| {
        finding.severity() == StateMigrationFindingSeverity::Warning
            && finding.code() == StateMigrationFindingCode::UnknownEmptyDirectory
    }));

    let blocker_fixture = Fixture::new("unknown-nonempty");
    let blocker_backend = blocker_fixture.backend();
    fs::create_dir(blocker_fixture.root.join("future-family")).expect("unknown directory");
    fs::write(
        blocker_fixture.root.join("future-family").join("record"),
        b"opaque",
    )
    .expect("unknown record");
    let blocker_inventory = blocker_backend
        .inspect_migration_inventory()
        .expect("blocker inventory");

    assert!(!blocker_inventory.is_migration_compatible());
    assert!(blocker_inventory.source_fingerprint().is_none());
    assert!(blocker_inventory
        .findings()
        .iter()
        .any(|finding| finding.code() == StateMigrationFindingCode::UnknownRecordFamily));
}

#[test]
fn corrupt_known_record_fails_closed_without_leaking_path_or_payload() {
    let fixture = Fixture::new("corrupt");
    let backend = fixture.backend();
    let secret = "ghp_super_secret_migration_value";
    fs::write(
        fixture
            .root
            .join("idempotency")
            .join(format!("{}.json", hash("corrupt"))),
        format!(r#"{{"result_ref":"{secret}""#),
    )
    .expect("corrupt record");

    let inventory = backend
        .inspect_migration_inventory()
        .expect("bounded inventory");
    let debug = format!("{inventory:?}");
    let serialized = serde_json::to_string(&inventory).expect("serialized");

    assert!(!inventory.is_migration_compatible());
    assert!(inventory.source_fingerprint().is_none());
    assert!(!debug.contains(secret));
    assert!(!serialized.contains(secret));
    assert!(!debug.contains(fixture.root.to_string_lossy().as_ref()));
    assert!(!serialized.contains(fixture.root.to_string_lossy().as_ref()));
}

#[test]
fn live_lock_is_counted_excluded_and_blocks_compatibility() {
    let fixture = Fixture::new("lock");
    let backend = fixture.backend();
    backend
        .acquire_lock(
            "migration/inventory",
            &ActorId::new("worker/migration").expect("actor"),
        )
        .expect("lock");

    let inventory = backend.inspect_migration_inventory().expect("inventory");
    let locks = inventory
        .record_count(StateMigrationRecordFamily::LocalLocks)
        .expect("locks");

    assert_eq!(locks.count(), 1);
    assert_eq!(
        locks.disposition(),
        StateMigrationDisposition::EphemeralExclude
    );
    assert!(!inventory.is_migration_compatible());
    assert!(inventory
        .findings()
        .iter()
        .any(|finding| finding.code() == StateMigrationFindingCode::LockPresent));
}

#[test]
fn dangling_side_effect_index_blocks_inventory_without_exposing_identity() {
    let fixture = Fixture::new("dangling-index");
    let backend = fixture.backend();
    let secret_identity = "side-effect-secret-token";
    fs::write(
        fixture
            .root
            .join("side_effects")
            .join("ids")
            .join(format!("{}.json", hash(secret_identity))),
        r#"{"run_id":"run-missing-side-effect"}"#,
    )
    .expect("dangling index");

    let inventory = backend
        .inspect_migration_inventory()
        .expect("bounded inventory");
    let output = serde_json::to_string(&inventory).expect("serialized");

    assert!(!inventory.is_migration_compatible());
    assert!(inventory.source_fingerprint().is_none());
    assert!(inventory.findings().iter().any(|finding| {
        finding.code() == StateMigrationFindingCode::IndexInconsistent
            && finding.family() == Some(StateMigrationRecordFamily::SideEffectIdIndexes)
    }));
    assert!(!output.contains(secret_identity));
    assert!(!output.contains("run-missing-side-effect"));
}

#[test]
fn immutable_bundle_files_are_retained_as_bounded_companion_state() {
    let fixture = Fixture::new("bundle");
    let backend = fixture.backend();
    let manifests = fixture.root.join("immutable-run-bundles").join("manifests");
    fs::create_dir_all(&manifests).expect("manifests directory");
    fs::write(
        manifests.join(format!("{}.json", hash("bundle"))),
        br#"{"bundle":"opaque-companion"}"#,
    )
    .expect("bundle");

    let inventory = backend.inspect_migration_inventory().expect("inventory");
    let bundles = inventory
        .record_count(StateMigrationRecordFamily::ImmutableRunBundles)
        .expect("bundles");

    assert_eq!(bundles.count(), 1);
    assert_eq!(
        bundles.disposition(),
        StateMigrationDisposition::CompanionPreserve
    );
    assert!(inventory.is_migration_compatible());
}

#[test]
fn inventory_serde_revalidates_derived_posture_and_errors_are_bounded() {
    let fixture = Fixture::new("serde");
    let backend = fixture.backend();
    populate_known_state(&backend);
    let inventory = backend.inspect_migration_inventory().expect("inventory");
    let serialized = serde_json::to_value(&inventory).expect("serialized");
    let round_trip: StateMigrationInventory =
        serde_json::from_value(serialized.clone()).expect("round trip");
    assert_eq!(round_trip, inventory);

    let mut tampered = serialized;
    tampered["healthy"] = serde_json::Value::Bool(false);
    tampered["unrecognized"] =
        serde_json::Value::String("authorization: Bearer secret-migration-token".to_owned());
    let error = serde_json::from_value::<StateMigrationInventory>(tampered)
        .expect_err("tampered derived state fails closed")
        .to_string();

    assert!(!error.contains("secret-migration-token"));
    assert!(error.contains("unknown field") || error.contains("invalid"));
}

#[cfg(unix)]
#[test]
fn symlink_inside_state_boundary_is_rejected() {
    use std::os::unix::fs::symlink;

    let fixture = Fixture::new("symlink");
    let backend = fixture.backend();
    let target = fixture.root.join("target");
    fs::create_dir(&target).expect("target");
    symlink(&target, fixture.root.join("unexpected-link")).expect("symlink");

    let inventory = backend.inspect_migration_inventory().expect("inventory");

    assert!(!inventory.is_migration_compatible());
    assert!(inventory
        .findings()
        .iter()
        .any(|finding| finding.code() == StateMigrationFindingCode::SymlinkRejected));
}

#[test]
fn model_rejects_invalid_digest_disposition_and_incomplete_family_coverage() {
    let secret = "ghp_secret_digest_value";
    let digest_error = StateMigrationDigest::new(secret).expect_err("invalid digest");
    assert_eq!(digest_error.code(), "state.migration.digest.invalid");
    assert!(!digest_error.to_string().contains(secret));

    let disposition_error = StateMigrationRecordCount::new(
        StateMigrationRecordFamily::WorkflowEvents,
        StateMigrationDisposition::ProjectionRebuild,
        0,
        None,
    )
    .expect_err("mismatched disposition");
    assert_eq!(
        disposition_error.code(),
        "state.migration.record_count.disposition_mismatch"
    );

    let incomplete_error =
        StateMigrationInventory::new(Vec::new(), Vec::new(), true).expect_err("incomplete");
    assert_eq!(
        incomplete_error.code(),
        "state.migration.inventory.incomplete"
    );
}

#[test]
fn missing_family_digest_cannot_be_reported_as_migration_compatible() {
    let records = StateMigrationRecordFamily::all()
        .iter()
        .copied()
        .map(|family| {
            StateMigrationRecordCount::new(
                family,
                family.disposition(),
                0,
                if family == StateMigrationRecordFamily::WorkflowEvents {
                    None
                } else {
                    Some(StateMigrationDigest::new("0".repeat(64)).expect("valid bounded digest"))
                },
            )
            .expect("record count")
        })
        .collect();

    let inventory =
        StateMigrationInventory::new(records, Vec::new(), true).expect("complete inventory");

    assert!(inventory.is_healthy());
    assert!(!inventory.is_migration_compatible());
    assert!(inventory.source_fingerprint().is_none());
}
