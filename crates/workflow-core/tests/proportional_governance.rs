#![allow(clippy::expect_used)]

//! Proportional-governance core decision model tests.

use workflow_core::{
    select_proportional_governance, GovernanceDecisionReason, GovernanceInteractionMode,
    GovernancePostureRequirement, GovernanceRiskClass, GovernanceStrictnessProfile,
    ProportionalGovernanceDecisionInput, WorkflowOsErrorKind,
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
