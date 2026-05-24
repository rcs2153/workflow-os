# Cancellation

Cancellation is an explicit terminal runtime transition. It must not leave a run ambiguous or partially active.

## Local Runtime Scope

The v0 local runtime exposes a cancellation request API for non-terminal runs. Cancellation emits `RunCanceled` and moves the run to `Canceled`.

Cancellation records:

- workflow run ID
- actor
- timestamp
- non-secret reason
- correlation ID

## Supported States

Cancellation is supported from non-terminal states, including `Running` and `WaitingForApproval`.

Cancellation after `Completed`, `Failed`, or `Canceled` is rejected.

## Non-Goals

v0 cancellation does not implement:

- external adapter cancellation
- compensating actions
- distributed worker interruption
- signal propagation into running user code

Future compensation behavior must be explicit, policy-governed, and auditable.
