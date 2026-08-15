use std::collections::BTreeSet;

use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    LocalStateBackend, PostgresStateBackend, SqliteStateBackend, WorkflowOsError,
    WorkflowOsErrorKind,
};

/// Version of the atomic authorized-execution continuity state contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorizedExecutionContinuityStateContractVersion {
    /// Initial contract and reference-conformance vocabulary.
    V1,
}

/// One atomic operation in the continuity state capability.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorizedExecutionContinuityOperationKind {
    /// Register one exact executor yield and optional typed waits.
    RegisterYield,
    /// Transition one exact typed wait from a verified wake source.
    TransitionWait,
    /// Consume one exact resume directive and durably start one attempt.
    ConsumeDirective,
    /// Record one ordinary terminal attempt outcome from the live consumer.
    RecordAttemptOutcome,
    /// Recover an orphaned started attempt as ambiguous.
    RecoverAmbiguousAttempt,
}

impl AuthorizedExecutionContinuityOperationKind {
    const ALL: [Self; 5] = [
        Self::RegisterYield,
        Self::TransitionWait,
        Self::ConsumeDirective,
        Self::RecordAttemptOutcome,
        Self::RecoverAmbiguousAttempt,
    ];

    /// Returns every V1 operation in stable order.
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &Self::ALL
    }
}

/// Declared support for one continuity operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorizedExecutionContinuityOperationSupport {
    /// The backend implements the operation and must pass dedicated conformance.
    Supported,
    /// The backend rejects the operation before changing state.
    Unsupported,
}

/// One explicit operation-support declaration.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AuthorizedExecutionContinuityOperationSupportEntry {
    kind: AuthorizedExecutionContinuityOperationKind,
    support: AuthorizedExecutionContinuityOperationSupport,
}

impl AuthorizedExecutionContinuityOperationSupportEntry {
    /// Creates one explicit support declaration.
    #[must_use]
    pub const fn new(
        kind: AuthorizedExecutionContinuityOperationKind,
        support: AuthorizedExecutionContinuityOperationSupport,
    ) -> Self {
        Self { kind, support }
    }

    /// Returns the operation kind.
    #[must_use]
    pub const fn kind(self) -> AuthorizedExecutionContinuityOperationKind {
        self.kind
    }

    /// Returns the declared support posture.
    #[must_use]
    pub const fn support(self) -> AuthorizedExecutionContinuityOperationSupport {
        self.support
    }
}

/// Validated backend declaration for the continuity-state capability.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AuthorizedExecutionContinuityStateContract {
    version: AuthorizedExecutionContinuityStateContractVersion,
    operations: Vec<AuthorizedExecutionContinuityOperationSupportEntry>,
}

impl AuthorizedExecutionContinuityStateContract {
    /// Creates a complete, duplicate-free operation declaration.
    ///
    /// # Errors
    ///
    /// Returns a stable error when an operation is missing or duplicated.
    pub fn new(
        version: AuthorizedExecutionContinuityStateContractVersion,
        operations: Vec<AuthorizedExecutionContinuityOperationSupportEntry>,
    ) -> Result<Self, WorkflowOsError> {
        let distinct = operations
            .iter()
            .map(|entry| entry.kind)
            .collect::<BTreeSet<_>>();
        if operations.len() != AuthorizedExecutionContinuityOperationKind::all().len()
            || distinct.len() != operations.len()
            || AuthorizedExecutionContinuityOperationKind::all()
                .iter()
                .any(|kind| !distinct.contains(kind))
        {
            return Err(state_error(
                WorkflowOsErrorKind::Validation,
                "contract.invalid",
                "authorized execution continuity state contract is invalid",
            ));
        }
        Ok(Self {
            version,
            operations,
        })
    }

    /// Returns the contract version.
    #[must_use]
    pub const fn version(&self) -> AuthorizedExecutionContinuityStateContractVersion {
        self.version
    }

    /// Returns support for one operation.
    #[must_use]
    pub fn support(
        &self,
        kind: AuthorizedExecutionContinuityOperationKind,
    ) -> AuthorizedExecutionContinuityOperationSupport {
        self.operations
            .iter()
            .find(|entry| entry.kind == kind)
            .map_or(
                AuthorizedExecutionContinuityOperationSupport::Unsupported,
                |entry| entry.support,
            )
    }

    /// Returns support declarations in stable contract order.
    #[must_use]
    pub fn operations(&self) -> &[AuthorizedExecutionContinuityOperationSupportEntry] {
        &self.operations
    }
}

#[derive(Deserialize)]
struct AuthorizedExecutionContinuityStateContractWire {
    version: AuthorizedExecutionContinuityStateContractVersion,
    operations: Vec<AuthorizedExecutionContinuityOperationSupportEntry>,
}

impl<'de> Deserialize<'de> for AuthorizedExecutionContinuityStateContract {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = AuthorizedExecutionContinuityStateContractWire::deserialize(deserializer)?;
        Self::new(wire.version, wire.operations).map_err(|_| {
            serde::de::Error::custom("authorized execution continuity state contract is invalid")
        })
    }
}

/// Backend declaration boundary for atomic continuity-state behavior.
pub trait AuthorizedExecutionContinuityStateContractProvider {
    /// Returns the backend's validated operation-support contract.
    ///
    /// # Errors
    ///
    /// Returns a stable error when the declaration is invalid.
    fn authorized_execution_continuity_state_contract(
        &self,
    ) -> Result<AuthorizedExecutionContinuityStateContract, WorkflowOsError>;
}

fn unsupported_support(
    kind: AuthorizedExecutionContinuityOperationKind,
) -> AuthorizedExecutionContinuityOperationSupport {
    match kind {
        AuthorizedExecutionContinuityOperationKind::RegisterYield
        | AuthorizedExecutionContinuityOperationKind::TransitionWait
        | AuthorizedExecutionContinuityOperationKind::ConsumeDirective
        | AuthorizedExecutionContinuityOperationKind::RecordAttemptOutcome
        | AuthorizedExecutionContinuityOperationKind::RecoverAmbiguousAttempt => {
            AuthorizedExecutionContinuityOperationSupport::Unsupported
        }
    }
}

fn unsupported_contract() -> Result<AuthorizedExecutionContinuityStateContract, WorkflowOsError> {
    AuthorizedExecutionContinuityStateContract::new(
        AuthorizedExecutionContinuityStateContractVersion::V1,
        AuthorizedExecutionContinuityOperationKind::all()
            .iter()
            .copied()
            .map(|kind| {
                AuthorizedExecutionContinuityOperationSupportEntry::new(
                    kind,
                    unsupported_support(kind),
                )
            })
            .collect(),
    )
}

impl AuthorizedExecutionContinuityStateContractProvider for LocalStateBackend {
    fn authorized_execution_continuity_state_contract(
        &self,
    ) -> Result<AuthorizedExecutionContinuityStateContract, WorkflowOsError> {
        unsupported_contract()
    }
}

impl AuthorizedExecutionContinuityStateContractProvider for SqliteStateBackend {
    fn authorized_execution_continuity_state_contract(
        &self,
    ) -> Result<AuthorizedExecutionContinuityStateContract, WorkflowOsError> {
        unsupported_contract()
    }
}

impl AuthorizedExecutionContinuityStateContractProvider for PostgresStateBackend {
    fn authorized_execution_continuity_state_contract(
        &self,
    ) -> Result<AuthorizedExecutionContinuityStateContract, WorkflowOsError> {
        unsupported_contract()
    }
}

fn state_error(
    kind: WorkflowOsErrorKind,
    suffix: &'static str,
    message: &'static str,
) -> WorkflowOsError {
    WorkflowOsError::new(
        kind,
        format!("authorized_execution_continuity_state.{suffix}"),
        message,
    )
}

#[allow(dead_code)]
mod internal {
    use std::collections::BTreeMap;
    use std::fmt;

    use sha2::{Digest, Sha256};

    use crate::{
        ActorId, AuthorizedExecutionAttemptId, AuthorizedExecutionAttemptOutcome,
        AuthorizedExecutionWaitConditionId, AuthorizedExecutionWakeTriggerKind,
        AuthorizedExecutionWindowId, AuthorizedExecutionYieldReason, EventId, EventSequenceNumber,
        ImmutableRunBundleBinding, SpecContentHash, StepId, Timestamp, WorkflowId, WorkflowOsError,
        WorkflowOsErrorKind, WorkflowRunId,
    };

    use super::AuthorizedExecutionContinuityOperationKind;

    const REFERENCE_MAX_BYTES: usize = 128;
    const MAX_WAITS: usize = 16;

    macro_rules! private_id {
        ($name:ident, $label:literal) => {
            #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
            pub(crate) struct $name(String);

            impl $name {
                pub(crate) fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
                    let value = value.into();
                    validate_reference($label, &value)?;
                    Ok(Self(value))
                }

                pub(crate) fn as_str(&self) -> &str {
                    &self.0
                }
            }

            impl fmt::Debug for $name {
                fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                    formatter
                        .debug_tuple(stringify!($name))
                        .field(&"[REDACTED]")
                        .finish()
                }
            }
        };
    }

    private_id!(ContinuityOperationId, "operation id");
    private_id!(ContinuityYieldGenerationId, "yield generation id");
    private_id!(ContinuityDirectiveId, "directive id");
    private_id!(ContinuityReceiptId, "receipt id");
    private_id!(ContinuityWakeSourceReference, "wake source reference");

    #[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub(crate) struct ContinuityRevision(u64);

    impl ContinuityRevision {
        pub(crate) fn new(value: u64) -> Result<Self, WorkflowOsError> {
            if value == 0 {
                return Err(continuity_state_error(
                    WorkflowOsErrorKind::Validation,
                    "input.invalid",
                    "authorized execution continuity input is invalid",
                ));
            }
            Ok(Self(value))
        }

        pub(crate) const fn get(self) -> u64 {
            self.0
        }

        pub(crate) const fn next(self) -> Self {
            Self(self.0 + 1)
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub(crate) struct ContinuityCursor {
        pub(crate) sequence_number: EventSequenceNumber,
        pub(crate) event_id: EventId,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub(crate) enum TrustedTimeSourceKind {
        CoreInjectedClockV1,
    }

    pub(super) struct TrustedTimeObservation {
        observed_at: Timestamp,
        source: TrustedTimeSourceKind,
        provenance_commitment: SpecContentHash,
    }

    impl fmt::Debug for TrustedTimeObservation {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter
                .debug_struct("TrustedTimeObservation")
                .field("source", &self.source)
                .field("binding", &"[REDACTED]")
                .finish_non_exhaustive()
        }
    }

    impl TrustedTimeObservation {
        pub(super) const fn observed_at(&self) -> Timestamp {
            self.observed_at
        }

        pub(super) const fn source(&self) -> TrustedTimeSourceKind {
            self.source
        }

        pub(super) fn provenance_commitment(&self) -> &SpecContentHash {
            &self.provenance_commitment
        }
    }

    pub(super) fn trusted_time_observation(
        observed_at: Timestamp,
        source: TrustedTimeSourceKind,
        provenance_commitment: SpecContentHash,
    ) -> TrustedTimeObservation {
        TrustedTimeObservation {
            observed_at,
            source,
            provenance_commitment,
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub(crate) enum AuthoritativeWindowState {
        AssessmentRequired,
        Executing,
        Yielded,
        Closed,
        RecoveryRequired,
        Expired,
        Revoked,
        Superseded,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub(crate) enum AuthoritativeAttemptState {
        Started,
        Yielded,
        Succeeded,
        RetryableFailure,
        TerminalFailure,
        AmbiguousMayHaveStarted,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub(crate) enum AuthoritativeWaitState {
        Unsatisfied,
        Satisfied,
        Expired,
        Superseded,
        Canceled,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub(crate) enum AuthoritativeDirectiveState {
        Available,
        Consumed,
        Invalidated,
        Expired,
    }

    #[derive(Clone, Eq, PartialEq)]
    pub(crate) struct AuthoritativeWindowRecord {
        pub(crate) workflow_id: WorkflowId,
        pub(crate) run_id: WorkflowRunId,
        pub(crate) step_id: StepId,
        pub(crate) window_id: AuthorizedExecutionWindowId,
        pub(crate) subject_actor_id: ActorId,
        pub(crate) immutable_run_bundle: ImmutableRunBundleBinding,
        pub(crate) governance_commitment: SpecContentHash,
        pub(crate) authority_commitment: SpecContentHash,
        pub(crate) cursor: ContinuityCursor,
        pub(crate) state: AuthoritativeWindowState,
        pub(crate) maximum_attempts: u32,
        pub(crate) next_attempt_number: u32,
        pub(crate) expires_at: Timestamp,
        pub(crate) trusted_time_watermark: Timestamp,
        pub(crate) revision: ContinuityRevision,
        pub(crate) active_yield: Option<ContinuityYieldGenerationId>,
    }

    impl fmt::Debug for AuthoritativeWindowRecord {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter
                .debug_struct("AuthoritativeWindowRecord")
                .field("state", &self.state)
                .field("maximum_attempts", &self.maximum_attempts)
                .field("next_attempt_number", &self.next_attempt_number)
                .field("revision", &self.revision)
                .field("binding", &"[REDACTED]")
                .finish_non_exhaustive()
        }
    }

    #[derive(Clone, Eq, PartialEq)]
    pub(crate) struct AuthoritativeAttemptRecord {
        pub(crate) attempt_id: AuthorizedExecutionAttemptId,
        pub(crate) attempt_number: u32,
        pub(crate) window_id: AuthorizedExecutionWindowId,
        pub(crate) subject_actor_id: ActorId,
        pub(crate) cursor: ContinuityCursor,
        pub(crate) authority_commitment: SpecContentHash,
        pub(crate) consume_operation_id: ContinuityOperationId,
        pub(crate) state: AuthoritativeAttemptState,
        pub(crate) revision: ContinuityRevision,
    }

    impl fmt::Debug for AuthoritativeAttemptRecord {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter
                .debug_struct("AuthoritativeAttemptRecord")
                .field("attempt_number", &self.attempt_number)
                .field("state", &self.state)
                .field("revision", &self.revision)
                .field("binding", &"[REDACTED]")
                .finish_non_exhaustive()
        }
    }

    #[derive(Clone, Eq, PartialEq)]
    pub(crate) struct AuthoritativeYieldRecord {
        pub(crate) generation_id: ContinuityYieldGenerationId,
        pub(crate) attempt_id: AuthorizedExecutionAttemptId,
        pub(crate) cursor: ContinuityCursor,
        pub(crate) reason: AuthorizedExecutionYieldReason,
        pub(crate) wait_ids: Vec<AuthorizedExecutionWaitConditionId>,
        pub(crate) registered_at: Timestamp,
    }

    #[derive(Clone, Eq, PartialEq)]
    pub(crate) struct AuthoritativeWaitRecord {
        pub(crate) condition_id: AuthorizedExecutionWaitConditionId,
        pub(crate) condition_version: u32,
        pub(crate) window_id: AuthorizedExecutionWindowId,
        pub(crate) generation_id: ContinuityYieldGenerationId,
        pub(crate) wake_trigger: AuthorizedExecutionWakeTriggerKind,
        pub(crate) state: AuthoritativeWaitState,
        pub(crate) source_commitment: Option<SpecContentHash>,
        pub(crate) source_revision: Option<u64>,
        pub(crate) revision: ContinuityRevision,
    }

    #[derive(Clone, Eq, PartialEq)]
    pub(crate) struct AuthoritativeDirectiveRecord {
        pub(crate) directive_id: ContinuityDirectiveId,
        pub(crate) window_id: AuthorizedExecutionWindowId,
        pub(crate) generation_id: ContinuityYieldGenerationId,
        pub(crate) cursor: ContinuityCursor,
        pub(crate) authority_commitment: SpecContentHash,
        pub(crate) state: AuthoritativeDirectiveState,
        pub(crate) revision: ContinuityRevision,
    }

    #[derive(Clone, Eq, PartialEq)]
    pub(crate) struct ContinuityReceipt {
        pub(crate) receipt_id: ContinuityReceiptId,
        pub(crate) operation_kind: AuthorizedExecutionContinuityOperationKind,
        pub(crate) operation_commitment: SpecContentHash,
        pub(crate) trusted_time_commitment: SpecContentHash,
        pub(crate) committed_at: Timestamp,
    }

    impl fmt::Debug for ContinuityReceipt {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter
                .debug_struct("ContinuityReceipt")
                .field("operation_kind", &self.operation_kind)
                .field("binding", &"[REDACTED]")
                .finish_non_exhaustive()
        }
    }

    #[derive(Clone, Eq, PartialEq)]
    pub(crate) enum RecordedOperationResult {
        YieldRegistered {
            generation_id: ContinuityYieldGenerationId,
            window_revision: ContinuityRevision,
        },
        WaitTransitioned {
            wait_revision: ContinuityRevision,
            window_revision: ContinuityRevision,
        },
        DirectiveConsumed {
            attempt_id: AuthorizedExecutionAttemptId,
            attempt_number: u32,
            window_revision: ContinuityRevision,
        },
        AttemptOutcomeRecorded {
            attempt_state: AuthoritativeAttemptState,
            window_state: AuthoritativeWindowState,
            window_revision: ContinuityRevision,
        },
    }

    #[derive(Clone, Eq, PartialEq)]
    pub(crate) struct AuthoritativeOperationRecord {
        pub(crate) operation_id: ContinuityOperationId,
        pub(crate) operation_kind: AuthorizedExecutionContinuityOperationKind,
        pub(crate) request_commitment: SpecContentHash,
        pub(crate) operation_commitment: SpecContentHash,
        pub(crate) receipt: ContinuityReceipt,
        pub(crate) result: RecordedOperationResult,
    }

    #[derive(Clone, Eq, PartialEq)]
    pub(crate) struct ExpectedWindowBinding {
        pub(crate) workflow_id: WorkflowId,
        pub(crate) run_id: WorkflowRunId,
        pub(crate) step_id: StepId,
        pub(crate) subject_actor_id: ActorId,
        pub(crate) immutable_run_bundle: ImmutableRunBundleBinding,
        pub(crate) governance_commitment: SpecContentHash,
        pub(crate) authority_commitment: SpecContentHash,
        pub(crate) cursor: ContinuityCursor,
    }

    pub(crate) struct AuthorityUseCapability {
        pub(super) window_id: AuthorizedExecutionWindowId,
        pub(super) window_revision: ContinuityRevision,
        pub(super) generation_id: ContinuityYieldGenerationId,
        pub(super) cursor: ContinuityCursor,
        pub(super) subject_actor_id: ActorId,
        pub(super) authority_commitment: SpecContentHash,
        pub(super) window_binding_commitment: SpecContentHash,
        pub(super) expected_waits: Vec<ExpectedWaitRevision>,
    }

    impl fmt::Debug for AuthorityUseCapability {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter
                .debug_struct("AuthorityUseCapability")
                .field("binding", &"[REDACTED]")
                .finish_non_exhaustive()
        }
    }

    pub(crate) struct WakeAssessmentCapability {
        pub(super) window_id: AuthorizedExecutionWindowId,
        pub(super) generation_id: ContinuityYieldGenerationId,
        pub(super) condition_id: AuthorizedExecutionWaitConditionId,
        pub(super) condition_version: u32,
        pub(super) trigger: AuthorizedExecutionWakeTriggerKind,
        pub(super) source_reference: ContinuityWakeSourceReference,
        pub(super) source_commitment: SpecContentHash,
        pub(super) source_revision: u64,
    }

    impl fmt::Debug for WakeAssessmentCapability {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter
                .debug_struct("WakeAssessmentCapability")
                .field("binding", &"[REDACTED]")
                .finish_non_exhaustive()
        }
    }

    pub(crate) struct AttemptUseCapability {
        pub(super) attempt_id: AuthorizedExecutionAttemptId,
        pub(super) subject_actor_id: ActorId,
        pub(super) window_id: AuthorizedExecutionWindowId,
        pub(super) window_revision: ContinuityRevision,
        pub(super) cursor: ContinuityCursor,
        pub(super) authority_commitment: SpecContentHash,
        pub(super) window_binding_commitment: SpecContentHash,
        pub(super) consume_operation_id: ContinuityOperationId,
    }

    impl fmt::Debug for AttemptUseCapability {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter
                .debug_struct("AttemptUseCapability")
                .field("binding", &"[REDACTED]")
                .finish_non_exhaustive()
        }
    }

    pub(crate) struct RegisterYieldRequest<'a> {
        pub(crate) operation_id: ContinuityOperationId,
        pub(crate) request_commitment: SpecContentHash,
        pub(crate) receipt_id: ContinuityReceiptId,
        pub(crate) generation_id: ContinuityYieldGenerationId,
        pub(crate) window_id: AuthorizedExecutionWindowId,
        pub(crate) expected_window_revision: ContinuityRevision,
        pub(crate) expected_window_binding: ExpectedWindowBinding,
        pub(crate) cursor: ContinuityCursor,
        pub(crate) attempt_id: AuthorizedExecutionAttemptId,
        pub(crate) attempt_capability: &'a AttemptUseCapability,
        pub(crate) reason: AuthorizedExecutionYieldReason,
        pub(crate) waits: Vec<SeedWait>,
    }

    pub(crate) struct TransitionWaitRequest<'a> {
        pub(crate) operation_id: ContinuityOperationId,
        pub(crate) request_commitment: SpecContentHash,
        pub(crate) receipt_id: ContinuityReceiptId,
        pub(crate) window_id: AuthorizedExecutionWindowId,
        pub(crate) expected_window_revision: ContinuityRevision,
        pub(crate) expected_window_binding: ExpectedWindowBinding,
        pub(crate) cursor: ContinuityCursor,
        pub(crate) condition_id: AuthorizedExecutionWaitConditionId,
        pub(crate) expected_generation_id: ContinuityYieldGenerationId,
        pub(crate) expected_condition_version: u32,
        pub(crate) expected_wait_revision: ContinuityRevision,
        pub(crate) target: AuthoritativeWaitState,
        pub(crate) wake_capability: Option<&'a WakeAssessmentCapability>,
    }

    pub(crate) struct ConsumeDirectiveRequest<'a> {
        pub(crate) operation_id: ContinuityOperationId,
        pub(crate) request_commitment: SpecContentHash,
        pub(crate) receipt_id: ContinuityReceiptId,
        pub(crate) directive_id: ContinuityDirectiveId,
        pub(crate) window_id: AuthorizedExecutionWindowId,
        pub(crate) expected_window_revision: ContinuityRevision,
        pub(crate) expected_window_binding: ExpectedWindowBinding,
        pub(crate) generation_id: ContinuityYieldGenerationId,
        pub(crate) cursor: ContinuityCursor,
        pub(crate) expected_waits: Vec<ExpectedWaitRevision>,
        pub(crate) authority_capability: &'a AuthorityUseCapability,
        pub(crate) generated_attempt_id: AuthorizedExecutionAttemptId,
    }

    pub(crate) struct RecordAttemptOutcomeRequest<'a> {
        pub(crate) operation_id: ContinuityOperationId,
        pub(crate) request_commitment: SpecContentHash,
        pub(crate) receipt_id: ContinuityReceiptId,
        pub(crate) window_id: AuthorizedExecutionWindowId,
        pub(crate) expected_window_revision: ContinuityRevision,
        pub(crate) expected_window_binding: ExpectedWindowBinding,
        pub(crate) attempt_id: AuthorizedExecutionAttemptId,
        pub(crate) expected_attempt_revision: ContinuityRevision,
        pub(crate) attempt_capability: &'a AttemptUseCapability,
        pub(crate) outcome: AuthorizedExecutionAttemptOutcome,
    }

    pub(crate) struct RecoverAmbiguousAttemptRequest {
        pub(crate) operation_id: ContinuityOperationId,
        pub(crate) request_commitment: SpecContentHash,
        pub(crate) receipt_id: ContinuityReceiptId,
        pub(crate) window_id: AuthorizedExecutionWindowId,
        pub(crate) expected_window_revision: ContinuityRevision,
        pub(crate) expected_window_binding: ExpectedWindowBinding,
        pub(crate) cursor: ContinuityCursor,
        pub(crate) attempt_id: AuthorizedExecutionAttemptId,
        pub(crate) expected_attempt_revision: ContinuityRevision,
    }

    #[derive(Clone, Eq, PartialEq)]
    pub(crate) struct SeedWait {
        pub(crate) condition_id: AuthorizedExecutionWaitConditionId,
        pub(crate) condition_version: u32,
        pub(crate) wake_trigger: AuthorizedExecutionWakeTriggerKind,
    }

    #[derive(Clone, Eq, PartialEq)]
    pub(crate) struct ExpectedWaitRevision {
        pub(crate) condition_id: AuthorizedExecutionWaitConditionId,
        pub(crate) condition_version: u32,
        pub(crate) revision: ContinuityRevision,
    }

    #[derive(Clone, Eq, PartialEq)]
    pub(crate) enum RegisterYieldResult {
        Registered(RecordedOperationResult),
        ExactReplay(RecordedOperationResult),
    }

    pub(crate) enum ConsumeDirectiveResult {
        Consumed {
            result: RecordedOperationResult,
            capability: AttemptUseCapability,
        },
        ExactReplay(RecordedOperationResult),
    }

    #[derive(Clone, Eq, PartialEq)]
    pub(crate) enum MutationResult {
        Recorded(RecordedOperationResult),
        ExactReplay(RecordedOperationResult),
    }

    pub(crate) trait AuthorizedExecutionContinuityStore {
        fn register_yield(
            &self,
            request: RegisterYieldRequest<'_>,
        ) -> Result<RegisterYieldResult, WorkflowOsError>;

        fn transition_wait(
            &self,
            request: TransitionWaitRequest<'_>,
        ) -> Result<MutationResult, WorkflowOsError>;

        fn consume_directive(
            &self,
            request: ConsumeDirectiveRequest<'_>,
        ) -> Result<ConsumeDirectiveResult, WorkflowOsError>;

        fn record_attempt_outcome(
            &self,
            request: RecordAttemptOutcomeRequest<'_>,
        ) -> Result<MutationResult, WorkflowOsError>;

        fn recover_ambiguous_attempt(
            &self,
            request: RecoverAmbiguousAttemptRequest,
        ) -> Result<MutationResult, WorkflowOsError>;
    }

    #[derive(Clone, Eq, PartialEq)]
    pub(crate) struct ReferenceContinuityState {
        pub(crate) windows: BTreeMap<AuthorizedExecutionWindowId, AuthoritativeWindowRecord>,
        pub(crate) yields: BTreeMap<ContinuityYieldGenerationId, AuthoritativeYieldRecord>,
        pub(crate) waits: BTreeMap<AuthorizedExecutionWaitConditionId, AuthoritativeWaitRecord>,
        pub(crate) directives: BTreeMap<ContinuityDirectiveId, AuthoritativeDirectiveRecord>,
        pub(crate) attempts: BTreeMap<AuthorizedExecutionAttemptId, AuthoritativeAttemptRecord>,
        pub(crate) operations: BTreeMap<ContinuityOperationId, AuthoritativeOperationRecord>,
    }

    pub(crate) fn request_commitment(
        domain: &'static str,
        operation_id: &ContinuityOperationId,
        fields: &[&str],
    ) -> SpecContentHash {
        let mut hasher = Sha256::new();
        frame(&mut hasher, "version", "v1");
        frame(&mut hasher, "domain", domain);
        frame(&mut hasher, "operation_id", operation_id.as_str());
        for (index, field) in fields.iter().enumerate() {
            frame(&mut hasher, &format!("field-{index}"), field);
        }
        SpecContentHash::from_bytes(hasher.finalize())
    }

    pub(crate) fn window_binding_commitment(binding: &ExpectedWindowBinding) -> SpecContentHash {
        let mut hasher = Sha256::new();
        frame(&mut hasher, "version", "v1");
        frame(
            &mut hasher,
            "domain",
            "workflow-os/authorized-execution-continuity/window-binding/v1",
        );
        frame(&mut hasher, "workflow_id", binding.workflow_id.as_str());
        frame(&mut hasher, "run_id", binding.run_id.as_str());
        frame(&mut hasher, "step_id", binding.step_id.as_str());
        frame(
            &mut hasher,
            "subject_actor_id",
            binding.subject_actor_id.as_str(),
        );
        frame(
            &mut hasher,
            "bundle_id",
            binding.immutable_run_bundle.bundle_id().as_str(),
        );
        frame(
            &mut hasher,
            "bundle_version",
            binding.immutable_run_bundle.bundle_version().as_str(),
        );
        frame(
            &mut hasher,
            "bundle_root",
            binding.immutable_run_bundle.root_hash().as_str(),
        );
        frame(
            &mut hasher,
            "governance_commitment",
            binding.governance_commitment.as_str(),
        );
        frame(
            &mut hasher,
            "authority_commitment",
            binding.authority_commitment.as_str(),
        );
        frame(
            &mut hasher,
            "cursor_sequence",
            &binding.cursor.sequence_number.get().to_string(),
        );
        frame(
            &mut hasher,
            "cursor_event_id",
            binding.cursor.event_id.as_str(),
        );
        SpecContentHash::from_bytes(hasher.finalize())
    }

    pub(crate) fn wait_revisions_commitment(waits: &[ExpectedWaitRevision]) -> SpecContentHash {
        let mut canonical = waits.to_vec();
        canonical.sort_by(|left, right| left.condition_id.cmp(&right.condition_id));
        let mut hasher = Sha256::new();
        frame(&mut hasher, "version", "v1");
        frame(
            &mut hasher,
            "domain",
            "workflow-os/authorized-execution-continuity/wait-revisions/v1",
        );
        frame(&mut hasher, "count", &canonical.len().to_string());
        for (index, wait) in canonical.iter().enumerate() {
            frame(
                &mut hasher,
                &format!("condition-{index}"),
                wait.condition_id.as_str(),
            );
            frame(
                &mut hasher,
                &format!("version-{index}"),
                &wait.condition_version.to_string(),
            );
            frame(
                &mut hasher,
                &format!("revision-{index}"),
                &wait.revision.get().to_string(),
            );
        }
        SpecContentHash::from_bytes(hasher.finalize())
    }

    pub(super) fn trusted_time_commitment(observation: &TrustedTimeObservation) -> SpecContentHash {
        let mut hasher = Sha256::new();
        frame(&mut hasher, "version", "v1");
        frame(
            &mut hasher,
            "domain",
            "workflow-os/authorized-execution-continuity/trusted-time/v1",
        );
        frame(
            &mut hasher,
            "source",
            match observation.source {
                TrustedTimeSourceKind::CoreInjectedClockV1 => "core_injected_clock_v1",
            },
        );
        frame(
            &mut hasher,
            "provenance",
            observation.provenance_commitment.as_str(),
        );
        frame(
            &mut hasher,
            "observed_at",
            &observation.observed_at.to_rfc3339(),
        );
        SpecContentHash::from_bytes(hasher.finalize())
    }

    pub(crate) fn expected_register_yield_commitment(
        request: &RegisterYieldRequest<'_>,
    ) -> SpecContentHash {
        let mut fields = vec![
            request.receipt_id.as_str().to_owned(),
            request.window_id.as_str().to_owned(),
            request.expected_window_revision.get().to_string(),
            window_binding_commitment(&request.expected_window_binding)
                .as_str()
                .to_owned(),
            request.cursor.sequence_number.get().to_string(),
            request.cursor.event_id.as_str().to_owned(),
            request.attempt_id.as_str().to_owned(),
            request
                .attempt_capability
                .consume_operation_id
                .as_str()
                .to_owned(),
            request.attempt_capability.window_id.as_str().to_owned(),
            request.attempt_capability.window_revision.get().to_string(),
            request
                .attempt_capability
                .cursor
                .sequence_number
                .get()
                .to_string(),
            request
                .attempt_capability
                .cursor
                .event_id
                .as_str()
                .to_owned(),
            request
                .attempt_capability
                .subject_actor_id
                .as_str()
                .to_owned(),
            request
                .attempt_capability
                .authority_commitment
                .as_str()
                .to_owned(),
            request
                .attempt_capability
                .window_binding_commitment
                .as_str()
                .to_owned(),
            request.generation_id.as_str().to_owned(),
            yield_reason_code(request.reason).to_owned(),
            request.waits.len().to_string(),
        ];
        let mut waits = request.waits.clone();
        waits.sort_by(|left, right| {
            left.condition_id
                .cmp(&right.condition_id)
                .then(left.condition_version.cmp(&right.condition_version))
        });
        for wait in waits {
            fields.push(wait.condition_id.as_str().to_owned());
            fields.push(wait.condition_version.to_string());
            fields.push(wake_trigger_code(wait.wake_trigger).to_owned());
        }
        request_commitment(
            "workflow-os/authorized-execution-continuity/register-yield/v1",
            &request.operation_id,
            &fields.iter().map(String::as_str).collect::<Vec<_>>(),
        )
    }

    pub(crate) fn expected_transition_wait_commitment(
        request: &TransitionWaitRequest<'_>,
    ) -> SpecContentHash {
        let wake_fields = request.wake_capability.map_or_else(
            || vec!["none".to_owned()],
            |capability| {
                vec![
                    capability.window_id.as_str().to_owned(),
                    capability.generation_id.as_str().to_owned(),
                    capability.condition_id.as_str().to_owned(),
                    capability.condition_version.to_string(),
                    wake_trigger_code(capability.trigger).to_owned(),
                    capability.source_reference.as_str().to_owned(),
                    capability.source_commitment.as_str().to_owned(),
                    capability.source_revision.to_string(),
                ]
            },
        );
        let mut fields = vec![
            request.receipt_id.as_str().to_owned(),
            request.window_id.as_str().to_owned(),
            request.expected_window_revision.get().to_string(),
            window_binding_commitment(&request.expected_window_binding)
                .as_str()
                .to_owned(),
            request.cursor.sequence_number.get().to_string(),
            request.cursor.event_id.as_str().to_owned(),
            request.condition_id.as_str().to_owned(),
            request.expected_generation_id.as_str().to_owned(),
            request.expected_condition_version.to_string(),
            request.expected_wait_revision.get().to_string(),
            wait_state_code(request.target).to_owned(),
        ];
        fields.extend(wake_fields);
        request_commitment(
            "workflow-os/authorized-execution-continuity/transition-wait/v1",
            &request.operation_id,
            &fields.iter().map(String::as_str).collect::<Vec<_>>(),
        )
    }

    pub(crate) fn expected_consume_directive_commitment(
        request: &ConsumeDirectiveRequest<'_>,
    ) -> SpecContentHash {
        let mut expected_waits = request.expected_waits.clone();
        expected_waits.sort_by(|left, right| left.condition_id.cmp(&right.condition_id));
        let mut fields = vec![
            request.receipt_id.as_str().to_owned(),
            request.directive_id.as_str().to_owned(),
            request.window_id.as_str().to_owned(),
            request.expected_window_revision.get().to_string(),
            window_binding_commitment(&request.expected_window_binding)
                .as_str()
                .to_owned(),
            request.generation_id.as_str().to_owned(),
            request.cursor.sequence_number.get().to_string(),
            request.cursor.event_id.as_str().to_owned(),
            request.authority_capability.window_id.as_str().to_owned(),
            request
                .authority_capability
                .window_revision
                .get()
                .to_string(),
            request
                .authority_capability
                .generation_id
                .as_str()
                .to_owned(),
            request
                .authority_capability
                .cursor
                .sequence_number
                .get()
                .to_string(),
            request
                .authority_capability
                .cursor
                .event_id
                .as_str()
                .to_owned(),
            request
                .authority_capability
                .subject_actor_id
                .as_str()
                .to_owned(),
            request
                .authority_capability
                .authority_commitment
                .as_str()
                .to_owned(),
            request
                .authority_capability
                .window_binding_commitment
                .as_str()
                .to_owned(),
            wait_revisions_commitment(&request.authority_capability.expected_waits)
                .as_str()
                .to_owned(),
            request.generated_attempt_id.as_str().to_owned(),
            expected_waits.len().to_string(),
        ];
        for wait in expected_waits {
            fields.push(wait.condition_id.as_str().to_owned());
            fields.push(wait.condition_version.to_string());
            fields.push(wait.revision.get().to_string());
        }
        request_commitment(
            "workflow-os/authorized-execution-continuity/consume-directive/v1",
            &request.operation_id,
            &fields.iter().map(String::as_str).collect::<Vec<_>>(),
        )
    }

    pub(crate) fn expected_attempt_outcome_commitment(
        request: &RecordAttemptOutcomeRequest<'_>,
    ) -> SpecContentHash {
        request_commitment(
            "workflow-os/authorized-execution-continuity/attempt-outcome/v1",
            &request.operation_id,
            &[
                request.receipt_id.as_str(),
                request.window_id.as_str(),
                &request.expected_window_revision.get().to_string(),
                window_binding_commitment(&request.expected_window_binding).as_str(),
                request.attempt_id.as_str(),
                &request.expected_attempt_revision.get().to_string(),
                request.attempt_capability.attempt_id.as_str(),
                request.attempt_capability.window_id.as_str(),
                &request.attempt_capability.window_revision.get().to_string(),
                &request
                    .attempt_capability
                    .cursor
                    .sequence_number
                    .get()
                    .to_string(),
                request.attempt_capability.cursor.event_id.as_str(),
                request.attempt_capability.subject_actor_id.as_str(),
                request.attempt_capability.authority_commitment.as_str(),
                request
                    .attempt_capability
                    .window_binding_commitment
                    .as_str(),
                request.attempt_capability.consume_operation_id.as_str(),
                attempt_outcome_code(request.outcome),
            ],
        )
    }

    pub(crate) fn expected_recovery_commitment(
        request: &RecoverAmbiguousAttemptRequest,
    ) -> SpecContentHash {
        request_commitment(
            "workflow-os/authorized-execution-continuity/recover-ambiguous/v1",
            &request.operation_id,
            &[
                request.receipt_id.as_str(),
                request.window_id.as_str(),
                &request.expected_window_revision.get().to_string(),
                window_binding_commitment(&request.expected_window_binding).as_str(),
                &request.cursor.sequence_number.get().to_string(),
                request.cursor.event_id.as_str(),
                request.attempt_id.as_str(),
                &request.expected_attempt_revision.get().to_string(),
            ],
        )
    }

    pub(crate) fn operation_commitment(
        request: &SpecContentHash,
        receipt_id: &ContinuityReceiptId,
        observed_at: Timestamp,
        trusted_time: &SpecContentHash,
        result: &RecordedOperationResult,
    ) -> SpecContentHash {
        let mut hasher = Sha256::new();
        frame(&mut hasher, "version", "v1");
        frame(
            &mut hasher,
            "domain",
            "workflow-os/authorized-execution-continuity/committed-operation/v1",
        );
        frame(&mut hasher, "request", request.as_str());
        frame(&mut hasher, "receipt", receipt_id.as_str());
        frame(&mut hasher, "observed_at", &observed_at.to_rfc3339());
        frame(&mut hasher, "trusted_time", trusted_time.as_str());
        frame(
            &mut hasher,
            "result_commitment",
            result_commitment(result).as_str(),
        );
        SpecContentHash::from_bytes(hasher.finalize())
    }

    fn result_commitment(result: &RecordedOperationResult) -> SpecContentHash {
        let mut hasher = Sha256::new();
        frame(&mut hasher, "version", "v1");
        frame(
            &mut hasher,
            "domain",
            "workflow-os/authorized-execution-continuity/operation-result/v1",
        );
        match result {
            RecordedOperationResult::YieldRegistered {
                generation_id,
                window_revision,
            } => {
                frame(&mut hasher, "kind", "yield_registered");
                frame(&mut hasher, "generation_id", generation_id.as_str());
                frame(
                    &mut hasher,
                    "window_revision",
                    &window_revision.get().to_string(),
                );
            }
            RecordedOperationResult::WaitTransitioned {
                wait_revision,
                window_revision,
            } => {
                frame(&mut hasher, "kind", "wait_transitioned");
                frame(
                    &mut hasher,
                    "wait_revision",
                    &wait_revision.get().to_string(),
                );
                frame(
                    &mut hasher,
                    "window_revision",
                    &window_revision.get().to_string(),
                );
            }
            RecordedOperationResult::DirectiveConsumed {
                attempt_id,
                attempt_number,
                window_revision,
            } => {
                frame(&mut hasher, "kind", "directive_consumed");
                frame(&mut hasher, "attempt_id", attempt_id.as_str());
                frame(&mut hasher, "attempt_number", &attempt_number.to_string());
                frame(
                    &mut hasher,
                    "window_revision",
                    &window_revision.get().to_string(),
                );
            }
            RecordedOperationResult::AttemptOutcomeRecorded {
                attempt_state,
                window_state,
                window_revision,
            } => {
                frame(&mut hasher, "kind", "attempt_outcome_recorded");
                frame(
                    &mut hasher,
                    "attempt_state",
                    authoritative_attempt_state_code(*attempt_state),
                );
                frame(
                    &mut hasher,
                    "window_state",
                    authoritative_window_state_code(*window_state),
                );
                frame(
                    &mut hasher,
                    "window_revision",
                    &window_revision.get().to_string(),
                );
            }
        }
        SpecContentHash::from_bytes(hasher.finalize())
    }

    fn authoritative_attempt_state_code(value: AuthoritativeAttemptState) -> &'static str {
        match value {
            AuthoritativeAttemptState::Started => "started",
            AuthoritativeAttemptState::Yielded => "yielded",
            AuthoritativeAttemptState::Succeeded => "succeeded",
            AuthoritativeAttemptState::RetryableFailure => "retryable_failure",
            AuthoritativeAttemptState::TerminalFailure => "terminal_failure",
            AuthoritativeAttemptState::AmbiguousMayHaveStarted => "ambiguous_may_have_started",
        }
    }

    fn authoritative_window_state_code(value: AuthoritativeWindowState) -> &'static str {
        match value {
            AuthoritativeWindowState::AssessmentRequired => "assessment_required",
            AuthoritativeWindowState::Executing => "executing",
            AuthoritativeWindowState::Yielded => "yielded",
            AuthoritativeWindowState::Closed => "closed",
            AuthoritativeWindowState::RecoveryRequired => "recovery_required",
            AuthoritativeWindowState::Expired => "expired",
            AuthoritativeWindowState::Revoked => "revoked",
            AuthoritativeWindowState::Superseded => "superseded",
        }
    }

    fn frame(hasher: &mut Sha256, label: &str, value: &str) {
        hasher.update((label.len() as u64).to_be_bytes());
        hasher.update(label.as_bytes());
        hasher.update((value.len() as u64).to_be_bytes());
        hasher.update(value.as_bytes());
    }

    fn yield_reason_code(value: AuthorizedExecutionYieldReason) -> &'static str {
        match value {
            AuthorizedExecutionYieldReason::TurnBoundary => "turn_boundary",
            AuthorizedExecutionYieldReason::ContextBudget => "context_budget",
            AuthorizedExecutionYieldReason::HostPreemption => "host_preemption",
            AuthorizedExecutionYieldReason::VoluntaryCheckpoint => "voluntary_checkpoint",
            AuthorizedExecutionYieldReason::TransientExecutorFailure => {
                "transient_executor_failure"
            }
        }
    }

    fn wake_trigger_code(value: AuthorizedExecutionWakeTriggerKind) -> &'static str {
        match value {
            AuthorizedExecutionWakeTriggerKind::ApprovalDecisionRecorded => {
                "approval_decision_recorded"
            }
            AuthorizedExecutionWakeTriggerKind::EvidenceAccepted => "evidence_accepted",
            AuthorizedExecutionWakeTriggerKind::CheckAccepted => "check_accepted",
            AuthorizedExecutionWakeTriggerKind::ExternalEventRecorded => "external_event_recorded",
            AuthorizedExecutionWakeTriggerKind::CapabilityAvailabilityChanged => {
                "capability_availability_changed"
            }
            AuthorizedExecutionWakeTriggerKind::DeadlineReached => "deadline_reached",
            AuthorizedExecutionWakeTriggerKind::AuthoritySourceChanged => {
                "authority_source_changed"
            }
            AuthorizedExecutionWakeTriggerKind::ConflictResolved => "conflict_resolved",
        }
    }

    fn wait_state_code(value: AuthoritativeWaitState) -> &'static str {
        match value {
            AuthoritativeWaitState::Unsatisfied => "unsatisfied",
            AuthoritativeWaitState::Satisfied => "satisfied",
            AuthoritativeWaitState::Expired => "expired",
            AuthoritativeWaitState::Superseded => "superseded",
            AuthoritativeWaitState::Canceled => "canceled",
        }
    }

    fn attempt_outcome_code(value: AuthorizedExecutionAttemptOutcome) -> &'static str {
        match value {
            AuthorizedExecutionAttemptOutcome::Succeeded => "succeeded",
            AuthorizedExecutionAttemptOutcome::RetryableFailure => "retryable_failure",
            AuthorizedExecutionAttemptOutcome::TerminalFailure => "terminal_failure",
            AuthorizedExecutionAttemptOutcome::AmbiguousMayHaveStarted => {
                "ambiguous_may_have_started"
            }
        }
    }

    fn validate_reference(label: &'static str, value: &str) -> Result<(), WorkflowOsError> {
        let valid = !value.is_empty()
            && value.len() <= REFERENCE_MAX_BYTES
            && value.is_ascii()
            && !value.chars().any(char::is_whitespace)
            && !["secret", "token", "authorization", "private_key", "bearer"]
                .iter()
                .any(|needle| value.to_ascii_lowercase().contains(needle));
        if !valid {
            return Err(continuity_state_error(
                WorkflowOsErrorKind::Validation,
                "input.invalid",
                "authorized execution continuity input is invalid",
            ));
        }
        let _ = label;
        Ok(())
    }

    pub(crate) fn validate_wait_count(count: usize) -> Result<(), WorkflowOsError> {
        if count > MAX_WAITS {
            return Err(continuity_state_error(
                WorkflowOsErrorKind::Validation,
                "input.invalid",
                "authorized execution continuity input is invalid",
            ));
        }
        Ok(())
    }

    pub(crate) fn continuity_state_error(
        kind: WorkflowOsErrorKind,
        suffix: &'static str,
        message: &'static str,
    ) -> WorkflowOsError {
        super::state_error(kind, suffix, message)
    }
}

#[cfg(test)]
#[allow(
    clippy::expect_used,
    clippy::manual_let_else,
    clippy::panic,
    clippy::too_many_lines
)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};
    use std::sync::{Arc, Barrier, Mutex};
    use std::thread;

    use crate::{
        ActorId, AuthorizedExecutionAttemptId, AuthorizedExecutionAttemptOutcome,
        AuthorizedExecutionWaitConditionId, AuthorizedExecutionWakeTriggerKind,
        AuthorizedExecutionWindowId, AuthorizedExecutionYieldReason, EventId, EventSequenceNumber,
        ImmutableRunBundleBinding, SpecContentHash, StepId, Timestamp, WorkflowId, WorkflowOsError,
        WorkflowOsErrorKind, WorkflowRunId,
    };

    use super::internal::*;
    use super::AuthorizedExecutionContinuityOperationKind;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum InjectedFault {
        Before,
        During,
        After,
    }

    struct ReferenceStoreInner {
        state: ReferenceContinuityState,
        next_fault: Option<InjectedFault>,
    }

    struct ReferenceTrustedClockState {
        observed_at: Timestamp,
        provenance_commitment: SpecContentHash,
        available: bool,
    }

    struct ReferenceTrustedClock {
        state: Mutex<ReferenceTrustedClockState>,
    }

    impl ReferenceTrustedClock {
        fn observe(&self) -> Result<TrustedTimeObservation, WorkflowOsError> {
            let state = self.state.lock().map_err(|_| storage_error())?;
            if !state.available {
                return Err(reference_error(
                    WorkflowOsErrorKind::InvalidState,
                    "time.unavailable",
                ));
            }
            Ok(trusted_time_observation(
                state.observed_at,
                TrustedTimeSourceKind::CoreInjectedClockV1,
                state.provenance_commitment.clone(),
            ))
        }

        fn set(&self, observed_at: Timestamp) {
            self.state.lock().expect("trusted clock lock").observed_at = observed_at;
        }

        fn set_available(&self, available: bool) {
            self.state.lock().expect("trusted clock lock").available = available;
        }

        fn set_provenance(&self, provenance_commitment: SpecContentHash) {
            self.state
                .lock()
                .expect("trusted clock lock")
                .provenance_commitment = provenance_commitment;
        }
    }

    #[derive(Clone)]
    struct ReferenceStore {
        inner: Arc<Mutex<ReferenceStoreInner>>,
        clock: Arc<ReferenceTrustedClock>,
        expected_clock_provenance: SpecContentHash,
    }

    impl ReferenceStore {
        fn from_state(state: ReferenceContinuityState) -> Self {
            let expected_clock_provenance = SpecContentHash::from_text("reference core clock");
            Self {
                inner: Arc::new(Mutex::new(ReferenceStoreInner {
                    state,
                    next_fault: None,
                })),
                clock: Arc::new(ReferenceTrustedClock {
                    state: Mutex::new(ReferenceTrustedClockState {
                        observed_at: timestamp("2026-08-15T12:01:00Z"),
                        provenance_commitment: expected_clock_provenance.clone(),
                        available: true,
                    }),
                }),
                expected_clock_provenance,
            }
        }

        fn inject_fault(&self, fault: InjectedFault) {
            self.inner.lock().expect("reference lock").next_fault = Some(fault);
        }

        fn snapshot(&self) -> ReferenceContinuityState {
            self.inner.lock().expect("reference lock").state.clone()
        }

        fn set_trusted_time(&self, observed_at: Timestamp) {
            self.clock.set(observed_at);
        }

        fn set_clock_available(&self, available: bool) {
            self.clock.set_available(available);
        }

        fn set_clock_provenance(&self, provenance_commitment: SpecContentHash) {
            self.clock.set_provenance(provenance_commitment);
        }

        fn transact<F>(
            &self,
            kind: AuthorizedExecutionContinuityOperationKind,
            operation_id: &ContinuityOperationId,
            request_commitment: &SpecContentHash,
            receipt_id: &ContinuityReceiptId,
            mutation: F,
        ) -> Result<(RecordedOperationResult, bool), WorkflowOsError>
        where
            F: FnOnce(
                &mut ReferenceContinuityState,
                Timestamp,
            ) -> Result<RecordedOperationResult, WorkflowOsError>,
        {
            let mut inner = self.inner.lock().map_err(|_| storage_error())?;
            if let Some(existing) = inner.state.operations.get(operation_id) {
                if existing.operation_kind != kind
                    || &existing.request_commitment != request_commitment
                    || &existing.receipt.receipt_id != receipt_id
                {
                    return Err(reference_error(
                        WorkflowOsErrorKind::InvalidState,
                        "operation.replay_conflict",
                    ));
                }
                let expected_commitment = operation_commitment(
                    request_commitment,
                    receipt_id,
                    existing.receipt.committed_at,
                    &existing.receipt.trusted_time_commitment,
                    &existing.result,
                );
                if &existing.operation_id != operation_id
                    || existing.receipt.operation_kind != kind
                    || existing.operation_commitment != expected_commitment
                    || existing.receipt.operation_commitment != expected_commitment
                {
                    return Err(state_corrupt());
                }
                return Ok((existing.result.clone(), true));
            }

            if inner
                .state
                .operations
                .values()
                .any(|record| &record.receipt.receipt_id == receipt_id)
            {
                return Err(reference_error(
                    WorkflowOsErrorKind::InvalidState,
                    "receipt.reused",
                ));
            }

            let trusted_time = self.clock.observe()?;
            if trusted_time.source() != TrustedTimeSourceKind::CoreInjectedClockV1
                || trusted_time.provenance_commitment() != &self.expected_clock_provenance
            {
                return Err(reference_error(
                    WorkflowOsErrorKind::Security,
                    "time.untrusted",
                ));
            }
            let observed_at = trusted_time.observed_at();
            let trusted_time_binding = trusted_time_commitment(&trusted_time);
            let mut working = inner.state.clone();
            let result = mutation(&mut working, observed_at)?;
            if inner.next_fault == Some(InjectedFault::During) {
                inner.next_fault = None;
                return Err(storage_error());
            }
            let committed = operation_commitment(
                request_commitment,
                receipt_id,
                observed_at,
                &trusted_time_binding,
                &result,
            );
            let receipt = ContinuityReceipt {
                receipt_id: receipt_id.clone(),
                operation_kind: kind,
                operation_commitment: committed.clone(),
                trusted_time_commitment: trusted_time_binding,
                committed_at: observed_at,
            };
            working.operations.insert(
                operation_id.clone(),
                AuthoritativeOperationRecord {
                    operation_id: operation_id.clone(),
                    operation_kind: kind,
                    request_commitment: request_commitment.clone(),
                    operation_commitment: committed,
                    receipt,
                    result: result.clone(),
                },
            );

            match inner.next_fault.take() {
                Some(InjectedFault::Before) => return Err(storage_error()),
                Some(InjectedFault::During) => unreachable!("fault consumed before commit"),
                Some(InjectedFault::After) => {
                    inner.state = working;
                    return Err(storage_error());
                }
                None => {}
            }
            inner.state = working;
            Ok((result, false))
        }
    }

    impl AuthorizedExecutionContinuityStore for ReferenceStore {
        fn register_yield(
            &self,
            request: RegisterYieldRequest<'_>,
        ) -> Result<RegisterYieldResult, WorkflowOsError> {
            validate_wait_count(request.waits.len())?;
            let expected = expected_register_yield_commitment(&request);
            if expected != request.request_commitment {
                return Err(reference_error(
                    WorkflowOsErrorKind::Validation,
                    "input.invalid",
                ));
            }
            let operation_id = request.operation_id.clone();
            let request_commitment = request.request_commitment.clone();
            let receipt_id = request.receipt_id.clone();
            let (result, replay) = self.transact(
                AuthorizedExecutionContinuityOperationKind::RegisterYield,
                &operation_id,
                &request_commitment,
                &receipt_id,
                |state, observed_at| {
                    let window = state
                        .windows
                        .get(&request.window_id)
                        .ok_or_else(state_corrupt)?;
                    validate_window(
                        window,
                        &request.expected_window_binding,
                        request.expected_window_revision,
                        &request.cursor,
                        observed_at,
                    )?;
                    if window.state != AuthoritativeWindowState::Executing
                        || request.attempt_capability.window_id != request.window_id
                        || request.attempt_capability.window_revision
                            != request.expected_window_revision
                        || request.attempt_capability.cursor != request.cursor
                        || request.attempt_capability.attempt_id != request.attempt_id
                        || request.attempt_capability.subject_actor_id != window.subject_actor_id
                        || request.attempt_capability.authority_commitment
                            != window.authority_commitment
                        || request.attempt_capability.window_binding_commitment
                            != window_binding_commitment(&request.expected_window_binding)
                    {
                        return Err(reference_error(
                            WorkflowOsErrorKind::Security,
                            "authority.binding_mismatch",
                        ));
                    }
                    let attempt = state
                        .attempts
                        .get(&request.attempt_id)
                        .ok_or_else(state_corrupt)?;
                    if attempt.state != AuthoritativeAttemptState::Started
                        || attempt.window_id != request.window_id
                        || attempt.subject_actor_id != window.subject_actor_id
                        || attempt.cursor != request.cursor
                        || attempt.authority_commitment != window.authority_commitment
                        || attempt.consume_operation_id
                            != request.attempt_capability.consume_operation_id
                    {
                        return Err(reference_error(
                            WorkflowOsErrorKind::InvalidState,
                            "attempt.recovery_required",
                        ));
                    }
                    if state.yields.contains_key(&request.generation_id) {
                        return Err(reference_error(
                            WorkflowOsErrorKind::InvalidState,
                            "operation.replay_conflict",
                        ));
                    }
                    let distinct = request
                        .waits
                        .iter()
                        .map(|wait| &wait.condition_id)
                        .collect::<BTreeSet<_>>();
                    if distinct.len() != request.waits.len()
                        || request.waits.iter().any(|wait| wait.condition_version == 0)
                    {
                        return Err(reference_error(
                            WorkflowOsErrorKind::Validation,
                            "input.invalid",
                        ));
                    }

                    let mut wait_ids = request
                        .waits
                        .iter()
                        .map(|wait| wait.condition_id.clone())
                        .collect::<Vec<_>>();
                    wait_ids.sort();
                    for wait in &request.waits {
                        if state.waits.contains_key(&wait.condition_id) {
                            return Err(reference_error(
                                WorkflowOsErrorKind::InvalidState,
                                "operation.replay_conflict",
                            ));
                        }
                        state.waits.insert(
                            wait.condition_id.clone(),
                            AuthoritativeWaitRecord {
                                condition_id: wait.condition_id.clone(),
                                condition_version: wait.condition_version,
                                window_id: request.window_id.clone(),
                                generation_id: request.generation_id.clone(),
                                wake_trigger: wait.wake_trigger,
                                state: AuthoritativeWaitState::Unsatisfied,
                                source_commitment: None,
                                source_revision: None,
                                revision: ContinuityRevision::new(1)?,
                            },
                        );
                    }
                    state.yields.insert(
                        request.generation_id.clone(),
                        AuthoritativeYieldRecord {
                            generation_id: request.generation_id.clone(),
                            attempt_id: request.attempt_id.clone(),
                            cursor: request.cursor.clone(),
                            reason: request.reason,
                            wait_ids,
                            registered_at: observed_at,
                        },
                    );
                    let attempt = state
                        .attempts
                        .get_mut(&request.attempt_id)
                        .ok_or_else(state_corrupt)?;
                    attempt.state = AuthoritativeAttemptState::Yielded;
                    attempt.revision = attempt.revision.next();
                    let window = state
                        .windows
                        .get_mut(&request.window_id)
                        .ok_or_else(state_corrupt)?;
                    window.state = AuthoritativeWindowState::Yielded;
                    window.active_yield = Some(request.generation_id.clone());
                    window.trusted_time_watermark = observed_at;
                    window.revision = window.revision.next();
                    Ok(RecordedOperationResult::YieldRegistered {
                        generation_id: request.generation_id.clone(),
                        window_revision: window.revision,
                    })
                },
            )?;
            Ok(if replay {
                RegisterYieldResult::ExactReplay(result)
            } else {
                RegisterYieldResult::Registered(result)
            })
        }

        fn transition_wait(
            &self,
            request: TransitionWaitRequest<'_>,
        ) -> Result<MutationResult, WorkflowOsError> {
            let expected = expected_transition_wait_commitment(&request);
            if expected != request.request_commitment
                || request.target != AuthoritativeWaitState::Satisfied
                || request.wake_capability.is_none()
            {
                return Err(reference_error(
                    WorkflowOsErrorKind::Validation,
                    "input.invalid",
                ));
            }
            let operation_id = request.operation_id.clone();
            let request_commitment = request.request_commitment.clone();
            let receipt_id = request.receipt_id.clone();
            let (result, replay) = self.transact(
                AuthorizedExecutionContinuityOperationKind::TransitionWait,
                &operation_id,
                &request_commitment,
                &receipt_id,
                |state, observed_at| {
                    let window = state
                        .windows
                        .get(&request.window_id)
                        .ok_or_else(state_corrupt)?;
                    validate_window(
                        window,
                        &request.expected_window_binding,
                        request.expected_window_revision,
                        &request.cursor,
                        observed_at,
                    )?;
                    if window.state != AuthoritativeWindowState::Yielded {
                        return Err(reference_error(
                            WorkflowOsErrorKind::InvalidState,
                            "window.ineligible",
                        ));
                    }
                    if window.active_yield.as_ref() != Some(&request.expected_generation_id) {
                        return Err(reference_error(
                            WorkflowOsErrorKind::InvalidState,
                            "yield.generation_stale",
                        ));
                    }
                    let active_yield = state
                        .yields
                        .get(&request.expected_generation_id)
                        .ok_or_else(state_corrupt)?;
                    if active_yield.cursor != request.cursor
                        || !active_yield.wait_ids.contains(&request.condition_id)
                    {
                        return Err(reference_error(
                            WorkflowOsErrorKind::InvalidState,
                            "yield.generation_stale",
                        ));
                    }
                    let wait = state
                        .waits
                        .get(&request.condition_id)
                        .ok_or_else(state_corrupt)?;
                    if wait.window_id != request.window_id
                        || wait.generation_id != request.expected_generation_id
                        || wait.condition_version != request.expected_condition_version
                        || wait.revision != request.expected_wait_revision
                        || wait.state != AuthoritativeWaitState::Unsatisfied
                    {
                        return Err(reference_error(
                            WorkflowOsErrorKind::InvalidState,
                            "wait.revision_stale",
                        ));
                    }
                    let capability = request.wake_capability.ok_or_else(|| {
                        reference_error(WorkflowOsErrorKind::Security, "wake.unavailable")
                    })?;
                    if capability.window_id != request.window_id
                        || capability.generation_id != request.expected_generation_id
                        || capability.condition_id != request.condition_id
                        || capability.condition_version != request.expected_condition_version
                        || capability.trigger != wait.wake_trigger
                        || capability.source_revision == 0
                    {
                        return Err(reference_error(
                            WorkflowOsErrorKind::Security,
                            "wake.binding_mismatch",
                        ));
                    }
                    let wait = state
                        .waits
                        .get_mut(&request.condition_id)
                        .ok_or_else(state_corrupt)?;
                    wait.state = request.target;
                    wait.revision = wait.revision.next();
                    wait.source_commitment = Some(capability.source_commitment.clone());
                    wait.source_revision = Some(capability.source_revision);
                    let wait_revision = wait.revision;
                    let window = state
                        .windows
                        .get_mut(&request.window_id)
                        .ok_or_else(state_corrupt)?;
                    window.trusted_time_watermark = observed_at;
                    window.revision = window.revision.next();
                    Ok(RecordedOperationResult::WaitTransitioned {
                        wait_revision,
                        window_revision: window.revision,
                    })
                },
            )?;
            Ok(if replay {
                MutationResult::ExactReplay(result)
            } else {
                MutationResult::Recorded(result)
            })
        }

        fn consume_directive(
            &self,
            request: ConsumeDirectiveRequest<'_>,
        ) -> Result<ConsumeDirectiveResult, WorkflowOsError> {
            let expected = expected_consume_directive_commitment(&request);
            if expected != request.request_commitment {
                return Err(reference_error(
                    WorkflowOsErrorKind::Validation,
                    "input.invalid",
                ));
            }
            validate_wait_count(request.expected_waits.len())?;
            let operation_id = request.operation_id.clone();
            let request_commitment = request.request_commitment.clone();
            let receipt_id = request.receipt_id.clone();
            let subject = request.authority_capability.subject_actor_id.clone();
            let authority_commitment = request.authority_capability.authority_commitment.clone();
            let cursor = request.cursor.clone();
            let attempt_id = request.generated_attempt_id.clone();
            let consume_operation_id = request.operation_id.clone();
            let (result, replay) = self.transact(
                AuthorizedExecutionContinuityOperationKind::ConsumeDirective,
                &operation_id,
                &request_commitment,
                &receipt_id,
                |state, observed_at| {
                    let window = state
                        .windows
                        .get(&request.window_id)
                        .ok_or_else(state_corrupt)?;
                    validate_window(
                        window,
                        &request.expected_window_binding,
                        request.expected_window_revision,
                        &request.cursor,
                        observed_at,
                    )?;
                    if window.state != AuthoritativeWindowState::Yielded
                        || window.active_yield.as_ref() != Some(&request.generation_id)
                    {
                        return Err(reference_error(
                            WorkflowOsErrorKind::InvalidState,
                            "window.ineligible",
                        ));
                    }
                    if request.authority_capability.window_id != request.window_id
                        || request.authority_capability.window_revision
                            != request.expected_window_revision
                        || request.authority_capability.generation_id != request.generation_id
                        || request.authority_capability.cursor != request.cursor
                        || request.authority_capability.subject_actor_id != window.subject_actor_id
                        || request.authority_capability.authority_commitment
                            != window.authority_commitment
                        || request.authority_capability.window_binding_commitment
                            != window_binding_commitment(&request.expected_window_binding)
                        || request.authority_capability.expected_waits != request.expected_waits
                    {
                        return Err(reference_error(
                            WorkflowOsErrorKind::Security,
                            "authority.binding_mismatch",
                        ));
                    }
                    if window.next_attempt_number > window.maximum_attempts {
                        return Err(reference_error(
                            WorkflowOsErrorKind::InvalidState,
                            "attempt.budget_exhausted",
                        ));
                    }
                    let yield_record = state
                        .yields
                        .get(&request.generation_id)
                        .ok_or_else(state_corrupt)?;
                    let expected_wait_ids = request
                        .expected_waits
                        .iter()
                        .map(|wait| &wait.condition_id)
                        .collect::<BTreeSet<_>>();
                    let yielded_wait_ids = yield_record.wait_ids.iter().collect::<BTreeSet<_>>();
                    let waits_match = expected_wait_ids.len() == request.expected_waits.len()
                        && expected_wait_ids == yielded_wait_ids
                        && request.expected_waits.iter().all(|expected| {
                            state.waits.get(&expected.condition_id).is_some_and(|wait| {
                                wait.window_id == request.window_id
                                    && wait.generation_id == request.generation_id
                                    && wait.condition_version == expected.condition_version
                                    && wait.revision == expected.revision
                                    && wait.state == AuthoritativeWaitState::Satisfied
                            })
                        });
                    if yield_record.cursor != request.cursor || !waits_match {
                        return Err(reference_error(
                            WorkflowOsErrorKind::InvalidState,
                            "wait.unsatisfied",
                        ));
                    }
                    let directive = state
                        .directives
                        .get_mut(&request.directive_id)
                        .ok_or_else(state_corrupt)?;
                    if directive.state != AuthoritativeDirectiveState::Available
                        || directive.window_id != request.window_id
                        || directive.generation_id != request.generation_id
                        || directive.cursor != request.cursor
                        || directive.authority_commitment
                            != request.authority_capability.authority_commitment
                    {
                        return Err(reference_error(
                            WorkflowOsErrorKind::InvalidState,
                            "directive.already_consumed",
                        ));
                    }
                    if state.attempts.contains_key(&request.generated_attempt_id) {
                        return Err(reference_error(
                            WorkflowOsErrorKind::InvalidState,
                            "operation.replay_conflict",
                        ));
                    }
                    directive.state = AuthoritativeDirectiveState::Consumed;
                    directive.revision = directive.revision.next();
                    let attempt_number = window.next_attempt_number;
                    state.attempts.insert(
                        request.generated_attempt_id.clone(),
                        AuthoritativeAttemptRecord {
                            attempt_id: request.generated_attempt_id.clone(),
                            attempt_number,
                            window_id: request.window_id.clone(),
                            subject_actor_id: window.subject_actor_id.clone(),
                            cursor: request.cursor.clone(),
                            authority_commitment: window.authority_commitment.clone(),
                            consume_operation_id: request.operation_id.clone(),
                            state: AuthoritativeAttemptState::Started,
                            revision: ContinuityRevision::new(1)?,
                        },
                    );
                    let window = state
                        .windows
                        .get_mut(&request.window_id)
                        .ok_or_else(state_corrupt)?;
                    window.state = AuthoritativeWindowState::Executing;
                    window.active_yield = None;
                    window.next_attempt_number += 1;
                    window.trusted_time_watermark = observed_at;
                    window.revision = window.revision.next();
                    Ok(RecordedOperationResult::DirectiveConsumed {
                        attempt_id: request.generated_attempt_id.clone(),
                        attempt_number,
                        window_revision: window.revision,
                    })
                },
            )?;
            if replay {
                return Ok(ConsumeDirectiveResult::ExactReplay(result));
            }
            let window_revision = match result {
                RecordedOperationResult::DirectiveConsumed {
                    window_revision, ..
                } => window_revision,
                _ => return Err(state_corrupt()),
            };
            Ok(ConsumeDirectiveResult::Consumed {
                result,
                capability: AttemptUseCapability {
                    attempt_id,
                    subject_actor_id: subject,
                    window_id: request.window_id,
                    window_revision,
                    cursor,
                    authority_commitment,
                    window_binding_commitment: window_binding_commitment(
                        &request.expected_window_binding,
                    ),
                    consume_operation_id,
                },
            })
        }

        fn record_attempt_outcome(
            &self,
            request: RecordAttemptOutcomeRequest<'_>,
        ) -> Result<MutationResult, WorkflowOsError> {
            let expected = expected_attempt_outcome_commitment(&request);
            if expected != request.request_commitment
                || request.outcome == AuthorizedExecutionAttemptOutcome::AmbiguousMayHaveStarted
            {
                return Err(reference_error(
                    WorkflowOsErrorKind::Validation,
                    "input.invalid",
                ));
            }
            let operation_id = request.operation_id.clone();
            let request_commitment = request.request_commitment.clone();
            let receipt_id = request.receipt_id.clone();
            let (result, replay) = self.transact(
                AuthorizedExecutionContinuityOperationKind::RecordAttemptOutcome,
                &operation_id,
                &request_commitment,
                &receipt_id,
                |state, observed_at| {
                    let window = state
                        .windows
                        .get(&request.window_id)
                        .ok_or_else(state_corrupt)?;
                    validate_window(
                        window,
                        &request.expected_window_binding,
                        request.expected_window_revision,
                        &request.attempt_capability.cursor,
                        observed_at,
                    )?;
                    if window.state != AuthoritativeWindowState::Executing
                        || request.attempt_capability.window_id != request.window_id
                        || request.attempt_capability.window_revision
                            != request.expected_window_revision
                        || request.attempt_capability.attempt_id != request.attempt_id
                        || request.attempt_capability.subject_actor_id != window.subject_actor_id
                        || request.attempt_capability.authority_commitment
                            != window.authority_commitment
                        || request.attempt_capability.window_binding_commitment
                            != window_binding_commitment(&request.expected_window_binding)
                    {
                        return Err(reference_error(
                            WorkflowOsErrorKind::Security,
                            "authority.binding_mismatch",
                        ));
                    }
                    let attempt = state
                        .attempts
                        .get_mut(&request.attempt_id)
                        .ok_or_else(state_corrupt)?;
                    if attempt.state != AuthoritativeAttemptState::Started
                        || attempt.revision != request.expected_attempt_revision
                        || attempt.window_id != request.window_id
                        || attempt.subject_actor_id != window.subject_actor_id
                        || attempt.cursor != request.attempt_capability.cursor
                        || attempt.authority_commitment != window.authority_commitment
                        || attempt.consume_operation_id
                            != request.attempt_capability.consume_operation_id
                    {
                        return Err(reference_error(
                            WorkflowOsErrorKind::InvalidState,
                            "attempt.outcome_already_recorded",
                        ));
                    }
                    attempt.state = match request.outcome {
                        AuthorizedExecutionAttemptOutcome::Succeeded => {
                            AuthoritativeAttemptState::Succeeded
                        }
                        AuthorizedExecutionAttemptOutcome::RetryableFailure => {
                            AuthoritativeAttemptState::RetryableFailure
                        }
                        AuthorizedExecutionAttemptOutcome::TerminalFailure => {
                            AuthoritativeAttemptState::TerminalFailure
                        }
                        AuthorizedExecutionAttemptOutcome::AmbiguousMayHaveStarted => {
                            return Err(reference_error(
                                WorkflowOsErrorKind::Validation,
                                "input.invalid",
                            ));
                        }
                    };
                    attempt.revision = attempt.revision.next();
                    let attempt_state = attempt.state;
                    let window = state
                        .windows
                        .get_mut(&request.window_id)
                        .ok_or_else(state_corrupt)?;
                    window.state = AuthoritativeWindowState::Closed;
                    window.trusted_time_watermark = observed_at;
                    window.revision = window.revision.next();
                    Ok(RecordedOperationResult::AttemptOutcomeRecorded {
                        attempt_state,
                        window_state: window.state,
                        window_revision: window.revision,
                    })
                },
            )?;
            Ok(if replay {
                MutationResult::ExactReplay(result)
            } else {
                MutationResult::Recorded(result)
            })
        }

        fn recover_ambiguous_attempt(
            &self,
            request: RecoverAmbiguousAttemptRequest,
        ) -> Result<MutationResult, WorkflowOsError> {
            let expected = expected_recovery_commitment(&request);
            if expected != request.request_commitment {
                return Err(reference_error(
                    WorkflowOsErrorKind::Validation,
                    "input.invalid",
                ));
            }
            let operation_id = request.operation_id.clone();
            let request_commitment = request.request_commitment.clone();
            let receipt_id = request.receipt_id.clone();
            let (result, replay) = self.transact(
                AuthorizedExecutionContinuityOperationKind::RecoverAmbiguousAttempt,
                &operation_id,
                &request_commitment,
                &receipt_id,
                |state, observed_at| {
                    let window = state
                        .windows
                        .get(&request.window_id)
                        .ok_or_else(state_corrupt)?;
                    validate_window(
                        window,
                        &request.expected_window_binding,
                        request.expected_window_revision,
                        &request.cursor,
                        observed_at,
                    )?;
                    if window.state != AuthoritativeWindowState::Executing {
                        return Err(reference_error(
                            WorkflowOsErrorKind::InvalidState,
                            "window.ineligible",
                        ));
                    }
                    let attempt = state
                        .attempts
                        .get_mut(&request.attempt_id)
                        .ok_or_else(state_corrupt)?;
                    if attempt.state != AuthoritativeAttemptState::Started
                        || attempt.revision != request.expected_attempt_revision
                        || attempt.window_id != request.window_id
                        || attempt.subject_actor_id != window.subject_actor_id
                        || attempt.cursor != request.cursor
                        || attempt.authority_commitment != window.authority_commitment
                    {
                        return Err(reference_error(
                            WorkflowOsErrorKind::InvalidState,
                            "attempt.outcome_already_recorded",
                        ));
                    }
                    attempt.state = AuthoritativeAttemptState::AmbiguousMayHaveStarted;
                    attempt.revision = attempt.revision.next();
                    let window = state
                        .windows
                        .get_mut(&request.window_id)
                        .ok_or_else(state_corrupt)?;
                    window.state = AuthoritativeWindowState::RecoveryRequired;
                    window.trusted_time_watermark = observed_at;
                    window.revision = window.revision.next();
                    Ok(RecordedOperationResult::AttemptOutcomeRecorded {
                        attempt_state: AuthoritativeAttemptState::AmbiguousMayHaveStarted,
                        window_state: AuthoritativeWindowState::RecoveryRequired,
                        window_revision: window.revision,
                    })
                },
            )?;
            Ok(if replay {
                MutationResult::ExactReplay(result)
            } else {
                MutationResult::Recorded(result)
            })
        }
    }

    fn validate_window(
        window: &AuthoritativeWindowRecord,
        expected_binding: &ExpectedWindowBinding,
        expected_revision: ContinuityRevision,
        expected_cursor: &ContinuityCursor,
        observed_at: Timestamp,
    ) -> Result<(), WorkflowOsError> {
        if window.workflow_id != expected_binding.workflow_id
            || window.run_id != expected_binding.run_id
            || window.step_id != expected_binding.step_id
            || window.subject_actor_id != expected_binding.subject_actor_id
            || window.immutable_run_bundle != expected_binding.immutable_run_bundle
            || window.governance_commitment != expected_binding.governance_commitment
            || window.authority_commitment != expected_binding.authority_commitment
            || window.cursor != expected_binding.cursor
        {
            return Err(reference_error(
                WorkflowOsErrorKind::Security,
                "window.binding_mismatch",
            ));
        }
        if window.revision != expected_revision {
            return Err(reference_error(
                WorkflowOsErrorKind::InvalidState,
                "window.revision_stale",
            ));
        }
        if &window.cursor != expected_cursor {
            return Err(reference_error(
                WorkflowOsErrorKind::InvalidState,
                "cursor.stale",
            ));
        }
        validate_time(window, observed_at)
    }

    fn validate_time(
        window: &AuthoritativeWindowRecord,
        observed_at: Timestamp,
    ) -> Result<(), WorkflowOsError> {
        if observed_at < window.trusted_time_watermark {
            return Err(reference_error(
                WorkflowOsErrorKind::InvalidState,
                "time.regressed",
            ));
        }
        if observed_at >= window.expires_at {
            return Err(reference_error(
                WorkflowOsErrorKind::InvalidState,
                "time.expired",
            ));
        }
        Ok(())
    }

    fn reference_error(kind: WorkflowOsErrorKind, suffix: &'static str) -> WorkflowOsError {
        continuity_state_error(
            kind,
            suffix,
            "authorized execution continuity state operation failed",
        )
    }

    fn storage_error() -> WorkflowOsError {
        reference_error(WorkflowOsErrorKind::InvalidState, "backend.unavailable")
    }

    fn state_corrupt() -> WorkflowOsError {
        reference_error(WorkflowOsErrorKind::InvalidState, "state.corrupt")
    }

    #[derive(Clone)]
    struct Fixture {
        store: ReferenceStore,
        window_id: AuthorizedExecutionWindowId,
        generation_id: ContinuityYieldGenerationId,
        directive_id: ContinuityDirectiveId,
        cursor: ContinuityCursor,
        subject: ActorId,
        authority_commitment: SpecContentHash,
        wait_id: Option<AuthorizedExecutionWaitConditionId>,
    }

    impl Fixture {
        fn yielded(with_wait: bool) -> Self {
            let workflow_id = WorkflowId::new("workflow/continuity-reference").expect("workflow");
            let run_id = WorkflowRunId::new("run/continuity-reference").expect("run");
            let step_id = StepId::new("step-reference").expect("step");
            let window_id =
                AuthorizedExecutionWindowId::new("window/continuity-reference").expect("window");
            let subject = ActorId::new("agent/continuity-reference").expect("subject");
            let cursor = ContinuityCursor {
                sequence_number: EventSequenceNumber::new(7).expect("sequence"),
                event_id: EventId::new("event/continuity-yielded").expect("event"),
            };
            let generation_id = ContinuityYieldGenerationId::new("yield/continuity-reference/1")
                .expect("yield generation");
            let directive_id =
                ContinuityDirectiveId::new("directive/continuity-reference/1").expect("directive");
            let prior_attempt_id =
                AuthorizedExecutionAttemptId::new("attempt/continuity-reference/1")
                    .expect("attempt");
            let authority_commitment = SpecContentHash::from_text("reference authority");
            let wait_id = with_wait.then(|| {
                AuthorizedExecutionWaitConditionId::new("wait/continuity-reference/1")
                    .expect("wait")
            });
            let binding: ImmutableRunBundleBinding = serde_json::from_value(serde_json::json!({
                "bundle_id": "bundle/continuity-reference",
                "bundle_version": "v1",
                "root_hash": SpecContentHash::from_text("reference bundle").as_str(),
            }))
            .expect("bundle binding");
            let revision = ContinuityRevision::new(1).expect("revision");
            let watermark = timestamp("2026-08-15T12:00:00Z");
            let window = AuthoritativeWindowRecord {
                workflow_id,
                run_id,
                step_id,
                window_id: window_id.clone(),
                subject_actor_id: subject.clone(),
                immutable_run_bundle: binding,
                governance_commitment: SpecContentHash::from_text("reference governance"),
                authority_commitment: authority_commitment.clone(),
                cursor: cursor.clone(),
                state: AuthoritativeWindowState::Yielded,
                maximum_attempts: 3,
                next_attempt_number: 2,
                expires_at: timestamp("2026-08-15T13:00:00Z"),
                trusted_time_watermark: watermark,
                revision,
                active_yield: Some(generation_id.clone()),
            };
            let prior_attempt = AuthoritativeAttemptRecord {
                attempt_id: prior_attempt_id.clone(),
                attempt_number: 1,
                window_id: window_id.clone(),
                subject_actor_id: subject.clone(),
                cursor: cursor.clone(),
                authority_commitment: authority_commitment.clone(),
                consume_operation_id: ContinuityOperationId::new("operation/prior-consume")
                    .expect("operation"),
                state: AuthoritativeAttemptState::Yielded,
                revision,
            };
            let yield_record = AuthoritativeYieldRecord {
                generation_id: generation_id.clone(),
                attempt_id: prior_attempt_id,
                cursor: cursor.clone(),
                reason: AuthorizedExecutionYieldReason::TurnBoundary,
                wait_ids: wait_id.iter().cloned().collect(),
                registered_at: watermark,
            };
            let directive = AuthoritativeDirectiveRecord {
                directive_id: directive_id.clone(),
                window_id: window_id.clone(),
                generation_id: generation_id.clone(),
                cursor: cursor.clone(),
                authority_commitment: authority_commitment.clone(),
                state: AuthoritativeDirectiveState::Available,
                revision,
            };
            let mut waits = BTreeMap::new();
            if let Some(condition_id) = &wait_id {
                waits.insert(
                    condition_id.clone(),
                    AuthoritativeWaitRecord {
                        condition_id: condition_id.clone(),
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
            Self {
                store: ReferenceStore::from_state(ReferenceContinuityState {
                    windows: BTreeMap::from([(window_id.clone(), window)]),
                    yields: BTreeMap::from([(generation_id.clone(), yield_record)]),
                    waits,
                    directives: BTreeMap::from([(directive_id.clone(), directive)]),
                    attempts: BTreeMap::from([(prior_attempt.attempt_id.clone(), prior_attempt)]),
                    operations: BTreeMap::new(),
                }),
                window_id,
                generation_id,
                directive_id,
                cursor,
                subject,
                authority_commitment,
                wait_id,
            }
        }

        fn authority_capability(&self) -> AuthorityUseCapability {
            let snapshot = self.store.snapshot();
            let window = snapshot.windows.get(&self.window_id).expect("window");
            let expected_waits = fixture_expected_waits(&snapshot, &self.generation_id);
            let binding = fixture_window_binding(window);
            AuthorityUseCapability {
                window_id: self.window_id.clone(),
                window_revision: window.revision,
                generation_id: self.generation_id.clone(),
                cursor: self.cursor.clone(),
                subject_actor_id: self.subject.clone(),
                authority_commitment: self.authority_commitment.clone(),
                window_binding_commitment: window_binding_commitment(&binding),
                expected_waits,
            }
        }

        fn window_binding(&self) -> ExpectedWindowBinding {
            let snapshot = self.store.snapshot();
            fixture_window_binding(snapshot.windows.get(&self.window_id).expect("window"))
        }

        fn wake_capability(&self) -> WakeAssessmentCapability {
            WakeAssessmentCapability {
                window_id: self.window_id.clone(),
                generation_id: self.generation_id.clone(),
                condition_id: self.wait_id.clone().expect("wait fixture"),
                condition_version: 1,
                trigger: AuthorizedExecutionWakeTriggerKind::EvidenceAccepted,
                source_reference: ContinuityWakeSourceReference::new("evidence/reference/1")
                    .expect("source"),
                source_commitment: SpecContentHash::from_text("wake source"),
                source_revision: 1,
            }
        }
    }

    fn fixture_window_binding(window: &AuthoritativeWindowRecord) -> ExpectedWindowBinding {
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

    fn fixture_expected_waits(
        snapshot: &ReferenceContinuityState,
        generation_id: &ContinuityYieldGenerationId,
    ) -> Vec<ExpectedWaitRevision> {
        snapshot
            .yields
            .get(generation_id)
            .expect("yield")
            .wait_ids
            .iter()
            .map(|condition_id| {
                let wait = snapshot.waits.get(condition_id).expect("wait");
                ExpectedWaitRevision {
                    condition_id: condition_id.clone(),
                    condition_version: wait.condition_version,
                    revision: wait.revision,
                }
            })
            .collect()
    }

    fn consume_request<'a>(
        fixture: &Fixture,
        capability: &'a AuthorityUseCapability,
        operation: &str,
        attempt: &str,
        observed_at: Timestamp,
    ) -> ConsumeDirectiveRequest<'a> {
        fixture.store.set_trusted_time(observed_at);
        let operation_id = ContinuityOperationId::new(operation).expect("operation");
        let snapshot = fixture.store.snapshot();
        let window = snapshot.windows.get(&fixture.window_id).expect("window");
        let expected_waits = fixture_expected_waits(&snapshot, &fixture.generation_id);
        let mut request = ConsumeDirectiveRequest {
            operation_id,
            request_commitment: SpecContentHash::from_text("placeholder"),
            receipt_id: ContinuityReceiptId::new(format!("receipt/{operation}")).expect("receipt"),
            directive_id: fixture.directive_id.clone(),
            window_id: fixture.window_id.clone(),
            expected_window_revision: window.revision,
            expected_window_binding: fixture_window_binding(window),
            generation_id: fixture.generation_id.clone(),
            cursor: fixture.cursor.clone(),
            expected_waits,
            authority_capability: capability,
            generated_attempt_id: AuthorizedExecutionAttemptId::new(attempt).expect("attempt"),
        };
        request.request_commitment = expected_consume_directive_commitment(&request);
        request
    }

    fn register_yield_request<'a>(
        fixture: &Fixture,
        capability: &'a AttemptUseCapability,
        operation: &str,
        generation: &str,
    ) -> RegisterYieldRequest<'a> {
        fixture
            .store
            .set_trusted_time(timestamp("2026-08-15T12:02:00Z"));
        let operation_id = ContinuityOperationId::new(operation).expect("operation");
        let mut request = RegisterYieldRequest {
            operation_id,
            request_commitment: SpecContentHash::from_text("placeholder"),
            receipt_id: ContinuityReceiptId::new(format!("receipt/{operation}")).expect("receipt"),
            generation_id: ContinuityYieldGenerationId::new(generation).expect("generation"),
            window_id: fixture.window_id.clone(),
            expected_window_revision: capability.window_revision,
            expected_window_binding: fixture.window_binding(),
            cursor: fixture.cursor.clone(),
            attempt_id: capability.attempt_id.clone(),
            attempt_capability: capability,
            reason: AuthorizedExecutionYieldReason::TurnBoundary,
            waits: Vec::new(),
        };
        request.request_commitment = expected_register_yield_commitment(&request);
        request
    }

    fn transition_wait_request<'a>(
        fixture: &Fixture,
        capability: &'a WakeAssessmentCapability,
        operation: &str,
    ) -> TransitionWaitRequest<'a> {
        fixture
            .store
            .set_trusted_time(timestamp("2026-08-15T12:01:00Z"));
        let snapshot = fixture.store.snapshot();
        let window = snapshot.windows.get(&fixture.window_id).expect("window");
        let wait_id = fixture.wait_id.clone().expect("wait");
        let wait = snapshot.waits.get(&wait_id).expect("wait");
        let operation_id = ContinuityOperationId::new(operation).expect("operation");
        let mut request = TransitionWaitRequest {
            operation_id,
            request_commitment: SpecContentHash::from_text("placeholder"),
            receipt_id: ContinuityReceiptId::new(format!("receipt/{operation}")).expect("receipt"),
            window_id: fixture.window_id.clone(),
            expected_window_revision: window.revision,
            expected_window_binding: fixture_window_binding(window),
            cursor: fixture.cursor.clone(),
            condition_id: wait_id,
            expected_generation_id: fixture.generation_id.clone(),
            expected_condition_version: wait.condition_version,
            expected_wait_revision: wait.revision,
            target: AuthoritativeWaitState::Satisfied,
            wake_capability: Some(capability),
        };
        request.request_commitment = expected_transition_wait_commitment(&request);
        request
    }

    fn attempt_outcome_request<'a>(
        fixture: &Fixture,
        capability: &'a AttemptUseCapability,
        operation: &str,
    ) -> RecordAttemptOutcomeRequest<'a> {
        fixture
            .store
            .set_trusted_time(timestamp("2026-08-15T12:02:00Z"));
        let operation_id = ContinuityOperationId::new(operation).expect("operation");
        let mut request = RecordAttemptOutcomeRequest {
            operation_id,
            request_commitment: SpecContentHash::from_text("placeholder"),
            receipt_id: ContinuityReceiptId::new(format!("receipt/{operation}")).expect("receipt"),
            window_id: fixture.window_id.clone(),
            expected_window_revision: capability.window_revision,
            expected_window_binding: fixture.window_binding(),
            attempt_id: capability.attempt_id.clone(),
            expected_attempt_revision: ContinuityRevision::new(1).expect("revision"),
            attempt_capability: capability,
            outcome: AuthorizedExecutionAttemptOutcome::Succeeded,
        };
        request.request_commitment = expected_attempt_outcome_commitment(&request);
        request
    }

    fn recovery_request(
        fixture: &Fixture,
        attempt_id: AuthorizedExecutionAttemptId,
        operation: &str,
    ) -> RecoverAmbiguousAttemptRequest {
        fixture
            .store
            .set_trusted_time(timestamp("2026-08-15T12:02:00Z"));
        let snapshot = fixture.store.snapshot();
        let window = snapshot.windows.get(&fixture.window_id).expect("window");
        let attempt = snapshot.attempts.get(&attempt_id).expect("attempt");
        let operation_id = ContinuityOperationId::new(operation).expect("operation");
        let mut request = RecoverAmbiguousAttemptRequest {
            operation_id,
            request_commitment: SpecContentHash::from_text("placeholder"),
            receipt_id: ContinuityReceiptId::new(format!("receipt/{operation}")).expect("receipt"),
            window_id: fixture.window_id.clone(),
            expected_window_revision: window.revision,
            expected_window_binding: fixture_window_binding(window),
            cursor: fixture.cursor.clone(),
            attempt_id,
            expected_attempt_revision: attempt.revision,
        };
        request.request_commitment = expected_recovery_commitment(&request);
        request
    }

    fn timestamp(value: &str) -> Timestamp {
        Timestamp::parse_rfc3339(value).expect("timestamp")
    }

    #[test]
    fn one_consumer_wins_and_exact_replay_returns_no_attempt_capability() {
        let fixture = Fixture::yielded(false);
        let authority = fixture.authority_capability();
        let first = consume_request(
            &fixture,
            &authority,
            "consume/one",
            "attempt/continuity-reference/2",
            timestamp("2026-08-15T12:01:00Z"),
        );
        let replay = consume_request(
            &fixture,
            &authority,
            "consume/one",
            "attempt/continuity-reference/2",
            timestamp("2026-08-15T12:01:00Z"),
        );
        let competitor = consume_request(
            &fixture,
            &authority,
            "consume/two",
            "attempt/continuity-reference/3",
            timestamp("2026-08-15T12:01:01Z"),
        );
        let consumed = fixture
            .store
            .consume_directive(first)
            .expect("first consumer wins");
        let capability = match consumed {
            ConsumeDirectiveResult::Consumed { capability, .. } => capability,
            ConsumeDirectiveResult::ExactReplay(_) => panic!("first call is not replay"),
        };
        assert!(format!("{capability:?}").contains("[REDACTED]"));
        assert!(!format!("{capability:?}").contains("continuity-reference"));

        assert!(matches!(
            fixture.store.consume_directive(replay).expect("replay"),
            ConsumeDirectiveResult::ExactReplay(_)
        ));

        let error = match fixture.store.consume_directive(competitor) {
            Err(error) => error,
            Ok(_) => panic!("second consumer must lose"),
        };
        assert_eq!(
            error.code(),
            "authorized_execution_continuity_state.window.revision_stale"
        );
    }

    #[test]
    fn concurrent_directive_consumers_have_exactly_one_winner() {
        let fixture = Fixture::yielded(false);
        let barrier = Arc::new(Barrier::new(3));
        let mut handles = Vec::new();

        for index in 0..2 {
            let fixture = fixture.clone();
            let barrier = Arc::clone(&barrier);
            handles.push(thread::spawn(move || {
                let authority = fixture.authority_capability();
                let request = consume_request(
                    &fixture,
                    &authority,
                    &format!("consume/concurrent-{index}"),
                    &format!("attempt/continuity-reference/concurrent-{index}"),
                    timestamp("2026-08-15T12:01:00Z"),
                );
                barrier.wait();
                fixture.store.consume_directive(request)
            }));
        }

        barrier.wait();
        let results = handles
            .into_iter()
            .map(|handle| handle.join().expect("consumer thread"))
            .collect::<Vec<_>>();
        assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
        assert_eq!(results.iter().filter(|result| result.is_err()).count(), 1);

        let snapshot = fixture.store.snapshot();
        assert_eq!(snapshot.attempts.len(), 2);
        assert_eq!(
            snapshot
                .directives
                .get(&fixture.directive_id)
                .expect("directive")
                .state,
            AuthoritativeDirectiveState::Consumed
        );
        assert_eq!(snapshot.operations.len(), 1);
    }

    #[test]
    fn concurrent_yield_registrations_have_exactly_one_winner() {
        let fixture = Fixture::yielded(false);
        let authority = fixture.authority_capability();
        let consumed = fixture
            .store
            .consume_directive(consume_request(
                &fixture,
                &authority,
                "consume/concurrent-yield",
                "attempt/continuity-reference/concurrent-yield",
                timestamp("2026-08-15T12:01:00Z"),
            ))
            .expect("consume");
        let capability = match consumed {
            ConsumeDirectiveResult::Consumed { capability, .. } => capability,
            ConsumeDirectiveResult::ExactReplay(_) => panic!("first call is not replay"),
        };
        let barrier = Arc::new(Barrier::new(3));

        let results = thread::scope(|scope| {
            let mut handles = Vec::new();
            for index in 0..2 {
                let fixture = fixture.clone();
                let barrier = Arc::clone(&barrier);
                let capability = &capability;
                handles.push(scope.spawn(move || {
                    let request = register_yield_request(
                        &fixture,
                        capability,
                        &format!("yield/concurrent-{index}"),
                        &format!("yield/continuity-reference/concurrent-{index}"),
                    );
                    barrier.wait();
                    fixture.store.register_yield(request)
                }));
            }
            barrier.wait();
            handles
                .into_iter()
                .map(|handle| handle.join().expect("yield thread"))
                .collect::<Vec<_>>()
        });

        assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
        assert_eq!(results.iter().filter(|result| result.is_err()).count(), 1);
        let snapshot = fixture.store.snapshot();
        assert_eq!(snapshot.yields.len(), 2);
        assert_eq!(snapshot.operations.len(), 2);
        assert_eq!(
            snapshot.attempts[&capability.attempt_id].state,
            AuthoritativeAttemptState::Yielded
        );
    }

    #[test]
    fn attempt_budget_and_restart_rehydration_remain_fail_closed() {
        let exhausted = Fixture::yielded(false);
        exhausted
            .store
            .inner
            .lock()
            .expect("reference lock")
            .state
            .windows
            .get_mut(&exhausted.window_id)
            .expect("window")
            .next_attempt_number = 4;
        let authority = exhausted.authority_capability();
        let before = exhausted.store.snapshot();
        let error = match exhausted.store.consume_directive(consume_request(
            &exhausted,
            &authority,
            "consume/budget-exhausted",
            "attempt/continuity-reference/budget-exhausted",
            timestamp("2026-08-15T12:01:00Z"),
        )) {
            Err(error) => error,
            Ok(_) => panic!("exhausted attempt budget must fail"),
        };
        assert_eq!(
            error.code(),
            "authorized_execution_continuity_state.attempt.budget_exhausted"
        );
        assert!(exhausted.store.snapshot() == before);

        let fixture = Fixture::yielded(false);
        let restarted = Fixture {
            store: ReferenceStore::from_state(fixture.store.snapshot()),
            ..fixture.clone()
        };
        let authority = restarted.authority_capability();
        assert!(matches!(
            restarted
                .store
                .consume_directive(consume_request(
                    &restarted,
                    &authority,
                    "consume/after-restart",
                    "attempt/continuity-reference/after-restart",
                    timestamp("2026-08-15T12:01:00Z"),
                ))
                .expect("consume after restart"),
            ConsumeDirectiveResult::Consumed { .. }
        ));

        let fixture = Fixture::yielded(false);
        let authority = fixture.authority_capability();
        let request = consume_request(
            &fixture,
            &authority,
            "consume/committed-before-restart",
            "attempt/continuity-reference/committed-before-restart",
            timestamp("2026-08-15T12:01:00Z"),
        );
        let replay = consume_request(
            &fixture,
            &authority,
            "consume/committed-before-restart",
            "attempt/continuity-reference/committed-before-restart",
            timestamp("2026-08-15T12:01:00Z"),
        );
        fixture.store.inject_fault(InjectedFault::After);
        assert!(fixture.store.consume_directive(request).is_err());
        let restarted_store = ReferenceStore::from_state(fixture.store.snapshot());
        assert!(matches!(
            restarted_store
                .consume_directive(replay)
                .expect("committed replay after restart"),
            ConsumeDirectiveResult::ExactReplay(_)
        ));
    }

    #[test]
    fn wait_requires_exact_wake_capability_before_directive_consumption() {
        let fixture = Fixture::yielded(true);
        let authority = fixture.authority_capability();
        let blocked = consume_request(
            &fixture,
            &authority,
            "consume/blocked",
            "attempt/continuity-reference/2",
            timestamp("2026-08-15T12:01:00Z"),
        );
        let error = match fixture.store.consume_directive(blocked) {
            Err(error) => error,
            Ok(_) => panic!("unsatisfied wait must block"),
        };
        assert_eq!(
            error.code(),
            "authorized_execution_continuity_state.wait.unsatisfied"
        );

        let wake = fixture.wake_capability();
        let operation_id = ContinuityOperationId::new("wait/satisfy").expect("operation");
        let window_revision = fixture
            .store
            .snapshot()
            .windows
            .get(&fixture.window_id)
            .expect("window")
            .revision;
        let mut stale_wake = fixture.wake_capability();
        stale_wake.generation_id =
            ContinuityYieldGenerationId::new("yield/continuity-reference/stale")
                .expect("generation");
        let stale_operation_id = ContinuityOperationId::new("wait/stale-wake").expect("operation");
        let mut stale_transition = TransitionWaitRequest {
            operation_id: stale_operation_id,
            request_commitment: SpecContentHash::from_text("placeholder"),
            receipt_id: ContinuityReceiptId::new("receipt/wait-stale-wake").expect("receipt"),
            window_id: fixture.window_id.clone(),
            expected_window_revision: window_revision,
            expected_window_binding: fixture.window_binding(),
            cursor: fixture.cursor.clone(),
            condition_id: fixture.wait_id.clone().expect("wait"),
            expected_generation_id: fixture.generation_id.clone(),
            expected_condition_version: 1,
            expected_wait_revision: ContinuityRevision::new(1).expect("revision"),
            target: AuthoritativeWaitState::Satisfied,
            wake_capability: Some(&stale_wake),
        };
        stale_transition.request_commitment =
            expected_transition_wait_commitment(&stale_transition);
        let error = match fixture.store.transition_wait(stale_transition) {
            Err(error) => error,
            Ok(_) => panic!("stale-generation wake must fail closed"),
        };
        assert_eq!(
            error.code(),
            "authorized_execution_continuity_state.wake.binding_mismatch"
        );

        let mut transition = TransitionWaitRequest {
            operation_id,
            request_commitment: SpecContentHash::from_text("placeholder"),
            receipt_id: ContinuityReceiptId::new("receipt/wait-satisfy").expect("receipt"),
            window_id: fixture.window_id.clone(),
            expected_window_revision: window_revision,
            expected_window_binding: fixture.window_binding(),
            cursor: fixture.cursor.clone(),
            condition_id: fixture.wait_id.clone().expect("wait"),
            expected_generation_id: fixture.generation_id.clone(),
            expected_condition_version: 1,
            expected_wait_revision: ContinuityRevision::new(1).expect("revision"),
            target: AuthoritativeWaitState::Satisfied,
            wake_capability: Some(&wake),
        };
        transition.request_commitment = expected_transition_wait_commitment(&transition);
        fixture
            .store
            .set_trusted_time(timestamp("2026-08-15T12:01:00Z"));
        assert!(matches!(
            fixture
                .store
                .transition_wait(transition)
                .expect("wait transition"),
            MutationResult::Recorded(_)
        ));

        let stale = consume_request(
            &fixture,
            &authority,
            "consume/stale-after-wake",
            "attempt/continuity-reference/2",
            timestamp("2026-08-15T12:01:01Z"),
        );
        let stale_error = match fixture.store.consume_directive(stale) {
            Err(error) => error,
            Ok(_) => panic!("pre-wake authority must not remain usable"),
        };
        assert_eq!(
            stale_error.code(),
            "authorized_execution_continuity_state.authority.binding_mismatch"
        );

        let fresh_authority = fixture.authority_capability();
        let allowed = consume_request(
            &fixture,
            &fresh_authority,
            "consume/after-wake",
            "attempt/continuity-reference/2",
            timestamp("2026-08-15T12:01:01Z"),
        );
        assert!(matches!(
            fixture
                .store
                .consume_directive(allowed)
                .expect("consume after wake"),
            ConsumeDirectiveResult::Consumed { .. }
        ));
    }

    #[test]
    fn attempt_capability_records_one_outcome_and_recovery_can_only_mark_ambiguity() {
        let fixture = Fixture::yielded(false);
        let authority = fixture.authority_capability();
        let consumed = fixture
            .store
            .consume_directive(consume_request(
                &fixture,
                &authority,
                "consume/outcome",
                "attempt/continuity-reference/2",
                timestamp("2026-08-15T12:01:00Z"),
            ))
            .expect("consume");
        let capability = match consumed {
            ConsumeDirectiveResult::Consumed { capability, .. } => capability,
            ConsumeDirectiveResult::ExactReplay(_) => panic!("first call is not replay"),
        };
        let operation_id = ContinuityOperationId::new("outcome/succeeded").expect("operation");
        let mut outcome = RecordAttemptOutcomeRequest {
            operation_id,
            request_commitment: SpecContentHash::from_text("placeholder"),
            receipt_id: ContinuityReceiptId::new("receipt/outcome-succeeded").expect("receipt"),
            window_id: fixture.window_id.clone(),
            expected_window_revision: capability.window_revision,
            expected_window_binding: fixture.window_binding(),
            attempt_id: capability.attempt_id.clone(),
            expected_attempt_revision: ContinuityRevision::new(1).expect("revision"),
            attempt_capability: &capability,
            outcome: AuthorizedExecutionAttemptOutcome::Succeeded,
        };
        outcome.request_commitment = expected_attempt_outcome_commitment(&outcome);
        fixture
            .store
            .set_trusted_time(timestamp("2026-08-15T12:02:00Z"));
        assert!(matches!(
            fixture
                .store
                .record_attempt_outcome(outcome)
                .expect("outcome"),
            MutationResult::Recorded(_)
        ));
        let snapshot = fixture.store.snapshot();
        assert_eq!(
            snapshot.windows[&fixture.window_id].state,
            AuthoritativeWindowState::Closed
        );

        let ambiguous = Fixture::yielded(false);
        let authority = ambiguous.authority_capability();
        let consumed = ambiguous
            .store
            .consume_directive(consume_request(
                &ambiguous,
                &authority,
                "consume/ambiguous",
                "attempt/continuity-reference/2",
                timestamp("2026-08-15T12:01:00Z"),
            ))
            .expect("consume");
        let (attempt_id, window_revision) = match consumed {
            ConsumeDirectiveResult::Consumed { result, .. } => match result {
                RecordedOperationResult::DirectiveConsumed {
                    attempt_id,
                    window_revision,
                    ..
                } => (attempt_id, window_revision),
                _ => panic!("directive result"),
            },
            ConsumeDirectiveResult::ExactReplay(_) => panic!("first call is not replay"),
        };
        let operation_id = ContinuityOperationId::new("recovery/ambiguous").expect("operation");
        let mut recovery = RecoverAmbiguousAttemptRequest {
            operation_id,
            request_commitment: SpecContentHash::from_text("placeholder"),
            receipt_id: ContinuityReceiptId::new("receipt/recovery-ambiguous").expect("receipt"),
            window_id: ambiguous.window_id.clone(),
            expected_window_revision: window_revision,
            expected_window_binding: ambiguous.window_binding(),
            cursor: ambiguous.cursor.clone(),
            attempt_id,
            expected_attempt_revision: ContinuityRevision::new(1).expect("revision"),
        };
        recovery.request_commitment = expected_recovery_commitment(&recovery);
        ambiguous
            .store
            .set_trusted_time(timestamp("2026-08-15T12:02:00Z"));
        ambiguous
            .store
            .recover_ambiguous_attempt(recovery)
            .expect("recover ambiguity");
        assert_eq!(
            ambiguous.store.snapshot().windows[&ambiguous.window_id].state,
            AuthoritativeWindowState::RecoveryRequired
        );
    }

    #[test]
    fn pre_commit_fault_writes_nothing_and_post_commit_retry_exact_replays() {
        for fault in [InjectedFault::Before, InjectedFault::During] {
            let fixture = Fixture::yielded(false);
            let authority = fixture.authority_capability();
            let before = fixture.store.snapshot();
            fixture.store.inject_fault(fault);
            let request = consume_request(
                &fixture,
                &authority,
                "consume/fault",
                "attempt/continuity-reference/2",
                timestamp("2026-08-15T12:01:00Z"),
            );
            assert!(fixture.store.consume_directive(request).is_err());
            assert_eq!(fixture.store.snapshot().windows, before.windows);
            assert_eq!(fixture.store.snapshot().attempts, before.attempts);
            assert!(fixture.store.snapshot().operations.is_empty());
        }

        let fixture = Fixture::yielded(false);
        let authority = fixture.authority_capability();
        fixture.store.inject_fault(InjectedFault::After);
        let request = consume_request(
            &fixture,
            &authority,
            "consume/fault",
            "attempt/continuity-reference/2",
            timestamp("2026-08-15T12:01:00Z"),
        );
        let replay = consume_request(
            &fixture,
            &authority,
            "consume/fault",
            "attempt/continuity-reference/2",
            timestamp("2026-08-15T12:01:00Z"),
        );
        assert!(fixture.store.consume_directive(request).is_err());
        assert!(matches!(
            fixture.store.consume_directive(replay).expect("replay"),
            ConsumeDirectiveResult::ExactReplay(_)
        ));
    }

    #[test]
    fn every_continuity_operation_obeys_before_during_and_after_commit_fault_posture() {
        for fault in [
            InjectedFault::Before,
            InjectedFault::During,
            InjectedFault::After,
        ] {
            let fixture = Fixture::yielded(false);
            let authority = fixture.authority_capability();
            let request = consume_request(
                &fixture,
                &authority,
                "consume/fault-matrix",
                "attempt/continuity-reference/fault-matrix",
                timestamp("2026-08-15T12:01:00Z"),
            );
            let replay = consume_request(
                &fixture,
                &authority,
                "consume/fault-matrix",
                "attempt/continuity-reference/fault-matrix",
                timestamp("2026-08-15T12:01:00Z"),
            );
            let before = fixture.store.snapshot();
            fixture.store.inject_fault(fault);
            assert!(fixture.store.consume_directive(request).is_err());
            if fault == InjectedFault::After {
                assert!(matches!(
                    fixture.store.consume_directive(replay).expect("replay"),
                    ConsumeDirectiveResult::ExactReplay(_)
                ));
            } else {
                assert!(fixture.store.snapshot() == before);
            }

            let fixture = Fixture::yielded(true);
            let wake = fixture.wake_capability();
            let request = transition_wait_request(&fixture, &wake, "wait/fault-matrix");
            let replay = transition_wait_request(&fixture, &wake, "wait/fault-matrix");
            let before = fixture.store.snapshot();
            fixture.store.inject_fault(fault);
            assert!(fixture.store.transition_wait(request).is_err());
            if fault == InjectedFault::After {
                assert!(matches!(
                    fixture.store.transition_wait(replay).expect("replay"),
                    MutationResult::ExactReplay(_)
                ));
            } else {
                assert!(fixture.store.snapshot() == before);
            }

            let fixture = Fixture::yielded(false);
            let authority = fixture.authority_capability();
            let consumed = fixture
                .store
                .consume_directive(consume_request(
                    &fixture,
                    &authority,
                    "consume/outcome-fault-matrix",
                    "attempt/continuity-reference/outcome-fault-matrix",
                    timestamp("2026-08-15T12:01:00Z"),
                ))
                .expect("consume");
            let capability = match consumed {
                ConsumeDirectiveResult::Consumed { capability, .. } => capability,
                ConsumeDirectiveResult::ExactReplay(_) => panic!("first call is not replay"),
            };
            let request = attempt_outcome_request(&fixture, &capability, "outcome/fault-matrix");
            let replay = attempt_outcome_request(&fixture, &capability, "outcome/fault-matrix");
            let before = fixture.store.snapshot();
            fixture.store.inject_fault(fault);
            assert!(fixture.store.record_attempt_outcome(request).is_err());
            if fault == InjectedFault::After {
                assert!(matches!(
                    fixture
                        .store
                        .record_attempt_outcome(replay)
                        .expect("replay"),
                    MutationResult::ExactReplay(_)
                ));
            } else {
                assert!(fixture.store.snapshot() == before);
            }

            let fixture = Fixture::yielded(false);
            let authority = fixture.authority_capability();
            let consumed = fixture
                .store
                .consume_directive(consume_request(
                    &fixture,
                    &authority,
                    "consume/recovery-fault-matrix",
                    "attempt/continuity-reference/recovery-fault-matrix",
                    timestamp("2026-08-15T12:01:00Z"),
                ))
                .expect("consume");
            let attempt_id = match consumed {
                ConsumeDirectiveResult::Consumed { result, .. } => match result {
                    RecordedOperationResult::DirectiveConsumed { attempt_id, .. } => attempt_id,
                    _ => panic!("directive result"),
                },
                ConsumeDirectiveResult::ExactReplay(_) => panic!("first call is not replay"),
            };
            let request = recovery_request(&fixture, attempt_id.clone(), "recovery/fault-matrix");
            let replay = recovery_request(&fixture, attempt_id, "recovery/fault-matrix");
            let before = fixture.store.snapshot();
            fixture.store.inject_fault(fault);
            assert!(fixture.store.recover_ambiguous_attempt(request).is_err());
            if fault == InjectedFault::After {
                assert!(matches!(
                    fixture
                        .store
                        .recover_ambiguous_attempt(replay)
                        .expect("replay"),
                    MutationResult::ExactReplay(_)
                ));
            } else {
                assert!(fixture.store.snapshot() == before);
            }

            let fixture = Fixture::yielded(false);
            let authority = fixture.authority_capability();
            let consumed = fixture
                .store
                .consume_directive(consume_request(
                    &fixture,
                    &authority,
                    "consume/yield-fault-matrix",
                    "attempt/continuity-reference/yield-fault-matrix",
                    timestamp("2026-08-15T12:01:00Z"),
                ))
                .expect("consume");
            let capability = match consumed {
                ConsumeDirectiveResult::Consumed { capability, .. } => capability,
                ConsumeDirectiveResult::ExactReplay(_) => panic!("first call is not replay"),
            };
            let request = register_yield_request(
                &fixture,
                &capability,
                "yield/fault-matrix",
                "yield/continuity-reference/fault-matrix",
            );
            let replay = register_yield_request(
                &fixture,
                &capability,
                "yield/fault-matrix",
                "yield/continuity-reference/fault-matrix",
            );
            let before = fixture.store.snapshot();
            fixture.store.inject_fault(fault);
            assert!(fixture.store.register_yield(request).is_err());
            if fault == InjectedFault::After {
                assert!(matches!(
                    fixture.store.register_yield(replay).expect("replay"),
                    RegisterYieldResult::ExactReplay(_)
                ));
            } else {
                assert!(fixture.store.snapshot() == before);
            }
        }
    }

    #[test]
    fn conflicting_replay_and_receipt_reuse_fail_closed() {
        let fixture = Fixture::yielded(false);
        let authority = fixture.authority_capability();
        let first = consume_request(
            &fixture,
            &authority,
            "consume/replay-binding",
            "attempt/continuity-reference/replay-binding",
            timestamp("2026-08-15T12:01:00Z"),
        );
        fixture
            .store
            .consume_directive(first)
            .expect("initial consume");
        let before = fixture.store.snapshot();

        let mut conflicting = consume_request(
            &fixture,
            &authority,
            "consume/replay-binding",
            "attempt/continuity-reference/replay-binding",
            timestamp("2026-08-15T12:01:00Z"),
        );
        conflicting.receipt_id =
            ContinuityReceiptId::new("receipt/different-replay").expect("receipt");
        conflicting.request_commitment = expected_consume_directive_commitment(&conflicting);
        let error = match fixture.store.consume_directive(conflicting) {
            Err(error) => error,
            Ok(_) => panic!("changed replay receipt must conflict"),
        };
        assert_eq!(
            error.code(),
            "authorized_execution_continuity_state.operation.replay_conflict"
        );
        assert_eq!(fixture.store.snapshot().windows, before.windows);
        assert_eq!(fixture.store.snapshot().attempts, before.attempts);
        let after = fixture.store.snapshot();
        assert_eq!(after.operations.len(), before.operations.len());
        assert!(after.operations.keys().eq(before.operations.keys()));

        let mut reused_receipt = consume_request(
            &fixture,
            &authority,
            "consume/reused-receipt",
            "attempt/continuity-reference/reused-receipt",
            timestamp("2026-08-15T12:01:00Z"),
        );
        reused_receipt.receipt_id =
            ContinuityReceiptId::new("receipt/consume/replay-binding").expect("receipt");
        reused_receipt.request_commitment = expected_consume_directive_commitment(&reused_receipt);
        let error = match fixture.store.consume_directive(reused_receipt) {
            Err(error) => error,
            Ok(_) => panic!("receipt reuse across operations must fail"),
        };
        assert_eq!(
            error.code(),
            "authorized_execution_continuity_state.receipt.reused"
        );
    }

    #[test]
    fn yield_registration_closes_attempt_and_canonicalizes_wait_order() {
        let fixture = Fixture::yielded(false);
        let authority = fixture.authority_capability();
        let consumed = fixture
            .store
            .consume_directive(consume_request(
                &fixture,
                &authority,
                "consume/yield",
                "attempt/continuity-reference/2",
                timestamp("2026-08-15T12:01:00Z"),
            ))
            .expect("consume");
        let capability = match consumed {
            ConsumeDirectiveResult::Consumed { capability, .. } => capability,
            ConsumeDirectiveResult::ExactReplay(_) => panic!("first call is not replay"),
        };
        let operation_id = ContinuityOperationId::new("yield/next").expect("operation");
        let waits = vec![
            SeedWait {
                condition_id: AuthorizedExecutionWaitConditionId::new("wait/z").expect("wait"),
                condition_version: 1,
                wake_trigger: AuthorizedExecutionWakeTriggerKind::EvidenceAccepted,
            },
            SeedWait {
                condition_id: AuthorizedExecutionWaitConditionId::new("wait/a").expect("wait"),
                condition_version: 1,
                wake_trigger: AuthorizedExecutionWakeTriggerKind::CheckAccepted,
            },
        ];
        let mut request = RegisterYieldRequest {
            operation_id,
            request_commitment: SpecContentHash::from_text("placeholder"),
            receipt_id: ContinuityReceiptId::new("receipt/yield-next").expect("receipt"),
            generation_id: ContinuityYieldGenerationId::new("yield/continuity-reference/2")
                .expect("generation"),
            window_id: fixture.window_id.clone(),
            expected_window_revision: capability.window_revision,
            expected_window_binding: fixture.window_binding(),
            cursor: fixture.cursor.clone(),
            attempt_id: capability.attempt_id.clone(),
            attempt_capability: &capability,
            reason: AuthorizedExecutionYieldReason::TurnBoundary,
            waits,
        };
        request.request_commitment = expected_register_yield_commitment(&request);
        fixture
            .store
            .set_trusted_time(timestamp("2026-08-15T12:02:00Z"));
        assert!(matches!(
            fixture.store.register_yield(request).expect("yield"),
            RegisterYieldResult::Registered(_)
        ));
        let snapshot = fixture.store.snapshot();
        assert_eq!(
            snapshot.attempts[&capability.attempt_id].state,
            AuthoritativeAttemptState::Yielded
        );
        assert_eq!(
            snapshot.windows[&fixture.window_id].state,
            AuthoritativeWindowState::Yielded
        );
    }

    #[test]
    fn trusted_time_regression_and_expiry_fail_closed_without_writes() {
        for observed_at in [
            timestamp("2026-08-15T11:59:59Z"),
            timestamp("2026-08-15T13:00:00Z"),
        ] {
            let fixture = Fixture::yielded(false);
            let authority = fixture.authority_capability();
            let before = fixture.store.snapshot();
            let request = consume_request(
                &fixture,
                &authority,
                "consume/time",
                "attempt/continuity-reference/2",
                observed_at,
            );
            let error = match fixture.store.consume_directive(request) {
                Err(error) => error,
                Ok(_) => panic!("invalid trusted time must fail closed"),
            };
            assert!(matches!(
                error.code(),
                "authorized_execution_continuity_state.time.regressed"
                    | "authorized_execution_continuity_state.time.expired"
            ));
            assert_eq!(fixture.store.snapshot().windows, before.windows);
            assert_eq!(fixture.store.snapshot().attempts, before.attempts);
        }
    }

    #[test]
    fn unavailable_or_incompatible_trusted_clock_fails_closed_without_writes() {
        for incompatible_provenance in [false, true] {
            let fixture = Fixture::yielded(false);
            let authority = fixture.authority_capability();
            let before = fixture.store.snapshot();
            if incompatible_provenance {
                fixture
                    .store
                    .set_clock_provenance(SpecContentHash::from_text("untrusted clock"));
            } else {
                fixture.store.set_clock_available(false);
            }
            let request = consume_request(
                &fixture,
                &authority,
                "consume/untrusted-clock",
                "attempt/continuity-reference/untrusted-clock",
                timestamp("2026-08-15T12:01:00Z"),
            );
            let error = match fixture.store.consume_directive(request) {
                Err(error) => error,
                Ok(_) => panic!("untrusted clock must fail closed"),
            };
            assert!(matches!(
                error.code(),
                "authorized_execution_continuity_state.time.unavailable"
                    | "authorized_execution_continuity_state.time.untrusted"
            ));
            assert_eq!(fixture.store.snapshot().windows, before.windows);
            assert_eq!(fixture.store.snapshot().attempts, before.attempts);
            assert!(fixture.store.snapshot().operations.is_empty());
        }
    }

    #[test]
    fn recovery_rejects_cross_run_and_stale_cursor_bindings_without_writes() {
        let fixture = Fixture::yielded(false);
        let authority = fixture.authority_capability();
        let consumed = fixture
            .store
            .consume_directive(consume_request(
                &fixture,
                &authority,
                "consume/recovery-binding",
                "attempt/continuity-reference/recovery-binding",
                timestamp("2026-08-15T12:01:00Z"),
            ))
            .expect("consume");
        let (attempt_id, window_revision) = match consumed {
            ConsumeDirectiveResult::Consumed { result, .. } => match result {
                RecordedOperationResult::DirectiveConsumed {
                    attempt_id,
                    window_revision,
                    ..
                } => (attempt_id, window_revision),
                _ => panic!("directive result"),
            },
            ConsumeDirectiveResult::ExactReplay(_) => panic!("first call is not replay"),
        };
        let before = fixture.store.snapshot();
        let operation_id = ContinuityOperationId::new("recovery/cross-run").expect("operation");
        let mut binding = fixture.window_binding();
        binding.run_id = WorkflowRunId::new("run/other-context").expect("run");
        let stale_cursor = ContinuityCursor {
            sequence_number: fixture.cursor.sequence_number,
            event_id: EventId::new("event/other-context").expect("event"),
        };
        let mut request = RecoverAmbiguousAttemptRequest {
            operation_id,
            request_commitment: SpecContentHash::from_text("placeholder"),
            receipt_id: ContinuityReceiptId::new("receipt/recovery-cross-run").expect("receipt"),
            window_id: fixture.window_id.clone(),
            expected_window_revision: window_revision,
            expected_window_binding: binding,
            cursor: stale_cursor,
            attempt_id,
            expected_attempt_revision: ContinuityRevision::new(1).expect("revision"),
        };
        request.request_commitment = expected_recovery_commitment(&request);
        fixture
            .store
            .set_trusted_time(timestamp("2026-08-15T12:02:00Z"));
        let error = match fixture.store.recover_ambiguous_attempt(request) {
            Err(error) => error,
            Ok(_) => panic!("cross-run recovery must fail closed"),
        };
        assert_eq!(
            error.code(),
            "authorized_execution_continuity_state.window.binding_mismatch"
        );
        assert_eq!(fixture.store.snapshot().windows, before.windows);
        assert_eq!(fixture.store.snapshot().attempts, before.attempts);
    }

    #[test]
    fn operation_ids_and_capability_debug_reject_secret_like_values() {
        let error = ContinuityOperationId::new("operation/token-value")
            .expect_err("secret-like operation id rejected");
        assert_eq!(
            error.code(),
            "authorized_execution_continuity_state.input.invalid"
        );
        assert!(!error.to_string().contains("token-value"));

        let fixture = Fixture::yielded(false);
        let capability = fixture.authority_capability();
        let debug = format!("{capability:?}");
        assert!(debug.contains("[REDACTED]"));
        assert!(!debug.contains("continuity-reference"));
    }
}
