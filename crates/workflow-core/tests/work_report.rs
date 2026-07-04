#![allow(clippy::expect_used)]

//! `WorkReport` core model tests.

use std::fmt::Write as _;
use std::fs;
use std::sync::atomic::{AtomicU64, Ordering};

use serde_json::json;
use sha2::{Digest, Sha256};
use workflow_core::{
    derive_workflow_report_artifact_gate_policy, expose_terminal_local_work_report_result,
    generate_terminal_local_work_report,
    generate_terminal_local_work_report_with_side_effect_discovery, parse_workflow_spec_yaml,
    validate_work_report_artifact_side_effect_integrity, ActorId, AgentHarnessHookDisclosureId,
    AgentHarnessHookInvocationId, ApprovalReferenceId, CancellationRecord, CorrelationId, EventId,
    EventLogStore, EventSequenceNumber, EvidenceReferenceId, FailureClass, FailureRecord,
    IdempotencyKey, LocalStateBackend, RedactionDisposition, RedactionFieldState,
    RedactionMetadata, RunSnapshotStore, SchemaVersion, SideEffectAuthority,
    SideEffectAuthorityDecision, SideEffectCapability, SideEffectId, SideEffectIdempotencyBinding,
    SideEffectIdempotencyScope, SideEffectLifecycleState, SideEffectRecord,
    SideEffectRecordDefinition, SideEffectRecordStore, SideEffectSensitivity, SideEffectTargetKind,
    SideEffectTargetReference, SpecContentHash, StepId, TerminalLocalWorkReportInput,
    TerminalLocalWorkReportResult, TerminalLocalWorkReportSideEffectDiscoveryInput, Timestamp,
    TypedHandoffId, ValidationReferenceId, WorkReport,
    WorkReportArtifactHighAssuranceDisclosurePolicy, WorkReportArtifactHighAssuranceRequirement,
    WorkReportArtifactRecord, WorkReportArtifactRequirement,
    WorkReportArtifactRequirementDefinition, WorkReportArtifactSideEffectIntegrityInput,
    WorkReportArtifactStore, WorkReportArtifactUnsupportedHighAssuranceRequirement,
    WorkReportCitation, WorkReportCitationDefinition, WorkReportCitationKind,
    WorkReportCitationTarget, WorkReportContractId, WorkReportContractVersion,
    WorkReportDefinition, WorkReportGenerationContext, WorkReportHandoffNote,
    WorkReportHighAssuranceApprovalDecision, WorkReportHighAssuranceApprovalDisclosure,
    WorkReportHighAssuranceApprovalDisclosureDefinition, WorkReportHighAssuranceExpirationPosture,
    WorkReportHighAssuranceRequesterApproverPosture, WorkReportHighAssuranceRevocationPosture,
    WorkReportId, WorkReportIncompleteWorkDisclosure, WorkReportKnownLimitation, WorkReportRisk,
    WorkReportSection, WorkReportSectionKind, WorkReportSensitivity, WorkReportStableReference,
    WorkReportStatus, WorkflowDefinition, WorkflowId, WorkflowReportArtifactGateDerivationInput,
    WorkflowRun, WorkflowRunEvent, WorkflowRunEventKind, WorkflowRunId, WorkflowRunStatus,
    WorkflowVersion, SUPPORTED_SCHEMA_VERSION,
};

static NEXT_ARTIFACT_TEST: AtomicU64 = AtomicU64::new(1);

fn report_id() -> WorkReportId {
    WorkReportId::new("report/local-run").expect("valid report id")
}

fn contract_id() -> WorkReportContractId {
    WorkReportContractId::new("governed/handoff").expect("valid contract id")
}

fn contract_version() -> WorkReportContractVersion {
    WorkReportContractVersion::new("v1").expect("valid contract version")
}

fn workflow_id() -> WorkflowId {
    WorkflowId::new("workflow/intake").expect("valid workflow id")
}

fn workflow_version() -> WorkflowVersion {
    WorkflowVersion::new("v1").expect("valid workflow version")
}

fn schema_version() -> SchemaVersion {
    SchemaVersion::new("workflowos/v0").expect("valid schema version")
}

fn spec_hash() -> SpecContentHash {
    SpecContentHash::from_text("workflow spec")
}

fn run_id() -> WorkflowRunId {
    WorkflowRunId::new("run-123").expect("valid run id")
}

fn actor_id() -> ActorId {
    ActorId::new("system/work-report").expect("valid actor")
}

fn generated_at() -> Timestamp {
    Timestamp::parse_rfc3339("2026-06-05T12:00:00Z").expect("valid timestamp")
}

fn redaction() -> RedactionMetadata {
    RedactionMetadata::empty()
}

fn redaction_with(field: &str, reason: &str) -> RedactionMetadata {
    RedactionMetadata {
        redacted_fields: vec![field.to_owned()],
        field_states: vec![RedactionFieldState {
            field: field.to_owned(),
            disposition: RedactionDisposition::Redacted,
            reason: reason.to_owned(),
        }],
    }
}

fn generation_context(status: WorkReportStatus) -> WorkReportGenerationContext {
    WorkReportGenerationContext {
        workflow_id: workflow_id(),
        workflow_version: workflow_version(),
        schema_version: schema_version(),
        spec_hash: spec_hash(),
        run_id: run_id(),
        terminal_run_status: status,
        generated_at: generated_at(),
        generated_by: actor_id(),
        correlation_id: Some(CorrelationId::new("correlation-1").expect("valid correlation")),
    }
}

fn evidence_citation() -> WorkReportCitation {
    WorkReportCitation::new(WorkReportCitationDefinition {
        target: WorkReportCitationTarget::EvidenceReference {
            evidence_reference_id: EvidenceReferenceId::new("evidence-1").expect("valid evidence"),
        },
        summary: Some("bounded evidence summary".to_owned()),
        missing: false,
        redaction: redaction(),
        sensitivity: WorkReportSensitivity::Confidential,
    })
    .expect("valid citation")
}

fn section(kind: WorkReportSectionKind) -> WorkReportSection {
    WorkReportSection::new(
        kind,
        Some("bounded section summary".to_owned()),
        vec![evidence_citation()],
    )
    .expect("valid section")
}

fn required_sections() -> Vec<WorkReportSection> {
    WorkReportSectionKind::v1_required_kinds()
        .into_iter()
        .map(section)
        .collect()
}

fn valid_report() -> WorkReport {
    WorkReport::new(WorkReportDefinition {
        report_id: report_id(),
        report_contract_id: contract_id(),
        report_contract_version: contract_version(),
        generation_context: generation_context(WorkReportStatus::Completed),
        sections: required_sections(),
        incomplete_work: vec![WorkReportIncompleteWorkDisclosure::new(
            "no deferred work",
            vec![evidence_citation()],
        )
        .expect("valid incomplete work disclosure")],
        known_limitations: vec![WorkReportKnownLimitation::new(
            "fixture first evaluation",
            vec![evidence_citation()],
        )
        .expect("valid known limitation")],
        risks: vec![
            WorkReportRisk::new("no production backend", vec![evidence_citation()])
                .expect("valid risk"),
        ],
        handoff_notes: vec![WorkReportHandoffNote::new(
            "operator should review citations",
            vec![evidence_citation()],
        )
        .expect("valid handoff note")],
        high_assurance_approval: None,
        sensitivity: WorkReportSensitivity::Confidential,
        redaction: redaction(),
    })
    .expect("valid work report")
}

fn workflow_with_artifact_requirement(requirement: Option<&str>) -> WorkflowDefinition {
    let requirement_yaml = requirement
        .map(|value| {
            format!(
                r"
report_artifact_requirements:
  high_assurance_approval: {value}
"
            )
        })
        .unwrap_or_default();
    parse_workflow_spec_yaml(&format!(
        r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: approval/request-review
version: v0
name: Request Review
{requirement_yaml}
steps:
  - id: draft
    skill_ref:
      id: local/draft-summary
      version: v0
"
    ))
    .expect("workflow parses")
}

fn run_event(sequence: u64, kind: WorkflowRunEventKind) -> WorkflowRunEvent {
    WorkflowRunEvent {
        sequence_number: EventSequenceNumber::new(sequence).expect("valid sequence"),
        event_id: EventId::new(format!("event-{sequence}")).expect("valid event id"),
        timestamp: generated_at(),
        run_id: run_id(),
        workflow_id: workflow_id(),
        schema_version: schema_version(),
        workflow_version: workflow_version(),
        spec_content_hash: spec_hash(),
        correlation_id: Some(CorrelationId::new("correlation-1").expect("valid correlation")),
        actor: Some(actor_id()),
        idempotency_key: None,
        kind,
    }
}

fn terminal_run(status: WorkflowRunStatus) -> WorkflowRun {
    let mut events = vec![
        run_event(1, WorkflowRunEventKind::RunCreated { summary: None }),
        run_event(2, WorkflowRunEventKind::RunValidated),
        run_event(3, WorkflowRunEventKind::RunStarted),
    ];
    let terminal_event = terminal_event_kind(status).expect("supported terminal fixture status");
    events.push(run_event(4, terminal_event));
    WorkflowRun::rehydrate(&events).expect("terminal run rehydrates")
}

fn terminal_event_kind(status: WorkflowRunStatus) -> Option<WorkflowRunEventKind> {
    match status {
        WorkflowRunStatus::Completed => WorkflowRunEventKind::RunCompleted,
        WorkflowRunStatus::Failed => WorkflowRunEventKind::RunFailed(FailureRecord {
            code: "runtime.failure".to_owned(),
            message: "failed safely".to_owned(),
            failure_class: FailureClass::Unknown,
        }),
        WorkflowRunStatus::Canceled => WorkflowRunEventKind::RunCanceled(CancellationRecord {
            run_id: run_id(),
            reason: "operator canceled".to_owned(),
            actor: actor_id(),
            canceled_at: generated_at(),
            correlation_id: CorrelationId::new("correlation-1").expect("valid correlation"),
        }),
        _ => return None,
    }
    .into()
}

fn running_run() -> WorkflowRun {
    let events = vec![
        run_event(1, WorkflowRunEventKind::RunCreated { summary: None }),
        run_event(2, WorkflowRunEventKind::RunValidated),
        run_event(3, WorkflowRunEventKind::RunStarted),
    ];
    WorkflowRun::rehydrate(&events).expect("running run rehydrates")
}

fn terminal_generation_input(run: &WorkflowRun) -> TerminalLocalWorkReportInput<'_> {
    TerminalLocalWorkReportInput {
        report_id: report_id(),
        report_contract_id: contract_id(),
        report_contract_version: contract_version(),
        run,
        generated_at: generated_at(),
        generated_by: actor_id(),
        correlation_id: Some(CorrelationId::new("correlation-1").expect("valid correlation")),
        sensitivity: WorkReportSensitivity::Confidential,
        redaction: redaction(),
        evidence_reference_ids: vec![EvidenceReferenceId::new("evidence-1").expect("valid id")],
        validation_reference_ids: vec![
            ValidationReferenceId::new("validation/schema-version").expect("valid validation id")
        ],
        local_check_result_references: vec![WorkReportStableReference::new(
            "local-check-result/docs/passed",
        )
        .expect("valid local check result reference")],
        workflow_event_ids: vec![EventId::new("event-4").expect("valid event id")],
        audit_event_ids: vec![EventId::new("audit-event-1").expect("valid audit event id")],
        adapter_telemetry_references: vec![
            WorkReportStableReference::new("adapter/invocation/1").expect("valid telemetry ref")
        ],
        policy_event_ids: vec![EventId::new("policy-event-1").expect("valid policy event id")],
        approval_reference_ids: vec![
            ApprovalReferenceId::new("approval-1").expect("valid approval reference")
        ],
        high_assurance_approval: None,
        typed_handoff_ids: vec![
            TypedHandoffId::new("typed-handoff/final-review").expect("valid typed handoff id")
        ],
        agent_harness_hook_invocation_ids: Vec::new(),
        agent_harness_hook_disclosure_ids: Vec::new(),
        side_effect_ids: Vec::new(),
        incomplete_work: vec!["No deferred work beyond report artifact persistence.".to_owned()],
        known_limitations: vec!["Generated report is in memory only.".to_owned()],
        risks: vec!["Report citation set depends on supplied stable references.".to_owned()],
        handoff_notes: vec!["Operator should review cited references.".to_owned()],
    }
}

fn generated_report_for(status: WorkflowRunStatus) -> WorkReport {
    let run = terminal_run(status);
    generate_terminal_local_work_report(terminal_generation_input(&run))
        .expect("terminal report generated")
}

fn high_assurance_disclosure() -> WorkReportHighAssuranceApprovalDisclosure {
    WorkReportHighAssuranceApprovalDisclosure::new(
        WorkReportHighAssuranceApprovalDisclosureDefinition {
            validation_used: true,
            validation_passed: true,
            decision: WorkReportHighAssuranceApprovalDecision::Granted,
            requester_approver_posture:
                WorkReportHighAssuranceRequesterApproverPosture::MustDifferValidated,
            required_reference_count: 2,
            supplied_reference_count: 2,
            expiration_posture: WorkReportHighAssuranceExpirationPosture::NotRequired,
            revocation_posture: WorkReportHighAssuranceRevocationPosture::Unsupported,
            denial_fail_closed: true,
        },
    )
    .expect("valid high-assurance disclosure")
}

fn artifact_record() -> WorkReportArtifactRecord {
    WorkReportArtifactRecord::new(valid_report()).expect("valid artifact record")
}

fn temp_state_backend(name: &str) -> LocalStateBackend {
    let id = NEXT_ARTIFACT_TEST.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!(
        "workflow-os-report-artifact-{name}-{}-{id}",
        std::process::id()
    ));
    if root.exists() {
        fs::remove_dir_all(&root).expect("stale artifact test state removed");
    }
    LocalStateBackend::new(root).expect("local backend")
}

fn side_effect_record_for_run(run: &WorkflowRun, side_effect_id: SideEffectId) -> SideEffectRecord {
    let identity = &run.snapshot.identity;
    SideEffectRecord::new(SideEffectRecordDefinition {
        side_effect_id,
        lifecycle_state: SideEffectLifecycleState::Proposed,
        target: SideEffectTargetReference::new(
            SideEffectTargetKind::AdapterResource,
            "github/pull-request/42",
        )
        .expect("valid target"),
        capability: SideEffectCapability::GitHubWrite,
        authority: SideEffectAuthority::new(
            SideEffectAuthorityDecision::NotEvaluated,
            Vec::new(),
            Vec::new(),
        )
        .expect("valid authority"),
        actor: Some(actor_id()),
        system_actor: None,
        workflow_id: identity.workflow_id.clone(),
        workflow_version: identity.workflow_version.clone(),
        schema_version: identity.schema_version.clone(),
        spec_hash: identity.spec_content_hash.clone(),
        run_id: identity.run_id.clone(),
        step_id: Some(StepId::new("step/implementation").expect("valid step id")),
        skill_id: None,
        skill_version: None,
        adapter_id: None,
        adapter_kind: None,
        integration_id: None,
        idempotency: SideEffectIdempotencyBinding::new(
            IdempotencyKey::new("idem/work-report-side-effect").expect("valid idempotency key"),
            SideEffectIdempotencyScope::Run,
            None,
            None,
        )
        .expect("valid idempotency"),
        references: Vec::new(),
        outcome_reference: None,
        created_at: generated_at(),
        updated_at: Some(generated_at()),
        correlation_id: None,
        summary: Some("bounded side-effect summary".to_owned()),
        reason_codes: Vec::new(),
        sensitivity: SideEffectSensitivity::Confidential,
        redaction: RedactionMetadata::empty(),
    })
    .expect("valid side-effect record")
}

fn artifact_with_side_effect_ids(
    side_effect_ids: Vec<SideEffectId>,
) -> (WorkflowRun, WorkReportArtifactRecord) {
    let run = terminal_run(WorkflowRunStatus::Completed);
    let report = generate_terminal_local_work_report(TerminalLocalWorkReportInput {
        side_effect_ids,
        ..terminal_generation_input(&run)
    })
    .expect("report with side-effect citations");
    let artifact = WorkReportArtifactRecord::new(report).expect("valid report artifact");
    (run, artifact)
}

fn side_effect_record_with_mismatched_workflow_id(
    run: &WorkflowRun,
    side_effect_id: SideEffectId,
) -> SideEffectRecord {
    let record = side_effect_record_for_run(run, side_effect_id);
    let mut value = serde_json::to_value(record).expect("serialize side-effect record");
    value["workflow_id"] = json!("workflow/other");
    serde_json::from_value(value).expect("valid side-effect record with mismatched workflow id")
}

fn encoded(value: &str) -> String {
    let digest = Sha256::digest(value.as_bytes());
    let mut output = String::with_capacity(digest.len() * 2);
    for byte in digest {
        write!(output, "{byte:02x}").expect("write to string");
    }
    output
}

fn artifact_path(
    backend: &LocalStateBackend,
    artifact: &WorkReportArtifactRecord,
) -> std::path::PathBuf {
    backend
        .root()
        .join("work_reports")
        .join(encoded(artifact.run_id().as_str()))
        .join(format!("{}.json", encoded(artifact.report_id().as_str())))
}

struct FailingSideEffectRecordStore {
    code: &'static str,
}

impl SideEffectRecordStore for FailingSideEffectRecordStore {
    fn write_side_effect_record(
        &self,
        _record: &SideEffectRecord,
    ) -> Result<(), workflow_core::WorkflowOsError> {
        Err(workflow_core::WorkflowOsError::invalid_state(
            self.code,
            "failing test store write",
        ))
    }

    fn read_side_effect_record(
        &self,
        _side_effect_id: &SideEffectId,
    ) -> Result<Option<SideEffectRecord>, workflow_core::WorkflowOsError> {
        Err(workflow_core::WorkflowOsError::invalid_state(
            self.code,
            "failing test store read",
        ))
    }

    fn list_side_effect_records(
        &self,
        _run_id: &WorkflowRunId,
    ) -> Result<Vec<SideEffectRecord>, workflow_core::WorkflowOsError> {
        Err(workflow_core::WorkflowOsError::invalid_state(
            self.code,
            "failing test store list",
        ))
    }

    fn list_side_effect_records_for_workflow_run(
        &self,
        _workflow_id: &WorkflowId,
        _run_id: &WorkflowRunId,
    ) -> Result<Vec<SideEffectRecord>, workflow_core::WorkflowOsError> {
        Err(workflow_core::WorkflowOsError::invalid_state(
            self.code,
            "failing test store list",
        ))
    }
}

#[test]
fn valid_minimal_work_report() {
    let report = valid_report();

    assert_eq!(report.report_id().as_str(), "report/local-run");
    assert_eq!(report.report_contract_id().as_str(), "governed/handoff");
    assert_eq!(report.report_contract_version().as_str(), "v1");
    assert_eq!(report.sections().len(), 11);
    assert_eq!(report.sensitivity(), WorkReportSensitivity::Confidential);
}

#[test]
fn required_identity_fields_are_accessible() {
    let report = valid_report();
    let context = report.generation_context();

    assert_eq!(context.workflow_id.as_str(), "workflow/intake");
    assert_eq!(context.workflow_version.as_str(), "v1");
    assert_eq!(context.schema_version.as_str(), "workflowos/v0");
    assert_eq!(context.spec_hash.as_str(), spec_hash().as_str());
    assert_eq!(context.run_id.as_str(), "run-123");
    assert_eq!(context.generated_by.as_str(), "system/work-report");
    assert_eq!(context.terminal_run_status, WorkReportStatus::Completed);
}

#[test]
fn invalid_report_id_rejected() {
    let error = WorkReportId::new("bad id").expect_err("invalid report id");

    assert_eq!(
        error.code(),
        "work_report_contract.identifier.invalid_character"
    );
    assert!(!error.to_string().contains("bad id"));
}

#[test]
fn invalid_contract_id_and_version_rejected() {
    let id_error = WorkReportContractId::new("bad id").expect_err("invalid contract id");
    let version_error = WorkReportContractVersion::new("").expect_err("invalid version");

    assert_eq!(
        id_error.code(),
        "work_report_contract.identifier.invalid_character"
    );
    assert_eq!(
        version_error.code(),
        "work_report_contract.identifier.empty"
    );
}

#[test]
fn invalid_workflow_and_run_identity_rejected() {
    let mut value = serde_json::to_value(valid_report()).expect("serialize report");
    value["generation_context"]["workflow_id"] = json!("bad workflow");
    value["generation_context"]["run_id"] = json!("bad run");

    let error = serde_json::from_value::<WorkReport>(value).expect_err("invalid identity fails");
    assert!(!error.to_string().contains("bad workflow"));
    assert!(!error.to_string().contains("bad run"));
}

#[test]
fn invalid_schema_version_rejected() {
    let mut value = serde_json::to_value(valid_report()).expect("serialize report");
    value["generation_context"]["schema_version"] = json!("bad version");

    let error = serde_json::from_value::<WorkReport>(value).expect_err("invalid schema version");
    assert!(!error.to_string().contains("bad version"));
}

#[test]
fn missing_spec_hash_rejected() {
    let mut value = serde_json::to_value(valid_report()).expect("serialize report");
    value["generation_context"]
        .as_object_mut()
        .expect("context object")
        .remove("spec_hash");

    let error = serde_json::from_value::<WorkReport>(value).expect_err("missing spec hash");
    assert!(!error.to_string().contains("workflow spec"));
}

#[test]
fn missing_generated_at_rejected() {
    let mut value = serde_json::to_value(valid_report()).expect("serialize report");
    value["generation_context"]
        .as_object_mut()
        .expect("context object")
        .remove("generated_at");

    serde_json::from_value::<WorkReport>(value).expect_err("missing generated_at");
}

#[test]
fn missing_generated_by_rejected() {
    let mut value = serde_json::to_value(valid_report()).expect("serialize report");
    value["generation_context"]
        .as_object_mut()
        .expect("context object")
        .remove("generated_by");

    serde_json::from_value::<WorkReport>(value).expect_err("missing generated_by");
}

#[test]
fn terminal_statuses_are_representable() {
    let statuses = [
        WorkReportStatus::Completed,
        WorkReportStatus::Failed,
        WorkReportStatus::Canceled,
        WorkReportStatus::Escalated,
        WorkReportStatus::Blocked,
    ];

    for status in statuses {
        let serialized = serde_json::to_string(&status).expect("serialize status");
        let deserialized: WorkReportStatus =
            serde_json::from_str(&serialized).expect("deserialize status");
        assert_eq!(deserialized, status);
    }
}

#[test]
fn required_v1_section_kinds_are_representable() {
    let sections = required_sections();

    assert_eq!(sections.len(), 11);
    assert!(sections
        .iter()
        .any(|section| section.kind() == WorkReportSectionKind::WorkPerformed));
    assert!(sections
        .iter()
        .any(|section| section.kind() == WorkReportSectionKind::EvidenceConsidered));
    assert!(sections
        .iter()
        .any(|section| section.kind() == WorkReportSectionKind::DecisionsMade));
    assert!(sections
        .iter()
        .any(|section| section.kind() == WorkReportSectionKind::PolicyGatesEvaluated));
    assert!(sections
        .iter()
        .any(|section| section.kind() == WorkReportSectionKind::Approvals));
    assert!(sections
        .iter()
        .any(|section| section.kind() == WorkReportSectionKind::ValidationAndQualityChecks));
    assert!(sections
        .iter()
        .any(|section| section.kind() == WorkReportSectionKind::SideEffects));
    assert!(sections
        .iter()
        .any(|section| section.kind() == WorkReportSectionKind::IncompleteOrDeferredWork));
    assert!(sections
        .iter()
        .any(|section| section.kind() == WorkReportSectionKind::KnownLimitations));
    assert!(sections
        .iter()
        .any(|section| section.kind() == WorkReportSectionKind::Risks));
    assert!(sections
        .iter()
        .any(|section| section.kind() == WorkReportSectionKind::OperatorHandoffNotes));
}

#[test]
fn duplicate_sections_rejected() {
    let mut sections = required_sections();
    sections.push(section(WorkReportSectionKind::WorkPerformed));

    let error = WorkReport::new(WorkReportDefinition {
        sections,
        ..valid_report_definition()
    })
    .expect_err("duplicate sections should fail");

    assert_eq!(error.code(), "work_report.sections.duplicate");
}

#[test]
fn domain_specific_sections_are_not_required_by_core() {
    let serialized = serde_json::to_value(valid_report()).expect("serialize report");
    let section_text = serde_json::to_string(&serialized["sections"]).expect("section json");

    assert!(!section_text.contains("pull_request"));
    assert!(!section_text.contains("jira"));
    assert!(!section_text.contains("legal_clause"));
    assert!(!section_text.contains("finance_exception"));
}

#[test]
fn evidence_reference_citation_target_validates() {
    let citation = evidence_citation();
    assert_eq!(
        citation.citation_kind(),
        WorkReportCitationKind::EvidenceReference
    );
    assert!(!citation.missing());
}

#[test]
fn adapter_telemetry_citation_target_validates() {
    let citation = WorkReportCitation::new(WorkReportCitationDefinition {
        target: WorkReportCitationTarget::AdapterTelemetry {
            reference: WorkReportStableReference::new("adapter/invocation/1")
                .expect("valid adapter ref"),
        },
        summary: Some("adapter response summary reference".to_owned()),
        missing: false,
        redaction: redaction(),
        sensitivity: WorkReportSensitivity::Confidential,
    })
    .expect("valid adapter telemetry citation");

    assert_eq!(
        citation.citation_kind(),
        WorkReportCitationKind::AdapterTelemetry
    );
}

#[test]
fn validation_diagnostic_citation_target_validates() {
    let citation = WorkReportCitation::new(WorkReportCitationDefinition {
        target: WorkReportCitationTarget::ValidationDiagnostic {
            validation_reference_id: ValidationReferenceId::new("validation/schema-version")
                .expect("valid validation ref"),
        },
        summary: None,
        missing: false,
        redaction: redaction(),
        sensitivity: WorkReportSensitivity::Confidential,
    })
    .expect("valid validation citation");

    assert_eq!(
        citation.citation_kind(),
        WorkReportCitationKind::ValidationDiagnostic
    );
}

#[test]
fn local_check_result_citation_target_validates() {
    let citation = WorkReportCitation::new(WorkReportCitationDefinition {
        target: WorkReportCitationTarget::LocalCheckResult {
            reference: WorkReportStableReference::new("local-check-result/docs/passed")
                .expect("valid local check result ref"),
        },
        summary: Some("local check result reference considered".to_owned()),
        missing: false,
        redaction: redaction(),
        sensitivity: WorkReportSensitivity::Confidential,
    })
    .expect("valid local check result citation");

    assert_eq!(
        citation.citation_kind(),
        WorkReportCitationKind::LocalCheckResult
    );
    assert!(!citation.missing());
}

#[test]
fn local_check_result_citation_target_serializes_and_deserializes() {
    let citation = WorkReportCitation::new(WorkReportCitationDefinition {
        target: WorkReportCitationTarget::LocalCheckResult {
            reference: WorkReportStableReference::new("local-check-result/docs/passed")
                .expect("valid local check result ref"),
        },
        summary: None,
        missing: false,
        redaction: redaction(),
        sensitivity: WorkReportSensitivity::Confidential,
    })
    .expect("valid local check result citation");

    let serialized = serde_json::to_string(&citation).expect("citation serializes");
    assert!(serialized.contains("\"kind\":\"local_check_result\""));
    assert!(serialized.contains("local-check-result/docs/passed"));

    let deserialized: WorkReportCitation =
        serde_json::from_str(&serialized).expect("citation deserializes");
    assert_eq!(deserialized, citation);
}

#[test]
fn local_check_result_citation_rejects_secret_like_reference_without_leaking() {
    let error = WorkReportStableReference::new("local-check-result/bearer-token-super-secret")
        .expect_err("secret-like local check reference rejected");

    assert_eq!(error.code(), "work_report_contract.secret_like_identifier");
    assert!(!error.to_string().contains("bearer-token-super-secret"));
}

#[test]
fn local_check_result_citation_debug_and_serialization_do_not_copy_raw_output() {
    let citation = WorkReportCitation::new(WorkReportCitationDefinition {
        target: WorkReportCitationTarget::LocalCheckResult {
            reference: WorkReportStableReference::new("local-check-result/docs/passed")
                .expect("valid local check result ref"),
        },
        summary: Some("local check result reference considered".to_owned()),
        missing: false,
        redaction: redaction(),
        sensitivity: WorkReportSensitivity::Confidential,
    })
    .expect("valid local check result citation");

    let debug = format!("{citation:?}");
    assert!(debug.contains("LocalCheckResult"));
    assert!(!debug.contains("local-check-result/docs/passed"));
    assert!(!debug.contains("stdout"));
    assert!(!debug.contains("stderr"));

    let serialized = serde_json::to_string(&citation).expect("citation serializes");
    assert!(!serialized.contains("raw stdout"));
    assert!(!serialized.contains("raw stderr"));
    assert!(!serialized.contains("raw command transcript"));
    assert!(!serialized.contains("bearer-token-super-secret"));
}

#[test]
fn typed_handoff_citation_target_validates() {
    let citation = WorkReportCitation::new(WorkReportCitationDefinition {
        target: WorkReportCitationTarget::TypedHandoff {
            typed_handoff_id: TypedHandoffId::new("typed-handoff/final-review")
                .expect("valid typed handoff id"),
        },
        summary: Some("typed handoff reference considered".to_owned()),
        missing: false,
        redaction: redaction(),
        sensitivity: WorkReportSensitivity::Confidential,
    })
    .expect("valid typed handoff citation");

    assert_eq!(
        citation.citation_kind(),
        WorkReportCitationKind::TypedHandoff
    );
    assert!(!citation.missing());
}

#[test]
fn typed_handoff_citation_target_serializes_and_deserializes() {
    let citation = WorkReportCitation::new(WorkReportCitationDefinition {
        target: WorkReportCitationTarget::TypedHandoff {
            typed_handoff_id: TypedHandoffId::new("typed-handoff/final-review")
                .expect("valid typed handoff id"),
        },
        summary: None,
        missing: false,
        redaction: redaction(),
        sensitivity: WorkReportSensitivity::Confidential,
    })
    .expect("valid typed handoff citation");

    let serialized = serde_json::to_string(&citation).expect("citation serializes");
    assert!(serialized.contains("\"kind\":\"typed_handoff\""));
    assert!(serialized.contains("typed-handoff/final-review"));

    let deserialized: WorkReportCitation =
        serde_json::from_str(&serialized).expect("citation deserializes");
    assert_eq!(deserialized, citation);
}

#[test]
fn typed_handoff_citation_rejects_secret_like_id_without_leaking() {
    let secret = "typed-handoff/bearer-token-super-secret";
    let error = TypedHandoffId::new(secret).expect_err("secret-like typed handoff id rejected");

    assert_eq!(error.code(), "typed_handoff.secret_like_value");
    assert!(!error.to_string().contains(secret));
}

#[test]
fn invalid_serialized_typed_handoff_citation_fails_closed_without_leaking() {
    let secret = "typed-handoff/bearer-token-super-secret";
    let value = json!({
        "target": {
            "kind": "typed_handoff",
            "typed_handoff_id": secret
        },
        "summary": null,
        "missing": false,
        "redaction": redaction(),
        "sensitivity": "confidential"
    });

    let error = serde_json::from_value::<WorkReportCitation>(value)
        .expect_err("invalid typed handoff citation fails closed");

    assert!(!error.to_string().contains(secret));
}

#[test]
fn typed_handoff_citation_debug_and_serialization_do_not_copy_handoff_payload() {
    let typed_handoff_id = "typed-handoff/final-review";
    let citation = WorkReportCitation::new(WorkReportCitationDefinition {
        target: WorkReportCitationTarget::TypedHandoff {
            typed_handoff_id: TypedHandoffId::new(typed_handoff_id)
                .expect("valid typed handoff id"),
        },
        summary: Some("typed handoff reference considered".to_owned()),
        missing: false,
        redaction: redaction(),
        sensitivity: WorkReportSensitivity::Confidential,
    })
    .expect("valid typed handoff citation");

    let debug = format!("{citation:?}");
    assert!(debug.contains("TypedHandoff"));
    assert!(!debug.contains(typed_handoff_id));
    assert!(!debug.contains("handoff obligation"));
    assert!(!debug.contains("handoff disclosure"));
    assert!(!debug.contains("handoff risk"));
    assert!(!debug.contains("operator note"));

    let serialized = serde_json::to_string(&citation).expect("citation serializes");
    assert!(serialized.contains(typed_handoff_id));
    assert!(!serialized.contains("handoff obligation"));
    assert!(!serialized.contains("handoff disclosure"));
    assert!(!serialized.contains("handoff risk"));
    assert!(!serialized.contains("operator note"));
    assert!(!serialized.contains("raw provider payload"));
    assert!(!serialized.contains("raw command output"));
    assert!(!serialized.contains("raw spec contents"));
    assert!(!serialized.contains("bearer-token-super-secret"));
}

#[test]
fn agent_harness_hook_citation_target_validates() {
    let citation = WorkReportCitation::new(WorkReportCitationDefinition {
        target: WorkReportCitationTarget::AgentHarnessHook {
            hook_invocation_id: AgentHarnessHookInvocationId::new(
                "hook-invocation/run-1/pre-validation",
            )
            .expect("valid hook invocation id"),
        },
        summary: Some("agent harness hook checkpoint reference considered".to_owned()),
        missing: false,
        redaction: redaction(),
        sensitivity: WorkReportSensitivity::Confidential,
    })
    .expect("valid agent harness hook citation");

    assert_eq!(
        citation.citation_kind(),
        WorkReportCitationKind::AgentHarnessHook
    );
    assert!(!citation.missing());
}

#[test]
fn agent_harness_hook_citation_target_serializes_and_deserializes() {
    let citation = WorkReportCitation::new(WorkReportCitationDefinition {
        target: WorkReportCitationTarget::AgentHarnessHook {
            hook_invocation_id: AgentHarnessHookInvocationId::new(
                "hook-invocation/run-1/pre-validation",
            )
            .expect("valid hook invocation id"),
        },
        summary: None,
        missing: false,
        redaction: redaction(),
        sensitivity: WorkReportSensitivity::Confidential,
    })
    .expect("valid agent harness hook citation");

    let serialized = serde_json::to_string(&citation).expect("citation serializes");
    assert!(serialized.contains("\"kind\":\"agent_harness_hook\""));
    assert!(serialized.contains("hook-invocation/run-1/pre-validation"));

    let deserialized: WorkReportCitation =
        serde_json::from_str(&serialized).expect("citation deserializes");
    assert_eq!(deserialized, citation);
}

#[test]
fn agent_harness_hook_citation_rejects_secret_like_id_without_leaking() {
    let secret = "hook-invocation/bearer-token-super-secret";
    let error = AgentHarnessHookInvocationId::new(secret)
        .expect_err("secret-like hook invocation id rejected");

    assert_eq!(
        error.code(),
        "agent_harness_hook_invocation.secret_like_value"
    );
    assert!(!error.to_string().contains(secret));
}

#[test]
fn invalid_serialized_agent_harness_hook_citation_fails_closed_without_leaking() {
    let secret = "hook-invocation/bearer-token-super-secret";
    let value = json!({
        "target": {
            "kind": "agent_harness_hook",
            "hook_invocation_id": secret
        },
        "summary": null,
        "missing": false,
        "redaction": redaction(),
        "sensitivity": "confidential"
    });

    let error = serde_json::from_value::<WorkReportCitation>(value)
        .expect_err("invalid hook citation fails closed");

    assert!(!error.to_string().contains(secret));
}

#[test]
fn agent_harness_hook_citation_debug_and_serialization_do_not_copy_hook_payload() {
    let hook_invocation_id = "hook-invocation/run-1/pre-validation";
    let citation = WorkReportCitation::new(WorkReportCitationDefinition {
        target: WorkReportCitationTarget::AgentHarnessHook {
            hook_invocation_id: AgentHarnessHookInvocationId::new(hook_invocation_id)
                .expect("valid hook invocation id"),
        },
        summary: Some("agent harness hook checkpoint reference considered".to_owned()),
        missing: false,
        redaction: redaction(),
        sensitivity: WorkReportSensitivity::Confidential,
    })
    .expect("valid agent harness hook citation");

    let debug = format!("{citation:?}");
    assert!(debug.contains("AgentHarnessHook"));
    assert!(!debug.contains(hook_invocation_id));
    assert!(!debug.contains("hook disclosure"));
    assert!(!debug.contains("hook input"));
    assert!(!debug.contains("hook output"));
    assert!(!debug.contains("operator note"));

    let serialized = serde_json::to_string(&citation).expect("citation serializes");
    assert!(serialized.contains(hook_invocation_id));
    assert!(!serialized.contains("hook disclosure"));
    assert!(!serialized.contains("hook input"));
    assert!(!serialized.contains("hook output"));
    assert!(!serialized.contains("operator note"));
    assert!(!serialized.contains("raw provider payload"));
    assert!(!serialized.contains("raw command output"));
    assert!(!serialized.contains("raw spec contents"));
    assert!(!serialized.contains("bearer-token-super-secret"));
}

#[test]
fn agent_harness_hook_disclosure_citation_target_validates() {
    let citation = WorkReportCitation::new(WorkReportCitationDefinition {
        target: WorkReportCitationTarget::AgentHarnessHookDisclosure {
            disclosure_id: AgentHarnessHookDisclosureId::new(
                "hook-disclosure/run-1/pre-validation-warning",
            )
            .expect("valid hook disclosure id"),
        },
        summary: Some("agent harness hook disclosure reference considered".to_owned()),
        missing: false,
        redaction: redaction(),
        sensitivity: WorkReportSensitivity::Confidential,
    })
    .expect("valid agent harness hook disclosure citation");

    assert_eq!(
        citation.citation_kind(),
        WorkReportCitationKind::AgentHarnessHookDisclosure
    );
    assert!(!citation.missing());
}

#[test]
fn agent_harness_hook_disclosure_citation_target_serializes_and_deserializes() {
    let citation = WorkReportCitation::new(WorkReportCitationDefinition {
        target: WorkReportCitationTarget::AgentHarnessHookDisclosure {
            disclosure_id: AgentHarnessHookDisclosureId::new(
                "hook-disclosure/run-1/pre-validation-warning",
            )
            .expect("valid hook disclosure id"),
        },
        summary: None,
        missing: false,
        redaction: redaction(),
        sensitivity: WorkReportSensitivity::Confidential,
    })
    .expect("valid agent harness hook disclosure citation");

    let serialized = serde_json::to_string(&citation).expect("citation serializes");
    assert!(serialized.contains("\"kind\":\"agent_harness_hook_disclosure\""));
    assert!(serialized.contains("hook-disclosure/run-1/pre-validation-warning"));

    let deserialized: WorkReportCitation =
        serde_json::from_str(&serialized).expect("citation deserializes");
    assert_eq!(deserialized, citation);
}

#[test]
fn agent_harness_hook_disclosure_citation_rejects_secret_like_id_without_leaking() {
    let secret = "hook-disclosure/bearer-token-super-secret";
    let error = AgentHarnessHookDisclosureId::new(secret)
        .expect_err("secret-like hook disclosure id rejected");

    assert_eq!(
        error.code(),
        "agent_harness_hook_invocation.secret_like_value"
    );
    assert!(!error.to_string().contains(secret));
}

#[test]
fn invalid_serialized_agent_harness_hook_disclosure_citation_fails_closed_without_leaking() {
    let secret = "hook-disclosure/bearer-token-super-secret";
    let value = json!({
        "target": {
            "kind": "agent_harness_hook_disclosure",
            "disclosure_id": secret
        },
        "summary": null,
        "missing": false,
        "redaction": redaction(),
        "sensitivity": "confidential"
    });

    let error = serde_json::from_value::<WorkReportCitation>(value)
        .expect_err("invalid hook disclosure citation fails closed");

    assert!(!error.to_string().contains(secret));
}

#[test]
fn agent_harness_hook_disclosure_citation_debug_and_serialization_do_not_copy_disclosure_payload() {
    let disclosure_id = "hook-disclosure/run-1/pre-validation-warning";
    let citation = WorkReportCitation::new(WorkReportCitationDefinition {
        target: WorkReportCitationTarget::AgentHarnessHookDisclosure {
            disclosure_id: AgentHarnessHookDisclosureId::new(disclosure_id)
                .expect("valid hook disclosure id"),
        },
        summary: Some("agent harness hook disclosure reference considered".to_owned()),
        missing: false,
        redaction: redaction(),
        sensitivity: WorkReportSensitivity::Confidential,
    })
    .expect("valid agent harness hook disclosure citation");

    let debug = format!("{citation:?}");
    assert!(debug.contains("AgentHarnessHookDisclosure"));
    assert!(!debug.contains(disclosure_id));
    assert!(!debug.contains("bounded checkpoint note"));
    assert!(!debug.contains("hook disclosure title"));
    assert!(!debug.contains("hook disclosure summary"));
    assert!(!debug.contains("hook input"));
    assert!(!debug.contains("hook output"));
    assert!(!debug.contains("operator note"));

    let serialized = serde_json::to_string(&citation).expect("citation serializes");
    assert!(serialized.contains(disclosure_id));
    assert!(!serialized.contains("bounded checkpoint note"));
    assert!(!serialized.contains("hook disclosure title"));
    assert!(!serialized.contains("hook disclosure summary"));
    assert!(!serialized.contains("hook input"));
    assert!(!serialized.contains("hook output"));
    assert!(!serialized.contains("operator note"));
    assert!(!serialized.contains("raw provider payload"));
    assert!(!serialized.contains("raw command output"));
    assert!(!serialized.contains("raw spec contents"));
    assert!(!serialized.contains("bearer-token-super-secret"));
}

#[test]
fn side_effect_citation_target_validates() {
    let citation = WorkReportCitation::new(WorkReportCitationDefinition {
        target: WorkReportCitationTarget::SideEffect {
            side_effect_id: SideEffectId::new("side-effect/run-1/proposed-write")
                .expect("valid side-effect id"),
        },
        summary: Some("side-effect record reference considered".to_owned()),
        missing: false,
        redaction: redaction(),
        sensitivity: WorkReportSensitivity::Confidential,
    })
    .expect("valid side-effect citation");

    assert_eq!(citation.citation_kind(), WorkReportCitationKind::SideEffect);
    assert!(!citation.missing());
}

#[test]
fn side_effect_citation_target_serializes_and_deserializes() {
    let citation = WorkReportCitation::new(WorkReportCitationDefinition {
        target: WorkReportCitationTarget::SideEffect {
            side_effect_id: SideEffectId::new("side-effect/run-1/proposed-write")
                .expect("valid side-effect id"),
        },
        summary: None,
        missing: false,
        redaction: redaction(),
        sensitivity: WorkReportSensitivity::Confidential,
    })
    .expect("valid side-effect citation");

    let serialized = serde_json::to_string(&citation).expect("citation serializes");
    assert!(serialized.contains("\"kind\":\"side_effect\""));
    assert!(serialized.contains("side-effect/run-1/proposed-write"));

    let deserialized: WorkReportCitation =
        serde_json::from_str(&serialized).expect("citation deserializes");
    assert_eq!(deserialized, citation);
}

#[test]
fn side_effect_citation_rejects_secret_like_id_without_leaking() {
    let secret = "side-effect/bearer-token-super-secret";
    let error = SideEffectId::new(secret).expect_err("secret-like side-effect id rejected");

    assert_eq!(error.code(), "side_effect.secret_like_value");
    assert!(!error.to_string().contains(secret));
}

#[test]
fn invalid_serialized_side_effect_citation_fails_closed_without_leaking() {
    let secret = "side-effect/bearer-token-super-secret";
    let value = json!({
        "target": {
            "kind": "side_effect",
            "side_effect_id": secret
        },
        "summary": null,
        "missing": false,
        "redaction": redaction(),
        "sensitivity": "confidential"
    });

    let error = serde_json::from_value::<WorkReportCitation>(value)
        .expect_err("invalid side-effect citation fails closed");

    assert!(!error.to_string().contains(secret));
}

#[test]
fn side_effect_citation_debug_and_serialization_do_not_copy_side_effect_payload() {
    let side_effect_id = "side-effect/run-1/proposed-write";
    let citation = WorkReportCitation::new(WorkReportCitationDefinition {
        target: WorkReportCitationTarget::SideEffect {
            side_effect_id: SideEffectId::new(side_effect_id).expect("valid side-effect id"),
        },
        summary: Some("side-effect record reference considered".to_owned()),
        missing: false,
        redaction: redaction(),
        sensitivity: WorkReportSensitivity::Confidential,
    })
    .expect("valid side-effect citation");

    let debug = format!("{citation:?}");
    assert!(debug.contains("SideEffect"));
    assert!(!debug.contains(side_effect_id));
    assert!(!debug.contains("target reference"));
    assert!(!debug.contains("outcome reference"));
    assert!(!debug.contains("reason code"));
    assert!(!debug.contains("side-effect summary"));

    let serialized = serde_json::to_string(&citation).expect("citation serializes");
    assert!(serialized.contains(side_effect_id));
    assert!(!serialized.contains("target reference"));
    assert!(!serialized.contains("outcome reference"));
    assert!(!serialized.contains("reason code"));
    assert!(!serialized.contains("side-effect summary"));
    assert!(!serialized.contains("raw provider payload"));
    assert!(!serialized.contains("raw command output"));
    assert!(!serialized.contains("raw spec contents"));
    assert!(!serialized.contains("bearer-token-super-secret"));
}

#[test]
fn side_effects_section_can_hold_side_effect_citation_without_write_support() {
    let citation = WorkReportCitation::new(WorkReportCitationDefinition {
        target: WorkReportCitationTarget::SideEffect {
            side_effect_id: SideEffectId::new("side-effect/run-1/denied-write")
                .expect("valid side-effect id"),
        },
        summary: None,
        missing: false,
        redaction: redaction(),
        sensitivity: WorkReportSensitivity::Confidential,
    })
    .expect("valid side-effect citation");

    let section = WorkReportSection::new(
        WorkReportSectionKind::SideEffects,
        Some("Side-effect records cited by stable ID only.".to_owned()),
        vec![citation],
    )
    .expect("side effects section with side-effect citation");

    assert_eq!(section.kind(), WorkReportSectionKind::SideEffects);
    assert_eq!(section.citations().len(), 1);
    assert_eq!(
        section.citations()[0].citation_kind(),
        WorkReportCitationKind::SideEffect
    );
}

#[test]
fn approval_and_policy_citation_vocabulary_is_representable_without_attachment() {
    let approval = WorkReportCitationTarget::ApprovalDecision {
        approval_reference_id: ApprovalReferenceId::new("approval-1").expect("valid approval ref"),
    };
    let policy = WorkReportCitationTarget::PolicyDecision {
        event_id: EventId::new("event-policy-1").expect("valid event id"),
    };

    assert_eq!(
        approval.citation_kind(),
        WorkReportCitationKind::ApprovalDecision
    );
    assert_eq!(
        policy.citation_kind(),
        WorkReportCitationKind::PolicyDecision
    );
}

#[test]
fn future_reasoning_lineage_citation_vocabulary_does_not_implement_lineage() {
    let target = WorkReportCitationTarget::ReasoningLineageNode {
        reference: WorkReportStableReference::new("lineage/future-node")
            .expect("valid future lineage ref"),
    };

    assert_eq!(
        target.citation_kind(),
        WorkReportCitationKind::ReasoningLineageNode
    );
}

#[test]
fn incomplete_deferred_disclosure_validates() {
    let disclosure =
        WorkReportIncompleteWorkDisclosure::new("none deferred", vec![evidence_citation()])
            .expect("valid disclosure");

    assert_eq!(disclosure.summary(), "none deferred");
    assert_eq!(disclosure.citations().len(), 1);
}

#[test]
fn known_limitation_validates() {
    let limitation = WorkReportKnownLimitation::new("local only", vec![evidence_citation()])
        .expect("valid limitation");

    assert_eq!(limitation.summary(), "local only");
}

#[test]
fn risk_validates() {
    let risk = WorkReportRisk::new("operator review required", vec![evidence_citation()])
        .expect("valid risk");

    assert_eq!(risk.summary(), "operator review required");
}

#[test]
fn side_effect_section_works_without_write_support() {
    let report = valid_report();

    assert!(report
        .sections()
        .iter()
        .any(|section| section.kind() == WorkReportSectionKind::SideEffects));
}

#[test]
fn serde_round_trip_for_valid_report() {
    let report = valid_report();
    let serialized = serde_json::to_string(&report).expect("serialize report");
    let deserialized: WorkReport = serde_json::from_str(&serialized).expect("deserialize report");

    assert_eq!(deserialized, report);
}

#[test]
fn work_report_artifact_record_binds_report_and_run_identity() {
    let artifact = artifact_record();

    assert_eq!(artifact.report_id(), valid_report().report_id());
    assert_eq!(artifact.run_id(), &run_id());
    assert_eq!(artifact.metadata().run_id(), &run_id());
    assert_eq!(
        artifact.metadata().terminal_run_status(),
        WorkReportStatus::Completed
    );
    artifact.validate().expect("artifact validates");
}

#[test]
fn work_report_artifact_record_serializes_and_deserializes() {
    let artifact = artifact_record();
    let serialized = serde_json::to_string(&artifact).expect("serialize artifact");
    let deserialized: WorkReportArtifactRecord =
        serde_json::from_str(&serialized).expect("deserialize artifact");

    assert_eq!(deserialized, artifact);
    assert_eq!(deserialized.work_report(), artifact.work_report());
}

#[test]
fn work_report_artifact_record_rejects_identity_mismatch() {
    let mut value = serde_json::to_value(artifact_record()).expect("serialize artifact");
    value["metadata"]["run_id"] = json!("run-other");

    let error = serde_json::from_value::<WorkReportArtifactRecord>(value)
        .expect_err("mismatched artifact metadata rejected");

    assert!(error
        .to_string()
        .contains("work_report_artifact.identity.mismatch"));
    assert!(!error.to_string().contains("run-other"));
}

#[test]
fn work_report_artifact_debug_does_not_leak_report_text_or_redaction_values() {
    let redaction = redaction_with("summary", "reference only bounded summary");
    let artifact = WorkReportArtifactRecord::new(
        WorkReport::new(WorkReportDefinition {
            redaction,
            ..valid_report_definition()
        })
        .expect("valid report with redaction"),
    )
    .expect("valid artifact");
    let debug = format!("{artifact:?}");

    assert!(!debug.contains("bounded section summary"));
    assert!(!debug.contains("operator should review citations"));
    assert!(!debug.contains("reference only bounded summary"));
    assert!(!debug.contains("workflow/intake"));
    assert!(!debug.contains("run-123"));
    assert!(debug.contains("work_report"));
    assert!(debug.contains("[REDACTED]"));
}

#[test]
fn local_backend_writes_reads_and_lists_work_report_artifacts() {
    let backend = temp_state_backend("write-read-list");
    let artifact = artifact_record();

    backend
        .write_work_report_artifact(&artifact)
        .expect("artifact written");
    let read = backend
        .read_work_report_artifact(artifact.run_id(), artifact.report_id())
        .expect("artifact read")
        .expect("artifact exists");
    let listed = backend
        .list_work_report_artifacts(artifact.run_id())
        .expect("artifacts listed");

    assert_eq!(read, artifact);
    assert_eq!(listed, vec![artifact]);
}

#[test]
fn local_backend_rejects_duplicate_work_report_artifact_write() {
    let backend = temp_state_backend("duplicate");
    let artifact = artifact_record();
    backend
        .write_work_report_artifact(&artifact)
        .expect("artifact written");

    let error = backend
        .write_work_report_artifact(&artifact)
        .expect_err("duplicate artifact rejected");

    assert_eq!(error.code(), "work_report_artifact.write.duplicate");
    assert!(!error.to_string().contains("report/local-run"));
    assert!(!error.to_string().contains("run-123"));
}

#[test]
fn side_effect_integrity_succeeds_with_no_side_effect_citations() {
    let backend = temp_state_backend("side-effect-integrity-none");
    let artifact = artifact_record();

    let result = validate_work_report_artifact_side_effect_integrity(
        &backend,
        WorkReportArtifactSideEffectIntegrityInput {
            artifact: &artifact,
            require_all_side_effect_citations: true,
        },
    )
    .expect("artifact without side-effect citations validates");

    assert_eq!(result.cited_side_effect_count(), 0);
    assert_eq!(result.resolved_side_effect_count(), 0);
    assert_eq!(result.missing_side_effect_count(), 0);
    assert_eq!(result.duplicate_side_effect_citation_count(), 0);
}

#[test]
fn side_effect_integrity_succeeds_for_matching_record() {
    let side_effect_id =
        SideEffectId::new("side-effect/run-123/proposed-write").expect("valid side-effect id");
    let (run, artifact) = artifact_with_side_effect_ids(vec![side_effect_id.clone()]);
    let backend = temp_state_backend("side-effect-integrity-matching");
    let record = side_effect_record_for_run(&run, side_effect_id);
    backend
        .write_side_effect_record(&record)
        .expect("side-effect record written");

    let result = validate_work_report_artifact_side_effect_integrity(
        &backend,
        WorkReportArtifactSideEffectIntegrityInput {
            artifact: &artifact,
            require_all_side_effect_citations: true,
        },
    )
    .expect("matching side-effect record validates");

    assert_eq!(result.cited_side_effect_count(), 1);
    assert_eq!(result.resolved_side_effect_count(), 1);
    assert_eq!(result.missing_side_effect_count(), 0);
    assert_eq!(result.duplicate_side_effect_citation_count(), 0);
}

#[test]
fn side_effect_integrity_deduplicates_citations_deterministically() {
    let side_effect_id =
        SideEffectId::new("side-effect/run-123/proposed-write").expect("valid side-effect id");
    let (run, artifact) =
        artifact_with_side_effect_ids(vec![side_effect_id.clone(), side_effect_id.clone()]);
    let backend = temp_state_backend("side-effect-integrity-duplicates");
    let record = side_effect_record_for_run(&run, side_effect_id);
    backend
        .write_side_effect_record(&record)
        .expect("side-effect record written");

    let result = validate_work_report_artifact_side_effect_integrity(
        &backend,
        WorkReportArtifactSideEffectIntegrityInput {
            artifact: &artifact,
            require_all_side_effect_citations: true,
        },
    )
    .expect("duplicate side-effect citations validate once");

    assert_eq!(result.cited_side_effect_count(), 1);
    assert_eq!(result.resolved_side_effect_count(), 1);
    assert_eq!(result.missing_side_effect_count(), 0);
    assert_eq!(result.duplicate_side_effect_citation_count(), 1);
}

#[test]
fn side_effect_integrity_missing_record_fails_in_strict_mode_without_leaking() {
    let side_effect_id =
        SideEffectId::new("side-effect/run-123/missing-record").expect("valid side-effect id");
    let (_run, artifact) = artifact_with_side_effect_ids(vec![side_effect_id]);
    let backend = temp_state_backend("side-effect-integrity-strict-missing");

    let error = validate_work_report_artifact_side_effect_integrity(
        &backend,
        WorkReportArtifactSideEffectIntegrityInput {
            artifact: &artifact,
            require_all_side_effect_citations: true,
        },
    )
    .expect_err("strict missing side-effect record rejected");

    assert_eq!(
        error.code(),
        "work_report_artifact.side_effect_integrity.record_missing"
    );
    assert!(!error.to_string().contains("missing-record"));
    assert!(!error.to_string().contains("run-123"));
}

#[test]
fn side_effect_integrity_missing_record_is_counted_in_permissive_mode() {
    let side_effect_id =
        SideEffectId::new("side-effect/run-123/missing-record").expect("valid side-effect id");
    let (_run, artifact) = artifact_with_side_effect_ids(vec![side_effect_id]);
    let backend = temp_state_backend("side-effect-integrity-permissive-missing");

    let result = validate_work_report_artifact_side_effect_integrity(
        &backend,
        WorkReportArtifactSideEffectIntegrityInput {
            artifact: &artifact,
            require_all_side_effect_citations: false,
        },
    )
    .expect("permissive missing side-effect record counted");

    assert_eq!(result.cited_side_effect_count(), 1);
    assert_eq!(result.resolved_side_effect_count(), 0);
    assert_eq!(result.missing_side_effect_count(), 1);
    assert_eq!(result.duplicate_side_effect_citation_count(), 0);
}

#[test]
fn side_effect_integrity_identity_mismatch_fails_without_leaking() {
    let side_effect_id =
        SideEffectId::new("side-effect/run-123/mismatched-record").expect("valid side-effect id");
    let (run, artifact) = artifact_with_side_effect_ids(vec![side_effect_id.clone()]);
    let backend = temp_state_backend("side-effect-integrity-mismatch");
    let record = side_effect_record_with_mismatched_workflow_id(&run, side_effect_id);
    backend
        .write_side_effect_record(&record)
        .expect("mismatched side-effect record written");

    let error = validate_work_report_artifact_side_effect_integrity(
        &backend,
        WorkReportArtifactSideEffectIntegrityInput {
            artifact: &artifact,
            require_all_side_effect_citations: false,
        },
    )
    .expect_err("identity mismatch rejected");

    assert_eq!(
        error.code(),
        "work_report_artifact.side_effect_integrity.identity_mismatch"
    );
    assert!(!error.to_string().contains("mismatched-record"));
    assert!(!error.to_string().contains("workflow/other"));
    assert!(!error.to_string().contains("workflow/intake"));
    assert!(!error.to_string().contains("run-123"));
}

#[test]
fn side_effect_integrity_store_errors_are_mapped_without_leaking() {
    let side_effect_id =
        SideEffectId::new("side-effect/run-123/store-failure").expect("valid side-effect id");
    let (_run, artifact) = artifact_with_side_effect_ids(vec![side_effect_id]);

    let corrupt_error = validate_work_report_artifact_side_effect_integrity(
        &FailingSideEffectRecordStore {
            code: "side_effect_record.read.corrupt",
        },
        WorkReportArtifactSideEffectIntegrityInput {
            artifact: &artifact,
            require_all_side_effect_citations: true,
        },
    )
    .expect_err("corrupt store read rejected");
    assert_eq!(
        corrupt_error.code(),
        "work_report_artifact.side_effect_integrity.record_corrupt"
    );
    assert!(!corrupt_error.to_string().contains("store-failure"));

    let read_error = validate_work_report_artifact_side_effect_integrity(
        &FailingSideEffectRecordStore {
            code: "state.local.read_failed",
        },
        WorkReportArtifactSideEffectIntegrityInput {
            artifact: &artifact,
            require_all_side_effect_citations: true,
        },
    )
    .expect_err("store read failure rejected");
    assert_eq!(
        read_error.code(),
        "work_report_artifact.side_effect_integrity.store_read_failed"
    );
    assert!(!read_error.to_string().contains("store-failure"));
}

#[test]
fn side_effect_integrity_debug_output_is_bounded_and_non_leaking() {
    let side_effect_id =
        SideEffectId::new("side-effect/run-123/debug-record").expect("valid side-effect id");
    let (run, artifact) = artifact_with_side_effect_ids(vec![side_effect_id.clone()]);
    let backend = temp_state_backend("side-effect-integrity-debug");
    let record = side_effect_record_for_run(&run, side_effect_id);
    backend
        .write_side_effect_record(&record)
        .expect("side-effect record written");

    let input = WorkReportArtifactSideEffectIntegrityInput {
        artifact: &artifact,
        require_all_side_effect_citations: true,
    };
    let result =
        validate_work_report_artifact_side_effect_integrity(&backend, input).expect("valid result");

    let input_debug = format!(
        "{:?}",
        WorkReportArtifactSideEffectIntegrityInput {
            artifact: &artifact,
            require_all_side_effect_citations: true,
        }
    );
    let result_debug = format!("{result:?}");

    assert!(!input_debug.contains("debug-record"));
    assert!(!input_debug.contains("report/local-run"));
    assert!(!input_debug.contains("run-123"));
    assert!(input_debug.contains("[REDACTED]"));
    assert!(!result_debug.contains("debug-record"));
    assert!(!result_debug.contains("report/local-run"));
    assert!(!result_debug.contains("run-123"));
    assert!(result_debug.contains("cited_side_effect_count"));
}

#[test]
fn local_backend_corrupt_artifact_read_fails_without_leaking_payload() {
    let backend = temp_state_backend("corrupt");
    let artifact = artifact_record();
    backend
        .write_work_report_artifact(&artifact)
        .expect("artifact written");
    let path = artifact_path(&backend, &artifact);
    fs::write(&path, r#"{"secret":"sk-artifact-secret"}"#).expect("corrupt artifact written");

    let error = backend
        .read_work_report_artifact(artifact.run_id(), artifact.report_id())
        .expect_err("corrupt artifact rejected");

    assert_eq!(error.code(), "work_report_artifact.read.corrupt");
    assert!(!error.to_string().contains("sk-artifact-secret"));
    assert!(!error.to_string().contains("report/local-run"));
    assert!(!error.to_string().contains("run-123"));
}

#[test]
fn work_report_artifact_write_does_not_mutate_runtime_state() {
    let backend = temp_state_backend("no-runtime-mutation");
    let artifact = artifact_record();
    let events_before = backend
        .read_events(artifact.run_id())
        .expect("events before");
    let snapshot_before = backend
        .load_snapshot(artifact.run_id())
        .expect("snapshot before");

    backend
        .write_work_report_artifact(&artifact)
        .expect("artifact written");

    let events_after = backend
        .read_events(artifact.run_id())
        .expect("events after");
    let snapshot_after = backend
        .load_snapshot(artifact.run_id())
        .expect("snapshot after");
    assert_eq!(events_before, events_after);
    assert_eq!(snapshot_before, snapshot_after);
}

#[test]
fn invalid_serialized_report_fails_closed() {
    let mut value = serde_json::to_value(valid_report()).expect("serialize report");
    value["sections"] = json!([
        {"kind": "work_performed", "summary": "one", "citations": []},
        {"kind": "work_performed", "summary": "two", "citations": []}
    ]);

    let error =
        serde_json::from_value::<WorkReport>(value).expect_err("duplicate sections fail closed");
    assert!(error.to_string().contains("work_report.sections.duplicate"));
}

#[test]
fn debug_output_does_not_leak_secret_like_values() {
    let secret = "token-should-not-appear";
    let error = WorkReportSection::new(
        WorkReportSectionKind::WorkPerformed,
        Some(secret.to_owned()),
        Vec::new(),
    )
    .expect_err("secret-like summary rejected");
    assert!(!error.to_string().contains(secret));

    let report = valid_report();
    let debug = format!("{report:?}");
    assert!(!debug.contains("operator should review citations"));
    assert!(!debug.contains("bounded section summary"));
    assert!(!debug.contains("workflow/intake"));
    assert!(!debug.contains("token"));
    assert!(!debug.contains("authorization"));
    assert!(!debug.contains("private"));
}

#[test]
fn serialization_does_not_leak_forbidden_raw_payload_fields() {
    let serialized = serde_json::to_string(&valid_report()).expect("serialize report");

    assert!(!serialized.contains("provider_payload"));
    assert!(!serialized.contains("authorization"));
    assert!(!serialized.contains("private_key"));
    assert!(!serialized.contains("jira_description"));
    assert!(!serialized.contains("jira_comment"));
    assert!(!serialized.contains("ci_log"));
    assert!(!serialized.contains("github_file_contents"));
    assert!(!serialized.contains("command_output"));
    assert!(!serialized.contains("env_value"));
    assert!(!serialized.contains("credential"));
}

#[test]
fn work_report_rejects_secret_like_redaction_metadata_field_names() {
    let secret = "api_token";
    let error = WorkReport::new(WorkReportDefinition {
        redaction: redaction_with(secret, "bounded non sensitive reason"),
        ..valid_report_definition()
    })
    .expect_err("secret-like redaction field rejected");

    assert_eq!(error.code(), "work_report_contract.secret_like_identifier");
    assert!(!error.to_string().contains(secret));
}

#[test]
fn work_report_rejects_secret_like_redaction_metadata_reasons() {
    let secret = "authorization bearer value";
    let error = WorkReport::new(WorkReportDefinition {
        redaction: redaction_with("summary", secret),
        ..valid_report_definition()
    })
    .expect_err("secret-like redaction reason rejected");

    assert_eq!(error.code(), "work_report_contract.secret_like_identifier");
    assert!(!error.to_string().contains(secret));
}

#[test]
fn work_report_citation_rejects_secret_like_redaction_metadata_field_names() {
    let secret = "private_key";
    let error = WorkReportCitation::new(WorkReportCitationDefinition {
        target: WorkReportCitationTarget::EvidenceReference {
            evidence_reference_id: EvidenceReferenceId::new("evidence-1").expect("valid evidence"),
        },
        summary: None,
        missing: false,
        redaction: redaction_with(secret, "bounded non sensitive reason"),
        sensitivity: WorkReportSensitivity::Confidential,
    })
    .expect_err("secret-like citation redaction field rejected");

    assert_eq!(error.code(), "work_report_contract.secret_like_identifier");
    assert!(!error.to_string().contains(secret));
}

#[test]
fn work_report_citation_rejects_secret_like_redaction_metadata_reasons() {
    let secret = "token should not be stored";
    let error = WorkReportCitation::new(WorkReportCitationDefinition {
        target: WorkReportCitationTarget::EvidenceReference {
            evidence_reference_id: EvidenceReferenceId::new("evidence-1").expect("valid evidence"),
        },
        summary: None,
        missing: false,
        redaction: redaction_with("summary", secret),
        sensitivity: WorkReportSensitivity::Confidential,
    })
    .expect_err("secret-like citation redaction reason rejected");

    assert_eq!(error.code(), "work_report_contract.secret_like_identifier");
    assert!(!error.to_string().contains(secret));
}

#[test]
fn work_report_debug_does_not_leak_redaction_metadata_values() {
    let field = "provider_response_summary";
    let reason = "reference only bounded summary";
    let report = WorkReport::new(WorkReportDefinition {
        redaction: redaction_with(field, reason),
        ..valid_report_definition()
    })
    .expect("valid redaction metadata");

    let debug = format!("{report:?}");
    assert!(!debug.contains(field));
    assert!(!debug.contains(reason));
    assert!(debug.contains("redacted_field_count"));
    assert!(debug.contains("field_state_count"));
}

#[test]
fn work_report_citation_debug_does_not_leak_redaction_metadata_values() {
    let field = "provider_response_summary";
    let reason = "reference only bounded summary";
    let citation = WorkReportCitation::new(WorkReportCitationDefinition {
        target: WorkReportCitationTarget::EvidenceReference {
            evidence_reference_id: EvidenceReferenceId::new("evidence-1").expect("valid evidence"),
        },
        summary: None,
        missing: false,
        redaction: redaction_with(field, reason),
        sensitivity: WorkReportSensitivity::Confidential,
    })
    .expect("valid redaction metadata");

    let debug = format!("{citation:?}");
    assert!(!debug.contains(field));
    assert!(!debug.contains(reason));
    assert!(debug.contains("redacted_field_count"));
    assert!(debug.contains("field_state_count"));
}

#[test]
fn serialized_work_report_does_not_silently_carry_secret_like_redaction_metadata() {
    let secret = "api_token";
    let result = WorkReport::new(WorkReportDefinition {
        redaction: redaction_with(secret, "bounded non sensitive reason"),
        ..valid_report_definition()
    });

    assert!(result.is_err());

    let mut value = serde_json::to_value(valid_report()).expect("serialize report");
    value["redaction"] = json!({
        "redacted_fields": [secret],
        "field_states": [{
            "field": secret,
            "disposition": "redacted",
            "reason": "bounded non sensitive reason"
        }]
    });

    let error = serde_json::from_value::<WorkReport>(value)
        .expect_err("serialized secret-like redaction metadata fails closed");
    assert!(!error.to_string().contains(secret));
}

#[test]
fn invalid_serialized_redaction_metadata_reason_fails_without_leaking_value() {
    let secret = "authorization bearer secret";
    let mut value = serde_json::to_value(valid_report()).expect("serialize report");
    value["redaction"] = json!({
        "redacted_fields": ["summary"],
        "field_states": [{
            "field": "summary",
            "disposition": "redacted",
            "reason": secret
        }]
    });

    let error = serde_json::from_value::<WorkReport>(value)
        .expect_err("serialized secret-like redaction reason fails closed");
    assert!(!error.to_string().contains(secret));
}

#[test]
fn valid_redaction_metadata_still_works_for_report_and_citation() {
    let redaction = redaction_with("summary", "reference only bounded summary");
    let report = WorkReport::new(WorkReportDefinition {
        redaction: redaction.clone(),
        ..valid_report_definition()
    })
    .expect("valid report redaction metadata");
    let citation = WorkReportCitation::new(WorkReportCitationDefinition {
        target: WorkReportCitationTarget::EvidenceReference {
            evidence_reference_id: EvidenceReferenceId::new("evidence-1").expect("valid evidence"),
        },
        summary: None,
        missing: false,
        redaction,
        sensitivity: WorkReportSensitivity::Confidential,
    })
    .expect("valid citation redaction metadata");

    let report_json = serde_json::to_string(&report).expect("serialize report");
    let citation_json = serde_json::to_string(&citation).expect("serialize citation");
    assert!(report_json.contains("reference only bounded summary"));
    assert!(citation_json.contains("reference only bounded summary"));
}

#[test]
fn completed_terminal_run_input_produces_valid_in_memory_report() {
    let report = generated_report_for(WorkflowRunStatus::Completed);

    assert_eq!(
        report.generation_context().terminal_run_status,
        WorkReportStatus::Completed
    );
    report.validate().expect("generated report validates");
}

#[test]
fn failed_terminal_run_input_produces_valid_in_memory_report() {
    let report = generated_report_for(WorkflowRunStatus::Failed);

    assert_eq!(
        report.generation_context().terminal_run_status,
        WorkReportStatus::Failed
    );
    report.validate().expect("generated report validates");
}

#[test]
fn canceled_terminal_run_input_produces_valid_in_memory_report() {
    let report = generated_report_for(WorkflowRunStatus::Canceled);

    assert_eq!(
        report.generation_context().terminal_run_status,
        WorkReportStatus::Canceled
    );
    report.validate().expect("generated report validates");
}

#[test]
fn non_terminal_runtime_status_is_rejected() {
    let run = running_run();
    let error = generate_terminal_local_work_report(terminal_generation_input(&run))
        .expect_err("running status rejected");

    assert_eq!(error.code(), "work_report_generation.status.not_terminal");
    assert!(!error.to_string().contains("Running"));
}

#[test]
fn generated_report_contains_all_required_v1_sections() {
    let report = generated_report_for(WorkflowRunStatus::Completed);
    let kinds: Vec<_> = report
        .sections()
        .iter()
        .map(WorkReportSection::kind)
        .collect();

    for required_kind in WorkReportSectionKind::v1_required_kinds() {
        assert!(kinds.contains(&required_kind));
    }
}

#[test]
fn generated_report_cites_evidence_ids_without_recreating_evidence() {
    let report = generated_report_for(WorkflowRunStatus::Completed);
    let evidence_section = report
        .sections()
        .iter()
        .find(|section| section.kind() == WorkReportSectionKind::EvidenceConsidered)
        .expect("evidence section");

    assert!(evidence_section.citations().iter().any(|citation| {
        matches!(
            citation.target(),
            WorkReportCitationTarget::EvidenceReference {
                evidence_reference_id
            } if evidence_reference_id.as_str() == "evidence-1"
        )
    }));
    let serialized = serde_json::to_string(&report).expect("serialize report");
    assert!(serialized.contains("\"evidence_reference_id\":\"evidence-1\""));
    assert!(!serialized.contains("\"EvidenceReference\""));
}

#[test]
fn generated_report_cites_validation_diagnostics_by_stable_reference() {
    let report = generated_report_for(WorkflowRunStatus::Completed);
    let validation_section = report
        .sections()
        .iter()
        .find(|section| section.kind() == WorkReportSectionKind::ValidationAndQualityChecks)
        .expect("validation section");

    assert!(validation_section.citations().iter().any(|citation| {
        matches!(
            citation.target(),
            WorkReportCitationTarget::ValidationDiagnostic {
                validation_reference_id
            } if validation_reference_id.as_str() == "validation/schema-version"
        )
    }));
}

#[test]
fn generated_report_cites_local_check_results_by_stable_reference() {
    let report = generated_report_for(WorkflowRunStatus::Completed);
    let validation_section = report
        .sections()
        .iter()
        .find(|section| section.kind() == WorkReportSectionKind::ValidationAndQualityChecks)
        .expect("validation and quality section");

    assert!(validation_section.citations().iter().any(|citation| {
        matches!(
            citation.target(),
            WorkReportCitationTarget::LocalCheckResult { reference }
                if reference.as_str() == "local-check-result/docs/passed"
        ) && citation.citation_kind() == WorkReportCitationKind::LocalCheckResult
    }));
    assert_eq!(
        validation_section.summary(),
        Some("Validation diagnostic and local check result references were supplied.")
    );

    let serialized = serde_json::to_string(&report).expect("serialize report");
    assert!(serialized.contains("\"kind\":\"local_check_result\""));
    assert!(serialized.contains("local-check-result/docs/passed"));
    assert!(!serialized.contains("raw stdout"));
    assert!(!serialized.contains("raw stderr"));
    assert!(!serialized.contains("raw command transcript"));
}

#[test]
fn generated_report_preserves_validation_diagnostics_with_local_check_citations() {
    let report = generated_report_for(WorkflowRunStatus::Completed);
    let validation_section = report
        .sections()
        .iter()
        .find(|section| section.kind() == WorkReportSectionKind::ValidationAndQualityChecks)
        .expect("validation and quality section");

    let kinds: Vec<_> = validation_section
        .citations()
        .iter()
        .map(WorkReportCitation::citation_kind)
        .collect();

    assert!(kinds.contains(&WorkReportCitationKind::ValidationDiagnostic));
    assert!(kinds.contains(&WorkReportCitationKind::LocalCheckResult));
}

#[test]
fn generated_report_cites_agent_harness_hooks_by_stable_reference() {
    let run = terminal_run(WorkflowRunStatus::Completed);
    let report = generate_terminal_local_work_report(TerminalLocalWorkReportInput {
        agent_harness_hook_invocation_ids: vec![AgentHarnessHookInvocationId::new(
            "hook-invocation/run-1/pre-validation",
        )
        .expect("valid hook invocation id")],
        ..terminal_generation_input(&run)
    })
    .expect("report with hook citations");

    let validation_section = report
        .sections()
        .iter()
        .find(|section| section.kind() == WorkReportSectionKind::ValidationAndQualityChecks)
        .expect("validation and quality section");

    assert_eq!(
        validation_section.summary(),
        Some("Validation diagnostic, local check result, and agent harness hook references were supplied.")
    );
    assert!(validation_section.citations().iter().any(|citation| {
        matches!(
            citation.target(),
            WorkReportCitationTarget::AgentHarnessHook { hook_invocation_id }
                if hook_invocation_id.as_str() == "hook-invocation/run-1/pre-validation"
        ) && citation.citation_kind() == WorkReportCitationKind::AgentHarnessHook
    }));
}

#[test]
fn generated_report_hook_citation_preserves_validation_and_local_check_citations() {
    let run = terminal_run(WorkflowRunStatus::Completed);
    let report = generate_terminal_local_work_report(TerminalLocalWorkReportInput {
        agent_harness_hook_invocation_ids: vec![AgentHarnessHookInvocationId::new(
            "hook-invocation/run-1/pre-validation",
        )
        .expect("valid hook invocation id")],
        ..terminal_generation_input(&run)
    })
    .expect("report with hook citations");

    let validation_section = report
        .sections()
        .iter()
        .find(|section| section.kind() == WorkReportSectionKind::ValidationAndQualityChecks)
        .expect("validation and quality section");
    let kinds: Vec<_> = validation_section
        .citations()
        .iter()
        .map(WorkReportCitation::citation_kind)
        .collect();

    assert!(kinds.contains(&WorkReportCitationKind::ValidationDiagnostic));
    assert!(kinds.contains(&WorkReportCitationKind::LocalCheckResult));
    assert!(kinds.contains(&WorkReportCitationKind::AgentHarnessHook));
}

#[test]
fn generated_report_cites_agent_harness_hook_disclosures_by_stable_reference() {
    let run = terminal_run(WorkflowRunStatus::Completed);
    let report = generate_terminal_local_work_report(TerminalLocalWorkReportInput {
        agent_harness_hook_disclosure_ids: vec![AgentHarnessHookDisclosureId::new(
            "hook-disclosure/run-1/pre-validation-warning",
        )
        .expect("valid hook disclosure id")],
        ..terminal_generation_input(&run)
    })
    .expect("report with hook disclosure citations");

    let validation_section = report
        .sections()
        .iter()
        .find(|section| section.kind() == WorkReportSectionKind::ValidationAndQualityChecks)
        .expect("validation and quality section");

    assert_eq!(
        validation_section.summary(),
        Some("Validation diagnostic, local check result, and agent harness hook disclosure references were supplied.")
    );
    assert!(validation_section.citations().iter().any(|citation| {
        matches!(
            citation.target(),
            WorkReportCitationTarget::AgentHarnessHookDisclosure { disclosure_id }
                if disclosure_id.as_str() == "hook-disclosure/run-1/pre-validation-warning"
        ) && citation.citation_kind() == WorkReportCitationKind::AgentHarnessHookDisclosure
    }));
}

#[test]
fn generated_report_hook_disclosure_citation_preserves_validation_local_check_and_hook_citations() {
    let run = terminal_run(WorkflowRunStatus::Completed);
    let report = generate_terminal_local_work_report(TerminalLocalWorkReportInput {
        agent_harness_hook_invocation_ids: vec![AgentHarnessHookInvocationId::new(
            "hook-invocation/run-1/pre-validation",
        )
        .expect("valid hook invocation id")],
        agent_harness_hook_disclosure_ids: vec![AgentHarnessHookDisclosureId::new(
            "hook-disclosure/run-1/pre-validation-warning",
        )
        .expect("valid hook disclosure id")],
        ..terminal_generation_input(&run)
    })
    .expect("report with hook and disclosure citations");

    let validation_section = report
        .sections()
        .iter()
        .find(|section| section.kind() == WorkReportSectionKind::ValidationAndQualityChecks)
        .expect("validation and quality section");
    let kinds: Vec<_> = validation_section
        .citations()
        .iter()
        .map(WorkReportCitation::citation_kind)
        .collect();

    assert!(kinds.contains(&WorkReportCitationKind::ValidationDiagnostic));
    assert!(kinds.contains(&WorkReportCitationKind::LocalCheckResult));
    assert!(kinds.contains(&WorkReportCitationKind::AgentHarnessHook));
    assert!(kinds.contains(&WorkReportCitationKind::AgentHarnessHookDisclosure));
    assert_eq!(
        validation_section.summary(),
        Some("Validation diagnostic, local check result, agent harness hook, and disclosure references were supplied.")
    );
}

#[test]
fn generated_report_without_agent_harness_hooks_preserves_validation_section_text() {
    let run = terminal_run(WorkflowRunStatus::Completed);
    let report = generate_terminal_local_work_report(TerminalLocalWorkReportInput {
        agent_harness_hook_invocation_ids: Vec::new(),
        agent_harness_hook_disclosure_ids: Vec::new(),
        ..terminal_generation_input(&run)
    })
    .expect("report without hook citations");

    let validation_section = report
        .sections()
        .iter()
        .find(|section| section.kind() == WorkReportSectionKind::ValidationAndQualityChecks)
        .expect("validation and quality section");

    assert_eq!(
        validation_section.summary(),
        Some("Validation diagnostic and local check result references were supplied.")
    );
    assert!(!validation_section
        .citations()
        .iter()
        .any(|citation| citation.citation_kind() == WorkReportCitationKind::AgentHarnessHook));
    assert!(!validation_section.citations().iter().any(|citation| {
        citation.citation_kind() == WorkReportCitationKind::AgentHarnessHookDisclosure
    }));
}

#[test]
fn generated_report_hook_disclosure_citation_does_not_copy_disclosure_payload() {
    let run = terminal_run(WorkflowRunStatus::Completed);
    let disclosure_id = "hook-disclosure/run-1/pre-validation-warning";
    let report = generate_terminal_local_work_report(TerminalLocalWorkReportInput {
        agent_harness_hook_disclosure_ids: vec![
            AgentHarnessHookDisclosureId::new(disclosure_id).expect("valid hook disclosure id")
        ],
        ..terminal_generation_input(&run)
    })
    .expect("report with hook disclosure citations");

    let debug = format!("{report:?}");
    let serialized = serde_json::to_string(&report).expect("serialize generated report");

    assert!(!debug.contains(disclosure_id));
    assert!(serialized.contains("\"kind\":\"agent_harness_hook_disclosure\""));
    assert!(serialized.contains(disclosure_id));
    assert!(!serialized.contains("hook disclosure title"));
    assert!(!serialized.contains("hook disclosure summary"));
    assert!(!serialized.contains("bounded checkpoint note"));
    assert!(!serialized.contains("hook input"));
    assert!(!serialized.contains("hook output"));
    assert!(!serialized.contains("hook audit record"));
    assert!(!serialized.contains("raw provider payload"));
    assert!(!serialized.contains("raw command output"));
    assert!(!serialized.contains("raw spec contents"));
    assert!(!serialized.contains("bearer-token-super-secret"));
}

#[test]
fn generated_report_hook_citation_does_not_copy_hook_payload() {
    let run = terminal_run(WorkflowRunStatus::Completed);
    let hook_invocation_id = "hook-invocation/run-1/pre-validation";
    let report = generate_terminal_local_work_report(TerminalLocalWorkReportInput {
        agent_harness_hook_invocation_ids: vec![AgentHarnessHookInvocationId::new(
            hook_invocation_id,
        )
        .expect("valid hook invocation id")],
        ..terminal_generation_input(&run)
    })
    .expect("report with hook citations");

    let debug = format!("{report:?}");
    let serialized = serde_json::to_string(&report).expect("serialize generated report");

    assert!(!debug.contains(hook_invocation_id));
    assert!(serialized.contains("\"kind\":\"agent_harness_hook\""));
    assert!(serialized.contains(hook_invocation_id));
    assert!(!serialized.contains("hook disclosure"));
    assert!(!serialized.contains("hook input"));
    assert!(!serialized.contains("hook output"));
    assert!(!serialized.contains("hook audit record"));
    assert!(!serialized.contains("raw provider payload"));
    assert!(!serialized.contains("raw command output"));
    assert!(!serialized.contains("raw spec contents"));
    assert!(!serialized.contains("bearer-token-super-secret"));
}

#[test]
fn generated_report_cites_adapter_telemetry_by_stable_reference() {
    let report = generated_report_for(WorkflowRunStatus::Completed);
    let evidence_section = report
        .sections()
        .iter()
        .find(|section| section.kind() == WorkReportSectionKind::EvidenceConsidered)
        .expect("evidence section");

    assert!(evidence_section.citations().iter().any(|citation| {
        matches!(
            citation.target(),
            WorkReportCitationTarget::AdapterTelemetry { reference }
                if reference.as_str() == "adapter/invocation/1"
        )
    }));
}

#[test]
fn missing_unavailable_references_become_not_available_section_text() {
    let run = terminal_run(WorkflowRunStatus::Completed);
    let report = generate_terminal_local_work_report(TerminalLocalWorkReportInput {
        evidence_reference_ids: Vec::new(),
        validation_reference_ids: Vec::new(),
        local_check_result_references: Vec::new(),
        agent_harness_hook_invocation_ids: Vec::new(),
        agent_harness_hook_disclosure_ids: Vec::new(),
        workflow_event_ids: Vec::new(),
        audit_event_ids: Vec::new(),
        adapter_telemetry_references: Vec::new(),
        policy_event_ids: Vec::new(),
        approval_reference_ids: Vec::new(),
        typed_handoff_ids: Vec::new(),
        incomplete_work: Vec::new(),
        known_limitations: Vec::new(),
        risks: Vec::new(),
        handoff_notes: Vec::new(),
        ..terminal_generation_input(&run)
    })
    .expect("report without optional citations");

    let approvals = report
        .sections()
        .iter()
        .find(|section| section.kind() == WorkReportSectionKind::Approvals)
        .expect("approval section");
    let evidence = report
        .sections()
        .iter()
        .find(|section| section.kind() == WorkReportSectionKind::EvidenceConsidered)
        .expect("evidence section");
    let validation = report
        .sections()
        .iter()
        .find(|section| section.kind() == WorkReportSectionKind::ValidationAndQualityChecks)
        .expect("validation and quality section");

    assert_eq!(
        approvals.summary(),
        Some("No stable approval references were supplied.")
    );
    assert_eq!(
        evidence.summary(),
        Some("No evidence, audit, or adapter telemetry references were supplied.")
    );
    assert_eq!(
        validation.summary(),
        Some("No validation diagnostic, local check result, or agent harness hook references were supplied.")
    );
    assert!(approvals.citations().is_empty());
    assert!(validation.citations().is_empty());
}

#[test]
fn high_assurance_approval_disclosure_populates_approval_posture_without_payloads() {
    let run = terminal_run(WorkflowRunStatus::Completed);
    let report = generate_terminal_local_work_report(TerminalLocalWorkReportInput {
        high_assurance_approval: Some(high_assurance_disclosure()),
        approval_reference_ids: vec![ApprovalReferenceId::new("approval/high-assurance-grant")
            .expect("valid approval reference")],
        evidence_reference_ids: vec![EvidenceReferenceId::new("evidence/high-assurance-context")
            .expect("valid evidence reference")],
        policy_event_ids: vec![
            EventId::new("event/high-assurance-policy").expect("valid policy event reference")
        ],
        validation_reference_ids: vec![ValidationReferenceId::new(
            "validation/high-assurance-context",
        )
        .expect("valid validation reference")],
        local_check_result_references: vec![WorkReportStableReference::new(
            "local-check/high-assurance/context",
        )
        .expect("valid local check reference")],
        ..terminal_generation_input(&run)
    })
    .expect("high-assurance report generated");

    let approvals = report
        .sections()
        .iter()
        .find(|section| section.kind() == WorkReportSectionKind::Approvals)
        .expect("approval section");
    assert_eq!(
        approvals.summary(),
        Some("High-assurance approval validation was used and passed before approval disclosure; stable approval references are cited when supplied.")
    );
    assert!(approvals.citations().iter().any(|citation| {
        matches!(
            citation.target(),
            WorkReportCitationTarget::ApprovalDecision { .. }
        ) && citation.citation_kind() == WorkReportCitationKind::ApprovalDecision
    }));

    assert!(report.known_limitations().iter().any(|limitation| {
        limitation.summary().contains("RBAC")
            && limitation.summary().contains("workflow-declared controls")
            && limitation.summary().contains("write access")
    }));

    let debug = format!("{report:?}");
    assert!(!debug.contains("approval/high-assurance-grant"));
    assert!(!debug.contains("evidence/high-assurance-context"));

    let serialized = serde_json::to_string(&report).expect("report serializes");
    assert!(!serialized.contains("raw approval payload"));
    assert!(!serialized.contains("authorization bearer"));
}

#[test]
fn default_report_without_high_assurance_disclosure_is_unchanged() {
    let report = generated_report_for(WorkflowRunStatus::Completed);
    let approvals = report
        .sections()
        .iter()
        .find(|section| section.kind() == WorkReportSectionKind::Approvals)
        .expect("approval section");

    assert_eq!(
        approvals.summary(),
        Some("Stable approval references were supplied.")
    );
    assert!(!report.known_limitations().iter().any(|limitation| {
        limitation
            .summary()
            .contains("High-assurance approval disclosure")
    }));
}

#[test]
fn high_assurance_approval_disclosure_rejects_inconsistent_validation_posture() {
    let error = WorkReportHighAssuranceApprovalDisclosure::new(
        WorkReportHighAssuranceApprovalDisclosureDefinition {
            validation_used: false,
            validation_passed: true,
            decision: WorkReportHighAssuranceApprovalDecision::Granted,
            requester_approver_posture:
                WorkReportHighAssuranceRequesterApproverPosture::MustDifferValidated,
            required_reference_count: 1,
            supplied_reference_count: 1,
            expiration_posture: WorkReportHighAssuranceExpirationPosture::NotRequired,
            revocation_posture: WorkReportHighAssuranceRevocationPosture::Unsupported,
            denial_fail_closed: true,
        },
    )
    .expect_err("inconsistent disclosure rejected");

    assert_eq!(
        error.code(),
        "work_report.high_assurance_approval.validation_inconsistent"
    );
    assert!(!error.to_string().contains("approval/high-assurance"));
}

#[test]
fn invalid_serialized_high_assurance_approval_disclosure_fails_closed() {
    let value = json!({
        "validation_used": false,
        "validation_passed": true,
        "decision": "granted",
        "requester_approver_posture": "must_differ_validated",
        "required_reference_count": 1,
        "supplied_reference_count": 1,
        "expiration_posture": "not_required",
        "revocation_posture": "unsupported",
        "denial_fail_closed": true
    });

    let error = serde_json::from_value::<WorkReportHighAssuranceApprovalDisclosure>(value)
        .expect_err("invalid serialized disclosure rejected");

    assert!(error
        .to_string()
        .contains("high-assurance approval disclosure cannot pass validation"));
    assert!(!error.to_string().contains("approval/high-assurance"));
}

#[test]
fn artifact_requirement_not_required_maps_to_disabled_gate_policy() {
    let requirement =
        WorkReportArtifactRequirement::new(WorkReportArtifactRequirementDefinition::default())
            .expect("valid default artifact requirement");

    assert_eq!(
        requirement.high_assurance_approval(),
        WorkReportArtifactHighAssuranceRequirement::NotRequired
    );
    assert_eq!(
        requirement.high_assurance_disclosure_policy(),
        WorkReportArtifactHighAssuranceDisclosurePolicy::disabled()
    );
    assert!(!requirement.high_assurance_disclosure_policy().is_enabled());
}

#[test]
fn artifact_requirement_maps_high_assurance_postures_to_gate_policies() {
    let cases = [
        (
            WorkReportArtifactHighAssuranceRequirement::DisclosureRequired,
            WorkReportArtifactHighAssuranceDisclosurePolicy::require_disclosure(),
        ),
        (
            WorkReportArtifactHighAssuranceRequirement::ValidatedDisclosureRequired,
            WorkReportArtifactHighAssuranceDisclosurePolicy::require_validated(),
        ),
        (
            WorkReportArtifactHighAssuranceRequirement::ValidatedFailClosedDisclosureRequired,
            WorkReportArtifactHighAssuranceDisclosurePolicy::require_validated_fail_closed(),
        ),
    ];

    for (posture, expected_policy) in cases {
        let requirement =
            WorkReportArtifactRequirement::new(WorkReportArtifactRequirementDefinition {
                high_assurance_approval: posture,
                unsupported_high_assurance_requirements: Vec::new(),
            })
            .expect("valid artifact requirement");

        assert_eq!(requirement.high_assurance_approval(), posture);
        assert_eq!(
            requirement.high_assurance_disclosure_policy(),
            expected_policy
        );
        assert!(requirement.high_assurance_disclosure_policy().is_enabled());
    }
}

#[test]
fn workflow_artifact_gate_derivation_defaults_absent_requirement_to_disabled_policy() {
    let workflow = workflow_with_artifact_requirement(None);
    let before = workflow.clone();

    let derivation =
        derive_workflow_report_artifact_gate_policy(WorkflowReportArtifactGateDerivationInput {
            workflow: &workflow,
        })
        .expect("derivation succeeds");

    assert_eq!(
        derivation.high_assurance_disclosure_policy(),
        WorkReportArtifactHighAssuranceDisclosurePolicy::disabled()
    );
    assert!(!derivation.high_assurance_disclosure_policy().is_enabled());
    assert_eq!(workflow, before);
}

#[test]
fn workflow_artifact_gate_derivation_maps_not_required_to_disabled_policy() {
    let workflow = workflow_with_artifact_requirement(Some("not_required"));

    let derivation =
        derive_workflow_report_artifact_gate_policy(WorkflowReportArtifactGateDerivationInput {
            workflow: &workflow,
        })
        .expect("derivation succeeds");

    assert_eq!(
        derivation.high_assurance_disclosure_policy(),
        WorkReportArtifactHighAssuranceDisclosurePolicy::disabled()
    );
    assert!(!derivation.high_assurance_disclosure_policy().is_enabled());
}

#[test]
fn workflow_artifact_gate_derivation_maps_enforcement_postures_to_gate_policies() {
    let cases = [
        (
            "disclosure_required",
            WorkReportArtifactHighAssuranceDisclosurePolicy::require_disclosure(),
        ),
        (
            "validated_disclosure_required",
            WorkReportArtifactHighAssuranceDisclosurePolicy::require_validated(),
        ),
        (
            "validated_fail_closed_disclosure_required",
            WorkReportArtifactHighAssuranceDisclosurePolicy::require_validated_fail_closed(),
        ),
    ];

    for (posture, expected_policy) in cases {
        let workflow = workflow_with_artifact_requirement(Some(posture));

        let derivation = derive_workflow_report_artifact_gate_policy(
            WorkflowReportArtifactGateDerivationInput {
                workflow: &workflow,
            },
        )
        .expect("derivation succeeds");

        assert_eq!(
            derivation.high_assurance_disclosure_policy(),
            expected_policy
        );
        assert!(derivation.high_assurance_disclosure_policy().is_enabled());
    }
}

#[test]
fn artifact_requirement_rejects_unsupported_future_governance_without_leaking() {
    let error = WorkReportArtifactRequirement::new(WorkReportArtifactRequirementDefinition {
        high_assurance_approval:
            WorkReportArtifactHighAssuranceRequirement::ValidatedDisclosureRequired,
        unsupported_high_assurance_requirements: vec![
            WorkReportArtifactUnsupportedHighAssuranceRequirement::QuorumApproval,
            WorkReportArtifactUnsupportedHighAssuranceRequirement::RevocationEnforcement,
        ],
    })
    .expect_err("unsupported future requirements fail closed");

    assert_eq!(
        error.code(),
        "work_report_artifact_requirement.high_assurance.unsupported"
    );
    assert!(!format!("{error:?}").contains("quorum"));
    assert!(!format!("{error:?}").contains("revocation"));
}

#[test]
fn artifact_requirement_rejects_duplicate_unsupported_future_governance_without_leaking() {
    let error = WorkReportArtifactRequirement::new(WorkReportArtifactRequirementDefinition {
        high_assurance_approval: WorkReportArtifactHighAssuranceRequirement::NotRequired,
        unsupported_high_assurance_requirements: vec![
            WorkReportArtifactUnsupportedHighAssuranceRequirement::RoleBoundAuthority,
            WorkReportArtifactUnsupportedHighAssuranceRequirement::RoleBoundAuthority,
        ],
    })
    .expect_err("duplicate unsupported requirements fail closed");

    assert_eq!(
        error.code(),
        "work_report_artifact_requirement.high_assurance.duplicate_unsupported"
    );
    assert!(!format!("{error:?}").contains("role"));
}

#[test]
fn artifact_requirement_serde_round_trip_uses_validated_shape() {
    let requirement = WorkReportArtifactRequirement::new(WorkReportArtifactRequirementDefinition {
        high_assurance_approval:
            WorkReportArtifactHighAssuranceRequirement::ValidatedFailClosedDisclosureRequired,
        unsupported_high_assurance_requirements: Vec::new(),
    })
    .expect("valid artifact requirement");

    let serialized = serde_json::to_string(&requirement).expect("requirement serializes");
    assert!(serialized.contains("validated_fail_closed_disclosure_required"));
    assert!(!serialized.contains("raw provider payload"));
    assert!(!serialized.contains("authorization bearer"));

    let deserialized: WorkReportArtifactRequirement =
        serde_json::from_str(&serialized).expect("requirement deserializes");
    assert_eq!(deserialized, requirement);
    assert_eq!(
        deserialized.high_assurance_disclosure_policy(),
        WorkReportArtifactHighAssuranceDisclosurePolicy::require_validated_fail_closed()
    );
}

#[test]
fn invalid_serialized_artifact_requirement_fails_closed_without_leaking_values() {
    let value = json!({
        "high_assurance_approval": "validated_disclosure_required",
        "unsupported_high_assurance_requirements": ["quorum_approval"]
    });

    let error = serde_json::from_value::<WorkReportArtifactRequirement>(value)
        .expect_err("invalid serialized artifact requirement rejected");

    assert!(error
        .to_string()
        .contains("invalid work report artifact requirement"));
    assert!(!error.to_string().contains("quorum"));
}

#[test]
fn generated_report_cites_typed_handoffs_by_stable_reference() {
    let report = generated_report_for(WorkflowRunStatus::Completed);
    let handoff_section = report
        .sections()
        .iter()
        .find(|section| section.kind() == WorkReportSectionKind::OperatorHandoffNotes)
        .expect("operator handoff section");

    assert_eq!(
        handoff_section.summary(),
        Some("Operator handoff notes were supplied.")
    );
    assert!(handoff_section.citations().iter().any(|citation| {
        matches!(
            citation.target(),
            WorkReportCitationTarget::TypedHandoff { typed_handoff_id }
                if typed_handoff_id.as_str() == "typed-handoff/final-review"
        ) && citation.citation_kind() == WorkReportCitationKind::TypedHandoff
    }));
}

#[test]
fn generated_report_without_typed_handoffs_preserves_operator_handoff_text() {
    let run = terminal_run(WorkflowRunStatus::Completed);
    let report = generate_terminal_local_work_report(TerminalLocalWorkReportInput {
        typed_handoff_ids: Vec::new(),
        handoff_notes: Vec::new(),
        ..terminal_generation_input(&run)
    })
    .expect("report without typed handoff citations");

    let handoff_section = report
        .sections()
        .iter()
        .find(|section| section.kind() == WorkReportSectionKind::OperatorHandoffNotes)
        .expect("operator handoff section");

    assert_eq!(
        handoff_section.summary(),
        Some("No operator handoff notes were supplied.")
    );
    assert!(handoff_section.citations().is_empty());
}

#[test]
fn generated_report_typed_handoff_citation_does_not_copy_handoff_payload() {
    let report = generated_report_for(WorkflowRunStatus::Completed);
    let debug = format!("{report:?}");
    let serialized = serde_json::to_string(&report).expect("serialize generated report");

    assert!(!debug.contains("typed-handoff/final-review"));
    assert!(serialized.contains("\"kind\":\"typed_handoff\""));
    assert!(serialized.contains("typed-handoff/final-review"));
    assert!(!serialized.contains("handoff obligation"));
    assert!(!serialized.contains("handoff disclosure"));
    assert!(!serialized.contains("handoff risk"));
    assert!(!serialized.contains("operator note payload"));
    assert!(!serialized.contains("raw provider payload"));
    assert!(!serialized.contains("raw command output"));
    assert!(!serialized.contains("raw spec contents"));
}

#[test]
fn side_effects_section_is_present_as_unsupported() {
    let report = generated_report_for(WorkflowRunStatus::Completed);
    let side_effects = report
        .sections()
        .iter()
        .find(|section| section.kind() == WorkReportSectionKind::SideEffects)
        .expect("side effects section");

    assert_eq!(
        side_effects.summary(),
        Some(
            "No write side effects are supported; side effects are none, skipped, or unsupported."
        )
    );
    assert!(side_effects.citations().is_empty());
}

#[test]
fn generated_report_cites_side_effect_ids_in_side_effects_section() {
    let run = terminal_run(WorkflowRunStatus::Completed);
    let report = generate_terminal_local_work_report(TerminalLocalWorkReportInput {
        side_effect_ids: vec![
            SideEffectId::new("side-effect/run-123/proposed-write").expect("valid side effect id")
        ],
        ..terminal_generation_input(&run)
    })
    .expect("report with side effect citation");

    let side_effects = report
        .sections()
        .iter()
        .find(|section| section.kind() == WorkReportSectionKind::SideEffects)
        .expect("side effects section");

    assert_eq!(
        side_effects.summary(),
        Some(
            "Side-effect records were supplied as stable references; no side-effect payloads are copied."
        )
    );
    assert_eq!(side_effects.citations().len(), 1);
    assert!(side_effects.citations().iter().any(|citation| {
        matches!(
            citation.target(),
            WorkReportCitationTarget::SideEffect { side_effect_id }
                if side_effect_id.as_str() == "side-effect/run-123/proposed-write"
        ) && citation.citation_kind() == WorkReportCitationKind::SideEffect
    }));
}

#[test]
fn generated_report_side_effect_citation_order_is_deterministic() {
    let run = terminal_run(WorkflowRunStatus::Completed);
    let report = generate_terminal_local_work_report(TerminalLocalWorkReportInput {
        side_effect_ids: vec![
            SideEffectId::new("side-effect/run-123/proposed-write").expect("valid side effect id"),
            SideEffectId::new("side-effect/run-123/denied-write").expect("valid side effect id"),
        ],
        ..terminal_generation_input(&run)
    })
    .expect("report with side effect citations");

    let side_effects = report
        .sections()
        .iter()
        .find(|section| section.kind() == WorkReportSectionKind::SideEffects)
        .expect("side effects section");

    let ids: Vec<_> = side_effects
        .citations()
        .iter()
        .filter_map(|citation| match citation.target() {
            WorkReportCitationTarget::SideEffect { side_effect_id } => {
                Some(side_effect_id.as_str())
            }
            _ => None,
        })
        .collect();

    assert_eq!(side_effects.citations().len(), ids.len());
    assert_eq!(
        ids,
        vec![
            "side-effect/run-123/proposed-write",
            "side-effect/run-123/denied-write"
        ]
    );
}

#[test]
fn generated_report_side_effect_citation_does_not_copy_side_effect_payload() {
    let run = terminal_run(WorkflowRunStatus::Completed);
    let side_effect_id = "side-effect/run-123/proposed-write";
    let report = generate_terminal_local_work_report(TerminalLocalWorkReportInput {
        side_effect_ids: vec![SideEffectId::new(side_effect_id).expect("valid side effect id")],
        ..terminal_generation_input(&run)
    })
    .expect("report with side effect citation");

    let debug = format!("{report:?}");
    let serialized = serde_json::to_string(&report).expect("serialize generated report");

    assert!(!debug.contains(side_effect_id));
    assert!(serialized.contains("\"kind\":\"side_effect\""));
    assert!(serialized.contains(side_effect_id));
    assert!(!serialized.contains("side effect target"));
    assert!(!serialized.contains("side effect summary"));
    assert!(!serialized.contains("side effect reason"));
    assert!(!serialized.contains("side effect outcome"));
    assert!(!serialized.contains("idempotency"));
    assert!(!serialized.contains("raw provider payload"));
    assert!(!serialized.contains("raw command output"));
    assert!(!serialized.contains("raw spec contents"));
    assert!(!serialized.contains("bearer-token-super-secret"));
}

#[test]
fn side_effect_discovery_helper_cites_store_backed_records() {
    let run = terminal_run(WorkflowRunStatus::Completed);
    let backend = temp_state_backend("side-effect-discovery-cites-records");
    let side_effect_id =
        SideEffectId::new("side-effect/run-123/proposed-write").expect("valid side-effect id");
    let record = side_effect_record_for_run(&run, side_effect_id);
    backend
        .write_side_effect_record(&record)
        .expect("side-effect record stored");

    let report = generate_terminal_local_work_report_with_side_effect_discovery(
        &backend,
        terminal_generation_input(&run),
        TerminalLocalWorkReportSideEffectDiscoveryInput {
            include_workflow_events: false,
            include_store_records: true,
            require_records: true,
        },
    )
    .expect("report generated with discovered side-effect citation");

    let side_effects = report
        .sections()
        .iter()
        .find(|section| section.kind() == WorkReportSectionKind::SideEffects)
        .expect("side effects section");
    assert_eq!(side_effects.citations().len(), 1);
    assert!(side_effects.citations().iter().any(|citation| {
        matches!(
            citation.target(),
            WorkReportCitationTarget::SideEffect { side_effect_id }
                if side_effect_id.as_str() == "side-effect/run-123/proposed-write"
        )
    }));

    let serialized = serde_json::to_string(&report).expect("serialize report");
    assert!(!serialized.contains("github/pull-request/42"));
    assert!(!serialized.contains("bounded side-effect summary"));
}

#[test]
fn side_effect_discovery_helper_merges_explicit_and_store_records_deterministically() {
    let run = terminal_run(WorkflowRunStatus::Completed);
    let backend = temp_state_backend("side-effect-discovery-merges-records");
    let discovered_id =
        SideEffectId::new("side-effect/run-123/proposed-write").expect("valid side-effect id");
    let explicit_id =
        SideEffectId::new("side-effect/run-123/explicit-write").expect("valid side-effect id");
    let record = side_effect_record_for_run(&run, discovered_id.clone());
    backend
        .write_side_effect_record(&record)
        .expect("side-effect record stored");

    let report = generate_terminal_local_work_report_with_side_effect_discovery(
        &backend,
        TerminalLocalWorkReportInput {
            side_effect_ids: vec![discovered_id.clone(), explicit_id, discovered_id],
            ..terminal_generation_input(&run)
        },
        TerminalLocalWorkReportSideEffectDiscoveryInput {
            include_workflow_events: false,
            include_store_records: true,
            require_records: false,
        },
    )
    .expect("report generated with merged side-effect citations");

    let side_effects = report
        .sections()
        .iter()
        .find(|section| section.kind() == WorkReportSectionKind::SideEffects)
        .expect("side effects section");
    let ids = side_effects
        .citations()
        .iter()
        .filter_map(|citation| match citation.target() {
            WorkReportCitationTarget::SideEffect { side_effect_id } => {
                Some(side_effect_id.as_str())
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(
        ids,
        vec![
            "side-effect/run-123/explicit-write",
            "side-effect/run-123/proposed-write"
        ]
    );
}

#[test]
fn side_effect_discovery_helper_rejects_missing_source_without_leaking() {
    let run = terminal_run(WorkflowRunStatus::Completed);
    let backend = temp_state_backend("side-effect-discovery-no-source");
    let error = generate_terminal_local_work_report_with_side_effect_discovery(
        &backend,
        TerminalLocalWorkReportInput {
            side_effect_ids: vec![SideEffectId::new("side-effect/run-123/no-source-marker")
                .expect("valid side-effect id")],
            ..terminal_generation_input(&run)
        },
        TerminalLocalWorkReportSideEffectDiscoveryInput {
            include_workflow_events: false,
            include_store_records: false,
            require_records: false,
        },
    )
    .expect_err("missing discovery source rejected");

    assert_eq!(
        error.code(),
        "work_report_generation.side_effect_discovery.source_required"
    );
    assert!(!error.to_string().contains("no-source-marker"));
}

#[test]
fn side_effect_discovery_helper_required_missing_record_fails_without_leaking() {
    let run = terminal_run(WorkflowRunStatus::Completed);
    let backend = temp_state_backend("side-effect-discovery-missing-record");
    let error = generate_terminal_local_work_report_with_side_effect_discovery(
        &backend,
        TerminalLocalWorkReportInput {
            side_effect_ids: vec![SideEffectId::new("side-effect/run-123/missing-record")
                .expect("valid side-effect id")],
            ..terminal_generation_input(&run)
        },
        TerminalLocalWorkReportSideEffectDiscoveryInput {
            include_workflow_events: false,
            include_store_records: true,
            require_records: true,
        },
    )
    .expect_err("missing required record rejected");

    assert_eq!(error.code(), "side_effect_discovery.record_missing");
    assert!(!error.to_string().contains("missing-record"));
}

#[test]
fn side_effect_discovery_helper_does_not_mutate_run_or_write_artifacts() {
    let run = terminal_run(WorkflowRunStatus::Completed);
    let before = run.clone();
    let backend = temp_state_backend("side-effect-discovery-no-mutation");
    let side_effect_id =
        SideEffectId::new("side-effect/run-123/proposed-write").expect("valid side-effect id");
    let record = side_effect_record_for_run(&run, side_effect_id);
    backend
        .write_side_effect_record(&record)
        .expect("side-effect record stored");

    let report = generate_terminal_local_work_report_with_side_effect_discovery(
        &backend,
        terminal_generation_input(&run),
        TerminalLocalWorkReportSideEffectDiscoveryInput {
            include_workflow_events: false,
            include_store_records: true,
            require_records: true,
        },
    )
    .expect("report generated");

    assert_eq!(run, before);
    assert_eq!(run.events.len(), before.events.len());
    assert!(backend
        .list_work_report_artifacts(&run.snapshot.identity.run_id)
        .expect("artifact list succeeds")
        .is_empty());
    assert_eq!(
        report.generation_context().run_id,
        run.snapshot.identity.run_id
    );
}

#[test]
fn report_generation_preserves_workflow_run_semantics_and_events() {
    let run = terminal_run(WorkflowRunStatus::Completed);
    let original = run.clone();

    let report = generate_terminal_local_work_report(terminal_generation_input(&run))
        .expect("report generated");

    assert_eq!(run, original);
    assert_eq!(run.snapshot.status, WorkflowRunStatus::Completed);
    assert_eq!(run.events.len(), original.events.len());
    assert_eq!(
        report.generation_context().terminal_run_status,
        WorkReportStatus::Completed
    );
}

#[test]
fn helper_can_be_used_without_state_backend_write() {
    let report = generated_report_for(WorkflowRunStatus::Completed);

    assert_eq!(report.report_id().as_str(), "report/local-run");
    assert_eq!(report.generation_context().run_id.as_str(), "run-123");
}

#[test]
fn helper_creates_no_filesystem_artifacts() {
    let report_dir = std::env::temp_dir().join(format!(
        "workflow-os-work-report-helper-{}",
        WorkReportId::generate().as_str()
    ));
    std::fs::create_dir_all(&report_dir).expect("test dir created");
    let before = std::fs::read_dir(&report_dir).expect("read dir").count();

    let report = generated_report_for(WorkflowRunStatus::Completed);

    let after = std::fs::read_dir(&report_dir)
        .expect("read dir after")
        .count();
    std::fs::remove_dir_all(&report_dir).expect("test dir removed");
    assert_eq!(before, 0);
    assert_eq!(after, 0);
    assert_eq!(report.report_id().as_str(), "report/local-run");
}

#[test]
fn raw_provider_spec_command_and_parser_payloads_are_not_copied() {
    let report = generated_report_for(WorkflowRunStatus::Completed);
    let serialized = serde_json::to_string(&report).expect("serialize generated report");

    assert!(!serialized.contains("provider_payload"));
    assert!(!serialized.contains("raw_spec_contents"));
    assert!(!serialized.contains("command_output"));
    assert!(!serialized.contains("parser_payload"));
    assert!(!serialized.contains("authorization"));
    assert!(!serialized.contains("private_key"));
}

#[test]
fn helper_returns_work_report_model_not_cli_output() {
    let report = generated_report_for(WorkflowRunStatus::Completed);

    assert_eq!(report.sections().len(), 11);
    assert_eq!(report.generation_context().run_id.as_str(), "run-123");
    assert!(!format!("{report:?}").contains("Usage:"));
    assert!(!format!("{report:?}").contains("workflow-os "));
}

#[test]
fn terminal_report_result_exposes_run_and_report_in_memory() {
    let run = terminal_run(WorkflowRunStatus::Completed);
    let original = run.clone();

    let result = expose_terminal_local_work_report_result(terminal_generation_input(&run))
        .expect("report result generated");

    assert_eq!(result.run(), &original);
    assert_eq!(run, original);
    assert_eq!(result.run().snapshot.status, WorkflowRunStatus::Completed);
    assert_eq!(
        result
            .work_report()
            .generation_context()
            .terminal_run_status,
        WorkReportStatus::Completed
    );
    assert_eq!(result.work_report().sections().len(), 11);
}

#[test]
fn terminal_report_result_rejects_non_terminal_run_without_mutation() {
    let run = running_run();
    let original = run.clone();
    let secret = "authorization bearer token";
    let error = expose_terminal_local_work_report_result(TerminalLocalWorkReportInput {
        handoff_notes: vec![secret.to_owned()],
        ..terminal_generation_input(&run)
    })
    .expect_err("non-terminal report result rejected");

    assert_eq!(error.code(), "work_report_generation.status.not_terminal");
    assert_eq!(run, original);
    assert!(!error.to_string().contains(secret));
}

#[test]
fn terminal_report_result_into_parts_returns_owned_values() {
    let run = terminal_run(WorkflowRunStatus::Failed);
    let result = expose_terminal_local_work_report_result(terminal_generation_input(&run))
        .expect("report result generated");

    let (owned_run, report) = result.into_parts();

    assert_eq!(owned_run.snapshot.status, WorkflowRunStatus::Failed);
    assert_eq!(
        report.generation_context().terminal_run_status,
        WorkReportStatus::Failed
    );
}

#[test]
fn terminal_report_result_debug_is_redaction_safe() {
    let run = terminal_run(WorkflowRunStatus::Completed);
    let report = generate_terminal_local_work_report(terminal_generation_input(&run))
        .expect("report generated");
    let result = TerminalLocalWorkReportResult::new(run, report);
    let debug = format!("{result:?}");

    assert!(debug.contains("TerminalLocalWorkReportResult"));
    assert!(debug.contains("run_event_count"));
    assert!(!debug.contains("workflow/intake"));
    assert!(!debug.contains("run-123"));
    assert!(!debug.contains("adapter/invocation/1"));
    assert!(!debug.contains("token"));
    assert!(!debug.contains("authorization"));
}

#[test]
fn secret_like_handoff_notes_are_rejected() {
    let run = terminal_run(WorkflowRunStatus::Completed);
    let secret = "authorization bearer token";
    let error = generate_terminal_local_work_report(TerminalLocalWorkReportInput {
        handoff_notes: vec![secret.to_owned()],
        ..terminal_generation_input(&run)
    })
    .expect_err("secret-like handoff note rejected");

    assert_eq!(error.code(), "work_report_contract.secret_like_identifier");
    assert!(!error.to_string().contains(secret));
}

#[test]
fn secret_like_known_limitations_are_rejected() {
    let run = terminal_run(WorkflowRunStatus::Completed);
    let secret = "authorization bearer token";
    let error = generate_terminal_local_work_report(TerminalLocalWorkReportInput {
        known_limitations: vec![secret.to_owned()],
        ..terminal_generation_input(&run)
    })
    .expect_err("secret-like known limitation rejected");

    assert_eq!(error.code(), "work_report_contract.secret_like_identifier");
    assert!(!error.to_string().contains(secret));
}

#[test]
fn secret_like_risks_are_rejected() {
    let run = terminal_run(WorkflowRunStatus::Completed);
    let secret = "authorization bearer token";
    let error = generate_terminal_local_work_report(TerminalLocalWorkReportInput {
        risks: vec![secret.to_owned()],
        ..terminal_generation_input(&run)
    })
    .expect_err("secret-like risk rejected");

    assert_eq!(error.code(), "work_report_contract.secret_like_identifier");
    assert!(!error.to_string().contains(secret));
}

#[test]
fn secret_like_incomplete_work_disclosures_are_rejected() {
    let run = terminal_run(WorkflowRunStatus::Completed);
    let secret = "authorization bearer token";
    let error = generate_terminal_local_work_report(TerminalLocalWorkReportInput {
        incomplete_work: vec![secret.to_owned()],
        ..terminal_generation_input(&run)
    })
    .expect_err("secret-like incomplete work disclosure rejected");

    assert_eq!(error.code(), "work_report_contract.secret_like_identifier");
    assert!(!error.to_string().contains(secret));
}

#[test]
fn generated_report_debug_and_serialization_do_not_leak_secret_like_inputs() {
    let run = terminal_run(WorkflowRunStatus::Completed);
    let secret = "token should not appear";
    let error = generate_terminal_local_work_report(TerminalLocalWorkReportInput {
        handoff_notes: vec![secret.to_owned()],
        ..terminal_generation_input(&run)
    })
    .expect_err("secret-like handoff note rejected");
    assert!(!error.to_string().contains(secret));

    let report = generated_report_for(WorkflowRunStatus::Completed);
    let debug = format!("{report:?}");
    let serialized = serde_json::to_string(&report).expect("serialize report");

    assert!(!debug.contains("Operator should review cited references."));
    assert!(!debug.contains("adapter/invocation/1"));
    assert!(!debug.contains("token"));
    assert!(!serialized.contains("token should not appear"));
    assert!(!serialized.contains("provider_payload"));
}

fn valid_report_definition() -> WorkReportDefinition {
    WorkReportDefinition {
        report_id: report_id(),
        report_contract_id: contract_id(),
        report_contract_version: contract_version(),
        generation_context: generation_context(WorkReportStatus::Completed),
        sections: required_sections(),
        incomplete_work: Vec::new(),
        known_limitations: Vec::new(),
        risks: Vec::new(),
        handoff_notes: Vec::new(),
        high_assurance_approval: None,
        sensitivity: WorkReportSensitivity::Confidential,
        redaction: redaction(),
    }
}
