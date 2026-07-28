#![allow(clippy::expect_used)]

//! Durable-state semantic contract and local-backend conformance tests.

use std::fs;
use std::sync::atomic::{AtomicU64, Ordering};

use workflow_core::{
    run_durable_state_conformance, ActorId, CorrelationId, DurableLeaseSemantics, DurableRevision,
    DurableStateBackendKind, DurableStateCapability, DurableStateConformanceFixture,
    DurableStateConformanceOutcome, DurableStateContractProvider, DurableStateSchemaMetadata,
    DurableStateSchemaPosture, DurableStateSemanticContract, DurableStateSupport,
    DurableStateTransactionKind, EventId, EventSequenceNumber, IdempotencyKey, LocalStateBackend,
    SchemaVersion, SpecContentHash, Timestamp, WorkflowId, WorkflowRunEvent, WorkflowRunEventKind,
    WorkflowRunId, WorkflowVersion,
};

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(1);

struct Fixture {
    backend: LocalStateBackend,
    created: WorkflowRunEvent,
    validated: WorkflowRunEvent,
}

impl Fixture {
    fn new() -> Self {
        let id = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
        let run_id = WorkflowRunId::new(format!("run-durable-contract-{id}")).expect("run id");
        let workflow_id =
            WorkflowId::new(format!("workflow/durable-contract-{id}")).expect("workflow id");
        let schema_version = SchemaVersion::new("workflowos.dev/v0").expect("schema version");
        let workflow_version = WorkflowVersion::new("v0").expect("workflow version");
        let spec_content_hash = SpecContentHash::from_text("durable state conformance fixture");
        let event = |sequence, kind| WorkflowRunEvent {
            sequence_number: EventSequenceNumber::new(sequence).expect("sequence"),
            event_id: EventId::new(format!("event-durable-contract-{id}-{sequence}"))
                .expect("event id"),
            timestamp: Timestamp::parse_rfc3339("2026-01-01T00:00:00Z").expect("timestamp"),
            run_id: run_id.clone(),
            workflow_id: workflow_id.clone(),
            schema_version: schema_version.clone(),
            workflow_version: workflow_version.clone(),
            spec_content_hash: spec_content_hash.clone(),
            correlation_id: Some(
                CorrelationId::new(format!("correlation-durable-{id}")).expect("correlation"),
            ),
            actor: Some(ActorId::new("system/conformance").expect("actor")),
            idempotency_key: None,
            kind,
        };
        let root = std::env::temp_dir().join(format!(
            "workflow-os-durable-contract-{}-{id}",
            std::process::id()
        ));
        if root.exists() {
            fs::remove_dir_all(&root).expect("stale fixture cleanup");
        }
        Self {
            backend: LocalStateBackend::new(root).expect("backend"),
            created: event(
                1,
                WorkflowRunEventKind::RunCreated {
                    summary: None,
                    immutable_run_bundle: None,
                },
            ),
            validated: event(2, WorkflowRunEventKind::RunValidated),
        }
    }

    fn conformance_fixture(&self) -> DurableStateConformanceFixture {
        DurableStateConformanceFixture::new(
            self.created.clone(),
            self.validated.clone(),
            IdempotencyKey::new(format!(
                "durable-conformance/{}",
                self.created.run_id.as_str()
            ))
            .expect("idempotency key"),
            ActorId::new("worker/conformance").expect("actor"),
        )
        .expect("fixture")
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        if self.backend.root().exists() {
            fs::remove_dir_all(self.backend.root()).expect("fixture cleanup");
        }
    }
}

#[test]
fn local_filesystem_backend_passes_applicable_common_conformance_scenarios() {
    let fixture = Fixture::new();

    let report = run_durable_state_conformance(&fixture.backend, &fixture.conformance_fixture())
        .expect("conformance passes");

    assert_eq!(report.results().len(), 21);
    assert!(report
        .results()
        .iter()
        .take(8)
        .all(|result| result.outcome() == DurableStateConformanceOutcome::Passed));
    assert!(report
        .results()
        .iter()
        .skip(8)
        .all(|result| result.outcome() == DurableStateConformanceOutcome::Unsupported));
}

#[test]
fn conformance_scenarios_accept_maximum_length_fixture_event_ids() {
    let mut fixture = Fixture::new();
    fixture.validated.event_id = EventId::new("e".repeat(128)).expect("maximum event id");

    let report = run_durable_state_conformance(&fixture.backend, &fixture.conformance_fixture())
        .expect("bounded fixture ids remain valid");

    assert_eq!(report.results().len(), 21);
}

#[test]
fn local_filesystem_contract_does_not_overclaim_transactional_guarantees() {
    let fixture = Fixture::new();
    let contract = fixture
        .backend
        .durable_state_contract()
        .expect("contract declaration");

    assert_eq!(
        contract.backend_kind(),
        DurableStateBackendKind::LocalFilesystemPreview
    );
    assert!(contract.supports_capability(DurableStateCapability::OrderedEventAppend));
    assert!(contract.supports_capability(DurableStateCapability::ImmutableRunIdentityValidation));
    assert!(contract.supports_capability(DurableStateCapability::IdempotencyReplay));
    assert!(contract.supports_capability(DurableStateCapability::ProcessLocalExclusiveLock));
    assert!(!contract.supports_capability(DurableStateCapability::CrossRecordAtomicCommit));
    assert!(!contract.supports_capability(DurableStateCapability::CompareAndSetRevision));
    assert!(!contract.supports_capability(DurableStateCapability::ExpiringFencedLease));
    assert!(!contract.supports_capability(DurableStateCapability::ManagedSchemaMigration));
    assert_eq!(
        contract.lease_semantics(),
        DurableLeaseSemantics::ProcessLocalUnfenced
    );
    assert_eq!(contract.schema(), DurableStateSchemaMetadata::unmanaged());
    assert!(DurableStateTransactionKind::all()
        .iter()
        .all(|kind| { contract.transaction_support(*kind) == DurableStateSupport::Unsupported }));
}

#[test]
fn durable_state_contract_serde_round_trip_preserves_declared_posture() {
    let fixture = Fixture::new();
    let contract = fixture
        .backend
        .durable_state_contract()
        .expect("contract declaration");

    let serialized = serde_json::to_string(&contract).expect("serialize");
    let decoded: DurableStateSemanticContract =
        serde_json::from_str(&serialized).expect("deserialize");

    assert_eq!(decoded, contract);
    assert!(!serialized.contains("password"));
    assert!(!serialized.contains("token"));
    assert!(!serialized.contains("provider_payload"));
}

#[test]
fn invalid_revision_and_schema_metadata_fail_closed() {
    let revision_error =
        serde_json::from_str::<DurableRevision>("0").expect_err("zero revision rejected");
    let schema_error = serde_json::from_str::<DurableStateSchemaMetadata>(
        r#"{"adapter_schema_version":null,"posture":"ready"}"#,
    )
    .expect_err("inconsistent schema metadata rejected");

    assert!(!revision_error.to_string().contains("secret-value"));
    assert!(!schema_error.to_string().contains("secret-value"));
    assert!(DurableStateSchemaMetadata::managed(0, DurableStateSchemaPosture::Ready).is_err());
}

#[test]
fn invalid_conformance_fixture_fails_without_identity_leakage() {
    let fixture = Fixture::new();
    let mut invalid = fixture.validated.clone();
    invalid.sequence_number = EventSequenceNumber::new(3).expect("sequence");

    let result = DurableStateConformanceFixture::new(
        fixture.created.clone(),
        invalid,
        IdempotencyKey::new("durable-conformance/secret-token-marker").expect("idempotency key"),
        ActorId::new("worker/conformance").expect("actor"),
    );
    assert!(result.is_err(), "invalid fixture accepted");
    let error = result.err().expect("invalid fixture error");

    assert_eq!(
        error.code(),
        "durable_state.contract.conformance_fixture.invalid"
    );
    assert!(!error.to_string().contains("secret-token-marker"));
    assert!(!error.to_string().contains(fixture.created.run_id.as_str()));
}
