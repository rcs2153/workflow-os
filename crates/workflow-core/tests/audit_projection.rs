#![allow(clippy::expect_used)]
//! Behavior tests for workflow-event audit projection.

use workflow_core::{
    ActorId, AgentHarnessHookContractId, AgentHarnessHookContractVersion,
    AgentHarnessHookInvocationId, AgentHarnessHookInvocationStatus, AgentHarnessHookKind,
    AgentHarnessHookWorkflowEvent, AgentHarnessHookWorkflowEventDefinition, AuditEvent, AuditSink,
    CorrelationId, EventId, EventSequenceNumber, GovernanceAssessmentBinding, IdempotencyKey,
    LocalAuditSink, ObservabilityEvent, RedactionDisposition, RedactionFieldState,
    RedactionMetadata, SchemaVersion, SideEffectId, SideEffectLifecycleState, SideEffectReference,
    SideEffectReferenceKind, SideEffectSensitivity, SideEffectWorkflowEvent,
    SideEffectWorkflowEventDefinition, SkillId, SkillVersion, SpecContentHash, StepId, Timestamp,
    WorkReportSensitivity, WorkflowId, WorkflowRunEvent, WorkflowRunEventKind,
    WorkflowRunEventKindName, WorkflowRunId, WorkflowVersion,
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
            run_id: WorkflowRunId::new("run-audit-hook").expect("run id"),
            workflow_id: WorkflowId::new("workflow/audit-hook").expect("workflow id"),
            schema_version: SchemaVersion::new("workflowos.dev/v0").expect("schema version"),
            workflow_version: WorkflowVersion::new("v0").expect("workflow version"),
            spec_hash: SpecContentHash::from_text("workflow audit hook spec"),
        }
    }

    fn event(&self, sequence: u64, kind: WorkflowRunEventKind) -> WorkflowRunEvent {
        WorkflowRunEvent {
            sequence_number: EventSequenceNumber::new(sequence).expect("sequence"),
            event_id: EventId::new(format!("event-hook-{sequence}")).expect("event id"),
            timestamp: Timestamp::parse_rfc3339("2026-01-01T00:00:00Z").expect("timestamp"),
            run_id: self.run_id.clone(),
            workflow_id: self.workflow_id.clone(),
            schema_version: self.schema_version.clone(),
            workflow_version: self.workflow_version.clone(),
            spec_content_hash: self.spec_hash.clone(),
            correlation_id: Some(CorrelationId::new("correlation-hook").expect("correlation")),
            actor: Some(ActorId::new("system").expect("actor")),
            idempotency_key: Some(IdempotencyKey::new("idem-hook").expect("idempotency")),
            kind,
        }
    }
}

fn hook_event_payload(status: AgentHarnessHookInvocationStatus) -> AgentHarnessHookWorkflowEvent {
    AgentHarnessHookWorkflowEvent::new(AgentHarnessHookWorkflowEventDefinition {
        hook_invocation_id: AgentHarnessHookInvocationId::new("hook-invocation/audit-projection")
            .expect("hook invocation id"),
        contract_id: AgentHarnessHookContractId::new("agent-harness/hooks/audit-projection")
            .expect("contract id"),
        contract_version: AgentHarnessHookContractVersion::new("v1").expect("version"),
        hook_kind: AgentHarnessHookKind::BeforeValidation,
        status,
        step_id: Some(StepId::new("validate").expect("step")),
        phase_id: Some("operator-phase".to_owned()),
        correlation_id: Some(CorrelationId::new("correlation-hook-payload").expect("correlation")),
        input_reference_count: 2,
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
    })
    .expect("hook event payload")
}

fn side_effect_event_payload(state: SideEffectLifecycleState) -> SideEffectWorkflowEvent {
    SideEffectWorkflowEvent::new(SideEffectWorkflowEventDefinition {
        side_effect_id: SideEffectId::new("side-effect/audit-projection").expect("side-effect id"),
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
        evidence_reference_count: 2,
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
    })
    .expect("side-effect payload")
}

fn governance_binding() -> GovernanceAssessmentBinding {
    serde_json::from_value(serde_json::json!({
        "binding_version": "v1",
        "assessment_set_algorithm": "v1",
        "workflow_id": "workflow/audit-hook",
        "run_id": "run-audit-hook",
        "immutable_run_bundle": {
            "bundle_id": "bundle/audit-hook",
            "bundle_version": "v1",
            "root_hash": SpecContentHash::from_text("bundle root").as_str(),
        },
        "aggregate_fingerprint": SpecContentHash::from_text("assessment set").as_str(),
        "step_count": 2,
        "execution": "require_approval",
        "disclosure": "visible",
        "completeness": "complete",
    }))
    .expect("binding fixture")
}

#[test]
fn hook_invocation_requested_projects_to_bounded_audit_event() {
    let fixture = Fixture::new();
    let event = fixture.event(
        1,
        WorkflowRunEventKind::HookInvocationRequested(Box::new(hook_event_payload(
            AgentHarnessHookInvocationStatus::Passed,
        ))),
    );

    let audit = AuditEvent::from_workflow_event(&event, "workflow-core.test");

    assert_eq!(audit.event_id, event.event_id);
    assert_eq!(
        audit.event_type,
        WorkflowRunEventKindName::HookInvocationRequested
    );
    assert_eq!(audit.workflow_id, fixture.workflow_id);
    assert_eq!(audit.workflow_run_id, fixture.run_id);
    assert_eq!(audit.schema_version, fixture.schema_version);
    assert_eq!(audit.workflow_version, fixture.workflow_version);
    assert_eq!(audit.spec_hash, fixture.spec_hash);
    assert_eq!(audit.step_id, Some(StepId::new("validate").expect("step")));
    assert_eq!(audit.actor, event.actor);
    assert_eq!(audit.correlation_id, event.correlation_id);
    assert_eq!(audit.idempotency_key, event.idempotency_key);
    assert_eq!(audit.action, None);
    assert_eq!(audit.skill_id, None);
    assert_eq!(audit.skill_version, None);
    assert_eq!(
        audit.decision_context.as_deref(),
        Some("hook invocation requested: status=passed")
    );
    assert_eq!(
        audit.input_reference.as_deref(),
        Some("hook-input-reference-count:2")
    );
    assert_eq!(
        audit.output_reference.as_deref(),
        Some("hook-output-reference-count:1")
    );
    assert!(audit.redaction.field_states.iter().any(|state| {
        state.field == "hook_context" && state.disposition == RedactionDisposition::ReferenceOnly
    }));
}

#[test]
fn governance_assessment_binding_projects_only_bounded_posture() {
    let fixture = Fixture::new();
    let binding = governance_binding();
    let aggregate_fingerprint = binding.aggregate_fingerprint().as_str().to_owned();
    let event = fixture.event(
        1,
        WorkflowRunEventKind::GovernanceAssessmentBound(Box::new(binding)),
    );

    let audit = AuditEvent::from_workflow_event(&event, "workflow-core.test");
    let serialized = serde_json::to_string(&audit).expect("audit serializes");

    assert_eq!(
        audit.event_type,
        WorkflowRunEventKindName::GovernanceAssessmentBound
    );
    assert_eq!(
        audit.decision_context.as_deref(),
        Some(
            "governance assessment bound: execution=require_approval; disclosure=visible; completeness=complete; steps=2"
        )
    );
    assert_eq!(audit.input_reference, None);
    assert_eq!(audit.output_reference, None);
    assert!(audit.redaction.field_states.iter().any(|state| {
        state.field == "governance_assessment_binding"
            && state.disposition == RedactionDisposition::ReferenceOnly
    }));
    assert!(!serialized.contains("bundle/audit-hook"));
    assert!(!serialized.contains(&aggregate_fingerprint));
    assert!(!serialized.contains("provider_payload"));
    assert!(!serialized.contains("command_output"));
}

#[test]
fn hook_invocation_evaluated_projects_status_without_copying_hook_metadata() {
    let fixture = Fixture::new();
    let event = fixture.event(
        1,
        WorkflowRunEventKind::HookInvocationEvaluated(Box::new(hook_event_payload(
            AgentHarnessHookInvocationStatus::Warning,
        ))),
    );

    let audit = AuditEvent::from_workflow_event(&event, "workflow-core.test");
    let serialized = serde_json::to_string(&audit).expect("audit serializes");

    assert_eq!(
        audit.event_type,
        WorkflowRunEventKindName::HookInvocationEvaluated
    );
    assert_eq!(
        audit.decision_context.as_deref(),
        Some("hook invocation evaluated: status=warning")
    );
    assert_eq!(audit.action, None);
    assert!(!serialized.contains("hook-invocation/audit-projection"));
    assert!(!serialized.contains("agent-harness/hooks/audit-projection"));
    assert!(!serialized.contains("operator-phase"));
    assert!(!serialized.contains("correlation-hook-payload"));
    assert!(!serialized.contains("raw command output"));
    assert!(!serialized.contains("provider payload"));
    assert!(!serialized.contains("parser payload"));
}

#[test]
fn hook_audit_projection_does_not_emit_dedicated_hook_records_or_observability() {
    let fixture = Fixture::new();
    let event = fixture.event(
        1,
        WorkflowRunEventKind::HookInvocationEvaluated(Box::new(hook_event_payload(
            AgentHarnessHookInvocationStatus::Passed,
        ))),
    );
    let audit_event = AuditEvent::from_workflow_event(&event, "workflow-core.test");
    let sink = LocalAuditSink::new();

    sink.record_audit_event(&audit_event)
        .expect("records generic audit event");

    assert_eq!(sink.events().len(), 1);
    assert!(sink.policy_records().is_empty());
    assert!(sink.adapter_records().is_empty());
    assert!(ObservabilityEvent::from_workflow_event(&event, "workflow-core.test").is_empty());
}

#[test]
fn side_effect_lifecycle_event_projects_to_bounded_audit_event() {
    let fixture = Fixture::new();
    let event = fixture.event(
        1,
        WorkflowRunEventKind::SideEffectProposed(Box::new(side_effect_event_payload(
            SideEffectLifecycleState::Proposed,
        ))),
    );

    let audit = AuditEvent::from_workflow_event(&event, "workflow-core.test");

    assert_eq!(audit.event_id, event.event_id);
    assert_eq!(
        audit.event_type,
        WorkflowRunEventKindName::SideEffectProposed
    );
    assert_eq!(audit.workflow_id, fixture.workflow_id);
    assert_eq!(audit.workflow_run_id, fixture.run_id);
    assert_eq!(audit.schema_version, fixture.schema_version);
    assert_eq!(audit.workflow_version, fixture.workflow_version);
    assert_eq!(audit.spec_hash, fixture.spec_hash);
    assert_eq!(
        audit.step_id,
        Some(StepId::new("write-boundary").expect("step"))
    );
    assert_eq!(
        audit.skill_id,
        Some(SkillId::new("local/write-boundary").expect("skill"))
    );
    assert_eq!(
        audit.skill_version,
        Some(SkillVersion::new("v0").expect("skill version"))
    );
    assert_eq!(audit.actor, event.actor);
    assert_eq!(audit.correlation_id, event.correlation_id);
    assert_eq!(audit.idempotency_key, event.idempotency_key);
    assert_eq!(audit.action, None);
    assert_eq!(
        audit.decision_context.as_deref(),
        Some("side effect proposed: lifecycle=proposed")
    );
    assert_eq!(
        audit.input_reference.as_deref(),
        Some("side-effect:side-effect/audit-projection")
    );
    assert_eq!(audit.output_reference, None);
    assert!(audit.redaction.field_states.iter().any(|state| {
        state.field == "side_effect_context"
            && state.disposition == RedactionDisposition::ReferenceOnly
    }));
}

#[test]
fn all_side_effect_lifecycle_events_project_to_generic_audit_events() {
    let fixture = Fixture::new();
    for (kind, expected_type, expected_context) in [
        (
            WorkflowRunEventKind::SideEffectProposed(Box::new(side_effect_event_payload(
                SideEffectLifecycleState::Proposed,
            ))),
            WorkflowRunEventKindName::SideEffectProposed,
            "side effect proposed: lifecycle=proposed",
        ),
        (
            WorkflowRunEventKind::SideEffectDenied(Box::new(side_effect_event_payload(
                SideEffectLifecycleState::Denied,
            ))),
            WorkflowRunEventKindName::SideEffectDenied,
            "side effect denied: lifecycle=denied",
        ),
        (
            WorkflowRunEventKind::SideEffectSkipped(Box::new(side_effect_event_payload(
                SideEffectLifecycleState::Skipped,
            ))),
            WorkflowRunEventKindName::SideEffectSkipped,
            "side effect skipped: lifecycle=skipped",
        ),
        (
            WorkflowRunEventKind::SideEffectAttempted(Box::new(side_effect_event_payload(
                SideEffectLifecycleState::Attempted,
            ))),
            WorkflowRunEventKindName::SideEffectAttempted,
            "side effect attempted: lifecycle=attempted",
        ),
        (
            WorkflowRunEventKind::SideEffectCompleted(Box::new(side_effect_event_payload(
                SideEffectLifecycleState::Completed,
            ))),
            WorkflowRunEventKindName::SideEffectCompleted,
            "side effect completed: lifecycle=completed",
        ),
        (
            WorkflowRunEventKind::SideEffectFailed(Box::new(side_effect_event_payload(
                SideEffectLifecycleState::Failed,
            ))),
            WorkflowRunEventKindName::SideEffectFailed,
            "side effect failed: lifecycle=failed",
        ),
    ] {
        let audit = AuditEvent::from_workflow_event(&fixture.event(1, kind), "workflow-core.test");

        assert_eq!(audit.event_type, expected_type);
        assert_eq!(audit.action, None);
        assert_eq!(audit.decision_context.as_deref(), Some(expected_context));
    }
}

#[test]
fn side_effect_completed_projection_uses_count_only_outcome_reference() {
    let fixture = Fixture::new();
    let event = fixture.event(
        1,
        WorkflowRunEventKind::SideEffectCompleted(Box::new(side_effect_event_payload(
            SideEffectLifecycleState::Completed,
        ))),
    );

    let audit = AuditEvent::from_workflow_event(&event, "workflow-core.test");
    let serialized = serde_json::to_string(&audit).expect("audit serializes");

    assert_eq!(
        audit.event_type,
        WorkflowRunEventKindName::SideEffectCompleted
    );
    assert_eq!(
        audit.decision_context.as_deref(),
        Some("side effect completed: lifecycle=completed")
    );
    assert_eq!(
        audit.output_reference.as_deref(),
        Some("side-effect-outcome-reference-count:1")
    );
    assert_eq!(audit.action, None);
    assert!(!serialized.contains("event/policy-1"));
    assert!(!serialized.contains("adapter-telemetry/1"));
    assert!(!serialized.contains("correlation-side-effect"));
    assert!(!serialized.contains("raw command output"));
    assert!(!serialized.contains("provider payload"));
    assert!(!serialized.contains("parser payload"));
}

#[test]
fn side_effect_audit_projection_does_not_emit_dedicated_records_or_observability() {
    let fixture = Fixture::new();
    let event = fixture.event(
        1,
        WorkflowRunEventKind::SideEffectFailed(Box::new(side_effect_event_payload(
            SideEffectLifecycleState::Failed,
        ))),
    );
    let audit_event = AuditEvent::from_workflow_event(&event, "workflow-core.test");
    let sink = LocalAuditSink::new();

    sink.record_audit_event(&audit_event)
        .expect("records generic audit event");

    assert_eq!(sink.events().len(), 1);
    assert!(sink.policy_records().is_empty());
    assert!(sink.adapter_records().is_empty());
    assert!(ObservabilityEvent::from_workflow_event(&event, "workflow-core.test").is_empty());
}
