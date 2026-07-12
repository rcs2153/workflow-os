#![allow(clippy::expect_used)]

//! Proportional-governance core decision model tests.

use workflow_core::{
    project_proportional_governance_decision, select_proportional_governance,
    GovernanceActionRequirement, GovernanceDecisionPosture, GovernanceDecisionReason,
    GovernanceInteractionMode, GovernancePersistencePosture, GovernancePostureRequirement,
    GovernanceRiskClass, GovernanceStrictnessProfile, ProportionalGovernanceDecisionInput,
    ProportionalGovernanceDecisionProjection, WorkflowOsErrorKind,
};

fn quiet_input() -> ProportionalGovernanceDecisionInput {
    ProportionalGovernanceDecisionInput {
        profile: GovernanceStrictnessProfile::ObserveAndReport,
        workflow: GovernancePostureRequirement::QuietCapture,
        policy: GovernancePostureRequirement::QuietCapture,
        authority: GovernancePostureRequirement::QuietCapture,
        evidence_and_checks: GovernancePostureRequirement::QuietCapture,
        sensitivity: GovernancePostureRequirement::QuietCapture,
        side_effect: GovernancePostureRequirement::QuietCapture,
        runtime_escalation: GovernancePostureRequirement::QuietCapture,
        prior_mode: None,
        steward_minimum: None,
    }
}

#[test]
fn bounded_observation_selects_quiet_capture() {
    let decision = select_proportional_governance(quiet_input()).expect("valid decision");

    assert_eq!(decision.mode(), GovernanceInteractionMode::QuietCapture);
    assert_eq!(
        decision.risk_class(),
        GovernanceRiskClass::BoundedObservation
    );
    assert_eq!(
        decision.reasons().iter().copied().collect::<Vec<_>>(),
        vec![GovernanceDecisionReason::ProfileMinimum]
    );
}

#[test]
fn strictest_requirement_wins_independent_of_other_inputs() {
    let mut input = quiet_input();
    input.evidence_and_checks = GovernancePostureRequirement::VisibleDisclosure;
    input.policy = GovernancePostureRequirement::BlockingApproval;

    let decision = select_proportional_governance(input).expect("valid decision");

    assert_eq!(decision.mode(), GovernanceInteractionMode::BlockingApproval);
    assert!(decision
        .reasons()
        .contains(&GovernanceDecisionReason::PolicyRequirement));
    assert!(decision
        .reasons()
        .contains(&GovernanceDecisionReason::EvidenceOrCheckRequirement));
}

#[test]
fn profile_minimum_cannot_be_downgraded() {
    let mut input = quiet_input();
    input.profile = GovernanceStrictnessProfile::HumanApprovalGated;

    let decision = select_proportional_governance(input).expect("valid decision");

    assert_eq!(decision.mode(), GovernanceInteractionMode::BlockingApproval);
    assert_eq!(decision.risk_class(), GovernanceRiskClass::ApprovalRequired);
}

#[test]
fn agent_assisted_profile_allows_quiet_when_requirements_are_satisfied() {
    let mut input = quiet_input();
    input.profile = GovernanceStrictnessProfile::AgentAssistedGated;

    let decision = select_proportional_governance(input).expect("valid decision");

    assert_eq!(decision.mode(), GovernanceInteractionMode::QuietCapture);
}

#[test]
fn strict_enterprise_requires_explicit_steward_minimum() {
    let mut input = quiet_input();
    input.profile = GovernanceStrictnessProfile::StrictEnterprise;

    let error = select_proportional_governance(input).expect_err("minimum must be required");

    assert_eq!(error.kind(), WorkflowOsErrorKind::InvalidState);
    assert_eq!(
        error.code(),
        "governance.proportional.steward_minimum.required"
    );
}

#[test]
fn strict_enterprise_applies_explicit_steward_minimum() {
    let mut input = quiet_input();
    input.profile = GovernanceStrictnessProfile::StrictEnterprise;
    input.steward_minimum = Some(GovernancePostureRequirement::VisibleDisclosure);

    let decision = select_proportional_governance(input).expect("valid decision");

    assert_eq!(
        decision.mode(),
        GovernanceInteractionMode::VisibleDisclosure
    );
    assert!(decision
        .reasons()
        .contains(&GovernanceDecisionReason::StewardMinimum));
}

#[test]
fn runtime_change_escalates_monotonically_from_prior_decision() {
    let mut input = quiet_input();
    input.prior_mode = Some(GovernanceInteractionMode::VisibleDisclosure);
    input.runtime_escalation = GovernancePostureRequirement::BlockingApproval;

    let decision = select_proportional_governance(input).expect("valid decision");

    assert_eq!(decision.mode(), GovernanceInteractionMode::BlockingApproval);
    assert!(decision
        .reasons()
        .contains(&GovernanceDecisionReason::PriorDecisionMinimum));
    assert!(decision
        .reasons()
        .contains(&GovernanceDecisionReason::RuntimeEscalation));
}

#[test]
fn prior_denial_cannot_be_downgraded() {
    let mut input = quiet_input();
    input.prior_mode = Some(GovernanceInteractionMode::Denied);

    let decision = select_proportional_governance(input).expect("valid decision");

    assert_eq!(decision.mode(), GovernanceInteractionMode::Denied);
    assert_eq!(decision.risk_class(), GovernanceRiskClass::Denied);
}

#[test]
fn explicit_denial_wins() {
    let mut input = quiet_input();
    input.authority = GovernancePostureRequirement::Denied;

    let decision = select_proportional_governance(input).expect("valid decision");

    assert_eq!(decision.mode(), GovernanceInteractionMode::Denied);
    assert!(decision
        .reasons()
        .contains(&GovernanceDecisionReason::AuthorityRequirement));
}

#[test]
fn unsupported_requirement_fails_closed_without_payloads() {
    let mut input = quiet_input();
    input.side_effect = GovernancePostureRequirement::Unsupported;

    let error = select_proportional_governance(input).expect_err("unsupported must fail");

    assert_eq!(error.kind(), WorkflowOsErrorKind::Unsupported);
    assert_eq!(
        error.code(),
        "governance.proportional.requirement.unsupported"
    );
    assert_eq!(
        error.message(),
        "a declared governance requirement is not supported"
    );
}

#[test]
fn serde_round_trip_preserves_bounded_decision() {
    let mut input = quiet_input();
    input.sensitivity = GovernancePostureRequirement::VisibleDisclosure;
    let decision = select_proportional_governance(input).expect("valid decision");

    let serialized = serde_json::to_string(&decision).expect("serialize");
    let decoded = serde_json::from_str(&serialized).expect("deserialize");

    assert_eq!(decision, decoded);
    assert!(!serialized.contains("payload"));
    assert!(!serialized.contains("command_output"));
}

#[test]
fn inconsistent_serialized_decision_fails_closed() {
    let invalid = serde_json::json!({
        "mode": "quiet_capture",
        "risk_class": "denied",
        "reasons": ["profile_minimum"]
    });

    let error = serde_json::from_value::<workflow_core::ProportionalGovernanceDecision>(invalid)
        .expect_err("inconsistent decision must fail");

    assert_eq!(
        error.to_string(),
        "invalid proportional governance decision"
    );
}

#[test]
fn serialized_decision_without_profile_reason_fails_closed() {
    let invalid = serde_json::json!({
        "mode": "quiet_capture",
        "risk_class": "bounded_observation",
        "reasons": []
    });

    let error = serde_json::from_value::<workflow_core::ProportionalGovernanceDecision>(invalid)
        .expect_err("unvalidated decision must fail");

    assert_eq!(
        error.to_string(),
        "invalid proportional governance decision"
    );
}

#[test]
fn every_reason_source_is_representable() {
    let mut input = quiet_input();
    input.workflow = GovernancePostureRequirement::VisibleDisclosure;
    input.policy = GovernancePostureRequirement::VisibleDisclosure;
    input.authority = GovernancePostureRequirement::VisibleDisclosure;
    input.evidence_and_checks = GovernancePostureRequirement::VisibleDisclosure;
    input.sensitivity = GovernancePostureRequirement::VisibleDisclosure;
    input.side_effect = GovernancePostureRequirement::VisibleDisclosure;
    input.runtime_escalation = GovernancePostureRequirement::VisibleDisclosure;
    input.prior_mode = Some(GovernanceInteractionMode::VisibleDisclosure);
    input.steward_minimum = Some(GovernancePostureRequirement::VisibleDisclosure);

    let decision = select_proportional_governance(input).expect("valid decision");

    assert_eq!(decision.reasons().len(), 10);
}

#[test]
fn projection_maps_every_mode_to_stable_operator_action() {
    let cases = [
        (
            GovernancePostureRequirement::QuietCapture,
            GovernanceInteractionMode::QuietCapture,
            GovernanceActionRequirement::None,
        ),
        (
            GovernancePostureRequirement::VisibleDisclosure,
            GovernanceInteractionMode::VisibleDisclosure,
            GovernanceActionRequirement::Review,
        ),
        (
            GovernancePostureRequirement::BlockingApproval,
            GovernanceInteractionMode::BlockingApproval,
            GovernanceActionRequirement::Approval,
        ),
        (
            GovernancePostureRequirement::Denied,
            GovernanceInteractionMode::Denied,
            GovernanceActionRequirement::Denied,
        ),
    ];

    for (requirement, mode, action_requirement) in cases {
        let mut input = quiet_input();
        input.workflow = requirement;
        let decision = select_proportional_governance(input).expect("valid decision");
        let projection = project_proportional_governance_decision(&decision);

        assert_eq!(projection.mode(), mode);
        assert_eq!(projection.risk_class(), decision.risk_class());
        assert_eq!(projection.reasons(), decision.reasons());
        assert_eq!(projection.action_requirement(), action_requirement);
        assert_eq!(
            projection.decision_posture(),
            GovernanceDecisionPosture::AssessedNotEnforced
        );
        assert_eq!(
            projection.persistence_posture(),
            GovernancePersistencePosture::NotPersisted
        );
    }
}

#[test]
fn projection_does_not_mutate_the_source_decision() {
    let mut input = quiet_input();
    input.policy = GovernancePostureRequirement::BlockingApproval;
    let decision = select_proportional_governance(input).expect("valid decision");
    let expected = decision.clone();

    let _projection = project_proportional_governance_decision(&decision);

    assert_eq!(decision, expected);
}

#[test]
fn projection_serde_round_trip_preserves_validated_posture() {
    let mut input = quiet_input();
    input.evidence_and_checks = GovernancePostureRequirement::VisibleDisclosure;
    let decision = select_proportional_governance(input).expect("valid decision");
    let projection = project_proportional_governance_decision(&decision);

    let serialized = serde_json::to_string(&projection).expect("serialize");
    let decoded = serde_json::from_str::<ProportionalGovernanceDecisionProjection>(&serialized)
        .expect("deserialize");

    assert_eq!(decoded, projection);
    assert!(serialized.contains("assessed_not_enforced"));
    assert!(serialized.contains("not_persisted"));
}

#[test]
fn inconsistent_projection_action_fails_closed() {
    let invalid = serde_json::json!({
        "mode": "quiet_capture",
        "risk_class": "bounded_observation",
        "reasons": ["profile_minimum"],
        "action_requirement": "approval",
        "decision_posture": "assessed_not_enforced",
        "persistence_posture": "not_persisted"
    });

    let error = serde_json::from_value::<ProportionalGovernanceDecisionProjection>(invalid)
        .expect_err("inconsistent action must fail");

    assert_eq!(
        error.to_string(),
        "invalid proportional governance decision projection"
    );
}

#[test]
fn inconsistent_projection_decision_fails_closed() {
    let invalid = serde_json::json!({
        "mode": "visible_disclosure",
        "risk_class": "approval_required",
        "reasons": ["profile_minimum"],
        "action_requirement": "review",
        "decision_posture": "assessed_not_enforced",
        "persistence_posture": "not_persisted"
    });

    let error = serde_json::from_value::<ProportionalGovernanceDecisionProjection>(invalid)
        .expect_err("inconsistent decision must fail");

    assert_eq!(
        error.to_string(),
        "invalid proportional governance decision projection"
    );
}

#[test]
fn projection_debug_and_serialization_remain_payload_free() {
    let decision = select_proportional_governance(quiet_input()).expect("valid decision");
    let projection = project_proportional_governance_decision(&decision);
    let debug = format!("{projection:?}");
    let serialized = serde_json::to_value(&projection).expect("serialize");
    let object = serialized.as_object().expect("projection object");

    for forbidden in [
        "workflow_id",
        "run_id",
        "actor_id",
        "evidence_id",
        "approval_id",
        "event_id",
        "report_id",
        "provider_payload",
        "path",
        "timestamp",
        "command_output",
        "token",
    ] {
        assert!(!object.contains_key(forbidden));
        assert!(!debug.contains(forbidden));
    }
}

#[test]
fn projection_unknown_values_fail_without_leaking_input() {
    let decision = select_proportional_governance(quiet_input()).expect("valid decision");
    let projection = project_proportional_governance_decision(&decision);
    let valid = serde_json::to_value(projection).expect("serialize");
    let secret = "token-sk-projection-secret";
    let cases = [
        ("mode", serde_json::Value::String(secret.to_owned())),
        ("risk_class", serde_json::Value::String(secret.to_owned())),
        (
            "action_requirement",
            serde_json::Value::String(secret.to_owned()),
        ),
        (
            "decision_posture",
            serde_json::Value::String(secret.to_owned()),
        ),
        (
            "persistence_posture",
            serde_json::Value::String(secret.to_owned()),
        ),
        ("mode", serde_json::Value::Number(424_242.into())),
    ];

    for (field, invalid_value) in cases {
        let mut invalid = valid.clone();
        invalid
            .as_object_mut()
            .expect("projection object")
            .insert(field.to_owned(), invalid_value);

        let error = serde_json::from_value::<ProportionalGovernanceDecisionProjection>(invalid)
            .expect_err("unknown projection value must fail");

        assert!(!error.to_string().contains(secret));
        assert!(!error.to_string().contains("424242"));
    }
}

#[test]
fn projection_unknown_reason_fails_without_leaking_input() {
    let decision = select_proportional_governance(quiet_input()).expect("valid decision");
    let projection = project_proportional_governance_decision(&decision);
    let mut invalid = serde_json::to_value(projection).expect("serialize");
    let secret = "token-sk-projection-reason";
    invalid.as_object_mut().expect("projection object").insert(
        "reasons".to_owned(),
        serde_json::json!(["profile_minimum", secret]),
    );

    let error = serde_json::from_value::<ProportionalGovernanceDecisionProjection>(invalid)
        .expect_err("unknown reason must fail");

    assert_eq!(
        error.to_string(),
        "invalid proportional governance projection reason"
    );
    assert!(!error.to_string().contains(secret));
}
