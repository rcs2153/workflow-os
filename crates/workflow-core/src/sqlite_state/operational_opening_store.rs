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

use super::{
    append_event_and_project_snapshot, encode_json, map_sqlite_error, snapshot_commitment,
};
use super::{continuity_codec, continuity_store, SqliteStateBackend};

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
        validate_request(&request)?;
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
        let result_cursor = next_cursor(&request.expected_cursor, &request.operation_id)?;
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
                    request.expected_cursor.sequence_number,
                    request.expected_cursor.event_id.clone(),
                ),
                committed_result_cursor: OperationalExecutionWindowOpeningProjectionCursor::new(
                    result_cursor.sequence_number,
                    result_cursor.event_id.clone(),
                ),
            },
        )?;
        let first = history.first().ok_or_else(corrupt)?;
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
        insert_operation(&transaction, &request, &result, &result_cursor)?;
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
                request.expected_cursor.event_id.as_str(),
                i64::try_from(request.expected_cursor.sequence_number.get())
                    .map_err(|_| corrupt())?,
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
    use std::sync::{Arc, Barrier};

    use rusqlite::Connection;

    use super::*;
    use crate::authorized_execution_continuity_state::internal::{
        expected_consume_directive_commitment, trusted_time_observation, window_binding_commitment,
        AuthorityUseCapability, AuthorizedExecutionContinuityProjectionStore,
        ConsumeDirectiveRequest, ConsumeDirectiveResult, ContinuityCursor, ContinuityDirectiveId,
        ContinuityOperationId, ContinuityReceiptId, ContinuityTrustedTimeEpochId,
        ContinuityYieldGenerationId, TrustedTimeSourceKind,
    };
    use crate::operational_execution_window_opening::{
        operation_binding_commitment, trusted_host_invocation_commitment,
        OperationalExecutionWindowOpeningAuthorization,
    };
    use crate::sqlite_state::dispatch_reservation_store::{
        inject_dispatch_commit_fault, InjectedDispatchCommitFault,
    };
    use crate::trusted_host_supervisor::{
        inject_supervisor_persistence_fault, supervise_one_local_skill_attempt,
        InjectedSupervisorPersistenceFault, LocalSkillAttemptExecutor,
        TrustedHostAttemptExecutionContext, TrustedHostAttemptExecutionResult,
        TrustedHostAttemptExecutor, TrustedHostSupervisorAttemptCapability,
        TrustedHostSupervisorInput, TrustedHostSupervisorPersistenceInput,
    };
    use crate::{
        ActorId, AuthorizedExecutionAttemptId, AuthorizedExecutionWindowId, EventId, EventLogStore,
        ImmutableRunBundleBinding, SchemaVersion, SkillHandler, SkillId, SkillInput, SkillOutput,
        SkillVersion, StateBackend, StepId, Timestamp, WorkflowId, WorkflowOsError,
        WorkflowRunEventKind, WorkflowRunId, WorkflowVersion,
    };
    use crate::{AuthorizedExecutionYieldReason, CorrelationId, SpecContentHash};

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
            let id = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "workflow-os-operational-opening-{}-{id}.sqlite3",
                std::process::id()
            ));
            cleanup_database(&path);
            let backend = SqliteStateBackend::open(&path).expect("SQLite backend");
            let workflow_id = WorkflowId::new(format!("workflow/opening-{id}")).expect("workflow");
            let run_id = WorkflowRunId::new(format!("run-opening-{id}")).expect("run");
            let step_id = StepId::new("invoke").expect("step");
            let bundle: ImmutableRunBundleBinding = serde_json::from_value(serde_json::json!({
                "bundle_id": format!("bundle/opening-{id}"),
                "bundle_version": "v1",
                "root_hash": SpecContentHash::from_text("opening bundle").as_str(),
            }))
            .expect("bundle binding");
            let schema_version = SchemaVersion::new("workflowos.dev/v0").expect("schema");
            let workflow_version = WorkflowVersion::new("v1").expect("workflow version");
            let spec_hash = SpecContentHash::from_text("opening fixture");
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
            TrustedHostAttemptExecutionResult::Yielded(AuthorizedExecutionYieldReason::TurnBoundary)
        }
    }

    struct CountingExecutor<'a> {
        calls: &'a AtomicUsize,
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
