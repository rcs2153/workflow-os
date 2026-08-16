use std::collections::BTreeMap;

use crate::{AuthorizedExecutionAttemptOutcome, Timestamp, WorkflowOsError, WorkflowOsErrorKind};

use super::internal::{
    continuity_state_error, AuthoritativeAttemptRecord, AuthoritativeAttemptState,
    AuthoritativeContinuationDisposition, AuthoritativeWaitIdentity, AuthoritativeWaitRecord,
    AuthoritativeWaitState, AuthoritativeWindowRecord, AuthoritativeWindowState,
    CommittedSecurityRejectionKind, ContinuityInstanceEligibility, ContinuityRevision,
    ContinuityTrustedTimeEpochId, ExpectedWindowBinding, TrustedTimeObservation,
    TrustedTimePosture, TrustedTimeSecurityRecord,
};

fn semantic_error(kind: WorkflowOsErrorKind, suffix: &'static str) -> WorkflowOsError {
    continuity_state_error(
        kind,
        suffix,
        "authorized execution continuity state operation failed",
    )
}

/// One checked attempt-number allocation from an authoritative window counter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct AttemptAllocation {
    pub(crate) attempt_number: u32,
    pub(crate) next_attempt_number: u32,
}

/// Backend-neutral loaded rows used for trusted-time security evaluation.
#[derive(Clone, Copy)]
pub(crate) struct SecuritySemanticSnapshot<'a> {
    pub(crate) trusted_time: &'a TrustedTimeSecurityRecord,
    pub(crate) window: &'a AuthoritativeWindowRecord,
}

/// Backend-neutral loaded rows used for an attempt/window transition.
#[derive(Clone, Copy)]
pub(crate) struct AttemptTransitionSnapshot<'a> {
    pub(crate) attempt: &'a AuthoritativeAttemptRecord,
    pub(crate) window: &'a AuthoritativeWindowRecord,
}

/// Exact mutable columns produced by an attempt/window transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct AttemptWindowWriteSet {
    pub(crate) attempt_state: AuthoritativeAttemptState,
    pub(crate) attempt_revision: ContinuityRevision,
    pub(crate) window_state: AuthoritativeWindowState,
    pub(crate) window_watermark: Timestamp,
    pub(crate) window_revision: ContinuityRevision,
}

/// Allocates one attempt without permitting budget or integer overflow.
pub(crate) fn allocate_attempt(
    next_attempt_number: u32,
    maximum_attempts: u32,
) -> Result<AttemptAllocation, WorkflowOsError> {
    if next_attempt_number == 0 || maximum_attempts == 0 || next_attempt_number > maximum_attempts {
        return Err(semantic_error(
            WorkflowOsErrorKind::InvalidState,
            "attempt.budget_exhausted",
        ));
    }
    let successor = next_attempt_number.checked_add(1).ok_or_else(|| {
        semantic_error(
            WorkflowOsErrorKind::InvalidState,
            "attempt.number_exhausted",
        )
    })?;
    Ok(AttemptAllocation {
        attempt_number: next_attempt_number,
        next_attempt_number: successor,
    })
}

/// Validates the immutable, revision, cursor, epoch, and time binding of a window.
pub(crate) fn validate_window(
    window: &AuthoritativeWindowRecord,
    expected_binding: &ExpectedWindowBinding,
    expected_revision: ContinuityRevision,
    expected_cursor: &super::internal::ContinuityCursor,
    expected_epoch_id: &ContinuityTrustedTimeEpochId,
    observed_at: Timestamp,
) -> Result<(), WorkflowOsError> {
    if window.workflow_id != expected_binding.workflow_id
        || window.run_id != expected_binding.run_id
        || window.step_id != expected_binding.step_id
        || window.subject_actor_id != expected_binding.subject_actor_id
        || window.immutable_run_bundle != expected_binding.immutable_run_bundle
        || window.governance_commitment != expected_binding.governance_commitment
        || window.authority_commitment != expected_binding.authority_commitment
        || window.cursor != expected_binding.cursor
    {
        return Err(semantic_error(
            WorkflowOsErrorKind::Security,
            "window.binding_mismatch",
        ));
    }
    if window.revision != expected_revision {
        return Err(semantic_error(
            WorkflowOsErrorKind::InvalidState,
            "window.revision_stale",
        ));
    }
    if &window.cursor != expected_cursor {
        return Err(semantic_error(
            WorkflowOsErrorKind::InvalidState,
            "cursor.stale",
        ));
    }
    if &window.trusted_time_epoch_id != expected_epoch_id {
        return Err(semantic_error(
            WorkflowOsErrorKind::Security,
            "time.epoch_mismatch",
        ));
    }
    if observed_at < window.trusted_time_watermark {
        return Err(semantic_error(
            WorkflowOsErrorKind::InvalidState,
            "time.regressed",
        ));
    }
    if observed_at >= window.expires_at {
        return Err(semantic_error(
            WorkflowOsErrorKind::InvalidState,
            "time.expired",
        ));
    }
    Ok(())
}

/// Classifies the trusted-time security outcome after static authority checks.
pub(crate) fn classify_security_rejection(
    snapshot: SecuritySemanticSnapshot<'_>,
    expected_provenance: &crate::SpecContentHash,
    observation: &TrustedTimeObservation,
) -> Option<CommittedSecurityRejectionKind> {
    let security = snapshot.trusted_time;
    let window = snapshot.window;
    let observed_at = observation.observed_at();
    if observation.source() != security.source
        || observation.provenance_commitment() != expected_provenance
    {
        Some(CommittedSecurityRejectionKind::Untrusted)
    } else if observation.epoch_id() != &security.epoch_id
        || observation.epoch_id() != &window.trusted_time_epoch_id
    {
        Some(CommittedSecurityRejectionKind::EpochMismatch)
    } else if security
        .last_observed_at
        .is_some_and(|last| observed_at < last)
        || observed_at < window.trusted_time_watermark
    {
        Some(CommittedSecurityRejectionKind::Regressed)
    } else if observed_at >= window.expires_at {
        Some(CommittedSecurityRejectionKind::Expired)
    } else {
        None
    }
}

/// Evaluates an ordinary outcome without mutating backend-owned rows.
pub(crate) fn attempt_outcome_write_set(
    snapshot: AttemptTransitionSnapshot<'_>,
    outcome: AuthorizedExecutionAttemptOutcome,
    observed_at: Timestamp,
) -> Result<AttemptWindowWriteSet, WorkflowOsError> {
    let attempt_state = match outcome {
        AuthorizedExecutionAttemptOutcome::Succeeded => AuthoritativeAttemptState::Succeeded,
        AuthorizedExecutionAttemptOutcome::RetryableFailure => {
            AuthoritativeAttemptState::RetryableFailure
        }
        AuthorizedExecutionAttemptOutcome::TerminalFailure => {
            AuthoritativeAttemptState::TerminalFailure
        }
        AuthorizedExecutionAttemptOutcome::AmbiguousMayHaveStarted => {
            return Err(semantic_error(
                WorkflowOsErrorKind::Validation,
                "input.invalid",
            ));
        }
    };
    Ok(AttemptWindowWriteSet {
        attempt_state,
        attempt_revision: snapshot.attempt.revision.checked_next()?,
        window_state: AuthoritativeWindowState::Closed,
        window_watermark: observed_at,
        window_revision: snapshot.window.revision.checked_next()?,
    })
}

/// Evaluates ambiguity recovery without mutating backend-owned rows.
pub(crate) fn ambiguity_recovery_write_set(
    snapshot: AttemptTransitionSnapshot<'_>,
    observed_at: Timestamp,
) -> Result<AttemptWindowWriteSet, WorkflowOsError> {
    Ok(AttemptWindowWriteSet {
        attempt_state: AuthoritativeAttemptState::AmbiguousMayHaveStarted,
        attempt_revision: snapshot.attempt.revision.checked_next()?,
        window_state: AuthoritativeWindowState::RecoveryRequired,
        window_watermark: observed_at,
        window_revision: snapshot.window.revision.checked_next()?,
    })
}

fn apply_attempt_window_write_set(
    attempt: &mut AuthoritativeAttemptRecord,
    window: &mut AuthoritativeWindowRecord,
    write_set: AttemptWindowWriteSet,
) {
    attempt.state = write_set.attempt_state;
    attempt.revision = write_set.attempt_revision;
    window.state = write_set.window_state;
    window.trusted_time_watermark = write_set.window_watermark;
    window.revision = write_set.window_revision;
}

/// Applies the shared attempt outcome transition to loaded authoritative rows.
pub(crate) fn apply_attempt_outcome(
    attempt: &mut AuthoritativeAttemptRecord,
    window: &mut AuthoritativeWindowRecord,
    outcome: AuthorizedExecutionAttemptOutcome,
    observed_at: Timestamp,
) -> Result<AuthoritativeAttemptState, WorkflowOsError> {
    let write_set = attempt_outcome_write_set(
        AttemptTransitionSnapshot { attempt, window },
        outcome,
        observed_at,
    )?;
    let state = write_set.attempt_state;
    apply_attempt_window_write_set(attempt, window, write_set);
    Ok(state)
}

/// Applies the shared ambiguity-recovery transition to loaded rows.
pub(crate) fn apply_ambiguity_recovery(
    attempt: &mut AuthoritativeAttemptRecord,
    window: &mut AuthoritativeWindowRecord,
    observed_at: Timestamp,
) -> Result<(), WorkflowOsError> {
    let write_set =
        ambiguity_recovery_write_set(AttemptTransitionSnapshot { attempt, window }, observed_at)?;
    apply_attempt_window_write_set(attempt, window, write_set);
    Ok(())
}

/// Produces the kernel-owned liveness classification from loaded records.
pub(crate) fn continuation_disposition(
    trusted_time: &TrustedTimeSecurityRecord,
    window: &AuthoritativeWindowRecord,
    waits: &BTreeMap<AuthoritativeWaitIdentity, AuthoritativeWaitRecord>,
    active_wait_ids: Option<&[AuthoritativeWaitIdentity]>,
    observed_at: Option<Timestamp>,
) -> Result<AuthoritativeContinuationDisposition, WorkflowOsError> {
    if matches!(
        window.state,
        AuthoritativeWindowState::Closed
            | AuthoritativeWindowState::Revoked
            | AuthoritativeWindowState::Superseded
    ) {
        return Ok(AuthoritativeContinuationDisposition::Terminal);
    }
    if window.state == AuthoritativeWindowState::Expired {
        return if window.trusted_time_watermark >= window.expires_at {
            Ok(AuthoritativeContinuationDisposition::Terminal)
        } else {
            Err(semantic_error(
                WorkflowOsErrorKind::InvalidState,
                "state.corrupt",
            ))
        };
    }
    let time_is_eligible = trusted_time.eligibility
        == ContinuityInstanceEligibility::LiveStateEligible
        && trusted_time.posture != TrustedTimePosture::Quarantined
        && trusted_time.epoch_id == window.trusted_time_epoch_id
        && observed_at.is_some_and(|now| {
            trusted_time
                .last_observed_at
                .map_or(true, |last| now >= last)
                && now >= window.trusted_time_watermark
                && now < window.expires_at
        });
    if !time_is_eligible {
        return Ok(AuthoritativeContinuationDisposition::Blocked);
    }
    match window.state {
        AuthoritativeWindowState::Yielded => {
            let wait_ids = active_wait_ids.ok_or_else(|| {
                semantic_error(WorkflowOsErrorKind::InvalidState, "state.corrupt")
            })?;
            let mut unsatisfied = false;
            for identity in wait_ids {
                let wait = waits.get(identity).ok_or_else(|| {
                    semantic_error(WorkflowOsErrorKind::InvalidState, "state.corrupt")
                })?;
                if wait.window_id != window.window_id
                    || wait.condition_id != identity.condition_id
                    || wait.condition_version != identity.condition_version
                {
                    return Err(semantic_error(
                        WorkflowOsErrorKind::InvalidState,
                        "state.corrupt",
                    ));
                }
                match wait.state {
                    AuthoritativeWaitState::Unsatisfied => unsatisfied = true,
                    AuthoritativeWaitState::Satisfied => {}
                    AuthoritativeWaitState::Expired
                    | AuthoritativeWaitState::Superseded
                    | AuthoritativeWaitState::Canceled => {
                        return Ok(AuthoritativeContinuationDisposition::Blocked);
                    }
                }
            }
            Ok(if unsatisfied {
                AuthoritativeContinuationDisposition::AwaitCondition
            } else {
                AuthoritativeContinuationDisposition::ResumeNow
            })
        }
        AuthoritativeWindowState::Executing
        | AuthoritativeWindowState::AssessmentRequired
        | AuthoritativeWindowState::RecoveryRequired => {
            Ok(AuthoritativeContinuationDisposition::Blocked)
        }
        AuthoritativeWindowState::Closed
        | AuthoritativeWindowState::Expired
        | AuthoritativeWindowState::Revoked
        | AuthoritativeWindowState::Superseded => {
            Ok(AuthoritativeContinuationDisposition::Terminal)
        }
    }
}
