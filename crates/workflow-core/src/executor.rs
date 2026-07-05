use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use crate::local_check::{
    DocsCheckLocalHandler, LocalCheckRegistrationMode, LocalCheckRegistrationProfile,
};
use crate::{
    derive_workflow_report_artifact_gate_policy, discover_high_assurance_approval_disclosure,
    execute_runtime_agent_harness_hook, execute_runtime_agent_harness_hook_failed_closed,
    expose_terminal_local_work_report_result,
    generate_terminal_local_work_report_with_side_effect_discovery,
    load_github_pr_comment_proposed_side_effect_event, load_project,
    validate_high_assurance_approval_decision, validate_loaded_project_with_capability,
    write_work_report_artifact_with_side_effect_integrity_and_approval_linkage, Action, ActorId,
    AdapterRuntimeAuditRecord, AdapterRuntimeObservabilityRecord, AdapterTelemetryRecord,
    AgentHarnessHookDisclosureId, AgentHarnessHookInvocationId, AgentHarnessHookInvocationInput,
    AgentHarnessHookInvocationStatus, AgentHarnessHookKind, AgentHarnessHookWorkflowEvent,
    AgentHarnessHookWorkflowEventDefinition, ApprovalDecision, ApprovalDecisionKind,
    ApprovalReferenceId, ApprovalRequest, AuditEvent, AuditSink, AutonomyLevel, CancellationRecord,
    Capability, ConservativePolicyEngine, CorrelationId, EscalationRecord, EventId,
    EventSequenceNumber, EvidenceReferenceId, FailureClass, FailureRecord,
    GitHubPullRequestCommentSideEffectEventContext, HighAssuranceApprovalControl,
    HighAssuranceApprovalDecisionValidationInput, HighAssuranceApprovalDisclosureDiscoveryInput,
    HighAssuranceApprovalSuppliedReference, IdempotencyKey, IdempotencyResult, IdempotencyWrite,
    LoadedSpec, LocalAuditSink, LocalObservabilitySink, LocalStructuredLogger, MappingExpression,
    ObservabilityEvent, ObservabilitySink, PolicyAuditRecord, PolicyAuditScope, PolicyDecision,
    PolicyEffect, PolicyEffectSet, PolicyEvaluationContext, PolicySpecDocument, ProjectBundle,
    ProjectValidationCapability, RedactionDisposition, RedactionFieldState, RedactionMetadata,
    RetryRecord, RuntimeAgentHarnessHookInput, SchemaVersion,
    SideEffectApprovalLinkageFromStoreResult, SideEffectId, SideEffectLifecycleState,
    SideEffectRecordStore, SideEffectWorkflowEvent, SkillAttemptId, SkillDefinition, SkillId,
    SkillInvocation, SkillInvocationAttempt, SkillInvocationId, SkillVersion, StateBackend,
    StepDefinition, StepId, StructuredLogRecord, StructuredLogger, TerminalBehavior,
    TerminalLocalWorkReportInput, TerminalLocalWorkReportSideEffectDiscoveryInput, TimeoutBehavior,
    Timestamp, TypedHandoffId, ValidationReferenceId, ValueMapping, WorkReport,
    WorkReportArtifactGovernedWriteInput, WorkReportArtifactHighAssuranceDisclosureGateResult,
    WorkReportArtifactHighAssuranceDisclosurePolicy, WorkReportArtifactRecord,
    WorkReportArtifactSideEffectIntegrityResult, WorkReportArtifactStore, WorkReportContractId,
    WorkReportContractVersion, WorkReportHighAssuranceApprovalDisclosure, WorkReportId,
    WorkReportSensitivity, WorkReportStableReference, WorkflowDefinition, WorkflowId,
    WorkflowOsError, WorkflowOsErrorKind, WorkflowReportArtifactGateDerivationInput, WorkflowRun,
    WorkflowRunEvent, WorkflowRunEventKind, WorkflowRunId, WorkflowRunIdentity, WorkflowRunStatus,
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

/// Explicit report artifact write policy for executor-integrated local reports.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalExecutionReportArtifactInputs {
    /// Whether every cited `SideEffect` ID must resolve to a stored record.
    pub require_all_side_effect_citations: bool,
    /// Whether `RequiresApproval` side-effect records must cite an approval request.
    pub require_approval_references_for_requires_approval: bool,
    /// Whether approved/denied side-effect records require matching approval decisions.
    pub require_decision_for_approved_or_denied: bool,
    /// Optional high-assurance approval disclosure gate policy.
    pub high_assurance_disclosure_policy: WorkReportArtifactHighAssuranceDisclosurePolicy,
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
    pub const fn new(
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
    pub const fn new(
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
        self.execute_with_validation_capability(request, ProjectValidationCapability::Default)
    }

    fn execute_with_validation_capability(
        &self,
        request: &LocalExecutionRequest,
        validation_capability: ProjectValidationCapability,
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
        };

        Ok((run, approval, decision))
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
                let plan =
                    Self::prepare_resume_execution(project_root, builder, &approval.step_id)?;
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
            ProjectValidationCapability::Default,
        )
    }

    fn prepare_execution_with_capability(
        request: &LocalExecutionRequest,
        run_id: WorkflowRunId,
        validation_capability: ProjectValidationCapability,
    ) -> Result<ExecutionPlan, WorkflowOsError> {
        let checkpoint_inputs = before_skill_invocation_checkpoint_inputs(request)?;
        let bundle = load_validated_project_bundle(&request.project_root, validation_capability)?;
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
        let workflow_report_artifact_policy = derive_workflow_report_artifact_gate_policy(
            WorkflowReportArtifactGateDerivationInput {
                workflow: &workflow.definition,
            },
        )?
        .high_assurance_disclosure_policy();

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
            workflow_report_artifact_policy,
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
        step_id: &StepId,
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
        };
        let mut plan = Self::prepare_execution(&request, builder.run_id.clone())?;
        let step_index = plan
            .steps
            .iter()
            .position(|candidate| &candidate.step.id == step_id)
            .ok_or_else(|| {
                executor_error(
                    WorkflowOsErrorKind::InvalidState,
                    "executor.workflow.multistep.resume_step_missing",
                    "approval resume step is not present in workflow definition",
                )
            })?;
        plan.set_current_step(step_index)?;
        plan.step_scheduled = true;
        plan.approval_already_granted = true;
        plan.event_builder = builder;
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
    let (run, workflow_report_artifact_policy) =
        execute_for_report_artifact_path(executor, &request.execution)?;

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

    match write_work_report_artifact_with_side_effect_integrity_and_approval_linkage(
        artifact_store,
        side_effect_store,
        WorkReportArtifactGovernedWriteInput {
            run: &run,
            artifact: &artifact,
            require_all_side_effect_citations: request.artifact.require_all_side_effect_citations,
            require_approval_references_for_requires_approval: request
                .artifact
                .require_approval_references_for_requires_approval,
            require_decision_for_approved_or_denied: request
                .artifact
                .require_decision_for_approved_or_denied,
            high_assurance_disclosure_policy: request
                .artifact
                .high_assurance_disclosure_policy
                .stricter(workflow_report_artifact_policy),
        },
    ) {
        Ok(write_result) => Ok(LocalExecutionWithReportArtifactResult::new(
            run,
            Some(work_report),
            None,
            Some(artifact),
            None,
            Some(*write_result.side_effect_integrity()),
            write_result.approval_linkage().copied(),
            write_result.high_assurance_disclosure().copied(),
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

fn execute_for_report_artifact_path<B>(
    executor: &LocalExecutor<'_, B>,
    request: &LocalExecutionRequest,
) -> Result<(WorkflowRun, WorkReportArtifactHighAssuranceDisclosurePolicy), WorkflowOsError>
where
    B: StateBackend,
{
    let run_id = request
        .run_id
        .clone()
        .unwrap_or_else(WorkflowRunId::generate);
    if executor.backend.read_events(&run_id)?.is_empty() {
        let mut plan = LocalExecutor::<B>::prepare_execution_with_capability(
            request,
            run_id,
            ProjectValidationCapability::ReportArtifactCapable {
                workflow_id: request.workflow_id.clone(),
            },
        )?;
        let policy = plan.workflow_report_artifact_policy;
        executor.evaluate_pre_run_policy(&plan, &request.actor, &request.correlation_id)?;
        executor.append_run_start(&mut plan)?;
        let run = executor.execute_steps(plan, &request.correlation_id)?;
        return Ok((run, policy));
    }

    let run = executor.backend.rehydrate_run(&run_id)?;
    let policy = workflow_report_artifact_policy_for_request(request, &run.snapshot.identity)?;
    Ok((run, policy))
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

fn workflow_report_artifact_policy_for_request(
    request: &LocalExecutionRequest,
    identity: &WorkflowRunIdentity,
) -> Result<WorkReportArtifactHighAssuranceDisclosurePolicy, WorkflowOsError> {
    let bundle = load_validated_project_bundle(
        &request.project_root,
        ProjectValidationCapability::ReportArtifactCapable {
            workflow_id: request.workflow_id.clone(),
        },
    )?;
    let workflow = find_workflow(&bundle.workflows, &request.workflow_id)?;
    validate_loaded_workflow_matches_run_identity(workflow, identity)?;
    Ok(
        derive_workflow_report_artifact_gate_policy(WorkflowReportArtifactGateDerivationInput {
            workflow: &workflow.definition,
        })?
        .high_assurance_disclosure_policy(),
    )
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
        high_assurance_approval: report.high_assurance_approval.clone(),
        typed_handoff_ids: report.typed_handoff_ids.clone(),
        agent_harness_hook_invocation_ids: report.agent_harness_hook_invocation_ids.clone(),
        agent_harness_hook_disclosure_ids: report.agent_harness_hook_disclosure_ids.clone(),
        side_effect_ids: report.side_effect_ids.clone(),
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
    workflow_report_artifact_policy: WorkReportArtifactHighAssuranceDisclosurePolicy,
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
