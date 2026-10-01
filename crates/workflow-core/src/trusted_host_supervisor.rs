use std::fmt;

#[cfg(test)]
use std::cell::Cell;

use crate::authorized_execution_continuity_state::internal::{
    expected_attempt_outcome_commitment, expected_recovery_commitment,
    expected_register_yield_commitment, AttemptUseCapability, AuthoritativeContinuationDisposition,
    AuthorizedExecutionAttemptDispatchValidator, AuthorizedExecutionContinuityProjectionStore,
    AuthorizedExecutionContinuityStore, CommittedOperationDisposition, ContinuityOperationId,
    ContinuityReceiptId, ContinuityRevision, ContinuityYieldGenerationId, ExpectedWindowBinding,
    MutationResult, ProjectedContinuityReconciliationResult, ReconcileOperationRequest,
    RecordAttemptOutcomeRequest, RecoverAmbiguousAttemptRequest, RegisterYieldRequest,
    RegisterYieldResult,
};
use crate::operational_execution_window_opening::{
    operation_binding_commitment, trusted_host_invocation_commitment,
    OperationalExecutionAttemptUseCapability,
};
use crate::{
    AuthorizedExecutionAttemptOutcome, AuthorizedExecutionYieldReason, SkillHandler, SkillInput,
    SkillOutput, SpecContentHash, SqliteStateBackend, WorkflowOsError, WorkflowOsErrorKind,
};

pub(crate) trait TrustedHostAttemptExecutor {
    fn binding_commitment(&self) -> SpecContentHash;

    fn execute(
        &self,
        context: &TrustedHostAttemptExecutionContext<'_>,
    ) -> TrustedHostAttemptExecutionResult;
}

pub(crate) struct LocalSkillAttemptExecutor<'a> {
    handler: &'a dyn SkillHandler,
    binding_commitment: SpecContentHash,
}

impl<'a> LocalSkillAttemptExecutor<'a> {
    pub(crate) fn new(handler: &'a dyn SkillHandler, binding_commitment: SpecContentHash) -> Self {
        Self {
            handler,
            binding_commitment,
        }
    }
}

impl TrustedHostAttemptExecutor for LocalSkillAttemptExecutor<'_> {
    fn binding_commitment(&self) -> SpecContentHash {
        self.binding_commitment.clone()
    }

    fn execute(
        &self,
        context: &TrustedHostAttemptExecutionContext<'_>,
    ) -> TrustedHostAttemptExecutionResult {
        match self.handler.invoke(context.skill_input.clone()) {
            Ok(output) => TrustedHostAttemptExecutionResult::Succeeded(output),
            Err(_) => TrustedHostAttemptExecutionResult::TerminalFailure,
        }
    }
}

pub(crate) struct TrustedHostAttemptExecutionContext<'a> {
    skill_input: &'a SkillInput,
}

impl fmt::Debug for TrustedHostAttemptExecutionContext<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostAttemptExecutionContext")
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

pub(crate) enum TrustedHostAttemptExecutionResult {
    Succeeded(SkillOutput),
    RetryableFailure,
    TerminalFailure,
    Yielded(AuthorizedExecutionYieldReason),
    AmbiguousMayHaveStarted,
}

#[cfg(test)]
#[derive(Clone, Copy)]
pub(crate) enum InjectedSupervisorPersistenceFault {
    Before,
    After,
}

#[cfg(test)]
thread_local! {
    static INJECTED_PERSISTENCE_FAULT: Cell<Option<InjectedSupervisorPersistenceFault>> = const { Cell::new(None) };
}

#[cfg(test)]
pub(crate) fn inject_supervisor_persistence_fault(fault: InjectedSupervisorPersistenceFault) {
    INJECTED_PERSISTENCE_FAULT.with(|slot| slot.set(Some(fault)));
}

#[cfg(test)]
fn take_supervisor_persistence_fault() -> Option<InjectedSupervisorPersistenceFault> {
    INJECTED_PERSISTENCE_FAULT.with(Cell::take)
}

pub(crate) struct TrustedHostSupervisorPersistenceInput {
    pub(crate) operation: ContinuityOperationId,
    pub(crate) receipt: ContinuityReceiptId,
    pub(crate) yield_generation: Option<ContinuityYieldGenerationId>,
}

pub(crate) enum TrustedHostSupervisorAttemptCapability {
    Opened(Box<OperationalExecutionAttemptUseCapability>),
    Resumed {
        capability: Box<AttemptUseCapability>,
        expected_window_binding: Box<ExpectedWindowBinding>,
        operation_binding_commitment: SpecContentHash,
    },
}

pub(crate) struct TrustedHostSupervisorInput<'a> {
    pub(crate) backend: &'a SqliteStateBackend,
    pub(crate) capability: TrustedHostSupervisorAttemptCapability,
    pub(crate) executor: &'a dyn TrustedHostAttemptExecutor,
    pub(crate) skill_input: SkillInput,
    pub(crate) persistence: TrustedHostSupervisorPersistenceInput,
}

pub(crate) struct TrustedHostSupervisorResult {
    pub(crate) disposition: AuthoritativeContinuationDisposition,
    pub(crate) skill_output: Option<SkillOutput>,
}

impl fmt::Debug for TrustedHostSupervisorResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostSupervisorResult")
            .field("disposition", &self.disposition)
            .field(
                "skill_output",
                &self.skill_output.as_ref().map(|_| "[REDACTED]"),
            )
            .finish()
    }
}

struct SupervisorCapability {
    capability: AttemptUseCapability,
    expected_window_binding: ExpectedWindowBinding,
    operation_binding_commitment: SpecContentHash,
}

pub(crate) fn supervise_one_local_skill_attempt(
    input: TrustedHostSupervisorInput<'_>,
) -> Result<TrustedHostSupervisorResult, WorkflowOsError> {
    let capability = normalize_capability(input.capability);
    validate_invocation_binding(
        &input.skill_input,
        &input.executor.binding_commitment(),
        &capability.expected_window_binding,
        &capability.operation_binding_commitment,
    )?;
    if !input
        .backend
        .attempt_dispatch_is_current(&capability.capability, &capability.expected_window_binding)?
    {
        return Err(supervisor_error(
            WorkflowOsErrorKind::InvalidState,
            "attempt_not_dispatchable",
            "trusted-host attempt authority is not dispatchable",
        ));
    }

    let execution = input.executor.execute(&TrustedHostAttemptExecutionContext {
        skill_input: &input.skill_input,
    });
    let mut skill_output = None;
    match execution {
        TrustedHostAttemptExecutionResult::Succeeded(output) => {
            persist_outcome(
                input.backend,
                &input.persistence,
                &capability,
                AuthorizedExecutionAttemptOutcome::Succeeded,
            )?;
            skill_output = Some(output);
        }
        TrustedHostAttemptExecutionResult::RetryableFailure => persist_outcome(
            input.backend,
            &input.persistence,
            &capability,
            AuthorizedExecutionAttemptOutcome::RetryableFailure,
        )?,
        TrustedHostAttemptExecutionResult::TerminalFailure => persist_outcome(
            input.backend,
            &input.persistence,
            &capability,
            AuthorizedExecutionAttemptOutcome::TerminalFailure,
        )?,
        TrustedHostAttemptExecutionResult::Yielded(reason) => {
            persist_yield(input.backend, &input.persistence, &capability, reason)?;
        }
        TrustedHostAttemptExecutionResult::AmbiguousMayHaveStarted => {
            persist_ambiguous(input.backend, &input.persistence, &capability)?;
        }
    }

    Ok(TrustedHostSupervisorResult {
        disposition: input
            .backend
            .continuation_disposition(&capability.capability.window_id)?,
        skill_output,
    })
}

fn normalize_capability(
    capability: TrustedHostSupervisorAttemptCapability,
) -> SupervisorCapability {
    match capability {
        TrustedHostSupervisorAttemptCapability::Opened(opened) => SupervisorCapability {
            capability: AttemptUseCapability {
                attempt_id: opened.attempt_id.clone(),
                subject_actor_id: opened.subject_actor_id.clone(),
                window_id: opened.window_id.clone(),
                window_revision: opened.window_revision,
                cursor: opened.cursor.clone(),
                authority_commitment: opened.authority_commitment.clone(),
                window_binding_commitment: opened.window_binding_commitment.clone(),
                consume_operation_id: opened.consume_operation_id.clone(),
            },
            expected_window_binding: opened.expected_window_binding,
            operation_binding_commitment: opened.operation_binding_commitment,
        },
        TrustedHostSupervisorAttemptCapability::Resumed {
            capability,
            expected_window_binding,
            operation_binding_commitment,
        } => SupervisorCapability {
            capability: AttemptUseCapability {
                attempt_id: capability.attempt_id,
                subject_actor_id: capability.subject_actor_id,
                window_id: capability.window_id,
                window_revision: capability.window_revision,
                cursor: capability.cursor,
                authority_commitment: capability.authority_commitment,
                window_binding_commitment: capability.window_binding_commitment,
                consume_operation_id: capability.consume_operation_id,
            },
            expected_window_binding: *expected_window_binding,
            operation_binding_commitment,
        },
    }
}

fn validate_invocation_binding(
    skill_input: &SkillInput,
    executor_binding: &SpecContentHash,
    expected: &ExpectedWindowBinding,
    expected_operation: &SpecContentHash,
) -> Result<(), WorkflowOsError> {
    if skill_input.workflow_id != expected.workflow_id
        || skill_input.run_id != expected.run_id
        || skill_input.step_id != expected.step_id
        || operation_binding_commitment(
            &skill_input.workflow_id,
            &skill_input.run_id,
            &skill_input.step_id,
            &trusted_host_invocation_commitment(skill_input, executor_binding),
        ) != *expected_operation
    {
        return Err(supervisor_error(
            WorkflowOsErrorKind::Security,
            "invocation_binding_mismatch",
            "trusted-host invocation binding does not match authorized work",
        ));
    }
    Ok(())
}

fn persist_yield(
    backend: &SqliteStateBackend,
    persistence: &TrustedHostSupervisorPersistenceInput,
    capability: &SupervisorCapability,
    reason: AuthorizedExecutionYieldReason,
) -> Result<(), WorkflowOsError> {
    let generation_id = persistence.yield_generation.clone().ok_or_else(|| {
        supervisor_error(
            WorkflowOsErrorKind::Validation,
            "yield_generation_missing",
            "trusted-host yield requires a generation identity",
        )
    })?;
    let mut request = RegisterYieldRequest {
        operation_id: persistence.operation.clone(),
        request_commitment: SpecContentHash::from_text("pending supervisor yield"),
        receipt_id: persistence.receipt.clone(),
        generation_id,
        window_id: capability.capability.window_id.clone(),
        expected_window_revision: capability.capability.window_revision,
        expected_window_binding: capability.expected_window_binding.clone(),
        cursor: capability.capability.cursor.clone(),
        attempt_id: capability.capability.attempt_id.clone(),
        attempt_capability: &capability.capability,
        reason,
        waits: Vec::new(),
    };
    request.request_commitment = expected_register_yield_commitment(&request);
    let reconciliation = ReconcileOperationRequest {
        operation_id: request.operation_id.clone(),
        expected_request_commitment: request.request_commitment.clone(),
        expected_receipt_id: request.receipt_id.clone(),
    };
    let result = match backend.register_yield_projected(request) {
        Ok(result) => result.result,
        Err(_) => return reconcile_persistence(backend, &reconciliation, "yield"),
    };
    match result {
        RegisterYieldResult::Registered(_) | RegisterYieldResult::ExactReplay(_) => Ok(()),
        RegisterYieldResult::SecurityRejected(_) => Err(supervisor_error(
            WorkflowOsErrorKind::Security,
            "yield_rejected",
            "trusted-host yield was rejected by current authority",
        )),
    }
}

fn persist_outcome(
    backend: &SqliteStateBackend,
    persistence: &TrustedHostSupervisorPersistenceInput,
    capability: &SupervisorCapability,
    outcome: AuthorizedExecutionAttemptOutcome,
) -> Result<(), WorkflowOsError> {
    let mut request = RecordAttemptOutcomeRequest {
        operation_id: persistence.operation.clone(),
        request_commitment: SpecContentHash::from_text("pending supervisor outcome"),
        receipt_id: persistence.receipt.clone(),
        window_id: capability.capability.window_id.clone(),
        expected_window_revision: capability.capability.window_revision,
        expected_window_binding: capability.expected_window_binding.clone(),
        attempt_id: capability.capability.attempt_id.clone(),
        expected_attempt_revision: ContinuityRevision::new(1)?,
        attempt_capability: &capability.capability,
        outcome,
    };
    request.request_commitment = expected_attempt_outcome_commitment(&request);
    let reconciliation = ReconcileOperationRequest {
        operation_id: request.operation_id.clone(),
        expected_request_commitment: request.request_commitment.clone(),
        expected_receipt_id: request.receipt_id.clone(),
    };
    #[cfg(test)]
    let injected_fault = take_supervisor_persistence_fault();
    #[cfg(test)]
    if matches!(
        injected_fault,
        Some(InjectedSupervisorPersistenceFault::Before)
    ) {
        return reconcile_persistence(backend, &reconciliation, "outcome");
    }
    let result = match backend.record_attempt_outcome_projected(request) {
        Ok(result) => result.result,
        Err(_) => return reconcile_persistence(backend, &reconciliation, "outcome"),
    };
    #[cfg(test)]
    if matches!(
        injected_fault,
        Some(InjectedSupervisorPersistenceFault::After)
    ) {
        return reconcile_persistence(backend, &reconciliation, "outcome");
    }
    match result {
        MutationResult::Recorded(_) | MutationResult::ExactReplay(_) => Ok(()),
        MutationResult::SecurityRejected(_) => Err(supervisor_error(
            WorkflowOsErrorKind::Security,
            "outcome_rejected",
            "trusted-host attempt outcome was rejected by current authority",
        )),
    }
}

fn persist_ambiguous(
    backend: &SqliteStateBackend,
    persistence: &TrustedHostSupervisorPersistenceInput,
    capability: &SupervisorCapability,
) -> Result<(), WorkflowOsError> {
    let mut request = RecoverAmbiguousAttemptRequest {
        operation_id: persistence.operation.clone(),
        request_commitment: SpecContentHash::from_text("pending supervisor recovery"),
        receipt_id: persistence.receipt.clone(),
        window_id: capability.capability.window_id.clone(),
        expected_window_revision: capability.capability.window_revision,
        expected_window_binding: capability.expected_window_binding.clone(),
        cursor: capability.capability.cursor.clone(),
        attempt_id: capability.capability.attempt_id.clone(),
        expected_attempt_revision: ContinuityRevision::new(1)?,
    };
    request.request_commitment = expected_recovery_commitment(&request);
    let reconciliation = ReconcileOperationRequest {
        operation_id: request.operation_id.clone(),
        expected_request_commitment: request.request_commitment.clone(),
        expected_receipt_id: request.receipt_id.clone(),
    };
    let result = match backend.recover_ambiguous_attempt_projected(request) {
        Ok(result) => result.result,
        Err(_) => return reconcile_persistence(backend, &reconciliation, "recovery"),
    };
    match result {
        MutationResult::Recorded(_) | MutationResult::ExactReplay(_) => Ok(()),
        MutationResult::SecurityRejected(_) => Err(supervisor_error(
            WorkflowOsErrorKind::Security,
            "recovery_rejected",
            "trusted-host ambiguous attempt recovery was rejected by current authority",
        )),
    }
}

fn reconcile_persistence(
    backend: &SqliteStateBackend,
    request: &ReconcileOperationRequest,
    operation: &str,
) -> Result<(), WorkflowOsError> {
    match backend.reconcile_projected_operation(request) {
        Ok(ProjectedContinuityReconciliationResult::DurablyCommitted(result)) => {
            match result.disposition {
                CommittedOperationDisposition::CommittedSuccess(_) => Ok(()),
                CommittedOperationDisposition::CommittedSecurityRejection(_) => {
                    Err(supervisor_error(
                        WorkflowOsErrorKind::Security,
                        &format!("{operation}_rejected"),
                        "trusted-host persistence was rejected by current authority",
                    ))
                }
            }
        }
        Ok(ProjectedContinuityReconciliationResult::ConfirmedAbsent) => Err(supervisor_error(
            WorkflowOsErrorKind::InvalidState,
            &format!("{operation}_not_committed"),
            "trusted-host persistence was confirmed absent after an ambiguous result",
        )),
        Err(_) => Err(supervisor_error(
            WorkflowOsErrorKind::InvalidState,
            &format!("{operation}_reconciliation_failed"),
            "trusted-host persistence could not be reconciled safely",
        )),
    }
}

fn supervisor_error(kind: WorkflowOsErrorKind, suffix: &str, message: &str) -> WorkflowOsError {
    WorkflowOsError::new(kind, format!("trusted_host_supervisor.{suffix}"), message)
}
