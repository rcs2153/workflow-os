#![allow(clippy::expect_used)]
//! Conservative v0 policy engine tests.

use workflow_core::{
    Action, ActorId, ApprovalSensitivity, AutonomyLevel, Capability, ConservativePolicyEngine,
    CorrelationId, PolicyEvaluationContext, WorkflowId, WorkflowRunId,
};

fn context(action: Action, capabilities: Vec<Capability>) -> PolicyEvaluationContext {
    PolicyEvaluationContext {
        action,
        capabilities,
        actor: Some(ActorId::new("system/policy-test").expect("actor")),
        workflow_id: Some(WorkflowId::new("workflow/policy-test").expect("workflow")),
        run_id: Some(WorkflowRunId::new("run/policy-test").expect("run")),
        step_id: None,
        skill_id: None,
        autonomy_level: Some(AutonomyLevel::Level1Assistive),
        approval_sensitivity: Some(ApprovalSensitivity::Low),
        has_approval_policy: false,
        adapter_id: None,
        correlation_id: Some(CorrelationId::new("correlation/policy-test").expect("correlation")),
    }
}

#[test]
fn allowed_local_skill_action() {
    let decision = ConservativePolicyEngine::new().evaluate(&context(
        Action::InvokeSkill,
        vec![
            Capability::LocalRead,
            Capability::LocalWrite,
            Capability::AuditWrite,
        ],
    ));

    assert!(decision.allowed);
    assert!(decision.violations.is_empty());
}

#[test]
fn denied_unknown_capability() {
    let decision = ConservativePolicyEngine::new().evaluate(&context(
        Action::InvokeSkill,
        vec![Capability::Unknown("mystery.power".to_owned())],
    ));

    assert!(!decision.allowed);
    assert!(decision
        .reason_codes
        .contains(&"policy.deny.unknown_capability".to_owned()));
}

#[test]
fn approval_required_for_sensitive_action() {
    let mut sensitive = context(Action::InvokeSkill, vec![Capability::LocalWrite]);
    sensitive.approval_sensitivity = Some(ApprovalSensitivity::High);

    let decision = ConservativePolicyEngine::new().evaluate(&sensitive);

    assert!(!decision.allowed);
    assert!(decision.requires_approval);
    assert!(decision
        .reason_codes
        .contains(&"policy.deny.approval_required".to_owned()));
}

#[test]
fn level_3_and_4_are_denied_by_default() {
    let mut level3 = context(Action::StartWorkflow, vec![Capability::LocalRead]);
    level3.autonomy_level = Some(AutonomyLevel::Level3ConditionalAutonomy);

    let decision = ConservativePolicyEngine::new().evaluate(&level3);

    assert!(!decision.allowed);
    assert!(decision
        .reason_codes
        .contains(&"policy.deny.autonomy_level".to_owned()));
}

#[test]
fn external_write_is_denied_in_v0() {
    let decision = ConservativePolicyEngine::new().evaluate(&context(
        Action::InvokeAdapter,
        vec![Capability::ExternalWrite],
    ));

    assert!(!decision.allowed);
    assert!(decision
        .reason_codes
        .contains(&"policy.deny.external_write_v0".to_owned()));
}

#[test]
fn missing_context_fails_closed() {
    let mut missing = context(Action::InvokeSkill, vec![Capability::LocalRead]);
    missing.actor = None;
    missing.correlation_id = None;

    let decision = ConservativePolicyEngine::new().evaluate(&missing);

    assert!(!decision.allowed);
    assert!(decision
        .reason_codes
        .contains(&"policy.deny.missing_actor".to_owned()));
    assert!(decision
        .reason_codes
        .contains(&"policy.deny.missing_correlation".to_owned()));
}

#[test]
fn kill_switch_allows_cancel_but_denies_execution() {
    let engine = ConservativePolicyEngine::kill_switch();

    let start = engine.evaluate(&context(Action::StartWorkflow, vec![Capability::LocalRead]));
    let cancel = engine.evaluate(&context(
        Action::CancelWorkflow,
        vec![Capability::WorkflowCancel],
    ));

    assert!(!start.allowed);
    assert!(start
        .reason_codes
        .contains(&"policy.deny.kill_switch".to_owned()));
    assert!(cancel.allowed);
}
