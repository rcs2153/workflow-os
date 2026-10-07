use std::fmt;

use sha2::{Digest, Sha256};

use crate::authorized_execution_continuity_state::internal::{
    AuthoritativeContinuationDisposition, AuthoritativeWaitDependencyBinding,
    AuthoritativeWaitState, AuthoritativeWindowState, ContinuityInstanceEligibility,
    ContinuityOperationId, ContinuityReceiptId, ContinuityRevision, ReferenceContinuityState,
    TrustedTimeObservation, TrustedTimePosture, TrustedTimeSourceKind,
};
use crate::trusted_host_supervisor::TrustedHostAttemptExecutor;
use crate::{
    AuthorizedExecutionWaitConditionId, SkillInput, SpecContentHash, Timestamp, WorkflowOsError,
    WorkflowOsErrorKind,
};

use super::continuity_store::observe_continuity_trusted_time;
use super::trusted_host_operational_entry::TrustedHostOperationalEntryLocator;
use super::trusted_host_redispatch_loop::TrustedHostRedispatchIdentityProvider;
use super::trusted_host_time_window_reinvocation::{
    reinvoke_after_time_window_wait, TrustedHostTimeWindowReinvocationInput,
    TrustedHostTimeWindowReinvocationOutcome,
};
use super::trusted_host_wait_handoff::{
    derive_handoff, derive_observation, validate_locator, TrustedHostWaitHandoff,
};
use super::{continuity_codec, SqliteStateBackend};

const TICKET_COMMITMENT_DOMAIN: &str = "workflow-os/trusted-host-time-window-schedule-ticket/v1";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TrustedHostTimeWindowScheduleNextOperation {
    WaitOnceThenRequestFreshVerification,
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct TrustedHostTimeWindowScheduleTicket {
    ticket_commitment: SpecContentHash,
    handoff_commitment: SpecContentHash,
    schedule_at: Timestamp,
    condition_id: AuthorizedExecutionWaitConditionId,
    condition_version: u32,
    wait_revision: ContinuityRevision,
    dependency_commitment: SpecContentHash,
    source_binding_commitment: SpecContentHash,
    next_operation: TrustedHostTimeWindowScheduleNextOperation,
}

impl fmt::Debug for TrustedHostTimeWindowScheduleTicket {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostTimeWindowScheduleTicket")
            .field("next_operation", &self.next_operation)
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

impl TrustedHostTimeWindowScheduleTicket {
    pub(super) const fn schedule_at(&self) -> Timestamp {
        self.schedule_at
    }

    #[cfg(test)]
    pub(super) fn commitment(&self) -> &SpecContentHash {
        &self.ticket_commitment
    }
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct TrustedHostTimeWindowSchedulingObservation {
    pub(crate) disposition: AuthoritativeContinuationDisposition,
    pub(crate) handoff: Option<TrustedHostWaitHandoff>,
    pub(crate) ticket: Option<TrustedHostTimeWindowScheduleTicket>,
}

impl fmt::Debug for TrustedHostTimeWindowSchedulingObservation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostTimeWindowSchedulingObservation")
            .field("disposition", &self.disposition)
            .field("handoff_present", &self.handoff.is_some())
            .field("ticket_present", &self.ticket.is_some())
            .finish()
    }
}

pub(crate) struct TrustedHostTimeWindowReadinessInput<'a> {
    pub(crate) backend: &'a SqliteStateBackend,
    pub(crate) locator: &'a TrustedHostOperationalEntryLocator,
    pub(crate) handoff: &'a TrustedHostWaitHandoff,
    pub(crate) ticket: &'a TrustedHostTimeWindowScheduleTicket,
}

impl fmt::Debug for TrustedHostTimeWindowReadinessInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostTimeWindowReadinessInput")
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TrustedHostTimeWindowReadinessStatus {
    Eligible,
    NotYetEligible,
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct TrustedHostTimeWindowReadinessAssessment {
    pub(crate) status: TrustedHostTimeWindowReadinessStatus,
    pub(crate) refreshed_scheduling: Option<TrustedHostTimeWindowSchedulingObservation>,
}

impl fmt::Debug for TrustedHostTimeWindowReadinessAssessment {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostTimeWindowReadinessAssessment")
            .field("status", &self.status)
            .field(
                "refreshed_scheduling_present",
                &self.refreshed_scheduling.is_some(),
            )
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TrustedHostDeadlineWaitOutcome {
    Woke,
    Canceled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TrustedHostDeadlineWaitFailure {
    Unavailable,
    Failed,
}

pub(crate) trait TrustedHostDeadlineWaiter {
    fn wait_until(
        &mut self,
        schedule_at: Timestamp,
    ) -> Result<TrustedHostDeadlineWaitOutcome, TrustedHostDeadlineWaitFailure>;
}

pub(crate) struct TrustedHostScheduleOnceInput<'a> {
    pub(crate) backend: &'a SqliteStateBackend,
    pub(crate) locator: TrustedHostOperationalEntryLocator,
    pub(crate) operation_id: ContinuityOperationId,
    pub(crate) receipt_id: ContinuityReceiptId,
    pub(crate) deadline_waiter: &'a mut dyn TrustedHostDeadlineWaiter,
    pub(crate) executor: &'a dyn TrustedHostAttemptExecutor,
    pub(crate) skill_input: SkillInput,
    pub(crate) identity_provider: &'a mut dyn TrustedHostRedispatchIdentityProvider,
}

impl fmt::Debug for TrustedHostScheduleOnceInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostScheduleOnceInput")
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

pub(crate) enum TrustedHostScheduleOnceOutcome {
    Canceled,
    NotYetEligible {
        refreshed_scheduling: Box<TrustedHostTimeWindowSchedulingObservation>,
    },
    Reinvoked(TrustedHostTimeWindowReinvocationOutcome),
}

impl fmt::Debug for TrustedHostScheduleOnceOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Canceled => formatter.write_str("TrustedHostScheduleOnceOutcome::Canceled"),
            Self::NotYetEligible { .. } => formatter
                .debug_struct("TrustedHostScheduleOnceOutcome::NotYetEligible")
                .field("refreshed_scheduling", &"[REDACTED]")
                .finish(),
            Self::Reinvoked(outcome) => formatter
                .debug_tuple("TrustedHostScheduleOnceOutcome::Reinvoked")
                .field(outcome)
                .finish(),
        }
    }
}

pub(crate) fn observe_trusted_host_time_window_scheduling(
    backend: &SqliteStateBackend,
    locator: &TrustedHostOperationalEntryLocator,
) -> Result<TrustedHostTimeWindowSchedulingObservation, WorkflowOsError> {
    observe_scheduling_with(backend, locator, observe_continuity_trusted_time)
}

pub(crate) fn assess_trusted_host_time_window_readiness(
    input: &TrustedHostTimeWindowReadinessInput<'_>,
) -> Result<TrustedHostTimeWindowReadinessAssessment, WorkflowOsError> {
    assess_readiness_with(input, observe_continuity_trusted_time)
}

pub(crate) fn schedule_trusted_host_time_window_once(
    input: TrustedHostScheduleOnceInput<'_>,
) -> Result<TrustedHostScheduleOnceOutcome, WorkflowOsError> {
    let scheduling = observe_trusted_host_time_window_scheduling(input.backend, &input.locator)?;
    let handoff = scheduling.handoff.ok_or_else(scheduling_ineligible)?;
    let ticket = scheduling.ticket.ok_or_else(scheduling_ineligible)?;

    match input.deadline_waiter.wait_until(ticket.schedule_at()) {
        Ok(TrustedHostDeadlineWaitOutcome::Canceled) => {
            return Ok(TrustedHostScheduleOnceOutcome::Canceled);
        }
        Err(failure) => return Err(deadline_wait_error(failure)),
        Ok(TrustedHostDeadlineWaitOutcome::Woke) => {}
    }

    let readiness =
        assess_trusted_host_time_window_readiness(&TrustedHostTimeWindowReadinessInput {
            backend: input.backend,
            locator: &input.locator,
            handoff: &handoff,
            ticket: &ticket,
        })?;
    match readiness.status {
        TrustedHostTimeWindowReadinessStatus::NotYetEligible => {
            let refreshed_scheduling = readiness
                .refreshed_scheduling
                .ok_or_else(scheduling_corrupt)?;
            Ok(TrustedHostScheduleOnceOutcome::NotYetEligible {
                refreshed_scheduling: Box::new(refreshed_scheduling),
            })
        }
        TrustedHostTimeWindowReadinessStatus::Eligible => {
            if readiness.refreshed_scheduling.is_some() {
                return Err(scheduling_corrupt());
            }
            let condition_id = ticket.condition_id.clone();
            let condition_version = ticket.condition_version;
            let outcome =
                reinvoke_after_time_window_wait(TrustedHostTimeWindowReinvocationInput {
                    backend: input.backend,
                    handoff,
                    locator: input.locator,
                    condition_id,
                    condition_version,
                    operation_id: input.operation_id,
                    receipt_id: input.receipt_id,
                    executor: input.executor,
                    skill_input: input.skill_input,
                    identity_provider: input.identity_provider,
                })?;
            Ok(TrustedHostScheduleOnceOutcome::Reinvoked(outcome))
        }
    }
}

fn observe_scheduling_with(
    backend: &SqliteStateBackend,
    locator: &TrustedHostOperationalEntryLocator,
    observe_time: impl FnOnce() -> Result<TrustedTimeObservation, WorkflowOsError>,
) -> Result<TrustedHostTimeWindowSchedulingObservation, WorkflowOsError> {
    let mut connection = backend.existing_read_only_connection()?;
    let transaction = connection
        .transaction_with_behavior(rusqlite::TransactionBehavior::Deferred)
        .map_err(|_| scheduling_corrupt())?;
    let state = continuity_codec::load_snapshot(&transaction)?;
    let observation = observe_time()?;
    let outcome = derive_scheduling_observation(&state, locator, &observation)?;
    transaction.commit().map_err(|_| scheduling_corrupt())?;
    Ok(outcome)
}

fn assess_readiness_with(
    input: &TrustedHostTimeWindowReadinessInput<'_>,
    observe_time: impl FnOnce() -> Result<TrustedTimeObservation, WorkflowOsError>,
) -> Result<TrustedHostTimeWindowReadinessAssessment, WorkflowOsError> {
    let mut connection = input.backend.existing_read_only_connection()?;
    let transaction = connection
        .transaction_with_behavior(rusqlite::TransactionBehavior::Deferred)
        .map_err(|_| scheduling_corrupt())?;
    let state = continuity_codec::load_snapshot(&transaction)?;
    let observation = observe_time()?;
    validate_time_observation(&state, &observation)?;

    let window = state
        .windows
        .get(&input.locator.window_id)
        .ok_or_else(scheduling_corrupt)?;
    validate_locator(input.locator, window)?;
    if window.state != AuthoritativeWindowState::Yielded {
        return Err(scheduling_ineligible());
    }
    let active_yield = state
        .yields
        .get(
            window
                .active_yield
                .as_ref()
                .ok_or_else(scheduling_corrupt)?,
        )
        .ok_or_else(scheduling_corrupt)?;
    let current_handoff = derive_handoff(&state, window, active_yield)?;
    let current_ticket = derive_ticket(&state, window, active_yield, &current_handoff)?;
    if current_handoff.commitment() != input.handoff.commitment()
        || current_ticket.ticket_commitment != input.ticket.ticket_commitment
    {
        return Err(scheduling_stale());
    }

    let current = derive_observation(&state, input.locator, &observation)?;
    let assessment = if observation.observed_at() < current_ticket.schedule_at {
        if current.disposition != AuthoritativeContinuationDisposition::AwaitCondition {
            return Err(scheduling_ineligible());
        }
        TrustedHostTimeWindowReadinessAssessment {
            status: TrustedHostTimeWindowReadinessStatus::NotYetEligible,
            refreshed_scheduling: Some(TrustedHostTimeWindowSchedulingObservation {
                disposition: current.disposition,
                handoff: Some(current_handoff),
                ticket: Some(current_ticket),
            }),
        }
    } else {
        if current.disposition != AuthoritativeContinuationDisposition::AwaitCondition {
            return Err(scheduling_ineligible());
        }
        TrustedHostTimeWindowReadinessAssessment {
            status: TrustedHostTimeWindowReadinessStatus::Eligible,
            refreshed_scheduling: None,
        }
    };
    transaction.commit().map_err(|_| scheduling_corrupt())?;
    Ok(assessment)
}

fn derive_scheduling_observation(
    state: &ReferenceContinuityState,
    locator: &TrustedHostOperationalEntryLocator,
    observation: &TrustedTimeObservation,
) -> Result<TrustedHostTimeWindowSchedulingObservation, WorkflowOsError> {
    validate_time_observation(state, observation)?;
    let observed = derive_observation(state, locator, observation)?;
    if observed.disposition != AuthoritativeContinuationDisposition::AwaitCondition {
        return Ok(TrustedHostTimeWindowSchedulingObservation {
            disposition: observed.disposition,
            handoff: None,
            ticket: None,
        });
    }
    let handoff = observed.handoff.ok_or_else(scheduling_corrupt)?;
    let window = state
        .windows
        .get(&locator.window_id)
        .ok_or_else(scheduling_corrupt)?;
    let active_yield = state
        .yields
        .get(
            window
                .active_yield
                .as_ref()
                .ok_or_else(scheduling_corrupt)?,
        )
        .ok_or_else(scheduling_corrupt)?;
    let ticket = derive_ticket(state, window, active_yield, &handoff)?;
    Ok(TrustedHostTimeWindowSchedulingObservation {
        disposition: observed.disposition,
        handoff: Some(handoff),
        ticket: Some(ticket),
    })
}

fn derive_ticket(
    state: &ReferenceContinuityState,
    window: &crate::authorized_execution_continuity_state::internal::AuthoritativeWindowRecord,
    active_yield: &crate::authorized_execution_continuity_state::internal::AuthoritativeYieldRecord,
    handoff: &TrustedHostWaitHandoff,
) -> Result<TrustedHostTimeWindowScheduleTicket, WorkflowOsError> {
    let [identity] = active_yield.wait_ids.as_slice() else {
        return Err(scheduling_unsupported());
    };
    let wait = state.waits.get(identity).ok_or_else(scheduling_corrupt)?;
    if wait.window_id != window.window_id
        || wait.generation_id != active_yield.generation_id
        || wait.condition_id != identity.condition_id
        || wait.condition_version != identity.condition_version
        || wait.state != AuthoritativeWaitState::Unsatisfied
    {
        return Err(scheduling_corrupt());
    }
    let Some(AuthoritativeWaitDependencyBinding::TimeWindow {
        dependency_commitment,
        deadline,
        source,
        provenance_commitment,
        epoch_id,
    }) = wait.dependency_binding.as_ref()
    else {
        return Err(scheduling_unsupported());
    };
    if *source != state.trusted_time.source
        || provenance_commitment != &state.trusted_time.provenance_commitment
        || epoch_id != &state.trusted_time.epoch_id
    {
        return Err(scheduling_source_mismatch());
    }
    let handoff_commitment = handoff.commitment();
    let source_binding_commitment = hash_fields(
        "workflow-os/trusted-host-time-window-source-binding/v1",
        &[
            source_label(*source).to_owned(),
            provenance_commitment.as_str().to_owned(),
            epoch_id.as_str().to_owned(),
        ],
    );
    let ticket_commitment = hash_fields(
        TICKET_COMMITMENT_DOMAIN,
        &[
            handoff_commitment.as_str().to_owned(),
            window.window_id.as_str().to_owned(),
            window.revision.get().to_string(),
            active_yield.generation_id.as_str().to_owned(),
            identity.condition_id.as_str().to_owned(),
            identity.condition_version.to_string(),
            wait.revision.get().to_string(),
            dependency_commitment.as_str().to_owned(),
            deadline.to_rfc3339(),
            source_binding_commitment.as_str().to_owned(),
            "wait_once_then_request_fresh_verification".to_owned(),
        ],
    );
    Ok(TrustedHostTimeWindowScheduleTicket {
        ticket_commitment,
        handoff_commitment,
        schedule_at: *deadline,
        condition_id: identity.condition_id.clone(),
        condition_version: identity.condition_version,
        wait_revision: wait.revision,
        dependency_commitment: dependency_commitment.clone(),
        source_binding_commitment,
        next_operation:
            TrustedHostTimeWindowScheduleNextOperation::WaitOnceThenRequestFreshVerification,
    })
}

fn validate_time_observation(
    state: &ReferenceContinuityState,
    observation: &TrustedTimeObservation,
) -> Result<(), WorkflowOsError> {
    if state.trusted_time.eligibility != ContinuityInstanceEligibility::LiveStateEligible
        || state.trusted_time.posture == TrustedTimePosture::Quarantined
    {
        return Err(scheduling_ineligible());
    }
    if observation.source() != state.trusted_time.source
        || observation.provenance_commitment() != &state.trusted_time.provenance_commitment
        || observation.epoch_id() != &state.trusted_time.epoch_id
    {
        return Err(scheduling_source_mismatch());
    }
    Ok(())
}

fn source_label(source: TrustedTimeSourceKind) -> &'static str {
    match source {
        TrustedTimeSourceKind::CoreInjectedClockV1 => "core_injected_clock_v1",
    }
}

fn hash_fields(domain: &str, fields: &[String]) -> SpecContentHash {
    let mut hasher = Sha256::new();
    hash_frame(&mut hasher, domain);
    for field in fields {
        hash_frame(&mut hasher, field);
    }
    SpecContentHash::from_bytes(hasher.finalize())
}

fn hash_frame(hasher: &mut Sha256, value: &str) {
    hasher.update(value.len().to_be_bytes());
    hasher.update(value.as_bytes());
}

#[cfg(test)]
pub(super) fn observe_trusted_host_time_window_scheduling_with_time(
    backend: &SqliteStateBackend,
    locator: &TrustedHostOperationalEntryLocator,
    observation: TrustedTimeObservation,
) -> Result<TrustedHostTimeWindowSchedulingObservation, WorkflowOsError> {
    observe_scheduling_with(backend, locator, || Ok(observation))
}

#[cfg(test)]
pub(super) fn assess_trusted_host_time_window_readiness_with_time(
    input: &TrustedHostTimeWindowReadinessInput<'_>,
    observation: TrustedTimeObservation,
) -> Result<TrustedHostTimeWindowReadinessAssessment, WorkflowOsError> {
    assess_readiness_with(input, || Ok(observation))
}

fn scheduling_corrupt() -> WorkflowOsError {
    scheduling_error(
        WorkflowOsErrorKind::InvalidState,
        "state_corrupt",
        "trusted-host TimeWindow scheduling state is invalid",
    )
}

fn scheduling_unsupported() -> WorkflowOsError {
    scheduling_error(
        WorkflowOsErrorKind::InvalidState,
        "posture_unsupported",
        "trusted-host TimeWindow scheduling posture is unsupported",
    )
}

fn scheduling_ineligible() -> WorkflowOsError {
    scheduling_error(
        WorkflowOsErrorKind::InvalidState,
        "posture_ineligible",
        "trusted-host TimeWindow scheduling posture is not eligible",
    )
}

fn scheduling_stale() -> WorkflowOsError {
    scheduling_error(
        WorkflowOsErrorKind::Security,
        "ticket_stale",
        "trusted-host TimeWindow scheduling ticket is not current",
    )
}

fn scheduling_source_mismatch() -> WorkflowOsError {
    scheduling_error(
        WorkflowOsErrorKind::Security,
        "trusted_time_binding_mismatch",
        "trusted-host TimeWindow scheduling trusted-time binding is invalid",
    )
}

fn deadline_wait_error(failure: TrustedHostDeadlineWaitFailure) -> WorkflowOsError {
    match failure {
        TrustedHostDeadlineWaitFailure::Unavailable => scheduling_error(
            WorkflowOsErrorKind::Unsupported,
            "deadline_wait_unavailable",
            "trusted-host deadline wait is unavailable",
        ),
        TrustedHostDeadlineWaitFailure::Failed => scheduling_error(
            WorkflowOsErrorKind::Internal,
            "deadline_wait_failed",
            "trusted-host deadline wait failed",
        ),
    }
}

fn scheduling_error(
    kind: WorkflowOsErrorKind,
    suffix: &'static str,
    message: &'static str,
) -> WorkflowOsError {
    WorkflowOsError::new(
        kind,
        format!("trusted_host_time_window_scheduling.{suffix}"),
        message,
    )
}
