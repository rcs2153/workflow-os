use std::fmt;

use crate::authorized_execution_continuity_state::internal::{
    ContinuityOperationId, ContinuityReceiptId,
};
use crate::trusted_host_supervisor::TrustedHostAttemptExecutor;
use crate::{AuthorizedExecutionWaitConditionId, SkillInput, WorkflowOsError, WorkflowOsErrorKind};

use super::trusted_host_operational_entry::{
    enter_existing_trusted_host_operation, TrustedHostExistingOperationalEntryInput,
    TrustedHostOperationalEntryLocator,
};
use super::trusted_host_redispatch_loop::{
    TrustedHostRedispatchIdentityProvider, TrustedHostRedispatchLoopOutcome,
};
use super::trusted_host_time_window_caller::{
    apply_trusted_host_time_window_wake, TrustedHostTimeWindowWakeInput,
    TrustedHostTimeWindowWakeStatus,
};
use super::trusted_host_wait_handoff::TrustedHostWaitHandoff;
use super::SqliteStateBackend;

pub(crate) struct TrustedHostTimeWindowReinvocationInput<'a> {
    pub(crate) backend: &'a SqliteStateBackend,
    pub(crate) handoff: TrustedHostWaitHandoff,
    pub(crate) locator: TrustedHostOperationalEntryLocator,
    pub(crate) condition_id: AuthorizedExecutionWaitConditionId,
    pub(crate) condition_version: u32,
    pub(crate) operation_id: ContinuityOperationId,
    pub(crate) receipt_id: ContinuityReceiptId,
    pub(crate) executor: &'a dyn TrustedHostAttemptExecutor,
    pub(crate) skill_input: SkillInput,
    pub(crate) identity_provider: &'a mut dyn TrustedHostRedispatchIdentityProvider,
}

impl fmt::Debug for TrustedHostTimeWindowReinvocationInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostTimeWindowReinvocationInput")
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

pub(crate) struct TrustedHostTimeWindowReinvocationOutcome {
    pub(crate) wake_status: TrustedHostTimeWindowWakeStatus,
    pub(crate) execution: TrustedHostRedispatchLoopOutcome,
}

impl fmt::Debug for TrustedHostTimeWindowReinvocationOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostTimeWindowReinvocationOutcome")
            .field("wake_status", &self.wake_status)
            .field("execution", &self.execution)
            .finish()
    }
}

pub(crate) fn reinvoke_after_time_window_wait(
    input: TrustedHostTimeWindowReinvocationInput<'_>,
) -> Result<TrustedHostTimeWindowReinvocationOutcome, WorkflowOsError> {
    let wake_status = apply_trusted_host_time_window_wake(TrustedHostTimeWindowWakeInput {
        backend: input.backend,
        locator: TrustedHostOperationalEntryLocator {
            workflow_id: input.locator.workflow_id.clone(),
            run_id: input.locator.run_id.clone(),
            step_id: input.locator.step_id.clone(),
            window_id: input.locator.window_id.clone(),
            subject_actor_id: input.locator.subject_actor_id.clone(),
            immutable_run_bundle: input.locator.immutable_run_bundle.clone(),
        },
        condition_id: input.condition_id,
        condition_version: input.condition_version,
        operation_id: input.operation_id,
        receipt_id: input.receipt_id,
        handoff_commitment: Some(input.handoff.commitment()),
    })?
    .status;
    if wake_status == TrustedHostTimeWindowWakeStatus::SecurityRejected {
        return Err(reinvocation_error(
            WorkflowOsErrorKind::Security,
            "wake_rejected",
            "trusted-host reinvocation wake was rejected",
        ));
    }
    let execution =
        enter_existing_trusted_host_operation(TrustedHostExistingOperationalEntryInput {
            backend: input.backend,
            locator: input.locator,
            executor: input.executor,
            skill_input: input.skill_input,
            identity_provider: input.identity_provider,
        })?;
    Ok(TrustedHostTimeWindowReinvocationOutcome {
        wake_status,
        execution,
    })
}

fn reinvocation_error(
    kind: WorkflowOsErrorKind,
    suffix: &'static str,
    message: &'static str,
) -> WorkflowOsError {
    WorkflowOsError::new(
        kind,
        format!("trusted_host_time_window_reinvocation.{suffix}"),
        message,
    )
}
