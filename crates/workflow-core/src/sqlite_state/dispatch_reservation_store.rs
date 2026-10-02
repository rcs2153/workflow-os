use rusqlite::{params, OptionalExtension, TransactionBehavior};
use serde::{Deserialize, Serialize};

#[cfg(test)]
use std::cell::Cell;

use crate::authorized_execution_continuity_state::internal::{
    trusted_time_commitment, window_binding_commitment, AuthoritativeAttemptState,
    AuthoritativeWindowState, ContinuityInstanceEligibility, ContinuityRevision,
    TrustedTimePosture,
};
use crate::dispatch_reservation::{
    admitted_capability, dispatch_admission_event_id, request_commitment, reservation_commitment,
    reservation_error, DispatchAdmissionProjectionCursor, DispatchAdmissionProjectionEvent,
    DispatchAdmissionProjectionEventDefinition, DispatchReservationBinding,
    DispatchReservationOutcome, DispatchReservationRecord, DispatchReservationReplayReceipt,
    DispatchReservationRequest, DispatchReservationStore,
};
use crate::{
    IdempotencyKey, WorkflowOsError, WorkflowOsErrorKind, WorkflowRun, WorkflowRunEvent,
    WorkflowRunEventKind, WorkflowRunSnapshot,
};

use super::continuity_codec::timestamp_parts;
use super::{
    append_event_and_project_snapshot, encode_json, snapshot_commitment, SqliteStateBackend,
};
use super::{continuity_codec, continuity_store};

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
struct ProjectionBinding {
    event: DispatchAdmissionProjectionEvent,
    snapshot_commitment: crate::SpecContentHash,
}

#[cfg(test)]
thread_local! {
    static INJECTED_COMMIT_FAULT: Cell<Option<InjectedDispatchCommitFault>> = const { Cell::new(None) };
}

#[cfg(test)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub(super) enum InjectedDispatchCommitFault {
    Before,
    After,
}

#[cfg(test)]
pub(super) fn inject_dispatch_commit_fault(fault: InjectedDispatchCommitFault) {
    INJECTED_COMMIT_FAULT.with(|slot| slot.set(Some(fault)));
}

impl DispatchReservationStore for SqliteStateBackend {
    #[allow(clippy::too_many_lines)]
    fn reserve_attempt_dispatch(
        &self,
        mut request: DispatchReservationRequest,
    ) -> Result<DispatchReservationOutcome, WorkflowOsError> {
        let expected_request = request_commitment(&request);
        if request.request_commitment != expected_request {
            return Err(reservation_error(
                WorkflowOsErrorKind::Validation,
                "request.invalid",
                "dispatch reservation request is invalid",
            ));
        }
        let observation = continuity_store::observe_continuity_trusted_time()?;
        let mut connection = self.connection()?;
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| storage_error("begin_failed"))?;

        if let Some((record, binding)) = read_existing(&transaction, request.operation_id.as_str())?
        {
            if record.request_commitment != request.request_commitment
                || record.receipt_id != request.receipt_id
            {
                return Err(reservation_error(
                    WorkflowOsErrorKind::Security,
                    "idempotency_conflict",
                    "dispatch reservation identity was reused with different content",
                ));
            }
            validate_existing_projection(&transaction, &record, &binding)?;
            transaction
                .commit()
                .map_err(|_| storage_error("commit_failed"))?;
            return Ok(DispatchReservationOutcome::AlreadyAdmitted {
                receipt: replay_receipt(&record, &binding),
            });
        }

        let mut state = continuity_codec::load_snapshot(&transaction)?;
        let window = state
            .windows
            .get_mut(&request.capability.window_id)
            .ok_or_else(not_dispatchable)?;
        let attempt = state
            .attempts
            .get_mut(&request.capability.attempt_id)
            .ok_or_else(not_dispatchable)?;
        if state.trusted_time.eligibility != ContinuityInstanceEligibility::LiveStateEligible
            || state.trusted_time.posture == TrustedTimePosture::Quarantined
            || observation.source() != state.trusted_time.source
            || observation.provenance_commitment() != &state.trusted_time.provenance_commitment
            || observation.epoch_id() != &state.trusted_time.epoch_id
            || state
                .trusted_time
                .last_observed_at
                .is_some_and(|prior| observation.observed_at() < prior)
            || observation.observed_at() >= window.expires_at
            || window.state != AuthoritativeWindowState::Executing
            || attempt.state != AuthoritativeAttemptState::Started
            || window.revision != request.capability.window_revision
            || attempt.revision != ContinuityRevision::new(1)?
            || window.cursor != request.capability.cursor
            || attempt.cursor != request.capability.cursor
            || window.workflow_id != request.expected_window_binding.workflow_id
            || window.run_id != request.expected_window_binding.run_id
            || window.step_id != request.expected_window_binding.step_id
            || window.subject_actor_id != request.expected_window_binding.subject_actor_id
            || window.immutable_run_bundle != request.expected_window_binding.immutable_run_bundle
            || window.governance_commitment != request.expected_window_binding.governance_commitment
            || window.authority_commitment != request.expected_window_binding.authority_commitment
            || request.capability.subject_actor_id != window.subject_actor_id
            || request.capability.authority_commitment != window.authority_commitment
            || request.capability.window_binding_commitment
                != window_binding_commitment(&request.expected_window_binding)
            || attempt.window_id != window.window_id
            || attempt.subject_actor_id != window.subject_actor_id
            || attempt.authority_commitment != window.authority_commitment
            || attempt.consume_operation_id != request.capability.consume_operation_id
        {
            return Err(not_dispatchable());
        }
        let conflicting: i64 = transaction
            .query_row(
                "SELECT COUNT(*) FROM dispatch_reservations WHERE attempt_id=?1",
                params![request.capability.attempt_id.as_str()],
                |row| row.get(0),
            )
            .map_err(|_| storage_error("read_failed"))?;
        if conflicting != 0 {
            return Err(not_dispatchable());
        }

        let history = SqliteStateBackend::read_events_with_connection(
            &transaction,
            &request.expected_window_binding.run_id,
        )?;
        let last = history.last().ok_or_else(corrupt)?;
        if last.sequence_number != request.capability.cursor.sequence_number
            || last.event_id != request.capability.cursor.event_id
        {
            return Err(not_dispatchable());
        }
        let next_sequence = crate::EventSequenceNumber::new(
            last.sequence_number
                .get()
                .checked_add(1)
                .ok_or_else(corrupt)?,
        )?;
        let event_id = dispatch_admission_event_id(&request.operation_id)?;
        let admission_cursor =
            DispatchAdmissionProjectionCursor::new(next_sequence, event_id.clone());
        let trusted_commitment = trusted_time_commitment(&observation);
        let next_revision = request.capability.window_revision.checked_next()?;
        let reservation_binding =
            reservation_commitment(&request, attempt.revision, &trusted_commitment);
        let event =
            DispatchAdmissionProjectionEvent::new(DispatchAdmissionProjectionEventDefinition {
                operation_id: request.operation_id.clone(),
                receipt_id: request.receipt_id.clone(),
                window_id: request.capability.window_id.clone(),
                attempt_id: request.capability.attempt_id.clone(),
                window_revision: next_revision.get(),
                attempt_revision: next_revision.get(),
                request_commitment: request.request_commitment.clone(),
                invocation_commitment: request.invocation_commitment.clone(),
                executor_commitment: request.executor_commitment.clone(),
                authority_commitment: window.authority_commitment.clone(),
                governance_commitment: window.governance_commitment.clone(),
                trusted_time_commitment: trusted_commitment,
                reservation_commitment: reservation_binding.clone(),
                expected_input_cursor: DispatchAdmissionProjectionCursor::new(
                    last.sequence_number,
                    last.event_id.clone(),
                ),
                committed_result_cursor: admission_cursor.clone(),
            })?;

        let continuity_cursor =
            crate::authorized_execution_continuity_state::internal::ContinuityCursor {
                sequence_number: next_sequence,
                event_id: event_id.clone(),
            };
        window.cursor = continuity_cursor.clone();
        window.revision = next_revision;
        window.trusted_time_watermark = observation.observed_at();
        attempt.cursor = continuity_cursor.clone();
        attempt.revision = next_revision;
        state.trusted_time.last_observed_at = Some(observation.observed_at());
        state.trusted_time.posture = TrustedTimePosture::Healthy;
        state.trusted_time.revision = state.trusted_time.revision.checked_next()?;
        continuity_store::persist_trusted_time(&transaction, &state.trusted_time)?;
        continuity_store::persist_window(&transaction, window)?;
        continuity_store::persist_attempt(&transaction, attempt)?;

        let workflow_event = WorkflowRunEvent {
            sequence_number: next_sequence,
            event_id,
            timestamp: observation.observed_at(),
            run_id: request.expected_window_binding.run_id.clone(),
            workflow_id: request.expected_window_binding.workflow_id.clone(),
            schema_version: last.schema_version.clone(),
            workflow_version: last.workflow_version.clone(),
            spec_content_hash: last.spec_content_hash.clone(),
            correlation_id: None,
            actor: None,
            idempotency_key: Some(IdempotencyKey::new(format!(
                "dispatch-reservation/{}",
                short_hash(request.operation_id.as_str())
            ))?),
            kind: WorkflowRunEventKind::AuthorizedExecutionAttemptDispatchAdmitted(Box::new(
                event.clone(),
            )),
        };
        let snapshot = append_event_and_project_snapshot(&transaction, &workflow_event)?;
        let snapshot_payload = encode_json(&snapshot, "snapshot")?;
        let snapshot_binding = snapshot_commitment(&snapshot_payload);
        let record = DispatchReservationRecord {
            operation_id: request.operation_id.clone(),
            receipt_id: request.receipt_id.clone(),
            window_id: request.capability.window_id.clone(),
            attempt_id: request.capability.attempt_id.clone(),
            window_revision: next_revision,
            attempt_revision: next_revision,
            request_commitment: request.request_commitment.clone(),
            reservation_commitment: reservation_binding.clone(),
            committed_at: observation.observed_at(),
        };
        insert_reservation(&transaction, &record, &event)?;
        insert_projection(
            &transaction,
            &record,
            &ProjectionBinding {
                event: event.clone(),
                snapshot_commitment: snapshot_binding.clone(),
            },
        )?;
        #[cfg(test)]
        let commit_fault = INJECTED_COMMIT_FAULT.with(Cell::take);
        #[cfg(not(test))]
        let commit_fault: Option<()> = None;
        #[cfg(test)]
        if commit_fault == Some(InjectedDispatchCommitFault::Before) {
            return Err(storage_error("write_failed"));
        }
        if transaction.commit().is_err() {
            return reconcile_committed_reservation(
                self,
                request.operation_id.as_str(),
                &request.request_commitment,
                &request.receipt_id,
            );
        }
        #[cfg(test)]
        if commit_fault == Some(InjectedDispatchCommitFault::After) {
            return reconcile_committed_reservation(
                self,
                request.operation_id.as_str(),
                &request.request_commitment,
                &request.receipt_id,
            );
        }
        #[cfg(not(test))]
        let _ = commit_fault;

        request.expected_window_binding.cursor = continuity_cursor.clone();
        request.capability.cursor = continuity_cursor;
        request.capability.window_revision = next_revision;
        request.capability.window_binding_commitment =
            window_binding_commitment(&request.expected_window_binding);
        let capability = admitted_capability(
            request.capability,
            request.expected_window_binding,
            request.operation_binding_commitment,
            request.receipt_id,
            reservation_binding,
            admission_cursor,
        );
        Ok(DispatchReservationOutcome::Admitted {
            capability: Box::new(capability),
        })
    }
}

fn insert_reservation(
    transaction: &rusqlite::Transaction<'_>,
    record: &DispatchReservationRecord,
    event: &DispatchAdmissionProjectionEvent,
) -> Result<(), WorkflowOsError> {
    let (seconds, nanos) = timestamp_parts(record.committed_at);
    let sequence = i64::try_from(event.committed_result_cursor().sequence_number().get())
        .map_err(|_| corrupt())?;
    transaction
        .execute(
            "INSERT INTO dispatch_reservations
             (attempt_id,window_id,operation_id,receipt_id,request_commitment,reservation_commitment,
              result_event_id,result_sequence,committed_seconds,committed_nanos,record_json)
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)",
            params![
                record.attempt_id.as_str(),
                record.window_id.as_str(),
                record.operation_id.as_str(),
                record.receipt_id.as_str(),
                record.request_commitment.as_str(),
                record.reservation_commitment.as_str(),
                event.committed_result_cursor().event_id().as_str(),
                sequence,
                seconds,
                nanos,
                encode_json(record, "dispatch reservation")?
            ],
        )
        .map_err(|_| storage_error("write_failed"))?;
    Ok(())
}

fn insert_projection(
    transaction: &rusqlite::Transaction<'_>,
    record: &DispatchReservationRecord,
    binding: &ProjectionBinding,
) -> Result<(), WorkflowOsError> {
    transaction
        .execute(
            "INSERT INTO dispatch_reservation_projection_bindings
             (operation_id,receipt_id,projection_commitment,snapshot_commitment,binding_json)
             VALUES (?1,?2,?3,?4,?5)",
            params![
                record.operation_id.as_str(),
                record.receipt_id.as_str(),
                binding.event.projection_commitment().as_str(),
                binding.snapshot_commitment.as_str(),
                encode_json(binding, "dispatch reservation projection")?
            ],
        )
        .map_err(|_| storage_error("write_failed"))?;
    Ok(())
}

fn read_existing(
    connection: &rusqlite::Connection,
    operation_id: &str,
) -> Result<Option<(DispatchReservationRecord, ProjectionBinding)>, WorkflowOsError> {
    let record_row = connection
        .query_row(
            "SELECT operation_id,receipt_id,attempt_id,window_id,request_commitment,
                    reservation_commitment,result_event_id,result_sequence,record_json
             FROM dispatch_reservations WHERE operation_id=?1",
            params![operation_id],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                    row.get::<_, String>(5)?,
                    row.get::<_, String>(6)?,
                    row.get::<_, i64>(7)?,
                    row.get::<_, String>(8)?,
                ))
            },
        )
        .optional()
        .map_err(|_| storage_error("read_failed"))?;
    let Some(record_row) = record_row else {
        return Ok(None);
    };
    let binding_row = connection
        .query_row(
            "SELECT operation_id,receipt_id,projection_commitment,snapshot_commitment,binding_json
             FROM dispatch_reservation_projection_bindings WHERE operation_id=?1",
            params![operation_id],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                ))
            },
        )
        .optional()
        .map_err(|_| storage_error("read_failed"))?
        .ok_or_else(corrupt)?;
    let record: DispatchReservationRecord =
        serde_json::from_str(&record_row.8).map_err(|_| corrupt())?;
    let binding: ProjectionBinding = serde_json::from_str(&binding_row.4).map_err(|_| corrupt())?;
    let sequence = i64::try_from(
        binding
            .event
            .committed_result_cursor()
            .sequence_number()
            .get(),
    )
    .map_err(|_| corrupt())?;
    if record_row.0 != record.operation_id.as_str()
        || record_row.1 != record.receipt_id.as_str()
        || record_row.2 != record.attempt_id.as_str()
        || record_row.3 != record.window_id.as_str()
        || record_row.4 != record.request_commitment.as_str()
        || record_row.5 != record.reservation_commitment.as_str()
        || record_row.6 != binding.event.committed_result_cursor().event_id().as_str()
        || record_row.7 != sequence
        || binding_row.0 != record.operation_id.as_str()
        || binding_row.1 != record.receipt_id.as_str()
        || binding_row.2 != binding.event.projection_commitment().as_str()
        || binding_row.3 != binding.snapshot_commitment.as_str()
    {
        return Err(corrupt());
    }
    Ok(Some((record, binding)))
}

fn validate_existing_projection(
    connection: &rusqlite::Connection,
    record: &DispatchReservationRecord,
    binding: &ProjectionBinding,
) -> Result<(), WorkflowOsError> {
    if binding.event.receipt_id() != &record.receipt_id
        || binding.event.attempt_id() != &record.attempt_id
        || binding.event.reservation_commitment() != &record.reservation_commitment
    {
        return Err(corrupt());
    }
    let durable_event_payload = connection
        .query_row(
            "SELECT payload FROM events WHERE event_id=?1",
            params![binding.event.committed_result_cursor().event_id().as_str()],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|_| storage_error("read_failed"))?
        .ok_or_else(corrupt)?;
    let durable_event: WorkflowRunEvent =
        serde_json::from_str(&durable_event_payload).map_err(|_| corrupt())?;
    let durable_projection = match &durable_event.kind {
        WorkflowRunEventKind::AuthorizedExecutionAttemptDispatchAdmitted(value) => value.as_ref(),
        _ => return Err(corrupt()),
    };
    if durable_projection != &binding.event
        || durable_event.sequence_number
            != binding.event.committed_result_cursor().sequence_number()
        || durable_event.event_id != *binding.event.committed_result_cursor().event_id()
    {
        return Err(corrupt());
    }

    let history =
        SqliteStateBackend::read_events_with_connection(connection, &durable_event.run_id)
            .map_err(|_| corrupt())?;
    let result_index = usize::try_from(
        binding
            .event
            .committed_result_cursor()
            .sequence_number()
            .get()
            .checked_sub(1)
            .ok_or_else(corrupt)?,
    )
    .map_err(|_| corrupt())?;
    let expected_index = usize::try_from(
        binding
            .event
            .expected_input_cursor()
            .sequence_number()
            .get()
            .checked_sub(1)
            .ok_or_else(corrupt)?,
    )
    .map_err(|_| corrupt())?;
    let expected_event = history.get(expected_index).ok_or_else(corrupt)?;
    let result_event = history.get(result_index).ok_or_else(corrupt)?;
    if expected_event.event_id != *binding.event.expected_input_cursor().event_id()
        || result_event != &durable_event
    {
        return Err(corrupt());
    }

    let projected_snapshot = WorkflowRun::rehydrate(&history[..=result_index])?.snapshot;
    if projected_snapshot.last_dispatch_admission_projection
        != Some(
            crate::dispatch_reservation::DispatchAdmissionProjectionSnapshot::from_event(
                &binding.event,
            ),
        )
        || snapshot_commitment(&encode_json(&projected_snapshot, "snapshot")?)
            != binding.snapshot_commitment
    {
        return Err(corrupt());
    }

    let current_row: (i64, String, String, String) = connection
        .query_row(
            "SELECT last_sequence_number,last_event_id,snapshot_commitment,payload
             FROM snapshots WHERE run_id=?1",
            params![durable_event.run_id.as_str()],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
        )
        .map_err(|_| corrupt())?;
    let current_snapshot: WorkflowRunSnapshot =
        super::decode_json(&current_row.3, "snapshot").map_err(|_| corrupt())?;
    let derived_snapshot = WorkflowRun::rehydrate(&history)?.snapshot;
    let current_sequence =
        i64::try_from(current_snapshot.last_sequence_number.get()).map_err(|_| corrupt())?;
    if current_snapshot != derived_snapshot
        || current_row.0 != current_sequence
        || current_row.1 != current_snapshot.last_event_id.as_str()
        || current_row.2
            != snapshot_commitment(&encode_json(&current_snapshot, "snapshot")?).as_str()
    {
        return Err(corrupt());
    }
    Ok(())
}

fn reconcile_committed_reservation(
    backend: &SqliteStateBackend,
    operation_id: &str,
    request_commitment: &crate::SpecContentHash,
    receipt_id: &crate::dispatch_reservation::DispatchReservationReceiptId,
) -> Result<DispatchReservationOutcome, WorkflowOsError> {
    let connection = backend.connection()?;
    match read_existing(&connection, operation_id)? {
        Some((record, binding))
            if record.request_commitment == *request_commitment
                && record.receipt_id == *receipt_id =>
        {
            validate_existing_projection(&connection, &record, &binding)?;
            Ok(
                DispatchReservationOutcome::CommittedButCapabilityUnavailable {
                    receipt: replay_receipt(&record, &binding),
                },
            )
        }
        Some(_) => Err(reservation_error(
            WorkflowOsErrorKind::Security,
            "idempotency_conflict",
            "dispatch reservation identity was reused with different content",
        )),
        None => Err(storage_error("commit_ambiguous")),
    }
}

fn replay_receipt(
    record: &DispatchReservationRecord,
    binding: &ProjectionBinding,
) -> DispatchReservationReplayReceipt {
    DispatchReservationReplayReceipt::new(
        record.receipt_id.clone(),
        record.reservation_commitment.clone(),
        binding.event.committed_result_cursor().clone(),
    )
}

pub(super) fn validate_dispatch_reservation_binding(
    connection: &rusqlite::Connection,
    attempt_id: &crate::AuthorizedExecutionAttemptId,
    supplied: &DispatchReservationBinding,
) -> Result<(), WorkflowOsError> {
    let operation_id = connection
        .query_row(
            "SELECT operation_id FROM dispatch_reservations WHERE attempt_id=?1",
            params![attempt_id.as_str()],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|_| storage_error("read_failed"))?
        .ok_or_else(dispatch_binding_missing)?;
    let (record, projection) = read_existing(connection, &operation_id)?.ok_or_else(corrupt)?;
    validate_existing_projection(connection, &record, &projection)?;
    if record.attempt_id != *attempt_id
        || record.receipt_id != supplied.receipt_id
        || record.reservation_commitment != supplied.reservation_commitment
        || projection.event.receipt_id() != &supplied.receipt_id
        || projection.event.attempt_id() != attempt_id
        || projection.event.reservation_commitment() != &supplied.reservation_commitment
        || projection.event.committed_result_cursor() != &supplied.admission_cursor
    {
        return Err(dispatch_binding_invalid());
    }
    Ok(())
}

fn short_hash(value: &str) -> String {
    use sha2::{Digest, Sha256};
    use std::fmt::Write as _;

    let digest = Sha256::digest(value.as_bytes());
    let mut rendered = String::with_capacity(24);
    for byte in &digest[..12] {
        let _ = write!(rendered, "{byte:02x}");
    }
    rendered
}

fn not_dispatchable() -> WorkflowOsError {
    reservation_error(
        WorkflowOsErrorKind::InvalidState,
        "attempt_not_dispatchable",
        "authorized attempt is not dispatchable",
    )
}

fn dispatch_binding_missing() -> WorkflowOsError {
    reservation_error(
        WorkflowOsErrorKind::Security,
        "binding_missing",
        "dispatch reservation binding is required",
    )
}

fn dispatch_binding_invalid() -> WorkflowOsError {
    reservation_error(
        WorkflowOsErrorKind::Security,
        "binding_invalid",
        "dispatch reservation binding is invalid",
    )
}

fn corrupt() -> WorkflowOsError {
    reservation_error(
        WorkflowOsErrorKind::InvalidState,
        "recovery_required",
        "dispatch reservation state requires recovery",
    )
}

fn storage_error(suffix: &'static str) -> WorkflowOsError {
    reservation_error(
        WorkflowOsErrorKind::InvalidState,
        suffix,
        "dispatch reservation storage operation failed",
    )
}
