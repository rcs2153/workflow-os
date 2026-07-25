#![allow(clippy::expect_used, clippy::unwrap_used)]

//! Focused model tests for immutable pre-execution local-check bindings.

use serde_json::{json, Value};
use workflow_core::{
    compute_local_check_command_contract_fingerprint, ImmutableLocalCheckExecutionBinding,
    ImmutableLocalCheckExecutionBindingAlgorithm, ImmutableLocalCheckExecutionBindingDefinition,
    ImmutableLocalCheckHandlerPosture, ImmutableLocalCheckHandlerRegistrationMode,
    ImmutableLocalCheckHandlerSelection, ImmutableRunBundleBinding, LocalCheckCommandContract,
    LocalCheckCommandContractDefinition, LocalCheckCommandId, LocalCheckCommandKind,
    LocalCheckEnvironmentPolicy, LocalCheckExecutionPosture, LocalCheckNetworkPolicy,
    LocalCheckOutputCapturePolicy, LocalCheckRedactionPolicy, LocalCheckSideEffectClass,
    LocalCheckWorkingDirectoryPolicy, SkillId, SkillVersion, SpecContentHash, StepId, Timestamp,
    WorkReportCitationKind, WorkflowId, WorkflowRunId,
};

fn bundle() -> ImmutableRunBundleBinding {
    serde_json::from_value(json!({
        "bundle_id": "bundle/test",
        "bundle_version": "v1",
        "root_hash": SpecContentHash::from_text("bundle-root").as_str(),
    }))
    .unwrap()
}

fn docs_contract_with_order(
    environment: Vec<&str>,
    citations: Vec<WorkReportCitationKind>,
) -> LocalCheckCommandContract {
    LocalCheckCommandContract::new(LocalCheckCommandContractDefinition {
        command_id: LocalCheckCommandId::new("local-check/docs").unwrap(),
        command_kind: LocalCheckCommandKind::DocsCheck,
        execution_posture: LocalCheckExecutionPosture::ModelOnly,
        executable: "npm".to_owned(),
        arguments: vec!["run".to_owned(), "check:docs".to_owned()],
        working_directory_policy: LocalCheckWorkingDirectoryPolicy::RepositoryRoot,
        environment_policy: LocalCheckEnvironmentPolicy::ExplicitAllowlistOnly,
        allowed_environment_variables: environment.into_iter().map(str::to_owned).collect(),
        network_policy: LocalCheckNetworkPolicy::Disabled,
        timeout_seconds: 120,
        side_effect_class: LocalCheckSideEffectClass::NoSourceWrites,
        permitted_output_directories: Vec::new(),
        output_capture: LocalCheckOutputCapturePolicy::bounded(16 * 1024, 16 * 1024),
        redaction_policy: LocalCheckRedactionPolicy::BoundedRedactedSummary,
        citation_kinds: citations,
    })
    .unwrap()
}

fn handler(command_kind: LocalCheckCommandKind) -> ImmutableLocalCheckHandlerSelection {
    ImmutableLocalCheckHandlerSelection::registered_unattested(
        command_kind,
        SkillId::new("local/check-docs").unwrap(),
        SkillVersion::new("v0").unwrap(),
    )
}

fn binding_for(contract: &LocalCheckCommandContract) -> ImmutableLocalCheckExecutionBinding {
    ImmutableLocalCheckExecutionBinding::new(ImmutableLocalCheckExecutionBindingDefinition {
        immutable_run_bundle: bundle(),
        workflow_id: WorkflowId::new("workflow/test").unwrap(),
        run_id: WorkflowRunId::new("run-test").unwrap(),
        step_id: StepId::new("check-docs").unwrap(),
        skill_id: SkillId::new("local/check-docs").unwrap(),
        skill_version: SkillVersion::new("v0").unwrap(),
        command_contract: contract,
        handler_selection: handler(LocalCheckCommandKind::DocsCheck),
        created_at: Timestamp::parse_rfc3339("2026-07-19T12:00:00Z").unwrap(),
    })
    .unwrap()
}

#[test]
fn valid_binding_is_payload_free_and_round_trips() {
    let contract = LocalCheckCommandContract::docs_check_model_only().unwrap();
    let binding = binding_for(&contract);

    assert_eq!(
        binding.algorithm(),
        ImmutableLocalCheckExecutionBindingAlgorithm::V1
    );
    assert_eq!(
        binding.handler_selection().posture(),
        ImmutableLocalCheckHandlerPosture::RegisteredUnattested
    );
    assert_eq!(
        binding.handler_selection().registration_mode(),
        ImmutableLocalCheckHandlerRegistrationMode::ExplicitProfile
    );
    assert_eq!(
        binding.command_contract_fingerprint(),
        &compute_local_check_command_contract_fingerprint(&contract)
    );
    assert_eq!(
        binding.command_contract_fingerprint().as_str(),
        "36f49eaf2768a1ffbc5bc0f57fd7da5a9c0da426e41e0bf8689b11c5b902154f"
    );
    assert_eq!(
        binding.binding_fingerprint().as_str(),
        "63542cf753fdf42dad1bebe4f35fcb8fe878f086a99dae762807719f40dbc4d2"
    );

    let encoded = serde_json::to_string(&binding).unwrap();
    let decoded: ImmutableLocalCheckExecutionBinding = serde_json::from_str(&encoded).unwrap();
    assert_eq!(decoded, binding);
    for forbidden in ["raw_output", "stdout", "stderr", "environment_values"] {
        assert!(!encoded.contains(forbidden));
    }
}

#[test]
fn command_fingerprint_canonicalizes_unordered_sets() {
    let first = docs_contract_with_order(
        vec!["NPM_CONFIG_CACHE", "CI"],
        vec![
            WorkReportCitationKind::WorkflowEvent,
            WorkReportCitationKind::ValidationDiagnostic,
        ],
    );
    let second = docs_contract_with_order(
        vec!["CI", "NPM_CONFIG_CACHE"],
        vec![
            WorkReportCitationKind::ValidationDiagnostic,
            WorkReportCitationKind::WorkflowEvent,
        ],
    );

    assert_eq!(
        compute_local_check_command_contract_fingerprint(&first),
        compute_local_check_command_contract_fingerprint(&second)
    );
}

#[test]
fn distinct_command_contracts_have_distinct_fingerprints() {
    let docs = LocalCheckCommandContract::docs_check_model_only().unwrap();
    let dogfood = LocalCheckCommandContract::dogfood_validate_model_only().unwrap();
    assert_ne!(
        compute_local_check_command_contract_fingerprint(&docs),
        compute_local_check_command_contract_fingerprint(&dogfood)
    );
}

#[test]
fn handler_selection_must_match_resolved_skill_and_command() {
    let contract = LocalCheckCommandContract::docs_check_model_only().unwrap();
    let error =
        ImmutableLocalCheckExecutionBinding::new(ImmutableLocalCheckExecutionBindingDefinition {
            immutable_run_bundle: bundle(),
            workflow_id: WorkflowId::new("workflow/test").unwrap(),
            run_id: WorkflowRunId::new("run-test").unwrap(),
            step_id: StepId::new("check-docs").unwrap(),
            skill_id: SkillId::new("local/other").unwrap(),
            skill_version: SkillVersion::new("v0").unwrap(),
            command_contract: &contract,
            handler_selection: handler(LocalCheckCommandKind::DocsCheck),
            created_at: Timestamp::parse_rfc3339("2026-07-19T12:00:00Z").unwrap(),
        })
        .unwrap_err();
    assert_eq!(
        error.code(),
        "immutable_local_check_execution_binding.handler_selection.skill_mismatch"
    );

    let error =
        ImmutableLocalCheckExecutionBinding::new(ImmutableLocalCheckExecutionBindingDefinition {
            immutable_run_bundle: bundle(),
            workflow_id: WorkflowId::new("workflow/test").unwrap(),
            run_id: WorkflowRunId::new("run-test").unwrap(),
            step_id: StepId::new("check-docs").unwrap(),
            skill_id: SkillId::new("local/check-docs").unwrap(),
            skill_version: SkillVersion::new("v0").unwrap(),
            command_contract: &contract,
            handler_selection: handler(LocalCheckCommandKind::CargoFmtCheck),
            created_at: Timestamp::parse_rfc3339("2026-07-19T12:00:00Z").unwrap(),
        })
        .unwrap_err();
    assert_eq!(
        error.code(),
        "immutable_local_check_execution_binding.handler_selection.command_mismatch"
    );
}

#[test]
fn serialized_binding_fails_closed_on_tampering_without_echoing_values() {
    let contract = LocalCheckCommandContract::docs_check_model_only().unwrap();
    let binding = binding_for(&contract);
    let mut value = serde_json::to_value(binding).unwrap();
    value["binding_fingerprint"] = Value::String(
        SpecContentHash::from_text("secret-token-marker")
            .as_str()
            .to_owned(),
    );
    let error = serde_json::from_value::<ImmutableLocalCheckExecutionBinding>(value).unwrap_err();
    assert_eq!(
        error.to_string(),
        "invalid immutable local check execution binding"
    );
    assert!(!error.to_string().contains("secret-token-marker"));
}

#[test]
fn handler_selection_tampering_fails_closed() {
    let contract = LocalCheckCommandContract::docs_check_model_only().unwrap();
    let binding = binding_for(&contract);
    let mut value = serde_json::to_value(binding).unwrap();
    value["handler_selection"]["selection_fingerprint"] =
        Value::String(SpecContentHash::from_text("changed").as_str().to_owned());
    let error = serde_json::from_value::<ImmutableLocalCheckExecutionBinding>(value).unwrap_err();
    assert_eq!(
        error.to_string(),
        "invalid immutable local check handler selection"
    );
}

#[test]
fn binding_identity_changes_with_run_or_creation_time() {
    let contract = LocalCheckCommandContract::docs_check_model_only().unwrap();
    let baseline = binding_for(&contract);
    let changed =
        ImmutableLocalCheckExecutionBinding::new(ImmutableLocalCheckExecutionBindingDefinition {
            immutable_run_bundle: bundle(),
            workflow_id: WorkflowId::new("workflow/test").unwrap(),
            run_id: WorkflowRunId::new("run-other").unwrap(),
            step_id: StepId::new("check-docs").unwrap(),
            skill_id: SkillId::new("local/check-docs").unwrap(),
            skill_version: SkillVersion::new("v0").unwrap(),
            command_contract: &contract,
            handler_selection: handler(LocalCheckCommandKind::DocsCheck),
            created_at: Timestamp::parse_rfc3339("2026-07-19T12:00:01Z").unwrap(),
        })
        .unwrap();
    assert_ne!(
        baseline.binding_fingerprint(),
        changed.binding_fingerprint()
    );
}

#[test]
fn debug_does_not_expose_bound_identifiers_or_fingerprints() {
    let contract = LocalCheckCommandContract::docs_check_model_only().unwrap();
    let debug = format!("{:?}", binding_for(&contract));
    for forbidden in [
        "workflow/test",
        "run-test",
        "check-docs",
        "local/check-docs",
        "bundle/test",
    ] {
        assert!(!debug.contains(forbidden));
    }
    assert!(debug.contains("[REDACTED]"));
}
