#![allow(clippy::expect_used)]
#![doc = "End-to-end proof for the unpublished foreground admission binary."]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};

use rusqlite::{params, Connection};
use serde_json::{json, Value};
use workflow_core::{
    admit_trusted_host_two_step_no_write_operation, ActorId, CorrelationId, EventLogStore,
    ExplicitLocalCheckProfileSelection, ImmutableRunBundleId, ImmutableRunBundleSensitivity,
    ImmutableRunBundleStore, ImmutableRunBundleVersion,
    LocalExecutionBeforeSkillInvocationCheckpointInputs, LocalExecutionImmutableRunBundleInputs,
    LocalExecutionRequest, LocalExecutionWithImmutableRunBundleRequest, SqliteStateBackend,
    StateBackend, Timestamp, TrustedHostLocalApplicationAdmissionOutcome,
    TrustedHostTwoStepNoWriteAdmissionRequest, TrustedHostTwoStepNoWriteProfileRegistry,
    WorkReportArtifactStore, WorkflowId, WorkflowRunEventKind, WorkflowRunEventKindName,
    WorkflowRunId, WorkflowRunStatus,
};
use workflow_local_host::LocalHostPreparedOperation;

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(1);

struct Fixture {
    id: u64,
    root: PathBuf,
    database: PathBuf,
    npm: PathBuf,
    workflow_os: PathBuf,
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
        let workflow_os = root.join("workflow-os");
        fs::write(
            &workflow_os,
            "#!/bin/sh\ntest \"$1\" = validate || exit 9\nexit 0\n",
        )
        .expect("fake workflow-os");
        let mut permissions = fs::metadata(&workflow_os)
            .expect("fake workflow-os metadata")
            .permissions();
        permissions.set_mode(0o700);
        fs::set_permissions(&workflow_os, permissions).expect("fake workflow-os executable");
        Self {
            id,
            database: root.join("state.sqlite3"),
            npm,
            workflow_os,
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

#[test]
fn foreground_binary_executes_explicit_project_validation_profile() {
    let fixture = Fixture::new();
    let governance_root = fixture.root.join("project-validation");
    write_project_validation_project(&governance_root);
    let (observed_at, expires_at) = window_times();
    let run_id = fixture.run_id("project-validation");
    let bundle_id = fixture.bundle_id("project-validation");

    let output = invoke_project_validation(
        &fixture,
        &governance_root,
        &run_id,
        &bundle_id,
        &observed_at,
        &expires_at,
    );

    assert!(
        output.status.success(),
        "foreground stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout).expect("bounded stdout"),
        "trusted_host_local_application: operation.terminal_report_artifact_persisted\n"
    );
    let backend = SqliteStateBackend::open(&fixture.database).expect("reopen SQLite");
    let run = backend.rehydrate_run(&run_id).expect("durable run");
    assert_eq!(run.snapshot.status, WorkflowRunStatus::Completed);
    assert!(run.events.iter().any(|event| matches!(
        event.kind,
        WorkflowRunEventKind::SkillInvocationSucceeded { .. }
    )));
    let artifacts = backend
        .list_work_report_artifacts(&run_id)
        .expect("persisted report artifact");
    assert_eq!(artifacts.len(), 1);
    assert_eq!(
        artifacts[0]
            .work_report()
            .report_contract_version()
            .as_str(),
        "v2"
    );
    let limitations = artifacts[0]
        .work_report()
        .known_limitations()
        .iter()
        .map(workflow_core::WorkReportKnownLimitation::summary)
        .collect::<Vec<_>>();
    assert!(!limitations
        .iter()
        .any(|summary| summary.contains("remains in memory and is not persisted")));
    assert!(limitations
        .iter()
        .any(|summary| summary.contains("no released inspection or export surface exists")));
}

#[test]
fn foreground_binary_executes_exact_two_step_no_write_workflow() {
    let fixture = Fixture::new();
    let governance_root = fixture.root.join("two-step-success");
    write_two_step_project(&governance_root);
    let (observed_at, expires_at) = window_times();
    let run_id = fixture.run_id("two-step-success");
    let bundle_id = fixture.bundle_id("two-step-success");

    let output = invoke_two_step(
        &fixture,
        &governance_root,
        &run_id,
        &bundle_id,
        &observed_at,
        &expires_at,
    );
    assert!(
        output.status.success(),
        "foreground stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout).expect("bounded stdout"),
        "trusted_host_local_application: operation.terminal_report_artifact_persisted\n"
    );

    let backend = SqliteStateBackend::open(&fixture.database).expect("reopen SQLite");
    let run = backend.rehydrate_run(&run_id).expect("durable run");
    assert_eq!(run.snapshot.status, WorkflowRunStatus::Completed);
    let scheduled = run
        .events
        .iter()
        .filter_map(|event| match &event.kind {
            WorkflowRunEventKind::StepScheduled { step_id } => Some(step_id.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(scheduled, vec!["project-validation", "docs-check"]);
    let results = run
        .events
        .iter()
        .filter_map(|event| match &event.kind {
            WorkflowRunEventKind::LocalCheckResultRecorded(result) => {
                Some(result.step_id().as_str())
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(results, vec!["project-validation", "docs-check"]);
    let artifacts = backend
        .list_work_report_artifacts(&run_id)
        .expect("persisted report artifact");
    assert_eq!(artifacts.len(), 1);
    assert_eq!(
        artifacts[0]
            .work_report()
            .report_contract_version()
            .as_str(),
        "v3"
    );
    assert_eq!(local_check_citation_count(artifacts[0].work_report()), 2);

    let replay = invoke_two_step(
        &fixture,
        &governance_root,
        &run_id,
        &bundle_id,
        &observed_at,
        &expires_at,
    );
    assert!(replay.status.success());
    assert_eq!(
        String::from_utf8(replay.stdout).expect("bounded replay stdout"),
        "trusted_host_local_application: admission.terminal_replay_report_artifact_already_persisted\n"
    );
    assert_eq!(
        backend.rehydrate_run(&run_id).expect("replayed run").events,
        run.events
    );
}

#[test]
fn two_step_terminal_replay_rejects_result_before_invocation_history_corruption() {
    let fixture = Fixture::new();
    let governance_root = fixture.root.join("two-step-corrupt-order");
    write_two_step_project(&governance_root);
    let (observed_at, expires_at) = window_times();
    let run_id = fixture.run_id("two-step-corrupt-order");
    let bundle_id = fixture.bundle_id("two-step-corrupt-order");

    let initial = invoke_two_step(
        &fixture,
        &governance_root,
        &run_id,
        &bundle_id,
        &observed_at,
        &expires_at,
    );
    assert!(initial.status.success());
    clear_artifacts_and_swap_event_order(
        &fixture.database,
        &run_id,
        "LocalCheckResultRecorded",
        "SkillInvocationSucceeded",
    );
    let backend = SqliteStateBackend::open(&fixture.database).expect("reopen corrupt SQLite");
    let corrupt_events = backend
        .rehydrate_run(&run_id)
        .expect("corrupt run remains readable")
        .events;

    let replay = invoke_two_step(
        &fixture,
        &governance_root,
        &run_id,
        &bundle_id,
        &observed_at,
        &expires_at,
    );
    assert!(!replay.status.success());
    assert_eq!(replay.stdout, Vec::<u8>::new());
    assert_eq!(
        String::from_utf8(replay.stderr).expect("bounded corrupt replay stderr"),
        "trusted_host_local_application: trusted_host_local_application.failure.unsupported\n"
    );
    let after = backend
        .rehydrate_run(&run_id)
        .expect("corrupt run remains readable after rejection");
    assert_eq!(after.events, corrupt_events);
    assert_eq!(
        backend
            .list_work_report_artifacts(&run_id)
            .expect("artifact list remains readable")
            .len(),
        0
    );
}

#[test]
fn two_step_first_failure_never_schedules_docs_check() {
    let fixture = Fixture::new();
    fs::write(
        &fixture.workflow_os,
        "#!/bin/sh\ntest \"$1\" = validate || exit 9\nexit 7\n",
    )
    .expect("failing workflow-os fixture");
    let governance_root = fixture.root.join("two-step-first-failure");
    write_two_step_project(&governance_root);
    let (observed_at, expires_at) = window_times();
    let run_id = fixture.run_id("two-step-first-failure");
    let bundle_id = fixture.bundle_id("two-step-first-failure");

    let output = invoke_two_step(
        &fixture,
        &governance_root,
        &run_id,
        &bundle_id,
        &observed_at,
        &expires_at,
    );
    assert!(output.status.success());
    let backend = SqliteStateBackend::open(&fixture.database).expect("reopen SQLite");
    let run = backend.rehydrate_run(&run_id).expect("failed run");
    assert_eq!(run.snapshot.status, WorkflowRunStatus::Failed);
    assert_eq!(
        run.events
            .iter()
            .filter(|event| matches!(
                event.kind,
                WorkflowRunEventKind::LocalCheckResultRecorded(_)
            ))
            .count(),
        1
    );
    assert!(!run.events.iter().any(|event| matches!(
        &event.kind,
        WorkflowRunEventKind::StepScheduled { step_id } if step_id.as_str() == "docs-check"
    )));
    let artifacts = backend
        .list_work_report_artifacts(&run_id)
        .expect("failure artifact");
    assert_eq!(artifacts.len(), 1);
    assert_eq!(local_check_citation_count(artifacts[0].work_report()), 1);
    assert!(artifacts[0]
        .work_report()
        .incomplete_work()
        .iter()
        .any(|item| item.summary().contains("not attempted")));
}

#[test]
fn two_step_second_failure_records_both_results_and_fails_honestly() {
    let fixture = Fixture::new();
    fs::write(&fixture.npm, "#!/bin/sh\nexit 8\n").expect("failing npm fixture");
    let governance_root = fixture.root.join("two-step-second-failure");
    write_two_step_project(&governance_root);
    let (observed_at, expires_at) = window_times();
    let run_id = fixture.run_id("two-step-second-failure");
    let bundle_id = fixture.bundle_id("two-step-second-failure");

    let output = invoke_two_step(
        &fixture,
        &governance_root,
        &run_id,
        &bundle_id,
        &observed_at,
        &expires_at,
    );
    assert!(output.status.success());
    let backend = SqliteStateBackend::open(&fixture.database).expect("reopen SQLite");
    let run = backend.rehydrate_run(&run_id).expect("failed run");
    assert_eq!(run.snapshot.status, WorkflowRunStatus::Failed);
    assert_eq!(
        run.events
            .iter()
            .filter(|event| matches!(
                event.kind,
                WorkflowRunEventKind::LocalCheckResultRecorded(_)
            ))
            .count(),
        2
    );
    assert!(run.events.iter().any(|event| matches!(
        &event.kind,
        WorkflowRunEventKind::SkillInvocationFailed { step_id, .. }
            if step_id.as_str() == "docs-check"
    )));
    let artifacts = backend
        .list_work_report_artifacts(&run_id)
        .expect("failure artifact");
    assert_eq!(artifacts.len(), 1);
    assert_eq!(local_check_citation_count(artifacts[0].work_report()), 2);
}

#[test]
fn two_step_mode_rejects_caller_selected_step_before_state_creation() {
    let fixture = Fixture::new();
    let governance_root = fixture.root.join("two-step-caller-step");
    write_two_step_project(&governance_root);
    let (observed_at, expires_at) = window_times();
    let run_id = fixture.run_id("two-step-caller-step");
    let bundle_id = fixture.bundle_id("two-step-caller-step");
    let mut command = base_foreground_command(
        &fixture,
        &governance_root,
        &run_id,
        &bundle_id,
        &observed_at,
        &expires_at,
    );
    let output = command
        .args([
            "--mode",
            "two-step-no-write",
            "--step-id",
            "project-validation",
            "--repository-root",
            repository_root().to_str().expect("repository root"),
            "--npm-executable",
            fixture.npm.to_str().expect("npm"),
            "--workflow-os-executable",
            fixture.workflow_os.to_str().expect("workflow-os"),
            "--workflow-id",
            "test/trusted-host-two-step",
            "--actor",
            "system/local-host-two-step-test",
            "--correlation-id",
            "correlation-local-host-two-step-test",
        ])
        .output()
        .expect("foreground binary");
    assert!(!output.status.success());
    assert_eq!(
        String::from_utf8(output.stderr).expect("bounded stderr"),
        "trusted_host_local_application: input.invalid\n"
    );
    assert!(!fixture.database.exists());
}

#[test]
fn two_step_restart_after_first_commit_resumes_only_docs_check() -> Result<(), &'static str> {
    let fixture = Fixture::new();
    let governance_root = fixture.root.join("two-step-restart");
    write_two_step_project(&governance_root);
    let (observed_at, expires_at) = window_times();
    let run_id = fixture.run_id("two-step-restart");
    let bundle_id = fixture.bundle_id("two-step-restart");
    let project_validation = ExplicitLocalCheckProfileSelection::workflow_os_project_validation()
        .resolve(fixture.workflow_os.clone(), governance_root.clone())
        .expect("project validation profile");
    let docs_check = ExplicitLocalCheckProfileSelection::docs_check()
        .resolve_docs_check(fixture.npm.clone(), repository_root(), None)
        .expect("docs profile");
    let registry = TrustedHostTwoStepNoWriteProfileRegistry::new(project_validation, docs_check)
        .expect("closed registry");
    let backend = SqliteStateBackend::open(&fixture.database).expect("open state");
    let mut request = TrustedHostTwoStepNoWriteAdmissionRequest {
        execution: LocalExecutionWithImmutableRunBundleRequest {
            execution: LocalExecutionRequest {
                project_root: governance_root,
                workflow_id: WorkflowId::new("test/trusted-host-two-step").expect("workflow"),
                run_id: Some(run_id.clone()),
                correlation_id: CorrelationId::new("correlation-two-step-restart")
                    .expect("correlation"),
                actor: ActorId::new("system/two-step-restart-test").expect("actor"),
                before_skill_invocation_checkpoints:
                    LocalExecutionBeforeSkillInvocationCheckpointInputs::default(),
                before_skill_invocation_hook: None,
                side_effect_events: Vec::new(),
                side_effect_lifecycle_events: Vec::new(),
            },
            bundle: LocalExecutionImmutableRunBundleInputs {
                bundle_id,
                bundle_version: ImmutableRunBundleVersion::new("v1").expect("version"),
                created_at: Timestamp::parse_rfc3339(&observed_at).expect("observed at"),
                sensitivity: ImmutableRunBundleSensitivity::Internal,
                redaction_required: true,
            },
        },
        observed_at: Timestamp::parse_rfc3339(&observed_at).expect("observed at"),
        expires_at: Timestamp::parse_rfc3339(&expires_at).expect("expires at"),
    };
    let first = admit_trusted_host_two_step_no_write_operation(&backend, &registry, &request)
        .expect("first admission");
    let TrustedHostLocalApplicationAdmissionOutcome::Prepared(first) = first else {
        return Err("first operation must be prepared");
    };
    LocalHostPreparedOperation::from_prepared(first)
        .run()
        .expect("first operation");
    let before = backend.rehydrate_run(&run_id).expect("intermediate run");
    assert_eq!(before.snapshot.status, WorkflowRunStatus::Running);
    assert_eq!(
        before
            .events
            .iter()
            .filter(|event| matches!(
                event.kind,
                WorkflowRunEventKind::LocalCheckResultRecorded(_)
            ))
            .count(),
        1
    );
    request.observed_at = Timestamp::now_utc();
    let second = admit_trusted_host_two_step_no_write_operation(&backend, &registry, &request)
        .expect("second admission");
    let TrustedHostLocalApplicationAdmissionOutcome::Prepared(second) = second else {
        return Err("second operation must be prepared");
    };
    LocalHostPreparedOperation::from_prepared(second)
        .run()
        .expect("second operation");
    let after = backend.rehydrate_run(&run_id).expect("terminal run");
    assert_eq!(after.snapshot.status, WorkflowRunStatus::Completed);
    let result_steps = after
        .events
        .iter()
        .filter_map(|event| match &event.kind {
            WorkflowRunEventKind::LocalCheckResultRecorded(result) => {
                Some(result.step_id().as_str())
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(result_steps, vec!["project-validation", "docs-check"]);
    assert!(matches!(
        admit_trusted_host_two_step_no_write_operation(&backend, &registry, &request)
            .expect("terminal replay"),
        TrustedHostLocalApplicationAdmissionOutcome::TerminalReplay
    ));
    Ok(())
}

#[test]
fn terminal_replay_recovers_one_missing_artifact_without_mutating_run_history() {
    let fixture = Fixture::new();
    let repository_root = repository_root();
    let governance_root = repository_root.join("dogfood/workflow-os-self-governance");
    let (observed_at, expires_at) = window_times();
    let run_id = fixture.run_id("artifact-recovery");
    let bundle_id = fixture.bundle_id("artifact-recovery");
    let initial = invoke_foreground(
        &fixture,
        &repository_root,
        &governance_root,
        &run_id,
        &bundle_id,
        &observed_at,
        &expires_at,
    );
    assert!(
        initial.status.success(),
        "foreground stderr: {}",
        String::from_utf8_lossy(&initial.stderr)
    );

    let backend = SqliteStateBackend::open(&fixture.database).expect("reopen SQLite");
    let event_count = backend
        .rehydrate_run(&run_id)
        .expect("terminal run before recovery")
        .events
        .len();
    clear_artifacts(&fixture.database, &run_id);

    let replay = invoke_foreground(
        &fixture,
        &repository_root,
        &governance_root,
        &run_id,
        &bundle_id,
        &observed_at,
        &expires_at,
    );
    assert!(
        replay.status.success(),
        "recovery stderr: {}",
        String::from_utf8_lossy(&replay.stderr)
    );
    assert_eq!(
        String::from_utf8(replay.stdout).expect("bounded recovery stdout"),
        "trusted_host_local_application: admission.terminal_replay_report_artifact_persisted\n"
    );
    assert_eq!(
        backend
            .rehydrate_run(&run_id)
            .expect("terminal run after recovery")
            .events
            .len(),
        event_count,
        "artifact recovery must not append workflow events"
    );
    assert_eq!(
        backend
            .list_work_report_artifacts(&run_id)
            .expect("recovered report artifact")
            .len(),
        1
    );
}

#[test]
fn concurrent_terminal_replays_converge_on_one_equal_artifact() {
    let fixture = Fixture::new();
    let repository_root = repository_root();
    let governance_root = repository_root.join("dogfood/workflow-os-self-governance");
    let (observed_at, expires_at) = window_times();
    let run_id = fixture.run_id("artifact-concurrency");
    let bundle_id = fixture.bundle_id("artifact-concurrency");
    let invocation = ForegroundInvocation {
        fixture: &fixture,
        repository_root: &repository_root,
        governance_root: &governance_root,
        workflow_id: "dg/trusted-host-docs-check",
        step_id: "docs-check",
        run_id: &run_id,
        bundle_id: &bundle_id,
        observed_at: &observed_at,
        expires_at: &expires_at,
        actor: "system/local-host-admission-test",
        correlation_id: "correlation-local-host-admission-test",
        npm_executable: &fixture.npm,
    };
    let initial = invoke_foreground_with(invocation);
    assert!(initial.status.success());

    let backend = SqliteStateBackend::open(&fixture.database).expect("reopen SQLite");
    let event_count = backend
        .rehydrate_run(&run_id)
        .expect("terminal run before concurrent recovery")
        .events
        .len();
    clear_artifacts(&fixture.database, &run_id);

    let mut first_command = foreground_command_with(invocation);
    first_command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let first = first_command.spawn().expect("first concurrent replay");
    let mut second_command = foreground_command_with(invocation);
    second_command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let second = second_command.spawn().expect("second concurrent replay");
    let outputs = [
        first.wait_with_output().expect("first replay output"),
        second.wait_with_output().expect("second replay output"),
    ];
    assert!(outputs.iter().all(|output| output.status.success()));
    let mut postures = outputs
        .into_iter()
        .map(|output| String::from_utf8(output.stdout).expect("bounded concurrent stdout"))
        .collect::<Vec<_>>();
    postures.sort();
    assert_eq!(
        postures,
        vec![
            "trusted_host_local_application: admission.terminal_replay_report_artifact_already_persisted\n",
            "trusted_host_local_application: admission.terminal_replay_report_artifact_persisted\n",
        ]
    );
    assert_eq!(
        backend
            .rehydrate_run(&run_id)
            .expect("terminal run after concurrent recovery")
            .events
            .len(),
        event_count
    );
    assert_eq!(
        backend
            .list_work_report_artifacts(&run_id)
            .expect("one converged artifact")
            .len(),
        1
    );
}

#[test]
fn terminal_replay_rejects_valid_conflicting_artifact_without_overwrite() {
    let fixture = Fixture::new();
    let repository_root = repository_root();
    let governance_root = repository_root.join("dogfood/workflow-os-self-governance");
    let (observed_at, expires_at) = window_times();
    let run_id = fixture.run_id("artifact-conflict");
    let bundle_id = fixture.bundle_id("artifact-conflict");
    let initial = invoke_foreground(
        &fixture,
        &repository_root,
        &governance_root,
        &run_id,
        &bundle_id,
        &observed_at,
        &expires_at,
    );
    assert!(initial.status.success());

    let backend = SqliteStateBackend::open(&fixture.database).expect("reopen SQLite");
    let event_count = backend
        .rehydrate_run(&run_id)
        .expect("terminal run before conflicting replay")
        .events
        .len();
    let conflicting_payload = mutate_artifact_payload(&fixture.database, &run_id, |value| {
        value["work_report"]["known_limitations"][0]["summary"] =
            json!("A different valid limitation remains bounded and local.");
    });

    let replay = invoke_foreground(
        &fixture,
        &repository_root,
        &governance_root,
        &run_id,
        &bundle_id,
        &observed_at,
        &expires_at,
    );
    assert_artifact_persistence_failure(&replay);
    assert_eq!(
        backend
            .rehydrate_run(&run_id)
            .expect("terminal run after conflicting replay")
            .events
            .len(),
        event_count
    );
    assert_eq!(
        read_artifact_payload(&fixture.database, &run_id),
        conflicting_payload
    );
}

#[test]
fn terminal_replay_rejects_unreadable_artifact_without_repair() {
    let fixture = Fixture::new();
    let repository_root = repository_root();
    let governance_root = repository_root.join("dogfood/workflow-os-self-governance");
    let (observed_at, expires_at) = window_times();
    let run_id = fixture.run_id("artifact-unreadable");
    let bundle_id = fixture.bundle_id("artifact-unreadable");
    let initial = invoke_foreground(
        &fixture,
        &repository_root,
        &governance_root,
        &run_id,
        &bundle_id,
        &observed_at,
        &expires_at,
    );
    assert!(initial.status.success());

    let backend = SqliteStateBackend::open(&fixture.database).expect("reopen SQLite");
    let event_count = backend
        .rehydrate_run(&run_id)
        .expect("terminal run before unreadable replay")
        .events
        .len();
    let unreadable_payload = "not-json";
    write_artifact_payload(&fixture.database, &run_id, unreadable_payload);

    let replay = invoke_foreground(
        &fixture,
        &repository_root,
        &governance_root,
        &run_id,
        &bundle_id,
        &observed_at,
        &expires_at,
    );
    assert_artifact_persistence_failure(&replay);
    assert_eq!(
        backend
            .rehydrate_run(&run_id)
            .expect("terminal run after unreadable replay")
            .events
            .len(),
        event_count
    );
    assert_eq!(
        read_artifact_payload(&fixture.database, &run_id),
        unreadable_payload
    );
}

#[test]
fn foreground_binary_terminalizes_nonpassing_checks_as_failure() {
    let fixture = Fixture::new();
    fs::write(&fixture.npm, "#!/bin/sh\nexit 7\n").expect("failing npm");
    let repository_root = repository_root();
    let governance_root = repository_root.join("dogfood/workflow-os-self-governance");
    let (observed_at, expires_at) = window_times();
    let run_id = fixture.run_id("docs-failure");
    let bundle_id = fixture.bundle_id("docs-failure");

    let output = invoke_foreground(
        &fixture,
        &repository_root,
        &governance_root,
        &run_id,
        &bundle_id,
        &observed_at,
        &expires_at,
    );
    assert!(output.status.success());
    let backend = SqliteStateBackend::open(&fixture.database).expect("reopen SQLite");
    let run = backend.rehydrate_run(&run_id).expect("durable failed run");
    assert_eq!(run.snapshot.status, WorkflowRunStatus::Failed);
    assert!(run.events.iter().any(|event| matches!(
        event.kind,
        WorkflowRunEventKind::SkillInvocationFailed { .. }
    )));
    assert_terminal_result_binding(&run, workflow_core::LocalCheckResultStatus::Failed);
    let artifacts = backend
        .list_work_report_artifacts(&run_id)
        .expect("failed terminal report artifact");
    assert_eq!(artifacts.len(), 1);
    assert_eq!(
        artifacts[0].metadata().terminal_run_status(),
        workflow_core::WorkReportStatus::Failed
    );
    assert!(!run.events.iter().any(|event| matches!(
        event.kind,
        WorkflowRunEventKind::SkillInvocationSucceeded { .. } | WorkflowRunEventKind::RunCompleted
    )));

    let event_count = run.events.len();
    let replay = invoke_foreground(
        &fixture,
        &repository_root,
        &governance_root,
        &run_id,
        &bundle_id,
        &observed_at,
        &expires_at,
    );
    assert!(
        replay.status.success(),
        "corrupt replay stderr: {}",
        String::from_utf8_lossy(&replay.stderr)
    );
    assert_eq!(
        String::from_utf8(replay.stdout).expect("bounded replay stdout"),
        "trusted_host_local_application: admission.terminal_replay_report_artifact_already_persisted\n"
    );
    assert_eq!(
        backend
            .rehydrate_run(&run_id)
            .expect("replayed failed run")
            .events
            .len(),
        event_count
    );
    assert_eq!(
        backend
            .list_work_report_artifacts(&run_id)
            .expect("replayed report artifacts")
            .len(),
        1
    );
}

#[test]
fn terminal_artifact_entry_rejects_passed_run_with_failed_result_history() {
    let fixture = Fixture::new();
    let repository_root = repository_root();
    let governance_root = repository_root.join("dogfood/workflow-os-self-governance");
    let (observed_at, expires_at) = window_times();
    let run_id = fixture.run_id("tampered-failed-result");
    let bundle_id = fixture.bundle_id("tampered-failed-result");
    let initial = invoke_foreground(
        &fixture,
        &repository_root,
        &governance_root,
        &run_id,
        &bundle_id,
        &observed_at,
        &expires_at,
    );
    assert!(initial.status.success());
    clear_artifacts_and_mutate_event(
        &fixture.database,
        &run_id,
        "LocalCheckResultRecorded",
        |kind| {
            kind["status"] = json!("failed");
        },
    );

    assert_corrupt_replay_fails_before_artifact(
        &fixture,
        &repository_root,
        &governance_root,
        &run_id,
        &bundle_id,
        &observed_at,
        &expires_at,
    );
}

#[test]
fn terminal_artifact_entry_rejects_failed_run_with_passed_result_history() {
    let fixture = Fixture::new();
    fs::write(&fixture.npm, "#!/bin/sh\nexit 7\n").expect("failing npm");
    let repository_root = repository_root();
    let governance_root = repository_root.join("dogfood/workflow-os-self-governance");
    let (observed_at, expires_at) = window_times();
    let run_id = fixture.run_id("tampered-passed-result");
    let bundle_id = fixture.bundle_id("tampered-passed-result");
    let initial = invoke_foreground(
        &fixture,
        &repository_root,
        &governance_root,
        &run_id,
        &bundle_id,
        &observed_at,
        &expires_at,
    );
    assert!(initial.status.success());
    clear_artifacts_and_mutate_event(
        &fixture.database,
        &run_id,
        "LocalCheckResultRecorded",
        |kind| {
            kind["status"] = json!("passed");
        },
    );

    assert_corrupt_replay_fails_before_artifact(
        &fixture,
        &repository_root,
        &governance_root,
        &run_id,
        &bundle_id,
        &observed_at,
        &expires_at,
    );
}

#[test]
fn terminal_artifact_entry_rejects_concrete_result_with_cancellation_history() {
    let fixture = Fixture::new();
    let repository_root = repository_root();
    let governance_root = repository_root.join("dogfood/workflow-os-self-governance");
    let (observed_at, expires_at) = window_times();
    let run_id = fixture.run_id("tampered-cancellation");
    let bundle_id = fixture.bundle_id("tampered-cancellation");
    let initial = invoke_foreground(
        &fixture,
        &repository_root,
        &governance_root,
        &run_id,
        &bundle_id,
        &observed_at,
        &expires_at,
    );
    assert!(initial.status.success());
    clear_artifacts_and_mutate_event(&fixture.database, &run_id, "RunCompleted", |kind| {
        *kind = json!({
            "kind": "RunCanceled",
            "reason": "tampered cancellation",
            "actor": "system/history-integrity-test",
            "canceled_at": observed_at,
            "correlation_id": "correlation-history-integrity-test"
        });
    });

    assert_corrupt_replay_fails_before_artifact(
        &fixture,
        &repository_root,
        &governance_root,
        &run_id,
        &bundle_id,
        &observed_at,
        &expires_at,
    );
}

#[test]
fn terminal_artifact_entry_rejects_invocation_before_result_history() {
    let fixture = Fixture::new();
    let repository_root = repository_root();
    let governance_root = repository_root.join("dogfood/workflow-os-self-governance");
    let (observed_at, expires_at) = window_times();
    let run_id = fixture.run_id("tampered-order");
    let bundle_id = fixture.bundle_id("tampered-order");
    let initial = invoke_foreground(
        &fixture,
        &repository_root,
        &governance_root,
        &run_id,
        &bundle_id,
        &observed_at,
        &expires_at,
    );
    assert!(initial.status.success());
    clear_artifacts_and_swap_event_order(
        &fixture.database,
        &run_id,
        "LocalCheckResultRecorded",
        "SkillInvocationSucceeded",
    );

    assert_corrupt_replay_fails_before_artifact(
        &fixture,
        &repository_root,
        &governance_root,
        &run_id,
        &bundle_id,
        &observed_at,
        &expires_at,
    );
}

#[test]
fn foreground_binary_project_validation_failure_is_bounded() {
    let fixture = Fixture::new();
    fs::write(
        &fixture.workflow_os,
        "#!/bin/sh\necho private-output >&2\nexit 8\n",
    )
    .expect("failing workflow-os");
    let governance_root = fixture.root.join("project-validation-failure");
    write_project_validation_project(&governance_root);
    let (observed_at, expires_at) = window_times();
    let run_id = fixture.run_id("project-validation-failure");
    let bundle_id = fixture.bundle_id("project-validation-failure");

    let output = invoke_project_validation(
        &fixture,
        &governance_root,
        &run_id,
        &bundle_id,
        &observed_at,
        &expires_at,
    );
    assert!(output.status.success());
    assert!(!String::from_utf8_lossy(&output.stdout).contains("private-output"));
    let run = SqliteStateBackend::open(&fixture.database)
        .expect("reopen SQLite")
        .rehydrate_run(&run_id)
        .expect("durable failed run");
    assert_eq!(run.snapshot.status, WorkflowRunStatus::Failed);
    assert!(!format!("{run:?}").contains("private-output"));
}

#[test]
fn foreground_binary_rejects_invalid_profile_shape_before_state_creation() {
    let fixture = Fixture::new();
    let governance_root = fixture.root.join("project-validation-selection");
    write_project_validation_project(&governance_root);
    let (observed_at, expires_at) = window_times();
    let run_id = fixture.run_id("invalid-selection");
    let bundle_id = fixture.bundle_id("invalid-selection");

    let output = base_foreground_command(
        &fixture,
        &governance_root,
        &run_id,
        &bundle_id,
        &observed_at,
        &expires_at,
    )
    .args([
        "--workflow-os-executable",
        fixture.workflow_os.to_str().expect("workflow-os path"),
        "--workflow-id",
        "test/trusted-host-project-validation",
        "--step-id",
        "project-validation",
        "--actor",
        "system/local-host-invalid-selection-test",
        "--correlation-id",
        "correlation-local-host-invalid-selection-test",
    ])
    .output()
    .expect("foreground binary");

    assert!(!output.status.success());
    assert_eq!(
        String::from_utf8(output.stderr).expect("bounded stderr"),
        "trusted_host_local_application: input.invalid\n"
    );
    assert!(!fixture.database.exists());
}

#[test]
fn foreground_binary_rejects_mixed_profile_arguments_before_state_creation() {
    let fixture = Fixture::new();
    let repository_root = repository_root();
    let governance_root = fixture.root.join("mixed-profile-selection");
    write_project_validation_project(&governance_root);
    let (observed_at, expires_at) = window_times();
    let run_id = fixture.run_id("mixed-profile-selection");
    let bundle_id = fixture.bundle_id("mixed-profile-selection");

    let output = base_foreground_command(
        &fixture,
        &governance_root,
        &run_id,
        &bundle_id,
        &observed_at,
        &expires_at,
    )
    .args([
        "--profile",
        "workflow-os-project-validation",
        "--workflow-os-executable",
        fixture.workflow_os.to_str().expect("workflow-os path"),
        "--repository-root",
        repository_root.to_str().expect("repository root"),
        "--npm-executable",
        fixture.npm.to_str().expect("npm path"),
        "--workflow-id",
        "test/trusted-host-project-validation",
        "--step-id",
        "project-validation",
        "--actor",
        "system/local-host-mixed-profile-test",
        "--correlation-id",
        "correlation-local-host-mixed-profile-test",
    ])
    .output()
    .expect("foreground binary");

    assert!(!output.status.success());
    assert_eq!(
        String::from_utf8(output.stderr).expect("bounded stderr"),
        "trusted_host_local_application: input.invalid\n"
    );
    assert!(!fixture.database.exists());
}

#[test]
fn foreground_binary_accepts_explicit_docs_profile_and_rejects_unknown_profile() {
    let fixture = Fixture::new();
    let repository_root = repository_root();
    let governance_root = repository_root.join("dogfood/workflow-os-self-governance");
    let (observed_at, expires_at) = window_times();
    let run_id = fixture.run_id("explicit-docs-profile");
    let bundle_id = fixture.bundle_id("explicit-docs-profile");
    let explicit = base_foreground_command(
        &fixture,
        &governance_root,
        &run_id,
        &bundle_id,
        &observed_at,
        &expires_at,
    )
    .args([
        "--profile",
        "docs-check",
        "--repository-root",
        repository_root.to_str().expect("repository root"),
        "--npm-executable",
        fixture.npm.to_str().expect("npm path"),
        "--workflow-id",
        "dg/trusted-host-docs-check",
        "--step-id",
        "docs-check",
        "--actor",
        "system/local-host-explicit-docs-test",
        "--correlation-id",
        "correlation-local-host-explicit-docs-test",
    ])
    .output()
    .expect("foreground binary");
    assert!(
        explicit.status.success(),
        "foreground stderr: {}",
        String::from_utf8_lossy(&explicit.stderr)
    );

    let unknown_fixture = Fixture::new();
    let unknown_run_id = unknown_fixture.run_id("unknown-profile");
    let unknown_bundle_id = unknown_fixture.bundle_id("unknown-profile");
    let unknown = base_foreground_command(
        &unknown_fixture,
        &governance_root,
        &unknown_run_id,
        &unknown_bundle_id,
        &observed_at,
        &expires_at,
    )
    .args([
        "--profile",
        "unknown-profile",
        "--workflow-id",
        "dg/trusted-host-docs-check",
        "--step-id",
        "docs-check",
        "--actor",
        "system/local-host-unknown-profile-test",
        "--correlation-id",
        "correlation-local-host-unknown-profile-test",
    ])
    .output()
    .expect("foreground binary");
    assert!(!unknown.status.success());
    assert_eq!(
        String::from_utf8(unknown.stderr).expect("bounded stderr"),
        "trusted_host_local_application: profile.invalid\n"
    );
    assert!(!unknown_fixture.database.exists());
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

    assert!(
        output.status.success(),
        "foreground stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout).expect("bounded stdout"),
        "trusted_host_local_application: operation.terminal_report_artifact_persisted\n"
    );
    assert_eq!(output.stderr, Vec::<u8>::new());

    let backend = SqliteStateBackend::open(&fixture.database).expect("reopen SQLite");
    let run = backend.rehydrate_run(&run_id).expect("durable run");
    assert_eq!(run.snapshot.status, WorkflowRunStatus::Completed);
    assert!(run.snapshot.last_continuity_projection.is_some());
    let event_kinds: Vec<_> = run
        .events
        .iter()
        .map(workflow_core::WorkflowRunEvent::kind)
        .collect();
    assert!(event_kinds.ends_with(&[
        WorkflowRunEventKindName::SkillInvocationRequested,
        WorkflowRunEventKindName::OperationalExecutionWindowOpened,
        WorkflowRunEventKindName::AuthorizedExecutionAttemptDispatchAdmitted,
        WorkflowRunEventKindName::SkillInvocationStarted,
        WorkflowRunEventKindName::AuthorizedExecutionContinuityProjected,
        WorkflowRunEventKindName::LocalCheckResultRecorded,
        WorkflowRunEventKindName::SkillInvocationSucceeded,
        WorkflowRunEventKindName::RunCompleted,
    ]));
    assert_terminal_result_binding(&run, workflow_core::LocalCheckResultStatus::Passed);
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
        "trusted_host_local_application: admission.terminal_replay_report_artifact_already_persisted\n"
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
    assert_eq!(
        backend
            .list_work_report_artifacts(&run_id)
            .expect("replayed report artifacts")
            .len(),
        1,
        "terminal operational replay must not duplicate artifacts"
    );
}

fn assert_terminal_result_binding(
    run: &workflow_core::WorkflowRun,
    expected_status: workflow_core::LocalCheckResultStatus,
) {
    let result_bindings: Vec<_> = run
        .events
        .iter()
        .filter_map(|event| match &event.kind {
            WorkflowRunEventKind::LocalCheckResultRecorded(binding) => Some(binding.as_ref()),
            _ => None,
        })
        .collect();
    assert_eq!(result_bindings.len(), 1);
    assert_eq!(result_bindings[0].status(), expected_status);
    assert_eq!(
        result_bindings[0].immutable_run_bundle(),
        run.snapshot
            .identity
            .immutable_run_bundle
            .as_ref()
            .expect("bundle binding")
    );
}

fn clear_artifacts_and_mutate_event(
    database: &Path,
    run_id: &WorkflowRunId,
    target_kind: &str,
    mutate: impl FnOnce(&mut Value),
) {
    let connection = Connection::open(database).expect("open fixture database for corruption test");
    clear_artifacts_with_connection(&connection, run_id);
    let (event_id, payload) = find_event_payload(&connection, run_id, target_kind);
    let mut value: Value = serde_json::from_str(&payload).expect("event payload JSON");
    mutate(&mut value["kind"]);
    connection
        .execute(
            "UPDATE events SET payload = ?1 WHERE event_id = ?2",
            params![value.to_string(), event_id],
        )
        .expect("write corrupt event payload");
}

fn clear_artifacts(database: &Path, run_id: &WorkflowRunId) {
    let connection = Connection::open(database).expect("open fixture database for artifact reset");
    clear_artifacts_with_connection(&connection, run_id);
}

fn clear_artifacts_with_connection(connection: &Connection, run_id: &WorkflowRunId) {
    connection
        .execute(
            "DELETE FROM work_report_artifacts WHERE run_id = ?1",
            params![run_id.as_str()],
        )
        .expect("clear prior artifact");
}

fn mutate_artifact_payload(
    database: &Path,
    run_id: &WorkflowRunId,
    mutate: impl FnOnce(&mut Value),
) -> String {
    let payload = read_artifact_payload(database, run_id);
    let mut value: Value = serde_json::from_str(&payload).expect("artifact payload JSON");
    mutate(&mut value);
    let payload = value.to_string();
    write_artifact_payload(database, run_id, &payload);
    payload
}

fn read_artifact_payload(database: &Path, run_id: &WorkflowRunId) -> String {
    Connection::open(database)
        .expect("open fixture database for artifact read")
        .query_row(
            "SELECT payload FROM work_report_artifacts WHERE run_id = ?1",
            params![run_id.as_str()],
            |row| row.get(0),
        )
        .expect("artifact payload")
}

fn write_artifact_payload(database: &Path, run_id: &WorkflowRunId, payload: &str) {
    Connection::open(database)
        .expect("open fixture database for artifact mutation")
        .execute(
            "UPDATE work_report_artifacts SET payload = ?1 WHERE run_id = ?2",
            params![payload, run_id.as_str()],
        )
        .expect("mutate artifact payload");
}

fn assert_artifact_persistence_failure(output: &Output) {
    assert!(!output.status.success());
    assert_eq!(output.stdout, Vec::<u8>::new());
    assert_eq!(
        String::from_utf8(output.stderr.clone()).expect("bounded artifact failure stderr"),
        "trusted_host_local_application: admission.terminal_replay_report_artifact_persistence_failed\n"
    );
}

fn clear_artifacts_and_swap_event_order(
    database: &Path,
    run_id: &WorkflowRunId,
    first_kind: &str,
    second_kind: &str,
) {
    let mut connection = Connection::open(database).expect("open fixture database for order test");
    clear_artifacts_with_connection(&connection, run_id);
    let (first_id, first_payload) = find_event_payload(&connection, run_id, first_kind);
    let (second_id, second_payload) = find_event_payload(&connection, run_id, second_kind);
    let mut first_value: Value =
        serde_json::from_str(&first_payload).expect("first event payload JSON");
    let mut second_value: Value =
        serde_json::from_str(&second_payload).expect("second event payload JSON");
    let first_sequence = i64::try_from(
        first_value["sequence_number"]
            .as_u64()
            .expect("first event sequence"),
    )
    .expect("first sequence fits SQLite");
    let second_sequence = i64::try_from(
        second_value["sequence_number"]
            .as_u64()
            .expect("second event sequence"),
    )
    .expect("second sequence fits SQLite");
    assert!(first_sequence < second_sequence);
    first_value["sequence_number"] = json!(second_sequence);
    second_value["sequence_number"] = json!(first_sequence);
    let temporary_sequence: i64 = connection
        .query_row(
            "SELECT MAX(sequence_number) + 100 FROM events WHERE run_id = ?1",
            params![run_id.as_str()],
            |row| row.get(0),
        )
        .expect("temporary sequence");
    let transaction = connection.transaction().expect("corruption transaction");
    transaction
        .execute(
            "UPDATE events SET sequence_number = ?1 WHERE event_id = ?2",
            params![temporary_sequence, first_id],
        )
        .expect("move first event temporarily");
    transaction
        .execute(
            "UPDATE events SET sequence_number = ?1, payload = ?2 WHERE event_id = ?3",
            params![first_sequence, second_value.to_string(), second_id],
        )
        .expect("move second event first");
    transaction
        .execute(
            "UPDATE events SET sequence_number = ?1, payload = ?2 WHERE event_id = ?3",
            params![second_sequence, first_value.to_string(), first_id],
        )
        .expect("move first event second");
    transaction.commit().expect("commit corrupt event order");
}

fn find_event_payload(
    connection: &Connection,
    run_id: &WorkflowRunId,
    target_kind: &str,
) -> (String, String) {
    let mut statement = connection
        .prepare("SELECT event_id, payload FROM events WHERE run_id = ?1 ORDER BY sequence_number")
        .expect("prepare event lookup");
    let found = statement
        .query_map(params![run_id.as_str()], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .expect("query event payloads")
        .map(|row| row.expect("event payload row"))
        .find(|(_, payload)| {
            serde_json::from_str::<Value>(payload)
                .ok()
                .and_then(|value| value["kind"]["kind"].as_str().map(str::to_owned))
                .as_deref()
                == Some(target_kind)
        });
    found.expect("target event kind")
}

fn assert_corrupt_replay_fails_before_artifact(
    fixture: &Fixture,
    repository_root: &Path,
    governance_root: &Path,
    run_id: &WorkflowRunId,
    bundle_id: &ImmutableRunBundleId,
    observed_at: &str,
    expires_at: &str,
) {
    let replay = invoke_foreground(
        fixture,
        repository_root,
        governance_root,
        run_id,
        bundle_id,
        observed_at,
        expires_at,
    );
    assert!(!replay.status.success());
    assert_eq!(replay.stdout, Vec::<u8>::new());
    let stderr = String::from_utf8(replay.stderr).expect("bounded corrupt replay stderr");
    assert!(matches!(
        stderr.as_str(),
        "trusted_host_local_application: admission.terminal_replay_report_generation_failed\n"
            | "trusted_host_local_application: trusted_host_local_application.failure.invalid_state\n"
    ));
    let backend = SqliteStateBackend::open(&fixture.database).expect("reopen SQLite");
    assert_eq!(
        backend
            .list_work_report_artifacts(run_id)
            .expect("artifact list remains readable")
            .len(),
        0
    );
}

#[test]
fn foreground_binary_fails_closed_when_required_output_is_missing() {
    let fixture = Fixture::new();
    let repository_root = repository_root();
    let governance_root = fixture.root.join("output-contract-project");
    write_output_contract_project(&repository_root, &governance_root);
    let (observed_at, expires_at) = window_times();
    let run_id = fixture.run_id("missing-output");
    let bundle_id = fixture.bundle_id("missing-output");

    let output = invoke_foreground(
        &fixture,
        &repository_root,
        &governance_root,
        &run_id,
        &bundle_id,
        &observed_at,
        &expires_at,
    );

    assert!(!output.status.success());
    assert_eq!(output.stdout, Vec::<u8>::new());
    assert_eq!(
        String::from_utf8(output.stderr).expect("bounded generation failure stderr"),
        "trusted_host_local_application: operation.terminal_report_generation_failed\n"
    );
    let backend = SqliteStateBackend::open(&fixture.database).expect("reopen SQLite");
    let run = backend.rehydrate_run(&run_id).expect("durable failed run");
    assert_eq!(run.snapshot.status, WorkflowRunStatus::Failed);
    let failure = run
        .events
        .iter()
        .find_map(|event| match &event.kind {
            WorkflowRunEventKind::SkillInvocationFailed { failure, .. } => Some(failure),
            _ => None,
        })
        .expect("durable invocation failure");
    assert_eq!(failure.code, "executor.output.required_missing");
    assert_eq!(
        backend
            .list_work_report_artifacts(&run_id)
            .expect("artifact list remains readable")
            .len(),
        0
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
    assert!(
        initial.status.success(),
        "foreground stderr: {}",
        String::from_utf8_lossy(&initial.stderr)
    );
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
    assert!(
        initial.status.success(),
        "foreground stderr: {}",
        String::from_utf8_lossy(&initial.stderr)
    );

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

fn write_output_contract_project(repository_root: &Path, project_root: &Path) {
    fs::create_dir_all(project_root.join("workflows")).expect("workflow directory");
    fs::create_dir_all(project_root.join("skills")).expect("skill directory");
    fs::create_dir_all(project_root.join("policies")).expect("policy directory");
    fs::create_dir_all(project_root.join("tests")).expect("tests directory");
    let source = repository_root.join("dogfood/workflow-os-self-governance");
    fs::copy(
        source.join("workflow-os.yml"),
        project_root.join("workflow-os.yml"),
    )
    .expect("manifest fixture");
    fs::copy(
        source.join("workflows/trusted-host-docs-check.workflow.yml"),
        project_root.join("workflows/trusted-host-docs-check.workflow.yml"),
    )
    .expect("workflow fixture");
    fs::copy(
        source.join("policies/self-governance-local.policy.yml"),
        project_root.join("policies/self-governance-local.policy.yml"),
    )
    .expect("policy fixture");
    let skill =
        fs::read_to_string(source.join("skills/check-docs.skill.yml")).expect("skill fixture");
    let skill = skill.replace(
        "    - name: local_check_status\n      field_type: string\n      description: Bounded local check status.\n  required:",
        "    - name: local_check_status\n      field_type: string\n      description: Bounded local check status.\n    - name: required_but_missing\n      field_type: string\n      description: Required output omitted by the fixture handler.\n  required:",
    );
    let skill = skill.replace(
        "  required:\n    - summary\n    - local_check_status\nallowed_capabilities:",
        "  required:\n    - summary\n    - local_check_status\n    - required_but_missing\nallowed_capabilities:",
    );
    fs::write(project_root.join("skills/check-docs.skill.yml"), skill)
        .expect("write skill fixture");
}

fn write_project_validation_project(project_root: &Path) {
    fs::create_dir_all(project_root.join("workflows")).expect("workflow directory");
    fs::create_dir_all(project_root.join("skills")).expect("skill directory");
    fs::create_dir_all(project_root.join("policies")).expect("policy directory");
    fs::create_dir_all(project_root.join("tests")).expect("tests directory");
    fs::write(
        project_root.join("workflow-os.yml"),
        PROJECT_VALIDATION_MANIFEST,
    )
    .expect("manifest");
    fs::write(
        project_root.join("workflows/project-validation.workflow.yml"),
        PROJECT_VALIDATION_WORKFLOW,
    )
    .expect("workflow");
    fs::write(
        project_root.join("skills/project-validation.skill.yml"),
        PROJECT_VALIDATION_SKILL,
    )
    .expect("skill");
    fs::write(
        project_root.join("policies/local.policy.yml"),
        "schema_version: workflowos.dev/v0\nid: local/d\nname: Local\ndescription: Bounded local policy.\nrules:\n  - id: allow-local\n    effect: allow_local\n",
    )
    .expect("policy");
}

fn write_two_step_project(project_root: &Path) {
    fs::create_dir_all(project_root.join("workflows")).expect("workflow directory");
    fs::create_dir_all(project_root.join("skills")).expect("skill directory");
    fs::create_dir_all(project_root.join("policies")).expect("policy directory");
    fs::create_dir_all(project_root.join("tests")).expect("tests directory");
    fs::write(
        project_root.join("workflow-os.yml"),
        PROJECT_VALIDATION_MANIFEST,
    )
    .expect("manifest");
    fs::write(
        project_root.join("workflows/two-step.workflow.yml"),
        TWO_STEP_WORKFLOW,
    )
    .expect("workflow");
    fs::write(
        project_root.join("skills/project-validation.skill.yml"),
        PROJECT_VALIDATION_SKILL,
    )
    .expect("project validation skill");
    fs::write(
        project_root.join("skills/docs-check.skill.yml"),
        DOCS_CHECK_SKILL,
    )
    .expect("docs check skill");
    fs::write(
        project_root.join("policies/local.policy.yml"),
        "schema_version: workflowos.dev/v0\nid: local/d\nname: Local\ndescription: Bounded local policy.\nrules:\n  - id: allow-local\n    effect: allow_local\n",
    )
    .expect("policy");
}

const PROJECT_VALIDATION_MANIFEST: &str = r"schema_version: workflowos.dev/v0
project:
  id: test/trusted-host-project-validation
  name: Trusted Host Project Validation
  description: Bounded project-validation fixture.
layout:
  workflows: workflows
  skills: skills
  policies: policies
  tests: tests
";

const PROJECT_VALIDATION_WORKFLOW: &str = r"schema_version: workflowos.dev/v0
id: test/trusted-host-project-validation
version: v0
display_name: Trusted Host Project Validation
description: One-step project validation fixture.
owner:
  owning_team: workflow-os-maintainers
  maintainer: workflow-os
  escalation_contact: workflow-os
  lifecycle_status: experimental
autonomy_level: level_1
triggers:
  - id: manual-start
    kind: manual
state_model:
  type: inline
  states: [requested, checked]
steps:
  - id: project-validation
    skill_ref:
      id: local/workflow-os-validate
      version: v0
    input_mapping:
      - from:
          type: literal
          value: trusted-host-project-validation
        to: request
    policy_requirements:
      - id: local/d
    idempotency_key_strategy:
      type: derived
    timeout:
      duration: 5m
    terminal_behavior: fail_workflow
cancellation_behavior: stop
audit_requirements:
  required: true
  events: [RunCreated, SkillInvocationRequested, RunCompleted]
  store_references_only: true
observability_requirements:
  metrics: [workflow_latency]
  tracing: true
  latency_tracking: true
";

const PROJECT_VALIDATION_SKILL: &str = r"schema_version: workflowos.dev/v0
id: local/workflow-os-validate
version: v0
display_name: Validate Workflow OS Project
description: Explicit local project-validation skill.
owner:
  owning_team: workflow-os-maintainers
  maintainer: workflow-os
  escalation_contact: workflow-os
  lifecycle_status: experimental
input_contract:
  fields:
    - name: request
      field_type: string
      description: Non-secret validation request marker.
  required: [request]
output_contract:
  fields:
    - name: summary
      field_type: string
      description: Bounded local check summary.
    - name: local_check_status
      field_type: string
      description: Bounded local check status.
  required: [summary, local_check_status]
allowed_capabilities:
  - name: local.read
failure_modes:
  - code: project_validation_failed
    description: Project validation failed.
    retryable: false
evaluation_criteria:
  - name: explicit_handler_only
    description: Execute only through the selected closed handler.
retry_compatibility: requires_policy
approval_sensitivity: low
audit_requirements:
  required: true
  events: [SkillInvocationRequested, SkillInvocationSucceeded]
  store_references_only: true
observability_requirements:
  metrics: [skill_latency]
  tracing: true
  latency_tracking: true
";

const DOCS_CHECK_SKILL: &str = r"schema_version: workflowos.dev/v0
id: local/check-docs
version: v0
display_name: Check Documentation
description: Explicit local documentation-check skill.
owner:
  owning_team: workflow-os-maintainers
  maintainer: workflow-os
  escalation_contact: workflow-os
  lifecycle_status: experimental
input_contract:
  fields:
    - name: request
      field_type: string
      description: Non-secret documentation request marker.
  required: [request]
output_contract:
  fields:
    - name: summary
      field_type: string
      description: Bounded local check summary.
    - name: local_check_status
      field_type: string
      description: Bounded local check status.
  required: [summary, local_check_status]
allowed_capabilities:
  - name: local.read
failure_modes:
  - code: docs_check_failed
    description: Documentation check failed.
    retryable: false
evaluation_criteria:
  - name: explicit_handler_only
    description: Execute only through the selected closed handler.
retry_compatibility: requires_policy
approval_sensitivity: low
audit_requirements:
  required: true
  events: [SkillInvocationRequested, SkillInvocationSucceeded]
  store_references_only: true
observability_requirements:
  metrics: [skill_latency]
  tracing: true
  latency_tracking: true
";

const TWO_STEP_WORKFLOW: &str = r"schema_version: workflowos.dev/v0
id: test/trusted-host-two-step
version: v0
display_name: Trusted Host Two Step
description: Exact two-step no-write trusted-host fixture.
owner:
  owning_team: workflow-os-maintainers
  maintainer: workflow-os
  escalation_contact: workflow-os
  lifecycle_status: experimental
autonomy_level: level_1
triggers:
  - id: manual-start
    kind: manual
state_model:
  type: inline
  states: [requested, validated, checked]
steps:
  - id: project-validation
    skill_ref:
      id: local/workflow-os-validate
      version: v0
    input_mapping:
      - from:
          type: literal
          value: trusted-host-two-step
        to: request
    policy_requirements:
      - id: local/d
    idempotency_key_strategy:
      type: derived
    timeout:
      duration: 5m
    terminal_behavior: continue
  - id: docs-check
    skill_ref:
      id: local/check-docs
      version: v0
    input_mapping:
      - from:
          type: literal
          value: trusted-host-two-step
        to: request
    policy_requirements:
      - id: local/d
    idempotency_key_strategy:
      type: derived
    timeout:
      duration: 5m
    terminal_behavior: fail_workflow
cancellation_behavior: stop
audit_requirements:
  required: true
  events: [RunCreated, SkillInvocationRequested, SkillInvocationSucceeded, RunCompleted]
  store_references_only: true
observability_requirements:
  metrics: [workflow_latency]
  tracing: true
  latency_tracking: true
";

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
    foreground_command_with(input)
        .output()
        .expect("foreground binary")
}

fn foreground_command_with(input: ForegroundInvocation<'_>) -> Command {
    let mut command = base_foreground_command(
        input.fixture,
        input.governance_root,
        input.run_id,
        input.bundle_id,
        input.observed_at,
        input.expires_at,
    );
    command.args([
        "--repository-root",
        input.repository_root.to_str().expect("repository root"),
        "--npm-executable",
        input.npm_executable.to_str().expect("fake npm"),
        "--workflow-id",
        input.workflow_id,
        "--step-id",
        input.step_id,
        "--actor",
        input.actor,
        "--correlation-id",
        input.correlation_id,
    ]);
    command
}

fn invoke_project_validation(
    fixture: &Fixture,
    governance_root: &Path,
    run_id: &WorkflowRunId,
    bundle_id: &ImmutableRunBundleId,
    observed_at: &str,
    expires_at: &str,
) -> Output {
    base_foreground_command(
        fixture,
        governance_root,
        run_id,
        bundle_id,
        observed_at,
        expires_at,
    )
    .args([
        "--profile",
        "workflow-os-project-validation",
        "--workflow-os-executable",
        fixture.workflow_os.to_str().expect("workflow-os path"),
        "--workflow-id",
        "test/trusted-host-project-validation",
        "--step-id",
        "project-validation",
        "--actor",
        "system/local-host-project-validation-test",
        "--correlation-id",
        "correlation-local-host-project-validation-test",
    ])
    .output()
    .expect("foreground binary")
}

fn invoke_two_step(
    fixture: &Fixture,
    governance_root: &Path,
    run_id: &WorkflowRunId,
    bundle_id: &ImmutableRunBundleId,
    observed_at: &str,
    expires_at: &str,
) -> Output {
    two_step_command(
        fixture,
        governance_root,
        run_id,
        bundle_id,
        observed_at,
        expires_at,
    )
    .output()
    .expect("foreground binary")
}

fn two_step_command(
    fixture: &Fixture,
    governance_root: &Path,
    run_id: &WorkflowRunId,
    bundle_id: &ImmutableRunBundleId,
    observed_at: &str,
    expires_at: &str,
) -> Command {
    let mut command = base_foreground_command(
        fixture,
        governance_root,
        run_id,
        bundle_id,
        observed_at,
        expires_at,
    );
    command.args([
        "--mode",
        "two-step-no-write",
        "--repository-root",
        repository_root().to_str().expect("repository root"),
        "--npm-executable",
        fixture.npm.to_str().expect("fake npm"),
        "--workflow-os-executable",
        fixture.workflow_os.to_str().expect("workflow-os path"),
        "--workflow-id",
        "test/trusted-host-two-step",
        "--actor",
        "system/local-host-two-step-test",
        "--correlation-id",
        "correlation-local-host-two-step-test",
    ]);
    command
}

fn local_check_citation_count(report: &workflow_core::WorkReport) -> usize {
    report
        .sections()
        .iter()
        .flat_map(workflow_core::WorkReportSection::citations)
        .filter(|citation| {
            matches!(
                citation.target(),
                workflow_core::WorkReportCitationTarget::LocalCheckResult { .. }
            )
        })
        .count()
}

fn base_foreground_command(
    fixture: &Fixture,
    governance_root: &Path,
    run_id: &WorkflowRunId,
    bundle_id: &ImmutableRunBundleId,
    observed_at: &str,
    expires_at: &str,
) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_workflow-local-host"));
    command.args([
        "--governance-project-root",
        governance_root.to_str().expect("governance root"),
        "--state-database",
        fixture.database.to_str().expect("database"),
        "--run-id",
        run_id.as_str(),
        "--bundle-id",
        bundle_id.as_str(),
        "--observed-at",
        observed_at,
        "--expires-at",
        expires_at,
    ]);
    command
}
