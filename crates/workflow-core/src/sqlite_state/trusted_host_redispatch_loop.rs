use std::fmt;

use crate::authorized_execution_continuity_state::internal::{
    expected_consume_directive_commitment, window_binding_commitment,
    AuthoritativeContinuationDisposition, AuthoritativeDirectiveState, AuthoritativeYieldRecord,
    AuthorityUseCapability, AuthorizedExecutionContinuityProjectionStore,
    AuthorizedExecutionContinuityStore, ConsumeDirectiveRequest, ConsumeDirectiveResult,
    ContinuityDirectiveId, ContinuityOperationId, ContinuityReceiptId, ContinuityYieldGenerationId,
    ExpectedWaitRevision, ReferenceContinuityState,
};
use crate::trusted_host_supervisor::{
    supervise_one_local_skill_attempt, trusted_host_supervisor_binding, TrustedHostAttemptExecutor,
    TrustedHostSupervisorAttemptCapability, TrustedHostSupervisorInput,
    TrustedHostSupervisorPersistenceInput,
};
use crate::{
    AuthorizedExecutionAttemptId, SkillInput, SkillOutput, SpecContentHash, StateBackend,
    WorkflowOsError, WorkflowOsErrorKind,
};

use super::{continuity_codec, SqliteStateBackend};

pub(crate) struct TrustedHostRedispatchIterationIdentity {
    pub(crate) consume_operation: ContinuityOperationId,
    pub(crate) consume_receipt: ContinuityReceiptId,
    pub(crate) generated_attempt: AuthorizedExecutionAttemptId,
    pub(crate) supervisor_operation: ContinuityOperationId,
    pub(crate) supervisor_receipt: ContinuityReceiptId,
    pub(crate) yield_generation: ContinuityYieldGenerationId,
}

impl fmt::Debug for TrustedHostRedispatchIterationIdentity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostRedispatchIterationIdentity")
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

pub(crate) trait TrustedHostRedispatchIdentityProvider {
    fn next_identity(
        &mut self,
        iteration: u32,
    ) -> Result<TrustedHostRedispatchIterationIdentity, WorkflowOsError>;
}

pub(crate) struct TrustedHostRedispatchLoopInput<'a> {
    pub(crate) backend: &'a SqliteStateBackend,
    pub(crate) initial_capability: TrustedHostSupervisorAttemptCapability,
    pub(crate) executor: &'a dyn TrustedHostAttemptExecutor,
    pub(crate) skill_input: SkillInput,
    pub(crate) initial_persistence: TrustedHostSupervisorPersistenceInput,
    pub(crate) identity_provider: &'a mut dyn TrustedHostRedispatchIdentityProvider,
}

impl fmt::Debug for TrustedHostRedispatchLoopInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostRedispatchLoopInput")
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TrustedHostRedispatchStopReason {
    AwaitCondition,
    Blocked,
    Terminal,
}

pub(crate) struct TrustedHostRedispatchLoopOutcome {
    pub(crate) disposition: AuthoritativeContinuationDisposition,
    pub(crate) executor_entries: u32,
    pub(crate) stop_reason: TrustedHostRedispatchStopReason,
    pub(crate) skill_output: Option<SkillOutput>,
}

impl fmt::Debug for TrustedHostRedispatchLoopOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostRedispatchLoopOutcome")
            .field("disposition", &self.disposition)
            .field("executor_entries", &self.executor_entries)
            .field("stop_reason", &self.stop_reason)
            .field(
                "skill_output",
                &self.skill_output.as_ref().map(|_| "[REDACTED]"),
            )
            .finish()
    }
}

pub(crate) fn run_bounded_trusted_host_redispatch_loop(
    input: TrustedHostRedispatchLoopInput<'_>,
) -> Result<TrustedHostRedispatchLoopOutcome, WorkflowOsError> {
    let binding = trusted_host_supervisor_binding(&input.initial_capability);
    let mut entries = 1_u32;
    let first = supervise_one_local_skill_attempt(TrustedHostSupervisorInput {
        backend: input.backend,
        capability: input.initial_capability,
        executor: input.executor,
        skill_input: input.skill_input.clone(),
        persistence: input.initial_persistence,
    })?;
    let mut disposition = first.disposition;
    let mut skill_output = first.skill_output;

    loop {
        if let Some(stop_reason) = stop_reason(disposition) {
            return Ok(TrustedHostRedispatchLoopOutcome {
                disposition,
                executor_entries: entries,
                stop_reason,
                skill_output,
            });
        }

        let fresh = input.backend.continuation_disposition(&binding.window_id)?;
        if let Some(stop_reason) = stop_reason(fresh) {
            return Ok(TrustedHostRedispatchLoopOutcome {
                disposition: fresh,
                executor_entries: entries,
                stop_reason,
                skill_output,
            });
        }

        let iteration = entries.checked_add(1).ok_or_else(|| {
            redispatch_error(
                WorkflowOsErrorKind::InvalidState,
                "iteration_count_exhausted",
                "trusted-host redispatch iteration count is invalid",
            )
        })?;
        ensure_resume_attempt_available(input.backend, &binding)?;
        let identity = input.identity_provider.next_identity(iteration)?;
        let resumed = consume_fresh_resume_directive(input.backend, &binding, identity)?;
        let result = supervise_one_local_skill_attempt(TrustedHostSupervisorInput {
            backend: input.backend,
            capability: resumed.capability,
            executor: input.executor,
            skill_input: input.skill_input.clone(),
            persistence: TrustedHostSupervisorPersistenceInput {
                operation: resumed.supervisor_operation,
                receipt: resumed.supervisor_receipt,
                yield_generation: Some(resumed.yield_generation),
            },
        })?;
        entries = iteration;
        disposition = result.disposition;
        if result.skill_output.is_some() {
            skill_output = result.skill_output;
        }
    }
}

fn ensure_resume_attempt_available(
    backend: &SqliteStateBackend,
    binding: &crate::trusted_host_supervisor::TrustedHostSupervisorBinding,
) -> Result<(), WorkflowOsError> {
    let connection = backend.connection()?;
    let state = continuity_codec::load_snapshot(&connection)?;
    let window = state.windows.get(&binding.window_id).ok_or_else(|| {
        redispatch_error(
            WorkflowOsErrorKind::InvalidState,
            "window_missing",
            "trusted-host redispatch state is unavailable",
        )
    })?;
    validate_window_binding(window, &binding.expected_window_binding)?;
    if window.next_attempt_number == 0
        || window.maximum_attempts == 0
        || window.next_attempt_number > window.maximum_attempts
    {
        return Err(redispatch_error(
            WorkflowOsErrorKind::InvalidState,
            "attempt_limit_inconsistent",
            "trusted-host redispatch liveness is inconsistent",
        ));
    }
    Ok(())
}

fn stop_reason(
    disposition: AuthoritativeContinuationDisposition,
) -> Option<TrustedHostRedispatchStopReason> {
    match disposition {
        AuthoritativeContinuationDisposition::ResumeNow => None,
        AuthoritativeContinuationDisposition::AwaitCondition => {
            Some(TrustedHostRedispatchStopReason::AwaitCondition)
        }
        AuthoritativeContinuationDisposition::Blocked => {
            Some(TrustedHostRedispatchStopReason::Blocked)
        }
        AuthoritativeContinuationDisposition::Terminal => {
            Some(TrustedHostRedispatchStopReason::Terminal)
        }
    }
}

struct ResumeCapabilityAndPersistence {
    capability: TrustedHostSupervisorAttemptCapability,
    supervisor_operation: ContinuityOperationId,
    supervisor_receipt: ContinuityReceiptId,
    yield_generation: ContinuityYieldGenerationId,
}

fn consume_fresh_resume_directive(
    backend: &SqliteStateBackend,
    binding: &crate::trusted_host_supervisor::TrustedHostSupervisorBinding,
    identity: TrustedHostRedispatchIterationIdentity,
) -> Result<ResumeCapabilityAndPersistence, WorkflowOsError> {
    let supervisor_operation = identity.supervisor_operation;
    let supervisor_receipt = identity.supervisor_receipt;
    let next_yield_generation = identity.yield_generation;
    let connection = backend.connection()?;
    let state = continuity_codec::load_snapshot(&connection)?;
    let window = state.windows.get(&binding.window_id).ok_or_else(|| {
        redispatch_error(
            WorkflowOsErrorKind::InvalidState,
            "window_missing",
            "trusted-host redispatch state is unavailable",
        )
    })?;
    validate_window_binding(window, &binding.expected_window_binding)?;
    if window.next_attempt_number == 0
        || window.maximum_attempts == 0
        || window.next_attempt_number > window.maximum_attempts
    {
        return Err(redispatch_error(
            WorkflowOsErrorKind::InvalidState,
            "attempt_limit_inconsistent",
            "trusted-host redispatch liveness is inconsistent",
        ));
    }
    let generation_id = window.active_yield.clone().ok_or_else(|| {
        redispatch_error(
            WorkflowOsErrorKind::InvalidState,
            "active_yield_missing",
            "trusted-host redispatch state is unavailable",
        )
    })?;
    let yielded = state.yields.get(&generation_id).ok_or_else(|| {
        redispatch_error(
            WorkflowOsErrorKind::InvalidState,
            "yield_missing",
            "trusted-host redispatch state is unavailable",
        )
    })?;
    let expected_waits = expected_wait_revisions(&state, yielded)?;
    let directive_id = available_directive_id(&state, binding, &generation_id)?;

    let mut expected_binding = binding.expected_window_binding.clone();
    expected_binding.cursor = window.cursor.clone();
    let mut request = ConsumeDirectiveRequest {
        operation_id: identity.consume_operation,
        request_commitment: SpecContentHash::from_text("pending redispatch directive"),
        receipt_id: identity.consume_receipt,
        directive_id,
        window_id: binding.window_id.clone(),
        expected_window_revision: window.revision,
        expected_window_binding: expected_binding.clone(),
        generation_id: generation_id.clone(),
        cursor: window.cursor.clone(),
        expected_waits: expected_waits.clone(),
        authority_capability: AuthorityUseCapability {
            window_id: binding.window_id.clone(),
            window_revision: window.revision,
            generation_id,
            cursor: window.cursor.clone(),
            subject_actor_id: window.subject_actor_id.clone(),
            authority_commitment: window.authority_commitment.clone(),
            window_binding_commitment: window_binding_commitment(&expected_binding),
            expected_waits,
        },
        generated_attempt_id: identity.generated_attempt,
    };
    request.request_commitment = expected_consume_directive_commitment(&request);
    drop(connection);
    let consumed = backend.consume_directive_projected(request)?;
    let capability = match consumed.result {
        ConsumeDirectiveResult::Consumed { capability, .. } => capability,
        ConsumeDirectiveResult::SecurityRejected(_) => {
            return Err(redispatch_error(
                WorkflowOsErrorKind::Security,
                "directive_rejected",
                "trusted-host redispatch authority was rejected",
            ));
        }
        ConsumeDirectiveResult::ExactReplay(_) => {
            return Err(redispatch_error(
                WorkflowOsErrorKind::InvalidState,
                "directive_replayed",
                "trusted-host redispatch authority is unavailable",
            ));
        }
    };
    let run = backend.rehydrate_run(&expected_binding.run_id)?;
    expected_binding.cursor =
        crate::authorized_execution_continuity_state::internal::ContinuityCursor {
            sequence_number: run.snapshot.last_sequence_number,
            event_id: run.snapshot.last_event_id,
        };
    Ok(ResumeCapabilityAndPersistence {
        capability: TrustedHostSupervisorAttemptCapability::Resumed {
            capability: Box::new(capability),
            expected_window_binding: Box::new(expected_binding),
            operation_binding_commitment: binding.operation_binding_commitment.clone(),
        },
        supervisor_operation,
        supervisor_receipt,
        yield_generation: next_yield_generation,
    })
}

fn expected_wait_revisions(
    state: &ReferenceContinuityState,
    yielded: &AuthoritativeYieldRecord,
) -> Result<Vec<ExpectedWaitRevision>, WorkflowOsError> {
    yielded
        .wait_ids
        .iter()
        .map(|wait_id| {
            let wait = state.waits.get(wait_id).ok_or_else(|| {
                redispatch_error(
                    WorkflowOsErrorKind::InvalidState,
                    "wait_missing",
                    "trusted-host redispatch state is unavailable",
                )
            })?;
            Ok(ExpectedWaitRevision {
                condition_id: wait.condition_id.clone(),
                condition_version: wait.condition_version,
                revision: wait.revision,
            })
        })
        .collect()
}

fn available_directive_id(
    state: &ReferenceContinuityState,
    binding: &crate::trusted_host_supervisor::TrustedHostSupervisorBinding,
    generation_id: &ContinuityYieldGenerationId,
) -> Result<ContinuityDirectiveId, WorkflowOsError> {
    let mut matching = state.directives.values().filter(|directive| {
        directive.window_id == binding.window_id
            && directive.generation_id == *generation_id
            && directive.state == AuthoritativeDirectiveState::Available
    });
    let directive_id = matching
        .next()
        .map(|directive| directive.directive_id.clone())
        .ok_or_else(|| {
            redispatch_error(
                WorkflowOsErrorKind::InvalidState,
                "directive_missing",
                "trusted-host redispatch authority is unavailable",
            )
        })?;
    if matching.next().is_some() {
        return Err(redispatch_error(
            WorkflowOsErrorKind::InvalidState,
            "directive_ambiguous",
            "trusted-host redispatch authority is unavailable",
        ));
    }
    Ok(directive_id)
}

fn validate_window_binding(
    window: &crate::authorized_execution_continuity_state::internal::AuthoritativeWindowRecord,
    expected: &crate::authorized_execution_continuity_state::internal::ExpectedWindowBinding,
) -> Result<(), WorkflowOsError> {
    if window.workflow_id != expected.workflow_id
        || window.run_id != expected.run_id
        || window.step_id != expected.step_id
        || window.subject_actor_id != expected.subject_actor_id
        || window.immutable_run_bundle != expected.immutable_run_bundle
        || window.governance_commitment != expected.governance_commitment
        || window.authority_commitment != expected.authority_commitment
    {
        return Err(redispatch_error(
            WorkflowOsErrorKind::Security,
            "window_binding_mismatch",
            "trusted-host redispatch binding is invalid",
        ));
    }
    Ok(())
}

fn redispatch_error(
    kind: WorkflowOsErrorKind,
    suffix: &'static str,
    message: &'static str,
) -> WorkflowOsError {
    WorkflowOsError::new(kind, format!("trusted_host_redispatch.{suffix}"), message)
}
