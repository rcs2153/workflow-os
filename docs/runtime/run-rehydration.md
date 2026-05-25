# Run Rehydration

Run rehydration reconstructs `WorkflowRunSnapshot` from an ordered event stream.

Rehydration is deterministic and does not depend on local process memory. Given the same ordered events, it must either produce the same snapshot or fail with the same structured error.

## Rules

The v0 rehydration logic requires:

- Event stream is non-empty.
- First event has sequence number `1`.
- First event is `RunCreated`.
- Sequence numbers are unique and contiguous.
- Every event carries the same run ID, workflow ID, schema version, workflow version, and spec content hash as `RunCreated`.
- Every event is valid from the current projected status.
- No event appears after a terminal status.
- Idempotency-keyed event kinds include an idempotency key.

## Projection

During replay, the rehydrator:

1. Creates the initial snapshot from `RunCreated`.
2. Validates the next sequence number.
3. Validates immutable identity.
4. Validates state transition.
5. Applies typed event payloads to the snapshot.
6. Updates the last sequence number and last event ID.

## Failure Behavior

Invalid streams fail deterministically with `WorkflowOsErrorKind::InvalidState`.

Examples of invalid streams:

- Missing `RunCreated`.
- Duplicate sequence number.
- Non-contiguous sequence number.
- Mismatched spec hash.
- Mismatched schema version.
- `RunStarted` before `RunValidated`.
- Mutating event after `RunCompleted`, `RunFailed`, or `RunCanceled`.

## Restart Safety

A future worker can restart, reload events from durable storage, rehydrate the snapshot, and continue from the derived state. Correctness must not depend on any in-memory worker state that is not represented in the event stream.

## Compatibility

Local state created before schema version was part of runtime event metadata is not auto-migrated in v0. Those event JSON files are missing required immutable identity data and fail deserialization or rehydration clearly instead of being silently defaulted. Preserve the old state root for forensic inspection and create a new local state root for new runs, or write an explicit migration in a future release.
