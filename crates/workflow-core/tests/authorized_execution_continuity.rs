#![allow(clippy::expect_used)]
#![doc = "Focused tests for the model-only authorized-execution continuity vocabulary."]

use workflow_core::{
    ActorId, ApprovalReferenceId, AuthorizedExecutionActionReference, AuthorizedExecutionAttemptId,
    AuthorizedExecutionAttemptOutcome, AuthorizedExecutionAuthorityPosture,
    AuthorizedExecutionAuthoritySourceReference, AuthorizedExecutionGateAssessment,
    AuthorizedExecutionGateAssessmentDefinition, AuthorizedExecutionGateBlocker,
    AuthorizedExecutionGateReadiness, AuthorizedExecutionResourceReference,
    AuthorizedExecutionResumeDisposition, AuthorizedExecutionWaitCondition,
    AuthorizedExecutionWaitConditionDefinition, AuthorizedExecutionWaitConditionId,
    AuthorizedExecutionWaitConditionKind, AuthorizedExecutionWaitStatus,
    AuthorizedExecutionWakeTriggerKind, AuthorizedExecutionWindow,
    AuthorizedExecutionWindowDefinition, AuthorizedExecutionWindowId,
    AuthorizedExecutionWindowStatus, AuthorizedExecutionYield, AuthorizedExecutionYieldDefinition,
    AuthorizedExecutionYieldReason, EventId, EventSequenceNumber, ImmutableRunBundleBinding,
    SpecContentHash, StepId, Timestamp, WorkReportSensitivity, WorkflowId, WorkflowRunId,
};

fn timestamp(value: &str) -> Timestamp {
    Timestamp::parse_rfc3339(value).expect("valid timestamp")
}

fn bundle_binding() -> ImmutableRunBundleBinding {
    serde_json::from_value(serde_json::json!({
        "bundle_id": "bundle/authorized-execution",
        "bundle_version": "v1",
        "root_hash": SpecContentHash::from_text("immutable bundle root").as_str(),
    }))
    .expect("valid immutable bundle binding")
}

fn window_definition() -> AuthorizedExecutionWindowDefinition {
    AuthorizedExecutionWindowDefinition {
        window_id: AuthorizedExecutionWindowId::new("window/run-1/step-1").expect("window id"),
        status: AuthorizedExecutionWindowStatus::Open,
        workflow_id: WorkflowId::new("workflow/authorized-execution").expect("workflow id"),
        run_id: WorkflowRunId::new("run/authorized-execution").expect("run id"),
        step_id: StepId::new("step-1").expect("step id"),
        immutable_run_bundle: bundle_binding(),
        subject_actor_id: ActorId::new("agent/maintainer").expect("actor"),
        approval_references: vec![
            ApprovalReferenceId::new("approval/run-1/step-1").expect("approval")
        ],
        allowed_actions: vec![
            AuthorizedExecutionActionReference::new("action/edit").expect("action")
        ],
        resource_scope: vec![
            AuthorizedExecutionResourceReference::new("repo/workflow-os").expect("resource"),
        ],
        authority_source: AuthorizedExecutionAuthoritySourceReference::new("authority/local")
            .expect("authority"),
        opened_sequence_number: EventSequenceNumber::new(6).expect("sequence"),
        opened_event_id: EventId::new("event/window-opened").expect("event id"),
        opened_at: timestamp("2026-08-15T12:00:00Z"),
        evaluated_at: timestamp("2026-08-15T12:00:01Z"),
        expires_at: timestamp("2026-08-15T12:05:00Z"),
        maximum_attempts: 2,
        status_event_id: None,
        sensitivity_ceiling: WorkReportSensitivity::Confidential,
        governance_commitment: SpecContentHash::from_text("governance inputs"),
        authority_commitment: SpecContentHash::from_text("authority inputs"),
    }
}

fn window() -> AuthorizedExecutionWindow {
    AuthorizedExecutionWindow::new(window_definition()).expect("window")
}

fn wait_definition(
    kind: AuthorizedExecutionWaitConditionKind,
) -> AuthorizedExecutionWaitConditionDefinition {
    let (wake_trigger, deadline) = match kind {
        AuthorizedExecutionWaitConditionKind::HumanDecision => (
            AuthorizedExecutionWakeTriggerKind::ApprovalDecisionRecorded,
            None,
        ),
        AuthorizedExecutionWaitConditionKind::EvidenceRequired => {
            (AuthorizedExecutionWakeTriggerKind::EvidenceAccepted, None)
        }
        AuthorizedExecutionWaitConditionKind::CheckRequired => {
            (AuthorizedExecutionWakeTriggerKind::CheckAccepted, None)
        }
        AuthorizedExecutionWaitConditionKind::ExternalEvent => (
            AuthorizedExecutionWakeTriggerKind::ExternalEventRecorded,
            None,
        ),
        AuthorizedExecutionWaitConditionKind::CapabilityUnavailable => (
            AuthorizedExecutionWakeTriggerKind::CapabilityAvailabilityChanged,
            None,
        ),
        AuthorizedExecutionWaitConditionKind::TimeWindow => (
            AuthorizedExecutionWakeTriggerKind::DeadlineReached,
            Some(timestamp("2026-08-15T12:04:00Z")),
        ),
        AuthorizedExecutionWaitConditionKind::AuthorityRefresh => (
            AuthorizedExecutionWakeTriggerKind::AuthoritySourceChanged,
            None,
        ),
        AuthorizedExecutionWaitConditionKind::ConflictResolution => {
            (AuthorizedExecutionWakeTriggerKind::ConflictResolved, None)
        }
    };
    AuthorizedExecutionWaitConditionDefinition {
        condition_id: AuthorizedExecutionWaitConditionId::new(
            format!("wait/{kind:?}").to_ascii_lowercase(),
        )
        .expect("condition id"),
        condition_version: 1,
        kind,
        workflow_id: WorkflowId::new("workflow/authorized-execution").expect("workflow id"),
        run_id: WorkflowRunId::new("run/authorized-execution").expect("run id"),
        window_id: AuthorizedExecutionWindowId::new("window/run-1/step-1").expect("window id"),
        action_reference: AuthorizedExecutionActionReference::new("action/edit").expect("action"),
        step_id: StepId::new("step-1").expect("step id"),
        attempt_id: AuthorizedExecutionAttemptId::new("attempt/run-1/step-1/1").expect("attempt"),
        expected_sequence_number: EventSequenceNumber::new(7).expect("sequence"),
        expected_event_id: EventId::new("event/yielded").expect("event"),
        required_reference: AuthorizedExecutionResourceReference::new("requirement/exact")
            .expect("required ref"),
        created_at: timestamp("2026-08-15T12:01:00Z"),
        deadline,
        wake_trigger,
        status: AuthorizedExecutionWaitStatus::Waiting,
    }
}

fn wait(kind: AuthorizedExecutionWaitConditionKind) -> AuthorizedExecutionWaitCondition {
    AuthorizedExecutionWaitCondition::new(wait_definition(kind)).expect("wait")
}

fn assessment_definition() -> AuthorizedExecutionGateAssessmentDefinition {
    AuthorizedExecutionGateAssessmentDefinition {
        workflow_id: WorkflowId::new("workflow/authorized-execution").expect("workflow id"),
        run_id: WorkflowRunId::new("run/authorized-execution").expect("run id"),
        step_id: StepId::new("step-1").expect("step id"),
        approval_reference: ApprovalReferenceId::new("approval/run-1/step-1").expect("approval"),
        action_reference: AuthorizedExecutionActionReference::new("action/edit").expect("action"),
        immutable_run_bundle: bundle_binding(),
        last_sequence_number: EventSequenceNumber::new(7).expect("sequence"),
        last_event_id: EventId::new("event/prerequisite").expect("event id"),
        assessed_at: timestamp("2026-08-15T12:00:00Z"),
        readiness: AuthorizedExecutionGateReadiness::PendingPrerequisites,
        blockers: vec![AuthorizedExecutionGateBlocker::EvidenceRequired],
        assessment_commitment: SpecContentHash::from_text("assessment inputs"),
    }
}

fn yield_definition(
    wait_conditions: Vec<AuthorizedExecutionWaitCondition>,
) -> AuthorizedExecutionYieldDefinition {
    AuthorizedExecutionYieldDefinition {
        attempt_id: AuthorizedExecutionAttemptId::new("attempt/run-1/step-1/1").expect("attempt"),
        yielded_sequence_number: EventSequenceNumber::new(7).expect("sequence"),
        yielded_event_id: EventId::new("event/yielded").expect("event"),
        yielded_at: timestamp("2026-08-15T12:01:00Z"),
        reason: AuthorizedExecutionYieldReason::TurnBoundary,
        resume_disposition: if wait_conditions.is_empty() {
            AuthorizedExecutionResumeDisposition::EligibleForFreshAuthorization
        } else {
            AuthorizedExecutionResumeDisposition::Wait
        },
        wait_conditions,
    }
}

#[test]
fn gate_readiness_is_non_authoritative_and_separate_from_execution_resume() {
    let pending = AuthorizedExecutionGateAssessment::new(assessment_definition()).expect("pending");
    assert_eq!(
        pending.authority_posture(),
        AuthorizedExecutionAuthorityPosture::NonAuthoritative
    );
    assert_eq!(
        pending.readiness(),
        AuthorizedExecutionGateReadiness::PendingPrerequisites
    );

    let mut ready = assessment_definition();
    ready.readiness = AuthorizedExecutionGateReadiness::ReadyForDecision;
    ready.blockers.clear();
    let ready = AuthorizedExecutionGateAssessment::new(ready).expect("ready for decision");
    assert_eq!(
        ready.readiness(),
        AuthorizedExecutionGateReadiness::ReadyForDecision
    );

    let encoded = serde_json::to_value(ready).expect("serializes");
    assert_eq!(encoded["authority_posture"], "non_authoritative");
    assert!(encoded.get("resume_disposition").is_none());
}

#[test]
fn gate_readiness_requires_exact_blocker_posture_and_unique_bounds() {
    let mut invalid = assessment_definition();
    invalid.readiness = AuthorizedExecutionGateReadiness::ReadyForDecision;
    assert_eq!(
        AuthorizedExecutionGateAssessment::new(invalid)
            .expect_err("blocked ready fails")
            .code(),
        "authorized_execution_continuity.gate_assessment.readiness_mismatch"
    );

    let mut invalid = assessment_definition();
    invalid.blockers.clear();
    assert_eq!(
        AuthorizedExecutionGateAssessment::new(invalid)
            .expect_err("empty pending fails")
            .code(),
        "authorized_execution_continuity.gate_assessment.readiness_mismatch"
    );

    let mut duplicate = assessment_definition();
    duplicate
        .blockers
        .push(AuthorizedExecutionGateBlocker::EvidenceRequired);
    assert_eq!(
        AuthorizedExecutionGateAssessment::new(duplicate)
            .expect_err("duplicate fails")
            .code(),
        "authorized_execution_continuity.gate_assessment.duplicate_blocker"
    );

    let mut all_unique = assessment_definition();
    all_unique.blockers = vec![
        AuthorizedExecutionGateBlocker::EvidenceRequired,
        AuthorizedExecutionGateBlocker::CheckRequired,
        AuthorizedExecutionGateBlocker::PolicyDenied,
        AuthorizedExecutionGateBlocker::AuthorityUnavailable,
        AuthorizedExecutionGateBlocker::ApprovalPresentationRequired,
        AuthorizedExecutionGateBlocker::SeparationOfDutyRequired,
        AuthorizedExecutionGateBlocker::StaleCursor,
        AuthorizedExecutionGateBlocker::AmbiguousFacts,
    ];
    AuthorizedExecutionGateAssessment::new(all_unique).expect("all unique blockers fit");
}

#[test]
fn all_wait_kinds_are_exactly_bound_and_round_trip() {
    let kinds = [
        AuthorizedExecutionWaitConditionKind::HumanDecision,
        AuthorizedExecutionWaitConditionKind::EvidenceRequired,
        AuthorizedExecutionWaitConditionKind::CheckRequired,
        AuthorizedExecutionWaitConditionKind::ExternalEvent,
        AuthorizedExecutionWaitConditionKind::CapabilityUnavailable,
        AuthorizedExecutionWaitConditionKind::TimeWindow,
        AuthorizedExecutionWaitConditionKind::AuthorityRefresh,
        AuthorizedExecutionWaitConditionKind::ConflictResolution,
    ];
    for kind in kinds {
        let condition = wait(kind);
        assert_eq!(condition.kind(), kind);
        assert_eq!(condition.status(), AuthorizedExecutionWaitStatus::Waiting);
        let encoded = serde_json::to_string(&condition).expect("serializes");
        let decoded: AuthorizedExecutionWaitCondition =
            serde_json::from_str(&encoded).expect("deserializes");
        assert_eq!(decoded, condition);
    }
}

#[test]
fn wait_validation_rejects_bad_version_deadline_and_wake_trigger() {
    let mut definition = wait_definition(AuthorizedExecutionWaitConditionKind::EvidenceRequired);
    definition.condition_version = 0;
    assert_eq!(
        AuthorizedExecutionWaitCondition::new(definition)
            .expect_err("zero version")
            .code(),
        "authorized_execution_continuity.wait.condition_version_zero"
    );

    let mut definition = wait_definition(AuthorizedExecutionWaitConditionKind::EvidenceRequired);
    definition.wake_trigger = AuthorizedExecutionWakeTriggerKind::CheckAccepted;
    assert_eq!(
        AuthorizedExecutionWaitCondition::new(definition)
            .expect_err("wrong trigger")
            .code(),
        "authorized_execution_continuity.wait.wake_trigger_mismatch"
    );

    let mut definition = wait_definition(AuthorizedExecutionWaitConditionKind::TimeWindow);
    definition.deadline = None;
    assert_eq!(
        AuthorizedExecutionWaitCondition::new(definition)
            .expect_err("deadline required")
            .code(),
        "authorized_execution_continuity.wait.deadline_required"
    );
}

#[test]
fn execution_window_is_subject_scope_authority_cursor_and_bundle_bound() {
    let window = window();
    assert_eq!(
        window.authority_posture(),
        AuthorizedExecutionAuthorityPosture::NonAuthoritative
    );
    assert_eq!(window.status(), AuthorizedExecutionWindowStatus::Open);
    let encoded = serde_json::to_value(&window).expect("serializes");
    assert_eq!(encoded["authority_posture"], "non_authoritative");
    assert_eq!(
        serde_json::from_value::<AuthorizedExecutionWindow>(encoded).expect("round trip"),
        window
    );
}

#[test]
fn execution_window_rejects_unbound_scope_unknown_sensitivity_and_invalid_lifecycle() {
    let mut definition = window_definition();
    definition.allowed_actions.clear();
    assert_eq!(
        AuthorizedExecutionWindow::new(definition)
            .expect_err("action scope required")
            .code(),
        "authorized_execution_continuity.window.allowed_actions_invalid"
    );

    let mut definition = window_definition();
    definition.sensitivity_ceiling = WorkReportSensitivity::Unknown;
    assert_eq!(
        AuthorizedExecutionWindow::new(definition)
            .expect_err("known sensitivity")
            .code(),
        "authorized_execution_continuity.window.sensitivity_unknown"
    );

    let mut definition = window_definition();
    definition.status = AuthorizedExecutionWindowStatus::Revoked;
    assert_eq!(
        AuthorizedExecutionWindow::new(definition)
            .expect_err("provenance required")
            .code(),
        "authorized_execution_continuity.window.status_provenance_mismatch"
    );

    let mut definition = window_definition();
    definition.evaluated_at = definition.expires_at;
    assert_eq!(
        AuthorizedExecutionWindow::new(definition)
            .expect_err("open expired")
            .code(),
        "authorized_execution_continuity.window.open_after_expiry"
    );
}

#[test]
fn ordinary_turn_boundary_yields_without_a_false_wait() {
    let window = window();
    let yielded =
        AuthorizedExecutionYield::new(&window, yield_definition(Vec::new())).expect("yield");
    assert!(yielded.wait_conditions().is_empty());
    assert_eq!(
        yielded.resume_disposition(),
        AuthorizedExecutionResumeDisposition::EligibleForFreshAuthorization
    );
    assert_eq!(
        yielded.authority_posture(),
        AuthorizedExecutionAuthorityPosture::NonAuthoritative
    );
    let encoded = serde_json::to_value(yielded).expect("serializes");
    assert_eq!(
        encoded["binding_verification"],
        "requires_owning_window_reconciliation"
    );
}

#[test]
fn genuine_wait_requires_active_exact_attempt_binding() {
    let window = window();
    let condition = wait(AuthorizedExecutionWaitConditionKind::EvidenceRequired);
    let yielded = AuthorizedExecutionYield::new(&window, yield_definition(vec![condition]))
        .expect("wait yield");
    assert_eq!(
        yielded.resume_disposition(),
        AuthorizedExecutionResumeDisposition::Wait
    );

    let mut stale = wait_definition(AuthorizedExecutionWaitConditionKind::EvidenceRequired);
    stale.attempt_id = AuthorizedExecutionAttemptId::new("attempt/other").expect("other attempt");
    let stale = AuthorizedExecutionWaitCondition::new(stale).expect("valid standalone wait");
    assert_eq!(
        AuthorizedExecutionYield::new(&window, yield_definition(vec![stale]))
            .expect_err("cross-attempt wait fails")
            .code(),
        "authorized_execution_continuity.yield.wait_binding_mismatch"
    );

    let mut stale_cursor = wait_definition(AuthorizedExecutionWaitConditionKind::EvidenceRequired);
    stale_cursor.expected_sequence_number = EventSequenceNumber::new(8).expect("sequence");
    let stale_cursor =
        AuthorizedExecutionWaitCondition::new(stale_cursor).expect("standalone wait");
    assert_eq!(
        AuthorizedExecutionYield::new(&window, yield_definition(vec![stale_cursor]))
            .expect_err("cursor mismatch fails")
            .code(),
        "authorized_execution_continuity.yield.wait_binding_mismatch"
    );

    let mut wrong_action = wait_definition(AuthorizedExecutionWaitConditionKind::EvidenceRequired);
    wrong_action.action_reference =
        AuthorizedExecutionActionReference::new("action/delete").expect("action");
    let wrong_action =
        AuthorizedExecutionWaitCondition::new(wrong_action).expect("standalone wait");
    assert_eq!(
        AuthorizedExecutionYield::new(&window, yield_definition(vec![wrong_action]))
            .expect_err("out-of-window action fails")
            .code(),
        "authorized_execution_continuity.yield.window_binding_mismatch"
    );

    let mut future_wait = wait_definition(AuthorizedExecutionWaitConditionKind::EvidenceRequired);
    future_wait.created_at = timestamp("2026-08-15T12:02:00Z");
    let future_wait = AuthorizedExecutionWaitCondition::new(future_wait).expect("standalone wait");
    assert_eq!(
        AuthorizedExecutionYield::new(&window, yield_definition(vec![future_wait]))
            .expect_err("future wait fails")
            .code(),
        "authorized_execution_continuity.yield.wait_binding_mismatch"
    );

    let mut predating_wait =
        wait_definition(AuthorizedExecutionWaitConditionKind::EvidenceRequired);
    predating_wait.created_at = timestamp("2026-08-15T12:00:00Z");
    let predating_wait =
        AuthorizedExecutionWaitCondition::new(predating_wait).expect("standalone wait");
    assert_eq!(
        AuthorizedExecutionYield::new(&window, yield_definition(vec![predating_wait]))
            .expect_err("wait predating authority evaluation fails")
            .code(),
        "authorized_execution_continuity.yield.window_binding_mismatch"
    );
}

#[test]
fn yield_derives_identity_from_window_and_rejects_temporal_mismatch() {
    let window = window();
    let mut definition = yield_definition(Vec::new());
    definition.yielded_at = timestamp("2026-08-15T11:59:59Z");
    assert_eq!(
        AuthorizedExecutionYield::new(&window, definition)
            .expect_err("predating yield fails")
            .code(),
        "authorized_execution_continuity.yield.window_binding_mismatch"
    );

    let mut definition = yield_definition(Vec::new());
    definition.yielded_at = timestamp("2026-08-15T12:00:00Z");
    assert_eq!(
        AuthorizedExecutionYield::new(&window, definition)
            .expect_err("yield predating authority evaluation fails")
            .code(),
        "authorized_execution_continuity.yield.window_binding_mismatch"
    );

    let mut definition = yield_definition(Vec::new());
    definition.yielded_sequence_number = EventSequenceNumber::new(6).expect("sequence");
    assert_eq!(
        AuthorizedExecutionYield::new(&window, definition)
            .expect_err("same cursor with different event fails")
            .code(),
        "authorized_execution_continuity.yield.window_binding_mismatch"
    );

    let mut definition = yield_definition(Vec::new());
    definition.yielded_at = timestamp("2026-08-15T12:05:00Z");
    assert_eq!(
        AuthorizedExecutionYield::new(&window, definition)
            .expect_err("expired yield fails")
            .code(),
        "authorized_execution_continuity.yield.window_binding_mismatch"
    );
}

#[test]
fn every_attempt_outcome_blocks_automatic_retry_until_fresh_authorization() {
    for outcome in [
        AuthorizedExecutionAttemptOutcome::Succeeded,
        AuthorizedExecutionAttemptOutcome::RetryableFailure,
        AuthorizedExecutionAttemptOutcome::TerminalFailure,
        AuthorizedExecutionAttemptOutcome::AmbiguousMayHaveStarted,
    ] {
        assert!(outcome.blocks_automatic_retry());
    }
    assert!(AuthorizedExecutionAttemptOutcome::AmbiguousMayHaveStarted.requires_reconciliation());
}

#[test]
fn malformed_serialized_models_fail_closed_without_echoing_secret_like_values() {
    let secret = "bearer-sensitive-value";
    for mut value in [
        serde_json::to_value(window()).expect("window value"),
        serde_json::to_value(
            AuthorizedExecutionGateAssessment::new(assessment_definition()).expect("assessment"),
        )
        .expect("assessment value"),
    ] {
        value[secret] = serde_json::json!(true);
        let error = if value.get("window_id").is_some() {
            serde_json::from_value::<AuthorizedExecutionWindow>(value)
                .expect_err("unknown field")
                .to_string()
        } else {
            serde_json::from_value::<AuthorizedExecutionGateAssessment>(value)
                .expect_err("unknown field")
                .to_string()
        };
        assert!(!error.contains(secret));
    }

    let mut value = serde_json::to_value(window()).expect("window value");
    value["status"] = serde_json::json!(secret);
    let error = serde_json::from_value::<AuthorizedExecutionWindow>(value)
        .expect_err("bad enum")
        .to_string();
    assert!(!error.contains(secret));
}

#[test]
fn debug_output_redacts_all_bound_identity_and_commitments() {
    let window = window();
    let condition = wait(AuthorizedExecutionWaitConditionKind::HumanDecision);
    let yielded = AuthorizedExecutionYield::new(&window, yield_definition(vec![condition.clone()]))
        .expect("yield");
    let assessment =
        AuthorizedExecutionGateAssessment::new(assessment_definition()).expect("assessment");
    let debug = format!("{window:?} {condition:?} {yielded:?} {assessment:?}");
    for forbidden in [
        "run/authorized-execution",
        "agent/maintainer",
        "repo/workflow-os",
        "approval/run-1/step-1",
        "authority/local",
    ] {
        assert!(!debug.contains(forbidden));
    }
    assert!(debug.contains("[REDACTED]"));
}
