use std::error::Error;
use std::fmt;

use crate::{WorkflowOsError, WorkflowOsErrorKind};

use super::trusted_host_explicit_local_operation::TrustedHostExplicitLocalOperationOutcome;
use super::trusted_host_explicit_local_process_owner::{
    TrustedHostExplicitLocalProcessOwner, TrustedHostExplicitLocalProcessOwnerOutcome,
};
use super::trusted_host_local_production_caller::TrustedHostLocalProductionIdentitySource;
use super::trusted_host_local_timer::{
    TrustedHostLocalTimerCancellation, TrustedHostLocalTimerCancellationHandle,
};
use super::trusted_host_operational_entry::{
    validate_trusted_host_operational_entry_preparation, TrustedHostOperationalEntryInput,
    TrustedHostOperationalEntryLocator, TrustedHostOperationalEntryPosture,
};
use super::trusted_host_redispatch_loop::TrustedHostRedispatchStopReason;
use super::trusted_host_time_window_scheduling::TrustedHostRepeatedSchedulingStopReason;
use super::SqliteStateBackend;
use crate::operational_execution_window_opening::OperationalExecutionWindowOpeningUseInput;
use crate::trusted_host_supervisor::{
    TrustedHostAttemptExecutor, TrustedHostSupervisorPersistenceInput,
};
use crate::SkillInput;

type SessionRunner<'a> =
    Box<dyn FnOnce() -> Result<TrustedHostLocalApplicationOutcome, WorkflowOsError> + 'a>;

/// Opaque, one-shot authority to run one Core-issued local trusted-host operation.
///
/// The session has no public constructor. Enabling the feature does not grant
/// authority; a future reviewed Core preparation boundary must issue it.
///
/// Sessions cannot be cloned:
///
/// ```compile_fail
/// fn duplicate(session: workflow_core::TrustedHostLocalApplicationSession<'_>) {
///     let _copy = session.clone();
/// }
/// ```
///
/// Sessions cannot be serialized:
///
/// ```compile_fail
/// fn serialize(session: &workflow_core::TrustedHostLocalApplicationSession<'_>) {
///     let _json = serde_json::to_string(session).unwrap();
/// }
/// ```
pub struct TrustedHostLocalApplicationSession<'a> {
    runner: SessionRunner<'a>,
}

impl TrustedHostLocalApplicationSession<'_> {
    /// Consumes the session and runs its single bound operation.
    ///
    /// # Errors
    ///
    /// Returns a bounded application failure projected from the private Core
    /// error. Messages, diagnostics, and source values do not cross the SPI.
    pub fn run(
        self,
    ) -> Result<TrustedHostLocalApplicationOutcome, TrustedHostLocalApplicationFailure> {
        (self.runner)().map_err(|error| TrustedHostLocalApplicationFailure::from_core_error(&error))
    }
}

#[cfg(test)]
#[allow(clippy::elidable_lifetime_names)]
impl<'a> TrustedHostLocalApplicationSession<'a> {
    pub(crate) fn from_process_owner(owner: TrustedHostExplicitLocalProcessOwner<'a>) -> Self {
        Self {
            runner: Box::new(move || owner.run().map(map_owner_outcome)),
        }
    }

    fn from_test_runner(runner: SessionRunner<'a>) -> Self {
        Self { runner }
    }
}

impl fmt::Debug for TrustedHostLocalApplicationSession<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostLocalApplicationSession")
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

/// Cooperative cancellation custody paired with a Core-issued session.
///
/// This handle cannot be constructed outside Core and does not interrupt an
/// already admitted executor attempt.
#[derive(Clone)]
pub struct TrustedHostLocalApplicationCancellationHandle {
    inner: TrustedHostLocalTimerCancellationHandle,
}

impl TrustedHostLocalApplicationCancellationHandle {
    pub(crate) fn from_timer_handle(inner: TrustedHostLocalTimerCancellationHandle) -> Self {
        Self { inner }
    }

    /// Requests cooperative cancellation at the accepted local-owner boundary.
    ///
    /// # Errors
    ///
    /// Returns a bounded failure if the private cancellation state cannot be read.
    pub fn request_cancellation(&self) -> Result<(), TrustedHostLocalApplicationFailure> {
        self.inner
            .cancel()
            .map_err(|error| TrustedHostLocalApplicationFailure::from_core_error(&error))
    }
}

impl fmt::Debug for TrustedHostLocalApplicationCancellationHandle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostLocalApplicationCancellationHandle")
            .field("state", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

/// Fixed, payload-free failure projected at the local application boundary.
///
/// The failure intentionally retains only the stable Core error category. It
/// does not store or expose the original code, message, diagnostics, source,
/// path, identifier, payload, or serialized error.
///
/// Failures cannot be serialized:
///
/// ```compile_fail
/// fn serialize(failure: &workflow_core::TrustedHostLocalApplicationFailure) {
///     let _json = serde_json::to_string(failure).unwrap();
/// }
/// ```
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum TrustedHostLocalApplicationFailure {
    /// Private Core input could not be parsed.
    Parse,
    /// Private Core input failed validation.
    Validation,
    /// The private operation is unsupported.
    Unsupported,
    /// Policy denied the private operation.
    PolicyDenied,
    /// Current private Core state is invalid or ambiguous.
    InvalidState,
    /// A private security boundary rejected the operation.
    Security,
    /// A private internal invariant failed.
    Internal,
}

impl TrustedHostLocalApplicationFailure {
    fn from_core_error(error: &WorkflowOsError) -> Self {
        match error.kind() {
            WorkflowOsErrorKind::Parse => Self::Parse,
            WorkflowOsErrorKind::Validation => Self::Validation,
            WorkflowOsErrorKind::Unsupported => Self::Unsupported,
            WorkflowOsErrorKind::PolicyDenied => Self::PolicyDenied,
            WorkflowOsErrorKind::InvalidState => Self::InvalidState,
            WorkflowOsErrorKind::Security => Self::Security,
            WorkflowOsErrorKind::Internal => Self::Internal,
        }
    }

    /// Returns the fixed stable application failure code.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::Parse => "trusted_host_local_application.failure.parse",
            Self::Validation => "trusted_host_local_application.failure.validation",
            Self::Unsupported => "trusted_host_local_application.failure.unsupported",
            Self::PolicyDenied => "trusted_host_local_application.failure.policy_denied",
            Self::InvalidState => "trusted_host_local_application.failure.invalid_state",
            Self::Security => "trusted_host_local_application.failure.security",
            Self::Internal => "trusted_host_local_application.failure.internal",
        }
    }
}

impl fmt::Debug for TrustedHostLocalApplicationFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("TrustedHostLocalApplicationFailure")
            .field(&self.code())
            .finish()
    }
}

impl fmt::Display for TrustedHostLocalApplicationFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

impl Error for TrustedHostLocalApplicationFailure {}

pub(crate) enum TrustedHostLocalApplicationPreparationPosture<'a> {
    Fresh {
        opening: Box<OperationalExecutionWindowOpeningUseInput<'a>>,
        persistence: TrustedHostSupervisorPersistenceInput,
    },
    Existing,
}

pub(crate) struct TrustedHostLocalApplicationPreparationInput<'a> {
    pub(crate) backend: &'a SqliteStateBackend,
    pub(crate) locator: TrustedHostOperationalEntryLocator,
    pub(crate) posture: TrustedHostLocalApplicationPreparationPosture<'a>,
    pub(crate) executor: &'a dyn TrustedHostAttemptExecutor,
    pub(crate) skill_input: SkillInput,
}

/// Opaque Core-issued session and cancellation pair for the unstable local-host SPI.
///
/// The pair has no public constructor. Enabling the feature permits a reviewed
/// consumer to receive and consume a prepared pair, but does not grant authority
/// to prepare one.
///
/// Prepared pairs cannot be cloned:
///
/// ```compile_fail
/// fn duplicate(pair: workflow_core::TrustedHostLocalApplicationPreparedSession<'_>) {
///     let _copy = pair.clone();
/// }
/// ```
///
/// Prepared pairs cannot be serialized:
///
/// ```compile_fail
/// fn serialize(pair: &workflow_core::TrustedHostLocalApplicationPreparedSession<'_>) {
///     let _json = serde_json::to_string(pair).unwrap();
/// }
/// ```
#[doc(hidden)]
pub struct TrustedHostLocalApplicationPreparedSession<'a> {
    session: TrustedHostLocalApplicationSession<'a>,
    cancellation_handle: TrustedHostLocalApplicationCancellationHandle,
}

impl<'a> TrustedHostLocalApplicationPreparedSession<'a> {
    /// Consumes the unforgeable pair into its one-shot session and scoped
    /// cooperative-cancellation handle.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        TrustedHostLocalApplicationSession<'a>,
        TrustedHostLocalApplicationCancellationHandle,
    ) {
        (self.session, self.cancellation_handle)
    }
}

impl fmt::Debug for TrustedHostLocalApplicationPreparedSession<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostLocalApplicationPreparedSession")
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

pub(crate) fn prepare_trusted_host_local_application_session(
    input: TrustedHostLocalApplicationPreparationInput<'_>,
) -> Result<TrustedHostLocalApplicationPreparedSession<'_>, WorkflowOsError> {
    let TrustedHostLocalApplicationPreparationInput {
        backend,
        locator,
        posture,
        executor,
        skill_input,
    } = input;
    let posture = match posture {
        TrustedHostLocalApplicationPreparationPosture::Fresh {
            opening,
            persistence,
        } => TrustedHostOperationalEntryPosture::Fresh {
            opening,
            persistence,
        },
        TrustedHostLocalApplicationPreparationPosture::Existing => {
            TrustedHostOperationalEntryPosture::Existing
        }
    };
    let preparation_binding = validate_trusted_host_operational_entry_preparation(
        backend,
        &locator,
        &posture,
        executor,
        &skill_input,
    )?;
    let (cancellation, timer_handle) = TrustedHostLocalTimerCancellation::new();
    let session = TrustedHostLocalApplicationSession {
        runner: Box::new(move || {
            let source = TrustedHostLocalProductionIdentitySource::new();
            let mut identity_provider = source.redispatch_provider();
            TrustedHostExplicitLocalProcessOwner::with_cancellation(
                TrustedHostOperationalEntryInput {
                    backend,
                    locator,
                    posture,
                    executor,
                    skill_input,
                    identity_provider: &mut identity_provider,
                    expected_preparation_commitment: Some(preparation_binding.commitment),
                },
                cancellation,
            )
            .run()
            .map(map_owner_outcome)
        }),
    };
    Ok(TrustedHostLocalApplicationPreparedSession {
        session,
        cancellation_handle: TrustedHostLocalApplicationCancellationHandle::from_timer_handle(
            timer_handle,
        ),
    })
}

/// Bounded result of consuming one local application session.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum TrustedHostLocalApplicationOutcome {
    /// Cancellation won before executor entry and no owner work was admitted.
    CanceledBeforeEntry,
    /// Initial entry stopped with one bounded classification.
    EntryStopped(TrustedHostLocalApplicationEntryStopReason),
    /// Timer-driven continuation stopped with one bounded classification.
    ContinuationStopped(TrustedHostLocalApplicationContinuationStopReason),
}

impl fmt::Debug for TrustedHostLocalApplicationOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CanceledBeforeEntry => formatter
                .debug_tuple("TrustedHostLocalApplicationOutcome::CanceledBeforeEntry")
                .finish(),
            Self::EntryStopped(reason) => formatter
                .debug_tuple("TrustedHostLocalApplicationOutcome::EntryStopped")
                .field(reason)
                .finish(),
            Self::ContinuationStopped(reason) => formatter
                .debug_tuple("TrustedHostLocalApplicationOutcome::ContinuationStopped")
                .field(reason)
                .finish(),
        }
    }
}

/// Fixed entry-stop vocabulary that does not expose workflow state or payloads.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrustedHostLocalApplicationEntryStopReason {
    /// The operation yielded a supported wait and did not reach continuation.
    AwaitCondition,
    /// Current authoritative state blocked executor admission.
    Blocked,
    /// Current authoritative state was already terminal.
    Terminal,
}

/// Fixed continuation-stop vocabulary that does not expose scheduling inputs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrustedHostLocalApplicationContinuationStopReason {
    /// Cooperative cancellation stopped continuation scheduling.
    Canceled,
    /// Current authoritative state blocked further continuation.
    Blocked,
    /// Current authoritative state reached a terminal posture.
    Terminal,
    /// The requested wait kind is outside the local timer contract.
    UnsupportedWait,
    /// The bounded wake budget was exhausted without a terminal posture.
    WakeBudgetExhausted,
}

fn map_owner_outcome(
    outcome: TrustedHostExplicitLocalProcessOwnerOutcome,
) -> TrustedHostLocalApplicationOutcome {
    match outcome {
        TrustedHostExplicitLocalProcessOwnerOutcome::CanceledBeforeEntry => {
            TrustedHostLocalApplicationOutcome::CanceledBeforeEntry
        }
        TrustedHostExplicitLocalProcessOwnerOutcome::OperationStopped(
            TrustedHostExplicitLocalOperationOutcome::EntryStopped(outcome),
        ) => TrustedHostLocalApplicationOutcome::EntryStopped(map_entry_stop_reason(
            outcome.stop_reason,
        )),
        TrustedHostExplicitLocalProcessOwnerOutcome::OperationStopped(
            TrustedHostExplicitLocalOperationOutcome::ContinuationStopped(outcome),
        ) => TrustedHostLocalApplicationOutcome::ContinuationStopped(map_continuation_stop_reason(
            outcome.stop_reason,
        )),
    }
}

fn map_entry_stop_reason(
    reason: TrustedHostRedispatchStopReason,
) -> TrustedHostLocalApplicationEntryStopReason {
    match reason {
        TrustedHostRedispatchStopReason::AwaitCondition => {
            TrustedHostLocalApplicationEntryStopReason::AwaitCondition
        }
        TrustedHostRedispatchStopReason::Blocked => {
            TrustedHostLocalApplicationEntryStopReason::Blocked
        }
        TrustedHostRedispatchStopReason::Terminal => {
            TrustedHostLocalApplicationEntryStopReason::Terminal
        }
    }
}

fn map_continuation_stop_reason(
    reason: TrustedHostRepeatedSchedulingStopReason,
) -> TrustedHostLocalApplicationContinuationStopReason {
    match reason {
        TrustedHostRepeatedSchedulingStopReason::Canceled => {
            TrustedHostLocalApplicationContinuationStopReason::Canceled
        }
        TrustedHostRepeatedSchedulingStopReason::Blocked => {
            TrustedHostLocalApplicationContinuationStopReason::Blocked
        }
        TrustedHostRepeatedSchedulingStopReason::Terminal => {
            TrustedHostLocalApplicationContinuationStopReason::Terminal
        }
        TrustedHostRepeatedSchedulingStopReason::UnsupportedWait => {
            TrustedHostLocalApplicationContinuationStopReason::UnsupportedWait
        }
        TrustedHostRepeatedSchedulingStopReason::WakeBudgetExhausted => {
            TrustedHostLocalApplicationContinuationStopReason::WakeBudgetExhausted
        }
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;
    use crate::sqlite_state::trusted_host_local_timer::TrustedHostLocalTimerCancellation;

    #[test]
    fn session_consumes_one_runner_and_returns_bounded_outcome() {
        let calls = AtomicUsize::new(0);
        let session = TrustedHostLocalApplicationSession::from_test_runner(Box::new(|| {
            calls.fetch_add(1, Ordering::Relaxed);
            Ok(TrustedHostLocalApplicationOutcome::EntryStopped(
                TrustedHostLocalApplicationEntryStopReason::Terminal,
            ))
        }));

        assert_eq!(
            session.run().expect("session outcome"),
            TrustedHostLocalApplicationOutcome::EntryStopped(
                TrustedHostLocalApplicationEntryStopReason::Terminal
            )
        );
        assert_eq!(calls.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn session_and_cancellation_debug_hide_bound_state() {
        let secret = "token-private-run-window";
        let session = TrustedHostLocalApplicationSession::from_test_runner(Box::new(|| {
            Ok(TrustedHostLocalApplicationOutcome::CanceledBeforeEntry)
        }));
        let (_cancellation, inner) = TrustedHostLocalTimerCancellation::new();
        let handle = TrustedHostLocalApplicationCancellationHandle::from_timer_handle(inner);

        for debug in [format!("{session:?}"), format!("{handle:?}")] {
            assert!(debug.contains("[REDACTED]"));
            assert!(!debug.contains(secret));
        }
    }

    #[test]
    fn cancellation_handle_preserves_idempotent_cooperative_request() {
        let (cancellation, inner) = TrustedHostLocalTimerCancellation::new();
        let handle = TrustedHostLocalApplicationCancellationHandle::from_timer_handle(inner);

        handle.request_cancellation().expect("first cancellation");
        handle.request_cancellation().expect("second cancellation");
        assert_eq!(
            cancellation.begin_entry().expect("entry decision"),
            super::super::trusted_host_local_timer::TrustedHostLocalTimerEntryDecision::Canceled
        );
    }

    #[test]
    fn cloned_cancellation_handles_share_one_scoped_cancellation_state() {
        let (cancellation, inner) = TrustedHostLocalTimerCancellation::new();
        let handle = TrustedHostLocalApplicationCancellationHandle::from_timer_handle(inner);
        let clone = handle.clone();

        clone.request_cancellation().expect("cloned cancellation");
        handle
            .request_cancellation()
            .expect("original cancellation");

        assert_eq!(
            cancellation.begin_entry().expect("entry decision"),
            super::super::trusted_host_local_timer::TrustedHostLocalTimerEntryDecision::Canceled
        );
    }

    #[test]
    fn cancellation_failure_is_bounded_and_payload_free() {
        let (_cancellation, inner) = TrustedHostLocalTimerCancellation::new();
        inner.poison();
        let handle = TrustedHostLocalApplicationCancellationHandle::from_timer_handle(inner);

        let failure = handle
            .request_cancellation()
            .expect_err("poisoned cancellation state must fail");

        assert_eq!(failure, TrustedHostLocalApplicationFailure::Internal);
        for output in [format!("{failure:?}"), failure.to_string()] {
            assert!(output.contains(failure.code()));
            assert!(!output.contains("poison"));
            assert!(!output.contains("token"));
            assert!(!output.contains("authorization"));
        }
    }

    #[test]
    fn outcome_debug_contains_only_fixed_classification() {
        let outcomes = [
            TrustedHostLocalApplicationOutcome::CanceledBeforeEntry,
            TrustedHostLocalApplicationOutcome::EntryStopped(
                TrustedHostLocalApplicationEntryStopReason::Blocked,
            ),
            TrustedHostLocalApplicationOutcome::ContinuationStopped(
                TrustedHostLocalApplicationContinuationStopReason::UnsupportedWait,
            ),
        ];

        for outcome in outcomes {
            let debug = format!("{outcome:?}");
            assert!(debug.starts_with("TrustedHostLocalApplicationOutcome::"));
            assert!(!debug.contains("workflow/"));
            assert!(!debug.contains("run/"));
            assert!(!debug.contains("token"));
        }
    }

    #[test]
    fn internal_error_projection_is_exhaustive_and_payload_free() {
        let cases = [
            (
                WorkflowOsErrorKind::Parse,
                TrustedHostLocalApplicationFailure::Parse,
            ),
            (
                WorkflowOsErrorKind::Validation,
                TrustedHostLocalApplicationFailure::Validation,
            ),
            (
                WorkflowOsErrorKind::Unsupported,
                TrustedHostLocalApplicationFailure::Unsupported,
            ),
            (
                WorkflowOsErrorKind::PolicyDenied,
                TrustedHostLocalApplicationFailure::PolicyDenied,
            ),
            (
                WorkflowOsErrorKind::InvalidState,
                TrustedHostLocalApplicationFailure::InvalidState,
            ),
            (
                WorkflowOsErrorKind::Security,
                TrustedHostLocalApplicationFailure::Security,
            ),
            (
                WorkflowOsErrorKind::Internal,
                TrustedHostLocalApplicationFailure::Internal,
            ),
        ];

        for (kind, expected) in cases {
            let session =
                TrustedHostLocalApplicationSession::from_test_runner(Box::new(move || {
                    Err(WorkflowOsError::new(
                        kind,
                        "private.token.path",
                        "authorization bearer secret-private-diagnostic",
                    ))
                }));
            let failure = session.run().expect_err("private error must fail");

            assert_eq!(failure, expected);
            for output in [format!("{failure:?}"), failure.to_string()] {
                assert!(output.contains(failure.code()));
                assert!(!output.contains("private"));
                assert!(!output.contains("authorization"));
                assert!(!output.contains("bearer"));
                assert!(!output.contains("secret"));
                assert!(!output.contains("diagnostic"));
            }
        }
    }
}
