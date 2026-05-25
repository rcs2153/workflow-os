#![deny(unsafe_code)]
#![doc = "Command-line interface for Workflow OS v0."]

use std::collections::BTreeMap;
use std::env;
use std::fmt::Write as _;
use std::path::PathBuf;

use workflow_core::{
    load_project, validate_loaded_project, ActorId, ApprovalDecisionKind, BackendHealthCheck,
    CorrelationId, Diagnostic, LocalApprovalDecisionRequest, LocalExecutionRequest, LocalExecutor,
    LocalSkillRegistry, LocalStateBackend, LocalStateInspection, LocalStateIssue,
    LocalStateIssueSeverity, SkillHandler, SkillInput, SkillOutput, StateBackend, WorkflowId,
    WorkflowOsError, WorkflowOsErrorKind, WorkflowRun, WorkflowRunEventKind,
    WorkflowRunEventKindName, WorkflowRunId, WorkflowRunStatus,
};

const EXIT_OK: i32 = 0;
const EXIT_VALIDATION: i32 = 1;
const EXIT_USAGE: i32 = 2;
const EXIT_RUNTIME: i32 = 3;

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
    if invocation.json {
        println!("{}", inspect_json(&run));
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
        if skill.definition.adapter_requirements.is_empty()
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
        Ok(SkillOutput::new(
            values,
            Some(format!("mock-local-cli-output/{}", input.run_id)),
        ))
    }
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
    Help,
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

fn inspect_json(run: &WorkflowRun) -> String {
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
        "{{\"run_id\":\"{}\",\"schema_version\":\"{}\",\"workflow_version\":\"{}\",\"spec_hash\":\"{}\",\"status\":\"{:?}\",\"events\":[{}],\"approvals\":{},\"retries\":{},\"escalations\":{}}}",
        json_escape(run.snapshot.identity.run_id.as_str()),
        json_escape(run.snapshot.identity.schema_version.as_str()),
        json_escape(run.snapshot.identity.workflow_version.as_str()),
        json_escape(run.snapshot.identity.spec_content_hash.as_str()),
        run.snapshot.status,
        events,
        run.snapshot.approval_requests.len(),
        run.snapshot.retries.len(),
        run.snapshot.escalations.len()
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
