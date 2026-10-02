use std::fmt;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::authorized_execution_continuity_state::internal::{
    AttemptUseCapability, ContinuityRevision, ExpectedWindowBinding,
};
use crate::{
    AuthorizedExecutionAttemptId, AuthorizedExecutionWindowId, EventId, EventSequenceNumber,
    SpecContentHash, Timestamp, WorkflowOsError, WorkflowOsErrorKind,
};

const IDENTIFIER_MAX_BYTES: usize = 128;

macro_rules! bounded_id {
    ($name:ident, $label:literal) => {
        #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
        #[serde(try_from = "String", into = "String")]
        pub(crate) struct $name(String);

        impl $name {
            pub(crate) fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
                let value = value.into();
                validate_identifier($label, &value)?;
                Ok(Self(value))
            }

            pub(crate) fn as_str(&self) -> &str {
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
    DispatchReservationOperationId,
    "dispatch reservation operation id"
);
bounded_id!(
    DispatchReservationReceiptId,
    "dispatch reservation receipt id"
);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DispatchAdmissionProjectionVersion {
    V1,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DispatchAdmissionProjectionCursor {
    sequence_number: EventSequenceNumber,
    event_id: EventId,
}

impl DispatchAdmissionProjectionCursor {
    pub(crate) const fn new(sequence_number: EventSequenceNumber, event_id: EventId) -> Self {
        Self {
            sequence_number,
            event_id,
        }
    }

    pub(crate) const fn sequence_number(&self) -> EventSequenceNumber {
        self.sequence_number
    }

    pub(crate) const fn event_id(&self) -> &EventId {
        &self.event_id
    }
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "DispatchAdmissionProjectionEventWire")]
pub struct DispatchAdmissionProjectionEvent {
    version: DispatchAdmissionProjectionVersion,
    operation_id: DispatchReservationOperationId,
    receipt_id: DispatchReservationReceiptId,
    window_id: AuthorizedExecutionWindowId,
    attempt_id: AuthorizedExecutionAttemptId,
    window_revision: u64,
    attempt_revision: u64,
    request_commitment: SpecContentHash,
    invocation_commitment: SpecContentHash,
    executor_commitment: SpecContentHash,
    authority_commitment: SpecContentHash,
    governance_commitment: SpecContentHash,
    trusted_time_commitment: SpecContentHash,
    reservation_commitment: SpecContentHash,
    projection_commitment: SpecContentHash,
    expected_input_cursor: DispatchAdmissionProjectionCursor,
    committed_result_cursor: DispatchAdmissionProjectionCursor,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DispatchAdmissionProjectionEventWire {
    version: DispatchAdmissionProjectionVersion,
    operation_id: DispatchReservationOperationId,
    receipt_id: DispatchReservationReceiptId,
    window_id: AuthorizedExecutionWindowId,
    attempt_id: AuthorizedExecutionAttemptId,
    window_revision: u64,
    attempt_revision: u64,
    request_commitment: SpecContentHash,
    invocation_commitment: SpecContentHash,
    executor_commitment: SpecContentHash,
    authority_commitment: SpecContentHash,
    governance_commitment: SpecContentHash,
    trusted_time_commitment: SpecContentHash,
    reservation_commitment: SpecContentHash,
    projection_commitment: SpecContentHash,
    expected_input_cursor: DispatchAdmissionProjectionCursor,
    committed_result_cursor: DispatchAdmissionProjectionCursor,
}

pub(crate) struct DispatchAdmissionProjectionEventDefinition {
    pub(crate) operation_id: DispatchReservationOperationId,
    pub(crate) receipt_id: DispatchReservationReceiptId,
    pub(crate) window_id: AuthorizedExecutionWindowId,
    pub(crate) attempt_id: AuthorizedExecutionAttemptId,
    pub(crate) window_revision: u64,
    pub(crate) attempt_revision: u64,
    pub(crate) request_commitment: SpecContentHash,
    pub(crate) invocation_commitment: SpecContentHash,
    pub(crate) executor_commitment: SpecContentHash,
    pub(crate) authority_commitment: SpecContentHash,
    pub(crate) governance_commitment: SpecContentHash,
    pub(crate) trusted_time_commitment: SpecContentHash,
    pub(crate) reservation_commitment: SpecContentHash,
    pub(crate) expected_input_cursor: DispatchAdmissionProjectionCursor,
    pub(crate) committed_result_cursor: DispatchAdmissionProjectionCursor,
}

impl DispatchAdmissionProjectionEvent {
    pub(crate) fn new(
        definition: DispatchAdmissionProjectionEventDefinition,
    ) -> Result<Self, WorkflowOsError> {
        if definition.window_revision == 0
            || definition.attempt_revision == 0
            || definition.committed_result_cursor.sequence_number().get()
                != definition
                    .expected_input_cursor
                    .sequence_number()
                    .get()
                    .checked_add(1)
                    .ok_or_else(projection_invalid)?
            || definition.committed_result_cursor.event_id()
                != &dispatch_admission_event_id(&definition.operation_id)?
        {
            return Err(projection_invalid());
        }
        let projection_commitment = projection_commitment(&definition);
        Ok(Self {
            version: DispatchAdmissionProjectionVersion::V1,
            operation_id: definition.operation_id,
            receipt_id: definition.receipt_id,
            window_id: definition.window_id,
            attempt_id: definition.attempt_id,
            window_revision: definition.window_revision,
            attempt_revision: definition.attempt_revision,
            request_commitment: definition.request_commitment,
            invocation_commitment: definition.invocation_commitment,
            executor_commitment: definition.executor_commitment,
            authority_commitment: definition.authority_commitment,
            governance_commitment: definition.governance_commitment,
            trusted_time_commitment: definition.trusted_time_commitment,
            reservation_commitment: definition.reservation_commitment,
            projection_commitment,
            expected_input_cursor: definition.expected_input_cursor,
            committed_result_cursor: definition.committed_result_cursor,
        })
    }

    pub(crate) const fn receipt_id(&self) -> &DispatchReservationReceiptId {
        &self.receipt_id
    }

    pub(crate) const fn attempt_id(&self) -> &AuthorizedExecutionAttemptId {
        &self.attempt_id
    }

    pub(crate) const fn reservation_commitment(&self) -> &SpecContentHash {
        &self.reservation_commitment
    }

    pub(crate) const fn projection_commitment(&self) -> &SpecContentHash {
        &self.projection_commitment
    }

    pub(crate) const fn committed_result_cursor(&self) -> &DispatchAdmissionProjectionCursor {
        &self.committed_result_cursor
    }
}

impl TryFrom<DispatchAdmissionProjectionEventWire> for DispatchAdmissionProjectionEvent {
    type Error = WorkflowOsError;

    fn try_from(wire: DispatchAdmissionProjectionEventWire) -> Result<Self, Self::Error> {
        if wire.version != DispatchAdmissionProjectionVersion::V1 {
            return Err(projection_invalid());
        }
        let expected = wire.projection_commitment.clone();
        let event = Self::new(DispatchAdmissionProjectionEventDefinition {
            operation_id: wire.operation_id,
            receipt_id: wire.receipt_id,
            window_id: wire.window_id,
            attempt_id: wire.attempt_id,
            window_revision: wire.window_revision,
            attempt_revision: wire.attempt_revision,
            request_commitment: wire.request_commitment,
            invocation_commitment: wire.invocation_commitment,
            executor_commitment: wire.executor_commitment,
            authority_commitment: wire.authority_commitment,
            governance_commitment: wire.governance_commitment,
            trusted_time_commitment: wire.trusted_time_commitment,
            reservation_commitment: wire.reservation_commitment,
            expected_input_cursor: wire.expected_input_cursor,
            committed_result_cursor: wire.committed_result_cursor,
        })?;
        if event.projection_commitment != expected {
            return Err(projection_invalid());
        }
        Ok(event)
    }
}

impl fmt::Debug for DispatchAdmissionProjectionEvent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DispatchAdmissionProjectionEvent")
            .field("version", &self.version)
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DispatchAdmissionProjectionSnapshot {
    version: DispatchAdmissionProjectionVersion,
    operation_id: DispatchReservationOperationId,
    window_id: AuthorizedExecutionWindowId,
    attempt_id: AuthorizedExecutionAttemptId,
    reservation_commitment: SpecContentHash,
    projection_commitment: SpecContentHash,
    committed_result_cursor: DispatchAdmissionProjectionCursor,
}

impl DispatchAdmissionProjectionSnapshot {
    pub(crate) fn from_event(event: &DispatchAdmissionProjectionEvent) -> Self {
        Self {
            version: event.version,
            operation_id: event.operation_id.clone(),
            window_id: event.window_id.clone(),
            attempt_id: event.attempt_id.clone(),
            reservation_commitment: event.reservation_commitment.clone(),
            projection_commitment: event.projection_commitment.clone(),
            committed_result_cursor: event.committed_result_cursor.clone(),
        }
    }
}

impl fmt::Debug for DispatchAdmissionProjectionSnapshot {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DispatchAdmissionProjectionSnapshot")
            .field("version", &self.version)
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

pub(crate) struct DispatchReservationRequest {
    pub(crate) operation_id: DispatchReservationOperationId,
    pub(crate) receipt_id: DispatchReservationReceiptId,
    pub(crate) request_commitment: SpecContentHash,
    pub(crate) capability: AttemptUseCapability,
    pub(crate) expected_window_binding: ExpectedWindowBinding,
    pub(crate) operation_binding_commitment: SpecContentHash,
    pub(crate) invocation_commitment: SpecContentHash,
    pub(crate) executor_commitment: SpecContentHash,
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct DispatchReservationRecord {
    pub(crate) operation_id: DispatchReservationOperationId,
    pub(crate) receipt_id: DispatchReservationReceiptId,
    pub(crate) window_id: AuthorizedExecutionWindowId,
    pub(crate) attempt_id: AuthorizedExecutionAttemptId,
    pub(crate) window_revision: ContinuityRevision,
    pub(crate) attempt_revision: ContinuityRevision,
    pub(crate) request_commitment: SpecContentHash,
    pub(crate) reservation_commitment: SpecContentHash,
    pub(crate) committed_at: Timestamp,
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct DispatchReservationBinding {
    pub(crate) receipt_id: DispatchReservationReceiptId,
    pub(crate) reservation_commitment: SpecContentHash,
    pub(crate) admission_cursor: DispatchAdmissionProjectionCursor,
}

impl fmt::Debug for DispatchReservationBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DispatchReservationBinding")
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

pub(crate) struct ReservedAttemptDispatchCapability {
    attempt_capability: AttemptUseCapability,
    expected_window_binding: ExpectedWindowBinding,
    operation_binding_commitment: SpecContentHash,
}

impl ReservedAttemptDispatchCapability {
    pub(crate) fn into_parts(self) -> ReservedAttemptDispatchParts {
        ReservedAttemptDispatchParts {
            attempt_capability: self.attempt_capability,
            expected_window_binding: self.expected_window_binding,
            operation_binding_commitment: self.operation_binding_commitment,
        }
    }
}

pub(crate) fn admitted_capability(
    attempt_capability: AttemptUseCapability,
    expected_window_binding: ExpectedWindowBinding,
    operation_binding_commitment: SpecContentHash,
    receipt_id: DispatchReservationReceiptId,
    reservation_commitment: SpecContentHash,
    admission_cursor: DispatchAdmissionProjectionCursor,
) -> ReservedAttemptDispatchCapability {
    let mut attempt_capability = attempt_capability;
    attempt_capability.dispatch_reservation = Some(DispatchReservationBinding {
        receipt_id,
        reservation_commitment,
        admission_cursor,
    });
    ReservedAttemptDispatchCapability {
        attempt_capability,
        expected_window_binding,
        operation_binding_commitment,
    }
}

impl fmt::Debug for ReservedAttemptDispatchCapability {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ReservedAttemptDispatchCapability")
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

pub(crate) struct ReservedAttemptDispatchParts {
    pub(crate) attempt_capability: AttemptUseCapability,
    pub(crate) expected_window_binding: ExpectedWindowBinding,
    pub(crate) operation_binding_commitment: SpecContentHash,
}

pub(crate) struct DispatchReservationReplayReceipt {
    receipt_id: DispatchReservationReceiptId,
    reservation_commitment: SpecContentHash,
    admission_cursor: DispatchAdmissionProjectionCursor,
}

impl DispatchReservationReplayReceipt {
    pub(crate) fn new(
        receipt_id: DispatchReservationReceiptId,
        reservation_commitment: SpecContentHash,
        admission_cursor: DispatchAdmissionProjectionCursor,
    ) -> Self {
        Self {
            receipt_id,
            reservation_commitment,
            admission_cursor,
        }
    }

    pub(crate) fn into_binding(self) -> DispatchReservationBinding {
        DispatchReservationBinding {
            receipt_id: self.receipt_id,
            reservation_commitment: self.reservation_commitment,
            admission_cursor: self.admission_cursor,
        }
    }
}

impl fmt::Debug for DispatchReservationReplayReceipt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DispatchReservationReplayReceipt")
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

pub(crate) enum DispatchReservationOutcome {
    Admitted {
        capability: Box<ReservedAttemptDispatchCapability>,
    },
    AlreadyAdmitted {
        receipt: DispatchReservationReplayReceipt,
    },
    CommittedButCapabilityUnavailable {
        receipt: DispatchReservationReplayReceipt,
    },
}

pub(crate) trait DispatchReservationStore {
    fn reserve_attempt_dispatch(
        &self,
        request: DispatchReservationRequest,
    ) -> Result<DispatchReservationOutcome, WorkflowOsError>;
}

pub(crate) fn request_commitment(request: &DispatchReservationRequest) -> SpecContentHash {
    hash_fields(
        "workflow-os/dispatch-reservation/request/v1",
        &[
            request.operation_id.as_str(),
            request.receipt_id.as_str(),
            request.capability.window_id.as_str(),
            request.capability.attempt_id.as_str(),
            &request.capability.window_revision.get().to_string(),
            request.capability.cursor.event_id.as_str(),
            &request.capability.cursor.sequence_number.get().to_string(),
            request.capability.authority_commitment.as_str(),
            request.capability.window_binding_commitment.as_str(),
            request.operation_binding_commitment.as_str(),
            request.invocation_commitment.as_str(),
            request.executor_commitment.as_str(),
        ],
    )
}

pub(crate) fn reservation_commitment(
    request: &DispatchReservationRequest,
    attempt_revision: ContinuityRevision,
    trusted_time_commitment: &SpecContentHash,
) -> SpecContentHash {
    hash_fields(
        "workflow-os/dispatch-reservation/admission/v1",
        &[
            request.request_commitment.as_str(),
            &attempt_revision.get().to_string(),
            trusted_time_commitment.as_str(),
        ],
    )
}

pub(crate) fn dispatch_admission_event_id(
    operation_id: &DispatchReservationOperationId,
) -> Result<EventId, WorkflowOsError> {
    EventId::new(format!(
        "dispatch-admission/{}",
        short_hash(operation_id.as_str())
    ))
}

fn projection_commitment(
    definition: &DispatchAdmissionProjectionEventDefinition,
) -> SpecContentHash {
    hash_fields(
        "workflow-os/dispatch-reservation/projection/v1",
        &[
            definition.operation_id.as_str(),
            definition.receipt_id.as_str(),
            definition.window_id.as_str(),
            definition.attempt_id.as_str(),
            &definition.window_revision.to_string(),
            &definition.attempt_revision.to_string(),
            definition.request_commitment.as_str(),
            definition.invocation_commitment.as_str(),
            definition.executor_commitment.as_str(),
            definition.authority_commitment.as_str(),
            definition.governance_commitment.as_str(),
            definition.trusted_time_commitment.as_str(),
            definition.reservation_commitment.as_str(),
            &definition
                .expected_input_cursor
                .sequence_number()
                .get()
                .to_string(),
            definition.expected_input_cursor.event_id().as_str(),
            &definition
                .committed_result_cursor
                .sequence_number()
                .get()
                .to_string(),
            definition.committed_result_cursor.event_id().as_str(),
        ],
    )
}

fn validate_identifier(label: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    let lower = value.to_ascii_lowercase();
    if value.is_empty()
        || value.len() > IDENTIFIER_MAX_BYTES
        || !value.is_ascii()
        || value.chars().any(char::is_whitespace)
        || ["secret", "token", "authorization", "private_key", "bearer"]
            .iter()
            .any(|needle| lower.contains(needle))
    {
        return Err(reservation_error(
            WorkflowOsErrorKind::Validation,
            "identifier.invalid",
            format!("{label} is invalid"),
        ));
    }
    Ok(())
}

fn hash_fields(domain: &str, fields: &[&str]) -> SpecContentHash {
    let mut hasher = Sha256::new();
    frame(&mut hasher, "version", "v1");
    frame(&mut hasher, "domain", domain);
    for (index, value) in fields.iter().enumerate() {
        frame(&mut hasher, &format!("field-{index}"), value);
    }
    SpecContentHash::from_bytes(hasher.finalize())
}

fn frame(hasher: &mut Sha256, label: &str, value: &str) {
    hasher.update(label.len().to_be_bytes());
    hasher.update(label.as_bytes());
    hasher.update(value.len().to_be_bytes());
    hasher.update(value.as_bytes());
}

fn short_hash(value: &str) -> String {
    use std::fmt::Write as _;

    let digest = Sha256::digest(value.as_bytes());
    let mut rendered = String::with_capacity(24);
    for byte in &digest[..12] {
        let _ = write!(rendered, "{byte:02x}");
    }
    rendered
}

fn projection_invalid() -> WorkflowOsError {
    reservation_error(
        WorkflowOsErrorKind::Validation,
        "projection.invalid",
        "dispatch admission projection is invalid",
    )
}

pub(crate) fn reservation_error(
    kind: WorkflowOsErrorKind,
    suffix: &'static str,
    message: impl Into<String>,
) -> WorkflowOsError {
    WorkflowOsError::new(kind, format!("dispatch_reservation.{suffix}"), message)
}
