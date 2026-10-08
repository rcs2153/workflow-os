#![allow(clippy::expect_used)]
#![doc = "End-to-end proof for the unpublished foreground admission binary."]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

use workflow_core::{
    EventLogStore, ImmutableRunBundleId, ImmutableRunBundleStore, SqliteStateBackend, StateBackend,
    Timestamp, WorkflowRunId, WorkflowRunStatus,
};

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(1);

struct Fixture {
    id: u64,
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
            id,
            database: root.join("state.sqlite3"),
            npm,
            root,
        }
    }

    fn run_id(&self, label: &str) -> WorkflowRunId {
        WorkflowRunId::new(format!("run-local-host-{label}-{}", self.id)).expect("run id")
    }

    fn bundle_id(&self, label: &str) -> ImmutableRunBundleId {
        ImmutableRunBundleId::new(format!("bundle/local-host-{label}-{}", self.id))
            .expect("bundle id")
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
    let run_id = fixture.run_id("admission");
    let bundle_id = fixture.bundle_id("admission");

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

#[test]
fn foreground_binary_returns_bounded_wait_for_approval_without_execution() {
    let fixture = Fixture::new();
    let repository_root = repository_root();
    let governance_root = fixture.root.join("approval-project");
    write_approval_project(&repository_root, &governance_root);
    let (observed_at, expires_at) = window_times();
    let run_id = fixture.run_id("approval-wait");
    let bundle_id = fixture.bundle_id("approval-wait");

    let output = invoke_foreground_with(ForegroundInvocation {
        fixture: &fixture,
        repository_root: &repository_root,
        governance_root: &governance_root,
        workflow_id: "test/trusted-host-approval",
        step_id: "docs-check",
        run_id: &run_id,
        bundle_id: &bundle_id,
        observed_at: &observed_at,
        expires_at: &expires_at,
        actor: "system/local-host-approval-test",
        correlation_id: "correlation-local-host-approval-test",
        npm_executable: &fixture.npm,
    });

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).expect("bounded stdout"),
        "trusted_host_local_application: admission.waiting_approval\n"
    );
    assert_eq!(output.stderr, Vec::<u8>::new());
    let backend = SqliteStateBackend::open(&fixture.database).expect("reopen SQLite");
    let run = backend.rehydrate_run(&run_id).expect("durable waiting run");
    assert_eq!(run.snapshot.status, WorkflowRunStatus::WaitingForApproval);
    assert!(run.snapshot.last_continuity_projection.is_none());
}

#[test]
fn foreground_binary_rejects_unsupported_topology_without_creating_a_run() {
    let fixture = Fixture::new();
    let repository_root = repository_root();
    let governance_root = repository_root.join("dogfood/workflow-os-self-governance");
    let (observed_at, expires_at) = window_times();
    let run_id = fixture.run_id("unsupported-topology");
    let bundle_id = fixture.bundle_id("unsupported-topology");

    let output = invoke_foreground_with(ForegroundInvocation {
        fixture: &fixture,
        repository_root: &repository_root,
        governance_root: &governance_root,
        workflow_id: "dg/implement",
        step_id: "scope-confirmation",
        run_id: &run_id,
        bundle_id: &bundle_id,
        observed_at: &observed_at,
        expires_at: &expires_at,
        actor: "system/local-host-unsupported-test",
        correlation_id: "correlation-local-host-unsupported-test",
        npm_executable: &fixture.npm,
    });

    assert!(!output.status.success());
    assert_eq!(output.stdout, Vec::<u8>::new());
    assert_eq!(
        String::from_utf8(output.stderr).expect("bounded stderr"),
        "trusted_host_local_application: trusted_host_local_application.failure.unsupported\n"
    );
    let backend = SqliteStateBackend::open(&fixture.database).expect("reopen SQLite");
    let events = backend.read_events(&run_id).expect("read events");
    assert_eq!(events.len(), 0);
}

#[test]
fn foreground_binary_rejects_replay_identity_substitution_without_leakage() {
    let fixture = Fixture::new();
    let repository_root = repository_root();
    let governance_root = repository_root.join("dogfood/workflow-os-self-governance");
    let (observed_at, expires_at) = window_times();
    let run_id = fixture.run_id("identity-substitution");
    let bundle_id = fixture.bundle_id("identity-substitution");
    let initial = invoke_foreground_with(ForegroundInvocation {
        fixture: &fixture,
        repository_root: &repository_root,
        governance_root: &governance_root,
        workflow_id: "dg/trusted-host-docs-check",
        step_id: "docs-check",
        run_id: &run_id,
        bundle_id: &bundle_id,
        observed_at: &observed_at,
        expires_at: &expires_at,
        actor: "system/local-host-binding-test",
        correlation_id: "correlation-local-host-binding-test",
        npm_executable: &fixture.npm,
    });
    assert!(initial.status.success());
    let backend = SqliteStateBackend::open(&fixture.database).expect("reopen SQLite");
    let event_count = backend
        .rehydrate_run(&run_id)
        .expect("durable run")
        .events
        .len();

    let secret_marker = "authorization-private-key-marker";
    let actor_substitution = invoke_foreground_with(ForegroundInvocation {
        fixture: &fixture,
        repository_root: &repository_root,
        governance_root: &governance_root,
        workflow_id: "dg/trusted-host-docs-check",
        step_id: "docs-check",
        run_id: &run_id,
        bundle_id: &bundle_id,
        observed_at: &observed_at,
        expires_at: &expires_at,
        actor: "system/authorization-private-key-marker",
        correlation_id: "correlation-local-host-binding-test",
        npm_executable: &fixture.npm,
    });
    assert!(!actor_substitution.status.success());
    assert_bounded_failure(&actor_substitution, secret_marker);

    let correlation_substitution = invoke_foreground_with(ForegroundInvocation {
        fixture: &fixture,
        repository_root: &repository_root,
        governance_root: &governance_root,
        workflow_id: "dg/trusted-host-docs-check",
        step_id: "docs-check",
        run_id: &run_id,
        bundle_id: &bundle_id,
        observed_at: &observed_at,
        expires_at: &expires_at,
        actor: "system/local-host-binding-test",
        correlation_id: "correlation-substituted-value",
        npm_executable: &fixture.npm,
    });
    assert!(!correlation_substitution.status.success());
    assert_bounded_failure(&correlation_substitution, "correlation-substituted-value");
    assert_eq!(
        backend
            .rehydrate_run(&run_id)
            .expect("unchanged durable run")
            .events
            .len(),
        event_count
    );
}

#[test]
fn foreground_binary_rejects_same_path_executable_replacement() {
    let fixture = Fixture::new();
    let repository_root = repository_root();
    let governance_root = repository_root.join("dogfood/workflow-os-self-governance");
    let (observed_at, expires_at) = window_times();
    let run_id = fixture.run_id("command-substitution");
    let bundle_id = fixture.bundle_id("command-substitution");
    let initial = invoke_foreground_with(ForegroundInvocation {
        fixture: &fixture,
        repository_root: &repository_root,
        governance_root: &governance_root,
        workflow_id: "dg/trusted-host-docs-check",
        step_id: "docs-check",
        run_id: &run_id,
        bundle_id: &bundle_id,
        observed_at: &observed_at,
        expires_at: &expires_at,
        actor: "system/local-host-command-test",
        correlation_id: "correlation-local-host-command-test",
        npm_executable: &fixture.npm,
    });
    assert!(initial.status.success());

    fs::write(&fixture.npm, "#!/bin/sh\n# changed executable\nexit 0\n")
        .expect("replace fake npm contents");

    let substituted = invoke_foreground_with(ForegroundInvocation {
        fixture: &fixture,
        repository_root: &repository_root,
        governance_root: &governance_root,
        workflow_id: "dg/trusted-host-docs-check",
        step_id: "docs-check",
        run_id: &run_id,
        bundle_id: &bundle_id,
        observed_at: &observed_at,
        expires_at: &expires_at,
        actor: "system/local-host-command-test",
        correlation_id: "correlation-local-host-command-test",
        npm_executable: &fixture.npm,
    });
    assert!(!substituted.status.success());
    assert_bounded_failure(&substituted, "changed executable");
}

#[test]
fn foreground_binary_rejects_incompatible_sqlite_without_path_leakage() {
    let fixture = Fixture::new();
    fs::write(&fixture.database, b"not-a-sqlite-database").expect("invalid database fixture");
    let repository_root = repository_root();
    let governance_root = repository_root.join("dogfood/workflow-os-self-governance");
    let (observed_at, expires_at) = window_times();
    let run_id = fixture.run_id("invalid-database");
    let bundle_id = fixture.bundle_id("invalid-database");
    let output = invoke_foreground_with(ForegroundInvocation {
        fixture: &fixture,
        repository_root: &repository_root,
        governance_root: &governance_root,
        workflow_id: "dg/trusted-host-docs-check",
        step_id: "docs-check",
        run_id: &run_id,
        bundle_id: &bundle_id,
        observed_at: &observed_at,
        expires_at: &expires_at,
        actor: "system/local-host-database-test",
        correlation_id: "correlation-local-host-database-test",
        npm_executable: &fixture.npm,
    });

    assert!(!output.status.success());
    assert_eq!(output.stdout, Vec::<u8>::new());
    let stderr = String::from_utf8(output.stderr).expect("bounded stderr");
    assert_eq!(
        stderr,
        "trusted_host_local_application: state.unavailable\n"
    );
    assert!(!stderr.contains(fixture.database.to_str().expect("database path")));
}

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("repository root")
        .to_path_buf()
}

fn window_times() -> (String, String) {
    let observed_at = Timestamp::now_utc();
    let expires_at = Timestamp::from_offset_date_time(
        observed_at.as_offset_date_time() + time::Duration::minutes(5),
    );
    (observed_at.to_rfc3339(), expires_at.to_rfc3339())
}

fn assert_bounded_failure(output: &Output, forbidden: &str) {
    assert_eq!(output.stdout, Vec::<u8>::new());
    let stderr = String::from_utf8(output.stderr.clone()).expect("bounded stderr");
    assert!(stderr
        .starts_with("trusted_host_local_application: trusted_host_local_application.failure."));
    assert!(!stderr.contains(forbidden));
}

fn write_approval_project(repository_root: &Path, project_root: &Path) {
    fs::create_dir_all(project_root.join("workflows")).expect("workflow directory");
    fs::create_dir_all(project_root.join("skills")).expect("skill directory");
    fs::create_dir_all(project_root.join("policies")).expect("policy directory");
    fs::create_dir_all(project_root.join("tests")).expect("tests directory");
    fs::write(
        project_root.join("workflow-os.yml"),
        "schema_version: workflowos.dev/v0\nproject:\n  id: test/trusted-host-approval\n  name: Trusted Host Approval Test\n  description: Bounded approval-wait fixture.\nlayout:\n  workflows: workflows\n  skills: skills\n  policies: policies\n  tests: tests\n",
    )
    .expect("manifest");
    fs::copy(
        repository_root.join("dogfood/workflow-os-self-governance/skills/check-docs.skill.yml"),
        project_root.join("skills/check-docs.skill.yml"),
    )
    .expect("skill fixture");
    fs::copy(
        repository_root
            .join("dogfood/workflow-os-self-governance/policies/self-governance-local.policy.yml"),
        project_root.join("policies/local.policy.yml"),
    )
    .expect("local policy fixture");
    fs::copy(
        repository_root.join(
            "dogfood/workflow-os-self-governance/policies/self-governance-approval.policy.yml",
        ),
        project_root.join("policies/approval.policy.yml"),
    )
    .expect("approval policy fixture");
    fs::write(
        project_root.join("workflows/approval.workflow.yml"),
        APPROVAL_WORKFLOW,
    )
    .expect("approval workflow fixture");
}

const APPROVAL_WORKFLOW: &str = r"schema_version: workflowos.dev/v0
id: test/trusted-host-approval
version: v0
display_name: Trusted Host Approval Test
description: One-step docs check requiring explicit approval.
owner:
  owning_team: workflow-os-maintainers
  maintainer: workflow-os
  escalation_contact: workflow-os
  lifecycle_status: experimental
autonomy_level: level_2
triggers:
  - id: manual-start
    kind: manual
state_model:
  type: inline
  states: [requested, checked]
steps:
  - id: docs-check
    skill_ref:
      id: local/check-docs
      version: v0
    input_mapping:
      - from:
          type: literal
          value: trusted-host-approval-test
        to: request
    policy_requirements:
      - id: local/d
    approval_policy:
      policy:
        id: approval/d
    idempotency_key_strategy:
      type: derived
    timeout:
      duration: 5m
    terminal_behavior: fail_workflow
approval_requirements:
  - id: trusted-host-approval
    reason: Approval required for the bounded test fixture.
    expires_after:
      duration: 30m
cancellation_behavior: stop
audit_requirements:
  required: true
  events: [RunCreated, ApprovalRequested]
  store_references_only: true
observability_requirements:
  metrics: [workflow_latency]
  tracing: true
  latency_tracking: true
";

fn invoke_foreground(
    fixture: &Fixture,
    repository_root: &Path,
    governance_root: &Path,
    run_id: &WorkflowRunId,
    bundle_id: &ImmutableRunBundleId,
    observed_at: &str,
    expires_at: &str,
) -> Output {
    invoke_foreground_with(ForegroundInvocation {
        fixture,
        repository_root,
        governance_root,
        workflow_id: "dg/trusted-host-docs-check",
        step_id: "docs-check",
        run_id,
        bundle_id,
        observed_at,
        expires_at,
        actor: "system/local-host-admission-test",
        correlation_id: "correlation-local-host-admission-test",
        npm_executable: &fixture.npm,
    })
}

#[derive(Clone, Copy)]
struct ForegroundInvocation<'a> {
    fixture: &'a Fixture,
    repository_root: &'a Path,
    governance_root: &'a Path,
    workflow_id: &'a str,
    step_id: &'a str,
    run_id: &'a WorkflowRunId,
    bundle_id: &'a ImmutableRunBundleId,
    observed_at: &'a str,
    expires_at: &'a str,
    actor: &'a str,
    correlation_id: &'a str,
    npm_executable: &'a Path,
}

fn invoke_foreground_with(input: ForegroundInvocation<'_>) -> Output {
    Command::new(env!("CARGO_BIN_EXE_workflow-local-host"))
        .args([
            "--governance-project-root",
            input.governance_root.to_str().expect("governance root"),
            "--repository-root",
            input.repository_root.to_str().expect("repository root"),
            "--state-database",
            input.fixture.database.to_str().expect("database"),
            "--npm-executable",
            input.npm_executable.to_str().expect("fake npm"),
            "--workflow-id",
            input.workflow_id,
            "--step-id",
            input.step_id,
            "--run-id",
            input.run_id.as_str(),
            "--bundle-id",
            input.bundle_id.as_str(),
            "--observed-at",
            input.observed_at,
            "--expires-at",
            input.expires_at,
            "--actor",
            input.actor,
            "--correlation-id",
            input.correlation_id,
        ])
        .output()
        .expect("foreground binary")
}
