use std::fmt;

use rusqlite::OptionalExtension;

use crate::authorized_execution_continuity_state::internal::{
    AuthoritativeContinuationDisposition, AuthoritativeOperationRecord,
    AuthoritativeWaitDependencyBinding, AuthoritativeWaitIdentity, AuthoritativeWaitState,
    AuthorizedExecutionContinuityStore, CommittedOperationDisposition, ContinuityOperationId,
    ContinuityReceiptId, ExpectedWindowBinding, MutationResult, RecordedOperationResult,
    TimeWindowTransitionRequest,
};
use crate::{
    AuthorizedExecutionContinuityOperationKind, AuthorizedExecutionWaitConditionId,
    AuthorizedExecutionWakeTriggerKind, SpecContentHash, WorkflowOsError, WorkflowOsErrorKind,
};

use super::trusted_host_operational_entry::TrustedHostOperationalEntryLocator;
use super::trusted_host_wait_handoff::validate_trusted_host_wait_handoff_commitment;
use super::{continuity_codec, SqliteStateBackend};

pub(crate) struct TrustedHostTimeWindowWakeInput<'a> {
    pub(crate) backend: &'a SqliteStateBackend,
    pub(crate) locator: TrustedHostOperationalEntryLocator,
    pub(crate) condition_id: AuthorizedExecutionWaitConditionId,
    pub(crate) condition_version: u32,
    pub(crate) operation_id: ContinuityOperationId,
    pub(crate) receipt_id: ContinuityReceiptId,
    pub(crate) handoff_commitment: Option<SpecContentHash>,
}

impl fmt::Debug for TrustedHostTimeWindowWakeInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostTimeWindowWakeInput")
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TrustedHostTimeWindowWakeStatus {
    Transitioned,
    ExactReplay,
    SecurityRejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct TrustedHostTimeWindowWakeOutcome {
    pub(crate) status: TrustedHostTimeWindowWakeStatus,
    pub(crate) disposition: AuthoritativeContinuationDisposition,
}

pub(crate) fn apply_trusted_host_time_window_wake(
    input: TrustedHostTimeWindowWakeInput<'_>,
) -> Result<TrustedHostTimeWindowWakeOutcome, WorkflowOsError> {
    let backend = input.backend;
    apply_with_verifier(input, |request| {
        backend.transition_time_window_wait(request)
    })
}

fn apply_with_verifier(
    input: TrustedHostTimeWindowWakeInput<'_>,
    verifier: impl FnOnce(&TimeWindowTransitionRequest) -> Result<MutationResult, WorkflowOsError>,
) -> Result<TrustedHostTimeWindowWakeOutcome, WorkflowOsError> {
    let state = continuity_codec::load_snapshot(&input.backend.connection()?)?;
    let window = state
        .windows
        .get(&input.locator.window_id)
        .ok_or_else(caller_corrupt)?;
    validate_locator(&input.locator, window)?;

    if let Some(operation) = state.operations.get(&input.operation_id) {
        validate_replay_identity(&input, operation)?;
        return bounded_outcome(
            input.backend,
            &input.locator.window_id,
            replay_status(operation)?,
        );
    }

    if let Some(commitment) = &input.handoff_commitment {
        validate_trusted_host_wait_handoff_commitment(&state, &input.locator, commitment)?;
    }

    let wait = state
        .waits
        .get(&AuthoritativeWaitIdentity::new(
            input.condition_id.clone(),
            input.condition_version,
        ))
        .ok_or_else(caller_corrupt)?;
    if wait.window_id != input.locator.window_id
        || wait.state != AuthoritativeWaitState::Unsatisfied
        || wait.wake_trigger != AuthorizedExecutionWakeTriggerKind::DeadlineReached
        || !matches!(
            wait.dependency_binding,
            Some(AuthoritativeWaitDependencyBinding::TimeWindow { .. })
        )
    {
        return Err(caller_error(
            WorkflowOsErrorKind::InvalidState,
            "wait_ineligible",
            "trusted-host TimeWindow wait is not eligible",
        ));
    }

    let request = TimeWindowTransitionRequest {
        operation_id: input.operation_id,
        receipt_id: input.receipt_id,
        window_id: input.locator.window_id.clone(),
        expected_window_revision: window.revision,
        expected_window_binding: ExpectedWindowBinding {
            workflow_id: window.workflow_id.clone(),
            run_id: window.run_id.clone(),
            step_id: window.step_id.clone(),
            subject_actor_id: window.subject_actor_id.clone(),
            immutable_run_bundle: window.immutable_run_bundle.clone(),
            governance_commitment: window.governance_commitment.clone(),
            authority_commitment: window.authority_commitment.clone(),
            cursor: window.cursor.clone(),
        },
        cursor: window.cursor.clone(),
        condition_id: input.condition_id,
        expected_generation_id: wait.generation_id.clone(),
        expected_condition_version: input.condition_version,
        expected_wait_revision: wait.revision,
        handoff_commitment: input.handoff_commitment,
    };
    let status = transition_status(&verifier(&request)?)?;
    bounded_outcome(input.backend, &request.window_id, status)
}

#[cfg(test)]
pub(super) fn apply_trusted_host_time_window_wake_with_verifier(
    input: TrustedHostTimeWindowWakeInput<'_>,
    verifier: impl FnOnce(&TimeWindowTransitionRequest) -> Result<MutationResult, WorkflowOsError>,
) -> Result<TrustedHostTimeWindowWakeOutcome, WorkflowOsError> {
    apply_with_verifier(input, verifier)
}

fn validate_locator(
    locator: &TrustedHostOperationalEntryLocator,
    window: &crate::authorized_execution_continuity_state::internal::AuthoritativeWindowRecord,
) -> Result<(), WorkflowOsError> {
    if window.workflow_id != locator.workflow_id
        || window.run_id != locator.run_id
        || window.step_id != locator.step_id
        || window.window_id != locator.window_id
        || window.subject_actor_id != locator.subject_actor_id
        || window.immutable_run_bundle != locator.immutable_run_bundle
    {
        return Err(caller_error(
            WorkflowOsErrorKind::Security,
            "window_binding_mismatch",
            "trusted-host TimeWindow caller binding is invalid",
        ));
    }
    Ok(())
}

fn validate_replay_identity(
    input: &TrustedHostTimeWindowWakeInput<'_>,
    operation: &AuthoritativeOperationRecord,
) -> Result<(), WorkflowOsError> {
    let row = input
        .backend
        .connection()?
        .query_row(
            "SELECT receipt_id, operation_kind, request_window_id, request_wait_condition_id, request_wait_condition_version, request_json FROM continuity_operations WHERE operation_id=?1",
            [input.operation_id.as_str()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, Option<String>>(3)?,
                    row.get::<_, Option<u32>>(4)?,
                    row.get::<_, String>(5)?,
                ))
            },
        )
        .optional()
        .map_err(|_| caller_corrupt())?
        .ok_or_else(caller_corrupt)?;
    if operation.operation_kind != AuthorizedExecutionContinuityOperationKind::TransitionWait
        || operation.receipt.receipt_id != input.receipt_id
        || row.0 != input.receipt_id.as_str()
        || row.1 != "transition_wait"
        || row.2 != input.locator.window_id.as_str()
        || row.3.as_deref() != Some(input.condition_id.as_str())
        || row.4 != Some(input.condition_version)
    {
        return Err(caller_error(
            WorkflowOsErrorKind::Security,
            "operation_replay_conflict",
            "trusted-host TimeWindow operation replay conflicts with durable state",
        ));
    }
    let envelope: continuity_codec::RequestEnvelope = continuity_codec::decode(&row.5)?;
    let expected_domain = if input.handoff_commitment.is_some() {
        "workflow-os/authorized-execution-continuity/transition_time_window_wait_handoff/v1"
    } else {
        "workflow-os/authorized-execution-continuity/transition_time_window_wait/v1"
    };
    let handoff_matches = match &input.handoff_commitment {
        Some(commitment) => envelope.fields.last().map(String::as_str) == Some(commitment.as_str()),
        None => envelope.fields.len() == 4,
    };
    if envelope.domain != expected_domain
        || envelope.window_id != input.locator.window_id.as_str()
        || envelope.wait_condition_id.as_deref() != Some(input.condition_id.as_str())
        || envelope.wait_condition_version != Some(input.condition_version)
        || !handoff_matches
    {
        return Err(caller_error(
            WorkflowOsErrorKind::Security,
            "operation_replay_conflict",
            "trusted-host TimeWindow operation replay conflicts with durable state",
        ));
    }
    Ok(())
}

fn replay_status(
    operation: &AuthoritativeOperationRecord,
) -> Result<TrustedHostTimeWindowWakeStatus, WorkflowOsError> {
    match &operation.disposition {
        CommittedOperationDisposition::CommittedSuccess(
            RecordedOperationResult::WaitTransitioned { .. },
        ) => Ok(TrustedHostTimeWindowWakeStatus::ExactReplay),
        CommittedOperationDisposition::CommittedSecurityRejection(_) => {
            Ok(TrustedHostTimeWindowWakeStatus::SecurityRejected)
        }
        CommittedOperationDisposition::CommittedSuccess(_) => Err(caller_corrupt()),
    }
}

fn transition_status(
    result: &MutationResult,
) -> Result<TrustedHostTimeWindowWakeStatus, WorkflowOsError> {
    match result {
        MutationResult::Recorded(RecordedOperationResult::WaitTransitioned { .. }) => {
            Ok(TrustedHostTimeWindowWakeStatus::Transitioned)
        }
        MutationResult::ExactReplay(CommittedOperationDisposition::CommittedSuccess(
            RecordedOperationResult::WaitTransitioned { .. },
        )) => Ok(TrustedHostTimeWindowWakeStatus::ExactReplay),
        MutationResult::SecurityRejected(_)
        | MutationResult::ExactReplay(CommittedOperationDisposition::CommittedSecurityRejection(
            _,
        )) => Ok(TrustedHostTimeWindowWakeStatus::SecurityRejected),
        _ => Err(caller_corrupt()),
    }
}

fn bounded_outcome(
    backend: &SqliteStateBackend,
    window_id: &crate::AuthorizedExecutionWindowId,
    status: TrustedHostTimeWindowWakeStatus,
) -> Result<TrustedHostTimeWindowWakeOutcome, WorkflowOsError> {
    Ok(TrustedHostTimeWindowWakeOutcome {
        status,
        disposition: backend.continuation_disposition(window_id)?,
    })
}

fn caller_corrupt() -> WorkflowOsError {
    caller_error(
        WorkflowOsErrorKind::InvalidState,
        "state_corrupt",
        "trusted-host TimeWindow caller state is invalid",
    )
}

fn caller_error(
    kind: WorkflowOsErrorKind,
    suffix: &'static str,
    message: &'static str,
) -> WorkflowOsError {
    WorkflowOsError::new(
        kind,
        format!("trusted_host_time_window_caller.{suffix}"),
        message,
    )
}
