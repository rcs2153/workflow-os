#![allow(clippy::expect_used)]

//! High-assurance approval control core model tests.

use serde_json::json;
use workflow_core::{
    EventId, EvidenceReferenceId, HighAssuranceApprovalControl,
    HighAssuranceApprovalControlDefinition, HighAssuranceApprovalControlId,
    HighAssuranceApprovalControlVersion, HighAssuranceApprovalDenialBehavior,
    HighAssuranceApprovalExpirationPolicy, HighAssuranceApprovalReportDisclosure,
    HighAssuranceApprovalRequiredReference, HighAssuranceApprovalRequiredReferenceTarget,
    HighAssuranceApprovalRevocationPolicy, HighAssuranceProtectedActionKind,
    HighAssuranceRequesterApproverRule, LocalCheckResultId, RedactionDisposition,
    RedactionFieldState, RedactionMetadata, SchemaVersion, SideEffectId, ValidationReferenceId,
    WorkReportId, WorkReportRedactionPolicy, WorkReportSensitivity, WorkReportStableReference,
};

fn control_id() -> HighAssuranceApprovalControlId {
    HighAssuranceApprovalControlId::new("approval-control/nuclear-key").expect("valid control id")
}

fn control_version() -> HighAssuranceApprovalControlVersion {
    HighAssuranceApprovalControlVersion::new("v1").expect("valid control version")
}

fn schema_version() -> SchemaVersion {
    SchemaVersion::new("workflowos/v0").expect("valid schema version")
}

fn redaction() -> RedactionMetadata {
    RedactionMetadata {
        redacted_fields: vec!["approval_context".to_owned()],
        field_states: vec![RedactionFieldState {
            field: "approval_context".to_owned(),
            disposition: RedactionDisposition::ReferenceOnly,
            reason: "stores bounded approval references".to_owned(),
        }],
    }
}

fn evidence_reference() -> HighAssuranceApprovalRequiredReference {
    HighAssuranceApprovalRequiredReference::new(
        "evidence_reference",
        HighAssuranceApprovalRequiredReferenceTarget::EvidenceReference {
            evidence_reference_id: EvidenceReferenceId::new("evidence/context")
                .expect("valid evidence reference id"),
        },
        true,
    )
    .expect("valid evidence requirement")
}

fn validation_reference() -> HighAssuranceApprovalRequiredReference {
    HighAssuranceApprovalRequiredReference::new(
        "validation_reference",
        HighAssuranceApprovalRequiredReferenceTarget::ValidationReference {
            validation_reference_id: ValidationReferenceId::new("validation/spec")
                .expect("valid validation reference id"),
        },
        true,
    )
    .expect("valid validation requirement")
}

fn valid_definition() -> HighAssuranceApprovalControlDefinition {
    HighAssuranceApprovalControlDefinition {
        control_id: control_id(),
        control_version: control_version(),
        schema_version: schema_version(),
        protected_actions: vec![
            HighAssuranceProtectedActionKind::AdapterWrite,
            HighAssuranceProtectedActionKind::SideEffectAttempt,
        ],
        requester_approver_rule: HighAssuranceRequesterApproverRule::MustDiffer,
        minimum_approvals: 1,
        required_references: vec![evidence_reference(), validation_reference()],
        expiration_policy: HighAssuranceApprovalExpirationPolicy::MustBeUnexpiredAtUse,
        revocation_policy: HighAssuranceApprovalRevocationPolicy::ExplicitEventBeforeUse,
        denial_behavior: HighAssuranceApprovalDenialBehavior::FailClosed,
        report_disclosures: vec![
            HighAssuranceApprovalReportDisclosure::Requested,
            HighAssuranceApprovalReportDisclosure::Granted,
            HighAssuranceApprovalReportDisclosure::Denied,
            HighAssuranceApprovalReportDisclosure::EvidenceConsidered,
            HighAssuranceApprovalReportDisclosure::SideEffectsAuthorized,
        ],
        sensitivity: WorkReportSensitivity::Confidential,
        redaction_policy: WorkReportRedactionPolicy::ReferenceOnly,
        redaction: redaction(),
    }
}

fn valid_control() -> HighAssuranceApprovalControl {
    HighAssuranceApprovalControl::new(valid_definition()).expect("valid control")
}

#[test]
fn valid_minimal_high_assurance_approval_control() {
    let control = valid_control();

    assert_eq!(
        control.control_id().as_str(),
        "approval-control/nuclear-key"
    );
    assert_eq!(control.control_version().as_str(), "v1");
    assert_eq!(control.schema_version().as_str(), "workflowos/v0");
    assert_eq!(control.protected_actions().len(), 2);
    assert_eq!(
        control.requester_approver_rule(),
        HighAssuranceRequesterApproverRule::MustDiffer
    );
    assert_eq!(control.minimum_approvals(), 1);
    assert_eq!(control.required_references().len(), 2);
    assert_eq!(
        control.expiration_policy(),
        HighAssuranceApprovalExpirationPolicy::MustBeUnexpiredAtUse
    );
    assert_eq!(
        control.revocation_policy(),
        HighAssuranceApprovalRevocationPolicy::ExplicitEventBeforeUse
    );
    assert_eq!(
        control.denial_behavior(),
        HighAssuranceApprovalDenialBehavior::FailClosed
    );
    assert_eq!(control.report_disclosures().len(), 5);
    assert_eq!(control.sensitivity(), WorkReportSensitivity::Confidential);
    assert_eq!(
        control.redaction_policy(),
        WorkReportRedactionPolicy::ReferenceOnly
    );
    assert_eq!(control.redaction().redacted_fields.len(), 1);
}

#[test]
fn invalid_control_id_rejected_without_leaking_value() {
    let error = HighAssuranceApprovalControlId::new("bad id").expect_err("invalid id");

    assert_eq!(
        error.code(),
        "high_assurance_approval.identifier.invalid_character"
    );
    assert!(!error.to_string().contains("bad id"));
}

#[test]
fn invalid_version_rejected_without_leaking_value() {
    let error = HighAssuranceApprovalControlVersion::new("").expect_err("invalid version");

    assert_eq!(error.code(), "high_assurance_approval.identifier.empty");
}

#[test]
fn invalid_schema_version_secret_like_value_is_rejected() {
    let mut definition = valid_definition();
    definition.schema_version =
        SchemaVersion::new("workflowos/token").expect("schema id primitive allows token text");

    let error =
        HighAssuranceApprovalControl::new(definition).expect_err("secret-like schema version");

    assert_eq!(error.code(), "high_assurance_approval.secret_like_value");
    assert!(!error.to_string().contains("workflowos/token"));
}

#[test]
fn empty_protected_actions_rejected() {
    let mut definition = valid_definition();
    definition.protected_actions.clear();

    let error = HighAssuranceApprovalControl::new(definition).expect_err("empty protected actions");

    assert_eq!(
        error.code(),
        "high_assurance_approval.protected_actions.required"
    );
}

#[test]
fn duplicate_protected_actions_rejected() {
    let mut definition = valid_definition();
    definition.protected_actions = vec![
        HighAssuranceProtectedActionKind::AdapterWrite,
        HighAssuranceProtectedActionKind::AdapterWrite,
    ];

    let error =
        HighAssuranceApprovalControl::new(definition).expect_err("duplicate protected actions");

    assert_eq!(
        error.code(),
        "high_assurance_approval.protected_actions.duplicate"
    );
}

#[test]
fn minimum_approval_count_must_be_nonzero() {
    let mut definition = valid_definition();
    definition.minimum_approvals = 0;

    let error = HighAssuranceApprovalControl::new(definition).expect_err("zero approvals");

    assert_eq!(
        error.code(),
        "high_assurance_approval.minimum_approvals.invalid"
    );
}

#[test]
fn empty_required_references_rejected() {
    let mut definition = valid_definition();
    definition.required_references.clear();

    let error =
        HighAssuranceApprovalControl::new(definition).expect_err("empty required references");

    assert_eq!(error.code(), "high_assurance_approval.references.required");
}

#[test]
fn duplicate_required_reference_names_rejected() {
    let mut definition = valid_definition();
    definition.required_references = vec![evidence_reference(), evidence_reference()];

    let error =
        HighAssuranceApprovalControl::new(definition).expect_err("duplicate reference names");

    assert_eq!(error.code(), "high_assurance_approval.references.duplicate");
}

#[test]
fn required_reference_target_vocabulary_is_representable() {
    let targets = vec![
        HighAssuranceApprovalRequiredReferenceTarget::EvidenceReference {
            evidence_reference_id: EvidenceReferenceId::new("evidence/context")
                .expect("valid evidence reference"),
        },
        HighAssuranceApprovalRequiredReferenceTarget::PolicyDecision {
            event_id: EventId::new("event/policy-decision").expect("valid event id"),
        },
        HighAssuranceApprovalRequiredReferenceTarget::SideEffect {
            side_effect_id: SideEffectId::new("side-effect/proposed").expect("valid side effect"),
        },
        HighAssuranceApprovalRequiredReferenceTarget::ValidationReference {
            validation_reference_id: ValidationReferenceId::new("validation/spec")
                .expect("valid validation reference"),
        },
        HighAssuranceApprovalRequiredReferenceTarget::LocalCheckResult {
            local_check_result_id: LocalCheckResultId::new("local-check/docs")
                .expect("valid local check result"),
        },
        HighAssuranceApprovalRequiredReferenceTarget::WorkflowEvent {
            event_id: EventId::new("event/workflow").expect("valid workflow event id"),
        },
        HighAssuranceApprovalRequiredReferenceTarget::AuditEvent {
            event_id: EventId::new("event/audit").expect("valid audit event id"),
        },
        HighAssuranceApprovalRequiredReferenceTarget::WorkReport {
            work_report_id: WorkReportId::new("work-report/terminal").expect("valid report id"),
        },
        HighAssuranceApprovalRequiredReferenceTarget::AdapterTelemetry {
            reference: WorkReportStableReference::new("adapter-telemetry/read-context")
                .expect("valid stable reference"),
        },
    ];

    let names: Vec<_> = targets
        .iter()
        .map(HighAssuranceApprovalRequiredReferenceTarget::kind_name)
        .collect();
    assert_eq!(
        names,
        vec![
            "evidence_reference",
            "policy_decision",
            "side_effect",
            "validation_reference",
            "local_check_result",
            "workflow_event",
            "audit_event",
            "work_report",
            "adapter_telemetry",
        ]
    );
}

#[test]
fn protected_action_vocabulary_includes_future_write_terms_without_runtime_execution() {
    let mut definition = valid_definition();
    definition.protected_actions = vec![
        HighAssuranceProtectedActionKind::AdapterWrite,
        HighAssuranceProtectedActionKind::SideEffectAttempt,
        HighAssuranceProtectedActionKind::ReportArtifactWrite,
    ];

    let control =
        HighAssuranceApprovalControl::new(definition).expect("future vocabulary is model-only");

    assert_eq!(control.protected_actions().len(), 3);
}

#[test]
fn requester_approver_separation_rules_are_representable() {
    for requester_approver_rule in [
        HighAssuranceRequesterApproverRule::SameActorAllowed,
        HighAssuranceRequesterApproverRule::MustDiffer,
        HighAssuranceRequesterApproverRule::HumanApproverMustDiffer,
    ] {
        let mut definition = valid_definition();
        definition.requester_approver_rule = requester_approver_rule;

        let control = HighAssuranceApprovalControl::new(definition)
            .expect("requester approver rule representable");

        assert_eq!(control.requester_approver_rule(), requester_approver_rule);
    }
}

#[test]
fn expiration_and_revocation_policies_are_representable() {
    for expiration_policy in [
        HighAssuranceApprovalExpirationPolicy::NotRequired,
        HighAssuranceApprovalExpirationPolicy::RequiredOnRequest,
        HighAssuranceApprovalExpirationPolicy::MustBeUnexpiredAtDecision,
        HighAssuranceApprovalExpirationPolicy::MustBeUnexpiredAtUse,
    ] {
        let mut definition = valid_definition();
        definition.expiration_policy = expiration_policy;

        let control =
            HighAssuranceApprovalControl::new(definition).expect("expiration policy valid");

        assert_eq!(control.expiration_policy(), expiration_policy);
    }

    for revocation_policy in [
        HighAssuranceApprovalRevocationPolicy::Unsupported,
        HighAssuranceApprovalRevocationPolicy::ExplicitEventBeforeUse,
        HighAssuranceApprovalRevocationPolicy::ReportOnlyAfterUse,
    ] {
        let mut definition = valid_definition();
        definition.revocation_policy = revocation_policy;

        let control =
            HighAssuranceApprovalControl::new(definition).expect("revocation policy valid");

        assert_eq!(control.revocation_policy(), revocation_policy);
    }
}

#[test]
fn report_disclosures_require_nonempty_unique_values() {
    let mut definition = valid_definition();
    definition.report_disclosures.clear();

    let error =
        HighAssuranceApprovalControl::new(definition).expect_err("empty disclosures rejected");

    assert_eq!(
        error.code(),
        "high_assurance_approval.report_disclosures.required"
    );

    let mut definition = valid_definition();
    definition.report_disclosures = vec![
        HighAssuranceApprovalReportDisclosure::Requested,
        HighAssuranceApprovalReportDisclosure::Requested,
    ];

    let error =
        HighAssuranceApprovalControl::new(definition).expect_err("duplicate disclosures rejected");

    assert_eq!(
        error.code(),
        "high_assurance_approval.report_disclosures.duplicate"
    );
}

#[test]
fn all_report_disclosures_are_representable() {
    let mut definition = valid_definition();
    definition.report_disclosures = vec![
        HighAssuranceApprovalReportDisclosure::Requested,
        HighAssuranceApprovalReportDisclosure::Granted,
        HighAssuranceApprovalReportDisclosure::Denied,
        HighAssuranceApprovalReportDisclosure::Expired,
        HighAssuranceApprovalReportDisclosure::Revoked,
        HighAssuranceApprovalReportDisclosure::Skipped,
        HighAssuranceApprovalReportDisclosure::Deferred,
        HighAssuranceApprovalReportDisclosure::EvidenceConsidered,
        HighAssuranceApprovalReportDisclosure::SideEffectsAuthorized,
    ];

    let control = HighAssuranceApprovalControl::new(definition).expect("all disclosures valid");

    assert_eq!(control.report_disclosures().len(), 9);
}

#[test]
fn serde_round_trip_for_valid_control() {
    let control = valid_control();
    let serialized = serde_json::to_string(&control).expect("serialize control");
    let deserialized: HighAssuranceApprovalControl =
        serde_json::from_str(&serialized).expect("deserialize control");

    assert_eq!(deserialized, control);
}

#[test]
fn invalid_serialized_control_fails_closed() {
    let mut value = serde_json::to_value(valid_control()).expect("serialize control");
    value["minimum_approvals"] = json!(0);

    let error = serde_json::from_value::<HighAssuranceApprovalControl>(value)
        .expect_err("invalid serialized control");

    assert!(error
        .to_string()
        .contains("high-assurance approval controls require at least one approval"));
}

#[test]
fn secret_like_redaction_metadata_is_rejected_without_leakage() {
    let mut definition = valid_definition();
    definition.redaction = RedactionMetadata {
        redacted_fields: vec!["authorization_token".to_owned()],
        field_states: vec![RedactionFieldState {
            field: "approval_context".to_owned(),
            disposition: RedactionDisposition::Redacted,
            reason: "contains Bearer abc.def.ghi".to_owned(),
        }],
    };

    let error =
        HighAssuranceApprovalControl::new(definition).expect_err("secret-like redaction metadata");

    assert_eq!(error.code(), "high_assurance_approval.secret_like_value");
    assert!(!error.to_string().contains("authorization_token"));
    assert!(!error.to_string().contains("abc.def.ghi"));
}

#[test]
fn debug_output_does_not_leak_secret_like_values() {
    let control = valid_control();
    let debug = format!("{control:?}");

    assert!(!debug.contains("approval-control/nuclear-key"));
    assert!(!debug.contains("evidence/context"));
    assert!(!debug.contains("approval_context"));
    assert!(debug.contains("protected_action_count"));
    assert!(debug.contains("[REDACTED]"));
}

#[test]
fn serialization_does_not_include_forbidden_raw_payload_fields() {
    let serialized = serde_json::to_string(&valid_control()).expect("serialize control");

    for forbidden in [
        "raw_provider_payload",
        "raw_command_output",
        "raw_spec_contents",
        "authorization_header",
        "private_key",
        "api_token",
    ] {
        assert!(
            !serialized.contains(forbidden),
            "serialized control should not include forbidden payload marker {forbidden}"
        );
    }
}
