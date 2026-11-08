use rusqlite::{params, OptionalExtension, Transaction, TransactionBehavior};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
#[cfg(test)]
use std::cell::Cell;
use std::fmt::Write as _;

use crate::authorized_execution_continuity_state::internal::{
    window_binding_commitment, AuthoritativeAttemptRecord, AuthoritativeAttemptState,
    AuthoritativeWindowRecord, AuthoritativeWindowState, ContinuityInstanceEligibility,
    ContinuityOperationId, ContinuityRevision, TrustedTimePosture, TrustedTimeSourceKind,
};
use crate::operational_execution_window_opening::{
    authorization_operation_binding_commitment, authorization_source_commitment,
    committed_window_binding, expected_window_binding, opening_error, opening_request_commitment,
    operational_opening_event_id, run_snapshot_commitment,
    OperationalExecutionAttemptUseCapability, OperationalExecutionWindowOpeningOperationId,
    OperationalExecutionWindowOpeningProjectionCursor,
    OperationalExecutionWindowOpeningProjectionEvent,
    OperationalExecutionWindowOpeningProjectionEventDefinition,
    OperationalExecutionWindowOpeningReceiptId,
    OperationalExecutionWindowOpeningReconciliationResult,
    OperationalExecutionWindowOpeningRecordedResult, OperationalExecutionWindowOpeningRequest,
    OperationalExecutionWindowOpeningResult, OperationalExecutionWindowOpeningStoreV1,
    ProjectedOperationalExecutionWindowOpeningResult,
};
use crate::{
    EventSequenceNumber, IdempotencyKey, SpecContentHash, WorkflowOsError, WorkflowOsErrorKind,
    WorkflowRunEvent, WorkflowRunEventKind, WorkflowRunStatus,
};

use super::live_sandbox_grant_store::{check_opening_lane, verify_live_opening_approval};
use super::{
    append_event_and_project_snapshot, encode_json, map_sqlite_error, snapshot_commitment,
};
use super::{continuity_codec, continuity_store, SqliteStateBackend};
use crate::live_sandbox_continuation_authority::LiveSandboxContinuationFacts;

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
struct OpeningAttemptRecord {
    attempt_id: crate::AuthorizedExecutionAttemptId,
    window_id: crate::AuthorizedExecutionWindowId,
    operation_id: OperationalExecutionWindowOpeningOperationId,
    attempt_number: u32,
    subject_actor_id: crate::ActorId,
    authority_commitment: SpecContentHash,
    operation_binding_commitment: SpecContentHash,
    state: &'static str,
    revision: u64,
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
struct OpeningProjectionBinding {
    event: OperationalExecutionWindowOpeningProjectionEvent,
    snapshot_commitment: SpecContentHash,
}

#[cfg(test)]
thread_local! {
    static INJECTED_COMMIT_FAULT: Cell<Option<InjectedOpeningCommitFault>> = const { Cell::new(None) };
}

#[cfg(test)]
#[derive(Clone, Copy, Eq, PartialEq)]
enum InjectedOpeningCommitFault {
    Before,
    During,
    After,
}

impl OperationalExecutionWindowOpeningStoreV1 for SqliteStateBackend {
    // Keeping this transaction contiguous makes the all-or-nothing security
    // boundary directly auditable.
    #[allow(clippy::too_many_lines)]
    fn open_window_and_start_attempt_projected(
        &self,
        request: OperationalExecutionWindowOpeningRequest<'_>,
    ) -> Result<ProjectedOperationalExecutionWindowOpeningResult, WorkflowOsError> {
        self.open_window_and_start_attempt_inner(request, None, None)
    }

    fn reconcile_operational_opening(
        &self,
        operation_id: &OperationalExecutionWindowOpeningOperationId,
        expected_request_commitment: &SpecContentHash,
        expected_receipt_id: &OperationalExecutionWindowOpeningReceiptId,
    ) -> Result<OperationalExecutionWindowOpeningReconciliationResult, WorkflowOsError> {
        let connection = self.connection()?;
        match read_existing(&connection, operation_id)? {
            Some((result, binding)) => {
                if &result.request_commitment != expected_request_commitment
                    || &result.receipt_id != expected_receipt_id
                {
                    return Err(opening_error(
                        WorkflowOsErrorKind::InvalidState,
                        "reconciliation_mismatch",
                        "operational opening reconciliation binding does not match",
                    ));
                }
                Ok(
                    OperationalExecutionWindowOpeningReconciliationResult::DurablyCommitted {
                        result: Box::new(result),
                        event: Box::new(binding.event),
                        snapshot_commitment: binding.snapshot_commitment,
                    },
                )
            }
            None => Ok(OperationalExecutionWindowOpeningReconciliationResult::ConfirmedAbsent),
        }
    }
}

impl SqliteStateBackend {
    pub(crate) fn open_window_and_start_attempt_with_lifecycle(
        &self,
        request: OperationalExecutionWindowOpeningRequest<'_>,
        lifecycle: &crate::trusted_host_workflow_lifecycle::TrustedHostWorkflowLifecycleContext,
    ) -> Result<ProjectedOperationalExecutionWindowOpeningResult, WorkflowOsError> {
        self.open_window_and_start_attempt_inner(request, Some(lifecycle), None)
    }

    #[allow(dead_code)] // The trusted live resolver is not connected to a public handler.
    pub(crate) fn open_live_sandbox_attempt(
        &self,
        request: OperationalExecutionWindowOpeningRequest<'_>,
        facts: LiveSandboxContinuationFacts<'_>,
    ) -> Result<ProjectedOperationalExecutionWindowOpeningResult, WorkflowOsError> {
        self.open_window_and_start_attempt_inner(request, None, Some(facts))
    }

    // Keeping this transaction contiguous makes the all-or-nothing security
    // boundary directly auditable.
    #[allow(clippy::too_many_lines)]
    fn open_window_and_start_attempt_inner(
        &self,
        request: OperationalExecutionWindowOpeningRequest<'_>,
        lifecycle: Option<
            &crate::trusted_host_workflow_lifecycle::TrustedHostWorkflowLifecycleContext,
        >,
        live: Option<LiveSandboxContinuationFacts<'_>>,
    ) -> Result<ProjectedOperationalExecutionWindowOpeningResult, WorkflowOsError> {
        validate_request(&request)?;
        if request.workflow_lifecycle_commitment
            != lifecycle.map(crate::trusted_host_workflow_lifecycle::TrustedHostWorkflowLifecycleContext::commitment)
        {
            return Err(opening_error(
                WorkflowOsErrorKind::InvalidState,
                "workflow_lifecycle_mismatch",
                "operational opening workflow lifecycle binding does not match",
            ));
        }
        let mut connection = self.connection()?;
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|error| {
                map_sqlite_error(
                    error,
                    "operational_opening.begin_failed",
                    "SQLite operational opening could not start",
                )
            })?;

        check_opening_lane(&transaction, &request, live)?;
        if let Some(replay) = read_existing(&transaction, &request.operation_id)? {
            if replay.0.request_commitment != request.request_commitment
                || replay.0.receipt_id != request.receipt_id
            {
                return Err(opening_error(
                    WorkflowOsErrorKind::InvalidState,
                    "idempotency_conflict",
                    "operational opening identity was reused with different content",
                ));
            }
            transaction.commit().map_err(|error| {
                map_sqlite_error(
                    error,
                    "operational_opening.commit_failed",
                    "SQLite operational opening replay could not commit",
                )
            })?;
            return Ok(ProjectedOperationalExecutionWindowOpeningResult {
                result: OperationalExecutionWindowOpeningResult::ExactReplay(replay.0),
                event: replay.1.event,
                snapshot_commitment: replay.1.snapshot_commitment,
            });
        }

        let history =
            SqliteStateBackend::read_events_with_connection(&transaction, &request.run_id)?;
        let run = crate::WorkflowRun::rehydrate(&history)?;
        validate_runtime_binding(&request, &run)?;
        if let Some(facts) = live {
            verify_live_opening_approval(&transaction, facts, &run)?;
        }

        let active_count: i64 = transaction
            .query_row(
                "SELECT COUNT(*) FROM continuity_windows
                 WHERE workflow_id=?1 AND run_id=?2 AND step_id=?3
                   AND state IN ('executing','yielded','recovery_required')",
                params![
                    request.workflow_id.as_str(),
                    request.run_id.as_str(),
                    request.step_id.as_str()
                ],
                |row| row.get(0),
            )
            .map_err(|_| corrupt())?;
        if active_count != 0 {
            return Err(opening_error(
                WorkflowOsErrorKind::InvalidState,
                "active_window_conflict",
                "an active operational execution window already owns this scope",
            ));
        }

        let mut continuity = continuity_codec::load_snapshot(&transaction)?;
        validate_and_advance_trusted_time(&request, &mut continuity.trusted_time)?;
        let observed_at = request.trusted_time.observed_at();
        let opening_input_cursor = if let Some(lifecycle) = lifecycle {
            crate::authorized_execution_continuity_state::internal::ContinuityCursor {
                sequence_number: crate::EventSequenceNumber::new(
                    request
                        .expected_cursor
                        .sequence_number
                        .get()
                        .checked_add(1)
                        .ok_or_else(corrupt)?,
                )?,
                event_id: lifecycle
                    .event_id("invocation-requested", request.operation_id.as_str())?,
            }
        } else {
            request.expected_cursor.clone()
        };
        let result_cursor = next_cursor(&opening_input_cursor, &request.operation_id)?;
        let window_revision = ContinuityRevision::new(1)?;
        let binding = committed_window_binding(&request, result_cursor.clone());
        let window = AuthoritativeWindowRecord {
            workflow_id: request.workflow_id.clone(),
            run_id: request.run_id.clone(),
            step_id: request.step_id.clone(),
            window_id: request.window_id.clone(),
            subject_actor_id: request.subject_actor_id.clone(),
            immutable_run_bundle: request.immutable_run_bundle.clone(),
            governance_commitment: binding.governance_commitment.clone(),
            authority_commitment: binding.authority_commitment.clone(),
            cursor: result_cursor.clone(),
            state: AuthoritativeWindowState::Executing,
            maximum_attempts: request.maximum_attempts,
            next_attempt_number: 2,
            expires_at: request.expires_at,
            trusted_time_watermark: observed_at,
            trusted_time_epoch_id: request.trusted_time.epoch_id().clone(),
            revision: window_revision,
            active_yield: None,
        };
        continuity_store::persist_trusted_time(&transaction, &continuity.trusted_time)?;
        continuity_store::persist_window(&transaction, &window)?;

        let consume_operation_id = ContinuityOperationId::new(request.operation_id.as_str())?;
        let continuity_attempt = AuthoritativeAttemptRecord {
            attempt_id: request.attempt_id.clone(),
            attempt_number: 1,
            window_id: request.window_id.clone(),
            subject_actor_id: request.subject_actor_id.clone(),
            cursor: result_cursor.clone(),
            authority_commitment: authorization_source_commitment(request.authorization).clone(),
            consume_operation_id,
            state: AuthoritativeAttemptState::Started,
            revision: ContinuityRevision::new(1)?,
        };
        let result = OperationalExecutionWindowOpeningRecordedResult {
            operation_id: request.operation_id.clone(),
            receipt_id: request.receipt_id.clone(),
            workflow_id: request.workflow_id.clone(),
            run_id: request.run_id.clone(),
            step_id: request.step_id.clone(),
            window_id: request.window_id.clone(),
            attempt_id: request.attempt_id.clone(),
            attempt_number: 1,
            window_revision,
            operation_binding_commitment: authorization_operation_binding_commitment(
                request.authorization,
            )
            .clone(),
            request_commitment: request.request_commitment.clone(),
            committed_at: observed_at,
        };
        let event = OperationalExecutionWindowOpeningProjectionEvent::new(
            OperationalExecutionWindowOpeningProjectionEventDefinition {
                operation_id: request.operation_id.clone(),
                receipt_id: request.receipt_id.clone(),
                window_id: request.window_id.clone(),
                attempt_id: request.attempt_id.clone(),
                attempt_number: 1,
                window_revision: 1,
                operation_binding_commitment: result.operation_binding_commitment.clone(),
                request_commitment: request.request_commitment.clone(),
                expected_input_cursor: OperationalExecutionWindowOpeningProjectionCursor::new(
                    opening_input_cursor.sequence_number,
                    opening_input_cursor.event_id.clone(),
                ),
                committed_result_cursor: OperationalExecutionWindowOpeningProjectionCursor::new(
                    result_cursor.sequence_number,
                    result_cursor.event_id.clone(),
                ),
            },
        )?;
        let first = history.first().ok_or_else(corrupt)?;
        if let Some(lifecycle) = lifecycle {
            append_event_and_project_snapshot(
                &transaction,
                &WorkflowRunEvent {
                    sequence_number: opening_input_cursor.sequence_number,
                    event_id: opening_input_cursor.event_id.clone(),
                    timestamp: observed_at,
                    run_id: request.run_id.clone(),
                    workflow_id: request.workflow_id.clone(),
                    schema_version: first.schema_version.clone(),
                    workflow_version: first.workflow_version.clone(),
                    spec_content_hash: first.spec_content_hash.clone(),
                    correlation_id: None,
                    actor: None,
                    idempotency_key: Some(lifecycle.idempotency_key("invocation-requested")?),
                    kind: lifecycle.request_event(),
                },
            )?;
        }
        let workflow_event = WorkflowRunEvent {
            sequence_number: result_cursor.sequence_number,
            event_id: result_cursor.event_id.clone(),
            timestamp: observed_at,
            run_id: request.run_id.clone(),
            workflow_id: request.workflow_id.clone(),
            schema_version: first.schema_version.clone(),
            workflow_version: first.workflow_version.clone(),
            spec_content_hash: first.spec_content_hash.clone(),
            correlation_id: None,
            actor: None,
            idempotency_key: Some(IdempotencyKey::new(format!(
                "operational-opening/{}",
                short_hash(request.operation_id.as_str())
            ))?),
            kind: WorkflowRunEventKind::OperationalExecutionWindowOpened(Box::new(event.clone())),
        };
        let snapshot = append_event_and_project_snapshot(&transaction, &workflow_event)?;
        let snapshot_payload = encode_json(&snapshot, "snapshot")?;
        let snapshot_commitment = snapshot_commitment(&snapshot_payload);

        let attempt = OpeningAttemptRecord {
            attempt_id: request.attempt_id.clone(),
            window_id: request.window_id.clone(),
            operation_id: request.operation_id.clone(),
            attempt_number: 1,
            subject_actor_id: request.subject_actor_id.clone(),
            authority_commitment: authorization_source_commitment(request.authorization).clone(),
            operation_binding_commitment: result.operation_binding_commitment.clone(),
            state: "started",
            revision: 1,
        };
        insert_operation(
            &transaction,
            &request,
            &result,
            &opening_input_cursor,
            &result_cursor,
        )?;
        insert_attempt(&transaction, &attempt)?;
        continuity_store::persist_attempt(&transaction, &continuity_attempt)?;
        let projection_binding = OpeningProjectionBinding {
            event: event.clone(),
            snapshot_commitment: snapshot_commitment.clone(),
        };
        insert_projection_binding(&transaction, &result, &projection_binding)?;

        #[cfg(test)]
        let commit_fault = INJECTED_COMMIT_FAULT.with(Cell::take);
        #[cfg(not(test))]
        let commit_fault: Option<()> = None;

        #[cfg(test)]
        if commit_fault == Some(InjectedOpeningCommitFault::Before) {
            return Err(opening_error(
                WorkflowOsErrorKind::InvalidState,
                "write_failed",
                "SQLite operational opening transaction failed",
            ));
        }

        transaction.commit().map_err(|error| {
            map_sqlite_error(
                error,
                "operational_opening.commit_ambiguous",
                "SQLite operational opening commit outcome is ambiguous",
            )
        })?;

        #[cfg(test)]
        if matches!(
            commit_fault,
            Some(InjectedOpeningCommitFault::During | InjectedOpeningCommitFault::After)
        ) {
            return Err(opening_error(
                WorkflowOsErrorKind::InvalidState,
                "commit_ambiguous",
                "SQLite operational opening commit outcome is ambiguous",
            ));
        }

        #[cfg(not(test))]
        let _ = commit_fault;

        Ok(ProjectedOperationalExecutionWindowOpeningResult {
            result: OperationalExecutionWindowOpeningResult::Opened {
                capability: Box::new(OperationalExecutionAttemptUseCapability {
                    attempt_id: result.attempt_id.clone(),
                    window_id: result.window_id.clone(),
                    subject_actor_id: request.subject_actor_id,
                    window_revision,
                    cursor: result_cursor,
                    authority_commitment: authorization_source_commitment(request.authorization)
                        .clone(),
                    window_binding_commitment: window_binding_commitment(&binding),
                    expected_window_binding: binding,
                    operation_binding_commitment: result.operation_binding_commitment.clone(),
                    consume_operation_id: continuity_attempt.consume_operation_id,
                    opening_operation_id: result.operation_id.clone(),
                }),
                result,
            },
            event,
            snapshot_commitment,
        })
    }
}

fn validate_request(
    request: &OperationalExecutionWindowOpeningRequest<'_>,
) -> Result<(), WorkflowOsError> {
    if request.maximum_attempts == 0 || request.expires_at <= request.trusted_time.observed_at() {
        return Err(opening_error(
            WorkflowOsErrorKind::Validation,
            "request.invalid",
            "operational opening request is invalid",
        ));
    }
    let expected_binding =
        crate::operational_execution_window_opening::operation_binding_commitment(
            &request.workflow_id,
            &request.run_id,
            &request.step_id,
            &request.invocation_binding_commitment,
        );
    if authorization_operation_binding_commitment(request.authorization) != &expected_binding {
        return Err(opening_error(
            WorkflowOsErrorKind::Security,
            "operation_binding_mismatch",
            "operational opening operation binding does not match",
        ));
    }
    let expected_request = opening_request_commitment(request);
    if request.request_commitment != expected_request {
        return Err(opening_error(
            WorkflowOsErrorKind::InvalidState,
            "request_commitment_mismatch",
            "operational opening request commitment does not match",
        ));
    }
    Ok(())
}

fn validate_runtime_binding(
    request: &OperationalExecutionWindowOpeningRequest<'_>,
    run: &crate::WorkflowRun,
) -> Result<(), WorkflowOsError> {
    if run.snapshot.identity.workflow_id != request.workflow_id
        || run.snapshot.identity.run_id != request.run_id
        || run.snapshot.identity.immutable_run_bundle.as_ref()
            != Some(&request.immutable_run_bundle)
        || run.snapshot.last_sequence_number != request.expected_cursor.sequence_number
        || run.snapshot.last_event_id != request.expected_cursor.event_id
        || run_snapshot_commitment(&run.snapshot)? != request.expected_snapshot_commitment
    {
        return Err(opening_error(
            WorkflowOsErrorKind::InvalidState,
            "runtime_binding_stale",
            "operational opening runtime binding is stale",
        ));
    }
    if !matches!(
        run.snapshot.status,
        WorkflowRunStatus::Running | WorkflowRunStatus::Retrying
    ) {
        return Err(opening_error(
            WorkflowOsErrorKind::InvalidState,
            "run_not_eligible",
            "workflow run is not eligible for operational opening",
        ));
    }
    Ok(())
}

fn validate_and_advance_trusted_time(
    request: &OperationalExecutionWindowOpeningRequest<'_>,
    trusted: &mut crate::authorized_execution_continuity_state::internal::TrustedTimeSecurityRecord,
) -> Result<(), WorkflowOsError> {
    if trusted.source != TrustedTimeSourceKind::CoreInjectedClockV1
        || request.trusted_time.source() != trusted.source
        || request.trusted_time.provenance_commitment() != &trusted.provenance_commitment
        || request.trusted_time.epoch_id() != &trusted.epoch_id
        || trusted.eligibility != ContinuityInstanceEligibility::LiveStateEligible
        || trusted.posture == TrustedTimePosture::Quarantined
        || trusted
            .last_observed_at
            .is_some_and(|last| request.trusted_time.observed_at() < last)
    {
        return Err(opening_error(
            WorkflowOsErrorKind::Security,
            "trusted_time_rejected",
            "operational opening trusted time was rejected",
        ));
    }
    trusted.last_observed_at = Some(request.trusted_time.observed_at());
    trusted.posture = TrustedTimePosture::Healthy;
    trusted.revision = trusted.revision.checked_next()?;
    Ok(())
}

fn next_cursor(
    current: &crate::authorized_execution_continuity_state::internal::ContinuityCursor,
    operation_id: &OperationalExecutionWindowOpeningOperationId,
) -> Result<crate::authorized_execution_continuity_state::internal::ContinuityCursor, WorkflowOsError>
{
    let next = current
        .sequence_number
        .get()
        .checked_add(1)
        .ok_or_else(corrupt)?;
    Ok(
        crate::authorized_execution_continuity_state::internal::ContinuityCursor {
            sequence_number: EventSequenceNumber::new(next)?,
            event_id: operational_opening_event_id(operation_id)?,
        },
    )
}

fn insert_operation(
    transaction: &Transaction<'_>,
    request: &OperationalExecutionWindowOpeningRequest<'_>,
    result: &OperationalExecutionWindowOpeningRecordedResult,
    opening_input_cursor: &crate::authorized_execution_continuity_state::internal::ContinuityCursor,
    result_cursor: &crate::authorized_execution_continuity_state::internal::ContinuityCursor,
) -> Result<(), WorkflowOsError> {
    let (seconds, nanos) = timestamp_parts(result.committed_at);
    transaction
        .execute(
            "INSERT INTO operational_opening_operations
         (operation_id,receipt_id,version,request_commitment,workflow_id,run_id,step_id,
          window_id,attempt_id,subject_actor_id,authority_commitment,governance_commitment,
          operation_binding,operation_binding_commitment,expected_snapshot_commitment,
          expected_event_id,expected_sequence,
          result_event_id,result_sequence,attempt_number,window_revision,committed_seconds,
          committed_nanos,record_json)
         VALUES (?1,?2,'v2',?3,?4,?5,?6,?7,?8,?9,?10,?11,
                 'invoke_current_step_skill',?12,?13,?14,?15,?16,?17,1,1,?18,?19,?20)",
            params![
                result.operation_id.as_str(),
                result.receipt_id.as_str(),
                result.request_commitment.as_str(),
                result.workflow_id.as_str(),
                result.run_id.as_str(),
                result.step_id.as_str(),
                result.window_id.as_str(),
                result.attempt_id.as_str(),
                request.subject_actor_id.as_str(),
                authorization_source_commitment(request.authorization).as_str(),
                expected_window_binding(request)
                    .governance_commitment
                    .as_str(),
                result.operation_binding_commitment.as_str(),
                request.expected_snapshot_commitment.as_str(),
                opening_input_cursor.event_id.as_str(),
                i64::try_from(opening_input_cursor.sequence_number.get()).map_err(|_| corrupt())?,
                result_cursor.event_id.as_str(),
                i64::try_from(result_cursor.sequence_number.get()).map_err(|_| corrupt())?,
                seconds,
                nanos,
                encode_json(result, "operational opening result")?
            ],
        )
        .map_err(|_| corrupt())?;
    Ok(())
}

fn insert_attempt(
    transaction: &Transaction<'_>,
    attempt: &OpeningAttemptRecord,
) -> Result<(), WorkflowOsError> {
    transaction
        .execute(
            "INSERT INTO operational_opening_attempts
         (attempt_id,window_id,operation_id,attempt_number,subject_actor_id,
          authority_commitment,operation_binding_commitment,state,revision,record_json)
         VALUES (?1,?2,?3,1,?4,?5,?6,'started',1,?7)",
            params![
                attempt.attempt_id.as_str(),
                attempt.window_id.as_str(),
                attempt.operation_id.as_str(),
                attempt.subject_actor_id.as_str(),
                attempt.authority_commitment.as_str(),
                attempt.operation_binding_commitment.as_str(),
                encode_json(attempt, "operational opening attempt")?
            ],
        )
        .map_err(|_| corrupt())?;
    Ok(())
}

fn insert_projection_binding(
    transaction: &Transaction<'_>,
    result: &OperationalExecutionWindowOpeningRecordedResult,
    binding: &OpeningProjectionBinding,
) -> Result<(), WorkflowOsError> {
    transaction
        .execute(
            "INSERT INTO operational_opening_projection_bindings
         (operation_id,receipt_id,workflow_id,run_id,projection_commitment,
          snapshot_commitment,binding_json) VALUES (?1,?2,?3,?4,?5,?6,?7)",
            params![
                result.operation_id.as_str(),
                result.receipt_id.as_str(),
                result.workflow_id.as_str(),
                result.run_id.as_str(),
                binding.event.projection_commitment().as_str(),
                binding.snapshot_commitment.as_str(),
                encode_json(binding, "operational opening projection binding")?
            ],
        )
        .map_err(|_| corrupt())?;
    Ok(())
}

fn read_existing(
    connection: &rusqlite::Connection,
    operation_id: &OperationalExecutionWindowOpeningOperationId,
) -> Result<
    Option<(
        OperationalExecutionWindowOpeningRecordedResult,
        OpeningProjectionBinding,
    )>,
    WorkflowOsError,
> {
    let result_json = connection
        .query_row(
            "SELECT record_json FROM operational_opening_operations WHERE operation_id=?1",
            params![operation_id.as_str()],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|_| corrupt())?;
    let Some(result_json) = result_json else {
        return Ok(None);
    };
    let binding_json = connection
        .query_row(
            "SELECT binding_json FROM operational_opening_projection_bindings WHERE operation_id=?1",
            params![operation_id.as_str()],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|_| corrupt())?
        .ok_or_else(corrupt)?;
    Ok(Some((
        serde_json::from_str(&result_json).map_err(|_| corrupt())?,
        serde_json::from_str(&binding_json).map_err(|_| corrupt())?,
    )))
}

fn timestamp_parts(timestamp: crate::Timestamp) -> (i64, i64) {
    let value = timestamp.as_offset_date_time();
    (value.unix_timestamp(), i64::from(value.nanosecond()))
}

fn short_hash(value: &str) -> String {
    let digest = Sha256::digest(value.as_bytes());
    digest[..12]
        .iter()
        .fold(String::with_capacity(24), |mut output, byte| {
            let _ = write!(output, "{byte:02x}");
            output
        })
}

fn corrupt() -> WorkflowOsError {
    opening_error(
        WorkflowOsErrorKind::InvalidState,
        "state_corrupt",
        "operational opening state is inconsistent",
    )
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::manual_let_else, clippy::panic)]
mod tests {
    use std::collections::BTreeMap;
    use std::fs;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
    use std::sync::{mpsc, Arc, Barrier, Condvar, Mutex};

    use rusqlite::Connection;

    use super::*;
    use crate::authorized_execution_continuity_state::internal::{
        expected_consume_directive_commitment, trusted_time_observation, window_binding_commitment,
        AuthoritativeContinuationDisposition, AuthorityUseCapability,
        AuthorizedExecutionContinuityProjectionStore, AuthorizedExecutionContinuityStore,
        ConsumeDirectiveRequest, ConsumeDirectiveResult, ContinuityCursor, ContinuityDirectiveId,
        ContinuityOperationId, ContinuityReceiptId, ContinuityTrustedTimeEpochId,
        ContinuityYieldGenerationId, TrustedTimeSourceKind,
    };
    #[cfg(feature = "trusted-host-application-spi")]
    use crate::current_authority_source::{
        registered_current_authority_test_fixture, registered_current_authority_test_source,
        RegisteredInMemoryCurrentAuthoritySource,
    };
    #[cfg(feature = "trusted-host-application-spi")]
    use crate::operational_execution_window_opening::OperationalExecutionWindowOpeningUseInput;
    use crate::operational_execution_window_opening::{
        operation_binding_commitment, trusted_host_invocation_commitment,
        OperationalExecutionWindowOpeningAuthorization,
        OperationalExecutionWindowOpeningOperationId, OperationalExecutionWindowOpeningReceiptId,
    };
    use crate::sqlite_state::dispatch_reservation_store::{
        inject_dispatch_commit_fault, InjectedDispatchCommitFault,
    };
    use crate::sqlite_state::trusted_host_explicit_local_operation::{
        run_explicit_trusted_host_local_operation, TrustedHostExplicitLocalOperationInput,
        TrustedHostExplicitLocalOperationOutcome,
    };
    use crate::sqlite_state::trusted_host_explicit_local_process_owner::{
        TrustedHostExplicitLocalProcessOwner, TrustedHostExplicitLocalProcessOwnerOutcome,
    };
    #[cfg(feature = "trusted-host-application-spi")]
    use crate::sqlite_state::trusted_host_local_application_spi::{
        prepare_trusted_host_local_application_session,
        TrustedHostLocalApplicationCancellationHandle, TrustedHostLocalApplicationOutcome,
        TrustedHostLocalApplicationPreparationInput, TrustedHostLocalApplicationPreparationPosture,
        TrustedHostLocalApplicationSession,
    };
    use crate::sqlite_state::trusted_host_local_production_caller::{
        run_trusted_host_local_production_caller, TrustedHostLocalProductionCallerInput,
    };
    use crate::sqlite_state::trusted_host_local_timer::{
        run_trusted_host_local_timer, TrustedHostLocalTimerCancellation, TrustedHostLocalTimerInput,
    };
    use crate::sqlite_state::trusted_host_operational_entry::{
        enter_trusted_host_operation, TrustedHostOperationalEntryInput,
        TrustedHostOperationalEntryLocator, TrustedHostOperationalEntryPosture,
    };
    use crate::sqlite_state::trusted_host_redispatch_loop::{
        run_bounded_trusted_host_redispatch_loop, TrustedHostRedispatchIdentityProvider,
        TrustedHostRedispatchIterationIdentity, TrustedHostRedispatchLoopInput,
        TrustedHostRedispatchStopReason,
    };
    use crate::sqlite_state::trusted_host_time_window_reinvocation::{
        reinvoke_after_time_window_wait, reinvoke_after_time_window_wait_with_test_transition_hook,
        TrustedHostTimeWindowReinvocationInput,
    };
    use crate::sqlite_state::trusted_host_time_window_scheduling::{
        run_bounded_trusted_host_repeated_scheduling, schedule_trusted_host_time_window_once,
        TrustedHostDeadlineWaitFailure, TrustedHostDeadlineWaitOutcome, TrustedHostDeadlineWaiter,
        TrustedHostRepeatedSchedulingInput, TrustedHostRepeatedSchedulingStopReason,
        TrustedHostRepeatedWakeBudget, TrustedHostScheduleOnceInput,
        TrustedHostScheduleOnceOutcome, TrustedHostScheduleWakeIdentity,
        TrustedHostScheduleWakeIdentityProvider,
    };
    use crate::sqlite_state::trusted_host_wait_handoff::observe_trusted_host_wait_with_time;
    use crate::trusted_host_supervisor::{
        inject_supervisor_persistence_fault, supervise_one_local_skill_attempt,
        InjectedSupervisorPersistenceFault, LocalSkillAttemptExecutor,
        TrustedHostAttemptExecutionContext, TrustedHostAttemptExecutionResult,
        TrustedHostAttemptExecutor, TrustedHostSupervisorAttemptCapability,
        TrustedHostSupervisorInput, TrustedHostSupervisorPersistenceInput, TrustedHostYieldRequest,
    };
    use crate::{
        ActorId, AuthorizedExecutionAttemptId, AuthorizedExecutionWindowId, EventId, EventLogStore,
        ImmutableRunBundleBinding, SchemaVersion, SkillHandler, SkillId, SkillInput, SkillOutput,
        SkillVersion, StateBackend, StepId, Timestamp, WorkflowId, WorkflowOsError,
        WorkflowRunEventKind, WorkflowRunId, WorkflowVersion,
    };
    use crate::{AuthorizedExecutionYieldReason, CorrelationId, SpecContentHash};
    #[cfg(feature = "trusted-host-application-spi")]
    use crate::{
        RedactionMetadata, RequiredContextContractBinding, RequiredContextExecutionBinding,
    };

    static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(1);

    struct Fixture {
        path: PathBuf,
        backend: SqliteStateBackend,
        workflow_id: WorkflowId,
        run_id: WorkflowRunId,
        step_id: StepId,
        bundle: ImmutableRunBundleBinding,
        cursor: ContinuityCursor,
    }

    impl Fixture {
        fn new() -> Self {
            Self::new_with_live_bundle(None)
        }

        fn new_with_live_bundle(trial: Option<LiveGrantTrial>) -> Self {
            use crate::ImmutableRunBundleStore;
            let id = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "workflow-os-operational-opening-{}-{id}.sqlite3",
                std::process::id()
            ));
            cleanup_database(&path);
            let backend = SqliteStateBackend::open(&path).expect("SQLite backend");
            let run_id = WorkflowRunId::new(format!("run-opening-{id}")).expect("run");
            let (workflow_id, step_id, bundle, workflow_version, spec_hash) =
                if let Some(trial) = trial {
                    let built = build_live_test_bundle(&run_id, trial);
                    if !matches!(trial, LiveGrantTrial::MissingBundle) {
                        backend
                            .publish_bundle_create_only(&built)
                            .expect("publish bundle");
                    }
                    let manifest = built.manifest();
                    (
                        manifest.workflow_id().clone(),
                        StepId::new("docs-check").expect("step"),
                        manifest.run_binding(),
                        manifest.workflow_version().clone(),
                        manifest.workflow_content_hash().clone(),
                    )
                } else {
                    (
                        WorkflowId::new(format!("workflow/opening-{id}")).expect("workflow"),
                        StepId::new("invoke").expect("step"),
                        serde_json::from_value(serde_json::json!({
                            "bundle_id": format!("bundle/opening-{id}"),
                            "bundle_version": "v1",
                            "root_hash": SpecContentHash::from_text("opening bundle").as_str(),
                        }))
                        .expect("bundle binding"),
                        WorkflowVersion::new("v1").expect("workflow version"),
                        SpecContentHash::from_text("opening fixture"),
                    )
                };
            let schema_version = SchemaVersion::new("workflowos.dev/v0").expect("schema");
            let timestamp = Timestamp::parse_rfc3339("2026-09-30T10:00:00Z").expect("timestamp");
            for (sequence, suffix, kind) in [
                (
                    1,
                    "created",
                    WorkflowRunEventKind::RunCreated {
                        summary: None,
                        immutable_run_bundle: Some(bundle.clone()),
                    },
                ),
                (2, "validated", WorkflowRunEventKind::RunValidated),
                (3, "started", WorkflowRunEventKind::RunStarted),
            ] {
                backend
                    .append_event(&WorkflowRunEvent {
                        sequence_number: EventSequenceNumber::new(sequence).expect("sequence"),
                        event_id: EventId::new(format!("event-opening-{id}-{suffix}"))
                            .expect("event"),
                        timestamp,
                        run_id: run_id.clone(),
                        workflow_id: workflow_id.clone(),
                        schema_version: schema_version.clone(),
                        workflow_version: workflow_version.clone(),
                        spec_content_hash: spec_hash.clone(),
                        correlation_id: None,
                        actor: Some(ActorId::new("system/opening-test").expect("actor")),
                        idempotency_key: None,
                        kind,
                    })
                    .expect("append fixture event");
            }
            Self {
                path,
                backend,
                workflow_id,
                run_id,
                step_id,
                bundle,
                cursor: ContinuityCursor {
                    sequence_number: EventSequenceNumber::new(3).expect("sequence"),
                    event_id: EventId::new(format!("event-opening-{id}-started")).expect("event"),
                },
            }
        }

        #[cfg(feature = "trusted-host-application-spi")]
        fn from_required_context_binding(binding: &RequiredContextExecutionBinding) -> Self {
            let id = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "workflow-os-operational-opening-{}-{id}.sqlite3",
                std::process::id()
            ));
            cleanup_database(&path);
            let backend = SqliteStateBackend::open(&path).expect("SQLite backend");
            let workflow_id = binding.workflow_id().clone();
            let run_id = binding.run_id().clone();
            let step_id = binding.step_id().clone();
            let bundle = binding.immutable_run_bundle().clone();
            let schema_version = SchemaVersion::new("workflowos.dev/v0").expect("schema");
            let workflow_version = WorkflowVersion::new("v1").expect("workflow version");
            let spec_hash = SpecContentHash::from_text("opening fixture");
            let timestamp = Timestamp::parse_rfc3339("2026-07-26T10:00:00Z").expect("timestamp");
            for (sequence, suffix, kind) in [
                (
                    1,
                    "created",
                    WorkflowRunEventKind::RunCreated {
                        summary: None,
                        immutable_run_bundle: Some(bundle.clone()),
                    },
                ),
                (2, "validated", WorkflowRunEventKind::RunValidated),
                (3, "started", WorkflowRunEventKind::RunStarted),
            ] {
                backend
                    .append_event(&WorkflowRunEvent {
                        sequence_number: EventSequenceNumber::new(sequence).expect("sequence"),
                        event_id: EventId::new(format!("event-opening-{id}-{suffix}"))
                            .expect("event"),
                        timestamp,
                        run_id: run_id.clone(),
                        workflow_id: workflow_id.clone(),
                        schema_version: schema_version.clone(),
                        workflow_version: workflow_version.clone(),
                        spec_content_hash: spec_hash.clone(),
                        correlation_id: None,
                        actor: Some(ActorId::new("system/opening-test").expect("actor")),
                        idempotency_key: None,
                        kind,
                    })
                    .expect("append fixture event");
            }
            Self {
                path,
                backend,
                workflow_id,
                run_id,
                step_id,
                bundle,
                cursor: ContinuityCursor {
                    sequence_number: EventSequenceNumber::new(3).expect("sequence"),
                    event_id: EventId::new(format!("event-opening-{id}-started")).expect("event"),
                },
            }
        }

        fn authorization(&self) -> OperationalExecutionWindowOpeningAuthorization {
            let invocation = trusted_host_invocation_commitment(
                &skill_input(self),
                &supervisor_executor_binding(),
            );
            OperationalExecutionWindowOpeningAuthorization::new(
                SpecContentHash::from_text("current authority"),
                SpecContentHash::from_text("core governance"),
                operation_binding_commitment(
                    &self.workflow_id,
                    &self.run_id,
                    &self.step_id,
                    &invocation,
                ),
            )
        }

        fn request<'a>(
            &'a self,
            authorization: &'a OperationalExecutionWindowOpeningAuthorization,
            operation_id: &str,
        ) -> OperationalExecutionWindowOpeningRequest<'a> {
            let operation_id = OperationalExecutionWindowOpeningOperationId::new(operation_id)
                .expect("operation id");
            let window_id =
                AuthorizedExecutionWindowId::new(format!("window/{}", operation_id.as_str()))
                    .expect("window id");
            let attempt_id =
                AuthorizedExecutionAttemptId::new(format!("attempt/{}", operation_id.as_str()))
                    .expect("attempt id");
            let snapshot = self
                .backend
                .rehydrate_run(&self.run_id)
                .expect("run")
                .snapshot;
            let mut request = OperationalExecutionWindowOpeningRequest {
                receipt_id: OperationalExecutionWindowOpeningReceiptId::new(format!(
                    "receipt/{}",
                    operation_id.as_str()
                ))
                .expect("receipt id"),
                operation_id,
                request_commitment: SpecContentHash::from_text("pending opening request"),
                workflow_id: self.workflow_id.clone(),
                run_id: self.run_id.clone(),
                step_id: self.step_id.clone(),
                invocation_binding_commitment: trusted_host_invocation_commitment(
                    &skill_input(self),
                    &supervisor_executor_binding(),
                ),
                workflow_lifecycle_commitment: None,
                window_id,
                attempt_id,
                subject_actor_id: ActorId::new("agent/opening-test").expect("actor"),
                immutable_run_bundle: self.bundle.clone(),
                expected_cursor: self.cursor.clone(),
                expected_snapshot_commitment: run_snapshot_commitment(&snapshot)
                    .expect("snapshot commitment"),
                expires_at: Timestamp::parse_rfc3339("2099-09-30T10:10:00Z").expect("expiry"),
                maximum_attempts: 1,
                trusted_time: trusted_time_observation(
                    Timestamp::parse_rfc3339("2026-09-30T10:01:00Z").expect("observed"),
                    TrustedTimeSourceKind::CoreInjectedClockV1,
                    SpecContentHash::new(super::super::CONTINUITY_CLOCK_PROVENANCE)
                        .expect("provenance"),
                    ContinuityTrustedTimeEpochId::new(super::super::CONTINUITY_CLOCK_EPOCH)
                        .expect("epoch"),
                ),
                authorization,
            };
            request.request_commitment = opening_request_commitment(&request);
            request
        }

        fn counts(&self) -> (i64, i64, i64, i64) {
            let connection = Connection::open(&self.path).expect("inspect database");
            let count = |table: &str| {
                connection
                    .query_row(&format!("SELECT COUNT(*) FROM {table}"), [], |row| {
                        row.get(0)
                    })
                    .expect("count")
            };
            (
                count("operational_opening_operations"),
                count("operational_opening_attempts"),
                count("operational_opening_projection_bindings"),
                count("continuity_windows"),
            )
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            cleanup_database(&self.path);
        }
    }

    #[cfg(feature = "trusted-host-application-spi")]
    struct FreshApplicationFixture {
        contract: RequiredContextContractBinding,
        binding: RequiredContextExecutionBinding,
        source: RegisteredInMemoryCurrentAuthoritySource,
        runtime: Fixture,
        redaction: RedactionMetadata,
    }

    #[cfg(feature = "trusted-host-application-spi")]
    impl FreshApplicationFixture {
        fn new() -> Self {
            let (contract, binding) = registered_current_authority_test_fixture();
            let source = registered_current_authority_test_source(&contract);
            let runtime = Fixture::from_required_context_binding(&binding);
            Self {
                contract,
                binding,
                source,
                runtime,
                redaction: RedactionMetadata::empty(),
            }
        }

        fn locator(&self) -> TrustedHostOperationalEntryLocator {
            TrustedHostOperationalEntryLocator {
                workflow_id: self.runtime.workflow_id.clone(),
                run_id: self.runtime.run_id.clone(),
                step_id: self.runtime.step_id.clone(),
                window_id: AuthorizedExecutionWindowId::new("window/application-prepare-fresh")
                    .expect("window"),
                subject_actor_id: self.binding.actor().clone(),
                immutable_run_bundle: self.runtime.bundle.clone(),
            }
        }

        fn input<'a>(
            &'a self,
            executor: &'a dyn TrustedHostAttemptExecutor,
            invocation_commitment: &'a SpecContentHash,
        ) -> TrustedHostLocalApplicationPreparationInput<'a> {
            let locator = self.locator();
            TrustedHostLocalApplicationPreparationInput {
                backend: &self.runtime.backend,
                posture: TrustedHostLocalApplicationPreparationPosture::Fresh {
                    opening: Box::new(OperationalExecutionWindowOpeningUseInput {
                        source: &self.source,
                        backend: &self.runtime.backend,
                        execution_binding: &self.binding,
                        contract: &self.contract,
                        invocation_binding_commitment: invocation_commitment,
                        operation_id: OperationalExecutionWindowOpeningOperationId::new(
                            "opening/application-prepare-fresh",
                        )
                        .expect("operation"),
                        receipt_id: OperationalExecutionWindowOpeningReceiptId::new(
                            "receipt/application-prepare-fresh",
                        )
                        .expect("receipt"),
                        window_id: locator.window_id.clone(),
                        attempt_id: AuthorizedExecutionAttemptId::new(
                            "attempt/application-prepare-fresh",
                        )
                        .expect("attempt"),
                        expires_at: Timestamp::parse_rfc3339("2099-07-26T11:00:00Z")
                            .expect("expiry"),
                        maximum_attempts: 1,
                        trusted_time: trusted_time_observation(
                            Timestamp::parse_rfc3339("2026-07-26T10:25:00Z").expect("observed"),
                            TrustedTimeSourceKind::CoreInjectedClockV1,
                            SpecContentHash::new(super::super::CONTINUITY_CLOCK_PROVENANCE)
                                .expect("provenance"),
                            ContinuityTrustedTimeEpochId::new(super::super::CONTINUITY_CLOCK_EPOCH)
                                .expect("epoch"),
                        ),
                        evaluated_at: Timestamp::parse_rfc3339("2026-07-26T10:25:00Z")
                            .expect("evaluated"),
                        redaction: &self.redaction,
                        workflow_lifecycle: None,
                    }),
                    persistence: TrustedHostSupervisorPersistenceInput {
                        operation: ContinuityOperationId::new(
                            "operation/application-prepare-fresh-supervisor",
                        )
                        .expect("operation"),
                        receipt: ContinuityReceiptId::new(
                            "receipt/application-prepare-fresh-supervisor",
                        )
                        .expect("receipt"),
                        yield_generation: None,
                    },
                },
                executor,
                skill_input: skill_input(&self.runtime),
                locator,
            }
        }

        fn invocation_commitment(
            &self,
            executor: &dyn TrustedHostAttemptExecutor,
        ) -> SpecContentHash {
            trusted_host_invocation_commitment(
                &skill_input(&self.runtime),
                &executor.binding_commitment(),
            )
        }
    }

    fn cleanup_database(path: &std::path::Path) {
        for suffix in ["", "-shm", "-wal"] {
            let _ = fs::remove_file(format!("{}{suffix}", path.display()));
        }
    }

    fn inject_commit_fault(fault: InjectedOpeningCommitFault) {
        INJECTED_COMMIT_FAULT.with(|slot| slot.set(Some(fault)));
    }

    fn opening_failure(
        result: Result<ProjectedOperationalExecutionWindowOpeningResult, WorkflowOsError>,
    ) -> WorkflowOsError {
        match result {
            Ok(_) => panic!("operational opening must fail"),
            Err(error) => error,
        }
    }

    fn assert_idempotency_conflict(
        fixture: &Fixture,
        mut request: OperationalExecutionWindowOpeningRequest<'_>,
    ) {
        request.request_commitment = opening_request_commitment(&request);
        let error = opening_failure(
            fixture
                .backend
                .open_window_and_start_attempt_projected(request),
        );
        assert_eq!(
            error.code(),
            "operational_execution_window_opening.idempotency_conflict"
        );
        assert_eq!(fixture.counts(), (1, 1, 1, 1));
    }

    #[test]
    fn opening_atomically_creates_event_window_and_started_attempt() {
        let fixture = Fixture::new();
        let authorization = fixture.authorization();
        let result = fixture
            .backend
            .open_window_and_start_attempt_projected(
                fixture.request(&authorization, "opening/atomic"),
            )
            .expect("opening succeeds");

        assert!(matches!(
            result.result,
            OperationalExecutionWindowOpeningResult::Opened { .. }
        ));
        assert_eq!(result.event.attempt_number(), 1);
        assert_eq!(result.event.window_revision(), 1);
        assert_eq!(fixture.counts(), (1, 1, 1, 1));
        let run = fixture.backend.rehydrate_run(&fixture.run_id).expect("run");
        assert_eq!(run.snapshot.status, WorkflowRunStatus::Running);
        assert_eq!(run.events.len(), 4);
        assert!(run.snapshot.last_operational_opening_projection.is_some());
    }

    #[test]
    fn exact_replay_is_stable_and_does_not_append_an_event() {
        let fixture = Fixture::new();
        let authorization = fixture.authorization();
        let first = fixture.request(&authorization, "opening/replay");
        let replay_request = fixture.request(&authorization, "opening/replay");
        fixture
            .backend
            .open_window_and_start_attempt_projected(first)
            .expect("first opening");
        let replay = fixture
            .backend
            .open_window_and_start_attempt_projected(replay_request)
            .expect("exact replay");

        assert!(matches!(
            replay.result,
            OperationalExecutionWindowOpeningResult::ExactReplay(_)
        ));
        assert_eq!(fixture.counts(), (1, 1, 1, 1));
        assert_eq!(
            fixture
                .backend
                .read_events(&fixture.run_id)
                .expect("events")
                .len(),
            4
        );
    }

    #[test]
    fn fixture_predispatch_inspection_accepts_exact_active_attempt_without_mutation() {
        let fixture = Fixture::new();
        let authorization = fixture.authorization();
        let request = fixture.request(&authorization, "opening/fixture-inspection");
        let actor = request.subject_actor_id.clone();
        fixture
            .backend
            .open_window_and_start_attempt_projected(request)
            .expect("opening succeeds");
        let reader = SqliteStateBackend::open_read_only(&fixture.path).expect("read-only backend");
        let run = reader.rehydrate_run(&fixture.run_id).expect("run");
        let opening = run
            .snapshot
            .last_operational_opening_projection
            .as_ref()
            .expect("opening");
        let counts = fixture.counts();
        assert!(reader
            .fixture_predispatch_attempt_is_current(
                &run,
                Some(&actor),
                Some(&fixture.step_id),
                opening,
            )
            .expect("inspection"));
        assert!(reader
            .connection()
            .expect("connection")
            .execute_batch("CREATE TABLE unexpected_write (value INTEGER)")
            .is_err());
        assert_eq!(counts, fixture.counts());
        assert_eq!(
            run,
            fixture
                .backend
                .rehydrate_run(&fixture.run_id)
                .expect("unchanged run")
        );
    }

    #[test]
    fn read_only_open_rejects_absent_database_without_creating_it() {
        let fixture = Fixture::new();
        let absent = fixture.path.with_extension("absent.sqlite3");
        assert!(!absent.exists());
        assert!(SqliteStateBackend::open_read_only(&absent).is_err());
        assert!(!absent.exists());
    }

    #[test]
    fn fixture_predispatch_inspection_rejects_actor_step_and_run_drift() {
        let fixture = Fixture::new();
        let authorization = fixture.authorization();
        let request = fixture.request(&authorization, "opening/fixture-drift");
        let actor = request.subject_actor_id.clone();
        fixture
            .backend
            .open_window_and_start_attempt_projected(request)
            .expect("opening succeeds");
        let run = fixture.backend.rehydrate_run(&fixture.run_id).expect("run");
        let opening = run
            .snapshot
            .last_operational_opening_projection
            .as_ref()
            .expect("opening");
        for (actor, step) in [
            (None, Some(fixture.step_id.clone())),
            (
                Some(ActorId::new("agent/other").expect("actor")),
                Some(fixture.step_id.clone()),
            ),
            (Some(actor.clone()), None),
            (
                Some(actor.clone()),
                Some(StepId::new("other").expect("step")),
            ),
        ] {
            assert!(!fixture
                .backend
                .fixture_predispatch_attempt_is_current(
                    &run,
                    actor.as_ref(),
                    step.as_ref(),
                    opening,
                )
                .expect("drift inspection"));
        }
        let mut changed = run.clone();
        changed.snapshot.identity.run_id = WorkflowRunId::new("other-run").expect("run id");
        assert!(!fixture
            .backend
            .fixture_predispatch_attempt_is_current(
                &changed,
                Some(&actor),
                Some(&fixture.step_id),
                opening,
            )
            .expect("run drift inspection"));
        let mut changed = run.clone();
        changed.snapshot.identity.workflow_id =
            WorkflowId::new("workflow/other").expect("workflow");
        assert!(!fixture
            .backend
            .fixture_predispatch_attempt_is_current(
                &changed,
                Some(&actor),
                Some(&fixture.step_id),
                opening,
            )
            .expect("workflow drift inspection"));
        let mut changed = run.clone();
        changed.snapshot.identity.immutable_run_bundle = None;
        assert!(!fixture
            .backend
            .fixture_predispatch_attempt_is_current(
                &changed,
                Some(&actor),
                Some(&fixture.step_id),
                opening,
            )
            .expect("bundle drift inspection"));
        assert_eq!(
            run,
            fixture
                .backend
                .rehydrate_run(&fixture.run_id)
                .expect("unchanged run")
        );
    }

    #[test]
    fn fixture_predispatch_inspection_rejects_expired_attempt_without_mutation() {
        let fixture = Fixture::new();
        let authorization = fixture.authorization();
        let mut request = fixture.request(&authorization, "opening/fixture-expired");
        request.expires_at = Timestamp::parse_rfc3339("2026-09-30T10:10:00Z").expect("expiry");
        request.request_commitment = opening_request_commitment(&request);
        let actor = request.subject_actor_id.clone();
        fixture
            .backend
            .open_window_and_start_attempt_projected(request)
            .expect("opening at historical trusted time");
        let run = fixture.backend.rehydrate_run(&fixture.run_id).expect("run");
        let opening = run
            .snapshot
            .last_operational_opening_projection
            .as_ref()
            .expect("opening");
        assert!(!fixture
            .backend
            .fixture_predispatch_attempt_is_current(
                &run,
                Some(&actor),
                Some(&fixture.step_id),
                opening,
            )
            .expect("expiry inspection"));
        assert_eq!(
            run,
            fixture
                .backend
                .rehydrate_run(&fixture.run_id)
                .expect("unchanged run")
        );
    }

    #[test]
    fn fixture_predispatch_inspection_rejects_revoked_window_without_mutation() {
        let fixture = Fixture::new();
        let authorization = fixture.authorization();
        let request = fixture.request(&authorization, "opening/fixture-revoked");
        let actor = request.subject_actor_id.clone();
        let window_id = request.window_id.clone();
        fixture
            .backend
            .open_window_and_start_attempt_projected(request)
            .expect("opening");
        let mut connection = fixture.backend.connection().expect("fixture connection");
        let transaction = connection
            .transaction()
            .expect("fixture revocation transaction");
        let mut state = super::super::continuity_codec::load_snapshot(&transaction)
            .expect("fixture continuity state");
        let window = state.windows.get_mut(&window_id).expect("fixture window");
        window.state = crate::authorized_execution_continuity_state::internal::AuthoritativeWindowState::Revoked;
        super::super::continuity_store::persist_window(&transaction, window)
            .expect("persist fixture revocation");
        transaction.commit().expect("commit fixture revocation");
        let run = fixture.backend.rehydrate_run(&fixture.run_id).expect("run");
        let opening = run
            .snapshot
            .last_operational_opening_projection
            .as_ref()
            .expect("opening");
        assert!(!fixture
            .backend
            .fixture_predispatch_attempt_is_current(
                &run,
                Some(&actor),
                Some(&fixture.step_id),
                opening,
            )
            .expect("revoked window inspection"));
        assert_eq!(
            fixture
                .backend
                .rehydrate_run(&fixture.run_id)
                .expect("unchanged run"),
            run
        );
    }

    #[test]
    fn same_operation_id_with_different_receipt_fails_closed() {
        let fixture = Fixture::new();
        let authorization = fixture.authorization();
        fixture
            .backend
            .open_window_and_start_attempt_projected(
                fixture.request(&authorization, "opening/conflict"),
            )
            .expect("first opening");
        let mut conflicting = fixture.request(&authorization, "opening/conflict");
        conflicting.receipt_id =
            OperationalExecutionWindowOpeningReceiptId::new("receipt/conflicting")
                .expect("receipt");
        conflicting.request_commitment = opening_request_commitment(&conflicting);
        let error = match fixture
            .backend
            .open_window_and_start_attempt_projected(conflicting)
        {
            Ok(_) => panic!("conflicting replay must fail"),
            Err(error) => error,
        };

        assert_eq!(
            error.code(),
            "operational_execution_window_opening.idempotency_conflict"
        );
        assert_eq!(fixture.counts(), (1, 1, 1, 1));
        assert_eq!(
            fixture
                .backend
                .read_events(&fixture.run_id)
                .expect("events")
                .len(),
            4
        );
    }

    #[test]
    fn same_operation_and_receipt_with_substituted_authority_context_fails_closed() {
        let fixture = Fixture::new();
        let authorization = fixture.authorization();
        fixture
            .backend
            .open_window_and_start_attempt_projected(
                fixture.request(&authorization, "opening/authority-substitution"),
            )
            .expect("first opening");
        let substituted_authority = OperationalExecutionWindowOpeningAuthorization::new(
            SpecContentHash::from_text("different current authority"),
            SpecContentHash::from_text("different core governance"),
            operation_binding_commitment(
                &fixture.workflow_id,
                &fixture.run_id,
                &fixture.step_id,
                &trusted_host_invocation_commitment(
                    &skill_input(&fixture),
                    &supervisor_executor_binding(),
                ),
            ),
        );
        let mut substituted =
            fixture.request(&substituted_authority, "opening/authority-substitution");
        substituted.request_commitment = opening_request_commitment(&substituted);
        let error = opening_failure(
            fixture
                .backend
                .open_window_and_start_attempt_projected(substituted),
        );

        assert_eq!(
            error.code(),
            "operational_execution_window_opening.idempotency_conflict"
        );
        assert_eq!(fixture.counts(), (1, 1, 1, 1));
    }

    #[test]
    fn changed_bound_input_without_matching_commitment_is_rejected_before_replay() {
        let fixture = Fixture::new();
        let authorization = fixture.authorization();
        fixture
            .backend
            .open_window_and_start_attempt_projected(
                fixture.request(&authorization, "opening/input-substitution"),
            )
            .expect("first opening");
        let mut substituted = fixture.request(&authorization, "opening/input-substitution");
        substituted.subject_actor_id = ActorId::new("agent/substituted").expect("actor");
        let error = opening_failure(
            fixture
                .backend
                .open_window_and_start_attempt_projected(substituted),
        );

        assert_eq!(
            error.code(),
            "operational_execution_window_opening.request_commitment_mismatch"
        );
        assert_eq!(fixture.counts(), (1, 1, 1, 1));
    }

    #[test]
    fn complete_request_commitment_rejects_every_substituted_input_family() {
        let fixture = Fixture::new();
        let authorization = fixture.authorization();
        fixture
            .backend
            .open_window_and_start_attempt_projected(
                fixture.request(&authorization, "opening/complete-binding"),
            )
            .expect("first opening");

        let mut actor = fixture.request(&authorization, "opening/complete-binding");
        actor.subject_actor_id = ActorId::new("agent/different").expect("actor");
        assert_idempotency_conflict(&fixture, actor);

        let mut bundle = fixture.request(&authorization, "opening/complete-binding");
        bundle.immutable_run_bundle = bundle
            .immutable_run_bundle
            .with_test_root_hash(SpecContentHash::from_text("different bundle"));
        assert_idempotency_conflict(&fixture, bundle);

        let mut cursor = fixture.request(&authorization, "opening/complete-binding");
        cursor.expected_cursor.event_id = EventId::new("event/different").expect("event");
        assert_idempotency_conflict(&fixture, cursor);

        let mut snapshot = fixture.request(&authorization, "opening/complete-binding");
        snapshot.expected_snapshot_commitment = SpecContentHash::from_text("different snapshot");
        assert_idempotency_conflict(&fixture, snapshot);

        let mut expiry = fixture.request(&authorization, "opening/complete-binding");
        expiry.expires_at = Timestamp::parse_rfc3339("2026-09-30T10:11:00Z").expect("expiry");
        assert_idempotency_conflict(&fixture, expiry);

        let mut budget = fixture.request(&authorization, "opening/complete-binding");
        budget.maximum_attempts = 2;
        assert_idempotency_conflict(&fixture, budget);

        let mut trusted = fixture.request(&authorization, "opening/complete-binding");
        trusted.trusted_time = trusted_time_observation(
            Timestamp::parse_rfc3339("2026-09-30T10:02:00Z").expect("observed"),
            TrustedTimeSourceKind::CoreInjectedClockV1,
            SpecContentHash::new(super::super::CONTINUITY_CLOCK_PROVENANCE).expect("provenance"),
            ContinuityTrustedTimeEpochId::new(super::super::CONTINUITY_CLOCK_EPOCH).expect("epoch"),
        );
        assert_idempotency_conflict(&fixture, trusted);
    }

    #[test]
    fn expected_snapshot_substitution_is_rejected_before_mutation() {
        let fixture = Fixture::new();
        let authorization = fixture.authorization();
        let mut request = fixture.request(&authorization, "opening/snapshot-substitution");
        request.expected_snapshot_commitment = SpecContentHash::from_text("different snapshot");
        request.request_commitment = opening_request_commitment(&request);
        let error = opening_failure(
            fixture
                .backend
                .open_window_and_start_attempt_projected(request),
        );

        assert_eq!(
            error.code(),
            "operational_execution_window_opening.runtime_binding_stale"
        );
        assert_eq!(fixture.counts(), (0, 0, 0, 0));
        assert_eq!(
            fixture
                .backend
                .read_events(&fixture.run_id)
                .expect("events")
                .len(),
            3
        );
    }

    #[test]
    fn stale_runtime_cursor_is_rejected_without_writes() {
        let fixture = Fixture::new();
        let authorization = fixture.authorization();
        let mut request = fixture.request(&authorization, "opening/stale");
        request.expected_cursor = ContinuityCursor {
            sequence_number: EventSequenceNumber::new(2).expect("sequence"),
            event_id: EventId::new("event/stale").expect("event"),
        };
        request.request_commitment = opening_request_commitment(&request);
        let error = match fixture
            .backend
            .open_window_and_start_attempt_projected(request)
        {
            Ok(_) => panic!("stale cursor must fail"),
            Err(error) => error,
        };

        assert_eq!(
            error.code(),
            "operational_execution_window_opening.runtime_binding_stale"
        );
        assert_eq!(fixture.counts(), (0, 0, 0, 0));
    }

    #[test]
    fn generic_runtime_event_wins_cursor_contention_without_partial_opening() {
        let fixture = Fixture::new();
        let authorization = fixture.authorization();
        let request = fixture.request(&authorization, "opening/cursor-contention");
        fixture
            .backend
            .append_event(&WorkflowRunEvent {
                sequence_number: EventSequenceNumber::new(4).expect("sequence"),
                event_id: EventId::new("event/generic-winner").expect("event"),
                timestamp: Timestamp::parse_rfc3339("2026-09-30T10:00:30Z").expect("timestamp"),
                run_id: fixture.run_id.clone(),
                workflow_id: fixture.workflow_id.clone(),
                schema_version: SchemaVersion::new("workflowos.dev/v0").expect("schema"),
                workflow_version: WorkflowVersion::new("v1").expect("version"),
                spec_content_hash: SpecContentHash::from_text("opening fixture"),
                correlation_id: None,
                actor: Some(ActorId::new("system/contention").expect("actor")),
                idempotency_key: None,
                kind: WorkflowRunEventKind::RunCompleted,
            })
            .expect("generic event wins");
        let error = opening_failure(
            fixture
                .backend
                .open_window_and_start_attempt_projected(request),
        );

        assert_eq!(
            error.code(),
            "operational_execution_window_opening.runtime_binding_stale"
        );
        assert_eq!(fixture.counts(), (0, 0, 0, 0));
        assert_eq!(
            fixture
                .backend
                .read_events(&fixture.run_id)
                .expect("events")
                .len(),
            4
        );
    }

    #[test]
    fn concurrent_openers_have_exactly_one_winner() {
        let fixture = Fixture::new();
        let first_authorization = fixture.authorization();
        let second_authorization = fixture.authorization();
        let first = fixture.request(&first_authorization, "opening/concurrent-a");
        let second = fixture.request(&second_authorization, "opening/concurrent-b");
        let barrier = Arc::new(Barrier::new(3));
        let first_backend = fixture.backend.clone();
        let second_backend = fixture.backend.clone();

        let (first_result, second_result) = std::thread::scope(|scope| {
            let first_barrier = Arc::clone(&barrier);
            let first_handle = scope.spawn(move || {
                first_barrier.wait();
                first_backend.open_window_and_start_attempt_projected(first)
            });
            let second_barrier = Arc::clone(&barrier);
            let second_handle = scope.spawn(move || {
                second_barrier.wait();
                second_backend.open_window_and_start_attempt_projected(second)
            });
            barrier.wait();
            (
                first_handle.join().expect("first opener"),
                second_handle.join().expect("second opener"),
            )
        });

        let results = [first_result, second_result];
        assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
        let loser = results
            .iter()
            .find_map(|result| result.as_ref().err())
            .expect("one loser");
        assert_eq!(
            loser.code(),
            "operational_execution_window_opening.runtime_binding_stale"
        );
        assert_eq!(fixture.counts(), (1, 1, 1, 1));
        assert_eq!(
            fixture
                .backend
                .read_events(&fixture.run_id)
                .expect("events")
                .len(),
            4
        );
    }

    #[test]
    fn substituted_operation_binding_is_rejected_without_writes() {
        let fixture = Fixture::new();
        let authorization = OperationalExecutionWindowOpeningAuthorization::new(
            SpecContentHash::from_text("current authority"),
            SpecContentHash::from_text("core governance"),
            operation_binding_commitment(
                &fixture.workflow_id,
                &fixture.run_id,
                &StepId::new("different-step").expect("step"),
                &trusted_host_invocation_commitment(
                    &skill_input(&fixture),
                    &supervisor_executor_binding(),
                ),
            ),
        );
        let result = fixture.backend.open_window_and_start_attempt_projected(
            fixture.request(&authorization, "opening/substitution"),
        );
        let error = match result {
            Ok(_) => panic!("binding substitution must be rejected"),
            Err(error) => error,
        };

        assert_eq!(
            error.code(),
            "operational_execution_window_opening.operation_binding_mismatch"
        );
        assert_eq!(fixture.counts(), (0, 0, 0, 0));
        assert_eq!(
            fixture
                .backend
                .read_events(&fixture.run_id)
                .expect("events")
                .len(),
            3
        );
    }

    #[test]
    fn reconciliation_recovers_committed_result_from_a_fresh_connection() {
        let fixture = Fixture::new();
        let authorization = fixture.authorization();
        let request = fixture.request(&authorization, "opening/reconcile");
        let operation_id = request.operation_id.clone();
        let request_commitment = request.request_commitment.clone();
        let receipt_id = request.receipt_id.clone();
        fixture
            .backend
            .open_window_and_start_attempt_projected(request)
            .expect("opening");
        let reopened = SqliteStateBackend::open(&fixture.path).expect("reopen");
        let reconciled = reopened
            .reconcile_operational_opening(&operation_id, &request_commitment, &receipt_id)
            .expect("reconcile");

        assert!(matches!(
            reconciled,
            OperationalExecutionWindowOpeningReconciliationResult::DurablyCommitted { .. }
        ));
    }

    #[test]
    fn commit_faults_never_return_capability_and_reconcile_durability() {
        for fault in [
            InjectedOpeningCommitFault::Before,
            InjectedOpeningCommitFault::During,
            InjectedOpeningCommitFault::After,
        ] {
            let fixture = Fixture::new();
            let authorization = fixture.authorization();
            let request = fixture.request(&authorization, "opening/fault");
            let operation_id = request.operation_id.clone();
            let receipt_id = request.receipt_id.clone();
            let request_commitment = request.request_commitment.clone();
            inject_commit_fault(fault);
            let error = opening_failure(
                fixture
                    .backend
                    .open_window_and_start_attempt_projected(request),
            );
            let reconciliation = fixture
                .backend
                .reconcile_operational_opening(&operation_id, &request_commitment, &receipt_id)
                .expect("reconciliation");

            match fault {
                InjectedOpeningCommitFault::Before => {
                    assert_eq!(
                        error.code(),
                        "operational_execution_window_opening.write_failed"
                    );
                    assert!(matches!(
                        reconciliation,
                        OperationalExecutionWindowOpeningReconciliationResult::ConfirmedAbsent
                    ));
                    assert_eq!(fixture.counts(), (0, 0, 0, 0));
                    assert_eq!(
                        fixture
                            .backend
                            .read_events(&fixture.run_id)
                            .expect("events")
                            .len(),
                        3
                    );
                }
                InjectedOpeningCommitFault::During | InjectedOpeningCommitFault::After => {
                    assert_eq!(
                        error.code(),
                        "operational_execution_window_opening.commit_ambiguous"
                    );
                    assert!(matches!(
                        reconciliation,
                        OperationalExecutionWindowOpeningReconciliationResult::DurablyCommitted { .. }
                    ));
                    assert_eq!(fixture.counts(), (1, 1, 1, 1));
                    assert_eq!(
                        fixture
                            .backend
                            .read_events(&fixture.run_id)
                            .expect("events")
                            .len(),
                        4
                    );
                }
            }
        }
    }

    struct YieldExecutor;

    impl TrustedHostAttemptExecutor for YieldExecutor {
        fn binding_commitment(&self) -> SpecContentHash {
            supervisor_executor_binding()
        }

        fn execute(
            &self,
            _context: &TrustedHostAttemptExecutionContext<'_>,
        ) -> TrustedHostAttemptExecutionResult {
            TrustedHostAttemptExecutionResult::Yielded(TrustedHostYieldRequest::without_wait(
                AuthorizedExecutionYieldReason::TurnBoundary,
            ))
        }
    }

    struct TimeWindowYieldExecutor {
        condition_version: u32,
        deadline: Timestamp,
    }

    impl TrustedHostAttemptExecutor for TimeWindowYieldExecutor {
        fn binding_commitment(&self) -> SpecContentHash {
            supervisor_executor_binding()
        }

        fn execute(
            &self,
            _context: &TrustedHostAttemptExecutionContext<'_>,
        ) -> TrustedHostAttemptExecutionResult {
            TrustedHostAttemptExecutionResult::Yielded(TrustedHostYieldRequest::with_time_window(
                AuthorizedExecutionYieldReason::ContextBudget,
                crate::AuthorizedExecutionWaitConditionId::new("wait/supervisor-time-window")
                    .expect("condition"),
                self.condition_version,
                self.deadline,
            ))
        }
    }

    struct YieldThenSucceedExecutor<'a> {
        calls: &'a AtomicUsize,
        yields_before_success: usize,
    }

    impl TrustedHostAttemptExecutor for YieldThenSucceedExecutor<'_> {
        fn binding_commitment(&self) -> SpecContentHash {
            supervisor_executor_binding()
        }

        fn execute(
            &self,
            _context: &TrustedHostAttemptExecutionContext<'_>,
        ) -> TrustedHostAttemptExecutionResult {
            if self.calls.fetch_add(1, Ordering::Relaxed) < self.yields_before_success {
                TrustedHostAttemptExecutionResult::Yielded(TrustedHostYieldRequest::without_wait(
                    AuthorizedExecutionYieldReason::TurnBoundary,
                ))
            } else {
                TrustedHostAttemptExecutionResult::Succeeded(SkillOutput::new(
                    BTreeMap::from([("status".to_owned(), "ok".to_owned())]),
                    Some("bounded-redispatch-output".to_owned()),
                ))
            }
        }
    }

    struct DeterministicRedispatchIdentityProvider {
        calls: usize,
    }

    impl TrustedHostRedispatchIdentityProvider for DeterministicRedispatchIdentityProvider {
        fn next_identity(
            &mut self,
            _iteration: u32,
        ) -> Result<TrustedHostRedispatchIterationIdentity, WorkflowOsError> {
            self.calls += 1;
            let identity_sequence = self.calls;
            Ok(TrustedHostRedispatchIterationIdentity {
                consume_operation: ContinuityOperationId::new(format!(
                    "operation/redispatch-consume-{identity_sequence}"
                ))?,
                consume_receipt: ContinuityReceiptId::new(format!(
                    "receipt/redispatch-consume-{identity_sequence}"
                ))?,
                generated_attempt: AuthorizedExecutionAttemptId::new(format!(
                    "attempt/redispatch-{identity_sequence}"
                ))?,
                supervisor_operation: ContinuityOperationId::new(format!(
                    "operation/redispatch-supervisor-{identity_sequence}"
                ))?,
                supervisor_receipt: ContinuityReceiptId::new(format!(
                    "receipt/redispatch-supervisor-{identity_sequence}"
                ))?,
                yield_generation: ContinuityYieldGenerationId::new(format!(
                    "yield/redispatch-{identity_sequence}"
                ))?,
            })
        }
    }

    struct CountingExecutor<'a> {
        calls: &'a AtomicUsize,
    }

    struct DifferentBindingExecutor<'a> {
        calls: &'a AtomicUsize,
    }

    struct BlockingTimeWindowExecutor {
        calls: Arc<AtomicUsize>,
        entered: mpsc::Sender<()>,
        release: Arc<(Mutex<bool>, Condvar)>,
    }

    struct CountingBindingExecutor<'a> {
        calls: &'a AtomicUsize,
        delegate: &'a dyn TrustedHostAttemptExecutor,
    }

    impl TrustedHostAttemptExecutor for CountingBindingExecutor<'_> {
        fn binding_commitment(&self) -> SpecContentHash {
            self.delegate.binding_commitment()
        }

        fn execute(
            &self,
            context: &TrustedHostAttemptExecutionContext<'_>,
        ) -> TrustedHostAttemptExecutionResult {
            self.calls.fetch_add(1, Ordering::Relaxed);
            self.delegate.execute(context)
        }
    }

    struct YieldTimeWindowThenFailExecutor<'a> {
        calls: &'a AtomicUsize,
    }

    struct YieldTwoTimeWindowsThenFailExecutor<'a> {
        calls: &'a AtomicUsize,
    }

    impl TrustedHostAttemptExecutor for YieldTimeWindowThenFailExecutor<'_> {
        fn binding_commitment(&self) -> SpecContentHash {
            supervisor_executor_binding()
        }

        fn execute(
            &self,
            _context: &TrustedHostAttemptExecutionContext<'_>,
        ) -> TrustedHostAttemptExecutionResult {
            if self.calls.fetch_add(1, Ordering::Relaxed) == 0 {
                TrustedHostAttemptExecutionResult::Yielded(
                    TrustedHostYieldRequest::with_time_window(
                        AuthorizedExecutionYieldReason::ContextBudget,
                        crate::AuthorizedExecutionWaitConditionId::new(
                            "wait/supervisor-time-window-follow-up",
                        )
                        .expect("condition"),
                        1,
                        Timestamp::from_offset_date_time(
                            Timestamp::now_utc().as_offset_date_time()
                                + time::Duration::milliseconds(200),
                        ),
                    ),
                )
            } else {
                TrustedHostAttemptExecutionResult::TerminalFailure
            }
        }
    }

    impl TrustedHostAttemptExecutor for YieldTwoTimeWindowsThenFailExecutor<'_> {
        fn binding_commitment(&self) -> SpecContentHash {
            supervisor_executor_binding()
        }

        fn execute(
            &self,
            _context: &TrustedHostAttemptExecutionContext<'_>,
        ) -> TrustedHostAttemptExecutionResult {
            let call = self.calls.fetch_add(1, Ordering::Relaxed);
            if call < 2 {
                TrustedHostAttemptExecutionResult::Yielded(
                    TrustedHostYieldRequest::with_time_window(
                        AuthorizedExecutionYieldReason::ContextBudget,
                        crate::AuthorizedExecutionWaitConditionId::new(format!(
                            "wait/explicit-local-operation-{call}"
                        ))
                        .expect("condition"),
                        1,
                        Timestamp::from_offset_date_time(
                            Timestamp::now_utc().as_offset_date_time()
                                + time::Duration::milliseconds(500),
                        ),
                    ),
                )
            } else {
                TrustedHostAttemptExecutionResult::TerminalFailure
            }
        }
    }

    impl TrustedHostAttemptExecutor for CountingExecutor<'_> {
        fn binding_commitment(&self) -> SpecContentHash {
            supervisor_executor_binding()
        }

        fn execute(
            &self,
            _context: &TrustedHostAttemptExecutionContext<'_>,
        ) -> TrustedHostAttemptExecutionResult {
            self.calls.fetch_add(1, Ordering::Relaxed);
            TrustedHostAttemptExecutionResult::TerminalFailure
        }
    }

    impl TrustedHostAttemptExecutor for DifferentBindingExecutor<'_> {
        fn binding_commitment(&self) -> SpecContentHash {
            SpecContentHash::from_text("different trusted-host executor binding")
        }

        fn execute(
            &self,
            _context: &TrustedHostAttemptExecutionContext<'_>,
        ) -> TrustedHostAttemptExecutionResult {
            self.calls.fetch_add(1, Ordering::Relaxed);
            TrustedHostAttemptExecutionResult::TerminalFailure
        }
    }

    impl TrustedHostAttemptExecutor for BlockingTimeWindowExecutor {
        fn binding_commitment(&self) -> SpecContentHash {
            supervisor_executor_binding()
        }

        fn execute(
            &self,
            _context: &TrustedHostAttemptExecutionContext<'_>,
        ) -> TrustedHostAttemptExecutionResult {
            self.calls.fetch_add(1, Ordering::Relaxed);
            self.entered.send(()).expect("signal executor entry");
            let (lock, signal) = &*self.release;
            let mut released = lock.lock().expect("release lock");
            while !*released {
                released = signal.wait(released).expect("release wait");
            }
            TrustedHostAttemptExecutionResult::Yielded(TrustedHostYieldRequest::with_time_window(
                AuthorizedExecutionYieldReason::ContextBudget,
                crate::AuthorizedExecutionWaitConditionId::new("wait/owner-active-attempt")
                    .expect("condition"),
                1,
                Timestamp::from_offset_date_time(
                    Timestamp::now_utc().as_offset_date_time() + time::Duration::hours(1),
                ),
            ))
        }
    }

    struct RecordingDeadlineWaiter<'a> {
        calls: &'a AtomicUsize,
        expected_deadline: Timestamp,
        outcome: Result<TrustedHostDeadlineWaitOutcome, TrustedHostDeadlineWaitFailure>,
    }

    impl TrustedHostDeadlineWaiter for RecordingDeadlineWaiter<'_> {
        fn wait_until(
            &mut self,
            schedule_at: Timestamp,
        ) -> Result<TrustedHostDeadlineWaitOutcome, TrustedHostDeadlineWaitFailure> {
            self.calls.fetch_add(1, Ordering::Relaxed);
            assert_eq!(schedule_at, self.expected_deadline);
            self.outcome
        }
    }

    struct SequenceDeadlineWaiter<'a> {
        calls: &'a AtomicUsize,
        expected_deadlines: Vec<Option<Timestamp>>,
        delays: Vec<Option<std::time::Duration>>,
        next: usize,
        outcome: TrustedHostDeadlineWaitOutcome,
    }

    impl TrustedHostDeadlineWaiter for SequenceDeadlineWaiter<'_> {
        fn wait_until(
            &mut self,
            schedule_at: Timestamp,
        ) -> Result<TrustedHostDeadlineWaitOutcome, TrustedHostDeadlineWaitFailure> {
            let expected = self
                .expected_deadlines
                .get(self.next)
                .expect("unexpected deadline wait");
            if let Some(expected) = expected {
                assert_eq!(&schedule_at, expected);
            }
            let delay = self.delays[self.next];
            self.next += 1;
            self.calls.fetch_add(1, Ordering::Relaxed);
            if let Some(delay) = delay {
                std::thread::sleep(delay);
            }
            Ok(self.outcome)
        }
    }

    struct DeterministicWakeIdentityProvider {
        calls: usize,
    }

    impl TrustedHostScheduleWakeIdentityProvider for DeterministicWakeIdentityProvider {
        fn next_identity(
            &mut self,
            wake_attempt: u32,
        ) -> Result<TrustedHostScheduleWakeIdentity, WorkflowOsError> {
            self.calls += 1;
            Ok(TrustedHostScheduleWakeIdentity {
                operation_id: ContinuityOperationId::new(format!(
                    "operation/repeated-wake-{wake_attempt}"
                ))?,
                receipt_id: ContinuityReceiptId::new(format!(
                    "receipt/repeated-wake-{wake_attempt}"
                ))?,
            })
        }
    }

    struct DuplicateWakeIdentityProvider;

    impl TrustedHostScheduleWakeIdentityProvider for DuplicateWakeIdentityProvider {
        fn next_identity(
            &mut self,
            _wake_attempt: u32,
        ) -> Result<TrustedHostScheduleWakeIdentity, WorkflowOsError> {
            Ok(TrustedHostScheduleWakeIdentity {
                operation_id: ContinuityOperationId::new("operation/local-timer-duplicate")?,
                receipt_id: ContinuityReceiptId::new("receipt/local-timer-duplicate")?,
            })
        }
    }

    struct BarrierDeadlineWaiter {
        calls: Arc<AtomicUsize>,
        barrier: Arc<Barrier>,
        expected_deadline: Timestamp,
    }

    impl TrustedHostDeadlineWaiter for BarrierDeadlineWaiter {
        fn wait_until(
            &mut self,
            schedule_at: Timestamp,
        ) -> Result<TrustedHostDeadlineWaitOutcome, TrustedHostDeadlineWaitFailure> {
            self.calls.fetch_add(1, Ordering::Relaxed);
            assert_eq!(schedule_at, self.expected_deadline);
            self.barrier.wait();
            Ok(TrustedHostDeadlineWaitOutcome::Woke)
        }
    }

    struct SnapshotTamperingExecutor<'a> {
        calls: &'a AtomicUsize,
        database_path: &'a PathBuf,
    }

    impl TrustedHostAttemptExecutor for SnapshotTamperingExecutor<'_> {
        fn binding_commitment(&self) -> SpecContentHash {
            supervisor_executor_binding()
        }

        fn execute(
            &self,
            _context: &TrustedHostAttemptExecutionContext<'_>,
        ) -> TrustedHostAttemptExecutionResult {
            self.calls.fetch_add(1, Ordering::Relaxed);
            Connection::open(self.database_path)
                .expect("tamper connection")
                .execute(
                    "UPDATE snapshots SET snapshot_commitment=?1",
                    rusqlite::params![SpecContentHash::from_text("tampered snapshot").as_str()],
                )
                .expect("tamper snapshot commitment");
            TrustedHostAttemptExecutionResult::TerminalFailure
        }
    }

    #[derive(Clone, Copy)]
    enum FixedExecutionResult {
        Succeeded,
        RetryableFailure,
        TerminalFailure,
        Ambiguous,
    }

    struct FixedResultExecutor<'a> {
        result: FixedExecutionResult,
        calls: &'a AtomicUsize,
        binding: SpecContentHash,
    }

    impl TrustedHostAttemptExecutor for FixedResultExecutor<'_> {
        fn binding_commitment(&self) -> SpecContentHash {
            self.binding.clone()
        }

        fn execute(
            &self,
            _context: &TrustedHostAttemptExecutionContext<'_>,
        ) -> TrustedHostAttemptExecutionResult {
            self.calls.fetch_add(1, Ordering::Relaxed);
            match self.result {
                FixedExecutionResult::Succeeded => TrustedHostAttemptExecutionResult::Succeeded(
                    SkillOutput::new(BTreeMap::new(), Some("bounded-output".to_owned())),
                ),
                FixedExecutionResult::RetryableFailure => {
                    TrustedHostAttemptExecutionResult::RetryableFailure
                }
                FixedExecutionResult::TerminalFailure => {
                    TrustedHostAttemptExecutionResult::TerminalFailure
                }
                FixedExecutionResult::Ambiguous => {
                    TrustedHostAttemptExecutionResult::AmbiguousMayHaveStarted
                }
            }
        }
    }

    struct SuccessHandler;

    impl SkillHandler for SuccessHandler {
        fn invoke(&self, _input: SkillInput) -> Result<SkillOutput, WorkflowOsError> {
            Ok(SkillOutput::new(
                BTreeMap::from([("status".to_owned(), "ok".to_owned())]),
                Some("local-supervisor-output".to_owned()),
            ))
        }
    }

    fn skill_input(fixture: &Fixture) -> SkillInput {
        SkillInput {
            run_id: fixture.run_id.clone(),
            workflow_id: fixture.workflow_id.clone(),
            workflow_version: WorkflowVersion::new("v1").expect("workflow version"),
            schema_version: SchemaVersion::new("workflowos.dev/v0").expect("schema"),
            spec_hash: SpecContentHash::from_text("opening fixture"),
            step_id: fixture.step_id.clone(),
            skill_id: SkillId::new("local/supervisor-proof").expect("skill"),
            skill_version: SkillVersion::new("v1").expect("skill version"),
            correlation_id: CorrelationId::new("correlation/supervisor-proof")
                .expect("correlation"),
            values: BTreeMap::new(),
        }
    }

    fn supervisor_executor_binding() -> SpecContentHash {
        SpecContentHash::from_text("local skill handler: local/supervisor-proof@v1")
    }

    fn open_supervisor_attempt(
        fixture: &Fixture,
        operation: &str,
        maximum_attempts: u32,
    ) -> Box<OperationalExecutionAttemptUseCapability> {
        let authorization = fixture.authorization();
        let mut request = fixture.request(&authorization, operation);
        request.maximum_attempts = maximum_attempts;
        request.request_commitment = opening_request_commitment(&request);
        let opened = fixture
            .backend
            .open_window_and_start_attempt_projected(request)
            .expect("opening");
        let OperationalExecutionWindowOpeningResult::Opened { capability, .. } = opened.result
        else {
            panic!("opening must return one-use capability");
        };
        capability
    }

    fn register_supervisor_time_window_wait(
        fixture: &Fixture,
        operation: &str,
        deadline: Timestamp,
    ) -> TrustedHostOperationalEntryLocator {
        register_supervisor_time_window_wait_with_attempts(fixture, operation, deadline, 2)
    }

    fn register_supervisor_time_window_wait_with_attempts(
        fixture: &Fixture,
        operation: &str,
        deadline: Timestamp,
        maximum_attempts: u32,
    ) -> TrustedHostOperationalEntryLocator {
        let capability = open_supervisor_attempt(fixture, operation, maximum_attempts);
        let locator = TrustedHostOperationalEntryLocator {
            workflow_id: fixture.workflow_id.clone(),
            run_id: fixture.run_id.clone(),
            step_id: fixture.step_id.clone(),
            window_id: capability.window_id.clone(),
            subject_actor_id: capability.subject_actor_id.clone(),
            immutable_run_bundle: fixture.bundle.clone(),
        };
        supervise_one_local_skill_attempt(TrustedHostSupervisorInput {
            backend: &fixture.backend,
            capability: TrustedHostSupervisorAttemptCapability::Opened(capability),
            executor: &TimeWindowYieldExecutor {
                condition_version: 1,
                deadline,
            },
            skill_input: skill_input(fixture),
            persistence: TrustedHostSupervisorPersistenceInput {
                operation: ContinuityOperationId::new(format!("operation/{operation}-yield"))
                    .expect("operation"),
                receipt: ContinuityReceiptId::new(format!("receipt/{operation}-yield"))
                    .expect("receipt"),
                yield_generation: Some(
                    ContinuityYieldGenerationId::new(format!("yield/{operation}"))
                        .expect("generation"),
                ),
            },
        })
        .expect("initial attempt yields");
        locator
    }

    fn register_resumable_operational_entry(
        fixture: &Fixture,
        operation: &str,
        maximum_attempts: u32,
    ) -> TrustedHostOperationalEntryLocator {
        let capability = open_supervisor_attempt(fixture, operation, maximum_attempts);
        let locator = TrustedHostOperationalEntryLocator {
            workflow_id: fixture.workflow_id.clone(),
            run_id: fixture.run_id.clone(),
            step_id: fixture.step_id.clone(),
            window_id: capability.window_id.clone(),
            subject_actor_id: capability.subject_actor_id.clone(),
            immutable_run_bundle: fixture.bundle.clone(),
        };
        let result = supervise_one_local_skill_attempt(TrustedHostSupervisorInput {
            backend: &fixture.backend,
            capability: TrustedHostSupervisorAttemptCapability::Opened(capability),
            executor: &YieldExecutor,
            skill_input: skill_input(fixture),
            persistence: TrustedHostSupervisorPersistenceInput {
                operation: ContinuityOperationId::new(format!("operation/{operation}-yield"))
                    .expect("operation"),
                receipt: ContinuityReceiptId::new(format!("receipt/{operation}-yield"))
                    .expect("receipt"),
                yield_generation: Some(
                    ContinuityYieldGenerationId::new(format!("yield/{operation}-resume"))
                        .expect("yield generation"),
                ),
            },
        })
        .expect("resumable yielded attempt");
        assert_eq!(
            result.disposition,
            AuthoritativeContinuationDisposition::ResumeNow
        );
        locator
    }

    fn register_supervisor_unsupported_wait(
        fixture: &Fixture,
        operation: &str,
    ) -> TrustedHostOperationalEntryLocator {
        let capability = open_supervisor_attempt(fixture, operation, 2);
        let locator = TrustedHostOperationalEntryLocator {
            workflow_id: fixture.workflow_id.clone(),
            run_id: fixture.run_id.clone(),
            step_id: fixture.step_id.clone(),
            window_id: capability.window_id.clone(),
            subject_actor_id: capability.subject_actor_id.clone(),
            immutable_run_bundle: fixture.bundle.clone(),
        };
        supervise_one_local_skill_attempt(TrustedHostSupervisorInput {
            backend: &fixture.backend,
            capability: TrustedHostSupervisorAttemptCapability::Opened(capability),
            executor: &YieldExecutor,
            skill_input: skill_input(fixture),
            persistence: TrustedHostSupervisorPersistenceInput {
                operation: ContinuityOperationId::new(format!("operation/{operation}-yield"))
                    .expect("operation"),
                receipt: ContinuityReceiptId::new(format!("receipt/{operation}-yield"))
                    .expect("receipt"),
                yield_generation: Some(
                    ContinuityYieldGenerationId::new(format!("yield/{operation}"))
                        .expect("generation"),
                ),
            },
        })
        .expect("initial attempt yields");
        locator
    }

    fn clone_opened_capability(
        capability: &OperationalExecutionAttemptUseCapability,
    ) -> Box<OperationalExecutionAttemptUseCapability> {
        Box::new(OperationalExecutionAttemptUseCapability {
            attempt_id: capability.attempt_id.clone(),
            window_id: capability.window_id.clone(),
            subject_actor_id: capability.subject_actor_id.clone(),
            window_revision: capability.window_revision,
            cursor: capability.cursor.clone(),
            authority_commitment: capability.authority_commitment.clone(),
            window_binding_commitment: capability.window_binding_commitment.clone(),
            expected_window_binding: capability.expected_window_binding.clone(),
            operation_binding_commitment: capability.operation_binding_commitment.clone(),
            consume_operation_id: capability.consume_operation_id.clone(),
            opening_operation_id: capability.opening_operation_id.clone(),
        })
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn one_shot_supervisor_yields_resumes_and_records_success_without_completing_run() {
        let fixture = Fixture::new();
        let authorization = fixture.authorization();
        let mut opening_request = fixture.request(&authorization, "opening/supervisor-proof");
        opening_request.maximum_attempts = 2;
        opening_request.request_commitment = opening_request_commitment(&opening_request);
        let opened = fixture
            .backend
            .open_window_and_start_attempt_projected(opening_request)
            .expect("opening");
        let OperationalExecutionWindowOpeningResult::Opened {
            capability: opened_capability,
            ..
        } = opened.result
        else {
            panic!("opening must return one-use capability");
        };
        let input = skill_input(&fixture);
        let generation_id =
            ContinuityYieldGenerationId::new("yield/supervisor-proof/1").expect("generation");
        let opened_window_id = opened_capability.window_id.clone();
        let opened_subject_actor_id = opened_capability.subject_actor_id.clone();
        let opened_authority_commitment = opened_capability.authority_commitment.clone();
        let opened_expected_binding = opened_capability.expected_window_binding.clone();
        let opened_operation_binding = opened_capability.operation_binding_commitment.clone();

        let yielded = supervise_one_local_skill_attempt(TrustedHostSupervisorInput {
            backend: &fixture.backend,
            capability: TrustedHostSupervisorAttemptCapability::Opened(opened_capability),
            executor: &YieldExecutor,
            skill_input: input.clone(),
            persistence: TrustedHostSupervisorPersistenceInput {
                operation: ContinuityOperationId::new("operation/supervisor-yield")
                    .expect("operation"),
                receipt: ContinuityReceiptId::new("receipt/supervisor-yield").expect("receipt"),
                yield_generation: Some(generation_id.clone()),
            },
        })
        .expect("yield persists");
        assert_eq!(
            yielded.disposition,
            crate::authorized_execution_continuity_state::internal::AuthoritativeContinuationDisposition::ResumeNow
        );

        let yielded_run = fixture.backend.rehydrate_run(&fixture.run_id).expect("run");
        let yielded_cursor = ContinuityCursor {
            sequence_number: yielded_run.snapshot.last_sequence_number,
            event_id: yielded_run.snapshot.last_event_id.clone(),
        };
        let yielded_connection = fixture.backend.connection().expect("SQLite connection");
        let yielded_state =
            crate::sqlite_state::continuity_codec::load_snapshot(&yielded_connection)
                .expect("continuity snapshot");
        let yielded_window_revision = yielded_state
            .windows
            .get(&opened_window_id)
            .expect("yielded window")
            .revision;
        let mut resumed_binding = opened_expected_binding;
        resumed_binding.cursor = yielded_cursor.clone();
        let consume_operation_id =
            ContinuityOperationId::new("operation/supervisor-resume").expect("operation");
        let generated_attempt_id =
            AuthorizedExecutionAttemptId::new("attempt/supervisor-resume").expect("attempt");
        let mut consume_request = ConsumeDirectiveRequest {
            operation_id: consume_operation_id,
            request_commitment: SpecContentHash::from_text("pending resume"),
            receipt_id: ContinuityReceiptId::new("receipt/supervisor-resume").expect("receipt"),
            directive_id: ContinuityDirectiveId::new(format!(
                "directive/{}",
                generation_id.as_str()
            ))
            .expect("directive"),
            window_id: opened_window_id.clone(),
            expected_window_revision: yielded_window_revision,
            expected_window_binding: resumed_binding.clone(),
            generation_id,
            cursor: yielded_cursor.clone(),
            expected_waits: Vec::new(),
            authority_capability: AuthorityUseCapability {
                window_id: opened_window_id,
                window_revision: yielded_window_revision,
                generation_id: ContinuityYieldGenerationId::new("yield/supervisor-proof/1")
                    .expect("generation"),
                cursor: yielded_cursor,
                subject_actor_id: opened_subject_actor_id,
                authority_commitment: opened_authority_commitment,
                window_binding_commitment: window_binding_commitment(&resumed_binding),
                expected_waits: Vec::new(),
            },
            generated_attempt_id,
        };
        consume_request.request_commitment =
            expected_consume_directive_commitment(&consume_request);
        let consumed = fixture
            .backend
            .consume_directive_projected(consume_request)
            .expect("directive consumption");
        let ConsumeDirectiveResult::Consumed {
            capability: resumed_capability,
            ..
        } = consumed.result
        else {
            panic!("directive must produce one-use attempt capability");
        };
        let resumed_run = fixture.backend.rehydrate_run(&fixture.run_id).expect("run");
        resumed_binding.cursor = ContinuityCursor {
            sequence_number: resumed_run.snapshot.last_sequence_number,
            event_id: resumed_run.snapshot.last_event_id.clone(),
        };

        let handler = SuccessHandler;
        let adapter = LocalSkillAttemptExecutor::new(&handler, supervisor_executor_binding());
        let completed = supervise_one_local_skill_attempt(TrustedHostSupervisorInput {
            backend: &fixture.backend,
            capability: TrustedHostSupervisorAttemptCapability::Resumed {
                capability: Box::new(resumed_capability),
                expected_window_binding: Box::new(resumed_binding),
                operation_binding_commitment: opened_operation_binding,
            },
            executor: &adapter,
            skill_input: input,
            persistence: TrustedHostSupervisorPersistenceInput {
                operation: ContinuityOperationId::new("operation/supervisor-success")
                    .expect("operation"),
                receipt: ContinuityReceiptId::new("receipt/supervisor-success").expect("receipt"),
                yield_generation: None,
            },
        })
        .expect("success persists");

        assert_eq!(
            completed.disposition,
            crate::authorized_execution_continuity_state::internal::AuthoritativeContinuationDisposition::Terminal
        );
        let debug = format!("{completed:?}");
        assert!(!debug.contains("local-supervisor-output"));
        assert!(!debug.contains("status"));
        assert_eq!(
            completed
                .skill_output
                .expect("bounded output")
                .output_ref
                .as_deref(),
            Some("local-supervisor-output")
        );
        let final_run = fixture.backend.rehydrate_run(&fixture.run_id).expect("run");
        assert_eq!(final_run.snapshot.status, WorkflowRunStatus::Running);
        assert_eq!(final_run.events.len(), 9);
    }

    #[test]
    fn one_shot_supervisor_registers_one_bound_time_window_wait_atomically() {
        let fixture = Fixture::new();
        let capability = open_supervisor_attempt(&fixture, "opening/time-window-yield", 2);
        let window_id = capability.window_id.clone();
        let generation_id =
            ContinuityYieldGenerationId::new("yield/supervisor-time-window/1").expect("generation");
        let deadline = Timestamp::parse_rfc3339("2099-09-30T10:05:00Z").expect("deadline");
        let result = supervise_one_local_skill_attempt(TrustedHostSupervisorInput {
            backend: &fixture.backend,
            capability: TrustedHostSupervisorAttemptCapability::Opened(capability),
            executor: &TimeWindowYieldExecutor {
                condition_version: 1,
                deadline,
            },
            skill_input: skill_input(&fixture),
            persistence: TrustedHostSupervisorPersistenceInput {
                operation: ContinuityOperationId::new("operation/supervisor-time-window-yield")
                    .expect("operation"),
                receipt: ContinuityReceiptId::new("receipt/supervisor-time-window-yield")
                    .expect("receipt"),
                yield_generation: Some(generation_id.clone()),
            },
        })
        .expect("time-window yield persists");
        assert_eq!(
            result.disposition,
            crate::authorized_execution_continuity_state::internal::AuthoritativeContinuationDisposition::AwaitCondition
        );

        let state = crate::sqlite_state::continuity_codec::load_snapshot(
            &fixture.backend.connection().expect("connection"),
        )
        .expect("continuity snapshot");
        let yielded = state.yields.get(&generation_id).expect("yield record");
        assert_eq!(yielded.wait_ids.len(), 1);
        let wait = state.waits.get(&yielded.wait_ids[0]).expect("wait record");
        assert_eq!(wait.window_id, window_id);
        assert_eq!(wait.generation_id, generation_id);
        assert_eq!(wait.condition_version, 1);
        assert_eq!(
            wait.wake_trigger,
            crate::AuthorizedExecutionWakeTriggerKind::DeadlineReached
        );
        assert_eq!(
            wait.state,
            crate::authorized_execution_continuity_state::internal::AuthoritativeWaitState::Unsatisfied
        );
        let Some(
            crate::authorized_execution_continuity_state::internal::AuthoritativeWaitDependencyBinding::TimeWindow {
                deadline: stored_deadline,
                source,
                provenance_commitment,
                epoch_id,
                ..
            },
        ) = &wait.dependency_binding
        else {
            panic!("time-window dependency binding must be stored");
        };
        assert_eq!(*stored_deadline, deadline);
        assert_eq!(*source, state.trusted_time.source);
        assert_eq!(
            provenance_commitment,
            &state.trusted_time.provenance_commitment
        );
        assert_eq!(epoch_id, &state.trusted_time.epoch_id);
    }

    #[test]
    fn one_shot_supervisor_rejects_invalid_time_window_declaration_without_wait() {
        let fixture = Fixture::new();
        let capability = open_supervisor_attempt(&fixture, "opening/invalid-time-window-yield", 2);
        let error = supervise_one_local_skill_attempt(TrustedHostSupervisorInput {
            backend: &fixture.backend,
            capability: TrustedHostSupervisorAttemptCapability::Opened(capability),
            executor: &TimeWindowYieldExecutor {
                condition_version: 0,
                deadline: Timestamp::parse_rfc3339("2099-09-30T10:05:00Z").expect("deadline"),
            },
            skill_input: skill_input(&fixture),
            persistence: TrustedHostSupervisorPersistenceInput {
                operation: ContinuityOperationId::new("operation/invalid-time-window-yield")
                    .expect("operation"),
                receipt: ContinuityReceiptId::new("receipt/invalid-time-window-yield")
                    .expect("receipt"),
                yield_generation: Some(
                    ContinuityYieldGenerationId::new("yield/invalid-time-window/1")
                        .expect("generation"),
                ),
            },
        })
        .expect_err("invalid condition version must fail closed");
        assert_eq!(
            error.code(),
            "trusted_host_supervisor.wait_declaration_invalid"
        );

        let state = crate::sqlite_state::continuity_codec::load_snapshot(
            &fixture.backend.connection().expect("connection"),
        )
        .expect("continuity snapshot");
        assert!(state.yields.is_empty());
        assert!(state.waits.is_empty());
        assert!(!format!(
            "{:?}",
            TrustedHostYieldRequest::with_time_window(
                AuthorizedExecutionYieldReason::ContextBudget,
                crate::AuthorizedExecutionWaitConditionId::new("wait/private-context")
                    .expect("condition"),
                1,
                Timestamp::parse_rfc3339("2099-09-30T10:05:00Z").expect("deadline"),
            )
        )
        .contains("private-context"));
    }

    #[test]
    fn one_shot_supervisor_rejects_time_window_deadline_outside_authorized_window() {
        let fixture = Fixture::new();
        let capability = open_supervisor_attempt(&fixture, "opening/outside-time-window-yield", 2);
        let forbidden_deadline = "2100-09-30T10:05:00Z";
        let error = supervise_one_local_skill_attempt(TrustedHostSupervisorInput {
            backend: &fixture.backend,
            capability: TrustedHostSupervisorAttemptCapability::Opened(capability),
            executor: &TimeWindowYieldExecutor {
                condition_version: 1,
                deadline: Timestamp::parse_rfc3339(forbidden_deadline).expect("deadline"),
            },
            skill_input: skill_input(&fixture),
            persistence: TrustedHostSupervisorPersistenceInput {
                operation: ContinuityOperationId::new("operation/outside-time-window-yield")
                    .expect("operation"),
                receipt: ContinuityReceiptId::new("receipt/outside-time-window-yield")
                    .expect("receipt"),
                yield_generation: Some(
                    ContinuityYieldGenerationId::new("yield/outside-time-window/1")
                        .expect("generation"),
                ),
            },
        })
        .expect_err("deadline outside the authorized window must fail closed");
        assert_eq!(
            error.code(),
            "sqlite_state.time_window_wait.binding_invalid"
        );
        assert!(!error.to_string().contains(forbidden_deadline));

        let state = crate::sqlite_state::continuity_codec::load_snapshot(
            &fixture.backend.connection().expect("connection"),
        )
        .expect("continuity snapshot");
        assert!(state.yields.is_empty());
        assert!(state.waits.is_empty());
    }

    #[test]
    fn one_shot_supervisor_rejects_substituted_binding_before_executor_entry() {
        let fixture = Fixture::new();
        let authorization = fixture.authorization();
        let opening_request = fixture.request(&authorization, "opening/binding-mismatch");
        let opened = fixture
            .backend
            .open_window_and_start_attempt_projected(opening_request)
            .expect("opening");
        let OperationalExecutionWindowOpeningResult::Opened {
            capability: opened_capability,
            ..
        } = opened.result
        else {
            panic!("opening must return one-use capability");
        };
        let mut input = skill_input(&fixture);
        input.step_id = StepId::new("substituted-step").expect("step");
        let calls = AtomicUsize::new(0);
        let executor = CountingExecutor { calls: &calls };

        let error = supervise_one_local_skill_attempt(TrustedHostSupervisorInput {
            backend: &fixture.backend,
            capability: TrustedHostSupervisorAttemptCapability::Opened(opened_capability),
            executor: &executor,
            skill_input: input,
            persistence: TrustedHostSupervisorPersistenceInput {
                operation: ContinuityOperationId::new("operation/binding-mismatch")
                    .expect("operation"),
                receipt: ContinuityReceiptId::new("receipt/binding-mismatch").expect("receipt"),
                yield_generation: None,
            },
        })
        .expect_err("substituted binding must fail closed");

        assert_eq!(
            error.code(),
            "trusted_host_supervisor.invocation_binding_mismatch"
        );
        assert_eq!(calls.load(Ordering::Relaxed), 0);
        assert_eq!(
            fixture
                .backend
                .read_events(&fixture.run_id)
                .expect("events")
                .len(),
            4
        );
    }

    #[test]
    fn one_shot_supervisor_records_closed_result_matrix() {
        for (index, result, maximum_attempts, expected) in [
            (
                1,
                FixedExecutionResult::Succeeded,
                1,
                crate::authorized_execution_continuity_state::internal::AuthoritativeContinuationDisposition::Terminal,
            ),
            (
                2,
                FixedExecutionResult::RetryableFailure,
                2,
                crate::authorized_execution_continuity_state::internal::AuthoritativeContinuationDisposition::Terminal,
            ),
            (
                3,
                FixedExecutionResult::TerminalFailure,
                1,
                crate::authorized_execution_continuity_state::internal::AuthoritativeContinuationDisposition::Terminal,
            ),
            (
                4,
                FixedExecutionResult::Ambiguous,
                1,
                crate::authorized_execution_continuity_state::internal::AuthoritativeContinuationDisposition::Blocked,
            ),
        ] {
            let fixture = Fixture::new();
            let capability = open_supervisor_attempt(
                &fixture,
                &format!("opening/result-matrix-{index}"),
                maximum_attempts,
            );
            let calls = AtomicUsize::new(0);
            let executor = FixedResultExecutor {
                result,
                calls: &calls,
                binding: supervisor_executor_binding(),
            };
            let result = supervise_one_local_skill_attempt(TrustedHostSupervisorInput {
                backend: &fixture.backend,
                capability: TrustedHostSupervisorAttemptCapability::Opened(capability),
                executor: &executor,
                skill_input: skill_input(&fixture),
                persistence: TrustedHostSupervisorPersistenceInput {
                    operation: ContinuityOperationId::new(format!(
                        "operation/result-matrix-{index}"
                    ))
                    .expect("operation"),
                    receipt: ContinuityReceiptId::new(format!("receipt/result-matrix-{index}"))
                        .expect("receipt"),
                    yield_generation: None,
                },
            })
            .expect("closed result persists");
            assert_eq!(result.disposition, expected);
            assert_eq!(calls.load(Ordering::Relaxed), 1);
        }
    }

    #[test]
    fn one_shot_supervisor_rejects_missing_yield_generation() {
        let fixture = Fixture::new();
        let capability = open_supervisor_attempt(&fixture, "opening/missing-yield", 2);
        let error = supervise_one_local_skill_attempt(TrustedHostSupervisorInput {
            backend: &fixture.backend,
            capability: TrustedHostSupervisorAttemptCapability::Opened(capability),
            executor: &YieldExecutor,
            skill_input: skill_input(&fixture),
            persistence: TrustedHostSupervisorPersistenceInput {
                operation: ContinuityOperationId::new("operation/missing-yield")
                    .expect("operation"),
                receipt: ContinuityReceiptId::new("receipt/missing-yield").expect("receipt"),
                yield_generation: None,
            },
        })
        .expect_err("yield generation is required");
        assert_eq!(
            error.code(),
            "trusted_host_supervisor.yield_generation_missing"
        );
    }

    #[test]
    fn consumed_or_stale_attempt_cannot_reenter_executor() {
        let fixture = Fixture::new();
        let capability = open_supervisor_attempt(&fixture, "opening/one-use", 1);
        let stale = clone_opened_capability(&capability);
        let calls = AtomicUsize::new(0);
        let executor = FixedResultExecutor {
            result: FixedExecutionResult::TerminalFailure,
            calls: &calls,
            binding: supervisor_executor_binding(),
        };
        supervise_one_local_skill_attempt(TrustedHostSupervisorInput {
            backend: &fixture.backend,
            capability: TrustedHostSupervisorAttemptCapability::Opened(capability),
            executor: &executor,
            skill_input: skill_input(&fixture),
            persistence: TrustedHostSupervisorPersistenceInput {
                operation: ContinuityOperationId::new("operation/one-use").expect("operation"),
                receipt: ContinuityReceiptId::new("receipt/one-use").expect("receipt"),
                yield_generation: None,
            },
        })
        .expect("first dispatch");
        let run = fixture.backend.rehydrate_run(&fixture.run_id).expect("run");
        let opening = run
            .snapshot
            .last_operational_opening_projection
            .as_ref()
            .expect("opening");
        let actor = ActorId::new("agent/opening-test").expect("actor");
        assert!(!fixture
            .backend
            .fixture_predispatch_attempt_is_current(
                &run,
                Some(&actor),
                Some(&fixture.step_id),
                opening,
            )
            .expect("consumed attempt inspection"));
        let error = supervise_one_local_skill_attempt(TrustedHostSupervisorInput {
            backend: &fixture.backend,
            capability: TrustedHostSupervisorAttemptCapability::Opened(stale),
            executor: &executor,
            skill_input: skill_input(&fixture),
            persistence: TrustedHostSupervisorPersistenceInput {
                operation: ContinuityOperationId::new("operation/one-use-replay")
                    .expect("operation"),
                receipt: ContinuityReceiptId::new("receipt/one-use-replay").expect("receipt"),
                yield_generation: None,
            },
        })
        .expect_err("stale attempt must not dispatch");
        assert_eq!(
            error.code(),
            "dispatch_reservation.attempt_not_dispatchable"
        );
        assert_eq!(calls.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn full_invocation_and_executor_substitutions_fail_before_dispatch() {
        for substitution in 0..10 {
            let fixture = Fixture::new();
            let capability = open_supervisor_attempt(
                &fixture,
                &format!("opening/full-binding-{substitution}"),
                1,
            );
            let mut input = skill_input(&fixture);
            match substitution {
                0 => input.workflow_id = WorkflowId::new("local/substituted").expect("workflow"),
                1 => input.run_id = WorkflowRunId::new("run-substituted").expect("run"),
                2 => input.workflow_version = WorkflowVersion::new("v2").expect("version"),
                3 => {
                    input.schema_version = SchemaVersion::new("workflowos.dev/v1").expect("schema");
                }
                4 => input.spec_hash = SpecContentHash::from_text("substituted spec"),
                5 => input.skill_id = SkillId::new("local/substituted").expect("skill"),
                6 => input.skill_version = SkillVersion::new("v2").expect("version"),
                7 => {
                    input.correlation_id =
                        CorrelationId::new("correlation/substituted").expect("correlation");
                }
                8 => {
                    input.values.insert("mode".to_owned(), "changed".to_owned());
                }
                9 => {}
                _ => unreachable!(),
            }
            let calls = AtomicUsize::new(0);
            let executor = FixedResultExecutor {
                result: FixedExecutionResult::TerminalFailure,
                calls: &calls,
                binding: if substitution == 9 {
                    SpecContentHash::from_text("substituted executor")
                } else {
                    supervisor_executor_binding()
                },
            };
            let error = supervise_one_local_skill_attempt(TrustedHostSupervisorInput {
                backend: &fixture.backend,
                capability: TrustedHostSupervisorAttemptCapability::Opened(capability),
                executor: &executor,
                skill_input: input,
                persistence: TrustedHostSupervisorPersistenceInput {
                    operation: ContinuityOperationId::new(format!(
                        "operation/full-binding-{substitution}"
                    ))
                    .expect("operation"),
                    receipt: ContinuityReceiptId::new(format!(
                        "receipt/full-binding-{substitution}"
                    ))
                    .expect("receipt"),
                    yield_generation: None,
                },
            })
            .expect_err("substitution must fail closed");
            assert_eq!(
                error.code(),
                "trusted_host_supervisor.invocation_binding_mismatch"
            );
            assert_eq!(calls.load(Ordering::Relaxed), 0);
        }
    }

    #[test]
    fn ambiguous_persistence_is_reconciled_on_a_fresh_connection() {
        for (index, fault, expected_code) in [
            (1, InjectedSupervisorPersistenceFault::After, None),
            (
                2,
                InjectedSupervisorPersistenceFault::Before,
                Some("trusted_host_supervisor.outcome_not_committed"),
            ),
        ] {
            let fixture = Fixture::new();
            let capability =
                open_supervisor_attempt(&fixture, &format!("opening/reconciliation-{index}"), 1);
            let calls = AtomicUsize::new(0);
            let executor = FixedResultExecutor {
                result: FixedExecutionResult::TerminalFailure,
                calls: &calls,
                binding: supervisor_executor_binding(),
            };
            inject_supervisor_persistence_fault(fault);
            let result = supervise_one_local_skill_attempt(TrustedHostSupervisorInput {
                backend: &fixture.backend,
                capability: TrustedHostSupervisorAttemptCapability::Opened(capability),
                executor: &executor,
                skill_input: skill_input(&fixture),
                persistence: TrustedHostSupervisorPersistenceInput {
                    operation: ContinuityOperationId::new(format!(
                        "operation/reconciliation-{index}"
                    ))
                    .expect("operation"),
                    receipt: ContinuityReceiptId::new(format!("receipt/reconciliation-{index}"))
                        .expect("receipt"),
                    yield_generation: None,
                },
            });
            match expected_code {
                Some(code) => assert_eq!(result.expect_err("confirmed absent").code(), code),
                None => assert_eq!(
                    result.expect("durably committed").disposition,
                    crate::authorized_execution_continuity_state::internal::AuthoritativeContinuationDisposition::Terminal
                ),
            }
            assert_eq!(calls.load(Ordering::Relaxed), 1);
        }
    }

    #[test]
    fn dispatch_reservation_failure_prevents_executor_entry() {
        let fixture = Fixture::new();
        let capability = open_supervisor_attempt(&fixture, "opening/dispatch-before", 1);
        let calls = AtomicUsize::new(0);
        let executor = CountingExecutor { calls: &calls };
        inject_dispatch_commit_fault(InjectedDispatchCommitFault::Before);
        let error = supervise_one_local_skill_attempt(TrustedHostSupervisorInput {
            backend: &fixture.backend,
            capability: TrustedHostSupervisorAttemptCapability::Opened(capability),
            executor: &executor,
            skill_input: skill_input(&fixture),
            persistence: TrustedHostSupervisorPersistenceInput {
                operation: ContinuityOperationId::new("operation/dispatch-before")
                    .expect("operation"),
                receipt: ContinuityReceiptId::new("receipt/dispatch-before").expect("receipt"),
                yield_generation: None,
            },
        })
        .expect_err("uncommitted reservation must fail closed");
        assert_eq!(error.code(), "dispatch_reservation.write_failed");
        assert_eq!(calls.load(Ordering::Relaxed), 0);
        assert_eq!(
            fixture
                .backend
                .read_events(&fixture.run_id)
                .expect("events")
                .len(),
            4
        );
    }

    #[test]
    fn ambiguous_dispatch_commit_never_reissues_executor_capability() {
        let fixture = Fixture::new();
        let capability = open_supervisor_attempt(&fixture, "opening/dispatch-after", 1);
        let replay = clone_opened_capability(&capability);
        let calls = AtomicUsize::new(0);
        let executor = CountingExecutor { calls: &calls };
        inject_dispatch_commit_fault(InjectedDispatchCommitFault::After);
        let input = skill_input(&fixture);
        let error = supervise_one_local_skill_attempt(TrustedHostSupervisorInput {
            backend: &fixture.backend,
            capability: TrustedHostSupervisorAttemptCapability::Opened(capability),
            executor: &executor,
            skill_input: input.clone(),
            persistence: TrustedHostSupervisorPersistenceInput {
                operation: ContinuityOperationId::new("operation/dispatch-after")
                    .expect("operation"),
                receipt: ContinuityReceiptId::new("receipt/dispatch-after").expect("receipt"),
                yield_generation: None,
            },
        })
        .expect_err("ambiguous committed reservation must withhold capability");
        assert_eq!(
            error.code(),
            "trusted_host_supervisor.dispatch_capability_unavailable"
        );
        assert_eq!(calls.load(Ordering::Relaxed), 0);

        let replay_error = supervise_one_local_skill_attempt(TrustedHostSupervisorInput {
            backend: &fixture.backend,
            capability: TrustedHostSupervisorAttemptCapability::Opened(replay),
            executor: &executor,
            skill_input: input,
            persistence: TrustedHostSupervisorPersistenceInput {
                operation: ContinuityOperationId::new("operation/dispatch-after")
                    .expect("operation"),
                receipt: ContinuityReceiptId::new("receipt/dispatch-after").expect("receipt"),
                yield_generation: None,
            },
        })
        .expect_err("exact replay must not mint a second capability");
        assert_eq!(
            replay_error.code(),
            "trusted_host_supervisor.attempt_already_admitted"
        );
        assert_eq!(calls.load(Ordering::Relaxed), 0);
        assert_eq!(
            fixture
                .backend
                .read_events(&fixture.run_id)
                .expect("events")
                .len(),
            5
        );
    }

    fn checked_reservation_request(
        opened: &OperationalExecutionAttemptUseCapability,
    ) -> crate::dispatch_reservation::DispatchReservationRequest {
        use crate::authorized_execution_continuity_state::internal::AttemptUseCapability;
        use crate::dispatch_reservation::{
            request_commitment, DispatchReservationOperationId, DispatchReservationReceiptId,
            DispatchReservationRequest,
        };

        let capability = clone_opened_capability(opened);
        let mut input = DispatchReservationRequest {
            operation_id: DispatchReservationOperationId::new("dispatch/checked-reservation")
                .expect("operation"),
            receipt_id: DispatchReservationReceiptId::new("receipt/checked-reservation")
                .expect("receipt"),
            request_commitment: SpecContentHash::from_text("pending reservation"),
            capability: AttemptUseCapability {
                attempt_id: capability.attempt_id,
                subject_actor_id: capability.subject_actor_id,
                window_id: capability.window_id,
                window_revision: capability.window_revision,
                cursor: capability.cursor,
                authority_commitment: capability.authority_commitment,
                window_binding_commitment: capability.window_binding_commitment,
                consume_operation_id: capability.consume_operation_id,
                dispatch_reservation: None,
            },
            expected_window_binding: capability.expected_window_binding,
            operation_binding_commitment: capability.operation_binding_commitment,
            invocation_commitment: SpecContentHash::from_text("synthetic request"),
            executor_commitment: SpecContentHash::from_text("disposable executor"),
            workflow_lifecycle_commitment: None,
        };
        input.request_commitment = request_commitment(&input);
        input
    }

    #[allow(clippy::too_many_lines)]
    fn persist_live_grant_test_approval(
        fixture: &Fixture,
        facts: crate::live_sandbox_continuation_authority::LiveSandboxContinuationFacts<'_>,
        trial: LiveGrantTrial,
    ) -> ContinuityCursor {
        use crate::{
            ApprovalPresentationRecordStore, ApprovalRequest, ExplicitLocalCheckProfileId,
            LocalCheckCommandId, LocalCheckCommandKind, LocalCheckResultRecordedEvent,
            LocalCheckResultRecordedEventDefinition, LocalCheckResultStatus, SkillInvocationId,
        };
        let mut event = fixture
            .backend
            .read_events(&fixture.run_id)
            .expect("history")
            .pop()
            .expect("running event");
        let request = ApprovalRequest {
            approval_id: facts.approval.as_str().to_owned(),
            run_id: fixture.run_id.clone(),
            workflow_id: fixture.workflow_id.clone(),
            schema_version: event.schema_version.clone(),
            workflow_version: event.workflow_version.clone(),
            spec_content_hash: event.spec_content_hash.clone(),
            resolved_execution_context_hash: Some(SpecContentHash::from_text("live context")),
            step_id: Some(fixture.step_id.clone()),
            skill_id: Some(SkillId::new("local/comment").expect("skill")),
            skill_version: Some(SkillVersion::new("v1").expect("skill version")),
            governance_approval_binding: None,
            requested_by: facts.action.original_actor.clone(),
            correlation_id: facts.decision.correlation_id.clone(),
            idempotency_key: Some(facts.action.idempotency_key.clone()),
            reason: "Exact synthetic sandbox comment".to_owned(),
            requested_at: facts.decision.decided_at,
            expires_after: None,
            expires_at: facts.approval_expires_at,
            decision: None,
        };
        let invocation = SkillInvocationId::new("invocation/live-check").expect("invocation");
        let terminal = EventId::new("event/live-check-succeeded").expect("terminal");
        let check = LocalCheckResultRecordedEvent::new(LocalCheckResultRecordedEventDefinition {
            result_id: facts.requirements.check_ids()[0].clone(),
            command_id: LocalCheckCommandId::new("local-check/docs").expect("command"),
            command_kind: LocalCheckCommandKind::DocsCheck,
            status: if matches!(trial, LiveGrantTrial::FailedCheck) {
                LocalCheckResultStatus::Failed
            } else {
                LocalCheckResultStatus::Passed
            },
            step_id: fixture.step_id.clone(),
            invocation_id: invocation.clone(),
            attempt_id: AuthorizedExecutionAttemptId::new("attempt/live-check").expect("attempt"),
            profile_id: ExplicitLocalCheckProfileId::DocsCheck,
            immutable_run_bundle: fixture.bundle.clone(),
            invocation_terminal_event_id: terminal.clone(),
        })
        .expect("check");
        let history = [
            (
                EventId::new("event/live-check").expect("check event"),
                WorkflowRunEventKind::LocalCheckResultRecorded(Box::new(check)),
            ),
            (
                terminal,
                WorkflowRunEventKind::SkillInvocationSucceeded {
                    invocation_id: invocation,
                    step_id: fixture.step_id.clone(),
                    skill_id: SkillId::new("local/check").expect("skill"),
                    skill_version: SkillVersion::new("v1").expect("version"),
                    output_ref: None,
                },
            ),
            (
                EventId::new("event/live-approval-requested").expect("request"),
                WorkflowRunEventKind::ApprovalRequested(Box::new(request)),
            ),
            (
                facts.approval_event.clone(),
                WorkflowRunEventKind::ApprovalGranted(facts.decision.clone()),
            ),
            (
                EventId::new("event/live-run-resumed").expect("resume"),
                WorkflowRunEventKind::RunResumed,
            ),
        ];
        for (id, kind) in history {
            event.event_id = id;
            event.idempotency_key =
                Some(IdempotencyKey::new(event.event_id.as_str()).expect("event idempotency"));
            event.sequence_number =
                EventSequenceNumber::new(event.sequence_number.get() + 1).expect("sequence");
            event.kind = kind;
            event.timestamp = facts.decision.decided_at;
            fixture
                .backend
                .append_event(&event)
                .expect("append approval history");
        }
        fixture
            .backend
            .write_approval_presentation_record(facts.presentation)
            .expect("persist exact presentation");
        ContinuityCursor {
            sequence_number: event.sequence_number,
            event_id: event.event_id,
        }
    }

    fn live_test_effect(
        fixture: &Fixture,
        facts: LiveSandboxContinuationFacts<'_>,
        opening_event: &EventId,
        policy_event: &EventId,
    ) -> crate::SideEffectRecord {
        use crate::{
            RedactionMetadata, SideEffectAuthority, SideEffectAuthorityDecision,
            SideEffectCapability, SideEffectIdempotencyBinding, SideEffectIdempotencyScope,
            SideEffectLifecycleState, SideEffectRecord, SideEffectRecordDefinition,
            SideEffectReference, SideEffectReferenceKind, SideEffectSensitivity,
            SideEffectTargetKind, SideEffectTargetReference,
        };
        let run = fixture.backend.rehydrate_run(&fixture.run_id).expect("run");
        let identity = run.snapshot.identity;
        let reference =
            |kind, value: &str| SideEffectReference::new(kind, value).expect("reference");
        SideEffectRecord::new(SideEffectRecordDefinition {
            side_effect_id: facts.action.side_effect.clone(),
            lifecycle_state: SideEffectLifecycleState::Attempted,
            target: SideEffectTargetReference::new(
                SideEffectTargetKind::AdapterResource,
                facts.action.target.reference(),
            )
            .expect("target"),
            capability: SideEffectCapability::GitHubWrite,
            authority: SideEffectAuthority::new(
                SideEffectAuthorityDecision::ApprovedByHuman,
                vec![reference(
                    SideEffectReferenceKind::PolicyDecision,
                    policy_event.as_str(),
                )],
                vec![reference(
                    SideEffectReferenceKind::ApprovalDecision,
                    facts.approval.as_str(),
                )],
            )
            .expect("authority"),
            actor: Some(facts.action.original_actor.clone()),
            system_actor: None,
            workflow_id: identity.workflow_id,
            workflow_version: identity.workflow_version,
            schema_version: identity.schema_version,
            spec_hash: identity.spec_content_hash,
            run_id: identity.run_id,
            step_id: Some(facts.action.step.clone()),
            skill_id: None,
            skill_version: None,
            adapter_id: None,
            adapter_kind: None,
            integration_id: None,
            idempotency: SideEffectIdempotencyBinding::new(
                facts.action.idempotency_key.clone(),
                SideEffectIdempotencyScope::Run,
                None,
                None,
            )
            .expect("idempotency"),
            references: vec![
                reference(
                    SideEffectReferenceKind::WorkflowEvent,
                    opening_event.as_str(),
                ),
                reference(
                    SideEffectReferenceKind::LocalCheckResult,
                    facts.requirements.check_ids()[0].as_str(),
                ),
            ],
            outcome_reference: None,
            created_at: facts.decision.decided_at,
            updated_at: Some(facts.decision.decided_at),
            correlation_id: Some(facts.decision.correlation_id.clone()),
            summary: Some("exact synthetic attempted effect".to_owned()),
            reason_codes: vec![],
            sensitivity: SideEffectSensitivity::Internal,
            redaction: RedactionMetadata::empty(),
        })
        .expect("attempted effect")
    }

    fn seed_durable_live_policy(
        fixture: &Fixture,
        facts: LiveSandboxContinuationFacts<'_>,
        trial: LiveGrantTrial,
        provision: &crate::live_sandbox_continuation_authority::LiveSandboxPolicyTarget,
    ) -> EventId {
        let synthetic = |id: &str, allowed| {
            let id = EventId::new(id).expect("policy event ID");
            let mut event = fixture
                .backend
                .read_events(&fixture.run_id)
                .expect("history")
                .pop()
                .expect("last event");
            let mut policy = facts
                .scoped_policy_decision(provision, Timestamp::now_utc())
                .expect("fixture policy");
            policy.allowed = allowed;
            policy.reason_codes.clear();
            event.event_id = id.clone();
            event.idempotency_key = None;
            event.timestamp = Timestamp::now_utc();
            event.sequence_number =
                EventSequenceNumber::new(event.sequence_number.get() + 1).expect("sequence");
            event.kind = WorkflowRunEventKind::PolicyDecisionRecorded(Box::new(policy));
            fixture
                .backend
                .append_event(&event)
                .expect("append synthetic policy");
            id
        };
        if matches!(trial, LiveGrantTrial::GenericPolicy) {
            return synthetic("event/live-generic-allow", true);
        }
        if matches!(trial, LiveGrantTrial::MissingBundle) {
            assert!(fixture.backend.record_live_sandbox_policy(facts).is_err());
            return EventId::new("event/no-durable-policy").expect("missing policy");
        }
        let id = fixture
            .backend
            .record_live_sandbox_policy(facts)
            .expect("record exact policy");
        assert_eq!(
            fixture
                .backend
                .record_live_sandbox_policy(facts)
                .expect("exact retry"),
            id
        );
        if matches!(trial, LiveGrantTrial::DeniedPolicy) {
            synthetic("event/live-later-denial", false);
            assert!(fixture.backend.record_live_sandbox_policy(facts).is_err());
        }
        id
    }

    #[test]
    fn live_grant_consumption_survives_only_the_committed_reservation() {
        exercise_live_grant_reservation(LiveGrantTrial::AmbiguousCommit);
    }

    #[test]
    fn live_grant_concurrent_reservation_returns_one_dispatch_right() {
        exercise_live_grant_reservation(LiveGrantTrial::Contended);
    }

    #[test]
    fn live_comment_recovers_lost_acknowledgement_without_resending() {
        exercise_live_grant_reservation(LiveGrantTrial::LostAcknowledgement);
    }

    #[test]
    fn live_grant_revocation_blocks_new_reservation_without_fallback() {
        exercise_live_grant_reservation(LiveGrantTrial::Revoked);
    }

    #[test]
    fn live_grant_principal_revocation_survives_reopen() {
        exercise_live_grant_reservation(LiveGrantTrial::PrincipalRevoked);
    }

    #[test]
    fn live_grant_rejects_persisted_policy_denial() {
        exercise_live_grant_reservation(LiveGrantTrial::DeniedPolicy);
    }

    #[test]
    fn live_grant_rejects_generic_adapter_policy_allow() {
        exercise_live_grant_reservation(LiveGrantTrial::GenericPolicy);
    }

    #[test]
    fn live_grant_target_revocation_survives_reopen() {
        exercise_live_grant_reservation(LiveGrantTrial::TargetRevoked);
    }

    #[test]
    fn live_grant_rejects_persisted_check_failure() {
        exercise_live_grant_reservation(LiveGrantTrial::FailedCheck);
    }

    #[test]
    fn live_grant_rejects_missing_immutable_bundle() {
        exercise_live_grant_reservation(LiveGrantTrial::MissingBundle);
    }

    #[test]
    fn live_grant_rejects_legacy_bundle_without_authoritative_declarations() {
        exercise_live_grant_reservation(LiveGrantTrial::LegacyBundle);
    }

    #[test]
    fn live_grant_rejects_required_assurance_with_only_a_passing_event() {
        exercise_live_grant_reservation(LiveGrantTrial::RequiredCheck);
    }

    #[cfg(all(feature = "trusted-host-application-spi", unix))]
    #[test]
    #[allow(clippy::too_many_lines)]
    fn supervised_native_proof_and_check_result_commit_or_roll_back_together() {
        use crate::{
            admit_trusted_host_local_application_operation, ExplicitLocalCheckProfileSelection,
            ImmutableRunBundleId, ImmutableRunBundleSensitivity, ImmutableRunBundleVersion,
            LocalExecutionBeforeSkillInvocationCheckpointInputs,
            LocalExecutionImmutableRunBundleInputs, LocalExecutionRequest,
            LocalExecutionWithImmutableRunBundleRequest,
            TrustedHostLocalApplicationAdmissionOutcome,
            TrustedHostLocalApplicationAdmissionRequest,
        };
        use std::fs;
        use std::os::unix::fs::PermissionsExt;
        struct ProjectRoot(PathBuf);
        impl Drop for ProjectRoot {
            fn drop(&mut self) {
                let _ = fs::remove_dir_all(&self.0);
            }
        }
        #[derive(Debug)]
        struct UnattestedRunner;
        impl crate::LocalCheckProcessRunner for UnattestedRunner {
            fn run(
                &self,
                _: &crate::LocalCheckProcessRequest,
            ) -> Result<crate::LocalCheckProcessOutput, WorkflowOsError> {
                Ok(crate::LocalCheckProcessOutput::completed(
                    Some(0),
                    true,
                    1,
                    Vec::new(),
                    Vec::new(),
                ))
            }
        }
        for rollback in [false, true] {
            let fixture = Fixture::new();
            let root = ProjectRoot(fixture.path.with_extension("project"));
            let source = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../dogfood/workflow-os-self-governance");
            for directory in ["workflows", "skills", "policies", "scripts", "bin"] {
                fs::create_dir_all(root.0.join(directory)).expect("directory");
            }
            for path in [
                "workflow-os.yml",
                "skills/check-docs.skill.yml",
                "policies/self-governance-local.policy.yml",
            ] {
                fs::copy(source.join(path), root.0.join(path)).expect("copy fixture definition");
            }
            let workflow =
                fs::read_to_string(source.join("workflows/trusted-host-docs-check.workflow.yml"))
                    .expect("workflow");
            let workflow = workflow.replace(
                "    terminal_behavior:",
                "
    local_check_requirements:
      - id: docs-required
        command_id: local-check/docs
        requirement_level: required
        minimum_assurance: kernel_observed_local_process
        accepted_statuses: [passed]
        freshness: {mode: max_age_seconds, seconds: 60}
        exact_immutable_run_binding_required: true
        truncation_allowed: false
        network_maximum: disabled
        side_effect_maximum: no_source_writes
    terminal_behavior:",
            );
            fs::write(root.0.join("workflows/check.workflow.yml"), workflow).expect("workflow");
            fs::write(root.0.join("package.json"), "{}").expect("package");
            fs::write(root.0.join("scripts/check-docs.mjs"), "// fixture").expect("script");
            let executable = root.0.join("bin/npm");
            fs::write(
                &executable,
                "#!/bin/sh\nprintf 'supervised native fixture'\n",
            )
            .expect("executable");
            fs::set_permissions(&executable, fs::Permissions::from_mode(0o700))
                .expect("permissions");
            let profile = ExplicitLocalCheckProfileSelection::docs_check()
                .resolve_docs_check(executable.clone(), root.0.clone(), None)
                .expect("native profile");
            fixture
                .backend
                .enable_live_sandbox_grant_store()
                .expect("opt-in store");
            let run_id = WorkflowRunId::new("run-supervised-native-proof").expect("run");
            let observed_at = Timestamp::now_utc();
            let request = TrustedHostLocalApplicationAdmissionRequest {
                execution: LocalExecutionWithImmutableRunBundleRequest {
                    execution: LocalExecutionRequest {
                        project_root: root.0.clone(),
                        workflow_id: WorkflowId::new("dg/trusted-host-docs-check")
                            .expect("workflow"),
                        run_id: Some(run_id.clone()),
                        correlation_id: CorrelationId::new("correlation/native-proof")
                            .expect("correlation"),
                        actor: ActorId::new("user/native-proof-test").expect("actor"),
                        before_skill_invocation_checkpoints:
                            LocalExecutionBeforeSkillInvocationCheckpointInputs::default(),
                        before_skill_invocation_hook: None,
                        side_effect_events: Vec::new(),
                        side_effect_lifecycle_events: Vec::new(),
                    },
                    bundle: LocalExecutionImmutableRunBundleInputs {
                        bundle_id: ImmutableRunBundleId::new("bundle/native-proof")
                            .expect("bundle"),
                        bundle_version: ImmutableRunBundleVersion::new("v1").expect("version"),
                        created_at: observed_at,
                        sensitivity: ImmutableRunBundleSensitivity::Internal,
                        redaction_required: true,
                    },
                },
                selected_step_id: StepId::new("docs-check").expect("step"),
                observed_at,
                expires_at: Timestamp::from_offset_date_time(
                    observed_at.as_offset_date_time() + time::Duration::minutes(2),
                ),
            };
            if !rollback {
                let injected = ExplicitLocalCheckProfileSelection::docs_check()
                    .resolve_docs_check_with_process_runner(
                        executable,
                        root.0.clone(),
                        None,
                        Arc::new(UnattestedRunner),
                    )
                    .expect("injected profile");
                let mut rejected = request.clone();
                let rejected_id = WorkflowRunId::new("run-injected-native-proof").expect("run");
                rejected.execution.execution.run_id = Some(rejected_id.clone());
                rejected.execution.bundle.bundle_id =
                    ImmutableRunBundleId::new("bundle/injected-native-proof").expect("bundle");
                assert!(admit_trusted_host_local_application_operation(
                    &fixture.backend,
                    &injected,
                    &rejected
                )
                .is_err());
                assert!(fixture
                    .backend
                    .read_events(&rejected_id)
                    .expect("rejected history")
                    .iter()
                    .all(|event| !matches!(
                        event.kind,
                        WorkflowRunEventKind::AuthorizedExecutionAttemptDispatchAdmitted(_)
                    )));
            }
            let TrustedHostLocalApplicationAdmissionOutcome::Prepared(prepared) =
                admit_trusted_host_local_application_operation(
                    &fixture.backend,
                    &profile,
                    &request,
                )
                .expect("native admission")
            else {
                panic!("native check must be prepared");
            };
            super::super::continuity_store::FAIL_AFTER_NATIVE_PROOF_WRITE
                .with(|flag| flag.set(rollback));
            let result = prepared.into_parts().0.run();
            assert_eq!(result.is_err(), rollback);
            assert!(
                !super::super::continuity_store::FAIL_AFTER_NATIVE_PROOF_WRITE
                    .with(std::cell::Cell::get)
            );

            let reopened = SqliteStateBackend::open(&fixture.path).expect("reopen");
            let events = reopened.read_events(&run_id).expect("events");
            assert_eq!(
                events
                    .iter()
                    .filter(|event| matches!(
                        event.kind,
                        WorkflowRunEventKind::AuthorizedExecutionAttemptDispatchAdmitted(_)
                    ))
                    .count(),
                1
            );
            let check_event = events.iter().find(|event| {
                matches!(
                    event.kind,
                    WorkflowRunEventKind::LocalCheckResultRecorded(_)
                )
            });
            assert_eq!(check_event.is_none(), rollback);
            assert_eq!(
                events
                    .iter()
                    .any(|event| matches!(event.kind, WorkflowRunEventKind::RunCompleted)),
                !rollback
            );
            let connection = reopened.connection().expect("connection");
            let stored_event: Option<String> = connection
                .query_row(
                    "SELECT event_id FROM live_sandbox_check_proofs WHERE run_id=?1",
                    [run_id.as_str()],
                    |row| row.get(0),
                )
                .optional()
                .expect("proof row");
            assert_eq!(
                stored_event.as_deref(),
                check_event.map(|event| event.event_id.as_str())
            );
        }
    }

    #[test]
    fn live_grant_rechecks_exact_durable_check_proof_at_reservation() {
        exercise_live_grant_reservation(LiveGrantTrial::RequiredProof);
    }

    #[test]
    fn live_grant_rejects_stale_and_no_reuse_proofs() {
        exercise_live_grant_reservation(LiveGrantTrial::StaleProof);
        exercise_live_grant_reservation(LiveGrantTrial::NoReuseProof);
    }

    #[test]
    fn live_grant_store_upgrades_existing_v11_without_rewriting_history() {
        use super::super::{
            LIVE_AUTHORITY_ADAPTER_SCHEMA_VERSION, LIVE_AUTHORITY_PREVIOUS_SCHEMA_CHECKSUM,
            LIVE_AUTHORITY_PREVIOUS_SCHEMA_VERSION, LIVE_SANDBOX_AUTHORITY_SCHEMA_V11,
        };
        let fixture = Fixture::new();
        let original = fixture
            .backend
            .read_events(&fixture.run_id)
            .expect("history");
        let connection = fixture.backend.connection().expect("test connection");
        connection
            .execute_batch(LIVE_SANDBOX_AUTHORITY_SCHEMA_V11)
            .expect("V11 schema");
        connection
            .execute(
                "UPDATE schema_metadata SET schema_version=?1, checksum=?2 WHERE singleton=1",
                params![
                    LIVE_AUTHORITY_PREVIOUS_SCHEMA_VERSION,
                    LIVE_AUTHORITY_PREVIOUS_SCHEMA_CHECKSUM
                ],
            )
            .expect("V11 metadata");
        connection
            .pragma_update(None, "user_version", LIVE_AUTHORITY_PREVIOUS_SCHEMA_VERSION)
            .expect("V11 version");
        let backend = SqliteStateBackend::open(&fixture.path).expect("read V11");
        backend
            .enable_live_sandbox_grant_store()
            .expect("upgrade to V13");
        backend
            .enable_live_sandbox_grant_store()
            .expect("repeat V13");
        let version: u32 = connection
            .pragma_query_value(None, "user_version", |row| row.get(0))
            .expect("version");
        assert_eq!(version, LIVE_AUTHORITY_ADAPTER_SCHEMA_VERSION);
        assert_eq!(
            backend.read_events(&fixture.run_id).expect("same history"),
            original
        );
    }

    #[test]
    fn live_grant_store_requires_explicit_scoped_policy_protocol_upgrade() {
        use super::super::{
            validate_operational_schema, LIVE_AUTHORITY_ADAPTER_SCHEMA_VERSION,
            LIVE_AUTHORITY_PROOF_SCHEMA_VERSION, LIVE_AUTHORITY_UNSCOPED_V12_CHECKSUM,
        };
        let fixture = Fixture::new();
        fixture
            .backend
            .enable_live_sandbox_grant_store()
            .expect("opt in");
        let connection = fixture.backend.connection().expect("connection");
        connection
            .execute("DROP TABLE live_sandbox_targets", [])
            .expect("previous manifest");
        connection
            .pragma_update(None, "user_version", LIVE_AUTHORITY_PROOF_SCHEMA_VERSION)
            .expect("previous version");
        connection
            .execute(
                "UPDATE schema_metadata SET checksum=?1, schema_version=?2 WHERE singleton=1",
                params![
                    LIVE_AUTHORITY_UNSCOPED_V12_CHECKSUM,
                    LIVE_AUTHORITY_PROOF_SCHEMA_VERSION
                ],
            )
            .expect("previous protocol");
        let reopened = SqliteStateBackend::open(&fixture.path).expect("recognize previous V12");
        assert!(validate_operational_schema(&connection).is_err());
        let original = reopened.read_events(&fixture.run_id).expect("history");
        reopened
            .enable_live_sandbox_grant_store()
            .expect("explicit policy protocol upgrade");
        assert_eq!(
            validate_operational_schema(&connection).expect("active protocol"),
            LIVE_AUTHORITY_ADAPTER_SCHEMA_VERSION
        );
        assert_eq!(
            reopened
                .read_events(&fixture.run_id)
                .expect("unchanged history"),
            original
        );
    }

    #[derive(Clone, Copy)]
    enum LiveGrantTrial {
        AmbiguousCommit,
        Contended,
        LostAcknowledgement,
        Revoked,
        PrincipalRevoked,
        TargetRevoked,
        DeniedPolicy,
        GenericPolicy,
        FailedCheck,
        MissingBundle,
        LegacyBundle,
        RequiredCheck,
        RequiredProof,
        StaleProof,
        NoReuseProof,
    }

    #[allow(clippy::too_many_lines)]
    fn exercise_live_comment_recovery(
        backend: &SqliteStateBackend,
        path: &std::path::Path,
        request: impl Fn() -> crate::dispatch_reservation::DispatchReservationRequest,
        admission: super::super::live_sandbox_grant_store::LiveGrantAdmission<'_>,
        action: crate::provider_write::GitHubCommentContinuationAction<'_>,
    ) {
        use super::super::dispatch_reservation_store::{
            LiveCommentDispatchPosture, LiveCommentLookup, LiveCommentRecovery,
        };
        use crate::{
            GitHubPullRequestCommentProviderLookupObservation as Comment,
            GitHubPullRequestCommentProviderLookupObservationDefinition as CommentDefinition,
            SideEffectRecordStore,
        };

        let operation = request().operation_id;
        assert!(backend
            .recover_live_sandbox_comment(&operation, action, |_, _| {
                panic!("no lookup before durable reservation")
            })
            .is_err());
        let mut accepted = None;
        let dispatch = backend
            .dispatch_live_sandbox_comment(request(), admission, action, |target, body, marker| {
                assert_eq!(target, action.target);
                assert_eq!(body, action.body);
                accepted = Some(
                    Comment::new(CommentDefinition {
                        target: target.clone(),
                        provider_comment_reference: Some("github/comment/17".to_owned()),
                        managed_marker: Some(marker.to_owned()),
                    })
                    .expect("provider accepted"),
                );
                false
            })
            .expect("dispatch");
        assert_eq!(dispatch, LiveCommentDispatchPosture::RecoveryOnly);
        let accepted = accepted.expect("synthetic effect");
        let reopened = SqliteStateBackend::open(path).expect("restart");
        assert_eq!(
            reopened
                .dispatch_live_sandbox_comment(request(), admission, action, |_, _, _| {
                    panic!("replay cannot resend")
                })
                .expect("dispatch replay"),
            LiveCommentDispatchPosture::RecoveryOnly
        );

        let drift = crate::provider_write::GitHubCommentContinuationAction {
            body: "substitution",
            ..action
        };
        assert!(reopened
            .recover_live_sandbox_comment(&operation, drift, |_, _| {
                panic!("changed action cannot invoke lookup")
            })
            .is_err());
        let wrong_marker = Comment::new(CommentDefinition {
            target: action.target.clone(),
            provider_comment_reference: Some("github/comment/17".to_owned()),
            managed_marker: Some("workflow-os/live-comment/wrong".to_owned()),
        })
        .expect("different marker");
        for (response, expected) in [
            (LiveCommentLookup::Absent, LiveCommentRecovery::Absent),
            (LiveCommentLookup::Ambiguous, LiveCommentRecovery::Ambiguous),
            (
                LiveCommentLookup::Unauthorized,
                LiveCommentRecovery::Unauthorized,
            ),
            (
                LiveCommentLookup::Unavailable,
                LiveCommentRecovery::Unavailable,
            ),
            (
                LiveCommentLookup::Observed {
                    comment: accepted.clone(),
                    verified_action: SpecContentHash::from_text("wrong observation"),
                },
                LiveCommentRecovery::Mismatch,
            ),
            (
                LiveCommentLookup::Observed {
                    comment: wrong_marker,
                    verified_action: action.commitment().expect("observed binding"),
                },
                LiveCommentRecovery::Mismatch,
            ),
        ] {
            assert_eq!(
                reopened
                    .recover_live_sandbox_comment(&operation, action, |_, _| response)
                    .expect("lookup posture"),
                expected
            );
        }
        for _ in 0..2 {
            let result = reopened
                .recover_live_sandbox_comment(&operation, action, |target, marker| {
                    assert_eq!(target, accepted.target());
                    assert_eq!(Some(marker), accepted.managed_marker());
                    LiveCommentLookup::Observed {
                        comment: accepted.clone(),
                        verified_action: action.commitment().expect("observed binding"),
                    }
                })
                .expect("recover accepted effect");
            assert_eq!(result, LiveCommentRecovery::Observed(accepted.clone()));
        }
        let lookup = |comment| LiveCommentLookup::Observed {
            comment,
            verified_action: action.commitment().expect("observed binding"),
        };
        let finalize = |store: &SqliteStateBackend| {
            store
                .reconcile_live_sandbox_comment(&operation, action, |_, _| lookup(accepted.clone()))
        };
        let read_effect = || {
            reopened
                .read_side_effect_record(action.side_effect)
                .expect("read effect")
                .expect("effect")
        };
        let attempted = read_effect();
        let inspect = || {
            reopened.inspect_synthetic_sandbox_recovery(
                action.run,
                operation.as_str(),
                action.side_effect,
            )
        };
        let history_before_inspection = reopened.read_events(action.run).expect("history");
        assert_eq!(
            serde_json::to_string(&inspect().expect("attempt inspection")).expect("bounded JSON"),
            "\"reconciliation_required\""
        );
        assert_eq!(
            reopened.read_events(action.run).expect("unchanged history"),
            history_before_inspection
        );
        assert_eq!(read_effect(), attempted);
        #[cfg(unix)]
        let trial_export = SyntheticRecoveryTrialExport::start(action.run);
        #[cfg(unix)]
        if let Some(export) = &trial_export {
            export.snapshot(&reopened, "unresolved");
        }
        assert!(reopened
            .inspect_synthetic_sandbox_recovery(
                action.run,
                operation.as_str(),
                &crate::SideEffectId::new("effect/other").expect("locator"),
            )
            .is_err());
        let db = reopened.connection().expect("test connection");
        let write_test_effect = |effect: &serde_json::Value| {
            db.execute(
                "UPDATE side_effect_records SET payload=?1 WHERE side_effect_id=?2",
                rusqlite::params![effect.to_string(), action.side_effect.as_str()],
            )
            .expect("change fixture effect");
        };
        let original = serde_json::to_value(&attempted).expect("effect JSON");
        let mut drifted = original.clone();
        drifted["summary"] = serde_json::json!("post-reservation drift");
        write_test_effect(&drifted);
        assert!(finalize(&reopened).is_err());
        assert!(inspect().is_err());
        write_test_effect(&original);

        db.execute_batch(
            "CREATE TRIGGER reject_comment_proof BEFORE INSERT ON events
          WHEN NEW.event_id LIKE 'live-comment-proof/%'
          BEGIN SELECT RAISE(ABORT, 'test proof rollback'); END;",
        )
        .expect("inject proof write failure");
        assert!(finalize(&reopened).is_err());
        assert_eq!(read_effect(), attempted);
        db.execute_batch("DROP TRIGGER reject_comment_proof")
            .expect("clear fault");
        assert!(reopened
            .reconcile_live_sandbox_comment(&operation, action, |_, _| {
                LiveCommentLookup::Ambiguous
            })
            .expect("ambiguous does not finalize")
            .is_none());
        let last = reopened
            .read_events(action.run)
            .expect("history")
            .pop()
            .expect("admission");
        reopened
            .append_event(&WorkflowRunEvent {
                sequence_number: EventSequenceNumber::new(last.sequence_number.get() + 1)
                    .expect("sequence"),
                event_id: EventId::new("event/live-comment-run-completed").expect("terminal event"),
                timestamp: Timestamp::now_utc(),
                correlation_id: None,
                actor: None,
                idempotency_key: None,
                kind: WorkflowRunEventKind::RunCompleted,
                ..last
            })
            .expect("complete run before recording provider outcome");
        let proof = finalize(&reopened)
            .expect("persist exact lookup")
            .expect("proof");
        let completed = read_effect();
        assert_eq!(
            completed.lifecycle_state(),
            crate::SideEffectLifecycleState::Completed
        );
        assert_eq!(
            completed
                .outcome_reference()
                .map(crate::SideEffectOutcomeReference::reference),
            accepted.provider_comment_reference()
        );
        let restarted = SqliteStateBackend::open(path).expect("reopen proof");
        assert_eq!(finalize(&restarted).expect("replay proof"), Some(proof));
        assert_eq!(
            restarted
                .rehydrate_run(action.run)
                .expect("terminal run")
                .snapshot
                .status,
            crate::WorkflowRunStatus::Completed
        );
        let conflicting = Comment::new(CommentDefinition {
            target: accepted.target().clone(),
            managed_marker: accepted.managed_marker().map(str::to_owned),
            provider_comment_reference: Some("github/comment/18".to_owned()),
        })
        .expect("conflicting observation");
        assert!(restarted
            .reconcile_live_sandbox_comment(&operation, action, |_, _| { lookup(conflicting) })
            .is_err());
        assert_eq!(read_effect(), completed);
        #[cfg(unix)]
        if let Some(export) = &trial_export {
            export.snapshot(&restarted, "completed");
            export.write_manifest(action, operation.as_str());
        }
        let completed_history = restarted.read_events(action.run).expect("history");
        assert_eq!(
            serde_json::to_string(&inspect().expect("proof inspection")).expect("bounded JSON"),
            "\"completion_verified\""
        );
        assert_eq!(
            restarted
                .read_events(action.run)
                .expect("unchanged history"),
            completed_history
        );
        let original_completed = serde_json::to_value(&completed).expect("completed JSON");
        let mut drifted = original_completed.clone();
        drifted["summary"] = serde_json::json!("drift after completion");
        write_test_effect(&drifted);
        assert!(inspect().is_err());
        write_test_effect(&original_completed);
    }

    #[cfg(unix)]
    struct SyntheticRecoveryTrialExport {
        root: PathBuf,
        kernel: PathBuf,
    }

    #[cfg(unix)]
    impl SyntheticRecoveryTrialExport {
        fn start(run_id: &WorkflowRunId) -> Option<Self> {
            use std::os::unix::fs::DirBuilderExt;

            let kernel = PathBuf::from(std::env::var_os("WORKFLOW_OS_SYNTHETIC_RECOVERY_KERNEL")?);
            assert!(
                kernel.is_absolute() && kernel.is_file(),
                "reviewed CLI path required"
            );
            let root = std::env::temp_dir().join(format!(
                "workflow-os-recovery-mcp-{}-{run_id}",
                std::process::id(),
            ));
            std::fs::DirBuilder::new()
                .mode(0o700)
                .create(&root)
                .expect("private export directory");
            Some(Self { root, kernel })
        }

        fn snapshot(&self, backend: &SqliteStateBackend, case: &str) {
            use std::os::unix::fs::{DirBuilderExt, PermissionsExt};

            let state = self.root.join(case);
            std::fs::DirBuilder::new()
                .mode(0o700)
                .create(&state)
                .expect("private state directory");
            let database = state.join("github-pr-comment-continuation.sqlite3");
            backend
                .connection()
                .expect("synthetic connection")
                .execute(
                    "VACUUM INTO ?1",
                    [database.to_str().expect("database path")],
                )
                .expect("snapshot synthetic state");
            std::fs::set_permissions(database, std::fs::Permissions::from_mode(0o600))
                .expect("private database");
        }

        fn write_manifest(
            &self,
            action: crate::provider_write::GitHubCommentContinuationAction<'_>,
            operation: &str,
        ) {
            use std::io::Write;
            use std::os::unix::fs::OpenOptionsExt;

            let cases = ["unresolved", "completed"].map(|id| {
                serde_json::json!({
                    "id": id, "state": self.root.join(id), "run_id": action.run,
                    "operation_id": operation, "side_effect_id": action.side_effect,
                })
            });
            let manifest = serde_json::json!({
                "mode": "synthetic-recovery", "kernel": self.kernel, "cases": cases,
            });
            let path = self.root.join("manifest.json");
            std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(0o600)
                .open(&path)
                .expect("private manifest")
                .write_all(manifest.to_string().as_bytes())
                .expect("write manifest");
            println!("synthetic_recovery_mcp_manifest: {}", path.display());
        }
    }

    impl LiveGrantTrial {
        fn denies_issuance(self) -> bool {
            matches!(
                self,
                Self::DeniedPolicy
                    | Self::GenericPolicy
                    | Self::FailedCheck
                    | Self::MissingBundle
                    | Self::LegacyBundle
                    | Self::RequiredCheck
                    | Self::StaleProof
                    | Self::NoReuseProof
            )
        }

        fn requires_check(self) -> bool {
            matches!(
                self,
                Self::RequiredCheck | Self::RequiredProof | Self::StaleProof | Self::NoReuseProof
            )
        }
    }

    fn build_live_test_bundle(
        run_id: &WorkflowRunId,
        trial: LiveGrantTrial,
    ) -> crate::ImmutableRunBundleBuildResult {
        use crate::{
            build_immutable_run_bundle, build_immutable_run_bundle_with_local_check_declarations,
            ImmutableRunBundleBuildRequest, ImmutableRunBundleExecutionPosture,
            ImmutableRunBundleHandlerPosture, ImmutableRunBundleHandlerReference,
            ImmutableRunBundleId, ImmutableRunBundleReferencePosture,
            ImmutableRunBundleSensitivity, ImmutableRunBundleVersion, LocalCheckCommandContract,
            LocalCheckCommandContractInventory,
        };
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../dogfood/workflow-os-self-governance");
        let mut project = crate::load_project(root).bundle.expect("dogfood project");
        let workflow_id = WorkflowId::new("dg/trusted-host-docs-check").expect("workflow");
        if trial.requires_check() {
            let workflow = project
                .workflows
                .iter_mut()
                .find(|workflow| workflow.definition.id == workflow_id)
                .expect("workflow");
            let freshness = if matches!(trial, LiveGrantTrial::NoReuseProof) {
                "mode: no_reuse"
            } else {
                "mode: max_age_seconds, seconds: 600"
            };
            workflow.definition.steps[0].local_check_requirements = serde_yaml::from_str(&format!(
                "- id: docs-required
  command_id: local-check/docs
  requirement_level: required
  minimum_assurance: kernel_observed_local_process
  accepted_statuses: [passed]
  freshness: {{{freshness}}}
  exact_immutable_run_binding_required: true
  truncation_allowed: false
  network_maximum: disabled
  side_effect_maximum: no_source_writes
"
            ))
            .expect("required check");
            workflow.definition.spec_content_hash = None;
            let source = serde_yaml::to_string(&workflow.definition).expect("workflow");
            workflow.definition = crate::parse_workflow_spec_yaml(&source).expect("parse workflow");
            workflow.content_hash =
                crate::canonical_yaml_content_hash(&source).expect("source hash");
        }
        let request = ImmutableRunBundleBuildRequest {
            project: &project,
            workflow_id: &workflow_id,
            bundle_id: ImmutableRunBundleId::new(format!("bundle/{run_id}")).expect("bundle"),
            bundle_version: ImmutableRunBundleVersion::new("v1").expect("version"),
            run_id: run_id.clone(),
            resolved_execution_context_hash: SpecContentHash::from_text("live context"),
            execution_posture: ImmutableRunBundleExecutionPosture::new(
                Vec::new(),
                ImmutableRunBundleReferencePosture::NotSupplied,
                ImmutableRunBundleReferencePosture::NotSupplied,
                ImmutableRunBundleReferencePosture::CommittedReference,
            )
            .expect("posture"),
            handlers: vec![ImmutableRunBundleHandlerReference {
                skill_id: SkillId::new("local/check-docs").expect("skill"),
                skill_version: SkillVersion::new("v0").expect("version"),
                posture: ImmutableRunBundleHandlerPosture::RegisteredUnattested,
            }],
            created_at: Timestamp::parse_rfc3339("2026-09-30T09:00:00Z").expect("created"),
            created_by: ActorId::new("system/kernel").expect("actor"),
            sensitivity: ImmutableRunBundleSensitivity::Internal,
            redaction_required: true,
        };
        if matches!(trial, LiveGrantTrial::LegacyBundle) {
            build_immutable_run_bundle(request).expect("legacy bundle")
        } else {
            let inventory = LocalCheckCommandContractInventory::new(vec![
                LocalCheckCommandContract::docs_check_model_only().expect("docs command"),
            ])
            .expect("inventory");
            build_immutable_run_bundle_with_local_check_declarations(request, &inventory)
                .expect("canonical live test bundle")
        }
    }

    fn seed_live_check_proof(
        backend: &SqliteStateBackend,
        fixture: &Fixture,
        trial: LiveGrantTrial,
    ) {
        use crate::LocalCheckAttestationFreshnessPolicy as Freshness;
        let events = backend.read_events(&fixture.run_id).expect("history");
        let (event, check) = events
            .iter()
            .find_map(|event| match &event.kind {
                WorkflowRunEventKind::LocalCheckResultRecorded(check) => {
                    Some((event, check.as_ref()))
                }
                _ => None,
            })
            .expect("recorded result");
        let connection = backend.connection().expect("test connection");
        let stored = SqliteStateBackend::read_exact_bundle_with_connection(
            &connection,
            &fixture.run_id,
            fixture.bundle.bundle_id(),
        )
        .expect("stored bundle");
        let freshness = if matches!(trial, LiveGrantTrial::NoReuseProof) {
            Freshness::NoReuse
        } else {
            Freshness::max_age_seconds(600).expect("max age")
        };
        let evaluated = if matches!(trial, LiveGrantTrial::StaleProof) {
            Timestamp::from_offset_date_time(
                event.timestamp.as_offset_date_time() - time::Duration::seconds(600),
            )
        } else {
            event.timestamp
        };
        let projection = |attested| {
            crate::local_check_attestation::verified_docs_check_projection_for_test(
                freshness,
                stored.clone(),
                check,
                evaluated,
                attested,
            )
        };
        assert!(backend.store_live_check_proof(&projection(false)).is_err());
        let accepted = projection(true);
        backend
            .store_live_check_proof(&accepted)
            .expect("store exact verifier proof");
        backend
            .store_live_check_proof(&accepted)
            .expect("exact storage retry");
    }

    #[allow(clippy::too_many_lines)]
    fn exercise_live_grant_reservation(trial: LiveGrantTrial) {
        use super::super::live_sandbox_grant_store::{live_opening_operation, LiveGrantAdmission};
        use crate::dispatch_reservation::{request_commitment, DispatchReservationStore};
        use crate::live_sandbox_continuation_authority::{
            tests::{with_live_grant_test_facts, LiveGrantTestContext},
            LiveSandboxPrincipalVerifier, LiveSandboxProviderBinding, LIVE_CAPABILITY,
        };
        use crate::local_fixture_replacement_authority::LocalFixturePrincipalVerifier;
        use crate::provider_write::GitHubCommentContinuationAction;
        use crate::{
            CapabilityGrantLifecycle, GitHubPullRequestCommentTarget,
            GitHubPullRequestCommentWriteMode, SideEffectId, SideEffectRecordStore,
        };

        let fixture = Fixture::new_with_live_bundle(Some(trial));
        let actor = ActorId::new("agent/opening-test").expect("actor");
        let action = GitHubCommentContinuationAction {
            run: &fixture.run_id,
            step: &fixture.step_id,
            bundle_root: fixture.bundle.root_hash(),
            original_actor: &actor,
            side_effect: &SideEffectId::new("effect/comment").expect("effect"),
            target: &GitHubPullRequestCommentTarget::new("example", "sandbox", 7).expect("target"),
            body: "synthetic approved comment",
            idempotency_key: &IdempotencyKey::new("comment/live-reservation").expect("key"),
            mode: GitHubPullRequestCommentWriteMode::LiveSandbox,
        };
        let invocation = action.commitment().expect("live action");
        let authorization = OperationalExecutionWindowOpeningAuthorization::new(
            SpecContentHash::from_text("current authority"),
            SpecContentHash::from_text("core governance"),
            operation_binding_commitment(
                &fixture.workflow_id,
                &fixture.run_id,
                &fixture.step_id,
                &invocation,
            ),
        );
        let operation = live_opening_operation(&invocation);
        let opening_identity = fixture.request(&authorization, &operation);
        let now = Timestamp::now_utc();
        let expiry = Timestamp::from_offset_date_time(
            now.as_offset_date_time() + time::Duration::minutes(4),
        );
        with_live_grant_test_facts(
            LiveGrantTestContext {
                action,
                workflow: &fixture.workflow_id,
                window: &opening_identity.window_id,
                attempt: &opening_identity.attempt_id,
                now,
                expiry,
                window_expiry: expiry,
            },
            |facts, grant_for| {
                let cursor = persist_live_grant_test_approval(&fixture, facts, trial);
                let observed = super::super::continuity_store::observe_continuity_trusted_time()
                    .expect("trusted time");
                let opening = |cursor: &ContinuityCursor, observed: &crate::authorized_execution_continuity_state::internal::TrustedTimeObservation| {
                    let mut request = fixture.request(&authorization, &operation);
                    request.expected_cursor = cursor.clone();
                    request.invocation_binding_commitment = invocation.clone();
                    request.expires_at = expiry;
                    request.trusted_time = observed.clone();
                    request.request_commitment = opening_request_commitment(&request);
                    request
                };
                assert!(fixture
                    .backend
                    .open_live_sandbox_attempt(opening(&cursor, &observed), facts)
                    .is_err());
                assert!(fixture
                    .backend
                    .open_window_and_start_attempt_projected(opening(&cursor, &observed))
                    .is_err());
                fixture
                    .backend
                    .enable_live_sandbox_grant_store()
                    .expect("explicit upgrade");
                let connection = fixture.backend.connection().expect("test connection");
                assert_eq!(
                    super::super::validate_schema_metadata_version(
                        &connection,
                        super::super::LIVE_AUTHORITY_ADAPTER_SCHEMA_VERSION,
                        "sha256:1dddc6985e2bb5cc014db4c86900a7591fbef63ef89ca10e759f85795f2eff9a",
                        "d73239544f08c79a64ce001bfea57a2adc8212809499dcf561f1ed2c03df0d51",
                    )
                    .expect_err("older writer must not accept the current live opening protocol")
                    .code(),
                    "state.sqlite.schema.recovery_required",
                );
                assert!(fixture
                    .backend
                    .open_window_and_start_attempt_projected(opening(&cursor, &observed))
                    .is_err());
                assert!(fixture
                    .backend
                    .open_live_sandbox_attempt(opening(&cursor, &observed), facts)
                    .is_err());
                assert!(fixture.backend.record_live_sandbox_policy(facts).is_err());
                let provider_binding = || LiveSandboxProviderBinding {
                    repository_id: std::num::NonZeroU64::new(123).expect("provider repository id"),
                    author_id: std::num::NonZeroU64::new(456).expect("provider author id"),
                    adapter_commitment: SpecContentHash::from_text("trusted synthetic adapter"),
                };
                let provision =
                    crate::live_sandbox_continuation_authority::LiveSandboxPolicyTarget::provision(
                        action.target.clone(),
                        facts.decision.actor.clone(),
                        SpecContentHash::from_text("synthetic target revision"),
                        expiry,
                        provider_binding(),
                    )
                    .expect("synthetic target");
                fixture
                    .backend
                    .provision_live_sandbox_target(&provision)
                    .expect("register target");
                assert!(fixture
                    .backend
                    .provision_live_sandbox_target(&provision)
                    .is_err());
                let policy_event = seed_durable_live_policy(&fixture, facts, trial, &provision);
                let last = fixture
                    .backend
                    .read_events(&fixture.run_id)
                    .expect("history")
                    .pop()
                    .expect("last event");
                let cursor = ContinuityCursor {
                    sequence_number: last.sequence_number,
                    event_id: last.event_id,
                };
                let observed = super::super::continuity_store::observe_continuity_trusted_time()
                    .expect("post-policy time");
                let substituted = LiveSandboxContinuationFacts {
                    action: GitHubCommentContinuationAction {
                        body: "different body",
                        ..action
                    },
                    ..facts
                };
                assert!(fixture
                    .backend
                    .open_live_sandbox_attempt(opening(&cursor, &observed), substituted)
                    .is_err());
                let replay_request = opening(&cursor, &observed);
                let opened = fixture
                    .backend
                    .open_live_sandbox_attempt(opening(&cursor, &observed), facts)
                    .expect("open live-only");
                let OperationalExecutionWindowOpeningResult::Opened { capability, .. } =
                    opened.result
                else {
                    panic!("fresh opening");
                };
                let grant = grant_for(LIVE_CAPABILITY, CapabilityGrantLifecycle::Active);
                let live_credential = [43; 32];
                let fixture_credential = [17; 32];
                let backend = SqliteStateBackend::open(&fixture.path).expect("reopen V13");
                assert!(matches!(
                    backend
                        .open_live_sandbox_attempt(replay_request, facts)
                        .expect("opening replay")
                        .result,
                    OperationalExecutionWindowOpeningResult::ExactReplay(_)
                ));
                let before_grant = checked_reservation_request(&capability);
                assert_eq!(
                    backend
                        .reserve_attempt_dispatch(before_grant)
                        .err()
                        .expect("denied")
                        .code(),
                    "live_sandbox_continuation.storage_denied",
                );
                assert!(backend
                    .reserve_live_sandbox_dispatch(
                        checked_reservation_request(&capability),
                        LiveGrantAdmission {
                            grant_id: grant.grant_id(),
                            side_effect: action.side_effect,
                            body: action.body,
                            credential: &live_credential,
                        }
                    )
                    .is_err());
                let reader =
                    SqliteStateBackend::open_read_only(&fixture.path).expect("inspect V13");
                assert!(reader.health_check().expect("V13 health").healthy);
                backend
                    .enable_live_sandbox_grant_store()
                    .expect("exact repeat upgrade");
                let fixture_only = grant_for(
                    "fixture/github-pr-comment-continuation",
                    CapabilityGrantLifecycle::Active,
                );
                let harness = facts.scope.harness_contract_id().expect("harness").clone();
                backend
                    .register_local_fixture_principal(&LocalFixturePrincipalVerifier::new(
                        facts.replacement.clone(),
                        harness.clone(),
                        &fixture_credential,
                    ))
                    .expect("register separate fixture principal");
                assert!(backend.issue_live_sandbox_grant(facts, &grant).is_err());
                let verifier = LiveSandboxPrincipalVerifier::new(
                    facts.replacement.clone(),
                    harness,
                    &live_credential,
                );
                backend
                    .register_live_sandbox_principal(&verifier)
                    .expect("register live principal");
                assert!(backend.register_live_sandbox_principal(&verifier).is_err());
                assert!(backend
                    .issue_live_sandbox_grant(facts, &fixture_only)
                    .is_err());
                assert!(backend.issue_live_sandbox_grant(facts, &grant).is_err());
                let effect =
                    live_test_effect(&fixture, facts, &capability.cursor.event_id, &policy_event);
                backend
                    .write_side_effect_record(&effect)
                    .expect("persist attempted effect");
                if matches!(
                    trial,
                    LiveGrantTrial::RequiredProof
                        | LiveGrantTrial::StaleProof
                        | LiveGrantTrial::NoReuseProof
                ) {
                    seed_live_check_proof(&backend, &fixture, trial);
                }
                if trial.denies_issuance() {
                    let error = backend
                        .issue_live_sandbox_grant(facts, &grant)
                        .expect_err("issuance denied");
                    let expected = if trial.requires_check() {
                        "live_sandbox_continuation.required_check_unverified"
                    } else {
                        "live_sandbox_continuation.storage_denied"
                    };
                    assert_eq!(error.code(), expected);
                    return;
                }
                let binding = backend
                    .issue_live_sandbox_grant(facts, &grant)
                    .expect("durable grant");
                let backend =
                    SqliteStateBackend::open(&fixture.path).expect("reopen durable grant");
                assert!(backend.issue_live_sandbox_grant(facts, &grant).is_err());
                let request = || {
                    let mut request = checked_reservation_request(&capability);
                    request.invocation_commitment = invocation.clone();
                    request.executor_commitment = binding.clone();
                    request.request_commitment = request_commitment(&request);
                    request
                };
                let connection = backend.connection().expect("independent connection");
                let event_count = || backend.read_events(&fixture.run_id).expect("events").len();
                let before = event_count();
                let counts = || {
                    connection
                        .query_row(
                            "SELECT (SELECT COUNT(*) FROM live_sandbox_grant_uses),
                        (SELECT COUNT(*) FROM dispatch_reservations)",
                            [],
                            |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?)),
                        )
                        .expect("count")
                };
                let admission = LiveGrantAdmission {
                    grant_id: grant.grant_id(),
                    side_effect: action.side_effect,
                    body: action.body,
                    credential: &live_credential,
                };
                let absent_effect = SideEffectId::new("effect/wrong").expect("effect locator");
                for wrong in [
                    LiveGrantAdmission {
                        body: "different body",
                        ..admission
                    },
                    LiveGrantAdmission {
                        side_effect: &absent_effect,
                        ..admission
                    },
                ] {
                    assert!(backend
                        .reserve_live_sandbox_dispatch(request(), wrong)
                        .is_err());
                    assert_eq!(counts(), (0, 0));
                }
                let revised_policy =
                    crate::live_sandbox_continuation_authority::LiveSandboxPolicyTarget::provision(
                        facts.action.target.clone(),
                        facts.decision.actor.clone(),
                        SpecContentHash::from_text("changed policy revision"),
                        facts.window_expires_at,
                        provider_binding(),
                    )
                    .expect("revised policy");
                let original_target = serde_json::to_string(&provision).expect("target payload");
                let set_target = |payload: &str| {
                    connection
                        .execute("UPDATE live_sandbox_targets SET payload=?1", [payload])
                        .expect("test target substitution")
                };
                set_target(
                    &serde_json::to_string(&revised_policy).expect("changed target payload"),
                );
                assert!(backend
                    .reserve_live_sandbox_dispatch(request(), admission)
                    .is_err());
                set_target(&original_target);
                let registered: serde_json::Value =
                    serde_json::from_str(&original_target).expect("registered target");
                let mut legacy = registered.clone();
                legacy
                    .as_object_mut()
                    .expect("target object")
                    .remove("provider");
                set_target(&legacy.to_string());
                assert!(backend
                    .reserve_live_sandbox_dispatch(request(), admission)
                    .is_err());
                for (field, replacement) in [
                    ("repository_id", serde_json::json!(124)),
                    ("author_id", serde_json::json!(457)),
                    (
                        "adapter_commitment",
                        serde_json::json!(SpecContentHash::from_text("substituted adapter")),
                    ),
                ] {
                    let mut drifted = registered.clone();
                    drifted["provider"][field] = replacement;
                    set_target(&drifted.to_string());
                    assert!(backend
                        .reserve_live_sandbox_dispatch(request(), admission)
                        .is_err());
                    assert_eq!(counts(), (0, 0));
                }
                set_target(&original_target);
                assert_eq!(counts(), (0, 0));
                if matches!(trial, LiveGrantTrial::TargetRevoked) {
                    backend
                        .revoke_live_sandbox_target(facts.action.target)
                        .expect("revoke target");
                    let reopened =
                        SqliteStateBackend::open(&fixture.path).expect("reopen revocation");
                    assert!(reopened.provision_live_sandbox_target(&provision).is_err());
                    assert!(reopened
                        .reserve_live_sandbox_dispatch(request(), admission)
                        .is_err());
                    assert_eq!(counts(), (0, 0));
                    return;
                }
                if matches!(trial, LiveGrantTrial::RequiredProof) {
                    let payload: String = connection
                        .query_row(
                            "SELECT payload FROM live_sandbox_check_proofs WHERE run_id=?1",
                            [fixture.run_id.as_str()],
                            |row| row.get(0),
                        )
                        .expect("read exact proof payload");
                    let set_proof =
                        |value: &str| {
                            connection.execute(
                        "UPDATE live_sandbox_check_proofs SET payload=?1 WHERE run_id=?2",
                        params![value, fixture.run_id.as_str()],
                    ).expect("inject proof drift in disposable fixture")
                        };
                    let mut tampered: serde_json::Value =
                        serde_json::from_str(&payload).expect("proof JSON");
                    tampered["verifier_proof"]["invocation_id"] = "invocation/substituted".into();
                    set_proof(&tampered.to_string());
                    let Err(error) = backend.reserve_live_sandbox_dispatch(request(), admission)
                    else {
                        panic!("tampered check proof must deny reservation");
                    };
                    assert_eq!(
                        error.code(),
                        "live_sandbox_continuation.required_check_unverified"
                    );
                    assert_eq!(counts(), (0, 0));
                    set_proof(&payload);
                }
                let unauthenticated = LiveGrantAdmission {
                    credential: &fixture_credential,
                    ..admission
                };
                assert!(backend
                    .reserve_live_sandbox_dispatch(request(), unauthenticated)
                    .is_err());
                assert!(backend.reserve_attempt_dispatch(request()).is_err());
                let set_bundle_root = |root: &SpecContentHash| {
                    connection
                        .execute(
                            "UPDATE immutable_run_bundles SET root_hash=?1 WHERE run_id=?2",
                            params![root.as_str(), fixture.run_id.as_str()],
                        )
                        .expect("inject bundle drift in disposable test");
                };
                set_bundle_root(&SpecContentHash::from_text("substituted stored bundle"));
                assert!(backend
                    .reserve_live_sandbox_dispatch(request(), admission)
                    .is_err());
                set_bundle_root(fixture.bundle.root_hash());
                let mut altered = serde_json::to_value(&effect).expect("effect JSON");
                altered["summary"] = serde_json::json!("substituted attempted effect");
                let substitute_effect = |payload: &str| {
                    connection
                        .execute(
                            "UPDATE side_effect_records SET payload=?1 WHERE side_effect_id=?2",
                            params![payload, effect.side_effect_id().as_str()],
                        )
                        .expect("inject effect drift in disposable test");
                };
                substitute_effect(&altered.to_string());
                assert!(backend
                    .reserve_live_sandbox_dispatch(request(), admission)
                    .is_err());
                substitute_effect(
                    &serde_json::to_string(&effect).expect("restore synthetic effect"),
                );
                assert_eq!(counts(), (0, 0));
                assert_eq!(event_count(), before);
                inject_dispatch_commit_fault(InjectedDispatchCommitFault::Before);
                assert!(backend
                    .reserve_live_sandbox_dispatch(request(), admission)
                    .is_err());
                assert_eq!(counts(), (0, 0));
                assert_eq!(event_count(), before);
                match trial {
                    LiveGrantTrial::TargetRevoked => {
                        unreachable!("revocation handled before reservation")
                    }
                    LiveGrantTrial::DeniedPolicy
                    | LiveGrantTrial::GenericPolicy
                    | LiveGrantTrial::FailedCheck
                    | LiveGrantTrial::MissingBundle
                    | LiveGrantTrial::LegacyBundle
                    | LiveGrantTrial::RequiredCheck
                    | LiveGrantTrial::StaleProof
                    | LiveGrantTrial::NoReuseProof => {
                        unreachable!("denied at issuance")
                    }
                    LiveGrantTrial::AmbiguousCommit => {
                        use super::super::dispatch_reservation_store::LiveCommentDispatchPosture;
                        inject_dispatch_commit_fault(InjectedDispatchCommitFault::After);
                        assert_eq!(
                            backend
                                .dispatch_live_sandbox_comment(
                                    request(),
                                    admission,
                                    action,
                                    |_, _, _| { panic!("uncertain reservation cannot send") }
                                )
                                .expect("uncertain"),
                            LiveCommentDispatchPosture::RecoveryOnly
                        );
                    }
                    LiveGrantTrial::LostAcknowledgement => {
                        exercise_live_comment_recovery(
                            &backend,
                            &fixture.path,
                            request,
                            admission,
                            action,
                        );
                    }
                    LiveGrantTrial::Contended | LiveGrantTrial::RequiredProof => {
                        let winners = std::thread::scope(|scope| {
                            let invoke = || {
                                backend
                                    .reserve_live_sandbox_dispatch(request(), admission)
                                    .expect("serialized reservation")
                                    .is_some()
                            };
                            let first = scope.spawn(invoke);
                            let second = scope.spawn(invoke);
                            u8::from(first.join().expect("first caller"))
                                + u8::from(second.join().expect("second caller"))
                        });
                        assert_eq!(winners, 1);
                    }
                    LiveGrantTrial::Revoked => {
                        backend
                            .revoke_live_sandbox_grant(grant.grant_id())
                            .expect("revoke");
                        let restarted =
                            SqliteStateBackend::open(&fixture.path).expect("reopen revoked grant");
                        assert!(restarted
                            .reserve_live_sandbox_dispatch(request(), admission)
                            .is_err());
                        assert!(restarted.reserve_attempt_dispatch(request()).is_err());
                        assert!(restarted.issue_live_sandbox_grant(facts, &grant).is_err());
                        assert_eq!(counts(), (0, 0));
                        assert_eq!(event_count(), before);
                        return;
                    }
                    LiveGrantTrial::PrincipalRevoked => {
                        backend
                            .revoke_live_sandbox_principal(facts.replacement)
                            .expect("revoke principal");
                        let restarted = SqliteStateBackend::open(&fixture.path)
                            .expect("reopen revoked principal");
                        assert!(restarted
                            .reserve_live_sandbox_dispatch(request(), admission)
                            .is_err());
                        assert!(restarted
                            .register_live_sandbox_principal(&verifier)
                            .is_err());
                        assert_eq!(counts(), (0, 0));
                        assert_eq!(event_count(), before);
                        return;
                    }
                }
                assert_eq!(counts(), (1, 1));
                assert_eq!(
                    event_count(),
                    before
                        + 1
                        + 2 * usize::from(matches!(trial, LiveGrantTrial::LostAcknowledgement))
                );
                assert!(backend
                    .reserve_live_sandbox_dispatch(request(), admission)
                    .expect("replay")
                    .is_none());
                assert!(backend
                    .reserve_live_sandbox_dispatch(request(), unauthenticated)
                    .is_err());
                assert!(backend.reserve_attempt_dispatch(request()).is_err());
                backend
                    .revoke_live_sandbox_principal(facts.replacement)
                    .expect("revoke principal after reservation");
                assert!(backend
                    .reserve_live_sandbox_dispatch(request(), admission)
                    .is_err());
                backend
                    .revoke_live_sandbox_grant(grant.grant_id())
                    .expect("revoke without removing use");
                assert!(backend.revoke_live_sandbox_grant(grant.grant_id()).is_err());
                assert_eq!(counts(), (1, 1));
            },
        );
    }

    #[test]
    fn dispatch_admission_check_and_reservation_share_commit_and_rollback() {
        use crate::dispatch_reservation::DispatchReservationOutcome;

        let fixture = Fixture::new();
        let opened = open_supervisor_attempt(&fixture, "opening/checked-reservation", 1);
        let connection = fixture.backend.connection().expect("test connection");
        connection
            .execute_batch("CREATE TABLE test_grant_use (id INTEGER PRIMARY KEY)")
            .expect("disposable transaction probe");
        let original_events = fixture
            .backend
            .read_events(&fixture.run_id)
            .expect("events");
        let request = || checked_reservation_request(&opened);
        let count_uses = || {
            connection
                .query_row("SELECT COUNT(*) FROM test_grant_use", [], |row| {
                    row.get::<_, i64>(0)
                })
                .expect("count uses from separate connection")
        };
        let record_use = |tx: &rusqlite::Transaction<'_>, now, expires| {
            assert!(now < expires);
            tx.execute("INSERT INTO test_grant_use VALUES (1)", [])
                .expect("test grant-use write");
            Ok(())
        };
        let denied = fixture.backend.reserve_attempt_dispatch_checked(
            request(),
            None,
            |tx, now, expires| {
                record_use(tx, now, expires)?;
                Err(WorkflowOsError::security(
                    "test.admission_denied",
                    "test authority denied",
                ))
            },
        );
        assert_eq!(
            denied.err().expect("denied").code(),
            "test.admission_denied"
        );
        assert_eq!(count_uses(), 0);
        assert_eq!(
            fixture
                .backend
                .read_events(&fixture.run_id)
                .expect("events"),
            original_events
        );

        inject_dispatch_commit_fault(InjectedDispatchCommitFault::Before);
        let aborted = fixture
            .backend
            .reserve_attempt_dispatch_checked(request(), None, record_use);
        assert_eq!(
            aborted.err().expect("aborted").code(),
            "dispatch_reservation.write_failed"
        );
        assert_eq!(count_uses(), 0);
        assert_eq!(
            fixture
                .backend
                .read_events(&fixture.run_id)
                .expect("events"),
            original_events
        );

        inject_dispatch_commit_fault(InjectedDispatchCommitFault::After);
        let uncertain =
            fixture
                .backend
                .reserve_attempt_dispatch_checked(request(), None, record_use);
        assert!(matches!(
            uncertain.expect("recover committed reservation"),
            DispatchReservationOutcome::CommittedButCapabilityUnavailable { .. }
        ));
        assert_eq!(count_uses(), 1);
        let committed = fixture
            .backend
            .read_events(&fixture.run_id)
            .expect("committed events");
        assert_eq!(committed.len(), original_events.len() + 1);

        let replay =
            fixture
                .backend
                .reserve_attempt_dispatch_checked(request(), None, |_, _, _| {
                    Err(WorkflowOsError::security(
                        "test.unexpected_check",
                        "replay attempted a new admission",
                    ))
                });
        assert!(matches!(
            replay.expect("recovery-only replay"),
            DispatchReservationOutcome::AlreadyAdmitted { .. }
        ));
        assert_eq!(count_uses(), 1);
        assert_eq!(
            fixture
                .backend
                .read_events(&fixture.run_id)
                .expect("events"),
            committed
        );
    }

    #[test]
    fn concurrent_checked_reservation_admits_one_attempt_and_checks_once() {
        use crate::dispatch_reservation::DispatchReservationOutcome;

        let fixture = Fixture::new();
        let opened = open_supervisor_attempt(&fixture, "opening/concurrent-checked-reservation", 1);
        let barrier = Barrier::new(2);
        let checks = AtomicUsize::new(0);
        let reserve = || {
            let input = checked_reservation_request(&opened);
            barrier.wait();
            fixture
                .backend
                .reserve_attempt_dispatch_checked(input, None, |_, _, _| {
                    checks.fetch_add(1, Ordering::Relaxed);
                    Ok(())
                })
                .expect("checked reservation or replay")
        };
        let (first, second) = std::thread::scope(|scope| {
            let competitor = scope.spawn(reserve);
            (reserve(), competitor.join().expect("competing reservation"))
        });
        assert!(matches!(
            (first, second),
            (
                DispatchReservationOutcome::Admitted { .. },
                DispatchReservationOutcome::AlreadyAdmitted { .. }
            ) | (
                DispatchReservationOutcome::AlreadyAdmitted { .. },
                DispatchReservationOutcome::Admitted { .. }
            )
        ));
        assert_eq!(checks.load(Ordering::Relaxed), 1);
        assert_eq!(
            fixture
                .backend
                .read_events(&fixture.run_id)
                .expect("events")
                .len(),
            5
        );
    }

    #[test]
    fn snapshot_projection_tampering_blocks_downstream_persistence_and_replay() {
        let fixture = Fixture::new();
        let capability = open_supervisor_attempt(&fixture, "opening/snapshot-tamper", 1);
        let replay = clone_opened_capability(&capability);
        let calls = AtomicUsize::new(0);
        let error = supervise_one_local_skill_attempt(TrustedHostSupervisorInput {
            backend: &fixture.backend,
            capability: TrustedHostSupervisorAttemptCapability::Opened(capability),
            executor: &SnapshotTamperingExecutor {
                calls: &calls,
                database_path: &fixture.path,
            },
            skill_input: skill_input(&fixture),
            persistence: TrustedHostSupervisorPersistenceInput {
                operation: ContinuityOperationId::new("operation/snapshot-tamper")
                    .expect("operation"),
                receipt: ContinuityReceiptId::new("receipt/snapshot-tamper").expect("receipt"),
                yield_generation: None,
            },
        })
        .expect_err("tampered snapshot must block downstream persistence");
        assert_eq!(
            error.code(),
            "trusted_host_supervisor.outcome_not_committed"
        );
        assert_eq!(calls.load(Ordering::Relaxed), 1);

        let replay_error = supervise_one_local_skill_attempt(TrustedHostSupervisorInput {
            backend: &fixture.backend,
            capability: TrustedHostSupervisorAttemptCapability::Opened(replay),
            executor: &CountingExecutor { calls: &calls },
            skill_input: skill_input(&fixture),
            persistence: TrustedHostSupervisorPersistenceInput {
                operation: ContinuityOperationId::new("operation/snapshot-tamper")
                    .expect("operation"),
                receipt: ContinuityReceiptId::new("receipt/snapshot-tamper").expect("receipt"),
                yield_generation: None,
            },
        })
        .expect_err("tampered snapshot must block replay");
        assert_eq!(
            replay_error.code(),
            "dispatch_reservation.recovery_required"
        );
        assert_eq!(calls.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn bounded_redispatch_loop_yields_then_succeeds_without_false_stall() {
        let fixture = Fixture::new();
        let capability = open_supervisor_attempt(&fixture, "opening/bounded-redispatch", 2);
        let calls = AtomicUsize::new(0);
        let executor = YieldThenSucceedExecutor {
            calls: &calls,
            yields_before_success: 1,
        };
        let mut identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };

        let outcome = run_bounded_trusted_host_redispatch_loop(TrustedHostRedispatchLoopInput {
            backend: &fixture.backend,
            initial_capability: TrustedHostSupervisorAttemptCapability::Opened(capability),
            executor: &executor,
            skill_input: skill_input(&fixture),
            initial_persistence: TrustedHostSupervisorPersistenceInput {
                operation: ContinuityOperationId::new("operation/bounded-redispatch-initial")
                    .expect("operation"),
                receipt: ContinuityReceiptId::new("receipt/bounded-redispatch-initial")
                    .expect("receipt"),
                yield_generation: Some(
                    ContinuityYieldGenerationId::new("yield/bounded-redispatch-initial")
                        .expect("generation"),
                ),
            },
            identity_provider: &mut identity_provider,
        })
        .expect("bounded redispatch completes");

        assert_eq!(
            outcome.disposition,
            crate::authorized_execution_continuity_state::internal::AuthoritativeContinuationDisposition::Terminal
        );
        assert_eq!(
            outcome.stop_reason,
            TrustedHostRedispatchStopReason::Terminal
        );
        assert_eq!(outcome.executor_entries, 2);
        assert_eq!(calls.load(Ordering::Relaxed), 2);
        assert_eq!(identity_provider.calls, 1);
        assert_eq!(
            outcome
                .skill_output
                .as_ref()
                .and_then(|output| output.output_ref.as_deref()),
            Some("bounded-redispatch-output")
        );
        let debug = format!("{outcome:?}");
        assert!(!debug.contains("bounded-redispatch-output"));
        assert!(!debug.contains("status"));
        assert_eq!(
            fixture
                .backend
                .rehydrate_run(&fixture.run_id)
                .expect("run")
                .snapshot
                .status,
            WorkflowRunStatus::Running
        );
    }

    #[test]
    fn operational_entry_surfaces_existing_executing_window_as_blocked() {
        let fixture = Fixture::new();
        let capability = open_supervisor_attempt(&fixture, "opening/entry-blocked", 2);
        let locator = TrustedHostOperationalEntryLocator {
            workflow_id: fixture.workflow_id.clone(),
            run_id: fixture.run_id.clone(),
            step_id: fixture.step_id.clone(),
            window_id: capability.window_id.clone(),
            subject_actor_id: capability.subject_actor_id.clone(),
            immutable_run_bundle: fixture.bundle.clone(),
        };
        drop(capability);
        let calls = AtomicUsize::new(0);
        let executor = CountingExecutor { calls: &calls };
        let mut identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };

        let outcome = enter_trusted_host_operation(TrustedHostOperationalEntryInput {
            backend: &fixture.backend,
            locator,
            posture: TrustedHostOperationalEntryPosture::Existing,
            executor: &executor,
            skill_input: skill_input(&fixture),
            identity_provider: &mut identity_provider,
            expected_preparation_commitment: None,
        })
        .expect("executing window is a bounded block");

        assert_eq!(
            outcome.disposition,
            crate::authorized_execution_continuity_state::internal::AuthoritativeContinuationDisposition::Blocked
        );
        assert_eq!(
            outcome.stop_reason,
            TrustedHostRedispatchStopReason::Blocked
        );
        assert_eq!(outcome.executor_entries, 0);
        assert_eq!(calls.load(Ordering::Relaxed), 0);
        assert_eq!(identity_provider.calls, 0);
    }

    #[test]
    fn operational_entry_requires_existing_window_for_existing_posture() {
        let fixture = Fixture::new();
        let calls = AtomicUsize::new(0);
        let executor = CountingExecutor { calls: &calls };
        let mut identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };

        let error = enter_trusted_host_operation(TrustedHostOperationalEntryInput {
            backend: &fixture.backend,
            locator: TrustedHostOperationalEntryLocator {
                workflow_id: fixture.workflow_id.clone(),
                run_id: fixture.run_id.clone(),
                step_id: fixture.step_id.clone(),
                window_id: AuthorizedExecutionWindowId::new("window/missing-opening-context")
                    .expect("window"),
                subject_actor_id: ActorId::new("agent/opening-test").expect("actor"),
                immutable_run_bundle: fixture.bundle.clone(),
            },
            posture: TrustedHostOperationalEntryPosture::Existing,
            executor: &executor,
            skill_input: skill_input(&fixture),
            identity_provider: &mut identity_provider,
            expected_preparation_commitment: None,
        })
        .expect_err("fresh scope without opening context must fail closed");

        assert_eq!(
            error.code(),
            "trusted_host_operational_entry.existing_window_unavailable"
        );
        assert_eq!(calls.load(Ordering::Relaxed), 0);
        assert_eq!(identity_provider.calls, 0);
    }

    #[test]
    fn operational_entry_surfaces_terminal_window_without_executor_entry() {
        let fixture = Fixture::new();
        let capability = open_supervisor_attempt(&fixture, "opening/entry-terminal", 1);
        let locator = TrustedHostOperationalEntryLocator {
            workflow_id: fixture.workflow_id.clone(),
            run_id: fixture.run_id.clone(),
            step_id: fixture.step_id.clone(),
            window_id: capability.window_id.clone(),
            subject_actor_id: capability.subject_actor_id.clone(),
            immutable_run_bundle: fixture.bundle.clone(),
        };
        let calls = AtomicUsize::new(0);
        let executor = CountingExecutor { calls: &calls };
        supervise_one_local_skill_attempt(TrustedHostSupervisorInput {
            backend: &fixture.backend,
            capability: TrustedHostSupervisorAttemptCapability::Opened(capability),
            executor: &executor,
            skill_input: skill_input(&fixture),
            persistence: TrustedHostSupervisorPersistenceInput {
                operation: ContinuityOperationId::new("operation/entry-terminal")
                    .expect("operation"),
                receipt: ContinuityReceiptId::new("receipt/entry-terminal").expect("receipt"),
                yield_generation: None,
            },
        })
        .expect("initial attempt reaches terminal window posture");
        let mut identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };

        let outcome = enter_trusted_host_operation(TrustedHostOperationalEntryInput {
            backend: &fixture.backend,
            locator,
            posture: TrustedHostOperationalEntryPosture::Existing,
            executor: &executor,
            skill_input: skill_input(&fixture),
            identity_provider: &mut identity_provider,
            expected_preparation_commitment: None,
        })
        .expect("terminal window is a bounded terminal outcome");

        assert_eq!(
            outcome.stop_reason,
            TrustedHostRedispatchStopReason::Terminal
        );
        assert_eq!(outcome.executor_entries, 0);
        assert_eq!(calls.load(Ordering::Relaxed), 1);
        assert_eq!(identity_provider.calls, 0);
    }

    #[test]
    fn operational_entry_rehydrates_resume_and_invokes_once() {
        let fixture = Fixture::new();
        let capability = open_supervisor_attempt(&fixture, "opening/entry-resume", 2);
        let locator = TrustedHostOperationalEntryLocator {
            workflow_id: fixture.workflow_id.clone(),
            run_id: fixture.run_id.clone(),
            step_id: fixture.step_id.clone(),
            window_id: capability.window_id.clone(),
            subject_actor_id: capability.subject_actor_id.clone(),
            immutable_run_bundle: fixture.bundle.clone(),
        };
        supervise_one_local_skill_attempt(TrustedHostSupervisorInput {
            backend: &fixture.backend,
            capability: TrustedHostSupervisorAttemptCapability::Opened(capability),
            executor: &YieldExecutor,
            skill_input: skill_input(&fixture),
            persistence: TrustedHostSupervisorPersistenceInput {
                operation: ContinuityOperationId::new("operation/entry-yield").expect("operation"),
                receipt: ContinuityReceiptId::new("receipt/entry-yield").expect("receipt"),
                yield_generation: Some(
                    ContinuityYieldGenerationId::new("yield/entry-yield").expect("generation"),
                ),
            },
        })
        .expect("initial attempt yields");
        let calls = AtomicUsize::new(0);
        let executor = CountingExecutor { calls: &calls };
        let mut identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };

        let outcome = enter_trusted_host_operation(TrustedHostOperationalEntryInput {
            backend: &fixture.backend,
            locator,
            posture: TrustedHostOperationalEntryPosture::Existing,
            executor: &executor,
            skill_input: skill_input(&fixture),
            identity_provider: &mut identity_provider,
            expected_preparation_commitment: None,
        })
        .expect("fresh entry resumes current authority");

        assert_eq!(
            outcome.stop_reason,
            TrustedHostRedispatchStopReason::Terminal
        );
        assert_eq!(outcome.executor_entries, 1);
        assert_eq!(calls.load(Ordering::Relaxed), 1);
        assert_eq!(identity_provider.calls, 1);
        assert_eq!(
            fixture
                .backend
                .rehydrate_run(&fixture.run_id)
                .expect("run")
                .snapshot
                .status,
            WorkflowRunStatus::Running
        );
    }

    #[test]
    fn schedule_once_elapsed_wake_reinvokes_exactly_once() {
        let fixture = Fixture::new();
        let deadline = Timestamp::from_offset_date_time(
            Timestamp::now_utc().as_offset_date_time() + time::Duration::milliseconds(500),
        );
        let locator =
            register_supervisor_time_window_wait(&fixture, "schedule-once-elapsed", deadline);
        std::thread::sleep(std::time::Duration::from_millis(600));
        let window_id = locator.window_id.clone();
        let waiter_calls = AtomicUsize::new(0);
        let executor_calls = AtomicUsize::new(0);
        let mut waiter = RecordingDeadlineWaiter {
            calls: &waiter_calls,
            expected_deadline: deadline,
            outcome: Ok(TrustedHostDeadlineWaitOutcome::Woke),
        };
        let executor = CountingExecutor {
            calls: &executor_calls,
        };
        let mut identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };

        let outcome = schedule_trusted_host_time_window_once(TrustedHostScheduleOnceInput {
            backend: &fixture.backend,
            locator,
            operation_id: ContinuityOperationId::new("operation/schedule-once-elapsed-wake")
                .expect("operation"),
            receipt_id: ContinuityReceiptId::new("receipt/schedule-once-elapsed-wake")
                .expect("receipt"),
            deadline_waiter: &mut waiter,
            executor: &executor,
            skill_input: skill_input(&fixture),
            identity_provider: &mut identity_provider,
        })
        .expect("elapsed deadline is reinvoked");

        let TrustedHostScheduleOnceOutcome::Reinvoked(outcome) = outcome else {
            panic!("elapsed wake must reinvoke");
        };
        assert_eq!(waiter_calls.load(Ordering::Relaxed), 1);
        assert_eq!(executor_calls.load(Ordering::Relaxed), 1);
        assert_eq!(identity_provider.calls, 1);
        assert_eq!(outcome.execution.executor_entries, 1);
        assert_eq!(
            outcome.execution.stop_reason,
            TrustedHostRedispatchStopReason::Terminal
        );
        assert_eq!(
            fixture
                .backend
                .continuation_disposition(&window_id)
                .expect("durable disposition"),
            AuthoritativeContinuationDisposition::Terminal
        );
    }

    #[test]
    fn repeated_schedule_budget_is_small_positive_and_redaction_safe() {
        assert_eq!(
            TrustedHostRepeatedWakeBudget::new(0)
                .expect_err("zero budget")
                .code(),
            "trusted_host_repeated_scheduling.wake_budget_invalid"
        );
        assert_eq!(
            TrustedHostRepeatedWakeBudget::new(9)
                .expect_err("oversized budget")
                .code(),
            "trusted_host_repeated_scheduling.wake_budget_invalid"
        );
        let budget = TrustedHostRepeatedWakeBudget::new(8).expect("maximum budget");
        assert_eq!(format!("{budget:?}"), "TrustedHostRepeatedWakeBudget(8)");
    }

    #[test]
    fn repeated_schedule_composes_two_time_windows_and_stops_terminal() {
        let fixture = Fixture::new();
        let deadline = Timestamp::from_offset_date_time(
            // Leave enough time for the opening and projected-yield transactions to
            // complete under a loaded full-suite runner. The production boundary
            // still rejects a wait whose deadline has elapsed before persistence.
            Timestamp::now_utc().as_offset_date_time() + time::Duration::seconds(5),
        );
        let locator = register_supervisor_time_window_wait_with_attempts(
            &fixture,
            "repeated-two-waits",
            deadline,
            3,
        );
        let remaining = deadline.as_offset_date_time() - Timestamp::now_utc().as_offset_date_time();
        if remaining.is_positive() {
            std::thread::sleep(
                std::time::Duration::try_from(remaining).expect("positive deadline duration"),
            );
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
        let waiter_calls = AtomicUsize::new(0);
        let executor_calls = AtomicUsize::new(0);
        let mut waiter = SequenceDeadlineWaiter {
            calls: &waiter_calls,
            expected_deadlines: vec![Some(deadline), None],
            delays: vec![None, Some(std::time::Duration::from_millis(300))],
            next: 0,
            outcome: TrustedHostDeadlineWaitOutcome::Woke,
        };
        let executor = YieldTimeWindowThenFailExecutor {
            calls: &executor_calls,
        };
        let mut redispatch_identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };
        let mut wake_identity_provider = DeterministicWakeIdentityProvider { calls: 0 };

        let outcome =
            run_bounded_trusted_host_repeated_scheduling(TrustedHostRepeatedSchedulingInput {
                backend: &fixture.backend,
                locator,
                wake_budget: TrustedHostRepeatedWakeBudget::new(2).expect("budget"),
                deadline_waiter: &mut waiter,
                executor: &executor,
                skill_input: skill_input(&fixture),
                redispatch_identity_provider: &mut redispatch_identity_provider,
                wake_identity_provider: &mut wake_identity_provider,
            })
            .expect("bounded repeated scheduling");

        assert_eq!(
            outcome.disposition,
            AuthoritativeContinuationDisposition::Terminal
        );
        assert_eq!(
            outcome.stop_reason,
            TrustedHostRepeatedSchedulingStopReason::Terminal
        );
        assert_eq!(outcome.scheduled_wakes, 2);
        assert_eq!(outcome.executor_entries, 2);
        assert_eq!(waiter_calls.load(Ordering::Relaxed), 2);
        assert_eq!(executor_calls.load(Ordering::Relaxed), 2);
        assert_eq!(wake_identity_provider.calls, 2);
    }

    #[test]
    fn local_timer_wrapper_waits_and_enters_executor_once() {
        let fixture = Fixture::new();
        let deadline = Timestamp::from_offset_date_time(
            Timestamp::now_utc().as_offset_date_time() + time::Duration::milliseconds(500),
        );
        let locator = register_supervisor_time_window_wait(&fixture, "local-timer", deadline);
        let executor_calls = AtomicUsize::new(0);
        let executor = CountingExecutor {
            calls: &executor_calls,
        };
        let mut redispatch_identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };
        let mut wake_identity_provider = DeterministicWakeIdentityProvider { calls: 0 };
        let (cancellation, _handle) = TrustedHostLocalTimerCancellation::new();

        let outcome = run_trusted_host_local_timer(TrustedHostLocalTimerInput {
            backend: &fixture.backend,
            locator,
            wake_budget: TrustedHostRepeatedWakeBudget::new(1).expect("budget"),
            cancellation,
            executor: &executor,
            skill_input: skill_input(&fixture),
            redispatch_identity_provider: &mut redispatch_identity_provider,
            wake_identity_provider: &mut wake_identity_provider,
        })
        .expect("local timer run");

        assert_eq!(
            outcome.stop_reason,
            TrustedHostRepeatedSchedulingStopReason::Terminal
        );
        assert_eq!(outcome.scheduled_wakes, 1);
        assert_eq!(outcome.executor_entries, 1);
        assert_eq!(executor_calls.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn local_production_caller_uses_fixed_boundary_and_enters_executor_once() {
        let fixture = Fixture::new();
        let deadline = Timestamp::from_offset_date_time(
            Timestamp::now_utc().as_offset_date_time() + time::Duration::milliseconds(500),
        );
        let locator =
            register_supervisor_time_window_wait(&fixture, "local-production-caller", deadline);
        let executor_calls = AtomicUsize::new(0);
        let executor = CountingExecutor {
            calls: &executor_calls,
        };
        let (cancellation, _handle) = TrustedHostLocalTimerCancellation::new();

        let outcome =
            run_trusted_host_local_production_caller(TrustedHostLocalProductionCallerInput {
                backend: &fixture.backend,
                locator,
                cancellation,
                executor: &executor,
                skill_input: skill_input(&fixture),
            })
            .expect("local production caller");

        assert_eq!(
            outcome.stop_reason,
            TrustedHostRepeatedSchedulingStopReason::Terminal
        );
        assert_eq!(outcome.scheduled_wakes, 1);
        assert_eq!(outcome.executor_entries, 1);
        assert_eq!(executor_calls.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn local_production_caller_cancellation_is_owner_reachable_and_zero_write() {
        let fixture = Fixture::new();
        let deadline = Timestamp::from_offset_date_time(
            Timestamp::now_utc().as_offset_date_time() + time::Duration::hours(1),
        );
        let locator = register_supervisor_time_window_wait(
            &fixture,
            "local-production-caller-cancel",
            deadline,
        );
        let before = crate::sqlite_state::continuity_codec::load_snapshot(
            &fixture.backend.connection().expect("connection"),
        )
        .expect("snapshot before cancellation");
        let executor_calls = AtomicUsize::new(0);
        let executor = CountingExecutor {
            calls: &executor_calls,
        };
        let (cancellation, handle) = TrustedHostLocalTimerCancellation::new();
        handle.cancel().expect("cancel");

        let outcome =
            run_trusted_host_local_production_caller(TrustedHostLocalProductionCallerInput {
                backend: &fixture.backend,
                locator,
                cancellation,
                executor: &executor,
                skill_input: skill_input(&fixture),
            })
            .expect("canceled local production caller");

        assert_eq!(
            outcome.stop_reason,
            TrustedHostRepeatedSchedulingStopReason::Canceled
        );
        assert_eq!(outcome.scheduled_wakes, 1);
        assert_eq!(outcome.executor_entries, 0);
        assert_eq!(executor_calls.load(Ordering::Relaxed), 0);
        let after = crate::sqlite_state::continuity_codec::load_snapshot(
            &fixture.backend.connection().expect("connection"),
        )
        .expect("snapshot after cancellation");
        assert!(
            before == after,
            "production caller cancellation must not mutate continuity state"
        );
    }

    #[test]
    fn local_production_caller_reopens_backend_and_preserves_outcome_shape() {
        let fixture = Fixture::new();
        let deadline = Timestamp::from_offset_date_time(
            Timestamp::now_utc().as_offset_date_time() + time::Duration::milliseconds(500),
        );
        let locator = register_supervisor_time_window_wait(
            &fixture,
            "local-production-caller-restart",
            deadline,
        );
        let reopened = SqliteStateBackend::open(&fixture.path).expect("reopen backend");
        let executor_calls = AtomicUsize::new(0);
        let executor = CountingExecutor {
            calls: &executor_calls,
        };
        let (cancellation, _handle) = TrustedHostLocalTimerCancellation::new();

        let outcome =
            run_trusted_host_local_production_caller(TrustedHostLocalProductionCallerInput {
                backend: &reopened,
                locator,
                cancellation,
                executor: &executor,
                skill_input: skill_input(&fixture),
            })
            .expect("reconstructed local production caller");

        assert_eq!(
            outcome.stop_reason,
            TrustedHostRepeatedSchedulingStopReason::Terminal
        );
        assert_eq!(outcome.scheduled_wakes, 1);
        assert_eq!(outcome.executor_entries, 1);
        assert_eq!(executor_calls.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn local_production_caller_uses_exactly_two_wakes() {
        let fixture = Fixture::new();
        let deadline = Timestamp::from_offset_date_time(
            Timestamp::now_utc().as_offset_date_time() + time::Duration::milliseconds(500),
        );
        let locator = register_supervisor_time_window_wait_with_attempts(
            &fixture,
            "local-production-caller-two-wakes",
            deadline,
            3,
        );
        let executor_calls = AtomicUsize::new(0);
        let executor = YieldTimeWindowThenFailExecutor {
            calls: &executor_calls,
        };
        let (cancellation, _handle) = TrustedHostLocalTimerCancellation::new();

        let outcome =
            run_trusted_host_local_production_caller(TrustedHostLocalProductionCallerInput {
                backend: &fixture.backend,
                locator,
                cancellation,
                executor: &executor,
                skill_input: skill_input(&fixture),
            })
            .expect("two-wake local production caller");

        assert_eq!(
            outcome.stop_reason,
            TrustedHostRepeatedSchedulingStopReason::Terminal
        );
        assert_eq!(outcome.scheduled_wakes, 2);
        assert_eq!(outcome.executor_entries, 2);
        assert_eq!(executor_calls.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn explicit_local_operation_returns_terminal_entry_without_scheduling() {
        let fixture = Fixture::new();
        let capability = open_supervisor_attempt(&fixture, "opening/explicit-terminal", 1);
        let locator = TrustedHostOperationalEntryLocator {
            workflow_id: fixture.workflow_id.clone(),
            run_id: fixture.run_id.clone(),
            step_id: fixture.step_id.clone(),
            window_id: capability.window_id.clone(),
            subject_actor_id: capability.subject_actor_id.clone(),
            immutable_run_bundle: fixture.bundle.clone(),
        };
        let calls = AtomicUsize::new(0);
        let executor = CountingExecutor { calls: &calls };
        supervise_one_local_skill_attempt(TrustedHostSupervisorInput {
            backend: &fixture.backend,
            capability: TrustedHostSupervisorAttemptCapability::Opened(capability),
            executor: &executor,
            skill_input: skill_input(&fixture),
            persistence: TrustedHostSupervisorPersistenceInput {
                operation: ContinuityOperationId::new("operation/explicit-terminal")
                    .expect("operation"),
                receipt: ContinuityReceiptId::new("receipt/explicit-terminal").expect("receipt"),
                yield_generation: None,
            },
        })
        .expect("terminal attempt");
        let mut identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };
        let (cancellation, _handle) = TrustedHostLocalTimerCancellation::new();

        let outcome =
            run_explicit_trusted_host_local_operation(TrustedHostExplicitLocalOperationInput {
                operational_entry: TrustedHostOperationalEntryInput {
                    backend: &fixture.backend,
                    locator,
                    posture: TrustedHostOperationalEntryPosture::Existing,
                    executor: &executor,
                    skill_input: skill_input(&fixture),
                    identity_provider: &mut identity_provider,
                    expected_preparation_commitment: None,
                },
                cancellation,
            })
            .expect("terminal entry is returned");

        let TrustedHostExplicitLocalOperationOutcome::EntryStopped(entry) = outcome else {
            panic!("terminal entry must bypass continuation scheduling");
        };
        assert_eq!(entry.stop_reason, TrustedHostRedispatchStopReason::Terminal);
        assert_eq!(entry.executor_entries, 0);
        assert_eq!(calls.load(Ordering::Relaxed), 1);
        assert_eq!(identity_provider.calls, 0);
    }

    #[test]
    fn explicit_local_operation_routes_time_window_yield_to_production_caller() {
        let fixture = Fixture::new();
        let locator = register_resumable_operational_entry(&fixture, "explicit-time-window", 3);
        let calls = AtomicUsize::new(0);
        let executor = YieldTimeWindowThenFailExecutor { calls: &calls };
        let mut identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };
        let (cancellation, _handle) = TrustedHostLocalTimerCancellation::new();

        let outcome =
            run_explicit_trusted_host_local_operation(TrustedHostExplicitLocalOperationInput {
                operational_entry: TrustedHostOperationalEntryInput {
                    backend: &fixture.backend,
                    locator,
                    posture: TrustedHostOperationalEntryPosture::Existing,
                    executor: &executor,
                    skill_input: skill_input(&fixture),
                    identity_provider: &mut identity_provider,
                    expected_preparation_commitment: None,
                },
                cancellation,
            })
            .expect("time-window continuation");

        let TrustedHostExplicitLocalOperationOutcome::ContinuationStopped(continuation) = outcome
        else {
            panic!("time-window yield must route to the production caller");
        };
        assert_eq!(
            continuation.stop_reason,
            TrustedHostRepeatedSchedulingStopReason::Terminal
        );
        assert_eq!(continuation.scheduled_wakes, 1);
        assert_eq!(continuation.executor_entries, 1);
        assert_eq!(calls.load(Ordering::Relaxed), 2);
        assert_eq!(identity_provider.calls, 1);
    }

    #[test]
    fn explicit_local_operation_preserves_fixed_two_wake_budget() {
        let fixture = Fixture::new();
        let locator = register_resumable_operational_entry(&fixture, "explicit-two-wakes", 4);
        let calls = AtomicUsize::new(0);
        let executor = YieldTwoTimeWindowsThenFailExecutor { calls: &calls };
        let mut identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };
        let (cancellation, _handle) = TrustedHostLocalTimerCancellation::new();

        let outcome =
            run_explicit_trusted_host_local_operation(TrustedHostExplicitLocalOperationInput {
                operational_entry: TrustedHostOperationalEntryInput {
                    backend: &fixture.backend,
                    locator,
                    posture: TrustedHostOperationalEntryPosture::Existing,
                    executor: &executor,
                    skill_input: skill_input(&fixture),
                    identity_provider: &mut identity_provider,
                    expected_preparation_commitment: None,
                },
                cancellation,
            })
            .expect("two-wake continuation");

        let TrustedHostExplicitLocalOperationOutcome::ContinuationStopped(continuation) = outcome
        else {
            panic!("awaiting entry must route to continuation scheduling");
        };
        assert_eq!(
            continuation.stop_reason,
            TrustedHostRepeatedSchedulingStopReason::Terminal
        );
        assert_eq!(continuation.scheduled_wakes, 2);
        assert_eq!(continuation.executor_entries, 2);
        assert_eq!(calls.load(Ordering::Relaxed), 3);
    }

    #[test]
    fn explicit_local_operation_does_not_route_resumable_yield_as_wait() {
        let fixture = Fixture::new();
        let locator =
            register_supervisor_unsupported_wait(&fixture, "explicit-operation-unsupported");
        let calls = AtomicUsize::new(0);
        let executor = CountingExecutor { calls: &calls };
        let mut identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };
        let (cancellation, _handle) = TrustedHostLocalTimerCancellation::new();

        let outcome =
            run_explicit_trusted_host_local_operation(TrustedHostExplicitLocalOperationInput {
                operational_entry: TrustedHostOperationalEntryInput {
                    backend: &fixture.backend,
                    locator,
                    posture: TrustedHostOperationalEntryPosture::Existing,
                    executor: &executor,
                    skill_input: skill_input(&fixture),
                    identity_provider: &mut identity_provider,
                    expected_preparation_commitment: None,
                },
                cancellation,
            })
            .expect("unsupported wait posture");

        let TrustedHostExplicitLocalOperationOutcome::EntryStopped(entry) = outcome else {
            panic!("resumable yield must remain in operational entry");
        };
        assert_eq!(entry.stop_reason, TrustedHostRedispatchStopReason::Terminal);
        assert_eq!(entry.executor_entries, 1);
        assert_eq!(calls.load(Ordering::Relaxed), 1);
        assert_eq!(identity_provider.calls, 1);
    }

    #[test]
    fn explicit_local_operation_owner_cancellation_stops_after_initial_yield() {
        let fixture = Fixture::new();
        let locator = register_resumable_operational_entry(&fixture, "explicit-cancel", 3);
        let calls = AtomicUsize::new(0);
        let executor = TimeWindowYieldExecutor {
            condition_version: 1,
            deadline: Timestamp::from_offset_date_time(
                Timestamp::now_utc().as_offset_date_time() + time::Duration::hours(1),
            ),
        };
        let mut identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };
        let (cancellation, handle) = TrustedHostLocalTimerCancellation::new();
        handle.cancel().expect("cancel");
        let counting_executor = CountingBindingExecutor {
            calls: &calls,
            delegate: &executor,
        };

        let outcome =
            run_explicit_trusted_host_local_operation(TrustedHostExplicitLocalOperationInput {
                operational_entry: TrustedHostOperationalEntryInput {
                    backend: &fixture.backend,
                    locator,
                    posture: TrustedHostOperationalEntryPosture::Existing,
                    executor: &counting_executor,
                    skill_input: skill_input(&fixture),
                    identity_provider: &mut identity_provider,
                    expected_preparation_commitment: None,
                },
                cancellation,
            })
            .expect("owner cancellation");

        let TrustedHostExplicitLocalOperationOutcome::ContinuationStopped(continuation) = outcome
        else {
            panic!("initial time-window yield must reach cancellation-aware caller");
        };
        assert_eq!(
            continuation.stop_reason,
            TrustedHostRepeatedSchedulingStopReason::Canceled
        );
        assert_eq!(continuation.executor_entries, 0);
        assert_eq!(calls.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn explicit_local_operation_rejects_substituted_input_without_leaking_debug() {
        let fixture = Fixture::new();
        let locator = register_resumable_operational_entry(&fixture, "explicit-substitution", 3);
        let calls = AtomicUsize::new(0);
        let executor = CountingExecutor { calls: &calls };
        let mut identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };
        let (cancellation, _handle) = TrustedHostLocalTimerCancellation::new();
        let mut substituted = skill_input(&fixture);
        substituted.values.insert(
            "context".to_owned(),
            "authorization=secret-explicit-operation-marker".to_owned(),
        );
        let input = TrustedHostExplicitLocalOperationInput {
            operational_entry: TrustedHostOperationalEntryInput {
                backend: &fixture.backend,
                locator,
                posture: TrustedHostOperationalEntryPosture::Existing,
                executor: &executor,
                skill_input: substituted,
                identity_provider: &mut identity_provider,
                expected_preparation_commitment: None,
            },
            cancellation,
        };
        let input_debug = format!("{input:?}");
        assert!(!input_debug.contains("secret-explicit-operation-marker"));
        assert!(!input_debug.contains(fixture.run_id.as_str()));

        let error = run_explicit_trusted_host_local_operation(input)
            .expect_err("substituted skill input must fail closed");

        assert_eq!(
            error.code(),
            "trusted_host_operational_entry.invocation_binding_mismatch"
        );
        assert_eq!(calls.load(Ordering::Relaxed), 0);
        assert_eq!(identity_provider.calls, 0);
        let error_debug = format!("{error:?}");
        assert!(!error_debug.contains("secret-explicit-operation-marker"));
        assert!(!error_debug.contains(fixture.run_id.as_str()));
    }

    #[test]
    fn explicit_local_operation_reopened_backend_preserves_binding_and_debug() {
        let fixture = Fixture::new();
        let locator = register_resumable_operational_entry(&fixture, "explicit-reopen", 3);
        let reopened = SqliteStateBackend::open(&fixture.path).expect("reopen backend");
        let calls = AtomicUsize::new(0);
        let executor = YieldTimeWindowThenFailExecutor { calls: &calls };
        let mut identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };
        let (cancellation, _handle) = TrustedHostLocalTimerCancellation::new();

        let outcome =
            run_explicit_trusted_host_local_operation(TrustedHostExplicitLocalOperationInput {
                operational_entry: TrustedHostOperationalEntryInput {
                    backend: &reopened,
                    locator,
                    posture: TrustedHostOperationalEntryPosture::Existing,
                    executor: &executor,
                    skill_input: skill_input(&fixture),
                    identity_provider: &mut identity_provider,
                    expected_preparation_commitment: None,
                },
                cancellation,
            })
            .expect("reopened explicit operation");

        let debug = format!("{outcome:?}");
        assert!(!debug.contains(fixture.run_id.as_str()));
        assert!(!debug.contains("local-supervisor-output"));
        let TrustedHostExplicitLocalOperationOutcome::ContinuationStopped(continuation) = outcome
        else {
            panic!("reopened awaiting operation must continue");
        };
        assert_eq!(
            continuation.stop_reason,
            TrustedHostRepeatedSchedulingStopReason::Terminal
        );
        assert_eq!(calls.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn process_owner_canceled_before_entry_is_zero_write_and_redacted() {
        let fixture = Fixture::new();
        let locator = register_resumable_operational_entry(&fixture, "owner-pre-cancel", 3);
        let before = crate::sqlite_state::continuity_codec::load_snapshot(
            &fixture.backend.connection().expect("connection"),
        )
        .expect("snapshot before owner");
        let events_before = fixture
            .backend
            .read_events(&fixture.run_id)
            .expect("events before owner")
            .len();
        let calls = AtomicUsize::new(0);
        let executor = CountingExecutor { calls: &calls };
        let mut identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };
        let (owner, handle) =
            TrustedHostExplicitLocalProcessOwner::new(TrustedHostOperationalEntryInput {
                backend: &fixture.backend,
                locator,
                posture: TrustedHostOperationalEntryPosture::Existing,
                executor: &executor,
                skill_input: skill_input(&fixture),
                identity_provider: &mut identity_provider,
                expected_preparation_commitment: None,
            });
        let owner_debug = format!("{owner:?}");
        assert!(owner_debug.contains("[REDACTED]"));
        assert!(!owner_debug.contains(fixture.run_id.as_str()));
        handle.cancel().expect("cancel before entry");

        let outcome = owner.run().expect("bounded canceled outcome");

        assert!(matches!(
            outcome,
            TrustedHostExplicitLocalProcessOwnerOutcome::CanceledBeforeEntry
        ));
        assert_eq!(calls.load(Ordering::Relaxed), 0);
        assert_eq!(identity_provider.calls, 0);
        let after = crate::sqlite_state::continuity_codec::load_snapshot(
            &fixture.backend.connection().expect("connection"),
        )
        .expect("snapshot after owner");
        assert!(before == after);
        assert_eq!(
            fixture
                .backend
                .read_events(&fixture.run_id)
                .expect("events after owner")
                .len(),
            events_before
        );
    }

    #[cfg(feature = "trusted-host-application-spi")]
    #[test]
    fn application_spi_wraps_owner_and_preserves_pre_entry_cancellation() {
        let fixture = Fixture::new();
        let locator = register_resumable_operational_entry(&fixture, "application-spi-cancel", 3);
        let before = crate::sqlite_state::continuity_codec::load_snapshot(
            &fixture.backend.connection().expect("connection"),
        )
        .expect("snapshot before session");
        let calls = AtomicUsize::new(0);
        let executor = CountingExecutor { calls: &calls };
        let mut identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };
        let (owner, timer_handle) =
            TrustedHostExplicitLocalProcessOwner::new(TrustedHostOperationalEntryInput {
                backend: &fixture.backend,
                locator,
                posture: TrustedHostOperationalEntryPosture::Existing,
                executor: &executor,
                skill_input: skill_input(&fixture),
                identity_provider: &mut identity_provider,
                expected_preparation_commitment: None,
            });
        let session = TrustedHostLocalApplicationSession::from_process_owner(owner);
        let handle = TrustedHostLocalApplicationCancellationHandle::from_timer_handle(timer_handle);

        handle
            .request_cancellation()
            .expect("cancel before application entry");
        let outcome = session.run().expect("bounded application outcome");

        assert_eq!(
            outcome,
            TrustedHostLocalApplicationOutcome::CanceledBeforeEntry
        );
        assert_eq!(calls.load(Ordering::Relaxed), 0);
        assert_eq!(identity_provider.calls, 0);
        let after = crate::sqlite_state::continuity_codec::load_snapshot(
            &fixture.backend.connection().expect("connection"),
        )
        .expect("snapshot after session");
        assert!(before == after);
    }

    #[cfg(feature = "trusted-host-application-spi")]
    #[test]
    fn application_preparation_is_read_only_and_runs_one_existing_window() {
        let fixture = Fixture::new();
        let locator =
            register_resumable_operational_entry(&fixture, "application-prepare-existing", 3);
        let before = crate::sqlite_state::continuity_codec::load_snapshot(
            &fixture.backend.connection().expect("connection"),
        )
        .expect("snapshot before preparation");
        let events_before = fixture
            .backend
            .read_events(&fixture.run_id)
            .expect("events before preparation")
            .len();
        let calls = AtomicUsize::new(0);
        let executor = CountingExecutor { calls: &calls };

        let prepared = prepare_trusted_host_local_application_session(
            TrustedHostLocalApplicationPreparationInput {
                backend: &fixture.backend,
                locator,
                posture: TrustedHostLocalApplicationPreparationPosture::Existing,
                executor: &executor,
                skill_input: skill_input(&fixture),
            },
        )
        .expect("prepare existing session");

        assert!(format!("{prepared:?}").contains("[REDACTED]"));
        assert!(
            crate::sqlite_state::continuity_codec::load_snapshot(
                &fixture.backend.connection().expect("connection"),
            )
            .expect("snapshot after preparation")
                == before
        );
        assert_eq!(
            fixture
                .backend
                .read_events(&fixture.run_id)
                .expect("events after preparation")
                .len(),
            events_before
        );
        assert_eq!(calls.load(Ordering::Relaxed), 0);

        let (session, _handle) = prepared.into_parts();
        assert_eq!(
            session.run().expect("prepared session runs"),
            TrustedHostLocalApplicationOutcome::EntryStopped(
                crate::TrustedHostLocalApplicationEntryStopReason::Terminal,
            )
        );
        assert_eq!(calls.load(Ordering::Relaxed), 1);
    }

    #[cfg(feature = "trusted-host-application-spi")]
    #[test]
    fn application_preparation_rejects_existing_window_actor_substitution_before_issuance() {
        let fixture = Fixture::new();
        let mut locator =
            register_resumable_operational_entry(&fixture, "application-prepare-actor", 3);
        locator.subject_actor_id = ActorId::new("agent/substituted").expect("actor");
        let before = crate::sqlite_state::continuity_codec::load_snapshot(
            &fixture.backend.connection().expect("connection"),
        )
        .expect("snapshot before preparation");
        let events_before = fixture
            .backend
            .read_events(&fixture.run_id)
            .expect("events before preparation")
            .len();
        let calls = AtomicUsize::new(0);
        let executor = CountingExecutor { calls: &calls };

        let error = prepare_trusted_host_local_application_session(
            TrustedHostLocalApplicationPreparationInput {
                backend: &fixture.backend,
                locator,
                posture: TrustedHostLocalApplicationPreparationPosture::Existing,
                executor: &executor,
                skill_input: skill_input(&fixture),
            },
        )
        .expect_err("actor substitution must fail before issuance");

        assert_eq!(
            error.code(),
            "trusted_host_operational_entry.window_binding_mismatch"
        );
        assert_eq!(calls.load(Ordering::Relaxed), 0);
        assert!(
            crate::sqlite_state::continuity_codec::load_snapshot(
                &fixture.backend.connection().expect("connection"),
            )
            .expect("snapshot after preparation")
                == before
        );
        assert_eq!(
            fixture
                .backend
                .read_events(&fixture.run_id)
                .expect("events after preparation")
                .len(),
            events_before
        );
    }

    #[cfg(feature = "trusted-host-application-spi")]
    #[test]
    fn application_preparation_rejects_existing_window_bundle_substitution_before_issuance() {
        let fixture = Fixture::new();
        let mut locator =
            register_resumable_operational_entry(&fixture, "application-prepare-bundle", 3);
        locator.immutable_run_bundle = locator
            .immutable_run_bundle
            .with_test_root_hash(SpecContentHash::from_text("substituted bundle"));
        let before = crate::sqlite_state::continuity_codec::load_snapshot(
            &fixture.backend.connection().expect("connection"),
        )
        .expect("snapshot before preparation");
        let events_before = fixture
            .backend
            .read_events(&fixture.run_id)
            .expect("events before preparation")
            .len();
        let calls = AtomicUsize::new(0);
        let executor = CountingExecutor { calls: &calls };

        let error = prepare_trusted_host_local_application_session(
            TrustedHostLocalApplicationPreparationInput {
                backend: &fixture.backend,
                locator,
                posture: TrustedHostLocalApplicationPreparationPosture::Existing,
                executor: &executor,
                skill_input: skill_input(&fixture),
            },
        )
        .expect_err("bundle substitution must fail before issuance");

        assert_eq!(
            error.code(),
            "trusted_host_operational_entry.run_not_eligible"
        );
        assert_eq!(calls.load(Ordering::Relaxed), 0);
        assert!(
            crate::sqlite_state::continuity_codec::load_snapshot(
                &fixture.backend.connection().expect("connection"),
            )
            .expect("snapshot after preparation")
                == before
        );
        assert_eq!(
            fixture
                .backend
                .read_events(&fixture.run_id)
                .expect("events after preparation")
                .len(),
            events_before
        );
    }

    #[cfg(feature = "trusted-host-application-spi")]
    #[test]
    fn application_preparation_rejects_existing_window_invocation_substitution_before_issuance() {
        let fixture = Fixture::new();
        let locator =
            register_resumable_operational_entry(&fixture, "application-prepare-invocation", 3);
        let before = crate::sqlite_state::continuity_codec::load_snapshot(
            &fixture.backend.connection().expect("connection"),
        )
        .expect("snapshot before preparation");
        let events_before = fixture
            .backend
            .read_events(&fixture.run_id)
            .expect("events before preparation")
            .len();
        let calls = AtomicUsize::new(0);
        let executor = DifferentBindingExecutor { calls: &calls };

        let error = prepare_trusted_host_local_application_session(
            TrustedHostLocalApplicationPreparationInput {
                backend: &fixture.backend,
                locator,
                posture: TrustedHostLocalApplicationPreparationPosture::Existing,
                executor: &executor,
                skill_input: skill_input(&fixture),
            },
        )
        .expect_err("invocation substitution must fail before issuance");

        assert_eq!(
            error.code(),
            "trusted_host_operational_entry.invocation_binding_mismatch"
        );
        assert_eq!(calls.load(Ordering::Relaxed), 0);
        assert!(
            crate::sqlite_state::continuity_codec::load_snapshot(
                &fixture.backend.connection().expect("connection"),
            )
            .expect("snapshot after preparation")
                == before
        );
        assert_eq!(
            fixture
                .backend
                .read_events(&fixture.run_id)
                .expect("events after preparation")
                .len(),
            events_before
        );
    }

    #[cfg(feature = "trusted-host-application-spi")]
    #[test]
    fn application_preparation_is_read_only_and_runs_one_fresh_window() {
        let fixture = FreshApplicationFixture::new();
        let calls = AtomicUsize::new(0);
        let executor = CountingExecutor { calls: &calls };
        let invocation_commitment = fixture.invocation_commitment(&executor);
        let before = crate::sqlite_state::continuity_codec::load_snapshot(
            &fixture.runtime.backend.connection().expect("connection"),
        )
        .expect("snapshot before preparation");
        let events_before = fixture
            .runtime
            .backend
            .read_events(&fixture.runtime.run_id)
            .expect("events before preparation")
            .len();

        let prepared = prepare_trusted_host_local_application_session(
            fixture.input(&executor, &invocation_commitment),
        )
        .expect("prepare fresh session");

        assert!(
            crate::sqlite_state::continuity_codec::load_snapshot(
                &fixture.runtime.backend.connection().expect("connection"),
            )
            .expect("snapshot after preparation")
                == before
        );
        assert_eq!(
            fixture
                .runtime
                .backend
                .read_events(&fixture.runtime.run_id)
                .expect("events after preparation")
                .len(),
            events_before
        );
        assert_eq!(fixture.runtime.counts(), (0, 0, 0, 0));
        assert_eq!(calls.load(Ordering::Relaxed), 0);

        let (session, _handle) = prepared.into_parts();
        assert_eq!(
            session.run().expect("prepared fresh session runs"),
            TrustedHostLocalApplicationOutcome::EntryStopped(
                crate::TrustedHostLocalApplicationEntryStopReason::Terminal,
            )
        );
        assert_eq!(calls.load(Ordering::Relaxed), 1);
        assert_eq!(fixture.runtime.counts(), (1, 1, 1, 1));
    }

    #[cfg(feature = "trusted-host-application-spi")]
    #[test]
    fn dropping_fresh_application_preparation_is_zero_write() {
        let fixture = FreshApplicationFixture::new();
        let calls = AtomicUsize::new(0);
        let executor = CountingExecutor { calls: &calls };
        let invocation_commitment = fixture.invocation_commitment(&executor);
        let before = crate::sqlite_state::continuity_codec::load_snapshot(
            &fixture.runtime.backend.connection().expect("connection"),
        )
        .expect("snapshot before preparation");
        let events_before = fixture
            .runtime
            .backend
            .read_events(&fixture.runtime.run_id)
            .expect("events before preparation")
            .len();

        let prepared = prepare_trusted_host_local_application_session(
            fixture.input(&executor, &invocation_commitment),
        )
        .expect("prepare fresh session");
        drop(prepared);

        assert_eq!(calls.load(Ordering::Relaxed), 0);
        assert_eq!(fixture.runtime.counts(), (0, 0, 0, 0));
        assert!(
            crate::sqlite_state::continuity_codec::load_snapshot(
                &fixture.runtime.backend.connection().expect("connection"),
            )
            .expect("snapshot after preparation")
                == before
        );
        assert_eq!(
            fixture
                .runtime
                .backend
                .read_events(&fixture.runtime.run_id)
                .expect("events after preparation")
                .len(),
            events_before
        );
    }

    #[cfg(feature = "trusted-host-application-spi")]
    #[test]
    fn fresh_application_preparation_cancellation_before_run_is_zero_entry() {
        let fixture = FreshApplicationFixture::new();
        let calls = AtomicUsize::new(0);
        let executor = CountingExecutor { calls: &calls };
        let invocation_commitment = fixture.invocation_commitment(&executor);
        let before = crate::sqlite_state::continuity_codec::load_snapshot(
            &fixture.runtime.backend.connection().expect("connection"),
        )
        .expect("snapshot before preparation");
        let events_before = fixture
            .runtime
            .backend
            .read_events(&fixture.runtime.run_id)
            .expect("events before preparation")
            .len();
        let prepared = prepare_trusted_host_local_application_session(
            fixture.input(&executor, &invocation_commitment),
        )
        .expect("prepare fresh session");
        let (session, handle) = prepared.into_parts();
        handle.request_cancellation().expect("bounded cancellation");

        assert_eq!(
            session.run().expect("canceled session outcome"),
            TrustedHostLocalApplicationOutcome::CanceledBeforeEntry
        );
        assert_eq!(calls.load(Ordering::Relaxed), 0);
        assert_eq!(fixture.runtime.counts(), (0, 0, 0, 0));
        assert!(
            crate::sqlite_state::continuity_codec::load_snapshot(
                &fixture.runtime.backend.connection().expect("connection"),
            )
            .expect("snapshot after cancellation")
                == before
        );
        assert_eq!(
            fixture
                .runtime
                .backend
                .read_events(&fixture.runtime.run_id)
                .expect("events after cancellation")
                .len(),
            events_before
        );
    }

    #[cfg(feature = "trusted-host-application-spi")]
    #[test]
    fn fresh_application_preparation_rejects_backend_substitution_before_issuance() {
        let fixture = FreshApplicationFixture::new();
        let substituted_backend = Fixture::new();
        let calls = AtomicUsize::new(0);
        let executor = CountingExecutor { calls: &calls };
        let invocation_commitment = fixture.invocation_commitment(&executor);
        let mut input = fixture.input(&executor, &invocation_commitment);
        let TrustedHostLocalApplicationPreparationPosture::Fresh { opening, .. } =
            &mut input.posture
        else {
            panic!("fresh preparation posture");
        };
        opening.backend = &substituted_backend.backend;

        let error = prepare_trusted_host_local_application_session(input)
            .expect_err("backend substitution must fail before issuance");

        assert_eq!(
            error.code(),
            "trusted_host_operational_entry.opening_binding_mismatch"
        );
        assert_eq!(calls.load(Ordering::Relaxed), 0);
        assert_eq!(fixture.runtime.counts(), (0, 0, 0, 0));
        assert_eq!(substituted_backend.counts(), (0, 0, 0, 0));
    }

    #[cfg(feature = "trusted-host-application-spi")]
    #[test]
    fn fresh_application_preparation_rejects_actor_substitution_before_issuance() {
        let fixture = FreshApplicationFixture::new();
        let calls = AtomicUsize::new(0);
        let executor = CountingExecutor { calls: &calls };
        let invocation_commitment = fixture.invocation_commitment(&executor);
        let mut input = fixture.input(&executor, &invocation_commitment);
        input.locator.subject_actor_id = ActorId::new("agent/substituted-actor").expect("actor");

        let error = prepare_trusted_host_local_application_session(input)
            .expect_err("actor substitution must fail before issuance");

        assert_eq!(
            error.code(),
            "trusted_host_operational_entry.opening_binding_mismatch"
        );
        assert_eq!(calls.load(Ordering::Relaxed), 0);
        assert_eq!(fixture.runtime.counts(), (0, 0, 0, 0));
    }

    #[cfg(feature = "trusted-host-application-spi")]
    #[test]
    fn fresh_application_preparation_rejects_bundle_substitution_before_issuance() {
        let fixture = FreshApplicationFixture::new();
        let calls = AtomicUsize::new(0);
        let executor = CountingExecutor { calls: &calls };
        let invocation_commitment = fixture.invocation_commitment(&executor);
        let mut input = fixture.input(&executor, &invocation_commitment);
        input.locator.immutable_run_bundle = input
            .locator
            .immutable_run_bundle
            .with_test_root_hash(SpecContentHash::from_text("substituted fresh bundle"));

        let error = prepare_trusted_host_local_application_session(input)
            .expect_err("bundle substitution must fail before issuance");

        assert_eq!(
            error.code(),
            "trusted_host_operational_entry.run_not_eligible"
        );
        assert_eq!(calls.load(Ordering::Relaxed), 0);
        assert_eq!(fixture.runtime.counts(), (0, 0, 0, 0));
    }

    #[cfg(feature = "trusted-host-application-spi")]
    #[test]
    fn fresh_application_preparation_rejects_invocation_substitution_before_issuance() {
        let fixture = FreshApplicationFixture::new();
        let calls = AtomicUsize::new(0);
        let committed_executor = CountingExecutor { calls: &calls };
        let substituted_executor = DifferentBindingExecutor { calls: &calls };
        let invocation_commitment = fixture.invocation_commitment(&committed_executor);

        let error = prepare_trusted_host_local_application_session(
            fixture.input(&substituted_executor, &invocation_commitment),
        )
        .expect_err("invocation substitution must fail before issuance");

        assert_eq!(
            error.code(),
            "trusted_host_operational_entry.opening_binding_mismatch"
        );
        assert_eq!(calls.load(Ordering::Relaxed), 0);
        assert_eq!(fixture.runtime.counts(), (0, 0, 0, 0));
    }

    #[cfg(feature = "trusted-host-application-spi")]
    #[test]
    fn fresh_application_preparation_rejects_invalid_opening_shape_before_issuance() {
        let fixture = FreshApplicationFixture::new();
        let calls = AtomicUsize::new(0);
        let executor = CountingExecutor { calls: &calls };
        let invocation_commitment = fixture.invocation_commitment(&executor);
        let mut input = fixture.input(&executor, &invocation_commitment);
        let TrustedHostLocalApplicationPreparationPosture::Fresh { opening, .. } =
            &mut input.posture
        else {
            panic!("fresh preparation posture");
        };
        opening.maximum_attempts = 0;

        let error = prepare_trusted_host_local_application_session(input)
            .expect_err("invalid opening shape must fail before issuance");

        assert_eq!(
            error.code(),
            "trusted_host_operational_entry.opening_context_invalid"
        );
        assert_eq!(calls.load(Ordering::Relaxed), 0);
        assert_eq!(fixture.runtime.counts(), (0, 0, 0, 0));
    }

    #[cfg(feature = "trusted-host-application-spi")]
    #[test]
    fn fresh_application_preparation_rejects_trusted_time_substitution_before_issuance() {
        let fixture = FreshApplicationFixture::new();
        let calls = AtomicUsize::new(0);
        let executor = CountingExecutor { calls: &calls };
        let invocation_commitment = fixture.invocation_commitment(&executor);
        let mut input = fixture.input(&executor, &invocation_commitment);
        let TrustedHostLocalApplicationPreparationPosture::Fresh { opening, .. } =
            &mut input.posture
        else {
            panic!("fresh preparation posture");
        };
        opening.trusted_time = trusted_time_observation(
            Timestamp::parse_rfc3339("2026-07-26T10:25:00Z").expect("observed"),
            TrustedTimeSourceKind::CoreInjectedClockV1,
            SpecContentHash::from_text("substituted trusted-time provenance"),
            ContinuityTrustedTimeEpochId::new(super::super::CONTINUITY_CLOCK_EPOCH).expect("epoch"),
        );

        let error = prepare_trusted_host_local_application_session(input)
            .expect_err("trusted-time substitution must fail before issuance");

        assert_eq!(
            error.code(),
            "trusted_host_operational_entry.opening_trusted_time_invalid"
        );
        assert_eq!(calls.load(Ordering::Relaxed), 0);
        assert_eq!(fixture.runtime.counts(), (0, 0, 0, 0));
    }

    #[cfg(feature = "trusted-host-application-spi")]
    #[test]
    fn dropping_application_preparation_is_zero_write() {
        let fixture = Fixture::new();
        let locator = register_resumable_operational_entry(&fixture, "application-prepare-drop", 3);
        let before = crate::sqlite_state::continuity_codec::load_snapshot(
            &fixture.backend.connection().expect("connection"),
        )
        .expect("snapshot before preparation");
        let events_before = fixture
            .backend
            .read_events(&fixture.run_id)
            .expect("events before preparation")
            .len();
        let calls = AtomicUsize::new(0);
        let executor = CountingExecutor { calls: &calls };

        let prepared = prepare_trusted_host_local_application_session(
            TrustedHostLocalApplicationPreparationInput {
                backend: &fixture.backend,
                locator,
                posture: TrustedHostLocalApplicationPreparationPosture::Existing,
                executor: &executor,
                skill_input: skill_input(&fixture),
            },
        )
        .expect("prepare existing session");
        drop(prepared);

        assert_eq!(calls.load(Ordering::Relaxed), 0);
        assert!(
            crate::sqlite_state::continuity_codec::load_snapshot(
                &fixture.backend.connection().expect("connection"),
            )
            .expect("snapshot after dropped preparation")
                == before
        );
        assert_eq!(
            fixture
                .backend
                .read_events(&fixture.run_id)
                .expect("events after dropped preparation")
                .len(),
            events_before
        );
    }

    #[cfg(feature = "trusted-host-application-spi")]
    #[test]
    fn application_preparation_rejects_state_changed_before_run() {
        let fixture = Fixture::new();
        let locator =
            register_resumable_operational_entry(&fixture, "application-prepare-stale", 3);
        let calls = AtomicUsize::new(0);
        let executor = CountingExecutor { calls: &calls };
        let prepared = prepare_trusted_host_local_application_session(
            TrustedHostLocalApplicationPreparationInput {
                backend: &fixture.backend,
                locator: TrustedHostOperationalEntryLocator {
                    workflow_id: locator.workflow_id.clone(),
                    run_id: locator.run_id.clone(),
                    step_id: locator.step_id.clone(),
                    window_id: locator.window_id.clone(),
                    subject_actor_id: locator.subject_actor_id.clone(),
                    immutable_run_bundle: locator.immutable_run_bundle.clone(),
                },
                posture: TrustedHostLocalApplicationPreparationPosture::Existing,
                executor: &executor,
                skill_input: skill_input(&fixture),
            },
        )
        .expect("prepare existing session");
        let mut identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };
        enter_trusted_host_operation(TrustedHostOperationalEntryInput {
            backend: &fixture.backend,
            locator,
            posture: TrustedHostOperationalEntryPosture::Existing,
            executor: &executor,
            skill_input: skill_input(&fixture),
            identity_provider: &mut identity_provider,
            expected_preparation_commitment: None,
        })
        .expect("advance state after preparation");

        let (session, _handle) = prepared.into_parts();
        assert_eq!(
            session.run().expect_err("stale preparation must fail"),
            crate::TrustedHostLocalApplicationFailure::Security
        );
        assert_eq!(calls.load(Ordering::Relaxed), 1);
    }

    #[cfg(feature = "trusted-host-application-spi")]
    #[test]
    fn application_preparation_cancellation_before_run_is_zero_entry() {
        let fixture = Fixture::new();
        let locator =
            register_resumable_operational_entry(&fixture, "application-prepare-cancel", 3);
        let before = crate::sqlite_state::continuity_codec::load_snapshot(
            &fixture.backend.connection().expect("connection"),
        )
        .expect("snapshot before preparation");
        let calls = AtomicUsize::new(0);
        let executor = CountingExecutor { calls: &calls };
        let prepared = prepare_trusted_host_local_application_session(
            TrustedHostLocalApplicationPreparationInput {
                backend: &fixture.backend,
                locator,
                posture: TrustedHostLocalApplicationPreparationPosture::Existing,
                executor: &executor,
                skill_input: skill_input(&fixture),
            },
        )
        .expect("prepare existing session");
        let (session, handle) = prepared.into_parts();
        handle.request_cancellation().expect("bounded cancellation");

        assert_eq!(
            session.run().expect("canceled session outcome"),
            TrustedHostLocalApplicationOutcome::CanceledBeforeEntry
        );
        assert_eq!(calls.load(Ordering::Relaxed), 0);
        assert!(
            crate::sqlite_state::continuity_codec::load_snapshot(
                &fixture.backend.connection().expect("connection"),
            )
            .expect("snapshot after cancellation")
                == before
        );
    }

    #[test]
    fn process_owner_admitted_attempt_is_not_interrupted_by_later_cancellation() {
        let fixture = Fixture::new();
        let locator = register_resumable_operational_entry(&fixture, "owner-admitted", 3);
        let backend = fixture.backend.clone();
        let input = skill_input(&fixture);
        let calls = Arc::new(AtomicUsize::new(0));
        let release = Arc::new((Mutex::new(false), Condvar::new()));
        let (entered_sender, entered_receiver) = mpsc::channel();
        let (handle_sender, handle_receiver) = mpsc::channel();

        let (outcome, retained_handle) = std::thread::scope(|scope| {
            let calls = Arc::clone(&calls);
            let release_for_worker = Arc::clone(&release);
            let worker = scope.spawn(move || {
                let executor = BlockingTimeWindowExecutor {
                    calls,
                    entered: entered_sender,
                    release: release_for_worker,
                };
                let mut identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };
                let (owner, handle) =
                    TrustedHostExplicitLocalProcessOwner::new(TrustedHostOperationalEntryInput {
                        backend: &backend,
                        locator,
                        posture: TrustedHostOperationalEntryPosture::Existing,
                        executor: &executor,
                        skill_input: input,
                        identity_provider: &mut identity_provider,
                        expected_preparation_commitment: None,
                    });
                handle_sender.send(handle.clone()).expect("send handle");
                owner.run()
            });
            let handle = handle_receiver.recv().expect("receive handle");
            entered_receiver.recv().expect("executor entered");
            handle.cancel().expect("cancel admitted owner");
            let (lock, signal) = &*release;
            *lock.lock().expect("release lock") = true;
            signal.notify_all();
            (
                worker.join().expect("owner worker").expect("owner outcome"),
                handle,
            )
        });

        let TrustedHostExplicitLocalProcessOwnerOutcome::OperationStopped(
            TrustedHostExplicitLocalOperationOutcome::ContinuationStopped(continuation),
        ) = outcome
        else {
            panic!("admitted yield must reach cancellation-aware timer continuation");
        };
        assert_eq!(
            continuation.stop_reason,
            TrustedHostRepeatedSchedulingStopReason::Canceled
        );
        assert_eq!(continuation.executor_entries, 0);
        assert_eq!(calls.load(Ordering::Relaxed), 1);
        retained_handle
            .cancel()
            .expect("post-return cancellation remains idempotent");
    }

    fn assert_competing_process_owners_admit_at_most_one_executor_entry() {
        let fixture = Fixture::new();
        let locator = register_resumable_operational_entry(&fixture, "owner-competing", 3);
        let calls = Arc::new(AtomicUsize::new(0));
        let barrier = Arc::new(Barrier::new(3));
        let mut workers = Vec::new();

        for _worker_index in 0..2 {
            let backend = fixture.backend.clone();
            let locator = TrustedHostOperationalEntryLocator {
                workflow_id: locator.workflow_id.clone(),
                run_id: locator.run_id.clone(),
                step_id: locator.step_id.clone(),
                window_id: locator.window_id.clone(),
                subject_actor_id: locator.subject_actor_id.clone(),
                immutable_run_bundle: locator.immutable_run_bundle.clone(),
            };
            let input = skill_input(&fixture);
            let calls = Arc::clone(&calls);
            let barrier = Arc::clone(&barrier);
            workers.push(std::thread::spawn(move || {
                let executor = CountingExecutor { calls: &calls };
                let mut identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };
                let (owner, _handle) =
                    TrustedHostExplicitLocalProcessOwner::new(TrustedHostOperationalEntryInput {
                        backend: &backend,
                        locator,
                        posture: TrustedHostOperationalEntryPosture::Existing,
                        executor: &executor,
                        skill_input: input,
                        identity_provider: &mut identity_provider,
                        expected_preparation_commitment: None,
                    });
                barrier.wait();
                owner.run()
            }));
        }

        barrier.wait();
        let outcomes = workers
            .into_iter()
            .map(|worker| worker.join().expect("owner worker"))
            .collect::<Vec<_>>();

        assert_eq!(calls.load(Ordering::Relaxed), 1);
        assert_eq!(outcomes.len(), 2);
        let mut winners = 0;
        let mut bounded_losers = 0;
        for outcome in outcomes {
            let debug = format!("{outcome:?}");
            assert!(!debug.contains(fixture.run_id.as_str()));
            assert!(!debug.contains("local-supervisor-output"));
            match outcome {
                Ok(TrustedHostExplicitLocalProcessOwnerOutcome::OperationStopped(
                    TrustedHostExplicitLocalOperationOutcome::EntryStopped(entry),
                )) if entry.disposition == AuthoritativeContinuationDisposition::Terminal
                    && entry.stop_reason == TrustedHostRedispatchStopReason::Terminal
                    && entry.executor_entries == 1 =>
                {
                    winners += 1;
                }
                Ok(TrustedHostExplicitLocalProcessOwnerOutcome::OperationStopped(
                    TrustedHostExplicitLocalOperationOutcome::EntryStopped(entry),
                )) if entry.disposition == AuthoritativeContinuationDisposition::Blocked
                    && entry.stop_reason == TrustedHostRedispatchStopReason::Blocked
                    && entry.executor_entries == 0 =>
                {
                    bounded_losers += 1;
                }
                Err(error)
                    if matches!(
                        error.code(),
                        "trusted_host_redispatch.directive_replayed"
                            | "trusted_host_redispatch.directive_missing"
                            | "trusted_host_redispatch.active_yield_missing"
                    ) =>
                {
                    bounded_losers += 1;
                }
                other => panic!("unexpected competing-owner result: {other:?}"),
            }
        }
        assert_eq!(winners, 1);
        assert_eq!(bounded_losers, 1);
    }

    #[test]
    fn competing_process_owners_admit_at_most_one_executor_entry() {
        for _ in 0..32 {
            assert_competing_process_owners_admit_at_most_one_executor_entry();
        }
    }

    #[test]
    fn local_timer_wrapper_cancellation_is_zero_write() {
        let fixture = Fixture::new();
        let deadline = Timestamp::from_offset_date_time(
            Timestamp::now_utc().as_offset_date_time() + time::Duration::hours(1),
        );
        let locator =
            register_supervisor_time_window_wait(&fixture, "local-timer-cancel", deadline);
        let before = crate::sqlite_state::continuity_codec::load_snapshot(
            &fixture.backend.connection().expect("connection"),
        )
        .expect("snapshot before cancellation");
        let executor_calls = AtomicUsize::new(0);
        let executor = CountingExecutor {
            calls: &executor_calls,
        };
        let mut redispatch_identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };
        let mut wake_identity_provider = DeterministicWakeIdentityProvider { calls: 0 };
        let (cancellation, handle) = TrustedHostLocalTimerCancellation::new();
        handle.cancel().expect("cancel");

        let outcome = run_trusted_host_local_timer(TrustedHostLocalTimerInput {
            backend: &fixture.backend,
            locator,
            wake_budget: TrustedHostRepeatedWakeBudget::new(2).expect("budget"),
            cancellation,
            executor: &executor,
            skill_input: skill_input(&fixture),
            redispatch_identity_provider: &mut redispatch_identity_provider,
            wake_identity_provider: &mut wake_identity_provider,
        })
        .expect("canceled local timer run");

        assert_eq!(
            outcome.stop_reason,
            TrustedHostRepeatedSchedulingStopReason::Canceled
        );
        assert_eq!(outcome.scheduled_wakes, 1);
        assert_eq!(outcome.executor_entries, 0);
        assert_eq!(executor_calls.load(Ordering::Relaxed), 0);
        let after = crate::sqlite_state::continuity_codec::load_snapshot(
            &fixture.backend.connection().expect("connection"),
        )
        .expect("snapshot after cancellation");
        assert!(
            before == after,
            "cancellation must not mutate continuity state"
        );
    }

    #[test]
    fn local_timer_wrapper_composes_two_real_deadlines() {
        let fixture = Fixture::new();
        let deadline = Timestamp::from_offset_date_time(
            Timestamp::now_utc().as_offset_date_time() + time::Duration::milliseconds(500),
        );
        let locator = register_supervisor_time_window_wait_with_attempts(
            &fixture,
            "local-timer-two-waits",
            deadline,
            3,
        );
        let executor_calls = AtomicUsize::new(0);
        let executor = YieldTimeWindowThenFailExecutor {
            calls: &executor_calls,
        };
        let mut redispatch_identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };
        let mut wake_identity_provider = DeterministicWakeIdentityProvider { calls: 0 };
        let (cancellation, _handle) = TrustedHostLocalTimerCancellation::new();

        let outcome = run_trusted_host_local_timer(TrustedHostLocalTimerInput {
            backend: &fixture.backend,
            locator,
            wake_budget: TrustedHostRepeatedWakeBudget::new(2).expect("budget"),
            cancellation,
            executor: &executor,
            skill_input: skill_input(&fixture),
            redispatch_identity_provider: &mut redispatch_identity_provider,
            wake_identity_provider: &mut wake_identity_provider,
        })
        .expect("two local timer waits");

        assert_eq!(
            outcome.stop_reason,
            TrustedHostRepeatedSchedulingStopReason::Terminal
        );
        assert_eq!(outcome.scheduled_wakes, 2);
        assert_eq!(outcome.executor_entries, 2);
        assert_eq!(executor_calls.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn local_timer_wrapper_reconstructs_after_backend_reopen() {
        let fixture = Fixture::new();
        let deadline = Timestamp::from_offset_date_time(
            Timestamp::now_utc().as_offset_date_time() + time::Duration::milliseconds(500),
        );
        let locator =
            register_supervisor_time_window_wait(&fixture, "local-timer-restart", deadline);
        let reopened = SqliteStateBackend::open(&fixture.path).expect("reopen backend");
        let executor_calls = AtomicUsize::new(0);
        let executor = CountingExecutor {
            calls: &executor_calls,
        };
        let mut redispatch_identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };
        let mut wake_identity_provider = DeterministicWakeIdentityProvider { calls: 0 };
        let (cancellation, _handle) = TrustedHostLocalTimerCancellation::new();

        let outcome = run_trusted_host_local_timer(TrustedHostLocalTimerInput {
            backend: &reopened,
            locator,
            wake_budget: TrustedHostRepeatedWakeBudget::new(1).expect("budget"),
            cancellation,
            executor: &executor,
            skill_input: skill_input(&fixture),
            redispatch_identity_provider: &mut redispatch_identity_provider,
            wake_identity_provider: &mut wake_identity_provider,
        })
        .expect("reconstructed local timer");

        assert_eq!(
            outcome.stop_reason,
            TrustedHostRepeatedSchedulingStopReason::Terminal
        );
        assert_eq!(outcome.executor_entries, 1);
        assert_eq!(executor_calls.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn competing_local_timer_wrappers_enter_executor_at_most_once() {
        let fixture = Fixture::new();
        let deadline = Timestamp::from_offset_date_time(
            Timestamp::now_utc().as_offset_date_time() + time::Duration::milliseconds(500),
        );
        let locator = register_supervisor_time_window_wait(
            &fixture,
            "local-timer-competing-callers",
            deadline,
        );
        let executor_calls = Arc::new(AtomicUsize::new(0));
        let mut workers = Vec::new();

        for _ in 0..2 {
            let backend = fixture.backend.clone();
            let locator = TrustedHostOperationalEntryLocator {
                workflow_id: locator.workflow_id.clone(),
                run_id: locator.run_id.clone(),
                step_id: locator.step_id.clone(),
                window_id: locator.window_id.clone(),
                subject_actor_id: locator.subject_actor_id.clone(),
                immutable_run_bundle: locator.immutable_run_bundle.clone(),
            };
            let input = skill_input(&fixture);
            let executor_calls = Arc::clone(&executor_calls);
            workers.push(std::thread::spawn(move || {
                let executor = CountingExecutor {
                    calls: &executor_calls,
                };
                let mut redispatch_identity_provider =
                    DeterministicRedispatchIdentityProvider { calls: 0 };
                let mut wake_identity_provider = DeterministicWakeIdentityProvider { calls: 0 };
                let (cancellation, _handle) = TrustedHostLocalTimerCancellation::new();
                run_trusted_host_local_timer(TrustedHostLocalTimerInput {
                    backend: &backend,
                    locator,
                    wake_budget: TrustedHostRepeatedWakeBudget::new(1).expect("budget"),
                    cancellation,
                    executor: &executor,
                    skill_input: input,
                    redispatch_identity_provider: &mut redispatch_identity_provider,
                    wake_identity_provider: &mut wake_identity_provider,
                })
            }));
        }

        let outcomes = workers
            .into_iter()
            .map(|worker| worker.join().expect("worker"))
            .collect::<Vec<_>>();
        assert_eq!(executor_calls.load(Ordering::Relaxed), 1);
        assert_eq!(
            outcomes
                .iter()
                .filter_map(|outcome| outcome.as_ref().ok())
                .map(|outcome| outcome.executor_entries)
                .sum::<u32>(),
            1
        );
        assert!(
            outcomes.iter().all(|outcome| match outcome {
                Ok(outcome) => outcome.executor_entries <= 1,
                Err(error) => matches!(
                    error.code(),
                    "trusted_host_time_window_scheduling.ticket_stale"
                        | "trusted_host_time_window_scheduling.posture_ineligible"
                        | "trusted_host_redispatch.directive_replayed"
                        | "trusted_host_redispatch.directive_missing"
                        | "trusted_host_redispatch.active_yield_missing"
                        | "trusted_host_redispatch.attempt_limit_inconsistent"
                        | "trusted_host_time_window_caller.operation_replay_conflict"
                ),
            }),
            "unexpected competing local-timer result: {outcomes:?}"
        );
    }

    #[test]
    fn local_timer_wrapper_duplicate_wake_identity_fails_closed() {
        let fixture = Fixture::new();
        let deadline = Timestamp::from_offset_date_time(
            Timestamp::now_utc().as_offset_date_time() + time::Duration::milliseconds(500),
        );
        let locator = register_supervisor_time_window_wait_with_attempts(
            &fixture,
            "local-timer-duplicate-identity",
            deadline,
            3,
        );
        let executor_calls = AtomicUsize::new(0);
        let executor = YieldTimeWindowThenFailExecutor {
            calls: &executor_calls,
        };
        let mut redispatch_identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };
        let mut wake_identity_provider = DuplicateWakeIdentityProvider;
        let (cancellation, _handle) = TrustedHostLocalTimerCancellation::new();

        let error = run_trusted_host_local_timer(TrustedHostLocalTimerInput {
            backend: &fixture.backend,
            locator,
            wake_budget: TrustedHostRepeatedWakeBudget::new(2).expect("budget"),
            cancellation,
            executor: &executor,
            skill_input: skill_input(&fixture),
            redispatch_identity_provider: &mut redispatch_identity_provider,
            wake_identity_provider: &mut wake_identity_provider,
        })
        .expect_err("duplicate wake identity must fail closed");

        assert_eq!(
            error.code(),
            "trusted_host_time_window_caller.operation_replay_conflict"
        );
        assert_eq!(executor_calls.load(Ordering::Relaxed), 1);
        assert!(!format!("{error:?}").contains(fixture.run_id.as_str()));
    }

    #[test]
    fn repeated_schedule_early_wakes_exhaust_budget_without_writes() {
        let fixture = Fixture::new();
        let deadline = Timestamp::from_offset_date_time(
            Timestamp::now_utc().as_offset_date_time() + time::Duration::hours(1),
        );
        let locator = register_supervisor_time_window_wait(&fixture, "repeated-early", deadline);
        let before = crate::sqlite_state::continuity_codec::load_snapshot(
            &fixture.backend.connection().expect("connection"),
        )
        .expect("snapshot before repeated early wakes");
        let waiter_calls = AtomicUsize::new(0);
        let executor_calls = AtomicUsize::new(0);
        let mut waiter = SequenceDeadlineWaiter {
            calls: &waiter_calls,
            expected_deadlines: vec![Some(deadline), Some(deadline), Some(deadline)],
            delays: vec![None, None, None],
            next: 0,
            outcome: TrustedHostDeadlineWaitOutcome::Woke,
        };
        let executor = CountingExecutor {
            calls: &executor_calls,
        };
        let mut redispatch_identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };
        let mut wake_identity_provider = DeterministicWakeIdentityProvider { calls: 0 };

        let outcome =
            run_bounded_trusted_host_repeated_scheduling(TrustedHostRepeatedSchedulingInput {
                backend: &fixture.backend,
                locator,
                wake_budget: TrustedHostRepeatedWakeBudget::new(3).expect("budget"),
                deadline_waiter: &mut waiter,
                executor: &executor,
                skill_input: skill_input(&fixture),
                redispatch_identity_provider: &mut redispatch_identity_provider,
                wake_identity_provider: &mut wake_identity_provider,
            })
            .expect("bounded early wakes");

        assert_eq!(
            outcome.disposition,
            AuthoritativeContinuationDisposition::AwaitCondition
        );
        assert_eq!(
            outcome.stop_reason,
            TrustedHostRepeatedSchedulingStopReason::WakeBudgetExhausted
        );
        assert_eq!(outcome.scheduled_wakes, 3);
        assert_eq!(outcome.executor_entries, 0);
        assert_eq!(waiter_calls.load(Ordering::Relaxed), 3);
        assert_eq!(executor_calls.load(Ordering::Relaxed), 0);
        assert_eq!(wake_identity_provider.calls, 3);
        let after = crate::sqlite_state::continuity_codec::load_snapshot(
            &fixture.backend.connection().expect("connection"),
        )
        .expect("snapshot after repeated early wakes");
        assert!(before == after, "early wake budget must not mutate state");
    }

    #[test]
    fn repeated_schedule_cancellation_and_wait_failure_do_not_retry() {
        for (index, wait_outcome) in [
            Ok(TrustedHostDeadlineWaitOutcome::Canceled),
            Err(TrustedHostDeadlineWaitFailure::Unavailable),
            Err(TrustedHostDeadlineWaitFailure::Failed),
        ]
        .into_iter()
        .enumerate()
        {
            let fixture = Fixture::new();
            let deadline = Timestamp::from_offset_date_time(
                Timestamp::now_utc().as_offset_date_time() + time::Duration::hours(1),
            );
            let locator = register_supervisor_time_window_wait(
                &fixture,
                &format!("repeated-stop-{index}"),
                deadline,
            );
            let waiter_calls = AtomicUsize::new(0);
            let executor_calls = AtomicUsize::new(0);
            let mut waiter = RecordingDeadlineWaiter {
                calls: &waiter_calls,
                expected_deadline: deadline,
                outcome: wait_outcome,
            };
            let executor = CountingExecutor {
                calls: &executor_calls,
            };
            let mut redispatch_identity_provider =
                DeterministicRedispatchIdentityProvider { calls: 0 };
            let mut wake_identity_provider = DeterministicWakeIdentityProvider { calls: 0 };
            let result =
                run_bounded_trusted_host_repeated_scheduling(TrustedHostRepeatedSchedulingInput {
                    backend: &fixture.backend,
                    locator,
                    wake_budget: TrustedHostRepeatedWakeBudget::new(3).expect("budget"),
                    deadline_waiter: &mut waiter,
                    executor: &executor,
                    skill_input: skill_input(&fixture),
                    redispatch_identity_provider: &mut redispatch_identity_provider,
                    wake_identity_provider: &mut wake_identity_provider,
                });

            if index == 0 {
                let outcome = result.expect("canceled outcome");
                assert_eq!(
                    outcome.stop_reason,
                    TrustedHostRepeatedSchedulingStopReason::Canceled
                );
                assert_eq!(outcome.scheduled_wakes, 1);
            } else {
                let error = result.expect_err("host wait failure");
                assert!(matches!(
                    error.code(),
                    "trusted_host_time_window_scheduling.deadline_wait_unavailable"
                        | "trusted_host_time_window_scheduling.deadline_wait_failed"
                ));
            }
            assert_eq!(waiter_calls.load(Ordering::Relaxed), 1);
            assert_eq!(executor_calls.load(Ordering::Relaxed), 0);
            assert_eq!(wake_identity_provider.calls, 1);
        }
    }

    #[test]
    fn repeated_schedule_stops_on_unsupported_wait_without_waiting() {
        let fixture = Fixture::new();
        let locator = register_supervisor_unsupported_wait(&fixture, "repeated-unsupported");
        let waiter_calls = AtomicUsize::new(0);
        let executor_calls = AtomicUsize::new(0);
        let mut waiter = RecordingDeadlineWaiter {
            calls: &waiter_calls,
            expected_deadline: Timestamp::now_utc(),
            outcome: Ok(TrustedHostDeadlineWaitOutcome::Woke),
        };
        let executor = CountingExecutor {
            calls: &executor_calls,
        };
        let mut redispatch_identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };
        let mut wake_identity_provider = DeterministicWakeIdentityProvider { calls: 0 };

        let outcome =
            run_bounded_trusted_host_repeated_scheduling(TrustedHostRepeatedSchedulingInput {
                backend: &fixture.backend,
                locator,
                wake_budget: TrustedHostRepeatedWakeBudget::new(3).expect("budget"),
                deadline_waiter: &mut waiter,
                executor: &executor,
                skill_input: skill_input(&fixture),
                redispatch_identity_provider: &mut redispatch_identity_provider,
                wake_identity_provider: &mut wake_identity_provider,
            })
            .expect("unsupported wait outcome");

        assert_eq!(
            outcome.stop_reason,
            TrustedHostRepeatedSchedulingStopReason::UnsupportedWait
        );
        assert_eq!(
            outcome.disposition,
            AuthoritativeContinuationDisposition::ResumeNow
        );
        assert_eq!(outcome.scheduled_wakes, 0);
        assert_eq!(waiter_calls.load(Ordering::Relaxed), 0);
        assert_eq!(executor_calls.load(Ordering::Relaxed), 0);
        assert_eq!(wake_identity_provider.calls, 1);
    }

    #[test]
    fn schedule_once_early_wake_is_zero_write_and_does_not_rearm() {
        let fixture = Fixture::new();
        let deadline = Timestamp::from_offset_date_time(
            Timestamp::now_utc().as_offset_date_time() + time::Duration::hours(1),
        );
        let locator =
            register_supervisor_time_window_wait(&fixture, "schedule-once-early", deadline);
        let before = crate::sqlite_state::continuity_codec::load_snapshot(
            &fixture.backend.connection().expect("connection"),
        )
        .expect("snapshot before early wake");
        let waiter_calls = AtomicUsize::new(0);
        let executor_calls = AtomicUsize::new(0);
        let mut waiter = RecordingDeadlineWaiter {
            calls: &waiter_calls,
            expected_deadline: deadline,
            outcome: Ok(TrustedHostDeadlineWaitOutcome::Woke),
        };
        let executor = CountingExecutor {
            calls: &executor_calls,
        };
        let mut identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };

        let outcome = schedule_trusted_host_time_window_once(TrustedHostScheduleOnceInput {
            backend: &fixture.backend,
            locator,
            operation_id: ContinuityOperationId::new("operation/schedule-once-early-wake")
                .expect("operation"),
            receipt_id: ContinuityReceiptId::new("receipt/schedule-once-early-wake")
                .expect("receipt"),
            deadline_waiter: &mut waiter,
            executor: &executor,
            skill_input: skill_input(&fixture),
            identity_provider: &mut identity_provider,
        })
        .expect("early wake remains inert");

        let TrustedHostScheduleOnceOutcome::NotYetEligible {
            refreshed_scheduling,
        } = outcome
        else {
            panic!("early wake must remain not yet eligible");
        };
        assert_eq!(
            refreshed_scheduling.disposition,
            AuthoritativeContinuationDisposition::AwaitCondition
        );
        assert!(refreshed_scheduling.handoff.is_some());
        assert!(refreshed_scheduling.ticket.is_some());
        assert_eq!(waiter_calls.load(Ordering::Relaxed), 1);
        assert_eq!(executor_calls.load(Ordering::Relaxed), 0);
        assert_eq!(identity_provider.calls, 0);
        let after = crate::sqlite_state::continuity_codec::load_snapshot(
            &fixture.backend.connection().expect("connection"),
        )
        .expect("snapshot after early wake");
        assert!(
            before == after,
            "early wake must not mutate continuity state"
        );
    }

    #[test]
    fn schedule_once_cancellation_and_wait_failures_are_zero_write() {
        let cases = [
            (Ok(TrustedHostDeadlineWaitOutcome::Canceled), None),
            (
                Err(TrustedHostDeadlineWaitFailure::Unavailable),
                Some("trusted_host_time_window_scheduling.deadline_wait_unavailable"),
            ),
            (
                Err(TrustedHostDeadlineWaitFailure::Failed),
                Some("trusted_host_time_window_scheduling.deadline_wait_failed"),
            ),
        ];

        for (index, (wait_outcome, expected_error)) in cases.into_iter().enumerate() {
            let fixture = Fixture::new();
            let deadline = Timestamp::from_offset_date_time(
                Timestamp::now_utc().as_offset_date_time() + time::Duration::hours(1),
            );
            let locator = register_supervisor_time_window_wait(
                &fixture,
                &format!("schedule-once-stop-{index}"),
                deadline,
            );
            let before = crate::sqlite_state::continuity_codec::load_snapshot(
                &fixture.backend.connection().expect("connection"),
            )
            .expect("snapshot before stop");
            let waiter_calls = AtomicUsize::new(0);
            let executor_calls = AtomicUsize::new(0);
            let mut waiter = RecordingDeadlineWaiter {
                calls: &waiter_calls,
                expected_deadline: deadline,
                outcome: wait_outcome,
            };
            let executor = CountingExecutor {
                calls: &executor_calls,
            };
            let mut identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };

            let result = schedule_trusted_host_time_window_once(TrustedHostScheduleOnceInput {
                backend: &fixture.backend,
                locator,
                operation_id: ContinuityOperationId::new(format!(
                    "operation/schedule-once-stop-{index}"
                ))
                .expect("operation"),
                receipt_id: ContinuityReceiptId::new(format!("receipt/schedule-once-stop-{index}"))
                    .expect("receipt"),
                deadline_waiter: &mut waiter,
                executor: &executor,
                skill_input: skill_input(&fixture),
                identity_provider: &mut identity_provider,
            });

            match expected_error {
                Some(code) => assert_eq!(result.expect_err("wait failure").code(), code),
                None => assert!(matches!(
                    result.expect("cancellation outcome"),
                    TrustedHostScheduleOnceOutcome::Canceled
                )),
            }
            assert_eq!(waiter_calls.load(Ordering::Relaxed), 1);
            assert_eq!(executor_calls.load(Ordering::Relaxed), 0);
            assert_eq!(identity_provider.calls, 0);
            let after = crate::sqlite_state::continuity_codec::load_snapshot(
                &fixture.backend.connection().expect("connection"),
            )
            .expect("snapshot after stop");
            assert!(
                before == after,
                "stop outcome must not mutate continuity state"
            );
        }
    }

    #[test]
    fn schedule_once_rehydrates_after_restart() {
        let fixture = Fixture::new();
        let deadline = Timestamp::from_offset_date_time(
            Timestamp::now_utc().as_offset_date_time() + time::Duration::milliseconds(500),
        );
        let locator =
            register_supervisor_time_window_wait(&fixture, "schedule-once-restart", deadline);
        std::thread::sleep(std::time::Duration::from_millis(600));
        let reopened = SqliteStateBackend::open(&fixture.path).expect("reopen backend");
        let waiter_calls = AtomicUsize::new(0);
        let executor_calls = AtomicUsize::new(0);
        let mut waiter = RecordingDeadlineWaiter {
            calls: &waiter_calls,
            expected_deadline: deadline,
            outcome: Ok(TrustedHostDeadlineWaitOutcome::Woke),
        };
        let executor = CountingExecutor {
            calls: &executor_calls,
        };
        let mut identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };

        let outcome = schedule_trusted_host_time_window_once(TrustedHostScheduleOnceInput {
            backend: &reopened,
            locator,
            operation_id: ContinuityOperationId::new("operation/schedule-once-restart")
                .expect("operation"),
            receipt_id: ContinuityReceiptId::new("receipt/schedule-once-restart").expect("receipt"),
            deadline_waiter: &mut waiter,
            executor: &executor,
            skill_input: skill_input(&fixture),
            identity_provider: &mut identity_provider,
        })
        .expect("restart reinvocation");

        assert!(matches!(
            outcome,
            TrustedHostScheduleOnceOutcome::Reinvoked(_)
        ));
        assert_eq!(waiter_calls.load(Ordering::Relaxed), 1);
        assert_eq!(executor_calls.load(Ordering::Relaxed), 1);
        assert_eq!(identity_provider.calls, 1);
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn concurrent_schedule_once_callbacks_enter_executor_at_most_once() {
        let fixture = Fixture::new();
        let deadline = Timestamp::from_offset_date_time(
            Timestamp::now_utc().as_offset_date_time() + time::Duration::milliseconds(500),
        );
        let locator =
            register_supervisor_time_window_wait(&fixture, "schedule-once-concurrent", deadline);
        std::thread::sleep(std::time::Duration::from_millis(600));
        let waiter_calls = Arc::new(AtomicUsize::new(0));
        let executor_calls = Arc::new(AtomicUsize::new(0));
        let barrier = Arc::new(Barrier::new(2));
        let window_id = locator.window_id.clone();
        let mut workers = Vec::new();

        for _ in 0..2 {
            let backend = fixture.backend.clone();
            let locator = TrustedHostOperationalEntryLocator {
                workflow_id: locator.workflow_id.clone(),
                run_id: locator.run_id.clone(),
                step_id: locator.step_id.clone(),
                window_id: locator.window_id.clone(),
                subject_actor_id: locator.subject_actor_id.clone(),
                immutable_run_bundle: locator.immutable_run_bundle.clone(),
            };
            let input = skill_input(&fixture);
            let waiter_calls = Arc::clone(&waiter_calls);
            let executor_calls = Arc::clone(&executor_calls);
            let barrier = Arc::clone(&barrier);
            workers.push(std::thread::spawn(move || {
                let mut waiter = BarrierDeadlineWaiter {
                    calls: waiter_calls,
                    barrier,
                    expected_deadline: deadline,
                };
                let executor = CountingExecutor {
                    calls: &executor_calls,
                };
                let mut identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };
                schedule_trusted_host_time_window_once(TrustedHostScheduleOnceInput {
                    backend: &backend,
                    locator,
                    operation_id: ContinuityOperationId::new("operation/schedule-once-concurrent")
                        .expect("operation"),
                    receipt_id: ContinuityReceiptId::new("receipt/schedule-once-concurrent")
                        .expect("receipt"),
                    deadline_waiter: &mut waiter,
                    executor: &executor,
                    skill_input: input,
                    identity_provider: &mut identity_provider,
                })
            }));
        }

        let outcomes = workers
            .into_iter()
            .map(|worker| worker.join().expect("worker"))
            .collect::<Vec<_>>();
        assert_eq!(waiter_calls.load(Ordering::Relaxed), 2);
        assert_eq!(executor_calls.load(Ordering::Relaxed), 1);
        assert_eq!(
            outcomes
                .iter()
                .filter_map(|outcome| outcome.as_ref().ok())
                .filter_map(|outcome| match outcome {
                    TrustedHostScheduleOnceOutcome::Reinvoked(outcome) => {
                        Some(outcome.execution.executor_entries)
                    }
                    _ => None,
                })
                .sum::<u32>(),
            1
        );
        assert!(outcomes.iter().all(|outcome| match outcome {
            Ok(TrustedHostScheduleOnceOutcome::Reinvoked(outcome)) => {
                outcome.execution.stop_reason == TrustedHostRedispatchStopReason::Terminal
                    && outcome.execution.executor_entries <= 1
            }
            Err(error) => matches!(
                error.code(),
                "trusted_host_time_window_scheduling.ticket_stale"
                    | "trusted_host_time_window_scheduling.posture_ineligible"
                    | "trusted_host_redispatch.directive_replayed"
                    | "trusted_host_redispatch.attempt_limit_inconsistent"
            ),
            Ok(
                TrustedHostScheduleOnceOutcome::Canceled
                | TrustedHostScheduleOnceOutcome::NotYetEligible { .. },
            ) => false,
        }));
        assert_eq!(
            fixture
                .backend
                .continuation_disposition(&window_id)
                .expect("durable disposition"),
            AuthoritativeContinuationDisposition::Terminal
        );
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn explicit_time_window_reinvocation_enters_existing_window_once() {
        let fixture = Fixture::new();
        let capability = open_supervisor_attempt(&fixture, "opening/explicit-reinvocation", 2);
        let window_id = capability.window_id.clone();
        let locator = TrustedHostOperationalEntryLocator {
            workflow_id: fixture.workflow_id.clone(),
            run_id: fixture.run_id.clone(),
            step_id: fixture.step_id.clone(),
            window_id: capability.window_id.clone(),
            subject_actor_id: capability.subject_actor_id.clone(),
            immutable_run_bundle: fixture.bundle.clone(),
        };
        let started_at = Timestamp::now_utc();
        let deadline = Timestamp::from_offset_date_time(
            started_at.as_offset_date_time() + time::Duration::seconds(2),
        );
        supervise_one_local_skill_attempt(TrustedHostSupervisorInput {
            backend: &fixture.backend,
            capability: TrustedHostSupervisorAttemptCapability::Opened(capability),
            executor: &TimeWindowYieldExecutor {
                condition_version: 1,
                deadline,
            },
            skill_input: skill_input(&fixture),
            persistence: TrustedHostSupervisorPersistenceInput {
                operation: ContinuityOperationId::new("operation/explicit-reinvocation-yield")
                    .expect("operation"),
                receipt: ContinuityReceiptId::new("receipt/explicit-reinvocation-yield")
                    .expect("receipt"),
                yield_generation: Some(
                    ContinuityYieldGenerationId::new("yield/explicit-reinvocation")
                        .expect("generation"),
                ),
            },
        })
        .expect("initial attempt yields");
        let observed_before_deadline = Timestamp::now_utc();
        let handoff = observe_trusted_host_wait_with_time(
            &fixture.backend,
            &locator,
            trusted_time_observation(
                observed_before_deadline,
                TrustedTimeSourceKind::CoreInjectedClockV1,
                SpecContentHash::new(super::super::CONTINUITY_CLOCK_PROVENANCE)
                    .expect("provenance"),
                ContinuityTrustedTimeEpochId::new(super::super::CONTINUITY_CLOCK_EPOCH)
                    .expect("epoch"),
            ),
        )
        .expect("handoff observation")
        .handoff
        .expect("handoff");
        std::thread::sleep(std::time::Duration::from_millis(2_100));
        let calls = AtomicUsize::new(0);
        let executor = CountingExecutor { calls: &calls };
        let mut identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };

        let outcome = reinvoke_after_time_window_wait(TrustedHostTimeWindowReinvocationInput {
            backend: &fixture.backend,
            handoff: handoff.clone(),
            locator,
            condition_id: crate::AuthorizedExecutionWaitConditionId::new(
                "wait/supervisor-time-window",
            )
            .expect("condition"),
            condition_version: 1,
            operation_id: ContinuityOperationId::new("operation/explicit-reinvocation-wake")
                .expect("operation"),
            receipt_id: ContinuityReceiptId::new("receipt/explicit-reinvocation-wake")
                .expect("receipt"),
            executor: &executor,
            skill_input: skill_input(&fixture),
            identity_provider: &mut identity_provider,
        })
        .expect("explicit reinvocation");

        assert_eq!(
            outcome.wake_status,
            crate::sqlite_state::trusted_host_time_window_caller::TrustedHostTimeWindowWakeStatus::Transitioned
        );
        assert_eq!(
            outcome.execution.stop_reason,
            TrustedHostRedispatchStopReason::Terminal
        );
        assert_eq!(outcome.execution.executor_entries, 1);
        assert_eq!(calls.load(Ordering::Relaxed), 1);
        assert_eq!(identity_provider.calls, 1);

        let reopened = SqliteStateBackend::open(&fixture.path).expect("reopen backend");
        let mut replay_identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };
        let replay = reinvoke_after_time_window_wait(TrustedHostTimeWindowReinvocationInput {
            backend: &reopened,
            handoff,
            locator: TrustedHostOperationalEntryLocator {
                workflow_id: fixture.workflow_id.clone(),
                run_id: fixture.run_id.clone(),
                step_id: fixture.step_id.clone(),
                window_id,
                subject_actor_id: ActorId::new("agent/opening-test").expect("actor"),
                immutable_run_bundle: fixture.bundle.clone(),
            },
            condition_id: crate::AuthorizedExecutionWaitConditionId::new(
                "wait/supervisor-time-window",
            )
            .expect("condition"),
            condition_version: 1,
            operation_id: ContinuityOperationId::new("operation/explicit-reinvocation-wake")
                .expect("operation"),
            receipt_id: ContinuityReceiptId::new("receipt/explicit-reinvocation-wake")
                .expect("receipt"),
            executor: &executor,
            skill_input: skill_input(&fixture),
            identity_provider: &mut replay_identity_provider,
        })
        .expect("reopened exact replay");
        assert_eq!(
            replay.wake_status,
            crate::sqlite_state::trusted_host_time_window_caller::TrustedHostTimeWindowWakeStatus::ExactReplay
        );
        assert_eq!(
            replay.execution.stop_reason,
            TrustedHostRedispatchStopReason::Terminal
        );
        assert_eq!(replay.execution.executor_entries, 0);
        assert_eq!(calls.load(Ordering::Relaxed), 1);
        assert_eq!(replay_identity_provider.calls, 0);
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn concurrent_explicit_time_window_reinvocations_enter_executor_at_most_once() {
        let fixture = Fixture::new();
        let capability = open_supervisor_attempt(&fixture, "opening/concurrent-reinvocation", 2);
        let window_id = capability.window_id.clone();
        let locator = TrustedHostOperationalEntryLocator {
            workflow_id: fixture.workflow_id.clone(),
            run_id: fixture.run_id.clone(),
            step_id: fixture.step_id.clone(),
            window_id: window_id.clone(),
            subject_actor_id: capability.subject_actor_id.clone(),
            immutable_run_bundle: fixture.bundle.clone(),
        };
        let deadline = Timestamp::from_offset_date_time(
            Timestamp::now_utc().as_offset_date_time() + time::Duration::seconds(2),
        );
        supervise_one_local_skill_attempt(TrustedHostSupervisorInput {
            backend: &fixture.backend,
            capability: TrustedHostSupervisorAttemptCapability::Opened(capability),
            executor: &TimeWindowYieldExecutor {
                condition_version: 1,
                deadline,
            },
            skill_input: skill_input(&fixture),
            persistence: TrustedHostSupervisorPersistenceInput {
                operation: ContinuityOperationId::new("operation/concurrent-reinvocation-yield")
                    .expect("operation"),
                receipt: ContinuityReceiptId::new("receipt/concurrent-reinvocation-yield")
                    .expect("receipt"),
                yield_generation: Some(
                    ContinuityYieldGenerationId::new("yield/concurrent-reinvocation")
                        .expect("generation"),
                ),
            },
        })
        .expect("initial attempt yields");
        let handoff = observe_trusted_host_wait_with_time(
            &fixture.backend,
            &locator,
            trusted_time_observation(
                Timestamp::now_utc(),
                TrustedTimeSourceKind::CoreInjectedClockV1,
                SpecContentHash::new(super::super::CONTINUITY_CLOCK_PROVENANCE)
                    .expect("provenance"),
                ContinuityTrustedTimeEpochId::new(super::super::CONTINUITY_CLOCK_EPOCH)
                    .expect("epoch"),
            ),
        )
        .expect("handoff observation")
        .handoff
        .expect("handoff");
        std::thread::sleep(std::time::Duration::from_millis(2_100));

        let calls = Arc::new(AtomicUsize::new(0));
        let barrier = Arc::new(Barrier::new(2));
        let mut workers = Vec::new();
        for _ in 0..2 {
            let backend = fixture.backend.clone();
            let handoff = handoff.clone();
            let workflow_id = fixture.workflow_id.clone();
            let run_id = fixture.run_id.clone();
            let step_id = fixture.step_id.clone();
            let bundle = fixture.bundle.clone();
            let window_id = window_id.clone();
            let input = skill_input(&fixture);
            let calls = Arc::clone(&calls);
            let barrier = Arc::clone(&barrier);
            workers.push(std::thread::spawn(move || {
                let executor = CountingExecutor { calls: &calls };
                let mut identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };
                barrier.wait();
                reinvoke_after_time_window_wait(TrustedHostTimeWindowReinvocationInput {
                    backend: &backend,
                    handoff,
                    locator: TrustedHostOperationalEntryLocator {
                        workflow_id,
                        run_id,
                        step_id,
                        window_id,
                        subject_actor_id: ActorId::new("agent/opening-test").expect("actor"),
                        immutable_run_bundle: bundle,
                    },
                    condition_id: crate::AuthorizedExecutionWaitConditionId::new(
                        "wait/supervisor-time-window",
                    )
                    .expect("condition"),
                    condition_version: 1,
                    operation_id: ContinuityOperationId::new(
                        "operation/concurrent-explicit-reinvocation-wake",
                    )
                    .expect("operation"),
                    receipt_id: ContinuityReceiptId::new(
                        "receipt/concurrent-explicit-reinvocation-wake",
                    )
                    .expect("receipt"),
                    executor: &executor,
                    skill_input: input,
                    identity_provider: &mut identity_provider,
                })
            }));
        }
        let outcomes = workers
            .into_iter()
            .map(|worker| worker.join().expect("worker"))
            .collect::<Vec<_>>();

        assert_eq!(calls.load(Ordering::Relaxed), 1);
        assert_eq!(
            outcomes
                .iter()
                .filter_map(|outcome| outcome.as_ref().ok())
                .map(|outcome| outcome.execution.executor_entries)
                .sum::<u32>(),
            1
        );
        assert_eq!(
            outcomes
                .iter()
                .filter_map(|outcome| outcome.as_ref().ok())
                .filter(|outcome| outcome.execution.executor_entries == 1)
                .count(),
            1
        );
        assert!(outcomes.iter().all(|outcome| match outcome {
            Ok(outcome) if outcome.execution.executor_entries == 1 => {
                outcome.wake_status
                    != crate::sqlite_state::trusted_host_time_window_caller::TrustedHostTimeWindowWakeStatus::SecurityRejected
                    && outcome.execution.stop_reason == TrustedHostRedispatchStopReason::Terminal
            }
            Ok(outcome) if outcome.execution.executor_entries == 0 => {
                outcome.wake_status
                    != crate::sqlite_state::trusted_host_time_window_caller::TrustedHostTimeWindowWakeStatus::SecurityRejected
                    && matches!(
                        outcome.execution.stop_reason,
                        TrustedHostRedispatchStopReason::Blocked
                            | TrustedHostRedispatchStopReason::Terminal
                    )
            }
            Ok(_) => false,
            Err(error) => matches!(
                error.code(),
                "trusted_host_redispatch.directive_replayed"
                    | "trusted_host_redispatch.directive_missing"
                    | "trusted_host_redispatch.active_yield_missing"
                    | "trusted_host_redispatch.attempt_limit_inconsistent"
            ),
        }), "unexpected concurrent reinvocation result: {outcomes:?}");
        assert_eq!(
            fixture
                .backend
                .continuation_disposition(&window_id)
                .expect("durable continuation disposition"),
            AuthoritativeContinuationDisposition::Terminal
        );
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn explicit_time_window_reinvocation_recovers_after_transition_before_entry() {
        let fixture = Fixture::new();
        let capability = open_supervisor_attempt(&fixture, "opening/reinvocation-crash", 2);
        let window_id = capability.window_id.clone();
        let locator = TrustedHostOperationalEntryLocator {
            workflow_id: fixture.workflow_id.clone(),
            run_id: fixture.run_id.clone(),
            step_id: fixture.step_id.clone(),
            window_id: window_id.clone(),
            subject_actor_id: capability.subject_actor_id.clone(),
            immutable_run_bundle: fixture.bundle.clone(),
        };
        let deadline = Timestamp::from_offset_date_time(
            Timestamp::now_utc().as_offset_date_time() + time::Duration::seconds(2),
        );
        supervise_one_local_skill_attempt(TrustedHostSupervisorInput {
            backend: &fixture.backend,
            capability: TrustedHostSupervisorAttemptCapability::Opened(capability),
            executor: &TimeWindowYieldExecutor {
                condition_version: 1,
                deadline,
            },
            skill_input: skill_input(&fixture),
            persistence: TrustedHostSupervisorPersistenceInput {
                operation: ContinuityOperationId::new("operation/reinvocation-crash-yield")
                    .expect("operation"),
                receipt: ContinuityReceiptId::new("receipt/reinvocation-crash-yield")
                    .expect("receipt"),
                yield_generation: Some(
                    ContinuityYieldGenerationId::new("yield/reinvocation-crash")
                        .expect("generation"),
                ),
            },
        })
        .expect("initial attempt yields");
        let handoff = observe_trusted_host_wait_with_time(
            &fixture.backend,
            &locator,
            trusted_time_observation(
                Timestamp::now_utc(),
                TrustedTimeSourceKind::CoreInjectedClockV1,
                SpecContentHash::new(super::super::CONTINUITY_CLOCK_PROVENANCE)
                    .expect("provenance"),
                ContinuityTrustedTimeEpochId::new(super::super::CONTINUITY_CLOCK_EPOCH)
                    .expect("epoch"),
            ),
        )
        .expect("handoff observation")
        .handoff
        .expect("handoff");
        std::thread::sleep(std::time::Duration::from_millis(2_100));
        let calls = AtomicUsize::new(0);
        let executor = CountingExecutor { calls: &calls };
        let mut failed_identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };

        let error = reinvoke_after_time_window_wait_with_test_transition_hook(
            TrustedHostTimeWindowReinvocationInput {
                backend: &fixture.backend,
                handoff: handoff.clone(),
                locator,
                condition_id: crate::AuthorizedExecutionWaitConditionId::new(
                    "wait/supervisor-time-window",
                )
                .expect("condition"),
                condition_version: 1,
                operation_id: ContinuityOperationId::new("operation/reinvocation-crash-wake")
                    .expect("operation"),
                receipt_id: ContinuityReceiptId::new("receipt/reinvocation-crash-wake")
                    .expect("receipt"),
                executor: &executor,
                skill_input: skill_input(&fixture),
                identity_provider: &mut failed_identity_provider,
            },
            || {
                Err(WorkflowOsError::new(
                    WorkflowOsErrorKind::Internal,
                    "trusted_host_time_window_reinvocation.test_transition_to_entry_crash",
                    "injected transition-to-entry crash",
                ))
            },
        )
        .expect_err("injected crash must interrupt before entry");
        assert_eq!(
            error.code(),
            "trusted_host_time_window_reinvocation.test_transition_to_entry_crash"
        );
        assert_eq!(calls.load(Ordering::Relaxed), 0);
        assert_eq!(failed_identity_provider.calls, 0);

        let reopened = SqliteStateBackend::open(&fixture.path).expect("reopen backend");
        let mut recovery_identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };
        let recovered = reinvoke_after_time_window_wait(TrustedHostTimeWindowReinvocationInput {
            backend: &reopened,
            handoff: handoff.clone(),
            locator: TrustedHostOperationalEntryLocator {
                workflow_id: fixture.workflow_id.clone(),
                run_id: fixture.run_id.clone(),
                step_id: fixture.step_id.clone(),
                window_id: window_id.clone(),
                subject_actor_id: ActorId::new("agent/opening-test").expect("actor"),
                immutable_run_bundle: fixture.bundle.clone(),
            },
            condition_id: crate::AuthorizedExecutionWaitConditionId::new(
                "wait/supervisor-time-window",
            )
            .expect("condition"),
            condition_version: 1,
            operation_id: ContinuityOperationId::new("operation/reinvocation-crash-wake")
                .expect("operation"),
            receipt_id: ContinuityReceiptId::new("receipt/reinvocation-crash-wake")
                .expect("receipt"),
            executor: &executor,
            skill_input: skill_input(&fixture),
            identity_provider: &mut recovery_identity_provider,
        })
        .expect("reopened exact replay recovers");
        assert_eq!(
            recovered.wake_status,
            crate::sqlite_state::trusted_host_time_window_caller::TrustedHostTimeWindowWakeStatus::ExactReplay
        );
        assert_eq!(recovered.execution.executor_entries, 1);
        assert_eq!(calls.load(Ordering::Relaxed), 1);
        assert_eq!(recovery_identity_provider.calls, 1);

        let mut replay_identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };
        let replay = reinvoke_after_time_window_wait(TrustedHostTimeWindowReinvocationInput {
            backend: &reopened,
            handoff,
            locator: TrustedHostOperationalEntryLocator {
                workflow_id: fixture.workflow_id.clone(),
                run_id: fixture.run_id.clone(),
                step_id: fixture.step_id.clone(),
                window_id,
                subject_actor_id: ActorId::new("agent/opening-test").expect("actor"),
                immutable_run_bundle: fixture.bundle.clone(),
            },
            condition_id: crate::AuthorizedExecutionWaitConditionId::new(
                "wait/supervisor-time-window",
            )
            .expect("condition"),
            condition_version: 1,
            operation_id: ContinuityOperationId::new("operation/reinvocation-crash-wake")
                .expect("operation"),
            receipt_id: ContinuityReceiptId::new("receipt/reinvocation-crash-wake")
                .expect("receipt"),
            executor: &executor,
            skill_input: skill_input(&fixture),
            identity_provider: &mut replay_identity_provider,
        })
        .expect("post-recovery replay");
        assert_eq!(replay.execution.executor_entries, 0);
        assert_eq!(calls.load(Ordering::Relaxed), 1);
        assert_eq!(replay_identity_provider.calls, 0);
    }

    #[test]
    fn operational_entry_rejects_changed_invocation_before_resume() {
        let fixture = Fixture::new();
        let capability = open_supervisor_attempt(&fixture, "opening/entry-substitution", 2);
        let locator = TrustedHostOperationalEntryLocator {
            workflow_id: fixture.workflow_id.clone(),
            run_id: fixture.run_id.clone(),
            step_id: fixture.step_id.clone(),
            window_id: capability.window_id.clone(),
            subject_actor_id: capability.subject_actor_id.clone(),
            immutable_run_bundle: fixture.bundle.clone(),
        };
        supervise_one_local_skill_attempt(TrustedHostSupervisorInput {
            backend: &fixture.backend,
            capability: TrustedHostSupervisorAttemptCapability::Opened(capability),
            executor: &YieldExecutor,
            skill_input: skill_input(&fixture),
            persistence: TrustedHostSupervisorPersistenceInput {
                operation: ContinuityOperationId::new("operation/entry-substitution-yield")
                    .expect("operation"),
                receipt: ContinuityReceiptId::new("receipt/entry-substitution-yield")
                    .expect("receipt"),
                yield_generation: Some(
                    ContinuityYieldGenerationId::new("yield/entry-substitution")
                        .expect("generation"),
                ),
            },
        })
        .expect("initial attempt yields");
        let calls = AtomicUsize::new(0);
        let executor = CountingExecutor { calls: &calls };
        let mut substituted = skill_input(&fixture);
        substituted.values.insert(
            "context".to_owned(),
            "authorization=secret-entry-marker".to_owned(),
        );
        let mut identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };

        let error = enter_trusted_host_operation(TrustedHostOperationalEntryInput {
            backend: &fixture.backend,
            locator,
            posture: TrustedHostOperationalEntryPosture::Existing,
            executor: &executor,
            skill_input: substituted,
            identity_provider: &mut identity_provider,
            expected_preparation_commitment: None,
        })
        .expect_err("changed invocation must fail before resume");

        assert_eq!(
            error.code(),
            "trusted_host_operational_entry.invocation_binding_mismatch"
        );
        assert_eq!(calls.load(Ordering::Relaxed), 0);
        assert_eq!(identity_provider.calls, 0);
        let debug = format!("{error:?}");
        assert!(!debug.contains("secret-entry-marker"));
        assert!(!debug.contains(fixture.run_id.as_str()));
    }

    #[test]
    fn bounded_redispatch_loop_consumes_distinct_authority_for_each_resume() {
        let fixture = Fixture::new();
        let capability = open_supervisor_attempt(&fixture, "opening/repeated-redispatch", 3);
        let calls = AtomicUsize::new(0);
        let executor = YieldThenSucceedExecutor {
            calls: &calls,
            yields_before_success: 2,
        };
        let mut identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };

        let outcome = run_bounded_trusted_host_redispatch_loop(TrustedHostRedispatchLoopInput {
            backend: &fixture.backend,
            initial_capability: TrustedHostSupervisorAttemptCapability::Opened(capability),
            executor: &executor,
            skill_input: skill_input(&fixture),
            initial_persistence: TrustedHostSupervisorPersistenceInput {
                operation: ContinuityOperationId::new("operation/repeated-redispatch-initial")
                    .expect("operation"),
                receipt: ContinuityReceiptId::new("receipt/repeated-redispatch-initial")
                    .expect("receipt"),
                yield_generation: Some(
                    ContinuityYieldGenerationId::new("yield/repeated-redispatch-initial")
                        .expect("generation"),
                ),
            },
            identity_provider: &mut identity_provider,
        })
        .expect("repeated redispatch completes");

        assert_eq!(
            outcome.stop_reason,
            TrustedHostRedispatchStopReason::Terminal
        );
        assert_eq!(outcome.executor_entries, 3);
        assert_eq!(calls.load(Ordering::Relaxed), 3);
        assert_eq!(identity_provider.calls, 2);
        assert_eq!(
            outcome.skill_output.and_then(|output| output.output_ref),
            Some("bounded-redispatch-output".to_owned())
        );
        assert_eq!(
            fixture
                .backend
                .rehydrate_run(&fixture.run_id)
                .expect("run")
                .snapshot
                .status,
            WorkflowRunStatus::Running
        );
    }

    #[test]
    fn bounded_redispatch_loop_fails_closed_when_resume_exceeds_attempt_limit() {
        let fixture = Fixture::new();
        let capability = open_supervisor_attempt(&fixture, "opening/redispatch-limit", 1);
        let mut identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };

        let error = run_bounded_trusted_host_redispatch_loop(TrustedHostRedispatchLoopInput {
            backend: &fixture.backend,
            initial_capability: TrustedHostSupervisorAttemptCapability::Opened(capability),
            executor: &YieldExecutor,
            skill_input: skill_input(&fixture),
            initial_persistence: TrustedHostSupervisorPersistenceInput {
                operation: ContinuityOperationId::new("operation/redispatch-limit")
                    .expect("operation"),
                receipt: ContinuityReceiptId::new("receipt/redispatch-limit").expect("receipt"),
                yield_generation: Some(
                    ContinuityYieldGenerationId::new("yield/redispatch-limit").expect("generation"),
                ),
            },
            identity_provider: &mut identity_provider,
        })
        .expect_err("exhausted immediate continuation must fail closed");

        assert_eq!(
            error.code(),
            "trusted_host_redispatch.attempt_limit_inconsistent"
        );
        assert_eq!(identity_provider.calls, 0);
        let debug = format!("{error:?}");
        assert!(!debug.contains(fixture.run_id.as_str()));
        assert!(!debug.contains("redispatch-limit"));
    }

    #[test]
    fn bounded_redispatch_loop_rejects_substituted_input_before_executor_entry() {
        let fixture = Fixture::new();
        let capability = open_supervisor_attempt(&fixture, "opening/redispatch-substitution", 2);
        let calls = AtomicUsize::new(0);
        let executor = CountingExecutor { calls: &calls };
        let mut input = skill_input(&fixture);
        input.values.insert(
            "context".to_owned(),
            "authorization=secret-redispatch-marker".to_owned(),
        );
        let mut identity_provider = DeterministicRedispatchIdentityProvider { calls: 0 };

        let error = run_bounded_trusted_host_redispatch_loop(TrustedHostRedispatchLoopInput {
            backend: &fixture.backend,
            initial_capability: TrustedHostSupervisorAttemptCapability::Opened(capability),
            executor: &executor,
            skill_input: input,
            initial_persistence: TrustedHostSupervisorPersistenceInput {
                operation: ContinuityOperationId::new("operation/redispatch-substitution")
                    .expect("operation"),
                receipt: ContinuityReceiptId::new("receipt/redispatch-substitution")
                    .expect("receipt"),
                yield_generation: Some(
                    ContinuityYieldGenerationId::new("yield/redispatch-substitution")
                        .expect("generation"),
                ),
            },
            identity_provider: &mut identity_provider,
        })
        .expect_err("substituted invocation must fail before executor entry");

        assert_eq!(
            error.code(),
            "trusted_host_supervisor.invocation_binding_mismatch"
        );
        assert_eq!(calls.load(Ordering::Relaxed), 0);
        assert_eq!(identity_provider.calls, 0);
        let debug = format!("{error:?}");
        assert!(!debug.contains("secret-redispatch-marker"));
        assert!(!debug.contains("authorization"));
    }

    #[test]
    fn concurrent_dispatchers_enter_executor_exactly_once() {
        let fixture = Fixture::new();
        let first = open_supervisor_attempt(&fixture, "opening/concurrent-dispatch", 1);
        let second = clone_opened_capability(&first);
        let barrier = Arc::new(Barrier::new(2));
        let calls = Arc::new(AtomicUsize::new(0));
        let mut workers = Vec::new();
        for capability in [first, second] {
            let backend = fixture.backend.clone();
            let input = skill_input(&fixture);
            let barrier = Arc::clone(&barrier);
            let calls = Arc::clone(&calls);
            workers.push(std::thread::spawn(move || {
                barrier.wait();
                let executor = CountingExecutor { calls: &calls };
                supervise_one_local_skill_attempt(TrustedHostSupervisorInput {
                    backend: &backend,
                    capability: TrustedHostSupervisorAttemptCapability::Opened(capability),
                    executor: &executor,
                    skill_input: input,
                    persistence: TrustedHostSupervisorPersistenceInput {
                        operation: ContinuityOperationId::new("operation/concurrent-dispatch")
                            .expect("operation"),
                        receipt: ContinuityReceiptId::new("receipt/concurrent-dispatch")
                            .expect("receipt"),
                        yield_generation: None,
                    },
                })
            }));
        }
        let results = workers
            .into_iter()
            .map(|worker| worker.join().expect("worker"))
            .collect::<Vec<_>>();
        assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
        let loser = results
            .iter()
            .find_map(|result| result.as_ref().err())
            .expect("one loser");
        assert_eq!(
            loser.code(),
            "trusted_host_supervisor.attempt_already_admitted"
        );
        assert_eq!(calls.load(Ordering::Relaxed), 1);
    }
}
