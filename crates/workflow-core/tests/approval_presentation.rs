//! Approval-presentation proof model tests.
#![allow(clippy::expect_used)]

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use workflow_core::{
    compute_approval_presentation_content_hash, validate_approval_presentation_for_request,
    ActorId, ApprovalDecision, ApprovalDecisionProofEnforcementMode, ApprovalDecisionProofMarker,
    ApprovalDecisionProofMarkerDefinition, ApprovalDecisionProofValidationPolicy,
    ApprovalPresentationChannel, ApprovalPresentationContentHash, ApprovalPresentationId,
    ApprovalPresentationRecord, ApprovalPresentationRecordDefinition,
    ApprovalPresentationRecordStore, ApprovalPresentationSensitivity,
    ApprovalPresentationValidationInput, ApprovalRequest, ApprovalStore, CorrelationId,
    EventLogStore, IdempotencyKey, LocalStateBackend, RedactionDisposition, RedactionFieldState,
    RedactionMetadata, SchemaVersion, SkillId, SkillVersion, SpecContentHash, StepId, Timestamp,
    WorkflowId, WorkflowRunId, WorkflowVersion,
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

fn valid_proof_marker() -> ApprovalDecisionProofMarker {
    ApprovalDecisionProofMarker::new(ApprovalDecisionProofMarkerDefinition {
        enforcement_mode: ApprovalDecisionProofEnforcementMode::ApprovalPresentationRequired,
        presentation_id: ApprovalPresentationId::new("presentation/run-1").expect("valid id"),
        presentation_content_hash: content_hash(),
        proof_validated_at: timestamp(),
        proof_validation_policy:
            ApprovalDecisionProofValidationPolicy::ApprovalPresentationRequestMatch,
        proof_age_ms: Some(500),
        proof_freshness_limit_ms: Some(1_000),
        proof_record_sensitivity: ApprovalPresentationSensitivity::Internal,
        redaction: RedactionMetadata {
            redacted_fields: vec!["approval_presentation_payload".to_owned()],
            field_states: vec![RedactionFieldState {
                field: "approval_presentation_payload".to_owned(),
                disposition: RedactionDisposition::ReferenceOnly,
                reason: "marker stores stable proof references only".to_owned(),
            }],
        },
    })
    .expect("valid proof marker")
}

fn valid_record() -> ApprovalPresentationRecord {
    valid_record_with_presentation_id("presentation/run-1")
}

fn valid_record_with_presentation_id(presentation_id: &str) -> ApprovalPresentationRecord {
    ApprovalPresentationRecord::new(ApprovalPresentationRecordDefinition {
        presentation_id: ApprovalPresentationId::new(presentation_id).expect("valid id"),
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

fn local_backend(name: &str) -> (LocalStateBackend, PathBuf) {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let root = std::env::temp_dir().join(format!(
        "workflow-os-approval-presentation-{name}-{}-{nanos}",
        std::process::id()
    ));
    if root.exists() {
        fs::remove_dir_all(&root).expect("stale temp cleanup");
    }
    let backend = LocalStateBackend::new(&root).expect("local backend");
    (backend, root)
}

fn first_json_file_under(root: &Path) -> PathBuf {
    for entry in fs::read_dir(root).expect("read directory") {
        let path = entry.expect("directory entry").path();
        if path.is_dir() {
            let found = first_json_file_under(&path);
            if found.exists() {
                return found;
            }
        } else if path.extension().and_then(|value| value.to_str()) == Some("json") {
            return path;
        }
    }
    root.join("missing.json")
}

fn approval_request() -> ApprovalRequest {
    ApprovalRequest {
        approval_id: "approval/run-1/implementation-approved".to_owned(),
        run_id: run_id(),
        workflow_id: workflow_id(),
        schema_version: schema_version(),
        workflow_version: workflow_version(),
        spec_content_hash: SpecContentHash::from_text("approval presentation workflow"),
        resolved_execution_context_hash: None,
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
fn valid_approval_decision_proof_marker_is_bounded_and_reference_only() {
    let marker = valid_proof_marker();

    assert_eq!(
        marker.enforcement_mode(),
        ApprovalDecisionProofEnforcementMode::ApprovalPresentationRequired
    );
    assert_eq!(marker.presentation_id().as_str(), "presentation/run-1");
    assert_eq!(marker.presentation_content_hash(), &content_hash());
    assert_eq!(
        marker.proof_validation_policy(),
        ApprovalDecisionProofValidationPolicy::ApprovalPresentationRequestMatch
    );
    assert_eq!(marker.proof_age_ms(), Some(500));
    assert_eq!(marker.proof_freshness_limit_ms(), Some(1_000));
    assert_eq!(
        marker.proof_record_sensitivity(),
        ApprovalPresentationSensitivity::Internal
    );
    assert_eq!(marker.redaction().redacted_fields.len(), 1);
}

#[test]
fn approval_decision_proof_marker_enums_are_representable() {
    let enforcement_mode = ApprovalDecisionProofEnforcementMode::ApprovalPresentationRequired;
    let validation_policy = ApprovalDecisionProofValidationPolicy::ApprovalPresentationRequestMatch;

    assert_eq!(
        serde_json::to_value(enforcement_mode).expect("serialize"),
        serde_json::json!("approval_presentation_required")
    );
    assert_eq!(
        serde_json::to_value(validation_policy).expect("serialize"),
        serde_json::json!("approval_presentation_request_match")
    );
}

#[test]
fn approval_decision_proof_marker_rejects_inconsistent_freshness() {
    let error = ApprovalDecisionProofMarker::new(ApprovalDecisionProofMarkerDefinition {
        enforcement_mode: ApprovalDecisionProofEnforcementMode::ApprovalPresentationRequired,
        presentation_id: ApprovalPresentationId::new("presentation/run-1").expect("valid id"),
        presentation_content_hash: content_hash(),
        proof_validated_at: timestamp(),
        proof_validation_policy:
            ApprovalDecisionProofValidationPolicy::ApprovalPresentationRequestMatch,
        proof_age_ms: Some(2_000),
        proof_freshness_limit_ms: Some(1_000),
        proof_record_sensitivity: ApprovalPresentationSensitivity::Internal,
        redaction: redaction(),
    })
    .expect_err("freshness mismatch rejected");

    assert_eq!(
        error.code(),
        "approval_event_proof_marker.freshness_mismatch"
    );
    assert!(!error.to_string().contains("presentation/run-1"));
}

#[test]
fn approval_decision_proof_marker_rejects_secret_like_redaction_metadata() {
    let error = ApprovalDecisionProofMarker::new(ApprovalDecisionProofMarkerDefinition {
        enforcement_mode: ApprovalDecisionProofEnforcementMode::ApprovalPresentationRequired,
        presentation_id: ApprovalPresentationId::new("presentation/run-1").expect("valid id"),
        presentation_content_hash: content_hash(),
        proof_validated_at: timestamp(),
        proof_validation_policy:
            ApprovalDecisionProofValidationPolicy::ApprovalPresentationRequestMatch,
        proof_age_ms: None,
        proof_freshness_limit_ms: None,
        proof_record_sensitivity: ApprovalPresentationSensitivity::Internal,
        redaction: RedactionMetadata {
            redacted_fields: vec!["api_token".to_owned()],
            field_states: vec![RedactionFieldState {
                field: "approval_handoff".to_owned(),
                disposition: RedactionDisposition::ReferenceOnly,
                reason: "bounded reference".to_owned(),
            }],
        },
    })
    .expect_err("secret-like redaction metadata rejected");

    assert_eq!(error.code(), "approval_presentation.secret_like_value");
    assert!(!error.to_string().contains("api_token"));
}

#[test]
fn approval_decision_proof_marker_round_trips_through_serde() -> TestResult {
    let marker = valid_proof_marker();
    let json = serde_json::to_string(&marker)?;
    let round_trip: ApprovalDecisionProofMarker = serde_json::from_str(&json)?;

    assert_eq!(round_trip, marker);
    assert!(json.contains("presentation_content_hash"));
    assert!(!json.contains("implement approval presentation model"));
    assert!(!json.contains("model/helper-only implementation"));

    Ok(())
}

#[test]
fn approval_decision_proof_marker_invalid_serialized_metadata_fails_closed() {
    let value = serde_json::json!({
        "enforcement_mode": "approval_presentation_required",
        "presentation_id": "presentation/run-1",
        "presentation_content_hash": content_hash().as_str(),
        "proof_validated_at": "2026-07-09T00:00:00Z",
        "proof_validation_policy": "approval_presentation_request_match",
        "proof_age_ms": 2000,
        "proof_freshness_limit_ms": 1000,
        "proof_record_sensitivity": "internal",
        "redaction": {
            "redacted_fields": ["approval_handoff"],
            "field_states": [{
                "field": "approval_handoff",
                "disposition": "reference_only",
                "reason": "bounded reference"
            }]
        }
    });

    let error = serde_json::from_value::<ApprovalDecisionProofMarker>(value)
        .expect_err("invalid serialized marker rejected");
    let message = error.to_string();

    assert!(message.contains("approval_event_proof_marker.freshness_mismatch"));
    assert!(!message.contains("presentation/run-1"));
}

#[test]
fn approval_decision_proof_marker_debug_redacts_sensitive_context() {
    let marker = valid_proof_marker();
    let debug = format!("{marker:?}");

    assert!(debug.contains("ApprovalDecisionProofMarker"));
    assert!(debug.contains("[REDACTED]"));
    assert!(!debug.contains("presentation/run-1"));
    assert!(!debug.contains("approval_presentation_payload"));
    assert!(!debug.contains("marker stores stable proof references only"));
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

#[test]
fn local_store_writes_reads_and_lists_presentation_records() -> TestResult {
    let (backend, root) = local_backend("write-read-list");
    let first = valid_record();
    let second = valid_record_with_presentation_id("presentation/run-2");

    backend.write_approval_presentation_record(&second)?;
    backend.write_approval_presentation_record(&first)?;

    let read = backend
        .read_approval_presentation_record(first.presentation_id())?
        .expect("record exists");
    let listed = backend.list_approval_presentation_records(&run_id())?;
    let approval_listed = backend.list_approval_presentation_records_for_approval(
        &run_id(),
        "approval/run-1/implementation-approved",
    )?;

    assert_eq!(read, first);
    assert_eq!(listed, vec![first.clone(), second.clone()]);
    assert_eq!(approval_listed, vec![first, second]);

    fs::remove_dir_all(root)?;
    Ok(())
}

#[test]
fn local_store_rejects_duplicate_presentation_id_without_leaking_values() -> TestResult {
    let (backend, root) = local_backend("duplicate");
    let record = valid_record();
    backend.write_approval_presentation_record(&record)?;

    let error = backend
        .write_approval_presentation_record(&record)
        .expect_err("duplicate rejected");

    assert_eq!(error.code(), "approval_presentation_record.write.duplicate");
    assert!(!error
        .to_string()
        .contains(record.presentation_id().as_str()));
    assert!(!error.to_string().contains(record.approval_id()));
    fs::remove_dir_all(root)?;
    Ok(())
}

#[test]
fn local_store_lookup_rejects_secret_like_approval_id_without_leaking() -> TestResult {
    let (backend, root) = local_backend("secret-lookup");
    let secret = "approval/token-secret";

    let error = backend
        .list_approval_presentation_records_for_approval(&run_id(), secret)
        .expect_err("secret-like approval id rejected");

    assert_eq!(error.code(), "approval_presentation.secret_like_value");
    assert!(!error.to_string().contains(secret));
    fs::remove_dir_all(root)?;
    Ok(())
}

#[test]
fn local_store_corrupt_record_read_fails_without_leaking_payload() -> TestResult {
    let (backend, root) = local_backend("corrupt-read");
    let record = valid_record();
    backend.write_approval_presentation_record(&record)?;
    let record_root = root.join("approval_presentations").join("records");
    let record_path = first_json_file_under(&record_root);
    fs::write(&record_path, r#"{"work_summary":"token should not leak"}"#)?;

    let error = backend
        .read_approval_presentation_record(record.presentation_id())
        .expect_err("corrupt record rejected");

    assert_eq!(error.code(), "approval_presentation_record.read.corrupt");
    assert!(!error.to_string().contains("token should not leak"));
    assert!(!error
        .to_string()
        .contains(record.presentation_id().as_str()));
    assert!(!error.to_string().contains(record.approval_id()));
    fs::remove_dir_all(root)?;
    Ok(())
}

#[test]
fn local_store_write_does_not_mutate_runtime_events_or_approvals() -> TestResult {
    let (backend, root) = local_backend("no-runtime-mutation");
    let record = valid_record();
    let events_before = backend.read_events(record.run_id())?;
    let approval_before = backend.load_approval_request(record.approval_id())?;

    backend.write_approval_presentation_record(&record)?;

    let events_after = backend.read_events(record.run_id())?;
    let approval_after = backend.load_approval_request(record.approval_id())?;

    assert_eq!(events_before, events_after);
    assert_eq!(approval_before, approval_after);
    fs::remove_dir_all(root)?;
    Ok(())
}
