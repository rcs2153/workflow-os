#![allow(clippy::expect_used)]

//! Provider write request/response model tests.

use serde_json::json;
use workflow_core::{
    compose_github_pr_comment_proposed_side_effect_record, github_pr_comment_preflight_definition,
    validate_github_pr_comment_fixture_write, ActorId, AdapterId, AdapterWritePolicyDecision,
    AdapterWritePreflightRequest, CorrelationId, GitHubPullRequestCommentFixture,
    GitHubPullRequestCommentFixtureDefinition, GitHubPullRequestCommentPreflightDefinitionInput,
    GitHubPullRequestCommentPreflightedWrite, GitHubPullRequestCommentSideEffectRecordInput,
    GitHubPullRequestCommentTarget, GitHubPullRequestCommentWriteMode,
    GitHubPullRequestCommentWriteOutcome, GitHubPullRequestCommentWriteRequest,
    GitHubPullRequestCommentWriteRequestDefinition, GitHubPullRequestCommentWriteResponse,
    GitHubPullRequestCommentWriteResponseDefinition, IdempotencyKey, IntegrationId,
    RedactionDisposition, RedactionFieldState, RedactionMetadata, SchemaVersion,
    SideEffectAuthorityDecision, SideEffectCapability, SideEffectId, SideEffectLifecycleState,
    SideEffectReference, SideEffectReferenceKind, SideEffectSensitivity, SideEffectTargetKind,
    SpecContentHash, StepId, Timestamp, WorkflowId, WorkflowRunId, WorkflowVersion,
};

fn target() -> GitHubPullRequestCommentTarget {
    GitHubPullRequestCommentTarget::new("workflow-os", "kernel", 42).expect("valid target")
}

fn side_effect_id() -> SideEffectId {
    SideEffectId::new("side-effect/github-pr-comment").expect("valid side-effect id")
}

fn idempotency_key() -> IdempotencyKey {
    IdempotencyKey::new("github-pr-comment-42").expect("valid idempotency key")
}

fn policy_ref() -> SideEffectReference {
    SideEffectReference::new(
        SideEffectReferenceKind::PolicyDecision,
        "event/policy-github-comment-allowed",
    )
    .expect("valid policy reference")
}

fn approval_ref() -> SideEffectReference {
    SideEffectReference::new(
        SideEffectReferenceKind::ApprovalDecision,
        "approval/github-comment-approved",
    )
    .expect("valid approval reference")
}

fn redaction() -> RedactionMetadata {
    RedactionMetadata {
        redacted_fields: vec!["comment_body".to_owned()],
        field_states: vec![RedactionFieldState {
            field: "comment_body".to_owned(),
            disposition: RedactionDisposition::ReferenceOnly,
            reason: "bounded comment text only".to_owned(),
        }],
    }
}

fn valid_request_definition() -> GitHubPullRequestCommentWriteRequestDefinition {
    let target = target();
    let side_effect_id = side_effect_id();
    let idempotency_key = idempotency_key();
    let preflight =
        github_pr_comment_preflight_definition(GitHubPullRequestCommentPreflightDefinitionInput {
            target: target.clone(),
            side_effect_id: side_effect_id.clone(),
            idempotency_key: idempotency_key.clone(),
            policy_decision: AdapterWritePolicyDecision::Allowed,
            policy_references: vec![policy_ref()],
            approval_references: vec![approval_ref()],
            summary: "bounded GitHub PR comment write preflight summary".to_owned(),
            sensitivity: SideEffectSensitivity::Internal,
            redaction: redaction(),
        })
        .and_then(workflow_core::AdapterWritePreflightRequest::new)
        .expect("valid preflight request");

    GitHubPullRequestCommentWriteRequestDefinition {
        adapter_id: AdapterId::new("adapter/github").expect("valid adapter id"),
        integration_id: IntegrationId::new("integration/github/sandbox")
            .expect("valid integration id"),
        correlation_id: CorrelationId::new("correlation/github-pr-comment")
            .expect("valid correlation id"),
        workflow_id: WorkflowId::new("workflow/write-candidate").expect("valid workflow id"),
        workflow_version: WorkflowVersion::new("v1").expect("valid workflow version"),
        schema_version: SchemaVersion::new("workflowos.dev/v0").expect("valid schema version"),
        spec_hash: SpecContentHash::from_text("write-candidate-spec"),
        run_id: WorkflowRunId::new("run/github-pr-comment").expect("valid run id"),
        step_id: Some(StepId::new("step/comment").expect("valid step id")),
        actor: ActorId::new("system/workflow-os").expect("valid actor"),
        target,
        comment_body: "Workflow OS governed comment preview.".to_owned(),
        summary: "bounded GitHub PR comment write request summary".to_owned(),
        side_effect_id,
        idempotency_key,
        mode: GitHubPullRequestCommentWriteMode::Fixture,
        preflight,
        sensitivity: SideEffectSensitivity::Internal,
        redaction: redaction(),
    }
}

fn valid_request() -> GitHubPullRequestCommentWriteRequest {
    GitHubPullRequestCommentWriteRequest::new(valid_request_definition()).expect("valid request")
}

fn preflighted_write() -> GitHubPullRequestCommentPreflightedWrite {
    GitHubPullRequestCommentPreflightedWrite::new(valid_request()).expect("valid preflighted write")
}

fn valid_fixture_definition() -> GitHubPullRequestCommentFixtureDefinition {
    GitHubPullRequestCommentFixtureDefinition {
        target: target(),
        side_effect_id: side_effect_id(),
        idempotency_key: idempotency_key(),
        mode: GitHubPullRequestCommentWriteMode::Fixture,
        fixture_reference: Some("fixture/github-pr-comment-42".to_owned()),
        summary: "fixture request validated without provider call".to_owned(),
        sensitivity: SideEffectSensitivity::Internal,
        redaction: redaction(),
    }
}

fn valid_fixture() -> GitHubPullRequestCommentFixture {
    GitHubPullRequestCommentFixture::new(valid_fixture_definition()).expect("valid fixture")
}

fn side_effect_record_input() -> GitHubPullRequestCommentSideEffectRecordInput {
    GitHubPullRequestCommentSideEffectRecordInput {
        created_at: Timestamp::parse_rfc3339("2026-06-20T12:00:00Z").expect("valid timestamp"),
        skill_id: None,
        skill_version: None,
        system_actor: None,
        additional_references: vec![SideEffectReference::new(
            SideEffectReferenceKind::EvidenceReference,
            "evidence/github-pr-comment-fixture",
        )
        .expect("valid reference")],
        summary_override: None,
        sensitivity: None,
    }
}

fn valid_response_definition() -> GitHubPullRequestCommentWriteResponseDefinition {
    GitHubPullRequestCommentWriteResponseDefinition {
        correlation_id: CorrelationId::new("correlation/github-pr-comment")
            .expect("valid correlation id"),
        mode: GitHubPullRequestCommentWriteMode::Fixture,
        outcome: GitHubPullRequestCommentWriteOutcome::FixtureValidated,
        provider_comment_reference: None,
        provider_error_code: None,
        summary: "fixture request validated without provider call".to_owned(),
        sensitivity: SideEffectSensitivity::Internal,
        redaction: redaction(),
    }
}

#[test]
fn valid_github_pr_comment_request_is_model_only_and_validated() {
    let request = valid_request();

    assert_eq!(request.mode(), GitHubPullRequestCommentWriteMode::Fixture);
    assert_eq!(
        request.target().reference(),
        "github/workflow-os/kernel/pull/42"
    );
    assert_eq!(
        request.comment_body(),
        "Workflow OS governed comment preview."
    );
    assert_eq!(
        request.side_effect_id().as_str(),
        "side-effect/github-pr-comment"
    );
    assert_eq!(request.idempotency_key().as_str(), "github-pr-comment-42");
    assert!(!request.provider_call_allowed());
    assert!(!request.workflow_event_append_allowed());
    assert!(!request.side_effect_lifecycle_transition_allowed());
}

#[test]
fn valid_github_pr_comment_request_composes_with_executed_preflight() {
    let preflighted = preflighted_write();

    assert_eq!(
        preflighted.preflight_decision().capability(),
        workflow_core::AdapterWriteCapability::GitHubPullRequestComment
    );
    assert_eq!(
        preflighted.preflight_decision().side_effect_id(),
        preflighted.request().side_effect_id()
    );
    assert_eq!(
        preflighted.preflight_decision().idempotency_key(),
        preflighted.request().idempotency_key()
    );
    assert!(!preflighted.provider_call_allowed());
    assert!(!preflighted.workflow_event_append_allowed());
    assert!(!preflighted.side_effect_lifecycle_transition_allowed());
    assert!(!preflighted.report_artifact_write_allowed());
}

#[test]
fn preflight_composition_calls_preflight_and_rejects_denied_policy() {
    let mut definition = valid_request_definition();
    let mut preflight_definition =
        github_pr_comment_preflight_definition(GitHubPullRequestCommentPreflightDefinitionInput {
            target: definition.target.clone(),
            side_effect_id: definition.side_effect_id.clone(),
            idempotency_key: definition.idempotency_key.clone(),
            policy_decision: AdapterWritePolicyDecision::Denied,
            policy_references: vec![policy_ref()],
            approval_references: vec![approval_ref()],
            summary: "bounded denied GitHub PR comment write preflight summary".to_owned(),
            sensitivity: SideEffectSensitivity::Internal,
            redaction: redaction(),
        })
        .expect("preflight definition");
    preflight_definition.policy_decision = AdapterWritePolicyDecision::Denied;
    definition.preflight =
        AdapterWritePreflightRequest::new(preflight_definition).expect("constructable preflight");
    let request = GitHubPullRequestCommentWriteRequest::new(definition).expect("valid request");

    let error = GitHubPullRequestCommentPreflightedWrite::new(request)
        .expect_err("denied policy fails composition");

    assert_eq!(error.code(), "adapter_write_preflight.policy.denied");
}

#[test]
fn preflight_composition_rejects_missing_required_approval() {
    let mut definition = valid_request_definition();
    let mut preflight_definition =
        github_pr_comment_preflight_definition(GitHubPullRequestCommentPreflightDefinitionInput {
            target: definition.target.clone(),
            side_effect_id: definition.side_effect_id.clone(),
            idempotency_key: definition.idempotency_key.clone(),
            policy_decision: AdapterWritePolicyDecision::Allowed,
            policy_references: vec![policy_ref()],
            approval_references: Vec::new(),
            summary: "bounded approval-required GitHub PR comment write preflight summary"
                .to_owned(),
            sensitivity: SideEffectSensitivity::Internal,
            redaction: redaction(),
        })
        .expect("preflight definition");
    preflight_definition.requires_approval = true;
    definition.preflight =
        AdapterWritePreflightRequest::new(preflight_definition).expect("constructable preflight");
    let request = GitHubPullRequestCommentWriteRequest::new(definition).expect("valid request");

    let error = GitHubPullRequestCommentPreflightedWrite::new(request)
        .expect_err("missing approval fails composition");

    assert_eq!(error.code(), "adapter_write_preflight.approval.missing");
}

#[test]
fn preflight_composition_rejects_live_sandbox_mode_before_provider_work() {
    let mut definition = valid_request_definition();
    definition.mode = GitHubPullRequestCommentWriteMode::LiveSandbox;
    let request = GitHubPullRequestCommentWriteRequest::new(definition).expect("valid request");

    let error = GitHubPullRequestCommentPreflightedWrite::new(request)
        .expect_err("live sandbox rejected by composition");

    assert_eq!(
        error.code(),
        "github_pr_comment_write.preflight.live_sandbox_unsupported"
    );
}

#[test]
fn preflighted_write_debug_redacts_request_and_decision_details() {
    let preflighted = preflighted_write();

    let debug = format!("{preflighted:?}");

    assert!(!debug.contains("Workflow OS governed comment preview"));
    assert!(!debug.contains("workflow-os"));
    assert!(!debug.contains("kernel"));
    assert!(!debug.contains("run/github-pr-comment"));
    assert!(!debug.contains("github-pr-comment-42"));
    assert!(!debug.contains("side-effect/github-pr-comment"));
    assert!(debug.contains("provider_call_allowed: false"));
}

#[test]
fn fixture_helper_returns_valid_fixture_response_from_preflighted_write() {
    let preflighted = preflighted_write();
    let fixture = valid_fixture();

    let response = validate_github_pr_comment_fixture_write(&preflighted, &fixture)
        .expect("valid fixture response");

    assert_eq!(
        response.outcome(),
        GitHubPullRequestCommentWriteOutcome::FixtureValidated
    );
    assert_eq!(response.provider_comment_reference(), None);
    assert_eq!(response.provider_error_code(), None);
    assert_eq!(
        response.summary(),
        "fixture request validated without provider call"
    );
    assert!(!response.workflow_event_append_allowed());
    assert!(!response.side_effect_lifecycle_transition_allowed());
}

#[test]
fn proposed_side_effect_record_composes_from_fixture_response() {
    let preflighted = preflighted_write();
    let fixture = valid_fixture();
    let response = validate_github_pr_comment_fixture_write(&preflighted, &fixture)
        .expect("valid fixture response");

    let record = compose_github_pr_comment_proposed_side_effect_record(
        &preflighted,
        Some(&response),
        side_effect_record_input(),
    )
    .expect("valid proposed record");

    assert_eq!(record.lifecycle_state(), SideEffectLifecycleState::Proposed);
    assert_eq!(record.capability(), SideEffectCapability::GitHubWrite);
    assert_eq!(
        record.target().kind(),
        SideEffectTargetKind::AdapterResource
    );
    assert_eq!(
        record.target().reference(),
        "github/workflow-os/kernel/pull/42"
    );
    assert_eq!(
        record.authority().decision,
        SideEffectAuthorityDecision::ApprovedByHuman
    );
    assert_eq!(record.authority().policy_references.len(), 1);
    assert_eq!(record.authority().approval_references.len(), 1);
    assert_eq!(
        record.side_effect_id().as_str(),
        "side-effect/github-pr-comment"
    );
    assert_eq!(record.workflow_id().as_str(), "workflow/write-candidate");
    assert_eq!(record.run_id().as_str(), "run/github-pr-comment");
    assert_eq!(
        record.summary(),
        Some("bounded GitHub PR comment write request summary")
    );
    assert_eq!(record.references().len(), 3);
    assert!(record.outcome_reference().is_none());
    assert!(record.reason_codes().is_empty());
}

#[test]
fn proposed_side_effect_record_maps_allowed_policy_without_approval() {
    let mut definition = valid_request_definition();
    definition.preflight =
        github_pr_comment_preflight_definition(GitHubPullRequestCommentPreflightDefinitionInput {
            target: definition.target.clone(),
            side_effect_id: definition.side_effect_id.clone(),
            idempotency_key: definition.idempotency_key.clone(),
            policy_decision: AdapterWritePolicyDecision::Allowed,
            policy_references: vec![policy_ref()],
            approval_references: Vec::new(),
            summary: "bounded GitHub PR comment write preflight summary".to_owned(),
            sensitivity: SideEffectSensitivity::Internal,
            redaction: redaction(),
        })
        .and_then(AdapterWritePreflightRequest::new)
        .expect("valid preflight request");
    let request = GitHubPullRequestCommentWriteRequest::new(definition).expect("valid request");
    let preflighted =
        GitHubPullRequestCommentPreflightedWrite::new(request).expect("valid preflighted write");

    let record = compose_github_pr_comment_proposed_side_effect_record(
        &preflighted,
        None,
        side_effect_record_input(),
    )
    .expect("valid proposed record");

    assert_eq!(
        record.authority().decision,
        SideEffectAuthorityDecision::AllowedByPolicy
    );
    assert!(record.authority().approval_references.is_empty());
}

#[test]
fn proposed_side_effect_record_composes_from_dry_run_response() {
    let mut definition = valid_request_definition();
    definition.mode = GitHubPullRequestCommentWriteMode::DryRun;
    let request = GitHubPullRequestCommentWriteRequest::new(definition).expect("valid request");
    let preflighted =
        GitHubPullRequestCommentPreflightedWrite::new(request).expect("valid preflighted write");
    let mut fixture_definition = valid_fixture_definition();
    fixture_definition.mode = GitHubPullRequestCommentWriteMode::DryRun;
    fixture_definition.fixture_reference = Some("fixture/github-pr-comment-dry-run-42".to_owned());
    fixture_definition.summary = "dry run request validated without provider call".to_owned();
    let fixture = GitHubPullRequestCommentFixture::new(fixture_definition).expect("valid fixture");
    let response = validate_github_pr_comment_fixture_write(&preflighted, &fixture)
        .expect("valid dry-run response");

    let record = compose_github_pr_comment_proposed_side_effect_record(
        &preflighted,
        Some(&response),
        side_effect_record_input(),
    )
    .expect("valid proposed record");

    assert_eq!(record.lifecycle_state(), SideEffectLifecycleState::Proposed);
    assert!(record.outcome_reference().is_none());
}

#[test]
fn proposed_side_effect_record_rejects_provider_response_without_leakage() {
    let preflighted = preflighted_write();
    let mut response_definition = valid_response_definition();
    response_definition.mode = GitHubPullRequestCommentWriteMode::LiveSandbox;
    response_definition.outcome = GitHubPullRequestCommentWriteOutcome::ProviderSucceeded;
    response_definition.provider_comment_reference = Some("github/comment/123".to_owned());
    let response = GitHubPullRequestCommentWriteResponse::new(response_definition)
        .expect("valid future provider response model");

    let error = compose_github_pr_comment_proposed_side_effect_record(
        &preflighted,
        Some(&response),
        side_effect_record_input(),
    )
    .expect_err("provider responses are unsupported for proposed records");

    assert_eq!(
        error.code(),
        "github_pr_comment_side_effect_record.response.unsupported"
    );
    assert!(!format!("{error:?}").contains("github/comment/123"));
}

#[test]
fn proposed_side_effect_record_rejects_secret_like_summary_override() {
    let preflighted = preflighted_write();
    let mut input = side_effect_record_input();
    input.summary_override = Some("raw_provider_payload must not be copied".to_owned());

    let error = compose_github_pr_comment_proposed_side_effect_record(&preflighted, None, input)
        .expect_err("secret-like summary rejected");

    assert_eq!(error.code(), "github_pr_comment_write.secret_like_value");
    assert!(!format!("{error:?}").contains("raw_provider_payload"));
}

#[test]
fn proposed_side_effect_record_rejects_system_actor_when_request_actor_exists() {
    let preflighted = preflighted_write();
    let mut input = side_effect_record_input();
    input.system_actor = Some(ActorId::new("system/alternate").expect("valid actor"));

    let error = compose_github_pr_comment_proposed_side_effect_record(&preflighted, None, input)
        .expect_err("system actor rejected when request already has actor");

    assert_eq!(
        error.code(),
        "github_pr_comment_side_effect_record.authority.unsupported"
    );
}

#[test]
fn proposed_side_effect_record_input_debug_redacts_sensitive_fields() {
    let input = side_effect_record_input();

    let debug = format!("{input:?}");

    assert!(!debug.contains("evidence/github-pr-comment-fixture"));
    assert!(debug.contains("additional_reference_count: 1"));
    assert!(debug.contains("provider_call_allowed: false"));
    assert!(debug.contains("report_artifact_write_allowed: false"));
}

#[test]
fn fixture_helper_returns_valid_dry_run_response() {
    let mut definition = valid_request_definition();
    definition.mode = GitHubPullRequestCommentWriteMode::DryRun;
    let request = GitHubPullRequestCommentWriteRequest::new(definition).expect("valid request");
    let preflighted =
        GitHubPullRequestCommentPreflightedWrite::new(request).expect("valid preflighted write");
    let mut fixture_definition = valid_fixture_definition();
    fixture_definition.mode = GitHubPullRequestCommentWriteMode::DryRun;
    fixture_definition.fixture_reference = Some("fixture/github-pr-comment-dry-run-42".to_owned());
    fixture_definition.summary = "dry run request validated without provider call".to_owned();
    let fixture = GitHubPullRequestCommentFixture::new(fixture_definition).expect("valid fixture");

    let response = validate_github_pr_comment_fixture_write(&preflighted, &fixture)
        .expect("valid dry-run response");

    assert_eq!(
        response.outcome(),
        GitHubPullRequestCommentWriteOutcome::DryRunValidated
    );
    assert_eq!(response.provider_comment_reference(), None);
    assert_eq!(response.provider_error_code(), None);
}

#[test]
fn fixture_helper_rejects_target_mismatch_without_leaking_target() {
    let preflighted = preflighted_write();
    let mut fixture_definition = valid_fixture_definition();
    fixture_definition.target =
        GitHubPullRequestCommentTarget::new("workflow-os", "other", 42).expect("valid target");
    let fixture = GitHubPullRequestCommentFixture::new(fixture_definition).expect("valid fixture");

    let error = validate_github_pr_comment_fixture_write(&preflighted, &fixture)
        .expect_err("target mismatch rejected");

    assert_eq!(error.code(), "github_pr_comment_fixture.target.mismatch");
    assert!(!format!("{error:?}").contains("other"));
}

#[test]
fn fixture_helper_rejects_side_effect_mismatch() {
    let preflighted = preflighted_write();
    let mut fixture_definition = valid_fixture_definition();
    fixture_definition.side_effect_id =
        SideEffectId::new("side-effect/other-comment").expect("valid side-effect id");
    let fixture = GitHubPullRequestCommentFixture::new(fixture_definition).expect("valid fixture");

    let error = validate_github_pr_comment_fixture_write(&preflighted, &fixture)
        .expect_err("side-effect mismatch rejected");

    assert_eq!(
        error.code(),
        "github_pr_comment_fixture.side_effect.mismatch"
    );
}

#[test]
fn fixture_helper_rejects_idempotency_mismatch() {
    let preflighted = preflighted_write();
    let mut fixture_definition = valid_fixture_definition();
    fixture_definition.idempotency_key =
        IdempotencyKey::new("github-pr-comment-other").expect("valid idempotency key");
    let fixture = GitHubPullRequestCommentFixture::new(fixture_definition).expect("valid fixture");

    let error = validate_github_pr_comment_fixture_write(&preflighted, &fixture)
        .expect_err("idempotency mismatch rejected");

    assert_eq!(
        error.code(),
        "github_pr_comment_fixture.idempotency.mismatch"
    );
}

#[test]
fn fixture_input_rejects_live_sandbox_mode() {
    let mut fixture_definition = valid_fixture_definition();
    fixture_definition.mode = GitHubPullRequestCommentWriteMode::LiveSandbox;
    let error = GitHubPullRequestCommentFixture::new(fixture_definition)
        .expect_err("live sandbox fixture rejected");

    assert_eq!(error.code(), "github_pr_comment_fixture.mode.unsupported");
}

#[test]
fn fixture_input_rejects_secret_like_summary_and_reference() {
    let mut fixture_definition = valid_fixture_definition();
    fixture_definition.summary = "raw_provider_payload should never appear".to_owned();
    let summary_error = GitHubPullRequestCommentFixture::new(fixture_definition)
        .expect_err("secret-like fixture summary rejected");

    assert_eq!(
        summary_error.code(),
        "github_pr_comment_write.secret_like_value"
    );

    let mut fixture_definition = valid_fixture_definition();
    fixture_definition.fixture_reference = Some("fixture/api_token".to_owned());
    let reference_error = GitHubPullRequestCommentFixture::new(fixture_definition)
        .expect_err("secret-like fixture reference rejected");

    assert_eq!(
        reference_error.code(),
        "github_pr_comment_write.secret_like_value"
    );
}

#[test]
fn fixture_debug_redacts_target_ids_reference_and_summary() {
    let fixture = valid_fixture();

    let debug = format!("{fixture:?}");

    assert!(!debug.contains("workflow-os"));
    assert!(!debug.contains("kernel"));
    assert!(!debug.contains("side-effect/github-pr-comment"));
    assert!(!debug.contains("github-pr-comment-42"));
    assert!(!debug.contains("fixture/github-pr-comment-42"));
    assert!(!debug.contains("fixture request validated"));
    assert!(debug.contains("provider_call_allowed: false"));
    assert!(debug.contains("report_artifact_write_allowed: false"));
}

#[test]
fn target_rejects_url_or_path_shaped_repository_values() {
    let error = GitHubPullRequestCommentTarget::new("workflow-os", "https://github.com/x/y", 1)
        .expect_err("URL-shaped repository rejected");

    assert_eq!(error.code(), "github_pr_comment_write.target.invalid");
    assert!(!format!("{error:?}").contains("https://github.com/x/y"));
}

#[test]
fn target_rejects_zero_pull_request_number() {
    let error = GitHubPullRequestCommentTarget::new("workflow-os", "kernel", 0)
        .expect_err("zero PR rejected");

    assert_eq!(
        error.code(),
        "github_pr_comment_write.target.pull_request_number"
    );
}

#[test]
fn request_rejects_mismatched_preflight_capability_or_target() {
    let mut definition = valid_request_definition();
    let wrong_target =
        GitHubPullRequestCommentTarget::new("workflow-os", "other", 42).expect("valid target");
    definition.target = wrong_target;

    let error =
        GitHubPullRequestCommentWriteRequest::new(definition).expect_err("mismatched target");

    assert_eq!(
        error.code(),
        "github_pr_comment_write.preflight.target_reference"
    );
}

#[test]
fn request_rejects_mismatched_side_effect_id() {
    let mut definition = valid_request_definition();
    definition.side_effect_id =
        SideEffectId::new("side-effect/other-comment").expect("valid side-effect id");

    let error =
        GitHubPullRequestCommentWriteRequest::new(definition).expect_err("mismatched side effect");

    assert_eq!(
        error.code(),
        "github_pr_comment_write.preflight.side_effect"
    );
}

#[test]
fn request_rejects_secret_like_comment_body_without_leaking_value() {
    let mut definition = valid_request_definition();
    definition.comment_body = "please use bearer token abc123".to_owned();

    let error = GitHubPullRequestCommentWriteRequest::new(definition)
        .expect_err("secret-like comment rejected");

    assert_eq!(error.code(), "github_pr_comment_write.secret_like_value");
    assert!(!format!("{error:?}").contains("abc123"));
}

#[test]
fn request_rejects_forbidden_raw_payload_markers() {
    let mut definition = valid_request_definition();
    definition.comment_body = "raw_provider_payload should never be copied".to_owned();

    let error =
        GitHubPullRequestCommentWriteRequest::new(definition).expect_err("raw marker rejected");

    assert_eq!(error.code(), "github_pr_comment_write.secret_like_value");
}

#[test]
fn request_rejects_secret_like_redaction_metadata() {
    let mut definition = valid_request_definition();
    definition.redaction = RedactionMetadata {
        redacted_fields: vec!["api_token".to_owned()],
        field_states: Vec::new(),
    };

    let error =
        GitHubPullRequestCommentWriteRequest::new(definition).expect_err("redaction rejected");

    assert_eq!(error.code(), "github_pr_comment_write.secret_like_value");
    assert!(!format!("{error:?}").contains("api_token"));
}

#[test]
fn request_debug_redacts_body_target_ids_and_preflight_details() {
    let request = valid_request();

    let debug = format!("{request:?}");

    assert!(!debug.contains("Workflow OS governed comment preview"));
    assert!(!debug.contains("workflow-os"));
    assert!(!debug.contains("kernel"));
    assert!(!debug.contains("run/github-pr-comment"));
    assert!(!debug.contains("github-pr-comment-42"));
    assert!(!debug.contains("side-effect/github-pr-comment"));
}

#[test]
fn request_serde_round_trip_preserves_valid_shape() {
    let request = valid_request();

    let serialized = serde_json::to_string(&request).expect("serialize request");
    let deserialized: GitHubPullRequestCommentWriteRequest =
        serde_json::from_str(&serialized).expect("deserialize request");

    assert_eq!(deserialized, request);
}

#[test]
fn invalid_serialized_request_fails_closed_without_leaking_secret_like_value() {
    let value = json!({
        "adapter_id": "adapter/github",
        "integration_id": "integration/github/sandbox",
        "correlation_id": "correlation/github-pr-comment",
        "workflow_id": "workflow/write-candidate",
        "workflow_version": "v1",
        "schema_version": "workflowos.dev/v0",
        "spec_hash": SpecContentHash::from_text("write-candidate-spec").as_str(),
        "run_id": "run/github-pr-comment",
        "step_id": "step/comment",
        "actor": "system/workflow-os",
        "target": {
            "owner": "workflow-os",
            "repository": "kernel",
            "pull_request_number": 42
        },
        "comment_body": "bearer super-sensitive-value",
        "summary": "bounded GitHub PR comment write request summary",
        "side_effect_id": "side-effect/github-pr-comment",
        "idempotency_key": "github-pr-comment-42",
        "mode": "fixture",
        "preflight": valid_request_definition().preflight,
        "sensitivity": "internal",
        "redaction": RedactionMetadata::empty()
    });

    let error = serde_json::from_value::<GitHubPullRequestCommentWriteRequest>(value)
        .expect_err("invalid serialized request rejected");
    let error = error.to_string();

    assert!(error.contains("github_pr_comment_write.secret_like_value"));
    assert!(!error.contains("super-sensitive-value"));
}

#[test]
fn valid_fixture_response_is_model_only() {
    let response =
        GitHubPullRequestCommentWriteResponse::new(valid_response_definition()).expect("valid");

    assert_eq!(
        response.outcome(),
        GitHubPullRequestCommentWriteOutcome::FixtureValidated
    );
    assert_eq!(response.provider_comment_reference(), None);
    assert_eq!(response.provider_error_code(), None);
    assert!(!response.workflow_event_append_allowed());
    assert!(!response.side_effect_lifecycle_transition_allowed());
}

#[test]
fn fixture_response_rejects_provider_reference() {
    let mut definition = valid_response_definition();
    definition.provider_comment_reference = Some("github/comment/123".to_owned());

    let error = GitHubPullRequestCommentWriteResponse::new(definition)
        .expect_err("fixture provider reference rejected");

    assert_eq!(
        error.code(),
        "github_pr_comment_write.response.provider_reference_unexpected"
    );
}

#[test]
fn provider_success_response_requires_provider_reference() {
    let mut definition = valid_response_definition();
    definition.mode = GitHubPullRequestCommentWriteMode::LiveSandbox;
    definition.outcome = GitHubPullRequestCommentWriteOutcome::ProviderSucceeded;

    let error = GitHubPullRequestCommentWriteResponse::new(definition)
        .expect_err("missing provider reference rejected");

    assert_eq!(
        error.code(),
        "github_pr_comment_write.response.provider_reference_missing"
    );
}

#[test]
fn provider_failed_response_requires_bounded_error_code() {
    let mut definition = valid_response_definition();
    definition.mode = GitHubPullRequestCommentWriteMode::LiveSandbox;
    definition.outcome = GitHubPullRequestCommentWriteOutcome::ProviderFailed;
    definition.provider_error_code = Some("github.rate_limited".to_owned());

    let response =
        GitHubPullRequestCommentWriteResponse::new(definition).expect("valid failure response");

    assert_eq!(
        response.outcome(),
        GitHubPullRequestCommentWriteOutcome::ProviderFailed
    );
    assert_eq!(response.provider_error_code(), Some("github.rate_limited"));
}

#[test]
fn response_debug_redacts_provider_reference_and_summary() {
    let mut definition = valid_response_definition();
    definition.mode = GitHubPullRequestCommentWriteMode::LiveSandbox;
    definition.outcome = GitHubPullRequestCommentWriteOutcome::ProviderSucceeded;
    definition.provider_comment_reference = Some("github/comment/123".to_owned());
    let response = GitHubPullRequestCommentWriteResponse::new(definition).expect("valid response");

    let debug = format!("{response:?}");

    assert!(!debug.contains("github/comment/123"));
    assert!(!debug.contains("fixture request validated"));
}
