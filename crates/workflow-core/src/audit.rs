use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::{
    Action, ActorId, CorrelationId, EventId, IdempotencyKey, PolicyDecision, SkillId, SkillVersion,
    SpecContentHash, StepId, Timestamp, WorkflowId, WorkflowOsError, WorkflowOsErrorKind,
    WorkflowRunEvent, WorkflowRunEventKind, WorkflowRunEventKindName, WorkflowRunId,
    WorkflowVersion,
};

/// Redaction metadata attached to audit and logging records.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RedactionMetadata {
    /// Field names whose values were redacted before recording.
    pub redacted_fields: Vec<String>,
}

impl RedactionMetadata {
    /// Creates empty redaction metadata.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            redacted_fields: Vec::new(),
        }
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

impl AuditEvent {
    /// Builds an audit event from a workflow run event.
    #[must_use]
    pub fn from_workflow_event(event: &WorkflowRunEvent, source_component: &str) -> Self {
        let mut redacted_fields = Vec::new();
        let output_reference = output_reference(event).map(|value| {
            let redacted = redact_sensitive_text(&value);
            if redacted != value {
                redacted_fields.push("output_reference".to_owned());
            }
            redacted
        });
        let redaction = RedactionMetadata { redacted_fields };

        Self {
            event_id: event.event_id.clone(),
            timestamp: event.timestamp,
            event_type: event.kind(),
            workflow_id: event.workflow_id.clone(),
            workflow_version: event.workflow_version.clone(),
            workflow_run_id: event.run_id.clone(),
            spec_hash: event.spec_content_hash.clone(),
            step_id: step_id(event),
            skill_id: skill_id(event),
            skill_version: None,
            actor: event.actor.clone(),
            action: action_for_event(event),
            decision_context: decision_context(event),
            input_reference: input_reference(event),
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
}

impl AuditSink for LocalAuditSink {
    fn record_audit_event(&self, event: &AuditEvent) -> Result<(), WorkflowOsError> {
        self.events
            .lock()
            .map_err(|_| audit_error("audit.local.lock", "local audit sink lock is poisoned"))?
            .push(event.clone());
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
        WorkflowRunEventKind::StepScheduled { step_id } => Some(step_id.clone()),
        WorkflowRunEventKind::SkillInvocationRequested(invocation) => {
            Some(invocation.step_id.clone())
        }
        WorkflowRunEventKind::SkillInvocationStarted(attempt) => Some(attempt.step_id.clone()),
        WorkflowRunEventKind::ApprovalRequested(request) => Some(request.step_id.clone()),
        WorkflowRunEventKind::RetryScheduled(record)
        | WorkflowRunEventKind::RetryStarted(record)
        | WorkflowRunEventKind::RetryExhausted(record) => record.step_id.clone(),
        WorkflowRunEventKind::EscalationTriggered(record) => record.step_id.clone(),
        _ => None,
    }
}

fn skill_id(event: &WorkflowRunEvent) -> Option<SkillId> {
    match &event.kind {
        WorkflowRunEventKind::SkillInvocationRequested(invocation) => {
            Some(invocation.skill_id.clone())
        }
        WorkflowRunEventKind::SkillInvocationStarted(attempt) => Some(attempt.skill_id.clone()),
        WorkflowRunEventKind::ApprovalRequested(request) => Some(request.skill_id.clone()),
        WorkflowRunEventKind::RetryScheduled(record)
        | WorkflowRunEventKind::RetryStarted(record)
        | WorkflowRunEventKind::RetryExhausted(record) => record.skill_id.clone(),
        WorkflowRunEventKind::EscalationTriggered(record) => record.skill_id.clone(),
        _ => None,
    }
}

fn input_reference(event: &WorkflowRunEvent) -> Option<String> {
    match &event.kind {
        WorkflowRunEventKind::SkillInvocationRequested(invocation) => {
            Some(format!("invocation-input:{}", invocation.invocation_id))
        }
        _ => None,
    }
}

fn output_reference(event: &WorkflowRunEvent) -> Option<String> {
    match &event.kind {
        WorkflowRunEventKind::SkillInvocationSucceeded { output_ref, .. } => output_ref.clone(),
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
        _ => None,
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
