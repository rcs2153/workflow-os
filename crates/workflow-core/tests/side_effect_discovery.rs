#![allow(clippy::expect_used)]

//! `SideEffect` in-memory discovery helper tests.

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use workflow_core::{
    discover_side_effect_references, discover_side_effect_references_from_store, ActorId,
    CorrelationId, EventId, EventSequenceNumber, IdempotencyKey, LocalStateBackend,
    RedactionMetadata, SchemaVersion, SideEffectAuthority, SideEffectAuthorityDecision,
    SideEffectCapability, SideEffectDiscoveryInput, SideEffectDiscoverySource, SideEffectId,
    SideEffectIdempotencyBinding, SideEffectIdempotencyScope, SideEffectLifecycleState,
    SideEffectRecord, SideEffectRecordDefinition, SideEffectRecordStore, SideEffectSensitivity,
    SideEffectStoreBackedDiscoveryInput, SideEffectTargetKind, SideEffectTargetReference,
    SideEffectWorkflowEvent, SideEffectWorkflowEventDefinition, SkillId, SkillVersion,
    SpecContentHash, StepId, Timestamp, WorkflowId, WorkflowOsError, WorkflowOsErrorKind,
    WorkflowRunEvent, WorkflowRunEventKind, WorkflowRunId, WorkflowVersion,
};

static NEXT_TEST_BACKEND: AtomicU64 = AtomicU64::new(1);

fn workflow_id() -> WorkflowId {
    WorkflowId::new("workflow/side-effect-discovery").expect("valid workflow id")
}

fn workflow_version() -> WorkflowVersion {
    WorkflowVersion::new("v1").expect("valid workflow version")
}

fn schema_version() -> SchemaVersion {
    SchemaVersion::new("workflowos/v0").expect("valid schema version")
}

fn spec_hash() -> SpecContentHash {
    SpecContentHash::from_text("side-effect discovery workflow spec")
}

fn run_id() -> WorkflowRunId {
    WorkflowRunId::new("run-side-effect-discovery").expect("valid run id")
}

fn timestamp() -> Timestamp {
    Timestamp::parse_rfc3339("2026-06-19T12:00:00Z").expect("valid timestamp")
}

fn side_effect_id(value: &str) -> SideEffectId {
    SideEffectId::new(value).expect("valid side-effect id")
}

fn base_input() -> SideEffectDiscoveryInput {
    SideEffectDiscoveryInput {
        workflow_id: workflow_id(),
        workflow_version: workflow_version(),
        schema_version: schema_version(),
        spec_hash: spec_hash(),
        run_id: run_id(),
        explicit_side_effect_ids: Vec::new(),
        workflow_events: Vec::new(),
        side_effect_records: Vec::new(),
        require_records: false,
    }
}

fn base_store_input() -> SideEffectStoreBackedDiscoveryInput {
    SideEffectStoreBackedDiscoveryInput {
        workflow_id: workflow_id(),
        workflow_version: workflow_version(),
        schema_version: schema_version(),
        spec_hash: spec_hash(),
        run_id: run_id(),
        explicit_side_effect_ids: Vec::new(),
        workflow_events: Vec::new(),
        require_records: false,
    }
}

fn local_backend() -> (LocalStateBackend, PathBuf) {
    let id = NEXT_TEST_BACKEND.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!(
        "workflow-os-side-effect-discovery-{}-{id}",
        std::process::id()
    ));
    if root.exists() {
        fs::remove_dir_all(&root).expect("stale backend cleanup");
    }
    (
        LocalStateBackend::new(root.clone()).expect("local backend"),
        root,
    )
}

fn side_effect_event_payload(
    id: SideEffectId,
    lifecycle_state: SideEffectLifecycleState,
) -> SideEffectWorkflowEvent {
    SideEffectWorkflowEvent::new(SideEffectWorkflowEventDefinition {
        side_effect_id: id,
        lifecycle_state,
        step_id: Some(StepId::new("step/implementation").expect("valid step id")),
        skill_id: Some(SkillId::new("skill/implementation").expect("valid skill id")),
        skill_version: Some(SkillVersion::new("v1").expect("valid skill version")),
        correlation_id: Some(CorrelationId::new("correlation/side-effect").expect("valid id")),
        references: Vec::new(),
        evidence_reference_count: 0,
        outcome_reference_count: 0,
        redaction: RedactionMetadata::empty(),
        sensitivity: SideEffectSensitivity::Confidential,
    })
    .expect("valid side-effect workflow event")
}

fn event(
    sequence: u64,
    kind: WorkflowRunEventKind,
    workflow_id: WorkflowId,
    workflow_version: WorkflowVersion,
    schema_version: SchemaVersion,
    spec_hash: SpecContentHash,
    run_id: WorkflowRunId,
) -> WorkflowRunEvent {
    WorkflowRunEvent {
        sequence_number: EventSequenceNumber::new(sequence).expect("valid sequence"),
        event_id: EventId::new(format!("event/side-effect-discovery-{sequence}"))
            .expect("valid event id"),
        timestamp: timestamp(),
        run_id,
        workflow_id,
        schema_version,
        workflow_version,
        spec_content_hash: spec_hash,
        correlation_id: Some(CorrelationId::new("correlation/side-effect").expect("valid id")),
        actor: Some(ActorId::new("system/workflow-os").expect("valid actor")),
        idempotency_key: Some(
            IdempotencyKey::new(format!("idem/side-effect-discovery-{sequence}"))
                .expect("valid idempotency key"),
        ),
        kind,
    }
}

fn side_effect_event(
    sequence: u64,
    id: SideEffectId,
    lifecycle_state: SideEffectLifecycleState,
) -> WorkflowRunEvent {
    let payload = side_effect_event_payload(id, lifecycle_state);
    let kind = match lifecycle_state {
        SideEffectLifecycleState::Proposed => {
            WorkflowRunEventKind::SideEffectProposed(Box::new(payload))
        }
        SideEffectLifecycleState::Denied => {
            WorkflowRunEventKind::SideEffectDenied(Box::new(payload))
        }
        SideEffectLifecycleState::Skipped => {
            WorkflowRunEventKind::SideEffectSkipped(Box::new(payload))
        }
        SideEffectLifecycleState::Attempted => {
            WorkflowRunEventKind::SideEffectAttempted(Box::new(payload))
        }
        SideEffectLifecycleState::Completed => {
            WorkflowRunEventKind::SideEffectCompleted(Box::new(payload))
        }
        SideEffectLifecycleState::Failed => {
            WorkflowRunEventKind::SideEffectFailed(Box::new(payload))
        }
    };
    event(
        sequence,
        kind,
        workflow_id(),
        workflow_version(),
        schema_version(),
        spec_hash(),
        run_id(),
    )
}

fn side_effect_record(id: SideEffectId) -> SideEffectRecord {
    SideEffectRecord::new(SideEffectRecordDefinition {
        side_effect_id: id,
        lifecycle_state: SideEffectLifecycleState::Proposed,
        target: SideEffectTargetReference::new(
            SideEffectTargetKind::AdapterResource,
            "github/pull-request/42",
        )
        .expect("valid target"),
        capability: SideEffectCapability::GitHubWrite,
        authority: SideEffectAuthority::new(
            SideEffectAuthorityDecision::NotEvaluated,
            Vec::new(),
            Vec::new(),
        )
        .expect("valid authority"),
        actor: Some(ActorId::new("operator/reviewer").expect("valid actor")),
        system_actor: None,
        workflow_id: workflow_id(),
        workflow_version: workflow_version(),
        schema_version: schema_version(),
        spec_hash: spec_hash(),
        run_id: run_id(),
        step_id: Some(StepId::new("step/implementation").expect("valid step id")),
        skill_id: Some(SkillId::new("skill/implementation").expect("valid skill id")),
        skill_version: Some(SkillVersion::new("v1").expect("valid skill version")),
        adapter_id: None,
        adapter_kind: None,
        integration_id: None,
        idempotency: SideEffectIdempotencyBinding::new(
            IdempotencyKey::new("idem/side-effect-record").expect("valid idempotency key"),
            SideEffectIdempotencyScope::Run,
            None,
            None,
        )
        .expect("valid idempotency"),
        references: Vec::new(),
        outcome_reference: None,
        created_at: timestamp(),
        updated_at: Some(timestamp()),
        correlation_id: None,
        summary: Some("bounded side-effect summary".to_owned()),
        reason_codes: Vec::new(),
        sensitivity: SideEffectSensitivity::Confidential,
        redaction: RedactionMetadata::empty(),
    })
    .expect("valid side-effect record")
}

#[test]
fn explicit_side_effect_ids_are_returned_deterministically() {
    let mut input = base_input();
    input.explicit_side_effect_ids = vec![
        side_effect_id("side-effect/z"),
        side_effect_id("side-effect/a"),
        side_effect_id("side-effect/z"),
    ];

    let result = discover_side_effect_references(&input).expect("discovery succeeds");

    let ids = result
        .references()
        .iter()
        .map(|reference| reference.side_effect_id().as_str())
        .collect::<Vec<_>>();
    assert_eq!(ids, vec!["side-effect/a", "side-effect/z"]);
    assert!(result
        .references()
        .iter()
        .all(|reference| reference.source() == SideEffectDiscoverySource::ExplicitInput));
    assert_eq!(result.missing_record_count(), 2);
}

#[test]
fn proposed_denied_and_skipped_events_are_discovered() {
    let mut input = base_input();
    input.workflow_events = vec![
        side_effect_event(
            1,
            side_effect_id("side-effect/proposed"),
            SideEffectLifecycleState::Proposed,
        ),
        side_effect_event(
            2,
            side_effect_id("side-effect/denied"),
            SideEffectLifecycleState::Denied,
        ),
        side_effect_event(
            3,
            side_effect_id("side-effect/skipped"),
            SideEffectLifecycleState::Skipped,
        ),
    ];

    let result = discover_side_effect_references(&input).expect("discovery succeeds");

    assert_eq!(result.references().len(), 3);
    assert!(result
        .references()
        .iter()
        .all(|reference| reference.source() == SideEffectDiscoverySource::WorkflowEvent));
}

#[test]
fn attempted_completed_and_failed_events_remain_unsupported_for_first_discovery_slice() {
    let mut input = base_input();
    input.workflow_events = vec![
        side_effect_event(
            1,
            side_effect_id("side-effect/attempted"),
            SideEffectLifecycleState::Attempted,
        ),
        side_effect_event(
            2,
            side_effect_id("side-effect/completed"),
            SideEffectLifecycleState::Completed,
        ),
        side_effect_event(
            3,
            side_effect_id("side-effect/failed"),
            SideEffectLifecycleState::Failed,
        ),
    ];

    let result = discover_side_effect_references(&input).expect("discovery succeeds");

    assert!(result.references().is_empty());
    assert_eq!(result.unsupported_event_count(), 3);
}

#[test]
fn matching_records_are_discovered_and_satisfy_required_record_policy() {
    let id = side_effect_id("side-effect/record");
    let mut input = base_input();
    input.explicit_side_effect_ids = vec![id.clone()];
    input.side_effect_records = vec![side_effect_record(id)];
    input.require_records = true;

    let result = discover_side_effect_references(&input).expect("record satisfies requirement");

    assert_eq!(result.references().len(), 1);
    assert_eq!(result.missing_record_count(), 0);
    assert!(result.records_required());
}

#[test]
fn required_missing_record_fails_closed_without_leaking_id() {
    let mut input = base_input();
    input.explicit_side_effect_ids = vec![side_effect_id("side-effect/missing-record")];
    input.require_records = true;

    let error = discover_side_effect_references(&input).expect_err("missing record rejected");

    assert_eq!(error.code(), "side_effect_discovery.record_missing");
    assert!(!error.to_string().contains("side-effect/missing-record"));
}

#[test]
fn event_identity_mismatch_fails_without_leaking_values() {
    let mut input = base_input();
    input.workflow_events = vec![event(
        1,
        WorkflowRunEventKind::RunStarted,
        WorkflowId::new("workflow/other").expect("valid workflow id"),
        workflow_version(),
        schema_version(),
        spec_hash(),
        run_id(),
    )];

    let error = discover_side_effect_references(&input).expect_err("identity mismatch rejected");

    assert_eq!(error.code(), "side_effect_discovery.identity_mismatch");
    assert!(!error.to_string().contains("workflow/other"));
    assert!(!error.to_string().contains(run_id().as_str()));
}

#[test]
fn record_identity_mismatch_fails_without_leaking_values() {
    let mut record_definition =
        side_effect_record(side_effect_id("side-effect/wrong-record")).into_definition_for_test();
    record_definition.workflow_version =
        WorkflowVersion::new("v99").expect("valid workflow version");
    let mismatched_record =
        SideEffectRecord::new(record_definition).expect("valid mismatched record");
    let mut input = base_input();
    input.side_effect_records = vec![mismatched_record];

    let error = discover_side_effect_references(&input).expect_err("identity mismatch rejected");

    assert_eq!(error.code(), "side_effect_discovery.identity_mismatch");
    assert!(!error.to_string().contains("v99"));
    assert!(!error.to_string().contains(run_id().as_str()));
}

#[test]
fn debug_output_does_not_leak_ids_or_identity_values() {
    let mut input = base_input();
    input.explicit_side_effect_ids = vec![side_effect_id("side-effect/debug-reference")];
    let result = discover_side_effect_references(&input).expect("discovery succeeds");

    let input_debug = format!("{input:?}");
    let result_debug = format!("{result:?}");
    let reference_debug = format!("{:?}", result.references()[0]);

    assert!(!input_debug.contains("side-effect/debug-reference"));
    assert!(!input_debug.contains(workflow_id().as_str()));
    assert!(!input_debug.contains(run_id().as_str()));
    assert!(!result_debug.contains("side-effect/debug-reference"));
    assert!(!reference_debug.contains("side-effect/debug-reference"));
}

#[test]
fn store_backed_discovery_returns_persisted_records_for_requested_run() {
    let (backend, root) = local_backend();
    let first = side_effect_record(side_effect_id("side-effect/store-a"));
    let second = side_effect_record(side_effect_id("side-effect/store-b"));
    backend
        .write_side_effect_record(&second)
        .expect("second record written");
    backend
        .write_side_effect_record(&first)
        .expect("first record written");
    let input = base_store_input();

    let result =
        discover_side_effect_references_from_store(&backend, &input).expect("discovery succeeds");

    let ids = result
        .references()
        .iter()
        .map(|reference| reference.side_effect_id().as_str())
        .collect::<Vec<_>>();
    assert_eq!(ids, vec!["side-effect/store-a", "side-effect/store-b"]);
    assert!(result
        .references()
        .iter()
        .all(|reference| reference.source() == SideEffectDiscoverySource::SideEffectRecord));
    assert_eq!(result.missing_record_count(), 0);
    fs::remove_dir_all(root).expect("cleanup local backend");
}

#[test]
fn store_backed_discovery_preserves_explicit_source_priority() {
    let (backend, root) = local_backend();
    let id = side_effect_id("side-effect/shared-explicit");
    backend
        .write_side_effect_record(&side_effect_record(id.clone()))
        .expect("record written");
    let mut input = base_store_input();
    input.explicit_side_effect_ids = vec![id];

    let result =
        discover_side_effect_references_from_store(&backend, &input).expect("discovery succeeds");

    assert_eq!(result.references().len(), 1);
    assert_eq!(
        result.references()[0].source(),
        SideEffectDiscoverySource::ExplicitInput
    );
    assert_eq!(result.missing_record_count(), 0);
    fs::remove_dir_all(root).expect("cleanup local backend");
}

#[test]
fn store_backed_discovery_preserves_workflow_event_priority_over_record_source() {
    let (backend, root) = local_backend();
    let id = side_effect_id("side-effect/shared-event");
    backend
        .write_side_effect_record(&side_effect_record(id.clone()))
        .expect("record written");
    let mut input = base_store_input();
    input.workflow_events = vec![side_effect_event(1, id, SideEffectLifecycleState::Proposed)];

    let result =
        discover_side_effect_references_from_store(&backend, &input).expect("discovery succeeds");

    assert_eq!(result.references().len(), 1);
    assert_eq!(
        result.references()[0].source(),
        SideEffectDiscoverySource::WorkflowEvent
    );
    assert_eq!(result.missing_record_count(), 0);
    fs::remove_dir_all(root).expect("cleanup local backend");
}

#[test]
fn store_backed_discovery_optional_missing_records_return_bounded_count() {
    let (backend, root) = local_backend();
    let mut input = base_store_input();
    input.explicit_side_effect_ids = vec![side_effect_id("side-effect/optional-missing")];

    let result =
        discover_side_effect_references_from_store(&backend, &input).expect("discovery succeeds");

    assert_eq!(result.references().len(), 1);
    assert_eq!(result.missing_record_count(), 1);
    assert!(!result.records_required());
    fs::remove_dir_all(root).expect("cleanup local backend");
}

#[test]
fn store_backed_discovery_required_missing_records_fail_closed_without_leaking_id() {
    let (backend, root) = local_backend();
    let mut input = base_store_input();
    input.explicit_side_effect_ids = vec![side_effect_id("side-effect/required-missing")];
    input.require_records = true;

    let error = discover_side_effect_references_from_store(&backend, &input)
        .expect_err("missing record rejected");

    assert_eq!(error.code(), "side_effect_discovery.record_missing");
    assert!(!error.to_string().contains("side-effect/required-missing"));
    fs::remove_dir_all(root).expect("cleanup local backend");
}

#[test]
fn store_backed_discovery_store_identity_error_is_mapped_without_leaking_values() {
    let store = FailingSideEffectStore {
        code: "side_effect_record.read.identity_mismatch",
        message: "secret workflow/run id sk-store-secret",
    };
    let input = base_store_input();

    let error = discover_side_effect_references_from_store(&store, &input)
        .expect_err("identity mismatch rejected");

    assert_eq!(error.code(), "side_effect_discovery.identity_mismatch");
    assert!(!error.to_string().contains("sk-store-secret"));
    assert!(!error.to_string().contains(run_id().as_str()));
}

#[test]
fn store_backed_discovery_store_corrupt_error_is_mapped_without_leaking_values() {
    let store = FailingSideEffectStore {
        code: "side_effect_record.read.corrupt",
        message: "corrupt side-effect record sk-store-secret",
    };
    let input = base_store_input();

    let error = discover_side_effect_references_from_store(&store, &input)
        .expect_err("corrupt store rejected");

    assert_eq!(error.code(), "side_effect_discovery.record_corrupt");
    assert!(!error.to_string().contains("sk-store-secret"));
    assert!(!error.to_string().contains(run_id().as_str()));
}

#[test]
fn store_backed_discovery_store_read_failure_is_mapped_without_leaking_values() {
    let store = FailingSideEffectStore {
        code: "state.local.path.failed",
        message: "/tmp/sk-store-secret",
    };
    let input = base_store_input();

    let error = discover_side_effect_references_from_store(&store, &input)
        .expect_err("store read failure rejected");

    assert_eq!(error.code(), "side_effect_discovery.store_read_failed");
    assert!(!error.to_string().contains("sk-store-secret"));
    assert!(!error.to_string().contains("/tmp"));
}

#[test]
fn store_backed_discovery_debug_output_does_not_leak_ids_or_identity_values() {
    let mut input = base_store_input();
    input.explicit_side_effect_ids = vec![side_effect_id("side-effect/store-debug")];

    let input_debug = format!("{input:?}");

    assert!(!input_debug.contains("side-effect/store-debug"));
    assert!(!input_debug.contains(workflow_id().as_str()));
    assert!(!input_debug.contains(run_id().as_str()));
    assert!(input_debug.contains("explicit_side_effect_id_count"));
}

struct FailingSideEffectStore {
    code: &'static str,
    message: &'static str,
}

impl SideEffectRecordStore for FailingSideEffectStore {
    fn write_side_effect_record(&self, _record: &SideEffectRecord) -> Result<(), WorkflowOsError> {
        Err(WorkflowOsError::new(
            WorkflowOsErrorKind::InvalidState,
            self.code,
            self.message,
        ))
    }

    fn read_side_effect_record(
        &self,
        _side_effect_id: &SideEffectId,
    ) -> Result<Option<SideEffectRecord>, WorkflowOsError> {
        Err(WorkflowOsError::new(
            WorkflowOsErrorKind::InvalidState,
            self.code,
            self.message,
        ))
    }

    fn list_side_effect_records(
        &self,
        _run_id: &WorkflowRunId,
    ) -> Result<Vec<SideEffectRecord>, WorkflowOsError> {
        Err(WorkflowOsError::new(
            WorkflowOsErrorKind::InvalidState,
            self.code,
            self.message,
        ))
    }

    fn list_side_effect_records_for_workflow_run(
        &self,
        _workflow_id: &WorkflowId,
        _run_id: &WorkflowRunId,
    ) -> Result<Vec<SideEffectRecord>, WorkflowOsError> {
        Err(WorkflowOsError::new(
            WorkflowOsErrorKind::InvalidState,
            self.code,
            self.message,
        ))
    }
}

trait SideEffectRecordTestExt {
    fn into_definition_for_test(self) -> SideEffectRecordDefinition;
}

impl SideEffectRecordTestExt for SideEffectRecord {
    fn into_definition_for_test(self) -> SideEffectRecordDefinition {
        SideEffectRecordDefinition {
            side_effect_id: self.side_effect_id().clone(),
            lifecycle_state: self.lifecycle_state(),
            target: self.target().clone(),
            capability: self.capability(),
            authority: self.authority().clone(),
            actor: Some(ActorId::new("operator/reviewer").expect("valid actor")),
            system_actor: None,
            workflow_id: self.workflow_id().clone(),
            workflow_version: self.workflow_version().clone(),
            schema_version: self.schema_version().clone(),
            spec_hash: self.spec_hash().clone(),
            run_id: self.run_id().clone(),
            step_id: Some(StepId::new("step/implementation").expect("valid step id")),
            skill_id: Some(SkillId::new("skill/implementation").expect("valid skill id")),
            skill_version: Some(SkillVersion::new("v1").expect("valid skill version")),
            adapter_id: None,
            adapter_kind: None,
            integration_id: None,
            idempotency: SideEffectIdempotencyBinding::new(
                IdempotencyKey::new("idem/side-effect-record-copy").expect("valid idempotency key"),
                SideEffectIdempotencyScope::Run,
                None,
                None,
            )
            .expect("valid idempotency"),
            references: Vec::new(),
            outcome_reference: None,
            created_at: timestamp(),
            updated_at: Some(timestamp()),
            correlation_id: None,
            summary: Some("bounded side-effect summary".to_owned()),
            reason_codes: Vec::new(),
            sensitivity: SideEffectSensitivity::Confidential,
            redaction: RedactionMetadata::empty(),
        }
    }
}
