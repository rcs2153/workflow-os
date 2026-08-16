#![allow(
    clippy::expect_used,
    clippy::manual_let_else,
    clippy::panic,
    clippy::too_many_arguments
)]

use std::collections::BTreeMap;

use crate::{
    ActorId, AuthorizedExecutionAttemptId, AuthorizedExecutionAttemptOutcome,
    AuthorizedExecutionWaitConditionId, AuthorizedExecutionWakeTriggerKind,
    AuthorizedExecutionWindowId, AuthorizedExecutionYieldReason, EventId, EventSequenceNumber,
    ImmutableRunBundleBinding, SpecContentHash, StepId, Timestamp, WorkflowId, WorkflowRunId,
};

use super::internal::{
    expected_attempt_outcome_commitment, expected_consume_directive_commitment,
    expected_recovery_commitment, expected_register_yield_commitment,
    expected_transition_wait_commitment, operation_commitment, trusted_time_commitment,
    trusted_time_observation, window_binding_commitment, AttemptUseCapability,
    AuthoritativeAttemptRecord, AuthoritativeAttemptState, AuthoritativeDirectiveRecord,
    AuthoritativeDirectiveState, AuthoritativeOperationRecord, AuthoritativeWaitIdentity,
    AuthoritativeWaitRecord, AuthoritativeWaitState, AuthoritativeWindowRecord,
    AuthoritativeWindowState, AuthoritativeYieldRecord, AuthorityUseCapability,
    AuthorizedExecutionContinuityEligibilityReader, AuthorizedExecutionContinuityReconciler,
    AuthorizedExecutionContinuityStore, CommittedOperationDisposition, ConsumeDirectiveRequest,
    ConsumeDirectiveResult, ContinuityCursor, ContinuityDirectiveId, ContinuityInstanceEligibility,
    ContinuityOperationId, ContinuityReceipt, ContinuityReceiptId, ContinuityReconciliationResult,
    ContinuityRevision, ContinuityTrustedTimeEpochId, ContinuityWakeSourceReference,
    ContinuityYieldGenerationId, ExpectedWaitRevision, ExpectedWindowBinding, MutationResult,
    ReconcileOperationRequest, RecordAttemptOutcomeRequest, RecordedOperationResult,
    RecoverAmbiguousAttemptRequest, ReferenceContinuityState, RegisterYieldRequest,
    RegisterYieldResult, TransitionWaitRequest, TrustedTimePosture, TrustedTimeSecurityRecord,
    TrustedTimeSourceKind, WakeAssessmentCapability,
};
use super::AuthorizedExecutionContinuityOperationKind;

/// Commit boundary at which a conformance backend injects one deterministic fault.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ContinuityConformanceFault {
    Before,
    During,
    After,
}

/// Test-only adapter used by named backend-parametric continuity scenarios.
///
/// Production bootstrap and bearer-capability minting intentionally remain
/// outside this interface.
pub(crate) trait ContinuityConformanceBackend:
    AuthorizedExecutionContinuityStore
    + AuthorizedExecutionContinuityReconciler
    + AuthorizedExecutionContinuityEligibilityReader
    + Clone
    + Send
    + Sync
    + 'static
    + Sized
{
    /// Returns the stable clock provenance expected by seeded conformance state.
    fn conformance_clock_provenance() -> SpecContentHash;

    /// Returns the stable clock epoch expected by seeded conformance state.
    fn conformance_clock_epoch() -> ContinuityTrustedTimeEpochId;

    /// Returns a read-only normalized authoritative snapshot.
    fn conformance_snapshot(&self) -> ReferenceContinuityState;

    /// Reopens a logically independent backend over the supplied durable state.
    fn conformance_reopen(state: ReferenceContinuityState) -> Self;

    /// Reopens the backend's current durable state without changing its contents.
    fn conformance_reopen_current(&self) -> Self {
        Self::conformance_reopen(self.conformance_snapshot())
    }

    /// Sets the deterministic trusted-time observation used by the next operation.
    fn conformance_set_time(&self, observed_at: Timestamp);

    /// Sets whether the injected trusted-time source is available.
    fn conformance_set_time_available(&self, available: bool);

    /// Rebinds the injected clock provenance for security-rejection scenarios.
    fn conformance_set_time_provenance(&self, provenance: SpecContentHash);

    /// Rebinds the injected clock epoch for epoch-mismatch scenarios.
    fn conformance_set_time_epoch(&self, epoch_id: ContinuityTrustedTimeEpochId);

    /// Injects one deterministic operation/commit-phase fault.
    fn conformance_inject_fault(&self, fault: ContinuityConformanceFault);
}

#[derive(Clone)]
pub(crate) struct ContinuityConformanceFixture<B> {
    pub(crate) backend: B,
    pub(crate) window_id: AuthorizedExecutionWindowId,
    pub(crate) attempt_id: AuthorizedExecutionAttemptId,
    pub(crate) generation_id: ContinuityYieldGenerationId,
    pub(crate) directive_id: ContinuityDirectiveId,
    pub(crate) wait_id: Option<AuthorizedExecutionWaitConditionId>,
    pub(crate) cursor: ContinuityCursor,
}

impl<B: ContinuityConformanceBackend> ContinuityConformanceFixture<B> {
    #[allow(clippy::too_many_lines)]
    pub(crate) fn new(yielded: bool, with_wait: bool) -> Self {
        let window_id = AuthorizedExecutionWindowId::new("window/conformance").expect("window");
        let attempt_id =
            AuthorizedExecutionAttemptId::new("attempt/conformance/1").expect("attempt");
        let generation_id = ContinuityYieldGenerationId::new("yield/conformance/1").expect("yield");
        let directive_id =
            ContinuityDirectiveId::new("directive/conformance/1").expect("directive");
        let wait_id = with_wait
            .then(|| AuthorizedExecutionWaitConditionId::new("wait/conformance/1").expect("wait"));
        let cursor = ContinuityCursor {
            sequence_number: EventSequenceNumber::new(7).expect("sequence"),
            event_id: EventId::new("event/conformance/7").expect("event"),
        };
        let revision = ContinuityRevision::new(1).expect("revision");
        let watermark = time("2026-08-15T12:00:00Z");
        let bundle: ImmutableRunBundleBinding = serde_json::from_value(serde_json::json!({
            "bundle_id": "bundle/conformance",
            "bundle_version": "v1",
            "root_hash": SpecContentHash::from_text("bundle").as_str()
        }))
        .expect("bundle");
        let seed_operation_id =
            ContinuityOperationId::new("operation/conformance-seed-consume").expect("operation");
        let authority = SpecContentHash::from_text("conformance authority");
        let window = AuthoritativeWindowRecord {
            workflow_id: WorkflowId::new("workflow/conformance").expect("workflow"),
            run_id: WorkflowRunId::new("run/conformance").expect("run"),
            step_id: StepId::new("step-conformance").expect("step"),
            window_id: window_id.clone(),
            subject_actor_id: ActorId::new("agent/conformance").expect("actor"),
            immutable_run_bundle: bundle,
            governance_commitment: SpecContentHash::from_text("governance"),
            authority_commitment: authority.clone(),
            cursor: cursor.clone(),
            state: if yielded {
                AuthoritativeWindowState::Yielded
            } else {
                AuthoritativeWindowState::Executing
            },
            maximum_attempts: 3,
            next_attempt_number: 2,
            expires_at: time("2026-08-15T13:00:00Z"),
            trusted_time_watermark: watermark,
            trusted_time_epoch_id: B::conformance_clock_epoch(),
            revision,
            active_yield: yielded.then(|| generation_id.clone()),
        };
        let attempt = AuthoritativeAttemptRecord {
            attempt_id: attempt_id.clone(),
            attempt_number: 1,
            window_id: window_id.clone(),
            subject_actor_id: window.subject_actor_id.clone(),
            cursor: cursor.clone(),
            authority_commitment: authority.clone(),
            consume_operation_id: seed_operation_id.clone(),
            state: if yielded {
                AuthoritativeAttemptState::Yielded
            } else {
                AuthoritativeAttemptState::Started
            },
            revision,
        };
        let observation = trusted_time_observation(
            watermark,
            TrustedTimeSourceKind::CoreInjectedClockV1,
            B::conformance_clock_provenance(),
            B::conformance_clock_epoch(),
        );
        let seed_result = RecordedOperationResult::DirectiveConsumed {
            window_id: window_id.clone(),
            directive_id: directive_id.clone(),
            generation_id: generation_id.clone(),
            attempt_id: attempt_id.clone(),
            attempt_number: 1,
            directive_state: AuthoritativeDirectiveState::Consumed,
            attempt_state: AuthoritativeAttemptState::Started,
            window_state: AuthoritativeWindowState::Executing,
            window_revision: revision,
        };
        let disposition = CommittedOperationDisposition::CommittedSuccess(seed_result);
        let request_commitment = SpecContentHash::from_text("seed request");
        let receipt_id = ContinuityReceiptId::new("receipt/conformance-seed").expect("receipt");
        let trusted = trusted_time_commitment(&observation);
        let committed = operation_commitment(
            &request_commitment,
            &receipt_id,
            &observation,
            &trusted,
            &disposition,
        );
        let operation = AuthoritativeOperationRecord {
            operation_id: seed_operation_id.clone(),
            operation_kind: AuthorizedExecutionContinuityOperationKind::ConsumeDirective,
            request_commitment,
            operation_commitment: committed.clone(),
            receipt: ContinuityReceipt {
                receipt_id,
                operation_kind: AuthorizedExecutionContinuityOperationKind::ConsumeDirective,
                operation_commitment: committed,
                trusted_time_commitment: trusted,
                committed_at: watermark,
            },
            trusted_time: observation,
            disposition,
        };
        let mut yields = BTreeMap::new();
        let mut directives = BTreeMap::new();
        let mut waits = BTreeMap::new();
        if yielded {
            let wait_ids = wait_id
                .iter()
                .cloned()
                .map(|id| AuthoritativeWaitIdentity::new(id, 1))
                .collect::<Vec<_>>();
            yields.insert(
                generation_id.clone(),
                AuthoritativeYieldRecord {
                    generation_id: generation_id.clone(),
                    attempt_id: attempt_id.clone(),
                    cursor: cursor.clone(),
                    reason: AuthorizedExecutionYieldReason::TurnBoundary,
                    wait_ids,
                    registered_at: watermark,
                },
            );
            directives.insert(
                directive_id.clone(),
                AuthoritativeDirectiveRecord {
                    directive_id: directive_id.clone(),
                    window_id: window_id.clone(),
                    generation_id: generation_id.clone(),
                    cursor: cursor.clone(),
                    authority_commitment: authority,
                    state: AuthoritativeDirectiveState::Available,
                    revision,
                },
            );
            if let Some(id) = wait_id.as_ref() {
                waits.insert(
                    AuthoritativeWaitIdentity::new(id.clone(), 1),
                    AuthoritativeWaitRecord {
                        condition_id: id.clone(),
                        condition_version: 1,
                        window_id: window_id.clone(),
                        generation_id: generation_id.clone(),
                        wake_trigger: AuthorizedExecutionWakeTriggerKind::EvidenceAccepted,
                        state: AuthoritativeWaitState::Unsatisfied,
                        source_commitment: None,
                        source_revision: None,
                        revision,
                    },
                );
            }
        }
        let state = ReferenceContinuityState {
            trusted_time: TrustedTimeSecurityRecord {
                source: TrustedTimeSourceKind::CoreInjectedClockV1,
                provenance_commitment: B::conformance_clock_provenance(),
                epoch_id: B::conformance_clock_epoch(),
                last_observed_at: Some(watermark),
                posture: TrustedTimePosture::Healthy,
                eligibility: ContinuityInstanceEligibility::LiveStateEligible,
                revision,
            },
            windows: BTreeMap::from([(window_id.clone(), window)]),
            yields,
            waits,
            directives,
            attempts: BTreeMap::from([(attempt_id.clone(), attempt)]),
            operations: BTreeMap::from([(seed_operation_id, operation)]),
        };
        Self {
            backend: B::conformance_reopen(state),
            window_id,
            attempt_id,
            generation_id,
            directive_id,
            wait_id,
            cursor,
        }
    }

    pub(crate) fn binding(&self) -> ExpectedWindowBinding {
        let state = self.backend.conformance_snapshot();
        let window = state.windows.get(&self.window_id).expect("window");
        ExpectedWindowBinding {
            workflow_id: window.workflow_id.clone(),
            run_id: window.run_id.clone(),
            step_id: window.step_id.clone(),
            subject_actor_id: window.subject_actor_id.clone(),
            immutable_run_bundle: window.immutable_run_bundle.clone(),
            governance_commitment: window.governance_commitment.clone(),
            authority_commitment: window.authority_commitment.clone(),
            cursor: window.cursor.clone(),
        }
    }

    pub(crate) fn reopen_with_state(&self, state: ReferenceContinuityState) -> Self {
        Self {
            backend: B::conformance_reopen(state),
            window_id: self.window_id.clone(),
            attempt_id: self.attempt_id.clone(),
            generation_id: self.generation_id.clone(),
            directive_id: self.directive_id.clone(),
            wait_id: self.wait_id.clone(),
            cursor: self.cursor.clone(),
        }
    }

    pub(crate) fn reopen_current(&self) -> Self {
        Self {
            backend: self.backend.conformance_reopen_current(),
            window_id: self.window_id.clone(),
            attempt_id: self.attempt_id.clone(),
            generation_id: self.generation_id.clone(),
            directive_id: self.directive_id.clone(),
            wait_id: self.wait_id.clone(),
            cursor: self.cursor.clone(),
        }
    }

    pub(crate) fn attempt_capability(&self) -> AttemptUseCapability {
        let state = self.backend.conformance_snapshot();
        let window = state.windows.get(&self.window_id).expect("window");
        let attempt = state.attempts.get(&self.attempt_id).expect("attempt");
        AttemptUseCapability {
            attempt_id: self.attempt_id.clone(),
            subject_actor_id: window.subject_actor_id.clone(),
            window_id: self.window_id.clone(),
            window_revision: window.revision,
            cursor: self.cursor.clone(),
            authority_commitment: window.authority_commitment.clone(),
            window_binding_commitment: window_binding_commitment(&self.binding()),
            consume_operation_id: attempt.consume_operation_id.clone(),
        }
    }

    pub(crate) fn authority_capability(&self) -> AuthorityUseCapability {
        let state = self.backend.conformance_snapshot();
        let window = state.windows.get(&self.window_id).expect("window");
        let expected_waits = state
            .yields
            .get(&self.generation_id)
            .expect("yield")
            .wait_ids
            .iter()
            .map(|id| {
                let wait = state.waits.get(id).expect("wait");
                ExpectedWaitRevision {
                    condition_id: id.condition_id.clone(),
                    condition_version: id.condition_version,
                    revision: wait.revision,
                }
            })
            .collect();
        AuthorityUseCapability {
            window_id: self.window_id.clone(),
            window_revision: window.revision,
            generation_id: self.generation_id.clone(),
            cursor: self.cursor.clone(),
            subject_actor_id: window.subject_actor_id.clone(),
            authority_commitment: window.authority_commitment.clone(),
            window_binding_commitment: window_binding_commitment(&self.binding()),
            expected_waits,
        }
    }

    pub(crate) fn register_request<'a>(
        &self,
        capability: &'a AttemptUseCapability,
    ) -> RegisterYieldRequest<'a> {
        let state = self.backend.conformance_snapshot();
        let window = state.windows.get(&self.window_id).expect("window");
        let mut request = RegisterYieldRequest {
            operation_id: ContinuityOperationId::new("operation/conformance-register")
                .expect("operation"),
            request_commitment: SpecContentHash::from_text("pending"),
            receipt_id: ContinuityReceiptId::new("receipt/conformance-register").expect("receipt"),
            generation_id: ContinuityYieldGenerationId::new("yield/conformance/2").expect("yield"),
            window_id: self.window_id.clone(),
            expected_window_revision: window.revision,
            expected_window_binding: self.binding(),
            cursor: self.cursor.clone(),
            attempt_id: self.attempt_id.clone(),
            attempt_capability: capability,
            reason: AuthorizedExecutionYieldReason::ContextBudget,
            waits: Vec::new(),
        };
        request.request_commitment = expected_register_yield_commitment(&request);
        request
    }

    pub(crate) fn wait_request<'a>(
        &self,
        wake: Option<&'a WakeAssessmentCapability>,
    ) -> TransitionWaitRequest<'a> {
        let state = self.backend.conformance_snapshot();
        let window = state.windows.get(&self.window_id).expect("window");
        let wait_id = self.wait_id.clone().expect("wait");
        let wait = state
            .waits
            .get(&AuthoritativeWaitIdentity::new(wait_id.clone(), 1))
            .expect("wait");
        let mut request = TransitionWaitRequest {
            operation_id: ContinuityOperationId::new("operation/conformance-wait")
                .expect("operation"),
            request_commitment: SpecContentHash::from_text("pending"),
            receipt_id: ContinuityReceiptId::new("receipt/conformance-wait").expect("receipt"),
            window_id: self.window_id.clone(),
            expected_window_revision: window.revision,
            expected_window_binding: self.binding(),
            cursor: self.cursor.clone(),
            condition_id: wait_id,
            expected_generation_id: self.generation_id.clone(),
            expected_condition_version: 1,
            expected_wait_revision: wait.revision,
            target: AuthoritativeWaitState::Satisfied,
            wake_capability: wake,
        };
        request.request_commitment = expected_transition_wait_commitment(&request);
        request
    }

    pub(crate) fn wake_capability(&self) -> WakeAssessmentCapability {
        WakeAssessmentCapability {
            window_id: self.window_id.clone(),
            generation_id: self.generation_id.clone(),
            condition_id: self.wait_id.clone().expect("wait"),
            condition_version: 1,
            trigger: AuthorizedExecutionWakeTriggerKind::EvidenceAccepted,
            source_reference: ContinuityWakeSourceReference::new("evidence/conformance/1")
                .expect("source"),
            source_commitment: SpecContentHash::from_text("source"),
            source_revision: 1,
        }
    }

    pub(crate) fn consume_request(
        &self,
        capability: AuthorityUseCapability,
    ) -> ConsumeDirectiveRequest {
        let state = self.backend.conformance_snapshot();
        let window = state.windows.get(&self.window_id).expect("window");
        let mut request = ConsumeDirectiveRequest {
            operation_id: ContinuityOperationId::new("operation/conformance-consume")
                .expect("operation"),
            request_commitment: SpecContentHash::from_text("pending"),
            receipt_id: ContinuityReceiptId::new("receipt/conformance-consume").expect("receipt"),
            directive_id: self.directive_id.clone(),
            window_id: self.window_id.clone(),
            expected_window_revision: window.revision,
            expected_window_binding: self.binding(),
            generation_id: self.generation_id.clone(),
            cursor: self.cursor.clone(),
            expected_waits: capability.expected_waits.clone(),
            authority_capability: capability,
            generated_attempt_id: AuthorizedExecutionAttemptId::new("attempt/conformance/2")
                .expect("attempt"),
        };
        request.request_commitment = expected_consume_directive_commitment(&request);
        request
    }

    pub(crate) fn outcome_request<'a>(
        &self,
        capability: &'a AttemptUseCapability,
        outcome: AuthorizedExecutionAttemptOutcome,
    ) -> RecordAttemptOutcomeRequest<'a> {
        let state = self.backend.conformance_snapshot();
        let window = state.windows.get(&self.window_id).expect("window");
        let attempt = state.attempts.get(&self.attempt_id).expect("attempt");
        let mut request = RecordAttemptOutcomeRequest {
            operation_id: ContinuityOperationId::new("operation/conformance-outcome")
                .expect("operation"),
            request_commitment: SpecContentHash::from_text("pending"),
            receipt_id: ContinuityReceiptId::new("receipt/conformance-outcome").expect("receipt"),
            window_id: self.window_id.clone(),
            expected_window_revision: window.revision,
            expected_window_binding: self.binding(),
            attempt_id: self.attempt_id.clone(),
            expected_attempt_revision: attempt.revision,
            attempt_capability: capability,
            outcome,
        };
        request.request_commitment = expected_attempt_outcome_commitment(&request);
        request
    }

    pub(crate) fn recovery_request(&self) -> RecoverAmbiguousAttemptRequest {
        let state = self.backend.conformance_snapshot();
        let window = state.windows.get(&self.window_id).expect("window");
        let attempt = state.attempts.get(&self.attempt_id).expect("attempt");
        let mut request = RecoverAmbiguousAttemptRequest {
            operation_id: ContinuityOperationId::new("operation/conformance-recover")
                .expect("operation"),
            request_commitment: SpecContentHash::from_text("pending"),
            receipt_id: ContinuityReceiptId::new("receipt/conformance-recover").expect("receipt"),
            window_id: self.window_id.clone(),
            expected_window_revision: window.revision,
            expected_window_binding: self.binding(),
            cursor: self.cursor.clone(),
            attempt_id: self.attempt_id.clone(),
            expected_attempt_revision: attempt.revision,
        };
        request.request_commitment = expected_recovery_commitment(&request);
        request
    }
}

fn time(value: &str) -> Timestamp {
    Timestamp::parse_rfc3339(value).expect("time")
}

fn assert_commit_ambiguity<B: ContinuityConformanceBackend>(
    backend: &B,
    operation_id: ContinuityOperationId,
    request_commitment: SpecContentHash,
    receipt_id: ContinuityReceiptId,
) {
    assert!(matches!(
        backend.reconcile_operation(&ReconcileOperationRequest {
            operation_id,
            expected_request_commitment: request_commitment,
            expected_receipt_id: receipt_id,
        }),
        ContinuityReconciliationResult::DurablyCommitted(_)
    ));
}

pub(crate) fn scenario_register_yield_replay_and_conflict<B: ContinuityConformanceBackend>() {
    let fixture = ContinuityConformanceFixture::<B>::new(false, false);
    let capability = fixture.attempt_capability();
    let request = fixture.register_request(&capability);
    let replay = fixture.register_request(&capability);
    assert!(matches!(
        fixture.backend.register_yield(request),
        Ok(RegisterYieldResult::Registered(_))
    ));
    fixture.backend.conformance_set_time_available(false);
    assert!(matches!(
        fixture.backend.register_yield(replay),
        Ok(RegisterYieldResult::ExactReplay(_))
    ));
}

pub(crate) fn scenario_transition_wait_replay<B: ContinuityConformanceBackend>() {
    let fixture = ContinuityConformanceFixture::<B>::new(true, true);
    let wake = fixture.wake_capability();
    let request = fixture.wait_request(Some(&wake));
    let replay = fixture.wait_request(Some(&wake));
    assert!(matches!(
        fixture.backend.transition_wait(request),
        Ok(MutationResult::Recorded(_))
    ));
    assert!(matches!(
        fixture.backend.transition_wait(replay),
        Ok(MutationResult::ExactReplay(_))
    ));
}

pub(crate) fn scenario_consume_directive_replay<B: ContinuityConformanceBackend>() {
    let fixture = ContinuityConformanceFixture::<B>::new(true, false);
    let request = fixture.consume_request(fixture.authority_capability());
    let replay = fixture.consume_request(fixture.authority_capability());
    assert!(matches!(
        fixture.backend.consume_directive(request),
        Ok(ConsumeDirectiveResult::Consumed { .. })
    ));
    assert!(matches!(
        fixture.backend.consume_directive(replay),
        Ok(ConsumeDirectiveResult::ExactReplay(_))
    ));
}

pub(crate) fn scenario_attempt_outcomes<B: ContinuityConformanceBackend>() {
    for outcome in [
        AuthorizedExecutionAttemptOutcome::Succeeded,
        AuthorizedExecutionAttemptOutcome::RetryableFailure,
        AuthorizedExecutionAttemptOutcome::TerminalFailure,
    ] {
        let fixture = ContinuityConformanceFixture::<B>::new(false, false);
        let capability = fixture.attempt_capability();
        let request = fixture.outcome_request(&capability, outcome);
        assert!(matches!(
            fixture.backend.record_attempt_outcome(request),
            Ok(MutationResult::Recorded(_))
        ));
    }
    let fixture = ContinuityConformanceFixture::<B>::new(false, false);
    assert!(matches!(
        fixture
            .backend
            .recover_ambiguous_attempt(fixture.recovery_request()),
        Ok(MutationResult::Recorded(_))
    ));
}

pub(crate) fn scenario_replay_conflicts_and_receipt_uniqueness<B: ContinuityConformanceBackend>() {
    let fixture = ContinuityConformanceFixture::<B>::new(false, false);
    let capability = fixture.attempt_capability();
    let first = fixture.register_request(&capability);
    let mut conflict = fixture.register_request(&capability);
    conflict.reason = AuthorizedExecutionYieldReason::HostPreemption;
    conflict.request_commitment = expected_register_yield_commitment(&conflict);
    assert!(matches!(
        fixture.backend.register_yield(first),
        Ok(RegisterYieldResult::Registered(_))
    ));
    let error = match fixture.backend.register_yield(conflict) {
        Err(error) => error,
        Ok(_) => panic!("register replay conflict must fail"),
    };
    assert!(error.code().ends_with("operation.replay_conflict"));

    let fixture = ContinuityConformanceFixture::<B>::new(true, true);
    let wake = fixture.wake_capability();
    let first = fixture.wait_request(Some(&wake));
    let mut conflict = fixture.wait_request(Some(&wake));
    conflict.expected_wait_revision = ContinuityRevision::new(2).expect("revision");
    conflict.request_commitment = expected_transition_wait_commitment(&conflict);
    assert!(matches!(
        fixture.backend.transition_wait(first),
        Ok(MutationResult::Recorded(_))
    ));
    let error = match fixture.backend.transition_wait(conflict) {
        Err(error) => error,
        Ok(_) => panic!("wait replay conflict must fail"),
    };
    assert!(error.code().ends_with("operation.replay_conflict"));

    let fixture = ContinuityConformanceFixture::<B>::new(true, false);
    let first = fixture.consume_request(fixture.authority_capability());
    let mut conflict = fixture.consume_request(fixture.authority_capability());
    conflict.generated_attempt_id =
        AuthorizedExecutionAttemptId::new("attempt/conformance/conflict").expect("attempt");
    conflict.request_commitment = expected_consume_directive_commitment(&conflict);
    assert!(matches!(
        fixture.backend.consume_directive(first),
        Ok(ConsumeDirectiveResult::Consumed { .. })
    ));
    let error = match fixture.backend.consume_directive(conflict) {
        Err(error) => error,
        Ok(_) => panic!("consume replay conflict must fail"),
    };
    assert!(error.code().ends_with("operation.replay_conflict"));

    let fixture = ContinuityConformanceFixture::<B>::new(false, false);
    let capability = fixture.attempt_capability();
    let first = fixture.outcome_request(&capability, AuthorizedExecutionAttemptOutcome::Succeeded);
    let mut conflict = fixture.outcome_request(
        &capability,
        AuthorizedExecutionAttemptOutcome::TerminalFailure,
    );
    conflict.request_commitment = expected_attempt_outcome_commitment(&conflict);
    assert!(matches!(
        fixture.backend.record_attempt_outcome(first),
        Ok(MutationResult::Recorded(_))
    ));
    let error = match fixture.backend.record_attempt_outcome(conflict) {
        Err(error) => error,
        Ok(_) => panic!("outcome replay conflict must fail"),
    };
    assert!(error.code().ends_with("operation.replay_conflict"));

    let fixture = ContinuityConformanceFixture::<B>::new(false, false);
    let first = fixture.recovery_request();
    let mut conflict = fixture.recovery_request();
    conflict.expected_attempt_revision = ContinuityRevision::new(2).expect("revision");
    conflict.request_commitment = expected_recovery_commitment(&conflict);
    assert!(matches!(
        fixture.backend.recover_ambiguous_attempt(first),
        Ok(MutationResult::Recorded(_))
    ));
    let error = match fixture.backend.recover_ambiguous_attempt(conflict) {
        Err(error) => error,
        Ok(_) => panic!("recovery replay conflict must fail"),
    };
    assert!(error.code().ends_with("operation.replay_conflict"));

    let fixture = ContinuityConformanceFixture::<B>::new(false, false);
    let capability = fixture.attempt_capability();
    let mut reused_receipt = fixture.register_request(&capability);
    reused_receipt.receipt_id =
        ContinuityReceiptId::new("receipt/conformance-seed").expect("receipt");
    reused_receipt.request_commitment = expected_register_yield_commitment(&reused_receipt);
    let error = match fixture.backend.register_yield(reused_receipt) {
        Err(error) => error,
        Ok(_) => panic!("cross-operation receipt reuse must fail"),
    };
    assert!(error.code().ends_with("receipt.reused"));
}

pub(crate) fn scenario_concurrent_one_winner_and_attempt_budget<B: ContinuityConformanceBackend>() {
    let fixture = ContinuityConformanceFixture::<B>::new(true, false);
    let mut first = fixture.consume_request(fixture.authority_capability());
    first.operation_id =
        ContinuityOperationId::new("operation/conformance-consume-a").expect("operation");
    first.receipt_id = ContinuityReceiptId::new("receipt/conformance-consume-a").expect("receipt");
    first.generated_attempt_id =
        AuthorizedExecutionAttemptId::new("attempt/conformance/a").expect("attempt");
    first.request_commitment = expected_consume_directive_commitment(&first);
    let mut second = fixture.consume_request(fixture.authority_capability());
    second.operation_id =
        ContinuityOperationId::new("operation/conformance-consume-b").expect("operation");
    second.receipt_id = ContinuityReceiptId::new("receipt/conformance-consume-b").expect("receipt");
    second.generated_attempt_id =
        AuthorizedExecutionAttemptId::new("attempt/conformance/b").expect("attempt");
    second.request_commitment = expected_consume_directive_commitment(&second);
    let left = fixture.backend.clone();
    let right = fixture.backend.clone();
    let (left_result, right_result) = std::thread::scope(|scope| {
        let left = scope.spawn(move || left.consume_directive(first));
        let right = scope.spawn(move || right.consume_directive(second));
        (
            left.join().expect("left consumer"),
            right.join().expect("right consumer"),
        )
    });
    let winners = [&left_result, &right_result]
        .into_iter()
        .filter(|result| matches!(result, Ok(ConsumeDirectiveResult::Consumed { .. })))
        .count();
    assert_eq!(winners, 1);
    assert_eq!(
        usize::from(left_result.is_err()) + usize::from(right_result.is_err()),
        1
    );

    let fixture = ContinuityConformanceFixture::<B>::new(false, false);
    let capability = fixture.attempt_capability();
    let mut first = fixture.register_request(&capability);
    first.operation_id =
        ContinuityOperationId::new("operation/conformance-register-a").expect("operation");
    first.receipt_id = ContinuityReceiptId::new("receipt/conformance-register-a").expect("receipt");
    first.generation_id = ContinuityYieldGenerationId::new("yield/conformance/a").expect("yield");
    first.request_commitment = expected_register_yield_commitment(&first);
    let mut second = fixture.register_request(&capability);
    second.operation_id =
        ContinuityOperationId::new("operation/conformance-register-b").expect("operation");
    second.receipt_id =
        ContinuityReceiptId::new("receipt/conformance-register-b").expect("receipt");
    second.generation_id = ContinuityYieldGenerationId::new("yield/conformance/b").expect("yield");
    second.request_commitment = expected_register_yield_commitment(&second);
    let left = fixture.backend.clone();
    let right = fixture.backend.clone();
    let (left_result, right_result) = std::thread::scope(|scope| {
        let left = scope.spawn(move || left.register_yield(first));
        let right = scope.spawn(move || right.register_yield(second));
        (
            left.join().expect("left yield"),
            right.join().expect("right yield"),
        )
    });
    let winners = [&left_result, &right_result]
        .into_iter()
        .filter(|result| matches!(result, Ok(RegisterYieldResult::Registered(_))))
        .count();
    assert_eq!(winners, 1);
    assert_eq!(
        usize::from(left_result.is_err()) + usize::from(right_result.is_err()),
        1
    );

    let fixture = ContinuityConformanceFixture::<B>::new(true, false);
    let mut exhausted = fixture.backend.conformance_snapshot();
    let window = exhausted
        .windows
        .get_mut(&fixture.window_id)
        .expect("window");
    window.maximum_attempts = 1;
    window.next_attempt_number = 2;
    let exhausted = fixture.reopen_with_state(exhausted);
    let error = match exhausted
        .backend
        .consume_directive(exhausted.consume_request(exhausted.authority_capability()))
    {
        Err(error) => error,
        Ok(_) => panic!("attempt budget exhaustion must fail"),
    };
    assert!(error.code().ends_with("attempt.budget_exhausted"));
}

pub(crate) fn scenario_wait_binding_and_fresh_authority<B: ContinuityConformanceBackend>() {
    let fixture = ContinuityConformanceFixture::<B>::new(true, true);
    let mut invalid_wake = fixture.wake_capability();
    invalid_wake.source_revision = 0;
    let invalid = fixture.wait_request(Some(&invalid_wake));
    let before = fixture.backend.conformance_snapshot();
    let error = match fixture.backend.transition_wait(invalid) {
        Err(error) => error,
        Ok(_) => panic!("zero-revision wake must fail"),
    };
    assert!(error.code().ends_with("wake.binding_mismatch"));
    assert!(fixture.backend.conformance_snapshot() == before);

    let wake = fixture.wake_capability();
    assert!(matches!(
        fixture
            .backend
            .transition_wait(fixture.wait_request(Some(&wake))),
        Ok(MutationResult::Recorded(_))
    ));
    let stale_authority = AuthorityUseCapability {
        window_revision: ContinuityRevision::new(1).expect("revision"),
        expected_waits: vec![ExpectedWaitRevision {
            condition_id: fixture.wait_id.clone().expect("wait"),
            condition_version: 1,
            revision: ContinuityRevision::new(1).expect("revision"),
        }],
        ..fixture.authority_capability()
    };
    let error = match fixture
        .backend
        .consume_directive(fixture.consume_request(stale_authority))
    {
        Err(error) => error,
        Ok(_) => panic!("stale pre-wake authority must fail"),
    };
    assert!(error.code().ends_with("authority.binding_mismatch"));
    assert!(matches!(
        fixture
            .backend
            .consume_directive(fixture.consume_request(fixture.authority_capability())),
        Ok(ConsumeDirectiveResult::Consumed { .. })
    ));
}

pub(crate) fn scenario_trusted_time_rejections_and_replay<B: ContinuityConformanceBackend>() {
    for (time_value, provenance, epoch) in [
        (
            Some("2026-08-15T11:59:59Z"),
            B::conformance_clock_provenance(),
            B::conformance_clock_epoch(),
        ),
        (
            None,
            SpecContentHash::from_text("wrong provenance"),
            B::conformance_clock_epoch(),
        ),
        (
            None,
            B::conformance_clock_provenance(),
            ContinuityTrustedTimeEpochId::new("epoch/conformance/wrong").expect("epoch"),
        ),
    ] {
        let fixture = ContinuityConformanceFixture::<B>::new(false, false);
        let capability = fixture.attempt_capability();
        let request = fixture.register_request(&capability);
        let replay = fixture.register_request(&capability);
        if let Some(value) = time_value {
            fixture.backend.conformance_set_time(time(value));
        }
        fixture.backend.conformance_set_time_provenance(provenance);
        fixture.backend.conformance_set_time_epoch(epoch);
        assert!(matches!(
            fixture.backend.register_yield(request),
            Ok(RegisterYieldResult::SecurityRejected(_))
        ));
        fixture.backend.conformance_set_time_available(false);
        assert!(matches!(
            fixture.backend.register_yield(replay),
            Ok(RegisterYieldResult::ExactReplay(_))
        ));
    }

    let fixture = ContinuityConformanceFixture::<B>::new(false, false);
    let capability = fixture.attempt_capability();
    let request = fixture.register_request(&capability);
    fixture
        .backend
        .conformance_set_time(time("2026-08-15T13:00:01Z"));
    assert!(matches!(
        fixture.backend.register_yield(request),
        Ok(RegisterYieldResult::SecurityRejected(_))
    ));

    let fixture = ContinuityConformanceFixture::<B>::new(false, false);
    let capability = fixture.attempt_capability();
    let request = fixture.register_request(&capability);
    let before = fixture.backend.conformance_snapshot();
    fixture.backend.conformance_set_time_available(false);
    assert!(fixture.backend.register_yield(request).is_err());
    assert!(fixture.backend.conformance_snapshot() == before);
}

pub(crate) fn scenario_restart_postures_and_reconciliation<B: ContinuityConformanceBackend>() {
    for outcome in [
        AuthorizedExecutionAttemptOutcome::Succeeded,
        AuthorizedExecutionAttemptOutcome::RetryableFailure,
        AuthorizedExecutionAttemptOutcome::TerminalFailure,
    ] {
        let fixture = ContinuityConformanceFixture::<B>::new(false, false);
        let capability = fixture.attempt_capability();
        assert!(fixture
            .backend
            .record_attempt_outcome(fixture.outcome_request(&capability, outcome))
            .is_ok());
        let reopened = fixture.reopen_current();
        assert!(reopened.backend.conformance_snapshot() == fixture.backend.conformance_snapshot());
    }

    let yielded = ContinuityConformanceFixture::<B>::new(true, false);
    assert!(
        yielded.reopen_current().backend.conformance_snapshot()
            == yielded.backend.conformance_snapshot()
    );
    let started = ContinuityConformanceFixture::<B>::new(false, false);
    assert!(
        started.reopen_current().backend.conformance_snapshot()
            == started.backend.conformance_snapshot()
    );
    assert!(matches!(
        started
            .reopen_current()
            .backend
            .recover_ambiguous_attempt(started.recovery_request()),
        Ok(MutationResult::Recorded(_))
    ));

    let fixture = ContinuityConformanceFixture::<B>::new(false, false);
    let absent = ReconcileOperationRequest {
        operation_id: ContinuityOperationId::new("operation/conformance-absent")
            .expect("operation"),
        expected_request_commitment: SpecContentHash::from_text("absent request"),
        expected_receipt_id: ContinuityReceiptId::new("receipt/conformance-absent")
            .expect("receipt"),
    };
    assert!(matches!(
        fixture.backend.reconcile_operation(&absent),
        ContinuityReconciliationResult::ConfirmedAbsent
    ));
    let reused_receipt = ReconcileOperationRequest {
        expected_receipt_id: ContinuityReceiptId::new("receipt/conformance-seed").expect("receipt"),
        ..absent
    };
    assert!(matches!(
        fixture.backend.reconcile_operation(&reused_receipt),
        ContinuityReconciliationResult::StateUnreadable
    ));
    let capability = fixture.attempt_capability();
    let committed_request = fixture.register_request(&capability);
    let operation_id = committed_request.operation_id.clone();
    let request_commitment = committed_request.request_commitment.clone();
    let receipt_id = committed_request.receipt_id.clone();
    assert!(matches!(
        fixture.backend.register_yield(committed_request),
        Ok(RegisterYieldResult::Registered(_))
    ));
    assert!(matches!(
        fixture
            .backend
            .reconcile_operation(&ReconcileOperationRequest {
                operation_id,
                expected_request_commitment: request_commitment,
                expected_receipt_id: receipt_id,
            }),
        ContinuityReconciliationResult::DurablyCommitted(_)
    ));
}

pub(crate) fn scenario_capability_burn_and_binding_changes<B: ContinuityConformanceBackend>() {
    let fixture = ContinuityConformanceFixture::<B>::new(true, false);
    let mut rolled_back = fixture.consume_request(fixture.authority_capability());
    let mut after_rollback = fixture.consume_request(fixture.authority_capability());
    after_rollback.operation_id =
        ContinuityOperationId::new("operation/conformance-consume-after-rollback")
            .expect("operation");
    after_rollback.receipt_id =
        ContinuityReceiptId::new("receipt/conformance-consume-after-rollback").expect("receipt");
    after_rollback.generated_attempt_id =
        AuthorizedExecutionAttemptId::new("attempt/conformance/after-rollback").expect("attempt");
    after_rollback.request_commitment = expected_consume_directive_commitment(&after_rollback);
    rolled_back.operation_id =
        ContinuityOperationId::new("operation/conformance-consume-rollback").expect("operation");
    rolled_back.receipt_id =
        ContinuityReceiptId::new("receipt/conformance-consume-rollback").expect("receipt");
    rolled_back.generated_attempt_id =
        AuthorizedExecutionAttemptId::new("attempt/conformance/rollback").expect("attempt");
    rolled_back.request_commitment = expected_consume_directive_commitment(&rolled_back);
    fixture
        .backend
        .conformance_inject_fault(ContinuityConformanceFault::Before);
    assert!(fixture.backend.consume_directive(rolled_back).is_err());
    assert!(matches!(
        fixture.backend.consume_directive(after_rollback),
        Ok(ConsumeDirectiveResult::Consumed { .. })
    ));

    let fixture = ContinuityConformanceFixture::<B>::new(true, false);
    let ambiguous = fixture.consume_request(fixture.authority_capability());
    let mut after_ambiguous = fixture.consume_request(fixture.authority_capability());
    after_ambiguous.operation_id =
        ContinuityOperationId::new("operation/conformance-consume-after-ambiguous")
            .expect("operation");
    after_ambiguous.receipt_id =
        ContinuityReceiptId::new("receipt/conformance-consume-after-ambiguous").expect("receipt");
    after_ambiguous.generated_attempt_id =
        AuthorizedExecutionAttemptId::new("attempt/conformance/after-ambiguous").expect("attempt");
    after_ambiguous.request_commitment = expected_consume_directive_commitment(&after_ambiguous);
    fixture
        .backend
        .conformance_inject_fault(ContinuityConformanceFault::After);
    assert!(fixture.backend.consume_directive(ambiguous).is_err());
    assert!(fixture.backend.consume_directive(after_ambiguous).is_err());

    let fixture = ContinuityConformanceFixture::<B>::new(true, false);
    let stale_request = fixture.consume_request(fixture.authority_capability());
    let mut changed = fixture.backend.conformance_snapshot();
    changed
        .windows
        .get_mut(&fixture.window_id)
        .expect("window")
        .authority_commitment = SpecContentHash::from_text("changed authority");
    let changed = fixture.reopen_with_state(changed);
    assert!(changed.backend.consume_directive(stale_request).is_err());

    let fixture = ContinuityConformanceFixture::<B>::new(false, false);
    let capability = fixture.attempt_capability();
    let stale_outcome =
        fixture.outcome_request(&capability, AuthorizedExecutionAttemptOutcome::Succeeded);
    let mut changed = fixture.backend.conformance_snapshot();
    changed
        .windows
        .get_mut(&fixture.window_id)
        .expect("window")
        .governance_commitment = SpecContentHash::from_text("changed governance");
    let changed = fixture.reopen_with_state(changed);
    assert!(changed
        .backend
        .record_attempt_outcome(stale_outcome)
        .is_err());
}

pub(crate) fn scenario_terminal_races_and_ambiguity_binding<B: ContinuityConformanceBackend>() {
    for state in [
        AuthoritativeWindowState::Closed,
        AuthoritativeWindowState::Expired,
        AuthoritativeWindowState::Revoked,
        AuthoritativeWindowState::Superseded,
    ] {
        let fixture = ContinuityConformanceFixture::<B>::new(false, false);
        let capability = fixture.attempt_capability();
        let request = fixture.register_request(&capability);
        let mut changed = fixture.backend.conformance_snapshot();
        changed
            .windows
            .get_mut(&fixture.window_id)
            .expect("window")
            .state = state;
        let changed = fixture.reopen_with_state(changed);
        let before = changed.backend.conformance_snapshot();
        assert!(changed.backend.register_yield(request).is_err());
        assert!(changed.backend.conformance_snapshot() == before);
    }

    let fixture = ContinuityConformanceFixture::<B>::new(true, true);
    let wake = fixture.wake_capability();
    let request = fixture.wait_request(Some(&wake));
    let mut canceled = fixture.backend.conformance_snapshot();
    canceled
        .waits
        .get_mut(&AuthoritativeWaitIdentity::new(
            fixture.wait_id.clone().expect("wait"),
            1,
        ))
        .expect("wait")
        .state = AuthoritativeWaitState::Canceled;
    let canceled = fixture.reopen_with_state(canceled);
    assert!(canceled.backend.transition_wait(request).is_err());

    let fixture = ContinuityConformanceFixture::<B>::new(false, false);
    let request = fixture.recovery_request();
    let mut changed = fixture.backend.conformance_snapshot();
    let window = changed.windows.get_mut(&fixture.window_id).expect("window");
    window.run_id = WorkflowRunId::new("run/conformance/changed").expect("run");
    window.cursor = ContinuityCursor {
        sequence_number: EventSequenceNumber::new(8).expect("sequence"),
        event_id: EventId::new("event/conformance/8").expect("event"),
    };
    let changed = fixture.reopen_with_state(changed);
    let before = changed.backend.conformance_snapshot();
    assert!(changed.backend.recover_ambiguous_attempt(request).is_err());
    assert!(changed.backend.conformance_snapshot() == before);
}

pub(crate) fn scenario_replay_survives_later_security_state<B: ContinuityConformanceBackend>() {
    let fixture = ContinuityConformanceFixture::<B>::new(false, false);
    let capability = fixture.attempt_capability();
    let request = fixture.register_request(&capability);
    let replay = fixture.register_request(&capability);
    assert!(matches!(
        fixture.backend.register_yield(request),
        Ok(RegisterYieldResult::Registered(_))
    ));
    let mut later = fixture.backend.conformance_snapshot();
    later.trusted_time.posture = TrustedTimePosture::Quarantined;
    later.trusted_time.eligibility = ContinuityInstanceEligibility::Quarantined;
    later.trusted_time.provenance_commitment = SpecContentHash::from_text("later provenance");
    later.trusted_time.epoch_id =
        ContinuityTrustedTimeEpochId::new("epoch/conformance/later").expect("epoch");
    let later = fixture.reopen_with_state(later);
    later.backend.conformance_set_time_available(false);
    assert!(matches!(
        later.backend.register_yield(replay),
        Ok(RegisterYieldResult::ExactReplay(_))
    ));
}

pub(crate) fn scenario_canonical_wait_ordering<B: ContinuityConformanceBackend>() {
    let fixture = ContinuityConformanceFixture::<B>::new(true, true);
    let mut ordered = fixture.authority_capability();
    ordered.expected_waits.push(ExpectedWaitRevision {
        condition_id: AuthorizedExecutionWaitConditionId::new("wait/conformance/2").expect("wait"),
        condition_version: 1,
        revision: ContinuityRevision::new(1).expect("revision"),
    });
    ordered.expected_waits.sort_by(|left, right| {
        left.condition_id
            .cmp(&right.condition_id)
            .then(left.condition_version.cmp(&right.condition_version))
    });
    let mut reversed_waits = ordered.expected_waits.clone();
    reversed_waits.reverse();
    let mut reversed = fixture.authority_capability();
    reversed.expected_waits = reversed_waits;
    let ordered = fixture.consume_request(ordered);
    let reversed = fixture.consume_request(reversed);
    assert_eq!(
        expected_consume_directive_commitment(&ordered),
        expected_consume_directive_commitment(&reversed)
    );
}

pub(crate) fn scenario_all_operation_commit_faults<B: ContinuityConformanceBackend>() {
    for fault in [
        ContinuityConformanceFault::Before,
        ContinuityConformanceFault::During,
        ContinuityConformanceFault::After,
    ] {
        fault_register::<B>(fault);
        fault_wait::<B>(fault);
        fault_consume::<B>(fault);
        fault_outcome::<B>(fault);
        fault_recovery::<B>(fault);
    }
}

fn assert_fault_result<B: ContinuityConformanceBackend>(
    operation: &str,
    backend: &B,
    before: &ReferenceContinuityState,
    fault: ContinuityConformanceFault,
    error_code: &str,
    operation_id: ContinuityOperationId,
    request_commitment: SpecContentHash,
    receipt_id: ContinuityReceiptId,
) {
    if fault == ContinuityConformanceFault::After {
        assert!(
            error_code.ends_with("commit_ambiguous") || error_code.ends_with("backend.unavailable"),
            "{operation} after-commit fault returned {error_code}"
        );
        assert_commit_ambiguity(backend, operation_id, request_commitment, receipt_id);
    } else {
        assert!(
            error_code.ends_with("write_failed") || error_code.ends_with("backend.unavailable")
        );
        assert!(backend.conformance_snapshot() == *before);
    }
}

fn fault_register<B: ContinuityConformanceBackend>(fault: ContinuityConformanceFault) {
    let fixture = ContinuityConformanceFixture::<B>::new(false, false);
    let capability = fixture.attempt_capability();
    let request = fixture.register_request(&capability);
    let operation_id = request.operation_id.clone();
    let request_commitment = request.request_commitment.clone();
    let receipt_id = request.receipt_id.clone();
    let before = fixture.backend.conformance_snapshot();
    fixture.backend.conformance_inject_fault(fault);
    let error = match fixture.backend.register_yield(request) {
        Err(error) => error,
        Ok(_) => panic!("fault must fail"),
    };
    assert_fault_result(
        "register_yield",
        &fixture.backend,
        &before,
        fault,
        error.code(),
        operation_id,
        request_commitment,
        receipt_id,
    );
}

fn fault_wait<B: ContinuityConformanceBackend>(fault: ContinuityConformanceFault) {
    let fixture = ContinuityConformanceFixture::<B>::new(true, true);
    let wake = fixture.wake_capability();
    let request = fixture.wait_request(Some(&wake));
    let operation_id = request.operation_id.clone();
    let request_commitment = request.request_commitment.clone();
    let receipt_id = request.receipt_id.clone();
    let before = fixture.backend.conformance_snapshot();
    fixture.backend.conformance_inject_fault(fault);
    let error = match fixture.backend.transition_wait(request) {
        Err(error) => error,
        Ok(_) => panic!("fault must fail"),
    };
    assert_fault_result(
        "transition_wait",
        &fixture.backend,
        &before,
        fault,
        error.code(),
        operation_id,
        request_commitment,
        receipt_id,
    );
}

fn fault_consume<B: ContinuityConformanceBackend>(fault: ContinuityConformanceFault) {
    let fixture = ContinuityConformanceFixture::<B>::new(true, false);
    let request = fixture.consume_request(fixture.authority_capability());
    let operation_id = request.operation_id.clone();
    let request_commitment = request.request_commitment.clone();
    let receipt_id = request.receipt_id.clone();
    let before = fixture.backend.conformance_snapshot();
    fixture.backend.conformance_inject_fault(fault);
    let error = match fixture.backend.consume_directive(request) {
        Err(error) => error,
        Ok(_) => panic!("fault must fail"),
    };
    assert_fault_result(
        "consume_directive",
        &fixture.backend,
        &before,
        fault,
        error.code(),
        operation_id,
        request_commitment,
        receipt_id,
    );
}

fn fault_outcome<B: ContinuityConformanceBackend>(fault: ContinuityConformanceFault) {
    let fixture = ContinuityConformanceFixture::<B>::new(false, false);
    let capability = fixture.attempt_capability();
    let request =
        fixture.outcome_request(&capability, AuthorizedExecutionAttemptOutcome::Succeeded);
    let operation_id = request.operation_id.clone();
    let request_commitment = request.request_commitment.clone();
    let receipt_id = request.receipt_id.clone();
    let before = fixture.backend.conformance_snapshot();
    fixture.backend.conformance_inject_fault(fault);
    let error = match fixture.backend.record_attempt_outcome(request) {
        Err(error) => error,
        Ok(_) => panic!("fault must fail"),
    };
    assert_fault_result(
        "record_attempt_outcome",
        &fixture.backend,
        &before,
        fault,
        error.code(),
        operation_id,
        request_commitment,
        receipt_id,
    );
}

fn fault_recovery<B: ContinuityConformanceBackend>(fault: ContinuityConformanceFault) {
    let fixture = ContinuityConformanceFixture::<B>::new(false, false);
    let request = fixture.recovery_request();
    let operation_id = request.operation_id.clone();
    let request_commitment = request.request_commitment.clone();
    let receipt_id = request.receipt_id.clone();
    let before = fixture.backend.conformance_snapshot();
    fixture.backend.conformance_inject_fault(fault);
    let error = match fixture.backend.recover_ambiguous_attempt(request) {
        Err(error) => error,
        Ok(_) => panic!("fault must fail"),
    };
    assert_fault_result(
        "recover_ambiguous_attempt",
        &fixture.backend,
        &before,
        fault,
        error.code(),
        operation_id,
        request_commitment,
        receipt_id,
    );
}

macro_rules! instantiate_continuity_conformance_tests {
    ($backend:ty) => {
        #[test]
        fn conformance_register_yield_replay_and_conflict() {
            $crate::authorized_execution_continuity_state::conformance::scenario_register_yield_replay_and_conflict::<$backend>();
        }

        #[test]
        fn conformance_transition_wait_replay() {
            $crate::authorized_execution_continuity_state::conformance::scenario_transition_wait_replay::<$backend>();
        }

        #[test]
        fn conformance_consume_directive_replay() {
            $crate::authorized_execution_continuity_state::conformance::scenario_consume_directive_replay::<$backend>();
        }

        #[test]
        fn conformance_attempt_outcomes() {
            $crate::authorized_execution_continuity_state::conformance::scenario_attempt_outcomes::<$backend>();
        }

        #[test]
        fn conformance_replay_conflicts_and_receipt_uniqueness() {
            $crate::authorized_execution_continuity_state::conformance::scenario_replay_conflicts_and_receipt_uniqueness::<$backend>();
        }

        #[test]
        fn conformance_concurrent_one_winner_and_attempt_budget() {
            $crate::authorized_execution_continuity_state::conformance::scenario_concurrent_one_winner_and_attempt_budget::<$backend>();
        }

        #[test]
        fn conformance_wait_binding_and_fresh_authority() {
            $crate::authorized_execution_continuity_state::conformance::scenario_wait_binding_and_fresh_authority::<$backend>();
        }

        #[test]
        fn conformance_trusted_time_rejections_and_replay() {
            $crate::authorized_execution_continuity_state::conformance::scenario_trusted_time_rejections_and_replay::<$backend>();
        }

        #[test]
        fn conformance_restart_postures_and_reconciliation() {
            $crate::authorized_execution_continuity_state::conformance::scenario_restart_postures_and_reconciliation::<$backend>();
        }

        #[test]
        fn conformance_capability_burn_and_binding_changes() {
            $crate::authorized_execution_continuity_state::conformance::scenario_capability_burn_and_binding_changes::<$backend>();
        }

        #[test]
        fn conformance_terminal_races_and_ambiguity_binding() {
            $crate::authorized_execution_continuity_state::conformance::scenario_terminal_races_and_ambiguity_binding::<$backend>();
        }

        #[test]
        fn conformance_replay_survives_later_security_state() {
            $crate::authorized_execution_continuity_state::conformance::scenario_replay_survives_later_security_state::<$backend>();
        }

        #[test]
        fn conformance_canonical_wait_ordering() {
            $crate::authorized_execution_continuity_state::conformance::scenario_canonical_wait_ordering::<$backend>();
        }

        #[test]
        fn conformance_all_operation_commit_faults() {
            $crate::authorized_execution_continuity_state::conformance::scenario_all_operation_commit_faults::<$backend>();
        }
    };
}

pub(crate) use instantiate_continuity_conformance_tests;
