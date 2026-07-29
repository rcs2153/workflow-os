#![allow(clippy::expect_used, clippy::panic, clippy::too_many_lines)]

//! Executable `PostgreSQL` shared-state conformance.
//!
//! Local runs skip when `WORKFLOW_OS_TEST_POSTGRES_URL` is absent. CI sets
//! `WORKFLOW_OS_REQUIRE_POSTGRES_TESTS=1`, making absence a hard failure.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;

use postgres::{Client, Config, NoTls};
use workflow_core::{
    build_immutable_run_bundle, compute_approval_presentation_content_hash,
    transition_side_effect_to_attempted, transition_side_effect_to_completed, ActorId, AdapterId,
    AdapterKind, ApprovalDecision, ApprovalDecisionKind, ApprovalDecisionProofEnforcementMode,
    ApprovalDecisionProofMarker, ApprovalDecisionProofMarkerDefinition,
    ApprovalDecisionProofValidationPolicy, ApprovalPresentationChannel, ApprovalPresentationId,
    ApprovalPresentationRecord, ApprovalPresentationRecordDefinition,
    ApprovalPresentationRecordStore, ApprovalPresentationSensitivity, ApprovalRequest,
    ApprovalStore, CorrelationId, DurableStateBackendKind, DurableStateCapability,
    DurableStateContractProvider, DurableStateSupport, DurableStateTransactionKind, EventId,
    EventLogStore, EventSequenceNumber, IdempotencyKey, IdempotencyResult, IdempotencyStore,
    IdempotencyWrite, ImmutableRunBundleBuildRequest, ImmutableRunBundleExecutionPosture,
    ImmutableRunBundleHandlerPosture, ImmutableRunBundleHandlerReference, ImmutableRunBundleId,
    ImmutableRunBundleReferencePosture, ImmutableRunBundleSensitivity, ImmutableRunBundleVersion,
    IntegrationId, LockStore, PostgresAuthoritativeProjectionRequest, PostgresConnectionFactory,
    PostgresLeaseAcquireRequest, PostgresLeaseKey, PostgresNoTlsConnectionFactory,
    PostgresRecordApprovalDecisionRequest, PostgresRecordExternalOutcomeRequest,
    PostgresReserveIntentRequest, PostgresSharedRunConsumerRequest, PostgresStateBackend,
    PostgresTransitionSideEffectRequest, RedactionMetadata, SchemaVersion,
    SideEffectAttemptTransitionInput, SideEffectAuthority, SideEffectAuthorityDecision,
    SideEffectCapability, SideEffectCompleteTransitionInput, SideEffectId,
    SideEffectIdempotencyBinding, SideEffectIdempotencyScope, SideEffectLifecycleState,
    SideEffectOutcomeReference, SideEffectOutcomeReferenceKind, SideEffectRecord,
    SideEffectRecordDefinition, SideEffectRecordStore, SideEffectReference,
    SideEffectReferenceKind, SideEffectSensitivity, SideEffectTargetKind,
    SideEffectTargetReference, SideEffectWorkflowEvent, SideEffectWorkflowEventDefinition, SkillId,
    SkillVersion, SpecContentHash, StateBackend, StepId, Timestamp, WorkflowId, WorkflowOsError,
    WorkflowRun, WorkflowRunEvent, WorkflowRunEventKind, WorkflowRunId, WorkflowVersion,
    SUPPORTED_SCHEMA_VERSION,
};

struct UnexpectedConnectionFactory;

impl PostgresConnectionFactory for UnexpectedConnectionFactory {
    fn connect(&self) -> Result<Client, WorkflowOsError> {
        panic!("invalid lease TTL must fail before opening PostgreSQL");
    }
}

#[test]
fn postgresql_lease_ttl_rejects_sub_millisecond_before_connection() {
    let backend = PostgresStateBackend::new(Arc::new(UnexpectedConnectionFactory));
    let key = PostgresLeaseKey::new("run/postgres-invalid-ttl").expect("lease key");
    let owner = ActorId::new("worker/postgres-invalid-ttl").expect("owner");
    let error = backend
        .acquire_fenced_lease(PostgresLeaseAcquireRequest {
            key: &key,
            owner: &owner,
            ttl: Duration::from_nanos(999_999),
        })
        .expect_err("sub-millisecond TTL rejected");
    assert_eq!(error.code(), "postgres_state.lease_ttl.invalid");
}

#[test]
fn postgresql_backend_proves_shared_state_milestone() {
    let Some(config) = test_config() else {
        return;
    };
    reset_schema(&config);
    let backend = PostgresStateBackend::new(Arc::new(PostgresNoTlsConnectionFactory::new(
        config.clone(),
    )));
    backend.initialize_schema().expect("managed schema");

    assert_contract(&backend);
    prove_common_store_contract(&backend);
    prove_competing_event_append(&config);
    prove_fenced_leases(&backend);
    prove_competing_idempotency_intent(&backend);
    prove_atomic_side_effect_families(&backend);
    prove_atomic_approval_family(&backend);
    prove_immutable_bundle_family(&backend);
    prove_authoritative_projection_and_shared_consumer(&backend);
    prove_projection_rebuild(&backend);
    prove_corrupt_payload_nonleak(&config, &backend);
    prove_schema_fail_closed(&config, &backend);

    let health = backend.health_check().expect("health");
    assert!(health.healthy);
    assert_eq!(health.backend, "postgresql");
    assert!(!format!("{backend:?}").contains("postgres://"));
}

#[test]
fn restored_postgresql_database_passes_integrity_rehearsal() {
    let Ok(value) = std::env::var("WORKFLOW_OS_RECOVERY_POSTGRES_URL") else {
        return;
    };
    let config: Config = value.parse().expect("recovery PostgreSQL URL parses");
    let backend = PostgresStateBackend::new(Arc::new(PostgresNoTlsConnectionFactory::new(config)));
    let initialized = backend.initialize_schema().expect("restored schema");
    assert!(initialized.healthy());
    assert!(!initialized.recovery_required());
    let plan = backend.plan_projection_rebuild().expect("restored plan");
    assert!(!plan.run_ids().is_empty());
    let result = backend
        .rebuild_projections(&plan)
        .expect("restored projection rebuild");
    assert_eq!(result.checked_run_count(), plan.run_ids().len());
    assert_eq!(result.rebuilt_snapshot_count(), plan.run_ids().len());
    assert!(backend
        .read_immutable_run_bundle(
            &WorkflowRunId::new("run-postgres-bundle").expect("restored run id")
        )
        .expect("read restored bundle")
        .is_some());
}

fn test_config() -> Option<Config> {
    let value = std::env::var("WORKFLOW_OS_TEST_POSTGRES_URL").ok();
    assert!(
        value.is_some() || std::env::var_os("WORKFLOW_OS_REQUIRE_POSTGRES_TESTS").is_none(),
        "WORKFLOW_OS_TEST_POSTGRES_URL is required"
    );
    value.map(|value| value.parse().expect("test PostgreSQL URL parses"))
}

fn reset_schema(config: &Config) {
    let mut client = config.connect(NoTls).expect("connect test PostgreSQL");
    client
        .batch_execute("DROP SCHEMA IF EXISTS workflow_os CASCADE")
        .expect("reset test schema");
}

fn assert_contract(backend: &PostgresStateBackend) {
    let contract = backend.durable_state_contract().expect("contract");
    assert_eq!(
        contract.backend_kind(),
        DurableStateBackendKind::SharedPostgresql
    );
    assert!(DurableStateTransactionKind::all()
        .iter()
        .all(|kind| contract.transaction_support(*kind) == DurableStateSupport::Supported));
    for capability in [
        DurableStateCapability::CrossRecordAtomicCommit,
        DurableStateCapability::CompareAndSetRevision,
        DurableStateCapability::ExpiringFencedLease,
        DurableStateCapability::ManagedSchemaMigration,
        DurableStateCapability::VerifiedBackupRestore,
        DurableStateCapability::SharedWorkerConcurrency,
    ] {
        assert!(contract.supports_capability(capability));
    }
}

fn prove_common_store_contract(backend: &PostgresStateBackend) {
    let events = event_sequence("common");
    backend.append_event(&events[0]).expect("created");
    backend.append_event(&events[1]).expect("validated");
    assert_eq!(
        backend.read_events(&events[0].run_id).expect("events"),
        events[..2]
    );

    let key = IdempotencyKey::new("postgres/common/idempotency").expect("idempotency key");
    let first = IdempotencyResult {
        result_ref: "result/first".to_owned(),
    };
    assert_eq!(
        backend
            .record_idempotency_result(&key, first.clone())
            .expect("first idempotency write"),
        IdempotencyWrite::FirstWrite(first.clone())
    );
    assert_eq!(
        backend
            .record_idempotency_result(
                &key,
                IdempotencyResult {
                    result_ref: "result/ignored".to_owned(),
                },
            )
            .expect("idempotent replay"),
        IdempotencyWrite::Duplicate(first)
    );

    let owner = ActorId::new("worker/postgres-common").expect("owner");
    let lease = backend
        .acquire_lock("postgres/common/lock", &owner)
        .expect("lock");
    let error = backend
        .acquire_lock("postgres/common/lock", &owner)
        .expect_err("contended");
    assert_eq!(error.code(), "state.lock_contended");
    backend.release_lock(&lease).expect("release");
}

fn prove_competing_event_append(config: &Config) {
    let backend = PostgresStateBackend::new(Arc::new(PostgresNoTlsConnectionFactory::new(
        config.clone(),
    )));
    let events = event_sequence("concurrent");
    backend.append_event(&events[0]).expect("created");
    let mut first = events[1].clone();
    first.event_id = EventId::new("event-postgres-concurrent-first").expect("event");
    let mut second = events[1].clone();
    second.event_id = EventId::new("event-postgres-concurrent-second").expect("event");
    let barrier = Arc::new(Barrier::new(3));
    let one = spawn_append(backend.clone(), first, Arc::clone(&barrier));
    let two = spawn_append(backend.clone(), second, Arc::clone(&barrier));
    barrier.wait();
    let results = [
        one.join().expect("first worker"),
        two.join().expect("second worker"),
    ];
    assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
    assert_eq!(
        backend
            .read_events(&events[0].run_id)
            .expect("authoritative events")
            .len(),
        2
    );
}

fn spawn_append(
    backend: PostgresStateBackend,
    event: WorkflowRunEvent,
    barrier: Arc<Barrier>,
) -> thread::JoinHandle<Result<(), workflow_core::WorkflowOsError>> {
    thread::spawn(move || {
        barrier.wait();
        backend.append_event(&event)
    })
}

fn prove_fenced_leases(backend: &PostgresStateBackend) {
    let key = PostgresLeaseKey::new("run/postgres-lease").expect("lease key");
    let first_owner = ActorId::new("worker/postgres-one").expect("owner");
    let second_owner = ActorId::new("worker/postgres-two").expect("owner");
    let first = backend
        .acquire_fenced_lease(PostgresLeaseAcquireRequest {
            key: &key,
            owner: &first_owner,
            ttl: Duration::from_secs(5),
        })
        .expect("first lease");
    let error = backend
        .acquire_fenced_lease(PostgresLeaseAcquireRequest {
            key: &key,
            owner: &second_owner,
            ttl: Duration::from_secs(5),
        })
        .expect_err("live competing owner rejected");
    assert_eq!(error.code(), "postgres_state.lease.contended");
    let renewed = backend
        .acquire_fenced_lease(PostgresLeaseAcquireRequest {
            key: &key,
            owner: &first_owner,
            ttl: Duration::from_secs(5),
        })
        .expect("same owner renews with new fence");
    assert!(renewed.fence_token() > first.fence_token());
    let stale_release = backend
        .release_fenced_lease(&first)
        .expect_err("old fence cannot release");
    assert_eq!(stale_release.code(), "postgres_state.lease.stale");
    backend
        .release_fenced_lease(&renewed)
        .expect("current fence releases");

    let crash_key = PostgresLeaseKey::new("run/postgres-crash-takeover").expect("lease key");
    let abandoned = backend
        .acquire_fenced_lease(PostgresLeaseAcquireRequest {
            key: &crash_key,
            owner: &first_owner,
            ttl: Duration::from_millis(1),
        })
        .expect("abandoned lease");
    thread::sleep(Duration::from_millis(20));
    let takeover = backend
        .acquire_fenced_lease(PostgresLeaseAcquireRequest {
            key: &crash_key,
            owner: &second_owner,
            ttl: Duration::from_secs(5),
        })
        .expect("expired lease takeover");
    assert!(takeover.fence_token() > abandoned.fence_token());

    let events = event_sequence("stale-fence");
    let run = WorkflowRun::rehydrate(&events[..1]).expect("created run");
    let stale_error = backend
        .commit_authoritative_result_and_projection(PostgresAuthoritativeProjectionRequest {
            event: &events[0],
            snapshot: &run.snapshot,
            expected_snapshot_revision: None,
            lease: Some(&abandoned),
        })
        .expect_err("expired holder cannot commit");
    assert_eq!(stale_error.code(), "postgres_state.lease.stale");
    assert!(backend
        .read_events(&events[0].run_id)
        .expect("stale fence leaves no event")
        .is_empty());
    backend
        .release_fenced_lease(&takeover)
        .expect("takeover lease releases");
}

fn prove_competing_idempotency_intent(backend: &PostgresStateBackend) {
    let events = event_sequence("idempotency-race");
    for event in &events[..3] {
        backend.append_event(event).expect("run setup");
    }
    let proposed = proposed_side_effect(&events[0]);
    let proposed_event = side_effect_event(
        &events[0],
        4,
        "proposed",
        WorkflowRunEventKind::SideEffectProposed(Box::new(
            side_effect_payload(&proposed).expect("payload"),
        )),
    );
    let key = IdempotencyKey::new("postgres/idempotency-race/intent").expect("idempotency");
    let barrier = Arc::new(Barrier::new(3));
    let mut workers = Vec::new();
    for _ in 0..2 {
        let backend = backend.clone();
        let barrier = Arc::clone(&barrier);
        let key = key.clone();
        let proposed = proposed.clone();
        let proposed_event = proposed_event.clone();
        workers.push(thread::spawn(move || {
            barrier.wait();
            backend.reserve_idempotency_and_record_intent(PostgresReserveIntentRequest {
                idempotency_key: &key,
                idempotency_result: IdempotencyResult {
                    result_ref: "intent/race-winner".to_owned(),
                },
                side_effect: &proposed,
                event: &proposed_event,
            })
        }));
    }
    barrier.wait();
    let results = workers
        .into_iter()
        .map(|worker| worker.join().expect("idempotency worker"))
        .collect::<Vec<_>>();
    let result_postures = results
        .iter()
        .map(|result| match result {
            Ok(IdempotencyWrite::FirstWrite(_)) => "first_write".to_owned(),
            Ok(IdempotencyWrite::Duplicate(_)) => "duplicate".to_owned(),
            Err(error) => format!("error:{}", error.code()),
        })
        .collect::<Vec<_>>();
    assert_eq!(
        results
            .iter()
            .filter(|result| matches!(result, Ok(IdempotencyWrite::FirstWrite(_))))
            .count(),
        1,
        "unexpected bounded idempotency result postures: {result_postures:?}"
    );
    assert_eq!(
        results
            .iter()
            .filter(|result| matches!(result, Ok(IdempotencyWrite::Duplicate(_))))
            .count(),
        1,
        "unexpected bounded idempotency result postures: {result_postures:?}"
    );
    assert_eq!(
        backend
            .read_events(&events[0].run_id)
            .expect("single intent event")
            .len(),
        4
    );
}

fn prove_atomic_side_effect_families(backend: &PostgresStateBackend) {
    let events = event_sequence("side-effect");
    for event in &events[..3] {
        backend.append_event(event).expect("run setup");
    }
    let proposed = proposed_side_effect(&events[0]);
    let proposed_event = side_effect_event(
        &events[0],
        4,
        "proposed",
        WorkflowRunEventKind::SideEffectProposed(Box::new(
            side_effect_payload(&proposed).expect("payload"),
        )),
    );
    let idempotency_key = IdempotencyKey::new("postgres/side-effect/intent").expect("idempotency");
    assert!(matches!(
        backend
            .reserve_idempotency_and_record_intent(PostgresReserveIntentRequest {
                idempotency_key: &idempotency_key,
                idempotency_result: IdempotencyResult {
                    result_ref: "intent/reserved".to_owned(),
                },
                side_effect: &proposed,
                event: &proposed_event,
            })
            .expect("intent transaction"),
        IdempotencyWrite::FirstWrite(_)
    ));
    let stored = backend
        .read_revisioned_side_effect(proposed.side_effect_id())
        .expect("read side effect")
        .expect("stored side effect");
    let transitioned = transition_side_effect_to_attempted(SideEffectAttemptTransitionInput {
        prior_record: stored.value(),
        transitioned_at: timestamp(),
        summary: Some("attempted under PostgreSQL transaction".to_owned()),
        additional_references: Vec::new(),
        evidence_reference_count: 0,
    })
    .expect("valid transition");
    let attempted_event = side_effect_event(
        &events[0],
        5,
        "attempted",
        WorkflowRunEventKind::SideEffectAttempted(Box::new(transitioned.event().clone())),
    );
    let barrier = Arc::new(Barrier::new(3));
    let mut workers = Vec::new();
    for worker_index in 0..2 {
        let backend = backend.clone();
        let barrier = Arc::clone(&barrier);
        let transitioned = transitioned.record().clone();
        let mut attempted_event = attempted_event.clone();
        attempted_event.event_id = EventId::new(format!(
            "event-postgres-side-effect-attempted-{worker_index}"
        ))
        .expect("event");
        let expected_revision = stored.revision();
        workers.push(thread::spawn(move || {
            barrier.wait();
            backend.transition_side_effect(PostgresTransitionSideEffectRequest {
                expected_revision,
                side_effect: &transitioned,
                event: &attempted_event,
            })
        }));
    }
    barrier.wait();
    let results = workers
        .into_iter()
        .map(|worker| worker.join().expect("SideEffect worker"))
        .collect::<Vec<_>>();
    assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
    let revision = results
        .into_iter()
        .find_map(Result::ok)
        .expect("one committed SideEffect revision");
    assert_eq!(revision.get(), 2);
    assert_eq!(
        backend
            .read_side_effect_record(proposed.side_effect_id())
            .expect("read")
            .expect("record")
            .lifecycle_state(),
        SideEffectLifecycleState::Attempted
    );
    let completed = transition_side_effect_to_completed(SideEffectCompleteTransitionInput {
        prior_record: transitioned.record(),
        transitioned_at: timestamp(),
        outcome_reference: SideEffectOutcomeReference::new(
            SideEffectOutcomeReferenceKind::Outcome,
            "provider-outcome/postgres-conformance",
        )
        .expect("outcome reference"),
        summary: Some("bounded external outcome recorded".to_owned()),
        additional_references: Vec::new(),
        evidence_reference_count: 1,
    })
    .expect("valid completed transition");
    let completed_event = side_effect_event(
        &events[0],
        6,
        "completed",
        WorkflowRunEventKind::SideEffectCompleted(Box::new(completed.event().clone())),
    );
    let completed_revision = backend
        .record_external_operation_outcome(PostgresRecordExternalOutcomeRequest {
            expected_revision: revision,
            side_effect: completed.record(),
            event: &completed_event,
        })
        .expect("atomic external outcome");
    assert_eq!(completed_revision.get(), 3);
}

fn prove_atomic_approval_family(backend: &PostgresStateBackend) {
    let events = event_sequence("approval");
    for event in &events[..3] {
        backend.append_event(event).expect("run setup");
    }
    let request = approval_request(&events[0], None);
    let requested_event = event(
        &events[0],
        4,
        "approval-requested",
        WorkflowRunEventKind::ApprovalRequested(Box::new(request.clone())),
    );
    backend
        .append_event(&requested_event)
        .expect("approval requested");
    backend
        .save_approval_request(&request)
        .expect("approval projection");
    let presentation = approval_presentation(&request);
    backend
        .write_approval_presentation_record(&presentation)
        .expect("approval presentation proof");
    let decision = ApprovalDecision {
        approval_id: request.approval_id.clone(),
        actor: ActorId::new("user/postgres-reviewer").expect("actor"),
        decided_at: timestamp(),
        decision: ApprovalDecisionKind::Granted,
        reason: "bounded PostgreSQL conformance approval".to_owned(),
        correlation_id: request.correlation_id.clone(),
        proof_marker: Some(
            ApprovalDecisionProofMarker::new(ApprovalDecisionProofMarkerDefinition {
                enforcement_mode:
                    ApprovalDecisionProofEnforcementMode::ApprovalPresentationRequired,
                presentation_id: presentation.presentation_id().clone(),
                presentation_content_hash: presentation.content_hash().clone(),
                proof_validated_at: timestamp(),
                proof_validation_policy:
                    ApprovalDecisionProofValidationPolicy::ApprovalPresentationRequestMatch,
                proof_age_ms: Some(0),
                proof_freshness_limit_ms: Some(60_000),
                proof_record_sensitivity: presentation.sensitivity(),
                redaction: RedactionMetadata::empty(),
            })
            .expect("proof marker"),
        ),
    };
    let decided = approval_request(&events[0], Some(decision.clone()));
    let granted_event = event(
        &events[0],
        5,
        "approval-granted",
        WorkflowRunEventKind::ApprovalGranted(decision),
    );
    let barrier = Arc::new(Barrier::new(3));
    let mut workers = Vec::new();
    for worker_index in 0..2 {
        let backend = backend.clone();
        let barrier = Arc::clone(&barrier);
        let decided = decided.clone();
        let presentation = presentation.clone();
        let mut granted_event = granted_event.clone();
        granted_event.event_id =
            EventId::new(format!("event-postgres-approval-granted-{worker_index}")).expect("event");
        workers.push(thread::spawn(move || {
            barrier.wait();
            backend.record_approval_decision(PostgresRecordApprovalDecisionRequest {
                approval: &decided,
                presentation: &presentation,
                event: &granted_event,
            })
        }));
    }
    barrier.wait();
    let results = workers
        .into_iter()
        .map(|worker| worker.join().expect("approval worker"))
        .collect::<Vec<_>>();
    assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
    assert_eq!(
        results
            .iter()
            .filter_map(|result| result.as_ref().err())
            .filter(|error| error.code() == "postgres_state.approval.already_decided")
            .count(),
        1
    );
    assert!(backend
        .load_approval_request(&request.approval_id)
        .expect("read approval")
        .expect("approval")
        .decision
        .is_some());
}

fn prove_immutable_bundle_family(backend: &PostgresStateBackend) {
    let project = TestProject::new();
    project.write_valid_project();
    let loaded = workflow_core::load_project(project.path());
    assert!(!loaded.has_errors(), "{:?}", loaded.diagnostics);
    let bundle = loaded.bundle.expect("project");
    let validation = workflow_core::validate_project_bundle(&bundle);
    assert!(!validation.has_errors(), "{:?}", validation.diagnostics);
    let workflow_id = WorkflowId::new("postgres/build").expect("workflow");
    let build = build_immutable_run_bundle(ImmutableRunBundleBuildRequest {
        project: &bundle,
        workflow_id: &workflow_id,
        bundle_id: ImmutableRunBundleId::new("bundle/postgres").expect("bundle id"),
        bundle_version: ImmutableRunBundleVersion::new("v1").expect("bundle version"),
        run_id: WorkflowRunId::new("run-postgres-bundle").expect("run"),
        resolved_execution_context_hash: SpecContentHash::from_text("resolved context"),
        execution_posture: ImmutableRunBundleExecutionPosture::new(
            vec![StepId::new("inspect").expect("step")],
            ImmutableRunBundleReferencePosture::NotSupplied,
            ImmutableRunBundleReferencePosture::NotSupplied,
            ImmutableRunBundleReferencePosture::CommittedReference,
        )
        .expect("posture"),
        handlers: vec![ImmutableRunBundleHandlerReference {
            skill_id: SkillId::new("local/check").expect("skill"),
            skill_version: SkillVersion::new("v1").expect("version"),
            posture: ImmutableRunBundleHandlerPosture::RegisteredUnattested,
        }],
        created_at: timestamp(),
        created_by: ActorId::new("system/postgres-test").expect("actor"),
        sensitivity: ImmutableRunBundleSensitivity::Internal,
        redaction_required: true,
    })
    .expect("build immutable bundle");
    let build = Arc::new(build);
    let barrier = Arc::new(Barrier::new(3));
    let mut workers = Vec::new();
    for _ in 0..2 {
        let backend = backend.clone();
        let barrier = Arc::clone(&barrier);
        let build = Arc::clone(&build);
        workers.push(thread::spawn(move || {
            barrier.wait();
            backend.publish_immutable_run_bundle(&build)
        }));
    }
    barrier.wait();
    let results = workers
        .into_iter()
        .map(|worker| worker.join().expect("bundle worker"))
        .collect::<Vec<_>>();
    assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
    assert_eq!(
        results
            .iter()
            .filter_map(|result| result.as_ref().err())
            .filter(|error| error.code() == "postgres_state.bundle.manifest_exists")
            .count(),
        1
    );
    let stored = backend
        .read_immutable_run_bundle(build.manifest().run_id())
        .expect("read immutable bundle")
        .expect("stored bundle");
    assert_eq!(stored.manifest(), build.manifest());
    let duplicate = backend
        .publish_immutable_run_bundle(&build)
        .expect_err("run binding is create-only");
    assert_eq!(duplicate.code(), "postgres_state.bundle.manifest_exists");
}

fn prove_authoritative_projection_and_shared_consumer(backend: &PostgresStateBackend) {
    let events = event_sequence("projection");
    let run = WorkflowRun::rehydrate(&events[..1]).expect("created run");
    let revision = backend
        .commit_authoritative_result_and_projection(PostgresAuthoritativeProjectionRequest {
            event: &events[0],
            snapshot: &run.snapshot,
            expected_snapshot_revision: None,
            lease: None,
        })
        .expect("authoritative event and projection");
    assert_eq!(revision.get(), 1);

    let worker = ActorId::new("worker/postgres-consumer").expect("worker");
    let result = backend
        .consume_shared_run_event(PostgresSharedRunConsumerRequest {
            event: &events[1],
            worker: &worker,
            lease_ttl: Duration::from_secs(10),
        })
        .expect("shared consumer");
    assert_eq!(result.run().events.len(), 2);
    assert_eq!(result.snapshot_revision().get(), 2);
}

fn prove_projection_rebuild(backend: &PostgresStateBackend) {
    let plan = backend.plan_projection_rebuild().expect("rebuild plan");
    assert!(!plan.run_ids().is_empty());
    let result = backend
        .rebuild_projections(&plan)
        .expect("projection rebuild");
    assert_eq!(result.checked_run_count(), plan.run_ids().len());
    assert_eq!(result.rebuilt_snapshot_count(), plan.run_ids().len());
}

fn prove_corrupt_payload_nonleak(config: &Config, backend: &PostgresStateBackend) {
    let run_id = WorkflowRunId::new("run-postgres-common").expect("run");
    let mut client = config.connect(NoTls).expect("connect");
    let row = client
        .query_one(
            "SELECT payload FROM workflow_os.events
              WHERE run_id = $1 AND sequence_number = 1",
            &[&run_id.as_str()],
        )
        .expect("read canonical event");
    let canonical: String = row.get(0);
    let secret = "authorization=Bearer postgres-secret-value";
    client
        .execute(
            "UPDATE workflow_os.events SET payload = $1
              WHERE run_id = $2 AND sequence_number = 1",
            &[&secret, &run_id.as_str()],
        )
        .expect("corrupt payload");
    let error = backend
        .read_events(&run_id)
        .expect_err("corrupt canonical payload fails closed");
    assert_eq!(error.code(), "postgres_state.deserialization.failed");
    let rendered = format!("{error:?} {error}");
    assert!(!rendered.contains(secret));
    assert!(!rendered.contains("postgres-secret-value"));
    client
        .execute(
            "UPDATE workflow_os.events SET payload = $1
              WHERE run_id = $2 AND sequence_number = 1",
            &[&canonical, &run_id.as_str()],
        )
        .expect("restore canonical payload");
}

fn prove_schema_fail_closed(config: &Config, backend: &PostgresStateBackend) {
    let mut client = config.connect(NoTls).expect("connect");
    let secret_checksum = "private_key=postgres-schema-secret";
    client
        .execute(
            "UPDATE workflow_os.schema_metadata SET checksum = $1 WHERE singleton = TRUE",
            &[&secret_checksum],
        )
        .expect("change checksum");
    let error = backend
        .initialize_schema()
        .expect_err("checksum mismatch fails closed");
    assert_eq!(error.code(), "postgres_state.schema.incompatible");
    let rendered = format!("{error:?} {error}");
    assert!(!rendered.contains(secret_checksum));
    assert!(!rendered.contains("postgres-schema-secret"));
    client
        .execute(
            "UPDATE workflow_os.schema_metadata
                SET checksum = 'workflow-os-postgresql-v1',
                    recovery_required = TRUE
              WHERE singleton = TRUE",
            &[],
        )
        .expect("mark recovery required");
    let error = backend
        .initialize_schema()
        .expect_err("recovery-required schema fails closed");
    assert_eq!(error.code(), "postgres_state.schema.recovery_required");
    client
        .execute(
            "UPDATE workflow_os.schema_metadata
                SET recovery_required = FALSE
              WHERE singleton = TRUE",
            &[],
        )
        .expect("restore healthy schema");
}

fn event_sequence(name: &str) -> Vec<WorkflowRunEvent> {
    let run_id = WorkflowRunId::new(format!("run-postgres-{name}")).expect("run");
    let workflow_id = WorkflowId::new(format!("postgres/{name}")).expect("workflow");
    let base = WorkflowRunEvent {
        sequence_number: EventSequenceNumber::first(),
        event_id: EventId::new(format!("event-postgres-{name}-created")).expect("event"),
        timestamp: timestamp(),
        run_id,
        workflow_id,
        schema_version: SchemaVersion::new(SUPPORTED_SCHEMA_VERSION).expect("schema"),
        workflow_version: WorkflowVersion::new("v1").expect("version"),
        spec_content_hash: SpecContentHash::from_text(&format!("postgres-{name}-spec")),
        correlation_id: Some(
            CorrelationId::new(format!("correlation-postgres-{name}")).expect("correlation"),
        ),
        actor: Some(ActorId::new("system/postgres-test").expect("actor")),
        idempotency_key: None,
        kind: WorkflowRunEventKind::RunCreated {
            summary: None,
            immutable_run_bundle: None,
        },
    };
    vec![
        base.clone(),
        event(&base, 2, "validated", WorkflowRunEventKind::RunValidated),
        event(&base, 3, "started", WorkflowRunEventKind::RunStarted),
    ]
}

fn event(
    base: &WorkflowRunEvent,
    sequence: u64,
    suffix: &str,
    kind: WorkflowRunEventKind,
) -> WorkflowRunEvent {
    let mut event = base.clone();
    event.sequence_number = EventSequenceNumber::new(sequence).expect("sequence");
    event.event_id = EventId::new(format!("{}-{suffix}", base.event_id.as_str())).expect("event");
    event.kind = kind;
    event
}

fn side_effect_event(
    base: &WorkflowRunEvent,
    sequence: u64,
    suffix: &str,
    kind: WorkflowRunEventKind,
) -> WorkflowRunEvent {
    event(base, sequence, suffix, kind)
}

fn proposed_side_effect(base: &WorkflowRunEvent) -> SideEffectRecord {
    SideEffectRecord::new(SideEffectRecordDefinition {
        side_effect_id: SideEffectId::new(format!("side-effect/{}", base.run_id.as_str()))
            .expect("side effect"),
        lifecycle_state: SideEffectLifecycleState::Proposed,
        target: SideEffectTargetReference::new(
            SideEffectTargetKind::AdapterResource,
            "github/pull-request/test",
        )
        .expect("target"),
        capability: SideEffectCapability::GitHubWrite,
        authority: SideEffectAuthority::new(
            SideEffectAuthorityDecision::AllowedByPolicy,
            vec![SideEffectReference::new(
                SideEffectReferenceKind::PolicyDecision,
                "event/policy-pending",
            )
            .expect("reference")],
            Vec::new(),
        )
        .expect("authority"),
        actor: Some(ActorId::new("user/postgres-test").expect("actor")),
        system_actor: None,
        workflow_id: base.workflow_id.clone(),
        workflow_version: base.workflow_version.clone(),
        schema_version: base.schema_version.clone(),
        spec_hash: base.spec_content_hash.clone(),
        run_id: base.run_id.clone(),
        step_id: Some(StepId::new("write-comment").expect("step")),
        skill_id: Some(SkillId::new("github/comment").expect("skill")),
        skill_version: Some(SkillVersion::new("v1").expect("version")),
        adapter_id: Some(AdapterId::new("adapter/github").expect("adapter")),
        adapter_kind: Some(AdapterKind::GitHub),
        integration_id: Some(IntegrationId::new("integration/github-test").expect("integration")),
        idempotency: SideEffectIdempotencyBinding::new(
            IdempotencyKey::new(format!("side-effect/{}", base.run_id.as_str()))
                .expect("idempotency"),
            SideEffectIdempotencyScope::Run,
            None,
            None,
        )
        .expect("binding"),
        references: Vec::new(),
        outcome_reference: None,
        created_at: timestamp(),
        updated_at: None,
        correlation_id: base.correlation_id.clone(),
        summary: Some("bounded PostgreSQL side-effect intent".to_owned()),
        reason_codes: Vec::new(),
        sensitivity: SideEffectSensitivity::Confidential,
        redaction: RedactionMetadata::empty(),
    })
    .expect("side effect record")
}

fn side_effect_payload(
    record: &SideEffectRecord,
) -> Result<SideEffectWorkflowEvent, workflow_core::WorkflowOsError> {
    SideEffectWorkflowEvent::new(SideEffectWorkflowEventDefinition {
        side_effect_id: record.side_effect_id().clone(),
        lifecycle_state: record.lifecycle_state(),
        step_id: record.step_id().cloned(),
        skill_id: record.skill_id().cloned(),
        skill_version: record.skill_version().cloned(),
        correlation_id: record.correlation_id().cloned(),
        references: record.references().to_vec(),
        evidence_reference_count: 0,
        outcome_reference_count: 0,
        redaction: RedactionMetadata::empty(),
        sensitivity: SideEffectSensitivity::Confidential,
    })
}

fn approval_presentation(request: &ApprovalRequest) -> ApprovalPresentationRecord {
    let requested_action = "approve bounded PostgreSQL transaction";
    let work_summary = "record one approval decision under shared durable state";
    let approved_scope = "approval decision and authoritative event only";
    let strict_non_goals = vec!["no provider mutation".to_owned()];
    let expected_touched_surfaces = vec!["PostgreSQL approval state".to_owned()];
    let validation_expectations = vec!["approval proof must match request".to_owned()];
    let why_now = "prove shared approval transaction semantics";
    let next_action = "record the bounded decision";
    let channel = ApprovalPresentationChannel::Terminal;
    let sensitivity = ApprovalPresentationSensitivity::Internal;
    let content_hash = compute_approval_presentation_content_hash(
        &request.run_id,
        &request.approval_id,
        &request.workflow_id,
        Some(&request.workflow_version),
        Some(&request.schema_version),
        request.step_id.as_ref(),
        requested_action,
        work_summary,
        approved_scope,
        &strict_non_goals,
        &expected_touched_surfaces,
        &validation_expectations,
        why_now,
        next_action,
        &channel,
        sensitivity,
    )
    .expect("presentation content hash");
    ApprovalPresentationRecord::new(ApprovalPresentationRecordDefinition {
        presentation_id: ApprovalPresentationId::new(format!(
            "presentation/{}/postgres",
            request.run_id.as_str()
        ))
        .expect("presentation id"),
        run_id: request.run_id.clone(),
        approval_id: request.approval_id.clone(),
        workflow_id: request.workflow_id.clone(),
        workflow_version: Some(request.workflow_version.clone()),
        schema_version: Some(request.schema_version.clone()),
        step_id: request.step_id.clone(),
        requested_action: requested_action.to_owned(),
        work_summary: work_summary.to_owned(),
        approved_scope: approved_scope.to_owned(),
        strict_non_goals,
        expected_touched_surfaces,
        validation_expectations,
        why_now: why_now.to_owned(),
        next_action: next_action.to_owned(),
        presented_at: timestamp(),
        presented_by: ActorId::new("system/postgres-test").expect("actor"),
        channel,
        content_hash,
        redaction: RedactionMetadata::empty(),
        sensitivity,
    })
    .expect("approval presentation")
}

fn approval_request(
    base: &WorkflowRunEvent,
    decision: Option<ApprovalDecision>,
) -> ApprovalRequest {
    ApprovalRequest {
        approval_id: format!("approval/{}/review", base.run_id.as_str()),
        run_id: base.run_id.clone(),
        workflow_id: base.workflow_id.clone(),
        schema_version: base.schema_version.clone(),
        workflow_version: base.workflow_version.clone(),
        spec_content_hash: base.spec_content_hash.clone(),
        resolved_execution_context_hash: Some(SpecContentHash::from_text("resolved context")),
        step_id: Some(StepId::new("review").expect("step")),
        skill_id: Some(SkillId::new("local/review").expect("skill")),
        skill_version: Some(SkillVersion::new("v1").expect("version")),
        governance_approval_binding: None,
        requested_by: ActorId::new("system/postgres-test").expect("actor"),
        correlation_id: base.correlation_id.clone().expect("correlation"),
        idempotency_key: None,
        reason: "review PostgreSQL transaction".to_owned(),
        requested_at: timestamp(),
        expires_after: None,
        expires_at: None,
        decision,
    }
}

fn timestamp() -> Timestamp {
    Timestamp::parse_rfc3339("2026-07-29T12:00:00Z").expect("timestamp")
}

struct TestProject {
    root: PathBuf,
}

impl TestProject {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "workflow-os-postgres-bundle-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).expect("project root");
        Self { root }
    }

    fn path(&self) -> &Path {
        &self.root
    }

    fn write(&self, relative: &str, content: &str) {
        let path = self.root.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("parent");
        }
        fs::write(path, content).expect("write fixture");
    }

    fn write_valid_project(&self) {
        self.write(
            "workflow-os.yml",
            &format!(
                "schema_version: {SUPPORTED_SCHEMA_VERSION}\nproject:\n  id: postgres/project\n  name: PostgreSQL Project\n"
            ),
        );
        self.write(
            "workflows/build.workflow.yml",
            &format!(
                "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: postgres/build\nversion: v1\ndisplay_name: PostgreSQL Build\ntriggers:\n  - id: manual\n    kind: manual\nsteps:\n  - id: inspect\n    skill_ref:\n      id: local/check\n      version: v1\n    policy_requirements:\n      - id: local/read-only\n    terminal_behavior: fail_workflow\ncancellation_behavior: stop\naudit_requirements:\n  required: true\n  events: [RunCreated, RunCompleted]\n  store_references_only: true\nobservability_requirements:\n  metrics: [workflow_latency]\n  tracing: true\n  latency_tracking: true\n"
            ),
        );
        self.write(
            "skills/check.skill.yml",
            &format!(
                "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: local/check\nversion: v1\ndisplay_name: Local Check\ninput_contract:\n  fields:\n    - name: request\n      field_type: string\noutput_contract:\n  fields:\n    - name: summary\n      field_type: string\nfailure_modes:\n  - code: failed\n    description: Check failed.\n    retryable: false\naudit_requirements:\n  required: true\n  events: [SkillInvocationRequested]\n  store_references_only: true\nobservability_requirements:\n  metrics: [skill_latency]\n  tracing: true\n  latency_tracking: true\n"
            ),
        );
        self.write(
            "policies/read-only.policy.yml",
            &format!(
                "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: local/read-only\nname: Read Only\nrules:\n  - id: allow-local\n    effect: allow_local\n"
            ),
        );
    }
}

impl Drop for TestProject {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}
