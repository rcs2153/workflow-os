//! Tests for governed workflow authoring steward-review helpers.

use workflow_core::{
    review_workflow_draft_for_promotion, ActorId, SpecContentHash,
    WorkflowDraftPromotionPreflightStatus, WorkflowDraftStewardReviewAuthorization,
    WorkflowDraftStewardReviewDecision, WorkflowDraftStewardReviewInput, WorkflowId,
    WorkflowOsError,
};

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn steward_review_err(input: WorkflowDraftStewardReviewInput) -> Result<WorkflowOsError, String> {
    match review_workflow_draft_for_promotion(input) {
        Ok(result) => Err(format!("steward review should fail, returned: {result:?}")),
        Err(error) => Ok(error),
    }
}

fn valid_input() -> Result<WorkflowDraftStewardReviewInput, WorkflowOsError> {
    let hash = SpecContentHash::from_text("bounded workflow draft");
    Ok(WorkflowDraftStewardReviewInput {
        draft_path: "workflows/drafts/repo-implementation.workflow.yml".to_owned(),
        candidate_workflow_id: WorkflowId::new("local/repo-implementation")?,
        preflight_draft_content_hash: hash.clone(),
        current_draft_content_hash: hash,
        preflight_status: WorkflowDraftPromotionPreflightStatus::Passed,
        preflight_blockers: Vec::new(),
        preflight_warnings: vec![
            "steward_approval_required_before_active_promotion".to_owned(),
            "side_effect_and_report_posture_requires_review".to_owned(),
        ],
        owner_summary: "owner and maintainer are assigned".to_owned(),
        escalation_summary: "escalation contact is assigned".to_owned(),
        policy_summary: "declared policy gates reviewed".to_owned(),
        evidence_report_summary: "evidence and report posture reviewed".to_owned(),
        side_effect_summary: "side effects unsupported for this draft".to_owned(),
        active_workflow_conflict: false,
        reviewer: ActorId::new("user/workflow-steward")?,
        decision: WorkflowDraftStewardReviewDecision::ApprovedForPromotion,
        approval_reason: "bounded steward approval reason".to_owned(),
    })
}

#[test]
fn approved_preflight_passing_draft_authorizes_future_promotion_without_mutation() -> TestResult {
    let result = review_workflow_draft_for_promotion(valid_input()?)?;

    assert_eq!(
        result.authorization(),
        WorkflowDraftStewardReviewAuthorization::AuthorizedForPromotion
    );
    assert_eq!(
        result.decision(),
        WorkflowDraftStewardReviewDecision::ApprovedForPromotion
    );
    assert_eq!(
        result.card().draft_path(),
        "workflows/drafts/repo-implementation.workflow.yml"
    );
    assert_eq!(
        result.card().candidate_workflow_id().as_str(),
        "local/repo-implementation"
    );
    assert_eq!(result.card().preflight_warnings().len(), 2);
    assert_eq!(
        result.card().owner_summary(),
        "owner and maintainer are assigned"
    );
    assert_eq!(
        result.card().approval_allows(),
        "future promotion of this exact unchanged draft through a separately implemented promotion step"
    );

    let boundary = result.boundary();
    assert!(!boundary.files_written);
    assert!(!boundary.workflow_registered);
    assert!(!boundary.workflow_promoted);
    assert!(!boundary.approval_persisted);
    assert!(!boundary.runtime_state_created);
    assert!(!boundary.commands_executed);
    assert!(!boundary.providers_called);

    Ok(())
}

#[test]
fn non_approval_decisions_do_not_authorize_promotion() -> TestResult {
    for decision in [
        WorkflowDraftStewardReviewDecision::Denied,
        WorkflowDraftStewardReviewDecision::NeedsChanges,
        WorkflowDraftStewardReviewDecision::Deferred,
    ] {
        let mut input = valid_input()?;
        input.decision = decision;

        let result = review_workflow_draft_for_promotion(input)?;

        assert_eq!(
            result.authorization(),
            WorkflowDraftStewardReviewAuthorization::NotAuthorized
        );
        assert_eq!(result.decision(), decision);
    }

    Ok(())
}

#[test]
fn preflight_blockers_fail_closed_without_leaking_blocker_payload() -> TestResult {
    let mut input = valid_input()?;
    input.preflight_status = WorkflowDraftPromotionPreflightStatus::Blocked;
    input.preflight_blockers = vec!["validation.workflow.steps_missing".to_owned()];

    let error = steward_review_err(input)?;

    assert_eq!(
        error.code(),
        "workflow_authoring.steward_review.preflight_blocked"
    );
    assert!(!format!("{error:?}").contains("validation.workflow.steps_missing"));

    Ok(())
}

#[test]
fn stale_preflight_hash_fails_closed() -> TestResult {
    let mut input = valid_input()?;
    input.current_draft_content_hash = SpecContentHash::from_text("changed workflow draft");

    let error = steward_review_err(input)?;

    assert_eq!(
        error.code(),
        "workflow_authoring.steward_review.stale_preflight"
    );

    Ok(())
}

#[test]
fn active_workflow_conflict_fails_closed() -> TestResult {
    let mut input = valid_input()?;
    input.active_workflow_conflict = true;

    let error = steward_review_err(input)?;

    assert_eq!(
        error.code(),
        "workflow_authoring.steward_review.active_conflict"
    );

    Ok(())
}

#[test]
fn unsafe_or_secret_like_inputs_are_rejected_without_leaking_values() -> TestResult {
    let mut input = valid_input()?;
    input.draft_path = "../secret-token.workflow.yml".to_owned();

    let error = steward_review_err(input)?;

    assert_eq!(
        error.code(),
        "workflow_authoring.steward_review.secret_like_value"
    );
    assert!(!format!("{error:?}").contains("secret-token"));

    let mut input = valid_input()?;
    input.approval_reason = "contains sk-secret-token".to_owned();

    let error = steward_review_err(input)?;

    assert_eq!(
        error.code(),
        "workflow_authoring.steward_review.secret_like_value"
    );
    assert!(!format!("{error:?}").contains("sk-secret-token"));

    Ok(())
}

#[test]
fn debug_output_redacts_bounded_review_text() -> TestResult {
    let input = valid_input()?;
    let result = review_workflow_draft_for_promotion(input.clone())?;

    let input_debug = format!("{input:?}");
    let result_debug = format!("{result:?}");

    assert!(!input_debug.contains("bounded steward approval reason"));
    assert!(!result_debug.contains("bounded steward approval reason"));
    assert!(!result_debug.contains("owner and maintainer are assigned"));
    assert!(!result_debug.contains("evidence and report posture reviewed"));
    assert!(result_debug.contains("WorkflowDraftStewardReviewResult"));

    Ok(())
}
