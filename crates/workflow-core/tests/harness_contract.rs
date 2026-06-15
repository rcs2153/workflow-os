#![allow(clippy::expect_used)]

//! `HarnessContract` core model tests.

use serde_json::json;
use workflow_core::{
    HarnessApprovalRequirement, HarnessAuthorityScope, HarnessContextRequirement, HarnessContract,
    HarnessContractDefinition, HarnessContractId, HarnessContractVersion,
    HarnessEvidenceRequirement, HarnessExecutionPolicy, HarnessFailureSemantics,
    HarnessHandoffRequirement, HarnessInputRequirement, HarnessOutputRequirement,
    HarnessSideEffectAllowance, HarnessToolAllowance, HarnessToolKind, RedactionDisposition,
    RedactionFieldState, RedactionMetadata, SchemaVersion, WorkReportRedactionPolicy,
    WorkReportSensitivity,
};

fn contract_id() -> HarnessContractId {
    HarnessContractId::new("harness/spec-review").expect("valid harness id")
}

fn contract_version() -> HarnessContractVersion {
    HarnessContractVersion::new("v1").expect("valid harness version")
}

fn schema_version() -> SchemaVersion {
    SchemaVersion::new("workflowos/v0").expect("valid schema version")
}

fn redaction() -> RedactionMetadata {
    RedactionMetadata {
        redacted_fields: vec!["purpose".to_owned()],
        field_states: vec![RedactionFieldState {
            field: "purpose".to_owned(),
            disposition: RedactionDisposition::ReferenceOnly,
            reason: "bounded contract metadata".to_owned(),
        }],
    }
}

fn valid_definition() -> HarnessContractDefinition {
    HarnessContractDefinition {
        contract_id: contract_id(),
        contract_version: contract_version(),
        schema_version: schema_version(),
        purpose: "review workflow specification inputs and produce governed handoff".to_owned(),
        input_requirements: vec![
            HarnessInputRequirement::new("workflow_spec", true).expect("valid input")
        ],
        context_requirements: vec![
            HarnessContextRequirement::new("engineering_standard", true).expect("valid context")
        ],
        tool_allowances: vec![
            HarnessToolAllowance::new("docs_check", HarnessToolKind::LocalCheck)
                .expect("valid tool"),
        ],
        authority_scopes: vec![
            HarnessAuthorityScope::ReadContext,
            HarnessAuthorityScope::RunLocalChecks,
            HarnessAuthorityScope::GenerateReports,
        ],
        side_effect_allowance: HarnessSideEffectAllowance::Unsupported,
        output_requirements: vec![
            HarnessOutputRequirement::new("typed_handoff", true).expect("valid output")
        ],
        evidence_requirements: vec![
            HarnessEvidenceRequirement::new("source_references", true).expect("valid evidence")
        ],
        approval_requirements: vec![
            HarnessApprovalRequirement::new("human_review", false).expect("valid approval")
        ],
        execution_policy: HarnessExecutionPolicy::conservative(),
        failure_semantics: vec![
            HarnessFailureSemantics::ProduceBlockedHandoff,
            HarnessFailureSemantics::Escalate,
        ],
        handoff_requirements: vec![
            HarnessHandoffRequirement::new("next_obligations", true).expect("valid handoff")
        ],
        sensitivity: WorkReportSensitivity::Confidential,
        redaction_policy: WorkReportRedactionPolicy::ReferenceOnly,
        redaction: redaction(),
    }
}

fn valid_contract() -> HarnessContract {
    HarnessContract::new(valid_definition()).expect("valid harness contract")
}

#[test]
fn valid_minimal_harness_contract() {
    let contract = valid_contract();

    assert_eq!(contract.contract_id().as_str(), "harness/spec-review");
    assert_eq!(contract.contract_version().as_str(), "v1");
    assert_eq!(contract.schema_version().as_str(), "workflowos/v0");
    assert_eq!(
        contract.purpose(),
        "review workflow specification inputs and produce governed handoff"
    );
    assert_eq!(contract.input_requirements().len(), 1);
    assert_eq!(contract.context_requirements().len(), 1);
    assert_eq!(contract.tool_allowances().len(), 1);
    assert_eq!(contract.output_requirements().len(), 1);
    assert_eq!(contract.evidence_requirements().len(), 1);
    assert_eq!(contract.approval_requirements().len(), 1);
    assert_eq!(contract.handoff_requirements().len(), 1);
    assert_eq!(contract.sensitivity(), WorkReportSensitivity::Confidential);
    assert_eq!(
        contract.redaction_policy(),
        WorkReportRedactionPolicy::ReferenceOnly
    );
}

#[test]
fn invalid_harness_id_rejected() {
    let error = HarnessContractId::new("bad id").expect_err("invalid id");

    assert_eq!(
        error.code(),
        "harness_contract.identifier.invalid_character"
    );
    assert!(!error.to_string().contains("bad id"));
}

#[test]
fn invalid_version_rejected() {
    let error = HarnessContractVersion::new("").expect_err("invalid version");

    assert_eq!(error.code(), "harness_contract.identifier.empty");
}

#[test]
fn missing_purpose_rejected() {
    let mut definition = valid_definition();
    definition.purpose.clear();

    let error = HarnessContract::new(definition).expect_err("missing purpose");
    assert_eq!(error.code(), "harness_contract.text.empty");
}

#[test]
fn empty_input_requirements_rejected() {
    let mut definition = valid_definition();
    definition.input_requirements.clear();

    let error = HarnessContract::new(definition).expect_err("missing inputs");
    assert_eq!(error.code(), "harness_contract.inputs.required");
}

#[test]
fn required_context_validates() {
    let contract = valid_contract();

    assert_eq!(
        contract.context_requirements()[0].name(),
        "engineering_standard"
    );
    assert!(contract.context_requirements()[0].required());
}

#[test]
fn authority_scope_validates_and_rejects_duplicates() {
    let mut definition = valid_definition();
    definition.authority_scopes = vec![
        HarnessAuthorityScope::ReadContext,
        HarnessAuthorityScope::ReadContext,
    ];

    let error = HarnessContract::new(definition).expect_err("duplicate authority");
    assert_eq!(error.code(), "harness_contract.authority.duplicate");
}

#[test]
fn side_effect_allowance_is_representable_without_write_support() {
    let mut definition = valid_definition();
    definition.side_effect_allowance = HarnessSideEffectAllowance::ProposedOnly;

    let contract = HarnessContract::new(definition).expect("proposed-only side effects valid");
    assert_eq!(
        contract.side_effect_allowance(),
        HarnessSideEffectAllowance::ProposedOnly
    );
    assert!(!serde_json::to_string(&contract)
        .expect("serialize contract")
        .contains("write_capable"));
}

#[test]
fn evidence_requirement_validates() {
    let contract = valid_contract();

    assert_eq!(
        contract.evidence_requirements()[0].name(),
        "source_references"
    );
    assert!(contract.evidence_requirements()[0].required());
}

#[test]
fn approval_requirement_validates() {
    let contract = valid_contract();

    assert_eq!(contract.approval_requirements()[0].name(), "human_review");
    assert!(!contract.approval_requirements()[0].required());
}

#[test]
fn failure_semantics_validate_and_reject_duplicates() {
    let mut definition = valid_definition();
    definition.failure_semantics = vec![
        HarnessFailureSemantics::Escalate,
        HarnessFailureSemantics::Escalate,
    ];

    let error = HarnessContract::new(definition).expect_err("duplicate failures");
    assert_eq!(error.code(), "harness_contract.failure.duplicate");
}

#[test]
fn handoff_requirement_validates() {
    let contract = valid_contract();

    assert_eq!(
        contract.handoff_requirements()[0].name(),
        "next_obligations"
    );
    assert!(contract.handoff_requirements()[0].required());
}

#[test]
fn sensitivity_and_redaction_policy_validate() {
    let mut value = serde_json::to_value(valid_contract()).expect("serialize contract");
    value["sensitivity"] = json!("regulated");
    value["redaction_policy"] = json!("bounded_summaries");

    let contract: HarnessContract = serde_json::from_value(value).expect("deserialize contract");
    assert_eq!(contract.sensitivity(), WorkReportSensitivity::Regulated);
    assert_eq!(
        contract.redaction_policy(),
        WorkReportRedactionPolicy::BoundedSummaries
    );
}

#[test]
fn serde_round_trip_for_valid_contract() {
    let contract = valid_contract();
    let serialized = serde_json::to_string(&contract).expect("serialize contract");
    let deserialized: HarnessContract =
        serde_json::from_str(&serialized).expect("deserialize contract");

    assert_eq!(deserialized, contract);
}

#[test]
fn invalid_serialized_contract_fails_closed() {
    let mut value = serde_json::to_value(valid_contract()).expect("serialize contract");
    value["input_requirements"] = json!([]);

    let error = serde_json::from_value::<HarnessContract>(value)
        .expect_err("empty inputs fail deserialization");
    assert!(error
        .to_string()
        .contains("harness_contract.inputs.required"));
}

#[test]
fn debug_output_does_not_leak_secret_like_values() {
    let secret_like = "token-should-not-appear";
    let id_error = HarnessContractId::new(secret_like).expect_err("secret-like id fails");
    assert_eq!(id_error.code(), "harness_contract.secret_like_identifier");
    assert!(!id_error.to_string().contains(secret_like));

    let debug = format!("{:?}", valid_contract());
    assert!(!debug.contains("workflow specification"));
    assert!(!debug.contains("engineering_standard"));
    assert!(!debug.contains("docs_check"));
    assert!(!debug.contains("source_references"));
    assert!(!debug.contains("token"));
    assert!(!debug.contains("authorization"));
    assert!(!debug.contains("private"));
}

#[test]
fn serialization_does_not_leak_forbidden_raw_payload_fields() {
    let serialized = serde_json::to_string(&valid_contract()).expect("serialize contract");

    assert!(!serialized.contains("provider_payload"));
    assert!(!serialized.contains("authorization"));
    assert!(!serialized.contains("private_key"));
    assert!(!serialized.contains("raw_command_output"));
    assert!(!serialized.contains("raw_spec_contents"));
    assert!(!serialized.contains("env_value"));
    assert!(!serialized.contains("credential"));
}

#[test]
fn no_runtime_execution_behavior_is_introduced() {
    let serialized = serde_json::to_string(&valid_contract()).expect("serialize contract");

    assert!(!serialized.contains("nested_execution"));
    assert!(!serialized.contains("runtime_schedule"));
    assert!(!serialized.contains("spawn"));
    assert!(!serialized.contains("swarm"));
}

#[test]
fn no_domain_specific_sections_are_required_by_core() {
    let serialized = serde_json::to_string(&valid_contract()).expect("serialize contract");

    assert!(!serialized.contains("pull_request"));
    assert!(!serialized.contains("jira"));
    assert!(!serialized.contains("legal_clause"));
    assert!(!serialized.contains("finance_exception"));
}
