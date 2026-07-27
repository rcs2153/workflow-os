#![allow(clippy::expect_used)]

//! Governed context-access model and pure projection-helper tests.

use workflow_core::{
    project_step_scoped_context, resolve_capability_authority, ActorId, ApprovalReferenceId,
    CapabilityAvailability, CapabilityAvailabilityRecord, CapabilityDelegationPosture,
    CapabilityGrant, CapabilityGrantDefinition, CapabilityGrantId, CapabilityGrantLifecycle,
    CapabilityGrantRequirements, CapabilityGrantScope, CapabilityResolution,
    CapabilityResolutionInput, CapabilityResourceKind, EvidenceReferenceId,
    GovernedContextAccessLevel, GovernedContextAvailability, GovernedContextProjection,
    GovernedContextProjectionCandidate, GovernedContextProjectionGapReason,
    GovernedContextProjectionInput, GovernedContextReference, GovernedContextReferenceKind,
    GovernedContextReferenceTarget, HarnessContractId, PolicyId, RedactionDisposition,
    RedactionFieldState, RedactionMetadata, SideEffectId, StepId, Timestamp, TypedHandoffId,
    ValidationReferenceId, WorkReportId, WorkReportSensitivity, WorkflowId, WorkflowOsErrorKind,
    WorkflowRunId,
};

fn timestamp(value: &str) -> Timestamp {
    Timestamp::parse_rfc3339(value).expect("valid timestamp")
}

fn actor() -> ActorId {
    ActorId::new("agent/context-reader").expect("actor")
}

fn workflow_id() -> WorkflowId {
    WorkflowId::new("workflow/context-review").expect("workflow")
}

fn run_id() -> WorkflowRunId {
    WorkflowRunId::new("run-context-review").expect("run")
}

fn step_id() -> StepId {
    StepId::new("review-context").expect("step")
}

fn harness_id() -> HarnessContractId {
    HarnessContractId::new("harness/context-reviewer").expect("harness")
}

fn redaction() -> RedactionMetadata {
    RedactionMetadata {
        redacted_fields: vec!["target_payload".to_owned()],
        field_states: vec![RedactionFieldState {
            field: "target_reference".to_owned(),
            disposition: RedactionDisposition::ReferenceOnly,
            reason: "stable reference only".to_owned(),
        }],
    }
}

fn reference(
    target: GovernedContextReferenceTarget,
    availability: GovernedContextAvailability,
    sensitivity: WorkReportSensitivity,
) -> GovernedContextReference {
    GovernedContextReference::new(target, sensitivity, availability, redaction())
        .expect("context reference")
}

#[derive(Clone, Copy)]
enum ResolutionMode {
    Authorized,
    MissingGrant,
    PolicyRequired,
}

fn resolution_for(
    reference: &GovernedContextReference,
    access_level: GovernedContextAccessLevel,
    mode: ResolutionMode,
) -> CapabilityResolution {
    let capability = access_level.required_capability().expect("capability");
    let resource = reference.capability_resource().expect("resource");
    let requirements = if matches!(mode, ResolutionMode::PolicyRequired) {
        CapabilityGrantRequirements::new(
            vec![PolicyId::new("policy/context-review").expect("policy")],
            vec![],
            vec![],
            vec![],
        )
        .expect("requirements")
    } else {
        CapabilityGrantRequirements::default()
    };
    let grant = CapabilityGrant::new(CapabilityGrantDefinition {
        grant_id: CapabilityGrantId::new("grant/context-review").expect("grant id"),
        subject: actor(),
        capability: capability.clone(),
        resource: resource.clone(),
        scope: CapabilityGrantScope::new(
            workflow_id(),
            Some(run_id()),
            Some(step_id()),
            Some(harness_id()),
        )
        .expect("scope"),
        issuer: ActorId::new("user/maintainer").expect("issuer"),
        issued_at: timestamp("2026-07-26T10:00:00Z"),
        expires_at: Some(timestamp("2026-07-26T12:00:00Z")),
        lifecycle: CapabilityGrantLifecycle::Active,
        revocation_reference: None,
        delegation: CapabilityDelegationPosture::Disabled,
        requirements,
        sensitivity_ceiling: WorkReportSensitivity::Secret,
        redaction: redaction(),
    })
    .expect("grant");
    let availability = CapabilityAvailabilityRecord::new(
        capability.clone(),
        resource.clone(),
        CapabilityAvailability::Available,
        timestamp("2026-07-26T10:00:00Z"),
        RedactionMetadata::empty(),
    )
    .expect("availability");
    let records = [availability];
    let grants = [grant];
    resolve_capability_authority(&CapabilityResolutionInput {
        capability: &capability,
        resource: &resource,
        actor: &actor(),
        workflow_id: &workflow_id(),
        run_id: &run_id(),
        step_id: &step_id(),
        harness_contract_id: Some(&harness_id()),
        requested_sensitivity: reference.sensitivity(),
        evaluated_at: timestamp("2026-07-26T10:30:00Z"),
        availability_records: &records,
        grants: if matches!(mode, ResolutionMode::MissingGrant) {
            &[]
        } else {
            &grants
        },
    })
    .expect("resolution")
}

fn candidate(
    target: GovernedContextReferenceTarget,
    access_level: GovernedContextAccessLevel,
    availability: GovernedContextAvailability,
    sensitivity: WorkReportSensitivity,
    mode: ResolutionMode,
) -> GovernedContextProjectionCandidate {
    let reference = reference(target, availability, sensitivity);
    let resolution = resolution_for(&reference, access_level, mode);
    GovernedContextProjectionCandidate::new(
        reference,
        timestamp("2026-07-26T10:15:00Z"),
        access_level,
        resolution,
    )
    .expect("candidate")
}

fn evidence_target(value: &str) -> GovernedContextReferenceTarget {
    GovernedContextReferenceTarget::EvidenceReference(
        EvidenceReferenceId::new(value).expect("evidence id"),
    )
}

fn project(
    candidates: &[GovernedContextProjectionCandidate],
    access_level: GovernedContextAccessLevel,
    sensitivity: WorkReportSensitivity,
) -> Result<GovernedContextProjection, workflow_core::WorkflowOsError> {
    project_step_scoped_context(&GovernedContextProjectionInput {
        actor: &actor(),
        workflow_id: &workflow_id(),
        run_id: &run_id(),
        step_id: &step_id(),
        harness_contract_id: Some(&harness_id()),
        projected_at: timestamp("2026-07-26T10:30:00Z"),
        maximum_allowed_sensitivity: sensitivity,
        requested_access_level: access_level,
        candidates,
        redaction: &redaction(),
    })
}

#[test]
fn reference_only_projection_returns_authorized_stable_identity_without_metadata() {
    let candidates = [candidate(
        evidence_target("evidence/context-review"),
        GovernedContextAccessLevel::ReferenceOnly,
        GovernedContextAvailability::Available,
        WorkReportSensitivity::Internal,
        ResolutionMode::Authorized,
    )];

    let projection = project(
        &candidates,
        GovernedContextAccessLevel::ReferenceOnly,
        WorkReportSensitivity::Confidential,
    )
    .expect("projection");

    assert_eq!(projection.candidates().len(), 1);
    assert_eq!(projection.entries().len(), 1);
    assert!(projection.gaps().is_empty());
    assert_eq!(
        projection.entries()[0].access_level(),
        GovernedContextAccessLevel::ReferenceOnly
    );
    assert!(projection.entries()[0].metadata().is_none());
}

#[test]
fn bounded_metadata_projection_exposes_only_fixed_typed_fields() {
    let candidates = [candidate(
        GovernedContextReferenceTarget::WorkReport(
            WorkReportId::new("work-report/context-review").expect("report id"),
        ),
        GovernedContextAccessLevel::BoundedMetadata,
        GovernedContextAvailability::Available,
        WorkReportSensitivity::Confidential,
        ResolutionMode::Authorized,
    )];

    let projection = project(
        &candidates,
        GovernedContextAccessLevel::BoundedMetadata,
        WorkReportSensitivity::Confidential,
    )
    .expect("projection");
    let metadata = projection.entries()[0].metadata().expect("metadata");

    assert_eq!(
        metadata.target_kind(),
        GovernedContextReferenceKind::WorkReport
    );
    assert_eq!(
        metadata.declared_sensitivity(),
        WorkReportSensitivity::Confidential
    );
    assert_eq!(
        metadata.availability_observed_at(),
        timestamp("2026-07-26T10:15:00Z")
    );
    let serialized = serde_json::to_string(&projection).expect("serialize");
    for forbidden in [
        "summary",
        "path",
        "url",
        "message",
        "event_payload",
        "snippet",
    ] {
        assert!(!serialized.contains(&format!("\"{forbidden}\":")));
    }
}

#[test]
fn all_first_slice_target_kinds_are_typed_and_representable() {
    let targets = [
        evidence_target("evidence/target"),
        GovernedContextReferenceTarget::WorkflowEvent(
            workflow_core::EventId::new("event/workflow").expect("event"),
        ),
        GovernedContextReferenceTarget::AuditEvent(
            workflow_core::EventId::new("event/audit").expect("event"),
        ),
        GovernedContextReferenceTarget::ValidationDiagnostic(
            ValidationReferenceId::new("validation/diagnostic").expect("validation"),
        ),
        GovernedContextReferenceTarget::ApprovalDecision(
            ApprovalReferenceId::new("approval/decision").expect("approval"),
        ),
        GovernedContextReferenceTarget::PolicyDecision(
            workflow_core::EventId::new("event/policy").expect("event"),
        ),
        GovernedContextReferenceTarget::SideEffect(
            SideEffectId::new("side-effect/context").expect("side effect"),
        ),
        GovernedContextReferenceTarget::TypedHandoff(
            TypedHandoffId::new("handoff/context").expect("handoff"),
        ),
        GovernedContextReferenceTarget::WorkReport(
            WorkReportId::new("work-report/context").expect("report"),
        ),
    ];
    let expected = [
        GovernedContextReferenceKind::EvidenceReference,
        GovernedContextReferenceKind::WorkflowEvent,
        GovernedContextReferenceKind::AuditEvent,
        GovernedContextReferenceKind::ValidationDiagnostic,
        GovernedContextReferenceKind::ApprovalDecision,
        GovernedContextReferenceKind::PolicyDecision,
        GovernedContextReferenceKind::SideEffect,
        GovernedContextReferenceKind::TypedHandoff,
        GovernedContextReferenceKind::WorkReport,
    ];

    assert_eq!(
        targets.map(|target| reference(
            target,
            GovernedContextAvailability::Available,
            WorkReportSensitivity::Internal
        )
        .kind()),
        expected
    );
}

#[test]
fn access_levels_map_to_exact_core_capabilities_and_context_resources() {
    assert_eq!(
        GovernedContextAccessLevel::ReferenceOnly
            .required_capability()
            .expect("capability")
            .as_str(),
        "context.reference.view"
    );
    assert_eq!(
        GovernedContextAccessLevel::BoundedMetadata
            .required_capability()
            .expect("capability")
            .as_str(),
        "context.metadata.view"
    );
    let reference = reference(
        evidence_target("evidence/canonical"),
        GovernedContextAvailability::Available,
        WorkReportSensitivity::Internal,
    );
    let resource = reference.capability_resource().expect("resource");
    assert_eq!(resource.kind(), CapabilityResourceKind::ContextReference);
    assert_eq!(
        resource.reference(),
        "evidence-reference/evidence/canonical"
    );
}

#[test]
fn unavailable_unknown_missing_and_independent_context_produce_bounded_gaps() {
    let candidates = [
        candidate(
            evidence_target("evidence/unavailable"),
            GovernedContextAccessLevel::ReferenceOnly,
            GovernedContextAvailability::Unavailable,
            WorkReportSensitivity::Internal,
            ResolutionMode::Authorized,
        ),
        candidate(
            evidence_target("evidence/unknown"),
            GovernedContextAccessLevel::ReferenceOnly,
            GovernedContextAvailability::Unknown,
            WorkReportSensitivity::Internal,
            ResolutionMode::Authorized,
        ),
        candidate(
            evidence_target("evidence/missing-grant"),
            GovernedContextAccessLevel::ReferenceOnly,
            GovernedContextAvailability::Available,
            WorkReportSensitivity::Internal,
            ResolutionMode::MissingGrant,
        ),
        candidate(
            evidence_target("evidence/policy"),
            GovernedContextAccessLevel::ReferenceOnly,
            GovernedContextAvailability::Available,
            WorkReportSensitivity::Internal,
            ResolutionMode::PolicyRequired,
        ),
        candidate(
            evidence_target("evidence/sensitive"),
            GovernedContextAccessLevel::ReferenceOnly,
            GovernedContextAvailability::Available,
            WorkReportSensitivity::Secret,
            ResolutionMode::Authorized,
        ),
    ];

    let projection = project(
        &candidates,
        GovernedContextAccessLevel::ReferenceOnly,
        WorkReportSensitivity::Confidential,
    )
    .expect("projection");
    let reasons = projection
        .gaps()
        .iter()
        .map(workflow_core::GovernedContextProjectionGap::reason)
        .collect::<Vec<_>>();

    assert!(projection.entries().is_empty());
    for expected in [
        GovernedContextProjectionGapReason::Unavailable,
        GovernedContextProjectionGapReason::UnknownAvailability,
        GovernedContextProjectionGapReason::NoMatchingAuthority,
        GovernedContextProjectionGapReason::IndependentPolicyEvaluationRequired,
        GovernedContextProjectionGapReason::SensitivityCeilingExceeded,
    ] {
        assert!(reasons.contains(&expected));
    }
}

#[test]
fn candidates_are_sorted_deterministically_and_duplicates_fail_closed() {
    let first = candidate(
        evidence_target("evidence/a"),
        GovernedContextAccessLevel::ReferenceOnly,
        GovernedContextAvailability::Available,
        WorkReportSensitivity::Internal,
        ResolutionMode::Authorized,
    );
    let second = candidate(
        evidence_target("evidence/b"),
        GovernedContextAccessLevel::ReferenceOnly,
        GovernedContextAvailability::Available,
        WorkReportSensitivity::Internal,
        ResolutionMode::Authorized,
    );
    let projection = project(
        &[second, first.clone()],
        GovernedContextAccessLevel::ReferenceOnly,
        WorkReportSensitivity::Internal,
    )
    .expect("sorted projection");
    let serialized = serde_json::to_value(&projection).expect("serialize");
    assert_eq!(
        serialized["candidates"][0]["reference"]["target"]["id"],
        "evidence/a"
    );

    let error = project(
        &[first.clone(), first],
        GovernedContextAccessLevel::ReferenceOnly,
        WorkReportSensitivity::Internal,
    )
    .expect_err("duplicate");
    assert_eq!(
        error.code(),
        "governed_context.projection.duplicate_candidate"
    );
}

#[test]
fn wrong_context_or_access_authority_fails_closed() {
    let wrong_context_candidate = candidate(
        evidence_target("evidence/wrong-context"),
        GovernedContextAccessLevel::ReferenceOnly,
        GovernedContextAvailability::Available,
        WorkReportSensitivity::Internal,
        ResolutionMode::Authorized,
    );
    let wrong_actor = ActorId::new("agent/other").expect("actor");
    let error = project_step_scoped_context(&GovernedContextProjectionInput {
        actor: &wrong_actor,
        workflow_id: &workflow_id(),
        run_id: &run_id(),
        step_id: &step_id(),
        harness_contract_id: Some(&harness_id()),
        projected_at: timestamp("2026-07-26T10:30:00Z"),
        maximum_allowed_sensitivity: WorkReportSensitivity::Internal,
        requested_access_level: GovernedContextAccessLevel::ReferenceOnly,
        candidates: &[wrong_context_candidate],
        redaction: &redaction(),
    })
    .expect_err("wrong context");
    assert_eq!(error.code(), "governed_context.projection.context_mismatch");

    let candidate = candidate(
        evidence_target("evidence/access"),
        GovernedContextAccessLevel::ReferenceOnly,
        GovernedContextAvailability::Available,
        WorkReportSensitivity::Internal,
        ResolutionMode::Authorized,
    );
    let error = project(
        &[candidate],
        GovernedContextAccessLevel::BoundedMetadata,
        WorkReportSensitivity::Internal,
    )
    .expect_err("wrong access");
    assert_eq!(
        error.code(),
        "governed_context.projection.access_level_mismatch"
    );
}

#[test]
fn projection_round_trips_and_wire_omission_or_reordering_fails_closed() {
    let candidates = [
        candidate(
            evidence_target("evidence/a"),
            GovernedContextAccessLevel::ReferenceOnly,
            GovernedContextAvailability::Available,
            WorkReportSensitivity::Internal,
            ResolutionMode::Authorized,
        ),
        candidate(
            evidence_target("evidence/b"),
            GovernedContextAccessLevel::ReferenceOnly,
            GovernedContextAvailability::Unavailable,
            WorkReportSensitivity::Internal,
            ResolutionMode::Authorized,
        ),
    ];
    let projection = project(
        &candidates,
        GovernedContextAccessLevel::ReferenceOnly,
        WorkReportSensitivity::Internal,
    )
    .expect("projection");
    let serialized = serde_json::to_string(&projection).expect("serialize");
    let decoded: GovernedContextProjection = serde_json::from_str(&serialized).expect("round trip");
    assert_eq!(decoded, projection);

    let mut omitted = serde_json::to_value(&projection).expect("wire");
    omitted["entries"] = serde_json::json!([]);
    let error = serde_json::from_value::<GovernedContextProjection>(omitted).expect_err("omission");
    assert!(error
        .to_string()
        .contains("governed_context.projection.derivation_inconsistent"));

    let mut reordered = serde_json::to_value(&projection).expect("wire");
    reordered["candidates"]
        .as_array_mut()
        .expect("array")
        .reverse();
    let error =
        serde_json::from_value::<GovernedContextProjection>(reordered).expect_err("reorder");
    assert!(error
        .to_string()
        .contains("governed_context.projection.candidates_unordered"));
}

#[test]
fn standalone_entry_rejects_unavailable_target_and_wire_errors_do_not_echo_values() {
    let candidates = [candidate(
        evidence_target("evidence/wire-safety"),
        GovernedContextAccessLevel::ReferenceOnly,
        GovernedContextAvailability::Available,
        WorkReportSensitivity::Internal,
        ResolutionMode::Authorized,
    )];
    let projection = project(
        &candidates,
        GovernedContextAccessLevel::ReferenceOnly,
        WorkReportSensitivity::Internal,
    )
    .expect("projection");
    let wire = serde_json::to_value(&projection).expect("wire");

    let mut unavailable_entry = wire["entries"][0].clone();
    unavailable_entry["reference"]["availability"] = serde_json::json!("unavailable");
    let error =
        serde_json::from_value::<workflow_core::GovernedContextProjectionEntry>(unavailable_entry)
            .expect_err("unavailable entry");
    assert!(error
        .to_string()
        .contains("governed_context.entry.authority_mismatch"));

    for path in [
        &["requested_access_level"][..],
        &["candidates", "0", "source_resolution", "posture"][..],
        &["candidates", "0", "reference", "target", "kind"][..],
        &[
            "candidates",
            "0",
            "reference",
            "redaction",
            "field_states",
            "0",
            "disposition",
        ][..],
    ] {
        let secret_value = "api_token_super_sensitive";
        let mut forged = wire.clone();
        let mut current = &mut forged;
        for segment in path {
            if let Ok(index) = segment.parse::<usize>() {
                current = &mut current[index];
            } else {
                current = &mut current[*segment];
            }
        }
        *current = serde_json::json!(secret_value);
        let error =
            serde_json::from_value::<GovernedContextProjection>(forged).expect_err("invalid wire");
        assert!(!error.to_string().contains(secret_value));
    }
}

#[test]
fn secret_like_metadata_and_ids_are_rejected_without_leakage() {
    let secret_id = "api_token_super_sensitive";
    let target = evidence_target(secret_id);
    let error = GovernedContextReference::new(
        target,
        WorkReportSensitivity::Internal,
        GovernedContextAvailability::Available,
        RedactionMetadata::empty(),
    )
    .expect_err("secret id");
    assert_eq!(error.kind(), WorkflowOsErrorKind::Validation);
    assert_eq!(error.code(), "governed_context.secret_like_value");
    assert!(!error.to_string().contains(secret_id));

    let secret_reason = "bearer-super-sensitive";
    let error = GovernedContextReference::new(
        evidence_target("evidence/safe"),
        WorkReportSensitivity::Internal,
        GovernedContextAvailability::Available,
        RedactionMetadata {
            redacted_fields: vec![],
            field_states: vec![RedactionFieldState {
                field: "target".to_owned(),
                disposition: RedactionDisposition::Redacted,
                reason: secret_reason.to_owned(),
            }],
        },
    )
    .expect_err("secret reason");
    assert_eq!(error.code(), "governed_context.secret_like_value");
    assert!(!error.to_string().contains(secret_reason));
}

#[test]
fn debug_and_serialization_never_copy_forbidden_payloads() {
    let secretish_but_valid_id = "evidence/private-context";
    let candidates = [candidate(
        evidence_target(secretish_but_valid_id),
        GovernedContextAccessLevel::ReferenceOnly,
        GovernedContextAvailability::Available,
        WorkReportSensitivity::Confidential,
        ResolutionMode::Authorized,
    )];
    let projection = project(
        &candidates,
        GovernedContextAccessLevel::ReferenceOnly,
        WorkReportSensitivity::Confidential,
    )
    .expect("projection");
    let debug = format!("{projection:?}");
    assert!(!debug.contains(secretish_but_valid_id));
    assert!(!debug.contains("target_payload"));
    assert!(!debug.contains("stable reference only"));

    let serialized = serde_json::to_string(&projection).expect("serialize");
    for forbidden_field in [
        "provider_payload",
        "command_output",
        "raw_spec_contents",
        "event_payload",
        "environment_values",
        "credentials",
        "authorization_header",
        "private_key",
        "source_contents",
    ] {
        assert!(!serialized.contains(&format!("\"{forbidden_field}\":")));
    }
}
