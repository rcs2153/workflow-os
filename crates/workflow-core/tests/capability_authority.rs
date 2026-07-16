#![allow(clippy::expect_used)]

//! Scoped capability-grant and availability core-model tests.

use workflow_core::{
    ActorId, ApprovalReferenceId, CapabilityAvailability, CapabilityAvailabilityRecord,
    CapabilityDelegationPosture, CapabilityGrant, CapabilityGrantDefinition, CapabilityGrantId,
    CapabilityGrantLifecycle, CapabilityGrantRequirements, CapabilityGrantScope,
    CapabilityReference, CapabilityResourceKind, CapabilityResourceScope, EvidenceReferenceId,
    HarnessContractId, LocalCheckResultId, PolicyId, RedactionDisposition, RedactionFieldState,
    RedactionMetadata, StepId, Timestamp, WorkReportSensitivity, WorkflowId, WorkflowOsErrorKind,
    WorkflowRunId,
};

fn timestamp(value: &str) -> Timestamp {
    Timestamp::parse_rfc3339(value).expect("valid timestamp")
}

fn scope() -> CapabilityGrantScope {
    CapabilityGrantScope::new(
        WorkflowId::new("workflow/review").expect("workflow id"),
        Some(WorkflowRunId::new("run-123").expect("run id")),
        Some(StepId::new("publish-comment").expect("step id")),
        Some(HarnessContractId::new("harness/reviewer").expect("harness id")),
    )
    .expect("valid scope")
}

fn redaction() -> RedactionMetadata {
    RedactionMetadata {
        redacted_fields: vec!["provider_payload".to_owned()],
        field_states: vec![RedactionFieldState {
            field: "resource".to_owned(),
            disposition: RedactionDisposition::ReferenceOnly,
            reason: "bounded stable reference".to_owned(),
        }],
    }
}

fn definition() -> CapabilityGrantDefinition {
    CapabilityGrantDefinition {
        grant_id: CapabilityGrantId::new("grant/review-comment").expect("grant id"),
        subject: ActorId::new("agent/reviewer").expect("subject"),
        capability: CapabilityReference::new("github.pull_request.comment.create")
            .expect("capability"),
        resource: CapabilityResourceScope::new(
            CapabilityResourceKind::Repository,
            "github/rcs2153/workflow-os",
        )
        .expect("resource"),
        scope: scope(),
        issuer: ActorId::new("user/maintainer").expect("issuer"),
        issued_at: timestamp("2026-07-15T10:00:00Z"),
        expires_at: Some(timestamp("2026-07-15T11:00:00Z")),
        lifecycle: CapabilityGrantLifecycle::Active,
        revocation_reference: None,
        delegation: CapabilityDelegationPosture::Disabled,
        requirements: CapabilityGrantRequirements::new(
            vec![PolicyId::new("policy/provider-write").expect("policy")],
            vec![ApprovalReferenceId::new("approval/run-123/publish").expect("approval")],
            vec![EvidenceReferenceId::new("evidence/preflight").expect("evidence")],
            vec![LocalCheckResultId::new("check/tests").expect("check")],
        )
        .expect("requirements"),
        sensitivity_ceiling: WorkReportSensitivity::Confidential,
        redaction: redaction(),
    }
}

#[test]
fn valid_scoped_capability_grant_preserves_required_fields() {
    let grant = CapabilityGrant::new(definition()).expect("valid grant");

    assert_eq!(grant.grant_id().as_str(), "grant/review-comment");
    assert_eq!(grant.subject().as_str(), "agent/reviewer");
    assert_eq!(
        grant.capability().as_str(),
        "github.pull_request.comment.create"
    );
    assert_eq!(grant.resource().kind(), CapabilityResourceKind::Repository);
    assert_eq!(grant.scope().run_id().expect("run").as_str(), "run-123");
    assert_eq!(grant.issuer().as_str(), "user/maintainer");
    assert_eq!(grant.lifecycle(), CapabilityGrantLifecycle::Active);
    assert_eq!(
        grant.sensitivity_ceiling(),
        WorkReportSensitivity::Confidential
    );
}

#[test]
fn all_capability_availability_states_are_explicitly_representable() {
    let states = [
        CapabilityAvailability::Available,
        CapabilityAvailability::DeclaredNotConnected,
        CapabilityAvailability::KnownUnsupported,
        CapabilityAvailability::Unknown,
    ];

    for state in states {
        let record = CapabilityAvailabilityRecord::new(
            CapabilityReference::new("github.pull_request.read").expect("capability"),
            CapabilityResourceScope::new(
                CapabilityResourceKind::Repository,
                "github/rcs2153/workflow-os",
            )
            .expect("resource"),
            state,
            timestamp("2026-07-15T10:00:00Z"),
            RedactionMetadata::empty(),
        )
        .expect("availability record");
        assert_eq!(record.availability(), state);
    }
}

#[test]
fn invalid_and_secret_like_grant_identifiers_are_rejected_without_leakage() {
    let invalid = CapabilityGrantId::new("grant with spaces").expect_err("invalid id");
    assert_eq!(
        invalid.code(),
        "capability_authority.identifier.invalid_character"
    );

    let secret_value = "api_token_super_sensitive";
    let secret = CapabilityReference::new(secret_value).expect_err("secret-like value");
    assert_eq!(secret.code(), "capability_authority.secret_like_value");
    assert!(!secret.to_string().contains(secret_value));
}

#[test]
fn resource_scope_rejects_unknown_raw_paths_urls_and_secret_like_values() {
    for (kind, value, code) in [
        (
            CapabilityResourceKind::Unknown,
            "github/owner/repo",
            "capability_authority.resource.kind_unknown",
        ),
        (
            CapabilityResourceKind::Repository,
            "/Users/example/private",
            "capability_authority.resource.not_canonical",
        ),
        (
            CapabilityResourceKind::Repository,
            "https://github.com/owner/repo",
            "capability_authority.resource.not_canonical",
        ),
        (
            CapabilityResourceKind::Repository,
            "github/api_token_value",
            "capability_authority.secret_like_value",
        ),
    ] {
        let error = CapabilityResourceScope::new(kind, value).expect_err("invalid resource");
        assert_eq!(error.code(), code);
        assert!(!error.to_string().contains(value));
    }
}

#[test]
fn step_scope_requires_exact_run_binding() {
    let error = CapabilityGrantScope::new(
        WorkflowId::new("workflow/review").expect("workflow"),
        None,
        Some(StepId::new("publish").expect("step")),
        None,
    )
    .expect_err("step without run");

    assert_eq!(error.code(), "capability_authority.scope.step_requires_run");
}

#[test]
fn expiry_must_follow_issuance() {
    let mut value = definition();
    value.expires_at = Some(value.issued_at);

    let error = CapabilityGrant::new(value).expect_err("invalid expiry");
    assert_eq!(
        error.code(),
        "capability_authority.expiry.not_after_issuance"
    );
}

#[test]
fn revocation_posture_is_consistent_and_bounded() {
    let mut active = definition();
    active.revocation_reference = Some("revocation/123".to_owned());
    assert_eq!(
        CapabilityGrant::new(active)
            .expect_err("active reference")
            .code(),
        "capability_authority.revocation.active_has_reference"
    );

    let mut revoked = definition();
    revoked.lifecycle = CapabilityGrantLifecycle::Revoked;
    assert_eq!(
        CapabilityGrant::new(revoked)
            .expect_err("missing reference")
            .code(),
        "capability_authority.revocation.reference_required"
    );

    let mut valid = definition();
    valid.lifecycle = CapabilityGrantLifecycle::Revoked;
    valid.revocation_reference = Some("revocation/123".to_owned());
    assert_eq!(
        CapabilityGrant::new(valid)
            .expect("revoked grant")
            .lifecycle(),
        CapabilityGrantLifecycle::Revoked
    );
}

#[test]
fn delegation_is_disabled_by_default_and_bounded_when_enabled() {
    assert_eq!(
        CapabilityDelegationPosture::default(),
        CapabilityDelegationPosture::Disabled
    );

    for depth in [0, 9] {
        let mut value = definition();
        value.delegation = CapabilityDelegationPosture::Allowed { max_depth: depth };
        assert_eq!(
            CapabilityGrant::new(value)
                .expect_err("invalid depth")
                .code(),
            "capability_authority.delegation.depth_invalid"
        );
    }

    let mut value = definition();
    value.delegation = CapabilityDelegationPosture::Allowed { max_depth: 2 };
    assert_eq!(
        CapabilityGrant::new(value)
            .expect("bounded delegation")
            .delegation(),
        CapabilityDelegationPosture::Allowed { max_depth: 2 }
    );
}

#[test]
fn duplicate_prerequisite_references_are_rejected() {
    let policy = PolicyId::new("policy/provider-write").expect("policy");
    let error =
        CapabilityGrantRequirements::new(vec![policy.clone(), policy], vec![], vec![], vec![])
            .expect_err("duplicate policy");
    assert_eq!(error.code(), "capability_authority.requirements.duplicate");
}

#[test]
fn unknown_sensitivity_ceiling_fails_closed() {
    let mut value = definition();
    value.sensitivity_ceiling = WorkReportSensitivity::Unknown;

    let error = CapabilityGrant::new(value).expect_err("unknown sensitivity");
    assert_eq!(error.code(), "capability_authority.sensitivity.unknown");
}

#[test]
fn valid_grant_and_availability_record_round_trip_through_json() {
    let grant = CapabilityGrant::new(definition()).expect("grant");
    let serialized = serde_json::to_string(&grant).expect("serialize grant");
    let decoded: CapabilityGrant = serde_json::from_str(&serialized).expect("decode grant");
    assert_eq!(decoded, grant);

    let record = CapabilityAvailabilityRecord::new(
        CapabilityReference::new("github.pull_request.read").expect("capability"),
        CapabilityResourceScope::new(
            CapabilityResourceKind::Repository,
            "github/rcs2153/workflow-os",
        )
        .expect("resource"),
        CapabilityAvailability::Available,
        timestamp("2026-07-15T10:00:00Z"),
        RedactionMetadata::empty(),
    )
    .expect("availability");
    let serialized = serde_json::to_string(&record).expect("serialize availability");
    let decoded: CapabilityAvailabilityRecord =
        serde_json::from_str(&serialized).expect("decode availability");
    assert_eq!(decoded, record);
}

#[test]
fn invalid_serialized_grant_fails_closed_without_echoing_secret_like_metadata() {
    let grant = CapabilityGrant::new(definition()).expect("grant");
    let mut wire = serde_json::to_value(grant).expect("serialize");
    let secret_value = "bearer-super-sensitive-value";
    wire["redaction"]["field_states"][0]["reason"] = serde_json::json!(secret_value);

    let error = serde_json::from_value::<CapabilityGrant>(wire).expect_err("invalid metadata");
    let rendered = error.to_string();
    assert!(rendered.contains("capability_authority.secret_like_value"));
    assert!(!rendered.contains(secret_value));
}

#[test]
fn debug_output_redacts_grant_and_availability_identifiers() {
    let grant = CapabilityGrant::new(definition()).expect("grant");
    let debug = format!("{grant:?}");
    for forbidden in [
        "grant/review-comment",
        "agent/reviewer",
        "github.pull_request.comment.create",
        "github/rcs2153/workflow-os",
        "user/maintainer",
        "provider_payload",
        "bounded stable reference",
    ] {
        assert!(!debug.contains(forbidden));
    }

    let availability = CapabilityAvailabilityRecord::new(
        CapabilityReference::new("github.pull_request.read").expect("capability"),
        CapabilityResourceScope::new(
            CapabilityResourceKind::Repository,
            "github/private/repository",
        )
        .expect("resource"),
        CapabilityAvailability::Available,
        timestamp("2026-07-15T10:00:00Z"),
        redaction(),
    )
    .expect("availability");
    let debug = format!("{availability:?}");
    assert!(!debug.contains("github.pull_request.read"));
    assert!(!debug.contains("github/private/repository"));
    assert!(!debug.contains("provider_payload"));

    let definition_debug = format!("{:?}", definition());
    assert!(!definition_debug.contains("grant/review-comment"));
    assert!(!definition_debug.contains("workflow/review"));
    assert!(!definition_debug.contains("approval/run-123/publish"));
}

#[test]
fn availability_records_cannot_assert_authority_outcomes() {
    let base = serde_json::json!({
        "capability": "github.pull_request.read",
        "resource": {
            "kind": "repository",
            "reference": "github/rcs2153/workflow-os"
        },
        "availability": "available",
        "observed_at": "2026-07-15T10:00:00Z",
        "redaction": {
            "redacted_fields": [],
            "field_states": []
        }
    });

    serde_json::from_value::<CapabilityAvailabilityRecord>(base.clone())
        .expect("inventory availability remains valid");

    for authority_outcome in [
        "available_and_authorized",
        "available_not_authorized",
        "expired_or_revoked",
        "denied",
    ] {
        let mut wire = base.clone();
        wire["availability"] = serde_json::json!(authority_outcome);
        serde_json::from_value::<CapabilityAvailabilityRecord>(wire)
            .expect_err("availability must not assert authority outcomes");
    }
}

#[test]
fn model_stores_references_not_forbidden_payload_fields() {
    let serialized = serde_json::to_string(&CapabilityGrant::new(definition()).expect("grant"))
        .expect("serialize");
    for forbidden_field in [
        "provider_payload",
        "command_output",
        "raw_spec_contents",
        "environment_values",
        "credentials",
        "authorization_header",
        "private_key",
    ] {
        assert!(!serialized.contains(&format!("\"{forbidden_field}\":")));
    }
}

#[test]
fn validation_errors_use_stable_non_leaking_codes() {
    let secret = "private_key_material";
    let error = CapabilityReference::new(secret).expect_err("secret-like capability");
    assert_eq!(error.kind(), WorkflowOsErrorKind::Validation);
    assert_eq!(error.code(), "capability_authority.secret_like_value");
    assert!(!error.message().contains(secret));
}
