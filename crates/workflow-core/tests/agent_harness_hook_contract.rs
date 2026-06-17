#![allow(clippy::expect_used)]

//! `AgentHarnessHookContract` model tests.

use serde_json::json;
use workflow_core::{
    AgentHarnessHookContract, AgentHarnessHookContractDefinition, AgentHarnessHookContractId,
    AgentHarnessHookContractVersion, AgentHarnessHookFailureSemantics,
    AgentHarnessHookInputRequirement, AgentHarnessHookKind, AgentHarnessHookOutputRequirement,
    AgentHarnessHookSideEffectAllowance, RedactionDisposition, RedactionFieldState,
    RedactionMetadata, SchemaVersion, WorkReportRedactionPolicy, WorkReportSensitivity,
};

fn hook_contract_id() -> AgentHarnessHookContractId {
    AgentHarnessHookContractId::new("agent-harness/hooks/pre-validation")
        .expect("valid hook contract id")
}

fn hook_contract_version() -> AgentHarnessHookContractVersion {
    AgentHarnessHookContractVersion::new("v1").expect("valid hook contract version")
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
            reason: "bounded hook contract metadata".to_owned(),
        }],
    }
}

fn valid_definition() -> AgentHarnessHookContractDefinition {
    AgentHarnessHookContractDefinition {
        contract_id: hook_contract_id(),
        contract_version: hook_contract_version(),
        schema_version: schema_version(),
        hook_kind: AgentHarnessHookKind::BeforeValidation,
        purpose: "check planned work before validation begins".to_owned(),
        input_requirements: vec![
            AgentHarnessHookInputRequirement::new("planned_work", true).expect("valid input")
        ],
        output_requirements: vec![AgentHarnessHookOutputRequirement::new(
            "checkpoint_result",
            true,
        )
        .expect("valid output")],
        failure_semantics: vec![
            AgentHarnessHookFailureSemantics::FailClosed,
            AgentHarnessHookFailureSemantics::Escalate,
        ],
        side_effect_allowance: AgentHarnessHookSideEffectAllowance::Unsupported,
        sensitivity: WorkReportSensitivity::Confidential,
        redaction_policy: WorkReportRedactionPolicy::ReferenceOnly,
        redaction: redaction(),
    }
}

fn valid_contract() -> AgentHarnessHookContract {
    AgentHarnessHookContract::new(valid_definition()).expect("valid hook contract")
}

#[test]
fn valid_minimal_agent_harness_hook_contract() {
    let contract = valid_contract();

    assert_eq!(
        contract.contract_id().as_str(),
        "agent-harness/hooks/pre-validation"
    );
    assert_eq!(contract.contract_version().as_str(), "v1");
    assert_eq!(contract.schema_version().as_str(), "workflowos/v0");
    assert_eq!(contract.hook_kind(), AgentHarnessHookKind::BeforeValidation);
    assert_eq!(
        contract.purpose(),
        "check planned work before validation begins"
    );
    assert_eq!(contract.input_requirements().len(), 1);
    assert_eq!(contract.output_requirements().len(), 1);
    assert_eq!(contract.failure_semantics().len(), 2);
    assert_eq!(
        contract.side_effect_allowance(),
        AgentHarnessHookSideEffectAllowance::Unsupported
    );
}

#[test]
fn invalid_hook_id_rejected_without_leaking_value() {
    let error = AgentHarnessHookContractId::new("bad hook id").expect_err("invalid id");

    assert_eq!(
        error.code(),
        "agent_harness_hook.identifier.invalid_character"
    );
    assert!(!error.to_string().contains("bad hook id"));
}

#[test]
fn invalid_hook_version_rejected() {
    let error = AgentHarnessHookContractVersion::new("").expect_err("invalid version");

    assert_eq!(error.code(), "agent_harness_hook.identifier.empty");
}

#[test]
fn hook_kinds_are_model_vocabulary_only() {
    let kinds = [
        AgentHarnessHookKind::BeforePlanning,
        AgentHarnessHookKind::AfterPlanning,
        AgentHarnessHookKind::BeforeImplementation,
        AgentHarnessHookKind::AfterImplementation,
        AgentHarnessHookKind::BeforeValidation,
        AgentHarnessHookKind::AfterValidation,
        AgentHarnessHookKind::BeforeReview,
        AgentHarnessHookKind::AfterReview,
        AgentHarnessHookKind::BeforeReport,
        AgentHarnessHookKind::AfterReport,
    ];

    for kind in kinds {
        let mut definition = valid_definition();
        definition.hook_kind = kind;
        let contract = AgentHarnessHookContract::new(definition).expect("kind is representable");
        assert_eq!(contract.hook_kind(), kind);
    }
}

#[test]
fn empty_input_requirements_rejected() {
    let mut definition = valid_definition();
    definition.input_requirements.clear();

    let error = AgentHarnessHookContract::new(definition).expect_err("missing inputs");
    assert_eq!(error.code(), "agent_harness_hook.inputs.required");
}

#[test]
fn duplicate_input_requirements_rejected() {
    let mut definition = valid_definition();
    definition.input_requirements = vec![
        AgentHarnessHookInputRequirement::new("planned_work", true).expect("valid input"),
        AgentHarnessHookInputRequirement::new("planned_work", false).expect("valid input"),
    ];

    let error = AgentHarnessHookContract::new(definition).expect_err("duplicate inputs");
    assert_eq!(error.code(), "agent_harness_hook.inputs.duplicate");
}

#[test]
fn empty_output_requirements_rejected() {
    let mut definition = valid_definition();
    definition.output_requirements.clear();

    let error = AgentHarnessHookContract::new(definition).expect_err("missing outputs");
    assert_eq!(error.code(), "agent_harness_hook.outputs.required");
}

#[test]
fn duplicate_output_requirements_rejected() {
    let mut definition = valid_definition();
    definition.output_requirements = vec![
        AgentHarnessHookOutputRequirement::new("checkpoint_result", true).expect("valid output"),
        AgentHarnessHookOutputRequirement::new("checkpoint_result", false).expect("valid output"),
    ];

    let error = AgentHarnessHookContract::new(definition).expect_err("duplicate outputs");
    assert_eq!(error.code(), "agent_harness_hook.outputs.duplicate");
}

#[test]
fn failure_semantics_validate_and_reject_duplicates() {
    let mut definition = valid_definition();
    definition.failure_semantics = vec![
        AgentHarnessHookFailureSemantics::WarningOnly,
        AgentHarnessHookFailureSemantics::WarningOnly,
    ];

    let error = AgentHarnessHookContract::new(definition).expect_err("duplicate failures");
    assert_eq!(error.code(), "agent_harness_hook.failure.duplicate");
}

#[test]
fn side_effect_authorization_is_rejected_in_model_only_phase() {
    let mut definition = valid_definition();
    definition.side_effect_allowance = AgentHarnessHookSideEffectAllowance::ProposedOnly;

    let error =
        AgentHarnessHookContract::new(definition).expect_err("side effects are out of scope");
    assert_eq!(error.code(), "agent_harness_hook.side_effect.unsupported");
}

#[test]
fn no_side_effect_allowance_is_representable_without_write_support() {
    let mut definition = valid_definition();
    definition.side_effect_allowance = AgentHarnessHookSideEffectAllowance::None;

    let contract = AgentHarnessHookContract::new(definition).expect("no side effects valid");
    assert_eq!(
        contract.side_effect_allowance(),
        AgentHarnessHookSideEffectAllowance::None
    );
}

#[test]
fn sensitivity_and_redaction_policy_validate() {
    let mut value = serde_json::to_value(valid_contract()).expect("serialize contract");
    value["sensitivity"] = json!("regulated");
    value["redaction_policy"] = json!("bounded_summaries");

    let contract: AgentHarnessHookContract =
        serde_json::from_value(value).expect("deserialize contract");
    assert_eq!(contract.sensitivity(), WorkReportSensitivity::Regulated);
    assert_eq!(
        contract.redaction_policy(),
        WorkReportRedactionPolicy::BoundedSummaries
    );
}

#[test]
fn serde_round_trip_for_valid_hook_contract() {
    let contract = valid_contract();
    let serialized = serde_json::to_string(&contract).expect("serialize contract");
    let deserialized: AgentHarnessHookContract =
        serde_json::from_str(&serialized).expect("deserialize contract");

    assert_eq!(deserialized, contract);
}

#[test]
fn invalid_serialized_hook_contract_fails_closed() {
    let mut value = serde_json::to_value(valid_contract()).expect("serialize contract");
    value["output_requirements"] = json!([]);

    let error = serde_json::from_value::<AgentHarnessHookContract>(value)
        .expect_err("empty outputs fail deserialization");
    assert!(error
        .to_string()
        .contains("agent_harness_hook.outputs.required"));
}

#[test]
fn deserialization_error_does_not_leak_secret_like_value() {
    let mut value = serde_json::to_value(valid_contract()).expect("serialize contract");
    value["purpose"] = json!("authorization bearer token value");

    let error = serde_json::from_value::<AgentHarnessHookContract>(value)
        .expect_err("secret-like purpose fails deserialization");
    assert!(error
        .to_string()
        .contains("agent_harness_hook.secret_like_value"));
    assert!(!error
        .to_string()
        .contains("authorization bearer token value"));
}

#[test]
fn redaction_metadata_is_validated_and_non_leaking() {
    let mut definition = valid_definition();
    definition.redaction = RedactionMetadata {
        redacted_fields: vec!["api_token".to_owned()],
        field_states: vec![],
    };

    let error = AgentHarnessHookContract::new(definition).expect_err("secret redaction field");
    assert_eq!(error.code(), "agent_harness_hook.secret_like_value");
    assert!(!error.to_string().contains("api_token"));
}

#[test]
fn raw_payload_markers_are_rejected_without_leaking_values() {
    let mut definition = valid_definition();
    definition.purpose = "copy raw_provider_payload into hook output".to_owned();

    let error = AgentHarnessHookContract::new(definition).expect_err("raw payload marker");
    assert_eq!(error.code(), "agent_harness_hook.secret_like_value");
    assert!(!error.to_string().contains("raw_provider_payload"));
}

#[test]
fn debug_output_does_not_leak_user_supplied_contract_text() {
    let contract = valid_contract();
    let debug = format!("{contract:?}");

    assert!(!debug.contains("check planned work before validation begins"));
    assert!(!debug.contains("planned_work"));
    assert!(!debug.contains("checkpoint_result"));
    assert!(!debug.contains("bounded hook contract metadata"));
}

#[test]
fn serialization_does_not_include_forbidden_raw_payload_fields() {
    let serialized = serde_json::to_string(&valid_contract()).expect("serialize contract");

    assert!(!serialized.contains("raw_provider_payload"));
    assert!(!serialized.contains("raw_command_output"));
    assert!(!serialized.contains("raw_spec_contents"));
    assert!(!serialized.contains("raw_parser_payload"));
    assert!(!serialized.contains("authorization"));
    assert!(!serialized.contains("private_key"));
}

#[test]
fn hook_contract_model_does_not_create_runtime_hook_behavior() {
    let serialized = serde_json::to_string(&valid_contract()).expect("serialize contract");

    assert!(!serialized.contains("execute_hook"));
    assert!(!serialized.contains("append_event"));
    assert!(!serialized.contains("state_backend"));
    assert!(!serialized.contains("cli_command"));
    assert!(!serialized.contains("workflow_schema"));
}
