use std::fmt;
use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;

use crate::trusted_host_supervisor::TrustedHostAttemptExecutor;
use crate::{SkillInput, Timestamp, WorkflowOsError, WorkflowOsErrorKind};

use super::trusted_host_operational_entry::TrustedHostOperationalEntryLocator;
use super::trusted_host_redispatch_loop::TrustedHostRedispatchIdentityProvider;
use super::trusted_host_time_window_scheduling::{
    run_bounded_trusted_host_repeated_scheduling, TrustedHostDeadlineWaitFailure,
    TrustedHostDeadlineWaitOutcome, TrustedHostDeadlineWaiter, TrustedHostRepeatedSchedulingInput,
    TrustedHostRepeatedSchedulingOutcome, TrustedHostRepeatedWakeBudget,
    TrustedHostScheduleWakeIdentityProvider,
};
use super::SqliteStateBackend;

struct CancellationState {
    canceled: bool,
    entry_started: bool,
}

struct SharedCancellation {
    state: Mutex<CancellationState>,
    signal: Condvar,
}

pub(crate) struct TrustedHostLocalTimerCancellation {
    shared: Arc<SharedCancellation>,
}

#[derive(Clone)]
pub(crate) struct TrustedHostLocalTimerCancellationHandle {
    shared: Arc<SharedCancellation>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TrustedHostLocalTimerEntryDecision {
    Started,
    Canceled,
}

impl TrustedHostLocalTimerCancellation {
    pub(crate) fn new() -> (Self, TrustedHostLocalTimerCancellationHandle) {
        let shared = Arc::new(SharedCancellation {
            state: Mutex::new(CancellationState {
                canceled: false,
                entry_started: false,
            }),
            signal: Condvar::new(),
        });
        (
            Self {
                shared: Arc::clone(&shared),
            },
            TrustedHostLocalTimerCancellationHandle { shared },
        )
    }

    pub(crate) fn begin_entry(
        &self,
    ) -> Result<TrustedHostLocalTimerEntryDecision, WorkflowOsError> {
        let mut state = self.shared.state.lock().map_err(|_| timer_state_error())?;
        if state.entry_started {
            return Err(entry_already_started_error());
        }
        if state.canceled {
            return Ok(TrustedHostLocalTimerEntryDecision::Canceled);
        }
        state.entry_started = true;
        Ok(TrustedHostLocalTimerEntryDecision::Started)
    }
}

impl fmt::Debug for TrustedHostLocalTimerCancellation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostLocalTimerCancellation")
            .finish_non_exhaustive()
    }
}

impl TrustedHostLocalTimerCancellationHandle {
    pub(crate) fn cancel(&self) -> Result<(), WorkflowOsError> {
        let mut state = self.shared.state.lock().map_err(|_| timer_state_error())?;
        state.canceled = true;
        self.shared.signal.notify_all();
        Ok(())
    }

    #[cfg(test)]
    fn notify_without_canceling(&self) {
        self.shared.signal.notify_all();
    }

    #[cfg(test)]
    #[allow(clippy::expect_used, clippy::panic)]
    fn poison(&self) {
        let shared = Arc::clone(&self.shared);
        let _ = std::thread::spawn(move || {
            let _guard = shared.state.lock().expect("test lock");
            panic!("intentional test poison");
        })
        .join();
    }
}

impl fmt::Debug for TrustedHostLocalTimerCancellationHandle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostLocalTimerCancellationHandle")
            .finish_non_exhaustive()
    }
}

type LocalClock = Arc<dyn Fn() -> Timestamp + Send + Sync>;

pub(crate) struct TrustedHostLocalDeadlineWaiter {
    cancellation: TrustedHostLocalTimerCancellation,
    clock: LocalClock,
}

impl TrustedHostLocalDeadlineWaiter {
    pub(crate) fn new(cancellation: TrustedHostLocalTimerCancellation) -> Self {
        Self {
            cancellation,
            clock: Arc::new(Timestamp::now_utc),
        }
    }

    #[cfg(test)]
    fn with_clock(
        cancellation: TrustedHostLocalTimerCancellation,
        clock: impl Fn() -> Timestamp + Send + Sync + 'static,
    ) -> Self {
        Self {
            cancellation,
            clock: Arc::new(clock),
        }
    }
}

impl fmt::Debug for TrustedHostLocalDeadlineWaiter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostLocalDeadlineWaiter")
            .field("clock", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

impl TrustedHostDeadlineWaiter for TrustedHostLocalDeadlineWaiter {
    fn wait_until(
        &mut self,
        schedule_at: Timestamp,
    ) -> Result<TrustedHostDeadlineWaitOutcome, TrustedHostDeadlineWaitFailure> {
        let mut state = self
            .cancellation
            .shared
            .state
            .lock()
            .map_err(|_| TrustedHostDeadlineWaitFailure::Failed)?;

        loop {
            if state.canceled {
                return Ok(TrustedHostDeadlineWaitOutcome::Canceled);
            }

            let now = (self.clock)();
            let remaining = schedule_at.as_offset_date_time() - now.as_offset_date_time();
            if remaining <= time::Duration::ZERO {
                return Ok(TrustedHostDeadlineWaitOutcome::Woke);
            }
            let timeout = Duration::try_from(remaining)
                .map_err(|_| TrustedHostDeadlineWaitFailure::Failed)?;
            let (next_state, _) = self
                .cancellation
                .shared
                .signal
                .wait_timeout(state, timeout)
                .map_err(|_| TrustedHostDeadlineWaitFailure::Failed)?;
            state = next_state;
        }
    }
}

pub(crate) struct TrustedHostLocalTimerInput<'a> {
    pub(crate) backend: &'a SqliteStateBackend,
    pub(crate) locator: TrustedHostOperationalEntryLocator,
    pub(crate) wake_budget: TrustedHostRepeatedWakeBudget,
    pub(crate) cancellation: TrustedHostLocalTimerCancellation,
    pub(crate) executor: &'a dyn TrustedHostAttemptExecutor,
    pub(crate) skill_input: SkillInput,
    pub(crate) redispatch_identity_provider: &'a mut dyn TrustedHostRedispatchIdentityProvider,
    pub(crate) wake_identity_provider: &'a mut dyn TrustedHostScheduleWakeIdentityProvider,
}

impl fmt::Debug for TrustedHostLocalTimerInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostLocalTimerInput")
            .field("wake_budget", &self.wake_budget)
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

pub(crate) fn run_trusted_host_local_timer(
    input: TrustedHostLocalTimerInput<'_>,
) -> Result<TrustedHostRepeatedSchedulingOutcome, WorkflowOsError> {
    let TrustedHostLocalTimerInput {
        backend,
        locator,
        wake_budget,
        cancellation,
        executor,
        skill_input,
        redispatch_identity_provider,
        wake_identity_provider,
    } = input;
    let mut deadline_waiter = TrustedHostLocalDeadlineWaiter::new(cancellation);
    run_bounded_trusted_host_repeated_scheduling(TrustedHostRepeatedSchedulingInput {
        backend,
        locator,
        wake_budget,
        deadline_waiter: &mut deadline_waiter,
        executor,
        skill_input,
        redispatch_identity_provider,
        wake_identity_provider,
    })
}

fn timer_state_error() -> WorkflowOsError {
    WorkflowOsError::new(
        WorkflowOsErrorKind::Internal,
        "trusted_host_local_timer.cancellation_state_failed",
        "trusted-host local timer cancellation state is unavailable",
    )
}

fn entry_already_started_error() -> WorkflowOsError {
    WorkflowOsError::new(
        WorkflowOsErrorKind::InvalidState,
        "trusted_host_local_timer.entry_already_started",
        "trusted-host local timer operational entry has already started",
    )
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod tests {
    use std::sync::mpsc;
    use std::time::{Duration, Instant};

    use super::*;

    #[test]
    fn elapsed_deadline_returns_without_blocking() {
        let (cancellation, _handle) = TrustedHostLocalTimerCancellation::new();
        let now = Timestamp::now_utc();
        let mut waiter = TrustedHostLocalDeadlineWaiter::with_clock(cancellation, move || now);

        let started = Instant::now();
        assert_eq!(
            waiter.wait_until(now).expect("elapsed wait"),
            TrustedHostDeadlineWaitOutcome::Woke
        );
        assert!(started.elapsed() < Duration::from_millis(100));
    }

    #[test]
    fn future_deadline_uses_real_timeout() {
        let (cancellation, _handle) = TrustedHostLocalTimerCancellation::new();
        let mut waiter = TrustedHostLocalDeadlineWaiter::new(cancellation);
        let deadline = Timestamp::from_offset_date_time(
            Timestamp::now_utc().as_offset_date_time() + time::Duration::milliseconds(30),
        );

        let started = Instant::now();
        assert_eq!(
            waiter.wait_until(deadline).expect("future wait"),
            TrustedHostDeadlineWaitOutcome::Woke
        );
        assert!(started.elapsed() >= Duration::from_millis(15));
        assert!(started.elapsed() < Duration::from_secs(2));
    }

    #[test]
    fn cancellation_before_wait_is_immediate_and_idempotent() {
        let (cancellation, handle) = TrustedHostLocalTimerCancellation::new();
        handle.cancel().expect("first cancel");
        handle.cancel().expect("second cancel");
        let mut waiter = TrustedHostLocalDeadlineWaiter::new(cancellation);
        let deadline = Timestamp::from_offset_date_time(
            Timestamp::now_utc().as_offset_date_time() + time::Duration::hours(1),
        );

        assert_eq!(
            waiter.wait_until(deadline).expect("canceled wait"),
            TrustedHostDeadlineWaitOutcome::Canceled
        );
    }

    #[test]
    fn cancellation_and_entry_have_one_linearized_decision() {
        let (canceled, canceled_handle) = TrustedHostLocalTimerCancellation::new();
        canceled_handle.cancel().expect("cancel before entry");
        assert_eq!(
            canceled.begin_entry().expect("canceled decision"),
            TrustedHostLocalTimerEntryDecision::Canceled
        );

        let (started, started_handle) = TrustedHostLocalTimerCancellation::new();
        assert_eq!(
            started.begin_entry().expect("started decision"),
            TrustedHostLocalTimerEntryDecision::Started
        );
        started_handle.cancel().expect("cancel after entry");
        assert_eq!(
            started.begin_entry().expect_err("entry is one-shot").code(),
            "trusted_host_local_timer.entry_already_started"
        );
    }

    #[test]
    fn cancellation_during_wait_wakes_promptly() {
        let (cancellation, handle) = TrustedHostLocalTimerCancellation::new();
        let deadline = Timestamp::from_offset_date_time(
            Timestamp::now_utc().as_offset_date_time() + time::Duration::hours(1),
        );
        let worker = std::thread::spawn(move || {
            let mut waiter = TrustedHostLocalDeadlineWaiter::new(cancellation);
            waiter.wait_until(deadline)
        });
        std::thread::sleep(Duration::from_millis(20));

        let started = Instant::now();
        handle.cancel().expect("cancel");
        assert_eq!(
            worker.join().expect("worker").expect("wait result"),
            TrustedHostDeadlineWaitOutcome::Canceled
        );
        assert!(started.elapsed() < Duration::from_secs(1));
    }

    #[test]
    fn spurious_notification_does_not_claim_deadline() {
        let (cancellation, handle) = TrustedHostLocalTimerCancellation::new();
        let deadline = Timestamp::from_offset_date_time(
            Timestamp::now_utc().as_offset_date_time() + time::Duration::hours(1),
        );
        let (sender, receiver) = mpsc::channel();
        let worker = std::thread::spawn(move || {
            let mut waiter = TrustedHostLocalDeadlineWaiter::new(cancellation);
            sender.send(waiter.wait_until(deadline)).expect("send");
        });
        std::thread::sleep(Duration::from_millis(20));
        handle.notify_without_canceling();
        assert!(receiver.recv_timeout(Duration::from_millis(40)).is_err());
        handle.cancel().expect("cancel");
        assert_eq!(
            receiver
                .recv_timeout(Duration::from_secs(1))
                .expect("result")
                .expect("wait"),
            TrustedHostDeadlineWaitOutcome::Canceled
        );
        worker.join().expect("worker");
    }

    #[test]
    fn poisoned_cancellation_state_fails_without_details() {
        let (cancellation, handle) = TrustedHostLocalTimerCancellation::new();
        handle.poison();
        let mut waiter = TrustedHostLocalDeadlineWaiter::new(cancellation);
        let error = waiter
            .wait_until(Timestamp::now_utc())
            .expect_err("poison must fail");
        assert_eq!(error, TrustedHostDeadlineWaitFailure::Failed);
        let cancel_error = handle.cancel().expect_err("poisoned cancel must fail");
        assert_eq!(
            cancel_error.code(),
            "trusted_host_local_timer.cancellation_state_failed"
        );
        assert!(!format!("{cancel_error:?}").contains("intentional test poison"));
    }

    #[test]
    fn debug_output_is_bounded_and_redacted() {
        let (cancellation, handle) = TrustedHostLocalTimerCancellation::new();
        let waiter = TrustedHostLocalDeadlineWaiter::new(cancellation);
        let debug = format!("{waiter:?} {handle:?}");
        assert!(debug.contains("[REDACTED]"));
        assert!(!debug.contains("Timestamp"));
        assert!(!debug.contains("Mutex"));
    }
}
