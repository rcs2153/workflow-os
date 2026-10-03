use std::fmt;

use sha2::{Digest, Sha256};

use crate::authorized_execution_continuity_state::internal::{
    AuthoritativeContinuationDisposition, AuthoritativeWaitDependencyBinding,
    AuthoritativeWaitIdentity, AuthoritativeWaitState, ContinuityRevision,
    ContinuityYieldGenerationId, ReferenceContinuityState, TrustedTimeObservation,
};
use crate::authorized_execution_continuity_state::semantics;
use crate::{
    AuthorizedExecutionWaitConditionId, AuthorizedExecutionWindowId, SpecContentHash,
    WorkflowOsError, WorkflowOsErrorKind,
};

use super::continuity_store::observe_continuity_trusted_time;
use super::trusted_host_operational_entry::TrustedHostOperationalEntryLocator;
use super::{continuity_codec, SqliteStateBackend};

const HANDOFF_ID_DOMAIN: &str = "workflow-os/trusted-host-wait-handoff/v1";
const CURSOR_COMMITMENT_DOMAIN: &str = "workflow-os/trusted-host-wait-handoff-cursor/v1";

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct TrustedHostWaitHandoffId(SpecContentHash);

impl fmt::Debug for TrustedHostWaitHandoffId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("TrustedHostWaitHandoffId([REDACTED])")
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum TrustedHostWaitHandoffDependencyKind {
    TimeWindow,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum TrustedHostWaitHandoffConditionState {
    Unsatisfied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TrustedHostWaitHandoffNextOperation {
    RequestFreshClassification,
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct TrustedHostWaitHandoffCondition {
    condition_id: AuthorizedExecutionWaitConditionId,
    condition_version: u32,
    dependency_kind: TrustedHostWaitHandoffDependencyKind,
    state: TrustedHostWaitHandoffConditionState,
}

impl fmt::Debug for TrustedHostWaitHandoffCondition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostWaitHandoffCondition")
            .field("dependency_kind", &self.dependency_kind)
            .field("state", &self.state)
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct TrustedHostWaitHandoff {
    handoff_id: TrustedHostWaitHandoffId,
    window_id: AuthorizedExecutionWindowId,
    window_revision: ContinuityRevision,
    cursor_commitment: SpecContentHash,
    generation_id: ContinuityYieldGenerationId,
    conditions: Vec<TrustedHostWaitHandoffCondition>,
    disposition: AuthoritativeContinuationDisposition,
    next_operation: TrustedHostWaitHandoffNextOperation,
}

impl fmt::Debug for TrustedHostWaitHandoff {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostWaitHandoff")
            .field("disposition", &self.disposition)
            .field("next_operation", &self.next_operation)
            .field("condition_count", &self.conditions.len())
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
impl TrustedHostWaitHandoff {
    pub(super) fn handoff_id(&self) -> &TrustedHostWaitHandoffId {
        &self.handoff_id
    }

    pub(super) fn condition_count(&self) -> usize {
        self.conditions.len()
    }

    pub(super) const fn next_operation(&self) -> TrustedHostWaitHandoffNextOperation {
        self.next_operation
    }
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct TrustedHostWaitObservation {
    pub(crate) disposition: AuthoritativeContinuationDisposition,
    pub(crate) handoff: Option<TrustedHostWaitHandoff>,
}

impl fmt::Debug for TrustedHostWaitObservation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostWaitObservation")
            .field("disposition", &self.disposition)
            .field("handoff_present", &self.handoff.is_some())
            .finish()
    }
}

pub(crate) fn observe_trusted_host_wait(
    backend: &SqliteStateBackend,
    locator: &TrustedHostOperationalEntryLocator,
) -> Result<TrustedHostWaitObservation, WorkflowOsError> {
    observe_with(backend, locator, observe_continuity_trusted_time)
}

fn observe_with(
    backend: &SqliteStateBackend,
    locator: &TrustedHostOperationalEntryLocator,
    observe_time: impl FnOnce() -> Result<TrustedTimeObservation, WorkflowOsError>,
) -> Result<TrustedHostWaitObservation, WorkflowOsError> {
    observe_with_hook(backend, locator, observe_time, || Ok(()))
}

fn observe_with_hook(
    backend: &SqliteStateBackend,
    locator: &TrustedHostOperationalEntryLocator,
    observe_time: impl FnOnce() -> Result<TrustedTimeObservation, WorkflowOsError>,
    after_snapshot: impl FnOnce() -> Result<(), WorkflowOsError>,
) -> Result<TrustedHostWaitObservation, WorkflowOsError> {
    let mut connection = backend.existing_read_only_connection()?;
    let transaction = connection
        .transaction_with_behavior(rusqlite::TransactionBehavior::Deferred)
        .map_err(|_| handoff_corrupt())?;
    let state = continuity_codec::load_snapshot(&transaction)?;
    after_snapshot()?;
    let observation = observe_time()?;
    let outcome = derive_observation(&state, locator, &observation)?;
    transaction.commit().map_err(|_| handoff_corrupt())?;
    Ok(outcome)
}

#[cfg(test)]
pub(super) fn observe_trusted_host_wait_with_time(
    backend: &SqliteStateBackend,
    locator: &TrustedHostOperationalEntryLocator,
    observation: TrustedTimeObservation,
) -> Result<TrustedHostWaitObservation, WorkflowOsError> {
    observe_with(backend, locator, || Ok(observation))
}

#[cfg(test)]
pub(super) fn observe_trusted_host_wait_with_time_and_hook(
    backend: &SqliteStateBackend,
    locator: &TrustedHostOperationalEntryLocator,
    observation: TrustedTimeObservation,
    after_snapshot: impl FnOnce() -> Result<(), WorkflowOsError>,
) -> Result<TrustedHostWaitObservation, WorkflowOsError> {
    observe_with_hook(backend, locator, || Ok(observation), after_snapshot)
}

#[cfg(test)]
pub(super) fn derive_trusted_host_wait_observation_for_test(
    state: &ReferenceContinuityState,
    locator: &TrustedHostOperationalEntryLocator,
    observation: &TrustedTimeObservation,
) -> Result<TrustedHostWaitObservation, WorkflowOsError> {
    derive_observation(state, locator, observation)
}

fn derive_observation(
    state: &ReferenceContinuityState,
    locator: &TrustedHostOperationalEntryLocator,
    observation: &TrustedTimeObservation,
) -> Result<TrustedHostWaitObservation, WorkflowOsError> {
    let window = state
        .windows
        .get(&locator.window_id)
        .ok_or_else(handoff_corrupt)?;
    validate_locator(locator, window)?;
    let active_yield = window
        .active_yield
        .as_ref()
        .map(|id| state.yields.get(id).ok_or_else(handoff_corrupt))
        .transpose()?;
    let active_wait_ids = active_yield.map(|record| record.wait_ids.as_slice());
    let observed_at = (observation.source() == state.trusted_time.source
        && observation.provenance_commitment() == &state.trusted_time.provenance_commitment
        && observation.epoch_id() == &state.trusted_time.epoch_id)
        .then(|| observation.observed_at());
    let disposition = semantics::continuation_disposition(
        &state.trusted_time,
        window,
        &state.waits,
        active_wait_ids,
        observed_at,
    )?;
    let handoff = if disposition == AuthoritativeContinuationDisposition::AwaitCondition {
        Some(derive_handoff(
            state,
            window,
            active_yield.ok_or_else(handoff_corrupt)?,
        )?)
    } else {
        None
    };
    Ok(TrustedHostWaitObservation {
        disposition,
        handoff,
    })
}

fn derive_handoff(
    state: &ReferenceContinuityState,
    window: &crate::authorized_execution_continuity_state::internal::AuthoritativeWindowRecord,
    active_yield: &crate::authorized_execution_continuity_state::internal::AuthoritativeYieldRecord,
) -> Result<TrustedHostWaitHandoff, WorkflowOsError> {
    if active_yield.generation_id != window.active_yield.clone().ok_or_else(handoff_corrupt)?
        || active_yield.cursor != window.cursor
        || active_yield.wait_ids.is_empty()
    {
        return Err(handoff_corrupt());
    }
    let mut conditions = Vec::with_capacity(active_yield.wait_ids.len());
    for identity in &active_yield.wait_ids {
        conditions.push(condition_descriptor(state, window, identity)?);
    }
    conditions.sort_by(|left, right| {
        left.condition_id
            .as_str()
            .cmp(right.condition_id.as_str())
            .then(left.condition_version.cmp(&right.condition_version))
            .then(left.dependency_kind.cmp(&right.dependency_kind))
            .then(left.state.cmp(&right.state))
    });
    let cursor_commitment = cursor_commitment(&window.cursor);
    let handoff_id = handoff_id(window, &cursor_commitment, active_yield, &conditions);
    Ok(TrustedHostWaitHandoff {
        handoff_id,
        window_id: window.window_id.clone(),
        window_revision: window.revision,
        cursor_commitment,
        generation_id: active_yield.generation_id.clone(),
        conditions,
        disposition: AuthoritativeContinuationDisposition::AwaitCondition,
        next_operation: TrustedHostWaitHandoffNextOperation::RequestFreshClassification,
    })
}

fn condition_descriptor(
    state: &ReferenceContinuityState,
    window: &crate::authorized_execution_continuity_state::internal::AuthoritativeWindowRecord,
    identity: &AuthoritativeWaitIdentity,
) -> Result<TrustedHostWaitHandoffCondition, WorkflowOsError> {
    let wait = state.waits.get(identity).ok_or_else(handoff_corrupt)?;
    if wait.window_id != window.window_id
        || wait.generation_id != window.active_yield.clone().ok_or_else(handoff_corrupt)?
        || wait.condition_id != identity.condition_id
        || wait.condition_version != identity.condition_version
        || wait.state != AuthoritativeWaitState::Unsatisfied
    {
        return Err(handoff_corrupt());
    }
    let dependency_kind = match wait.dependency_binding {
        Some(AuthoritativeWaitDependencyBinding::TimeWindow { .. }) => {
            TrustedHostWaitHandoffDependencyKind::TimeWindow
        }
        None => return Err(handoff_unsupported()),
    };
    Ok(TrustedHostWaitHandoffCondition {
        condition_id: identity.condition_id.clone(),
        condition_version: identity.condition_version,
        dependency_kind,
        state: TrustedHostWaitHandoffConditionState::Unsatisfied,
    })
}

fn cursor_commitment(
    cursor: &crate::authorized_execution_continuity_state::internal::ContinuityCursor,
) -> SpecContentHash {
    hash_fields(
        CURSOR_COMMITMENT_DOMAIN,
        &[
            cursor.sequence_number.get().to_string(),
            cursor.event_id.as_str().to_owned(),
        ],
    )
}

fn handoff_id(
    window: &crate::authorized_execution_continuity_state::internal::AuthoritativeWindowRecord,
    cursor_commitment: &SpecContentHash,
    active_yield: &crate::authorized_execution_continuity_state::internal::AuthoritativeYieldRecord,
    conditions: &[TrustedHostWaitHandoffCondition],
) -> TrustedHostWaitHandoffId {
    let mut fields = vec![
        window.window_id.as_str().to_owned(),
        window.revision.get().to_string(),
        cursor_commitment.as_str().to_owned(),
        active_yield.generation_id.as_str().to_owned(),
    ];
    for condition in conditions {
        fields.extend([
            condition.condition_id.as_str().to_owned(),
            condition.condition_version.to_string(),
            "time_window".to_owned(),
            "unsatisfied".to_owned(),
        ]);
    }
    TrustedHostWaitHandoffId(hash_fields(HANDOFF_ID_DOMAIN, &fields))
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
        return Err(handoff_error(
            WorkflowOsErrorKind::Security,
            "window_binding_mismatch",
            "trusted-host wait handoff binding is invalid",
        ));
    }
    Ok(())
}

fn handoff_corrupt() -> WorkflowOsError {
    handoff_error(
        WorkflowOsErrorKind::InvalidState,
        "state_corrupt",
        "trusted-host wait handoff state is invalid",
    )
}

fn handoff_unsupported() -> WorkflowOsError {
    handoff_error(
        WorkflowOsErrorKind::InvalidState,
        "dependency_unsupported",
        "trusted-host wait handoff dependency is unsupported",
    )
}

fn handoff_error(
    kind: WorkflowOsErrorKind,
    suffix: &'static str,
    message: &'static str,
) -> WorkflowOsError {
    WorkflowOsError::new(kind, format!("trusted_host_wait_handoff.{suffix}"), message)
}
