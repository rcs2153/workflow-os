#![deny(unsafe_code)]
#![doc = "Unpublished local trusted-host composition boundary for Workflow OS."]
#![doc = ""]
#![doc = "This crate consumes one opaque Core-prepared operation. It does not"]
#![doc = "prepare authority, discover work, schedule runs, or define workflow truth."]

use std::fmt;

use workflow_core::{
    TrustedHostLocalApplicationCancellationHandle, TrustedHostLocalApplicationFailure,
    TrustedHostLocalApplicationOutcome, TrustedHostLocalApplicationPreparedSession,
    TrustedHostLocalApplicationSession,
};

trait RunOnce {
    fn run_once(
        self,
    ) -> Result<TrustedHostLocalApplicationOutcome, TrustedHostLocalApplicationFailure>;
}

impl RunOnce for TrustedHostLocalApplicationSession<'_> {
    fn run_once(
        self,
    ) -> Result<TrustedHostLocalApplicationOutcome, TrustedHostLocalApplicationFailure> {
        self.run()
    }
}

trait RequestCancellation: Clone {
    fn request(&self) -> Result<(), TrustedHostLocalApplicationFailure>;
}

impl RequestCancellation for TrustedHostLocalApplicationCancellationHandle {
    fn request(&self) -> Result<(), TrustedHostLocalApplicationFailure> {
        self.request_cancellation()
    }
}

struct PreparedOperation<S, C> {
    session: S,
    cancellation: C,
}

impl<S, C> PreparedOperation<S, C>
where
    S: RunOnce,
    C: RequestCancellation,
{
    fn cancellation(&self) -> C {
        self.cancellation.clone()
    }

    fn run(self) -> Result<TrustedHostLocalApplicationOutcome, TrustedHostLocalApplicationFailure> {
        let Self {
            session,
            cancellation,
        } = self;
        let result = session.run_once();
        let _retained_until_return = cancellation;
        result
    }
}

/// One opaque Core-prepared operation owned by the local composition library.
///
/// The operation is one-shot, cannot be cloned or serialized, and does not
/// expose any workflow, run, actor, authority, or persistence binding.
///
/// Operations cannot be cloned:
///
/// ```compile_fail
/// fn duplicate(operation: workflow_local_host::LocalHostPreparedOperation<'_>) {
///     let _copy = operation.clone();
/// }
/// ```
pub struct LocalHostPreparedOperation<'a> {
    inner: PreparedOperation<
        TrustedHostLocalApplicationSession<'a>,
        TrustedHostLocalApplicationCancellationHandle,
    >,
}

impl<'a> LocalHostPreparedOperation<'a> {
    /// Takes custody of one unforgeable Core-issued prepared pair.
    #[must_use]
    pub fn from_prepared(prepared: TrustedHostLocalApplicationPreparedSession<'a>) -> Self {
        let (session, cancellation) = prepared.into_parts();
        Self {
            inner: PreparedOperation {
                session,
                cancellation,
            },
        }
    }

    /// Returns a bounded control for the same operation-scoped cancellation state.
    #[must_use]
    pub fn cancellation_control(&self) -> LocalHostCancellationControl {
        LocalHostCancellationControl {
            inner: self.inner.cancellation(),
        }
    }

    /// Consumes the operation and synchronously runs its single bound session.
    ///
    /// # Errors
    ///
    /// Returns only the fixed, payload-free application failure projected by
    /// Core. The method does not reinterpret the result as workflow status.
    pub fn run(
        self,
    ) -> Result<TrustedHostLocalApplicationOutcome, TrustedHostLocalApplicationFailure> {
        self.inner.run()
    }
}

impl fmt::Debug for LocalHostPreparedOperation<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalHostPreparedOperation")
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

/// Cloneable control for one shared, operation-scoped cancellation state.
///
/// Cloning this value does not create execution authority, a new session, or a
/// new cancellation domain. Cancellation remains cooperative and does not
/// interrupt an attempt after executor admission.
#[derive(Clone)]
pub struct LocalHostCancellationControl {
    inner: TrustedHostLocalApplicationCancellationHandle,
}

impl LocalHostCancellationControl {
    /// Requests cooperative cancellation for the one associated operation.
    ///
    /// # Errors
    ///
    /// Returns only the fixed, payload-free Core application failure.
    pub fn request_cancellation(&self) -> Result<(), TrustedHostLocalApplicationFailure> {
        self.inner.request()
    }
}

impl fmt::Debug for LocalHostCancellationControl {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalHostCancellationControl")
            .field("state", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use workflow_core::TrustedHostLocalApplicationEntryStopReason;

    struct FakeSession {
        calls: Arc<Mutex<usize>>,
    }

    impl RunOnce for FakeSession {
        fn run_once(
            self,
        ) -> Result<TrustedHostLocalApplicationOutcome, TrustedHostLocalApplicationFailure>
        {
            let mut calls = self
                .calls
                .lock()
                .map_err(|_| TrustedHostLocalApplicationFailure::Internal)?;
            *calls += 1;
            Ok(TrustedHostLocalApplicationOutcome::EntryStopped(
                TrustedHostLocalApplicationEntryStopReason::Terminal,
            ))
        }
    }

    #[derive(Clone)]
    struct FakeCancellation {
        canceled: Arc<Mutex<bool>>,
    }

    impl RequestCancellation for FakeCancellation {
        fn request(&self) -> Result<(), TrustedHostLocalApplicationFailure> {
            let mut canceled = self
                .canceled
                .lock()
                .map_err(|_| TrustedHostLocalApplicationFailure::Internal)?;
            *canceled = true;
            Ok(())
        }
    }

    #[test]
    fn private_runner_consumes_one_session_and_propagates_bounded_outcome() {
        let calls = Arc::new(Mutex::new(0));
        let operation = PreparedOperation {
            session: FakeSession {
                calls: Arc::clone(&calls),
            },
            cancellation: FakeCancellation {
                canceled: Arc::new(Mutex::new(false)),
            },
        };

        assert_eq!(
            operation.run().expect("bounded outcome"),
            TrustedHostLocalApplicationOutcome::EntryStopped(
                TrustedHostLocalApplicationEntryStopReason::Terminal
            )
        );
        assert_eq!(*calls.lock().expect("call count"), 1);
    }

    #[test]
    fn private_cancellation_clones_share_one_scoped_state() {
        let canceled = Arc::new(Mutex::new(false));
        let operation = PreparedOperation {
            session: FakeSession {
                calls: Arc::new(Mutex::new(0)),
            },
            cancellation: FakeCancellation {
                canceled: Arc::clone(&canceled),
            },
        };
        let first = operation.cancellation();
        let second = operation.cancellation();

        first.request().expect("first cancellation");
        second.request().expect("second cancellation");

        assert!(*canceled.lock().expect("canceled state"));
    }

    #[test]
    fn dropping_an_unrun_operation_does_not_invoke_the_session() {
        let calls = Arc::new(Mutex::new(0));
        let operation = PreparedOperation {
            session: FakeSession {
                calls: Arc::clone(&calls),
            },
            cancellation: FakeCancellation {
                canceled: Arc::new(Mutex::new(false)),
            },
        };

        drop(operation);

        assert_eq!(*calls.lock().expect("call count"), 0);
    }

    #[test]
    fn cancellation_control_after_return_remains_bounded_and_scoped() {
        let canceled = Arc::new(Mutex::new(false));
        let operation = PreparedOperation {
            session: FakeSession {
                calls: Arc::new(Mutex::new(0)),
            },
            cancellation: FakeCancellation {
                canceled: Arc::clone(&canceled),
            },
        };
        let control = operation.cancellation();

        let _outcome = operation.run().expect("bounded outcome");
        control.request().expect("post-return cancellation");

        assert!(*canceled.lock().expect("canceled state"));
    }

    #[test]
    fn public_type_names_expose_no_bound_context() {
        let operation_name = std::any::type_name::<LocalHostPreparedOperation<'static>>();
        let control_name = std::any::type_name::<LocalHostCancellationControl>();

        assert!(operation_name.contains("LocalHostPreparedOperation"));
        assert!(control_name.contains("LocalHostCancellationControl"));
        assert!(!operation_name.contains("token-private"));
        assert!(!control_name.contains("token-private"));
    }
}
