use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::{
    AdapterRuntimeObservabilityRecord, BackendHealthCheck, CorrelationId, EventId, PolicyDecision,
    Timestamp, WorkflowId, WorkflowOsError, WorkflowOsErrorKind, WorkflowRunEvent,
    WorkflowRunEventKind, WorkflowRunEventKindName, WorkflowRunId,
};

/// Observability signal kind emitted by the v0 runtime.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObservabilityEventKind {
    /// Workflow run started.
    WorkflowRunStarted,
    /// Workflow run completed.
    WorkflowRunCompleted,
    /// Workflow run failed.
    WorkflowRunFailed,
    /// Workflow run canceled.
    WorkflowRunCanceled,
    /// Workflow latency was observed.
    WorkflowLatency,
    /// Skill invocation latency was observed.
    SkillInvocationLatency,
    /// Skill invocation succeeded.
    SkillInvocationSucceeded,
    /// Skill invocation failed.
    SkillInvocationFailed,
    /// Retry was started.
    RetryCount,
    /// Retry budget was exhausted.
    RetryExhaustionCount,
    /// Escalation was triggered.
    EscalationCount,
    /// Approval was requested.
    ApprovalRequested,
    /// Approval was granted.
    ApprovalGranted,
    /// Approval was denied.
    ApprovalDenied,
    /// Approval wait duration was observed.
    ApprovalWaitDuration,
    /// Policy allowed an action.
    PolicyAllowed,
    /// Policy denied an action.
    PolicyDenied,
    /// Policy required approval.
    PolicyApprovalRequired,
    /// Stuck workflow detection hook fired.
    StuckWorkflowDetection,
    /// Backend health check result.
    BackendHealthCheck,
    /// Runtime error was observed.
    RuntimeErrorCount,
}

/// One local observability event.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ObservabilityEvent {
    /// Source workflow event ID when available.
    pub event_id: Option<EventId>,
    /// Signal timestamp.
    pub timestamp: Timestamp,
    /// Signal kind.
    pub kind: ObservabilityEventKind,
    /// Source workflow event type when available.
    pub workflow_event_type: Option<WorkflowRunEventKindName>,
    /// Workflow ID.
    pub workflow_id: Option<WorkflowId>,
    /// Workflow run ID.
    pub workflow_run_id: Option<WorkflowRunId>,
    /// Correlation ID.
    pub correlation_id: Option<CorrelationId>,
    /// Numeric value for counts and durations.
    pub value: Option<u64>,
    /// Non-secret attributes.
    pub attributes: BTreeMap<String, String>,
    /// Source component.
    pub source_component: String,
}

impl ObservabilityEvent {
    /// Builds observability events from one workflow run event.
    #[must_use]
    pub fn from_workflow_event(
        event: &WorkflowRunEvent,
        source_component: &str,
    ) -> Vec<ObservabilityEvent> {
        let Some(kind) = kind_for_event(event) else {
            return Vec::new();
        };
        let mut events = vec![base_event(event, source_component, kind)];
        if let WorkflowRunEventKind::PolicyDecisionRecorded(decision) = &event.kind {
            if decision.requires_approval {
                events.push(base_event(
                    event,
                    source_component,
                    ObservabilityEventKind::PolicyApprovalRequired,
                ));
            }
        }
        events
    }

    /// Creates a backend health check observability event.
    #[must_use]
    pub fn backend_health(check: &BackendHealthCheck, source_component: &str) -> Self {
        let mut attributes = BTreeMap::new();
        attributes.insert("backend".to_owned(), check.backend.clone());
        attributes.insert("healthy".to_owned(), check.healthy.to_string());
        attributes.insert("message".to_owned(), check.message.clone());
        Self {
            event_id: None,
            timestamp: Timestamp::now_utc(),
            kind: ObservabilityEventKind::BackendHealthCheck,
            workflow_event_type: None,
            workflow_id: None,
            workflow_run_id: None,
            correlation_id: None,
            value: Some(u64::from(check.healthy)),
            attributes,
            source_component: source_component.to_owned(),
        }
    }

    /// Creates a stuck workflow detection hook event.
    #[must_use]
    pub fn stuck_workflow_hook(
        run_id: WorkflowRunId,
        workflow_id: WorkflowId,
        correlation_id: Option<CorrelationId>,
        source_component: &str,
    ) -> Self {
        Self {
            event_id: None,
            timestamp: Timestamp::now_utc(),
            kind: ObservabilityEventKind::StuckWorkflowDetection,
            workflow_event_type: None,
            workflow_id: Some(workflow_id),
            workflow_run_id: Some(run_id),
            correlation_id,
            value: Some(1),
            attributes: BTreeMap::new(),
            source_component: source_component.to_owned(),
        }
    }
}

/// Pluggable observability sink.
pub trait ObservabilitySink {
    /// Records one observability event.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the sink cannot accept the signal.
    fn record_observability_event(&self, event: &ObservabilityEvent)
        -> Result<(), WorkflowOsError>;

    /// Records one adapter runtime observability telemetry record.
    ///
    /// Default implementations may ignore this preview-only record type.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the sink cannot accept the signal.
    fn record_adapter_observability_record(
        &self,
        _event: &AdapterRuntimeObservabilityRecord,
    ) -> Result<(), WorkflowOsError> {
        Ok(())
    }
}

/// Local in-process observability sink for development and tests.
#[derive(Clone, Debug, Default)]
pub struct LocalObservabilitySink {
    events: Arc<Mutex<Vec<ObservabilityEvent>>>,
    adapter_events: Arc<Mutex<Vec<AdapterRuntimeObservabilityRecord>>>,
}

impl LocalObservabilitySink {
    /// Creates an empty local observability sink.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns recorded observability events.
    #[must_use]
    pub fn events(&self) -> Vec<ObservabilityEvent> {
        self.events
            .lock()
            .map_or_else(|_| Vec::new(), |events| events.clone())
    }

    /// Returns recorded adapter runtime observability telemetry records.
    #[must_use]
    pub fn adapter_events(&self) -> Vec<AdapterRuntimeObservabilityRecord> {
        self.adapter_events
            .lock()
            .map_or_else(|_| Vec::new(), |events| events.clone())
    }
}

impl ObservabilitySink for LocalObservabilitySink {
    fn record_observability_event(
        &self,
        event: &ObservabilityEvent,
    ) -> Result<(), WorkflowOsError> {
        self.events
            .lock()
            .map_err(|_| {
                observability_error(
                    "observability.local.lock",
                    "local observability sink lock is poisoned",
                )
            })?
            .push(event.clone());
        Ok(())
    }

    fn record_adapter_observability_record(
        &self,
        event: &AdapterRuntimeObservabilityRecord,
    ) -> Result<(), WorkflowOsError> {
        self.adapter_events
            .lock()
            .map_err(|_| {
                observability_error(
                    "observability.local.lock",
                    "local observability sink lock is poisoned",
                )
            })?
            .push(event.clone());
        Ok(())
    }
}

fn base_event(
    event: &WorkflowRunEvent,
    source_component: &str,
    kind: ObservabilityEventKind,
) -> ObservabilityEvent {
    let mut attributes = BTreeMap::new();
    if let WorkflowRunEventKind::RunFailed(failure)
    | WorkflowRunEventKind::SkillInvocationFailed { failure, .. } = &event.kind
    {
        attributes.insert("failure_code".to_owned(), failure.code.clone());
        attributes.insert(
            "failure_class".to_owned(),
            format!("{:?}", failure.failure_class),
        );
    }
    if let WorkflowRunEventKind::PolicyDecisionRecorded(decision) = &event.kind {
        attributes.insert("action".to_owned(), format!("{:?}", decision.action));
        attributes.insert("reason_codes".to_owned(), decision.reason_codes.join(","));
    }

    ObservabilityEvent {
        event_id: Some(event.event_id.clone()),
        timestamp: event.timestamp,
        kind,
        workflow_event_type: Some(event.kind()),
        workflow_id: Some(event.workflow_id.clone()),
        workflow_run_id: Some(event.run_id.clone()),
        correlation_id: event.correlation_id.clone(),
        value: Some(1),
        attributes,
        source_component: source_component.to_owned(),
    }
}

fn kind_for_event(event: &WorkflowRunEvent) -> Option<ObservabilityEventKind> {
    match &event.kind {
        WorkflowRunEventKind::RunStarted => Some(ObservabilityEventKind::WorkflowRunStarted),
        WorkflowRunEventKind::RunCompleted => Some(ObservabilityEventKind::WorkflowRunCompleted),
        WorkflowRunEventKind::RunFailed(_) => Some(ObservabilityEventKind::WorkflowRunFailed),
        WorkflowRunEventKind::RunCanceled(_) => Some(ObservabilityEventKind::WorkflowRunCanceled),
        WorkflowRunEventKind::SkillInvocationSucceeded { .. } => {
            Some(ObservabilityEventKind::SkillInvocationSucceeded)
        }
        WorkflowRunEventKind::SkillInvocationFailed { .. } => {
            Some(ObservabilityEventKind::SkillInvocationFailed)
        }
        WorkflowRunEventKind::RetryStarted(_) => Some(ObservabilityEventKind::RetryCount),
        WorkflowRunEventKind::RetryExhausted(_) => {
            Some(ObservabilityEventKind::RetryExhaustionCount)
        }
        WorkflowRunEventKind::EscalationTriggered(_) => {
            Some(ObservabilityEventKind::EscalationCount)
        }
        WorkflowRunEventKind::ApprovalRequested(_) => {
            Some(ObservabilityEventKind::ApprovalRequested)
        }
        WorkflowRunEventKind::ApprovalGranted(_) => Some(ObservabilityEventKind::ApprovalGranted),
        WorkflowRunEventKind::ApprovalDenied(_) => Some(ObservabilityEventKind::ApprovalDenied),
        WorkflowRunEventKind::PolicyDecisionRecorded(decision) => Some(policy_kind(decision)),
        _ => None,
    }
}

fn policy_kind(decision: &PolicyDecision) -> ObservabilityEventKind {
    if decision.allowed {
        ObservabilityEventKind::PolicyAllowed
    } else {
        ObservabilityEventKind::PolicyDenied
    }
}

fn observability_error(code: impl Into<String>, message: impl Into<String>) -> WorkflowOsError {
    WorkflowOsError::new(WorkflowOsErrorKind::Internal, code, message)
}
