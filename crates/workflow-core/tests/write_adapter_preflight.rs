#![allow(clippy::expect_used)]

//! Write adapter preflight helper tests.

use serde_json::json;
use workflow_core::{
    preflight_adapter_write, AdapterWriteCapability, AdapterWritePolicyDecision,
    AdapterWritePreflightRequest, AdapterWritePreflightRequestDefinition,
    AdapterWriteReadinessPolicy, AdapterWriteReadinessPolicyDefinition, AdapterWriteTarget,
    AdapterWriteTargetKind, IdempotencyKey, RedactionDisposition, RedactionFieldState,
    RedactionMetadata, SideEffectId, SideEffectReference, SideEffectReferenceKind,
    SideEffectSensitivity, WorkflowOsErrorKind,
};

fn target() -> AdapterWriteTarget {
    AdapterWriteTarget::new(AdapterWriteTargetKind::GitHubPullRequest, "github/pr/123")
        .expect("valid target")
}

fn side_effect_id() -> SideEffectId {
    SideEffectId::new("side-effect/write-preflight").expect("valid side-effect id")
}

fn idempotency_key() -> IdempotencyKey {
    IdempotencyKey::new("write-preflight-key").expect("valid idempotency key")
}

fn policy_ref() -> SideEffectReference {
    SideEffectReference::new(
        SideEffectReferenceKind::PolicyDecision,
        "event/policy-allowed",
    )
    .expect("valid policy reference")
}

fn approval_ref() -> SideEffectReference {
    SideEffectReference::new(
        SideEffectReferenceKind::ApprovalDecision,
        "approval/decision-1",
    )
    .expect("valid approval reference")
}

fn evidence_ref() -> SideEffectReference {
    SideEffectReference::new(
        SideEffectReferenceKind::EvidenceReference,
        "evidence/reference-1",
    )
    .expect("valid evidence reference")
}

fn valid_definition() -> AdapterWritePreflightRequestDefinition {
    AdapterWritePreflightRequestDefinition {
        capability: AdapterWriteCapability::GitHubPullRequestComment,
        target: target(),
        side_effect_id: Some(side_effect_id()),
        idempotency_key: Some(idempotency_key()),
        policy_decision: AdapterWritePolicyDecision::Allowed,
        policy_references: vec![policy_ref()],
        requires_approval: false,
        approval_references: Vec::new(),
        high_assurance_required: false,
        high_assurance_references: Vec::new(),
        summary: "bounded preflight summary".to_owned(),
        sensitivity: SideEffectSensitivity::Internal,
        redaction: RedactionMetadata::empty(),
        readiness_policy: AdapterWriteReadinessPolicy::local_preview_comments_only(),
    }
}

fn valid_request() -> AdapterWritePreflightRequest {
    AdapterWritePreflightRequest::new(valid_definition()).expect("valid request")
}

#[test]
fn valid_low_risk_preflight_returns_ready_decision_without_execution_authority() {
    let request = valid_request();

    let decision = preflight_adapter_write(&request).expect("ready preflight");

    assert_eq!(
        decision.capability(),
        AdapterWriteCapability::GitHubPullRequestComment
    );
    assert_eq!(
        decision.side_effect_id().as_str(),
        "side-effect/write-preflight"
    );
    assert_eq!(decision.idempotency_key().as_str(), "write-preflight-key");
    assert!(decision
        .reason_codes()
        .iter()
        .any(|code| code == "adapter_write_preflight.ready"));
    assert!(!decision.provider_call_allowed());
    assert!(!decision.side_effect_lifecycle_transition_allowed());
    assert!(!decision.workflow_event_append_allowed());
    assert!(!decision.report_artifact_write_allowed());
}

#[test]
fn unsupported_capability_is_rejected_fail_closed() {
    let mut definition = valid_definition();
    definition.capability = AdapterWriteCapability::GitHubMerge;

    let request = AdapterWritePreflightRequest::new(definition).expect("constructable request");
    let error = preflight_adapter_write(&request).expect_err("unsupported capability");

    assert_eq!(error.kind(), WorkflowOsErrorKind::Validation);
    assert_eq!(
        error.code(),
        "adapter_write_preflight.capability.unsupported"
    );
}

#[test]
fn unknown_capability_is_rejected_before_decision() {
    let mut definition = valid_definition();
    definition.capability = AdapterWriteCapability::Unknown;

    let error = AdapterWritePreflightRequest::new(definition).expect_err("unknown capability");

    assert_eq!(error.code(), "adapter_write_preflight.capability.unknown");
}

#[test]
fn missing_side_effect_id_is_rejected() {
    let mut definition = valid_definition();
    definition.side_effect_id = None;
    let request = AdapterWritePreflightRequest::new(definition).expect("constructable request");

    let error = preflight_adapter_write(&request).expect_err("missing side effect");

    assert_eq!(error.code(), "adapter_write_preflight.side_effect.missing");
}

#[test]
fn missing_idempotency_key_is_rejected() {
    let mut definition = valid_definition();
    definition.idempotency_key = None;
    let request = AdapterWritePreflightRequest::new(definition).expect("constructable request");

    let error = preflight_adapter_write(&request).expect_err("missing idempotency");

    assert_eq!(error.code(), "adapter_write_preflight.idempotency.missing");
}

#[test]
fn missing_policy_reference_is_rejected() {
    let mut definition = valid_definition();
    definition.policy_references.clear();
    let request = AdapterWritePreflightRequest::new(definition).expect("constructable request");

    let error = preflight_adapter_write(&request).expect_err("missing policy");

    assert_eq!(error.code(), "adapter_write_preflight.policy.missing");
}

#[test]
fn denied_policy_is_rejected_as_policy_denied() {
    let mut definition = valid_definition();
    definition.policy_decision = AdapterWritePolicyDecision::Denied;
    let request = AdapterWritePreflightRequest::new(definition).expect("constructable request");

    let error = preflight_adapter_write(&request).expect_err("denied policy");

    assert_eq!(error.kind(), WorkflowOsErrorKind::PolicyDenied);
    assert_eq!(error.code(), "adapter_write_preflight.policy.denied");
}

#[test]
fn approval_required_without_approval_reference_is_rejected() {
    let mut definition = valid_definition();
    definition.requires_approval = true;
    let request = AdapterWritePreflightRequest::new(definition).expect("constructable request");

    let error = preflight_adapter_write(&request).expect_err("missing approval");

    assert_eq!(error.code(), "adapter_write_preflight.approval.missing");
}

#[test]
fn sensitive_policy_capability_requires_approval_reference() {
    let mut definition = valid_definition();
    definition.readiness_policy =
        AdapterWriteReadinessPolicy::new(AdapterWriteReadinessPolicyDefinition {
            supported_capabilities: vec![AdapterWriteCapability::GitHubPullRequestComment],
            sensitive_capabilities: vec![AdapterWriteCapability::GitHubPullRequestComment],
        })
        .expect("valid readiness policy");

    let request = AdapterWritePreflightRequest::new(definition).expect("constructable request");
    let error = preflight_adapter_write(&request).expect_err("missing sensitive approval");

    assert_eq!(error.code(), "adapter_write_preflight.approval.missing");
}

#[test]
fn high_assurance_required_without_reference_is_rejected() {
    let mut definition = valid_definition();
    definition.high_assurance_required = true;
    definition.approval_references = vec![approval_ref()];
    let request = AdapterWritePreflightRequest::new(definition).expect("constructable request");

    let error = preflight_adapter_write(&request).expect_err("missing high assurance");

    assert_eq!(
        error.code(),
        "adapter_write_preflight.high_assurance.missing"
    );
}

#[test]
fn high_assurance_reference_accepts_stable_evidence_reference() {
    let mut definition = valid_definition();
    definition.requires_approval = true;
    definition.approval_references = vec![approval_ref()];
    definition.high_assurance_required = true;
    definition.high_assurance_references = vec![evidence_ref()];

    let request = AdapterWritePreflightRequest::new(definition).expect("valid request");
    let decision = preflight_adapter_write(&request).expect("ready preflight");

    assert!(decision
        .reason_codes()
        .iter()
        .any(|code| code == "adapter_write_preflight.high_assurance_verified"));
}

#[test]
fn duplicate_policy_references_are_rejected() {
    let mut definition = valid_definition();
    definition.policy_references = vec![policy_ref(), policy_ref()];
    let request = AdapterWritePreflightRequest::new(definition).expect("constructable request");

    let error = preflight_adapter_write(&request).expect_err("duplicate reference");

    assert_eq!(error.code(), "adapter_write_preflight.reference.duplicate");
}

#[test]
fn secret_like_target_is_rejected_without_leaking() {
    let error = AdapterWriteTarget::new(
        AdapterWriteTargetKind::GitHubPullRequest,
        "github/pr/authorization-token",
    )
    .expect_err("secret-like target");

    assert_eq!(error.code(), "adapter_write_preflight.secret_like_value");
    assert!(!error.to_string().contains("authorization-token"));
}

#[test]
fn secret_like_summary_is_rejected_without_leaking() {
    let mut definition = valid_definition();
    definition.summary = "raw provider output bearer-token-super-secret".to_owned();

    let error = AdapterWritePreflightRequest::new(definition).expect_err("secret-like summary");

    assert_eq!(error.code(), "adapter_write_preflight.secret_like_value");
    assert!(!error.to_string().contains("bearer-token-super-secret"));
}

#[test]
fn secret_like_redaction_metadata_is_rejected_without_leaking() {
    let mut definition = valid_definition();
    definition.redaction = RedactionMetadata {
        redacted_fields: vec!["authorization_header".to_owned()],
        field_states: vec![RedactionFieldState {
            field: "authorization_header".to_owned(),
            disposition: RedactionDisposition::ReferenceOnly,
            reason: "contains bearer token".to_owned(),
        }],
    };

    let error = AdapterWritePreflightRequest::new(definition).expect_err("secret-like redaction");

    assert_eq!(error.code(), "adapter_write_preflight.secret_like_value");
    assert!(!error.to_string().contains("authorization_header"));
    assert!(!error.to_string().contains("bearer token"));
}

#[test]
fn debug_output_redacts_target_summary_ids_and_redaction_metadata() {
    let mut definition = valid_definition();
    definition.requires_approval = true;
    definition.approval_references = vec![approval_ref()];
    definition.summary = "bounded safe report text".to_owned();
    let request = AdapterWritePreflightRequest::new(definition).expect("valid request");
    let debug = format!("{request:?}");

    assert!(!debug.contains("github/pr/123"));
    assert!(!debug.contains("bounded safe report text"));
    assert!(!debug.contains("side-effect/write-preflight"));
    assert!(!debug.contains("write-preflight-key"));
    assert!(debug.contains("[REDACTED]"));

    let decision = preflight_adapter_write(&request).expect("ready preflight");
    let decision_debug = format!("{decision:?}");
    assert!(!decision_debug.contains("side-effect/write-preflight"));
    assert!(!decision_debug.contains("write-preflight-key"));
}

#[test]
fn serde_round_trip_preserves_valid_request() {
    let mut definition = valid_definition();
    definition.requires_approval = true;
    definition.approval_references = vec![approval_ref()];
    let request = AdapterWritePreflightRequest::new(definition).expect("valid request");

    let serialized = serde_json::to_string(&request).expect("serialize request");
    let round_trip: AdapterWritePreflightRequest =
        serde_json::from_str(&serialized).expect("deserialize request");

    assert_eq!(
        round_trip.capability(),
        AdapterWriteCapability::GitHubPullRequestComment
    );
    assert_eq!(round_trip.policy_references().len(), 1);
    assert_eq!(round_trip.approval_references().len(), 1);
}

#[test]
fn invalid_serialized_secret_like_target_fails_closed_without_leaking() {
    let mut value = serde_json::to_value(valid_request()).expect("serialize request");
    value["target"]["reference"] = json!("github/pr/private_key_material");

    let error = serde_json::from_value::<AdapterWritePreflightRequest>(value)
        .expect_err("secret-like target rejected");

    assert!(error
        .to_string()
        .contains("adapter_write_preflight.secret_like_value"));
    assert!(!error.to_string().contains("private_key_material"));
}

#[test]
fn serialization_does_not_include_forbidden_raw_payload_markers() {
    let request = valid_request();
    let serialized = serde_json::to_string(&request).expect("serialize request");

    assert!(!serialized.contains("raw provider payload"));
    assert!(!serialized.contains("raw command output"));
    assert!(!serialized.contains("raw parser payload"));
    assert!(!serialized.contains("authorization"));
    assert!(!serialized.contains("private_key"));
}

#[test]
fn read_only_adapter_vocabulary_remains_outside_write_preflight() {
    let policy = AdapterWriteReadinessPolicy::local_preview_comments_only();

    assert!(!policy
        .supported_capabilities()
        .contains(&AdapterWriteCapability::GenericProviderWrite));
    assert!(!policy
        .supported_capabilities()
        .contains(&AdapterWriteCapability::CiRerun));
}
