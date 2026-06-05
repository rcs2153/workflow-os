#![allow(clippy::expect_used)]

//! `EvidenceReference` core model tests.

use std::collections::BTreeMap;

use workflow_core::{
    ActorId, AdapterId, AdapterKind, CorrelationId, EventId, EvidenceKind, EvidenceMetadata,
    EvidenceRedactionMetadata, EvidenceReference, EvidenceReferenceId,
    EvidenceReferenceRequiredFields, EvidenceReferenceTarget, EvidenceScope, EvidenceSensitivity,
    EvidenceSourceComponent, SchemaVersion, SkillId, SkillVersion, SpecContentHash, StepId,
    Timestamp, ValidationReferenceId, WorkflowId, WorkflowRunId, WorkflowVersion,
};

fn redaction() -> EvidenceRedactionMetadata {
    EvidenceRedactionMetadata::reference_only("target", "stores a reference, not raw evidence")
        .expect("redaction metadata")
}

fn timestamp() -> Timestamp {
    Timestamp::parse_rfc3339("2026-06-04T00:00:00Z").expect("timestamp")
}

fn required_fields(
    id: &str,
    kind: EvidenceKind,
    title: &str,
    target: EvidenceReferenceTarget,
    source_component: EvidenceSourceComponent,
    scope: EvidenceScope,
    sensitivity: Option<EvidenceSensitivity>,
) -> EvidenceReferenceRequiredFields {
    EvidenceReferenceRequiredFields {
        id: EvidenceReferenceId::new(id).expect("evidence id"),
        kind,
        title: title.to_owned(),
        target,
        source_component,
        scope,
        created_at: timestamp(),
        redaction_metadata: redaction(),
        sensitivity,
    }
}

fn base_reference(kind: EvidenceKind, scope: EvidenceScope) -> EvidenceReference {
    EvidenceReference::new(required_fields(
        "evidence/test",
        kind,
        "Validation evidence",
        EvidenceReferenceTarget::internal("validation_result", "validation/result-1")
            .expect("target"),
        EvidenceSourceComponent::Validator,
        scope,
        Some(EvidenceSensitivity::Internal),
    ))
    .expect("evidence reference")
}

fn run_identity(reference: EvidenceReference) -> EvidenceReference {
    reference.with_run_identity(
        WorkflowId::new("workflow/evidence").expect("workflow id"),
        WorkflowVersion::new("v1").expect("workflow version"),
        SchemaVersion::new("workflow-os/v0").expect("schema version"),
        SpecContentHash::from_text("workflow"),
        WorkflowRunId::new("run/evidence").expect("run id"),
    )
}

#[test]
fn serialization_round_trips_valid_evidence_reference() {
    let mut reference = base_reference(EvidenceKind::ValidationResult, EvidenceScope::Validation)
        .with_validation_result_id(
            ValidationReferenceId::new("validation/result-1").expect("validation reference"),
        );
    reference
        .set_summary("Validation succeeded")
        .expect("summary");
    reference.correlation_id =
        Some(CorrelationId::new("correlation/evidence").expect("correlation"));
    reference.actor = Some(ActorId::new("user/evaluator").expect("actor"));
    reference.validate().expect("valid reference");

    let json = serde_json::to_string(&reference).expect("serialize");
    let decoded: EvidenceReference = serde_json::from_str(&json).expect("deserialize");

    assert_eq!(decoded, reference);
}

#[test]
fn kind_taxonomy_serializes_as_snake_case() {
    let cases = [
        (EvidenceKind::LocalFile, "local_file"),
        (EvidenceKind::SpecFile, "spec_file"),
        (EvidenceKind::ValidationResult, "validation_result"),
        (EvidenceKind::WorkflowEvent, "workflow_event"),
        (EvidenceKind::AuditEvent, "audit_event"),
        (EvidenceKind::AdapterInvocation, "adapter_invocation"),
        (
            EvidenceKind::AdapterResponseSummary,
            "adapter_response_summary",
        ),
        (EvidenceKind::ApprovalDecision, "approval_decision"),
        (EvidenceKind::PolicyDecision, "policy_decision"),
        (EvidenceKind::OperatorNote, "operator_note"),
        (EvidenceKind::ExternalReference, "external_reference"),
        (EvidenceKind::TestResult, "test_result"),
        (EvidenceKind::CommandOutput, "command_output"),
        (EvidenceKind::ReleaseReview, "release_review"),
        (EvidenceKind::LiveSmokeEvidence, "live_smoke_evidence"),
    ];

    for (kind, expected) in cases {
        let json = serde_json::to_string(&kind).expect("serialize kind");
        assert_eq!(json, format!("\"{expected}\""));
        let decoded: EvidenceKind = serde_json::from_str(&json).expect("deserialize kind");
        assert_eq!(decoded, kind);
    }
}

#[test]
fn scope_taxonomy_serializes_as_snake_case() {
    let cases = [
        (EvidenceScope::Project, "project"),
        (EvidenceScope::Workflow, "workflow"),
        (EvidenceScope::Run, "run"),
        (EvidenceScope::Step, "step"),
        (EvidenceScope::Skill, "skill"),
        (EvidenceScope::Adapter, "adapter"),
        (EvidenceScope::Audit, "audit"),
        (EvidenceScope::Validation, "validation"),
        (EvidenceScope::Approval, "approval"),
        (EvidenceScope::Policy, "policy"),
        (EvidenceScope::External, "external"),
        (EvidenceScope::Release, "release"),
        (EvidenceScope::Operator, "operator"),
    ];

    for (scope, expected) in cases {
        let json = serde_json::to_string(&scope).expect("serialize scope");
        assert_eq!(json, format!("\"{expected}\""));
        let decoded: EvidenceScope = serde_json::from_str(&json).expect("deserialize scope");
        assert_eq!(decoded, scope);
    }
}

#[test]
fn valid_project_scoped_evidence_validates() {
    let reference = base_reference(EvidenceKind::SpecFile, EvidenceScope::Project);

    reference.validate().expect("project evidence validates");
}

#[test]
fn valid_run_scoped_evidence_validates() {
    let reference = run_identity(base_reference(
        EvidenceKind::WorkflowEvent,
        EvidenceScope::Run,
    ))
    .with_workflow_event_id(EventId::new("event/run").expect("event id"));

    reference.validate().expect("run evidence validates");
}

#[test]
fn run_scoped_evidence_missing_immutable_identity_fails_validation() {
    let reference = base_reference(EvidenceKind::SpecFile, EvidenceScope::Run);

    let error = reference.validate().expect_err("missing run identity");
    assert_eq!(error.code(), "evidence.scope.workflow_id_required");
    assert!(!error.to_string().contains("token="));
}

#[test]
fn step_scoped_evidence_missing_step_id_fails_validation() {
    let reference = run_identity(base_reference(EvidenceKind::SpecFile, EvidenceScope::Step));

    let error = reference.validate().expect_err("missing step id");
    assert_eq!(error.code(), "evidence.scope.step_id_required");
}

#[test]
fn skill_scoped_evidence_missing_skill_version_fails_validation() {
    let mut reference = run_identity(base_reference(EvidenceKind::SpecFile, EvidenceScope::Skill));
    reference.skill_id = Some(SkillId::new("skill/evidence").expect("skill id"));

    let error = reference.validate().expect_err("missing skill version");
    assert_eq!(error.code(), "evidence.scope.skill_version_required");
}

#[test]
fn adapter_scoped_evidence_missing_adapter_fields_fails_validation() {
    let reference = base_reference(EvidenceKind::AdapterInvocation, EvidenceScope::Adapter);

    let error = reference.validate().expect_err("missing adapter id");
    assert_eq!(error.code(), "evidence.scope.adapter_id_required");
}

#[test]
fn adapter_scoped_evidence_with_adapter_fields_validates() {
    let reference = base_reference(EvidenceKind::AdapterInvocation, EvidenceScope::Adapter)
        .with_adapter(
            AdapterId::new("adapter/github").expect("adapter id"),
            AdapterKind::GitHub,
        );

    reference.validate().expect("adapter evidence validates");
}

#[test]
fn audit_scoped_evidence_missing_audit_event_id_fails_validation() {
    let reference = base_reference(EvidenceKind::AuditEvent, EvidenceScope::Audit);

    let error = reference.validate().expect_err("missing audit event id");
    assert_eq!(error.code(), "evidence.scope.audit_event_id_required");
}

#[test]
fn validation_scoped_evidence_missing_validation_reference_fails_validation() {
    let reference = base_reference(EvidenceKind::ValidationResult, EvidenceScope::Validation);

    let error = reference
        .validate()
        .expect_err("missing validation reference");
    assert_eq!(error.code(), "evidence.scope.validation_reference_required");
}

#[test]
fn approval_scoped_evidence_requires_approval_reference() {
    let reference = base_reference(EvidenceKind::ApprovalDecision, EvidenceScope::Approval);

    let error = reference.validate().expect_err("missing approval id");
    assert_eq!(error.code(), "evidence.scope.approval_id_required");
}

#[test]
fn external_reference_defaults_sensitivity_conservatively() {
    let reference = EvidenceReference::new(required_fields(
        "evidence/external",
        EvidenceKind::ExternalReference,
        "External reference",
        EvidenceReferenceTarget::external("jira", "KAN-1").expect("external target"),
        EvidenceSourceComponent::Adapter,
        EvidenceScope::External,
        Some(EvidenceSensitivity::Public),
    ))
    .expect("external evidence");

    assert_eq!(reference.sensitivity, EvidenceSensitivity::Confidential);
}

#[test]
fn command_output_cannot_store_raw_output_by_default() {
    let reference = EvidenceReference::new(required_fields(
        "evidence/command",
        EvidenceKind::CommandOutput,
        "Command output",
        EvidenceReferenceTarget::command_output(
            "npm run check",
            "raw CI log: Authorization: Bearer should-not-appear",
        )
        .expect("command output target"),
        EvidenceSourceComponent::Cli,
        EvidenceScope::Operator,
        Some(EvidenceSensitivity::Internal),
    ))
    .expect("command output evidence");

    reference.validate().expect("command output validates");
    let json = serde_json::to_string(&reference).expect("serialize");
    assert!(!json.contains("should-not-appear"));
    assert!(!json.contains("Authorization: Bearer"));
    assert!(json.contains("[REDACTED]"));
}

#[test]
fn command_output_requires_redaction_metadata_that_is_not_safe_only() {
    let safe_redaction = EvidenceRedactionMetadata::new(vec![workflow_core::RedactionFieldState {
        field: "target".to_owned(),
        disposition: workflow_core::RedactionDisposition::Safe,
        reason: "safe metadata".to_owned(),
    }])
    .expect("safe redaction");
    let mut fields = required_fields(
        "evidence/command-safe",
        EvidenceKind::CommandOutput,
        "Command output",
        EvidenceReferenceTarget::command_output("cargo test", "summary").expect("target"),
        EvidenceSourceComponent::Cli,
        EvidenceScope::Operator,
        Some(EvidenceSensitivity::Internal),
    );
    fields.redaction_metadata = safe_redaction;
    let reference = EvidenceReference::new(fields).expect("command output evidence");

    let error = reference.validate().expect_err("safe-only redaction fails");
    assert_eq!(
        error.code(),
        "evidence.kind.command_output_redaction_required"
    );
}

#[test]
fn title_bound_is_enforced() {
    let long_title = "a".repeat(161);
    let error = EvidenceReference::new(required_fields(
        "evidence/title",
        EvidenceKind::SpecFile,
        &long_title,
        EvidenceReferenceTarget::file("workflow-os.yml").expect("target"),
        EvidenceSourceComponent::Cli,
        EvidenceScope::Project,
        Some(EvidenceSensitivity::Internal),
    ))
    .expect_err("title too long");

    assert_eq!(error.code(), "evidence.string.too_long");
    assert!(!error.to_string().contains(&long_title));
}

#[test]
fn summary_bound_is_enforced() {
    let mut reference = base_reference(EvidenceKind::SpecFile, EvidenceScope::Project);
    let long_summary = "a".repeat(2_001);

    let error = reference
        .set_summary(&long_summary)
        .expect_err("summary too long");
    assert_eq!(error.code(), "evidence.string.too_long");
    assert!(!error.to_string().contains(&long_summary));
}

#[test]
fn metadata_bound_is_enforced() {
    let mut values = BTreeMap::new();
    values.insert("key".to_owned(), "a".repeat(257));

    let error = EvidenceMetadata::new(values).expect_err("metadata too long");
    assert_eq!(error.code(), "evidence.string.too_long");
}

#[test]
fn metadata_entry_count_bound_is_enforced() {
    let values = (0..33)
        .map(|index| (format!("key-{index}"), "value".to_owned()))
        .collect();

    let error = EvidenceMetadata::new(values).expect_err("too many metadata entries");
    assert_eq!(error.code(), "evidence.metadata.too_many_entries");
}

#[test]
fn debug_output_does_not_leak_secret_like_title_summary_metadata_or_target() {
    let mut metadata = BTreeMap::new();
    metadata.insert(
        "safe".to_owned(),
        "token=metadata-secret Authorization: Bearer hidden".to_owned(),
    );
    let mut reference = EvidenceReference::new(required_fields(
        "evidence/debug",
        EvidenceKind::ExternalReference,
        "token=title-secret",
        EvidenceReferenceTarget::uri("https://example.test/path?token=target-secret")
            .expect("target"),
        EvidenceSourceComponent::Adapter,
        EvidenceScope::External,
        Some(EvidenceSensitivity::Secret),
    ))
    .expect("evidence");
    reference
        .set_summary("Authorization: Bearer summary-secret")
        .expect("summary");
    reference.set_metadata(EvidenceMetadata::new(metadata).expect("metadata"));

    let debug = format!("{reference:?}");
    assert!(!debug.contains("title-secret"));
    assert!(!debug.contains("target-secret"));
    assert!(!debug.contains("summary-secret"));
    assert!(!debug.contains("metadata-secret"));
    assert!(debug.contains("[REDACTED]"));
}

#[test]
fn display_output_does_not_leak_secret_like_values() {
    let reference = EvidenceReference::new(required_fields(
        "evidence/display",
        EvidenceKind::ExternalReference,
        "Authorization: Bearer display-secret",
        EvidenceReferenceTarget::opaque("token=target-secret").expect("target"),
        EvidenceSourceComponent::Adapter,
        EvidenceScope::External,
        Some(EvidenceSensitivity::Secret),
    ))
    .expect("evidence");

    let display = reference.to_string();
    assert!(!display.contains("display-secret"));
    assert!(!display.contains("target-secret"));
    assert!(display.contains("[REDACTED]"));
}

#[test]
fn serialization_does_not_include_raw_secret_like_metadata_values_when_redacted() {
    let mut metadata = BTreeMap::new();
    metadata.insert(
        "source".to_owned(),
        "github_pat_secret-value private key".to_owned(),
    );
    let mut reference = base_reference(EvidenceKind::SpecFile, EvidenceScope::Project);
    reference.set_metadata(EvidenceMetadata::new(metadata).expect("metadata"));

    let json = serde_json::to_string(&reference).expect("serialize");
    assert!(!json.contains("github_pat_secret-value"));
    assert!(!json.contains("private key"));
    assert!(json.contains("[REDACTED]"));
}

#[test]
fn content_hash_and_provider_etag_are_treated_as_sensitive_where_appropriate() {
    let mut reference = base_reference(EvidenceKind::ExternalReference, EvidenceScope::External);
    reference.content_hash = Some(SpecContentHash::from_text("private artifact"));
    reference
        .set_provider_etag_or_version("etag-private-provider-version")
        .expect("etag");

    assert_eq!(reference.sensitivity, EvidenceSensitivity::Confidential);
    let debug = format!("{reference:?}");
    assert!(!debug.contains("etag-private-provider-version"));
}

#[test]
fn constructor_requires_redaction_metadata_and_sensitivity_path_defaults() {
    let empty_redaction = EvidenceRedactionMetadata::new(Vec::new())
        .expect_err("redaction metadata cannot be omitted");
    assert_eq!(empty_redaction.code(), "evidence.redaction.required");

    let reference = EvidenceReference::new(required_fields(
        "evidence/default-sensitivity",
        EvidenceKind::SpecFile,
        "Spec file",
        EvidenceReferenceTarget::file("workflow-os.yml").expect("target"),
        EvidenceSourceComponent::Cli,
        EvidenceScope::Project,
        None,
    ))
    .expect("reference");
    assert_eq!(reference.sensitivity, EvidenceSensitivity::Confidential);
}

#[test]
fn error_messages_do_not_include_secret_like_payloads() {
    let secret = "Authorization: Bearer very-secret-token";
    let error =
        EvidenceRedactionMetadata::reference_only("target", secret).expect_err("secret rejected");

    assert!(!error.to_string().contains("very-secret-token"));
    assert!(!error.to_string().contains("Authorization: Bearer"));
}

#[test]
fn deserialization_sanitizes_secret_like_fields_before_reserialization() {
    let json = r#"{
        "id": "evidence/deser",
        "kind": "external_reference",
        "title": "token=title-secret",
        "target": { "kind": "uri", "uri": "https://example.test?token=target-secret" },
        "source_component": "adapter",
        "scope": "external",
        "workflow_id": null,
        "workflow_version": null,
        "schema_version": null,
        "spec_hash": null,
        "run_id": null,
        "step_id": null,
        "skill_id": null,
        "skill_version": null,
        "adapter_id": null,
        "adapter_kind": null,
        "audit_event_id": null,
        "workflow_event_id": null,
        "approval_id": null,
        "validation_result_id": null,
        "correlation_id": null,
        "actor": null,
        "system_actor": null,
        "created_at": "2026-06-04T00:00:00Z",
        "summary": "Authorization: Bearer summary-secret",
        "content_hash": null,
        "provider_etag_or_version": "token=etag-secret",
        "redaction_metadata": {
            "metadata": {
                "redacted_fields": [],
                "field_states": [
                    {
                        "field": "target",
                        "disposition": "reference_only",
                        "reason": "reference only"
                    }
                ]
            }
        },
        "sensitivity": "secret",
        "retention_hint": null,
        "metadata": { "source": "password=metadata-secret" }
    }"#;

    let reference: EvidenceReference = serde_json::from_str(json).expect("deserialize");
    let serialized = serde_json::to_string(&reference).expect("serialize");
    assert!(!serialized.contains("title-secret"));
    assert!(!serialized.contains("target-secret"));
    assert!(!serialized.contains("summary-secret"));
    assert!(!serialized.contains("etag-secret"));
    assert!(!serialized.contains("metadata-secret"));
    assert!(serialized.contains("[REDACTED]"));
}

#[test]
fn step_and_skill_scoped_valid_evidence_validate_with_required_fields() {
    let step_reference = run_identity(base_reference(EvidenceKind::SpecFile, EvidenceScope::Step))
        .with_step_id(StepId::new("step/review").expect("step id"));
    step_reference.validate().expect("step validates");

    let skill_reference =
        run_identity(base_reference(EvidenceKind::SpecFile, EvidenceScope::Skill)).with_skill(
            SkillId::new("skill/review").expect("skill id"),
            SkillVersion::new("v1").expect("skill version"),
        );
    skill_reference.validate().expect("skill validates");
}

#[test]
fn workflow_event_kind_requires_workflow_event_id() {
    let reference = run_identity(base_reference(
        EvidenceKind::WorkflowEvent,
        EvidenceScope::Run,
    ));

    let error = reference
        .validate()
        .expect_err("workflow event id required");
    assert_eq!(error.code(), "evidence.kind.workflow_event_id_required");
}
