use std::fmt;

use crate::WorkflowOsError;

use super::trusted_host_explicit_local_operation::{
    run_explicit_trusted_host_local_operation, TrustedHostExplicitLocalOperationInput,
    TrustedHostExplicitLocalOperationOutcome,
};
use super::trusted_host_local_timer::{
    TrustedHostLocalTimerCancellation, TrustedHostLocalTimerCancellationHandle,
    TrustedHostLocalTimerEntryDecision,
};
use super::trusted_host_operational_entry::TrustedHostOperationalEntryInput;

pub(crate) struct TrustedHostExplicitLocalProcessOwner<'a> {
    operational_entry: TrustedHostOperationalEntryInput<'a>,
    cancellation: TrustedHostLocalTimerCancellation,
}

pub(crate) enum TrustedHostExplicitLocalProcessOwnerOutcome {
    CanceledBeforeEntry,
    OperationStopped(TrustedHostExplicitLocalOperationOutcome),
}

impl<'a> TrustedHostExplicitLocalProcessOwner<'a> {
    pub(crate) fn new(
        operational_entry: TrustedHostOperationalEntryInput<'a>,
    ) -> (Self, TrustedHostLocalTimerCancellationHandle) {
        let (cancellation, handle) = TrustedHostLocalTimerCancellation::new();
        (
            Self::with_cancellation(operational_entry, cancellation),
            handle,
        )
    }

    pub(crate) fn with_cancellation(
        operational_entry: TrustedHostOperationalEntryInput<'a>,
        cancellation: TrustedHostLocalTimerCancellation,
    ) -> Self {
        Self {
            operational_entry,
            cancellation,
        }
    }

    pub(crate) fn run(
        self,
    ) -> Result<TrustedHostExplicitLocalProcessOwnerOutcome, WorkflowOsError> {
        match self.cancellation.begin_entry()? {
            TrustedHostLocalTimerEntryDecision::Canceled => {
                Ok(TrustedHostExplicitLocalProcessOwnerOutcome::CanceledBeforeEntry)
            }
            TrustedHostLocalTimerEntryDecision::Started => {
                let outcome = run_explicit_trusted_host_local_operation(
                    TrustedHostExplicitLocalOperationInput {
                        operational_entry: self.operational_entry,
                        cancellation: self.cancellation,
                    },
                )?;
                Ok(TrustedHostExplicitLocalProcessOwnerOutcome::OperationStopped(outcome))
            }
        }
    }
}

impl fmt::Debug for TrustedHostExplicitLocalProcessOwner<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostExplicitLocalProcessOwner")
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

impl fmt::Debug for TrustedHostExplicitLocalProcessOwnerOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CanceledBeforeEntry => formatter
                .debug_tuple("TrustedHostExplicitLocalProcessOwnerOutcome::CanceledBeforeEntry")
                .finish(),
            Self::OperationStopped(outcome) => formatter
                .debug_tuple("TrustedHostExplicitLocalProcessOwnerOutcome::OperationStopped")
                .field(outcome)
                .finish(),
        }
    }
}
