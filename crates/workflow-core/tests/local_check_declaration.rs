#![allow(clippy::expect_used)]
//! Schema-facing local-check requirement declaration tests.

use workflow_core::{
    LocalCheckAttestationAssurance, LocalCheckAttestationFreshnessPolicy, LocalCheckCommandId,
    LocalCheckNetworkPolicy, LocalCheckRequirementDeclaration,
    LocalCheckRequirementDeclarationDefinition, LocalCheckRequirementId,
    LocalCheckRequirementLevel, LocalCheckResultStatus, LocalCheckSideEffectClass,
};

fn valid_definition() -> LocalCheckRequirementDeclarationDefinition {
    LocalCheckRequirementDeclarationDefinition {
        id: LocalCheckRequirementId::new("docs-required").expect("valid requirement id"),
        command_id: LocalCheckCommandId::new("workflow-os/docs-check").expect("valid command id"),
        requirement_level: LocalCheckRequirementLevel::Required,
        minimum_assurance: LocalCheckAttestationAssurance::KernelObservedLocalProcess,
        accepted_statuses: vec![LocalCheckResultStatus::Passed],
        freshness: LocalCheckAttestationFreshnessPolicy::NoReuse,
        exact_immutable_run_binding_required: true,
        truncation_allowed: false,
        network_maximum: LocalCheckNetworkPolicy::Disabled,
        side_effect_maximum: LocalCheckSideEffectClass::NoSourceWrites,
    }
}

#[test]
fn valid_local_check_declaration_is_bounded_and_accessible() {
    let declaration =
        LocalCheckRequirementDeclaration::new(valid_definition()).expect("valid declaration");

    assert_eq!(declaration.id().as_str(), "docs-required");
    assert_eq!(declaration.command_id().as_str(), "workflow-os/docs-check");
    assert_eq!(
        declaration.requirement_level(),
        LocalCheckRequirementLevel::Required
    );
    assert_eq!(
        declaration.minimum_assurance(),
        LocalCheckAttestationAssurance::KernelObservedLocalProcess
    );
    assert_eq!(
        declaration.accepted_statuses(),
        &[LocalCheckResultStatus::Passed]
    );
    assert!(declaration.exact_immutable_run_binding_required());
    assert!(!declaration.truncation_allowed());
    assert_eq!(
        declaration.network_maximum(),
        LocalCheckNetworkPolicy::Disabled
    );
    assert_eq!(
        declaration.side_effect_maximum(),
        LocalCheckSideEffectClass::NoSourceWrites
    );
}

#[test]
fn declaration_rejects_weak_assurance_without_leaking_ids() {
    let mut definition = valid_definition();
    definition.minimum_assurance = LocalCheckAttestationAssurance::CallerAsserted;

    let error = LocalCheckRequirementDeclaration::new(definition)
        .expect_err("caller assertion cannot satisfy the declaration");

    assert_eq!(
        error.code(),
        "local_check.declaration.assurance_unsupported"
    );
    assert!(!error.to_string().contains("docs-required"));
}

#[test]
fn declaration_rejects_non_passing_and_ambiguous_status_sets() {
    for statuses in [
        Vec::new(),
        vec![LocalCheckResultStatus::Failed],
        vec![
            LocalCheckResultStatus::Passed,
            LocalCheckResultStatus::Passed,
        ],
    ] {
        let mut definition = valid_definition();
        definition.accepted_statuses = statuses;

        let error = LocalCheckRequirementDeclaration::new(definition)
            .expect_err("only exactly one passed status is supported");

        assert_eq!(
            error.code(),
            "local_check.declaration.accepted_statuses_unsupported"
        );
    }
}

#[test]
fn declaration_rejects_relaxed_bundle_and_unclassified_side_effect_posture() {
    let mut missing_binding = valid_definition();
    missing_binding.exact_immutable_run_binding_required = false;
    assert_eq!(
        LocalCheckRequirementDeclaration::new(missing_binding)
            .expect_err("exact binding is mandatory")
            .code(),
        "local_check.declaration.bundle_binding_required"
    );

    let mut unclassified = valid_definition();
    unclassified.side_effect_maximum = LocalCheckSideEffectClass::Unclassified;
    assert_eq!(
        LocalCheckRequirementDeclaration::new(unclassified)
            .expect_err("unclassified effects fail closed")
            .code(),
        "local_check.declaration.side_effect_unclassified"
    );
}

#[test]
fn declaration_serde_round_trip_is_validated_and_debug_redacts_ids() {
    let declaration =
        LocalCheckRequirementDeclaration::new(valid_definition()).expect("valid declaration");
    let serialized = serde_json::to_string(&declaration).expect("serialization succeeds");
    let round_trip: LocalCheckRequirementDeclaration =
        serde_json::from_str(&serialized).expect("valid declaration deserializes");

    assert_eq!(round_trip, declaration);
    let debug = format!("{declaration:?}");
    assert!(!debug.contains("docs-required"));
    assert!(!debug.contains("workflow-os/docs-check"));

    let invalid = serialized.replace("kernel_observed_local_process", "caller_asserted");
    let error = serde_json::from_str::<LocalCheckRequirementDeclaration>(&invalid)
        .expect_err("invalid serialized declaration fails closed");
    assert!(!error.to_string().contains("docs-required"));
    assert!(!error.to_string().contains("workflow-os/docs-check"));
}

#[test]
fn requirement_id_rejects_secret_like_values_without_leaking() {
    let error = LocalCheckRequirementId::new("authorization-bearer-token")
        .expect_err("secret-like identifiers fail closed");

    assert_eq!(error.code(), "local_check.secret_like_value");
    assert!(!error.to_string().contains("authorization-bearer-token"));
}
