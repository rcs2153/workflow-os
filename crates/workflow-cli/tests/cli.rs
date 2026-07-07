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
",
        );
        self.write(
            "policies/approval.policy.yml",
            r"
schema_version: workflowos.dev/v0
id: approval/required
name: Required Approval
rules:
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
        id: approval/required"
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
fn validate_missing_manifest_suggests_repo_governance_scaffold() {
    let project = TestProject::new("validate-missing-manifest");

    let output = workflow_os(&project, &["validate"]);

    assert!(!output.status.success());
    assert!(stdout(&output).contains("loader.manifest_missing"));
    assert!(stdout(&output).contains("next_step: workflow-os init-repo-governance"));
    assert!(!project.state_root().exists());
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
    assert!(stdout(&output).contains("init-repo-governance"));
    assert!(stdout(&output).contains("existing-repo governance scaffold"));
    assert!(stdout(&output).contains("first-run"));
    assert!(stdout(&output).contains("report-ready first-run context"));
}

#[test]
fn command_local_help_does_not_become_positional_workflow_id() {
    let project = TestProject::new("run-help");

    let output = workflow_os(&project, &["run", "--help"]);

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(stdout(&output).contains("Workflow OS CLI"));
    assert!(stdout(&output).contains("run <workflow-id>"));
    assert!(!stdout(&output).contains("executor.workflow.not_found"));
    assert!(stderr(&output).is_empty());
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
    assert!(agents.contains("engineering standard or contribution guide if one exists"));
    assert!(agents.contains(".workflow-os/README.md"));
    assert!(agents.contains(".workflow-os/agent-harness-prompt.md"));
    assert!(!agents.contains("docs/ENGINEERING_STANDARD.md"));
    assert!(prompt.contains("Agent executes. Workflow OS governs."));
    assert!(prompt.contains("Use Workflow OS as the governing layer"));
    assert!(prompt.contains("do not bypass validation, policy, approvals, or failed checks"));
    assert!(stdout(&output).contains("paste .workflow-os/agent-harness-prompt.md"));
    assert!(!agents.to_ascii_lowercase().contains("agent swarm"));
    assert!(!agents.to_ascii_lowercase().contains("recursive agent"));
    assert!(!prompt.to_ascii_lowercase().contains("agent swarm"));
    assert!(!prompt.to_ascii_lowercase().contains("recursive agent"));
}

#[test]
fn init_agent_harness_unmanaged_agents_file_is_preserved_without_force() {
    let project = TestProject::new("agent-harness-unmanaged");
    project.write(
        "AGENTS.md",
        "user maintained instructions with secret-token-marker",
    );

    let output = workflow_os(&project, &["init-agent-harness"]);

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(stdout(&output).contains("preserved_unmanaged_agent_guidance: AGENTS.md"));
    assert!(stdout(&output).contains("appended_managed_agent_guidance: AGENTS.md"));
    assert!(!stderr(&output).contains("secret-token-marker"));
    let agents = fs::read_to_string(project.path().join("AGENTS.md")).expect("AGENTS.md exists");
    assert!(agents.contains("user maintained instructions with secret-token-marker"));
    assert!(agents.contains("Agent executes. Workflow OS governs."));
    assert_eq!(
        agents
            .matches("<!-- BEGIN WORKFLOW OS AGENT HARNESS -->")
            .count(),
        1
    );
}

#[test]
fn init_agent_harness_unmanaged_prompt_file_fails_without_force() {
    let project = TestProject::new("agent-harness-unmanaged-prompt");
    project.write(
        "AGENTS.md",
        r"# Managed Agents

<!-- BEGIN WORKFLOW OS AGENT HARNESS -->
old managed text
<!-- END WORKFLOW OS AGENT HARNESS -->
",
    );
    project.write(
        ".workflow-os/agent-harness-prompt.md",
        "private prompt instructions with secret-token-marker",
    );

    let output = workflow_os(&project, &["init-agent-harness"]);

    assert!(!output.status.success());
    assert!(stderr(&output).contains("cli.init_agent_harness.unmanaged_file"));
    assert!(stderr(&output).contains(".workflow-os/agent-harness-prompt.md"));
    assert!(!stderr(&output).contains("secret-token-marker"));
    let prompt = fs::read_to_string(
        project
            .path()
            .join(".workflow-os")
            .join("agent-harness-prompt.md"),
    )
    .expect("prompt file exists");
    assert_eq!(
        prompt,
        "private prompt instructions with secret-token-marker"
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
fn init_agent_harness_dry_run_preserves_unmanaged_agents_file() {
    let project = TestProject::new("agent-harness-dry-run-unmanaged");
    project.write(
        "AGENTS.md",
        "existing repo guidance with secret-token-marker",
    );

    let output = workflow_os(&project, &["init-agent-harness", "--dry-run"]);

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(stdout(&output).contains("dry_run: true"));
    assert!(stdout(&output).contains("would_preserve_unmanaged_agent_guidance: AGENTS.md"));
    assert!(stdout(&output).contains("would_append_managed_agent_guidance: AGENTS.md"));
    assert!(!stdout(&output).contains("secret-token-marker"));
    assert!(!stderr(&output).contains("secret-token-marker"));
    let agents = fs::read_to_string(project.path().join("AGENTS.md")).expect("AGENTS.md exists");
    assert_eq!(agents, "existing repo guidance with secret-token-marker");
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
fn init_repo_governance_creates_valid_local_project() {
    let project = TestProject::new("repo-governance-create");

    let output = workflow_os(&project, &["init-repo-governance"]);

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(project.path().join("workflow-os.yml").exists());
    assert!(project
        .path()
        .join("workflows")
        .join("first-run-governance.workflow.yml")
        .exists());
    assert!(project
        .path()
        .join("skills")
        .join("first-run-report.skill.yml")
        .exists());
    assert!(project
        .path()
        .join("policies")
        .join("default-governance.policy.yml")
        .exists());
    assert!(project
        .path()
        .join("tests")
        .join("first-run-governance.test.yml")
        .exists());
    assert!(project
        .path()
        .join(".workflow-os")
        .join("README.md")
        .exists());
    assert!(project.path().join("AGENTS.md").exists());
    assert!(project
        .path()
        .join(".workflow-os")
        .join("agent-harness-prompt.md")
        .exists());
    assert!(stdout(&output).contains("workflow-os validate"));
    assert!(stdout(&output).contains("workflow-os first-run"));
    assert!(stdout(&output).contains("local/first-run-governance"));
    let agents = fs::read_to_string(project.path().join("AGENTS.md")).expect("AGENTS.md exists");
    assert!(agents.contains("engineering standard or contribution guide if one exists"));
    assert!(agents.contains(".workflow-os/README.md"));
    assert!(agents.contains(".workflow-os/agent-harness-prompt.md"));
    assert!(!agents.contains("docs/ENGINEERING_STANDARD.md"));

    let validate = workflow_os(&project, &["validate"]);
    assert!(validate.status.success(), "{}", stderr(&validate));
    assert!(stdout(&validate).contains("Project is valid."));
}

#[test]
fn init_repo_governance_generated_workflow_runs_to_approval_with_mock_skill() {
    let project = TestProject::new("repo-governance-run");
    let init = workflow_os(&project, &["init-repo-governance"]);
    assert!(init.status.success(), "{}", stderr(&init));

    let output = workflow_os(
        &project,
        &[
            "--mock-all-local-skills",
            "run",
            "local/first-run-governance",
        ],
    );

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(stdout(&output).contains("status: WaitingForApproval"));
    assert!(stdout(&output).contains("approval_id:"));
    let run_id = run_id(&output);
    let approval_id = approval_id(&output);
    let approve = workflow_os(
        &project,
        &[
            "--mock-all-local-skills",
            "approve",
            &run_id,
            &approval_id,
            "--actor",
            "user/local-reviewer",
            "--reason",
            "reviewed-first-run-governance",
        ],
    );
    assert!(approve.status.success(), "{}", stderr(&approve));
    assert!(stdout(&approve).contains("status: Completed"));
}

#[test]
fn init_repo_governance_dry_run_writes_no_project_files_or_state() {
    let project = TestProject::new("repo-governance-dry-run");

    let output = workflow_os(&project, &["init-repo-governance", "--dry-run"]);

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(stdout(&output).contains("dry_run: true"));
    assert!(stdout(&output).contains("would_write: workflow-os.yml"));
    assert!(!project.path().join("workflow-os.yml").exists());
    assert!(!project.path().join("workflows").exists());
    assert!(!project.path().join("skills").exists());
    assert!(!project.path().join("policies").exists());
    assert!(!project.path().join("tests").exists());
    assert!(!project.path().join("AGENTS.md").exists());
    assert!(!project.state_root().exists());
}

#[test]
fn init_repo_governance_preserves_existing_agents_file_by_default() {
    let project = TestProject::new("repo-governance-existing-agents");
    project.write(
        "AGENTS.md",
        "repo-specific agent notes with secret-token-marker",
    );

    let output = workflow_os(&project, &["init-repo-governance"]);

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(stdout(&output).contains("preserved_unmanaged_agent_guidance: AGENTS.md"));
    assert!(stdout(&output).contains("appended_managed_agent_guidance: AGENTS.md"));
    assert!(!stdout(&output).contains("secret-token-marker"));
    assert!(!stderr(&output).contains("secret-token-marker"));
    let agents = fs::read_to_string(project.path().join("AGENTS.md")).expect("AGENTS.md exists");
    assert!(agents.contains("repo-specific agent notes with secret-token-marker"));
    assert!(agents.contains("Agent executes. Workflow OS governs."));
    assert_eq!(
        agents
            .matches("<!-- BEGIN WORKFLOW OS AGENT HARNESS -->")
            .count(),
        1
    );

    let validate = workflow_os(&project, &["validate"]);
    assert!(validate.status.success(), "{}", stderr(&validate));
}

#[test]
fn init_repo_governance_dry_run_preserves_existing_agents_file() {
    let project = TestProject::new("repo-governance-dry-run-existing-agents");
    project.write(
        "AGENTS.md",
        "repo-specific agent notes with secret-token-marker",
    );

    let output = workflow_os(&project, &["init-repo-governance", "--dry-run"]);

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(stdout(&output).contains("would_preserve_unmanaged_agent_guidance: AGENTS.md"));
    assert!(stdout(&output).contains("would_append_managed_agent_guidance: AGENTS.md"));
    assert!(!stdout(&output).contains("secret-token-marker"));
    assert!(!stderr(&output).contains("secret-token-marker"));
    let agents = fs::read_to_string(project.path().join("AGENTS.md")).expect("AGENTS.md exists");
    assert_eq!(agents, "repo-specific agent notes with secret-token-marker");
    assert!(!project.path().join("workflow-os.yml").exists());
    assert!(!project.state_root().exists());
}

#[test]
fn init_repo_governance_existing_project_file_fails_closed_without_leaking_content() {
    let project = TestProject::new("repo-governance-existing");
    project.write("workflow-os.yml", "secret-token-existing-manifest");

    let output = workflow_os(&project, &["init-repo-governance"]);

    assert!(!output.status.success());
    assert!(stderr(&output).contains("cli.init_repo_governance.file_exists"));
    assert!(stderr(&output).contains("workflow-os.yml already exists"));
    assert!(!stderr(&output).contains("secret-token-existing-manifest"));
    let manifest =
        fs::read_to_string(project.path().join("workflow-os.yml")).expect("manifest remains");
    assert_eq!(manifest, "secret-token-existing-manifest");
    assert!(!project.path().join("workflows").exists());
}

#[test]
fn init_repo_governance_force_replaces_existing_project_scaffold_targets() {
    let project = TestProject::new("repo-governance-force");
    project.write("workflow-os.yml", "old unmanaged manifest");

    let output = workflow_os(&project, &["init-repo-governance", "--force"]);

    assert!(output.status.success(), "{}", stderr(&output));
    let manifest =
        fs::read_to_string(project.path().join("workflow-os.yml")).expect("manifest exists");
    assert!(manifest.contains("local/existing-repo"));
    assert!(!manifest.contains("old unmanaged manifest"));
    let validate = workflow_os(&project, &["validate"]);
    assert!(validate.status.success(), "{}", stderr(&validate));
}

#[test]
fn init_repo_governance_force_replaces_existing_agents_file_with_warning() {
    let project = TestProject::new("repo-governance-force-agents");
    project.write(
        "AGENTS.md",
        "repo-specific agent notes with secret-token-marker",
    );

    let output = workflow_os(&project, &["init-repo-governance", "--force"]);

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(stdout(&output).contains("replaced_existing_agent_guidance: AGENTS.md"));
    assert!(!stdout(&output).contains("secret-token-marker"));
    assert!(!stderr(&output).contains("secret-token-marker"));
    let agents = fs::read_to_string(project.path().join("AGENTS.md")).expect("AGENTS.md exists");
    assert!(!agents.contains("secret-token-marker"));
    assert!(agents.contains("Agent executes. Workflow OS governs."));
}

#[test]
fn first_run_after_repo_governance_outputs_report_ready_context() {
    let project = TestProject::new("first-run-report-ready");
    let init = workflow_os(&project, &["init-repo-governance"]);
    assert!(init.status.success(), "{}", stderr(&init));

    let output = workflow_os(&project, &["first-run"]);

    assert!(output.status.success(), "{}", stderr(&output));
    let out = stdout(&output);
    assert!(out.contains("Workflow OS first-run summary"));
    assert!(out.contains("status: ready_for_review"));
    assert!(out
        .contains("what_happened: validated a bounded governance envelope without starting a run"));
    assert!(out.contains(
        "what_was_not_done: no workflow run, runtime state, artifacts, local checks, or external writes were created"
    ));
    assert!(out.contains("what_matters_now:"));
    assert!(
        out.contains("  - review the governance findings before treating the repo as configured")
    );
    assert!(out.contains("  - assign ownership, escalation, evidence, and validation obligations"));
    assert!(out.contains(
        "  - the mock first-run workflow is optional and demonstrates approval/audit mechanics only"
    ));
    assert!(out.contains(
        "recommended_next_action: review first-run findings and assign ownership/check obligations"
    ));
    assert!(out.contains("recommendation_next_actions:"));
    assert!(out.contains(
        "  - review_only: recommendations are not active workflows until authored and reviewed"
    ));
    assert!(out.contains("  - start_with: first_run.assign_ownership"));
    assert!(out.contains("  - workflow_candidate: first_run.repo_implementation"));
    assert!(out.contains("  - validation_candidate: first_run.evidence_check_requirements"));
    assert!(out.contains("  - safety_candidate: first_run.side_effect_posture"));
    assert!(out.contains("  - closure_candidate: first_run.report_handoff_obligations"));
    assert!(out.contains("optional_approval_audit_demo: workflow-os --mock-all-local-skills run local/first-run-governance"));
    assert!(out.contains(
        "optional_demo_note: mock skill run demonstrates approval and event history; it is not additional repository analysis"
    ));
    assert!(
        out.contains("detail: run `workflow-os first-run --verbose` for the full posture matrix")
    );
    assert!(!out.contains("Detailed posture:"));
    assert!(!out.contains("section: work_performed"));
    assert!(!out.contains("workflow_discovery_recommendations:"));
    assert!(!out.contains("run_id:"));
    assert!(!out.contains("approval_id:"));
    assert!(!project.state_root().exists());
}

#[test]
fn first_run_verbose_outputs_full_posture_matrix() {
    let project = TestProject::new("first-run-verbose-report-ready");
    let init = workflow_os(&project, &["init-repo-governance"]);
    assert!(init.status.success(), "{}", stderr(&init));

    let output = workflow_os(&project, &["first-run", "--verbose"]);

    assert!(output.status.success(), "{}", stderr(&output));
    let out = stdout(&output);
    assert!(out.contains("Workflow OS first-run summary"));
    assert!(out.contains("what_matters_now:"));
    assert!(out.contains("Detailed posture:"));
    assert!(out.contains("first_run_report_ready: true"));
    assert!(out.contains("mode: report_ready_context"));
    assert!(out.contains("validation: passed"));
    assert!(out.contains("scaffold: present"));
    assert!(out.contains("sections: 11"));
    assert!(out.contains("section: work_performed"));
    assert!(out.contains("section: evidence_considered"));
    assert!(out.contains("section: operator_handoff_notes"));
    assert!(out.contains("evidence: not_available"));
    assert!(out.contains("checks: skipped"));
    assert!(out.contains("side_effects: none_skipped_unsupported"));
    assert!(out.contains("governance_profile: observe_and_report"));
    assert!(out.contains("profile_posture: disclosed_not_enforced"));
    assert!(out.contains("ownership: placeholder"));
    assert!(out.contains("escalation: placeholder"));
    assert!(out.contains("approvals: configured"));
    assert!(out.contains("policy_gates: declared_not_evaluated"));
    assert!(out.contains("field_evidence: not_available"));
    assert!(out.contains("field_checks: skipped"));
    assert!(out.contains("field_side_effects: none_skipped_unsupported"));
    assert!(out.contains("audit_observability: declared_runtime_after_run"));
    assert!(out.contains("triggers_declared_not_background_executed"));
    assert!(out.contains("state_model_advisory"));
    assert!(out.contains("tests_declared_not_automatically_executed"));
    assert!(out.contains("workflow_recommendations_review_only"));
    assert!(out.contains("ownership_escalation_check: warnings"));
    assert!(out.contains("ownership_escalation_findings:"));
    assert!(out.contains("ownership_missing_owner: 0"));
    assert!(out.contains("ownership_placeholder_owner: 2"));
    assert!(out.contains("escalation_missing_contact: 0"));
    assert!(out.contains("escalation_placeholder_contact: 2"));
    assert!(out.contains("ownership_escalation_finding: target=workflow#1 code=ownership.placeholder_owner severity=warning"));
    assert!(out.contains("ownership_escalation_finding: target=skill#1 code=escalation.placeholder_contact severity=warning"));
    assert!(out.contains("spec_field_coverage_check: warnings"));
    assert!(out.contains("spec_field_coverage_enforced:"));
    assert!(out.contains("spec_field_coverage_validated:"));
    assert!(out.contains("spec_field_coverage_disclosed:"));
    assert!(out.contains("spec_field_coverage_advisory:"));
    assert!(out.contains("spec_field_coverage_deferred:"));
    assert!(out.contains("spec_field_coverage_item: surface=workflow field=triggers posture=validated_deferred_execution code=spec_field.triggers.not_background_executed"));
    assert!(out.contains("spec_field_coverage_item: surface=workflow field=state_model posture=advisory code=spec_field.workflow.state_model_advisory"));
    assert!(out.contains("spec_field_coverage_item: surface=skill field=capabilities_adapters posture=validated_writes_deferred code=spec_field.skill.capabilities_adapters_writes_deferred"));
    assert!(out.contains("spec_field_coverage_item: surface=test field=assertions posture=validated_deferred_execution code=spec_field.tests.not_automatically_executed"));
    assert!(out.contains("workflow_discovery_recommendations: 7"));
    assert!(out.contains("workflow_discovery_recommendation: id=first_run.repo_implementation kind=create_workflow target=project#1 status=review_only summary=repo_implementation_workflow"));
    assert!(out.contains("next_action=review_and_author_workflow_spec"));
    assert!(out.contains("coverage=spec_field.workflow.steps_enforced_supported_local_paths|spec_field.workflow.policy_requirements_enforced_supported_local_paths|spec_field.workflow.audit_observability_disclosed"));
    assert!(out.contains("workflow_discovery_recommendation: id=first_run.assign_ownership kind=assign_ownership target=project#1 status=needs_human_review summary=assign_workflow_stewardship"));
    assert!(out.contains("next_action=replace_placeholder_owner_and_escalation"));
    assert!(out.contains(
        "ownership=authority.owner_context_required|escalation.placeholder_contact|ownership.placeholder_owner"
    ));
    assert!(out.contains("workflow_discovery_recommendation: id=first_run.evidence_check_requirements kind=add_evidence_check_requirements"));
    assert!(out.contains("next_action=define_evidence_and_validation_obligations"));
    assert!(out.contains("workflow_discovery_recommendation: id=first_run.side_effect_posture kind=add_side_effect_posture"));
    assert!(out.contains("next_action=define_side_effect_posture_before_writes"));
    assert!(out.contains("workflow_discovery_recommendation: id=first_run.report_handoff_obligations kind=add_report_handoff_obligations"));
    assert!(out.contains("next_action=define_report_and_handoff_obligations"));
    assert!(out.contains("recommendation_next_actions:"));
    assert!(out.contains("formalize a repo implementation workflow"));
    assert!(out.contains("workflow-os --mock-all-local-skills run local/first-run-governance"));
    assert!(!out.contains("run_id:"));
    assert!(!out.contains("approval_id:"));
    assert!(!project.state_root().exists());
}

#[test]
fn first_run_json_is_bounded_and_report_ready() {
    let project = TestProject::new("first-run-json");
    let init = workflow_os(&project, &["init-repo-governance"]);
    assert!(init.status.success(), "{}", stderr(&init));

    let output = workflow_os(&project, &["--json", "first-run"]);

    assert!(output.status.success(), "{}", stderr(&output));
    let out = stdout(&output);
    assert!(out.contains(r#""first_run_report_ready":true"#));
    assert!(out.contains(r#""mode":"report_ready_context""#));
    assert!(out.contains(r#""sections":["work_performed""#));
    assert!(out.contains(r#""side_effects":"none_skipped_unsupported""#));
    assert!(out.contains(r#""governance_profile":"observe_and_report""#));
    assert!(out.contains(r#""profile_posture":"disclosed_not_enforced""#));
    assert!(out.contains(r#""ownership":"placeholder""#));
    assert!(out.contains(r#""escalation":"placeholder""#));
    assert!(out.contains(r#""approvals":"configured""#));
    assert!(out.contains(r#""policy_gates":"declared_not_evaluated""#));
    assert!(out.contains(r#""deferred_fields":["triggers_declared_not_background_executed""#));
    assert!(out.contains(r#""ownership_escalation_check":{"status":"warnings""#));
    assert!(out.contains(r#""placeholder_owner":2"#));
    assert!(out.contains(r#""placeholder_escalation":2"#));
    assert!(out.contains(r#""code":"ownership.placeholder_owner""#));
    assert!(out.contains(r#""spec_field_coverage_check":{"status":"warnings""#));
    assert!(out.contains(r#""surface":"workflow","field":"triggers","category":"validated","posture":"validated_deferred_execution","code":"spec_field.triggers.not_background_executed""#));
    assert!(out.contains(r#""surface":"test","field":"assertions","category":"deferred","posture":"validated_deferred_execution","code":"spec_field.tests.not_automatically_executed""#));
    assert!(
        out.contains(r#""workflow_discovery_recommendations":{"status":"review_only","count":7"#)
    );
    assert!(out.contains(r#""id":"first_run.repo_implementation","kind":"create_workflow""#));
    assert!(out.contains(r#""summary":"repo_implementation_workflow""#));
    assert!(out.contains(r#""rationale_codes":["first_run.report_ready_context","governed_work_pattern.implementation_boundary"]"#));
    assert!(out.contains(r#""coverage_codes":["spec_field.workflow.steps_enforced_supported_local_paths","spec_field.workflow.policy_requirements_enforced_supported_local_paths","spec_field.workflow.audit_observability_disclosed"]"#));
    assert!(out.contains(r#""next_action":"review_and_author_workflow_spec""#));
    assert!(out.contains(r#""id":"first_run.assign_ownership","kind":"assign_ownership""#));
    assert!(out.contains(r#""status":"needs_human_review""#));
    assert!(out.contains(r#""ownership_issue_codes":["authority.owner_context_required","escalation.placeholder_contact","ownership.placeholder_owner"]"#));
    assert!(out.contains(r#""next_action":"replace_placeholder_owner_and_escalation""#));
    assert!(out.contains(r#""recommendation_next_actions":["review_only: recommendations are not active workflows until authored and reviewed","start_with: first_run.assign_ownership","workflow_candidate: first_run.repo_implementation","validation_candidate: first_run.evidence_check_requirements","safety_candidate: first_run.side_effect_posture","closure_candidate: first_run.report_handoff_obligations"]"#));
    assert!(!out.contains("local-maintainer"));
    assert!(!out.contains("local-maintainers"));
    assert!(!out.contains("run_id"));
    assert!(!out.contains("approval_id"));
    assert!(!project.state_root().exists());
}

#[test]
fn first_run_recommendation_detail_is_bounded_and_review_only() {
    let project = TestProject::new("first-run-recommendation-detail");
    let init = workflow_os(&project, &["init-repo-governance"]);
    assert!(init.status.success(), "{}", stderr(&init));

    let output = workflow_os(
        &project,
        &[
            "first-run",
            "--recommendation",
            "first_run.repo_implementation",
        ],
    );

    assert!(output.status.success(), "{}", stderr(&output));
    let out = stdout(&output);
    assert!(out.contains("Workflow OS first-run recommendation detail"));
    assert!(out.contains("id: first_run.repo_implementation"));
    assert!(out.contains("kind: create_workflow"));
    assert!(out.contains("target: project#1"));
    assert!(out.contains("status: review_only"));
    assert!(out.contains("review_posture: review_only_not_active_workflow"));
    assert!(out.contains("summary: repo_implementation_workflow"));
    assert!(out.contains(
        "rationale: first_run.report_ready_context|governed_work_pattern.implementation_boundary"
    ));
    assert!(out.contains("metadata_signals: none"));
    assert!(out.contains("next_action: review_and_author_workflow_spec"));
    assert!(out.contains("draft_proposal_status: inactive_review_required"));
    assert!(out.contains("draft_proposal_kind: workflow_draft_proposal"));
    assert!(out.contains("proposed_lifecycle_status: draft"));
    assert!(out.contains("required_authoring_decisions: choose_workflow_id|assign_owner|assign_escalation_contact|define_step_boundaries|define_policy_gates|define_evidence_and_check_obligations|define_side_effect_posture|define_report_handoff_posture"));
    assert!(out.contains("draft_non_goals: no_file_written|no_workflow_registered|no_command_executed|no_provider_call|no_runtime_state_created|no_active_workflow_created"));
    assert!(out.contains("author_and_review_workflow_spec_with_owner_policy_evidence_checks_side_effects_and_report_posture"));
    assert!(out.contains(
        "what_workflow_os_did_not_do: no_workflow_generated_no_file_written_no_command_executed"
    ));
    assert!(out.contains("privacy_boundary: bounded_codes_only_no_raw_payloads"));
    assert!(!out.contains("Workflow OS first-run summary"));
    assert!(!out.contains("workflow_discovery_recommendations:"));
    assert!(!out.contains("run_id:"));
    assert!(!out.contains("approval_id:"));
    assert!(!project.state_root().exists());
}

#[test]
fn first_run_recommendation_detail_uses_safe_metadata_codes_only() {
    let project = TestProject::new("first-run-recommendation-detail-package");
    project.write(
        "package.json",
        r#"{
  "scripts": {
    "test": "node test.js --token=secret-detail-script-token"
  },
  "devDependencies": {
    "typescript": "5.0.0",
    "secret-detail-dependency": "1.0.0"
  }
}"#,
    );
    project.write("tsconfig.json", "{}");
    let init = workflow_os(&project, &["init-repo-governance"]);
    assert!(init.status.success(), "{}", stderr(&init));

    let output = workflow_os(
        &project,
        &[
            "first-run",
            "--recommendation",
            "first_run.typescript_implementation",
        ],
    );

    assert!(output.status.success(), "{}", stderr(&output));
    let out = stdout(&output);
    assert!(out.contains("id: first_run.typescript_implementation"));
    assert!(out.contains(
        "metadata_signals: repo_metadata.package_json_present|repo_metadata.typescript_detected"
    ));
    assert!(out.contains("next_action: review_and_author_workflow_spec"));
    assert!(!out.contains("secret-detail-script-token"));
    assert!(!out.contains("secret-detail-dependency"));
    assert!(!out.contains("node test.js"));
    assert!(!project.state_root().exists());
}

#[test]
fn first_run_recommendation_detail_json_is_bounded() {
    let project = TestProject::new("first-run-recommendation-detail-json");
    let init = workflow_os(&project, &["init-repo-governance"]);
    assert!(init.status.success(), "{}", stderr(&init));

    let output = workflow_os(
        &project,
        &[
            "--json",
            "first-run",
            "--recommendation",
            "first_run.assign_ownership",
        ],
    );

    assert!(output.status.success(), "{}", stderr(&output));
    let out = stdout(&output);
    assert!(out.contains(r#""first_run_recommendation_detail":{"id":"first_run.assign_ownership""#));
    assert!(out.contains(r#""kind":"assign_ownership""#));
    assert!(out.contains(r#""status":"needs_human_review""#));
    assert!(out.contains(r#""review_posture":"review_only_not_active_workflow""#));
    assert!(out.contains(r#""metadata_signals":[]"#));
    assert!(out.contains(r#""ownership_issue_codes":["authority.owner_context_required","escalation.placeholder_contact","ownership.placeholder_owner"]"#));
    assert!(out.contains(r#""next_action":"replace_placeholder_owner_and_escalation""#));
    assert!(out.contains(r#""draft_proposal":{"source_recommendation_id":"first_run.assign_ownership","status":"inactive_review_required""#));
    assert!(out.contains(r#""proposal_kind":"ownership_update_proposal""#));
    assert!(out.contains(r#""required_authoring_decisions":["assign_owner","assign_escalation_contact","define_authority_context","review_local_vs_enterprise_stewardship"]"#));
    assert!(out.contains(
        r#""what_workflow_os_did_not_do":"no_rbac_no_idp_no_paging_no_escalation_notification""#
    ));
    assert!(!out.contains("local-maintainer"));
    assert!(!out.contains("run_id"));
    assert!(!out.contains("approval_id"));
    assert!(!project.state_root().exists());
}

#[test]
fn first_run_unknown_recommendation_id_fails_closed_without_state() {
    let project = TestProject::new("first-run-recommendation-detail-missing");
    let init = workflow_os(&project, &["init-repo-governance"]);
    assert!(init.status.success(), "{}", stderr(&init));

    let output = workflow_os(
        &project,
        &[
            "first-run",
            "--recommendation",
            "first_run.not_a_recommendation",
        ],
    );

    assert!(!output.status.success());
    assert!(stderr(&output).contains("cli.first_run.recommendation_not_found"));
    assert!(!stderr(&output).contains("first_run.not_a_recommendation"));
    assert!(!project.state_root().exists());
}

#[test]
fn author_workflow_dry_run_requires_dry_run() {
    let project = TestProject::new("author-workflow-requires-dry-run");
    let init = workflow_os(&project, &["init-repo-governance"]);
    assert!(init.status.success(), "{}", stderr(&init));

    let output = workflow_os(
        &project,
        &[
            "author",
            "workflow",
            "--from-recommendation",
            "first_run.repo_implementation",
        ],
    );

    assert!(!output.status.success());
    assert!(stderr(&output).contains("cli.workflow_authoring.dry_run_required"));
    assert!(!project.state_root().exists());
}

#[test]
fn author_workflow_dry_run_requires_recommendation() {
    let project = TestProject::new("author-workflow-requires-recommendation");
    let init = workflow_os(&project, &["init-repo-governance"]);
    assert!(init.status.success(), "{}", stderr(&init));

    let output = workflow_os(&project, &["author", "workflow", "--dry-run"]);

    assert!(!output.status.success());
    assert!(stderr(&output).contains("cli.workflow_authoring.recommendation_required"));
    assert!(!project.state_root().exists());
}

#[test]
fn author_workflow_dry_run_preview_is_bounded_and_non_mutating() {
    let project = TestProject::new("author-workflow-dry-run-preview");
    let init = workflow_os(&project, &["init-repo-governance"]);
    assert!(init.status.success(), "{}", stderr(&init));

    let output = workflow_os(
        &project,
        &[
            "author",
            "workflow",
            "--from-recommendation",
            "first_run.repo_implementation",
            "--dry-run",
        ],
    );

    assert!(output.status.success(), "{}", stderr(&output));
    let out = stdout(&output);
    assert!(out.contains("Workflow OS governed workflow authoring dry-run"));
    assert!(out.contains("mode: author_workflow_dry_run"));
    assert!(out.contains("status: preview_only"));
    assert!(out.contains("source_recommendation_id: first_run.repo_implementation"));
    assert!(out.contains("source_recommendation_kind: create_workflow"));
    assert!(out.contains("draft_proposal_status: inactive_review_required"));
    assert!(out.contains("draft_proposal_kind: workflow_draft_proposal"));
    assert!(out.contains("proposed_lifecycle_status: draft"));
    assert!(out.contains("proposed_purpose_code: repo_implementation_workflow"));
    assert!(out.contains("required_authoring_decisions: choose_workflow_id|assign_owner|assign_escalation_contact|define_step_boundaries|define_policy_gates|define_evidence_and_check_obligations|define_side_effect_posture|define_report_handoff_posture"));
    assert!(out.contains(
        "missing_required_fields: workflow_id|owner|escalation|steps|policy_gates|evidence_checks|side_effects|report_handoff"
    ));
    assert!(out.contains("no_files_written: true"));
    assert!(out.contains("no_workflow_registered: true"));
    assert!(out.contains("no_commands_executed: true"));
    assert!(out.contains("no_providers_called: true"));
    assert!(out.contains("no_runtime_state_created: true"));
    assert!(out.contains("privacy_boundary: bounded_codes_only_no_raw_payloads"));
    assert!(out.contains("next_action: review_this_preview_fill_required_authoring_decisions_then_validate_before_promotion"));
    assert!(!out.contains("run_id:"));
    assert!(!out.contains("approval_id:"));
    assert!(!project.state_root().exists());
}

#[test]
fn author_workflow_dry_run_json_is_bounded() {
    let project = TestProject::new("author-workflow-dry-run-json");
    let init = workflow_os(&project, &["init-repo-governance"]);
    assert!(init.status.success(), "{}", stderr(&init));

    let output = workflow_os(
        &project,
        &[
            "--json",
            "author",
            "workflow",
            "--from-recommendation",
            "first_run.assign_ownership",
            "--dry-run",
        ],
    );

    assert!(output.status.success(), "{}", stderr(&output));
    let out = stdout(&output);
    assert!(out.contains(r#""author_workflow_dry_run":{"schema_version":"workflowos.dev/v0""#));
    assert!(out.contains(r#""mode":"author_workflow_dry_run""#));
    assert!(out.contains(r#""status":"preview_only""#));
    assert!(out.contains(r#""source_recommendation_id":"first_run.assign_ownership""#));
    assert!(out.contains(r#""source_recommendation_kind":"assign_ownership""#));
    assert!(out.contains(r#""draft_proposal_status":"inactive_review_required""#));
    assert!(out.contains(r#""draft_proposal_kind":"ownership_update_proposal""#));
    assert!(out.contains(r#""required_authoring_decisions":["assign_owner","assign_escalation_contact","define_authority_context","review_local_vs_enterprise_stewardship"]"#));
    assert!(out.contains(r#""files_written":false"#));
    assert!(out.contains(r#""workflow_registered":false"#));
    assert!(out.contains(r#""commands_executed":false"#));
    assert!(out.contains(r#""providers_called":false"#));
    assert!(out.contains(r#""runtime_state_created":false"#));
    assert!(!out.contains("local-maintainer"));
    assert!(!out.contains("run_id"));
    assert!(!out.contains("approval_id"));
    assert!(!project.state_root().exists());
}

#[test]
fn author_workflow_dry_run_unknown_recommendation_fails_closed_without_leakage() {
    let project = TestProject::new("author-workflow-unknown-recommendation");
    let init = workflow_os(&project, &["init-repo-governance"]);
    assert!(init.status.success(), "{}", stderr(&init));

    let output = workflow_os(
        &project,
        &[
            "author",
            "workflow",
            "--from-recommendation",
            "first_run.not_a_recommendation",
            "--dry-run",
        ],
    );

    assert!(!output.status.success());
    assert!(stderr(&output).contains("cli.workflow_authoring.recommendation_not_found"));
    assert!(!stderr(&output).contains("first_run.not_a_recommendation"));
    assert!(!project.state_root().exists());
}

#[test]
fn author_workflow_dry_run_rejects_secret_like_recommendation_without_leakage() {
    let project = TestProject::new("author-workflow-secret-like-recommendation");
    let init = workflow_os(&project, &["init-repo-governance"]);
    assert!(init.status.success(), "{}", stderr(&init));

    let output = workflow_os(
        &project,
        &[
            "author",
            "workflow",
            "--from-recommendation",
            "first_run.secret-token-authoring",
            "--dry-run",
        ],
    );

    assert!(!output.status.success());
    assert!(stderr(&output).contains("cli.workflow_authoring.unsafe_payload_rejected"));
    assert!(!stderr(&output).contains("secret-token-authoring"));
    assert!(!project.state_root().exists());
}

#[test]
fn author_workflow_output_dry_run_is_non_mutating() {
    let project = TestProject::new("author-workflow-output-dry-run");
    let init = workflow_os(&project, &["init-repo-governance"]);
    assert!(init.status.success(), "{}", stderr(&init));

    let output = workflow_os(
        &project,
        &[
            "author",
            "workflow",
            "--from-recommendation",
            "first_run.repo_implementation",
            "--output",
            "workflows/drafts/repo-implementation.workflow.yml",
            "--dry-run",
        ],
    );

    assert!(output.status.success(), "{}", stderr(&output));
    let out = stdout(&output);
    assert!(out.contains("mode: author_workflow_file_output_dry_run"));
    assert!(out.contains("status: preview_only"));
    assert!(out.contains("output_path: workflows/drafts/repo-implementation.workflow.yml"));
    assert!(out.contains("proposed_workflow_id: draft/repo-implementation"));
    assert!(out.contains("draft_loaded_by_current_project_loader: false"));
    assert!(out.contains("files_written: false"));
    assert!(out.contains("workflow_registered: false"));
    assert!(out.contains("runtime_state_created: false"));
    assert!(!project
        .path()
        .join("workflows/drafts/repo-implementation.workflow.yml")
        .exists());
    assert!(!project.state_root().exists());
}

#[test]
fn author_workflow_output_writes_inactive_draft_without_registration() {
    let project = TestProject::new("author-workflow-output-write");
    let init = workflow_os(&project, &["init-repo-governance"]);
    assert!(init.status.success(), "{}", stderr(&init));

    let output = workflow_os(
        &project,
        &[
            "author",
            "workflow",
            "--from-recommendation",
            "first_run.repo_implementation",
            "--output",
            "workflows/drafts/repo-implementation.workflow.yml",
        ],
    );

    assert!(output.status.success(), "{}", stderr(&output));
    let out = stdout(&output);
    assert!(out.contains("mode: author_workflow_file_output"));
    assert!(out.contains("status: inactive_draft_written"));
    assert!(out.contains("workflow_registered: false"));
    assert!(out.contains("workflow_promoted: false"));
    assert!(out.contains("commands_executed: false"));
    assert!(out.contains("providers_called: false"));
    assert!(out.contains("runtime_state_created: false"));

    let draft_path = project
        .path()
        .join("workflows/drafts/repo-implementation.workflow.yml");
    assert!(draft_path.exists());
    let draft = fs::read_to_string(draft_path).expect("draft can be read");
    assert!(draft.contains("# Workflow OS inactive draft"));
    assert!(draft.contains("# source_recommendation_id: first_run.repo_implementation"));
    assert!(draft.contains("id: draft/repo-implementation"));
    assert!(draft.contains("owner:\n  lifecycle_status: experimental"));
    assert!(draft.contains("disabled_by_default: true"));
    assert!(draft.contains("steps: []"));
    assert!(draft.contains("workflow-os-draft"));
    assert!(!draft.contains("local-maintainer"));
    assert!(!draft.contains("run_id"));
    assert!(!draft.contains("approval_id"));
    assert!(!project.state_root().exists());

    let validate = workflow_os(&project, &["validate"]);
    assert!(
        validate.status.success(),
        "drafts directory must not be loaded as active workflow: {}",
        stderr(&validate)
    );
}

#[test]
fn author_workflow_output_rejects_unsafe_paths_without_leakage() {
    let project = TestProject::new("author-workflow-output-unsafe-path");
    let init = workflow_os(&project, &["init-repo-governance"]);
    assert!(init.status.success(), "{}", stderr(&init));

    let output = workflow_os(
        &project,
        &[
            "author",
            "workflow",
            "--from-recommendation",
            "first_run.repo_implementation",
            "--output",
            "../secret-token.workflow.yml",
        ],
    );

    assert!(!output.status.success());
    assert!(stderr(&output).contains("cli.workflow_authoring.output_path_rejected"));
    assert!(!stderr(&output).contains("secret-token"));
    assert!(!project.state_root().exists());
}

#[test]
fn author_workflow_output_refuses_overwrite() {
    let project = TestProject::new("author-workflow-output-overwrite");
    let init = workflow_os(&project, &["init-repo-governance"]);
    assert!(init.status.success(), "{}", stderr(&init));
    project.write(
        "workflows/drafts/repo-implementation.workflow.yml",
        "existing unmanaged draft",
    );

    let output = workflow_os(
        &project,
        &[
            "author",
            "workflow",
            "--from-recommendation",
            "first_run.repo_implementation",
            "--output",
            "workflows/drafts/repo-implementation.workflow.yml",
        ],
    );

    assert!(!output.status.success());
    assert!(stderr(&output).contains("cli.workflow_authoring.output_exists"));
    let draft = fs::read_to_string(
        project
            .path()
            .join("workflows/drafts/repo-implementation.workflow.yml"),
    )
    .expect("existing draft can be read");
    assert_eq!(draft, "existing unmanaged draft");
    assert!(!project.state_root().exists());
}

#[test]
fn author_workflow_output_rejects_duplicate_workflow_id() {
    let project = TestProject::new("author-workflow-output-duplicate-id");
    project.write_valid_project(false, false);
    project.write(
        "workflows/existing-draft-id.workflow.yml",
        r"
schema_version: workflowos.dev/v0
id: draft/repo-implementation
version: v0
display_name: Existing Draft Id
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
          value: hello
        to: request
    policy_requirements:
      - id: local/allow
    terminal_behavior: fail_workflow
cancellation_behavior: stop
audit_requirements:
  required: true
  events:
    - RunCreated
observability_requirements:
  metrics:
    - workflow_latency
",
    );

    let output = workflow_os(
        &project,
        &[
            "author",
            "workflow",
            "--from-recommendation",
            "first_run.repo_implementation",
            "--output",
            "workflows/drafts/repo-implementation.workflow.yml",
        ],
    );

    assert!(!output.status.success());
    assert!(
        stderr(&output).contains("cli.workflow_authoring.workflow_id_conflict"),
        "{}",
        stderr(&output)
    );
    assert!(!project
        .path()
        .join("workflows/drafts/repo-implementation.workflow.yml")
        .exists());
    assert!(!project.state_root().exists());
}

#[test]
fn author_workflow_preflight_blocks_incomplete_generated_draft_without_mutation() {
    let project = TestProject::new("author-workflow-preflight-incomplete");
    let init = workflow_os(&project, &["init-repo-governance"]);
    assert!(init.status.success(), "{}", stderr(&init));
    let write = workflow_os(
        &project,
        &[
            "author",
            "workflow",
            "--from-recommendation",
            "first_run.repo_implementation",
            "--output",
            "workflows/drafts/repo-implementation.workflow.yml",
        ],
    );
    assert!(write.status.success(), "{}", stderr(&write));

    let preflight = workflow_os(
        &project,
        &[
            "author",
            "workflow",
            "preflight",
            "--draft",
            "workflows/drafts/repo-implementation.workflow.yml",
        ],
    );

    assert!(!preflight.status.success());
    let out = stdout(&preflight);
    assert!(out.contains("mode: author_workflow_promotion_preflight"));
    assert!(out.contains("status: promotion_blocked"));
    assert!(out.contains("candidate_workflow_id: draft/repo-implementation"));
    assert!(out.contains("workflow_id_still_draft_namespace"));
    assert!(out.contains("owner_posture_incomplete"));
    assert!(out.contains("escalation_posture_incomplete"));
    assert!(out.contains("triggers_missing"));
    assert!(out.contains("steps_missing"));
    assert!(out.contains("validation_error:validation.workflow.triggers_missing"));
    assert!(out.contains("validation_error:validation.workflow.steps_missing"));
    assert!(out.contains("files_written: false"));
    assert!(out.contains("workflow_registered: false"));
    assert!(out.contains("workflow_promoted: false"));
    assert!(out.contains("runtime_state_created: false"));
    assert!(stderr(&preflight).contains("cli.workflow_authoring.preflight_blocked"));
    assert!(!project.state_root().exists());
}

#[test]
fn author_workflow_preflight_passes_complete_draft_without_promotion() {
    let project = TestProject::new("author-workflow-preflight-valid");
    let init = workflow_os(&project, &["init-repo-governance"]);
    assert!(init.status.success(), "{}", stderr(&init));
    project.write(
        "workflows/drafts/repo-implementation.workflow.yml",
        r"
schema_version: workflowos.dev/v0
id: local/repo-implementation
version: v0
display_name: Repo Implementation
description: Governed implementation workflow candidate for review.
owner:
  owning_team: workflow-stewards
  maintainer: user/workflow-steward
  escalation_contact: user/workflow-escalation
  lifecycle_status: stable
autonomy_level: level_1
triggers:
  - id: manual-start
    kind: manual
steps:
  - id: report
    skill_ref:
      id: local/first-run-report
      version: v0
    input_mapping:
      - from:
          type: literal
          value: bounded-governed-implementation
        to: task
    policy_requirements:
      - id: local/allow
    approval_policy:
      policy:
        id: default/governed-work
    terminal_behavior: fail_workflow
cancellation_behavior: stop
audit_requirements:
  required: true
  events:
    - RunCreated
observability_requirements:
  tracing: true
  latency_tracking: true
",
    );

    let preflight = workflow_os(
        &project,
        &[
            "author",
            "workflow",
            "preflight",
            "--draft",
            "workflows/drafts/repo-implementation.workflow.yml",
        ],
    );

    assert!(preflight.status.success(), "{}", stderr(&preflight));
    let out = stdout(&preflight);
    assert!(out.contains("status: promotable_preflight_passed"));
    assert!(out.contains("candidate_workflow_id: local/repo-implementation"));
    assert!(out.contains("blockers: none"));
    assert!(out.contains("steward_approval_required_before_active_promotion"));
    assert!(out.contains("workflow_registered: false"));
    assert!(out.contains("workflow_promoted: false"));
    assert!(out.contains("commands_executed: false"));
    assert!(out.contains("providers_called: false"));
    assert!(out.contains("runtime_state_created: false"));
    assert!(!project
        .path()
        .join("workflows/repo-implementation.workflow.yml")
        .exists());
    assert!(!project.state_root().exists());
}

#[test]
fn author_workflow_preflight_rejects_duplicate_active_workflow_id() {
    let project = TestProject::new("author-workflow-preflight-duplicate-id");
    let init = workflow_os(&project, &["init-repo-governance"]);
    assert!(init.status.success(), "{}", stderr(&init));
    project.write(
        "workflows/drafts/repo-implementation.workflow.yml",
        r"
schema_version: workflowos.dev/v0
id: local/first-run-governance
version: v0
display_name: Duplicate Workflow
description: Duplicate active workflow id candidate.
owner:
  owning_team: workflow-stewards
  maintainer: user/workflow-steward
  escalation_contact: user/workflow-escalation
  lifecycle_status: stable
autonomy_level: level_1
triggers:
  - id: manual-start
    kind: manual
steps:
  - id: report
    skill_ref:
      id: local/first-run-report
      version: v0
    input_mapping:
      - from:
          type: literal
          value: duplicate-id-check
        to: task
    policy_requirements:
      - id: local/allow
    approval_policy:
      policy:
        id: default/governed-work
    terminal_behavior: fail_workflow
cancellation_behavior: stop
audit_requirements:
  required: true
  events:
    - RunCreated
observability_requirements:
  tracing: true
",
    );

    let preflight = workflow_os(
        &project,
        &[
            "author",
            "workflow",
            "preflight",
            "--draft",
            "workflows/drafts/repo-implementation.workflow.yml",
        ],
    );

    assert!(!preflight.status.success());
    let out = stdout(&preflight);
    assert!(out.contains("active_workflow_id_conflict"));
    assert!(out.contains("validation_error:validation.workflow.duplicate_id"));
    assert!(stderr(&preflight).contains("cli.workflow_authoring.preflight_blocked"));
    assert!(!project.state_root().exists());
}

#[test]
fn author_workflow_preflight_json_is_bounded_and_non_mutating() {
    let project = TestProject::new("author-workflow-preflight-json");
    let init = workflow_os(&project, &["init-repo-governance"]);
    assert!(init.status.success(), "{}", stderr(&init));
    project.write(
        "workflows/drafts/repo-implementation.workflow.yml",
        r"
schema_version: workflowos.dev/v0
id: local/repo-implementation
version: v0
display_name: Repo Implementation
description: Governed implementation workflow candidate for review.
owner:
  owning_team: workflow-stewards
  maintainer: user/workflow-steward
  escalation_contact: user/workflow-escalation
  lifecycle_status: stable
autonomy_level: level_1
triggers:
  - id: manual-start
    kind: manual
steps:
  - id: report
    skill_ref:
      id: local/first-run-report
      version: v0
    input_mapping:
      - from:
          type: literal
          value: bounded-governed-implementation
        to: task
    policy_requirements:
      - id: local/allow
    approval_policy:
      policy:
        id: default/governed-work
    terminal_behavior: fail_workflow
cancellation_behavior: stop
audit_requirements:
  required: true
  events:
    - RunCreated
observability_requirements:
  tracing: true
  latency_tracking: true
",
    );

    let preflight = workflow_os(
        &project,
        &[
            "--json",
            "author",
            "workflow",
            "preflight",
            "--draft",
            "workflows/drafts/repo-implementation.workflow.yml",
        ],
    );

    assert!(preflight.status.success(), "{}", stderr(&preflight));
    let out = stdout(&preflight);
    assert!(out.contains(r#""mode":"author_workflow_promotion_preflight""#));
    assert!(out.contains(r#""status":"promotable_preflight_passed""#));
    assert!(out.contains(r#""candidate_workflow_id":"local/repo-implementation""#));
    assert!(out.contains(r#""blockers":[]"#));
    assert!(out.contains(r#""workflow_registered":false"#));
    assert!(out.contains(r#""workflow_promoted":false"#));
    assert!(out.contains(r#""runtime_state_created":false"#));
    assert!(!out.contains("bounded-governed-implementation"));
    assert!(!project.state_root().exists());
}

#[test]
fn author_workflow_preflight_rejects_unsafe_path_without_leakage() {
    let project = TestProject::new("author-workflow-preflight-unsafe-path");
    let init = workflow_os(&project, &["init-repo-governance"]);
    assert!(init.status.success(), "{}", stderr(&init));

    let preflight = workflow_os(
        &project,
        &[
            "author",
            "workflow",
            "preflight",
            "--draft",
            "workflows/drafts/secret-token.workflow.yml",
        ],
    );

    assert!(!preflight.status.success());
    assert!(stderr(&preflight).contains("cli.workflow_authoring.output_path_rejected"));
    assert!(!stderr(&preflight).contains("secret-token"));
    assert!(!project.state_root().exists());
}

#[test]
fn first_run_detects_package_metadata_without_copying_script_payloads() {
    let project = TestProject::new("first-run-package-metadata");
    project.write(
        "package.json",
        r#"{
  "name": "metadata-fixture",
  "scripts": {
    "build": "tsc",
    "test": "node test.js --token=secret-package-script-token",
    "lint": "xo"
  },
  "devDependencies": {
    "typescript": "5.0.0",
    "secret-dependency-marker": "1.0.0"
  }
}"#,
    );
    project.write("package-lock.json", "{}");
    project.write("tsconfig.json", "{}");
    project.write("source/index.ts", "export const value = 1;");
    project.write("test/test.ts", "import '../source/index.ts';");
    project.write(".github/workflows/ci.yml", "name: CI");
    let init = workflow_os(&project, &["init-repo-governance"]);
    assert!(init.status.success(), "{}", stderr(&init));

    let output = workflow_os(&project, &["first-run", "--verbose"]);

    assert!(output.status.success(), "{}", stderr(&output));
    let out = stdout(&output);
    assert!(out.contains("safe_repo_metadata:"));
    assert!(out.contains("  package_json: present"));
    assert!(out.contains("  package_manager: npm"));
    assert!(out.contains("  package_scripts: build|lint|test"));
    assert!(out.contains("  typescript: present"));
    assert!(out.contains("  typescript_markers: dependency_typescript|tsconfig_json"));
    assert!(out.contains("  github_workflows: 1"));
    assert!(out.contains("  source_dirs: source"));
    assert!(out.contains("  test_dirs: test"));
    assert!(out.contains("workflow_discovery_recommendations: 10"));
    assert!(out.contains(
        "  - detected TypeScript/package metadata can guide implementation and validation workflows"
    ));
    assert!(out.contains("workflow_discovery_recommendation: id=first_run.typescript_implementation kind=create_workflow target=project#1 status=review_only summary=typescript_implementation_workflow"));
    assert!(out.contains("workflow_candidate: first_run.typescript_implementation"));
    assert!(out.contains(
        "rationale=repo_metadata.package_json_present|repo_metadata.typescript_detected"
    ));
    assert!(out.contains("workflow_discovery_recommendation: id=first_run.package_validation_obligations kind=add_evidence_check_requirements target=project#1 status=review_only summary=add_package_validation_obligations"));
    assert!(out.contains("validation_candidate: first_run.package_validation_obligations"));
    assert!(out.contains("review TypeScript package metadata and decide required build, test, lint, and typecheck obligations"));
    assert!(!out.contains("secret-package-script-token"));
    assert!(!out.contains("secret-dependency-marker"));
    assert!(!out.contains("node test.js"));
    assert!(!out.contains("\"tsc\""));
    assert!(!out.contains("xo"));
    assert!(!project.state_root().exists());
}

#[test]
fn first_run_json_reports_bounded_package_metadata() {
    let project = TestProject::new("first-run-package-metadata-json");
    project.write(
        "package.json",
        r#"{
  "scripts": {
    "build": "secret-build-command",
    "prepare": "secret-prepare-command",
    "release": "secret-release-command"
  },
  "dependencies": {
    "tsx": "4.0.0"
  }
}"#,
    );
    project.write("pnpm-lock.yaml", "");
    project.write("src/lib.ts", "export {};");
    project.write("tests/lib.test.ts", "import '../src/lib.ts';");
    let init = workflow_os(&project, &["init-repo-governance"]);
    assert!(init.status.success(), "{}", stderr(&init));

    let output = workflow_os(&project, &["--json", "first-run"]);

    assert!(output.status.success(), "{}", stderr(&output));
    let out = stdout(&output);
    assert!(out.contains(r#""safe_repo_metadata":{"package_json":{"present":true"#));
    assert!(out.contains(r#""package_manager":"pnpm""#));
    assert!(out.contains(r#""common_script_keys":["build","prepare","release"]"#));
    assert!(out.contains(r#""typescript_detected":true"#));
    assert!(out.contains(r#""typescript_markers":["dependency_tsx"]"#));
    assert!(out.contains(r#""conventional_source_dirs":["src"]"#));
    assert!(out.contains(r#""conventional_test_dirs":["tests"]"#));
    assert!(out.contains(r#""id":"first_run.typescript_implementation""#));
    assert!(out.contains(r#""id":"first_run.package_validation_obligations""#));
    assert!(out.contains(r#""workflow_candidate: first_run.typescript_implementation""#));
    assert!(out.contains(r#""validation_candidate: first_run.package_validation_obligations""#));
    assert!(!out.contains("secret-build-command"));
    assert!(!out.contains("secret-prepare-command"));
    assert!(!out.contains("secret-release-command"));
    assert!(!out.contains(r#""tsx":"#));
    assert!(!project.state_root().exists());
}

#[test]
fn first_run_detects_broader_ecosystem_metadata_without_copying_payloads() {
    let project = TestProject::new("first-run-broader-ecosystem-metadata");
    project.write(
        "Cargo.toml",
        "[package]\nname = \"secret-rust-package-name\"\n",
    );
    project.write("Cargo.lock", "secret-cargo-lock-payload");
    project.write(
        "pyproject.toml",
        "[project]\nname = \"secret-python-project-name\"\n",
    );
    project.write("uv.lock", "secret-python-lock-payload");
    project.write("go.mod", "module example.com/secret-go-module\n");
    project.write("go.sum", "secret-go-sum-payload");
    project.write(".github/workflows/ci.yaml", "name: secret-ci-workflow-name");
    project.write("src/lib.rs", "pub fn secret_source_payload() {}");
    project.write("tests/integration.rs", "secret-test-payload");
    let init = workflow_os(&project, &["init-repo-governance"]);
    assert!(init.status.success(), "{}", stderr(&init));

    let output = workflow_os(&project, &["first-run", "--verbose"]);

    assert!(output.status.success(), "{}", stderr(&output));
    let out = stdout(&output);
    assert!(out
        .contains("  - detected Rust metadata can guide implementation and validation workflows"));
    assert!(out.contains("  cargo_toml: present"));
    assert!(out.contains("  cargo_lock: present"));
    assert!(out.contains("  pyproject_toml: present"));
    assert!(out.contains("  python_lock_files: uv_lock"));
    assert!(out.contains("  go_mod: present"));
    assert!(out.contains("  go_sum: present"));
    assert!(out.contains("  github_workflows: 1"));
    assert!(out.contains("workflow_discovery_recommendation: id=first_run.rust_implementation kind=create_workflow target=project#1 status=review_only summary=rust_implementation_workflow"));
    assert!(out.contains("workflow_candidate: first_run.rust_implementation"));
    assert!(out.contains("workflow_discovery_recommendation: id=first_run.rust_validation_obligations kind=add_evidence_check_requirements target=project#1 status=review_only summary=add_rust_validation_obligations"));
    assert!(out.contains("validation_candidate: first_run.rust_validation_obligations"));
    assert!(out.contains("workflow_discovery_recommendation: id=first_run.python_implementation kind=create_workflow target=project#1 status=review_only summary=python_implementation_workflow"));
    assert!(out.contains("workflow_discovery_recommendation: id=first_run.python_validation_obligations kind=add_evidence_check_requirements target=project#1 status=review_only summary=add_python_validation_obligations"));
    assert!(out.contains("workflow_discovery_recommendation: id=first_run.go_implementation kind=create_workflow target=project#1 status=review_only summary=go_implementation_workflow"));
    assert!(out.contains("workflow_discovery_recommendation: id=first_run.go_validation_obligations kind=add_evidence_check_requirements target=project#1 status=review_only summary=add_go_validation_obligations"));
    assert!(out.contains("workflow_discovery_recommendation: id=first_run.github_actions_ci_evidence kind=add_evidence_check_requirements target=project#1 status=review_only summary=add_github_actions_ci_evidence_obligations"));
    assert!(out.contains(
        "review Rust metadata and decide required fmt, clippy, test, and release obligations"
    ));
    assert!(out.contains("review Python metadata and decide required test, lint, typecheck, and packaging obligations"));
    assert!(out.contains(
        "review Go metadata and decide required test, vet, build, and module obligations"
    ));
    assert!(
        out.contains("review GitHub Actions workflow presence and decide CI evidence obligations")
    );
    assert!(!out.contains("secret-rust-package-name"));
    assert!(!out.contains("secret-cargo-lock-payload"));
    assert!(!out.contains("secret-python-project-name"));
    assert!(!out.contains("secret-python-lock-payload"));
    assert!(!out.contains("secret-go-module"));
    assert!(!out.contains("secret-go-sum-payload"));
    assert!(!out.contains("secret-ci-workflow-name"));
    assert!(!out.contains("secret_source_payload"));
    assert!(!out.contains("secret-test-payload"));
    assert!(!project.state_root().exists());
}

#[test]
fn first_run_json_reports_broader_ecosystem_metadata_bounded() {
    let project = TestProject::new("first-run-broader-ecosystem-metadata-json");
    project.write("Cargo.toml", "[package]\nname = \"secret-rust-json\"\n");
    project.write("Cargo.lock", "secret-cargo-json-lock");
    project.write(
        "pyproject.toml",
        "[project]\nname = \"secret-python-json\"\n",
    );
    project.write("poetry.lock", "secret-poetry-lock");
    project.write("go.mod", "module example.com/secret-json-module\n");
    project.write("go.sum", "secret-go-json-sum");
    project.write(
        ".github/workflows/release.yml",
        "name: secret-release-workflow",
    );
    let init = workflow_os(&project, &["init-repo-governance"]);
    assert!(init.status.success(), "{}", stderr(&init));

    let output = workflow_os(&project, &["--json", "first-run"]);

    assert!(output.status.success(), "{}", stderr(&output));
    let out = stdout(&output);
    assert!(out.contains(r#""cargo_toml_present":true"#));
    assert!(out.contains(r#""cargo_lock_present":true"#));
    assert!(out.contains(r#""pyproject_toml_present":true"#));
    assert!(out.contains(r#""python_lock_files":["poetry_lock"]"#));
    assert!(out.contains(r#""go_mod_present":true"#));
    assert!(out.contains(r#""go_sum_present":true"#));
    assert!(out.contains(r#""github_workflow_count":1"#));
    assert!(out.contains(r#""github_actions_detected":true"#));
    assert!(out.contains(r#""id":"first_run.rust_implementation""#));
    assert!(out.contains(r#""id":"first_run.python_implementation""#));
    assert!(out.contains(r#""id":"first_run.go_implementation""#));
    assert!(out.contains(r#""id":"first_run.github_actions_ci_evidence""#));
    assert!(!out.contains("secret-rust-json"));
    assert!(!out.contains("secret-cargo-json-lock"));
    assert!(!out.contains("secret-python-json"));
    assert!(!out.contains("secret-poetry-lock"));
    assert!(!out.contains("secret-json-module"));
    assert!(!out.contains("secret-go-json-sum"));
    assert!(!out.contains("secret-release-workflow"));
    assert!(!project.state_root().exists());
}

#[test]
fn first_run_discloses_configured_owner_without_printing_owner_values() {
    let project = TestProject::new("first-run-configured-owner");
    let init = workflow_os(&project, &["init-repo-governance"]);
    assert!(init.status.success(), "{}", stderr(&init));
    let workflow_path = project
        .path()
        .join("workflows")
        .join("first-run-governance.workflow.yml");
    let workflow = fs::read_to_string(&workflow_path)
        .expect("workflow scaffold exists")
        .replace("local-maintainers", "platform-governance")
        .replace("local-maintainer", "platform-owner");
    fs::write(&workflow_path, workflow).expect("workflow owner is updated");
    let skill_path = project
        .path()
        .join("skills")
        .join("first-run-report.skill.yml");
    let skill = fs::read_to_string(&skill_path)
        .expect("skill scaffold exists")
        .replace("local-maintainers", "platform-governance")
        .replace("local-maintainer", "platform-owner");
    fs::write(&skill_path, skill).expect("skill owner is updated");

    let output = workflow_os(&project, &["first-run", "--verbose"]);

    assert!(output.status.success(), "{}", stderr(&output));
    let out = stdout(&output);
    assert!(out.contains("ownership: configured"));
    assert!(out.contains("escalation: configured"));
    assert!(!out.contains("platform-governance"));
    assert!(!out.contains("platform-owner"));
}

#[test]
fn first_run_ownership_escalation_check_reports_missing_metadata_without_leaking_values() {
    let project = TestProject::new("first-run-missing-owner-escalation");
    let init = workflow_os(&project, &["init-repo-governance"]);
    assert!(init.status.success(), "{}", stderr(&init));
    let workflow_path = project
        .path()
        .join("workflows")
        .join("first-run-governance.workflow.yml");
    let workflow = fs::read_to_string(&workflow_path)
        .expect("workflow scaffold exists")
        .replace("  owning_team: local-maintainers\n", "")
        .replace("  maintainer: local-maintainer\n", "")
        .replace("  escalation_contact: local-maintainer\n", "");
    fs::write(&workflow_path, workflow).expect("workflow owner metadata is updated");
    let skill_path = project
        .path()
        .join("skills")
        .join("first-run-report.skill.yml");
    let skill = fs::read_to_string(&skill_path)
        .expect("skill scaffold exists")
        .replace("  owning_team: local-maintainers\n", "")
        .replace("  maintainer: local-maintainer\n", "")
        .replace("  escalation_contact: local-maintainer\n", "");
    fs::write(&skill_path, skill).expect("skill owner metadata is updated");

    let output = workflow_os(&project, &["first-run", "--verbose"]);

    assert!(output.status.success(), "{}", stderr(&output));
    let out = stdout(&output);
    assert!(out.contains("ownership: missing"));
    assert!(out.contains("escalation: missing"));
    assert!(out.contains("ownership_escalation_check: warnings"));
    assert!(out.contains("ownership_missing_owner: 2"));
    assert!(out.contains("escalation_missing_contact: 2"));
    assert!(out.contains(
        "ownership_escalation_finding: target=workflow#1 code=ownership.missing_owner severity=warning"
    ));
    assert!(out.contains(
        "ownership_escalation_finding: target=skill#1 code=escalation.missing_contact severity=warning"
    ));
    assert!(!out.contains("local-maintainer"));
    assert!(!out.contains("local-maintainers"));
    assert!(!project.state_root().exists());
}

#[test]
fn first_run_missing_manifest_fails_actionably_without_writes() {
    let project = TestProject::new("first-run-missing");

    let output = workflow_os(&project, &["first-run"]);

    assert!(!output.status.success());
    assert!(stderr(&output).contains("cli.first_run.manifest_missing"));
    assert!(stderr(&output).contains("init-repo-governance"));
    assert!(!project.state_root().exists());
}

#[test]
fn first_run_invalid_project_fails_without_leaking_secret_like_contents() {
    let project = TestProject::new("first-run-invalid");
    project.write("workflow-os.yml", "secret-token-first-run-invalid: [");

    let output = workflow_os(&project, &["first-run"]);

    assert!(!output.status.success());
    assert!(stderr(&output).contains("cli.first_run.validation_failed"));
    assert!(!stderr(&output).contains("secret-token-first-run-invalid"));
    assert!(!project.state_root().exists());
}

#[test]
fn first_run_does_not_copy_raw_repo_payloads_or_create_artifacts() {
    let project = TestProject::new("first-run-no-payload-copy");
    let init = workflow_os(&project, &["init-repo-governance"]);
    assert!(init.status.success(), "{}", stderr(&init));
    let manifest_path = project.path().join("workflow-os.yml");
    let manifest = fs::read_to_string(&manifest_path)
        .expect("manifest exists")
        .replace(
            "value: first-run",
            "value: raw-config-value-should-not-print",
        );
    fs::write(manifest_path, manifest).expect("manifest is updated with raw config marker");
    let workflow_path = project
        .path()
        .join("workflows")
        .join("first-run-governance.workflow.yml");
    let workflow = fs::read_to_string(&workflow_path)
        .expect("workflow scaffold exists")
        .replace(
            "First-run governance posture",
            "raw-mapping-literal-should-not-print",
        );
    fs::write(workflow_path, workflow).expect("workflow scaffold is updated");
    project.write(
        "src/app.rs",
        "raw-provider-payload secret-token-command-output raw-parser-payload",
    );

    let output = workflow_os(&project, &["first-run"]);

    assert!(output.status.success(), "{}", stderr(&output));
    let out = stdout(&output);
    assert!(!out.contains("raw-provider-payload"));
    assert!(!out.contains("secret-token-command-output"));
    assert!(!out.contains("raw-parser-payload"));
    assert!(!out.contains("raw-config-value-should-not-print"));
    assert!(!out.contains("raw-mapping-literal-should-not-print"));
    assert!(!project.state_root().exists());
    assert!(!project.path().join(".workflow-os").join("reports").exists());
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
            "docs-check",
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
            "docs-check",
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
    assert!(stdout(&output).contains("schemas: unavailable_optional"));
    assert!(!stdout(&output).contains("schemas: failed"));
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
