use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};

use rusqlite::{params, Connection, Transaction, TransactionBehavior};

use crate::authorized_execution_continuity_state::internal::{
    continuity_state_error, expected_attempt_outcome_commitment,
    expected_consume_directive_commitment, expected_recovery_commitment,
    expected_register_yield_commitment, expected_transition_wait_commitment, operation_commitment,
    rejection_commitment, result_commitment, trusted_time_commitment, trusted_time_observation,
    validate_wait_count, window_binding_commitment, AttemptUseCapability,
    AuthoritativeAttemptRecord, AuthoritativeAttemptState, AuthoritativeContinuationDisposition,
    AuthoritativeDirectiveRecord, AuthoritativeDirectiveState, AuthoritativeOperationRecord,
    AuthoritativeWaitIdentity, AuthoritativeWaitRecord, AuthoritativeWaitState,
    AuthoritativeWindowRecord, AuthoritativeWindowState, AuthoritativeYieldRecord,
    AuthorizedExecutionContinuityEligibilityReader, AuthorizedExecutionContinuityReconciler,
    AuthorizedExecutionContinuityStore, CommittedOperationDisposition, CommittedSecurityRejection,
    CommittedSecurityRejectionKind, ConsumeDirectiveRequest, ConsumeDirectiveResult,
    ContinuityCursor, ContinuityDirectiveId, ContinuityInstanceEligibility, ContinuityOperationId,
    ContinuityReceipt, ContinuityReceiptId, ContinuityReconciliationResult, ContinuityRevision,
    ContinuityTrustedTimeEpochId, ContinuityYieldGenerationId, ExpectedWindowBinding,
    MutationResult, ReconcileOperationRequest, RecordAttemptOutcomeRequest,
    RecordedOperationResult, RecoverAmbiguousAttemptRequest, ReferenceContinuityState,
    RegisterYieldRequest, RegisterYieldResult, SecurityRejectionCommitmentInput,
    TransitionWaitRequest, TrustedTimeObservation, TrustedTimePosture, TrustedTimeSecurityRecord,
    TrustedTimeSecuritySnapshot, TrustedTimeSourceKind, WindowSecuritySnapshot,
};
use crate::authorized_execution_continuity_state::semantics;
use crate::{
    AuthorizedExecutionAttemptOutcome, AuthorizedExecutionContinuityOperationKind, SpecContentHash,
    Timestamp, WorkflowOsError, WorkflowOsErrorKind,
};

use super::continuity_codec::{
    attempt_state, corrupt, directive_state, disposition_code, encode, operation_kind,
    rejection_kind, timestamp_parts, wait_state, wake_trigger, window_state, yield_reason,
    RequestEnvelope,
};
use super::{
    map_sqlite_error, sqlite_state_error, SqliteStateBackend, CONTINUITY_CLOCK_EPOCH,
    CONTINUITY_CLOCK_PROVENANCE,
};

trait ContinuityClock: Send + Sync {
    fn observe(&self) -> Result<TrustedTimeObservation, WorkflowOsError>;
}

struct SystemContinuityClock;

impl ContinuityClock for SystemContinuityClock {
    fn observe(&self) -> Result<TrustedTimeObservation, WorkflowOsError> {
        Ok(trusted_time_observation(
            Timestamp::now_utc(),
            TrustedTimeSourceKind::CoreInjectedClockV1,
            expected_provenance()?,
            expected_epoch()?,
        ))
    }
}

#[derive(Clone)]
struct SqliteContinuityStore {
    backend: SqliteStateBackend,
    clock: Arc<dyn ContinuityClock>,
    fault: Arc<Mutex<Option<InjectedCommitFault>>>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum InjectedCommitFault {
    Before,
    During,
    After,
}

impl SqliteContinuityStore {
    fn system(backend: &SqliteStateBackend) -> Self {
        Self {
            backend: backend.clone(),
            clock: Arc::new(SystemContinuityClock),
            fault: Arc::new(Mutex::new(None)),
        }
    }

    // Keep the complete SQLite transaction protocol visible as one auditable boundary.
    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    fn transact<F>(
        &self,
        kind: AuthorizedExecutionContinuityOperationKind,
        operation_id: &ContinuityOperationId,
        request_commitment: &SpecContentHash,
        receipt_id: &ContinuityReceiptId,
        window_id: &crate::AuthorizedExecutionWindowId,
        request_envelope: &RequestEnvelope,
        mutation: F,
    ) -> Result<(CommittedOperationDisposition, bool), WorkflowOsError>
    where
        F: Fn(
            &mut ReferenceContinuityState,
            Timestamp,
        ) -> Result<RecordedOperationResult, WorkflowOsError>,
    {
        request_envelope.validate()?;
        let mut connection = self.backend.connection()?;
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|error| {
                map_sqlite_error(
                    error,
                    "write_conflict",
                    "SQLite continuity state is busy; reread before retry",
                )
            })?;
        let mut state = super::continuity_codec::load_snapshot(&transaction)?;

        if let Some(existing) = state.operations.get(operation_id) {
            if existing.operation_kind != kind
                || existing.request_commitment != *request_commitment
                || existing.receipt.receipt_id != *receipt_id
            {
                return Err(semantic_error(
                    WorkflowOsErrorKind::InvalidState,
                    "operation.replay_conflict",
                ));
            }
            validate_operation(&state, existing)?;
            return Ok((existing.disposition.clone(), true));
        }
        if state
            .operations
            .values()
            .any(|record| record.receipt.receipt_id == *receipt_id)
        {
            return Err(semantic_error(
                WorkflowOsErrorKind::InvalidState,
                "receipt.reused",
            ));
        }
        if state.trusted_time.eligibility != ContinuityInstanceEligibility::LiveStateEligible
            || state.trusted_time.posture == TrustedTimePosture::Quarantined
            || state.trusted_time.provenance_commitment != expected_provenance()?
            || state.trusted_time.epoch_id != expected_epoch()?
        {
            return Err(semantic_error(
                WorkflowOsErrorKind::Security,
                "instance.ineligible",
            ));
        }

        let window = state.windows.get(window_id).ok_or_else(corrupt)?;
        let window_expires_at = window.expires_at;
        let mut preflight = state.clone();
        mutation(&mut preflight, window.trusted_time_watermark)?;

        let observation = self.clock.observe()?;
        let observed_at = observation.observed_at();
        let trusted_time_binding = trusted_time_commitment(&observation);
        let prior_trusted_time = trusted_snapshot(&state.trusted_time);
        let prior_window = window_snapshot(window);
        let rejection = semantics::classify_security_rejection(
            semantics::SecuritySemanticSnapshot {
                trusted_time: &state.trusted_time,
                window,
            },
            &expected_provenance()?,
            &observation,
        );
        let disposition = if let Some(kind) = rejection {
            state.trusted_time.revision = state.trusted_time.revision.checked_next()?;
            match kind {
                CommittedSecurityRejectionKind::Expired => {
                    state.trusted_time.last_observed_at = Some(observed_at);
                    state.trusted_time.posture = TrustedTimePosture::Healthy;
                    let window = state.windows.get_mut(window_id).ok_or_else(corrupt)?;
                    window.state = AuthoritativeWindowState::Expired;
                    window.trusted_time_watermark = observed_at;
                    window.revision = window.revision.checked_next()?;
                }
                CommittedSecurityRejectionKind::Regressed
                | CommittedSecurityRejectionKind::Untrusted
                | CommittedSecurityRejectionKind::EpochMismatch => {
                    state.trusted_time.posture = TrustedTimePosture::Quarantined;
                    state.trusted_time.eligibility = ContinuityInstanceEligibility::Quarantined;
                }
            }
            let resulting_trusted_time = trusted_snapshot(&state.trusted_time);
            let resulting_window =
                window_snapshot(state.windows.get(window_id).ok_or_else(corrupt)?);
            let security = CommittedSecurityRejection {
                kind,
                rejection_commitment: rejection_commitment(&SecurityRejectionCommitmentInput {
                    kind,
                    observation: &observation,
                    expected_time_source: TrustedTimeSourceKind::CoreInjectedClockV1,
                    expected_provenance_commitment: &expected_provenance()?,
                    expected_epoch_id: &expected_epoch()?,
                    window_id,
                    window_expires_at,
                    prior_trusted_time: &prior_trusted_time,
                    resulting_trusted_time: &resulting_trusted_time,
                    prior_window: &prior_window,
                    resulting_window: &resulting_window,
                }),
                trusted_time: observation.clone(),
                expected_time_source: TrustedTimeSourceKind::CoreInjectedClockV1,
                expected_provenance_commitment: expected_provenance()?,
                expected_epoch_id: expected_epoch()?,
                window_id: window_id.clone(),
                window_expires_at,
                prior_trusted_time,
                resulting_trusted_time,
                prior_window,
                resulting_window,
            };
            CommittedOperationDisposition::CommittedSecurityRejection(security)
        } else {
            state.trusted_time.last_observed_at = Some(observed_at);
            state.trusted_time.posture = TrustedTimePosture::Healthy;
            state.trusted_time.revision = state.trusted_time.revision.checked_next()?;
            CommittedOperationDisposition::CommittedSuccess(mutation(&mut state, observed_at)?)
        };
        let committed = operation_commitment(
            request_commitment,
            receipt_id,
            &observation,
            &trusted_time_binding,
            &disposition,
        );
        let record = AuthoritativeOperationRecord {
            operation_id: operation_id.clone(),
            operation_kind: kind,
            request_commitment: request_commitment.clone(),
            operation_commitment: committed.clone(),
            receipt: ContinuityReceipt {
                receipt_id: receipt_id.clone(),
                operation_kind: kind,
                operation_commitment: committed,
                trusted_time_commitment: trusted_time_binding,
                committed_at: observed_at,
            },
            trusted_time: observation,
            disposition: disposition.clone(),
        };
        state
            .operations
            .insert(operation_id.clone(), record.clone());
        let fault = self.fault.lock().map_err(|_| corrupt())?.take();
        if fault == Some(InjectedCommitFault::Before) || fault == Some(InjectedCommitFault::During)
        {
            return Err(sqlite_state_error(
                "write_failed",
                "SQLite continuity transaction failed",
            ));
        }
        persist_snapshot(&transaction, &state, &record, request_envelope)?;
        transaction.commit().map_err(|_| {
            sqlite_state_error(
                "commit_ambiguous",
                "SQLite continuity commit outcome is ambiguous",
            )
        })?;
        if fault == Some(InjectedCommitFault::After) {
            return Err(sqlite_state_error(
                "commit_ambiguous",
                "SQLite continuity commit outcome is ambiguous",
            ));
        }
        Ok((disposition, false))
    }
}

impl AuthorizedExecutionContinuityStore for SqliteContinuityStore {
    #[allow(clippy::too_many_lines)]
    fn register_yield(
        &self,
        request: RegisterYieldRequest<'_>,
    ) -> Result<RegisterYieldResult, WorkflowOsError> {
        let expected = expected_register_yield_commitment(&request);
        validate_wait_count(request.waits.len())?;
        if expected != request.request_commitment {
            return Err(input_invalid());
        }
        let envelope = envelope(
            "register_yield",
            &request.operation_id,
            &request.window_id,
            Some(request.generation_id.as_str()),
            None,
            None,
            None,
            vec![
                request.receipt_id.as_str(),
                request.attempt_id.as_str(),
                request.cursor.event_id.as_str(),
            ],
        );
        let (result, replay) = self.transact(
            AuthorizedExecutionContinuityOperationKind::RegisterYield,
            &request.operation_id,
            &request.request_commitment,
            &request.receipt_id,
            &request.window_id,
            &envelope,
            |state, observed_at| {
                let window = state.windows.get(&request.window_id).ok_or_else(corrupt)?;
                validate_window(
                    window,
                    &request.expected_window_binding,
                    request.expected_window_revision,
                    &request.cursor,
                    observed_at,
                )?;
                let attempt = state
                    .attempts
                    .get(&request.attempt_id)
                    .ok_or_else(corrupt)?;
                if window.state != AuthoritativeWindowState::Executing
                    || request.attempt_capability.attempt_id != request.attempt_id
                    || request.attempt_capability.window_id != request.window_id
                    || request.attempt_capability.window_revision
                        != request.expected_window_revision
                    || request.attempt_capability.cursor != request.cursor
                    || request.attempt_capability.subject_actor_id != window.subject_actor_id
                    || request.attempt_capability.authority_commitment
                        != window.authority_commitment
                    || request.attempt_capability.window_binding_commitment
                        != window_binding_commitment(&request.expected_window_binding)
                    || attempt.state != AuthoritativeAttemptState::Started
                    || attempt.consume_operation_id
                        != request.attempt_capability.consume_operation_id
                {
                    return Err(semantic_error(
                        WorkflowOsErrorKind::Security,
                        "authority.binding_mismatch",
                    ));
                }
                if state.yields.contains_key(&request.generation_id) {
                    return Err(semantic_error(
                        WorkflowOsErrorKind::InvalidState,
                        "operation.replay_conflict",
                    ));
                }
                let mut wait_ids = Vec::with_capacity(request.waits.len());
                for seed in &request.waits {
                    let identity = AuthoritativeWaitIdentity::new(
                        seed.condition_id.clone(),
                        seed.condition_version,
                    );
                    if seed.condition_version == 0 || state.waits.contains_key(&identity) {
                        return Err(semantic_error(
                            WorkflowOsErrorKind::InvalidState,
                            "wait.identity_conflict",
                        ));
                    }
                    wait_ids.push(identity);
                }
                let directive_id = ContinuityDirectiveId::new(format!(
                    "directive/{}",
                    request.generation_id.as_str()
                ))?;
                state.yields.insert(
                    request.generation_id.clone(),
                    AuthoritativeYieldRecord {
                        generation_id: request.generation_id.clone(),
                        attempt_id: request.attempt_id.clone(),
                        cursor: request.cursor.clone(),
                        reason: request.reason,
                        wait_ids: wait_ids.clone(),
                        registered_at: observed_at,
                    },
                );
                for (identity, seed) in wait_ids.iter().zip(&request.waits) {
                    state.waits.insert(
                        identity.clone(),
                        AuthoritativeWaitRecord {
                            condition_id: seed.condition_id.clone(),
                            condition_version: seed.condition_version,
                            window_id: request.window_id.clone(),
                            generation_id: request.generation_id.clone(),
                            wake_trigger: seed.wake_trigger,
                            state: AuthoritativeWaitState::Unsatisfied,
                            source_commitment: None,
                            source_revision: None,
                            revision: ContinuityRevision::new(1)?,
                        },
                    );
                }
                state.directives.insert(
                    directive_id.clone(),
                    AuthoritativeDirectiveRecord {
                        directive_id,
                        window_id: request.window_id.clone(),
                        generation_id: request.generation_id.clone(),
                        cursor: request.cursor.clone(),
                        authority_commitment: window.authority_commitment.clone(),
                        state: AuthoritativeDirectiveState::Available,
                        revision: ContinuityRevision::new(1)?,
                    },
                );
                state
                    .attempts
                    .get_mut(&request.attempt_id)
                    .ok_or_else(corrupt)?
                    .state = AuthoritativeAttemptState::Yielded;
                let window = state
                    .windows
                    .get_mut(&request.window_id)
                    .ok_or_else(corrupt)?;
                window.state = AuthoritativeWindowState::Yielded;
                window.active_yield = Some(request.generation_id.clone());
                window.trusted_time_watermark = observed_at;
                window.revision = window.revision.checked_next()?;
                Ok(RecordedOperationResult::YieldRegistered {
                    window_id: request.window_id.clone(),
                    generation_id: request.generation_id.clone(),
                    attempt_id: request.attempt_id.clone(),
                    attempt_state: AuthoritativeAttemptState::Yielded,
                    window_state: window.state,
                    window_revision: window.revision,
                })
            },
        )?;
        Ok(if replay {
            RegisterYieldResult::ExactReplay(result)
        } else {
            match result {
                CommittedOperationDisposition::CommittedSuccess(value) => {
                    RegisterYieldResult::Registered(value)
                }
                CommittedOperationDisposition::CommittedSecurityRejection(value) => {
                    RegisterYieldResult::SecurityRejected(value)
                }
            }
        })
    }

    #[allow(clippy::too_many_lines)]
    fn transition_wait(
        &self,
        request: TransitionWaitRequest<'_>,
    ) -> Result<MutationResult, WorkflowOsError> {
        if expected_transition_wait_commitment(&request) != request.request_commitment {
            return Err(input_invalid());
        }
        let envelope = envelope(
            "transition_wait",
            &request.operation_id,
            &request.window_id,
            None,
            Some(request.condition_id.as_str()),
            Some(request.expected_condition_version),
            None,
            vec![
                request.receipt_id.as_str(),
                request.expected_generation_id.as_str(),
                request.cursor.event_id.as_str(),
            ],
        );
        let (result, replay) = self.transact(
            AuthorizedExecutionContinuityOperationKind::TransitionWait,
            &request.operation_id,
            &request.request_commitment,
            &request.receipt_id,
            &request.window_id,
            &envelope,
            |state, observed_at| {
                let window = state.windows.get(&request.window_id).ok_or_else(corrupt)?;
                validate_window(
                    window,
                    &request.expected_window_binding,
                    request.expected_window_revision,
                    &request.cursor,
                    observed_at,
                )?;
                if window.state != AuthoritativeWindowState::Yielded
                    || window.active_yield.as_ref() != Some(&request.expected_generation_id)
                {
                    return Err(semantic_error(
                        WorkflowOsErrorKind::InvalidState,
                        "window.ineligible",
                    ));
                }
                let identity = AuthoritativeWaitIdentity::new(
                    request.condition_id.clone(),
                    request.expected_condition_version,
                );
                let wait = state.waits.get(&identity).ok_or_else(corrupt)?;
                if wait.window_id != request.window_id
                    || wait.generation_id != request.expected_generation_id
                    || wait.revision != request.expected_wait_revision
                {
                    return Err(semantic_error(
                        WorkflowOsErrorKind::InvalidState,
                        "wait.revision_stale",
                    ));
                }
                match request.target {
                    AuthoritativeWaitState::Satisfied => {
                        let capability = request.wake_capability.ok_or_else(|| {
                            semantic_error(
                                WorkflowOsErrorKind::Security,
                                "wake.capability_required",
                            )
                        })?;
                        if capability.window_id != request.window_id
                            || capability.generation_id != request.expected_generation_id
                            || capability.condition_id != request.condition_id
                            || capability.condition_version != request.expected_condition_version
                            || capability.trigger != wait.wake_trigger
                            || capability.source_revision == 0
                        {
                            return Err(semantic_error(
                                WorkflowOsErrorKind::Security,
                                "wake.binding_mismatch",
                            ));
                        }
                    }
                    AuthoritativeWaitState::Expired
                    | AuthoritativeWaitState::Superseded
                    | AuthoritativeWaitState::Canceled => {}
                    AuthoritativeWaitState::Unsatisfied => return Err(input_invalid()),
                }
                if wait.state != AuthoritativeWaitState::Unsatisfied {
                    return Err(semantic_error(
                        WorkflowOsErrorKind::InvalidState,
                        "wait.already_transitioned",
                    ));
                }
                let wait = state.waits.get_mut(&identity).ok_or_else(corrupt)?;
                wait.state = request.target;
                if let Some(capability) = request.wake_capability {
                    wait.source_commitment = Some(capability.source_commitment.clone());
                    wait.source_revision = Some(capability.source_revision);
                }
                wait.revision = wait.revision.checked_next()?;
                let wait_revision = wait.revision;
                let window = state
                    .windows
                    .get_mut(&request.window_id)
                    .ok_or_else(corrupt)?;
                window.trusted_time_watermark = observed_at;
                window.revision = window.revision.checked_next()?;
                Ok(RecordedOperationResult::WaitTransitioned {
                    window_id: request.window_id.clone(),
                    generation_id: request.expected_generation_id.clone(),
                    condition_id: request.condition_id.clone(),
                    condition_version: request.expected_condition_version,
                    wait_state: request.target,
                    wait_revision,
                    window_revision: window.revision,
                })
            },
        )?;
        Ok(mutation_result(result, replay))
    }

    #[allow(clippy::too_many_lines)]
    fn consume_directive(
        &self,
        request: ConsumeDirectiveRequest,
    ) -> Result<ConsumeDirectiveResult, WorkflowOsError> {
        if expected_consume_directive_commitment(&request) != request.request_commitment {
            return Err(input_invalid());
        }
        validate_wait_count(request.expected_waits.len())?;
        let subject = request.authority_capability.subject_actor_id.clone();
        let authority = request.authority_capability.authority_commitment.clone();
        let cursor = request.cursor.clone();
        let attempt_id = request.generated_attempt_id.clone();
        let consume_operation_id = request.operation_id.clone();
        let envelope = envelope(
            "consume_directive",
            &request.operation_id,
            &request.window_id,
            None,
            None,
            None,
            Some(request.generated_attempt_id.as_str()),
            vec![
                request.receipt_id.as_str(),
                request.directive_id.as_str(),
                request.generation_id.as_str(),
                request.cursor.event_id.as_str(),
            ],
        );
        let (result, replay) = self.transact(
            AuthorizedExecutionContinuityOperationKind::ConsumeDirective,
            &request.operation_id,
            &request.request_commitment,
            &request.receipt_id,
            &request.window_id,
            &envelope,
            |state, observed_at| {
                let window = state.windows.get(&request.window_id).ok_or_else(corrupt)?;
                validate_window(
                    window,
                    &request.expected_window_binding,
                    request.expected_window_revision,
                    &request.cursor,
                    observed_at,
                )?;
                if window.state != AuthoritativeWindowState::Yielded
                    || window.active_yield.as_ref() != Some(&request.generation_id)
                {
                    return Err(semantic_error(
                        WorkflowOsErrorKind::InvalidState,
                        "window.ineligible",
                    ));
                }
                if request.authority_capability.window_id != request.window_id
                    || request.authority_capability.window_revision
                        != request.expected_window_revision
                    || request.authority_capability.generation_id != request.generation_id
                    || request.authority_capability.cursor != request.cursor
                    || request.authority_capability.subject_actor_id != window.subject_actor_id
                    || request.authority_capability.authority_commitment
                        != window.authority_commitment
                    || request.authority_capability.window_binding_commitment
                        != window_binding_commitment(&request.expected_window_binding)
                    || request.authority_capability.expected_waits != request.expected_waits
                {
                    return Err(semantic_error(
                        WorkflowOsErrorKind::Security,
                        "authority.binding_mismatch",
                    ));
                }
                let allocation = semantics::allocate_attempt(
                    window.next_attempt_number,
                    window.maximum_attempts,
                )?;
                let yielded = state
                    .yields
                    .get(&request.generation_id)
                    .ok_or_else(corrupt)?;
                let expected_ids = request
                    .expected_waits
                    .iter()
                    .map(|wait| {
                        AuthoritativeWaitIdentity::new(
                            wait.condition_id.clone(),
                            wait.condition_version,
                        )
                    })
                    .collect::<BTreeSet<_>>();
                let yielded_ids = yielded.wait_ids.iter().cloned().collect::<BTreeSet<_>>();
                let waits_match = expected_ids.len() == request.expected_waits.len()
                    && expected_ids == yielded_ids
                    && request.expected_waits.iter().all(|expected| {
                        state
                            .waits
                            .get(&AuthoritativeWaitIdentity::new(
                                expected.condition_id.clone(),
                                expected.condition_version,
                            ))
                            .is_some_and(|wait| {
                                wait.window_id == request.window_id
                                    && wait.generation_id == request.generation_id
                                    && wait.revision == expected.revision
                                    && wait.state == AuthoritativeWaitState::Satisfied
                            })
                    });
                if yielded.cursor != request.cursor || !waits_match {
                    return Err(semantic_error(
                        WorkflowOsErrorKind::InvalidState,
                        "wait.unsatisfied",
                    ));
                }
                let directive = state
                    .directives
                    .get_mut(&request.directive_id)
                    .ok_or_else(corrupt)?;
                if directive.state != AuthoritativeDirectiveState::Available
                    || directive.window_id != request.window_id
                    || directive.generation_id != request.generation_id
                    || directive.cursor != request.cursor
                    || directive.authority_commitment
                        != request.authority_capability.authority_commitment
                {
                    return Err(semantic_error(
                        WorkflowOsErrorKind::InvalidState,
                        "directive.already_consumed",
                    ));
                }
                if state.attempts.contains_key(&request.generated_attempt_id) {
                    return Err(semantic_error(
                        WorkflowOsErrorKind::InvalidState,
                        "operation.replay_conflict",
                    ));
                }
                directive.state = AuthoritativeDirectiveState::Consumed;
                directive.revision = directive.revision.checked_next()?;
                state.attempts.insert(
                    request.generated_attempt_id.clone(),
                    AuthoritativeAttemptRecord {
                        attempt_id: request.generated_attempt_id.clone(),
                        attempt_number: allocation.attempt_number,
                        window_id: request.window_id.clone(),
                        subject_actor_id: window.subject_actor_id.clone(),
                        cursor: request.cursor.clone(),
                        authority_commitment: window.authority_commitment.clone(),
                        consume_operation_id: request.operation_id.clone(),
                        state: AuthoritativeAttemptState::Started,
                        revision: ContinuityRevision::new(1)?,
                    },
                );
                let window = state
                    .windows
                    .get_mut(&request.window_id)
                    .ok_or_else(corrupt)?;
                window.state = AuthoritativeWindowState::Executing;
                window.active_yield = None;
                window.next_attempt_number = allocation.next_attempt_number;
                window.trusted_time_watermark = observed_at;
                window.revision = window.revision.checked_next()?;
                Ok(RecordedOperationResult::DirectiveConsumed {
                    window_id: request.window_id.clone(),
                    directive_id: request.directive_id.clone(),
                    generation_id: request.generation_id.clone(),
                    attempt_id: request.generated_attempt_id.clone(),
                    attempt_number: allocation.attempt_number,
                    directive_state: AuthoritativeDirectiveState::Consumed,
                    attempt_state: AuthoritativeAttemptState::Started,
                    window_state: window.state,
                    window_revision: window.revision,
                })
            },
        )?;
        if replay {
            return Ok(ConsumeDirectiveResult::ExactReplay(result));
        }
        match result {
            CommittedOperationDisposition::CommittedSecurityRejection(value) => {
                Ok(ConsumeDirectiveResult::SecurityRejected(value))
            }
            CommittedOperationDisposition::CommittedSuccess(result) => {
                let RecordedOperationResult::DirectiveConsumed {
                    window_revision, ..
                } = result
                else {
                    return Err(corrupt());
                };
                Ok(ConsumeDirectiveResult::Consumed {
                    result,
                    capability: AttemptUseCapability {
                        attempt_id,
                        subject_actor_id: subject,
                        window_id: request.window_id,
                        window_revision,
                        cursor,
                        authority_commitment: authority,
                        window_binding_commitment: window_binding_commitment(
                            &request.expected_window_binding,
                        ),
                        consume_operation_id,
                    },
                })
            }
        }
    }

    fn record_attempt_outcome(
        &self,
        request: RecordAttemptOutcomeRequest<'_>,
    ) -> Result<MutationResult, WorkflowOsError> {
        if expected_attempt_outcome_commitment(&request) != request.request_commitment
            || request.outcome == AuthorizedExecutionAttemptOutcome::AmbiguousMayHaveStarted
        {
            return Err(input_invalid());
        }
        let envelope = envelope(
            "record_attempt_outcome",
            &request.operation_id,
            &request.window_id,
            None,
            None,
            None,
            Some(request.attempt_id.as_str()),
            vec![request.receipt_id.as_str(), request.attempt_id.as_str()],
        );
        let (result, replay) = self.transact(
            AuthorizedExecutionContinuityOperationKind::RecordAttemptOutcome,
            &request.operation_id,
            &request.request_commitment,
            &request.receipt_id,
            &request.window_id,
            &envelope,
            |state, observed_at| {
                validate_attempt_request(
                    state,
                    &request.window_id,
                    request.expected_window_revision,
                    &request.expected_window_binding,
                    &request.attempt_id,
                    request.expected_attempt_revision,
                    request.attempt_capability,
                    observed_at,
                )?;
                let (attempts, windows) = (&mut state.attempts, &mut state.windows);
                let attempt = attempts.get_mut(&request.attempt_id).ok_or_else(corrupt)?;
                let window = windows.get_mut(&request.window_id).ok_or_else(corrupt)?;
                let attempt_state = semantics::apply_attempt_outcome(
                    attempt,
                    window,
                    request.outcome,
                    observed_at,
                )?;
                Ok(RecordedOperationResult::AttemptOutcomeRecorded {
                    window_id: request.window_id.clone(),
                    attempt_id: request.attempt_id.clone(),
                    attempt_state,
                    window_state: window.state,
                    window_revision: window.revision,
                })
            },
        )?;
        Ok(mutation_result(result, replay))
    }

    fn recover_ambiguous_attempt(
        &self,
        request: RecoverAmbiguousAttemptRequest,
    ) -> Result<MutationResult, WorkflowOsError> {
        if expected_recovery_commitment(&request) != request.request_commitment {
            return Err(input_invalid());
        }
        let envelope = envelope(
            "recover_ambiguous_attempt",
            &request.operation_id,
            &request.window_id,
            None,
            None,
            None,
            Some(request.attempt_id.as_str()),
            vec![
                request.receipt_id.as_str(),
                request.attempt_id.as_str(),
                request.cursor.event_id.as_str(),
            ],
        );
        let (result, replay) = self.transact(
            AuthorizedExecutionContinuityOperationKind::RecoverAmbiguousAttempt,
            &request.operation_id,
            &request.request_commitment,
            &request.receipt_id,
            &request.window_id,
            &envelope,
            |state, observed_at| {
                let window = state.windows.get(&request.window_id).ok_or_else(corrupt)?;
                validate_window(
                    window,
                    &request.expected_window_binding,
                    request.expected_window_revision,
                    &request.cursor,
                    observed_at,
                )?;
                let attempt = state
                    .attempts
                    .get(&request.attempt_id)
                    .ok_or_else(corrupt)?;
                if window.state != AuthoritativeWindowState::Executing
                    || attempt.state != AuthoritativeAttemptState::Started
                    || attempt.revision != request.expected_attempt_revision
                    || attempt.window_id != request.window_id
                    || attempt.subject_actor_id != window.subject_actor_id
                    || attempt.cursor != request.cursor
                    || attempt.authority_commitment != window.authority_commitment
                {
                    return Err(semantic_error(
                        WorkflowOsErrorKind::InvalidState,
                        "attempt.outcome_already_recorded",
                    ));
                }
                let (attempts, windows) = (&mut state.attempts, &mut state.windows);
                let attempt = attempts.get_mut(&request.attempt_id).ok_or_else(corrupt)?;
                let window = windows.get_mut(&request.window_id).ok_or_else(corrupt)?;
                semantics::apply_ambiguity_recovery(attempt, window, observed_at)?;
                Ok(RecordedOperationResult::AttemptOutcomeRecorded {
                    window_id: request.window_id.clone(),
                    attempt_id: request.attempt_id.clone(),
                    attempt_state: AuthoritativeAttemptState::AmbiguousMayHaveStarted,
                    window_state: window.state,
                    window_revision: window.revision,
                })
            },
        )?;
        Ok(mutation_result(result, replay))
    }

    fn continuation_disposition(
        &self,
        window_id: &crate::AuthorizedExecutionWindowId,
    ) -> Result<AuthoritativeContinuationDisposition, WorkflowOsError> {
        let connection = self.backend.connection()?;
        let state = super::continuity_codec::load_snapshot(&connection)?;
        let window = state.windows.get(window_id).ok_or_else(corrupt)?;
        let active_wait_ids = window
            .active_yield
            .as_ref()
            .map(|id| state.yields.get(id).ok_or_else(corrupt))
            .transpose()?
            .map(|record| record.wait_ids.as_slice());
        let observed_at = self
            .clock
            .observe()
            .ok()
            .filter(|observation| {
                observation.source() == state.trusted_time.source
                    && observation.provenance_commitment()
                        == &state.trusted_time.provenance_commitment
                    && observation.epoch_id() == &state.trusted_time.epoch_id
            })
            .map(|observation| observation.observed_at());
        semantics::continuation_disposition(
            &state.trusted_time,
            window,
            &state.waits,
            active_wait_ids,
            observed_at,
        )
    }
}

impl AuthorizedExecutionContinuityReconciler for SqliteContinuityStore {
    fn reconcile_operation(
        &self,
        request: &ReconcileOperationRequest,
    ) -> ContinuityReconciliationResult {
        let Ok(connection) = self.backend.connection() else {
            return ContinuityReconciliationResult::StateUnreadable;
        };
        let Ok(state) = super::continuity_codec::load_snapshot(&connection) else {
            return ContinuityReconciliationResult::StateUnreadable;
        };
        if let Some(record) = state.operations.get(&request.operation_id) {
            if record.request_commitment != request.expected_request_commitment
                || record.receipt.receipt_id != request.expected_receipt_id
                || validate_operation(&state, record).is_err()
            {
                return ContinuityReconciliationResult::StateUnreadable;
            }
            return ContinuityReconciliationResult::DurablyCommitted(Box::new(
                record.disposition.clone(),
            ));
        }
        if state
            .operations
            .values()
            .any(|record| record.receipt.receipt_id == request.expected_receipt_id)
        {
            ContinuityReconciliationResult::StateUnreadable
        } else {
            ContinuityReconciliationResult::ConfirmedAbsent
        }
    }
}

impl AuthorizedExecutionContinuityEligibilityReader for SqliteContinuityStore {
    fn continuity_instance_eligibility(
        &self,
    ) -> Result<ContinuityInstanceEligibility, WorkflowOsError> {
        Ok(
            super::continuity_codec::load_snapshot(&self.backend.connection()?)?
                .trusted_time
                .eligibility,
        )
    }
}

impl AuthorizedExecutionContinuityStore for SqliteStateBackend {
    fn register_yield(
        &self,
        request: RegisterYieldRequest<'_>,
    ) -> Result<RegisterYieldResult, WorkflowOsError> {
        SqliteContinuityStore::system(self).register_yield(request)
    }
    fn transition_wait(
        &self,
        request: TransitionWaitRequest<'_>,
    ) -> Result<MutationResult, WorkflowOsError> {
        SqliteContinuityStore::system(self).transition_wait(request)
    }
    fn consume_directive(
        &self,
        request: ConsumeDirectiveRequest,
    ) -> Result<ConsumeDirectiveResult, WorkflowOsError> {
        SqliteContinuityStore::system(self).consume_directive(request)
    }
    fn record_attempt_outcome(
        &self,
        request: RecordAttemptOutcomeRequest<'_>,
    ) -> Result<MutationResult, WorkflowOsError> {
        SqliteContinuityStore::system(self).record_attempt_outcome(request)
    }
    fn recover_ambiguous_attempt(
        &self,
        request: RecoverAmbiguousAttemptRequest,
    ) -> Result<MutationResult, WorkflowOsError> {
        SqliteContinuityStore::system(self).recover_ambiguous_attempt(request)
    }
    fn continuation_disposition(
        &self,
        window_id: &crate::AuthorizedExecutionWindowId,
    ) -> Result<AuthoritativeContinuationDisposition, WorkflowOsError> {
        SqliteContinuityStore::system(self).continuation_disposition(window_id)
    }
}

impl AuthorizedExecutionContinuityReconciler for SqliteStateBackend {
    fn reconcile_operation(
        &self,
        request: &ReconcileOperationRequest,
    ) -> ContinuityReconciliationResult {
        SqliteContinuityStore::system(self).reconcile_operation(request)
    }
}

impl AuthorizedExecutionContinuityEligibilityReader for SqliteStateBackend {
    fn continuity_instance_eligibility(
        &self,
    ) -> Result<ContinuityInstanceEligibility, WorkflowOsError> {
        SqliteContinuityStore::system(self).continuity_instance_eligibility()
    }
}

fn validate_window(
    window: &AuthoritativeWindowRecord,
    binding: &ExpectedWindowBinding,
    revision: ContinuityRevision,
    cursor: &ContinuityCursor,
    observed_at: Timestamp,
) -> Result<(), WorkflowOsError> {
    semantics::validate_window(
        window,
        binding,
        revision,
        cursor,
        &expected_epoch()?,
        observed_at,
    )
}

#[allow(clippy::too_many_arguments)]
fn validate_attempt_request(
    state: &ReferenceContinuityState,
    window_id: &crate::AuthorizedExecutionWindowId,
    window_revision: ContinuityRevision,
    binding: &ExpectedWindowBinding,
    attempt_id: &crate::AuthorizedExecutionAttemptId,
    attempt_revision: ContinuityRevision,
    capability: &AttemptUseCapability,
    observed_at: Timestamp,
) -> Result<(), WorkflowOsError> {
    let window = state.windows.get(window_id).ok_or_else(corrupt)?;
    validate_window(
        window,
        binding,
        window_revision,
        &capability.cursor,
        observed_at,
    )?;
    let attempt = state.attempts.get(attempt_id).ok_or_else(corrupt)?;
    if window.state != AuthoritativeWindowState::Executing
        || capability.window_id != *window_id
        || capability.window_revision != window_revision
        || capability.attempt_id != *attempt_id
        || capability.subject_actor_id != window.subject_actor_id
        || capability.authority_commitment != window.authority_commitment
        || capability.window_binding_commitment != window_binding_commitment(binding)
        || attempt.state != AuthoritativeAttemptState::Started
        || attempt.revision != attempt_revision
        || attempt.window_id != *window_id
        || attempt.subject_actor_id != window.subject_actor_id
        || attempt.cursor != capability.cursor
        || attempt.authority_commitment != window.authority_commitment
        || attempt.consume_operation_id != capability.consume_operation_id
    {
        return Err(semantic_error(
            WorkflowOsErrorKind::InvalidState,
            "attempt.outcome_already_recorded",
        ));
    }
    Ok(())
}

fn persist_snapshot(
    transaction: &Transaction<'_>,
    state: &ReferenceContinuityState,
    new_operation: &AuthoritativeOperationRecord,
    request: &RequestEnvelope,
) -> Result<(), WorkflowOsError> {
    persist_trusted_time(transaction, &state.trusted_time)?;
    for record in state.windows.values() {
        persist_window(transaction, record)?;
    }
    for record in state.attempts.values() {
        persist_attempt(transaction, record)?;
    }
    for record in state.yields.values() {
        persist_yield(transaction, record, state)?;
    }
    for record in state.waits.values() {
        persist_wait(transaction, record)?;
    }
    for record in state.directives.values() {
        persist_directive(transaction, record)?;
    }
    persist_operation(transaction, new_operation, request)
}

fn persist_trusted_time(
    connection: &Connection,
    record: &TrustedTimeSecurityRecord,
) -> Result<(), WorkflowOsError> {
    let (seconds, nanos) = record
        .last_observed_at
        .map(timestamp_parts)
        .map_or((None, None), |(s, n)| (Some(s), Some(n)));
    connection.execute("UPDATE continuity_trusted_time SET source_kind='core_injected_clock_v1', provenance_commitment=?1, epoch_id=?2, observed_seconds=?3, observed_nanos=?4, posture=?5, eligibility=?6, revision=?7 WHERE singleton_id=1", params![record.provenance_commitment.as_str(), record.epoch_id.as_str(), seconds, nanos, trusted_posture(record.posture), eligibility(record.eligibility), to_i64(record.revision.get())?]).map_err(|_| corrupt())?;
    Ok(())
}

fn persist_window(
    connection: &Connection,
    record: &AuthoritativeWindowRecord,
) -> Result<(), WorkflowOsError> {
    let (es, en) = timestamp_parts(record.expires_at);
    let (ws, wn) = timestamp_parts(record.trusted_time_watermark);
    let binding = window_binding_commitment(&ExpectedWindowBinding {
        workflow_id: record.workflow_id.clone(),
        run_id: record.run_id.clone(),
        step_id: record.step_id.clone(),
        subject_actor_id: record.subject_actor_id.clone(),
        immutable_run_bundle: record.immutable_run_bundle.clone(),
        governance_commitment: record.governance_commitment.clone(),
        authority_commitment: record.authority_commitment.clone(),
        cursor: record.cursor.clone(),
    });
    connection.execute("INSERT INTO continuity_windows (window_id,workflow_id,run_id,step_id,window_binding_commitment,subject_actor_id,immutable_bundle_commitment,governance_commitment,authority_commitment,cursor_sequence,cursor_event_id,state,maximum_attempts,next_attempt_number,expires_seconds,expires_nanos,watermark_seconds,watermark_nanos,trusted_time_epoch_id,revision,active_yield_generation_id,record_json) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20,?21,?22) ON CONFLICT(window_id) DO UPDATE SET state=excluded.state,next_attempt_number=excluded.next_attempt_number,watermark_seconds=excluded.watermark_seconds,watermark_nanos=excluded.watermark_nanos,revision=excluded.revision,active_yield_generation_id=excluded.active_yield_generation_id,record_json=excluded.record_json", params![record.window_id.as_str(),record.workflow_id.as_str(),record.run_id.as_str(),record.step_id.as_str(),binding.as_str(),record.subject_actor_id.as_str(),record.immutable_run_bundle.root_hash().as_str(),record.governance_commitment.as_str(),record.authority_commitment.as_str(),to_i64(record.cursor.sequence_number.get())?,record.cursor.event_id.as_str(),window_state(record.state),i64::from(record.maximum_attempts),i64::from(record.next_attempt_number),es,en,ws,wn,record.trusted_time_epoch_id.as_str(),to_i64(record.revision.get())?,record.active_yield.as_ref().map(ContinuityYieldGenerationId::as_str),encode(record)?]).map_err(|_| corrupt())?;
    Ok(())
}

fn persist_attempt(
    connection: &Connection,
    record: &AuthoritativeAttemptRecord,
) -> Result<(), WorkflowOsError> {
    connection.execute("INSERT INTO continuity_attempts (attempt_id,window_id,attempt_number,subject_actor_id,cursor_sequence,cursor_event_id,authority_commitment,consume_operation_id,state,revision,record_json) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11) ON CONFLICT(attempt_id) DO UPDATE SET state=excluded.state,revision=excluded.revision,record_json=excluded.record_json",params![record.attempt_id.as_str(),record.window_id.as_str(),i64::from(record.attempt_number),record.subject_actor_id.as_str(),to_i64(record.cursor.sequence_number.get())?,record.cursor.event_id.as_str(),record.authority_commitment.as_str(),record.consume_operation_id.as_str(),attempt_state(record.state),to_i64(record.revision.get())?,encode(record)?]).map_err(|_|corrupt())?;
    Ok(())
}
fn persist_yield(
    connection: &Connection,
    record: &AuthoritativeYieldRecord,
    state: &ReferenceContinuityState,
) -> Result<(), WorkflowOsError> {
    let window_id = state
        .attempts
        .get(&record.attempt_id)
        .ok_or_else(corrupt)?
        .window_id
        .as_str();
    let (s, n) = timestamp_parts(record.registered_at);
    connection.execute("INSERT INTO continuity_yields (generation_id,window_id,attempt_id,cursor_sequence,cursor_event_id,reason,registered_seconds,registered_nanos,record_json) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9) ON CONFLICT(generation_id) DO UPDATE SET record_json=excluded.record_json",params![record.generation_id.as_str(),window_id,record.attempt_id.as_str(),to_i64(record.cursor.sequence_number.get())?,record.cursor.event_id.as_str(),yield_reason(record.reason),s,n,encode(record)?]).map_err(|_|corrupt())?;
    Ok(())
}
fn persist_wait(
    connection: &Connection,
    record: &AuthoritativeWaitRecord,
) -> Result<(), WorkflowOsError> {
    connection.execute("INSERT INTO continuity_waits (condition_id,condition_version,window_id,generation_id,wake_trigger,state,source_commitment,source_revision,revision,record_json) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10) ON CONFLICT(condition_id,condition_version) DO UPDATE SET state=excluded.state,source_commitment=excluded.source_commitment,source_revision=excluded.source_revision,revision=excluded.revision,record_json=excluded.record_json",params![record.condition_id.as_str(),i64::from(record.condition_version),record.window_id.as_str(),record.generation_id.as_str(),wake_trigger(record.wake_trigger),wait_state(record.state),record.source_commitment.as_ref().map(SpecContentHash::as_str),record.source_revision.map(to_i64).transpose()?,to_i64(record.revision.get())?,encode(record)?]).map_err(|_|corrupt())?;
    Ok(())
}
fn persist_directive(
    connection: &Connection,
    record: &AuthoritativeDirectiveRecord,
) -> Result<(), WorkflowOsError> {
    connection.execute("INSERT INTO continuity_directives (directive_id,window_id,generation_id,cursor_sequence,cursor_event_id,authority_commitment,state,revision,record_json) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9) ON CONFLICT(directive_id) DO UPDATE SET state=excluded.state,revision=excluded.revision,record_json=excluded.record_json",params![record.directive_id.as_str(),record.window_id.as_str(),record.generation_id.as_str(),to_i64(record.cursor.sequence_number.get())?,record.cursor.event_id.as_str(),record.authority_commitment.as_str(),directive_state(record.state),to_i64(record.revision.get())?,encode(record)?]).map_err(|_|corrupt())?;
    Ok(())
}

fn persist_operation(
    connection: &Connection,
    record: &AuthoritativeOperationRecord,
    request: &RequestEnvelope,
) -> Result<(), WorkflowOsError> {
    let (s, n) = timestamp_parts(record.trusted_time.observed_at());
    let trusted = trusted_time_commitment(&record.trusted_time);
    let payload = encode(record)?;
    let (
        sy,
        sw,
        swv,
        sa,
        sco,
        result_commitment_value,
        rejection_commitment_value,
        rejection_kind_value,
        result_json,
        rejection_json,
    ) = match &record.disposition {
        CommittedOperationDisposition::CommittedSuccess(result) => {
            let (sy, sw, swv, sa, sco) = match result {
                RecordedOperationResult::YieldRegistered { generation_id, .. } => {
                    (Some(generation_id.as_str()), None, None, None, None)
                }
                RecordedOperationResult::WaitTransitioned {
                    condition_id,
                    condition_version,
                    ..
                } => (
                    None,
                    Some(condition_id.as_str()),
                    Some(i64::from(*condition_version)),
                    None,
                    None,
                ),
                RecordedOperationResult::DirectiveConsumed { attempt_id, .. } => (
                    None,
                    None,
                    None,
                    Some(attempt_id.as_str()),
                    Some(record.operation_id.as_str()),
                ),
                RecordedOperationResult::AttemptOutcomeRecorded { attempt_id, .. } => {
                    (None, None, None, Some(attempt_id.as_str()), None)
                }
            };
            (
                sy,
                sw,
                swv,
                sa,
                sco,
                Some(result_commitment(result)),
                None,
                None,
                Some(payload),
                None,
            )
        }
        CommittedOperationDisposition::CommittedSecurityRejection(rejection) => (
            None,
            None,
            None,
            None,
            None,
            None,
            Some(rejection.rejection_commitment.clone()),
            Some(rejection_kind(rejection.kind)),
            None,
            Some(payload),
        ),
    };
    connection.execute("INSERT INTO continuity_operations (operation_id,receipt_id,operation_kind,request_commitment,request_json,operation_commitment,disposition,request_window_id,request_yield_generation_id,request_wait_condition_id,request_wait_condition_version,request_attempt_id,success_yield_generation_id,success_wait_condition_id,success_wait_condition_version,success_attempt_id,success_consume_operation_id,result_commitment,rejection_commitment,rejection_kind,trusted_time_source_kind,trusted_time_provenance_commitment,trusted_time_epoch_id,observed_seconds,observed_nanos,trusted_time_commitment,result_json,rejection_json,committed_seconds,committed_nanos) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20,'core_injected_clock_v1',?21,?22,?23,?24,?25,?26,?27,?23,?24)",params![record.operation_id.as_str(),record.receipt.receipt_id.as_str(),operation_kind(record.operation_kind),record.request_commitment.as_str(),encode(request)?,record.operation_commitment.as_str(),disposition_code(&record.disposition),request.window_id,request.yield_generation_id,request.wait_condition_id,request.wait_condition_version.map(i64::from),request.attempt_id,sy,sw,swv,sa,sco,result_commitment_value.as_ref().map(SpecContentHash::as_str),rejection_commitment_value.as_ref().map(SpecContentHash::as_str),rejection_kind_value,record.trusted_time.provenance_commitment().as_str(),record.trusted_time.epoch_id().as_str(),s,n,trusted.as_str(),result_json,rejection_json]).map_err(|_|corrupt())?;
    Ok(())
}

fn validate_operation(
    state: &ReferenceContinuityState,
    record: &AuthoritativeOperationRecord,
) -> Result<(), WorkflowOsError> {
    let trusted = trusted_time_commitment(&record.trusted_time);
    if record.receipt.trusted_time_commitment != trusted
        || record.receipt.committed_at != record.trusted_time.observed_at()
        || record.receipt.operation_kind != record.operation_kind
        || record.operation_commitment
            != operation_commitment(
                &record.request_commitment,
                &record.receipt.receipt_id,
                &record.trusted_time,
                &trusted,
                &record.disposition,
            )
        || record.receipt.operation_commitment != record.operation_commitment
    {
        return Err(corrupt());
    }
    match &record.disposition {
        CommittedOperationDisposition::CommittedSuccess(result) => {
            let valid = match result {
                RecordedOperationResult::YieldRegistered {
                    window_id,
                    generation_id,
                    attempt_id,
                    ..
                } => {
                    state.windows.contains_key(window_id)
                        && state
                            .yields
                            .get(generation_id)
                            .is_some_and(|value| value.attempt_id == *attempt_id)
                }
                RecordedOperationResult::WaitTransitioned {
                    window_id,
                    condition_id,
                    condition_version,
                    ..
                } => state
                    .waits
                    .get(&AuthoritativeWaitIdentity::new(
                        condition_id.clone(),
                        *condition_version,
                    ))
                    .is_some_and(|value| value.window_id == *window_id),
                RecordedOperationResult::DirectiveConsumed {
                    window_id,
                    attempt_id,
                    ..
                }
                | RecordedOperationResult::AttemptOutcomeRecorded {
                    window_id,
                    attempt_id,
                    ..
                } => state
                    .attempts
                    .get(attempt_id)
                    .is_some_and(|value| value.window_id == *window_id),
            };
            if !valid {
                return Err(corrupt());
            }
        }
        CommittedOperationDisposition::CommittedSecurityRejection(rejection) => {
            let expected = rejection_commitment(&SecurityRejectionCommitmentInput {
                kind: rejection.kind,
                observation: &rejection.trusted_time,
                expected_time_source: rejection.expected_time_source,
                expected_provenance_commitment: &rejection.expected_provenance_commitment,
                expected_epoch_id: &rejection.expected_epoch_id,
                window_id: &rejection.window_id,
                window_expires_at: rejection.window_expires_at,
                prior_trusted_time: &rejection.prior_trusted_time,
                resulting_trusted_time: &rejection.resulting_trusted_time,
                prior_window: &rejection.prior_window,
                resulting_window: &rejection.resulting_window,
            });
            if rejection.rejection_commitment != expected
                || rejection.trusted_time != record.trusted_time
            {
                return Err(corrupt());
            }
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn envelope(
    domain: &str,
    operation_id: &ContinuityOperationId,
    window_id: &crate::AuthorizedExecutionWindowId,
    yield_id: Option<&str>,
    wait_id: Option<&str>,
    wait_version: Option<u32>,
    attempt_id: Option<&str>,
    fields: Vec<&str>,
) -> RequestEnvelope {
    RequestEnvelope {
        version: 1,
        domain: format!("workflow-os/authorized-execution-continuity/{domain}/v1"),
        fields: std::iter::once(operation_id.as_str().to_owned())
            .chain(fields.into_iter().map(str::to_owned))
            .collect(),
        window_id: window_id.as_str().to_owned(),
        yield_generation_id: yield_id.map(str::to_owned),
        wait_condition_id: wait_id.map(str::to_owned),
        wait_condition_version: wait_version,
        attempt_id: attempt_id.map(str::to_owned),
    }
}
fn mutation_result(value: CommittedOperationDisposition, replay: bool) -> MutationResult {
    if replay {
        MutationResult::ExactReplay(value)
    } else {
        match value {
            CommittedOperationDisposition::CommittedSuccess(value) => {
                MutationResult::Recorded(value)
            }
            CommittedOperationDisposition::CommittedSecurityRejection(value) => {
                MutationResult::SecurityRejected(value)
            }
        }
    }
}
fn trusted_snapshot(value: &TrustedTimeSecurityRecord) -> TrustedTimeSecuritySnapshot {
    TrustedTimeSecuritySnapshot {
        last_observed_at: value.last_observed_at,
        posture: value.posture,
        eligibility: value.eligibility,
        revision: value.revision,
    }
}
fn window_snapshot(value: &AuthoritativeWindowRecord) -> WindowSecuritySnapshot {
    WindowSecuritySnapshot {
        state: value.state,
        trusted_time_watermark: value.trusted_time_watermark,
        revision: value.revision,
    }
}
fn expected_provenance() -> Result<SpecContentHash, WorkflowOsError> {
    SpecContentHash::new(CONTINUITY_CLOCK_PROVENANCE)
}
fn expected_epoch() -> Result<ContinuityTrustedTimeEpochId, WorkflowOsError> {
    ContinuityTrustedTimeEpochId::new(CONTINUITY_CLOCK_EPOCH)
}
fn to_i64(value: u64) -> Result<i64, WorkflowOsError> {
    i64::try_from(value).map_err(|_| corrupt())
}
fn trusted_posture(value: TrustedTimePosture) -> &'static str {
    match value {
        TrustedTimePosture::Unobserved => "unobserved",
        TrustedTimePosture::Healthy => "healthy",
        TrustedTimePosture::Quarantined => "quarantined",
    }
}
fn eligibility(value: ContinuityInstanceEligibility) -> &'static str {
    match value {
        ContinuityInstanceEligibility::LiveStateEligible => "live_state_eligible",
        ContinuityInstanceEligibility::RestoreUnverified => "restore_unverified",
        ContinuityInstanceEligibility::Quarantined => "quarantined",
    }
}
fn semantic_error(kind: WorkflowOsErrorKind, suffix: &'static str) -> WorkflowOsError {
    continuity_state_error(
        kind,
        suffix,
        "authorized execution continuity state operation failed",
    )
}
fn input_invalid() -> WorkflowOsError {
    semantic_error(WorkflowOsErrorKind::Validation, "input.invalid")
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod conformance_backend {
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::Mutex;
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::authorized_execution_continuity_state::conformance::{
        ContinuityConformanceBackend, ContinuityConformanceFault, ContinuityConformanceFixture,
    };

    use super::*;

    struct TestClockState {
        observed_at: Timestamp,
        provenance: SpecContentHash,
        epoch: ContinuityTrustedTimeEpochId,
        available: bool,
    }

    struct TestClock(Mutex<TestClockState>);

    impl ContinuityClock for TestClock {
        fn observe(&self) -> Result<TrustedTimeObservation, WorkflowOsError> {
            let state = self.0.lock().map_err(|_| corrupt())?;
            if !state.available {
                return Err(semantic_error(
                    WorkflowOsErrorKind::InvalidState,
                    "time.unavailable",
                ));
            }
            Ok(trusted_time_observation(
                state.observed_at,
                TrustedTimeSourceKind::CoreInjectedClockV1,
                state.provenance.clone(),
                state.epoch.clone(),
            ))
        }
    }

    #[derive(Clone)]
    pub(super) struct SqliteConformanceBackend {
        store: SqliteContinuityStore,
        clock: Arc<TestClock>,
    }

    impl SqliteConformanceBackend {
        fn seeded(state: &ReferenceContinuityState) -> Self {
            let backend =
                SqliteStateBackend::open(unique_path()).expect("SQLite conformance backend");
            let mut connection = backend.connection().expect("SQLite connection");
            let transaction = connection
                .transaction_with_behavior(TransactionBehavior::Immediate)
                .expect("SQLite seed transaction");
            persist_seed(&transaction, state).expect("SQLite seed state");
            transaction.commit().expect("SQLite seed commit");
            Self::from_backend(backend)
        }

        fn reopen_path(path: PathBuf) -> Self {
            Self::from_backend(
                SqliteStateBackend::open(path).expect("reopen SQLite conformance backend"),
            )
        }

        fn from_backend(backend: SqliteStateBackend) -> Self {
            let clock = Arc::new(TestClock(Mutex::new(TestClockState {
                observed_at: Timestamp::parse_rfc3339("2026-08-15T12:01:00Z").expect("time"),
                provenance: expected_provenance().expect("provenance"),
                epoch: expected_epoch().expect("epoch"),
                available: true,
            })));
            Self {
                store: SqliteContinuityStore {
                    backend,
                    clock: clock.clone(),
                    fault: Arc::new(Mutex::new(None)),
                },
                clock,
            }
        }
    }

    impl AuthorizedExecutionContinuityStore for SqliteConformanceBackend {
        fn register_yield(
            &self,
            request: RegisterYieldRequest<'_>,
        ) -> Result<RegisterYieldResult, WorkflowOsError> {
            self.store.register_yield(request)
        }
        fn transition_wait(
            &self,
            request: TransitionWaitRequest<'_>,
        ) -> Result<MutationResult, WorkflowOsError> {
            self.store.transition_wait(request)
        }
        fn consume_directive(
            &self,
            request: ConsumeDirectiveRequest,
        ) -> Result<ConsumeDirectiveResult, WorkflowOsError> {
            self.store.consume_directive(request)
        }
        fn record_attempt_outcome(
            &self,
            request: RecordAttemptOutcomeRequest<'_>,
        ) -> Result<MutationResult, WorkflowOsError> {
            self.store.record_attempt_outcome(request)
        }
        fn recover_ambiguous_attempt(
            &self,
            request: RecoverAmbiguousAttemptRequest,
        ) -> Result<MutationResult, WorkflowOsError> {
            self.store.recover_ambiguous_attempt(request)
        }
        fn continuation_disposition(
            &self,
            window_id: &crate::AuthorizedExecutionWindowId,
        ) -> Result<AuthoritativeContinuationDisposition, WorkflowOsError> {
            self.store.continuation_disposition(window_id)
        }
    }
    impl AuthorizedExecutionContinuityReconciler for SqliteConformanceBackend {
        fn reconcile_operation(
            &self,
            request: &ReconcileOperationRequest,
        ) -> ContinuityReconciliationResult {
            self.store.reconcile_operation(request)
        }
    }
    impl AuthorizedExecutionContinuityEligibilityReader for SqliteConformanceBackend {
        fn continuity_instance_eligibility(
            &self,
        ) -> Result<ContinuityInstanceEligibility, WorkflowOsError> {
            self.store.continuity_instance_eligibility()
        }
    }

    impl ContinuityConformanceBackend for SqliteConformanceBackend {
        fn conformance_clock_provenance() -> SpecContentHash {
            expected_provenance().expect("provenance")
        }

        fn conformance_clock_epoch() -> ContinuityTrustedTimeEpochId {
            expected_epoch().expect("epoch")
        }

        fn conformance_snapshot(&self) -> ReferenceContinuityState {
            super::super::continuity_codec::load_snapshot(
                &self.store.backend.connection().expect("connection"),
            )
            .expect("snapshot")
        }
        fn conformance_reopen(state: ReferenceContinuityState) -> Self {
            Self::seeded(&state)
        }
        fn conformance_reopen_current(&self) -> Self {
            Self::reopen_path(self.store.backend.database_path.clone())
        }
        fn conformance_set_time(&self, observed_at: Timestamp) {
            self.clock.0.lock().expect("clock").observed_at = observed_at;
        }
        fn conformance_set_time_available(&self, available: bool) {
            self.clock.0.lock().expect("clock").available = available;
        }
        fn conformance_set_time_provenance(&self, provenance: SpecContentHash) {
            self.clock.0.lock().expect("clock").provenance = provenance;
        }
        fn conformance_set_time_epoch(&self, epoch_id: ContinuityTrustedTimeEpochId) {
            self.clock.0.lock().expect("clock").epoch = epoch_id;
        }
        fn conformance_inject_fault(&self, fault: ContinuityConformanceFault) {
            *self.store.fault.lock().expect("fault") = Some(match fault {
                ContinuityConformanceFault::Before => InjectedCommitFault::Before,
                ContinuityConformanceFault::During => InjectedCommitFault::During,
                ContinuityConformanceFault::After => InjectedCommitFault::After,
            });
        }
    }

    crate::authorized_execution_continuity_state::conformance::instantiate_continuity_conformance_tests!(
        SqliteConformanceBackend
    );

    fn persist_seed(
        transaction: &Transaction<'_>,
        state: &ReferenceContinuityState,
    ) -> Result<(), WorkflowOsError> {
        persist_trusted_time(transaction, &state.trusted_time)?;
        for record in state.windows.values() {
            persist_window(transaction, record)?;
        }
        for record in state.attempts.values() {
            persist_attempt(transaction, record)?;
        }
        for record in state.yields.values() {
            persist_yield(transaction, record, state)?;
        }
        for record in state.waits.values() {
            persist_wait(transaction, record)?;
        }
        for record in state.directives.values() {
            persist_directive(transaction, record)?;
        }
        for record in state.operations.values() {
            let request = seed_envelope(record);
            persist_operation(transaction, record, &request)?;
        }
        Ok(())
    }

    fn seed_envelope(record: &AuthoritativeOperationRecord) -> RequestEnvelope {
        let (window, yield_id, wait_id, wait_version, attempt_id) = match &record.disposition {
            CommittedOperationDisposition::CommittedSuccess(result) => match result {
                RecordedOperationResult::YieldRegistered {
                    window_id,
                    generation_id,
                    ..
                } => (
                    window_id.as_str(),
                    Some(generation_id.as_str()),
                    None,
                    None,
                    None,
                ),
                RecordedOperationResult::WaitTransitioned {
                    window_id,
                    condition_id,
                    condition_version,
                    ..
                } => (
                    window_id.as_str(),
                    None,
                    Some(condition_id.as_str()),
                    Some(*condition_version),
                    None,
                ),
                RecordedOperationResult::DirectiveConsumed {
                    window_id,
                    attempt_id,
                    ..
                }
                | RecordedOperationResult::AttemptOutcomeRecorded {
                    window_id,
                    attempt_id,
                    ..
                } => (
                    window_id.as_str(),
                    None,
                    None,
                    None,
                    Some(attempt_id.as_str()),
                ),
            },
            CommittedOperationDisposition::CommittedSecurityRejection(value) => (
                value.window_id.as_str(),
                match record.operation_kind {
                    AuthorizedExecutionContinuityOperationKind::RegisterYield => {
                        Some("seed-generation")
                    }
                    _ => None,
                },
                match record.operation_kind {
                    AuthorizedExecutionContinuityOperationKind::TransitionWait => Some("seed-wait"),
                    _ => None,
                },
                match record.operation_kind {
                    AuthorizedExecutionContinuityOperationKind::TransitionWait => Some(1),
                    _ => None,
                },
                match record.operation_kind {
                    AuthorizedExecutionContinuityOperationKind::ConsumeDirective
                    | AuthorizedExecutionContinuityOperationKind::RecordAttemptOutcome
                    | AuthorizedExecutionContinuityOperationKind::RecoverAmbiguousAttempt => {
                        Some("seed-attempt")
                    }
                    _ => None,
                },
            ),
        };
        RequestEnvelope {
            version: 1,
            domain: "workflow-os/authorized-execution-continuity/seed/v1".to_owned(),
            fields: vec![
                record.operation_id.as_str().to_owned(),
                record.receipt.receipt_id.as_str().to_owned(),
            ],
            window_id: window.to_owned(),
            yield_generation_id: yield_id.map(str::to_owned),
            wait_condition_id: wait_id.map(str::to_owned),
            wait_condition_version: wait_version,
            attempt_id: attempt_id.map(str::to_owned),
        }
    }

    fn unique_path() -> PathBuf {
        static NEXT_FIXTURE_ID: AtomicU64 = AtomicU64::new(1);

        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "workflow-os-continuity-{}-{nanos}-{}.sqlite",
            std::process::id(),
            NEXT_FIXTURE_ID.fetch_add(1, Ordering::Relaxed)
        ))
    }

    #[cfg(test)]
    mod tests {
        use std::collections::BTreeMap;
        use std::process::{Command, Stdio};
        use std::sync::{Arc, Barrier};
        use std::thread;

        use crate::authorized_execution_continuity_state::internal::{
            AuthorityUseCapability, ContinuityWakeSourceReference, ExpectedWaitRevision,
            WakeAssessmentCapability,
        };
        use crate::{
            ActorId, AuthorizedExecutionAttemptId, AuthorizedExecutionWaitConditionId,
            AuthorizedExecutionWakeTriggerKind, AuthorizedExecutionWindowId,
            AuthorizedExecutionYieldReason, EventId, EventSequenceNumber,
            ImmutableRunBundleBinding, StepId, WorkflowId, WorkflowRunId,
        };

        use super::*;

        const SUBPROCESS_CHILD_TEST: &str = "sqlite_state::continuity_store::conformance_backend::tests::sqlite_subprocess_crash_child";

        struct Fixture {
            backend: SqliteConformanceBackend,
            window_id: AuthorizedExecutionWindowId,
            attempt_id: AuthorizedExecutionAttemptId,
            generation_id: ContinuityYieldGenerationId,
            directive_id: ContinuityDirectiveId,
            wait_id: Option<AuthorizedExecutionWaitConditionId>,
            cursor: ContinuityCursor,
        }

        #[allow(clippy::too_many_lines)]
        fn fixture(yielded: bool, with_wait: bool) -> Fixture {
            let window_id =
                AuthorizedExecutionWindowId::new("window/sqlite-continuity").expect("window");
            let attempt_id =
                AuthorizedExecutionAttemptId::new("attempt/sqlite-continuity/1").expect("attempt");
            let generation_id =
                ContinuityYieldGenerationId::new("yield/sqlite-continuity/1").expect("yield");
            let directive_id =
                ContinuityDirectiveId::new("directive/sqlite-continuity/1").expect("directive");
            let wait_id = with_wait.then(|| {
                AuthorizedExecutionWaitConditionId::new("wait/sqlite-continuity/1").expect("wait")
            });
            let cursor = ContinuityCursor {
                sequence_number: EventSequenceNumber::new(7).expect("sequence"),
                event_id: EventId::new("event/sqlite-continuity/7").expect("event"),
            };
            let revision = ContinuityRevision::new(1).expect("revision");
            let watermark = Timestamp::parse_rfc3339("2026-08-15T12:00:00Z").expect("time");
            let bundle: ImmutableRunBundleBinding = serde_json::from_value(serde_json::json!({"bundle_id":"bundle/sqlite-continuity","bundle_version":"v1","root_hash":SpecContentHash::from_text("bundle").as_str()})).expect("bundle");
            let operation_id =
                ContinuityOperationId::new("operation/sqlite-seed-consume").expect("operation");
            let authority = SpecContentHash::from_text("sqlite authority");
            let window = AuthoritativeWindowRecord {
                workflow_id: WorkflowId::new("workflow/sqlite-continuity").expect("workflow"),
                run_id: WorkflowRunId::new("run/sqlite-continuity").expect("run"),
                step_id: StepId::new("step-sqlite-continuity").expect("step"),
                window_id: window_id.clone(),
                subject_actor_id: ActorId::new("agent/sqlite-continuity").expect("actor"),
                immutable_run_bundle: bundle,
                governance_commitment: SpecContentHash::from_text("governance"),
                authority_commitment: authority.clone(),
                cursor: cursor.clone(),
                state: if yielded {
                    AuthoritativeWindowState::Yielded
                } else {
                    AuthoritativeWindowState::Executing
                },
                maximum_attempts: 3,
                next_attempt_number: 2,
                expires_at: Timestamp::parse_rfc3339("2026-08-15T13:00:00Z").expect("expiry"),
                trusted_time_watermark: watermark,
                trusted_time_epoch_id: expected_epoch().expect("epoch"),
                revision,
                active_yield: yielded.then(|| generation_id.clone()),
            };
            let attempt = AuthoritativeAttemptRecord {
                attempt_id: attempt_id.clone(),
                attempt_number: 1,
                window_id: window_id.clone(),
                subject_actor_id: window.subject_actor_id.clone(),
                cursor: cursor.clone(),
                authority_commitment: authority.clone(),
                consume_operation_id: operation_id.clone(),
                state: if yielded {
                    AuthoritativeAttemptState::Yielded
                } else {
                    AuthoritativeAttemptState::Started
                },
                revision,
            };
            let observation = trusted_time_observation(
                watermark,
                TrustedTimeSourceKind::CoreInjectedClockV1,
                expected_provenance().expect("provenance"),
                expected_epoch().expect("epoch"),
            );
            let seed_result = RecordedOperationResult::DirectiveConsumed {
                window_id: window_id.clone(),
                directive_id: directive_id.clone(),
                generation_id: generation_id.clone(),
                attempt_id: attempt_id.clone(),
                attempt_number: 1,
                directive_state: AuthoritativeDirectiveState::Consumed,
                attempt_state: AuthoritativeAttemptState::Started,
                window_state: AuthoritativeWindowState::Executing,
                window_revision: revision,
            };
            let disposition = CommittedOperationDisposition::CommittedSuccess(seed_result);
            let request_commitment = SpecContentHash::from_text("seed request");
            let receipt_id =
                ContinuityReceiptId::new("receipt/sqlite-seed-consume").expect("receipt");
            let trusted = trusted_time_commitment(&observation);
            let committed = operation_commitment(
                &request_commitment,
                &receipt_id,
                &observation,
                &trusted,
                &disposition,
            );
            let operation = AuthoritativeOperationRecord {
                operation_id: operation_id.clone(),
                operation_kind: AuthorizedExecutionContinuityOperationKind::ConsumeDirective,
                request_commitment,
                operation_commitment: committed.clone(),
                receipt: ContinuityReceipt {
                    receipt_id,
                    operation_kind: AuthorizedExecutionContinuityOperationKind::ConsumeDirective,
                    operation_commitment: committed,
                    trusted_time_commitment: trusted,
                    committed_at: watermark,
                },
                trusted_time: observation,
                disposition,
            };
            let mut yields = BTreeMap::new();
            let mut directives = BTreeMap::new();
            let mut waits = BTreeMap::new();
            if yielded {
                let wait_ids = wait_id
                    .iter()
                    .cloned()
                    .map(|id| AuthoritativeWaitIdentity::new(id, 1))
                    .collect::<Vec<_>>();
                yields.insert(
                    generation_id.clone(),
                    AuthoritativeYieldRecord {
                        generation_id: generation_id.clone(),
                        attempt_id: attempt_id.clone(),
                        cursor: cursor.clone(),
                        reason: AuthorizedExecutionYieldReason::TurnBoundary,
                        wait_ids: wait_ids.clone(),
                        registered_at: watermark,
                    },
                );
                directives.insert(
                    directive_id.clone(),
                    AuthoritativeDirectiveRecord {
                        directive_id: directive_id.clone(),
                        window_id: window_id.clone(),
                        generation_id: generation_id.clone(),
                        cursor: cursor.clone(),
                        authority_commitment: authority,
                        state: AuthoritativeDirectiveState::Available,
                        revision,
                    },
                );
                if let Some(id) = wait_id.as_ref() {
                    waits.insert(
                        AuthoritativeWaitIdentity::new(id.clone(), 1),
                        AuthoritativeWaitRecord {
                            condition_id: id.clone(),
                            condition_version: 1,
                            window_id: window_id.clone(),
                            generation_id: generation_id.clone(),
                            wake_trigger: AuthorizedExecutionWakeTriggerKind::EvidenceAccepted,
                            state: AuthoritativeWaitState::Unsatisfied,
                            source_commitment: None,
                            source_revision: None,
                            revision,
                        },
                    );
                }
            }
            let state = ReferenceContinuityState {
                trusted_time: TrustedTimeSecurityRecord {
                    source: TrustedTimeSourceKind::CoreInjectedClockV1,
                    provenance_commitment: expected_provenance().expect("provenance"),
                    epoch_id: expected_epoch().expect("epoch"),
                    last_observed_at: Some(watermark),
                    posture: TrustedTimePosture::Healthy,
                    eligibility: ContinuityInstanceEligibility::LiveStateEligible,
                    revision,
                },
                windows: BTreeMap::from([(window_id.clone(), window)]),
                yields,
                waits,
                directives,
                attempts: BTreeMap::from([(attempt_id.clone(), attempt)]),
                operations: BTreeMap::from([(operation_id, operation)]),
            };
            Fixture {
                backend: SqliteConformanceBackend::seeded(&state),
                window_id,
                attempt_id,
                generation_id,
                directive_id,
                wait_id,
                cursor,
            }
        }

        fn binding(f: &Fixture) -> ExpectedWindowBinding {
            let state = f.backend.conformance_snapshot();
            let window = state.windows.get(&f.window_id).expect("window");
            ExpectedWindowBinding {
                workflow_id: window.workflow_id.clone(),
                run_id: window.run_id.clone(),
                step_id: window.step_id.clone(),
                subject_actor_id: window.subject_actor_id.clone(),
                immutable_run_bundle: window.immutable_run_bundle.clone(),
                governance_commitment: window.governance_commitment.clone(),
                authority_commitment: window.authority_commitment.clone(),
                cursor: window.cursor.clone(),
            }
        }
        fn attempt_capability(f: &Fixture) -> AttemptUseCapability {
            let state = f.backend.conformance_snapshot();
            let window = state.windows.get(&f.window_id).expect("window");
            let attempt = state.attempts.get(&f.attempt_id).expect("attempt");
            AttemptUseCapability {
                attempt_id: f.attempt_id.clone(),
                subject_actor_id: window.subject_actor_id.clone(),
                window_id: f.window_id.clone(),
                window_revision: window.revision,
                cursor: f.cursor.clone(),
                authority_commitment: window.authority_commitment.clone(),
                window_binding_commitment: window_binding_commitment(&binding(f)),
                consume_operation_id: attempt.consume_operation_id.clone(),
            }
        }
        fn authority_capability(f: &Fixture) -> AuthorityUseCapability {
            let state = f.backend.conformance_snapshot();
            let window = state.windows.get(&f.window_id).expect("window");
            let waits = state
                .yields
                .get(&f.generation_id)
                .expect("yield")
                .wait_ids
                .iter()
                .map(|id| {
                    let wait = state.waits.get(id).expect("wait");
                    ExpectedWaitRevision {
                        condition_id: id.condition_id.clone(),
                        condition_version: id.condition_version,
                        revision: wait.revision,
                    }
                })
                .collect();
            AuthorityUseCapability {
                window_id: f.window_id.clone(),
                window_revision: window.revision,
                generation_id: f.generation_id.clone(),
                cursor: f.cursor.clone(),
                subject_actor_id: window.subject_actor_id.clone(),
                authority_commitment: window.authority_commitment.clone(),
                window_binding_commitment: window_binding_commitment(&binding(f)),
                expected_waits: waits,
            }
        }

        fn register_request<'a>(
            f: &Fixture,
            capability: &'a AttemptUseCapability,
        ) -> RegisterYieldRequest<'a> {
            let state = f.backend.conformance_snapshot();
            let window = state.windows.get(&f.window_id).expect("window");
            let mut request = RegisterYieldRequest {
                operation_id: ContinuityOperationId::new("operation/sqlite-register")
                    .expect("operation"),
                request_commitment: SpecContentHash::from_text("pending"),
                receipt_id: ContinuityReceiptId::new("receipt/sqlite-register").expect("receipt"),
                generation_id: ContinuityYieldGenerationId::new("yield/sqlite-register/2")
                    .expect("yield"),
                window_id: f.window_id.clone(),
                expected_window_revision: window.revision,
                expected_window_binding: binding(f),
                cursor: f.cursor.clone(),
                attempt_id: f.attempt_id.clone(),
                attempt_capability: capability,
                reason: AuthorizedExecutionYieldReason::ContextBudget,
                waits: Vec::new(),
            };
            request.request_commitment = expected_register_yield_commitment(&request);
            request
        }

        #[test]
        fn sqlite_subprocess_crash_child() {
            let Ok(mode) = std::env::var("WORKFLOW_OS_SQLITE_CONTINUITY_CHILD_MODE") else {
                return;
            };
            let path = PathBuf::from(
                std::env::var("WORKFLOW_OS_SQLITE_CONTINUITY_CHILD_PATH")
                    .expect("child database path"),
            );
            match mode.as_str() {
                "uncommitted-crash" => {
                    let connection = Connection::open(path).expect("child connection");
                    connection
                        .execute_batch(
                            "PRAGMA journal_mode = WAL;
                             BEGIN IMMEDIATE;
                             UPDATE continuity_windows
                             SET state = 'closed'
                             WHERE window_id = 'window/conformance';",
                        )
                        .expect("child uncommitted mutation");
                    std::process::abort();
                }
                "committed-operation" => {
                    let fixture = ContinuityConformanceFixture {
                        backend: SqliteConformanceBackend::reopen_path(path),
                        window_id: AuthorizedExecutionWindowId::new("window/conformance")
                            .expect("window"),
                        attempt_id: AuthorizedExecutionAttemptId::new("attempt/conformance/1")
                            .expect("attempt"),
                        generation_id: ContinuityYieldGenerationId::new("yield/conformance/1")
                            .expect("yield"),
                        directive_id: ContinuityDirectiveId::new("directive/conformance/1")
                            .expect("directive"),
                        wait_id: None,
                        cursor: ContinuityCursor {
                            sequence_number: EventSequenceNumber::new(7).expect("sequence"),
                            event_id: EventId::new("event/conformance/7").expect("event"),
                        },
                    };
                    let capability = fixture.attempt_capability();
                    assert!(matches!(
                        fixture
                            .backend
                            .register_yield(fixture.register_request(&capability)),
                        Ok(RegisterYieldResult::Registered(_))
                    ));
                }
                "verify-attempt-state" => {
                    let expected =
                        std::env::var("WORKFLOW_OS_SQLITE_CONTINUITY_EXPECTED_ATTEMPT_STATE")
                            .expect("expected attempt state");
                    let backend = SqliteConformanceBackend::reopen_path(path);
                    let snapshot = backend.conformance_snapshot();
                    let attempt = snapshot
                        .attempts
                        .get(
                            &AuthorizedExecutionAttemptId::new("attempt/conformance/1")
                                .expect("attempt"),
                        )
                        .expect("persisted attempt");
                    let actual = match attempt.state {
                        AuthoritativeAttemptState::Started => "started",
                        AuthoritativeAttemptState::Yielded => "yielded",
                        AuthoritativeAttemptState::Succeeded => "succeeded",
                        AuthoritativeAttemptState::RetryableFailure => "retryable_failure",
                        AuthoritativeAttemptState::TerminalFailure => "terminal_failure",
                        AuthoritativeAttemptState::AmbiguousMayHaveStarted => {
                            "ambiguous_may_have_started"
                        }
                    };
                    assert_eq!(actual, expected);
                    assert_eq!(
                        snapshot.trusted_time.eligibility,
                        ContinuityInstanceEligibility::LiveStateEligible
                    );
                }
                _ => panic!("unknown child mode"),
            }
        }

        fn assert_subprocess_attempt_state(path: &PathBuf, expected: &str) {
            let status = Command::new(std::env::current_exe().expect("test binary"))
                .arg("--exact")
                .arg(SUBPROCESS_CHILD_TEST)
                .arg("--nocapture")
                .env(
                    "WORKFLOW_OS_SQLITE_CONTINUITY_CHILD_MODE",
                    "verify-attempt-state",
                )
                .env("WORKFLOW_OS_SQLITE_CONTINUITY_CHILD_PATH", path)
                .env(
                    "WORKFLOW_OS_SQLITE_CONTINUITY_EXPECTED_ATTEMPT_STATE",
                    expected,
                )
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .expect("attempt-state child");
            assert!(status.success(), "subprocess did not verify {expected}");
        }

        #[test]
        fn sqlite_subprocess_crash_and_wal_recovery_use_one_durable_path() {
            let test_binary = std::env::current_exe().expect("test binary");

            let rollback =
                ContinuityConformanceFixture::<SqliteConformanceBackend>::new(false, false);
            let rollback_path = rollback.backend.store.backend.database_path.clone();
            let before = rollback.backend.conformance_snapshot();
            let status = Command::new(&test_binary)
                .arg("--exact")
                .arg(SUBPROCESS_CHILD_TEST)
                .arg("--nocapture")
                .env(
                    "WORKFLOW_OS_SQLITE_CONTINUITY_CHILD_MODE",
                    "uncommitted-crash",
                )
                .env("WORKFLOW_OS_SQLITE_CONTINUITY_CHILD_PATH", &rollback_path)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .expect("crash child");
            assert!(!status.success());
            let reopened = SqliteConformanceBackend::reopen_path(rollback_path);
            assert!(reopened.conformance_snapshot() == before);
            assert!(matches!(
                reopened.recover_ambiguous_attempt(rollback.recovery_request()),
                Ok(MutationResult::Recorded(_))
            ));

            let committed =
                ContinuityConformanceFixture::<SqliteConformanceBackend>::new(false, false);
            let committed_path = committed.backend.store.backend.database_path.clone();
            let capability = committed.attempt_capability();
            let replay = committed.register_request(&capability);
            let operation_id = replay.operation_id.clone();
            let request_commitment = replay.request_commitment.clone();
            let receipt_id = replay.receipt_id.clone();
            let status = Command::new(test_binary)
                .arg("--exact")
                .arg(SUBPROCESS_CHILD_TEST)
                .arg("--nocapture")
                .env(
                    "WORKFLOW_OS_SQLITE_CONTINUITY_CHILD_MODE",
                    "committed-operation",
                )
                .env("WORKFLOW_OS_SQLITE_CONTINUITY_CHILD_PATH", &committed_path)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .expect("commit child");
            assert!(status.success());
            let reopened = SqliteConformanceBackend::reopen_path(committed_path);
            assert!(matches!(
                reopened.reconcile_operation(&ReconcileOperationRequest {
                    operation_id,
                    expected_request_commitment: request_commitment,
                    expected_receipt_id: receipt_id,
                }),
                ContinuityReconciliationResult::DurablyCommitted(_)
            ));
            reopened.conformance_set_time_available(false);
            assert!(matches!(
                reopened.register_yield(replay),
                Ok(RegisterYieldResult::ExactReplay(_))
            ));
        }

        #[test]
        fn sqlite_subprocess_restart_proves_every_attempt_posture() {
            let started =
                ContinuityConformanceFixture::<SqliteConformanceBackend>::new(false, false);
            assert_subprocess_attempt_state(
                &started.backend.store.backend.database_path,
                "started",
            );

            let yielded =
                ContinuityConformanceFixture::<SqliteConformanceBackend>::new(true, false);
            assert_subprocess_attempt_state(
                &yielded.backend.store.backend.database_path,
                "yielded",
            );

            for (outcome, expected) in [
                (AuthorizedExecutionAttemptOutcome::Succeeded, "succeeded"),
                (
                    AuthorizedExecutionAttemptOutcome::RetryableFailure,
                    "retryable_failure",
                ),
                (
                    AuthorizedExecutionAttemptOutcome::TerminalFailure,
                    "terminal_failure",
                ),
            ] {
                let fixture =
                    ContinuityConformanceFixture::<SqliteConformanceBackend>::new(false, false);
                let capability = fixture.attempt_capability();
                assert!(fixture
                    .backend
                    .record_attempt_outcome(fixture.outcome_request(&capability, outcome))
                    .is_ok());
                assert_subprocess_attempt_state(
                    &fixture.backend.store.backend.database_path,
                    expected,
                );
            }

            let ambiguous =
                ContinuityConformanceFixture::<SqliteConformanceBackend>::new(false, false);
            assert!(ambiguous
                .backend
                .recover_ambiguous_attempt(ambiguous.recovery_request())
                .is_ok());
            assert_subprocess_attempt_state(
                &ambiguous.backend.store.backend.database_path,
                "ambiguous_may_have_started",
            );
        }

        #[test]
        fn sqlite_register_yield_commits_and_exact_replay_precedes_clock() {
            let f = fixture(false, false);
            let capability = attempt_capability(&f);
            let request = register_request(&f, &capability);
            let replay = register_request(&f, &capability);
            assert!(matches!(
                f.backend.register_yield(request),
                Ok(RegisterYieldResult::Registered(_))
            ));
            f.backend.conformance_set_time_available(false);
            let replay_result = f.backend.register_yield(replay);
            match replay_result {
                Ok(RegisterYieldResult::ExactReplay(_)) => {}
                Ok(_) => panic!("exact replay returned a non-replay result"),
                Err(error) => panic!("exact replay failed with {error:?}"),
            }
        }

        #[test]
        fn sqlite_commit_faults_roll_back_or_reconcile_without_partial_state() {
            for fault in [
                ContinuityConformanceFault::Before,
                ContinuityConformanceFault::During,
            ] {
                let f = fixture(false, false);
                let capability = attempt_capability(&f);
                let request = register_request(&f, &capability);
                let operation_id = request.operation_id.clone();
                let generation_id = request.generation_id.clone();
                let before = f.backend.conformance_snapshot();
                let before_window = before.windows.get(&f.window_id).expect("window").clone();
                f.backend.conformance_inject_fault(fault);

                let Err(error) = f.backend.register_yield(request) else {
                    panic!("injected fault must fail");
                };
                assert_eq!(error.code(), "state.sqlite.write_failed");

                let after = f.backend.conformance_snapshot();
                let after_window = after.windows.get(&f.window_id).expect("window");
                assert_eq!(after_window.state, before_window.state);
                assert_eq!(after_window.revision, before_window.revision);
                assert_eq!(after_window.active_yield, before_window.active_yield);
                assert_eq!(after.trusted_time.revision, before.trusted_time.revision);
                assert!(!after.operations.contains_key(&operation_id));
                assert!(!after.yields.contains_key(&generation_id));
            }

            let f = fixture(false, false);
            let capability = attempt_capability(&f);
            let request = register_request(&f, &capability);
            let operation_id = request.operation_id.clone();
            let request_commitment = request.request_commitment.clone();
            let receipt_id = request.receipt_id.clone();
            f.backend
                .conformance_inject_fault(ContinuityConformanceFault::After);

            let Err(error) = f.backend.register_yield(request) else {
                panic!("post-commit acknowledgement fault must be ambiguous");
            };
            assert_eq!(error.code(), "state.sqlite.commit_ambiguous");
            assert!(matches!(
                f.backend.reconcile_operation(&ReconcileOperationRequest {
                    operation_id,
                    expected_request_commitment: request_commitment,
                    expected_receipt_id: receipt_id,
                }),
                ContinuityReconciliationResult::DurablyCommitted(_)
            ));
        }

        #[test]
        fn sqlite_operation_projection_tampering_fails_closed_without_leakage() {
            let f = fixture(false, false);
            let capability = attempt_capability(&f);
            f.backend
                .register_yield(register_request(&f, &capability))
                .expect("register yield");
            let secret_marker = "token-secret-relational-commitment";
            f.backend
                .store
                .backend
                .connection()
                .expect("connection")
                .execute(
                    "UPDATE continuity_operations
                     SET result_commitment = ?1
                     WHERE operation_id = 'operation/sqlite-register'",
                    [secret_marker],
                )
                .expect("tamper relational commitment");

            let Err(error) = super::super::super::continuity_codec::load_snapshot(
                &f.backend.store.backend.connection().expect("connection"),
            ) else {
                panic!("projection tampering must fail closed");
            };
            assert_eq!(
                error.code(),
                "authorized_execution_continuity_state.state.corrupt"
            );
            assert!(!format!("{error:?}").contains(secret_marker));
        }

        #[test]
        fn sqlite_trusted_time_unavailability_rolls_back_without_an_operation() {
            let f = fixture(false, false);
            let capability = attempt_capability(&f);
            let request = register_request(&f, &capability);
            let operation_id = request.operation_id.clone();
            let before = f.backend.conformance_snapshot();
            f.backend.conformance_set_time_available(false);

            let Err(error) = f.backend.register_yield(request) else {
                panic!("unavailable trusted time must fail");
            };
            assert_eq!(
                error.code(),
                "authorized_execution_continuity_state.time.unavailable"
            );
            let after = f.backend.conformance_snapshot();
            assert!(!after.operations.contains_key(&operation_id));
            assert_eq!(after.trusted_time.revision, before.trusted_time.revision);
            assert_eq!(
                after.windows.get(&f.window_id).expect("window").revision,
                before.windows.get(&f.window_id).expect("window").revision
            );
        }

        #[test]
        fn sqlite_trusted_time_regression_provenance_and_epoch_mismatch_quarantine() {
            for rejection in [
                CommittedSecurityRejectionKind::Regressed,
                CommittedSecurityRejectionKind::Untrusted,
                CommittedSecurityRejectionKind::EpochMismatch,
            ] {
                let f = fixture(false, false);
                let capability = attempt_capability(&f);
                let request = register_request(&f, &capability);
                match rejection {
                    CommittedSecurityRejectionKind::Regressed => f.backend.conformance_set_time(
                        Timestamp::parse_rfc3339("2026-08-15T11:59:59Z").expect("time"),
                    ),
                    CommittedSecurityRejectionKind::Untrusted => f
                        .backend
                        .conformance_set_time_provenance(SpecContentHash::from_text(
                            "different trusted clock provenance",
                        )),
                    CommittedSecurityRejectionKind::EpochMismatch => {
                        f.backend.conformance_set_time_epoch(
                            ContinuityTrustedTimeEpochId::new("epoch/different/1").expect("epoch"),
                        );
                    }
                    CommittedSecurityRejectionKind::Expired => unreachable!(),
                }

                match f.backend.register_yield(request) {
                    Ok(RegisterYieldResult::SecurityRejected(value)) => {
                        assert_eq!(value.kind, rejection);
                    }
                    Ok(_) => panic!("trusted-time mismatch must be security rejected"),
                    Err(error) => panic!("trusted-time mismatch was not committed: {error:?}"),
                }
                let state = f.backend.conformance_snapshot();
                assert_eq!(state.trusted_time.posture, TrustedTimePosture::Quarantined);
                assert_eq!(
                    state.trusted_time.eligibility,
                    ContinuityInstanceEligibility::Quarantined
                );
            }
        }

        #[test]
        fn sqlite_trusted_time_expiry_commits_window_expiration_without_global_quarantine() {
            let f = fixture(false, false);
            let capability = attempt_capability(&f);
            let request = register_request(&f, &capability);
            f.backend.conformance_set_time(
                Timestamp::parse_rfc3339("2026-08-15T13:00:01Z").expect("time"),
            );

            match f.backend.register_yield(request) {
                Ok(RegisterYieldResult::SecurityRejected(value)) => {
                    assert_eq!(value.kind, CommittedSecurityRejectionKind::Expired);
                }
                Ok(_) => panic!("expired window must be security rejected"),
                Err(error) => panic!("window expiration was not committed: {error:?}"),
            }
            let state = f.backend.conformance_snapshot();
            assert_eq!(
                state.windows.get(&f.window_id).expect("window").state,
                AuthoritativeWindowState::Expired
            );
            assert_eq!(state.trusted_time.posture, TrustedTimePosture::Healthy);
            assert_eq!(
                state.trusted_time.eligibility,
                ContinuityInstanceEligibility::LiveStateEligible
            );
        }

        #[test]
        fn sqlite_transition_wait_commits_under_immediate_transaction() {
            let f = fixture(true, true);
            let state = f.backend.conformance_snapshot();
            let window = state.windows.get(&f.window_id).expect("window");
            let wait_id = f.wait_id.clone().expect("wait");
            let wake = WakeAssessmentCapability {
                window_id: f.window_id.clone(),
                generation_id: f.generation_id.clone(),
                condition_id: wait_id.clone(),
                condition_version: 1,
                trigger: AuthorizedExecutionWakeTriggerKind::EvidenceAccepted,
                source_reference: ContinuityWakeSourceReference::new("evidence/sqlite/1")
                    .expect("source"),
                source_commitment: SpecContentHash::from_text("source"),
                source_revision: 1,
            };
            let mut request = TransitionWaitRequest {
                operation_id: ContinuityOperationId::new("operation/sqlite-wait")
                    .expect("operation"),
                request_commitment: SpecContentHash::from_text("pending"),
                receipt_id: ContinuityReceiptId::new("receipt/sqlite-wait").expect("receipt"),
                window_id: f.window_id.clone(),
                expected_window_revision: window.revision,
                expected_window_binding: binding(&f),
                cursor: f.cursor.clone(),
                condition_id: wait_id,
                expected_generation_id: f.generation_id.clone(),
                expected_condition_version: 1,
                expected_wait_revision: ContinuityRevision::new(1).expect("revision"),
                target: AuthoritativeWaitState::Satisfied,
                wake_capability: Some(&wake),
            };
            request.request_commitment = expected_transition_wait_commitment(&request);
            assert!(matches!(
                f.backend.transition_wait(request),
                Ok(MutationResult::Recorded(_))
            ));
        }

        #[test]
        fn sqlite_consume_directive_is_one_use_and_replay_has_no_capability() {
            let f = fixture(true, false);
            let capability = authority_capability(&f);
            let state = f.backend.conformance_snapshot();
            let window = state.windows.get(&f.window_id).expect("window");
            let attempt =
                AuthorizedExecutionAttemptId::new("attempt/sqlite-continuity/2").expect("attempt");
            let mut request = ConsumeDirectiveRequest {
                operation_id: ContinuityOperationId::new("operation/sqlite-consume")
                    .expect("operation"),
                request_commitment: SpecContentHash::from_text("pending"),
                receipt_id: ContinuityReceiptId::new("receipt/sqlite-consume").expect("receipt"),
                directive_id: f.directive_id.clone(),
                window_id: f.window_id.clone(),
                expected_window_revision: window.revision,
                expected_window_binding: binding(&f),
                generation_id: f.generation_id.clone(),
                cursor: f.cursor.clone(),
                expected_waits: Vec::new(),
                authority_capability: capability,
                generated_attempt_id: attempt.clone(),
            };
            request.request_commitment = expected_consume_directive_commitment(&request);
            assert!(matches!(
                f.backend.consume_directive(request),
                Ok(ConsumeDirectiveResult::Consumed { .. })
            ));
            let capability = authority_capability_from_snapshot(&f, window.revision);
            let mut replay = ConsumeDirectiveRequest {
                operation_id: ContinuityOperationId::new("operation/sqlite-consume")
                    .expect("operation"),
                request_commitment: SpecContentHash::from_text("pending"),
                receipt_id: ContinuityReceiptId::new("receipt/sqlite-consume").expect("receipt"),
                directive_id: f.directive_id.clone(),
                window_id: f.window_id.clone(),
                expected_window_revision: window.revision,
                expected_window_binding: binding(&f),
                generation_id: f.generation_id.clone(),
                cursor: f.cursor.clone(),
                expected_waits: Vec::new(),
                authority_capability: capability,
                generated_attempt_id: attempt,
            };
            replay.request_commitment = expected_consume_directive_commitment(&replay);
            assert!(matches!(
                f.backend.consume_directive(replay),
                Ok(ConsumeDirectiveResult::ExactReplay(_))
            ));
        }

        fn authority_capability_from_snapshot(
            f: &Fixture,
            revision: ContinuityRevision,
        ) -> AuthorityUseCapability {
            let state = f.backend.conformance_snapshot();
            let window = state.windows.get(&f.window_id).expect("window");
            AuthorityUseCapability {
                window_id: f.window_id.clone(),
                window_revision: revision,
                generation_id: f.generation_id.clone(),
                cursor: f.cursor.clone(),
                subject_actor_id: window.subject_actor_id.clone(),
                authority_commitment: window.authority_commitment.clone(),
                window_binding_commitment: window_binding_commitment(&binding(f)),
                expected_waits: Vec::new(),
            }
        }

        #[test]
        fn sqlite_record_attempt_outcome_and_recover_ambiguity_are_durable() {
            let f = fixture(false, false);
            let capability = attempt_capability(&f);
            let state = f.backend.conformance_snapshot();
            let window = state.windows.get(&f.window_id).expect("window");
            let mut request = RecordAttemptOutcomeRequest {
                operation_id: ContinuityOperationId::new("operation/sqlite-outcome")
                    .expect("operation"),
                request_commitment: SpecContentHash::from_text("pending"),
                receipt_id: ContinuityReceiptId::new("receipt/sqlite-outcome").expect("receipt"),
                window_id: f.window_id.clone(),
                expected_window_revision: window.revision,
                expected_window_binding: binding(&f),
                attempt_id: f.attempt_id.clone(),
                expected_attempt_revision: ContinuityRevision::new(1).expect("revision"),
                attempt_capability: &capability,
                outcome: AuthorizedExecutionAttemptOutcome::Succeeded,
            };
            request.request_commitment = expected_attempt_outcome_commitment(&request);
            assert!(matches!(
                f.backend.record_attempt_outcome(request),
                Ok(MutationResult::Recorded(_))
            ));
            let g = fixture(false, false);
            let state = g.backend.conformance_snapshot();
            let window = state.windows.get(&g.window_id).expect("window");
            let mut recover = RecoverAmbiguousAttemptRequest {
                operation_id: ContinuityOperationId::new("operation/sqlite-recover")
                    .expect("operation"),
                request_commitment: SpecContentHash::from_text("pending"),
                receipt_id: ContinuityReceiptId::new("receipt/sqlite-recover").expect("receipt"),
                window_id: g.window_id.clone(),
                expected_window_revision: window.revision,
                expected_window_binding: binding(&g),
                cursor: g.cursor.clone(),
                attempt_id: g.attempt_id.clone(),
                expected_attempt_revision: ContinuityRevision::new(1).expect("revision"),
            };
            recover.request_commitment = expected_recovery_commitment(&recover);
            assert!(matches!(
                g.backend.recover_ambiguous_attempt(recover),
                Ok(MutationResult::Recorded(_))
            ));
        }

        #[test]
        fn sqlite_restart_reconciliation_trusted_time_and_contention_fail_closed() {
            let f = fixture(false, false);
            let capability = attempt_capability(&f);
            let request = register_request(&f, &capability);
            let operation_id = request.operation_id.clone();
            let request_commitment = request.request_commitment.clone();
            let receipt_id = request.receipt_id.clone();
            f.backend.register_yield(request).expect("register");
            let reopened =
                SqliteConformanceBackend::conformance_reopen(f.backend.conformance_snapshot());
            reopened.conformance_set_time_available(false);
            let reconcile = reopened.reconcile_operation(&ReconcileOperationRequest {
                operation_id,
                expected_request_commitment: request_commitment,
                expected_receipt_id: receipt_id,
            });
            assert!(matches!(
                reconcile,
                ContinuityReconciliationResult::DurablyCommitted(_)
            ));
            let race = fixture(false, false);
            let barrier = Arc::new(Barrier::new(3));
            let mut joins = Vec::new();
            for _ in 0..2 {
                let backend = race.backend.clone();
                let barrier = barrier.clone();
                let window_id = race.window_id.clone();
                let attempt_id = race.attempt_id.clone();
                let cursor = race.cursor.clone();
                joins.push(thread::spawn(move || {
                    let state = backend.conformance_snapshot();
                    let window = state.windows.get(&window_id).expect("window");
                    let attempt = state.attempts.get(&attempt_id).expect("attempt");
                    let binding = ExpectedWindowBinding {
                        workflow_id: window.workflow_id.clone(),
                        run_id: window.run_id.clone(),
                        step_id: window.step_id.clone(),
                        subject_actor_id: window.subject_actor_id.clone(),
                        immutable_run_bundle: window.immutable_run_bundle.clone(),
                        governance_commitment: window.governance_commitment.clone(),
                        authority_commitment: window.authority_commitment.clone(),
                        cursor: cursor.clone(),
                    };
                    let capability = AttemptUseCapability {
                        attempt_id: attempt_id.clone(),
                        subject_actor_id: window.subject_actor_id.clone(),
                        window_id: window_id.clone(),
                        window_revision: window.revision,
                        cursor: cursor.clone(),
                        authority_commitment: window.authority_commitment.clone(),
                        window_binding_commitment: window_binding_commitment(&binding),
                        consume_operation_id: attempt.consume_operation_id.clone(),
                    };
                    let mut request = RegisterYieldRequest {
                        operation_id: ContinuityOperationId::new("operation/sqlite-race")
                            .expect("operation"),
                        request_commitment: SpecContentHash::from_text("pending"),
                        receipt_id: ContinuityReceiptId::new("receipt/sqlite-race")
                            .expect("receipt"),
                        generation_id: ContinuityYieldGenerationId::new("yield/sqlite-race/2")
                            .expect("yield"),
                        window_id,
                        expected_window_revision: window.revision,
                        expected_window_binding: binding,
                        cursor,
                        attempt_id,
                        attempt_capability: &capability,
                        reason: AuthorizedExecutionYieldReason::HostPreemption,
                        waits: Vec::new(),
                    };
                    request.request_commitment = expected_register_yield_commitment(&request);
                    barrier.wait();
                    backend.register_yield(request)
                }));
            }
            barrier.wait();
            let results = joins
                .into_iter()
                .map(|join| join.join().expect("join"))
                .collect::<Vec<_>>();
            assert_eq!(
                results
                    .iter()
                    .filter(|result| matches!(result, Ok(RegisterYieldResult::Registered(_))))
                    .count(),
                1
            );
            assert_eq!(
                results
                    .iter()
                    .filter(|result| matches!(result, Ok(RegisterYieldResult::ExactReplay(_))))
                    .count(),
                1
            );
        }
    }
}
