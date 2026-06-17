#![allow(clippy::expect_used)]

//! `SideEffect` core model tests.

use serde_json::json;
use workflow_core::{
    ActorId, AdapterId, AdapterKind, CorrelationId, IdempotencyKey, IntegrationId,
    RedactionDisposition, RedactionFieldState, RedactionMetadata, SchemaVersion,
    SideEffectAuthority, SideEffectAuthorityDecision, SideEffectCapability, SideEffectId,
    SideEffectIdempotencyBinding, SideEffectIdempotencyScope, SideEffectLifecycleState,
    SideEffectOutcomeReference, SideEffectOutcomeReferenceKind, SideEffectRecord,
    SideEffectRecordDefinition, SideEffectReference, SideEffectReferenceKind,
    SideEffectSensitivity, SideEffectTargetKind, SideEffectTargetReference, SkillId, SkillVersion,
    SpecContentHash, StepId, Timestamp, WorkflowId, WorkflowRunId, WorkflowVersion,
};

fn side_effect_id() -> SideEffectId {
    SideEffectId::new("side-effect/1").expect("valid side-effect id")
}

fn target() -> SideEffectTargetReference {
    SideEffectTargetReference::new(
        SideEffectTargetKind::AdapterResource,
        "github/pull-request/42",
    )
    .expect("valid target")
}

fn actor() -> ActorId {
    ActorId::new("operator/reviewer").expect("valid actor")
}

fn system_actor() -> ActorId {
    ActorId::new("system/workflow-os").expect("valid system actor")
}

fn workflow_id() -> WorkflowId {
    WorkflowId::new("workflow/self-governed-build").expect("valid workflow id")
}

fn workflow_version() -> WorkflowVersion {
    WorkflowVersion::new("v1").expect("valid workflow version")
}

fn schema_version() -> SchemaVersion {
    SchemaVersion::new("workflowos/v0").expect("valid schema version")
}

fn run_id() -> WorkflowRunId {
    WorkflowRunId::new("run-123").expect("valid run id")
}

fn created_at() -> Timestamp {
    Timestamp::parse_rfc3339("2026-06-17T12:00:00Z").expect("valid timestamp")
}

fn redaction() -> RedactionMetadata {
    RedactionMetadata::empty()
}

fn redaction_with(field: &str, reason: &str) -> RedactionMetadata {
    RedactionMetadata {
        redacted_fields: vec![field.to_owned()],
        field_states: vec![RedactionFieldState {
            field: field.to_owned(),
            disposition: RedactionDisposition::ReferenceOnly,
            reason: reason.to_owned(),
        }],
    }
}

fn authority(decision: SideEffectAuthorityDecision) -> SideEffectAuthority {
    SideEffectAuthority::new(
        decision,
        vec![SideEffectReference::new(
            SideEffectReferenceKind::PolicyDecision,
            "event/policy-allowed",
        )
        .expect("valid policy reference")],
        Vec::new(),
    )
    .expect("valid authority")
}

fn approval_authority() -> SideEffectAuthority {
    SideEffectAuthority::new(
        SideEffectAuthorityDecision::ApprovedByHuman,
        vec![SideEffectReference::new(
            SideEffectReferenceKind::PolicyDecision,
            "event/policy-requires-approval",
        )
        .expect("valid policy reference")],
        vec![SideEffectReference::new(
            SideEffectReferenceKind::ApprovalDecision,
            "approval/decision-1",
        )
        .expect("valid approval reference")],
    )
    .expect("valid approval authority")
}

fn idempotency() -> SideEffectIdempotencyBinding {
    SideEffectIdempotencyBinding::new(
        IdempotencyKey::new("side-effect-key-1").expect("valid idempotency key"),
        SideEffectIdempotencyScope::Run,
        None,
        None,
    )
    .expect("valid idempotency binding")
}

fn outcome(kind: SideEffectOutcomeReferenceKind) -> SideEffectOutcomeReference {
    SideEffectOutcomeReference::new(kind, "adapter/outcome/1").expect("valid outcome reference")
}

fn reference() -> SideEffectReference {
    SideEffectReference::new(SideEffectReferenceKind::EvidenceReference, "evidence/1")
        .expect("valid side-effect reference")
}

fn valid_definition(state: SideEffectLifecycleState) -> SideEffectRecordDefinition {
    let (authority, outcome_reference, reason_codes) = match state {
        SideEffectLifecycleState::Attempted => (
            authority(SideEffectAuthorityDecision::AllowedByPolicy),
            None,
            Vec::new(),
        ),
        SideEffectLifecycleState::Completed => (
            approval_authority(),
            Some(outcome(SideEffectOutcomeReferenceKind::Outcome)),
            Vec::new(),
        ),
        SideEffectLifecycleState::Denied => (
            authority(SideEffectAuthorityDecision::DeniedByPolicy),
            None,
            vec!["policy.denied".to_owned()],
        ),
        SideEffectLifecycleState::Skipped => (
            authority(SideEffectAuthorityDecision::NotEvaluated),
            None,
            vec!["operator.skipped".to_owned()],
        ),
        SideEffectLifecycleState::Failed => (
            authority(SideEffectAuthorityDecision::AllowedByPolicy),
            Some(outcome(SideEffectOutcomeReferenceKind::Failure)),
            vec!["adapter.failed".to_owned()],
        ),
        SideEffectLifecycleState::Proposed => (
            authority(SideEffectAuthorityDecision::NotEvaluated),
            None,
            Vec::new(),
        ),
    };

    SideEffectRecordDefinition {
        side_effect_id: side_effect_id(),
        lifecycle_state: state,
        target: target(),
        capability: SideEffectCapability::GitHubWrite,
        authority,
        actor: Some(actor()),
        system_actor: None,
        workflow_id: workflow_id(),
        workflow_version: workflow_version(),
        schema_version: schema_version(),
        spec_hash: SpecContentHash::from_text("workflow spec"),
        run_id: run_id(),
        step_id: Some(StepId::new("step/review").expect("valid step id")),
        skill_id: Some(SkillId::new("skill/review").expect("valid skill id")),
        skill_version: Some(SkillVersion::new("v1").expect("valid skill version")),
        adapter_id: Some(AdapterId::new("adapter/github").expect("valid adapter id")),
        adapter_kind: Some(AdapterKind::GitHub),
        integration_id: Some(IntegrationId::new("integration/github").expect("valid integration")),
        idempotency: idempotency(),
        references: vec![reference()],
        outcome_reference,
        created_at: created_at(),
        updated_at: Some(created_at()),
        correlation_id: Some(CorrelationId::new("correlation-1").expect("valid correlation")),
        summary: Some("bounded side-effect record summary".to_owned()),
        reason_codes,
        sensitivity: SideEffectSensitivity::Confidential,
        redaction: redaction(),
    }
}

fn valid_record(state: SideEffectLifecycleState) -> SideEffectRecord {
    SideEffectRecord::new(valid_definition(state)).expect("valid side-effect record")
}

#[test]
fn valid_minimal_proposed_side_effect_record() {
    let record = valid_record(SideEffectLifecycleState::Proposed);

    assert_eq!(record.lifecycle_state(), SideEffectLifecycleState::Proposed);
    assert_eq!(record.capability(), SideEffectCapability::GitHubWrite);
    assert_eq!(
        record.target().kind(),
        SideEffectTargetKind::AdapterResource
    );
    assert_eq!(record.references().len(), 1);
}

#[test]
fn all_required_lifecycle_states_are_representable() {
    for state in [
        SideEffectLifecycleState::Proposed,
        SideEffectLifecycleState::Attempted,
        SideEffectLifecycleState::Completed,
        SideEffectLifecycleState::Denied,
        SideEffectLifecycleState::Skipped,
        SideEffectLifecycleState::Failed,
    ] {
        let record = valid_record(state);
        assert_eq!(record.lifecycle_state(), state);
    }
}

#[test]
fn completed_side_effect_requires_outcome_reference() {
    let mut definition = valid_definition(SideEffectLifecycleState::Completed);
    definition.outcome_reference = None;

    let error = SideEffectRecord::new(definition).expect_err("missing outcome rejected");

    assert_eq!(error.code(), "side_effect.outcome.required");
}

#[test]
fn attempted_side_effect_rejects_denied_authority() {
    let mut definition = valid_definition(SideEffectLifecycleState::Attempted);
    definition.authority = authority(SideEffectAuthorityDecision::DeniedByPolicy);

    let error = SideEffectRecord::new(definition).expect_err("denied authority rejected");

    assert_eq!(error.code(), "side_effect.authority.not_allowed");
}

#[test]
fn denied_side_effect_requires_denied_or_unsupported_authority() {
    let mut definition = valid_definition(SideEffectLifecycleState::Denied);
    definition.authority = authority(SideEffectAuthorityDecision::AllowedByPolicy);

    let error = SideEffectRecord::new(definition).expect_err("allowed denied record rejected");

    assert_eq!(error.code(), "side_effect.authority.denied_required");
}

#[test]
fn denied_and_skipped_side_effects_require_stable_reason_codes() {
    for state in [
        SideEffectLifecycleState::Denied,
        SideEffectLifecycleState::Skipped,
    ] {
        let mut definition = valid_definition(state);
        definition.reason_codes.clear();

        let error = SideEffectRecord::new(definition).expect_err("missing reason rejected");

        assert_eq!(error.code(), "side_effect.reason.required");
    }
}

#[test]
fn unknown_capability_fails_closed_for_attempted_completed_and_failed() {
    for state in [
        SideEffectLifecycleState::Attempted,
        SideEffectLifecycleState::Completed,
        SideEffectLifecycleState::Failed,
    ] {
        let mut definition = valid_definition(state);
        definition.capability = SideEffectCapability::Unknown;

        let error =
            SideEffectRecord::new(definition).expect_err("unknown unsafe capability rejected");

        assert_eq!(error.code(), "side_effect.capability.unknown");
    }
}

#[test]
fn unknown_capability_can_record_denied_request_without_write_support() {
    let mut definition = valid_definition(SideEffectLifecycleState::Denied);
    definition.capability = SideEffectCapability::Unknown;
    definition.authority = authority(SideEffectAuthorityDecision::DeniedByCapability);
    definition.reason_codes = vec!["capability.unknown".to_owned()];

    let record = SideEffectRecord::new(definition).expect("denied unknown capability recorded");

    assert_eq!(record.capability(), SideEffectCapability::Unknown);
}

#[test]
fn invalid_side_effect_id_is_rejected_without_leaking_value() {
    let error = SideEffectId::new("bad id with spaces").expect_err("invalid id rejected");

    assert_eq!(error.code(), "side_effect.identifier.invalid_character");
    assert!(!error.to_string().contains("bad id with spaces"));
}

#[test]
fn secret_like_target_reference_is_rejected_without_leaking_value() {
    let secret = "github/pull-request/42?token=super-sensitive";
    let error = SideEffectTargetReference::new(SideEffectTargetKind::AdapterResource, secret)
        .expect_err("secret-like target rejected");

    assert_eq!(error.code(), "side_effect.secret_like_value");
    assert!(!error.to_string().contains(secret));
}

#[test]
fn duplicate_references_are_rejected() {
    let duplicate =
        SideEffectReference::new(SideEffectReferenceKind::EvidenceReference, "evidence/1")
            .expect("valid reference");
    let mut definition = valid_definition(SideEffectLifecycleState::Proposed);
    definition.references = vec![duplicate.clone(), duplicate];

    let error = SideEffectRecord::new(definition).expect_err("duplicate reference rejected");

    assert_eq!(error.code(), "side_effect.reference.duplicate");
}

#[test]
fn missing_actor_and_system_actor_is_rejected() {
    let mut definition = valid_definition(SideEffectLifecycleState::Proposed);
    definition.actor = None;
    definition.system_actor = None;

    let error = SideEffectRecord::new(definition).expect_err("missing actor rejected");

    assert_eq!(error.code(), "side_effect.actor.required");
}

#[test]
fn system_actor_satisfies_actor_requirement() {
    let mut definition = valid_definition(SideEffectLifecycleState::Proposed);
    definition.actor = None;
    definition.system_actor = Some(system_actor());

    let record = SideEffectRecord::new(definition).expect("system actor accepted");

    assert_eq!(record.lifecycle_state(), SideEffectLifecycleState::Proposed);
}

#[test]
fn secret_like_summary_reason_and_redaction_metadata_are_rejected() {
    let mut definition = valid_definition(SideEffectLifecycleState::Proposed);
    definition.summary = Some("contains bearer token".to_owned());
    let error = SideEffectRecord::new(definition).expect_err("secret summary rejected");
    assert_eq!(error.code(), "side_effect.secret_like_value");
    assert!(!error.to_string().contains("bearer token"));

    let mut definition = valid_definition(SideEffectLifecycleState::Denied);
    definition.reason_codes = vec!["secret.reason".to_owned()];
    let error = SideEffectRecord::new(definition).expect_err("secret reason rejected");
    assert_eq!(error.code(), "side_effect.secret_like_value");
    assert!(!error.to_string().contains("secret.reason"));

    let mut definition = valid_definition(SideEffectLifecycleState::Proposed);
    definition.redaction = redaction_with("authorization_header", "safe reason");
    let error = SideEffectRecord::new(definition).expect_err("secret redaction field rejected");
    assert_eq!(error.code(), "side_effect.secret_like_value");
    assert!(!error.to_string().contains("authorization_header"));
}

#[test]
fn serde_round_trip_for_valid_record() {
    let record = valid_record(SideEffectLifecycleState::Completed);

    let encoded = serde_json::to_string(&record).expect("serialize record");
    let decoded: SideEffectRecord = serde_json::from_str(&encoded).expect("deserialize record");

    assert_eq!(decoded, record);
}

#[test]
fn invalid_serialized_record_fails_closed_without_leaking_secret_value() {
    let mut value = serde_json::to_value(valid_record(SideEffectLifecycleState::Proposed))
        .expect("serialize record to value");
    value["target"]["reference"] = json!("github/pr/1?api_token=do-not-leak");

    let error = serde_json::from_value::<SideEffectRecord>(value)
        .expect_err("invalid serialized target fails closed");
    let message = error.to_string();

    assert!(message.contains("side_effect.secret_like_value"));
    assert!(!message.contains("do-not-leak"));
    assert!(!message.contains("api_token=do-not-leak"));
}

#[test]
fn debug_output_redacts_sensitive_record_fields() {
    let record = valid_record(SideEffectLifecycleState::Completed);

    let debug = format!("{record:?}");

    assert!(!debug.contains("github/pull-request/42"));
    assert!(!debug.contains("workflow/self-governed-build"));
    assert!(!debug.contains("bounded side-effect record summary"));
    assert!(!debug.contains("event/policy-allowed"));
    assert!(debug.contains("reference_count"));
}

#[test]
fn serialization_does_not_include_forbidden_raw_payload_markers() {
    let record = valid_record(SideEffectLifecycleState::Completed);

    let encoded = serde_json::to_string(&record).expect("serialize record");

    for forbidden in [
        "raw_provider_payload",
        "raw_command_output",
        "raw_ci_log",
        "raw_spec_contents",
        "parser_payload",
        "authorization",
        "private_key",
        "api_token",
    ] {
        assert!(!encoded.contains(forbidden));
    }
}

#[test]
fn valid_redaction_metadata_still_works() {
    let mut definition = valid_definition(SideEffectLifecycleState::Proposed);
    definition.redaction = redaction_with(
        "target_reference",
        "record stores bounded reference instead of raw payload",
    );

    let record = SideEffectRecord::new(definition).expect("valid redaction metadata accepted");

    assert_eq!(record.lifecycle_state(), SideEffectLifecycleState::Proposed);
}

#[test]
fn idempotency_binding_can_reference_prior_side_effect_without_retry_behavior() {
    let binding = SideEffectIdempotencyBinding::new(
        IdempotencyKey::new("side-effect-key-duplicate").expect("valid key"),
        SideEffectIdempotencyScope::Adapter,
        Some(SideEffectId::new("side-effect/prior").expect("valid prior id")),
        Some(outcome(SideEffectOutcomeReferenceKind::Duplicate)),
    )
    .expect("valid duplicate binding");
    let mut definition = valid_definition(SideEffectLifecycleState::Proposed);
    definition.idempotency = binding;

    let record = SideEffectRecord::new(definition).expect("record with duplicate binding");

    assert_eq!(record.lifecycle_state(), SideEffectLifecycleState::Proposed);
}
