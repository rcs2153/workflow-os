#![allow(clippy::expect_used)]

//! Workflow catalog and stewardship core model tests.

use serde_json::json;
use workflow_core::{
    ActorId, ApprovalReferenceId, EventId, EvidenceReferenceId, RedactionDisposition,
    RedactionFieldState, RedactionMetadata, SchemaVersion, SpecContentHash, Timestamp,
    ValidationReferenceId, WorkReportId, WorkReportSensitivity, WorkflowArchiveRecord,
    WorkflowArchiveRecordDefinition, WorkflowArchiveRecordId, WorkflowCatalogRecord,
    WorkflowCatalogRecordDefinition, WorkflowCatalogRecordId, WorkflowId, WorkflowLifecycleStatus,
    WorkflowOsErrorKind, WorkflowStewardshipDecisionId, WorkflowStewardshipDecisionKind,
    WorkflowStewardshipRecord, WorkflowStewardshipRecordDefinition,
};

fn catalog_id() -> WorkflowCatalogRecordId {
    WorkflowCatalogRecordId::new("catalog/workflow/pr-review").expect("valid catalog id")
}

fn stewardship_id() -> WorkflowStewardshipDecisionId {
    WorkflowStewardshipDecisionId::new("stewardship/pr-review/approved")
        .expect("valid stewardship id")
}

fn archive_id() -> WorkflowArchiveRecordId {
    WorkflowArchiveRecordId::new("archive/pr-review/draft-1").expect("valid archive id")
}

fn workflow_id() -> WorkflowId {
    WorkflowId::new("local/pr-review").expect("valid workflow id")
}

fn schema_version() -> SchemaVersion {
    SchemaVersion::new("workflowos.dev/v0").expect("valid schema version")
}

fn content_hash(text: &str) -> SpecContentHash {
    SpecContentHash::from_text(text)
}

fn actor() -> ActorId {
    ActorId::new("user/steward").expect("valid actor")
}

fn timestamp() -> Timestamp {
    Timestamp::parse_rfc3339("2026-07-08T00:00:00Z").expect("valid timestamp")
}

fn redaction() -> RedactionMetadata {
    RedactionMetadata {
        redacted_fields: vec!["reason_summary".to_owned()],
        field_states: vec![RedactionFieldState {
            field: "reason_summary".to_owned(),
            disposition: RedactionDisposition::ReferenceOnly,
            reason: "bounded stewardship summary stored".to_owned(),
        }],
    }
}

fn catalog_definition() -> WorkflowCatalogRecordDefinition {
    WorkflowCatalogRecordDefinition {
        record_id: catalog_id(),
        workflow_id: workflow_id(),
        workflow_path: "workflows/pr-review.workflow.yml".to_owned(),
        workflow_content_hash: content_hash("active workflow"),
        schema_version: schema_version(),
        lifecycle_status: WorkflowLifecycleStatus::Active,
        source_recommendation_id: Some("first_run.pr_review".to_owned()),
        source_draft_path: Some("workflows/drafts/pr-review.workflow.yml".to_owned()),
        archived_draft_path: Some("workflows/drafts/archive/pr-review.workflow.yml".to_owned()),
        owner: Some(actor()),
        escalation_contact: Some(ActorId::new("user/escalation").expect("valid actor")),
        authority_scope: Some("governs pull request review workflow".to_owned()),
        evidence_check_report_posture: Some("requires validation evidence and handoff".to_owned()),
        side_effect_posture: Some("none_skipped_unsupported".to_owned()),
        latest_stewardship_decision_id: Some(stewardship_id()),
        latest_promotion_decision_id: Some(stewardship_id()),
        latest_archive_record_id: Some(archive_id()),
        created_at: timestamp(),
        updated_at: timestamp(),
        sensitivity: WorkReportSensitivity::Confidential,
        redaction: redaction(),
    }
}

fn stewardship_definition() -> WorkflowStewardshipRecordDefinition {
    WorkflowStewardshipRecordDefinition {
        decision_id: stewardship_id(),
        decision_kind: WorkflowStewardshipDecisionKind::ApprovedForPromotion,
        workflow_id: workflow_id(),
        draft_path: Some("workflows/drafts/pr-review.workflow.yml".to_owned()),
        active_workflow_path: Some("workflows/pr-review.workflow.yml".to_owned()),
        archive_path: Some("workflows/drafts/archive/pr-review.workflow.yml".to_owned()),
        candidate_content_hash: content_hash("draft workflow"),
        active_content_hash: Some(content_hash("active workflow")),
        reviewer: actor(),
        decided_at: timestamp(),
        reason_summary: Some("bounded steward approval summary".to_owned()),
        preflight_reference: Some(
            ValidationReferenceId::new("validation/preflight").expect("valid validation ref"),
        ),
        steward_review_reference: Some(
            ValidationReferenceId::new("validation/steward-review").expect("valid validation ref"),
        ),
        evidence_references: vec![
            EvidenceReferenceId::new("evidence/preflight").expect("valid evidence")
        ],
        approval_references: vec![
            ApprovalReferenceId::new("approval/steward").expect("valid approval")
        ],
        policy_decision_references: vec![EventId::new("event/policy-1").expect("valid event")],
        validation_references: vec![
            ValidationReferenceId::new("validation/project").expect("valid validation")
        ],
        work_report_references: vec![
            WorkReportId::new("work-report/authoring").expect("valid report")
        ],
        known_limitations: vec!["catalog persistence is not implemented".to_owned()],
        strict_non_goals: vec!["does not authorize provider writes".to_owned()],
        sensitivity: WorkReportSensitivity::Confidential,
        redaction: redaction(),
    }
}

fn archive_definition() -> WorkflowArchiveRecordDefinition {
    WorkflowArchiveRecordDefinition {
        archive_record_id: archive_id(),
        original_draft_path: "workflows/drafts/pr-review.workflow.yml".to_owned(),
        archive_path: "workflows/drafts/archive/pr-review.workflow.yml".to_owned(),
        workflow_id: workflow_id(),
        draft_content_hash: content_hash("draft workflow"),
        active_workflow_path: Some("workflows/pr-review.workflow.yml".to_owned()),
        active_workflow_content_hash: Some(content_hash("active workflow")),
        prior_draft_status: "promoted_preserved".to_owned(),
        archive_actor: actor(),
        archive_reason_summary: Some("archived after promotion".to_owned()),
        archived_at: timestamp(),
        validation_reference: Some(
            ValidationReferenceId::new("validation/archive").expect("valid validation"),
        ),
        stewardship_decision_id: Some(stewardship_id()),
        sensitivity: WorkReportSensitivity::Confidential,
        redaction: redaction(),
    }
}

#[test]
fn valid_catalog_record_preserves_identity() {
    let record = WorkflowCatalogRecord::new(catalog_definition()).expect("valid catalog record");

    assert_eq!(record.record_id().as_str(), "catalog/workflow/pr-review");
    assert_eq!(record.workflow_id().as_str(), "local/pr-review");
    assert_eq!(record.workflow_path(), "workflows/pr-review.workflow.yml");
    assert_eq!(record.lifecycle_status(), WorkflowLifecycleStatus::Active);
    assert_eq!(record.sensitivity(), WorkReportSensitivity::Confidential);
    record.validate().expect("record remains valid");
}

#[test]
fn invalid_catalog_id_is_rejected_without_leaking_value() {
    let error = WorkflowCatalogRecordId::new("catalog token value").expect_err("invalid id");

    assert_eq!(error.kind(), WorkflowOsErrorKind::Validation);
    assert_eq!(
        error.code(),
        "workflow_catalog.identifier.invalid_character"
    );
    assert!(!error.to_string().contains("token"));
}

#[test]
fn unsafe_catalog_paths_are_rejected() {
    let mut definition = catalog_definition();
    definition.workflow_path = "/private/tmp/workflow.yml".to_owned();

    let error = WorkflowCatalogRecord::new(definition).expect_err("absolute path rejected");
    assert_eq!(error.code(), "workflow_catalog.path.absolute");

    let mut definition = catalog_definition();
    definition.source_draft_path = Some("../draft.workflow.yml".to_owned());

    let error = WorkflowCatalogRecord::new(definition).expect_err("traversal rejected");
    assert_eq!(error.code(), "workflow_catalog.path.unsafe");
}

#[test]
fn lifecycle_status_vocabulary_is_representable() {
    let statuses = [
        WorkflowLifecycleStatus::Draft,
        WorkflowLifecycleStatus::ReviewPending,
        WorkflowLifecycleStatus::Approved,
        WorkflowLifecycleStatus::Active,
        WorkflowLifecycleStatus::Superseded,
        WorkflowLifecycleStatus::Archived,
        WorkflowLifecycleStatus::Deprecated,
        WorkflowLifecycleStatus::Rejected,
    ];

    assert_eq!(statuses.len(), 8);
}

#[test]
fn stewardship_record_carries_references_without_payloads() {
    let record =
        WorkflowStewardshipRecord::new(stewardship_definition()).expect("valid stewardship");

    assert_eq!(
        record.decision_id().as_str(),
        "stewardship/pr-review/approved"
    );
    assert_eq!(
        record.decision_kind(),
        WorkflowStewardshipDecisionKind::ApprovedForPromotion
    );
    assert_eq!(record.workflow_id().as_str(), "local/pr-review");
    assert_eq!(record.sensitivity(), WorkReportSensitivity::Confidential);
}

#[test]
fn stewardship_secret_like_reason_is_rejected_without_leakage() {
    let mut definition = stewardship_definition();
    definition.reason_summary = Some("bearer token should never be here".to_owned());

    let error = WorkflowStewardshipRecord::new(definition).expect_err("secret-like reason");

    assert_eq!(error.kind(), WorkflowOsErrorKind::Validation);
    assert_eq!(error.code(), "workflow_catalog.secret_like_value");
    assert!(!error.to_string().contains("bearer"));
    assert!(!error.to_string().contains("token"));
}

#[test]
fn archive_record_links_draft_and_archive_paths() {
    let record = WorkflowArchiveRecord::new(archive_definition()).expect("valid archive record");

    assert_eq!(
        record.archive_record_id().as_str(),
        "archive/pr-review/draft-1"
    );
    assert_eq!(
        record.original_draft_path(),
        "workflows/drafts/pr-review.workflow.yml"
    );
    assert_eq!(
        record.archive_path(),
        "workflows/drafts/archive/pr-review.workflow.yml"
    );
}

#[test]
fn serde_round_trip_valid_catalog_record() {
    let record = WorkflowCatalogRecord::new(catalog_definition()).expect("valid catalog record");

    let serialized = serde_json::to_string(&record).expect("serialize");
    let deserialized: WorkflowCatalogRecord =
        serde_json::from_str(&serialized).expect("deserialize");

    assert_eq!(deserialized, record);
}

#[test]
fn invalid_serialized_catalog_record_fails_closed() {
    let serialized = json!({
        "record_id": "catalog/workflow/pr-review",
        "workflow_id": "local/pr-review",
        "workflow_path": "../workflow.yml",
        "workflow_content_hash": content_hash("active workflow").as_str(),
        "schema_version": "workflowos.dev/v0",
        "lifecycle_status": "active",
        "source_recommendation_id": null,
        "source_draft_path": null,
        "archived_draft_path": null,
        "owner": null,
        "escalation_contact": null,
        "authority_scope": null,
        "evidence_check_report_posture": null,
        "side_effect_posture": null,
        "latest_stewardship_decision_id": null,
        "latest_promotion_decision_id": null,
        "latest_archive_record_id": null,
        "created_at": "2026-07-08T00:00:00Z",
        "updated_at": "2026-07-08T00:00:00Z",
        "sensitivity": "confidential",
        "redaction": {
            "redacted_fields": [],
            "field_states": []
        }
    })
    .to_string();

    let error = serde_json::from_str::<WorkflowCatalogRecord>(&serialized)
        .expect_err("invalid path fails closed");

    assert!(!error.to_string().contains("../workflow.yml"));
}

#[test]
fn debug_output_redacts_sensitive_summaries() {
    let record =
        WorkflowStewardshipRecord::new(stewardship_definition()).expect("valid stewardship");

    let debug = format!("{record:?}");

    assert!(debug.contains("reason_summary"));
    assert!(debug.contains("[REDACTED]"));
    assert!(!debug.contains("bounded steward approval summary"));
    assert!(!debug.contains("catalog persistence is not implemented"));
    assert!(!debug.contains("does not authorize provider writes"));
}

#[test]
fn serialized_records_do_not_silently_carry_secret_like_reasons() {
    let serialized = json!({
        "decision_id": "stewardship/pr-review/approved",
        "decision_kind": "approved_for_promotion",
        "workflow_id": "local/pr-review",
        "draft_path": "workflows/drafts/pr-review.workflow.yml",
        "active_workflow_path": "workflows/pr-review.workflow.yml",
        "archive_path": null,
        "candidate_content_hash": content_hash("draft workflow").as_str(),
        "active_content_hash": null,
        "reviewer": "user/steward",
        "decided_at": "2026-07-08T00:00:00Z",
        "reason_summary": "private_key should not deserialize",
        "preflight_reference": null,
        "steward_review_reference": null,
        "evidence_references": [],
        "approval_references": [],
        "policy_decision_references": [],
        "validation_references": [],
        "work_report_references": [],
        "known_limitations": [],
        "strict_non_goals": [],
        "sensitivity": "confidential",
        "redaction": {
            "redacted_fields": [],
            "field_states": []
        }
    })
    .to_string();

    let error = serde_json::from_str::<WorkflowStewardshipRecord>(&serialized)
        .expect_err("secret-like serialized reason fails");

    assert!(!error.to_string().contains("private_key"));
}

#[test]
fn redaction_metadata_is_validated_and_debug_safe() {
    let mut definition = archive_definition();
    definition.redaction = RedactionMetadata {
        redacted_fields: vec!["api_token_field".to_owned()],
        field_states: vec![RedactionFieldState {
            field: "api_token_field".to_owned(),
            disposition: RedactionDisposition::Redacted,
            reason: "token reason".to_owned(),
        }],
    };

    let error = WorkflowArchiveRecord::new(definition).expect_err("secret-like redaction");

    assert_eq!(error.code(), "workflow_catalog.secret_like_value");
    assert!(!error.to_string().contains("api_token_field"));
    assert!(!error.to_string().contains("token reason"));
}
