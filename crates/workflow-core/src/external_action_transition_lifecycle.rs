use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use serde::{Deserialize, Serialize};

use crate::{
    state::with_local_state_shared_writer_guard, ActorId, AuthorizedExecutionAttemptId,
    AuthorizedExecutionResourceReference, AuthorizedExecutionWaitCondition,
    AuthorizedExecutionWaitConditionId, AuthorizedExecutionWaitStatus,
    AuthorizedExecutionWakeTriggerKind, AuthorizedExecutionWindowId, EventId, EventSequenceNumber,
    ExternalActionAuthorizationProjection, ExternalActionAuthorizationProjectionId,
    ExternalActionTransition, ExternalActionTransitionKind, SpecContentHash, Timestamp,
    WorkflowOsError, WorkflowRunId,
};

const MODEL_DOMAIN: &str = "workflow-os.external-action-transition-lifecycle.v1";
const OPERATIONS_DIR: &str = "external-action-transition-lifecycle";
const DEFINITION_FILE: &str = "definition.json";
const EVENTS_DIR: &str = "events";
static TEMP_FILE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// Stable identity of one projected external transition operation.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalActionTransitionOperationId(String);

impl ExternalActionTransitionOperationId {
    fn from_commitment(commitment: &SpecContentHash) -> Self {
        Self(format!("external-action-operation/{}", commitment.as_str()))
    }

    /// Returns the stable operation identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Debug for ExternalActionTransitionOperationId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_tuple("ExternalActionTransitionOperationId")
            .field(&"[REDACTED]")
            .finish()
    }
}

/// Immutable operation definition derived from an authorization projection.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct ExternalActionTransitionOperation {
    operation_id: ExternalActionTransitionOperationId,
    operation_definition_commitment: SpecContentHash,
    projection_id: ExternalActionAuthorizationProjectionId,
    projection_commitment: SpecContentHash,
    run_id: WorkflowRunId,
    execution_window_id: AuthorizedExecutionWindowId,
    actor: ActorId,
    transition_kind: ExternalActionTransitionKind,
    transition_operation_commitment: SpecContentHash,
    transition_payload_commitment: SpecContentHash,
    expires_at: Timestamp,
}

#[derive(Serialize)]
struct OperationCommitmentInput<'a> {
    domain: &'static str,
    projection_id: &'a ExternalActionAuthorizationProjectionId,
    projection_commitment: &'a SpecContentHash,
    run_id: &'a WorkflowRunId,
    execution_window_id: &'a AuthorizedExecutionWindowId,
    actor: &'a ActorId,
    transition_kind: ExternalActionTransitionKind,
    transition_operation_commitment: &'a SpecContentHash,
    transition_payload_commitment: &'a SpecContentHash,
    expires_at: Timestamp,
}

impl ExternalActionTransitionOperation {
    /// Derives one immutable operation from an exact projected transition.
    ///
    /// # Errors
    ///
    /// Returns an error when the requested transition is not projected or the
    /// deterministic operation commitment cannot be encoded.
    pub fn from_projection(
        projection: &ExternalActionAuthorizationProjection,
        transition_kind: ExternalActionTransitionKind,
    ) -> Result<Self, WorkflowOsError> {
        let transition = projection.transition(transition_kind).ok_or_else(|| {
            lifecycle_error(
                "operation.transition_not_projected",
                "external action transition is not present in the authorization projection",
            )
        })?;
        Self::derive(projection, transition)
    }

    fn derive(
        projection: &ExternalActionAuthorizationProjection,
        transition: &ExternalActionTransition,
    ) -> Result<Self, WorkflowOsError> {
        let input = OperationCommitmentInput {
            domain: MODEL_DOMAIN,
            projection_id: projection.projection_id(),
            projection_commitment: projection.projection_commitment(),
            run_id: projection.run_id(),
            execution_window_id: projection.execution_window_id(),
            actor: projection.actor(),
            transition_kind: transition.kind(),
            transition_operation_commitment: transition.operation_commitment(),
            transition_payload_commitment: transition.payload_commitment(),
            expires_at: projection.expires_at(),
        };
        let bytes = serde_json::to_vec(&input).map_err(|_| {
            lifecycle_error(
                "operation.commitment_failed",
                "external action transition operation commitment failed",
            )
        })?;
        let operation_definition_commitment = SpecContentHash::from_bytes(bytes);
        Ok(Self {
            operation_id: ExternalActionTransitionOperationId::from_commitment(
                &operation_definition_commitment,
            ),
            operation_definition_commitment,
            projection_id: projection.projection_id().clone(),
            projection_commitment: projection.projection_commitment().clone(),
            run_id: projection.run_id().clone(),
            execution_window_id: projection.execution_window_id().clone(),
            actor: projection.actor().clone(),
            transition_kind: transition.kind(),
            transition_operation_commitment: transition.operation_commitment().clone(),
            transition_payload_commitment: transition.payload_commitment().clone(),
            expires_at: projection.expires_at(),
        })
    }

    /// Returns the stable operation identity.
    #[must_use]
    pub const fn operation_id(&self) -> &ExternalActionTransitionOperationId {
        &self.operation_id
    }

    /// Returns the immutable operation-definition commitment.
    #[must_use]
    pub const fn operation_definition_commitment(&self) -> &SpecContentHash {
        &self.operation_definition_commitment
    }

    /// Returns the projected transition kind.
    #[must_use]
    pub const fn transition_kind(&self) -> ExternalActionTransitionKind {
        self.transition_kind
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        let input = OperationCommitmentInput {
            domain: MODEL_DOMAIN,
            projection_id: &self.projection_id,
            projection_commitment: &self.projection_commitment,
            run_id: &self.run_id,
            execution_window_id: &self.execution_window_id,
            actor: &self.actor,
            transition_kind: self.transition_kind,
            transition_operation_commitment: &self.transition_operation_commitment,
            transition_payload_commitment: &self.transition_payload_commitment,
            expires_at: self.expires_at,
        };
        let bytes = serde_json::to_vec(&input).map_err(|_| {
            lifecycle_error(
                "operation.commitment_failed",
                "external action transition operation commitment failed",
            )
        })?;
        let commitment = SpecContentHash::from_bytes(bytes);
        if commitment != self.operation_definition_commitment
            || ExternalActionTransitionOperationId::from_commitment(&commitment)
                != self.operation_id
        {
            return Err(lifecycle_error(
                "operation.commitment_mismatch",
                "external action transition operation commitment is invalid",
            ));
        }
        Ok(())
    }
}

impl std::fmt::Debug for ExternalActionTransitionOperation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ExternalActionTransitionOperation")
            .field("transition_kind", &self.transition_kind)
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

impl<'de> Deserialize<'de> for ExternalActionTransitionOperation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Wire {
            operation_id: ExternalActionTransitionOperationId,
            operation_definition_commitment: SpecContentHash,
            projection_id: ExternalActionAuthorizationProjectionId,
            projection_commitment: SpecContentHash,
            run_id: WorkflowRunId,
            execution_window_id: AuthorizedExecutionWindowId,
            actor: ActorId,
            transition_kind: ExternalActionTransitionKind,
            transition_operation_commitment: SpecContentHash,
            transition_payload_commitment: SpecContentHash,
            expires_at: Timestamp,
        }
        let wire = Wire::deserialize(deserializer)?;
        let operation = Self {
            operation_id: wire.operation_id,
            operation_definition_commitment: wire.operation_definition_commitment,
            projection_id: wire.projection_id,
            projection_commitment: wire.projection_commitment,
            run_id: wire.run_id,
            execution_window_id: wire.execution_window_id,
            actor: wire.actor,
            transition_kind: wire.transition_kind,
            transition_operation_commitment: wire.transition_operation_commitment,
            transition_payload_commitment: wire.transition_payload_commitment,
            expires_at: wire.expires_at,
        };
        operation.validate().map_err(|_| {
            serde::de::Error::custom("external action transition operation is invalid")
        })?;
        Ok(operation)
    }
}

/// Current durable lifecycle posture of one projected transition.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalActionTransitionLifecycleStatus {
    Projected,
    Reserved,
    Admitted,
    Attempted,
    OutcomeSucceeded,
    OutcomeFailed,
    OutcomeAmbiguous,
    ReconciledSucceeded,
    ReconciledFailed,
    Waiting,
    Denied,
    Consumed,
    Revoked,
}

/// Bounded outcome observed or established by reconciliation.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalActionTransitionLifecycleOutcome {
    Succeeded,
    Failed,
    Ambiguous,
}

/// Typed refusal class for durable connector-admission and continuity reporting.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalActionTransitionLifecycleRefusalKind {
    AuthorityDenied,
    EvidenceIncomplete,
    PlatformPermissionMissing,
    ConnectorUnavailable,
    RateLimited,
    ExternalStateConflict,
    OutOfScope,
}

/// Durable lifecycle event kind.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalActionTransitionLifecycleEventKind {
    Projected,
    Reserved,
    Admitted,
    Attempted,
    OutcomeObserved,
    OutcomeReconciled,
    WaitRegistered,
    WaitResolved,
    Denied,
    Consumed,
    Revoked,
}

/// Payload-free proof that one exact durable wait was reassessed against a
/// named source observation.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExternalActionWaitResolutionProof {
    condition_id: AuthorizedExecutionWaitConditionId,
    condition_version: u32,
    expected_sequence_number: EventSequenceNumber,
    expected_event_id: EventId,
    required_reference: AuthorizedExecutionResourceReference,
    wake_trigger: AuthorizedExecutionWakeTriggerKind,
    source_observation_commitment: SpecContentHash,
    status: AuthorizedExecutionWaitStatus,
    resolved_at: Timestamp,
    proof_commitment: SpecContentHash,
}

impl ExternalActionWaitResolutionProof {
    pub(crate) fn new(
        wait: &AuthorizedExecutionWaitCondition,
        source_observation_commitment: SpecContentHash,
        status: AuthorizedExecutionWaitStatus,
        resolved_at: Timestamp,
    ) -> Result<Self, WorkflowOsError> {
        if status == AuthorizedExecutionWaitStatus::Waiting {
            return Err(lifecycle_error(
                "wait_resolution.status_invalid",
                "external action wait resolution must be terminal",
            ));
        }
        if status == AuthorizedExecutionWaitStatus::Satisfied
            && wait
                .deadline()
                .is_some_and(|deadline| resolved_at > deadline)
        {
            return Err(lifecycle_error(
                "wait_resolution.deadline_expired",
                "external action wait cannot be satisfied after its deadline",
            ));
        }
        if status == AuthorizedExecutionWaitStatus::Expired
            && wait
                .deadline()
                .map_or(true, |deadline| resolved_at < deadline)
        {
            return Err(lifecycle_error(
                "wait_resolution.expiry_invalid",
                "external action wait cannot expire before its deadline",
            ));
        }
        let proof_commitment = wait_resolution_proof_commitment(
            wait,
            &source_observation_commitment,
            status,
            resolved_at,
        )?;
        Ok(Self {
            condition_id: wait.condition_id().clone(),
            condition_version: wait.condition_version(),
            expected_sequence_number: *wait.expected_sequence_number(),
            expected_event_id: wait.expected_event_id().clone(),
            required_reference: wait.required_reference().clone(),
            wake_trigger: wait.wake_trigger(),
            source_observation_commitment,
            status,
            resolved_at,
            proof_commitment,
        })
    }

    fn validate(
        &self,
        wait: &AuthorizedExecutionWaitCondition,
        recorded_at: Timestamp,
    ) -> Result<(), WorkflowOsError> {
        let expected = Self::new(
            wait,
            self.source_observation_commitment.clone(),
            self.status,
            recorded_at,
        )?;
        if self != &expected || self.resolved_at != recorded_at {
            return Err(lifecycle_error(
                "wait_resolution.proof_mismatch",
                "external action wait resolution proof does not match the active condition",
            ));
        }
        Ok(())
    }

    pub(crate) const fn condition_id(&self) -> &AuthorizedExecutionWaitConditionId {
        &self.condition_id
    }

    pub(crate) const fn status(&self) -> AuthorizedExecutionWaitStatus {
        self.status
    }

    pub(crate) const fn proof_commitment(&self) -> &SpecContentHash {
        &self.proof_commitment
    }
}

fn wait_resolution_proof_commitment(
    wait: &AuthorizedExecutionWaitCondition,
    source_observation_commitment: &SpecContentHash,
    status: AuthorizedExecutionWaitStatus,
    resolved_at: Timestamp,
) -> Result<SpecContentHash, WorkflowOsError> {
    #[derive(Serialize)]
    struct Input<'a> {
        domain: &'static str,
        condition_id: &'a AuthorizedExecutionWaitConditionId,
        condition_version: u32,
        expected_sequence_number: &'a EventSequenceNumber,
        expected_event_id: &'a EventId,
        required_reference: &'a AuthorizedExecutionResourceReference,
        wake_trigger: AuthorizedExecutionWakeTriggerKind,
        source_observation_commitment: &'a SpecContentHash,
        status: AuthorizedExecutionWaitStatus,
        resolved_at: Timestamp,
    }
    Ok(SpecContentHash::from_bytes(canonical_bytes(
        &Input {
            domain: "workflow-os.external-action-wait-resolution.v1",
            condition_id: wait.condition_id(),
            condition_version: wait.condition_version(),
            expected_sequence_number: wait.expected_sequence_number(),
            expected_event_id: wait.expected_event_id(),
            required_reference: wait.required_reference(),
            wake_trigger: wait.wake_trigger(),
            source_observation_commitment,
            status,
            resolved_at,
        },
        "wait resolution proof commitment failed",
    )?))
}

/// One explicit lifecycle transition command. All commitments are payload-free.
#[allow(missing_docs)]
#[derive(Clone, Eq, PartialEq)]
pub enum ExternalActionTransitionLifecycleCommand {
    Reserve {
        attempt_id: AuthorizedExecutionAttemptId,
    },
    Admit {
        attempt_id: AuthorizedExecutionAttemptId,
        admission_commitment: SpecContentHash,
    },
    MarkAttempted {
        attempt_id: AuthorizedExecutionAttemptId,
        attempt_commitment: SpecContentHash,
    },
    ObserveOutcome {
        attempt_id: AuthorizedExecutionAttemptId,
        outcome: ExternalActionTransitionLifecycleOutcome,
        observation_commitment: SpecContentHash,
    },
    ReconcileOutcome {
        attempt_id: AuthorizedExecutionAttemptId,
        outcome: ExternalActionTransitionLifecycleOutcome,
        reconciliation_commitment: SpecContentHash,
    },
    RegisterWait {
        wait_condition: Box<AuthorizedExecutionWaitCondition>,
    },
    ResolveWait {
        proof: ExternalActionWaitResolutionProof,
    },
    Deny {
        refusal: ExternalActionTransitionLifecycleRefusalKind,
        decision_commitment: SpecContentHash,
    },
    Consume {
        attempt_id: AuthorizedExecutionAttemptId,
        consumption_commitment: SpecContentHash,
    },
    Revoke {
        revocation_commitment: SpecContentHash,
    },
}

/// Optimistic append cursor binding a sequence to the exact current head.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExternalActionTransitionLifecycleCursor {
    sequence: u64,
    head_commitment: SpecContentHash,
}

impl ExternalActionTransitionLifecycleCursor {
    /// Returns the current event sequence.
    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    /// Returns the exact current event commitment.
    #[must_use]
    pub const fn head_commitment(&self) -> &SpecContentHash {
        &self.head_commitment
    }
}

/// One immutable, commitment-chained lifecycle event.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExternalActionTransitionLifecycleEvent {
    sequence: u64,
    event_kind: ExternalActionTransitionLifecycleEventKind,
    prior_status: Option<ExternalActionTransitionLifecycleStatus>,
    resulting_status: ExternalActionTransitionLifecycleStatus,
    operation_id: ExternalActionTransitionOperationId,
    operation_definition_commitment: SpecContentHash,
    previous_event_commitment: Option<SpecContentHash>,
    attempt_id: Option<AuthorizedExecutionAttemptId>,
    detail_commitment: SpecContentHash,
    outcome: Option<ExternalActionTransitionLifecycleOutcome>,
    refusal: Option<ExternalActionTransitionLifecycleRefusalKind>,
    wait_condition: Option<AuthorizedExecutionWaitCondition>,
    wait_resolution_proof: Option<ExternalActionWaitResolutionProof>,
    recorded_at: Timestamp,
    event_commitment: SpecContentHash,
}

impl ExternalActionTransitionLifecycleEvent {
    /// Returns the event sequence.
    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    /// Returns the event kind.
    #[must_use]
    pub const fn event_kind(&self) -> ExternalActionTransitionLifecycleEventKind {
        self.event_kind
    }

    /// Returns the resulting status.
    #[must_use]
    pub const fn resulting_status(&self) -> ExternalActionTransitionLifecycleStatus {
        self.resulting_status
    }

    /// Returns the complete event commitment.
    #[must_use]
    pub const fn event_commitment(&self) -> &SpecContentHash {
        &self.event_commitment
    }
}

/// Rehydrated durable lifecycle snapshot.
#[derive(Clone, Eq, PartialEq)]
pub struct ExternalActionTransitionLifecycleSnapshot {
    operation: ExternalActionTransitionOperation,
    status: ExternalActionTransitionLifecycleStatus,
    active_attempt_id: Option<AuthorizedExecutionAttemptId>,
    suspended_status: Option<ExternalActionTransitionLifecycleStatus>,
    active_wait: Option<AuthorizedExecutionWaitCondition>,
    events: Vec<ExternalActionTransitionLifecycleEvent>,
    cursor: ExternalActionTransitionLifecycleCursor,
}

impl ExternalActionTransitionLifecycleSnapshot {
    /// Returns the immutable operation definition.
    #[must_use]
    pub const fn operation(&self) -> &ExternalActionTransitionOperation {
        &self.operation
    }

    /// Returns the current lifecycle status.
    #[must_use]
    pub const fn status(&self) -> ExternalActionTransitionLifecycleStatus {
        self.status
    }

    /// Returns the optimistic append cursor.
    #[must_use]
    pub const fn cursor(&self) -> &ExternalActionTransitionLifecycleCursor {
        &self.cursor
    }

    /// Returns the complete immutable event history.
    #[must_use]
    pub fn events(&self) -> &[ExternalActionTransitionLifecycleEvent] {
        &self.events
    }

    /// Returns the active attempt bound to the current lifecycle posture.
    #[must_use]
    pub(crate) const fn active_attempt_id(&self) -> Option<&AuthorizedExecutionAttemptId> {
        self.active_attempt_id.as_ref()
    }

    /// Returns the active durable wait when the transition is suspended.
    #[must_use]
    pub(crate) const fn active_wait(&self) -> Option<&AuthorizedExecutionWaitCondition> {
        self.active_wait.as_ref()
    }

    /// Returns the attempt from a structurally verified satisfied-wait event.
    #[must_use]
    pub(crate) fn verified_wait_resume_attempt_id(&self) -> Option<&AuthorizedExecutionAttemptId> {
        let event = self.events.last()?;
        let proof = event.wait_resolution_proof.as_ref()?;
        (self.status == ExternalActionTransitionLifecycleStatus::Reserved
            && proof.status() == AuthorizedExecutionWaitStatus::Satisfied
            && event.event_kind == ExternalActionTransitionLifecycleEventKind::WaitResolved)
            .then_some(event.attempt_id.as_ref())
            .flatten()
    }
}

/// Result of exact-idempotent operation initialization.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExternalActionTransitionLifecycleInitializationOutcome {
    Initialized,
    AlreadyInitialized,
}

/// Result of an optimistic lifecycle append.
#[allow(missing_docs)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExternalActionTransitionLifecycleAppendOutcome {
    Appended(ExternalActionTransitionLifecycleCursor),
    ExactReplay(ExternalActionTransitionLifecycleCursor),
}

/// Local append-only store for projected external transition lifecycles.
#[derive(Clone, Eq, PartialEq)]
pub struct LocalExternalActionTransitionLifecycleStore {
    state_root: PathBuf,
}

impl LocalExternalActionTransitionLifecycleStore {
    /// Creates a lifecycle store rooted in one local Workflow OS state root.
    #[must_use]
    pub fn new(state_root: impl Into<PathBuf>) -> Self {
        Self {
            state_root: state_root.into(),
        }
    }

    /// Initializes one operation using create-only exact-idempotent semantics.
    ///
    /// # Errors
    ///
    /// Fails closed for conflicting definitions, corrupt state, or persistence errors.
    pub fn initialize(
        &self,
        operation: &ExternalActionTransitionOperation,
        recorded_at: Timestamp,
    ) -> Result<ExternalActionTransitionLifecycleInitializationOutcome, WorkflowOsError> {
        operation.validate()?;
        with_local_state_shared_writer_guard(&self.state_root, || {
            let operation_dir = self.operation_dir(operation.operation_id());
            fs::create_dir_all(operation_dir.join(EVENTS_DIR)).map_err(|_| {
                lifecycle_error(
                    "store.create_failed",
                    "external action lifecycle store write failed",
                )
            })?;
            let definition_path = operation_dir.join(DEFINITION_FILE);
            let definition_bytes = canonical_bytes(operation, "operation serialization failed")?;
            let existed = definition_path.exists();
            if existed {
                let existing = read_bytes(&definition_path)?;
                if existing != definition_bytes {
                    return Err(lifecycle_error(
                        "store.definition_conflict",
                        "external action lifecycle operation has conflicting content",
                    ));
                }
                let parsed: ExternalActionTransitionOperation = parse_bytes(&existing)?;
                parsed.validate()?;
            } else {
                write_create_only(&definition_path, &definition_bytes)?;
            }
            let projected = build_event(
                operation,
                1,
                ExternalActionTransitionLifecycleEventKind::Projected,
                None,
                ExternalActionTransitionLifecycleStatus::Projected,
                None,
                None,
                operation.operation_definition_commitment().clone(),
                None,
                None,
                None,
                None,
                recorded_at,
            )?;
            let event_path = self.event_path(operation.operation_id(), 1);
            let event_bytes = canonical_bytes(&projected, "event serialization failed")?;
            if event_path.exists() {
                if read_bytes(&event_path)? != event_bytes {
                    return Err(lifecycle_error(
                        "store.initial_event_conflict",
                        "external action lifecycle initial event conflicts with durable state",
                    ));
                }
            } else {
                write_create_only(&event_path, &event_bytes)?;
            }
            self.read_snapshot_locked(operation.operation_id())?;
            Ok(if existed {
                ExternalActionTransitionLifecycleInitializationOutcome::AlreadyInitialized
            } else {
                ExternalActionTransitionLifecycleInitializationOutcome::Initialized
            })
        })
    }

    /// Appends one legal transition against an exact durable cursor.
    ///
    /// # Errors
    ///
    /// Fails closed for stale cursors, illegal transitions, corrupt state, or
    /// persistence errors. An exact replay returns without duplicating an event.
    pub fn append(
        &self,
        operation_id: &ExternalActionTransitionOperationId,
        expected: &ExternalActionTransitionLifecycleCursor,
        command: &ExternalActionTransitionLifecycleCommand,
        recorded_at: Timestamp,
    ) -> Result<ExternalActionTransitionLifecycleAppendOutcome, WorkflowOsError> {
        with_local_state_shared_writer_guard(&self.state_root, || {
            let snapshot = self.read_snapshot_locked(operation_id)?;
            if &snapshot.cursor != expected {
                if snapshot.cursor.sequence == expected.sequence + 1 {
                    let prior = rehydrate(
                        snapshot.operation.clone(),
                        snapshot.events[..snapshot.events.len() - 1].to_vec(),
                    )?;
                    if &prior.cursor == expected {
                        let candidate = build_next_event(&prior, command, recorded_at)?;
                        if snapshot.events.last() == Some(&candidate) {
                            return Ok(
                                ExternalActionTransitionLifecycleAppendOutcome::ExactReplay(
                                    snapshot.cursor,
                                ),
                            );
                        }
                    }
                }
                return Err(lifecycle_error(
                    "append.stale_cursor",
                    "external action lifecycle append cursor is stale",
                ));
            }
            let event = build_next_event(&snapshot, command, recorded_at)?;
            let path = self.event_path(operation_id, event.sequence);
            write_create_only(
                &path,
                &canonical_bytes(&event, "event serialization failed")?,
            )?;
            Ok(ExternalActionTransitionLifecycleAppendOutcome::Appended(
                ExternalActionTransitionLifecycleCursor {
                    sequence: event.sequence,
                    head_commitment: event.event_commitment.clone(),
                },
            ))
        })
    }

    /// Reads and validates one complete durable lifecycle.
    ///
    /// # Errors
    ///
    /// Fails closed when the definition, event chain, or state machine is corrupt.
    pub fn read_snapshot(
        &self,
        operation_id: &ExternalActionTransitionOperationId,
    ) -> Result<ExternalActionTransitionLifecycleSnapshot, WorkflowOsError> {
        self.read_snapshot_locked(operation_id)
    }

    fn operation_dir(&self, operation_id: &ExternalActionTransitionOperationId) -> PathBuf {
        self.state_root
            .join(OPERATIONS_DIR)
            .join(SpecContentHash::from_text(operation_id.as_str()).as_str())
    }

    fn event_path(
        &self,
        operation_id: &ExternalActionTransitionOperationId,
        sequence: u64,
    ) -> PathBuf {
        self.operation_dir(operation_id)
            .join(EVENTS_DIR)
            .join(format!("{sequence:020}.json"))
    }

    fn read_snapshot_locked(
        &self,
        operation_id: &ExternalActionTransitionOperationId,
    ) -> Result<ExternalActionTransitionLifecycleSnapshot, WorkflowOsError> {
        let operation_path = self.operation_dir(operation_id).join(DEFINITION_FILE);
        let operation: ExternalActionTransitionOperation =
            parse_bytes(&read_bytes(&operation_path)?)?;
        operation.validate()?;
        if operation.operation_id() != operation_id {
            return Err(lifecycle_error(
                "store.address_mismatch",
                "external action lifecycle operation does not match its storage address",
            ));
        }
        let events_dir = self.operation_dir(operation_id).join(EVENTS_DIR);
        let mut entries = fs::read_dir(events_dir)
            .map_err(|_| {
                lifecycle_error(
                    "store.read_failed",
                    "external action lifecycle store read failed",
                )
            })?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| {
                lifecycle_error(
                    "store.read_failed",
                    "external action lifecycle store read failed",
                )
            })?;
        entries.retain(|entry| !entry.file_name().to_string_lossy().starts_with(".tmp-"));
        entries.sort_by_key(std::fs::DirEntry::file_name);
        if entries.is_empty() {
            return Err(lifecycle_error(
                "store.history_empty",
                "external action lifecycle history is empty",
            ));
        }
        let mut events = Vec::with_capacity(entries.len());
        for entry in entries {
            if !entry
                .file_type()
                .map_err(|_| {
                    lifecycle_error(
                        "store.read_failed",
                        "external action lifecycle store read failed",
                    )
                })?
                .is_file()
            {
                return Err(lifecycle_error(
                    "store.unexpected_entry",
                    "external action lifecycle store contains an unexpected entry",
                ));
            }
            events.push(parse_bytes(&read_bytes(&entry.path())?)?);
        }
        rehydrate(operation, events)
    }
}

#[allow(clippy::too_many_lines)]
fn build_next_event(
    snapshot: &ExternalActionTransitionLifecycleSnapshot,
    command: &ExternalActionTransitionLifecycleCommand,
    recorded_at: Timestamp,
) -> Result<ExternalActionTransitionLifecycleEvent, WorkflowOsError> {
    let sequence = snapshot.cursor.sequence + 1;
    let previous = Some(snapshot.cursor.head_commitment.clone());
    let prior = Some(snapshot.status);
    let operation = &snapshot.operation;
    let (kind, resulting, attempt, detail, outcome, refusal, wait, wait_resolution_proof) =
        match command {
            ExternalActionTransitionLifecycleCommand::Reserve { attempt_id } => {
                if !matches!(
                    snapshot.status,
                    ExternalActionTransitionLifecycleStatus::Projected
                        | ExternalActionTransitionLifecycleStatus::OutcomeFailed
                        | ExternalActionTransitionLifecycleStatus::ReconciledFailed
                ) {
                    return Err(illegal_transition());
                }
                (
                    ExternalActionTransitionLifecycleEventKind::Reserved,
                    ExternalActionTransitionLifecycleStatus::Reserved,
                    Some(attempt_id.clone()),
                    SpecContentHash::from_text(attempt_id.as_str()),
                    None,
                    None,
                    None,
                    None,
                )
            }
            ExternalActionTransitionLifecycleCommand::Admit {
                attempt_id,
                admission_commitment,
            } => {
                require_attempt(
                    snapshot,
                    attempt_id,
                    ExternalActionTransitionLifecycleStatus::Reserved,
                )?;
                (
                    ExternalActionTransitionLifecycleEventKind::Admitted,
                    ExternalActionTransitionLifecycleStatus::Admitted,
                    Some(attempt_id.clone()),
                    admission_commitment.clone(),
                    None,
                    None,
                    None,
                    None,
                )
            }
            ExternalActionTransitionLifecycleCommand::MarkAttempted {
                attempt_id,
                attempt_commitment,
            } => {
                require_attempt(
                    snapshot,
                    attempt_id,
                    ExternalActionTransitionLifecycleStatus::Admitted,
                )?;
                (
                    ExternalActionTransitionLifecycleEventKind::Attempted,
                    ExternalActionTransitionLifecycleStatus::Attempted,
                    Some(attempt_id.clone()),
                    attempt_commitment.clone(),
                    None,
                    None,
                    None,
                    None,
                )
            }
            ExternalActionTransitionLifecycleCommand::ObserveOutcome {
                attempt_id,
                outcome,
                observation_commitment,
            } => {
                require_attempt(
                    snapshot,
                    attempt_id,
                    ExternalActionTransitionLifecycleStatus::Attempted,
                )?;
                let status = outcome_status(*outcome, false)?;
                (
                    ExternalActionTransitionLifecycleEventKind::OutcomeObserved,
                    status,
                    Some(attempt_id.clone()),
                    observation_commitment.clone(),
                    Some(*outcome),
                    None,
                    None,
                    None,
                )
            }
            ExternalActionTransitionLifecycleCommand::ReconcileOutcome {
                attempt_id,
                outcome,
                reconciliation_commitment,
            } => {
                require_attempt(
                    snapshot,
                    attempt_id,
                    ExternalActionTransitionLifecycleStatus::OutcomeAmbiguous,
                )?;
                let status = outcome_status(*outcome, true)?;
                (
                    ExternalActionTransitionLifecycleEventKind::OutcomeReconciled,
                    status,
                    Some(attempt_id.clone()),
                    reconciliation_commitment.clone(),
                    Some(*outcome),
                    None,
                    None,
                    None,
                )
            }
            ExternalActionTransitionLifecycleCommand::RegisterWait { wait_condition } => {
                if matches!(
                    snapshot.status,
                    ExternalActionTransitionLifecycleStatus::Denied
                        | ExternalActionTransitionLifecycleStatus::Consumed
                        | ExternalActionTransitionLifecycleStatus::Revoked
                        | ExternalActionTransitionLifecycleStatus::Waiting
                ) {
                    return Err(illegal_transition());
                }
                validate_wait(operation, snapshot, wait_condition)?;
                (
                    ExternalActionTransitionLifecycleEventKind::WaitRegistered,
                    ExternalActionTransitionLifecycleStatus::Waiting,
                    Some(wait_condition.attempt_id().clone()),
                    SpecContentHash::from_text(wait_condition.condition_id().as_str()),
                    None,
                    None,
                    Some((**wait_condition).clone()),
                    None,
                )
            }
            ExternalActionTransitionLifecycleCommand::ResolveWait { proof } => {
                let active = snapshot
                    .active_wait
                    .as_ref()
                    .ok_or_else(illegal_transition)?;
                if snapshot.status != ExternalActionTransitionLifecycleStatus::Waiting
                    || active.condition_id() != proof.condition_id()
                {
                    return Err(illegal_transition());
                }
                proof.validate(active, recorded_at)?;
                let resulting = if proof.status() == AuthorizedExecutionWaitStatus::Satisfied {
                    snapshot.suspended_status.ok_or_else(illegal_transition)?
                } else {
                    ExternalActionTransitionLifecycleStatus::Denied
                };
                (
                    ExternalActionTransitionLifecycleEventKind::WaitResolved,
                    resulting,
                    Some(active.attempt_id().clone()),
                    proof.proof_commitment().clone(),
                    None,
                    None,
                    None,
                    Some(proof.clone()),
                )
            }
            ExternalActionTransitionLifecycleCommand::Deny {
                refusal,
                decision_commitment,
            } => {
                if matches!(
                    snapshot.status,
                    ExternalActionTransitionLifecycleStatus::Attempted
                        | ExternalActionTransitionLifecycleStatus::OutcomeSucceeded
                        | ExternalActionTransitionLifecycleStatus::OutcomeFailed
                        | ExternalActionTransitionLifecycleStatus::OutcomeAmbiguous
                        | ExternalActionTransitionLifecycleStatus::ReconciledSucceeded
                        | ExternalActionTransitionLifecycleStatus::Consumed
                        | ExternalActionTransitionLifecycleStatus::Revoked
                        | ExternalActionTransitionLifecycleStatus::Denied
                ) {
                    return Err(illegal_transition());
                }
                (
                    ExternalActionTransitionLifecycleEventKind::Denied,
                    ExternalActionTransitionLifecycleStatus::Denied,
                    snapshot.active_attempt_id.clone(),
                    decision_commitment.clone(),
                    None,
                    Some(*refusal),
                    None,
                    None,
                )
            }
            ExternalActionTransitionLifecycleCommand::Consume {
                attempt_id,
                consumption_commitment,
            } => {
                require_attempt(
                    snapshot,
                    attempt_id,
                    ExternalActionTransitionLifecycleStatus::ReconciledSucceeded,
                )
                .or_else(|_| {
                    require_attempt(
                        snapshot,
                        attempt_id,
                        ExternalActionTransitionLifecycleStatus::OutcomeSucceeded,
                    )
                })?;
                (
                    ExternalActionTransitionLifecycleEventKind::Consumed,
                    ExternalActionTransitionLifecycleStatus::Consumed,
                    Some(attempt_id.clone()),
                    consumption_commitment.clone(),
                    None,
                    None,
                    None,
                    None,
                )
            }
            ExternalActionTransitionLifecycleCommand::Revoke {
                revocation_commitment,
            } => {
                if matches!(
                    snapshot.status,
                    ExternalActionTransitionLifecycleStatus::Attempted
                        | ExternalActionTransitionLifecycleStatus::OutcomeSucceeded
                        | ExternalActionTransitionLifecycleStatus::OutcomeFailed
                        | ExternalActionTransitionLifecycleStatus::OutcomeAmbiguous
                        | ExternalActionTransitionLifecycleStatus::ReconciledSucceeded
                        | ExternalActionTransitionLifecycleStatus::ReconciledFailed
                        | ExternalActionTransitionLifecycleStatus::Consumed
                        | ExternalActionTransitionLifecycleStatus::Revoked
                        | ExternalActionTransitionLifecycleStatus::Denied
                ) {
                    return Err(illegal_transition());
                }
                (
                    ExternalActionTransitionLifecycleEventKind::Revoked,
                    ExternalActionTransitionLifecycleStatus::Revoked,
                    snapshot.active_attempt_id.clone(),
                    revocation_commitment.clone(),
                    None,
                    None,
                    None,
                    None,
                )
            }
        };
    build_event(
        operation,
        sequence,
        kind,
        prior,
        resulting,
        previous,
        attempt,
        detail,
        outcome,
        refusal,
        wait,
        wait_resolution_proof,
        recorded_at,
    )
}

#[allow(clippy::too_many_arguments)]
fn build_event(
    operation: &ExternalActionTransitionOperation,
    sequence: u64,
    event_kind: ExternalActionTransitionLifecycleEventKind,
    prior_status: Option<ExternalActionTransitionLifecycleStatus>,
    resulting_status: ExternalActionTransitionLifecycleStatus,
    previous_event_commitment: Option<SpecContentHash>,
    attempt_id: Option<AuthorizedExecutionAttemptId>,
    detail_commitment: SpecContentHash,
    outcome: Option<ExternalActionTransitionLifecycleOutcome>,
    refusal: Option<ExternalActionTransitionLifecycleRefusalKind>,
    wait_condition: Option<AuthorizedExecutionWaitCondition>,
    wait_resolution_proof: Option<ExternalActionWaitResolutionProof>,
    recorded_at: Timestamp,
) -> Result<ExternalActionTransitionLifecycleEvent, WorkflowOsError> {
    #[derive(Serialize)]
    struct EventCommitmentInput<'a> {
        domain: &'static str,
        sequence: u64,
        event_kind: ExternalActionTransitionLifecycleEventKind,
        prior_status: Option<ExternalActionTransitionLifecycleStatus>,
        resulting_status: ExternalActionTransitionLifecycleStatus,
        operation_id: &'a ExternalActionTransitionOperationId,
        operation_definition_commitment: &'a SpecContentHash,
        previous_event_commitment: &'a Option<SpecContentHash>,
        attempt_id: &'a Option<AuthorizedExecutionAttemptId>,
        detail_commitment: &'a SpecContentHash,
        outcome: Option<ExternalActionTransitionLifecycleOutcome>,
        refusal: Option<ExternalActionTransitionLifecycleRefusalKind>,
        wait_condition: &'a Option<AuthorizedExecutionWaitCondition>,
        wait_resolution_proof: &'a Option<ExternalActionWaitResolutionProof>,
        recorded_at: Timestamp,
    }
    let mut event = ExternalActionTransitionLifecycleEvent {
        sequence,
        event_kind,
        prior_status,
        resulting_status,
        operation_id: operation.operation_id.clone(),
        operation_definition_commitment: operation.operation_definition_commitment.clone(),
        previous_event_commitment,
        attempt_id,
        detail_commitment,
        outcome,
        refusal,
        wait_condition,
        wait_resolution_proof,
        recorded_at,
        event_commitment: SpecContentHash::from_text("pending"),
    };
    let input = EventCommitmentInput {
        domain: MODEL_DOMAIN,
        sequence: event.sequence,
        event_kind: event.event_kind,
        prior_status: event.prior_status,
        resulting_status: event.resulting_status,
        operation_id: &event.operation_id,
        operation_definition_commitment: &event.operation_definition_commitment,
        previous_event_commitment: &event.previous_event_commitment,
        attempt_id: &event.attempt_id,
        detail_commitment: &event.detail_commitment,
        outcome: event.outcome,
        refusal: event.refusal,
        wait_condition: &event.wait_condition,
        wait_resolution_proof: &event.wait_resolution_proof,
        recorded_at: event.recorded_at,
    };
    event.event_commitment =
        SpecContentHash::from_bytes(canonical_bytes(&input, "event commitment failed")?);
    Ok(event)
}

fn rehydrate(
    operation: ExternalActionTransitionOperation,
    events: Vec<ExternalActionTransitionLifecycleEvent>,
) -> Result<ExternalActionTransitionLifecycleSnapshot, WorkflowOsError> {
    let mut status = ExternalActionTransitionLifecycleStatus::Projected;
    let mut active_attempt_id = None;
    let mut suspended_status = None;
    let mut active_wait = None;
    let mut previous = None;
    for (index, event) in events.iter().enumerate() {
        if event.sequence != index as u64 + 1
            || event.operation_id != operation.operation_id
            || event.operation_definition_commitment != operation.operation_definition_commitment
            || event.previous_event_commitment != previous
        {
            return Err(lifecycle_error(
                "store.history_corrupt",
                "external action lifecycle history is invalid",
            ));
        }
        let expected = build_event(
            &operation,
            event.sequence,
            event.event_kind,
            event.prior_status,
            event.resulting_status,
            event.previous_event_commitment.clone(),
            event.attempt_id.clone(),
            event.detail_commitment.clone(),
            event.outcome,
            event.refusal,
            event.wait_condition.clone(),
            event.wait_resolution_proof.clone(),
            event.recorded_at,
        )?;
        if &expected != event
            || (index == 0
                && (event.event_kind != ExternalActionTransitionLifecycleEventKind::Projected
                    || event.prior_status.is_some()))
            || (index > 0 && event.prior_status != Some(status))
        {
            return Err(lifecycle_error(
                "store.history_corrupt",
                "external action lifecycle history is invalid",
            ));
        }
        validate_rehydrated_transition(
            &operation,
            status,
            active_attempt_id.as_ref(),
            active_wait.as_ref(),
            suspended_status,
            event,
        )?;
        match event.event_kind {
            ExternalActionTransitionLifecycleEventKind::Reserved => {
                active_attempt_id.clone_from(&event.attempt_id);
            }
            ExternalActionTransitionLifecycleEventKind::WaitRegistered => {
                suspended_status = event.prior_status;
                active_wait.clone_from(&event.wait_condition);
            }
            ExternalActionTransitionLifecycleEventKind::WaitResolved
            | ExternalActionTransitionLifecycleEventKind::Denied
            | ExternalActionTransitionLifecycleEventKind::Consumed
            | ExternalActionTransitionLifecycleEventKind::Revoked => {
                active_wait = None;
                suspended_status = None;
            }
            _ => {}
        }
        status = event.resulting_status;
        previous = Some(event.event_commitment.clone());
    }
    let head = events.last().ok_or_else(|| {
        lifecycle_error(
            "store.history_empty",
            "external action lifecycle history is empty",
        )
    })?;
    let cursor = ExternalActionTransitionLifecycleCursor {
        sequence: head.sequence,
        head_commitment: head.event_commitment.clone(),
    };
    Ok(ExternalActionTransitionLifecycleSnapshot {
        operation,
        status,
        active_attempt_id,
        suspended_status,
        active_wait,
        events,
        cursor,
    })
}

fn validate_wait(
    operation: &ExternalActionTransitionOperation,
    snapshot: &ExternalActionTransitionLifecycleSnapshot,
    wait: &AuthorizedExecutionWaitCondition,
) -> Result<(), WorkflowOsError> {
    let expected_action = format!("external-action/{}", operation.operation_id.as_str());
    let expected_event = format!(
        "event/external-action/{}",
        snapshot.cursor.head_commitment.as_str()
    );
    let condition_suffix = wait
        .condition_id()
        .as_str()
        .strip_prefix("external-action-wait/");
    let required_suffix = wait
        .required_reference()
        .as_str()
        .strip_prefix("external-action-requirement/");
    if wait.status() != AuthorizedExecutionWaitStatus::Waiting
        || wait.run_id() != &operation.run_id
        || wait.window_id() != &operation.execution_window_id
        || wait.action_reference().as_str() != expected_action
        || snapshot.active_attempt_id.as_ref() != Some(wait.attempt_id())
        || wait.expected_sequence_number().get() != snapshot.cursor.sequence
        || wait.expected_event_id().as_str() != expected_event
        || condition_suffix.is_none()
        || condition_suffix != required_suffix
    {
        return Err(lifecycle_error(
            "wait.binding_mismatch",
            "external action wait does not match the exact operation and attempt",
        ));
    }
    Ok(())
}

#[allow(clippy::too_many_lines)]
fn validate_rehydrated_transition(
    operation: &ExternalActionTransitionOperation,
    current: ExternalActionTransitionLifecycleStatus,
    active_attempt_id: Option<&AuthorizedExecutionAttemptId>,
    active_wait: Option<&AuthorizedExecutionWaitCondition>,
    suspended_status: Option<ExternalActionTransitionLifecycleStatus>,
    event: &ExternalActionTransitionLifecycleEvent,
) -> Result<(), WorkflowOsError> {
    if event.sequence == 1 {
        return Ok(());
    }
    let legal = match event.event_kind {
        ExternalActionTransitionLifecycleEventKind::Projected => false,
        ExternalActionTransitionLifecycleEventKind::Reserved => {
            matches!(
                current,
                ExternalActionTransitionLifecycleStatus::Projected
                    | ExternalActionTransitionLifecycleStatus::OutcomeFailed
                    | ExternalActionTransitionLifecycleStatus::ReconciledFailed
            ) && event.attempt_id.is_some()
        }
        ExternalActionTransitionLifecycleEventKind::Admitted => {
            current == ExternalActionTransitionLifecycleStatus::Reserved
                && event.attempt_id.as_ref() == active_attempt_id
        }
        ExternalActionTransitionLifecycleEventKind::Attempted => {
            current == ExternalActionTransitionLifecycleStatus::Admitted
                && event.attempt_id.as_ref() == active_attempt_id
        }
        ExternalActionTransitionLifecycleEventKind::OutcomeObserved => {
            current == ExternalActionTransitionLifecycleStatus::Attempted
                && event.attempt_id.as_ref() == active_attempt_id
                && event.outcome.is_some()
        }
        ExternalActionTransitionLifecycleEventKind::OutcomeReconciled => {
            current == ExternalActionTransitionLifecycleStatus::OutcomeAmbiguous
                && event.attempt_id.as_ref() == active_attempt_id
                && matches!(
                    event.outcome,
                    Some(
                        ExternalActionTransitionLifecycleOutcome::Succeeded
                            | ExternalActionTransitionLifecycleOutcome::Failed
                    )
                )
        }
        ExternalActionTransitionLifecycleEventKind::WaitRegistered => {
            let wait = event.wait_condition.as_ref();
            !matches!(
                current,
                ExternalActionTransitionLifecycleStatus::Denied
                    | ExternalActionTransitionLifecycleStatus::Consumed
                    | ExternalActionTransitionLifecycleStatus::Revoked
                    | ExternalActionTransitionLifecycleStatus::Waiting
            ) && wait.is_some_and(|wait| {
                wait.status() == AuthorizedExecutionWaitStatus::Waiting
                    && wait.run_id() == &operation.run_id
                    && wait.window_id() == &operation.execution_window_id
                    && wait.action_reference().as_str()
                        == format!("external-action/{}", operation.operation_id.as_str())
                    && active_attempt_id == Some(wait.attempt_id())
                    && wait.expected_sequence_number().get() + 1 == event.sequence
                    && event
                        .previous_event_commitment
                        .as_ref()
                        .is_some_and(|commitment| {
                            wait.expected_event_id().as_str()
                                == format!("event/external-action/{}", commitment.as_str())
                        })
                    && wait
                        .condition_id()
                        .as_str()
                        .strip_prefix("external-action-wait/")
                        == wait
                            .required_reference()
                            .as_str()
                            .strip_prefix("external-action-requirement/")
            })
        }
        ExternalActionTransitionLifecycleEventKind::WaitResolved => {
            current == ExternalActionTransitionLifecycleStatus::Waiting
                && active_wait.is_some_and(|wait| {
                    event.wait_resolution_proof.as_ref().is_some_and(|proof| {
                        proof.condition_id() == wait.condition_id()
                            && proof.validate(wait, event.recorded_at).is_ok()
                    })
                })
                && match event
                    .wait_resolution_proof
                    .as_ref()
                    .map(ExternalActionWaitResolutionProof::status)
                {
                    Some(AuthorizedExecutionWaitStatus::Satisfied) => {
                        event.resulting_status == suspended_status.unwrap_or(current)
                    }
                    Some(
                        AuthorizedExecutionWaitStatus::Expired
                        | AuthorizedExecutionWaitStatus::Superseded,
                    ) => event.resulting_status == ExternalActionTransitionLifecycleStatus::Denied,
                    _ => false,
                }
        }
        ExternalActionTransitionLifecycleEventKind::Denied => {
            !matches!(
                current,
                ExternalActionTransitionLifecycleStatus::Attempted
                    | ExternalActionTransitionLifecycleStatus::OutcomeSucceeded
                    | ExternalActionTransitionLifecycleStatus::OutcomeFailed
                    | ExternalActionTransitionLifecycleStatus::OutcomeAmbiguous
                    | ExternalActionTransitionLifecycleStatus::ReconciledSucceeded
                    | ExternalActionTransitionLifecycleStatus::Consumed
                    | ExternalActionTransitionLifecycleStatus::Revoked
                    | ExternalActionTransitionLifecycleStatus::Denied
            ) && event.refusal.is_some()
        }
        ExternalActionTransitionLifecycleEventKind::Consumed => {
            matches!(
                current,
                ExternalActionTransitionLifecycleStatus::OutcomeSucceeded
                    | ExternalActionTransitionLifecycleStatus::ReconciledSucceeded
            ) && event.attempt_id.as_ref() == active_attempt_id
        }
        ExternalActionTransitionLifecycleEventKind::Revoked => !matches!(
            current,
            ExternalActionTransitionLifecycleStatus::Attempted
                | ExternalActionTransitionLifecycleStatus::OutcomeSucceeded
                | ExternalActionTransitionLifecycleStatus::OutcomeFailed
                | ExternalActionTransitionLifecycleStatus::OutcomeAmbiguous
                | ExternalActionTransitionLifecycleStatus::ReconciledSucceeded
                | ExternalActionTransitionLifecycleStatus::ReconciledFailed
                | ExternalActionTransitionLifecycleStatus::Consumed
                | ExternalActionTransitionLifecycleStatus::Revoked
                | ExternalActionTransitionLifecycleStatus::Denied
        ),
    };
    if !legal {
        return Err(lifecycle_error(
            "store.history_illegal_transition",
            "external action lifecycle history contains an illegal transition",
        ));
    }
    Ok(())
}

fn require_attempt(
    snapshot: &ExternalActionTransitionLifecycleSnapshot,
    attempt_id: &AuthorizedExecutionAttemptId,
    status: ExternalActionTransitionLifecycleStatus,
) -> Result<(), WorkflowOsError> {
    if snapshot.status != status || snapshot.active_attempt_id.as_ref() != Some(attempt_id) {
        return Err(illegal_transition());
    }
    Ok(())
}

fn outcome_status(
    outcome: ExternalActionTransitionLifecycleOutcome,
    reconciled: bool,
) -> Result<ExternalActionTransitionLifecycleStatus, WorkflowOsError> {
    match (outcome, reconciled) {
        (ExternalActionTransitionLifecycleOutcome::Succeeded, false) => {
            Ok(ExternalActionTransitionLifecycleStatus::OutcomeSucceeded)
        }
        (ExternalActionTransitionLifecycleOutcome::Failed, false) => {
            Ok(ExternalActionTransitionLifecycleStatus::OutcomeFailed)
        }
        (ExternalActionTransitionLifecycleOutcome::Ambiguous, false) => {
            Ok(ExternalActionTransitionLifecycleStatus::OutcomeAmbiguous)
        }
        (ExternalActionTransitionLifecycleOutcome::Succeeded, true) => {
            Ok(ExternalActionTransitionLifecycleStatus::ReconciledSucceeded)
        }
        (ExternalActionTransitionLifecycleOutcome::Failed, true) => {
            Ok(ExternalActionTransitionLifecycleStatus::ReconciledFailed)
        }
        (ExternalActionTransitionLifecycleOutcome::Ambiguous, true) => Err(lifecycle_error(
            "reconciliation.ambiguous",
            "reconciliation must resolve the ambiguous outcome",
        )),
    }
}

fn illegal_transition() -> WorkflowOsError {
    lifecycle_error(
        "transition.illegal",
        "external action lifecycle transition is not legal from the current durable state",
    )
}

fn canonical_bytes(
    value: &impl Serialize,
    message: &'static str,
) -> Result<Vec<u8>, WorkflowOsError> {
    serde_json::to_vec(value).map_err(|_| lifecycle_error("serialization.failed", message))
}

fn parse_bytes<T: for<'de> Deserialize<'de>>(bytes: &[u8]) -> Result<T, WorkflowOsError> {
    serde_json::from_slice(bytes).map_err(|_| {
        lifecycle_error(
            "store.record_invalid",
            "external action lifecycle record is invalid",
        )
    })
}

fn read_bytes(path: &Path) -> Result<Vec<u8>, WorkflowOsError> {
    let mut file = File::open(path).map_err(|_| {
        lifecycle_error(
            "store.read_failed",
            "external action lifecycle store read failed",
        )
    })?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).map_err(|_| {
        lifecycle_error(
            "store.read_failed",
            "external action lifecycle store read failed",
        )
    })?;
    Ok(bytes)
}

fn write_create_only(path: &Path, bytes: &[u8]) -> Result<(), WorkflowOsError> {
    let parent = path.parent().ok_or_else(|| {
        lifecycle_error(
            "store.path_invalid",
            "external action lifecycle store path is invalid",
        )
    })?;
    fs::create_dir_all(parent).map_err(|_| {
        lifecycle_error(
            "store.create_failed",
            "external action lifecycle store write failed",
        )
    })?;
    let temporary = parent.join(format!(
        ".tmp-{}-{}",
        std::process::id(),
        TEMP_FILE_SEQUENCE.fetch_add(1, Ordering::Relaxed)
    ));
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|_| {
            lifecycle_error(
                "store.create_failed",
                "external action lifecycle store write failed",
            )
        })?;
    file.write_all(bytes)
        .and_then(|()| file.sync_all())
        .map_err(|_| {
            lifecycle_error(
                "store.write_failed",
                "external action lifecycle store write failed",
            )
        })?;
    fs::hard_link(&temporary, path).map_err(|_| {
        lifecycle_error(
            "store.commit_failed",
            "external action lifecycle store commit failed",
        )
    })?;
    let _ = fs::remove_file(&temporary);
    Ok(())
}

fn lifecycle_error(code: &'static str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(
        format!("external_action_transition_lifecycle.{code}"),
        message,
    )
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use std::fs;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::{Arc, Barrier};
    use std::thread;

    use crate::{
        compute_external_action_approval_scope_commitment, ActorId,
        ApprovalPresentationContentHash, ApprovalPresentationId, ApprovalReferenceId,
        AuthorizedExecutionActionReference, AuthorizedExecutionAttemptId,
        AuthorizedExecutionResourceReference, AuthorizedExecutionWaitConditionDefinition,
        AuthorizedExecutionWaitConditionId, AuthorizedExecutionWaitConditionKind,
        AuthorizedExecutionWakeTriggerKind, AuthorizedExecutionWindowId, CapabilityGrantId,
        EventId, EventSequenceNumber, ExternalActionAuthorizationProjectionDefinition,
        ExternalActionEvidencePredicate, ExternalActionEvidencePredicateKind,
        ExternalActionPayloadClass, ExternalConnectorKind, GitHubExternalActionResourceBinding,
        StepId, WorkReportSensitivity, WorkflowId,
    };

    use super::*;

    static TEST_SEQUENCE: AtomicU64 = AtomicU64::new(0);

    fn time(value: &str) -> Timestamp {
        Timestamp::parse_rfc3339(value).expect("valid time")
    }

    fn hash(value: &str) -> SpecContentHash {
        SpecContentHash::from_text(value)
    }

    fn predicate(kind: ExternalActionEvidencePredicateKind) -> ExternalActionEvidencePredicate {
        ExternalActionEvidencePredicate::new(
            kind,
            hash(&format!("source-{kind:?}")),
            hash(&format!("subject-{kind:?}")),
            time("2026-11-02T20:00:00Z"),
            3_600,
        )
        .expect("predicate")
    }

    fn projection() -> ExternalActionAuthorizationProjection {
        let connector_kind = ExternalConnectorKind::GitHub;
        let connector_account_reference = "github-installation/workflow-os".to_owned();
        let resource = GitHubExternalActionResourceBinding::new(
            "rcs2153/workflow-os",
            "main",
            "1111111111111111111111111111111111111111",
            "codex/lifecycle",
            "2222222222222222222222222222222222222222",
        )
        .expect("resource");
        let transition = ExternalActionTransition::new(
            ExternalActionTransitionKind::GitHubPullRequestCreateDraft,
            hash("operation-create-draft"),
            ExternalActionPayloadClass::GitHubDraftPullRequest,
            hash("payload-create-draft"),
            vec![
                predicate(ExternalActionEvidencePredicateKind::ReviewedHeadMatches),
                predicate(ExternalActionEvidencePredicateKind::BaseTargetMatches),
                predicate(ExternalActionEvidencePredicateKind::PayloadCommitmentMatches),
            ],
        )
        .expect("transition");
        let transitions = vec![transition];
        let non_goals = hash("strict-non-goals");
        let scope = compute_external_action_approval_scope_commitment(
            connector_kind,
            &connector_account_reference,
            &resource,
            &transitions,
            &non_goals,
        )
        .expect("scope");
        ExternalActionAuthorizationProjection::new(
            ExternalActionAuthorizationProjectionDefinition {
                workflow_id: WorkflowId::new("dg/pr").expect("workflow"),
                run_id: WorkflowRunId::new("run/lifecycle").expect("run"),
                approval_reference_id: ApprovalReferenceId::new("approval/lifecycle")
                    .expect("approval"),
                presentation_id: ApprovalPresentationId::new("presentation/lifecycle")
                    .expect("presentation"),
                presentation_content_hash: ApprovalPresentationContentHash::new(
                    hash("presentation").as_str(),
                )
                .expect("presentation hash"),
                approval_presentation_scope_commitment: scope,
                capability_grant_id: CapabilityGrantId::new("grant/lifecycle").expect("grant"),
                policy_decision_event_id: EventId::new("event/lifecycle").expect("event"),
                governance_assessment_commitment: hash("governance"),
                execution_window_id: AuthorizedExecutionWindowId::new("window/lifecycle")
                    .expect("window"),
                immutable_run_bundle_commitment: hash("bundle"),
                connector_kind,
                connector_account_reference,
                resource,
                transitions,
                actor: ActorId::new("actor/delegated-maintainer").expect("actor"),
                delegator: ActorId::new("actor/repository-owner").expect("delegator"),
                delegate: ActorId::new("actor/delegated-maintainer").expect("delegate"),
                delegation_scope_commitment: hash("delegation"),
                strict_non_goals_commitment: non_goals,
                issued_at: time("2026-11-02T20:00:00Z"),
                not_before: time("2026-11-02T20:00:00Z"),
                expires_at: time("2026-11-02T22:00:00Z"),
                revocation_source_commitment: hash("revocation-source"),
                sensitivity: WorkReportSensitivity::Internal,
            },
        )
        .expect("projection")
    }

    fn operation() -> ExternalActionTransitionOperation {
        ExternalActionTransitionOperation::from_projection(
            &projection(),
            ExternalActionTransitionKind::GitHubPullRequestCreateDraft,
        )
        .expect("operation")
    }

    fn store() -> (LocalExternalActionTransitionLifecycleStore, PathBuf) {
        let root = std::env::temp_dir().join(format!(
            "workflow-os-external-lifecycle-{}-{}",
            std::process::id(),
            TEST_SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
        (
            LocalExternalActionTransitionLifecycleStore::new(&root),
            root,
        )
    }

    fn reserve(
        store: &LocalExternalActionTransitionLifecycleStore,
        operation: &ExternalActionTransitionOperation,
    ) -> (
        AuthorizedExecutionAttemptId,
        ExternalActionTransitionLifecycleSnapshot,
    ) {
        let snapshot = store
            .read_snapshot(operation.operation_id())
            .expect("snapshot");
        let attempt = AuthorizedExecutionAttemptId::new("attempt/one").expect("attempt");
        store
            .append(
                operation.operation_id(),
                snapshot.cursor(),
                &ExternalActionTransitionLifecycleCommand::Reserve {
                    attempt_id: attempt.clone(),
                },
                time("2026-11-02T20:01:00Z"),
            )
            .expect("reserve");
        (
            attempt,
            store
                .read_snapshot(operation.operation_id())
                .expect("snapshot"),
        )
    }

    fn append(
        store: &LocalExternalActionTransitionLifecycleStore,
        operation: &ExternalActionTransitionOperation,
        command: &ExternalActionTransitionLifecycleCommand,
        timestamp: &str,
    ) -> ExternalActionTransitionLifecycleSnapshot {
        let before = store
            .read_snapshot(operation.operation_id())
            .expect("snapshot");
        store
            .append(
                operation.operation_id(),
                before.cursor(),
                command,
                time(timestamp),
            )
            .expect("append");
        store
            .read_snapshot(operation.operation_id())
            .expect("snapshot")
    }

    #[test]
    fn successful_lifecycle_is_durable_and_exactly_replayable() {
        let (store, root) = store();
        let operation = operation();
        assert_eq!(
            store
                .initialize(&operation, time("2026-11-02T20:00:01Z"))
                .expect("initialize"),
            ExternalActionTransitionLifecycleInitializationOutcome::Initialized
        );
        assert_eq!(
            store
                .initialize(&operation, time("2026-11-02T20:00:01Z"))
                .expect("initialize replay"),
            ExternalActionTransitionLifecycleInitializationOutcome::AlreadyInitialized
        );
        let initial = store
            .read_snapshot(operation.operation_id())
            .expect("initial");
        let attempt = AuthorizedExecutionAttemptId::new("attempt/one").expect("attempt");
        let reserve = ExternalActionTransitionLifecycleCommand::Reserve {
            attempt_id: attempt.clone(),
        };
        let first = store
            .append(
                operation.operation_id(),
                initial.cursor(),
                &reserve,
                time("2026-11-02T20:01:00Z"),
            )
            .expect("reserve");
        assert!(matches!(
            first,
            ExternalActionTransitionLifecycleAppendOutcome::Appended(_)
        ));
        assert!(matches!(
            store
                .append(
                    operation.operation_id(),
                    initial.cursor(),
                    &reserve,
                    time("2026-11-02T20:01:00Z"),
                )
                .expect("exact replay"),
            ExternalActionTransitionLifecycleAppendOutcome::ExactReplay(_)
        ));
        append(
            &store,
            &operation,
            &ExternalActionTransitionLifecycleCommand::Admit {
                attempt_id: attempt.clone(),
                admission_commitment: hash("admission"),
            },
            "2026-11-02T20:02:00Z",
        );
        append(
            &store,
            &operation,
            &ExternalActionTransitionLifecycleCommand::MarkAttempted {
                attempt_id: attempt.clone(),
                attempt_commitment: hash("attempted"),
            },
            "2026-11-02T20:03:00Z",
        );
        append(
            &store,
            &operation,
            &ExternalActionTransitionLifecycleCommand::ObserveOutcome {
                attempt_id: attempt.clone(),
                outcome: ExternalActionTransitionLifecycleOutcome::Succeeded,
                observation_commitment: hash("succeeded"),
            },
            "2026-11-02T20:04:00Z",
        );
        let completed = append(
            &store,
            &operation,
            &ExternalActionTransitionLifecycleCommand::Consume {
                attempt_id: attempt,
                consumption_commitment: hash("consumed"),
            },
            "2026-11-02T20:05:00Z",
        );
        assert_eq!(
            completed.status(),
            ExternalActionTransitionLifecycleStatus::Consumed
        );
        assert_eq!(completed.events().len(), 6);
        fs::remove_dir_all(root).expect("cleanup");
    }

    #[test]
    fn stale_cursor_and_terminal_state_fail_closed() {
        let (store, root) = store();
        let operation = operation();
        store
            .initialize(&operation, time("2026-11-02T20:00:01Z"))
            .expect("initialize");
        let stale = store
            .read_snapshot(operation.operation_id())
            .expect("initial")
            .cursor()
            .clone();
        let (_, reserved) = reserve(&store, &operation);
        let error = store
            .append(
                operation.operation_id(),
                &stale,
                &ExternalActionTransitionLifecycleCommand::Deny {
                    refusal: ExternalActionTransitionLifecycleRefusalKind::EvidenceIncomplete,
                    decision_commitment: hash("denied"),
                },
                time("2026-11-02T20:02:00Z"),
            )
            .expect_err("stale");
        assert_eq!(
            error.code(),
            "external_action_transition_lifecycle.append.stale_cursor"
        );
        store
            .append(
                operation.operation_id(),
                reserved.cursor(),
                &ExternalActionTransitionLifecycleCommand::Revoke {
                    revocation_commitment: hash("revoked"),
                },
                time("2026-11-02T20:02:00Z"),
            )
            .expect("revoke");
        let terminal = store
            .read_snapshot(operation.operation_id())
            .expect("terminal");
        assert_eq!(
            store
                .append(
                    operation.operation_id(),
                    terminal.cursor(),
                    &ExternalActionTransitionLifecycleCommand::Reserve {
                        attempt_id: AuthorizedExecutionAttemptId::new("attempt/two")
                            .expect("attempt"),
                    },
                    time("2026-11-02T20:03:00Z"),
                )
                .expect_err("terminal")
                .code(),
            "external_action_transition_lifecycle.transition.illegal"
        );
        fs::remove_dir_all(root).expect("cleanup");
    }

    #[test]
    fn concurrent_reservations_have_exactly_one_winner() {
        let (store, root) = store();
        let operation = operation();
        store
            .initialize(&operation, time("2026-11-02T20:00:01Z"))
            .expect("initialize");
        let cursor = store
            .read_snapshot(operation.operation_id())
            .expect("snapshot")
            .cursor()
            .clone();
        let store = Arc::new(store);
        let operation = Arc::new(operation);
        let barrier = Arc::new(Barrier::new(3));
        let mut handles = Vec::new();
        for suffix in ["one", "two"] {
            let store = Arc::clone(&store);
            let operation = Arc::clone(&operation);
            let barrier = Arc::clone(&barrier);
            let cursor = cursor.clone();
            handles.push(thread::spawn(move || {
                let command = ExternalActionTransitionLifecycleCommand::Reserve {
                    attempt_id: AuthorizedExecutionAttemptId::new(format!("attempt/{suffix}"))
                        .expect("attempt"),
                };
                barrier.wait();
                store.append(
                    operation.operation_id(),
                    &cursor,
                    &command,
                    time("2026-11-02T20:01:00Z"),
                )
            }));
        }
        barrier.wait();
        let results = handles
            .into_iter()
            .map(|handle| handle.join().expect("thread"))
            .collect::<Vec<_>>();
        assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
        assert_eq!(
            results
                .iter()
                .filter_map(|result| result.as_ref().err())
                .count(),
            1
        );
        assert_eq!(
            store
                .read_snapshot(operation.operation_id())
                .expect("snapshot")
                .events()
                .len(),
            2
        );
        fs::remove_dir_all(root).expect("cleanup");
    }

    #[test]
    fn ambiguous_outcome_requires_reconciliation_before_consumption_or_retry() {
        let (store, root) = store();
        let operation = operation();
        store
            .initialize(&operation, time("2026-11-02T20:00:01Z"))
            .expect("initialize");
        let (attempt, _) = reserve(&store, &operation);
        append(
            &store,
            &operation,
            &ExternalActionTransitionLifecycleCommand::Admit {
                attempt_id: attempt.clone(),
                admission_commitment: hash("admission"),
            },
            "2026-11-02T20:02:00Z",
        );
        append(
            &store,
            &operation,
            &ExternalActionTransitionLifecycleCommand::MarkAttempted {
                attempt_id: attempt.clone(),
                attempt_commitment: hash("attempted"),
            },
            "2026-11-02T20:03:00Z",
        );
        let ambiguous = append(
            &store,
            &operation,
            &ExternalActionTransitionLifecycleCommand::ObserveOutcome {
                attempt_id: attempt.clone(),
                outcome: ExternalActionTransitionLifecycleOutcome::Ambiguous,
                observation_commitment: hash("ambiguous"),
            },
            "2026-11-02T20:04:00Z",
        );
        assert_eq!(
            store
                .append(
                    operation.operation_id(),
                    ambiguous.cursor(),
                    &ExternalActionTransitionLifecycleCommand::Reserve {
                        attempt_id: AuthorizedExecutionAttemptId::new("attempt/two")
                            .expect("attempt"),
                    },
                    time("2026-11-02T20:05:00Z"),
                )
                .expect_err("redispatch blocked")
                .code(),
            "external_action_transition_lifecycle.transition.illegal"
        );
        let reconciled = append(
            &store,
            &operation,
            &ExternalActionTransitionLifecycleCommand::ReconcileOutcome {
                attempt_id: attempt.clone(),
                outcome: ExternalActionTransitionLifecycleOutcome::Succeeded,
                reconciliation_commitment: hash("reconciled"),
            },
            "2026-11-02T20:06:00Z",
        );
        assert_eq!(
            reconciled.status(),
            ExternalActionTransitionLifecycleStatus::ReconciledSucceeded
        );
        fs::remove_dir_all(root).expect("cleanup");
    }

    #[test]
    fn genuine_wait_resumes_exact_suspended_state() {
        let (store, root) = store();
        let operation = operation();
        store
            .initialize(&operation, time("2026-11-02T20:00:01Z"))
            .expect("initialize");
        let (attempt, reserved) = reserve(&store, &operation);
        let wait =
            AuthorizedExecutionWaitCondition::new(AuthorizedExecutionWaitConditionDefinition {
                condition_id: AuthorizedExecutionWaitConditionId::new(
                    "external-action-wait/connector",
                )
                .expect("condition"),
                condition_version: 1,
                kind: AuthorizedExecutionWaitConditionKind::CapabilityUnavailable,
                workflow_id: WorkflowId::new("dg/pr").expect("workflow"),
                run_id: WorkflowRunId::new("run/lifecycle").expect("run"),
                window_id: AuthorizedExecutionWindowId::new("window/lifecycle").expect("window"),
                action_reference: AuthorizedExecutionActionReference::new(format!(
                    "external-action/{}",
                    operation.operation_id().as_str()
                ))
                .expect("action"),
                step_id: StepId::new("connector-action").expect("step"),
                attempt_id: attempt,
                expected_sequence_number: EventSequenceNumber::new(reserved.cursor().sequence())
                    .expect("sequence"),
                expected_event_id: EventId::new(format!(
                    "event/external-action/{}",
                    reserved.cursor().head_commitment().as_str()
                ))
                .expect("event"),
                required_reference: AuthorizedExecutionResourceReference::new(
                    "external-action-requirement/connector",
                )
                .expect("resource"),
                created_at: time("2026-11-02T20:02:00Z"),
                deadline: Some(time("2026-11-02T21:00:00Z")),
                wake_trigger: AuthorizedExecutionWakeTriggerKind::CapabilityAvailabilityChanged,
                status: AuthorizedExecutionWaitStatus::Waiting,
            })
            .expect("wait");
        let waiting = append(
            &store,
            &operation,
            &ExternalActionTransitionLifecycleCommand::RegisterWait {
                wait_condition: Box::new(wait.clone()),
            },
            "2026-11-02T20:02:00Z",
        );
        assert_eq!(
            waiting.status(),
            ExternalActionTransitionLifecycleStatus::Waiting
        );
        let mut mismatched_proof = ExternalActionWaitResolutionProof::new(
            &wait,
            hash("wait-satisfied-source"),
            AuthorizedExecutionWaitStatus::Satisfied,
            time("2026-11-02T20:03:00Z"),
        )
        .expect("resolution proof");
        mismatched_proof.required_reference =
            AuthorizedExecutionResourceReference::new("external-action-requirement/substituted")
                .expect("substituted resource");
        assert_eq!(
            store
                .append(
                    operation.operation_id(),
                    waiting.cursor(),
                    &ExternalActionTransitionLifecycleCommand::ResolveWait {
                        proof: mismatched_proof,
                    },
                    time("2026-11-02T20:03:00Z"),
                )
                .expect_err("mismatched proof rejected")
                .code(),
            "external_action_transition_lifecycle.wait_resolution.proof_mismatch"
        );
        let resumed = append(
            &store,
            &operation,
            &ExternalActionTransitionLifecycleCommand::ResolveWait {
                proof: ExternalActionWaitResolutionProof::new(
                    &wait,
                    hash("wait-satisfied-source"),
                    AuthorizedExecutionWaitStatus::Satisfied,
                    time("2026-11-02T20:03:00Z"),
                )
                .expect("resolution proof"),
            },
            "2026-11-02T20:03:00Z",
        );
        assert_eq!(
            resumed.status(),
            ExternalActionTransitionLifecycleStatus::Reserved
        );
        fs::remove_dir_all(root).expect("cleanup");
    }

    #[test]
    fn tampered_event_fails_rehydration() {
        let (store, root) = store();
        let operation = operation();
        store
            .initialize(&operation, time("2026-11-02T20:00:01Z"))
            .expect("initialize");
        reserve(&store, &operation);
        let event_path = store.event_path(operation.operation_id(), 2);
        let mut bytes = fs::read(&event_path).expect("event");
        let position = bytes
            .windows(b"reserved".len())
            .position(|window| window == b"reserved")
            .expect("status text");
        bytes[position..position + b"reserved".len()].copy_from_slice(b"revoked_");
        fs::write(&event_path, bytes).expect("tamper");
        assert_eq!(
            store
                .read_snapshot(operation.operation_id())
                .err()
                .expect("tamper rejected")
                .code(),
            "external_action_transition_lifecycle.store.record_invalid"
        );
        fs::remove_dir_all(root).expect("cleanup");
    }
}
