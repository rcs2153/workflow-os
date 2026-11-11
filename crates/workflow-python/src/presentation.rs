//! Durable presentations returned to the embedding application's human surface.
use super::error;
use workflow_core::{
    compute_approval_presentation_content_hash, ActorId, ApprovalPresentationChannel,
    ApprovalPresentationId, ApprovalPresentationRecord, ApprovalPresentationRecordDefinition,
    ApprovalPresentationRecordStore, ApprovalPresentationSensitivity, RedactionMetadata, Timestamp,
    WorkflowOsError, WorkflowRun,
};

pub(super) fn persist<B: ApprovalPresentationRecordStore>(
    backend: &B,
    run: &WorkflowRun,
    fixture: bool,
    create: bool,
) -> Result<ApprovalPresentationRecord, WorkflowOsError> {
    let approval = run.snapshot.approval_requests.last().ok_or_else(|| {
        error(
            "python.approval.missing",
            "Waiting run lacks an approval request",
        )
    })?;
    let existing = backend.list_approval_presentation_records_for_approval(
        &run.snapshot.identity.run_id,
        &approval.approval_id,
    )?;
    match existing.as_slice() {
        [record] => return Ok(record.clone()),
        [] => {}
        _ => {
            return Err(error(
                "python.approval.ambiguous",
                "Multiple presentations for one approval",
            ))
        }
    }
    if !create {
        return Err(error(
            "python.approval.presentation_missing",
            "Waiting run lacks a persisted presentation; inspect never creates approval proof",
        ));
    }
    let requested_action = if fixture {
        "Approve the exact fixture-backed GitHub repository metadata read"
    } else {
        "Approve and resume the declared local Python skill workflow"
    }
    .to_owned();
    let work_summary = if fixture { "Read the immutable repository metadata fixture through the existing Rust adapter." } else { "Invoke explicitly registered trusted Python callbacks through the Rust local executor after policy and approval checks." }.to_owned();
    let approved_scope = "Only the exact run, declared workflow, skills, policies, inputs, and authored approval boundary.".to_owned();
    let strict_non_goals = vec!["No new provider writes, automatic approvals, tool interception, sandboxing, or authenticated identity claims.".to_owned()];
    let expected_touched_surfaces = vec![
        "Local durable run state, audit history, and explicitly registered tool handlers."
            .to_owned(),
    ];
    let validation_expectations = vec![
        "Core must validate the current workflow and approval context before allowing execution."
            .to_owned(),
    ];
    let why_now =
        "The declared workflow has reached its authored human approval checkpoint.".to_owned();
    let next_action = "Present this complete record to the human, then call approve with this run and approval ID, actor, and reason.".to_owned();
    let channel = ApprovalPresentationChannel::Custom("python-sdk".to_owned());
    let sensitivity = ApprovalPresentationSensitivity::Internal;
    let content_hash = compute_approval_presentation_content_hash(
        &run.snapshot.identity.run_id,
        &approval.approval_id,
        &approval.workflow_id,
        Some(&approval.workflow_version),
        Some(&approval.schema_version),
        approval.step_id.as_ref(),
        &requested_action,
        &work_summary,
        &approved_scope,
        &strict_non_goals,
        &expected_touched_surfaces,
        &validation_expectations,
        &why_now,
        &next_action,
        &channel,
        sensitivity,
    )?;
    let record = ApprovalPresentationRecord::new(ApprovalPresentationRecordDefinition {
        presentation_id: ApprovalPresentationId::new(format!(
            "presentation/{}",
            &content_hash.as_str()[..16]
        ))?,
        run_id: run.snapshot.identity.run_id.clone(),
        approval_id: approval.approval_id.clone(),
        workflow_id: approval.workflow_id.clone(),
        workflow_version: Some(approval.workflow_version.clone()),
        schema_version: Some(approval.schema_version.clone()),
        step_id: approval.step_id.clone(),
        requested_action,
        work_summary,
        approved_scope,
        strict_non_goals,
        expected_touched_surfaces,
        validation_expectations,
        why_now,
        next_action,
        presented_at: Timestamp::now_utc(),
        presented_by: ActorId::new("system/workflow-os-python")?,
        channel,
        content_hash,
        redaction: RedactionMetadata::empty(),
        sensitivity,
    })?;
    backend.write_approval_presentation_record(&record)?;
    Ok(record)
}
