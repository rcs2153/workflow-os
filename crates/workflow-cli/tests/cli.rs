#![allow(clippy::expect_used)]
//! CLI integration tests.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

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
    Command::new(env!("CARGO_BIN_EXE_workflow-os"))
        .arg("--project-dir")
        .arg(project.path())
        .arg("--state-dir")
        .arg(project.state_root())
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
fn run_minimal_local_workflow() {
    let project = TestProject::new("run-minimal");
    project.write_valid_project(false, false);

    let output = workflow_os(&project, &["run", "local/main"]);

    assert!(output.status.success());
    assert!(stdout(&output).contains("status: Completed"));
}

#[test]
fn status_shows_completed_run() {
    let project = TestProject::new("status-completed");
    project.write_valid_project(false, false);
    let run = workflow_os(&project, &["run", "local/main"]);
    let run_id = run_id(&run);

    let output = workflow_os(&project, &["status", &run_id]);

    assert!(output.status.success());
    assert!(stdout(&output).contains("status: Completed"));
    assert!(stdout(&output).contains("terminal: true"));
}

#[test]
fn approval_gated_run_pauses() {
    let project = TestProject::new("approval-pauses");
    project.write_valid_project(true, false);

    let output = workflow_os(&project, &["run", "local/main"]);

    assert!(output.status.success());
    assert!(stdout(&output).contains("status: WaitingForApproval"));
    assert!(stdout(&output).contains("approval_id: "));
}

#[test]
fn approve_resumes_approval_gated_run() {
    let project = TestProject::new("approval-resumes");
    project.write_valid_project(true, false);
    let waiting = workflow_os(&project, &["run", "local/main"]);
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
        ],
    );

    assert!(output.status.success());
    assert!(stdout(&output).contains("status: Completed"));
}

#[test]
fn inspect_shows_event_history() {
    let project = TestProject::new("inspect");
    project.write_valid_project(false, false);
    let run = workflow_os(&project, &["run", "local/main"]);
    let run_id = run_id(&run);

    let output = workflow_os(&project, &["inspect", &run_id]);

    assert!(output.status.success());
    assert!(stdout(&output).contains("events:"));
    assert!(stdout(&output).contains("RunCreated"));
}

#[test]
fn doctor_detects_missing_project() {
    let project = TestProject::new("doctor-missing");

    let output = workflow_os(&project, &["doctor"]);

    assert!(!output.status.success());
    assert!(stdout(&output).contains("project_manifest: failed"));
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
    let run = workflow_os(&project, &["run", "local/main"]);
    let run_id = run_id(&run);

    let output = workflow_os(&project, &["inspect", &run_id]);

    assert!(output.status.success());
    assert!(!stdout(&output).contains("secret-token-should-not-print"));
}
