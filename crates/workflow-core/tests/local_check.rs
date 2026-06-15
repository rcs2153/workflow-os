#![allow(clippy::expect_used)]

//! Local validation/check command contract model tests.

use serde_json::json;
use workflow_core::{
    LocalCheckCommandContract, LocalCheckCommandContractDefinition, LocalCheckCommandId,
    LocalCheckCommandKind, LocalCheckEnvironmentPolicy, LocalCheckExecutionPosture,
    LocalCheckNetworkPolicy, LocalCheckOutputCapturePolicy, LocalCheckRedactionPolicy,
    LocalCheckResultStatus, LocalCheckSideEffectClass, LocalCheckWorkingDirectoryPolicy,
    WorkReportCitationKind,
};

fn command_id() -> LocalCheckCommandId {
    LocalCheckCommandId::new("local-check/docs").expect("valid local check id")
}

fn valid_definition() -> LocalCheckCommandContractDefinition {
    LocalCheckCommandContractDefinition {
        command_id: command_id(),
        command_kind: LocalCheckCommandKind::DocsCheck,
        execution_posture: LocalCheckExecutionPosture::ModelOnly,
        executable: "npm".to_owned(),
        arguments: vec!["run".to_owned(), "check:docs".to_owned()],
        working_directory_policy: LocalCheckWorkingDirectoryPolicy::RepositoryRoot,
        environment_policy: LocalCheckEnvironmentPolicy::SanitizedMinimal,
        allowed_environment_variables: Vec::new(),
        network_policy: LocalCheckNetworkPolicy::Disabled,
        timeout_seconds: 120,
        side_effect_class: LocalCheckSideEffectClass::NoSourceWrites,
        permitted_output_directories: Vec::new(),
        output_capture: LocalCheckOutputCapturePolicy::bounded(16 * 1024, 16 * 1024),
        redaction_policy: LocalCheckRedactionPolicy::BoundedRedactedSummary,
        citation_kinds: vec![
            WorkReportCitationKind::ValidationDiagnostic,
            WorkReportCitationKind::WorkflowEvent,
        ],
    }
}

fn valid_contract() -> LocalCheckCommandContract {
    LocalCheckCommandContract::new(valid_definition()).expect("valid local check contract")
}

fn definition_for_kind(
    command_kind: LocalCheckCommandKind,
    executable: &str,
    arguments: &[&str],
) -> LocalCheckCommandContractDefinition {
    LocalCheckCommandContractDefinition {
        command_id: command_id(),
        command_kind,
        execution_posture: LocalCheckExecutionPosture::ModelOnly,
        executable: executable.to_owned(),
        arguments: arguments
            .iter()
            .map(|argument| (*argument).to_owned())
            .collect(),
        working_directory_policy: LocalCheckWorkingDirectoryPolicy::RepositoryRoot,
        environment_policy: LocalCheckEnvironmentPolicy::SanitizedMinimal,
        allowed_environment_variables: Vec::new(),
        network_policy: LocalCheckNetworkPolicy::Disabled,
        timeout_seconds: 120,
        side_effect_class: LocalCheckSideEffectClass::NoSourceWrites,
        permitted_output_directories: Vec::new(),
        output_capture: LocalCheckOutputCapturePolicy::bounded(16 * 1024, 16 * 1024),
        redaction_policy: LocalCheckRedactionPolicy::BoundedRedactedSummary,
        citation_kinds: vec![WorkReportCitationKind::ValidationDiagnostic],
    }
}

#[test]
fn valid_model_only_local_check_contract() {
    let contract = valid_contract();

    assert_eq!(contract.command_id().as_str(), "local-check/docs");
    assert_eq!(contract.command_kind(), LocalCheckCommandKind::DocsCheck);
    assert_eq!(
        contract.execution_posture(),
        LocalCheckExecutionPosture::ModelOnly
    );
    assert_eq!(contract.executable(), "npm");
    assert_eq!(contract.arguments(), ["run", "check:docs"]);
    assert_eq!(
        contract.working_directory_policy(),
        LocalCheckWorkingDirectoryPolicy::RepositoryRoot
    );
    assert_eq!(contract.network_policy(), LocalCheckNetworkPolicy::Disabled);
    assert_eq!(
        contract.side_effect_class(),
        LocalCheckSideEffectClass::NoSourceWrites
    );
}

#[test]
fn dogfood_validate_contract_is_model_only_and_non_executing() {
    let contract =
        LocalCheckCommandContract::dogfood_validate_model_only().expect("valid dogfood contract");

    assert_eq!(
        contract.command_kind(),
        LocalCheckCommandKind::WorkflowOsValidateDogfood
    );
    assert_eq!(
        contract.execution_posture(),
        LocalCheckExecutionPosture::ModelOnly
    );
    assert_eq!(contract.executable(), "workflow-os");
    assert_eq!(contract.arguments().len(), 3);
}

#[test]
fn all_planned_command_kinds_are_representable() {
    let kinds = [
        LocalCheckCommandKind::WorkflowOsValidateDogfood,
        LocalCheckCommandKind::DocsCheck,
        LocalCheckCommandKind::CargoFmtCheck,
        LocalCheckCommandKind::CargoClippyWorkspace,
        LocalCheckCommandKind::CargoTestWorkspace,
        LocalCheckCommandKind::TypeScriptCheck,
        LocalCheckCommandKind::ContractCheck,
        LocalCheckCommandKind::IntegrationCheck,
    ];

    assert_eq!(kinds.len(), 8);
}

#[test]
fn all_planned_command_kinds_bind_to_canonical_templates() {
    let cases = [
        (
            LocalCheckCommandKind::WorkflowOsValidateDogfood,
            "workflow-os",
            &[
                "--project-dir",
                "dogfood/workflow-os-self-governance",
                "validate",
            ][..],
        ),
        (
            LocalCheckCommandKind::DocsCheck,
            "npm",
            &["run", "check:docs"][..],
        ),
        (
            LocalCheckCommandKind::CargoFmtCheck,
            "cargo",
            &["fmt", "--all", "--check"][..],
        ),
        (
            LocalCheckCommandKind::CargoClippyWorkspace,
            "cargo",
            &[
                "clippy",
                "--workspace",
                "--all-targets",
                "--",
                "-D",
                "warnings",
            ][..],
        ),
        (
            LocalCheckCommandKind::CargoTestWorkspace,
            "cargo",
            &["test", "--workspace"][..],
        ),
        (
            LocalCheckCommandKind::TypeScriptCheck,
            "npm",
            &["run", "check:ts"][..],
        ),
        (
            LocalCheckCommandKind::ContractCheck,
            "npm",
            &["run", "check:contracts"][..],
        ),
        (
            LocalCheckCommandKind::IntegrationCheck,
            "npm",
            &["run", "check:integrations"][..],
        ),
    ];

    for (kind, executable, arguments) in cases {
        let contract =
            LocalCheckCommandContract::new(definition_for_kind(kind, executable, arguments))
                .expect("canonical template validates");

        assert_eq!(contract.command_kind(), kind);
        assert_eq!(contract.executable(), executable);
        assert_eq!(contract.arguments(), arguments);
    }
}

#[test]
fn result_status_vocabulary_is_representable_without_execution() {
    let statuses = [
        LocalCheckResultStatus::Passed,
        LocalCheckResultStatus::Failed,
        LocalCheckResultStatus::TimedOut,
        LocalCheckResultStatus::Skipped,
        LocalCheckResultStatus::NotAvailable,
        LocalCheckResultStatus::InternalError,
        LocalCheckResultStatus::PolicyDenied,
        LocalCheckResultStatus::RedactionFailed,
    ];

    assert_eq!(statuses.len(), 8);
}

#[test]
fn execution_posture_rejects_premature_handler_authorization() {
    let mut definition = valid_definition();
    definition.execution_posture = LocalCheckExecutionPosture::AllowlistedHandlerOnly;

    let error = LocalCheckCommandContract::new(definition).expect_err("execution remains deferred");

    assert_eq!(error.code(), "local_check.execution.deferred");
}

#[test]
fn command_kind_rejects_mismatched_executable_without_leaking_value() {
    let mut definition = valid_definition();
    definition.executable = "cargo".to_owned();
    definition.arguments = vec!["test".to_owned(), "--workspace".to_owned()];

    let error = LocalCheckCommandContract::new(definition).expect_err("template mismatch rejected");

    assert_eq!(error.code(), "local_check.command_template.mismatch");
    assert!(!error.to_string().contains("cargo"));
}

#[test]
fn command_kind_rejects_mismatched_arguments_without_leaking_value() {
    let mut definition = valid_definition();
    definition.arguments = vec!["run".to_owned(), "check:ts".to_owned()];

    let error = LocalCheckCommandContract::new(definition).expect_err("template mismatch rejected");

    assert_eq!(error.code(), "local_check.command_template.mismatch");
    assert!(!error.to_string().contains("check:ts"));
}

#[test]
fn shell_metacharacters_are_rejected_without_leaking_argument() {
    let mut definition = valid_definition();
    definition
        .arguments
        .push("check:docs && cat secret".to_owned());

    let error = LocalCheckCommandContract::new(definition).expect_err("shell metachar rejected");

    assert_eq!(
        error.code(),
        "local_check.command_token.shell_metacharacter"
    );
    assert!(!error.to_string().contains("cat secret"));
}

#[test]
fn whitespace_in_command_tokens_is_rejected_without_leaking_value() {
    let mut definition = valid_definition();
    definition.arguments.push("two words".to_owned());

    let error = LocalCheckCommandContract::new(definition).expect_err("whitespace rejected");

    assert_eq!(
        error.code(),
        "local_check.command_token.shell_metacharacter"
    );
    assert!(!error.to_string().contains("two words"));
}

#[test]
fn secret_like_arguments_and_environment_names_are_rejected() {
    let mut definition = valid_definition();
    definition.arguments.push("api_token=abc123".to_owned());
    let error = LocalCheckCommandContract::new(definition).expect_err("secret-like arg rejected");
    assert_eq!(error.code(), "local_check.secret_like_value");
    assert!(!error.to_string().contains("api_token"));

    let mut definition = valid_definition();
    definition
        .allowed_environment_variables
        .push("AUTHORIZATION_HEADER".to_owned());
    let error = LocalCheckCommandContract::new(definition).expect_err("secret-like env rejected");
    assert_eq!(error.code(), "local_check.secret_like_value");
    assert!(!error.to_string().contains("AUTHORIZATION_HEADER"));
}

#[test]
fn excessive_arguments_environment_and_timeout_are_rejected() {
    let mut definition = valid_definition();
    definition.arguments = (0..33).map(|index| format!("arg{index}")).collect();
    let error = LocalCheckCommandContract::new(definition).expect_err("too many args rejected");
    assert_eq!(error.code(), "local_check.arguments.too_many");

    let mut definition = valid_definition();
    definition.allowed_environment_variables =
        (0..17).map(|index| format!("SAFE_VAR_{index}")).collect();
    let error = LocalCheckCommandContract::new(definition).expect_err("too many env vars rejected");
    assert_eq!(error.code(), "local_check.environment.too_many");

    let mut definition = valid_definition();
    definition.timeout_seconds = 30 * 60 + 1;
    let error = LocalCheckCommandContract::new(definition).expect_err("timeout too large rejected");
    assert_eq!(error.code(), "local_check.timeout.too_large");
}

#[test]
fn output_capture_rejects_raw_persistence_and_unbounded_capture() {
    let mut definition = valid_definition();
    definition.output_capture = LocalCheckOutputCapturePolicy {
        stdout_max_bytes: 16 * 1024,
        stderr_max_bytes: 16 * 1024,
        persist_raw_output: true,
    };
    let error = LocalCheckCommandContract::new(definition).expect_err("raw output rejected");
    assert_eq!(
        error.code(),
        "local_check.output.raw_persistence_unsupported"
    );

    let mut definition = valid_definition();
    definition.output_capture = LocalCheckOutputCapturePolicy::bounded(0, 16 * 1024);
    let error = LocalCheckCommandContract::new(definition).expect_err("zero bound rejected");
    assert_eq!(error.code(), "local_check.output.bound_required");
}

#[test]
fn unclassified_side_effects_are_rejected() {
    let mut definition = valid_definition();
    definition.side_effect_class = LocalCheckSideEffectClass::Unclassified;

    let error =
        LocalCheckCommandContract::new(definition).expect_err("side effects must be classified");

    assert_eq!(error.code(), "local_check.side_effect.unclassified");
}

#[test]
fn duplicate_citation_kinds_are_rejected() {
    let mut definition = valid_definition();
    definition
        .citation_kinds
        .push(WorkReportCitationKind::ValidationDiagnostic);

    let error = LocalCheckCommandContract::new(definition).expect_err("duplicate citation");

    assert_eq!(error.code(), "local_check.citation.duplicate");
}

#[test]
fn invalid_serialized_contract_fails_closed_without_leaking_payload() {
    let payload = json!({
        "command_id": "local-check/docs",
        "command_kind": "docs_check",
        "execution_posture": "model_only",
        "executable": "npm",
        "arguments": ["run", "check:docs", "bearer-token-super-secret"],
        "working_directory_policy": "repository_root",
        "environment_policy": "sanitized_minimal",
        "allowed_environment_variables": [],
        "network_policy": "disabled",
        "timeout_seconds": 120,
        "side_effect_class": "no_source_writes",
        "permitted_output_directories": [],
        "output_capture": {
            "stdout_max_bytes": 16384,
            "stderr_max_bytes": 16384,
            "persist_raw_output": false
        },
        "redaction_policy": "bounded_redacted_summary",
        "citation_kinds": ["validation_diagnostic"]
    });

    let error = serde_json::from_value::<LocalCheckCommandContract>(payload)
        .expect_err("invalid serialized contract fails");
    let error_text = error.to_string();

    assert!(!error_text.contains("bearer-token-super-secret"));
    assert!(!error_text.contains("bearer"));
}

#[test]
fn serde_round_trip_for_valid_contract() {
    let contract = valid_contract();

    let serialized = serde_json::to_string(&contract).expect("contract serializes");
    let deserialized: LocalCheckCommandContract =
        serde_json::from_str(&serialized).expect("contract deserializes");

    assert_eq!(deserialized, contract);
}

#[test]
fn debug_output_redacts_command_tokens_and_environment_names() {
    let mut definition = valid_definition();
    definition
        .allowed_environment_variables
        .push("SAFE_CACHE_DIR".to_owned());
    let contract = LocalCheckCommandContract::new(definition).expect("valid contract");

    let debug = format!("{contract:?}");

    assert!(debug.contains("LocalCheckCommandContract"));
    assert!(!debug.contains("npm"));
    assert!(!debug.contains("check:docs"));
    assert!(!debug.contains("SAFE_CACHE_DIR"));
}
