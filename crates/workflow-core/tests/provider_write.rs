#![allow(clippy::expect_used)]

//! Provider write request/response model tests.

use std::{
    cell::RefCell,
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

use serde_json::json;
use workflow_core::{
    assess_provider_write_sandbox_readiness,
    compose_and_persist_github_pr_comment_proposed_side_effect_record,
    compose_github_pr_comment_proposed_side_effect_event,
    compose_github_pr_comment_proposed_side_effect_record, github_pr_comment_preflight_definition,
    integrate_github_pr_comment_provider_lookup_recovery,
    load_github_pr_comment_proposed_side_effect_event,
    load_github_pr_comment_proposed_side_effect_event_input,
    orchestrate_github_pr_comment_no_provider_outcome, orchestrate_github_pr_comment_provider_call,
    orchestrate_github_pr_comment_write_attempt_without_provider_call,
    reconcile_github_pr_comment_provider_lookup, reconcile_github_pr_comment_provider_write,
    summarize_github_pr_comment_provider_lookup_operator_recovery,
    validate_and_orchestrate_github_pr_comment_live_sandbox,
    validate_github_pr_comment_fixture_write, ActorId, AdapterId, AdapterKind,
    AdapterWriteCapability, AdapterWritePolicyDecision, AdapterWritePreflightRequest,
    AdapterWriteTarget, AdapterWriteTargetKind, ApprovalDecision, ApprovalDecisionKind,
    ApprovalRequest, CorrelationId, EventId, EventSequenceNumber, GitHubPullRequestCommentFixture,
    GitHubPullRequestCommentFixtureDefinition, GitHubPullRequestCommentHttpProvider,
    GitHubPullRequestCommentHttpRequest, GitHubPullRequestCommentHttpResponse,
    GitHubPullRequestCommentHttpTransport, GitHubPullRequestCommentLiveSandboxValidationInput,
    GitHubPullRequestCommentLookupHttpClient, GitHubPullRequestCommentLookupHttpRequest,
    GitHubPullRequestCommentLookupHttpResponse, GitHubPullRequestCommentLookupHttpTransport,
    GitHubPullRequestCommentNoProviderOutcome,
    GitHubPullRequestCommentNoProviderOutcomeOrchestrationInput,
    GitHubPullRequestCommentPreflightDefinitionInput, GitHubPullRequestCommentPreflightedWrite,
    GitHubPullRequestCommentProvider, GitHubPullRequestCommentProviderAuth,
    GitHubPullRequestCommentProviderCallInput,
    GitHubPullRequestCommentProviderCallOrchestrationInput,
    GitHubPullRequestCommentProviderCallRequest,
    GitHubPullRequestCommentProviderEventProofRecoveryInput,
    GitHubPullRequestCommentProviderEventProofRecoveryNextAction,
    GitHubPullRequestCommentProviderEventProofRecoveryPosture,
    GitHubPullRequestCommentProviderLookupClient,
    GitHubPullRequestCommentProviderLookupObservation,
    GitHubPullRequestCommentProviderLookupObservationDefinition,
    GitHubPullRequestCommentProviderLookupOperatorRecoveryNextAction,
    GitHubPullRequestCommentProviderLookupOperatorRecoverySummary,
    GitHubPullRequestCommentProviderLookupOutcome,
    GitHubPullRequestCommentProviderLookupReconciliationInput,
    GitHubPullRequestCommentProviderLookupReconciliationNextAction,
    GitHubPullRequestCommentProviderLookupReconciliationPosture,
    GitHubPullRequestCommentProviderLookupRecoveryIntegrationInput,
    GitHubPullRequestCommentProviderLookupRequest, GitHubPullRequestCommentProviderLookupResponse,
    GitHubPullRequestCommentProviderLookupResponseDefinition,
    GitHubPullRequestCommentProviderWriteReconciliationCandidate,
    GitHubPullRequestCommentProviderWriteReconciliationInput,
    GitHubPullRequestCommentProviderWriteReconciliationStatus,
    GitHubPullRequestCommentSideEffectAppendInput, GitHubPullRequestCommentSideEffectEventContext,
    GitHubPullRequestCommentSideEffectRecordInput, GitHubPullRequestCommentTarget,
    GitHubPullRequestCommentWriteAttemptOrchestrationInput, GitHubPullRequestCommentWriteMode,
    GitHubPullRequestCommentWriteOutcome, GitHubPullRequestCommentWriteRequest,
    GitHubPullRequestCommentWriteRequestDefinition, GitHubPullRequestCommentWriteResponse,
    GitHubPullRequestCommentWriteResponseDefinition, IdempotencyKey, IntegrationId,
    LocalStateBackend, ProviderWriteSandboxApprovalPosture, ProviderWriteSandboxAuthPosture,
    ProviderWriteSandboxEventProofPosture, ProviderWriteSandboxProviderLocalPosture,
    ProviderWriteSandboxReadinessDecision, ProviderWriteSandboxReadinessInput,
    ProviderWriteSandboxReadinessIssue, ProviderWriteSandboxSideEffectPosture,
    ProviderWriteSandboxTargetClassification, ProviderWriteSandboxTargetPosture,
    ProviderWriteSandboxTargetProof, ProviderWriteSandboxTargetProofDefinition,
    RedactionDisposition, RedactionFieldState, RedactionMetadata, SchemaVersion,
    SideEffectAuthority, SideEffectAuthorityDecision, SideEffectCapability, SideEffectId,
    SideEffectIdempotencyBinding, SideEffectIdempotencyScope, SideEffectLifecycleState,
    SideEffectOutcomeReference, SideEffectOutcomeReferenceKind, SideEffectRecord,
    SideEffectRecordDefinition, SideEffectRecordStore, SideEffectReference,
    SideEffectReferenceKind, SideEffectSensitivity, SideEffectTargetKind,
    SideEffectTargetReference, SkillId, SkillVersion, SpecContentHash, StepId, Timestamp,
    WorkReportSensitivity, WorkflowId, WorkflowOsError, WorkflowRun, WorkflowRunEvent,
    WorkflowRunEventKind, WorkflowRunId, WorkflowVersion,
};

static STATE_BACKEND_COUNTER: AtomicU64 = AtomicU64::new(0);

struct TestStateBackend {
    backend: LocalStateBackend,
    root: PathBuf,
}

impl TestStateBackend {
    fn backend(&self) -> &LocalStateBackend {
        &self.backend
    }
}

impl Drop for TestStateBackend {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn test_state_backend() -> TestStateBackend {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time after epoch")
        .as_nanos();
    let sequence = STATE_BACKEND_COUNTER.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!(
        "workflow-os-provider-write-state-{}-{nonce}-{sequence}",
        std::process::id()
    ));
    let backend = LocalStateBackend::new(&root).expect("local state backend");
    TestStateBackend { backend, root }
}

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

fn sandbox_readiness_input() -> ProviderWriteSandboxReadinessInput {
    ProviderWriteSandboxReadinessInput {
        capability: AdapterWriteCapability::GitHubPullRequestComment,
        target: AdapterWriteTarget::new(
            AdapterWriteTargetKind::GitHubPullRequest,
            "github/sandbox-owner/sandbox-repo/pull/42",
        )
        .expect("valid sandbox target"),
        target_posture: ProviderWriteSandboxTargetPosture::ExplicitSandbox,
        auth_posture: ProviderWriteSandboxAuthPosture::ExplicitCallerSupplied,
        approval_required: true,
        approval_posture: ProviderWriteSandboxApprovalPosture::LinkedAndApproved,
        side_effect_posture: ProviderWriteSandboxSideEffectPosture::Attempted,
        event_proof_required: true,
        event_proof_posture: ProviderWriteSandboxEventProofPosture::Present,
        provider_local_posture: ProviderWriteSandboxProviderLocalPosture::NotYetAttempted,
        sensitivity: SideEffectSensitivity::Internal,
        redaction: redaction(),
    }
}

fn sandbox_target_proof() -> ProviderWriteSandboxTargetProof {
    ProviderWriteSandboxTargetProof::new(ProviderWriteSandboxTargetProofDefinition {
        target: GitHubPullRequestCommentTarget::new("sandbox-owner", "sandbox-repo", 42)
            .expect("valid target"),
        classification: ProviderWriteSandboxTargetClassification::Disposable,
        capability: AdapterWriteCapability::GitHubPullRequestComment,
        non_production_confirmed: true,
        non_production_statement: "disposable maintainer sandbox pull request".to_owned(),
        actor: ActorId::new("user/sandbox-maintainer").expect("valid actor"),
        correlation_id: CorrelationId::new("correlation/sandbox-target-proof")
            .expect("valid correlation"),
        idempotency_key: IdempotencyKey::new("idempotency/sandbox-target-proof")
            .expect("valid idempotency"),
        sensitivity: SideEffectSensitivity::Internal,
        redaction: redaction(),
    })
    .expect("valid target proof")
}

fn live_sandbox_target_proof() -> ProviderWriteSandboxTargetProof {
    ProviderWriteSandboxTargetProof::new(ProviderWriteSandboxTargetProofDefinition {
        target: target(),
        classification: ProviderWriteSandboxTargetClassification::MaintainerSandbox,
        capability: AdapterWriteCapability::GitHubPullRequestComment,
        non_production_confirmed: true,
        non_production_statement: "explicit maintainer sandbox pull request only".to_owned(),
        actor: ActorId::new("user/sandbox-maintainer").expect("valid actor"),
        correlation_id: CorrelationId::new("correlation/live-sandbox-validation")
            .expect("valid correlation"),
        idempotency_key: idempotency_key(),
        sensitivity: SideEffectSensitivity::Internal,
        redaction: redaction(),
    })
    .expect("valid live sandbox target proof")
}

fn live_sandbox_readiness_input(
    target_proof: &ProviderWriteSandboxTargetProof,
) -> ProviderWriteSandboxReadinessInput {
    ProviderWriteSandboxReadinessInput {
        capability: target_proof.capability(),
        target: target_proof
            .adapter_target()
            .expect("valid adapter target from proof"),
        target_posture: target_proof.target_posture(),
        auth_posture: ProviderWriteSandboxAuthPosture::ExplicitCallerSupplied,
        approval_required: true,
        approval_posture: ProviderWriteSandboxApprovalPosture::LinkedAndApproved,
        side_effect_posture: ProviderWriteSandboxSideEffectPosture::Attempted,
        event_proof_required: true,
        event_proof_posture: ProviderWriteSandboxEventProofPosture::Present,
        provider_local_posture: ProviderWriteSandboxProviderLocalPosture::NotYetAttempted,
        sensitivity: SideEffectSensitivity::Internal,
        redaction: redaction(),
    }
}

#[test]
fn sandbox_target_proof_derives_explicit_sandbox_readiness_posture() {
    let proof = sandbox_target_proof();

    assert_eq!(
        proof.target_posture(),
        ProviderWriteSandboxTargetPosture::ExplicitSandbox
    );
    assert_eq!(
        proof.capability(),
        AdapterWriteCapability::GitHubPullRequestComment
    );
    assert!(proof.non_production_confirmed());
    assert!(!proof.provider_call_allowed());
    assert!(!proof.workflow_event_append_allowed());
    assert!(!proof.report_artifact_write_allowed());

    let adapter_target = proof.adapter_target().expect("valid adapter target");
    assert_eq!(
        adapter_target.kind(),
        AdapterWriteTargetKind::GitHubPullRequest
    );
    assert_eq!(
        adapter_target.reference(),
        "github/sandbox-owner/sandbox-repo/pull/42"
    );
}

#[test]
fn sandbox_target_proof_feeds_readiness_without_provider_authority() {
    let proof = sandbox_target_proof();
    let mut input = sandbox_readiness_input();
    input.target = proof.adapter_target().expect("valid proof target");
    input.target_posture = proof.target_posture();
    input.capability = proof.capability();
    input.sensitivity = proof.sensitivity();
    input.redaction = proof.redaction().clone();

    let result = assess_provider_write_sandbox_readiness(&input).expect("readiness result");

    assert_eq!(
        result.decision(),
        ProviderWriteSandboxReadinessDecision::AllowedForSandbox
    );
    assert!(!result.provider_call_allowed());
}

#[test]
fn sandbox_target_proof_fails_closed_for_unconfirmed_non_production_target() {
    let mut definition = ProviderWriteSandboxTargetProofDefinition {
        target: GitHubPullRequestCommentTarget::new("sandbox-owner", "sandbox-repo", 42)
            .expect("valid target"),
        classification: ProviderWriteSandboxTargetClassification::Disposable,
        capability: AdapterWriteCapability::GitHubPullRequestComment,
        non_production_confirmed: false,
        non_production_statement: "classification is not confirmed".to_owned(),
        actor: ActorId::new("user/sandbox-maintainer").expect("valid actor"),
        correlation_id: CorrelationId::new("correlation/sandbox-target-proof")
            .expect("valid correlation"),
        idempotency_key: IdempotencyKey::new("idempotency/sandbox-target-proof")
            .expect("valid idempotency"),
        sensitivity: SideEffectSensitivity::Internal,
        redaction: redaction(),
    };

    let proof =
        ProviderWriteSandboxTargetProof::new(definition.clone()).expect("valid proof model");

    assert_eq!(
        proof.target_posture(),
        ProviderWriteSandboxTargetPosture::ProductionLike
    );

    definition.classification = ProviderWriteSandboxTargetClassification::Unknown;
    definition.non_production_confirmed = true;
    let unknown =
        ProviderWriteSandboxTargetProof::new(definition).expect("valid unknown proof model");
    assert_eq!(
        unknown.target_posture(),
        ProviderWriteSandboxTargetPosture::Unknown
    );
}

#[test]
fn sandbox_target_proof_rejects_secret_like_statement_without_leakage() {
    let error = ProviderWriteSandboxTargetProof::new(ProviderWriteSandboxTargetProofDefinition {
        target: GitHubPullRequestCommentTarget::new("sandbox-owner", "sandbox-repo", 42)
            .expect("valid target"),
        classification: ProviderWriteSandboxTargetClassification::Disposable,
        capability: AdapterWriteCapability::GitHubPullRequestComment,
        non_production_confirmed: true,
        non_production_statement: "token=raw_provider_payload".to_owned(),
        actor: ActorId::new("user/sandbox-maintainer").expect("valid actor"),
        correlation_id: CorrelationId::new("correlation/sandbox-target-proof")
            .expect("valid correlation"),
        idempotency_key: IdempotencyKey::new("idempotency/sandbox-target-proof")
            .expect("valid idempotency"),
        sensitivity: SideEffectSensitivity::Internal,
        redaction: redaction(),
    })
    .expect_err("secret-like proof statement rejected");

    assert_eq!(error.code(), "github_pr_comment_write.secret_like_value");
    assert!(!error.to_string().contains("raw_provider_payload"));
    assert!(!format!("{error:?}").contains("raw_provider_payload"));
}

#[test]
fn sandbox_target_proof_rejects_unsupported_capability() {
    let error = ProviderWriteSandboxTargetProof::new(ProviderWriteSandboxTargetProofDefinition {
        target: GitHubPullRequestCommentTarget::new("sandbox-owner", "sandbox-repo", 42)
            .expect("valid target"),
        classification: ProviderWriteSandboxTargetClassification::Disposable,
        capability: AdapterWriteCapability::GitHubMerge,
        non_production_confirmed: true,
        non_production_statement: "disposable maintainer sandbox pull request".to_owned(),
        actor: ActorId::new("user/sandbox-maintainer").expect("valid actor"),
        correlation_id: CorrelationId::new("correlation/sandbox-target-proof")
            .expect("valid correlation"),
        idempotency_key: IdempotencyKey::new("idempotency/sandbox-target-proof")
            .expect("valid idempotency"),
        sensitivity: SideEffectSensitivity::Internal,
        redaction: redaction(),
    })
    .expect_err("unsupported capability rejected");

    assert_eq!(
        error.code(),
        "provider_write_sandbox_target_proof.capability.unsupported"
    );
    assert!(!error.to_string().contains("sandbox-owner"));
}

#[test]
fn sandbox_target_proof_debug_redacts_sensitive_values() {
    let proof = sandbox_target_proof();

    let debug = format!("{proof:?}");

    assert!(!debug.contains("sandbox-owner"));
    assert!(!debug.contains("sandbox-repo"));
    assert!(!debug.contains("disposable maintainer sandbox"));
    assert!(!debug.contains("user/sandbox-maintainer"));
    assert!(!debug.contains("correlation/sandbox-target-proof"));
    assert!(!debug.contains("idempotency/sandbox-target-proof"));
    assert!(!debug.contains("comment_body"));
    assert!(debug.contains("provider_call_allowed: false"));
}

#[test]
fn sandbox_target_proof_serde_round_trip_and_invalid_wire_fails_closed() {
    let proof = sandbox_target_proof();

    let serialized = serde_json::to_string(&proof).expect("serialize proof");
    let deserialized: ProviderWriteSandboxTargetProof =
        serde_json::from_str(&serialized).expect("deserialize proof");
    assert_eq!(deserialized.target_posture(), proof.target_posture());

    let invalid = json!({
        "target": {
            "owner": "sandbox-owner",
            "repository": "sandbox-repo",
            "pull_request_number": 42
        },
        "classification": "disposable",
        "capability": "github_merge",
        "non_production_confirmed": true,
        "non_production_statement": "disposable maintainer sandbox pull request",
        "actor": "user/sandbox-maintainer",
        "correlation_id": "correlation/sandbox-target-proof",
        "idempotency_key": "idempotency/sandbox-target-proof",
        "sensitivity": "internal",
        "redaction": redaction()
    });

    let error = serde_json::from_value::<ProviderWriteSandboxTargetProof>(invalid)
        .expect_err("invalid wire fails closed");

    assert!(!error.to_string().contains("sandbox-owner"));
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

fn orchestration_input<'a>(
    response: Option<&'a GitHubPullRequestCommentWriteResponse>,
    run: Option<&'a WorkflowRun>,
) -> GitHubPullRequestCommentWriteAttemptOrchestrationInput<'a> {
    GitHubPullRequestCommentWriteAttemptOrchestrationInput {
        fixture_response: response,
        record_input: side_effect_record_input(),
        approval_run: run,
        require_approval_linkage: true,
        transitioned_at: Timestamp::parse_rfc3339("2026-06-20T12:01:00Z").expect("valid timestamp"),
        transition_summary: Some("attempt boundary recorded without provider call".to_owned()),
        transition_references: vec![SideEffectReference::new(
            SideEffectReferenceKind::EvidenceReference,
            "evidence/github-pr-comment-attempt",
        )
        .expect("valid transition reference")],
        evidence_reference_count: 1,
    }
}

fn no_provider_completed_input() -> GitHubPullRequestCommentNoProviderOutcomeOrchestrationInput {
    GitHubPullRequestCommentNoProviderOutcomeOrchestrationInput {
        transitioned_at: Timestamp::parse_rfc3339("2026-06-20T12:02:00Z").expect("valid timestamp"),
        outcome: GitHubPullRequestCommentNoProviderOutcome::Completed {
            outcome_reference: SideEffectOutcomeReference::new(
                SideEffectOutcomeReferenceKind::Outcome,
                "fixture/github-pr-comment-42/completed",
            )
            .expect("valid local outcome reference"),
        },
        transition_summary: Some("fixture outcome closed without provider call".to_owned()),
        transition_references: vec![SideEffectReference::new(
            SideEffectReferenceKind::EvidenceReference,
            "evidence/github-pr-comment-completed",
        )
        .expect("valid transition reference")],
        evidence_reference_count: 1,
    }
}

fn no_provider_failed_input() -> GitHubPullRequestCommentNoProviderOutcomeOrchestrationInput {
    GitHubPullRequestCommentNoProviderOutcomeOrchestrationInput {
        transitioned_at: Timestamp::parse_rfc3339("2026-06-20T12:02:00Z").expect("valid timestamp"),
        outcome: GitHubPullRequestCommentNoProviderOutcome::Failed {
            outcome_reference: Some(
                SideEffectOutcomeReference::new(
                    SideEffectOutcomeReferenceKind::Failure,
                    "local/github-pr-comment-42/validation-failed",
                )
                .expect("valid local failure reference"),
            ),
            reason_codes: vec!["local.fixture_validation_failed".to_owned()],
        },
        transition_summary: Some("local failure classified without provider call".to_owned()),
        transition_references: vec![SideEffectReference::new(
            SideEffectReferenceKind::EvidenceReference,
            "evidence/github-pr-comment-failed",
        )
        .expect("valid transition reference")],
        evidence_reference_count: 1,
    }
}

fn persisted_attempted_record(store: &LocalStateBackend) -> workflow_core::SideEffectRecord {
    let run = run_with_approval(ApprovalDecisionKind::Granted);
    orchestrate_github_pr_comment_write_attempt_without_provider_call(
        store,
        &preflighted_write(),
        orchestration_input(None, Some(&run)),
    )
    .expect("attempt orchestration succeeds")
    .attempted_transition()
    .record()
    .clone()
}

fn side_effect_event_context() -> GitHubPullRequestCommentSideEffectEventContext {
    GitHubPullRequestCommentSideEffectEventContext {
        workflow_id: WorkflowId::new("workflow/write-candidate").expect("valid workflow id"),
        workflow_version: WorkflowVersion::new("v1").expect("valid workflow version"),
        schema_version: SchemaVersion::new("workflowos.dev/v0").expect("valid schema version"),
        spec_hash: SpecContentHash::from_text("write-candidate-spec"),
        run_id: WorkflowRunId::new("run/github-pr-comment").expect("valid run id"),
    }
}

fn workflow_event(sequence: u64, kind: WorkflowRunEventKind) -> WorkflowRunEvent {
    WorkflowRunEvent {
        sequence_number: EventSequenceNumber::new(sequence).expect("valid sequence"),
        event_id: EventId::new(format!("event/github-pr-comment-{sequence}"))
            .expect("valid event id"),
        timestamp: Timestamp::parse_rfc3339("2026-06-20T12:00:00Z").expect("valid timestamp"),
        run_id: WorkflowRunId::new("run/github-pr-comment").expect("valid run id"),
        workflow_id: WorkflowId::new("workflow/write-candidate").expect("valid workflow id"),
        schema_version: SchemaVersion::new("workflowos.dev/v0").expect("valid schema version"),
        workflow_version: WorkflowVersion::new("v1").expect("valid workflow version"),
        spec_content_hash: SpecContentHash::from_text("write-candidate-spec"),
        correlation_id: Some(
            CorrelationId::new("correlation/github-pr-comment").expect("valid correlation id"),
        ),
        actor: Some(ActorId::new("system/workflow-os").expect("valid actor")),
        idempotency_key: None,
        kind,
    }
}

fn approval_request(approval_id: &str) -> ApprovalRequest {
    ApprovalRequest {
        approval_id: approval_id.to_owned(),
        run_id: WorkflowRunId::new("run/github-pr-comment").expect("valid run id"),
        workflow_id: WorkflowId::new("workflow/write-candidate").expect("valid workflow id"),
        schema_version: SchemaVersion::new("workflowos.dev/v0").expect("valid schema version"),
        workflow_version: WorkflowVersion::new("v1").expect("valid workflow version"),
        spec_content_hash: SpecContentHash::from_text("write-candidate-spec"),
        step_id: StepId::new("step/comment").expect("valid step id"),
        skill_id: SkillId::new("skill/github-comment").expect("valid skill id"),
        skill_version: SkillVersion::new("v1").expect("valid skill version"),
        requested_by: ActorId::new("system/workflow-os").expect("valid actor"),
        correlation_id: CorrelationId::new("correlation/github-pr-comment")
            .expect("valid correlation id"),
        idempotency_key: Some(
            IdempotencyKey::new("approval-github-pr-comment").expect("valid idempotency"),
        ),
        reason: "approval required for governed GitHub PR comment attempt".to_owned(),
        requested_at: Timestamp::parse_rfc3339("2026-06-20T12:00:00Z").expect("valid timestamp"),
        expires_after: Some("30m".to_owned()),
        expires_at: None,
        decision: None,
    }
}

fn approval_decision(approval_id: &str, decision: ApprovalDecisionKind) -> ApprovalDecision {
    ApprovalDecision {
        approval_id: approval_id.to_owned(),
        actor: ActorId::new("user/reviewer").expect("valid actor"),
        decided_at: Timestamp::parse_rfc3339("2026-06-20T12:00:30Z").expect("valid timestamp"),
        decision,
        reason: "bounded delegated approval decision".to_owned(),
        correlation_id: CorrelationId::new("correlation/github-pr-comment")
            .expect("valid correlation id"),
        proof_marker: None,
    }
}

fn run_with_approval(decision: ApprovalDecisionKind) -> WorkflowRun {
    let approval_id = "approval/github-comment-approved";
    let decision_event = match decision {
        ApprovalDecisionKind::Granted => {
            WorkflowRunEventKind::ApprovalGranted(approval_decision(approval_id, decision))
        }
        ApprovalDecisionKind::Denied => {
            WorkflowRunEventKind::ApprovalDenied(approval_decision(approval_id, decision))
        }
    };
    WorkflowRun::rehydrate(&[
        workflow_event(1, WorkflowRunEventKind::RunCreated { summary: None }),
        workflow_event(2, WorkflowRunEventKind::RunValidated),
        workflow_event(3, WorkflowRunEventKind::RunStarted),
        workflow_event(
            4,
            WorkflowRunEventKind::ApprovalRequested(Box::new(approval_request(approval_id))),
        ),
        workflow_event(5, decision_event),
    ])
    .expect("valid approval run")
}

fn side_effect_append_input() -> GitHubPullRequestCommentSideEffectAppendInput {
    GitHubPullRequestCommentSideEffectAppendInput {
        side_effect_id: side_effect_id(),
        context: side_effect_event_context(),
        step_id: StepId::new("step/comment").expect("valid step id"),
        skill_id: SkillId::new("local/github-comment").expect("valid skill id"),
        skill_version: SkillVersion::new("v1").expect("valid skill version"),
        correlation_id: Some(
            CorrelationId::new("correlation/github-pr-comment").expect("valid correlation id"),
        ),
    }
}

fn persisted_proposed_record(store: &LocalStateBackend) -> workflow_core::SideEffectRecord {
    compose_and_persist_github_pr_comment_proposed_side_effect_record(
        store,
        &preflighted_write(),
        Some(&valid_fixture_response()),
        side_effect_record_input(),
    )
    .expect("persisted proposed record")
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

fn valid_fixture_response() -> GitHubPullRequestCommentWriteResponse {
    GitHubPullRequestCommentWriteResponse::new(valid_response_definition())
        .expect("valid fixture response")
}

#[test]
fn sandbox_readiness_all_gates_satisfied_allows_without_mutation_authority() {
    let input = sandbox_readiness_input();

    let result = assess_provider_write_sandbox_readiness(&input).expect("readiness result");

    assert_eq!(
        result.decision(),
        ProviderWriteSandboxReadinessDecision::AllowedForSandbox
    );
    assert!(result.issues().is_empty());
    assert!(!result.retry_blocked());
    assert!(!result.operator_action_required());
    assert!(!result.provider_call_allowed());
    assert!(!result.workflow_event_append_allowed());
    assert!(!result.side_effect_record_write_allowed());
    assert!(!result.report_artifact_write_allowed());
}

#[test]
fn sandbox_readiness_missing_explicit_auth_is_denied() {
    let mut input = sandbox_readiness_input();
    input.auth_posture = ProviderWriteSandboxAuthPosture::Missing;

    let result = assess_provider_write_sandbox_readiness(&input).expect("readiness result");

    assert_eq!(
        result.decision(),
        ProviderWriteSandboxReadinessDecision::Denied
    );
    assert!(result
        .issues()
        .contains(&ProviderWriteSandboxReadinessIssue::AuthNotExplicit));
    assert!(result.retry_blocked());
    assert!(result.operator_action_required());
}

#[test]
fn sandbox_readiness_hidden_or_ambient_auth_is_denied() {
    let mut input = sandbox_readiness_input();
    input.auth_posture = ProviderWriteSandboxAuthPosture::HiddenOrAmbient;

    let result = assess_provider_write_sandbox_readiness(&input).expect("readiness result");

    assert_eq!(
        result.decision(),
        ProviderWriteSandboxReadinessDecision::Denied
    );
    assert!(result
        .issues()
        .contains(&ProviderWriteSandboxReadinessIssue::AuthNotExplicit));
    assert!(result.retry_blocked());
    assert!(result.operator_action_required());
}

#[test]
fn sandbox_readiness_unknown_auth_is_denied() {
    let mut input = sandbox_readiness_input();
    input.auth_posture = ProviderWriteSandboxAuthPosture::Unknown;

    let result = assess_provider_write_sandbox_readiness(&input).expect("readiness result");

    assert_eq!(
        result.decision(),
        ProviderWriteSandboxReadinessDecision::Denied
    );
    assert!(result
        .issues()
        .contains(&ProviderWriteSandboxReadinessIssue::AuthNotExplicit));
}

#[test]
fn sandbox_readiness_missing_required_approval_is_denied() {
    let mut input = sandbox_readiness_input();
    input.approval_posture = ProviderWriteSandboxApprovalPosture::Missing;

    let result = assess_provider_write_sandbox_readiness(&input).expect("readiness result");

    assert_eq!(
        result.decision(),
        ProviderWriteSandboxReadinessDecision::Denied
    );
    assert!(result
        .issues()
        .contains(&ProviderWriteSandboxReadinessIssue::ApprovalMissing));
}

#[test]
fn sandbox_readiness_missing_attempted_side_effect_is_denied() {
    let mut input = sandbox_readiness_input();
    input.side_effect_posture = ProviderWriteSandboxSideEffectPosture::NotAttempted;

    let result = assess_provider_write_sandbox_readiness(&input).expect("readiness result");

    assert_eq!(
        result.decision(),
        ProviderWriteSandboxReadinessDecision::Denied
    );
    assert!(result
        .issues()
        .contains(&ProviderWriteSandboxReadinessIssue::SideEffectAttemptMissing));
}

#[test]
fn sandbox_readiness_missing_event_proof_is_denied_when_required() {
    let mut input = sandbox_readiness_input();
    input.event_proof_posture = ProviderWriteSandboxEventProofPosture::Missing;

    let result = assess_provider_write_sandbox_readiness(&input).expect("readiness result");

    assert_eq!(
        result.decision(),
        ProviderWriteSandboxReadinessDecision::Denied
    );
    assert!(result
        .issues()
        .contains(&ProviderWriteSandboxReadinessIssue::EventProofMissing));
}

#[test]
fn sandbox_readiness_ambiguous_provider_local_posture_is_deferred() {
    let mut input = sandbox_readiness_input();
    input.provider_local_posture = ProviderWriteSandboxProviderLocalPosture::Ambiguous;

    let result = assess_provider_write_sandbox_readiness(&input).expect("readiness result");

    assert_eq!(
        result.decision(),
        ProviderWriteSandboxReadinessDecision::Deferred
    );
    assert!(result
        .issues()
        .contains(&ProviderWriteSandboxReadinessIssue::ProviderLocalAmbiguous));
    assert!(result.retry_blocked());
    assert!(result.operator_action_required());
}

#[test]
fn sandbox_readiness_production_like_target_is_denied() {
    let mut input = sandbox_readiness_input();
    input.target_posture = ProviderWriteSandboxTargetPosture::ProductionLike;

    let result = assess_provider_write_sandbox_readiness(&input).expect("readiness result");

    assert_eq!(
        result.decision(),
        ProviderWriteSandboxReadinessDecision::Denied
    );
    assert!(result
        .issues()
        .contains(&ProviderWriteSandboxReadinessIssue::TargetNotSandbox));
}

#[test]
fn sandbox_readiness_unsupported_capability_is_denied() {
    let mut input = sandbox_readiness_input();
    input.capability = AdapterWriteCapability::GitHubMerge;

    let result = assess_provider_write_sandbox_readiness(&input).expect("readiness result");

    assert_eq!(
        result.decision(),
        ProviderWriteSandboxReadinessDecision::Denied
    );
    assert!(result
        .issues()
        .contains(&ProviderWriteSandboxReadinessIssue::UnsupportedCapability));
}

#[test]
fn sandbox_readiness_debug_and_serialization_do_not_leak_sensitive_inputs() {
    let input = sandbox_readiness_input();

    let input_debug = format!("{input:?}");
    assert!(!input_debug.contains("sandbox-owner"));
    assert!(!input_debug.contains("sandbox-repo"));
    assert!(!input_debug.contains("comment_body"));
    assert!(!input_debug.contains("bounded comment text only"));

    let result = assess_provider_write_sandbox_readiness(&input).expect("readiness result");
    let result_debug = format!("{result:?}");
    assert!(!result_debug.contains("sandbox-owner"));
    assert!(!result_debug.contains("sandbox-repo"));
    assert!(!result_debug.contains("comment_body"));
    assert!(!result_debug.contains("bounded comment text only"));

    let serialized = serde_json::to_string(&result).expect("serialize readiness result");
    assert!(!serialized.contains("sandbox-owner"));
    assert!(!serialized.contains("sandbox-repo"));
    assert!(!serialized.contains("token"));
    assert!(!serialized.contains("provider payload"));
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
fn proposed_side_effect_record_persistence_writes_fixture_record_to_store() {
    let state = test_state_backend();
    let preflighted = preflighted_write();
    let fixture = valid_fixture();
    let response = validate_github_pr_comment_fixture_write(&preflighted, &fixture)
        .expect("valid fixture response");

    let record = compose_and_persist_github_pr_comment_proposed_side_effect_record(
        state.backend(),
        &preflighted,
        Some(&response),
        side_effect_record_input(),
    )
    .expect("persisted proposed record");

    assert_eq!(record.lifecycle_state(), SideEffectLifecycleState::Proposed);
    assert_eq!(
        record.side_effect_id().as_str(),
        "side-effect/github-pr-comment"
    );
    let read_back = state
        .backend()
        .read_side_effect_record(record.side_effect_id())
        .expect("read persisted record")
        .expect("record exists");
    assert_eq!(read_back, record);
    let listed = state
        .backend()
        .list_side_effect_records(record.run_id())
        .expect("list persisted records");
    assert_eq!(listed, vec![record]);
}

#[test]
fn proposed_side_effect_record_persistence_writes_dry_run_record_to_store() {
    let state = test_state_backend();
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

    let record = compose_and_persist_github_pr_comment_proposed_side_effect_record(
        state.backend(),
        &preflighted,
        Some(&response),
        side_effect_record_input(),
    )
    .expect("persisted dry-run proposed record");

    assert_eq!(record.lifecycle_state(), SideEffectLifecycleState::Proposed);
    assert_eq!(record.outcome_reference(), None);
    assert_eq!(
        state
            .backend()
            .list_side_effect_records(record.run_id())
            .expect("list persisted records")
            .len(),
        1
    );
}

#[test]
fn proposed_side_effect_record_persistence_rejects_duplicate_without_leakage() {
    let state = test_state_backend();
    let preflighted = preflighted_write();

    compose_and_persist_github_pr_comment_proposed_side_effect_record(
        state.backend(),
        &preflighted,
        None,
        side_effect_record_input(),
    )
    .expect("first proposed record persisted");

    let error = compose_and_persist_github_pr_comment_proposed_side_effect_record(
        state.backend(),
        &preflighted,
        None,
        side_effect_record_input(),
    )
    .expect_err("duplicate proposed record rejected");

    assert_eq!(
        error.code(),
        "github_pr_comment_side_effect_record.persistence.duplicate"
    );
    let debug = format!("{error:?}");
    assert!(!debug.contains("side-effect/github-pr-comment"));
    assert!(!debug.contains("run/github-pr-comment"));
    assert_eq!(
        state
            .backend()
            .list_side_effect_records(preflighted.request().run_id())
            .expect("list persisted records")
            .len(),
        1
    );
}

#[test]
fn proposed_side_effect_record_persistence_rejects_provider_response_before_store_write() {
    let state = test_state_backend();
    let preflighted = preflighted_write();
    let mut response_definition = valid_response_definition();
    response_definition.mode = GitHubPullRequestCommentWriteMode::LiveSandbox;
    response_definition.outcome = GitHubPullRequestCommentWriteOutcome::ProviderSucceeded;
    response_definition.provider_comment_reference = Some("github/comment/123".to_owned());
    let response = GitHubPullRequestCommentWriteResponse::new(response_definition)
        .expect("valid future provider response model");

    let error = compose_and_persist_github_pr_comment_proposed_side_effect_record(
        state.backend(),
        &preflighted,
        Some(&response),
        side_effect_record_input(),
    )
    .expect_err("provider responses are rejected before persistence");

    assert_eq!(
        error.code(),
        "github_pr_comment_side_effect_record.response.unsupported"
    );
    assert!(!format!("{error:?}").contains("github/comment/123"));
    assert!(state
        .backend()
        .read_side_effect_record(preflighted.request().side_effect_id())
        .expect("read side-effect record")
        .is_none());
}

#[test]
fn proposed_side_effect_record_persistence_rejects_secret_like_summary_before_store_write() {
    let state = test_state_backend();
    let preflighted = preflighted_write();
    let mut input = side_effect_record_input();
    input.summary_override = Some("raw_provider_payload must not be copied".to_owned());

    let error = compose_and_persist_github_pr_comment_proposed_side_effect_record(
        state.backend(),
        &preflighted,
        None,
        input,
    )
    .expect_err("secret-like summary rejected before persistence");

    assert_eq!(error.code(), "github_pr_comment_write.secret_like_value");
    assert!(!format!("{error:?}").contains("raw_provider_payload"));
    assert!(state
        .backend()
        .read_side_effect_record(preflighted.request().side_effect_id())
        .expect("read side-effect record")
        .is_none());
}

#[test]
fn write_attempt_orchestration_persists_and_transitions_without_provider_call() {
    let state = test_state_backend();
    let preflighted = preflighted_write();
    let response = validate_github_pr_comment_fixture_write(&preflighted, &valid_fixture())
        .expect("valid fixture response");
    let run = run_with_approval(ApprovalDecisionKind::Granted);
    let initial_event_count = run.events.len();

    let result = orchestrate_github_pr_comment_write_attempt_without_provider_call(
        state.backend(),
        &preflighted,
        orchestration_input(Some(&response), Some(&run)),
    )
    .expect("attempt orchestration succeeds");

    assert_eq!(
        result.proposed_record().lifecycle_state(),
        SideEffectLifecycleState::Proposed
    );
    assert_eq!(
        result.attempted_transition().record().lifecycle_state(),
        SideEffectLifecycleState::Attempted
    );
    assert_eq!(
        result
            .approval_linkage()
            .expect("approval linkage exists")
            .linked_approval_reference_count(),
        1
    );
    assert!(!result.provider_call_performed());
    assert!(!result.workflow_event_appended());
    assert!(!result.report_artifact_written());
    assert_eq!(run.events.len(), initial_event_count);

    let stored = state
        .backend()
        .read_side_effect_record(preflighted.request().side_effect_id())
        .expect("read stored record")
        .expect("record exists");
    assert_eq!(
        stored.lifecycle_state(),
        SideEffectLifecycleState::Attempted
    );
    assert!(stored.outcome_reference().is_none());
    assert_eq!(
        result.attempted_transition().event().lifecycle_state(),
        SideEffectLifecycleState::Attempted
    );
}

#[test]
fn write_attempt_orchestration_rejects_missing_approval_run_before_attempt() {
    let state = test_state_backend();
    let preflighted = preflighted_write();

    let error = orchestrate_github_pr_comment_write_attempt_without_provider_call(
        state.backend(),
        &preflighted,
        orchestration_input(None, None),
    )
    .expect_err("missing approval run rejected");

    assert_eq!(
        error.code(),
        "github_pr_comment_write_attempt.approval_run_missing"
    );
    assert!(!format!("{error:?}").contains("approval/github-comment-approved"));
    let stored = state
        .backend()
        .read_side_effect_record(preflighted.request().side_effect_id())
        .expect("read stored record")
        .expect("proposed record remains inspectable");
    assert_eq!(stored.lifecycle_state(), SideEffectLifecycleState::Proposed);
}

#[test]
fn write_attempt_orchestration_rejects_denied_approval_before_attempt() {
    let state = test_state_backend();
    let preflighted = preflighted_write();
    let run = run_with_approval(ApprovalDecisionKind::Denied);

    let error = orchestrate_github_pr_comment_write_attempt_without_provider_call(
        state.backend(),
        &preflighted,
        orchestration_input(None, Some(&run)),
    )
    .expect_err("denied approval rejected");

    assert_eq!(
        error.code(),
        "side_effect_approval_linkage.decision_kind_mismatch"
    );
    assert!(!format!("{error:?}").contains("github-pr-comment"));
    let stored = state
        .backend()
        .read_side_effect_record(preflighted.request().side_effect_id())
        .expect("read stored record")
        .expect("proposed record remains inspectable");
    assert_eq!(stored.lifecycle_state(), SideEffectLifecycleState::Proposed);
}

#[test]
fn write_attempt_orchestration_rejects_secret_like_attempt_summary_without_store_write() {
    let state = test_state_backend();
    let preflighted = preflighted_write();
    let run = run_with_approval(ApprovalDecisionKind::Granted);
    let mut input = orchestration_input(None, Some(&run));
    input.transition_summary = Some("token=raw_provider_payload".to_owned());

    let error = orchestrate_github_pr_comment_write_attempt_without_provider_call(
        state.backend(),
        &preflighted,
        input,
    )
    .expect_err("secret-like transition summary rejected");

    assert_eq!(error.code(), "github_pr_comment_write.secret_like_value");
    assert!(!format!("{error:?}").contains("raw_provider_payload"));
    assert!(state
        .backend()
        .read_side_effect_record(preflighted.request().side_effect_id())
        .expect("read side-effect record")
        .is_none());
}

#[test]
fn write_attempt_orchestration_debug_redacts_sensitive_values() {
    let run = run_with_approval(ApprovalDecisionKind::Granted);
    let input = orchestration_input(None, Some(&run));
    let input_debug = format!("{input:?}");

    assert!(input_debug.contains("GitHubPullRequestCommentWriteAttemptOrchestrationInput"));
    assert!(!input_debug.contains("run/github-pr-comment"));
    assert!(!input_debug.contains("approval/github-comment-approved"));
    assert!(!input_debug.contains("attempt boundary recorded"));

    let state = test_state_backend();
    let result = orchestrate_github_pr_comment_write_attempt_without_provider_call(
        state.backend(),
        &preflighted_write(),
        input,
    )
    .expect("attempt orchestration succeeds");
    let result_debug = format!("{result:?}");

    assert!(result_debug.contains("GitHubPullRequestCommentWriteAttemptOrchestrationResult"));
    assert!(result_debug.contains("provider_call_performed: false"));
    assert!(!result_debug.contains("run/github-pr-comment"));
    assert!(!result_debug.contains("approval/github-comment-approved"));
    assert!(!result_debug.contains("Workflow OS governed comment preview"));
}

#[test]
fn no_provider_outcome_orchestration_completes_attempted_record_without_provider_call() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());

    let result = orchestrate_github_pr_comment_no_provider_outcome(
        state.backend(),
        attempted.side_effect_id(),
        no_provider_completed_input(),
    )
    .expect("local completed outcome succeeds");

    assert_eq!(
        result.outcome_transition().record().lifecycle_state(),
        SideEffectLifecycleState::Completed
    );
    assert_eq!(
        result
            .outcome_transition()
            .record()
            .outcome_reference()
            .expect("outcome reference")
            .kind(),
        SideEffectOutcomeReferenceKind::Outcome
    );
    assert_eq!(
        result.outcome_transition().event().lifecycle_state(),
        SideEffectLifecycleState::Completed
    );
    assert_eq!(
        result
            .outcome_transition()
            .event()
            .outcome_reference_count(),
        1
    );
    assert!(!result.provider_call_performed());
    assert!(!result.workflow_event_appended());
    assert!(!result.report_artifact_written());

    let stored = state
        .backend()
        .read_side_effect_record(attempted.side_effect_id())
        .expect("read stored record")
        .expect("record exists");
    assert_eq!(
        stored.lifecycle_state(),
        SideEffectLifecycleState::Completed
    );
}

#[test]
fn no_provider_outcome_orchestration_fails_attempted_record_with_reason_code() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());

    let result = orchestrate_github_pr_comment_no_provider_outcome(
        state.backend(),
        attempted.side_effect_id(),
        no_provider_failed_input(),
    )
    .expect("local failed outcome succeeds");

    assert_eq!(
        result.outcome_transition().record().lifecycle_state(),
        SideEffectLifecycleState::Failed
    );
    assert_eq!(
        result.outcome_transition().record().reason_codes(),
        ["local.fixture_validation_failed".to_owned()]
    );
    assert_eq!(
        result
            .outcome_transition()
            .record()
            .outcome_reference()
            .expect("failure reference")
            .kind(),
        SideEffectOutcomeReferenceKind::Failure
    );
    assert!(!result.provider_call_performed());
    assert!(!result.workflow_event_appended());
    assert!(!result.report_artifact_written());
}

#[test]
fn no_provider_outcome_orchestration_rejects_non_attempted_record() {
    let state = test_state_backend();
    let record = persisted_proposed_record(state.backend());

    let error = orchestrate_github_pr_comment_no_provider_outcome(
        state.backend(),
        record.side_effect_id(),
        no_provider_completed_input(),
    )
    .expect_err("proposed record cannot be closed as outcome");

    assert_eq!(
        error.code(),
        "github_pr_comment_write_outcome.unsupported_lifecycle"
    );
    assert!(!format!("{error:?}").contains("github-pr-comment"));
    let stored = state
        .backend()
        .read_side_effect_record(record.side_effect_id())
        .expect("read stored record")
        .expect("record exists");
    assert_eq!(stored.lifecycle_state(), SideEffectLifecycleState::Proposed);
}

#[test]
fn no_provider_outcome_orchestration_rejects_provider_outcome_reference() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let mut input = no_provider_completed_input();
    input.outcome = GitHubPullRequestCommentNoProviderOutcome::Completed {
        outcome_reference: SideEffectOutcomeReference::new(
            SideEffectOutcomeReferenceKind::Outcome,
            "provider/github-comment-42",
        )
        .expect("valid provider-shaped reference"),
    };

    let error = orchestrate_github_pr_comment_no_provider_outcome(
        state.backend(),
        attempted.side_effect_id(),
        input,
    )
    .expect_err("provider reference rejected in no-provider lane");

    assert_eq!(
        error.code(),
        "github_pr_comment_write_outcome.provider_reference_not_allowed"
    );
    assert!(!format!("{error:?}").contains("provider/github-comment-42"));
    let stored = state
        .backend()
        .read_side_effect_record(attempted.side_effect_id())
        .expect("read stored record")
        .expect("record exists");
    assert_eq!(
        stored.lifecycle_state(),
        SideEffectLifecycleState::Attempted
    );
}

#[test]
fn no_provider_outcome_orchestration_rejects_secret_like_summary_without_transition() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let mut input = no_provider_failed_input();
    input.transition_summary = Some("token=raw_provider_payload".to_owned());

    let error = orchestrate_github_pr_comment_no_provider_outcome(
        state.backend(),
        attempted.side_effect_id(),
        input,
    )
    .expect_err("secret-like outcome summary rejected");

    assert_eq!(error.code(), "github_pr_comment_write.secret_like_value");
    assert!(!format!("{error:?}").contains("raw_provider_payload"));
    let stored = state
        .backend()
        .read_side_effect_record(attempted.side_effect_id())
        .expect("read stored record")
        .expect("record exists");
    assert_eq!(
        stored.lifecycle_state(),
        SideEffectLifecycleState::Attempted
    );
}

#[test]
fn no_provider_outcome_orchestration_debug_redacts_sensitive_values() {
    let input = no_provider_completed_input();
    let input_debug = format!("{input:?}");

    assert!(input_debug.contains("GitHubPullRequestCommentNoProviderOutcomeOrchestrationInput"));
    assert!(!input_debug.contains("fixture/github-pr-comment-42/completed"));
    assert!(!input_debug.contains("fixture outcome closed without provider call"));

    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let result = orchestrate_github_pr_comment_no_provider_outcome(
        state.backend(),
        attempted.side_effect_id(),
        input,
    )
    .expect("local completed outcome succeeds");
    let result_debug = format!("{result:?}");

    assert!(result_debug.contains("GitHubPullRequestCommentNoProviderOutcomeOrchestrationResult"));
    assert!(result_debug.contains("provider_call_performed: false"));
    assert!(!result_debug.contains("fixture/github-pr-comment-42/completed"));
    assert!(!result_debug.contains("Workflow OS governed comment preview"));
}

#[test]
fn proposed_side_effect_event_composes_from_persisted_record_without_provider_payload() {
    let state = test_state_backend();
    let record = persisted_proposed_record(state.backend());

    let event =
        compose_github_pr_comment_proposed_side_effect_event(&record, &side_effect_event_context())
            .expect("valid proposed side-effect event");

    assert_eq!(event.lifecycle_state(), SideEffectLifecycleState::Proposed);
    assert_eq!(event.side_effect_id(), record.side_effect_id());
    assert_eq!(event.step_id(), record.step_id());
    assert_eq!(event.skill_id(), record.skill_id());
    assert_eq!(event.skill_version(), record.skill_version());
    assert_eq!(event.correlation_id(), record.correlation_id());
    assert_eq!(event.references(), record.references());
    assert_eq!(event.evidence_reference_count(), 1);
    assert_eq!(event.outcome_reference_count(), 0);
    assert_eq!(event.sensitivity(), record.sensitivity());

    let debug = format!("{event:?}");
    assert!(!debug.contains("workflow-os"));
    assert!(!debug.contains("kernel"));
    assert!(!debug.contains("bounded GitHub PR comment write request summary"));
    assert!(!debug.contains("Workflow OS governed comment preview"));

    let serialized = serde_json::to_string(&event).expect("event serializes");
    assert!(!serialized.contains("workflow-os"));
    assert!(!serialized.contains("kernel"));
    assert!(!serialized.contains("bounded GitHub PR comment write request summary"));
    assert!(!serialized.contains("Workflow OS governed comment preview"));
}

#[test]
fn proposed_side_effect_event_loads_from_store_by_stable_id() {
    let state = test_state_backend();
    let record = persisted_proposed_record(state.backend());

    let event = load_github_pr_comment_proposed_side_effect_event(
        state.backend(),
        record.side_effect_id(),
        &side_effect_event_context(),
    )
    .expect("event loaded from store");

    assert_eq!(event.lifecycle_state(), SideEffectLifecycleState::Proposed);
    assert_eq!(event.side_effect_id(), record.side_effect_id());
}

#[test]
fn proposed_side_effect_event_input_loads_persisted_record_for_executor_append() {
    let state = test_state_backend();
    let record = persisted_proposed_record(state.backend());

    let input = load_github_pr_comment_proposed_side_effect_event_input(
        state.backend(),
        side_effect_append_input(),
    )
    .expect("executor side-effect event input");

    assert_eq!(input.step_id.as_str(), "step/comment");
    assert_eq!(input.skill_id.as_str(), "local/github-comment");
    assert_eq!(input.skill_version.as_str(), "v1");
    assert_eq!(
        input.event.lifecycle_state(),
        SideEffectLifecycleState::Proposed
    );
    assert_eq!(input.event.side_effect_id(), record.side_effect_id());
    assert_eq!(input.event.step_id(), record.step_id());
    assert_eq!(input.event.references(), record.references());

    let debug = format!("{input:?}");
    assert!(debug.contains("LocalExecutionSideEffectEventInput"));
    assert!(!debug.contains("step/comment"));
    assert!(!debug.contains("local/github-comment"));
    assert!(!debug.contains("github-pr-comment"));
}

#[test]
fn proposed_side_effect_event_rejects_missing_store_record_without_leakage() {
    let state = test_state_backend();
    let missing_id = SideEffectId::new("side-effect/missing-github-comment").expect("valid id");

    let error = load_github_pr_comment_proposed_side_effect_event(
        state.backend(),
        &missing_id,
        &side_effect_event_context(),
    )
    .expect_err("missing record rejected");

    assert_eq!(
        error.code(),
        "github_pr_comment_side_effect_event.record_missing"
    );
    assert!(!format!("{error:?}").contains("missing-github-comment"));
}

#[test]
fn proposed_side_effect_event_input_maps_missing_record_without_leakage() {
    let state = test_state_backend();

    let error = load_github_pr_comment_proposed_side_effect_event_input(
        state.backend(),
        side_effect_append_input(),
    )
    .expect_err("missing record rejected");

    assert_eq!(
        error.code(),
        "github_pr_comment_side_effect_event_input.record_missing"
    );
    assert!(!format!("{error:?}").contains("github-pr-comment"));
}

#[test]
fn proposed_side_effect_event_rejects_non_proposed_record() {
    let record = SideEffectRecord::new(SideEffectRecordDefinition {
        side_effect_id: SideEffectId::new("side-effect/github-pr-comment-skipped")
            .expect("valid side-effect id"),
        lifecycle_state: SideEffectLifecycleState::Skipped,
        target: SideEffectTargetReference::new(
            SideEffectTargetKind::AdapterResource,
            "github/workflow-os/kernel/pull/42",
        )
        .expect("valid target"),
        capability: SideEffectCapability::GitHubWrite,
        authority: SideEffectAuthority::new(
            SideEffectAuthorityDecision::AllowedByPolicy,
            vec![policy_ref()],
            Vec::new(),
        )
        .expect("valid authority"),
        actor: Some(ActorId::new("system/workflow-os").expect("valid actor")),
        system_actor: None,
        workflow_id: WorkflowId::new("workflow/write-candidate").expect("valid workflow id"),
        workflow_version: WorkflowVersion::new("v1").expect("valid workflow version"),
        schema_version: SchemaVersion::new("workflowos.dev/v0").expect("valid schema version"),
        spec_hash: SpecContentHash::from_text("write-candidate-spec"),
        run_id: WorkflowRunId::new("run/github-pr-comment").expect("valid run id"),
        step_id: Some(StepId::new("step/comment").expect("valid step id")),
        skill_id: None,
        skill_version: None,
        adapter_id: Some(AdapterId::new("adapter/github").expect("valid adapter id")),
        adapter_kind: Some(AdapterKind::GitHub),
        integration_id: Some(
            IntegrationId::new("integration/github/sandbox").expect("valid integration id"),
        ),
        idempotency: SideEffectIdempotencyBinding::new(
            IdempotencyKey::new("github-pr-comment-42").expect("valid idempotency key"),
            SideEffectIdempotencyScope::Run,
            None,
            None,
        )
        .expect("valid idempotency"),
        references: vec![policy_ref()],
        outcome_reference: None,
        created_at: Timestamp::parse_rfc3339("2026-06-20T12:00:00Z").expect("valid timestamp"),
        updated_at: None,
        correlation_id: Some(
            CorrelationId::new("correlation/github-pr-comment").expect("valid correlation id"),
        ),
        summary: Some("bounded skipped GitHub PR comment summary".to_owned()),
        reason_codes: vec!["skipped-before-provider-write".to_owned()],
        sensitivity: SideEffectSensitivity::Internal,
        redaction: redaction(),
    })
    .expect("valid skipped record");

    let error =
        compose_github_pr_comment_proposed_side_effect_event(&record, &side_effect_event_context())
            .expect_err("non-proposed record rejected");

    assert_eq!(
        error.code(),
        "github_pr_comment_side_effect_event.unsupported_lifecycle"
    );
    assert!(!format!("{error:?}").contains("github-pr-comment-skipped"));
}

#[test]
fn proposed_side_effect_event_input_rejects_step_mismatch_without_leakage() {
    let state = test_state_backend();
    persisted_proposed_record(state.backend());
    let mut input = side_effect_append_input();
    input.step_id = StepId::new("step/other-comment").expect("valid step id");

    let error = load_github_pr_comment_proposed_side_effect_event_input(state.backend(), input)
        .expect_err("step mismatch rejected");

    assert_eq!(
        error.code(),
        "github_pr_comment_side_effect_event_input.target_mismatch"
    );
    let debug = format!("{error:?}");
    assert!(!debug.contains("step/comment"));
    assert!(!debug.contains("step/other-comment"));
}

#[test]
fn proposed_side_effect_event_input_rejects_correlation_mismatch_without_leakage() {
    let state = test_state_backend();
    persisted_proposed_record(state.backend());
    let mut input = side_effect_append_input();
    input.correlation_id =
        Some(CorrelationId::new("correlation/other-comment").expect("valid correlation id"));

    let error = load_github_pr_comment_proposed_side_effect_event_input(state.backend(), input)
        .expect_err("correlation mismatch rejected");

    assert_eq!(
        error.code(),
        "github_pr_comment_side_effect_event_input.correlation_mismatch"
    );
    let debug = format!("{error:?}");
    assert!(!debug.contains("github-pr-comment"));
    assert!(!debug.contains("other-comment"));
}

#[test]
fn proposed_side_effect_event_rejects_identity_mismatch_without_leakage() {
    let state = test_state_backend();
    let record = persisted_proposed_record(state.backend());
    let mut context = side_effect_event_context();
    context.run_id = WorkflowRunId::new("run/other-github-comment").expect("valid run id");

    let error = compose_github_pr_comment_proposed_side_effect_event(&record, &context)
        .expect_err("identity mismatch rejected");

    assert_eq!(
        error.code(),
        "github_pr_comment_side_effect_event.identity_mismatch"
    );
    let debug = format!("{error:?}");
    assert!(!debug.contains("run/github-pr-comment"));
    assert!(!debug.contains("run/other-github-comment"));
    assert!(!debug.contains("workflow-os"));
    assert!(!debug.contains("kernel"));
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

fn provider_auth() -> GitHubPullRequestCommentProviderAuth {
    GitHubPullRequestCommentProviderAuth::new(
        "ghp_test_auth_value_for_injected_provider",
        Some("sandbox pull request comments only".to_owned()),
    )
    .expect("valid provider auth")
}

fn provider_call_input(
    attempted_record: &SideEffectRecord,
) -> GitHubPullRequestCommentProviderCallInput<'_> {
    GitHubPullRequestCommentProviderCallInput {
        attempted_record,
        target: target(),
        comment_body: "Workflow OS governed live sandbox comment.".to_owned(),
        idempotency_key: idempotency_key(),
        mode: GitHubPullRequestCommentWriteMode::LiveSandbox,
        auth: provider_auth(),
        live_call_enabled: true,
        provider_call_enabled: true,
        summary: "bounded provider-call request summary".to_owned(),
        sensitivity: SideEffectSensitivity::Internal,
        redaction: redaction(),
    }
}

fn provider_call_orchestration_input(
    attempted_record: &SideEffectRecord,
) -> GitHubPullRequestCommentProviderCallOrchestrationInput<'_> {
    GitHubPullRequestCommentProviderCallOrchestrationInput {
        provider_call: provider_call_input(attempted_record),
        transitioned_at: Timestamp::parse_rfc3339("2026-06-20T12:03:00Z").expect("valid timestamp"),
        transition_references: vec![SideEffectReference::new(
            SideEffectReferenceKind::EvidenceReference,
            "evidence/github-pr-comment-provider-call",
        )
        .expect("valid reference")],
        evidence_reference_count: 1,
    }
}

#[derive(Clone, Copy)]
enum MockProviderOutcome {
    Succeeded,
    Failed,
    Fixture,
    Unclassified,
}

struct MockProvider {
    outcome: MockProviderOutcome,
}

impl GitHubPullRequestCommentProvider for MockProvider {
    fn create_pull_request_comment(
        &self,
        request: &GitHubPullRequestCommentProviderCallRequest,
    ) -> Result<GitHubPullRequestCommentWriteResponse, workflow_core::WorkflowOsError> {
        assert_eq!(
            request.comment_body(),
            "Workflow OS governed live sandbox comment."
        );
        match self.outcome {
            MockProviderOutcome::Succeeded => GitHubPullRequestCommentWriteResponse::new(
                GitHubPullRequestCommentWriteResponseDefinition {
                    correlation_id: CorrelationId::new("correlation/provider-call-response")
                        .expect("valid correlation"),
                    mode: GitHubPullRequestCommentWriteMode::LiveSandbox,
                    outcome: GitHubPullRequestCommentWriteOutcome::ProviderSucceeded,
                    provider_comment_reference: Some("github/comment/123".to_owned()),
                    provider_error_code: None,
                    summary: "provider returned a bounded comment reference".to_owned(),
                    sensitivity: SideEffectSensitivity::Internal,
                    redaction: redaction(),
                },
            ),
            MockProviderOutcome::Failed => GitHubPullRequestCommentWriteResponse::new(
                GitHubPullRequestCommentWriteResponseDefinition {
                    correlation_id: CorrelationId::new("correlation/provider-call-response")
                        .expect("valid correlation"),
                    mode: GitHubPullRequestCommentWriteMode::LiveSandbox,
                    outcome: GitHubPullRequestCommentWriteOutcome::ProviderFailed,
                    provider_comment_reference: None,
                    provider_error_code: Some("github.rate_limited".to_owned()),
                    summary: "provider returned a bounded failure classification".to_owned(),
                    sensitivity: SideEffectSensitivity::Internal,
                    redaction: redaction(),
                },
            ),
            MockProviderOutcome::Fixture => GitHubPullRequestCommentWriteResponse::new(
                GitHubPullRequestCommentWriteResponseDefinition {
                    correlation_id: CorrelationId::new("correlation/provider-call-response")
                        .expect("valid correlation"),
                    mode: GitHubPullRequestCommentWriteMode::Fixture,
                    outcome: GitHubPullRequestCommentWriteOutcome::FixtureValidated,
                    provider_comment_reference: None,
                    provider_error_code: None,
                    summary: "fixture response is not a live provider response".to_owned(),
                    sensitivity: SideEffectSensitivity::Internal,
                    redaction: redaction(),
                },
            ),
            MockProviderOutcome::Unclassified => Err(workflow_core::WorkflowOsError::validation(
                "test.unclassified",
                "unclassified provider error",
            )),
        }
    }
}

struct ResponseCountingProvider {
    calls: RefCell<u32>,
    outcome: MockProviderOutcome,
}

impl ResponseCountingProvider {
    fn new(outcome: MockProviderOutcome) -> Self {
        Self {
            calls: RefCell::new(0),
            outcome,
        }
    }

    fn calls(&self) -> u32 {
        *self.calls.borrow()
    }
}

impl GitHubPullRequestCommentProvider for ResponseCountingProvider {
    fn create_pull_request_comment(
        &self,
        request: &GitHubPullRequestCommentProviderCallRequest,
    ) -> Result<GitHubPullRequestCommentWriteResponse, workflow_core::WorkflowOsError> {
        *self.calls.borrow_mut() += 1;
        MockProvider {
            outcome: match self.outcome {
                MockProviderOutcome::Succeeded => MockProviderOutcome::Succeeded,
                MockProviderOutcome::Failed => MockProviderOutcome::Failed,
                MockProviderOutcome::Fixture => MockProviderOutcome::Fixture,
                MockProviderOutcome::Unclassified => MockProviderOutcome::Unclassified,
            },
        }
        .create_pull_request_comment(request)
    }
}

struct CountingProvider<'a> {
    calls: &'a AtomicU64,
}

impl GitHubPullRequestCommentProvider for CountingProvider<'_> {
    fn create_pull_request_comment(
        &self,
        _request: &GitHubPullRequestCommentProviderCallRequest,
    ) -> Result<GitHubPullRequestCommentWriteResponse, workflow_core::WorkflowOsError> {
        self.calls.fetch_add(1, Ordering::Relaxed);
        Err(workflow_core::WorkflowOsError::validation(
            "test.provider_invoked",
            "provider should not be invoked",
        ))
    }
}

struct RecordingHttpTransport<'a> {
    calls: &'a AtomicU64,
    last_method: &'a RefCell<Option<String>>,
    last_url: &'a RefCell<Option<String>>,
    last_authorization_header: &'a RefCell<Option<String>>,
    last_body: &'a RefCell<Option<String>>,
    last_request_debug: &'a RefCell<Option<String>>,
    response: Result<GitHubPullRequestCommentHttpResponse, WorkflowOsError>,
}

impl GitHubPullRequestCommentHttpTransport for RecordingHttpTransport<'_> {
    fn send(
        &self,
        request: &GitHubPullRequestCommentHttpRequest,
    ) -> Result<GitHubPullRequestCommentHttpResponse, WorkflowOsError> {
        self.calls.fetch_add(1, Ordering::Relaxed);
        *self.last_method.borrow_mut() = Some(request.method().to_owned());
        *self.last_url.borrow_mut() = Some(request.url().to_owned());
        *self.last_authorization_header.borrow_mut() =
            Some(request.authorization_header_for_transport().to_owned());
        *self.last_body.borrow_mut() = Some(request.body_for_transport().to_owned());
        *self.last_request_debug.borrow_mut() = Some(format!("{request:?}"));
        self.response.clone()
    }
}

struct HttpTransportProbe {
    calls: AtomicU64,
    last_method: RefCell<Option<String>>,
    last_url: RefCell<Option<String>>,
    last_authorization_header: RefCell<Option<String>>,
    last_body: RefCell<Option<String>>,
    last_request_debug: RefCell<Option<String>>,
}

impl HttpTransportProbe {
    fn new() -> Self {
        Self {
            calls: AtomicU64::new(0),
            last_method: RefCell::new(None),
            last_url: RefCell::new(None),
            last_authorization_header: RefCell::new(None),
            last_body: RefCell::new(None),
            last_request_debug: RefCell::new(None),
        }
    }

    fn transport(
        &self,
        response: Result<GitHubPullRequestCommentHttpResponse, WorkflowOsError>,
    ) -> RecordingHttpTransport<'_> {
        RecordingHttpTransport {
            calls: &self.calls,
            last_method: &self.last_method,
            last_url: &self.last_url,
            last_authorization_header: &self.last_authorization_header,
            last_body: &self.last_body,
            last_request_debug: &self.last_request_debug,
            response,
        }
    }

    fn calls(&self) -> u64 {
        self.calls.load(Ordering::Relaxed)
    }
}

fn http_provider<T>(transport: T) -> GitHubPullRequestCommentHttpProvider<T> {
    GitHubPullRequestCommentHttpProvider::new(
        transport,
        "https://api.github.test",
        provider_auth(),
        SideEffectSensitivity::Internal,
        redaction(),
    )
    .expect("valid HTTP provider")
}

struct RecordingLookupHttpTransport<'a> {
    calls: &'a AtomicU64,
    last_method: &'a RefCell<Option<String>>,
    last_url: &'a RefCell<Option<String>>,
    last_authorization_header: &'a RefCell<Option<String>>,
    last_expected_provider_reference: &'a RefCell<Option<String>>,
    last_expected_managed_marker: &'a RefCell<Option<String>>,
    last_request_debug: &'a RefCell<Option<String>>,
    response: Result<GitHubPullRequestCommentLookupHttpResponse, WorkflowOsError>,
}

impl GitHubPullRequestCommentLookupHttpTransport for RecordingLookupHttpTransport<'_> {
    fn send_lookup(
        &self,
        request: &GitHubPullRequestCommentLookupHttpRequest,
    ) -> Result<GitHubPullRequestCommentLookupHttpResponse, WorkflowOsError> {
        self.calls.fetch_add(1, Ordering::Relaxed);
        *self.last_method.borrow_mut() = Some(request.method().to_owned());
        *self.last_url.borrow_mut() = Some(request.url().to_owned());
        *self.last_authorization_header.borrow_mut() =
            Some(request.authorization_header_for_transport().to_owned());
        *self.last_expected_provider_reference.borrow_mut() =
            request.expected_provider_reference().map(str::to_owned);
        *self.last_expected_managed_marker.borrow_mut() =
            request.expected_managed_marker().map(str::to_owned);
        *self.last_request_debug.borrow_mut() = Some(format!("{request:?}"));
        self.response.clone()
    }
}

struct LookupHttpTransportProbe {
    calls: AtomicU64,
    last_method: RefCell<Option<String>>,
    last_url: RefCell<Option<String>>,
    last_authorization_header: RefCell<Option<String>>,
    last_expected_provider_reference: RefCell<Option<String>>,
    last_expected_managed_marker: RefCell<Option<String>>,
    last_request_debug: RefCell<Option<String>>,
}

impl LookupHttpTransportProbe {
    fn new() -> Self {
        Self {
            calls: AtomicU64::new(0),
            last_method: RefCell::new(None),
            last_url: RefCell::new(None),
            last_authorization_header: RefCell::new(None),
            last_expected_provider_reference: RefCell::new(None),
            last_expected_managed_marker: RefCell::new(None),
            last_request_debug: RefCell::new(None),
        }
    }

    fn transport(
        &self,
        response: Result<GitHubPullRequestCommentLookupHttpResponse, WorkflowOsError>,
    ) -> RecordingLookupHttpTransport<'_> {
        RecordingLookupHttpTransport {
            calls: &self.calls,
            last_method: &self.last_method,
            last_url: &self.last_url,
            last_authorization_header: &self.last_authorization_header,
            last_expected_provider_reference: &self.last_expected_provider_reference,
            last_expected_managed_marker: &self.last_expected_managed_marker,
            last_request_debug: &self.last_request_debug,
            response,
        }
    }

    fn calls(&self) -> u64 {
        self.calls.load(Ordering::Relaxed)
    }
}

fn lookup_http_client<T>(transport: T) -> GitHubPullRequestCommentLookupHttpClient<T> {
    GitHubPullRequestCommentLookupHttpClient::new(
        transport,
        "https://api.github.test",
        provider_auth(),
        SideEffectSensitivity::Internal,
        redaction(),
    )
    .expect("valid lookup HTTP client")
}

#[test]
fn provider_call_orchestration_completes_attempted_record_from_injected_provider_success() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let result = orchestrate_github_pr_comment_provider_call(
        state.backend(),
        &MockProvider {
            outcome: MockProviderOutcome::Succeeded,
        },
        provider_call_orchestration_input(&attempted),
    )
    .expect("provider success transitions record");

    assert_eq!(
        result.provider_response().outcome(),
        GitHubPullRequestCommentWriteOutcome::ProviderSucceeded
    );
    assert_eq!(
        result.outcome_transition().record().lifecycle_state(),
        SideEffectLifecycleState::Completed
    );
    assert_eq!(
        result
            .outcome_transition()
            .record()
            .outcome_reference()
            .expect("outcome reference")
            .reference(),
        "github/comment/123"
    );
    assert!(!result.workflow_event_appended());
    assert!(!result.report_artifact_written());
}

#[test]
fn provider_call_orchestration_fails_attempted_record_from_classified_provider_failure() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let result = orchestrate_github_pr_comment_provider_call(
        state.backend(),
        &MockProvider {
            outcome: MockProviderOutcome::Failed,
        },
        provider_call_orchestration_input(&attempted),
    )
    .expect("provider failure transitions record");

    assert_eq!(
        result.provider_response().outcome(),
        GitHubPullRequestCommentWriteOutcome::ProviderFailed
    );
    assert_eq!(
        result.outcome_transition().record().lifecycle_state(),
        SideEffectLifecycleState::Failed
    );
    assert_eq!(
        result.outcome_transition().record().reason_codes(),
        &["github.rate_limited".to_owned()]
    );
    assert!(!result.workflow_event_appended());
    assert!(!result.report_artifact_written());
}

#[test]
fn provider_call_orchestration_rejects_unclassified_provider_error_without_transition() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let error = orchestrate_github_pr_comment_provider_call(
        state.backend(),
        &MockProvider {
            outcome: MockProviderOutcome::Unclassified,
        },
        provider_call_orchestration_input(&attempted),
    )
    .expect_err("unclassified provider error rejected");

    assert_eq!(error.code(), "github_pr_comment_provider.call_unclassified");
    assert_eq!(
        state
            .backend()
            .read_side_effect_record(attempted.side_effect_id())
            .expect("store read")
            .expect("record exists")
            .lifecycle_state(),
        SideEffectLifecycleState::Attempted
    );
}

#[test]
fn provider_call_orchestration_rejects_non_provider_response_without_transition() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let error = orchestrate_github_pr_comment_provider_call(
        state.backend(),
        &MockProvider {
            outcome: MockProviderOutcome::Fixture,
        },
        provider_call_orchestration_input(&attempted),
    )
    .expect_err("fixture provider response rejected");

    assert_eq!(
        error.code(),
        "github_pr_comment_provider.response.outcome_unsupported"
    );
    assert_eq!(
        state
            .backend()
            .read_side_effect_record(attempted.side_effect_id())
            .expect("store read")
            .expect("record exists")
            .lifecycle_state(),
        SideEffectLifecycleState::Attempted
    );
}

#[test]
fn provider_call_orchestration_does_not_invoke_provider_when_pre_call_gate_fails() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let mut input = provider_call_orchestration_input(&attempted);
    input.provider_call.live_call_enabled = false;
    let calls = AtomicU64::new(0);

    let error = orchestrate_github_pr_comment_provider_call(
        state.backend(),
        &CountingProvider { calls: &calls },
        input,
    )
    .expect_err("pre-call gate rejected");

    assert_eq!(
        error.code(),
        "github_pr_comment_provider.live_call_disabled"
    );
    assert_eq!(calls.load(Ordering::Relaxed), 0);
    assert_eq!(
        state
            .backend()
            .read_side_effect_record(attempted.side_effect_id())
            .expect("store read")
            .expect("record exists")
            .lifecycle_state(),
        SideEffectLifecycleState::Attempted
    );
}

#[test]
fn provider_call_orchestration_debug_redacts_sensitive_values() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let result = orchestrate_github_pr_comment_provider_call(
        state.backend(),
        &MockProvider {
            outcome: MockProviderOutcome::Succeeded,
        },
        provider_call_orchestration_input(&attempted),
    )
    .expect("provider success transitions record");

    let debug = format!("{result:?}");

    assert!(debug.contains("GitHubPullRequestCommentProviderCallOrchestrationResult"));
    assert!(!debug.contains("ghp_test_auth_value_for_injected_provider"));
    assert!(!debug.contains("Workflow OS governed live sandbox comment."));
    assert!(!debug.contains("github/comment/123"));
}

#[test]
fn provider_call_orchestration_store_record_mismatch_fails_without_provider_invocation() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let mut input = provider_call_orchestration_input(&attempted);
    input.provider_call.idempotency_key =
        IdempotencyKey::new("github-pr-comment-other").expect("valid idempotency key");
    let calls = AtomicU64::new(0);

    let error = orchestrate_github_pr_comment_provider_call(
        state.backend(),
        &CountingProvider { calls: &calls },
        input,
    )
    .expect_err("request idempotency mismatch rejected");

    assert_eq!(
        error.code(),
        "github_pr_comment_provider.idempotency.mismatch"
    );
    assert_eq!(calls.load(Ordering::Relaxed), 0);
}

fn provider_success_response() -> GitHubPullRequestCommentWriteResponse {
    GitHubPullRequestCommentWriteResponse::new(GitHubPullRequestCommentWriteResponseDefinition {
        correlation_id: CorrelationId::new("correlation/provider-success")
            .expect("valid correlation"),
        mode: GitHubPullRequestCommentWriteMode::LiveSandbox,
        outcome: GitHubPullRequestCommentWriteOutcome::ProviderSucceeded,
        provider_comment_reference: Some("github/comment/123".to_owned()),
        provider_error_code: None,
        summary: "provider returned a bounded comment reference".to_owned(),
        sensitivity: SideEffectSensitivity::Internal,
        redaction: redaction(),
    })
    .expect("valid provider success response")
}

fn provider_failure_response() -> GitHubPullRequestCommentWriteResponse {
    GitHubPullRequestCommentWriteResponse::new(GitHubPullRequestCommentWriteResponseDefinition {
        correlation_id: CorrelationId::new("correlation/provider-failure")
            .expect("valid correlation"),
        mode: GitHubPullRequestCommentWriteMode::LiveSandbox,
        outcome: GitHubPullRequestCommentWriteOutcome::ProviderFailed,
        provider_comment_reference: None,
        provider_error_code: Some("github.rate_limited".to_owned()),
        summary: "provider returned a bounded failure classification".to_owned(),
        sensitivity: SideEffectSensitivity::Internal,
        redaction: redaction(),
    })
    .expect("valid provider failure response")
}

fn reconciliation_input<'a>(
    attempted_record: &'a SideEffectRecord,
    provider_response: Option<&'a GitHubPullRequestCommentWriteResponse>,
    local_transition: Option<&'a workflow_core::SideEffectLifecycleTransitionResult>,
) -> GitHubPullRequestCommentProviderWriteReconciliationInput<'a> {
    GitHubPullRequestCommentProviderWriteReconciliationInput {
        attempted_record,
        provider_response,
        local_transition,
        provider_call_attempted: provider_response.is_some(),
        local_transition_error_code: None,
        ambiguity_error_code: None,
        sensitivity: SideEffectSensitivity::Internal,
        redaction: redaction(),
    }
}

#[test]
fn provider_write_reconciliation_classifies_normal_success() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let result = orchestrate_github_pr_comment_provider_call(
        state.backend(),
        &MockProvider {
            outcome: MockProviderOutcome::Succeeded,
        },
        provider_call_orchestration_input(&attempted),
    )
    .expect("provider success transitions record");
    let candidate = reconcile_github_pr_comment_provider_write(reconciliation_input(
        &attempted,
        Some(result.provider_response()),
        Some(result.outcome_transition()),
    ))
    .expect("normal success reconciles");

    assert_eq!(
        candidate.status(),
        GitHubPullRequestCommentProviderWriteReconciliationStatus::ProviderSucceededLocalCompleted
    );
    assert_eq!(candidate.provider_reference(), Some("github/comment/123"));
    assert!(!candidate.retry_blocked());
    assert!(!candidate.operator_action_required());
    assert!(!candidate.provider_call_performed());
    assert!(!candidate.workflow_event_appended());
    assert!(!candidate.report_artifact_written());
}

#[test]
fn provider_write_reconciliation_classifies_normal_failure() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let result = orchestrate_github_pr_comment_provider_call(
        state.backend(),
        &MockProvider {
            outcome: MockProviderOutcome::Failed,
        },
        provider_call_orchestration_input(&attempted),
    )
    .expect("provider failure transitions record");
    let candidate = reconcile_github_pr_comment_provider_write(reconciliation_input(
        &attempted,
        Some(result.provider_response()),
        Some(result.outcome_transition()),
    ))
    .expect("normal failure reconciles");

    assert_eq!(
        candidate.status(),
        GitHubPullRequestCommentProviderWriteReconciliationStatus::ProviderFailedLocalFailed
    );
    assert_eq!(candidate.provider_error_code(), Some("github.rate_limited"));
    assert!(!candidate.retry_blocked());
    assert!(!candidate.operator_action_required());
}

#[test]
fn provider_write_reconciliation_blocks_retry_after_remote_success_local_failure() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let response = provider_success_response();
    let mut input = reconciliation_input(&attempted, Some(&response), None);
    input.local_transition_error_code = Some("side_effect.transition.write_failed".to_owned());
    let candidate = reconcile_github_pr_comment_provider_write(input)
        .expect("remote/local mismatch classified");

    assert_eq!(
        candidate.status(),
        GitHubPullRequestCommentProviderWriteReconciliationStatus::ProviderSucceededLocalTransitionFailed
    );
    assert_eq!(candidate.provider_reference(), Some("github/comment/123"));
    assert!(candidate.retry_blocked());
    assert!(candidate.operator_action_required());
}

#[test]
fn provider_write_reconciliation_blocks_retry_after_remote_failure_local_failure() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let response = provider_failure_response();
    let mut input = reconciliation_input(&attempted, Some(&response), None);
    input.local_transition_error_code = Some("side_effect.transition.write_failed".to_owned());
    let candidate = reconcile_github_pr_comment_provider_write(input)
        .expect("remote/local mismatch classified");

    assert_eq!(
        candidate.status(),
        GitHubPullRequestCommentProviderWriteReconciliationStatus::ProviderFailedLocalTransitionFailed
    );
    assert_eq!(candidate.provider_error_code(), Some("github.rate_limited"));
    assert!(candidate.retry_blocked());
    assert!(candidate.operator_action_required());
}

#[test]
fn provider_write_reconciliation_treats_transport_ambiguity_as_ambiguous_not_not_called() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let mut input = reconciliation_input(&attempted, None, None);
    input.provider_call_attempted = true;
    input.ambiguity_error_code = Some("github.transport_unclassified".to_owned());
    let candidate =
        reconcile_github_pr_comment_provider_write(input).expect("ambiguity classified");

    assert_eq!(
        candidate.status(),
        GitHubPullRequestCommentProviderWriteReconciliationStatus::ProviderResponseAmbiguous
    );
    assert_ne!(
        candidate.status(),
        GitHubPullRequestCommentProviderWriteReconciliationStatus::ProviderNotCalled
    );
    assert_eq!(
        candidate.provider_error_code(),
        Some("github.transport_unclassified")
    );
    assert!(candidate.retry_blocked());
    assert!(candidate.operator_action_required());
}

#[test]
fn provider_write_reconciliation_represents_provider_not_called() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let candidate =
        reconcile_github_pr_comment_provider_write(reconciliation_input(&attempted, None, None))
            .expect("provider-not-called represented");

    assert_eq!(
        candidate.status(),
        GitHubPullRequestCommentProviderWriteReconciliationStatus::ProviderNotCalled
    );
    assert!(!candidate.retry_blocked());
    assert!(!candidate.operator_action_required());
}

#[test]
fn provider_write_reconciliation_rejects_secret_like_local_error_without_leakage() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let response = provider_success_response();
    let mut input = reconciliation_input(&attempted, Some(&response), None);
    input.local_transition_error_code = Some("token-leaked".to_owned());

    let error = reconcile_github_pr_comment_provider_write(input)
        .expect_err("secret-like transition code rejected");

    assert_eq!(
        error.code(),
        "github_pr_comment_reconciliation.local_transition_error.invalid"
    );
    assert!(!error.to_string().contains("token-leaked"));
}

#[test]
fn provider_write_reconciliation_rejects_secret_like_redaction_without_leakage() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let mut input = reconciliation_input(&attempted, None, None);
    input.redaction = RedactionMetadata {
        redacted_fields: vec!["secret_token_field".to_owned()],
        field_states: vec![],
    };

    let error = reconcile_github_pr_comment_provider_write(input).expect_err("redaction rejected");

    assert_eq!(error.code(), "github_pr_comment_write.secret_like_value");
    assert!(!error.to_string().contains("secret_token_field"));
}

#[test]
fn provider_write_reconciliation_debug_and_serialization_do_not_leak_forbidden_values() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let response = provider_success_response();
    let mut input = reconciliation_input(&attempted, Some(&response), None);
    input.local_transition_error_code = Some("side_effect.transition.write_failed".to_owned());
    let candidate = reconcile_github_pr_comment_provider_write(input)
        .expect("remote/local mismatch classified");

    let debug = format!("{candidate:?}");
    let serialized = serde_json::to_string(&candidate).expect("candidate serializes");

    assert!(debug.contains("GitHubPullRequestCommentProviderWriteReconciliationCandidate"));
    assert!(!debug.contains("github/comment/123"));
    assert!(!debug.contains("github-pr-comment-42"));
    assert!(!serialized.contains("Workflow OS governed live sandbox comment."));
    assert!(!serialized.contains("ghp_test_auth_value_for_injected_provider"));
}

#[test]
fn provider_write_reconciliation_serde_round_trip_and_invalid_wire_fails_closed() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let response = provider_success_response();
    let mut input = reconciliation_input(&attempted, Some(&response), None);
    input.local_transition_error_code = Some("side_effect.transition.write_failed".to_owned());
    let candidate = reconcile_github_pr_comment_provider_write(input)
        .expect("remote/local mismatch classified");
    let serialized = serde_json::to_string(&candidate).expect("candidate serializes");
    let round_trip: GitHubPullRequestCommentProviderWriteReconciliationCandidate =
        serde_json::from_str(&serialized).expect("candidate deserializes");

    assert_eq!(round_trip.status(), candidate.status());

    let invalid = json!({
        "side_effect_id": "side-effect/github-pr-comment",
        "idempotency_key": "github-pr-comment-42",
        "target_kind": "adapter_resource",
        "provider_kind": "github_pr_comment",
        "local_lifecycle_state": "attempted",
        "status": "provider_response_ambiguous",
        "provider_reference": "github/pr-comment/token-leak",
        "provider_error_code": null,
        "retry_blocked": true,
        "operator_action_required": true,
        "sensitivity": "internal",
        "redaction": redaction()
    });
    let invalid_text = serde_json::to_string(&invalid).expect("invalid wire serializes");
    let error =
        serde_json::from_str::<GitHubPullRequestCommentProviderWriteReconciliationCandidate>(
            &invalid_text,
        )
        .expect_err("invalid provider reference fails closed");

    assert!(!error.to_string().contains("token-leak"));
}

#[test]
fn provider_write_reconciliation_candidate_constructor_validates_provider_kind() {
    let error = GitHubPullRequestCommentProviderWriteReconciliationCandidate::new(
        side_effect_id(),
        idempotency_key(),
        SideEffectTargetKind::AdapterResource,
        "token-provider",
        SideEffectLifecycleState::Attempted,
        GitHubPullRequestCommentProviderWriteReconciliationStatus::ReconciliationRequired,
        None,
        None,
        true,
        true,
        SideEffectSensitivity::Internal,
        redaction(),
    )
    .expect_err("secret-like provider kind rejected");

    assert_eq!(error.code(), "github_pr_comment_write.secret_like_value");
    assert!(!error.to_string().contains("token-provider"));
}

fn lookup_observation(
    provider_reference: Option<&str>,
    marker: Option<&str>,
) -> GitHubPullRequestCommentProviderLookupObservation {
    GitHubPullRequestCommentProviderLookupObservation::new(
        GitHubPullRequestCommentProviderLookupObservationDefinition {
            target: target(),
            provider_comment_reference: provider_reference.map(ToOwned::to_owned),
            managed_marker: marker.map(ToOwned::to_owned),
        },
    )
    .expect("valid lookup observation")
}

fn lookup_response(
    outcome: GitHubPullRequestCommentProviderLookupOutcome,
    observations: Vec<GitHubPullRequestCommentProviderLookupObservation>,
    provider_error_code: Option<&str>,
) -> GitHubPullRequestCommentProviderLookupResponse {
    GitHubPullRequestCommentProviderLookupResponse::new(
        GitHubPullRequestCommentProviderLookupResponseDefinition {
            outcome,
            observations,
            provider_error_code: provider_error_code.map(ToOwned::to_owned),
            sensitivity: SideEffectSensitivity::Internal,
            redaction: redaction(),
        },
    )
    .expect("valid lookup response")
}

fn lookup_input(
    attempted_record: &SideEffectRecord,
) -> GitHubPullRequestCommentProviderLookupReconciliationInput<'_> {
    GitHubPullRequestCommentProviderLookupReconciliationInput {
        attempted_record,
        target: target(),
        expected_provider_reference: Some("github/comment/123".to_owned()),
        expected_managed_marker: Some("wfos/github-pr-comment/side-effect-123".to_owned()),
        auth: provider_auth(),
        sensitivity: SideEffectSensitivity::Internal,
        redaction: redaction(),
    }
}

struct MockLookupClient {
    response: Result<GitHubPullRequestCommentProviderLookupResponse, WorkflowOsError>,
    calls: AtomicU64,
    last_request_debug: RefCell<Option<String>>,
}

impl MockLookupClient {
    fn new(
        response: Result<GitHubPullRequestCommentProviderLookupResponse, WorkflowOsError>,
    ) -> Self {
        Self {
            response,
            calls: AtomicU64::new(0),
            last_request_debug: RefCell::new(None),
        }
    }

    fn calls(&self) -> u64 {
        self.calls.load(Ordering::Relaxed)
    }
}

impl GitHubPullRequestCommentProviderLookupClient for MockLookupClient {
    fn lookup_pull_request_comment(
        &self,
        request: &GitHubPullRequestCommentProviderLookupRequest,
    ) -> Result<GitHubPullRequestCommentProviderLookupResponse, WorkflowOsError> {
        self.calls.fetch_add(1, Ordering::Relaxed);
        *self.last_request_debug.borrow_mut() = Some(format!("{request:?}"));
        self.response.clone()
    }
}

fn lookup_recovery_input(
    attempted_record: &SideEffectRecord,
) -> GitHubPullRequestCommentProviderLookupRecoveryIntegrationInput<'_> {
    GitHubPullRequestCommentProviderLookupRecoveryIntegrationInput {
        lookup: lookup_input(attempted_record),
        recovery: GitHubPullRequestCommentProviderEventProofRecoveryInput {
            disclosure: None,
            event_proof_mismatch: false,
            sensitivity: WorkReportSensitivity::Confidential,
            redaction: RedactionMetadata::empty(),
        },
    }
}

#[test]
fn provider_lookup_reconciliation_observes_remote_comment_by_exact_reference() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let client = MockLookupClient::new(Ok(lookup_response(
        GitHubPullRequestCommentProviderLookupOutcome::Completed,
        vec![lookup_observation(Some("github/comment/123"), None)],
        None,
    )));

    let result = reconcile_github_pr_comment_provider_lookup(&client, lookup_input(&attempted))
        .expect("lookup reconciles observed remote comment");

    assert_eq!(client.calls(), 1);
    assert_eq!(
        result.posture(),
        GitHubPullRequestCommentProviderLookupReconciliationPosture::RemoteCommentObserved
    );
    assert_eq!(
        result.observed_provider_reference(),
        Some("github/comment/123")
    );
    assert_eq!(result.observed_match_count(), 1);
    assert!(result.retry_blocked());
    assert!(result.manual_state_repair_may_be_planned());
    assert!(result.artifact_write_blocked());
    assert!(!result.artifact_write_may_proceed());
    assert!(result.operator_action_required());
    assert_eq!(
        result.next_action(),
        GitHubPullRequestCommentProviderLookupReconciliationNextAction::PlanManualStateRepair
    );
    assert!(!result.workflow_event_appended());
    assert!(!result.side_effect_record_mutated());
    assert!(!result.report_artifact_written());
    assert!(!result.cli_output_emitted());
}

#[test]
fn provider_lookup_reconciliation_observes_remote_comment_by_bounded_marker() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let client = MockLookupClient::new(Ok(lookup_response(
        GitHubPullRequestCommentProviderLookupOutcome::Completed,
        vec![lookup_observation(
            Some("github/comment/456"),
            Some("wfos/github-pr-comment/side-effect-123"),
        )],
        None,
    )));
    let mut input = lookup_input(&attempted);
    input.expected_provider_reference = None;

    let result = reconcile_github_pr_comment_provider_lookup(&client, input)
        .expect("marker-based lookup reconciles observed remote comment");

    assert_eq!(
        result.posture(),
        GitHubPullRequestCommentProviderLookupReconciliationPosture::RemoteCommentObserved
    );
    assert_eq!(
        result.observed_provider_reference(),
        Some("github/comment/456")
    );
    assert!(result.artifact_write_blocked());
    assert!(!result.artifact_write_may_proceed());
}

#[test]
fn provider_lookup_reconciliation_reports_absent_remote_comment() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let client = MockLookupClient::new(Ok(lookup_response(
        GitHubPullRequestCommentProviderLookupOutcome::Completed,
        vec![],
        None,
    )));

    let result = reconcile_github_pr_comment_provider_lookup(&client, lookup_input(&attempted))
        .expect("absence reconciles");

    assert_eq!(
        result.posture(),
        GitHubPullRequestCommentProviderLookupReconciliationPosture::RemoteCommentAbsent
    );
    assert_eq!(result.observed_provider_reference(), None);
    assert_eq!(result.observed_match_count(), 0);
    assert!(!result.retry_blocked());
    assert!(!result.operator_action_required());
    assert_eq!(
        result.next_action(),
        GitHubPullRequestCommentProviderLookupReconciliationNextAction::ReevaluateRetryEligibility
    );
    assert!(result.artifact_write_blocked());
}

#[test]
fn provider_lookup_reconciliation_reports_ambiguous_remote_comments() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let client = MockLookupClient::new(Ok(lookup_response(
        GitHubPullRequestCommentProviderLookupOutcome::Completed,
        vec![
            lookup_observation(
                Some("github/comment/456"),
                Some("wfos/github-pr-comment/side-effect-123"),
            ),
            lookup_observation(
                Some("github/comment/789"),
                Some("wfos/github-pr-comment/side-effect-123"),
            ),
        ],
        None,
    )));
    let mut input = lookup_input(&attempted);
    input.expected_provider_reference = None;

    let result = reconcile_github_pr_comment_provider_lookup(&client, input)
        .expect("ambiguity is represented");

    assert_eq!(
        result.posture(),
        GitHubPullRequestCommentProviderLookupReconciliationPosture::RemoteCommentAmbiguous
    );
    assert_eq!(result.observed_match_count(), 2);
    assert_eq!(result.observed_provider_reference(), None);
    assert!(result.retry_blocked());
    assert!(result.operator_action_required());
}

#[test]
fn provider_lookup_reconciliation_maps_denied_unavailable_rate_limited_and_untrusted() {
    let cases = [
        (
            GitHubPullRequestCommentProviderLookupOutcome::NotAuthorized,
            "github.auth_failed",
            GitHubPullRequestCommentProviderLookupReconciliationPosture::LookupNotAuthorized,
            GitHubPullRequestCommentProviderLookupReconciliationNextAction::ProvideAuthorizedLookup,
        ),
        (
            GitHubPullRequestCommentProviderLookupOutcome::Unavailable,
            "github.server_error",
            GitHubPullRequestCommentProviderLookupReconciliationPosture::LookupUnavailable,
            GitHubPullRequestCommentProviderLookupReconciliationNextAction::RetryLookupLater,
        ),
        (
            GitHubPullRequestCommentProviderLookupOutcome::RateLimited,
            "github.rate_limited",
            GitHubPullRequestCommentProviderLookupReconciliationPosture::LookupRateLimited,
            GitHubPullRequestCommentProviderLookupReconciliationNextAction::RetryLookupLater,
        ),
        (
            GitHubPullRequestCommentProviderLookupOutcome::ResponseUntrusted,
            "github.response_untrusted",
            GitHubPullRequestCommentProviderLookupReconciliationPosture::LookupResponseUntrusted,
            GitHubPullRequestCommentProviderLookupReconciliationNextAction::FixLookupInput,
        ),
    ];

    for (outcome, provider_error, expected_posture, expected_action) in cases {
        let state = test_state_backend();
        let attempted = persisted_attempted_record(state.backend());
        let client =
            MockLookupClient::new(Ok(lookup_response(outcome, vec![], Some(provider_error))));

        let result = reconcile_github_pr_comment_provider_lookup(&client, lookup_input(&attempted))
            .expect("lookup posture represented");

        assert_eq!(result.posture(), expected_posture);
        assert_eq!(result.provider_error_code(), Some(provider_error));
        assert_eq!(result.next_action(), expected_action);
        assert!(result.retry_blocked());
        assert!(result.operator_action_required());
        assert!(result.artifact_write_blocked());
    }
}

#[test]
fn provider_lookup_reconciliation_rejects_target_mismatch_before_provider_lookup() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let client = MockLookupClient::new(Ok(lookup_response(
        GitHubPullRequestCommentProviderLookupOutcome::Completed,
        vec![],
        None,
    )));
    let mut input = lookup_input(&attempted);
    input.target =
        GitHubPullRequestCommentTarget::new("workflow-os", "other", 42).expect("valid target");

    let error = reconcile_github_pr_comment_provider_lookup(&client, input)
        .expect_err("target mismatch rejected before lookup");

    assert_eq!(
        error.code(),
        "github_pr_comment_provider_lookup_reconciliation.target_mismatch"
    );
    assert_eq!(client.calls(), 0);
    assert!(!format!("{error:?}").contains("other"));
}

#[test]
fn provider_lookup_reconciliation_rejects_secret_like_inputs_without_leakage() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let client = MockLookupClient::new(Ok(lookup_response(
        GitHubPullRequestCommentProviderLookupOutcome::Completed,
        vec![],
        None,
    )));
    let mut input = lookup_input(&attempted);
    input.expected_managed_marker = Some("wfos/raw_provider_payload".to_owned());

    let error = reconcile_github_pr_comment_provider_lookup(&client, input)
        .expect_err("secret-like marker rejected");

    assert_eq!(error.code(), "github_pr_comment_write.secret_like_value");
    assert_eq!(client.calls(), 0);
    assert!(!error.to_string().contains("raw_provider_payload"));
}

#[test]
fn provider_lookup_reconciliation_maps_unclassified_client_error_without_leakage() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let client = MockLookupClient::new(Err(WorkflowOsError::validation(
        "provider.raw_provider_payload",
        "raw_provider_payload bearer secret",
    )));

    let error = reconcile_github_pr_comment_provider_lookup(&client, lookup_input(&attempted))
        .expect_err("unclassified client error mapped");

    assert_eq!(
        error.code(),
        "github_pr_comment_provider_lookup_reconciliation.lookup_unavailable"
    );
    assert!(!error.to_string().contains("raw_provider_payload"));
    assert!(!format!("{error:?}").contains("secret"));
}

#[test]
fn provider_lookup_reconciliation_debug_and_serialization_do_not_leak_values() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let client = MockLookupClient::new(Ok(lookup_response(
        GitHubPullRequestCommentProviderLookupOutcome::Completed,
        vec![lookup_observation(Some("github/comment/123"), None)],
        None,
    )));

    let result = reconcile_github_pr_comment_provider_lookup(&client, lookup_input(&attempted))
        .expect("lookup reconciles");
    let debug = format!("{result:?}");
    let serialized = serde_json::to_string(&result).expect("lookup result serializes");
    let request_debug = client
        .last_request_debug
        .borrow()
        .clone()
        .expect("request debug captured");

    assert!(debug.contains("GitHubPullRequestCommentProviderLookupReconciliationResult"));
    assert!(!debug.contains("github/comment/123"));
    assert!(!debug.contains("github-pr-comment-42"));
    assert!(!request_debug.contains("ghp_test_auth_value_for_injected_provider"));
    assert!(!request_debug.contains("side-effect/github-pr-comment"));
    assert!(!serialized.contains("raw_provider_payload"));
    assert!(!serialized.contains("Workflow OS governed live sandbox comment."));
}

#[test]
fn provider_lookup_reconciliation_serde_round_trip_and_invalid_wire_fails_closed() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let client = MockLookupClient::new(Ok(lookup_response(
        GitHubPullRequestCommentProviderLookupOutcome::Completed,
        vec![lookup_observation(Some("github/comment/123"), None)],
        None,
    )));
    let result = reconcile_github_pr_comment_provider_lookup(&client, lookup_input(&attempted))
        .expect("lookup reconciles");
    let serialized = serde_json::to_string(&result).expect("lookup result serializes");
    let round_trip: workflow_core::GitHubPullRequestCommentProviderLookupReconciliationResult =
        serde_json::from_str(&serialized).expect("lookup result deserializes");

    assert_eq!(round_trip.posture(), result.posture());
    assert!(round_trip.artifact_write_blocked());

    let invalid = json!({
        "side_effect_id": "side-effect/github-pr-comment",
        "idempotency_key": "github-pr-comment-42",
        "target_kind": "adapter_resource",
        "provider_kind": "github_pr_comment",
        "local_lifecycle_state": "attempted",
        "posture": "remote_comment_observed",
        "observed_provider_reference": "github/comment/api_token",
        "observed_match_count": 1,
        "provider_error_code": null,
        "retry_blocked": true,
        "manual_state_repair_may_be_planned": true,
        "artifact_write_blocked": true,
        "operator_action_required": true,
        "next_action": "plan_manual_state_repair",
        "sensitivity": "internal",
        "redaction": redaction()
    });
    let error = serde_json::from_value::<
        workflow_core::GitHubPullRequestCommentProviderLookupReconciliationResult,
    >(invalid)
    .expect_err("invalid wire fails closed");

    assert!(!error.to_string().contains("api_token"));
}

#[test]
fn provider_lookup_recovery_integration_composes_lookup_and_recovery_without_mutation() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let client = MockLookupClient::new(Ok(lookup_response(
        GitHubPullRequestCommentProviderLookupOutcome::Completed,
        vec![lookup_observation(Some("github/comment/123"), None)],
        None,
    )));

    let result = integrate_github_pr_comment_provider_lookup_recovery(
        &client,
        lookup_recovery_input(&attempted),
    )
    .expect("lookup recovery integration succeeds");

    assert_eq!(client.calls(), 1);
    assert_eq!(
        result.lookup_reconciliation().posture(),
        GitHubPullRequestCommentProviderLookupReconciliationPosture::RemoteCommentObserved
    );
    assert_eq!(
        result.recovery().posture(),
        GitHubPullRequestCommentProviderEventProofRecoveryPosture::ReconciliationUnavailable
    );
    assert_eq!(
        result.recovery().next_action(),
        GitHubPullRequestCommentProviderEventProofRecoveryNextAction::InspectReconciliationCandidate
    );
    assert!(result.retry_blocked());
    assert!(result.artifact_write_blocked());
    assert!(!result.artifact_write_may_proceed());
    assert!(result.operator_action_required());
    assert!(result.provider_lookup_performed());
    assert!(!result.provider_write_performed());
    assert!(!result.workflow_event_appended());
    assert!(!result.side_effect_record_mutated());
    assert!(!result.report_artifact_written());
    assert!(!result.cli_output_emitted());
}

#[test]
fn provider_lookup_recovery_integration_allows_retry_reevaluation_when_remote_absent() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let client = MockLookupClient::new(Ok(lookup_response(
        GitHubPullRequestCommentProviderLookupOutcome::Completed,
        vec![],
        None,
    )));

    let result = integrate_github_pr_comment_provider_lookup_recovery(
        &client,
        lookup_recovery_input(&attempted),
    )
    .expect("lookup recovery integration succeeds");

    assert_eq!(
        result.lookup_reconciliation().posture(),
        GitHubPullRequestCommentProviderLookupReconciliationPosture::RemoteCommentAbsent
    );
    assert_eq!(
        result.lookup_reconciliation().next_action(),
        GitHubPullRequestCommentProviderLookupReconciliationNextAction::ReevaluateRetryEligibility
    );
    assert!(result.retry_blocked());
    assert!(result.artifact_write_blocked());
    assert!(result.operator_action_required());
}

#[test]
fn provider_lookup_recovery_integration_maps_recovery_errors_without_leakage() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let client = MockLookupClient::new(Ok(lookup_response(
        GitHubPullRequestCommentProviderLookupOutcome::Completed,
        vec![],
        None,
    )));
    let mut input = lookup_recovery_input(&attempted);
    input.recovery.redaction = RedactionMetadata {
        redacted_fields: vec!["api_token".to_owned()],
        field_states: Vec::new(),
    };

    let error = integrate_github_pr_comment_provider_lookup_recovery(&client, input)
        .expect_err("secret-like recovery redaction rejected");

    assert_eq!(
        error.code(),
        "github_pr_comment_provider_lookup_recovery.recovery_invalid"
    );
    assert_eq!(client.calls(), 1);
    assert!(!error.to_string().contains("api_token"));
    assert!(!format!("{error:?}").contains("api_token"));
}

#[test]
fn provider_lookup_recovery_integration_debug_and_serialization_do_not_leak_raw_inputs() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let client = MockLookupClient::new(Ok(lookup_response(
        GitHubPullRequestCommentProviderLookupOutcome::Completed,
        vec![lookup_observation(Some("github/comment/123"), None)],
        None,
    )));

    let result = integrate_github_pr_comment_provider_lookup_recovery(
        &client,
        lookup_recovery_input(&attempted),
    )
    .expect("lookup recovery integration succeeds");
    let debug = format!("{result:?}");
    let serialized = serde_json::to_string(&result).expect("integration result serializes");

    assert!(debug.contains("GitHubPullRequestCommentProviderLookupRecoveryIntegrationResult"));
    assert!(!debug.contains("github/comment/123"));
    assert!(!debug.contains("github-pr-comment-42"));
    assert!(!debug.contains("ghp_test_auth_value_for_injected_provider"));
    assert!(!debug.contains("raw_provider_payload"));
    assert!(serialized.contains("lookup_reconciliation"));
    assert!(serialized.contains("recovery"));
    assert!(!serialized.contains("raw_provider_payload"));
    assert!(!serialized.contains("Workflow OS governed live sandbox comment."));
}

#[test]
fn provider_lookup_operator_recovery_summary_blocks_artifacts_for_observed_remote_without_event_proof(
) {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let client = MockLookupClient::new(Ok(lookup_response(
        GitHubPullRequestCommentProviderLookupOutcome::Completed,
        vec![lookup_observation(Some("github/comment/123"), None)],
        None,
    )));
    let integration = integrate_github_pr_comment_provider_lookup_recovery(
        &client,
        lookup_recovery_input(&attempted),
    )
    .expect("lookup recovery integration succeeds");

    let summary = summarize_github_pr_comment_provider_lookup_operator_recovery(&integration)
        .expect("operator recovery summary is represented");

    assert_eq!(
        summary.lookup_posture(),
        GitHubPullRequestCommentProviderLookupReconciliationPosture::RemoteCommentObserved
    );
    assert_eq!(
        summary.recovery_posture(),
        GitHubPullRequestCommentProviderEventProofRecoveryPosture::ReconciliationUnavailable
    );
    assert_eq!(summary.observed_match_count(), 1);
    assert!(summary.has_observed_provider_reference());
    assert!(!summary.has_provider_error_code());
    assert!(summary.retry_blocked());
    assert!(summary.artifact_write_blocked());
    assert!(summary.operator_action_required());
    assert_eq!(
        summary.next_actions(),
        &[
            GitHubPullRequestCommentProviderLookupOperatorRecoveryNextAction::PlanManualStateRepair,
            GitHubPullRequestCommentProviderLookupOperatorRecoveryNextAction::InspectReconciliationCandidate,
            GitHubPullRequestCommentProviderLookupOperatorRecoveryNextAction::BlockReportArtifactWrite,
        ]
    );
    assert!(!summary.provider_lookup_performed());
    assert!(!summary.provider_write_performed());
    assert!(!summary.workflow_event_appended());
    assert!(!summary.side_effect_record_mutated());
    assert!(!summary.report_artifact_written());
    assert!(!summary.cli_output_emitted());
}

#[test]
fn provider_lookup_operator_recovery_summary_keeps_event_proof_gate_for_absent_remote_comment() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let client = MockLookupClient::new(Ok(lookup_response(
        GitHubPullRequestCommentProviderLookupOutcome::Completed,
        vec![],
        None,
    )));
    let integration = integrate_github_pr_comment_provider_lookup_recovery(
        &client,
        lookup_recovery_input(&attempted),
    )
    .expect("lookup recovery integration succeeds");

    let summary = summarize_github_pr_comment_provider_lookup_operator_recovery(&integration)
        .expect("operator recovery summary is represented");

    assert_eq!(
        summary.lookup_posture(),
        GitHubPullRequestCommentProviderLookupReconciliationPosture::RemoteCommentAbsent
    );
    assert!(!summary.has_observed_provider_reference());
    assert!(summary.retry_blocked());
    assert!(summary.artifact_write_blocked());
    assert!(summary.operator_action_required());
    assert!(summary.next_actions().contains(
        &GitHubPullRequestCommentProviderLookupOperatorRecoveryNextAction::ReevaluateRetryEligibility
    ));
    assert!(summary.next_actions().contains(
        &GitHubPullRequestCommentProviderLookupOperatorRecoveryNextAction::BlockReportArtifactWrite
    ));
}

#[test]
fn provider_lookup_operator_recovery_summary_debug_and_serialization_do_not_leak_raw_inputs() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let client = MockLookupClient::new(Ok(lookup_response(
        GitHubPullRequestCommentProviderLookupOutcome::Completed,
        vec![lookup_observation(Some("github/comment/123"), None)],
        None,
    )));
    let integration = integrate_github_pr_comment_provider_lookup_recovery(
        &client,
        lookup_recovery_input(&attempted),
    )
    .expect("lookup recovery integration succeeds");
    let summary = summarize_github_pr_comment_provider_lookup_operator_recovery(&integration)
        .expect("operator recovery summary is represented");

    let debug = format!("{summary:?}");
    let serialized = serde_json::to_string(&summary).expect("summary serializes");

    assert!(debug.contains("GitHubPullRequestCommentProviderLookupOperatorRecoverySummary"));
    assert!(!debug.contains("github/comment/123"));
    assert!(!debug.contains("github-pr-comment-42"));
    assert!(!debug.contains("ghp_test_auth_value_for_injected_provider"));
    assert!(!debug.contains("raw_provider_payload"));
    assert!(!serialized.contains("github/comment/123"));
    assert!(!serialized.contains("github-pr-comment-42"));
    assert!(!serialized.contains("ghp_test_auth_value_for_injected_provider"));
    assert!(!serialized.contains("raw_provider_payload"));
    assert!(serialized.contains("remote_comment_observed"));
    assert!(serialized.contains("block_report_artifact_write"));
}

#[test]
fn provider_lookup_operator_recovery_summary_invalid_wire_fails_closed_without_leakage() {
    let invalid = json!({
        "lookup_posture": "remote_comment_observed",
        "recovery_posture": "reconciliation_unavailable",
        "observed_match_count": 99,
        "observed_provider_reference": "present",
        "provider_error_code": "absent",
        "retry_gate": "blocked",
        "artifact_write_gate": "blocked",
        "operator_action": "required",
        "next_actions": [
            "plan_manual_state_repair",
            "block_report_artifact_write"
        ],
        "sensitivity": "internal",
        "redaction": RedactionMetadata {
            redacted_fields: vec!["api_token".to_owned()],
            field_states: Vec::new(),
        }
    });

    let error = serde_json::from_value::<
        GitHubPullRequestCommentProviderLookupOperatorRecoverySummary,
    >(invalid)
    .expect_err("invalid wire fails closed");

    assert!(!error.to_string().contains("api_token"));
    assert!(!format!("{error:?}").contains("api_token"));
}

#[test]
fn lookup_http_client_success_maps_bounded_observations_and_request_shape() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let mut input = lookup_input(&attempted);
    input.expected_provider_reference = Some("github/comment/123456".to_owned());
    input.expected_managed_marker = Some("wfos/run-123".to_owned());
    let request =
        GitHubPullRequestCommentProviderLookupRequest::new(input).expect("valid lookup request");
    let probe = LookupHttpTransportProbe::new();
    let client = lookup_http_client(probe.transport(
        GitHubPullRequestCommentLookupHttpResponse::new(
            200,
            vec![lookup_observation(
                Some("github/comment/123456"),
                Some("wfos/run-123"),
            )],
        ),
    ));

    let response = client
        .lookup_pull_request_comment(&request)
        .expect("lookup HTTP response");

    assert_eq!(probe.calls(), 1);
    assert_eq!(
        response.outcome(),
        GitHubPullRequestCommentProviderLookupOutcome::Completed
    );
    assert_eq!(response.observations().len(), 1);
    assert_eq!(
        response.observations()[0].provider_comment_reference(),
        Some("github/comment/123456")
    );
    assert_eq!(
        response.observations()[0].managed_marker(),
        Some("wfos/run-123")
    );
    assert_eq!(probe.last_method.borrow().as_deref(), Some("GET"));
    assert_eq!(
        probe.last_url.borrow().as_deref(),
        Some("https://api.github.test/repos/workflow-os/kernel/issues/42/comments?per_page=100")
    );
    assert_eq!(
        probe.last_authorization_header.borrow().as_deref(),
        Some("Bearer ghp_test_auth_value_for_injected_provider")
    );
    assert_eq!(
        probe.last_expected_provider_reference.borrow().as_deref(),
        Some("github/comment/123456")
    );
    assert_eq!(
        probe.last_expected_managed_marker.borrow().as_deref(),
        Some("wfos/run-123")
    );
}

#[test]
fn lookup_http_client_reuses_reconciliation_helper_without_recreating_evidence() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let probe = LookupHttpTransportProbe::new();
    let client = lookup_http_client(probe.transport(
        GitHubPullRequestCommentLookupHttpResponse::new(
            200,
            vec![lookup_observation(Some("github/comment/123"), None)],
        ),
    ));

    let result = reconcile_github_pr_comment_provider_lookup(&client, lookup_input(&attempted))
        .expect("lookup reconciles through HTTP client");

    assert_eq!(
        result.posture(),
        GitHubPullRequestCommentProviderLookupReconciliationPosture::RemoteCommentObserved
    );
    assert_eq!(
        result.observed_provider_reference(),
        Some("github/comment/123")
    );
    assert_eq!(probe.calls(), 1);
    assert!(result.retry_blocked());
    assert!(result.artifact_write_blocked());
}

#[test]
fn lookup_http_client_classifies_github_statuses() {
    let cases = [
        (
            401,
            GitHubPullRequestCommentProviderLookupOutcome::NotAuthorized,
            "github.auth_failed",
        ),
        (
            403,
            GitHubPullRequestCommentProviderLookupOutcome::NotAuthorized,
            "github.forbidden",
        ),
        (
            404,
            GitHubPullRequestCommentProviderLookupOutcome::Unavailable,
            "github.not_found",
        ),
        (
            408,
            GitHubPullRequestCommentProviderLookupOutcome::Unavailable,
            "github.timeout",
        ),
        (
            429,
            GitHubPullRequestCommentProviderLookupOutcome::RateLimited,
            "github.rate_limited",
        ),
        (
            500,
            GitHubPullRequestCommentProviderLookupOutcome::Unavailable,
            "github.server_error",
        ),
        (
            422,
            GitHubPullRequestCommentProviderLookupOutcome::ResponseUntrusted,
            "github.response_untrusted",
        ),
    ];

    for (status, expected_outcome, expected_code) in cases {
        let state = test_state_backend();
        let attempted = persisted_attempted_record(state.backend());
        let request = GitHubPullRequestCommentProviderLookupRequest::new(lookup_input(&attempted))
            .expect("valid lookup request");
        let probe = LookupHttpTransportProbe::new();
        let client = lookup_http_client(probe.transport(
            GitHubPullRequestCommentLookupHttpResponse::new(status, vec![]),
        ));

        let response = client
            .lookup_pull_request_comment(&request)
            .expect("classified lookup response");

        assert_eq!(response.outcome(), expected_outcome);
        assert_eq!(response.provider_error_code(), Some(expected_code));
        assert!(response.observations().is_empty());
        assert_eq!(probe.calls(), 1);
    }
}

#[test]
fn lookup_http_client_transport_failure_returns_stable_non_leaking_error() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let request = GitHubPullRequestCommentProviderLookupRequest::new(lookup_input(&attempted))
        .expect("valid lookup request");
    let probe = LookupHttpTransportProbe::new();
    let client = lookup_http_client(probe.transport(Err(WorkflowOsError::validation(
        "transport.raw_provider_payload",
        "raw_provider_payload bearer super-secret",
    ))));

    let error = client
        .lookup_pull_request_comment(&request)
        .expect_err("transport error remains unclassified");

    assert_eq!(
        error.code(),
        "github_pr_comment_provider_lookup_http.transport_unclassified"
    );
    let debug = format!("{error:?}");
    assert!(!debug.contains("raw_provider_payload"));
    assert!(!debug.contains("super-secret"));
    assert_eq!(probe.calls(), 1);
}

#[test]
fn lookup_http_client_rejects_auth_mismatch_before_transport_call() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let request = GitHubPullRequestCommentProviderLookupRequest::new(lookup_input(&attempted))
        .expect("valid lookup request");
    let probe = LookupHttpTransportProbe::new();
    let client = GitHubPullRequestCommentLookupHttpClient::new(
        probe.transport(GitHubPullRequestCommentLookupHttpResponse::new(200, vec![])),
        "https://api.github.test",
        GitHubPullRequestCommentProviderAuth::new(
            "ghp_different_explicit_test_auth",
            Some("sandbox pull request comments only".to_owned()),
        )
        .expect("valid provider auth"),
        SideEffectSensitivity::Internal,
        redaction(),
    )
    .expect("valid lookup HTTP client");

    let error = client
        .lookup_pull_request_comment(&request)
        .expect_err("auth mismatch rejected");

    assert_eq!(
        error.code(),
        "github_pr_comment_provider_lookup_http.auth.mismatch"
    );
    assert_eq!(probe.calls(), 0);
    let debug = format!("{error:?}");
    assert!(!debug.contains("ghp_different"));
    assert!(!debug.contains("ghp_test_auth"));
}

#[test]
fn lookup_http_client_rejects_auth_scope_mismatch_before_transport_call() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let request = GitHubPullRequestCommentProviderLookupRequest::new(lookup_input(&attempted))
        .expect("valid lookup request");
    let probe = LookupHttpTransportProbe::new();
    let client = GitHubPullRequestCommentLookupHttpClient::new(
        probe.transport(GitHubPullRequestCommentLookupHttpResponse::new(200, vec![])),
        "https://api.github.test",
        GitHubPullRequestCommentProviderAuth::new(
            "ghp_test_auth_value_for_injected_provider",
            Some("different sandbox scope summary".to_owned()),
        )
        .expect("valid provider auth"),
        SideEffectSensitivity::Internal,
        redaction(),
    )
    .expect("valid lookup HTTP client");

    let error = client
        .lookup_pull_request_comment(&request)
        .expect_err("auth scope mismatch rejected");

    assert_eq!(
        error.code(),
        "github_pr_comment_provider_lookup_http.auth.mismatch"
    );
    assert_eq!(probe.calls(), 0);
    let debug = format!("{error:?}");
    assert!(!debug.contains("ghp_test_auth"));
    assert!(!debug.contains("different sandbox scope summary"));
    assert!(!debug.contains("sandbox pull request comments only"));
}

#[test]
fn lookup_http_client_debug_and_request_debug_do_not_leak_payloads() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let mut input = lookup_input(&attempted);
    input.expected_provider_reference = Some("github/comment/123456".to_owned());
    input.expected_managed_marker = Some("wfos/run-123".to_owned());
    let request =
        GitHubPullRequestCommentProviderLookupRequest::new(input).expect("valid lookup request");
    let probe = LookupHttpTransportProbe::new();
    let client = lookup_http_client(probe.transport(
        GitHubPullRequestCommentLookupHttpResponse::new(
            200,
            vec![lookup_observation(
                Some("github/comment/123456"),
                Some("wfos/run-123"),
            )],
        ),
    ));

    let client_debug = format!("{client:?}");
    assert!(client_debug.contains("GitHubPullRequestCommentLookupHttpClient"));
    assert!(!client_debug.contains("api.github.test"));
    assert!(!client_debug.contains("ghp_test_auth_value_for_injected_provider"));
    assert!(client_debug.contains("workflow_event_append_allowed: false"));
    assert!(client_debug.contains("report_artifact_write_allowed: false"));
    assert!(client_debug.contains("side_effect_record_write_allowed: false"));

    client
        .lookup_pull_request_comment(&request)
        .expect("lookup response");
    let lookup_request_debug = probe
        .last_request_debug
        .borrow()
        .clone()
        .expect("recorded request debug");
    assert!(lookup_request_debug.contains("GitHubPullRequestCommentLookupHttpRequest"));
    assert!(!lookup_request_debug.contains("api.github.test"));
    assert!(!lookup_request_debug.contains("ghp_test_auth_value_for_injected_provider"));
    assert!(!lookup_request_debug.contains("github/comment/123456"));
    assert!(!lookup_request_debug.contains("wfos/run-123"));
}

#[test]
fn lookup_http_client_does_not_write_state_events_artifacts_or_cli_output() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let request = GitHubPullRequestCommentProviderLookupRequest::new(lookup_input(&attempted))
        .expect("valid lookup request");
    let probe = LookupHttpTransportProbe::new();
    let client = lookup_http_client(
        probe.transport(GitHubPullRequestCommentLookupHttpResponse::new(200, vec![])),
    );

    let response = client
        .lookup_pull_request_comment(&request)
        .expect("lookup response");

    assert_eq!(
        state
            .backend()
            .read_side_effect_record(attempted.side_effect_id())
            .expect("store read")
            .expect("record exists")
            .lifecycle_state(),
        SideEffectLifecycleState::Attempted
    );
    assert_eq!(
        response.outcome(),
        GitHubPullRequestCommentProviderLookupOutcome::Completed
    );
    assert!(!client.workflow_event_append_allowed());
    assert!(!client.report_artifact_write_allowed());
    assert!(!client.side_effect_record_write_allowed());
}

#[test]
fn live_sandbox_validation_allows_injected_provider_after_target_proof_and_readiness() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let target_proof = live_sandbox_target_proof();
    let provider = ResponseCountingProvider::new(MockProviderOutcome::Succeeded);

    let result = validate_and_orchestrate_github_pr_comment_live_sandbox(
        state.backend(),
        &provider,
        GitHubPullRequestCommentLiveSandboxValidationInput {
            target_proof: &target_proof,
            readiness: live_sandbox_readiness_input(&target_proof),
            provider_call: provider_call_input(&attempted),
            transitioned_at: Timestamp::parse_rfc3339("2026-06-20T12:04:00Z")
                .expect("valid timestamp"),
            transition_references: vec![SideEffectReference::new(
                SideEffectReferenceKind::EvidenceReference,
                "evidence/live-sandbox-validation",
            )
            .expect("valid reference")],
            evidence_reference_count: 1,
        },
    )
    .expect("live sandbox validation succeeds");

    assert_eq!(provider.calls(), 1);
    assert_eq!(
        result.readiness().decision(),
        ProviderWriteSandboxReadinessDecision::AllowedForSandbox
    );
    assert_eq!(
        result.provider_call().provider_response().outcome(),
        GitHubPullRequestCommentWriteOutcome::ProviderSucceeded
    );
    assert_eq!(
        result
            .provider_call()
            .outcome_transition()
            .record()
            .lifecycle_state(),
        SideEffectLifecycleState::Completed
    );
    assert!(!result.workflow_event_appended());
    assert!(!result.report_artifact_written());
}

#[test]
fn live_sandbox_validation_transitions_failed_record_from_classified_provider_failure() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let target_proof = live_sandbox_target_proof();
    let provider = ResponseCountingProvider::new(MockProviderOutcome::Failed);

    let result = validate_and_orchestrate_github_pr_comment_live_sandbox(
        state.backend(),
        &provider,
        GitHubPullRequestCommentLiveSandboxValidationInput {
            target_proof: &target_proof,
            readiness: live_sandbox_readiness_input(&target_proof),
            provider_call: provider_call_input(&attempted),
            transitioned_at: Timestamp::parse_rfc3339("2026-06-20T12:04:00Z")
                .expect("valid timestamp"),
            transition_references: vec![SideEffectReference::new(
                SideEffectReferenceKind::EvidenceReference,
                "evidence/live-sandbox-validation-failure",
            )
            .expect("valid reference")],
            evidence_reference_count: 1,
        },
    )
    .expect("classified provider failure is represented as failed lifecycle state");

    assert_eq!(provider.calls(), 1);
    assert_eq!(
        result.provider_call().provider_response().outcome(),
        GitHubPullRequestCommentWriteOutcome::ProviderFailed
    );
    assert_eq!(
        result
            .provider_call()
            .outcome_transition()
            .record()
            .lifecycle_state(),
        SideEffectLifecycleState::Failed
    );
    assert_eq!(
        result
            .provider_call()
            .outcome_transition()
            .record()
            .reason_codes(),
        &["github.rate_limited".to_owned()]
    );
    assert!(!result.workflow_event_appended());
    assert!(!result.report_artifact_written());
}

#[test]
fn live_sandbox_validation_target_proof_failure_prevents_provider_invocation() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let target_proof = sandbox_target_proof();
    let provider = ResponseCountingProvider::new(MockProviderOutcome::Succeeded);

    let error = validate_and_orchestrate_github_pr_comment_live_sandbox(
        state.backend(),
        &provider,
        GitHubPullRequestCommentLiveSandboxValidationInput {
            target_proof: &target_proof,
            readiness: live_sandbox_readiness_input(&live_sandbox_target_proof()),
            provider_call: provider_call_input(&attempted),
            transitioned_at: Timestamp::parse_rfc3339("2026-06-20T12:04:00Z")
                .expect("valid timestamp"),
            transition_references: vec![],
            evidence_reference_count: 0,
        },
    )
    .expect_err("mismatched target proof rejected");

    assert_eq!(
        error.code(),
        "github_pr_comment_live_sandbox_validation.target.mismatch"
    );
    assert_eq!(provider.calls(), 0);
    assert!(!error.provider_call_attempted());
    let debug = format!("{error:?}");
    assert!(!debug.contains("sandbox-owner"));
    assert!(!debug.contains("sandbox-repo"));
    assert!(!debug.contains("Workflow OS governed live sandbox comment."));
}

#[test]
fn live_sandbox_validation_capability_mismatch_prevents_provider_invocation() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let target_proof = live_sandbox_target_proof();
    let provider = ResponseCountingProvider::new(MockProviderOutcome::Succeeded);
    let mut readiness = live_sandbox_readiness_input(&target_proof);
    readiness.capability = AdapterWriteCapability::GitHubMerge;

    let error = validate_and_orchestrate_github_pr_comment_live_sandbox(
        state.backend(),
        &provider,
        GitHubPullRequestCommentLiveSandboxValidationInput {
            target_proof: &target_proof,
            readiness,
            provider_call: provider_call_input(&attempted),
            transitioned_at: Timestamp::parse_rfc3339("2026-06-20T12:04:00Z")
                .expect("valid timestamp"),
            transition_references: vec![],
            evidence_reference_count: 0,
        },
    )
    .expect_err("capability mismatch rejected");

    assert_eq!(
        error.code(),
        "github_pr_comment_live_sandbox_validation.capability.mismatch"
    );
    assert_eq!(provider.calls(), 0);
    assert!(!error.provider_call_attempted());
}

#[test]
fn live_sandbox_validation_target_posture_mismatch_prevents_provider_invocation() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let target_proof = live_sandbox_target_proof();
    let provider = ResponseCountingProvider::new(MockProviderOutcome::Succeeded);
    let mut readiness = live_sandbox_readiness_input(&target_proof);
    readiness.target_posture = ProviderWriteSandboxTargetPosture::ProductionLike;

    let error = validate_and_orchestrate_github_pr_comment_live_sandbox(
        state.backend(),
        &provider,
        GitHubPullRequestCommentLiveSandboxValidationInput {
            target_proof: &target_proof,
            readiness,
            provider_call: provider_call_input(&attempted),
            transitioned_at: Timestamp::parse_rfc3339("2026-06-20T12:04:00Z")
                .expect("valid timestamp"),
            transition_references: vec![],
            evidence_reference_count: 0,
        },
    )
    .expect_err("target posture mismatch rejected");

    assert_eq!(
        error.code(),
        "github_pr_comment_live_sandbox_validation.target_posture.mismatch"
    );
    assert_eq!(provider.calls(), 0);
    assert!(!error.provider_call_attempted());
}

#[test]
fn live_sandbox_validation_denied_readiness_prevents_provider_invocation() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let target_proof = live_sandbox_target_proof();
    let provider = ResponseCountingProvider::new(MockProviderOutcome::Succeeded);
    let mut readiness = live_sandbox_readiness_input(&target_proof);
    readiness.approval_posture = ProviderWriteSandboxApprovalPosture::Missing;

    let error = validate_and_orchestrate_github_pr_comment_live_sandbox(
        state.backend(),
        &provider,
        GitHubPullRequestCommentLiveSandboxValidationInput {
            target_proof: &target_proof,
            readiness,
            provider_call: provider_call_input(&attempted),
            transitioned_at: Timestamp::parse_rfc3339("2026-06-20T12:04:00Z")
                .expect("valid timestamp"),
            transition_references: vec![],
            evidence_reference_count: 0,
        },
    )
    .expect_err("denied readiness rejected");

    assert_eq!(
        error.code(),
        "github_pr_comment_live_sandbox_validation.readiness_not_allowed"
    );
    assert_eq!(provider.calls(), 0);
    assert!(!error.provider_call_attempted());
}

#[test]
fn live_sandbox_validation_auth_posture_failure_prevents_provider_invocation() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let target_proof = live_sandbox_target_proof();
    let provider = ResponseCountingProvider::new(MockProviderOutcome::Succeeded);
    let mut readiness = live_sandbox_readiness_input(&target_proof);
    readiness.auth_posture = ProviderWriteSandboxAuthPosture::HiddenOrAmbient;

    let error = validate_and_orchestrate_github_pr_comment_live_sandbox(
        state.backend(),
        &provider,
        GitHubPullRequestCommentLiveSandboxValidationInput {
            target_proof: &target_proof,
            readiness,
            provider_call: provider_call_input(&attempted),
            transitioned_at: Timestamp::parse_rfc3339("2026-06-20T12:04:00Z")
                .expect("valid timestamp"),
            transition_references: vec![],
            evidence_reference_count: 0,
        },
    )
    .expect_err("hidden auth posture rejected");

    assert_eq!(
        error.code(),
        "github_pr_comment_live_sandbox_validation.auth_posture.not_explicit"
    );
    assert_eq!(provider.calls(), 0);
    assert!(!error.provider_call_attempted());
}

#[test]
fn live_sandbox_validation_debug_does_not_leak_payloads_or_secret_like_inputs() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let target_proof = live_sandbox_target_proof();
    let input = GitHubPullRequestCommentLiveSandboxValidationInput {
        target_proof: &target_proof,
        readiness: live_sandbox_readiness_input(&target_proof),
        provider_call: provider_call_input(&attempted),
        transitioned_at: Timestamp::parse_rfc3339("2026-06-20T12:04:00Z").expect("valid timestamp"),
        transition_references: vec![SideEffectReference::new(
            SideEffectReferenceKind::EvidenceReference,
            "evidence/live-sandbox-validation",
        )
        .expect("valid reference")],
        evidence_reference_count: 1,
    };

    let debug = format!("{input:?}");

    assert!(debug.contains("GitHubPullRequestCommentLiveSandboxValidationInput"));
    assert!(!debug.contains("Workflow OS governed live sandbox comment."));
    assert!(!debug.contains("ghp_test_auth"));
    assert!(!debug.contains("explicit maintainer sandbox"));
    assert!(!debug.contains("correlation/live-sandbox-validation"));
    assert!(!debug.contains("github-pr-comment-42"));
}

#[test]
fn provider_call_request_validates_attempted_record_and_explicit_opt_in() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());

    let request = GitHubPullRequestCommentProviderCallRequest::new(provider_call_input(&attempted))
        .expect("valid provider-call request");

    assert_eq!(request.side_effect_id(), attempted.side_effect_id());
    assert_eq!(request.target().reference(), target().reference());
    assert_eq!(request.idempotency_key(), &idempotency_key());
    assert_eq!(
        request.mode(),
        GitHubPullRequestCommentWriteMode::LiveSandbox
    );
    assert_eq!(request.summary(), "bounded provider-call request summary");
    assert!(request.provider_call_allowed());
    assert!(!request.workflow_event_append_allowed());
    assert!(!request.report_artifact_write_allowed());
}

#[test]
fn injected_provider_trait_returns_validated_response_without_builtin_network_client() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let request = GitHubPullRequestCommentProviderCallRequest::new(provider_call_input(&attempted))
        .expect("valid provider-call request");

    let response = MockProvider {
        outcome: MockProviderOutcome::Succeeded,
    }
    .create_pull_request_comment(&request)
    .expect("mock provider response");

    assert_eq!(
        response.outcome(),
        GitHubPullRequestCommentWriteOutcome::ProviderSucceeded
    );
    assert_eq!(
        response.provider_comment_reference(),
        Some("github/comment/123")
    );
    assert!(!response.workflow_event_append_allowed());
    assert!(!response.side_effect_lifecycle_transition_allowed());
}

#[test]
fn provider_call_request_rejects_missing_auth_before_provider_invocation() {
    let error = GitHubPullRequestCommentProviderAuth::new("", None)
        .expect_err("empty auth rejected before storage");

    assert_eq!(error.code(), "github_pr_comment_provider.auth.missing");
    assert!(!format!("{error:?}").contains("ghp_"));
}

#[test]
fn provider_call_auth_error_does_not_leak_secret_like_value() {
    let error = GitHubPullRequestCommentProviderAuth::new("x".repeat(9 * 1024), None)
        .expect_err("oversized auth rejected");
    let error_debug = format!("{error:?}");

    assert_eq!(error.code(), "github_pr_comment_provider.auth.too_long");
    assert!(!error_debug.contains("ghp_"));
}

#[test]
fn provider_call_request_rejects_disabled_live_call_before_provider_invocation() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let mut input = provider_call_input(&attempted);
    input.live_call_enabled = false;

    let error = GitHubPullRequestCommentProviderCallRequest::new(input)
        .expect_err("disabled live call rejected");

    assert_eq!(
        error.code(),
        "github_pr_comment_provider.live_call_disabled"
    );
}

#[test]
fn provider_call_request_rejects_disabled_provider_call_before_provider_invocation() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let mut input = provider_call_input(&attempted);
    input.provider_call_enabled = false;

    let error = GitHubPullRequestCommentProviderCallRequest::new(input)
        .expect_err("disabled provider call rejected");

    assert_eq!(
        error.code(),
        "github_pr_comment_provider.provider_call_disabled"
    );
}

#[test]
fn provider_call_request_rejects_non_live_mode() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let mut input = provider_call_input(&attempted);
    input.mode = GitHubPullRequestCommentWriteMode::DryRun;

    let error =
        GitHubPullRequestCommentProviderCallRequest::new(input).expect_err("dry-run rejected");

    assert_eq!(error.code(), "github_pr_comment_provider.mode.unsupported");
}

#[test]
fn provider_call_request_rejects_target_mismatch() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let mut input = provider_call_input(&attempted);
    input.target =
        GitHubPullRequestCommentTarget::new("workflow-os", "different-kernel", 42).expect("target");

    let error = GitHubPullRequestCommentProviderCallRequest::new(input)
        .expect_err("target mismatch rejected");

    assert_eq!(error.code(), "github_pr_comment_provider.target.mismatch");
}

#[test]
fn provider_call_request_rejects_idempotency_mismatch() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let mut input = provider_call_input(&attempted);
    input.idempotency_key =
        IdempotencyKey::new("github-pr-comment-other").expect("valid idempotency key");

    let error = GitHubPullRequestCommentProviderCallRequest::new(input)
        .expect_err("idempotency mismatch rejected");

    assert_eq!(
        error.code(),
        "github_pr_comment_provider.idempotency.mismatch"
    );
}

#[test]
fn provider_call_request_debug_redacts_auth_comment_and_ids() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let request = GitHubPullRequestCommentProviderCallRequest::new(provider_call_input(&attempted))
        .expect("valid provider-call request");

    let debug = format!("{request:?}");

    assert!(debug.contains("GitHubPullRequestCommentProviderCallRequest"));
    assert!(!debug.contains("ghp_test_auth_value_for_injected_provider"));
    assert!(!debug.contains("Workflow OS governed live sandbox comment."));
    assert!(!debug.contains("side-effect/github-pr-comment"));
    assert!(!debug.contains("github-pr-comment-42"));
}

#[test]
fn http_provider_success_maps_to_provider_succeeded_response() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let request = GitHubPullRequestCommentProviderCallRequest::new(provider_call_input(&attempted))
        .expect("valid provider-call request");
    let probe = HttpTransportProbe::new();
    let provider = http_provider(probe.transport(GitHubPullRequestCommentHttpResponse::new(
        201,
        Some("123456".to_owned()),
    )));

    let response = provider
        .create_pull_request_comment(&request)
        .expect("provider success response");

    assert_eq!(probe.calls(), 1);
    assert_eq!(
        response.outcome(),
        GitHubPullRequestCommentWriteOutcome::ProviderSucceeded
    );
    assert_eq!(
        response.provider_comment_reference(),
        Some("github/pr-comment/workflow-os/kernel/42/123456")
    );
    assert_eq!(response.provider_error_code(), None);
    assert_eq!(probe.last_method.borrow().as_deref(), Some("POST"));
    assert_eq!(
        probe.last_url.borrow().as_deref(),
        Some("https://api.github.test/repos/workflow-os/kernel/issues/42/comments")
    );
    assert_eq!(
        probe.last_authorization_header.borrow().as_deref(),
        Some("Bearer ghp_test_auth_value_for_injected_provider")
    );
    assert_eq!(
        probe.last_body.borrow().as_deref(),
        Some("{\"body\":\"Workflow OS governed live sandbox comment.\"}")
    );
}

#[test]
fn http_provider_classifies_github_status_failures() {
    let cases = [
        (401, "github.auth_failed"),
        (403, "github.forbidden"),
        (404, "github.not_found"),
        (408, "github.timeout"),
        (409, "github.conflict"),
        (422, "github.validation_failed"),
        (429, "github.rate_limited"),
        (500, "github.server_error"),
        (418, "github.transport_unclassified"),
    ];

    for (status, expected_code) in cases {
        let state = test_state_backend();
        let attempted = persisted_attempted_record(state.backend());
        let request =
            GitHubPullRequestCommentProviderCallRequest::new(provider_call_input(&attempted))
                .expect("valid provider-call request");
        let probe = HttpTransportProbe::new();
        let provider =
            http_provider(probe.transport(GitHubPullRequestCommentHttpResponse::new(status, None)));

        let response = provider
            .create_pull_request_comment(&request)
            .expect("classified provider failure");

        assert_eq!(
            response.outcome(),
            GitHubPullRequestCommentWriteOutcome::ProviderFailed
        );
        assert_eq!(response.provider_error_code(), Some(expected_code));
        assert_eq!(response.provider_comment_reference(), None);
        assert_eq!(probe.calls(), 1);
    }
}

#[test]
fn http_provider_transport_failure_returns_stable_non_leaking_error() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let request = GitHubPullRequestCommentProviderCallRequest::new(provider_call_input(&attempted))
        .expect("valid provider-call request");
    let probe = HttpTransportProbe::new();
    let provider = http_provider(probe.transport(Err(WorkflowOsError::validation(
        "transport.raw_provider_payload",
        "raw_provider_payload bearer super-secret",
    ))));

    let error = provider
        .create_pull_request_comment(&request)
        .expect_err("transport error remains unclassified");

    assert_eq!(
        error.code(),
        "github_pr_comment_provider_http.transport_unclassified"
    );
    let debug = format!("{error:?}");
    assert!(!debug.contains("raw_provider_payload"));
    assert!(!debug.contains("super-secret"));
    assert_eq!(probe.calls(), 1);
}

#[test]
fn http_provider_rejects_auth_mismatch_before_transport_call() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let request = GitHubPullRequestCommentProviderCallRequest::new(provider_call_input(&attempted))
        .expect("valid provider-call request");
    let probe = HttpTransportProbe::new();
    let provider = GitHubPullRequestCommentHttpProvider::new(
        probe.transport(GitHubPullRequestCommentHttpResponse::new(
            201,
            Some("123456".to_owned()),
        )),
        "https://api.github.test",
        GitHubPullRequestCommentProviderAuth::new(
            "ghp_different_explicit_test_auth",
            Some("sandbox pull request comments only".to_owned()),
        )
        .expect("valid provider auth"),
        SideEffectSensitivity::Internal,
        redaction(),
    )
    .expect("valid HTTP provider");

    let error = provider
        .create_pull_request_comment(&request)
        .expect_err("auth mismatch rejected");

    assert_eq!(
        error.code(),
        "github_pr_comment_provider_http.auth.mismatch"
    );
    assert_eq!(probe.calls(), 0);
    let debug = format!("{error:?}");
    assert!(!debug.contains("ghp_different"));
    assert!(!debug.contains("ghp_test_auth"));
}

#[test]
fn http_provider_rejects_auth_scope_mismatch_before_transport_call() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let request = GitHubPullRequestCommentProviderCallRequest::new(provider_call_input(&attempted))
        .expect("valid provider-call request");
    let probe = HttpTransportProbe::new();
    let provider = GitHubPullRequestCommentHttpProvider::new(
        probe.transport(GitHubPullRequestCommentHttpResponse::new(
            201,
            Some("123456".to_owned()),
        )),
        "https://api.github.test",
        GitHubPullRequestCommentProviderAuth::new(
            "ghp_test_auth_value_for_injected_provider",
            Some("different sandbox scope summary".to_owned()),
        )
        .expect("valid provider auth"),
        SideEffectSensitivity::Internal,
        redaction(),
    )
    .expect("valid HTTP provider");

    let error = provider
        .create_pull_request_comment(&request)
        .expect_err("auth scope mismatch rejected");

    assert_eq!(
        error.code(),
        "github_pr_comment_provider_http.auth.mismatch"
    );
    assert_eq!(probe.calls(), 0);
    let debug = format!("{error:?}");
    assert!(!debug.contains("ghp_test_auth"));
    assert!(!debug.contains("different sandbox scope summary"));
    assert!(!debug.contains("sandbox pull request comments only"));
}

#[test]
fn http_provider_rejects_success_without_comment_id() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let request = GitHubPullRequestCommentProviderCallRequest::new(provider_call_input(&attempted))
        .expect("valid provider-call request");
    let probe = HttpTransportProbe::new();
    let provider =
        http_provider(probe.transport(GitHubPullRequestCommentHttpResponse::new(201, None)));

    let error = provider
        .create_pull_request_comment(&request)
        .expect_err("success without comment ID rejected");

    assert_eq!(
        error.code(),
        "github_pr_comment_provider_http.comment_id.missing"
    );
    assert_eq!(probe.calls(), 1);
}

#[test]
fn http_provider_debug_and_request_debug_do_not_leak_payloads() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let request = GitHubPullRequestCommentProviderCallRequest::new(provider_call_input(&attempted))
        .expect("valid provider-call request");
    let probe = HttpTransportProbe::new();
    let provider = http_provider(probe.transport(GitHubPullRequestCommentHttpResponse::new(
        201,
        Some("123456".to_owned()),
    )));

    let provider_debug = format!("{provider:?}");
    assert!(provider_debug.contains("GitHubPullRequestCommentHttpProvider"));
    assert!(!provider_debug.contains("api.github.test"));
    assert!(!provider_debug.contains("ghp_test_auth_value_for_injected_provider"));
    assert!(provider_debug.contains("workflow_event_append_allowed: false"));
    assert!(provider_debug.contains("report_artifact_write_allowed: false"));
    assert!(provider_debug.contains("side_effect_record_write_allowed: false"));

    provider
        .create_pull_request_comment(&request)
        .expect("provider success response");
    let http_request_debug = probe
        .last_request_debug
        .borrow()
        .clone()
        .expect("recorded request debug");
    assert!(http_request_debug.contains("GitHubPullRequestCommentHttpRequest"));
    assert!(!http_request_debug.contains("api.github.test"));
    assert!(!http_request_debug.contains("Workflow OS governed live sandbox comment."));
    assert!(!http_request_debug.contains("ghp_test_auth_value_for_injected_provider"));
}

#[test]
fn http_provider_does_not_write_state_events_artifacts_or_cli_output() {
    let state = test_state_backend();
    let attempted = persisted_attempted_record(state.backend());
    let request = GitHubPullRequestCommentProviderCallRequest::new(provider_call_input(&attempted))
        .expect("valid provider-call request");
    let probe = HttpTransportProbe::new();
    let provider = http_provider(probe.transport(GitHubPullRequestCommentHttpResponse::new(
        201,
        Some("123456".to_owned()),
    )));

    let response = provider
        .create_pull_request_comment(&request)
        .expect("provider success response");

    assert_eq!(
        state
            .backend()
            .read_side_effect_record(attempted.side_effect_id())
            .expect("store read")
            .expect("record exists")
            .lifecycle_state(),
        SideEffectLifecycleState::Attempted
    );
    assert!(!response.workflow_event_append_allowed());
    assert!(!response.side_effect_lifecycle_transition_allowed());
    assert!(!provider.workflow_event_append_allowed());
    assert!(!provider.report_artifact_write_allowed());
    assert!(!provider.side_effect_record_write_allowed());
}
