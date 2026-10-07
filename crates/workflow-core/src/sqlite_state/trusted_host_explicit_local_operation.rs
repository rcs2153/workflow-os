use std::fmt;

use crate::WorkflowOsError;

use super::trusted_host_local_production_caller::{
    run_trusted_host_local_production_caller, TrustedHostLocalProductionCallerInput,
};
use super::trusted_host_local_timer::TrustedHostLocalTimerCancellation;
use super::trusted_host_operational_entry::{
    enter_trusted_host_operation, TrustedHostOperationalEntryInput,
    TrustedHostOperationalEntryLocator,
};
use super::trusted_host_redispatch_loop::{
    TrustedHostRedispatchLoopOutcome, TrustedHostRedispatchStopReason,
};
use super::trusted_host_time_window_scheduling::TrustedHostRepeatedSchedulingOutcome;

pub(crate) struct TrustedHostExplicitLocalOperationInput<'a> {
    pub(crate) operational_entry: TrustedHostOperationalEntryInput<'a>,
    pub(crate) cancellation: TrustedHostLocalTimerCancellation,
}

impl fmt::Debug for TrustedHostExplicitLocalOperationInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostExplicitLocalOperationInput")
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

pub(crate) enum TrustedHostExplicitLocalOperationOutcome {
    EntryStopped(TrustedHostRedispatchLoopOutcome),
    ContinuationStopped(TrustedHostRepeatedSchedulingOutcome),
}

impl fmt::Debug for TrustedHostExplicitLocalOperationOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EntryStopped(outcome) => formatter
                .debug_tuple("TrustedHostExplicitLocalOperationOutcome::EntryStopped")
                .field(outcome)
                .finish(),
            Self::ContinuationStopped(outcome) => formatter
                .debug_tuple("TrustedHostExplicitLocalOperationOutcome::ContinuationStopped")
                .field(outcome)
                .finish(),
        }
    }
}

pub(crate) fn run_explicit_trusted_host_local_operation(
    input: TrustedHostExplicitLocalOperationInput<'_>,
) -> Result<TrustedHostExplicitLocalOperationOutcome, WorkflowOsError> {
    let TrustedHostExplicitLocalOperationInput {
        operational_entry,
        cancellation,
    } = input;
    let TrustedHostOperationalEntryInput {
        backend,
        locator,
        opening,
        executor,
        skill_input,
        opening_persistence,
        identity_provider,
    } = operational_entry;
    let continuation_locator = clone_locator(&locator);
    let continuation_skill_input = skill_input.clone();

    let entry = enter_trusted_host_operation(TrustedHostOperationalEntryInput {
        backend,
        locator,
        opening,
        executor,
        skill_input,
        opening_persistence,
        identity_provider,
    })?;

    if entry.stop_reason != TrustedHostRedispatchStopReason::AwaitCondition {
        return Ok(TrustedHostExplicitLocalOperationOutcome::EntryStopped(
            entry,
        ));
    }

    let continuation =
        run_trusted_host_local_production_caller(TrustedHostLocalProductionCallerInput {
            backend,
            locator: continuation_locator,
            cancellation,
            executor,
            skill_input: continuation_skill_input,
        })?;
    Ok(TrustedHostExplicitLocalOperationOutcome::ContinuationStopped(continuation))
}

fn clone_locator(
    locator: &TrustedHostOperationalEntryLocator,
) -> TrustedHostOperationalEntryLocator {
    TrustedHostOperationalEntryLocator {
        workflow_id: locator.workflow_id.clone(),
        run_id: locator.run_id.clone(),
        step_id: locator.step_id.clone(),
        window_id: locator.window_id.clone(),
        subject_actor_id: locator.subject_actor_id.clone(),
        immutable_run_bundle: locator.immutable_run_bundle.clone(),
    }
}
