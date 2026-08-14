use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use postgres::error::SqlState;
use postgres::{Client, Config, IsolationLevel, NoTls, Row, Transaction};
use serde::de::DeserializeOwned;
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::hosted::{
    HostedExecutionAttempt, HostedExecutionAttemptStatus, HostedExecutionProviderId,
    HostedExecutionProviderVersion, HostedSkillDispatch, HostedTerminalReportArtifact,
    HostedTerminalResultProjection, HostedUnreceiptedOutcome, HostedUnreceiptedResultProjection,
};
use crate::{
    validate_approval_presentation_for_request, ActorId, AdapterRuntimeAuditRecord,
    AdapterRuntimeObservabilityRecord, ApprovalPresentationId, ApprovalPresentationRecord,
    ApprovalPresentationRecordStore, ApprovalPresentationValidationInput, ApprovalReferenceId,
    ApprovalRequest, ApprovalStore, BackendHealthCheck, DurableLeaseSemantics, DurableRevision,
    DurableStateBackendKind, DurableStateCapability, DurableStateContractProvider,
    DurableStateContractVersion, DurableStateSchemaMetadata, DurableStateSchemaPosture,
    DurableStateSemanticContract, DurableStateSupport, DurableStateTransactionKind,
    DurableStateTransactionSupport, EventLogStore, HostedExecutionReceipt, HostedExecutionStatus,
    HostedProjectAccessDecision, HostedProjectCatalogVersion, HostedProjectResourceBinding,
    HostedProjectResourceBindingReader, HostedProjectResourceBindingStatus,
    HostedProjectResourceKind, HostedProjectScope, HostedWorkItem, HostedWorkItemId,
    HostedWorkItemStatus, IdempotencyKey, IdempotencyResult, IdempotencyStore, IdempotencyWrite,
    ImmutableRunBundleBuildResult, ImmutableRunBundleDefinitionRecord, ImmutableRunBundleId,
    ImmutableRunBundleManifest, LockLease, LockStore, OrganizationId, PolicyAuditRecord,
    PolicyAuditStore, ProjectApprovalAuthoritySnapshotCommitment,
    ProjectApprovalAuthorityViewCommitmentAlgorithm, ProjectApprovalNotificationPosture,
    ProjectApprovalRouteCreateResult, ProjectApprovalRouteLogicalSubjectId,
    ProjectApprovalRouteRecord, ProjectApprovalRouteRecordVersion,
    ProjectApprovalRouteSourceCommitmentAlgorithm, ProjectApprovalRouteStatus,
    ProjectApprovalRouteStore, ProjectApprovalRoutingReason, ProjectId, ProjectStateRecord,
    ProjectStateStore, RunSnapshotStore, SideEffectId, SideEffectRecord, SideEffectRecordStore,
    SpecContentHash, StateBackend, StoredImmutableRunBundle, WorkReportArtifactRecord,
    WorkReportArtifactStore, WorkReportId, WorkflowOsError, WorkflowOsErrorKind, WorkflowRun,
    WorkflowRunEvent, WorkflowRunId, WorkflowRunSnapshot, WorkflowRunStatus,
};

const SCHEMA_VERSION: i32 = 2;
const SCHEMA_CHECKSUM: &str = "workflow-os-postgresql-v2";
const SCHEMA_V1_VERSION: i32 = 1;
const SCHEMA_V1_CHECKSUM: &str = "workflow-os-postgresql-v1";
const MAX_TRANSACTION_ATTEMPTS: usize = 3;
const DEFAULT_LEASE_TTL: Duration = Duration::from_secs(30);
const MAX_PROJECT_APPROVAL_ROUTE_LIST_LIMIT: usize = 1_000;

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

/// Atomic hosted work-item creation request.
#[derive(Clone, Copy)]
pub struct PostgresCreateHostedWorkItemRequest<'a> {
    /// Validated queued work item.
    pub work_item: &'a HostedWorkItem,
}

/// Atomic authoritative invocation-event and hosted-work dispatch request.
#[derive(Clone, Copy)]
pub struct PostgresDispatchHostedSkillRequest<'a> {
    /// Core-validated dispatch projection.
    pub dispatch: &'a HostedSkillDispatch,
    /// Optional collaborative scope committed with the work item.
    pub project_scope: Option<&'a HostedProjectScope>,
}

/// Result of an idempotent hosted work-item creation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PostgresHostedWorkItemCreateResult {
    /// The work item was committed by this request.
    Created(PostgresRevisionedRecord<HostedWorkItem>),
    /// An exact idempotent replay returned the original work item.
    Replayed(PostgresRevisionedRecord<HostedWorkItem>),
}

/// Request for one deterministic fenced hosted work-item claim.
#[derive(Clone, Copy)]
pub struct PostgresClaimHostedWorkItemRequest<'a> {
    /// Worker identity.
    pub worker: &'a ActorId,
    /// Lease duration.
    pub lease_ttl: Duration,
}

/// One revisioned hosted work item claimed under an active fence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PostgresClaimedHostedWorkItem {
    work_item: PostgresRevisionedRecord<HostedWorkItem>,
    lease: PostgresFencedLease,
}

impl PostgresClaimedHostedWorkItem {
    /// Returns the claimed work item and committed revision.
    #[must_use]
    pub const fn work_item(&self) -> &PostgresRevisionedRecord<HostedWorkItem> {
        &self.work_item
    }

    /// Returns the active worker fence.
    #[must_use]
    pub const fn lease(&self) -> &PostgresFencedLease {
        &self.lease
    }
}

/// Fenced hosted work-item transition request.
#[derive(Clone, Copy)]
pub struct PostgresTransitionHostedWorkItemRequest<'a> {
    /// Expected durable work-item revision.
    pub expected_revision: DurableRevision,
    /// Exact validated next work item.
    pub work_item: &'a HostedWorkItem,
    /// Active worker fence. Required for transitions from `running`.
    pub lease: Option<&'a PostgresFencedLease>,
}

/// Atomic fenced hosted no-write receipt commit.
#[derive(Clone, Copy)]
pub struct PostgresCommitHostedReceiptRequest<'a> {
    /// Expected prior work-item revision.
    pub expected_work_item_revision: DurableRevision,
    /// Exact transitioned terminal work item.
    pub work_item: &'a HostedWorkItem,
    /// Validated terminal provider receipt.
    pub receipt: &'a HostedExecutionReceipt,
    /// Active worker fence.
    pub lease: &'a PostgresFencedLease,
}

/// Atomic hosted receipt, attempt, work-item, event, and run projection commit.
#[derive(Clone, Copy)]
pub struct PostgresCommitHostedReceiptProjectionRequest<'a> {
    /// Existing fenced receipt commit fields.
    pub receipt_commit: PostgresCommitHostedReceiptRequest<'a>,
    /// Expected durable invocation-attempt revision.
    pub expected_attempt_revision: DurableRevision,
    /// Core-validated terminal workflow projection.
    pub projection: &'a HostedTerminalResultProjection,
    /// Core-validated terminal report artifact.
    pub report_artifact: &'a HostedTerminalReportArtifact,
}

/// Atomic hosted failure or reconciliation projection without a provider
/// receipt.
#[derive(Clone, Copy)]
pub struct PostgresCommitHostedUnreceiptedProjectionRequest<'a> {
    /// Expected prior work-item revision.
    pub expected_work_item_revision: DurableRevision,
    /// Exact failed or ambiguous work item.
    pub work_item: &'a HostedWorkItem,
    /// Expected invocation-attempt revision for reconciliation outcomes.
    pub expected_attempt_revision: Option<DurableRevision>,
    /// Active worker fence.
    pub lease: &'a PostgresFencedLease,
    /// Core-validated workflow projection.
    pub projection: &'a HostedUnreceiptedResultProjection,
}

#[derive(Serialize)]
struct HostedExecutionAttemptIntent<'a> {
    execution_id: &'a crate::HostedExecutionId,
    work_item_id: &'a HostedWorkItemId,
    request_fingerprint: crate::HostedExecutionRequestFingerprint,
    provider_id: &'a HostedExecutionProviderId,
    provider_version: &'a HostedExecutionProviderVersion,
    provider_configuration_hash: &'a SpecContentHash,
}

/// Revision committed by one atomic hosted no-write receipt transaction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PostgresHostedReceiptCommitResult {
    work_item_revision: DurableRevision,
    attempt_revision: Option<DurableRevision>,
}

impl PostgresHostedReceiptCommitResult {
    /// Returns the committed work-item revision.
    #[must_use]
    pub const fn work_item_revision(self) -> DurableRevision {
        self.work_item_revision
    }

    /// Returns the committed invocation-attempt revision when the
    /// attempt-aware atomic commit path was used.
    #[must_use]
    pub const fn attempt_revision(self) -> Option<DurableRevision> {
        self.attempt_revision
    }
}

/// Bounded, low-cardinality hosted queue metrics observed using database time.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PostgresHostedQueueMetricsSnapshot {
    queued_work_items: u64,
    running_work_items: u64,
    waiting_work_items: u64,
    completed_work_items: u64,
    failed_work_items: u64,
    canceled_work_items: u64,
    ambiguous_work_items: u64,
    prepared_attempts: u64,
    invoking_attempts: u64,
    reconciliation_required_attempts: u64,
    terminal_attempts: u64,
    oldest_queued_age_ms: Option<u64>,
    observed_at_epoch_ms: i64,
}

impl PostgresHostedQueueMetricsSnapshot {
    /// Returns queued work-item count.
    #[must_use]
    pub const fn queued_work_items(self) -> u64 {
        self.queued_work_items
    }

    /// Returns running work-item count.
    #[must_use]
    pub const fn running_work_items(self) -> u64 {
        self.running_work_items
    }

    /// Returns waiting work-item count.
    #[must_use]
    pub const fn waiting_work_items(self) -> u64 {
        self.waiting_work_items
    }

    /// Returns completed work-item count.
    #[must_use]
    pub const fn completed_work_items(self) -> u64 {
        self.completed_work_items
    }

    /// Returns failed work-item count.
    #[must_use]
    pub const fn failed_work_items(self) -> u64 {
        self.failed_work_items
    }

    /// Returns canceled work-item count.
    #[must_use]
    pub const fn canceled_work_items(self) -> u64 {
        self.canceled_work_items
    }

    /// Returns ambiguous work-item count.
    #[must_use]
    pub const fn ambiguous_work_items(self) -> u64 {
        self.ambiguous_work_items
    }

    /// Returns prepared invocation-attempt count.
    #[must_use]
    pub const fn prepared_attempts(self) -> u64 {
        self.prepared_attempts
    }

    /// Returns invoking attempt count.
    #[must_use]
    pub const fn invoking_attempts(self) -> u64 {
        self.invoking_attempts
    }

    /// Returns reconciliation-required attempt count.
    #[must_use]
    pub const fn reconciliation_required_attempts(self) -> u64 {
        self.reconciliation_required_attempts
    }

    /// Returns terminal attempt count.
    #[must_use]
    pub const fn terminal_attempts(self) -> u64 {
        self.terminal_attempts
    }

    /// Returns database-time age of the oldest queued item, when any exist.
    #[must_use]
    pub const fn oldest_queued_age_ms(self) -> Option<u64> {
        self.oldest_queued_age_ms
    }

    /// Returns the database observation time as Unix epoch milliseconds.
    #[must_use]
    pub const fn observed_at_epoch_ms(self) -> i64 {
        self.observed_at_epoch_ms
    }
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
        tx.batch_execute(SCHEMA_BOOTSTRAP_SQL)
            .map_err(|error| database_error("schema_bootstrap", &error))?;
        let existing = tx
            .query_opt(
                "SELECT schema_version, checksum, recovery_required
                   FROM workflow_os.schema_metadata WHERE singleton = TRUE",
                &[],
            )
            .map_err(|error| database_error("schema_read", &error))?;
        match existing {
            None => {
                let existing_table_count: i64 = tx
                    .query_one(
                        "SELECT count(*) FROM information_schema.tables
                          WHERE table_schema = 'workflow_os'
                            AND table_name <> 'schema_metadata'",
                        &[],
                    )
                    .map_err(|error| database_error("schema_inventory", &error))?
                    .get(0);
                if existing_table_count != 0 {
                    return Err(state_error(
                        "postgres_state.schema.incompatible",
                        "PostgreSQL state schema is incompatible",
                    ));
                }
                tx.batch_execute(SCHEMA_V1_SQL)
                    .map_err(|error| database_error("schema_install_v1", &error))?;
                tx.batch_execute(SCHEMA_V2_SQL)
                    .map_err(|error| database_error("schema_install_v2", &error))?;
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
                if recovery_required {
                    return Err(state_error(
                        "postgres_state.schema.recovery_required",
                        "PostgreSQL state schema requires operator recovery",
                    ));
                }
                if version == SCHEMA_V1_VERSION && checksum == SCHEMA_V1_CHECKSUM {
                    tx.batch_execute(SCHEMA_V2_SQL)
                        .map_err(|error| database_error("schema_migrate_v2", &error))?;
                    tx.execute(
                        "UPDATE workflow_os.schema_metadata
                            SET schema_version = $1, checksum = $2,
                                updated_at = clock_timestamp()
                          WHERE singleton = TRUE",
                        &[&SCHEMA_VERSION, &SCHEMA_CHECKSUM],
                    )
                    .map_err(|error| database_error("schema_advance_v2", &error))?;
                } else if version != SCHEMA_VERSION || checksum != SCHEMA_CHECKSUM {
                    return Err(state_error(
                        "postgres_state.schema.incompatible",
                        "PostgreSQL state schema is incompatible",
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

    /// Creates or advances the deployment authority high watermark before hosted traffic starts.
    ///
    /// Exact replay is accepted. Rollback and same-revision content conflicts fail closed.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error for invalid revisions, rollback, conflicting content,
    /// or unavailable durable state.
    pub fn activate_project_approval_authority_high_watermark(
        &self,
        organization_id: &OrganizationId,
        snapshot: &ProjectApprovalAuthoritySnapshotCommitment,
    ) -> Result<(), WorkflowOsError> {
        if snapshot.authority_view().organization_id() != organization_id {
            return Err(state_error(
                "postgres_state.project_approval_authority.scope.mismatch",
                "project approval authority scope does not match the deployment",
            ));
        }
        let authority_revision = i64::try_from(snapshot.revision().get()).map_err(|_| {
            state_error(
                "postgres_state.project_approval_authority.revision.invalid",
                "project approval authority revision is invalid",
            )
        })?;
        let algorithm = authority_commitment_algorithm_label(snapshot.authority_view().algorithm());
        let fingerprint = snapshot.authority_view().fingerprint().as_str();
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| {
            let existing = tx
                .query_opt(
                    "SELECT authority_revision, commitment_algorithm, commitment_fingerprint
                       FROM workflow_os.hosted_authority_registry_high_watermarks
                      WHERE organization_id = $1 FOR UPDATE",
                    &[&organization_id.as_str()],
                )
                .map_err(|error| database_error("authority_high_watermark_read", &error))?;
            match existing {
                None => {
                    tx.execute(
                        "INSERT INTO workflow_os.hosted_authority_registry_high_watermarks
                           (organization_id, authority_revision, commitment_algorithm,
                            commitment_fingerprint)
                         VALUES ($1, $2, $3, $4)",
                        &[
                            &organization_id.as_str(),
                            &authority_revision,
                            &algorithm,
                            &fingerprint,
                        ],
                    )
                    .map_err(|error| database_error("authority_high_watermark_insert", &error))?;
                    Ok(())
                }
                Some(row) => {
                    let stored_revision: i64 = row.get(0);
                    let stored_algorithm: String = row.get(1);
                    let stored_fingerprint: String = row.get(2);
                    validate_authority_high_watermark_row(
                        stored_revision,
                        &stored_algorithm,
                        &stored_fingerprint,
                    )?;
                    if authority_revision < stored_revision {
                        return Err(state_error(
                            "postgres_state.project_approval_authority.revision.rollback",
                            "project approval authority revision would roll back durable state",
                        ));
                    }
                    if authority_revision == stored_revision {
                        if stored_algorithm == algorithm && stored_fingerprint == fingerprint {
                            return Ok(());
                        }
                        return Err(state_error(
                            "postgres_state.project_approval_authority.revision.conflict",
                            "project approval authority revision conflicts with durable state",
                        ));
                    }
                    tx.execute(
                        "UPDATE workflow_os.hosted_authority_registry_high_watermarks
                            SET authority_revision = $2, commitment_algorithm = $3,
                                commitment_fingerprint = $4,
                                accepted_at = clock_timestamp()
                          WHERE organization_id = $1",
                        &[
                            &organization_id.as_str(),
                            &authority_revision,
                            &algorithm,
                            &fingerprint,
                        ],
                    )
                    .map_err(|error| database_error("authority_high_watermark_advance", &error))?;
                    Ok(())
                }
            }
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
        validate_ttl(request.ttl)?;
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| acquire_fenced_lease_tx(tx, request))
    }

    /// Renews one exact live lease without changing its fencing token.
    ///
    /// # Errors
    ///
    /// Rejects an expired, replaced, or otherwise stale lease. Unlike
    /// acquisition, renewal never creates a lease and never advances a fence.
    pub fn renew_fenced_lease(
        &self,
        lease: &PostgresFencedLease,
        ttl: Duration,
    ) -> Result<PostgresFencedLease, WorkflowOsError> {
        validate_ttl(ttl)?;
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| renew_fenced_lease_tx(tx, lease, ttl))
    }

    /// Releases a fenced lease only when owner and token still match.
    ///
    /// # Errors
    ///
    /// Rejects stale holders rather than releasing a newer lease.
    pub fn release_fenced_lease(&self, lease: &PostgresFencedLease) -> Result<(), WorkflowOsError> {
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| release_fenced_lease_tx(tx, lease))
    }

    /// Atomically reserves idempotency and creates one queued hosted work item.
    ///
    /// # Errors
    ///
    /// Fails closed for a missing or mismatched immutable bundle, conflicting
    /// idempotency replay, duplicate identity, or unavailable durable state.
    pub fn create_hosted_work_item(
        &self,
        request: PostgresCreateHostedWorkItemRequest<'_>,
    ) -> Result<PostgresHostedWorkItemCreateResult, WorkflowOsError> {
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| {
            create_hosted_work_item_tx(tx, request.work_item)
        })
    }

    /// Atomically appends the authoritative invocation request/start events,
    /// updates the run snapshot, and creates the exactly bound hosted work item.
    ///
    /// # Errors
    ///
    /// Fails closed on stale run state, event substitution, idempotency
    /// conflict, immutable-bundle mismatch, or any partial durable write.
    pub fn dispatch_hosted_skill(
        &self,
        request: PostgresDispatchHostedSkillRequest<'_>,
    ) -> Result<PostgresHostedWorkItemCreateResult, WorkflowOsError> {
        let dispatch = request.dispatch;
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| {
            if let Some(scope) = request.project_scope {
                activate_reserved_project_resource_binding_tx(
                    tx,
                    scope,
                    HostedProjectResourceKind::Run,
                    dispatch.work_item().run_id().as_str(),
                )?;
            }
            if let Some(existing) =
                read_hosted_work_item_tx(tx, dispatch.work_item().work_item_id(), true)?
            {
                if existing.value() != dispatch.work_item()
                    || !event_exists_exact_tx(tx, dispatch.invocation_requested())?
                    || !event_exists_exact_tx(tx, dispatch.invocation_started())?
                {
                    return Err(state_error(
                        "postgres_state.hosted_dispatch.replay_conflict",
                        "hosted dispatch replay conflicts with durable state",
                    ));
                }
                if let Some(scope) = request.project_scope {
                    require_project_resource_binding_tx(
                        tx,
                        scope,
                        HostedProjectResourceKind::WorkItem,
                        dispatch.work_item().work_item_id().as_str(),
                    )?;
                }
                return Ok(PostgresHostedWorkItemCreateResult::Replayed(existing));
            }

            let snapshot_revision =
                current_snapshot_revision_tx(tx, dispatch.work_item().run_id())?;
            append_event_tx(tx, dispatch.invocation_requested())?;
            append_event_tx(tx, dispatch.invocation_started())?;
            let events = read_events_tx(tx, dispatch.work_item().run_id())?;
            let projected = WorkflowRun::rehydrate(&events)?;
            if projected.snapshot.status != WorkflowRunStatus::Running {
                return Err(state_error(
                    "postgres_state.hosted_dispatch.projection_invalid",
                    "hosted dispatch did not preserve a running projection",
                ));
            }
            put_record(
                tx,
                "snapshot",
                dispatch.work_item().run_id().as_str(),
                "",
                &projected.snapshot,
                Some(snapshot_revision),
                false,
            )?;
            let created = create_hosted_work_item_tx(tx, dispatch.work_item())?;
            if let Some(scope) = request.project_scope {
                create_project_resource_binding_tx(
                    tx,
                    &HostedProjectResourceBinding::new(
                        scope.clone(),
                        HostedProjectResourceKind::WorkItem,
                        dispatch.work_item().work_item_id().as_str(),
                        HostedProjectResourceBindingStatus::Active,
                        crate::Timestamp::now_utc(),
                    )?,
                )?;
            }
            Ok(created)
        })
    }

    /// Reserves one resource identity for an exact project scope.
    ///
    /// # Errors
    ///
    /// Fails closed on conflicting scope, invalid state, or storage failure.
    pub fn reserve_hosted_project_resource(
        &self,
        binding: &HostedProjectResourceBinding,
    ) -> Result<(), WorkflowOsError> {
        if binding.status() != HostedProjectResourceBindingStatus::Reserved {
            return Err(state_error(
                "postgres_state.hosted_project_binding.status.invalid",
                "hosted project resource reservation status is invalid",
            ));
        }
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| {
            create_project_resource_binding_tx(tx, binding)
        })
    }

    /// Activates an exact existing project resource reservation.
    ///
    /// # Errors
    ///
    /// Fails closed when the reservation is missing, mismatched, or unavailable.
    pub fn activate_hosted_project_resource(
        &self,
        scope: &HostedProjectScope,
        kind: HostedProjectResourceKind,
        resource_id: &str,
    ) -> Result<HostedProjectResourceBinding, WorkflowOsError> {
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| {
            let existing = read_project_resource_binding_tx(tx, kind, resource_id, true)?
                .ok_or_else(|| {
                    state_error(
                        "postgres_state.hosted_project_binding.missing",
                        "hosted project resource reservation is missing",
                    )
                })?;
            if existing.value().scope() != scope {
                return Err(state_error(
                    "postgres_state.hosted_project_binding.scope_conflict",
                    "hosted project resource scope conflicts with durable state",
                ));
            }
            let active = existing.value().activate()?;
            if &active == existing.value() {
                return Ok(active);
            }
            put_record(
                tx,
                "hosted_project_binding",
                resource_id,
                kind.storage_key(),
                &active,
                Some(existing.revision()),
                false,
            )?;
            Ok(active)
        })
    }

    /// Reads one durable project resource binding.
    ///
    /// # Errors
    ///
    /// Returns a stable error when storage is unavailable or invalid.
    pub fn read_hosted_project_resource_binding(
        &self,
        kind: HostedProjectResourceKind,
        resource_id: &str,
    ) -> Result<Option<PostgresRevisionedRecord<HostedProjectResourceBinding>>, WorkflowOsError>
    {
        let mut client = self.connections.connect()?;
        read_record(
            &mut client,
            "hosted_project_binding",
            resource_id,
            kind.storage_key(),
        )
    }

    /// Publishes one immutable project-scoped catalog version.
    ///
    /// # Errors
    ///
    /// Fails closed on invalid stewardship, immutable conflict, or storage failure.
    pub fn publish_hosted_project_catalog_version(
        &self,
        version: &HostedProjectCatalogVersion,
        stewardship: &crate::WorkflowStewardshipRecord,
    ) -> Result<(), WorkflowOsError> {
        if stewardship.decision_id() != version.stewardship_decision_id()
            || stewardship.workflow_id() != version.workflow_id()
            || stewardship.reviewer() != version.published_by()
            || stewardship.decision_kind()
                != crate::WorkflowStewardshipDecisionKind::ApprovedForPromotion
        {
            return Err(state_error(
                "postgres_state.hosted_project_catalog.stewardship_invalid",
                "hosted project catalog stewardship proof is invalid",
            ));
        }
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| {
            let stewardship_key = hosted_scope_storage_prefix(version.scope());
            match read_record_tx::<crate::WorkflowStewardshipRecord>(
                tx,
                "hosted_project_stewardship",
                &stewardship_key,
                stewardship.decision_id().as_str(),
                true,
            )? {
                Some(existing) if existing.value() == stewardship => {}
                Some(_) => {
                    return Err(state_error(
                        "postgres_state.hosted_project_catalog.stewardship_conflict",
                        "hosted project catalog stewardship proof conflicts with durable state",
                    ));
                }
                None => {
                    put_record(
                        tx,
                        "hosted_project_stewardship",
                        &stewardship_key,
                        stewardship.decision_id().as_str(),
                        stewardship,
                        None,
                        true,
                    )?;
                }
            }
            let key1 = hosted_catalog_storage_key(version.scope(), version.workflow_id());
            let key2 = version.workflow_version().as_str();
            match read_record_tx::<HostedProjectCatalogVersion>(
                tx,
                "hosted_project_catalog",
                &key1,
                key2,
                true,
            )? {
                Some(existing) if existing.value() == version => Ok(()),
                Some(_) => Err(state_error(
                    "postgres_state.hosted_project_catalog.conflict",
                    "hosted project catalog version conflicts with durable state",
                )),
                None => put_record(
                    tx,
                    "hosted_project_catalog",
                    &key1,
                    key2,
                    version,
                    None,
                    true,
                )
                .map(|_| ()),
            }
        })
    }

    /// Reads one immutable project-scoped catalog version.
    ///
    /// # Errors
    ///
    /// Returns a stable error when storage is unavailable or invalid.
    pub fn read_hosted_project_catalog_version(
        &self,
        scope: &HostedProjectScope,
        workflow_id: &crate::WorkflowId,
        workflow_version: &crate::WorkflowVersion,
    ) -> Result<Option<HostedProjectCatalogVersion>, WorkflowOsError> {
        let mut client = self.connections.connect()?;
        Ok(read_record::<HostedProjectCatalogVersion>(
            &mut client,
            "hosted_project_catalog",
            &hosted_catalog_storage_key(scope, workflow_id),
            workflow_version.as_str(),
        )?
        .map(PostgresRevisionedRecord::into_parts)
        .map(|parts| parts.0))
    }

    /// Lists immutable catalog versions for exactly one project scope.
    ///
    /// # Errors
    ///
    /// Fails closed on storage failure or decoded scope mismatch.
    pub fn list_hosted_project_catalog_versions(
        &self,
        scope: &HostedProjectScope,
    ) -> Result<Vec<HostedProjectCatalogVersion>, WorkflowOsError> {
        let prefix = hosted_scope_storage_prefix(scope);
        let mut client = self.connections.connect()?;
        let rows = client
            .query(
                "SELECT payload FROM workflow_os.records
                 WHERE family = $1 AND left(key1, char_length($2)) = $2
                 ORDER BY key1 ASC, key2 ASC",
                &[&"hosted_project_catalog", &prefix],
            )
            .map_err(|error| database_error("hosted_project_catalog_list", &error))?;
        let mut versions = rows
            .into_iter()
            .map(|row| decode::<HostedProjectCatalogVersion>(row.get::<_, String>(0).as_str()))
            .collect::<Result<Vec<_>, _>>()?;
        if versions.iter().any(|version| version.scope() != scope) {
            return Err(state_error(
                "postgres_state.hosted_project_catalog.scope_mismatch",
                "hosted project catalog scope conflicts with durable state",
            ));
        }
        versions.sort_by(|left, right| {
            left.workflow_id()
                .cmp(right.workflow_id())
                .then_with(|| left.workflow_version().cmp(right.workflow_version()))
        });
        Ok(versions)
    }

    /// Persists one bounded collaborative access decision for audit.
    ///
    /// # Errors
    ///
    /// Fails closed on duplicate identity or storage failure.
    pub fn write_hosted_project_access_decision(
        &self,
        decision: &HostedProjectAccessDecision,
    ) -> Result<(), WorkflowOsError> {
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| {
            put_record(
                tx,
                "hosted_project_access_decision",
                decision.decision_id().as_str(),
                &hosted_scope_storage_prefix(decision.scope()),
                decision,
                None,
                true,
            )
            .map(|_| ())
        })
    }

    /// Lists bounded authorization decisions for exactly one project scope.
    ///
    /// # Errors
    ///
    /// Fails closed on storage failure or decoded scope mismatch.
    pub fn list_hosted_project_access_decisions(
        &self,
        scope: &HostedProjectScope,
    ) -> Result<Vec<HostedProjectAccessDecision>, WorkflowOsError> {
        let scope_key = hosted_scope_storage_prefix(scope);
        let mut client = self.connections.connect()?;
        let rows = client
            .query(
                "SELECT payload FROM workflow_os.records
                 WHERE family = $1 AND key2 = $2
                 ORDER BY key1 ASC",
                &[&"hosted_project_access_decision", &scope_key],
            )
            .map_err(|error| database_error("hosted_project_access_decision_list", &error))?;
        let decisions = rows
            .into_iter()
            .map(|row| decode::<HostedProjectAccessDecision>(row.get::<_, String>(0).as_str()))
            .collect::<Result<Vec<_>, _>>()?;
        if decisions.iter().any(|decision| decision.scope() != scope) {
            return Err(state_error(
                "postgres_state.hosted_project_access_decision.scope_mismatch",
                "hosted project access decision scope conflicts with durable state",
            ));
        }
        Ok(decisions)
    }

    /// Reads one hosted work item across its indexed lifecycle status.
    ///
    /// # Errors
    ///
    /// Fails closed when multiple status rows, corrupt payloads, or identity
    /// mismatches are present.
    pub fn read_revisioned_hosted_work_item(
        &self,
        work_item_id: &HostedWorkItemId,
    ) -> Result<Option<PostgresRevisionedRecord<HostedWorkItem>>, WorkflowOsError> {
        let mut client = self.connections.connect()?;
        let mut transaction = client
            .transaction()
            .map_err(|error| database_error("hosted_work_item_read", &error))?;
        let result = read_hosted_work_item_tx(&mut transaction, work_item_id, false)?;
        transaction
            .commit()
            .map_err(|error| database_error("hosted_work_item_read", &error))?;
        Ok(result)
    }

    /// Persists an exact invocation identity before a provider call can start.
    ///
    /// An exact replay returns the original prepared attempt. Reusing the
    /// invocation identity with different request, provider, configuration, or
    /// work-item binding fails closed.
    ///
    /// # Errors
    ///
    /// Rejects stale work-item revisions or fences, non-running work, and
    /// conflicting invocation identity without exposing bound identifiers.
    #[allow(clippy::too_many_arguments)]
    pub fn prepare_hosted_execution_attempt(
        &self,
        expected_work_item_revision: DurableRevision,
        work_item_id: &HostedWorkItemId,
        execution_id: &crate::HostedExecutionId,
        provider_id: &HostedExecutionProviderId,
        provider_version: &HostedExecutionProviderVersion,
        provider_configuration_hash: &SpecContentHash,
        lease: &PostgresFencedLease,
    ) -> Result<PostgresRevisionedRecord<HostedExecutionAttempt>, WorkflowOsError> {
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| {
            prepare_hosted_execution_attempt_tx(
                tx,
                expected_work_item_revision,
                work_item_id,
                execution_id,
                provider_id,
                provider_version,
                provider_configuration_hash,
                lease,
            )
        })
    }

    /// Reads the single durable invocation attempt for a hosted work item.
    ///
    /// # Errors
    ///
    /// Fails closed on conflicting status rows, corrupt payloads, or identity
    /// mismatch.
    pub fn read_revisioned_hosted_execution_attempt(
        &self,
        work_item_id: &HostedWorkItemId,
    ) -> Result<Option<PostgresRevisionedRecord<HostedExecutionAttempt>>, WorkflowOsError> {
        let mut client = self.connections.connect()?;
        let mut transaction = client
            .transaction()
            .map_err(|error| database_error("hosted_execution_attempt_read", &error))?;
        let result = read_hosted_execution_attempt_tx(&mut transaction, work_item_id, false)?;
        transaction
            .commit()
            .map_err(|error| database_error("hosted_execution_attempt_read", &error))?;
        Ok(result)
    }

    /// Fenced CAS transition from `prepared` to `invoking`.
    ///
    /// # Errors
    ///
    /// Rejects stale revisions, stale fences, and invalid lifecycle posture.
    pub fn mark_hosted_execution_attempt_invoking(
        &self,
        work_item_id: &HostedWorkItemId,
        expected_attempt_revision: DurableRevision,
        lease: &PostgresFencedLease,
    ) -> Result<PostgresRevisionedRecord<HostedExecutionAttempt>, WorkflowOsError> {
        self.transition_hosted_execution_attempt(
            work_item_id,
            expected_attempt_revision,
            HostedExecutionAttemptStatus::Invoking,
            lease,
        )
    }

    /// Fenced CAS transition from `invoking` to reconciliation-required.
    ///
    /// # Errors
    ///
    /// Rejects stale revisions, stale fences, and invalid lifecycle posture.
    pub fn mark_hosted_execution_attempt_reconciliation_required(
        &self,
        work_item_id: &HostedWorkItemId,
        expected_attempt_revision: DurableRevision,
        lease: &PostgresFencedLease,
    ) -> Result<PostgresRevisionedRecord<HostedExecutionAttempt>, WorkflowOsError> {
        self.transition_hosted_execution_attempt(
            work_item_id,
            expected_attempt_revision,
            HostedExecutionAttemptStatus::ReconciliationRequired,
            lease,
        )
    }

    fn transition_hosted_execution_attempt(
        &self,
        work_item_id: &HostedWorkItemId,
        expected_attempt_revision: DurableRevision,
        target: HostedExecutionAttemptStatus,
        lease: &PostgresFencedLease,
    ) -> Result<PostgresRevisionedRecord<HostedExecutionAttempt>, WorkflowOsError> {
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| {
            let work_item = read_hosted_work_item_tx(tx, work_item_id, true)?.ok_or_else(|| {
                state_error(
                    "postgres_state.hosted_work_item.missing",
                    "hosted work item is missing",
                )
            })?;
            if work_item.value().status() != HostedWorkItemStatus::Running {
                return Err(state_error(
                    "postgres_state.hosted_execution_attempt.work_item_status.invalid",
                    "hosted execution attempt requires a running work item",
                ));
            }
            validate_hosted_work_item_lease(lease, work_item_id)?;
            validate_fence_tx(tx, lease)?;
            let prior =
                read_hosted_execution_attempt_tx(tx, work_item_id, true)?.ok_or_else(|| {
                    state_error(
                        "postgres_state.hosted_execution_attempt.missing",
                        "hosted execution attempt is missing",
                    )
                })?;
            if prior.revision() != expected_attempt_revision {
                return Err(state_error(
                    "postgres_state.revision.stale",
                    "PostgreSQL record revision is stale",
                ));
            }
            let updated_at = database_timestamp_tx(tx)?;
            let next = match target {
                HostedExecutionAttemptStatus::Invoking => {
                    prior.value().mark_invoking(updated_at)?
                }
                HostedExecutionAttemptStatus::ReconciliationRequired => {
                    prior.value().require_reconciliation(updated_at)?
                }
                HostedExecutionAttemptStatus::Prepared | HostedExecutionAttemptStatus::Terminal => {
                    return Err(state_error(
                        "postgres_state.hosted_execution_attempt.transition.invalid",
                        "hosted execution attempt transition is invalid",
                    ));
                }
            };
            let revision = update_hosted_execution_attempt_tx(
                tx,
                prior.value(),
                &next,
                expected_attempt_revision,
                true,
            )?;
            Ok(PostgresRevisionedRecord {
                value: next,
                revision,
            })
        })
    }

    /// Returns bounded queue and invocation-attempt status metrics.
    ///
    /// The snapshot uses database time and fixed fields only. It contains no
    /// work-item, invocation, workflow, tenant, or actor identifiers.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking storage error when the query fails.
    pub fn hosted_queue_metrics_snapshot(
        &self,
    ) -> Result<PostgresHostedQueueMetricsSnapshot, WorkflowOsError> {
        let mut client = self.connections.connect()?;
        let row = client
            .query_one(
                "WITH observed AS (SELECT clock_timestamp() AS at)
                 SELECT
                   count(*) FILTER (WHERE family = 'hosted_work_item' AND key2 = 'queued'),
                   count(*) FILTER (WHERE family = 'hosted_work_item' AND key2 = 'running'),
                   count(*) FILTER (WHERE family = 'hosted_work_item' AND key2 = 'waiting_for_approval'),
                   count(*) FILTER (WHERE family = 'hosted_work_item' AND key2 = 'completed'),
                   count(*) FILTER (WHERE family = 'hosted_work_item' AND key2 = 'failed'),
                   count(*) FILTER (WHERE family = 'hosted_work_item' AND key2 = 'canceled'),
                   count(*) FILTER (WHERE family = 'hosted_work_item' AND key2 = 'ambiguous'),
                   count(*) FILTER (WHERE family = 'hosted_execution_attempt' AND key2 = 'prepared'),
                   count(*) FILTER (WHERE family = 'hosted_execution_attempt' AND key2 = 'invoking'),
                   count(*) FILTER (WHERE family = 'hosted_execution_attempt' AND key2 = 'reconciliation_required'),
                   count(*) FILTER (WHERE family = 'hosted_execution_attempt' AND key2 = 'terminal'),
                   (extract(epoch FROM (
                       observed.at
                       - min(created_at) FILTER (
                           WHERE family = 'hosted_work_item' AND key2 = 'queued'
                         )
                     )) * 1000)::bigint,
                   (extract(epoch FROM observed.at) * 1000)::bigint
                 FROM observed
                 LEFT JOIN workflow_os.records
                   ON family IN ('hosted_work_item', 'hosted_execution_attempt')
                 GROUP BY observed.at",
                &[],
            )
            .map_err(|error| database_error("hosted_queue_metrics", &error))?;
        Ok(PostgresHostedQueueMetricsSnapshot {
            queued_work_items: count_from_i64(row.get(0))?,
            running_work_items: count_from_i64(row.get(1))?,
            waiting_work_items: count_from_i64(row.get(2))?,
            completed_work_items: count_from_i64(row.get(3))?,
            failed_work_items: count_from_i64(row.get(4))?,
            canceled_work_items: count_from_i64(row.get(5))?,
            ambiguous_work_items: count_from_i64(row.get(6))?,
            prepared_attempts: count_from_i64(row.get(7))?,
            invoking_attempts: count_from_i64(row.get(8))?,
            reconciliation_required_attempts: count_from_i64(row.get(9))?,
            terminal_attempts: count_from_i64(row.get(10))?,
            oldest_queued_age_ms: optional_count_from_i64(row.get::<_, Option<i64>>(11))?,
            observed_at_epoch_ms: row.get(12),
        })
    }

    /// Claims the next queued hosted work item and its database-time lease in
    /// one serializable transaction.
    ///
    /// # Errors
    ///
    /// Returns stable non-leaking state errors for invalid queue data, lease
    /// contention, or unavailable durable state. An empty queue returns
    /// `Ok(None)`.
    pub fn claim_next_hosted_work_item(
        &self,
        request: PostgresClaimHostedWorkItemRequest<'_>,
    ) -> Result<Option<PostgresClaimedHostedWorkItem>, WorkflowOsError> {
        validate_ttl(request.lease_ttl)?;
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| {
            let row = tx
                .query_opt(
                    "SELECT r.key1, r.key2, r.payload, r.revision
                       FROM workflow_os.records AS r
                       LEFT JOIN workflow_os.worker_leases AS l
                         ON l.lease_key = ('hosted-work-item/' || r.key1)
                      WHERE r.family = 'hosted_work_item'
                        AND (
                          r.key2 = 'queued'
                          OR (
                            r.key2 = 'running'
                            AND (l.lease_key IS NULL OR l.expires_at <= clock_timestamp())
                          )
                        )
                      ORDER BY r.created_at, r.key1
                      FOR UPDATE OF r SKIP LOCKED
                      LIMIT 1",
                    &[],
                )
                .map_err(|error| database_error("hosted_work_item_claim", &error))?;
            let Some(row) = row else {
                return Ok(None);
            };
            let key1: String = row.get(0);
            let key2: String = row.get(1);
            let prior: HostedWorkItem = decode(row.get::<_, String>(2).as_str())?;
            let prior_revision = revision_from_i64(row.get(3))?;
            validate_hosted_work_item_storage_identity(&prior, &key1, &key2)?;
            let claimed_at = database_timestamp_tx(tx)?;
            let next = if prior.status() == HostedWorkItemStatus::Queued {
                prior.transition(HostedWorkItemStatus::Running, claimed_at)?
            } else {
                prior.reclaim(claimed_at)?
            };
            let lease_key = PostgresLeaseKey::new(format!(
                "hosted-work-item/{}",
                prior.work_item_id().as_str()
            ))?;
            let lease = acquire_fenced_lease_tx(
                tx,
                PostgresLeaseAcquireRequest {
                    key: &lease_key,
                    owner: request.worker,
                    ttl: request.lease_ttl,
                },
            )?;
            let revision = update_hosted_work_item_tx(tx, &prior, &next, prior_revision, false)?;
            Ok(Some(PostgresClaimedHostedWorkItem {
                work_item: PostgresRevisionedRecord {
                    value: next,
                    revision,
                },
                lease,
            }))
        })
    }

    /// Commits one exact hosted work-item transition under expected revision
    /// and, when leaving `running`, an active worker fence.
    ///
    /// # Errors
    ///
    /// Rejects stale revisions, stale fences, identity changes, and illegal
    /// lifecycle transitions without modifying durable state.
    pub fn transition_hosted_work_item(
        &self,
        request: PostgresTransitionHostedWorkItemRequest<'_>,
    ) -> Result<PostgresRevisionedRecord<HostedWorkItem>, WorkflowOsError> {
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| {
            let prior = read_hosted_work_item_tx(tx, request.work_item.work_item_id(), true)?
                .ok_or_else(|| {
                    state_error(
                        "postgres_state.hosted_work_item.missing",
                        "hosted work item is missing",
                    )
                })?;
            if prior.revision() != request.expected_revision {
                return Err(state_error(
                    "postgres_state.revision.stale",
                    "PostgreSQL record revision is stale",
                ));
            }
            if prior.value().status() == HostedWorkItemStatus::Running {
                let lease = request.lease.ok_or_else(|| {
                    state_error(
                        "postgres_state.hosted_work_item.fence_required",
                        "hosted running transition requires an active worker fence",
                    )
                })?;
                validate_hosted_work_item_lease(lease, prior.value().work_item_id())?;
                validate_fence_tx(tx, lease)?;
            } else if request.lease.is_some() {
                return Err(state_error(
                    "postgres_state.hosted_work_item.fence_unexpected",
                    "hosted non-running transition cannot use a worker fence",
                ));
            }
            let expected = prior
                .value()
                .transition(request.work_item.status(), request.work_item.updated_at())?;
            if expected != *request.work_item {
                return Err(state_error(
                    "postgres_state.hosted_work_item.identity_mismatch",
                    "hosted work item transition changed immutable identity",
                ));
            }
            let release_lease = prior.value().status() == HostedWorkItemStatus::Running
                && request.work_item.status() != HostedWorkItemStatus::Running;
            let revision = update_hosted_work_item_tx(
                tx,
                prior.value(),
                request.work_item,
                request.expected_revision,
                true,
            )?;
            if release_lease {
                release_fenced_lease_tx(
                    tx,
                    request.lease.ok_or_else(|| {
                        state_error(
                            "postgres_state.hosted_work_item.fence_required",
                            "hosted running transition requires an active worker fence",
                        )
                    })?,
                )?;
            }
            Ok(PostgresRevisionedRecord {
                value: request.work_item.clone(),
                revision,
            })
        })
    }

    /// Atomically persists a no-write provider receipt, terminal work-item
    /// transition, and lease release.
    ///
    /// # Errors
    ///
    /// Fails closed on stale fences, mismatched immutable identity, invalid
    /// receipt binding, or conflicting receipt identity. This foundation API
    /// deliberately does not append workflow events or mutate run snapshots.
    pub fn commit_hosted_receipt(
        &self,
        request: PostgresCommitHostedReceiptRequest<'_>,
    ) -> Result<PostgresHostedReceiptCommitResult, WorkflowOsError> {
        validate_hosted_receipt_input(&request)?;
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| {
            let prior = read_hosted_work_item_tx(tx, request.work_item.work_item_id(), true)?
                .ok_or_else(|| {
                    state_error(
                        "postgres_state.hosted_work_item.missing",
                        "hosted work item is missing",
                    )
                })?;
            if prior.revision() != request.expected_work_item_revision {
                return Err(state_error(
                    "postgres_state.revision.stale",
                    "PostgreSQL record revision is stale",
                ));
            }
            if prior.value().status() != HostedWorkItemStatus::Running {
                return Err(state_error(
                    "postgres_state.hosted_execution.prior_status.invalid",
                    "hosted execution result requires a running work item",
                ));
            }
            validate_hosted_work_item_lease(request.lease, prior.value().work_item_id())?;
            validate_fence_tx(tx, request.lease)?;
            let expected = prior
                .value()
                .transition(request.work_item.status(), request.work_item.updated_at())?;
            if expected != *request.work_item {
                return Err(state_error(
                    "postgres_state.hosted_work_item.identity_mismatch",
                    "hosted work item transition changed immutable identity",
                ));
            }

            let work_item_revision = update_hosted_work_item_tx(
                tx,
                prior.value(),
                request.work_item,
                request.expected_work_item_revision,
                true,
            )?;
            put_record(
                tx,
                "hosted_execution_receipt",
                request.work_item.work_item_id().as_str(),
                request.receipt.execution_id().as_str(),
                request.receipt,
                None,
                true,
            )?;
            release_fenced_lease_tx(tx, request.lease)?;
            Ok(PostgresHostedReceiptCommitResult {
                work_item_revision,
                attempt_revision: None,
            })
        })
    }

    /// Atomically commits an exactly bound receipt, terminal attempt, terminal
    /// work item, and worker-lease release.
    ///
    /// # Errors
    ///
    /// Fails closed on stale revisions or fences, receipt substitution,
    /// invalid attempt posture, conflicting receipt identity, or storage
    /// failure. No partial terminal state is committed.
    pub fn commit_hosted_receipt_with_attempt(
        &self,
        request: PostgresCommitHostedReceiptRequest<'_>,
        expected_attempt_revision: DurableRevision,
    ) -> Result<PostgresHostedReceiptCommitResult, WorkflowOsError> {
        validate_hosted_receipt_input(&request)?;
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| {
            let prior_work_item =
                read_hosted_work_item_tx(tx, request.work_item.work_item_id(), true)?.ok_or_else(
                    || {
                        state_error(
                            "postgres_state.hosted_work_item.missing",
                            "hosted work item is missing",
                        )
                    },
                )?;
            if prior_work_item.revision() != request.expected_work_item_revision {
                return Err(state_error(
                    "postgres_state.revision.stale",
                    "PostgreSQL record revision is stale",
                ));
            }
            if prior_work_item.value().status() != HostedWorkItemStatus::Running {
                return Err(state_error(
                    "postgres_state.hosted_execution.prior_status.invalid",
                    "hosted execution result requires a running work item",
                ));
            }
            validate_hosted_work_item_lease(request.lease, prior_work_item.value().work_item_id())?;
            validate_fence_tx(tx, request.lease)?;
            let prior_attempt =
                read_hosted_execution_attempt_tx(tx, prior_work_item.value().work_item_id(), true)?
                    .ok_or_else(|| {
                        state_error(
                            "postgres_state.hosted_execution_attempt.missing",
                            "hosted execution attempt is missing",
                        )
                    })?;
            if prior_attempt.revision() != expected_attempt_revision {
                return Err(state_error(
                    "postgres_state.revision.stale",
                    "PostgreSQL record revision is stale",
                ));
            }
            let expected_work_item = prior_work_item
                .value()
                .transition(request.work_item.status(), request.work_item.updated_at())?;
            if expected_work_item != *request.work_item {
                return Err(state_error(
                    "postgres_state.hosted_work_item.identity_mismatch",
                    "hosted work item transition changed immutable identity",
                ));
            }
            let terminal_attempt = prior_attempt.value().mark_terminal(request.receipt)?;

            let work_item_revision = update_hosted_work_item_tx(
                tx,
                prior_work_item.value(),
                request.work_item,
                request.expected_work_item_revision,
                true,
            )?;
            let attempt_revision = update_hosted_execution_attempt_tx(
                tx,
                prior_attempt.value(),
                &terminal_attempt,
                expected_attempt_revision,
                true,
            )?;
            put_record(
                tx,
                "hosted_execution_receipt",
                request.work_item.work_item_id().as_str(),
                request.receipt.execution_id().as_str(),
                request.receipt,
                None,
                true,
            )?;
            release_fenced_lease_tx(tx, request.lease)?;
            Ok(PostgresHostedReceiptCommitResult {
                work_item_revision,
                attempt_revision: Some(attempt_revision),
            })
        })
    }

    /// Atomically commits the exactly bound hosted receipt and attempt together
    /// with authoritative terminal workflow events and snapshot projection.
    ///
    /// # Errors
    ///
    /// Fails closed on stale run/work-item/attempt state, a substituted
    /// receipt or event, cancellation races, or any partial durable write.
    pub fn commit_hosted_receipt_and_projection(
        &self,
        request: PostgresCommitHostedReceiptProjectionRequest<'_>,
    ) -> Result<PostgresHostedReceiptCommitResult, WorkflowOsError> {
        let receipt_request = request.receipt_commit;
        validate_hosted_receipt_input(&receipt_request)?;
        if request.projection.receipt() != receipt_request.receipt
            || request.projection.projected_run().snapshot.identity.run_id
                != *receipt_request.work_item.run_id()
            || request.report_artifact.record().run_id() != receipt_request.work_item.run_id()
            || request
                .report_artifact
                .record()
                .metadata()
                .terminal_run_status()
                != crate::WorkReportStatus::Completed
            || request.report_artifact.record().metadata().generated_at()
                != receipt_request.receipt.terminal_at()
        {
            return Err(state_error(
                "postgres_state.hosted_projection.binding.invalid",
                "hosted terminal projection binding is invalid",
            ));
        }
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| {
            if let Some(replayed) = replay_hosted_receipt_projection_tx(tx, &request)? {
                return Ok(replayed);
            }
            commit_fresh_hosted_receipt_projection_tx(tx, &request)
        })
    }

    /// Atomically commits a pre-provider rejection or an ambiguous provider
    /// outcome together with its authoritative workflow projection.
    ///
    /// # Errors
    ///
    /// Fails closed on stale run, work-item, attempt, or fence state; invalid
    /// outcome posture; conflicting replay; or any partial durable write.
    pub fn commit_hosted_unreceipted_projection(
        &self,
        request: PostgresCommitHostedUnreceiptedProjectionRequest<'_>,
    ) -> Result<PostgresHostedReceiptCommitResult, WorkflowOsError> {
        validate_hosted_unreceipted_projection_input(&request)?;
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| {
            if let Some(replayed) = replay_hosted_unreceipted_projection_tx(tx, &request)? {
                return Ok(replayed);
            }
            commit_fresh_hosted_unreceipted_projection_tx(tx, &request)
        })
    }

    /// Reads one durable hosted execution receipt by work item and execution.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when storage is unavailable or the
    /// stored payload does not match its key.
    pub fn read_hosted_execution_receipt(
        &self,
        work_item_id: &HostedWorkItemId,
        execution_id: &crate::HostedExecutionId,
    ) -> Result<Option<HostedExecutionReceipt>, WorkflowOsError> {
        let mut client = self.connections.connect()?;
        let row = client
            .query_opt(
                "SELECT payload FROM workflow_os.records
                  WHERE family = 'hosted_execution_receipt' AND key1 = $1 AND key2 = $2",
                &[&work_item_id.as_str(), &execution_id.as_str()],
            )
            .map_err(|error| database_error("hosted_execution_receipt_read", &error))?;
        let Some(row) = row else {
            return Ok(None);
        };
        let receipt: HostedExecutionReceipt = decode(row.get::<_, String>(0).as_str())?;
        if receipt.execution_id() != execution_id {
            return Err(state_error(
                "postgres_state.hosted_execution_receipt.identity_mismatch",
                "hosted execution receipt storage identity is invalid",
            ));
        }
        Ok(Some(receipt))
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
        validate_side_effect_intent_idempotency_binding(side_effect, event, idempotency_key)?;
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| {
            let key = idempotency_key.as_str();
            let payload = encode(&idempotency_result)?;
            let intent_ref = format!(
                "{}/{}",
                side_effect.side_effect_id().as_str(),
                event.event_id.as_str()
            );
            let reserved = tx
                .query_opt(
                    "INSERT INTO workflow_os.idempotency (key, payload, intent_ref)
                     VALUES ($1, $2, $3)
                     ON CONFLICT (key) DO NOTHING
                     RETURNING key",
                    &[&key, &payload, &intent_ref],
                )
                .map_err(|error| database_error("idempotency_reserve", &error))?
                .is_some();
            if !reserved {
                let row = tx
                    .query_one(
                        "SELECT payload, intent_ref
                           FROM workflow_os.idempotency WHERE key = $1",
                        &[&key],
                    )
                    .map_err(|error| database_error("idempotency_read", &error))?;
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

impl crate::immutable_run_bundle_store::ImmutableRunBundleStore for PostgresStateBackend {
    fn publish_bundle_create_only(
        &self,
        bundle: &ImmutableRunBundleBuildResult,
    ) -> Result<crate::immutable_run_bundle_store::ImmutableRunBundlePublishOutcome, WorkflowOsError>
    {
        match self.publish_immutable_run_bundle(bundle) {
            Ok(()) => {
                Ok(crate::immutable_run_bundle_store::ImmutableRunBundlePublishOutcome::Published)
            }
            Err(error) if error.code() == "postgres_state.bundle.manifest_exists" => Ok(
                crate::immutable_run_bundle_store::ImmutableRunBundlePublishOutcome::AlreadyExists,
            ),
            Err(error) => Err(error),
        }
    }

    fn read_exact_bundle(
        &self,
        run_id: &WorkflowRunId,
        bundle_id: &ImmutableRunBundleId,
    ) -> Result<StoredImmutableRunBundle, WorkflowOsError> {
        let stored = self.read_immutable_run_bundle(run_id)?.ok_or_else(|| {
            state_error(
                "postgres_state.bundle.missing",
                "immutable run bundle is missing",
            )
        })?;
        if stored.manifest().bundle_id() != bundle_id {
            return Err(state_error(
                "postgres_state.bundle.identity_mismatch",
                "immutable run bundle storage identity does not match payload",
            ));
        }
        Ok(stored)
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

impl HostedProjectResourceBindingReader for PostgresStateBackend {
    fn read_project_resource_binding(
        &self,
        kind: HostedProjectResourceKind,
        resource_id: &str,
    ) -> Result<Option<HostedProjectResourceBinding>, WorkflowOsError> {
        self.read_hosted_project_resource_binding(kind, resource_id)
            .map(|record| record.map(|record| record.into_parts().0))
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

impl ProjectApprovalRouteStore for PostgresStateBackend {
    fn create_project_approval_route(
        &self,
        record: ProjectApprovalRouteRecord,
    ) -> Result<ProjectApprovalRouteCreateResult, WorkflowOsError> {
        let encoded = encode_project_approval_route(&record)?;
        let candidate_authority = authority_identity_from_route_record(&record)?;
        let mut client = self.connections.connect()?;
        serializable(&mut client, |tx| {
            let high_watermark =
                read_authority_high_watermark_tx(tx, record.route().scope().organization_id())?;
            require_current_route_authority(&candidate_authority, &high_watermark)?;
            require_pending_project_approval_history_tx(tx, &record)?;
            require_active_project_approval_run_binding_tx(tx, &record)?;
            if let Some((existing, stored_revision)) =
                read_project_approval_route_tx(tx, record.logical_subject_id(), true)?
            {
                if stored_revision == candidate_authority.revision
                    && existing.is_decision_equivalent(&record)
                {
                    return Ok(ProjectApprovalRouteCreateResult::ReconciledExisting(
                        existing,
                    ));
                }
                return Err(project_approval_route_conflict_error());
            }

            let inserted = insert_project_approval_route_tx(tx, &record, &encoded)?;
            if inserted {
                return Ok(ProjectApprovalRouteCreateResult::Created(record.clone()));
            }
            match read_project_approval_route_tx(tx, record.logical_subject_id(), true)? {
                Some((existing, stored_revision))
                    if stored_revision == candidate_authority.revision
                        && existing.is_decision_equivalent(&record) =>
                {
                    Ok(ProjectApprovalRouteCreateResult::ReconciledExisting(
                        existing,
                    ))
                }
                Some(_) => Err(project_approval_route_conflict_error()),
                None => Err(state_error(
                    "postgres_state.transaction.retryable",
                    "PostgreSQL transaction encountered a retryable conflict",
                )),
            }
        })
    }

    fn read_project_approval_route(
        &self,
        logical_subject_id: &ProjectApprovalRouteLogicalSubjectId,
    ) -> Result<Option<ProjectApprovalRouteRecord>, WorkflowOsError> {
        let mut client = self.connections.connect()?;
        let row = client
            .query_opt(
                PROJECT_APPROVAL_ROUTE_SELECT_BY_SUBJECT,
                &[&logical_subject_id.as_str()],
            )
            .map_err(|error| database_error("project_approval_route_read", &error))?;
        row.map(|row| decode_project_approval_route_row(&row).map(|(record, _)| record))
            .transpose()
    }

    fn list_project_approval_routes_for_recipient(
        &self,
        scope: &HostedProjectScope,
        recipient: &ActorId,
        limit: usize,
    ) -> Result<Vec<ProjectApprovalRouteRecord>, WorkflowOsError> {
        validate_project_approval_route_list_limit(limit)?;
        scope.validate()?;
        let limit = i64::try_from(limit).map_err(|_| project_approval_route_limit_error())?;
        let mut client = self.connections.connect()?;
        let rows = client
            .query(
                PROJECT_APPROVAL_ROUTE_SELECT_RECIPIENT,
                &[
                    &scope.organization_id().as_str(),
                    &scope.project_id().as_str(),
                    &recipient.as_str(),
                    &limit,
                ],
            )
            .map_err(|error| database_error("project_approval_route_recipient_list", &error))?;
        decode_project_approval_route_rows(rows)
    }

    fn list_project_approval_routes_for_approval(
        &self,
        scope: &HostedProjectScope,
        run_id: &WorkflowRunId,
        approval_id: &ApprovalReferenceId,
        limit: usize,
    ) -> Result<Vec<ProjectApprovalRouteRecord>, WorkflowOsError> {
        validate_project_approval_route_list_limit(limit)?;
        scope.validate()?;
        let limit = i64::try_from(limit).map_err(|_| project_approval_route_limit_error())?;
        let mut client = self.connections.connect()?;
        let rows = client
            .query(
                PROJECT_APPROVAL_ROUTE_SELECT_APPROVAL,
                &[
                    &scope.organization_id().as_str(),
                    &scope.project_id().as_str(),
                    &run_id.as_str(),
                    &approval_id.as_str(),
                    &limit,
                ],
            )
            .map_err(|error| database_error("project_approval_route_approval_list", &error))?;
        decode_project_approval_route_rows(rows)
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

const PROJECT_APPROVAL_ROUTE_SELECT_BY_SUBJECT: &str = "SELECT logical_subject_id, route_id,
    organization_id, project_id, run_id, approval_id, routing_reason, escalation_id, route_status,
    recipient_actor_id, notification_posture, authority_revision, source_algorithm,
    source_route_id, source_fingerprint, record_version, resolved_at, record_created_at,
    canonical_payload, canonical_payload_hash
    FROM workflow_os.project_approval_routes WHERE logical_subject_id = $1";

const PROJECT_APPROVAL_ROUTE_SELECT_RECIPIENT: &str = "SELECT logical_subject_id, route_id,
    organization_id, project_id, run_id, approval_id, routing_reason, escalation_id, route_status,
    recipient_actor_id, notification_posture, authority_revision, source_algorithm,
    source_route_id, source_fingerprint, record_version, resolved_at, record_created_at,
    canonical_payload, canonical_payload_hash
    FROM workflow_os.project_approval_routes
    WHERE organization_id = $1 AND project_id = $2 AND recipient_actor_id = $3
      AND route_status = 'routed'
    ORDER BY logical_subject_id LIMIT $4";

const PROJECT_APPROVAL_ROUTE_SELECT_APPROVAL: &str = "SELECT logical_subject_id, route_id,
    organization_id, project_id, run_id, approval_id, routing_reason, escalation_id, route_status,
    recipient_actor_id, notification_posture, authority_revision, source_algorithm,
    source_route_id, source_fingerprint, record_version, resolved_at, record_created_at,
    canonical_payload, canonical_payload_hash
    FROM workflow_os.project_approval_routes
    WHERE organization_id = $1 AND project_id = $2 AND run_id = $3 AND approval_id = $4
    ORDER BY logical_subject_id LIMIT $5";

struct EncodedProjectApprovalRoute {
    payload: String,
    payload_hash: String,
}

fn encode_project_approval_route(
    record: &ProjectApprovalRouteRecord,
) -> Result<EncodedProjectApprovalRoute, WorkflowOsError> {
    let payload = encode(record)?;
    let decoded: ProjectApprovalRouteRecord = decode(&payload)?;
    if &decoded != record {
        return Err(project_approval_route_corruption_error());
    }
    let payload_hash = project_approval_route_payload_hash(&payload);
    Ok(EncodedProjectApprovalRoute {
        payload,
        payload_hash,
    })
}

fn insert_project_approval_route_tx(
    tx: &mut Transaction<'_>,
    record: &ProjectApprovalRouteRecord,
    encoded: &EncodedProjectApprovalRoute,
) -> Result<bool, WorkflowOsError> {
    let route = record.route();
    let source = record.source_commitment();
    let authority_revision = route_authority_revision(source)?;
    let inserted = tx
        .execute(
            "INSERT INTO workflow_os.project_approval_routes
               (logical_subject_id, route_id, organization_id, project_id, run_id, approval_id,
                routing_reason, escalation_id, route_status, recipient_actor_id,
                notification_posture, authority_revision, source_algorithm, source_route_id,
                source_fingerprint, record_version, resolved_at, record_created_at,
                canonical_payload, canonical_payload_hash)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
                     $11, $12, $13, $14, $15, $16, $17, $18, $19, $20)
             ON CONFLICT DO NOTHING",
            &[
                &record.logical_subject_id().as_str(),
                &route.route_id().as_str(),
                &route.scope().organization_id().as_str(),
                &route.scope().project_id().as_str(),
                &route.run_id().as_str(),
                &route.approval_id(),
                &project_approval_routing_reason_label(route.routing_reason()),
                &route.escalation_id(),
                &project_approval_route_status_label(route.status()),
                &route.recipient().map(ActorId::as_str),
                &project_approval_notification_posture_label(route.notification_posture()),
                &authority_revision,
                &project_approval_source_algorithm_label(source.algorithm()),
                &source.route_id().as_str(),
                &source.fingerprint().as_str(),
                &project_approval_record_version_label(record.record_version()),
                &route.resolved_at().to_rfc3339(),
                &record.created_at().to_rfc3339(),
                &encoded.payload,
                &encoded.payload_hash,
            ],
        )
        .map_err(|error| database_error("project_approval_route_insert", &error))?;
    Ok(inserted == 1)
}

fn read_project_approval_route_tx(
    tx: &mut Transaction<'_>,
    logical_subject_id: &ProjectApprovalRouteLogicalSubjectId,
    for_update: bool,
) -> Result<Option<(ProjectApprovalRouteRecord, i64)>, WorkflowOsError> {
    let query = if for_update {
        "SELECT logical_subject_id, route_id, organization_id, project_id, run_id, approval_id,
                routing_reason, escalation_id, route_status, recipient_actor_id,
                notification_posture, authority_revision, source_algorithm, source_route_id,
                source_fingerprint, record_version, resolved_at, record_created_at,
                canonical_payload, canonical_payload_hash
           FROM workflow_os.project_approval_routes
          WHERE logical_subject_id = $1 FOR UPDATE"
    } else {
        PROJECT_APPROVAL_ROUTE_SELECT_BY_SUBJECT
    };
    tx.query_opt(query, &[&logical_subject_id.as_str()])
        .map_err(|error| database_error("project_approval_route_read", &error))?
        .map(|row| decode_project_approval_route_row(&row))
        .transpose()
}

struct ProjectApprovalAuthorityIdentity {
    revision: i64,
    algorithm: &'static str,
    fingerprint: String,
}

struct StoredProjectApprovalAuthorityHighWatermark {
    revision: i64,
    algorithm: String,
    fingerprint: String,
}

fn authority_identity_from_route_record(
    record: &ProjectApprovalRouteRecord,
) -> Result<ProjectApprovalAuthorityIdentity, WorkflowOsError> {
    let source = record.source_commitment();
    let authority_view = source.authority_snapshot().authority_view();
    if authority_view.organization_id() != record.route().scope().organization_id() {
        return Err(project_approval_route_authority_mismatch_error());
    }
    Ok(ProjectApprovalAuthorityIdentity {
        revision: route_authority_revision(source)?,
        algorithm: authority_commitment_algorithm_label(authority_view.algorithm()),
        fingerprint: authority_view.fingerprint().as_str().to_owned(),
    })
}

fn route_authority_revision(
    source: &crate::ProjectApprovalRouteSourceCommitment,
) -> Result<i64, WorkflowOsError> {
    i64::try_from(source.authority_registry_revision().get()).map_err(|_| {
        state_error(
            "postgres_state.project_approval_authority.revision.invalid",
            "project approval authority revision is invalid",
        )
    })
}

fn read_authority_high_watermark_tx(
    tx: &mut Transaction<'_>,
    organization_id: &OrganizationId,
) -> Result<StoredProjectApprovalAuthorityHighWatermark, WorkflowOsError> {
    tx.query_opt(
        "SELECT authority_revision, commitment_algorithm, commitment_fingerprint
           FROM workflow_os.hosted_authority_registry_high_watermarks
          WHERE organization_id = $1 FOR SHARE",
        &[&organization_id.as_str()],
    )
    .map_err(|error| database_error("authority_high_watermark_read", &error))?
    .map(|row| StoredProjectApprovalAuthorityHighWatermark {
        revision: row.get(0),
        algorithm: row.get(1),
        fingerprint: row.get(2),
    })
    .ok_or_else(|| {
        state_error(
            "postgres_state.project_approval_authority.uninitialized",
            "project approval authority high watermark is not initialized",
        )
    })
}

fn require_current_route_authority(
    candidate: &ProjectApprovalAuthorityIdentity,
    current: &StoredProjectApprovalAuthorityHighWatermark,
) -> Result<(), WorkflowOsError> {
    validate_authority_high_watermark_row(
        current.revision,
        &current.algorithm,
        &current.fingerprint,
    )?;
    if candidate.revision != current.revision
        || candidate.algorithm != current.algorithm
        || candidate.fingerprint != current.fingerprint
    {
        return Err(project_approval_route_authority_mismatch_error());
    }
    Ok(())
}

fn require_pending_project_approval_history_tx(
    tx: &mut Transaction<'_>,
    record: &ProjectApprovalRouteRecord,
) -> Result<(), WorkflowOsError> {
    let route = record.route();
    let events = read_events_tx(tx, route.run_id())?;
    let mut requested = None;
    let mut decided = false;
    for event in &events {
        match &event.kind {
            crate::WorkflowRunEventKind::ApprovalRequested(candidate)
                if candidate.approval_id == route.approval_id()
                    && requested.replace(candidate.as_ref()).is_some() =>
            {
                return Err(project_approval_route_approval_history_error());
            }
            crate::WorkflowRunEventKind::ApprovalGranted(decision)
            | crate::WorkflowRunEventKind::ApprovalDenied(decision)
                if decision.approval_id == route.approval_id() =>
            {
                decided = true;
            }
            _ => {}
        }
    }
    let Some(approval) = requested else {
        return Err(project_approval_route_approval_history_error());
    };
    if decided
        || approval.decision.is_some()
        || approval.run_id != *route.run_id()
        || approval.approval_id != route.approval_id()
    {
        return Err(project_approval_route_approval_history_error());
    }
    Ok(())
}

fn require_active_project_approval_run_binding_tx(
    tx: &mut Transaction<'_>,
    record: &ProjectApprovalRouteRecord,
) -> Result<(), WorkflowOsError> {
    let route = record.route();
    let binding = read_project_resource_binding_tx(
        tx,
        HostedProjectResourceKind::Run,
        route.run_id().as_str(),
        true,
    )?
    .ok_or_else(project_approval_route_run_binding_error)?;
    if binding.value().scope() != route.scope()
        || binding.value().resource_kind() != HostedProjectResourceKind::Run
        || binding.value().resource_id() != route.run_id().as_str()
        || binding.value().status() != HostedProjectResourceBindingStatus::Active
    {
        return Err(project_approval_route_run_binding_error());
    }
    Ok(())
}

fn validate_authority_high_watermark_row(
    revision: i64,
    algorithm: &str,
    fingerprint: &str,
) -> Result<(), WorkflowOsError> {
    if revision <= 0
        || algorithm
            != authority_commitment_algorithm_label(
                ProjectApprovalAuthorityViewCommitmentAlgorithm::V1,
            )
        || SpecContentHash::new(fingerprint).is_err()
    {
        return Err(project_approval_authority_corruption_error());
    }
    Ok(())
}

fn decode_project_approval_route_rows(
    rows: Vec<Row>,
) -> Result<Vec<ProjectApprovalRouteRecord>, WorkflowOsError> {
    rows.into_iter()
        .map(|row| decode_project_approval_route_row(&row).map(|(record, _)| record))
        .collect()
}

fn decode_project_approval_route_row(
    row: &Row,
) -> Result<(ProjectApprovalRouteRecord, i64), WorkflowOsError> {
    let logical_subject_id: String = row.get(0);
    let route_id: String = row.get(1);
    let organization_id: String = row.get(2);
    let project_id: String = row.get(3);
    let run_id: String = row.get(4);
    let approval_id: String = row.get(5);
    let routing_reason: String = row.get(6);
    let escalation_id: Option<String> = row.get(7);
    let route_status: String = row.get(8);
    let recipient_actor_id: Option<String> = row.get(9);
    let notification_posture: String = row.get(10);
    let authority_revision: i64 = row.get(11);
    let source_algorithm: String = row.get(12);
    let source_route_id: String = row.get(13);
    let source_fingerprint: String = row.get(14);
    let record_version: String = row.get(15);
    let resolved_at: String = row.get(16);
    let record_created_at: String = row.get(17);
    let canonical_payload: String = row.get(18);
    let canonical_payload_hash: String = row.get(19);

    let record: ProjectApprovalRouteRecord =
        decode(&canonical_payload).map_err(|_| project_approval_route_corruption_error())?;
    let reencoded = encode(&record).map_err(|_| project_approval_route_corruption_error())?;
    let route = record.route();
    let source = record.source_commitment();
    let valid = canonical_payload == reencoded
        && canonical_payload_hash == project_approval_route_payload_hash(&canonical_payload)
        && logical_subject_id == record.logical_subject_id().as_str()
        && route_id == route.route_id().as_str()
        && organization_id == route.scope().organization_id().as_str()
        && project_id == route.scope().project_id().as_str()
        && run_id == route.run_id().as_str()
        && approval_id == route.approval_id()
        && routing_reason == project_approval_routing_reason_label(route.routing_reason())
        && escalation_id.as_deref() == route.escalation_id()
        && route_status == project_approval_route_status_label(route.status())
        && recipient_actor_id.as_deref() == route.recipient().map(ActorId::as_str)
        && notification_posture
            == project_approval_notification_posture_label(route.notification_posture())
        && authority_revision == route_authority_revision(source)?
        && source_algorithm == project_approval_source_algorithm_label(source.algorithm())
        && source_route_id == source.route_id().as_str()
        && source_fingerprint == source.fingerprint().as_str()
        && record_version == project_approval_record_version_label(record.record_version())
        && resolved_at == route.resolved_at().to_rfc3339()
        && record_created_at == record.created_at().to_rfc3339();
    if !valid {
        return Err(project_approval_route_corruption_error());
    }
    Ok((record, authority_revision))
}

fn project_approval_route_payload_hash(payload: &str) -> String {
    let mut hasher = Sha256::new();
    let domain = b"workflow-os/postgresql/project-approval-route-payload/v1";
    hasher.update((domain.len() as u64).to_be_bytes());
    hasher.update(domain);
    hasher.update((payload.len() as u64).to_be_bytes());
    hasher.update(payload.as_bytes());
    SpecContentHash::from_bytes(hasher.finalize())
        .as_str()
        .to_owned()
}

fn validate_project_approval_route_list_limit(limit: usize) -> Result<(), WorkflowOsError> {
    if limit == 0 || limit > MAX_PROJECT_APPROVAL_ROUTE_LIST_LIMIT {
        return Err(project_approval_route_limit_error());
    }
    Ok(())
}

fn project_approval_route_limit_error() -> WorkflowOsError {
    state_error(
        "project_approval_route_store.list.limit.invalid",
        "project approval route list limit is invalid",
    )
}

fn project_approval_route_conflict_error() -> WorkflowOsError {
    state_error(
        "project_approval_route_store.create.conflict",
        "project approval route record conflicts with existing state",
    )
}

fn project_approval_route_corruption_error() -> WorkflowOsError {
    state_error(
        "postgres_state.project_approval_route.corrupt",
        "stored PostgreSQL project approval route is invalid",
    )
}

fn project_approval_route_authority_mismatch_error() -> WorkflowOsError {
    state_error(
        "postgres_state.project_approval_route.authority_mismatch",
        "project approval route authority does not match durable current authority",
    )
}

fn project_approval_route_approval_history_error() -> WorkflowOsError {
    state_error(
        "postgres_state.project_approval_route.approval_history_invalid",
        "project approval route requires exact pending durable approval history",
    )
}

fn project_approval_route_run_binding_error() -> WorkflowOsError {
    state_error(
        "postgres_state.project_approval_route.run_binding_invalid",
        "project approval route requires an active exact durable run binding",
    )
}

fn project_approval_authority_corruption_error() -> WorkflowOsError {
    state_error(
        "postgres_state.project_approval_authority.corrupt",
        "stored PostgreSQL project approval authority is invalid",
    )
}

const fn project_approval_routing_reason_label(
    reason: ProjectApprovalRoutingReason,
) -> &'static str {
    match reason {
        ProjectApprovalRoutingReason::WorkflowMaintainer => "workflow_maintainer",
        ProjectApprovalRoutingReason::WorkflowEscalationContact => "workflow_escalation_contact",
    }
}

const fn project_approval_route_status_label(status: ProjectApprovalRouteStatus) -> &'static str {
    match status {
        ProjectApprovalRouteStatus::Routed => "routed",
        ProjectApprovalRouteStatus::UnresolvedMissingMetadata => "unresolved_missing_metadata",
        ProjectApprovalRouteStatus::UnresolvedAuthorityUnavailable => {
            "unresolved_authority_unavailable"
        }
    }
}

const fn project_approval_notification_posture_label(
    posture: ProjectApprovalNotificationPosture,
) -> &'static str {
    match posture {
        ProjectApprovalNotificationPosture::AvailableForProjectInbox => {
            "available_for_project_inbox"
        }
        ProjectApprovalNotificationPosture::UnavailableRouteUnresolved => {
            "unavailable_route_unresolved"
        }
    }
}

const fn project_approval_source_algorithm_label(
    algorithm: ProjectApprovalRouteSourceCommitmentAlgorithm,
) -> &'static str {
    match algorithm {
        ProjectApprovalRouteSourceCommitmentAlgorithm::V1 => "v1",
    }
}

const fn project_approval_record_version_label(
    version: ProjectApprovalRouteRecordVersion,
) -> &'static str {
    match version {
        ProjectApprovalRouteRecordVersion::V1 => "v1",
    }
}

const fn authority_commitment_algorithm_label(
    algorithm: ProjectApprovalAuthorityViewCommitmentAlgorithm,
) -> &'static str {
    match algorithm {
        ProjectApprovalAuthorityViewCommitmentAlgorithm::V1 => "v1",
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

fn read_record_tx<T: DeserializeOwned>(
    tx: &mut Transaction<'_>,
    family: &str,
    key1: &str,
    key2: &str,
    for_update: bool,
) -> Result<Option<PostgresRevisionedRecord<T>>, WorkflowOsError> {
    let query = if for_update {
        "SELECT payload, revision FROM workflow_os.records
          WHERE family = $1 AND key1 = $2 AND key2 = $3 FOR UPDATE"
    } else {
        "SELECT payload, revision FROM workflow_os.records
          WHERE family = $1 AND key1 = $2 AND key2 = $3"
    };
    let row = tx
        .query_opt(query, &[&family, &key1, &key2])
        .map_err(|error| database_error("record_read", &error))?;
    decode_optional_record(row)
}

fn create_project_resource_binding_tx(
    tx: &mut Transaction<'_>,
    binding: &HostedProjectResourceBinding,
) -> Result<(), WorkflowOsError> {
    match read_project_resource_binding_tx(
        tx,
        binding.resource_kind(),
        binding.resource_id(),
        true,
    )? {
        Some(existing)
            if existing.value() == binding
                || (existing.value().scope() == binding.scope()
                    && existing.value().resource_kind() == binding.resource_kind()
                    && existing.value().resource_id() == binding.resource_id()
                    && binding.status() == HostedProjectResourceBindingStatus::Reserved) =>
        {
            Ok(())
        }
        Some(_) => Err(state_error(
            "postgres_state.hosted_project_binding.scope_conflict",
            "hosted project resource scope conflicts with durable state",
        )),
        None => put_record(
            tx,
            "hosted_project_binding",
            binding.resource_id(),
            binding.resource_kind().storage_key(),
            binding,
            None,
            true,
        )
        .map(|_| ()),
    }
}

fn read_project_resource_binding_tx(
    tx: &mut Transaction<'_>,
    kind: HostedProjectResourceKind,
    resource_id: &str,
    for_update: bool,
) -> Result<Option<PostgresRevisionedRecord<HostedProjectResourceBinding>>, WorkflowOsError> {
    read_record_tx(
        tx,
        "hosted_project_binding",
        resource_id,
        kind.storage_key(),
        for_update,
    )
}

fn require_project_resource_binding_tx(
    tx: &mut Transaction<'_>,
    scope: &HostedProjectScope,
    kind: HostedProjectResourceKind,
    resource_id: &str,
) -> Result<(), WorkflowOsError> {
    let binding =
        read_project_resource_binding_tx(tx, kind, resource_id, false)?.ok_or_else(|| {
            state_error(
                "postgres_state.hosted_project_binding.missing",
                "hosted project resource binding is missing",
            )
        })?;
    if binding.value().scope() != scope
        || binding.value().status() != HostedProjectResourceBindingStatus::Active
    {
        return Err(state_error(
            "postgres_state.hosted_project_binding.scope_mismatch",
            "hosted project resource binding does not authorize access",
        ));
    }
    Ok(())
}

fn activate_reserved_project_resource_binding_tx(
    tx: &mut Transaction<'_>,
    scope: &HostedProjectScope,
    kind: HostedProjectResourceKind,
    resource_id: &str,
) -> Result<(), WorkflowOsError> {
    let existing =
        read_project_resource_binding_tx(tx, kind, resource_id, true)?.ok_or_else(|| {
            state_error(
                "postgres_state.hosted_project_binding.missing",
                "hosted project resource binding is missing",
            )
        })?;
    if existing.value().scope() != scope {
        return Err(state_error(
            "postgres_state.hosted_project_binding.scope_mismatch",
            "hosted project resource binding does not authorize access",
        ));
    }
    if existing.value().status() == HostedProjectResourceBindingStatus::Reserved {
        let active = existing.value().activate()?;
        put_record(
            tx,
            "hosted_project_binding",
            resource_id,
            kind.storage_key(),
            &active,
            Some(existing.revision()),
            false,
        )?;
    }
    Ok(())
}

fn hosted_scope_storage_prefix(scope: &HostedProjectScope) -> String {
    format!(
        "{}\u{1f}{}\u{1f}",
        scope.organization_id().as_str(),
        scope.project_id().as_str()
    )
}

fn hosted_catalog_storage_key(
    scope: &HostedProjectScope,
    workflow_id: &crate::WorkflowId,
) -> String {
    format!(
        "{}{}",
        hosted_scope_storage_prefix(scope),
        workflow_id.as_str()
    )
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

fn validate_side_effect_intent_idempotency_binding(
    record: &SideEffectRecord,
    event: &WorkflowRunEvent,
    idempotency_key: &IdempotencyKey,
) -> Result<(), WorkflowOsError> {
    if record.idempotency().key() != idempotency_key
        || event.idempotency_key.as_ref() != Some(idempotency_key)
    {
        return Err(state_error(
            "postgres_state.idempotency.intent_mismatch",
            "PostgreSQL idempotency reservation does not match its intent",
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

fn acquire_fenced_lease_tx(
    tx: &mut Transaction<'_>,
    request: PostgresLeaseAcquireRequest<'_>,
) -> Result<PostgresFencedLease, WorkflowOsError> {
    let ttl = validate_ttl(request.ttl)?;
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
             RETURNING (extract(epoch FROM expires_at) * 1000)::bigint",
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
}

fn renew_fenced_lease_tx(
    tx: &mut Transaction<'_>,
    lease: &PostgresFencedLease,
    ttl: Duration,
) -> Result<PostgresFencedLease, WorkflowOsError> {
    let ttl = validate_ttl(ttl)?;
    let fence = i64::try_from(lease.fence_token).map_err(|_| {
        state_error(
            "postgres_state.lease.fence_invalid",
            "PostgreSQL worker lease fencing token is invalid",
        )
    })?;
    let ttl_ms = ttl.as_secs_f64() * 1_000.0;
    let row = tx
        .query_opt(
            "UPDATE workflow_os.worker_leases
                SET expires_at = clock_timestamp()
                  + ($4::double precision * interval '1 millisecond')
              WHERE lease_key = $1
                AND owner = $2
                AND fence_token = $3
                AND expires_at > clock_timestamp()
            RETURNING (extract(epoch FROM expires_at) * 1000)::bigint",
            &[&lease.key.as_str(), &lease.owner.as_str(), &fence, &ttl_ms],
        )
        .map_err(|error| database_error("lease_renew", &error))?;
    let Some(row) = row else {
        return Err(state_error(
            "postgres_state.lease.stale",
            "PostgreSQL worker lease is expired or stale",
        ));
    };
    Ok(PostgresFencedLease {
        key: lease.key.clone(),
        owner: lease.owner.clone(),
        fence_token: lease.fence_token,
        expires_at_epoch_ms: row.get(0),
    })
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn prepare_hosted_execution_attempt_tx(
    tx: &mut Transaction<'_>,
    expected_work_item_revision: DurableRevision,
    work_item_id: &HostedWorkItemId,
    execution_id: &crate::HostedExecutionId,
    provider_id: &HostedExecutionProviderId,
    provider_version: &HostedExecutionProviderVersion,
    provider_configuration_hash: &SpecContentHash,
    lease: &PostgresFencedLease,
) -> Result<PostgresRevisionedRecord<HostedExecutionAttempt>, WorkflowOsError> {
    let work_item = read_hosted_work_item_tx(tx, work_item_id, true)?.ok_or_else(|| {
        state_error(
            "postgres_state.hosted_work_item.missing",
            "hosted work item is missing",
        )
    })?;
    if work_item.revision() != expected_work_item_revision {
        return Err(state_error(
            "postgres_state.revision.stale",
            "PostgreSQL record revision is stale",
        ));
    }
    if work_item.value().status() != HostedWorkItemStatus::Running {
        return Err(state_error(
            "postgres_state.hosted_execution_attempt.work_item_status.invalid",
            "hosted execution attempt requires a running work item",
        ));
    }
    validate_hosted_work_item_lease(lease, work_item_id)?;
    validate_fence_tx(tx, lease)?;

    let request_fingerprint = work_item.value().execution_request().fingerprint();
    let intent_hash = SpecContentHash::from_text(&encode(&HostedExecutionAttemptIntent {
        execution_id,
        work_item_id,
        request_fingerprint: request_fingerprint.clone(),
        provider_id,
        provider_version,
        provider_configuration_hash,
    })?);
    let intent_ref = format!("hosted-execution-attempt/{}", intent_hash.as_str());
    let attempt = HostedExecutionAttempt::prepared(
        execution_id.clone(),
        work_item_id.clone(),
        request_fingerprint,
        provider_id.clone(),
        provider_version.clone(),
        provider_configuration_hash.clone(),
        database_timestamp_tx(tx)?,
    );
    let storage_key = format!(
        "hosted/invocation/{}",
        SpecContentHash::from_text(execution_id.as_str()).as_str()
    );
    let reservation = tx
        .query_opt(
            "SELECT payload, intent_ref FROM workflow_os.idempotency
              WHERE key = $1 FOR UPDATE",
            &[&storage_key],
        )
        .map_err(|error| database_error("hosted_execution_attempt_idempotency", &error))?;
    if let Some(row) = reservation {
        let result: IdempotencyResult = decode(row.get::<_, String>(0).as_str())?;
        let stored_intent: Option<String> = row.get(1);
        if stored_intent.as_deref() != Some(intent_ref.as_str())
            || result.result_ref != work_item_id.as_str()
        {
            return Err(state_error(
                "postgres_state.hosted_execution_attempt.idempotency_conflict",
                "hosted execution identity is bound to another invocation",
            ));
        }
        let existing =
            read_hosted_execution_attempt_tx(tx, work_item_id, false)?.ok_or_else(|| {
                state_error(
                    "postgres_state.hosted_execution_attempt.replay_missing",
                    "hosted execution attempt replay is missing its durable record",
                )
            })?;
        validate_hosted_execution_attempt_replay(existing.value(), &attempt)?;
        return Ok(existing);
    }
    if read_hosted_execution_attempt_tx(tx, work_item_id, true)?.is_some() {
        return Err(state_error(
            "postgres_state.hosted_execution_attempt.exists",
            "hosted work item already has an execution attempt",
        ));
    }
    tx.execute(
        "INSERT INTO workflow_os.idempotency (key, payload, intent_ref)
         VALUES ($1, $2, $3)",
        &[
            &storage_key,
            &encode(&IdempotencyResult {
                result_ref: work_item_id.as_str().to_owned(),
            })?,
            &intent_ref,
        ],
    )
    .map_err(|error| database_error("hosted_execution_attempt_idempotency", &error))?;
    let revision = put_record(
        tx,
        "hosted_execution_attempt",
        work_item_id.as_str(),
        attempt.status().storage_key(),
        &attempt,
        None,
        true,
    )?;
    Ok(PostgresRevisionedRecord {
        value: attempt,
        revision,
    })
}

fn release_fenced_lease_tx(
    tx: &mut Transaction<'_>,
    lease: &PostgresFencedLease,
) -> Result<(), WorkflowOsError> {
    let fence = i64::try_from(lease.fence_token).map_err(|_| {
        state_error(
            "postgres_state.lease.fence_invalid",
            "PostgreSQL worker lease fencing token is invalid",
        )
    })?;
    let affected = tx
        .execute(
            "UPDATE workflow_os.worker_leases
                SET expires_at = clock_timestamp()
              WHERE lease_key = $1
                AND owner = $2
                AND fence_token = $3
                AND expires_at > clock_timestamp()",
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

fn read_hosted_work_item_tx(
    tx: &mut Transaction<'_>,
    work_item_id: &HostedWorkItemId,
    for_update: bool,
) -> Result<Option<PostgresRevisionedRecord<HostedWorkItem>>, WorkflowOsError> {
    let suffix = if for_update { " FOR UPDATE" } else { "" };
    let query = format!(
        "SELECT key1, key2, payload, revision
           FROM workflow_os.records
          WHERE family = 'hosted_work_item' AND key1 = $1
          ORDER BY key2{suffix}"
    );
    let rows = tx
        .query(&query, &[&work_item_id.as_str()])
        .map_err(|error| database_error("hosted_work_item_read", &error))?;
    if rows.len() > 1 {
        return Err(state_error(
            "postgres_state.hosted_work_item.multiple_status_rows",
            "hosted work item has conflicting durable status rows",
        ));
    }
    let Some(row) = rows.into_iter().next() else {
        return Ok(None);
    };
    let key1: String = row.get(0);
    let key2: String = row.get(1);
    let value: HostedWorkItem = decode(row.get::<_, String>(2).as_str())?;
    validate_hosted_work_item_storage_identity(&value, &key1, &key2)?;
    Ok(Some(PostgresRevisionedRecord {
        value,
        revision: revision_from_i64(row.get(3))?,
    }))
}

fn read_hosted_execution_attempt_tx(
    tx: &mut Transaction<'_>,
    work_item_id: &HostedWorkItemId,
    for_update: bool,
) -> Result<Option<PostgresRevisionedRecord<HostedExecutionAttempt>>, WorkflowOsError> {
    let suffix = if for_update { " FOR UPDATE" } else { "" };
    let query = format!(
        "SELECT key1, key2, payload, revision
           FROM workflow_os.records
          WHERE family = 'hosted_execution_attempt' AND key1 = $1
          ORDER BY key2{suffix}"
    );
    let rows = tx
        .query(&query, &[&work_item_id.as_str()])
        .map_err(|error| database_error("hosted_execution_attempt_read", &error))?;
    if rows.len() > 1 {
        return Err(state_error(
            "postgres_state.hosted_execution_attempt.multiple_status_rows",
            "hosted execution attempt has conflicting durable status rows",
        ));
    }
    let Some(row) = rows.into_iter().next() else {
        return Ok(None);
    };
    let key1: String = row.get(0);
    let key2: String = row.get(1);
    let value: HostedExecutionAttempt = decode(row.get::<_, String>(2).as_str())?;
    validate_hosted_execution_attempt_storage_identity(&value, &key1, &key2)?;
    Ok(Some(PostgresRevisionedRecord {
        value,
        revision: revision_from_i64(row.get(3))?,
    }))
}

fn update_hosted_execution_attempt_tx(
    tx: &mut Transaction<'_>,
    prior: &HostedExecutionAttempt,
    next: &HostedExecutionAttempt,
    expected_revision: DurableRevision,
    row_already_locked: bool,
) -> Result<DurableRevision, WorkflowOsError> {
    if !row_already_locked {
        let current = read_hosted_execution_attempt_tx(tx, prior.work_item_id(), true)?
            .ok_or_else(|| {
                state_error(
                    "postgres_state.hosted_execution_attempt.missing",
                    "hosted execution attempt is missing",
                )
            })?;
        if current.value() != prior || current.revision() != expected_revision {
            return Err(state_error(
                "postgres_state.revision.stale",
                "PostgreSQL record revision is stale",
            ));
        }
    }
    if prior.execution_id() != next.execution_id()
        || prior.work_item_id() != next.work_item_id()
        || prior.request_fingerprint() != next.request_fingerprint()
        || prior.provider_id() != next.provider_id()
        || prior.provider_version() != next.provider_version()
        || prior.provider_configuration_hash() != next.provider_configuration_hash()
        || prior.prepared_at() != next.prepared_at()
    {
        return Err(state_error(
            "postgres_state.hosted_execution_attempt.identity_mismatch",
            "hosted execution attempt transition changed immutable binding",
        ));
    }
    let expected_revision_i64 = i64::try_from(expected_revision.get()).map_err(|_| {
        state_error(
            "postgres_state.revision.invalid",
            "PostgreSQL expected revision is invalid",
        )
    })?;
    let payload = encode(next)?;
    let row = tx
        .query_opt(
            "UPDATE workflow_os.records
                SET key2 = $4, payload = $5, revision = revision + 1,
                    updated_at = clock_timestamp()
              WHERE family = 'hosted_execution_attempt' AND key1 = $1 AND key2 = $2
                AND revision = $3
            RETURNING revision",
            &[
                &prior.work_item_id().as_str(),
                &prior.status().storage_key(),
                &expected_revision_i64,
                &next.status().storage_key(),
                &payload,
            ],
        )
        .map_err(|error| database_error("hosted_execution_attempt_update", &error))?;
    let Some(row) = row else {
        return Err(state_error(
            "postgres_state.revision.stale",
            "PostgreSQL record revision is stale",
        ));
    };
    revision_from_i64(row.get(0))
}

fn validate_hosted_execution_attempt_storage_identity(
    attempt: &HostedExecutionAttempt,
    key1: &str,
    key2: &str,
) -> Result<(), WorkflowOsError> {
    if attempt.work_item_id().as_str() != key1 || attempt.status().storage_key() != key2 {
        return Err(state_error(
            "postgres_state.hosted_execution_attempt.identity_mismatch",
            "hosted execution attempt durable identity does not match its payload",
        ));
    }
    Ok(())
}

fn validate_hosted_execution_attempt_replay(
    existing: &HostedExecutionAttempt,
    expected: &HostedExecutionAttempt,
) -> Result<(), WorkflowOsError> {
    if existing.execution_id() != expected.execution_id()
        || existing.work_item_id() != expected.work_item_id()
        || existing.request_fingerprint() != expected.request_fingerprint()
        || existing.provider_id() != expected.provider_id()
        || existing.provider_version() != expected.provider_version()
        || existing.provider_configuration_hash() != expected.provider_configuration_hash()
    {
        return Err(state_error(
            "postgres_state.hosted_execution_attempt.idempotency_conflict",
            "hosted execution identity is bound to another invocation",
        ));
    }
    Ok(())
}

fn update_hosted_work_item_tx(
    tx: &mut Transaction<'_>,
    prior: &HostedWorkItem,
    next: &HostedWorkItem,
    expected_revision: DurableRevision,
    row_already_locked: bool,
) -> Result<DurableRevision, WorkflowOsError> {
    if !row_already_locked {
        let current =
            read_hosted_work_item_tx(tx, prior.work_item_id(), true)?.ok_or_else(|| {
                state_error(
                    "postgres_state.hosted_work_item.missing",
                    "hosted work item is missing",
                )
            })?;
        if current.value() != prior || current.revision() != expected_revision {
            return Err(state_error(
                "postgres_state.revision.stale",
                "PostgreSQL record revision is stale",
            ));
        }
    }
    let expected_revision_i64 = i64::try_from(expected_revision.get()).map_err(|_| {
        state_error(
            "postgres_state.revision.invalid",
            "PostgreSQL expected revision is invalid",
        )
    })?;
    let payload = encode(next)?;
    let row = tx
        .query_opt(
            "UPDATE workflow_os.records
                SET key2 = $4, payload = $5, revision = revision + 1,
                    updated_at = clock_timestamp()
              WHERE family = 'hosted_work_item' AND key1 = $1 AND key2 = $2
                AND revision = $3
            RETURNING revision",
            &[
                &prior.work_item_id().as_str(),
                &prior.status().storage_key(),
                &expected_revision_i64,
                &next.status().storage_key(),
                &payload,
            ],
        )
        .map_err(|error| database_error("hosted_work_item_update", &error))?;
    let Some(row) = row else {
        return Err(state_error(
            "postgres_state.revision.stale",
            "PostgreSQL record revision is stale",
        ));
    };
    revision_from_i64(row.get(0))
}

fn validate_hosted_work_item_storage_identity(
    work_item: &HostedWorkItem,
    key1: &str,
    key2: &str,
) -> Result<(), WorkflowOsError> {
    if work_item.work_item_id().as_str() != key1 || work_item.status().storage_key() != key2 {
        return Err(state_error(
            "postgres_state.hosted_work_item.identity_mismatch",
            "hosted work item durable identity does not match its payload",
        ));
    }
    Ok(())
}

fn validate_hosted_unreceipted_projection_input(
    request: &PostgresCommitHostedUnreceiptedProjectionRequest<'_>,
) -> Result<(), WorkflowOsError> {
    if request.projection.projected_run().snapshot.identity.run_id != *request.work_item.run_id()
        || request
            .projection
            .events()
            .iter()
            .any(|event| event.run_id != *request.work_item.run_id())
    {
        return Err(state_error(
            "postgres_state.hosted_unreceipted_projection.binding.invalid",
            "hosted unreceipted projection binding is invalid",
        ));
    }
    let valid = match request.projection.outcome() {
        HostedUnreceiptedOutcome::RejectedBeforeStart => {
            request.work_item.status() == HostedWorkItemStatus::Failed
                && request.expected_attempt_revision.is_none()
                && request.projection.projected_run().snapshot.status == WorkflowRunStatus::Failed
        }
        HostedUnreceiptedOutcome::ReconciliationRequired => {
            request.work_item.status() == HostedWorkItemStatus::Ambiguous
                && request.expected_attempt_revision.is_some()
                && request.projection.projected_run().snapshot.status
                    == WorkflowRunStatus::Escalated
        }
    };
    if !valid {
        return Err(state_error(
            "postgres_state.hosted_unreceipted_projection.posture.invalid",
            "hosted unreceipted projection posture is invalid",
        ));
    }
    Ok(())
}

fn replay_hosted_unreceipted_projection_tx(
    tx: &mut Transaction<'_>,
    request: &PostgresCommitHostedUnreceiptedProjectionRequest<'_>,
) -> Result<Option<PostgresHostedReceiptCommitResult>, WorkflowOsError> {
    let existing = read_hosted_work_item_tx(tx, request.work_item.work_item_id(), true)?
        .ok_or_else(|| {
            state_error(
                "postgres_state.hosted_work_item.missing",
                "hosted work item is missing",
            )
        })?;
    if existing.value().status() == HostedWorkItemStatus::Running {
        return Ok(None);
    }
    let mut events_match = true;
    for event in request.projection.events() {
        if !event_exists_exact_tx(tx, event)? {
            events_match = false;
            break;
        }
    }
    if existing.value() != request.work_item || !events_match {
        return Err(state_error(
            "postgres_state.hosted_unreceipted_projection.replay_conflict",
            "hosted unreceipted projection replay conflicts with durable state",
        ));
    }
    let events = read_events_tx(tx, request.work_item.run_id())?;
    let projected = WorkflowRun::rehydrate(&events)?;
    if projected != *request.projection.projected_run() {
        return Err(state_error(
            "postgres_state.hosted_unreceipted_projection.replay_conflict",
            "hosted unreceipted projection replay conflicts with durable state",
        ));
    }
    let attempt = read_hosted_execution_attempt_tx(tx, request.work_item.work_item_id(), true)?;
    let attempt_revision = match request.projection.outcome() {
        HostedUnreceiptedOutcome::RejectedBeforeStart => {
            if attempt.is_some() {
                return Err(state_error(
                    "postgres_state.hosted_unreceipted_projection.replay_conflict",
                    "hosted unreceipted projection replay conflicts with durable state",
                ));
            }
            None
        }
        HostedUnreceiptedOutcome::ReconciliationRequired => {
            let attempt = attempt.ok_or_else(|| {
                state_error(
                    "postgres_state.hosted_execution_attempt.missing",
                    "hosted execution attempt is missing",
                )
            })?;
            if attempt.value().status() != HostedExecutionAttemptStatus::ReconciliationRequired {
                return Err(state_error(
                    "postgres_state.hosted_unreceipted_projection.replay_conflict",
                    "hosted unreceipted projection replay conflicts with durable state",
                ));
            }
            Some(attempt.revision())
        }
    };
    Ok(Some(PostgresHostedReceiptCommitResult {
        work_item_revision: existing.revision(),
        attempt_revision,
    }))
}

fn commit_fresh_hosted_unreceipted_projection_tx(
    tx: &mut Transaction<'_>,
    request: &PostgresCommitHostedUnreceiptedProjectionRequest<'_>,
) -> Result<PostgresHostedReceiptCommitResult, WorkflowOsError> {
    let prior =
        read_hosted_work_item_tx(tx, request.work_item.work_item_id(), true)?.ok_or_else(|| {
            state_error(
                "postgres_state.hosted_work_item.missing",
                "hosted work item is missing",
            )
        })?;
    if prior.revision() != request.expected_work_item_revision
        || prior.value().status() != HostedWorkItemStatus::Running
    {
        return Err(state_error(
            "postgres_state.hosted_unreceipted_projection.work_item_stale",
            "hosted unreceipted projection work item is stale",
        ));
    }
    validate_hosted_work_item_lease(request.lease, prior.value().work_item_id())?;
    validate_fence_tx(tx, request.lease)?;
    let expected = prior
        .value()
        .transition(request.work_item.status(), request.work_item.updated_at())?;
    if expected != *request.work_item {
        return Err(state_error(
            "postgres_state.hosted_work_item.identity_mismatch",
            "hosted work item transition changed immutable identity",
        ));
    }

    let attempt = read_hosted_execution_attempt_tx(tx, prior.value().work_item_id(), true)?;
    let next_attempt = match request.projection.outcome() {
        HostedUnreceiptedOutcome::RejectedBeforeStart => {
            if attempt.is_some() {
                return Err(state_error(
                    "postgres_state.hosted_unreceipted_projection.attempt.invalid",
                    "pre-provider rejection cannot include an invocation attempt",
                ));
            }
            None
        }
        HostedUnreceiptedOutcome::ReconciliationRequired => {
            let attempt = attempt.ok_or_else(|| {
                state_error(
                    "postgres_state.hosted_execution_attempt.missing",
                    "hosted execution attempt is missing",
                )
            })?;
            if Some(attempt.revision()) != request.expected_attempt_revision {
                return Err(state_error(
                    "postgres_state.revision.stale",
                    "PostgreSQL record revision is stale",
                ));
            }
            let next = attempt
                .value()
                .require_reconciliation(request.work_item.updated_at())?;
            Some((attempt, next))
        }
    };

    let snapshot_revision = current_snapshot_revision_tx(tx, request.work_item.run_id())?;
    for event in request.projection.events() {
        append_event_tx(tx, event)?;
    }
    let events = read_events_tx(tx, request.work_item.run_id())?;
    let projected = WorkflowRun::rehydrate(&events)?;
    if projected != *request.projection.projected_run() {
        return Err(state_error(
            "postgres_state.hosted_unreceipted_projection.result_mismatch",
            "hosted unreceipted projection does not match authoritative history",
        ));
    }
    put_record(
        tx,
        "snapshot",
        request.work_item.run_id().as_str(),
        "",
        &projected.snapshot,
        Some(snapshot_revision),
        false,
    )?;
    let work_item_revision = update_hosted_work_item_tx(
        tx,
        prior.value(),
        request.work_item,
        request.expected_work_item_revision,
        true,
    )?;
    let attempt_revision = if let Some((attempt, next)) = next_attempt {
        Some(update_hosted_execution_attempt_tx(
            tx,
            attempt.value(),
            &next,
            attempt.revision(),
            true,
        )?)
    } else {
        None
    };
    release_fenced_lease_tx(tx, request.lease)?;
    Ok(PostgresHostedReceiptCommitResult {
        work_item_revision,
        attempt_revision,
    })
}

fn replay_hosted_receipt_projection_tx(
    tx: &mut Transaction<'_>,
    request: &PostgresCommitHostedReceiptProjectionRequest<'_>,
) -> Result<Option<PostgresHostedReceiptCommitResult>, WorkflowOsError> {
    let receipt_request = request.receipt_commit;
    let Some(existing_receipt) = read_hosted_execution_receipt_tx(
        tx,
        receipt_request.work_item.work_item_id(),
        receipt_request.receipt.execution_id(),
    )?
    else {
        return Ok(None);
    };
    let existing_work_item =
        read_hosted_work_item_tx(tx, receipt_request.work_item.work_item_id(), true)?.ok_or_else(
            || {
                state_error(
                    "postgres_state.hosted_work_item.replay_missing",
                    "hosted terminal replay is missing its work item",
                )
            },
        )?;
    let existing_artifact = read_work_report_artifact_tx(
        tx,
        request.report_artifact.record().run_id(),
        request.report_artifact.record().report_id(),
    )?
    .ok_or_else(|| {
        state_error(
            "postgres_state.hosted_projection.replay_conflict",
            "hosted terminal replay conflicts with durable state",
        )
    })?;
    let mut events_match = true;
    for event in request.projection.events() {
        if !event_exists_exact_tx(tx, event)? {
            events_match = false;
            break;
        }
    }
    if existing_receipt != *receipt_request.receipt
        || existing_work_item.value() != receipt_request.work_item
        || existing_artifact != *request.report_artifact.record()
        || !events_match
    {
        return Err(state_error(
            "postgres_state.hosted_projection.replay_conflict",
            "hosted terminal replay conflicts with durable state",
        ));
    }
    if let Some(work_item_binding) = read_project_resource_binding_tx(
        tx,
        HostedProjectResourceKind::WorkItem,
        receipt_request.work_item.work_item_id().as_str(),
        false,
    )? {
        let scope = work_item_binding.value().scope();
        require_project_resource_binding_tx(
            tx,
            scope,
            HostedProjectResourceKind::Run,
            receipt_request.work_item.run_id().as_str(),
        )?;
        require_project_resource_binding_tx(
            tx,
            scope,
            HostedProjectResourceKind::ExecutionReceipt,
            receipt_request.receipt.execution_id().as_str(),
        )?;
        require_project_resource_binding_tx(
            tx,
            scope,
            HostedProjectResourceKind::Report,
            request.report_artifact.record().report_id().as_str(),
        )?;
    }
    Ok(Some(PostgresHostedReceiptCommitResult {
        work_item_revision: existing_work_item.revision(),
        attempt_revision: read_hosted_execution_attempt_tx(
            tx,
            receipt_request.work_item.work_item_id(),
            true,
        )?
        .map(|attempt| attempt.revision()),
    }))
}

fn commit_fresh_hosted_receipt_projection_tx(
    tx: &mut Transaction<'_>,
    request: &PostgresCommitHostedReceiptProjectionRequest<'_>,
) -> Result<PostgresHostedReceiptCommitResult, WorkflowOsError> {
    let receipt_request = request.receipt_commit;
    let prior_work_item =
        read_hosted_work_item_tx(tx, receipt_request.work_item.work_item_id(), true)?.ok_or_else(
            || {
                state_error(
                    "postgres_state.hosted_work_item.missing",
                    "hosted work item is missing",
                )
            },
        )?;
    if prior_work_item.revision() != receipt_request.expected_work_item_revision
        || prior_work_item.value().status() != HostedWorkItemStatus::Running
    {
        return Err(state_error(
            "postgres_state.hosted_projection.work_item_stale",
            "hosted terminal projection work item is stale",
        ));
    }
    validate_hosted_work_item_lease(
        receipt_request.lease,
        prior_work_item.value().work_item_id(),
    )?;
    validate_fence_tx(tx, receipt_request.lease)?;
    let prior_attempt =
        read_hosted_execution_attempt_tx(tx, prior_work_item.value().work_item_id(), true)?
            .ok_or_else(|| {
                state_error(
                    "postgres_state.hosted_execution_attempt.missing",
                    "hosted execution attempt is missing",
                )
            })?;
    if prior_attempt.revision() != request.expected_attempt_revision {
        return Err(state_error(
            "postgres_state.revision.stale",
            "PostgreSQL record revision is stale",
        ));
    }
    let expected_work_item = prior_work_item.value().transition(
        receipt_request.work_item.status(),
        receipt_request.work_item.updated_at(),
    )?;
    if expected_work_item != *receipt_request.work_item {
        return Err(state_error(
            "postgres_state.hosted_work_item.identity_mismatch",
            "hosted work item transition changed immutable identity",
        ));
    }
    let terminal_attempt = prior_attempt
        .value()
        .mark_terminal(receipt_request.receipt)?;
    let snapshot_revision = current_snapshot_revision_tx(tx, receipt_request.work_item.run_id())?;
    for event in request.projection.events() {
        append_event_tx(tx, event)?;
    }
    let events = read_events_tx(tx, receipt_request.work_item.run_id())?;
    let projected = WorkflowRun::rehydrate(&events)?;
    if projected != *request.projection.projected_run() {
        return Err(state_error(
            "postgres_state.hosted_projection.result_mismatch",
            "hosted terminal projection does not match authoritative history",
        ));
    }
    put_record(
        tx,
        "snapshot",
        receipt_request.work_item.run_id().as_str(),
        "",
        &projected.snapshot,
        Some(snapshot_revision),
        false,
    )?;
    let work_item_revision = update_hosted_work_item_tx(
        tx,
        prior_work_item.value(),
        receipt_request.work_item,
        receipt_request.expected_work_item_revision,
        true,
    )?;
    let attempt_revision = update_hosted_execution_attempt_tx(
        tx,
        prior_attempt.value(),
        &terminal_attempt,
        request.expected_attempt_revision,
        true,
    )?;
    persist_hosted_receipt_and_report_tx(tx, request)?;
    release_fenced_lease_tx(tx, receipt_request.lease)?;
    Ok(PostgresHostedReceiptCommitResult {
        work_item_revision,
        attempt_revision: Some(attempt_revision),
    })
}

fn persist_hosted_receipt_and_report_tx(
    tx: &mut Transaction<'_>,
    request: &PostgresCommitHostedReceiptProjectionRequest<'_>,
) -> Result<(), WorkflowOsError> {
    let receipt_request = request.receipt_commit;
    put_record(
        tx,
        "hosted_execution_receipt",
        receipt_request.work_item.work_item_id().as_str(),
        receipt_request.receipt.execution_id().as_str(),
        receipt_request.receipt,
        None,
        true,
    )?;
    put_record(
        tx,
        "work_report",
        request.report_artifact.record().report_id().as_str(),
        request.report_artifact.record().run_id().as_str(),
        request.report_artifact.record(),
        None,
        true,
    )?;
    if let Some(work_item_binding) = read_project_resource_binding_tx(
        tx,
        HostedProjectResourceKind::WorkItem,
        receipt_request.work_item.work_item_id().as_str(),
        false,
    )? {
        let scope = work_item_binding.value().scope();
        require_project_resource_binding_tx(
            tx,
            scope,
            HostedProjectResourceKind::Run,
            receipt_request.work_item.run_id().as_str(),
        )?;
        create_project_resource_binding_tx(
            tx,
            &HostedProjectResourceBinding::new(
                scope.clone(),
                HostedProjectResourceKind::ExecutionReceipt,
                receipt_request.receipt.execution_id().as_str(),
                HostedProjectResourceBindingStatus::Active,
                receipt_request.receipt.terminal_at(),
            )?,
        )?;
        create_project_resource_binding_tx(
            tx,
            &HostedProjectResourceBinding::new(
                scope.clone(),
                HostedProjectResourceKind::Report,
                request.report_artifact.record().report_id().as_str(),
                HostedProjectResourceBindingStatus::Active,
                receipt_request.receipt.terminal_at(),
            )?,
        )?;
    }
    Ok(())
}

fn create_hosted_work_item_tx(
    tx: &mut Transaction<'_>,
    work_item: &HostedWorkItem,
) -> Result<PostgresHostedWorkItemCreateResult, WorkflowOsError> {
    if work_item.status() != HostedWorkItemStatus::Queued || work_item.attempt_count() != 0 {
        return Err(state_error(
            "postgres_state.hosted_work_item.create_posture.invalid",
            "hosted work item must be newly queued",
        ));
    }
    let payload = encode(work_item)?;
    let fingerprint = SpecContentHash::from_text(&payload);
    let storage_key = format!(
        "hosted/create/{}",
        SpecContentHash::from_text(work_item.idempotency_key().as_str()).as_str()
    );
    let intent_ref = format!("hosted-work-request/{}", fingerprint.as_str());
    let result = IdempotencyResult {
        result_ref: work_item.work_item_id().as_str().to_owned(),
    };
    let existing_reservation = tx
        .query_opt(
            "SELECT payload, intent_ref FROM workflow_os.idempotency
              WHERE key = $1 FOR UPDATE",
            &[&storage_key],
        )
        .map_err(|error| database_error("hosted_work_item_idempotency", &error))?;
    if let Some(row) = existing_reservation {
        let prior: IdempotencyResult = decode(row.get::<_, String>(0).as_str())?;
        let stored_intent: Option<String> = row.get(1);
        if stored_intent.as_deref() != Some(intent_ref.as_str())
            || prior.result_ref != work_item.work_item_id().as_str()
        {
            return Err(state_error(
                "postgres_state.idempotency.intent_conflict",
                "PostgreSQL idempotency key is bound to another intent",
            ));
        }
        let existing =
            read_hosted_work_item_tx(tx, work_item.work_item_id(), false)?.ok_or_else(|| {
                state_error(
                    "postgres_state.hosted_work_item.replay_missing",
                    "idempotent hosted work item replay is missing its durable record",
                )
            })?;
        return Ok(PostgresHostedWorkItemCreateResult::Replayed(existing));
    }
    validate_hosted_work_item_bundle_tx(tx, work_item)?;
    validate_hosted_work_item_run_tx(tx, work_item)?;
    tx.execute(
        "INSERT INTO workflow_os.idempotency (key, payload, intent_ref)
         VALUES ($1, $2, $3)",
        &[&storage_key, &encode(&result)?, &intent_ref],
    )
    .map_err(|error| database_error("hosted_work_item_idempotency", &error))?;
    let revision = put_record(
        tx,
        "hosted_work_item",
        work_item.work_item_id().as_str(),
        work_item.status().storage_key(),
        work_item,
        None,
        true,
    )?;
    Ok(PostgresHostedWorkItemCreateResult::Created(
        PostgresRevisionedRecord {
            value: work_item.clone(),
            revision,
        },
    ))
}

fn current_snapshot_revision_tx(
    tx: &mut Transaction<'_>,
    run_id: &WorkflowRunId,
) -> Result<DurableRevision, WorkflowOsError> {
    let row = tx
        .query_opt(
            "SELECT revision FROM workflow_os.records
              WHERE family = 'snapshot' AND key1 = $1 AND key2 = ''
              FOR UPDATE",
            &[&run_id.as_str()],
        )
        .map_err(|error| database_error("hosted_projection_snapshot_read", &error))?
        .ok_or_else(|| {
            state_error(
                "postgres_state.hosted_projection.snapshot_missing",
                "hosted projection snapshot is missing",
            )
        })?;
    revision_from_i64(row.get(0))
}

fn event_exists_exact_tx(
    tx: &mut Transaction<'_>,
    event: &WorkflowRunEvent,
) -> Result<bool, WorkflowOsError> {
    let row = tx
        .query_opt(
            "SELECT payload FROM workflow_os.events WHERE event_id = $1",
            &[&event.event_id.as_str()],
        )
        .map_err(|error| database_error("hosted_projection_event_read", &error))?;
    let Some(row) = row else {
        return Ok(false);
    };
    let stored: WorkflowRunEvent = decode(row.get::<_, String>(0).as_str())?;
    Ok(stored == *event)
}

fn read_hosted_execution_receipt_tx(
    tx: &mut Transaction<'_>,
    work_item_id: &HostedWorkItemId,
    execution_id: &crate::HostedExecutionId,
) -> Result<Option<HostedExecutionReceipt>, WorkflowOsError> {
    let row = tx
        .query_opt(
            "SELECT payload FROM workflow_os.records
              WHERE family = 'hosted_execution_receipt' AND key1 = $1 AND key2 = $2
              FOR UPDATE",
            &[&work_item_id.as_str(), &execution_id.as_str()],
        )
        .map_err(|error| database_error("hosted_execution_receipt_read", &error))?;
    let Some(row) = row else {
        return Ok(None);
    };
    let receipt: HostedExecutionReceipt = decode(row.get::<_, String>(0).as_str())?;
    if receipt.execution_id() != execution_id {
        return Err(state_error(
            "postgres_state.hosted_execution_receipt.identity_mismatch",
            "hosted execution receipt storage identity is invalid",
        ));
    }
    Ok(Some(receipt))
}

fn read_work_report_artifact_tx(
    tx: &mut Transaction<'_>,
    run_id: &WorkflowRunId,
    report_id: &WorkReportId,
) -> Result<Option<WorkReportArtifactRecord>, WorkflowOsError> {
    let row = tx
        .query_opt(
            "SELECT payload FROM workflow_os.records
              WHERE family = 'work_report' AND key1 = $1 AND key2 = $2
              FOR UPDATE",
            &[&report_id.as_str(), &run_id.as_str()],
        )
        .map_err(|error| database_error("work_report_read", &error))?;
    let Some(row) = row else {
        return Ok(None);
    };
    let artifact: WorkReportArtifactRecord = decode(row.get::<_, String>(0).as_str())?;
    if artifact.report_id() != report_id || artifact.run_id() != run_id {
        return Err(state_error(
            "postgres_state.work_report.identity_mismatch",
            "PostgreSQL work report artifact identity is invalid",
        ));
    }
    Ok(Some(artifact))
}

fn validate_hosted_work_item_lease(
    lease: &PostgresFencedLease,
    work_item_id: &HostedWorkItemId,
) -> Result<(), WorkflowOsError> {
    let expected = PostgresLeaseKey::new(format!("hosted-work-item/{}", work_item_id.as_str()))?;
    if lease.key() != &expected {
        return Err(state_error(
            "postgres_state.hosted_work_item.lease_identity_mismatch",
            "hosted work item lease identity does not match",
        ));
    }
    Ok(())
}

fn validate_hosted_work_item_bundle_tx(
    tx: &mut Transaction<'_>,
    work_item: &HostedWorkItem,
) -> Result<(), WorkflowOsError> {
    let row = tx
        .query_opt(
            "SELECT root_hash, payload FROM workflow_os.immutable_manifests
              WHERE run_id = $1 FOR SHARE",
            &[&work_item.run_id().as_str()],
        )
        .map_err(|error| database_error("hosted_work_item_bundle_read", &error))?
        .ok_or_else(|| {
            state_error(
                "postgres_state.hosted_work_item.bundle_missing",
                "hosted work item immutable bundle is missing",
            )
        })?;
    let stored_root: String = row.get(0);
    let manifest: ImmutableRunBundleManifest = decode(row.get::<_, String>(1).as_str())?;
    if stored_root != work_item.bundle_root_hash().as_str()
        || manifest.root_hash() != work_item.bundle_root_hash()
        || manifest.run_id() != work_item.run_id()
        || manifest.workflow_id() != work_item.workflow_id()
        || manifest.bundle_id() != work_item.bundle_id()
        || manifest.bundle_version() != work_item.bundle_version()
    {
        return Err(state_error(
            "postgres_state.hosted_work_item.bundle_mismatch",
            "hosted work item immutable bundle binding does not match",
        ));
    }
    Ok(())
}

fn validate_hosted_work_item_run_tx(
    tx: &mut Transaction<'_>,
    work_item: &HostedWorkItem,
) -> Result<(), WorkflowOsError> {
    let row = tx
        .query_opt(
            "SELECT payload FROM workflow_os.records
              WHERE family = 'snapshot' AND key1 = $1 AND key2 = ''
              FOR SHARE",
            &[&work_item.run_id().as_str()],
        )
        .map_err(|error| database_error("hosted_work_item_run_read", &error))?
        .ok_or_else(|| {
            state_error(
                "postgres_state.hosted_work_item.run_missing",
                "hosted work item governed run is missing",
            )
        })?;
    let snapshot: WorkflowRunSnapshot = decode(row.get::<_, String>(0).as_str())?;
    let bundle = snapshot
        .identity
        .immutable_run_bundle
        .as_ref()
        .ok_or_else(|| {
            state_error(
                "postgres_state.hosted_work_item.run_bundle_missing",
                "hosted work item governed run is not bundle-backed",
            )
        })?;
    if snapshot.status != WorkflowRunStatus::Running {
        return Err(state_error(
            "postgres_state.hosted_work_item.run_not_running",
            "hosted work item governed run is not running",
        ));
    }
    if snapshot.identity.run_id != *work_item.run_id()
        || snapshot.identity.workflow_id != *work_item.workflow_id()
        || bundle.bundle_id() != work_item.bundle_id()
        || bundle.bundle_version() != work_item.bundle_version()
        || bundle.root_hash() != work_item.bundle_root_hash()
    {
        return Err(state_error(
            "postgres_state.hosted_work_item.run_binding_mismatch",
            "hosted work item governed run binding does not match",
        ));
    }
    Ok(())
}

fn validate_hosted_receipt_input(
    request: &PostgresCommitHostedReceiptRequest<'_>,
) -> Result<(), WorkflowOsError> {
    let execution_request = request.work_item.execution_request();
    if request.work_item.run_id() != execution_request.run_id()
        || request.work_item.workflow_id() != execution_request.workflow_id()
        || request.work_item.bundle_id() != execution_request.bundle_id()
        || request.work_item.bundle_version() != execution_request.bundle_version()
        || request.work_item.bundle_root_hash() != execution_request.bundle_root_hash()
        || request.receipt.request_fingerprint() != &execution_request.fingerprint()
        || request.receipt.policy_hash() != execution_request.policy().policy_hash()
    {
        return Err(state_error(
            "postgres_state.hosted_execution.binding.invalid",
            "hosted execution result binding is invalid",
        ));
    }
    let expected_status = match request.receipt.status() {
        HostedExecutionStatus::Completed => HostedWorkItemStatus::Completed,
        HostedExecutionStatus::Failed => HostedWorkItemStatus::Failed,
        HostedExecutionStatus::Canceled => HostedWorkItemStatus::Canceled,
        HostedExecutionStatus::Ambiguous => HostedWorkItemStatus::Ambiguous,
    };
    if request.work_item.status() != expected_status {
        return Err(state_error(
            "postgres_state.hosted_execution.status_mismatch",
            "hosted execution result status is inconsistent",
        ));
    }
    Ok(())
}

fn database_timestamp_tx(tx: &mut Transaction<'_>) -> Result<crate::Timestamp, WorkflowOsError> {
    let epoch_millis: i64 = tx
        .query_one(
            "SELECT (extract(epoch FROM clock_timestamp()) * 1000)::bigint",
            &[],
        )
        .map_err(|error| database_error("database_time", &error))?
        .get(0);
    let nanos = i128::from(epoch_millis)
        .checked_mul(1_000_000)
        .ok_or_else(|| {
            state_error(
                "postgres_state.database_time.invalid",
                "database time is invalid",
            )
        })?;
    let value = time::OffsetDateTime::from_unix_timestamp_nanos(nanos).map_err(|_| {
        state_error(
            "postgres_state.database_time.invalid",
            "database time is invalid",
        )
    })?;
    Ok(crate::Timestamp::from_offset_date_time(value))
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

fn count_from_i64(value: i64) -> Result<u64, WorkflowOsError> {
    u64::try_from(value).map_err(|_| {
        state_error(
            "postgres_state.metrics.count.invalid",
            "stored PostgreSQL metrics count is invalid",
        )
    })
}

fn optional_count_from_i64(value: Option<i64>) -> Result<Option<u64>, WorkflowOsError> {
    value.map(count_from_i64).transpose()
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

const SCHEMA_BOOTSTRAP_SQL: &str = r"
CREATE SCHEMA IF NOT EXISTS workflow_os;

CREATE TABLE IF NOT EXISTS workflow_os.schema_metadata (
    singleton BOOLEAN PRIMARY KEY DEFAULT TRUE CHECK (singleton),
    schema_version INTEGER NOT NULL CHECK (schema_version > 0),
    checksum TEXT NOT NULL,
    recovery_required BOOLEAN NOT NULL DEFAULT FALSE,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);
";

const SCHEMA_V1_SQL: &str = r"
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

const SCHEMA_V2_SQL: &str = r"
CREATE TABLE workflow_os.hosted_authority_registry_high_watermarks (
    organization_id TEXT PRIMARY KEY,
    authority_revision BIGINT NOT NULL CHECK (authority_revision > 0),
    commitment_algorithm TEXT NOT NULL,
    commitment_fingerprint TEXT NOT NULL,
    accepted_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

CREATE TABLE workflow_os.project_approval_routes (
    logical_subject_id TEXT PRIMARY KEY,
    route_id TEXT NOT NULL UNIQUE,
    organization_id TEXT NOT NULL,
    project_id TEXT NOT NULL,
    run_id TEXT NOT NULL,
    approval_id TEXT NOT NULL,
    routing_reason TEXT NOT NULL,
    escalation_id TEXT,
    route_status TEXT NOT NULL,
    recipient_actor_id TEXT,
    notification_posture TEXT NOT NULL,
    authority_revision BIGINT NOT NULL CHECK (authority_revision > 0),
    source_algorithm TEXT NOT NULL,
    source_route_id TEXT NOT NULL,
    source_fingerprint TEXT NOT NULL,
    record_version TEXT NOT NULL,
    resolved_at TEXT NOT NULL,
    record_created_at TEXT NOT NULL,
    canonical_payload TEXT NOT NULL,
    canonical_payload_hash TEXT NOT NULL,
    inserted_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    CHECK (source_route_id = route_id),
    CHECK (
        (route_status = 'routed'
            AND recipient_actor_id IS NOT NULL
            AND notification_posture = 'available_for_project_inbox')
        OR
        (route_status IN ('unresolved_missing_metadata', 'unresolved_authority_unavailable')
            AND recipient_actor_id IS NULL
            AND notification_posture = 'unavailable_route_unresolved')
    ),
    CHECK (
        (routing_reason = 'workflow_maintainer' AND escalation_id IS NULL)
        OR
        (routing_reason = 'workflow_escalation_contact' AND escalation_id IS NOT NULL)
    )
);
CREATE INDEX project_approval_routes_recipient_idx
    ON workflow_os.project_approval_routes
       (organization_id, project_id, recipient_actor_id, logical_subject_id)
    WHERE route_status = 'routed' AND recipient_actor_id IS NOT NULL;
CREATE INDEX project_approval_routes_approval_idx
    ON workflow_os.project_approval_routes
       (organization_id, project_id, run_id, approval_id, logical_subject_id);
CREATE INDEX project_approval_routes_reconciliation_idx
    ON workflow_os.project_approval_routes
       (organization_id, project_id, run_id, approval_id,
        routing_reason, escalation_id, logical_subject_id);
";
