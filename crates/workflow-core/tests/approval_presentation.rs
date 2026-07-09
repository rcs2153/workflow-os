//! Approval-presentation proof model tests.
#![allow(clippy::expect_used)]

use workflow_core::{
    compute_approval_presentation_content_hash, validate_approval_presentation_for_request,
    ActorId, ApprovalDecision, ApprovalPresentationChannel, ApprovalPresentationContentHash,
    ApprovalPresentationId, ApprovalPresentationRecord, ApprovalPresentationRecordDefinition,
    ApprovalPresentationSensitivity, ApprovalPresentationValidationInput, ApprovalRequest,
    CorrelationId, IdempotencyKey, RedactionDisposition, RedactionFieldState, RedactionMetadata,
    SchemaVersion, SkillId, SkillVersion, SpecContentHash, StepId, Timestamp, WorkflowId,
    WorkflowRunId, WorkflowVersion,
};

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn run_id() -> WorkflowRunId {
    WorkflowRunId::new("run-approval-presentation").expect("valid run id")
}

fn workflow_id() -> WorkflowId {
    WorkflowId::new("wf/approval-presentation").expect("valid workflow id")
}

fn workflow_version() -> WorkflowVersion {
    WorkflowVersion::new("v1").expect("valid workflow version")
}

fn schema_version() -> SchemaVersion {
    SchemaVersion::new("workflowos.dev/v0").expect("valid schema version")
}

fn step_id() -> StepId {
    StepId::new("approval-step").expect("valid step id")
}

fn actor_id() -> ActorId {
    ActorId::new("user/reviewer").expect("valid actor id")
}

fn timestamp() -> Timestamp {
    Timestamp::parse_rfc3339("2026-07-09T00:00:00Z").expect("valid timestamp")
}

fn redaction() -> RedactionMetadata {
    RedactionMetadata {
        redacted_fields: vec!["approval_handoff".to_owned()],
        field_states: vec![RedactionFieldState {
            field: "approval_handoff".to_owned(),
            disposition: RedactionDisposition::ReferenceOnly,
            reason: "stores bounded presentation summaries".to_owned(),
        }],
    }
}

fn strict_non_goals() -> Vec<String> {
    vec!["no runtime approval enforcement".to_owned()]
}

fn touched_surfaces() -> Vec<String> {
    vec!["workflow-core approval presentation model".to_owned()]
}

fn validation_expectations() -> Vec<String> {
    vec!["cargo test --workspace".to_owned()]
}

fn content_hash() -> ApprovalPresentationContentHash {
    compute_approval_presentation_content_hash(
        &run_id(),
        "approval/run-1/implementation-approved",
        &workflow_id(),
        Some(&workflow_version()),
        Some(&schema_version()),
        Some(&step_id()),
        "approve bounded implementation phase",
        "implement approval presentation model",
        "model/helper-only implementation",
        &strict_non_goals(),
        &touched_surfaces(),
        &validation_expectations(),
        "close presentation proof model gap",
        "run validation and report",
        &ApprovalPresentationChannel::Terminal,
        ApprovalPresentationSensitivity::Internal,
    )
    .expect("valid content hash")
}

fn valid_record() -> ApprovalPresentationRecord {
    ApprovalPresentationRecord::new(ApprovalPresentationRecordDefinition {
        presentation_id: ApprovalPresentationId::new("presentation/run-1").expect("valid id"),
        run_id: run_id(),
        approval_id: "approval/run-1/implementation-approved".to_owned(),
        workflow_id: workflow_id(),
        workflow_version: Some(workflow_version()),
        schema_version: Some(schema_version()),
        step_id: Some(step_id()),
        requested_action: "approve bounded implementation phase".to_owned(),
        work_summary: "implement approval presentation model".to_owned(),
        approved_scope: "model/helper-only implementation".to_owned(),
        strict_non_goals: strict_non_goals(),
        expected_touched_surfaces: touched_surfaces(),
        validation_expectations: validation_expectations(),
        why_now: "close presentation proof model gap".to_owned(),
        next_action: "run validation and report".to_owned(),
        presented_at: timestamp(),
        presented_by: actor_id(),
        channel: ApprovalPresentationChannel::Terminal,
        content_hash: content_hash(),
        redaction: redaction(),
        sensitivity: ApprovalPresentationSensitivity::Internal,
    })
    .expect("valid approval presentation")
}

fn approval_request() -> ApprovalRequest {
    ApprovalRequest {
        approval_id: "approval/run-1/implementation-approved".to_owned(),
        run_id: run_id(),
        workflow_id: workflow_id(),
        schema_version: schema_version(),
        workflow_version: workflow_version(),
        spec_content_hash: SpecContentHash::from_text("approval presentation workflow"),
        step_id: step_id(),
        skill_id: SkillId::new("skill/implementation").expect("valid skill id"),
        skill_version: SkillVersion::new("v1").expect("valid skill version"),
        requested_by: ActorId::new("system/kernel").expect("valid requester"),
        correlation_id: CorrelationId::new("correlation/approval-presentation")
            .expect("valid correlation"),
        idempotency_key: Some(IdempotencyKey::new("approval-presentation-key").expect("valid key")),
        reason: "bounded implementation approval".to_owned(),
        requested_at: timestamp(),
        expires_after: None,
        expires_at: None,
        decision: None::<ApprovalDecision>,
    }
}

#[test]
fn valid_minimal_approval_presentation_record() {
    let record = valid_record();

    assert_eq!(record.run_id(), &run_id());
    assert_eq!(
        record.approval_id(),
        "approval/run-1/implementation-approved"
    );
    assert_eq!(record.workflow_id(), &workflow_id());
    assert_eq!(record.workflow_version(), Some(&workflow_version()));
    assert_eq!(record.schema_version(), Some(&schema_version()));
    assert_eq!(record.step_id(), Some(&step_id()));
    assert_eq!(record.strict_non_goals().len(), 1);
    assert_eq!(record.expected_touched_surfaces().len(), 1);
    assert_eq!(record.validation_expectations().len(), 1);
    assert_eq!(
        record.sensitivity(),
        ApprovalPresentationSensitivity::Internal
    );
}

#[test]
fn all_channels_and_sensitivity_values_are_representable() -> TestResult {
    let channels = [
        ApprovalPresentationChannel::Terminal,
        ApprovalPresentationChannel::Chat,
        ApprovalPresentationChannel::PullRequest,
        ApprovalPresentationChannel::LocalReport,
        ApprovalPresentationChannel::Custom("maintainer-console".to_owned()),
    ];
    let sensitivities = [
        ApprovalPresentationSensitivity::Public,
        ApprovalPresentationSensitivity::Internal,
        ApprovalPresentationSensitivity::Confidential,
        ApprovalPresentationSensitivity::Restricted,
    ];

    for channel in channels {
        let hash = compute_approval_presentation_content_hash(
            &run_id(),
            "approval/run-1/implementation-approved",
            &workflow_id(),
            Some(&workflow_version()),
            Some(&schema_version()),
            Some(&step_id()),
            "approve bounded implementation phase",
            "implement approval presentation model",
            "model/helper-only implementation",
            &strict_non_goals(),
            &touched_surfaces(),
            &validation_expectations(),
            "close presentation proof model gap",
            "run validation and report",
            &channel,
            ApprovalPresentationSensitivity::Internal,
        )?;

        let record = ApprovalPresentationRecord::new(ApprovalPresentationRecordDefinition {
            presentation_id: ApprovalPresentationId::new("presentation/channel")?,
            run_id: run_id(),
            approval_id: "approval/run-1/implementation-approved".to_owned(),
            workflow_id: workflow_id(),
            workflow_version: Some(workflow_version()),
            schema_version: Some(schema_version()),
            step_id: Some(step_id()),
            requested_action: "approve bounded implementation phase".to_owned(),
            work_summary: "implement approval presentation model".to_owned(),
            approved_scope: "model/helper-only implementation".to_owned(),
            strict_non_goals: strict_non_goals(),
            expected_touched_surfaces: touched_surfaces(),
            validation_expectations: validation_expectations(),
            why_now: "close presentation proof model gap".to_owned(),
            next_action: "run validation and report".to_owned(),
            presented_at: timestamp(),
            presented_by: actor_id(),
            channel,
            content_hash: hash,
            redaction: redaction(),
            sensitivity: ApprovalPresentationSensitivity::Internal,
        })?;
        assert_eq!(
            record.sensitivity(),
            ApprovalPresentationSensitivity::Internal
        );
    }

    assert_eq!(sensitivities.len(), 4);
    Ok(())
}

#[test]
fn content_hash_is_deterministic_and_changes_with_scope() -> TestResult {
    let first = content_hash();
    let second = content_hash();
    assert_eq!(first, second);

    let changed = compute_approval_presentation_content_hash(
        &run_id(),
        "approval/run-1/implementation-approved",
        &workflow_id(),
        Some(&workflow_version()),
        Some(&schema_version()),
        Some(&step_id()),
        "approve bounded implementation phase",
        "implement approval presentation model",
        "changed implementation scope",
        &strict_non_goals(),
        &touched_surfaces(),
        &validation_expectations(),
        "close presentation proof model gap",
        "run validation and report",
        &ApprovalPresentationChannel::Terminal,
        ApprovalPresentationSensitivity::Internal,
    )?;

    assert_ne!(first, changed);
    Ok(())
}

#[test]
fn mismatched_content_hash_is_rejected() {
    let error = ApprovalPresentationRecord::new(ApprovalPresentationRecordDefinition {
        presentation_id: ApprovalPresentationId::new("presentation/run-1").expect("valid id"),
        run_id: run_id(),
        approval_id: "approval/run-1/implementation-approved".to_owned(),
        workflow_id: workflow_id(),
        workflow_version: Some(workflow_version()),
        schema_version: Some(schema_version()),
        step_id: Some(step_id()),
        requested_action: "approve bounded implementation phase".to_owned(),
        work_summary: "implement approval presentation model".to_owned(),
        approved_scope: "model/helper-only implementation".to_owned(),
        strict_non_goals: strict_non_goals(),
        expected_touched_surfaces: touched_surfaces(),
        validation_expectations: validation_expectations(),
        why_now: "close presentation proof model gap".to_owned(),
        next_action: "run validation and report".to_owned(),
        presented_at: timestamp(),
        presented_by: actor_id(),
        channel: ApprovalPresentationChannel::Terminal,
        content_hash: ApprovalPresentationContentHash::new(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        )
        .expect("valid hash"),
        redaction: redaction(),
        sensitivity: ApprovalPresentationSensitivity::Internal,
    })
    .expect_err("hash mismatch");

    assert_eq!(error.code(), "approval_presentation.content_hash.mismatch");
}

#[test]
fn request_identity_validation_passes_for_matching_request() -> TestResult {
    validate_approval_presentation_for_request(ApprovalPresentationValidationInput {
        presentation: &valid_record(),
        approval_request: &approval_request(),
    })?;
    Ok(())
}

#[test]
fn request_identity_validation_rejects_mismatched_approval_id() {
    let mut request = approval_request();
    request.approval_id = "approval/other".to_owned();

    let error = validate_approval_presentation_for_request(ApprovalPresentationValidationInput {
        presentation: &valid_record(),
        approval_request: &request,
    })
    .expect_err("mismatched approval id");

    assert_eq!(
        error.code(),
        "approval_presentation.request.approval_id_mismatch"
    );
    assert!(!error.to_string().contains("approval/other"));
}

#[test]
fn duplicate_collection_entries_are_rejected() {
    let mut non_goals = strict_non_goals();
    non_goals.push("no runtime approval enforcement".to_owned());

    let error = ApprovalPresentationRecord::new(ApprovalPresentationRecordDefinition {
        presentation_id: ApprovalPresentationId::new("presentation/run-1").expect("valid id"),
        run_id: run_id(),
        approval_id: "approval/run-1/implementation-approved".to_owned(),
        workflow_id: workflow_id(),
        workflow_version: Some(workflow_version()),
        schema_version: Some(schema_version()),
        step_id: Some(step_id()),
        requested_action: "approve bounded implementation phase".to_owned(),
        work_summary: "implement approval presentation model".to_owned(),
        approved_scope: "model/helper-only implementation".to_owned(),
        strict_non_goals: non_goals,
        expected_touched_surfaces: touched_surfaces(),
        validation_expectations: validation_expectations(),
        why_now: "close presentation proof model gap".to_owned(),
        next_action: "run validation and report".to_owned(),
        presented_at: timestamp(),
        presented_by: actor_id(),
        channel: ApprovalPresentationChannel::Terminal,
        content_hash: content_hash(),
        redaction: redaction(),
        sensitivity: ApprovalPresentationSensitivity::Internal,
    })
    .expect_err("duplicate non-goals");

    assert_eq!(
        error.code(),
        "approval_presentation.strict_non_goals.duplicate"
    );
}

#[test]
fn secret_like_presentation_text_is_rejected_without_leakage() {
    let secret = "bearer sk-live-secret-token";
    let error = compute_approval_presentation_content_hash(
        &run_id(),
        "approval/run-1/implementation-approved",
        &workflow_id(),
        Some(&workflow_version()),
        Some(&schema_version()),
        Some(&step_id()),
        "approve bounded implementation phase",
        secret,
        "model/helper-only implementation",
        &strict_non_goals(),
        &touched_surfaces(),
        &validation_expectations(),
        "close presentation proof model gap",
        "run validation and report",
        &ApprovalPresentationChannel::Terminal,
        ApprovalPresentationSensitivity::Internal,
    )
    .expect_err("secret-like work summary");

    assert_eq!(error.code(), "approval_presentation.secret_like_value");
    assert!(!error.to_string().contains(secret));
}

#[test]
fn redaction_metadata_is_validated_and_debug_safe() {
    let mut metadata = redaction();
    metadata.field_states[0].reason = "secret token inside reason".to_owned();

    let error = ApprovalPresentationRecord::new(ApprovalPresentationRecordDefinition {
        presentation_id: ApprovalPresentationId::new("presentation/run-1").expect("valid id"),
        run_id: run_id(),
        approval_id: "approval/run-1/implementation-approved".to_owned(),
        workflow_id: workflow_id(),
        workflow_version: Some(workflow_version()),
        schema_version: Some(schema_version()),
        step_id: Some(step_id()),
        requested_action: "approve bounded implementation phase".to_owned(),
        work_summary: "implement approval presentation model".to_owned(),
        approved_scope: "model/helper-only implementation".to_owned(),
        strict_non_goals: strict_non_goals(),
        expected_touched_surfaces: touched_surfaces(),
        validation_expectations: validation_expectations(),
        why_now: "close presentation proof model gap".to_owned(),
        next_action: "run validation and report".to_owned(),
        presented_at: timestamp(),
        presented_by: actor_id(),
        channel: ApprovalPresentationChannel::Terminal,
        content_hash: content_hash(),
        redaction: metadata,
        sensitivity: ApprovalPresentationSensitivity::Internal,
    })
    .expect_err("secret-like redaction reason");

    assert_eq!(error.code(), "approval_presentation.secret_like_value");

    let debug = format!("{:?}", valid_record());
    assert!(!debug.contains("implement approval presentation model"));
    assert!(!debug.contains("model/helper-only implementation"));
    assert!(!debug.contains("stores bounded presentation summaries"));
    assert!(debug.contains("strict_non_goal_count"));
}

#[test]
fn serde_round_trip_validates_record_and_preserves_shape() -> TestResult {
    let record = valid_record();
    let json = serde_json::to_string(&record)?;
    assert!(json.contains("approval_id"));
    assert!(json.contains("content_hash"));
    assert!(!json.contains("raw_provider_payload"));

    let round_trip: ApprovalPresentationRecord = serde_json::from_str(&json)?;
    assert_eq!(round_trip, record);
    Ok(())
}

#[test]
fn invalid_serialized_record_fails_closed_without_secret_leakage() -> TestResult {
    let record = valid_record();
    let mut value = serde_json::to_value(&record)?;
    value["work_summary"] = serde_json::Value::String("token should not leak".to_owned());

    let error = serde_json::from_value::<ApprovalPresentationRecord>(value)
        .expect_err("invalid serialized record");
    let error_text = error.to_string();

    assert!(error_text.contains("approval_presentation.secret_like_value"));
    assert!(!error_text.contains("token should not leak"));
    Ok(())
}
