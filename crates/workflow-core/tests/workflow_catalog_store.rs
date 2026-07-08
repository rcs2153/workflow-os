#![allow(clippy::expect_used)]

//! Local workflow catalog store helper tests.

use std::fs;
use std::path::PathBuf;

use serde::Serialize;
use serde_json::json;
use workflow_core::{
    build_workflow_catalog_index, propose_workflow_catalog_repairs,
    review_workflow_catalog_repair_proposal, ActorId, ApprovalReferenceId, EventId,
    EvidenceReferenceId, LocalWorkflowCatalogStore, RedactionDisposition, RedactionFieldState,
    RedactionMetadata, SchemaVersion, SpecContentHash, Timestamp, ValidationReferenceId,
    WorkReportId, WorkReportSensitivity, WorkflowArchiveRecord, WorkflowArchiveRecordDefinition,
    WorkflowArchiveRecordId, WorkflowCatalogActiveWorkflowSummary, WorkflowCatalogIndexInput,
    WorkflowCatalogRecord, WorkflowCatalogRecordDefinition, WorkflowCatalogRecordId,
    WorkflowCatalogRepairProposal, WorkflowCatalogRepairProposalDecisionKind,
    WorkflowCatalogRepairProposalReview, WorkflowCatalogRepairProposalReviewId,
    WorkflowCatalogRepairProposalReviewInput, WorkflowId, WorkflowLifecycleStatus,
    WorkflowOsErrorKind, WorkflowStewardshipDecisionId, WorkflowStewardshipDecisionKind,
    WorkflowStewardshipRecord, WorkflowStewardshipRecordDefinition,
};

fn temp_catalog_root(test_name: &str) -> PathBuf {
    let mut root = std::env::temp_dir();
    root.push(format!(
        "workflow-os-catalog-store-{test_name}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    root
}

fn encoded_id_file_name(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len() * 2 + 5);
    for byte in value.as_bytes() {
        use std::fmt::Write as _;
        let _ = write!(&mut encoded, "{byte:02x}");
    }
    encoded.push_str(".json");
    encoded
}

fn catalog_id(value: &str) -> WorkflowCatalogRecordId {
    WorkflowCatalogRecordId::new(value).expect("valid catalog id")
}

fn stewardship_id(value: &str) -> WorkflowStewardshipDecisionId {
    WorkflowStewardshipDecisionId::new(value).expect("valid stewardship id")
}

fn archive_id(value: &str) -> WorkflowArchiveRecordId {
    WorkflowArchiveRecordId::new(value).expect("valid archive id")
}

fn repair_review_id(value: &str) -> WorkflowCatalogRepairProposalReviewId {
    WorkflowCatalogRepairProposalReviewId::new(value).expect("valid repair review id")
}

fn workflow_id(value: &str) -> WorkflowId {
    WorkflowId::new(value).expect("valid workflow id")
}

fn schema_version() -> SchemaVersion {
    SchemaVersion::new("workflowos.dev/v0").expect("valid schema version")
}

fn content_hash(text: &str) -> SpecContentHash {
    SpecContentHash::from_text(text)
}

fn actor(value: &str) -> ActorId {
    ActorId::new(value).expect("valid actor")
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
            reason: "bounded catalog metadata stored".to_owned(),
        }],
    }
}

fn catalog_record(id: &str, workflow: &str) -> WorkflowCatalogRecord {
    WorkflowCatalogRecord::new(WorkflowCatalogRecordDefinition {
        record_id: catalog_id(id),
        workflow_id: workflow_id(workflow),
        workflow_path: format!("workflows/{workflow}.workflow.yml"),
        workflow_content_hash: content_hash(workflow),
        schema_version: schema_version(),
        lifecycle_status: WorkflowLifecycleStatus::Active,
        source_recommendation_id: Some("first_run.pr_review".to_owned()),
        source_draft_path: Some(format!("workflows/drafts/{workflow}.workflow.yml")),
        archived_draft_path: None,
        owner: Some(actor("user/steward")),
        escalation_contact: Some(actor("user/escalation")),
        authority_scope: Some("governs bounded workflow authoring".to_owned()),
        evidence_check_report_posture: Some("requires validation evidence".to_owned()),
        side_effect_posture: Some("none_skipped_unsupported".to_owned()),
        latest_stewardship_decision_id: Some(stewardship_id("stewardship/pr-review/approved")),
        latest_promotion_decision_id: None,
        latest_archive_record_id: Some(archive_id("archive/pr-review/draft-1")),
        created_at: timestamp(),
        updated_at: timestamp(),
        sensitivity: WorkReportSensitivity::Confidential,
        redaction: redaction(),
    })
    .expect("valid catalog record")
}

fn stewardship_record(id: &str, workflow: &str) -> WorkflowStewardshipRecord {
    WorkflowStewardshipRecord::new(WorkflowStewardshipRecordDefinition {
        decision_id: stewardship_id(id),
        decision_kind: WorkflowStewardshipDecisionKind::ApprovedForPromotion,
        workflow_id: workflow_id(workflow),
        draft_path: Some(format!("workflows/drafts/{workflow}.workflow.yml")),
        active_workflow_path: Some(format!("workflows/{workflow}.workflow.yml")),
        archive_path: None,
        candidate_content_hash: content_hash("draft workflow"),
        active_content_hash: Some(content_hash("active workflow")),
        reviewer: actor("user/steward"),
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
        known_limitations: vec!["command integration is deferred".to_owned()],
        strict_non_goals: vec!["does not authorize provider writes".to_owned()],
        sensitivity: WorkReportSensitivity::Confidential,
        redaction: redaction(),
    })
    .expect("valid stewardship record")
}

fn archive_record(id: &str, workflow: &str) -> WorkflowArchiveRecord {
    WorkflowArchiveRecord::new(WorkflowArchiveRecordDefinition {
        archive_record_id: archive_id(id),
        original_draft_path: format!("workflows/drafts/{workflow}.workflow.yml"),
        archive_path: format!("workflows/drafts/archive/{workflow}.workflow.yml"),
        workflow_id: workflow_id(workflow),
        draft_content_hash: content_hash("draft workflow"),
        active_workflow_path: Some(format!("workflows/{workflow}.workflow.yml")),
        active_workflow_content_hash: Some(content_hash("active workflow")),
        prior_draft_status: "promoted_preserved".to_owned(),
        archive_actor: actor("user/steward"),
        archive_reason_summary: Some("archived after promotion".to_owned()),
        archived_at: timestamp(),
        validation_reference: Some(
            ValidationReferenceId::new("validation/archive").expect("valid validation"),
        ),
        stewardship_decision_id: Some(stewardship_id("stewardship/pr-review/approved")),
        sensitivity: WorkReportSensitivity::Confidential,
        redaction: redaction(),
    })
    .expect("valid archive record")
}

fn repair_proposal() -> WorkflowCatalogRepairProposal {
    let active = WorkflowCatalogActiveWorkflowSummary::new(
        workflow_id("local/pr-review"),
        "workflows/pr-review.workflow.yml",
        content_hash("active workflow"),
        schema_version(),
    )
    .expect("valid active summary");
    let index = build_workflow_catalog_index(
        WorkflowCatalogIndexInput::new().with_active_workflows(vec![active]),
    )
    .expect("index builds");

    propose_workflow_catalog_repairs(&index)
        .expect("proposals build")
        .remove(0)
}

fn stale_repair_proposal() -> WorkflowCatalogRepairProposal {
    let active = WorkflowCatalogActiveWorkflowSummary::new(
        workflow_id("local/pr-review"),
        "workflows/renamed-pr-review.workflow.yml",
        content_hash("active workflow"),
        schema_version(),
    )
    .expect("valid active summary");
    let index = build_workflow_catalog_index(
        WorkflowCatalogIndexInput::new().with_active_workflows(vec![active]),
    )
    .expect("index builds");

    propose_workflow_catalog_repairs(&index)
        .expect("proposals build")
        .remove(0)
}

fn repair_review(
    id: &str,
    proposal: &WorkflowCatalogRepairProposal,
) -> WorkflowCatalogRepairProposalReview {
    review_workflow_catalog_repair_proposal(WorkflowCatalogRepairProposalReviewInput {
        review_id: repair_review_id(id),
        proposal,
        reviewer: actor("user/catalog-steward"),
        reason: "bounded maintainer review for catalog repair planning".to_owned(),
        decision_kind: WorkflowCatalogRepairProposalDecisionKind::ApprovedForFutureApplyPlanning,
        reviewed_at: timestamp(),
        approval_references: vec![
            ApprovalReferenceId::new("approval/catalog-repair").expect("valid approval")
        ],
        policy_decision_references: vec![
            EventId::new("event/catalog-repair-policy").expect("valid event")
        ],
        evidence_references: vec![
            EvidenceReferenceId::new("evidence/catalog-repair").expect("valid evidence")
        ],
        validation_references: vec![
            ValidationReferenceId::new("validation/catalog-repair").expect("valid validation")
        ],
        work_report_references: vec![
            WorkReportId::new("work-report/catalog-repair").expect("valid report")
        ],
        sensitivity: WorkReportSensitivity::Confidential,
        redaction: redaction(),
    })
    .expect("valid repair review")
}

fn write_json_file<T: Serialize>(path: PathBuf, value: &T) {
    fs::create_dir_all(path.parent().expect("parent")).expect("create parent");
    let bytes = serde_json::to_vec_pretty(value).expect("serialize");
    fs::write(path, bytes).expect("write json");
}

#[test]
fn writes_and_reads_each_catalog_record_kind() {
    let root = temp_catalog_root("round-trip");
    let store = LocalWorkflowCatalogStore::new(&root);

    let catalog = catalog_record("catalog/workflow/pr-review", "local/pr-review");
    let stewardship = stewardship_record("stewardship/pr-review/approved", "local/pr-review");
    let archive = archive_record("archive/pr-review/draft-1", "local/pr-review");

    store
        .write_catalog_record_if_absent(&catalog)
        .expect("write catalog");
    store
        .write_stewardship_record_if_absent(&stewardship)
        .expect("write stewardship");
    store
        .write_archive_record_if_absent(&archive)
        .expect("write archive");

    assert_eq!(
        store
            .read_catalog_record(catalog.record_id())
            .expect("read catalog"),
        catalog
    );
    assert_eq!(
        store
            .read_stewardship_record(stewardship.decision_id())
            .expect("read stewardship"),
        stewardship
    );
    assert_eq!(
        store
            .read_archive_record(archive.archive_record_id())
            .expect("read archive"),
        archive
    );

    let health = store.health_check().expect("health");
    assert!(health.root_exists());
    assert_eq!(health.catalog_records(), 1);
    assert_eq!(health.stewardship_records(), 1);
    assert_eq!(health.archive_records(), 1);
    assert_eq!(health.repair_review_records(), 0);
}

#[test]
fn writes_reads_and_lists_repair_review_records() {
    let root = temp_catalog_root("repair-review-round-trip");
    let store = LocalWorkflowCatalogStore::new(&root);
    let proposal = repair_proposal();
    let later = repair_review("catalog-repair/review-0002", &proposal);
    let earlier = repair_review("catalog-repair/review-0001", &proposal);

    store
        .write_repair_review_record_if_absent(&later, &proposal)
        .expect("write later repair review");
    store
        .write_repair_review_record_if_absent(&earlier, &proposal)
        .expect("write earlier repair review");

    assert_eq!(
        store
            .read_repair_review_record(earlier.review_id())
            .expect("read repair review"),
        earlier
    );

    let records = store
        .list_repair_review_records()
        .expect("list repair reviews");
    assert_eq!(records, vec![earlier, later]);

    let health = store.health_check().expect("health");
    assert_eq!(health.catalog_records(), 0);
    assert_eq!(health.stewardship_records(), 0);
    assert_eq!(health.archive_records(), 0);
    assert_eq!(health.repair_review_records(), 2);
}

#[test]
fn repair_review_records_use_separate_safe_sidecar_directory() {
    let root = temp_catalog_root("repair-review-sidecar");
    let store = LocalWorkflowCatalogStore::new(&root);
    let proposal = repair_proposal();
    let review = repair_review("catalog-repair/review-sidecar", &proposal);

    store
        .write_repair_review_record_if_absent(&review, &proposal)
        .expect("write repair review");

    let expected_file = root
        .join("repair-reviews")
        .join(encoded_id_file_name("catalog-repair/review-sidecar"));
    assert!(expected_file.exists());
    assert!(!root.join("workflows").exists());
    assert!(!root.join("stewardship").exists());
    assert!(!root.join("archives").exists());
}

#[test]
fn duplicate_repair_review_write_is_rejected_without_overwrite() {
    let store = LocalWorkflowCatalogStore::new(temp_catalog_root("repair-review-duplicate"));
    let proposal = repair_proposal();
    let review = repair_review("catalog-repair/review-duplicate", &proposal);

    store
        .write_repair_review_record_if_absent(&review, &proposal)
        .expect("first write");
    let error = store
        .write_repair_review_record_if_absent(&review, &proposal)
        .expect_err("duplicate rejected");

    assert_eq!(error.kind(), WorkflowOsErrorKind::InvalidState);
    assert_eq!(
        error.code(),
        "workflow_catalog.repair_review_store.duplicate_review"
    );
    assert!(!error.to_string().contains("review-duplicate"));
}

#[test]
fn stale_repair_review_is_rejected_before_persistence() {
    let root = temp_catalog_root("repair-review-stale");
    let store = LocalWorkflowCatalogStore::new(&root);
    let proposal = repair_proposal();
    let stale = stale_repair_proposal();
    let review = repair_review("catalog-repair/review-stale", &proposal);

    let error = store
        .write_repair_review_record_if_absent(&review, &stale)
        .expect_err("stale proposal rejected");

    assert_eq!(
        error.code(),
        "workflow_catalog.repair_review_store.stale_proposal"
    );
    assert!(!error.to_string().contains("renamed-pr-review"));
    assert!(!root.join("repair-reviews").exists());
}

#[test]
fn duplicate_catalog_write_is_rejected_without_overwrite() {
    let store = LocalWorkflowCatalogStore::new(temp_catalog_root("duplicate"));
    let record = catalog_record("catalog/workflow/pr-review", "local/pr-review");

    store
        .write_catalog_record_if_absent(&record)
        .expect("first write");
    let error = store
        .write_catalog_record_if_absent(&record)
        .expect_err("duplicate rejected");

    assert_eq!(error.kind(), WorkflowOsErrorKind::InvalidState);
    assert_eq!(error.code(), "workflow_catalog_store.record_exists");
    assert!(!error.to_string().contains("catalog/workflow/pr-review"));
}

#[test]
fn record_ids_are_encoded_as_safe_file_names() {
    let root = temp_catalog_root("safe-file-name");
    let store = LocalWorkflowCatalogStore::new(&root);
    let record = catalog_record("catalog/workflow/pr-review", "local/pr-review");

    store
        .write_catalog_record_if_absent(&record)
        .expect("write catalog");

    let expected_file = root
        .join("workflows")
        .join(encoded_id_file_name("catalog/workflow/pr-review"));
    assert!(expected_file.exists());
    assert!(!root.join("workflows").join("catalog").exists());
}

#[test]
fn listing_order_is_deterministic() {
    let store = LocalWorkflowCatalogStore::new(temp_catalog_root("list-order"));
    let later = catalog_record("catalog/workflow/z-last", "local/z-last");
    let earlier = catalog_record("catalog/workflow/a-first", "local/a-first");

    store
        .write_catalog_record_if_absent(&later)
        .expect("write later");
    store
        .write_catalog_record_if_absent(&earlier)
        .expect("write earlier");

    let records = store.list_catalog_records().expect("list records");

    assert_eq!(records[0].record_id().as_str(), "catalog/workflow/a-first");
    assert_eq!(records[1].record_id().as_str(), "catalog/workflow/z-last");
}

#[test]
fn stewardship_listing_filters_by_workflow() {
    let store = LocalWorkflowCatalogStore::new(temp_catalog_root("stewardship-filter"));
    let included = stewardship_record("stewardship/pr-review/approved", "local/pr-review");
    let excluded = stewardship_record("stewardship/release/approved", "local/release");

    store
        .write_stewardship_record_if_absent(&excluded)
        .expect("write excluded");
    store
        .write_stewardship_record_if_absent(&included)
        .expect("write included");

    let records = store
        .list_stewardship_records_for_workflow(&workflow_id("local/pr-review"))
        .expect("list stewardship");

    assert_eq!(records, vec![included]);
}

#[test]
fn missing_record_error_is_stable_and_non_leaking() {
    let store = LocalWorkflowCatalogStore::new(temp_catalog_root("missing"));
    let id = catalog_id("catalog/workflow/private-review");

    let error = store
        .read_catalog_record(&id)
        .expect_err("missing record rejected");

    assert_eq!(error.code(), "workflow_catalog_store.not_found");
    assert!(!error.to_string().contains("private-review"));
}

#[test]
fn invalid_serialized_record_fails_closed_without_leaking_payload() {
    let root = temp_catalog_root("invalid-serialized");
    let store = LocalWorkflowCatalogStore::new(&root);
    let id = "catalog/workflow/pr-review";
    let path = root.join("workflows").join(encoded_id_file_name(id));
    let invalid = json!({
        "record_id": id,
        "workflow_id": "local/pr-review",
        "workflow_path": "../secret-token-workflow.yml",
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
    });
    write_json_file(path, &invalid);

    let error = store
        .read_catalog_record(&catalog_id(id))
        .expect_err("invalid serialized record fails");

    assert_eq!(error.code(), "workflow_catalog_store.invalid_record");
    assert!(!error.to_string().contains("secret-token-workflow"));
}

#[test]
fn invalid_serialized_repair_review_fails_closed_without_leaking_payload() {
    let root = temp_catalog_root("invalid-serialized-repair-review");
    let store = LocalWorkflowCatalogStore::new(&root);
    let proposal = repair_proposal();
    let review = repair_review("catalog-repair/review-invalid-serialized", &proposal);
    let path = root
        .join("repair-reviews")
        .join(encoded_id_file_name(review.review_id().as_str()));
    let mut invalid = serde_json::to_value(&review).expect("serialize review");
    invalid["reason"] = json!("private bearer token value");
    write_json_file(path, &invalid);

    let error = store
        .read_repair_review_record(review.review_id())
        .expect_err("invalid serialized review fails");

    assert_eq!(error.code(), "workflow_catalog_store.invalid_record");
    assert!(!error.to_string().contains("bearer token value"));
    assert!(!error.to_string().contains("review-invalid-serialized"));
}

#[test]
fn corrupt_json_fails_without_leaking_file_contents() {
    let root = temp_catalog_root("corrupt-json");
    let store = LocalWorkflowCatalogStore::new(&root);
    let id = "catalog/workflow/pr-review";
    let path = root.join("workflows").join(encoded_id_file_name(id));
    fs::create_dir_all(path.parent().expect("parent")).expect("create parent");
    fs::write(&path, b"{\"secret\":\"bearer token value\"").expect("write corrupt");

    let error = store
        .read_catalog_record(&catalog_id(id))
        .expect_err("corrupt json fails");

    assert_eq!(error.code(), "workflow_catalog_store.invalid_record");
    assert!(!error.to_string().contains("bearer"));
    assert!(!error.to_string().contains("token"));
}

#[test]
fn identity_mismatch_between_file_name_and_record_fails_closed() {
    let root = temp_catalog_root("identity-mismatch");
    let store = LocalWorkflowCatalogStore::new(&root);
    let record = catalog_record("catalog/workflow/pr-review", "local/pr-review");
    let wrong_path = root
        .join("workflows")
        .join(encoded_id_file_name("catalog/workflow/other"));
    write_json_file(wrong_path, &record);

    let error = store
        .list_catalog_records()
        .expect_err("identity mismatch rejected");

    assert_eq!(error.code(), "workflow_catalog_store.identity_mismatch");
    assert!(!error.to_string().contains("catalog/workflow/pr-review"));
}

#[test]
fn debug_output_does_not_leak_catalog_root_path() {
    let root = temp_catalog_root("debug");
    let store = LocalWorkflowCatalogStore::new(&root);

    let debug = format!("{store:?}");

    assert!(debug.contains("LocalWorkflowCatalogStore"));
    assert!(debug.contains("[REDACTED]"));
    assert!(!debug.contains(root.to_string_lossy().as_ref()));
}
