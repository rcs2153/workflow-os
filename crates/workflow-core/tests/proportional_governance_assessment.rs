#![allow(clippy::expect_used)]

//! Deterministic proportional-governance workload-assessment tests.

use workflow_core::{
    assess_proportional_governance_workload, GovernanceAssessmentCompleteness,
    GovernanceAssessmentUnknownFact, GovernanceDecisionPosture, GovernanceDisclosureRequirement,
    GovernanceExecutionDisposition, GovernancePersistencePosture, GovernancePostureRequirement,
    GovernanceStrictnessProfile, GovernanceWorkloadActionClass,
    GovernanceWorkloadAssessmentAlgorithm, GovernanceWorkloadAuthorityPosture,
    GovernanceWorkloadEvidenceCheckPosture, GovernanceWorkloadSensitivity,
    GovernanceWorkloadSideEffectPosture, ProportionalGovernanceWorkloadAssessmentInput,
    SpecContentHash, WorkflowOsErrorKind,
};

fn bounded_read_input() -> ProportionalGovernanceWorkloadAssessmentInput {
    ProportionalGovernanceWorkloadAssessmentInput {
        definition_root: SpecContentHash::from_text("immutable definition root"),
        profile: GovernanceStrictnessProfile::ObserveAndReport,
        workflow_minimum: GovernancePostureRequirement::quiet(),
        policy_minimum: GovernancePostureRequirement::quiet(),
        action_class: GovernanceWorkloadActionClass::ReadOnly,
        authority: GovernanceWorkloadAuthorityPosture::Sufficient,
        authority_minimum: GovernancePostureRequirement::quiet(),
        evidence_and_checks: GovernanceWorkloadEvidenceCheckPosture::Satisfied,
        evidence_and_check_minimum: GovernancePostureRequirement::quiet(),
        sensitivity: GovernanceWorkloadSensitivity::Routine,
        sensitivity_minimum: GovernancePostureRequirement::quiet(),
        side_effect: GovernanceWorkloadSideEffectPosture::None,
        side_effect_minimum: GovernancePostureRequirement::quiet(),
        runtime_escalation: GovernancePostureRequirement::quiet(),
        prior_execution: None,
        prior_disclosure: None,
        steward_minimum: None,
    }
}

#[test]
fn bounded_read_only_workload_is_complete_and_quiet() {
    let input = bounded_read_input();
    let assessment =
        assess_proportional_governance_workload(&input).expect("assessment should succeed");

    assert_eq!(
        assessment.decision().execution(),
        GovernanceExecutionDisposition::Proceed
    );
    assert_eq!(
        assessment.decision().disclosure(),
        GovernanceDisclosureRequirement::Quiet
    );
    assert_eq!(
        assessment.completeness(),
        GovernanceAssessmentCompleteness::Complete
    );
    assert!(assessment.unknown_facts().is_empty());
    assert_eq!(
        assessment.algorithm(),
        GovernanceWorkloadAssessmentAlgorithm::V1
    );
    assert_eq!(
        assessment.decision_posture(),
        GovernanceDecisionPosture::AssessedNotEnforced
    );
    assert_eq!(
        assessment.persistence_posture(),
        GovernancePersistencePosture::NotPersisted
    );
}

#[test]
fn local_mutation_requires_disclosure_without_blocking() {
    let mut input = bounded_read_input();
    input.action_class = GovernanceWorkloadActionClass::LocalMutation;
    input.side_effect = GovernanceWorkloadSideEffectPosture::LocalReversible;

    let assessment =
        assess_proportional_governance_workload(&input).expect("assessment should succeed");

    assert_eq!(
        assessment.decision().execution(),
        GovernanceExecutionDisposition::Proceed
    );
    assert_eq!(
        assessment.decision().disclosure(),
        GovernanceDisclosureRequirement::Visible
    );
}

#[test]
fn external_mutation_requires_approval() {
    let mut input = bounded_read_input();
    input.action_class = GovernanceWorkloadActionClass::ExternalMutation;
    input.side_effect = GovernanceWorkloadSideEffectPosture::ExternalReversible;

    let assessment =
        assess_proportional_governance_workload(&input).expect("assessment should succeed");

    assert_eq!(
        assessment.decision().execution(),
        GovernanceExecutionDisposition::RequireApproval
    );
    assert_eq!(
        assessment.decision().disclosure(),
        GovernanceDisclosureRequirement::Visible
    );
    assert!(assessment
        .decision()
        .reasons()
        .contains(&workflow_core::GovernanceDecisionReason::WorkloadAssessment));
    assert!(!assessment
        .decision()
        .reasons()
        .contains(&workflow_core::GovernanceDecisionReason::WorkflowRequirement));
}

#[test]
fn explicit_workflow_minimum_retains_workflow_reason_provenance() {
    let mut input = bounded_read_input();
    input.workflow_minimum = GovernancePostureRequirement::approval();

    let assessment =
        assess_proportional_governance_workload(&input).expect("assessment should succeed");

    assert!(assessment
        .decision()
        .reasons()
        .contains(&workflow_core::GovernanceDecisionReason::WorkflowRequirement));
    assert!(!assessment
        .decision()
        .reasons()
        .contains(&workflow_core::GovernanceDecisionReason::WorkloadAssessment));
}

#[test]
fn failed_or_missing_required_checks_deny_instead_of_requesting_approval() {
    for posture in [
        GovernanceWorkloadEvidenceCheckPosture::RequiredUnavailable,
        GovernanceWorkloadEvidenceCheckPosture::Failed,
    ] {
        let mut input = bounded_read_input();
        input.evidence_and_checks = posture;

        let assessment =
            assess_proportional_governance_workload(&input).expect("assessment should succeed");

        assert_eq!(
            assessment.decision().execution(),
            GovernanceExecutionDisposition::Denied
        );
    }
}

#[test]
fn unavailable_authority_and_ambiguous_side_effect_deny() {
    let mut unavailable_authority = bounded_read_input();
    unavailable_authority.authority = GovernanceWorkloadAuthorityPosture::Unavailable;
    assert_eq!(
        assess_proportional_governance_workload(&unavailable_authority)
            .expect("assessment should succeed")
            .decision()
            .execution(),
        GovernanceExecutionDisposition::Denied
    );

    let mut ambiguous_side_effect = bounded_read_input();
    ambiguous_side_effect.side_effect = GovernanceWorkloadSideEffectPosture::Ambiguous;
    assert_eq!(
        assess_proportional_governance_workload(&ambiguous_side_effect)
            .expect("assessment should succeed")
            .decision()
            .execution(),
        GovernanceExecutionDisposition::Denied
    );
}

#[test]
fn unknown_safety_facts_are_explicit_and_conservatively_block_for_approval() {
    let mut input = bounded_read_input();
    input.action_class = GovernanceWorkloadActionClass::Unknown;
    input.authority = GovernanceWorkloadAuthorityPosture::Unknown;
    input.evidence_and_checks = GovernanceWorkloadEvidenceCheckPosture::Unknown;
    input.sensitivity = GovernanceWorkloadSensitivity::Unknown;
    input.side_effect = GovernanceWorkloadSideEffectPosture::Unknown;

    let assessment =
        assess_proportional_governance_workload(&input).expect("assessment should succeed");

    assert_eq!(
        assessment.decision().execution(),
        GovernanceExecutionDisposition::RequireApproval
    );
    assert_eq!(
        assessment.completeness(),
        GovernanceAssessmentCompleteness::Incomplete
    );
    assert_eq!(
        assessment
            .unknown_facts()
            .iter()
            .copied()
            .collect::<Vec<_>>(),
        vec![
            GovernanceAssessmentUnknownFact::ActionClass,
            GovernanceAssessmentUnknownFact::Authority,
            GovernanceAssessmentUnknownFact::EvidenceAndChecks,
            GovernanceAssessmentUnknownFact::Sensitivity,
            GovernanceAssessmentUnknownFact::SideEffect,
        ]
    );
}

#[test]
fn explicit_minima_cannot_be_downgraded_by_quiet_inference() {
    let minimums = [
        |input: &mut ProportionalGovernanceWorkloadAssessmentInput| {
            input.workflow_minimum = GovernancePostureRequirement::approval();
        },
        |input: &mut ProportionalGovernanceWorkloadAssessmentInput| {
            input.policy_minimum = GovernancePostureRequirement::approval();
        },
        |input: &mut ProportionalGovernanceWorkloadAssessmentInput| {
            input.authority_minimum = GovernancePostureRequirement::approval();
        },
        |input: &mut ProportionalGovernanceWorkloadAssessmentInput| {
            input.evidence_and_check_minimum = GovernancePostureRequirement::approval();
        },
        |input: &mut ProportionalGovernanceWorkloadAssessmentInput| {
            input.sensitivity_minimum = GovernancePostureRequirement::approval();
        },
        |input: &mut ProportionalGovernanceWorkloadAssessmentInput| {
            input.side_effect_minimum = GovernancePostureRequirement::approval();
        },
        |input: &mut ProportionalGovernanceWorkloadAssessmentInput| {
            input.runtime_escalation = GovernancePostureRequirement::approval();
        },
    ];

    for apply_minimum in minimums {
        let mut input = bounded_read_input();
        apply_minimum(&mut input);
        let assessment =
            assess_proportional_governance_workload(&input).expect("assessment should succeed");
        assert_eq!(
            assessment.decision().execution(),
            GovernanceExecutionDisposition::RequireApproval
        );
    }
}

#[test]
fn profile_prior_decision_and_steward_minimum_remain_authoritative() {
    let mut profile = bounded_read_input();
    profile.profile = GovernanceStrictnessProfile::HumanApprovalGated;
    assert_eq!(
        assess_proportional_governance_workload(&profile)
            .expect("assessment should succeed")
            .decision()
            .execution(),
        GovernanceExecutionDisposition::RequireApproval
    );

    let mut prior = bounded_read_input();
    prior.prior_execution = Some(GovernanceExecutionDisposition::Denied);
    assert_eq!(
        assess_proportional_governance_workload(&prior)
            .expect("assessment should succeed")
            .decision()
            .execution(),
        GovernanceExecutionDisposition::Denied
    );

    let mut enterprise = bounded_read_input();
    enterprise.profile = GovernanceStrictnessProfile::StrictEnterprise;
    enterprise.steward_minimum = Some(GovernancePostureRequirement::approval());
    assert_eq!(
        assess_proportional_governance_workload(&enterprise)
            .expect("assessment should succeed")
            .decision()
            .execution(),
        GovernanceExecutionDisposition::RequireApproval
    );
}

#[test]
fn strict_enterprise_without_steward_minimum_fails_with_stable_error() {
    let mut input = bounded_read_input();
    input.profile = GovernanceStrictnessProfile::StrictEnterprise;

    let error = assess_proportional_governance_workload(&input)
        .expect_err("strict enterprise must fail closed");

    assert_eq!(error.kind(), WorkflowOsErrorKind::InvalidState);
    assert_eq!(
        error.code(),
        "governance.proportional.steward_minimum.required"
    );
}

#[test]
fn identical_inputs_have_identical_decisions_and_fingerprints() {
    let input = bounded_read_input();
    let first = assess_proportional_governance_workload(&input).expect("assessment should succeed");
    let second =
        assess_proportional_governance_workload(&input).expect("assessment should succeed");

    assert_eq!(first, second);
    assert_eq!(first.input_fingerprint(), second.input_fingerprint());
}

#[test]
fn v1_fingerprint_has_a_fixed_known_vector() {
    let input = bounded_read_input();
    let assessment =
        assess_proportional_governance_workload(&input).expect("assessment should succeed");

    assert_eq!(
        assessment.input_fingerprint().as_str(),
        "77748614823b70948a582a314fe9059152eaf21f948d28357bae8a930d28d25c"
    );
}

#[test]
fn every_decision_relevant_fact_family_invalidates_the_fingerprint() {
    let baseline_input = bounded_read_input();
    let baseline = assess_proportional_governance_workload(&baseline_input)
        .expect("assessment should succeed")
        .input_fingerprint()
        .clone();

    let variants = [
        {
            let mut input = bounded_read_input();
            input.definition_root = SpecContentHash::from_text("changed definition root");
            input
        },
        {
            let mut input = bounded_read_input();
            input.action_class = GovernanceWorkloadActionClass::LocalMutation;
            input
        },
        {
            let mut input = bounded_read_input();
            input.profile = GovernanceStrictnessProfile::AgentAssistedGated;
            input
        },
        {
            let mut input = bounded_read_input();
            input.workflow_minimum = GovernancePostureRequirement::visible();
            input
        },
        {
            let mut input = bounded_read_input();
            input.authority = GovernanceWorkloadAuthorityPosture::ApprovalRequired;
            input
        },
        {
            let mut input = bounded_read_input();
            input.evidence_and_checks = GovernanceWorkloadEvidenceCheckPosture::OptionalUnavailable;
            input
        },
        {
            let mut input = bounded_read_input();
            input.sensitivity = GovernanceWorkloadSensitivity::Elevated;
            input
        },
        {
            let mut input = bounded_read_input();
            input.side_effect = GovernanceWorkloadSideEffectPosture::LocalReversible;
            input
        },
        {
            let mut input = bounded_read_input();
            input.policy_minimum = GovernancePostureRequirement::visible();
            input
        },
        {
            let mut input = bounded_read_input();
            input.authority_minimum = GovernancePostureRequirement::visible();
            input
        },
        {
            let mut input = bounded_read_input();
            input.evidence_and_check_minimum = GovernancePostureRequirement::visible();
            input
        },
        {
            let mut input = bounded_read_input();
            input.sensitivity_minimum = GovernancePostureRequirement::visible();
            input
        },
        {
            let mut input = bounded_read_input();
            input.side_effect_minimum = GovernancePostureRequirement::visible();
            input
        },
        {
            let mut input = bounded_read_input();
            input.runtime_escalation = GovernancePostureRequirement::visible();
            input
        },
        {
            let mut input = bounded_read_input();
            input.prior_execution = Some(GovernanceExecutionDisposition::RequireApproval);
            input
        },
        {
            let mut input = bounded_read_input();
            input.prior_disclosure = Some(GovernanceDisclosureRequirement::Visible);
            input
        },
        {
            let mut input = bounded_read_input();
            input.steward_minimum = Some(GovernancePostureRequirement::visible());
            input
        },
    ];

    for variant in variants {
        let fingerprint = assess_proportional_governance_workload(&variant)
            .expect("assessment should succeed")
            .input_fingerprint()
            .clone();
        assert_ne!(fingerprint, baseline);
    }
}

#[test]
fn presentation_is_not_an_assessment_input_or_execution_authority() {
    let mut input = bounded_read_input();
    input.policy_minimum = GovernancePostureRequirement::visible();

    let assessment =
        assess_proportional_governance_workload(&input).expect("assessment should succeed");

    assert_eq!(
        assessment.decision().execution(),
        GovernanceExecutionDisposition::Proceed
    );
    assert_eq!(
        assessment.decision().disclosure(),
        GovernanceDisclosureRequirement::Visible
    );
}

#[test]
fn debug_and_serialization_are_payload_free_and_debug_redacts_hashes() {
    let secret_markers = [
        "ghp_super_secret",
        "Authorization: Bearer",
        "raw provider payload",
        "raw command output",
        "private key",
    ];
    let input = bounded_read_input();
    let assessment =
        assess_proportional_governance_workload(&input).expect("assessment should succeed");

    let input_debug = format!("{input:?}");
    let assessment_debug = format!("{assessment:?}");
    let serialized = serde_json::to_string(&assessment).expect("serialization should succeed");

    assert!(input_debug.contains("<redacted>"));
    assert!(assessment_debug.contains("<redacted>"));
    assert!(!input_debug.contains(input.definition_root.as_str()));
    assert!(!assessment_debug.contains(assessment.input_fingerprint().as_str()));
    for marker in secret_markers {
        assert!(!input_debug.contains(marker));
        assert!(!assessment_debug.contains(marker));
        assert!(!serialized.contains(marker));
    }
}
