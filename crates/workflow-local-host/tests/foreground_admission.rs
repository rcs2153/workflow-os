#![allow(clippy::expect_used)]
#![doc = "End-to-end proof for the unpublished foreground admission binary."]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

use workflow_core::{
    ImmutableRunBundleId, ImmutableRunBundleStore, SqliteStateBackend, StateBackend, Timestamp,
    WorkflowRunId, WorkflowRunStatus,
};

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(1);

struct Fixture {
    root: PathBuf,
    database: PathBuf,
    npm: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let id = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "workflow-os-local-host-admission-{}-{id}",
            std::process::id()
        ));
        fs::create_dir_all(&root).expect("fixture root");
        let npm = root.join("npm");
        fs::write(&npm, "#!/bin/sh\nexit 0\n").expect("fake npm");
        let mut permissions = fs::metadata(&npm).expect("fake npm metadata").permissions();
        permissions.set_mode(0o700);
        fs::set_permissions(&npm, permissions).expect("fake npm executable");
        Self {
            database: root.join("state.sqlite3"),
            npm,
            root,
        }
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn foreground_binary_admits_executes_and_persists_one_docs_check() {
    let fixture = Fixture::new();
    let repository_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("repository root");
    let governance_root = repository_root.join("dogfood/workflow-os-self-governance");
    let observed_at = Timestamp::now_utc();
    let expires_at = Timestamp::from_offset_date_time(
        observed_at.as_offset_date_time() + time::Duration::minutes(5),
    );
    let observed_at_text = observed_at.to_rfc3339();
    let expires_at_text = expires_at.to_rfc3339();
    let run_id = WorkflowRunId::new(format!(
        "run-local-host-admission-{}",
        NEXT_FIXTURE.load(Ordering::Relaxed)
    ))
    .expect("run id");
    let bundle_id = ImmutableRunBundleId::new(format!(
        "bundle/local-host-admission-{}",
        NEXT_FIXTURE.load(Ordering::Relaxed)
    ))
    .expect("bundle id");

    let output = invoke_foreground(
        &fixture,
        repository_root,
        &governance_root,
        &run_id,
        &bundle_id,
        &observed_at_text,
        &expires_at_text,
    );

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).expect("bounded stdout"),
        "trusted_host_local_application: operation.terminal\n"
    );
    assert_eq!(output.stderr, Vec::<u8>::new());

    let backend = SqliteStateBackend::open(&fixture.database).expect("reopen SQLite");
    let run = backend.rehydrate_run(&run_id).expect("durable run");
    assert_eq!(run.snapshot.status, WorkflowRunStatus::Running);
    assert!(run.snapshot.last_continuity_projection.is_some());
    assert!(run.events.len() >= 7);
    let stored = backend
        .read_exact_bundle(&run_id, &bundle_id)
        .expect("durable immutable bundle");
    assert_eq!(stored.manifest().run_id(), &run_id);
    assert_eq!(stored.manifest().bundle_id(), &bundle_id);

    let event_count = run.events.len();
    let replay = invoke_foreground(
        &fixture,
        repository_root,
        &governance_root,
        &run_id,
        &bundle_id,
        &observed_at_text,
        &expires_at_text,
    );
    assert!(replay.status.success());
    assert_eq!(
        String::from_utf8(replay.stdout).expect("bounded replay stdout"),
        "trusted_host_local_application: operation.terminal\n"
    );
    assert_eq!(
        backend
            .rehydrate_run(&run_id)
            .expect("replayed durable run")
            .events
            .len(),
        event_count,
        "terminal operational replay must not duplicate events"
    );
}

fn invoke_foreground(
    fixture: &Fixture,
    repository_root: &Path,
    governance_root: &Path,
    run_id: &WorkflowRunId,
    bundle_id: &ImmutableRunBundleId,
    observed_at: &str,
    expires_at: &str,
) -> Output {
    Command::new(env!("CARGO_BIN_EXE_workflow-local-host"))
        .args([
            "--governance-project-root",
            governance_root.to_str().expect("governance root"),
            "--repository-root",
            repository_root.to_str().expect("repository root"),
            "--state-database",
            fixture.database.to_str().expect("database"),
            "--npm-executable",
            fixture.npm.to_str().expect("fake npm"),
            "--workflow-id",
            "dg/trusted-host-docs-check",
            "--step-id",
            "docs-check",
            "--run-id",
            run_id.as_str(),
            "--bundle-id",
            bundle_id.as_str(),
            "--observed-at",
            observed_at,
            "--expires-at",
            expires_at,
            "--actor",
            "system/local-host-admission-test",
            "--correlation-id",
            "correlation-local-host-admission-test",
        ])
        .output()
        .expect("foreground binary")
}
