#![allow(clippy::expect_used)]
//! CI read-only reference example contract tests.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

use workflow_core::{
    ci_actions, github_actions_read_request, ActorId, AdapterCapability, AdapterOperationMode,
    AdapterPolicyPrecheck, AdapterPolicyPrecheckProvenance, AdapterResponseStatus,
    AdapterTelemetryStore, CorrelationId, GitHubActionsFixtureClient, GitHubActionsReadOnlyAdapter,
    GitHubActionsReadOnlyConfig, LocalStateBackend, WorkflowRunId,
};

static NEXT_STATE: AtomicU64 = AtomicU64::new(1);

fn example_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("examples")
        .join("ci-read-only-failure-summary")
}

fn state_root(name: &str) -> PathBuf {
    let id = NEXT_STATE.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!(
        "workflow-os-ci-read-only-{name}-{}-{id}",
        std::process::id()
    ));
    if root.exists() {
        fs::remove_dir_all(&root).expect("stale state cleanup succeeds");
    }
    root
}

fn workflow_os(project: &Path, state: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_workflow-os"))
        .arg("--project-dir")
        .arg(project)
        .arg("--state-dir")
        .arg(state)
        .args(args)
        .output()
        .expect("workflow-os command runs")
}

fn stdout(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).expect("stdout is utf8")
}

fn stderr(output: &Output) -> String {
    String::from_utf8(output.stderr.clone()).expect("stderr is utf8")
}

fn run_id(output: &Output) -> String {
    stdout(output)
        .lines()
        .find_map(|line| line.strip_prefix("run_id: "))
        .expect("run id is printed")
        .to_owned()
}

fn approval_id(output: &Output) -> String {
    stdout(output)
        .lines()
        .find_map(|line| line.strip_prefix("approval_id: "))
        .expect("approval id is printed")
        .to_owned()
}

fn copy_dir(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("destination created");
    for entry in fs::read_dir(source).expect("source read") {
        let entry = entry.expect("entry");
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        if source_path.is_dir() {
            copy_dir(&source_path, &destination_path);
        } else {
            fs::copy(&source_path, &destination_path).expect("file copied");
        }
    }
}

fn fixture_metadata() -> BTreeMap<String, String> {
    BTreeMap::from([
        ("owner".to_owned(), "acme".to_owned()),
        ("repo".to_owned(), "widgets".to_owned()),
        ("run_id".to_owned(), "12345".to_owned()),
        ("job_id".to_owned(), "777".to_owned()),
        ("ref".to_owned(), "abc123".to_owned()),
    ])
}

#[test]
fn example_validates() {
    let state = state_root("validate");
    let output = workflow_os(&example_root(), &state, &["validate"]);

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(stdout(&output).contains("Project is valid."));
}

#[test]
fn example_runs_against_fixture_adapter() {
    let state = state_root("run");
    let waiting = workflow_os(
        &example_root(),
        &state,
        &["--mock-all-local-skills", "run", "ex/ci"],
    );
    assert!(waiting.status.success(), "{}", stderr(&waiting));
    assert!(stdout(&waiting).contains("status: WaitingForApproval"));
    let run_id = run_id(&waiting);
    let approval_id = approval_id(&waiting);

    let approved = workflow_os(
        &example_root(),
        &state,
        &[
            "--mock-all-local-skills",
            "approve",
            &run_id,
            &approval_id,
            "--actor",
            "user/example-reviewer",
            "--reason",
            "reviewed fixture ci context",
        ],
    );

    assert!(approved.status.success(), "{}", stderr(&approved));
    assert!(stdout(&approved).contains("status: Completed"));
}

#[test]
fn fixture_example_persists_runtime_adapter_telemetry() {
    let state = state_root("adapter-telemetry");
    let waiting = workflow_os(
        &example_root(),
        &state,
        &["--mock-all-local-skills", "run", "ex/ci"],
    );
    let run_id = run_id(&waiting);
    let approval_id = approval_id(&waiting);
    let approved = workflow_os(
        &example_root(),
        &state,
        &["--mock-all-local-skills", "approve", &run_id, &approval_id],
    );
    assert!(approved.status.success(), "{}", stderr(&approved));

    let backend = LocalStateBackend::new(&state).expect("backend opens");
    let run_id = WorkflowRunId::new(&run_id).expect("run id");
    let audit = backend
        .read_adapter_audit_records(&run_id)
        .expect("adapter audit");
    let observability = backend
        .read_adapter_observability_records(&run_id)
        .expect("adapter observability");

    assert_eq!(audit.len(), 5);
    assert_eq!(observability.len(), 5);
    let first = audit.first().expect("first adapter audit");
    assert_eq!(first.adapter_kind, workflow_core::AdapterKind::Ci);
    assert_eq!(first.capability, AdapterCapability::CiRead);
    assert_eq!(first.operation_mode, AdapterOperationMode::Fixture);
    assert_eq!(first.workflow_run_id.as_ref(), Some(&run_id));
    assert!(matches!(
        first.policy_precheck,
        AdapterPolicyPrecheck::Allowed {
            provenance: AdapterPolicyPrecheckProvenance::FixtureTest,
            ..
        }
    ));
    let audit_debug = format!("{audit:?}");
    assert!(!audit_debug.contains("ghs_secret_token"));
    assert!(!audit_debug.contains("hunter2"));

    let inspected = workflow_os(&example_root(), &state, &["inspect", run_id.as_str()]);
    assert!(inspected.status.success(), "{}", stderr(&inspected));
    assert!(stdout(&inspected).contains("adapter_telemetry: 5"));
}

#[test]
fn completed_run_can_be_inspected() {
    let state = state_root("inspect");
    let waiting = workflow_os(
        &example_root(),
        &state,
        &["--mock-all-local-skills", "run", "ex/ci"],
    );
    let run_id = run_id(&waiting);
    let approval_id = approval_id(&waiting);
    let approved = workflow_os(
        &example_root(),
        &state,
        &["--mock-all-local-skills", "approve", &run_id, &approval_id],
    );
    assert!(approved.status.success(), "{}", stderr(&approved));

    let inspected = workflow_os(&example_root(), &state, &["inspect", &run_id]);

    assert!(inspected.status.success(), "{}", stderr(&inspected));
    assert!(stdout(&inspected).contains("SkillInvocationSucceeded"));
    assert!(stdout(&inspected).contains("status: Completed"));
}

#[test]
fn adapter_read_audit_and_observability_records_are_emitted_by_adapter() {
    let client = GitHubActionsFixtureClient::new().with_json(
        "/repos/acme/widgets/actions/runs/12345",
        r#"{"id":12345,"name":"CI","status":"completed","conclusion":"failure","html_url":"https://github.com/acme/widgets/actions/runs/12345"}"#,
    );
    let adapter = GitHubActionsReadOnlyAdapter::new(
        GitHubActionsReadOnlyConfig::fixture().expect("fixture config"),
        client,
    );
    let request = github_actions_read_request(
        ci_actions::WORKFLOW_RUN_METADATA,
        ActorId::new("system/ci-example-test").expect("actor"),
        CorrelationId::new("correlation/ci-example-test").expect("correlation"),
        fixture_metadata(),
        AdapterOperationMode::Fixture,
        AdapterPolicyPrecheck::fixture_test_allowed(vec!["policy.fixture.read".to_owned()]),
    )
    .expect("request");

    let outcome = adapter
        .read_workflow_run_metadata(&request, "acme", "widgets", "12345")
        .expect("adapter read");

    assert_eq!(outcome.invocation.capability, AdapterCapability::CiRead);
    assert_eq!(outcome.invocation.status, AdapterResponseStatus::Success);
    assert_eq!(outcome.observability.status, AdapterResponseStatus::Success);
}

#[test]
fn no_write_or_rerun_action_occurs_or_is_declared() {
    let root = example_root();
    for path in [
        "workflow-os.yml",
        "workflows/failure-summary.workflow.yml",
        "skills/ci-failure-summary.skill.yml",
        "policies/ci-read-only.policy.yml",
        "tests/ci-failure-summary.test.yml",
    ] {
        let content = fs::read_to_string(root.join(path)).expect("example file readable");
        assert!(!content.contains("ci.write"));
        assert!(!content.contains("ci.rerun"));
        assert!(!content.contains("external.write"));
        assert!(!content.contains("workflow.dispatch"));
        assert!(!content.contains("adapter.write"));
    }
}

#[test]
fn log_redaction_works() {
    let client = GitHubActionsFixtureClient::new().with_text(
        "/repos/acme/widgets/actions/jobs/777/logs",
        "safe context\nTOKEN=ghs_secret_token\npassword=hunter2\n",
    );
    let adapter = GitHubActionsReadOnlyAdapter::new(
        GitHubActionsReadOnlyConfig::fixture().expect("fixture config"),
        client,
    );
    let request = github_actions_read_request(
        ci_actions::LOG_EXCERPT,
        ActorId::new("system/ci-example-test").expect("actor"),
        CorrelationId::new("correlation/ci-example-test").expect("correlation"),
        fixture_metadata(),
        AdapterOperationMode::Fixture,
        AdapterPolicyPrecheck::fixture_test_allowed(vec!["policy.fixture.read".to_owned()]),
    )
    .expect("request");

    let response = adapter
        .read_log_excerpt(&request, "acme", "widgets", "777")
        .expect("log excerpt");

    assert!(response.response.summary.contains("safe context"));
    assert!(response.response.summary.contains("[REDACTED_LOG_LINE]"));
    assert!(!response.response.summary.contains("ghs_secret_token"));
    assert!(!response.response.summary.contains("hunter2"));
}

#[test]
fn missing_fixture_fails_clearly() {
    let source = example_root();
    let project = state_root("missing-fixture-project");
    copy_dir(&source, &project);
    fs::remove_file(project.join("fixtures/github-actions/run-12345-jobs.json"))
        .expect("fixture removed");
    let state = state_root("missing-fixture-state");
    let waiting = workflow_os(
        &project,
        &state,
        &["--mock-all-local-skills", "run", "ex/ci"],
    );
    let run_id = run_id(&waiting);
    let approval_id = approval_id(&waiting);

    let approved = workflow_os(
        &project,
        &state,
        &["--mock-all-local-skills", "approve", &run_id, &approval_id],
    );

    assert!(approved.status.success(), "{}", stderr(&approved));
    assert!(stdout(&approved).contains("status: Failed"));
    let inspected = workflow_os(&project, &state, &["inspect", &run_id]);
    assert!(stdout(&inspected).contains("ci.fixture.missing"));
}

#[test]
fn live_mode_is_skipped_by_default() {
    let config = GitHubActionsReadOnlyConfig::fixture().expect("fixture config");

    assert!(!config.credential_present());
}
