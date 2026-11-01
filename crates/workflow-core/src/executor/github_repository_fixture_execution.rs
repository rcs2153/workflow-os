use std::collections::BTreeMap;

use sha2::{Digest, Sha256};

use super::github_repository_atomic_lifecycle::{
    prepare_atomic_github_repository_metadata_read, GitHubRepositoryAtomicLifecycleInput,
    GitHubRepositoryMetadataAttemptOperation, GitHubRepositoryMetadataAttemptResult,
};
use super::*;
use crate::authorized_execution_continuity_state::internal::{
    trusted_time_observation, ContinuityOperationId, ContinuityReceiptId,
    ContinuityTrustedTimeEpochId, TrustedTimeSourceKind,
};
use crate::trusted_host_supervisor::TrustedHostSupervisorPersistenceInput;
use crate::{
    github_actions, github_read_request, AdapterOperationId, AdapterOperationMode,
    AdapterPolicyPrecheck, AdapterRunScope, AdapterRuntimeAuditRecord,
    AdapterRuntimeObservabilityRecord, AdapterTelemetryStore, GitHubFixtureClient,
    GitHubReadOnlyAdapter, GitHubReadOnlyConfig, HarnessContractId, HarnessContractVersion,
    RequiredContextContractBinding, RequiredContextExecutionBinding,
    RequiredContextExecutionBindingInput, SqliteStateBackend, StateBackend,
};

const OPERATION_CODE: &str = "github.repository.metadata";

/// Exact fixture-only request for the first public GitHub repository read path.
#[derive(Clone)]
pub struct GitHubRepositoryMetadataFixtureExecutionRequest {
    /// Immutable execution request and run-bundle inputs.
    pub execution: LocalExecutionWithImmutableRunBundleRequest,
    /// Exact single step selected for the provider read.
    pub selected_step_id: StepId,
    /// Fixture payload whose hash is bound into the approved execution context.
    pub fixture_json: String,
    /// Trusted observation time used by the local supervisor.
    pub observed_at: Timestamp,
    /// Bounded execution-window expiry.
    pub expires_at: Timestamp,
}

impl fmt::Debug for GitHubRepositoryMetadataFixtureExecutionRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubRepositoryMetadataFixtureExecutionRequest")
            .field("execution", &"[REDACTED]")
            .field("selected_step_id", &"[REDACTED]")
            .field("fixture_json", &"[REDACTED]")
            .field("observed_at", &"[REDACTED]")
            .field("expires_at", &"[REDACTED]")
            .finish()
    }
}

/// Public posture for the bounded fixture-backed provider execution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GitHubRepositoryMetadataFixtureExecutionPosture {
    /// The immutable run is paused at its authored approval gate.
    WaitingForApproval,
    /// The fixture read completed and its bounded telemetry was persisted.
    Completed,
    /// A repeated invocation found an already-terminal run and did no work.
    ExistingTerminal,
    /// The provider attempt is waiting for explicit operator recovery.
    RecoveryRequired,
    /// The run cannot advance from its current state.
    Blocked,
}

/// Bounded result for the fixture-backed GitHub repository metadata path.
pub struct GitHubRepositoryMetadataFixtureExecutionResult {
    run: WorkflowRun,
    posture: GitHubRepositoryMetadataFixtureExecutionPosture,
}

impl GitHubRepositoryMetadataFixtureExecutionResult {
    /// Returns the durable run projection.
    #[must_use]
    pub const fn run(&self) -> &WorkflowRun {
        &self.run
    }

    /// Returns the provider execution posture.
    #[must_use]
    pub const fn posture(&self) -> GitHubRepositoryMetadataFixtureExecutionPosture {
        self.posture
    }

    /// Returns the exact supported provider operation code.
    #[must_use]
    pub const fn operation_code(&self) -> &'static str {
        OPERATION_CODE
    }

    /// Returns the enforced fixture-only operation mode.
    #[must_use]
    pub const fn operation_mode(&self) -> AdapterOperationMode {
        AdapterOperationMode::Fixture
    }
}

impl fmt::Debug for GitHubRepositoryMetadataFixtureExecutionResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubRepositoryMetadataFixtureExecutionResult")
            .field("run_status", &self.run.snapshot.status)
            .field("posture", &self.posture)
            .field("operation_code", &OPERATION_CODE)
            .field("operation_mode", &AdapterOperationMode::Fixture)
            .finish()
    }
}

struct FixtureOperation<'a> {
    backend: &'a SqliteStateBackend,
    fixture_json: String,
    actor: ActorId,
    step_id: StepId,
    skill_id: SkillId,
    skill_version: SkillVersion,
}

impl GitHubRepositoryMetadataAttemptOperation for FixtureOperation<'_> {
    fn execute(
        &self,
        input: &SkillInput,
        access: Option<&str>,
    ) -> GitHubRepositoryMetadataAttemptResult {
        if access.is_some() {
            return GitHubRepositoryMetadataAttemptResult::TerminalFailure;
        }
        let Some(owner) = input.values.get("owner") else {
            return GitHubRepositoryMetadataAttemptResult::TerminalFailure;
        };
        let Some(repository) = input.values.get("repo") else {
            return GitHubRepositoryMetadataAttemptResult::TerminalFailure;
        };
        let client = GitHubFixtureClient::new().with_json(
            format!("/repos/{owner}/{repository}"),
            self.fixture_json.clone(),
        );
        let Ok(config) = GitHubReadOnlyConfig::fixture() else {
            return GitHubRepositoryMetadataAttemptResult::TerminalFailure;
        };
        let adapter = GitHubReadOnlyAdapter::new(config, client);
        let metadata = BTreeMap::from([
            ("owner".to_owned(), owner.clone()),
            ("repo".to_owned(), repository.clone()),
        ]);
        let Ok(mut request) = github_read_request(
            github_actions::REPOSITORY_METADATA,
            self.actor.clone(),
            input.correlation_id.clone(),
            metadata,
            AdapterOperationMode::Fixture,
            AdapterPolicyPrecheck::fixture_test_allowed(vec![
                "policy.fixture.github_repository_metadata".to_owned(),
            ]),
        ) else {
            return GitHubRepositoryMetadataAttemptResult::TerminalFailure;
        };
        request.run_scope = Some(AdapterRunScope {
            workflow_run_id: input.run_id.clone(),
            workflow_id: input.workflow_id.clone(),
            workflow_version: input.workflow_version.clone(),
            schema_version: input.schema_version.clone(),
            spec_hash: input.spec_hash.clone(),
        });
        request.input_reference = Some(format!(
            "github-repository-metadata-fixture/{}/{}",
            input.run_id, input.step_id
        ));
        match adapter.read_repository_metadata(&request, owner, repository) {
            Ok(outcome) => {
                let audit = AdapterRuntimeAuditRecord::from_invocation(
                    &outcome.invocation,
                    Some(self.step_id.clone()),
                    Some(self.skill_id.clone()),
                    Some(self.skill_version.clone()),
                    "workflow-core.github-repository-fixture-execution",
                );
                if self.backend.append_adapter_audit_record(&audit).is_err() {
                    return GitHubRepositoryMetadataAttemptResult::TerminalFailure;
                }
                let observability = AdapterRuntimeObservabilityRecord::from_records(
                    &outcome.invocation,
                    &outcome.observability,
                    Some(self.step_id.clone()),
                    Some(self.skill_id.clone()),
                    Some(self.skill_version.clone()),
                    "workflow-core.github-repository-fixture-execution",
                );
                if self
                    .backend
                    .append_adapter_observability_record(&observability)
                    .is_err()
                {
                    return GitHubRepositoryMetadataAttemptResult::TerminalFailure;
                }
                let values = BTreeMap::from([
                    ("operation".to_owned(), OPERATION_CODE.to_owned()),
                    ("mode".to_owned(), "fixture".to_owned()),
                    (
                        "summary".to_owned(),
                        "GitHub repository metadata fixture read completed".to_owned(),
                    ),
                ]);
                GitHubRepositoryMetadataAttemptResult::Succeeded(SkillOutput::new(
                    values,
                    Some(format!(
                        "github-repository-metadata-fixture/{}/{}",
                        input.run_id, input.step_id
                    )),
                ))
            }
            Err(_) => GitHubRepositoryMetadataAttemptResult::TerminalFailure,
        }
    }
}

/// Starts, resumes, or replays the exact fixture-backed repository metadata run.
///
/// # Errors
///
/// Returns a structured error when the project, immutable run context, fixture
/// commitment, provider admission, or durable state is invalid.
pub fn execute_github_repository_metadata_fixture(
    backend: &SqliteStateBackend,
    request: &GitHubRepositoryMetadataFixtureExecutionRequest,
) -> Result<GitHubRepositoryMetadataFixtureExecutionResult, WorkflowOsError> {
    let run_id =
        request.execution.execution.run_id.clone().ok_or_else(|| {
            fixture_error("run_id_required", "fixture execution requires a run ID")
        })?;
    let registry = LocalSkillRegistry::new();
    let executor = LocalExecutor::new(backend, &registry);
    if backend.read_events(&run_id)?.is_empty() {
        return start_fixture_run(&executor, backend, request, run_id);
    }

    let run = backend.rehydrate_run(&run_id)?;
    validate_existing_fixture_run(
        backend,
        &run,
        &request.execution.execution.project_root,
        &request.fixture_json,
    )?;
    if run.snapshot.status.is_terminal() {
        return Ok(result(
            run,
            GitHubRepositoryMetadataFixtureExecutionPosture::ExistingTerminal,
        ));
    }
    match run.snapshot.status {
        WorkflowRunStatus::WaitingForApproval => {
            return Ok(result(
                run,
                GitHubRepositoryMetadataFixtureExecutionPosture::WaitingForApproval,
            ));
        }
        WorkflowRunStatus::WaitingForExternalEvent => {
            return Ok(result(
                run,
                GitHubRepositoryMetadataFixtureExecutionPosture::RecoveryRequired,
            ));
        }
        WorkflowRunStatus::Running | WorkflowRunStatus::Retrying => {}
        _ => {
            return Ok(result(
                run,
                GitHubRepositoryMetadataFixtureExecutionPosture::Blocked,
            ));
        }
    }
    execute_fixture_attempt(backend, request, &run)
}

/// Records one approval decision for the exact fixture operation without
/// entering the provider. A granted decision makes a later `run` invocation
/// eligible for fresh Core reassessment and atomic admission.
///
/// # Errors
///
/// Returns a structured error when presentation proof, immutable context,
/// fixture commitment, approval state, or durable persistence is invalid.
pub fn decide_github_repository_metadata_fixture_approval(
    backend: &SqliteStateBackend,
    request: LocalApprovalPresentationDecisionRequest,
    fixture_json: Option<&str>,
) -> Result<WorkflowRun, WorkflowOsError> {
    let registry = LocalSkillRegistry::new();
    let executor = LocalExecutor::new(backend, &registry);
    let LocalApprovalPresentationDecisionRequest {
        approval: approval_request,
        proof,
        max_presentation_age,
    } = request;
    let (run, approval, decision) = executor.prepare_approval_decision(&approval_request)?;
    let decision = executor.approval_decision_with_presentation_proof(
        &approval,
        decision,
        &proof,
        max_presentation_age,
    )?;
    let mut builder = EventBuilder::from_snapshot(
        &run.snapshot,
        decision.correlation_id.clone(),
        decision.actor.clone(),
    );
    match decision.decision {
        ApprovalDecisionKind::Granted => {
            let fixture_json = fixture_json.ok_or_else(|| {
                fixture_error(
                    "fixture_required",
                    "granting fixture execution requires the exact fixture payload",
                )
            })?;
            validate_existing_fixture_run(
                backend,
                &run,
                &approval_request.project_root,
                fixture_json,
            )?;
            let commitment = fixture_context_commitment(fixture_json)?;
            let _plan = LocalExecutor::<SqliteStateBackend>::prepare_resume_execution(
                &approval_request.project_root,
                &builder,
                &approval,
                &ProjectValidationCapability::Default,
                Some(&commitment),
            )?;
            executor.append(
                &mut builder,
                WorkflowRunEventKind::ApprovalGranted(decision),
                None,
            )?;
            let context = PolicyEvaluationContext {
                action: Action::ResumeWorkflow,
                capabilities: vec![Capability::WorkflowResume, Capability::AuditWrite],
                actor: Some(builder.actor.clone()),
                workflow_id: Some(builder.workflow_id.clone()),
                run_id: Some(builder.run_id.clone()),
                step_id: approval.step_id.clone(),
                skill_id: approval.skill_id.clone(),
                autonomy_level: None,
                approval_sensitivity: None,
                has_approval_policy: false,
                policy_effects: PolicyEffectSet::default(),
                adapter_id: None,
                correlation_id: Some(builder.correlation_id.clone()),
            };
            executor.evaluate_and_record_policy(&mut builder, &context)?;
            executor.append(&mut builder, WorkflowRunEventKind::RunResumed, None)?;
            executor.rehydrate_and_project(&builder.run_id)
        }
        ApprovalDecisionKind::Denied => {
            executor.append(
                &mut builder,
                WorkflowRunEventKind::ApprovalDenied(decision),
                None,
            )?;
            executor.fail_run(
                builder,
                "executor.approval.denied",
                "approval was denied; run failed closed",
            )
        }
    }
}

fn validate_existing_fixture_run(
    backend: &SqliteStateBackend,
    run: &WorkflowRun,
    project_root: &std::path::Path,
    fixture_json: &str,
) -> Result<(), WorkflowOsError> {
    let binding = run
        .snapshot
        .identity
        .immutable_run_bundle
        .as_ref()
        .ok_or_else(|| fixture_error("bundle_missing", "fixture run lacks immutable binding"))?;
    let stored = crate::ImmutableRunBundleStore::read_exact_bundle(
        backend,
        &run.snapshot.identity.run_id,
        binding.bundle_id(),
    )?;
    let workflow = immutable_continuation_workflow(&stored)?;
    let [step] = workflow.steps.as_slice() else {
        return Err(fixture_error(
            "operation_unsupported",
            "fixture execution requires one exact workflow step",
        ));
    };
    let skill = immutable_continuation_skill(&stored, step)?;
    let request = LocalExecutionRequest {
        project_root: project_root.to_path_buf(),
        workflow_id: workflow.id.clone(),
        run_id: Some(run.snapshot.identity.run_id.clone()),
        correlation_id: CorrelationId::generate(),
        actor: run
            .events
            .first()
            .and_then(|event| event.actor.clone())
            .ok_or_else(|| fixture_error("actor_missing", "fixture run actor is unavailable"))?,
        before_skill_invocation_checkpoints:
            LocalExecutionBeforeSkillInvocationCheckpointInputs::default(),
        before_skill_invocation_hook: None,
        side_effect_events: Vec::new(),
        side_effect_lifecycle_events: Vec::new(),
    };
    let commitment = fixture_context_commitment(fixture_json)?;
    let plan = LocalExecutor::<SqliteStateBackend>::prepare_execution_with_capability_and_resolved_context(
        &request,
        run.snapshot.identity.run_id.clone(),
        &ProjectValidationCapability::Default,
        Some(&commitment),
    )?;
    if plan.step.id != step.id || plan.skill_id != skill.id || plan.skill_version != skill.version {
        return Err(fixture_error(
            "immutable_context_mismatch",
            "fixture approval context does not match the immutable run",
        ));
    }
    validate_fixture_plan(&plan, &step.id)
}

fn start_fixture_run(
    executor: &LocalExecutor<'_, SqliteStateBackend>,
    backend: &SqliteStateBackend,
    request: &GitHubRepositoryMetadataFixtureExecutionRequest,
    run_id: WorkflowRunId,
) -> Result<GitHubRepositoryMetadataFixtureExecutionResult, WorkflowOsError> {
    let execution = &request.execution.execution;
    let commitment = fixture_context_commitment(&request.fixture_json)?;
    let mut plan = LocalExecutor::<SqliteStateBackend>::prepare_execution_with_capability_and_resolved_context(
        execution,
        run_id.clone(),
        &ProjectValidationCapability::Default,
        Some(&commitment),
    )?;
    validate_fixture_plan(&plan, &request.selected_step_id)?;
    executor.evaluate_pre_run_policy(&plan, &execution.actor, &execution.correlation_id)?;
    let project = load_validated_project_bundle(
        &execution.project_root,
        ProjectValidationCapability::Default,
    )?;
    let expected = project
        .manifest
        .definition
        .governance
        .as_ref()
        .and_then(|governance| governance.authoritative_execution);
    let activation = authoritative_execution_activation(&project, expected)?;
    let posture = immutable_run_bundle_execution_posture(execution, activation)?;
    let bundle = crate::immutable_run_bundle_builder::build_immutable_run_bundle(
        crate::ImmutableRunBundleBuildRequest {
            project: &project,
            workflow_id: &execution.workflow_id,
            bundle_id: request.execution.bundle.bundle_id.clone(),
            bundle_version: request.execution.bundle.bundle_version.clone(),
            run_id,
            resolved_execution_context_hash: plan.resolved_execution_context_hash.clone(),
            execution_posture: posture,
            handlers: vec![crate::ImmutableRunBundleHandlerReference {
                skill_id: plan.skill_id.clone(),
                skill_version: plan.skill_version.clone(),
                posture: crate::ImmutableRunBundleHandlerPosture::HostedProviderBound,
            }],
            created_at: request.execution.bundle.created_at,
            created_by: execution.actor.clone(),
            sensitivity: request.execution.bundle.sensitivity,
            redaction_required: request.execution.bundle.redaction_required,
        },
    )?;
    validate_immutable_run_bundle_matches_plan(bundle.manifest(), &plan)?;
    persist_or_validate_immutable_run_bundle(backend, &bundle)?;
    plan.immutable_run_bundle = Some(bundle.manifest().run_binding());
    executor.append_run_start(&mut plan)?;
    executor.append_step_scheduled(&mut plan)?;
    plan.step_scheduled = true;
    if plan.policy_effects.requires_approval() {
        let run = executor.pause_for_approval(plan)?;
        return Ok(result(
            run,
            GitHubRepositoryMetadataFixtureExecutionPosture::WaitingForApproval,
        ));
    }
    let run = backend.rehydrate_run(bundle.manifest().run_id())?;
    execute_fixture_attempt(backend, request, &run)
}

#[allow(clippy::too_many_lines)]
fn execute_fixture_attempt(
    backend: &SqliteStateBackend,
    request: &GitHubRepositoryMetadataFixtureExecutionRequest,
    run: &WorkflowRun,
) -> Result<GitHubRepositoryMetadataFixtureExecutionResult, WorkflowOsError> {
    let binding = run
        .snapshot
        .identity
        .immutable_run_bundle
        .as_ref()
        .ok_or_else(|| fixture_error("bundle_missing", "fixture run lacks immutable binding"))?;
    let stored = crate::ImmutableRunBundleStore::read_exact_bundle(
        backend,
        &run.snapshot.identity.run_id,
        binding.bundle_id(),
    )?;
    let contract = RequiredContextContractBinding::explicit_none_for_github_repository_metadata(
        HarnessContractId::new("harness/github-repository-metadata")?,
        HarnessContractVersion::new("v1")?,
    )?;
    let execution_binding =
        RequiredContextExecutionBinding::new(RequiredContextExecutionBindingInput {
            bundle: &stored,
            contract: &contract,
            actor: request.execution.execution.actor.clone(),
            step_id: request.selected_step_id.clone(),
            maximum_sensitivity: WorkReportSensitivity::Internal,
            bound_at: request.observed_at,
        })?;
    let workflow = immutable_continuation_workflow(&stored)?;
    let step = workflow
        .steps
        .iter()
        .find(|step| step.id == request.selected_step_id)
        .ok_or_else(|| fixture_error("step_missing", "fixture step is unavailable"))?;
    let skill = immutable_continuation_skill(&stored, step)?;
    let lifecycle =
        crate::trusted_host_workflow_lifecycle::TrustedHostWorkflowLifecycleContext::from_skill(
            SkillInvocationId::generate(),
            invocation_idempotency_key(
                &run.snapshot.identity.run_id,
                &workflow.id,
                &workflow.version,
                &step.id,
                &skill.id,
                &skill.version,
            )?,
            step.id.clone(),
            skill,
        );
    let suffix = trusted_host_admission_suffix(&run.snapshot.identity.run_id, &step.id);
    let prepared =
        prepare_atomic_github_repository_metadata_read(GitHubRepositoryAtomicLifecycleInput {
            backend,
            execution_binding,
            contract,
            correlation_id: request.execution.execution.correlation_id.clone(),
            workflow_lifecycle: Some(lifecycle),
            window_id: crate::AuthorizedExecutionWindowId::new(format!("window/{suffix}"))?,
            attempt_id: crate::AuthorizedExecutionAttemptId::new(format!("attempt/{suffix}"))?,
            opening_operation_id: crate::OperationalExecutionWindowOpeningOperationId::new(
                format!("opening/{suffix}"),
            )?,
            opening_receipt_id: crate::OperationalExecutionWindowOpeningReceiptId::new(format!(
                "opening-receipt/{suffix}"
            ))?,
            expires_at: request.expires_at,
            maximum_attempts: 1,
            trusted_time: trusted_time_observation(
                request.observed_at,
                TrustedTimeSourceKind::CoreInjectedClockV1,
                SpecContentHash::new(crate::sqlite_state::CONTINUITY_CLOCK_PROVENANCE)?,
                ContinuityTrustedTimeEpochId::new(crate::sqlite_state::CONTINUITY_CLOCK_EPOCH)?,
            ),
            evaluated_at: request.observed_at,
            redaction: RedactionMetadata::empty(),
            persistence: TrustedHostSupervisorPersistenceInput {
                operation: ContinuityOperationId::new(format!("supervisor/{suffix}"))?,
                receipt: ContinuityReceiptId::new(format!("supervisor-receipt/{suffix}"))?,
                yield_generation: None,
            },
            operation_mode: AdapterOperationMode::Fixture,
            environment: |_key: &str| None,
            operation: FixtureOperation {
                backend,
                fixture_json: request.fixture_json.clone(),
                actor: request.execution.execution.actor.clone(),
                step_id: step.id.clone(),
                skill_id: skill.id.clone(),
                skill_version: skill.version.clone(),
            },
        })?;
    let (session, _cancellation) = prepared.into_parts();
    let outcome = session.run().map_err(|_| {
        fixture_error(
            "session_failed",
            "fixture provider session failed before a bounded outcome was available",
        )
    })?;
    let run = backend.rehydrate_run(&run.snapshot.identity.run_id)?;
    let posture = match outcome {
        crate::TrustedHostLocalApplicationOutcome::EntryStopped(
            crate::TrustedHostLocalApplicationEntryStopReason::Terminal,
        )
        | crate::TrustedHostLocalApplicationOutcome::ContinuationStopped(
            crate::TrustedHostLocalApplicationContinuationStopReason::Terminal,
        ) => GitHubRepositoryMetadataFixtureExecutionPosture::Completed,
        crate::TrustedHostLocalApplicationOutcome::EntryStopped(
            crate::TrustedHostLocalApplicationEntryStopReason::AwaitCondition,
        ) => GitHubRepositoryMetadataFixtureExecutionPosture::RecoveryRequired,
        _ => GitHubRepositoryMetadataFixtureExecutionPosture::Blocked,
    };
    Ok(result(run, posture))
}

fn validate_fixture_plan(
    plan: &ExecutionPlan,
    selected_step_id: &StepId,
) -> Result<(), WorkflowOsError> {
    if plan.steps.len() != 1
        || &plan.step.id != selected_step_id
        || plan.skill.adapter_requirements.len() != 1
        || plan.skill.adapter_requirements[0].adapter_id.as_str() != "symbolic/github-read-only"
        || plan.skill.adapter_requirements[0]
            .operation
            .as_ref()
            .map(AdapterOperationId::as_str)
            != Some(OPERATION_CODE)
    {
        return Err(fixture_error(
            "operation_unsupported",
            "fixture execution requires the exact GitHub repository metadata workflow",
        ));
    }
    Ok(())
}

fn fixture_context_commitment(fixture_json: &str) -> Result<SpecContentHash, WorkflowOsError> {
    let mut hasher = Sha256::new();
    hasher.update(b"workflow-os/github-repository-metadata-fixture/v1\0");
    hasher.update(fixture_json.as_bytes());
    SpecContentHash::new(format!("{:x}", hasher.finalize()))
}

fn result(
    run: WorkflowRun,
    posture: GitHubRepositoryMetadataFixtureExecutionPosture,
) -> GitHubRepositoryMetadataFixtureExecutionResult {
    GitHubRepositoryMetadataFixtureExecutionResult { run, posture }
}

fn fixture_error(suffix: &'static str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::invalid_state(
        format!("executor.github_repository_metadata.fixture.{suffix}"),
        message,
    )
}
