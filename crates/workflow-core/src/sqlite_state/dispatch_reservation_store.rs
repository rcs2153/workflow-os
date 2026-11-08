use rusqlite::{params, OptionalExtension, TransactionBehavior};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

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
    DispatchReservationOperationId, DispatchReservationOutcome, DispatchReservationRecord,
    DispatchReservationReplayReceipt, DispatchReservationRequest, DispatchReservationStore,
    ReservedAttemptDispatchCapability,
};
use crate::provider_write::GitHubCommentContinuationAction;
use crate::{
    transition_side_effect_to_completed, EventId, EventSequenceNumber,
    GitHubPullRequestCommentProviderLookupObservation, GitHubPullRequestCommentTarget,
    IdempotencyKey, SideEffectCompleteTransitionInput, SideEffectLifecycleState,
    SideEffectOutcomeReference, SideEffectOutcomeReferenceKind, SideEffectRecord,
    SideEffectReference, SideEffectReferenceKind, SpecContentHash, Timestamp, WorkflowOsError,
    WorkflowOsErrorKind, WorkflowRun, WorkflowRunEvent, WorkflowRunEventKind, WorkflowRunSnapshot,
};

use super::continuity_codec::timestamp_parts;
use super::live_sandbox_grant_store::{
    check_dispatch_lane, read_live_effect_record, LiveGrantAdmission,
};
use super::{
    append_event_and_project_snapshot, encode_json, snapshot_commitment, SqliteStateBackend,
};
use super::{continuity_codec, continuity_store};

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
struct ProjectionBinding {
    event: DispatchAdmissionProjectionEvent,
    snapshot_commitment: crate::SpecContentHash,
}

/// Verified, reference-free execution posture of a synthetic sandbox reservation.
/// Neither variant grants permission to dispatch or retry.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SyntheticSandboxRecoveryPosture {
    /// A matching attempted record exists; no completion has been established.
    ReconciliationRequired,
    /// The completed record and its proof event match the exact admission.
    CompletionVerified,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum LiveCommentDispatchPosture {
    Acknowledged,
    RecoveryOnly,
}

// Only a trusted synthetic lookup may supply this result until a provider
// adapter can independently verify author and exact accepted payload.
#[allow(dead_code)] // No production transport is connected.
pub(super) enum LiveCommentLookup {
    Absent,
    Ambiguous,
    Unauthorized,
    Unavailable,
    Observed {
        comment: GitHubPullRequestCommentProviderLookupObservation,
        verified_action: SpecContentHash,
    },
}

#[derive(Debug, Eq, PartialEq)]
pub(super) enum LiveCommentRecovery {
    Absent,
    Ambiguous,
    Unauthorized,
    Unavailable,
    Mismatch,
    Observed(GitHubPullRequestCommentProviderLookupObservation),
}

fn live_comment_marker(action: &SpecContentHash, reservation: &SpecContentHash) -> String {
    let mut hash = Sha256::new();
    hash.update(b"workflow-os/live-comment-recovery/v1\0");
    hash.update(action.as_str().as_bytes());
    hash.update(b"\0");
    hash.update(reservation.as_str().as_bytes());
    format!("workflow-os/live-comment/{:x}", hash.finalize())
}

fn live_effect_commitment(
    domain: &[u8],
    first: &SpecContentHash,
    second: &SpecContentHash,
    effect: &SideEffectRecord,
) -> Result<SpecContentHash, WorkflowOsError> {
    let mut hash = Sha256::new();
    hash.update(domain);
    hash.update(serde_json::to_vec(&(first, second, effect)).map_err(|_| corrupt())?);
    SpecContentHash::new(format!("{:x}", hash.finalize()))
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
    fn reserve_attempt_dispatch(
        &self,
        request: DispatchReservationRequest,
    ) -> Result<DispatchReservationOutcome, WorkflowOsError> {
        self.reserve_attempt_dispatch_checked(request, None, |_, _, _| Ok(()))
    }
}

impl SqliteStateBackend {
    /// Inspects an existing synthetic reservation without granting authority,
    /// making provider calls, or mutating the database.
    ///
    /// # Errors
    ///
    /// Returns an error for missing, unsupported, or mismatched durable proof.
    pub fn inspect_synthetic_sandbox_recovery(
        &self,
        run_id: &crate::WorkflowRunId,
        operation_id: &str,
        side_effect_id: &crate::SideEffectId,
    ) -> Result<SyntheticSandboxRecoveryPosture, WorkflowOsError> {
        let operation = DispatchReservationOperationId::new(operation_id)?;
        let mut connection = self.existing_read_only_connection()?;
        let tx = connection
            .transaction()
            .map_err(|_| storage_error("begin_failed"))?;
        let admission = read_live_recovery_admission(&tx, &operation, run_id)?;
        let history = Self::read_events_with_connection(&tx, run_id)?;
        let effect = read_live_effect_record(&tx, side_effect_id)?;
        if effect.side_effect_id() != side_effect_id || effect.run_id() != run_id {
            return Err(not_dispatchable());
        }
        let proof_id = live_comment_proof_id(&admission)?;
        if let Some(event) = history.iter().find(|event| event.event_id == proof_id) {
            verify_live_comment_proof(&admission, &effect, event)?;
            return Ok(SyntheticSandboxRecoveryPosture::CompletionVerified);
        }
        verify_live_comment_attempt(&admission, &effect)?;
        Ok(SyntheticSandboxRecoveryPosture::ReconciliationRequired)
    }

    pub(crate) fn reserve_attempt_dispatch_with_lifecycle(
        &self,
        request: DispatchReservationRequest,
        lifecycle: &crate::trusted_host_workflow_lifecycle::TrustedHostWorkflowLifecycleContext,
    ) -> Result<DispatchReservationOutcome, WorkflowOsError> {
        self.reserve_attempt_dispatch_checked(request, Some(lifecycle), |_, _, _| Ok(()))
    }

    /// Runs a kernel-owned admission check on the locked, current state before
    /// writing a new reservation. Exact replay never calls the check or issues
    /// a capability; its receipt is for trusted callers, not caller authentication.
    /// The check may persist a grant-use binding but must not end the transaction
    /// or perform any external effect.
    pub(crate) fn reserve_attempt_dispatch_checked(
        &self,
        request: DispatchReservationRequest,
        lifecycle: Option<
            &crate::trusted_host_workflow_lifecycle::TrustedHostWorkflowLifecycleContext,
        >,
        check_admission: impl FnOnce(
            &rusqlite::Transaction<'_>,
            Timestamp,
            Timestamp,
        ) -> Result<(), WorkflowOsError>,
    ) -> Result<DispatchReservationOutcome, WorkflowOsError> {
        self.reserve_dispatch(request, lifecycle, None, check_admission)
    }

    /// Live-only admission. Exact replays and ambiguous commits yield no
    /// dispatch right or receipt through this entry point.
    #[allow(dead_code)] // The private live opening path is not connected yet.
    pub(super) fn reserve_live_sandbox_dispatch(
        &self,
        request: DispatchReservationRequest,
        admission: LiveGrantAdmission<'_>,
    ) -> Result<Option<Box<ReservedAttemptDispatchCapability>>, WorkflowOsError> {
        let outcome = self.reserve_dispatch(request, None, Some(admission), |_, _, _| Ok(()))?;
        Ok(match outcome {
            DispatchReservationOutcome::Admitted { capability } => Some(capability),
            DispatchReservationOutcome::AlreadyAdmitted { .. }
            | DispatchReservationOutcome::CommittedButCapabilityUnavailable { .. } => None,
        })
    }

    /// A fresh, committed reservation is the only path to the injected send.
    /// An acknowledgement is not durable execution proof.
    #[allow(dead_code)] // Private until synthetic dispatch is exposed.
    pub(super) fn dispatch_live_sandbox_comment(
        &self,
        request: DispatchReservationRequest,
        admission: LiveGrantAdmission<'_>,
        action: GitHubCommentContinuationAction<'_>,
        send: impl FnOnce(&GitHubPullRequestCommentTarget, &str, &str) -> bool,
    ) -> Result<LiveCommentDispatchPosture, WorkflowOsError> {
        let invocation = action.commitment()?;
        if invocation != request.invocation_commitment
            || admission.side_effect != action.side_effect
            || admission.body != action.body
        {
            return Err(not_dispatchable());
        }
        let Some(capability) = self.reserve_live_sandbox_dispatch(request, admission)? else {
            return Ok(LiveCommentDispatchPosture::RecoveryOnly);
        };
        let binding = (*capability)
            .into_parts()
            .attempt_capability
            .dispatch_reservation
            .ok_or_else(corrupt)?;
        let marker = live_comment_marker(&invocation, &binding.reservation_commitment);
        Ok(if send(action.target, action.body, &marker) {
            LiveCommentDispatchPosture::Acknowledged
        } else {
            LiveCommentDispatchPosture::RecoveryOnly
        })
    }

    /// Operator-only lookup. This validates historical admission, not current
    /// permission to write. It never consumes a grant or persists an outcome.
    #[allow(dead_code)] // Private until a reviewed recovery caller exists.
    pub(super) fn recover_live_sandbox_comment(
        &self,
        operation: &DispatchReservationOperationId,
        action: GitHubCommentContinuationAction<'_>,
        lookup: impl FnOnce(&GitHubPullRequestCommentTarget, &str) -> LiveCommentLookup,
    ) -> Result<LiveCommentRecovery, WorkflowOsError> {
        let invocation = action.commitment()?;
        let mut connection = self.existing_connection()?;
        let tx = connection
            .transaction()
            .map_err(|_| storage_error("begin_failed"))?;
        let admission = read_live_recovery_admission(&tx, operation, action.run)?;
        if admission.invocation_commitment() != &invocation {
            return Err(not_dispatchable());
        }
        let marker = live_comment_marker(&invocation, admission.reservation_commitment());
        tx.commit().map_err(|_| storage_error("commit_failed"))?;
        Ok(match lookup(action.target, &marker) {
            LiveCommentLookup::Absent => LiveCommentRecovery::Absent,
            LiveCommentLookup::Ambiguous => LiveCommentRecovery::Ambiguous,
            LiveCommentLookup::Unauthorized => LiveCommentRecovery::Unauthorized,
            LiveCommentLookup::Unavailable => LiveCommentRecovery::Unavailable,
            LiveCommentLookup::Observed {
                comment,
                verified_action,
            } if verified_action == invocation
                && comment.target() == action.target
                && comment.managed_marker() == Some(marker.as_str())
                && comment.provider_comment_reference().is_some()
                && comment.validate().is_ok() =>
            {
                LiveCommentRecovery::Observed(comment)
            }
            LiveCommentLookup::Observed { .. } => LiveCommentRecovery::Mismatch,
        })
    }

    /// An exact positive lookup is the only route to atomic completion.
    #[allow(dead_code)] // Private until synthetic proof is exposed.
    pub(super) fn reconcile_live_sandbox_comment(
        &self,
        operation: &DispatchReservationOperationId,
        action: GitHubCommentContinuationAction<'_>,
        lookup: impl FnOnce(&GitHubPullRequestCommentTarget, &str) -> LiveCommentLookup,
    ) -> Result<Option<EventId>, WorkflowOsError> {
        let LiveCommentRecovery::Observed(comment) =
            self.recover_live_sandbox_comment(operation, action, lookup)?
        else {
            return Ok(None);
        };
        let invocation = action.commitment()?;
        let mut connection = self.existing_connection()?;
        let tx = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| storage_error("begin_failed"))?;
        let admission = read_live_recovery_admission(&tx, operation, action.run)?;
        if admission.invocation_commitment() != &invocation
            || comment.managed_marker()
                != Some(
                    live_comment_marker(&invocation, admission.reservation_commitment()).as_str(),
                )
        {
            return Err(not_dispatchable());
        }
        let outcome = SideEffectOutcomeReference::new(
            SideEffectOutcomeReferenceKind::Outcome,
            comment
                .provider_comment_reference()
                .ok_or_else(not_dispatchable)?,
        )?;
        let id = persist_live_comment_outcome(&tx, action, &admission, outcome)?;
        tx.commit().map_err(|_| storage_error("commit_failed"))?;
        Ok(Some(id))
    }

    #[allow(clippy::too_many_lines)]
    fn reserve_dispatch(
        &self,
        mut request: DispatchReservationRequest,
        lifecycle: Option<
            &crate::trusted_host_workflow_lifecycle::TrustedHostWorkflowLifecycleContext,
        >,
        live: Option<LiveGrantAdmission<'_>>,
        check_admission: impl FnOnce(
            &rusqlite::Transaction<'_>,
            Timestamp,
            Timestamp,
        ) -> Result<(), WorkflowOsError>,
    ) -> Result<DispatchReservationOutcome, WorkflowOsError> {
        let expected_request = request_commitment(&request);
        if request.request_commitment != expected_request
            || request.workflow_lifecycle_commitment
                != lifecycle.map(crate::trusted_host_workflow_lifecycle::TrustedHostWorkflowLifecycleContext::commitment)
        {
            return Err(reservation_error(
                WorkflowOsErrorKind::Validation,
                "request.invalid",
                "dispatch reservation request is invalid",
            ));
        }
        let mut connection = self.connection()?;
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| storage_error("begin_failed"))?;
        check_dispatch_lane(
            &transaction,
            &request.capability.window_id,
            &request.capability.attempt_id,
            live.is_some(),
        )?;
        let observation = continuity_store::observe_continuity_trusted_time()?;
        let live = live
            .map(|admission| {
                admission.authenticate(&transaction, &request, observation.observed_at())
            })
            .transpose()?;

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
            || (live.is_some()
                && (observation.epoch_id() != &window.trusted_time_epoch_id
                    || observation.observed_at() < window.trusted_time_watermark))
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
        check_admission(&transaction, observation.observed_at(), window.expires_at)?;
        let attempted_effect = if let Some(admission) = live {
            Some(admission.consume(
                &transaction,
                &request,
                observation.observed_at(),
                window.expires_at,
            )?)
        } else {
            None
        };
        let next_sequence = crate::EventSequenceNumber::new(
            last.sequence_number
                .get()
                .checked_add(1)
                .ok_or_else(corrupt)?,
        )?;
        let event_id = dispatch_admission_event_id(&request.operation_id)?;
        let terminal_sequence = if lifecycle.is_some() {
            crate::EventSequenceNumber::new(
                next_sequence.get().checked_add(1).ok_or_else(corrupt)?,
            )?
        } else {
            next_sequence
        };
        let terminal_event_id = if let Some(lifecycle) = lifecycle {
            lifecycle.event_id("invocation-started", request.operation_id.as_str())?
        } else {
            event_id.clone()
        };
        let admission_cursor =
            DispatchAdmissionProjectionCursor::new(next_sequence, event_id.clone());
        let trusted_commitment = trusted_time_commitment(&observation);
        let next_revision = request.capability.window_revision.checked_next()?;
        let reservation_binding = if let Some(effect) = &attempted_effect {
            live_effect_commitment(
                b"workflow-os/live-comment-attempt/v1\0",
                &request.request_commitment,
                &trusted_commitment,
                effect,
            )?
        } else {
            reservation_commitment(&request, attempt.revision, &trusted_commitment)
        };
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
                sequence_number: terminal_sequence,
                event_id: terminal_event_id.clone(),
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
        let projection_snapshot_binding = snapshot_commitment(&snapshot_payload);
        if let Some(lifecycle) = lifecycle {
            append_event_and_project_snapshot(
                &transaction,
                &WorkflowRunEvent {
                    sequence_number: terminal_sequence,
                    event_id: terminal_event_id,
                    timestamp: observation.observed_at(),
                    run_id: request.expected_window_binding.run_id.clone(),
                    workflow_id: request.expected_window_binding.workflow_id.clone(),
                    schema_version: last.schema_version.clone(),
                    workflow_version: last.workflow_version.clone(),
                    spec_content_hash: last.spec_content_hash.clone(),
                    correlation_id: None,
                    actor: None,
                    idempotency_key: Some(lifecycle.idempotency_key("invocation-started")?),
                    kind: lifecycle.started_event(&request.capability.attempt_id)?,
                },
            )?;
        }
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
        insert_reservation(
            &transaction,
            &request.expected_window_binding.run_id,
            &record,
            &event,
        )?;
        insert_projection(
            &transaction,
            &record,
            &ProjectionBinding {
                event: event.clone(),
                snapshot_commitment: projection_snapshot_binding,
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

fn read_live_recovery_admission(
    tx: &rusqlite::Transaction<'_>,
    operation: &DispatchReservationOperationId,
    run_id: &crate::WorkflowRunId,
) -> Result<DispatchAdmissionProjectionEvent, WorkflowOsError> {
    let (record, binding) = read_existing(tx, operation.as_str())?.ok_or_else(not_dispatchable)?;
    check_dispatch_lane(tx, &record.window_id, &record.attempt_id, true)?;
    validate_existing_projection(tx, &record, &binding)?;
    let has_exact_live_use: bool = tx
        .query_row(
            "SELECT EXISTS(
           SELECT 1 FROM events e JOIN live_sandbox_grant_uses u ON u.attempt_id=?3
           JOIN live_sandbox_grants g ON g.grant_id=u.grant_id AND g.attempt_id=u.attempt_id
           WHERE e.event_id=?1 AND e.run_id=?2 AND g.binding=?4
         )",
            params![
                binding.event.committed_result_cursor().event_id().as_str(),
                run_id.as_str(),
                record.attempt_id.as_str(),
                binding.event.executor_commitment().as_str(),
            ],
            |row| row.get(0),
        )
        .map_err(|_| corrupt())?;
    if !has_exact_live_use {
        return Err(not_dispatchable());
    }
    Ok(binding.event)
}

fn live_comment_proof_id(
    admission: &DispatchAdmissionProjectionEvent,
) -> Result<EventId, WorkflowOsError> {
    EventId::new(format!(
        "live-comment-proof/{}",
        admission.reservation_commitment()
    ))
}

fn verify_live_comment_attempt(
    admission: &DispatchAdmissionProjectionEvent,
    effect: &SideEffectRecord,
) -> Result<(), WorkflowOsError> {
    let attempted = live_effect_commitment(
        b"workflow-os/live-comment-attempt/v1\0",
        admission.request_commitment(),
        admission.trusted_time_commitment(),
        effect,
    )?;
    if effect.lifecycle_state() != SideEffectLifecycleState::Attempted
        || &attempted != admission.reservation_commitment()
    {
        return Err(not_dispatchable());
    }
    Ok(())
}

fn verify_live_comment_proof(
    admission: &DispatchAdmissionProjectionEvent,
    effect: &SideEffectRecord,
    event: &WorkflowRunEvent,
) -> Result<(), WorkflowOsError> {
    let expected = crate::side_effect::transition_event(effect, 0)?;
    let admission_event = admission.committed_result_cursor();
    if effect.lifecycle_state() != SideEffectLifecycleState::Completed
        || !effect
            .outcome_reference()
            .is_some_and(|outcome| outcome.kind() == SideEffectOutcomeReferenceKind::Outcome)
        || event.sequence_number <= admission_event.sequence_number()
        || Some(event.timestamp) != effect.updated_at()
        || event.idempotency_key.as_ref() != Some(&live_comment_proof_key(admission, effect)?)
        || !effect.references().iter().any(|reference| {
            reference.kind() == SideEffectReferenceKind::WorkflowEvent
                && reference.reference() == admission_event.event_id().as_str()
        })
        || !matches!(&event.kind, WorkflowRunEventKind::SideEffectCompleted(payload) if payload.as_ref() == &expected)
    {
        return Err(not_dispatchable());
    }
    Ok(())
}

fn live_comment_proof_key(
    admission: &DispatchAdmissionProjectionEvent,
    effect: &SideEffectRecord,
) -> Result<IdempotencyKey, WorkflowOsError> {
    let proof = live_effect_commitment(
        b"workflow-os/live-comment-proof/v1\0",
        admission.invocation_commitment(),
        admission.reservation_commitment(),
        effect,
    )?;
    IdempotencyKey::new(format!("live-comment-proof/{proof}"))
}

fn persist_live_comment_outcome(
    tx: &rusqlite::Transaction<'_>,
    action: GitHubCommentContinuationAction<'_>,
    admission: &DispatchAdmissionProjectionEvent,
    outcome: SideEffectOutcomeReference,
) -> Result<EventId, WorkflowOsError> {
    let history = SqliteStateBackend::read_events_with_connection(tx, action.run)?;
    let run = WorkflowRun::rehydrate(&history)?;
    let identity = &run.snapshot.identity;
    let effect = read_live_effect_record(tx, action.side_effect)?;
    let id = live_comment_proof_id(admission)?;
    let admission_event = admission.committed_result_cursor().event_id();
    if effect.side_effect_id() != action.side_effect
        || effect.run_id() != action.run
        || effect.workflow_id() != &identity.workflow_id
        || effect.workflow_version() != &identity.workflow_version
        || effect.schema_version() != &identity.schema_version
        || effect.spec_hash() != &identity.spec_content_hash
        || effect.step_id() != Some(action.step)
        || effect.actor() != Some(action.original_actor)
        || effect.target().reference() != action.target.reference()
        || effect.idempotency().key() != action.idempotency_key
    {
        return Err(not_dispatchable());
    }
    if let Some(event) = history.iter().find(|event| event.event_id == id) {
        if effect.outcome_reference() != Some(&outcome) {
            return Err(not_dispatchable());
        }
        verify_live_comment_proof(admission, &effect, event)?;
        return Ok(id);
    }
    verify_live_comment_attempt(admission, &effect)?;
    let last = history.last().ok_or_else(corrupt)?;
    let now = continuity_store::observe_continuity_trusted_time()?.observed_at();
    if now < last.timestamp {
        return Err(not_dispatchable());
    }
    let transition = transition_side_effect_to_completed(SideEffectCompleteTransitionInput {
        prior_record: &effect,
        transitioned_at: now,
        outcome_reference: outcome,
        summary: None,
        additional_references: vec![SideEffectReference::new(
            SideEffectReferenceKind::WorkflowEvent,
            admission_event.as_str(),
        )?],
        evidence_reference_count: 0,
    })?;
    let updated = transition.record();
    let changed = tx.execute(
        "UPDATE side_effect_records SET payload=?1 WHERE side_effect_id=?2 AND run_id=?3 AND workflow_id=?4",
        params![encode_json(updated, "live comment outcome")?, action.side_effect.as_str(),
            action.run.as_str(), identity.workflow_id.as_str()],
    ).map_err(|_| storage_error("write_failed"))?;
    if changed != 1 {
        return Err(corrupt());
    }
    append_event_and_project_snapshot(
        tx,
        &WorkflowRunEvent {
            sequence_number: EventSequenceNumber::new(
                last.sequence_number
                    .get()
                    .checked_add(1)
                    .ok_or_else(corrupt)?,
            )?,
            event_id: id.clone(),
            timestamp: now,
            run_id: identity.run_id.clone(),
            workflow_id: identity.workflow_id.clone(),
            schema_version: identity.schema_version.clone(),
            workflow_version: identity.workflow_version.clone(),
            spec_content_hash: identity.spec_content_hash.clone(),
            correlation_id: updated.correlation_id().cloned(),
            actor: None,
            idempotency_key: Some(live_comment_proof_key(admission, updated)?),
            kind: WorkflowRunEventKind::SideEffectCompleted(Box::new(transition.event().clone())),
        },
    )?;
    Ok(id)
}

fn insert_reservation(
    transaction: &rusqlite::Transaction<'_>,
    run_id: &crate::WorkflowRunId,
    record: &DispatchReservationRecord,
    event: &DispatchAdmissionProjectionEvent,
) -> Result<(), WorkflowOsError> {
    let (seconds, nanos) = timestamp_parts(record.committed_at);
    let sequence = i64::try_from(event.committed_result_cursor().sequence_number().get())
        .map_err(|_| corrupt())?;
    transaction
        .execute(
            "INSERT INTO dispatch_reservations
             (attempt_id,window_id,run_id,operation_id,receipt_id,request_commitment,
              reservation_commitment,result_event_id,result_sequence,committed_seconds,
              committed_nanos,record_json)
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12)",
            params![
                record.attempt_id.as_str(),
                record.window_id.as_str(),
                run_id.as_str(),
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
