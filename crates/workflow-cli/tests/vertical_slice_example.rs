#![allow(clippy::expect_used)]
//! Vertical-slice example contract tests.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

use workflow_core::{
    ActorId, ConservativePolicyEngine, CorrelationId, LocalAuditSink, LocalExecutionRequest,
    LocalExecutor, LocalObservabilitySink, LocalSkillRegistry, LocalStateBackend,
    LocalStructuredLogger, ObservabilityEventKind, SkillHandler, SkillId, SkillInput, SkillOutput,
    SkillVersion, WorkflowId, WorkflowOsError, WorkflowRunEventKindName, WorkflowRunStatus,
};

static NEXT_STATE: AtomicU64 = AtomicU64::new(1);

fn example_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("examples")
        .join("vertical-slice-approval")
}

fn state_root(name: &str) -> PathBuf {
    let id = NEXT_STATE.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!(
        "workflow-os-vertical-slice-{name}-{}-{id}",
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

struct ExampleHandler;

impl SkillHandler for ExampleHandler {
    fn invoke(&self, input: SkillInput) -> Result<SkillOutput, WorkflowOsError> {
        let mut values = BTreeMap::new();
        let request = input
            .values
            .get("request")
            .cloned()
            .unwrap_or_else(|| "request".to_owned());
        values.insert("summary".to_owned(), format!("recommend-review:{request}"));
        Ok(SkillOutput::new(
            values,
            Some(format!("example-output/{}", input.run_id)),
        ))
    }
}

fn registry() -> LocalSkillRegistry {
    let mut registry = LocalSkillRegistry::new();
    registry.register(
        SkillId::new("local/rec").expect("skill id"),
        SkillVersion::new("v0").expect("skill version"),
        Box::new(ExampleHandler),
    );
    registry
}

#[test]
fn example_validates() {
    let state = state_root("validate");
    let output = workflow_os(&example_root(), &state, &["validate"]);

    assert!(output.status.success(), "{}", stdout(&output));
    assert!(stdout(&output).contains("Project is valid."));
}

#[test]
fn example_run_pauses_for_approval() {
    let state = state_root("pause");
    let output = workflow_os(&example_root(), &state, &["run", "ex/review"]);

    assert!(output.status.success(), "{}", stdout(&output));
    assert!(stdout(&output).contains("status: WaitingForApproval"));
    assert!(stdout(&output).contains("approval_id: "));
}

#[test]
fn example_approval_resumes_run() {
    let state = state_root("approve");
    let waiting = workflow_os(&example_root(), &state, &["run", "ex/review"]);
    let run_id = run_id(&waiting);
    let approval_id = approval_id(&waiting);

    let approved = workflow_os(
        &example_root(),
        &state,
        &[
            "approve",
            &run_id,
            &approval_id,
            "--actor",
            "user/example-approver",
            "--reason",
            "reviewed local vertical slice",
        ],
    );

    assert!(approved.status.success(), "{}", stdout(&approved));
    assert!(stdout(&approved).contains("status: Completed"));
}

#[test]
fn example_completed_run_can_be_inspected() {
    let state = state_root("inspect");
    let waiting = workflow_os(&example_root(), &state, &["run", "ex/review"]);
    let run_id = run_id(&waiting);
    let approval_id = approval_id(&waiting);
    let approved = workflow_os(&example_root(), &state, &["approve", &run_id, &approval_id]);
    assert!(approved.status.success(), "{}", stdout(&approved));

    let inspected = workflow_os(&example_root(), &state, &["inspect", &run_id]);

    assert!(inspected.status.success(), "{}", stdout(&inspected));
    assert!(stdout(&inspected).contains("PolicyDecisionRecorded"));
    assert!(stdout(&inspected).contains("SkillInvocationSucceeded"));
    assert!(stdout(&inspected).contains("status: Completed"));
}

#[test]
fn example_emits_audit_and_observability_events() {
    let state = state_root("signals");
    let backend = LocalStateBackend::new(state).expect("state backend");
    let registry = registry();
    let audit = LocalAuditSink::new();
    let observability = LocalObservabilitySink::new();
    let executor = LocalExecutor::new_with_sinks(
        &backend,
        &registry,
        ConservativePolicyEngine::new(),
        audit.clone(),
        observability.clone(),
        LocalStructuredLogger::new(),
    );
    let run = executor
        .execute(&LocalExecutionRequest {
            project_root: example_root(),
            workflow_id: WorkflowId::new("ex/review").expect("workflow id"),
            run_id: None,
            correlation_id: CorrelationId::new("correlation/example").expect("correlation"),
            actor: ActorId::new("system/example-test").expect("actor"),
        })
        .expect("example starts");

    assert_eq!(run.snapshot.status, WorkflowRunStatus::WaitingForApproval);
    assert!(audit
        .events()
        .iter()
        .any(|event| event.event_type == WorkflowRunEventKindName::PolicyDecisionRecorded));
    assert!(audit
        .events()
        .iter()
        .any(|event| event.event_type == WorkflowRunEventKindName::ApprovalRequested));
    assert!(observability
        .events()
        .iter()
        .any(|event| event.kind == ObservabilityEventKind::PolicyAllowed));
    assert!(observability
        .events()
        .iter()
        .any(|event| event.kind == ObservabilityEventKind::ApprovalRequested));
}

#[test]
fn example_requires_no_external_services_or_secrets() {
    let root = example_root();
    for path in [
        "workflow-os.yml",
        "workflows/request-review.workflow.yml",
        "skills/draft-summary.skill.yml",
        "policies/default-approval.policy.yml",
        "tests/request-review.test.yml",
    ] {
        let content = fs::read_to_string(root.join(path)).expect("example file is readable");
        assert!(!content.contains("adapter_requirements"));
        assert!(!content.contains("external.write"));
        assert!(!content.contains("api_key"));
        assert!(!content.contains("password"));
        assert!(!content.contains("token"));
    }
}
