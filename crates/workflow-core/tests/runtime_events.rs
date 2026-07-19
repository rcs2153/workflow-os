#![allow(clippy::expect_used)]
//! Behavior tests for event-sourced workflow run state.

use workflow_core::{
    ActorId, AgentHarnessHookContractId, AgentHarnessHookContractVersion,
    AgentHarnessHookInvocationId, AgentHarnessHookInvocationStatus, AgentHarnessHookKind,
    AgentHarnessHookWorkflowEvent, AgentHarnessHookWorkflowEventDefinition, ApprovalDecision,
    ApprovalDecisionKind, ApprovalRequest, CorrelationId, EscalationRecord, EventId,
    EventSequenceNumber, FailureClass, FailureRecord, GovernanceAssessmentBinding, IdempotencyKey,
    ImmutableRunBundleId, ImmutableRunBundleVersion, RedactionDisposition, RedactionFieldState,
    RedactionMetadata, RetryRecord, RunRehydration, SchemaVersion, SideEffectId,
    SideEffectLifecycleState, SideEffectReference, SideEffectReferenceKind, SideEffectSensitivity,
    SideEffectWorkflowEvent, SideEffectWorkflowEventDefinition, SkillAttemptId, SkillId,
    SkillInvocation, SkillInvocationAttempt, SkillInvocationId, SkillVersion, SpecContentHash,
    StepId, Timestamp, WorkReportSensitivity, WorkflowId, WorkflowRun, WorkflowRunEvent,
    WorkflowRunEventKind, WorkflowRunEventKindName, WorkflowRunId, WorkflowRunStatus,
    WorkflowVersion,
};

#[derive(Clone)]
struct Fixture {
    run_id: WorkflowRunId,
    workflow_id: WorkflowId,
    schema_version: SchemaVersion,
    workflow_version: WorkflowVersion,
    spec_hash: SpecContentHash,
}

impl Fixture {
    fn new() -> Self {
        Self {
            run_id: WorkflowRunId::new("run-test").expect("run id"),
            workflow_id: WorkflowId::new("workflow/test").expect("workflow id"),
            schema_version: SchemaVersion::new("workflowos.dev/v0").expect("schema version"),
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
            schema_version: self.schema_version.clone(),
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
        self.event(
            1,
            WorkflowRunEventKind::RunCreated {
                summary: None,
                immutable_run_bundle: None,
            },
        )
    }

    fn created_with_bundle(&self) -> WorkflowRunEvent {
        let binding = governance_binding(self.run_id.as_str(), self.workflow_id.as_str());
        self.event(
            1,
            WorkflowRunEventKind::RunCreated {
                summary: None,
                immutable_run_bundle: Some(binding.immutable_run_bundle().clone()),
            },
        )
    }
}

fn governance_binding(run_id: &str, workflow_id: &str) -> GovernanceAssessmentBinding {
    serde_json::from_value(serde_json::json!({
        "binding_version": "v1",
        "assessment_set_algorithm": "v1",
        "workflow_id": workflow_id,
        "run_id": run_id,
        "immutable_run_bundle": {
            "bundle_id": "bundle/run-test",
            "bundle_version": "v1",
            "root_hash": SpecContentHash::from_text("bundle root").as_str(),
        },
        "aggregate_fingerprint": SpecContentHash::from_text("assessment set").as_str(),
        "step_count": 2,
        "execution": "proceed",
        "disclosure": "quiet",
        "completeness": "complete",
    }))
    .expect("binding fixture")
}

fn base_running_events(fixture: &Fixture) -> Vec<WorkflowRunEvent> {
    vec![
        fixture.created(),
        fixture.event(2, WorkflowRunEventKind::RunValidated),
        fixture.event(3, WorkflowRunEventKind::RunStarted),
    ]
}

#[test]
fn legacy_run_created_without_bundle_binding_remains_readable() {
    let fixture = Fixture::new();
    let value = serde_json::to_value(fixture.created()).expect("event serializes");

    assert!(value["kind"].get("immutable_run_bundle").is_none());
    let event = serde_json::from_value::<WorkflowRunEvent>(value).expect("legacy event reads");
    let run = WorkflowRun::rehydrate(&[event]).expect("legacy run rehydrates");

    assert!(run.snapshot.identity.immutable_run_bundle.is_none());
}

#[test]
fn run_created_bundle_binding_round_trips_into_run_identity() {
    let fixture = Fixture::new();
    let mut value = serde_json::to_value(fixture.created()).expect("event serializes");
    value["kind"]["immutable_run_bundle"] = serde_json::json!({
        "bundle_id": String::from(
            ImmutableRunBundleId::new("bundle/run-test").expect("bundle id")
        ),
        "bundle_version": String::from(
            ImmutableRunBundleVersion::new("v1").expect("bundle version")
        ),
        "root_hash": SpecContentHash::from_text("bundle root").as_str(),
    });

    let event = serde_json::from_value::<WorkflowRunEvent>(value).expect("bound event reads");
    let run = WorkflowRun::rehydrate(&[event]).expect("bound run rehydrates");
    let binding = run
        .snapshot
        .identity
        .immutable_run_bundle
        .expect("bundle binding");

    assert_eq!(binding.bundle_id().as_str(), "bundle/run-test");
    assert_eq!(binding.bundle_version().as_str(), "v1");
    assert_eq!(
        binding.root_hash(),
        &SpecContentHash::from_text("bundle root")
    );
}

#[test]
fn governance_assessment_binding_is_recorded_before_validation() {
    let fixture = Fixture::new();
    let binding = governance_binding(fixture.run_id.as_str(), fixture.workflow_id.as_str());
    let events = vec![
        fixture.created_with_bundle(),
        fixture.idempotent_event(
            2,
            WorkflowRunEventKind::GovernanceAssessmentBound(Box::new(binding.clone())),
        ),
        fixture.event(3, WorkflowRunEventKind::RunValidated),
    ];

    let run = WorkflowRun::rehydrate(&events).expect("bound run rehydrates");

    assert_eq!(run.snapshot.status, WorkflowRunStatus::Validated);
    assert_eq!(run.snapshot.governance_assessment_binding, Some(binding));
    assert_eq!(
        events[1].kind(),
        WorkflowRunEventKindName::GovernanceAssessmentBound
    );
}

#[test]
fn governance_assessment_binding_requires_idempotency_and_exact_identity() {
    let fixture = Fixture::new();
    let binding = governance_binding(fixture.run_id.as_str(), fixture.workflow_id.as_str());
    let missing_key = vec![
        fixture.created_with_bundle(),
        fixture.event(
            2,
            WorkflowRunEventKind::GovernanceAssessmentBound(Box::new(binding.clone())),
        ),
    ];
    assert_eq!(
        WorkflowRun::rehydrate(&missing_key)
            .expect_err("idempotency required")
            .code(),
        "runtime.idempotency_key.missing"
    );

    let mismatched = governance_binding("run-other", fixture.workflow_id.as_str());
    let identity_mismatch = vec![
        fixture.created_with_bundle(),
        fixture.idempotent_event(
            2,
            WorkflowRunEventKind::GovernanceAssessmentBound(Box::new(mismatched)),
        ),
    ];
    assert_eq!(
        WorkflowRun::rehydrate(&identity_mismatch)
            .expect_err("identity mismatch rejected")
            .code(),
        "runtime.governance_assessment_binding.identity_mismatch"
    );

    let duplicate = vec![
        fixture.created_with_bundle(),
        fixture.idempotent_event(
            2,
            WorkflowRunEventKind::GovernanceAssessmentBound(Box::new(binding.clone())),
        ),
        fixture.idempotent_event(
            3,
            WorkflowRunEventKind::GovernanceAssessmentBound(Box::new(binding)),
        ),
    ];
    assert_eq!(
        WorkflowRun::rehydrate(&duplicate)
            .expect_err("duplicate rejected")
            .code(),
        "runtime.governance_assessment_binding.duplicate"
    );
}

#[test]
fn legacy_snapshot_without_governance_binding_remains_readable() {
    let fixture = Fixture::new();
    let run = WorkflowRun::rehydrate(&[fixture.created()]).expect("run rehydrates");
    let mut value = serde_json::to_value(&run.snapshot).expect("snapshot serializes");
    value
        .as_object_mut()
        .expect("snapshot object")
        .remove("governance_assessment_binding");

    let snapshot = serde_json::from_value::<workflow_core::WorkflowRunSnapshot>(value)
        .expect("legacy snapshot reads");

    assert!(snapshot.governance_assessment_binding.is_none());
}

fn hook_event_payload(status: AgentHarnessHookInvocationStatus) -> AgentHarnessHookWorkflowEvent {
    AgentHarnessHookWorkflowEvent::new(hook_event_definition(status)).expect("hook event payload")
}

fn hook_event_definition(
    status: AgentHarnessHookInvocationStatus,
) -> AgentHarnessHookWorkflowEventDefinition {
    AgentHarnessHookWorkflowEventDefinition {
        hook_invocation_id: AgentHarnessHookInvocationId::new("hook-invocation/runtime-event")
            .expect("hook invocation id"),
        contract_id: AgentHarnessHookContractId::new("agent-harness/hooks/runtime")
            .expect("contract id"),
        contract_version: AgentHarnessHookContractVersion::new("v1").expect("version"),
        hook_kind: AgentHarnessHookKind::BeforeValidation,
        status,
        step_id: Some(StepId::new("validate").expect("step")),
        phase_id: Some("validation".to_owned()),
        correlation_id: Some(CorrelationId::new("correlation-hook").expect("correlation")),
        input_reference_count: 1,
        output_reference_count: 1,
        redaction: RedactionMetadata {
            redacted_fields: vec!["hook_context".to_owned()],
            field_states: vec![RedactionFieldState {
                field: "hook_context".to_owned(),
                disposition: RedactionDisposition::ReferenceOnly,
                reason: "hook event stores stable references only".to_owned(),
            }],
        },
        sensitivity: WorkReportSensitivity::Confidential,
    }
}

fn side_effect_event_payload(state: SideEffectLifecycleState) -> SideEffectWorkflowEvent {
    SideEffectWorkflowEvent::new(side_effect_event_definition(state))
        .expect("side-effect event payload")
}

fn side_effect_event_definition(
    state: SideEffectLifecycleState,
) -> SideEffectWorkflowEventDefinition {
    SideEffectWorkflowEventDefinition {
        side_effect_id: SideEffectId::new("side-effect/runtime-event").expect("side-effect id"),
        lifecycle_state: state,
        step_id: Some(StepId::new("write-boundary").expect("step")),
        skill_id: Some(SkillId::new("local/write-boundary").expect("skill")),
        skill_version: Some(SkillVersion::new("v0").expect("skill version")),
        correlation_id: Some(CorrelationId::new("correlation-side-effect").expect("correlation")),
        references: vec![
            SideEffectReference::new(SideEffectReferenceKind::PolicyDecision, "event/policy-1")
                .expect("policy reference"),
            SideEffectReference::new(
                SideEffectReferenceKind::AdapterTelemetry,
                "adapter-telemetry/1",
            )
            .expect("adapter reference"),
        ],
        evidence_reference_count: 1,
        outcome_reference_count: 1,
        redaction: RedactionMetadata {
            redacted_fields: vec!["side_effect_context".to_owned()],
            field_states: vec![RedactionFieldState {
                field: "side_effect_context".to_owned(),
                disposition: RedactionDisposition::ReferenceOnly,
                reason: "side-effect event stores stable references only".to_owned(),
            }],
        },
        sensitivity: SideEffectSensitivity::Confidential,
    }
}

#[test]
fn creates_run_from_run_created() {
    let fixture = Fixture::new();

    let snapshot = RunRehydration::rehydrate(&[fixture.created()]).expect("rehydrates");

    assert_eq!(snapshot.status, WorkflowRunStatus::Created);
    assert_eq!(snapshot.identity.run_id, fixture.run_id);
    assert_eq!(snapshot.identity.schema_version, fixture.schema_version);
    assert_eq!(snapshot.identity.workflow_version, fixture.workflow_version);
    assert_eq!(snapshot.identity.spec_content_hash, fixture.spec_hash);
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
fn hook_event_kind_names_are_representable_and_stable() {
    assert_eq!(
        serde_json::to_string(&WorkflowRunEventKindName::HookInvocationRequested)
            .expect("serializes"),
        "\"HookInvocationRequested\""
    );
    assert_eq!(
        serde_json::to_string(&WorkflowRunEventKindName::HookInvocationEvaluated)
            .expect("serializes"),
        "\"HookInvocationEvaluated\""
    );
}

#[test]
fn valid_hook_workflow_event_payload_is_bounded_and_accessible() {
    let payload = hook_event_payload(AgentHarnessHookInvocationStatus::Passed);

    assert_eq!(payload.hook_kind(), AgentHarnessHookKind::BeforeValidation);
    assert_eq!(payload.status(), AgentHarnessHookInvocationStatus::Passed);
    assert_eq!(payload.phase_id(), Some("validation"));
    assert_eq!(payload.input_reference_count(), 1);
    assert_eq!(payload.output_reference_count(), 1);
    assert_eq!(payload.sensitivity(), WorkReportSensitivity::Confidential);
}

#[test]
fn hook_workflow_event_debug_redacts_ids_context_and_redaction_metadata() {
    let payload = hook_event_payload(AgentHarnessHookInvocationStatus::Warning);
    let debug = format!("{payload:?}");

    assert!(debug.contains("AgentHarnessHookWorkflowEvent"));
    assert!(debug.contains("BeforeValidation"));
    assert!(debug.contains("Warning"));
    assert!(!debug.contains("hook-invocation/runtime-event"));
    assert!(!debug.contains("agent-harness/hooks/runtime"));
    assert!(!debug.contains("validation"));
    assert!(!debug.contains("hook_context"));
}

#[test]
fn hook_workflow_event_rejects_secret_like_phase_id_without_leaking_value() {
    let error = AgentHarnessHookWorkflowEvent::new(AgentHarnessHookWorkflowEventDefinition {
        phase_id: Some("token-secret-phase".to_owned()),
        ..hook_event_definition(AgentHarnessHookInvocationStatus::Passed)
    })
    .expect_err("secret-like phase fails");

    assert_eq!(error.code(), "runtime.hook_event.phase_id.secret_like");
    assert!(!format!("{error:?}").contains("token-secret-phase"));
}

#[test]
fn hook_workflow_event_rejects_secret_like_redaction_metadata_without_leaking_value() {
    let error = AgentHarnessHookWorkflowEvent::new(AgentHarnessHookWorkflowEventDefinition {
        redaction: RedactionMetadata {
            redacted_fields: vec!["authorization_token".to_owned()],
            field_states: Vec::new(),
        },
        ..hook_event_definition(AgentHarnessHookInvocationStatus::Passed)
    })
    .expect_err("secret-like redaction fails");

    assert_eq!(
        error.code(),
        "runtime.hook_event.redaction.field.secret_like"
    );
    assert!(!format!("{error:?}").contains("authorization_token"));
}

#[test]
fn hook_workflow_event_serialization_does_not_include_raw_payload_markers() {
    let payload = hook_event_payload(AgentHarnessHookInvocationStatus::Passed);
    let serialized = serde_json::to_string(&payload).expect("serializes");

    assert!(serialized.contains("hook_invocation_id"));
    assert!(!serialized.contains("raw provider payload"));
    assert!(!serialized.contains("raw command output"));
    assert!(!serialized.contains("parser payload"));
    assert!(!serialized.contains("authorization"));
    assert!(!serialized.contains("private_key"));
}

#[test]
fn invalid_serialized_hook_workflow_event_fails_closed_without_leaking_value() {
    let mut value =
        serde_json::to_value(hook_event_payload(AgentHarnessHookInvocationStatus::Passed))
            .expect("payload serializes");
    value["redaction"]["field_states"][0]["reason"] = serde_json::json!("contains bearer token");

    let error = serde_json::from_value::<AgentHarnessHookWorkflowEvent>(value)
        .expect_err("invalid redaction reason fails");

    assert!(error
        .to_string()
        .contains("runtime.hook_event.redaction.reason.secret_like"));
    assert!(!error.to_string().contains("bearer token"));
}

#[test]
fn hook_events_rehydrate_as_state_preserving_from_running() {
    let fixture = Fixture::new();
    let mut events = base_running_events(&fixture);
    events.push(fixture.idempotent_event(
        4,
        WorkflowRunEventKind::HookInvocationRequested(Box::new(hook_event_payload(
            AgentHarnessHookInvocationStatus::Passed,
        ))),
    ));
    events.push(fixture.idempotent_event(
        5,
        WorkflowRunEventKind::HookInvocationEvaluated(Box::new(hook_event_payload(
            AgentHarnessHookInvocationStatus::Passed,
        ))),
    ));

    let snapshot = RunRehydration::rehydrate(&events).expect("rehydrates");

    assert_eq!(snapshot.status, WorkflowRunStatus::Running);
    assert_eq!(snapshot.last_sequence_number.get(), 5);
    assert!(snapshot.skill_invocations.is_empty());
    assert!(snapshot.approval_requests.is_empty());
    assert!(snapshot.policy_decisions.is_empty());
}

#[test]
fn hook_events_require_idempotency_key() {
    let fixture = Fixture::new();
    let mut events = base_running_events(&fixture);
    events.push(fixture.event(
        4,
        WorkflowRunEventKind::HookInvocationRequested(Box::new(hook_event_payload(
            AgentHarnessHookInvocationStatus::Passed,
        ))),
    ));

    let error = RunRehydration::rehydrate(&events).expect_err("missing idempotency fails");

    assert_eq!(error.code(), "runtime.idempotency_key.missing");
}

#[test]
fn terminal_state_rejects_hook_events() {
    let fixture = Fixture::new();
    let mut events = base_running_events(&fixture);
    events.push(fixture.event(4, WorkflowRunEventKind::RunCompleted));
    events.push(fixture.idempotent_event(
        5,
        WorkflowRunEventKind::HookInvocationEvaluated(Box::new(hook_event_payload(
            AgentHarnessHookInvocationStatus::Passed,
        ))),
    ));

    let error = RunRehydration::rehydrate(&events).expect_err("terminal hook event fails");

    assert_eq!(error.code(), "runtime.transition.invalid");
}

#[test]
fn hook_events_are_rejected_before_running() {
    let fixture = Fixture::new();
    let events = vec![
        fixture.created(),
        fixture.idempotent_event(
            2,
            WorkflowRunEventKind::HookInvocationRequested(Box::new(hook_event_payload(
                AgentHarnessHookInvocationStatus::Passed,
            ))),
        ),
    ];

    let error = RunRehydration::rehydrate(&events).expect_err("created hook event fails");

    assert_eq!(error.code(), "runtime.transition.invalid");
}

#[test]
fn side_effect_event_kind_names_are_representable_and_stable() {
    assert_eq!(
        serde_json::to_string(&WorkflowRunEventKindName::SideEffectProposed).expect("serializes"),
        "\"SideEffectProposed\""
    );
    assert_eq!(
        serde_json::to_string(&WorkflowRunEventKindName::SideEffectDenied).expect("serializes"),
        "\"SideEffectDenied\""
    );
    assert_eq!(
        serde_json::to_string(&WorkflowRunEventKindName::SideEffectSkipped).expect("serializes"),
        "\"SideEffectSkipped\""
    );
    assert_eq!(
        serde_json::to_string(&WorkflowRunEventKindName::SideEffectAttempted).expect("serializes"),
        "\"SideEffectAttempted\""
    );
    assert_eq!(
        serde_json::to_string(&WorkflowRunEventKindName::SideEffectCompleted).expect("serializes"),
        "\"SideEffectCompleted\""
    );
    assert_eq!(
        serde_json::to_string(&WorkflowRunEventKindName::SideEffectFailed).expect("serializes"),
        "\"SideEffectFailed\""
    );
}

#[test]
fn valid_side_effect_workflow_event_payload_is_bounded_and_accessible() {
    let payload = side_effect_event_payload(SideEffectLifecycleState::Proposed);

    assert_eq!(
        payload.lifecycle_state(),
        SideEffectLifecycleState::Proposed
    );
    assert_eq!(
        payload.step_id(),
        Some(&StepId::new("write-boundary").expect("step"))
    );
    assert_eq!(
        payload.skill_id(),
        Some(&SkillId::new("local/write-boundary").expect("skill"))
    );
    assert_eq!(payload.references().len(), 2);
    assert_eq!(payload.evidence_reference_count(), 1);
    assert_eq!(payload.outcome_reference_count(), 1);
    assert_eq!(payload.sensitivity(), SideEffectSensitivity::Confidential);
}

#[test]
fn side_effect_workflow_event_debug_redacts_ids_references_and_redaction_metadata() {
    let payload = side_effect_event_payload(SideEffectLifecycleState::Completed);
    let debug = format!("{payload:?}");

    assert!(debug.contains("SideEffectWorkflowEvent"));
    assert!(debug.contains("Completed"));
    assert!(!debug.contains("side-effect/runtime-event"));
    assert!(!debug.contains("write-boundary"));
    assert!(!debug.contains("local/write-boundary"));
    assert!(!debug.contains("event/policy-1"));
    assert!(!debug.contains("adapter-telemetry/1"));
    assert!(!debug.contains("side_effect_context"));
}

#[test]
fn serialized_side_effect_workflow_event_rejects_secret_like_id_without_leaking_value() {
    let mut value = serde_json::to_value(side_effect_event_payload(
        SideEffectLifecycleState::Proposed,
    ))
    .expect("payload serializes");
    value["side_effect_id"] = serde_json::json!("side-effect/token-secret");

    let error = serde_json::from_value::<SideEffectWorkflowEvent>(value)
        .expect_err("secret-like side-effect id fails");

    assert!(!error.to_string().contains("token-secret"));
}

#[test]
fn side_effect_workflow_event_rejects_secret_like_redaction_metadata_without_leaking_value() {
    let error = SideEffectWorkflowEvent::new(SideEffectWorkflowEventDefinition {
        redaction: RedactionMetadata {
            redacted_fields: vec!["authorization_token".to_owned()],
            field_states: Vec::new(),
        },
        ..side_effect_event_definition(SideEffectLifecycleState::Proposed)
    })
    .expect_err("secret-like redaction fails");

    assert_eq!(
        error.code(),
        "runtime.side_effect_event.redaction.field.secret_like"
    );
    assert!(!format!("{error:?}").contains("authorization_token"));
}

#[test]
fn side_effect_workflow_event_serialization_does_not_include_raw_payload_markers() {
    let payload = side_effect_event_payload(SideEffectLifecycleState::Proposed);
    let serialized = serde_json::to_string(&payload).expect("serializes");

    assert!(serialized.contains("side_effect_id"));
    assert!(!serialized.contains("target"));
    assert!(!serialized.contains("authority"));
    assert!(!serialized.contains("reason_codes"));
    assert!(!serialized.contains("raw provider payload"));
    assert!(!serialized.contains("raw command output"));
    assert!(!serialized.contains("parser payload"));
    assert!(!serialized.contains("authorization"));
    assert!(!serialized.contains("private_key"));
}

#[test]
fn invalid_serialized_side_effect_workflow_event_fails_closed_without_leaking_value() {
    let mut value = serde_json::to_value(side_effect_event_payload(
        SideEffectLifecycleState::Proposed,
    ))
    .expect("payload serializes");
    value["redaction"]["field_states"][0]["reason"] = serde_json::json!("contains bearer token");

    let error = serde_json::from_value::<SideEffectWorkflowEvent>(value)
        .expect_err("invalid redaction reason fails");

    assert!(error
        .to_string()
        .contains("runtime.side_effect_event.redaction.reason.secret_like"));
    assert!(!error.to_string().contains("bearer token"));
}

#[test]
fn side_effect_events_rehydrate_as_state_preserving_from_running() {
    let fixture = Fixture::new();
    let mut events = base_running_events(&fixture);
    events.push(fixture.idempotent_event(
        4,
        WorkflowRunEventKind::SideEffectProposed(Box::new(side_effect_event_payload(
            SideEffectLifecycleState::Proposed,
        ))),
    ));
    events.push(fixture.idempotent_event(
        5,
        WorkflowRunEventKind::SideEffectAttempted(Box::new(side_effect_event_payload(
            SideEffectLifecycleState::Attempted,
        ))),
    ));
    events.push(fixture.idempotent_event(
        6,
        WorkflowRunEventKind::SideEffectCompleted(Box::new(side_effect_event_payload(
            SideEffectLifecycleState::Completed,
        ))),
    ));

    let snapshot = RunRehydration::rehydrate(&events).expect("rehydrates");

    assert_eq!(snapshot.status, WorkflowRunStatus::Running);
    assert_eq!(snapshot.last_sequence_number.get(), 6);
    assert!(snapshot.skill_invocations.is_empty());
    assert!(snapshot.approval_requests.is_empty());
    assert!(snapshot.policy_decisions.is_empty());
}

#[test]
fn side_effect_event_kind_must_match_payload_lifecycle() {
    let fixture = Fixture::new();
    let mut events = base_running_events(&fixture);
    events.push(fixture.idempotent_event(
        4,
        WorkflowRunEventKind::SideEffectCompleted(Box::new(side_effect_event_payload(
            SideEffectLifecycleState::Failed,
        ))),
    ));

    let error = RunRehydration::rehydrate(&events).expect_err("mismatch fails");

    assert_eq!(error.code(), "runtime.side_effect_event.lifecycle.mismatch");
    assert!(!format!("{error:?}").contains("side-effect/runtime-event"));
}

#[test]
fn all_side_effect_events_require_idempotency_key() {
    let fixture = Fixture::new();
    for kind in [
        WorkflowRunEventKind::SideEffectProposed(Box::new(side_effect_event_payload(
            SideEffectLifecycleState::Proposed,
        ))),
        WorkflowRunEventKind::SideEffectDenied(Box::new(side_effect_event_payload(
            SideEffectLifecycleState::Denied,
        ))),
        WorkflowRunEventKind::SideEffectSkipped(Box::new(side_effect_event_payload(
            SideEffectLifecycleState::Skipped,
        ))),
        WorkflowRunEventKind::SideEffectAttempted(Box::new(side_effect_event_payload(
            SideEffectLifecycleState::Attempted,
        ))),
        WorkflowRunEventKind::SideEffectCompleted(Box::new(side_effect_event_payload(
            SideEffectLifecycleState::Completed,
        ))),
        WorkflowRunEventKind::SideEffectFailed(Box::new(side_effect_event_payload(
            SideEffectLifecycleState::Failed,
        ))),
    ] {
        let mut events = base_running_events(&fixture);
        events.push(fixture.event(4, kind));

        let error = RunRehydration::rehydrate(&events)
            .expect_err("missing idempotency fails for side-effect event");

        assert_eq!(error.code(), "runtime.idempotency_key.missing");
    }
}

#[test]
fn terminal_state_allows_completed_and_failed_side_effect_outcome_events() {
    let fixture = Fixture::new();
    let mut events = base_running_events(&fixture);
    events.push(fixture.event(4, WorkflowRunEventKind::RunCompleted));
    events.push(fixture.idempotent_event(
        5,
        WorkflowRunEventKind::SideEffectCompleted(Box::new(side_effect_event_payload(
            SideEffectLifecycleState::Completed,
        ))),
    ));
    events.push(fixture.idempotent_event(
        6,
        WorkflowRunEventKind::SideEffectFailed(Box::new(side_effect_event_payload(
            SideEffectLifecycleState::Failed,
        ))),
    ));

    let snapshot = RunRehydration::rehydrate(&events).expect("terminal projections rehydrate");

    assert_eq!(snapshot.status, WorkflowRunStatus::Completed);
    assert_eq!(snapshot.last_sequence_number.get(), 6);
}

#[test]
fn terminal_state_rejects_non_outcome_side_effect_events() {
    let fixture = Fixture::new();
    for kind in [
        WorkflowRunEventKind::SideEffectProposed(Box::new(side_effect_event_payload(
            SideEffectLifecycleState::Proposed,
        ))),
        WorkflowRunEventKind::SideEffectAttempted(Box::new(side_effect_event_payload(
            SideEffectLifecycleState::Attempted,
        ))),
    ] {
        let mut events = base_running_events(&fixture);
        events.push(fixture.event(4, WorkflowRunEventKind::RunCompleted));
        events.push(fixture.idempotent_event(5, kind));

        let error = RunRehydration::rehydrate(&events).expect_err("terminal event fails");

        assert_eq!(error.code(), "runtime.transition.invalid");
    }
}

#[test]
fn side_effect_events_are_rejected_before_running() {
    let fixture = Fixture::new();
    let events = vec![
        fixture.created(),
        fixture.idempotent_event(
            2,
            WorkflowRunEventKind::SideEffectProposed(Box::new(side_effect_event_payload(
                SideEffectLifecycleState::Proposed,
            ))),
        ),
    ];

    let error = RunRehydration::rehydrate(&events).expect_err("created event fails");

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
            schema_version: fixture.schema_version.clone(),
            workflow_version: fixture.workflow_version.clone(),
            spec_content_hash: fixture.spec_hash.clone(),
            resolved_execution_context_hash: None,
            step_id: StepId::new("review").expect("step"),
            skill_id: SkillId::new("local/review").expect("skill"),
            skill_version: SkillVersion::new("v0").expect("skill version"),
            requested_by: ActorId::new("system").expect("actor"),
            correlation_id: CorrelationId::new("correlation-test").expect("correlation"),
            idempotency_key: Some(IdempotencyKey::new("approval-idem").expect("key")),
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
            proof_marker: None,
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
            skill_version: Some(SkillVersion::new("v0").expect("skill version")),
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
            skill_version: Some(SkillVersion::new("v0").expect("skill version")),
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
            skill_version: Some(SkillVersion::new("v0").expect("skill version")),
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
fn all_events_retain_schema_version_identity() {
    let fixture = Fixture::new();
    let mut events = base_running_events(&fixture);
    events.push(fixture.event(4, WorkflowRunEventKind::RunCompleted));

    let snapshot = RunRehydration::rehydrate(&events).expect("rehydrates");

    assert_eq!(snapshot.identity.schema_version, fixture.schema_version);
    for event in &events {
        assert_eq!(event.schema_version, fixture.schema_version);
        assert_eq!(event.identity().schema_version, fixture.schema_version);
    }
}

#[test]
fn mismatched_schema_version_is_rejected() {
    let fixture = Fixture::new();
    let mut mismatched = fixture.event(2, WorkflowRunEventKind::RunValidated);
    mismatched.schema_version = SchemaVersion::new("workflowos.dev/v1").expect("schema version");
    let events = vec![fixture.created(), mismatched];

    let error = RunRehydration::rehydrate(&events).expect_err("schema mismatch fails");

    assert_eq!(error.code(), "runtime.identity.mismatch");
}

#[test]
fn persisted_event_without_schema_version_is_rejected() {
    let legacy_event = serde_json::json!({
        "sequence_number": 1,
        "event_id": "event-legacy",
        "timestamp": "2026-01-01T00:00:00Z",
        "run_id": "run-legacy",
        "workflow_id": "workflow/legacy",
        "workflow_version": "v0",
        "spec_content_hash": SpecContentHash::from_text("legacy").as_str(),
        "correlation_id": "correlation-legacy",
        "actor": "system",
        "idempotency_key": null,
        "kind": {
            "kind": "RunCreated",
            "summary": null
        }
    });

    let error = serde_json::from_value::<WorkflowRunEvent>(legacy_event)
        .expect_err("legacy event without schema_version is rejected");

    assert!(error.to_string().contains("schema_version"));
}

#[test]
fn approval_decision_without_proof_marker_remains_compatible() {
    let fixture = Fixture::new();
    let event = fixture.event(
        4,
        WorkflowRunEventKind::ApprovalDenied(ApprovalDecision {
            approval_id: "approval-1".to_owned(),
            actor: ActorId::new("approver").expect("actor"),
            decided_at: Timestamp::parse_rfc3339("2026-01-01T00:01:00Z").expect("timestamp"),
            decision: ApprovalDecisionKind::Denied,
            reason: "denied".to_owned(),
            correlation_id: CorrelationId::new("correlation-approval").expect("correlation"),
            proof_marker: None,
        }),
    );

    let serialized = serde_json::to_string(&event).expect("serializes");
    assert!(!serialized.contains("proof_marker"));

    let round_trip: WorkflowRunEvent = serde_json::from_str(&serialized).expect("deserializes");
    assert!(matches!(
        round_trip.kind,
        WorkflowRunEventKind::ApprovalDenied(ApprovalDecision {
            proof_marker: None,
            ..
        })
    ));
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
            skill_version: SkillVersion::new("v0").expect("skill version"),
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
            skill_version: SkillVersion::new("v0").expect("skill version"),
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
