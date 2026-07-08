#![deny(unsafe_code)]
#![doc = "Command-line interface for Workflow OS v0."]

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Component, Path, PathBuf};

use workflow_core::{
    canonical_yaml_content_hash, ci_actions, github_actions, github_actions_read_request,
    github_read_request, jira_actions, jira_read_request, load_project, parse_workflow_spec_yaml,
    review_workflow_draft_for_promotion, validate_loaded_project, validate_project_bundle, ActorId,
    AdapterOperationMode, AdapterPolicyPrecheck, AdapterRunScope, AdapterTelemetryRecord,
    AdapterTelemetryStore, ApprovalDecisionKind, BackendHealthCheck, CorrelationId, Diagnostic,
    DiagnosticSeverity, GitHubActionsFixtureClient, GitHubActionsReadOnlyAdapter,
    GitHubActionsReadOnlyConfig, GitHubFixtureClient, GitHubReadOnlyAdapter, GitHubReadOnlyConfig,
    JiraFixtureClient, JiraReadOnlyAdapter, JiraReadOnlyConfig, LifecycleStatus, LoadedSpec,
    LocalApprovalDecisionRequest, LocalExecutionBeforeSkillInvocationCheckpointInputs,
    LocalExecutionRequest, LocalExecutor, LocalSkillRegistry, LocalStateBackend,
    LocalStateInspection, LocalStateIssue, LocalStateIssueSeverity, SkillDefinition, SkillHandler,
    SkillInput, SkillOutput, StateBackend, WorkReportArtifactHighAssuranceRequirement,
    WorkReportHandoffNote, WorkReportIncompleteWorkDisclosure, WorkReportKnownLimitation,
    WorkReportRisk, WorkReportSection, WorkReportSectionKind, WorkflowDefinition,
    WorkflowDraftPromotionPreflightStatus, WorkflowDraftStewardReviewAuthorization,
    WorkflowDraftStewardReviewDecision, WorkflowDraftStewardReviewInput,
    WorkflowDraftStewardReviewResult, WorkflowId, WorkflowOsError, WorkflowOsErrorKind,
    WorkflowRun, WorkflowRunEventKind, WorkflowRunEventKindName, WorkflowRunId, WorkflowRunStatus,
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
        Command::InitRepoGovernance {
            output_dir,
            agent,
            force,
            dry_run,
        } => init_repo_governance_command(
            &invocation,
            output_dir.as_deref(),
            *agent,
            *force,
            *dry_run,
        ),
        Command::FirstRun {
            verbose,
            recommendation,
        } => first_run_command(&invocation, *verbose, recommendation.as_deref()),
        Command::AuthorWorkflow {
            from_recommendation,
            dry_run,
            output,
        } => author_workflow_command(
            &invocation,
            from_recommendation.as_deref(),
            *dry_run,
            output.as_deref(),
        ),
        Command::AuthorWorkflowPreflight { draft } => {
            author_workflow_preflight_command(&invocation, draft)
        }
        Command::AuthorWorkflowDraftStatus { draft } => {
            author_workflow_draft_status_command(&invocation, draft)
        }
        Command::AuthorWorkflowArchiveDraft {
            draft,
            reviewer,
            reason,
            dry_run,
        } => author_workflow_archive_draft_command(&invocation, draft, reviewer, reason, *dry_run),
        Command::AuthorWorkflowStewardReview {
            draft,
            decision,
            reviewer,
            reason,
        } => {
            author_workflow_steward_review_command(&invocation, draft, *decision, reviewer, reason)
        }
        Command::AuthorWorkflowPromote {
            draft,
            reviewer,
            reason,
            dry_run,
        } => author_workflow_promote_command(&invocation, draft, reviewer, reason, *dry_run),
        Command::Help => {
            print_help();
            Ok(())
        }
    }
}

fn validate_command(invocation: &Invocation) -> Result<(), WorkflowOsError> {
    let load_result = load_project(&invocation.project_dir);
    let validation = validate_loaded_project(&load_result);
    let manifest_missing = validation
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code() == "loader.manifest_missing");
    if invocation.json {
        print_diagnostics_json(&validation.diagnostics);
    } else {
        print_diagnostics_text(&validation.diagnostics);
        if manifest_missing {
            println!("next_step: workflow-os init-repo-governance");
        }
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
        before_skill_invocation_checkpoints:
            LocalExecutionBeforeSkillInvocationCheckpointInputs::default(),
        before_skill_invocation_hook: None,
        side_effect_events: Vec::new(),
        side_effect_lifecycle_events: Vec::new(),
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
        println!("schemas: {}", schema_status_word(schemas_exist));
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
        print_agent_harness_preservation_notice(&agents_path, force, true);
        return Ok(());
    }

    print_agent_harness_preservation_notice(&agents_path, force, false);
    write_scaffold_file(&agents_path, &agents_content)?;
    write_scaffold_file(&prompt_path, &prompt_content)?;
    println!("created_or_updated: AGENTS.md");
    println!("created_or_updated: .workflow-os/agent-harness-prompt.md");
    println!("mode: documentation scaffold only");
    println!("next_step: paste .workflow-os/agent-harness-prompt.md into your coding agent");
    Ok(())
}

fn init_repo_governance_command(
    invocation: &Invocation,
    output_dir: Option<&Path>,
    agent: AgentHarnessFlavor,
    force: bool,
    dry_run: bool,
) -> Result<(), WorkflowOsError> {
    let root = output_dir.map_or_else(|| invocation.project_dir.clone(), Path::to_path_buf);
    let files = repo_governance_scaffold_files(agent);
    let mut planned = Vec::new();
    for (relative_path, content, kind) in &files {
        let path = root.join(relative_path);
        let content = match kind {
            ScaffoldKind::Plain => {
                plain_scaffold_file_content(&path, content, force, relative_path)?
            }
            ScaffoldKind::ManagedBlock => {
                scaffold_file_content(&path, content, force, relative_path)?
            }
        };
        planned.push((path, *relative_path, content));
    }

    if dry_run {
        println!("dry_run: true");
        for (path, relative_path, _) in &planned {
            println!("would_write: {relative_path}");
            if *relative_path == "AGENTS.md" {
                print_agent_harness_preservation_notice(path, force, true);
            }
        }
        println!("mode: existing repo governance scaffold only");
        return Ok(());
    }

    for (path, relative_path, content) in planned {
        if relative_path == "AGENTS.md" {
            print_agent_harness_preservation_notice(&path, force, false);
        }
        write_scaffold_file(&path, &content)?;
        println!("created_or_updated: {relative_path}");
    }
    println!("mode: existing repo governance scaffold only");
    println!("next_step: workflow-os validate");
    println!("next_step: workflow-os first-run");
    println!("next_step: workflow-os --mock-all-local-skills run local/first-run-governance");
    Ok(())
}

fn first_run_command(
    invocation: &Invocation,
    verbose: bool,
    recommendation_id: Option<&str>,
) -> Result<(), WorkflowOsError> {
    let load_result = load_project(&invocation.project_dir);
    let validation = validate_loaded_project(&load_result);
    if load_result.bundle.is_none()
        && validation
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code() == "loader.manifest_missing")
    {
        return Err(WorkflowOsError::validation(
            "cli.first_run.manifest_missing",
            "no Workflow OS project was found; run `workflow-os init-repo-governance` first",
        ));
    }
    if validation.has_errors() {
        return Err(WorkflowOsError::validation(
            "cli.first_run.validation_failed",
            "project validation failed; run `workflow-os validate` for diagnostics",
        ));
    }
    let bundle = load_result.bundle.as_ref().ok_or_else(|| {
        WorkflowOsError::validation(
            "cli.first_run.project_unavailable",
            "first-run requires a loaded Workflow OS project",
        )
    })?;
    let context = FirstRunReportReadyContext::new(invocation, bundle)?;
    if let Some(recommendation_id) = recommendation_id {
        let recommendation = context
            .workflow_discovery_recommendation(recommendation_id)
            .ok_or_else(|| {
                WorkflowOsError::validation(
                    "cli.first_run.recommendation_not_found",
                    "requested first-run recommendation was not found; run `workflow-os first-run --verbose` for available recommendation ids",
                )
            })?;
        let draft_proposal = governed_workflow_draft_proposal_from_recommendation(recommendation)?;
        if invocation.json {
            println!(
                "{}",
                first_run_recommendation_detail_json(recommendation, &draft_proposal)
            );
        } else {
            print_first_run_recommendation_detail(recommendation, &draft_proposal);
        }
        return Ok(());
    }
    if invocation.json {
        println!("{}", first_run_json(&context));
    } else {
        print_first_run_text(&context, verbose);
    }
    Ok(())
}

fn author_workflow_command(
    invocation: &Invocation,
    from_recommendation: Option<&str>,
    dry_run: bool,
    output: Option<&Path>,
) -> Result<(), WorkflowOsError> {
    if !dry_run && output.is_none() {
        return Err(WorkflowOsError::validation(
            "cli.workflow_authoring.dry_run_required",
            "workflow authoring is preview-only; rerun with --dry-run",
        ));
    }
    let recommendation_id = from_recommendation.ok_or_else(|| {
        WorkflowOsError::validation(
            "cli.workflow_authoring.recommendation_required",
            "workflow authoring dry-run requires --from-recommendation <id>",
        )
    })?;
    validate_authoring_recommendation_id(recommendation_id)?;
    let load_result = load_project(&invocation.project_dir);
    let validation = validate_loaded_project(&load_result);
    if load_result.bundle.is_none()
        && validation
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code() == "loader.manifest_missing")
    {
        return Err(WorkflowOsError::validation(
            "cli.workflow_authoring.manifest_missing",
            "no Workflow OS project was found; run `workflow-os init-repo-governance` first",
        ));
    }
    if validation.has_errors() {
        return Err(WorkflowOsError::validation(
            "cli.workflow_authoring.validation_failed",
            "project validation failed; run `workflow-os validate` for diagnostics",
        ));
    }
    let bundle = load_result.bundle.as_ref().ok_or_else(|| {
        WorkflowOsError::validation(
            "cli.workflow_authoring.project_unavailable",
            "workflow authoring dry-run requires a loaded Workflow OS project",
        )
    })?;
    let context = FirstRunReportReadyContext::new(invocation, bundle)?;
    let recommendation = context
        .workflow_discovery_recommendation(recommendation_id)
        .ok_or_else(|| {
            WorkflowOsError::validation(
                "cli.workflow_authoring.recommendation_not_found",
                "requested first-run recommendation was not found; run `workflow-os first-run --verbose` for available recommendation ids",
            )
        })?;
    let draft_proposal = governed_workflow_draft_proposal_from_recommendation(recommendation)?;
    if let Some(output) = output {
        return author_workflow_output_command(
            invocation,
            bundle,
            recommendation,
            &draft_proposal,
            dry_run,
            output,
        );
    }
    if invocation.json {
        println!(
            "{}",
            author_workflow_dry_run_json(recommendation, &draft_proposal)
        );
    } else {
        print_author_workflow_dry_run(recommendation, &draft_proposal);
    }
    Ok(())
}

fn author_workflow_output_command(
    invocation: &Invocation,
    bundle: &workflow_core::ProjectBundle,
    recommendation: &WorkflowDiscoveryRecommendation,
    draft_proposal: &GovernedWorkflowDraftProposal,
    dry_run: bool,
    output: &Path,
) -> Result<(), WorkflowOsError> {
    let output = validate_author_workflow_output_path(output)?;
    let proposed_workflow_id = proposed_workflow_id_from_output(&output)?;
    ensure_no_workflow_id_conflict(bundle, &proposed_workflow_id)?;
    if dry_run {
        if invocation.json {
            println!(
                "{}",
                author_workflow_file_output_preview_json(
                    recommendation,
                    draft_proposal,
                    &output,
                    &proposed_workflow_id,
                )
            );
        } else {
            print_author_workflow_file_output_preview(
                recommendation,
                draft_proposal,
                &output,
                &proposed_workflow_id,
            );
        }
        return Ok(());
    }
    write_author_workflow_draft(
        invocation,
        recommendation,
        draft_proposal,
        &output,
        &proposed_workflow_id,
    )?;
    if invocation.json {
        println!(
            "{}",
            author_workflow_file_output_result_json(recommendation, &output, &proposed_workflow_id)
        );
    } else {
        print_author_workflow_file_output_result(recommendation, &output, &proposed_workflow_id);
    }
    Ok(())
}

fn author_workflow_preflight_command(
    invocation: &Invocation,
    draft: &Path,
) -> Result<(), WorkflowOsError> {
    let bundle = load_author_workflow_preflight_bundle(invocation)?;
    let draft = validate_author_workflow_output_path(draft)?;
    let (absolute_draft_path, definition, content_hash) =
        load_author_workflow_preflight_draft(invocation, &draft)?;
    let candidate_workflow_id = definition.id.clone();
    let (blockers, warnings, validation_error_codes) =
        assess_author_workflow_preflight(&bundle, absolute_draft_path, definition, content_hash);
    let status = if blockers.is_empty() {
        "promotable_preflight_passed"
    } else {
        "promotion_blocked"
    };
    if invocation.json {
        println!(
            "{}",
            author_workflow_preflight_json(
                &draft,
                &candidate_workflow_id,
                status,
                &blockers,
                &warnings,
                &validation_error_codes,
            )
        );
    } else {
        print_author_workflow_preflight_result(
            &draft,
            &candidate_workflow_id,
            status,
            &blockers,
            &warnings,
        );
    }
    if !blockers.is_empty() {
        return Err(WorkflowOsError::validation(
            "cli.workflow_authoring.preflight_blocked",
            "workflow authoring preflight found promotion blockers",
        ));
    }
    Ok(())
}

fn author_workflow_draft_status_command(
    invocation: &Invocation,
    draft: &Path,
) -> Result<(), WorkflowOsError> {
    let bundle = load_author_workflow_preflight_bundle(invocation)?;
    let draft = validate_author_workflow_output_path(draft)?;
    let active_path = active_workflow_path_from_draft(&draft)?;
    let (_, definition, content_hash) = load_author_workflow_preflight_draft(invocation, &draft)?;
    let status = assess_author_workflow_draft_status(
        invocation,
        &bundle,
        &draft,
        &active_path,
        &definition,
        content_hash,
    )?;
    if invocation.json {
        println!("{}", author_workflow_draft_status_json(&status));
    } else {
        print_author_workflow_draft_status(&status);
    }
    Ok(())
}

fn author_workflow_archive_draft_command(
    invocation: &Invocation,
    draft: &Path,
    reviewer: &ActorId,
    reason: &str,
    dry_run: bool,
) -> Result<(), WorkflowOsError> {
    validate_author_workflow_archive_reason(reason)?;
    let bundle = load_author_workflow_preflight_bundle(invocation)?;
    let draft = validate_author_workflow_output_path(draft)?;
    let active_path = active_workflow_path_from_draft(&draft)?;
    let (absolute_draft_path, definition, content_hash) =
        load_author_workflow_preflight_draft(invocation, &draft)?;
    let status = assess_author_workflow_draft_status(
        invocation,
        &bundle,
        &draft,
        &active_path,
        &definition,
        content_hash,
    )?;
    let archive_path = archive_workflow_path_from_draft(&draft)?;
    let archive_absolute_path = invocation.project_dir.join(&archive_path);
    if !matches!(
        status.inferred_draft_state,
        "promoted_preserved" | "superseded_by_active"
    ) {
        emit_author_workflow_archive_draft_result(
            invocation,
            &status,
            &archive_path,
            reviewer,
            "archive_blocked",
            false,
        );
        return Err(WorkflowOsError::validation(
            "cli.workflow_authoring.archive_draft_not_eligible",
            "workflow authoring draft archive requires a promoted or superseded draft",
        ));
    }
    if archive_absolute_path.exists() {
        return Err(WorkflowOsError::validation(
            "cli.workflow_authoring.archive_destination_exists",
            "workflow authoring draft archive destination already exists",
        ));
    }
    if dry_run {
        emit_author_workflow_archive_draft_result(
            invocation,
            &status,
            &archive_path,
            reviewer,
            "archive_dry_run",
            false,
        );
        return Ok(());
    }

    archive_author_workflow_draft(invocation, &absolute_draft_path, &archive_path)?;
    validate_author_workflow_project_after_archive(invocation)?;
    emit_author_workflow_archive_draft_result(
        invocation,
        &status,
        &archive_path,
        reviewer,
        "draft_archived",
        true,
    );
    Ok(())
}

fn author_workflow_steward_review_command(
    invocation: &Invocation,
    draft: &Path,
    decision: WorkflowDraftStewardReviewDecision,
    reviewer: &ActorId,
    reason: &str,
) -> Result<(), WorkflowOsError> {
    let bundle = load_author_workflow_preflight_bundle(invocation)?;
    let draft = validate_author_workflow_output_path(draft)?;
    let (absolute_draft_path, definition, content_hash) =
        load_author_workflow_preflight_draft(invocation, &draft)?;
    let candidate_workflow_id = definition.id.clone();
    let (blockers, warnings, validation_error_codes) = assess_author_workflow_preflight(
        &bundle,
        absolute_draft_path,
        definition.clone(),
        content_hash.clone(),
    );
    if !blockers.is_empty() {
        if invocation.json {
            println!(
                "{}",
                author_workflow_steward_review_blocked_json(
                    &draft,
                    &candidate_workflow_id,
                    &blockers,
                    &warnings,
                    &validation_error_codes,
                )
            );
        } else {
            print_author_workflow_steward_review_blocked(
                &draft,
                &candidate_workflow_id,
                &blockers,
                &warnings,
            );
        }
        return Err(WorkflowOsError::validation(
            "cli.workflow_authoring.steward_review_blocked",
            "workflow authoring steward review requires passing preflight",
        ));
    }

    let input = WorkflowDraftStewardReviewInput {
        draft_path: draft.display().to_string(),
        candidate_workflow_id,
        preflight_draft_content_hash: content_hash.clone(),
        current_draft_content_hash: content_hash,
        preflight_status: WorkflowDraftPromotionPreflightStatus::Passed,
        preflight_blockers: blockers,
        preflight_warnings: warnings,
        owner_summary: owner_review_summary(&definition),
        escalation_summary: escalation_review_summary(&definition),
        policy_summary: policy_review_summary(&definition),
        evidence_report_summary: evidence_report_review_summary(&definition),
        side_effect_summary: "side_effect_posture_requires_steward_review".to_owned(),
        active_workflow_conflict: false,
        reviewer: reviewer.clone(),
        decision,
        approval_reason: reason.to_owned(),
    };
    let review = review_workflow_draft_for_promotion(input).map_err(|error| {
        WorkflowOsError::validation(error.code().to_owned(), error.message().to_owned())
    })?;

    if invocation.json {
        println!("{}", author_workflow_steward_review_json(&review));
    } else {
        print_author_workflow_steward_review_result(&review);
    }
    Ok(())
}

fn author_workflow_promote_command(
    invocation: &Invocation,
    draft: &Path,
    reviewer: &ActorId,
    reason: &str,
    dry_run: bool,
) -> Result<(), WorkflowOsError> {
    let bundle = load_author_workflow_preflight_bundle(invocation)?;
    let draft = validate_author_workflow_output_path(draft)?;
    let active_path = active_workflow_path_from_draft(&draft)?;
    let active_absolute_path = invocation.project_dir.join(&active_path);
    if active_absolute_path.exists() {
        return Err(WorkflowOsError::validation(
            "cli.workflow_authoring.active_promotion_output_exists",
            "workflow authoring active promotion output already exists",
        ));
    }

    let (absolute_draft_path, definition, content_hash) =
        load_author_workflow_preflight_draft(invocation, &draft)?;
    let draft_source = read_author_workflow_draft_source(&absolute_draft_path)?;
    let candidate_workflow_id = definition.id.clone();
    let (blockers, warnings, validation_error_codes) = assess_author_workflow_preflight(
        &bundle,
        absolute_draft_path,
        definition.clone(),
        content_hash.clone(),
    );
    if !blockers.is_empty() {
        emit_author_workflow_active_promotion_blocked(
            invocation,
            &draft,
            &active_path,
            &candidate_workflow_id,
            &blockers,
            &warnings,
            &validation_error_codes,
        );
        return Err(WorkflowOsError::validation(
            "cli.workflow_authoring.active_promotion_blocked",
            "workflow authoring active promotion requires passing preflight",
        ));
    }

    validate_author_workflow_active_context(
        &bundle,
        invocation.project_dir.join(&active_path),
        &definition,
        content_hash.clone(),
    )?;

    let review = authorize_author_workflow_active_promotion(
        &draft,
        &definition,
        content_hash.clone(),
        warnings.clone(),
        reviewer,
        reason,
    )?;

    if dry_run {
        emit_author_workflow_active_promotion_result(
            invocation,
            &review,
            &active_path,
            "active_promotion_dry_run",
            false,
        );
        return Ok(());
    }

    write_author_workflow_active_file(invocation, &active_path, &draft_source)?;
    validate_author_workflow_project_after_promotion(invocation)?;

    emit_author_workflow_active_promotion_result(
        invocation,
        &review,
        &active_path,
        "active_workflow_promoted",
        true,
    );
    Ok(())
}

fn emit_author_workflow_active_promotion_blocked(
    invocation: &Invocation,
    draft: &Path,
    active_path: &Path,
    candidate_workflow_id: &WorkflowId,
    blockers: &[String],
    warnings: &[String],
    validation_error_codes: &[String],
) {
    if invocation.json {
        println!(
            "{}",
            author_workflow_active_promotion_blocked_json(
                draft,
                active_path,
                candidate_workflow_id,
                blockers,
                warnings,
                validation_error_codes,
            )
        );
    } else {
        print_author_workflow_active_promotion_blocked(
            draft,
            active_path,
            candidate_workflow_id,
            blockers,
            warnings,
        );
    }
}

fn emit_author_workflow_active_promotion_result(
    invocation: &Invocation,
    review: &WorkflowDraftStewardReviewResult,
    active_path: &Path,
    status: &str,
    file_written: bool,
) {
    if invocation.json {
        println!(
            "{}",
            author_workflow_active_promotion_json(review, active_path, status, file_written)
        );
    } else {
        print_author_workflow_active_promotion_result(review, active_path, status, file_written);
    }
}

fn load_author_workflow_preflight_bundle(
    invocation: &Invocation,
) -> Result<workflow_core::ProjectBundle, WorkflowOsError> {
    let load_result = load_project(&invocation.project_dir);
    let validation = validate_loaded_project(&load_result);
    if load_result.bundle.is_none()
        && validation
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code() == "loader.manifest_missing")
    {
        return Err(WorkflowOsError::validation(
            "cli.workflow_authoring.manifest_missing",
            "no Workflow OS project was found; run `workflow-os init-repo-governance` first",
        ));
    }
    if validation.has_errors() {
        return Err(WorkflowOsError::validation(
            "cli.workflow_authoring.validation_failed",
            "project validation failed; run `workflow-os validate` for diagnostics",
        ));
    }
    let bundle = load_result.bundle.as_ref().ok_or_else(|| {
        WorkflowOsError::validation(
            "cli.workflow_authoring.project_unavailable",
            "workflow authoring preflight requires a loaded Workflow OS project",
        )
    })?;
    Ok(bundle.clone())
}

fn load_author_workflow_preflight_draft(
    invocation: &Invocation,
    draft: &Path,
) -> Result<(PathBuf, WorkflowDefinition, workflow_core::SpecContentHash), WorkflowOsError> {
    let absolute_draft_path = invocation.project_dir.join(draft);
    if !absolute_draft_path.is_file() {
        return Err(WorkflowOsError::validation(
            "cli.workflow_authoring.preflight_draft_missing",
            "workflow authoring preflight draft file was not found",
        ));
    }
    let source = fs::read_to_string(&absolute_draft_path).map_err(|_| {
        WorkflowOsError::invalid_state(
            "cli.workflow_authoring.preflight_read_failed",
            "workflow authoring preflight draft file could not be read",
        )
    })?;
    let definition = parse_workflow_spec_yaml(&source).map_err(|_| {
        WorkflowOsError::validation(
            "cli.workflow_authoring.preflight_parse_failed",
            "workflow authoring preflight draft could not be parsed",
        )
    })?;
    let content_hash = canonical_yaml_content_hash(&source).map_err(|_| {
        WorkflowOsError::validation(
            "cli.workflow_authoring.preflight_parse_failed",
            "workflow authoring preflight draft could not be parsed",
        )
    })?;
    Ok((absolute_draft_path, definition, content_hash))
}

fn read_author_workflow_draft_source(
    absolute_draft_path: &Path,
) -> Result<String, WorkflowOsError> {
    fs::read_to_string(absolute_draft_path).map_err(|_| {
        WorkflowOsError::invalid_state(
            "cli.workflow_authoring.preflight_read_failed",
            "workflow authoring preflight draft file could not be read",
        )
    })
}

fn assess_author_workflow_preflight(
    bundle: &workflow_core::ProjectBundle,
    absolute_draft_path: PathBuf,
    definition: WorkflowDefinition,
    content_hash: workflow_core::SpecContentHash,
) -> (Vec<String>, Vec<String>, Vec<String>) {
    let candidate_workflow_id = definition.id.clone();
    let mut blockers = BTreeSet::new();
    let mut warnings = BTreeSet::new();

    if candidate_workflow_id.as_str().starts_with("draft/") {
        blockers.insert("workflow_id_still_draft_namespace".to_owned());
    }
    if bundle
        .workflows
        .iter()
        .any(|workflow| workflow.definition.id == candidate_workflow_id)
    {
        blockers.insert("active_workflow_id_conflict".to_owned());
    }
    collect_promotion_preflight_field_blockers(&definition, &mut blockers);
    warnings.insert("purpose_authority_overlap_taxonomy_deferred".to_owned());
    warnings.insert("steward_approval_required_before_active_promotion".to_owned());
    warnings.insert("side_effect_and_report_posture_requires_review".to_owned());

    let candidate = LoadedSpec {
        path: absolute_draft_path,
        content_hash,
        definition,
    };
    let mut candidate_bundle = bundle.clone();
    candidate_bundle.workflows.push(candidate);
    let candidate_validation = validate_project_bundle(&candidate_bundle);
    let validation_error_codes = candidate_validation
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.severity() == DiagnosticSeverity::Error)
        .map(|diagnostic| diagnostic.code().to_owned())
        .collect::<BTreeSet<_>>();
    for code in &validation_error_codes {
        blockers.insert(format!("validation_error:{code}"));
    }

    let blockers = blockers.into_iter().collect::<Vec<_>>();
    let warnings = warnings.into_iter().collect::<Vec<_>>();
    let validation_error_codes = validation_error_codes.into_iter().collect::<Vec<_>>();
    (blockers, warnings, validation_error_codes)
}

struct AuthorWorkflowDraftStatus {
    draft_path: PathBuf,
    active_workflow_path: PathBuf,
    candidate_workflow_id: WorkflowId,
    draft_content_hash: workflow_core::SpecContentHash,
    matching_active_workflow_path: Option<PathBuf>,
    active_workflow_id_conflict_status: &'static str,
    inferred_draft_state: &'static str,
    recommended_next_action: &'static str,
}

fn assess_author_workflow_draft_status(
    invocation: &Invocation,
    bundle: &workflow_core::ProjectBundle,
    draft_path: &Path,
    active_path: &Path,
    definition: &WorkflowDefinition,
    content_hash: workflow_core::SpecContentHash,
) -> Result<AuthorWorkflowDraftStatus, WorkflowOsError> {
    let candidate_workflow_id = definition.id.clone();
    let active_absolute_path = invocation.project_dir.join(active_path);
    let active_path_definition = if active_absolute_path.is_file() {
        Some(read_author_workflow_active_definition(
            &active_absolute_path,
        )?)
    } else {
        None
    };
    let matching_active_workflow_path = bundle
        .workflows
        .iter()
        .find(|workflow| workflow.definition.id == candidate_workflow_id)
        .and_then(|workflow| relative_project_path(invocation, &workflow.path));

    let active_path_matches_candidate = active_path_definition
        .as_ref()
        .is_some_and(|active_definition| active_definition.id == candidate_workflow_id);
    let active_path_occupied = active_path_definition.is_some() && !active_path_matches_candidate;

    let active_workflow_id_conflict_status = if active_path_matches_candidate {
        "matching_active_workflow_present"
    } else if matching_active_workflow_path.is_some() {
        "active_workflow_id_conflict"
    } else if active_path_occupied {
        "active_workflow_path_occupied"
    } else {
        "none"
    };

    let (inferred_draft_state, recommended_next_action) = if active_path_matches_candidate {
        (
            "promoted_preserved",
            "preserve_for_review_or_plan_archive_separately",
        )
    } else if matching_active_workflow_path.is_some() || active_path_occupied {
        (
            "superseded_by_active",
            "review_active_workflow_before_any_new_promotion_attempt",
        )
    } else {
        (
            "active_candidate",
            "run_preflight_then_steward_review_before_active_promotion",
        )
    };

    Ok(AuthorWorkflowDraftStatus {
        draft_path: draft_path.to_path_buf(),
        active_workflow_path: active_path.to_path_buf(),
        candidate_workflow_id,
        draft_content_hash: content_hash,
        matching_active_workflow_path,
        active_workflow_id_conflict_status,
        inferred_draft_state,
        recommended_next_action,
    })
}

fn read_author_workflow_active_definition(
    active_path: &Path,
) -> Result<WorkflowDefinition, WorkflowOsError> {
    let source = fs::read_to_string(active_path).map_err(|_| {
        WorkflowOsError::invalid_state(
            "cli.workflow_authoring.draft_status_active_read_failed",
            "workflow authoring draft status active workflow file could not be read",
        )
    })?;
    parse_workflow_spec_yaml(&source).map_err(|_| {
        WorkflowOsError::validation(
            "cli.workflow_authoring.draft_status_active_parse_failed",
            "workflow authoring draft status active workflow file could not be parsed",
        )
    })
}

fn relative_project_path(invocation: &Invocation, path: &Path) -> Option<PathBuf> {
    path.strip_prefix(&invocation.project_dir)
        .ok()
        .map(Path::to_path_buf)
        .or_else(|| {
            if path.is_relative() {
                Some(path.to_path_buf())
            } else {
                None
            }
        })
}

fn validate_author_workflow_active_context(
    bundle: &workflow_core::ProjectBundle,
    active_absolute_path: PathBuf,
    definition: &WorkflowDefinition,
    content_hash: workflow_core::SpecContentHash,
) -> Result<(), WorkflowOsError> {
    let candidate = LoadedSpec {
        path: active_absolute_path,
        content_hash,
        definition: definition.clone(),
    };
    let mut candidate_bundle = bundle.clone();
    candidate_bundle.workflows.push(candidate);
    let validation = validate_project_bundle(&candidate_bundle);
    if validation.has_errors() {
        return Err(WorkflowOsError::validation(
            "cli.workflow_authoring.active_context_validation_failed",
            "workflow authoring active promotion failed active-context validation",
        ));
    }
    Ok(())
}

fn authorize_author_workflow_active_promotion(
    draft: &Path,
    definition: &WorkflowDefinition,
    content_hash: workflow_core::SpecContentHash,
    warnings: Vec<String>,
    reviewer: &ActorId,
    reason: &str,
) -> Result<WorkflowDraftStewardReviewResult, WorkflowOsError> {
    let input = WorkflowDraftStewardReviewInput {
        draft_path: draft.display().to_string(),
        candidate_workflow_id: definition.id.clone(),
        preflight_draft_content_hash: content_hash.clone(),
        current_draft_content_hash: content_hash,
        preflight_status: WorkflowDraftPromotionPreflightStatus::Passed,
        preflight_blockers: Vec::new(),
        preflight_warnings: warnings,
        owner_summary: owner_review_summary(definition),
        escalation_summary: escalation_review_summary(definition),
        policy_summary: policy_review_summary(definition),
        evidence_report_summary: evidence_report_review_summary(definition),
        side_effect_summary: "side_effect_posture_requires_steward_review".to_owned(),
        active_workflow_conflict: false,
        reviewer: reviewer.clone(),
        decision: WorkflowDraftStewardReviewDecision::ApprovedForPromotion,
        approval_reason: reason.to_owned(),
    };
    let review = review_workflow_draft_for_promotion(input).map_err(|error| {
        WorkflowOsError::validation(error.code().to_owned(), error.message().to_owned())
    })?;
    if review.authorization() != WorkflowDraftStewardReviewAuthorization::AuthorizedForPromotion {
        return Err(WorkflowOsError::validation(
            "cli.workflow_authoring.active_promotion_not_authorized",
            "workflow authoring active promotion was not authorized",
        ));
    }
    Ok(review)
}

fn active_workflow_path_from_draft(draft: &Path) -> Result<PathBuf, WorkflowOsError> {
    let file = draft.file_name().ok_or_else(|| {
        WorkflowOsError::validation(
            "cli.workflow_authoring.output_path_rejected",
            "workflow authoring output path must be workflows/drafts/<name>.workflow.yml",
        )
    })?;
    Ok(PathBuf::from("workflows").join(file))
}

fn archive_workflow_path_from_draft(draft: &Path) -> Result<PathBuf, WorkflowOsError> {
    let file = draft.file_name().ok_or_else(|| {
        WorkflowOsError::validation(
            "cli.workflow_authoring.archive_draft_unsafe_path",
            "workflow authoring draft archive path was rejected",
        )
    })?;
    Ok(PathBuf::from("workflows")
        .join("drafts")
        .join("archive")
        .join(file))
}

fn validate_author_workflow_archive_reason(reason: &str) -> Result<(), WorkflowOsError> {
    let trimmed = reason.trim();
    if trimmed.is_empty() || trimmed.len() > 160 || looks_secret_like(trimmed) {
        return Err(WorkflowOsError::validation(
            "cli.workflow_authoring.archive_reason_invalid",
            "workflow authoring draft archive reason was rejected",
        ));
    }
    Ok(())
}

fn archive_author_workflow_draft(
    invocation: &Invocation,
    absolute_draft_path: &Path,
    archive_path: &Path,
) -> Result<(), WorkflowOsError> {
    let archive_absolute_path = invocation.project_dir.join(archive_path);
    let parent = archive_absolute_path.parent().ok_or_else(|| {
        WorkflowOsError::invalid_state(
            "cli.workflow_authoring.archive_failed",
            "workflow authoring draft archive failed",
        )
    })?;
    fs::create_dir_all(parent).map_err(|_| {
        WorkflowOsError::invalid_state(
            "cli.workflow_authoring.archive_failed",
            "workflow authoring draft archive failed",
        )
    })?;
    fs::rename(absolute_draft_path, &archive_absolute_path).map_err(|_| {
        WorkflowOsError::invalid_state(
            "cli.workflow_authoring.archive_failed",
            "workflow authoring draft archive failed",
        )
    })
}

fn write_author_workflow_active_file(
    invocation: &Invocation,
    active_path: &Path,
    draft_source: &str,
) -> Result<(), WorkflowOsError> {
    let absolute = invocation.project_dir.join(active_path);
    if absolute.exists() {
        return Err(WorkflowOsError::validation(
            "cli.workflow_authoring.active_promotion_output_exists",
            "workflow authoring active promotion output already exists",
        ));
    }
    let parent = absolute.parent().ok_or_else(|| {
        WorkflowOsError::invalid_state(
            "cli.workflow_authoring.active_promotion_write_failed",
            "workflow authoring active promotion file could not be written",
        )
    })?;
    fs::create_dir_all(parent).map_err(|_| {
        WorkflowOsError::invalid_state(
            "cli.workflow_authoring.active_promotion_write_failed",
            "workflow authoring active promotion file could not be written",
        )
    })?;
    let file_name = active_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            WorkflowOsError::validation(
                "cli.workflow_authoring.output_path_rejected",
                "workflow authoring output path was rejected",
            )
        })?;
    let temporary_name = format!(".{file_name}.tmp-{}", std::process::id());
    let temporary = parent.join(temporary_name);
    fs::write(&temporary, draft_source).map_err(|_| {
        WorkflowOsError::invalid_state(
            "cli.workflow_authoring.active_promotion_write_failed",
            "workflow authoring active promotion file could not be written",
        )
    })?;
    fs::rename(&temporary, &absolute).map_err(|_| {
        let _ = fs::remove_file(&temporary);
        WorkflowOsError::invalid_state(
            "cli.workflow_authoring.active_promotion_write_failed",
            "workflow authoring active promotion file could not be written",
        )
    })
}

fn validate_author_workflow_project_after_promotion(
    invocation: &Invocation,
) -> Result<(), WorkflowOsError> {
    let load_result = load_project(&invocation.project_dir);
    let validation = validate_loaded_project(&load_result);
    if validation.has_errors() {
        return Err(WorkflowOsError::validation(
            "cli.workflow_authoring.active_promotion_post_validation_failed",
            "workflow authoring active promotion post-write validation failed",
        ));
    }
    Ok(())
}

fn validate_author_workflow_project_after_archive(
    invocation: &Invocation,
) -> Result<(), WorkflowOsError> {
    let load_result = load_project(&invocation.project_dir);
    let validation = validate_loaded_project(&load_result);
    if validation.has_errors() {
        return Err(WorkflowOsError::validation(
            "cli.workflow_authoring.archive_post_validation_failed",
            "workflow authoring draft archive post-write validation failed",
        ));
    }
    Ok(())
}

fn collect_promotion_preflight_field_blockers(
    definition: &WorkflowDefinition,
    blockers: &mut BTreeSet<String>,
) {
    if option_str_missing_or_placeholder(definition.owner.owning_team.as_deref())
        || definition
            .owner
            .maintainer
            .as_ref()
            .map_or(true, |maintainer| {
                is_placeholder_owner_value(maintainer.as_str())
            })
    {
        blockers.insert("owner_posture_incomplete".to_owned());
    }
    if definition
        .owner
        .escalation_contact
        .as_ref()
        .map_or(true, |contact| is_placeholder_owner_value(contact.as_str()))
    {
        blockers.insert("escalation_posture_incomplete".to_owned());
    }
    if option_str_missing_or_placeholder(definition.description.as_deref()) {
        blockers.insert("purpose_missing".to_owned());
    }
    if definition.triggers.is_empty() {
        blockers.insert("triggers_missing".to_owned());
    }
    if definition.steps.is_empty() {
        blockers.insert("steps_missing".to_owned());
    }
    if definition.owner.lifecycle_status == LifecycleStatus::Experimental
        && definition.disabled_by_default
    {
        blockers.insert("draft_lifecycle_still_inactive".to_owned());
    }
}

fn owner_review_summary(definition: &WorkflowDefinition) -> String {
    if option_str_missing_or_placeholder(definition.owner.owning_team.as_deref())
        || definition
            .owner
            .maintainer
            .as_ref()
            .map_or(true, |maintainer| {
                is_placeholder_owner_value(maintainer.as_str())
            })
    {
        "owner_posture_incomplete".to_owned()
    } else {
        "owner_posture_configured".to_owned()
    }
}

fn escalation_review_summary(definition: &WorkflowDefinition) -> String {
    if definition
        .owner
        .escalation_contact
        .as_ref()
        .map_or(true, |contact| is_placeholder_owner_value(contact.as_str()))
    {
        "escalation_posture_incomplete".to_owned()
    } else {
        "escalation_posture_configured".to_owned()
    }
}

fn policy_review_summary(definition: &WorkflowDefinition) -> String {
    let policy_refs = definition
        .steps
        .iter()
        .map(|step| step.policy_requirements.len())
        .sum::<usize>();
    let approval_refs = definition
        .steps
        .iter()
        .filter(|step| step.approval_policy.is_some())
        .count();
    if policy_refs > 0 && approval_refs > 0 {
        "policy_and_approval_posture_declared".to_owned()
    } else if policy_refs > 0 {
        "policy_posture_declared_without_step_approval".to_owned()
    } else if approval_refs > 0 {
        "approval_posture_declared_without_policy_refs".to_owned()
    } else {
        "policy_posture_not_declared".to_owned()
    }
}

fn evidence_report_review_summary(definition: &WorkflowDefinition) -> String {
    if definition
        .report_artifact_requirements
        .high_assurance_approval
        != WorkReportArtifactHighAssuranceRequirement::NotRequired
    {
        "report_artifact_high_assurance_posture_declared".to_owned()
    } else if definition.audit_requirements.required {
        "audit_posture_declared_report_artifact_not_required".to_owned()
    } else {
        "evidence_report_posture_not_declared".to_owned()
    }
}

fn option_str_missing_or_placeholder(value: Option<&str>) -> bool {
    value.map_or(true, is_placeholder_owner_value)
}

fn is_placeholder_owner_value(value: &str) -> bool {
    let normalized = value.trim();
    normalized.is_empty()
        || normalized == "local-maintainer"
        || normalized == "local-maintainers"
        || normalized == "placeholder"
        || normalized == "todo"
        || looks_secret_like(normalized)
}

fn parse_steward_review_decision(
    value: &str,
) -> Result<WorkflowDraftStewardReviewDecision, WorkflowOsError> {
    match value {
        "approved-for-promotion" => Ok(WorkflowDraftStewardReviewDecision::ApprovedForPromotion),
        "denied" => Ok(WorkflowDraftStewardReviewDecision::Denied),
        "needs-changes" => Ok(WorkflowDraftStewardReviewDecision::NeedsChanges),
        "deferred" => Ok(WorkflowDraftStewardReviewDecision::Deferred),
        _ => Err(WorkflowOsError::validation(
            "cli.workflow_authoring.steward_review_decision_invalid",
            "workflow authoring steward review decision was rejected",
        )),
    }
}

fn steward_review_decision_label(decision: WorkflowDraftStewardReviewDecision) -> &'static str {
    match decision {
        WorkflowDraftStewardReviewDecision::ApprovedForPromotion => "approved_for_promotion",
        WorkflowDraftStewardReviewDecision::Denied => "denied",
        WorkflowDraftStewardReviewDecision::NeedsChanges => "needs_changes",
        WorkflowDraftStewardReviewDecision::Deferred => "deferred",
    }
}

fn steward_review_status_label(
    authorization: WorkflowDraftStewardReviewAuthorization,
) -> &'static str {
    match authorization {
        WorkflowDraftStewardReviewAuthorization::AuthorizedForPromotion => {
            "approved_for_future_promotion"
        }
        WorkflowDraftStewardReviewAuthorization::NotAuthorized => "not_authorized",
    }
}

fn preflight_status_label(status: WorkflowDraftPromotionPreflightStatus) -> &'static str {
    match status {
        WorkflowDraftPromotionPreflightStatus::Passed => "passed",
        WorkflowDraftPromotionPreflightStatus::Blocked => "blocked",
    }
}

struct FirstRunReportReadyContext {
    scaffold_present: bool,
    git_present: bool,
    workflow_count: usize,
    skill_count: usize,
    policy_count: usize,
    test_count: usize,
    governance_posture: GovernanceFieldPosture,
    ownership_escalation_check: OwnershipEscalationCheck,
    spec_field_coverage_check: SpecFieldCoverageCheck,
    repo_metadata: SafeRepoMetadata,
    sections: Vec<WorkReportSection>,
    incomplete_work: Vec<WorkReportIncompleteWorkDisclosure>,
    known_limitations: Vec<WorkReportKnownLimitation>,
    risks: Vec<WorkReportRisk>,
    handoff_notes: Vec<WorkReportHandoffNote>,
    workflow_discovery_recommendations: Vec<WorkflowDiscoveryRecommendation>,
    recommendation_next_actions: Vec<&'static str>,
    recommendations: Vec<&'static str>,
}

impl FirstRunReportReadyContext {
    fn new(
        invocation: &Invocation,
        bundle: &workflow_core::ProjectBundle,
    ) -> Result<Self, WorkflowOsError> {
        let scaffold_present = bundle
            .workflows
            .iter()
            .any(|workflow| workflow.definition.id.as_str() == "local/first-run-governance");
        let governance_posture = GovernanceFieldPosture::from_bundle(bundle);
        let ownership_escalation_check = OwnershipEscalationCheck::from_bundle(bundle);
        let spec_field_coverage_check = SpecFieldCoverageCheck::from_bundle(bundle);
        let repo_metadata = SafeRepoMetadata::from_project_dir(&invocation.project_dir);
        let workflow_discovery_recommendations = first_run_workflow_discovery_recommendations(
            &governance_posture,
            &ownership_escalation_check,
            &spec_field_coverage_check,
            &repo_metadata,
        );
        let recommendation_next_actions =
            first_run_recommendation_next_actions(&workflow_discovery_recommendations);
        let recommendations = first_run_recommendations(&repo_metadata);
        Ok(Self {
            scaffold_present,
            git_present: invocation.project_dir.join(".git").is_dir(),
            workflow_count: bundle.workflows.len(),
            skill_count: bundle.skills.len(),
            policy_count: bundle.policies.len(),
            test_count: bundle.tests.len(),
            governance_posture,
            ownership_escalation_check,
            spec_field_coverage_check,
            repo_metadata,
            sections: first_run_sections(scaffold_present)?,
            incomplete_work: first_run_incomplete_work()?,
            known_limitations: first_run_known_limitations()?,
            risks: first_run_risks()?,
            handoff_notes: first_run_handoff_notes()?,
            workflow_discovery_recommendations,
            recommendation_next_actions,
            recommendations,
        })
    }

    fn workflow_discovery_recommendation(
        &self,
        recommendation_id: &str,
    ) -> Option<&WorkflowDiscoveryRecommendation> {
        self.workflow_discovery_recommendations
            .iter()
            .find(|recommendation| recommendation.id == recommendation_id)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct WorkflowDiscoveryRecommendation {
    id: &'static str,
    kind: WorkflowDiscoveryRecommendationKind,
    target: WorkflowDiscoveryRecommendationTarget,
    status: WorkflowDiscoveryRecommendationStatus,
    summary: &'static str,
    rationale_codes: Vec<&'static str>,
    coverage_codes: Vec<&'static str>,
    ownership_issue_codes: Vec<&'static str>,
    next_action: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SafeRepoMetadata {
    package_json: Option<PackageJsonMetadata>,
    ecosystem_files: Vec<&'static str>,
    cargo_lock_present: bool,
    python_lock_files: Vec<&'static str>,
    go_sum_present: bool,
    github_workflow_count: usize,
    conventional_source_dirs: Vec<&'static str>,
    conventional_test_dirs: Vec<&'static str>,
    repo_documents: Vec<&'static str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PackageJsonMetadata {
    package_manager: Option<&'static str>,
    common_script_keys: Vec<&'static str>,
    typescript_markers: Vec<&'static str>,
}

impl SafeRepoMetadata {
    fn from_project_dir(project_dir: &Path) -> Self {
        let package_json = package_json_metadata(project_dir);
        Self {
            package_json,
            ecosystem_files: present_files(
                project_dir,
                &[
                    ("Cargo.toml", "cargo_toml"),
                    ("pyproject.toml", "pyproject_toml"),
                    ("go.mod", "go_mod"),
                ],
            ),
            cargo_lock_present: project_dir.join("Cargo.lock").is_file(),
            python_lock_files: present_files(
                project_dir,
                &[
                    ("uv.lock", "uv_lock"),
                    ("poetry.lock", "poetry_lock"),
                    ("Pipfile.lock", "pipfile_lock"),
                    ("requirements.txt", "requirements_txt"),
                ],
            ),
            go_sum_present: project_dir.join("go.sum").is_file(),
            github_workflow_count: github_workflow_count(project_dir),
            conventional_source_dirs: present_dirs(
                project_dir,
                &[("src", "src"), ("source", "source")],
            ),
            conventional_test_dirs: present_dirs(
                project_dir,
                &[("test", "test"), ("tests", "tests")],
            ),
            repo_documents: present_file_groups(
                project_dir,
                &[
                    ("readme", &["README.md", "README"][..]),
                    ("license", &["LICENSE", "LICENSE.md", "COPYING"]),
                    ("contributing", &["CONTRIBUTING.md", "CONTRIBUTING"]),
                    ("security_policy", &["SECURITY.md", ".github/SECURITY.md"]),
                ],
            ),
        }
    }

    fn npm_package_present(&self) -> bool {
        self.package_json.is_some()
    }

    fn typescript_detected(&self) -> bool {
        self.package_json
            .as_ref()
            .is_some_and(|metadata| !metadata.typescript_markers.is_empty())
    }

    fn rust_detected(&self) -> bool {
        self.ecosystem_files.contains(&"cargo_toml")
    }

    fn python_detected(&self) -> bool {
        self.ecosystem_files.contains(&"pyproject_toml")
    }

    fn go_detected(&self) -> bool {
        self.ecosystem_files.contains(&"go_mod")
    }

    fn github_actions_detected(&self) -> bool {
        self.github_workflow_count > 0
    }
}

impl PackageJsonMetadata {
    fn has_script(&self, script_key: &str) -> bool {
        self.common_script_keys.contains(&script_key)
    }
}

fn package_json_metadata(project_dir: &Path) -> Option<PackageJsonMetadata> {
    let package_path = project_dir.join("package.json");
    let package_json = fs::read_to_string(package_path).ok()?;
    let value = serde_json::from_str::<serde_json::Value>(&package_json).ok()?;
    let scripts = value
        .get("scripts")
        .and_then(serde_json::Value::as_object)
        .map(common_package_script_keys)
        .unwrap_or_default();
    let dependencies = value
        .get("dependencies")
        .and_then(serde_json::Value::as_object);
    let dev_dependencies = value
        .get("devDependencies")
        .and_then(serde_json::Value::as_object);
    let mut typescript_markers = Vec::new();
    if package_dependency_present(dependencies, dev_dependencies, "typescript") {
        typescript_markers.push("dependency_typescript");
    }
    if package_dependency_present(dependencies, dev_dependencies, "ts-node") {
        typescript_markers.push("dependency_ts_node");
    }
    if package_dependency_present(dependencies, dev_dependencies, "tsx") {
        typescript_markers.push("dependency_tsx");
    }
    if project_dir.join("tsconfig.json").is_file() {
        typescript_markers.push("tsconfig_json");
    }
    typescript_markers.sort_unstable();
    typescript_markers.dedup();
    Some(PackageJsonMetadata {
        package_manager: package_manager_label(project_dir),
        common_script_keys: scripts,
        typescript_markers,
    })
}

fn common_package_script_keys(
    scripts: &serde_json::Map<String, serde_json::Value>,
) -> Vec<&'static str> {
    let mut keys = [
        "build",
        "test",
        "lint",
        "typecheck",
        "format",
        "prepare",
        "release",
    ]
    .into_iter()
    .filter(|key| scripts.contains_key(*key))
    .collect::<Vec<_>>();
    keys.sort_unstable();
    keys
}

fn package_dependency_present(
    dependencies: Option<&serde_json::Map<String, serde_json::Value>>,
    dev_dependencies: Option<&serde_json::Map<String, serde_json::Value>>,
    dependency_name: &str,
) -> bool {
    dependencies.is_some_and(|entries| entries.contains_key(dependency_name))
        || dev_dependencies.is_some_and(|entries| entries.contains_key(dependency_name))
}

fn package_manager_label(project_dir: &Path) -> Option<&'static str> {
    if project_dir.join("pnpm-lock.yaml").is_file() {
        Some("pnpm")
    } else if project_dir.join("yarn.lock").is_file() {
        Some("yarn")
    } else if project_dir.join("package-lock.json").is_file() {
        Some("npm")
    } else if project_dir.join("bun.lockb").is_file() || project_dir.join("bun.lock").is_file() {
        Some("bun")
    } else {
        None
    }
}

fn github_workflow_count(project_dir: &Path) -> usize {
    let workflows_dir = project_dir.join(".github").join("workflows");
    let Ok(entries) = fs::read_dir(workflows_dir) else {
        return 0;
    };
    entries
        .filter_map(Result::ok)
        .filter(|entry| {
            let path = entry.path();
            path.is_file()
                && matches!(
                    path.extension().and_then(|extension| extension.to_str()),
                    Some("yml" | "yaml")
                )
        })
        .count()
}

fn present_dirs(
    project_dir: &Path,
    candidates: &[(&'static str, &'static str)],
) -> Vec<&'static str> {
    candidates
        .iter()
        .filter_map(|(path, label)| project_dir.join(path).is_dir().then_some(*label))
        .collect()
}

fn present_files(
    project_dir: &Path,
    candidates: &[(&'static str, &'static str)],
) -> Vec<&'static str> {
    candidates
        .iter()
        .filter_map(|(path, label)| project_dir.join(path).is_file().then_some(*label))
        .collect()
}

fn present_file_groups(
    project_dir: &Path,
    candidates: &[(&'static str, &[&str])],
) -> Vec<&'static str> {
    candidates
        .iter()
        .filter_map(|(label, paths)| any_file_present(project_dir, paths).then_some(*label))
        .collect()
}

fn any_file_present(project_dir: &Path, candidates: &[&str]) -> bool {
    candidates
        .iter()
        .any(|candidate| project_dir.join(candidate).is_file())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WorkflowDiscoveryRecommendationKind {
    CreateWorkflow,
    AssignOwnership,
    AddEvidenceCheckRequirements,
    AddSideEffectPosture,
    AddReportHandoffObligations,
}

impl WorkflowDiscoveryRecommendationKind {
    fn label(self) -> &'static str {
        match self {
            Self::CreateWorkflow => "create_workflow",
            Self::AssignOwnership => "assign_ownership",
            Self::AddEvidenceCheckRequirements => "add_evidence_check_requirements",
            Self::AddSideEffectPosture => "add_side_effect_posture",
            Self::AddReportHandoffObligations => "add_report_handoff_obligations",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct WorkflowDiscoveryRecommendationTarget {
    surface: WorkflowDiscoveryRecommendationTargetSurface,
    ordinal: usize,
}

impl WorkflowDiscoveryRecommendationTarget {
    fn project() -> Self {
        Self {
            surface: WorkflowDiscoveryRecommendationTargetSurface::Project,
            ordinal: 1,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WorkflowDiscoveryRecommendationTargetSurface {
    Project,
}

impl WorkflowDiscoveryRecommendationTargetSurface {
    fn label(self) -> &'static str {
        match self {
            Self::Project => "project",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WorkflowDiscoveryRecommendationStatus {
    ReviewOnly,
    NeedsHumanReview,
}

impl WorkflowDiscoveryRecommendationStatus {
    fn label(self) -> &'static str {
        match self {
            Self::ReviewOnly => "review_only",
            Self::NeedsHumanReview => "needs_human_review",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GovernanceFieldPosture {
    profile: &'static str,
    profile_posture: &'static str,
    ownership: FieldPosture,
    escalation: FieldPosture,
    approvals: FieldPosture,
    policy_gates: FieldPosture,
    evidence: FieldPosture,
    checks: FieldPosture,
    side_effects: FieldPosture,
    audit_observability: FieldPosture,
    deferred_fields: &'static [&'static str],
}

impl GovernanceFieldPosture {
    fn from_bundle(bundle: &workflow_core::ProjectBundle) -> Self {
        let ownership = ownership_posture(bundle);
        let escalation = escalation_posture(bundle);
        let approvals = if has_approval_posture(bundle) {
            FieldPosture::Configured
        } else {
            FieldPosture::NotRequired
        };
        let policy_gates = if has_policy_requirements(bundle) {
            FieldPosture::DeclaredNotEvaluated
        } else {
            FieldPosture::NotDeclared
        };
        let audit_observability = if has_audit_or_observability(bundle) {
            FieldPosture::DeclaredRuntimeAfterRun
        } else {
            FieldPosture::Missing
        };
        let side_effects = if has_external_adapter_requirements(bundle) {
            FieldPosture::DeclaredUnsupported
        } else {
            FieldPosture::NoneSkippedUnsupported
        };

        Self {
            profile: "observe_and_report",
            profile_posture: "disclosed_not_enforced",
            ownership,
            escalation,
            approvals,
            policy_gates,
            evidence: FieldPosture::NotAvailable,
            checks: FieldPosture::Skipped,
            side_effects,
            audit_observability,
            deferred_fields: &[
                "triggers_declared_not_background_executed",
                "state_model_advisory",
                "tests_declared_not_automatically_executed",
                "workflow_recommendations_review_only",
            ],
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FieldPosture {
    Configured,
    Placeholder,
    Missing,
    NotRequired,
    NotDeclared,
    DeclaredNotEvaluated,
    DeclaredRuntimeAfterRun,
    DeclaredUnsupported,
    NotAvailable,
    Skipped,
    NoneSkippedUnsupported,
}

impl FieldPosture {
    fn label(self) -> &'static str {
        match self {
            Self::Configured => "configured",
            Self::Placeholder => "placeholder",
            Self::Missing => "missing",
            Self::NotRequired => "not_required",
            Self::NotDeclared => "not_declared",
            Self::DeclaredNotEvaluated => "declared_not_evaluated",
            Self::DeclaredRuntimeAfterRun => "declared_runtime_after_run",
            Self::DeclaredUnsupported => "declared_unsupported",
            Self::NotAvailable => "not_available",
            Self::Skipped => "skipped",
            Self::NoneSkippedUnsupported => "none_skipped_unsupported",
        }
    }
}

fn ownership_posture(bundle: &workflow_core::ProjectBundle) -> FieldPosture {
    let mut saw_owner = false;
    let mut saw_placeholder = false;
    let mut saw_configured = false;

    for owner in bundle
        .workflows
        .iter()
        .map(|workflow| &workflow.definition.owner)
        .chain(bundle.skills.iter().map(|skill| &skill.definition.owner))
    {
        let values = [
            owner.owning_team.as_deref(),
            owner.maintainer.as_ref().map(ActorId::as_str),
        ];
        for value in values.into_iter().flatten() {
            saw_owner = true;
            if is_placeholder_governance_value(value) {
                saw_placeholder = true;
            } else {
                saw_configured = true;
            }
        }
    }

    classify_configured_placeholder(saw_owner, saw_configured, saw_placeholder)
}

fn escalation_posture(bundle: &workflow_core::ProjectBundle) -> FieldPosture {
    let mut saw_escalation = false;
    let mut saw_placeholder = false;
    let mut saw_configured = false;

    for value in bundle
        .workflows
        .iter()
        .filter_map(|workflow| workflow.definition.owner.escalation_contact.as_ref())
        .chain(
            bundle
                .skills
                .iter()
                .filter_map(|skill| skill.definition.owner.escalation_contact.as_ref()),
        )
        .map(ActorId::as_str)
    {
        saw_escalation = true;
        if is_placeholder_governance_value(value) {
            saw_placeholder = true;
        } else {
            saw_configured = true;
        }
    }

    classify_configured_placeholder(saw_escalation, saw_configured, saw_placeholder)
}

fn classify_configured_placeholder(
    saw_any: bool,
    saw_configured: bool,
    saw_placeholder: bool,
) -> FieldPosture {
    if !saw_any {
        FieldPosture::Missing
    } else if saw_configured && !saw_placeholder {
        FieldPosture::Configured
    } else {
        FieldPosture::Placeholder
    }
}

fn is_placeholder_governance_value(value: &str) -> bool {
    matches!(
        value,
        "local-maintainer" | "local-maintainers" | "workflow-os" | "workflow-os-maintainers"
    )
}

fn has_approval_posture(bundle: &workflow_core::ProjectBundle) -> bool {
    bundle
        .workflows
        .iter()
        .any(|workflow| !workflow.definition.approval_requirements.is_empty())
        || bundle.workflows.iter().any(|workflow| {
            workflow
                .definition
                .steps
                .iter()
                .any(|step| step.approval_policy.is_some())
        })
        || bundle.skills.iter().any(|skill| {
            matches!(
                skill.definition.approval_sensitivity,
                workflow_core::ApprovalSensitivity::Medium
                    | workflow_core::ApprovalSensitivity::High
            )
        })
}

fn has_policy_requirements(bundle: &workflow_core::ProjectBundle) -> bool {
    bundle.workflows.iter().any(|workflow| {
        !workflow.definition.approval_requirements.is_empty()
            || !workflow.definition.retry_policy_refs.is_empty()
            || !workflow.definition.escalation_policy_refs.is_empty()
            || workflow
                .definition
                .steps
                .iter()
                .any(|step| !step.policy_requirements.is_empty())
    })
}

fn has_audit_or_observability(bundle: &workflow_core::ProjectBundle) -> bool {
    bundle.workflows.iter().any(|workflow| {
        workflow.definition.audit_requirements.required
            || !workflow.definition.audit_requirements.events.is_empty()
            || !workflow
                .definition
                .observability_requirements
                .metrics
                .is_empty()
            || workflow.definition.observability_requirements.tracing
            || workflow
                .definition
                .observability_requirements
                .latency_tracking
    }) || bundle.skills.iter().any(|skill| {
        skill.definition.audit_requirements.required
            || !skill.definition.audit_requirements.events.is_empty()
            || !skill
                .definition
                .observability_requirements
                .metrics
                .is_empty()
            || skill.definition.observability_requirements.tracing
            || skill.definition.observability_requirements.latency_tracking
    })
}

fn has_external_adapter_requirements(bundle: &workflow_core::ProjectBundle) -> bool {
    bundle
        .skills
        .iter()
        .any(|skill| !skill.definition.adapter_requirements.is_empty())
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SpecFieldCoverageCheck {
    items: Vec<SpecFieldCoverageItem>,
}

impl SpecFieldCoverageCheck {
    fn from_bundle(bundle: &workflow_core::ProjectBundle) -> Self {
        let mut items = project_spec_field_coverage_items().to_vec();

        if !bundle.workflows.is_empty() {
            items.extend(workflow_spec_field_coverage_items());
        }

        if !bundle.skills.is_empty() {
            items.extend(skill_spec_field_coverage_items());
        }

        if !bundle.policies.is_empty() {
            items.extend(policy_spec_field_coverage_items());
        }

        if !bundle.tests.is_empty() {
            items.extend(test_spec_field_coverage_items());
        }

        Self { items }
    }

    fn status_label(&self) -> &'static str {
        if self.count(SpecFieldCoverageCategory::Advisory) > 0
            || self.count(SpecFieldCoverageCategory::Deferred) > 0
        {
            "warnings"
        } else {
            "passed"
        }
    }

    fn count(&self, category: SpecFieldCoverageCategory) -> usize {
        self.items
            .iter()
            .filter(|item| item.category == category)
            .count()
    }
}

fn project_spec_field_coverage_items() -> [SpecFieldCoverageItem; 4] {
    [
        SpecFieldCoverageItem::new(
            SpecFieldCoverageSurface::Project,
            "schema_version",
            SpecFieldCoverageCategory::Validated,
            "validated",
            "spec_field.project.schema_version_validated",
        ),
        SpecFieldCoverageItem::new(
            SpecFieldCoverageSurface::Project,
            "project_identity",
            SpecFieldCoverageCategory::Validated,
            "validated",
            "spec_field.project.identity_validated",
        ),
        SpecFieldCoverageItem::new(
            SpecFieldCoverageSurface::Project,
            "layout",
            SpecFieldCoverageCategory::Validated,
            "validated",
            "spec_field.project.layout_validated",
        ),
        SpecFieldCoverageItem::new(
            SpecFieldCoverageSurface::Project,
            "config",
            SpecFieldCoverageCategory::Advisory,
            "advisory_no_value_output",
            "spec_field.project.config_advisory",
        ),
    ]
}

fn workflow_spec_field_coverage_items() -> [SpecFieldCoverageItem; 13] {
    [
        SpecFieldCoverageItem::new(
            SpecFieldCoverageSurface::Workflow,
            "identity",
            SpecFieldCoverageCategory::Validated,
            "validated",
            "spec_field.workflow.identity_validated",
        ),
        SpecFieldCoverageItem::new(
            SpecFieldCoverageSurface::Workflow,
            "owner",
            SpecFieldCoverageCategory::Disclosed,
            "disclosed_without_values",
            "spec_field.workflow.owner_disclosed",
        ),
        SpecFieldCoverageItem::new(
            SpecFieldCoverageSurface::Workflow,
            "autonomy",
            SpecFieldCoverageCategory::Validated,
            "validated_and_disclosed",
            "spec_field.workflow.autonomy_validated",
        ),
        SpecFieldCoverageItem::new(
            SpecFieldCoverageSurface::Workflow,
            "triggers",
            SpecFieldCoverageCategory::Validated,
            "validated_deferred_execution",
            "spec_field.triggers.not_background_executed",
        ),
        SpecFieldCoverageItem::new(
            SpecFieldCoverageSurface::Workflow,
            "state_model",
            SpecFieldCoverageCategory::Advisory,
            "advisory",
            "spec_field.workflow.state_model_advisory",
        ),
        SpecFieldCoverageItem::new(
            SpecFieldCoverageSurface::Workflow,
            "steps",
            SpecFieldCoverageCategory::Enforced,
            "enforced_supported_local_paths",
            "spec_field.workflow.steps_enforced_supported_local_paths",
        ),
        SpecFieldCoverageItem::new(
            SpecFieldCoverageSurface::Workflow,
            "branches",
            SpecFieldCoverageCategory::Validated,
            "validated_deferred_branching_runtime",
            "spec_field.workflow.branches_not_runtime_branching",
        ),
        SpecFieldCoverageItem::new(
            SpecFieldCoverageSurface::Workflow,
            "mappings",
            SpecFieldCoverageCategory::Advisory,
            "advisory_no_value_output",
            "spec_field.workflow.mappings_advisory",
        ),
        SpecFieldCoverageItem::new(
            SpecFieldCoverageSurface::Workflow,
            "policy_requirements",
            SpecFieldCoverageCategory::Enforced,
            "enforced_supported_local_paths",
            "spec_field.workflow.policy_requirements_enforced_supported_local_paths",
        ),
        SpecFieldCoverageItem::new(
            SpecFieldCoverageSurface::Workflow,
            "approval_requirements",
            SpecFieldCoverageCategory::Enforced,
            "enforced_supported_local_paths",
            "spec_field.workflow.approval_requirements_enforced_supported_local_paths",
        ),
        SpecFieldCoverageItem::new(
            SpecFieldCoverageSurface::Workflow,
            "retry_escalation",
            SpecFieldCoverageCategory::Enforced,
            "enforced_supported_local_paths",
            "spec_field.workflow.retry_escalation_enforced_supported_local_paths",
        ),
        SpecFieldCoverageItem::new(
            SpecFieldCoverageSurface::Workflow,
            "timeout_cancellation",
            SpecFieldCoverageCategory::Validated,
            "validated_selected_cases",
            "spec_field.workflow.timeout_cancellation_validated",
        ),
        SpecFieldCoverageItem::new(
            SpecFieldCoverageSurface::Workflow,
            "audit_observability",
            SpecFieldCoverageCategory::Disclosed,
            "disclosed_runtime_after_run",
            "spec_field.workflow.audit_observability_disclosed",
        ),
    ]
}

fn skill_spec_field_coverage_items() -> [SpecFieldCoverageItem; 5] {
    [
        SpecFieldCoverageItem::new(
            SpecFieldCoverageSurface::Skill,
            "identity",
            SpecFieldCoverageCategory::Validated,
            "validated",
            "spec_field.skill.identity_validated",
        ),
        SpecFieldCoverageItem::new(
            SpecFieldCoverageSurface::Skill,
            "contracts",
            SpecFieldCoverageCategory::Validated,
            "validated",
            "spec_field.skill.contracts_validated",
        ),
        SpecFieldCoverageItem::new(
            SpecFieldCoverageSurface::Skill,
            "sensitivity_redaction",
            SpecFieldCoverageCategory::Validated,
            "validated",
            "spec_field.skill.sensitivity_redaction_validated",
        ),
        SpecFieldCoverageItem::new(
            SpecFieldCoverageSurface::Skill,
            "capabilities_adapters",
            SpecFieldCoverageCategory::Disclosed,
            "validated_writes_deferred",
            "spec_field.skill.capabilities_adapters_writes_deferred",
        ),
        SpecFieldCoverageItem::new(
            SpecFieldCoverageSurface::Skill,
            "failure_evaluation",
            SpecFieldCoverageCategory::Validated,
            "validated",
            "spec_field.skill.failure_evaluation_validated",
        ),
    ]
}

fn policy_spec_field_coverage_items() -> [SpecFieldCoverageItem; 2] {
    [
        SpecFieldCoverageItem::new(
            SpecFieldCoverageSurface::Policy,
            "identity_rules",
            SpecFieldCoverageCategory::Validated,
            "validated",
            "spec_field.policy.identity_rules_validated",
        ),
        SpecFieldCoverageItem::new(
            SpecFieldCoverageSurface::Policy,
            "effects",
            SpecFieldCoverageCategory::Validated,
            "validated",
            "spec_field.policy.effects_validated",
        ),
    ]
}

fn test_spec_field_coverage_items() -> [SpecFieldCoverageItem; 2] {
    [
        SpecFieldCoverageItem::new(
            SpecFieldCoverageSurface::Test,
            "identity_target",
            SpecFieldCoverageCategory::Validated,
            "validated",
            "spec_field.test.identity_target_validated",
        ),
        SpecFieldCoverageItem::new(
            SpecFieldCoverageSurface::Test,
            "assertions",
            SpecFieldCoverageCategory::Deferred,
            "validated_deferred_execution",
            "spec_field.tests.not_automatically_executed",
        ),
    ]
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SpecFieldCoverageSurface {
    Project,
    Workflow,
    Skill,
    Policy,
    Test,
}

impl SpecFieldCoverageSurface {
    fn label(self) -> &'static str {
        match self {
            Self::Project => "project",
            Self::Workflow => "workflow",
            Self::Skill => "skill",
            Self::Policy => "policy",
            Self::Test => "test",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SpecFieldCoverageCategory {
    Enforced,
    Validated,
    Disclosed,
    Advisory,
    Deferred,
}

impl SpecFieldCoverageCategory {
    fn label(self) -> &'static str {
        match self {
            Self::Enforced => "enforced",
            Self::Validated => "validated",
            Self::Disclosed => "disclosed",
            Self::Advisory => "advisory",
            Self::Deferred => "deferred",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SpecFieldCoverageItem {
    surface: SpecFieldCoverageSurface,
    field: &'static str,
    category: SpecFieldCoverageCategory,
    posture: &'static str,
    code: &'static str,
}

impl SpecFieldCoverageItem {
    fn new(
        surface: SpecFieldCoverageSurface,
        field: &'static str,
        category: SpecFieldCoverageCategory,
        posture: &'static str,
        code: &'static str,
    ) -> Self {
        Self {
            surface,
            field,
            category,
            posture,
            code,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct OwnershipEscalationCheck {
    issues: Vec<OwnershipEscalationIssue>,
}

impl OwnershipEscalationCheck {
    fn from_bundle(bundle: &workflow_core::ProjectBundle) -> Self {
        let mut issues = Vec::new();
        for (index, workflow) in bundle.workflows.iter().enumerate() {
            let ordinal = index + 1;
            let owner_posture = owner_metadata_posture(&workflow.definition.owner);
            let escalation_posture = escalation_metadata_posture(&workflow.definition.owner);
            push_owner_escalation_issues(
                &mut issues,
                OwnershipEscalationTargetKind::Workflow,
                ordinal,
                owner_posture,
                escalation_posture,
            );
            push_lifecycle_issue(
                &mut issues,
                OwnershipEscalationTargetKind::Workflow,
                ordinal,
                workflow.definition.owner.lifecycle_status,
            );
            if workflow_requires_responsible_context(&workflow.definition)
                && (owner_posture != FieldPosture::Configured
                    || escalation_posture != FieldPosture::Configured)
            {
                issues.push(OwnershipEscalationIssue::new(
                    OwnershipEscalationTargetKind::Workflow,
                    ordinal,
                    OwnershipEscalationIssueCode::AuthorityContextRequired,
                ));
            }
        }
        for (index, skill) in bundle.skills.iter().enumerate() {
            let ordinal = index + 1;
            let owner_posture = owner_metadata_posture(&skill.definition.owner);
            let escalation_posture = escalation_metadata_posture(&skill.definition.owner);
            push_owner_escalation_issues(
                &mut issues,
                OwnershipEscalationTargetKind::Skill,
                ordinal,
                owner_posture,
                escalation_posture,
            );
            push_lifecycle_issue(
                &mut issues,
                OwnershipEscalationTargetKind::Skill,
                ordinal,
                skill.definition.owner.lifecycle_status,
            );
            if skill_requires_responsible_context(&skill.definition)
                && (owner_posture != FieldPosture::Configured
                    || escalation_posture != FieldPosture::Configured)
            {
                issues.push(OwnershipEscalationIssue::new(
                    OwnershipEscalationTargetKind::Skill,
                    ordinal,
                    OwnershipEscalationIssueCode::AuthorityContextRequired,
                ));
            }
        }
        Self { issues }
    }

    fn status_label(&self) -> &'static str {
        if self.issues.is_empty() {
            "passed"
        } else {
            "warnings"
        }
    }

    fn count(&self, code: OwnershipEscalationIssueCode) -> usize {
        self.issues
            .iter()
            .filter(|issue| issue.code == code)
            .count()
    }

    fn lifecycle_warning_count(&self) -> usize {
        self.count(OwnershipEscalationIssueCode::LifecycleExperimental)
            + self.count(OwnershipEscalationIssueCode::LifecycleDeprecated)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OwnershipEscalationTargetKind {
    Workflow,
    Skill,
}

impl OwnershipEscalationTargetKind {
    fn label(self) -> &'static str {
        match self {
            Self::Workflow => "workflow",
            Self::Skill => "skill",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OwnershipEscalationIssueCode {
    MissingOwner,
    PlaceholderOwner,
    MissingEscalation,
    PlaceholderEscalation,
    LifecycleExperimental,
    LifecycleDeprecated,
    AuthorityContextRequired,
}

impl OwnershipEscalationIssueCode {
    fn label(self) -> &'static str {
        match self {
            Self::MissingOwner => "ownership.missing_owner",
            Self::PlaceholderOwner => "ownership.placeholder_owner",
            Self::MissingEscalation => "escalation.missing_contact",
            Self::PlaceholderEscalation => "escalation.placeholder_contact",
            Self::LifecycleExperimental => "lifecycle.experimental",
            Self::LifecycleDeprecated => "lifecycle.deprecated",
            Self::AuthorityContextRequired => "authority.owner_context_required",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct OwnershipEscalationIssue {
    target_kind: OwnershipEscalationTargetKind,
    target_ordinal: usize,
    code: OwnershipEscalationIssueCode,
}

impl OwnershipEscalationIssue {
    fn new(
        target_kind: OwnershipEscalationTargetKind,
        target_ordinal: usize,
        code: OwnershipEscalationIssueCode,
    ) -> Self {
        Self {
            target_kind,
            target_ordinal,
            code,
        }
    }
}

fn owner_metadata_posture(owner: &workflow_core::OwnershipMetadata) -> FieldPosture {
    let mut saw_owner = false;
    let mut saw_placeholder = false;
    let mut saw_configured = false;
    let values = [
        owner.owning_team.as_deref(),
        owner.maintainer.as_ref().map(ActorId::as_str),
    ];
    for value in values.into_iter().flatten() {
        saw_owner = true;
        if is_placeholder_governance_value(value) {
            saw_placeholder = true;
        } else {
            saw_configured = true;
        }
    }
    classify_configured_placeholder(saw_owner, saw_configured, saw_placeholder)
}

fn escalation_metadata_posture(owner: &workflow_core::OwnershipMetadata) -> FieldPosture {
    match owner.escalation_contact.as_ref().map(ActorId::as_str) {
        None => FieldPosture::Missing,
        Some(value) if is_placeholder_governance_value(value) => FieldPosture::Placeholder,
        Some(_) => FieldPosture::Configured,
    }
}

fn push_owner_escalation_issues(
    issues: &mut Vec<OwnershipEscalationIssue>,
    target_kind: OwnershipEscalationTargetKind,
    target_ordinal: usize,
    owner_posture: FieldPosture,
    escalation_posture: FieldPosture,
) {
    match owner_posture {
        FieldPosture::Missing => issues.push(OwnershipEscalationIssue::new(
            target_kind,
            target_ordinal,
            OwnershipEscalationIssueCode::MissingOwner,
        )),
        FieldPosture::Placeholder => issues.push(OwnershipEscalationIssue::new(
            target_kind,
            target_ordinal,
            OwnershipEscalationIssueCode::PlaceholderOwner,
        )),
        _ => {}
    }
    match escalation_posture {
        FieldPosture::Missing => issues.push(OwnershipEscalationIssue::new(
            target_kind,
            target_ordinal,
            OwnershipEscalationIssueCode::MissingEscalation,
        )),
        FieldPosture::Placeholder => issues.push(OwnershipEscalationIssue::new(
            target_kind,
            target_ordinal,
            OwnershipEscalationIssueCode::PlaceholderEscalation,
        )),
        _ => {}
    }
}

fn push_lifecycle_issue(
    issues: &mut Vec<OwnershipEscalationIssue>,
    target_kind: OwnershipEscalationTargetKind,
    target_ordinal: usize,
    lifecycle_status: workflow_core::LifecycleStatus,
) {
    match lifecycle_status {
        workflow_core::LifecycleStatus::Experimental => issues.push(OwnershipEscalationIssue::new(
            target_kind,
            target_ordinal,
            OwnershipEscalationIssueCode::LifecycleExperimental,
        )),
        workflow_core::LifecycleStatus::Deprecated => issues.push(OwnershipEscalationIssue::new(
            target_kind,
            target_ordinal,
            OwnershipEscalationIssueCode::LifecycleDeprecated,
        )),
        workflow_core::LifecycleStatus::Stable => {}
    }
}

fn workflow_requires_responsible_context(definition: &workflow_core::WorkflowDefinition) -> bool {
    !definition.approval_requirements.is_empty()
        || !definition.escalation_policy_refs.is_empty()
        || definition
            .steps
            .iter()
            .any(|step| step.approval_policy.is_some())
}

fn skill_requires_responsible_context(definition: &workflow_core::SkillDefinition) -> bool {
    !definition.adapter_requirements.is_empty()
        || matches!(
            definition.approval_sensitivity,
            workflow_core::ApprovalSensitivity::Medium | workflow_core::ApprovalSensitivity::High
        )
}

fn first_run_sections(scaffold_present: bool) -> Result<Vec<WorkReportSection>, WorkflowOsError> {
    let scaffold_summary = if scaffold_present {
        "First-run governance scaffold was detected and validated; no workflow run was executed."
    } else {
        "A valid Workflow OS project was detected; first-run governance scaffold was not detected."
    };
    let definitions = [
        (WorkReportSectionKind::WorkPerformed, scaffold_summary),
        (
            WorkReportSectionKind::EvidenceConsidered,
            "Evidence is not available yet; first-run mode did not execute adapters, checks, or provider reads.",
        ),
        (
            WorkReportSectionKind::DecisionsMade,
            "No runtime decisions were made; recommended workflow candidates are review-only.",
        ),
        (
            WorkReportSectionKind::PolicyGatesEvaluated,
            "Project validation passed; no runtime policy gate was evaluated.",
        ),
        (
            WorkReportSectionKind::Approvals,
            "No approval was requested, granted, denied, or recorded by first-run mode.",
        ),
        (
            WorkReportSectionKind::ValidationAndQualityChecks,
            "Static Workflow OS project validation passed; local commands and external checks were skipped.",
        ),
        (
            WorkReportSectionKind::SideEffects,
            "Side effects are none, skipped, and unsupported in first-run mode.",
        ),
        (
            WorkReportSectionKind::IncompleteOrDeferredWork,
            "Command execution, provider reads, evidence capture, workflow execution, and report artifacts are deferred.",
        ),
        (
            WorkReportSectionKind::KnownLimitations,
            "This is a report-ready context, not a terminal WorkReport from a completed workflow run.",
        ),
        (
            WorkReportSectionKind::Risks,
            "Review-only recommendations may be incomplete until real project evidence and checks are supplied.",
        ),
        (
            WorkReportSectionKind::OperatorHandoffNotes,
            "Next step is to review the scaffold, then run the governed workflow explicitly if desired.",
        ),
    ];
    definitions
        .into_iter()
        .map(|(kind, summary)| WorkReportSection::new(kind, Some(summary.to_owned()), Vec::new()))
        .collect()
}

fn first_run_incomplete_work() -> Result<Vec<WorkReportIncompleteWorkDisclosure>, WorkflowOsError> {
    Ok(vec![
        WorkReportIncompleteWorkDisclosure::new(
            "No workflow run was started and no runtime events were appended.",
            Vec::new(),
        )?,
        WorkReportIncompleteWorkDisclosure::new(
            "No repository commands, tests, provider reads, or local check handlers were executed.",
            Vec::new(),
        )?,
        WorkReportIncompleteWorkDisclosure::new(
            "No WorkReport artifact was written; report artifact generation remains separately scoped.",
            Vec::new(),
        )?,
    ])
}

fn first_run_known_limitations() -> Result<Vec<WorkReportKnownLimitation>, WorkflowOsError> {
    Ok(vec![
        WorkReportKnownLimitation::new(
            "First-run mode validates the Workflow OS project envelope but does not inspect raw repository contents.",
            Vec::new(),
        )?,
        WorkReportKnownLimitation::new(
            "Workflow recommendations are bounded operator hints, not automatically registered workflows.",
            Vec::new(),
        )?,
    ])
}

fn first_run_risks() -> Result<Vec<WorkReportRisk>, WorkflowOsError> {
    Ok(vec![
        WorkReportRisk::new(
            "A scaffold can be mistaken for executed governance unless operators review the explicit skipped sections.",
            Vec::new(),
        )?,
        WorkReportRisk::new(
            "Real evidence, local checks, and approvals are still required before relying on governed outcomes.",
            Vec::new(),
        )?,
    ])
}

fn first_run_handoff_notes() -> Result<Vec<WorkReportHandoffNote>, WorkflowOsError> {
    Ok(vec![
        WorkReportHandoffNote::new(
            "Review AGENTS.md and .workflow-os/agent-harness-prompt.md with the agent or maintainer.",
            Vec::new(),
        )?,
        WorkReportHandoffNote::new(
            "When ready, run `workflow-os --mock-all-local-skills run local/first-run-governance` explicitly.",
            Vec::new(),
        )?,
    ])
}

fn first_run_recommendations(repo_metadata: &SafeRepoMetadata) -> Vec<&'static str> {
    let mut recommendations = vec![
        "formalize a repo implementation workflow with evidence and final report obligations",
        "formalize a PR review workflow before merge-sensitive changes",
        "formalize a release readiness workflow before public release or package publishing",
    ];
    if repo_metadata.typescript_detected() {
        recommendations.push(
            "review TypeScript package metadata and decide required build, test, lint, and typecheck obligations",
        );
    } else if repo_metadata.npm_package_present() {
        recommendations
            .push("review package metadata and decide required package validation obligations");
    }
    if repo_metadata.rust_detected() {
        recommendations.push(
            "review Rust metadata and decide required fmt, clippy, test, and release obligations",
        );
    }
    if repo_metadata.python_detected() {
        recommendations.push(
            "review Python metadata and decide required test, lint, typecheck, and packaging obligations",
        );
    }
    if repo_metadata.go_detected() {
        recommendations.push(
            "review Go metadata and decide required test, vet, build, and module obligations",
        );
    }
    if repo_metadata.github_actions_detected() {
        recommendations
            .push("review GitHub Actions workflow presence and decide CI evidence obligations");
    }
    recommendations
}

fn first_run_recommendation_next_actions(
    recommendations: &[WorkflowDiscoveryRecommendation],
) -> Vec<&'static str> {
    let mut actions =
        vec!["review_only: recommendations are not active workflows until authored and reviewed"];
    if recommendation_present(recommendations, "first_run.assign_ownership") {
        actions.push("start_with: first_run.assign_ownership");
    }
    if let Some(workflow_id) = first_present_recommendation(
        recommendations,
        &[
            "first_run.typescript_implementation",
            "first_run.rust_implementation",
            "first_run.python_implementation",
            "first_run.go_implementation",
            "first_run.repo_implementation",
        ],
    ) {
        actions.push(match workflow_id {
            "first_run.typescript_implementation" => {
                "workflow_candidate: first_run.typescript_implementation"
            }
            "first_run.rust_implementation" => "workflow_candidate: first_run.rust_implementation",
            "first_run.python_implementation" => {
                "workflow_candidate: first_run.python_implementation"
            }
            "first_run.go_implementation" => "workflow_candidate: first_run.go_implementation",
            _ => "workflow_candidate: first_run.repo_implementation",
        });
    }
    if let Some(validation_id) = first_present_recommendation(
        recommendations,
        &[
            "first_run.package_validation_obligations",
            "first_run.rust_validation_obligations",
            "first_run.python_validation_obligations",
            "first_run.go_validation_obligations",
            "first_run.github_actions_ci_evidence",
            "first_run.evidence_check_requirements",
        ],
    ) {
        actions.push(match validation_id {
            "first_run.package_validation_obligations" => {
                "validation_candidate: first_run.package_validation_obligations"
            }
            "first_run.rust_validation_obligations" => {
                "validation_candidate: first_run.rust_validation_obligations"
            }
            "first_run.python_validation_obligations" => {
                "validation_candidate: first_run.python_validation_obligations"
            }
            "first_run.go_validation_obligations" => {
                "validation_candidate: first_run.go_validation_obligations"
            }
            "first_run.github_actions_ci_evidence" => {
                "validation_candidate: first_run.github_actions_ci_evidence"
            }
            _ => "validation_candidate: first_run.evidence_check_requirements",
        });
    }
    if recommendation_present(recommendations, "first_run.side_effect_posture") {
        actions.push("safety_candidate: first_run.side_effect_posture");
    }
    if recommendation_present(recommendations, "first_run.report_handoff_obligations") {
        actions.push("closure_candidate: first_run.report_handoff_obligations");
    }
    actions
}

fn recommendation_present(
    recommendations: &[WorkflowDiscoveryRecommendation],
    id: &'static str,
) -> bool {
    recommendations
        .iter()
        .any(|recommendation| recommendation.id == id)
}

fn first_present_recommendation(
    recommendations: &[WorkflowDiscoveryRecommendation],
    ordered_ids: &[&'static str],
) -> Option<&'static str> {
    ordered_ids
        .iter()
        .copied()
        .find(|id| recommendation_present(recommendations, id))
}

fn first_run_workflow_discovery_recommendations(
    governance_posture: &GovernanceFieldPosture,
    ownership_escalation_check: &OwnershipEscalationCheck,
    spec_field_coverage_check: &SpecFieldCoverageCheck,
    repo_metadata: &SafeRepoMetadata,
) -> Vec<WorkflowDiscoveryRecommendation> {
    let mut recommendations =
        workflow_discovery_create_workflow_recommendations(spec_field_coverage_check);
    recommendations.extend(workflow_discovery_metadata_recommendations(
        repo_metadata,
        spec_field_coverage_check,
    ));
    recommendations.extend(workflow_discovery_assign_ownership_recommendation(
        ownership_escalation_check,
        spec_field_coverage_check,
    ));
    recommendations.extend(workflow_discovery_evidence_check_recommendation(
        governance_posture,
        spec_field_coverage_check,
    ));
    recommendations.push(workflow_discovery_side_effect_recommendation(
        governance_posture,
        spec_field_coverage_check,
    ));
    recommendations.push(workflow_discovery_report_handoff_recommendation(
        spec_field_coverage_check,
    ));
    recommendations
}

fn workflow_discovery_metadata_recommendations(
    repo_metadata: &SafeRepoMetadata,
    spec_field_coverage_check: &SpecFieldCoverageCheck,
) -> Vec<WorkflowDiscoveryRecommendation> {
    let mut recommendations = Vec::new();
    let package_metadata = repo_metadata.package_json.as_ref();
    if repo_metadata.typescript_detected() {
        recommendations.push(workflow_discovery_metadata_recommendation(
            "first_run.typescript_implementation",
            WorkflowDiscoveryRecommendationKind::CreateWorkflow,
            "typescript_implementation_workflow",
            &[
                "repo_metadata.package_json_present",
                "repo_metadata.typescript_detected",
            ],
            metadata_implementation_coverage_codes(),
            spec_field_coverage_check,
        ));
    }
    if package_metadata.is_some_and(|metadata| {
        metadata.has_script("test")
            || metadata.has_script("build")
            || metadata.has_script("lint")
            || metadata.has_script("typecheck")
    }) {
        recommendations.push(workflow_discovery_metadata_recommendation(
            "first_run.package_validation_obligations",
            WorkflowDiscoveryRecommendationKind::AddEvidenceCheckRequirements,
            "add_package_validation_obligations",
            &[
                "repo_metadata.package_json_present",
                "repo_metadata.common_scripts_detected",
            ],
            metadata_validation_coverage_codes(),
            spec_field_coverage_check,
        ));
    }
    if repo_metadata.rust_detected() {
        recommendations.extend(workflow_discovery_ecosystem_pair(
            "first_run.rust_implementation",
            "rust_implementation_workflow",
            "first_run.rust_validation_obligations",
            "add_rust_validation_obligations",
            "repo_metadata.cargo_toml_present",
            spec_field_coverage_check,
        ));
    }
    if repo_metadata.python_detected() {
        recommendations.extend(workflow_discovery_ecosystem_pair(
            "first_run.python_implementation",
            "python_implementation_workflow",
            "first_run.python_validation_obligations",
            "add_python_validation_obligations",
            "repo_metadata.pyproject_toml_present",
            spec_field_coverage_check,
        ));
    }
    if repo_metadata.go_detected() {
        recommendations.extend(workflow_discovery_ecosystem_pair(
            "first_run.go_implementation",
            "go_implementation_workflow",
            "first_run.go_validation_obligations",
            "add_go_validation_obligations",
            "repo_metadata.go_mod_present",
            spec_field_coverage_check,
        ));
    }
    if repo_metadata.github_actions_detected() {
        recommendations.push(workflow_discovery_metadata_recommendation(
            "first_run.github_actions_ci_evidence",
            WorkflowDiscoveryRecommendationKind::AddEvidenceCheckRequirements,
            "add_github_actions_ci_evidence_obligations",
            &["repo_metadata.github_actions_present"],
            &[
                "spec_field.workflow.audit_observability_disclosed",
                "spec_field.tests.not_automatically_executed",
            ],
            spec_field_coverage_check,
        ));
    }
    recommendations
}

fn workflow_discovery_ecosystem_pair(
    implementation_id: &'static str,
    implementation_summary: &'static str,
    validation_id: &'static str,
    validation_summary: &'static str,
    rationale_code: &'static str,
    spec_field_coverage_check: &SpecFieldCoverageCheck,
) -> [WorkflowDiscoveryRecommendation; 2] {
    [
        workflow_discovery_metadata_recommendation(
            implementation_id,
            WorkflowDiscoveryRecommendationKind::CreateWorkflow,
            implementation_summary,
            &[rationale_code],
            metadata_implementation_coverage_codes(),
            spec_field_coverage_check,
        ),
        workflow_discovery_metadata_recommendation(
            validation_id,
            WorkflowDiscoveryRecommendationKind::AddEvidenceCheckRequirements,
            validation_summary,
            &[rationale_code],
            metadata_validation_coverage_codes(),
            spec_field_coverage_check,
        ),
    ]
}

fn workflow_discovery_metadata_recommendation(
    id: &'static str,
    kind: WorkflowDiscoveryRecommendationKind,
    summary: &'static str,
    rationale_codes: &[&'static str],
    coverage_candidates: &[&'static str],
    spec_field_coverage_check: &SpecFieldCoverageCheck,
) -> WorkflowDiscoveryRecommendation {
    WorkflowDiscoveryRecommendation {
        id,
        kind,
        target: WorkflowDiscoveryRecommendationTarget::project(),
        status: WorkflowDiscoveryRecommendationStatus::ReviewOnly,
        summary,
        rationale_codes: rationale_codes.to_vec(),
        coverage_codes: matching_coverage_codes(spec_field_coverage_check, coverage_candidates),
        ownership_issue_codes: Vec::new(),
        next_action: workflow_discovery_next_action(kind),
    }
}

fn workflow_discovery_next_action(kind: WorkflowDiscoveryRecommendationKind) -> &'static str {
    match kind {
        WorkflowDiscoveryRecommendationKind::CreateWorkflow => "review_and_author_workflow_spec",
        WorkflowDiscoveryRecommendationKind::AssignOwnership => {
            "replace_placeholder_owner_and_escalation"
        }
        WorkflowDiscoveryRecommendationKind::AddEvidenceCheckRequirements => {
            "define_evidence_and_validation_obligations"
        }
        WorkflowDiscoveryRecommendationKind::AddSideEffectPosture => {
            "define_side_effect_posture_before_writes"
        }
        WorkflowDiscoveryRecommendationKind::AddReportHandoffObligations => {
            "define_report_and_handoff_obligations"
        }
    }
}

fn metadata_implementation_coverage_codes() -> &'static [&'static str] {
    &[
        "spec_field.workflow.steps_enforced_supported_local_paths",
        "spec_field.tests.not_automatically_executed",
    ]
}

fn metadata_validation_coverage_codes() -> &'static [&'static str] {
    &[
        "spec_field.test.identity_target_validated",
        "spec_field.tests.not_automatically_executed",
    ]
}

fn workflow_discovery_create_workflow_recommendations(
    spec_field_coverage_check: &SpecFieldCoverageCheck,
) -> Vec<WorkflowDiscoveryRecommendation> {
    vec![
        WorkflowDiscoveryRecommendation {
            id: "first_run.repo_implementation",
            kind: WorkflowDiscoveryRecommendationKind::CreateWorkflow,
            target: WorkflowDiscoveryRecommendationTarget::project(),
            status: WorkflowDiscoveryRecommendationStatus::ReviewOnly,
            summary: "repo_implementation_workflow",
            rationale_codes: vec![
                "first_run.report_ready_context",
                "governed_work_pattern.implementation_boundary",
            ],
            coverage_codes: matching_coverage_codes(
                spec_field_coverage_check,
                &[
                    "spec_field.workflow.steps_enforced_supported_local_paths",
                    "spec_field.workflow.policy_requirements_enforced_supported_local_paths",
                    "spec_field.workflow.audit_observability_disclosed",
                ],
            ),
            ownership_issue_codes: Vec::new(),
            next_action: workflow_discovery_next_action(
                WorkflowDiscoveryRecommendationKind::CreateWorkflow,
            ),
        },
        WorkflowDiscoveryRecommendation {
            id: "first_run.pr_review",
            kind: WorkflowDiscoveryRecommendationKind::CreateWorkflow,
            target: WorkflowDiscoveryRecommendationTarget::project(),
            status: WorkflowDiscoveryRecommendationStatus::ReviewOnly,
            summary: "pr_review_workflow",
            rationale_codes: vec![
                "first_run.merge_sensitive_work",
                "governed_work_pattern.review_boundary",
            ],
            coverage_codes: matching_coverage_codes(
                spec_field_coverage_check,
                &[
                    "spec_field.workflow.approval_requirements_enforced_supported_local_paths",
                    "spec_field.workflow.audit_observability_disclosed",
                ],
            ),
            ownership_issue_codes: Vec::new(),
            next_action: workflow_discovery_next_action(
                WorkflowDiscoveryRecommendationKind::CreateWorkflow,
            ),
        },
        WorkflowDiscoveryRecommendation {
            id: "first_run.release_readiness",
            kind: WorkflowDiscoveryRecommendationKind::CreateWorkflow,
            target: WorkflowDiscoveryRecommendationTarget::project(),
            status: WorkflowDiscoveryRecommendationStatus::ReviewOnly,
            summary: "release_readiness_workflow",
            rationale_codes: vec![
                "first_run.release_sensitive_work",
                "governed_work_pattern.report_closure",
            ],
            coverage_codes: matching_coverage_codes(
                spec_field_coverage_check,
                &[
                    "spec_field.test.identity_target_validated",
                    "spec_field.tests.not_automatically_executed",
                    "spec_field.workflow.audit_observability_disclosed",
                ],
            ),
            ownership_issue_codes: Vec::new(),
            next_action: workflow_discovery_next_action(
                WorkflowDiscoveryRecommendationKind::CreateWorkflow,
            ),
        },
    ]
}

fn workflow_discovery_assign_ownership_recommendation(
    ownership_escalation_check: &OwnershipEscalationCheck,
    spec_field_coverage_check: &SpecFieldCoverageCheck,
) -> Option<WorkflowDiscoveryRecommendation> {
    if ownership_escalation_check.issues.is_empty() {
        return None;
    }
    Some(WorkflowDiscoveryRecommendation {
        id: "first_run.assign_ownership",
        kind: WorkflowDiscoveryRecommendationKind::AssignOwnership,
        target: WorkflowDiscoveryRecommendationTarget::project(),
        status: WorkflowDiscoveryRecommendationStatus::NeedsHumanReview,
        summary: "assign_workflow_stewardship",
        rationale_codes: vec!["ownership_escalation.warnings_present"],
        coverage_codes: matching_coverage_codes(
            spec_field_coverage_check,
            &[
                "spec_field.workflow.owner_disclosed",
                "spec_field.skill.identity_validated",
            ],
        ),
        ownership_issue_codes: unique_ownership_issue_codes(ownership_escalation_check),
        next_action: workflow_discovery_next_action(
            WorkflowDiscoveryRecommendationKind::AssignOwnership,
        ),
    })
}

fn workflow_discovery_evidence_check_recommendation(
    governance_posture: &GovernanceFieldPosture,
    spec_field_coverage_check: &SpecFieldCoverageCheck,
) -> Option<WorkflowDiscoveryRecommendation> {
    if governance_posture.evidence != FieldPosture::NotAvailable
        && governance_posture.checks != FieldPosture::Skipped
    {
        return None;
    }
    Some(WorkflowDiscoveryRecommendation {
        id: "first_run.evidence_check_requirements",
        kind: WorkflowDiscoveryRecommendationKind::AddEvidenceCheckRequirements,
        target: WorkflowDiscoveryRecommendationTarget::project(),
        status: WorkflowDiscoveryRecommendationStatus::ReviewOnly,
        summary: "add_evidence_and_check_obligations",
        rationale_codes: vec!["field_evidence.not_available", "field_checks.skipped"],
        coverage_codes: matching_coverage_codes(
            spec_field_coverage_check,
            &[
                "spec_field.project.config_advisory",
                "spec_field.test.identity_target_validated",
                "spec_field.tests.not_automatically_executed",
            ],
        ),
        ownership_issue_codes: Vec::new(),
        next_action: workflow_discovery_next_action(
            WorkflowDiscoveryRecommendationKind::AddEvidenceCheckRequirements,
        ),
    })
}

fn workflow_discovery_side_effect_recommendation(
    governance_posture: &GovernanceFieldPosture,
    spec_field_coverage_check: &SpecFieldCoverageCheck,
) -> WorkflowDiscoveryRecommendation {
    let rationale = match governance_posture.side_effects {
        FieldPosture::DeclaredUnsupported => "field_side_effects.declared_unsupported",
        _ => "field_side_effects.none_skipped_unsupported",
    };
    WorkflowDiscoveryRecommendation {
        id: "first_run.side_effect_posture",
        kind: WorkflowDiscoveryRecommendationKind::AddSideEffectPosture,
        target: WorkflowDiscoveryRecommendationTarget::project(),
        status: WorkflowDiscoveryRecommendationStatus::ReviewOnly,
        summary: "add_side_effect_disclosure",
        rationale_codes: vec![rationale],
        coverage_codes: matching_coverage_codes(
            spec_field_coverage_check,
            &["spec_field.skill.capabilities_adapters_writes_deferred"],
        ),
        ownership_issue_codes: Vec::new(),
        next_action: workflow_discovery_next_action(
            WorkflowDiscoveryRecommendationKind::AddSideEffectPosture,
        ),
    }
}

fn workflow_discovery_report_handoff_recommendation(
    spec_field_coverage_check: &SpecFieldCoverageCheck,
) -> WorkflowDiscoveryRecommendation {
    WorkflowDiscoveryRecommendation {
        id: "first_run.report_handoff_obligations",
        kind: WorkflowDiscoveryRecommendationKind::AddReportHandoffObligations,
        target: WorkflowDiscoveryRecommendationTarget::project(),
        status: WorkflowDiscoveryRecommendationStatus::ReviewOnly,
        summary: "add_report_and_handoff_obligations",
        rationale_codes: vec!["first_run.report_ready_context"],
        coverage_codes: matching_coverage_codes(
            spec_field_coverage_check,
            &[
                "spec_field.workflow.audit_observability_disclosed",
                "spec_field.workflow.mappings_advisory",
            ],
        ),
        ownership_issue_codes: Vec::new(),
        next_action: workflow_discovery_next_action(
            WorkflowDiscoveryRecommendationKind::AddReportHandoffObligations,
        ),
    }
}

fn matching_coverage_codes(
    spec_field_coverage_check: &SpecFieldCoverageCheck,
    codes: &[&'static str],
) -> Vec<&'static str> {
    codes
        .iter()
        .copied()
        .filter(|code| {
            spec_field_coverage_check
                .items
                .iter()
                .any(|item| item.code == *code)
        })
        .collect()
}

fn unique_ownership_issue_codes(
    ownership_escalation_check: &OwnershipEscalationCheck,
) -> Vec<&'static str> {
    ownership_escalation_check
        .issues
        .iter()
        .map(|issue| issue.code.label())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn print_first_run_text(context: &FirstRunReportReadyContext, verbose: bool) {
    println!("Workflow OS first-run summary");
    println!("status: ready_for_review");
    println!("what_happened: validated a bounded governance envelope without starting a run");
    println!(
        "what_was_not_done: no workflow run, runtime state, artifacts, local checks, or external writes were created"
    );
    println!("what_matters_now:");
    println!("  - review the governance findings before treating the repo as configured");
    if context.repo_metadata.typescript_detected() {
        println!("  - detected TypeScript/package metadata can guide implementation and validation workflows");
    } else if context.repo_metadata.npm_package_present() {
        println!("  - detected package metadata can guide validation obligations");
    } else if context.repo_metadata.rust_detected() {
        println!("  - detected Rust metadata can guide implementation and validation workflows");
    } else if context.repo_metadata.python_detected() {
        println!("  - detected Python metadata can guide implementation and validation workflows");
    } else if context.repo_metadata.go_detected() {
        println!("  - detected Go metadata can guide implementation and validation workflows");
    } else if context.repo_metadata.github_actions_detected() {
        println!("  - detected GitHub Actions metadata can guide CI evidence obligations");
    } else {
        println!("  - assign ownership, escalation, evidence, and validation obligations");
    }
    println!("  - the mock first-run workflow is optional and demonstrates approval/audit mechanics only");
    println!(
        "recommended_next_action: review first-run findings and assign ownership/check obligations"
    );
    print_recommendation_next_actions(&context.recommendation_next_actions);
    println!("optional_approval_audit_demo: workflow-os --mock-all-local-skills run local/first-run-governance");
    println!(
        "optional_demo_note: mock skill run demonstrates approval and event history; it is not additional repository analysis"
    );
    println!("detail: run `workflow-os first-run --verbose` for the full posture matrix");
    if verbose {
        println!();
        print_first_run_verbose_text(context);
    }
}

fn print_first_run_verbose_text(context: &FirstRunReportReadyContext) {
    println!();
    println!("Detailed posture:");
    println!("first_run_report_ready: true");
    println!("mode: report_ready_context");
    println!("validation: passed");
    println!("scaffold: {}", presence_label(context.scaffold_present));
    println!("git_repository: {}", presence_label(context.git_present));
    println!(
        "spec_counts: workflows={} skills={} policies={} tests={}",
        context.workflow_count, context.skill_count, context.policy_count, context.test_count
    );
    print_safe_repo_metadata(&context.repo_metadata);
    println!("sections: {}", context.sections.len());
    for section in &context.sections {
        println!("section: {}", section_kind_label(section.kind()));
    }
    println!(
        "incomplete_work_disclosures: {}",
        context.incomplete_work.len()
    );
    println!("known_limitations: {}", context.known_limitations.len());
    println!("risks: {}", context.risks.len());
    println!("handoff_notes: {}", context.handoff_notes.len());
    println!("evidence: not_available");
    println!("checks: skipped");
    println!("side_effects: none_skipped_unsupported");
    println!("governance_profile: {}", context.governance_posture.profile);
    println!(
        "profile_posture: {}",
        context.governance_posture.profile_posture
    );
    println!(
        "ownership: {}",
        context.governance_posture.ownership.label()
    );
    println!(
        "escalation: {}",
        context.governance_posture.escalation.label()
    );
    println!(
        "approvals: {}",
        context.governance_posture.approvals.label()
    );
    println!(
        "policy_gates: {}",
        context.governance_posture.policy_gates.label()
    );
    println!(
        "field_evidence: {}",
        context.governance_posture.evidence.label()
    );
    println!(
        "field_checks: {}",
        context.governance_posture.checks.label()
    );
    println!(
        "field_side_effects: {}",
        context.governance_posture.side_effects.label()
    );
    println!(
        "audit_observability: {}",
        context.governance_posture.audit_observability.label()
    );
    println!("deferred_fields:");
    for field in context.governance_posture.deferred_fields {
        println!("  - {field}");
    }
    print_ownership_escalation_check(&context.ownership_escalation_check);
    print_spec_field_coverage_check(&context.spec_field_coverage_check);
    print_workflow_discovery_recommendations(&context.workflow_discovery_recommendations);
    print_recommendation_next_actions(&context.recommendation_next_actions);
    println!("recommendations:");
    for recommendation in &context.recommendations {
        println!("  - {recommendation}");
    }
    println!("next_step: workflow-os --mock-all-local-skills run local/first-run-governance");
}

fn print_recommendation_next_actions(actions: &[&'static str]) {
    println!("recommendation_next_actions:");
    for action in actions {
        println!("  - {action}");
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GovernedWorkflowDraftProposal {
    source_recommendation_id: &'static str,
    status: &'static str,
    proposed_lifecycle_status: &'static str,
    proposal_kind: &'static str,
    proposed_purpose_code: &'static str,
    required_authoring_decisions: Vec<&'static str>,
    validation_expectations: Vec<&'static str>,
    missing_required_fields: Vec<&'static str>,
    non_goals: Vec<&'static str>,
    privacy_boundary: &'static str,
}

fn governed_workflow_draft_proposal_from_recommendation(
    recommendation: &WorkflowDiscoveryRecommendation,
) -> Result<GovernedWorkflowDraftProposal, WorkflowOsError> {
    validate_authoring_recommendation_id(recommendation.id)?;
    Ok(GovernedWorkflowDraftProposal {
        source_recommendation_id: recommendation.id,
        status: "inactive_review_required",
        proposed_lifecycle_status: "draft",
        proposal_kind: draft_proposal_kind(recommendation.kind),
        proposed_purpose_code: recommendation.summary,
        required_authoring_decisions: draft_required_authoring_decisions(recommendation.kind),
        validation_expectations: draft_validation_expectations(recommendation.kind),
        missing_required_fields: draft_missing_required_fields(recommendation.kind),
        non_goals: draft_non_goals(recommendation.kind),
        privacy_boundary: "bounded_codes_only_no_raw_payloads",
    })
}

fn validate_authoring_recommendation_id(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() || value.len() > 96 {
        return Err(WorkflowOsError::validation(
            "cli.workflow_authoring.unsafe_payload_rejected",
            "recommendation cannot be used for governed workflow draft authoring",
        ));
    }
    if !value.starts_with("first_run.")
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
        || looks_secret_like(value)
    {
        return Err(WorkflowOsError::validation(
            "cli.workflow_authoring.unsafe_payload_rejected",
            "recommendation cannot be used for governed workflow draft authoring",
        ));
    }
    Ok(())
}

fn draft_proposal_kind(kind: WorkflowDiscoveryRecommendationKind) -> &'static str {
    match kind {
        WorkflowDiscoveryRecommendationKind::CreateWorkflow => "workflow_draft_proposal",
        WorkflowDiscoveryRecommendationKind::AssignOwnership => "ownership_update_proposal",
        WorkflowDiscoveryRecommendationKind::AddEvidenceCheckRequirements => {
            "evidence_check_obligation_proposal"
        }
        WorkflowDiscoveryRecommendationKind::AddSideEffectPosture => "side_effect_posture_proposal",
        WorkflowDiscoveryRecommendationKind::AddReportHandoffObligations => {
            "report_handoff_obligation_proposal"
        }
    }
}

fn draft_required_authoring_decisions(
    kind: WorkflowDiscoveryRecommendationKind,
) -> Vec<&'static str> {
    match kind {
        WorkflowDiscoveryRecommendationKind::CreateWorkflow => vec![
            "choose_workflow_id",
            "assign_owner",
            "assign_escalation_contact",
            "define_step_boundaries",
            "define_policy_gates",
            "define_evidence_and_check_obligations",
            "define_side_effect_posture",
            "define_report_handoff_posture",
        ],
        WorkflowDiscoveryRecommendationKind::AssignOwnership => vec![
            "assign_owner",
            "assign_escalation_contact",
            "define_authority_context",
            "review_local_vs_enterprise_stewardship",
        ],
        WorkflowDiscoveryRecommendationKind::AddEvidenceCheckRequirements => vec![
            "define_validation_obligations",
            "define_evidence_references",
            "define_check_failure_semantics",
            "review_command_execution_boundary",
        ],
        WorkflowDiscoveryRecommendationKind::AddSideEffectPosture => vec![
            "define_side_effect_posture",
            "document_unsupported_writes",
            "define_approval_before_write_boundary",
            "review_provider_mutation_boundary",
        ],
        WorkflowDiscoveryRecommendationKind::AddReportHandoffObligations => vec![
            "define_required_report_sections",
            "define_typed_handoff_obligations",
            "define_incomplete_work_disclosures",
            "define_closure_validation",
        ],
    }
}

fn draft_validation_expectations(kind: WorkflowDiscoveryRecommendationKind) -> Vec<&'static str> {
    let mut expectations = vec![
        "validate_project_after_authoring",
        "review_recommendation_detail_before_authoring",
        "confirm_proposal_remains_inactive",
    ];
    match kind {
        WorkflowDiscoveryRecommendationKind::CreateWorkflow => {
            expectations.push("check_workflow_id_conflicts_before_promotion");
            expectations.push("review_policy_evidence_checks_side_effects_and_report_posture");
        }
        WorkflowDiscoveryRecommendationKind::AssignOwnership => {
            expectations.push("review_owner_and_escalation_placeholders");
        }
        WorkflowDiscoveryRecommendationKind::AddEvidenceCheckRequirements => {
            expectations.push("review_checks_without_executing_commands");
        }
        WorkflowDiscoveryRecommendationKind::AddSideEffectPosture => {
            expectations.push("confirm_no_write_capability_enabled");
        }
        WorkflowDiscoveryRecommendationKind::AddReportHandoffObligations => {
            expectations.push("review_work_report_and_handoff_requirements");
        }
    }
    expectations
}

fn draft_missing_required_fields(kind: WorkflowDiscoveryRecommendationKind) -> Vec<&'static str> {
    match kind {
        WorkflowDiscoveryRecommendationKind::CreateWorkflow => vec![
            "workflow_id",
            "owner",
            "escalation",
            "steps",
            "policy_gates",
            "evidence_checks",
            "side_effects",
            "report_handoff",
        ],
        WorkflowDiscoveryRecommendationKind::AssignOwnership => {
            vec!["owner", "escalation", "authority_context"]
        }
        WorkflowDiscoveryRecommendationKind::AddEvidenceCheckRequirements => {
            vec![
                "validation_obligations",
                "evidence_references",
                "failure_semantics",
            ]
        }
        WorkflowDiscoveryRecommendationKind::AddSideEffectPosture => {
            vec!["side_effect_posture", "write_boundary", "approval_boundary"]
        }
        WorkflowDiscoveryRecommendationKind::AddReportHandoffObligations => {
            vec![
                "report_sections",
                "handoff_requirements",
                "closure_validation",
            ]
        }
    }
}

fn draft_non_goals(kind: WorkflowDiscoveryRecommendationKind) -> Vec<&'static str> {
    let mut non_goals = vec![
        "no_file_written",
        "no_workflow_registered",
        "no_command_executed",
        "no_provider_call",
        "no_runtime_state_created",
    ];
    match kind {
        WorkflowDiscoveryRecommendationKind::CreateWorkflow => {
            non_goals.push("no_active_workflow_created");
        }
        WorkflowDiscoveryRecommendationKind::AssignOwnership => {
            non_goals.push("no_rbac_no_idp_no_paging");
        }
        WorkflowDiscoveryRecommendationKind::AddEvidenceCheckRequirements => {
            non_goals.push("no_check_registered_no_evidence_fabricated");
        }
        WorkflowDiscoveryRecommendationKind::AddSideEffectPosture => {
            non_goals.push("no_write_enabled_no_side_effect_executed");
        }
        WorkflowDiscoveryRecommendationKind::AddReportHandoffObligations => {
            non_goals.push("no_report_artifact_written_no_handoff_sent");
        }
    }
    non_goals
}

fn looks_secret_like(value: &str) -> bool {
    let lowercase = value.to_ascii_lowercase();
    lowercase.contains("authorization")
        || lowercase.contains("bearer")
        || lowercase.contains("credential")
        || lowercase.contains("password")
        || lowercase.contains("private_key")
        || lowercase.contains("api_token")
        || lowercase.contains("api-token")
        || lowercase.contains("secret")
        || lowercase.contains("token")
}

fn print_first_run_recommendation_detail(
    recommendation: &WorkflowDiscoveryRecommendation,
    draft_proposal: &GovernedWorkflowDraftProposal,
) {
    println!("Workflow OS first-run recommendation detail");
    println!("id: {}", recommendation.id);
    println!("kind: {}", recommendation.kind.label());
    println!(
        "target: {}#{}",
        recommendation.target.surface.label(),
        recommendation.target.ordinal
    );
    println!("status: {}", recommendation.status.label());
    println!("review_posture: review_only_not_active_workflow");
    println!("summary: {}", recommendation.summary);
    println!(
        "rationale: {}",
        joined_codes(&recommendation.rationale_codes)
    );
    println!(
        "metadata_signals: {}",
        joined_codes(&metadata_signal_codes(recommendation))
    );
    println!("coverage: {}", joined_codes(&recommendation.coverage_codes));
    println!(
        "ownership: {}",
        joined_codes(&recommendation.ownership_issue_codes)
    );
    println!("next_action: {}", recommendation.next_action);
    println!("draft_proposal_status: {}", draft_proposal.status);
    println!("draft_proposal_kind: {}", draft_proposal.proposal_kind);
    println!(
        "proposed_lifecycle_status: {}",
        draft_proposal.proposed_lifecycle_status
    );
    println!(
        "required_authoring_decisions: {}",
        joined_codes(&draft_proposal.required_authoring_decisions)
    );
    println!(
        "validation_expectations: {}",
        joined_codes(&draft_proposal.validation_expectations)
    );
    println!(
        "missing_required_fields: {}",
        joined_codes(&draft_proposal.missing_required_fields)
    );
    println!(
        "authoring_required: {}",
        recommendation_authoring_required(recommendation.kind)
    );
    println!(
        "what_workflow_os_did_not_do: {}",
        recommendation_non_execution_boundary(recommendation.kind)
    );
    println!(
        "draft_non_goals: {}",
        joined_codes(&draft_proposal.non_goals)
    );
    println!("privacy_boundary: {}", draft_proposal.privacy_boundary);
}

fn print_author_workflow_dry_run(
    recommendation: &WorkflowDiscoveryRecommendation,
    draft_proposal: &GovernedWorkflowDraftProposal,
) {
    println!("Workflow OS governed workflow authoring dry-run");
    println!("mode: author_workflow_dry_run");
    println!("status: preview_only");
    println!("source_recommendation_id: {}", recommendation.id);
    println!(
        "source_recommendation_kind: {}",
        recommendation.kind.label()
    );
    println!(
        "source_target: {}#{}",
        recommendation.target.surface.label(),
        recommendation.target.ordinal
    );
    println!("source_summary: {}", recommendation.summary);
    println!("draft_proposal_status: {}", draft_proposal.status);
    println!("draft_proposal_kind: {}", draft_proposal.proposal_kind);
    println!(
        "proposed_lifecycle_status: {}",
        draft_proposal.proposed_lifecycle_status
    );
    println!(
        "proposed_purpose_code: {}",
        draft_proposal.proposed_purpose_code
    );
    println!(
        "required_authoring_decisions: {}",
        joined_codes(&draft_proposal.required_authoring_decisions)
    );
    println!(
        "validation_expectations: {}",
        joined_codes(&draft_proposal.validation_expectations)
    );
    println!(
        "missing_required_fields: {}",
        joined_codes(&draft_proposal.missing_required_fields)
    );
    println!(
        "authoring_required: {}",
        recommendation_authoring_required(recommendation.kind)
    );
    println!("non_mutation:");
    println!("  no_files_written: true");
    println!("  no_workflow_registered: true");
    println!("  no_workflow_promoted: true");
    println!("  no_commands_executed: true");
    println!("  no_providers_called: true");
    println!("  no_runtime_state_created: true");
    println!(
        "draft_non_goals: {}",
        joined_codes(&draft_proposal.non_goals)
    );
    println!("privacy_boundary: {}", draft_proposal.privacy_boundary);
    println!("next_action: review_this_preview_fill_required_authoring_decisions_then_validate_before_promotion");
}

fn author_workflow_dry_run_json(
    recommendation: &WorkflowDiscoveryRecommendation,
    draft_proposal: &GovernedWorkflowDraftProposal,
) -> String {
    format!(
        "{{\"author_workflow_dry_run\":{{\"schema_version\":\"workflowos.dev/v0\",\"mode\":\"author_workflow_dry_run\",\"status\":\"preview_only\",\"proposal\":{{\"source_recommendation_id\":\"{}\",\"source_recommendation_kind\":\"{}\",\"source_target\":{{\"surface\":\"{}\",\"ordinal\":{}}},\"source_summary\":\"{}\",\"draft_proposal_status\":\"{}\",\"draft_proposal_kind\":\"{}\",\"proposed_lifecycle_status\":\"{}\",\"proposed_purpose_code\":\"{}\",\"required_authoring_decisions\":{},\"validation_expectations\":{},\"missing_required_fields\":{},\"non_goals\":{},\"privacy_boundary\":\"{}\"}},\"non_mutation\":{{\"files_written\":false,\"workflow_registered\":false,\"workflow_promoted\":false,\"commands_executed\":false,\"providers_called\":false,\"runtime_state_created\":false}},\"next_action\":\"review_this_preview_fill_required_authoring_decisions_then_validate_before_promotion\"}}}}",
        json_escape(recommendation.id),
        recommendation.kind.label(),
        recommendation.target.surface.label(),
        recommendation.target.ordinal,
        json_escape(recommendation.summary),
        draft_proposal.status,
        draft_proposal.proposal_kind,
        draft_proposal.proposed_lifecycle_status,
        json_escape(draft_proposal.proposed_purpose_code),
        json_string_array(&draft_proposal.required_authoring_decisions),
        json_string_array(&draft_proposal.validation_expectations),
        json_string_array(&draft_proposal.missing_required_fields),
        json_string_array(&draft_proposal.non_goals),
        draft_proposal.privacy_boundary,
    )
}

fn validate_author_workflow_output_path(output: &Path) -> Result<PathBuf, WorkflowOsError> {
    let components = output.components().collect::<Vec<_>>();
    let [Component::Normal(workflows), Component::Normal(drafts), Component::Normal(file)] =
        components.as_slice()
    else {
        return Err(WorkflowOsError::validation(
            "cli.workflow_authoring.output_path_rejected",
            "workflow authoring output path must be workflows/drafts/<name>.workflow.yml",
        ));
    };
    if workflows.to_str() != Some("workflows") || drafts.to_str() != Some("drafts") {
        return Err(WorkflowOsError::validation(
            "cli.workflow_authoring.output_path_rejected",
            "workflow authoring output path must be workflows/drafts/<name>.workflow.yml",
        ));
    }
    let file = file.to_str().ok_or_else(|| {
        WorkflowOsError::validation(
            "cli.workflow_authoring.output_path_rejected",
            "workflow authoring output path must be valid UTF-8",
        )
    })?;
    if !file.ends_with(".workflow.yml") || file == ".workflow.yml" {
        return Err(WorkflowOsError::validation(
            "cli.workflow_authoring.output_path_rejected",
            "workflow authoring output path must end with .workflow.yml",
        ));
    }
    if !file
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
        || looks_secret_like(file)
    {
        return Err(WorkflowOsError::validation(
            "cli.workflow_authoring.output_path_rejected",
            "workflow authoring output path was rejected",
        ));
    }
    Ok(PathBuf::from("workflows").join("drafts").join(file))
}

fn proposed_workflow_id_from_output(output: &Path) -> Result<WorkflowId, WorkflowOsError> {
    let file_name = output
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            WorkflowOsError::validation(
                "cli.workflow_authoring.workflow_id_invalid",
                "workflow authoring output file name was rejected",
            )
        })?;
    let stem = file_name.strip_suffix(".workflow.yml").ok_or_else(|| {
        WorkflowOsError::validation(
            "cli.workflow_authoring.workflow_id_invalid",
            "workflow authoring output file name was rejected",
        )
    })?;
    if stem.is_empty() || looks_secret_like(stem) {
        return Err(WorkflowOsError::validation(
            "cli.workflow_authoring.workflow_id_invalid",
            "workflow authoring draft workflow id was rejected",
        ));
    }
    WorkflowId::new(format!("draft/{stem}")).map_err(|_| {
        WorkflowOsError::validation(
            "cli.workflow_authoring.workflow_id_invalid",
            "workflow authoring draft workflow id was rejected",
        )
    })
}

fn ensure_no_workflow_id_conflict(
    bundle: &workflow_core::ProjectBundle,
    proposed_workflow_id: &WorkflowId,
) -> Result<(), WorkflowOsError> {
    if bundle
        .workflows
        .iter()
        .any(|workflow| workflow.definition.id == *proposed_workflow_id)
    {
        return Err(WorkflowOsError::validation(
            "cli.workflow_authoring.workflow_id_conflict",
            "workflow authoring draft workflow id conflicts with an existing workflow",
        ));
    }
    Ok(())
}

fn write_author_workflow_draft(
    invocation: &Invocation,
    recommendation: &WorkflowDiscoveryRecommendation,
    draft_proposal: &GovernedWorkflowDraftProposal,
    output: &Path,
    proposed_workflow_id: &WorkflowId,
) -> Result<(), WorkflowOsError> {
    let absolute = invocation.project_dir.join(output);
    if absolute.exists() {
        return Err(WorkflowOsError::validation(
            "cli.workflow_authoring.output_exists",
            "workflow authoring output file already exists",
        ));
    }
    if let Some(parent) = absolute.parent() {
        fs::create_dir_all(parent).map_err(|_| {
            WorkflowOsError::invalid_state(
                "cli.workflow_authoring.draft_write_failed",
                "workflow authoring draft file could not be written",
            )
        })?;
    }
    let content =
        render_author_workflow_draft(recommendation, draft_proposal, proposed_workflow_id);
    fs::write(&absolute, content).map_err(|_| {
        WorkflowOsError::invalid_state(
            "cli.workflow_authoring.draft_write_failed",
            "workflow authoring draft file could not be written",
        )
    })
}

fn render_author_workflow_draft(
    recommendation: &WorkflowDiscoveryRecommendation,
    draft_proposal: &GovernedWorkflowDraftProposal,
    proposed_workflow_id: &WorkflowId,
) -> String {
    let display_name = title_from_code(draft_proposal.proposed_purpose_code);
    let mut output = String::new();
    output.push_str("# Workflow OS inactive draft. Review and complete before promotion.\n");
    let _ = writeln!(output, "# source_recommendation_id: {}", recommendation.id);
    let _ = writeln!(
        output,
        "# required_authoring_decisions: {}",
        joined_codes(&draft_proposal.required_authoring_decisions)
    );
    let _ = writeln!(
        output,
        "# validation_expectations: {}",
        joined_codes(&draft_proposal.validation_expectations)
    );
    let _ = writeln!(
        output,
        "# missing_required_fields: {}",
        joined_codes(&draft_proposal.missing_required_fields)
    );
    let _ = writeln!(
        output,
        "# non_goals: {}",
        joined_codes(&draft_proposal.non_goals)
    );
    output.push_str("schema_version: workflowos.dev/v0\n");
    let _ = writeln!(output, "id: {proposed_workflow_id}");
    output.push_str("version: v0\n");
    let _ = writeln!(output, "display_name: {display_name}");
    output.push_str("description: Inactive draft generated from a bounded Workflow OS first-run recommendation. Complete required fields before promotion.\n");
    output.push_str("owner:\n");
    output.push_str("  lifecycle_status: experimental\n");
    output.push_str("autonomy_level: level_1\n");
    output.push_str("disabled_by_default: true\n");
    output.push_str("triggers: []\n");
    output.push_str("steps: []\n");
    output.push_str("cancellation_behavior: stop\n");
    output.push_str("audit_requirements:\n");
    output.push_str("  required: true\n");
    output.push_str("  store_references_only: true\n");
    output.push_str("observability_requirements:\n");
    output.push_str("  tracing: true\n");
    output.push_str("  latency_tracking: true\n");
    output.push_str("tags:\n");
    output.push_str("  - workflow-os-draft\n");
    output.push_str("  - inactive\n");
    output.push_str("  - source-first-run-recommendation\n");
    output
}

fn title_from_code(code: &str) -> String {
    let mut title = String::from("Draft ");
    for (index, part) in code.split('_').filter(|part| !part.is_empty()).enumerate() {
        if index > 0 {
            title.push(' ');
        }
        let mut chars = part.chars();
        if let Some(first) = chars.next() {
            title.push(first.to_ascii_uppercase());
            title.push_str(chars.as_str());
        }
    }
    title
}

fn print_author_workflow_file_output_preview(
    recommendation: &WorkflowDiscoveryRecommendation,
    draft_proposal: &GovernedWorkflowDraftProposal,
    output: &Path,
    proposed_workflow_id: &WorkflowId,
) {
    println!("Workflow OS governed workflow authoring file-output dry-run");
    println!("mode: author_workflow_file_output_dry_run");
    println!("status: preview_only");
    println!("source_recommendation_id: {}", recommendation.id);
    println!("proposed_workflow_id: {proposed_workflow_id}");
    println!("output_path: {}", output.display());
    println!("draft_inactive: true");
    println!("draft_loaded_by_current_project_loader: false");
    println!("files_written: false");
    println!("workflow_registered: false");
    println!("workflow_promoted: false");
    println!("commands_executed: false");
    println!("providers_called: false");
    println!("runtime_state_created: false");
    println!(
        "required_authoring_decisions: {}",
        joined_codes(&draft_proposal.required_authoring_decisions)
    );
    println!(
        "missing_required_fields: {}",
        joined_codes(&draft_proposal.missing_required_fields)
    );
    println!("privacy_boundary: bounded_codes_only_no_raw_payloads");
    println!("next_action: rerun_without_dry_run_to_write_inactive_draft_file_for_review");
}

fn print_author_workflow_file_output_result(
    recommendation: &WorkflowDiscoveryRecommendation,
    output: &Path,
    proposed_workflow_id: &WorkflowId,
) {
    println!("Workflow OS governed workflow authoring file output");
    println!("mode: author_workflow_file_output");
    println!("status: inactive_draft_written");
    println!("source_recommendation_id: {}", recommendation.id);
    println!("proposed_workflow_id: {proposed_workflow_id}");
    println!("output_path: {}", output.display());
    println!("draft_inactive: true");
    println!("draft_loaded_by_current_project_loader: false");
    println!("workflow_registered: false");
    println!("workflow_promoted: false");
    println!("commands_executed: false");
    println!("providers_called: false");
    println!("runtime_state_created: false");
    println!(
        "next_action: review_complete_required_fields_validate_then_plan_promotion_separately"
    );
}

fn print_author_workflow_preflight_result(
    draft: &Path,
    candidate_workflow_id: &WorkflowId,
    status: &str,
    blockers: &[String],
    warnings: &[String],
) {
    println!("Workflow OS governed workflow authoring promotion preflight");
    println!("mode: author_workflow_promotion_preflight");
    println!("status: {status}");
    println!("draft_path: {}", draft.display());
    println!("candidate_workflow_id: {candidate_workflow_id}");
    println!("blockers: {}", joined_dynamic_codes(blockers));
    println!("warnings: {}", joined_dynamic_codes(warnings));
    println!("files_written: false");
    println!("workflow_registered: false");
    println!("workflow_promoted: false");
    println!("commands_executed: false");
    println!("providers_called: false");
    println!("runtime_state_created: false");
    println!("privacy_boundary: bounded_codes_only_no_raw_payloads");
    if blockers.is_empty() {
        println!("next_action: steward_review_required_before_any_future_active_promotion");
    } else {
        println!("next_action: resolve_preflight_blockers_then_rerun_preflight");
    }
}

fn author_workflow_file_output_preview_json(
    recommendation: &WorkflowDiscoveryRecommendation,
    draft_proposal: &GovernedWorkflowDraftProposal,
    output: &Path,
    proposed_workflow_id: &WorkflowId,
) -> String {
    format!(
        "{{\"author_workflow_file_output_dry_run\":{{\"schema_version\":\"workflowos.dev/v0\",\"mode\":\"author_workflow_file_output_dry_run\",\"status\":\"preview_only\",\"source_recommendation_id\":\"{}\",\"proposed_workflow_id\":\"{}\",\"output_path\":\"{}\",\"draft_inactive\":true,\"draft_loaded_by_current_project_loader\":false,\"non_mutation\":{{\"files_written\":false,\"workflow_registered\":false,\"workflow_promoted\":false,\"commands_executed\":false,\"providers_called\":false,\"runtime_state_created\":false}},\"required_authoring_decisions\":{},\"missing_required_fields\":{},\"privacy_boundary\":\"bounded_codes_only_no_raw_payloads\",\"next_action\":\"rerun_without_dry_run_to_write_inactive_draft_file_for_review\"}}}}",
        json_escape(recommendation.id),
        json_escape(proposed_workflow_id.as_str()),
        json_escape(&output.display().to_string()),
        json_string_array(&draft_proposal.required_authoring_decisions),
        json_string_array(&draft_proposal.missing_required_fields),
    )
}

fn author_workflow_preflight_json(
    draft: &Path,
    candidate_workflow_id: &WorkflowId,
    status: &str,
    blockers: &[String],
    warnings: &[String],
    validation_error_codes: &[String],
) -> String {
    format!(
        "{{\"author_workflow_promotion_preflight\":{{\"schema_version\":\"workflowos.dev/v0\",\"mode\":\"author_workflow_promotion_preflight\",\"status\":\"{}\",\"draft_path\":\"{}\",\"candidate_workflow_id\":\"{}\",\"blockers\":{},\"warnings\":{},\"validation_error_codes\":{},\"non_mutation\":{{\"files_written\":false,\"workflow_registered\":false,\"workflow_promoted\":false,\"commands_executed\":false,\"providers_called\":false,\"runtime_state_created\":false}},\"privacy_boundary\":\"bounded_codes_only_no_raw_payloads\",\"next_action\":\"{}\"}}}}",
        status,
        json_escape(&draft.display().to_string()),
        json_escape(candidate_workflow_id.as_str()),
        json_string_array_dynamic(blockers),
        json_string_array_dynamic(warnings),
        json_string_array_dynamic(validation_error_codes),
        if blockers.is_empty() {
            "steward_review_required_before_any_future_active_promotion"
        } else {
            "resolve_preflight_blockers_then_rerun_preflight"
        }
    )
}

fn print_author_workflow_draft_status(status: &AuthorWorkflowDraftStatus) {
    println!("Workflow OS governed workflow authoring draft status");
    println!("mode: author_workflow_draft_status");
    println!("status: {}", status.inferred_draft_state);
    println!("draft_path: {}", status.draft_path.display());
    println!(
        "active_workflow_path: {}",
        status.active_workflow_path.display()
    );
    println!("candidate_workflow_id: {}", status.candidate_workflow_id);
    println!("draft_content_hash: {}", status.draft_content_hash);
    println!(
        "matching_active_workflow_path: {}",
        status.matching_active_workflow_path.as_ref().map_or_else(
            || "not_available".to_owned(),
            |path| path.display().to_string()
        )
    );
    println!(
        "active_workflow_id_conflict_status: {}",
        status.active_workflow_id_conflict_status
    );
    print_author_workflow_draft_status_boundary();
    println!("privacy_boundary: bounded_codes_only_no_raw_payloads");
    println!("next_action: {}", status.recommended_next_action);
}

fn print_author_workflow_draft_status_boundary() {
    println!("files_written: false");
    println!("active_workflow_file_written: false");
    println!("draft_moved: false");
    println!("draft_deleted: false");
    println!("draft_archived: false");
    println!("workflow_registered: false");
    println!("workflow_promoted: false");
    println!("approval_persisted: false");
    println!("commands_executed: false");
    println!("providers_called: false");
    println!("runtime_state_created: false");
    println!("report_artifact_written: false");
}

fn author_workflow_draft_status_json(status: &AuthorWorkflowDraftStatus) -> String {
    format!(
        "{{\"author_workflow_draft_status\":{{\"schema_version\":\"workflowos.dev/v0\",\"mode\":\"author_workflow_draft_status\",\"status\":\"{}\",\"draft_path\":\"{}\",\"active_workflow_path\":\"{}\",\"candidate_workflow_id\":\"{}\",\"draft_content_hash\":\"{}\",\"matching_active_workflow_path\":\"{}\",\"active_workflow_id_conflict_status\":\"{}\",\"boundary\":{{\"files_written\":false,\"active_workflow_file_written\":false,\"draft_moved\":false,\"draft_deleted\":false,\"draft_archived\":false,\"workflow_registered\":false,\"workflow_promoted\":false,\"approval_persisted\":false,\"commands_executed\":false,\"providers_called\":false,\"runtime_state_created\":false,\"report_artifact_written\":false}},\"privacy_boundary\":\"bounded_codes_only_no_raw_payloads\",\"next_action\":\"{}\"}}}}",
        status.inferred_draft_state,
        json_escape(&status.draft_path.display().to_string()),
        json_escape(&status.active_workflow_path.display().to_string()),
        json_escape(status.candidate_workflow_id.as_str()),
        json_escape(&status.draft_content_hash.to_string()),
        json_escape(
            &status
                .matching_active_workflow_path
                .as_ref()
                .map_or_else(|| "not_available".to_owned(), |path| path.display().to_string())
        ),
        json_escape(status.active_workflow_id_conflict_status),
        json_escape(status.recommended_next_action),
    )
}

fn emit_author_workflow_archive_draft_result(
    invocation: &Invocation,
    draft_status: &AuthorWorkflowDraftStatus,
    archive_path: &Path,
    reviewer: &ActorId,
    status: &'static str,
    archived: bool,
) {
    if invocation.json {
        println!(
            "{}",
            author_workflow_archive_draft_json(
                draft_status,
                archive_path,
                reviewer,
                status,
                archived,
            )
        );
    } else {
        print_author_workflow_archive_draft_result(
            draft_status,
            archive_path,
            reviewer,
            status,
            archived,
        );
    }
}

fn print_author_workflow_archive_draft_result(
    draft_status: &AuthorWorkflowDraftStatus,
    archive_path: &Path,
    reviewer: &ActorId,
    status: &'static str,
    archived: bool,
) {
    println!("Workflow OS governed workflow authoring draft archive");
    println!("mode: author_workflow_draft_archive");
    println!("status: {status}");
    println!("prior_draft_status: {}", draft_status.inferred_draft_state);
    println!("draft_path: {}", draft_status.draft_path.display());
    println!("archive_path: {}", archive_path.display());
    println!(
        "active_workflow_path: {}",
        draft_status.active_workflow_path.display()
    );
    println!(
        "candidate_workflow_id: {}",
        draft_status.candidate_workflow_id
    );
    println!("draft_content_hash: {}", draft_status.draft_content_hash);
    println!(
        "matching_active_workflow_path: {}",
        draft_status
            .matching_active_workflow_path
            .as_ref()
            .map_or_else(
                || "not_available".to_owned(),
                |path| path.display().to_string()
            )
    );
    println!(
        "active_workflow_id_conflict_status: {}",
        draft_status.active_workflow_id_conflict_status
    );
    println!("reviewer: {reviewer}");
    println!("reason_status: provided");
    println!("files_written: {archived}");
    println!("active_workflow_file_written: false");
    println!("draft_moved: {archived}");
    println!("draft_deleted: false");
    println!("draft_archived: {archived}");
    println!("workflow_registered: false");
    println!("workflow_promoted: false");
    println!("approval_persisted: false");
    println!("commands_executed: false");
    println!("providers_called: false");
    println!("runtime_state_created: false");
    println!("report_artifact_written: false");
    println!("privacy_boundary: bounded_codes_only_no_raw_payloads");
    println!(
        "next_action: {}",
        author_workflow_archive_draft_next_action(status, archived)
    );
}

fn author_workflow_archive_draft_json(
    draft_status: &AuthorWorkflowDraftStatus,
    archive_path: &Path,
    reviewer: &ActorId,
    status: &'static str,
    archived: bool,
) -> String {
    format!(
        "{{\"author_workflow_draft_archive\":{{\"schema_version\":\"workflowos.dev/v0\",\"mode\":\"author_workflow_draft_archive\",\"status\":\"{}\",\"prior_draft_status\":\"{}\",\"draft_path\":\"{}\",\"archive_path\":\"{}\",\"active_workflow_path\":\"{}\",\"candidate_workflow_id\":\"{}\",\"draft_content_hash\":\"{}\",\"matching_active_workflow_path\":\"{}\",\"active_workflow_id_conflict_status\":\"{}\",\"reviewer\":\"{}\",\"reason_status\":\"provided\",\"boundary\":{{\"files_written\":{},\"active_workflow_file_written\":false,\"draft_moved\":{},\"draft_deleted\":false,\"draft_archived\":{},\"workflow_registered\":false,\"workflow_promoted\":false,\"approval_persisted\":false,\"commands_executed\":false,\"providers_called\":false,\"runtime_state_created\":false,\"report_artifact_written\":false}},\"privacy_boundary\":\"bounded_codes_only_no_raw_payloads\",\"next_action\":\"{}\"}}}}",
        status,
        json_escape(draft_status.inferred_draft_state),
        json_escape(&draft_status.draft_path.display().to_string()),
        json_escape(&archive_path.display().to_string()),
        json_escape(&draft_status.active_workflow_path.display().to_string()),
        json_escape(draft_status.candidate_workflow_id.as_str()),
        json_escape(&draft_status.draft_content_hash.to_string()),
        json_escape(
            &draft_status
                .matching_active_workflow_path
                .as_ref()
                .map_or_else(|| "not_available".to_owned(), |path| path.display().to_string())
        ),
        json_escape(draft_status.active_workflow_id_conflict_status),
        json_escape(reviewer.as_str()),
        archived,
        archived,
        archived,
        json_escape(author_workflow_archive_draft_next_action(status, archived)),
    )
}

fn author_workflow_archive_draft_next_action(status: &'static str, archived: bool) -> &'static str {
    if archived {
        "run_validate_to_confirm_project_remains_valid"
    } else if status == "archive_dry_run" {
        "rerun_without_dry_run_to_archive_eligible_draft"
    } else {
        "inspect_draft_status_before_archive"
    }
}

fn print_author_workflow_steward_review_blocked(
    draft: &Path,
    candidate_workflow_id: &WorkflowId,
    blockers: &[String],
    warnings: &[String],
) {
    println!("Workflow OS governed workflow authoring steward review preview");
    println!("mode: author_workflow_steward_review_preview");
    println!("status: review_blocked");
    println!("draft_path: {}", draft.display());
    println!("candidate_workflow_id: {candidate_workflow_id}");
    println!("preflight_status: blocked");
    println!("blockers: {}", joined_dynamic_codes(blockers));
    println!("warnings: {}", joined_dynamic_codes(warnings));
    print_author_workflow_steward_review_non_mutation();
    println!("privacy_boundary: bounded_codes_only_no_raw_payloads");
    println!("next_action: resolve_preflight_blockers_then_rerun_steward_review");
}

fn print_author_workflow_steward_review_result(review: &WorkflowDraftStewardReviewResult) {
    let card = review.card();
    println!("Workflow OS governed workflow authoring steward review preview");
    println!("mode: author_workflow_steward_review_preview");
    println!(
        "status: {}",
        steward_review_status_label(review.authorization())
    );
    println!("draft_path: {}", card.draft_path());
    println!("candidate_workflow_id: {}", card.candidate_workflow_id());
    println!("draft_content_hash: {}", card.draft_content_hash());
    println!(
        "preflight_status: {}",
        preflight_status_label(card.preflight_status())
    );
    println!(
        "warnings: {}",
        joined_dynamic_codes(card.preflight_warnings())
    );
    println!(
        "decision: {}",
        steward_review_decision_label(review.decision())
    );
    println!("reviewer: {}", review.reviewer());
    println!("owner_summary: {}", card.owner_summary());
    println!("escalation_summary: {}", card.escalation_summary());
    println!("policy_summary: {}", card.policy_summary());
    println!(
        "evidence_report_summary: {}",
        card.evidence_report_summary()
    );
    println!("side_effect_summary: {}", card.side_effect_summary());
    println!("approval_allows: {}", card.approval_allows());
    println!(
        "approval_does_not_allow: {}",
        card.approval_does_not_allow()
    );
    print_author_workflow_steward_review_non_mutation();
    println!("privacy_boundary: bounded_codes_only_no_raw_payloads");
    println!("next_action: {}", card.next_action());
}

fn print_author_workflow_steward_review_non_mutation() {
    println!("files_written: false");
    println!("workflow_registered: false");
    println!("workflow_promoted: false");
    println!("approval_persisted: false");
    println!("commands_executed: false");
    println!("providers_called: false");
    println!("runtime_state_created: false");
}

fn author_workflow_steward_review_blocked_json(
    draft: &Path,
    candidate_workflow_id: &WorkflowId,
    blockers: &[String],
    warnings: &[String],
    validation_error_codes: &[String],
) -> String {
    format!(
        "{{\"author_workflow_steward_review_preview\":{{\"schema_version\":\"workflowos.dev/v0\",\"mode\":\"author_workflow_steward_review_preview\",\"status\":\"review_blocked\",\"draft_path\":\"{}\",\"candidate_workflow_id\":\"{}\",\"preflight_status\":\"blocked\",\"blockers\":{},\"warnings\":{},\"validation_error_codes\":{},\"non_mutation\":{{\"files_written\":false,\"workflow_registered\":false,\"workflow_promoted\":false,\"approval_persisted\":false,\"commands_executed\":false,\"providers_called\":false,\"runtime_state_created\":false}},\"privacy_boundary\":\"bounded_codes_only_no_raw_payloads\",\"next_action\":\"resolve_preflight_blockers_then_rerun_steward_review\"}}}}",
        json_escape(&draft.display().to_string()),
        json_escape(candidate_workflow_id.as_str()),
        json_string_array_dynamic(blockers),
        json_string_array_dynamic(warnings),
        json_string_array_dynamic(validation_error_codes),
    )
}

fn author_workflow_steward_review_json(review: &WorkflowDraftStewardReviewResult) -> String {
    let card = review.card();
    format!(
        "{{\"author_workflow_steward_review_preview\":{{\"schema_version\":\"workflowos.dev/v0\",\"mode\":\"author_workflow_steward_review_preview\",\"status\":\"{}\",\"draft_path\":\"{}\",\"candidate_workflow_id\":\"{}\",\"draft_content_hash\":\"{}\",\"preflight_status\":\"{}\",\"warnings\":{},\"decision\":\"{}\",\"reviewer\":\"{}\",\"owner_summary\":\"{}\",\"escalation_summary\":\"{}\",\"policy_summary\":\"{}\",\"evidence_report_summary\":\"{}\",\"side_effect_summary\":\"{}\",\"approval_allows\":\"{}\",\"approval_does_not_allow\":\"{}\",\"non_mutation\":{{\"files_written\":false,\"workflow_registered\":false,\"workflow_promoted\":false,\"approval_persisted\":false,\"commands_executed\":false,\"providers_called\":false,\"runtime_state_created\":false}},\"privacy_boundary\":\"bounded_codes_only_no_raw_payloads\",\"next_action\":\"{}\"}}}}",
        steward_review_status_label(review.authorization()),
        json_escape(card.draft_path()),
        json_escape(card.candidate_workflow_id().as_str()),
        json_escape(&card.draft_content_hash().to_string()),
        preflight_status_label(card.preflight_status()),
        json_string_array_dynamic(card.preflight_warnings()),
        steward_review_decision_label(review.decision()),
        json_escape(review.reviewer().as_str()),
        json_escape(card.owner_summary()),
        json_escape(card.escalation_summary()),
        json_escape(card.policy_summary()),
        json_escape(card.evidence_report_summary()),
        json_escape(card.side_effect_summary()),
        json_escape(card.approval_allows()),
        json_escape(card.approval_does_not_allow()),
        json_escape(card.next_action()),
    )
}

fn print_author_workflow_active_promotion_blocked(
    draft: &Path,
    active_path: &Path,
    candidate_workflow_id: &WorkflowId,
    blockers: &[String],
    warnings: &[String],
) {
    println!("Workflow OS governed workflow active promotion");
    println!("mode: author_workflow_active_promotion");
    println!("status: active_promotion_blocked");
    println!("draft_path: {}", draft.display());
    println!("active_workflow_path: {}", active_path.display());
    println!("candidate_workflow_id: {candidate_workflow_id}");
    println!("preflight_status: blocked");
    println!("blockers: {}", joined_dynamic_codes(blockers));
    println!("warnings: {}", joined_dynamic_codes(warnings));
    println!("pre_write_active_context_validated: false");
    print_author_workflow_active_promotion_boundary(false);
    println!("next_action: resolve_preflight_blockers_then_rerun_active_promotion");
}

fn print_author_workflow_active_promotion_result(
    review: &WorkflowDraftStewardReviewResult,
    active_path: &Path,
    status: &str,
    file_written: bool,
) {
    let card = review.card();
    println!("Workflow OS governed workflow active promotion");
    println!(
        "mode: {}",
        if file_written {
            "author_workflow_active_promotion"
        } else {
            "author_workflow_active_promotion_dry_run"
        }
    );
    println!("status: {status}");
    println!("draft_path: {}", card.draft_path());
    println!("active_workflow_path: {}", active_path.display());
    println!("candidate_workflow_id: {}", card.candidate_workflow_id());
    println!("draft_content_hash: {}", card.draft_content_hash());
    println!(
        "preflight_status: {}",
        preflight_status_label(card.preflight_status())
    );
    println!(
        "warnings: {}",
        joined_dynamic_codes(card.preflight_warnings())
    );
    println!("decision: approved_for_promotion");
    println!("reviewer: {}", review.reviewer());
    println!("pre_write_active_context_validated: true");
    println!(
        "post_write_project_validation: {}",
        if file_written {
            "passed"
        } else {
            "not_run_dry_run"
        }
    );
    print_author_workflow_active_promotion_boundary(file_written);
    println!(
        "next_action: {}",
        if file_written {
            "run_workflow_os_validate_then_review_active_workflow_before_runtime_use"
        } else {
            "rerun_without_dry_run_to_write_active_workflow_file"
        }
    );
}

fn print_author_workflow_active_promotion_boundary(file_written: bool) {
    println!("files_written: {file_written}");
    println!("active_workflow_file_written: {file_written}");
    println!("draft_preserved: true");
    println!("workflow_promoted: {file_written}");
    println!("approval_persisted: false");
    println!("commands_executed: false");
    println!("providers_called: false");
    println!("runtime_state_created: false");
    println!("report_artifact_written: false");
    println!("privacy_boundary: bounded_codes_only_no_raw_payloads");
}

fn author_workflow_active_promotion_blocked_json(
    draft: &Path,
    active_path: &Path,
    candidate_workflow_id: &WorkflowId,
    blockers: &[String],
    warnings: &[String],
    validation_error_codes: &[String],
) -> String {
    format!(
        "{{\"author_workflow_active_promotion\":{{\"schema_version\":\"workflowos.dev/v0\",\"mode\":\"author_workflow_active_promotion\",\"status\":\"active_promotion_blocked\",\"draft_path\":\"{}\",\"active_workflow_path\":\"{}\",\"candidate_workflow_id\":\"{}\",\"preflight_status\":\"blocked\",\"blockers\":{},\"warnings\":{},\"validation_error_codes\":{},\"pre_write_active_context_validated\":false,\"boundary\":{{\"files_written\":false,\"active_workflow_file_written\":false,\"draft_preserved\":true,\"workflow_promoted\":false,\"approval_persisted\":false,\"commands_executed\":false,\"providers_called\":false,\"runtime_state_created\":false,\"report_artifact_written\":false}},\"privacy_boundary\":\"bounded_codes_only_no_raw_payloads\",\"next_action\":\"resolve_preflight_blockers_then_rerun_active_promotion\"}}}}",
        json_escape(&draft.display().to_string()),
        json_escape(&active_path.display().to_string()),
        json_escape(candidate_workflow_id.as_str()),
        json_string_array_dynamic(blockers),
        json_string_array_dynamic(warnings),
        json_string_array_dynamic(validation_error_codes),
    )
}

fn author_workflow_active_promotion_json(
    review: &WorkflowDraftStewardReviewResult,
    active_path: &Path,
    status: &str,
    file_written: bool,
) -> String {
    let card = review.card();
    format!(
        "{{\"{}\":{{\"schema_version\":\"workflowos.dev/v0\",\"mode\":\"{}\",\"status\":\"{}\",\"draft_path\":\"{}\",\"active_workflow_path\":\"{}\",\"candidate_workflow_id\":\"{}\",\"draft_content_hash\":\"{}\",\"preflight_status\":\"{}\",\"warnings\":{},\"decision\":\"approved_for_promotion\",\"reviewer\":\"{}\",\"pre_write_active_context_validated\":true,\"post_write_project_validation\":\"{}\",\"boundary\":{{\"files_written\":{},\"active_workflow_file_written\":{},\"draft_preserved\":true,\"workflow_promoted\":{},\"approval_persisted\":false,\"commands_executed\":false,\"providers_called\":false,\"runtime_state_created\":false,\"report_artifact_written\":false}},\"privacy_boundary\":\"bounded_codes_only_no_raw_payloads\",\"next_action\":\"{}\"}}}}",
        if file_written {
            "author_workflow_active_promotion"
        } else {
            "author_workflow_active_promotion_dry_run"
        },
        if file_written {
            "author_workflow_active_promotion"
        } else {
            "author_workflow_active_promotion_dry_run"
        },
        status,
        json_escape(card.draft_path()),
        json_escape(&active_path.display().to_string()),
        json_escape(card.candidate_workflow_id().as_str()),
        json_escape(&card.draft_content_hash().to_string()),
        preflight_status_label(card.preflight_status()),
        json_string_array_dynamic(card.preflight_warnings()),
        json_escape(review.reviewer().as_str()),
        if file_written { "passed" } else { "not_run_dry_run" },
        file_written,
        file_written,
        file_written,
        if file_written {
            "run_workflow_os_validate_then_review_active_workflow_before_runtime_use"
        } else {
            "rerun_without_dry_run_to_write_active_workflow_file"
        },
    )
}

fn author_workflow_file_output_result_json(
    recommendation: &WorkflowDiscoveryRecommendation,
    output: &Path,
    proposed_workflow_id: &WorkflowId,
) -> String {
    format!(
        "{{\"author_workflow_file_output\":{{\"schema_version\":\"workflowos.dev/v0\",\"mode\":\"author_workflow_file_output\",\"status\":\"inactive_draft_written\",\"source_recommendation_id\":\"{}\",\"proposed_workflow_id\":\"{}\",\"output_path\":\"{}\",\"draft_inactive\":true,\"draft_loaded_by_current_project_loader\":false,\"workflow_registered\":false,\"workflow_promoted\":false,\"commands_executed\":false,\"providers_called\":false,\"runtime_state_created\":false,\"next_action\":\"review_complete_required_fields_validate_then_plan_promotion_separately\"}}}}",
        json_escape(recommendation.id),
        json_escape(proposed_workflow_id.as_str()),
        json_escape(&output.display().to_string()),
    )
}

fn metadata_signal_codes(recommendation: &WorkflowDiscoveryRecommendation) -> Vec<&'static str> {
    recommendation
        .rationale_codes
        .iter()
        .copied()
        .filter(|code| code.starts_with("repo_metadata."))
        .collect()
}

fn recommendation_authoring_required(kind: WorkflowDiscoveryRecommendationKind) -> &'static str {
    match kind {
        WorkflowDiscoveryRecommendationKind::CreateWorkflow => {
            "author_and_review_workflow_spec_with_owner_policy_evidence_checks_side_effects_and_report_posture"
        }
        WorkflowDiscoveryRecommendationKind::AssignOwnership => {
            "replace_placeholder_stewardship_and_escalation_fields_before_treating_governance_as_configured"
        }
        WorkflowDiscoveryRecommendationKind::AddEvidenceCheckRequirements => {
            "define_validation_and_evidence_obligations_before_treating_checks_as_enforced"
        }
        WorkflowDiscoveryRecommendationKind::AddSideEffectPosture => {
            "decide_and_document_side_effect_posture_before_any_write_capability"
        }
        WorkflowDiscoveryRecommendationKind::AddReportHandoffObligations => {
            "define_report_and_handoff_obligations_before_treating_work_as_closed"
        }
    }
}

fn recommendation_non_execution_boundary(
    kind: WorkflowDiscoveryRecommendationKind,
) -> &'static str {
    match kind {
        WorkflowDiscoveryRecommendationKind::CreateWorkflow => {
            "no_workflow_generated_no_file_written_no_command_executed"
        }
        WorkflowDiscoveryRecommendationKind::AssignOwnership => {
            "no_rbac_no_idp_no_paging_no_escalation_notification"
        }
        WorkflowDiscoveryRecommendationKind::AddEvidenceCheckRequirements => {
            "no_check_registered_no_check_executed_no_evidence_fabricated"
        }
        WorkflowDiscoveryRecommendationKind::AddSideEffectPosture => {
            "no_write_enabled_no_provider_mutation_no_side_effect_executed"
        }
        WorkflowDiscoveryRecommendationKind::AddReportHandoffObligations => {
            "no_report_artifact_written_no_runtime_state_created_no_handoff_sent"
        }
    }
}

fn print_safe_repo_metadata(metadata: &SafeRepoMetadata) {
    println!("safe_repo_metadata:");
    println!(
        "  package_json: {}",
        presence_label(metadata.package_json.is_some())
    );
    if let Some(package_json) = &metadata.package_json {
        println!(
            "  package_manager: {}",
            package_json.package_manager.unwrap_or("not_available")
        );
        println!(
            "  package_scripts: {}",
            joined_codes(&package_json.common_script_keys)
        );
        println!(
            "  typescript: {}",
            presence_label(!package_json.typescript_markers.is_empty())
        );
        println!(
            "  typescript_markers: {}",
            joined_codes(&package_json.typescript_markers)
        );
    }
    println!(
        "  cargo_toml: {}",
        presence_label(metadata.ecosystem_files.contains(&"cargo_toml"))
    );
    println!(
        "  cargo_lock: {}",
        presence_label(metadata.cargo_lock_present)
    );
    println!(
        "  pyproject_toml: {}",
        presence_label(metadata.ecosystem_files.contains(&"pyproject_toml"))
    );
    println!(
        "  python_lock_files: {}",
        joined_codes(&metadata.python_lock_files)
    );
    println!(
        "  go_mod: {}",
        presence_label(metadata.ecosystem_files.contains(&"go_mod"))
    );
    println!("  go_sum: {}", presence_label(metadata.go_sum_present));
    println!("  github_workflows: {}", metadata.github_workflow_count);
    println!(
        "  source_dirs: {}",
        joined_codes(&metadata.conventional_source_dirs)
    );
    println!(
        "  test_dirs: {}",
        joined_codes(&metadata.conventional_test_dirs)
    );
    println!(
        "  readme: {}",
        presence_label(metadata.repo_documents.contains(&"readme"))
    );
    println!(
        "  license: {}",
        presence_label(metadata.repo_documents.contains(&"license"))
    );
    println!(
        "  contributing: {}",
        presence_label(metadata.repo_documents.contains(&"contributing"))
    );
    println!(
        "  security_policy: {}",
        presence_label(metadata.repo_documents.contains(&"security_policy"))
    );
}

fn print_ownership_escalation_check(check: &OwnershipEscalationCheck) {
    println!("ownership_escalation_check: {}", check.status_label());
    println!("ownership_escalation_findings: {}", check.issues.len());
    println!(
        "ownership_missing_owner: {}",
        check.count(OwnershipEscalationIssueCode::MissingOwner)
    );
    println!(
        "ownership_placeholder_owner: {}",
        check.count(OwnershipEscalationIssueCode::PlaceholderOwner)
    );
    println!(
        "escalation_missing_contact: {}",
        check.count(OwnershipEscalationIssueCode::MissingEscalation)
    );
    println!(
        "escalation_placeholder_contact: {}",
        check.count(OwnershipEscalationIssueCode::PlaceholderEscalation)
    );
    println!("lifecycle_warnings: {}", check.lifecycle_warning_count());
    println!(
        "authority_context_warnings: {}",
        check.count(OwnershipEscalationIssueCode::AuthorityContextRequired)
    );
    for issue in &check.issues {
        println!(
            "ownership_escalation_finding: target={}#{} code={} severity=warning",
            issue.target_kind.label(),
            issue.target_ordinal,
            issue.code.label()
        );
    }
}

fn print_spec_field_coverage_check(check: &SpecFieldCoverageCheck) {
    println!("spec_field_coverage_check: {}", check.status_label());
    println!(
        "spec_field_coverage_enforced: {}",
        check.count(SpecFieldCoverageCategory::Enforced)
    );
    println!(
        "spec_field_coverage_validated: {}",
        check.count(SpecFieldCoverageCategory::Validated)
    );
    println!(
        "spec_field_coverage_disclosed: {}",
        check.count(SpecFieldCoverageCategory::Disclosed)
    );
    println!(
        "spec_field_coverage_advisory: {}",
        check.count(SpecFieldCoverageCategory::Advisory)
    );
    println!(
        "spec_field_coverage_deferred: {}",
        check.count(SpecFieldCoverageCategory::Deferred)
    );
    for item in &check.items {
        println!(
            "spec_field_coverage_item: surface={} field={} posture={} code={}",
            item.surface.label(),
            item.field,
            item.posture,
            item.code
        );
    }
}

fn print_workflow_discovery_recommendations(recommendations: &[WorkflowDiscoveryRecommendation]) {
    println!(
        "workflow_discovery_recommendations: {}",
        recommendations.len()
    );
    for recommendation in recommendations {
        println!(
            "workflow_discovery_recommendation: id={} kind={} target={}#{} status={} summary={} rationale={} coverage={} ownership={} next_action={}",
            recommendation.id,
            recommendation.kind.label(),
            recommendation.target.surface.label(),
            recommendation.target.ordinal,
            recommendation.status.label(),
            recommendation.summary,
            joined_codes(&recommendation.rationale_codes),
            joined_codes(&recommendation.coverage_codes),
            joined_codes(&recommendation.ownership_issue_codes),
            recommendation.next_action
        );
    }
}

fn first_run_json(context: &FirstRunReportReadyContext) -> String {
    let sections = context
        .sections
        .iter()
        .map(|section| format!("\"{}\"", section_kind_label(section.kind())))
        .collect::<Vec<_>>()
        .join(",");
    let recommendations = context
        .recommendations
        .iter()
        .map(|recommendation| format!("\"{}\"", json_escape(recommendation)))
        .collect::<Vec<_>>()
        .join(",");
    let deferred_fields = context
        .governance_posture
        .deferred_fields
        .iter()
        .map(|field| format!("\"{}\"", json_escape(field)))
        .collect::<Vec<_>>()
        .join(",");
    let ownership_escalation_issues = context
        .ownership_escalation_check
        .issues
        .iter()
        .map(|issue| {
            format!(
                "{{\"target\":\"{}\",\"ordinal\":{},\"code\":\"{}\",\"severity\":\"warning\"}}",
                issue.target_kind.label(),
                issue.target_ordinal,
                issue.code.label()
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let spec_field_coverage = spec_field_coverage_check_json(&context.spec_field_coverage_check);
    let workflow_discovery_recommendations =
        workflow_discovery_recommendations_json(&context.workflow_discovery_recommendations);
    let recommendation_next_actions = json_string_array(&context.recommendation_next_actions);
    let safe_repo_metadata = safe_repo_metadata_json(&context.repo_metadata);
    format!(
        "{{\"first_run_report_ready\":true,\"mode\":\"report_ready_context\",\"validation\":\"passed\",\"scaffold_present\":{},\"git_repository_present\":{},\"spec_counts\":{{\"workflows\":{},\"skills\":{},\"policies\":{},\"tests\":{}}},\"safe_repo_metadata\":{},\"sections\":[{}],\"incomplete_work_disclosures\":{},\"known_limitations\":{},\"risks\":{},\"handoff_notes\":{},\"evidence\":\"not_available\",\"checks\":\"skipped\",\"side_effects\":\"none_skipped_unsupported\",\"governance_profile\":\"{}\",\"profile_posture\":\"{}\",\"governance_field_posture\":{{\"ownership\":\"{}\",\"escalation\":\"{}\",\"approvals\":\"{}\",\"policy_gates\":\"{}\",\"evidence\":\"{}\",\"checks\":\"{}\",\"side_effects\":\"{}\",\"audit_observability\":\"{}\",\"deferred_fields\":[{}]}},\"ownership_escalation_check\":{{\"status\":\"{}\",\"findings\":{},\"missing_owner\":{},\"placeholder_owner\":{},\"missing_escalation\":{},\"placeholder_escalation\":{},\"lifecycle_warnings\":{},\"authority_context_warnings\":{},\"issues\":[{}]}},\"spec_field_coverage_check\":{},\"workflow_discovery_recommendations\":{},\"recommendation_next_actions\":{},\"recommendations\":[{}]}}",
        context.scaffold_present,
        context.git_present,
        context.workflow_count,
        context.skill_count,
        context.policy_count,
        context.test_count,
        safe_repo_metadata,
        sections,
        context.incomplete_work.len(),
        context.known_limitations.len(),
        context.risks.len(),
        context.handoff_notes.len(),
        context.governance_posture.profile,
        context.governance_posture.profile_posture,
        context.governance_posture.ownership.label(),
        context.governance_posture.escalation.label(),
        context.governance_posture.approvals.label(),
        context.governance_posture.policy_gates.label(),
        context.governance_posture.evidence.label(),
        context.governance_posture.checks.label(),
        context.governance_posture.side_effects.label(),
        context.governance_posture.audit_observability.label(),
        deferred_fields,
        context.ownership_escalation_check.status_label(),
        context.ownership_escalation_check.issues.len(),
        context
            .ownership_escalation_check
            .count(OwnershipEscalationIssueCode::MissingOwner),
        context
            .ownership_escalation_check
            .count(OwnershipEscalationIssueCode::PlaceholderOwner),
        context
            .ownership_escalation_check
            .count(OwnershipEscalationIssueCode::MissingEscalation),
        context
            .ownership_escalation_check
            .count(OwnershipEscalationIssueCode::PlaceholderEscalation),
        context.ownership_escalation_check.lifecycle_warning_count(),
        context
            .ownership_escalation_check
            .count(OwnershipEscalationIssueCode::AuthorityContextRequired),
        ownership_escalation_issues,
        spec_field_coverage,
        workflow_discovery_recommendations,
        recommendation_next_actions,
        recommendations
    )
}

fn workflow_discovery_recommendations_json(
    recommendations: &[WorkflowDiscoveryRecommendation],
) -> String {
    let items = recommendations
        .iter()
        .map(|recommendation| {
            format!(
                "{{\"id\":\"{}\",\"kind\":\"{}\",\"target\":{{\"surface\":\"{}\",\"ordinal\":{}}},\"status\":\"{}\",\"summary\":\"{}\",\"rationale_codes\":{},\"coverage_codes\":{},\"ownership_issue_codes\":{},\"next_action\":\"{}\"}}",
                json_escape(recommendation.id),
                recommendation.kind.label(),
                recommendation.target.surface.label(),
                recommendation.target.ordinal,
                recommendation.status.label(),
                json_escape(recommendation.summary),
                json_string_array(&recommendation.rationale_codes),
                json_string_array(&recommendation.coverage_codes),
                json_string_array(&recommendation.ownership_issue_codes),
                recommendation.next_action
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"status\":\"review_only\",\"count\":{},\"items\":[{}]}}",
        recommendations.len(),
        items
    )
}

fn first_run_recommendation_detail_json(
    recommendation: &WorkflowDiscoveryRecommendation,
    draft_proposal: &GovernedWorkflowDraftProposal,
) -> String {
    format!(
        "{{\"first_run_recommendation_detail\":{{\"id\":\"{}\",\"kind\":\"{}\",\"target\":{{\"surface\":\"{}\",\"ordinal\":{}}},\"status\":\"{}\",\"review_posture\":\"review_only_not_active_workflow\",\"summary\":\"{}\",\"rationale_codes\":{},\"metadata_signals\":{},\"coverage_codes\":{},\"ownership_issue_codes\":{},\"next_action\":\"{}\",\"draft_proposal\":{{\"source_recommendation_id\":\"{}\",\"status\":\"{}\",\"proposed_lifecycle_status\":\"{}\",\"proposal_kind\":\"{}\",\"proposed_purpose_code\":\"{}\",\"required_authoring_decisions\":{},\"validation_expectations\":{},\"missing_required_fields\":{},\"non_goals\":{},\"privacy_boundary\":\"{}\"}},\"authoring_required\":\"{}\",\"what_workflow_os_did_not_do\":\"{}\",\"privacy_boundary\":\"bounded_codes_only_no_raw_payloads\"}}}}",
        json_escape(recommendation.id),
        recommendation.kind.label(),
        recommendation.target.surface.label(),
        recommendation.target.ordinal,
        recommendation.status.label(),
        json_escape(recommendation.summary),
        json_string_array(&recommendation.rationale_codes),
        json_string_array(&metadata_signal_codes(recommendation)),
        json_string_array(&recommendation.coverage_codes),
        json_string_array(&recommendation.ownership_issue_codes),
        recommendation.next_action,
        json_escape(draft_proposal.source_recommendation_id),
        draft_proposal.status,
        draft_proposal.proposed_lifecycle_status,
        draft_proposal.proposal_kind,
        json_escape(draft_proposal.proposed_purpose_code),
        json_string_array(&draft_proposal.required_authoring_decisions),
        json_string_array(&draft_proposal.validation_expectations),
        json_string_array(&draft_proposal.missing_required_fields),
        json_string_array(&draft_proposal.non_goals),
        draft_proposal.privacy_boundary,
        recommendation_authoring_required(recommendation.kind),
        recommendation_non_execution_boundary(recommendation.kind)
    )
}

fn safe_repo_metadata_json(metadata: &SafeRepoMetadata) -> String {
    let package_json = if let Some(package_json) = &metadata.package_json {
        format!(
            "{{\"present\":true,\"package_manager\":\"{}\",\"common_script_keys\":{},\"typescript_detected\":{},\"typescript_markers\":{}}}",
            json_escape(package_json.package_manager.unwrap_or("not_available")),
            json_string_array(&package_json.common_script_keys),
            !package_json.typescript_markers.is_empty(),
            json_string_array(&package_json.typescript_markers)
        )
    } else {
        "{\"present\":false,\"package_manager\":\"not_available\",\"common_script_keys\":[],\"typescript_detected\":false,\"typescript_markers\":[]}".to_string()
    };
    format!(
        "{{\"package_json\":{},\"cargo_toml_present\":{},\"cargo_lock_present\":{},\"pyproject_toml_present\":{},\"python_lock_files\":{},\"go_mod_present\":{},\"go_sum_present\":{},\"github_workflow_count\":{},\"github_actions_detected\":{},\"conventional_source_dirs\":{},\"conventional_test_dirs\":{},\"readme_present\":{},\"license_present\":{},\"contributing_present\":{},\"security_policy_present\":{}}}",
        package_json,
        metadata.ecosystem_files.contains(&"cargo_toml"),
        metadata.cargo_lock_present,
        metadata.ecosystem_files.contains(&"pyproject_toml"),
        json_string_array(&metadata.python_lock_files),
        metadata.ecosystem_files.contains(&"go_mod"),
        metadata.go_sum_present,
        metadata.github_workflow_count,
        metadata.github_actions_detected(),
        json_string_array(&metadata.conventional_source_dirs),
        json_string_array(&metadata.conventional_test_dirs),
        metadata.repo_documents.contains(&"readme"),
        metadata.repo_documents.contains(&"license"),
        metadata.repo_documents.contains(&"contributing"),
        metadata.repo_documents.contains(&"security_policy")
    )
}

fn spec_field_coverage_check_json(check: &SpecFieldCoverageCheck) -> String {
    let items = check
        .items
        .iter()
        .map(|item| {
            format!(
                "{{\"surface\":\"{}\",\"field\":\"{}\",\"category\":\"{}\",\"posture\":\"{}\",\"code\":\"{}\"}}",
                item.surface.label(),
                json_escape(item.field),
                item.category.label(),
                json_escape(item.posture),
                json_escape(item.code)
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"status\":\"{}\",\"enforced\":{},\"validated\":{},\"disclosed\":{},\"advisory\":{},\"deferred\":{},\"items\":[{}]}}",
        check.status_label(),
        check.count(SpecFieldCoverageCategory::Enforced),
        check.count(SpecFieldCoverageCategory::Validated),
        check.count(SpecFieldCoverageCategory::Disclosed),
        check.count(SpecFieldCoverageCategory::Advisory),
        check.count(SpecFieldCoverageCategory::Deferred),
        items
    )
}

fn joined_codes(codes: &[&'static str]) -> String {
    if codes.is_empty() {
        "none".to_string()
    } else {
        codes.join("|")
    }
}

fn joined_dynamic_codes(codes: &[String]) -> String {
    if codes.is_empty() {
        "none".to_string()
    } else {
        codes.join("|")
    }
}

fn json_string_array(values: &[&'static str]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(|value| format!("\"{}\"", json_escape(value)))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn json_string_array_dynamic(values: &[String]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(|value| format!("\"{}\"", json_escape(value)))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn presence_label(present: bool) -> &'static str {
    if present {
        "present"
    } else {
        "not_detected"
    }
}

fn section_kind_label(kind: WorkReportSectionKind) -> &'static str {
    match kind {
        WorkReportSectionKind::WorkPerformed => "work_performed",
        WorkReportSectionKind::EvidenceConsidered => "evidence_considered",
        WorkReportSectionKind::DecisionsMade => "decisions_made",
        WorkReportSectionKind::PolicyGatesEvaluated => "policy_gates_evaluated",
        WorkReportSectionKind::Approvals => "approvals",
        WorkReportSectionKind::ValidationAndQualityChecks => "validation_and_quality_checks",
        WorkReportSectionKind::SideEffects => "side_effects",
        WorkReportSectionKind::IncompleteOrDeferredWork => "incomplete_or_deferred_work",
        WorkReportSectionKind::KnownLimitations => "known_limitations",
        WorkReportSectionKind::Risks => "risks",
        WorkReportSectionKind::OperatorHandoffNotes => "operator_handoff_notes",
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ScaffoldKind {
    Plain,
    ManagedBlock,
}

fn repo_governance_scaffold_files(
    agent: AgentHarnessFlavor,
) -> Vec<(&'static str, String, ScaffoldKind)> {
    vec![
        (
            "workflow-os.yml",
            repo_governance_manifest(),
            ScaffoldKind::Plain,
        ),
        (
            "workflows/first-run-governance.workflow.yml",
            repo_governance_workflow(),
            ScaffoldKind::Plain,
        ),
        (
            "skills/first-run-report.skill.yml",
            repo_governance_skill(),
            ScaffoldKind::Plain,
        ),
        (
            "policies/default-governance.policy.yml",
            repo_governance_policy(),
            ScaffoldKind::Plain,
        ),
        (
            "policies/local.policy.yml",
            repo_governance_local_policy(),
            ScaffoldKind::Plain,
        ),
        (
            "tests/first-run-governance.test.yml",
            repo_governance_test(),
            ScaffoldKind::Plain,
        ),
        (
            ".workflow-os/README.md",
            repo_governance_setup_note(),
            ScaffoldKind::Plain,
        ),
        (
            "AGENTS.md",
            agent_harness_agents_file(agent),
            ScaffoldKind::ManagedBlock,
        ),
        (
            ".workflow-os/agent-harness-prompt.md",
            agent_harness_prompt_file(agent),
            ScaffoldKind::ManagedBlock,
        ),
    ]
}

fn plain_scaffold_file_content(
    path: &Path,
    generated: &str,
    force: bool,
    label: &'static str,
) -> Result<String, WorkflowOsError> {
    if !path.exists() || force {
        return Ok(generated.to_owned());
    }
    Err(WorkflowOsError::new(
        WorkflowOsErrorKind::InvalidState,
        "cli.init_repo_governance.file_exists",
        format!("{label} already exists; rerun with --force to replace it"),
    ))
}

fn repo_governance_manifest() -> String {
    r"schema_version: workflowos.dev/v0
project:
  id: local/existing-repo
  name: Existing Repo Governed Work
  description: Minimal local governance envelope for agent-assisted work in this repository.
layout:
  workflows: workflows
  skills: skills
  policies: policies
  tests: tests
config:
  - environment: local
    vars:
      - name: governance_mode
        value: first-run
"
    .to_owned()
}

fn repo_governance_workflow() -> String {
    r"schema_version: workflowos.dev/v0
id: local/first-run-governance
version: v0
display_name: First-Run Governed Work
description: Map the initial governed-work posture for this repository and require human approval before the mock report step.
owner:
  owning_team: local-maintainers
  maintainer: local-maintainer
  escalation_contact: local-maintainer
  lifecycle_status: stable
autonomy_level: level_2
triggers:
  - id: manual-start
    kind: manual
state_model:
  type: inline
  states:
    - scoped
    - approved
    - reported
steps:
  - id: first-run-report
    skill_ref:
      id: local/first-run-report
      version: v0
    input_mapping:
      - from:
          type: literal
          value: first-run-governed-work
        to: task
    policy_requirements:
      - id: local/allow
    idempotency_key_strategy:
      type: derived
    approval_policy:
      policy:
        id: default/governed-work
    timeout:
      duration: 5m
    terminal_behavior: fail_workflow
approval_requirements:
  - id: human-first-run-review
    reason: Human review is required before accepting the first-run governed-work report posture.
    expires_after:
      duration: 30m
cancellation_behavior: stop
audit_requirements:
  required: true
  events:
    - RunCreated
    - PolicyDecisionRecorded
    - ApprovalRequested
    - ApprovalGranted
    - SkillInvocationSucceeded
  store_references_only: true
observability_requirements:
  metrics:
    - workflow_latency
    - policy_decision_count
    - approval_wait_time
    - skill_latency
  tracing: true
  latency_tracking: true
tags:
  - first-run
  - governed-work-pattern
  - local-first
"
    .to_owned()
}

fn repo_governance_skill() -> String {
    r"schema_version: workflowos.dev/v0
id: local/first-run-report
version: v0
display_name: First-Run Governance Report
description: Deterministic local mock skill placeholder for first-run governed-work reporting.
owner:
  owning_team: local-maintainers
  maintainer: local-maintainer
  escalation_contact: local-maintainer
  lifecycle_status: stable
input_contract:
  fields:
    - name: task
      field_type: string
      description: Non-secret governed-work task label.
  required:
    - task
output_contract:
  fields:
    - name: summary
      field_type: string
      description: Bounded first-run governance summary.
  required:
    - summary
allowed_capabilities:
  - name: local.read
failure_modes:
  - code: insufficient_context
    description: Required non-secret context is missing.
    retryable: false
evaluation_criteria:
  - name: governed_work_pattern_posture
    description: Summary should disclose goal, context, evidence, checks, approvals, side effects, risks, and deferred workflow recommendations.
retry_compatibility: requires_policy
approval_sensitivity: medium
audit_requirements:
  required: true
  events:
    - SkillInvocationRequested
    - SkillInvocationSucceeded
  store_references_only: true
observability_requirements:
  metrics:
    - skill_latency
  tracing: true
  latency_tracking: true
tags:
  - first-run
  - report-posture
  - local-first
"
    .to_owned()
}

fn repo_governance_policy() -> String {
    r"schema_version: workflowos.dev/v0
id: default/governed-work
name: Default Governed Work Policy
description: Conservative local policy requiring approval before first-run governed work.
rules:
  - id: require-human-approval
    effect: require_approval
"
    .to_owned()
}

fn repo_governance_local_policy() -> String {
    r"schema_version: workflowos.dev/v0
id: local/allow
name: Local Allow Policy
description: Conservative local policy requirement for first-run governed work.
rules:
  - id: allow-local-read
    effect: allow_local
"
    .to_owned()
}

fn repo_governance_test() -> String {
    r"schema_version: workflowos.dev/v0
id: local/first-run-governance-basic
name: First-run governed work validates and pauses for approval
target:
  id: local/first-run-governance
  version: v0
assertions:
  - id: approval-required
    description: First-run governed work requires human approval before report posture is accepted.
  - id: no-real-command-execution
    description: The generated project uses a mockable local skill placeholder and does not enable arbitrary command execution.
  - id: report-posture
    description: The generated workflow tees up evidence, checks, side-effect disclosure, risks, and workflow recommendation posture.
"
    .to_owned()
}

fn repo_governance_setup_note() -> String {
    r"# Workflow OS Existing Repo Governance

This repository has been scaffolded as a local Workflow OS project.

The first-run workflow is:

```sh
workflow-os validate
workflow-os --mock-all-local-skills run local/first-run-governance
```

This scaffold applies the default Governed Work Pattern posture: bounded goal, context, evidence, checks, approval, side-effect disclosure, risks, skipped work, final report closure, and future workflow recommendations.

Current boundaries:

- The generated skill is a governed placeholder unless a real handler is implemented, registered, and reviewed.
- `--mock-all-local-skills` is a local preview convenience, not proof of real command execution.
- Workflow OS governs; agents or humans execute unsupported repository work.
- This scaffold does not execute arbitrary shell commands, write to providers, create branches, open PRs, rerun CI, persist report artifacts, host agents, enable recursive agents, or enable Level 3/4 autonomy.
"
    .to_owned()
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
    if let Some(updated) = replace_managed_block(&existing, generated_block.as_str()) {
        return Ok(updated);
    }
    if label == "AGENTS.md" {
        return Ok(append_managed_block(&existing, generated_block.as_str()));
    }
    Err(WorkflowOsError::new(
        WorkflowOsErrorKind::InvalidState,
        "cli.init_agent_harness.unmanaged_file",
        format!("{label} has unmanaged content; rerun with --force to replace it"),
    ))
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

fn append_managed_block(existing: &str, replacement_block: &str) -> String {
    let mut output = existing.to_owned();
    if !output.ends_with('\n') {
        output.push('\n');
    }
    if !output.ends_with("\n\n") {
        output.push('\n');
    }
    output.push_str(replacement_block);
    if !output.ends_with('\n') {
        output.push('\n');
    }
    output
}

fn print_agent_harness_preservation_notice(path: &Path, force: bool, dry_run: bool) {
    if !path.exists() {
        return;
    }
    let existing = fs::read_to_string(path).unwrap_or_default();
    let has_managed_block =
        existing.contains(AGENT_HARNESS_BEGIN) && existing.contains(AGENT_HARNESS_END);
    if force {
        if dry_run {
            println!("would_replace_existing_agent_guidance: AGENTS.md");
        } else {
            println!("replaced_existing_agent_guidance: AGENTS.md");
        }
    } else if has_managed_block {
        if dry_run {
            println!("would_update_managed_agent_guidance: AGENTS.md");
        } else {
            println!("updated_managed_agent_guidance: AGENTS.md");
        }
    } else if dry_run {
        println!("would_preserve_unmanaged_agent_guidance: AGENTS.md");
        println!("would_append_managed_agent_guidance: AGENTS.md");
    } else {
        println!("preserved_unmanaged_agent_guidance: AGENTS.md");
        println!("appended_managed_agent_guidance: AGENTS.md");
    }
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
- Read this repository's engineering standard or contribution guide if one exists.
- Read `.workflow-os/README.md` and `.workflow-os/agent-harness-prompt.md` before governed work.
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
                "--help" | "-h" if positional.is_empty() => positional.push("help".to_owned()),
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
    InitRepoGovernance {
        output_dir: Option<PathBuf>,
        agent: AgentHarnessFlavor,
        force: bool,
        dry_run: bool,
    },
    FirstRun {
        verbose: bool,
        recommendation: Option<String>,
    },
    AuthorWorkflow {
        from_recommendation: Option<String>,
        dry_run: bool,
        output: Option<PathBuf>,
    },
    AuthorWorkflowPreflight {
        draft: PathBuf,
    },
    AuthorWorkflowDraftStatus {
        draft: PathBuf,
    },
    AuthorWorkflowArchiveDraft {
        draft: PathBuf,
        reviewer: ActorId,
        reason: String,
        dry_run: bool,
    },
    AuthorWorkflowStewardReview {
        draft: PathBuf,
        decision: WorkflowDraftStewardReviewDecision,
        reviewer: ActorId,
        reason: String,
    },
    AuthorWorkflowPromote {
        draft: PathBuf,
        reviewer: ActorId,
        reason: String,
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
    if command_help_requested(args) && is_helpable_command(command) {
        return Ok(Command::Help);
    }
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
        "init-repo-governance" => Ok(Command::InitRepoGovernance {
            output_dir: flag_value(args, "--output-dir").map(PathBuf::from),
            agent: flag_value(args, "--agent")
                .as_deref()
                .map(AgentHarnessFlavor::parse)
                .transpose()?
                .unwrap_or(AgentHarnessFlavor::Generic),
            force: flag_present(args, "--force"),
            dry_run: flag_present(args, "--dry-run"),
        }),
        "first-run" => {
            let recommendation = optional_flag_value(args, "--recommendation")?;
            Ok(Command::FirstRun {
                verbose: flag_present(args, "--verbose"),
                recommendation,
            })
        }
        "author" => parse_author_command(args),
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

fn parse_author_command(args: &[String]) -> Result<Command, WorkflowOsError> {
    match args.get(1).map(String::as_str) {
        Some("workflow") => parse_author_workflow_command(args),
        Some(other) => Err(usage(format!("unknown author subcommand {other}"))),
        None => Err(usage("author requires <subcommand>")),
    }
}

fn parse_author_workflow_command(args: &[String]) -> Result<Command, WorkflowOsError> {
    match args.get(2).map(String::as_str) {
        Some("draft-status") => Ok(Command::AuthorWorkflowDraftStatus {
            draft: flag_value(args, "--draft")
                .map(PathBuf::from)
                .ok_or_else(|| usage("author workflow draft-status requires --draft <path>"))?,
        }),
        Some("archive-draft") => parse_author_workflow_archive_draft_command(args),
        Some("preflight") => Ok(Command::AuthorWorkflowPreflight {
            draft: flag_value(args, "--draft")
                .map(PathBuf::from)
                .ok_or_else(|| usage("author workflow preflight requires --draft <path>"))?,
        }),
        Some("steward-review") => parse_author_workflow_steward_review_command(args),
        Some("promote") => parse_author_workflow_promote_command(args),
        Some(other) if other.starts_with("--") => parse_author_workflow_draft_command(args),
        Some(other) => Err(usage(format!("unknown author workflow subcommand {other}"))),
        None => parse_author_workflow_draft_command(args),
    }
}

fn parse_author_workflow_draft_command(args: &[String]) -> Result<Command, WorkflowOsError> {
    let from_recommendation = optional_flag_value(args, "--from-recommendation")?;
    Ok(Command::AuthorWorkflow {
        from_recommendation,
        dry_run: flag_present(args, "--dry-run"),
        output: flag_value(args, "--output").map(PathBuf::from),
    })
}

fn parse_author_workflow_archive_draft_command(
    args: &[String],
) -> Result<Command, WorkflowOsError> {
    let draft = flag_value(args, "--draft")
        .map(PathBuf::from)
        .ok_or_else(|| usage("author workflow archive-draft requires --draft <path>"))?;
    let reviewer = ActorId::new(
        flag_value(args, "--reviewer")
            .ok_or_else(|| usage("author workflow archive-draft requires --reviewer <actor>"))?,
    )
    .map_err(|_| {
        WorkflowOsError::validation(
            "cli.workflow_authoring.archive_reviewer_invalid",
            "workflow authoring draft archive reviewer was rejected",
        )
    })?;
    let reason = optional_flag_value(args, "--reason")?
        .ok_or_else(|| usage("author workflow archive-draft requires --reason <reason>"))?;
    Ok(Command::AuthorWorkflowArchiveDraft {
        draft,
        reviewer,
        reason,
        dry_run: flag_present(args, "--dry-run"),
    })
}

fn parse_author_workflow_steward_review_command(
    args: &[String],
) -> Result<Command, WorkflowOsError> {
    let draft = flag_value(args, "--draft")
        .map(PathBuf::from)
        .ok_or_else(|| usage("author workflow steward-review requires --draft <path>"))?;
    let decision =
        parse_steward_review_decision(&flag_value(args, "--decision").ok_or_else(|| {
            usage("author workflow steward-review requires --decision <decision>")
        })?)?;
    let reviewer = ActorId::new(
        flag_value(args, "--reviewer")
            .ok_or_else(|| usage("author workflow steward-review requires --reviewer <actor>"))?,
    )
    .map_err(|_| {
        WorkflowOsError::validation(
            "cli.workflow_authoring.steward_review_reviewer_invalid",
            "workflow authoring steward review reviewer was rejected",
        )
    })?;
    let reason = optional_flag_value(args, "--reason")?
        .ok_or_else(|| usage("author workflow steward-review requires --reason <reason>"))?;
    Ok(Command::AuthorWorkflowStewardReview {
        draft,
        decision,
        reviewer,
        reason,
    })
}

fn parse_author_workflow_promote_command(args: &[String]) -> Result<Command, WorkflowOsError> {
    let draft = flag_value(args, "--draft")
        .map(PathBuf::from)
        .ok_or_else(|| usage("author workflow promote requires --draft <path>"))?;
    let reviewer = ActorId::new(
        flag_value(args, "--reviewer")
            .ok_or_else(|| usage("author workflow promote requires --reviewer <actor>"))?,
    )
    .map_err(|_| {
        WorkflowOsError::validation(
            "cli.workflow_authoring.active_promotion_reviewer_invalid",
            "workflow authoring active promotion reviewer was rejected",
        )
    })?;
    let reason = optional_flag_value(args, "--reason")?
        .ok_or_else(|| usage("author workflow promote requires --reason <reason>"))?;
    Ok(Command::AuthorWorkflowPromote {
        draft,
        reviewer,
        reason,
        dry_run: flag_present(args, "--dry-run"),
    })
}

fn is_helpable_command(command: &str) -> bool {
    matches!(
        command,
        "validate"
            | "doctor"
            | "init-agent-harness"
            | "init-repo-governance"
            | "first-run"
            | "author"
            | "run"
            | "status"
            | "approve"
            | "inspect"
    )
}

fn command_help_requested(args: &[String]) -> bool {
    args.iter().skip(1).any(|arg| is_help_arg(arg))
}

fn is_help_arg(value: &str) -> bool {
    matches!(value, "--help" | "-h")
}

fn flag_value(args: &[String], flag: &str) -> Option<String> {
    args.windows(2)
        .find(|window| window[0] == flag)
        .map(|window| window[1].clone())
}

fn optional_flag_value(args: &[String], flag: &str) -> Result<Option<String>, WorkflowOsError> {
    if !flag_present(args, flag) {
        return Ok(None);
    }
    let value = flag_value(args, flag).ok_or_else(missing_value)?;
    if value.starts_with("--") {
        return Err(missing_value());
    }
    Ok(Some(value))
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
    println!("  init-repo-governance [--output-dir <path>] [--agent generic|codex|claude] [--force] [--dry-run]");
    println!(
        "      existing-repo governance scaffold only; creates a valid local project envelope"
    );
    println!("  first-run");
    println!(
        "      emit a bounded report-ready first-run context; does not run workflows or write artifacts"
    );
    println!("      use first-run --verbose for the full posture matrix");
    println!("      use first-run --recommendation <id> for one bounded recommendation detail");
    println!("  author workflow --from-recommendation <id> --dry-run");
    println!(
        "      preview inactive workflow authoring obligations; writes no files and registers nothing"
    );
    println!("  author workflow draft-status --draft workflows/drafts/<name>.workflow.yml");
    println!(
        "      inspect inactive draft active/supersession status; writes no files, promotes nothing, and registers nothing"
    );
    println!(
        "  author workflow archive-draft --draft workflows/drafts/<name>.workflow.yml --reviewer user/<reviewer> --reason <reason> [--dry-run]"
    );
    println!(
        "      archive one promoted/superseded inactive draft; does not delete, promote, register, create runtime state, or run workflows"
    );
    println!("  author workflow preflight --draft workflows/drafts/<name>.workflow.yml");
    println!(
        "      inspect inactive draft promotability; writes no files, promotes nothing, and registers nothing"
    );
    println!(
        "  author workflow steward-review --draft workflows/drafts/<name>.workflow.yml --decision approved-for-promotion --reviewer user/<reviewer> --reason <reason>"
    );
    println!(
        "      preview steward review of a preflight-passing inactive draft; writes no files and promotes nothing"
    );
    println!(
        "  author workflow promote --draft workflows/drafts/<name>.workflow.yml --reviewer user/<reviewer> --reason <reason> [--dry-run]"
    );
    println!(
        "      explicitly promote one reviewed draft to workflows/; does not persist approvals, create runtime state, or run workflows"
    );
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

fn schema_status_word(available: bool) -> &'static str {
    if available {
        "ok"
    } else {
        "unavailable_optional"
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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_recommendation(
        id: &'static str,
        kind: WorkflowDiscoveryRecommendationKind,
    ) -> WorkflowDiscoveryRecommendation {
        WorkflowDiscoveryRecommendation {
            id,
            kind,
            target: WorkflowDiscoveryRecommendationTarget::project(),
            status: WorkflowDiscoveryRecommendationStatus::ReviewOnly,
            summary: "test_summary_code",
            rationale_codes: vec!["first_run.report_ready_context"],
            coverage_codes: vec!["spec_field.workflow.steps_enforced_supported_local_paths"],
            ownership_issue_codes: Vec::new(),
            next_action: workflow_discovery_next_action(kind),
        }
    }

    #[test]
    fn governed_workflow_draft_proposal_is_inactive_and_bounded() -> Result<(), WorkflowOsError> {
        let recommendation = test_recommendation(
            "first_run.typescript_implementation",
            WorkflowDiscoveryRecommendationKind::CreateWorkflow,
        );

        let proposal = governed_workflow_draft_proposal_from_recommendation(&recommendation)?;

        assert_eq!(proposal.source_recommendation_id, recommendation.id);
        assert_eq!(proposal.status, "inactive_review_required");
        assert_eq!(proposal.proposed_lifecycle_status, "draft");
        assert_eq!(proposal.proposal_kind, "workflow_draft_proposal");
        assert_eq!(proposal.proposed_purpose_code, "test_summary_code");
        assert!(proposal
            .required_authoring_decisions
            .contains(&"assign_owner"));
        assert!(proposal
            .required_authoring_decisions
            .contains(&"define_side_effect_posture"));
        assert!(proposal
            .validation_expectations
            .contains(&"check_workflow_id_conflicts_before_promotion"));
        assert!(proposal.missing_required_fields.contains(&"workflow_id"));
        assert!(proposal.non_goals.contains(&"no_file_written"));
        assert!(proposal.non_goals.contains(&"no_active_workflow_created"));
        assert_eq!(
            proposal.privacy_boundary,
            "bounded_codes_only_no_raw_payloads"
        );
        Ok(())
    }

    #[test]
    fn governed_workflow_draft_proposal_rejects_secret_like_recommendation_id(
    ) -> Result<(), WorkflowOsError> {
        let secret_id = "first_run.secret-token-workflow";
        let recommendation = test_recommendation(
            secret_id,
            WorkflowDiscoveryRecommendationKind::CreateWorkflow,
        );

        let Err(error) = governed_workflow_draft_proposal_from_recommendation(&recommendation)
        else {
            return Err(WorkflowOsError::validation(
                "test.expected_error",
                "secret-like recommendation id should be rejected",
            ));
        };

        assert_eq!(
            error.code(),
            "cli.workflow_authoring.unsafe_payload_rejected"
        );
        assert!(!error.to_string().contains(secret_id));
        Ok(())
    }

    #[test]
    fn side_effect_draft_proposal_does_not_enable_writes() -> Result<(), WorkflowOsError> {
        let recommendation = test_recommendation(
            "first_run.side_effect_posture",
            WorkflowDiscoveryRecommendationKind::AddSideEffectPosture,
        );

        let proposal = governed_workflow_draft_proposal_from_recommendation(&recommendation)?;

        assert_eq!(proposal.proposal_kind, "side_effect_posture_proposal");
        assert!(proposal
            .required_authoring_decisions
            .contains(&"document_unsupported_writes"));
        assert!(proposal
            .non_goals
            .contains(&"no_write_enabled_no_side_effect_executed"));
        assert!(proposal
            .validation_expectations
            .contains(&"confirm_no_write_capability_enabled"));
        Ok(())
    }

    #[test]
    fn report_handoff_draft_proposal_requires_closure_obligations() -> Result<(), WorkflowOsError> {
        let recommendation = test_recommendation(
            "first_run.report_handoff_obligations",
            WorkflowDiscoveryRecommendationKind::AddReportHandoffObligations,
        );

        let proposal = governed_workflow_draft_proposal_from_recommendation(&recommendation)?;

        assert_eq!(proposal.proposal_kind, "report_handoff_obligation_proposal");
        assert!(proposal
            .required_authoring_decisions
            .contains(&"define_required_report_sections"));
        assert!(proposal
            .missing_required_fields
            .contains(&"handoff_requirements"));
        assert!(proposal
            .non_goals
            .contains(&"no_report_artifact_written_no_handoff_sent"));
        Ok(())
    }
}
