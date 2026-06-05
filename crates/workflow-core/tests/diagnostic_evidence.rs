#![allow(clippy::expect_used)]
//! `EvidenceReference` attachment tests for core diagnostics.

use std::collections::BTreeMap;

use serde_json::json;
use workflow_core::{
    Diagnostic, DiagnosticSeverity, EvidenceKind, EvidenceMetadata, EvidenceRedactionMetadata,
    EvidenceReference, EvidenceReferenceId, EvidenceReferenceRequiredFields,
    EvidenceReferenceTarget, EvidenceScope, EvidenceSensitivity, EvidenceSourceComponent,
    RedactionDisposition, RedactionFieldState, SourceLocation, Timestamp, ValidationReferenceId,
};

fn redaction() -> EvidenceRedactionMetadata {
    EvidenceRedactionMetadata::new(vec![RedactionFieldState {
        field: "target".to_owned(),
        disposition: RedactionDisposition::ReferenceOnly,
        reason: "stores a reference, not raw evidence".to_owned(),
    }])
    .expect("redaction metadata")
}

fn validation_evidence(id: &str) -> EvidenceReference {
    EvidenceReference::new(EvidenceReferenceRequiredFields {
        id: EvidenceReferenceId::new(id).expect("evidence id"),
        kind: EvidenceKind::ValidationResult,
        title: "Validation evidence".to_owned(),
        target: EvidenceReferenceTarget::internal("diagnostic", "validation/missing-field")
            .expect("internal target"),
        source_component: EvidenceSourceComponent::Validator,
        scope: EvidenceScope::Validation,
        created_at: Timestamp::parse_rfc3339("2026-06-04T12:00:00Z").expect("timestamp"),
        redaction_metadata: redaction(),
        sensitivity: Some(EvidenceSensitivity::Internal),
    })
    .expect("evidence reference")
    .with_validation_result_id(
        ValidationReferenceId::new("validation/missing-field").expect("validation id"),
    )
}

fn spec_file_evidence(id: &str) -> EvidenceReference {
    EvidenceReference::new(EvidenceReferenceRequiredFields {
        id: EvidenceReferenceId::new(id).expect("evidence id"),
        kind: EvidenceKind::SpecFile,
        title: "Spec file reference".to_owned(),
        target: EvidenceReferenceTarget::file("workflows/request.workflow.yml")
            .expect("file target"),
        source_component: EvidenceSourceComponent::Validator,
        scope: EvidenceScope::Project,
        created_at: Timestamp::parse_rfc3339("2026-06-04T12:00:00Z").expect("timestamp"),
        redaction_metadata: redaction(),
        sensitivity: Some(EvidenceSensitivity::Internal),
    })
    .expect("evidence reference")
}

fn diagnostic() -> Diagnostic {
    Diagnostic::new(
        DiagnosticSeverity::Error,
        "validation.workflow.step_missing",
        "workflow must declare at least one step",
    )
    .with_source_location(
        SourceLocation::new("workflows/request.workflow.yml")
            .with_line(12)
            .with_column(5)
            .with_document_path("$.steps"),
    )
}

#[test]
fn diagnostic_without_evidence_behaves_as_before() {
    let diagnostic = diagnostic();

    assert!(diagnostic.evidence_references().is_empty());
    assert_eq!(
        diagnostic.to_string(),
        "workflows/request.workflow.yml:12:5 $.steps: error[validation.workflow.step_missing]: workflow must declare at least one step"
    );
    let serialized = serde_json::to_string(&diagnostic).expect("diagnostic serializes");
    assert!(!serialized.contains("evidence_references"));
}

#[test]
fn diagnostic_attaches_one_valid_evidence_reference() {
    let mut diagnostic = diagnostic();
    let evidence = validation_evidence("evidence/validation-one");

    diagnostic
        .attach_evidence_reference(&evidence)
        .expect("valid evidence attaches");

    assert_eq!(diagnostic.evidence_references().len(), 1);
    assert_eq!(
        diagnostic.evidence_references()[0].kind,
        EvidenceKind::ValidationResult
    );
}

#[test]
fn diagnostic_attaches_multiple_valid_evidence_references_atomically() {
    let diagnostic = diagnostic()
        .with_evidence_references(vec![
            validation_evidence("evidence/validation-multiple"),
            spec_file_evidence("evidence/spec-file"),
        ])
        .expect("valid evidence attaches");

    assert_eq!(diagnostic.evidence_references().len(), 2);
    assert_eq!(
        diagnostic.evidence_references()[0].kind,
        EvidenceKind::ValidationResult
    );
    assert_eq!(
        diagnostic.evidence_references()[1].kind,
        EvidenceKind::SpecFile
    );
}

#[test]
fn diagnostic_rejects_invalid_evidence_reference() {
    let mut diagnostic = diagnostic();
    let mut evidence = validation_evidence("evidence/invalid");
    evidence.validation_result_id = None;

    let error = diagnostic
        .attach_evidence_reference(&evidence)
        .expect_err("invalid evidence rejected");

    assert_eq!(error.code(), "evidence.scope.validation_reference_required");
    assert!(diagnostic.evidence_references().is_empty());
}

#[test]
fn diagnostic_multiple_attachment_fails_atomically_when_one_reference_is_invalid() {
    let mut diagnostic = diagnostic();
    let mut invalid = validation_evidence("evidence/invalid-batch");
    invalid.validation_result_id = None;

    let error = diagnostic
        .attach_evidence_references(vec![
            validation_evidence("evidence/valid-batch"),
            invalid,
            spec_file_evidence("evidence/spec-batch"),
        ])
        .expect_err("invalid evidence rejects whole batch");

    assert_eq!(error.code(), "evidence.scope.validation_reference_required");
    assert!(diagnostic.evidence_references().is_empty());
}

#[test]
fn diagnostic_evidence_accessor_is_read_only() {
    let diagnostic = diagnostic()
        .with_evidence_references(vec![validation_evidence("evidence/accessor")])
        .expect("valid evidence attaches");

    // Rust privacy prevents callers from mutating the private collection.
    // The public accessor returns a slice, so callers cannot push directly.
    let evidence = diagnostic.evidence_references();
    assert_eq!(evidence.len(), 1);
    assert_eq!(evidence[0].kind, EvidenceKind::ValidationResult);
}

#[test]
fn valid_evidence_bearing_diagnostic_serializes_and_deserializes() {
    let diagnostic = diagnostic()
        .with_evidence_references(vec![validation_evidence("evidence/serde-valid")])
        .expect("valid evidence attaches");

    let serialized = serde_json::to_string(&diagnostic).expect("diagnostic serializes");
    assert!(serialized.contains("evidence_references"));

    let decoded: Diagnostic =
        serde_json::from_str(&serialized).expect("valid evidence-bearing diagnostic deserializes");
    assert_eq!(decoded.evidence_references().len(), 1);
    assert_eq!(
        decoded.evidence_references()[0].kind,
        EvidenceKind::ValidationResult
    );
}

#[test]
fn invalid_evidence_bearing_diagnostic_fails_deserialization_without_leaking_values() {
    let diagnostic = diagnostic()
        .with_evidence_references(vec![validation_evidence("evidence/serde-invalid")])
        .expect("valid evidence attaches");
    let mut value = serde_json::to_value(&diagnostic).expect("diagnostic serializes");

    value["evidence_references"][0]["validation_result_id"] = serde_json::Value::Null;
    value["evidence_references"][0]["title"] = json!("github_pat_secret-title");
    value["evidence_references"][0]["target"] = json!({
        "kind": "file",
        "path": "raw ci log authorization: Bearer secret"
    });

    let error = serde_json::from_value::<Diagnostic>(value)
        .expect_err("invalid evidence-bearing diagnostic is rejected");
    let error_text = error.to_string();

    assert!(error_text.contains("evidence.scope.validation_reference_required"));
    assert!(!error_text.contains("github_pat_secret-title"));
    assert!(!error_text.contains("authorization: Bearer secret"));
    assert!(!error_text.contains("raw ci log"));
}

#[test]
fn diagnostic_fields_are_preserved_after_evidence_attachment() {
    let diagnostic = diagnostic();
    let expected = diagnostic.clone();
    let diagnostic = diagnostic
        .with_evidence_references(vec![validation_evidence("evidence/preserved")])
        .expect("valid evidence attaches");

    assert_eq!(diagnostic.severity(), expected.severity());
    assert_eq!(diagnostic.code(), expected.code());
    assert_eq!(diagnostic.message(), expected.message());
    assert_eq!(diagnostic.source_location(), expected.source_location());
}

#[test]
fn source_location_remains_source_of_truth_and_is_not_copied_to_summary() {
    let diagnostic = diagnostic()
        .with_evidence_references(vec![spec_file_evidence("evidence/source-location")])
        .expect("valid evidence attaches");

    let location = diagnostic
        .source_location()
        .expect("source location remains on diagnostic");
    assert_eq!(location.line(), Some(12));
    assert_eq!(location.column(), Some(5));
    assert_eq!(location.document_path(), Some("$.steps"));
    assert!(diagnostic.evidence_references()[0].summary.is_none());
}

#[test]
fn diagnostic_accepts_validation_result_and_spec_file_evidence() {
    let diagnostic = diagnostic()
        .with_evidence_references(vec![
            validation_evidence("evidence/accept-validation"),
            spec_file_evidence("evidence/accept-spec"),
        ])
        .expect("supported evidence kinds attach");

    assert_eq!(diagnostic.evidence_references().len(), 2);
}

#[test]
fn diagnostic_rejects_command_output_evidence() {
    let mut diagnostic = diagnostic();
    let evidence = EvidenceReference::new(EvidenceReferenceRequiredFields {
        id: EvidenceReferenceId::new("evidence/command-output").expect("evidence id"),
        kind: EvidenceKind::CommandOutput,
        title: "Command summary".to_owned(),
        target: EvidenceReferenceTarget::command_output("workflow-os validate", "bounded summary")
            .expect("command output target"),
        source_component: EvidenceSourceComponent::Validator,
        scope: EvidenceScope::Project,
        created_at: Timestamp::parse_rfc3339("2026-06-04T12:00:00Z").expect("timestamp"),
        redaction_metadata: redaction(),
        sensitivity: Some(EvidenceSensitivity::Internal),
    })
    .expect("command output evidence");

    let error = diagnostic
        .attach_evidence_reference(&evidence)
        .expect_err("command output evidence rejected");

    assert_eq!(error.code(), "diagnostic.evidence.kind_unsupported");
    assert!(diagnostic.evidence_references().is_empty());
}

#[test]
fn diagnostic_rejects_adapter_evidence_kinds() {
    for kind in [
        EvidenceKind::AdapterInvocation,
        EvidenceKind::AdapterResponseSummary,
    ] {
        let mut diagnostic = diagnostic();
        let evidence = EvidenceReference::new(EvidenceReferenceRequiredFields {
            id: EvidenceReferenceId::new(format!("evidence/{kind:?}")).expect("evidence id"),
            kind,
            title: "Adapter evidence".to_owned(),
            target: EvidenceReferenceTarget::internal("adapter", "adapter/evidence")
                .expect("internal target"),
            source_component: EvidenceSourceComponent::Adapter,
            scope: EvidenceScope::Project,
            created_at: Timestamp::parse_rfc3339("2026-06-04T12:00:00Z").expect("timestamp"),
            redaction_metadata: redaction(),
            sensitivity: Some(EvidenceSensitivity::Internal),
        })
        .expect("adapter evidence");

        let error = diagnostic
            .attach_evidence_reference(&evidence)
            .expect_err("adapter evidence kind rejected");

        assert_eq!(error.code(), "diagnostic.evidence.kind_unsupported");
        assert!(diagnostic.evidence_references().is_empty());
    }
}

#[test]
fn diagnostic_rejects_other_future_evidence_kinds() {
    for kind in [
        EvidenceKind::ApprovalDecision,
        EvidenceKind::PolicyDecision,
        EvidenceKind::ExternalReference,
        EvidenceKind::ReleaseReview,
        EvidenceKind::LiveSmokeEvidence,
    ] {
        let mut diagnostic = diagnostic();
        let evidence = EvidenceReference::new(EvidenceReferenceRequiredFields {
            id: EvidenceReferenceId::new(format!("evidence/{kind:?}")).expect("evidence id"),
            kind,
            title: "Future evidence".to_owned(),
            target: EvidenceReferenceTarget::internal("future", "future/evidence")
                .expect("internal target"),
            source_component: EvidenceSourceComponent::Validator,
            scope: EvidenceScope::Project,
            created_at: Timestamp::parse_rfc3339("2026-06-04T12:00:00Z").expect("timestamp"),
            redaction_metadata: redaction(),
            sensitivity: Some(EvidenceSensitivity::Internal),
        })
        .expect("future evidence");

        let error = diagnostic
            .attach_evidence_reference(&evidence)
            .expect_err("future evidence kind rejected");

        assert_eq!(error.code(), "diagnostic.evidence.kind_unsupported");
        assert!(diagnostic.evidence_references().is_empty());
    }
}

#[test]
fn diagnostic_rejects_unsupported_evidence_scope() {
    let mut diagnostic = diagnostic();
    let evidence = EvidenceReference::new(EvidenceReferenceRequiredFields {
        id: EvidenceReferenceId::new("evidence/release").expect("evidence id"),
        kind: EvidenceKind::SpecFile,
        title: "Release evidence".to_owned(),
        target: EvidenceReferenceTarget::file("docs/release/review.md").expect("file target"),
        source_component: EvidenceSourceComponent::ReleaseReview,
        scope: EvidenceScope::Release,
        created_at: Timestamp::parse_rfc3339("2026-06-04T12:00:00Z").expect("timestamp"),
        redaction_metadata: redaction(),
        sensitivity: Some(EvidenceSensitivity::Internal),
    })
    .expect("release evidence");

    let error = diagnostic
        .attach_evidence_reference(&evidence)
        .expect_err("release evidence scope rejected");

    assert_eq!(error.code(), "diagnostic.evidence.scope_unsupported");
    assert!(diagnostic.evidence_references().is_empty());
}

#[test]
fn diagnostic_evidence_debug_and_serialization_do_not_leak_secret_like_values() {
    let mut metadata = BTreeMap::new();
    metadata.insert(
        "note".to_owned(),
        "jira description: customer secret".to_owned(),
    );
    let mut evidence = validation_evidence("evidence/secret-safe");
    evidence.title = "github_pat_secret-title".to_owned();
    evidence.target =
        EvidenceReferenceTarget::file("raw ci log authorization: Bearer secret").expect("target");
    evidence
        .set_summary("jira comment: private key and token=secret")
        .expect("summary");
    evidence.set_metadata(EvidenceMetadata::new(metadata).expect("metadata"));

    let diagnostic = diagnostic()
        .with_evidence_references(vec![evidence])
        .expect("secret-like evidence sanitizes and attaches");

    let debug = format!("{diagnostic:?}");
    let serialized = serde_json::to_string(&diagnostic).expect("diagnostic serializes");

    for output in [debug, serialized] {
        assert!(!output.contains("github_pat_secret-title"));
        assert!(!output.contains("authorization: Bearer secret"));
        assert!(!output.contains("jira comment:"));
        assert!(!output.contains("private key"));
        assert!(!output.contains("token=secret"));
        assert!(!output.contains("jira description:"));
        assert!(output.contains("[REDACTED]"));
    }
}
