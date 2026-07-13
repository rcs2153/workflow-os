use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::local_check::{
    DocsCheckLocalHandler, LocalCheckRegistrationMode, LocalCheckRegistrationProfile,
};
use crate::{
    derive_workflow_report_artifact_approval_proof_marker_gate_policy,
    derive_workflow_report_artifact_gate_policy, discover_high_assurance_approval_disclosure,
    execute_runtime_agent_harness_hook, execute_runtime_agent_harness_hook_failed_closed,
    expose_terminal_local_work_report_result,
    generate_terminal_local_work_report_with_side_effect_discovery,
    load_github_pr_comment_proposed_side_effect_event, load_project,
    orchestrate_github_pr_comment_provider_call, persist_approval_proof_marker_projections_for_run,
    reconcile_github_pr_comment_provider_write,
    validate_and_orchestrate_github_pr_comment_live_sandbox,
    validate_approval_presentation_for_request,
    validate_github_pr_comment_provider_report_artifact_event_proof_gate,
    validate_high_assurance_approval_decision, validate_loaded_project_with_capability,
    write_report_artifact_with_explicit_integrations,
    write_work_report_artifact_with_governance_gates, Action, ActorId, AdapterRuntimeAuditRecord,
    AdapterRuntimeObservabilityRecord, AdapterTelemetryRecord, AgentHarnessHookDisclosureId,
    AgentHarnessHookInvocationId, AgentHarnessHookInvocationInput,
    AgentHarnessHookInvocationStatus, AgentHarnessHookKind, AgentHarnessHookWorkflowEvent,
    AgentHarnessHookWorkflowEventDefinition, ApprovalDecision, ApprovalDecisionKind,
    ApprovalDecisionProofEnforcementMode, ApprovalDecisionProofMarker,
    ApprovalDecisionProofMarkerDefinition, ApprovalDecisionProofValidationPolicy,
    ApprovalPresentationId, ApprovalPresentationRecord, ApprovalPresentationValidationInput,
    ApprovalProofMarkerProjectionPersistenceInput, ApprovalProofMarkerProjectionPersistencePolicy,
    ApprovalProofMarkerProjectionPersistenceResult, ApprovalReferenceId, ApprovalRequest,
    AuditEvent, AuditSink, AutonomyLevel, CancellationRecord, Capability, ConservativePolicyEngine,
    CorrelationId, EscalationRecord, EventId, EventSequenceNumber, EvidenceReferenceId,
    FailureClass, FailureRecord, GitHubPullRequestCommentLiveSandboxValidationInput,
    GitHubPullRequestCommentLiveSandboxValidationResult, GitHubPullRequestCommentProvider,
    GitHubPullRequestCommentProviderCallOrchestrationError,
    GitHubPullRequestCommentProviderCallOrchestrationInput,
    GitHubPullRequestCommentProviderReportArtifactEventProofGatePolicy,
    GitHubPullRequestCommentProviderWriteReconciliationCandidate,
    GitHubPullRequestCommentProviderWriteReconciliationInput,
    GitHubPullRequestCommentProviderWriteReconciliationStatus,
    GitHubPullRequestCommentReportArtifactCitationPolicy,
    GitHubPullRequestCommentSideEffectEventContext, GitHubPullRequestCommentWriteOutcome,
    GitHubPullRequestCommentWriteResponse, HighAssuranceApprovalControl,
    HighAssuranceApprovalDecisionValidationInput, HighAssuranceApprovalDisclosureDiscoveryInput,
    HighAssuranceApprovalSuppliedReference, IdempotencyKey, IdempotencyResult, IdempotencyWrite,
    LoadedSpec, LocalApprovalProofMarkerAuditProjectionStore, LocalAuditSink,
    LocalObservabilitySink, LocalStructuredLogger, MappingExpression, ObservabilityEvent,
    ObservabilitySink, PolicyAuditRecord, PolicyAuditScope, PolicyDecision, PolicyEffect,
    PolicyEffectSet, PolicyEvaluationContext, PolicySpecDocument, ProjectBundle,
    ProjectValidationCapability, RedactionDisposition, RedactionFieldState, RedactionMetadata,
    ReportArtifactWriteIntegrationInput, ReportArtifactWriteProviderIntegration, RetryRecord,
    RuntimeAgentHarnessHookInput, SchemaVersion, SideEffectApprovalLinkageFromStoreInput,
    SideEffectApprovalLinkageFromStoreResult, SideEffectApprovalLinkageStoreLoadMode,
    SideEffectAuthorityDecision, SideEffectId, SideEffectLifecycleState,
    SideEffectLifecycleTransitionResult, SideEffectMissingRecordPolicy, SideEffectRecord,
    SideEffectRecordStore, SideEffectReferenceKind, SideEffectSensitivity, SideEffectTargetKind,
    SideEffectWorkflowEvent, SkillAttemptId, SkillDefinition, SkillId, SkillInvocation,
    SkillInvocationAttempt, SkillInvocationId, SkillVersion, StateBackend, StepDefinition, StepId,
    StructuredLogRecord, StructuredLogger, TerminalBehavior, TerminalLocalWorkReportInput,
    TerminalLocalWorkReportSideEffectDiscoveryInput,
    TerminalReportApprovalProofMarkerCitationPolicy, TimeoutBehavior, Timestamp, TypedHandoffId,
    ValidationReferenceId, ValueMapping, WorkReport,
    WorkReportArtifactApprovalProofMarkerGatePolicy, WorkReportArtifactGovernedWriteInput,
    WorkReportArtifactHighAssuranceDisclosureGateResult,
    WorkReportArtifactHighAssuranceDisclosurePolicy,
    WorkReportArtifactProofMarkerGovernedWriteInput,
    WorkReportArtifactProofMarkerGovernedWriteResult, WorkReportArtifactRecord,
    WorkReportArtifactSideEffectIntegrityResult, WorkReportArtifactStore, WorkReportCitationTarget,
    WorkReportContractId, WorkReportContractVersion, WorkReportHighAssuranceApprovalDisclosure,
    WorkReportId, WorkReportSensitivity, WorkReportStableReference, WorkflowDefinition, WorkflowId,
    WorkflowOsError, WorkflowOsErrorKind, WorkflowReportArtifactGateDerivationInput,
    WorkflowReportArtifactProofMarkerDerivationMode,
    WorkflowReportArtifactProofMarkerGateDerivationInput, WorkflowRun, WorkflowRunEvent,
    WorkflowRunEventKind, WorkflowRunId, WorkflowRunIdentity, WorkflowRunStatus, WorkflowVersion,
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
    /// Workflow schema version.
    pub schema_version: SchemaVersion,
    /// Workflow spec content hash.
    pub spec_hash: crate::SpecContentHash,
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
    /// Adapter telemetry produced by controlled read-only fixture handlers.
    pub adapter_telemetry: Vec<AdapterTelemetryRecord>,
}

impl SkillOutput {
    /// Creates a skill output from values and an optional non-secret reference.
    #[must_use]
    pub fn new(values: BTreeMap<String, String>, output_ref: Option<String>) -> Self {
        Self {
            values,
            output_ref,
            adapter_telemetry: Vec::new(),
        }
    }

    /// Attaches controlled adapter telemetry to this skill output.
    #[must_use]
    pub fn with_adapter_telemetry(mut self, telemetry: Vec<AdapterTelemetryRecord>) -> Self {
        self.adapter_telemetry = telemetry;
        self
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

    /// Registers the explicit docs check handler for the canonical local docs
    /// check skill.
    ///
    /// # Errors
    ///
    /// Returns an error only if the built-in skill identity is invalid.
    pub fn register_docs_check_handler(
        &mut self,
        handler: DocsCheckLocalHandler,
    ) -> Result<(), WorkflowOsError> {
        self.register(
            SkillId::new("local/check-docs")?,
            SkillVersion::new("v0")?,
            Box::new(handler),
        );
        Ok(())
    }

    /// Registers local check handlers from an explicit non-default profile.
    ///
    /// `LocalSkillRegistry::new()` remains empty. This helper exists so callers
    /// can make local check registration posture explicit without enabling
    /// ambient default registration, CLI exposure, workflow schema behavior, or
    /// broad handler discovery.
    ///
    /// # Errors
    ///
    /// Returns an error if the profile is internally inconsistent.
    pub fn register_local_check_profile(
        &mut self,
        profile: LocalCheckRegistrationProfile,
    ) -> Result<(), WorkflowOsError> {
        match profile.mode() {
            LocalCheckRegistrationMode::None => Ok(()),
            LocalCheckRegistrationMode::ExplicitDocsCheck => {
                let handler = profile.into_docs_check_handler().ok_or_else(|| {
                    WorkflowOsError::new(
                        WorkflowOsErrorKind::Validation,
                        "local_check.registration.docs_check_missing",
                        "explicit docs check registration requires a supplied handler",
                    )
                })?;
                self.register_docs_check_handler(handler)
            }
        }
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
    /// Explicit required `BeforeSkillInvocation` checkpoint policy for local skill invocation.
    pub before_skill_invocation_checkpoints: LocalExecutionBeforeSkillInvocationCheckpointInputs,
    /// Optional explicit `BeforeSkillInvocation` hook for one targeted local skill invocation.
    pub before_skill_invocation_hook: Option<LocalExecutionBeforeSkillInvocationHookInput>,
    /// Explicit side-effect lifecycle disclosures to append before local skill invocation.
    pub side_effect_events: Vec<LocalExecutionSideEffectEventInput>,
    /// Explicit store-backed side-effect lifecycle outcomes to append before local skill invocation.
    pub side_effect_lifecycle_events: Vec<LocalExecutionSideEffectLifecycleEventInput>,
}

/// Explicit required `BeforeSkillInvocation` checkpoint policy for local execution.
#[derive(Clone, Default, Eq, PartialEq)]
pub struct LocalExecutionBeforeSkillInvocationCheckpointInputs {
    /// Step IDs that must have a matching explicit `BeforeSkillInvocation` hook.
    pub required_step_ids: Vec<StepId>,
}

impl LocalExecutionBeforeSkillInvocationCheckpointInputs {
    fn requires_step(&self, step_id: &StepId) -> bool {
        self.required_step_ids
            .iter()
            .any(|required_step_id| required_step_id == step_id)
    }
}

impl fmt::Debug for LocalExecutionBeforeSkillInvocationCheckpointInputs {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalExecutionBeforeSkillInvocationCheckpointInputs")
            .field("required_step_count", &self.required_step_ids.len())
            .finish()
    }
}

/// Explicit `SideEffect` workflow event input for one targeted local skill invocation.
#[derive(Clone, Eq, PartialEq)]
pub struct LocalExecutionSideEffectEventInput {
    /// Target step ID. The event is considered only for this step.
    pub step_id: StepId,
    /// Target skill ID.
    pub skill_id: SkillId,
    /// Target skill version.
    pub skill_version: SkillVersion,
    /// Validated `SideEffect` workflow event payload to append.
    pub event: SideEffectWorkflowEvent,
}

/// Explicit attempted/completed/failed `SideEffect` workflow event input for one targeted local skill invocation.
///
/// Values of this type are constructed from a validated store-backed lifecycle
/// transition result. This keeps attempted/completed/failed append behavior
/// separate from generic caller-supplied proposal/denial/skipping disclosure.
#[derive(Clone, Eq, PartialEq)]
pub struct LocalExecutionSideEffectLifecycleEventInput {
    /// Target step ID. The event is considered only for this step.
    pub step_id: StepId,
    /// Target skill ID.
    pub skill_id: SkillId,
    /// Target skill version.
    pub skill_version: SkillVersion,
    /// Validated attempted/completed/failed `SideEffect` workflow event payload to append.
    event: SideEffectWorkflowEvent,
}

impl LocalExecutionSideEffectLifecycleEventInput {
    /// Creates a lifecycle append input from a validated store-backed transition result.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when the transition result is not an
    /// attempted/completed/failed outcome or when the transitioned record and
    /// event payload do not agree on side-effect ID or lifecycle state.
    pub fn from_transition_result(
        step_id: StepId,
        skill_id: SkillId,
        skill_version: SkillVersion,
        transition: &SideEffectLifecycleTransitionResult,
    ) -> Result<Self, WorkflowOsError> {
        let lifecycle = transition.event().lifecycle_state();
        if !matches!(
            lifecycle,
            SideEffectLifecycleState::Attempted
                | SideEffectLifecycleState::Completed
                | SideEffectLifecycleState::Failed
        ) {
            return Err(executor_error(
                WorkflowOsErrorKind::Validation,
                "executor.side_effect_lifecycle_event.unsupported_lifecycle",
                "side-effect lifecycle append input requires attempted, completed, or failed transition output",
            ));
        }
        if transition.record().side_effect_id() != transition.event().side_effect_id()
            || transition.record().lifecycle_state() != lifecycle
        {
            return Err(executor_error(
                WorkflowOsErrorKind::Validation,
                "executor.side_effect_lifecycle_event.transition_mismatch",
                "side-effect lifecycle transition output is inconsistent",
            ));
        }
        Ok(Self {
            step_id,
            skill_id,
            skill_version,
            event: transition.event().clone(),
        })
    }

    /// Returns the validated workflow event payload.
    #[must_use]
    pub const fn event(&self) -> &SideEffectWorkflowEvent {
        &self.event
    }
}

impl fmt::Debug for LocalExecutionSideEffectLifecycleEventInput {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalExecutionSideEffectLifecycleEventInput")
            .field("step_id", &"[REDACTED]")
            .field("skill_id", &"[REDACTED]")
            .field("skill_version", &"[REDACTED]")
            .field("side_effect_id", &"[REDACTED]")
            .field("lifecycle_state", &self.event.lifecycle_state())
            .field("reference_count", &self.event.references().len())
            .field(
                "evidence_reference_count",
                &self.event.evidence_reference_count(),
            )
            .field(
                "outcome_reference_count",
                &self.event.outcome_reference_count(),
            )
            .finish()
    }
}

impl fmt::Debug for LocalExecutionSideEffectEventInput {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalExecutionSideEffectEventInput")
            .field("step_id", &"[REDACTED]")
            .field("skill_id", &"[REDACTED]")
            .field("skill_version", &"[REDACTED]")
            .field("side_effect_id", &"[REDACTED]")
            .field("lifecycle_state", &self.event.lifecycle_state())
            .field("reference_count", &self.event.references().len())
            .field(
                "evidence_reference_count",
                &self.event.evidence_reference_count(),
            )
            .field(
                "outcome_reference_count",
                &self.event.outcome_reference_count(),
            )
            .finish()
    }
}

/// Explicit input for turning a persisted GitHub pull request comment proposed
/// `SideEffectRecord` into a local executor side-effect event input.
///
/// This input does not authorize provider calls, provider mutation, event append
/// by itself, report artifacts, CLI behavior, or schema behavior. It only gives
/// the helper enough explicit context to load a persisted proposed record,
/// compose a reference-only proposed event, and target the existing local
/// executor side-effect event append path.
#[derive(Clone, Eq, PartialEq)]
pub struct GitHubPullRequestCommentSideEffectAppendInput {
    /// Stable side-effect ID to load from the caller-supplied store.
    pub side_effect_id: SideEffectId,
    /// Expected workflow/run identity for the loaded record.
    pub context: GitHubPullRequestCommentSideEffectEventContext,
    /// Target step ID for the existing local executor side-effect event input.
    pub step_id: StepId,
    /// Target skill ID for the existing local executor side-effect event input.
    pub skill_id: SkillId,
    /// Target skill version for the existing local executor side-effect event input.
    pub skill_version: SkillVersion,
    /// Optional expected correlation ID for the proposed event.
    pub correlation_id: Option<CorrelationId>,
}

impl fmt::Debug for GitHubPullRequestCommentSideEffectAppendInput {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentSideEffectAppendInput")
            .field("side_effect_id", &"[REDACTED]")
            .field("context", &"[REDACTED]")
            .field("step_id", &"[REDACTED]")
            .field("skill_id", &"[REDACTED]")
            .field("skill_version", &"[REDACTED]")
            .field(
                "correlation_id",
                &self.correlation_id.as_ref().map(|_| "[REDACTED]"),
            )
            .finish()
    }
}

/// Loads a persisted GitHub pull request comment proposed side-effect record and
/// returns an explicit local executor side-effect event input.
///
/// This helper returns only `LocalExecutionSideEffectEventInput`. It does not
/// call GitHub, mutate providers, append workflow events, emit audit records,
/// write reports, or execute side effects.
///
/// # Errors
///
/// Returns stable, non-leaking errors when the persisted record cannot be
/// loaded, cannot be projected as a proposed event, or does not match the
/// explicit step/skill/correlation target.
pub fn load_github_pr_comment_proposed_side_effect_event_input(
    store: &impl SideEffectRecordStore,
    input: GitHubPullRequestCommentSideEffectAppendInput,
) -> Result<LocalExecutionSideEffectEventInput, WorkflowOsError> {
    let event = load_github_pr_comment_proposed_side_effect_event(
        store,
        &input.side_effect_id,
        &input.context,
    )
    .map_err(|error| map_github_pr_comment_side_effect_event_input_error(&error))?;

    if event
        .step_id()
        .is_some_and(|step_id| step_id != &input.step_id)
        || event
            .skill_id()
            .is_some_and(|skill_id| skill_id != &input.skill_id)
        || event
            .skill_version()
            .is_some_and(|skill_version| skill_version != &input.skill_version)
    {
        return Err(executor_error(
            WorkflowOsErrorKind::Validation,
            "github_pr_comment_side_effect_event_input.target_mismatch",
            "GitHub PR comment SideEffect event input does not match the target step or skill",
        ));
    }

    if input.correlation_id.as_ref().is_some_and(|expected| {
        event
            .correlation_id()
            .is_some_and(|actual| actual != expected)
    }) {
        return Err(executor_error(
            WorkflowOsErrorKind::Validation,
            "github_pr_comment_side_effect_event_input.correlation_mismatch",
            "GitHub PR comment SideEffect event input does not match the expected correlation",
        ));
    }

    Ok(LocalExecutionSideEffectEventInput {
        step_id: input.step_id,
        skill_id: input.skill_id,
        skill_version: input.skill_version,
        event,
    })
}

/// Explicit `BeforeSkillInvocation` hook input for one targeted local skill invocation.
#[derive(Clone, Eq, PartialEq)]
pub struct LocalExecutionBeforeSkillInvocationHookInput {
    /// Stable caller-supplied hook invocation ID.
    pub hook_invocation_id: AgentHarnessHookInvocationId,
    /// Target step ID. The hook is considered only for this step.
    pub step_id: StepId,
    /// Target skill ID.
    pub skill_id: SkillId,
    /// Target skill version.
    pub skill_version: SkillVersion,
    /// Explicit result status requested for this bounded hook checkpoint.
    pub result_status: AgentHarnessHookInvocationStatus,
    /// Explicit hook invocation context.
    pub invocation: AgentHarnessHookInvocationInput,
}

impl fmt::Debug for LocalExecutionBeforeSkillInvocationHookInput {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalExecutionBeforeSkillInvocationHookInput")
            .field("hook_invocation_id", &"[REDACTED]")
            .field("step_id", &"[REDACTED]")
            .field("skill_id", &"[REDACTED]")
            .field("skill_version", &"[REDACTED]")
            .field("hook_kind", &self.invocation.hook_kind)
            .field("result_status", &self.result_status)
            .field("workflow_id", &"[REDACTED]")
            .field("run_id", &"[REDACTED]")
            .field(
                "input_reference_count",
                &self.invocation.input_references.len(),
            )
            .field(
                "output_reference_count",
                &self.invocation.output_references.len(),
            )
            .field(
                "supplemental_reference_count",
                &self.invocation.supplemental_references.len(),
            )
            .field("disclosure_count", &self.invocation.disclosures.len())
            .finish()
    }
}

/// Explicit report inputs for executor-integrated in-memory report results.
#[derive(Clone, Eq, PartialEq)]
pub struct LocalExecutionReportInputs {
    /// Report ID.
    pub report_id: WorkReportId,
    /// Report contract ID.
    pub report_contract_id: WorkReportContractId,
    /// Report contract version.
    pub report_contract_version: WorkReportContractVersion,
    /// Report generation timestamp.
    pub generated_at: Timestamp,
    /// Actor or system actor generating the report.
    pub generated_by: ActorId,
    /// Report sensitivity.
    pub sensitivity: WorkReportSensitivity,
    /// Report redaction metadata.
    pub redaction: RedactionMetadata,
    /// Optional report-specific correlation ID override.
    pub correlation_id: Option<CorrelationId>,
    /// Existing evidence reference IDs to cite.
    pub evidence_reference_ids: Vec<EvidenceReferenceId>,
    /// Validation diagnostic/result references to cite.
    pub validation_reference_ids: Vec<ValidationReferenceId>,
    /// Stable local check result references to cite.
    pub local_check_result_references: Vec<WorkReportStableReference>,
    /// Workflow event IDs to cite.
    pub workflow_event_ids: Vec<EventId>,
    /// Audit event IDs to cite.
    pub audit_event_ids: Vec<EventId>,
    /// Stable adapter telemetry references to cite.
    pub adapter_telemetry_references: Vec<WorkReportStableReference>,
    /// Policy decision event IDs to cite.
    pub policy_event_ids: Vec<EventId>,
    /// Approval decision references to cite, where stable IDs already exist.
    pub approval_reference_ids: Vec<ApprovalReferenceId>,
    /// Optional policy for deriving approval proof-marker citations during
    /// terminal local report generation.
    pub approval_proof_marker_citation_policy:
        Option<TerminalReportApprovalProofMarkerCitationPolicy>,
    /// Optional report-safe high-assurance approval posture disclosure.
    pub high_assurance_approval: Option<WorkReportHighAssuranceApprovalDisclosure>,
    /// Typed handoff IDs to cite, where stable IDs already exist.
    pub typed_handoff_ids: Vec<TypedHandoffId>,
    /// Agent harness hook invocation IDs to cite, where stable IDs already exist.
    pub agent_harness_hook_invocation_ids: Vec<AgentHarnessHookInvocationId>,
    /// Agent harness hook disclosure IDs to cite, where stable IDs already exist.
    pub agent_harness_hook_disclosure_ids: Vec<AgentHarnessHookDisclosureId>,
    /// `SideEffect` IDs to cite, where stable IDs already exist.
    pub side_effect_ids: Vec<SideEffectId>,
    /// Bounded GitHub PR comment provider reconciliation disclosures to
    /// summarize in the report Side Effects section.
    pub github_pr_comment_provider_disclosures:
        Vec<GitHubPullRequestCommentProviderWriteReportDisclosure>,
    /// Explicit hook checkpoint enforcement policy for report generation.
    pub hook_checkpoints: LocalExecutionHookCheckpointInputs,
    /// Optional explicit `BeforeReport` hook to execute in memory before report generation.
    pub before_report_hook: Option<LocalExecutionBeforeReportHookInput>,
    /// Bounded incomplete/deferred work disclosures.
    pub incomplete_work: Vec<String>,
    /// Bounded known limitations.
    pub known_limitations: Vec<String>,
    /// Bounded risks.
    pub risks: Vec<String>,
    /// Bounded operator handoff notes.
    pub handoff_notes: Vec<String>,
}

impl fmt::Debug for LocalExecutionReportInputs {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalExecutionReportInputs")
            .field("report_id", &"[REDACTED]")
            .field("report_contract_id", &"[REDACTED]")
            .field("report_contract_version", &"[REDACTED]")
            .field("generated_at", &self.generated_at)
            .field("generated_by", &"[REDACTED]")
            .field("sensitivity", &self.sensitivity)
            .field("redaction", &"[REDACTED]")
            .field("has_correlation_id", &self.correlation_id.is_some())
            .field(
                "evidence_reference_count",
                &self.evidence_reference_ids.len(),
            )
            .field(
                "validation_reference_count",
                &self.validation_reference_ids.len(),
            )
            .field(
                "local_check_result_reference_count",
                &self.local_check_result_references.len(),
            )
            .field("workflow_event_count", &self.workflow_event_ids.len())
            .field("audit_event_count", &self.audit_event_ids.len())
            .field(
                "adapter_telemetry_reference_count",
                &self.adapter_telemetry_references.len(),
            )
            .field("policy_event_count", &self.policy_event_ids.len())
            .field(
                "approval_reference_count",
                &self.approval_reference_ids.len(),
            )
            .field(
                "has_approval_proof_marker_citation_policy",
                &self.approval_proof_marker_citation_policy.is_some(),
            )
            .field(
                "has_high_assurance_approval",
                &self.high_assurance_approval.is_some(),
            )
            .field("typed_handoff_count", &self.typed_handoff_ids.len())
            .field(
                "agent_harness_hook_count",
                &self.agent_harness_hook_invocation_ids.len(),
            )
            .field(
                "agent_harness_hook_disclosure_count",
                &self.agent_harness_hook_disclosure_ids.len(),
            )
            .field("side_effect_count", &self.side_effect_ids.len())
            .field(
                "github_pr_comment_provider_disclosure_count",
                &self.github_pr_comment_provider_disclosures.len(),
            )
            .field("hook_checkpoints", &self.hook_checkpoints)
            .field("has_before_report_hook", &self.before_report_hook.is_some())
            .field("incomplete_work_count", &self.incomplete_work.len())
            .field("known_limitations_count", &self.known_limitations.len())
            .field("risks_count", &self.risks.len())
            .field("handoff_notes_count", &self.handoff_notes.len())
            .finish()
    }
}

/// Explicit hook checkpoint enforcement policy for local report generation.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LocalExecutionHookCheckpointInputs {
    /// Require an explicit `BeforeReport` hook to pass before report generation.
    pub require_before_report: bool,
}

/// Explicit in-memory `BeforeReport` hook input for report-bearing execution.
#[derive(Clone, Eq, PartialEq)]
pub struct LocalExecutionBeforeReportHookInput {
    /// Stable caller-supplied hook invocation ID.
    pub hook_invocation_id: AgentHarnessHookInvocationId,
    /// Explicit hook invocation context.
    pub invocation: AgentHarnessHookInvocationInput,
}

impl fmt::Debug for LocalExecutionBeforeReportHookInput {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalExecutionBeforeReportHookInput")
            .field("hook_invocation_id", &"[REDACTED]")
            .field("hook_kind", &self.invocation.hook_kind)
            .field("workflow_id", &"[REDACTED]")
            .field("run_id", &"[REDACTED]")
            .field(
                "input_reference_count",
                &self.invocation.input_references.len(),
            )
            .field(
                "output_reference_count",
                &self.invocation.output_references.len(),
            )
            .field(
                "supplemental_reference_count",
                &self.invocation.supplemental_references.len(),
            )
            .field("disclosure_count", &self.invocation.disclosures.len())
            .finish()
    }
}

/// Request to execute one local workflow and derive an in-memory report result.
#[derive(Clone, Eq, PartialEq)]
pub struct LocalExecutionWithReportRequest {
    /// Existing local execution request.
    pub execution: LocalExecutionRequest,
    /// Explicit report generation inputs.
    pub report: LocalExecutionReportInputs,
}

impl fmt::Debug for LocalExecutionWithReportRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalExecutionWithReportRequest")
            .field("execution", &"[REDACTED]")
            .field("workflow_id", &"[REDACTED]")
            .field("has_run_id", &self.execution.run_id.is_some())
            .field("report", &self.report)
            .finish()
    }
}

/// Explicit `SideEffect` discovery policy for opt-in executor report generation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalExecutionSideEffectDiscoveryInputs {
    /// Whether to discover supported `SideEffect` IDs from events already present
    /// on the returned workflow run.
    pub include_workflow_events: bool,
    /// Whether to discover persisted `SideEffect` records through the explicitly
    /// supplied `SideEffectRecordStore`.
    pub include_store_records: bool,
    /// Whether every discovered `SideEffect` ID must have a matching record.
    pub require_records: bool,
}

impl From<LocalExecutionSideEffectDiscoveryInputs>
    for TerminalLocalWorkReportSideEffectDiscoveryInput
{
    fn from(inputs: LocalExecutionSideEffectDiscoveryInputs) -> Self {
        Self {
            include_workflow_events: inputs.include_workflow_events,
            include_store_records: inputs.include_store_records,
            require_records: inputs.require_records,
        }
    }
}

/// Request to execute one local workflow and opt into `SideEffect` discovery for
/// the in-memory terminal report result.
#[derive(Clone, Eq, PartialEq)]
pub struct LocalExecutionWithReportAndSideEffectDiscoveryRequest {
    /// Existing local execution request.
    pub execution: LocalExecutionRequest,
    /// Explicit report generation inputs.
    pub report: LocalExecutionReportInputs,
    /// Explicit `SideEffect` discovery policy.
    pub side_effect_discovery: LocalExecutionSideEffectDiscoveryInputs,
}

impl fmt::Debug for LocalExecutionWithReportAndSideEffectDiscoveryRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalExecutionWithReportAndSideEffectDiscoveryRequest")
            .field("execution", &"[REDACTED]")
            .field("workflow_id", &"[REDACTED]")
            .field("has_run_id", &self.execution.run_id.is_some())
            .field("report", &self.report)
            .field("side_effect_discovery", &self.side_effect_discovery)
            .finish()
    }
}

/// Explicit provider-candidate integration context for executor-integrated
/// local report artifacts.
///
/// These inputs are validation context only. They do not call providers,
/// execute side effects, create comments, append events, or fabricate IDs.
#[derive(Clone, Eq, PartialEq)]
pub enum LocalExecutionReportArtifactProviderIntegrationInputs {
    /// Validate that the report artifact is consistent with an expected
    /// proposed GitHub pull request comment side effect.
    GitHubPullRequestComment {
        /// Expected proposed side-effect identity.
        side_effect_id: SideEffectId,
        /// Caller-supplied workflow events used for proposed-event proof.
        workflow_events: Vec<WorkflowRunEvent>,
        /// Provider-candidate citation policy.
        citation_policy: GitHubPullRequestCommentReportArtifactCitationPolicy,
    },
}

impl fmt::Debug for LocalExecutionReportArtifactProviderIntegrationInputs {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GitHubPullRequestComment {
                workflow_events,
                citation_policy,
                ..
            } => formatter
                .debug_struct(
                    "LocalExecutionReportArtifactProviderIntegrationInputs::GitHubPullRequestComment",
                )
                .field("side_effect_id", &"[REDACTED]")
                .field("workflow_event_count", &workflow_events.len())
                .field("citation_policy", citation_policy)
                .finish(),
        }
    }
}

/// Explicit report artifact write policy for executor-integrated local reports.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalExecutionReportArtifactInputs {
    /// Whether every cited `SideEffect` ID must resolve to a stored record.
    pub require_all_side_effect_citations: bool,
    /// Whether `RequiresApproval` side-effect records must cite an approval request.
    pub require_approval_references_for_requires_approval: bool,
    /// Whether approved/denied side-effect records require matching approval decisions.
    pub require_decision_for_approved_or_denied: bool,
    /// Optional high-assurance approval disclosure gate policy.
    pub high_assurance_disclosure_policy: WorkReportArtifactHighAssuranceDisclosurePolicy,
    /// Optional provider-candidate-specific validation context.
    pub provider_integration: Option<LocalExecutionReportArtifactProviderIntegrationInputs>,
}

/// Explicit proof-marker gate inputs for executor-integrated local report
/// artifacts.
///
/// These inputs are opt-in validation context only. They borrow a caller-supplied
/// local approval proof-marker projection store and an explicit policy. They do
/// not discover hidden stores, persist projection records, append events, create
/// artifacts by default, call providers, execute side effects, expose CLI
/// behavior, or change workflow pass/fail semantics.
#[derive(Clone, Copy)]
pub struct LocalExecutionReportArtifactProofMarkerGateInputs<'a> {
    /// Explicit local approval proof-marker projection store.
    pub projection_store: &'a LocalApprovalProofMarkerAuditProjectionStore,
    /// Store-backed approval proof-marker gate policy.
    pub policy: WorkReportArtifactApprovalProofMarkerGatePolicy,
}

impl fmt::Debug for LocalExecutionReportArtifactProofMarkerGateInputs<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalExecutionReportArtifactProofMarkerGateInputs")
            .field("projection_store", &"[REDACTED]")
            .field("policy", &self.policy)
            .finish()
    }
}

/// Explicit projection persistence and proof-marker gate inputs for one local
/// report artifact path.
///
/// These inputs are opt-in validation and local persistence context only. They
/// do not discover hidden stores, enable default executor behavior, persist
/// projections for all approvals, append workflow events, call providers,
/// execute side effects, expose CLI behavior, add schemas, or change workflow
/// pass/fail semantics.
#[derive(Clone, Copy)]
pub struct LocalExecutionProjectedProofMarkerArtifactInputs<'a> {
    /// Explicit local approval proof-marker projection store.
    pub projection_store: &'a LocalApprovalProofMarkerAuditProjectionStore,
    /// Policy for persisting bounded approval proof-marker projections from the
    /// terminal run before artifact proof-marker gates evaluate.
    pub projection_policy: ApprovalProofMarkerProjectionPersistencePolicy,
    /// Optional selected approval references for projection persistence. Empty
    /// means all approval decision events on the run are considered.
    pub selected_approval_reference_ids: &'a [ApprovalReferenceId],
    /// Sensitivity assigned to persisted projection records.
    pub projection_sensitivity: WorkReportSensitivity,
    /// Redaction metadata assigned to persisted projection records.
    pub projection_redaction: &'a RedactionMetadata,
    /// Store-backed approval proof-marker artifact gate policy.
    pub proof_marker_policy: WorkReportArtifactApprovalProofMarkerGatePolicy,
}

impl fmt::Debug for LocalExecutionProjectedProofMarkerArtifactInputs<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalExecutionProjectedProofMarkerArtifactInputs")
            .field("projection_store", &"[REDACTED]")
            .field("projection_policy", &self.projection_policy)
            .field(
                "selected_approval_reference_count",
                &self.selected_approval_reference_ids.len(),
            )
            .field("projection_sensitivity", &self.projection_sensitivity)
            .field("projection_redaction", &"[REDACTED]")
            .field("proof_marker_policy", &self.proof_marker_policy)
            .finish()
    }
}

/// Explicit approval-resume request for proof-enforced report artifact and
/// projection composition.
///
/// This request is local, opt-in, and caller-supplied-store bounded. It does
/// not change default approval behavior, discover hidden stores, enable
/// automatic artifact writing, call providers, execute side effects, expose CLI
/// behavior, add schemas, or change workflow pass/fail semantics.
pub struct LocalApprovalResumeWithProjectedProofMarkerArtifactRequest<'a> {
    /// Explicit projection persistence and artifact proof-marker gate inputs.
    pub projection: LocalExecutionProjectedProofMarkerArtifactInputs<'a>,
    /// Proof-enforced approval decision request.
    pub approval: LocalApprovalPresentationDecisionRequest,
    /// Explicit terminal report generation inputs.
    pub report: &'a LocalExecutionReportInputs,
    /// Optional explicit `SideEffect` discovery policy for report generation.
    pub side_effect_discovery: Option<LocalExecutionSideEffectDiscoveryInputs>,
    /// Explicit artifact gate policy.
    pub artifact: &'a LocalExecutionReportArtifactInputs,
}

impl fmt::Debug for LocalApprovalResumeWithProjectedProofMarkerArtifactRequest<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalApprovalResumeWithProjectedProofMarkerArtifactRequest")
            .field("projection", &self.projection)
            .field("approval", &self.approval)
            .field("report", &self.report)
            .field(
                "has_side_effect_discovery",
                &self.side_effect_discovery.is_some(),
            )
            .field("artifact", &self.artifact)
            .finish()
    }
}

/// Explicit high-assurance approval-resume request for report artifact and
/// proof-marker projection composition.
///
/// This request is local, opt-in, and caller-supplied-store bounded. It does
/// not change default approval behavior, discover hidden stores, enable
/// automatic artifact writing, call providers, execute side effects, expose CLI
/// behavior, add schemas, or change workflow pass/fail semantics.
pub struct LocalHighAssuranceApprovalResumeWithProjectedProofMarkerArtifactRequest<'a> {
    /// Explicit projection persistence and artifact proof-marker gate inputs.
    pub projection: LocalExecutionProjectedProofMarkerArtifactInputs<'a>,
    /// High-assurance approval decision request.
    pub approval: LocalHighAssuranceApprovalDecisionRequest,
    /// Durable presentation proof resolution strategy.
    pub proof: LocalApprovalPresentationProof,
    /// Optional maximum age for presentation proof.
    pub max_presentation_age: Option<Duration>,
    /// Explicit terminal report generation inputs.
    pub report: &'a LocalExecutionReportInputs,
    /// Optional explicit `SideEffect` discovery policy for report generation.
    pub side_effect_discovery: Option<LocalExecutionSideEffectDiscoveryInputs>,
    /// Explicit artifact gate policy.
    pub artifact: &'a LocalExecutionReportArtifactInputs,
}

impl fmt::Debug for LocalHighAssuranceApprovalResumeWithProjectedProofMarkerArtifactRequest<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalHighAssuranceApprovalResumeWithProjectedProofMarkerArtifactRequest")
            .field("projection", &self.projection)
            .field("approval", &self.approval)
            .field(
                "proof",
                &match self.proof {
                    LocalApprovalPresentationProof::PresentationId(_) => "presentation_id",
                    LocalApprovalPresentationProof::ResolveByRunAndApproval => {
                        "resolve_by_run_and_approval"
                    }
                },
            )
            .field(
                "has_max_presentation_age",
                &self.max_presentation_age.is_some(),
            )
            .field("report", &self.report)
            .field(
                "has_side_effect_discovery",
                &self.side_effect_discovery.is_some(),
            )
            .field("artifact", &self.artifact)
            .finish()
    }
}

/// Request to execute one local workflow, derive a report, and explicitly write
/// a governed local report artifact when all artifact gates pass.
#[derive(Clone, Eq, PartialEq)]
pub struct LocalExecutionWithReportArtifactRequest {
    /// Existing local execution request.
    pub execution: LocalExecutionRequest,
    /// Explicit report generation inputs.
    pub report: LocalExecutionReportInputs,
    /// Optional explicit `SideEffect` discovery policy for report generation.
    pub side_effect_discovery: Option<LocalExecutionSideEffectDiscoveryInputs>,
    /// Explicit artifact gate policy.
    pub artifact: LocalExecutionReportArtifactInputs,
}

impl fmt::Debug for LocalExecutionWithReportArtifactRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalExecutionWithReportArtifactRequest")
            .field("execution", &"[REDACTED]")
            .field("workflow_id", &"[REDACTED]")
            .field("has_run_id", &self.execution.run_id.is_some())
            .field("report", &self.report)
            .field(
                "has_side_effect_discovery",
                &self.side_effect_discovery.is_some(),
            )
            .field("artifact", &self.artifact)
            .finish()
    }
}

/// In-memory result for explicit local execution with report generation.
#[derive(Clone, Eq, PartialEq)]
pub struct LocalExecutionWithReportResult {
    run: WorkflowRun,
    work_report: Option<WorkReport>,
    report_generation_error: Option<WorkflowOsError>,
}

impl LocalExecutionWithReportResult {
    /// Creates an in-memory execution/report result.
    #[must_use]
    pub fn new(
        run: WorkflowRun,
        work_report: Option<WorkReport>,
        report_generation_error: Option<WorkflowOsError>,
    ) -> Self {
        Self {
            run,
            work_report,
            report_generation_error,
        }
    }

    /// Returns the workflow run.
    #[must_use]
    pub const fn run(&self) -> &WorkflowRun {
        &self.run
    }

    /// Returns the generated report, when report generation succeeded.
    #[must_use]
    pub const fn work_report(&self) -> Option<&WorkReport> {
        self.work_report.as_ref()
    }

    /// Returns the report-generation error, when report generation failed after execution.
    #[must_use]
    pub const fn report_generation_error(&self) -> Option<&WorkflowOsError> {
        self.report_generation_error.as_ref()
    }

    /// Consumes the result into owned parts.
    #[must_use]
    pub fn into_parts(self) -> (WorkflowRun, Option<WorkReport>, Option<WorkflowOsError>) {
        (self.run, self.work_report, self.report_generation_error)
    }
}

impl fmt::Debug for LocalExecutionWithReportResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalExecutionWithReportResult")
            .field("run_status", &self.run.snapshot.status)
            .field("run_event_count", &self.run.events.len())
            .field("has_work_report", &self.work_report.is_some())
            .field(
                "report_generation_error_code",
                &self
                    .report_generation_error
                    .as_ref()
                    .map(WorkflowOsError::code),
            )
            .finish()
    }
}

/// Owned parts returned by `LocalExecutionWithReportArtifactResult::into_parts`.
pub type LocalExecutionWithReportArtifactParts = (
    WorkflowRun,
    Option<WorkReport>,
    Option<WorkflowOsError>,
    Option<WorkReportArtifactRecord>,
    Option<WorkflowOsError>,
    Option<WorkReportArtifactSideEffectIntegrityResult>,
    Option<SideEffectApprovalLinkageFromStoreResult>,
    Option<WorkReportArtifactHighAssuranceDisclosureGateResult>,
);

/// Owned parts returned by
/// `LocalExecutionWithProjectedProofMarkerArtifactResult::into_parts`.
pub type LocalExecutionWithProjectedProofMarkerArtifactParts = (
    LocalExecutionWithReportArtifactResult,
    Option<ApprovalProofMarkerProjectionPersistenceResult>,
    Option<WorkflowOsError>,
);

struct LocalExecutorArtifactWriteGateResult {
    side_effect_integrity: WorkReportArtifactSideEffectIntegrityResult,
    approval_linkage: Option<SideEffectApprovalLinkageFromStoreResult>,
    high_assurance_disclosure: Option<WorkReportArtifactHighAssuranceDisclosureGateResult>,
}

struct LocalExecutorArtifactWriteGateInput<'a> {
    run: &'a WorkflowRun,
    artifact: &'a WorkReportArtifactRecord,
    artifact_inputs: &'a LocalExecutionReportArtifactInputs,
    provider_integration: ReportArtifactWriteProviderIntegration<'a>,
    high_assurance_disclosure_policy: WorkReportArtifactHighAssuranceDisclosurePolicy,
    approval_proof_marker_policy: Option<WorkReportArtifactApprovalProofMarkerGatePolicy>,
    proof_marker_gate: Option<LocalExecutionReportArtifactProofMarkerGateInputs<'a>>,
}

struct ProjectedProofMarkerArtifactFinishInput<'a> {
    run: WorkflowRun,
    workflow_report_artifact_policies: WorkflowReportArtifactPolicies,
    projection_persistence: ApprovalProofMarkerProjectionPersistenceResult,
    projection_inputs: LocalExecutionProjectedProofMarkerArtifactInputs<'a>,
    report: &'a LocalExecutionReportInputs,
    side_effect_discovery: Option<LocalExecutionSideEffectDiscoveryInputs>,
    artifact_inputs: &'a LocalExecutionReportArtifactInputs,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct WorkflowReportArtifactPolicies {
    high_assurance_disclosure_policy: WorkReportArtifactHighAssuranceDisclosurePolicy,
    approval_proof_marker_policy: Option<WorkReportArtifactApprovalProofMarkerGatePolicy>,
}

/// In-memory result for explicit local execution with projection persistence and
/// report artifact persistence.
#[derive(Clone, Eq, PartialEq)]
pub struct LocalExecutionWithProjectedProofMarkerArtifactResult {
    artifact_result: LocalExecutionWithReportArtifactResult,
    projection_persistence: Option<ApprovalProofMarkerProjectionPersistenceResult>,
    projection_persistence_error: Option<WorkflowOsError>,
}

impl LocalExecutionWithProjectedProofMarkerArtifactResult {
    /// Creates an executor-integrated projection/artifact result.
    #[must_use]
    pub fn new(
        artifact_result: LocalExecutionWithReportArtifactResult,
        projection_persistence: Option<ApprovalProofMarkerProjectionPersistenceResult>,
        projection_persistence_error: Option<WorkflowOsError>,
    ) -> Self {
        Self {
            artifact_result,
            projection_persistence,
            projection_persistence_error,
        }
    }

    /// Returns the underlying report artifact result.
    #[must_use]
    pub const fn artifact_result(&self) -> &LocalExecutionWithReportArtifactResult {
        &self.artifact_result
    }

    /// Returns the workflow run.
    #[must_use]
    pub const fn run(&self) -> &WorkflowRun {
        self.artifact_result.run()
    }

    /// Returns projection persistence posture when projection persistence succeeded.
    #[must_use]
    pub const fn projection_persistence(
        &self,
    ) -> Option<&ApprovalProofMarkerProjectionPersistenceResult> {
        self.projection_persistence.as_ref()
    }

    /// Returns projection persistence error when projection persistence failed after a run existed.
    #[must_use]
    pub const fn projection_persistence_error(&self) -> Option<&WorkflowOsError> {
        self.projection_persistence_error.as_ref()
    }

    /// Consumes the result into owned parts.
    #[must_use]
    pub fn into_parts(self) -> LocalExecutionWithProjectedProofMarkerArtifactParts {
        (
            self.artifact_result,
            self.projection_persistence,
            self.projection_persistence_error,
        )
    }
}

impl fmt::Debug for LocalExecutionWithProjectedProofMarkerArtifactResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalExecutionWithProjectedProofMarkerArtifactResult")
            .field("artifact_result", &self.artifact_result)
            .field(
                "has_projection_persistence",
                &self.projection_persistence.is_some(),
            )
            .field(
                "projection_persistence_error_code",
                &self
                    .projection_persistence_error
                    .as_ref()
                    .map(WorkflowOsError::code),
            )
            .finish()
    }
}

/// In-memory result for explicit local execution with report artifact persistence.
#[derive(Clone, Eq, PartialEq)]
pub struct LocalExecutionWithReportArtifactResult {
    run: WorkflowRun,
    work_report: Option<WorkReport>,
    report_generation_error: Option<WorkflowOsError>,
    work_report_artifact: Option<WorkReportArtifactRecord>,
    artifact_write_error: Option<WorkflowOsError>,
    side_effect_integrity: Option<WorkReportArtifactSideEffectIntegrityResult>,
    approval_linkage: Option<SideEffectApprovalLinkageFromStoreResult>,
    high_assurance_disclosure: Option<WorkReportArtifactHighAssuranceDisclosureGateResult>,
}

impl LocalExecutionWithReportArtifactResult {
    /// Creates an executor-integrated report artifact result.
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new(
        run: WorkflowRun,
        work_report: Option<WorkReport>,
        report_generation_error: Option<WorkflowOsError>,
        work_report_artifact: Option<WorkReportArtifactRecord>,
        artifact_write_error: Option<WorkflowOsError>,
        side_effect_integrity: Option<WorkReportArtifactSideEffectIntegrityResult>,
        approval_linkage: Option<SideEffectApprovalLinkageFromStoreResult>,
        high_assurance_disclosure: Option<WorkReportArtifactHighAssuranceDisclosureGateResult>,
    ) -> Self {
        Self {
            run,
            work_report,
            report_generation_error,
            work_report_artifact,
            artifact_write_error,
            side_effect_integrity,
            approval_linkage,
            high_assurance_disclosure,
        }
    }

    /// Returns the workflow run.
    #[must_use]
    pub const fn run(&self) -> &WorkflowRun {
        &self.run
    }

    /// Returns the generated report, when report generation succeeded.
    #[must_use]
    pub const fn work_report(&self) -> Option<&WorkReport> {
        self.work_report.as_ref()
    }

    /// Returns the report-generation error, when report generation failed.
    #[must_use]
    pub const fn report_generation_error(&self) -> Option<&WorkflowOsError> {
        self.report_generation_error.as_ref()
    }

    /// Returns the written report artifact, when artifact persistence succeeded.
    #[must_use]
    pub const fn work_report_artifact(&self) -> Option<&WorkReportArtifactRecord> {
        self.work_report_artifact.as_ref()
    }

    /// Returns the artifact-path error, when artifact persistence failed after report generation.
    #[must_use]
    pub const fn artifact_write_error(&self) -> Option<&WorkflowOsError> {
        self.artifact_write_error.as_ref()
    }

    /// Returns side-effect integrity counts when artifact gates ran.
    #[must_use]
    pub const fn side_effect_integrity(
        &self,
    ) -> Option<&WorkReportArtifactSideEffectIntegrityResult> {
        self.side_effect_integrity.as_ref()
    }

    /// Returns approval-linkage counts when side-effect citations required linkage.
    #[must_use]
    pub const fn approval_linkage(&self) -> Option<&SideEffectApprovalLinkageFromStoreResult> {
        self.approval_linkage.as_ref()
    }

    /// Returns high-assurance disclosure gate posture when artifact gates ran.
    #[must_use]
    pub const fn high_assurance_disclosure(
        &self,
    ) -> Option<&WorkReportArtifactHighAssuranceDisclosureGateResult> {
        self.high_assurance_disclosure.as_ref()
    }

    /// Consumes the result into owned parts.
    #[must_use]
    pub fn into_parts(self) -> LocalExecutionWithReportArtifactParts {
        (
            self.run,
            self.work_report,
            self.report_generation_error,
            self.work_report_artifact,
            self.artifact_write_error,
            self.side_effect_integrity,
            self.approval_linkage,
            self.high_assurance_disclosure,
        )
    }
}

impl fmt::Debug for LocalExecutionWithReportArtifactResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalExecutionWithReportArtifactResult")
            .field("run_status", &self.run.snapshot.status)
            .field("run_event_count", &self.run.events.len())
            .field("has_work_report", &self.work_report.is_some())
            .field(
                "report_generation_error_code",
                &self
                    .report_generation_error
                    .as_ref()
                    .map(WorkflowOsError::code),
            )
            .field(
                "has_work_report_artifact",
                &self.work_report_artifact.is_some(),
            )
            .field(
                "artifact_write_error_code",
                &self
                    .artifact_write_error
                    .as_ref()
                    .map(WorkflowOsError::code),
            )
            .field(
                "has_side_effect_integrity",
                &self.side_effect_integrity.is_some(),
            )
            .field("has_approval_linkage", &self.approval_linkage.is_some())
            .field(
                "has_high_assurance_disclosure",
                &self.high_assurance_disclosure.is_some(),
            )
            .finish()
    }
}

/// Explicit inputs for one executor-integrated GitHub PR comment provider write.
///
/// This input composes an already-reviewed provider-call orchestration boundary
/// with reconciliation metadata. It does not authorize default executor writes,
/// hidden auth loading, automatic retries, workflow event appends, report
/// artifact writes, CLI behavior, schemas, examples, hosted behavior, or release
/// posture changes.
#[derive(Clone)]
pub struct LocalExecutionGitHubPrCommentProviderWriteInputs<'a> {
    /// Provider-call orchestration input. The caller must supply an attempted
    /// side-effect record and explicit auth/provider-call opt-in.
    pub provider_call: GitHubPullRequestCommentProviderCallOrchestrationInput<'a>,
    /// Sensitivity assigned to any reconciliation candidate.
    pub reconciliation_sensitivity: SideEffectSensitivity,
    /// Redaction metadata assigned to any reconciliation candidate.
    pub reconciliation_redaction: RedactionMetadata,
}

impl fmt::Debug for LocalExecutionGitHubPrCommentProviderWriteInputs<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalExecutionGitHubPrCommentProviderWriteInputs")
            .field("provider_call", &self.provider_call)
            .field(
                "reconciliation_sensitivity",
                &self.reconciliation_sensitivity,
            )
            .field("reconciliation_redaction", &"[REDACTED]")
            .field("workflow_event_append_allowed", &false)
            .field("report_artifact_write_allowed", &false)
            .finish()
    }
}

/// Explicit local execution request that opts into one injected GitHub PR
/// comment provider write after workflow execution.
#[derive(Clone)]
pub struct LocalExecutionWithGitHubPrCommentProviderWriteRequest<'a> {
    /// Existing local execution request. Default executor behavior is unchanged.
    pub execution: LocalExecutionRequest,
    /// Explicit provider-write inputs.
    pub provider_write: LocalExecutionGitHubPrCommentProviderWriteInputs<'a>,
}

impl fmt::Debug for LocalExecutionWithGitHubPrCommentProviderWriteRequest<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalExecutionWithGitHubPrCommentProviderWriteRequest")
            .field("execution", &self.execution)
            .field("provider_write", &self.provider_write)
            .finish()
    }
}

/// Explicit local execution request that opts into one injected GitHub PR
/// comment provider write with an approval-presentation proof gate.
#[derive(Clone)]
pub struct LocalExecutionWithGitHubPrCommentProviderWritePresentationGateRequest<'a> {
    /// Existing provider-write request. Default executor behavior is unchanged.
    pub request: LocalExecutionWithGitHubPrCommentProviderWriteRequest<'a>,
    /// Explicit approval-presentation policy for the provider-write boundary.
    pub presentation_policy: ApprovalPresentationDefaultEnforcementPolicy,
}

impl fmt::Debug for LocalExecutionWithGitHubPrCommentProviderWritePresentationGateRequest<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalExecutionWithGitHubPrCommentProviderWritePresentationGateRequest")
            .field("request", &self.request)
            .field("presentation_policy", &self.presentation_policy)
            .finish()
    }
}

/// Explicit local runtime composition request for one GitHub PR comment
/// provider-write lane.
///
/// This request is a named composition boundary over existing reviewed
/// executor, approval-presentation, provider-call, reconciliation, workflow
/// event proof, and report-disclosure helpers. It remains local, in-memory, and
/// opt-in. It does not make provider writes automatic, load hidden auth, add
/// runtime config, write report artifacts, expose CLI behavior, add schemas or
/// examples, perform lookup/recovery, broaden providers, retry automatically,
/// or change release posture.
#[derive(Clone)]
pub struct GitHubPrCommentProviderWriteRuntimeCompositionRequest<'a> {
    /// Explicit proof-gated provider-write request.
    pub provider_write: LocalExecutionWithGitHubPrCommentProviderWritePresentationGateRequest<'a>,
}

/// Explicit runtime-composition request for one accepted GitHub PR comment live
/// sandbox validation path.
///
/// This request composes the accepted live-sandbox validation helper into the
/// executor-adjacent runtime surface. It remains local, explicit, and
/// caller-supplied. It does not execute `LocalExecutor`, make provider writes
/// automatic, load hidden auth, infer runtime config, append workflow events,
/// write report artifacts, expose CLI behavior, add schemas/examples, perform
/// lookup/recovery, broaden providers, or change release posture.
pub struct GitHubPrCommentLiveSandboxRuntimeCompositionRequest<'a> {
    /// Explicit proof/readiness/provider-call input for the live sandbox path.
    pub live_sandbox: GitHubPullRequestCommentLiveSandboxValidationInput<'a>,
}

/// Explicit request that binds one GitHub PR comment live-sandbox call to
/// proof-enforced approval authority.
///
/// The caller supplies a terminal run and presentation policy, but cannot
/// independently assert that approval is linked and granted. This composition
/// derives that posture from the run, durable presentation proof, and the
/// persisted attempted `SideEffectRecord` before the provider is reachable.
pub struct GitHubPrCommentLiveSandboxApprovalAuthorityCompositionRequest<'a> {
    /// Existing explicit live-sandbox runtime request.
    pub live_sandbox: GitHubPrCommentLiveSandboxRuntimeCompositionRequest<'a>,
    /// Terminal workflow run containing the approval request and decision.
    pub run: &'a WorkflowRun,
    /// Required approval-presentation proof policy.
    pub presentation_policy: ApprovalPresentationDefaultEnforcementPolicy,
}

impl fmt::Debug for GitHubPrCommentLiveSandboxApprovalAuthorityCompositionRequest<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPrCommentLiveSandboxApprovalAuthorityCompositionRequest")
            .field("run_status", &self.run.snapshot.status)
            .field("run_event_count", &self.run.events.len())
            .field("presentation_policy", &self.presentation_policy)
            .field("provider_call_allowed_before_validation", &false)
            .finish()
    }
}

/// Explicit event-proof append policy for one already-composed GitHub PR comment
/// live sandbox result.
///
/// This policy does not authorize provider calls, hidden auth loading, report
/// artifact writes, CLI behavior, schemas, examples, hosted behavior, retries,
/// repair, or broader runtime mutation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GitHubPrCommentLiveSandboxEventProofAppendPolicy {
    /// Do not append workflow event proof.
    Disabled,
    /// Append a completed/failed workflow event when missing.
    AppendIfMissing,
}

/// Bounded event-proof composition status for one already-composed GitHub PR
/// comment live sandbox result.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GitHubPrCommentLiveSandboxEventProofStatus {
    /// Event proof was not requested by policy.
    NotRequested,
    /// The supplied result/run pair was not eligible for event proof.
    NotEligible,
    /// The helper blocked before append because the run was not terminal.
    Blocked,
    /// Durable workflow event proof was appended.
    Appended,
    /// Durable workflow event proof was already present for the idempotency key.
    AlreadyPresent,
    /// Event proof conflicted with existing run or side-effect identity.
    Conflict,
    /// Event proof append failed.
    Failed,
}

/// Explicit event-proof composition request for one already-composed GitHub PR
/// comment live sandbox result.
///
/// This request is a post-provider projection boundary only. It consumes an
/// existing in-memory live sandbox runtime composition result and appends
/// completed/failed workflow event proof to the caller-supplied run through the
/// existing local event append path. It does not call the provider again,
/// recreate `EvidenceReference` values, load auth, discover runtime config,
/// write reports, expose CLI behavior, add schemas/examples, retry/repair, or
/// change release posture.
pub struct GitHubPrCommentLiveSandboxEventProofCompositionRequest<'a> {
    /// Existing accepted live sandbox runtime composition result.
    pub live_sandbox: GitHubPrCommentLiveSandboxRuntimeCompositionResult,
    /// Existing workflow run to receive durable event proof.
    pub run: &'a WorkflowRun,
    /// Explicit event append policy.
    pub append_policy: GitHubPrCommentLiveSandboxEventProofAppendPolicy,
    /// Correlation ID expected to match the accepted `SideEffect` transition.
    pub correlation_id: CorrelationId,
    /// Actor used for the local event append.
    pub actor: ActorId,
}

impl fmt::Debug for GitHubPrCommentProviderWriteRuntimeCompositionRequest<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPrCommentProviderWriteRuntimeCompositionRequest")
            .field("provider_write", &self.provider_write)
            .field("automatic_provider_write_allowed", &false)
            .field("hidden_auth_loading_allowed", &false)
            .field("report_artifact_write_allowed", &false)
            .field("lookup_recovery_allowed", &false)
            .finish()
    }
}

impl fmt::Debug for GitHubPrCommentLiveSandboxRuntimeCompositionRequest<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPrCommentLiveSandboxRuntimeCompositionRequest")
            .field("live_sandbox", &self.live_sandbox)
            .field("automatic_provider_write_allowed", &false)
            .field("hidden_auth_loading_allowed", &false)
            .field("workflow_event_append_allowed", &false)
            .field("report_artifact_write_allowed", &false)
            .field("lookup_recovery_allowed", &false)
            .finish()
    }
}

impl fmt::Debug for GitHubPrCommentLiveSandboxEventProofCompositionRequest<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPrCommentLiveSandboxEventProofCompositionRequest")
            .field("live_sandbox", &self.live_sandbox)
            .field("run", &"[REDACTED]")
            .field("append_policy", &self.append_policy)
            .field("correlation_id", &"[REDACTED]")
            .field("actor", &"[REDACTED]")
            .field("provider_call_allowed", &false)
            .field("hidden_auth_loading_allowed", &false)
            .field("report_artifact_write_allowed", &false)
            .field("lookup_recovery_allowed", &false)
            .finish()
    }
}

/// Explicit artifact-gated composition request for one GitHub PR comment
/// provider-write lane.
///
/// This request composes the existing in-memory provider-write runtime helper
/// with existing report artifact governance gates. It remains local, explicit,
/// and caller supplied. It does not make provider writes automatic, infer auth,
/// infer stores, add runtime config, expose CLI behavior, perform lookup or
/// recovery, broaden providers, or change release posture.
#[derive(Clone)]
pub struct GitHubPrCommentProviderWriteArtifactGatedCompositionRequest<'a> {
    /// Explicit in-memory provider-write composition request.
    pub provider_write: GitHubPrCommentProviderWriteRuntimeCompositionRequest<'a>,
    /// Validated report artifact to write after provider-write composition and
    /// artifact gates pass.
    pub artifact: &'a WorkReportArtifactRecord,
    /// Expected GitHub PR comment `SideEffect` citation in the artifact.
    pub side_effect_id: &'a SideEffectId,
    /// GitHub PR comment citation validation requirements.
    pub citation_policy: GitHubPullRequestCommentReportArtifactCitationPolicy,
    /// Provider disclosure event-proof gate policy.
    pub provider_event_proof_gate_policy:
        GitHubPullRequestCommentProviderReportArtifactEventProofGatePolicy,
    /// Whether every cited `SideEffect` ID must resolve to a stored record.
    pub require_all_side_effect_citations: bool,
    /// Whether `RequiresApproval` side-effect records must cite an approval request.
    pub require_approval_references_for_requires_approval: bool,
    /// Whether approved/denied side-effect records require matching approval decisions.
    pub require_decision_for_approved_or_denied: bool,
    /// Optional high-assurance approval disclosure gate policy.
    pub high_assurance_disclosure_policy: WorkReportArtifactHighAssuranceDisclosurePolicy,
    /// Explicit local approval proof-marker projection store.
    pub approval_proof_marker_projection_store: &'a LocalApprovalProofMarkerAuditProjectionStore,
    /// Store-backed approval proof-marker gate policy.
    pub approval_proof_marker_policy: WorkReportArtifactApprovalProofMarkerGatePolicy,
}

impl fmt::Debug for GitHubPrCommentProviderWriteArtifactGatedCompositionRequest<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPrCommentProviderWriteArtifactGatedCompositionRequest")
            .field("provider_write", &self.provider_write)
            .field("artifact", &"[REDACTED]")
            .field("side_effect_id", &"[REDACTED]")
            .field("citation_policy", &self.citation_policy)
            .field(
                "provider_event_proof_gate_policy",
                &self.provider_event_proof_gate_policy,
            )
            .field(
                "require_all_side_effect_citations",
                &self.require_all_side_effect_citations,
            )
            .field(
                "require_approval_references_for_requires_approval",
                &self.require_approval_references_for_requires_approval,
            )
            .field(
                "require_decision_for_approved_or_denied",
                &self.require_decision_for_approved_or_denied,
            )
            .field(
                "high_assurance_disclosure_policy",
                &self.high_assurance_disclosure_policy,
            )
            .field("approval_proof_marker_projection_store", &"[REDACTED]")
            .field(
                "approval_proof_marker_policy",
                &self.approval_proof_marker_policy,
            )
            .field("automatic_provider_write_allowed", &false)
            .field("automatic_artifact_write_allowed", &false)
            .field("hidden_auth_loading_allowed", &false)
            .field("hidden_store_loading_allowed", &false)
            .finish()
    }
}

/// Owned parts returned by
/// `LocalExecutionWithGitHubPrCommentProviderWriteResult::into_parts`.
pub type LocalExecutionWithGitHubPrCommentProviderWriteParts = (
    WorkflowRun,
    Option<GitHubPullRequestCommentWriteResponse>,
    Option<SideEffectLifecycleTransitionResult>,
    Option<GitHubPullRequestCommentProviderWriteReconciliationCandidate>,
    Option<WorkflowOsError>,
    bool,
);

/// Owned parts returned by
/// `GitHubPrCommentProviderWriteRuntimeCompositionResult::into_parts`.
pub type GitHubPrCommentProviderWriteRuntimeCompositionParts = (
    LocalExecutionWithGitHubPrCommentProviderWriteResult,
    GitHubPullRequestCommentProviderWriteReportDisclosure,
);

/// Owned parts returned by
/// `GitHubPrCommentLiveSandboxRuntimeCompositionResult::into_parts`.
pub type GitHubPrCommentLiveSandboxRuntimeCompositionParts =
    GitHubPullRequestCommentLiveSandboxValidationResult;

/// Owned parts returned by
/// `GitHubPrCommentLiveSandboxEventProofCompositionResult::into_parts`.
pub type GitHubPrCommentLiveSandboxEventProofCompositionParts = (
    GitHubPrCommentLiveSandboxRuntimeCompositionResult,
    WorkflowRun,
    GitHubPrCommentLiveSandboxEventProofStatus,
    Option<WorkflowOsError>,
);

/// Owned parts returned by
/// `GitHubPrCommentProviderWriteArtifactGatedCompositionResult::into_parts`.
pub type GitHubPrCommentProviderWriteArtifactGatedCompositionParts = (
    GitHubPrCommentProviderWriteRuntimeCompositionResult,
    Option<WorkReportArtifactProofMarkerGovernedWriteResult>,
    Option<WorkflowOsError>,
);

/// Bounded gate state for the executor-integrated GitHub PR comment provider
/// write path.
///
/// This is projection vocabulary only. It does not authorize provider writes,
/// event appends, retries, repairs, artifact writes, hidden auth, CLI behavior,
/// schemas, examples, hosted behavior, or release posture changes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GitHubPullRequestCommentProviderWriteGateState {
    /// The gate was satisfied by validated local context.
    Satisfied,
    /// The gate was evaluated and blocks the provider-write closure path.
    Blocked,
    /// The gate was not reached by this explicit result.
    NotEvaluated,
    /// The gate is not required for this explicit result.
    NotRequired,
}

/// Bounded gate clarity projection for one explicit executor-integrated GitHub
/// PR comment provider-write result.
///
/// The projection answers what the current result proves about pre-provider and
/// post-provider gates without copying raw provider payloads or treating
/// provider responses as workflow event proof.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct GitHubPullRequestCommentProviderWriteGateClarity {
    preflight_context: GitHubPullRequestCommentProviderWriteGateState,
    attempted_record: GitHubPullRequestCommentProviderWriteGateState,
    approval_linkage: GitHubPullRequestCommentProviderWriteGateState,
    approval_presentation: GitHubPullRequestCommentProviderWriteGateState,
    attempted_lifecycle: GitHubPullRequestCommentProviderWriteGateState,
    provider_call: GitHubPullRequestCommentProviderWriteGateState,
    provider_response: GitHubPullRequestCommentProviderWriteGateState,
    post_provider_local_transition: GitHubPullRequestCommentProviderWriteGateState,
    workflow_event_proof: GitHubPullRequestCommentProviderWriteGateState,
    retry: GitHubPullRequestCommentProviderWriteGateState,
    report_artifact_event_proof: GitHubPullRequestCommentProviderWriteGateState,
    operator_recovery: GitHubPullRequestCommentProviderWriteGateState,
}

impl GitHubPullRequestCommentProviderWriteGateClarity {
    #[allow(clippy::too_many_arguments)]
    const fn new(
        preflight_context: GitHubPullRequestCommentProviderWriteGateState,
        attempted_record: GitHubPullRequestCommentProviderWriteGateState,
        approval_linkage: GitHubPullRequestCommentProviderWriteGateState,
        approval_presentation: GitHubPullRequestCommentProviderWriteGateState,
        attempted_lifecycle: GitHubPullRequestCommentProviderWriteGateState,
        provider_call: GitHubPullRequestCommentProviderWriteGateState,
        provider_response: GitHubPullRequestCommentProviderWriteGateState,
        post_provider_local_transition: GitHubPullRequestCommentProviderWriteGateState,
        workflow_event_proof: GitHubPullRequestCommentProviderWriteGateState,
        retry: GitHubPullRequestCommentProviderWriteGateState,
        report_artifact_event_proof: GitHubPullRequestCommentProviderWriteGateState,
        operator_recovery: GitHubPullRequestCommentProviderWriteGateState,
    ) -> Self {
        Self {
            preflight_context,
            attempted_record,
            approval_linkage,
            approval_presentation,
            attempted_lifecycle,
            provider_call,
            provider_response,
            post_provider_local_transition,
            workflow_event_proof,
            retry,
            report_artifact_event_proof,
            operator_recovery,
        }
    }

    /// Returns whether preflight-derived context was available in the attempted
    /// `SideEffectRecord`.
    #[must_use]
    pub const fn preflight_context(&self) -> GitHubPullRequestCommentProviderWriteGateState {
        self.preflight_context
    }

    /// Returns whether the attempted `SideEffectRecord` was present and valid
    /// enough for the provider-write path.
    #[must_use]
    pub const fn attempted_record(&self) -> GitHubPullRequestCommentProviderWriteGateState {
        self.attempted_record
    }

    /// Returns whether approval linkage was satisfied or not required.
    #[must_use]
    pub const fn approval_linkage(&self) -> GitHubPullRequestCommentProviderWriteGateState {
        self.approval_linkage
    }

    /// Returns whether the approval-presentation proof gate was satisfied,
    /// blocked, not evaluated, or not required.
    #[must_use]
    pub const fn approval_presentation(&self) -> GitHubPullRequestCommentProviderWriteGateState {
        self.approval_presentation
    }

    /// Returns whether the attempted lifecycle boundary was satisfied.
    #[must_use]
    pub const fn attempted_lifecycle(&self) -> GitHubPullRequestCommentProviderWriteGateState {
        self.attempted_lifecycle
    }

    /// Returns whether the provider call was attempted or blocked before call.
    #[must_use]
    pub const fn provider_call(&self) -> GitHubPullRequestCommentProviderWriteGateState {
        self.provider_call
    }

    /// Returns whether a classified provider response is available.
    #[must_use]
    pub const fn provider_response(&self) -> GitHubPullRequestCommentProviderWriteGateState {
        self.provider_response
    }

    /// Returns whether post-provider local lifecycle transition succeeded.
    #[must_use]
    pub const fn post_provider_local_transition(
        &self,
    ) -> GitHubPullRequestCommentProviderWriteGateState {
        self.post_provider_local_transition
    }

    /// Returns whether durable workflow event proof is present.
    #[must_use]
    pub const fn workflow_event_proof(&self) -> GitHubPullRequestCommentProviderWriteGateState {
        self.workflow_event_proof
    }

    /// Returns retry posture for this provider-write result.
    #[must_use]
    pub const fn retry(&self) -> GitHubPullRequestCommentProviderWriteGateState {
        self.retry
    }

    /// Returns whether the provider event-proof gate for later report artifact
    /// writes is satisfied.
    #[must_use]
    pub const fn report_artifact_event_proof(
        &self,
    ) -> GitHubPullRequestCommentProviderWriteGateState {
        self.report_artifact_event_proof
    }

    /// Returns operator recovery posture for this provider-write result.
    #[must_use]
    pub const fn operator_recovery(&self) -> GitHubPullRequestCommentProviderWriteGateState {
        self.operator_recovery
    }
}

impl fmt::Debug for GitHubPullRequestCommentProviderWriteGateClarity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentProviderWriteGateClarity")
            .field("preflight_context", &self.preflight_context)
            .field("attempted_record", &self.attempted_record)
            .field("approval_linkage", &self.approval_linkage)
            .field("approval_presentation", &self.approval_presentation)
            .field("attempted_lifecycle", &self.attempted_lifecycle)
            .field("provider_call", &self.provider_call)
            .field("provider_response", &self.provider_response)
            .field(
                "post_provider_local_transition",
                &self.post_provider_local_transition,
            )
            .field("workflow_event_proof", &self.workflow_event_proof)
            .field("retry", &self.retry)
            .field(
                "report_artifact_event_proof",
                &self.report_artifact_event_proof,
            )
            .field("operator_recovery", &self.operator_recovery)
            .finish()
    }
}

/// In-memory result for explicit local execution with one injected GitHub PR
/// comment provider write.
pub struct LocalExecutionWithGitHubPrCommentProviderWriteResult {
    run: WorkflowRun,
    provider_response: Option<GitHubPullRequestCommentWriteResponse>,
    outcome_transition: Option<SideEffectLifecycleTransitionResult>,
    reconciliation_candidate: Option<GitHubPullRequestCommentProviderWriteReconciliationCandidate>,
    provider_write_error: Option<WorkflowOsError>,
    workflow_event_appended: bool,
    gate_clarity: GitHubPullRequestCommentProviderWriteGateClarity,
}

/// In-memory result for the explicit GitHub PR comment provider-write runtime
/// composition helper.
///
/// This result exposes the existing provider-write result and a bounded
/// report-disclosure projection. It does not contain raw provider payloads,
/// auth material, approval-presentation body text, report artifacts, lookup
/// recovery records, or CLI output.
pub struct GitHubPrCommentProviderWriteRuntimeCompositionResult {
    provider_write: LocalExecutionWithGitHubPrCommentProviderWriteResult,
    report_disclosure: GitHubPullRequestCommentProviderWriteReportDisclosure,
}

/// In-memory result for the explicit GitHub PR comment live-sandbox runtime
/// composition helper.
///
/// This result owns the accepted live-sandbox validation result and exposes
/// bounded runtime posture. It does not contain raw provider payloads, auth
/// material, report artifacts, workflow event append output, lookup/recovery
/// records, or CLI output.
pub struct GitHubPrCommentLiveSandboxRuntimeCompositionResult {
    live_sandbox: GitHubPullRequestCommentLiveSandboxValidationResult,
}

/// Accepted live-sandbox result with bounded proof that approval authority was
/// derived from durable run and `SideEffect` state.
pub struct GitHubPrCommentLiveSandboxApprovalAuthorityCompositionResult {
    live_sandbox: GitHubPrCommentLiveSandboxRuntimeCompositionResult,
    approval_linkage: SideEffectApprovalLinkageFromStoreResult,
}

/// In-memory result for explicit GitHub PR comment live-sandbox event-proof
/// composition.
///
/// The result owns the accepted live-sandbox runtime composition result and the
/// run after attempted event-proof projection. It does not contain raw provider
/// payloads, auth material, report artifacts, local paths, lookup/recovery
/// records, or CLI output.
pub struct GitHubPrCommentLiveSandboxEventProofCompositionResult {
    live_sandbox: GitHubPrCommentLiveSandboxRuntimeCompositionResult,
    run: WorkflowRun,
    status: GitHubPrCommentLiveSandboxEventProofStatus,
    event_append_error: Option<WorkflowOsError>,
}

/// In-memory result for explicit GitHub PR comment provider-write composition
/// with report artifact governance gates.
///
/// The result owns the provider-write composition result and bounded artifact
/// write posture. It does not contain raw provider payloads, auth material,
/// approval-presentation body text, report text, local paths, lookup/recovery
/// records, or CLI output.
pub struct GitHubPrCommentProviderWriteArtifactGatedCompositionResult {
    provider_write: GitHubPrCommentProviderWriteRuntimeCompositionResult,
    artifact_write: Option<WorkReportArtifactProofMarkerGovernedWriteResult>,
    artifact_write_error: Option<WorkflowOsError>,
}

/// Bounded report/artifact disclosure posture for one explicit GitHub PR
/// comment provider-write result.
///
/// This is a projection vocabulary only. It does not authorize provider calls,
/// event appends, report artifact writes, retries, auth loading, CLI behavior,
/// schemas, examples, hosted behavior, broader writes, or release posture
/// changes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GitHubPullRequestCommentProviderWriteDisclosurePosture {
    /// No provider call occurred.
    ProviderNotCalled,
    /// Provider success, local completed transition, reconciliation, and
    /// completed workflow event projection agree.
    ProviderSucceededLocalCompletedEventAppended,
    /// Provider success, local completed transition, and reconciliation agree,
    /// but completed workflow event proof is missing or failed.
    ProviderSucceededLocalCompletedEventMissing,
    /// Provider failure, local failed transition, reconciliation, and failed
    /// workflow event projection agree.
    ProviderFailedLocalFailedEventAppended,
    /// Provider failure, local failed transition, and reconciliation agree, but
    /// failed workflow event proof is missing or failed.
    ProviderFailedLocalFailedEventMissing,
    /// Provider response is ambiguous.
    ProviderResponseAmbiguous,
    /// Provider succeeded but local completed transition failed.
    ProviderSucceededLocalTransitionFailed,
    /// Provider failed but local failed transition failed.
    ProviderFailedLocalTransitionFailed,
    /// Local lifecycle state cannot be reconciled safely.
    LocalStateAmbiguous,
    /// A separate reconciliation step is required.
    ReconciliationRequired,
    /// Reconciliation candidate construction failed or is unavailable.
    ReconciliationUnavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
enum GitHubPullRequestCommentProviderWriteDisclosureFlag {
    No,
    Yes,
}

impl GitHubPullRequestCommentProviderWriteDisclosureFlag {
    const fn from_bool(value: bool) -> Self {
        if value {
            Self::Yes
        } else {
            Self::No
        }
    }

    const fn as_bool(self) -> bool {
        matches!(self, Self::Yes)
    }
}

/// Bounded disclosure model derived from an explicit GitHub PR comment
/// provider-write result.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct GitHubPullRequestCommentProviderWriteReportDisclosure {
    posture: GitHubPullRequestCommentProviderWriteDisclosurePosture,
    reconciliation_status: Option<GitHubPullRequestCommentProviderWriteReconciliationStatus>,
    outcome_lifecycle_state: Option<SideEffectLifecycleState>,
    provider_call_performed: GitHubPullRequestCommentProviderWriteDisclosureFlag,
    provider_response_present: GitHubPullRequestCommentProviderWriteDisclosureFlag,
    outcome_transition_present: GitHubPullRequestCommentProviderWriteDisclosureFlag,
    provider_write_error_present: GitHubPullRequestCommentProviderWriteDisclosureFlag,
    workflow_event_appended: GitHubPullRequestCommentProviderWriteDisclosureFlag,
    retry_blocked: GitHubPullRequestCommentProviderWriteDisclosureFlag,
    operator_action_required: GitHubPullRequestCommentProviderWriteDisclosureFlag,
}

impl GitHubPullRequestCommentProviderWriteReportDisclosure {
    /// Returns disclosure posture.
    #[must_use]
    pub const fn posture(&self) -> GitHubPullRequestCommentProviderWriteDisclosurePosture {
        self.posture
    }

    /// Returns underlying reconciliation status when available.
    #[must_use]
    pub const fn reconciliation_status(
        &self,
    ) -> Option<GitHubPullRequestCommentProviderWriteReconciliationStatus> {
        self.reconciliation_status
    }

    /// Returns outcome lifecycle state when available.
    #[must_use]
    pub const fn outcome_lifecycle_state(&self) -> Option<SideEffectLifecycleState> {
        self.outcome_lifecycle_state
    }

    /// Returns whether a provider call was performed or may have been
    /// performed.
    #[must_use]
    pub const fn provider_call_performed(&self) -> bool {
        self.provider_call_performed.as_bool()
    }

    /// Returns whether a classified provider response is present.
    #[must_use]
    pub const fn provider_response_present(&self) -> bool {
        self.provider_response_present.as_bool()
    }

    /// Returns whether a local outcome lifecycle transition is present.
    #[must_use]
    pub const fn outcome_transition_present(&self) -> bool {
        self.outcome_transition_present.as_bool()
    }

    /// Returns whether provider-write error posture is present.
    #[must_use]
    pub const fn provider_write_error_present(&self) -> bool {
        self.provider_write_error_present.as_bool()
    }

    /// Returns whether a completed/failed workflow event projection was
    /// appended.
    #[must_use]
    pub const fn workflow_event_appended(&self) -> bool {
        self.workflow_event_appended.as_bool()
    }

    /// Returns whether retry is blocked by provider/local reconciliation
    /// posture.
    #[must_use]
    pub const fn retry_blocked(&self) -> bool {
        self.retry_blocked.as_bool()
    }

    /// Returns whether operator action is required.
    #[must_use]
    pub const fn operator_action_required(&self) -> bool {
        self.operator_action_required.as_bool()
    }

    /// Returns whether this disclosure helper performed provider calls.
    #[must_use]
    pub const fn provider_call_allowed(&self) -> bool {
        false
    }

    /// Returns whether this disclosure helper appended workflow events.
    #[must_use]
    pub const fn workflow_event_append_allowed(&self) -> bool {
        false
    }

    /// Returns whether this disclosure helper wrote report artifacts.
    #[must_use]
    pub const fn report_artifact_write_allowed(&self) -> bool {
        false
    }
}

impl fmt::Debug for GitHubPullRequestCommentProviderWriteReportDisclosure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentProviderWriteReportDisclosure")
            .field("posture", &self.posture)
            .field("reconciliation_status", &self.reconciliation_status)
            .field("outcome_lifecycle_state", &self.outcome_lifecycle_state)
            .field("provider_call_performed", &self.provider_call_performed)
            .field("provider_response_present", &self.provider_response_present)
            .field(
                "outcome_transition_present",
                &self.outcome_transition_present,
            )
            .field(
                "provider_write_error_present",
                &self.provider_write_error_present,
            )
            .field("workflow_event_appended", &self.workflow_event_appended)
            .field("retry_blocked", &self.retry_blocked)
            .field("operator_action_required", &self.operator_action_required)
            .field("provider_call_allowed", &false)
            .field("workflow_event_append_allowed", &false)
            .field("report_artifact_write_allowed", &false)
            .finish()
    }
}

impl LocalExecutionWithGitHubPrCommentProviderWriteResult {
    /// Creates an in-memory execution/provider-write result.
    #[must_use]
    pub fn new(
        run: WorkflowRun,
        provider_response: Option<GitHubPullRequestCommentWriteResponse>,
        outcome_transition: Option<SideEffectLifecycleTransitionResult>,
        reconciliation_candidate: Option<
            GitHubPullRequestCommentProviderWriteReconciliationCandidate,
        >,
        provider_write_error: Option<WorkflowOsError>,
        workflow_event_appended: bool,
    ) -> Self {
        let provider_call_attempted = provider_response.is_some();
        let gate_clarity = Self::gate_clarity_from_parts(
            provider_response.as_ref(),
            outcome_transition.as_ref(),
            reconciliation_candidate.as_ref(),
            provider_write_error.as_ref(),
            workflow_event_appended,
            provider_call_attempted,
        );
        Self::new_with_gate_clarity(
            run,
            provider_response,
            outcome_transition,
            reconciliation_candidate,
            provider_write_error,
            workflow_event_appended,
            gate_clarity,
        )
    }

    /// Creates an in-memory execution/provider-write result with explicit gate
    /// clarity projection.
    #[must_use]
    pub fn new_with_gate_clarity(
        run: WorkflowRun,
        provider_response: Option<GitHubPullRequestCommentWriteResponse>,
        outcome_transition: Option<SideEffectLifecycleTransitionResult>,
        reconciliation_candidate: Option<
            GitHubPullRequestCommentProviderWriteReconciliationCandidate,
        >,
        provider_write_error: Option<WorkflowOsError>,
        workflow_event_appended: bool,
        gate_clarity: GitHubPullRequestCommentProviderWriteGateClarity,
    ) -> Self {
        Self {
            run,
            provider_response,
            outcome_transition,
            reconciliation_candidate,
            provider_write_error,
            workflow_event_appended,
            gate_clarity,
        }
    }

    fn gate_clarity_from_parts(
        provider_response: Option<&GitHubPullRequestCommentWriteResponse>,
        outcome_transition: Option<&SideEffectLifecycleTransitionResult>,
        reconciliation_candidate: Option<
            &GitHubPullRequestCommentProviderWriteReconciliationCandidate,
        >,
        provider_write_error: Option<&WorkflowOsError>,
        workflow_event_appended: bool,
        provider_call_attempted: bool,
    ) -> GitHubPullRequestCommentProviderWriteGateClarity {
        let provider_call = if provider_call_attempted {
            GitHubPullRequestCommentProviderWriteGateState::Satisfied
        } else if provider_write_error.is_some() {
            GitHubPullRequestCommentProviderWriteGateState::Blocked
        } else {
            GitHubPullRequestCommentProviderWriteGateState::NotEvaluated
        };
        let provider_response_gate = if provider_response.is_some() {
            GitHubPullRequestCommentProviderWriteGateState::Satisfied
        } else if provider_call_attempted {
            GitHubPullRequestCommentProviderWriteGateState::Blocked
        } else {
            GitHubPullRequestCommentProviderWriteGateState::NotEvaluated
        };
        let local_transition = if outcome_transition.is_some() {
            GitHubPullRequestCommentProviderWriteGateState::Satisfied
        } else if provider_response.is_some() || provider_call_attempted {
            GitHubPullRequestCommentProviderWriteGateState::Blocked
        } else {
            GitHubPullRequestCommentProviderWriteGateState::NotEvaluated
        };
        let workflow_event_proof = if workflow_event_appended {
            GitHubPullRequestCommentProviderWriteGateState::Satisfied
        } else if provider_response.is_some() || provider_call_attempted {
            GitHubPullRequestCommentProviderWriteGateState::Blocked
        } else {
            GitHubPullRequestCommentProviderWriteGateState::NotEvaluated
        };
        let retry_or_operator_blocked = match reconciliation_candidate {
            Some(candidate) => candidate.retry_blocked() || candidate.operator_action_required(),
            None => provider_call_attempted && provider_write_error.is_some(),
        };
        let retry = if retry_or_operator_blocked {
            GitHubPullRequestCommentProviderWriteGateState::Blocked
        } else {
            GitHubPullRequestCommentProviderWriteGateState::Satisfied
        };
        let operator_recovery = if retry_or_operator_blocked {
            GitHubPullRequestCommentProviderWriteGateState::Blocked
        } else {
            GitHubPullRequestCommentProviderWriteGateState::Satisfied
        };
        let report_artifact_event_proof = if workflow_event_appended {
            GitHubPullRequestCommentProviderWriteGateState::Satisfied
        } else if provider_response.is_some() || provider_call_attempted {
            GitHubPullRequestCommentProviderWriteGateState::Blocked
        } else {
            GitHubPullRequestCommentProviderWriteGateState::NotEvaluated
        };

        GitHubPullRequestCommentProviderWriteGateClarity::new(
            GitHubPullRequestCommentProviderWriteGateState::NotEvaluated,
            GitHubPullRequestCommentProviderWriteGateState::NotEvaluated,
            GitHubPullRequestCommentProviderWriteGateState::NotEvaluated,
            GitHubPullRequestCommentProviderWriteGateState::NotEvaluated,
            GitHubPullRequestCommentProviderWriteGateState::NotEvaluated,
            provider_call,
            provider_response_gate,
            local_transition,
            workflow_event_proof,
            retry,
            report_artifact_event_proof,
            operator_recovery,
        )
    }

    /// Returns the workflow run produced by local execution.
    #[must_use]
    pub const fn run(&self) -> &WorkflowRun {
        &self.run
    }

    /// Returns the classified provider response when the injected provider
    /// returned one.
    #[must_use]
    pub const fn provider_response(&self) -> Option<&GitHubPullRequestCommentWriteResponse> {
        self.provider_response.as_ref()
    }

    /// Returns the store-backed completed/failed transition when it succeeded.
    #[must_use]
    pub const fn outcome_transition(&self) -> Option<&SideEffectLifecycleTransitionResult> {
        self.outcome_transition.as_ref()
    }

    /// Returns reconciliation posture when available.
    #[must_use]
    pub const fn reconciliation_candidate(
        &self,
    ) -> Option<&GitHubPullRequestCommentProviderWriteReconciliationCandidate> {
        self.reconciliation_candidate.as_ref()
    }

    /// Returns provider-write error when the write path failed after a run
    /// existed.
    #[must_use]
    pub const fn provider_write_error(&self) -> Option<&WorkflowOsError> {
        self.provider_write_error.as_ref()
    }

    /// Returns bounded gate clarity for this provider-write result.
    #[must_use]
    pub const fn gate_clarity(&self) -> &GitHubPullRequestCommentProviderWriteGateClarity {
        &self.gate_clarity
    }

    /// Returns whether the injected provider was called or may have been called.
    #[must_use]
    pub fn provider_call_performed(&self) -> bool {
        self.provider_response.is_some()
            || self.reconciliation_candidate.as_ref().is_some_and(
                GitHubPullRequestCommentProviderWriteReconciliationCandidate::retry_blocked,
            )
    }

    /// Returns whether this helper appended workflow events.
    #[must_use]
    pub const fn workflow_event_appended(&self) -> bool {
        self.workflow_event_appended
    }

    /// Returns whether this helper wrote report artifacts.
    #[must_use]
    pub const fn report_artifact_written(&self) -> bool {
        false
    }

    /// Returns whether retry is blocked by reconciliation posture.
    #[must_use]
    pub fn retry_blocked(&self) -> bool {
        self.reconciliation_candidate.as_ref().is_some_and(
            GitHubPullRequestCommentProviderWriteReconciliationCandidate::retry_blocked,
        )
    }

    /// Returns whether operator/future reconciliation action is required.
    #[must_use]
    pub fn operator_action_required(&self) -> bool {
        self.reconciliation_candidate.as_ref().is_some_and(
            GitHubPullRequestCommentProviderWriteReconciliationCandidate::operator_action_required,
        )
    }

    /// Consumes the result into owned parts.
    #[must_use]
    pub fn into_parts(self) -> LocalExecutionWithGitHubPrCommentProviderWriteParts {
        (
            self.run,
            self.provider_response,
            self.outcome_transition,
            self.reconciliation_candidate,
            self.provider_write_error,
            self.workflow_event_appended,
        )
    }

    /// Derives bounded report/artifact disclosure posture from this explicit
    /// provider-write result.
    ///
    /// The helper is pure projection. It does not call providers, append
    /// workflow events, mutate side-effect records, write report artifacts,
    /// retry, load auth, read hidden state, or create evidence.
    #[must_use]
    pub fn report_disclosure(&self) -> GitHubPullRequestCommentProviderWriteReportDisclosure {
        let reconciliation_status = self
            .reconciliation_candidate
            .as_ref()
            .map(GitHubPullRequestCommentProviderWriteReconciliationCandidate::status);
        let outcome_lifecycle_state = self
            .outcome_transition
            .as_ref()
            .map(|transition| transition.record().lifecycle_state());
        let posture = match reconciliation_status {
            Some(GitHubPullRequestCommentProviderWriteReconciliationStatus::ProviderNotCalled) => {
                GitHubPullRequestCommentProviderWriteDisclosurePosture::ProviderNotCalled
            }
            Some(
                GitHubPullRequestCommentProviderWriteReconciliationStatus::ProviderSucceededLocalCompleted,
            ) if self.workflow_event_appended => {
                GitHubPullRequestCommentProviderWriteDisclosurePosture::ProviderSucceededLocalCompletedEventAppended
            }
            Some(
                GitHubPullRequestCommentProviderWriteReconciliationStatus::ProviderSucceededLocalCompleted,
            ) => {
                GitHubPullRequestCommentProviderWriteDisclosurePosture::ProviderSucceededLocalCompletedEventMissing
            }
            Some(
                GitHubPullRequestCommentProviderWriteReconciliationStatus::ProviderFailedLocalFailed,
            ) if self.workflow_event_appended => {
                GitHubPullRequestCommentProviderWriteDisclosurePosture::ProviderFailedLocalFailedEventAppended
            }
            Some(
                GitHubPullRequestCommentProviderWriteReconciliationStatus::ProviderFailedLocalFailed,
            ) => {
                GitHubPullRequestCommentProviderWriteDisclosurePosture::ProviderFailedLocalFailedEventMissing
            }
            Some(
                GitHubPullRequestCommentProviderWriteReconciliationStatus::ProviderResponseAmbiguous,
            ) => GitHubPullRequestCommentProviderWriteDisclosurePosture::ProviderResponseAmbiguous,
            Some(
                GitHubPullRequestCommentProviderWriteReconciliationStatus::ProviderSucceededLocalTransitionFailed,
            ) => {
                GitHubPullRequestCommentProviderWriteDisclosurePosture::ProviderSucceededLocalTransitionFailed
            }
            Some(
                GitHubPullRequestCommentProviderWriteReconciliationStatus::ProviderFailedLocalTransitionFailed,
            ) => {
                GitHubPullRequestCommentProviderWriteDisclosurePosture::ProviderFailedLocalTransitionFailed
            }
            Some(GitHubPullRequestCommentProviderWriteReconciliationStatus::LocalStateAmbiguous) => {
                GitHubPullRequestCommentProviderWriteDisclosurePosture::LocalStateAmbiguous
            }
            Some(GitHubPullRequestCommentProviderWriteReconciliationStatus::ReconciliationRequired) => {
                GitHubPullRequestCommentProviderWriteDisclosurePosture::ReconciliationRequired
            }
            None => {
                GitHubPullRequestCommentProviderWriteDisclosurePosture::ReconciliationUnavailable
            }
        };

        GitHubPullRequestCommentProviderWriteReportDisclosure {
            posture,
            reconciliation_status,
            outcome_lifecycle_state,
            provider_call_performed: GitHubPullRequestCommentProviderWriteDisclosureFlag::from_bool(
                self.provider_call_performed(),
            ),
            provider_response_present:
                GitHubPullRequestCommentProviderWriteDisclosureFlag::from_bool(
                    self.provider_response.is_some(),
                ),
            outcome_transition_present:
                GitHubPullRequestCommentProviderWriteDisclosureFlag::from_bool(
                    self.outcome_transition.is_some(),
                ),
            provider_write_error_present:
                GitHubPullRequestCommentProviderWriteDisclosureFlag::from_bool(
                    self.provider_write_error.is_some(),
                ),
            workflow_event_appended: GitHubPullRequestCommentProviderWriteDisclosureFlag::from_bool(
                self.workflow_event_appended,
            ),
            retry_blocked: GitHubPullRequestCommentProviderWriteDisclosureFlag::from_bool(
                self.retry_blocked(),
            ),
            operator_action_required:
                GitHubPullRequestCommentProviderWriteDisclosureFlag::from_bool(
                    self.operator_action_required(),
                ),
        }
    }
}

impl GitHubPrCommentProviderWriteRuntimeCompositionResult {
    /// Creates an in-memory provider-write runtime composition result from the
    /// already-composed explicit provider-write result.
    #[must_use]
    pub fn new(provider_write: LocalExecutionWithGitHubPrCommentProviderWriteResult) -> Self {
        let report_disclosure = provider_write.report_disclosure();
        Self {
            provider_write,
            report_disclosure,
        }
    }

    /// Returns the composed provider-write result.
    #[must_use]
    pub const fn provider_write(&self) -> &LocalExecutionWithGitHubPrCommentProviderWriteResult {
        &self.provider_write
    }

    /// Returns bounded report-disclosure posture derived from the provider-write result.
    #[must_use]
    pub const fn report_disclosure(
        &self,
    ) -> &GitHubPullRequestCommentProviderWriteReportDisclosure {
        &self.report_disclosure
    }

    /// Returns bounded gate clarity for this composition.
    #[must_use]
    pub const fn gate_clarity(&self) -> &GitHubPullRequestCommentProviderWriteGateClarity {
        self.provider_write.gate_clarity()
    }

    /// Returns the workflow run produced by local execution.
    #[must_use]
    pub const fn run(&self) -> &WorkflowRun {
        self.provider_write.run()
    }

    /// Returns provider-write error when the write path failed after a run
    /// existed.
    #[must_use]
    pub const fn provider_write_error(&self) -> Option<&WorkflowOsError> {
        self.provider_write.provider_write_error()
    }

    /// Returns whether the injected provider was called or may have been called.
    #[must_use]
    pub fn provider_call_performed(&self) -> bool {
        self.provider_write.provider_call_performed()
    }

    /// Returns whether this composition appended a completed/failed workflow event.
    #[must_use]
    pub const fn workflow_event_appended(&self) -> bool {
        self.provider_write.workflow_event_appended()
    }

    /// Returns whether this composition wrote report artifacts.
    #[must_use]
    pub const fn report_artifact_written(&self) -> bool {
        false
    }

    /// Consumes the result into owned parts.
    #[must_use]
    pub fn into_parts(self) -> GitHubPrCommentProviderWriteRuntimeCompositionParts {
        (self.provider_write, self.report_disclosure)
    }
}

impl GitHubPrCommentLiveSandboxRuntimeCompositionResult {
    /// Creates an in-memory live-sandbox runtime composition result from the
    /// accepted validation result.
    #[must_use]
    pub const fn new(live_sandbox: GitHubPullRequestCommentLiveSandboxValidationResult) -> Self {
        Self { live_sandbox }
    }

    /// Returns the accepted live-sandbox validation result.
    #[must_use]
    pub const fn live_sandbox(&self) -> &GitHubPullRequestCommentLiveSandboxValidationResult {
        &self.live_sandbox
    }

    /// Returns the sandbox readiness decision that allowed the provider call.
    #[must_use]
    pub const fn readiness(&self) -> &crate::ProviderWriteSandboxReadinessResult {
        self.live_sandbox.readiness()
    }

    /// Returns the provider-call orchestration result.
    #[must_use]
    pub const fn provider_call(
        &self,
    ) -> &crate::GitHubPullRequestCommentProviderCallOrchestrationResult {
        self.live_sandbox.provider_call()
    }

    /// Returns whether the injected provider was called.
    #[must_use]
    pub const fn provider_call_performed(&self) -> bool {
        true
    }

    /// Returns whether this composition appended workflow events.
    #[must_use]
    pub const fn workflow_event_appended(&self) -> bool {
        false
    }

    /// Returns whether this composition wrote report artifacts.
    #[must_use]
    pub const fn report_artifact_written(&self) -> bool {
        false
    }

    /// Consumes the result into owned parts.
    #[must_use]
    pub fn into_parts(self) -> GitHubPrCommentLiveSandboxRuntimeCompositionParts {
        self.live_sandbox
    }
}

impl GitHubPrCommentLiveSandboxApprovalAuthorityCompositionResult {
    /// Creates a proof-bound live-sandbox result.
    #[must_use]
    pub const fn new(
        live_sandbox: GitHubPrCommentLiveSandboxRuntimeCompositionResult,
        approval_linkage: SideEffectApprovalLinkageFromStoreResult,
    ) -> Self {
        Self {
            live_sandbox,
            approval_linkage,
        }
    }

    /// Returns the accepted live-sandbox result.
    #[must_use]
    pub const fn live_sandbox(&self) -> &GitHubPrCommentLiveSandboxRuntimeCompositionResult {
        &self.live_sandbox
    }

    /// Returns bounded store-backed approval-linkage proof.
    #[must_use]
    pub const fn approval_linkage(&self) -> &SideEffectApprovalLinkageFromStoreResult {
        &self.approval_linkage
    }

    /// Returns whether the injected provider was called.
    #[must_use]
    pub const fn provider_call_performed(&self) -> bool {
        self.live_sandbox.provider_call_performed()
    }

    /// Consumes the result into owned parts.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        GitHubPrCommentLiveSandboxRuntimeCompositionResult,
        SideEffectApprovalLinkageFromStoreResult,
    ) {
        (self.live_sandbox, self.approval_linkage)
    }
}

impl fmt::Debug for GitHubPrCommentLiveSandboxApprovalAuthorityCompositionResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPrCommentLiveSandboxApprovalAuthorityCompositionResult")
            .field("live_sandbox", &self.live_sandbox)
            .field("approval_linkage", &self.approval_linkage)
            .finish()
    }
}

impl GitHubPrCommentLiveSandboxEventProofCompositionResult {
    /// Creates an event-proof composition result.
    #[must_use]
    pub const fn new(
        live_sandbox: GitHubPrCommentLiveSandboxRuntimeCompositionResult,
        run: WorkflowRun,
        status: GitHubPrCommentLiveSandboxEventProofStatus,
        event_append_error: Option<WorkflowOsError>,
    ) -> Self {
        Self {
            live_sandbox,
            run,
            status,
            event_append_error,
        }
    }

    /// Returns the accepted live sandbox runtime composition result.
    #[must_use]
    pub const fn live_sandbox(&self) -> &GitHubPrCommentLiveSandboxRuntimeCompositionResult {
        &self.live_sandbox
    }

    /// Returns the run after attempted event-proof projection.
    #[must_use]
    pub const fn run(&self) -> &WorkflowRun {
        &self.run
    }

    /// Returns the bounded event-proof composition status.
    #[must_use]
    pub const fn status(&self) -> GitHubPrCommentLiveSandboxEventProofStatus {
        self.status
    }

    /// Returns any non-leaking event append error.
    #[must_use]
    pub const fn event_append_error(&self) -> Option<&WorkflowOsError> {
        self.event_append_error.as_ref()
    }

    /// Returns whether the provider call happened in the already-composed live
    /// sandbox result.
    #[must_use]
    pub const fn provider_call_performed(&self) -> bool {
        self.live_sandbox.provider_call_performed()
    }

    /// Returns whether this helper appended durable workflow event proof.
    #[must_use]
    pub const fn workflow_event_appended(&self) -> bool {
        matches!(
            self.status,
            GitHubPrCommentLiveSandboxEventProofStatus::Appended
        )
    }

    /// Returns whether durable workflow event proof is present after this
    /// helper ran.
    #[must_use]
    pub const fn workflow_event_proof_present(&self) -> bool {
        matches!(
            self.status,
            GitHubPrCommentLiveSandboxEventProofStatus::Appended
                | GitHubPrCommentLiveSandboxEventProofStatus::AlreadyPresent
        )
    }

    /// Returns whether this composition wrote report artifacts.
    #[must_use]
    pub const fn report_artifact_written(&self) -> bool {
        false
    }

    /// Consumes the result into owned parts.
    #[must_use]
    pub fn into_parts(self) -> GitHubPrCommentLiveSandboxEventProofCompositionParts {
        (
            self.live_sandbox,
            self.run,
            self.status,
            self.event_append_error,
        )
    }
}

impl GitHubPrCommentProviderWriteArtifactGatedCompositionResult {
    /// Creates an artifact-gated provider-write composition result.
    #[must_use]
    pub const fn new(
        provider_write: GitHubPrCommentProviderWriteRuntimeCompositionResult,
        artifact_write: Option<WorkReportArtifactProofMarkerGovernedWriteResult>,
        artifact_write_error: Option<WorkflowOsError>,
    ) -> Self {
        Self {
            provider_write,
            artifact_write,
            artifact_write_error,
        }
    }

    /// Returns the provider-write runtime composition result.
    #[must_use]
    pub const fn provider_write(&self) -> &GitHubPrCommentProviderWriteRuntimeCompositionResult {
        &self.provider_write
    }

    /// Returns the artifact governance-gated write result when artifact writing
    /// succeeded.
    #[must_use]
    pub const fn artifact_write(
        &self,
    ) -> Option<&WorkReportArtifactProofMarkerGovernedWriteResult> {
        self.artifact_write.as_ref()
    }

    /// Returns artifact write/gate error when provider-write composition
    /// completed but artifact gates failed or the artifact store rejected the
    /// write.
    #[must_use]
    pub const fn artifact_write_error(&self) -> Option<&WorkflowOsError> {
        self.artifact_write_error.as_ref()
    }

    /// Returns the workflow run produced by local execution.
    #[must_use]
    pub const fn run(&self) -> &WorkflowRun {
        self.provider_write.run()
    }

    /// Returns bounded provider-write disclosure posture.
    #[must_use]
    pub const fn report_disclosure(
        &self,
    ) -> &GitHubPullRequestCommentProviderWriteReportDisclosure {
        self.provider_write.report_disclosure()
    }

    /// Returns bounded provider-write gate clarity.
    #[must_use]
    pub const fn gate_clarity(&self) -> &GitHubPullRequestCommentProviderWriteGateClarity {
        self.provider_write.gate_clarity()
    }

    /// Returns whether the injected provider was called or may have been called.
    #[must_use]
    pub fn provider_call_performed(&self) -> bool {
        self.provider_write.provider_call_performed()
    }

    /// Returns whether provider-write composition appended a completed/failed
    /// workflow event.
    #[must_use]
    pub const fn workflow_event_appended(&self) -> bool {
        self.provider_write.workflow_event_appended()
    }

    /// Returns whether this composition wrote a report artifact.
    #[must_use]
    pub const fn report_artifact_written(&self) -> bool {
        self.artifact_write.is_some()
    }

    /// Consumes the result into owned parts.
    #[must_use]
    pub fn into_parts(self) -> GitHubPrCommentProviderWriteArtifactGatedCompositionParts {
        (
            self.provider_write,
            self.artifact_write,
            self.artifact_write_error,
        )
    }
}

impl fmt::Debug for GitHubPrCommentProviderWriteRuntimeCompositionResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPrCommentProviderWriteRuntimeCompositionResult")
            .field("run_status", &self.provider_write.run().snapshot.status)
            .field("run_event_count", &self.provider_write.run().events.len())
            .field(
                "provider_write_error_code",
                &self
                    .provider_write
                    .provider_write_error()
                    .map(WorkflowOsError::code),
            )
            .field("provider_call_performed", &self.provider_call_performed())
            .field("workflow_event_appended", &self.workflow_event_appended())
            .field("report_artifact_written", &false)
            .field(
                "report_disclosure_posture",
                &self.report_disclosure.posture(),
            )
            .field("gate_clarity", self.provider_write.gate_clarity())
            .finish()
    }
}

impl fmt::Debug for GitHubPrCommentLiveSandboxRuntimeCompositionResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPrCommentLiveSandboxRuntimeCompositionResult")
            .field("readiness_decision", &self.readiness().decision())
            .field(
                "provider_response_outcome",
                &self.provider_call().provider_response().outcome(),
            )
            .field("provider_call_performed", &self.provider_call_performed())
            .field("workflow_event_appended", &false)
            .field("report_artifact_written", &false)
            .finish()
    }
}

impl fmt::Debug for GitHubPrCommentLiveSandboxEventProofCompositionResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPrCommentLiveSandboxEventProofCompositionResult")
            .field("live_sandbox", &self.live_sandbox)
            .field("run_status", &self.run.snapshot.status)
            .field("run_event_count", &self.run.events.len())
            .field("status", &self.status)
            .field(
                "event_append_error_code",
                &self.event_append_error.as_ref().map(WorkflowOsError::code),
            )
            .field("provider_call_performed", &self.provider_call_performed())
            .field("workflow_event_appended", &self.workflow_event_appended())
            .field(
                "workflow_event_proof_present",
                &self.workflow_event_proof_present(),
            )
            .field("report_artifact_written", &false)
            .finish()
    }
}

impl fmt::Debug for GitHubPrCommentProviderWriteArtifactGatedCompositionResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPrCommentProviderWriteArtifactGatedCompositionResult")
            .field("run_status", &self.run().snapshot.status)
            .field("run_event_count", &self.run().events.len())
            .field("provider_call_performed", &self.provider_call_performed())
            .field("workflow_event_appended", &self.workflow_event_appended())
            .field("report_artifact_written", &self.report_artifact_written())
            .field(
                "provider_disclosure_posture",
                &self.report_disclosure().posture(),
            )
            .field(
                "artifact_write_error_code",
                &self.artifact_write_error().map(WorkflowOsError::code),
            )
            .field("artifact_write_present", &self.artifact_write.is_some())
            .finish_non_exhaustive()
    }
}

impl fmt::Debug for LocalExecutionWithGitHubPrCommentProviderWriteResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalExecutionWithGitHubPrCommentProviderWriteResult")
            .field("run_status", &self.run.snapshot.status)
            .field("run_event_count", &self.run.events.len())
            .field("has_provider_response", &self.provider_response.is_some())
            .field(
                "outcome_lifecycle_state",
                &self
                    .outcome_transition
                    .as_ref()
                    .map(|transition| transition.record().lifecycle_state()),
            )
            .field(
                "reconciliation_status",
                &self
                    .reconciliation_candidate
                    .as_ref()
                    .map(GitHubPullRequestCommentProviderWriteReconciliationCandidate::status),
            )
            .field(
                "provider_write_error_code",
                &self
                    .provider_write_error
                    .as_ref()
                    .map(WorkflowOsError::code),
            )
            .field("provider_call_performed", &self.provider_call_performed())
            .field("workflow_event_appended", &self.workflow_event_appended)
            .field("report_artifact_written", &false)
            .field("retry_blocked", &self.retry_blocked())
            .field("operator_action_required", &self.operator_action_required())
            .field("gate_clarity", &self.gate_clarity)
            .finish()
    }
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

/// How an opt-in approval decision should resolve durable presentation proof.
#[derive(Clone, Eq, PartialEq)]
pub enum LocalApprovalPresentationProof {
    /// Load and validate one explicit presentation proof record.
    PresentationId(ApprovalPresentationId),
    /// Resolve proof from the pending run ID and approval ID.
    ///
    /// This fails closed when no proof exists or when more than one proof
    /// record exists for the approval.
    ResolveByRunAndApproval,
}

/// Request to submit a local approval decision only after validating durable
/// approval-presentation proof.
#[derive(Clone, Eq, PartialEq)]
pub struct LocalApprovalPresentationDecisionRequest {
    /// Existing local approval decision request.
    pub approval: LocalApprovalDecisionRequest,
    /// Durable presentation proof resolution strategy.
    pub proof: LocalApprovalPresentationProof,
    /// Optional maximum age for presentation proof.
    pub max_presentation_age: Option<Duration>,
}

impl fmt::Debug for LocalApprovalPresentationDecisionRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalApprovalPresentationDecisionRequest")
            .field("approval", &"[REDACTED]")
            .field(
                "proof",
                &match self.proof {
                    LocalApprovalPresentationProof::PresentationId(_) => "presentation_id",
                    LocalApprovalPresentationProof::ResolveByRunAndApproval => {
                        "resolve_by_run_and_approval"
                    }
                },
            )
            .field(
                "has_max_presentation_age",
                &self.max_presentation_age.is_some(),
            )
            .finish()
    }
}

/// How a caller wants the default approval-presentation enforcement boundary to
/// handle one approval decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApprovalPresentationDefaultEnforcementMode {
    /// Preserve ordinary approval behavior and require no presentation proof.
    NotRequired,
    /// Require matching durable presentation proof for this approval decision.
    Required,
    /// Require matching durable presentation proof only when the caller also
    /// supplies explicit bounded sensitive/write-adjacent posture.
    RequiredForSensitiveAction,
}

/// Caller-supplied bounded posture that makes proof required for
/// `RequiredForSensitiveAction`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApprovalPresentationSensitiveActionPosture {
    /// The approval is associated with high-assurance controls.
    HighAssurance,
    /// The approval is adjacent to a write-capable provider gate.
    WriteAdjacent,
    /// The approval is associated with side-effect governance.
    SideEffect,
}

/// Explicit policy for routing an approval decision through ordinary approval
/// behavior or the existing proof-enforced presentation path.
#[derive(Clone, Eq, PartialEq)]
pub struct ApprovalPresentationDefaultEnforcementPolicy {
    /// Enforcement mode for this approval decision.
    pub mode: ApprovalPresentationDefaultEnforcementMode,
    /// Durable presentation proof resolution strategy, required when proof is
    /// required.
    pub proof: Option<LocalApprovalPresentationProof>,
    /// Optional maximum proof age when proof is required.
    pub max_presentation_age: Option<Duration>,
    /// Explicit bounded posture required for `RequiredForSensitiveAction`.
    pub sensitive_action_posture: Option<ApprovalPresentationSensitiveActionPosture>,
}

impl ApprovalPresentationDefaultEnforcementPolicy {
    /// Preserve existing approval behavior.
    #[must_use]
    pub const fn not_required() -> Self {
        Self {
            mode: ApprovalPresentationDefaultEnforcementMode::NotRequired,
            proof: None,
            max_presentation_age: None,
            sensitive_action_posture: None,
        }
    }

    /// Require matching approval-presentation proof.
    #[must_use]
    pub fn required(proof: LocalApprovalPresentationProof) -> Self {
        Self {
            mode: ApprovalPresentationDefaultEnforcementMode::Required,
            proof: Some(proof),
            max_presentation_age: None,
            sensitive_action_posture: None,
        }
    }

    /// Require matching approval-presentation proof for a caller-declared
    /// sensitive/write-adjacent approval posture.
    #[must_use]
    pub fn required_for_sensitive_action(
        proof: LocalApprovalPresentationProof,
        sensitive_action_posture: ApprovalPresentationSensitiveActionPosture,
    ) -> Self {
        Self {
            mode: ApprovalPresentationDefaultEnforcementMode::RequiredForSensitiveAction,
            proof: Some(proof),
            max_presentation_age: None,
            sensitive_action_posture: Some(sensitive_action_posture),
        }
    }

    /// Adds a deterministic freshness bound for required proof.
    #[must_use]
    pub fn with_max_presentation_age(mut self, max_presentation_age: Duration) -> Self {
        self.max_presentation_age = Some(max_presentation_age);
        self
    }
}

impl fmt::Debug for ApprovalPresentationDefaultEnforcementPolicy {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ApprovalPresentationDefaultEnforcementPolicy")
            .field("mode", &self.mode)
            .field(
                "proof",
                &match self.proof {
                    Some(LocalApprovalPresentationProof::PresentationId(_)) => {
                        Some("presentation_id")
                    }
                    Some(LocalApprovalPresentationProof::ResolveByRunAndApproval) => {
                        Some("resolve_by_run_and_approval")
                    }
                    None => None,
                },
            )
            .field(
                "has_max_presentation_age",
                &self.max_presentation_age.is_some(),
            )
            .field("sensitive_action_posture", &self.sensitive_action_posture)
            .finish()
    }
}

/// Request to submit a local approval decision through an explicit
/// default-enforcement policy boundary.
#[derive(Clone, Eq, PartialEq)]
pub struct LocalApprovalPresentationDefaultDecisionRequest {
    /// Existing local approval decision request.
    pub approval: LocalApprovalDecisionRequest,
    /// Explicit default-enforcement policy for this decision.
    pub policy: ApprovalPresentationDefaultEnforcementPolicy,
}

impl fmt::Debug for LocalApprovalPresentationDefaultDecisionRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalApprovalPresentationDefaultDecisionRequest")
            .field("approval", &"[REDACTED]")
            .field("policy", &self.policy)
            .finish()
    }
}

/// Request to submit a local approval decision through explicit high-assurance controls.
#[derive(Clone, Eq, PartialEq)]
pub struct LocalHighAssuranceApprovalDecisionRequest {
    /// Existing local approval decision request.
    pub approval: LocalApprovalDecisionRequest,
    /// Explicit high-assurance controls to validate before appending decision events.
    pub controls: Vec<HighAssuranceApprovalControl>,
    /// Stable references supplied with the approval packet.
    pub supplied_references: Vec<HighAssuranceApprovalSuppliedReference>,
    /// Current time used for deterministic decision-time expiration checks.
    pub current_time: Timestamp,
}

impl fmt::Debug for LocalHighAssuranceApprovalDecisionRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalHighAssuranceApprovalDecisionRequest")
            .field("approval", &"[REDACTED]")
            .field("control_count", &self.controls.len())
            .field("supplied_reference_count", &self.supplied_references.len())
            .field("current_time", &self.current_time)
            .finish()
    }
}

/// Request to submit a high-assurance local approval decision through an
/// explicit approval-presentation policy boundary.
#[derive(Clone, Eq, PartialEq)]
pub struct LocalHighAssuranceApprovalPresentationDecisionRequest {
    /// Existing high-assurance local approval decision request.
    pub approval: LocalHighAssuranceApprovalDecisionRequest,
    /// Explicit approval-presentation policy for this decision.
    pub presentation_policy: ApprovalPresentationDefaultEnforcementPolicy,
}

impl fmt::Debug for LocalHighAssuranceApprovalPresentationDecisionRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalHighAssuranceApprovalPresentationDecisionRequest")
            .field("approval", &"[REDACTED]")
            .field("presentation_policy", &self.presentation_policy)
            .finish()
    }
}

/// In-memory result for an explicit high-assurance approval decision plus
/// report-safe disclosure.
#[derive(Clone, Eq, PartialEq)]
pub struct LocalHighAssuranceApprovalDecisionWithDisclosureResult {
    run: WorkflowRun,
    high_assurance_approval: WorkReportHighAssuranceApprovalDisclosure,
}

impl LocalHighAssuranceApprovalDecisionWithDisclosureResult {
    /// Creates a high-assurance approval disclosure result.
    #[must_use]
    pub const fn new(
        run: WorkflowRun,
        high_assurance_approval: WorkReportHighAssuranceApprovalDisclosure,
    ) -> Self {
        Self {
            run,
            high_assurance_approval,
        }
    }

    /// Returns the workflow run produced by the approval decision.
    #[must_use]
    pub const fn run(&self) -> &WorkflowRun {
        &self.run
    }

    /// Returns the report-safe high-assurance approval disclosure.
    #[must_use]
    pub const fn high_assurance_approval(&self) -> &WorkReportHighAssuranceApprovalDisclosure {
        &self.high_assurance_approval
    }

    /// Consumes the result into owned parts.
    #[must_use]
    pub fn into_parts(self) -> (WorkflowRun, WorkReportHighAssuranceApprovalDisclosure) {
        (self.run, self.high_assurance_approval)
    }
}

impl fmt::Debug for LocalHighAssuranceApprovalDecisionWithDisclosureResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalHighAssuranceApprovalDecisionWithDisclosureResult")
            .field("run_status", &self.run.snapshot.status)
            .field("run_event_count", &self.run.events.len())
            .field("high_assurance_approval", &"[REDACTED]")
            .field("has_high_assurance_approval", &true)
            .finish()
    }
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

/// Minimal local executor for local-handler workflows.
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

    /// Loads, validates, and executes a local workflow.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the project cannot be loaded or
    /// validated, the workflow is outside the v0 executor scope, state cannot be
    /// persisted, a local handler is missing, or output contract checks fail.
    pub fn execute(&self, request: &LocalExecutionRequest) -> Result<WorkflowRun, WorkflowOsError> {
        self.execute_with_validation_capability(request, &ProjectValidationCapability::Default)
    }

    fn execute_with_validation_capability(
        &self,
        request: &LocalExecutionRequest,
        validation_capability: &ProjectValidationCapability,
    ) -> Result<WorkflowRun, WorkflowOsError> {
        let run_id = request
            .run_id
            .clone()
            .unwrap_or_else(WorkflowRunId::generate);
        if !self.backend.read_events(&run_id)?.is_empty() {
            return self.backend.rehydrate_run(&run_id);
        }

        let mut plan =
            Self::prepare_execution_with_capability(request, run_id, validation_capability)?;
        self.evaluate_pre_run_policy(&plan, &request.actor, &request.correlation_id)?;
        self.append_run_start(&mut plan)?;
        self.execute_steps(plan, &request.correlation_id)
    }

    /// Executes a local workflow and derives an in-memory report result.
    ///
    /// Existing executor methods and workflow semantics remain unchanged. If
    /// workflow execution itself fails before producing a run, the execution
    /// error is returned. If execution produces a run but report generation
    /// fails, the run is returned with a structured report-generation error and
    /// no report.
    ///
    /// # Errors
    ///
    /// Returns the same structured errors as `execute(...)` when execution
    /// fails before a workflow run exists.
    pub fn execute_with_report(
        &self,
        request: &LocalExecutionWithReportRequest,
    ) -> Result<LocalExecutionWithReportResult, WorkflowOsError> {
        let run = self.execute(&request.execution)?;
        let mut report = request.report.clone();
        if run.snapshot.status.is_terminal() {
            if let Err(error) = apply_before_report_hook_checkpoint(&run, &mut report) {
                return Ok(LocalExecutionWithReportResult::new(run, None, Some(error)));
            }
        }
        let input = terminal_report_input_for_run(&run, &report);
        match expose_terminal_local_work_report_result(input) {
            Ok(result) => {
                let (run, work_report) = result.into_parts();
                Ok(LocalExecutionWithReportResult::new(
                    run,
                    Some(work_report),
                    None,
                ))
            }
            Err(error) => Ok(LocalExecutionWithReportResult::new(run, None, Some(error))),
        }
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
        let (run, approval, decision) = self.prepare_approval_decision(&request)?;
        let LocalApprovalDecisionRequest {
            project_root,
            correlation_id,
            ..
        } = request;
        self.apply_approval_decision(&project_root, &correlation_id, &run, &approval, decision)
    }

    /// Applies a local approval decision after opt-in durable presentation
    /// proof validation.
    ///
    /// Existing approval behavior remains unchanged: this method is an
    /// additive fail-closed path for callers that require proof the approval
    /// scope was presented before the decision was accepted.
    ///
    /// # Errors
    ///
    /// Returns the same structured state errors as `decide_approval(...)`, or a
    /// stable approval-presentation enforcement error before mutating runtime
    /// state.
    pub fn decide_approval_with_presentation(
        &self,
        request: LocalApprovalPresentationDecisionRequest,
    ) -> Result<WorkflowRun, WorkflowOsError> {
        let LocalApprovalPresentationDecisionRequest {
            approval: approval_request,
            proof,
            max_presentation_age,
        } = request;
        let (run, approval, decision) = self.prepare_approval_decision(&approval_request)?;
        let presentation = self.resolve_approval_presentation_proof(&approval, &proof)?;
        validate_approval_presentation_enforcement(
            &presentation,
            &approval,
            &decision,
            max_presentation_age,
        )?;
        let proof_marker =
            approval_decision_proof_marker(&presentation, &decision, max_presentation_age)?;
        let decision = ApprovalDecision {
            proof_marker: Some(proof_marker),
            ..decision
        };
        let LocalApprovalDecisionRequest {
            project_root,
            correlation_id,
            ..
        } = approval_request;
        self.apply_approval_decision(&project_root, &correlation_id, &run, &approval, decision)
    }

    /// Applies a local approval decision through an explicit
    /// approval-presentation default-enforcement policy.
    ///
    /// This method is additive. It does not change
    /// `LocalExecutor::decide_approval(...)`; callers must opt into this
    /// policy boundary explicitly.
    ///
    /// # Errors
    ///
    /// Returns the same structured errors as `decide_approval(...)` when proof
    /// is not required, the same structured errors as
    /// `decide_approval_with_presentation(...)` when proof is required, or a
    /// stable default-enforcement policy error before mutating runtime state.
    pub fn decide_approval_with_default_presentation_policy(
        &self,
        request: LocalApprovalPresentationDefaultDecisionRequest,
    ) -> Result<WorkflowRun, WorkflowOsError> {
        let LocalApprovalPresentationDefaultDecisionRequest { approval, policy } = request;
        let approval_request = approval;
        let (run, approval, decision) = self.prepare_approval_decision(&approval_request)?;
        let decision =
            self.approval_decision_with_presentation_policy(&approval, decision, &policy, None)?;
        let LocalApprovalDecisionRequest {
            project_root,
            correlation_id,
            ..
        } = approval_request;
        self.apply_approval_decision(&project_root, &correlation_id, &run, &approval, decision)
    }

    /// Applies a local approval decision after explicit high-assurance validation.
    ///
    /// Existing approval behavior remains opt-in: this method does not change
    /// `decide_approval(...)`. High-assurance validation runs before any
    /// approval decision event is appended.
    ///
    /// # Errors
    ///
    /// Returns the same structured state errors as `decide_approval(...)`, or a
    /// stable high-assurance validation error before mutating runtime state.
    pub fn decide_approval_with_high_assurance(
        &self,
        request: LocalHighAssuranceApprovalDecisionRequest,
    ) -> Result<WorkflowRun, WorkflowOsError> {
        let LocalHighAssuranceApprovalDecisionRequest {
            approval: approval_request,
            controls,
            supplied_references,
            current_time,
        } = request;
        let (run, approval, decision) = self.prepare_approval_decision(&approval_request)?;
        validate_high_assurance_approval_decision(&HighAssuranceApprovalDecisionValidationInput {
            approval_request: &approval,
            approval_decision: &decision,
            controls: &controls,
            supplied_references: &supplied_references,
            current_time,
        })?;
        let LocalApprovalDecisionRequest {
            project_root,
            correlation_id,
            ..
        } = approval_request;
        self.apply_approval_decision(&project_root, &correlation_id, &run, &approval, decision)
    }

    /// Applies a high-assurance local approval decision through an explicit
    /// approval-presentation policy boundary.
    ///
    /// High-assurance controls and approval-presentation proof are validated
    /// before any approval decision event is appended.
    ///
    /// # Errors
    ///
    /// Returns the same structured state and high-assurance errors as
    /// `decide_approval_with_high_assurance(...)`, the same structured proof
    /// validation errors as the proof-enforced approval path, or a stable
    /// policy error before mutating runtime state.
    pub fn decide_approval_with_high_assurance_presentation_policy(
        &self,
        request: LocalHighAssuranceApprovalPresentationDecisionRequest,
    ) -> Result<WorkflowRun, WorkflowOsError> {
        let LocalHighAssuranceApprovalPresentationDecisionRequest {
            approval:
                LocalHighAssuranceApprovalDecisionRequest {
                    approval: approval_request,
                    controls,
                    supplied_references,
                    current_time,
                },
            presentation_policy,
        } = request;
        let (run, approval, decision) = self.prepare_approval_decision(&approval_request)?;
        validate_high_assurance_approval_decision(&HighAssuranceApprovalDecisionValidationInput {
            approval_request: &approval,
            approval_decision: &decision,
            controls: &controls,
            supplied_references: &supplied_references,
            current_time,
        })?;
        let decision = self.approval_decision_with_presentation_policy(
            &approval,
            decision,
            &presentation_policy,
            Some(ApprovalPresentationSensitiveActionPosture::HighAssurance),
        )?;
        let LocalApprovalDecisionRequest {
            project_root,
            correlation_id,
            ..
        } = approval_request;
        self.apply_approval_decision(&project_root, &correlation_id, &run, &approval, decision)
    }

    /// Applies a local high-assurance approval decision and returns
    /// report-safe disclosure for explicit later report generation.
    ///
    /// This is an additive in-memory bridge. It does not change existing
    /// approval methods, generate reports automatically, append disclosure
    /// events, write artifacts, or persist disclosure records.
    ///
    /// # Errors
    ///
    /// Returns the same structured state and validation errors as
    /// `decide_approval_with_high_assurance(...)`, or a stable non-leaking
    /// disclosure integration error before mutating runtime state.
    pub fn decide_approval_with_high_assurance_disclosure(
        &self,
        request: LocalHighAssuranceApprovalDecisionRequest,
    ) -> Result<LocalHighAssuranceApprovalDecisionWithDisclosureResult, WorkflowOsError> {
        let LocalHighAssuranceApprovalDecisionRequest {
            approval: approval_request,
            controls,
            supplied_references,
            current_time,
        } = request;
        let (run, approval, decision) = self.prepare_approval_decision(&approval_request)?;
        validate_high_assurance_approval_decision(&HighAssuranceApprovalDecisionValidationInput {
            approval_request: &approval,
            approval_decision: &decision,
            controls: &controls,
            supplied_references: &supplied_references,
            current_time,
        })?;
        let high_assurance_approval = high_assurance_disclosure_from_validated_controls(
            decision.decision,
            &controls,
            supplied_references.len(),
        )?;
        let LocalApprovalDecisionRequest {
            project_root,
            correlation_id,
            ..
        } = approval_request;
        let run = self.apply_approval_decision(
            &project_root,
            &correlation_id,
            &run,
            &approval,
            decision,
        )?;
        Ok(LocalHighAssuranceApprovalDecisionWithDisclosureResult::new(
            run,
            high_assurance_approval,
        ))
    }

    /// Applies a high-assurance approval decision through an explicit
    /// approval-presentation policy boundary and returns report-safe
    /// disclosure for later explicit report generation.
    ///
    /// This is an additive in-memory bridge. It does not change existing
    /// approval methods, generate reports automatically, append disclosure
    /// events, write artifacts, or persist disclosure records.
    ///
    /// # Errors
    ///
    /// Returns structured state, high-assurance, presentation-proof, or
    /// disclosure integration errors before mutating runtime state.
    pub fn decide_approval_with_high_assurance_presentation_policy_disclosure(
        &self,
        request: LocalHighAssuranceApprovalPresentationDecisionRequest,
    ) -> Result<LocalHighAssuranceApprovalDecisionWithDisclosureResult, WorkflowOsError> {
        let LocalHighAssuranceApprovalPresentationDecisionRequest {
            approval:
                LocalHighAssuranceApprovalDecisionRequest {
                    approval: approval_request,
                    controls,
                    supplied_references,
                    current_time,
                },
            presentation_policy,
        } = request;
        let (run, approval, decision) = self.prepare_approval_decision(&approval_request)?;
        validate_high_assurance_approval_decision(&HighAssuranceApprovalDecisionValidationInput {
            approval_request: &approval,
            approval_decision: &decision,
            controls: &controls,
            supplied_references: &supplied_references,
            current_time,
        })?;
        let decision = self.approval_decision_with_presentation_policy(
            &approval,
            decision,
            &presentation_policy,
            Some(ApprovalPresentationSensitiveActionPosture::HighAssurance),
        )?;
        let high_assurance_approval = high_assurance_disclosure_from_validated_controls(
            decision.decision,
            &controls,
            supplied_references.len(),
        )?;
        let LocalApprovalDecisionRequest {
            project_root,
            correlation_id,
            ..
        } = approval_request;
        let run = self.apply_approval_decision(
            &project_root,
            &correlation_id,
            &run,
            &approval,
            decision,
        )?;
        Ok(LocalHighAssuranceApprovalDecisionWithDisclosureResult::new(
            run,
            high_assurance_approval,
        ))
    }

    fn prepare_approval_decision(
        &self,
        request: &LocalApprovalDecisionRequest,
    ) -> Result<(WorkflowRun, ApprovalRequest, ApprovalDecision), WorkflowOsError> {
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

        let decision = ApprovalDecision {
            approval_id: request.approval_id.clone(),
            actor: request.actor.clone(),
            decided_at: Timestamp::now_utc(),
            decision: request.decision,
            reason: request.reason.clone(),
            correlation_id: request.correlation_id.clone(),
            proof_marker: None,
        };

        Ok((run, approval, decision))
    }

    fn approval_decision_with_presentation_policy(
        &self,
        approval: &ApprovalRequest,
        decision: ApprovalDecision,
        policy: &ApprovalPresentationDefaultEnforcementPolicy,
        required_sensitive_posture: Option<ApprovalPresentationSensitiveActionPosture>,
    ) -> Result<ApprovalDecision, WorkflowOsError> {
        let proof = match policy.mode {
            ApprovalPresentationDefaultEnforcementMode::NotRequired => {
                if policy.proof.is_some()
                    || policy.max_presentation_age.is_some()
                    || policy.sensitive_action_posture.is_some()
                {
                    return Err(approval_presentation_default_enforcement_error(
                        "approval_presentation_default_enforcement.proof_not_required",
                        "approval-presentation proof is not required by this policy",
                    ));
                }
                return Ok(decision);
            }
            ApprovalPresentationDefaultEnforcementMode::Required => {
                policy.proof.as_ref().ok_or_else(|| {
                    approval_presentation_default_enforcement_error(
                        "approval_presentation_default_enforcement.proof_missing",
                        "approval-presentation proof is required",
                    )
                })?
            }
            ApprovalPresentationDefaultEnforcementMode::RequiredForSensitiveAction => {
                let posture = policy.sensitive_action_posture.ok_or_else(|| {
                    approval_presentation_default_enforcement_error(
                        "approval_presentation_default_enforcement.sensitive_posture_missing",
                        "explicit sensitive approval posture is required",
                    )
                })?;
                if let Some(required_posture) = required_sensitive_posture {
                    if posture != required_posture {
                        return Err(approval_presentation_default_enforcement_error(
                            "approval_presentation_default_enforcement.sensitive_posture_mismatch",
                            "explicit sensitive approval posture does not match this approval path",
                        ));
                    }
                }
                policy.proof.as_ref().ok_or_else(|| {
                    approval_presentation_default_enforcement_error(
                        "approval_presentation_default_enforcement.proof_missing",
                        "approval-presentation proof is required",
                    )
                })?
            }
        };
        let presentation = self.resolve_approval_presentation_proof(approval, proof)?;
        validate_approval_presentation_enforcement(
            &presentation,
            approval,
            &decision,
            policy.max_presentation_age,
        )?;
        let proof_marker =
            approval_decision_proof_marker(&presentation, &decision, policy.max_presentation_age)?;
        Ok(ApprovalDecision {
            proof_marker: Some(proof_marker),
            ..decision
        })
    }

    fn apply_approval_decision(
        &self,
        project_root: &std::path::Path,
        correlation_id: &CorrelationId,
        run: &WorkflowRun,
        approval: &ApprovalRequest,
        decision: ApprovalDecision,
    ) -> Result<WorkflowRun, WorkflowOsError> {
        let mut builder = EventBuilder::from_snapshot(
            &run.snapshot,
            decision.correlation_id.clone(),
            decision.actor.clone(),
        );
        match decision.decision {
            ApprovalDecisionKind::Granted => {
                let mut plan = Self::prepare_resume_execution(project_root, &builder, approval)?;
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
                    policy_effects: PolicyEffectSet::default(),
                    adapter_id: None,
                    correlation_id: Some(builder.correlation_id.clone()),
                };
                self.evaluate_and_record_policy(&mut builder, &resume_context)?;
                self.append(&mut builder, WorkflowRunEventKind::RunResumed, None)?;
                plan.event_builder = builder;
                self.execute_steps(plan, correlation_id)
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

    fn resolve_approval_presentation_proof(
        &self,
        approval: &ApprovalRequest,
        proof: &LocalApprovalPresentationProof,
    ) -> Result<ApprovalPresentationRecord, WorkflowOsError> {
        match proof {
            LocalApprovalPresentationProof::PresentationId(presentation_id) => self
                .backend
                .read_approval_presentation_record(presentation_id)
                .map_err(|_| {
                    approval_presentation_enforcement_error(
                        "approval_presentation_enforcement.proof_corrupt",
                        "approval-presentation proof could not be read or validated",
                    )
                })?
                .ok_or_else(|| {
                    approval_presentation_enforcement_error(
                        "approval_presentation_enforcement.proof_missing",
                        "approval-presentation proof is required",
                    )
                }),
            LocalApprovalPresentationProof::ResolveByRunAndApproval => {
                let records = self
                    .backend
                    .list_approval_presentation_records_for_approval(
                        &approval.run_id,
                        &approval.approval_id,
                    )
                    .map_err(|_| {
                        approval_presentation_enforcement_error(
                            "approval_presentation_enforcement.proof_corrupt",
                            "approval-presentation proof could not be read or validated",
                        )
                    })?;
                match records.as_slice() {
                    [] => Err(approval_presentation_enforcement_error(
                        "approval_presentation_enforcement.proof_missing",
                        "approval-presentation proof is required",
                    )),
                    [record] => Ok(record.clone()),
                    _ => Err(approval_presentation_enforcement_error(
                        "approval_presentation_enforcement.proof_ambiguous",
                        "approval-presentation proof is ambiguous",
                    )),
                }
            }
        }
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
            policy_effects: PolicyEffectSet::default(),
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
            before_skill_invocation_checkpoints:
                LocalExecutionBeforeSkillInvocationCheckpointInputs::default(),
            before_skill_invocation_hook: None,
            side_effect_events: Vec::new(),
            side_effect_lifecycle_events: Vec::new(),
        };
        let plan = Self::prepare_execution(&request, WorkflowRunId::generate())?;
        Ok(plan.timeout_policy)
    }

    fn prepare_execution(
        request: &LocalExecutionRequest,
        run_id: WorkflowRunId,
    ) -> Result<ExecutionPlan, WorkflowOsError> {
        Self::prepare_execution_with_capability(
            request,
            run_id,
            &ProjectValidationCapability::Default,
        )
    }

    fn prepare_execution_with_capability(
        request: &LocalExecutionRequest,
        run_id: WorkflowRunId,
        validation_capability: &ProjectValidationCapability,
    ) -> Result<ExecutionPlan, WorkflowOsError> {
        Self::prepare_execution_with_capability_and_artifact_policy(
            request,
            run_id,
            validation_capability,
            None,
        )
    }

    fn prepare_execution_with_capability_and_artifact_policy(
        request: &LocalExecutionRequest,
        run_id: WorkflowRunId,
        validation_capability: &ProjectValidationCapability,
        caller_proof_marker_policy: Option<WorkReportArtifactApprovalProofMarkerGatePolicy>,
    ) -> Result<ExecutionPlan, WorkflowOsError> {
        let checkpoint_inputs = before_skill_invocation_checkpoint_inputs(request)?;
        let bundle =
            load_validated_project_bundle(&request.project_root, validation_capability.clone())?;
        let workflow = find_workflow(&bundle.workflows, &request.workflow_id)?;
        reject_branch_execution(workflow)?;
        validate_before_skill_invocation_required_steps(
            &checkpoint_inputs,
            &workflow.definition.steps,
        )?;
        let steps = step_execution_plans(
            &workflow.definition.steps,
            &bundle.skills,
            &bundle.policies,
            &run_id,
            &workflow.definition.id,
            &workflow.definition.version,
        )?;
        let first_step = steps.first().cloned().ok_or_else(|| {
            executor_error(
                WorkflowOsErrorKind::Unsupported,
                "executor.workflow.no_steps",
                "local executor requires at least one step",
            )
        })?;
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
        let approval_expires_after = approval_expires_after(&workflow.definition);
        let workflow_report_artifact_policies =
            Self::derive_execution_plan_report_artifact_policies(
                workflow,
                validation_capability,
                caller_proof_marker_policy,
            )?;
        let resolved_execution_context_hash = resolved_execution_context_hash(
            workflow,
            &bundle.skills,
            &bundle.policies,
            &checkpoint_inputs,
            request,
            workflow_report_artifact_policies,
        )?;

        Ok(ExecutionPlan {
            event_builder: EventBuilder::new(
                run_id,
                workflow_id,
                schema_version,
                workflow_version,
                spec_hash,
                request.correlation_id.clone(),
                request.actor.clone(),
            ),
            resolved_execution_context_hash,
            steps,
            current_step_index: 0,
            step_scheduled: false,
            approval_already_granted: false,
            step: first_step.step,
            skill: first_step.skill,
            skill_id: first_step.skill_id,
            skill_version: first_step.skill_version,
            invocation_id: first_step.invocation_id,
            idempotency_key: first_step.idempotency_key,
            approval_expires_after,
            retry_max_attempts: first_step.retry_max_attempts,
            escalation_enabled: first_step.escalation_enabled,
            escalation_contact,
            timeout_policy,
            autonomy_level: workflow.definition.autonomy_level,
            approval_sensitivity: first_step.approval_sensitivity,
            adapter_id: first_step.adapter_id,
            capabilities: first_step.capabilities,
            policy_effects: first_step.policy_effects,
            before_skill_invocation_checkpoints: checkpoint_inputs,
            before_skill_invocation_hook: request.before_skill_invocation_hook.clone(),
            side_effect_events: request.side_effect_events.clone(),
            side_effect_lifecycle_events: request.side_effect_lifecycle_events.clone(),
            workflow_report_artifact_policy: workflow_report_artifact_policies
                .high_assurance_disclosure_policy,
            workflow_report_artifact_proof_marker_policy: workflow_report_artifact_policies
                .approval_proof_marker_policy,
        })
    }

    fn derive_execution_plan_report_artifact_policies(
        workflow: &LoadedSpec<WorkflowDefinition>,
        validation_capability: &ProjectValidationCapability,
        caller_proof_marker_policy: Option<WorkReportArtifactApprovalProofMarkerGatePolicy>,
    ) -> Result<WorkflowReportArtifactPolicies, WorkflowOsError> {
        let high_assurance_disclosure_policy = derive_workflow_report_artifact_gate_policy(
            WorkflowReportArtifactGateDerivationInput {
                workflow: &workflow.definition,
            },
        )?
        .high_assurance_disclosure_policy();
        let proof_marker_derivation_mode = match validation_capability {
            ProjectValidationCapability::ReportArtifactCapable {
                workflow_id,
                approval_proof_marker_capable,
            } if workflow_id == &workflow.definition.id && *approval_proof_marker_capable => {
                WorkflowReportArtifactProofMarkerDerivationMode::ArtifactCapable
            }
            _ => WorkflowReportArtifactProofMarkerDerivationMode::DefaultValidation,
        };
        let approval_proof_marker_policy =
            derive_workflow_report_artifact_approval_proof_marker_gate_policy(
                WorkflowReportArtifactProofMarkerGateDerivationInput {
                    workflow: &workflow.definition,
                    caller_policy: caller_proof_marker_policy,
                    derivation_mode: proof_marker_derivation_mode,
                },
            )?
            .approval_proof_marker_policy();

        Ok(WorkflowReportArtifactPolicies {
            high_assurance_disclosure_policy,
            approval_proof_marker_policy,
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
        Ok(())
    }

    fn append_step_scheduled(&self, plan: &mut ExecutionPlan) -> Result<(), WorkflowOsError> {
        self.append(
            &mut plan.event_builder,
            WorkflowRunEventKind::StepScheduled {
                step_id: plan.step.id.clone(),
            },
            None,
        )
    }

    fn execute_steps(
        &self,
        mut plan: ExecutionPlan,
        correlation_id: &CorrelationId,
    ) -> Result<WorkflowRun, WorkflowOsError> {
        loop {
            if !plan.step_scheduled {
                self.append_step_scheduled(&mut plan)?;
                plan.step_scheduled = true;
            }

            if plan.policy_effects.requires_approval() && !plan.approval_already_granted {
                return self.pause_for_approval(plan);
            }

            match self.invoke_local_skill(plan, correlation_id)? {
                StepExecutionResult::Terminal(run) => return Ok(*run),
                StepExecutionResult::Succeeded(succeeded_plan) => {
                    let mut succeeded_plan = *succeeded_plan;
                    if !succeeded_plan.should_continue_after_success() {
                        return self.complete_run(succeeded_plan.event_builder);
                    }
                    if !succeeded_plan.has_next_step() {
                        return self.fail_run(
                            succeeded_plan.event_builder,
                            "executor.workflow.multistep.no_next_step",
                            "step requested continuation but no next step is available",
                        );
                    }
                    succeeded_plan.advance_to_next_step()?;
                    plan = succeeded_plan;
                }
            }
        }
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
            policy_effects: plan.policy_effects.clone(),
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
            resolved_execution_context_hash: Some(plan.resolved_execution_context_hash.clone()),
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
        builder: &EventBuilder,
        approval: &ApprovalRequest,
    ) -> Result<ExecutionPlan, WorkflowOsError> {
        let request = LocalExecutionRequest {
            project_root: project_root.to_path_buf(),
            workflow_id: builder.workflow_id.clone(),
            run_id: Some(builder.run_id.clone()),
            correlation_id: builder.correlation_id.clone(),
            actor: builder.actor.clone(),
            before_skill_invocation_checkpoints:
                LocalExecutionBeforeSkillInvocationCheckpointInputs::default(),
            before_skill_invocation_hook: None,
            side_effect_events: Vec::new(),
            side_effect_lifecycle_events: Vec::new(),
        };
        let mut plan = Self::prepare_execution(&request, builder.run_id.clone()).map_err(|_| {
            executor_error(
                WorkflowOsErrorKind::InvalidState,
                "executor.approval.resume_context_unavailable",
                "approval resume context could not be reconstructed safely",
            )
        })?;
        if plan.event_builder.workflow_id != builder.workflow_id
            || plan.event_builder.schema_version != builder.schema_version
            || plan.event_builder.workflow_version != builder.workflow_version
            || plan.event_builder.spec_hash != builder.spec_hash
        {
            return Err(executor_error(
                WorkflowOsErrorKind::InvalidState,
                "executor.approval.workflow_identity_mismatch",
                "approval resume workflow identity does not match the approved run",
            ));
        }
        let approved_context_hash = approval
            .resolved_execution_context_hash
            .as_ref()
            .ok_or_else(|| {
                executor_error(
                    WorkflowOsErrorKind::InvalidState,
                    "executor.approval.resume_context_missing",
                    "approval resume requires a resolved execution-context commitment",
                )
            })?;
        if &plan.resolved_execution_context_hash != approved_context_hash {
            return Err(executor_error(
                WorkflowOsErrorKind::InvalidState,
                "executor.approval.resume_context_mismatch",
                "approval resume context does not match the context originally approved",
            ));
        }
        let step_index = plan
            .steps
            .iter()
            .position(|candidate| candidate.step.id == approval.step_id)
            .ok_or_else(|| {
                executor_error(
                    WorkflowOsErrorKind::InvalidState,
                    "executor.workflow.multistep.resume_step_missing",
                    "approval resume step is not present in workflow definition",
                )
            })?;
        plan.set_current_step(step_index)?;
        if plan.skill_id != approval.skill_id || plan.skill_version != approval.skill_version {
            return Err(executor_error(
                WorkflowOsErrorKind::InvalidState,
                "executor.approval.resume_context_mismatch",
                "approval resume step does not match the context originally approved",
            ));
        }
        plan.step_scheduled = true;
        plan.approval_already_granted = true;
        Ok(plan)
    }

    fn invoke_local_skill(
        &self,
        mut plan: ExecutionPlan,
        correlation_id: &CorrelationId,
    ) -> Result<StepExecutionResult, WorkflowOsError> {
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
            has_approval_policy: plan.policy_effects.requires_approval(),
            policy_effects: plan.policy_effects.clone(),
            adapter_id: plan.adapter_id.clone(),
            correlation_id: Some(plan.event_builder.correlation_id.clone()),
        };
        if let Err(error) =
            self.evaluate_and_record_policy(&mut plan.event_builder, &invoke_context)
        {
            return self.fail_step_from_error(plan, &error);
        }
        let Some(handler) = self.registry.get(&plan.skill_id, &plan.skill_version) else {
            return self
                .fail_run(
                    plan.event_builder,
                    "executor.skill_handler.missing",
                    format!(
                        "no local handler registered for {}@{}",
                        plan.skill_id, plan.skill_version
                    ),
                )
                .map(|run| StepExecutionResult::Terminal(Box::new(run)));
        };
        if let Err(error) = self.append_side_effect_events(&mut plan) {
            return self.fail_step_from_error(plan, &error);
        }
        if let Err(error) = self.append_before_skill_invocation_hook(&mut plan) {
            return self.fail_step_from_error(plan, &error);
        }
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
                return self
                    .backend
                    .rehydrate_run(&plan.event_builder.run_id)
                    .map(|run| StepExecutionResult::Terminal(Box::new(run)));
            }

            self.append_skill_attempt_started(&mut plan, attempt_number, &attempt_key)?;
            let input = SkillInput {
                run_id: plan.event_builder.run_id.clone(),
                workflow_id: plan.event_builder.workflow_id.clone(),
                workflow_version: plan.event_builder.workflow_version.clone(),
                schema_version: plan.event_builder.schema_version.clone(),
                spec_hash: plan.event_builder.spec_hash.clone(),
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
                    return self
                        .exhaust_retries(plan, max_attempts, &error)
                        .map(|run| StepExecutionResult::Terminal(Box::new(run)));
                }
            }
        }
        self.fail_run(
            plan.event_builder,
            "executor.retry.invalid_state",
            "retry loop ended without a terminal runtime event",
        )
        .map(|run| StepExecutionResult::Terminal(Box::new(run)))
    }

    fn fail_step_from_error(
        &self,
        plan: ExecutionPlan,
        error: &WorkflowOsError,
    ) -> Result<StepExecutionResult, WorkflowOsError> {
        self.fail_run(
            plan.event_builder,
            error.code().to_owned(),
            error.message().to_owned(),
        )
        .map(|run| StepExecutionResult::Terminal(Box::new(run)))
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

    fn append_before_skill_invocation_hook(
        &self,
        plan: &mut ExecutionPlan,
    ) -> Result<(), WorkflowOsError> {
        let requires_hook = plan
            .before_skill_invocation_checkpoints
            .requires_step(&plan.step.id);
        let Some(hook_input) = plan.before_skill_invocation_hook.clone() else {
            return if requires_hook {
                Err(required_before_skill_invocation_hook_error())
            } else {
                Ok(())
            };
        };
        if hook_input.step_id != plan.step.id {
            return if requires_hook {
                Err(required_before_skill_invocation_hook_error())
            } else {
                Ok(())
            };
        }
        validate_before_skill_invocation_hook_input(plan, &hook_input)?;

        let runtime_input = RuntimeAgentHarnessHookInput {
            hook_invocation_id: hook_input.hook_invocation_id.clone(),
            invocation: hook_input.invocation.clone(),
        };
        let result = match hook_input.result_status {
            AgentHarnessHookInvocationStatus::Passed => {
                execute_runtime_agent_harness_hook(runtime_input)?
            }
            AgentHarnessHookInvocationStatus::FailedClosed => {
                execute_runtime_agent_harness_hook_failed_closed(runtime_input)?
            }
            AgentHarnessHookInvocationStatus::Warning
            | AgentHarnessHookInvocationStatus::SkippedWithDisclosure
            | AgentHarnessHookInvocationStatus::Blocked => {
                return Err(executor_error(
                    WorkflowOsErrorKind::InvalidState,
                    "executor.hook.before_skill_invocation.unsupported_status",
                    "before-skill-invocation hook status is not supported by this phase",
                ));
            }
        };
        let status = result.invocation_result().status();

        let requested = hook_workflow_event_from_input(&hook_input, status)?;
        let evaluated = hook_workflow_event_from_input(&hook_input, status)?;
        self.append(
            &mut plan.event_builder,
            WorkflowRunEventKind::HookInvocationRequested(Box::new(requested)),
            Some(hook_requested_idempotency_key(&plan.idempotency_key)?),
        )?;
        self.append(
            &mut plan.event_builder,
            WorkflowRunEventKind::HookInvocationEvaluated(Box::new(evaluated)),
            Some(hook_evaluated_idempotency_key(&plan.idempotency_key)?),
        )?;
        if status == AgentHarnessHookInvocationStatus::FailedClosed {
            return Err(executor_error(
                WorkflowOsErrorKind::InvalidState,
                "executor.hook.before_skill_invocation.failed_closed",
                "before-skill-invocation hook failed closed before skill invocation",
            ));
        }
        Ok(())
    }

    fn append_side_effect_events(&self, plan: &mut ExecutionPlan) -> Result<(), WorkflowOsError> {
        for (index, input) in plan.side_effect_events.clone().into_iter().enumerate() {
            if input.step_id != plan.step.id {
                continue;
            }
            validate_side_effect_event_input(plan, &input)?;
            let lifecycle = input.event.lifecycle_state();
            let event_kind = match lifecycle {
                SideEffectLifecycleState::Proposed => {
                    WorkflowRunEventKind::SideEffectProposed(Box::new(input.event))
                }
                SideEffectLifecycleState::Denied => {
                    WorkflowRunEventKind::SideEffectDenied(Box::new(input.event))
                }
                SideEffectLifecycleState::Skipped => {
                    WorkflowRunEventKind::SideEffectSkipped(Box::new(input.event))
                }
                SideEffectLifecycleState::Attempted
                | SideEffectLifecycleState::Completed
                | SideEffectLifecycleState::Failed => {
                    return Err(executor_error(
                        WorkflowOsErrorKind::Unsupported,
                        "executor.side_effect_event.lifecycle.unsupported",
                        "executor side-effect event append currently supports proposed, denied, and skipped only",
                    ));
                }
            };
            self.append(
                &mut plan.event_builder,
                event_kind,
                Some(side_effect_event_idempotency_key(index, lifecycle)?),
            )?;
        }
        for (index, input) in plan
            .side_effect_lifecycle_events
            .clone()
            .into_iter()
            .enumerate()
        {
            if input.step_id != plan.step.id {
                continue;
            }
            validate_side_effect_lifecycle_event_input(plan, &input)?;
            let lifecycle = input.event.lifecycle_state();
            let event_kind = match lifecycle {
                SideEffectLifecycleState::Attempted => {
                    WorkflowRunEventKind::SideEffectAttempted(Box::new(input.event))
                }
                SideEffectLifecycleState::Completed => {
                    WorkflowRunEventKind::SideEffectCompleted(Box::new(input.event))
                }
                SideEffectLifecycleState::Failed => {
                    WorkflowRunEventKind::SideEffectFailed(Box::new(input.event))
                }
                SideEffectLifecycleState::Proposed
                | SideEffectLifecycleState::Denied
                | SideEffectLifecycleState::Skipped => {
                    return Err(executor_error(
                        WorkflowOsErrorKind::Validation,
                        "executor.side_effect_lifecycle_event.lifecycle.invalid",
                        "side-effect lifecycle append input requires attempted, completed, or failed lifecycle",
                    ));
                }
            };
            self.append(
                &mut plan.event_builder,
                event_kind,
                Some(side_effect_lifecycle_event_idempotency_key(
                    index, lifecycle,
                )?),
            )?;
        }
        Ok(())
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
    ) -> Result<StepExecutionResult, WorkflowOsError> {
        if let Err(error) = validate_required_outputs(&plan.skill, &output) {
            self.record_attempt_failure(&mut plan, &error, &attempt_key)?;
            let attempts = plan.retry_max_attempts;
            return self
                .exhaust_retries(plan, attempts, &error)
                .map(|run| StepExecutionResult::Terminal(Box::new(run)));
        }
        self.emit_adapter_telemetry(&plan, &output.adapter_telemetry)?;
        self.append(
            &mut plan.event_builder,
            WorkflowRunEventKind::SkillInvocationSucceeded {
                invocation_id: plan.invocation_id.clone(),
                step_id: plan.step.id.clone(),
                skill_id: plan.skill_id.clone(),
                skill_version: plan.skill_version.clone(),
                output_ref: output.output_ref,
            },
            Some(attempt_key),
        )?;
        Ok(StepExecutionResult::Succeeded(Box::new(plan)))
    }

    fn emit_adapter_telemetry(
        &self,
        plan: &ExecutionPlan,
        telemetry: &[AdapterTelemetryRecord],
    ) -> Result<(), WorkflowOsError> {
        for record in telemetry {
            let audit_record = AdapterRuntimeAuditRecord::from_invocation(
                &record.invocation,
                Some(plan.step.id.clone()),
                Some(plan.skill_id.clone()),
                Some(plan.skill_version.clone()),
                "workflow-core.local-executor",
            );
            self.backend.append_adapter_audit_record(&audit_record)?;
            self.audit_sink.record_adapter_audit_record(&audit_record)?;

            let observability_record = AdapterRuntimeObservabilityRecord::from_records(
                &record.invocation,
                &record.observability,
                Some(plan.step.id.clone()),
                Some(plan.skill_id.clone()),
                Some(plan.skill_version.clone()),
                "workflow-core.local-executor",
            );
            self.backend
                .append_adapter_observability_record(&observability_record)?;
            self.observability_sink
                .record_adapter_observability_record(&observability_record)?;
        }
        Ok(())
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
            policy_effects: PolicyEffectSet::default(),
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

    fn complete_run(&self, mut builder: EventBuilder) -> Result<WorkflowRun, WorkflowOsError> {
        self.append(&mut builder, WorkflowRunEventKind::RunCompleted, None)?;
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

fn approval_decision_proof_marker(
    presentation: &ApprovalPresentationRecord,
    decision: &ApprovalDecision,
    max_presentation_age: Option<Duration>,
) -> Result<ApprovalDecisionProofMarker, WorkflowOsError> {
    let proof_age_ms = max_presentation_age.and_then(|_| {
        proof_age_millis(presentation.presented_at(), decision.decided_at)
            .and_then(|age| u64::try_from(age).ok())
    });
    let proof_freshness_limit_ms =
        max_presentation_age.map(|age| u64::try_from(age.as_millis()).unwrap_or(u64::MAX));
    ApprovalDecisionProofMarker::new(ApprovalDecisionProofMarkerDefinition {
        enforcement_mode: ApprovalDecisionProofEnforcementMode::ApprovalPresentationRequired,
        presentation_id: presentation.presentation_id().clone(),
        presentation_content_hash: presentation.content_hash().clone(),
        proof_validated_at: decision.decided_at,
        proof_validation_policy:
            ApprovalDecisionProofValidationPolicy::ApprovalPresentationRequestMatch,
        proof_age_ms,
        proof_freshness_limit_ms,
        proof_record_sensitivity: presentation.sensitivity(),
        redaction: presentation.redaction().clone(),
    })
}

fn proof_age_millis(presented_at: Timestamp, decided_at: Timestamp) -> Option<i128> {
    let age = decided_at.as_offset_date_time() - presented_at.as_offset_date_time();
    age.is_positive().then(|| age.whole_milliseconds())
}

/// Executes a local workflow and explicitly opts into `SideEffect` discovery for
/// the in-memory terminal report result.
///
/// This helper is additive: it reuses `LocalExecutor::execute(...)`, keeps
/// `LocalExecutor::execute_with_report(...)` unchanged, and reads `SideEffect`
/// records only through the supplied `SideEffectRecordStore`.
///
/// # Errors
///
/// Returns the same structured errors as `execute(...)` when execution fails
/// before a workflow run exists.
pub fn execute_with_report_and_side_effect_discovery<B>(
    executor: &LocalExecutor<'_, B>,
    store: &impl SideEffectRecordStore,
    request: &LocalExecutionWithReportAndSideEffectDiscoveryRequest,
) -> Result<LocalExecutionWithReportResult, WorkflowOsError>
where
    B: StateBackend,
{
    let run = executor.execute(&request.execution)?;
    if !run.snapshot.status.is_terminal() {
        return Ok(LocalExecutionWithReportResult::new(
            run,
            None,
            Some(terminal_work_report_status_error()),
        ));
    }

    let mut report = request.report.clone();
    if let Err(error) = apply_before_report_hook_checkpoint(&run, &mut report) {
        return Ok(LocalExecutionWithReportResult::new(run, None, Some(error)));
    }

    let input = terminal_report_input_for_run(&run, &report);
    match generate_terminal_local_work_report_with_side_effect_discovery(
        store,
        input,
        request.side_effect_discovery.into(),
    ) {
        Ok(work_report) => Ok(LocalExecutionWithReportResult::new(
            run,
            Some(work_report),
            None,
        )),
        Err(error) => Ok(LocalExecutionWithReportResult::new(run, None, Some(error))),
    }
}

/// Executes a local workflow, derives an in-memory report, and explicitly writes
/// a governed local report artifact after side-effect integrity and approval
/// linkage gates pass.
///
/// This helper is additive. It preserves `LocalExecutor::execute(...)`,
/// `LocalExecutor::execute_with_report(...)`, and
/// `execute_with_report_and_side_effect_discovery(...)` behavior. Workflow
/// execution failures before a run still return `Err`. Report-generation and
/// artifact-write failures after a run exists are returned inside the result
/// without mutating the run, appending events, executing side effects, calling
/// providers, exposing CLI output, or changing workflow pass/fail semantics.
///
/// # Errors
///
/// Returns the same structured errors as `execute(...)` when execution fails
/// before a workflow run exists.
pub fn execute_with_report_artifact_and_side_effect_gates<B>(
    executor: &LocalExecutor<'_, B>,
    artifact_store: &impl WorkReportArtifactStore,
    side_effect_store: &impl SideEffectRecordStore,
    request: &LocalExecutionWithReportArtifactRequest,
) -> Result<LocalExecutionWithReportArtifactResult, WorkflowOsError>
where
    B: StateBackend,
{
    execute_with_report_artifact_gates(executor, artifact_store, side_effect_store, request, None)
}

/// Executes a local workflow, derives an in-memory report, and explicitly writes
/// a governed local report artifact after side-effect integrity, approval
/// linkage, high-assurance disclosure, optional provider-candidate validation,
/// and store-backed approval proof-marker gates pass.
///
/// This helper is additive and opt-in. It preserves
/// `execute_with_report_artifact_and_side_effect_gates(...)` when callers do
/// not supply proof-marker gate inputs. It does not discover hidden projection
/// stores, persist projection records, mutate workflow state, append events,
/// execute side effects, call providers, expose CLI output, add schemas, or
/// change workflow pass/fail semantics.
///
/// # Errors
///
/// Returns the same structured errors as `execute(...)` when execution fails
/// before a workflow run exists.
pub fn execute_with_report_artifact_and_proof_marker_gates<B>(
    executor: &LocalExecutor<'_, B>,
    artifact_store: &impl WorkReportArtifactStore,
    side_effect_store: &impl SideEffectRecordStore,
    proof_marker_gate: LocalExecutionReportArtifactProofMarkerGateInputs<'_>,
    request: &LocalExecutionWithReportArtifactRequest,
) -> Result<LocalExecutionWithReportArtifactResult, WorkflowOsError>
where
    B: StateBackend,
{
    execute_with_report_artifact_gates(
        executor,
        artifact_store,
        side_effect_store,
        request,
        Some(proof_marker_gate),
    )
}

/// Executes a local workflow, persists bounded approval proof-marker projections
/// from the terminal run into a caller-supplied store, derives an in-memory
/// report, and explicitly writes a governed local report artifact after the
/// existing artifact gates pass.
///
/// This helper is additive and opt-in. It preserves
/// `LocalExecutor::execute(...)`, `execute_with_report_artifact_and_side_effect_gates(...)`,
/// and `execute_with_report_artifact_and_proof_marker_gates(...)`. It does not
/// discover hidden projection stores, persist projections by default, append
/// workflow events, execute side effects, call providers, expose CLI output, add
/// schemas or examples, or change workflow pass/fail semantics.
///
/// # Errors
///
/// Returns the same structured errors as `execute(...)` when execution fails
/// before a workflow run exists.
pub fn execute_with_report_artifact_and_projected_proof_markers<B>(
    executor: &LocalExecutor<'_, B>,
    artifact_store: &impl WorkReportArtifactStore,
    side_effect_store: &impl SideEffectRecordStore,
    projection_inputs: LocalExecutionProjectedProofMarkerArtifactInputs<'_>,
    request: &LocalExecutionWithReportArtifactRequest,
) -> Result<LocalExecutionWithProjectedProofMarkerArtifactResult, WorkflowOsError>
where
    B: StateBackend,
{
    let (run, workflow_report_artifact_policies) = execute_for_report_artifact_path(
        executor,
        &request.execution,
        Some(projection_inputs.proof_marker_policy),
    )?;

    if !run.snapshot.status.is_terminal() {
        return Ok(projected_proof_marker_artifact_non_terminal_result(run));
    }

    let projection_persistence =
        match persist_projected_proof_marker_artifact_projections(&run, projection_inputs) {
            Ok(result) => result,
            Err(error) => return Ok(projected_proof_marker_artifact_projection_error(run, error)),
        };

    Ok(finish_projected_proof_marker_artifact_path(
        artifact_store,
        side_effect_store,
        ProjectedProofMarkerArtifactFinishInput {
            run,
            workflow_report_artifact_policies,
            projection_persistence,
            projection_inputs,
            report: &request.report,
            side_effect_discovery: request.side_effect_discovery,
            artifact_inputs: &request.artifact,
        },
    ))
}

/// Applies a proof-enforced local approval decision, persists bounded approval
/// proof-marker projections from the resumed terminal run into a caller-supplied
/// store, derives an in-memory report, and explicitly writes a governed local
/// report artifact after the existing artifact gates pass.
///
/// This helper is additive and opt-in. It preserves
/// `LocalExecutor::decide_approval(...)`,
/// `LocalExecutor::decide_approval_with_presentation(...)`, and existing
/// execution/report/artifact helpers. It does not discover hidden stores,
/// persist projections by default, append extra workflow events, mutate runtime
/// state beyond the approval decision itself, execute side effects, call
/// providers, expose CLI output, add schemas or examples, or change workflow
/// pass/fail semantics.
///
/// # Errors
///
/// Returns the same structured errors as
/// `LocalExecutor::decide_approval_with_presentation(...)` when approval
/// proof validation or the approval decision fails before a resumed run exists.
pub fn decide_approval_with_report_artifact_and_projected_proof_markers<B>(
    executor: &LocalExecutor<'_, B>,
    artifact_store: &impl WorkReportArtifactStore,
    side_effect_store: &impl SideEffectRecordStore,
    request: LocalApprovalResumeWithProjectedProofMarkerArtifactRequest<'_>,
) -> Result<LocalExecutionWithProjectedProofMarkerArtifactResult, WorkflowOsError>
where
    B: StateBackend,
{
    let LocalApprovalResumeWithProjectedProofMarkerArtifactRequest {
        projection,
        approval,
        report,
        side_effect_discovery,
        artifact,
    } = request;
    let project_root = approval.approval.project_root.clone();
    let run_id = approval.approval.run_id.clone();
    let correlation_id = approval.approval.correlation_id.clone();
    let actor = approval.approval.actor.clone();
    let run = executor.decide_approval_with_presentation(approval)?;
    let policy_request = LocalExecutionRequest {
        project_root,
        workflow_id: run.snapshot.identity.workflow_id.clone(),
        run_id: Some(run_id),
        correlation_id,
        actor,
        before_skill_invocation_checkpoints:
            LocalExecutionBeforeSkillInvocationCheckpointInputs::default(),
        before_skill_invocation_hook: None,
        side_effect_events: Vec::new(),
        side_effect_lifecycle_events: Vec::new(),
    };
    let workflow_report_artifact_policies =
        workflow_report_artifact_policy_for_request_with_proof_marker_policy(
            &policy_request,
            &run.snapshot.identity,
            Some(projection.proof_marker_policy),
        )?;

    if !run.snapshot.status.is_terminal() {
        return Ok(projected_proof_marker_artifact_non_terminal_result(run));
    }

    let projection_persistence =
        match persist_projected_proof_marker_artifact_projections(&run, projection) {
            Ok(result) => result,
            Err(error) => return Ok(projected_proof_marker_artifact_projection_error(run, error)),
        };

    Ok(finish_projected_proof_marker_artifact_path(
        artifact_store,
        side_effect_store,
        ProjectedProofMarkerArtifactFinishInput {
            run,
            workflow_report_artifact_policies,
            projection_persistence,
            projection_inputs: projection,
            report,
            side_effect_discovery,
            artifact_inputs: artifact,
        },
    ))
}

/// Applies a high-assurance local approval decision, persists bounded approval
/// proof-marker projections from the resumed terminal run into a
/// caller-supplied store, derives an in-memory report with the report-safe
/// high-assurance disclosure, and explicitly writes a governed local report
/// artifact after the existing artifact gates pass.
///
/// This helper is additive and opt-in. It preserves
/// `LocalExecutor::decide_approval(...)`,
/// `LocalExecutor::decide_approval_with_high_assurance(...)`, and existing
/// execution/report/artifact helpers. It does not discover hidden stores,
/// persist projections by default, append extra workflow events, mutate runtime
/// state beyond the approval decision itself, execute side effects, call
/// providers, expose CLI output, add schemas or examples, or change workflow
/// pass/fail semantics.
///
/// # Errors
///
/// Returns the same structured errors as
/// `LocalExecutor::decide_approval_with_presentation(...)` and
/// `LocalExecutor::decide_approval_with_high_assurance_disclosure(...)` when
/// presentation proof validation, high-assurance validation, or the approval
/// decision fails before a resumed run exists.
pub fn decide_approval_with_high_assurance_report_artifact_and_projected_proof_markers<B>(
    executor: &LocalExecutor<'_, B>,
    artifact_store: &impl WorkReportArtifactStore,
    side_effect_store: &impl SideEffectRecordStore,
    request: LocalHighAssuranceApprovalResumeWithProjectedProofMarkerArtifactRequest<'_>,
) -> Result<LocalExecutionWithProjectedProofMarkerArtifactResult, WorkflowOsError>
where
    B: StateBackend,
{
    let LocalHighAssuranceApprovalResumeWithProjectedProofMarkerArtifactRequest {
        projection,
        approval,
        proof,
        max_presentation_age,
        report,
        side_effect_discovery,
        artifact,
    } = request;
    let LocalHighAssuranceApprovalDecisionRequest {
        approval: approval_request,
        controls,
        supplied_references,
        current_time,
    } = approval;
    let project_root = approval_request.project_root.clone();
    let run_id = approval_request.run_id.clone();
    let correlation_id = approval_request.correlation_id.clone();
    let actor = approval_request.actor.clone();
    let (prepared_run, approval_request_state, decision) =
        executor.prepare_approval_decision(&approval_request)?;
    let presentation =
        executor.resolve_approval_presentation_proof(&approval_request_state, &proof)?;
    validate_approval_presentation_enforcement(
        &presentation,
        &approval_request_state,
        &decision,
        max_presentation_age,
    )?;
    validate_high_assurance_approval_decision(&HighAssuranceApprovalDecisionValidationInput {
        approval_request: &approval_request_state,
        approval_decision: &decision,
        controls: &controls,
        supplied_references: &supplied_references,
        current_time,
    })?;
    let high_assurance_approval = high_assurance_disclosure_from_validated_controls(
        decision.decision,
        &controls,
        supplied_references.len(),
    )?;
    let proof_marker =
        approval_decision_proof_marker(&presentation, &decision, max_presentation_age)?;
    let decision = ApprovalDecision {
        proof_marker: Some(proof_marker),
        ..decision
    };
    let policy_request = LocalExecutionRequest {
        project_root: project_root.clone(),
        workflow_id: prepared_run.snapshot.identity.workflow_id.clone(),
        run_id: Some(run_id.clone()),
        correlation_id: correlation_id.clone(),
        actor: actor.clone(),
        before_skill_invocation_checkpoints:
            LocalExecutionBeforeSkillInvocationCheckpointInputs::default(),
        before_skill_invocation_hook: None,
        side_effect_events: Vec::new(),
        side_effect_lifecycle_events: Vec::new(),
    };
    let workflow_report_artifact_policies =
        workflow_report_artifact_policy_for_request_with_proof_marker_policy(
            &policy_request,
            &prepared_run.snapshot.identity,
            Some(projection.proof_marker_policy),
        )?;
    let run = executor.apply_approval_decision(
        &project_root,
        &correlation_id,
        &prepared_run,
        &approval_request_state,
        decision,
    )?;

    if !run.snapshot.status.is_terminal() {
        return Ok(projected_proof_marker_artifact_non_terminal_result(run));
    }

    let projection_persistence =
        match persist_projected_proof_marker_artifact_projections(&run, projection) {
            Ok(result) => result,
            Err(error) => return Ok(projected_proof_marker_artifact_projection_error(run, error)),
        };

    let mut report_with_high_assurance = report.clone();
    report_with_high_assurance.high_assurance_approval = Some(high_assurance_approval);

    Ok(finish_projected_proof_marker_artifact_path(
        artifact_store,
        side_effect_store,
        ProjectedProofMarkerArtifactFinishInput {
            run,
            workflow_report_artifact_policies,
            projection_persistence,
            projection_inputs: projection,
            report: &report_with_high_assurance,
            side_effect_discovery,
            artifact_inputs: artifact,
        },
    ))
}

fn projected_proof_marker_artifact_non_terminal_result(
    run: WorkflowRun,
) -> LocalExecutionWithProjectedProofMarkerArtifactResult {
    LocalExecutionWithProjectedProofMarkerArtifactResult::new(
        LocalExecutionWithReportArtifactResult::new(
            run,
            None,
            Some(terminal_work_report_status_error()),
            None,
            None,
            None,
            None,
            None,
        ),
        None,
        None,
    )
}

fn projected_proof_marker_artifact_projection_error(
    run: WorkflowRun,
    error: WorkflowOsError,
) -> LocalExecutionWithProjectedProofMarkerArtifactResult {
    LocalExecutionWithProjectedProofMarkerArtifactResult::new(
        LocalExecutionWithReportArtifactResult::new(run, None, None, None, None, None, None, None),
        None,
        Some(error),
    )
}

fn persist_projected_proof_marker_artifact_projections(
    run: &WorkflowRun,
    projection_inputs: LocalExecutionProjectedProofMarkerArtifactInputs<'_>,
) -> Result<ApprovalProofMarkerProjectionPersistenceResult, WorkflowOsError> {
    persist_approval_proof_marker_projections_for_run(
        ApprovalProofMarkerProjectionPersistenceInput {
            run,
            projection_store: projection_inputs.projection_store,
            policy: projection_inputs.projection_policy,
            selected_approval_reference_ids: projection_inputs.selected_approval_reference_ids,
            sensitivity: projection_inputs.projection_sensitivity,
            redaction: projection_inputs.projection_redaction.clone(),
        },
    )
}

fn finish_projected_proof_marker_artifact_path(
    artifact_store: &impl WorkReportArtifactStore,
    side_effect_store: &impl SideEffectRecordStore,
    input: ProjectedProofMarkerArtifactFinishInput<'_>,
) -> LocalExecutionWithProjectedProofMarkerArtifactResult {
    let ProjectedProofMarkerArtifactFinishInput {
        run,
        workflow_report_artifact_policies,
        projection_persistence,
        projection_inputs,
        report,
        side_effect_discovery,
        artifact_inputs,
    } = input;
    let work_report = match generate_work_report_for_artifact_path(
        &run,
        report,
        side_effect_discovery,
        side_effect_store,
    ) {
        Ok(work_report) => work_report,
        Err(error) => {
            return LocalExecutionWithProjectedProofMarkerArtifactResult::new(
                LocalExecutionWithReportArtifactResult::new(
                    run,
                    None,
                    Some(error),
                    None,
                    None,
                    None,
                    None,
                    None,
                ),
                Some(projection_persistence),
                None,
            );
        }
    };

    let artifact = match WorkReportArtifactRecord::new(work_report.clone()) {
        Ok(artifact) => artifact,
        Err(error) => {
            return LocalExecutionWithProjectedProofMarkerArtifactResult::new(
                LocalExecutionWithReportArtifactResult::new(
                    run,
                    Some(work_report),
                    None,
                    None,
                    Some(error),
                    None,
                    None,
                    None,
                ),
                Some(projection_persistence),
                None,
            );
        }
    };

    let provider_integration =
        report_artifact_provider_integration(artifact_inputs.provider_integration.as_ref());
    let high_assurance_disclosure_policy = artifact_inputs
        .high_assurance_disclosure_policy
        .stricter(workflow_report_artifact_policies.high_assurance_disclosure_policy);
    let artifact_write_input = LocalExecutorArtifactWriteGateInput {
        run: &run,
        artifact: &artifact,
        artifact_inputs,
        provider_integration,
        high_assurance_disclosure_policy,
        approval_proof_marker_policy: workflow_report_artifact_policies
            .approval_proof_marker_policy,
        proof_marker_gate: Some(LocalExecutionReportArtifactProofMarkerGateInputs {
            projection_store: projection_inputs.projection_store,
            policy: projection_inputs.proof_marker_policy,
        }),
    };

    let artifact_write_result =
        write_executor_report_artifact(artifact_store, side_effect_store, &artifact_write_input);

    projected_proof_marker_artifact_write_result(
        run,
        work_report,
        artifact,
        projection_persistence,
        artifact_write_result,
    )
}

fn projected_proof_marker_artifact_write_result(
    run: WorkflowRun,
    work_report: WorkReport,
    artifact: WorkReportArtifactRecord,
    projection_persistence: ApprovalProofMarkerProjectionPersistenceResult,
    artifact_write_result: Result<LocalExecutorArtifactWriteGateResult, WorkflowOsError>,
) -> LocalExecutionWithProjectedProofMarkerArtifactResult {
    let artifact_result = match artifact_write_result {
        Ok(gate_result) => LocalExecutionWithReportArtifactResult::new(
            run,
            Some(work_report),
            None,
            Some(artifact),
            None,
            Some(gate_result.side_effect_integrity),
            gate_result.approval_linkage,
            gate_result.high_assurance_disclosure,
        ),
        Err(error) => LocalExecutionWithReportArtifactResult::new(
            run,
            Some(work_report),
            None,
            None,
            Some(error),
            None,
            None,
            None,
        ),
    };
    LocalExecutionWithProjectedProofMarkerArtifactResult::new(
        artifact_result,
        Some(projection_persistence),
        None,
    )
}

fn execute_with_report_artifact_gates<B>(
    executor: &LocalExecutor<'_, B>,
    artifact_store: &impl WorkReportArtifactStore,
    side_effect_store: &impl SideEffectRecordStore,
    request: &LocalExecutionWithReportArtifactRequest,
    proof_marker_gate: Option<LocalExecutionReportArtifactProofMarkerGateInputs<'_>>,
) -> Result<LocalExecutionWithReportArtifactResult, WorkflowOsError>
where
    B: StateBackend,
{
    let (run, workflow_report_artifact_policies) = execute_for_report_artifact_path(
        executor,
        &request.execution,
        proof_marker_gate.map(|gate| gate.policy),
    )?;

    if !run.snapshot.status.is_terminal() {
        return Ok(LocalExecutionWithReportArtifactResult::new(
            run,
            None,
            Some(terminal_work_report_status_error()),
            None,
            None,
            None,
            None,
            None,
        ));
    }

    let work_report = match generate_work_report_for_artifact_path(
        &run,
        &request.report,
        request.side_effect_discovery,
        side_effect_store,
    ) {
        Ok(work_report) => work_report,
        Err(error) => {
            return Ok(LocalExecutionWithReportArtifactResult::new(
                run,
                None,
                Some(error),
                None,
                None,
                None,
                None,
                None,
            ));
        }
    };

    let artifact = match WorkReportArtifactRecord::new(work_report.clone()) {
        Ok(artifact) => artifact,
        Err(error) => {
            return Ok(LocalExecutionWithReportArtifactResult::new(
                run,
                Some(work_report),
                None,
                None,
                Some(error),
                None,
                None,
                None,
            ));
        }
    };

    let provider_integration =
        report_artifact_provider_integration(request.artifact.provider_integration.as_ref());
    let high_assurance_disclosure_policy = request
        .artifact
        .high_assurance_disclosure_policy
        .stricter(workflow_report_artifact_policies.high_assurance_disclosure_policy);

    let artifact_write_input = LocalExecutorArtifactWriteGateInput {
        run: &run,
        artifact: &artifact,
        artifact_inputs: &request.artifact,
        provider_integration,
        high_assurance_disclosure_policy,
        approval_proof_marker_policy: workflow_report_artifact_policies
            .approval_proof_marker_policy,
        proof_marker_gate,
    };
    match write_executor_report_artifact(artifact_store, side_effect_store, &artifact_write_input) {
        Ok(gate_result) => Ok(LocalExecutionWithReportArtifactResult::new(
            run,
            Some(work_report),
            None,
            Some(artifact),
            None,
            Some(gate_result.side_effect_integrity),
            gate_result.approval_linkage,
            gate_result.high_assurance_disclosure,
        )),
        Err(error) => Ok(LocalExecutionWithReportArtifactResult::new(
            run,
            Some(work_report),
            None,
            None,
            Some(error),
            None,
            None,
            None,
        )),
    }
}

fn write_executor_report_artifact(
    artifact_store: &impl WorkReportArtifactStore,
    side_effect_store: &impl SideEffectRecordStore,
    input: &LocalExecutorArtifactWriteGateInput<'_>,
) -> Result<LocalExecutorArtifactWriteGateResult, WorkflowOsError> {
    if let Some(proof_marker_gate) = input.proof_marker_gate {
        let write_result = write_work_report_artifact_with_governance_gates(
            artifact_store,
            side_effect_store,
            WorkReportArtifactProofMarkerGovernedWriteInput {
                governed_write: WorkReportArtifactGovernedWriteInput {
                    run: input.run,
                    artifact: input.artifact,
                    require_all_side_effect_citations: input
                        .artifact_inputs
                        .require_all_side_effect_citations,
                    require_approval_references_for_requires_approval: input
                        .artifact_inputs
                        .require_approval_references_for_requires_approval,
                    require_decision_for_approved_or_denied: input
                        .artifact_inputs
                        .require_decision_for_approved_or_denied,
                    high_assurance_disclosure_policy: input.high_assurance_disclosure_policy,
                },
                provider_integration: input.provider_integration,
                approval_proof_marker_projection_store: proof_marker_gate.projection_store,
                approval_proof_marker_policy: input
                    .approval_proof_marker_policy
                    .unwrap_or(proof_marker_gate.policy),
            },
        )?;
        return Ok(LocalExecutorArtifactWriteGateResult {
            side_effect_integrity: *write_result.side_effect_integrity(),
            approval_linkage: write_result.approval_linkage().copied(),
            high_assurance_disclosure: write_result.high_assurance_disclosure().copied(),
        });
    }

    let write_result = write_report_artifact_with_explicit_integrations(
        artifact_store,
        side_effect_store,
        ReportArtifactWriteIntegrationInput {
            run: input.run,
            artifact: input.artifact,
            require_all_side_effect_citations: input
                .artifact_inputs
                .require_all_side_effect_citations,
            require_approval_references_for_requires_approval: input
                .artifact_inputs
                .require_approval_references_for_requires_approval,
            require_decision_for_approved_or_denied: input
                .artifact_inputs
                .require_decision_for_approved_or_denied,
            high_assurance_disclosure_policy: input.high_assurance_disclosure_policy,
            provider_integration: input.provider_integration,
        },
    )?;
    Ok(LocalExecutorArtifactWriteGateResult {
        side_effect_integrity: *write_result.artifact_write().side_effect_integrity(),
        approval_linkage: write_result.artifact_write().approval_linkage().copied(),
        high_assurance_disclosure: write_result
            .artifact_write()
            .high_assurance_disclosure()
            .copied(),
    })
}

/// Executes a local workflow and explicitly opts into one injected GitHub PR
/// comment provider write after a run exists.
///
/// This helper is additive. It preserves `LocalExecutor::execute(...)`, does
/// not make provider writes automatic, and calls only the supplied provider
/// trait after the existing provider-call/store gates validate. It does not
/// append workflow events, emit audit/observability records, write report
/// artifacts, persist reports, expose CLI output, add schemas/examples, load
/// hidden auth, retry automatically, or broaden write support.
///
/// # Errors
///
/// Returns the same structured errors as `execute(...)` when execution fails
/// before a workflow run exists.
pub fn execute_with_github_pr_comment_provider_write<B, P>(
    executor: &LocalExecutor<'_, B>,
    store: &impl SideEffectRecordStore,
    provider: &P,
    request: &LocalExecutionWithGitHubPrCommentProviderWriteRequest<'_>,
) -> Result<LocalExecutionWithGitHubPrCommentProviderWriteResult, WorkflowOsError>
where
    B: StateBackend,
    P: GitHubPullRequestCommentProvider,
{
    let run = executor.execute(&request.execution)?;
    if !run.snapshot.status.is_terminal() {
        let error = executor_error(
            WorkflowOsErrorKind::Validation,
            "executor_github_pr_comment_write.status.not_terminal",
            "GitHub PR comment provider write requires a terminal local workflow run",
        );
        return Ok(provider_write_result_for_nonterminal_run(
            run,
            &request.provider_write,
            error,
        ));
    }

    match orchestrate_github_pr_comment_provider_call(
        store,
        provider,
        request.provider_write.provider_call.clone(),
    ) {
        Ok(result) => Ok(provider_write_result_for_successful_orchestration(
            executor,
            run,
            request,
            result.into_parts(),
        )),
        Err(orchestration_error) => Ok(provider_write_result_for_orchestration_error(
            run,
            &request.provider_write,
            orchestration_error,
        )),
    }
}

/// Executes a local workflow and explicitly opts into one injected GitHub PR
/// comment provider write only after approval-presentation proof is validated.
///
/// This helper is additive. It preserves default executor behavior and the
/// existing explicit provider-write helper. When proof is required and invalid,
/// it returns an in-memory provider-write result with no provider call.
///
/// # Errors
///
/// Returns the same structured errors as `execute(...)` when execution fails
/// before a workflow run exists.
pub fn execute_with_github_pr_comment_provider_write_presentation_gate<B, P>(
    executor: &LocalExecutor<'_, B>,
    store: &impl SideEffectRecordStore,
    provider: &P,
    request: &LocalExecutionWithGitHubPrCommentProviderWritePresentationGateRequest<'_>,
) -> Result<LocalExecutionWithGitHubPrCommentProviderWriteResult, WorkflowOsError>
where
    B: StateBackend,
    P: GitHubPullRequestCommentProvider,
{
    let run = executor.execute(&request.request.execution)?;
    if !run.snapshot.status.is_terminal() {
        let error = executor_error(
            WorkflowOsErrorKind::Validation,
            "executor_github_pr_comment_write.status.not_terminal",
            "GitHub PR comment provider write requires a terminal local workflow run",
        );
        return Ok(provider_write_result_for_nonterminal_run(
            run,
            &request.request.provider_write,
            error,
        ));
    }

    match provider_write_approval_presentation_gate(
        executor,
        &run,
        &request.request.provider_write,
        &request.presentation_policy,
    ) {
        Ok(gate_state) => {
            let mut result = match orchestrate_github_pr_comment_provider_call(
                store,
                provider,
                request.request.provider_write.provider_call.clone(),
            ) {
                Ok(result) => provider_write_result_for_successful_orchestration(
                    executor,
                    run,
                    &request.request,
                    result.into_parts(),
                ),
                Err(orchestration_error) => provider_write_result_for_orchestration_error(
                    run,
                    &request.request.provider_write,
                    orchestration_error,
                ),
            };
            result.gate_clarity.approval_presentation = gate_state;
            Ok(result)
        }
        Err(error) => Ok(provider_write_result_for_presentation_gate_error(
            run,
            &request.request.provider_write,
            error,
        )),
    }
}

/// Composes the explicit local GitHub PR comment provider-write runtime path
/// using existing reviewed executor, approval-presentation, provider-call,
/// reconciliation, workflow-event proof, and report-disclosure helpers.
///
/// This helper is additive and opt-in. It does not change
/// `LocalExecutor::execute(...)`, does not make writes automatic, does not load
/// hidden auth or runtime config, does not write report artifacts, does not
/// expose CLI output, does not perform lookup/recovery, and does not broaden
/// provider support.
///
/// # Errors
///
/// Returns the same structured errors as `execute(...)` when execution fails
/// before a workflow run exists.
pub fn compose_github_pr_comment_provider_write_runtime<B, P>(
    executor: &LocalExecutor<'_, B>,
    store: &impl SideEffectRecordStore,
    provider: &P,
    request: &GitHubPrCommentProviderWriteRuntimeCompositionRequest<'_>,
) -> Result<GitHubPrCommentProviderWriteRuntimeCompositionResult, WorkflowOsError>
where
    B: StateBackend,
    P: GitHubPullRequestCommentProvider,
{
    let provider_write = execute_with_github_pr_comment_provider_write_presentation_gate(
        executor,
        store,
        provider,
        &request.provider_write,
    )?;
    Ok(GitHubPrCommentProviderWriteRuntimeCompositionResult::new(
        provider_write,
    ))
}

/// Composes the accepted GitHub PR comment live-sandbox validation helper into
/// an explicit runtime composition helper.
///
/// This helper is additive and opt-in. It delegates all proof/readiness/provider
/// call behavior to `validate_and_orchestrate_github_pr_comment_live_sandbox`.
/// It does not execute `LocalExecutor`, make provider writes automatic, load
/// hidden auth or runtime config, append workflow events, write report
/// artifacts, expose CLI output, perform lookup/recovery, broaden provider
/// support, or change release posture.
///
/// # Errors
///
/// Returns the same structured, non-leaking orchestration error as the accepted
/// live-sandbox validation helper when any gate fails or the provider-call
/// orchestration fails.
pub fn compose_github_pr_comment_live_sandbox_runtime<P>(
    store: &impl SideEffectRecordStore,
    provider: &P,
    request: GitHubPrCommentLiveSandboxRuntimeCompositionRequest<'_>,
) -> Result<
    GitHubPrCommentLiveSandboxRuntimeCompositionResult,
    GitHubPullRequestCommentProviderCallOrchestrationError,
>
where
    P: GitHubPullRequestCommentProvider,
{
    let live_sandbox = validate_and_orchestrate_github_pr_comment_live_sandbox(
        store,
        provider,
        request.live_sandbox,
    )?;
    Ok(GitHubPrCommentLiveSandboxRuntimeCompositionResult::new(
        live_sandbox,
    ))
}

/// Composes one GitHub PR comment live-sandbox call only after deriving its
/// approval posture from proof-enforced approval and store-backed linkage.
///
/// This helper is additive and opt-in. It does not trust the caller-supplied
/// sandbox approval posture as authority. It validates the attempted record's
/// stable approval reference against the supplied terminal run, validates the
/// corresponding approval-presentation proof, validates the persisted
/// `SideEffect` linkage, and only then derives `LinkedAndApproved` for the
/// accepted live-sandbox helper. It does not load auth, append events, write
/// artifacts, retry, repair, or broaden provider support.
///
/// # Errors
///
/// Returns a stable, non-leaking error before the provider is called when run
/// identity, approval presentation, approval decision, or persisted linkage is
/// missing, stale, denied, or inconsistent. Provider orchestration errors are
/// returned with their existing stable code and bounded message.
pub fn compose_github_pr_comment_live_sandbox_runtime_with_approval_authority<B, P>(
    executor: &LocalExecutor<'_, B>,
    store: &impl SideEffectRecordStore,
    provider: &P,
    request: GitHubPrCommentLiveSandboxApprovalAuthorityCompositionRequest<'_>,
) -> Result<GitHubPrCommentLiveSandboxApprovalAuthorityCompositionResult, WorkflowOsError>
where
    B: StateBackend,
    P: GitHubPullRequestCommentProvider,
{
    if request.presentation_policy.mode == ApprovalPresentationDefaultEnforcementMode::NotRequired {
        return Err(executor_error(
            WorkflowOsErrorKind::Validation,
            "github_pr_comment_live_sandbox.approval_authority.presentation_policy_required",
            "live sandbox approval authority requires proof-enforced presentation policy",
        ));
    }
    let durable_run = executor
        .backend
        .rehydrate_run(&request.run.snapshot.identity.run_id)
        .map_err(|_| {
            executor_error(
                WorkflowOsErrorKind::InvalidState,
                "github_pr_comment_live_sandbox.approval_authority.durable_run_unavailable",
                "live sandbox approval authority requires durable workflow run state",
            )
        })?;
    if &durable_run != request.run {
        return Err(executor_error(
            WorkflowOsErrorKind::InvalidState,
            "github_pr_comment_live_sandbox.approval_authority.durable_run_mismatch",
            "live sandbox approval authority requires caller run to match durable state",
        ));
    }
    let attempted_record = request
        .live_sandbox
        .live_sandbox
        .provider_call
        .attempted_record;
    validate_live_sandbox_approval_authority_run(&durable_run, attempted_record)?;
    provider_write_approval_presentation_gate_for_record(
        executor,
        &durable_run,
        attempted_record,
        &request.presentation_policy,
    )?;

    let side_effect_ids = [attempted_record.side_effect_id().clone()];
    let approval_linkage = crate::validate_side_effect_approval_linkage_from_store(
        store,
        SideEffectApprovalLinkageFromStoreInput {
            run: &durable_run,
            side_effect_ids: &side_effect_ids,
            load_mode: SideEffectApprovalLinkageStoreLoadMode::ExplicitIds,
            missing_record_policy: SideEffectMissingRecordPolicy::RequireAll,
            require_approval_references_for_requires_approval: true,
            require_decision_for_approved_or_denied: true,
        },
    )?;

    let GitHubPrCommentLiveSandboxRuntimeCompositionRequest {
        live_sandbox:
            GitHubPullRequestCommentLiveSandboxValidationInput {
                target_proof,
                mut readiness,
                provider_call,
                transitioned_at,
                transition_references,
                evidence_reference_count,
            },
    } = request.live_sandbox;
    readiness.approval_required = true;
    readiness.approval_posture = crate::ProviderWriteSandboxApprovalPosture::LinkedAndApproved;

    let live_sandbox = compose_github_pr_comment_live_sandbox_runtime(
        store,
        provider,
        GitHubPrCommentLiveSandboxRuntimeCompositionRequest {
            live_sandbox: GitHubPullRequestCommentLiveSandboxValidationInput {
                target_proof,
                readiness,
                provider_call,
                transitioned_at,
                transition_references,
                evidence_reference_count,
            },
        },
    )
    .map_err(|error| executor_error(error.error().kind(), error.code(), error.error().message()))?;

    Ok(
        GitHubPrCommentLiveSandboxApprovalAuthorityCompositionResult::new(
            live_sandbox,
            approval_linkage,
        ),
    )
}

/// Composes an already-accepted GitHub PR comment live sandbox runtime result
/// into durable workflow event proof.
///
/// This helper is additive and opt-in. It does not call the provider, execute
/// `LocalExecutor`, make writes automatic, load hidden auth or runtime config,
/// write report artifacts, expose CLI output, perform lookup/recovery,
/// retry/repair, broaden provider support, or change release posture.
#[must_use]
pub fn compose_github_pr_comment_live_sandbox_event_proof<B>(
    executor: &LocalExecutor<'_, B>,
    request: GitHubPrCommentLiveSandboxEventProofCompositionRequest<'_>,
) -> GitHubPrCommentLiveSandboxEventProofCompositionResult
where
    B: StateBackend,
{
    if matches!(
        request.append_policy,
        GitHubPrCommentLiveSandboxEventProofAppendPolicy::Disabled
    ) {
        return GitHubPrCommentLiveSandboxEventProofCompositionResult::new(
            request.live_sandbox,
            request.run.clone(),
            GitHubPrCommentLiveSandboxEventProofStatus::NotRequested,
            None,
        );
    }

    let current_run = match executor.rehydrate_and_project(&request.run.snapshot.identity.run_id) {
        Ok(run) => run,
        Err(error) => {
            return GitHubPrCommentLiveSandboxEventProofCompositionResult::new(
                request.live_sandbox,
                request.run.clone(),
                GitHubPrCommentLiveSandboxEventProofStatus::Failed,
                Some(error),
            );
        }
    };

    if !matches!(
        current_run.snapshot.status,
        WorkflowRunStatus::Completed | WorkflowRunStatus::Failed | WorkflowRunStatus::Canceled
    ) {
        return GitHubPrCommentLiveSandboxEventProofCompositionResult::new(
            request.live_sandbox,
            current_run,
            GitHubPrCommentLiveSandboxEventProofStatus::Blocked,
            Some(github_pr_comment_live_sandbox_event_proof_error(
                "status.not_terminal",
                "live sandbox event-proof composition requires a terminal workflow run",
            )),
        );
    }

    let event_kind = match live_sandbox_event_proof_event_kind(
        &current_run,
        request.live_sandbox.provider_call(),
        &request.correlation_id,
    ) {
        Ok(event_kind) => event_kind,
        Err(error) => {
            let status = if error.code().ends_with("identity_mismatch") {
                GitHubPrCommentLiveSandboxEventProofStatus::Conflict
            } else {
                GitHubPrCommentLiveSandboxEventProofStatus::NotEligible
            };
            return GitHubPrCommentLiveSandboxEventProofCompositionResult::new(
                request.live_sandbox,
                current_run,
                status,
                Some(error),
            );
        }
    };
    let idempotency_key =
        match live_sandbox_event_proof_idempotency_key(request.live_sandbox.provider_call()) {
            Ok(idempotency_key) => idempotency_key,
            Err(error) => {
                return GitHubPrCommentLiveSandboxEventProofCompositionResult::new(
                    request.live_sandbox,
                    current_run,
                    GitHubPrCommentLiveSandboxEventProofStatus::NotEligible,
                    Some(error),
                );
            }
        };

    match live_sandbox_event_proof_existing_status(&current_run, &event_kind, &idempotency_key) {
        Ok(Some(status)) => {
            return GitHubPrCommentLiveSandboxEventProofCompositionResult::new(
                request.live_sandbox,
                current_run,
                status,
                None,
            );
        }
        Ok(None) => {}
        Err(error) => {
            return GitHubPrCommentLiveSandboxEventProofCompositionResult::new(
                request.live_sandbox,
                current_run,
                GitHubPrCommentLiveSandboxEventProofStatus::Conflict,
                Some(error),
            );
        }
    }

    append_live_sandbox_event_proof(
        executor,
        request.live_sandbox,
        current_run,
        event_kind,
        idempotency_key,
        request.correlation_id,
        request.actor,
    )
}

/// Composes the explicit local GitHub PR comment provider-write runtime path
/// with explicit report artifact governance gates.
///
/// This helper is additive and opt-in. It delegates provider-write behavior to
/// `compose_github_pr_comment_provider_write_runtime`, then writes the supplied
/// report artifact only if artifact gates pass. It does not change
/// `LocalExecutor::execute(...)`, make writes automatic, load hidden auth or
/// stores, infer runtime config, expose CLI output, perform lookup/recovery,
/// broaden provider support, or change release posture.
///
/// # Errors
///
/// Returns the same structured errors as provider-write composition when
/// execution fails before a workflow run exists. Artifact gate/write failures
/// after a run exists are returned inside
/// `GitHubPrCommentProviderWriteArtifactGatedCompositionResult`.
pub fn compose_github_pr_comment_provider_write_with_artifact_gates<B, P>(
    executor: &LocalExecutor<'_, B>,
    side_effect_store: &impl SideEffectRecordStore,
    artifact_store: &impl WorkReportArtifactStore,
    provider: &P,
    request: &GitHubPrCommentProviderWriteArtifactGatedCompositionRequest<'_>,
) -> Result<GitHubPrCommentProviderWriteArtifactGatedCompositionResult, WorkflowOsError>
where
    B: StateBackend,
    P: GitHubPullRequestCommentProvider,
{
    let provider_write = compose_github_pr_comment_provider_write_runtime(
        executor,
        side_effect_store,
        provider,
        &request.provider_write,
    )?;

    if !provider_write.provider_call_performed()
        || provider_write.provider_write_error().is_some()
        || provider_write
            .provider_write()
            .provider_response()
            .is_none()
        || provider_write
            .provider_write()
            .outcome_transition()
            .is_none()
        || provider_write
            .provider_write()
            .reconciliation_candidate()
            .is_none()
    {
        return Ok(
            GitHubPrCommentProviderWriteArtifactGatedCompositionResult::new(
                provider_write,
                None,
                Some(github_pr_comment_provider_write_artifact_composition_error(
                    "not_eligible",
                )),
            ),
        );
    }

    let provider_disclosures = [provider_write.report_disclosure().clone()];
    validate_provider_write_artifact_side_effect_citation(
        request.artifact,
        request.side_effect_id,
        request.citation_policy,
    )?;
    let provider_event_proof_gate =
        validate_github_pr_comment_provider_report_artifact_event_proof_gate(
            &provider_disclosures,
            request.provider_event_proof_gate_policy,
        );
    if let Err(error) = provider_event_proof_gate {
        return Ok(
            GitHubPrCommentProviderWriteArtifactGatedCompositionResult::new(
                provider_write,
                None,
                Some(error),
            ),
        );
    }
    let artifact_write = write_work_report_artifact_with_governance_gates(
        artifact_store,
        side_effect_store,
        WorkReportArtifactProofMarkerGovernedWriteInput {
            governed_write: WorkReportArtifactGovernedWriteInput {
                run: provider_write.run(),
                artifact: request.artifact,
                require_all_side_effect_citations: request.require_all_side_effect_citations,
                require_approval_references_for_requires_approval: request
                    .require_approval_references_for_requires_approval,
                require_decision_for_approved_or_denied: request
                    .require_decision_for_approved_or_denied,
                high_assurance_disclosure_policy: request.high_assurance_disclosure_policy,
            },
            provider_integration: ReportArtifactWriteProviderIntegration::None,
            approval_proof_marker_projection_store: request.approval_proof_marker_projection_store,
            approval_proof_marker_policy: request.approval_proof_marker_policy,
        },
    );

    match artifact_write {
        Ok(artifact_write) => Ok(
            GitHubPrCommentProviderWriteArtifactGatedCompositionResult::new(
                provider_write,
                Some(artifact_write),
                None,
            ),
        ),
        Err(error) => Ok(
            GitHubPrCommentProviderWriteArtifactGatedCompositionResult::new(
                provider_write,
                None,
                Some(error),
            ),
        ),
    }
}

fn validate_provider_write_artifact_side_effect_citation(
    artifact: &WorkReportArtifactRecord,
    side_effect_id: &SideEffectId,
    citation_policy: GitHubPullRequestCommentReportArtifactCitationPolicy,
) -> Result<(), WorkflowOsError> {
    artifact.validate().map_err(|_| {
        github_pr_comment_provider_write_artifact_composition_error("citation_invalid")
    })?;

    let cited = artifact
        .work_report()
        .sections()
        .iter()
        .flat_map(crate::WorkReportSection::citations)
        .any(|citation| {
            matches!(
                citation.target(),
                WorkReportCitationTarget::SideEffect { side_effect_id: cited }
                    if cited == side_effect_id
            )
        });

    if citation_policy.require_record && !cited {
        return Err(github_pr_comment_provider_write_artifact_composition_error(
            "side_effect_missing",
        ));
    }

    Ok(())
}

fn github_pr_comment_provider_write_artifact_composition_error(
    reason: &'static str,
) -> WorkflowOsError {
    WorkflowOsError::validation(
        format!("github_pr_comment_provider_write_artifact_composition.{reason}"),
        "provider-write artifact-gated composition could not write an artifact",
    )
}

fn provider_write_result_for_nonterminal_run(
    run: WorkflowRun,
    inputs: &LocalExecutionGitHubPrCommentProviderWriteInputs<'_>,
    error: WorkflowOsError,
) -> LocalExecutionWithGitHubPrCommentProviderWriteResult {
    let reconciliation = reconcile_provider_write_after_error(inputs, None, false, &error);
    let gate_clarity = provider_write_gate_clarity_from_explicit_inputs(
        inputs,
        None,
        None,
        reconciliation.as_ref(),
        Some(&error),
        false,
        false,
    );
    LocalExecutionWithGitHubPrCommentProviderWriteResult::new_with_gate_clarity(
        run,
        None,
        None,
        reconciliation,
        Some(error),
        false,
        gate_clarity,
    )
}

fn provider_write_result_for_presentation_gate_error(
    run: WorkflowRun,
    inputs: &LocalExecutionGitHubPrCommentProviderWriteInputs<'_>,
    error: WorkflowOsError,
) -> LocalExecutionWithGitHubPrCommentProviderWriteResult {
    let reconciliation = reconcile_provider_write_after_error(inputs, None, false, &error);
    let mut gate_clarity = provider_write_gate_clarity_from_explicit_inputs(
        inputs,
        None,
        None,
        reconciliation.as_ref(),
        Some(&error),
        false,
        false,
    );
    gate_clarity.approval_presentation = GitHubPullRequestCommentProviderWriteGateState::Blocked;
    LocalExecutionWithGitHubPrCommentProviderWriteResult::new_with_gate_clarity(
        run,
        None,
        None,
        reconciliation,
        Some(error),
        false,
        gate_clarity,
    )
}

fn provider_write_result_for_successful_orchestration<B>(
    executor: &LocalExecutor<'_, B>,
    mut run: WorkflowRun,
    request: &LocalExecutionWithGitHubPrCommentProviderWriteRequest<'_>,
    orchestration_parts: (
        GitHubPullRequestCommentWriteResponse,
        SideEffectLifecycleTransitionResult,
    ),
) -> LocalExecutionWithGitHubPrCommentProviderWriteResult
where
    B: StateBackend,
{
    let (provider_response, outcome_transition) = orchestration_parts;
    let reconciliation_result = reconcile_github_pr_comment_provider_write(
        GitHubPullRequestCommentProviderWriteReconciliationInput {
            attempted_record: request
                .provider_write
                .provider_call
                .provider_call
                .attempted_record,
            provider_response: Some(&provider_response),
            local_transition: Some(&outcome_transition),
            provider_call_attempted: true,
            local_transition_error_code: None,
            ambiguity_error_code: None,
            sensitivity: request.provider_write.reconciliation_sensitivity,
            redaction: request.provider_write.reconciliation_redaction.clone(),
        },
    );
    let (reconciliation, mut provider_write_error) = match reconciliation_result {
        Ok(candidate) => (Some(candidate), None),
        Err(error) => (None, Some(error)),
    };
    let mut workflow_event_appended = false;
    if let (Some(candidate), None) = (&reconciliation, &provider_write_error) {
        match append_reconciled_github_pr_comment_provider_write_event(
            executor,
            &run,
            &outcome_transition,
            candidate,
            &request.execution,
        ) {
            Ok(Some(updated_run)) => {
                run = updated_run;
                workflow_event_appended = true;
            }
            Ok(None) => {}
            Err(error) => {
                provider_write_error = Some(error);
            }
        }
    }
    let gate_clarity = provider_write_gate_clarity_from_explicit_inputs(
        &request.provider_write,
        Some(&provider_response),
        Some(&outcome_transition),
        reconciliation.as_ref(),
        provider_write_error.as_ref(),
        workflow_event_appended,
        true,
    );
    LocalExecutionWithGitHubPrCommentProviderWriteResult::new_with_gate_clarity(
        run,
        Some(provider_response),
        Some(outcome_transition),
        reconciliation,
        provider_write_error,
        workflow_event_appended,
        gate_clarity,
    )
}

fn provider_write_result_for_orchestration_error(
    run: WorkflowRun,
    inputs: &LocalExecutionGitHubPrCommentProviderWriteInputs<'_>,
    orchestration_error: GitHubPullRequestCommentProviderCallOrchestrationError,
) -> LocalExecutionWithGitHubPrCommentProviderWriteResult {
    let provider_call_attempted = orchestration_error.provider_call_attempted();
    let (error, provider_response) = orchestration_error.into_parts();
    let reconciliation = reconcile_provider_write_after_error(
        inputs,
        provider_response.as_ref(),
        provider_call_attempted,
        &error,
    );
    let gate_clarity = provider_write_gate_clarity_from_explicit_inputs(
        inputs,
        provider_response.as_ref(),
        None,
        reconciliation.as_ref(),
        Some(&error),
        false,
        provider_call_attempted,
    );
    LocalExecutionWithGitHubPrCommentProviderWriteResult::new_with_gate_clarity(
        run,
        provider_response,
        None,
        reconciliation,
        Some(error),
        false,
        gate_clarity,
    )
}

fn append_reconciled_github_pr_comment_provider_write_event<B>(
    executor: &LocalExecutor<'_, B>,
    run: &WorkflowRun,
    transition: &SideEffectLifecycleTransitionResult,
    reconciliation: &GitHubPullRequestCommentProviderWriteReconciliationCandidate,
    request: &LocalExecutionRequest,
) -> Result<Option<WorkflowRun>, WorkflowOsError>
where
    B: StateBackend,
{
    let expected_lifecycle = match reconciliation.status() {
        GitHubPullRequestCommentProviderWriteReconciliationStatus::ProviderSucceededLocalCompleted => {
            SideEffectLifecycleState::Completed
        }
        GitHubPullRequestCommentProviderWriteReconciliationStatus::ProviderFailedLocalFailed => {
            SideEffectLifecycleState::Failed
        }
        _ => return Ok(None),
    };
    validate_provider_write_event_append_identity(
        run,
        transition,
        reconciliation,
        expected_lifecycle,
    )?;

    let event_payload = transition.event().clone();
    let event_kind = match expected_lifecycle {
        SideEffectLifecycleState::Completed => {
            WorkflowRunEventKind::SideEffectCompleted(Box::new(event_payload))
        }
        SideEffectLifecycleState::Failed => {
            WorkflowRunEventKind::SideEffectFailed(Box::new(event_payload))
        }
        SideEffectLifecycleState::Proposed
        | SideEffectLifecycleState::Attempted
        | SideEffectLifecycleState::Denied
        | SideEffectLifecycleState::Skipped => return Ok(None),
    };
    let idempotency_key =
        provider_write_lifecycle_event_idempotency_key(reconciliation, expected_lifecycle)?;
    if run
        .events
        .iter()
        .any(|event| event.idempotency_key.as_ref() == Some(&idempotency_key))
    {
        return Ok(Some(run.clone()));
    }

    let mut builder = EventBuilder::from_snapshot(
        &run.snapshot,
        request.correlation_id.clone(),
        request.actor.clone(),
    );
    executor.append(&mut builder, event_kind, Some(idempotency_key))?;
    executor
        .rehydrate_and_project(&run.snapshot.identity.run_id)
        .map(Some)
}

fn live_sandbox_event_proof_event_kind(
    run: &WorkflowRun,
    provider_call: &crate::GitHubPullRequestCommentProviderCallOrchestrationResult,
    correlation_id: &CorrelationId,
) -> Result<WorkflowRunEventKind, WorkflowOsError> {
    let expected_lifecycle = match provider_call.provider_response().outcome() {
        GitHubPullRequestCommentWriteOutcome::ProviderSucceeded => {
            SideEffectLifecycleState::Completed
        }
        GitHubPullRequestCommentWriteOutcome::ProviderFailed => SideEffectLifecycleState::Failed,
        GitHubPullRequestCommentWriteOutcome::FixtureValidated
        | GitHubPullRequestCommentWriteOutcome::DryRunValidated => {
            return Err(github_pr_comment_live_sandbox_event_proof_error(
                "outcome.not_provider_result",
                "live sandbox event-proof composition requires a classified provider success or failure",
            ));
        }
    };
    let transition = provider_call.outcome_transition();
    validate_live_sandbox_event_proof_identity(
        run,
        transition,
        expected_lifecycle,
        correlation_id,
    )?;
    let event_payload = transition.event().clone();
    match expected_lifecycle {
        SideEffectLifecycleState::Completed => {
            Ok(WorkflowRunEventKind::SideEffectCompleted(Box::new(event_payload)))
        }
        SideEffectLifecycleState::Failed => {
            Ok(WorkflowRunEventKind::SideEffectFailed(Box::new(event_payload)))
        }
        SideEffectLifecycleState::Proposed
        | SideEffectLifecycleState::Attempted
        | SideEffectLifecycleState::Denied
        | SideEffectLifecycleState::Skipped => Err(
            github_pr_comment_live_sandbox_event_proof_error(
                "lifecycle.unsupported",
                "live sandbox event-proof composition requires a completed or failed lifecycle output",
            ),
        ),
    }
}

fn validate_live_sandbox_event_proof_identity(
    run: &WorkflowRun,
    transition: &SideEffectLifecycleTransitionResult,
    expected_lifecycle: SideEffectLifecycleState,
    correlation_id: &CorrelationId,
) -> Result<(), WorkflowOsError> {
    let record = transition.record();
    let event = transition.event();
    let identity = &run.snapshot.identity;
    if record.lifecycle_state() != expected_lifecycle
        || event.lifecycle_state() != expected_lifecycle
        || record.side_effect_id() != event.side_effect_id()
        || record.workflow_id() != &identity.workflow_id
        || record.workflow_version() != &identity.workflow_version
        || record.schema_version() != &identity.schema_version
        || record.spec_hash() != &identity.spec_content_hash
        || record.run_id() != &identity.run_id
        || record.correlation_id() != Some(correlation_id)
        || event.correlation_id() != Some(correlation_id)
    {
        return Err(github_pr_comment_live_sandbox_event_proof_error(
            "identity_mismatch",
            "live sandbox event-proof composition identity does not match the workflow run",
        ));
    }
    if event.step_id().is_none() || event.skill_id().is_none() || event.skill_version().is_none() {
        return Err(github_pr_comment_live_sandbox_event_proof_error(
            "identity_missing",
            "live sandbox event-proof composition requires step and skill identity",
        ));
    }
    Ok(())
}

fn live_sandbox_event_proof_idempotency_key(
    provider_call: &crate::GitHubPullRequestCommentProviderCallOrchestrationResult,
) -> Result<IdempotencyKey, WorkflowOsError> {
    let record = provider_call.outcome_transition().record();
    let lifecycle = record.lifecycle_state();
    if !matches!(
        lifecycle,
        SideEffectLifecycleState::Completed | SideEffectLifecycleState::Failed
    ) {
        return Err(github_pr_comment_live_sandbox_event_proof_error(
            "lifecycle.unsupported",
            "live sandbox event-proof composition requires a completed or failed lifecycle output",
        ));
    }

    let mut hasher = Sha256::new();
    update_idempotency_hash(&mut hasher, "side_effect", record.side_effect_id().as_str());
    update_idempotency_hash(
        &mut hasher,
        "idempotency",
        record.idempotency().key().as_str(),
    );
    update_idempotency_hash(&mut hasher, "provider", "github_pr_comment");
    update_idempotency_hash(
        &mut hasher,
        "target",
        side_effect_target_kind_label(record.target().kind()),
    );
    update_idempotency_hash(
        &mut hasher,
        "outcome",
        live_sandbox_provider_outcome_label(provider_call.provider_response().outcome()),
    );
    update_idempotency_hash(
        &mut hasher,
        "lifecycle",
        side_effect_lifecycle_label(lifecycle),
    );
    IdempotencyKey::new(format!(
        "live-sandbox-event-proof/{}",
        hex_digest(hasher.finalize())
    ))
    .map_err(|_| {
        github_pr_comment_live_sandbox_event_proof_error(
            "idempotency.invalid",
            "live sandbox event-proof composition could not derive a valid event idempotency key",
        )
    })
}

const fn live_sandbox_provider_outcome_label(
    outcome: GitHubPullRequestCommentWriteOutcome,
) -> &'static str {
    match outcome {
        GitHubPullRequestCommentWriteOutcome::FixtureValidated => "fixture_validated",
        GitHubPullRequestCommentWriteOutcome::DryRunValidated => "dry_run_validated",
        GitHubPullRequestCommentWriteOutcome::ProviderSucceeded => "provider_succeeded",
        GitHubPullRequestCommentWriteOutcome::ProviderFailed => "provider_failed",
    }
}

fn live_sandbox_event_proof_existing_status(
    run: &WorkflowRun,
    expected_kind: &WorkflowRunEventKind,
    idempotency_key: &IdempotencyKey,
) -> Result<Option<GitHubPrCommentLiveSandboxEventProofStatus>, WorkflowOsError> {
    let Some((expected_side_effect_id, expected_lifecycle)) =
        side_effect_outcome_event_identity(expected_kind)
    else {
        return Err(github_pr_comment_live_sandbox_event_proof_error(
            "lifecycle.unsupported",
            "live sandbox event-proof composition requires a completed or failed lifecycle output",
        ));
    };

    for event in &run.events {
        let event_identity = side_effect_outcome_event_identity(&event.kind);
        let idempotency_matches = event.idempotency_key.as_ref() == Some(idempotency_key);
        let outcome_matches = event_identity.is_some_and(|(side_effect_id, lifecycle)| {
            side_effect_id == expected_side_effect_id && lifecycle == expected_lifecycle
        });

        if idempotency_matches && outcome_matches {
            return Ok(Some(
                GitHubPrCommentLiveSandboxEventProofStatus::AlreadyPresent,
            ));
        }
        if idempotency_matches || outcome_matches {
            return Err(github_pr_comment_live_sandbox_event_proof_error(
                "idempotency_conflict",
                "live sandbox event-proof composition conflicts with existing workflow event proof",
            ));
        }
    }

    Ok(None)
}

fn append_live_sandbox_event_proof<B>(
    executor: &LocalExecutor<'_, B>,
    live_sandbox: GitHubPrCommentLiveSandboxRuntimeCompositionResult,
    current_run: WorkflowRun,
    event_kind: WorkflowRunEventKind,
    idempotency_key: IdempotencyKey,
    correlation_id: CorrelationId,
    actor: ActorId,
) -> GitHubPrCommentLiveSandboxEventProofCompositionResult
where
    B: StateBackend,
{
    let mut builder = EventBuilder::from_snapshot(&current_run.snapshot, correlation_id, actor);
    let append_result = executor
        .append(&mut builder, event_kind, Some(idempotency_key))
        .and_then(|()| executor.rehydrate_and_project(&current_run.snapshot.identity.run_id));
    match append_result {
        Ok(run) => GitHubPrCommentLiveSandboxEventProofCompositionResult::new(
            live_sandbox,
            run,
            GitHubPrCommentLiveSandboxEventProofStatus::Appended,
            None,
        ),
        Err(error) => GitHubPrCommentLiveSandboxEventProofCompositionResult::new(
            live_sandbox,
            current_run,
            GitHubPrCommentLiveSandboxEventProofStatus::Failed,
            Some(error),
        ),
    }
}

fn side_effect_outcome_event_identity(
    kind: &WorkflowRunEventKind,
) -> Option<(&SideEffectId, SideEffectLifecycleState)> {
    match kind {
        WorkflowRunEventKind::SideEffectCompleted(event) => {
            Some((event.side_effect_id(), SideEffectLifecycleState::Completed))
        }
        WorkflowRunEventKind::SideEffectFailed(event) => {
            Some((event.side_effect_id(), SideEffectLifecycleState::Failed))
        }
        _ => None,
    }
}

fn github_pr_comment_live_sandbox_event_proof_error(
    reason: &'static str,
    message: &'static str,
) -> WorkflowOsError {
    WorkflowOsError::validation(
        format!("github_pr_comment_live_sandbox_event_proof.{reason}"),
        message,
    )
}

fn validate_provider_write_event_append_identity(
    run: &WorkflowRun,
    transition: &SideEffectLifecycleTransitionResult,
    reconciliation: &GitHubPullRequestCommentProviderWriteReconciliationCandidate,
    expected_lifecycle: SideEffectLifecycleState,
) -> Result<(), WorkflowOsError> {
    let record = transition.record();
    let event = transition.event();
    let identity = &run.snapshot.identity;
    if record.lifecycle_state() != expected_lifecycle
        || event.lifecycle_state() != expected_lifecycle
        || record.side_effect_id() != event.side_effect_id()
        || record.side_effect_id() != reconciliation.side_effect_id()
        || record.idempotency().key() != reconciliation.idempotency_key()
        || record.workflow_id() != &identity.workflow_id
        || record.workflow_version() != &identity.workflow_version
        || record.schema_version() != &identity.schema_version
        || record.spec_hash() != &identity.spec_content_hash
        || record.run_id() != &identity.run_id
        || record.target().kind() != reconciliation.target_kind()
    {
        return Err(executor_error(
            WorkflowOsErrorKind::Validation,
            "executor_github_pr_comment_write.event_append_identity_mismatch",
            "GitHub PR comment provider write event append identity does not match the run and reconciliation context",
        ));
    }
    if event.step_id().is_none() || event.skill_id().is_none() || event.skill_version().is_none() {
        return Err(executor_error(
            WorkflowOsErrorKind::Validation,
            "executor_github_pr_comment_write.event_append_identity_missing",
            "GitHub PR comment provider write event append requires step and skill identity",
        ));
    }
    Ok(())
}

fn provider_write_lifecycle_event_idempotency_key(
    reconciliation: &GitHubPullRequestCommentProviderWriteReconciliationCandidate,
    lifecycle: SideEffectLifecycleState,
) -> Result<IdempotencyKey, WorkflowOsError> {
    let mut hasher = Sha256::new();
    update_idempotency_hash(
        &mut hasher,
        "side_effect",
        reconciliation.side_effect_id().as_str(),
    );
    update_idempotency_hash(
        &mut hasher,
        "idempotency",
        reconciliation.idempotency_key().as_str(),
    );
    update_idempotency_hash(&mut hasher, "provider", reconciliation.provider_kind());
    update_idempotency_hash(
        &mut hasher,
        "target",
        &format!("{:?}", reconciliation.target_kind()),
    );
    update_idempotency_hash(
        &mut hasher,
        "status",
        &format!("{:?}", reconciliation.status()),
    );
    update_idempotency_hash(
        &mut hasher,
        "lifecycle",
        side_effect_lifecycle_label(lifecycle),
    );
    IdempotencyKey::new(format!(
        "provider-write-event/{}",
        hex_digest(hasher.finalize())
    ))
}

fn reconcile_provider_write_after_error(
    inputs: &LocalExecutionGitHubPrCommentProviderWriteInputs<'_>,
    provider_response: Option<&GitHubPullRequestCommentWriteResponse>,
    provider_call_attempted: bool,
    error: &WorkflowOsError,
) -> Option<GitHubPullRequestCommentProviderWriteReconciliationCandidate> {
    reconcile_github_pr_comment_provider_write(
        GitHubPullRequestCommentProviderWriteReconciliationInput {
            attempted_record: inputs.provider_call.provider_call.attempted_record,
            provider_response,
            local_transition: None,
            provider_call_attempted,
            local_transition_error_code: provider_response.map(|_| error.code().to_owned()),
            ambiguity_error_code: (provider_response.is_none() && provider_call_attempted)
                .then(|| error.code().to_owned()),
            sensitivity: inputs.reconciliation_sensitivity,
            redaction: inputs.reconciliation_redaction.clone(),
        },
    )
    .ok()
}

fn provider_write_gate_clarity_from_explicit_inputs(
    inputs: &LocalExecutionGitHubPrCommentProviderWriteInputs<'_>,
    provider_response: Option<&GitHubPullRequestCommentWriteResponse>,
    outcome_transition: Option<&SideEffectLifecycleTransitionResult>,
    reconciliation: Option<&GitHubPullRequestCommentProviderWriteReconciliationCandidate>,
    provider_write_error: Option<&WorkflowOsError>,
    workflow_event_appended: bool,
    provider_call_attempted: bool,
) -> GitHubPullRequestCommentProviderWriteGateClarity {
    let provider_call = &inputs.provider_call.provider_call;
    let attempted_record = provider_call.attempted_record;
    let attempted_record_valid = attempted_record.validate().is_ok();
    let attempted_lifecycle_valid =
        attempted_record.lifecycle_state() == SideEffectLifecycleState::Attempted;
    let preflight_context_available = attempted_record_valid
        && attempted_lifecycle_valid
        && !attempted_record.authority().policy_references.is_empty()
        && attempted_record.idempotency().key() == &provider_call.idempotency_key
        && attempted_record.target().reference() == provider_call.target.reference();
    let approval_linkage = match attempted_record.authority().decision {
        SideEffectAuthorityDecision::ApprovedByHuman => {
            if attempted_record_valid
                && !attempted_record.authority().approval_references.is_empty()
            {
                GitHubPullRequestCommentProviderWriteGateState::Satisfied
            } else {
                GitHubPullRequestCommentProviderWriteGateState::Blocked
            }
        }
        SideEffectAuthorityDecision::AllowedByPolicy => {
            GitHubPullRequestCommentProviderWriteGateState::NotRequired
        }
        SideEffectAuthorityDecision::NotEvaluated
        | SideEffectAuthorityDecision::RequiresApproval
        | SideEffectAuthorityDecision::DeniedByPolicy
        | SideEffectAuthorityDecision::DeniedByApproval
        | SideEffectAuthorityDecision::DeniedByCapability
        | SideEffectAuthorityDecision::DeniedByKillSwitch
        | SideEffectAuthorityDecision::DeniedByValidation
        | SideEffectAuthorityDecision::Unsupported => {
            GitHubPullRequestCommentProviderWriteGateState::Blocked
        }
    };
    let attempted_record_gate = if attempted_record_valid {
        GitHubPullRequestCommentProviderWriteGateState::Satisfied
    } else {
        GitHubPullRequestCommentProviderWriteGateState::Blocked
    };
    let attempted_lifecycle = if attempted_lifecycle_valid {
        GitHubPullRequestCommentProviderWriteGateState::Satisfied
    } else {
        GitHubPullRequestCommentProviderWriteGateState::Blocked
    };
    let preflight_context = if preflight_context_available {
        GitHubPullRequestCommentProviderWriteGateState::Satisfied
    } else {
        GitHubPullRequestCommentProviderWriteGateState::Blocked
    };
    let mut gate_clarity =
        LocalExecutionWithGitHubPrCommentProviderWriteResult::gate_clarity_from_parts(
            provider_response,
            outcome_transition,
            reconciliation,
            provider_write_error,
            workflow_event_appended,
            provider_call_attempted,
        );
    gate_clarity.preflight_context = preflight_context;
    gate_clarity.attempted_record = attempted_record_gate;
    gate_clarity.approval_linkage = approval_linkage;
    gate_clarity.attempted_lifecycle = attempted_lifecycle;
    gate_clarity
}

fn provider_write_approval_presentation_gate<B>(
    executor: &LocalExecutor<'_, B>,
    run: &WorkflowRun,
    inputs: &LocalExecutionGitHubPrCommentProviderWriteInputs<'_>,
    policy: &ApprovalPresentationDefaultEnforcementPolicy,
) -> Result<GitHubPullRequestCommentProviderWriteGateState, WorkflowOsError>
where
    B: StateBackend,
{
    provider_write_approval_presentation_gate_for_record(
        executor,
        run,
        inputs.provider_call.provider_call.attempted_record,
        policy,
    )
}

fn provider_write_approval_presentation_gate_for_record<B>(
    executor: &LocalExecutor<'_, B>,
    run: &WorkflowRun,
    attempted_record: &SideEffectRecord,
    policy: &ApprovalPresentationDefaultEnforcementPolicy,
) -> Result<GitHubPullRequestCommentProviderWriteGateState, WorkflowOsError>
where
    B: StateBackend,
{
    if policy.mode == ApprovalPresentationDefaultEnforcementMode::NotRequired {
        if policy.proof.is_some()
            || policy.max_presentation_age.is_some()
            || policy.sensitive_action_posture.is_some()
        {
            return Err(approval_presentation_default_enforcement_error(
                "approval_presentation_default_enforcement.proof_not_required",
                "approval-presentation proof is not required by this policy",
            ));
        }
        return Ok(GitHubPullRequestCommentProviderWriteGateState::NotRequired);
    }

    let approval_id = provider_write_approval_reference(attempted_record)?;
    let approval = run
        .snapshot
        .approval_requests
        .iter()
        .find(|approval| approval.approval_id == approval_id)
        .cloned()
        .ok_or_else(|| {
            executor_error(
                WorkflowOsErrorKind::Validation,
                "executor_github_pr_comment_write.approval_presentation.approval_not_found",
                "provider-write approval-presentation gate could not resolve approval context",
            )
        })?;
    let decision = provider_write_approval_decision(run, approval_id)?;
    executor.approval_decision_with_presentation_policy(
        &approval,
        decision,
        policy,
        Some(ApprovalPresentationSensitiveActionPosture::WriteAdjacent),
    )?;
    Ok(GitHubPullRequestCommentProviderWriteGateState::Satisfied)
}

fn provider_write_approval_reference(
    attempted_record: &SideEffectRecord,
) -> Result<&str, WorkflowOsError> {
    attempted_record
        .authority()
        .approval_references
        .iter()
        .find(|reference| reference.kind() == SideEffectReferenceKind::ApprovalDecision)
        .map(crate::SideEffectReference::reference)
        .ok_or_else(|| {
            executor_error(
                WorkflowOsErrorKind::Validation,
                "executor_github_pr_comment_write.approval_presentation.approval_reference_missing",
                "provider-write approval-presentation gate requires a stable approval reference",
            )
        })
}

fn validate_live_sandbox_approval_authority_run(
    run: &WorkflowRun,
    attempted_record: &SideEffectRecord,
) -> Result<(), WorkflowOsError> {
    if !run.snapshot.status.is_terminal()
        || run.snapshot.identity.workflow_id != attempted_record.workflow_id().clone()
        || run.snapshot.identity.workflow_version != attempted_record.workflow_version().clone()
        || run.snapshot.identity.schema_version != attempted_record.schema_version().clone()
        || run.snapshot.identity.spec_content_hash != attempted_record.spec_hash().clone()
        || run.snapshot.identity.run_id != attempted_record.run_id().clone()
    {
        return Err(executor_error(
            WorkflowOsErrorKind::Validation,
            "github_pr_comment_live_sandbox.approval_authority.identity_mismatch",
            "live sandbox approval authority requires a matching terminal workflow run",
        ));
    }
    Ok(())
}

fn provider_write_approval_decision(
    run: &WorkflowRun,
    approval_id: &str,
) -> Result<ApprovalDecision, WorkflowOsError> {
    run.events
        .iter()
        .rev()
        .find_map(|event| match &event.kind {
            WorkflowRunEventKind::ApprovalGranted(decision)
            | WorkflowRunEventKind::ApprovalDenied(decision)
                if decision.approval_id == approval_id =>
            {
                Some(decision.clone())
            }
            _ => None,
        })
        .ok_or_else(|| {
            executor_error(
                WorkflowOsErrorKind::Validation,
                "executor_github_pr_comment_write.approval_presentation.decision_not_found",
                "provider-write approval-presentation gate could not resolve approval decision",
            )
        })
}

fn report_artifact_provider_integration(
    input: Option<&LocalExecutionReportArtifactProviderIntegrationInputs>,
) -> ReportArtifactWriteProviderIntegration<'_> {
    match input {
        Some(LocalExecutionReportArtifactProviderIntegrationInputs::GitHubPullRequestComment {
            side_effect_id,
            workflow_events,
            citation_policy,
        }) => ReportArtifactWriteProviderIntegration::GitHubPullRequestComment {
            side_effect_id,
            workflow_events: Some(workflow_events.as_slice()),
            citation_policy: *citation_policy,
            provider_event_proof_gate_policy:
                GitHubPullRequestCommentProviderReportArtifactEventProofGatePolicy::default(),
            provider_disclosures: &[],
        },
        None => ReportArtifactWriteProviderIntegration::None,
    }
}

fn execute_for_report_artifact_path<B>(
    executor: &LocalExecutor<'_, B>,
    request: &LocalExecutionRequest,
    caller_proof_marker_policy: Option<WorkReportArtifactApprovalProofMarkerGatePolicy>,
) -> Result<(WorkflowRun, WorkflowReportArtifactPolicies), WorkflowOsError>
where
    B: StateBackend,
{
    let run_id = request
        .run_id
        .clone()
        .unwrap_or_else(WorkflowRunId::generate);
    if executor.backend.read_events(&run_id)?.is_empty() {
        let validation_capability = ProjectValidationCapability::ReportArtifactCapable {
            workflow_id: request.workflow_id.clone(),
            approval_proof_marker_capable: caller_proof_marker_policy.is_some(),
        };
        let mut plan = LocalExecutor::<B>::prepare_execution_with_capability_and_artifact_policy(
            request,
            run_id,
            &validation_capability,
            caller_proof_marker_policy,
        )?;
        let policies = WorkflowReportArtifactPolicies {
            high_assurance_disclosure_policy: plan.workflow_report_artifact_policy,
            approval_proof_marker_policy: plan.workflow_report_artifact_proof_marker_policy,
        };
        executor.evaluate_pre_run_policy(&plan, &request.actor, &request.correlation_id)?;
        executor.append_run_start(&mut plan)?;
        let run = executor.execute_steps(plan, &request.correlation_id)?;
        return Ok((run, policies));
    }

    let run = executor.backend.rehydrate_run(&run_id)?;
    let policies = workflow_report_artifact_policy_for_request_with_proof_marker_policy(
        request,
        &run.snapshot.identity,
        caller_proof_marker_policy,
    )?;
    Ok((run, policies))
}

fn generate_work_report_for_artifact_path(
    run: &WorkflowRun,
    report: &LocalExecutionReportInputs,
    side_effect_discovery: Option<LocalExecutionSideEffectDiscoveryInputs>,
    side_effect_store: &impl SideEffectRecordStore,
) -> Result<WorkReport, WorkflowOsError> {
    let mut report = report.clone();
    apply_before_report_hook_checkpoint(run, &mut report)?;
    let input = terminal_report_input_for_run(run, &report);
    if let Some(side_effect_discovery) = side_effect_discovery {
        generate_terminal_local_work_report_with_side_effect_discovery(
            side_effect_store,
            input,
            side_effect_discovery.into(),
        )
    } else {
        let (_, work_report) = expose_terminal_local_work_report_result(input)?.into_parts();
        Ok(work_report)
    }
}

enum StepExecutionResult {
    Succeeded(Box<ExecutionPlan>),
    Terminal(Box<WorkflowRun>),
}

fn terminal_work_report_status_error() -> WorkflowOsError {
    WorkflowOsError::validation(
        "work_report_generation.status.not_terminal",
        "terminal local work report generation requires a completed, failed, or canceled run",
    )
}

fn validate_approval_presentation_enforcement(
    presentation: &ApprovalPresentationRecord,
    approval: &ApprovalRequest,
    decision: &ApprovalDecision,
    max_presentation_age: Option<Duration>,
) -> Result<(), WorkflowOsError> {
    validate_approval_presentation_for_request(ApprovalPresentationValidationInput {
        presentation,
        approval_request: approval,
    })
    .map_err(|_| {
        approval_presentation_enforcement_error(
            "approval_presentation_enforcement.proof_mismatch",
            "approval-presentation proof does not match the approval request",
        )
    })?;

    if presentation.presented_at() > decision.decided_at {
        return Err(approval_presentation_enforcement_error(
            "approval_presentation_enforcement.decision_time_invalid",
            "approval-presentation proof must be presented before the approval decision",
        ));
    }

    if let Some(max_age) = max_presentation_age {
        let max_age = time::Duration::try_from(max_age).map_err(|_| {
            approval_presentation_enforcement_error(
                "approval_presentation_enforcement.proof_stale",
                "approval-presentation proof is stale",
            )
        })?;
        let proof_age = decision.decided_at.as_offset_date_time()
            - presentation.presented_at().as_offset_date_time();
        if proof_age > max_age {
            return Err(approval_presentation_enforcement_error(
                "approval_presentation_enforcement.proof_stale",
                "approval-presentation proof is stale",
            ));
        }
    }

    Ok(())
}

fn approval_presentation_enforcement_error(
    code: &'static str,
    message: &'static str,
) -> WorkflowOsError {
    WorkflowOsError::new(WorkflowOsErrorKind::Validation, code, message)
}

fn approval_presentation_default_enforcement_error(
    code: &'static str,
    message: &'static str,
) -> WorkflowOsError {
    WorkflowOsError::new(WorkflowOsErrorKind::Validation, code, message)
}

fn workflow_report_artifact_policy_for_request_with_proof_marker_policy(
    request: &LocalExecutionRequest,
    identity: &WorkflowRunIdentity,
    caller_proof_marker_policy: Option<WorkReportArtifactApprovalProofMarkerGatePolicy>,
) -> Result<WorkflowReportArtifactPolicies, WorkflowOsError> {
    let bundle = load_validated_project_bundle(
        &request.project_root,
        ProjectValidationCapability::ReportArtifactCapable {
            workflow_id: request.workflow_id.clone(),
            approval_proof_marker_capable: caller_proof_marker_policy.is_some(),
        },
    )?;
    let workflow = find_workflow(&bundle.workflows, &request.workflow_id)?;
    validate_loaded_workflow_matches_run_identity(workflow, identity)?;
    let high_assurance_disclosure_policy =
        derive_workflow_report_artifact_gate_policy(WorkflowReportArtifactGateDerivationInput {
            workflow: &workflow.definition,
        })?
        .high_assurance_disclosure_policy();
    let proof_marker_derivation_mode = if caller_proof_marker_policy.is_some() {
        WorkflowReportArtifactProofMarkerDerivationMode::ArtifactCapable
    } else {
        WorkflowReportArtifactProofMarkerDerivationMode::DefaultValidation
    };
    let approval_proof_marker_policy =
        derive_workflow_report_artifact_approval_proof_marker_gate_policy(
            WorkflowReportArtifactProofMarkerGateDerivationInput {
                workflow: &workflow.definition,
                caller_policy: caller_proof_marker_policy,
                derivation_mode: proof_marker_derivation_mode,
            },
        )?
        .approval_proof_marker_policy();
    Ok(WorkflowReportArtifactPolicies {
        high_assurance_disclosure_policy,
        approval_proof_marker_policy,
    })
}

fn validate_loaded_workflow_matches_run_identity(
    workflow: &LoadedSpec<WorkflowDefinition>,
    identity: &WorkflowRunIdentity,
) -> Result<(), WorkflowOsError> {
    if workflow.definition.id != identity.workflow_id
        || workflow.definition.schema_version != identity.schema_version
        || workflow.definition.version != identity.workflow_version
        || workflow.content_hash != identity.spec_content_hash
    {
        return Err(executor_error(
            WorkflowOsErrorKind::InvalidState,
            "executor.report_artifact.workflow_identity_mismatch",
            "report artifact policy derivation requires the current workflow spec to match the existing run identity",
        ));
    }
    Ok(())
}

fn load_validated_project_bundle(
    project_root: &Path,
    validation_capability: ProjectValidationCapability,
) -> Result<ProjectBundle, WorkflowOsError> {
    let load_result = load_project(project_root);
    if load_result.has_errors() {
        return Err(WorkflowOsError::validation(
            "executor.project.load_failed",
            "project could not be loaded for local execution",
        )
        .with_diagnostics(load_result.diagnostics));
    }
    let validation = validate_loaded_project_with_capability(&load_result, validation_capability);
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
    Ok(bundle)
}

fn terminal_report_input_for_run<'a>(
    run: &'a WorkflowRun,
    report: &LocalExecutionReportInputs,
) -> TerminalLocalWorkReportInput<'a> {
    TerminalLocalWorkReportInput {
        report_id: report.report_id.clone(),
        report_contract_id: report.report_contract_id.clone(),
        report_contract_version: report.report_contract_version.clone(),
        run,
        generated_at: report.generated_at,
        generated_by: report.generated_by.clone(),
        correlation_id: report.correlation_id.clone().or_else(|| {
            run.events
                .last()
                .and_then(|event| event.correlation_id.clone())
        }),
        sensitivity: report.sensitivity,
        redaction: report.redaction.clone(),
        evidence_reference_ids: report.evidence_reference_ids.clone(),
        validation_reference_ids: report.validation_reference_ids.clone(),
        local_check_result_references: report.local_check_result_references.clone(),
        workflow_event_ids: report.workflow_event_ids.clone(),
        audit_event_ids: report.audit_event_ids.clone(),
        adapter_telemetry_references: report.adapter_telemetry_references.clone(),
        policy_event_ids: report.policy_event_ids.clone(),
        approval_reference_ids: report.approval_reference_ids.clone(),
        approval_proof_marker_citation_policy: report.approval_proof_marker_citation_policy,
        high_assurance_approval: report.high_assurance_approval.clone(),
        typed_handoff_ids: report.typed_handoff_ids.clone(),
        agent_harness_hook_invocation_ids: report.agent_harness_hook_invocation_ids.clone(),
        agent_harness_hook_disclosure_ids: report.agent_harness_hook_disclosure_ids.clone(),
        side_effect_ids: report.side_effect_ids.clone(),
        github_pr_comment_provider_disclosures: report
            .github_pr_comment_provider_disclosures
            .clone(),
        incomplete_work: report.incomplete_work.clone(),
        known_limitations: report.known_limitations.clone(),
        risks: report.risks.clone(),
        handoff_notes: report.handoff_notes.clone(),
    }
}

struct BeforeReportHookExecutionResult {
    hook_invocation_id: AgentHarnessHookInvocationId,
    disclosure_ids: Vec<AgentHarnessHookDisclosureId>,
}

fn execute_before_report_hook(
    run: &WorkflowRun,
    hook_input: LocalExecutionBeforeReportHookInput,
) -> Result<BeforeReportHookExecutionResult, WorkflowOsError> {
    if hook_input.invocation.hook_kind != AgentHarnessHookKind::BeforeReport {
        return Err(executor_error(
            WorkflowOsErrorKind::Validation,
            "executor.hook.before_report.kind_mismatch",
            "before-report executor hook input must use the BeforeReport hook kind",
        ));
    }

    let identity = &run.snapshot.identity;
    if hook_input.invocation.workflow_id != identity.workflow_id
        || hook_input.invocation.workflow_version != identity.workflow_version
        || hook_input.invocation.run_id != identity.run_id
        || hook_input.invocation.schema_version != identity.schema_version
        || hook_input.invocation.spec_hash != identity.spec_content_hash
    {
        return Err(executor_error(
            WorkflowOsErrorKind::Validation,
            "executor.hook.before_report.identity_mismatch",
            "before-report executor hook input must match the terminal workflow run identity",
        ));
    }

    let result = execute_runtime_agent_harness_hook(RuntimeAgentHarnessHookInput {
        hook_invocation_id: hook_input.hook_invocation_id,
        invocation: hook_input.invocation,
    })?;
    Ok(BeforeReportHookExecutionResult {
        hook_invocation_id: result.hook_invocation_id().clone(),
        disclosure_ids: result
            .invocation_result()
            .disclosures()
            .iter()
            .map(|disclosure| disclosure.disclosure_id().clone())
            .collect(),
    })
}

fn apply_before_report_hook_checkpoint(
    run: &WorkflowRun,
    report: &mut LocalExecutionReportInputs,
) -> Result<(), WorkflowOsError> {
    let Some(hook_input) = report.before_report_hook.take() else {
        if report.hook_checkpoints.require_before_report {
            return Err(executor_error(
                WorkflowOsErrorKind::InvalidState,
                "executor.hook.before_report.required",
                "before-report hook checkpoint is required before report generation",
            ));
        }
        return Ok(());
    };

    let hook_result = execute_before_report_hook(run, hook_input)?;
    let hook_invocation_id = hook_result.hook_invocation_id;
    if !report
        .agent_harness_hook_invocation_ids
        .contains(&hook_invocation_id)
    {
        report
            .agent_harness_hook_invocation_ids
            .push(hook_invocation_id);
    }
    merge_hook_disclosure_ids(
        &mut report.agent_harness_hook_disclosure_ids,
        hook_result.disclosure_ids,
    );
    Ok(())
}

fn merge_hook_disclosure_ids(
    existing: &mut Vec<AgentHarnessHookDisclosureId>,
    discovered: Vec<AgentHarnessHookDisclosureId>,
) {
    for disclosure_id in discovered {
        if !existing.contains(&disclosure_id) {
            existing.push(disclosure_id);
        }
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
    resolved_execution_context_hash: crate::SpecContentHash,
    steps: Vec<StepExecutionPlan>,
    current_step_index: usize,
    step_scheduled: bool,
    approval_already_granted: bool,
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
    policy_effects: PolicyEffectSet,
    before_skill_invocation_checkpoints: LocalExecutionBeforeSkillInvocationCheckpointInputs,
    before_skill_invocation_hook: Option<LocalExecutionBeforeSkillInvocationHookInput>,
    side_effect_events: Vec<LocalExecutionSideEffectEventInput>,
    side_effect_lifecycle_events: Vec<LocalExecutionSideEffectLifecycleEventInput>,
    workflow_report_artifact_policy: WorkReportArtifactHighAssuranceDisclosurePolicy,
    workflow_report_artifact_proof_marker_policy:
        Option<WorkReportArtifactApprovalProofMarkerGatePolicy>,
}

#[derive(Clone)]
struct StepExecutionPlan {
    step: StepDefinition,
    skill: SkillDefinition,
    skill_id: SkillId,
    skill_version: SkillVersion,
    invocation_id: SkillInvocationId,
    idempotency_key: IdempotencyKey,
    retry_max_attempts: u32,
    escalation_enabled: bool,
    approval_sensitivity: crate::ApprovalSensitivity,
    adapter_id: Option<String>,
    capabilities: Vec<Capability>,
    policy_effects: PolicyEffectSet,
}

impl ExecutionPlan {
    fn set_current_step(&mut self, index: usize) -> Result<(), WorkflowOsError> {
        let next = self.steps.get(index).cloned().ok_or_else(|| {
            executor_error(
                WorkflowOsErrorKind::InvalidState,
                "executor.workflow.multistep.step_index_invalid",
                "step cursor is outside the executable step list",
            )
        })?;
        self.current_step_index = index;
        self.step_scheduled = false;
        self.approval_already_granted = false;
        self.step = next.step;
        self.skill = next.skill;
        self.skill_id = next.skill_id;
        self.skill_version = next.skill_version;
        self.invocation_id = next.invocation_id;
        self.idempotency_key = next.idempotency_key;
        self.retry_max_attempts = next.retry_max_attempts;
        self.escalation_enabled = next.escalation_enabled;
        self.approval_sensitivity = next.approval_sensitivity;
        self.adapter_id = next.adapter_id;
        self.capabilities = next.capabilities;
        self.policy_effects = next.policy_effects;
        Ok(())
    }

    fn has_next_step(&self) -> bool {
        self.current_step_index + 1 < self.steps.len()
    }

    fn advance_to_next_step(&mut self) -> Result<(), WorkflowOsError> {
        self.set_current_step(self.current_step_index + 1)
    }

    fn should_continue_after_success(&self) -> bool {
        matches!(
            self.step.terminal_behavior,
            Some(TerminalBehavior::Continue)
        )
    }
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
    fn new(
        run_id: WorkflowRunId,
        workflow_id: WorkflowId,
        schema_version: SchemaVersion,
        workflow_version: WorkflowVersion,
        spec_hash: crate::SpecContentHash,
        correlation_id: CorrelationId,
        actor: ActorId,
    ) -> Self {
        Self {
            next_sequence_number: EventSequenceNumber::first(),
            run_id,
            workflow_id,
            schema_version,
            workflow_version,
            spec_hash,
            correlation_id,
            actor,
        }
    }

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

fn reject_branch_execution(
    workflow: &LoadedSpec<WorkflowDefinition>,
) -> Result<(), WorkflowOsError> {
    if workflow.definition.branches.is_empty() {
        return Ok(());
    }
    Err(executor_error(
        WorkflowOsErrorKind::Unsupported,
        "executor.workflow.multistep.unsupported_branching",
        "local executor does not execute branch declarations",
    ))
}

fn step_execution_plans(
    steps: &[StepDefinition],
    skills: &[LoadedSpec<SkillDefinition>],
    policies: &[LoadedSpec<PolicySpecDocument>],
    run_id: &WorkflowRunId,
    workflow_id: &WorkflowId,
    workflow_version: &WorkflowVersion,
) -> Result<Vec<StepExecutionPlan>, WorkflowOsError> {
    if steps.is_empty() {
        return Err(executor_error(
            WorkflowOsErrorKind::Unsupported,
            "executor.workflow.no_steps",
            "local executor requires at least one step",
        ));
    }

    steps
        .iter()
        .map(|step| {
            let skill = resolve_skill(skills, step)?;
            let skill_id = skill.definition.id.clone();
            let skill_version = skill.definition.version.clone();
            let policy_effects = policy_effects_for_step(step, policies);
            Ok(StepExecutionPlan {
                step: step.clone(),
                skill: skill.definition.clone(),
                skill_id: skill_id.clone(),
                skill_version: skill_version.clone(),
                invocation_id: SkillInvocationId::generate(),
                idempotency_key: invocation_idempotency_key(
                    run_id,
                    workflow_id,
                    workflow_version,
                    &step.id,
                    &skill_id,
                    &skill_version,
                )?,
                retry_max_attempts: retry_max_attempts(&policy_effects),
                escalation_enabled: policy_effects.allows_escalation(),
                approval_sensitivity: skill.definition.approval_sensitivity,
                adapter_id: skill
                    .definition
                    .adapter_requirements
                    .first()
                    .map(|adapter| adapter.adapter_id.to_string()),
                capabilities: capabilities_for_skill(&skill.definition),
                policy_effects,
            })
        })
        .collect()
}

fn resolved_execution_context_hash(
    workflow: &LoadedSpec<WorkflowDefinition>,
    skills: &[LoadedSpec<SkillDefinition>],
    policies: &[LoadedSpec<PolicySpecDocument>],
    checkpoint_inputs: &LocalExecutionBeforeSkillInvocationCheckpointInputs,
    request: &LocalExecutionRequest,
    artifact_policies: WorkflowReportArtifactPolicies,
) -> Result<crate::SpecContentHash, WorkflowOsError> {
    let mut hasher = Sha256::new();
    update_idempotency_hash(
        &mut hasher,
        "domain",
        "workflow-os/resolved-execution-context/v1",
    );
    update_idempotency_hash(&mut hasher, "workflow", workflow.definition.id.as_str());
    update_idempotency_hash(
        &mut hasher,
        "workflow_version",
        workflow.definition.version.as_str(),
    );
    update_idempotency_hash(
        &mut hasher,
        "schema_version",
        workflow.definition.schema_version.as_str(),
    );
    update_idempotency_hash(&mut hasher, "workflow_hash", workflow.content_hash.as_str());

    for (index, step) in workflow.definition.steps.iter().enumerate() {
        let skill = resolve_skill(skills, step)?;
        update_idempotency_hash(&mut hasher, "step_index", &index.to_string());
        update_idempotency_hash(&mut hasher, "step", step.id.as_str());
        update_idempotency_hash(&mut hasher, "skill", skill.definition.id.as_str());
        update_idempotency_hash(
            &mut hasher,
            "skill_version",
            skill.definition.version.as_str(),
        );
        update_idempotency_hash(&mut hasher, "skill_hash", skill.content_hash.as_str());
    }

    let mut referenced_policies = BTreeMap::new();
    for step in &workflow.definition.steps {
        for policy_ref in step
            .policy_requirements
            .iter()
            .chain(step.approval_policy.iter().map(|approval| &approval.policy))
            .chain(step.retry_policy.iter().map(|retry| &retry.policy))
            .chain(
                step.escalation_policy
                    .iter()
                    .map(|escalation| &escalation.policy),
            )
        {
            if let Some(policy) = policies
                .iter()
                .find(|policy| policy.definition.id == policy_ref.id)
            {
                referenced_policies.insert(
                    policy.definition.id.as_str().to_owned(),
                    policy.content_hash.as_str().to_owned(),
                );
            }
        }
    }
    for (policy_id, content_hash) in referenced_policies {
        update_idempotency_hash(&mut hasher, "policy", &policy_id);
        update_idempotency_hash(&mut hasher, "policy_hash", &content_hash);
    }

    let mut checkpoint_ids = checkpoint_inputs
        .required_step_ids
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    checkpoint_ids.sort();
    checkpoint_ids.dedup();
    for step_id in checkpoint_ids {
        update_idempotency_hash(&mut hasher, "required_checkpoint", &step_id);
    }
    update_idempotency_hash(
        &mut hasher,
        "hook_input_present",
        if request.before_skill_invocation_hook.is_some() {
            "true"
        } else {
            "false"
        },
    );
    update_idempotency_hash(
        &mut hasher,
        "side_effect_event_count",
        &request.side_effect_events.len().to_string(),
    );
    update_idempotency_hash(
        &mut hasher,
        "side_effect_lifecycle_event_count",
        &request.side_effect_lifecycle_events.len().to_string(),
    );
    update_idempotency_hash(
        &mut hasher,
        "artifact_policies",
        &format!("{artifact_policies:?}"),
    );

    Ok(crate::SpecContentHash::from_bytes(hasher.finalize()))
}

fn approval_expires_after(workflow: &WorkflowDefinition) -> Option<String> {
    workflow
        .approval_requirements
        .iter()
        .find_map(|requirement| {
            requirement
                .expires_after
                .as_ref()
                .map(|duration| duration.duration.clone())
        })
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
    let mut hasher = Sha256::new();
    update_idempotency_hash(&mut hasher, "run", run_id.as_str());
    update_idempotency_hash(&mut hasher, "workflow", workflow_id.as_str());
    update_idempotency_hash(&mut hasher, "version", workflow_version.as_str());
    update_idempotency_hash(&mut hasher, "step", step_id.as_str());
    update_idempotency_hash(&mut hasher, "skill", skill_id.as_str());
    update_idempotency_hash(&mut hasher, "skillversion", skill_version.as_str());
    IdempotencyKey::new(format!(
        "skill-invocation/{}",
        hex_digest(hasher.finalize())
    ))
}

fn update_idempotency_hash(hasher: &mut Sha256, label: &str, value: &str) {
    hasher.update(label.as_bytes());
    hasher.update([0]);
    hasher.update(value.as_bytes());
    hasher.update([0xff]);
}

fn hex_digest(bytes: impl AsRef<[u8]>) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let bytes = bytes.as_ref();
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(HEX[(byte >> 4) as usize] as char);
        encoded.push(HEX[(byte & 0x0f) as usize] as char);
    }
    encoded
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

fn hook_requested_idempotency_key(
    invocation_key: &IdempotencyKey,
) -> Result<IdempotencyKey, WorkflowOsError> {
    IdempotencyKey::new(format!("{invocation_key}/hook/bsi/req"))
}

fn hook_evaluated_idempotency_key(
    invocation_key: &IdempotencyKey,
) -> Result<IdempotencyKey, WorkflowOsError> {
    IdempotencyKey::new(format!("{invocation_key}/hook/bsi/eval"))
}

fn side_effect_event_idempotency_key(
    index: usize,
    lifecycle: SideEffectLifecycleState,
) -> Result<IdempotencyKey, WorkflowOsError> {
    IdempotencyKey::new(format!(
        "side-effect/{index}/{}",
        side_effect_lifecycle_label(lifecycle)
    ))
}

fn side_effect_lifecycle_event_idempotency_key(
    index: usize,
    lifecycle: SideEffectLifecycleState,
) -> Result<IdempotencyKey, WorkflowOsError> {
    IdempotencyKey::new(format!(
        "side-effect-lifecycle/{index}/{}",
        side_effect_lifecycle_label(lifecycle)
    ))
}

fn side_effect_lifecycle_label(lifecycle: SideEffectLifecycleState) -> &'static str {
    match lifecycle {
        SideEffectLifecycleState::Proposed => "proposed",
        SideEffectLifecycleState::Attempted => "attempted",
        SideEffectLifecycleState::Completed => "completed",
        SideEffectLifecycleState::Denied => "denied",
        SideEffectLifecycleState::Skipped => "skipped",
        SideEffectLifecycleState::Failed => "failed",
    }
}

const fn side_effect_target_kind_label(kind: SideEffectTargetKind) -> &'static str {
    match kind {
        SideEffectTargetKind::ExternalResource => "external_resource",
        SideEffectTargetKind::AdapterResource => "adapter_resource",
        SideEffectTargetKind::WorkflowResource => "workflow_resource",
        SideEffectTargetKind::LocalResource => "local_resource",
        SideEffectTargetKind::ProviderOperation => "provider_operation",
        SideEffectTargetKind::Unknown => "unknown",
    }
}

fn validate_side_effect_event_input(
    plan: &ExecutionPlan,
    input: &LocalExecutionSideEffectEventInput,
) -> Result<(), WorkflowOsError> {
    if input.skill_id != plan.skill_id || input.skill_version != plan.skill_version {
        return Err(executor_error(
            WorkflowOsErrorKind::Validation,
            "executor.side_effect_event.skill_mismatch",
            "side-effect event input must match the active skill identity",
        ));
    }
    if input
        .event
        .step_id()
        .is_some_and(|step_id| step_id != &plan.step.id)
        || input
            .event
            .skill_id()
            .is_some_and(|skill_id| skill_id != &plan.skill_id)
        || input
            .event
            .skill_version()
            .is_some_and(|skill_version| skill_version != &plan.skill_version)
        || input
            .event
            .correlation_id()
            .is_some_and(|correlation_id| correlation_id != &plan.event_builder.correlation_id)
    {
        return Err(executor_error(
            WorkflowOsErrorKind::Validation,
            "executor.side_effect_event.identity_mismatch",
            "side-effect event input must match the active workflow invocation identity",
        ));
    }
    Ok(())
}

fn validate_side_effect_lifecycle_event_input(
    plan: &ExecutionPlan,
    input: &LocalExecutionSideEffectLifecycleEventInput,
) -> Result<(), WorkflowOsError> {
    if input.skill_id != plan.skill_id || input.skill_version != plan.skill_version {
        return Err(executor_error(
            WorkflowOsErrorKind::Validation,
            "executor.side_effect_lifecycle_event.skill_mismatch",
            "side-effect lifecycle event input must match the active skill identity",
        ));
    }
    if input
        .event()
        .step_id()
        .is_some_and(|step_id| step_id != &plan.step.id)
        || input
            .event()
            .skill_id()
            .is_some_and(|skill_id| skill_id != &plan.skill_id)
        || input
            .event()
            .skill_version()
            .is_some_and(|skill_version| skill_version != &plan.skill_version)
        || input
            .event()
            .correlation_id()
            .is_some_and(|correlation_id| correlation_id != &plan.event_builder.correlation_id)
    {
        return Err(executor_error(
            WorkflowOsErrorKind::Validation,
            "executor.side_effect_lifecycle_event.identity_mismatch",
            "side-effect lifecycle event input must match the active workflow invocation identity",
        ));
    }
    Ok(())
}

fn map_github_pr_comment_side_effect_event_input_error(error: &WorkflowOsError) -> WorkflowOsError {
    match error.code() {
        "github_pr_comment_side_effect_event.record_missing" => executor_error(
            WorkflowOsErrorKind::InvalidState,
            "github_pr_comment_side_effect_event_input.record_missing",
            "GitHub PR comment SideEffect record is missing",
        ),
        "github_pr_comment_side_effect_event.store_read_failed" => executor_error(
            WorkflowOsErrorKind::InvalidState,
            "github_pr_comment_side_effect_event_input.store_read_failed",
            "GitHub PR comment SideEffect record could not be loaded",
        ),
        "github_pr_comment_side_effect_event.identity_mismatch" => executor_error(
            WorkflowOsErrorKind::Validation,
            "github_pr_comment_side_effect_event_input.identity_mismatch",
            "GitHub PR comment SideEffect record identity does not match expected context",
        ),
        "github_pr_comment_side_effect_event.unsupported_lifecycle"
        | "github_pr_comment_side_effect_event.unsupported_capability"
        | "github_pr_comment_side_effect_event.unsupported_target"
        | "github_pr_comment_side_effect_event.outcome_not_supported"
        | "github_pr_comment_side_effect_event.event.invalid"
        | "github_pr_comment_side_effect_event.reference_count.invalid" => executor_error(
            WorkflowOsErrorKind::Validation,
            "github_pr_comment_side_effect_event_input.record_invalid",
            "GitHub PR comment SideEffect record cannot be converted to an executor event input",
        ),
        _ => executor_error(
            WorkflowOsErrorKind::InvalidState,
            "github_pr_comment_side_effect_event_input.record_invalid",
            "GitHub PR comment SideEffect event input could not be constructed",
        ),
    }
}

fn validate_before_skill_invocation_checkpoint_inputs(
    checkpoints: &LocalExecutionBeforeSkillInvocationCheckpointInputs,
) -> Result<(), WorkflowOsError> {
    for (index, step_id) in checkpoints.required_step_ids.iter().enumerate() {
        if checkpoints
            .required_step_ids
            .iter()
            .skip(index + 1)
            .any(|candidate| candidate == step_id)
        {
            return Err(executor_error(
                WorkflowOsErrorKind::Validation,
                "executor.hook.before_skill_invocation.duplicate_required_step",
                "before-skill-invocation required checkpoint policy contains duplicate step requirements",
            ));
        }
    }
    Ok(())
}

fn before_skill_invocation_checkpoint_inputs(
    request: &LocalExecutionRequest,
) -> Result<LocalExecutionBeforeSkillInvocationCheckpointInputs, WorkflowOsError> {
    validate_before_skill_invocation_checkpoint_inputs(
        &request.before_skill_invocation_checkpoints,
    )?;
    Ok(request.before_skill_invocation_checkpoints.clone())
}

fn validate_before_skill_invocation_required_steps(
    checkpoints: &LocalExecutionBeforeSkillInvocationCheckpointInputs,
    steps: &[StepDefinition],
) -> Result<(), WorkflowOsError> {
    for required_step_id in &checkpoints.required_step_ids {
        if !steps.iter().any(|step| &step.id == required_step_id) {
            return Err(executor_error(
                WorkflowOsErrorKind::Validation,
                "executor.hook.before_skill_invocation.unknown_required_step",
                "before-skill-invocation required checkpoint policy references an unknown step",
            ));
        }
    }
    Ok(())
}

fn required_before_skill_invocation_hook_error() -> WorkflowOsError {
    executor_error(
        WorkflowOsErrorKind::InvalidState,
        "executor.hook.before_skill_invocation.required",
        "before-skill-invocation hook is required before skill invocation",
    )
}

fn validate_before_skill_invocation_hook_input(
    plan: &ExecutionPlan,
    hook_input: &LocalExecutionBeforeSkillInvocationHookInput,
) -> Result<(), WorkflowOsError> {
    if hook_input.invocation.hook_kind != AgentHarnessHookKind::BeforeSkillInvocation {
        return Err(executor_error(
            WorkflowOsErrorKind::Validation,
            "executor.hook.before_skill_invocation.kind_mismatch",
            "before-skill-invocation executor hook input must use the BeforeSkillInvocation hook kind",
        ));
    }
    if hook_input.skill_id != plan.skill_id || hook_input.skill_version != plan.skill_version {
        return Err(executor_error(
            WorkflowOsErrorKind::Validation,
            "executor.hook.before_skill_invocation.skill_mismatch",
            "before-skill-invocation executor hook input must match the active skill identity",
        ));
    }
    if hook_input.invocation.workflow_id != plan.event_builder.workflow_id
        || hook_input.invocation.workflow_version != plan.event_builder.workflow_version
        || hook_input.invocation.run_id != plan.event_builder.run_id
        || hook_input.invocation.schema_version != plan.event_builder.schema_version
        || hook_input.invocation.spec_hash != plan.event_builder.spec_hash
        || hook_input.invocation.step_id.as_ref() != Some(&plan.step.id)
    {
        return Err(executor_error(
            WorkflowOsErrorKind::Validation,
            "executor.hook.before_skill_invocation.identity_mismatch",
            "before-skill-invocation executor hook input must match the active workflow run and step identity",
        ));
    }
    Ok(())
}

fn hook_workflow_event_from_input(
    hook_input: &LocalExecutionBeforeSkillInvocationHookInput,
    status: AgentHarnessHookInvocationStatus,
) -> Result<AgentHarnessHookWorkflowEvent, WorkflowOsError> {
    AgentHarnessHookWorkflowEvent::new(AgentHarnessHookWorkflowEventDefinition {
        hook_invocation_id: hook_input.hook_invocation_id.clone(),
        contract_id: hook_input.invocation.contract.contract_id().clone(),
        contract_version: hook_input.invocation.contract.contract_version().clone(),
        hook_kind: hook_input.invocation.hook_kind,
        status,
        step_id: hook_input.invocation.step_id.clone(),
        phase_id: hook_input.invocation.phase_id.clone(),
        correlation_id: hook_input.invocation.correlation_id.clone(),
        input_reference_count: u32::try_from(hook_input.invocation.input_references.len())
            .map_err(|_| {
                executor_error(
                    WorkflowOsErrorKind::Validation,
                    "executor.hook.before_skill_invocation.reference_count",
                    "before-skill-invocation hook input reference count is too large",
                )
            })?,
        output_reference_count: u32::try_from(hook_input.invocation.output_references.len())
            .map_err(|_| {
                executor_error(
                    WorkflowOsErrorKind::Validation,
                    "executor.hook.before_skill_invocation.reference_count",
                    "before-skill-invocation hook output reference count is too large",
                )
            })?,
        redaction: hook_input.invocation.redaction.clone(),
        sensitivity: hook_input.invocation.sensitivity,
    })
}

fn approval_id(run_id: &WorkflowRunId, step_id: &StepId) -> String {
    format!("approval/{run_id}/{step_id}")
}

fn policy_effects_for_step(
    step: &StepDefinition,
    policies: &[LoadedSpec<PolicySpecDocument>],
) -> PolicyEffectSet {
    let mut effects = PolicyEffectSet::default();
    for policy_ref in step
        .policy_requirements
        .iter()
        .chain(step.approval_policy.iter().map(|approval| &approval.policy))
        .chain(step.retry_policy.iter().map(|retry| &retry.policy))
        .chain(
            step.escalation_policy
                .iter()
                .map(|escalation| &escalation.policy),
        )
    {
        let Some(policy) = policies
            .iter()
            .find(|policy| policy.definition.id == policy_ref.id)
        else {
            continue;
        };
        for rule in &policy.definition.rules {
            if let Ok(effect) = PolicyEffect::parse(&rule.effect) {
                effects.insert(effect);
            }
        }
    }
    effects
}

fn retry_max_attempts(policy_effects: &PolicyEffectSet) -> u32 {
    if !policy_effects.has_bounded_retry() {
        return 1;
    }
    policy_effects.max_attempts().unwrap_or(2).max(1)
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
        .map(|capability| Capability::from_declared_name(&capability.name))
        .collect::<Vec<_>>();
    if !capabilities
        .iter()
        .any(|capability| capability == &Capability::AuditWrite)
    {
        capabilities.push(Capability::AuditWrite);
    }
    capabilities
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

fn high_assurance_disclosure_from_validated_controls(
    decision: ApprovalDecisionKind,
    controls: &[HighAssuranceApprovalControl],
    supplied_reference_count: usize,
) -> Result<WorkReportHighAssuranceApprovalDisclosure, WorkflowOsError> {
    let first_control = controls.first().ok_or_else(|| {
        executor_error(
            WorkflowOsErrorKind::Validation,
            "high_assurance_approval.disclosure_integration.controls.required",
            "high-assurance approval disclosure integration requires at least one control",
        )
    })?;
    let requester_approver_rule = first_control.requester_approver_rule();
    let expiration_policy = first_control.expiration_policy();
    let revocation_policy = first_control.revocation_policy();
    let denial_behavior = first_control.denial_behavior();

    if controls.iter().skip(1).any(|control| {
        control.requester_approver_rule() != requester_approver_rule
            || control.expiration_policy() != expiration_policy
            || control.revocation_policy() != revocation_policy
            || control.denial_behavior() != denial_behavior
    }) {
        return Err(executor_error(
            WorkflowOsErrorKind::Validation,
            "high_assurance_approval.disclosure_integration.control_posture_conflict",
            "high-assurance approval disclosure integration cannot summarize conflicting control posture",
        ));
    }

    let required_reference_count = controls
        .iter()
        .flat_map(HighAssuranceApprovalControl::required_references)
        .filter(|reference| reference.required())
        .count();
    discover_high_assurance_approval_disclosure(HighAssuranceApprovalDisclosureDiscoveryInput {
        validation_used: true,
        validation_passed: true,
        decision,
        control_count: controls.len(),
        requester_approver_rule,
        required_reference_count,
        supplied_reference_count,
        expiration_policy,
        revocation_policy,
        denial_behavior,
    })?
    .into_disclosure()
    .ok_or_else(|| {
        executor_error(
            WorkflowOsErrorKind::Internal,
            "high_assurance_approval.disclosure_integration.disclosure_missing",
            "high-assurance approval disclosure integration did not produce disclosure",
        )
    })
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
