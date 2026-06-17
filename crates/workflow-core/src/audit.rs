use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::{
    Action, ActorId, AdapterRuntimeAuditRecord, Capability, CorrelationId, EventId, IdempotencyKey,
    PolicyDecision, SchemaVersion, SkillId, SkillVersion, SpecContentHash, StepId, Timestamp,
    WorkflowId, WorkflowOsError, WorkflowOsErrorKind, WorkflowRunEvent, WorkflowRunEventKind,
    WorkflowRunEventKindName, WorkflowRunId, WorkflowVersion,
};

/// Redaction metadata attached to audit and logging records.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RedactionMetadata {
    /// Field names whose values were redacted before recording.
    pub redacted_fields: Vec<String>,
    /// Field-level handling applied before recording.
    pub field_states: Vec<RedactionFieldState>,
}

/// Field-level redaction or reference handling.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RedactionFieldState {
    /// Audit field name.
    pub field: String,
    /// How the field was handled.
    pub disposition: RedactionDisposition,
    /// Non-secret reason for the handling.
    pub reason: String,
}

/// Redaction disposition for one audit field.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionDisposition {
    /// Field is safe metadata.
    Safe,
    /// Field value was redacted.
    Redacted,
    /// Field stores a reference or summary rather than raw payload.
    ReferenceOnly,
}

impl RedactionMetadata {
    /// Creates empty redaction metadata.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            redacted_fields: Vec::new(),
            field_states: Vec::new(),
        }
    }

    fn mark(&mut self, field: &str, disposition: RedactionDisposition, reason: &str) {
        if disposition == RedactionDisposition::Redacted
            && !self
                .redacted_fields
                .iter()
                .any(|existing| existing == field)
        {
            self.redacted_fields.push(field.to_owned());
        }
        self.field_states.push(RedactionFieldState {
            field: field.to_owned(),
            disposition,
            reason: reason.to_owned(),
        });
    }
}

/// Runtime audit event emitted from a workflow run event.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AuditEvent {
    /// Source workflow event ID.
    pub event_id: EventId,
    /// Source event timestamp.
    pub timestamp: Timestamp,
    /// Source workflow event type.
    pub event_type: WorkflowRunEventKindName,
    /// Workflow ID.
    pub workflow_id: WorkflowId,
    /// Workflow spec schema version.
    pub schema_version: SchemaVersion,
    /// Workflow version.
    pub workflow_version: WorkflowVersion,
    /// Workflow run ID.
    pub workflow_run_id: WorkflowRunId,
    /// Workflow spec content hash.
    pub spec_hash: SpecContentHash,
    /// Step ID where relevant.
    pub step_id: Option<StepId>,
    /// Skill ID where relevant.
    pub skill_id: Option<SkillId>,
    /// Skill version where available to the runtime path.
    pub skill_version: Option<SkillVersion>,
    /// Actor or system actor.
    pub actor: Option<ActorId>,
    /// Runtime action associated with the event.
    pub action: Option<Action>,
    /// Non-secret decision context summary.
    pub decision_context: Option<String>,
    /// Non-secret input reference.
    pub input_reference: Option<String>,
    /// Non-secret output reference.
    pub output_reference: Option<String>,
    /// Source policy decision event ID when this audit event records a policy decision.
    pub policy_decision_reference: Option<EventId>,
    /// Correlation ID.
    pub correlation_id: Option<CorrelationId>,
    /// Idempotency key where relevant.
    pub idempotency_key: Option<IdempotencyKey>,
    /// Redaction metadata.
    pub redaction: RedactionMetadata,
    /// Runtime component that emitted the event.
    pub source_component: String,
}

/// Scope of a durable policy audit record.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyAuditScope {
    /// Policy was evaluated before a workflow run event stream existed.
    PreRun,
    /// Policy was evaluated for an existing workflow run event stream.
    Run,
}

/// Durable audit record for one policy decision.
///
/// Pre-run decisions intentionally live outside the workflow run event stream
/// so denied starts do not create misleading workflow runs.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PolicyAuditRecord {
    /// Stable audit record ID.
    pub audit_id: EventId,
    /// Decision timestamp.
    pub timestamp: Timestamp,
    /// Pre-run or run-scoped policy decision.
    pub scope: PolicyAuditScope,
    /// Workflow event ID when the decision is also represented by a run event.
    pub workflow_event_id: Option<EventId>,
    /// Evaluated action.
    pub action: Action,
    /// Required capabilities.
    pub capabilities: Vec<Capability>,
    /// Whether the decision allowed the action.
    pub allowed: bool,
    /// Whether human approval is required.
    pub requires_approval: bool,
    /// Stable reason codes.
    pub reason_codes: Vec<String>,
    /// Non-secret violation summaries.
    pub violations: Vec<String>,
    /// Actor or system actor evaluated by policy.
    pub actor: Option<ActorId>,
    /// Workflow ID where available.
    pub workflow_id: Option<WorkflowId>,
    /// Workflow spec schema version where available.
    pub schema_version: Option<SchemaVersion>,
    /// Workflow version where available.
    pub workflow_version: Option<WorkflowVersion>,
    /// Workflow run ID or pending run ID where available.
    pub workflow_run_id: Option<WorkflowRunId>,
    /// Workflow spec hash where available.
    pub spec_hash: Option<SpecContentHash>,
    /// Step ID for step-scoped decisions.
    pub step_id: Option<StepId>,
    /// Skill ID for skill-scoped decisions.
    pub skill_id: Option<SkillId>,
    /// Correlation ID.
    pub correlation_id: Option<CorrelationId>,
    /// Idempotency key where relevant.
    pub idempotency_key: Option<IdempotencyKey>,
    /// Redaction metadata for policy audit fields.
    pub redaction: RedactionMetadata,
    /// Non-secret policy context summary.
    pub policy_context: String,
    /// Runtime component that emitted the record.
    pub source_component: String,
}

impl AuditEvent {
    /// Builds an audit event from a workflow run event.
    #[must_use]
    pub fn from_workflow_event(event: &WorkflowRunEvent, source_component: &str) -> Self {
        let mut redaction = RedactionMetadata::empty();
        let output_reference = output_reference(event).map(|value| {
            let redacted = redact_sensitive_text(&value);
            if redacted == value {
                redaction.mark(
                    "output_reference",
                    RedactionDisposition::ReferenceOnly,
                    "audit stores output references instead of raw payloads",
                );
            } else {
                redaction.mark(
                    "output_reference",
                    RedactionDisposition::Redacted,
                    "sensitive-looking output reference was redacted",
                );
            }
            redacted
        });
        let input_reference = input_reference(event);
        if input_reference.is_some() {
            redaction.mark(
                "input_reference",
                RedactionDisposition::ReferenceOnly,
                "audit stores input references instead of raw payloads",
            );
        }
        if hook_payload(event).is_some() {
            redaction.mark(
                "hook_context",
                RedactionDisposition::ReferenceOnly,
                "audit projects hook workflow events as bounded references and status vocabulary",
            );
        }
        let decision_context = decision_context(event).map(|value| {
            let redacted = redact_sensitive_text(&value);
            if redacted == value {
                redaction.mark(
                    "decision_context",
                    RedactionDisposition::Safe,
                    "decision context is a non-secret summary",
                );
            } else {
                redaction.mark(
                    "decision_context",
                    RedactionDisposition::Redacted,
                    "sensitive-looking decision context was redacted",
                );
            }
            redacted
        });

        Self {
            event_id: event.event_id.clone(),
            timestamp: event.timestamp,
            event_type: event.kind(),
            workflow_id: event.workflow_id.clone(),
            schema_version: event.schema_version.clone(),
            workflow_version: event.workflow_version.clone(),
            workflow_run_id: event.run_id.clone(),
            spec_hash: event.spec_content_hash.clone(),
            step_id: step_id(event),
            skill_id: skill_id(event),
            skill_version: skill_version(event),
            actor: event.actor.clone(),
            action: action_for_event(event),
            decision_context,
            input_reference,
            output_reference,
            policy_decision_reference: policy_decision_reference(event),
            correlation_id: event.correlation_id.clone(),
            idempotency_key: event.idempotency_key.clone(),
            redaction,
            source_component: source_component.to_owned(),
        }
    }
}

/// Structured log record suitable for local runtime logs.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StructuredLogRecord {
    /// Log timestamp.
    pub timestamp: Timestamp,
    /// Log level.
    pub level: String,
    /// Non-secret message.
    pub message: String,
    /// Correlation ID.
    pub correlation_id: Option<CorrelationId>,
    /// Non-secret fields.
    pub fields: BTreeMap<String, String>,
    /// Redaction metadata.
    pub redaction: RedactionMetadata,
    /// Source component.
    pub source_component: String,
}

/// Pluggable audit sink.
pub trait AuditSink {
    /// Records one audit event.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the sink cannot durably or locally
    /// accept the audit record.
    fn record_audit_event(&self, event: &AuditEvent) -> Result<(), WorkflowOsError>;

    /// Records one policy audit record.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the sink cannot accept the record.
    fn record_policy_audit_record(&self, record: &PolicyAuditRecord)
        -> Result<(), WorkflowOsError>;

    /// Records one adapter runtime audit telemetry record.
    ///
    /// Default implementations may ignore this preview-only record type.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the sink cannot accept the record.
    fn record_adapter_audit_record(
        &self,
        _record: &AdapterRuntimeAuditRecord,
    ) -> Result<(), WorkflowOsError> {
        Ok(())
    }
}

/// Pluggable structured logger.
pub trait StructuredLogger {
    /// Records one structured log.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the logger cannot accept the record.
    fn record_log(&self, record: &StructuredLogRecord) -> Result<(), WorkflowOsError>;
}

/// Local in-process audit sink for development and tests.
#[derive(Clone, Debug, Default)]
pub struct LocalAuditSink {
    events: Arc<Mutex<Vec<AuditEvent>>>,
    policy_records: Arc<Mutex<Vec<PolicyAuditRecord>>>,
    adapter_records: Arc<Mutex<Vec<AdapterRuntimeAuditRecord>>>,
}

impl LocalAuditSink {
    /// Creates an empty local audit sink.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns recorded audit events.
    #[must_use]
    pub fn events(&self) -> Vec<AuditEvent> {
        self.events
            .lock()
            .map_or_else(|_| Vec::new(), |events| events.clone())
    }

    /// Returns recorded policy audit records.
    #[must_use]
    pub fn policy_records(&self) -> Vec<PolicyAuditRecord> {
        self.policy_records
            .lock()
            .map_or_else(|_| Vec::new(), |records| records.clone())
    }

    /// Returns recorded adapter runtime audit telemetry records.
    #[must_use]
    pub fn adapter_records(&self) -> Vec<AdapterRuntimeAuditRecord> {
        self.adapter_records
            .lock()
            .map_or_else(|_| Vec::new(), |records| records.clone())
    }
}

impl AuditSink for LocalAuditSink {
    fn record_audit_event(&self, event: &AuditEvent) -> Result<(), WorkflowOsError> {
        self.events
            .lock()
            .map_err(|_| audit_error("audit.local.lock", "local audit sink lock is poisoned"))?
            .push(event.clone());
        Ok(())
    }

    fn record_policy_audit_record(
        &self,
        record: &PolicyAuditRecord,
    ) -> Result<(), WorkflowOsError> {
        self.policy_records
            .lock()
            .map_err(|_| audit_error("audit.local.lock", "local audit sink lock is poisoned"))?
            .push(record.clone());
        Ok(())
    }

    fn record_adapter_audit_record(
        &self,
        record: &AdapterRuntimeAuditRecord,
    ) -> Result<(), WorkflowOsError> {
        self.adapter_records
            .lock()
            .map_err(|_| audit_error("audit.local.lock", "local audit sink lock is poisoned"))?
            .push(record.clone());
        Ok(())
    }
}

/// Local in-process structured logger for development and tests.
#[derive(Clone, Debug, Default)]
pub struct LocalStructuredLogger {
    records: Arc<Mutex<Vec<StructuredLogRecord>>>,
}

impl LocalStructuredLogger {
    /// Creates an empty local structured logger.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns recorded structured logs.
    #[must_use]
    pub fn records(&self) -> Vec<StructuredLogRecord> {
        self.records
            .lock()
            .map_or_else(|_| Vec::new(), |records| records.clone())
    }
}

impl StructuredLogger for LocalStructuredLogger {
    fn record_log(&self, record: &StructuredLogRecord) -> Result<(), WorkflowOsError> {
        self.records
            .lock()
            .map_err(|_| audit_error("log.local.lock", "local structured log lock is poisoned"))?
            .push(record.clone());
        Ok(())
    }
}

/// Audit sink that deterministically fails, intended for tests.
#[derive(Clone, Debug, Default)]
pub struct FailingAuditSink;

impl AuditSink for FailingAuditSink {
    fn record_audit_event(&self, _event: &AuditEvent) -> Result<(), WorkflowOsError> {
        Err(audit_error(
            "audit.sink.failed",
            "audit sink rejected the audit event",
        ))
    }

    fn record_policy_audit_record(
        &self,
        _record: &PolicyAuditRecord,
    ) -> Result<(), WorkflowOsError> {
        Err(audit_error(
            "audit.sink.failed",
            "audit sink rejected the policy audit record",
        ))
    }

    fn record_adapter_audit_record(
        &self,
        _record: &AdapterRuntimeAuditRecord,
    ) -> Result<(), WorkflowOsError> {
        Err(audit_error(
            "audit.sink.failed",
            "audit sink rejected the adapter audit record",
        ))
    }
}

fn action_for_event(event: &WorkflowRunEvent) -> Option<Action> {
    match &event.kind {
        WorkflowRunEventKind::RunCreated { .. } => Some(Action::StartWorkflow),
        WorkflowRunEventKind::ApprovalRequested(_) => Some(Action::RequestApproval),
        WorkflowRunEventKind::ApprovalGranted(_) | WorkflowRunEventKind::RunResumed => {
            Some(Action::ResumeWorkflow)
        }
        WorkflowRunEventKind::SkillInvocationRequested(_)
        | WorkflowRunEventKind::SkillInvocationStarted(_)
        | WorkflowRunEventKind::SkillInvocationSucceeded { .. }
        | WorkflowRunEventKind::SkillInvocationFailed { .. } => Some(Action::InvokeSkill),
        WorkflowRunEventKind::RunCanceled(_) => Some(Action::CancelWorkflow),
        WorkflowRunEventKind::PolicyDecisionRecorded(decision) => Some(decision.action.clone()),
        _ => None,
    }
}

fn step_id(event: &WorkflowRunEvent) -> Option<StepId> {
    match &event.kind {
        WorkflowRunEventKind::StepScheduled { step_id }
        | WorkflowRunEventKind::SkillInvocationSucceeded { step_id, .. }
        | WorkflowRunEventKind::SkillInvocationFailed { step_id, .. } => Some(step_id.clone()),
        WorkflowRunEventKind::SkillInvocationRequested(invocation) => {
            Some(invocation.step_id.clone())
        }
        WorkflowRunEventKind::SkillInvocationStarted(attempt) => Some(attempt.step_id.clone()),
        WorkflowRunEventKind::ApprovalRequested(request) => Some(request.step_id.clone()),
        WorkflowRunEventKind::RetryScheduled(record)
        | WorkflowRunEventKind::RetryStarted(record)
        | WorkflowRunEventKind::RetryExhausted(record) => record.step_id.clone(),
        WorkflowRunEventKind::EscalationTriggered(record) => record.step_id.clone(),
        WorkflowRunEventKind::HookInvocationRequested(payload)
        | WorkflowRunEventKind::HookInvocationEvaluated(payload) => payload.step_id().cloned(),
        _ => None,
    }
}

fn skill_id(event: &WorkflowRunEvent) -> Option<SkillId> {
    match &event.kind {
        WorkflowRunEventKind::SkillInvocationRequested(invocation) => {
            Some(invocation.skill_id.clone())
        }
        WorkflowRunEventKind::SkillInvocationStarted(attempt) => Some(attempt.skill_id.clone()),
        WorkflowRunEventKind::SkillInvocationSucceeded { skill_id, .. }
        | WorkflowRunEventKind::SkillInvocationFailed { skill_id, .. } => Some(skill_id.clone()),
        WorkflowRunEventKind::ApprovalRequested(request) => Some(request.skill_id.clone()),
        WorkflowRunEventKind::RetryScheduled(record)
        | WorkflowRunEventKind::RetryStarted(record)
        | WorkflowRunEventKind::RetryExhausted(record) => record.skill_id.clone(),
        WorkflowRunEventKind::EscalationTriggered(record) => record.skill_id.clone(),
        _ => None,
    }
}

fn skill_version(event: &WorkflowRunEvent) -> Option<SkillVersion> {
    match &event.kind {
        WorkflowRunEventKind::SkillInvocationRequested(invocation) => {
            Some(invocation.skill_version.clone())
        }
        WorkflowRunEventKind::SkillInvocationStarted(attempt) => {
            Some(attempt.skill_version.clone())
        }
        WorkflowRunEventKind::SkillInvocationSucceeded { skill_version, .. }
        | WorkflowRunEventKind::SkillInvocationFailed { skill_version, .. } => {
            Some(skill_version.clone())
        }
        WorkflowRunEventKind::ApprovalRequested(request) => Some(request.skill_version.clone()),
        WorkflowRunEventKind::RetryScheduled(record)
        | WorkflowRunEventKind::RetryStarted(record)
        | WorkflowRunEventKind::RetryExhausted(record) => record.skill_version.clone(),
        WorkflowRunEventKind::EscalationTriggered(record) => record.skill_version.clone(),
        _ => None,
    }
}

fn input_reference(event: &WorkflowRunEvent) -> Option<String> {
    match &event.kind {
        WorkflowRunEventKind::SkillInvocationRequested(invocation) => {
            Some(format!("invocation-input:{}", invocation.invocation_id))
        }
        WorkflowRunEventKind::HookInvocationRequested(payload)
        | WorkflowRunEventKind::HookInvocationEvaluated(payload)
            if payload.input_reference_count() > 0 =>
        {
            Some(format!(
                "hook-input-reference-count:{}",
                payload.input_reference_count()
            ))
        }
        _ => None,
    }
}

fn output_reference(event: &WorkflowRunEvent) -> Option<String> {
    match &event.kind {
        WorkflowRunEventKind::SkillInvocationSucceeded { output_ref, .. } => output_ref.clone(),
        WorkflowRunEventKind::HookInvocationRequested(payload)
        | WorkflowRunEventKind::HookInvocationEvaluated(payload)
            if payload.output_reference_count() > 0 =>
        {
            Some(format!(
                "hook-output-reference-count:{}",
                payload.output_reference_count()
            ))
        }
        _ => None,
    }
}

fn policy_decision_reference(event: &WorkflowRunEvent) -> Option<EventId> {
    if matches!(event.kind, WorkflowRunEventKind::PolicyDecisionRecorded(_)) {
        Some(event.event_id.clone())
    } else {
        None
    }
}

fn decision_context(event: &WorkflowRunEvent) -> Option<String> {
    match &event.kind {
        WorkflowRunEventKind::PolicyDecisionRecorded(decision) => Some(policy_summary(decision)),
        WorkflowRunEventKind::ApprovalRequested(request) => {
            Some(format!("approval requested: {}", request.reason))
        }
        WorkflowRunEventKind::ApprovalGranted(decision)
        | WorkflowRunEventKind::ApprovalDenied(decision) => {
            Some(format!("approval decision: {}", decision.reason))
        }
        WorkflowRunEventKind::RetryScheduled(record)
        | WorkflowRunEventKind::RetryStarted(record)
        | WorkflowRunEventKind::RetryExhausted(record) => Some(record.reason.clone()),
        WorkflowRunEventKind::EscalationTriggered(record) => Some(record.reason.clone()),
        WorkflowRunEventKind::RunFailed(record) => Some(record.code.clone()),
        WorkflowRunEventKind::HookInvocationRequested(payload) => Some(format!(
            "hook invocation requested: status={}",
            hook_status_label(payload.status())
        )),
        WorkflowRunEventKind::HookInvocationEvaluated(payload) => Some(format!(
            "hook invocation evaluated: status={}",
            hook_status_label(payload.status())
        )),
        _ => None,
    }
}

fn hook_payload(event: &WorkflowRunEvent) -> Option<&crate::AgentHarnessHookWorkflowEvent> {
    match &event.kind {
        WorkflowRunEventKind::HookInvocationRequested(payload)
        | WorkflowRunEventKind::HookInvocationEvaluated(payload) => Some(payload),
        _ => None,
    }
}

fn hook_status_label(status: crate::AgentHarnessHookInvocationStatus) -> &'static str {
    match status {
        crate::AgentHarnessHookInvocationStatus::Passed => "passed",
        crate::AgentHarnessHookInvocationStatus::Warning => "warning",
        crate::AgentHarnessHookInvocationStatus::FailedClosed => "failed_closed",
        crate::AgentHarnessHookInvocationStatus::SkippedWithDisclosure => "skipped_with_disclosure",
        crate::AgentHarnessHookInvocationStatus::Blocked => "blocked",
    }
}

fn policy_summary(decision: &PolicyDecision) -> String {
    let outcome = if decision.allowed { "allow" } else { "deny" };
    format!(
        "{outcome}; requires_approval={}; reasons={}",
        decision.requires_approval,
        decision.reason_codes.join(",")
    )
}

fn redact_sensitive_text(value: &str) -> String {
    let lower = value.to_ascii_lowercase();
    if lower.contains("secret")
        || lower.contains("token")
        || lower.contains("password")
        || lower.contains("credential")
        || lower.contains("api_key")
    {
        "[REDACTED]".to_owned()
    } else {
        value.to_owned()
    }
}

fn audit_error(code: impl Into<String>, message: impl Into<String>) -> WorkflowOsError {
    WorkflowOsError::new(WorkflowOsErrorKind::Internal, code, message)
}
