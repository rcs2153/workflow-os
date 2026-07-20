#![allow(clippy::expect_used, clippy::unwrap_used)]

//! Focused model-only tests for independent local check attestation candidates.

use serde_json::{json, Value};
use workflow_core::{
    IdempotencyKey, ImmutableRunBundleBinding, LocalCheckAttestationAlgorithm,
    LocalCheckAttestationAssurance, LocalCheckAttestationBinding,
    LocalCheckAttestationBindingDefinition, LocalCheckAttestationExitCodePosture,
    LocalCheckAttestationFreshnessPolicy, LocalCheckAttestationId,
    LocalCheckAttestationRequirement, LocalCheckAttestationRequirementDefinition,
    LocalCheckAttestationSource, LocalCheckAttestationVerificationPosture, LocalCheckCommandId,
    LocalCheckResultId, LocalCheckResultStatus, SkillInvocationId, SpecContentHash, StepId,
    Timestamp, WorkflowId, WorkflowRunId,
};

fn hash(seed: &str) -> SpecContentHash {
    SpecContentHash::from_text(seed)
}

fn bundle() -> ImmutableRunBundleBinding {
    serde_json::from_value(json!({
        "bundle_id": "bundle/test",
        "bundle_version": "v1",
        "root_hash": hash("bundle-root").as_str(),
    }))
    .expect("bundle binding should deserialize")
}

fn candidate_definition(
    assurance: LocalCheckAttestationAssurance,
    source: LocalCheckAttestationSource,
) -> LocalCheckAttestationBindingDefinition {
    let requirement = independent_requirement();
    LocalCheckAttestationBindingDefinition {
        attestation_id: LocalCheckAttestationId::new("attestation/check-1").unwrap(),
        algorithm: LocalCheckAttestationAlgorithm::V1,
        assurance,
        source,
        command_id: LocalCheckCommandId::new("local-check/docs").unwrap(),
        command_contract_fingerprint: hash("command-contract"),
        requirement_fingerprint: requirement.requirement_fingerprint().clone(),
        immutable_run_bundle: bundle(),
        workflow_id: WorkflowId::new("workflow/test").unwrap(),
        run_id: WorkflowRunId::new("run-test").unwrap(),
        step_id: StepId::new("check-docs").unwrap(),
        invocation_id: SkillInvocationId::new("skill-invocation-test").unwrap(),
        idempotency_key: IdempotencyKey::new("check/test").unwrap(),
        handler_fingerprint: hash("handler"),
        result_id: LocalCheckResultId::new("local-check-result/test").unwrap(),
        result_status: LocalCheckResultStatus::Passed,
        exit_code_posture: LocalCheckAttestationExitCodePosture::Zero,
        stdout_truncated: false,
        stderr_truncated: false,
        observed_started_at: Timestamp::parse_rfc3339("2026-07-19T12:00:00Z").unwrap(),
        observed_completed_at: Timestamp::parse_rfc3339("2026-07-19T12:00:02Z").unwrap(),
        freshness: LocalCheckAttestationFreshnessPolicy::NoReuse,
    }
}

fn independent_requirement() -> LocalCheckAttestationRequirement {
    LocalCheckAttestationRequirement::new(LocalCheckAttestationRequirementDefinition {
        command_id: LocalCheckCommandId::new("local-check/docs").unwrap(),
        minimum_assurance: LocalCheckAttestationAssurance::KernelObservedLocalProcess,
        accepted_statuses: vec![LocalCheckResultStatus::Passed],
        freshness: LocalCheckAttestationFreshnessPolicy::NoReuse,
        exact_immutable_run_binding_required: true,
        truncation_allowed: false,
    })
    .unwrap()
}

#[test]
fn independent_requirement_is_exact_and_round_trips() {
    let requirement = independent_requirement();
    assert!(requirement.exact_immutable_run_binding_required());
    assert_eq!(
        requirement.minimum_assurance(),
        LocalCheckAttestationAssurance::KernelObservedLocalProcess
    );
    let encoded = serde_json::to_string(&requirement).unwrap();
    let decoded: LocalCheckAttestationRequirement = serde_json::from_str(&encoded).unwrap();
    assert_eq!(decoded, requirement);
    assert_eq!(
        requirement.requirement_fingerprint().as_str(),
        "4277d60ad6c9bebfb4344dd605f880cccb81a69c21d89bac65416621d4e191a4"
    );
}

#[test]
fn caller_and_mock_assurance_cannot_define_independent_requirement() {
    for assurance in [
        LocalCheckAttestationAssurance::CallerAsserted,
        LocalCheckAttestationAssurance::MockObserved,
        LocalCheckAttestationAssurance::ExternalVerifier,
    ] {
        let error =
            LocalCheckAttestationRequirement::new(LocalCheckAttestationRequirementDefinition {
                command_id: LocalCheckCommandId::new("local-check/docs").unwrap(),
                minimum_assurance: assurance,
                accepted_statuses: vec![LocalCheckResultStatus::Passed],
                freshness: LocalCheckAttestationFreshnessPolicy::NoReuse,
                exact_immutable_run_binding_required: true,
                truncation_allowed: false,
            })
            .unwrap_err();
        assert_eq!(
            error.code(),
            "local_check_attestation.requirement.assurance_unsupported"
        );
    }
}

#[test]
fn every_constructed_candidate_remains_explicitly_unverified() {
    let caller = LocalCheckAttestationBinding::new(candidate_definition(
        LocalCheckAttestationAssurance::CallerAsserted,
        LocalCheckAttestationSource::Caller,
    ))
    .unwrap();
    assert_eq!(
        caller.verification_posture(),
        LocalCheckAttestationVerificationPosture::Unverified
    );
    assert!(!caller.eligible_for_v0_verification());

    let kernel_claim = LocalCheckAttestationBinding::new(candidate_definition(
        LocalCheckAttestationAssurance::KernelObservedLocalProcess,
        LocalCheckAttestationSource::KernelLocalProcessRunner,
    ))
    .unwrap();
    assert_eq!(
        kernel_claim.verification_posture(),
        LocalCheckAttestationVerificationPosture::Unverified
    );
    assert!(kernel_claim.eligible_for_v0_verification());
}

#[test]
fn candidate_round_trip_fails_closed_on_fingerprint_tampering() {
    let candidate = LocalCheckAttestationBinding::new(candidate_definition(
        LocalCheckAttestationAssurance::MockObserved,
        LocalCheckAttestationSource::MockHandler,
    ))
    .unwrap();
    let encoded = serde_json::to_string(&candidate).unwrap();
    let decoded: LocalCheckAttestationBinding = serde_json::from_str(&encoded).unwrap();
    assert_eq!(decoded, candidate);

    let mut tampered: Value = serde_json::from_str(&encoded).unwrap();
    tampered["binding_fingerprint"] = Value::String(hash("tampered").as_str().to_owned());
    let error = serde_json::from_value::<LocalCheckAttestationBinding>(tampered).unwrap_err();
    assert_eq!(error.to_string(), "invalid local check attestation binding");
}

#[test]
fn serialized_verified_posture_is_rejected_without_echoing_values() {
    let candidate = LocalCheckAttestationBinding::new(candidate_definition(
        LocalCheckAttestationAssurance::CallerAsserted,
        LocalCheckAttestationSource::Caller,
    ))
    .unwrap();
    let mut value = serde_json::to_value(candidate).unwrap();
    value["verification_posture"] = Value::String("verified-secret-token".to_owned());
    let error = serde_json::from_value::<LocalCheckAttestationBinding>(value).unwrap_err();
    assert!(!error.to_string().contains("secret-token"));
}

#[test]
fn binding_fingerprint_changes_with_decision_relevant_input() {
    let first = LocalCheckAttestationBinding::new(candidate_definition(
        LocalCheckAttestationAssurance::CallerAsserted,
        LocalCheckAttestationSource::Caller,
    ))
    .unwrap();
    let mut changed = candidate_definition(
        LocalCheckAttestationAssurance::CallerAsserted,
        LocalCheckAttestationSource::Caller,
    );
    changed.stdout_truncated = true;
    let changed = LocalCheckAttestationBinding::new(changed).unwrap();
    assert_ne!(first.binding_fingerprint(), changed.binding_fingerprint());
    assert_eq!(
        first.binding_fingerprint().as_str(),
        "684c5bf5e55fc6e11203f4dbd2db6c7bd4be9d0f22e2a4a98ca99f90672b9e0f"
    );
}

#[test]
fn record_id_does_not_change_canonical_proof_identity() {
    let first = LocalCheckAttestationBinding::new(candidate_definition(
        LocalCheckAttestationAssurance::CallerAsserted,
        LocalCheckAttestationSource::Caller,
    ))
    .unwrap();
    let mut second_definition = candidate_definition(
        LocalCheckAttestationAssurance::CallerAsserted,
        LocalCheckAttestationSource::Caller,
    );
    second_definition.attestation_id =
        LocalCheckAttestationId::new("attestation/different-record").unwrap();
    let second = LocalCheckAttestationBinding::new(second_definition).unwrap();
    assert_eq!(first.binding_fingerprint(), second.binding_fingerprint());
}

#[test]
fn every_valid_requirement_field_changes_or_canonicalizes_the_fingerprint() {
    let baseline = independent_requirement();

    let changed_command =
        LocalCheckAttestationRequirement::new(LocalCheckAttestationRequirementDefinition {
            command_id: LocalCheckCommandId::new("local-check/cargo-fmt").unwrap(),
            minimum_assurance: LocalCheckAttestationAssurance::KernelObservedLocalProcess,
            accepted_statuses: vec![LocalCheckResultStatus::Passed],
            freshness: LocalCheckAttestationFreshnessPolicy::NoReuse,
            exact_immutable_run_binding_required: true,
            truncation_allowed: false,
        })
        .unwrap();
    assert_ne!(
        baseline.requirement_fingerprint(),
        changed_command.requirement_fingerprint()
    );

    let changed_statuses =
        LocalCheckAttestationRequirement::new(LocalCheckAttestationRequirementDefinition {
            command_id: LocalCheckCommandId::new("local-check/docs").unwrap(),
            minimum_assurance: LocalCheckAttestationAssurance::KernelObservedLocalProcess,
            accepted_statuses: vec![
                LocalCheckResultStatus::Failed,
                LocalCheckResultStatus::Passed,
            ],
            freshness: LocalCheckAttestationFreshnessPolicy::NoReuse,
            exact_immutable_run_binding_required: true,
            truncation_allowed: false,
        })
        .unwrap();
    assert_ne!(
        baseline.requirement_fingerprint(),
        changed_statuses.requirement_fingerprint()
    );

    let changed_freshness =
        LocalCheckAttestationRequirement::new(LocalCheckAttestationRequirementDefinition {
            command_id: LocalCheckCommandId::new("local-check/docs").unwrap(),
            minimum_assurance: LocalCheckAttestationAssurance::KernelObservedLocalProcess,
            accepted_statuses: vec![LocalCheckResultStatus::Passed],
            freshness: LocalCheckAttestationFreshnessPolicy::max_age_seconds(60).unwrap(),
            exact_immutable_run_binding_required: true,
            truncation_allowed: false,
        })
        .unwrap();
    assert_ne!(
        baseline.requirement_fingerprint(),
        changed_freshness.requirement_fingerprint()
    );

    let changed_truncation =
        LocalCheckAttestationRequirement::new(LocalCheckAttestationRequirementDefinition {
            command_id: LocalCheckCommandId::new("local-check/docs").unwrap(),
            minimum_assurance: LocalCheckAttestationAssurance::KernelObservedLocalProcess,
            accepted_statuses: vec![LocalCheckResultStatus::Passed],
            freshness: LocalCheckAttestationFreshnessPolicy::NoReuse,
            exact_immutable_run_binding_required: true,
            truncation_allowed: true,
        })
        .unwrap();
    assert_ne!(
        baseline.requirement_fingerprint(),
        changed_truncation.requirement_fingerprint()
    );

    let canonical_order =
        LocalCheckAttestationRequirement::new(LocalCheckAttestationRequirementDefinition {
            command_id: LocalCheckCommandId::new("local-check/docs").unwrap(),
            minimum_assurance: LocalCheckAttestationAssurance::KernelObservedLocalProcess,
            accepted_statuses: vec![
                LocalCheckResultStatus::Passed,
                LocalCheckResultStatus::Failed,
            ],
            freshness: LocalCheckAttestationFreshnessPolicy::NoReuse,
            exact_immutable_run_binding_required: true,
            truncation_allowed: false,
        })
        .unwrap();
    assert_eq!(
        canonical_order.requirement_fingerprint(),
        changed_statuses.requirement_fingerprint()
    );

    for invalid_assurance in [
        LocalCheckAttestationAssurance::CallerAsserted,
        LocalCheckAttestationAssurance::MockObserved,
        LocalCheckAttestationAssurance::ExternalVerifier,
    ] {
        assert!(LocalCheckAttestationRequirement::new(
            LocalCheckAttestationRequirementDefinition {
                command_id: LocalCheckCommandId::new("local-check/docs").unwrap(),
                minimum_assurance: invalid_assurance,
                accepted_statuses: vec![LocalCheckResultStatus::Passed],
                freshness: LocalCheckAttestationFreshnessPolicy::NoReuse,
                exact_immutable_run_binding_required: true,
                truncation_allowed: false,
            }
        )
        .is_err());
    }
    assert!(
        LocalCheckAttestationRequirement::new(LocalCheckAttestationRequirementDefinition {
            command_id: LocalCheckCommandId::new("local-check/docs").unwrap(),
            minimum_assurance: LocalCheckAttestationAssurance::KernelObservedLocalProcess,
            accepted_statuses: vec![LocalCheckResultStatus::Passed],
            freshness: LocalCheckAttestationFreshnessPolicy::NoReuse,
            exact_immutable_run_binding_required: false,
            truncation_allowed: false,
        })
        .is_err()
    );
}

#[test]
fn debug_and_serialization_exclude_raw_payload_fields() {
    let candidate = LocalCheckAttestationBinding::new(candidate_definition(
        LocalCheckAttestationAssurance::CallerAsserted,
        LocalCheckAttestationSource::Caller,
    ))
    .unwrap();
    let debug = format!("{candidate:?}");
    for forbidden in ["workflow/test", "run-test", "check/test", "bundle/test"] {
        assert!(!debug.contains(forbidden));
    }
    let serialized = serde_json::to_string(&candidate).unwrap();
    for forbidden_field in [
        "stdout",
        "stderr",
        "summary",
        "arguments",
        "environment_values",
        "source_contents",
        "provider_payload",
    ] {
        assert!(!serialized.contains(&format!("\"{forbidden_field}\"")));
    }
}

#[test]
fn invalid_source_assurance_and_time_order_fail_without_values() {
    let mut mismatch = candidate_definition(
        LocalCheckAttestationAssurance::CallerAsserted,
        LocalCheckAttestationSource::MockHandler,
    );
    let error = LocalCheckAttestationBinding::new(mismatch).unwrap_err();
    assert_eq!(
        error.code(),
        "local_check_attestation.binding.source_assurance_mismatch"
    );

    mismatch = candidate_definition(
        LocalCheckAttestationAssurance::CallerAsserted,
        LocalCheckAttestationSource::Caller,
    );
    mismatch.observed_started_at = Timestamp::parse_rfc3339("2026-07-19T12:01:00Z").unwrap();
    let error = LocalCheckAttestationBinding::new(mismatch).unwrap_err();
    assert_eq!(
        error.code(),
        "local_check_attestation.binding.observation_order_invalid"
    );
}

#[test]
fn freshness_policy_is_bounded_and_fails_closed_in_serde() {
    assert!(LocalCheckAttestationFreshnessPolicy::max_age_seconds(60).is_ok());
    assert!(LocalCheckAttestationFreshnessPolicy::max_age_seconds(0).is_err());
    let error = serde_json::from_value::<LocalCheckAttestationFreshnessPolicy>(json!({
        "mode": "max_age_seconds",
        "seconds": 99_999_999
    }))
    .unwrap_err();
    assert_eq!(
        error.to_string(),
        "local check attestation freshness policy is invalid"
    );
}
