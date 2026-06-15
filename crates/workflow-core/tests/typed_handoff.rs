#![allow(clippy::expect_used)]

//! `TypedHandoff` core model tests.

use serde_json::json;
use workflow_core::{
    ApprovalReferenceId, EventId, EvidenceReferenceId, HarnessContractId, RedactionDisposition,
    RedactionFieldState, RedactionMetadata, SchemaVersion, TypedHandoff, TypedHandoffContract,
    TypedHandoffContractDefinition, TypedHandoffContractId, TypedHandoffContractVersion,
    TypedHandoffDefinition, TypedHandoffEndpoint, TypedHandoffEndpointKind,
    TypedHandoffFailureSemantics, TypedHandoffId, TypedHandoffReference,
    TypedHandoffReferenceTarget, TypedHandoffStatus, TypedHandoffTextItem, ValidationReferenceId,
    WorkReportId, WorkReportRedactionPolicy, WorkReportSensitivity, WorkReportStableReference,
    WorkflowId, WorkflowRunId,
};

fn schema_version() -> SchemaVersion {
    SchemaVersion::new("workflowos/v0").expect("valid schema version")
}

fn contract_id() -> TypedHandoffContractId {
    TypedHandoffContractId::new("handoff/spec-to-plan").expect("valid handoff contract id")
}

fn contract_version() -> TypedHandoffContractVersion {
    TypedHandoffContractVersion::new("v1").expect("valid handoff contract version")
}

fn handoff_id() -> TypedHandoffId {
    TypedHandoffId::new("handoff/run-1/spec-to-plan").expect("valid handoff id")
}

fn stable(value: &str) -> WorkReportStableReference {
    WorkReportStableReference::new(value).expect("valid stable reference")
}

fn redaction() -> RedactionMetadata {
    RedactionMetadata {
        redacted_fields: vec!["notes".to_owned()],
        field_states: vec![RedactionFieldState {
            field: "notes".to_owned(),
            disposition: RedactionDisposition::ReferenceOnly,
            reason: "bounded handoff metadata".to_owned(),
        }],
    }
}

fn source_endpoint() -> TypedHandoffEndpoint {
    TypedHandoffEndpoint::new(
        "spec_harness",
        TypedHandoffEndpointKind::Harness,
        stable("harness/spec"),
    )
    .expect("valid source endpoint")
}

fn target_endpoint() -> TypedHandoffEndpoint {
    TypedHandoffEndpoint::new(
        "planning_harness",
        TypedHandoffEndpointKind::Harness,
        stable("harness/planning"),
    )
    .expect("valid target endpoint")
}

fn input_ref() -> TypedHandoffReference {
    TypedHandoffReference::new(
        "workflow_spec",
        TypedHandoffReferenceTarget::Input {
            reference: stable("artifact/workflow-spec"),
        },
        true,
    )
    .expect("valid input reference")
}

fn output_ref() -> TypedHandoffReference {
    TypedHandoffReference::new(
        "planning_context",
        TypedHandoffReferenceTarget::Output {
            reference: stable("artifact/planning-context"),
        },
        true,
    )
    .expect("valid output reference")
}

fn evidence_ref() -> TypedHandoffReference {
    TypedHandoffReference::new(
        "source_evidence",
        TypedHandoffReferenceTarget::EvidenceReference {
            evidence_reference_id: EvidenceReferenceId::new("evidence/spec-file")
                .expect("valid evidence id"),
        },
        true,
    )
    .expect("valid evidence reference")
}

fn validation_ref() -> TypedHandoffReference {
    TypedHandoffReference::new(
        "schema_validation",
        TypedHandoffReferenceTarget::ValidationReference {
            validation_reference_id: ValidationReferenceId::new("validation/schema")
                .expect("valid validation id"),
        },
        true,
    )
    .expect("valid validation reference")
}

fn obligation() -> TypedHandoffTextItem {
    TypedHandoffTextItem::new("next_step", "produce bounded implementation plan")
        .expect("valid obligation")
}

fn valid_contract_definition() -> TypedHandoffContractDefinition {
    TypedHandoffContractDefinition {
        contract_id: contract_id(),
        contract_version: contract_version(),
        schema_version: schema_version(),
        source_harness_contract_id: Some(
            HarnessContractId::new("harness/spec").expect("valid source harness id"),
        ),
        target_harness_contract_id: Some(
            HarnessContractId::new("harness/planning").expect("valid target harness id"),
        ),
        required_input_references: vec![input_ref()],
        required_output_references: vec![output_ref()],
        required_evidence_references: vec![evidence_ref()],
        required_validation_references: vec![validation_ref()],
        required_obligations: vec![obligation()],
        failure_semantics: vec![
            TypedHandoffFailureSemantics::BlockTarget,
            TypedHandoffFailureSemantics::Escalate,
        ],
        sensitivity: WorkReportSensitivity::Confidential,
        redaction_policy: WorkReportRedactionPolicy::ReferenceOnly,
        redaction: redaction(),
    }
}

fn valid_contract() -> TypedHandoffContract {
    TypedHandoffContract::new(valid_contract_definition()).expect("valid contract")
}

fn valid_handoff_definition() -> TypedHandoffDefinition {
    TypedHandoffDefinition {
        handoff_id: handoff_id(),
        contract_id: contract_id(),
        contract_version: contract_version(),
        schema_version: schema_version(),
        workflow_id: Some(WorkflowId::new("workflow/software-delivery").expect("valid workflow")),
        run_id: Some(WorkflowRunId::new("run-typed-handoff").expect("valid run id")),
        status: TypedHandoffStatus::Ready,
        source: source_endpoint(),
        target: target_endpoint(),
        input_references: vec![input_ref()],
        output_references: vec![output_ref()],
        evidence_references: vec![evidence_ref()],
        validation_references: vec![validation_ref()],
        audit_references: vec![TypedHandoffReference::new(
            "audit_event",
            TypedHandoffReferenceTarget::AuditEvent {
                event_id: EventId::new("event/audit-1").expect("valid audit event id"),
            },
            false,
        )
        .expect("valid audit reference")],
        policy_references: vec![TypedHandoffReference::new(
            "policy_decision",
            TypedHandoffReferenceTarget::PolicyDecision {
                event_id: EventId::new("event/policy-1").expect("valid policy event id"),
            },
            false,
        )
        .expect("valid policy reference")],
        approval_references: vec![TypedHandoffReference::new(
            "approval_decision",
            TypedHandoffReferenceTarget::ApprovalDecision {
                approval_reference_id: ApprovalReferenceId::new("approval/review-1")
                    .expect("valid approval id"),
            },
            false,
        )
        .expect("valid approval reference")],
        work_report_references: vec![TypedHandoffReference::new(
            "terminal_report",
            TypedHandoffReferenceTarget::WorkReport {
                work_report_id: WorkReportId::new("work-report/final")
                    .expect("valid work report id"),
            },
            false,
        )
        .expect("valid work report reference")],
        obligations: vec![obligation()],
        disclosures: vec![TypedHandoffTextItem::new(
            "deferred_work",
            "approval evidence attachment remains deferred",
        )
        .expect("valid disclosure")],
        risks: vec![TypedHandoffTextItem::new(
            "risk",
            "downstream implementation still requires review",
        )
        .expect("valid risk")],
        notes: vec![
            TypedHandoffTextItem::new("operator_note", "bounded operator note")
                .expect("valid note"),
        ],
        sensitivity: WorkReportSensitivity::Confidential,
        redaction_policy: WorkReportRedactionPolicy::ReferenceOnly,
        redaction: redaction(),
    }
}

fn valid_handoff() -> TypedHandoff {
    TypedHandoff::new(valid_handoff_definition()).expect("valid typed handoff")
}

#[test]
fn valid_minimal_typed_handoff_contract() {
    let contract = valid_contract();

    assert_eq!(contract.contract_id().as_str(), "handoff/spec-to-plan");
    assert_eq!(contract.contract_version().as_str(), "v1");
    assert_eq!(contract.schema_version().as_str(), "workflowos/v0");
    assert_eq!(contract.required_input_references().len(), 1);
    assert_eq!(contract.required_output_references().len(), 1);
    assert_eq!(contract.required_evidence_references().len(), 1);
    assert_eq!(contract.required_validation_references().len(), 1);
    assert_eq!(contract.required_obligations().len(), 1);
    assert_eq!(contract.failure_semantics().len(), 2);
}

#[test]
fn valid_minimal_typed_handoff() {
    let handoff = valid_handoff();

    assert_eq!(handoff.handoff_id().as_str(), "handoff/run-1/spec-to-plan");
    assert_eq!(handoff.contract_id().as_str(), "handoff/spec-to-plan");
    assert_eq!(handoff.contract_version().as_str(), "v1");
    assert_eq!(handoff.status(), TypedHandoffStatus::Ready);
    assert_eq!(handoff.source().name(), "spec_harness");
    assert_eq!(handoff.target().name(), "planning_harness");
    assert_eq!(handoff.input_references().len(), 1);
    assert_eq!(handoff.output_references().len(), 1);
    assert_eq!(handoff.evidence_references().len(), 1);
    assert_eq!(handoff.validation_references().len(), 1);
    assert_eq!(handoff.obligations().len(), 1);
}

#[test]
fn invalid_handoff_id_rejected() {
    let error = TypedHandoffId::new("bad id").expect_err("invalid id");

    assert_eq!(error.code(), "typed_handoff.identifier.invalid_character");
    assert!(!error.to_string().contains("bad id"));
}

#[test]
fn invalid_contract_version_rejected() {
    let error = TypedHandoffContractVersion::new("").expect_err("invalid version");

    assert_eq!(error.code(), "typed_handoff.identifier.empty");
}

#[test]
fn missing_source_or_target_is_rejected() {
    let mut definition = valid_handoff_definition();
    definition.target = source_endpoint();

    let error = TypedHandoff::new(definition).expect_err("same endpoint rejected");
    assert_eq!(error.code(), "typed_handoff.endpoint.same");
}

#[test]
fn missing_required_reference_groups_are_rejected() {
    let mut definition = valid_handoff_definition();
    definition.evidence_references.clear();

    let error = TypedHandoff::new(definition).expect_err("missing evidence rejected");
    assert_eq!(error.code(), "typed_handoff.evidence.required");
}

#[test]
fn duplicate_references_are_rejected() {
    let mut definition = valid_handoff_definition();
    definition.output_references = vec![output_ref(), output_ref()];

    let error = TypedHandoff::new(definition).expect_err("duplicate refs rejected");
    assert_eq!(error.code(), "typed_handoff.references.duplicate");
}

#[test]
fn duplicate_text_items_are_rejected() {
    let mut definition = valid_handoff_definition();
    definition.obligations = vec![obligation(), obligation()];

    let error = TypedHandoff::new(definition).expect_err("duplicate obligations rejected");
    assert_eq!(error.code(), "typed_handoff.text_items.duplicate");
}

#[test]
fn reference_target_vocabulary_is_representable() {
    let targets = vec![
        TypedHandoffReferenceTarget::Input {
            reference: stable("input/ref"),
        },
        TypedHandoffReferenceTarget::Output {
            reference: stable("output/ref"),
        },
        TypedHandoffReferenceTarget::EvidenceReference {
            evidence_reference_id: EvidenceReferenceId::new("evidence/ref").expect("evidence"),
        },
        TypedHandoffReferenceTarget::ValidationReference {
            validation_reference_id: ValidationReferenceId::new("validation/ref")
                .expect("validation"),
        },
        TypedHandoffReferenceTarget::LocalCheckResult {
            reference: stable("local-check/result"),
        },
        TypedHandoffReferenceTarget::WorkflowEvent {
            event_id: EventId::new("event/workflow").expect("event"),
        },
        TypedHandoffReferenceTarget::AuditEvent {
            event_id: EventId::new("event/audit").expect("event"),
        },
        TypedHandoffReferenceTarget::PolicyDecision {
            event_id: EventId::new("event/policy").expect("event"),
        },
        TypedHandoffReferenceTarget::ApprovalDecision {
            approval_reference_id: ApprovalReferenceId::new("approval/ref").expect("approval"),
        },
        TypedHandoffReferenceTarget::WorkReport {
            work_report_id: WorkReportId::new("work-report/ref").expect("work report"),
        },
        TypedHandoffReferenceTarget::AdapterTelemetry {
            reference: stable("adapter/telemetry"),
        },
    ];

    assert_eq!(targets.len(), 11);
    assert_eq!(targets[0].kind_name(), "input");
    assert_eq!(targets[10].kind_name(), "adapter_telemetry");
}

#[test]
fn secret_like_notes_risks_limitations_and_references_are_rejected() {
    let note_error = TypedHandoffTextItem::new("note", "authorization bearer token")
        .expect_err("secret-like note rejected");
    assert_eq!(note_error.code(), "typed_handoff.secret_like_value");
    assert!(!note_error
        .to_string()
        .contains("authorization bearer token"));

    let ref_error = TypedHandoffReference::new(
        "api_token_ref",
        TypedHandoffReferenceTarget::Input {
            reference: stable("input/safe"),
        },
        true,
    )
    .expect_err("secret-like reference name rejected");
    assert_eq!(ref_error.code(), "typed_handoff.secret_like_value");
    assert!(!ref_error.to_string().contains("api_token_ref"));
}

#[test]
fn serde_round_trip_for_valid_handoff() {
    let handoff = valid_handoff();
    let serialized = serde_json::to_string(&handoff).expect("serialize handoff");
    let deserialized: TypedHandoff =
        serde_json::from_str(&serialized).expect("deserialize handoff");

    assert_eq!(deserialized, handoff);
}

#[test]
fn serde_round_trip_for_valid_contract() {
    let contract = valid_contract();
    let serialized = serde_json::to_string(&contract).expect("serialize contract");
    let deserialized: TypedHandoffContract =
        serde_json::from_str(&serialized).expect("deserialize contract");

    assert_eq!(deserialized, contract);
}

#[test]
fn invalid_serialized_handoff_fails_closed() {
    let mut value = serde_json::to_value(valid_handoff()).expect("serialize handoff");
    value["output_references"] = json!([]);

    let error =
        serde_json::from_value::<TypedHandoff>(value).expect_err("invalid handoff rejected");
    assert!(error.to_string().contains("typed_handoff.outputs.required"));
}

#[test]
fn invalid_serialized_contract_fails_closed() {
    let mut value = serde_json::to_value(valid_contract()).expect("serialize contract");
    value["required_obligations"] = json!([]);

    let error = serde_json::from_value::<TypedHandoffContract>(value)
        .expect_err("invalid contract rejected");
    assert!(error
        .to_string()
        .contains("typed_handoff.contract.obligations.required"));
}

#[test]
fn debug_output_is_redaction_safe() {
    let debug = format!("{:?} {:?}", valid_handoff(), valid_contract());

    assert!(!debug.contains("workflow_spec"));
    assert!(!debug.contains("planning_context"));
    assert!(!debug.contains("source_evidence"));
    assert!(!debug.contains("produce bounded"));
    assert!(!debug.contains("approval evidence"));
    assert!(!debug.contains("downstream implementation"));
    assert!(!debug.contains("bounded operator note"));
    assert!(!debug.contains("authorization"));
    assert!(!debug.contains("private_key"));
    assert!(!debug.contains("token"));
}

#[test]
fn serialization_does_not_leak_forbidden_raw_payload_fields() {
    let serialized = serde_json::to_string(&valid_handoff()).expect("serialize handoff");

    assert!(!serialized.contains("provider_payload"));
    assert!(!serialized.contains("authorization"));
    assert!(!serialized.contains("private_key"));
    assert!(!serialized.contains("raw_command_output"));
    assert!(!serialized.contains("raw_spec_contents"));
    assert!(!serialized.contains("parser_payload"));
    assert!(!serialized.contains("env_value"));
    assert!(!serialized.contains("credential"));
}

#[test]
fn redaction_metadata_rejects_secret_like_values_without_leaking() {
    let mut definition = valid_handoff_definition();
    definition.redaction = RedactionMetadata {
        redacted_fields: vec!["authorization_header".to_owned()],
        field_states: vec![],
    };

    let error = TypedHandoff::new(definition).expect_err("secret-like redaction rejected");
    assert_eq!(error.code(), "typed_handoff.secret_like_value");
    assert!(!error.to_string().contains("authorization_header"));
}

#[test]
fn no_runtime_schema_cli_persistence_or_write_behavior_is_introduced() {
    let serialized = serde_json::to_string(&valid_handoff()).expect("serialize handoff");

    assert!(!serialized.contains("nested_execution"));
    assert!(!serialized.contains("runtime_schedule"));
    assert!(!serialized.contains("workflow_schema"));
    assert!(!serialized.contains("cli_output"));
    assert!(!serialized.contains("persist"));
    assert!(!serialized.contains("write_capable"));
    assert!(!serialized.contains("spawn"));
    assert!(!serialized.contains("swarm"));
}
