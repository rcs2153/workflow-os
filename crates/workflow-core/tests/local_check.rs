#![allow(clippy::expect_used)]

//! Local validation/check command contract model tests.

use std::collections::BTreeMap;
use std::fmt;
use std::sync::{Arc, Mutex};

use serde_json::json;
use workflow_core::{
    CorrelationId, DocsCheckLocalHandler, EventId, LocalCheckCommandContract,
    LocalCheckCommandContractDefinition, LocalCheckCommandId, LocalCheckCommandKind,
    LocalCheckEnvironmentPolicy, LocalCheckExecutionPosture, LocalCheckNetworkPolicy,
    LocalCheckOutputCapturePolicy, LocalCheckProcessOutput, LocalCheckProcessRequest,
    LocalCheckProcessRunner, LocalCheckRedactionPolicy, LocalCheckResult,
    LocalCheckResultDefinition, LocalCheckResultId, LocalCheckResultReference,
    LocalCheckResultReferenceDefinition, LocalCheckResultStatus, LocalCheckSideEffectClass,
    LocalCheckWorkingDirectoryPolicy, RedactionDisposition, RedactionFieldState, RedactionMetadata,
    SchemaVersion, SkillHandler, SkillId, SkillInput, SkillVersion, SpecContentHash, StepId,
    TestOnlyWorkflowOsValidateDogfoodHandler, WorkReportCitationKind, WorkReportSensitivity,
    WorkflowId, WorkflowRunId, WorkflowVersion, SUPPORTED_SCHEMA_VERSION,
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

fn valid_result() -> LocalCheckResult {
    LocalCheckResult::new(LocalCheckResultDefinition {
        command_id: command_id(),
        command_kind: LocalCheckCommandKind::DocsCheck,
        status: LocalCheckResultStatus::Passed,
        exit_code: Some(0),
        duration_ms: 42,
        stdout_summary: "docs passed".to_owned(),
        stderr_summary: String::new(),
        stdout_truncated: false,
        stderr_truncated: false,
        error_code: None,
    })
    .expect("valid local check result")
}

fn result_reference_id() -> LocalCheckResultId {
    LocalCheckResultId::new("local-check-result/run-local-check/local-check-docs/1")
        .expect("valid result id")
}

fn redaction_metadata() -> RedactionMetadata {
    RedactionMetadata {
        redacted_fields: vec!["stdout_summary".to_owned()],
        field_states: vec![RedactionFieldState {
            field: "stdout_summary".to_owned(),
            disposition: RedactionDisposition::ReferenceOnly,
            reason: "bounded_summary_only".to_owned(),
        }],
    }
}

fn valid_result_reference() -> LocalCheckResultReference {
    LocalCheckResultReference::new(valid_result_reference_definition())
        .expect("valid local check result reference")
}

fn valid_result_reference_definition() -> LocalCheckResultReferenceDefinition {
    LocalCheckResultReferenceDefinition {
        result_id: result_reference_id(),
        command_id: command_id(),
        command_kind: LocalCheckCommandKind::DocsCheck,
        status: LocalCheckResultStatus::Passed,
        workflow_id: WorkflowId::new("wf/local-check").expect("workflow id"),
        run_id: WorkflowRunId::new("run/local-check").expect("run id"),
        workflow_event_id: Some(EventId::new("event/local-check").expect("event id")),
        audit_event_id: Some(EventId::new("audit/local-check").expect("audit id")),
        output_reference: Some("local-check-result/docs/passed".to_owned()),
        redaction: redaction_metadata(),
        sensitivity: WorkReportSensitivity::Confidential,
    }
}

fn repository_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("workspace root")
        .to_path_buf()
}

fn skill_input() -> SkillInput {
    SkillInput {
        run_id: WorkflowRunId::new("run/local-check").expect("run id"),
        workflow_id: WorkflowId::new("local/main").expect("workflow id"),
        workflow_version: WorkflowVersion::new("v0").expect("workflow version"),
        schema_version: SchemaVersion::new(SUPPORTED_SCHEMA_VERSION).expect("schema version"),
        spec_hash: SpecContentHash::from_text("schema_version: workflowos.dev/v0\n"),
        step_id: StepId::new("check").expect("step id"),
        skill_id: SkillId::new("local/check-dogfood").expect("skill id"),
        skill_version: SkillVersion::new("v0").expect("skill version"),
        correlation_id: CorrelationId::new("correlation/local-check").expect("correlation"),
        values: BTreeMap::new(),
    }
}

#[derive(Clone)]
struct FakeRunner {
    output: Result<LocalCheckProcessOutput, String>,
    last_request: Arc<Mutex<Option<LocalCheckProcessRequest>>>,
}

impl FakeRunner {
    fn with_output(output: LocalCheckProcessOutput) -> Self {
        Self {
            output: Ok(output),
            last_request: Arc::new(Mutex::new(None)),
        }
    }

    fn failing() -> Self {
        Self {
            output: Err("bearer-token-super-secret".to_owned()),
            last_request: Arc::new(Mutex::new(None)),
        }
    }
}

impl fmt::Debug for FakeRunner {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FakeRunner")
    }
}

impl LocalCheckProcessRunner for FakeRunner {
    fn run(
        &self,
        request: &LocalCheckProcessRequest,
    ) -> Result<LocalCheckProcessOutput, workflow_core::WorkflowOsError> {
        *self.last_request.lock().expect("request lock") = Some(request.clone());
        self.output.clone().map_err(|_| {
            workflow_core::WorkflowOsError::new(
                workflow_core::WorkflowOsErrorKind::Internal,
                "local_check.test.runner_failed",
                "fake local check runner failed",
            )
        })
    }
}

fn handler_with_runner(runner: Arc<FakeRunner>) -> TestOnlyWorkflowOsValidateDogfoodHandler {
    let contract =
        LocalCheckCommandContract::dogfood_validate_model_only().expect("valid dogfood contract");
    TestOnlyWorkflowOsValidateDogfoodHandler::new_with_process_runner(
        contract,
        std::env::current_exe().expect("current test binary"),
        repository_root(),
        runner,
    )
    .expect("handler with fake runner")
}

fn docs_handler_with_runner(runner: Arc<FakeRunner>) -> DocsCheckLocalHandler {
    let contract = LocalCheckCommandContract::docs_check_model_only().expect("valid docs contract");
    DocsCheckLocalHandler::new_with_process_runner(
        contract,
        std::env::current_exe().expect("current test binary"),
        repository_root(),
        Some(std::env::temp_dir().join("workflow-os-npm-cache")),
        runner,
    )
    .expect("docs handler with fake runner")
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
fn docs_check_contract_is_model_only_and_non_executing() {
    let contract = LocalCheckCommandContract::docs_check_model_only().expect("valid docs contract");

    assert_eq!(contract.command_kind(), LocalCheckCommandKind::DocsCheck);
    assert_eq!(
        contract.execution_posture(),
        LocalCheckExecutionPosture::ModelOnly
    );
    assert_eq!(contract.executable(), "npm");
    assert_eq!(contract.arguments(), ["run", "check:docs"]);
    assert_eq!(
        contract.allowed_environment_variables(),
        ["NPM_CONFIG_CACHE"]
    );
    assert_eq!(contract.network_policy(), LocalCheckNetworkPolicy::Disabled);
    assert_eq!(
        contract.side_effect_class(),
        LocalCheckSideEffectClass::NoSourceWrites
    );
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
fn valid_local_check_result_is_structured_and_accessible() {
    let result = valid_result();

    assert_eq!(result.command_id().as_str(), "local-check/docs");
    assert_eq!(result.command_kind(), LocalCheckCommandKind::DocsCheck);
    assert_eq!(result.status(), LocalCheckResultStatus::Passed);
    assert_eq!(result.exit_code(), Some(0));
    assert_eq!(result.duration_ms(), 42);
    assert_eq!(result.stdout_summary(), "docs passed");
    assert_eq!(result.stderr_summary(), "");
    assert!(!result.stdout_truncated());
    assert!(!result.stderr_truncated());
    assert_eq!(result.error_code(), None);
}

#[test]
fn local_check_result_accepts_failed_result() {
    let result = LocalCheckResult::new(LocalCheckResultDefinition {
        command_id: command_id(),
        command_kind: LocalCheckCommandKind::DocsCheck,
        status: LocalCheckResultStatus::Failed,
        exit_code: Some(1),
        duration_ms: 7,
        stdout_summary: String::new(),
        stderr_summary: "docs failed".to_owned(),
        stdout_truncated: false,
        stderr_truncated: false,
        error_code: None,
    })
    .expect("failed result is representable");

    assert_eq!(result.status(), LocalCheckResultStatus::Failed);
    assert_eq!(result.exit_code(), Some(1));
}

#[test]
fn local_check_result_rejects_secret_like_summaries_without_leaking() {
    let error = LocalCheckResult::new(LocalCheckResultDefinition {
        command_id: command_id(),
        command_kind: LocalCheckCommandKind::DocsCheck,
        status: LocalCheckResultStatus::Failed,
        exit_code: Some(1),
        duration_ms: 7,
        stdout_summary: "bearer-token-super-secret".to_owned(),
        stderr_summary: String::new(),
        stdout_truncated: false,
        stderr_truncated: false,
        error_code: None,
    })
    .expect_err("secret-like stdout summary is rejected");

    assert_eq!(error.code(), "local_check.secret_like_value");
    assert!(!error.to_string().contains("bearer-token-super-secret"));

    let error = LocalCheckResult::new(LocalCheckResultDefinition {
        command_id: command_id(),
        command_kind: LocalCheckCommandKind::DocsCheck,
        status: LocalCheckResultStatus::Failed,
        exit_code: Some(1),
        duration_ms: 7,
        stdout_summary: String::new(),
        stderr_summary: "api_token=super-secret".to_owned(),
        stdout_truncated: false,
        stderr_truncated: false,
        error_code: None,
    })
    .expect_err("secret-like stderr summary is rejected");

    assert_eq!(error.code(), "local_check.secret_like_value");
    assert!(!error.to_string().contains("api_token"));
}

#[test]
fn local_check_result_rejects_unbounded_summaries() {
    let error = LocalCheckResult::new(LocalCheckResultDefinition {
        command_id: command_id(),
        command_kind: LocalCheckCommandKind::DocsCheck,
        status: LocalCheckResultStatus::Failed,
        exit_code: Some(1),
        duration_ms: 7,
        stdout_summary: "x".repeat(64 * 1024 + 1),
        stderr_summary: String::new(),
        stdout_truncated: true,
        stderr_truncated: false,
        error_code: None,
    })
    .expect_err("unbounded stdout rejected");

    assert_eq!(error.code(), "local_check.result.summary_too_large");
}

#[test]
fn local_check_result_debug_and_serialization_do_not_leak_summaries() {
    let result = valid_result();

    let debug = format!("{result:?}");
    assert!(debug.contains("LocalCheckResult"));
    assert!(!debug.contains("docs passed"));

    let serialized = serde_json::to_string(&result).expect("result serializes");
    assert!(serialized.contains("docs passed"));
    assert!(!serialized.contains("bearer-token"));
}

#[test]
fn local_check_process_output_debug_does_not_leak_raw_output() {
    let output = LocalCheckProcessOutput::completed(
        Some(1),
        false,
        42,
        b"bearer-token-super-secret".to_vec(),
        b"api_token=super-secret".to_vec(),
    );

    let debug = format!("{output:?}");

    assert!(debug.contains("LocalCheckProcessOutput"));
    assert!(debug.contains("stdout_byte_count"));
    assert!(debug.contains("stderr_byte_count"));
    assert!(debug.contains("[REDACTED]"));
    assert!(!debug.contains("bearer-token-super-secret"));
    assert!(!debug.contains("api_token"));
    assert!(!debug.contains("super-secret"));
}

#[test]
fn local_check_process_timeout_output_debug_does_not_leak_raw_output() {
    let output = LocalCheckProcessOutput::timed_out(
        120_000,
        b"authorization: bearer-token-super-secret".to_vec(),
        b"private_key=super-secret".to_vec(),
    );

    let debug = format!("{output:?}");

    assert!(debug.contains("timed_out"));
    assert!(debug.contains("[REDACTED]"));
    assert!(!debug.contains("authorization"));
    assert!(!debug.contains("private_key"));
    assert!(!debug.contains("super-secret"));
}

#[test]
fn local_check_result_serde_round_trip_and_invalid_payload_fails_closed() {
    let result = valid_result();
    let serialized = serde_json::to_string(&result).expect("result serializes");
    let deserialized: LocalCheckResult =
        serde_json::from_str(&serialized).expect("result deserializes");
    assert_eq!(deserialized, result);

    let payload = json!({
        "command_id": "local-check/docs",
        "command_kind": "docs_check",
        "status": "failed",
        "exit_code": 1,
        "duration_ms": 7,
        "stdout_summary": "bearer-token-super-secret",
        "stderr_summary": "",
        "stdout_truncated": false,
        "stderr_truncated": false,
        "error_code": null
    });

    let error = serde_json::from_value::<LocalCheckResult>(payload)
        .expect_err("invalid serialized result fails");
    assert!(!error.to_string().contains("bearer-token-super-secret"));
}

#[test]
fn valid_local_check_result_reference_is_structured_and_accessible() {
    let reference = valid_result_reference();

    assert_eq!(
        reference.result_id().as_str(),
        "local-check-result/run-local-check/local-check-docs/1"
    );
    assert_eq!(reference.command_id().as_str(), "local-check/docs");
    assert_eq!(reference.command_kind(), LocalCheckCommandKind::DocsCheck);
    assert_eq!(reference.status(), LocalCheckResultStatus::Passed);
    assert_eq!(reference.workflow_id().as_str(), "wf/local-check");
    assert_eq!(reference.run_id().as_str(), "run/local-check");
    assert_eq!(
        reference.workflow_event_id().map(EventId::as_str),
        Some("event/local-check")
    );
    assert_eq!(
        reference.audit_event_id().map(EventId::as_str),
        Some("audit/local-check")
    );
    assert_eq!(
        reference.output_reference(),
        Some("local-check-result/docs/passed")
    );
    assert_eq!(reference.sensitivity(), WorkReportSensitivity::Confidential);
}

#[test]
fn local_check_result_reference_from_result_preserves_identity_without_summaries() {
    let result = valid_result();
    let reference = LocalCheckResultReference::from_result(
        result_reference_id(),
        &result,
        WorkflowId::new("wf/local-check").expect("workflow id"),
        WorkflowRunId::new("run/local-check").expect("run id"),
        None,
        None,
        Some("local-check-result/docs/passed".to_owned()),
        redaction_metadata(),
        WorkReportSensitivity::Confidential,
    )
    .expect("reference from result");

    assert_eq!(reference.command_id(), result.command_id());
    assert_eq!(reference.command_kind(), result.command_kind());
    assert_eq!(reference.status(), result.status());

    let debug = format!("{reference:?}");
    assert!(!debug.contains("docs passed"));
    assert!(!debug.contains("local-check-result/docs/passed"));
}

#[test]
fn local_check_result_id_rejects_invalid_and_secret_like_values_without_leaking() {
    let error = LocalCheckResultId::new("local-check-result/authorization-token")
        .expect_err("secret-like result id rejected");

    assert_eq!(error.code(), "local_check.secret_like_value");
    assert!(!error.to_string().contains("authorization-token"));

    let error =
        LocalCheckResultId::new("local check result").expect_err("invalid result id rejected");

    assert_eq!(error.code(), "local_check.identifier.invalid_character");
}

#[test]
fn local_check_result_reference_rejects_secret_like_output_reference_without_leaking() {
    let error = LocalCheckResultReference::new(LocalCheckResultReferenceDefinition {
        output_reference: Some("local-check-result/bearer-token-super-secret".to_owned()),
        ..valid_result_reference_definition()
    })
    .expect_err("secret-like output reference rejected");

    assert_eq!(error.code(), "local_check.secret_like_value");
    assert!(!error.to_string().contains("bearer-token-super-secret"));
}

#[test]
fn local_check_result_reference_rejects_secret_like_redaction_metadata_without_leaking() {
    let mut redaction = redaction_metadata();
    redaction.field_states.push(RedactionFieldState {
        field: "stdout_summary".to_owned(),
        disposition: RedactionDisposition::Redacted,
        reason: "removed bearer-token-super-secret".to_owned(),
    });

    let error = LocalCheckResultReference::new(LocalCheckResultReferenceDefinition {
        redaction,
        ..valid_result_reference_definition()
    })
    .expect_err("secret-like redaction reason rejected");

    assert_eq!(error.code(), "local_check.secret_like_value");
    assert!(!error.to_string().contains("bearer-token-super-secret"));

    let mut redaction = redaction_metadata();
    redaction.redacted_fields.push("api_token".to_owned());
    let error = LocalCheckResultReference::new(LocalCheckResultReferenceDefinition {
        redaction,
        ..valid_result_reference_definition()
    })
    .expect_err("secret-like redaction field rejected");

    assert_eq!(error.code(), "local_check.secret_like_value");
    assert!(!error.to_string().contains("api_token"));
}

#[test]
fn local_check_result_reference_debug_and_serialization_do_not_copy_raw_output() {
    let reference = valid_result_reference();

    let debug = format!("{reference:?}");
    assert!(debug.contains("LocalCheckResultReference"));
    assert!(!debug.contains("local-check-result/run-local-check"));
    assert!(!debug.contains("local-check-result/docs/passed"));
    assert!(!debug.contains("stdout_summary"));

    let serialized = serde_json::to_string(&reference).expect("reference serializes");
    assert!(serialized.contains("local-check-result/docs/passed"));
    assert!(!serialized.contains("docs passed"));
    assert!(!serialized.contains("raw provider payload"));
    assert!(!serialized.contains("raw command transcript"));
    assert!(!serialized.contains("bearer-token-super-secret"));
}

#[test]
fn local_check_result_reference_serde_round_trip_and_invalid_payload_fails_closed() {
    let reference = valid_result_reference();
    let serialized = serde_json::to_string(&reference).expect("reference serializes");
    let deserialized: LocalCheckResultReference =
        serde_json::from_str(&serialized).expect("reference deserializes");
    assert_eq!(deserialized, reference);

    let payload = json!({
        "result_id": "local-check-result/run-local-check/local-check-docs/1",
        "command_id": "local-check/docs",
        "command_kind": "docs_check",
        "status": "passed",
        "workflow_id": "wf/local-check",
        "run_id": "run/local-check",
        "workflow_event_id": null,
        "audit_event_id": null,
        "output_reference": "local-check-result/private-key-super-secret",
        "redaction": {
            "redacted_fields": [],
            "field_states": []
        },
        "sensitivity": "confidential"
    });

    let error = serde_json::from_value::<LocalCheckResultReference>(payload)
        .expect_err("invalid serialized reference fails");
    assert!(!error.to_string().contains("private-key-super-secret"));
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

#[test]
fn test_only_handler_rejects_unsupported_command_kind_without_leaking_paths() {
    let contract = valid_contract();

    let error = TestOnlyWorkflowOsValidateDogfoodHandler::new(
        contract,
        std::env::current_exe().expect("current test binary"),
        repository_root(),
    )
    .expect_err("unsupported check kind is rejected");

    assert_eq!(error.code(), "local_check.handler.unsupported_kind");
    assert!(!error.to_string().contains("workflow-os"));
    assert!(!error.to_string().contains(env!("CARGO_MANIFEST_DIR")));
}

#[test]
fn test_only_docs_handler_rejects_unsupported_command_kind_without_leaking_paths() {
    let contract =
        LocalCheckCommandContract::dogfood_validate_model_only().expect("valid dogfood contract");

    let error = DocsCheckLocalHandler::new_with_process_runner(
        contract,
        std::env::current_exe().expect("current test binary"),
        repository_root(),
        None,
        Arc::new(FakeRunner::with_output(LocalCheckProcessOutput::completed(
            Some(0),
            true,
            12,
            Vec::new(),
            Vec::new(),
        ))),
    )
    .expect_err("unsupported check kind is rejected");

    assert_eq!(error.code(), "local_check.handler.unsupported_kind");
    assert!(!error.to_string().contains("check-docs"));
    assert!(!error.to_string().contains(env!("CARGO_MANIFEST_DIR")));
}

#[test]
fn test_only_docs_handler_rejects_secret_like_cache_path_without_leaking() {
    let contract = LocalCheckCommandContract::docs_check_model_only().expect("valid docs contract");

    let error = DocsCheckLocalHandler::new_with_process_runner(
        contract,
        std::env::current_exe().expect("current test binary"),
        repository_root(),
        Some(std::env::temp_dir().join("secret-token-cache")),
        Arc::new(FakeRunner::with_output(LocalCheckProcessOutput::completed(
            Some(0),
            true,
            12,
            Vec::new(),
            Vec::new(),
        ))),
    )
    .expect_err("secret-like cache path is rejected");

    assert_eq!(error.code(), "local_check.secret_like_value");
    assert!(!error.to_string().contains("secret-token-cache"));
}

#[test]
fn docs_check_local_handler_debug_redacts_local_paths_and_cache() {
    let contract = LocalCheckCommandContract::docs_check_model_only().expect("valid docs contract");
    let handler = DocsCheckLocalHandler::new_with_process_runner(
        contract,
        std::env::current_exe().expect("current test binary"),
        repository_root(),
        Some(std::env::temp_dir().join("workflow-os-npm-cache")),
        Arc::new(FakeRunner::with_output(LocalCheckProcessOutput::completed(
            Some(0),
            true,
            12,
            Vec::new(),
            Vec::new(),
        ))),
    )
    .expect("valid docs handler");

    let debug = format!("{handler:?}");

    assert!(debug.contains("DocsCheckLocalHandler"));
    assert!(!debug.contains(env!("CARGO_MANIFEST_DIR")));
    assert!(!debug.contains("workflow-os-npm-cache"));
    assert!(!debug.contains("check:docs"));
}

#[test]
fn test_only_handler_debug_redacts_local_paths() {
    let contract =
        LocalCheckCommandContract::dogfood_validate_model_only().expect("valid dogfood contract");
    let handler = TestOnlyWorkflowOsValidateDogfoodHandler::new(
        contract,
        std::env::current_exe().expect("current test binary"),
        repository_root(),
    )
    .expect("valid test-only handler");

    let debug = format!("{handler:?}");

    assert!(debug.contains("TestOnlyWorkflowOsValidateDogfoodHandler"));
    assert!(!debug.contains(env!("CARGO_MANIFEST_DIR")));
    assert!(!debug.contains("local_check"));
}

#[test]
fn docs_handler_injected_runner_maps_success_to_passed_skill_output() {
    let runner = Arc::new(FakeRunner::with_output(LocalCheckProcessOutput::completed(
        Some(0),
        true,
        12,
        b"docs passed".to_vec(),
        Vec::new(),
    )));
    let handler = docs_handler_with_runner(Arc::clone(&runner));

    let output = handler.invoke(skill_input()).expect("handler succeeds");

    assert_eq!(
        output.values.get("local_check_status").map(String::as_str),
        Some("passed")
    );
    assert_eq!(
        output.values.get("local_check_kind").map(String::as_str),
        Some("docs_check")
    );
    assert_eq!(
        output.values.get("stdout_summary").map(String::as_str),
        Some("docs passed")
    );
    assert!(output
        .output_ref
        .as_deref()
        .expect("output ref")
        .ends_with("/passed"));
    let request = runner
        .last_request
        .lock()
        .expect("request lock")
        .clone()
        .expect("runner request captured");
    assert_eq!(request.arguments(), ["run", "check:docs"]);
    assert_eq!(request.environment().len(), 2);
    assert!(request.environment().contains_key("PATH"));
    assert!(request.environment().contains_key("NPM_CONFIG_CACHE"));
}

#[test]
fn docs_handler_injected_runner_maps_non_zero_exit_to_failed_skill_output() {
    let runner = Arc::new(FakeRunner::with_output(LocalCheckProcessOutput::completed(
        Some(1),
        false,
        12,
        Vec::new(),
        b"docs failed".to_vec(),
    )));
    let handler = docs_handler_with_runner(runner);

    let output = handler.invoke(skill_input()).expect("handler succeeds");

    assert_eq!(
        output.values.get("local_check_status").map(String::as_str),
        Some("failed")
    );
    assert_eq!(
        output.values.get("stderr_summary").map(String::as_str),
        Some("docs failed")
    );
}

#[test]
fn docs_handler_injected_runner_maps_timeout_to_timed_out_skill_output() {
    let runner = Arc::new(FakeRunner::with_output(LocalCheckProcessOutput::timed_out(
        120_000,
        Vec::new(),
        Vec::new(),
    )));
    let handler = docs_handler_with_runner(runner);

    let output = handler.invoke(skill_input()).expect("handler succeeds");

    assert_eq!(
        output.values.get("local_check_status").map(String::as_str),
        Some("timed_out")
    );
    assert_eq!(
        output.values.get("error_code").map(String::as_str),
        Some("local_check.handler.timed_out")
    );
}

#[test]
fn docs_handler_secret_like_stdout_and_stderr_fail_without_leaking() {
    let runner = Arc::new(FakeRunner::with_output(LocalCheckProcessOutput::completed(
        Some(0),
        true,
        12,
        b"bearer-token-super-secret".to_vec(),
        Vec::new(),
    )));
    let handler = docs_handler_with_runner(runner);

    let error = handler
        .invoke(skill_input())
        .expect_err("secret-like stdout fails");

    assert_eq!(error.code(), "local_check.output.secret_like");
    assert!(!error.to_string().contains("bearer-token-super-secret"));

    let runner = Arc::new(FakeRunner::with_output(LocalCheckProcessOutput::completed(
        Some(1),
        false,
        12,
        Vec::new(),
        b"api_token=super-secret".to_vec(),
    )));
    let handler = docs_handler_with_runner(runner);

    let error = handler
        .invoke(skill_input())
        .expect_err("secret-like stderr fails");

    assert_eq!(error.code(), "local_check.output.secret_like");
    assert!(!error.to_string().contains("api_token"));
}

#[test]
fn injected_runner_maps_success_to_passed_skill_output() {
    let runner = Arc::new(FakeRunner::with_output(LocalCheckProcessOutput::completed(
        Some(0),
        true,
        12,
        b"validated".to_vec(),
        Vec::new(),
    )));
    let handler = handler_with_runner(Arc::clone(&runner));

    let output = handler.invoke(skill_input()).expect("handler succeeds");

    assert_eq!(
        output.values.get("local_check_status").map(String::as_str),
        Some("passed")
    );
    assert_eq!(
        output.values.get("stdout_summary").map(String::as_str),
        Some("validated")
    );
    assert!(output
        .output_ref
        .as_deref()
        .expect("output ref")
        .ends_with("/passed"));
    let request = runner
        .last_request
        .lock()
        .expect("request lock")
        .clone()
        .expect("runner request captured");
    assert_eq!(request.arguments().len(), 3);
    assert_eq!(request.environment().len(), 1);
    assert!(request.environment().contains_key("PATH"));
}

#[test]
fn injected_runner_maps_non_zero_exit_to_failed_skill_output() {
    let runner = Arc::new(FakeRunner::with_output(LocalCheckProcessOutput::completed(
        Some(2),
        false,
        12,
        Vec::new(),
        b"validation failed".to_vec(),
    )));
    let handler = handler_with_runner(runner);

    let output = handler.invoke(skill_input()).expect("handler succeeds");

    assert_eq!(
        output.values.get("local_check_status").map(String::as_str),
        Some("failed")
    );
    assert_eq!(
        output.values.get("stderr_summary").map(String::as_str),
        Some("validation failed")
    );
    assert!(output
        .output_ref
        .as_deref()
        .expect("output ref")
        .ends_with("/failed"));
}

#[test]
fn injected_runner_maps_timeout_to_timed_out_skill_output() {
    let runner = Arc::new(FakeRunner::with_output(LocalCheckProcessOutput::timed_out(
        120_000,
        Vec::new(),
        Vec::new(),
    )));
    let handler = handler_with_runner(runner);

    let output = handler.invoke(skill_input()).expect("handler succeeds");

    assert_eq!(
        output.values.get("local_check_status").map(String::as_str),
        Some("timed_out")
    );
    assert_eq!(
        output.values.get("error_code").map(String::as_str),
        Some("local_check.handler.timed_out")
    );
}

#[test]
fn injected_runner_secret_like_stdout_fails_without_leaking() {
    let runner = Arc::new(FakeRunner::with_output(LocalCheckProcessOutput::completed(
        Some(0),
        true,
        12,
        b"bearer-token-super-secret".to_vec(),
        Vec::new(),
    )));
    let handler = handler_with_runner(runner);

    let error = handler
        .invoke(skill_input())
        .expect_err("secret-like stdout fails");

    assert_eq!(error.code(), "local_check.output.secret_like");
    assert!(!error.to_string().contains("bearer-token-super-secret"));
}

#[test]
fn injected_runner_secret_like_stderr_fails_without_leaking() {
    let runner = Arc::new(FakeRunner::with_output(LocalCheckProcessOutput::completed(
        Some(1),
        false,
        12,
        Vec::new(),
        b"api_token=super-secret".to_vec(),
    )));
    let handler = handler_with_runner(runner);

    let error = handler
        .invoke(skill_input())
        .expect_err("secret-like stderr fails");

    assert_eq!(error.code(), "local_check.output.secret_like");
    assert!(!error.to_string().contains("api_token"));
}

#[test]
fn injected_runner_failure_returns_stable_non_leaking_error() {
    let runner = Arc::new(FakeRunner::failing());
    let handler = handler_with_runner(runner);

    let error = handler.invoke(skill_input()).expect_err("runner fails");

    assert_eq!(error.code(), "local_check.test.runner_failed");
    assert!(!error.to_string().contains("bearer-token-super-secret"));
}

#[test]
fn process_request_rejects_secret_like_environment() {
    let mut environment = BTreeMap::new();
    environment.insert("AUTHORIZATION".to_owned(), "safe".to_owned());

    let error = LocalCheckProcessRequest::new(
        std::env::current_exe().expect("current exe"),
        vec!["validate".to_owned()],
        repository_root(),
        environment,
        std::time::Duration::from_secs(1),
    )
    .expect_err("secret-like environment key rejected");

    assert_eq!(error.code(), "local_check.secret_like_value");
    assert!(!error.to_string().contains("AUTHORIZATION"));
}
