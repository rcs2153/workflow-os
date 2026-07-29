use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use postgres::error::SqlState;
use postgres::{Client, Config, IsolationLevel, NoTls, Transaction};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::{
    validate_approval_presentation_for_request, ActorId, AdapterRuntimeAuditRecord,
    AdapterRuntimeObservabilityRecord, ApprovalPresentationId, ApprovalPresentationRecord,
    ApprovalPresentationRecordStore, ApprovalPresentationValidationInput, ApprovalRequest,
    ApprovalStore, BackendHealthCheck, DurableLeaseSemantics, DurableRevision,
    DurableStateBackendKind, DurableStateCapability, DurableStateContractProvider,
    DurableStateContractVersion, DurableStateSchemaMetadata, DurableStateSchemaPosture,
    DurableStateSemanticContract, DurableStateSupport, DurableStateTransactionKind,
    DurableStateTransactionSupport, EventLogStore, IdempotencyKey, IdempotencyResult,
    IdempotencyStore, IdempotencyWrite, ImmutableRunBundleBuildResult,
    ImmutableRunBundleDefinitionRecord, ImmutableRunBundleManifest, LockLease, LockStore,
    PolicyAuditRecord, PolicyAuditStore, ProjectId, ProjectStateRecord, ProjectStateStore,
    RunSnapshotStore, SideEffectId, SideEffectRecord, SideEffectRecordStore, StateBackend,
    StoredImmutableRunBundle, WorkReportArtifactRecord, WorkReportArtifactStore, WorkReportId,
    WorkflowOsError, WorkflowOsErrorKind, WorkflowRun, WorkflowRunEvent, WorkflowRunId,
    WorkflowRunSnapshot,
};

const SCHEMA_VERSION: i32 = 1;
const SCHEMA_CHECKSUM: &str = "workflow-os-postgresql-v1";
const MAX_TRANSACTION_ATTEMPTS: usize = 3;
const DEFAULT_LEASE_TTL: Duration = Duration::from_secs(30);

/// Creates one connected `PostgreSQL` client for a bounded store operation.
///
/// Implementations own credential retrieval, TLS, timeouts, and pooling. They
/// must not expose connection material through `Debug` or returned errors.
pub trait PostgresConnectionFactory: Send + Sync {
    /// Opens one connected client.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when a connection is unavailable.
    fn connect(&self) -> Result<Client, WorkflowOsError>;
}

/// Explicit local/test-only `PostgreSQL` connection factory using `NoTls`.
///
/// This type is not a production transport recommendation. Production callers
/// should inject a factory with reviewed TLS and credential handling.
#[derive(Clone)]
pub struct PostgresNoTlsConnectionFactory {
    config: Config,
}

impl PostgresNoTlsConnectionFactory {
    /// Creates an explicit local/test `NoTls` factory from parsed configuration.
    #[must_use]
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl fmt::Debug for PostgresNoTlsConnectionFactory {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PostgresNoTlsConnectionFactory")
            .field("transport", &"no_tls_local_or_test_only")
            .field("connection", &"[REDACTED]")
            .finish()
    }
}

impl PostgresConnectionFactory for PostgresNoTlsConnectionFactory {
    fn connect(&self) -> Result<Client, WorkflowOsError> {
        self.config
            .connect(NoTls)
            .map_err(|error| database_error("connect", &error))
    }
}

/// One validated value together with its committed durable revision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PostgresRevisionedRecord<T> {
    value: T,
    revision: DurableRevision,
}

impl<T> PostgresRevisionedRecord<T> {
    /// Returns the stored value.
    #[must_use]
    pub const fn value(&self) -> &T {
        &self.value
    }

    /// Returns the committed revision.
    #[must_use]
    pub const fn revision(&self) -> DurableRevision {
        self.revision
    }

    /// Consumes this record into its parts.
    #[must_use]
    pub fn into_parts(self) -> (T, DurableRevision) {
        (self.value, self.revision)
    }
}

/// Bounded lease key used by shared `PostgreSQL` workers.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PostgresLeaseKey(String);

impl PostgresLeaseKey {
    /// Creates a bounded non-secret lease key.
    ///
    /// # Errors
    ///
    /// Rejects empty, oversized, control-character, or secret-like values.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        if value.is_empty()
            || value.len() > 256
            || value.chars().any(char::is_control)
            || looks_secret_like(&value)
        {
            return Err(state_error(
                "postgres_state.lease_key.invalid",
                "PostgreSQL lease key is invalid",
            ));
        }
        Ok(Self(value))
    }

    /// Returns the validated key text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for PostgresLeaseKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("PostgresLeaseKey([REDACTED])")
    }
}

/// Request to acquire or renew an expiring fenced worker lease.
#[derive(Clone, Copy)]
pub struct PostgresLeaseAcquireRequest<'a> {
    /// Lease identity.
    pub key: &'a PostgresLeaseKey,
    /// Worker identity.
    pub owner: &'a ActorId,
    /// Positive lease duration.
    pub ttl: Duration,
}

/// Expiring lease with a monotonically increasing fencing token.
#[derive(Clone, Eq, PartialEq)]
pub struct PostgresFencedLease {
    key: PostgresLeaseKey,
    owner: ActorId,
    fence_token: u64,
    expires_at_epoch_ms: i64,
}

impl PostgresFencedLease {
    /// Returns the lease key.
    #[must_use]
    pub const fn key(&self) -> &PostgresLeaseKey {
        &self.key
    }

    /// Returns the lease owner.
    #[must_use]
    pub const fn owner(&self) -> &ActorId {
        &self.owner
    }

    /// Returns the fencing token.
    #[must_use]
    pub const fn fence_token(&self) -> u64 {
        self.fence_token
    }

    /// Returns the database-derived expiration time as Unix epoch milliseconds.
    #[must_use]
    pub const fn expires_at_epoch_ms(&self) -> i64 {
        self.expires_at_epoch_ms
    }
}

impl fmt::Debug for PostgresFencedLease {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PostgresFencedLease")
            .field("key", &"[REDACTED]")
            .field("owner", &"[REDACTED]")
            .field("fence_token", &self.fence_token)
            .field("expires_at_epoch_ms", &self.expires_at_epoch_ms)
            .finish()
    }
}

/// Atomic pre-effect reservation input.
pub struct PostgresReserveIntentRequest<'a> {
    /// Idempotency key reserved before any provider call.
    pub idempotency_key: &'a IdempotencyKey,
    /// Bounded result reference returned on exact replay.
    pub idempotency_result: IdempotencyResult,
    /// Validated proposed/attempted `SideEffect` intent.
    pub side_effect: &'a SideEffectRecord,
    /// Authoritative event describing the intent.
    pub event: &'a WorkflowRunEvent,
}

/// Atomic external-outcome recording input.
#[derive(Clone, Copy)]
pub struct PostgresRecordExternalOutcomeRequest<'a> {
    /// Expected prior `SideEffect` revision.
    pub expected_revision: DurableRevision,
    /// Validated transitioned `SideEffect` record.
    pub side_effect: &'a SideEffectRecord,
    /// Authoritative outcome event.
    pub event: &'a WorkflowRunEvent,
}

/// Atomic approval-decision recording input.
#[derive(Clone, Copy)]
pub struct PostgresRecordApprovalDecisionRequest<'a> {
    /// Updated approval projection containing a decision.
    pub approval: &'a ApprovalRequest,
    /// Durable presentation proof that was shown before the decision.
    pub presentation: &'a ApprovalPresentationRecord,
    /// Authoritative approval-decision event.
    pub event: &'a WorkflowRunEvent,
}

/// Atomic `SideEffect` transition input.
#[derive(Clone, Copy)]
pub struct PostgresTransitionSideEffectRequest<'a> {
    /// Expected prior revision.
    pub expected_revision: DurableRevision,
    /// Validated next `SideEffect` record.
    pub side_effect: &'a SideEffectRecord,
    /// Authoritative lifecycle event.
    pub event: &'a WorkflowRunEvent,
}

/// Atomic authoritative-event and snapshot-projection input.
#[derive(Clone, Copy)]
pub struct PostgresAuthoritativeProjectionRequest<'a> {
    /// Authoritative next event.
    pub event: &'a WorkflowRunEvent,
    /// Derived snapshot after applying the event.
    pub snapshot: &'a WorkflowRunSnapshot,
    /// Expected snapshot revision, or `None` for first publication.
    pub expected_snapshot_revision: Option<DurableRevision>,
    /// Optional active fence required for a shared worker commit.
    pub lease: Option<&'a PostgresFencedLease>,
}

/// Input for the explicit shared run consumer.
#[derive(Clone, Copy)]
pub struct PostgresSharedRunConsumerRequest<'a> {
    /// Authoritative next event to consume.
    pub event: &'a WorkflowRunEvent,
    /// Worker identity.
    pub worker: &'a ActorId,
    /// Lease duration. Zero uses the conservative default.
    pub lease_ttl: Duration,
}

/// Result of one explicit shared run-consumer transition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PostgresSharedRunConsumerResult {
    run: WorkflowRun,
    snapshot_revision: DurableRevision,
    fence_token: u64,
}

impl PostgresSharedRunConsumerResult {
    /// Returns the rehydrated run after the committed event.
    #[must_use]
    pub const fn run(&self) -> &WorkflowRun {
        &self.run
    }

    /// Returns the committed snapshot revision.
    #[must_use]
    pub const fn snapshot_revision(&self) -> DurableRevision {
        self.snapshot_revision
    }

    /// Returns the fencing token used for the commit.
    #[must_use]
    pub const fn fence_token(&self) -> u64 {
        self.fence_token
    }
}

/// Read-only projection rebuild plan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PostgresStateIntegrityPlan {
    run_ids: Vec<WorkflowRunId>,
}

impl PostgresStateIntegrityPlan {
    /// Returns runs whose projections can be rebuilt from authoritative events.
    #[must_use]
    pub fn run_ids(&self) -> &[WorkflowRunId] {
        &self.run_ids
    }
}

/// Result of an integrity verification or projection rebuild.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PostgresStateIntegrityResult {
    checked_run_count: usize,
    rebuilt_snapshot_count: usize,
}

impl PostgresStateIntegrityResult {
    /// Returns the number of authoritative run streams checked.
    #[must_use]
    pub const fn checked_run_count(&self) -> usize {
        self.checked_run_count
    }

    /// Returns the number of snapshots rebuilt.
    #[must_use]
    pub const fn rebuilt_snapshot_count(&self) -> usize {
        self.rebuilt_snapshot_count
    }
}

/// Detailed bounded `PostgreSQL` health posture.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PostgresStateHealthReport {
    schema_version: u32,
    healthy: bool,
    recovery_required: bool,
}

impl PostgresStateHealthReport {
    /// Returns the adapter schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Returns whether ordinary operations are healthy.
    #[must_use]
    pub const fn healthy(&self) -> bool {
        self.healthy
    }

    /// Returns whether operator recovery is required.
    #[must_use]
    pub const fn recovery_required(&self) -> bool {
        self.recovery_required
    }
}

/// Explicit shared `PostgreSQL` durable-state backend.
#[derive(Clone)]
pub struct PostgresStateBackend {
    connections: Arc<dyn PostgresConnectionFactory>,
}

impl PostgresStateBackend {
    /// Creates an uninitialized explicit `PostgreSQL` backend.
    #[must_use]
    pub fn new(connections: Arc<dyn PostgresConnectionFactory>) -> Self {
        Self { connections }
    }

    /// Initializes or verifies the managed schema under a migration advisory lock.
    ///
    /// # Errors
    ///
    /// Fails closed for newer, checksum-mismatched, or recovery-required schemas.
    pub fn initialize_schema(&self) -> Result<PostgresStateHealthReport, WorkflowOsError> {
        let mut client = self.connections.connect()?;
        let mut tx = client
            .transaction()
            .map_err(|error| database_error("schema_begin", &error))?;
        tx.execute(
            "SELECT pg_advisory_xact_lock(hashtext('workflow_os_schema_migration'))",
            &[],
        )
        .map_err(|error| database_error("schema_lock", &error))?;
        tx.batch_execute(SCHEMA_SQL)
            .map_err(|error| database_error("schema_apply", &error))?;
        let existing = tx
            .query_opt(
                "SELECT schema_version, checksum, recovery_required
                   FROM workflow_os.schema_metadata WHERE singleton = TRUE",
                &[],
            )
            .map_err(|error| database_error("schema_read", &error))?;
        match existing {
            None => {
                tx.execute(
                    "INSERT INTO workflow_os.schema_metadata
                       (singleton, schema_version, checksum, recovery_required)
                     VALUES (TRUE, $1, $2, FALSE)",
                    &[&SCHEMA_VERSION, &SCHEMA_CHECKSUM],
                )
                .map_err(|error| database_error("schema_record", &error))?;
            }
            Some(row) => {
                let version: i32 = row.get(0);
                let checksum: String = row.get(1);
                let recovery_required: bool = row.get(2);
                if version != SCHEMA_VERSION || checksum != SCHEMA_CHECKSUM {
                    return Err(state_error(
                        "postgres_state.schema.incompatible",
                        "PostgreSQL state schema is incompatible",
                    ));
                }
                if recovery_required {
                    return Err(state_error(
                        "postgres_state.schema.recovery_required",
                        "PostgreSQL state schema requires operator recovery",
                    ));
                }
            }
        }
        tx.commit()
            .map_err(|error| database_error("schema_commit", &error))?;
        Ok(PostgresStateHealthReport {
            schema_version: SCHEMA_VERSION as u32,
            healthy: true,
            recovery_required: false,
        })
    }

    /// Acquires or renews an expiring fenced lease using database time.
    ///
    /// # Errors
    ///
    /// Returns deterministic contention when another live owner holds the lease.
    pub fn acquire_fenced_lease(
        &self,
        request: PostgresLeaseAcquireRequest<'_>,
    ) -> Result<PostgresFencedLease, WorkflowOsError> {
        let ttl = validate_ttl(request.ttl)?;
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| {
            let owner = request.owner.as_str();
            let row = tx
                .query_opt(
                    "SELECT owner, fence_token,
                            expires_at <= clock_timestamp() AS expired
                       FROM workflow_os.worker_leases
                      WHERE lease_key = $1
                      FOR UPDATE",
                    &[&request.key.as_str()],
                )
                .map_err(|error| database_error("lease_read", &error))?;
            let fence = match row {
                None => 1_i64,
                Some(row) => {
                    let current_owner: String = row.get(0);
                    let current_fence: i64 = row.get(1);
                    let expired: bool = row.get(2);
                    if !expired && current_owner != owner {
                        return Err(state_error(
                            "postgres_state.lease.contended",
                            "PostgreSQL worker lease is held by another owner",
                        ));
                    }
                    current_fence.checked_add(1).ok_or_else(|| {
                        state_error(
                            "postgres_state.lease.fence_exhausted",
                            "PostgreSQL worker lease fencing token is exhausted",
                        )
                    })?
                }
            };
            let ttl_ms = ttl.as_secs_f64() * 1_000.0;
            let row = tx
                .query_one(
                    "INSERT INTO workflow_os.worker_leases
                       (lease_key, owner, fence_token, expires_at)
                     VALUES ($1, $2, $3,
                             clock_timestamp()
                               + ($4::double precision * interval '1 millisecond'))
                     ON CONFLICT (lease_key) DO UPDATE SET
                       owner = EXCLUDED.owner,
                       fence_token = EXCLUDED.fence_token,
                       expires_at = EXCLUDED.expires_at
                     RETURNING
                       (extract(epoch FROM expires_at) * 1000)::bigint",
                    &[&request.key.as_str(), &owner, &fence, &ttl_ms],
                )
                .map_err(|error| database_error("lease_write", &error))?;
            let expires_at_epoch_ms: i64 = row.get(0);
            Ok(PostgresFencedLease {
                key: request.key.clone(),
                owner: request.owner.clone(),
                fence_token: u64::try_from(fence).map_err(|_| {
                    state_error(
                        "postgres_state.lease.fence_invalid",
                        "PostgreSQL worker lease fencing token is invalid",
                    )
                })?,
                expires_at_epoch_ms,
            })
        })
    }

    /// Releases a fenced lease only when owner and token still match.
    ///
    /// # Errors
    ///
    /// Rejects stale holders rather than releasing a newer lease.
    pub fn release_fenced_lease(&self, lease: &PostgresFencedLease) -> Result<(), WorkflowOsError> {
        let mut client = self.connections.connect()?;
        let fence = i64::try_from(lease.fence_token).map_err(|_| {
            state_error(
                "postgres_state.lease.fence_invalid",
                "PostgreSQL worker lease fencing token is invalid",
            )
        })?;
        let affected = client
            .execute(
                "DELETE FROM workflow_os.worker_leases
                  WHERE lease_key = $1 AND owner = $2 AND fence_token = $3",
                &[&lease.key.as_str(), &lease.owner.as_str(), &fence],
            )
            .map_err(|error| database_error("lease_release", &error))?;
        if affected != 1 {
            return Err(state_error(
                "postgres_state.lease.stale",
                "PostgreSQL worker lease is stale",
            ));
        }
        Ok(())
    }

    /// Atomically reserves idempotency and records pre-effect intent.
    ///
    /// # Errors
    ///
    /// Conflicting replay or invalid event/SideEffect state fails closed.
    pub fn reserve_idempotency_and_record_intent(
        &self,
        request: PostgresReserveIntentRequest<'_>,
    ) -> Result<IdempotencyWrite, WorkflowOsError> {
        let PostgresReserveIntentRequest {
            idempotency_key,
            idempotency_result,
            side_effect,
            event,
        } = request;
        side_effect.validate()?;
        validate_side_effect_event_binding(side_effect, event)?;
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| {
            let key = idempotency_key.as_str();
            let intent_ref = format!(
                "{}/{}",
                side_effect.side_effect_id().as_str(),
                event.event_id.as_str()
            );
            if let Some(row) = tx
                .query_opt(
                    "SELECT payload, intent_ref
                       FROM workflow_os.idempotency WHERE key = $1 FOR UPDATE",
                    &[&key],
                )
                .map_err(|error| database_error("idempotency_read", &error))?
            {
                let stored_intent_ref: Option<String> = row.get(1);
                if stored_intent_ref.as_deref() != Some(intent_ref.as_str()) {
                    return Err(state_error(
                        "postgres_state.idempotency.intent_conflict",
                        "PostgreSQL idempotency key is bound to another intent",
                    ));
                }
                let prior: IdempotencyResult = decode(row.get::<_, String>(0).as_str())?;
                return Ok(IdempotencyWrite::Duplicate(prior));
            }
            insert_side_effect_create_only(tx, side_effect)?;
            append_event_tx(tx, event)?;
            tx.execute(
                "INSERT INTO workflow_os.idempotency (key, payload, intent_ref)
                 VALUES ($1, $2, $3)",
                &[&key, &encode(&idempotency_result)?, &intent_ref],
            )
            .map_err(|error| database_error("idempotency_write", &error))?;
            Ok(IdempotencyWrite::FirstWrite(idempotency_result.clone()))
        })
    }

    /// Atomically records an external outcome, `SideEffect` revision, and event.
    ///
    /// # Errors
    ///
    /// Fails closed when the transition, event binding, or expected revision is invalid.
    pub fn record_external_operation_outcome(
        &self,
        request: PostgresRecordExternalOutcomeRequest<'_>,
    ) -> Result<DurableRevision, WorkflowOsError> {
        self.transition_side_effect(PostgresTransitionSideEffectRequest {
            expected_revision: request.expected_revision,
            side_effect: request.side_effect,
            event: request.event,
        })
    }

    /// Atomically records an approval projection and authoritative decision event.
    ///
    /// # Errors
    ///
    /// Fails closed when the pending approval, presentation proof, decision event,
    /// or expected projection revision does not match durable state.
    pub fn record_approval_decision(
        &self,
        request: PostgresRecordApprovalDecisionRequest<'_>,
    ) -> Result<(), WorkflowOsError> {
        request.approval.validate_subject()?;
        let Some(decision) = request.approval.decision.as_ref() else {
            return Err(state_error(
                "postgres_state.approval.decision_missing",
                "PostgreSQL approval transaction requires a decision",
            ));
        };
        validate_approval_event_binding(request.approval, request.event)?;
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| {
            let row = tx
                .query_opt(
                    "SELECT payload, revision FROM workflow_os.records
                      WHERE family = 'approval' AND key1 = $1 AND key2 = ''
                      FOR UPDATE",
                    &[&request.approval.approval_id],
                )
                .map_err(|error| database_error("approval_read", &error))?
                .ok_or_else(|| {
                    state_error(
                        "postgres_state.approval.missing",
                        "PostgreSQL approval request is missing",
                    )
                })?;
            let prior: ApprovalRequest = decode(row.get::<_, String>(0).as_str())?;
            if prior.decision.is_some() {
                return Err(state_error(
                    "postgres_state.approval.already_decided",
                    "PostgreSQL approval request already has a decision",
                ));
            }
            let mut expected = prior.clone();
            expected.decision = Some(decision.clone());
            if expected != *request.approval {
                return Err(state_error(
                    "postgres_state.approval.identity_mismatch",
                    "PostgreSQL approval decision does not match the pending request",
                ));
            }
            validate_approval_presentation_for_request(ApprovalPresentationValidationInput {
                presentation: request.presentation,
                approval_request: &prior,
            })
            .map_err(|_| {
                state_error(
                    "postgres_state.approval.presentation_mismatch",
                    "PostgreSQL approval presentation does not match the pending request",
                )
            })?;
            let stored_presentation = tx
                .query_opt(
                    "SELECT payload FROM workflow_os.records
                      WHERE family = 'approval_presentation' AND key1 = $1
                      FOR UPDATE",
                    &[&request.presentation.presentation_id().as_str()],
                )
                .map_err(|error| database_error("approval_presentation_read", &error))?
                .ok_or_else(|| {
                    state_error(
                        "postgres_state.approval.presentation_missing",
                        "PostgreSQL approval presentation proof is missing",
                    )
                })?;
            let stored_presentation: ApprovalPresentationRecord =
                decode(stored_presentation.get::<_, String>(0).as_str())?;
            if stored_presentation != *request.presentation {
                return Err(state_error(
                    "postgres_state.approval.presentation_mismatch",
                    "PostgreSQL approval presentation does not match durable proof",
                ));
            }
            let proof_marker = decision.proof_marker.as_ref().ok_or_else(|| {
                state_error(
                    "postgres_state.approval.proof_marker_missing",
                    "PostgreSQL approval decision proof marker is missing",
                )
            })?;
            if proof_marker.presentation_id() != request.presentation.presentation_id()
                || proof_marker.presentation_content_hash() != request.presentation.content_hash()
            {
                return Err(state_error(
                    "postgres_state.approval.proof_marker_mismatch",
                    "PostgreSQL approval decision proof marker does not match presentation proof",
                ));
            }
            append_event_tx(tx, request.event)?;
            let revision = revision_from_i64(row.get(1))?;
            put_record(
                tx,
                "approval",
                &request.approval.approval_id,
                "",
                request.approval,
                Some(revision),
                false,
            )?;
            Ok(())
        })
    }

    /// Atomically compare-and-transitions a `SideEffect` plus its event.
    ///
    /// # Errors
    ///
    /// Fails closed when the lifecycle transition, event binding, or expected
    /// revision is invalid.
    pub fn transition_side_effect(
        &self,
        request: PostgresTransitionSideEffectRequest<'_>,
    ) -> Result<DurableRevision, WorkflowOsError> {
        request.side_effect.validate()?;
        validate_side_effect_event_binding(request.side_effect, request.event)?;
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| {
            append_event_tx(tx, request.event)?;
            update_side_effect_cas(tx, request.side_effect, request.expected_revision)
        })
    }

    /// Atomically publishes all records and one create-only immutable run manifest.
    ///
    /// # Errors
    ///
    /// Fails closed when referenced records conflict, storage is unavailable, or
    /// the run already has an immutable manifest.
    pub fn publish_immutable_run_bundle(
        &self,
        bundle: &ImmutableRunBundleBuildResult,
    ) -> Result<(), WorkflowOsError> {
        let manifest = bundle.manifest();
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| {
            if tx
                .query_opt(
                    "SELECT 1 FROM workflow_os.immutable_manifests WHERE run_id = $1",
                    &[&manifest.run_id().as_str()],
                )
                .map_err(|error| database_error("bundle_manifest_read", &error))?
                .is_some()
            {
                return Err(state_error(
                    "postgres_state.bundle.manifest_exists",
                    "immutable run bundle manifest already exists",
                ));
            }
            for record in bundle.definition_records() {
                insert_content_addressed_definition(tx, record)?;
            }
            for record in bundle.local_check_declaration_set_records() {
                insert_content_addressed_local_check_set(tx, record)?;
            }
            tx.execute(
                "INSERT INTO workflow_os.immutable_manifests
                   (run_id, root_hash, payload, definition_hashes, local_check_hashes)
                 VALUES ($1, $2, $3, $4, $5)",
                &[
                    &manifest.run_id().as_str(),
                    &manifest.root_hash().as_str(),
                    &encode(manifest)?,
                    &encode(
                        &bundle
                            .definition_records()
                            .iter()
                            .map(|record| record.canonical_record_hash().as_str().to_owned())
                            .collect::<Vec<_>>(),
                    )?,
                    &encode(
                        &bundle
                            .local_check_declaration_set_records()
                            .iter()
                            .map(|record| record.declaration_set_fingerprint().as_str().to_owned())
                            .collect::<Vec<_>>(),
                    )?,
                ],
            )
            .map_err(|error| database_error("bundle_manifest_write", &error))?;
            Ok(())
        })
    }

    /// Reads one complete immutable run bundle and validates every reference.
    ///
    /// # Errors
    ///
    /// Fails closed when stored identities, hashes, payloads, or references do
    /// not form a valid immutable bundle.
    pub fn read_immutable_run_bundle(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<Option<StoredImmutableRunBundle>, WorkflowOsError> {
        let mut client = self.connections.connect()?;
        let Some(row) = client
            .query_opt(
                "SELECT root_hash, payload, definition_hashes, local_check_hashes
                   FROM workflow_os.immutable_manifests WHERE run_id = $1",
                &[&run_id.as_str()],
            )
            .map_err(|error| database_error("bundle_manifest_read", &error))?
        else {
            return Ok(None);
        };
        let root_hash: String = row.get(0);
        let manifest: ImmutableRunBundleManifest = decode(row.get::<_, String>(1).as_str())?;
        if manifest.run_id() != run_id || manifest.root_hash().as_str() != root_hash {
            return Err(state_error(
                "postgres_state.bundle.identity_mismatch",
                "immutable run bundle storage identity does not match payload",
            ));
        }
        let definition_hashes: Vec<String> = decode(row.get::<_, String>(2).as_str())?;
        let local_check_hashes: Vec<String> = decode(row.get::<_, String>(3).as_str())?;
        let definitions = load_bundle_definitions(&mut client, &definition_hashes)?;
        let local_checks = load_bundle_local_check_sets(&mut client, &local_check_hashes)?;
        Ok(Some(StoredImmutableRunBundle::from_validated_parts(
            manifest,
            definitions,
            local_checks,
        )?))
    }

    /// Atomically commits an authoritative event and its derived snapshot projection.
    ///
    /// # Errors
    ///
    /// Fails closed when event ordering, the optional lease fence, or the
    /// expected snapshot revision is invalid.
    pub fn commit_authoritative_result_and_projection(
        &self,
        request: PostgresAuthoritativeProjectionRequest<'_>,
    ) -> Result<DurableRevision, WorkflowOsError> {
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| {
            if let Some(lease) = request.lease {
                validate_fence_tx(tx, lease)?;
            }
            append_event_tx(tx, request.event)?;
            put_record(
                tx,
                "snapshot",
                request.snapshot.identity.run_id.as_str(),
                "",
                request.snapshot,
                request.expected_snapshot_revision,
                request.expected_snapshot_revision.is_none(),
            )
        })
    }

    /// Consumes one explicit run event under an expiring fenced lease.
    ///
    /// # Errors
    ///
    /// Fails closed when lease acquisition, event replay, rehydration, or the
    /// fenced projection commit is invalid.
    pub fn consume_shared_run_event(
        &self,
        request: PostgresSharedRunConsumerRequest<'_>,
    ) -> Result<PostgresSharedRunConsumerResult, WorkflowOsError> {
        let key = PostgresLeaseKey::new(format!("run/{}", request.event.identity().run_id))?;
        let lease = self.acquire_fenced_lease(PostgresLeaseAcquireRequest {
            key: &key,
            owner: request.worker,
            ttl: if request.lease_ttl.is_zero() {
                DEFAULT_LEASE_TTL
            } else {
                request.lease_ttl
            },
        })?;
        let existing = self.read_events(&request.event.identity().run_id)?;
        crate::state::validate_append_against_history(&existing, request.event)?;
        let mut all_events = existing;
        all_events.push(request.event.clone());
        let run = WorkflowRun::rehydrate(&all_events)?;
        let prior = self.load_revisioned_snapshot(&request.event.identity().run_id)?;
        let revision = self.commit_authoritative_result_and_projection(
            PostgresAuthoritativeProjectionRequest {
                event: request.event,
                snapshot: &run.snapshot,
                expected_snapshot_revision: prior.as_ref().map(PostgresRevisionedRecord::revision),
                lease: Some(&lease),
            },
        )?;
        self.release_fenced_lease(&lease)?;
        Ok(PostgresSharedRunConsumerResult {
            run,
            snapshot_revision: revision,
            fence_token: lease.fence_token,
        })
    }

    /// Builds a read-only plan for deterministic snapshot rebuild.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when authoritative event discovery fails.
    pub fn plan_projection_rebuild(&self) -> Result<PostgresStateIntegrityPlan, WorkflowOsError> {
        let mut client = self.connections.connect()?;
        let rows = client
            .query(
                "SELECT DISTINCT run_id FROM workflow_os.events ORDER BY run_id",
                &[],
            )
            .map_err(|error| database_error("integrity_plan", &error))?;
        let mut run_ids = Vec::with_capacity(rows.len());
        for row in rows {
            run_ids.push(WorkflowRunId::new(row.get::<_, String>(0))?);
        }
        Ok(PostgresStateIntegrityPlan { run_ids })
    }

    /// Rebuilds snapshots deterministically from authoritative events.
    ///
    /// # Errors
    ///
    /// Fails closed when an event stream cannot be rehydrated or a snapshot
    /// projection cannot be committed with its expected revision.
    pub fn rebuild_projections(
        &self,
        plan: &PostgresStateIntegrityPlan,
    ) -> Result<PostgresStateIntegrityResult, WorkflowOsError> {
        let mut rebuilt = 0;
        for run_id in &plan.run_ids {
            let run = self.rehydrate_run(run_id)?;
            let prior = self.load_revisioned_snapshot(run_id)?;
            let mut client = self.connections.connect()?;
            serializable(&mut client, |tx| {
                put_record(
                    tx,
                    "snapshot",
                    run_id.as_str(),
                    "",
                    &run.snapshot,
                    prior.as_ref().map(PostgresRevisionedRecord::revision),
                    prior.is_none(),
                )
                .map(|_| ())
            })?;
            rebuilt += 1;
        }
        Ok(PostgresStateIntegrityResult {
            checked_run_count: plan.run_ids.len(),
            rebuilt_snapshot_count: rebuilt,
        })
    }

    /// Loads a revisioned run snapshot.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error for invalid or unavailable stored data.
    pub fn load_revisioned_snapshot(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<Option<PostgresRevisionedRecord<WorkflowRunSnapshot>>, WorkflowOsError> {
        let mut client = self.connections.connect()?;
        read_record(&mut client, "snapshot", run_id.as_str(), "")
    }

    /// Returns bounded adapter health and schema posture.
    ///
    /// # Errors
    ///
    /// Fails closed when schema metadata is unavailable or invalid.
    pub fn detailed_health_check(&self) -> Result<PostgresStateHealthReport, WorkflowOsError> {
        let mut client = self.connections.connect()?;
        let row = client
            .query_opt(
                "SELECT schema_version, checksum, recovery_required
                   FROM workflow_os.schema_metadata WHERE singleton = TRUE",
                &[],
            )
            .map_err(|error| database_error("health", &error))?
            .ok_or_else(|| {
                state_error(
                    "postgres_state.schema.missing",
                    "PostgreSQL state schema is not initialized",
                )
            })?;
        let version: i32 = row.get(0);
        let checksum: String = row.get(1);
        let recovery_required: bool = row.get(2);
        Ok(PostgresStateHealthReport {
            schema_version: u32::try_from(version).unwrap_or_default(),
            healthy: version == SCHEMA_VERSION && checksum == SCHEMA_CHECKSUM && !recovery_required,
            recovery_required,
        })
    }
}

impl fmt::Debug for PostgresStateBackend {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PostgresStateBackend")
            .field("connections", &"[REDACTED]")
            .finish()
    }
}

impl EventLogStore for PostgresStateBackend {
    fn append_event(&self, event: &WorkflowRunEvent) -> Result<(), WorkflowOsError> {
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| append_event_tx(tx, event))
    }

    fn read_events(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<Vec<WorkflowRunEvent>, WorkflowOsError> {
        let mut client = self.connections.connect()?;
        read_events_client(&mut client, run_id)
    }
}

impl RunSnapshotStore for PostgresStateBackend {
    fn save_snapshot(&self, snapshot: &WorkflowRunSnapshot) -> Result<(), WorkflowOsError> {
        let prior = self.load_revisioned_snapshot(&snapshot.identity.run_id)?;
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| {
            put_record(
                tx,
                "snapshot",
                snapshot.identity.run_id.as_str(),
                "",
                snapshot,
                prior.as_ref().map(PostgresRevisionedRecord::revision),
                prior.is_none(),
            )
            .map(|_| ())
        })
    }

    fn load_snapshot(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<Option<WorkflowRunSnapshot>, WorkflowOsError> {
        Ok(self
            .load_revisioned_snapshot(run_id)?
            .map(|record| record.value))
    }
}

impl IdempotencyStore for PostgresStateBackend {
    fn record_idempotency_result(
        &self,
        key: &IdempotencyKey,
        result: IdempotencyResult,
    ) -> Result<IdempotencyWrite, WorkflowOsError> {
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| {
            let key = key.as_str();
            if let Some(row) = tx
                .query_opt(
                    "SELECT payload FROM workflow_os.idempotency WHERE key = $1 FOR UPDATE",
                    &[&key],
                )
                .map_err(|error| database_error("idempotency_read", &error))?
            {
                return Ok(IdempotencyWrite::Duplicate(decode(
                    row.get::<_, String>(0).as_str(),
                )?));
            }
            tx.execute(
                "INSERT INTO workflow_os.idempotency (key, payload) VALUES ($1, $2)",
                &[&key, &encode(&result)?],
            )
            .map_err(|error| database_error("idempotency_write", &error))?;
            Ok(IdempotencyWrite::FirstWrite(result.clone()))
        })
    }
}

impl LockStore for PostgresStateBackend {
    fn acquire_lock(&self, key: &str, owner: &ActorId) -> Result<LockLease, WorkflowOsError> {
        validate_lock_key(key)?;
        let mut client = self.connections.connect()?;
        let inserted = client
            .execute(
                "INSERT INTO workflow_os.local_locks (lock_key, owner)
                 VALUES ($1, $2) ON CONFLICT DO NOTHING",
                &[&key, &owner.as_str()],
            )
            .map_err(|error| database_error("lock_acquire", &error))?;
        if inserted != 1 {
            return Err(state_error(
                "state.lock_contended",
                "PostgreSQL lock is already held",
            ));
        }
        Ok(LockLease {
            key: key.to_owned(),
            owner: owner.clone(),
        })
    }

    fn release_lock(&self, lease: &LockLease) -> Result<(), WorkflowOsError> {
        let mut client = self.connections.connect()?;
        let deleted = client
            .execute(
                "DELETE FROM workflow_os.local_locks WHERE lock_key = $1 AND owner = $2",
                &[&lease.key, &lease.owner.as_str()],
            )
            .map_err(|error| database_error("lock_release", &error))?;
        if deleted != 1 {
            return Err(state_error(
                "state.lock_owner_mismatch",
                "PostgreSQL lock lease is stale or owned by another actor",
            ));
        }
        Ok(())
    }
}

impl ApprovalStore for PostgresStateBackend {
    fn save_approval_request(&self, request: &ApprovalRequest) -> Result<(), WorkflowOsError> {
        request.validate_subject()?;
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| {
            let expected_revision = current_revision_tx(tx, "approval", &request.approval_id, "")?;
            put_record(
                tx,
                "approval",
                &request.approval_id,
                "",
                request,
                expected_revision,
                false,
            )
            .map(|_| ())
        })
    }

    fn load_approval_request(
        &self,
        approval_id: &str,
    ) -> Result<Option<ApprovalRequest>, WorkflowOsError> {
        let mut client = self.connections.connect()?;
        Ok(
            read_record::<ApprovalRequest>(&mut client, "approval", approval_id, "")?
                .map(|record| record.value),
        )
    }

    fn delete_approval_request(&self, approval_id: &str) -> Result<(), WorkflowOsError> {
        let mut client = self.connections.connect()?;
        client
            .execute(
                "DELETE FROM workflow_os.records
                  WHERE family = 'approval' AND key1 = $1 AND key2 = ''",
                &[&approval_id],
            )
            .map_err(|error| database_error("approval_delete", &error))?;
        Ok(())
    }
}

impl ApprovalPresentationRecordStore for PostgresStateBackend {
    fn write_approval_presentation_record(
        &self,
        record: &ApprovalPresentationRecord,
    ) -> Result<(), WorkflowOsError> {
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| {
            put_record(
                tx,
                "approval_presentation",
                record.presentation_id().as_str(),
                record.run_id().as_str(),
                record,
                None,
                true,
            )
            .map(|_| ())
        })
    }

    fn read_approval_presentation_record(
        &self,
        presentation_id: &ApprovalPresentationId,
    ) -> Result<Option<ApprovalPresentationRecord>, WorkflowOsError> {
        let mut client = self.connections.connect()?;
        let row = client
            .query_opt(
                "SELECT payload, revision FROM workflow_os.records
                  WHERE family = 'approval_presentation' AND key1 = $1",
                &[&presentation_id.as_str()],
            )
            .map_err(|error| database_error("presentation_read", &error))?;
        decode_optional_record(row).map(|record| record.map(|value| value.value))
    }

    fn list_approval_presentation_records(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<Vec<ApprovalPresentationRecord>, WorkflowOsError> {
        self.list_records_by_key2("approval_presentation", run_id.as_str())
    }

    fn list_approval_presentation_records_for_approval(
        &self,
        run_id: &WorkflowRunId,
        approval_id: &str,
    ) -> Result<Vec<ApprovalPresentationRecord>, WorkflowOsError> {
        crate::validate_approval_presentation_approval_id(approval_id)?;
        Ok(self
            .list_approval_presentation_records(run_id)?
            .into_iter()
            .filter(|record| record.approval_id() == approval_id)
            .collect())
    }
}

impl ProjectStateStore for PostgresStateBackend {
    fn save_project_state(&self, state: &ProjectStateRecord) -> Result<(), WorkflowOsError> {
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| {
            let expected_revision =
                current_revision_tx(tx, "project", state.project_id.as_str(), "")?;
            put_record(
                tx,
                "project",
                state.project_id.as_str(),
                "",
                state,
                expected_revision,
                false,
            )
            .map(|_| ())
        })
    }

    fn load_project_state(
        &self,
        project_id: &ProjectId,
    ) -> Result<Option<ProjectStateRecord>, WorkflowOsError> {
        let mut client = self.connections.connect()?;
        Ok(
            read_record::<ProjectStateRecord>(&mut client, "project", project_id.as_str(), "")?
                .map(|record| record.value),
        )
    }
}

impl PolicyAuditStore for PostgresStateBackend {
    fn append_policy_audit_record(
        &self,
        record: &PolicyAuditRecord,
    ) -> Result<(), WorkflowOsError> {
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| {
            put_record(
                tx,
                "policy_audit",
                record.audit_id.as_str(),
                record
                    .workflow_run_id
                    .as_ref()
                    .map_or("", WorkflowRunId::as_str),
                record,
                None,
                true,
            )
            .map(|_| ())
        })
    }

    fn read_policy_audit_records(&self) -> Result<Vec<PolicyAuditRecord>, WorkflowOsError> {
        self.list_records("policy_audit")
    }
}

impl crate::AdapterTelemetryStore for PostgresStateBackend {
    fn append_adapter_audit_record(
        &self,
        record: &AdapterRuntimeAuditRecord,
    ) -> Result<(), WorkflowOsError> {
        self.insert_telemetry(
            "adapter_audit",
            record.telemetry_id.as_str(),
            record
                .workflow_run_id
                .as_ref()
                .map_or("", WorkflowRunId::as_str),
            record,
        )
    }

    fn read_adapter_audit_records(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<Vec<AdapterRuntimeAuditRecord>, WorkflowOsError> {
        self.list_records_by_key2("adapter_audit", run_id.as_str())
    }

    fn append_adapter_observability_record(
        &self,
        record: &AdapterRuntimeObservabilityRecord,
    ) -> Result<(), WorkflowOsError> {
        self.insert_telemetry(
            "adapter_observability",
            record.telemetry_id.as_str(),
            record
                .workflow_run_id
                .as_ref()
                .map_or("", WorkflowRunId::as_str),
            record,
        )
    }

    fn read_adapter_observability_records(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<Vec<AdapterRuntimeObservabilityRecord>, WorkflowOsError> {
        self.list_records_by_key2("adapter_observability", run_id.as_str())
    }
}

impl WorkReportArtifactStore for PostgresStateBackend {
    fn write_work_report_artifact(
        &self,
        artifact: &WorkReportArtifactRecord,
    ) -> Result<(), WorkflowOsError> {
        artifact.validate()?;
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| {
            put_record(
                tx,
                "work_report",
                artifact.report_id().as_str(),
                artifact.run_id().as_str(),
                artifact,
                None,
                true,
            )
            .map(|_| ())
        })
    }

    fn read_work_report_artifact(
        &self,
        run_id: &WorkflowRunId,
        report_id: &WorkReportId,
    ) -> Result<Option<WorkReportArtifactRecord>, WorkflowOsError> {
        let mut client = self.connections.connect()?;
        Ok(read_record::<WorkReportArtifactRecord>(
            &mut client,
            "work_report",
            report_id.as_str(),
            run_id.as_str(),
        )?
        .map(|record| record.value))
    }

    fn list_work_report_artifacts(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<Vec<WorkReportArtifactRecord>, WorkflowOsError> {
        self.list_records_by_key2("work_report", run_id.as_str())
    }
}

impl SideEffectRecordStore for PostgresStateBackend {
    fn write_side_effect_record(&self, record: &SideEffectRecord) -> Result<(), WorkflowOsError> {
        record.validate()?;
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| {
            insert_side_effect_create_only(tx, record).map(|_| ())
        })
    }

    fn update_side_effect_record(&self, record: &SideEffectRecord) -> Result<(), WorkflowOsError> {
        record.validate()?;
        let prior = self.read_revisioned_side_effect(record.side_effect_id())?;
        let Some(prior) = prior else {
            return Err(state_error(
                "postgres_state.side_effect.missing",
                "PostgreSQL SideEffect record is missing",
            ));
        };
        validate_side_effect_identity(prior.value(), record)?;
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| {
            update_side_effect_cas(tx, record, prior.revision()).map(|_| ())
        })
    }

    fn read_side_effect_record(
        &self,
        side_effect_id: &SideEffectId,
    ) -> Result<Option<SideEffectRecord>, WorkflowOsError> {
        Ok(self
            .read_revisioned_side_effect(side_effect_id)?
            .map(|record| record.value))
    }

    fn list_side_effect_records(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<Vec<SideEffectRecord>, WorkflowOsError> {
        self.list_records_by_key2("side_effect", run_id.as_str())
    }

    fn list_side_effect_records_for_workflow_run(
        &self,
        workflow_id: &crate::WorkflowId,
        run_id: &WorkflowRunId,
    ) -> Result<Vec<SideEffectRecord>, WorkflowOsError> {
        let records = self.list_side_effect_records(run_id)?;
        if records
            .iter()
            .any(|record| record.workflow_id() != workflow_id || record.run_id() != run_id)
        {
            return Err(state_error(
                "postgres_state.side_effect.identity_mismatch",
                "stored SideEffect identity does not match requested workflow run",
            ));
        }
        Ok(records)
    }
}

impl StateBackend for PostgresStateBackend {
    fn health_check(&self) -> Result<BackendHealthCheck, WorkflowOsError> {
        let report = self.detailed_health_check()?;
        Ok(BackendHealthCheck {
            healthy: report.healthy,
            backend: "postgresql".to_owned(),
            message: if report.healthy {
                "PostgreSQL state backend is healthy".to_owned()
            } else {
                "PostgreSQL state backend requires operator attention".to_owned()
            },
        })
    }
}

impl DurableStateContractProvider for PostgresStateBackend {
    fn durable_state_contract(&self) -> Result<DurableStateSemanticContract, WorkflowOsError> {
        DurableStateSemanticContract::new(
            DurableStateContractVersion::V1,
            DurableStateBackendKind::SharedPostgresql,
            vec![
                DurableStateCapability::OrderedEventAppend,
                DurableStateCapability::ImmutableRunIdentityValidation,
                DurableStateCapability::IdempotencyReplay,
                DurableStateCapability::ProcessLocalExclusiveLock,
                DurableStateCapability::CrossRecordAtomicCommit,
                DurableStateCapability::CompareAndSetRevision,
                DurableStateCapability::ExpiringFencedLease,
                DurableStateCapability::ManagedSchemaMigration,
                DurableStateCapability::VerifiedBackupRestore,
                DurableStateCapability::SharedWorkerConcurrency,
            ],
            DurableStateTransactionKind::all()
                .iter()
                .copied()
                .map(|kind| {
                    DurableStateTransactionSupport::new(kind, DurableStateSupport::Supported)
                })
                .collect(),
            DurableLeaseSemantics::ExpiringFenced,
            DurableStateSchemaMetadata::managed(
                SCHEMA_VERSION as u32,
                DurableStateSchemaPosture::Ready,
            )?,
        )
    }
}

impl PostgresStateBackend {
    /// Loads a revisioned `SideEffect` record for compare-and-set transitions.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error for invalid or unavailable stored data.
    pub fn read_revisioned_side_effect(
        &self,
        side_effect_id: &SideEffectId,
    ) -> Result<Option<PostgresRevisionedRecord<SideEffectRecord>>, WorkflowOsError> {
        let mut client = self.connections.connect()?;
        let row = client
            .query_opt(
                "SELECT payload, revision FROM workflow_os.records
                  WHERE family = 'side_effect' AND key1 = $1",
                &[&side_effect_id.as_str()],
            )
            .map_err(|error| database_error("side_effect_read", &error))?;
        decode_optional_record(row)
    }

    fn insert_telemetry<T: Serialize>(
        &self,
        family: &str,
        key1: &str,
        key2: &str,
        value: &T,
    ) -> Result<(), WorkflowOsError> {
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| {
            put_record(tx, family, key1, key2, value, None, true).map(|_| ())
        })
    }

    fn list_records<T: DeserializeOwned>(&self, family: &str) -> Result<Vec<T>, WorkflowOsError> {
        let mut client = self.connections.connect()?;
        let rows = client
            .query(
                "SELECT payload FROM workflow_os.records
                  WHERE family = $1 ORDER BY key1, key2",
                &[&family],
            )
            .map_err(|error| database_error("record_list", &error))?;
        rows.into_iter()
            .map(|row| decode(row.get::<_, String>(0).as_str()))
            .collect()
    }

    fn list_records_by_key2<T: DeserializeOwned>(
        &self,
        family: &str,
        key2: &str,
    ) -> Result<Vec<T>, WorkflowOsError> {
        let mut client = self.connections.connect()?;
        let rows = client
            .query(
                "SELECT payload FROM workflow_os.records
                  WHERE family = $1 AND key2 = $2 ORDER BY key1",
                &[&family, &key2],
            )
            .map_err(|error| database_error("record_list", &error))?;
        rows.into_iter()
            .map(|row| decode(row.get::<_, String>(0).as_str()))
            .collect()
    }
}

fn serializable<T>(
    client: &mut Client,
    mut operation: impl FnMut(&mut Transaction<'_>) -> Result<T, WorkflowOsError>,
) -> Result<T, WorkflowOsError> {
    let mut last_conflict = None;
    for _ in 0..MAX_TRANSACTION_ATTEMPTS {
        let mut transaction = client
            .build_transaction()
            .isolation_level(IsolationLevel::Serializable)
            .start()
            .map_err(|error| database_error("transaction_begin", &error))?;
        match operation(&mut transaction) {
            Ok(value) => match transaction.commit() {
                Ok(()) => return Ok(value),
                Err(error) if is_retryable_database_error(&error) => {
                    last_conflict = Some(database_error("transaction_commit", &error));
                }
                Err(error) => return Err(database_error("transaction_commit", &error)),
            },
            Err(error) if error.code() == "postgres_state.transaction.retryable" => {
                last_conflict = Some(error);
            }
            Err(error) => return Err(error),
        }
    }
    Err(last_conflict.unwrap_or_else(|| {
        state_error(
            "postgres_state.transaction.retry_exhausted",
            "PostgreSQL transaction retry budget was exhausted",
        )
    }))
}

fn append_event_tx(
    tx: &mut Transaction<'_>,
    event: &WorkflowRunEvent,
) -> Result<(), WorkflowOsError> {
    if tx
        .query_opt(
            "SELECT 1 FROM workflow_os.events WHERE event_id = $1",
            &[&event.event_id.as_str()],
        )
        .map_err(|error| database_error("event_identity_read", &error))?
        .is_some()
    {
        return Err(state_error(
            "state.event.duplicate_id",
            "event ID already exists",
        ));
    }
    let sequence = i64::try_from(event.sequence_number.get()).map_err(|_| {
        state_error(
            "postgres_state.event.sequence_invalid",
            "PostgreSQL event sequence is invalid",
        )
    })?;
    if tx
        .query_opt(
            "SELECT 1 FROM workflow_os.events
              WHERE run_id = $1 AND sequence_number = $2",
            &[&event.run_id.as_str(), &sequence],
        )
        .map_err(|error| database_error("event_identity_read", &error))?
        .is_some()
    {
        return Err(state_error(
            "state.event.duplicate_sequence",
            "event sequence number already exists for workflow run",
        ));
    }
    let existing = read_events_tx(tx, &event.run_id)?;
    crate::state::validate_append_against_history(&existing, event)?;
    tx.execute(
        "INSERT INTO workflow_os.events
           (run_id, sequence_number, event_id, workflow_id, schema_version,
            workflow_version, spec_hash, payload)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        &[
            &event.run_id.as_str(),
            &sequence,
            &event.event_id.as_str(),
            &event.workflow_id.as_str(),
            &event.schema_version.as_str(),
            &event.workflow_version.as_str(),
            &event.spec_content_hash.as_str(),
            &encode(event)?,
        ],
    )
    .map_err(|error| database_error("event_append", &error))?;
    Ok(())
}

fn read_events_tx(
    tx: &mut Transaction<'_>,
    run_id: &WorkflowRunId,
) -> Result<Vec<WorkflowRunEvent>, WorkflowOsError> {
    let rows = tx
        .query(
            "SELECT run_id, sequence_number, event_id, workflow_id, schema_version,
                    workflow_version, spec_hash, payload
               FROM workflow_os.events
              WHERE run_id = $1
              ORDER BY sequence_number
              FOR UPDATE",
            &[&run_id.as_str()],
        )
        .map_err(|error| database_error("event_read", &error))?;
    decode_event_rows(rows, run_id)
}

fn read_events_client(
    client: &mut Client,
    run_id: &WorkflowRunId,
) -> Result<Vec<WorkflowRunEvent>, WorkflowOsError> {
    let rows = client
        .query(
            "SELECT run_id, sequence_number, event_id, workflow_id, schema_version,
                    workflow_version, spec_hash, payload
               FROM workflow_os.events
              WHERE run_id = $1
              ORDER BY sequence_number",
            &[&run_id.as_str()],
        )
        .map_err(|error| database_error("event_read", &error))?;
    decode_event_rows(rows, run_id)
}

fn decode_event_rows(
    rows: Vec<postgres::Row>,
    requested_run_id: &WorkflowRunId,
) -> Result<Vec<WorkflowRunEvent>, WorkflowOsError> {
    let mut events = Vec::with_capacity(rows.len());
    for row in rows {
        let stored_run_id: String = row.get(0);
        let stored_sequence: i64 = row.get(1);
        let stored_event_id: String = row.get(2);
        let stored_workflow_id: String = row.get(3);
        let stored_schema_version: String = row.get(4);
        let stored_workflow_version: String = row.get(5);
        let stored_spec_hash: String = row.get(6);
        let event: WorkflowRunEvent = decode(row.get::<_, String>(7).as_str())?;
        let sequence = u64::try_from(stored_sequence).map_err(|_| {
            state_error(
                "postgres_state.event.identity_mismatch",
                "stored PostgreSQL event identity is invalid",
            )
        })?;
        if stored_run_id != requested_run_id.as_str()
            || event.run_id != *requested_run_id
            || event.sequence_number.get() != sequence
            || event.event_id.as_str() != stored_event_id
            || event.workflow_id.as_str() != stored_workflow_id
            || event.schema_version.as_str() != stored_schema_version
            || event.workflow_version.as_str() != stored_workflow_version
            || event.spec_content_hash.as_str() != stored_spec_hash
        {
            return Err(state_error(
                "postgres_state.event.identity_mismatch",
                "stored PostgreSQL event identity does not match payload",
            ));
        }
        events.push(event);
    }
    if !events.is_empty() {
        WorkflowRun::rehydrate(&events)?;
    }
    Ok(events)
}

fn put_record<T: Serialize>(
    tx: &mut Transaction<'_>,
    family: &str,
    key1: &str,
    key2: &str,
    value: &T,
    expected_revision: Option<DurableRevision>,
    create_only: bool,
) -> Result<DurableRevision, WorkflowOsError> {
    let payload = encode(value)?;
    if create_only {
        let inserted = tx
            .execute(
                "INSERT INTO workflow_os.records
                   (family, key1, key2, payload, revision)
                 VALUES ($1, $2, $3, $4, 1)
                 ON CONFLICT DO NOTHING",
                &[&family, &key1, &key2, &payload],
            )
            .map_err(|error| database_error("record_create", &error))?;
        if inserted != 1 {
            return Err(state_error(
                "postgres_state.record.exists",
                "PostgreSQL create-only record already exists",
            ));
        }
        return DurableRevision::new(1);
    }
    if let Some(expected) = expected_revision {
        let expected = i64::try_from(expected.get()).map_err(|_| {
            state_error(
                "postgres_state.revision.invalid",
                "PostgreSQL expected revision is invalid",
            )
        })?;
        let row = tx
            .query_opt(
                "UPDATE workflow_os.records
                    SET payload = $4, revision = revision + 1,
                        updated_at = clock_timestamp()
                  WHERE family = $1 AND key1 = $2 AND key2 = $3
                    AND revision = $5
                RETURNING revision",
                &[&family, &key1, &key2, &payload, &expected],
            )
            .map_err(|error| database_error("record_update", &error))?;
        let Some(row) = row else {
            return Err(state_error(
                "postgres_state.revision.stale",
                "PostgreSQL record revision is stale",
            ));
        };
        revision_from_i64(row.get(0))
    } else {
        let row = tx
            .query_one(
                "INSERT INTO workflow_os.records
                   (family, key1, key2, payload, revision)
                 VALUES ($1, $2, $3, $4, 1)
                 ON CONFLICT (family, key1, key2) DO UPDATE SET
                   payload = EXCLUDED.payload,
                   revision = workflow_os.records.revision + 1,
                   updated_at = clock_timestamp()
                 RETURNING revision",
                &[&family, &key1, &key2, &payload],
            )
            .map_err(|error| database_error("record_upsert", &error))?;
        revision_from_i64(row.get(0))
    }
}

fn current_revision_tx(
    tx: &mut Transaction<'_>,
    family: &str,
    key1: &str,
    key2: &str,
) -> Result<Option<DurableRevision>, WorkflowOsError> {
    tx.query_opt(
        "SELECT revision FROM workflow_os.records
          WHERE family = $1 AND key1 = $2 AND key2 = $3
          FOR UPDATE",
        &[&family, &key1, &key2],
    )
    .map_err(|error| database_error("record_revision_read", &error))?
    .map(|row| revision_from_i64(row.get(0)))
    .transpose()
}

fn read_record<T: DeserializeOwned>(
    client: &mut Client,
    family: &str,
    key1: &str,
    key2: &str,
) -> Result<Option<PostgresRevisionedRecord<T>>, WorkflowOsError> {
    let row = client
        .query_opt(
            "SELECT payload, revision FROM workflow_os.records
              WHERE family = $1 AND key1 = $2 AND key2 = $3",
            &[&family, &key1, &key2],
        )
        .map_err(|error| database_error("record_read", &error))?;
    decode_optional_record(row)
}

fn decode_optional_record<T: DeserializeOwned>(
    row: Option<postgres::Row>,
) -> Result<Option<PostgresRevisionedRecord<T>>, WorkflowOsError> {
    row.map(|row| {
        Ok(PostgresRevisionedRecord {
            value: decode(row.get::<_, String>(0).as_str())?,
            revision: revision_from_i64(row.get(1))?,
        })
    })
    .transpose()
}

fn insert_side_effect_create_only(
    tx: &mut Transaction<'_>,
    record: &SideEffectRecord,
) -> Result<DurableRevision, WorkflowOsError> {
    put_record(
        tx,
        "side_effect",
        record.side_effect_id().as_str(),
        record.run_id().as_str(),
        record,
        None,
        true,
    )
}

fn update_side_effect_cas(
    tx: &mut Transaction<'_>,
    record: &SideEffectRecord,
    expected_revision: DurableRevision,
) -> Result<DurableRevision, WorkflowOsError> {
    let row = tx
        .query_opt(
            "SELECT payload FROM workflow_os.records
              WHERE family = 'side_effect' AND key1 = $1 AND key2 = $2
              FOR UPDATE",
            &[&record.side_effect_id().as_str(), &record.run_id().as_str()],
        )
        .map_err(|error| database_error("side_effect_read", &error))?
        .ok_or_else(|| {
            state_error(
                "postgres_state.side_effect.missing",
                "PostgreSQL SideEffect record is missing",
            )
        })?;
    let prior: SideEffectRecord = decode(row.get::<_, String>(0).as_str())?;
    validate_side_effect_identity(&prior, record)?;
    put_record(
        tx,
        "side_effect",
        record.side_effect_id().as_str(),
        record.run_id().as_str(),
        record,
        Some(expected_revision),
        false,
    )
}

fn validate_side_effect_identity(
    prior: &SideEffectRecord,
    next: &SideEffectRecord,
) -> Result<(), WorkflowOsError> {
    if prior.side_effect_id() != next.side_effect_id()
        || prior.workflow_id() != next.workflow_id()
        || prior.workflow_version() != next.workflow_version()
        || prior.schema_version() != next.schema_version()
        || prior.spec_hash() != next.spec_hash()
        || prior.run_id() != next.run_id()
        || prior.step_id() != next.step_id()
        || prior.skill_id() != next.skill_id()
        || prior.skill_version() != next.skill_version()
    {
        return Err(state_error(
            "postgres_state.side_effect.identity_mismatch",
            "PostgreSQL SideEffect immutable identity does not match",
        ));
    }
    if prior.lifecycle_state() == next.lifecycle_state() {
        return Err(state_error(
            "postgres_state.side_effect.transition_invalid",
            "PostgreSQL SideEffect transition must advance lifecycle state",
        ));
    }
    Ok(())
}

fn validate_side_effect_event_binding(
    record: &SideEffectRecord,
    event: &WorkflowRunEvent,
) -> Result<(), WorkflowOsError> {
    let (crate::WorkflowRunEventKind::SideEffectProposed(payload)
    | crate::WorkflowRunEventKind::SideEffectDenied(payload)
    | crate::WorkflowRunEventKind::SideEffectSkipped(payload)
    | crate::WorkflowRunEventKind::SideEffectAttempted(payload)
    | crate::WorkflowRunEventKind::SideEffectCompleted(payload)
    | crate::WorkflowRunEventKind::SideEffectFailed(payload)) = &event.kind
    else {
        return Err(state_error(
            "postgres_state.side_effect.event_invalid",
            "PostgreSQL SideEffect transaction requires a SideEffect workflow event",
        ));
    };
    if payload.side_effect_id() != record.side_effect_id()
        || payload.lifecycle_state() != record.lifecycle_state()
        || event.run_id != *record.run_id()
        || event.workflow_id != *record.workflow_id()
        || event.workflow_version != *record.workflow_version()
        || event.schema_version != *record.schema_version()
        || event.spec_content_hash != *record.spec_hash()
    {
        return Err(state_error(
            "postgres_state.side_effect.event_mismatch",
            "PostgreSQL SideEffect event does not match the transitioned record",
        ));
    }
    Ok(())
}

fn validate_approval_event_binding(
    approval: &ApprovalRequest,
    event: &WorkflowRunEvent,
) -> Result<(), WorkflowOsError> {
    let Some(approval_decision) = approval.decision.as_ref() else {
        return Err(state_error(
            "postgres_state.approval.decision_missing",
            "PostgreSQL approval transaction requires a decision",
        ));
    };
    let (crate::WorkflowRunEventKind::ApprovalGranted(event_decision)
    | crate::WorkflowRunEventKind::ApprovalDenied(event_decision)) = &event.kind
    else {
        return Err(state_error(
            "postgres_state.approval.event_invalid",
            "PostgreSQL approval transaction requires an approval decision event",
        ));
    };
    if event_decision != approval_decision
        || approval.approval_id != approval_decision.approval_id
        || event.run_id != approval.run_id
        || event.workflow_id != approval.workflow_id
        || event.workflow_version != approval.workflow_version
        || event.schema_version != approval.schema_version
        || event.spec_content_hash != approval.spec_content_hash
    {
        return Err(state_error(
            "postgres_state.approval.event_mismatch",
            "PostgreSQL approval event does not match the approval projection",
        ));
    }
    Ok(())
}

fn insert_content_addressed_definition(
    tx: &mut Transaction<'_>,
    record: &ImmutableRunBundleDefinitionRecord,
) -> Result<(), WorkflowOsError> {
    insert_content_addressed(
        tx,
        "immutable_definition",
        record.canonical_record_hash().as_str(),
        record,
    )
}

fn insert_content_addressed_local_check_set(
    tx: &mut Transaction<'_>,
    record: &crate::CanonicalLocalCheckDeclarationSetRecord,
) -> Result<(), WorkflowOsError> {
    insert_content_addressed(
        tx,
        "immutable_local_check_set",
        record.declaration_set_fingerprint().as_str(),
        record,
    )
}

fn insert_content_addressed<T: Serialize>(
    tx: &mut Transaction<'_>,
    family: &str,
    hash: &str,
    record: &T,
) -> Result<(), WorkflowOsError> {
    let payload = encode(record)?;
    let existing = tx
        .query_opt(
            "SELECT payload FROM workflow_os.content_records
              WHERE family = $1 AND content_hash = $2",
            &[&family, &hash],
        )
        .map_err(|error| database_error("content_record_read", &error))?;
    if let Some(row) = existing {
        if row.get::<_, String>(0) == payload {
            return Ok(());
        }
        return Err(state_error(
            "postgres_state.content_address.conflict",
            "PostgreSQL content-addressed record conflicts with existing content",
        ));
    }
    tx.execute(
        "INSERT INTO workflow_os.content_records
           (family, content_hash, payload) VALUES ($1, $2, $3)",
        &[&family, &hash, &payload],
    )
    .map_err(|error| database_error("content_record_write", &error))?;
    Ok(())
}

fn load_bundle_definitions(
    client: &mut Client,
    hashes: &[String],
) -> Result<Vec<ImmutableRunBundleDefinitionRecord>, WorkflowOsError> {
    let mut records = Vec::with_capacity(hashes.len());
    for hash in hashes {
        let row = client
            .query_opt(
                "SELECT payload FROM workflow_os.content_records
                  WHERE family = 'immutable_definition' AND content_hash = $1",
                &[hash],
            )
            .map_err(|error| database_error("content_record_read", &error))?
            .ok_or_else(|| {
                state_error(
                    "postgres_state.bundle.reference_missing",
                    "immutable run bundle definition reference is missing",
                )
            })?;
        let record: ImmutableRunBundleDefinitionRecord = decode(row.get::<_, String>(0).as_str())?;
        if record.canonical_record_hash().as_str() != hash {
            return Err(state_error(
                "postgres_state.bundle.identity_mismatch",
                "immutable definition content address does not match payload",
            ));
        }
        records.push(record);
    }
    Ok(records)
}

fn load_bundle_local_check_sets(
    client: &mut Client,
    hashes: &[String],
) -> Result<Vec<crate::CanonicalLocalCheckDeclarationSetRecord>, WorkflowOsError> {
    let mut records = Vec::with_capacity(hashes.len());
    for hash in hashes {
        let row = client
            .query_opt(
                "SELECT payload FROM workflow_os.content_records
                  WHERE family = 'immutable_local_check_set' AND content_hash = $1",
                &[hash],
            )
            .map_err(|error| database_error("content_record_read", &error))?
            .ok_or_else(|| {
                state_error(
                    "postgres_state.bundle.reference_missing",
                    "immutable run bundle local-check reference is missing",
                )
            })?;
        let record: crate::CanonicalLocalCheckDeclarationSetRecord =
            decode(row.get::<_, String>(0).as_str())?;
        if record.declaration_set_fingerprint().as_str() != hash {
            return Err(state_error(
                "postgres_state.bundle.identity_mismatch",
                "immutable local-check content address does not match payload",
            ));
        }
        records.push(record);
    }
    Ok(records)
}

fn validate_fence_tx(
    tx: &mut Transaction<'_>,
    lease: &PostgresFencedLease,
) -> Result<(), WorkflowOsError> {
    let fence = i64::try_from(lease.fence_token).map_err(|_| {
        state_error(
            "postgres_state.lease.fence_invalid",
            "PostgreSQL worker lease fencing token is invalid",
        )
    })?;
    let valid = tx
        .query_opt(
            "SELECT 1 FROM workflow_os.worker_leases
              WHERE lease_key = $1 AND owner = $2 AND fence_token = $3
                AND expires_at > clock_timestamp()
              FOR UPDATE",
            &[&lease.key.as_str(), &lease.owner.as_str(), &fence],
        )
        .map_err(|error| database_error("lease_validate", &error))?
        .is_some();
    if !valid {
        return Err(state_error(
            "postgres_state.lease.stale",
            "PostgreSQL worker lease is expired or stale",
        ));
    }
    Ok(())
}

fn validate_ttl(ttl: Duration) -> Result<Duration, WorkflowOsError> {
    if ttl < Duration::from_millis(1) || ttl > Duration::from_secs(3_600) {
        return Err(state_error(
            "postgres_state.lease_ttl.invalid",
            "PostgreSQL lease duration must be between one millisecond and one hour",
        ));
    }
    Ok(ttl)
}

fn validate_lock_key(key: &str) -> Result<(), WorkflowOsError> {
    if key.is_empty() || key.len() > 256 || key.chars().any(char::is_control) {
        return Err(state_error(
            "postgres_state.lock_key.invalid",
            "PostgreSQL lock key is invalid",
        ));
    }
    Ok(())
}

fn revision_from_i64(value: i64) -> Result<DurableRevision, WorkflowOsError> {
    let value = u64::try_from(value).map_err(|_| {
        state_error(
            "postgres_state.revision.invalid",
            "stored PostgreSQL revision is invalid",
        )
    })?;
    DurableRevision::new(value)
}

fn encode<T: Serialize>(value: &T) -> Result<String, WorkflowOsError> {
    serde_json::to_string(value).map_err(|_| {
        state_error(
            "postgres_state.serialization.failed",
            "failed to encode PostgreSQL state record",
        )
    })
}

fn decode<T: DeserializeOwned>(payload: &str) -> Result<T, WorkflowOsError> {
    serde_json::from_str(payload).map_err(|_| {
        state_error(
            "postgres_state.deserialization.failed",
            "failed to decode PostgreSQL state record",
        )
    })
}

fn is_retryable_database_error(error: &postgres::Error) -> bool {
    error.code().is_some_and(|code| {
        code == &SqlState::T_R_SERIALIZATION_FAILURE || code == &SqlState::T_R_DEADLOCK_DETECTED
    })
}

fn database_error(operation: &str, error: &postgres::Error) -> WorkflowOsError {
    let (kind, code, message) = if is_retryable_database_error(error) {
        (
            WorkflowOsErrorKind::InvalidState,
            "postgres_state.transaction.retryable",
            "PostgreSQL transaction encountered a retryable conflict",
        )
    } else if error.code() == Some(&SqlState::UNIQUE_VIOLATION) {
        (
            WorkflowOsErrorKind::InvalidState,
            "postgres_state.identity.conflict",
            "PostgreSQL state identity conflicts with an existing record",
        )
    } else {
        (
            WorkflowOsErrorKind::InvalidState,
            "postgres_state.backend.unavailable",
            "PostgreSQL state operation failed",
        )
    };
    let _ = operation;
    WorkflowOsError::new(kind, code, message)
}

fn state_error(code: &'static str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::new(WorkflowOsErrorKind::InvalidState, code, message)
}

fn looks_secret_like(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    [
        "authorization",
        "bearer ",
        "password",
        "private_key",
        "private key",
        "api_key",
        "api-key",
        "secret",
        "token=",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
}

const SCHEMA_SQL: &str = r"
CREATE SCHEMA IF NOT EXISTS workflow_os;

CREATE TABLE IF NOT EXISTS workflow_os.schema_metadata (
    singleton BOOLEAN PRIMARY KEY DEFAULT TRUE CHECK (singleton),
    schema_version INTEGER NOT NULL CHECK (schema_version > 0),
    checksum TEXT NOT NULL,
    recovery_required BOOLEAN NOT NULL DEFAULT FALSE,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

CREATE TABLE IF NOT EXISTS workflow_os.events (
    run_id TEXT NOT NULL,
    sequence_number BIGINT NOT NULL CHECK (sequence_number > 0),
    event_id TEXT NOT NULL UNIQUE,
    workflow_id TEXT NOT NULL,
    schema_version TEXT NOT NULL,
    workflow_version TEXT NOT NULL,
    spec_hash TEXT NOT NULL,
    payload TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    PRIMARY KEY (run_id, sequence_number)
);

CREATE TABLE IF NOT EXISTS workflow_os.records (
    family TEXT NOT NULL,
    key1 TEXT NOT NULL,
    key2 TEXT NOT NULL DEFAULT '',
    payload TEXT NOT NULL,
    revision BIGINT NOT NULL CHECK (revision > 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    PRIMARY KEY (family, key1, key2)
);
CREATE INDEX IF NOT EXISTS records_family_key2_idx
    ON workflow_os.records (family, key2, key1);

CREATE TABLE IF NOT EXISTS workflow_os.idempotency (
    key TEXT PRIMARY KEY,
    payload TEXT NOT NULL,
    intent_ref TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

CREATE TABLE IF NOT EXISTS workflow_os.local_locks (
    lock_key TEXT PRIMARY KEY,
    owner TEXT NOT NULL,
    acquired_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

CREATE TABLE IF NOT EXISTS workflow_os.worker_leases (
    lease_key TEXT PRIMARY KEY,
    owner TEXT NOT NULL,
    fence_token BIGINT NOT NULL CHECK (fence_token > 0),
    expires_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE IF NOT EXISTS workflow_os.content_records (
    family TEXT NOT NULL,
    content_hash TEXT NOT NULL,
    payload TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    PRIMARY KEY (family, content_hash)
);

CREATE TABLE IF NOT EXISTS workflow_os.immutable_manifests (
    run_id TEXT PRIMARY KEY,
    root_hash TEXT NOT NULL,
    payload TEXT NOT NULL,
    definition_hashes TEXT NOT NULL,
    local_check_hashes TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);
";
