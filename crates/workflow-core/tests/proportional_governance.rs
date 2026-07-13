#![allow(clippy::expect_used)]

//! Proportional-governance core decision and projection tests.

use workflow_core::{
    project_proportional_governance_decision, select_proportional_governance,
    GovernanceActionRequirement, GovernanceDecisionPosture, GovernanceDecisionReason,
    GovernanceDisclosureObligation, GovernanceDisclosureRequirement,
    GovernanceExecutionDisposition, GovernanceExecutionRequirement, GovernancePersistencePosture,
    GovernancePostureRequirement, GovernanceRiskClass, GovernanceStrictnessProfile,
    ProportionalGovernanceDecisionInput, ProportionalGovernanceDecisionProjection,
    WorkflowOsErrorKind,
};

fn quiet_input() -> ProportionalGovernanceDecisionInput {
    ProportionalGovernanceDecisionInput {
        profile: GovernanceStrictnessProfile::ObserveAndReport,
        workflow: GovernancePostureRequirement::quiet(),
        policy: GovernancePostureRequirement::quiet(),
        authority: GovernancePostureRequirement::quiet(),
        evidence_and_checks: GovernancePostureRequirement::quiet(),
        sensitivity: GovernancePostureRequirement::quiet(),
        side_effect: GovernancePostureRequirement::quiet(),
        runtime_escalation: GovernancePostureRequirement::quiet(),
        prior_execution: None,
        prior_disclosure: None,
        steward_minimum: None,
    }
}

#[test]
fn bounded_observation_proceeds_quietly() {
    let decision = select_proportional_governance(quiet_input()).expect("valid decision");

    assert_eq!(
        decision.execution(),
        GovernanceExecutionDisposition::Proceed
    );
    assert_eq!(
        decision.disclosure(),
        GovernanceDisclosureRequirement::Quiet
    );
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
fn visible_disclosure_does_not_block_execution() {
    let mut input = quiet_input();
    input.evidence_and_checks = GovernancePostureRequirement::visible();

    let decision = select_proportional_governance(input).expect("valid decision");

    assert_eq!(
        decision.execution(),
        GovernanceExecutionDisposition::Proceed
    );
    assert_eq!(
        decision.disclosure(),
        GovernanceDisclosureRequirement::Visible
    );
    assert_eq!(decision.risk_class(), GovernanceRiskClass::ReviewWorthy);
}

#[test]
fn execution_and_disclosure_requirements_compose_independently() {
    let mut input = quiet_input();
    input.workflow = GovernancePostureRequirement::new(
        GovernanceExecutionRequirement::RequireApproval,
        GovernanceDisclosureObligation::QuietAllowed,
    );
    input.policy = GovernancePostureRequirement::visible();

    let decision = select_proportional_governance(input).expect("valid decision");

    assert_eq!(
        decision.execution(),
        GovernanceExecutionDisposition::RequireApproval
    );
    assert_eq!(
        decision.disclosure(),
        GovernanceDisclosureRequirement::Visible
    );
    assert!(decision
        .reasons()
        .contains(&GovernanceDecisionReason::WorkflowRequirement));
    assert!(decision
        .reasons()
        .contains(&GovernanceDecisionReason::PolicyRequirement));
}

#[test]
fn approval_requirement_normalizes_quiet_disclosure_to_visible() {
    let mut input = quiet_input();
    input.workflow = GovernancePostureRequirement::new(
        GovernanceExecutionRequirement::RequireApproval,
        GovernanceDisclosureObligation::QuietAllowed,
    );

    let decision = select_proportional_governance(input).expect("valid decision");

    assert_eq!(
        decision.execution(),
        GovernanceExecutionDisposition::RequireApproval
    );
    assert_eq!(
        decision.disclosure(),
        GovernanceDisclosureRequirement::Visible
    );
}

#[test]
fn denied_requirement_normalizes_quiet_disclosure_to_visible() {
    let mut input = quiet_input();
    input.authority = GovernancePostureRequirement::new(
        GovernanceExecutionRequirement::Denied,
        GovernanceDisclosureObligation::QuietAllowed,
    );

    let decision = select_proportional_governance(input).expect("valid decision");

    assert_eq!(decision.execution(), GovernanceExecutionDisposition::Denied);
    assert_eq!(
        decision.disclosure(),
        GovernanceDisclosureRequirement::Visible
    );
}

#[test]
fn human_approval_profile_cannot_be_downgraded() {
    let mut input = quiet_input();
    input.profile = GovernanceStrictnessProfile::HumanApprovalGated;

    let decision = select_proportional_governance(input).expect("valid decision");

    assert_eq!(
        decision.execution(),
        GovernanceExecutionDisposition::RequireApproval
    );
    assert_eq!(
        decision.disclosure(),
        GovernanceDisclosureRequirement::Visible
    );
    assert_eq!(decision.risk_class(), GovernanceRiskClass::ApprovalRequired);
}

#[test]
fn agent_assisted_profile_allows_quiet_proceed() {
    let mut input = quiet_input();
    input.profile = GovernanceStrictnessProfile::AgentAssistedGated;

    let decision = select_proportional_governance(input).expect("valid decision");

    assert_eq!(
        decision.execution(),
        GovernanceExecutionDisposition::Proceed
    );
    assert_eq!(
        decision.disclosure(),
        GovernanceDisclosureRequirement::Quiet
    );
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
fn strict_enterprise_applies_both_steward_minimum_axes() {
    let mut input = quiet_input();
    input.profile = GovernanceStrictnessProfile::StrictEnterprise;
    input.steward_minimum = Some(GovernancePostureRequirement::visible());

    let decision = select_proportional_governance(input).expect("valid decision");

    assert_eq!(
        decision.execution(),
        GovernanceExecutionDisposition::Proceed
    );
    assert_eq!(
        decision.disclosure(),
        GovernanceDisclosureRequirement::Visible
    );
    assert!(decision
        .reasons()
        .contains(&GovernanceDecisionReason::StewardMinimum));
}

#[test]
fn runtime_change_escalates_both_axes_monotonically() {
    let mut input = quiet_input();
    input.prior_execution = Some(GovernanceExecutionDisposition::RequireApproval);
    input.prior_disclosure = Some(GovernanceDisclosureRequirement::Visible);

    let decision = select_proportional_governance(input).expect("valid decision");

    assert_eq!(
        decision.execution(),
        GovernanceExecutionDisposition::RequireApproval
    );
    assert_eq!(
        decision.disclosure(),
        GovernanceDisclosureRequirement::Visible
    );
    assert!(decision
        .reasons()
        .contains(&GovernanceDecisionReason::PriorDecisionMinimum));
}

#[test]
fn prior_denial_cannot_be_downgraded() {
    let mut input = quiet_input();
    input.prior_execution = Some(GovernanceExecutionDisposition::Denied);

    let decision = select_proportional_governance(input).expect("valid decision");

    assert_eq!(decision.execution(), GovernanceExecutionDisposition::Denied);
    assert_eq!(decision.risk_class(), GovernanceRiskClass::Denied);
}

#[test]
fn explicit_denial_wins_without_suppressing_disclosure() {
    let mut input = quiet_input();
    input.authority = GovernancePostureRequirement::denied();

    let decision = select_proportional_governance(input).expect("valid decision");

    assert_eq!(decision.execution(), GovernanceExecutionDisposition::Denied);
    assert_eq!(
        decision.disclosure(),
        GovernanceDisclosureRequirement::Visible
    );
    assert!(decision
        .reasons()
        .contains(&GovernanceDecisionReason::AuthorityRequirement));
}

#[test]
fn unsupported_execution_or_disclosure_fails_closed_without_payloads() {
    for requirement in [
        GovernancePostureRequirement::new(
            GovernanceExecutionRequirement::Unsupported,
            GovernanceDisclosureObligation::QuietAllowed,
        ),
        GovernancePostureRequirement::new(
            GovernanceExecutionRequirement::Proceed,
            GovernanceDisclosureObligation::Unsupported,
        ),
    ] {
        let mut input = quiet_input();
        input.side_effect = requirement;

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
}

#[test]
fn every_reason_source_is_representable() {
    let visible = GovernancePostureRequirement::visible();
    let mut input = quiet_input();
    input.workflow = visible;
    input.policy = visible;
    input.authority = visible;
    input.evidence_and_checks = visible;
    input.sensitivity = visible;
    input.side_effect = visible;
    input.runtime_escalation = visible;
    input.prior_disclosure = Some(GovernanceDisclosureRequirement::Visible);
    input.steward_minimum = Some(visible);

    let decision = select_proportional_governance(input).expect("valid decision");

    assert_eq!(decision.reasons().len(), 10);
}

#[test]
fn decision_serde_round_trip_preserves_both_axes() {
    let mut input = quiet_input();
    input.policy = GovernancePostureRequirement::approval();
    let decision = select_proportional_governance(input).expect("valid decision");

    let serialized = serde_json::to_string(&decision).expect("serialize");
    let decoded = serde_json::from_str(&serialized).expect("deserialize");

    assert_eq!(decision, decoded);
    assert!(serialized.contains("require_approval"));
    assert!(serialized.contains("visible"));
    assert!(!serialized.contains("payload"));
}

#[test]
fn inconsistent_serialized_decision_fails_closed() {
    let invalid = serde_json::json!({
        "execution": "proceed",
        "disclosure": "quiet",
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
fn serialized_blocking_decision_with_quiet_disclosure_fails_closed() {
    for execution in ["require_approval", "denied"] {
        let invalid = serde_json::json!({
            "execution": execution,
            "disclosure": "quiet",
            "risk_class": if execution == "denied" { "denied" } else { "approval_required" },
            "reasons": ["profile_minimum"]
        });

        let error =
            serde_json::from_value::<workflow_core::ProportionalGovernanceDecision>(invalid)
                .expect_err("blocking decision must require visible disclosure");

        assert_eq!(
            error.to_string(),
            "invalid proportional governance decision"
        );
    }
}

#[test]
fn decision_unknown_axis_values_fail_without_leaking_input() {
    let decision = select_proportional_governance(quiet_input()).expect("valid decision");
    let valid = serde_json::to_value(decision).expect("serialize");
    let secret = "token-sk-governance-axis";

    for field in ["execution", "disclosure", "risk_class"] {
        let mut invalid = valid.clone();
        invalid
            .as_object_mut()
            .expect("decision object")
            .insert(field.to_owned(), secret.into());

        let error =
            serde_json::from_value::<workflow_core::ProportionalGovernanceDecision>(invalid)
                .expect_err("unknown value must fail");

        assert!(!error.to_string().contains(secret));
    }
}

#[test]
fn projection_exposes_action_and_disclosure_independently() {
    let cases = [
        (
            GovernancePostureRequirement::quiet(),
            GovernanceExecutionDisposition::Proceed,
            GovernanceDisclosureRequirement::Quiet,
            GovernanceActionRequirement::None,
        ),
        (
            GovernancePostureRequirement::visible(),
            GovernanceExecutionDisposition::Proceed,
            GovernanceDisclosureRequirement::Visible,
            GovernanceActionRequirement::None,
        ),
        (
            GovernancePostureRequirement::approval(),
            GovernanceExecutionDisposition::RequireApproval,
            GovernanceDisclosureRequirement::Visible,
            GovernanceActionRequirement::Approval,
        ),
        (
            GovernancePostureRequirement::denied(),
            GovernanceExecutionDisposition::Denied,
            GovernanceDisclosureRequirement::Visible,
            GovernanceActionRequirement::Denied,
        ),
    ];

    for (requirement, execution, disclosure, action) in cases {
        let mut input = quiet_input();
        input.workflow = requirement;
        let decision = select_proportional_governance(input).expect("valid decision");
        let projection = project_proportional_governance_decision(&decision);

        assert_eq!(projection.execution(), execution);
        assert_eq!(projection.disclosure(), disclosure);
        assert_eq!(projection.risk_class(), decision.risk_class());
        assert_eq!(projection.reasons(), decision.reasons());
        assert_eq!(projection.action_requirement(), action);
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
fn projection_serde_round_trip_preserves_both_axes() {
    let mut input = quiet_input();
    input.evidence_and_checks = GovernancePostureRequirement::visible();
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
        "execution": "proceed",
        "disclosure": "visible",
        "risk_class": "review_worthy",
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
fn projection_unknown_values_fail_without_leaking_input() {
    let decision = select_proportional_governance(quiet_input()).expect("valid decision");
    let projection = project_proportional_governance_decision(&decision);
    let valid = serde_json::to_value(projection).expect("serialize");
    let secret = "token-sk-projection-secret";

    for field in [
        "execution",
        "disclosure",
        "risk_class",
        "action_requirement",
        "decision_posture",
        "persistence_posture",
    ] {
        let mut invalid = valid.clone();
        invalid
            .as_object_mut()
            .expect("projection object")
            .insert(field.to_owned(), secret.into());

        let error = serde_json::from_value::<ProportionalGovernanceDecisionProjection>(invalid)
            .expect_err("unknown projection value must fail");

        assert!(!error.to_string().contains(secret));
    }
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
