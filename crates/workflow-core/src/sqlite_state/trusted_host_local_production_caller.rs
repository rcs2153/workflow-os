use std::cell::RefCell;
use std::fmt;

use crate::authorized_execution_continuity_state::internal::{
    ContinuityOperationId, ContinuityReceiptId, ContinuityYieldGenerationId,
};
use crate::trusted_host_supervisor::TrustedHostAttemptExecutor;
use crate::{AuthorizedExecutionAttemptId, SkillInput, WorkflowOsError, WorkflowOsErrorKind};

use super::trusted_host_local_timer::{
    run_trusted_host_local_timer, TrustedHostLocalTimerCancellation, TrustedHostLocalTimerInput,
};
use super::trusted_host_operational_entry::TrustedHostOperationalEntryLocator;
use super::trusted_host_redispatch_loop::{
    TrustedHostRedispatchIdentityProvider, TrustedHostRedispatchIterationIdentity,
};
use super::trusted_host_time_window_scheduling::{
    TrustedHostRepeatedSchedulingOutcome, TrustedHostRepeatedWakeBudget,
    TrustedHostScheduleWakeIdentity, TrustedHostScheduleWakeIdentityProvider,
};
use super::SqliteStateBackend;

const LOCAL_PRODUCTION_WAKE_BUDGET: u32 = 2;
const REDISPATCH_RANDOM_BYTES: usize = 96;
const WAKE_RANDOM_BYTES: usize = 32;
const RANDOM_ID_BYTES: usize = 16;

#[cfg(test)]
type TestByteFiller = Box<dyn FnMut(&mut [u8]) -> Result<(), ()>>;

enum IdentityByteFiller {
    Production,
    #[cfg(test)]
    Test(TestByteFiller),
}

impl IdentityByteFiller {
    fn fill(&mut self, destination: &mut [u8]) -> Result<(), WorkflowOsError> {
        let result = match self {
            Self::Production => getrandom::getrandom(destination).map_err(|_| ()),
            #[cfg(test)]
            Self::Test(filler) => filler(destination),
        };
        result.map_err(|()| identity_error())
    }
}

pub(crate) struct TrustedHostLocalProductionIdentitySource {
    filler: RefCell<IdentityByteFiller>,
}

impl TrustedHostLocalProductionIdentitySource {
    pub(crate) const fn new() -> Self {
        Self {
            filler: RefCell::new(IdentityByteFiller::Production),
        }
    }

    #[cfg(test)]
    fn with_test_filler(filler: impl FnMut(&mut [u8]) -> Result<(), ()> + 'static) -> Self {
        Self {
            filler: RefCell::new(IdentityByteFiller::Test(Box::new(filler))),
        }
    }

    pub(crate) fn redispatch_provider(&self) -> RedispatchIdentityProvider<'_> {
        RedispatchIdentityProvider { source: self }
    }

    fn wake_provider(&self) -> WakeIdentityProvider<'_> {
        WakeIdentityProvider { source: self }
    }

    fn fill<const N: usize>(&self) -> Result<[u8; N], WorkflowOsError> {
        let mut bytes = [0_u8; N];
        self.filler.borrow_mut().fill(&mut bytes)?;
        Ok(bytes)
    }
}

impl fmt::Debug for TrustedHostLocalProductionIdentitySource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostLocalProductionIdentitySource")
            .field("identity_material", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

pub(crate) struct RedispatchIdentityProvider<'a> {
    source: &'a TrustedHostLocalProductionIdentitySource,
}

impl TrustedHostRedispatchIdentityProvider for RedispatchIdentityProvider<'_> {
    fn next_identity(
        &mut self,
        _iteration: u32,
    ) -> Result<TrustedHostRedispatchIterationIdentity, WorkflowOsError> {
        let bytes = self.source.fill::<REDISPATCH_RANDOM_BYTES>()?;
        Ok(TrustedHostRedispatchIterationIdentity {
            consume_operation: operation_id("operation/thpc/consume", chunk(&bytes, 0))?,
            consume_receipt: receipt_id("receipt/thpc/consume", chunk(&bytes, 1))?,
            generated_attempt: AuthorizedExecutionAttemptId::new(identifier(
                "attempt/thpc",
                chunk(&bytes, 2),
            ))?,
            supervisor_operation: operation_id("operation/thpc/supervisor", chunk(&bytes, 3))?,
            supervisor_receipt: receipt_id("receipt/thpc/supervisor", chunk(&bytes, 4))?,
            yield_generation: ContinuityYieldGenerationId::new(identifier(
                "yield/thpc",
                chunk(&bytes, 5),
            ))?,
        })
    }
}

struct WakeIdentityProvider<'a> {
    source: &'a TrustedHostLocalProductionIdentitySource,
}

impl TrustedHostScheduleWakeIdentityProvider for WakeIdentityProvider<'_> {
    fn next_identity(
        &mut self,
        _wake_attempt: u32,
    ) -> Result<TrustedHostScheduleWakeIdentity, WorkflowOsError> {
        let bytes = self.source.fill::<WAKE_RANDOM_BYTES>()?;
        Ok(TrustedHostScheduleWakeIdentity {
            operation_id: operation_id("operation/thpc/wake", chunk(&bytes, 0))?,
            receipt_id: receipt_id("receipt/thpc/wake", chunk(&bytes, 1))?,
        })
    }
}

pub(crate) struct TrustedHostLocalProductionCallerInput<'a> {
    pub(crate) backend: &'a SqliteStateBackend,
    pub(crate) locator: TrustedHostOperationalEntryLocator,
    pub(crate) cancellation: TrustedHostLocalTimerCancellation,
    pub(crate) executor: &'a dyn TrustedHostAttemptExecutor,
    pub(crate) skill_input: SkillInput,
}

impl fmt::Debug for TrustedHostLocalProductionCallerInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostLocalProductionCallerInput")
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

pub(crate) fn run_trusted_host_local_production_caller(
    input: TrustedHostLocalProductionCallerInput<'_>,
) -> Result<TrustedHostRepeatedSchedulingOutcome, WorkflowOsError> {
    let source = TrustedHostLocalProductionIdentitySource::new();
    run_with_identity_source(input, &source)
}

fn run_with_identity_source(
    input: TrustedHostLocalProductionCallerInput<'_>,
    source: &TrustedHostLocalProductionIdentitySource,
) -> Result<TrustedHostRepeatedSchedulingOutcome, WorkflowOsError> {
    let mut redispatch_identity_provider = source.redispatch_provider();
    let mut wake_identity_provider = source.wake_provider();
    run_trusted_host_local_timer(TrustedHostLocalTimerInput {
        backend: input.backend,
        locator: input.locator,
        wake_budget: TrustedHostRepeatedWakeBudget::new(LOCAL_PRODUCTION_WAKE_BUDGET)?,
        cancellation: input.cancellation,
        executor: input.executor,
        skill_input: input.skill_input,
        redispatch_identity_provider: &mut redispatch_identity_provider,
        wake_identity_provider: &mut wake_identity_provider,
    })
}

fn chunk<const N: usize>(bytes: &[u8; N], index: usize) -> &[u8] {
    let start = index * RANDOM_ID_BYTES;
    &bytes[start..start + RANDOM_ID_BYTES]
}

fn operation_id(prefix: &str, bytes: &[u8]) -> Result<ContinuityOperationId, WorkflowOsError> {
    ContinuityOperationId::new(identifier(prefix, bytes))
}

fn receipt_id(prefix: &str, bytes: &[u8]) -> Result<ContinuityReceiptId, WorkflowOsError> {
    ContinuityReceiptId::new(identifier(prefix, bytes))
}

fn identifier(prefix: &str, bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut value = String::with_capacity(prefix.len() + 1 + bytes.len() * 2);
    value.push_str(prefix);
    value.push('/');
    for byte in bytes {
        value.push(char::from(HEX[usize::from(byte >> 4)]));
        value.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    value
}

fn identity_error() -> WorkflowOsError {
    WorkflowOsError::new(
        WorkflowOsErrorKind::Internal,
        "trusted_host_local_production_caller.identity_generation_failed",
        "trusted-host local production identity generation failed",
    )
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use std::cell::Cell;
    use std::collections::BTreeSet;
    use std::rc::Rc;

    use super::*;

    #[test]
    fn redispatch_identity_uses_one_atomic_fill_and_independent_domains() {
        let calls = Rc::new(Cell::new(0_usize));
        let observed = Rc::clone(&calls);
        let source = TrustedHostLocalProductionIdentitySource::with_test_filler(move |bytes| {
            observed.set(observed.get() + 1);
            for (index, byte) in bytes.iter_mut().enumerate() {
                *byte = u8::try_from(index).expect("bounded index");
            }
            Ok(())
        });
        let mut provider = source.redispatch_provider();

        let identity = provider.next_identity(1).expect("identity");
        let values = BTreeSet::from([
            identity.consume_operation.as_str(),
            identity.consume_receipt.as_str(),
            identity.generated_attempt.as_str(),
            identity.supervisor_operation.as_str(),
            identity.supervisor_receipt.as_str(),
            identity.yield_generation.as_str(),
        ]);

        assert_eq!(calls.get(), 1);
        assert_eq!(values.len(), 6);
        assert!(identity
            .consume_operation
            .as_str()
            .starts_with("operation/thpc/consume/"));
        assert!(identity
            .consume_receipt
            .as_str()
            .starts_with("receipt/thpc/consume/"));
        assert!(identity
            .generated_attempt
            .as_str()
            .starts_with("attempt/thpc/"));
        assert!(identity
            .supervisor_operation
            .as_str()
            .starts_with("operation/thpc/supervisor/"));
        assert!(identity
            .supervisor_receipt
            .as_str()
            .starts_with("receipt/thpc/supervisor/"));
        assert!(identity
            .yield_generation
            .as_str()
            .starts_with("yield/thpc/"));
        assert!(values.iter().all(|value| value.len() <= 128));
    }

    #[test]
    fn wake_identity_uses_one_atomic_fill_and_distinct_domains() {
        let calls = Rc::new(Cell::new(0_usize));
        let observed = Rc::clone(&calls);
        let source = TrustedHostLocalProductionIdentitySource::with_test_filler(move |bytes| {
            observed.set(observed.get() + 1);
            for (index, byte) in bytes.iter_mut().enumerate() {
                *byte = u8::try_from(index + 1).expect("bounded index");
            }
            Ok(())
        });
        let mut provider = source.wake_provider();

        let identity = provider.next_identity(1).expect("identity");

        assert_eq!(calls.get(), 1);
        assert!(identity
            .operation_id
            .as_str()
            .starts_with("operation/thpc/wake/"));
        assert!(identity
            .receipt_id
            .as_str()
            .starts_with("receipt/thpc/wake/"));
        assert_ne!(identity.operation_id.as_str(), identity.receipt_id.as_str());
    }

    #[test]
    fn entropy_failure_is_stable_non_leaking_and_returns_no_identity() {
        let source = TrustedHostLocalProductionIdentitySource::with_test_filler(|bytes| {
            bytes.fill(0x5a);
            Err(())
        });
        let mut redispatch = source.redispatch_provider();
        let error = redispatch.next_identity(1).expect_err("entropy failure");

        assert_eq!(
            error.code(),
            "trusted_host_local_production_caller.identity_generation_failed"
        );
        let debug = format!("{error:?}");
        assert!(!debug.contains("5a"));
        assert!(!debug.contains("operation/thpc"));
    }

    #[test]
    fn identity_debug_redacts_random_material() {
        let source = TrustedHostLocalProductionIdentitySource::with_test_filler(|bytes| {
            bytes.fill(0xab);
            Ok(())
        });
        let debug = format!("{source:?}");

        assert!(!debug.contains("ab"));
        assert!(debug.contains("[REDACTED]"));
    }
}
