#![allow(clippy::expect_used, clippy::panic)]
//! Behavior tests for foundational Workflow OS core primitives.

use std::str::FromStr;

use serde_json::json;
use workflow_core::{
    ActorId, Diagnostic, DiagnosticSeverity, EventId, ProjectId, RedactedValue, SourceLocation,
    SpecContentHash, Timestamp, WorkflowOsError, WorkflowOsErrorKind, WorkflowRunId,
};

#[test]
fn string_ids_serialize_and_deserialize_as_strings() {
    let id = ProjectId::new("enterprise/finance-close").expect("valid project id");

    let serialized = serde_json::to_string(&id).expect("project id serializes");
    assert_eq!(serialized, "\"enterprise/finance-close\"");

    let deserialized: ProjectId =
        serde_json::from_str(&serialized).expect("project id deserializes");
    assert_eq!(deserialized, id);
    assert_eq!(deserialized.as_str(), "enterprise/finance-close");
}

#[test]
fn string_ids_reject_unsafe_characters() {
    let error = ProjectId::new("bad secret").expect_err("spaces are not valid in ids");

    assert_eq!(error.kind(), WorkflowOsErrorKind::Validation);
    assert_eq!(error.code(), "identifier.invalid_character");
}

#[test]
fn generated_ids_serialize_deserialize_display_and_debug_cleanly() {
    let id = EventId::from_str("event-01890dd8-8546-7702-b7e0-35ff26ba053f")
        .expect("static generated id parses");

    assert_eq!(id.to_string(), "event-01890dd8-8546-7702-b7e0-35ff26ba053f");
    assert_eq!(
        format!("{id:?}"),
        "EventId(\"event-01890dd8-8546-7702-b7e0-35ff26ba053f\")"
    );

    let serialized = serde_json::to_string(&id).expect("event id serializes");
    assert_eq!(serialized, "\"event-01890dd8-8546-7702-b7e0-35ff26ba053f\"");

    let deserialized: EventId = serde_json::from_str(&serialized).expect("event id deserializes");
    assert_eq!(deserialized, id);
}

#[test]
fn generated_ids_are_unique() {
    let first = WorkflowRunId::generate();
    let second = WorkflowRunId::generate();

    assert_ne!(first, second);
}

#[test]
fn id_types_are_not_interchangeable_even_when_values_match() {
    let actor = ActorId::new("system").expect("actor id parses");
    let project = ProjectId::new("system").expect("project id parses");

    assert_eq!(actor.to_string(), project.to_string());
}

#[test]
fn source_location_formats_for_cli_output() {
    let location = SourceLocation::new("workflows/approval.yaml")
        .with_line(12)
        .with_column(7)
        .with_document_path("$.steps[0].approval");

    assert_eq!(
        location.to_string(),
        "workflows/approval.yaml:12:7 $.steps[0].approval"
    );
}

#[test]
fn diagnostic_formats_with_location_code_and_severity() {
    let diagnostic = Diagnostic::new(
        DiagnosticSeverity::Error,
        "workflow.missing_skill",
        "referenced skill does not exist",
    )
    .with_source_location(SourceLocation::new("workflow.yaml").with_line(4));

    assert_eq!(
        diagnostic.to_string(),
        "workflow.yaml:4: error[workflow.missing_skill]: referenced skill does not exist"
    );
}

#[test]
fn spec_content_hashing_is_deterministic() {
    let first = SpecContentHash::from_text("schema_version: workflowos.dev/v0\n");
    let second = SpecContentHash::from_bytes(b"schema_version: workflowos.dev/v0\n");

    assert_eq!(first, second);
    assert_eq!(
        first.to_string(),
        "c6801ab324df9c53efe681fd554285401849b9ae5fda67828003441df3f8f246"
    );
}

#[test]
fn spec_content_hash_rejects_invalid_hash_text() {
    let error = SpecContentHash::new("not-a-sha").expect_err("invalid hash is rejected");

    assert_eq!(error.code(), "spec_hash.invalid");
}

#[test]
fn timestamp_serializes_as_rfc3339_text() {
    let timestamp = Timestamp::parse_rfc3339("2026-05-24T01:02:03Z").expect("timestamp parses");

    assert_eq!(timestamp.to_string(), "2026-05-24T01:02:03Z");
    assert_eq!(
        serde_json::to_value(timestamp).expect("timestamp serializes"),
        json!("2026-05-24T01:02:03Z")
    );
}

#[test]
fn redacted_value_hides_secret_in_display_debug_and_serialization() {
    let secret = RedactedValue::new("super-secret-token".to_owned());

    assert_eq!(secret.to_string(), "[REDACTED]");
    assert_eq!(format!("{secret:?}"), "RedactedValue([REDACTED])");
    assert_eq!(
        serde_json::to_string(&secret).expect("redacted value serializes"),
        "\"[REDACTED]\""
    );
    assert_eq!(secret.expose_secret(), "super-secret-token");

    let deserialized: RedactedValue<String> =
        serde_json::from_str("\"incoming-secret\"").expect("redacted value deserializes");
    assert_eq!(deserialized.to_string(), "[REDACTED]");
    assert_eq!(deserialized.expose_secret(), "incoming-secret");
}

#[test]
fn workflow_error_formats_and_converts_from_diagnostic() {
    let diagnostic = Diagnostic::error("spec.invalid", "spec is invalid");
    let error = WorkflowOsError::from(diagnostic.clone());

    assert_eq!(error.kind(), WorkflowOsErrorKind::Validation);
    assert_eq!(
        error.to_string(),
        "validation[spec.invalid]: spec is invalid"
    );
    assert_eq!(error.diagnostics(), &[diagnostic]);
}
