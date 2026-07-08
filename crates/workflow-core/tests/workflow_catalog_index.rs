#![allow(clippy::expect_used)]

//! Workflow catalog indexing and conflict helper tests.

use workflow_core::{
    build_workflow_catalog_index, ActorId, RedactionDisposition, RedactionFieldState,
    RedactionMetadata, SchemaVersion, SpecContentHash, Timestamp, WorkReportSensitivity,
    WorkflowArchiveRecord, WorkflowArchiveRecordDefinition, WorkflowArchiveRecordId,
    WorkflowCatalogActiveWorkflowSummary, WorkflowCatalogArchivedDraftSummary,
    WorkflowCatalogConflictKind, WorkflowCatalogConflictSeverity, WorkflowCatalogDraftSummary,
    WorkflowCatalogIndexInput, WorkflowCatalogRecord, WorkflowCatalogRecordDefinition,
    WorkflowCatalogRecordId, WorkflowId, WorkflowLifecycleStatus, WorkflowOsErrorKind,
    WorkflowStewardshipDecisionId, WorkflowStewardshipDecisionKind, WorkflowStewardshipRecord,
    WorkflowStewardshipRecordDefinition,
};

fn workflow_id(value: &str) -> WorkflowId {
    WorkflowId::new(value).expect("valid workflow id")
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

fn schema_version() -> SchemaVersion {
    SchemaVersion::new("workflowos.dev/v0").expect("valid schema version")
}

fn content_hash(value: &str) -> SpecContentHash {
    SpecContentHash::from_text(value)
}

fn timestamp() -> Timestamp {
    Timestamp::parse_rfc3339("2026-07-08T00:00:00Z").expect("valid timestamp")
}

fn actor(value: &str) -> ActorId {
    ActorId::new(value).expect("valid actor id")
}

fn redaction() -> RedactionMetadata {
    RedactionMetadata {
        redacted_fields: vec!["catalog_summary".to_owned()],
        field_states: vec![RedactionFieldState {
            field: "catalog_summary".to_owned(),
            disposition: RedactionDisposition::ReferenceOnly,
            reason: "bounded catalog summary only".to_owned(),
        }],
    }
}

fn active_summary() -> WorkflowCatalogActiveWorkflowSummary {
    WorkflowCatalogActiveWorkflowSummary::new(
        workflow_id("local/pr-review"),
        "workflows/pr-review.workflow.yml",
        content_hash("active workflow"),
        schema_version(),
    )
    .expect("valid active summary")
}

fn draft_summary() -> WorkflowCatalogDraftSummary {
    WorkflowCatalogDraftSummary::new(
        workflow_id("local/pr-review"),
        "workflows/drafts/pr-review.workflow.yml",
        content_hash("draft workflow"),
        Some("candidate".to_owned()),
    )
    .expect("valid draft summary")
}

fn archived_draft_summary() -> WorkflowCatalogArchivedDraftSummary {
    WorkflowCatalogArchivedDraftSummary::new(
        workflow_id("local/pr-review"),
        "workflows/drafts/pr-review.workflow.yml",
        "workflows/drafts/archive/pr-review.workflow.yml",
        content_hash("draft workflow"),
    )
    .expect("valid archived draft summary")
}

fn catalog_record() -> WorkflowCatalogRecord {
    WorkflowCatalogRecord::new(WorkflowCatalogRecordDefinition {
        record_id: catalog_id("catalog/workflow/pr-review"),
        workflow_id: workflow_id("local/pr-review"),
        workflow_path: "workflows/pr-review.workflow.yml".to_owned(),
        workflow_content_hash: content_hash("active workflow"),
        schema_version: schema_version(),
        lifecycle_status: WorkflowLifecycleStatus::Active,
        source_recommendation_id: Some("first_run.pr_review".to_owned()),
        source_draft_path: Some("workflows/drafts/pr-review.workflow.yml".to_owned()),
        archived_draft_path: Some("workflows/drafts/archive/pr-review.workflow.yml".to_owned()),
        owner: Some(actor("user/owner")),
        escalation_contact: Some(actor("user/escalation")),
        authority_scope: Some("governs pull request review".to_owned()),
        evidence_check_report_posture: Some("requires validation evidence".to_owned()),
        side_effect_posture: Some("none_skipped_unsupported".to_owned()),
        latest_stewardship_decision_id: Some(stewardship_id("stewardship/pr-review/approved")),
        latest_promotion_decision_id: Some(stewardship_id("stewardship/pr-review/promoted")),
        latest_archive_record_id: Some(archive_id("archive/pr-review/draft-1")),
        created_at: timestamp(),
        updated_at: timestamp(),
        sensitivity: WorkReportSensitivity::Confidential,
        redaction: redaction(),
    })
    .expect("valid catalog record")
}

fn stewardship_record(candidate_text: &str) -> WorkflowStewardshipRecord {
    WorkflowStewardshipRecord::new(WorkflowStewardshipRecordDefinition {
        decision_id: stewardship_id("stewardship/pr-review/approved"),
        decision_kind: WorkflowStewardshipDecisionKind::ApprovedForPromotion,
        workflow_id: workflow_id("local/pr-review"),
        draft_path: Some("workflows/drafts/pr-review.workflow.yml".to_owned()),
        active_workflow_path: Some("workflows/pr-review.workflow.yml".to_owned()),
        archive_path: Some("workflows/drafts/archive/pr-review.workflow.yml".to_owned()),
        candidate_content_hash: content_hash(candidate_text),
        active_content_hash: Some(content_hash("active workflow")),
        reviewer: actor("user/steward"),
        decided_at: timestamp(),
        reason_summary: Some("bounded approval summary".to_owned()),
        preflight_reference: None,
        steward_review_reference: None,
        evidence_references: Vec::new(),
        approval_references: Vec::new(),
        policy_decision_references: Vec::new(),
        validation_references: Vec::new(),
        work_report_references: Vec::new(),
        known_limitations: Vec::new(),
        strict_non_goals: Vec::new(),
        sensitivity: WorkReportSensitivity::Confidential,
        redaction: redaction(),
    })
    .expect("valid stewardship record")
}

fn archive_record(draft_text: &str) -> WorkflowArchiveRecord {
    WorkflowArchiveRecord::new(WorkflowArchiveRecordDefinition {
        archive_record_id: archive_id("archive/pr-review/draft-1"),
        original_draft_path: "workflows/drafts/pr-review.workflow.yml".to_owned(),
        archive_path: "workflows/drafts/archive/pr-review.workflow.yml".to_owned(),
        workflow_id: workflow_id("local/pr-review"),
        draft_content_hash: content_hash(draft_text),
        active_workflow_path: Some("workflows/pr-review.workflow.yml".to_owned()),
        active_workflow_content_hash: Some(content_hash("active workflow")),
        prior_draft_status: "promoted_preserved".to_owned(),
        archive_actor: actor("user/steward"),
        archive_reason_summary: Some("archived after promotion".to_owned()),
        archived_at: timestamp(),
        validation_reference: None,
        stewardship_decision_id: Some(stewardship_id("stewardship/pr-review/approved")),
        sensitivity: WorkReportSensitivity::Confidential,
        redaction: redaction(),
    })
    .expect("valid archive record")
}

fn conflict_kinds(input: WorkflowCatalogIndexInput) -> Vec<WorkflowCatalogConflictKind> {
    build_workflow_catalog_index(input)
        .expect("index builds")
        .conflicts()
        .iter()
        .map(workflow_core::WorkflowCatalogConflict::kind)
        .collect()
}

#[test]
fn valid_empty_index_constructs() {
    let index =
        build_workflow_catalog_index(WorkflowCatalogIndexInput::new()).expect("empty index");

    assert!(index.active_workflows().is_empty());
    assert!(index.drafts().is_empty());
    assert!(index.archived_drafts().is_empty());
    assert!(index.conflicts().is_empty());
}

#[test]
fn valid_catalog_index_orders_inputs_deterministically() {
    let second = WorkflowCatalogActiveWorkflowSummary::new(
        workflow_id("local/z-release"),
        "workflows/z-release.workflow.yml",
        content_hash("release"),
        schema_version(),
    )
    .expect("valid active summary");

    let input = WorkflowCatalogIndexInput::new()
        .with_active_workflows(vec![second, active_summary()])
        .with_drafts(vec![draft_summary()])
        .with_archived_drafts(vec![archived_draft_summary()])
        .with_catalog_records(vec![catalog_record()])
        .with_stewardship_records(vec![stewardship_record("draft workflow")])
        .with_archive_records(vec![archive_record("draft workflow")]);

    let index = build_workflow_catalog_index(input).expect("index builds");

    assert_eq!(
        index.active_workflows()[0].workflow_id().as_str(),
        "local/pr-review"
    );
    assert_eq!(
        index.active_workflows()[1].workflow_id().as_str(),
        "local/z-release"
    );
    assert_eq!(
        index.conflict_count_by_severity(WorkflowCatalogConflictSeverity::Warning),
        1
    );
}

#[test]
fn duplicate_active_workflow_id_and_path_are_blockers() {
    let duplicate_id = WorkflowCatalogActiveWorkflowSummary::new(
        workflow_id("local/pr-review"),
        "workflows/other.workflow.yml",
        content_hash("other"),
        schema_version(),
    )
    .expect("valid active summary");
    let duplicate_path = WorkflowCatalogActiveWorkflowSummary::new(
        workflow_id("local/other"),
        "workflows/pr-review.workflow.yml",
        content_hash("other"),
        schema_version(),
    )
    .expect("valid active summary");

    let input = WorkflowCatalogIndexInput::new().with_active_workflows(vec![
        active_summary(),
        duplicate_id,
        duplicate_path,
    ]);

    let index = build_workflow_catalog_index(input).expect("index builds");

    assert!(index.conflicts().iter().any(|conflict| {
        conflict.kind() == WorkflowCatalogConflictKind::DuplicateActiveWorkflowId
            && conflict.severity() == WorkflowCatalogConflictSeverity::Blocker
    }));
    assert!(index.conflicts().iter().any(|conflict| {
        conflict.kind() == WorkflowCatalogConflictKind::DuplicateActiveWorkflowPath
            && conflict.severity() == WorkflowCatalogConflictSeverity::Blocker
    }));
}

#[test]
fn missing_catalog_record_warns_by_default_and_blocks_when_required() {
    let default_input =
        WorkflowCatalogIndexInput::new().with_active_workflows(vec![active_summary()]);

    let default_index = build_workflow_catalog_index(default_input).expect("index builds");
    assert_eq!(
        default_index.conflicts()[0].kind(),
        WorkflowCatalogConflictKind::ActiveWorkflowMissingCatalogRecord
    );
    assert_eq!(
        default_index.conflicts()[0].severity(),
        WorkflowCatalogConflictSeverity::Warning
    );

    let strict_input = WorkflowCatalogIndexInput::new()
        .with_active_workflows(vec![active_summary()])
        .require_catalog_records_for_active_workflows(true);

    let strict_index = build_workflow_catalog_index(strict_input).expect("index builds");
    assert_eq!(
        strict_index.conflicts()[0].severity(),
        WorkflowCatalogConflictSeverity::Blocker
    );
}

#[test]
fn catalog_active_mismatches_are_blockers() {
    let mut mismatched_definition = WorkflowCatalogRecordDefinition {
        record_id: catalog_id("catalog/workflow/pr-review"),
        workflow_id: workflow_id("local/pr-review"),
        workflow_path: "workflows/moved.workflow.yml".to_owned(),
        workflow_content_hash: content_hash("stale active workflow"),
        schema_version: schema_version(),
        lifecycle_status: WorkflowLifecycleStatus::Active,
        source_recommendation_id: None,
        source_draft_path: None,
        archived_draft_path: None,
        owner: Some(actor("user/owner")),
        escalation_contact: Some(actor("user/escalation")),
        authority_scope: Some("bounded authority".to_owned()),
        evidence_check_report_posture: Some("bounded posture".to_owned()),
        side_effect_posture: Some("none_skipped_unsupported".to_owned()),
        latest_stewardship_decision_id: Some(stewardship_id("stewardship/pr-review/approved")),
        latest_promotion_decision_id: None,
        latest_archive_record_id: None,
        created_at: timestamp(),
        updated_at: timestamp(),
        sensitivity: WorkReportSensitivity::Confidential,
        redaction: redaction(),
    };
    let record = WorkflowCatalogRecord::new(mismatched_definition.clone()).expect("valid record");

    let kinds = conflict_kinds(
        WorkflowCatalogIndexInput::new()
            .with_active_workflows(vec![active_summary()])
            .with_catalog_records(vec![record]),
    );

    assert!(kinds.contains(&WorkflowCatalogConflictKind::CatalogActivePathMismatch));
    assert!(kinds.contains(&WorkflowCatalogConflictKind::CatalogActiveHashMismatch));

    mismatched_definition.workflow_id = workflow_id("local/missing");
    let missing_record =
        WorkflowCatalogRecord::new(mismatched_definition).expect("valid missing record");
    let kinds =
        conflict_kinds(WorkflowCatalogIndexInput::new().with_catalog_records(vec![missing_record]));
    assert!(kinds.contains(&WorkflowCatalogConflictKind::CatalogActiveMissingWorkflowFile));
}

#[test]
fn stale_stewardship_and_archive_records_are_blockers() {
    let kinds = conflict_kinds(
        WorkflowCatalogIndexInput::new()
            .with_drafts(vec![draft_summary()])
            .with_archived_drafts(vec![archived_draft_summary()])
            .with_stewardship_records(vec![stewardship_record("changed draft")])
            .with_archive_records(vec![archive_record("changed draft")]),
    );

    assert!(kinds.contains(&WorkflowCatalogConflictKind::DraftStewardshipHashMismatch));
    assert!(kinds.contains(&WorkflowCatalogConflictKind::ArchiveHashMismatch));

    let kinds = conflict_kinds(
        WorkflowCatalogIndexInput::new()
            .with_archive_records(vec![archive_record("draft workflow")]),
    );
    assert!(kinds.contains(&WorkflowCatalogConflictKind::ArchiveRecordMissingArchivedDraft));
}

#[test]
fn missing_owner_escalation_stewardship_and_side_effect_posture_are_warnings() {
    let mut definition = WorkflowCatalogRecordDefinition {
        record_id: catalog_id("catalog/workflow/pr-review"),
        workflow_id: workflow_id("local/pr-review"),
        workflow_path: "workflows/pr-review.workflow.yml".to_owned(),
        workflow_content_hash: content_hash("active workflow"),
        schema_version: schema_version(),
        lifecycle_status: WorkflowLifecycleStatus::Active,
        source_recommendation_id: None,
        source_draft_path: None,
        archived_draft_path: None,
        owner: None,
        escalation_contact: None,
        authority_scope: None,
        evidence_check_report_posture: None,
        side_effect_posture: None,
        latest_stewardship_decision_id: None,
        latest_promotion_decision_id: None,
        latest_archive_record_id: None,
        created_at: timestamp(),
        updated_at: timestamp(),
        sensitivity: WorkReportSensitivity::Confidential,
        redaction: redaction(),
    };
    let record = WorkflowCatalogRecord::new(definition.clone()).expect("valid catalog record");

    let index = build_workflow_catalog_index(
        WorkflowCatalogIndexInput::new()
            .with_active_workflows(vec![active_summary()])
            .with_catalog_records(vec![record]),
    )
    .expect("index builds");

    assert_eq!(
        index.conflict_count_by_severity(WorkflowCatalogConflictSeverity::Warning),
        4
    );
    assert!(index
        .conflicts()
        .iter()
        .all(|conflict| conflict.severity() == WorkflowCatalogConflictSeverity::Warning));

    definition.owner = Some(actor("user/owner"));
    definition.escalation_contact = Some(actor("user/escalation"));
    definition.side_effect_posture = Some("none_skipped_unsupported".to_owned());
    definition.latest_stewardship_decision_id =
        Some(stewardship_id("stewardship/pr-review/approved"));
    let clean_record = WorkflowCatalogRecord::new(definition).expect("valid catalog record");
    let index = build_workflow_catalog_index(
        WorkflowCatalogIndexInput::new()
            .with_active_workflows(vec![active_summary()])
            .with_catalog_records(vec![clean_record]),
    )
    .expect("index builds");
    assert!(index.conflicts().is_empty());
}

#[test]
fn unsafe_and_secret_like_paths_are_rejected_without_leaking_values() {
    let error = WorkflowCatalogActiveWorkflowSummary::new(
        workflow_id("local/pr-review"),
        "../workflow.yml",
        content_hash("active workflow"),
        schema_version(),
    )
    .expect_err("unsafe path rejected");

    assert_eq!(error.kind(), WorkflowOsErrorKind::Validation);
    assert_eq!(error.code(), "workflow_catalog_index.path.unsafe");
    assert!(!error.to_string().contains("../workflow.yml"));

    let error = WorkflowCatalogDraftSummary::new(
        workflow_id("local/pr-review"),
        "workflows/drafts/api-token.workflow.yml",
        content_hash("draft workflow"),
        None,
    )
    .expect_err("secret-like path rejected");

    assert_eq!(error.code(), "workflow_catalog_index.secret_like_value");
    assert!(!error.to_string().contains("api-token"));
}

#[test]
fn debug_output_does_not_leak_conflict_source_references() {
    let index = build_workflow_catalog_index(
        WorkflowCatalogIndexInput::new().with_active_workflows(vec![active_summary()]),
    )
    .expect("index builds");

    let debug = format!("{:?}", index.conflicts()[0]);

    assert!(debug.contains("source_reference"));
    assert!(debug.contains("[REDACTED]"));
    assert!(!debug.contains("workflows/pr-review.workflow.yml"));
}
