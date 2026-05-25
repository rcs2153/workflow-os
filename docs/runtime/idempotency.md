# Idempotency

Idempotency prevents duplicate side effects and duplicate event appends when work is retried or delivered more than once.

## Store Contract

`IdempotencyStore` records a key and a non-secret result reference.

The first write for a key returns `FirstWrite`. Later writes with the same key return `Duplicate` and the original stored result. The duplicate caller must not perform the side effect again.

## Event Model Relationship

The event model requires idempotency keys on relevant skill invocation and retry events. The state backend preserves those keys in durable event records.

Appending the same workflow event twice is not treated as an idempotent success. The event log rejects duplicate event IDs and duplicate run sequence numbers so callers cannot accidentally create ambiguous history. Runtime idempotency for work results is handled through `IdempotencyStore` before side effects and before side-effecting event sequences continue.

Future executors should:

1. Derive or read the idempotency key before side effects.
2. Check or record the key before appending side-effecting events.
3. Return the existing result on duplicate keys.
4. Avoid storing raw sensitive payloads as idempotency results.

## Result Values

Idempotency results are references or summaries. They must not contain secrets or full sensitive payloads.

## Non-Goals

The v0 idempotency store does not execute work, call adapters, or coordinate distributed systems. It only defines and tests the durable local semantics needed for future restart-safe execution.
