#![allow(clippy::expect_used)]
//! Payload-free visible-disclosure delivery model tests.

use serde_json::{json, Value};
use workflow_core::{
    CorrelationId, GovernanceAssessmentBinding, GovernanceDisclosureAcknowledgement,
    GovernanceDisclosureDeliveryId, GovernanceDisclosureDeliveryReceipt,
    GovernanceDisclosureDeliveryRequest, GovernanceDisclosureDeliveryStatus,
    GovernanceDisclosureHumanObservation, GovernanceDisclosureRedactionPosture,
    GovernanceDisclosureSensitivity, GovernanceDisclosureSurface, GovernanceDisclosureSurfaceKind,
    SpecContentHash, Timestamp,
};

fn assessment(
    execution: &str,
    disclosure: &str,
    completeness: &str,
    source_bound: bool,
    run_id: &str,
) -> GovernanceAssessmentBinding {
    let mut value = json!({
        "binding_version": if source_bound { "v2" } else { "v1" },
        "assessment_set_algorithm": "v1",
        "workflow_id": "workflow/visible-disclosure",
        "run_id": run_id,
        "immutable_run_bundle": {
            "bundle_id": "bundle/visible-disclosure",
            "bundle_version": "v1",
            "root_hash": SpecContentHash::from_text("bundle root").as_str(),
        },
        "aggregate_fingerprint": SpecContentHash::from_text("visible assessment").as_str(),
        "step_count": 1,
        "execution": execution,
        "disclosure": disclosure,
        "completeness": completeness,
    });
    if source_bound {
        value["source_binding"] = json!({
            "kind": "authoritative_local_check_reassessment",
            "algorithm": "v1",
            "fingerprint": SpecContentHash::from_text("authoritative check").as_str(),
            "selected_step_id": "docs-check",
        });
    }
    serde_json::from_value(value).expect("valid assessment fixture")
}

fn request_with(
    delivery_id: &str,
    assessment: GovernanceAssessmentBinding,
) -> Result<GovernanceDisclosureDeliveryRequest, workflow_core::WorkflowOsError> {
    GovernanceDisclosureDeliveryRequest::new(
        GovernanceDisclosureDeliveryId::new(delivery_id)?,
        assessment,
        GovernanceDisclosureSurface::new(
            GovernanceDisclosureSurfaceKind::InjectedLocal,
            "surface/operator-stream",
        )?,
        CorrelationId::new("correlation-visible")?,
        Timestamp::parse_rfc3339("2026-07-26T02:00:00Z")?,
        GovernanceDisclosureSensitivity::Internal,
    )
}

fn request() -> GovernanceDisclosureDeliveryRequest {
    request_with(
        "delivery/visible-run",
        assessment("proceed", "visible", "complete", true, "run-visible"),
    )
    .expect("valid request")
}

fn receipt() -> GovernanceDisclosureDeliveryReceipt {
    GovernanceDisclosureDeliveryReceipt::surface_accepted(
        request(),
        Timestamp::parse_rfc3339("2026-07-26T02:00:01Z").expect("timestamp"),
    )
    .expect("valid receipt")
}

#[test]
fn valid_request_binds_exact_source_bound_visible_proceed_assessment() {
    let request = request();

    assert_eq!(request.delivery_id().as_str(), "delivery/visible-run");
    assert_eq!(request.assessment().run_id().as_str(), "run-visible");
    assert_eq!(
        request.surface().kind(),
        GovernanceDisclosureSurfaceKind::InjectedLocal
    );
    assert_eq!(request.surface().reference(), "surface/operator-stream");
    assert_eq!(
        request.redaction(),
        GovernanceDisclosureRedactionPosture::ReferenceOnly
    );
}

#[test]
fn request_rejects_incomplete_quiet_approval_and_unbound_assessments() {
    let cases = [
        (
            assessment("proceed", "visible", "incomplete", true, "run-incomplete"),
            "governance.disclosure_delivery.request.assessment_incomplete",
        ),
        (
            assessment("proceed", "quiet", "complete", true, "run-quiet"),
            "governance.disclosure_delivery.request.route_invalid",
        ),
        (
            assessment(
                "require_approval",
                "visible",
                "complete",
                true,
                "run-approval",
            ),
            "governance.disclosure_delivery.request.route_invalid",
        ),
        (
            assessment("proceed", "visible", "complete", false, "run-unbound"),
            "governance.disclosure_delivery.request.source_binding_required",
        ),
    ];

    for (assessment, expected_code) in cases {
        let error = request_with("delivery/invalid", assessment).expect_err("request rejected");
        assert_eq!(error.code(), expected_code);
    }
}

#[test]
fn receipt_claims_surface_acceptance_without_human_observation_or_acknowledgement() {
    let receipt = receipt();

    assert_eq!(
        receipt.status(),
        GovernanceDisclosureDeliveryStatus::SurfaceAccepted
    );
    assert_eq!(
        receipt.human_observation(),
        GovernanceDisclosureHumanObservation::NotClaimed
    );
    assert_eq!(
        receipt.acknowledgement(),
        GovernanceDisclosureAcknowledgement::NotClaimed
    );
    receipt
        .validate_for_request(&request())
        .expect("exact request matches");
}

#[test]
fn receipt_rejects_a_different_run_or_delivery_request() {
    let receipt = receipt();
    let changed_delivery = request_with(
        "delivery/other",
        assessment("proceed", "visible", "complete", true, "run-visible"),
    )
    .expect("changed request");
    let changed_run = request_with(
        "delivery/visible-run",
        assessment("proceed", "visible", "complete", true, "run-other"),
    )
    .expect("changed run");

    for changed in [&changed_delivery, &changed_run] {
        let error = receipt
            .validate_for_request(changed)
            .expect_err("mismatch rejected");
        assert_eq!(
            error.code(),
            "governance.disclosure_delivery.receipt.request_mismatch"
        );
    }
}

#[test]
fn receipt_rejects_acceptance_before_the_delivery_request() {
    let error = GovernanceDisclosureDeliveryReceipt::surface_accepted(
        request(),
        Timestamp::parse_rfc3339("2026-07-26T01:59:59Z").expect("timestamp"),
    )
    .expect_err("timestamp rejected");

    assert_eq!(
        error.code(),
        "governance.disclosure_delivery.receipt.timestamp_invalid"
    );
}

#[test]
fn valid_request_and_receipt_round_trip_through_serde() {
    let request = request();
    let request_wire = serde_json::to_string(&request).expect("request serialized");
    let request_round_trip: GovernanceDisclosureDeliveryRequest =
        serde_json::from_str(&request_wire).expect("request deserialized");
    assert_eq!(request_round_trip, request);

    let receipt = receipt();
    let receipt_wire = serde_json::to_string(&receipt).expect("receipt serialized");
    let receipt_round_trip: GovernanceDisclosureDeliveryReceipt =
        serde_json::from_str(&receipt_wire).expect("receipt deserialized");
    assert_eq!(receipt_round_trip, receipt);
}

#[test]
fn invalid_wire_claims_fail_closed_without_echoing_secret_like_values() {
    let mut value = serde_json::to_value(receipt()).expect("receipt value");
    value["status"] = Value::String("bearer-super-sensitive".to_owned());

    let error = serde_json::from_value::<GovernanceDisclosureDeliveryReceipt>(value)
        .expect_err("unknown claim rejected")
        .to_string();

    assert!(!error.contains("bearer-super-sensitive"));
    assert!(error.contains("delivery status is invalid"));
}

#[test]
fn secret_like_delivery_surface_and_correlation_identifiers_are_rejected_safely() {
    let delivery_error = GovernanceDisclosureDeliveryId::new("delivery/api-token-value")
        .expect_err("secret-like delivery id rejected");
    assert_eq!(
        delivery_error.code(),
        "governance.disclosure_delivery.identifier.secret_like"
    );
    assert!(!delivery_error.to_string().contains("api-token-value"));

    let surface_error = GovernanceDisclosureSurface::new(
        GovernanceDisclosureSurfaceKind::InjectedLocal,
        "surface/private-key-value",
    )
    .expect_err("secret-like surface rejected");
    assert_eq!(
        surface_error.code(),
        "governance.disclosure_delivery.identifier.secret_like"
    );
    assert!(!surface_error.to_string().contains("private-key-value"));

    let correlation = CorrelationId::new("correlation-secret-value").expect("core identifier");
    let error = GovernanceDisclosureDeliveryRequest::new(
        GovernanceDisclosureDeliveryId::new("delivery/valid").expect("delivery id"),
        assessment("proceed", "visible", "complete", true, "run-visible"),
        GovernanceDisclosureSurface::new(
            GovernanceDisclosureSurfaceKind::InjectedLocal,
            "surface/valid",
        )
        .expect("surface"),
        correlation,
        Timestamp::parse_rfc3339("2026-07-26T02:00:00Z").expect("timestamp"),
        GovernanceDisclosureSensitivity::Internal,
    )
    .expect_err("secret-like correlation rejected");
    assert_eq!(
        error.code(),
        "governance.disclosure_delivery.identifier.secret_like"
    );
    assert!(!error.to_string().contains("correlation-secret-value"));
}

#[test]
fn debug_and_serialization_are_payload_free_and_debug_redacts_identifiers() {
    let request = request();
    let receipt = receipt();
    let debug = format!("{request:?} {receipt:?}");
    let serialized = serde_json::to_string(&receipt).expect("serialized");

    for value in [
        "delivery/visible-run",
        "surface/operator-stream",
        "correlation-visible",
        "workflow/visible-disclosure",
        "run-visible",
    ] {
        assert!(!debug.contains(value));
    }
    for forbidden_field in [
        "\"payload\"",
        "\"message\"",
        "\"summary\"",
        "\"command\"",
        "\"output\"",
        "\"path\"",
        "\"acknowledged_by\"",
        "\"observed_by\"",
    ] {
        assert!(!serialized.contains(forbidden_field));
    }
    assert!(serialized.contains("\"human_observation\":\"not_claimed\""));
    assert!(serialized.contains("\"acknowledgement\":\"not_claimed\""));
}

#[test]
fn deserialization_rejects_claim_and_request_tampering() {
    let mut observed = serde_json::to_value(receipt()).expect("receipt value");
    observed["human_observation"] = Value::String("observed".to_owned());
    assert!(serde_json::from_value::<GovernanceDisclosureDeliveryReceipt>(observed).is_err());

    let mut unknown_field = serde_json::to_value(request()).expect("request value");
    unknown_field["authorization-super-sensitive"] = Value::String("must-not-be-stored".to_owned());
    let error = serde_json::from_value::<GovernanceDisclosureDeliveryRequest>(unknown_field)
        .expect_err("unknown field rejected")
        .to_string();
    assert!(!error.contains("authorization-super-sensitive"));
    assert!(!error.contains("must-not-be-stored"));
    assert!(error.contains("contains an unknown field"));
}
