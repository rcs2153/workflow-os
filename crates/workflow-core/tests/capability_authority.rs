#![allow(clippy::expect_used)]

//! Scoped capability-grant and availability core-model tests.

use workflow_core::{
    resolve_capability_authority, ActorId, ApprovalReferenceId, CapabilityAvailability,
    CapabilityAvailabilityRecord, CapabilityDelegationPosture, CapabilityGrant,
    CapabilityGrantDefinition, CapabilityGrantId, CapabilityGrantLifecycle,
    CapabilityGrantRequirements, CapabilityGrantScope, CapabilityReference,
    CapabilityResolutionInput, CapabilityResolutionPosture, CapabilityResolutionReason,
    CapabilityResourceKind, CapabilityResourceScope, EvidenceReferenceId, HarnessContractId,
    LocalCheckResultId, PolicyId, RedactionDisposition, RedactionFieldState, RedactionMetadata,
    StepId, Timestamp, WorkReportSensitivity, WorkflowId, WorkflowOsErrorKind, WorkflowRunId,
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
            "posture": "not_authorized",
            "availability": "available",
            "selected_grant_id": null,
            "reasons": ["capability_not_connected"],
            "evaluated_at": "2026-07-15T10:30:00Z"
        }),
        serde_json::json!({
            "posture": "not_authorized",
            "availability": "declared_not_connected",
            "selected_grant_id": null,
            "reasons": ["capability_unsupported"],
            "evaluated_at": "2026-07-15T10:30:00Z"
        }),
        serde_json::json!({
            "posture": "not_authorized",
            "availability": null,
            "selected_grant_id": null,
            "reasons": ["no_matching_grant"],
            "evaluated_at": "2026-07-15T10:30:00Z"
        }),
        serde_json::json!({
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
