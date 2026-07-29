use std::collections::BTreeSet;

use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    ActorId, EventId, EventSequenceNumber, IdempotencyKey, IdempotencyResult, IdempotencyWrite,
    LocalStateBackend, SpecContentHash, StateBackend, WorkflowOsError, WorkflowOsErrorKind,
    WorkflowRunEvent,
};

/// Version of the Core-owned durable-state semantic contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DurableStateContractVersion {
    /// Initial contract vocabulary and local-backend conformance baseline.
    V1,
}

/// Physical deployment posture of a durable-state backend.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DurableStateBackendKind {
    /// Preview filesystem backend for one local project.
    LocalFilesystemPreview,
    /// Future embedded `SQLite` backend.
    EmbeddedSqlite,
    /// Explicit shared `PostgreSQL` backend.
    SharedPostgresql,
}

/// One observable durable-state guarantee.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DurableStateCapability {
    /// Events are validated and returned in canonical sequence order.
    OrderedEventAppend,
    /// Immutable run identity is validated during event append.
    ImmutableRunIdentityValidation,
    /// Duplicate idempotency keys return the original bounded result.
    IdempotencyReplay,
    /// A process-local exclusive lock can report deterministic contention.
    ProcessLocalExclusiveLock,
    /// Independent authoritative records can be committed atomically.
    CrossRecordAtomicCommit,
    /// Mutations can enforce an expected durable revision.
    CompareAndSetRevision,
    /// Leases expire and use fencing tokens that reject stale holders.
    ExpiringFencedLease,
    /// Schema compatibility and migration state are durably managed.
    ManagedSchemaMigration,
    /// The backend supports tested backup and restore semantics.
    VerifiedBackupRestore,
    /// The backend supports concurrent stateless workers.
    SharedWorkerConcurrency,
}

/// One bounded transaction family owned by Core.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DurableStateTransactionKind {
    /// Validate the current event tail and append exactly one next event.
    AppendRunEvent,
    /// Reserve idempotency and record durable pre-effect intent.
    ReserveIdempotencyAndRecordIntent,
    /// Record an observed external-operation outcome after the provider call.
    RecordExternalOperationOutcome,
    /// Validate approval context and persist the decision plus authority event.
    RecordApprovalDecision,
    /// Compare and transition a `SideEffect` record plus its authority event.
    TransitionSideEffect,
    /// Verify references and publish an immutable run bundle once.
    PublishImmutableRunBundle,
    /// Commit an authoritative record before dependent projections.
    CommitAuthoritativeResultAndProjections,
}

impl DurableStateTransactionKind {
    const ALL: [Self; 7] = [
        Self::AppendRunEvent,
        Self::ReserveIdempotencyAndRecordIntent,
        Self::RecordExternalOperationOutcome,
        Self::RecordApprovalDecision,
        Self::TransitionSideEffect,
        Self::PublishImmutableRunBundle,
        Self::CommitAuthoritativeResultAndProjections,
    ];

    /// Returns every transaction family in stable order.
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &Self::ALL
    }
}

/// Declared support for a contract behavior.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DurableStateSupport {
    /// The backend claims the behavior and must pass its conformance scenarios.
    Supported,
    /// The backend rejects or does not expose the behavior.
    Unsupported,
}

/// Stable durable-state conflict and recovery classification.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DurableStateConflictKind {
    /// Caller input is invalid before storage is consulted.
    InvalidInput,
    /// A durable invariant would be violated.
    InvariantViolation,
    /// An idempotent operation resolved to its prior result.
    IdempotentReplay,
    /// A bounded retry may succeed after reading current durable state.
    RetryableWriteConflict,
    /// A lock or lease is held by another owner.
    LockOrLeaseContention,
    /// The durable backend is unavailable.
    BackendUnavailable,
    /// The stored schema is not compatible with this adapter.
    IncompatibleSchema,
    /// Stored state is corrupt.
    CorruptState,
    /// A schema migration is required before use.
    MigrationRequired,
    /// Operator reconciliation or recovery is required.
    RecoveryRequired,
}

/// Monotonic record revision used by future compare-and-set operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(transparent)]
pub struct DurableRevision(u64);

impl DurableRevision {
    /// Creates a positive durable revision.
    ///
    /// # Errors
    ///
    /// Returns a stable error when the revision is zero.
    pub fn new(value: u64) -> Result<Self, WorkflowOsError> {
        if value == 0 {
            return Err(contract_error(
                "revision.invalid",
                "durable revision must be positive",
            ));
        }
        Ok(Self(value))
    }

    /// Returns the numeric revision.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl<'de> Deserialize<'de> for DurableRevision {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u64::deserialize(deserializer)?;
        Self::new(value).map_err(|_| serde::de::Error::custom("durable revision is invalid"))
    }
}

/// Lease semantics declared by a backend.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DurableLeaseSemantics {
    /// Lock ownership is local and has no expiry or fencing guarantee.
    ProcessLocalUnfenced,
    /// Lease ownership expires and stale holders are rejected by fencing token.
    ExpiringFenced,
}

/// Durable adapter schema posture.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DurableStateSchemaPosture {
    /// The backend does not manage a database schema.
    NotManaged,
    /// The adapter schema is current and ready.
    Ready,
    /// A compatible migration is required before ordinary use.
    MigrationRequired,
    /// A partial or inconsistent migration requires operator recovery.
    RecoveryRequired,
}

/// Bounded adapter schema metadata.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct DurableStateSchemaMetadata {
    adapter_schema_version: Option<u32>,
    posture: DurableStateSchemaPosture,
}

impl DurableStateSchemaMetadata {
    /// Creates metadata for a backend without a managed database schema.
    #[must_use]
    pub const fn unmanaged() -> Self {
        Self {
            adapter_schema_version: None,
            posture: DurableStateSchemaPosture::NotManaged,
        }
    }

    /// Creates metadata for a managed adapter schema.
    ///
    /// # Errors
    ///
    /// Returns a stable error for a zero version or `NotManaged` posture.
    pub fn managed(
        adapter_schema_version: u32,
        posture: DurableStateSchemaPosture,
    ) -> Result<Self, WorkflowOsError> {
        if adapter_schema_version == 0 || posture == DurableStateSchemaPosture::NotManaged {
            return Err(contract_error(
                "schema_metadata.invalid",
                "durable schema metadata is invalid",
            ));
        }
        Ok(Self {
            adapter_schema_version: Some(adapter_schema_version),
            posture,
        })
    }

    /// Returns the adapter schema version, when managed.
    #[must_use]
    pub const fn adapter_schema_version(self) -> Option<u32> {
        self.adapter_schema_version
    }

    /// Returns the declared schema posture.
    #[must_use]
    pub const fn posture(self) -> DurableStateSchemaPosture {
        self.posture
    }
}

#[derive(Deserialize)]
struct DurableStateSchemaMetadataWire {
    adapter_schema_version: Option<u32>,
    posture: DurableStateSchemaPosture,
}

impl<'de> Deserialize<'de> for DurableStateSchemaMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = DurableStateSchemaMetadataWire::deserialize(deserializer)?;
        match (wire.adapter_schema_version, wire.posture) {
            (None, DurableStateSchemaPosture::NotManaged) => Ok(Self::unmanaged()),
            (Some(version), posture) => Self::managed(version, posture)
                .map_err(|_| serde::de::Error::custom("durable schema metadata is invalid")),
            _ => Err(serde::de::Error::custom(
                "durable schema metadata is invalid",
            )),
        }
    }
}

/// One transaction-family support declaration.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DurableStateTransactionSupport {
    kind: DurableStateTransactionKind,
    support: DurableStateSupport,
}

impl DurableStateTransactionSupport {
    /// Creates one explicit support declaration.
    #[must_use]
    pub const fn new(kind: DurableStateTransactionKind, support: DurableStateSupport) -> Self {
        Self { kind, support }
    }

    /// Returns the transaction family.
    #[must_use]
    pub const fn kind(self) -> DurableStateTransactionKind {
        self.kind
    }

    /// Returns its support posture.
    #[must_use]
    pub const fn support(self) -> DurableStateSupport {
        self.support
    }
}

/// Validated durable-state semantic contract declared by one backend.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DurableStateSemanticContract {
    version: DurableStateContractVersion,
    backend_kind: DurableStateBackendKind,
    capabilities: Vec<DurableStateCapability>,
    transactions: Vec<DurableStateTransactionSupport>,
    lease_semantics: DurableLeaseSemantics,
    schema: DurableStateSchemaMetadata,
}

impl DurableStateSemanticContract {
    /// Creates and validates a backend semantic contract.
    ///
    /// # Errors
    ///
    /// Returns a stable error for duplicate capabilities, duplicate transaction
    /// declarations, or missing transaction-family declarations.
    pub fn new(
        version: DurableStateContractVersion,
        backend_kind: DurableStateBackendKind,
        capabilities: Vec<DurableStateCapability>,
        transactions: Vec<DurableStateTransactionSupport>,
        lease_semantics: DurableLeaseSemantics,
        schema: DurableStateSchemaMetadata,
    ) -> Result<Self, WorkflowOsError> {
        let capability_set = capabilities.iter().copied().collect::<BTreeSet<_>>();
        if capability_set.len() != capabilities.len() {
            return Err(contract_error(
                "capability.duplicate",
                "durable state capabilities contain a duplicate",
            ));
        }

        let transaction_set = transactions
            .iter()
            .map(|declaration| declaration.kind())
            .collect::<BTreeSet<_>>();
        if transaction_set.len() != transactions.len() {
            return Err(contract_error(
                "transaction.duplicate",
                "durable state transaction declarations contain a duplicate",
            ));
        }
        if transaction_set
            != DurableStateTransactionKind::all()
                .iter()
                .copied()
                .collect::<BTreeSet<_>>()
        {
            return Err(contract_error(
                "transaction.incomplete",
                "durable state transaction declarations are incomplete",
            ));
        }

        let mut capabilities = capabilities;
        capabilities.sort_unstable();
        let mut transactions = transactions;
        transactions.sort_unstable_by_key(|declaration| declaration.kind());
        Ok(Self {
            version,
            backend_kind,
            capabilities,
            transactions,
            lease_semantics,
            schema,
        })
    }

    /// Returns the contract version.
    #[must_use]
    pub const fn version(&self) -> DurableStateContractVersion {
        self.version
    }

    /// Returns the backend deployment kind.
    #[must_use]
    pub const fn backend_kind(&self) -> DurableStateBackendKind {
        self.backend_kind
    }

    /// Returns capabilities in stable order.
    #[must_use]
    pub fn capabilities(&self) -> &[DurableStateCapability] {
        &self.capabilities
    }

    /// Returns whether a capability is declared.
    #[must_use]
    pub fn supports_capability(&self, capability: DurableStateCapability) -> bool {
        self.capabilities.binary_search(&capability).is_ok()
    }

    /// Returns transaction declarations in stable order.
    #[must_use]
    pub fn transactions(&self) -> &[DurableStateTransactionSupport] {
        &self.transactions
    }

    /// Returns the support posture for one transaction family.
    #[must_use]
    pub fn transaction_support(&self, kind: DurableStateTransactionKind) -> DurableStateSupport {
        self.transactions
            .iter()
            .find(|declaration| declaration.kind() == kind)
            .map_or(DurableStateSupport::Unsupported, |declaration| {
                declaration.support()
            })
    }

    /// Returns lease semantics.
    #[must_use]
    pub const fn lease_semantics(&self) -> DurableLeaseSemantics {
        self.lease_semantics
    }

    /// Returns schema metadata.
    #[must_use]
    pub const fn schema(&self) -> DurableStateSchemaMetadata {
        self.schema
    }
}

#[derive(Deserialize)]
struct DurableStateSemanticContractWire {
    version: DurableStateContractVersion,
    backend_kind: DurableStateBackendKind,
    capabilities: Vec<DurableStateCapability>,
    transactions: Vec<DurableStateTransactionSupport>,
    lease_semantics: DurableLeaseSemantics,
    schema: DurableStateSchemaMetadata,
}

impl<'de> Deserialize<'de> for DurableStateSemanticContract {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = DurableStateSemanticContractWire::deserialize(deserializer)?;
        Self::new(
            wire.version,
            wire.backend_kind,
            wire.capabilities,
            wire.transactions,
            wire.lease_semantics,
            wire.schema,
        )
        .map_err(|_| serde::de::Error::custom("durable state semantic contract is invalid"))
    }
}

/// Backend capability declaration boundary.
pub trait DurableStateContractProvider {
    /// Returns the backend's validated semantic contract.
    ///
    /// # Errors
    ///
    /// Returns a stable error when the backend declaration is invalid.
    fn durable_state_contract(&self) -> Result<DurableStateSemanticContract, WorkflowOsError>;
}

impl DurableStateContractProvider for LocalStateBackend {
    fn durable_state_contract(&self) -> Result<DurableStateSemanticContract, WorkflowOsError> {
        DurableStateSemanticContract::new(
            DurableStateContractVersion::V1,
            DurableStateBackendKind::LocalFilesystemPreview,
            vec![
                DurableStateCapability::OrderedEventAppend,
                DurableStateCapability::ImmutableRunIdentityValidation,
                DurableStateCapability::IdempotencyReplay,
                DurableStateCapability::ProcessLocalExclusiveLock,
            ],
            DurableStateTransactionKind::all()
                .iter()
                .copied()
                .map(|kind| {
                    DurableStateTransactionSupport::new(kind, DurableStateSupport::Unsupported)
                })
                .collect(),
            DurableLeaseSemantics::ProcessLocalUnfenced,
            DurableStateSchemaMetadata::unmanaged(),
        )
    }
}

/// Stable conformance scenario identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DurableStateConformanceScenario {
    /// Backend health is available.
    BackendHealth,
    /// Events append contiguously and read in sequence order.
    OrderedEventAppend,
    /// An event with mismatched immutable run identity fails closed.
    ImmutableRunIdentityMismatchRejected,
    /// Duplicate event IDs fail closed.
    DuplicateEventIdRejected,
    /// Duplicate event sequence numbers fail closed.
    DuplicateEventSequenceRejected,
    /// Non-contiguous event sequence numbers fail closed.
    NonContiguousEventSequenceRejected,
    /// Duplicate idempotency keys replay the first result.
    IdempotencyFirstWriteReplay,
    /// Local lock contention is explicit and release permits reacquisition.
    LockContentionAndRelease,
    /// Managed adapter-schema metadata is present and ready.
    ManagedSchemaReady,
    /// One Core-owned transaction family is explicitly supported or unsupported.
    TransactionFamily {
        /// Transaction family being assessed.
        kind: DurableStateTransactionKind,
    },
    /// One advanced backend capability is explicitly unsupported by this harness
    /// or requires a future executable scenario before it may be claimed.
    AdvancedCapability {
        /// Capability being assessed.
        capability: DurableStateCapability,
    },
}

/// Outcome of one executable conformance scenario.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DurableStateConformanceOutcome {
    /// The scenario passed.
    Passed,
    /// The backend explicitly declares that this scenario is unsupported.
    Unsupported,
}

/// One payload-free conformance result.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DurableStateConformanceResult {
    scenario: DurableStateConformanceScenario,
    outcome: DurableStateConformanceOutcome,
}

impl DurableStateConformanceResult {
    /// Returns the scenario identifier.
    #[must_use]
    pub const fn scenario(self) -> DurableStateConformanceScenario {
        self.scenario
    }

    /// Returns the outcome.
    #[must_use]
    pub const fn outcome(self) -> DurableStateConformanceOutcome {
        self.outcome
    }
}

/// Prepared bounded fixture for the executable common conformance harness.
///
/// The backend supplied to the runner must be disposable and unused.
pub struct DurableStateConformanceFixture {
    created_event: WorkflowRunEvent,
    next_event: WorkflowRunEvent,
    idempotency_key: IdempotencyKey,
    lock_owner: ActorId,
}

impl DurableStateConformanceFixture {
    /// Creates a fixture from two canonical first events.
    ///
    /// # Errors
    ///
    /// Returns a stable error unless the events use sequence numbers one and
    /// two with matching immutable run identity.
    pub fn new(
        created_event: WorkflowRunEvent,
        next_event: WorkflowRunEvent,
        idempotency_key: IdempotencyKey,
        lock_owner: ActorId,
    ) -> Result<Self, WorkflowOsError> {
        if created_event.sequence_number.get() != 1
            || next_event.sequence_number.get() != 2
            || created_event.run_id != next_event.run_id
            || created_event.workflow_id != next_event.workflow_id
            || created_event.schema_version != next_event.schema_version
            || created_event.workflow_version != next_event.workflow_version
            || created_event.spec_content_hash != next_event.spec_content_hash
        {
            return Err(contract_error(
                "conformance_fixture.invalid",
                "durable state conformance fixture is invalid",
            ));
        }
        Ok(Self {
            created_event,
            next_event,
            idempotency_key,
            lock_owner,
        })
    }
}

/// Payload-free report from the executable common conformance harness.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DurableStateConformanceReport {
    contract: DurableStateSemanticContract,
    results: Vec<DurableStateConformanceResult>,
}

impl DurableStateConformanceReport {
    /// Returns the tested backend contract.
    #[must_use]
    pub const fn contract(&self) -> &DurableStateSemanticContract {
        &self.contract
    }

    /// Returns scenario results in stable order.
    #[must_use]
    pub fn results(&self) -> &[DurableStateConformanceResult] {
        &self.results
    }
}

/// Runs the common executable baseline against a fresh disposable backend.
///
/// This harness validates currently applicable event, idempotency, lock, and
/// health behavior. Cross-record transactions, CAS revisions, expiring fenced
/// leases, migrations, backup/restore, crash recovery, and shared-worker
/// concurrency remain explicit contract declarations for later adapter suites.
///
/// # Errors
///
/// Returns a stable non-leaking error when a required scenario fails.
pub fn run_durable_state_conformance(
    backend: &(impl StateBackend + DurableStateContractProvider),
    fixture: &DurableStateConformanceFixture,
) -> Result<DurableStateConformanceReport, WorkflowOsError> {
    let contract = backend
        .durable_state_contract()
        .map_err(|_| conformance_failure())?;
    let mut results = Vec::new();

    let health = backend.health_check().map_err(|_| conformance_failure())?;
    if !health.healthy {
        return Err(conformance_failure());
    }
    pass(&mut results, DurableStateConformanceScenario::BackendHealth);

    run_event_scenarios(backend, fixture, &mut results)?;
    run_idempotency_scenario(backend, fixture, &mut results)?;
    run_lock_scenario(backend, fixture, &mut results)?;
    append_schema_scenario(&contract, &mut results)?;
    append_declared_support_scenarios(&contract, &mut results)?;

    Ok(DurableStateConformanceReport { contract, results })
}

fn run_event_scenarios(
    backend: &impl StateBackend,
    fixture: &DurableStateConformanceFixture,
    results: &mut Vec<DurableStateConformanceResult>,
) -> Result<(), WorkflowOsError> {
    backend
        .append_event(&fixture.created_event)
        .map_err(|_| conformance_failure())?;
    backend
        .append_event(&fixture.next_event)
        .map_err(|_| conformance_failure())?;
    let events = backend
        .read_events(&fixture.created_event.run_id)
        .map_err(|_| conformance_failure())?;
    if events.len() != 2
        || events[0].sequence_number.get() != 1
        || events[1].sequence_number.get() != 2
    {
        return Err(conformance_failure());
    }
    pass(results, DurableStateConformanceScenario::OrderedEventAppend);

    let mut identity_mismatch = fixture.next_event.clone();
    identity_mismatch.sequence_number =
        EventSequenceNumber::new(3).map_err(|_| conformance_failure())?;
    identity_mismatch.event_id = EventId::generate();
    let mismatch_source = format!(
        "durable-state-conformance-mismatch:{}",
        fixture.next_event.spec_content_hash.as_str()
    );
    identity_mismatch.spec_content_hash = SpecContentHash::from_text(&mismatch_source);
    expect_error_code(
        backend.append_event(&identity_mismatch),
        "runtime.identity.mismatch",
    )?;
    pass(
        results,
        DurableStateConformanceScenario::ImmutableRunIdentityMismatchRejected,
    );

    let mut duplicate_id = fixture.next_event.clone();
    duplicate_id.event_id = fixture.created_event.event_id.clone();
    expect_error_code(
        backend.append_event(&duplicate_id),
        "state.event.duplicate_id",
    )?;
    pass(
        results,
        DurableStateConformanceScenario::DuplicateEventIdRejected,
    );

    let mut duplicate_sequence = fixture.next_event.clone();
    duplicate_sequence.event_id = EventId::generate();
    expect_error_code(
        backend.append_event(&duplicate_sequence),
        "state.event.duplicate_sequence",
    )?;
    pass(
        results,
        DurableStateConformanceScenario::DuplicateEventSequenceRejected,
    );

    let mut non_contiguous = fixture.next_event.clone();
    non_contiguous.sequence_number =
        EventSequenceNumber::new(4).map_err(|_| conformance_failure())?;
    non_contiguous.event_id = EventId::generate();
    expect_error_code(
        backend.append_event(&non_contiguous),
        "runtime.sequence.non_contiguous",
    )?;
    pass(
        results,
        DurableStateConformanceScenario::NonContiguousEventSequenceRejected,
    );
    Ok(())
}

fn run_idempotency_scenario(
    backend: &impl StateBackend,
    fixture: &DurableStateConformanceFixture,
    results: &mut Vec<DurableStateConformanceResult>,
) -> Result<(), WorkflowOsError> {
    let first_result = IdempotencyResult {
        result_ref: "conformance-result-first".to_owned(),
    };
    let first = backend
        .record_idempotency_result(&fixture.idempotency_key, first_result.clone())
        .map_err(|_| conformance_failure())?;
    let duplicate = backend
        .record_idempotency_result(
            &fixture.idempotency_key,
            IdempotencyResult {
                result_ref: "conformance-result-second".to_owned(),
            },
        )
        .map_err(|_| conformance_failure())?;
    if first != IdempotencyWrite::FirstWrite(first_result.clone())
        || duplicate != IdempotencyWrite::Duplicate(first_result)
    {
        return Err(conformance_failure());
    }
    pass(
        results,
        DurableStateConformanceScenario::IdempotencyFirstWriteReplay,
    );
    Ok(())
}

fn run_lock_scenario(
    backend: &impl StateBackend,
    fixture: &DurableStateConformanceFixture,
    results: &mut Vec<DurableStateConformanceResult>,
) -> Result<(), WorkflowOsError> {
    let lock_key = format!("conformance/{}", fixture.created_event.run_id.as_str());
    let lease = backend
        .acquire_lock(&lock_key, &fixture.lock_owner)
        .map_err(|_| conformance_failure())?;
    expect_error_code(
        backend.acquire_lock(&lock_key, &fixture.lock_owner),
        "state.lock_contended",
    )?;
    backend
        .release_lock(&lease)
        .map_err(|_| conformance_failure())?;
    let reacquired = backend
        .acquire_lock(&lock_key, &fixture.lock_owner)
        .map_err(|_| conformance_failure())?;
    backend
        .release_lock(&reacquired)
        .map_err(|_| conformance_failure())?;
    pass(
        results,
        DurableStateConformanceScenario::LockContentionAndRelease,
    );
    Ok(())
}

fn append_declared_support_scenarios(
    contract: &DurableStateSemanticContract,
    results: &mut Vec<DurableStateConformanceResult>,
) -> Result<(), WorkflowOsError> {
    for kind in DurableStateTransactionKind::all() {
        match contract.transaction_support(*kind) {
            DurableStateSupport::Unsupported => unsupported(
                results,
                DurableStateConformanceScenario::TransactionFamily { kind: *kind },
            ),
            DurableStateSupport::Supported
                if *kind == DurableStateTransactionKind::AppendRunEvent =>
            {
                pass(
                    results,
                    DurableStateConformanceScenario::TransactionFamily { kind: *kind },
                );
            }
            DurableStateSupport::Supported => return Err(conformance_coverage_missing()),
        }
    }

    for capability in [
        DurableStateCapability::CrossRecordAtomicCommit,
        DurableStateCapability::CompareAndSetRevision,
        DurableStateCapability::ExpiringFencedLease,
        DurableStateCapability::ManagedSchemaMigration,
        DurableStateCapability::VerifiedBackupRestore,
        DurableStateCapability::SharedWorkerConcurrency,
    ] {
        if contract.supports_capability(capability) {
            return Err(conformance_coverage_missing());
        }
        unsupported(
            results,
            DurableStateConformanceScenario::AdvancedCapability { capability },
        );
    }
    Ok(())
}

fn append_schema_scenario(
    contract: &DurableStateSemanticContract,
    results: &mut Vec<DurableStateConformanceResult>,
) -> Result<(), WorkflowOsError> {
    match (
        contract.schema().adapter_schema_version(),
        contract.schema().posture(),
    ) {
        (None, DurableStateSchemaPosture::NotManaged) => {
            unsupported(results, DurableStateConformanceScenario::ManagedSchemaReady);
        }
        (Some(_), DurableStateSchemaPosture::Ready) => {
            pass(results, DurableStateConformanceScenario::ManagedSchemaReady);
        }
        _ => return Err(conformance_failure()),
    }
    Ok(())
}

fn pass(
    results: &mut Vec<DurableStateConformanceResult>,
    scenario: DurableStateConformanceScenario,
) {
    results.push(DurableStateConformanceResult {
        scenario,
        outcome: DurableStateConformanceOutcome::Passed,
    });
}

fn unsupported(
    results: &mut Vec<DurableStateConformanceResult>,
    scenario: DurableStateConformanceScenario,
) {
    results.push(DurableStateConformanceResult {
        scenario,
        outcome: DurableStateConformanceOutcome::Unsupported,
    });
}

fn expect_error_code<T>(
    result: Result<T, WorkflowOsError>,
    expected_code: &str,
) -> Result<(), WorkflowOsError> {
    match result {
        Err(error) if error.code() == expected_code => Ok(()),
        _ => Err(conformance_failure()),
    }
}

fn conformance_failure() -> WorkflowOsError {
    contract_error(
        "conformance.failed",
        "durable state backend conformance scenario failed",
    )
}

fn conformance_coverage_missing() -> WorkflowOsError {
    contract_error(
        "conformance.coverage_missing",
        "durable state backend claims a guarantee without an executable conformance scenario",
    )
}

fn contract_error(suffix: &str, message: &str) -> WorkflowOsError {
    WorkflowOsError::new(
        WorkflowOsErrorKind::Validation,
        format!("durable_state.contract.{suffix}"),
        message,
    )
}
