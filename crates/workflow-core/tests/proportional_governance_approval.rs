#![allow(clippy::expect_used)]
//! Aggregate proportional-governance approval-binding tests.

use serde_json::{json, Value};
use workflow_core::{
    GovernanceApprovalBinding, GovernanceApprovalBindingId, GovernanceApprovalBindingVersion,
    GovernanceAssessmentBinding, GovernanceExecutionDisposition, SpecContentHash,
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
        "workflow_id": "workflow/aggregate-approval",
        "run_id": run_id,
        "immutable_run_bundle": {
            "bundle_id": "bundle/aggregate-approval",
            "bundle_version": "v1",
            "root_hash": SpecContentHash::from_text("aggregate approval bundle").as_str(),
        },
        "aggregate_fingerprint": SpecContentHash::from_text("aggregate approval assessment").as_str(),
        "step_count": 3,
        "execution": execution,
        "disclosure": disclosure,
        "completeness": completeness,
    });
    if source_bound {
        value["source_binding"] = json!({
            "kind": "authoritative_local_check_reassessment",
            "algorithm": "v1",
            "fingerprint": SpecContentHash::from_text("authoritative approval check").as_str(),
            "selected_step_id": "docs-check",
        });
    }
    serde_json::from_value(value).expect("valid assessment fixture")
}

fn binding_with(
    id: &str,
    assessment: GovernanceAssessmentBinding,
) -> Result<GovernanceApprovalBinding, workflow_core::WorkflowOsError> {
    GovernanceApprovalBinding::new(GovernanceApprovalBindingId::new(id)?, assessment)
}

fn binding() -> GovernanceApprovalBinding {
    binding_with(
        "approval-binding/aggregate-run",
        assessment(
            "require_approval",
            "visible",
            "complete",
            true,
            "run-aggregate-approval",
        ),
    )
    .expect("valid binding")
}

#[test]
fn valid_binding_commits_to_the_exact_authoritative_aggregate_assessment() {
    let binding = binding();

    assert_eq!(
        binding.binding_version(),
        GovernanceApprovalBindingVersion::V1
    );
    assert_eq!(
        binding.approval_binding_id().as_str(),
        "approval-binding/aggregate-run"
    );
    assert_eq!(
        binding.workflow_id().as_str(),
        "workflow/aggregate-approval"
    );
    assert_eq!(binding.run_id().as_str(), "run-aggregate-approval");
    assert_eq!(
        binding.assessment().execution(),
        GovernanceExecutionDisposition::RequireApproval
    );
    assert!(binding.assessment().source_binding().is_some());
}

#[test]
fn binding_rejects_non_approval_quiet_incomplete_and_unbound_assessments() {
    let cases = [
        (
            assessment("proceed", "visible", "complete", true, "run-proceed"),
            "governance.proportional_approval_binding.assessment.route_invalid",
        ),
        (
            assessment("denied", "visible", "complete", true, "run-denied"),
            "governance.proportional_approval_binding.assessment.route_invalid",
        ),
        (
            assessment(
                "require_approval",
                "visible",
                "incomplete",
                true,
                "run-incomplete",
            ),
            "governance.proportional_approval_binding.assessment.incomplete",
        ),
        (
            assessment(
                "require_approval",
                "visible",
                "complete",
                false,
                "run-unbound",
            ),
            "governance.proportional_approval_binding.assessment.source_binding_required",
        ),
    ];

    for (assessment, expected_code) in cases {
        let error = binding_with("approval-binding/invalid", assessment)
            .expect_err("invalid assessment rejected");
        assert_eq!(error.code(), expected_code);
    }
}

#[test]
fn binding_identifier_is_bounded_and_rejects_secret_like_text_without_leakage() {
    let too_long = "a".repeat(129);
    for value in ["", too_long.as_str(), "approval/token-value"] {
        let error = GovernanceApprovalBindingId::new(value).expect_err("identifier rejected");
        assert!(
            error.code() == "governance.proportional_approval_binding.identifier.invalid"
                || error.code()
                    == "governance.proportional_approval_binding.identifier.secret_like"
        );
        if !value.is_empty() {
            assert!(!error.to_string().contains(value));
        }
    }
}

#[test]
fn valid_binding_round_trips_and_invalid_serialized_routes_fail_closed() {
    let binding = binding();
    let serialized = serde_json::to_value(&binding).expect("serialize");
    let round_trip: GovernanceApprovalBinding =
        serde_json::from_value(serialized.clone()).expect("deserialize");
    assert_eq!(round_trip, binding);

    for (field, value) in [
        ("execution", "proceed"),
        ("disclosure", "quiet"),
        ("completeness", "incomplete"),
    ] {
        let mut changed = serialized.clone();
        changed["assessment"][field] = Value::String(value.to_owned());
        let error =
            serde_json::from_value::<GovernanceApprovalBinding>(changed).expect_err("rejected");
        assert!(!error.to_string().contains("run-aggregate-approval"));
        assert!(!error.to_string().contains("approval-binding/aggregate-run"));
    }
}

#[test]
fn unknown_serialized_fields_fail_closed() {
    let mut serialized = serde_json::to_value(binding()).expect("serialize");
    serialized["unexpected_payload"] = json!("must-not-be-accepted");

    let error =
        serde_json::from_value::<GovernanceApprovalBinding>(serialized).expect_err("rejected");
    assert!(error.to_string().contains("unknown field"));
    assert!(!error.to_string().contains("must-not-be-accepted"));
}

#[test]
fn debug_output_redacts_binding_and_assessment_identity() {
    let rendered = format!("{:?}", binding());

    assert!(!rendered.contains("approval-binding/aggregate-run"));
    assert!(!rendered.contains("workflow/aggregate-approval"));
    assert!(!rendered.contains("run-aggregate-approval"));
    assert!(rendered.contains("<redacted>"));
}
