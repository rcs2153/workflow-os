#![allow(clippy::expect_used)]

//! `WorkReportContract` core model tests.

use serde_json::json;
use workflow_core::{
    SchemaVersion, WorkReportCitationKind, WorkReportCitationRequirement, WorkReportContract,
    WorkReportContractDefinition, WorkReportContractId, WorkReportContractVersion,
    WorkReportDisclosureKind, WorkReportDisclosureRequirements, WorkReportRedactionPolicy,
    WorkReportSectionKind, WorkReportSectionRequirement, WorkReportSensitivity,
};

fn contract_id() -> WorkReportContractId {
    WorkReportContractId::new("governed/handoff").expect("valid contract id")
}

fn contract_version() -> WorkReportContractVersion {
    WorkReportContractVersion::new("v1").expect("valid contract version")
}

fn schema_version() -> SchemaVersion {
    SchemaVersion::new("workflowos/v0").expect("valid schema version")
}

fn required_sections() -> Vec<WorkReportSectionRequirement> {
    WorkReportSectionKind::v1_required_kinds()
        .into_iter()
        .map(WorkReportSectionRequirement::required)
        .collect()
}

fn citation_requirements() -> Vec<WorkReportCitationRequirement> {
    vec![
        WorkReportCitationRequirement::new(WorkReportCitationKind::EvidenceReference, true),
        WorkReportCitationRequirement::new(WorkReportCitationKind::WorkflowEvent, true),
        WorkReportCitationRequirement::new(WorkReportCitationKind::AuditEvent, true),
        WorkReportCitationRequirement::new(WorkReportCitationKind::ValidationDiagnostic, true),
        WorkReportCitationRequirement::new(WorkReportCitationKind::ApprovalDecision, false),
        WorkReportCitationRequirement::new(WorkReportCitationKind::AdapterTelemetry, false),
        WorkReportCitationRequirement::new(WorkReportCitationKind::TypedHandoff, false),
        WorkReportCitationRequirement::new(WorkReportCitationKind::AgentHarnessHook, false),
        WorkReportCitationRequirement::new(WorkReportCitationKind::PolicyDecision, true),
    ]
}

fn valid_contract() -> WorkReportContract {
    WorkReportContract::new(contract_definition(
        required_sections(),
        citation_requirements(),
        WorkReportDisclosureRequirements::v1_required(),
    ))
    .expect("valid work report contract")
}

fn contract_definition(
    required_sections: Vec<WorkReportSectionRequirement>,
    citation_requirements: Vec<WorkReportCitationRequirement>,
    disclosure_requirements: WorkReportDisclosureRequirements,
) -> WorkReportContractDefinition {
    WorkReportContractDefinition {
        contract_id: contract_id(),
        contract_version: contract_version(),
        schema_version: schema_version(),
        required_sections,
        citation_requirements,
        redaction_policy: WorkReportRedactionPolicy::ReferenceOnly,
        sensitivity: WorkReportSensitivity::Confidential,
        disclosure_requirements,
    }
}

#[test]
fn valid_minimal_work_report_contract() {
    let contract = WorkReportContract::v1(contract_id(), contract_version(), schema_version())
        .expect("valid v1 contract");

    assert_eq!(contract.contract_id().as_str(), "governed/handoff");
    assert_eq!(contract.contract_version().as_str(), "v1");
    assert_eq!(contract.schema_version().as_str(), "workflowos/v0");
    assert_eq!(
        contract.redaction_policy(),
        WorkReportRedactionPolicy::ReferenceOnly
    );
    assert_eq!(contract.sensitivity(), WorkReportSensitivity::Confidential);
    assert!(contract.incomplete_work_disclosure_required());
    assert!(contract.known_limitations_disclosure_required());
    assert!(contract.risks_disclosure_required());
    assert!(contract.side_effect_disclosure_required());
}

#[test]
fn required_identity_fields_are_accessible() {
    let contract = valid_contract();

    assert_eq!(contract.contract_id().as_str(), "governed/handoff");
    assert_eq!(contract.contract_version().as_str(), "v1");
    assert_eq!(contract.schema_version().as_str(), "workflowos/v0");
}

#[test]
fn invalid_contract_id_rejected() {
    let error = WorkReportContractId::new("bad id").expect_err("invalid id");

    assert_eq!(
        error.code(),
        "work_report_contract.identifier.invalid_character"
    );
    assert!(!error.to_string().contains("bad id"));
}

#[test]
fn invalid_version_rejected() {
    let error = WorkReportContractVersion::new("").expect_err("invalid version");

    assert_eq!(error.code(), "work_report_contract.identifier.empty");
}

#[test]
fn invalid_schema_version_rejected() {
    let payload = json!({
        "contract_id": "governed/handoff",
        "contract_version": "v1",
        "schema_version": "bad version",
        "required_sections": [{"kind": "work_performed"}],
        "citation_requirements": [],
        "redaction_policy": "reference_only",
        "sensitivity": "confidential",
        "disclosure_requirements": {
            "required": []
        }
    });

    let error = serde_json::from_value::<WorkReportContract>(payload)
        .expect_err("invalid schema version should fail");
    assert!(!error.to_string().contains("bad version"));
}

#[test]
fn empty_required_sections_rejected() {
    let error = WorkReportContract::new(contract_definition(
        Vec::new(),
        citation_requirements(),
        WorkReportDisclosureRequirements::new([]),
    ))
    .expect_err("empty sections should fail");

    assert_eq!(error.code(), "work_report_contract.sections.required");
}

#[test]
fn duplicate_section_kinds_rejected() {
    let error = WorkReportContract::new(contract_definition(
        vec![
            WorkReportSectionRequirement::required(WorkReportSectionKind::WorkPerformed),
            WorkReportSectionRequirement::required(WorkReportSectionKind::WorkPerformed),
        ],
        citation_requirements(),
        WorkReportDisclosureRequirements::new([]),
    ))
    .expect_err("duplicate sections should fail");

    assert_eq!(error.code(), "work_report_contract.sections.duplicate");
}

#[test]
fn all_required_v1_section_kinds_are_representable() {
    let kinds = WorkReportSectionKind::v1_required_kinds();

    assert_eq!(kinds.len(), 11);
    assert!(kinds.contains(&WorkReportSectionKind::WorkPerformed));
    assert!(kinds.contains(&WorkReportSectionKind::EvidenceConsidered));
    assert!(kinds.contains(&WorkReportSectionKind::DecisionsMade));
    assert!(kinds.contains(&WorkReportSectionKind::PolicyGatesEvaluated));
    assert!(kinds.contains(&WorkReportSectionKind::Approvals));
    assert!(kinds.contains(&WorkReportSectionKind::ValidationAndQualityChecks));
    assert!(kinds.contains(&WorkReportSectionKind::SideEffects));
    assert!(kinds.contains(&WorkReportSectionKind::IncompleteOrDeferredWork));
    assert!(kinds.contains(&WorkReportSectionKind::KnownLimitations));
    assert!(kinds.contains(&WorkReportSectionKind::Risks));
    assert!(kinds.contains(&WorkReportSectionKind::OperatorHandoffNotes));
}

#[test]
fn domain_specific_sections_are_not_required_by_core() {
    let serialized = serde_json::to_value(valid_contract()).expect("serialize contract");
    let sections = serialized["required_sections"]
        .as_array()
        .expect("sections array");
    let section_text = serde_json::to_string(sections).expect("section json");

    assert!(!section_text.contains("pull_request"));
    assert!(!section_text.contains("jira"));
    assert!(!section_text.contains("legal_clause"));
    assert!(!section_text.contains("finance_exception"));
}

#[test]
fn citation_requirements_validate() {
    let contract = valid_contract();

    assert!(contract
        .citation_requirements()
        .iter()
        .any(
            |requirement| requirement.kind == WorkReportCitationKind::EvidenceReference
                && requirement.required
        ));
    assert!(contract
        .citation_requirements()
        .iter()
        .any(
            |requirement| requirement.kind == WorkReportCitationKind::AgentHarnessHook
                && !requirement.required
        ));
}

#[test]
fn duplicate_citation_requirements_rejected() {
    let error = WorkReportContract::new(contract_definition(
        required_sections(),
        vec![
            WorkReportCitationRequirement::new(WorkReportCitationKind::EvidenceReference, true),
            WorkReportCitationRequirement::new(WorkReportCitationKind::EvidenceReference, false),
        ],
        WorkReportDisclosureRequirements::v1_required(),
    ))
    .expect_err("duplicate citation requirements should fail");

    assert_eq!(error.code(), "work_report_contract.citations.duplicate");
}

#[test]
fn redaction_policy_validates() {
    let mut value = serde_json::to_value(valid_contract()).expect("serialize contract");
    value["redaction_policy"] = json!("bounded_summaries");

    let contract: WorkReportContract =
        serde_json::from_value(value).expect("bounded summaries policy is valid");
    assert_eq!(
        contract.redaction_policy(),
        WorkReportRedactionPolicy::BoundedSummaries
    );
}

#[test]
fn sensitivity_validates() {
    let mut value = serde_json::to_value(valid_contract()).expect("serialize contract");
    value["sensitivity"] = json!("regulated");

    let contract: WorkReportContract =
        serde_json::from_value(value).expect("regulated sensitivity is valid");
    assert_eq!(contract.sensitivity(), WorkReportSensitivity::Regulated);
}

#[test]
fn incomplete_deferred_disclosure_requirement_validates() {
    let contract = valid_contract();
    assert!(contract.incomplete_work_disclosure_required());

    let sections = vec![WorkReportSectionRequirement::required(
        WorkReportSectionKind::WorkPerformed,
    )];
    let error = WorkReportContract::new(contract_definition(
        sections,
        citation_requirements(),
        WorkReportDisclosureRequirements::new([WorkReportDisclosureKind::IncompleteOrDeferredWork]),
    ))
    .expect_err("missing incomplete section should fail");
    assert_eq!(
        error.code(),
        "work_report_contract.incomplete_section_required"
    );
}

#[test]
fn known_limitations_disclosure_requirement_validates() {
    let contract = valid_contract();
    assert!(contract.known_limitations_disclosure_required());

    let sections = vec![WorkReportSectionRequirement::required(
        WorkReportSectionKind::WorkPerformed,
    )];
    let error = WorkReportContract::new(contract_definition(
        sections,
        citation_requirements(),
        WorkReportDisclosureRequirements::new([WorkReportDisclosureKind::KnownLimitations]),
    ))
    .expect_err("missing known limitations section should fail");
    assert_eq!(
        error.code(),
        "work_report_contract.known_limitations_section_required"
    );
}

#[test]
fn risk_disclosure_requirement_validates() {
    let contract = valid_contract();
    assert!(contract.risks_disclosure_required());

    let sections = vec![WorkReportSectionRequirement::required(
        WorkReportSectionKind::WorkPerformed,
    )];
    let error = WorkReportContract::new(contract_definition(
        sections,
        citation_requirements(),
        WorkReportDisclosureRequirements::new([WorkReportDisclosureKind::Risks]),
    ))
    .expect_err("missing risks section should fail");
    assert_eq!(error.code(), "work_report_contract.risks_section_required");
}

#[test]
fn side_effect_section_requirement_works_without_write_support() {
    let contract = valid_contract();
    assert!(contract.side_effect_disclosure_required());
    assert!(contract
        .required_sections()
        .iter()
        .any(|section| section.kind == WorkReportSectionKind::SideEffects));

    let sections = vec![WorkReportSectionRequirement::required(
        WorkReportSectionKind::WorkPerformed,
    )];
    let error = WorkReportContract::new(contract_definition(
        sections,
        citation_requirements(),
        WorkReportDisclosureRequirements::new([WorkReportDisclosureKind::SideEffects]),
    ))
    .expect_err("missing side-effect section should fail");
    assert_eq!(
        error.code(),
        "work_report_contract.side_effect_section_required"
    );
}

#[test]
fn serde_round_trip_for_valid_contract() {
    let contract = valid_contract();
    let serialized = serde_json::to_string(&contract).expect("serialize contract");
    let deserialized: WorkReportContract =
        serde_json::from_str(&serialized).expect("deserialize contract");

    assert_eq!(deserialized, contract);
}

#[test]
fn invalid_serialized_contract_fails_closed() {
    let mut value = serde_json::to_value(valid_contract()).expect("serialize contract");
    value["required_sections"] = json!([
        {"kind": "work_performed"},
        {"kind": "work_performed"}
    ]);

    let error = serde_json::from_value::<WorkReportContract>(value)
        .expect_err("duplicate sections fail deserialization");
    assert!(error
        .to_string()
        .contains("work_report_contract.sections.duplicate"));
}

#[test]
fn debug_output_does_not_leak_secret_like_values() {
    let secret_like = "token-should-not-appear";
    let id_error = WorkReportContractId::new(secret_like).expect_err("secret-like id fails");
    assert_eq!(
        id_error.code(),
        "work_report_contract.secret_like_identifier"
    );
    assert!(!id_error.to_string().contains(secret_like));

    let contract = valid_contract();
    let debug = format!("{contract:?}");
    assert!(!debug.contains("governed/handoff"));
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
    assert!(!serialized.contains("jira_description"));
    assert!(!serialized.contains("ci_log"));
    assert!(!serialized.contains("github_file_contents"));
    assert!(!serialized.contains("command_output"));
    assert!(!serialized.contains("env_value"));
    assert!(!serialized.contains("credential"));
}
