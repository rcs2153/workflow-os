use std::fmt;

use rusqlite::OptionalExtension;

use crate::authorized_execution_continuity_state::internal::{
    AuthoritativeContinuationDisposition, AuthorizedExecutionContinuityStore, ExpectedWindowBinding,
};
use crate::operational_execution_window_opening::{
    open_with_registered_current_authority, operation_binding_commitment, run_snapshot_commitment,
    trusted_host_invocation_commitment, OperationalExecutionWindowOpeningResult,
    OperationalExecutionWindowOpeningUseInput,
};
use crate::trusted_host_supervisor::{
    TrustedHostAttemptExecutor, TrustedHostSupervisorAttemptCapability,
    TrustedHostSupervisorBinding, TrustedHostSupervisorPersistenceInput,
};
use crate::{
    ActorId, AuthorizedExecutionWindowId, ImmutableRunBundleBinding, SkillInput, SpecContentHash,
    StateBackend, StepId, WorkflowId, WorkflowOsError, WorkflowOsErrorKind, WorkflowRunId,
};

use super::trusted_host_redispatch_loop::{
    consume_fresh_resume_directive, run_bounded_trusted_host_redispatch_loop,
    TrustedHostRedispatchIdentityProvider, TrustedHostRedispatchLoopInput,
    TrustedHostRedispatchLoopOutcome, TrustedHostRedispatchStopReason,
};
use super::{continuity_codec, SqliteStateBackend};

pub(crate) struct TrustedHostOperationalEntryLocator {
    pub(crate) workflow_id: WorkflowId,
    pub(crate) run_id: WorkflowRunId,
    pub(crate) step_id: StepId,
    pub(crate) window_id: AuthorizedExecutionWindowId,
    pub(crate) subject_actor_id: ActorId,
    pub(crate) immutable_run_bundle: ImmutableRunBundleBinding,
}

impl fmt::Debug for TrustedHostOperationalEntryLocator {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostOperationalEntryLocator")
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

pub(crate) struct TrustedHostOperationalEntryInput<'a> {
    pub(crate) backend: &'a SqliteStateBackend,
    pub(crate) locator: TrustedHostOperationalEntryLocator,
    pub(crate) posture: TrustedHostOperationalEntryPosture<'a>,
    pub(crate) executor: &'a dyn TrustedHostAttemptExecutor,
    pub(crate) skill_input: SkillInput,
    pub(crate) identity_provider: &'a mut dyn TrustedHostRedispatchIdentityProvider,
    pub(crate) expected_preparation_commitment: Option<SpecContentHash>,
}

pub(crate) enum TrustedHostOperationalEntryPosture<'a> {
    Fresh {
        opening: Box<OperationalExecutionWindowOpeningUseInput<'a>>,
        persistence: TrustedHostSupervisorPersistenceInput,
    },
    Existing,
}

pub(crate) struct TrustedHostOperationalEntryPreparationBinding {
    pub(crate) commitment: SpecContentHash,
}

pub(crate) struct TrustedHostExistingOperationalEntryInput<'a> {
    pub(crate) backend: &'a SqliteStateBackend,
    pub(crate) locator: TrustedHostOperationalEntryLocator,
    pub(crate) executor: &'a dyn TrustedHostAttemptExecutor,
    pub(crate) skill_input: SkillInput,
    pub(crate) identity_provider: &'a mut dyn TrustedHostRedispatchIdentityProvider,
}

impl fmt::Debug for TrustedHostExistingOperationalEntryInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostExistingOperationalEntryInput")
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

impl fmt::Debug for TrustedHostOperationalEntryInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostOperationalEntryInput")
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

pub(crate) fn enter_trusted_host_operation(
    input: TrustedHostOperationalEntryInput<'_>,
) -> Result<TrustedHostRedispatchLoopOutcome, WorkflowOsError> {
    let invocation_commitment = trusted_host_invocation_commitment(
        &input.skill_input,
        &input.executor.binding_commitment(),
    );
    validate_skill_identity(&input.locator, &input.skill_input)?;

    let current = current_preparation_context(
        input.backend,
        &input.locator,
        &input.posture,
        &invocation_commitment,
    )?;
    if input
        .expected_preparation_commitment
        .as_ref()
        .is_some_and(|expected| expected != &current.binding.commitment)
    {
        return Err(entry_error(
            WorkflowOsErrorKind::Security,
            "preparation_binding_changed",
            "trusted-host operational preparation binding changed",
        ));
    }

    let (initial_capability, initial_persistence) = match current.window.as_ref() {
        None => open_initial_attempt(&input, &invocation_commitment)?,
        Some(window) => {
            if window.window_id != input.locator.window_id {
                return Err(entry_error(
                    WorkflowOsErrorKind::Security,
                    "window_identity_mismatch",
                    "trusted-host operational entry binding is invalid",
                ));
            }
            let disposition = input
                .backend
                .continuation_disposition(&input.locator.window_id)?;
            if disposition != AuthoritativeContinuationDisposition::ResumeNow {
                return closed_outcome(disposition);
            }
            resume_existing_attempt(
                input.backend,
                &input.locator,
                window,
                &invocation_commitment,
                input.identity_provider,
            )?
        }
    };

    run_bounded_trusted_host_redispatch_loop(TrustedHostRedispatchLoopInput {
        backend: input.backend,
        initial_capability,
        executor: input.executor,
        skill_input: input.skill_input,
        initial_persistence,
        identity_provider: input.identity_provider,
    })
}

pub(crate) fn validate_trusted_host_operational_entry_preparation(
    backend: &SqliteStateBackend,
    locator: &TrustedHostOperationalEntryLocator,
    posture: &TrustedHostOperationalEntryPosture<'_>,
    executor: &dyn TrustedHostAttemptExecutor,
    skill_input: &SkillInput,
) -> Result<TrustedHostOperationalEntryPreparationBinding, WorkflowOsError> {
    let invocation_commitment =
        trusted_host_invocation_commitment(skill_input, &executor.binding_commitment());
    validate_skill_identity(locator, skill_input)?;
    current_preparation_context(backend, locator, posture, &invocation_commitment)
        .map(|current| current.binding)
}

struct TrustedHostOperationalEntryPreparationContext {
    binding: TrustedHostOperationalEntryPreparationBinding,
    window:
        Option<crate::authorized_execution_continuity_state::internal::AuthoritativeWindowRecord>,
}

fn current_preparation_context(
    backend: &SqliteStateBackend,
    locator: &TrustedHostOperationalEntryLocator,
    posture: &TrustedHostOperationalEntryPosture<'_>,
    invocation_commitment: &SpecContentHash,
) -> Result<TrustedHostOperationalEntryPreparationContext, WorkflowOsError> {
    let run = backend.rehydrate_run(&locator.run_id)?;
    if run.snapshot.identity.workflow_id != locator.workflow_id
        || !matches!(
            run.snapshot.status,
            crate::WorkflowRunStatus::Running | crate::WorkflowRunStatus::Retrying
        )
        || run.snapshot.identity.immutable_run_bundle.as_ref()
            != Some(&locator.immutable_run_bundle)
    {
        return Err(entry_error(
            WorkflowOsErrorKind::InvalidState,
            "run_not_eligible",
            "trusted-host operational run is not eligible",
        ));
    }
    let snapshot_commitment = run_snapshot_commitment(&run.snapshot)?;
    let state = continuity_codec::load_snapshot(&backend.connection()?)?;
    let matching = state
        .windows
        .values()
        .filter(|window| {
            window.workflow_id == locator.workflow_id
                && window.run_id == locator.run_id
                && window.step_id == locator.step_id
        })
        .cloned()
        .collect::<Vec<_>>();

    let (posture_label, window, disposition, operation_binding) =
        match (posture, matching.as_slice()) {
            (TrustedHostOperationalEntryPosture::Fresh { opening, .. }, []) => {
                validate_opening_binding(backend, locator, opening, invocation_commitment)?;
                ("fresh", None, None, None)
            }
            (TrustedHostOperationalEntryPosture::Existing, [window]) => {
                if window.window_id != locator.window_id {
                    return Err(entry_error(
                        WorkflowOsErrorKind::Security,
                        "window_identity_mismatch",
                        "trusted-host operational entry binding is invalid",
                    ));
                }
                let binding =
                    existing_window_binding(backend, locator, window, invocation_commitment)?;
                let disposition = backend.continuation_disposition(&window.window_id)?;
                (
                    "existing",
                    Some(window.clone()),
                    Some(disposition),
                    Some(binding.operation_binding_commitment),
                )
            }
            (TrustedHostOperationalEntryPosture::Fresh { .. }, [_]) => {
                return Err(entry_error(
                    WorkflowOsErrorKind::Security,
                    "preparation_posture_changed",
                    "trusted-host operational preparation posture changed",
                ));
            }
            (TrustedHostOperationalEntryPosture::Existing, []) => {
                return Err(entry_error(
                    WorkflowOsErrorKind::InvalidState,
                    "existing_window_unavailable",
                    "trusted-host existing operational window is unavailable",
                ));
            }
            (_, _) => {
                return Err(entry_error(
                    WorkflowOsErrorKind::InvalidState,
                    "window_ambiguous",
                    "trusted-host operational entry state is ambiguous",
                ));
            }
        };

    let payload = serde_json::to_vec(&(
        "workflow-os/trusted-host-operational-preparation/v1",
        posture_label,
        &snapshot_commitment,
        invocation_commitment,
        &locator.workflow_id,
        &locator.run_id,
        &locator.step_id,
        &locator.window_id,
        &locator.subject_actor_id,
        &locator.immutable_run_bundle,
        &window,
        disposition,
        operation_binding,
    ))
    .map_err(|_| entry_corrupt())?;
    Ok(TrustedHostOperationalEntryPreparationContext {
        binding: TrustedHostOperationalEntryPreparationBinding {
            commitment: SpecContentHash::from_bytes(payload),
        },
        window,
    })
}

pub(crate) fn enter_existing_trusted_host_operation(
    input: TrustedHostExistingOperationalEntryInput<'_>,
) -> Result<TrustedHostRedispatchLoopOutcome, WorkflowOsError> {
    let invocation_commitment = trusted_host_invocation_commitment(
        &input.skill_input,
        &input.executor.binding_commitment(),
    );
    validate_skill_identity(&input.locator, &input.skill_input)?;
    let state = continuity_codec::load_snapshot(&input.backend.connection()?)?;
    let matching = state
        .windows
        .values()
        .filter(|window| {
            window.workflow_id == input.locator.workflow_id
                && window.run_id == input.locator.run_id
                && window.step_id == input.locator.step_id
        })
        .collect::<Vec<_>>();
    let [window] = matching.as_slice() else {
        return Err(entry_error(
            WorkflowOsErrorKind::InvalidState,
            "existing_window_unavailable",
            "trusted-host existing operational window is unavailable",
        ));
    };
    if window.window_id != input.locator.window_id {
        return Err(entry_error(
            WorkflowOsErrorKind::Security,
            "window_identity_mismatch",
            "trusted-host operational entry binding is invalid",
        ));
    }
    let disposition = input
        .backend
        .continuation_disposition(&input.locator.window_id)?;
    if disposition != AuthoritativeContinuationDisposition::ResumeNow {
        return closed_outcome(disposition);
    }
    let (initial_capability, initial_persistence) = resume_existing_attempt(
        input.backend,
        &input.locator,
        window,
        &invocation_commitment,
        input.identity_provider,
    )?;
    run_bounded_trusted_host_redispatch_loop(TrustedHostRedispatchLoopInput {
        backend: input.backend,
        initial_capability,
        executor: input.executor,
        skill_input: input.skill_input,
        initial_persistence,
        identity_provider: input.identity_provider,
    })
}

fn resume_existing_attempt(
    backend: &SqliteStateBackend,
    locator: &TrustedHostOperationalEntryLocator,
    window: &crate::authorized_execution_continuity_state::internal::AuthoritativeWindowRecord,
    invocation_commitment: &SpecContentHash,
    identity_provider: &mut dyn TrustedHostRedispatchIdentityProvider,
) -> Result<
    (
        TrustedHostSupervisorAttemptCapability,
        TrustedHostSupervisorPersistenceInput,
    ),
    WorkflowOsError,
> {
    let binding = existing_window_binding(backend, locator, window, invocation_commitment)?;
    let identity = identity_provider.next_identity(1)?;
    let resumed = consume_fresh_resume_directive(backend, &binding, identity)?;
    Ok((
        resumed.capability,
        TrustedHostSupervisorPersistenceInput {
            operation: resumed.supervisor_operation,
            receipt: resumed.supervisor_receipt,
            yield_generation: Some(resumed.yield_generation),
        },
    ))
}

fn open_initial_attempt(
    input: &TrustedHostOperationalEntryInput<'_>,
    invocation_commitment: &SpecContentHash,
) -> Result<
    (
        TrustedHostSupervisorAttemptCapability,
        TrustedHostSupervisorPersistenceInput,
    ),
    WorkflowOsError,
> {
    let TrustedHostOperationalEntryPosture::Fresh {
        opening,
        persistence,
    } = &input.posture
    else {
        return Err(entry_error(
            WorkflowOsErrorKind::InvalidState,
            "opening_context_missing",
            "trusted-host operational opening context is unavailable",
        ));
    };
    validate_opening_binding(
        input.backend,
        &input.locator,
        opening,
        invocation_commitment,
    )?;

    let opened = open_with_registered_current_authority(opening)?;
    match opened.result {
        OperationalExecutionWindowOpeningResult::Opened { capability, .. } => Ok((
            TrustedHostSupervisorAttemptCapability::Opened(capability),
            TrustedHostSupervisorPersistenceInput {
                operation: persistence.operation.clone(),
                receipt: persistence.receipt.clone(),
                yield_generation: persistence.yield_generation.clone(),
            },
        )),
        OperationalExecutionWindowOpeningResult::ExactReplay(_) => Err(entry_error(
            WorkflowOsErrorKind::InvalidState,
            "opening_capability_unavailable",
            "trusted-host operational opening authority is unavailable",
        )),
    }
}

fn validate_opening_binding(
    backend: &SqliteStateBackend,
    locator: &TrustedHostOperationalEntryLocator,
    opening: &OperationalExecutionWindowOpeningUseInput<'_>,
    invocation_commitment: &SpecContentHash,
) -> Result<(), WorkflowOsError> {
    if !std::ptr::eq(opening.backend, backend)
        || opening.execution_binding.workflow_id() != &locator.workflow_id
        || opening.execution_binding.run_id() != &locator.run_id
        || opening.execution_binding.step_id() != &locator.step_id
        || opening.execution_binding.actor() != &locator.subject_actor_id
        || opening.execution_binding.immutable_run_bundle() != &locator.immutable_run_bundle
        || opening.window_id != locator.window_id
        || opening.invocation_binding_commitment != invocation_commitment
    {
        return Err(entry_error(
            WorkflowOsErrorKind::Security,
            "opening_binding_mismatch",
            "trusted-host operational opening binding is invalid",
        ));
    }
    Ok(())
}

fn existing_window_binding(
    backend: &SqliteStateBackend,
    locator: &TrustedHostOperationalEntryLocator,
    window: &crate::authorized_execution_continuity_state::internal::AuthoritativeWindowRecord,
    invocation_commitment: &SpecContentHash,
) -> Result<TrustedHostSupervisorBinding, WorkflowOsError> {
    if window.subject_actor_id != locator.subject_actor_id
        || window.immutable_run_bundle != locator.immutable_run_bundle
    {
        return Err(entry_error(
            WorkflowOsErrorKind::Security,
            "window_binding_mismatch",
            "trusted-host operational entry binding is invalid",
        ));
    }
    let expected_operation_binding = operation_binding_commitment(
        &locator.workflow_id,
        &locator.run_id,
        &locator.step_id,
        invocation_commitment,
    );
    let persisted_operation_binding = read_opening_operation_binding(backend, &locator.window_id)?;
    if persisted_operation_binding != expected_operation_binding {
        return Err(entry_error(
            WorkflowOsErrorKind::Security,
            "invocation_binding_mismatch",
            "trusted-host operational invocation binding is invalid",
        ));
    }
    Ok(TrustedHostSupervisorBinding {
        window_id: locator.window_id.clone(),
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
        operation_binding_commitment: persisted_operation_binding,
    })
}

fn read_opening_operation_binding(
    backend: &SqliteStateBackend,
    window_id: &AuthorizedExecutionWindowId,
) -> Result<SpecContentHash, WorkflowOsError> {
    let connection = backend.connection()?;
    let value = connection
        .query_row(
            "SELECT operation_binding_commitment FROM operational_opening_operations WHERE window_id=?1",
            [window_id.as_str()],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|_| entry_corrupt())?
        .ok_or_else(entry_corrupt)?;
    SpecContentHash::new(value).map_err(|_| entry_corrupt())
}

fn validate_skill_identity(
    locator: &TrustedHostOperationalEntryLocator,
    input: &SkillInput,
) -> Result<(), WorkflowOsError> {
    if input.workflow_id != locator.workflow_id
        || input.run_id != locator.run_id
        || input.step_id != locator.step_id
    {
        return Err(entry_error(
            WorkflowOsErrorKind::Security,
            "skill_identity_mismatch",
            "trusted-host operational invocation identity is invalid",
        ));
    }
    Ok(())
}

fn closed_outcome(
    disposition: AuthoritativeContinuationDisposition,
) -> Result<TrustedHostRedispatchLoopOutcome, WorkflowOsError> {
    let stop_reason = match disposition {
        AuthoritativeContinuationDisposition::AwaitCondition => {
            TrustedHostRedispatchStopReason::AwaitCondition
        }
        AuthoritativeContinuationDisposition::Blocked => TrustedHostRedispatchStopReason::Blocked,
        AuthoritativeContinuationDisposition::Terminal => TrustedHostRedispatchStopReason::Terminal,
        AuthoritativeContinuationDisposition::ResumeNow => {
            return Err(entry_error(
                WorkflowOsErrorKind::InvalidState,
                "resume_not_consumed",
                "trusted-host operational resume authority was not consumed",
            ));
        }
    };
    Ok(TrustedHostRedispatchLoopOutcome {
        disposition,
        executor_entries: 0,
        stop_reason,
        skill_output: None,
    })
}

fn entry_corrupt() -> WorkflowOsError {
    entry_error(
        WorkflowOsErrorKind::InvalidState,
        "state_corrupt",
        "trusted-host operational entry state is invalid",
    )
}

fn entry_error(
    kind: WorkflowOsErrorKind,
    suffix: &'static str,
    message: &'static str,
) -> WorkflowOsError {
    WorkflowOsError::new(
        kind,
        format!("trusted_host_operational_entry.{suffix}"),
        message,
    )
}
