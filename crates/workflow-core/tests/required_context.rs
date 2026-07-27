#![allow(clippy::expect_used)]

//! Required-context contract consumption model and pure-helper tests.

use workflow_core::{
    consume_required_context, project_step_scoped_context, resolve_capability_authority, ActorId,
    CapabilityAvailability, CapabilityAvailabilityRecord, CapabilityDelegationPosture,
    CapabilityGrant, CapabilityGrantDefinition, CapabilityGrantId, CapabilityGrantLifecycle,
    CapabilityGrantRequirements, CapabilityGrantScope, CapabilityResolution,
    CapabilityResolutionInput, EvidenceReferenceId, GovernedContextAccessLevel,
    GovernedContextAvailability, GovernedContextProjection, GovernedContextProjectionCandidate,
    GovernedContextProjectionInput, GovernedContextReference, GovernedContextReferenceTarget,
    HarnessContractId, HarnessContractVersion, RedactionMetadata,
    RequiredContextConsumptionContext, RequiredContextConsumptionInput,
    RequiredContextConsumptionPosture, RequiredContextContractBinding, RequiredContextGapReason,
    RequiredContextObligation, RequiredContextRequirement, RequiredContextRequirementId, StepId,
    Timestamp, WorkReportId, WorkReportSensitivity, WorkflowId, WorkflowOsErrorKind, WorkflowRunId,
};

fn timestamp(value: &str) -> Timestamp {
    Timestamp::parse_rfc3339(value).expect("valid timestamp")
}

fn actor() -> ActorId {
    ActorId::new("agent/context-consumer").expect("actor")
}

fn workflow_id() -> WorkflowId {
    WorkflowId::new("workflow/context-consumption").expect("workflow")
}

fn run_id() -> WorkflowRunId {
    WorkflowRunId::new("run-context-consumption").expect("run")
}

fn step_id() -> StepId {
    StepId::new("consume-context").expect("step")
}

fn harness_id() -> HarnessContractId {
    HarnessContractId::new("harness/context-consumer").expect("harness")
}

fn harness_version() -> HarnessContractVersion {
    HarnessContractVersion::new("v1").expect("version")
}

fn evidence_target(value: &str) -> GovernedContextReferenceTarget {
    GovernedContextReferenceTarget::EvidenceReference(
        EvidenceReferenceId::new(value).expect("evidence id"),
    )
}

fn report_target(value: &str) -> GovernedContextReferenceTarget {
    GovernedContextReferenceTarget::WorkReport(WorkReportId::new(value).expect("report id"))
}

fn reference(
    target: GovernedContextReferenceTarget,
    availability: GovernedContextAvailability,
    sensitivity: WorkReportSensitivity,
) -> GovernedContextReference {
    GovernedContextReference::new(
        target,
        sensitivity,
        availability,
        RedactionMetadata::empty(),
    )
    .expect("reference")
}

#[derive(Clone, Copy)]
enum ResolutionMode {
    Authorized,
    MissingGrant,
}

fn resolution_for(
    reference: &GovernedContextReference,
    access_level: GovernedContextAccessLevel,
    mode: ResolutionMode,
) -> CapabilityResolution {
    let capability = access_level.required_capability().expect("capability");
    let resource = reference.capability_resource().expect("resource");
    let grant = CapabilityGrant::new(CapabilityGrantDefinition {
        grant_id: CapabilityGrantId::new(format!(
            "grant/{}",
            match access_level {
                GovernedContextAccessLevel::ReferenceOnly => "reference",
                GovernedContextAccessLevel::BoundedMetadata => "metadata",
            }
        ))
        .expect("grant"),
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
        requirements: CapabilityGrantRequirements::default(),
        sensitivity_ceiling: WorkReportSensitivity::Secret,
        redaction: RedactionMetadata::empty(),
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
    let availability_records = [availability];
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
        availability_records: &availability_records,
        grants: if matches!(mode, ResolutionMode::Authorized) {
            &grants
        } else {
            &[]
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

fn project(
    candidates: &[GovernedContextProjectionCandidate],
    access_level: GovernedContextAccessLevel,
    sensitivity: WorkReportSensitivity,
) -> GovernedContextProjection {
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
        redaction: &RedactionMetadata::empty(),
    })
    .expect("projection")
}

fn requirement(
    id: &str,
    target: GovernedContextReferenceTarget,
    access_level: GovernedContextAccessLevel,
    obligation: RequiredContextObligation,
    sensitivity: WorkReportSensitivity,
) -> RequiredContextRequirement {
    RequiredContextRequirement::new(
        RequiredContextRequirementId::new(id).expect("requirement id"),
        target,
        access_level,
        obligation,
        sensitivity,
    )
    .expect("requirement")
}

fn contract(requirements: Vec<RequiredContextRequirement>) -> RequiredContextContractBinding {
    RequiredContextContractBinding::new(harness_id(), harness_version(), requirements)
        .expect("contract")
}

fn consumption_context() -> RequiredContextConsumptionContext {
    RequiredContextConsumptionContext::new(
        actor(),
        workflow_id(),
        run_id(),
        step_id(),
        harness_id(),
        timestamp("2026-07-26T10:30:00Z"),
    )
}

#[test]
fn exact_required_reference_is_satisfied_without_payload_access() {
    let target = evidence_target("evidence/required");
    let binding = contract(vec![requirement(
        "required/evidence",
        target.clone(),
        GovernedContextAccessLevel::ReferenceOnly,
        RequiredContextObligation::Required,
        WorkReportSensitivity::Confidential,
    )]);
    let projection = project(
        &[candidate(
            target,
            GovernedContextAccessLevel::ReferenceOnly,
            GovernedContextAvailability::Available,
            WorkReportSensitivity::Internal,
            ResolutionMode::Authorized,
        )],
        GovernedContextAccessLevel::ReferenceOnly,
        WorkReportSensitivity::Confidential,
    );

    let result = consume_required_context(&RequiredContextConsumptionInput {
        contract: &binding,
        context: &consumption_context(),
        projections: &[projection],
    })
    .expect("consumption");

    assert_eq!(
        result.posture(),
        RequiredContextConsumptionPosture::Satisfied
    );
    assert_eq!(result.satisfactions().len(), 1);
    assert!(result.gaps().is_empty());
    assert_eq!(result.context(), &consumption_context());
}

#[test]
fn projections_must_match_independently_declared_execution_context() {
    let target = evidence_target("evidence/context-bound");
    let binding = contract(vec![requirement(
        "required/context-bound",
        target.clone(),
        GovernedContextAccessLevel::ReferenceOnly,
        RequiredContextObligation::Required,
        WorkReportSensitivity::Confidential,
    )]);
    let projection = project(
        &[candidate(
            target,
            GovernedContextAccessLevel::ReferenceOnly,
            GovernedContextAvailability::Available,
            WorkReportSensitivity::Internal,
            ResolutionMode::Authorized,
        )],
        GovernedContextAccessLevel::ReferenceOnly,
        WorkReportSensitivity::Confidential,
    );
    let mismatches = [
        RequiredContextConsumptionContext::new(
            ActorId::new("agent/other").expect("actor"),
            workflow_id(),
            run_id(),
            step_id(),
            harness_id(),
            timestamp("2026-07-26T10:30:00Z"),
        ),
        RequiredContextConsumptionContext::new(
            actor(),
            WorkflowId::new("workflow/other").expect("workflow"),
            run_id(),
            step_id(),
            harness_id(),
            timestamp("2026-07-26T10:30:00Z"),
        ),
        RequiredContextConsumptionContext::new(
            actor(),
            workflow_id(),
            WorkflowRunId::new("run-other").expect("run"),
            step_id(),
            harness_id(),
            timestamp("2026-07-26T10:30:00Z"),
        ),
        RequiredContextConsumptionContext::new(
            actor(),
            workflow_id(),
            run_id(),
            StepId::new("other-step").expect("step"),
            harness_id(),
            timestamp("2026-07-26T10:30:00Z"),
        ),
        RequiredContextConsumptionContext::new(
            actor(),
            workflow_id(),
            run_id(),
            step_id(),
            harness_id(),
            timestamp("2026-07-26T10:31:00Z"),
        ),
    ];

    for context in mismatches {
        let error = consume_required_context(&RequiredContextConsumptionInput {
            contract: &binding,
            context: &context,
            projections: std::slice::from_ref(&projection),
        })
        .expect_err("context mismatch");
        assert_eq!(
            error.code(),
            "required_context.consumption.projection_context_mismatch"
        );
    }

    let wrong_harness = RequiredContextConsumptionContext::new(
        actor(),
        workflow_id(),
        run_id(),
        step_id(),
        HarnessContractId::new("harness/other").expect("harness"),
        timestamp("2026-07-26T10:30:00Z"),
    );
    let error = consume_required_context(&RequiredContextConsumptionInput {
        contract: &binding,
        context: &wrong_harness,
        projections: &[projection],
    })
    .expect_err("harness mismatch");
    assert_eq!(
        error.code(),
        "required_context.consumption.contract_context_mismatch"
    );
}

#[test]
fn mixed_exact_access_levels_are_consumed_from_separate_projections() {
    let evidence = evidence_target("evidence/mixed");
    let report = report_target("report/mixed");
    let binding = contract(vec![
        requirement(
            "required/evidence",
            evidence.clone(),
            GovernedContextAccessLevel::ReferenceOnly,
            RequiredContextObligation::Required,
            WorkReportSensitivity::Confidential,
        ),
        requirement(
            "required/report",
            report.clone(),
            GovernedContextAccessLevel::BoundedMetadata,
            RequiredContextObligation::Required,
            WorkReportSensitivity::Confidential,
        ),
    ]);
    let reference_projection = project(
        &[candidate(
            evidence,
            GovernedContextAccessLevel::ReferenceOnly,
            GovernedContextAvailability::Available,
            WorkReportSensitivity::Internal,
            ResolutionMode::Authorized,
        )],
        GovernedContextAccessLevel::ReferenceOnly,
        WorkReportSensitivity::Confidential,
    );
    let metadata_projection = project(
        &[candidate(
            report,
            GovernedContextAccessLevel::BoundedMetadata,
            GovernedContextAvailability::Available,
            WorkReportSensitivity::Internal,
            ResolutionMode::Authorized,
        )],
        GovernedContextAccessLevel::BoundedMetadata,
        WorkReportSensitivity::Confidential,
    );

    let result = consume_required_context(&RequiredContextConsumptionInput {
        contract: &binding,
        context: &consumption_context(),
        projections: &[metadata_projection, reference_projection],
    })
    .expect("consumption");

    assert_eq!(result.satisfactions().len(), 2);
    assert_eq!(
        result.projections()[0].requested_access_level(),
        GovernedContextAccessLevel::ReferenceOnly
    );
}

#[test]
fn unavailable_required_context_blocks_consumption() {
    let target = evidence_target("evidence/unavailable");
    let binding = contract(vec![requirement(
        "required/unavailable",
        target.clone(),
        GovernedContextAccessLevel::ReferenceOnly,
        RequiredContextObligation::Required,
        WorkReportSensitivity::Confidential,
    )]);
    let projection = project(
        &[candidate(
            target,
            GovernedContextAccessLevel::ReferenceOnly,
            GovernedContextAvailability::Unavailable,
            WorkReportSensitivity::Internal,
            ResolutionMode::Authorized,
        )],
        GovernedContextAccessLevel::ReferenceOnly,
        WorkReportSensitivity::Confidential,
    );

    let result = consume_required_context(&RequiredContextConsumptionInput {
        contract: &binding,
        context: &consumption_context(),
        projections: &[projection],
    })
    .expect("bounded result");

    assert_eq!(result.posture(), RequiredContextConsumptionPosture::Blocked);
    assert_eq!(
        result.gaps()[0].reason(),
        RequiredContextGapReason::Unavailable
    );
}

#[test]
fn unavailable_optional_context_is_explicit_but_non_blocking() {
    let target = evidence_target("evidence/optional");
    let binding = contract(vec![requirement(
        "optional/evidence",
        target.clone(),
        GovernedContextAccessLevel::ReferenceOnly,
        RequiredContextObligation::Optional,
        WorkReportSensitivity::Confidential,
    )]);
    let projection = project(
        &[candidate(
            target,
            GovernedContextAccessLevel::ReferenceOnly,
            GovernedContextAvailability::Unavailable,
            WorkReportSensitivity::Internal,
            ResolutionMode::Authorized,
        )],
        GovernedContextAccessLevel::ReferenceOnly,
        WorkReportSensitivity::Confidential,
    );

    let result = consume_required_context(&RequiredContextConsumptionInput {
        contract: &binding,
        context: &consumption_context(),
        projections: &[projection],
    })
    .expect("bounded result");

    assert_eq!(
        result.posture(),
        RequiredContextConsumptionPosture::Satisfied
    );
    assert_eq!(
        result.gaps()[0].obligation(),
        RequiredContextObligation::Optional
    );
}

#[test]
fn declaration_does_not_create_missing_authority() {
    let target = evidence_target("evidence/no-authority");
    let binding = contract(vec![requirement(
        "required/no-authority",
        target.clone(),
        GovernedContextAccessLevel::ReferenceOnly,
        RequiredContextObligation::Required,
        WorkReportSensitivity::Confidential,
    )]);
    let projection = project(
        &[candidate(
            target,
            GovernedContextAccessLevel::ReferenceOnly,
            GovernedContextAvailability::Available,
            WorkReportSensitivity::Internal,
            ResolutionMode::MissingGrant,
        )],
        GovernedContextAccessLevel::ReferenceOnly,
        WorkReportSensitivity::Confidential,
    );

    let result = consume_required_context(&RequiredContextConsumptionInput {
        contract: &binding,
        context: &consumption_context(),
        projections: &[projection],
    })
    .expect("bounded result");

    assert_eq!(result.posture(), RequiredContextConsumptionPosture::Blocked);
    assert_eq!(
        result.gaps()[0].reason(),
        RequiredContextGapReason::NoMatchingAuthority
    );
}

#[test]
fn undeclared_projected_context_fails_closed() {
    let declared = evidence_target("evidence/declared");
    let extra = evidence_target("evidence/extra");
    let binding = contract(vec![requirement(
        "required/declared",
        declared.clone(),
        GovernedContextAccessLevel::ReferenceOnly,
        RequiredContextObligation::Required,
        WorkReportSensitivity::Confidential,
    )]);
    let projection = project(
        &[
            candidate(
                declared,
                GovernedContextAccessLevel::ReferenceOnly,
                GovernedContextAvailability::Available,
                WorkReportSensitivity::Internal,
                ResolutionMode::Authorized,
            ),
            candidate(
                extra,
                GovernedContextAccessLevel::ReferenceOnly,
                GovernedContextAvailability::Available,
                WorkReportSensitivity::Internal,
                ResolutionMode::Authorized,
            ),
        ],
        GovernedContextAccessLevel::ReferenceOnly,
        WorkReportSensitivity::Confidential,
    );

    let error = consume_required_context(&RequiredContextConsumptionInput {
        contract: &binding,
        context: &consumption_context(),
        projections: &[projection],
    })
    .expect_err("extra context must fail");

    assert_eq!(
        error.code(),
        "required_context.consumption.target_set_mismatch"
    );
}

#[test]
fn broader_access_does_not_satisfy_reference_only_requirement() {
    let target = evidence_target("evidence/exact-access");
    let binding = contract(vec![requirement(
        "required/exact-access",
        target.clone(),
        GovernedContextAccessLevel::ReferenceOnly,
        RequiredContextObligation::Required,
        WorkReportSensitivity::Confidential,
    )]);
    let projection = project(
        &[candidate(
            target,
            GovernedContextAccessLevel::BoundedMetadata,
            GovernedContextAvailability::Available,
            WorkReportSensitivity::Internal,
            ResolutionMode::Authorized,
        )],
        GovernedContextAccessLevel::BoundedMetadata,
        WorkReportSensitivity::Confidential,
    );

    let error = consume_required_context(&RequiredContextConsumptionInput {
        contract: &binding,
        context: &consumption_context(),
        projections: &[projection],
    })
    .expect_err("access mismatch");

    assert_eq!(
        error.code(),
        "required_context.consumption.target_set_mismatch"
    );
}

#[test]
fn requirement_sensitivity_ceiling_blocks_even_authorized_projection() {
    let target = evidence_target("evidence/sensitive");
    let binding = contract(vec![requirement(
        "required/sensitive",
        target.clone(),
        GovernedContextAccessLevel::ReferenceOnly,
        RequiredContextObligation::Required,
        WorkReportSensitivity::Internal,
    )]);
    let projection = project(
        &[candidate(
            target,
            GovernedContextAccessLevel::ReferenceOnly,
            GovernedContextAvailability::Available,
            WorkReportSensitivity::Confidential,
            ResolutionMode::Authorized,
        )],
        GovernedContextAccessLevel::ReferenceOnly,
        WorkReportSensitivity::Secret,
    );

    let result = consume_required_context(&RequiredContextConsumptionInput {
        contract: &binding,
        context: &consumption_context(),
        projections: &[projection],
    })
    .expect("bounded result");

    assert_eq!(result.posture(), RequiredContextConsumptionPosture::Blocked);
    assert_eq!(
        result.gaps()[0].reason(),
        RequiredContextGapReason::RequirementSensitivityCeilingExceeded
    );
}

#[test]
fn contract_and_result_serde_fail_closed_on_tampering() {
    let target = evidence_target("evidence/serde");
    let binding = contract(vec![requirement(
        "required/serde",
        target.clone(),
        GovernedContextAccessLevel::ReferenceOnly,
        RequiredContextObligation::Required,
        WorkReportSensitivity::Confidential,
    )]);
    let projection = project(
        &[candidate(
            target,
            GovernedContextAccessLevel::ReferenceOnly,
            GovernedContextAvailability::Available,
            WorkReportSensitivity::Internal,
            ResolutionMode::Authorized,
        )],
        GovernedContextAccessLevel::ReferenceOnly,
        WorkReportSensitivity::Confidential,
    );
    let result = consume_required_context(&RequiredContextConsumptionInput {
        contract: &binding,
        context: &consumption_context(),
        projections: &[projection],
    })
    .expect("result");

    let serialized = serde_json::to_string(&result).expect("serialize");
    let round_trip =
        serde_json::from_str::<workflow_core::RequiredContextConsumptionResult>(&serialized)
            .expect("round trip");
    assert_eq!(round_trip, result);

    let mut forged = serde_json::to_value(&result).expect("value");
    forged["contract"]["content_hash"] = serde_json::Value::String("a".repeat(64));
    let error = serde_json::from_value::<workflow_core::RequiredContextConsumptionResult>(forged)
        .expect_err("hash tamper");
    assert!(error
        .to_string()
        .contains("required_context.contract.content_hash_mismatch"));

    let mut omitted = serde_json::to_value(&result).expect("value");
    omitted["satisfactions"] = serde_json::json!([]);
    let error = serde_json::from_value::<workflow_core::RequiredContextConsumptionResult>(omitted)
        .expect_err("derived omission");
    assert!(error
        .to_string()
        .contains("required_context.consumption.derivation_inconsistent"));

    let substituted_run = "run-substituted";
    let mut forged_context = serde_json::to_value(&result).expect("value");
    forged_context["context"]["run_id"] = serde_json::Value::String(substituted_run.to_owned());
    let error =
        serde_json::from_value::<workflow_core::RequiredContextConsumptionResult>(forged_context)
            .expect_err("context substitution");
    assert!(!error.to_string().contains(substituted_run));
}

#[test]
fn invalid_wire_and_secret_like_ids_fail_without_value_leakage() {
    let secret = "token-like-requirement";
    let error = RequiredContextRequirementId::new(secret).expect_err("secret-like id");
    assert_eq!(error.kind(), WorkflowOsErrorKind::Validation);
    assert!(!error.to_string().contains(secret));

    let target = evidence_target("evidence/wire");
    let binding = contract(vec![requirement(
        "required/wire",
        target,
        GovernedContextAccessLevel::ReferenceOnly,
        RequiredContextObligation::Required,
        WorkReportSensitivity::Confidential,
    )]);
    let mut forged = serde_json::to_value(&binding).expect("value");
    let forged_value = "forged-secret-like-obligation";
    forged["requirements"][0]["obligation"] = serde_json::Value::String(forged_value.to_owned());
    let error = serde_json::from_value::<RequiredContextContractBinding>(forged)
        .expect_err("unknown obligation");
    assert!(!error.to_string().contains(forged_value));
}

#[test]
fn debug_and_serialization_are_payload_free() {
    let target_id = "evidence/private-context";
    let requirement_id = "required/private-context";
    let binding = contract(vec![requirement(
        requirement_id,
        evidence_target(target_id),
        GovernedContextAccessLevel::ReferenceOnly,
        RequiredContextObligation::Required,
        WorkReportSensitivity::Confidential,
    )]);

    let debug = format!("{binding:?}");
    assert!(!debug.contains(target_id));
    assert!(!debug.contains(requirement_id));
    assert!(!debug.contains(harness_id().as_str()));
    assert!(!debug.contains(harness_version().as_str()));
    assert!(!debug.contains(binding.content_hash().as_str()));

    let serialized = serde_json::to_string(&binding).expect("serialize");
    for forbidden in [
        "raw_provider_payload",
        "command_output",
        "raw_spec_contents",
        "parser_payload",
        "environment_value",
        "authorization_header",
        "private_key",
        "credential",
    ] {
        assert!(!serialized.contains(forbidden));
    }
}
