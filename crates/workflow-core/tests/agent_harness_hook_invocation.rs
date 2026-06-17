#![allow(clippy::expect_used)]

//! `AgentHarnessHookInvocationResult` helper model tests.

use serde_json::json;
use workflow_core::{
    execute_runtime_agent_harness_hook, execute_runtime_agent_harness_hook_failed_closed,
    invoke_agent_harness_hook, invoke_agent_harness_hook_failed_closed, ActorId,
    AgentHarnessHookAuditRecord, AgentHarnessHookAuditRecordDefinition, AgentHarnessHookContract,
    AgentHarnessHookContractDefinition, AgentHarnessHookContractId,
    AgentHarnessHookContractVersion, AgentHarnessHookDisclosure, AgentHarnessHookDisclosureKind,
    AgentHarnessHookFailureSemantics, AgentHarnessHookInputRequirement,
    AgentHarnessHookInvocationId, AgentHarnessHookInvocationInput,
    AgentHarnessHookInvocationResult, AgentHarnessHookInvocationStatus, AgentHarnessHookKind,
    AgentHarnessHookNamedReference, AgentHarnessHookOutputRequirement, AgentHarnessHookReference,
    AgentHarnessHookSideEffectAllowance, ApprovalReferenceId, CorrelationId, EventId,
    EvidenceReferenceId, LocalCheckResultId, PolicyId, RedactionDisposition, RedactionFieldState,
    RedactionMetadata, RuntimeAgentHarnessHookInput, RuntimeAgentHarnessHookResult, SchemaVersion,
    SpecContentHash, StepId, Timestamp, TypedHandoffId, ValidationReferenceId,
    WorkReportCitationKind, WorkReportCitationTarget, WorkReportRedactionPolicy,
    WorkReportSensitivity, WorkflowId, WorkflowRunId, WorkflowVersion,
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
        redacted_fields: vec!["phase_id".to_owned()],
        field_states: vec![RedactionFieldState {
            field: "phase_id".to_owned(),
            disposition: RedactionDisposition::ReferenceOnly,
            reason: "bounded hook invocation metadata".to_owned(),
        }],
    }
}

fn hook_contract() -> AgentHarnessHookContract {
    AgentHarnessHookContract::new(AgentHarnessHookContractDefinition {
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
        failure_semantics: vec![AgentHarnessHookFailureSemantics::FailClosed],
        side_effect_allowance: AgentHarnessHookSideEffectAllowance::Unsupported,
        sensitivity: WorkReportSensitivity::Confidential,
        redaction_policy: WorkReportRedactionPolicy::ReferenceOnly,
        redaction: redaction(),
    })
    .expect("valid contract")
}

fn timestamp() -> Timestamp {
    Timestamp::parse_rfc3339("2026-06-16T12:00:00Z").expect("valid timestamp")
}

fn evidence_reference() -> AgentHarnessHookNamedReference {
    AgentHarnessHookNamedReference::new(
        "planned_work",
        AgentHarnessHookReference::EvidenceReference(
            EvidenceReferenceId::new("evidence/planned-work").expect("valid evidence id"),
        ),
    )
    .expect("valid input reference")
}

fn output_reference() -> AgentHarnessHookNamedReference {
    AgentHarnessHookNamedReference::new(
        "checkpoint_result",
        AgentHarnessHookReference::Validation(
            ValidationReferenceId::new("validation/pre-validation").expect("valid validation id"),
        ),
    )
    .expect("valid output reference")
}

fn valid_input() -> AgentHarnessHookInvocationInput {
    AgentHarnessHookInvocationInput {
        contract: hook_contract(),
        workflow_id: WorkflowId::new("workflow/self-governance").expect("valid workflow id"),
        workflow_version: WorkflowVersion::new("v1").expect("valid workflow version"),
        run_id: WorkflowRunId::new("run/self-governance").expect("valid run id"),
        schema_version: schema_version(),
        spec_hash: SpecContentHash::from_text("self-governance workflow"),
        hook_kind: AgentHarnessHookKind::BeforeValidation,
        actor: ActorId::new("system/agent-harness").expect("valid actor"),
        invoked_at: timestamp(),
        correlation_id: Some(CorrelationId::new("correlation/hook").expect("valid correlation")),
        step_id: Some(StepId::new("validation").expect("valid step id")),
        phase_id: Some("before_validation".to_owned()),
        input_references: vec![evidence_reference()],
        output_references: vec![output_reference()],
        supplemental_references: vec![
            AgentHarnessHookReference::LocalCheckResult(
                LocalCheckResultId::new("local-check/docs").expect("valid local check id"),
            ),
            AgentHarnessHookReference::TypedHandoff(
                TypedHandoffId::new("handoff/planning").expect("valid typed handoff id"),
            ),
            AgentHarnessHookReference::Policy(
                PolicyId::new("policy/governed-checkpoint").expect("valid policy id"),
            ),
            AgentHarnessHookReference::ApprovalDecision(
                ApprovalReferenceId::new("approval/pre-validation").expect("valid approval id"),
            ),
            AgentHarnessHookReference::WorkflowEvent(
                EventId::new("event/workflow-hook").expect("valid event id"),
            ),
            AgentHarnessHookReference::AuditEvent(
                EventId::new("event/audit-hook").expect("valid audit event id"),
            ),
            AgentHarnessHookReference::PolicyDecisionEvent(
                EventId::new("event/policy-hook").expect("valid policy event id"),
            ),
        ],
        require_outputs: true,
        side_effect_requested: false,
        disclosures: vec![AgentHarnessHookDisclosure::new(
            AgentHarnessHookDisclosureKind::Note,
            "bounded checkpoint context",
        )
        .expect("valid disclosure")],
        redaction: redaction(),
        sensitivity: WorkReportSensitivity::Confidential,
    }
}

fn valid_result() -> AgentHarnessHookInvocationResult {
    invoke_agent_harness_hook(valid_input()).expect("valid invocation")
}

fn valid_failed_closed_result() -> AgentHarnessHookInvocationResult {
    invoke_agent_harness_hook_failed_closed(valid_input()).expect("valid failed-closed invocation")
}

fn hook_invocation_id() -> AgentHarnessHookInvocationId {
    AgentHarnessHookInvocationId::new("hook-invocation/run-1/pre-validation")
        .expect("valid hook invocation id")
}

fn valid_audit_record() -> AgentHarnessHookAuditRecord {
    AgentHarnessHookAuditRecord::from_invocation_result(hook_invocation_id(), valid_result())
        .expect("valid hook audit record")
}

fn valid_runtime_result() -> RuntimeAgentHarnessHookResult {
    execute_runtime_agent_harness_hook(RuntimeAgentHarnessHookInput {
        hook_invocation_id: hook_invocation_id(),
        invocation: valid_input(),
    })
    .expect("valid runtime hook result")
}

fn valid_failed_closed_runtime_result() -> RuntimeAgentHarnessHookResult {
    execute_runtime_agent_harness_hook_failed_closed(RuntimeAgentHarnessHookInput {
        hook_invocation_id: hook_invocation_id(),
        invocation: valid_input(),
    })
    .expect("valid failed-closed runtime hook result")
}

#[test]
fn valid_phase_level_hook_invocation_returns_in_memory_result() {
    let result = valid_result();

    assert_eq!(result.status(), AgentHarnessHookInvocationStatus::Passed);
    assert_eq!(result.hook_kind(), AgentHarnessHookKind::BeforeValidation);
    assert_eq!(
        result.contract_id().as_str(),
        "agent-harness/hooks/pre-validation"
    );
    assert_eq!(result.workflow_id().as_str(), "workflow/self-governance");
    assert_eq!(result.workflow_version().as_str(), "v1");
    assert_eq!(result.run_id().as_str(), "run/self-governance");
    assert_eq!(result.input_references().len(), 1);
    assert_eq!(result.output_references().len(), 1);
    assert_eq!(result.supplemental_references().len(), 7);
    assert_eq!(result.disclosures().len(), 1);
}

#[test]
fn valid_failed_closed_hook_invocation_returns_in_memory_result() {
    let result = valid_failed_closed_result();

    assert_eq!(
        result.status(),
        AgentHarnessHookInvocationStatus::FailedClosed
    );
    assert_eq!(result.hook_kind(), AgentHarnessHookKind::BeforeValidation);
    assert_eq!(
        result.contract_id().as_str(),
        "agent-harness/hooks/pre-validation"
    );
    assert_eq!(result.workflow_id().as_str(), "workflow/self-governance");
    assert_eq!(result.workflow_version().as_str(), "v1");
    assert_eq!(result.run_id().as_str(), "run/self-governance");
    assert_eq!(result.input_references().len(), 1);
    assert_eq!(result.output_references().len(), 1);
    assert_eq!(result.supplemental_references().len(), 7);
    assert_eq!(result.disclosures().len(), 1);
}

#[test]
fn hook_kind_mismatch_is_rejected_without_leaking_context() {
    let mut input = valid_input();
    input.hook_kind = AgentHarnessHookKind::AfterValidation;

    let error = invoke_agent_harness_hook(input).expect_err("kind mismatch");
    assert_eq!(error.code(), "agent_harness_hook_invocation.kind.mismatch");
    assert!(!error.to_string().contains("workflow/self-governance"));
}

#[test]
fn missing_required_input_fails_closed() {
    let mut input = valid_input();
    input.input_references.clear();

    let error = invoke_agent_harness_hook(input).expect_err("missing input");
    assert_eq!(
        error.code(),
        "agent_harness_hook_invocation.inputs.missing_required"
    );
}

#[test]
fn missing_required_output_fails_closed_when_outputs_are_required() {
    let mut input = valid_input();
    input.output_references.clear();

    let error = invoke_agent_harness_hook(input).expect_err("missing output");
    assert_eq!(
        error.code(),
        "agent_harness_hook_invocation.outputs.missing_required"
    );
}

#[test]
fn missing_required_output_is_allowed_when_outputs_are_not_required() {
    let mut input = valid_input();
    input.output_references.clear();
    input.require_outputs = false;

    let result = invoke_agent_harness_hook(input).expect("before hook output can be absent");
    assert!(result.output_references().is_empty());
}

#[test]
fn duplicate_input_reference_names_are_rejected() {
    let mut input = valid_input();
    input.input_references = vec![evidence_reference(), evidence_reference()];

    let error = invoke_agent_harness_hook(input).expect_err("duplicate input");
    assert_eq!(
        error.code(),
        "agent_harness_hook_invocation.inputs.duplicate"
    );
}

#[test]
fn stable_reference_kinds_are_accepted_without_recreating_evidence() {
    let result = valid_result();
    let serialized = serde_json::to_string(&result).expect("serialize result");

    assert!(serialized.contains("evidence_reference"));
    assert!(serialized.contains("local_check_result"));
    assert!(serialized.contains("typed_handoff"));
    assert!(serialized.contains("validation"));
    assert!(serialized.contains("policy"));
    assert!(serialized.contains("approval_decision"));
    assert!(!serialized.contains("EvidenceReference"));
    assert!(!serialized.contains("title"));
    assert!(!serialized.contains("summary"));
}

#[test]
fn absent_optional_references_do_not_fabricate_citations() {
    let mut input = valid_input();
    input.supplemental_references.clear();

    let result = invoke_agent_harness_hook(input).expect("valid without optional refs");
    assert!(result.supplemental_references().is_empty());
}

#[test]
fn side_effect_requests_are_rejected_without_mutating_runtime_state() {
    let mut input = valid_input();
    input.side_effect_requested = true;

    let error = invoke_agent_harness_hook(input).expect_err("side effects rejected");
    assert_eq!(
        error.code(),
        "agent_harness_hook_invocation.side_effect.unsupported"
    );
}

#[test]
fn failed_closed_side_effect_requests_are_rejected_without_leaking() {
    let mut input = valid_input();
    input.side_effect_requested = true;

    let error = invoke_agent_harness_hook_failed_closed(input)
        .expect_err("failed-closed side effects rejected");
    assert_eq!(
        error.code(),
        "agent_harness_hook_invocation.side_effect.unsupported"
    );
    assert!(!error.to_string().contains("workflow/self-governance"));
    assert!(!error.to_string().contains("bounded checkpoint context"));
}

#[test]
fn secret_like_reference_values_are_rejected_without_leaking() {
    let mut input = valid_input();
    input.supplemental_references = vec![AgentHarnessHookReference::Policy(
        PolicyId::new("policy/authorization-token").expect("identifier accepts canonical chars"),
    )];

    let error = invoke_agent_harness_hook(input).expect_err("secret-like policy ref");
    assert_eq!(
        error.code(),
        "agent_harness_hook_invocation.secret_like_value"
    );
    assert!(!error.to_string().contains("authorization-token"));
}

#[test]
fn secret_like_disclosures_are_rejected_without_leaking() {
    let error = AgentHarnessHookDisclosure::new(
        AgentHarnessHookDisclosureKind::Risk,
        "contains raw_provider_payload marker",
    )
    .expect_err("secret-like disclosure rejected");
    assert_eq!(
        error.code(),
        "agent_harness_hook_invocation.secret_like_value"
    );
    assert!(!error.to_string().contains("raw_provider_payload"));
}

#[test]
fn debug_output_does_not_leak_context_references_or_disclosures() {
    let result = valid_result();
    let debug = format!("{result:?}");

    assert!(!debug.contains("workflow/self-governance"));
    assert!(!debug.contains("run/self-governance"));
    assert!(!debug.contains("planned_work"));
    assert!(!debug.contains("checkpoint_result"));
    assert!(!debug.contains("bounded checkpoint context"));
    assert!(!debug.contains("local-check/docs"));
}

#[test]
fn serialization_does_not_include_forbidden_raw_payload_markers() {
    let serialized = serde_json::to_string(&valid_result()).expect("serialize result");

    assert!(!serialized.contains("raw_provider_payload"));
    assert!(!serialized.contains("raw_command_output"));
    assert!(!serialized.contains("raw_spec_contents"));
    assert!(!serialized.contains("raw_parser_payload"));
    assert!(!serialized.contains("authorization"));
    assert!(!serialized.contains("private_key"));
}

#[test]
fn serde_round_trip_for_valid_invocation_result() {
    let result = valid_result();
    let serialized = serde_json::to_string(&result).expect("serialize result");
    let deserialized: AgentHarnessHookInvocationResult =
        serde_json::from_str(&serialized).expect("deserialize result");

    assert_eq!(deserialized, result);
}

#[test]
fn invalid_serialized_invocation_result_fails_closed() {
    let mut value = serde_json::to_value(valid_result()).expect("serialize result");
    value["phase_id"] = json!("bad phase id");

    let error = serde_json::from_value::<AgentHarnessHookInvocationResult>(value)
        .expect_err("invalid phase id");
    assert!(error
        .to_string()
        .contains("agent_harness_hook_invocation.identifier.invalid_character"));
    assert!(!error.to_string().contains("bad phase id"));
}

#[test]
fn invocation_result_does_not_encode_runtime_execution_behavior() {
    let serialized = serde_json::to_string(&valid_result()).expect("serialize result");

    assert!(!serialized.contains("execute_hook"));
    assert!(!serialized.contains("append_event"));
    assert!(!serialized.contains("emit_audit"));
    assert!(!serialized.contains("state_backend"));
    assert!(!serialized.contains("local_executor"));
    assert!(!serialized.contains("cli_command"));
    assert!(!serialized.contains("workflow_schema"));
}

#[test]
fn hook_invocation_id_rejects_invalid_and_secret_like_values_without_leaking() {
    let invalid = AgentHarnessHookInvocationId::new("bad id").expect_err("invalid id");
    assert_eq!(
        invalid.code(),
        "agent_harness_hook_invocation.identifier.invalid_character"
    );
    assert!(!invalid.to_string().contains("bad id"));

    let secret = AgentHarnessHookInvocationId::new("hook-invocation/authorization-token")
        .expect_err("secret-like hook invocation id");
    assert_eq!(
        secret.code(),
        "agent_harness_hook_invocation.secret_like_value"
    );
    assert!(!secret.to_string().contains("authorization-token"));
}

#[test]
fn valid_hook_audit_record_is_model_only_and_accessible() {
    let record = valid_audit_record();

    assert_eq!(
        record.hook_invocation_id().as_str(),
        "hook-invocation/run-1/pre-validation"
    );
    assert_eq!(
        record.contract_id().as_str(),
        "agent-harness/hooks/pre-validation"
    );
    assert_eq!(record.contract_version().as_str(), "v1");
    assert_eq!(record.hook_kind(), AgentHarnessHookKind::BeforeValidation);
    assert_eq!(record.workflow_id().as_str(), "workflow/self-governance");
    assert_eq!(record.workflow_version().as_str(), "v1");
    assert_eq!(record.run_id().as_str(), "run/self-governance");
    assert_eq!(record.status(), AgentHarnessHookInvocationStatus::Passed);
    assert_eq!(record.input_references().len(), 1);
    assert_eq!(record.output_references().len(), 1);
    assert_eq!(record.supplemental_references().len(), 7);
    assert_eq!(record.disclosures().len(), 1);
    assert_eq!(record.sensitivity(), WorkReportSensitivity::Confidential);
}

#[test]
fn runtime_hook_execution_returns_in_memory_result_and_audit_record() {
    let result = valid_runtime_result();

    assert_eq!(
        result.hook_invocation_id().as_str(),
        "hook-invocation/run-1/pre-validation"
    );
    assert_eq!(
        result.audit_record().hook_invocation_id(),
        result.hook_invocation_id()
    );
    assert_eq!(
        result.invocation_result().status(),
        AgentHarnessHookInvocationStatus::Passed
    );
    assert_eq!(
        result.audit_record().status(),
        result.invocation_result().status()
    );
    assert_eq!(result.invocation_result().input_references().len(), 1);
    assert_eq!(result.invocation_result().output_references().len(), 1);
}

#[test]
fn runtime_hook_execution_returns_failed_closed_result_and_audit_record() {
    let result = valid_failed_closed_runtime_result();

    assert_eq!(
        result.hook_invocation_id().as_str(),
        "hook-invocation/run-1/pre-validation"
    );
    assert_eq!(
        result.invocation_result().status(),
        AgentHarnessHookInvocationStatus::FailedClosed
    );
    assert_eq!(
        result.audit_record().status(),
        AgentHarnessHookInvocationStatus::FailedClosed
    );
    assert_eq!(
        result.audit_record().hook_invocation_id(),
        result.hook_invocation_id()
    );
    assert_eq!(result.invocation_result().input_references().len(), 1);
    assert_eq!(result.invocation_result().output_references().len(), 1);
}

#[test]
fn runtime_hook_execution_preserves_caller_supplied_invocation_id_for_report_citation() {
    let result = valid_runtime_result();

    assert_eq!(
        result.report_citation_target().citation_kind(),
        WorkReportCitationKind::AgentHarnessHook
    );
    if let WorkReportCitationTarget::AgentHarnessHook { hook_invocation_id } =
        result.report_citation_target()
    {
        assert_eq!(
            hook_invocation_id.as_str(),
            "hook-invocation/run-1/pre-validation"
        );
    }
}

#[test]
fn runtime_hook_result_into_parts_returns_owned_validated_parts() {
    let result = valid_runtime_result();
    let (hook_invocation_id, invocation_result, audit_record) = result.into_parts();

    assert_eq!(
        hook_invocation_id.as_str(),
        "hook-invocation/run-1/pre-validation"
    );
    assert_eq!(
        invocation_result.status(),
        AgentHarnessHookInvocationStatus::Passed
    );
    assert_eq!(audit_record.hook_invocation_id(), &hook_invocation_id);
}

#[test]
fn runtime_hook_result_rejects_mismatched_audit_record_without_leaking_ids() {
    let error = RuntimeAgentHarnessHookResult::new(
        AgentHarnessHookInvocationId::new("hook-invocation/run-2/pre-validation")
            .expect("valid hook invocation id"),
        valid_result(),
        valid_audit_record(),
    )
    .expect_err("mismatched hook invocation id");

    assert_eq!(
        error.code(),
        "agent_harness_hook_runtime.invocation_id.mismatch"
    );
    assert!(!error.to_string().contains("hook-invocation/run-1"));
    assert!(!error.to_string().contains("hook-invocation/run-2"));
}

#[test]
fn runtime_hook_execution_fails_closed_on_hook_kind_mismatch() {
    let mut input = valid_input();
    input.hook_kind = AgentHarnessHookKind::AfterValidation;

    let error = execute_runtime_agent_harness_hook(RuntimeAgentHarnessHookInput {
        hook_invocation_id: hook_invocation_id(),
        invocation: input,
    })
    .expect_err("kind mismatch");

    assert_eq!(error.code(), "agent_harness_hook_invocation.kind.mismatch");
    assert!(!error.to_string().contains("workflow/self-governance"));
}

#[test]
fn runtime_hook_execution_fails_closed_when_required_inputs_are_missing() {
    let mut input = valid_input();
    input.input_references.clear();

    let error = execute_runtime_agent_harness_hook(RuntimeAgentHarnessHookInput {
        hook_invocation_id: hook_invocation_id(),
        invocation: input,
    })
    .expect_err("missing input reference");

    assert_eq!(
        error.code(),
        "agent_harness_hook_invocation.inputs.missing_required"
    );
}

#[test]
fn runtime_hook_execution_fails_closed_when_required_outputs_are_missing() {
    let mut input = valid_input();
    input.output_references.clear();

    let error = execute_runtime_agent_harness_hook(RuntimeAgentHarnessHookInput {
        hook_invocation_id: hook_invocation_id(),
        invocation: input,
    })
    .expect_err("missing output reference");

    assert_eq!(
        error.code(),
        "agent_harness_hook_invocation.outputs.missing_required"
    );
}

#[test]
fn runtime_hook_execution_rejects_side_effect_requests_without_runtime_mutation() {
    let mut input = valid_input();
    input.side_effect_requested = true;

    let error = execute_runtime_agent_harness_hook(RuntimeAgentHarnessHookInput {
        hook_invocation_id: hook_invocation_id(),
        invocation: input,
    })
    .expect_err("side effects rejected");

    assert_eq!(
        error.code(),
        "agent_harness_hook_invocation.side_effect.unsupported"
    );
}

#[test]
fn runtime_failed_closed_execution_rejects_side_effect_requests_without_leaking() {
    let mut input = valid_input();
    input.side_effect_requested = true;

    let error = execute_runtime_agent_harness_hook_failed_closed(RuntimeAgentHarnessHookInput {
        hook_invocation_id: hook_invocation_id(),
        invocation: input,
    })
    .expect_err("failed-closed side effects rejected");

    assert_eq!(
        error.code(),
        "agent_harness_hook_invocation.side_effect.unsupported"
    );
    assert!(!error.to_string().contains("workflow/self-governance"));
    assert!(!error.to_string().contains("bounded checkpoint context"));
}

#[test]
fn runtime_hook_execution_accepts_stable_references_without_creating_evidence() {
    let result = valid_runtime_result();

    assert!(result
        .invocation_result()
        .supplemental_references()
        .contains(&AgentHarnessHookReference::LocalCheckResult(
            LocalCheckResultId::new("local-check/docs").expect("valid local check id")
        )));
    assert!(result
        .invocation_result()
        .input_references()
        .iter()
        .any(|reference| matches!(
            reference.reference(),
            AgentHarnessHookReference::EvidenceReference(_)
        )));

    let invocation_json =
        serde_json::to_string(result.invocation_result()).expect("serialize invocation result");
    assert!(invocation_json.contains("evidence_reference"));
    assert!(invocation_json.contains("local_check_result"));
    assert!(!invocation_json.contains("EvidenceReference"));
    assert!(!invocation_json.contains("title"));
    assert!(!invocation_json.contains("summary"));
}

#[test]
fn runtime_hook_execution_does_not_fabricate_optional_references() {
    let mut input = valid_input();
    input.supplemental_references.clear();

    let result = execute_runtime_agent_harness_hook(RuntimeAgentHarnessHookInput {
        hook_invocation_id: hook_invocation_id(),
        invocation: input,
    })
    .expect("valid runtime hook result without optional references");

    assert!(result
        .invocation_result()
        .supplemental_references()
        .is_empty());
    assert!(result.audit_record().supplemental_references().is_empty());
}

#[test]
fn runtime_hook_execution_secret_like_values_fail_without_leaking() {
    let mut input = valid_input();
    input.supplemental_references = vec![AgentHarnessHookReference::Policy(
        PolicyId::new("policy/raw_provider_payload").expect("identifier accepts canonical chars"),
    )];

    let error = execute_runtime_agent_harness_hook(RuntimeAgentHarnessHookInput {
        hook_invocation_id: hook_invocation_id(),
        invocation: input,
    })
    .expect_err("secret-like value rejected");

    assert_eq!(
        error.code(),
        "agent_harness_hook_invocation.secret_like_value"
    );
    assert!(!error.to_string().contains("raw_provider_payload"));
}

#[test]
fn runtime_hook_debug_does_not_leak_context_references_or_disclosures() {
    let result = valid_runtime_result();
    let debug = format!("{result:?}");

    assert!(debug.contains("RuntimeAgentHarnessHookResult"));
    assert!(!debug.contains("hook-invocation/run-1/pre-validation"));
    assert!(!debug.contains("workflow/self-governance"));
    assert!(!debug.contains("run/self-governance"));
    assert!(!debug.contains("planned_work"));
    assert!(!debug.contains("checkpoint_result"));
    assert!(!debug.contains("bounded checkpoint context"));
    assert!(!debug.contains("local-check/docs"));
}

#[test]
fn runtime_hook_result_does_not_encode_executor_events_state_or_side_effects() {
    let debug = format!("{:?}", valid_runtime_result());

    assert!(!debug.contains("append_event"));
    assert!(!debug.contains("emit_audit"));
    assert!(!debug.contains("audit_sink"));
    assert!(!debug.contains("state_backend"));
    assert!(!debug.contains("local_executor"));
    assert!(!debug.contains("workflow_run"));
    assert!(!debug.contains("workflow_snapshot"));
    assert!(!debug.contains("execute_command"));
    assert!(!debug.contains("local_check_handler"));
    assert!(!debug.contains("adapter_invocation"));
    assert!(!debug.contains("filesystem"));
    assert!(!debug.contains("cli_command"));
    assert!(!debug.contains("workflow_schema"));
}

#[test]
fn hook_audit_record_can_be_built_from_invocation_result_without_reexecuting_hook() {
    let result = valid_result();
    let record =
        AgentHarnessHookAuditRecord::from_invocation_result(hook_invocation_id(), result.clone())
            .expect("valid record");

    assert_eq!(record.status(), result.status());
    assert_eq!(record.hook_kind(), result.hook_kind());
    assert_eq!(record.input_references(), result.input_references());
    assert_eq!(record.output_references(), result.output_references());
}

#[test]
fn hook_audit_record_rejects_duplicate_output_references() {
    let mut definition = AgentHarnessHookAuditRecordDefinition {
        hook_invocation_id: hook_invocation_id(),
        contract_id: hook_contract_id(),
        contract_version: hook_contract_version(),
        hook_kind: AgentHarnessHookKind::BeforeValidation,
        workflow_id: WorkflowId::new("workflow/self-governance").expect("valid workflow id"),
        workflow_version: WorkflowVersion::new("v1").expect("valid workflow version"),
        run_id: WorkflowRunId::new("run/self-governance").expect("valid run id"),
        schema_version: schema_version(),
        spec_hash: SpecContentHash::from_text("self-governance workflow"),
        actor: ActorId::new("system/agent-harness").expect("valid actor"),
        invoked_at: timestamp(),
        correlation_id: None,
        step_id: None,
        phase_id: None,
        status: AgentHarnessHookInvocationStatus::Passed,
        input_references: vec![evidence_reference()],
        output_references: vec![output_reference()],
        supplemental_references: Vec::new(),
        disclosures: Vec::new(),
        redaction: redaction(),
        sensitivity: WorkReportSensitivity::Confidential,
    };
    definition.output_references = vec![output_reference(), output_reference()];

    let error = AgentHarnessHookAuditRecord::new(definition).expect_err("duplicate output");
    assert_eq!(error.code(), "agent_harness_hook_audit.outputs.duplicate");
}

#[test]
fn hook_audit_record_rejects_secret_like_references_without_leaking() {
    let mut record = serde_json::to_value(valid_audit_record()).expect("serialize record");
    record["supplemental_references"] = json!([
        {
            "kind": "policy",
            "id": "policy/authorization-token"
        }
    ]);

    let error = serde_json::from_value::<AgentHarnessHookAuditRecord>(record)
        .expect_err("secret-like reference rejected");
    assert!(error
        .to_string()
        .contains("agent_harness_hook_invocation.secret_like_value"));
    assert!(!error.to_string().contains("authorization-token"));
}

#[test]
fn hook_audit_record_debug_does_not_leak_context_or_disclosures() {
    let record = valid_audit_record();
    let debug = format!("{record:?}");

    assert!(debug.contains("AgentHarnessHookAuditRecord"));
    assert!(!debug.contains("hook-invocation/run-1/pre-validation"));
    assert!(!debug.contains("workflow/self-governance"));
    assert!(!debug.contains("run/self-governance"));
    assert!(!debug.contains("planned_work"));
    assert!(!debug.contains("checkpoint_result"));
    assert!(!debug.contains("bounded checkpoint context"));
    assert!(!debug.contains("local-check/docs"));
}

#[test]
fn hook_audit_record_serialization_does_not_include_forbidden_raw_payload_markers() {
    let serialized = serde_json::to_string(&valid_audit_record()).expect("serialize record");

    assert!(!serialized.contains("raw_provider_payload"));
    assert!(!serialized.contains("raw_command_output"));
    assert!(!serialized.contains("raw_spec_contents"));
    assert!(!serialized.contains("raw_parser_payload"));
    assert!(!serialized.contains("authorization"));
    assert!(!serialized.contains("private_key"));
}

#[test]
fn hook_audit_record_serde_round_trip_and_invalid_payload_fails_closed() {
    let record = valid_audit_record();
    let serialized = serde_json::to_string(&record).expect("serialize record");
    let deserialized: AgentHarnessHookAuditRecord =
        serde_json::from_str(&serialized).expect("deserialize record");
    assert_eq!(deserialized, record);

    let mut value = serde_json::to_value(record).expect("serialize record");
    value["phase_id"] = json!("bad phase id");

    let error =
        serde_json::from_value::<AgentHarnessHookAuditRecord>(value).expect_err("invalid phase id");
    assert!(error
        .to_string()
        .contains("agent_harness_hook_invocation.identifier.invalid_character"));
    assert!(!error.to_string().contains("bad phase id"));
}

#[test]
fn hook_audit_record_does_not_encode_runtime_event_or_sink_behavior() {
    let serialized = serde_json::to_string(&valid_audit_record()).expect("serialize record");

    assert!(!serialized.contains("append_event"));
    assert!(!serialized.contains("emit_audit"));
    assert!(!serialized.contains("audit_sink"));
    assert!(!serialized.contains("state_backend"));
    assert!(!serialized.contains("local_executor"));
    assert!(!serialized.contains("execute_hook"));
    assert!(!serialized.contains("workflow_schema"));
    assert!(!serialized.contains("cli_command"));
}
