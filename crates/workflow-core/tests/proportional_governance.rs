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

    let decision = select_proportional_governance(input).expect("valid decision");

    assert_eq!(decision.reasons().len(), 9);
}
