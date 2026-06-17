use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    ActorId, AgentHarnessHookContractId, AgentHarnessHookContractVersion,
    AgentHarnessHookInvocationId, AgentHarnessHookInvocationStatus, AgentHarnessHookKind,
    CorrelationId, EventId, IdempotencyKey, PolicyDecision, RedactionMetadata, SchemaVersion,
    SkillAttemptId, SkillId, SkillInvocationId, SkillVersion, SpecContentHash, StepId, Timestamp,
    WorkReportSensitivity, WorkflowId, WorkflowOsError, WorkflowOsErrorKind, WorkflowRunId,
    WorkflowVersion,
};

const HOOK_EVENT_PHASE_ID_MAX_BYTES: usize = 128;
const HOOK_EVENT_REDACTION_FIELD_MAX_BYTES: usize = 128;
const HOOK_EVENT_REDACTION_REASON_MAX_BYTES: usize = 512;
const HOOK_EVENT_REDACTION_MAX_ENTRIES: usize = 64;
const HOOK_EVENT_REFERENCE_COUNT_MAX: u32 = 1_024;

/// Monotonic sequence number for a workflow run event stream.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "u64", into = "u64")]
pub struct EventSequenceNumber(u64);

impl EventSequenceNumber {
    /// Creates a sequence number.
    ///
    /// # Errors
    ///
    /// Returns an error when the sequence number is zero.
    pub fn new(value: u64) -> Result<Self, WorkflowOsError> {
        if value == 0 {
            return Err(WorkflowOsError::validation(
                "runtime.sequence.zero",
                "event sequence numbers must start at 1",
            ));
        }
        Ok(Self(value))
    }

    /// Returns the raw sequence number.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Returns the first event sequence number.
    #[must_use]
    pub const fn first() -> Self {
        Self(1)
    }

    /// Returns the next sequence number.
    #[must_use]
    pub const fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

impl TryFrom<u64> for EventSequenceNumber {
    type Error = WorkflowOsError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<EventSequenceNumber> for u64 {
    fn from(value: EventSequenceNumber) -> Self {
        value.0
    }
}

impl fmt::Display for EventSequenceNumber {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// Current status of a workflow run projection.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowRunStatus {
    /// Run record exists and references immutable workflow identity.
    Created,
    /// Deterministic validation has succeeded.
    Validated,
    /// Runtime may actively advance the run.
    Running,
    /// Run is paused for human approval.
    WaitingForApproval,
    /// Run is paused for an external event.
    WaitingForExternalEvent,
    /// Run is waiting for or starting a retry.
    Retrying,
    /// Run requires human or operator intervention.
    Escalated,
    /// Run completed successfully.
    Completed,
    /// Run failed terminally.
    Failed,
    /// Run was canceled terminally.
    Canceled,
}

impl WorkflowRunStatus {
    /// Returns true for terminal statuses.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Canceled)
    }
}

/// Immutable workflow identity captured when a run is created.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WorkflowRunIdentity {
    /// Workflow run ID.
    pub run_id: WorkflowRunId,
    /// Workflow definition ID.
    pub workflow_id: WorkflowId,
    /// Workflow spec schema version.
    pub schema_version: SchemaVersion,
    /// Workflow definition version.
    pub workflow_version: WorkflowVersion,
    /// Workflow spec content hash.
    pub spec_content_hash: SpecContentHash,
}

/// Event-sourced workflow run wrapper.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WorkflowRun {
    /// Current snapshot derived from events.
    pub snapshot: WorkflowRunSnapshot,
    /// Append-only event history.
    pub events: Vec<WorkflowRunEvent>,
}

impl WorkflowRun {
    /// Rehydrates a workflow run from ordered events.
    ///
    /// # Errors
    ///
    /// Returns an error for missing `RunCreated`, duplicate or non-contiguous
    /// sequence numbers, mismatched run identity, invalid transitions, or
    /// events after terminal state.
    pub fn rehydrate(events: &[WorkflowRunEvent]) -> Result<Self, WorkflowOsError> {
        let snapshot = RunRehydration::rehydrate(events)?;
        Ok(Self {
            snapshot,
            events: events.to_vec(),
        })
    }

    /// Appends one event after validating it against the current snapshot.
    ///
    /// # Errors
    ///
    /// Returns an error when the event sequence or state transition is invalid.
    pub fn append_event(&mut self, event: WorkflowRunEvent) -> Result<(), WorkflowOsError> {
        validate_next_event(&self.snapshot, &event)?;
        self.snapshot.apply(&event)?;
        self.events.push(event);
        Ok(())
    }
}

/// Current state derived from workflow run events.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WorkflowRunSnapshot {
    /// Immutable workflow run identity.
    pub identity: WorkflowRunIdentity,
    /// Current status.
    pub status: WorkflowRunStatus,
    /// Last applied event sequence.
    pub last_sequence_number: EventSequenceNumber,
    /// Last applied event ID.
    pub last_event_id: EventId,
    /// Recorded skill invocations.
    pub skill_invocations: Vec<SkillInvocation>,
    /// Approval requests.
    pub approval_requests: Vec<ApprovalRequest>,
    /// Retry records.
    pub retries: Vec<RetryRecord>,
    /// Escalation records.
    pub escalations: Vec<EscalationRecord>,
    /// Cancellation record, if canceled.
    pub cancellation: Option<CancellationRecord>,
    /// Failure record, if failed.
    pub failure: Option<FailureRecord>,
    /// Policy decisions recorded for audit.
    pub policy_decisions: Vec<PolicyDecision>,
}

impl WorkflowRunSnapshot {
    fn from_created(event: &WorkflowRunEvent, identity: WorkflowRunIdentity) -> Self {
        Self {
            identity,
            status: WorkflowRunStatus::Created,
            last_sequence_number: event.sequence_number,
            last_event_id: event.event_id.clone(),
            skill_invocations: Vec::new(),
            approval_requests: Vec::new(),
            retries: Vec::new(),
            escalations: Vec::new(),
            cancellation: None,
            failure: None,
            policy_decisions: Vec::new(),
        }
    }

    fn apply(&mut self, event: &WorkflowRunEvent) -> Result<(), WorkflowOsError> {
        let transition = StateTransition::for_event(self.status, event.kind())?;
        self.status = transition.to;
        self.last_sequence_number = event.sequence_number;
        self.last_event_id = event.event_id.clone();

        match &event.kind {
            WorkflowRunEventKind::SkillInvocationRequested(record) => {
                self.skill_invocations.push(record.clone());
            }
            WorkflowRunEventKind::SkillInvocationStarted(attempt) => {
                if let Some(invocation) = self
                    .skill_invocations
                    .iter_mut()
                    .find(|invocation| invocation.invocation_id == attempt.invocation_id)
                {
                    invocation.attempts.push(attempt.clone());
                } else {
                    self.skill_invocations.push(SkillInvocation {
                        invocation_id: attempt.invocation_id.clone(),
                        step_id: attempt.step_id.clone(),
                        skill_id: attempt.skill_id.clone(),
                        skill_version: attempt.skill_version.clone(),
                        idempotency_key: event.idempotency_key.clone(),
                        attempts: vec![attempt.clone()],
                    });
                }
            }
            WorkflowRunEventKind::ApprovalRequested(record) => {
                self.approval_requests.push(record.as_ref().clone());
            }
            WorkflowRunEventKind::ApprovalGranted(decision)
            | WorkflowRunEventKind::ApprovalDenied(decision) => {
                if let Some(request) = self
                    .approval_requests
                    .iter_mut()
                    .find(|request| request.approval_id == decision.approval_id)
                {
                    request.decision = Some(decision.clone());
                }
            }
            WorkflowRunEventKind::RetryScheduled(record)
            | WorkflowRunEventKind::RetryStarted(record)
            | WorkflowRunEventKind::RetryExhausted(record) => {
                self.retries.push(record.clone());
            }
            WorkflowRunEventKind::EscalationTriggered(record) => {
                self.escalations.push(record.clone());
            }
            WorkflowRunEventKind::RunFailed(record) => {
                self.failure = Some(record.clone());
            }
            WorkflowRunEventKind::RunCanceled(record) => {
                self.cancellation = Some(record.clone());
            }
            WorkflowRunEventKind::PolicyDecisionRecorded(decision) => {
                self.policy_decisions.push(decision.as_ref().clone());
            }
            WorkflowRunEventKind::RunCreated { .. }
            | WorkflowRunEventKind::RunValidated
            | WorkflowRunEventKind::RunStarted
            | WorkflowRunEventKind::StepScheduled { .. }
            | WorkflowRunEventKind::SkillInvocationSucceeded { .. }
            | WorkflowRunEventKind::SkillInvocationFailed { .. }
            | WorkflowRunEventKind::ExternalEventReceived { .. }
            | WorkflowRunEventKind::RunPaused { .. }
            | WorkflowRunEventKind::RunResumed
            | WorkflowRunEventKind::RunCompleted
            | WorkflowRunEventKind::HookInvocationRequested(_)
            | WorkflowRunEventKind::HookInvocationEvaluated(_) => {}
        }

        Ok(())
    }
}

/// One append-only workflow run event.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WorkflowRunEvent {
    /// Event sequence number in the run event stream.
    pub sequence_number: EventSequenceNumber,
    /// Event ID.
    pub event_id: EventId,
    /// Event timestamp.
    pub timestamp: Timestamp,
    /// Run ID.
    pub run_id: WorkflowRunId,
    /// Workflow ID.
    pub workflow_id: WorkflowId,
    /// Workflow spec schema version.
    pub schema_version: SchemaVersion,
    /// Workflow version.
    pub workflow_version: WorkflowVersion,
    /// Workflow spec content hash.
    pub spec_content_hash: SpecContentHash,
    /// Correlation ID for related work.
    pub correlation_id: Option<CorrelationId>,
    /// Actor or system actor responsible for the event.
    pub actor: Option<ActorId>,
    /// Idempotency key where relevant.
    pub idempotency_key: Option<IdempotencyKey>,
    /// Typed event payload.
    pub kind: WorkflowRunEventKind,
}

impl WorkflowRunEvent {
    /// Returns the event kind discriminator.
    #[must_use]
    pub const fn kind(&self) -> WorkflowRunEventKindName {
        self.kind.name()
    }

    /// Returns immutable workflow identity from event metadata.
    #[must_use]
    pub fn identity(&self) -> WorkflowRunIdentity {
        WorkflowRunIdentity {
            run_id: self.run_id.clone(),
            workflow_id: self.workflow_id.clone(),
            schema_version: self.schema_version.clone(),
            workflow_version: self.workflow_version.clone(),
            spec_content_hash: self.spec_content_hash.clone(),
        }
    }
}

/// Names of workflow run event kinds.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum WorkflowRunEventKindName {
    /// `RunCreated`.
    RunCreated,
    /// `RunValidated`.
    RunValidated,
    /// `RunStarted`.
    RunStarted,
    /// `StepScheduled`.
    StepScheduled,
    /// `SkillInvocationRequested`.
    SkillInvocationRequested,
    /// `SkillInvocationStarted`.
    SkillInvocationStarted,
    /// `SkillInvocationSucceeded`.
    SkillInvocationSucceeded,
    /// `SkillInvocationFailed`.
    SkillInvocationFailed,
    /// `ApprovalRequested`.
    ApprovalRequested,
    /// `ApprovalGranted`.
    ApprovalGranted,
    /// `ApprovalDenied`.
    ApprovalDenied,
    /// `RetryScheduled`.
    RetryScheduled,
    /// `RetryStarted`.
    RetryStarted,
    /// `RetryExhausted`.
    RetryExhausted,
    /// `EscalationTriggered`.
    EscalationTriggered,
    /// `ExternalEventReceived`.
    ExternalEventReceived,
    /// `RunPaused`.
    RunPaused,
    /// `RunResumed`.
    RunResumed,
    /// `RunCompleted`.
    RunCompleted,
    /// `RunFailed`.
    RunFailed,
    /// `RunCanceled`.
    RunCanceled,
    /// `PolicyDecisionRecorded`.
    PolicyDecisionRecorded,
    /// `HookInvocationRequested`.
    HookInvocationRequested,
    /// `HookInvocationEvaluated`.
    HookInvocationEvaluated,
}

/// Typed workflow run event payload.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "PascalCase")]
pub enum WorkflowRunEventKind {
    /// Run was created for an immutable workflow identity.
    RunCreated {
        /// Optional creation summary.
        summary: Option<String>,
    },
    /// Run passed deterministic validation.
    RunValidated,
    /// Run started.
    RunStarted,
    /// Step was scheduled.
    StepScheduled {
        /// Step ID.
        step_id: StepId,
    },
    /// Skill invocation was requested.
    SkillInvocationRequested(SkillInvocation),
    /// Skill invocation attempt started.
    SkillInvocationStarted(SkillInvocationAttempt),
    /// Skill invocation succeeded.
    SkillInvocationSucceeded {
        /// Invocation ID.
        invocation_id: SkillInvocationId,
        /// Step ID.
        step_id: StepId,
        /// Skill ID.
        skill_id: SkillId,
        /// Skill version.
        skill_version: SkillVersion,
        /// Output reference or summary.
        output_ref: Option<String>,
    },
    /// Skill invocation failed.
    SkillInvocationFailed {
        /// Invocation ID.
        invocation_id: SkillInvocationId,
        /// Step ID.
        step_id: StepId,
        /// Skill ID.
        skill_id: SkillId,
        /// Skill version.
        skill_version: SkillVersion,
        /// Failure record.
        failure: FailureRecord,
    },
    /// Approval was requested.
    ApprovalRequested(Box<ApprovalRequest>),
    /// Approval was granted.
    ApprovalGranted(ApprovalDecision),
    /// Approval was denied.
    ApprovalDenied(ApprovalDecision),
    /// Retry was scheduled.
    RetryScheduled(RetryRecord),
    /// Retry started.
    RetryStarted(RetryRecord),
    /// Retry budget was exhausted.
    RetryExhausted(RetryRecord),
    /// Escalation was triggered.
    EscalationTriggered(EscalationRecord),
    /// External event was received.
    ExternalEventReceived {
        /// External event key.
        external_event_key: String,
    },
    /// Run was paused.
    RunPaused {
        /// Pause target status.
        status: WorkflowRunStatus,
        /// Pause reason.
        reason: String,
    },
    /// Run was resumed.
    RunResumed,
    /// Run completed.
    RunCompleted,
    /// Run failed terminally.
    RunFailed(FailureRecord),
    /// Run was canceled terminally.
    RunCanceled(CancellationRecord),
    /// Policy decision was recorded for audit.
    PolicyDecisionRecorded(Box<PolicyDecision>),
    /// Hook invocation was requested as model-only event vocabulary.
    HookInvocationRequested(Box<AgentHarnessHookWorkflowEvent>),
    /// Hook invocation was evaluated as model-only event vocabulary.
    HookInvocationEvaluated(Box<AgentHarnessHookWorkflowEvent>),
}

impl WorkflowRunEventKind {
    const fn name(&self) -> WorkflowRunEventKindName {
        match self {
            Self::RunCreated { .. } => WorkflowRunEventKindName::RunCreated,
            Self::RunValidated => WorkflowRunEventKindName::RunValidated,
            Self::RunStarted => WorkflowRunEventKindName::RunStarted,
            Self::StepScheduled { .. } => WorkflowRunEventKindName::StepScheduled,
            Self::SkillInvocationRequested(_) => WorkflowRunEventKindName::SkillInvocationRequested,
            Self::SkillInvocationStarted(_) => WorkflowRunEventKindName::SkillInvocationStarted,
            Self::SkillInvocationSucceeded { .. } => {
                WorkflowRunEventKindName::SkillInvocationSucceeded
            }
            Self::SkillInvocationFailed { .. } => WorkflowRunEventKindName::SkillInvocationFailed,
            Self::ApprovalRequested(_) => WorkflowRunEventKindName::ApprovalRequested,
            Self::ApprovalGranted(_) => WorkflowRunEventKindName::ApprovalGranted,
            Self::ApprovalDenied(_) => WorkflowRunEventKindName::ApprovalDenied,
            Self::RetryScheduled(_) => WorkflowRunEventKindName::RetryScheduled,
            Self::RetryStarted(_) => WorkflowRunEventKindName::RetryStarted,
            Self::RetryExhausted(_) => WorkflowRunEventKindName::RetryExhausted,
            Self::EscalationTriggered(_) => WorkflowRunEventKindName::EscalationTriggered,
            Self::ExternalEventReceived { .. } => WorkflowRunEventKindName::ExternalEventReceived,
            Self::RunPaused { .. } => WorkflowRunEventKindName::RunPaused,
            Self::RunResumed => WorkflowRunEventKindName::RunResumed,
            Self::RunCompleted => WorkflowRunEventKindName::RunCompleted,
            Self::RunFailed(_) => WorkflowRunEventKindName::RunFailed,
            Self::RunCanceled(_) => WorkflowRunEventKindName::RunCanceled,
            Self::PolicyDecisionRecorded(_) => WorkflowRunEventKindName::PolicyDecisionRecorded,
            Self::HookInvocationRequested(_) => WorkflowRunEventKindName::HookInvocationRequested,
            Self::HookInvocationEvaluated(_) => WorkflowRunEventKindName::HookInvocationEvaluated,
        }
    }
}

/// Model-only workflow event payload for future agent harness hook checkpoints.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct AgentHarnessHookWorkflowEvent {
    hook_invocation_id: AgentHarnessHookInvocationId,
    contract_id: AgentHarnessHookContractId,
    contract_version: AgentHarnessHookContractVersion,
    hook_kind: AgentHarnessHookKind,
    status: AgentHarnessHookInvocationStatus,
    step_id: Option<StepId>,
    phase_id: Option<String>,
    correlation_id: Option<CorrelationId>,
    input_reference_count: u32,
    output_reference_count: u32,
    redaction: RedactionMetadata,
    sensitivity: WorkReportSensitivity,
}

/// Input fields for constructing a validated `AgentHarnessHookWorkflowEvent`.
pub struct AgentHarnessHookWorkflowEventDefinition {
    /// Stable hook invocation ID.
    pub hook_invocation_id: AgentHarnessHookInvocationId,
    /// Hook contract ID.
    pub contract_id: AgentHarnessHookContractId,
    /// Hook contract version.
    pub contract_version: AgentHarnessHookContractVersion,
    /// Hook kind.
    pub hook_kind: AgentHarnessHookKind,
    /// Hook invocation status.
    pub status: AgentHarnessHookInvocationStatus,
    /// Optional step ID.
    pub step_id: Option<StepId>,
    /// Optional phase ID.
    pub phase_id: Option<String>,
    /// Optional correlation ID.
    pub correlation_id: Option<CorrelationId>,
    /// Count of already validated input references.
    pub input_reference_count: u32,
    /// Count of already validated output references.
    pub output_reference_count: u32,
    /// Redaction metadata.
    pub redaction: RedactionMetadata,
    /// Sensitivity classification.
    pub sensitivity: WorkReportSensitivity,
}

impl AgentHarnessHookWorkflowEvent {
    /// Creates a validated model-only hook workflow event payload.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when bounded text, reference counts,
    /// or redaction metadata are invalid.
    pub fn new(
        definition: AgentHarnessHookWorkflowEventDefinition,
    ) -> Result<Self, WorkflowOsError> {
        if let Some(phase_id) = &definition.phase_id {
            validate_hook_event_phase_id(phase_id)?;
        }
        validate_hook_event_reference_count(definition.input_reference_count)?;
        validate_hook_event_reference_count(definition.output_reference_count)?;
        validate_hook_event_redaction_metadata(&definition.redaction)?;

        Ok(Self {
            hook_invocation_id: definition.hook_invocation_id,
            contract_id: definition.contract_id,
            contract_version: definition.contract_version,
            hook_kind: definition.hook_kind,
            status: definition.status,
            step_id: definition.step_id,
            phase_id: definition.phase_id,
            correlation_id: definition.correlation_id,
            input_reference_count: definition.input_reference_count,
            output_reference_count: definition.output_reference_count,
            redaction: definition.redaction,
            sensitivity: definition.sensitivity,
        })
    }

    /// Returns the hook invocation ID.
    #[must_use]
    pub const fn hook_invocation_id(&self) -> &AgentHarnessHookInvocationId {
        &self.hook_invocation_id
    }

    /// Returns the hook contract ID.
    #[must_use]
    pub const fn contract_id(&self) -> &AgentHarnessHookContractId {
        &self.contract_id
    }

    /// Returns the hook contract version.
    #[must_use]
    pub const fn contract_version(&self) -> &AgentHarnessHookContractVersion {
        &self.contract_version
    }

    /// Returns the hook kind.
    #[must_use]
    pub const fn hook_kind(&self) -> AgentHarnessHookKind {
        self.hook_kind
    }

    /// Returns the hook invocation status.
    #[must_use]
    pub const fn status(&self) -> AgentHarnessHookInvocationStatus {
        self.status
    }

    /// Returns the optional step ID.
    #[must_use]
    pub const fn step_id(&self) -> Option<&StepId> {
        self.step_id.as_ref()
    }

    /// Returns the optional phase ID.
    #[must_use]
    pub fn phase_id(&self) -> Option<&str> {
        self.phase_id.as_deref()
    }

    /// Returns the optional correlation ID.
    #[must_use]
    pub const fn correlation_id(&self) -> Option<&CorrelationId> {
        self.correlation_id.as_ref()
    }

    /// Returns the input reference count.
    #[must_use]
    pub const fn input_reference_count(&self) -> u32 {
        self.input_reference_count
    }

    /// Returns the output reference count.
    #[must_use]
    pub const fn output_reference_count(&self) -> u32 {
        self.output_reference_count
    }

    /// Returns redaction metadata.
    #[must_use]
    pub const fn redaction(&self) -> &RedactionMetadata {
        &self.redaction
    }

    /// Returns the sensitivity classification.
    #[must_use]
    pub const fn sensitivity(&self) -> WorkReportSensitivity {
        self.sensitivity
    }
}

impl fmt::Debug for AgentHarnessHookWorkflowEvent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AgentHarnessHookWorkflowEvent")
            .field("hook_invocation_id", &"[REDACTED]")
            .field("contract_id", &"[REDACTED]")
            .field("contract_version", &"[REDACTED]")
            .field("hook_kind", &self.hook_kind)
            .field("status", &self.status)
            .field("has_step_id", &self.step_id.is_some())
            .field("has_phase_id", &self.phase_id.is_some())
            .field("has_correlation_id", &self.correlation_id.is_some())
            .field("input_reference_count", &self.input_reference_count)
            .field("output_reference_count", &self.output_reference_count)
            .field("redaction", &"[REDACTED]")
            .field("sensitivity", &self.sensitivity)
            .finish()
    }
}

impl<'de> Deserialize<'de> for AgentHarnessHookWorkflowEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            hook_invocation_id: AgentHarnessHookInvocationId,
            contract_id: AgentHarnessHookContractId,
            contract_version: AgentHarnessHookContractVersion,
            hook_kind: AgentHarnessHookKind,
            status: AgentHarnessHookInvocationStatus,
            step_id: Option<StepId>,
            phase_id: Option<String>,
            correlation_id: Option<CorrelationId>,
            input_reference_count: u32,
            output_reference_count: u32,
            redaction: RedactionMetadata,
            sensitivity: WorkReportSensitivity,
        }

        let wire = Wire::deserialize(deserializer)?;
        Self::new(AgentHarnessHookWorkflowEventDefinition {
            hook_invocation_id: wire.hook_invocation_id,
            contract_id: wire.contract_id,
            contract_version: wire.contract_version,
            hook_kind: wire.hook_kind,
            status: wire.status,
            step_id: wire.step_id,
            phase_id: wire.phase_id,
            correlation_id: wire.correlation_id,
            input_reference_count: wire.input_reference_count,
            output_reference_count: wire.output_reference_count,
            redaction: wire.redaction,
            sensitivity: wire.sensitivity,
        })
        .map_err(serde::de::Error::custom)
    }
}

/// Allowed status transition created by a run event.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct StateTransition {
    /// Previous status.
    pub from: WorkflowRunStatus,
    /// New status.
    pub to: WorkflowRunStatus,
    /// Event kind causing the transition.
    pub event_kind: WorkflowRunEventKindName,
}

impl StateTransition {
    /// Determines and validates the transition for an event kind.
    ///
    /// # Errors
    ///
    /// Returns an error when the event is invalid from the current status.
    pub fn for_event(
        from: WorkflowRunStatus,
        event_kind: WorkflowRunEventKindName,
    ) -> Result<Self, WorkflowOsError> {
        if from.is_terminal() {
            return Err(invalid_transition(
                from,
                event_kind,
                "terminal states reject further mutating events",
            ));
        }

        let Some(to) = transition_target(from, event_kind) else {
            return Err(invalid_transition(
                from,
                event_kind,
                "event is not valid from current status",
            ));
        };

        Ok(Self {
            from,
            to,
            event_kind,
        })
    }
}

fn transition_target(
    from: WorkflowRunStatus,
    event_kind: WorkflowRunEventKindName,
) -> Option<WorkflowRunStatus> {
    match event_kind {
        WorkflowRunEventKindName::RunValidated if from == WorkflowRunStatus::Created => {
            Some(WorkflowRunStatus::Validated)
        }
        WorkflowRunEventKindName::RunFailed if !from.is_terminal() => {
            Some(WorkflowRunStatus::Failed)
        }
        WorkflowRunEventKindName::RunCanceled if !from.is_terminal() => {
            Some(WorkflowRunStatus::Canceled)
        }
        WorkflowRunEventKindName::PolicyDecisionRecorded if !from.is_terminal() => Some(from),
        WorkflowRunEventKindName::RunStarted if from == WorkflowRunStatus::Validated => {
            Some(WorkflowRunStatus::Running)
        }
        WorkflowRunEventKindName::RetryStarted if from == WorkflowRunStatus::Retrying => {
            Some(WorkflowRunStatus::Running)
        }
        WorkflowRunEventKindName::ApprovalGranted | WorkflowRunEventKindName::ApprovalDenied
            if from == WorkflowRunStatus::WaitingForApproval =>
        {
            Some(WorkflowRunStatus::WaitingForApproval)
        }
        WorkflowRunEventKindName::ExternalEventReceived
            if from == WorkflowRunStatus::WaitingForExternalEvent =>
        {
            Some(WorkflowRunStatus::Running)
        }
        WorkflowRunEventKindName::RunResumed
            if matches!(
                from,
                WorkflowRunStatus::WaitingForApproval
                    | WorkflowRunStatus::WaitingForExternalEvent
                    | WorkflowRunStatus::Escalated
            ) =>
        {
            Some(WorkflowRunStatus::Running)
        }
        WorkflowRunEventKindName::StepScheduled
        | WorkflowRunEventKindName::SkillInvocationRequested
        | WorkflowRunEventKindName::SkillInvocationStarted
        | WorkflowRunEventKindName::SkillInvocationSucceeded
        | WorkflowRunEventKindName::SkillInvocationFailed
        | WorkflowRunEventKindName::HookInvocationRequested
        | WorkflowRunEventKindName::HookInvocationEvaluated
            if from == WorkflowRunStatus::Running =>
        {
            Some(WorkflowRunStatus::Running)
        }
        WorkflowRunEventKindName::ApprovalRequested if from == WorkflowRunStatus::Running => {
            Some(WorkflowRunStatus::WaitingForApproval)
        }
        WorkflowRunEventKindName::EscalationTriggered
            if matches!(
                from,
                WorkflowRunStatus::Running
                    | WorkflowRunStatus::WaitingForApproval
                    | WorkflowRunStatus::WaitingForExternalEvent
                    | WorkflowRunStatus::Retrying
            ) =>
        {
            Some(WorkflowRunStatus::Escalated)
        }
        WorkflowRunEventKindName::RetryExhausted
            if matches!(
                from,
                WorkflowRunStatus::Running | WorkflowRunStatus::Retrying
            ) =>
        {
            Some(from)
        }
        WorkflowRunEventKindName::RetryScheduled if from == WorkflowRunStatus::Running => {
            Some(WorkflowRunStatus::Retrying)
        }
        WorkflowRunEventKindName::RunPaused if from == WorkflowRunStatus::Running => {
            Some(WorkflowRunStatus::WaitingForExternalEvent)
        }
        WorkflowRunEventKindName::RunCompleted if from == WorkflowRunStatus::Running => {
            Some(WorkflowRunStatus::Completed)
        }
        _ => None,
    }
}

/// Logical skill invocation record.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SkillInvocation {
    /// Invocation ID.
    pub invocation_id: SkillInvocationId,
    /// Step ID.
    pub step_id: StepId,
    /// Skill ID.
    pub skill_id: SkillId,
    /// Skill version.
    pub skill_version: SkillVersion,
    /// Idempotency key for the invocation.
    pub idempotency_key: Option<IdempotencyKey>,
    /// Invocation attempts.
    pub attempts: Vec<SkillInvocationAttempt>,
}

/// One attempt of a skill invocation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SkillInvocationAttempt {
    /// Invocation ID.
    pub invocation_id: SkillInvocationId,
    /// Attempt ID.
    pub attempt_id: SkillAttemptId,
    /// Step ID.
    pub step_id: StepId,
    /// Skill ID.
    pub skill_id: SkillId,
    /// Skill version.
    pub skill_version: SkillVersion,
    /// Attempt number, starting at 1.
    pub attempt_number: u32,
}

/// Approval request record.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ApprovalRequest {
    /// Approval ID.
    pub approval_id: String,
    /// Workflow run ID.
    pub run_id: WorkflowRunId,
    /// Workflow ID.
    pub workflow_id: WorkflowId,
    /// Workflow spec schema version.
    pub schema_version: SchemaVersion,
    /// Workflow version.
    pub workflow_version: WorkflowVersion,
    /// Workflow spec content hash.
    pub spec_content_hash: SpecContentHash,
    /// Step ID requiring approval.
    pub step_id: StepId,
    /// Skill ID gated by the approval.
    pub skill_id: SkillId,
    /// Skill version gated by the approval.
    pub skill_version: SkillVersion,
    /// Actor or system actor that requested approval.
    pub requested_by: ActorId,
    /// Correlation ID for the approval request.
    pub correlation_id: CorrelationId,
    /// Idempotency key relevant to the gated skill invocation, when available.
    pub idempotency_key: Option<IdempotencyKey>,
    /// Human-readable reason.
    pub reason: String,
    /// Request creation timestamp.
    pub requested_at: Timestamp,
    /// Optional human-authored expiration duration from the workflow spec.
    pub expires_after: Option<String>,
    /// Optional concrete expiration timestamp when known.
    pub expires_at: Option<Timestamp>,
    /// Current decision, if any.
    pub decision: Option<ApprovalDecision>,
}

/// Approval decision kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalDecisionKind {
    /// Approval was granted.
    Granted,
    /// Approval was denied.
    Denied,
}

/// Approval decision record.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ApprovalDecision {
    /// Approval ID.
    pub approval_id: String,
    /// Deciding actor.
    pub actor: ActorId,
    /// Decision timestamp.
    pub decided_at: Timestamp,
    /// Decision kind.
    pub decision: ApprovalDecisionKind,
    /// Human-readable non-secret reason.
    pub reason: String,
    /// Correlation ID for the decision operation.
    pub correlation_id: CorrelationId,
}

/// Retry record.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RetryRecord {
    /// Step ID being retried, if step-scoped.
    pub step_id: Option<StepId>,
    /// Skill ID being retried, if skill-scoped.
    pub skill_id: Option<SkillId>,
    /// Skill version being retried, if skill-scoped.
    pub skill_version: Option<SkillVersion>,
    /// Invocation ID being retried, if known.
    pub invocation_id: Option<SkillInvocationId>,
    /// Attempt number.
    pub attempt_number: u32,
    /// Maximum attempts.
    pub max_attempts: u32,
    /// Retry reason.
    pub reason: String,
    /// Last failure code or summary.
    pub last_error: Option<String>,
    /// Failure class used for retry and escalation decisions.
    pub failure_class: FailureClass,
    /// Suggested next action if retry does not succeed.
    pub suggested_next_action: String,
}

/// Escalation record.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EscalationRecord {
    /// Escalation ID.
    pub escalation_id: String,
    /// Workflow run ID.
    pub run_id: WorkflowRunId,
    /// Step ID that caused escalation, if step-scoped.
    pub step_id: Option<StepId>,
    /// Skill ID that caused escalation, if skill-scoped.
    pub skill_id: Option<SkillId>,
    /// Skill version that caused escalation, if skill-scoped.
    pub skill_version: Option<SkillVersion>,
    /// Attempts made before escalation.
    pub attempts: u32,
    /// Last failure code or summary.
    pub last_error: String,
    /// Failure class used for the escalation decision.
    pub failure_class: FailureClass,
    /// Suggested next action for a human operator.
    pub suggested_next_action: String,
    /// Escalation reason.
    pub reason: String,
    /// Optional escalation contact.
    pub contact: Option<ActorId>,
}

/// Cancellation record.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CancellationRecord {
    /// Workflow run ID.
    pub run_id: WorkflowRunId,
    /// Cancellation reason.
    pub reason: String,
    /// Actor that requested cancellation.
    pub actor: ActorId,
    /// Cancellation timestamp.
    pub canceled_at: Timestamp,
    /// Correlation ID for the cancellation operation.
    pub correlation_id: CorrelationId,
}

/// Runtime failure class.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureClass {
    /// Transient failure that may be retried when policy allows.
    Transient,
    /// Permanent failure that should not be retried automatically.
    Permanent,
    /// Policy denied or unsafe action.
    PolicyDenied,
    /// Timeout-related failure.
    Timeout,
    /// Cancellation-related failure.
    Canceled,
    /// Unknown failure class.
    #[default]
    Unknown,
}

/// Failure record.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FailureRecord {
    /// Stable failure code.
    pub code: String,
    /// Failure summary.
    pub message: String,
    /// Failure class.
    pub failure_class: FailureClass,
}

/// Rehydrates run state from events.
pub struct RunRehydration;

impl RunRehydration {
    /// Rehydrates a snapshot from ordered events.
    ///
    /// # Errors
    ///
    /// Returns an error when the stream is missing `RunCreated`, has duplicate
    /// or non-contiguous sequence numbers, mismatched immutable identity, or
    /// invalid state transitions.
    pub fn rehydrate(events: &[WorkflowRunEvent]) -> Result<WorkflowRunSnapshot, WorkflowOsError> {
        let first = events.first().ok_or_else(|| {
            WorkflowOsError::invalid_state(
                "runtime.events.empty",
                "event stream must contain RunCreated",
            )
        })?;
        if first.sequence_number != EventSequenceNumber::first() {
            return Err(WorkflowOsError::invalid_state(
                "runtime.sequence.first",
                "first event sequence number must be 1",
            ));
        }
        let WorkflowRunEventKind::RunCreated { .. } = &first.kind else {
            return Err(WorkflowOsError::invalid_state(
                "runtime.run_created.missing",
                "first event must be RunCreated",
            ));
        };

        let identity = first.identity();
        let mut snapshot = WorkflowRunSnapshot::from_created(first, identity);
        let mut seen_sequences = BTreeSet::new();
        seen_sequences.insert(first.sequence_number);

        for event in events.iter().skip(1) {
            if !seen_sequences.insert(event.sequence_number) {
                return Err(WorkflowOsError::invalid_state(
                    "runtime.sequence.duplicate",
                    format!("duplicate event sequence number {}", event.sequence_number),
                ));
            }
            validate_next_event(&snapshot, event)?;
            snapshot.apply(event)?;
        }

        Ok(snapshot)
    }
}

fn validate_next_event(
    snapshot: &WorkflowRunSnapshot,
    event: &WorkflowRunEvent,
) -> Result<(), WorkflowOsError> {
    if event.sequence_number != snapshot.last_sequence_number.next() {
        return Err(WorkflowOsError::invalid_state(
            "runtime.sequence.non_contiguous",
            format!(
                "expected event sequence {}, got {}",
                snapshot.last_sequence_number.next(),
                event.sequence_number
            ),
        ));
    }
    if event.identity() != snapshot.identity {
        return Err(WorkflowOsError::invalid_state(
            "runtime.identity.mismatch",
            "event workflow identity does not match run identity",
        ));
    }
    if matches!(event.kind, WorkflowRunEventKind::RunCreated { .. }) {
        return Err(WorkflowOsError::invalid_state(
            "runtime.run_created.duplicate",
            "RunCreated may only appear as the first event",
        ));
    }
    if event.kind_requires_idempotency_key() && event.idempotency_key.is_none() {
        return Err(WorkflowOsError::invalid_state(
            "runtime.idempotency_key.missing",
            "event requires idempotency key",
        ));
    }
    StateTransition::for_event(snapshot.status, event.kind())?;
    Ok(())
}

impl WorkflowRunEvent {
    fn kind_requires_idempotency_key(&self) -> bool {
        matches!(
            self.kind,
            WorkflowRunEventKind::SkillInvocationRequested(_)
                | WorkflowRunEventKind::SkillInvocationStarted(_)
                | WorkflowRunEventKind::SkillInvocationSucceeded { .. }
                | WorkflowRunEventKind::SkillInvocationFailed { .. }
                | WorkflowRunEventKind::RetryScheduled(_)
                | WorkflowRunEventKind::RetryStarted(_)
                | WorkflowRunEventKind::RetryExhausted(_)
                | WorkflowRunEventKind::HookInvocationRequested(_)
                | WorkflowRunEventKind::HookInvocationEvaluated(_)
        )
    }
}

fn invalid_transition(
    from: WorkflowRunStatus,
    event_kind: WorkflowRunEventKindName,
    message: &'static str,
) -> WorkflowOsError {
    WorkflowOsError::new(
        WorkflowOsErrorKind::InvalidState,
        "runtime.transition.invalid",
        format!("{message}: {event_kind:?} from {from:?}"),
    )
}

fn validate_hook_event_phase_id(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(WorkflowOsError::validation(
            "runtime.hook_event.phase_id.empty",
            "hook workflow event phase ID cannot be empty",
        ));
    }
    if value.len() > HOOK_EVENT_PHASE_ID_MAX_BYTES {
        return Err(WorkflowOsError::validation(
            "runtime.hook_event.phase_id.too_long",
            format!(
                "hook workflow event phase ID cannot exceed {HOOK_EVENT_PHASE_ID_MAX_BYTES} bytes"
            ),
        ));
    }
    validate_hook_event_not_secret_like("runtime.hook_event.phase_id.secret_like", value)
}

fn validate_hook_event_reference_count(value: u32) -> Result<(), WorkflowOsError> {
    if value > HOOK_EVENT_REFERENCE_COUNT_MAX {
        return Err(WorkflowOsError::validation(
            "runtime.hook_event.reference_count.too_large",
            "hook workflow event reference count exceeds the supported bound",
        ));
    }
    Ok(())
}

fn validate_hook_event_redaction_metadata(
    redaction: &RedactionMetadata,
) -> Result<(), WorkflowOsError> {
    if redaction.redacted_fields.len() > HOOK_EVENT_REDACTION_MAX_ENTRIES {
        return Err(WorkflowOsError::validation(
            "runtime.hook_event.redaction.too_many_fields",
            "hook workflow event redaction metadata contains too many fields",
        ));
    }
    if redaction.field_states.len() > HOOK_EVENT_REDACTION_MAX_ENTRIES {
        return Err(WorkflowOsError::validation(
            "runtime.hook_event.redaction.too_many_states",
            "hook workflow event redaction metadata contains too many field states",
        ));
    }
    for field in &redaction.redacted_fields {
        validate_hook_event_redaction_field(field)?;
    }
    for state in &redaction.field_states {
        validate_hook_event_redaction_field(&state.field)?;
        validate_hook_event_redaction_reason(&state.reason)?;
    }
    Ok(())
}

fn validate_hook_event_redaction_field(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(WorkflowOsError::validation(
            "runtime.hook_event.redaction.field.empty",
            "hook workflow event redaction field cannot be empty",
        ));
    }
    if value.len() > HOOK_EVENT_REDACTION_FIELD_MAX_BYTES {
        return Err(WorkflowOsError::validation(
            "runtime.hook_event.redaction.field.too_long",
            format!(
                "hook workflow event redaction field cannot exceed {HOOK_EVENT_REDACTION_FIELD_MAX_BYTES} bytes"
            ),
        ));
    }
    validate_hook_event_not_secret_like("runtime.hook_event.redaction.field.secret_like", value)
}

fn validate_hook_event_redaction_reason(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(WorkflowOsError::validation(
            "runtime.hook_event.redaction.reason.empty",
            "hook workflow event redaction reason cannot be empty",
        ));
    }
    if value.len() > HOOK_EVENT_REDACTION_REASON_MAX_BYTES {
        return Err(WorkflowOsError::validation(
            "runtime.hook_event.redaction.reason.too_long",
            format!(
                "hook workflow event redaction reason cannot exceed {HOOK_EVENT_REDACTION_REASON_MAX_BYTES} bytes"
            ),
        ));
    }
    validate_hook_event_not_secret_like("runtime.hook_event.redaction.reason.secret_like", value)
}

fn validate_hook_event_not_secret_like(
    code: &'static str,
    value: &str,
) -> Result<(), WorkflowOsError> {
    let lowercase = value.to_ascii_lowercase();
    if lowercase.contains("secret")
        || lowercase.contains("token")
        || lowercase.contains("authorization")
        || lowercase.contains("bearer")
        || lowercase.contains("private_key")
        || lowercase.contains("password")
    {
        return Err(WorkflowOsError::validation(
            code,
            "hook workflow event metadata must not contain secret-like values",
        ));
    }
    Ok(())
}
