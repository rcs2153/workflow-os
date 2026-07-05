#![allow(clippy::expect_used)]

//! `SideEffect` core model tests.

use std::cell::RefCell;
use std::collections::BTreeMap;

use serde_json::json;
use workflow_core::SideEffectRecordStore;
use workflow_core::{
    transition_side_effect_to_attempted, transition_side_effect_to_attempted_in_store,
    transition_side_effect_to_completed, transition_side_effect_to_completed_in_store,
    transition_side_effect_to_failed, transition_side_effect_to_failed_in_store,
    validate_side_effect_approval_linkage, validate_side_effect_approval_linkage_from_store,
    ActorId, AdapterId, AdapterKind, ApprovalDecision, ApprovalDecisionKind, ApprovalRequest,
    CorrelationId, EventId, EventSequenceNumber, IdempotencyKey, IntegrationId,
    RedactionDisposition, RedactionFieldState, RedactionMetadata, SchemaVersion,
    SideEffectApprovalLinkageFromStoreInput, SideEffectApprovalLinkageInput,
    SideEffectApprovalLinkageStoreLoadMode, SideEffectAttemptTransitionInput,
    SideEffectAttemptTransitionStoreInput, SideEffectAuthority, SideEffectAuthorityDecision,
    SideEffectCapability, SideEffectCompleteTransitionInput,
    SideEffectCompleteTransitionStoreInput, SideEffectFailTransitionInput,
    SideEffectFailTransitionStoreInput, SideEffectId, SideEffectIdempotencyBinding,
    SideEffectIdempotencyScope, SideEffectLifecycleState, SideEffectMissingRecordPolicy,
    SideEffectOutcomeReference, SideEffectOutcomeReferenceKind, SideEffectRecord,
    SideEffectRecordDefinition, SideEffectReference, SideEffectReferenceKind,
    SideEffectSensitivity, SideEffectTargetKind, SideEffectTargetReference, SkillId, SkillVersion,
    SpecContentHash, StepId, Timestamp, WorkflowId, WorkflowOsError, WorkflowRun, WorkflowRunEvent,
    WorkflowRunEventKind, WorkflowRunId, WorkflowVersion,
};

fn side_effect_id() -> SideEffectId {
    SideEffectId::new("side-effect/1").expect("valid side-effect id")
}

fn target() -> SideEffectTargetReference {
    SideEffectTargetReference::new(
        SideEffectTargetKind::AdapterResource,
        "github/pull-request/42",
    )
    .expect("valid target")
}

fn actor() -> ActorId {
    ActorId::new("operator/reviewer").expect("valid actor")
}

fn system_actor() -> ActorId {
    ActorId::new("system/workflow-os").expect("valid system actor")
}

fn workflow_id() -> WorkflowId {
    WorkflowId::new("workflow/self-governed-build").expect("valid workflow id")
}

fn workflow_version() -> WorkflowVersion {
    WorkflowVersion::new("v1").expect("valid workflow version")
}

fn schema_version() -> SchemaVersion {
    SchemaVersion::new("workflowos/v0").expect("valid schema version")
}

fn run_id() -> WorkflowRunId {
    WorkflowRunId::new("run-123").expect("valid run id")
}

fn created_at() -> Timestamp {
    Timestamp::parse_rfc3339("2026-06-17T12:00:00Z").expect("valid timestamp")
}

fn redaction() -> RedactionMetadata {
    RedactionMetadata::empty()
}

fn redaction_with(field: &str, reason: &str) -> RedactionMetadata {
    RedactionMetadata {
        redacted_fields: vec![field.to_owned()],
        field_states: vec![RedactionFieldState {
            field: field.to_owned(),
            disposition: RedactionDisposition::ReferenceOnly,
            reason: reason.to_owned(),
        }],
    }
}

fn authority(decision: SideEffectAuthorityDecision) -> SideEffectAuthority {
    SideEffectAuthority::new(
        decision,
        vec![SideEffectReference::new(
            SideEffectReferenceKind::PolicyDecision,
            "event/policy-allowed",
        )
        .expect("valid policy reference")],
        Vec::new(),
    )
    .expect("valid authority")
}

fn approval_authority() -> SideEffectAuthority {
    approval_authority_with_decision(
        SideEffectAuthorityDecision::ApprovedByHuman,
        "approval/decision-1",
    )
}

fn approval_authority_with_decision(
    decision: SideEffectAuthorityDecision,
    approval_id: &str,
) -> SideEffectAuthority {
    SideEffectAuthority::new(
        decision,
        vec![SideEffectReference::new(
            SideEffectReferenceKind::PolicyDecision,
            "event/policy-requires-approval",
        )
        .expect("valid policy reference")],
        vec![
            SideEffectReference::new(SideEffectReferenceKind::ApprovalDecision, approval_id)
                .expect("valid approval reference"),
        ],
    )
    .expect("valid approval authority")
}

fn idempotency() -> SideEffectIdempotencyBinding {
    SideEffectIdempotencyBinding::new(
        IdempotencyKey::new("side-effect-key-1").expect("valid idempotency key"),
        SideEffectIdempotencyScope::Run,
        None,
        None,
    )
    .expect("valid idempotency binding")
}

fn outcome(kind: SideEffectOutcomeReferenceKind) -> SideEffectOutcomeReference {
    SideEffectOutcomeReference::new(kind, "adapter/outcome/1").expect("valid outcome reference")
}

fn reference() -> SideEffectReference {
    SideEffectReference::new(SideEffectReferenceKind::EvidenceReference, "evidence/1")
        .expect("valid side-effect reference")
}

fn valid_definition(state: SideEffectLifecycleState) -> SideEffectRecordDefinition {
    let (authority, outcome_reference, reason_codes) = match state {
        SideEffectLifecycleState::Attempted => (
            authority(SideEffectAuthorityDecision::AllowedByPolicy),
            None,
            Vec::new(),
        ),
        SideEffectLifecycleState::Completed => (
            approval_authority(),
            Some(outcome(SideEffectOutcomeReferenceKind::Outcome)),
            Vec::new(),
        ),
        SideEffectLifecycleState::Denied => (
            authority(SideEffectAuthorityDecision::DeniedByPolicy),
            None,
            vec!["policy.denied".to_owned()],
        ),
        SideEffectLifecycleState::Skipped => (
            authority(SideEffectAuthorityDecision::NotEvaluated),
            None,
            vec!["operator.skipped".to_owned()],
        ),
        SideEffectLifecycleState::Failed => (
            authority(SideEffectAuthorityDecision::AllowedByPolicy),
            Some(outcome(SideEffectOutcomeReferenceKind::Failure)),
            vec!["adapter.failed".to_owned()],
        ),
        SideEffectLifecycleState::Proposed => (
            authority(SideEffectAuthorityDecision::NotEvaluated),
            None,
            Vec::new(),
        ),
    };

    SideEffectRecordDefinition {
        side_effect_id: side_effect_id(),
        lifecycle_state: state,
        target: target(),
        capability: SideEffectCapability::GitHubWrite,
        authority,
        actor: Some(actor()),
        system_actor: None,
        workflow_id: workflow_id(),
        workflow_version: workflow_version(),
        schema_version: schema_version(),
        spec_hash: SpecContentHash::from_text("workflow spec"),
        run_id: run_id(),
        step_id: Some(StepId::new("step/review").expect("valid step id")),
        skill_id: Some(SkillId::new("skill/review").expect("valid skill id")),
        skill_version: Some(SkillVersion::new("v1").expect("valid skill version")),
        adapter_id: Some(AdapterId::new("adapter/github").expect("valid adapter id")),
        adapter_kind: Some(AdapterKind::GitHub),
        integration_id: Some(IntegrationId::new("integration/github").expect("valid integration")),
        idempotency: idempotency(),
        references: vec![reference()],
        outcome_reference,
        created_at: created_at(),
        updated_at: Some(created_at()),
        correlation_id: Some(CorrelationId::new("correlation-1").expect("valid correlation")),
        summary: Some("bounded side-effect record summary".to_owned()),
        reason_codes,
        sensitivity: SideEffectSensitivity::Confidential,
        redaction: redaction(),
    }
}

fn valid_record(state: SideEffectLifecycleState) -> SideEffectRecord {
    SideEffectRecord::new(valid_definition(state)).expect("valid side-effect record")
}

fn valid_record_with_id(state: SideEffectLifecycleState, id: &str) -> SideEffectRecord {
    let mut definition = valid_definition(state);
    definition.side_effect_id = SideEffectId::new(id).expect("valid side-effect id");
    SideEffectRecord::new(definition).expect("valid side-effect record")
}

fn proposed_record_with_allowed_authority() -> SideEffectRecord {
    let mut definition = valid_definition(SideEffectLifecycleState::Proposed);
    definition.authority = authority(SideEffectAuthorityDecision::AllowedByPolicy);
    SideEffectRecord::new(definition).expect("valid proposed record with allowed authority")
}

#[derive(Default)]
struct TestSideEffectRecordStore {
    records: RefCell<BTreeMap<SideEffectId, SideEffectRecord>>,
    fail_reads: bool,
    fail_lists: bool,
    fail_writes: bool,
}

impl TestSideEffectRecordStore {
    fn with_records(records: Vec<SideEffectRecord>) -> Self {
        Self {
            records: RefCell::new(
                records
                    .into_iter()
                    .map(|record| (record.side_effect_id().clone(), record))
                    .collect(),
            ),
            fail_reads: false,
            fail_lists: false,
            fail_writes: false,
        }
    }

    fn failing_reads(records: Vec<SideEffectRecord>) -> Self {
        Self {
            fail_reads: true,
            ..Self::with_records(records)
        }
    }

    fn failing_writes(records: Vec<SideEffectRecord>) -> Self {
        Self {
            fail_writes: true,
            ..Self::with_records(records)
        }
    }
}

impl SideEffectRecordStore for TestSideEffectRecordStore {
    fn write_side_effect_record(&self, record: &SideEffectRecord) -> Result<(), WorkflowOsError> {
        if self.fail_writes {
            return Err(WorkflowOsError::validation(
                "test_side_effect_store.write.failed",
                "store write failure with token-like secret should be hidden",
            ));
        }
        record.validate()?;
        let mut records = self.records.borrow_mut();
        if records.contains_key(record.side_effect_id()) {
            return Err(WorkflowOsError::validation(
                "test_side_effect_store.write.duplicate",
                "side-effect record already exists",
            ));
        }
        records.insert(record.side_effect_id().clone(), record.clone());
        Ok(())
    }

    fn update_side_effect_record(&self, record: &SideEffectRecord) -> Result<(), WorkflowOsError> {
        if self.fail_writes {
            return Err(WorkflowOsError::validation(
                "test_side_effect_store.update.failed",
                "store update failure with token-like secret should be hidden",
            ));
        }
        record.validate()?;
        let mut records = self.records.borrow_mut();
        let existing = records.get(record.side_effect_id()).ok_or_else(|| {
            WorkflowOsError::validation(
                "test_side_effect_store.update.missing",
                "side-effect record does not exist",
            )
        })?;
        if existing.workflow_id() != record.workflow_id()
            || existing.workflow_version() != record.workflow_version()
            || existing.schema_version() != record.schema_version()
            || existing.spec_hash() != record.spec_hash()
            || existing.run_id() != record.run_id()
        {
            return Err(WorkflowOsError::validation(
                "test_side_effect_store.update.identity_mismatch",
                "side-effect record identity mismatch",
            ));
        }
        if !test_store_allows_lifecycle_update(existing, record) {
            return Err(WorkflowOsError::validation(
                "test_side_effect_store.update.invalid_lifecycle_transition",
                "side-effect record update lifecycle transition is not supported",
            ));
        }
        records.insert(record.side_effect_id().clone(), record.clone());
        Ok(())
    }

    fn read_side_effect_record(
        &self,
        side_effect_id: &SideEffectId,
    ) -> Result<Option<SideEffectRecord>, WorkflowOsError> {
        if self.fail_reads {
            return Err(WorkflowOsError::validation(
                "test_side_effect_store.read.failed",
                "store failure with token-like secret should be hidden",
            ));
        }
        Ok(self.records.borrow().get(side_effect_id).cloned())
    }

    fn list_side_effect_records(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<Vec<SideEffectRecord>, WorkflowOsError> {
        if self.fail_lists {
            return Err(WorkflowOsError::validation(
                "test_side_effect_store.list.failed",
                "store failure with token-like secret should be hidden",
            ));
        }
        let mut records = self
            .records
            .borrow()
            .values()
            .filter(|record| record.run_id() == run_id)
            .cloned()
            .collect::<Vec<_>>();
        records.sort_by(|left, right| left.side_effect_id().cmp(right.side_effect_id()));
        Ok(records)
    }

    fn list_side_effect_records_for_workflow_run(
        &self,
        workflow_id: &WorkflowId,
        run_id: &WorkflowRunId,
    ) -> Result<Vec<SideEffectRecord>, WorkflowOsError> {
        let records = self.list_side_effect_records(run_id)?;
        Ok(records
            .into_iter()
            .filter(|record| record.workflow_id() == workflow_id)
            .collect())
    }
}

fn test_store_allows_lifecycle_update(
    existing: &SideEffectRecord,
    next: &SideEffectRecord,
) -> bool {
    matches!(
        (existing.lifecycle_state(), next.lifecycle_state()),
        (
            SideEffectLifecycleState::Proposed,
            SideEffectLifecycleState::Attempted
        ) | (
            SideEffectLifecycleState::Attempted,
            SideEffectLifecycleState::Completed | SideEffectLifecycleState::Failed
        )
    )
}

fn event(sequence: u64, kind: WorkflowRunEventKind) -> WorkflowRunEvent {
    WorkflowRunEvent {
        sequence_number: EventSequenceNumber::new(sequence).expect("valid sequence"),
        event_id: EventId::new(format!("event-{sequence}")).expect("valid event id"),
        timestamp: created_at(),
        run_id: run_id(),
        workflow_id: workflow_id(),
        schema_version: schema_version(),
        workflow_version: workflow_version(),
        spec_content_hash: SpecContentHash::from_text("workflow spec"),
        correlation_id: Some(CorrelationId::new("correlation-1").expect("valid correlation")),
        actor: Some(actor()),
        idempotency_key: None,
        kind,
    }
}

fn approval_request(approval_id: &str) -> ApprovalRequest {
    ApprovalRequest {
        approval_id: approval_id.to_owned(),
        run_id: run_id(),
        workflow_id: workflow_id(),
        schema_version: schema_version(),
        workflow_version: workflow_version(),
        spec_content_hash: SpecContentHash::from_text("workflow spec"),
        step_id: StepId::new("step/review").expect("valid step id"),
        skill_id: SkillId::new("skill/review").expect("valid skill id"),
        skill_version: SkillVersion::new("v1").expect("valid skill version"),
        requested_by: actor(),
        correlation_id: CorrelationId::new("correlation-1").expect("valid correlation"),
        idempotency_key: Some(IdempotencyKey::new("approval-idem").expect("valid idempotency")),
        reason: "human approval required".to_owned(),
        requested_at: created_at(),
        expires_after: Some("30m".to_owned()),
        expires_at: None,
        decision: None,
    }
}

fn approval_decision(approval_id: &str, decision: ApprovalDecisionKind) -> ApprovalDecision {
    ApprovalDecision {
        approval_id: approval_id.to_owned(),
        actor: actor(),
        decided_at: created_at(),
        decision,
        reason: "bounded approval decision".to_owned(),
        correlation_id: CorrelationId::new("correlation-1").expect("valid correlation"),
    }
}

fn run_with_approval(approval_id: &str, decision: Option<ApprovalDecisionKind>) -> WorkflowRun {
    let mut events = vec![
        event(1, WorkflowRunEventKind::RunCreated { summary: None }),
        event(2, WorkflowRunEventKind::RunValidated),
        event(3, WorkflowRunEventKind::RunStarted),
        event(
            4,
            WorkflowRunEventKind::ApprovalRequested(Box::new(approval_request(approval_id))),
        ),
    ];
    if let Some(decision) = decision {
        let event_kind = match decision {
            ApprovalDecisionKind::Granted => {
                WorkflowRunEventKind::ApprovalGranted(approval_decision(approval_id, decision))
            }
            ApprovalDecisionKind::Denied => {
                WorkflowRunEventKind::ApprovalDenied(approval_decision(approval_id, decision))
            }
        };
        events.push(event(5, event_kind));
    }

    WorkflowRun::rehydrate(&events).expect("valid approval run")
}

#[test]
fn valid_minimal_proposed_side_effect_record() {
    let record = valid_record(SideEffectLifecycleState::Proposed);

    assert_eq!(record.lifecycle_state(), SideEffectLifecycleState::Proposed);
    assert_eq!(record.capability(), SideEffectCapability::GitHubWrite);
    assert_eq!(
        record.target().kind(),
        SideEffectTargetKind::AdapterResource
    );
    assert_eq!(record.references().len(), 1);
}

#[test]
fn approval_linkage_accepts_matching_granted_approval() {
    let run = run_with_approval("approval/decision-1", Some(ApprovalDecisionKind::Granted));
    let record = valid_record(SideEffectLifecycleState::Completed);

    let result = validate_side_effect_approval_linkage(SideEffectApprovalLinkageInput {
        run: &run,
        side_effect_records: &[record],
        require_approval_references_for_requires_approval: true,
        require_decision_for_approved_or_denied: true,
    })
    .expect("approval linkage succeeds");

    assert_eq!(result.side_effect_record_count(), 1);
    assert_eq!(result.approval_reference_count(), 1);
    assert_eq!(result.linked_approval_reference_count(), 1);
    assert_eq!(result.duplicate_approval_reference_count(), 0);
}

#[test]
fn approval_linkage_accepts_matching_denied_approval() {
    let run = run_with_approval("approval/decision-1", Some(ApprovalDecisionKind::Denied));
    let mut definition = valid_definition(SideEffectLifecycleState::Denied);
    definition.authority = approval_authority_with_decision(
        SideEffectAuthorityDecision::DeniedByApproval,
        "approval/decision-1",
    );
    definition.reason_codes = vec!["approval.denied".to_owned()];
    let record = SideEffectRecord::new(definition).expect("valid denied by approval record");

    let result = validate_side_effect_approval_linkage(SideEffectApprovalLinkageInput {
        run: &run,
        side_effect_records: &[record],
        require_approval_references_for_requires_approval: true,
        require_decision_for_approved_or_denied: true,
    })
    .expect("denied approval linkage succeeds");

    assert_eq!(result.linked_approval_reference_count(), 1);
}

#[test]
fn approval_linkage_accepts_requires_approval_request_without_decision() {
    let run = run_with_approval("approval/decision-1", None);
    let mut definition = valid_definition(SideEffectLifecycleState::Proposed);
    definition.authority = approval_authority_with_decision(
        SideEffectAuthorityDecision::RequiresApproval,
        "approval/decision-1",
    );
    let record = SideEffectRecord::new(definition).expect("valid requires approval record");

    let result = validate_side_effect_approval_linkage(SideEffectApprovalLinkageInput {
        run: &run,
        side_effect_records: &[record],
        require_approval_references_for_requires_approval: true,
        require_decision_for_approved_or_denied: true,
    })
    .expect("approval request linkage succeeds");

    assert_eq!(result.linked_approval_reference_count(), 1);
}

#[test]
fn approval_linkage_rejects_missing_required_reference_without_leaking() {
    let run = run_with_approval("approval/decision-1", Some(ApprovalDecisionKind::Granted));
    let mut definition = valid_definition(SideEffectLifecycleState::Completed);
    definition.authority = authority(SideEffectAuthorityDecision::ApprovedByHuman);
    let record = SideEffectRecord::new(definition).expect("valid record");

    let error = validate_side_effect_approval_linkage(SideEffectApprovalLinkageInput {
        run: &run,
        side_effect_records: &[record],
        require_approval_references_for_requires_approval: true,
        require_decision_for_approved_or_denied: true,
    })
    .expect_err("missing approval decision reference rejected");

    assert_eq!(
        error.code(),
        "side_effect_approval_linkage.decision_missing"
    );
    assert!(!error.to_string().contains("approval/decision-1"));
}

#[test]
fn approval_linkage_rejects_missing_decision_for_approved_authority() {
    let run = run_with_approval("approval/decision-1", None);
    let record = valid_record(SideEffectLifecycleState::Completed);

    let error = validate_side_effect_approval_linkage(SideEffectApprovalLinkageInput {
        run: &run,
        side_effect_records: &[record],
        require_approval_references_for_requires_approval: true,
        require_decision_for_approved_or_denied: true,
    })
    .expect_err("missing approval decision rejected");

    assert_eq!(
        error.code(),
        "side_effect_approval_linkage.decision_missing"
    );
    assert!(!error.to_string().contains("approval/decision-1"));
}

#[test]
fn approval_linkage_rejects_granted_denied_decision_mismatch() {
    let run = run_with_approval("approval/decision-1", Some(ApprovalDecisionKind::Denied));
    let record = valid_record(SideEffectLifecycleState::Completed);

    let error = validate_side_effect_approval_linkage(SideEffectApprovalLinkageInput {
        run: &run,
        side_effect_records: &[record],
        require_approval_references_for_requires_approval: true,
        require_decision_for_approved_or_denied: true,
    })
    .expect_err("wrong decision kind rejected");

    assert_eq!(
        error.code(),
        "side_effect_approval_linkage.decision_kind_mismatch"
    );
    assert!(!error.to_string().contains("approval/decision-1"));
}

#[test]
fn approval_linkage_rejects_side_effect_run_identity_mismatch_without_leaking() {
    let run = run_with_approval("approval/decision-1", Some(ApprovalDecisionKind::Granted));
    let mut definition = valid_definition(SideEffectLifecycleState::Completed);
    definition.workflow_version = WorkflowVersion::new("v2").expect("valid version");
    let record = SideEffectRecord::new(definition).expect("valid mismatched record");

    let error = validate_side_effect_approval_linkage(SideEffectApprovalLinkageInput {
        run: &run,
        side_effect_records: &[record],
        require_approval_references_for_requires_approval: true,
        require_decision_for_approved_or_denied: true,
    })
    .expect_err("identity mismatch rejected");

    assert_eq!(
        error.code(),
        "side_effect_approval_linkage.identity_mismatch"
    );
    assert!(!error.to_string().contains("v2"));
    assert!(!error.to_string().contains("run-123"));
}

#[test]
fn approval_linkage_rejects_step_mismatch_without_leaking() {
    let run = run_with_approval("approval/decision-1", Some(ApprovalDecisionKind::Granted));
    let mut definition = valid_definition(SideEffectLifecycleState::Completed);
    definition.step_id = Some(StepId::new("step/other").expect("valid step id"));
    let record = SideEffectRecord::new(definition).expect("valid step mismatch record");

    let error = validate_side_effect_approval_linkage(SideEffectApprovalLinkageInput {
        run: &run,
        side_effect_records: &[record],
        require_approval_references_for_requires_approval: true,
        require_decision_for_approved_or_denied: true,
    })
    .expect_err("step mismatch rejected");

    assert_eq!(error.code(), "side_effect_approval_linkage.step_mismatch");
    assert!(!error.to_string().contains("step/other"));
    assert!(!error.to_string().contains("step/review"));
}

#[test]
fn approval_linkage_counts_duplicate_references_across_records() {
    let run = run_with_approval("approval/decision-1", Some(ApprovalDecisionKind::Granted));
    let first = valid_record(SideEffectLifecycleState::Completed);
    let mut definition = valid_definition(SideEffectLifecycleState::Completed);
    definition.side_effect_id = SideEffectId::new("side-effect/2").expect("valid side-effect id");
    let second = SideEffectRecord::new(definition).expect("valid second record");

    let result = validate_side_effect_approval_linkage(SideEffectApprovalLinkageInput {
        run: &run,
        side_effect_records: &[first, second],
        require_approval_references_for_requires_approval: true,
        require_decision_for_approved_or_denied: true,
    })
    .expect("duplicate references counted");

    assert_eq!(result.approval_reference_count(), 2);
    assert_eq!(result.linked_approval_reference_count(), 2);
    assert_eq!(result.duplicate_approval_reference_count(), 1);
}

#[test]
fn approval_linkage_accepts_records_without_approval_authority() {
    let run = run_with_approval("approval/decision-1", Some(ApprovalDecisionKind::Granted));
    let record = valid_record(SideEffectLifecycleState::Proposed);

    let result = validate_side_effect_approval_linkage(SideEffectApprovalLinkageInput {
        run: &run,
        side_effect_records: &[record],
        require_approval_references_for_requires_approval: true,
        require_decision_for_approved_or_denied: true,
    })
    .expect("records without approval authority accepted");

    assert_eq!(result.approval_reference_count(), 0);
    assert_eq!(result.linked_approval_reference_count(), 0);
}

#[test]
fn approval_linkage_debug_output_is_bounded_and_non_leaking() {
    let run = run_with_approval("approval/decision-1", Some(ApprovalDecisionKind::Granted));
    let record = valid_record(SideEffectLifecycleState::Completed);
    let input = SideEffectApprovalLinkageInput {
        run: &run,
        side_effect_records: std::slice::from_ref(&record),
        require_approval_references_for_requires_approval: true,
        require_decision_for_approved_or_denied: true,
    };
    let result = validate_side_effect_approval_linkage(input).expect("valid linkage");

    let input_debug = format!("{input:?}");
    let result_debug = format!("{result:?}");

    for debug in [input_debug, result_debug] {
        assert!(!debug.contains("approval/decision-1"));
        assert!(!debug.contains("side-effect/1"));
        assert!(!debug.contains("github/pull-request/42"));
        assert!(!debug.contains("run-123"));
        assert!(debug.contains("count"));
    }
}

#[test]
fn approval_linkage_from_store_accepts_explicit_matching_granted_record() {
    let run = run_with_approval("approval/decision-1", Some(ApprovalDecisionKind::Granted));
    let record = valid_record(SideEffectLifecycleState::Completed);
    let store = TestSideEffectRecordStore::with_records(vec![record.clone()]);

    let result = validate_side_effect_approval_linkage_from_store(
        &store,
        SideEffectApprovalLinkageFromStoreInput {
            run: &run,
            side_effect_ids: std::slice::from_ref(record.side_effect_id()),
            load_mode: SideEffectApprovalLinkageStoreLoadMode::ExplicitIds,
            missing_record_policy: SideEffectMissingRecordPolicy::RequireAll,
            require_approval_references_for_requires_approval: true,
            require_decision_for_approved_or_denied: true,
        },
    )
    .expect("store-backed approval linkage succeeds");

    assert_eq!(result.explicit_side_effect_id_count(), 1);
    assert_eq!(result.loaded_side_effect_record_count(), 1);
    assert_eq!(result.approval_linkage_side_effect_record_count(), 1);
    assert_eq!(result.approval_reference_count(), 1);
    assert_eq!(result.linked_approval_reference_count(), 1);
    assert_eq!(result.missing_side_effect_record_count(), 0);
}

#[test]
fn approval_linkage_from_store_accepts_all_records_for_run() {
    let run = run_with_approval("approval/decision-1", Some(ApprovalDecisionKind::Granted));
    let approved = valid_record(SideEffectLifecycleState::Completed);
    let proposed = valid_record_with_id(SideEffectLifecycleState::Proposed, "side-effect/2");
    let store = TestSideEffectRecordStore::with_records(vec![approved, proposed]);

    let result = validate_side_effect_approval_linkage_from_store(
        &store,
        SideEffectApprovalLinkageFromStoreInput {
            run: &run,
            side_effect_ids: &[],
            load_mode: SideEffectApprovalLinkageStoreLoadMode::AllRecordsForRun,
            missing_record_policy: SideEffectMissingRecordPolicy::RequireAll,
            require_approval_references_for_requires_approval: true,
            require_decision_for_approved_or_denied: true,
        },
    )
    .expect("all records for run are linked");

    assert_eq!(result.explicit_side_effect_id_count(), 0);
    assert_eq!(result.loaded_side_effect_record_count(), 2);
    assert_eq!(result.approval_linkage_side_effect_record_count(), 2);
    assert_eq!(result.approval_reference_count(), 1);
}

#[test]
fn approval_linkage_from_store_deduplicates_explicit_ids() {
    let run = run_with_approval("approval/decision-1", Some(ApprovalDecisionKind::Granted));
    let record = valid_record(SideEffectLifecycleState::Completed);
    let store = TestSideEffectRecordStore::with_records(vec![record.clone()]);
    let side_effect_ids = vec![
        record.side_effect_id().clone(),
        record.side_effect_id().clone(),
    ];

    let result = validate_side_effect_approval_linkage_from_store(
        &store,
        SideEffectApprovalLinkageFromStoreInput {
            run: &run,
            side_effect_ids: &side_effect_ids,
            load_mode: SideEffectApprovalLinkageStoreLoadMode::ExplicitIds,
            missing_record_policy: SideEffectMissingRecordPolicy::RequireAll,
            require_approval_references_for_requires_approval: true,
            require_decision_for_approved_or_denied: true,
        },
    )
    .expect("duplicate ids are de-duplicated");

    assert_eq!(result.explicit_side_effect_id_count(), 2);
    assert_eq!(result.duplicate_side_effect_id_count(), 1);
    assert_eq!(result.loaded_side_effect_record_count(), 1);
    assert_eq!(result.approval_reference_count(), 1);
}

#[test]
fn approval_linkage_from_store_missing_required_record_fails_without_leaking() {
    let run = run_with_approval("approval/decision-1", Some(ApprovalDecisionKind::Granted));
    let missing_id = SideEffectId::new("side-effect/missing-secret-token")
        .expect_err("secret-like side-effect id rejected before store lookup");
    assert_eq!(missing_id.code(), "side_effect.secret_like_value");

    let missing_id = SideEffectId::new("side-effect/missing").expect("valid missing id");
    let store = TestSideEffectRecordStore::default();

    let error = validate_side_effect_approval_linkage_from_store(
        &store,
        SideEffectApprovalLinkageFromStoreInput {
            run: &run,
            side_effect_ids: std::slice::from_ref(&missing_id),
            load_mode: SideEffectApprovalLinkageStoreLoadMode::ExplicitIds,
            missing_record_policy: SideEffectMissingRecordPolicy::RequireAll,
            require_approval_references_for_requires_approval: true,
            require_decision_for_approved_or_denied: true,
        },
    )
    .expect_err("missing required record rejected");

    assert_eq!(error.code(), "side_effect_approval_linkage.record_missing");
    assert!(!error.to_string().contains("side-effect/missing"));
}

#[test]
fn approval_linkage_from_store_optional_missing_record_is_counted() {
    let run = run_with_approval("approval/decision-1", Some(ApprovalDecisionKind::Granted));
    let missing_id = SideEffectId::new("side-effect/missing").expect("valid missing id");
    let store = TestSideEffectRecordStore::default();

    let result = validate_side_effect_approval_linkage_from_store(
        &store,
        SideEffectApprovalLinkageFromStoreInput {
            run: &run,
            side_effect_ids: std::slice::from_ref(&missing_id),
            load_mode: SideEffectApprovalLinkageStoreLoadMode::ExplicitIds,
            missing_record_policy: SideEffectMissingRecordPolicy::CountMissing,
            require_approval_references_for_requires_approval: true,
            require_decision_for_approved_or_denied: true,
        },
    )
    .expect("optional missing record counted");

    assert_eq!(result.explicit_side_effect_id_count(), 1);
    assert_eq!(result.loaded_side_effect_record_count(), 0);
    assert_eq!(result.missing_side_effect_record_count(), 1);
}

#[test]
fn approval_linkage_from_store_maps_store_failure_without_leaking() {
    let run = run_with_approval("approval/decision-1", Some(ApprovalDecisionKind::Granted));
    let record = valid_record(SideEffectLifecycleState::Completed);
    let store = TestSideEffectRecordStore::failing_reads(vec![record.clone()]);

    let error = validate_side_effect_approval_linkage_from_store(
        &store,
        SideEffectApprovalLinkageFromStoreInput {
            run: &run,
            side_effect_ids: std::slice::from_ref(record.side_effect_id()),
            load_mode: SideEffectApprovalLinkageStoreLoadMode::ExplicitIds,
            missing_record_policy: SideEffectMissingRecordPolicy::RequireAll,
            require_approval_references_for_requires_approval: true,
            require_decision_for_approved_or_denied: true,
        },
    )
    .expect_err("store read failure is mapped");

    assert_eq!(
        error.code(),
        "side_effect_approval_linkage.store_read_failed"
    );
    assert!(!error.to_string().contains("token-like"));
    assert!(!error.to_string().contains("side-effect/1"));
}

#[test]
fn approval_linkage_from_store_rejects_invalid_input_without_leaking() {
    let run = run_with_approval("approval/decision-1", Some(ApprovalDecisionKind::Granted));
    let store = TestSideEffectRecordStore::default();

    let error = validate_side_effect_approval_linkage_from_store(
        &store,
        SideEffectApprovalLinkageFromStoreInput {
            run: &run,
            side_effect_ids: &[],
            load_mode: SideEffectApprovalLinkageStoreLoadMode::ExplicitIds,
            missing_record_policy: SideEffectMissingRecordPolicy::RequireAll,
            require_approval_references_for_requires_approval: true,
            require_decision_for_approved_or_denied: true,
        },
    )
    .expect_err("no source selected");

    assert_eq!(error.code(), "side_effect_approval_linkage.invalid_input");
    assert!(!error.to_string().contains("run-123"));
}

#[test]
fn approval_linkage_from_store_reuses_decision_mismatch_validation() {
    let run = run_with_approval("approval/decision-1", Some(ApprovalDecisionKind::Denied));
    let record = valid_record(SideEffectLifecycleState::Completed);
    let store = TestSideEffectRecordStore::with_records(vec![record.clone()]);

    let error = validate_side_effect_approval_linkage_from_store(
        &store,
        SideEffectApprovalLinkageFromStoreInput {
            run: &run,
            side_effect_ids: std::slice::from_ref(record.side_effect_id()),
            load_mode: SideEffectApprovalLinkageStoreLoadMode::ExplicitIds,
            missing_record_policy: SideEffectMissingRecordPolicy::RequireAll,
            require_approval_references_for_requires_approval: true,
            require_decision_for_approved_or_denied: true,
        },
    )
    .expect_err("decision mismatch rejected");

    assert_eq!(
        error.code(),
        "side_effect_approval_linkage.decision_kind_mismatch"
    );
    assert!(!error.to_string().contains("approval/decision-1"));
}

#[test]
fn approval_linkage_from_store_debug_output_is_bounded_and_non_leaking() {
    let run = run_with_approval("approval/decision-1", Some(ApprovalDecisionKind::Granted));
    let record = valid_record(SideEffectLifecycleState::Completed);
    let store = TestSideEffectRecordStore::with_records(vec![record.clone()]);
    let input = SideEffectApprovalLinkageFromStoreInput {
        run: &run,
        side_effect_ids: std::slice::from_ref(record.side_effect_id()),
        load_mode: SideEffectApprovalLinkageStoreLoadMode::ExplicitIdsAndAllRecordsForRun,
        missing_record_policy: SideEffectMissingRecordPolicy::RequireAll,
        require_approval_references_for_requires_approval: true,
        require_decision_for_approved_or_denied: true,
    };
    let result =
        validate_side_effect_approval_linkage_from_store(&store, input).expect("valid linkage");

    let input_debug = format!("{input:?}");
    let result_debug = format!("{result:?}");

    for debug in [input_debug, result_debug] {
        assert!(!debug.contains("approval/decision-1"));
        assert!(!debug.contains("side-effect/1"));
        assert!(!debug.contains("github/pull-request/42"));
        assert!(!debug.contains("run-123"));
        assert!(debug.contains("count"));
    }
}

#[test]
fn all_required_lifecycle_states_are_representable() {
    for state in [
        SideEffectLifecycleState::Proposed,
        SideEffectLifecycleState::Attempted,
        SideEffectLifecycleState::Completed,
        SideEffectLifecycleState::Denied,
        SideEffectLifecycleState::Skipped,
        SideEffectLifecycleState::Failed,
    ] {
        let record = valid_record(state);
        assert_eq!(record.lifecycle_state(), state);
    }
}

#[test]
fn completed_side_effect_requires_outcome_reference() {
    let mut definition = valid_definition(SideEffectLifecycleState::Completed);
    definition.outcome_reference = None;

    let error = SideEffectRecord::new(definition).expect_err("missing outcome rejected");

    assert_eq!(error.code(), "side_effect.outcome.required");
}

#[test]
fn attempted_side_effect_rejects_denied_authority() {
    let mut definition = valid_definition(SideEffectLifecycleState::Attempted);
    definition.authority = authority(SideEffectAuthorityDecision::DeniedByPolicy);

    let error = SideEffectRecord::new(definition).expect_err("denied authority rejected");

    assert_eq!(error.code(), "side_effect.authority.not_allowed");
}

#[test]
fn proposed_side_effect_transitions_to_attempted_with_reference_only_event() {
    let prior = proposed_record_with_allowed_authority();
    let additional_reference = SideEffectReference::new(
        SideEffectReferenceKind::AdapterTelemetry,
        "adapter-telemetry/github-pr-comment-preflight",
    )
    .expect("valid telemetry reference");

    let result = transition_side_effect_to_attempted(SideEffectAttemptTransitionInput {
        prior_record: &prior,
        transitioned_at: Timestamp::parse_rfc3339("2026-06-17T12:01:00Z").expect("valid timestamp"),
        summary: Some("provider attempt boundary reached".to_owned()),
        additional_references: vec![additional_reference],
        evidence_reference_count: 2,
    })
    .expect("attempt transition succeeds");

    let record = result.record();
    let event = result.event();

    assert_eq!(
        record.lifecycle_state(),
        SideEffectLifecycleState::Attempted
    );
    assert_eq!(record.side_effect_id(), prior.side_effect_id());
    assert_eq!(record.workflow_id(), prior.workflow_id());
    assert_eq!(record.workflow_version(), prior.workflow_version());
    assert_eq!(record.schema_version(), prior.schema_version());
    assert_eq!(record.spec_hash(), prior.spec_hash());
    assert_eq!(record.run_id(), prior.run_id());
    assert_eq!(record.target(), prior.target());
    assert_eq!(record.capability(), prior.capability());
    assert_eq!(record.authority(), prior.authority());
    assert_eq!(record.outcome_reference(), None);
    assert_eq!(record.references().len(), 2);
    assert_eq!(record.summary(), Some("provider attempt boundary reached"));

    assert_eq!(event.lifecycle_state(), SideEffectLifecycleState::Attempted);
    assert_eq!(event.side_effect_id(), prior.side_effect_id());
    assert_eq!(event.references().len(), 2);
    assert_eq!(event.evidence_reference_count(), 2);
    assert_eq!(event.outcome_reference_count(), 0);
}

#[test]
fn attempted_transition_rejects_prior_denied_or_skipped_state() {
    for prior in [
        valid_record(SideEffectLifecycleState::Denied),
        valid_record(SideEffectLifecycleState::Skipped),
    ] {
        let error = transition_side_effect_to_attempted(SideEffectAttemptTransitionInput {
            prior_record: &prior,
            transitioned_at: created_at(),
            summary: None,
            additional_references: Vec::new(),
            evidence_reference_count: 0,
        })
        .expect_err("pre-attempt terminal state rejected");

        assert_eq!(error.code(), "side_effect.transition.invalid_prior_state");
        assert!(!error.to_string().contains("github/pull-request/42"));
        assert!(!error.to_string().contains("side-effect/1"));
    }
}

#[test]
fn attempted_transition_rejects_prior_record_without_allowed_authority() {
    let prior = valid_record(SideEffectLifecycleState::Proposed);

    let error = transition_side_effect_to_attempted(SideEffectAttemptTransitionInput {
        prior_record: &prior,
        transitioned_at: created_at(),
        summary: None,
        additional_references: Vec::new(),
        evidence_reference_count: 0,
    })
    .expect_err("not evaluated authority rejected by attempted record validation");

    assert_eq!(error.code(), "side_effect.authority.not_allowed");
}

#[test]
fn attempted_side_effect_transitions_to_completed_with_outcome_reference() {
    let prior_attempt = transition_side_effect_to_attempted(SideEffectAttemptTransitionInput {
        prior_record: &proposed_record_with_allowed_authority(),
        transitioned_at: created_at(),
        summary: None,
        additional_references: Vec::new(),
        evidence_reference_count: 0,
    })
    .expect("attempt transition")
    .into_parts()
    .0;

    let result = transition_side_effect_to_completed(SideEffectCompleteTransitionInput {
        prior_record: &prior_attempt,
        transitioned_at: Timestamp::parse_rfc3339("2026-06-17T12:02:00Z").expect("valid timestamp"),
        outcome_reference: outcome(SideEffectOutcomeReferenceKind::Outcome),
        summary: Some("provider returned stable outcome reference".to_owned()),
        additional_references: Vec::new(),
        evidence_reference_count: 1,
    })
    .expect("completed transition succeeds");

    assert_eq!(
        result.record().lifecycle_state(),
        SideEffectLifecycleState::Completed
    );
    assert!(result.record().outcome_reference().is_some());
    assert_eq!(
        result.event().lifecycle_state(),
        SideEffectLifecycleState::Completed
    );
    assert_eq!(result.event().outcome_reference_count(), 1);
    assert_eq!(result.event().evidence_reference_count(), 1);
    assert_eq!(result.record().run_id(), prior_attempt.run_id());
    assert_eq!(
        result.record().side_effect_id(),
        prior_attempt.side_effect_id()
    );
}

#[test]
fn completed_transition_rejects_non_attempted_prior_state() {
    let prior = proposed_record_with_allowed_authority();

    let error = transition_side_effect_to_completed(SideEffectCompleteTransitionInput {
        prior_record: &prior,
        transitioned_at: created_at(),
        outcome_reference: outcome(SideEffectOutcomeReferenceKind::Outcome),
        summary: None,
        additional_references: Vec::new(),
        evidence_reference_count: 0,
    })
    .expect_err("completed transition requires attempted prior");

    assert_eq!(error.code(), "side_effect.transition.invalid_prior_state");
}

#[test]
fn attempted_side_effect_transitions_to_failed_with_stable_reason() {
    let prior_attempt = transition_side_effect_to_attempted(SideEffectAttemptTransitionInput {
        prior_record: &proposed_record_with_allowed_authority(),
        transitioned_at: created_at(),
        summary: None,
        additional_references: Vec::new(),
        evidence_reference_count: 0,
    })
    .expect("attempt transition")
    .into_parts()
    .0;

    let result = transition_side_effect_to_failed(SideEffectFailTransitionInput {
        prior_record: &prior_attempt,
        transitioned_at: Timestamp::parse_rfc3339("2026-06-17T12:03:00Z").expect("valid timestamp"),
        outcome_reference: Some(outcome(SideEffectOutcomeReferenceKind::Failure)),
        reason_codes: vec!["provider.network_failed".to_owned()],
        summary: Some("provider failure classified without payload".to_owned()),
        additional_references: Vec::new(),
        evidence_reference_count: 0,
    })
    .expect("failed transition succeeds");

    assert_eq!(
        result.record().lifecycle_state(),
        SideEffectLifecycleState::Failed
    );
    assert_eq!(
        result.record().reason_codes(),
        ["provider.network_failed".to_owned()]
    );
    assert!(result.record().outcome_reference().is_some());
    assert_eq!(
        result.event().lifecycle_state(),
        SideEffectLifecycleState::Failed
    );
    assert_eq!(result.event().outcome_reference_count(), 1);
}

#[test]
fn failed_transition_requires_reason_or_failure_reference() {
    let prior_attempt = valid_record(SideEffectLifecycleState::Attempted);

    let error = transition_side_effect_to_failed(SideEffectFailTransitionInput {
        prior_record: &prior_attempt,
        transitioned_at: created_at(),
        outcome_reference: None,
        reason_codes: Vec::new(),
        summary: None,
        additional_references: Vec::new(),
        evidence_reference_count: 0,
    })
    .expect_err("failure reason or reference required");

    assert_eq!(error.code(), "side_effect.failure_reference.required");
}

#[test]
fn transition_helpers_do_not_write_side_effect_store_or_append_events() {
    let prior = proposed_record_with_allowed_authority();
    let store = TestSideEffectRecordStore::with_records(vec![prior.clone()]);

    let result = transition_side_effect_to_attempted(SideEffectAttemptTransitionInput {
        prior_record: &prior,
        transitioned_at: created_at(),
        summary: None,
        additional_references: Vec::new(),
        evidence_reference_count: 0,
    })
    .expect("attempt transition succeeds");

    assert_eq!(store.records.borrow().len(), 1);
    assert_eq!(
        store
            .read_side_effect_record(prior.side_effect_id())
            .expect("store read")
            .expect("record present")
            .lifecycle_state(),
        SideEffectLifecycleState::Proposed
    );
    assert_eq!(
        result.event().lifecycle_state(),
        SideEffectLifecycleState::Attempted
    );
}

#[test]
fn store_backed_proposed_side_effect_transitions_to_attempted_and_updates_store() {
    let prior = proposed_record_with_allowed_authority();
    let side_effect_id = prior.side_effect_id().clone();
    let store = TestSideEffectRecordStore::with_records(vec![prior.clone()]);

    let result = transition_side_effect_to_attempted_in_store(
        &store,
        SideEffectAttemptTransitionStoreInput {
            side_effect_id: &side_effect_id,
            transitioned_at: Timestamp::parse_rfc3339("2026-06-17T12:01:00Z")
                .expect("valid timestamp"),
            summary: Some("attempted via store-backed transition".to_owned()),
            additional_references: vec![SideEffectReference::new(
                SideEffectReferenceKind::WorkflowEvent,
                "event/provider-attempted",
            )
            .expect("valid event reference")],
            evidence_reference_count: 1,
        },
    )
    .expect("store-backed attempt transition succeeds");

    assert_eq!(
        result.record().lifecycle_state(),
        SideEffectLifecycleState::Attempted
    );
    assert_eq!(
        result.record().side_effect_id(),
        prior.side_effect_id(),
        "transition preserves side-effect identity"
    );
    assert_eq!(
        result.event().lifecycle_state(),
        SideEffectLifecycleState::Attempted
    );
    assert_eq!(result.event().evidence_reference_count(), 1);
    assert_eq!(store.records.borrow().len(), 1);
    let stored = store
        .read_side_effect_record(&side_effect_id)
        .expect("store read")
        .expect("transitioned record present");
    assert_eq!(
        stored.lifecycle_state(),
        SideEffectLifecycleState::Attempted
    );
    assert_eq!(
        stored.summary(),
        Some("attempted via store-backed transition")
    );
}

#[test]
fn store_backed_attempted_side_effect_transitions_to_completed_and_updates_store() {
    let prior_attempt = valid_record(SideEffectLifecycleState::Attempted);
    let side_effect_id = prior_attempt.side_effect_id().clone();
    let store = TestSideEffectRecordStore::with_records(vec![prior_attempt.clone()]);

    let result = transition_side_effect_to_completed_in_store(
        &store,
        SideEffectCompleteTransitionStoreInput {
            side_effect_id: &side_effect_id,
            transitioned_at: Timestamp::parse_rfc3339("2026-06-17T12:02:00Z")
                .expect("valid timestamp"),
            outcome_reference: outcome(SideEffectOutcomeReferenceKind::Outcome),
            summary: Some("completed via stable provider outcome reference".to_owned()),
            additional_references: Vec::new(),
            evidence_reference_count: 2,
        },
    )
    .expect("store-backed completed transition succeeds");

    assert_eq!(
        result.record().lifecycle_state(),
        SideEffectLifecycleState::Completed
    );
    assert_eq!(
        result.record().side_effect_id(),
        prior_attempt.side_effect_id()
    );
    assert_eq!(
        result.event().lifecycle_state(),
        SideEffectLifecycleState::Completed
    );
    assert_eq!(result.event().outcome_reference_count(), 1);
    assert_eq!(result.event().evidence_reference_count(), 2);
    assert_eq!(store.records.borrow().len(), 1);
    let stored = store
        .read_side_effect_record(&side_effect_id)
        .expect("store read")
        .expect("transitioned record present");
    assert_eq!(
        stored.lifecycle_state(),
        SideEffectLifecycleState::Completed
    );
    assert!(stored.outcome_reference().is_some());
}

#[test]
fn store_backed_attempted_side_effect_transitions_to_failed_and_updates_store() {
    let prior_attempt = valid_record(SideEffectLifecycleState::Attempted);
    let side_effect_id = prior_attempt.side_effect_id().clone();
    let store = TestSideEffectRecordStore::with_records(vec![prior_attempt.clone()]);

    let result = transition_side_effect_to_failed_in_store(
        &store,
        SideEffectFailTransitionStoreInput {
            side_effect_id: &side_effect_id,
            transitioned_at: Timestamp::parse_rfc3339("2026-06-17T12:03:00Z")
                .expect("valid timestamp"),
            outcome_reference: Some(outcome(SideEffectOutcomeReferenceKind::Failure)),
            reason_codes: vec!["provider.network_failed".to_owned()],
            summary: Some("failed via stable provider failure reference".to_owned()),
            additional_references: Vec::new(),
            evidence_reference_count: 0,
        },
    )
    .expect("store-backed failed transition succeeds");

    assert_eq!(
        result.record().lifecycle_state(),
        SideEffectLifecycleState::Failed
    );
    assert_eq!(
        result.record().side_effect_id(),
        prior_attempt.side_effect_id()
    );
    assert_eq!(
        result.event().lifecycle_state(),
        SideEffectLifecycleState::Failed
    );
    assert_eq!(result.event().outcome_reference_count(), 1);
    assert_eq!(store.records.borrow().len(), 1);
    let stored = store
        .read_side_effect_record(&side_effect_id)
        .expect("store read")
        .expect("transitioned record present");
    assert_eq!(stored.lifecycle_state(), SideEffectLifecycleState::Failed);
    assert_eq!(
        stored.reason_codes(),
        ["provider.network_failed".to_owned()]
    );
}

#[test]
fn store_backed_transition_missing_prior_fails_closed() {
    let side_effect_id = side_effect_id();
    let store = TestSideEffectRecordStore::default();

    let error = transition_side_effect_to_attempted_in_store(
        &store,
        SideEffectAttemptTransitionStoreInput {
            side_effect_id: &side_effect_id,
            transitioned_at: created_at(),
            summary: None,
            additional_references: Vec::new(),
            evidence_reference_count: 0,
        },
    )
    .expect_err("missing prior record fails closed");

    assert_eq!(error.code(), "side_effect.transition.prior_missing");
    assert_eq!(store.records.borrow().len(), 0);
}

#[test]
fn store_backed_transition_read_failure_is_non_leaking() {
    let side_effect_id = side_effect_id();
    let store =
        TestSideEffectRecordStore::failing_reads(vec![proposed_record_with_allowed_authority()]);

    let error = transition_side_effect_to_attempted_in_store(
        &store,
        SideEffectAttemptTransitionStoreInput {
            side_effect_id: &side_effect_id,
            transitioned_at: created_at(),
            summary: None,
            additional_references: Vec::new(),
            evidence_reference_count: 0,
        },
    )
    .expect_err("read failure is mapped");

    assert_eq!(error.code(), "side_effect.transition.store_read_failed");
    assert!(!error.to_string().contains("token-like secret"));
}

#[test]
fn store_backed_transition_write_failure_is_non_leaking_and_does_not_emit_partial_update() {
    let prior = proposed_record_with_allowed_authority();
    let side_effect_id = prior.side_effect_id().clone();
    let store = TestSideEffectRecordStore::failing_writes(vec![prior.clone()]);

    let error = transition_side_effect_to_attempted_in_store(
        &store,
        SideEffectAttemptTransitionStoreInput {
            side_effect_id: &side_effect_id,
            transitioned_at: created_at(),
            summary: None,
            additional_references: Vec::new(),
            evidence_reference_count: 0,
        },
    )
    .expect_err("write failure is mapped");

    assert_eq!(error.code(), "side_effect.transition.store_write_failed");
    assert!(!error.to_string().contains("token-like secret"));
    let stored = store
        .read_side_effect_record(&side_effect_id)
        .expect("store read")
        .expect("original record present");
    assert_eq!(stored.lifecycle_state(), SideEffectLifecycleState::Proposed);
}

#[test]
fn store_backed_transition_rejects_repeated_attempt_after_update() {
    let prior = proposed_record_with_allowed_authority();
    let side_effect_id = prior.side_effect_id().clone();
    let store = TestSideEffectRecordStore::with_records(vec![prior]);

    transition_side_effect_to_attempted_in_store(
        &store,
        SideEffectAttemptTransitionStoreInput {
            side_effect_id: &side_effect_id,
            transitioned_at: created_at(),
            summary: None,
            additional_references: Vec::new(),
            evidence_reference_count: 0,
        },
    )
    .expect("first transition succeeds");

    let error = transition_side_effect_to_attempted_in_store(
        &store,
        SideEffectAttemptTransitionStoreInput {
            side_effect_id: &side_effect_id,
            transitioned_at: created_at(),
            summary: None,
            additional_references: Vec::new(),
            evidence_reference_count: 0,
        },
    )
    .expect_err("second attempt is rejected by lifecycle validation");

    assert_eq!(error.code(), "side_effect.transition.invalid_prior_state");
    assert_eq!(store.records.borrow().len(), 1);
}

#[test]
fn direct_store_update_rejects_lifecycle_jump_without_leaking_values() {
    let prior = proposed_record_with_allowed_authority();
    let mut completed_definition = valid_definition(SideEffectLifecycleState::Completed);
    completed_definition.side_effect_id = prior.side_effect_id().clone();
    completed_definition.workflow_id = prior.workflow_id().clone();
    completed_definition.workflow_version = prior.workflow_version().clone();
    completed_definition.schema_version = prior.schema_version().clone();
    completed_definition.spec_hash = prior.spec_hash().clone();
    completed_definition.run_id = prior.run_id().clone();
    let completed = SideEffectRecord::new(completed_definition).expect("valid completed record");
    let store = TestSideEffectRecordStore::with_records(vec![prior]);

    let error = store
        .update_side_effect_record(&completed)
        .expect_err("direct proposed-to-completed update rejected");

    assert_eq!(
        error.code(),
        "test_side_effect_store.update.invalid_lifecycle_transition"
    );
    assert!(!error.to_string().contains("github/pull-request/42"));
    assert!(!error.to_string().contains("side-effect/1"));
}

#[test]
fn direct_store_update_rejects_same_state_replace_without_leaking_values() {
    let prior_attempt = valid_record(SideEffectLifecycleState::Attempted);
    let mut next_attempt_definition = valid_definition(SideEffectLifecycleState::Attempted);
    next_attempt_definition.side_effect_id = prior_attempt.side_effect_id().clone();
    next_attempt_definition.workflow_id = prior_attempt.workflow_id().clone();
    next_attempt_definition.workflow_version = prior_attempt.workflow_version().clone();
    next_attempt_definition.schema_version = prior_attempt.schema_version().clone();
    next_attempt_definition.spec_hash = prior_attempt.spec_hash().clone();
    next_attempt_definition.run_id = prior_attempt.run_id().clone();
    next_attempt_definition.summary = Some("attempt replay should not replace directly".to_owned());
    let next_attempt =
        SideEffectRecord::new(next_attempt_definition).expect("valid attempted replacement");
    let store = TestSideEffectRecordStore::with_records(vec![prior_attempt]);

    let error = store
        .update_side_effect_record(&next_attempt)
        .expect_err("direct same-state replacement rejected");

    assert_eq!(
        error.code(),
        "test_side_effect_store.update.invalid_lifecycle_transition"
    );
    assert!(!error.to_string().contains("attempt replay"));
}

#[test]
fn transition_debug_and_serialization_do_not_leak_provider_payloads() {
    let prior = proposed_record_with_allowed_authority();
    let result = transition_side_effect_to_attempted(SideEffectAttemptTransitionInput {
        prior_record: &prior,
        transitioned_at: created_at(),
        summary: Some("safe attempted transition".to_owned()),
        additional_references: Vec::new(),
        evidence_reference_count: 0,
    })
    .expect("attempt transition succeeds");

    let debug = format!("{result:?}");
    let serialized = serde_json::to_string(result.record()).expect("serialize record");

    for forbidden in [
        "github/pull-request/42",
        "side-effect/1",
        "raw_provider_payload",
        "Authorization",
        "Bearer",
    ] {
        assert!(!debug.contains(forbidden));
        assert!(!serialized.contains("raw_provider_payload"));
    }
}

#[test]
fn transition_rejects_secret_like_summary_without_leaking_value() {
    let prior = proposed_record_with_allowed_authority();

    let error = transition_side_effect_to_attempted(SideEffectAttemptTransitionInput {
        prior_record: &prior,
        transitioned_at: created_at(),
        summary: Some("Authorization Bearer secret-token-value".to_owned()),
        additional_references: Vec::new(),
        evidence_reference_count: 0,
    })
    .expect_err("secret-like summary rejected");

    assert_eq!(error.code(), "side_effect.secret_like_value");
    assert!(!error.to_string().contains("secret-token-value"));
    assert!(!error.to_string().contains("Authorization Bearer"));
}

#[test]
fn denied_side_effect_requires_denied_or_unsupported_authority() {
    let mut definition = valid_definition(SideEffectLifecycleState::Denied);
    definition.authority = authority(SideEffectAuthorityDecision::AllowedByPolicy);

    let error = SideEffectRecord::new(definition).expect_err("allowed denied record rejected");

    assert_eq!(error.code(), "side_effect.authority.denied_required");
}

#[test]
fn denied_and_skipped_side_effects_require_stable_reason_codes() {
    for state in [
        SideEffectLifecycleState::Denied,
        SideEffectLifecycleState::Skipped,
    ] {
        let mut definition = valid_definition(state);
        definition.reason_codes.clear();

        let error = SideEffectRecord::new(definition).expect_err("missing reason rejected");

        assert_eq!(error.code(), "side_effect.reason.required");
    }
}

#[test]
fn unknown_capability_fails_closed_for_attempted_completed_and_failed() {
    for state in [
        SideEffectLifecycleState::Attempted,
        SideEffectLifecycleState::Completed,
        SideEffectLifecycleState::Failed,
    ] {
        let mut definition = valid_definition(state);
        definition.capability = SideEffectCapability::Unknown;

        let error =
            SideEffectRecord::new(definition).expect_err("unknown unsafe capability rejected");

        assert_eq!(error.code(), "side_effect.capability.unknown");
    }
}

#[test]
fn unknown_capability_can_record_denied_request_without_write_support() {
    let mut definition = valid_definition(SideEffectLifecycleState::Denied);
    definition.capability = SideEffectCapability::Unknown;
    definition.authority = authority(SideEffectAuthorityDecision::DeniedByCapability);
    definition.reason_codes = vec!["capability.unknown".to_owned()];

    let record = SideEffectRecord::new(definition).expect("denied unknown capability recorded");

    assert_eq!(record.capability(), SideEffectCapability::Unknown);
}

#[test]
fn invalid_side_effect_id_is_rejected_without_leaking_value() {
    let error = SideEffectId::new("bad id with spaces").expect_err("invalid id rejected");

    assert_eq!(error.code(), "side_effect.identifier.invalid_character");
    assert!(!error.to_string().contains("bad id with spaces"));
}

#[test]
fn secret_like_target_reference_is_rejected_without_leaking_value() {
    let secret = "github/pull-request/42?token=super-sensitive";
    let error = SideEffectTargetReference::new(SideEffectTargetKind::AdapterResource, secret)
        .expect_err("secret-like target rejected");

    assert_eq!(error.code(), "side_effect.secret_like_value");
    assert!(!error.to_string().contains(secret));
}

#[test]
fn duplicate_references_are_rejected() {
    let duplicate =
        SideEffectReference::new(SideEffectReferenceKind::EvidenceReference, "evidence/1")
            .expect("valid reference");
    let mut definition = valid_definition(SideEffectLifecycleState::Proposed);
    definition.references = vec![duplicate.clone(), duplicate];

    let error = SideEffectRecord::new(definition).expect_err("duplicate reference rejected");

    assert_eq!(error.code(), "side_effect.reference.duplicate");
}

#[test]
fn missing_actor_and_system_actor_is_rejected() {
    let mut definition = valid_definition(SideEffectLifecycleState::Proposed);
    definition.actor = None;
    definition.system_actor = None;

    let error = SideEffectRecord::new(definition).expect_err("missing actor rejected");

    assert_eq!(error.code(), "side_effect.actor.required");
}

#[test]
fn system_actor_satisfies_actor_requirement() {
    let mut definition = valid_definition(SideEffectLifecycleState::Proposed);
    definition.actor = None;
    definition.system_actor = Some(system_actor());

    let record = SideEffectRecord::new(definition).expect("system actor accepted");

    assert_eq!(record.lifecycle_state(), SideEffectLifecycleState::Proposed);
}

#[test]
fn secret_like_summary_reason_and_redaction_metadata_are_rejected() {
    let mut definition = valid_definition(SideEffectLifecycleState::Proposed);
    definition.summary = Some("contains bearer token".to_owned());
    let error = SideEffectRecord::new(definition).expect_err("secret summary rejected");
    assert_eq!(error.code(), "side_effect.secret_like_value");
    assert!(!error.to_string().contains("bearer token"));

    let mut definition = valid_definition(SideEffectLifecycleState::Denied);
    definition.reason_codes = vec!["secret.reason".to_owned()];
    let error = SideEffectRecord::new(definition).expect_err("secret reason rejected");
    assert_eq!(error.code(), "side_effect.secret_like_value");
    assert!(!error.to_string().contains("secret.reason"));

    let mut definition = valid_definition(SideEffectLifecycleState::Proposed);
    definition.redaction = redaction_with("authorization_header", "safe reason");
    let error = SideEffectRecord::new(definition).expect_err("secret redaction field rejected");
    assert_eq!(error.code(), "side_effect.secret_like_value");
    assert!(!error.to_string().contains("authorization_header"));
}

#[test]
fn serde_round_trip_for_valid_record() {
    let record = valid_record(SideEffectLifecycleState::Completed);

    let encoded = serde_json::to_string(&record).expect("serialize record");
    let decoded: SideEffectRecord = serde_json::from_str(&encoded).expect("deserialize record");

    assert_eq!(decoded, record);
}

#[test]
fn invalid_serialized_record_fails_closed_without_leaking_secret_value() {
    let mut value = serde_json::to_value(valid_record(SideEffectLifecycleState::Proposed))
        .expect("serialize record to value");
    value["target"]["reference"] = json!("github/pr/1?api_token=do-not-leak");

    let error = serde_json::from_value::<SideEffectRecord>(value)
        .expect_err("invalid serialized target fails closed");
    let message = error.to_string();

    assert!(message.contains("side_effect.secret_like_value"));
    assert!(!message.contains("do-not-leak"));
    assert!(!message.contains("api_token=do-not-leak"));
}

#[test]
fn debug_output_redacts_sensitive_record_fields() {
    let record = valid_record(SideEffectLifecycleState::Completed);

    let debug = format!("{record:?}");

    assert!(!debug.contains("github/pull-request/42"));
    assert!(!debug.contains("workflow/self-governed-build"));
    assert!(!debug.contains("bounded side-effect record summary"));
    assert!(!debug.contains("event/policy-allowed"));
    assert!(debug.contains("reference_count"));
}

#[test]
fn serialization_does_not_include_forbidden_raw_payload_markers() {
    let record = valid_record(SideEffectLifecycleState::Completed);

    let encoded = serde_json::to_string(&record).expect("serialize record");

    for forbidden in [
        "raw_provider_payload",
        "raw_command_output",
        "raw_ci_log",
        "raw_spec_contents",
        "parser_payload",
        "authorization",
        "private_key",
        "api_token",
    ] {
        assert!(!encoded.contains(forbidden));
    }
}

#[test]
fn valid_redaction_metadata_still_works() {
    let mut definition = valid_definition(SideEffectLifecycleState::Proposed);
    definition.redaction = redaction_with(
        "target_reference",
        "record stores bounded reference instead of raw payload",
    );

    let record = SideEffectRecord::new(definition).expect("valid redaction metadata accepted");

    assert_eq!(record.lifecycle_state(), SideEffectLifecycleState::Proposed);
}

#[test]
fn idempotency_binding_can_reference_prior_side_effect_without_retry_behavior() {
    let binding = SideEffectIdempotencyBinding::new(
        IdempotencyKey::new("side-effect-key-duplicate").expect("valid key"),
        SideEffectIdempotencyScope::Adapter,
        Some(SideEffectId::new("side-effect/prior").expect("valid prior id")),
        Some(outcome(SideEffectOutcomeReferenceKind::Duplicate)),
    )
    .expect("valid duplicate binding");
    let mut definition = valid_definition(SideEffectLifecycleState::Proposed);
    definition.idempotency = binding;

    let record = SideEffectRecord::new(definition).expect("record with duplicate binding");

    assert_eq!(record.lifecycle_state(), SideEffectLifecycleState::Proposed);
}
