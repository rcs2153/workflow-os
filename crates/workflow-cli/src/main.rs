#![deny(unsafe_code)]
#![doc = "Command-line interface for Workflow OS v0."]

use std::collections::BTreeMap;
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use workflow_core::{
    ci_actions, github_actions, github_actions_read_request, github_read_request, jira_actions,
    jira_read_request, load_project, validate_loaded_project, ActorId, AdapterOperationMode,
    AdapterPolicyPrecheck, AdapterRunScope, AdapterTelemetryRecord, AdapterTelemetryStore,
    ApprovalDecisionKind, BackendHealthCheck, CorrelationId, Diagnostic,
    GitHubActionsFixtureClient, GitHubActionsReadOnlyAdapter, GitHubActionsReadOnlyConfig,
    GitHubFixtureClient, GitHubReadOnlyAdapter, GitHubReadOnlyConfig, JiraFixtureClient,
    JiraReadOnlyAdapter, JiraReadOnlyConfig, LocalApprovalDecisionRequest, LocalExecutionRequest,
    LocalExecutor, LocalSkillRegistry, LocalStateBackend, LocalStateInspection, LocalStateIssue,
    LocalStateIssueSeverity, SkillDefinition, SkillHandler, SkillInput, SkillOutput, StateBackend,
    WorkflowId, WorkflowOsError, WorkflowOsErrorKind, WorkflowRun, WorkflowRunEventKind,
    WorkflowRunEventKindName, WorkflowRunId, WorkflowRunStatus,
};

const EXIT_OK: i32 = 0;
const EXIT_VALIDATION: i32 = 1;
const EXIT_USAGE: i32 = 2;
const EXIT_RUNTIME: i32 = 3;
const AGENT_HARNESS_BEGIN: &str = "<!-- BEGIN WORKFLOW OS AGENT HARNESS -->";
const AGENT_HARNESS_END: &str = "<!-- END WORKFLOW OS AGENT HARNESS -->";

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let exit = match run(&args) {
        Ok(()) => EXIT_OK,
        Err(error) => {
            eprintln!("error[{}]: {}", error.code(), error.message());
            for diagnostic in error.diagnostics() {
                eprintln!("{diagnostic}");
            }
            exit_code_for_error(&error)
        }
    };
    std::process::exit(exit);
}

fn run(args: &[String]) -> Result<(), WorkflowOsError> {
    let invocation = Invocation::parse(args)?;
    match &invocation.command {
        Command::Validate => validate_command(&invocation),
        Command::Run {
            workflow_id,
            run_id,
        } => run_command(&invocation, workflow_id, run_id.as_deref()),
        Command::Status { run_id } => status_command(&invocation, run_id),
        Command::Approve {
            run_id,
            approval_id,
            actor,
            reason,
            deny,
        } => approve_command(
            &invocation,
            run_id,
            approval_id,
            actor.as_deref(),
            reason.as_deref(),
            *deny,
        ),
        Command::Inspect { run_id } => inspect_command(&invocation, run_id),
        Command::Doctor => doctor_command(&invocation),
        Command::DoctorState => doctor_state_command(&invocation),
        Command::InitAgentHarness {
            output_dir,
            agent,
            force,
            dry_run,
        } => {
            init_agent_harness_command(&invocation, output_dir.as_deref(), *agent, *force, *dry_run)
        }
        Command::Help => {
            print_help();
            Ok(())
        }
    }
}

fn validate_command(invocation: &Invocation) -> Result<(), WorkflowOsError> {
    let load_result = load_project(&invocation.project_dir);
    let validation = validate_loaded_project(&load_result);
    if invocation.json {
        print_diagnostics_json(&validation.diagnostics);
    } else {
        print_diagnostics_text(&validation.diagnostics);
        if !validation.has_errors() {
            println!("Project is valid.");
        }
    }
    if validation.has_errors() {
        return Err(WorkflowOsError::validation(
            "cli.validate.failed",
            "project validation failed",
        )
        .with_diagnostics(validation.diagnostics));
    }
    Ok(())
}

fn run_command(
    invocation: &Invocation,
    workflow_id: &str,
    run_id: Option<&str>,
) -> Result<(), WorkflowOsError> {
    let workflow_id = WorkflowId::new(workflow_id)?;
    let run_id = run_id.map(WorkflowRunId::new).transpose()?;
    let backend = local_backend(invocation)?;
    let registry = local_registry(invocation)?;
    let executor = LocalExecutor::new(&backend, &registry);
    let request = LocalExecutionRequest {
        project_root: invocation.project_dir.clone(),
        workflow_id,
        run_id,
        correlation_id: CorrelationId::generate(),
        actor: ActorId::new("system/workflow-os-cli")?,
    };
    let run = executor.execute(&request)?;
    print_run_summary(invocation, &run);
    if run.snapshot.status == WorkflowRunStatus::Failed {
        return Err(WorkflowOsError::new(
            WorkflowOsErrorKind::InvalidState,
            "cli.run.failed",
            "workflow run failed; inspect the run event history for details",
        ));
    }
    Ok(())
}

fn status_command(invocation: &Invocation, run_id: &str) -> Result<(), WorkflowOsError> {
    let run_id = WorkflowRunId::new(run_id)?;
    let backend = local_backend(invocation)?;
    let run = backend.rehydrate_run(&run_id)?;
    if invocation.json {
        println!("{}", run_status_json(&run));
    } else {
        println!("run_id: {}", run.snapshot.identity.run_id);
        println!("schema_version: {}", run.snapshot.identity.schema_version);
        println!("status: {:?}", run.snapshot.status);
        if let Some(step) = current_step(&run) {
            println!("current_step: {step}");
        }
        if run.snapshot.status.is_terminal() {
            println!("terminal: true");
        }
        println!("last_event_id: {}", run.snapshot.last_event_id);
        if let Some(last) = run.events.last() {
            println!("last_event_at: {}", last.timestamp);
        }
    }
    Ok(())
}

fn approve_command(
    invocation: &Invocation,
    run_id: &str,
    approval_id: &str,
    actor: Option<&str>,
    reason: Option<&str>,
    deny: bool,
) -> Result<(), WorkflowOsError> {
    if deny && reason.is_none() {
        return Err(usage(
            "approval denial requires --reason so the event log captures operator intent",
        ));
    }
    let backend = local_backend(invocation)?;
    let registry = local_registry(invocation)?;
    let executor = LocalExecutor::new(&backend, &registry);
    let decision = if deny {
        ApprovalDecisionKind::Denied
    } else {
        ApprovalDecisionKind::Granted
    };
    let request = LocalApprovalDecisionRequest {
        project_root: invocation.project_dir.clone(),
        run_id: WorkflowRunId::new(run_id)?,
        approval_id: approval_id.to_owned(),
        decision,
        actor: ActorId::new(actor.unwrap_or("user/local-approver"))?,
        reason: reason
            .unwrap_or("approved through workflow-os CLI")
            .to_owned(),
        correlation_id: CorrelationId::generate(),
    };
    let run = executor.decide_approval(request)?;
    print_approval_summary(invocation, &run, decision);
    Ok(())
}

fn inspect_command(invocation: &Invocation, run_id: &str) -> Result<(), WorkflowOsError> {
    let run_id = WorkflowRunId::new(run_id)?;
    let backend = local_backend(invocation)?;
    let run = backend.rehydrate_run(&run_id)?;
    let adapter_audit = backend.read_adapter_audit_records(&run_id)?;
    let adapter_observability = backend.read_adapter_observability_records(&run_id)?;
    if invocation.json {
        println!(
            "{}",
            inspect_json(&run, &adapter_audit, &adapter_observability)
        );
    } else {
        println!("run_id: {}", run.snapshot.identity.run_id);
        println!("workflow_id: {}", run.snapshot.identity.workflow_id);
        println!("schema_version: {}", run.snapshot.identity.schema_version);
        println!(
            "workflow_version: {}",
            run.snapshot.identity.workflow_version
        );
        println!("spec_hash: {}", run.snapshot.identity.spec_content_hash);
        println!("status: {:?}", run.snapshot.status);
        if let Some(failure) = &run.snapshot.failure {
            println!("failure_code: {}", failure.code);
            println!("failure_message: {}", failure.message);
        }
        println!("events:");
        for event in &run.events {
            println!(
                "  {} {} {}",
                event.sequence_number,
                event.event_id,
                format_event(&event.kind, event.kind())
            );
        }
        if !run.snapshot.approval_requests.is_empty() {
            println!("approvals: {}", run.snapshot.approval_requests.len());
        }
        if !run.snapshot.retries.is_empty() {
            println!("retries: {}", run.snapshot.retries.len());
        }
        if !run.snapshot.escalations.is_empty() {
            println!("escalations: {}", run.snapshot.escalations.len());
        }
        if !adapter_audit.is_empty() {
            println!("adapter_telemetry: {}", adapter_audit.len());
            for record in &adapter_audit {
                println!(
                    "  {:?} {} action={} capability={:?} mode={:?} policy_precheck={:?} status={:?}",
                    record.adapter_kind,
                    record.adapter_id,
                    record.action,
                    record.capability,
                    record.operation_mode,
                    record.policy_precheck,
                    record.status
                );
            }
        }
        if !adapter_observability.is_empty() {
            println!("adapter_observability: {}", adapter_observability.len());
        }
    }
    Ok(())
}

fn doctor_command(invocation: &Invocation) -> Result<(), WorkflowOsError> {
    let manifest_exists = invocation.project_dir.join("workflow-os.yml").is_file();
    let schemas_exist =
        PathBuf::from("schemas").is_dir() || invocation.project_dir.join("schemas").is_dir();
    let backend = local_backend(invocation)?;
    let health = backend.health_check()?;
    let load_result = load_project(&invocation.project_dir);
    let ok = manifest_exists && health.healthy && !load_result.has_errors();

    if invocation.json {
        println!(
            "{{\"ok\":{},\"manifest_exists\":{},\"backend\":{},\"schemas_available\":{},\"diagnostics\":{}}}",
            ok,
            manifest_exists,
            backend_health_json(&health),
            schemas_exist,
            diagnostics_json(&load_result.diagnostics)
        );
    } else {
        println!("project_manifest: {}", status_word(manifest_exists));
        println!(
            "backend: {} ({})",
            status_word(health.healthy),
            health.backend
        );
        println!("backend_message: {}", health.message);
        println!("schemas: {}", status_word(schemas_exist));
        print_diagnostics_text(&load_result.diagnostics);
    }

    if ok {
        Ok(())
    } else {
        Err(
            WorkflowOsError::validation("cli.doctor.failed", "doctor found local project issues")
                .with_diagnostics(load_result.diagnostics),
        )
    }
}

fn doctor_state_command(invocation: &Invocation) -> Result<(), WorkflowOsError> {
    let backend = LocalStateBackend::for_inspection(invocation.state_dir());
    let inspection = backend.inspect_state();
    if invocation.json {
        println!("{}", state_inspection_json(&inspection));
    } else {
        println!(
            "state_backend: {} ({})",
            status_word(inspection.healthy),
            inspection.backend
        );
        println!("state_root: {}", inspection.root.display());
        if inspection.issues.is_empty() {
            println!("issues: none");
        } else {
            println!("issues:");
            for issue in &inspection.issues {
                println!("  {}", format_state_issue(issue));
            }
        }
    }

    if inspection.healthy {
        Ok(())
    } else {
        Err(WorkflowOsError::new(
            WorkflowOsErrorKind::InvalidState,
            "cli.doctor_state.unhealthy",
            "local state inspection found unhealthy state",
        ))
    }
}

fn init_agent_harness_command(
    invocation: &Invocation,
    output_dir: Option<&Path>,
    agent: AgentHarnessFlavor,
    force: bool,
    dry_run: bool,
) -> Result<(), WorkflowOsError> {
    let root = output_dir.map_or_else(|| invocation.project_dir.clone(), Path::to_path_buf);
    let agents_path = root.join("AGENTS.md");
    let prompt_path = root.join(".workflow-os").join("agent-harness-prompt.md");
    let agents_content = scaffold_file_content(
        &agents_path,
        &agent_harness_agents_file(agent),
        force,
        "AGENTS.md",
    )?;
    let prompt_content = scaffold_file_content(
        &prompt_path,
        &agent_harness_prompt_file(agent),
        force,
        ".workflow-os/agent-harness-prompt.md",
    )?;

    if dry_run {
        println!("dry_run: true");
        println!("would_write: AGENTS.md");
        println!("would_write: .workflow-os/agent-harness-prompt.md");
        return Ok(());
    }

    write_scaffold_file(&agents_path, &agents_content)?;
    write_scaffold_file(&prompt_path, &prompt_content)?;
    println!("created_or_updated: AGENTS.md");
    println!("created_or_updated: .workflow-os/agent-harness-prompt.md");
    println!("mode: documentation scaffold only");
    println!("next_step: paste .workflow-os/agent-harness-prompt.md into your coding agent");
    Ok(())
}

fn scaffold_file_content(
    path: &Path,
    generated: &str,
    force: bool,
    label: &'static str,
) -> Result<String, WorkflowOsError> {
    if !path.exists() || force {
        return Ok(generated.to_owned());
    }
    let existing = fs::read_to_string(path).map_err(|_| {
        WorkflowOsError::new(
            WorkflowOsErrorKind::InvalidState,
            "cli.init_agent_harness.read_failed",
            "failed to read existing scaffold target",
        )
    })?;
    let generated_block = managed_block(generated)?;
    replace_managed_block(&existing, generated_block.as_str()).ok_or_else(|| {
        WorkflowOsError::new(
            WorkflowOsErrorKind::InvalidState,
            "cli.init_agent_harness.unmanaged_file",
            format!("{label} has unmanaged content; rerun with --force to replace it"),
        )
    })
}

fn write_scaffold_file(path: &Path, content: &str) -> Result<(), WorkflowOsError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|_| {
            WorkflowOsError::new(
                WorkflowOsErrorKind::InvalidState,
                "cli.init_agent_harness.create_dir_failed",
                "failed to create scaffold directory",
            )
        })?;
    }
    fs::write(path, content).map_err(|_| {
        WorkflowOsError::new(
            WorkflowOsErrorKind::InvalidState,
            "cli.init_agent_harness.write_failed",
            "failed to write scaffold file",
        )
    })
}

fn replace_managed_block(existing: &str, replacement_block: &str) -> Option<String> {
    let begin = existing.find(AGENT_HARNESS_BEGIN)?;
    let end_start = existing.find(AGENT_HARNESS_END)?;
    if end_start < begin {
        return None;
    }
    let end = end_start + AGENT_HARNESS_END.len();
    let mut output = String::new();
    output.push_str(&existing[..begin]);
    output.push_str(replacement_block);
    output.push_str(&existing[end..]);
    Some(output)
}

fn managed_block(content: &str) -> Result<String, WorkflowOsError> {
    let begin = content.find(AGENT_HARNESS_BEGIN).ok_or_else(|| {
        WorkflowOsError::new(
            WorkflowOsErrorKind::Internal,
            "cli.init_agent_harness.template_invalid",
            "generated scaffold template is invalid",
        )
    })?;
    let end_start = content.find(AGENT_HARNESS_END).ok_or_else(|| {
        WorkflowOsError::new(
            WorkflowOsErrorKind::Internal,
            "cli.init_agent_harness.template_invalid",
            "generated scaffold template is invalid",
        )
    })?;
    let end = end_start + AGENT_HARNESS_END.len();
    Ok(content[begin..end].to_owned())
}

fn agent_harness_agents_file(agent: AgentHarnessFlavor) -> String {
    format!(
        r"# Workflow OS Agent Instructions

{begin}
Agent executes. Workflow OS governs.

Audience: {audience}

Before changing this repository:
- Read `docs/ENGINEERING_STANDARD.md` and the current phase plan or review.
- Validate project state before implementation when the phase requires it.
- Start or resume the relevant governed workflow when required by the user or sprint plan.
- Treat approval checkpoints as mandatory governed boundaries.
- Stay inside the approved phase scope and call out deferred work explicitly.

Do not invent or hand-edit workflow state, approvals, evidence, audit events, work reports, validation results, or command outputs.

Unsupported in this scaffold:
- automatic workflow execution;
- automatic approval decisions;
- automatic local check execution or handler registration;
- report artifact generation;
- write-capable adapters;
- hosted or distributed execution;
- higher-autonomy operation.

Use `docs/user-guide/agent-harness-quickstart.md` for the current local adoption loop.
{end}
",
        begin = AGENT_HARNESS_BEGIN,
        end = AGENT_HARNESS_END,
        audience = agent.audience_label(),
    )
}

fn agent_harness_prompt_file(agent: AgentHarnessFlavor) -> String {
    format!(
        r"# Workflow OS Agent Harness Prompt

{begin}
Agent executes. Workflow OS governs.

Use Workflow OS as the governing layer for this repository.

Agent profile: {audience}

Before implementing:
- read the engineering standard and active phase documentation;
- validate the project when required;
- use the governed workflow as the source of truth for phase scope, approvals, checks, and reports.

While working:
- do not bypass validation, policy, approvals, or failed checks;
- do not mutate Workflow OS state files by hand;
- do not replace deterministic governance with model self-review;
- do not invent workflow state, approvals, evidence, audit events, work reports, validation results, or command outputs;
- do not claim unsupported runtime, write, hosted, or higher-autonomy capabilities.

When finished, report:
- scope completed;
- scope explicitly not completed;
- validation commands and results;
- remaining limitations;
- recommended next phase.
{end}
",
        begin = AGENT_HARNESS_BEGIN,
        end = AGENT_HARNESS_END,
        audience = agent.audience_label(),
    )
}

fn local_backend(invocation: &Invocation) -> Result<LocalStateBackend, WorkflowOsError> {
    LocalStateBackend::new(invocation.state_dir())
}

fn local_registry(invocation: &Invocation) -> Result<LocalSkillRegistry, WorkflowOsError> {
    if !invocation.mock_all_local_skills {
        return Ok(LocalSkillRegistry::new());
    }
    let project_dir = &invocation.project_dir;
    let load_result = load_project(project_dir);
    let Some(bundle) = load_result.bundle else {
        return Err(WorkflowOsError::validation(
            "cli.project.load_failed",
            "project could not be loaded for local skill registration",
        )
        .with_diagnostics(load_result.diagnostics));
    };
    let mut registry = LocalSkillRegistry::new();
    for skill in bundle.skills {
        if is_github_read_only_skill(&skill.definition) {
            registry.register(
                skill.definition.id,
                skill.definition.version,
                Box::new(CliGitHubReadOnlyFixtureHandler {
                    project_dir: project_dir.clone(),
                }),
            );
        } else if is_jira_read_only_skill(&skill.definition) {
            registry.register(
                skill.definition.id,
                skill.definition.version,
                Box::new(CliJiraReadOnlyFixtureHandler {
                    project_dir: project_dir.clone(),
                }),
            );
        } else if is_ci_read_only_skill(&skill.definition) {
            registry.register(
                skill.definition.id,
                skill.definition.version,
                Box::new(CliCiReadOnlyFixtureHandler {
                    project_dir: project_dir.clone(),
                }),
            );
        } else if skill.definition.adapter_requirements.is_empty()
            && skill.definition.id.as_str().starts_with("local/")
        {
            registry.register(
                skill.definition.id,
                skill.definition.version,
                Box::new(CliLocalSkillHandler),
            );
        }
    }
    Ok(registry)
}

fn is_github_read_only_skill(skill: &SkillDefinition) -> bool {
    skill.adapter_requirements.iter().any(|adapter| {
        adapter.adapter_id.as_str() == "symbolic/github-read-only"
            && adapter
                .capabilities
                .iter()
                .any(|capability| capability == "external.read")
    })
}

fn is_jira_read_only_skill(skill: &SkillDefinition) -> bool {
    skill.adapter_requirements.iter().any(|adapter| {
        adapter.adapter_id.as_str() == "symbolic/jira-read-only"
            && adapter
                .capabilities
                .iter()
                .any(|capability| capability == "external.read")
    })
}

fn is_ci_read_only_skill(skill: &SkillDefinition) -> bool {
    skill.adapter_requirements.iter().any(|adapter| {
        matches!(
            adapter.adapter_id.as_str(),
            "symbolic/ci-read-only" | "symbolic/github-actions-read-only"
        ) && adapter
            .capabilities
            .iter()
            .any(|capability| capability == "external.read")
    })
}

struct CliLocalSkillHandler;

impl SkillHandler for CliLocalSkillHandler {
    fn invoke(&self, input: SkillInput) -> Result<SkillOutput, WorkflowOsError> {
        let mut values = BTreeMap::new();
        let summary = input
            .values
            .get("request")
            .cloned()
            .or_else(|| input.values.values().next().cloned())
            .unwrap_or_else(|| "completed".to_owned());
        values.insert("summary".to_owned(), summary);
        if input.skill_id.as_str().starts_with("local/check-") {
            values.insert("local_check_status".to_owned(), "mocked".to_owned());
        }
        Ok(SkillOutput::new(
            values,
            Some(format!("mock-local-cli-output/{}", input.run_id)),
        ))
    }
}

struct CliGitHubReadOnlyFixtureHandler {
    project_dir: PathBuf,
}

impl SkillHandler for CliGitHubReadOnlyFixtureHandler {
    fn invoke(&self, input: SkillInput) -> Result<SkillOutput, WorkflowOsError> {
        let owner = required_input(&input, "owner", "GitHub")?;
        let repo = required_input(&input, "repo", "GitHub")?;
        let pull_number = required_input(&input, "pull_number", "GitHub")?;
        let reference = input
            .values
            .get("ref")
            .cloned()
            .unwrap_or_else(|| "main".to_owned());
        let fixture_root = self.project_dir.join("fixtures").join("github");
        let client = github_fixture_client(&fixture_root, &owner, &repo, &pull_number, &reference)?;
        let config = GitHubReadOnlyConfig::fixture().map_err(adapter_error)?;
        let adapter = GitHubReadOnlyAdapter::new(config, client);

        let mut metadata = BTreeMap::from([
            ("owner".to_owned(), owner.clone()),
            ("repo".to_owned(), repo.clone()),
            ("pull_number".to_owned(), pull_number.clone()),
            ("ref".to_owned(), reference),
        ]);
        let mut pr_request = github_read_request(
            github_actions::PULL_REQUEST_METADATA,
            ActorId::new("system/github-read-only-example")?,
            input.correlation_id.clone(),
            metadata.clone(),
            AdapterOperationMode::Fixture,
            fixture_policy_precheck("policy.fixture.github_read"),
        )
        .map_err(adapter_error)?;
        attach_run_scope(&mut pr_request, &input);
        let pr = adapter
            .read_pull_request_metadata(
                &pr_request,
                &owner,
                &repo,
                parse_pull_number(&pull_number)?,
            )
            .map_err(adapter_error)?;

        metadata.insert("action".to_owned(), "changed-files".to_owned());
        let mut files_request = github_read_request(
            github_actions::PULL_REQUEST_CHANGED_FILES,
            ActorId::new("system/github-read-only-example")?,
            input.correlation_id.clone(),
            metadata,
            AdapterOperationMode::Fixture,
            fixture_policy_precheck("policy.fixture.github_read"),
        )
        .map_err(adapter_error)?;
        attach_run_scope(&mut files_request, &input);
        let files = adapter
            .read_pull_request_changed_files(
                &files_request,
                &owner,
                &repo,
                parse_pull_number(&pull_number)?,
            )
            .map_err(adapter_error)?;

        let mut values = BTreeMap::new();
        values.insert(
            "summary".to_owned(),
            format!(
                "review-context: {}; {}; adapter_contract_telemetry_records=2",
                pr.response.summary, files.response.summary
            ),
        );
        Ok(SkillOutput::new(
            values,
            Some(format!(
                "github-read-only-fixture/{owner}/{repo}/pull/{pull_number}"
            )),
        )
        .with_adapter_telemetry(vec![
            AdapterTelemetryRecord::new(pr.invocation, pr.observability),
            AdapterTelemetryRecord::new(files.invocation, files.observability),
        ]))
    }
}

struct CliJiraReadOnlyFixtureHandler {
    project_dir: PathBuf,
}

impl SkillHandler for CliJiraReadOnlyFixtureHandler {
    fn invoke(&self, input: SkillInput) -> Result<SkillOutput, WorkflowOsError> {
        let issue_key = required_input(&input, "issue_key", "Jira")?;
        let fixture_root = self.project_dir.join("fixtures").join("jira");
        let client = jira_fixture_client(&fixture_root, &issue_key)?;
        let config = JiraReadOnlyConfig::fixture().map_err(adapter_error)?;
        let adapter = JiraReadOnlyAdapter::new(config, client);

        let metadata = BTreeMap::from([("issue_key".to_owned(), issue_key.clone())]);
        let mut issue_request = jira_read_request(
            jira_actions::ISSUE_METADATA,
            ActorId::new("system/jira-read-only-example")?,
            input.correlation_id.clone(),
            metadata.clone(),
            AdapterOperationMode::Fixture,
            fixture_policy_precheck("policy.fixture.jira_read"),
        )
        .map_err(adapter_error)?;
        attach_run_scope(&mut issue_request, &input);
        let issue = adapter
            .read_issue_metadata(&issue_request, &issue_key)
            .map_err(adapter_error)?;

        let mut description_request = jira_read_request(
            jira_actions::ISSUE_DESCRIPTION,
            ActorId::new("system/jira-read-only-example")?,
            input.correlation_id.clone(),
            metadata.clone(),
            AdapterOperationMode::Fixture,
            fixture_policy_precheck("policy.fixture.jira_read"),
        )
        .map_err(adapter_error)?;
        attach_run_scope(&mut description_request, &input);
        let description = adapter
            .read_issue_description(&description_request, &issue_key)
            .map_err(adapter_error)?;

        let mut comments_request = jira_read_request(
            jira_actions::ISSUE_COMMENTS,
            ActorId::new("system/jira-read-only-example")?,
            input.correlation_id.clone(),
            metadata,
            AdapterOperationMode::Fixture,
            fixture_policy_precheck("policy.fixture.jira_read"),
        )
        .map_err(adapter_error)?;
        attach_run_scope(&mut comments_request, &input);
        let comments = adapter
            .read_issue_comments(&comments_request, &issue_key)
            .map_err(adapter_error)?;

        let mut values = BTreeMap::new();
        values.insert(
            "summary".to_owned(),
            format!(
                "intake-quality: {}; {}; {}; adapter_contract_telemetry_records=3",
                issue.response.summary, description.response.summary, comments.response.summary
            ),
        );
        Ok(SkillOutput::new(
            values,
            Some(format!("jira-read-only-fixture/issue/{issue_key}")),
        )
        .with_adapter_telemetry(vec![
            AdapterTelemetryRecord::new(issue.invocation, issue.observability),
            AdapterTelemetryRecord::new(description.invocation, description.observability),
            AdapterTelemetryRecord::new(comments.invocation, comments.observability),
        ]))
    }
}

struct CliCiReadOnlyFixtureHandler {
    project_dir: PathBuf,
}

impl SkillHandler for CliCiReadOnlyFixtureHandler {
    fn invoke(&self, input: SkillInput) -> Result<SkillOutput, WorkflowOsError> {
        let owner = required_input(&input, "owner", "CI")?;
        let repo = required_input(&input, "repo", "CI")?;
        let run_id = required_input(&input, "ci_run_id", "CI")?;
        let job_id = required_input(&input, "job_id", "CI")?;
        let reference = input
            .values
            .get("ref")
            .cloned()
            .unwrap_or_else(|| "main".to_owned());
        let fixture_root = self.project_dir.join("fixtures").join("github-actions");
        let client = ci_fixture_client(&fixture_root, &owner, &repo, &run_id, &job_id, &reference)?;
        let config = GitHubActionsReadOnlyConfig::fixture().map_err(adapter_error)?;
        let adapter = GitHubActionsReadOnlyAdapter::new(config, client);

        let metadata = BTreeMap::from([
            ("owner".to_owned(), owner.clone()),
            ("repo".to_owned(), repo.clone()),
            ("run_id".to_owned(), run_id.clone()),
            ("job_id".to_owned(), job_id.clone()),
            ("ref".to_owned(), reference),
        ]);
        let run_request =
            ci_fixture_request(ci_actions::WORKFLOW_RUN_METADATA, &input, metadata.clone())?;
        let run = adapter
            .read_workflow_run_metadata(&run_request, &owner, &repo, &run_id)
            .map_err(adapter_error)?;

        let jobs_request =
            ci_fixture_request(ci_actions::JOB_STATUS_SUMMARY, &input, metadata.clone())?;
        let jobs = adapter
            .read_workflow_jobs(&jobs_request, &owner, &repo, &run_id)
            .map_err(adapter_error)?;

        let failure_request =
            ci_fixture_request(ci_actions::FAILURE_SUMMARY, &input, metadata.clone())?;
        let failure = adapter
            .read_failure_summary(&failure_request, &owner, &repo, &run_id)
            .map_err(adapter_error)?;

        let log_request = ci_fixture_request(ci_actions::LOG_REFERENCE, &input, metadata.clone())?;
        let log_reference = adapter
            .read_log_reference(&log_request, &owner, &repo, &run_id)
            .map_err(adapter_error)?;

        let excerpt_request = ci_fixture_request(ci_actions::LOG_EXCERPT, &input, metadata)?;
        let excerpt = adapter
            .read_log_excerpt(&excerpt_request, &owner, &repo, &job_id)
            .map_err(adapter_error)?;

        let mut values = BTreeMap::new();
        values.insert(
            "summary".to_owned(),
            format!(
                "ci-failure-diagnosis: {}; {}; {}; {}; {}; escalation_recommendation=manual_review_if_ambiguous; adapter_contract_telemetry_records=5",
                run.response.summary,
                jobs.response.summary,
                failure.response.summary,
                log_reference.response.summary,
                excerpt.response.summary
            ),
        );
        Ok(SkillOutput::new(
            values,
            Some(format!(
                "github-actions-read-only-fixture/{owner}/{repo}/run/{run_id}"
            )),
        )
        .with_adapter_telemetry(vec![
            AdapterTelemetryRecord::new(run.invocation, run.observability),
            AdapterTelemetryRecord::new(jobs.invocation, jobs.observability),
            AdapterTelemetryRecord::new(failure.invocation, failure.observability),
            AdapterTelemetryRecord::new(log_reference.invocation, log_reference.observability),
            AdapterTelemetryRecord::new(excerpt.invocation, excerpt.observability),
        ]))
    }
}

fn ci_fixture_request(
    action: &'static str,
    input: &SkillInput,
    metadata: BTreeMap<String, String>,
) -> Result<workflow_core::AdapterRequest, WorkflowOsError> {
    let mut request = github_actions_read_request(
        action,
        ActorId::new("system/ci-read-only-example")?,
        input.correlation_id.clone(),
        metadata,
        AdapterOperationMode::Fixture,
        fixture_policy_precheck("policy.fixture.ci_read"),
    )
    .map_err(adapter_error)?;
    attach_run_scope(&mut request, input);
    Ok(request)
}

fn attach_run_scope(request: &mut workflow_core::AdapterRequest, input: &SkillInput) {
    request.run_scope = Some(AdapterRunScope {
        workflow_run_id: input.run_id.clone(),
        workflow_id: input.workflow_id.clone(),
        workflow_version: input.workflow_version.clone(),
        schema_version: input.schema_version.clone(),
        spec_hash: input.spec_hash.clone(),
    });
    request.input_reference = Some(format!(
        "adapter-fixture-input/{}/{}/{}",
        input.run_id, input.step_id, input.skill_id
    ));
}

fn github_fixture_client(
    fixture_root: &Path,
    owner: &str,
    repo: &str,
    pull_number: &str,
    reference: &str,
) -> Result<GitHubFixtureClient, WorkflowOsError> {
    let repo_json = read_fixture(fixture_root, "repository.json")?;
    let pr_json = read_fixture(fixture_root, &format!("pull-{pull_number}.json"))?;
    let files_json = read_fixture(fixture_root, &format!("pull-{pull_number}-files.json"))?;
    let comments_json = read_fixture(fixture_root, &format!("pull-{pull_number}-comments.json"))
        .unwrap_or_else(|_| "[]".to_owned());
    let checks_json = read_fixture(fixture_root, &format!("checks-{reference}.json"))
        .unwrap_or_else(|_| r#"{"check_runs":[]}"#.to_owned());
    let diff = read_fixture(fixture_root, &format!("pull-{pull_number}.diff"))
        .unwrap_or_else(|_| String::new());

    Ok(GitHubFixtureClient::new()
        .with_json(format!("/repos/{owner}/{repo}"), repo_json)
        .with_json(
            format!("/repos/{owner}/{repo}/pulls/{pull_number}"),
            pr_json,
        )
        .with_json(
            format!("/repos/{owner}/{repo}/pulls/{pull_number}/files"),
            files_json,
        )
        .with_json(
            format!("/repos/{owner}/{repo}/issues/{pull_number}/comments"),
            comments_json,
        )
        .with_json(
            format!("/repos/{owner}/{repo}/commits/{reference}/check-runs"),
            checks_json,
        )
        .with_json(
            format!("/repos/{owner}/{repo}/pulls/{pull_number}.diff"),
            diff,
        ))
}

fn jira_fixture_client(
    fixture_root: &Path,
    issue_key: &str,
) -> Result<JiraFixtureClient, WorkflowOsError> {
    let issue_json = read_jira_fixture(fixture_root, &format!("issue-{issue_key}.json"))?;
    let comments_json =
        read_jira_fixture(fixture_root, &format!("issue-{issue_key}-comments.json"))
            .unwrap_or_else(|_| r#"{"comments":[]}"#.to_owned());

    Ok(JiraFixtureClient::new()
        .with_json(format!("/rest/api/3/issue/{issue_key}"), issue_json)
        .with_json(
            format!("/rest/api/3/issue/{issue_key}/comment"),
            comments_json,
        ))
}

fn ci_fixture_client(
    fixture_root: &Path,
    owner: &str,
    repo: &str,
    run_id: &str,
    job_id: &str,
    reference: &str,
) -> Result<GitHubActionsFixtureClient, WorkflowOsError> {
    let run_json = read_ci_fixture(fixture_root, &format!("run-{run_id}.json"))?;
    let jobs_json = read_ci_fixture(fixture_root, &format!("run-{run_id}-jobs.json"))?;
    let checks_json = read_ci_fixture(fixture_root, &format!("checks-{reference}.json"))
        .unwrap_or_else(|_| r#"{"check_runs":[]}"#.to_owned());
    let logs = read_ci_fixture(fixture_root, &format!("job-{job_id}.log.fixture"))?;

    Ok(GitHubActionsFixtureClient::new()
        .with_json(
            format!("/repos/{owner}/{repo}/actions/runs/{run_id}"),
            run_json,
        )
        .with_json(
            format!("/repos/{owner}/{repo}/actions/runs/{run_id}/jobs"),
            jobs_json,
        )
        .with_json(
            format!("/repos/{owner}/{repo}/commits/{reference}/check-runs"),
            checks_json,
        )
        .with_text(
            format!("/repos/{owner}/{repo}/actions/jobs/{job_id}/logs"),
            logs,
        ))
}

fn read_fixture(fixture_root: &Path, file_name: &str) -> Result<String, WorkflowOsError> {
    let path = fixture_root.join(file_name);
    fs::read_to_string(&path).map_err(|error| {
        WorkflowOsError::new(
            WorkflowOsErrorKind::InvalidState,
            "github.fixture.missing",
            format!(
                "GitHub read-only fixture is missing or unreadable: {} ({error})",
                path.display()
            ),
        )
    })
}

fn read_jira_fixture(fixture_root: &Path, file_name: &str) -> Result<String, WorkflowOsError> {
    let path = fixture_root.join(file_name);
    fs::read_to_string(&path).map_err(|error| {
        WorkflowOsError::new(
            WorkflowOsErrorKind::InvalidState,
            "jira.fixture.missing",
            format!(
                "Jira read-only fixture is missing or unreadable: {} ({error})",
                path.display()
            ),
        )
    })
}

fn read_ci_fixture(fixture_root: &Path, file_name: &str) -> Result<String, WorkflowOsError> {
    let path = fixture_root.join(file_name);
    fs::read_to_string(&path).map_err(|error| {
        WorkflowOsError::new(
            WorkflowOsErrorKind::InvalidState,
            "ci.fixture.missing",
            format!(
                "CI read-only fixture is missing or unreadable: {} ({error})",
                path.display()
            ),
        )
    })
}

fn required_input(
    input: &SkillInput,
    key: &str,
    boundary: &str,
) -> Result<String, WorkflowOsError> {
    input.values.get(key).cloned().ok_or_else(|| {
        WorkflowOsError::validation(
            format!("{}.input.{key}.required", boundary.to_ascii_lowercase()),
            format!("{boundary} read-only example requires input value {key}"),
        )
    })
}

fn parse_pull_number(value: &str) -> Result<u64, WorkflowOsError> {
    value.parse::<u64>().map_err(|error| {
        WorkflowOsError::validation(
            "github.input.pull_number.invalid",
            format!("pull_number must be an unsigned integer ({error})"),
        )
    })
}

fn adapter_error(error: workflow_core::AdapterError) -> WorkflowOsError {
    WorkflowOsError::new(WorkflowOsErrorKind::InvalidState, error.code, error.message)
}

fn fixture_policy_precheck(reason_code: &str) -> AdapterPolicyPrecheck {
    AdapterPolicyPrecheck::fixture_test_allowed(vec![reason_code.to_owned()])
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Invocation {
    project_dir: PathBuf,
    state_dir: Option<PathBuf>,
    mock_all_local_skills: bool,
    json: bool,
    command: Command,
}

impl Invocation {
    fn parse(args: &[String]) -> Result<Self, WorkflowOsError> {
        let mut project_dir = env::current_dir().map_err(|error| {
            WorkflowOsError::new(
                WorkflowOsErrorKind::Internal,
                "cli.cwd",
                format!("failed to read current directory: {error}"),
            )
        })?;
        let mut state_dir = None;
        let mut mock_all_local_skills = false;
        let mut json = false;
        let mut positional = Vec::new();
        let mut index = 0;
        while index < args.len() {
            match args[index].as_str() {
                "--json" => json = true,
                "--mock-all-local-skills" => mock_all_local_skills = true,
                "--project-dir" | "-p" => {
                    index += 1;
                    project_dir = PathBuf::from(args.get(index).ok_or_else(missing_value)?);
                }
                "--state-dir" => {
                    index += 1;
                    state_dir = Some(PathBuf::from(args.get(index).ok_or_else(missing_value)?));
                }
                "--help" | "-h" => positional.push("help".to_owned()),
                value => positional.push(value.to_owned()),
            }
            index += 1;
        }
        let command = parse_command(&positional)?;
        Ok(Self {
            project_dir,
            state_dir,
            mock_all_local_skills,
            json,
            command,
        })
    }

    fn state_dir(&self) -> PathBuf {
        self.state_dir
            .clone()
            .unwrap_or_else(|| self.project_dir.join(".workflow-os").join("state"))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Command {
    Validate,
    Run {
        workflow_id: String,
        run_id: Option<String>,
    },
    Status {
        run_id: String,
    },
    Approve {
        run_id: String,
        approval_id: String,
        actor: Option<String>,
        reason: Option<String>,
        deny: bool,
    },
    Inspect {
        run_id: String,
    },
    Doctor,
    DoctorState,
    InitAgentHarness {
        output_dir: Option<PathBuf>,
        agent: AgentHarnessFlavor,
        force: bool,
        dry_run: bool,
    },
    Help,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AgentHarnessFlavor {
    Generic,
    Codex,
    Claude,
}

impl AgentHarnessFlavor {
    fn parse(value: &str) -> Result<Self, WorkflowOsError> {
        match value {
            "generic" => Ok(Self::Generic),
            "codex" => Ok(Self::Codex),
            "claude" => Ok(Self::Claude),
            _ => Err(usage("agent must be one of: generic, codex, claude")),
        }
    }

    fn audience_label(self) -> &'static str {
        match self {
            Self::Generic => "generic coding agent",
            Self::Codex => "Codex",
            Self::Claude => "Claude Code",
        }
    }
}

fn parse_command(args: &[String]) -> Result<Command, WorkflowOsError> {
    let Some(command) = args.first().map(String::as_str) else {
        return Ok(Command::Help);
    };
    match command {
        "help" => Ok(Command::Help),
        "validate" => Ok(Command::Validate),
        "doctor" => match args.get(1).map(String::as_str) {
            None => Ok(Command::Doctor),
            Some("state") => Ok(Command::DoctorState),
            Some(other) => Err(usage(format!("unknown doctor subcommand {other}"))),
        },
        "init-agent-harness" => Ok(Command::InitAgentHarness {
            output_dir: flag_value(args, "--output-dir").map(PathBuf::from),
            agent: flag_value(args, "--agent")
                .as_deref()
                .map(AgentHarnessFlavor::parse)
                .transpose()?
                .unwrap_or(AgentHarnessFlavor::Generic),
            force: flag_present(args, "--force"),
            dry_run: flag_present(args, "--dry-run"),
        }),
        "run" => {
            let workflow_id = args
                .get(1)
                .ok_or_else(|| usage("run requires <workflow-id>"))?;
            let run_id = flag_value(args, "--run-id");
            Ok(Command::Run {
                workflow_id: workflow_id.clone(),
                run_id,
            })
        }
        "status" => Ok(Command::Status {
            run_id: args
                .get(1)
                .ok_or_else(|| usage("status requires <run-id>"))?
                .clone(),
        }),
        "approve" => Ok(Command::Approve {
            run_id: args
                .get(1)
                .ok_or_else(|| usage("approve requires <run-id> <approval-id>"))?
                .clone(),
            approval_id: args
                .get(2)
                .ok_or_else(|| usage("approve requires <run-id> <approval-id>"))?
                .clone(),
            actor: flag_value(args, "--actor"),
            reason: flag_value(args, "--reason"),
            deny: flag_present(args, "--deny"),
        }),
        "inspect" => Ok(Command::Inspect {
            run_id: args
                .get(1)
                .ok_or_else(|| usage("inspect requires <run-id>"))?
                .clone(),
        }),
        other => Err(usage(format!("unknown command {other}"))),
    }
}

fn flag_value(args: &[String], flag: &str) -> Option<String> {
    args.windows(2)
        .find(|window| window[0] == flag)
        .map(|window| window[1].clone())
}

fn flag_present(args: &[String], flag: &str) -> bool {
    args.iter().any(|arg| arg == flag)
}

fn print_help() {
    println!("Workflow OS CLI");
    println!();
    println!(
        "Usage: workflow-os [--project-dir <path>] [--state-dir <path>] [--json] [--mock-all-local-skills] <command>"
    );
    println!();
    println!("Global options:");
    println!("  --json                  emit experimental preview JSON where implemented");
    println!("  --mock-all-local-skills  register deterministic mock handlers for local/* skills");
    println!();
    println!("Commands:");
    println!("  validate");
    println!("  run <workflow-id> [--run-id <run-id>]");
    println!("  status <run-id>");
    println!("  approve <run-id> <approval-id> [--deny] [--actor <actor>] [--reason <reason>]");
    println!("  inspect <run-id>");
    println!("  doctor");
    println!("  doctor state");
    println!("  init-agent-harness [--output-dir <path>] [--agent generic|codex|claude] [--force] [--dry-run]");
    println!("      documentation scaffold only; does not run workflows or approve checkpoints");
}

fn print_approval_summary(
    invocation: &Invocation,
    run: &WorkflowRun,
    decision: ApprovalDecisionKind,
) {
    if invocation.json {
        println!("{}", approval_result_json(run, decision));
    } else {
        println!("decision: {}", approval_decision_label(decision));
        print_run_summary(invocation, run);
    }
}

fn print_run_summary(invocation: &Invocation, run: &WorkflowRun) {
    if invocation.json {
        println!("{}", run_status_json(run));
    } else {
        println!("run_id: {}", run.snapshot.identity.run_id);
        println!("schema_version: {}", run.snapshot.identity.schema_version);
        println!("status: {:?}", run.snapshot.status);
        if let Some(approval) = run.snapshot.approval_requests.last() {
            if run.snapshot.status == WorkflowRunStatus::WaitingForApproval {
                println!("approval_id: {}", approval.approval_id);
            }
        }
    }
}

fn print_diagnostics_text(diagnostics: &[Diagnostic]) {
    for diagnostic in diagnostics {
        println!("{diagnostic}");
    }
}

fn print_diagnostics_json(diagnostics: &[Diagnostic]) {
    println!("{}", diagnostics_json(diagnostics));
}

fn current_step(run: &WorkflowRun) -> Option<String> {
    run.snapshot
        .approval_requests
        .last()
        .map(|approval| approval.step_id.to_string())
        .or_else(|| {
            run.snapshot
                .skill_invocations
                .last()
                .map(|invocation| invocation.step_id.to_string())
        })
}

fn format_event(kind: &WorkflowRunEventKind, name: WorkflowRunEventKindName) -> String {
    match kind {
        WorkflowRunEventKind::SkillInvocationSucceeded { output_ref, .. } => {
            format!(
                "SkillInvocationSucceeded output_ref={}",
                redact_option(output_ref.as_ref())
            )
        }
        _ => format!("{name:?}"),
    }
}

fn redact_option(value: Option<&String>) -> String {
    value.map_or_else(
        || "none".to_owned(),
        |value| {
            let lower = value.to_ascii_lowercase();
            if lower.contains("secret") || lower.contains("token") || lower.contains("password") {
                "[REDACTED]".to_owned()
            } else {
                value.clone()
            }
        },
    )
}

fn run_status_json(run: &WorkflowRun) -> String {
    format!(
        "{{\"run_id\":\"{}\",\"workflow_id\":\"{}\",\"schema_version\":\"{}\",\"status\":\"{:?}\",\"current_step\":{},\"terminal\":{}}}",
        json_escape(run.snapshot.identity.run_id.as_str()),
        json_escape(run.snapshot.identity.workflow_id.as_str()),
        json_escape(run.snapshot.identity.schema_version.as_str()),
        run.snapshot.status,
        json_string_option(current_step(run).as_deref()),
        run.snapshot.status.is_terminal()
    )
}

fn approval_result_json(run: &WorkflowRun, decision: ApprovalDecisionKind) -> String {
    format!(
        "{{\"decision\":\"{}\",\"run_id\":\"{}\",\"workflow_id\":\"{}\",\"schema_version\":\"{}\",\"status\":\"{:?}\",\"current_step\":{},\"terminal\":{}}}",
        approval_decision_label(decision),
        json_escape(run.snapshot.identity.run_id.as_str()),
        json_escape(run.snapshot.identity.workflow_id.as_str()),
        json_escape(run.snapshot.identity.schema_version.as_str()),
        run.snapshot.status,
        json_string_option(current_step(run).as_deref()),
        run.snapshot.status.is_terminal()
    )
}

fn approval_decision_label(decision: ApprovalDecisionKind) -> &'static str {
    match decision {
        ApprovalDecisionKind::Granted => "granted",
        ApprovalDecisionKind::Denied => "denied",
    }
}

fn inspect_json(
    run: &WorkflowRun,
    adapter_audit: &[workflow_core::AdapterRuntimeAuditRecord],
    adapter_observability: &[workflow_core::AdapterRuntimeObservabilityRecord],
) -> String {
    let events = run
        .events
        .iter()
        .map(|event| {
            format!(
                "{{\"sequence\":{},\"event_id\":\"{}\",\"schema_version\":\"{}\",\"kind\":\"{:?}\"}}",
                event.sequence_number.get(),
                json_escape(event.event_id.as_str()),
                json_escape(event.schema_version.as_str()),
                event.kind()
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"run_id\":\"{}\",\"schema_version\":\"{}\",\"workflow_version\":\"{}\",\"spec_hash\":\"{}\",\"status\":\"{:?}\",\"events\":[{}],\"approvals\":{},\"retries\":{},\"escalations\":{},\"adapter_audit_records\":{},\"adapter_observability_records\":{}}}",
        json_escape(run.snapshot.identity.run_id.as_str()),
        json_escape(run.snapshot.identity.schema_version.as_str()),
        json_escape(run.snapshot.identity.workflow_version.as_str()),
        json_escape(run.snapshot.identity.spec_content_hash.as_str()),
        run.snapshot.status,
        events,
        run.snapshot.approval_requests.len(),
        run.snapshot.retries.len(),
        run.snapshot.escalations.len(),
        adapter_audit.len(),
        adapter_observability.len()
    )
}

fn diagnostics_json(diagnostics: &[Diagnostic]) -> String {
    let values = diagnostics
        .iter()
        .map(|diagnostic| {
            let source = diagnostic.source_location().map_or_else(
                || "null".to_owned(),
                |source| {
                    format!(
                        "{{\"file\":\"{}\",\"line\":{},\"column\":{},\"path\":{}}}",
                        json_escape(&source.file_path().display().to_string()),
                        option_u32(source.line()),
                        option_u32(source.column()),
                        json_string_option(source.document_path())
                    )
                },
            );
            format!(
                "{{\"severity\":\"{}\",\"code\":\"{}\",\"message\":\"{}\",\"source\":{}}}",
                diagnostic.severity(),
                json_escape(diagnostic.code()),
                json_escape(diagnostic.message()),
                source
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!("[{values}]")
}

fn backend_health_json(health: &BackendHealthCheck) -> String {
    format!(
        "{{\"healthy\":{},\"backend\":\"{}\",\"message\":\"{}\"}}",
        health.healthy,
        json_escape(&health.backend),
        json_escape(&health.message)
    )
}

fn state_inspection_json(inspection: &LocalStateInspection) -> String {
    let issues = inspection
        .issues
        .iter()
        .map(state_issue_json)
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"healthy\":{},\"backend\":\"{}\",\"root\":\"{}\",\"issues\":[{}]}}",
        inspection.healthy,
        json_escape(&inspection.backend),
        json_escape(&inspection.root.display().to_string()),
        issues
    )
}

fn state_issue_json(issue: &LocalStateIssue) -> String {
    format!(
        "{{\"severity\":\"{}\",\"code\":\"{}\",\"message\":\"{}\",\"path\":{},\"run_id\":{},\"sequence_number\":{},\"event_id\":{}}}",
        issue.severity.as_str(),
        json_escape(&issue.code),
        json_escape(&issue.message),
        json_string_option(issue.path.as_ref().map(|path| path.display().to_string()).as_deref()),
        json_string_option(
            issue
                .run_id
                .as_ref()
                .map(std::string::ToString::to_string)
                .as_deref(),
        ),
        issue
            .sequence_number
            .map_or_else(|| "null".to_owned(), |sequence| sequence.get().to_string()),
        json_string_option(
            issue
                .event_id
                .as_ref()
                .map(std::string::ToString::to_string)
                .as_deref(),
        )
    )
}

fn format_state_issue(issue: &LocalStateIssue) -> String {
    let mut output = format!(
        "{}[{}]: {}",
        state_issue_severity_label(issue.severity),
        issue.code,
        issue.message
    );
    if let Some(path) = &issue.path {
        let _ = write!(output, " path={}", path.display());
    }
    if let Some(run_id) = &issue.run_id {
        let _ = write!(output, " run_id={run_id}");
    }
    if let Some(sequence) = issue.sequence_number {
        let _ = write!(output, " sequence={}", sequence.get());
    }
    if let Some(event_id) = &issue.event_id {
        let _ = write!(output, " event_id={event_id}");
    }
    output
}

fn state_issue_severity_label(severity: LocalStateIssueSeverity) -> &'static str {
    match severity {
        LocalStateIssueSeverity::Error => "error",
        LocalStateIssueSeverity::Warning => "warning",
    }
}

fn json_string_option(value: Option<&str>) -> String {
    value.map_or_else(
        || "null".to_owned(),
        |value| format!("\"{}\"", json_escape(value)),
    )
}

fn option_u32(value: Option<u32>) -> String {
    value.map_or_else(|| "null".to_owned(), |value| value.to_string())
}

fn json_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

fn status_word(ok: bool) -> &'static str {
    if ok {
        "ok"
    } else {
        "failed"
    }
}

fn exit_code_for_error(error: &WorkflowOsError) -> i32 {
    match error.kind() {
        WorkflowOsErrorKind::Validation | WorkflowOsErrorKind::Parse => EXIT_VALIDATION,
        WorkflowOsErrorKind::Unsupported => EXIT_USAGE,
        WorkflowOsErrorKind::PolicyDenied
        | WorkflowOsErrorKind::InvalidState
        | WorkflowOsErrorKind::Security
        | WorkflowOsErrorKind::Internal => EXIT_RUNTIME,
    }
}

fn missing_value() -> WorkflowOsError {
    usage("flag requires a value")
}

fn usage(message: impl Into<String>) -> WorkflowOsError {
    WorkflowOsError::new(WorkflowOsErrorKind::Unsupported, "cli.usage", message)
}
