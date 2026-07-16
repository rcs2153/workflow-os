#![allow(clippy::expect_used)]

//! Scoped capability-grant and availability core-model tests.

use workflow_core::{
    project_capability_request_for_review, resolve_capability_authority, ActorId,
    ApprovalReferenceId, CapabilityAvailability, CapabilityAvailabilityRecord,
    CapabilityDelegationPosture, CapabilityGrant, CapabilityGrantDefinition, CapabilityGrantId,
    CapabilityGrantLifecycle, CapabilityGrantRequirements, CapabilityGrantScope,
    CapabilityReference, CapabilityRequest, CapabilityRequestAuthorityPosture,
    CapabilityRequestDefinition, CapabilityRequestId, CapabilityRequestPurpose,
    CapabilityRequestReviewAction, CapabilityResolution, CapabilityResolutionInput,
    CapabilityResolutionPosture, CapabilityResolutionReason, CapabilityResourceKind,
    CapabilityResourceScope, EvidenceReferenceId, HarnessContractId, LocalCheckResultId, PolicyId,
    RedactionDisposition, RedactionFieldState, RedactionMetadata, StepId, Timestamp,
    WorkReportSensitivity, WorkflowId, WorkflowOsErrorKind, WorkflowRunId,
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

fn grant_without_requirements() -> CapabilityGrant {
    let mut value = definition();
    value.requirements = CapabilityGrantRequirements::default();
    CapabilityGrant::new(value).expect("grant")
}

fn available_record() -> CapabilityAvailabilityRecord {
    CapabilityAvailabilityRecord::new(
        CapabilityReference::new("github.pull_request.comment.create").expect("capability"),
        CapabilityResourceScope::new(
            CapabilityResourceKind::Repository,
            "github/rcs2153/workflow-os",
        )
        .expect("resource"),
        CapabilityAvailability::Available,
        timestamp("2026-07-15T10:00:00Z"),
        RedactionMetadata::empty(),
    )
    .expect("availability")
}

fn resolution_input<'a>(
    grant: &'a CapabilityGrant,
    availability_records: &'a [CapabilityAvailabilityRecord],
    grants: &'a [CapabilityGrant],
) -> CapabilityResolutionInput<'a> {
    CapabilityResolutionInput {
        capability: grant.capability(),
        resource: grant.resource(),
        actor: grant.subject(),
        workflow_id: grant.scope().workflow_id(),
        run_id: grant.scope().run_id().expect("run"),
        step_id: grant.scope().step_id().expect("step"),
        harness_contract_id: grant.scope().harness_contract_id(),
        requested_sensitivity: WorkReportSensitivity::Internal,
        evaluated_at: timestamp("2026-07-15T10:30:00Z"),
        availability_records,
        grants,
    }
}

fn request_definition(
    grant: &CapabilityGrant,
    resolution: CapabilityResolution,
) -> CapabilityRequestDefinition {
    CapabilityRequestDefinition {
        request_id: CapabilityRequestId::new("request/review-comment").expect("request id"),
        capability: grant.capability().clone(),
        resource: grant.resource().clone(),
        purpose: CapabilityRequestPurpose::WorkflowStep,
        requester: grant.subject().clone(),
        workflow_id: grant.scope().workflow_id().clone(),
        run_id: grant.scope().run_id().expect("run").clone(),
        step_id: grant.scope().step_id().expect("step").clone(),
        harness_contract_id: grant.scope().harness_contract_id().cloned(),
        requested_sensitivity: WorkReportSensitivity::Internal,
        resolution,
        review_steward: Some(ActorId::new("user/maintainer").expect("steward")),
        requested_at: timestamp("2026-07-15T10:31:00Z"),
        expires_at: timestamp("2026-07-15T11:31:00Z"),
        redaction: redaction(),
    }
}

fn resolution_context_wire() -> serde_json::Value {
    serde_json::json!({
        "capability": "github.pull_request.comment.create",
        "resource": {
            "kind": "repository",
            "reference": "github/rcs2153/workflow-os"
        },
        "actor": "agent/reviewer",
        "workflow_id": "workflow/review",
        "run_id": "run/review-123",
        "step_id": "step/comment",
        "harness_contract_id": "harness/reviewer",
        "requested_sensitivity": "internal"
    })
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

#[test]
fn available_capability_with_exact_active_grant_is_authorized() {
    let grants = [grant_without_requirements()];
    let records = [available_record()];
    let input = resolution_input(&grants[0], &records, &grants);

    let result = resolve_capability_authority(&input).expect("resolution");

    assert_eq!(result.posture(), CapabilityResolutionPosture::Authorized);
    assert_eq!(
        result.availability(),
        Some(CapabilityAvailability::Available)
    );
    assert_eq!(
        result.selected_grant_id().expect("grant").as_str(),
        "grant/review-comment"
    );
    assert_eq!(
        result.reasons(),
        &[CapabilityResolutionReason::ActiveGrantMatched]
    );
}

#[test]
fn availability_without_matching_grant_never_authorizes() {
    let grant = grant_without_requirements();
    let records = [available_record()];
    let input = resolution_input(&grant, &records, &[]);

    let result = resolve_capability_authority(&input).expect("resolution");

    assert_eq!(result.posture(), CapabilityResolutionPosture::NotAuthorized);
    assert_eq!(
        result.reasons(),
        &[CapabilityResolutionReason::NoMatchingGrant]
    );
}

#[test]
fn unavailable_inventory_states_fail_closed_before_grant_resolution() {
    for (availability, reason) in [
        (
            CapabilityAvailability::DeclaredNotConnected,
            CapabilityResolutionReason::CapabilityNotConnected,
        ),
        (
            CapabilityAvailability::KnownUnsupported,
            CapabilityResolutionReason::CapabilityUnsupported,
        ),
        (
            CapabilityAvailability::Unknown,
            CapabilityResolutionReason::CapabilityAvailabilityUnknown,
        ),
    ] {
        let grants = [grant_without_requirements()];
        let records = [CapabilityAvailabilityRecord::new(
            grants[0].capability().clone(),
            grants[0].resource().clone(),
            availability,
            timestamp("2026-07-15T10:00:00Z"),
            RedactionMetadata::empty(),
        )
        .expect("record")];
        let input = resolution_input(&grants[0], &records, &grants);

        let result = resolve_capability_authority(&input).expect("resolution");
        assert_eq!(result.posture(), CapabilityResolutionPosture::NotAuthorized);
        assert_eq!(result.reasons(), &[reason]);
    }
}

#[test]
fn missing_inventory_record_fails_closed() {
    let grants = [grant_without_requirements()];
    let input = resolution_input(&grants[0], &[], &grants);

    let result = resolve_capability_authority(&input).expect("resolution");

    assert_eq!(result.availability(), None);
    assert_eq!(
        result.reasons(),
        &[CapabilityResolutionReason::AvailabilityRecordMissing]
    );
}

#[test]
fn matching_grant_prerequisites_require_independent_evaluation() {
    let grants = [CapabilityGrant::new(definition()).expect("grant")];
    let records = [available_record()];
    let input = resolution_input(&grants[0], &records, &grants);

    let result = resolve_capability_authority(&input).expect("resolution");

    assert_eq!(
        result.posture(),
        CapabilityResolutionPosture::RequiresIndependentEvaluation
    );
    assert_eq!(
        result.reasons(),
        &[
            CapabilityResolutionReason::PolicyEvaluationRequired,
            CapabilityResolutionReason::ApprovalEvaluationRequired,
            CapabilityResolutionReason::EvidenceEvaluationRequired,
            CapabilityResolutionReason::CheckEvaluationRequired,
        ]
    );
}

#[test]
fn actor_run_step_and_harness_mismatches_do_not_match_grant_scope() {
    let grants = [grant_without_requirements()];
    let records = [available_record()];
    let other_actor = ActorId::new("agent/other").expect("actor");
    let other_run = WorkflowRunId::new("run-other").expect("run");
    let other_step = StepId::new("other-step").expect("step");
    let other_harness = HarnessContractId::new("harness/other").expect("harness");

    let mut actor_input = resolution_input(&grants[0], &records, &grants);
    actor_input.actor = &other_actor;
    let mut run_input = resolution_input(&grants[0], &records, &grants);
    run_input.run_id = &other_run;
    let mut step_input = resolution_input(&grants[0], &records, &grants);
    step_input.step_id = &other_step;
    let mut harness_input = resolution_input(&grants[0], &records, &grants);
    harness_input.harness_contract_id = Some(&other_harness);

    for input in [actor_input, run_input, step_input, harness_input] {
        assert_eq!(
            resolve_capability_authority(&input)
                .expect("resolution")
                .reasons(),
            &[CapabilityResolutionReason::NoMatchingGrant]
        );
    }
}

#[test]
fn revoked_expired_and_insufficient_sensitivity_grants_fail_closed() {
    let mut revoked = definition();
    revoked.lifecycle = CapabilityGrantLifecycle::Revoked;
    revoked.revocation_reference = Some("revocation/123".to_owned());
    revoked.requirements = CapabilityGrantRequirements::default();

    let mut expired = definition();
    expired.expires_at = Some(timestamp("2026-07-15T10:20:00Z"));
    expired.requirements = CapabilityGrantRequirements::default();

    let mut insufficient = definition();
    insufficient.sensitivity_ceiling = WorkReportSensitivity::Public;
    insufficient.requirements = CapabilityGrantRequirements::default();

    for (grant, expected) in [
        (
            CapabilityGrant::new(revoked).expect("revoked"),
            CapabilityResolutionReason::MatchingGrantRevoked,
        ),
        (
            CapabilityGrant::new(expired).expect("expired"),
            CapabilityResolutionReason::MatchingGrantExpired,
        ),
        (
            CapabilityGrant::new(insufficient).expect("insufficient"),
            CapabilityResolutionReason::SensitivityExceedsGrant,
        ),
    ] {
        let grants = [grant];
        let records = [available_record()];
        let input = resolution_input(&grants[0], &records, &grants);
        let result = resolve_capability_authority(&input).expect("resolution");
        assert_eq!(result.posture(), CapabilityResolutionPosture::NotAuthorized);
        assert_eq!(result.reasons(), &[expected]);
    }
}

#[test]
fn most_specific_authorizing_grant_is_selected_deterministically() {
    let mut broad = definition();
    broad.grant_id = CapabilityGrantId::new("grant/broad").expect("id");
    broad.scope = CapabilityGrantScope::new(broad.scope.workflow_id().clone(), None, None, None)
        .expect("scope");
    broad.requirements = CapabilityGrantRequirements::default();
    let grants = [
        CapabilityGrant::new(broad).expect("broad"),
        grant_without_requirements(),
    ];
    let records = [available_record()];
    let input = resolution_input(&grants[1], &records, &grants);

    let result = resolve_capability_authority(&input).expect("resolution");

    assert_eq!(
        result.selected_grant_id().expect("grant").as_str(),
        "grant/review-comment"
    );
}

#[test]
fn broader_grant_cannot_bypass_more_specific_prerequisites() {
    let mut broad = definition();
    broad.grant_id = CapabilityGrantId::new("grant/broad").expect("id");
    broad.scope = CapabilityGrantScope::new(broad.scope.workflow_id().clone(), None, None, None)
        .expect("scope");
    broad.requirements = CapabilityGrantRequirements::default();
    let grants = [
        CapabilityGrant::new(broad).expect("broad"),
        CapabilityGrant::new(definition()).expect("specific"),
    ];
    let records = [available_record()];
    let input = resolution_input(&grants[1], &records, &grants);

    let result = resolve_capability_authority(&input).expect("resolution");

    assert_eq!(
        result.posture(),
        CapabilityResolutionPosture::RequiresIndependentEvaluation
    );
    assert_eq!(
        result.selected_grant_id().expect("grant").as_str(),
        "grant/review-comment"
    );
    assert!(result
        .reasons()
        .contains(&CapabilityResolutionReason::PolicyEvaluationRequired));
}

#[test]
fn ambiguous_inventory_duplicate_grants_and_future_observations_fail_closed() {
    let grant = grant_without_requirements();
    let duplicate_records = [available_record(), available_record()];
    let grants = [grant.clone()];
    assert_eq!(
        resolve_capability_authority(&resolution_input(&grant, &duplicate_records, &grants,))
            .expect_err("ambiguous")
            .code(),
        "capability_authority.resolution.availability_ambiguous"
    );

    let records = [available_record()];
    let duplicate_grants = [grant.clone(), grant.clone()];
    assert_eq!(
        resolve_capability_authority(&resolution_input(&grant, &records, &duplicate_grants,))
            .expect_err("duplicate")
            .code(),
        "capability_authority.resolution.duplicate_grant"
    );

    let future_records = [CapabilityAvailabilityRecord::new(
        grant.capability().clone(),
        grant.resource().clone(),
        CapabilityAvailability::Available,
        timestamp("2026-07-15T11:00:00Z"),
        RedactionMetadata::empty(),
    )
    .expect("record")];
    assert_eq!(
        resolve_capability_authority(&resolution_input(&grant, &future_records, &grants,))
            .expect_err("future")
            .code(),
        "capability_authority.resolution.observation_in_future"
    );
}

#[test]
fn unknown_requested_sensitivity_fails_closed() {
    let grants = [grant_without_requirements()];
    let records = [available_record()];
    let mut input = resolution_input(&grants[0], &records, &grants);
    input.requested_sensitivity = WorkReportSensitivity::Unknown;

    assert_eq!(
        resolve_capability_authority(&input)
            .expect_err("unknown")
            .code(),
        "capability_authority.resolution.sensitivity_unknown"
    );
}

#[test]
fn resolution_debug_does_not_leak_sensitive_identifiers() {
    let grants = [grant_without_requirements()];
    let records = [available_record()];
    let input = resolution_input(&grants[0], &records, &grants);
    let result = resolve_capability_authority(&input).expect("resolution");

    let input_debug = format!("{input:?}");
    let result_debug = format!("{result:?}");
    for value in [
        "github.pull_request.comment.create",
        "github/rcs2153/workflow-os",
        "agent/reviewer",
        "grant/review-comment",
    ] {
        assert!(!input_debug.contains(value));
        assert!(!result_debug.contains(value));
    }

    let serialized = serde_json::to_string(&result).expect("serialize");
    assert!(serialized.contains("active_grant_matched"));
    assert!(!serialized.contains("provider_payload"));

    let decoded: workflow_core::CapabilityResolution =
        serde_json::from_str(&serialized).expect("round trip");
    assert_eq!(decoded, result);
}

#[test]
fn invalid_serialized_resolution_fails_closed_without_identifier_leakage() {
    let secret_like_grant = "grant/api_token_sensitive";
    let wire = serde_json::json!({
        "context": resolution_context_wire(),
        "posture": "authorized",
        "availability": "available",
        "selected_grant_id": null,
        "reasons": ["active_grant_matched"],
        "evaluated_at": "2026-07-15T10:30:00Z"
    });
    let error = serde_json::from_value::<workflow_core::CapabilityResolution>(wire)
        .expect_err("missing grant");
    assert!(error
        .to_string()
        .contains("capability_authority.resolution.inconsistent"));
    assert!(!error.to_string().contains(secret_like_grant));

    let wire = serde_json::json!({
        "context": resolution_context_wire(),
        "posture": "authorized",
        "availability": "available",
        "selected_grant_id": secret_like_grant,
        "reasons": ["active_grant_matched"],
        "evaluated_at": "2026-07-15T10:30:00Z"
    });
    let error = serde_json::from_value::<workflow_core::CapabilityResolution>(wire)
        .expect_err("secret-like grant");
    assert!(error
        .to_string()
        .contains("capability_authority.secret_like_value"));
    assert!(!error.to_string().contains(secret_like_grant));
}

#[test]
fn invalid_not_authorized_wire_combinations_fail_closed() {
    let invalid_values = [
        serde_json::json!({
            "context": resolution_context_wire(),
            "posture": "not_authorized",
            "availability": "available",
            "selected_grant_id": null,
            "reasons": ["capability_not_connected"],
            "evaluated_at": "2026-07-15T10:30:00Z"
        }),
        serde_json::json!({
            "context": resolution_context_wire(),
            "posture": "not_authorized",
            "availability": "declared_not_connected",
            "selected_grant_id": null,
            "reasons": ["capability_unsupported"],
            "evaluated_at": "2026-07-15T10:30:00Z"
        }),
        serde_json::json!({
            "context": resolution_context_wire(),
            "posture": "not_authorized",
            "availability": null,
            "selected_grant_id": null,
            "reasons": ["no_matching_grant"],
            "evaluated_at": "2026-07-15T10:30:00Z"
        }),
        serde_json::json!({
            "context": resolution_context_wire(),
            "posture": "not_authorized",
            "availability": "available",
            "selected_grant_id": null,
            "reasons": ["no_matching_grant", "matching_grant_revoked"],
            "evaluated_at": "2026-07-15T10:30:00Z"
        }),
    ];

    for value in invalid_values {
        let error = serde_json::from_value::<workflow_core::CapabilityResolution>(value)
            .expect_err("inconsistent denial must fail closed");
        assert!(!error.to_string().contains("github/rcs2153/workflow-os"));
        assert!(!error.to_string().contains("agent/reviewer"));
    }
}

#[test]
fn missing_grant_projects_review_only_non_authority() {
    let grant = grant_without_requirements();
    let records = [available_record()];
    let resolution =
        resolve_capability_authority(&resolution_input(&grant, &records, &[])).expect("resolution");
    let request = CapabilityRequest::new(request_definition(&grant, resolution)).expect("request");

    let projection = project_capability_request_for_review(&request).expect("projection");

    assert_eq!(
        request.authority_posture(),
        CapabilityRequestAuthorityPosture::NotGranted
    );
    assert_eq!(
        projection.authority_posture(),
        CapabilityRequestAuthorityPosture::NotGranted
    );
    assert_eq!(
        projection.actions(),
        &[CapabilityRequestReviewAction::ReviewScopedGrant]
    );
    assert_eq!(projection.request_id().as_str(), "request/review-comment");
}

#[test]
fn already_authorized_work_cannot_become_a_capability_request() {
    let grants = [grant_without_requirements()];
    let records = [available_record()];
    let resolution = resolve_capability_authority(&resolution_input(&grants[0], &records, &grants))
        .expect("resolution");

    let error = CapabilityRequest::new(request_definition(&grants[0], resolution))
        .expect_err("authorized work must not create request");

    assert_eq!(
        error.code(),
        "capability_authority.request.already_authorized"
    );
}

#[test]
fn independent_prerequisites_project_deterministic_review_actions() {
    let grant = CapabilityGrant::new(definition()).expect("grant");
    let records = [available_record()];
    let grants = [grant.clone()];
    let resolution = resolve_capability_authority(&resolution_input(&grant, &records, &grants))
        .expect("resolution");
    let request = CapabilityRequest::new(request_definition(&grant, resolution)).expect("request");

    let projection = project_capability_request_for_review(&request).expect("projection");

    assert_eq!(
        projection.resolution_posture(),
        CapabilityResolutionPosture::RequiresIndependentEvaluation
    );
    assert_eq!(
        projection.actions(),
        &[
            CapabilityRequestReviewAction::EvaluatePolicy,
            CapabilityRequestReviewAction::EvaluateApproval,
            CapabilityRequestReviewAction::ValidateEvidence,
            CapabilityRequestReviewAction::ValidateChecks,
        ]
    );
}

#[test]
fn unavailable_capabilities_project_bounded_next_actions() {
    for (availability, expected) in [
        (
            CapabilityAvailability::DeclaredNotConnected,
            CapabilityRequestReviewAction::ReviewConnectorAvailability,
        ),
        (
            CapabilityAvailability::KnownUnsupported,
            CapabilityRequestReviewAction::ResolveUnsupportedCapability,
        ),
        (
            CapabilityAvailability::Unknown,
            CapabilityRequestReviewAction::EstablishAvailability,
        ),
    ] {
        let grant = grant_without_requirements();
        let records = [CapabilityAvailabilityRecord::new(
            grant.capability().clone(),
            grant.resource().clone(),
            availability,
            timestamp("2026-07-15T10:00:00Z"),
            RedactionMetadata::empty(),
        )
        .expect("record")];
        let grants = [grant.clone()];
        let resolution = resolve_capability_authority(&resolution_input(&grant, &records, &grants))
            .expect("resolution");
        let request =
            CapabilityRequest::new(request_definition(&grant, resolution)).expect("request");

        assert_eq!(
            project_capability_request_for_review(&request)
                .expect("projection")
                .actions(),
            &[expected]
        );
    }
}

#[test]
fn missing_inventory_projects_availability_review() {
    let grant = grant_without_requirements();
    let resolution =
        resolve_capability_authority(&resolution_input(&grant, &[], &[])).expect("resolution");
    let request = CapabilityRequest::new(request_definition(&grant, resolution)).expect("request");

    assert_eq!(
        project_capability_request_for_review(&request)
            .expect("projection")
            .actions(),
        &[CapabilityRequestReviewAction::EstablishAvailability]
    );
}

#[test]
fn rejected_grants_project_lifecycle_and_scope_review_actions() {
    let mut revoked = definition();
    revoked.lifecycle = CapabilityGrantLifecycle::Revoked;
    revoked.revocation_reference = Some("revocation/123".to_owned());
    revoked.requirements = CapabilityGrantRequirements::default();

    let mut insufficient = definition();
    insufficient.sensitivity_ceiling = WorkReportSensitivity::Public;
    insufficient.requirements = CapabilityGrantRequirements::default();

    for (grant, expected) in [
        (
            CapabilityGrant::new(revoked).expect("revoked"),
            CapabilityRequestReviewAction::ReviewGrantLifecycle,
        ),
        (
            CapabilityGrant::new(insufficient).expect("insufficient"),
            CapabilityRequestReviewAction::NarrowRequestedScope,
        ),
    ] {
        let records = [available_record()];
        let grants = [grant.clone()];
        let resolution = resolve_capability_authority(&resolution_input(&grant, &records, &grants))
            .expect("resolution");
        let request =
            CapabilityRequest::new(request_definition(&grant, resolution)).expect("request");

        assert_eq!(
            project_capability_request_for_review(&request)
                .expect("projection")
                .actions(),
            &[expected]
        );
    }
}

#[test]
fn request_lifecycle_and_sensitivity_fail_closed() {
    let grant = grant_without_requirements();
    let records = [available_record()];
    let resolution =
        resolve_capability_authority(&resolution_input(&grant, &records, &[])).expect("resolution");

    let mut unknown = request_definition(&grant, resolution.clone());
    unknown.requested_sensitivity = WorkReportSensitivity::Unknown;
    assert_eq!(
        CapabilityRequest::new(unknown)
            .expect_err("unknown sensitivity")
            .code(),
        "capability_authority.request.sensitivity_unknown"
    );

    let mut expired = request_definition(&grant, resolution.clone());
    expired.expires_at = expired.requested_at;
    assert_eq!(
        CapabilityRequest::new(expired)
            .expect_err("invalid expiry")
            .code(),
        "capability_authority.request.expiry_invalid"
    );

    let mut future_resolution = request_definition(&grant, resolution);
    future_resolution.requested_at = timestamp("2026-07-15T10:29:00Z");
    assert_eq!(
        CapabilityRequest::new(future_resolution)
            .expect_err("future resolution")
            .code(),
        "capability_authority.request.resolution_in_future"
    );
}

#[test]
fn request_rejects_resolution_from_different_identity_or_scope() {
    let grant = grant_without_requirements();
    let records = [available_record()];
    let resolution =
        resolve_capability_authority(&resolution_input(&grant, &records, &[])).expect("resolution");

    let mut different_actor = request_definition(&grant, resolution.clone());
    different_actor.requester = ActorId::new("agent/other").expect("actor");
    assert_eq!(
        CapabilityRequest::new(different_actor)
            .expect_err("actor mismatch")
            .code(),
        "capability_authority.request.resolution_context_mismatch"
    );

    let mut different_resource = request_definition(&grant, resolution.clone());
    different_resource.resource =
        CapabilityResourceScope::new(CapabilityResourceKind::Repository, "github/rcs2153/other")
            .expect("resource");
    assert_eq!(
        CapabilityRequest::new(different_resource)
            .expect_err("resource mismatch")
            .code(),
        "capability_authority.request.resolution_context_mismatch"
    );

    let mut different_run = request_definition(&grant, resolution.clone());
    different_run.run_id = WorkflowRunId::new("run/other").expect("run");
    assert_eq!(
        CapabilityRequest::new(different_run)
            .expect_err("run mismatch")
            .code(),
        "capability_authority.request.resolution_context_mismatch"
    );

    let mut different_sensitivity = request_definition(&grant, resolution);
    different_sensitivity.requested_sensitivity = WorkReportSensitivity::Confidential;
    assert_eq!(
        CapabilityRequest::new(different_sensitivity)
            .expect_err("sensitivity mismatch")
            .code(),
        "capability_authority.request.resolution_context_mismatch"
    );
}

#[test]
fn request_identifier_and_redaction_fail_without_leaking_values() {
    let secret = "token-sk-capability-request";
    let error = CapabilityRequestId::new(secret).expect_err("secret-like id");
    assert!(!error.message().contains(secret));

    let grant = grant_without_requirements();
    let records = [available_record()];
    let resolution =
        resolve_capability_authority(&resolution_input(&grant, &records, &[])).expect("resolution");
    let mut definition = request_definition(&grant, resolution);
    definition.redaction.redacted_fields = vec![secret.to_owned()];

    let error = CapabilityRequest::new(definition).expect_err("secret-like redaction");
    assert!(!error.message().contains(secret));
}

#[test]
fn request_and_projection_serde_round_trip_through_validated_boundaries() {
    let grant = grant_without_requirements();
    let records = [available_record()];
    let resolution =
        resolve_capability_authority(&resolution_input(&grant, &records, &[])).expect("resolution");
    let request = CapabilityRequest::new(request_definition(&grant, resolution)).expect("request");
    let projection = project_capability_request_for_review(&request).expect("projection");

    let request_wire = serde_json::to_string(&request).expect("serialize request");
    let projection_wire = serde_json::to_string(&projection).expect("serialize projection");
    assert_eq!(
        serde_json::from_str::<CapabilityRequest>(&request_wire).expect("request round trip"),
        request
    );
    assert_eq!(
        serde_json::from_str::<workflow_core::CapabilityRequestReviewProjection>(&projection_wire)
            .expect("projection round trip"),
        projection
    );
}

#[test]
fn request_and_projection_debug_are_redaction_safe() {
    let grant = grant_without_requirements();
    let records = [available_record()];
    let resolution =
        resolve_capability_authority(&resolution_input(&grant, &records, &[])).expect("resolution");
    let request = CapabilityRequest::new(request_definition(&grant, resolution)).expect("request");
    let projection = project_capability_request_for_review(&request).expect("projection");

    for debug in [format!("{request:?}"), format!("{projection:?}")] {
        for value in [
            "request/review-comment",
            "github.pull_request.comment.create",
            "github/rcs2153/workflow-os",
            "agent/reviewer",
            "user/maintainer",
        ] {
            assert!(!debug.contains(value));
        }
    }

    let serialized = serde_json::to_string(&projection).expect("serialize");
    for forbidden in ["provider_payload", "command_output", "authorization_header"] {
        assert!(!serialized.contains(forbidden));
    }
}

#[test]
fn invalid_projection_wire_fails_closed() {
    let value = serde_json::json!({
        "request_id": "request/review-comment",
        "authority_posture": "not_granted",
        "resolution_posture": "authorized",
        "resolution_reasons": ["active_grant_matched"],
        "actions": ["review_scoped_grant"],
        "review_steward": null,
        "review_by": "2026-07-15T11:31:00Z",
        "requested_sensitivity": "internal"
    });

    let error = serde_json::from_value::<workflow_core::CapabilityRequestReviewProjection>(value)
        .expect_err("authorized projection must fail");
    assert!(!error.to_string().contains("github/rcs2153/workflow-os"));
}

#[test]
fn projection_wire_rejects_actions_that_do_not_match_resolution_reasons() {
    let value = serde_json::json!({
        "request_id": "request/review-comment",
        "authority_posture": "not_granted",
        "resolution_posture": "not_authorized",
        "resolution_reasons": ["no_matching_grant"],
        "actions": ["evaluate_approval"],
        "review_steward": null,
        "review_by": "2026-07-15T11:31:00Z",
        "requested_sensitivity": "internal"
    });

    let error = serde_json::from_value::<workflow_core::CapabilityRequestReviewProjection>(value)
        .expect_err("mismatched action must fail");
    assert!(error
        .to_string()
        .contains("capability_authority.request_projection.actions_inconsistent"));
    assert!(!error.to_string().contains("github/rcs2153/workflow-os"));
}

#[test]
fn projection_wire_rejects_reasons_that_do_not_match_resolution_posture() {
    for (posture, reason, action) in [
        (
            "not_authorized",
            "active_grant_matched",
            "review_scoped_grant",
        ),
        (
            "requires_independent_evaluation",
            "no_matching_grant",
            "review_scoped_grant",
        ),
    ] {
        let value = serde_json::json!({
            "request_id": "request/review-comment",
            "authority_posture": "not_granted",
            "resolution_posture": posture,
            "resolution_reasons": [reason],
            "actions": [action],
            "review_steward": null,
            "review_by": "2026-07-15T11:31:00Z",
            "requested_sensitivity": "internal"
        });

        let error =
            serde_json::from_value::<workflow_core::CapabilityRequestReviewProjection>(value)
                .expect_err("posture and reason mismatch must fail");
        assert!(error
            .to_string()
            .contains("capability_authority.request_projection.reasons_inconsistent"));
    }
}
