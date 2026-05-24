#![allow(clippy::expect_used)]
//! Behavior tests for event-sourced workflow run state.

use workflow_core::{
    ActorId, ApprovalDecision, ApprovalDecisionKind, ApprovalRequest, CorrelationId,
    EscalationRecord, EventId, EventSequenceNumber, FailureClass, FailureRecord, IdempotencyKey,
    RetryRecord, RunRehydration, SkillAttemptId, SkillId, SkillInvocation, SkillInvocationAttempt,
    SkillInvocationId, SpecContentHash, StepId, Timestamp, WorkflowId, WorkflowRun,
    WorkflowRunEvent, WorkflowRunEventKind, WorkflowRunId, WorkflowRunStatus, WorkflowVersion,
};

#[derive(Clone)]
struct Fixture {
    run_id: WorkflowRunId,
    workflow_id: WorkflowId,
    workflow_version: WorkflowVersion,
    spec_hash: SpecContentHash,
}

impl Fixture {
    fn new() -> Self {
        Self {
            run_id: WorkflowRunId::new("run-test").expect("run id"),
            workflow_id: WorkflowId::new("workflow/test").expect("workflow id"),
            workflow_version: WorkflowVersion::new("v0").expect("workflow version"),
            spec_hash: SpecContentHash::from_text("workflow test spec"),
        }
    }

    fn event(&self, sequence: u64, kind: WorkflowRunEventKind) -> WorkflowRunEvent {
        WorkflowRunEvent {
            sequence_number: EventSequenceNumber::new(sequence).expect("sequence"),
            event_id: EventId::new(format!("event-{sequence}")).expect("event id"),
            timestamp: Timestamp::parse_rfc3339("2026-01-01T00:00:00Z").expect("timestamp"),
            run_id: self.run_id.clone(),
            workflow_id: self.workflow_id.clone(),
            workflow_version: self.workflow_version.clone(),
            spec_content_hash: self.spec_hash.clone(),
            correlation_id: Some(CorrelationId::new("correlation-test").expect("correlation")),
            actor: Some(ActorId::new("system").expect("actor")),
            idempotency_key: None,
            kind,
        }
    }

    fn idempotent_event(&self, sequence: u64, kind: WorkflowRunEventKind) -> WorkflowRunEvent {
        let mut event = self.event(sequence, kind);
        event.idempotency_key = Some(IdempotencyKey::new(format!("idem-{sequence}")).expect("key"));
        event
    }

    fn created(&self) -> WorkflowRunEvent {
        self.event(1, WorkflowRunEventKind::RunCreated { summary: None })
    }
}

fn base_running_events(fixture: &Fixture) -> Vec<WorkflowRunEvent> {
    vec![
        fixture.created(),
        fixture.event(2, WorkflowRunEventKind::RunValidated),
        fixture.event(3, WorkflowRunEventKind::RunStarted),
    ]
}

#[test]
fn creates_run_from_run_created() {
    let fixture = Fixture::new();

    let snapshot = RunRehydration::rehydrate(&[fixture.created()]).expect("rehydrates");

    assert_eq!(snapshot.status, WorkflowRunStatus::Created);
    assert_eq!(snapshot.identity.run_id, fixture.run_id);
}

#[test]
fn valid_transition_sequence_rehydrates() {
    let fixture = Fixture::new();
    let mut events = base_running_events(&fixture);
    events.push(fixture.event(4, WorkflowRunEventKind::RunCompleted));

    let snapshot = RunRehydration::rehydrate(&events).expect("rehydrates");

    assert_eq!(snapshot.status, WorkflowRunStatus::Completed);
    assert_eq!(snapshot.last_sequence_number.get(), 4);
}

#[test]
fn invalid_transition_is_rejected() {
    let fixture = Fixture::new();
    let events = vec![
        fixture.created(),
        fixture.event(2, WorkflowRunEventKind::RunStarted),
    ];

    let error = RunRehydration::rehydrate(&events).expect_err("invalid transition fails");

    assert_eq!(error.code(), "runtime.transition.invalid");
}

#[test]
fn terminal_state_rejects_mutation() {
    let fixture = Fixture::new();
    let mut events = base_running_events(&fixture);
    events.push(fixture.event(4, WorkflowRunEventKind::RunCompleted));
    events.push(fixture.event(
        5,
        WorkflowRunEventKind::StepScheduled {
            step_id: StepId::new("draft").expect("step"),
        },
    ));

    let error = RunRehydration::rehydrate(&events).expect_err("terminal mutation fails");

    assert_eq!(error.code(), "runtime.transition.invalid");
}

#[test]
fn rehydrates_completed_run() {
    let fixture = Fixture::new();
    let mut events = base_running_events(&fixture);
    events.push(fixture.event(4, WorkflowRunEventKind::RunCompleted));

    let run = WorkflowRun::rehydrate(&events).expect("run rehydrates");

    assert_eq!(run.snapshot.status, WorkflowRunStatus::Completed);
    assert_eq!(run.events.len(), 4);
}

#[test]
fn rehydrates_failed_run() {
    let fixture = Fixture::new();
    let mut events = base_running_events(&fixture);
    events.push(fixture.event(
        4,
        WorkflowRunEventKind::RunFailed(FailureRecord {
            code: "runtime.failure".to_owned(),
            message: "failed safely".to_owned(),
            failure_class: FailureClass::Unknown,
        }),
    ));

    let snapshot = RunRehydration::rehydrate(&events).expect("rehydrates");

    assert_eq!(snapshot.status, WorkflowRunStatus::Failed);
    assert_eq!(
        snapshot.failure.expect("failure").code,
        "runtime.failure".to_owned()
    );
}

#[test]
fn approval_pause_resume_event_sequence() {
    let fixture = Fixture::new();
    let mut events = base_running_events(&fixture);
    events.push(fixture.event(
        4,
        WorkflowRunEventKind::ApprovalRequested(Box::new(ApprovalRequest {
            approval_id: "approval-1".to_owned(),
            run_id: fixture.run_id.clone(),
            workflow_id: fixture.workflow_id.clone(),
            workflow_version: fixture.workflow_version.clone(),
            spec_content_hash: fixture.spec_hash.clone(),
            step_id: StepId::new("review").expect("step"),
            skill_id: SkillId::new("local/review").expect("skill"),
            reason: "human approval required".to_owned(),
            requested_at: Timestamp::parse_rfc3339("2026-01-01T00:00:00Z").expect("timestamp"),
            expires_after: Some("30m".to_owned()),
            expires_at: None,
            decision: None,
        })),
    ));
    events.push(fixture.event(
        5,
        WorkflowRunEventKind::ApprovalGranted(ApprovalDecision {
            approval_id: "approval-1".to_owned(),
            actor: ActorId::new("approver").expect("actor"),
            decided_at: Timestamp::parse_rfc3339("2026-01-01T00:01:00Z").expect("timestamp"),
            decision: ApprovalDecisionKind::Granted,
            reason: "approved".to_owned(),
            correlation_id: CorrelationId::new("correlation-approval").expect("correlation"),
        }),
    ));
    events.push(fixture.event(6, WorkflowRunEventKind::RunResumed));

    let snapshot = RunRehydration::rehydrate(&events).expect("rehydrates");

    assert_eq!(snapshot.status, WorkflowRunStatus::Running);
    assert!(snapshot.approval_requests[0].decision.is_some());
}

#[test]
fn retry_event_sequence() {
    let fixture = Fixture::new();
    let mut events = base_running_events(&fixture);
    events.push(fixture.idempotent_event(
        4,
        WorkflowRunEventKind::RetryScheduled(RetryRecord {
            step_id: Some(StepId::new("draft").expect("step")),
            skill_id: Some(SkillId::new("local/draft").expect("skill")),
            invocation_id: None,
            attempt_number: 2,
            max_attempts: 3,
            reason: "retryable failure".to_owned(),
            last_error: Some("runtime.transient".to_owned()),
            failure_class: FailureClass::Transient,
            suggested_next_action: "retry".to_owned(),
        }),
    ));
    events.push(fixture.idempotent_event(
        5,
        WorkflowRunEventKind::RetryStarted(RetryRecord {
            step_id: Some(StepId::new("draft").expect("step")),
            skill_id: Some(SkillId::new("local/draft").expect("skill")),
            invocation_id: None,
            attempt_number: 2,
            max_attempts: 3,
            reason: "starting retry".to_owned(),
            last_error: Some("runtime.transient".to_owned()),
            failure_class: FailureClass::Transient,
            suggested_next_action: "retry".to_owned(),
        }),
    ));

    let snapshot = RunRehydration::rehydrate(&events).expect("rehydrates");

    assert_eq!(snapshot.status, WorkflowRunStatus::Running);
    assert_eq!(snapshot.retries.len(), 2);
}

#[test]
fn escalation_event_sequence() {
    let fixture = Fixture::new();
    let mut events = base_running_events(&fixture);
    events.push(fixture.event(
        4,
        WorkflowRunEventKind::EscalationTriggered(EscalationRecord {
            escalation_id: "esc-1".to_owned(),
            run_id: fixture.run_id.clone(),
            step_id: Some(StepId::new("draft").expect("step")),
            skill_id: Some(SkillId::new("local/draft").expect("skill")),
            attempts: 3,
            last_error: "runtime.failure".to_owned(),
            failure_class: FailureClass::Unknown,
            suggested_next_action: "manual review".to_owned(),
            reason: "operator review".to_owned(),
            contact: Some(ActorId::new("ops").expect("actor")),
        }),
    ));

    let snapshot = RunRehydration::rehydrate(&events).expect("rehydrates");

    assert_eq!(snapshot.status, WorkflowRunStatus::Escalated);
    assert_eq!(snapshot.escalations.len(), 1);
}

#[test]
fn duplicate_sequence_number_is_rejected() {
    let fixture = Fixture::new();
    let events = vec![
        fixture.created(),
        fixture.event(2, WorkflowRunEventKind::RunValidated),
        fixture.event(2, WorkflowRunEventKind::RunStarted),
    ];

    let error = RunRehydration::rehydrate(&events).expect_err("duplicate sequence fails");

    assert_eq!(error.code(), "runtime.sequence.duplicate");
}

#[test]
fn missing_run_created_is_rejected() {
    let fixture = Fixture::new();
    let events = vec![fixture.event(1, WorkflowRunEventKind::RunValidated)];

    let error = RunRehydration::rehydrate(&events).expect_err("missing created fails");

    assert_eq!(error.code(), "runtime.run_created.missing");
}

#[test]
fn spec_hash_is_retained() {
    let fixture = Fixture::new();
    let snapshot = RunRehydration::rehydrate(&[fixture.created()]).expect("rehydrates");

    assert_eq!(snapshot.identity.spec_content_hash, fixture.spec_hash);
}

#[test]
fn idempotency_key_is_retained_on_relevant_events() {
    let fixture = Fixture::new();
    let mut events = base_running_events(&fixture);
    let invocation_id = SkillInvocationId::new("skill-invocation-1").expect("invocation");
    events.push(fixture.idempotent_event(
        4,
        WorkflowRunEventKind::SkillInvocationRequested(SkillInvocation {
            invocation_id: invocation_id.clone(),
            step_id: StepId::new("draft").expect("step"),
            skill_id: SkillId::new("local/draft").expect("skill"),
            idempotency_key: Some(IdempotencyKey::new("idem-4").expect("key")),
            attempts: Vec::new(),
        }),
    ));
    events.push(fixture.idempotent_event(
        5,
        WorkflowRunEventKind::SkillInvocationStarted(SkillInvocationAttempt {
            invocation_id,
            attempt_id: SkillAttemptId::new("skill-attempt-1").expect("attempt"),
            step_id: StepId::new("draft").expect("step"),
            skill_id: SkillId::new("local/draft").expect("skill"),
            attempt_number: 1,
        }),
    ));

    let snapshot = RunRehydration::rehydrate(&events).expect("rehydrates");

    assert_eq!(
        snapshot.skill_invocations[0]
            .idempotency_key
            .as_ref()
            .expect("idempotency")
            .as_str(),
        "idem-4"
    );
    assert_eq!(snapshot.skill_invocations[0].attempts.len(), 1);
}
