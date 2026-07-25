#![allow(clippy::expect_used)]
//! Canonical local-check declaration-set resolver tests.

use workflow_core::{
    compute_local_check_command_contract_fingerprint, parse_workflow_spec_yaml,
    resolve_canonical_local_check_declaration_set, ImmutableRunBundleVersion,
    LocalCheckCommandContract, LocalCheckCommandContractDefinition,
    LocalCheckCommandContractInventory, LocalCheckCommandId, LocalCheckCommandKind,
    LocalCheckEnvironmentPolicy, LocalCheckExecutionPosture, LocalCheckNetworkPolicy,
    LocalCheckOutputCapturePolicy, LocalCheckRedactionPolicy, LocalCheckSideEffectClass,
    LocalCheckWorkingDirectoryPolicy, ResolveCanonicalLocalCheckDeclarationSetInput, StepId,
    WorkReportCitationKind, WorkflowDefinition,
};

fn workflow(requirements: &str) -> WorkflowDefinition {
    parse_workflow_spec_yaml(&format!(
        r"
schema_version: workflowos.dev/v0
id: local/check-declarations
version: v1
display_name: Local check declarations
steps:
  - id: verify
    skill_ref:
      id: local/verify
      version: v1
    local_check_requirements:
{requirements}
"
    ))
    .expect("valid workflow")
}

fn declaration(id: &str, command_id: &str, level: &str, side_effect: &str) -> String {
    format!(
        r"      - id: {id}
        command_id: {command_id}
        requirement_level: {level}
        minimum_assurance: kernel_observed_local_process
        accepted_statuses: [passed]
        freshness:
          mode: no_reuse
        exact_immutable_run_binding_required: true
        truncation_allowed: false
        network_maximum: disabled
        side_effect_maximum: {side_effect}"
    )
}

fn bundle_version() -> ImmutableRunBundleVersion {
    ImmutableRunBundleVersion::new("v1").expect("valid bundle version")
}

fn step_id() -> StepId {
    StepId::new("verify").expect("valid step id")
}

fn inventory(contracts: Vec<LocalCheckCommandContract>) -> LocalCheckCommandContractInventory {
    LocalCheckCommandContractInventory::new(contracts).expect("valid inventory")
}

fn resolve(
    workflow: &WorkflowDefinition,
    inventory: &LocalCheckCommandContractInventory,
) -> workflow_core::CanonicalLocalCheckDeclarationSetRecord {
    resolve_canonical_local_check_declaration_set(ResolveCanonicalLocalCheckDeclarationSetInput {
        workflow,
        step_id: &step_id(),
        command_inventory: inventory,
        immutable_bundle_version: bundle_version(),
    })
    .expect("resolution succeeds")
}

fn build_writing_docs_contract(
    command_id: &str,
    output_directory: &str,
) -> LocalCheckCommandContract {
    LocalCheckCommandContract::new(LocalCheckCommandContractDefinition {
        command_id: LocalCheckCommandId::new(command_id).expect("valid command id"),
        command_kind: LocalCheckCommandKind::DocsCheck,
        execution_posture: LocalCheckExecutionPosture::ModelOnly,
        executable: "npm".to_owned(),
        arguments: vec!["run".to_owned(), "check:docs".to_owned()],
        working_directory_policy: LocalCheckWorkingDirectoryPolicy::RepositoryRoot,
        environment_policy: LocalCheckEnvironmentPolicy::SanitizedMinimal,
        allowed_environment_variables: vec!["NPM_CONFIG_CACHE".to_owned()],
        network_policy: LocalCheckNetworkPolicy::Disabled,
        timeout_seconds: 120,
        side_effect_class: LocalCheckSideEffectClass::BuildOrCacheWrites,
        permitted_output_directories: vec![output_directory.to_owned()],
        output_capture: LocalCheckOutputCapturePolicy::bounded(16 * 1024, 16 * 1024),
        redaction_policy: LocalCheckRedactionPolicy::BoundedRedactedSummary,
        citation_kinds: vec![
            WorkReportCitationKind::ValidationDiagnostic,
            WorkReportCitationKind::WorkflowEvent,
            WorkReportCitationKind::AuditEvent,
        ],
    })
    .expect("valid writing docs contract")
}

#[test]
fn resolver_produces_canonical_content_addressed_declaration_set() {
    let docs = LocalCheckCommandContract::docs_check_model_only().expect("docs contract");
    let dogfood =
        LocalCheckCommandContract::dogfood_validate_model_only().expect("dogfood contract");
    let dogfood_requirement = declaration(
        "dogfood-required",
        "local-check/dogfood-validate",
        "required",
        "no_source_writes",
    );
    let docs_requirement = declaration(
        "docs-required",
        "local-check/docs",
        "required",
        "no_source_writes",
    );
    let requirements = format!("{dogfood_requirement}\n{docs_requirement}");
    let reversed_requirements = format!("{docs_requirement}\n{dogfood_requirement}");
    let unreferenced = build_writing_docs_contract(
        "local-check/unreferenced",
        ".workflow-os/unreferenced-cache",
    );
    let first = resolve(
        &workflow(&requirements),
        &inventory(vec![docs.clone(), dogfood.clone(), unreferenced]),
    );
    let second = resolve(
        &workflow(&reversed_requirements),
        &inventory(vec![dogfood, docs]),
    );

    assert_eq!(
        first.declaration_set_fingerprint(),
        second.declaration_set_fingerprint()
    );
    assert_eq!(first.declarations(), second.declarations());
    assert_eq!(first.workflow_id().as_str(), "local/check-declarations");
    assert_eq!(first.workflow_version().as_str(), "v1");
    assert_eq!(first.step_id().as_str(), "verify");
    assert_eq!(first.immutable_bundle_version().as_str(), "v1");
    assert_eq!(first.declarations().len(), 2);
}

#[test]
fn resolver_binds_command_contract_and_independent_requirement_fingerprints() {
    let contract = LocalCheckCommandContract::docs_check_model_only().expect("docs contract");
    let record = resolve(
        &workflow(&declaration(
            "docs-required",
            "local-check/docs",
            "required",
            "no_source_writes",
        )),
        &inventory(vec![contract.clone()]),
    );
    let resolved = &record.declarations()[0];

    assert_eq!(
        resolved.command_contract_fingerprint(),
        &compute_local_check_command_contract_fingerprint(&contract)
    );
    assert_eq!(resolved.command_kind(), LocalCheckCommandKind::DocsCheck);
    assert_eq!(resolved.requirement_id().as_str(), "docs-required");
    assert_eq!(resolved.command_id().as_str(), "local-check/docs");
    assert_ne!(
        resolved.command_contract_fingerprint(),
        resolved.attestation_requirement_fingerprint()
    );
}

#[test]
fn resolver_emits_authoritative_empty_record() {
    let record = resolve(&workflow("      []"), &inventory(Vec::new()));

    assert!(record.declarations().is_empty());
    assert_eq!(record.declaration_set_fingerprint().as_str().len(), 64);
}

#[test]
fn inventory_and_resolution_fail_closed_for_ambiguous_or_unknown_commands() {
    let docs = LocalCheckCommandContract::docs_check_model_only().expect("docs contract");
    let error = LocalCheckCommandContractInventory::new(vec![docs.clone(), docs])
        .expect_err("duplicate command identity rejected");
    assert_eq!(
        error.code(),
        "local_check.declaration_set.inventory.duplicate_command"
    );

    let source = workflow(&declaration(
        "missing-required",
        "local-check/missing",
        "required",
        "no_source_writes",
    ));
    let error = resolve_canonical_local_check_declaration_set(
        ResolveCanonicalLocalCheckDeclarationSetInput {
            workflow: &source,
            step_id: &step_id(),
            command_inventory: &inventory(Vec::new()),
            immutable_bundle_version: bundle_version(),
        },
    )
    .expect_err("unknown command rejected");
    assert_eq!(
        error.code(),
        "local_check.declaration_set.command.unresolved"
    );
    assert!(!error.to_string().contains("local-check/missing"));
}

#[test]
fn resolver_rejects_contract_that_exceeds_declared_side_effect_maximum() {
    let contract = build_writing_docs_contract("local-check/docs", ".workflow-os/cache");
    let source = workflow(&declaration(
        "docs-required",
        "local-check/docs",
        "required",
        "no_source_writes",
    ));
    let error = resolve_canonical_local_check_declaration_set(
        ResolveCanonicalLocalCheckDeclarationSetInput {
            workflow: &source,
            step_id: &step_id(),
            command_inventory: &inventory(vec![contract]),
            immutable_bundle_version: bundle_version(),
        },
    )
    .expect_err("broader contract rejected");

    assert_eq!(
        error.code(),
        "local_check.declaration_set.command.side_effect_exceeds_maximum"
    );
}

#[test]
fn resolver_requires_exact_step_and_rejects_duplicate_obligations() {
    let source = workflow(&declaration(
        "docs-required",
        "local-check/docs",
        "required",
        "no_source_writes",
    ));
    let contracts = inventory(vec![
        LocalCheckCommandContract::docs_check_model_only().expect("docs contract")
    ]);
    let missing = StepId::new("missing").expect("valid step id");
    let error = resolve_canonical_local_check_declaration_set(
        ResolveCanonicalLocalCheckDeclarationSetInput {
            workflow: &source,
            step_id: &missing,
            command_inventory: &contracts,
            immutable_bundle_version: bundle_version(),
        },
    )
    .expect_err("missing step rejected");
    assert_eq!(error.code(), "local_check.declaration_set.step.missing");

    let duplicate = [
        declaration(
            "docs-required",
            "local-check/docs",
            "required",
            "no_source_writes",
        ),
        declaration(
            "docs-alias",
            "local-check/docs",
            "optional",
            "no_source_writes",
        ),
    ]
    .join("\n");
    let error = resolve_canonical_local_check_declaration_set(
        ResolveCanonicalLocalCheckDeclarationSetInput {
            workflow: &workflow(&duplicate),
            step_id: &step_id(),
            command_inventory: &contracts,
            immutable_bundle_version: bundle_version(),
        },
    )
    .expect_err("duplicate semantic obligation rejected");
    assert_eq!(
        error.code(),
        "local_check.declaration_set.record.duplicate_command"
    );
}

#[test]
fn fingerprints_change_for_decision_relevant_contract_and_bundle_changes() {
    let source = workflow(&declaration(
        "docs-required",
        "local-check/docs",
        "required",
        "build_or_cache_writes",
    ));
    let first_contract = build_writing_docs_contract("local-check/docs", ".workflow-os/cache-a");
    let second_contract = build_writing_docs_contract("local-check/docs", ".workflow-os/cache-b");
    let first = resolve(&source, &inventory(vec![first_contract]));
    let second = resolve(&source, &inventory(vec![second_contract]));
    assert_ne!(
        first.declaration_set_fingerprint(),
        second.declaration_set_fingerprint()
    );

    let version_two = resolve_canonical_local_check_declaration_set(
        ResolveCanonicalLocalCheckDeclarationSetInput {
            workflow: &source,
            step_id: &step_id(),
            command_inventory: &inventory(vec![build_writing_docs_contract(
                "local-check/docs",
                ".workflow-os/cache-a",
            )]),
            immutable_bundle_version: ImmutableRunBundleVersion::new("v2")
                .expect("valid bundle version"),
        },
    )
    .expect("versioned resolution");
    assert_ne!(
        first.declaration_set_fingerprint(),
        version_two.declaration_set_fingerprint()
    );
}

#[test]
fn serde_recomputes_fingerprints_and_debug_redacts_identifiers() {
    let record = resolve(
        &workflow(&declaration(
            "docs-required",
            "local-check/docs",
            "required",
            "no_source_writes",
        )),
        &inventory(vec![
            LocalCheckCommandContract::docs_check_model_only().expect("docs contract")
        ]),
    );
    let serialized = serde_json::to_string(&record).expect("serialize record");
    let decoded: workflow_core::CanonicalLocalCheckDeclarationSetRecord =
        serde_json::from_str(&serialized).expect("deserialize valid record");
    assert_eq!(decoded, record);

    let mut nested_tamper: serde_json::Value =
        serde_json::from_str(&serialized).expect("parse serialized record");
    nested_tamper["declarations"][0]["requirement_level"] =
        serde_json::Value::String("optional".to_owned());
    let error = serde_json::from_value::<workflow_core::CanonicalLocalCheckDeclarationSetRecord>(
        nested_tamper,
    )
    .expect_err("tampered nested declaration rejected");
    assert_eq!(
        error.to_string(),
        "invalid canonical local check declaration set"
    );

    let mut value: serde_json::Value =
        serde_json::from_str(&serialized).expect("parse serialized record");
    value["declaration_set_fingerprint"] = serde_json::Value::String("0".repeat(64));
    let error =
        serde_json::from_value::<workflow_core::CanonicalLocalCheckDeclarationSetRecord>(value)
            .expect_err("tampered fingerprint rejected");
    assert_eq!(
        error.to_string(),
        "invalid canonical local check declaration set"
    );

    let debug = format!("{record:?}");
    assert!(!debug.contains("local/check-declarations"));
    assert!(!debug.contains("docs-required"));
    assert!(!debug.contains("local-check/docs"));
}

#[test]
fn canonical_record_excludes_executable_payload_fields() {
    let record = resolve(
        &workflow(&declaration(
            "docs-required",
            "local-check/docs",
            "required",
            "no_source_writes",
        )),
        &inventory(vec![
            LocalCheckCommandContract::docs_check_model_only().expect("docs contract")
        ]),
    );
    let serialized = serde_json::to_string(&record).expect("serialize record");

    for forbidden in [
        "\"executable\"",
        "\"arguments\"",
        "\"working_directory\"",
        "\"environment\"",
        "\"raw_output\"",
        "\"npm\"",
        "\"check:docs\"",
        "NPM_CONFIG_CACHE",
    ] {
        assert!(
            !serialized.contains(forbidden),
            "record leaked forbidden field or value: {forbidden}"
        );
    }
}
