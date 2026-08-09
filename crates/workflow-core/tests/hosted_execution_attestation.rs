//! Hosted execution attestation contract tests.

#![allow(clippy::expect_used, clippy::panic)]

use workflow_core::{
    HostedExecutionAttestation, HostedExecutionCleanupPosture, HostedExecutionControlPosture,
    HostedExecutionEnforcementMode, HostedExecutionObservationSummary,
    HostedExecutionPolicyRevision, HostedExecutionReference, HostedExecutionReferenceKind,
    SpecContentHash,
};

fn reference(kind: HostedExecutionReferenceKind, value: &str) -> HostedExecutionReference {
    HostedExecutionReference::new(kind, value).unwrap_or_else(|error| panic!("{error}"))
}

fn observations() -> HostedExecutionObservationSummary {
    HostedExecutionObservationSummary::new(
        0,
        1,
        1,
        1,
        0,
        0,
        reference(
            HostedExecutionReferenceKind::Telemetry,
            "provider/observations/stable",
        ),
    )
    .unwrap_or_else(|error| panic!("{error}"))
}

fn attestation() -> HostedExecutionAttestation {
    HostedExecutionAttestation::new(
        SpecContentHash::from_text("runtime-image"),
        HostedExecutionPolicyRevision::new("revision/stable")
            .unwrap_or_else(|error| panic!("{error}")),
        SpecContentHash::from_text("effective-policy"),
        HostedExecutionEnforcementMode::Enforce,
        HostedExecutionControlPosture::Enforced,
        HostedExecutionControlPosture::Enforced,
        HostedExecutionControlPosture::Enforced,
        observations(),
        HostedExecutionCleanupPosture::Completed,
        reference(
            HostedExecutionReferenceKind::Telemetry,
            "provider/cleanup/stable",
        ),
    )
    .unwrap_or_else(|error| panic!("{error}"))
}

#[test]
fn valid_attestation_round_trips_and_satisfies_hard_requirements() {
    let value = attestation();
    let json = serde_json::to_string(&value).unwrap_or_else(|error| panic!("{error}"));
    let restored: HostedExecutionAttestation =
        serde_json::from_str(&json).unwrap_or_else(|error| panic!("{error}"));

    assert_eq!(restored, value);
    assert!(restored.satisfies_hard_requirements());
}

#[test]
fn degraded_attestation_round_trips_but_cannot_claim_hard_requirements() {
    let value = HostedExecutionAttestation::new(
        SpecContentHash::from_text("runtime-image"),
        HostedExecutionPolicyRevision::new("revision/degraded")
            .unwrap_or_else(|error| panic!("{error}")),
        SpecContentHash::from_text("effective-policy"),
        HostedExecutionEnforcementMode::Audit,
        HostedExecutionControlPosture::Degraded,
        HostedExecutionControlPosture::Enforced,
        HostedExecutionControlPosture::Enforced,
        observations(),
        HostedExecutionCleanupPosture::Completed,
        reference(
            HostedExecutionReferenceKind::Telemetry,
            "provider/cleanup/degraded",
        ),
    )
    .unwrap_or_else(|error| panic!("{error}"));

    assert!(!value.satisfies_hard_requirements());
}

#[test]
fn invalid_serialized_cleanup_reference_fails_closed_without_leaking_value() {
    let mut value = serde_json::to_value(attestation()).unwrap_or_else(|error| panic!("{error}"));
    value["cleanup_reference"]["kind"] = serde_json::json!("artifact");
    value["cleanup_reference"]["value"] = serde_json::json!("private-cleanup-marker");

    let error = serde_json::from_value::<HostedExecutionAttestation>(value)
        .expect_err("invalid cleanup reference must fail closed");
    let message = error.to_string();
    assert!(!message.contains("private-cleanup-marker"));
}

#[test]
fn debug_output_redacts_policy_image_and_reference_identity() {
    let debug = format!("{:?}", attestation());

    assert!(!debug.contains("runtime-image"));
    assert!(!debug.contains("effective-policy"));
    assert!(!debug.contains("revision/stable"));
    assert!(!debug.contains("provider/cleanup/stable"));
}
