use std::fmt;

#[cfg(test)]
use std::cell::Cell;

use crate::authorized_execution_continuity_state::internal::{
    expected_attempt_outcome_commitment, expected_recovery_commitment,
    expected_register_yield_commitment, AttemptUseCapability, AuthoritativeContinuationDisposition,
    AuthorizedExecutionContinuityProjectionStore, AuthorizedExecutionContinuityStore,
    CommittedOperationDisposition, ContinuityOperationId, ContinuityReceiptId,
    ContinuityYieldGenerationId, ExpectedWindowBinding, MutationResult,
    ProjectedContinuityReconciliationResult, ReconcileOperationRequest,
    RecordAttemptOutcomeRequest, RecoverAmbiguousAttemptRequest, RegisterYieldRequest,
    RegisterYieldResult, SeedWait,
};
use crate::dispatch_reservation::{
    request_commitment as dispatch_request_commitment, DispatchReservationOperationId,
    DispatchReservationOutcome, DispatchReservationReceiptId, DispatchReservationRequest,
    DispatchReservationStore,
};
use crate::operational_execution_window_opening::{
    operation_binding_commitment, trusted_host_invocation_commitment,
    OperationalExecutionAttemptUseCapability,
};
use crate::{
    AuthorizedExecutionAttemptOutcome, AuthorizedExecutionWaitConditionId,
    AuthorizedExecutionWakeTriggerKind, AuthorizedExecutionYieldReason, SkillHandler, SkillInput,
    SkillOutput, SpecContentHash, SqliteStateBackend, Timestamp, WorkflowOsError,
    WorkflowOsErrorKind,
};
#[cfg(feature = "trusted-host-application-spi")]
use crate::{LocalCheckResultStatus, ResolvedExplicitLocalCheckProfile};

pub(crate) trait TrustedHostAttemptExecutor {
    fn binding_commitment(&self) -> SpecContentHash;

    fn workflow_lifecycle_context(
        &self,
    ) -> Option<&crate::trusted_host_workflow_lifecycle::TrustedHostWorkflowLifecycleContext> {
        None
    }

    fn execute(
        &self,
        context: &TrustedHostAttemptExecutionContext<'_>,
    ) -> TrustedHostAttemptExecutionResult;
}

pub(crate) struct LocalSkillAttemptExecutor<'a> {
    handler: &'a dyn SkillHandler,
    binding_commitment: SpecContentHash,
    workflow_lifecycle:
        Option<crate::trusted_host_workflow_lifecycle::TrustedHostWorkflowLifecycleContext>,
}

impl<'a> LocalSkillAttemptExecutor<'a> {
    pub(crate) fn new(handler: &'a dyn SkillHandler, binding_commitment: SpecContentHash) -> Self {
        Self {
            handler,
            binding_commitment,
            workflow_lifecycle: None,
        }
    }

    pub(crate) fn with_workflow_lifecycle(
        mut self,
        workflow_lifecycle: crate::trusted_host_workflow_lifecycle::TrustedHostWorkflowLifecycleContext,
    ) -> Self {
        self.workflow_lifecycle = Some(workflow_lifecycle);
        self
    }
}

impl TrustedHostAttemptExecutor for LocalSkillAttemptExecutor<'_> {
    fn binding_commitment(&self) -> SpecContentHash {
        self.binding_commitment.clone()
    }

    fn workflow_lifecycle_context(
        &self,
    ) -> Option<&crate::trusted_host_workflow_lifecycle::TrustedHostWorkflowLifecycleContext> {
        self.workflow_lifecycle.as_ref()
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

#[cfg(feature = "trusted-host-application-spi")]
pub(crate) struct TrustedHostLocalCheckAttemptExecutor<'a> {
    profile: &'a ResolvedExplicitLocalCheckProfile,
    binding_commitment: SpecContentHash,
    workflow_lifecycle:
        Option<crate::trusted_host_workflow_lifecycle::TrustedHostWorkflowLifecycleContext>,
}

#[cfg(feature = "trusted-host-application-spi")]
impl<'a> TrustedHostLocalCheckAttemptExecutor<'a> {
    pub(crate) fn new(
        profile: &'a ResolvedExplicitLocalCheckProfile,
        binding_commitment: SpecContentHash,
    ) -> Self {
        Self {
            profile,
            binding_commitment,
            workflow_lifecycle: None,
        }
    }

    pub(crate) fn with_workflow_lifecycle(
        mut self,
        workflow_lifecycle: crate::trusted_host_workflow_lifecycle::TrustedHostWorkflowLifecycleContext,
    ) -> Self {
        self.workflow_lifecycle = Some(workflow_lifecycle);
        self
    }
}

#[cfg(feature = "trusted-host-application-spi")]
impl TrustedHostAttemptExecutor for TrustedHostLocalCheckAttemptExecutor<'_> {
    fn binding_commitment(&self) -> SpecContentHash {
        self.binding_commitment.clone()
    }

    fn workflow_lifecycle_context(
        &self,
    ) -> Option<&crate::trusted_host_workflow_lifecycle::TrustedHostWorkflowLifecycleContext> {
        self.workflow_lifecycle.as_ref()
    }

    fn execute(
        &self,
        _context: &TrustedHostAttemptExecutionContext<'_>,
    ) -> TrustedHostAttemptExecutionResult {
        match self.profile.execute() {
            Ok(result) if result.status() == LocalCheckResultStatus::Passed => {
                TrustedHostAttemptExecutionResult::Succeeded(result.to_skill_output())
            }
            Ok(_) | Err(_) => TrustedHostAttemptExecutionResult::TerminalFailure,
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
    Yielded(TrustedHostYieldRequest),
    AmbiguousMayHaveStarted,
}

pub(crate) struct TrustedHostYieldRequest {
    reason: AuthorizedExecutionYieldReason,
    time_window_wait: Option<TrustedHostTimeWindowWaitDeclaration>,
}

impl TrustedHostYieldRequest {
    pub(crate) fn without_wait(reason: AuthorizedExecutionYieldReason) -> Self {
        Self {
            reason,
            time_window_wait: None,
        }
    }

    pub(crate) fn with_time_window(
        reason: AuthorizedExecutionYieldReason,
        condition_id: AuthorizedExecutionWaitConditionId,
        condition_version: u32,
        deadline: Timestamp,
    ) -> Self {
        Self {
            reason,
            time_window_wait: Some(TrustedHostTimeWindowWaitDeclaration {
                condition_id,
                condition_version,
                deadline,
            }),
        }
    }
}

impl fmt::Debug for TrustedHostYieldRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostYieldRequest")
            .field("reason", &self.reason)
            .field("wait_count", &usize::from(self.time_window_wait.is_some()))
            .field("binding", &"[REDACTED]")
            .finish()
    }
}

struct TrustedHostTimeWindowWaitDeclaration {
    condition_id: AuthorizedExecutionWaitConditionId,
    condition_version: u32,
    deadline: Timestamp,
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

pub(crate) struct TrustedHostSupervisorBinding {
    pub(crate) window_id: crate::AuthorizedExecutionWindowId,
    pub(crate) expected_window_binding: ExpectedWindowBinding,
    pub(crate) operation_binding_commitment: SpecContentHash,
}

pub(crate) fn trusted_host_supervisor_binding(
    capability: &TrustedHostSupervisorAttemptCapability,
) -> TrustedHostSupervisorBinding {
    match capability {
        TrustedHostSupervisorAttemptCapability::Opened(capability) => {
            TrustedHostSupervisorBinding {
                window_id: capability.window_id.clone(),
                expected_window_binding: capability.expected_window_binding.clone(),
                operation_binding_commitment: capability.operation_binding_commitment.clone(),
            }
        }
        TrustedHostSupervisorAttemptCapability::Resumed {
            capability,
            expected_window_binding,
            operation_binding_commitment,
        } => TrustedHostSupervisorBinding {
            window_id: capability.window_id.clone(),
            expected_window_binding: (**expected_window_binding).clone(),
            operation_binding_commitment: operation_binding_commitment.clone(),
        },
    }
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

struct UnreservedSupervisorCapability {
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
    let invocation_commitment = trusted_host_invocation_commitment(
        &input.skill_input,
        &input.executor.binding_commitment(),
    );
    let capability = reserve_dispatch(
        input.backend,
        &input.persistence,
        input.executor,
        capability,
        invocation_commitment,
    )?;
    let execution = input.executor.execute(&TrustedHostAttemptExecutionContext {
        skill_input: &input.skill_input,
    });
    let skill_output = persist_execution_result(
        input.backend,
        &input.persistence,
        &capability,
        input.executor,
        execution,
    )?;

    Ok(TrustedHostSupervisorResult {
        disposition: input
            .backend
            .continuation_disposition(&capability.capability.window_id)?,
        skill_output,
    })
}

fn reserve_dispatch(
    backend: &SqliteStateBackend,
    persistence: &TrustedHostSupervisorPersistenceInput,
    executor: &dyn TrustedHostAttemptExecutor,
    capability: UnreservedSupervisorCapability,
    invocation_commitment: SpecContentHash,
) -> Result<SupervisorCapability, WorkflowOsError> {
    let mut reservation_request = DispatchReservationRequest {
        operation_id: DispatchReservationOperationId::new(persistence.operation.as_str())?,
        receipt_id: DispatchReservationReceiptId::new(persistence.receipt.as_str())?,
        request_commitment: SpecContentHash::from_text("pending dispatch reservation"),
        capability: capability.capability,
        expected_window_binding: capability.expected_window_binding,
        operation_binding_commitment: capability.operation_binding_commitment,
        invocation_commitment,
        executor_commitment: executor.binding_commitment(),
        workflow_lifecycle_commitment: executor.workflow_lifecycle_context().map(
            crate::trusted_host_workflow_lifecycle::TrustedHostWorkflowLifecycleContext::commitment,
        ),
    };
    reservation_request.request_commitment = dispatch_request_commitment(&reservation_request);
    let reservation = if let Some(lifecycle) = executor.workflow_lifecycle_context() {
        backend.reserve_attempt_dispatch_with_lifecycle(reservation_request, lifecycle)?
    } else {
        backend.reserve_attempt_dispatch(reservation_request)?
    };
    match reservation {
        DispatchReservationOutcome::Admitted { capability, .. } => {
            let parts = capability.into_parts();
            Ok(SupervisorCapability {
                capability: parts.attempt_capability,
                expected_window_binding: parts.expected_window_binding,
                operation_binding_commitment: parts.operation_binding_commitment,
            })
        }
        DispatchReservationOutcome::AlreadyAdmitted { receipt } => {
            let _binding = receipt.into_binding();
            Err(supervisor_error(
                WorkflowOsErrorKind::InvalidState,
                "attempt_already_admitted",
                "trusted-host attempt was already admitted",
            ))
        }
        DispatchReservationOutcome::CommittedButCapabilityUnavailable { receipt } => {
            let _binding = receipt.into_binding();
            Err(supervisor_error(
                WorkflowOsErrorKind::InvalidState,
                "dispatch_capability_unavailable",
                "trusted-host dispatch authority is unavailable",
            ))
        }
    }
}

fn persist_execution_result(
    backend: &SqliteStateBackend,
    persistence: &TrustedHostSupervisorPersistenceInput,
    capability: &SupervisorCapability,
    executor: &dyn TrustedHostAttemptExecutor,
    execution: TrustedHostAttemptExecutionResult,
) -> Result<Option<SkillOutput>, WorkflowOsError> {
    let mut skill_output = None;
    match execution {
        TrustedHostAttemptExecutionResult::Succeeded(output) => {
            if let Some(lifecycle) = executor.workflow_lifecycle_context() {
                let workflow_outcome = lifecycle.normalize_success(&output);
                persist_outcome(
                    backend,
                    persistence,
                    capability,
                    workflow_outcome.continuity_outcome(),
                    Some((lifecycle, &workflow_outcome)),
                )?;
                if matches!(
                    workflow_outcome,
                    crate::trusted_host_workflow_lifecycle::TrustedHostWorkflowTerminalOutcome::Succeeded { .. }
                ) {
                    skill_output = Some(output);
                }
            } else {
                persist_outcome(
                    backend,
                    persistence,
                    capability,
                    AuthorizedExecutionAttemptOutcome::Succeeded,
                    None,
                )?;
                skill_output = Some(output);
            }
        }
        TrustedHostAttemptExecutionResult::RetryableFailure => persist_outcome(
            backend,
            persistence,
            capability,
            AuthorizedExecutionAttemptOutcome::RetryableFailure,
            None,
        )?,
        TrustedHostAttemptExecutionResult::TerminalFailure => {
            if let Some(lifecycle) = executor.workflow_lifecycle_context() {
                let workflow_outcome = crate::trusted_host_workflow_lifecycle::TrustedHostWorkflowTerminalOutcome::terminal_failure();
                persist_outcome(
                    backend,
                    persistence,
                    capability,
                    workflow_outcome.continuity_outcome(),
                    Some((lifecycle, &workflow_outcome)),
                )?;
            } else {
                persist_outcome(
                    backend,
                    persistence,
                    capability,
                    AuthorizedExecutionAttemptOutcome::TerminalFailure,
                    None,
                )?;
            }
        }
        TrustedHostAttemptExecutionResult::Yielded(request) => {
            persist_yield(backend, persistence, capability, request)?;
        }
        TrustedHostAttemptExecutionResult::AmbiguousMayHaveStarted => {
            persist_ambiguous(backend, persistence, capability)?;
        }
    }
    Ok(skill_output)
}

fn normalize_capability(
    capability: TrustedHostSupervisorAttemptCapability,
) -> UnreservedSupervisorCapability {
    match capability {
        TrustedHostSupervisorAttemptCapability::Opened(opened) => UnreservedSupervisorCapability {
            capability: AttemptUseCapability {
                attempt_id: opened.attempt_id.clone(),
                subject_actor_id: opened.subject_actor_id.clone(),
                window_id: opened.window_id.clone(),
                window_revision: opened.window_revision,
                cursor: opened.cursor.clone(),
                authority_commitment: opened.authority_commitment.clone(),
                window_binding_commitment: opened.window_binding_commitment.clone(),
                consume_operation_id: opened.consume_operation_id.clone(),
                dispatch_reservation: None,
            },
            expected_window_binding: opened.expected_window_binding,
            operation_binding_commitment: opened.operation_binding_commitment,
        },
        TrustedHostSupervisorAttemptCapability::Resumed {
            capability,
            expected_window_binding,
            operation_binding_commitment,
        } => UnreservedSupervisorCapability {
            capability: AttemptUseCapability {
                attempt_id: capability.attempt_id,
                subject_actor_id: capability.subject_actor_id,
                window_id: capability.window_id,
                window_revision: capability.window_revision,
                cursor: capability.cursor,
                authority_commitment: capability.authority_commitment,
                window_binding_commitment: capability.window_binding_commitment,
                consume_operation_id: capability.consume_operation_id,
                dispatch_reservation: None,
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
    yield_request: TrustedHostYieldRequest,
) -> Result<(), WorkflowOsError> {
    let generation_id = persistence.yield_generation.clone().ok_or_else(|| {
        supervisor_error(
            WorkflowOsErrorKind::Validation,
            "yield_generation_missing",
            "trusted-host yield requires a generation identity",
        )
    })?;
    let waits = match yield_request.time_window_wait {
        Some(declaration) => {
            if declaration.condition_version == 0 {
                return Err(supervisor_error(
                    WorkflowOsErrorKind::Validation,
                    "wait_declaration_invalid",
                    "trusted-host wait declaration is invalid",
                ));
            }
            vec![SeedWait {
                condition_id: declaration.condition_id,
                condition_version: declaration.condition_version,
                wake_trigger: AuthorizedExecutionWakeTriggerKind::DeadlineReached,
                dependency_binding: Some(backend.derive_time_window_wait_binding(
                    &capability.capability.window_id,
                    declaration.deadline,
                )?),
            }]
        }
        None => Vec::new(),
    };
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
        reason: yield_request.reason,
        waits,
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
    workflow_lifecycle: Option<(
        &crate::trusted_host_workflow_lifecycle::TrustedHostWorkflowLifecycleContext,
        &crate::trusted_host_workflow_lifecycle::TrustedHostWorkflowTerminalOutcome,
    )>,
) -> Result<(), WorkflowOsError> {
    let mut request = RecordAttemptOutcomeRequest {
        operation_id: persistence.operation.clone(),
        request_commitment: SpecContentHash::from_text("pending supervisor outcome"),
        receipt_id: persistence.receipt.clone(),
        window_id: capability.capability.window_id.clone(),
        expected_window_revision: capability.capability.window_revision,
        expected_window_binding: capability.expected_window_binding.clone(),
        attempt_id: capability.capability.attempt_id.clone(),
        expected_attempt_revision: capability.capability.window_revision,
        attempt_capability: &capability.capability,
        outcome,
    };
    request.request_commitment = expected_attempt_outcome_commitment(&request);
    if let Some((lifecycle, workflow_outcome)) = workflow_lifecycle {
        request.request_commitment =
            lifecycle.outcome_request_commitment(&request.request_commitment, workflow_outcome);
    }
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
    let persisted = if let Some((lifecycle, workflow_outcome)) = workflow_lifecycle {
        backend.record_trusted_host_attempt_outcome_projected(&request, lifecycle, workflow_outcome)
    } else {
        backend.record_attempt_outcome_projected(request)
    };
    let result = match persisted {
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
        expected_attempt_revision: capability.capability.window_revision,
        dispatch_reservation: capability.capability.dispatch_reservation.clone(),
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

#[cfg(all(test, feature = "trusted-host-application-spi"))]
mod local_check_attempt_tests {
    #![allow(clippy::expect_used)]

    use std::collections::BTreeMap;
    use std::fs;
    use std::sync::Arc;

    use super::*;
    use crate::{
        CorrelationId, ExplicitLocalCheckProfileSelection, LocalCheckProcessOutput,
        LocalCheckProcessRequest, LocalCheckProcessRunner, SchemaVersion, SkillId, SkillVersion,
        StepId, WorkflowId, WorkflowRunId, WorkflowVersion,
    };

    #[derive(Debug)]
    struct TimedOutRunner;

    impl LocalCheckProcessRunner for TimedOutRunner {
        fn run(
            &self,
            _request: &LocalCheckProcessRequest,
        ) -> Result<LocalCheckProcessOutput, WorkflowOsError> {
            Ok(LocalCheckProcessOutput::timed_out(
                120_000,
                b"private-timeout-output".to_vec(),
                b"private-timeout-error".to_vec(),
            ))
        }
    }

    #[test]
    fn timed_out_closed_profile_is_terminal_failure() {
        let root = std::env::temp_dir().join(format!(
            "workflow-os-trusted-host-timeout-{}",
            std::process::id()
        ));
        fs::create_dir_all(&root).expect("fixture root");
        fs::write(
            root.join("workflow-os.yml"),
            "schema_version: workflowos.dev/v0\n",
        )
        .expect("fixture manifest");
        let profile = ExplicitLocalCheckProfileSelection::workflow_os_project_validation()
            .resolve_with_process_runner(
                std::env::current_exe().expect("test executable"),
                root.clone(),
                Arc::new(TimedOutRunner),
            )
            .expect("resolved profile");
        let executor = TrustedHostLocalCheckAttemptExecutor::new(
            &profile,
            SpecContentHash::from_text("timeout-binding"),
        );
        let input = SkillInput {
            run_id: WorkflowRunId::new("run-timeout-adapter").expect("run id"),
            workflow_id: WorkflowId::new("test/timeout-adapter").expect("workflow id"),
            workflow_version: WorkflowVersion::new("v0").expect("workflow version"),
            schema_version: SchemaVersion::new("workflowos.dev/v0").expect("schema version"),
            spec_hash: SpecContentHash::from_text("timeout-spec"),
            step_id: StepId::new("timeout-check").expect("step id"),
            skill_id: SkillId::new("local/workflow-os-validate").expect("skill id"),
            skill_version: SkillVersion::new("v0").expect("skill version"),
            correlation_id: CorrelationId::new("correlation-timeout-adapter")
                .expect("correlation id"),
            values: BTreeMap::new(),
        };

        assert!(matches!(
            executor.execute(&TrustedHostAttemptExecutionContext {
                skill_input: &input
            }),
            TrustedHostAttemptExecutionResult::TerminalFailure
        ));
        let _ = fs::remove_dir_all(root);
    }
}
