#![allow(clippy::expect_used)]
//! Jira read-only reference example contract tests.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

use workflow_core::{
    jira_actions, jira_read_request, ActorId, AdapterCapability, AdapterOperationMode,
    AdapterPolicyPrecheck, AdapterResponseStatus, CorrelationId, JiraFixtureClient,
    JiraReadOnlyAdapter, JiraReadOnlyConfig,
};

static NEXT_STATE: AtomicU64 = AtomicU64::new(1);

fn example_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("examples")
        .join("jira-read-only-intake-quality")
}

fn state_root(name: &str) -> PathBuf {
    let id = NEXT_STATE.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!(
        "workflow-os-jira-read-only-{name}-{}-{id}",
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
        &["--mock-all-local-skills", "run", "ex/jira"],
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
            "reviewed fixture intake",
        ],
    );

    assert!(approved.status.success(), "{}", stderr(&approved));
    assert!(stdout(&approved).contains("status: Completed"));
}

#[test]
fn completed_run_can_be_inspected() {
    let state = state_root("inspect");
    let waiting = workflow_os(
        &example_root(),
        &state,
        &["--mock-all-local-skills", "run", "ex/jira"],
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
    let client = JiraFixtureClient::new().with_json(
        "/rest/api/3/issue/OPS-42",
        r#"{"key":"OPS-42","self":"https://example.atlassian.net/rest/api/3/issue/10042","fields":{"summary":"Clarify intake","description":{"type":"doc"},"status":{"name":"Needs Review"},"priority":{"name":"Medium"},"labels":["intake"],"assignee":{"displayName":"Avery"},"reporter":{"displayName":"Riley"}}}"#,
    );
    let adapter = JiraReadOnlyAdapter::new(
        JiraReadOnlyConfig::fixture().expect("fixture config"),
        client,
    );
    let request = jira_read_request(
        jira_actions::ISSUE_METADATA,
        ActorId::new("system/jira-example-test").expect("actor"),
        CorrelationId::new("correlation/jira-example-test").expect("correlation"),
        [("issue_key".to_owned(), "OPS-42".to_owned())]
            .into_iter()
            .collect(),
        AdapterOperationMode::Fixture,
        AdapterPolicyPrecheck::fixture_test_allowed(vec!["policy.fixture.read".to_owned()]),
    )
    .expect("request");

    let outcome = adapter
        .read_issue_metadata(&request, "OPS-42")
        .expect("adapter read");

    assert_eq!(outcome.invocation.capability, AdapterCapability::JiraRead);
    assert_eq!(outcome.invocation.status, AdapterResponseStatus::Success);
    assert_eq!(outcome.observability.status, AdapterResponseStatus::Success);
}

#[test]
fn no_write_action_occurs_or_is_declared() {
    let root = example_root();
    for path in [
        "workflow-os.yml",
        "workflows/intake-quality.workflow.yml",
        "skills/jira-intake-quality.skill.yml",
        "policies/jira-read-only.policy.yml",
        "tests/jira-intake-quality.test.yml",
    ] {
        let content = fs::read_to_string(root.join(path)).expect("example file readable");
        assert!(!content.contains("jira.write"));
        assert!(!content.contains("external.write"));
        assert!(!content.contains("issue.update"));
    }
}

#[test]
fn missing_fixture_fails_clearly() {
    let source = example_root();
    let project = state_root("missing-fixture-project");
    copy_dir(&source, &project);
    fs::remove_file(project.join("fixtures/jira/issue-OPS-42.json")).expect("fixture removed");
    let state = state_root("missing-fixture-state");
    let waiting = workflow_os(
        &project,
        &state,
        &["--mock-all-local-skills", "run", "ex/jira"],
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
    assert!(stdout(&inspected).contains("jira.fixture.missing"));
}

#[test]
fn live_mode_is_skipped_by_default() {
    let config = JiraReadOnlyConfig::fixture().expect("fixture config");

    assert!(!config.credential_present());
}
