use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::{
    load_project, validate_loaded_project, Action, ActorId, ApprovalDecision, ApprovalDecisionKind,
    ApprovalRequest, AuditEvent, AuditSink, AutonomyLevel, CancellationRecord, Capability,
    ConservativePolicyEngine, CorrelationId, EscalationRecord, EventId, EventSequenceNumber,
    FailureClass, FailureRecord, IdempotencyKey, IdempotencyResult, IdempotencyWrite, LoadedSpec,
    LocalAuditSink, LocalObservabilitySink, LocalStructuredLogger, MappingExpression,
    ObservabilityEvent, ObservabilitySink, PolicyAuditRecord, PolicyAuditScope, PolicyDecision,
    PolicyEvaluationContext, PolicySpecDocument, RedactionDisposition, RedactionFieldState,
    RedactionMetadata, RetryRecord, SchemaVersion, SkillAttemptId, SkillDefinition, SkillId,
    SkillInvocation, SkillInvocationAttempt, SkillInvocationId, SkillVersion, StateBackend,
    StepDefinition, StepId, StructuredLogRecord, StructuredLogger, TimeoutBehavior, Timestamp,
    ValueMapping, WorkflowDefinition, WorkflowId, WorkflowOsError, WorkflowOsErrorKind,
    WorkflowRun, WorkflowRunEvent, WorkflowRunEventKind, WorkflowRunId, WorkflowRunStatus,
    WorkflowVersion,
};

/// Input passed to a local skill handler.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SkillInput {
    /// Workflow run ID.
    pub run_id: WorkflowRunId,
    /// Workflow ID.
    pub workflow_id: WorkflowId,
    /// Workflow version.
    pub workflow_version: WorkflowVersion,
    /// Step ID.
    pub step_id: StepId,
    /// Skill ID.
    pub skill_id: SkillId,
    /// Skill version.
    pub skill_version: SkillVersion,
    /// Correlation ID propagated from the execution request.
    pub correlation_id: CorrelationId,
    /// Non-secret input values derived from supported literal mappings.
    pub values: BTreeMap<String, String>,
}

/// Output returned by a local skill handler.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SkillOutput {
    /// Non-secret output values. Sensitive payloads must be represented by references.
    pub values: BTreeMap<String, String>,
    /// Non-secret output reference or summary stored in runtime events.
    pub output_ref: Option<String>,
}

impl SkillOutput {
    /// Creates a skill output from values and an optional non-secret reference.
    #[must_use]
    pub fn new(values: BTreeMap<String, String>, output_ref: Option<String>) -> Self {
        Self { values, output_ref }
    }
}

/// Local skill handler interface for deterministic test and development execution.
pub trait SkillHandler {
    /// Invokes a local skill.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the skill cannot produce a valid output.
    fn invoke(&self, input: SkillInput) -> Result<SkillOutput, WorkflowOsError>;
}

/// Registry for local skill handlers.
#[derive(Default)]
pub struct LocalSkillRegistry {
    handlers: BTreeMap<(SkillId, SkillVersion), Box<dyn SkillHandler>>,
}

impl LocalSkillRegistry {
    /// Creates an empty local skill registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers one local skill handler.
    pub fn register(
        &mut self,
        skill_id: SkillId,
        skill_version: SkillVersion,
        handler: Box<dyn SkillHandler>,
    ) {
        self.handlers.insert((skill_id, skill_version), handler);
    }

    fn get(&self, skill_id: &SkillId, skill_version: &SkillVersion) -> Option<&dyn SkillHandler> {
        self.handlers
            .get(&(skill_id.clone(), skill_version.clone()))
            .map(std::convert::AsRef::as_ref)
    }
}

/// Request to execute one local workflow run.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalExecutionRequest {
    /// Project root containing `workflow-os.yml`.
    pub project_root: PathBuf,
    /// Workflow ID to run.
    pub workflow_id: WorkflowId,
    /// Optional explicit run ID. Reusing an existing run ID returns the durable run.
    pub run_id: Option<WorkflowRunId>,
    /// Runtime correlation ID.
    pub correlation_id: CorrelationId,
    /// Actor requesting local execution.
    pub actor: ActorId,
}

/// Request to submit a local manual approval decision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalApprovalDecisionRequest {
    /// Project root containing the immutable workflow definition used by the run.
    pub project_root: PathBuf,
    /// Workflow run ID awaiting approval.
    pub run_id: WorkflowRunId,
    /// Approval request ID being decided.
    pub approval_id: String,
    /// Approval decision.
    pub decision: ApprovalDecisionKind,
    /// Human actor making the decision.
    pub actor: ActorId,
    /// Non-secret reason for the decision.
    pub reason: String,
    /// Correlation ID for audit and logs.
    pub correlation_id: CorrelationId,
}

/// Request to cancel a local workflow run.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalCancellationRequest {
    /// Workflow run ID to cancel.
    pub run_id: WorkflowRunId,
    /// Actor requesting cancellation.
    pub actor: ActorId,
    /// Non-secret cancellation reason.
    pub reason: String,
    /// Correlation ID for audit and logs.
    pub correlation_id: CorrelationId,
}

/// Runtime timeout classification for the local executor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalTimeoutPolicy {
    /// Human-authored maximum duration.
    pub max_duration: String,
    /// Declared behavior when timeout is detected.
    pub on_timeout: TimeoutBehavior,
    /// Failure class associated with timeout handling.
    pub failure_class: FailureClass,
}

/// Minimal local executor for a single-step, local-handler workflow.
pub struct LocalExecutor<
    'a,
    B,
    A = LocalAuditSink,
    O = LocalObservabilitySink,
    L = LocalStructuredLogger,
> where
    B: StateBackend,
    A: AuditSink,
    O: ObservabilitySink,
    L: StructuredLogger,
{
    backend: &'a B,
    registry: &'a LocalSkillRegistry,
    policy_engine: ConservativePolicyEngine,
    audit_sink: A,
    observability_sink: O,
    logger: L,
}

impl<'a, B> LocalExecutor<'a, B>
where
    B: StateBackend,
{
    /// Creates a local executor over a durable state backend and local handler registry.
    #[must_use]
    pub fn new(backend: &'a B, registry: &'a LocalSkillRegistry) -> Self {
        Self {
            backend,
            registry,
            policy_engine: ConservativePolicyEngine::new(),
            audit_sink: LocalAuditSink::new(),
            observability_sink: LocalObservabilitySink::new(),
            logger: LocalStructuredLogger::new(),
        }
    }

    /// Creates a local executor with an explicit policy engine.
    #[must_use]
    pub fn new_with_policy(
        backend: &'a B,
        registry: &'a LocalSkillRegistry,
        policy_engine: ConservativePolicyEngine,
    ) -> Self {
        Self {
            backend,
            registry,
            policy_engine,
            audit_sink: LocalAuditSink::new(),
            observability_sink: LocalObservabilitySink::new(),
            logger: LocalStructuredLogger::new(),
        }
    }
}

impl<'a, B, A, O, L> LocalExecutor<'a, B, A, O, L>
where
    B: StateBackend,
    A: AuditSink,
    O: ObservabilitySink,
    L: StructuredLogger,
{
    /// Creates a local executor with explicit audit, observability, and logging sinks.
    #[must_use]
    pub fn new_with_sinks(
        backend: &'a B,
        registry: &'a LocalSkillRegistry,
        policy_engine: ConservativePolicyEngine,
        audit_sink: A,
        observability_sink: O,
        logger: L,
    ) -> Self {
        Self {
            backend,
            registry,
            policy_engine,
            audit_sink,
            observability_sink,
            logger,
        }
    }

    /// Loads, validates, and executes a single-step local workflow.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the project cannot be loaded or
    /// validated, the workflow is outside the v0 executor scope, state cannot be
    /// persisted, a local handler is missing, or output contract checks fail.
    pub fn execute(&self, request: &LocalExecutionRequest) -> Result<WorkflowRun, WorkflowOsError> {
        let run_id = request
            .run_id
            .clone()
            .unwrap_or_else(WorkflowRunId::generate);
        if !self.backend.read_events(&run_id)?.is_empty() {
            return self.backend.rehydrate_run(&run_id);
        }

        let mut plan = Self::prepare_execution(request, run_id)?;
        self.evaluate_pre_run_policy(&plan, &request.actor, &request.correlation_id)?;
        self.append_run_start(&mut plan)?;
        if plan.step.approval_policy.is_some() {
            return self.pause_for_approval(plan);
        }
        self.invoke_local_skill(plan, &request.correlation_id)
    }

    /// Applies a local approval decision and resumes approved runs.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the run is not waiting for approval, the
    /// approval ID is unknown or already decided, the run is terminal, or
    /// durable state cannot be updated.
    pub fn decide_approval(
        &self,
        request: LocalApprovalDecisionRequest,
    ) -> Result<WorkflowRun, WorkflowOsError> {
        let run = self.backend.rehydrate_run(&request.run_id)?;
        if run.snapshot.status.is_terminal() {
            return Err(executor_error(
                WorkflowOsErrorKind::InvalidState,
                "executor.approval.terminal",
                "approval decisions after a terminal state are rejected",
            ));
        }
        if run.snapshot.status != WorkflowRunStatus::WaitingForApproval {
            return Err(executor_error(
                WorkflowOsErrorKind::InvalidState,
                "executor.approval.not_waiting",
                "run is not waiting for approval",
            ));
        }

        let approval = self.event_backed_approval(&run, &request.approval_id)?;
        if approval.decision.is_some() {
            return Err(executor_error(
                WorkflowOsErrorKind::InvalidState,
                "executor.approval.duplicate_decision",
                format!("approval {} already has a decision", request.approval_id),
            ));
        }

        let mut builder = EventBuilder::from_snapshot(
            &run.snapshot,
            request.correlation_id.clone(),
            request.actor.clone(),
        );
        let decision = ApprovalDecision {
            approval_id: request.approval_id.clone(),
            actor: request.actor,
            decided_at: Timestamp::now_utc(),
            decision: request.decision,
            reason: request.reason,
            correlation_id: request.correlation_id.clone(),
        };

        match decision.decision {
            ApprovalDecisionKind::Granted => {
                self.append(
                    &mut builder,
                    WorkflowRunEventKind::ApprovalGranted(decision),
                    None,
                )?;
                let resume_context = PolicyEvaluationContext {
                    action: Action::ResumeWorkflow,
                    capabilities: vec![Capability::WorkflowResume, Capability::AuditWrite],
                    actor: Some(builder.actor.clone()),
                    workflow_id: Some(builder.workflow_id.clone()),
                    run_id: Some(builder.run_id.clone()),
                    step_id: Some(approval.step_id.clone()),
                    skill_id: Some(approval.skill_id.clone()),
                    autonomy_level: None,
                    approval_sensitivity: None,
                    has_approval_policy: false,
                    adapter_id: None,
                    correlation_id: Some(builder.correlation_id.clone()),
                };
                self.evaluate_and_record_policy(&mut builder, &resume_context)?;
                self.append(&mut builder, WorkflowRunEventKind::RunResumed, None)?;
                let plan = Self::prepare_resume_execution(&request.project_root, builder)?;
                self.invoke_local_skill(plan, &request.correlation_id)
            }
            ApprovalDecisionKind::Denied => {
                self.append(
                    &mut builder,
                    WorkflowRunEventKind::ApprovalDenied(decision),
                    None,
                )?;
                self.fail_run(
                    builder,
                    "executor.approval.denied",
                    "approval was denied; run failed closed",
                )
            }
        }
    }

    fn event_backed_approval(
        &self,
        run: &WorkflowRun,
        approval_id: &str,
    ) -> Result<ApprovalRequest, WorkflowOsError> {
        let approval = run
            .snapshot
            .approval_requests
            .iter()
            .find(|approval| approval.approval_id == approval_id)
            .cloned()
            .ok_or_else(|| {
                executor_error(
                    WorkflowOsErrorKind::Validation,
                    "executor.approval.not_found",
                    format!("approval {approval_id} was not found"),
                )
            })?;
        if let Some(projection) = self.backend.load_approval_request(approval_id)? {
            if projection.run_id != run.snapshot.identity.run_id {
                return Err(executor_error(
                    WorkflowOsErrorKind::InvalidState,
                    "executor.approval.projection_mismatch",
                    format!(
                        "approval projection {approval_id} does not match run {}",
                        run.snapshot.identity.run_id
                    ),
                ));
            }
        } else {
            self.backend.save_approval_request(&approval)?;
        }
        Ok(approval)
    }

    /// Cancels a non-terminal local workflow run.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the run is terminal or durable state
    /// cannot be updated.
    pub fn cancel_run(
        &self,
        request: LocalCancellationRequest,
    ) -> Result<WorkflowRun, WorkflowOsError> {
        let run = self.backend.rehydrate_run(&request.run_id)?;
        if run.snapshot.status.is_terminal() {
            return Err(executor_error(
                WorkflowOsErrorKind::InvalidState,
                "executor.cancellation.terminal",
                "terminal runs cannot be canceled",
            ));
        }
        let mut builder = EventBuilder::from_snapshot(
            &run.snapshot,
            request.correlation_id.clone(),
            request.actor.clone(),
        );
        let cancel_context = PolicyEvaluationContext {
            action: Action::CancelWorkflow,
            capabilities: vec![Capability::WorkflowCancel, Capability::AuditWrite],
            actor: Some(request.actor.clone()),
            workflow_id: Some(builder.workflow_id.clone()),
            run_id: Some(request.run_id.clone()),
            step_id: None,
            skill_id: None,
            autonomy_level: None,
            approval_sensitivity: None,
            has_approval_policy: false,
            adapter_id: None,
            correlation_id: Some(request.correlation_id.clone()),
        };
        self.evaluate_and_record_policy(&mut builder, &cancel_context)?;
        self.append(
            &mut builder,
            WorkflowRunEventKind::RunCanceled(CancellationRecord {
                run_id: request.run_id,
                reason: request.reason,
                actor: request.actor,
                canceled_at: Timestamp::now_utc(),
                correlation_id: request.correlation_id,
            }),
            None,
        )?;
        self.rehydrate_and_project(&builder.run_id)
    }

    /// Loads and classifies the workflow timeout policy without starting a run.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the project cannot be loaded or validated.
    pub fn timeout_policy_for_project(
        project_root: PathBuf,
        workflow_id: WorkflowId,
    ) -> Result<Option<LocalTimeoutPolicy>, WorkflowOsError> {
        let request = LocalExecutionRequest {
            project_root,
            workflow_id,
            run_id: Some(WorkflowRunId::generate()),
            correlation_id: CorrelationId::generate(),
            actor: ActorId::new("system/timeout-inspection").map_err(|error| {
                executor_error(
                    WorkflowOsErrorKind::Internal,
                    "executor.actor.invalid",
                    error.to_string(),
                )
            })?,
        };
        let plan = Self::prepare_execution(&request, WorkflowRunId::generate())?;
        Ok(plan.timeout_policy)
    }

    fn prepare_execution(
        request: &LocalExecutionRequest,
        run_id: WorkflowRunId,
    ) -> Result<ExecutionPlan, WorkflowOsError> {
        let load_result = load_project(&request.project_root);
        if load_result.has_errors() {
            return Err(WorkflowOsError::validation(
                "executor.project.load_failed",
                "project could not be loaded for local execution",
            )
            .with_diagnostics(load_result.diagnostics));
        }

        let validation = validate_loaded_project(&load_result);
        if validation.has_errors() {
            return Err(WorkflowOsError::validation(
                "executor.project.invalid",
                "project failed deterministic validation before execution",
            )
            .with_diagnostics(validation.diagnostics));
        }

        let bundle = load_result.bundle.ok_or_else(|| {
            executor_error(
                WorkflowOsErrorKind::InvalidState,
                "executor.project.bundle_missing",
                "loader produced no project bundle",
            )
        })?;
        let workflow = find_workflow(&bundle.workflows, &request.workflow_id)?;
        let step = single_step(workflow)?;
        let skill = resolve_skill(&bundle.skills, step)?;
        let retry_max_attempts = retry_max_attempts(step, &bundle.policies);
        let escalation_enabled = step.escalation_policy.is_some();
        let escalation_contact = workflow.definition.owner.escalation_contact.clone();
        let timeout_policy =
            workflow
                .definition
                .timeout_policy
                .as_ref()
                .map(|policy| LocalTimeoutPolicy {
                    max_duration: policy.max_duration.duration.clone(),
                    on_timeout: policy.on_timeout,
                    failure_class: FailureClass::Timeout,
                });

        let workflow_id = workflow.definition.id.clone();
        let schema_version = workflow.definition.schema_version.clone();
        let workflow_version = workflow.definition.version.clone();
        let spec_hash = workflow.content_hash.clone();
        let skill_id = skill.definition.id.clone();
        let skill_version = skill.definition.version.clone();
        let approval_expires_after =
            workflow
                .definition
                .approval_requirements
                .iter()
                .find_map(|requirement| {
                    requirement
                        .expires_after
                        .as_ref()
                        .map(|duration| duration.duration.clone())
                });
        let invocation_id = SkillInvocationId::generate();
        let idempotency_key = invocation_idempotency_key(
            &run_id,
            &workflow_id,
            &workflow_version,
            &step.id,
            &skill_id,
            &skill_version,
        )?;

        Ok(ExecutionPlan {
            event_builder: EventBuilder {
                next_sequence_number: EventSequenceNumber::first(),
                run_id,
                workflow_id,
                schema_version,
                workflow_version,
                spec_hash,
                correlation_id: request.correlation_id.clone(),
                actor: request.actor.clone(),
            },
            step: step.clone(),
            skill: skill.definition.clone(),
            skill_id,
            skill_version,
            invocation_id,
            idempotency_key,
            approval_expires_after,
            retry_max_attempts,
            escalation_enabled,
            escalation_contact,
            timeout_policy,
            autonomy_level: workflow.definition.autonomy_level,
            approval_sensitivity: skill.definition.approval_sensitivity,
            adapter_id: skill
                .definition
                .adapter_requirements
                .first()
                .map(|adapter| adapter.adapter_id.to_string()),
            capabilities: capabilities_for_skill(&skill.definition),
        })
    }

    fn append_run_start(&self, plan: &mut ExecutionPlan) -> Result<(), WorkflowOsError> {
        self.append(
            &mut plan.event_builder,
            WorkflowRunEventKind::RunCreated {
                summary: Some("local executor run created".to_owned()),
            },
            None,
        )?;
        self.append(
            &mut plan.event_builder,
            WorkflowRunEventKind::RunValidated,
            None,
        )?;
        self.append(
            &mut plan.event_builder,
            WorkflowRunEventKind::RunStarted,
            None,
        )?;
        self.append(
            &mut plan.event_builder,
            WorkflowRunEventKind::StepScheduled {
                step_id: plan.step.id.clone(),
            },
            None,
        )?;

        Ok(())
    }

    fn pause_for_approval(&self, mut plan: ExecutionPlan) -> Result<WorkflowRun, WorkflowOsError> {
        let approval_context = PolicyEvaluationContext {
            action: Action::RequestApproval,
            capabilities: vec![Capability::ApprovalRequest, Capability::AuditWrite],
            actor: Some(plan.event_builder.actor.clone()),
            workflow_id: Some(plan.event_builder.workflow_id.clone()),
            run_id: Some(plan.event_builder.run_id.clone()),
            step_id: Some(plan.step.id.clone()),
            skill_id: Some(plan.skill_id.clone()),
            autonomy_level: Some(plan.autonomy_level),
            approval_sensitivity: Some(plan.approval_sensitivity),
            has_approval_policy: true,
            adapter_id: None,
            correlation_id: Some(plan.event_builder.correlation_id.clone()),
        };
        if let Err(error) =
            self.evaluate_and_record_policy(&mut plan.event_builder, &approval_context)
        {
            return self.fail_run(
                plan.event_builder,
                error.code().to_owned(),
                error.message().to_owned(),
            );
        }
        let approval = ApprovalRequest {
            approval_id: approval_id(&plan.event_builder.run_id, &plan.step.id),
            run_id: plan.event_builder.run_id.clone(),
            workflow_id: plan.event_builder.workflow_id.clone(),
            schema_version: plan.event_builder.schema_version.clone(),
            workflow_version: plan.event_builder.workflow_version.clone(),
            spec_content_hash: plan.event_builder.spec_hash.clone(),
            step_id: plan.step.id.clone(),
            skill_id: plan.skill_id.clone(),
            skill_version: plan.skill_version.clone(),
            requested_by: plan.event_builder.actor.clone(),
            correlation_id: plan.event_builder.correlation_id.clone(),
            idempotency_key: Some(plan.idempotency_key.clone()),
            reason: "step requires explicit approval before local skill execution".to_owned(),
            requested_at: Timestamp::now_utc(),
            expires_after: plan.approval_expires_after.clone(),
            expires_at: None,
            decision: None,
        };
        let projection = approval.clone();
        self.append(
            &mut plan.event_builder,
            WorkflowRunEventKind::ApprovalRequested(Box::new(approval)),
            None,
        )?;
        self.backend.save_approval_request(&projection)?;
        self.rehydrate_and_project(&plan.event_builder.run_id)
    }

    fn prepare_resume_execution(
        project_root: &std::path::Path,
        builder: EventBuilder,
    ) -> Result<ExecutionPlan, WorkflowOsError> {
        let request = LocalExecutionRequest {
            project_root: project_root.to_path_buf(),
            workflow_id: builder.workflow_id.clone(),
            run_id: Some(builder.run_id.clone()),
            correlation_id: builder.correlation_id.clone(),
            actor: builder.actor.clone(),
        };
        let mut plan = Self::prepare_execution(&request, builder.run_id.clone())?;
        plan.event_builder = builder;
        Ok(plan)
    }

    fn invoke_local_skill(
        &self,
        mut plan: ExecutionPlan,
        correlation_id: &CorrelationId,
    ) -> Result<WorkflowRun, WorkflowOsError> {
        let invoke_context = PolicyEvaluationContext {
            action: if plan.adapter_id.is_some() {
                Action::InvokeAdapter
            } else {
                Action::InvokeSkill
            },
            capabilities: plan.capabilities.clone(),
            actor: Some(plan.event_builder.actor.clone()),
            workflow_id: Some(plan.event_builder.workflow_id.clone()),
            run_id: Some(plan.event_builder.run_id.clone()),
            step_id: Some(plan.step.id.clone()),
            skill_id: Some(plan.skill_id.clone()),
            autonomy_level: Some(plan.autonomy_level),
            approval_sensitivity: Some(plan.approval_sensitivity),
            has_approval_policy: plan.step.approval_policy.is_some(),
            adapter_id: plan.adapter_id.clone(),
            correlation_id: Some(plan.event_builder.correlation_id.clone()),
        };
        if let Err(error) =
            self.evaluate_and_record_policy(&mut plan.event_builder, &invoke_context)
        {
            return self.fail_run(
                plan.event_builder,
                error.code().to_owned(),
                error.message().to_owned(),
            );
        }
        let Some(handler) = self.registry.get(&plan.skill_id, &plan.skill_version) else {
            return self.fail_run(
                plan.event_builder,
                "executor.skill_handler.missing",
                format!(
                    "no local handler registered for {}@{}",
                    plan.skill_id, plan.skill_version
                ),
            );
        };
        self.append_skill_invocation_requested(&mut plan)?;

        let input_values = build_input_values(&plan.step.input_mapping)?;
        let max_attempts = plan.retry_max_attempts;
        for attempt_number in 1..=max_attempts {
            let attempt_key = attempt_idempotency_key(&plan.idempotency_key, attempt_number)?;
            let idempotency = self.backend.record_idempotency_result(
                &attempt_key,
                IdempotencyResult {
                    result_ref: format!("{}:{attempt_number}", plan.invocation_id),
                },
            )?;
            if matches!(idempotency, IdempotencyWrite::Duplicate(_)) {
                return self.backend.rehydrate_run(&plan.event_builder.run_id);
            }

            self.append_skill_attempt_started(&mut plan, attempt_number, &attempt_key)?;
            let input = SkillInput {
                run_id: plan.event_builder.run_id.clone(),
                workflow_id: plan.event_builder.workflow_id.clone(),
                workflow_version: plan.event_builder.workflow_version.clone(),
                step_id: plan.step.id.clone(),
                skill_id: plan.skill_id.clone(),
                skill_version: plan.skill_version.clone(),
                correlation_id: correlation_id.clone(),
                values: input_values.clone(),
            };

            match handler.invoke(input) {
                Ok(output) => return self.record_skill_success(plan, output, attempt_key),
                Err(error) if attempt_number < max_attempts => {
                    self.record_attempt_failure(&mut plan, &error, &attempt_key)?;
                    self.schedule_retry(&mut plan, attempt_number + 1, max_attempts, &error)?;
                }
                Err(error) => {
                    self.record_attempt_failure(&mut plan, &error, &attempt_key)?;
                    return self.exhaust_retries(plan, max_attempts, &error);
                }
            }
        }
        self.fail_run(
            plan.event_builder,
            "executor.retry.invalid_state",
            "retry loop ended without a terminal runtime event",
        )
    }

    fn append_skill_invocation_requested(
        &self,
        plan: &mut ExecutionPlan,
    ) -> Result<(), WorkflowOsError> {
        let invocation = SkillInvocation {
            invocation_id: plan.invocation_id.clone(),
            step_id: plan.step.id.clone(),
            skill_id: plan.skill_id.clone(),
            skill_version: plan.skill_version.clone(),
            idempotency_key: Some(plan.idempotency_key.clone()),
            attempts: Vec::new(),
        };
        self.append(
            &mut plan.event_builder,
            WorkflowRunEventKind::SkillInvocationRequested(invocation),
            Some(plan.idempotency_key.clone()),
        )
    }

    fn append_skill_attempt_started(
        &self,
        plan: &mut ExecutionPlan,
        attempt_number: u32,
        attempt_key: &IdempotencyKey,
    ) -> Result<(), WorkflowOsError> {
        let attempt = SkillInvocationAttempt {
            invocation_id: plan.invocation_id.clone(),
            attempt_id: SkillAttemptId::generate(),
            step_id: plan.step.id.clone(),
            skill_id: plan.skill_id.clone(),
            skill_version: plan.skill_version.clone(),
            attempt_number,
        };
        self.append(
            &mut plan.event_builder,
            WorkflowRunEventKind::SkillInvocationStarted(attempt),
            Some(attempt_key.clone()),
        )
    }

    fn record_skill_success(
        &self,
        mut plan: ExecutionPlan,
        output: SkillOutput,
        attempt_key: IdempotencyKey,
    ) -> Result<WorkflowRun, WorkflowOsError> {
        if let Err(error) = validate_required_outputs(&plan.skill, &output) {
            self.record_attempt_failure(&mut plan, &error, &attempt_key)?;
            let attempts = plan.retry_max_attempts;
            return self.exhaust_retries(plan, attempts, &error);
        }
        self.append(
            &mut plan.event_builder,
            WorkflowRunEventKind::SkillInvocationSucceeded {
                invocation_id: plan.invocation_id,
                step_id: plan.step.id.clone(),
                skill_id: plan.skill_id.clone(),
                skill_version: plan.skill_version.clone(),
                output_ref: output.output_ref,
            },
            Some(attempt_key),
        )?;
        self.append(
            &mut plan.event_builder,
            WorkflowRunEventKind::RunCompleted,
            None,
        )?;
        self.rehydrate_and_project(&plan.event_builder.run_id)
    }

    fn record_attempt_failure(
        &self,
        plan: &mut ExecutionPlan,
        error: &WorkflowOsError,
        attempt_key: &IdempotencyKey,
    ) -> Result<(), WorkflowOsError> {
        self.append(
            &mut plan.event_builder,
            WorkflowRunEventKind::SkillInvocationFailed {
                invocation_id: plan.invocation_id.clone(),
                step_id: plan.step.id.clone(),
                skill_id: plan.skill_id.clone(),
                skill_version: plan.skill_version.clone(),
                failure: FailureRecord {
                    code: error.code().to_owned(),
                    message: error.message().to_owned(),
                    failure_class: classify_failure(error),
                },
            },
            Some(attempt_key.clone()),
        )
    }

    fn schedule_retry(
        &self,
        plan: &mut ExecutionPlan,
        next_attempt: u32,
        max_attempts: u32,
        error: &WorkflowOsError,
    ) -> Result<(), WorkflowOsError> {
        let retry_key = retry_idempotency_key(&plan.idempotency_key, next_attempt)?;
        let record = retry_record(plan, next_attempt, max_attempts, error);
        self.append(
            &mut plan.event_builder,
            WorkflowRunEventKind::RetryScheduled(record.clone()),
            Some(retry_key.clone()),
        )?;
        self.append(
            &mut plan.event_builder,
            WorkflowRunEventKind::RetryStarted(record),
            Some(retry_key),
        )
    }

    fn exhaust_retries(
        &self,
        mut plan: ExecutionPlan,
        attempts: u32,
        error: &WorkflowOsError,
    ) -> Result<WorkflowRun, WorkflowOsError> {
        let retry_key = retry_idempotency_key(&plan.idempotency_key, attempts)?;
        let exhausted = retry_record(&plan, attempts, attempts, error);
        self.append(
            &mut plan.event_builder,
            WorkflowRunEventKind::RetryExhausted(exhausted),
            Some(retry_key),
        )?;
        if plan.escalation_enabled {
            let escalation = escalation_record(&plan, attempts, error);
            self.append(
                &mut plan.event_builder,
                WorkflowRunEventKind::EscalationTriggered(escalation),
                None,
            )?;
            return self.rehydrate_and_project(&plan.event_builder.run_id);
        }
        self.fail_run(
            plan.event_builder,
            error.code().to_owned(),
            error.message().to_owned(),
        )
    }

    fn append(
        &self,
        builder: &mut EventBuilder,
        kind: WorkflowRunEventKind,
        idempotency_key: Option<IdempotencyKey>,
    ) -> Result<(), WorkflowOsError> {
        let event = builder.event(kind, idempotency_key);
        self.backend.append_event(&event)?;
        self.emit_runtime_signals(&event)?;
        Ok(())
    }

    fn emit_runtime_signals(&self, event: &WorkflowRunEvent) -> Result<(), WorkflowOsError> {
        let audit_event = AuditEvent::from_workflow_event(event, "workflow-core.local-executor");
        self.audit_sink.record_audit_event(&audit_event)?;
        for observability_event in
            ObservabilityEvent::from_workflow_event(event, "workflow-core.local-executor")
        {
            self.observability_sink
                .record_observability_event(&observability_event)?;
        }
        self.logger.record_log(&structured_log_from_event(event))?;
        Ok(())
    }

    fn evaluate_pre_run_policy(
        &self,
        plan: &ExecutionPlan,
        actor: &ActorId,
        correlation_id: &CorrelationId,
    ) -> Result<(), WorkflowOsError> {
        let context = PolicyEvaluationContext {
            action: Action::StartWorkflow,
            capabilities: vec![Capability::LocalRead, Capability::AuditWrite],
            actor: Some(actor.clone()),
            workflow_id: Some(plan.event_builder.workflow_id.clone()),
            run_id: Some(plan.event_builder.run_id.clone()),
            step_id: None,
            skill_id: None,
            autonomy_level: Some(plan.autonomy_level),
            approval_sensitivity: None,
            has_approval_policy: false,
            adapter_id: None,
            correlation_id: Some(correlation_id.clone()),
        };
        let decision = self.policy_engine.evaluate(&context);
        self.record_policy_audit(PolicyAuditEmission {
            decision: &decision,
            context: &context,
            scope: PolicyAuditScope::PreRun,
            workflow_event_id: None,
            timestamp: Timestamp::now_utc(),
            builder: Some(&plan.event_builder),
            idempotency_key: None,
        })?;
        policy_result(&decision)
    }

    fn evaluate_and_record_policy(
        &self,
        builder: &mut EventBuilder,
        context: &PolicyEvaluationContext,
    ) -> Result<(), WorkflowOsError> {
        let decision = self.policy_engine.evaluate(context);
        let event = builder.event(
            WorkflowRunEventKind::PolicyDecisionRecorded(Box::new(decision.clone())),
            None,
        );
        self.backend.append_event(&event)?;
        self.emit_runtime_signals(&event)?;
        self.record_policy_audit(PolicyAuditEmission {
            decision: &decision,
            context,
            scope: PolicyAuditScope::Run,
            workflow_event_id: Some(event.event_id.clone()),
            timestamp: event.timestamp,
            builder: Some(builder),
            idempotency_key: None,
        })?;
        policy_result(&decision)
    }

    fn record_policy_audit(
        &self,
        emission: PolicyAuditEmission<'_>,
    ) -> Result<(), WorkflowOsError> {
        let policy_context = policy_audit_context(emission.decision, emission.context);
        let redaction = RedactionMetadata {
            redacted_fields: Vec::new(),
            field_states: vec![RedactionFieldState {
                field: "policy_context".to_owned(),
                disposition: RedactionDisposition::Safe,
                reason: "policy context is a non-secret summary".to_owned(),
            }],
        };
        let record = PolicyAuditRecord {
            audit_id: emission
                .workflow_event_id
                .clone()
                .unwrap_or_else(EventId::generate),
            timestamp: emission.timestamp,
            scope: emission.scope,
            workflow_event_id: emission.workflow_event_id,
            action: emission.decision.action.clone(),
            capabilities: emission.decision.capabilities.clone(),
            allowed: emission.decision.allowed,
            requires_approval: emission.decision.requires_approval,
            reason_codes: emission.decision.reason_codes.clone(),
            violations: emission
                .decision
                .violations
                .iter()
                .map(|violation| format!("{}: {}", violation.code, violation.message))
                .collect(),
            actor: emission
                .decision
                .actor
                .clone()
                .or_else(|| emission.context.actor.clone()),
            workflow_id: emission
                .decision
                .workflow_id
                .clone()
                .or_else(|| emission.context.workflow_id.clone()),
            schema_version: emission
                .builder
                .map(|builder| builder.schema_version.clone()),
            workflow_version: emission
                .builder
                .map(|builder| builder.workflow_version.clone()),
            workflow_run_id: emission
                .decision
                .run_id
                .clone()
                .or_else(|| emission.context.run_id.clone()),
            spec_hash: emission.builder.map(|builder| builder.spec_hash.clone()),
            step_id: emission.context.step_id.clone(),
            skill_id: emission.context.skill_id.clone(),
            correlation_id: emission
                .decision
                .correlation_id
                .clone()
                .or_else(|| emission.context.correlation_id.clone()),
            idempotency_key: emission.idempotency_key,
            redaction,
            policy_context,
            source_component: "workflow-core.local-executor".to_owned(),
        };
        self.backend.append_policy_audit_record(&record)?;
        self.audit_sink.record_policy_audit_record(&record)
    }

    fn fail_run(
        &self,
        mut builder: EventBuilder,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Result<WorkflowRun, WorkflowOsError> {
        self.append(
            &mut builder,
            WorkflowRunEventKind::RunFailed(FailureRecord {
                code: code.into(),
                message: message.into(),
                failure_class: FailureClass::Unknown,
            }),
            None,
        )?;
        self.rehydrate_and_project(&builder.run_id)
    }

    fn rehydrate_and_project(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<WorkflowRun, WorkflowOsError> {
        let run = self.backend.rehydrate_run(run_id)?;
        self.backend.save_snapshot(&run.snapshot)?;
        Ok(run)
    }
}

struct EventBuilder {
    next_sequence_number: EventSequenceNumber,
    run_id: WorkflowRunId,
    workflow_id: WorkflowId,
    schema_version: SchemaVersion,
    workflow_version: WorkflowVersion,
    spec_hash: crate::SpecContentHash,
    correlation_id: CorrelationId,
    actor: ActorId,
}

struct ExecutionPlan {
    event_builder: EventBuilder,
    step: StepDefinition,
    skill: SkillDefinition,
    skill_id: SkillId,
    skill_version: SkillVersion,
    invocation_id: SkillInvocationId,
    idempotency_key: IdempotencyKey,
    approval_expires_after: Option<String>,
    retry_max_attempts: u32,
    escalation_enabled: bool,
    escalation_contact: Option<ActorId>,
    timeout_policy: Option<LocalTimeoutPolicy>,
    autonomy_level: AutonomyLevel,
    approval_sensitivity: crate::ApprovalSensitivity,
    adapter_id: Option<String>,
    capabilities: Vec<Capability>,
}

struct PolicyAuditEmission<'a> {
    decision: &'a PolicyDecision,
    context: &'a PolicyEvaluationContext,
    scope: PolicyAuditScope,
    workflow_event_id: Option<EventId>,
    timestamp: Timestamp,
    builder: Option<&'a EventBuilder>,
    idempotency_key: Option<IdempotencyKey>,
}

impl EventBuilder {
    fn from_snapshot(
        snapshot: &crate::WorkflowRunSnapshot,
        correlation_id: CorrelationId,
        actor: ActorId,
    ) -> Self {
        Self {
            next_sequence_number: snapshot.last_sequence_number.next(),
            run_id: snapshot.identity.run_id.clone(),
            workflow_id: snapshot.identity.workflow_id.clone(),
            schema_version: snapshot.identity.schema_version.clone(),
            workflow_version: snapshot.identity.workflow_version.clone(),
            spec_hash: snapshot.identity.spec_content_hash.clone(),
            correlation_id,
            actor,
        }
    }

    fn event(
        &mut self,
        kind: WorkflowRunEventKind,
        idempotency_key: Option<IdempotencyKey>,
    ) -> WorkflowRunEvent {
        let event = WorkflowRunEvent {
            sequence_number: self.next_sequence_number,
            event_id: EventId::generate(),
            timestamp: Timestamp::now_utc(),
            run_id: self.run_id.clone(),
            workflow_id: self.workflow_id.clone(),
            schema_version: self.schema_version.clone(),
            workflow_version: self.workflow_version.clone(),
            spec_content_hash: self.spec_hash.clone(),
            correlation_id: Some(self.correlation_id.clone()),
            actor: Some(self.actor.clone()),
            idempotency_key,
            kind,
        };
        self.next_sequence_number = self.next_sequence_number.next();
        event
    }
}

fn find_workflow<'a>(
    workflows: &'a [LoadedSpec<WorkflowDefinition>],
    workflow_id: &WorkflowId,
) -> Result<&'a LoadedSpec<WorkflowDefinition>, WorkflowOsError> {
    workflows
        .iter()
        .find(|workflow| &workflow.definition.id == workflow_id)
        .ok_or_else(|| {
            executor_error(
                WorkflowOsErrorKind::Validation,
                "executor.workflow.not_found",
                format!("workflow {workflow_id} was not found"),
            )
        })
}

fn single_step(
    workflow: &LoadedSpec<WorkflowDefinition>,
) -> Result<&StepDefinition, WorkflowOsError> {
    match workflow.definition.steps.as_slice() {
        [step] => Ok(step),
        [] => Err(executor_error(
            WorkflowOsErrorKind::Unsupported,
            "executor.workflow.no_steps",
            "local executor requires exactly one step",
        )),
        _ => Err(executor_error(
            WorkflowOsErrorKind::Unsupported,
            "executor.workflow.multistep_unsupported",
            "local executor supports exactly one step in v0",
        )),
    }
}

fn resolve_skill<'a>(
    skills: &'a [LoadedSpec<SkillDefinition>],
    step: &StepDefinition,
) -> Result<&'a LoadedSpec<SkillDefinition>, WorkflowOsError> {
    let skill_id = &step.skill_ref.id;
    let versions = skills
        .iter()
        .filter(|skill| &skill.definition.id == skill_id)
        .collect::<Vec<_>>();
    let version = if let Some(version) = &step.skill_ref.version {
        version
    } else {
        let [skill] = versions.as_slice() else {
            return Err(executor_error(
                WorkflowOsErrorKind::Validation,
                "executor.skill_version.ambiguous",
                format!("skill {skill_id} requires an explicit version"),
            ));
        };
        &skill.definition.version
    };

    skills
        .iter()
        .find(|skill| &skill.definition.id == skill_id && &skill.definition.version == version)
        .ok_or_else(|| {
            executor_error(
                WorkflowOsErrorKind::Validation,
                "executor.skill.not_found",
                format!("skill {skill_id}@{version} was not found"),
            )
        })
}

fn invocation_idempotency_key(
    run_id: &WorkflowRunId,
    workflow_id: &WorkflowId,
    workflow_version: &WorkflowVersion,
    step_id: &StepId,
    skill_id: &SkillId,
    skill_version: &SkillVersion,
) -> Result<IdempotencyKey, WorkflowOsError> {
    IdempotencyKey::new(format!(
        "run/{run_id}/workflow/{workflow_id}/version/{workflow_version}/step/{step_id}/skill/{skill_id}/skillversion/{skill_version}"
    ))
}

fn attempt_idempotency_key(
    invocation_key: &IdempotencyKey,
    attempt_number: u32,
) -> Result<IdempotencyKey, WorkflowOsError> {
    IdempotencyKey::new(format!("{invocation_key}/attempt/{attempt_number}"))
}

fn retry_idempotency_key(
    invocation_key: &IdempotencyKey,
    attempt_number: u32,
) -> Result<IdempotencyKey, WorkflowOsError> {
    IdempotencyKey::new(format!("{invocation_key}/retry/{attempt_number}"))
}

fn approval_id(run_id: &WorkflowRunId, step_id: &StepId) -> String {
    format!("approval/{run_id}/{step_id}")
}

fn retry_max_attempts(step: &StepDefinition, policies: &[LoadedSpec<PolicySpecDocument>]) -> u32 {
    let Some(retry) = &step.retry_policy else {
        return 1;
    };
    let Some(policy) = policies
        .iter()
        .find(|policy| policy.definition.id == retry.policy.id)
    else {
        return 1;
    };
    policy
        .definition
        .rules
        .iter()
        .find_map(|rule| parse_max_attempts(&rule.effect))
        .unwrap_or(2)
        .max(1)
}

fn parse_max_attempts(effect: &str) -> Option<u32> {
    effect
        .strip_prefix("max_attempts=")
        .or_else(|| effect.strip_prefix("max_attempts:"))
        .and_then(|value| value.parse::<u32>().ok())
}

fn capabilities_for_skill(skill: &SkillDefinition) -> Vec<Capability> {
    if skill.allowed_capabilities.is_empty() && skill.adapter_requirements.is_empty() {
        return vec![
            Capability::LocalRead,
            Capability::LocalWrite,
            Capability::AuditWrite,
        ];
    }
    let mut capabilities = skill
        .allowed_capabilities
        .iter()
        .map(|capability| capability_from_name(&capability.name))
        .collect::<Vec<_>>();
    if !capabilities
        .iter()
        .any(|capability| capability == &Capability::AuditWrite)
    {
        capabilities.push(Capability::AuditWrite);
    }
    capabilities
}

fn capability_from_name(name: &str) -> Capability {
    match name {
        "local.read" => Capability::LocalRead,
        "local.write" => Capability::LocalWrite,
        "external.read" => Capability::ExternalRead,
        "external.write" => Capability::ExternalWrite,
        "approval.request" => Capability::ApprovalRequest,
        "workflow.cancel" => Capability::WorkflowCancel,
        "workflow.resume" => Capability::WorkflowResume,
        "adapter.invoke" => Capability::AdapterInvoke,
        "secret.read" => Capability::SecretRead,
        "audit.write" => Capability::AuditWrite,
        other => Capability::Unknown(other.to_owned()),
    }
}

fn policy_result(decision: &PolicyDecision) -> Result<(), WorkflowOsError> {
    if decision.allowed {
        return Ok(());
    }
    let reason = decision
        .reason_codes
        .iter()
        .find(|code| code.starts_with("policy.deny."))
        .cloned()
        .unwrap_or_else(|| "policy.deny".to_owned());
    Err(WorkflowOsError::new(
        WorkflowOsErrorKind::PolicyDenied,
        reason,
        "policy denied runtime action",
    ))
}

fn policy_audit_context(decision: &PolicyDecision, context: &PolicyEvaluationContext) -> String {
    let outcome = if decision.allowed { "allow" } else { "deny" };
    let step = context
        .step_id
        .as_ref()
        .map_or_else(|| "none".to_owned(), ToString::to_string);
    let skill = context
        .skill_id
        .as_ref()
        .map_or_else(|| "none".to_owned(), ToString::to_string);
    format!(
        "{outcome}; action={:?}; step={step}; skill={skill}; requires_approval={}; reasons={}",
        decision.action,
        decision.requires_approval,
        decision.reason_codes.join(",")
    )
}

fn retry_record(
    plan: &ExecutionPlan,
    attempt_number: u32,
    max_attempts: u32,
    error: &WorkflowOsError,
) -> RetryRecord {
    RetryRecord {
        step_id: Some(plan.step.id.clone()),
        skill_id: Some(plan.skill_id.clone()),
        skill_version: Some(plan.skill_version.clone()),
        invocation_id: Some(plan.invocation_id.clone()),
        attempt_number,
        max_attempts,
        reason: "local skill failed and bounded retry policy applies".to_owned(),
        last_error: Some(error.code().to_owned()),
        failure_class: classify_failure(error),
        suggested_next_action: "allow retry to continue or inspect the final failure".to_owned(),
    }
}

fn escalation_record(
    plan: &ExecutionPlan,
    attempts: u32,
    error: &WorkflowOsError,
) -> EscalationRecord {
    EscalationRecord {
        escalation_id: format!("escalation/{}/{}", plan.event_builder.run_id, plan.step.id),
        run_id: plan.event_builder.run_id.clone(),
        step_id: Some(plan.step.id.clone()),
        skill_id: Some(plan.skill_id.clone()),
        skill_version: Some(plan.skill_version.clone()),
        attempts,
        last_error: error.code().to_owned(),
        failure_class: classify_failure(error),
        suggested_next_action: "manual operator review required before any further action"
            .to_owned(),
        reason: "bounded retry attempts were exhausted".to_owned(),
        contact: plan.escalation_contact.clone(),
    }
}

fn classify_failure(error: &WorkflowOsError) -> FailureClass {
    if error.code().contains("transient") {
        FailureClass::Transient
    } else {
        match error.kind() {
            WorkflowOsErrorKind::PolicyDenied | WorkflowOsErrorKind::Security => {
                FailureClass::PolicyDenied
            }
            WorkflowOsErrorKind::Unsupported | WorkflowOsErrorKind::Validation => {
                FailureClass::Permanent
            }
            WorkflowOsErrorKind::Parse
            | WorkflowOsErrorKind::InvalidState
            | WorkflowOsErrorKind::Internal => FailureClass::Unknown,
        }
    }
}

fn build_input_values(
    mappings: &[ValueMapping],
) -> Result<BTreeMap<String, String>, WorkflowOsError> {
    let mut values = BTreeMap::new();
    for mapping in mappings {
        match &mapping.from {
            MappingExpression::Literal { value } => {
                values.insert(mapping.to.clone(), value.clone());
            }
            MappingExpression::Field { .. } | MappingExpression::ConfigRef { .. } => {
                return Err(executor_error(
                    WorkflowOsErrorKind::Unsupported,
                    "executor.input_mapping.unsupported",
                    "local executor only supports literal input mappings in v0",
                ));
            }
        }
    }
    Ok(values)
}

fn validate_required_outputs(
    skill: &SkillDefinition,
    output: &SkillOutput,
) -> Result<(), WorkflowOsError> {
    for required in &skill.output_contract.required {
        if !output.values.contains_key(required) {
            return Err(executor_error(
                WorkflowOsErrorKind::Validation,
                "executor.output.required_missing",
                format!(
                    "skill {} output is missing required field {required}",
                    skill.id
                ),
            ));
        }
    }
    Ok(())
}

fn structured_log_from_event(event: &WorkflowRunEvent) -> StructuredLogRecord {
    let mut fields = BTreeMap::new();
    fields.insert("event_id".to_owned(), event.event_id.to_string());
    fields.insert("event_type".to_owned(), format!("{:?}", event.kind()));
    fields.insert("workflow_id".to_owned(), event.workflow_id.to_string());
    fields.insert(
        "schema_version".to_owned(),
        event.schema_version.to_string(),
    );
    fields.insert("run_id".to_owned(), event.run_id.to_string());
    StructuredLogRecord {
        timestamp: event.timestamp,
        level: "INFO".to_owned(),
        message: "workflow runtime event emitted".to_owned(),
        correlation_id: event.correlation_id.clone(),
        fields,
        redaction: crate::RedactionMetadata::empty(),
        source_component: "workflow-core.local-executor".to_owned(),
    }
}

fn executor_error(
    kind: WorkflowOsErrorKind,
    code: impl Into<String>,
    message: impl Into<String>,
) -> WorkflowOsError {
    WorkflowOsError::new(kind, code, message)
}
