use std::fmt;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::authorized_execution_continuity_state::internal::{
    ContinuityCursor, ContinuityRevision, ExpectedWindowBinding, TrustedTimeObservation,
};
use crate::{
    ActorId, AuthorizedExecutionAttemptId, AuthorizedExecutionWindowId, EventId,
    EventSequenceNumber, ImmutableRunBundleBinding, LocalStateBackend, PostgresStateBackend,
    RedactionMetadata, RequiredContextContractBinding, RequiredContextExecutionBinding, SkillInput,
    SpecContentHash, StateBackend, StepId, Timestamp, WorkflowId, WorkflowOsError,
    WorkflowOsErrorKind, WorkflowRunId,
};

const IDENTIFIER_MAX_BYTES: usize = 128;

macro_rules! bounded_id {
    ($(#[$meta:meta])* $name:ident, $label:literal) => {
        $(#[$meta])*
        #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            /// Creates a bounded, non-secret identifier.
            ///
            /// # Errors
            ///
            /// Returns a stable validation error when the identifier is invalid.
            pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
                let value = value.into();
                validate_identifier($label, &value)?;
                Ok(Self(value))
            }

            #[must_use]
            /// Returns the validated identifier.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter
                    .debug_tuple(stringify!($name))
                    .field(&"[REDACTED]")
                    .finish()
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = WorkflowOsError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                Self::new(String::deserialize(deserializer)?)
                    .map_err(|_| serde::de::Error::custom(concat!("invalid ", $label)))
            }
        }
    };
}

bounded_id!(
    /// Stable identity for one operational execution-window opening request.
    OperationalExecutionWindowOpeningOperationId,
    "operational opening operation id"
);
bounded_id!(
    /// Stable receipt identity for one committed operational opening.
    OperationalExecutionWindowOpeningReceiptId,
    "operational opening receipt id"
);

/// Version of the operational execution-window opening projection contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationalExecutionWindowOpeningVersion {
    /// Initial opening-only projection contract.
    V1,
    /// Complete request/snapshot commitment and validated projection contract.
    V2,
}

/// Exact operation authorized by an operational execution window.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationalExecutionOperationBinding {
    /// Invoke the current workflow step's registered skill.
    InvokeCurrentStepSkill,
}

/// Durable event cursor used by the operational opening projection.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OperationalExecutionWindowOpeningProjectionCursor {
    sequence_number: EventSequenceNumber,
    event_id: EventId,
}

impl OperationalExecutionWindowOpeningProjectionCursor {
    pub(crate) const fn new(sequence_number: EventSequenceNumber, event_id: EventId) -> Self {
        Self {
            sequence_number,
            event_id,
        }
    }

    #[must_use]
    /// Returns the committed event sequence number.
    pub const fn sequence_number(&self) -> EventSequenceNumber {
        self.sequence_number
    }

    #[must_use]
    /// Returns the committed event identity.
    pub const fn event_id(&self) -> &EventId {
        &self.event_id
    }
}

/// Payload-free durable projection of one operational execution-window opening.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "OperationalExecutionWindowOpeningProjectionEventWire")]
pub struct OperationalExecutionWindowOpeningProjectionEvent {
    version: OperationalExecutionWindowOpeningVersion,
    operation_id: OperationalExecutionWindowOpeningOperationId,
    receipt_id: OperationalExecutionWindowOpeningReceiptId,
    window_id: AuthorizedExecutionWindowId,
    attempt_id: AuthorizedExecutionAttemptId,
    attempt_number: u32,
    window_revision: u64,
    operation_binding: OperationalExecutionOperationBinding,
    operation_binding_commitment: SpecContentHash,
    request_commitment: SpecContentHash,
    projection_commitment: SpecContentHash,
    expected_input_cursor: OperationalExecutionWindowOpeningProjectionCursor,
    committed_result_cursor: OperationalExecutionWindowOpeningProjectionCursor,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct OperationalExecutionWindowOpeningProjectionEventWire {
    version: OperationalExecutionWindowOpeningVersion,
    operation_id: OperationalExecutionWindowOpeningOperationId,
    receipt_id: OperationalExecutionWindowOpeningReceiptId,
    window_id: AuthorizedExecutionWindowId,
    attempt_id: AuthorizedExecutionAttemptId,
    attempt_number: u32,
    window_revision: u64,
    operation_binding: OperationalExecutionOperationBinding,
    operation_binding_commitment: SpecContentHash,
    request_commitment: SpecContentHash,
    projection_commitment: SpecContentHash,
    expected_input_cursor: OperationalExecutionWindowOpeningProjectionCursor,
    committed_result_cursor: OperationalExecutionWindowOpeningProjectionCursor,
}

impl OperationalExecutionWindowOpeningProjectionEvent {
    pub(crate) fn new(
        definition: OperationalExecutionWindowOpeningProjectionEventDefinition,
    ) -> Result<Self, WorkflowOsError> {
        if definition.attempt_number != 1
            || definition.window_revision != 1
            || definition.committed_result_cursor.sequence_number().get()
                != definition
                    .expected_input_cursor
                    .sequence_number()
                    .get()
                    .checked_add(1)
                    .ok_or_else(projection_invalid)?
            || definition.committed_result_cursor.event_id()
                != &operational_opening_event_id(&definition.operation_id)?
        {
            return Err(opening_error(
                WorkflowOsErrorKind::Validation,
                "projection.invalid",
                "operational opening projection is invalid",
            ));
        }
        let projection_commitment = opening_projection_commitment(&definition);
        Ok(Self {
            version: OperationalExecutionWindowOpeningVersion::V2,
            operation_id: definition.operation_id,
            receipt_id: definition.receipt_id,
            window_id: definition.window_id,
            attempt_id: definition.attempt_id,
            attempt_number: definition.attempt_number,
            window_revision: definition.window_revision,
            operation_binding: OperationalExecutionOperationBinding::InvokeCurrentStepSkill,
            operation_binding_commitment: definition.operation_binding_commitment,
            request_commitment: definition.request_commitment,
            projection_commitment,
            expected_input_cursor: definition.expected_input_cursor,
            committed_result_cursor: definition.committed_result_cursor,
        })
    }

    #[must_use]
    /// Returns the projection contract version.
    pub const fn version(&self) -> OperationalExecutionWindowOpeningVersion {
        self.version
    }

    #[must_use]
    /// Returns the stable opening operation identity.
    pub const fn operation_id(&self) -> &OperationalExecutionWindowOpeningOperationId {
        &self.operation_id
    }

    #[must_use]
    /// Returns the durable opening receipt identity.
    pub const fn receipt_id(&self) -> &OperationalExecutionWindowOpeningReceiptId {
        &self.receipt_id
    }

    #[must_use]
    /// Returns the execution-window identity.
    pub const fn window_id(&self) -> &AuthorizedExecutionWindowId {
        &self.window_id
    }

    #[must_use]
    /// Returns the first operational attempt identity.
    pub const fn attempt_id(&self) -> &AuthorizedExecutionAttemptId {
        &self.attempt_id
    }

    #[must_use]
    /// Returns the attempt number created by the opening transaction.
    pub const fn attempt_number(&self) -> u32 {
        self.attempt_number
    }

    #[must_use]
    /// Returns the initial authoritative window revision.
    pub const fn window_revision(&self) -> u64 {
        self.window_revision
    }

    #[must_use]
    /// Returns the exact operation bound to the opened window.
    pub const fn operation_binding(&self) -> OperationalExecutionOperationBinding {
        self.operation_binding
    }

    #[must_use]
    /// Returns the payload-free projection commitment.
    pub const fn projection_commitment(&self) -> &SpecContentHash {
        &self.projection_commitment
    }

    #[must_use]
    /// Returns the event cursor committed by the opening transaction.
    pub const fn committed_result_cursor(
        &self,
    ) -> &OperationalExecutionWindowOpeningProjectionCursor {
        &self.committed_result_cursor
    }
}

impl TryFrom<OperationalExecutionWindowOpeningProjectionEventWire>
    for OperationalExecutionWindowOpeningProjectionEvent
{
    type Error = WorkflowOsError;

    fn try_from(
        wire: OperationalExecutionWindowOpeningProjectionEventWire,
    ) -> Result<Self, Self::Error> {
        if wire.version != OperationalExecutionWindowOpeningVersion::V2
            || wire.operation_binding
                != OperationalExecutionOperationBinding::InvokeCurrentStepSkill
        {
            return Err(projection_invalid());
        }
        let expected_commitment = wire.projection_commitment.clone();
        let event = Self::new(OperationalExecutionWindowOpeningProjectionEventDefinition {
            operation_id: wire.operation_id,
            receipt_id: wire.receipt_id,
            window_id: wire.window_id,
            attempt_id: wire.attempt_id,
            attempt_number: wire.attempt_number,
            window_revision: wire.window_revision,
            operation_binding_commitment: wire.operation_binding_commitment,
            request_commitment: wire.request_commitment,
            expected_input_cursor: wire.expected_input_cursor,
            committed_result_cursor: wire.committed_result_cursor,
        })?;
        if event.projection_commitment != expected_commitment {
            return Err(projection_invalid());
        }
        Ok(event)
    }
}

impl fmt::Debug for OperationalExecutionWindowOpeningProjectionEvent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OperationalExecutionWindowOpeningProjectionEvent")
            .field("version", &self.version)
            .field("attempt_number", &self.attempt_number)
            .field("window_revision", &self.window_revision)
            .field("operation_binding", &self.operation_binding)
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

pub(crate) struct OperationalExecutionWindowOpeningProjectionEventDefinition {
    pub(crate) operation_id: OperationalExecutionWindowOpeningOperationId,
    pub(crate) receipt_id: OperationalExecutionWindowOpeningReceiptId,
    pub(crate) window_id: AuthorizedExecutionWindowId,
    pub(crate) attempt_id: AuthorizedExecutionAttemptId,
    pub(crate) attempt_number: u32,
    pub(crate) window_revision: u64,
    pub(crate) operation_binding_commitment: SpecContentHash,
    pub(crate) request_commitment: SpecContentHash,
    pub(crate) expected_input_cursor: OperationalExecutionWindowOpeningProjectionCursor,
    pub(crate) committed_result_cursor: OperationalExecutionWindowOpeningProjectionCursor,
}

/// Latest durable operational execution-window opening projection for a run.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct OperationalExecutionWindowOpeningProjectionSnapshot {
    version: OperationalExecutionWindowOpeningVersion,
    operation_id: OperationalExecutionWindowOpeningOperationId,
    window_id: AuthorizedExecutionWindowId,
    attempt_id: AuthorizedExecutionAttemptId,
    attempt_number: u32,
    window_revision: u64,
    operation_binding: OperationalExecutionOperationBinding,
    projection_commitment: SpecContentHash,
    committed_result_cursor: OperationalExecutionWindowOpeningProjectionCursor,
}

impl OperationalExecutionWindowOpeningProjectionSnapshot {
    pub(crate) fn from_event(event: &OperationalExecutionWindowOpeningProjectionEvent) -> Self {
        Self {
            version: event.version,
            operation_id: event.operation_id.clone(),
            window_id: event.window_id.clone(),
            attempt_id: event.attempt_id.clone(),
            attempt_number: event.attempt_number,
            window_revision: event.window_revision,
            operation_binding: event.operation_binding,
            projection_commitment: event.projection_commitment.clone(),
            committed_result_cursor: event.committed_result_cursor.clone(),
        }
    }
}

impl fmt::Debug for OperationalExecutionWindowOpeningProjectionSnapshot {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OperationalExecutionWindowOpeningProjectionSnapshot")
            .field("version", &self.version)
            .field("attempt_number", &self.attempt_number)
            .field("window_revision", &self.window_revision)
            .field("operation_binding", &self.operation_binding)
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

#[allow(clippy::struct_field_names)]
pub(crate) struct OperationalExecutionWindowOpeningAuthorization {
    source_authority_commitment: SpecContentHash,
    governance_commitment: SpecContentHash,
    operation_binding_commitment: SpecContentHash,
}

pub(crate) struct OperationalExecutionWindowOpeningUseInput<'a> {
    pub(crate) source:
        &'a crate::current_authority_source::RegisteredInMemoryCurrentAuthoritySource,
    pub(crate) backend: &'a crate::SqliteStateBackend,
    pub(crate) execution_binding: &'a RequiredContextExecutionBinding,
    pub(crate) contract: &'a RequiredContextContractBinding,
    pub(crate) invocation_binding_commitment: &'a SpecContentHash,
    pub(crate) operation_id: OperationalExecutionWindowOpeningOperationId,
    pub(crate) receipt_id: OperationalExecutionWindowOpeningReceiptId,
    pub(crate) window_id: AuthorizedExecutionWindowId,
    pub(crate) attempt_id: AuthorizedExecutionAttemptId,
    pub(crate) expires_at: Timestamp,
    pub(crate) maximum_attempts: u32,
    pub(crate) trusted_time: TrustedTimeObservation,
    pub(crate) evaluated_at: Timestamp,
    pub(crate) redaction: &'a RedactionMetadata,
}

#[allow(clippy::too_many_lines)]
pub(crate) fn open_with_registered_current_authority(
    input: &OperationalExecutionWindowOpeningUseInput<'_>,
) -> Result<ProjectedOperationalExecutionWindowOpeningResult, WorkflowOsError> {
    let run = input
        .backend
        .rehydrate_run(input.execution_binding.run_id())?;
    if run.snapshot.identity.workflow_id != *input.execution_binding.workflow_id()
        || run.snapshot.identity.run_id != *input.execution_binding.run_id()
        || !matches!(
            run.snapshot.status,
            crate::WorkflowRunStatus::Running | crate::WorkflowRunStatus::Retrying
        )
    {
        return Err(opening_error(
            WorkflowOsErrorKind::InvalidState,
            "run_not_eligible",
            "workflow run is not eligible for operational opening",
        ));
    }
    let immutable_run_bundle = run
        .snapshot
        .identity
        .immutable_run_bundle
        .clone()
        .ok_or_else(|| {
            opening_error(
                WorkflowOsErrorKind::InvalidState,
                "immutable_bundle_missing",
                "operational opening requires an immutable run bundle",
            )
        })?;
    let operation_binding_commitment = operation_binding_commitment(
        input.execution_binding.workflow_id(),
        input.execution_binding.run_id(),
        input.execution_binding.step_id(),
        input.invocation_binding_commitment,
    );
    let expected_snapshot_commitment = run_snapshot_commitment(&run.snapshot)?;

    let mut store_result = None;
    let mut store_error = None;
    let use_outcome = input.source.use_current_authority(
        &crate::current_authority_source::RegisteredCurrentAuthorityUseInput {
            execution_binding: input.execution_binding,
            contract: input.contract,
            evaluated_at: input.evaluated_at,
            redaction: input.redaction,
        },
        |capability| {
            let source_commitment = match capability.continuation_governance_commitment() {
                Ok(commitment) => commitment,
                Err(error) => {
                    store_error = Some(error);
                    return crate::current_authority_source::RegisteredCurrentAuthorityConsumerResult::Failed;
                }
            };
            let governance_commitment = core_governance_commitment(
                &run,
                &immutable_run_bundle,
                &source_commitment,
            );
            let authorization = OperationalExecutionWindowOpeningAuthorization::new(
                source_commitment,
                governance_commitment,
                operation_binding_commitment.clone(),
            );
            let mut request = OperationalExecutionWindowOpeningRequest {
                operation_id: input.operation_id.clone(),
                receipt_id: input.receipt_id.clone(),
                request_commitment: SpecContentHash::from_text("pending opening request"),
                workflow_id: input.execution_binding.workflow_id().clone(),
                run_id: input.execution_binding.run_id().clone(),
                step_id: input.execution_binding.step_id().clone(),
                invocation_binding_commitment: input.invocation_binding_commitment.clone(),
                window_id: input.window_id.clone(),
                attempt_id: input.attempt_id.clone(),
                subject_actor_id: input.execution_binding.actor().clone(),
                immutable_run_bundle: immutable_run_bundle.clone(),
                expected_cursor: ContinuityCursor {
                    sequence_number: run.snapshot.last_sequence_number,
                    event_id: run.snapshot.last_event_id.clone(),
                },
                expected_snapshot_commitment: expected_snapshot_commitment.clone(),
                expires_at: input.expires_at,
                maximum_attempts: input.maximum_attempts,
                trusted_time: input.trusted_time.clone(),
                authorization: &authorization,
            };
            request.request_commitment = opening_request_commitment(&request);
            match input.backend.open_window_and_start_attempt_projected(request) {
                Ok(result) => {
                    store_result = Some(result);
                    crate::current_authority_source::RegisteredCurrentAuthorityConsumerResult::Succeeded
                }
                Err(error) => {
                    store_error = Some(error);
                    crate::current_authority_source::RegisteredCurrentAuthorityConsumerResult::Failed
                }
            }
        },
    )?;
    match use_outcome.posture() {
        crate::current_authority_source::RegisteredCurrentAuthorityUsePosture::ConsumerSucceeded => {
            store_result.ok_or_else(|| {
                opening_error(
                    WorkflowOsErrorKind::InvalidState,
                    "result_missing",
                    "operational opening result is unavailable",
                )
            })
        }
        crate::current_authority_source::RegisteredCurrentAuthorityUsePosture::ConsumerFailed => {
            Err(store_error.unwrap_or_else(|| {
                opening_error(
                    WorkflowOsErrorKind::InvalidState,
                    "store_failed",
                    "operational opening store failed",
                )
            }))
        }
        crate::current_authority_source::RegisteredCurrentAuthorityUsePosture::BlockedBeforeUse
        | crate::current_authority_source::RegisteredCurrentAuthorityUsePosture::SourceFailure => {
            Err(opening_error(
                WorkflowOsErrorKind::Security,
                "authority_not_ready",
                "current authority did not permit operational opening",
            ))
        }
        crate::current_authority_source::RegisteredCurrentAuthorityUsePosture::ConsumerOutcomeAmbiguous => {
            Err(opening_error(
                WorkflowOsErrorKind::InvalidState,
                "authority_use_ambiguous",
                "operational opening authority use outcome is ambiguous",
            ))
        }
    }
}

fn core_governance_commitment(
    run: &crate::WorkflowRun,
    immutable_run_bundle: &ImmutableRunBundleBinding,
    source_commitment: &SpecContentHash,
) -> SpecContentHash {
    hash_fields(
        "workflow-os/operational-opening/core-governance/v1",
        &[
            run.snapshot.identity.workflow_id.as_str(),
            run.snapshot.identity.run_id.as_str(),
            immutable_run_bundle.root_hash().as_str(),
            source_commitment.as_str(),
            &run.snapshot.policy_decisions.len().to_string(),
            &run.snapshot.approval_requests.len().to_string(),
            &run.snapshot.last_sequence_number.get().to_string(),
            run.snapshot.last_event_id.as_str(),
        ],
    )
}

impl OperationalExecutionWindowOpeningAuthorization {
    pub(crate) fn new(
        source_authority_commitment: SpecContentHash,
        governance_commitment: SpecContentHash,
        operation_binding_commitment: SpecContentHash,
    ) -> Self {
        Self {
            source_authority_commitment,
            governance_commitment,
            operation_binding_commitment,
        }
    }
}

impl fmt::Debug for OperationalExecutionWindowOpeningAuthorization {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OperationalExecutionWindowOpeningAuthorization")
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

pub(crate) struct OperationalExecutionWindowOpeningRequest<'a> {
    pub(crate) operation_id: OperationalExecutionWindowOpeningOperationId,
    pub(crate) receipt_id: OperationalExecutionWindowOpeningReceiptId,
    pub(crate) request_commitment: SpecContentHash,
    pub(crate) workflow_id: WorkflowId,
    pub(crate) run_id: WorkflowRunId,
    pub(crate) step_id: StepId,
    pub(crate) invocation_binding_commitment: SpecContentHash,
    pub(crate) window_id: AuthorizedExecutionWindowId,
    pub(crate) attempt_id: AuthorizedExecutionAttemptId,
    pub(crate) subject_actor_id: ActorId,
    pub(crate) immutable_run_bundle: ImmutableRunBundleBinding,
    pub(crate) expected_cursor: ContinuityCursor,
    pub(crate) expected_snapshot_commitment: SpecContentHash,
    pub(crate) expires_at: Timestamp,
    pub(crate) maximum_attempts: u32,
    pub(crate) trusted_time: TrustedTimeObservation,
    pub(crate) authorization: &'a OperationalExecutionWindowOpeningAuthorization,
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct OperationalExecutionWindowOpeningRecordedResult {
    pub(crate) operation_id: OperationalExecutionWindowOpeningOperationId,
    pub(crate) receipt_id: OperationalExecutionWindowOpeningReceiptId,
    pub(crate) workflow_id: WorkflowId,
    pub(crate) run_id: WorkflowRunId,
    pub(crate) step_id: StepId,
    pub(crate) window_id: AuthorizedExecutionWindowId,
    pub(crate) attempt_id: AuthorizedExecutionAttemptId,
    pub(crate) attempt_number: u32,
    pub(crate) window_revision: ContinuityRevision,
    pub(crate) operation_binding_commitment: SpecContentHash,
    pub(crate) request_commitment: SpecContentHash,
    pub(crate) committed_at: Timestamp,
}

pub(crate) struct OperationalExecutionAttemptUseCapability {
    pub(crate) attempt_id: AuthorizedExecutionAttemptId,
    pub(crate) window_id: AuthorizedExecutionWindowId,
    pub(crate) subject_actor_id: ActorId,
    pub(crate) window_revision: ContinuityRevision,
    pub(crate) cursor: ContinuityCursor,
    pub(crate) authority_commitment: SpecContentHash,
    pub(crate) window_binding_commitment: SpecContentHash,
    pub(crate) expected_window_binding: ExpectedWindowBinding,
    pub(crate) operation_binding_commitment: SpecContentHash,
    pub(crate) consume_operation_id:
        crate::authorized_execution_continuity_state::internal::ContinuityOperationId,
    pub(crate) opening_operation_id: OperationalExecutionWindowOpeningOperationId,
}

impl fmt::Debug for OperationalExecutionAttemptUseCapability {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OperationalExecutionAttemptUseCapability")
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

pub(crate) enum OperationalExecutionWindowOpeningResult {
    Opened {
        result: OperationalExecutionWindowOpeningRecordedResult,
        capability: Box<OperationalExecutionAttemptUseCapability>,
    },
    ExactReplay(OperationalExecutionWindowOpeningRecordedResult),
}

pub(crate) struct ProjectedOperationalExecutionWindowOpeningResult {
    pub(crate) result: OperationalExecutionWindowOpeningResult,
    pub(crate) event: OperationalExecutionWindowOpeningProjectionEvent,
    pub(crate) snapshot_commitment: SpecContentHash,
}

pub(crate) enum OperationalExecutionWindowOpeningReconciliationResult {
    DurablyCommitted {
        result: Box<OperationalExecutionWindowOpeningRecordedResult>,
        event: Box<OperationalExecutionWindowOpeningProjectionEvent>,
        snapshot_commitment: SpecContentHash,
    },
    ConfirmedAbsent,
}

pub(crate) trait OperationalExecutionWindowOpeningStoreV1 {
    fn open_window_and_start_attempt_projected(
        &self,
        request: OperationalExecutionWindowOpeningRequest<'_>,
    ) -> Result<ProjectedOperationalExecutionWindowOpeningResult, WorkflowOsError>;

    fn reconcile_operational_opening(
        &self,
        operation_id: &OperationalExecutionWindowOpeningOperationId,
        expected_request_commitment: &SpecContentHash,
        expected_receipt_id: &OperationalExecutionWindowOpeningReceiptId,
    ) -> Result<OperationalExecutionWindowOpeningReconciliationResult, WorkflowOsError>;
}

macro_rules! unsupported_store {
    ($type:ty) => {
        impl OperationalExecutionWindowOpeningStoreV1 for $type {
            fn open_window_and_start_attempt_projected(
                &self,
                _request: OperationalExecutionWindowOpeningRequest<'_>,
            ) -> Result<ProjectedOperationalExecutionWindowOpeningResult, WorkflowOsError> {
                Err(opening_unsupported())
            }

            fn reconcile_operational_opening(
                &self,
                _operation_id: &OperationalExecutionWindowOpeningOperationId,
                _expected_request_commitment: &SpecContentHash,
                _expected_receipt_id: &OperationalExecutionWindowOpeningReceiptId,
            ) -> Result<OperationalExecutionWindowOpeningReconciliationResult, WorkflowOsError>
            {
                Err(opening_unsupported())
            }
        }
    };
}

unsupported_store!(LocalStateBackend);
unsupported_store!(PostgresStateBackend);

pub(crate) fn opening_request_commitment(
    request: &OperationalExecutionWindowOpeningRequest<'_>,
) -> SpecContentHash {
    let observed_nanos = request
        .trusted_time
        .observed_at()
        .as_offset_date_time()
        .unix_timestamp_nanos()
        .to_string();
    let expires_nanos = request
        .expires_at
        .as_offset_date_time()
        .unix_timestamp_nanos()
        .to_string();
    let expected_sequence = request.expected_cursor.sequence_number.get().to_string();
    let maximum_attempts = request.maximum_attempts.to_string();
    hash_fields(
        "workflow-os/operational-execution-window-opening/request/v2",
        &[
            request.operation_id.as_str(),
            request.receipt_id.as_str(),
            request.workflow_id.as_str(),
            request.run_id.as_str(),
            request.step_id.as_str(),
            request.invocation_binding_commitment.as_str(),
            request.window_id.as_str(),
            request.attempt_id.as_str(),
            request.subject_actor_id.as_str(),
            request.immutable_run_bundle.bundle_id().as_str(),
            request.immutable_run_bundle.bundle_version().as_str(),
            request.immutable_run_bundle.root_hash().as_str(),
            request.expected_cursor.event_id.as_str(),
            &expected_sequence,
            request.expected_snapshot_commitment.as_str(),
            &expires_nanos,
            &maximum_attempts,
            &observed_nanos,
            "core_injected_clock_v1",
            request.trusted_time.provenance_commitment().as_str(),
            request.trusted_time.epoch_id().as_str(),
            request.authorization.source_authority_commitment.as_str(),
            request.authorization.governance_commitment.as_str(),
            request.authorization.operation_binding_commitment.as_str(),
        ],
    )
}

pub(crate) fn run_snapshot_commitment(
    snapshot: &crate::WorkflowRunSnapshot,
) -> Result<SpecContentHash, WorkflowOsError> {
    let payload = serde_json::to_string(snapshot).map_err(|_| {
        opening_error(
            WorkflowOsErrorKind::Validation,
            "snapshot.invalid",
            "operational opening snapshot is invalid",
        )
    })?;
    let mut hasher = Sha256::new();
    hasher.update(b"workflow-os/run-snapshot/v1\0");
    hasher.update(payload.as_bytes());
    Ok(SpecContentHash::from_bytes(hasher.finalize()))
}

fn opening_projection_commitment(
    definition: &OperationalExecutionWindowOpeningProjectionEventDefinition,
) -> SpecContentHash {
    let attempt_number = definition.attempt_number.to_string();
    let window_revision = definition.window_revision.to_string();
    let expected_sequence = definition
        .expected_input_cursor
        .sequence_number()
        .get()
        .to_string();
    let committed_sequence = definition
        .committed_result_cursor
        .sequence_number()
        .get()
        .to_string();
    hash_fields(
        "workflow-os/operational-opening/projection/v2",
        &[
            "v2",
            definition.operation_id.as_str(),
            definition.receipt_id.as_str(),
            definition.window_id.as_str(),
            definition.attempt_id.as_str(),
            &attempt_number,
            &window_revision,
            "invoke_current_step_skill",
            definition.operation_binding_commitment.as_str(),
            definition.request_commitment.as_str(),
            definition.expected_input_cursor.event_id().as_str(),
            &expected_sequence,
            definition.committed_result_cursor.event_id().as_str(),
            &committed_sequence,
        ],
    )
}

pub(crate) fn operational_opening_event_id(
    operation_id: &OperationalExecutionWindowOpeningOperationId,
) -> Result<EventId, WorkflowOsError> {
    let digest = Sha256::digest(operation_id.as_str().as_bytes());
    let suffix = digest[..12]
        .iter()
        .fold(String::with_capacity(24), |mut output, byte| {
            use std::fmt::Write as _;
            let _ = write!(output, "{byte:02x}");
            output
        });
    EventId::new(format!("event/operational-opening/{suffix}"))
}

fn projection_invalid() -> WorkflowOsError {
    opening_error(
        WorkflowOsErrorKind::Validation,
        "projection.invalid",
        "operational opening projection is invalid",
    )
}

pub(crate) fn operation_binding_commitment(
    workflow_id: &WorkflowId,
    run_id: &WorkflowRunId,
    step_id: &StepId,
    invocation_binding_commitment: &SpecContentHash,
) -> SpecContentHash {
    hash_fields(
        "workflow-os/operational-execution-operation-binding/v1",
        &[
            "invoke_current_step_skill",
            workflow_id.as_str(),
            run_id.as_str(),
            step_id.as_str(),
            invocation_binding_commitment.as_str(),
        ],
    )
}

pub(crate) fn trusted_host_invocation_commitment(
    input: &SkillInput,
    executor_binding_commitment: &SpecContentHash,
) -> SpecContentHash {
    let mut fields = vec![
        input.workflow_id.as_str().to_owned(),
        input.workflow_version.as_str().to_owned(),
        input.schema_version.as_str().to_owned(),
        input.spec_hash.as_str().to_owned(),
        input.run_id.as_str().to_owned(),
        input.step_id.as_str().to_owned(),
        input.skill_id.as_str().to_owned(),
        input.skill_version.as_str().to_owned(),
        input.correlation_id.as_str().to_owned(),
        executor_binding_commitment.as_str().to_owned(),
    ];
    for (name, value) in &input.values {
        fields.push(name.clone());
        fields.push(value.clone());
    }
    let framed = fields.iter().map(String::as_str).collect::<Vec<_>>();
    hash_fields(
        "workflow-os/trusted-host-skill-invocation-binding/v1",
        &framed,
    )
}

pub(crate) fn expected_window_binding(
    request: &OperationalExecutionWindowOpeningRequest<'_>,
) -> ExpectedWindowBinding {
    ExpectedWindowBinding {
        workflow_id: request.workflow_id.clone(),
        run_id: request.run_id.clone(),
        step_id: request.step_id.clone(),
        subject_actor_id: request.subject_actor_id.clone(),
        immutable_run_bundle: request.immutable_run_bundle.clone(),
        governance_commitment: request.authorization.governance_commitment.clone(),
        authority_commitment: request.authorization.source_authority_commitment.clone(),
        cursor: request.expected_cursor.clone(),
    }
}

pub(crate) fn committed_window_binding(
    request: &OperationalExecutionWindowOpeningRequest<'_>,
    cursor: ContinuityCursor,
) -> ExpectedWindowBinding {
    ExpectedWindowBinding {
        cursor,
        ..expected_window_binding(request)
    }
}

pub(crate) fn authorization_operation_binding_commitment(
    authorization: &OperationalExecutionWindowOpeningAuthorization,
) -> &SpecContentHash {
    &authorization.operation_binding_commitment
}

pub(crate) fn authorization_source_commitment(
    authorization: &OperationalExecutionWindowOpeningAuthorization,
) -> &SpecContentHash {
    &authorization.source_authority_commitment
}

fn validate_identifier(label: &str, value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty()
        || value.len() > IDENTIFIER_MAX_BYTES
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/'))
    {
        return Err(opening_error(
            WorkflowOsErrorKind::Validation,
            "identifier.invalid",
            &format!("{label} is invalid"),
        ));
    }
    let lowercase = value.to_ascii_lowercase();
    if [
        "authorization",
        "bearer",
        "private_key",
        "private-key",
        "api_token",
        "api-token",
        "secret",
        "token",
    ]
    .iter()
    .any(|needle| lowercase.contains(needle))
    {
        return Err(opening_error(
            WorkflowOsErrorKind::Validation,
            "identifier.secret_like",
            &format!("{label} contains sensitive-looking text"),
        ));
    }
    Ok(())
}

fn hash_fields(domain: &str, fields: &[&str]) -> SpecContentHash {
    let mut hasher = Sha256::new();
    frame(&mut hasher, "domain", domain);
    for (index, field) in fields.iter().enumerate() {
        frame(&mut hasher, &format!("field-{index}"), field);
    }
    SpecContentHash::from_bytes(hasher.finalize())
}

fn frame(hasher: &mut Sha256, label: &str, value: &str) {
    hasher.update(label.len().to_be_bytes());
    hasher.update(label.as_bytes());
    hasher.update(value.len().to_be_bytes());
    hasher.update(value.as_bytes());
}

fn opening_unsupported() -> WorkflowOsError {
    opening_error(
        WorkflowOsErrorKind::Unsupported,
        "unsupported",
        "operational execution-window opening is unsupported by this backend",
    )
}

pub(crate) fn opening_error(
    kind: WorkflowOsErrorKind,
    suffix: &'static str,
    message: &str,
) -> WorkflowOsError {
    WorkflowOsError::new(
        kind,
        format!("operational_execution_window_opening.{suffix}"),
        message,
    )
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;

    fn projection_event() -> OperationalExecutionWindowOpeningProjectionEvent {
        OperationalExecutionWindowOpeningProjectionEvent::new(
            OperationalExecutionWindowOpeningProjectionEventDefinition {
                operation_id: OperationalExecutionWindowOpeningOperationId::new("opening/privacy")
                    .expect("operation"),
                receipt_id: OperationalExecutionWindowOpeningReceiptId::new("receipt/privacy")
                    .expect("receipt"),
                window_id: AuthorizedExecutionWindowId::new("window/privacy").expect("window"),
                attempt_id: AuthorizedExecutionAttemptId::new("attempt/privacy").expect("attempt"),
                attempt_number: 1,
                window_revision: 1,
                operation_binding_commitment: SpecContentHash::from_text("operation binding"),
                request_commitment: SpecContentHash::from_text("request"),
                expected_input_cursor: OperationalExecutionWindowOpeningProjectionCursor::new(
                    EventSequenceNumber::new(3).expect("sequence"),
                    EventId::new("event/privacy/3").expect("event"),
                ),
                committed_result_cursor: OperationalExecutionWindowOpeningProjectionCursor::new(
                    EventSequenceNumber::new(4).expect("sequence"),
                    operational_opening_event_id(
                        &OperationalExecutionWindowOpeningOperationId::new("opening/privacy")
                            .expect("operation"),
                    )
                    .expect("event"),
                ),
            },
        )
        .expect("projection")
    }

    #[test]
    fn opening_identifiers_reject_secret_like_values_without_leakage() {
        let marker = "api_token_super_secret_marker";
        let error = OperationalExecutionWindowOpeningOperationId::new(marker)
            .expect_err("secret-like identifier rejected");

        assert_eq!(
            error.code(),
            "operational_execution_window_opening.identifier.secret_like"
        );
        assert!(!format!("{error:?}").contains(marker));
        assert!(!error.to_string().contains(marker));
    }

    #[test]
    fn projection_debug_is_redacted_and_serialization_is_payload_free() {
        let event = projection_event();
        let debug = format!("{event:?}");
        let serialized = serde_json::to_string(&event).expect("serialize");

        assert!(!debug.contains("opening/privacy"));
        assert!(!debug.contains("receipt/privacy"));
        for forbidden in [
            "raw_payload",
            "command_output",
            "provider_payload",
            "authorization_header",
            "private_key",
        ] {
            assert!(!serialized.contains(forbidden));
        }
    }

    #[test]
    fn projection_deserialization_revalidates_commitment_and_cursor() {
        let event = projection_event();
        let original = serde_json::to_value(&event).expect("serialize");
        let mut tampered = Vec::new();

        let mut attempt = original.clone();
        attempt["attempt_number"] = serde_json::json!(2);
        tampered.push(attempt);
        let mut revision = original.clone();
        revision["window_revision"] = serde_json::json!(2);
        tampered.push(revision);
        let mut binding = original.clone();
        binding["operation_binding_commitment"] =
            serde_json::json!(SpecContentHash::from_text("tampered binding").as_str());
        tampered.push(binding);
        let mut request = original.clone();
        request["request_commitment"] =
            serde_json::json!(SpecContentHash::from_text("tampered request").as_str());
        tampered.push(request);
        let mut projection = original.clone();
        projection["projection_commitment"] =
            serde_json::json!(SpecContentHash::from_text("tampered projection").as_str());
        tampered.push(projection);
        let mut sequence = original.clone();
        sequence["committed_result_cursor"]["sequence_number"] = serde_json::json!(9);
        tampered.push(sequence);
        let mut event_id = original;
        event_id["committed_result_cursor"]["event_id"] = serde_json::json!("event/tampered");
        tampered.push(event_id);

        for value in tampered {
            let error =
                serde_json::from_value::<OperationalExecutionWindowOpeningProjectionEvent>(value)
                    .expect_err("tampered projection rejected");
            assert!(!error.to_string().contains("opening/privacy"));
            assert!(error.to_string().contains("invalid"));
        }
    }

    #[test]
    fn invalid_serialized_identifier_fails_without_echoing_value() {
        let marker = "bearer_super_secret_marker";
        let mut value = serde_json::to_value(projection_event()).expect("serialize");
        value["operation_id"] = serde_json::json!(marker);
        let error =
            serde_json::from_value::<OperationalExecutionWindowOpeningProjectionEvent>(value)
                .expect_err("invalid identifier rejected");

        assert!(!error.to_string().contains(marker));
    }
}
