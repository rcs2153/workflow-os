#![allow(clippy::expect_used)]
//! GitHub read-only reference example contract tests.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

use workflow_core::{
    github_actions, github_read_request, ActorId, AdapterCapability, AdapterOperationMode,
    AdapterPolicyPrecheck, AdapterPolicyPrecheckProvenance, AdapterResponseStatus,
    AdapterTelemetryStore, CorrelationId, GitHubFixtureClient, GitHubReadOnlyAdapter,
    GitHubReadOnlyConfig, LocalStateBackend, WorkflowRunId,
};

static NEXT_STATE: AtomicU64 = AtomicU64::new(1);

fn example_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("examples")
        .join("github-read-only-review-context")
}

fn repository_metadata_fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("examples")
        .join("github-repository-metadata-fixture")
}

fn state_root(name: &str) -> PathBuf {
    let id = NEXT_STATE.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!(
        "workflow-os-github-read-only-{name}-{}-{id}",
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

fn rewrite_manifest(project: &Path, transform: impl FnOnce(String) -> String) {
    let path = project.join("workflow-os.yml");
    let source = fs::read_to_string(&path).expect("project manifest read");
    fs::write(path, transform(source)).expect("project manifest rewritten");
}

fn assert_no_provider_entry(project: &Path, state: &Path, run_id: &str) {
    let inspect = workflow_os(project, state, &["inspect", run_id]);
    assert!(inspect.status.success(), "{}", stderr(&inspect));
    let output = stdout(&inspect);
    assert!(!output.contains("ApprovalGranted"));
    assert!(!output.contains("SkillInvocationRequested"));
    assert!(!output.contains("adapter_telemetry:"));
}

fn assert_fixture_next_action_rejects_non_running(project: &Path, state: &Path, run_id: &str) {
    let inspect_before = workflow_os(project, state, &["inspect", run_id]);
    assert!(
        inspect_before.status.success(),
        "{}",
        stderr(&inspect_before)
    );
    let next_action = workflow_os(project, state, &["next-action", run_id]);
    assert!(!next_action.status.success());
    assert!(stderr(&next_action).contains("executor.governed_continuation.run.not_running"));
    assert!(!stderr(&next_action).contains("runtime.events.empty"));
    let inspect_after = workflow_os(project, state, &["inspect", run_id]);
    assert_eq!(stdout(&inspect_after), stdout(&inspect_before));
}

fn assert_fixture_running_next_action(project: &Path, state: &Path, run_id: &str) {
    let inspect_before = workflow_os(project, state, &["inspect", run_id]);
    assert!(
        inspect_before.status.success(),
        "{}",
        stderr(&inspect_before)
    );
    let next_action = workflow_os(project, state, &["next-action", run_id]);
    assert!(next_action.status.success(), "{}", stderr(&next_action));
    let output = stdout(&next_action);
    assert!(output.contains("allowed_next_action: invoke_current_step_skill"));
    assert!(output.contains("provider_operation: github.repository.metadata"));
    assert!(output.contains("provider_mode: fixture"));
    assert!(output.contains("live_transport: disabled"));

    let json = workflow_os(project, state, &["--json", "next-action", run_id]);
    assert!(json.status.success(), "{}", stderr(&json));
    let value: serde_json::Value =
        serde_json::from_str(stdout(&json).trim()).expect("next-action JSON parses");
    assert_eq!(value["allowed_next_action"], "invoke_current_step_skill");
    assert_eq!(value["provider_operation"], "github.repository.metadata");
    assert_eq!(value["provider_mode"], "fixture");
    assert_eq!(value["live_transport"], "disabled");

    let inspect_after = workflow_os(project, state, &["inspect", run_id]);
    assert_eq!(stdout(&inspect_after), stdout(&inspect_before));
}

#[test]
fn repository_metadata_fixture_completes_through_public_cli_without_network() {
    let state = state_root("repository-metadata-fixture");
    let project = repository_metadata_fixture_root();
    let waiting = workflow_os(
        &project,
        &state,
        &["run", "example/github-repository-metadata-fixture"],
    );
    assert!(waiting.status.success(), "{}", stderr(&waiting));
    let waiting_output = stdout(&waiting);
    assert!(waiting_output.contains("status: WaitingForApproval"));
    assert!(waiting_output.contains("provider_mode: fixture"));
    assert!(waiting_output.contains("live_transport: disabled"));
    assert!(waiting_output.contains("approval_handoff:"));
    assert!(waiting_output.contains("A WorkReport artifact, live transport"));
    let run_id = run_id(&waiting);
    let approval_id = approval_id(&waiting);
    assert_fixture_next_action_rejects_non_running(&project, &state, &run_id);

    let approved = workflow_os(
        &project,
        &state,
        &[
            "approve",
            &run_id,
            &approval_id,
            "--actor",
            "user/example-reviewer",
            "--reason",
            "reviewed exact fixture read",
        ],
    );
    assert!(approved.status.success(), "{}", stderr(&approved));
    assert!(stdout(&approved).contains("status: Running"));
    assert_fixture_running_next_action(&project, &state, &run_id);

    let completed = workflow_os(
        &project,
        &state,
        &[
            "run",
            "example/github-repository-metadata-fixture",
            "--run-id",
            &run_id,
        ],
    );
    assert!(completed.status.success(), "{}", stderr(&completed));
    assert!(stdout(&completed).contains("status: Completed"));
    assert!(stdout(&completed).contains("provider_posture: completed"));
    assert_fixture_next_action_rejects_non_running(&project, &state, &run_id);

    let status = workflow_os(&project, &state, &["status", &run_id]);
    assert!(status.status.success(), "{}", stderr(&status));
    assert!(stdout(&status).contains("provider_operation: github.repository.metadata"));
    assert!(stdout(&status).contains("terminal: true"));

    let inspect = workflow_os(&project, &state, &["inspect", &run_id]);
    assert!(inspect.status.success(), "{}", stderr(&inspect));
    let inspect_output = stdout(&inspect);
    assert!(inspect_output.contains("provider_mode: fixture"));
    assert!(inspect_output.contains("live_transport: disabled"));
    assert!(inspect_output.contains("adapter_telemetry: 1"));

    let replay = workflow_os(
        &project,
        &state,
        &[
            "run",
            "example/github-repository-metadata-fixture",
            "--run-id",
            &run_id,
        ],
    );
    assert!(replay.status.success(), "{}", stderr(&replay));
    assert!(stdout(&replay).contains("provider_posture: terminal_replay"));
}

#[test]
fn repository_metadata_fixture_supports_two_runs_in_one_state_and_terminal_replay_after_restart() {
    let state = state_root("repository-metadata-fixture-two-runs");
    let project = state_root("repository-metadata-fixture-two-runs-project");
    copy_dir(&repository_metadata_fixture_root(), &project);
    let mut completed_run_ids = Vec::new();

    for reason in ["reviewed first fixture run", "reviewed second fixture run"] {
        let waiting = workflow_os(
            &project,
            &state,
            &["run", "example/github-repository-metadata-fixture"],
        );
        assert!(waiting.status.success(), "{}", stderr(&waiting));
        let current_run_id = run_id(&waiting);
        let current_approval_id = approval_id(&waiting);

        let approved = workflow_os(
            &project,
            &state,
            &[
                "approve",
                &current_run_id,
                &current_approval_id,
                "--actor",
                "user/example-reviewer",
                "--reason",
                reason,
            ],
        );
        assert!(approved.status.success(), "{}", stderr(&approved));

        let completed = workflow_os(
            &project,
            &state,
            &[
                "run",
                "example/github-repository-metadata-fixture",
                "--run-id",
                &current_run_id,
            ],
        );
        assert!(completed.status.success(), "{}", stderr(&completed));
        assert!(stdout(&completed).contains("status: Completed"));
        completed_run_ids.push(current_run_id);
    }

    assert_ne!(completed_run_ids[0], completed_run_ids[1]);
    for completed_run_id in &completed_run_ids {
        let inspected = workflow_os(&project, &state, &["inspect", completed_run_id]);
        assert!(inspected.status.success(), "{}", stderr(&inspected));
        let output = stdout(&inspected);
        assert!(output.contains("status: Completed"));
        assert!(output.contains("adapter_telemetry: 1"));
    }

    fs::remove_file(project.join("fixtures/github/repository.json"))
        .expect("fixture removed after both runs completed");
    let replay = workflow_os(
        &project,
        &state,
        &[
            "run",
            "example/github-repository-metadata-fixture",
            "--run-id",
            &completed_run_ids[1],
        ],
    );
    assert!(replay.status.success(), "{}", stderr(&replay));
    assert!(stdout(&replay).contains("provider_posture: terminal_replay"));
}

#[test]
fn repository_metadata_fixture_change_after_presentation_fails_closed() {
    let state = state_root("repository-metadata-fixture-drift");
    let project = state_root("repository-metadata-fixture-project");
    copy_dir(&repository_metadata_fixture_root(), &project);
    let waiting = workflow_os(
        &project,
        &state,
        &["run", "example/github-repository-metadata-fixture"],
    );
    assert!(waiting.status.success(), "{}", stderr(&waiting));
    let run_id = run_id(&waiting);
    let approval_id = approval_id(&waiting);
    fs::write(
        project.join("fixtures/github/repository.json"),
        r#"{"full_name":"acme/changed","default_branch":"main","private":true,"html_url":"https://github.com/acme/changed"}"#,
    )
    .expect("fixture drift written");

    let approved = workflow_os(
        &project,
        &state,
        &[
            "approve",
            &run_id,
            &approval_id,
            "--actor",
            "user/example-reviewer",
            "--reason",
            "attempt approval after fixture drift",
        ],
    );
    assert!(!approved.status.success());
    assert!(stderr(&approved).contains("executor.approval.resume_context_mismatch"));
}

type ManifestMutation = fn(String) -> String;

fn project_restriction_drift_cases() -> [(&'static str, ManifestMutation, &'static str); 9] {
    [
        (
            "resource-replaced",
            |source| source.replace("repository: widgets", "repository: replacement"),
            "executor.approval.resume_context_mismatch",
        ),
        (
            "resource-broadened",
            |source| {
                source.replace(
                    "        - owner: acme\n          repository: widgets\n",
                    "        - owner: acme\n          repository: widgets\n        - owner: acme\n          repository: second\n",
                )
            },
            "executor.approval.resume_context_mismatch",
        ),
        (
            "resource-removed",
            |source| {
                source.replace(
                    "      resources:\n        - owner: acme\n          repository: widgets\n",
                    "      resources: []\n",
                )
            },
            "executor.project.load_failed",
        ),
        (
            "resource-duplicated",
            |source| {
                source.replace(
                    "        - owner: acme\n          repository: widgets\n",
                    "        - owner: acme\n          repository: widgets\n        - owner: acme\n          repository: widgets\n",
                )
            },
            "executor.project.load_failed",
        ),
        (
            "provider-disabled",
            |source| source.replace("      enabled: true", "      enabled: false"),
            "executor.project.load_failed",
        ),
        (
            "endpoint-changed",
            |source| source.replace("endpoint: github.com", "endpoint: github.example"),
            "executor.project.load_failed",
        ),
        (
            "access-reference-changed",
            |source| {
                source.replace(
                    "access_reference: env/WORKFLOW_OS_GITHUB_TOKEN",
                    "access_reference: env/WORKFLOW_OS_GITHUB_TOKEN_SECONDARY",
                )
            },
            "executor.project.load_failed",
        ),
        (
            "project-identity-changed",
            |source| {
                source.replace(
                    "id: examples/github-repository-metadata-fixture",
                    "id: examples/github-repository-metadata-fixture-replacement",
                )
            },
            "executor.approval.resume_context_mismatch",
        ),
        (
            "policy-changed",
            |source| source,
            "executor.approval.resume_context_mismatch",
        ),
    ]
}

#[test]
fn repository_metadata_project_restriction_drift_before_approval_fails_closed() {
    for (name, mutate_manifest, expected_code) in project_restriction_drift_cases() {
        let state = state_root(name);
        let project = state_root(&format!("{name}-project"));
        copy_dir(&repository_metadata_fixture_root(), &project);
        let waiting = workflow_os(
            &project,
            &state,
            &["run", "example/github-repository-metadata-fixture"],
        );
        assert!(waiting.status.success(), "{name}: {}", stderr(&waiting));
        let run_id = run_id(&waiting);
        let approval_id = approval_id(&waiting);
        if name == "policy-changed" {
            fs::write(
                project.join("policies/read.policy.yml"),
                "schema_version: workflowos.dev/v0\nid: external/read\nname: External Read Changed\nrules:\n  - id: allow\n    effect: allow_external_read\n",
            )
            .expect("policy changed");
        } else {
            rewrite_manifest(&project, mutate_manifest);
        }

        let approved = workflow_os(
            &project,
            &state,
            &[
                "approve",
                &run_id,
                &approval_id,
                "--actor",
                "user/example-reviewer",
                "--reason",
                "attempt approval after project restriction drift",
            ],
        );
        assert!(!approved.status.success(), "{name} unexpectedly approved");
        assert!(
            stderr(&approved).contains(expected_code),
            "{name}: {}",
            stderr(&approved)
        );
        assert_no_provider_entry(&project, &state, &run_id);
    }
}

#[test]
fn repository_metadata_project_restriction_drift_after_approval_blocks_provider_entry() {
    let state = state_root("project-restriction-post-approval-drift");
    let project = state_root("project-restriction-post-approval-drift-project");
    copy_dir(&repository_metadata_fixture_root(), &project);
    let waiting = workflow_os(
        &project,
        &state,
        &["run", "example/github-repository-metadata-fixture"],
    );
    assert!(waiting.status.success(), "{}", stderr(&waiting));
    let run_id = run_id(&waiting);
    let approval_id = approval_id(&waiting);
    let approved = workflow_os(
        &project,
        &state,
        &[
            "approve",
            &run_id,
            &approval_id,
            "--actor",
            "user/example-reviewer",
            "--reason",
            "reviewed exact project restriction",
        ],
    );
    assert!(approved.status.success(), "{}", stderr(&approved));
    assert!(stdout(&approved).contains("status: Running"));
    rewrite_manifest(&project, |source| {
        source.replace("repository: widgets", "repository: replacement")
    });

    let resumed = workflow_os(
        &project,
        &state,
        &[
            "run",
            "example/github-repository-metadata-fixture",
            "--run-id",
            &run_id,
        ],
    );
    assert!(!resumed.status.success());
    assert!(
        stderr(&resumed)
            .contains("executor.github_repository_metadata.atomic.project_restriction_mismatch"),
        "{}",
        stderr(&resumed)
    );
    let inspect = workflow_os(&project, &state, &["inspect", &run_id]);
    assert!(inspect.status.success(), "{}", stderr(&inspect));
    let output = stdout(&inspect);
    assert!(output.contains("ApprovalGranted"));
    assert!(!output.contains("SkillInvocationRequested"));
    assert!(!output.contains("adapter_telemetry:"));
}

#[test]
fn repository_metadata_project_restriction_uses_normalized_resource_order() {
    let state = state_root("project-restriction-normalized-order");
    let project = state_root("project-restriction-normalized-order-project");
    copy_dir(&repository_metadata_fixture_root(), &project);
    rewrite_manifest(&project, |source| {
        source.replace(
            "        - owner: acme\n          repository: widgets\n",
            "        - owner: acme\n          repository: second\n        - owner: acme\n          repository: widgets\n",
        )
    });
    let waiting = workflow_os(
        &project,
        &state,
        &["run", "example/github-repository-metadata-fixture"],
    );
    assert!(waiting.status.success(), "{}", stderr(&waiting));
    let run_id = run_id(&waiting);
    let approval_id = approval_id(&waiting);
    rewrite_manifest(&project, |source| {
        source.replace(
            "        - owner: acme\n          repository: second\n        - owner: acme\n          repository: widgets\n",
            "        - owner: acme\n          repository: widgets\n        - owner: acme\n          repository: second\n",
        )
    });

    let approved = workflow_os(
        &project,
        &state,
        &[
            "approve",
            &run_id,
            &approval_id,
            "--actor",
            "user/example-reviewer",
            "--reason",
            "reviewed normalized exact resource set",
        ],
    );
    assert!(approved.status.success(), "{}", stderr(&approved));
    let completed = workflow_os(
        &project,
        &state,
        &[
            "run",
            "example/github-repository-metadata-fixture",
            "--run-id",
            &run_id,
        ],
    );
    assert!(completed.status.success(), "{}", stderr(&completed));
    assert!(stdout(&completed).contains("status: Completed"));
}

#[test]
fn repository_metadata_fixture_can_be_denied_after_fixture_is_removed() {
    let state = state_root("repository-metadata-fixture-denial");
    let project = state_root("repository-metadata-fixture-denial-project");
    copy_dir(&repository_metadata_fixture_root(), &project);
    let waiting = workflow_os(
        &project,
        &state,
        &["run", "example/github-repository-metadata-fixture"],
    );
    assert!(waiting.status.success(), "{}", stderr(&waiting));
    let run_id = run_id(&waiting);
    let approval_id = approval_id(&waiting);
    fs::remove_file(project.join("fixtures/github/repository.json"))
        .expect("fixture removed after presentation");

    let denied = workflow_os(
        &project,
        &state,
        &[
            "approve",
            &run_id,
            &approval_id,
            "--deny",
            "--actor",
            "user/example-reviewer",
            "--reason",
            "fixture unavailable; deny without provider entry",
        ],
    );
    assert!(denied.status.success(), "{}", stderr(&denied));
    assert!(stdout(&denied).contains("decision: denied"));
    assert!(stdout(&denied).contains("status: Failed"));
    assert!(stdout(&denied).contains(&format!("next_action: workflow-os inspect {run_id}")));

    let inspect = workflow_os(&project, &state, &["inspect", &run_id]);
    assert!(inspect.status.success(), "{}", stderr(&inspect));
    let inspect_output = stdout(&inspect);
    assert!(inspect_output.contains("ApprovalDenied"));
    assert!(!inspect_output.contains("SkillInvocationRequested"));
    assert!(!inspect_output.contains("adapter_telemetry:"));
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
        &["--mock-all-local-skills", "run", "ex/gh"],
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
            "reviewed fixture context",
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
        &["--mock-all-local-skills", "run", "ex/gh"],
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

    assert_eq!(audit.len(), 2);
    assert_eq!(observability.len(), 2);
    let first = audit.first().expect("first adapter audit");
    assert_eq!(first.adapter_kind, workflow_core::AdapterKind::GitHub);
    assert_eq!(first.capability, AdapterCapability::GitHubRead);
    assert_eq!(first.operation_mode, AdapterOperationMode::Fixture);
    assert_eq!(first.workflow_run_id.as_ref(), Some(&run_id));
    assert!(first.correlation_id.as_str().starts_with("correlation-"));
    assert!(matches!(
        first.policy_precheck,
        AdapterPolicyPrecheck::Allowed {
            provenance: AdapterPolicyPrecheckProvenance::FixtureTest,
            ..
        }
    ));
    assert!(!format!("{audit:?}").contains("ghp_"));

    let inspected = workflow_os(&example_root(), &state, &["inspect", run_id.as_str()]);
    assert!(inspected.status.success(), "{}", stderr(&inspected));
    assert!(stdout(&inspected).contains("adapter_telemetry: 2"));
}

#[test]
fn completed_run_can_be_inspected() {
    let state = state_root("inspect");
    let waiting = workflow_os(
        &example_root(),
        &state,
        &["--mock-all-local-skills", "run", "ex/gh"],
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
    let client = GitHubFixtureClient::new()
        .with_json(
            "/repos/acme/widgets",
            r#"{"full_name":"acme/widgets","default_branch":"main","private":true}"#,
        )
        .with_json(
            "/repos/acme/widgets/pulls/7",
            r#"{"number":7,"state":"open","title":"Improve widget validation"}"#,
        );
    let adapter = GitHubReadOnlyAdapter::new(
        GitHubReadOnlyConfig::fixture().expect("fixture config"),
        client,
    );
    let request = github_read_request(
        github_actions::PULL_REQUEST_METADATA,
        ActorId::new("system/github-example-test").expect("actor"),
        CorrelationId::new("correlation/github-example-test").expect("correlation"),
        [
            ("owner".to_owned(), "acme".to_owned()),
            ("repo".to_owned(), "widgets".to_owned()),
            ("pull_number".to_owned(), "7".to_owned()),
        ]
        .into_iter()
        .collect(),
        AdapterOperationMode::Fixture,
        AdapterPolicyPrecheck::fixture_test_allowed(vec!["policy.fixture.read".to_owned()]),
    )
    .expect("request");

    let outcome = adapter
        .read_pull_request_metadata(&request, "acme", "widgets", 7)
        .expect("adapter read");

    assert_eq!(outcome.invocation.capability, AdapterCapability::GitHubRead);
    assert_eq!(outcome.invocation.status, AdapterResponseStatus::Success);
    assert_eq!(outcome.observability.status, AdapterResponseStatus::Success);
}

#[test]
fn no_write_action_occurs_or_is_declared() {
    let root = example_root();
    for path in [
        "workflow-os.yml",
        "workflows/review-context.workflow.yml",
        "skills/github-review-context.skill.yml",
        "policies/github-read-only.policy.yml",
        "tests/github-review-context.test.yml",
    ] {
        let content = fs::read_to_string(root.join(path)).expect("example file readable");
        assert!(!content.contains("github.write"));
        assert!(!content.contains("external.write"));
        assert!(!content.contains("ci.rerun"));
    }
}

#[test]
fn missing_fixture_fails_clearly() {
    let source = example_root();
    let project = state_root("missing-fixture-project");
    copy_dir(&source, &project);
    fs::remove_file(project.join("fixtures/github/pull-7-files.json")).expect("fixture removed");
    let state = state_root("missing-fixture-state");
    let waiting = workflow_os(
        &project,
        &state,
        &["--mock-all-local-skills", "run", "ex/gh"],
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
    assert!(stdout(&inspected).contains("github.fixture.missing"));
}

#[test]
fn live_mode_is_skipped_by_default() {
    let config = GitHubReadOnlyConfig::fixture().expect("fixture config");

    assert!(!config.credential_present());
}
