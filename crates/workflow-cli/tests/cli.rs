#![allow(clippy::expect_used)]
//! CLI integration tests.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

use sha2::{Digest, Sha256};

use workflow_core::{
    AuditEvent, EventLogStore, LocalStateBackend, StateBackend, WorkflowRunEventKind,
    WorkflowRunId, WorkflowRunStatus,
};

static NEXT_TEST_PROJECT: AtomicU64 = AtomicU64::new(1);

struct TestProject {
    root: PathBuf,
}

impl TestProject {
    fn new(name: &str) -> Self {
        let id = NEXT_TEST_PROJECT.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "workflow-os-cli-{name}-{}-{id}",
            std::process::id()
        ));
        if root.exists() {
            fs::remove_dir_all(&root).expect("stale test project cleanup succeeds");
        }
        fs::create_dir_all(&root).expect("test project root is created");
        Self { root }
    }

    fn path(&self) -> &Path {
        &self.root
    }

    fn state_root(&self) -> PathBuf {
        self.root.join(".workflow-os").join("state")
    }

    fn write(&self, relative_path: &str, content: &str) {
        let path = self.root.join(relative_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("parent directory is created");
        }
        fs::write(path, content).expect("test file is written");
    }

    #[allow(clippy::too_many_lines)]
    fn write_valid_project(&self, approval: bool, secret_literal: bool) {
        fs::create_dir_all(self.root.join("tests")).expect("tests directory is created");
        self.write(
            "workflow-os.yml",
            r"
schema_version: workflowos.dev/v0
project:
  id: acme/cli
  name: CLI Test
layout:
  workflows: workflows
  skills: skills
  policies: policies
  tests: tests
",
        );
        self.write(
            "policies/local.policy.yml",
            r"
schema_version: workflowos.dev/v0
id: local/allow
name: Local Allow
rules:
  - id: local-only
    effect: allow_local
  - id: approve
    effect: require_approval
",
        );
        self.write(
            "skills/echo.skill.yml",
            r"
schema_version: workflowos.dev/v0
id: local/echo
version: v0
display_name: Echo
owner:
  lifecycle_status: stable
input_contract:
  fields:
    - name: request
      field_type: string
  required:
    - request
output_contract:
  fields:
    - name: summary
      field_type: string
  required:
    - summary
failure_modes:
  - code: failed
    description: Handler failed.
evaluation_criteria:
  - name: deterministic
    description: Local deterministic handler.
audit_requirements:
  required: true
  events:
    - SkillInvocationRequested
observability_requirements:
  metrics:
    - skill_latency
",
        );
        let approval_policy = if approval {
            r"
    approval_policy:
      policy:
        id: local/allow"
        } else {
            ""
        };
        let approval_requirements = if approval {
            r"
approval_requirements:
  - id: local-human-approval
    reason: Human approval required before local execution.
    expires_after:
      duration: 30m"
        } else {
            ""
        };
        let literal = if secret_literal {
            "secret-token-should-not-print"
        } else {
            "hello"
        };
        self.write(
            "workflows/main.workflow.yml",
            &format!(
                r"
schema_version: workflowos.dev/v0
id: local/main
version: v0
display_name: Local Main
owner:
  lifecycle_status: stable
autonomy_level: level_1
triggers:
  - id: manual
    kind: manual
steps:
  - id: echo
    skill_ref:
      id: local/echo
      version: v0
    input_mapping:
      - from:
          type: literal
          value: {literal}
        to: request
    policy_requirements:
      - id: local/allow
{approval_policy}
    terminal_behavior: fail_workflow
{approval_requirements}
cancellation_behavior: stop
audit_requirements:
  required: true
  events:
    - RunCreated
observability_requirements:
  metrics:
    - workflow_latency
"
            ),
        );
    }

    fn write_invalid_project(&self) {
        self.write(
            "workflow-os.yml",
            r"
schema_version: workflowos.dev/v0
project:
  id: acme/cli
  name: CLI Test
layout:
  workflows: workflows
  skills: skills
  policies: policies
  tests: tests
",
        );
        self.write(
            "workflows/main.workflow.yml",
            r"
schema_version: workflowos.dev/v0
id: local/main
version: v0
display_name: Invalid
owner:
  lifecycle_status: stable
cancellation_behavior: stop
audit_requirements:
  required: true
observability_requirements:
  metrics:
    - workflow_latency
",
        );
    }
}

impl Drop for TestProject {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn workflow_os(project: &TestProject, args: &[&str]) -> Output {
    workflow_os_with_paths(project.path(), &project.state_root(), args)
}

fn workflow_os_with_paths(project_dir: &Path, state_dir: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_workflow-os"))
        .arg("--project-dir")
        .arg(project_dir)
        .arg("--state-dir")
        .arg(state_dir)
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

fn run_events(project: &TestProject, run_id: &str) -> Vec<workflow_core::WorkflowRunEvent> {
    run_events_from_state(&project.state_root(), run_id)
}

fn run_events_from_state(state_root: &Path, run_id: &str) -> Vec<workflow_core::WorkflowRunEvent> {
    let backend = LocalStateBackend::new(state_root).expect("state backend");
    backend
        .read_events(&WorkflowRunId::new(run_id).expect("valid run id"))
        .expect("run events are readable")
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("repository root")
        .to_path_buf()
}

fn dogfood_project_root() -> PathBuf {
    repository_root().join("dogfood/workflow-os-self-governance")
}

fn event_step_ids<'a>(
    events: &'a [workflow_core::WorkflowRunEvent],
    predicate: impl Fn(&'a WorkflowRunEventKind) -> Option<&'a str>,
) -> Vec<&'a str> {
    events
        .iter()
        .filter_map(|event| predicate(&event.kind))
        .collect()
}

fn first_json_file_under(root: &Path) -> Option<PathBuf> {
    let mut stack = vec![root.to_path_buf()];
    while let Some(path) = stack.pop() {
        for entry in fs::read_dir(&path).expect("test directory is readable") {
            let path = entry.expect("test directory entry").path();
            if path.is_dir() {
                stack.push(path);
            } else if path
                .extension()
                .is_some_and(|extension| extension == "json")
            {
                return Some(path);
            }
        }
    }
    None
}

fn encode_key(value: &str) -> String {
    let digest = Sha256::digest(value.as_bytes());
    digest.iter().fold(
        String::with_capacity(digest.len() * 2),
        |mut output, byte| {
            let _ = write!(output, "{byte:02x}");
            output
        },
    )
}

#[test]
fn validate_valid_project() {
    let project = TestProject::new("validate-valid");
    project.write_valid_project(false, false);

    let output = workflow_os(&project, &["validate"]);

    assert!(output.status.success());
    assert!(stdout(&output).contains("Project is valid."));
}

#[test]
fn validate_invalid_project_exits_non_zero() {
    let project = TestProject::new("validate-invalid");
    project.write_invalid_project();

    let output = workflow_os(&project, &["validate"]);

    assert!(!output.status.success());
    assert!(stdout(&output).contains("validation.workflow.triggers_missing"));
}

#[test]
fn help_explains_explicit_mock_local_skill_flag() {
    let project = TestProject::new("help");

    let output = workflow_os(&project, &["help"]);

    assert!(output.status.success());
    assert!(stdout(&output).contains("experimental preview JSON"));
    assert!(stdout(&output).contains("--mock-all-local-skills"));
    assert!(stdout(&output).contains("deterministic mock handlers"));
    assert!(stdout(&output).contains("doctor state"));
    assert!(stdout(&output).contains("init-agent-harness"));
    assert!(stdout(&output).contains("documentation scaffold"));
}

#[test]
fn init_agent_harness_creates_scaffold_files() {
    let project = TestProject::new("agent-harness-create");

    let output = workflow_os(&project, &["init-agent-harness"]);

    assert!(output.status.success(), "{}", stderr(&output));
    let agents = fs::read_to_string(project.path().join("AGENTS.md")).expect("AGENTS.md exists");
    let prompt = fs::read_to_string(
        project
            .path()
            .join(".workflow-os")
            .join("agent-harness-prompt.md"),
    )
    .expect("prompt exists");
    assert!(agents.contains("Agent executes. Workflow OS governs."));
    assert!(agents.contains("approval checkpoints"));
    assert!(agents.contains("automatic local check execution or handler registration"));
    assert!(prompt.contains("Use Workflow OS as the governing layer"));
    assert!(prompt.contains("do not bypass validation, policy, approvals, or failed checks"));
    assert!(!agents.to_ascii_lowercase().contains("agent swarm"));
    assert!(!agents.to_ascii_lowercase().contains("recursive agent"));
    assert!(!prompt.to_ascii_lowercase().contains("agent swarm"));
    assert!(!prompt.to_ascii_lowercase().contains("recursive agent"));
}

#[test]
fn init_agent_harness_unmanaged_agents_file_fails_without_force() {
    let project = TestProject::new("agent-harness-unmanaged");
    project.write(
        "AGENTS.md",
        "user maintained instructions with secret-token-marker",
    );

    let output = workflow_os(&project, &["init-agent-harness"]);

    assert!(!output.status.success());
    assert!(stderr(&output).contains("cli.init_agent_harness.unmanaged_file"));
    assert!(!stderr(&output).contains("secret-token-marker"));
    let agents = fs::read_to_string(project.path().join("AGENTS.md")).expect("AGENTS.md exists");
    assert_eq!(
        agents,
        "user maintained instructions with secret-token-marker"
    );
}

#[test]
fn init_agent_harness_force_replaces_unmanaged_files_without_leaking_content() {
    let project = TestProject::new("agent-harness-force");
    project.write(
        "AGENTS.md",
        "private unmanaged instructions with secret-token-marker",
    );

    let output = workflow_os(&project, &["init-agent-harness", "--force"]);

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(!stderr(&output).contains("secret-token-marker"));
    let agents = fs::read_to_string(project.path().join("AGENTS.md")).expect("AGENTS.md exists");
    assert!(agents.contains("Agent executes. Workflow OS governs."));
    assert!(!agents.contains("secret-token-marker"));
}

#[test]
fn init_agent_harness_managed_block_update_preserves_surrounding_content() {
    let project = TestProject::new("agent-harness-update");
    project.write(
        "AGENTS.md",
        r"# User Notes

Keep this line.

<!-- BEGIN WORKFLOW OS AGENT HARNESS -->
old generated text
<!-- END WORKFLOW OS AGENT HARNESS -->

Keep this footer.
",
    );
    project.write(
        ".workflow-os/agent-harness-prompt.md",
        r"# Existing Prompt

<!-- BEGIN WORKFLOW OS AGENT HARNESS -->
old prompt
<!-- END WORKFLOW OS AGENT HARNESS -->
",
    );

    let output = workflow_os(&project, &["init-agent-harness", "--agent", "codex"]);

    assert!(output.status.success(), "{}", stderr(&output));
    let agents = fs::read_to_string(project.path().join("AGENTS.md")).expect("AGENTS.md exists");
    assert!(agents.contains("Keep this line."));
    assert!(agents.contains("Keep this footer."));
    assert!(agents.contains("Codex"));
    assert!(!agents.contains("old generated text"));
}

#[test]
fn init_agent_harness_dry_run_writes_no_files_or_state() {
    let project = TestProject::new("agent-harness-dry-run");

    let output = workflow_os(&project, &["init-agent-harness", "--dry-run"]);

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(stdout(&output).contains("dry_run: true"));
    assert!(!project.path().join("AGENTS.md").exists());
    assert!(!project.path().join(".workflow-os").exists());
}

#[test]
fn init_agent_harness_is_scaffold_only_and_does_not_touch_runtime_state() {
    let project = TestProject::new("agent-harness-no-runtime");

    let output = workflow_os(&project, &["init-agent-harness"]);

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(!project.state_root().exists());
    assert!(!stdout(&output).contains("run_id:"));
    assert!(!stdout(&output).contains("approval_id:"));
    assert!(!stdout(&output).contains("status:"));
}

#[test]
fn init_agent_harness_rejects_invalid_agent_without_file_writes() {
    let project = TestProject::new("agent-harness-invalid-agent");

    let output = workflow_os(
        &project,
        &["init-agent-harness", "--agent", "secret-token-agent"],
    );

    assert!(!output.status.success());
    assert!(stderr(&output).contains("agent must be one of"));
    assert!(!stderr(&output).contains("secret-token-agent"));
    assert!(!project.path().join("AGENTS.md").exists());
}

#[test]
fn run_minimal_local_workflow() {
    let project = TestProject::new("run-minimal");
    project.write_valid_project(false, false);

    let output = workflow_os(&project, &["--mock-all-local-skills", "run", "local/main"]);

    assert!(output.status.success());
    assert!(stdout(&output).contains("status: Completed"));
}

#[test]
fn run_with_unregistered_local_skill_fails_clearly_by_default() {
    let project = TestProject::new("run-unregistered");
    project.write_valid_project(false, false);

    let output = workflow_os(&project, &["run", "local/main"]);

    assert!(!output.status.success());
    assert!(stdout(&output).contains("status: Failed"));
    assert!(stderr(&output).contains("workflow run failed"));
    let run_id = run_id(&output);
    let events = run_events(&project, &run_id);
    assert!(events.iter().any(|event| match &event.kind {
        WorkflowRunEventKind::RunFailed(failure) => {
            failure.code == "executor.skill_handler.missing"
                && failure.message.contains("no local handler registered")
        }
        _ => false,
    }));
}

#[test]
fn status_shows_completed_run() {
    let project = TestProject::new("status-completed");
    project.write_valid_project(false, false);
    let run = workflow_os(&project, &["--mock-all-local-skills", "run", "local/main"]);
    let run_id = run_id(&run);

    let output = workflow_os(&project, &["status", &run_id]);

    assert!(output.status.success());
    assert!(stdout(&output).contains("status: Completed"));
    assert!(stdout(&output).contains("schema_version: workflowos.dev/v0"));
    assert!(stdout(&output).contains("terminal: true"));
}

#[test]
fn approval_gated_run_pauses() {
    let project = TestProject::new("approval-pauses");
    project.write_valid_project(true, false);

    let output = workflow_os(&project, &["--mock-all-local-skills", "run", "local/main"]);

    assert!(output.status.success());
    assert!(stdout(&output).contains("status: WaitingForApproval"));
    assert!(stdout(&output).contains("approval_id: "));
}

#[test]
fn approve_resumes_approval_gated_run() {
    let project = TestProject::new("approval-resumes");
    project.write_valid_project(true, false);
    let waiting = workflow_os(&project, &["--mock-all-local-skills", "run", "local/main"]);
    let run_id = run_id(&waiting);
    let approval_id = approval_id(&waiting);

    let output = workflow_os(
        &project,
        &[
            "approve",
            &run_id,
            &approval_id,
            "--actor",
            "user/tester",
            "--reason",
            "approved in cli test",
            "--mock-all-local-skills",
        ],
    );

    assert!(output.status.success());
    assert!(stdout(&output).contains("status: Completed"));
}

#[test]
fn dogfood_multi_step_workflow_pauses_at_planning_approval() {
    let state = TestProject::new("dogfood-multistep-pause");
    let project_dir = dogfood_project_root();
    let validate = workflow_os_with_paths(&project_dir, &state.state_root(), &["validate"]);
    assert!(validate.status.success(), "{}", stderr(&validate));

    let waiting = workflow_os_with_paths(
        &project_dir,
        &state.state_root(),
        &["--mock-all-local-skills", "run", "dg/d"],
    );

    assert!(waiting.status.success(), "{}", stderr(&waiting));
    assert!(stdout(&waiting).contains("status: WaitingForApproval"));
    assert!(approval_id(&waiting).contains("/planning-approved"));
    let events = run_events_from_state(&state.state_root(), &run_id(&waiting));
    let scheduled = event_step_ids(&events, |kind| match kind {
        WorkflowRunEventKind::StepScheduled { step_id } => Some(step_id.as_str()),
        _ => None,
    });
    assert_eq!(scheduled, ["scope-requested", "planning-approved"]);
    let requested = event_step_ids(&events, |kind| match kind {
        WorkflowRunEventKind::SkillInvocationRequested(invocation) => {
            Some(invocation.step_id.as_str())
        }
        _ => None,
    });
    assert_eq!(requested, ["scope-requested"]);
}

#[test]
fn dogfood_multi_step_workflow_approval_grant_completes_all_steps() {
    let state = TestProject::new("dogfood-multistep-grant");
    let project_dir = dogfood_project_root();
    let waiting = workflow_os_with_paths(
        &project_dir,
        &state.state_root(),
        &["--mock-all-local-skills", "run", "dg/d"],
    );
    let run_id = run_id(&waiting);
    let approval_id = approval_id(&waiting);

    let completed = workflow_os_with_paths(
        &project_dir,
        &state.state_root(),
        &[
            "--mock-all-local-skills",
            "approve",
            &run_id,
            &approval_id,
            "--actor",
            "user/dogfood-reviewer",
            "--reason",
            "approved multi-step dogfood run",
        ],
    );

    assert!(completed.status.success(), "{}", stderr(&completed));
    assert!(stdout(&completed).contains("status: Completed"));
    let events = run_events_from_state(&state.state_root(), &run_id);
    let scheduled = event_step_ids(&events, |kind| match kind {
        WorkflowRunEventKind::StepScheduled { step_id } => Some(step_id.as_str()),
        _ => None,
    });
    assert_eq!(
        scheduled,
        [
            "scope-requested",
            "planning-approved",
            "implementation-handoff",
            "validation-disclosure",
            "review-and-report-posture"
        ]
    );
    let succeeded = event_step_ids(&events, |kind| match kind {
        WorkflowRunEventKind::SkillInvocationSucceeded { step_id, .. } => Some(step_id.as_str()),
        _ => None,
    });
    assert_eq!(
        succeeded,
        [
            "scope-requested",
            "planning-approved",
            "implementation-handoff",
            "validation-disclosure",
            "review-and-report-posture"
        ]
    );
}

#[test]
fn dogfood_multi_step_workflow_approval_denial_stops_downstream_steps() {
    let state = TestProject::new("dogfood-multistep-deny");
    let project_dir = dogfood_project_root();
    let waiting = workflow_os_with_paths(
        &project_dir,
        &state.state_root(),
        &["--mock-all-local-skills", "run", "dg/d"],
    );
    let run_id = run_id(&waiting);
    let approval_id = approval_id(&waiting);

    let denied = workflow_os_with_paths(
        &project_dir,
        &state.state_root(),
        &[
            "approve",
            &run_id,
            &approval_id,
            "--deny",
            "--actor",
            "user/dogfood-reviewer",
            "--reason",
            "denied multi-step dogfood run",
        ],
    );

    assert!(denied.status.success(), "{}", stderr(&denied));
    assert!(stdout(&denied).contains("status: Failed"));
    let events = run_events_from_state(&state.state_root(), &run_id);
    let scheduled = event_step_ids(&events, |kind| match kind {
        WorkflowRunEventKind::StepScheduled { step_id } => Some(step_id.as_str()),
        _ => None,
    });
    assert_eq!(scheduled, ["scope-requested", "planning-approved"]);
    let requested = event_step_ids(&events, |kind| match kind {
        WorkflowRunEventKind::SkillInvocationRequested(invocation) => {
            Some(invocation.step_id.as_str())
        }
        _ => None,
    });
    assert_eq!(requested, ["scope-requested"]);
}

#[test]
fn approval_denial_fails_run_and_records_decision_metadata() {
    let project = TestProject::new("approval-denial");
    project.write_valid_project(true, false);
    let waiting = workflow_os(&project, &["--mock-all-local-skills", "run", "local/main"]);
    let run_id = run_id(&waiting);
    let approval_id = approval_id(&waiting);

    let output = workflow_os(
        &project,
        &[
            "approve",
            &run_id,
            &approval_id,
            "--deny",
            "--actor",
            "user/denier",
            "--reason",
            "risk is not acceptable",
        ],
    );

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(stdout(&output).contains("decision: denied"));
    assert!(stdout(&output).contains("status: Failed"));

    let events = run_events(&project, &run_id);
    let denied = events
        .iter()
        .find_map(|event| match &event.kind {
            WorkflowRunEventKind::ApprovalDenied(decision) => Some((event, decision)),
            _ => None,
        })
        .expect("approval denial event is emitted");
    assert_eq!(denied.1.actor.as_str(), "user/denier");
    assert_eq!(denied.1.reason, "risk is not acceptable");
    assert!(events
        .iter()
        .any(|event| matches!(event.kind, WorkflowRunEventKind::RunFailed(_))));

    let audit = AuditEvent::from_workflow_event(denied.0, "workflow-cli-test");
    assert_eq!(audit.actor.expect("audit actor").as_str(), "user/denier");
    assert!(audit
        .decision_context
        .expect("audit decision context")
        .contains("risk is not acceptable"));
}

#[test]
fn approval_denial_does_not_execute_gated_skill() {
    let project = TestProject::new("approval-denial-no-skill");
    project.write_valid_project(true, false);
    let waiting = workflow_os(&project, &["--mock-all-local-skills", "run", "local/main"]);
    let run_id = run_id(&waiting);
    let approval_id = approval_id(&waiting);

    let output = workflow_os(
        &project,
        &[
            "approve",
            &run_id,
            &approval_id,
            "--deny",
            "--reason",
            "operator denied local action",
        ],
    );

    assert!(output.status.success(), "{}", stderr(&output));
    let events = run_events(&project, &run_id);
    assert!(!events.iter().any(|event| matches!(
        event.kind,
        WorkflowRunEventKind::SkillInvocationRequested(_)
            | WorkflowRunEventKind::SkillInvocationStarted(_)
            | WorkflowRunEventKind::SkillInvocationSucceeded { .. }
    )));
}

#[test]
fn approval_denial_requires_reason() {
    let project = TestProject::new("approval-denial-reason");
    project.write_valid_project(true, false);
    let waiting = workflow_os(&project, &["--mock-all-local-skills", "run", "local/main"]);
    let run_id = run_id(&waiting);
    let approval_id = approval_id(&waiting);

    let output = workflow_os(&project, &["approve", &run_id, &approval_id, "--deny"]);

    assert!(!output.status.success());
    assert!(stderr(&output).contains("approval denial requires --reason"));
}

#[test]
fn approval_denial_after_terminal_state_is_rejected() {
    let project = TestProject::new("approval-denial-terminal");
    project.write_valid_project(true, false);
    let waiting = workflow_os(&project, &["--mock-all-local-skills", "run", "local/main"]);
    let run_id = run_id(&waiting);
    let approval_id = approval_id(&waiting);
    let granted = workflow_os(
        &project,
        &["--mock-all-local-skills", "approve", &run_id, &approval_id],
    );
    assert!(granted.status.success(), "{}", stderr(&granted));

    let denied = workflow_os(
        &project,
        &[
            "approve",
            &run_id,
            &approval_id,
            "--deny",
            "--reason",
            "late denial should fail",
        ],
    );

    assert!(!denied.status.success());
    assert!(stderr(&denied).contains("approval decisions after a terminal state are rejected"));
}

#[test]
fn duplicate_denial_and_later_grant_are_rejected_deterministically() {
    let project = TestProject::new("approval-denial-duplicate");
    project.write_valid_project(true, false);
    let waiting = workflow_os(&project, &["--mock-all-local-skills", "run", "local/main"]);
    let run_id = run_id(&waiting);
    let approval_id = approval_id(&waiting);
    let denied = workflow_os(
        &project,
        &[
            "approve",
            &run_id,
            &approval_id,
            "--deny",
            "--reason",
            "first denial",
        ],
    );
    assert!(denied.status.success(), "{}", stderr(&denied));

    let duplicate_denial = workflow_os(
        &project,
        &[
            "approve",
            &run_id,
            &approval_id,
            "--deny",
            "--reason",
            "second denial",
        ],
    );
    let later_grant = workflow_os(&project, &["approve", &run_id, &approval_id]);

    assert!(!duplicate_denial.status.success());
    assert!(stderr(&duplicate_denial)
        .contains("approval decisions after a terminal state are rejected"));
    assert!(!later_grant.status.success());
    assert!(stderr(&later_grant).contains("approval decisions after a terminal state are rejected"));

    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let run = backend
        .rehydrate_run(&WorkflowRunId::new(&run_id).expect("valid run id"))
        .expect("run rehydrates");
    assert_eq!(run.snapshot.status, WorkflowRunStatus::Failed);
}

#[test]
fn approval_denial_json_output_includes_decision() {
    let project = TestProject::new("approval-denial-json");
    project.write_valid_project(true, false);
    let waiting = workflow_os(&project, &["--mock-all-local-skills", "run", "local/main"]);
    let run_id = run_id(&waiting);
    let approval_id = approval_id(&waiting);

    let output = workflow_os(
        &project,
        &[
            "--json",
            "approve",
            &run_id,
            &approval_id,
            "--deny",
            "--reason",
            "json denial",
        ],
    );

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(stdout(&output).contains("\"decision\":\"denied\""));
    assert!(stdout(&output).contains("\"status\":\"Failed\""));
}

#[test]
fn inspect_shows_event_history() {
    let project = TestProject::new("inspect");
    project.write_valid_project(false, false);
    let run = workflow_os(&project, &["--mock-all-local-skills", "run", "local/main"]);
    let run_id = run_id(&run);

    let output = workflow_os(&project, &["inspect", &run_id]);

    assert!(output.status.success());
    assert!(stdout(&output).contains("events:"));
    assert!(stdout(&output).contains("RunCreated"));
    assert!(stdout(&output).contains("mock-local-cli-output/"));
    assert!(stdout(&output).contains("schema_version: workflowos.dev/v0"));
}

#[test]
fn inspect_json_exposes_schema_version() {
    let project = TestProject::new("inspect-json-schema-version");
    project.write_valid_project(false, false);
    let run = workflow_os(&project, &["--mock-all-local-skills", "run", "local/main"]);
    let run_id = run_id(&run);

    let output = workflow_os(&project, &["--json", "inspect", &run_id]);

    assert!(output.status.success());
    assert!(stdout(&output).contains("\"schema_version\":\"workflowos.dev/v0\""));
}

#[test]
fn doctor_detects_missing_project() {
    let project = TestProject::new("doctor-missing");

    let output = workflow_os(&project, &["doctor"]);

    assert!(!output.status.success());
    assert!(stdout(&output).contains("project_manifest: failed"));
}

#[test]
fn doctor_state_reports_healthy_local_state() {
    let project = TestProject::new("doctor-state-healthy");
    project.write_valid_project(false, false);
    let run = workflow_os(&project, &["--mock-all-local-skills", "run", "local/main"]);
    assert!(run.status.success(), "{}", stderr(&run));

    let output = workflow_os(&project, &["doctor", "state"]);

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(stdout(&output).contains("state_backend: ok"));
    assert!(stdout(&output).contains("issues: none"));
}

#[test]
fn doctor_state_reports_missing_event_index() {
    let project = TestProject::new("doctor-state-missing-index");
    project.write_valid_project(false, false);
    let run = workflow_os(&project, &["--mock-all-local-skills", "run", "local/main"]);
    assert!(run.status.success(), "{}", stderr(&run));
    let index_path =
        first_json_file_under(&project.state_root().join("event_ids")).expect("event index exists");
    fs::remove_file(index_path).expect("test removes event index");

    let output = workflow_os(&project, &["doctor", "state"]);

    assert!(!output.status.success());
    assert!(stdout(&output).contains("state_backend: failed"));
    assert!(stdout(&output).contains("state.event_index.missing"));
}

#[test]
fn doctor_state_reports_dangling_event_index() {
    let project = TestProject::new("doctor-state-dangling-index");
    project.write_valid_project(false, false);
    let run = workflow_os(&project, &["--mock-all-local-skills", "run", "local/main"]);
    assert!(run.status.success(), "{}", stderr(&run));
    let event_path =
        first_json_file_under(&project.state_root().join("events")).expect("event file exists");
    fs::remove_file(event_path).expect("test removes event file");

    let output = workflow_os(&project, &["doctor", "state"]);

    assert!(!output.status.success());
    assert!(stdout(&output).contains("state.event_index.dangling"));
}

#[test]
fn doctor_state_reports_corrupt_event_file() {
    let project = TestProject::new("doctor-state-corrupt-event");
    project.write_valid_project(false, false);
    let run = workflow_os(&project, &["--mock-all-local-skills", "run", "local/main"]);
    assert!(run.status.success(), "{}", stderr(&run));
    let event_path =
        first_json_file_under(&project.state_root().join("events")).expect("event file exists");
    fs::write(event_path, "{not valid json").expect("test corrupts event file");

    let output = workflow_os(&project, &["doctor", "state"]);

    assert!(!output.status.success());
    assert!(stdout(&output).contains("state.corrupt"));
}

#[test]
fn doctor_state_reports_rehydration_failure() {
    let project = TestProject::new("doctor-state-rehydrate");
    project.write_valid_project(false, false);
    let run = workflow_os(&project, &["--mock-all-local-skills", "run", "local/main"]);
    assert!(run.status.success(), "{}", stderr(&run));
    let run_id = run_id(&run);
    let events = run_events(&project, &run_id);
    let middle_event = events
        .iter()
        .find(|event| event.sequence_number.get() == 2)
        .expect("completed run has second event");
    let event_path = project
        .state_root()
        .join("events")
        .join(encode_key(middle_event.run_id.as_str()))
        .join(format!("{:020}.json", middle_event.sequence_number.get()));
    let index_path = project.state_root().join("event_ids").join(format!(
        "{}.json",
        encode_key(middle_event.event_id.as_str())
    ));
    fs::remove_file(event_path).expect("test removes event file");
    fs::remove_file(index_path).expect("test removes matching event index");

    let output = workflow_os(&project, &["doctor", "state"]);

    assert!(!output.status.success());
    assert!(stdout(&output).contains("state.rehydration.failed"));
}

#[test]
fn doctor_state_json_reports_unhealthy_state() {
    let project = TestProject::new("doctor-state-json");
    project.write_valid_project(false, false);
    let run = workflow_os(&project, &["--mock-all-local-skills", "run", "local/main"]);
    assert!(run.status.success(), "{}", stderr(&run));
    let event_path =
        first_json_file_under(&project.state_root().join("events")).expect("event file exists");
    fs::write(event_path, "{not valid json").expect("test corrupts event file");

    let output = workflow_os(&project, &["--json", "doctor", "state"]);

    assert!(!output.status.success());
    assert!(stdout(&output).contains("\"healthy\":false"));
    assert!(stdout(&output).contains("\"code\":\"state.corrupt\""));
}

#[test]
fn doctor_state_does_not_create_missing_state_root() {
    let project = TestProject::new("doctor-state-read-only");
    let state_root = project.path().join("missing-state-root");

    let output = Command::new(env!("CARGO_BIN_EXE_workflow-os"))
        .arg("--project-dir")
        .arg(project.path())
        .arg("--state-dir")
        .arg(&state_root)
        .arg("doctor")
        .arg("state")
        .output()
        .expect("workflow-os command runs");

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(!state_root.exists());
}

#[test]
fn json_output_is_available_for_validate() {
    let project = TestProject::new("json-validate");
    project.write_valid_project(false, false);

    let output = workflow_os(&project, &["--json", "validate"]);

    assert!(output.status.success());
    assert_eq!(stdout(&output).trim(), "[]");
}

#[test]
fn inspect_does_not_print_sensitive_literal() {
    let project = TestProject::new("inspect-redaction");
    project.write_valid_project(false, true);
    let run = workflow_os(&project, &["--mock-all-local-skills", "run", "local/main"]);
    let run_id = run_id(&run);

    let output = workflow_os(&project, &["inspect", &run_id]);

    assert!(output.status.success());
    assert!(!stdout(&output).contains("secret-token-should-not-print"));
}
