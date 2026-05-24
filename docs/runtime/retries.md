# Retries

Workflow OS retries are bounded runtime behavior. A retry must never be an unbounded loop and must never hide repeated failures.

## Local Runtime Scope

The v0 local executor supports bounded retries for its single local skill step.

A step opts into retry by declaring a `retry_policy`. The referenced policy must include retry behavior and bounded retry behavior according to semantic validation. The local runtime reads an optional policy rule effect of `max_attempts=N` or `max_attempts:N`. When no explicit maximum is present, v0 uses `2` attempts. A workflow without a retry policy receives one attempt.

## Retry Events

On a failed attempt with retry budget remaining, the local runtime emits:

- `SkillInvocationFailed`
- `RetryScheduled`
- `RetryStarted`
- the next `SkillInvocationStarted`

When the retry budget is exhausted, the local runtime emits:

- `SkillInvocationFailed`
- `RetryExhausted`
- `EscalationTriggered` if an escalation policy exists
- otherwise `RunFailed`

## Idempotency

Each skill attempt uses a derived idempotency key based on the logical skill invocation key plus the attempt number. Replaying the same run ID rehydrates the durable run instead of invoking the local handler again.

This prevents duplicate side effects for the same persisted run. Real external side-effect retries remain deferred until adapter execution exists.

## Failure Classes

Retry records include the last error, failure class, attempt number, maximum attempts, and suggested next action. The v0 local runtime classifies errors with `transient` in the error code as transient; unsupported, validation, policy, and security failures are treated as non-transient classes.

## Non-Goals

v0 retries do not implement:

- background schedulers
- delayed retry timers
- jitter or backoff timing
- distributed worker coordination
- adapter side-effect retry

Future retry scheduling must preserve the same bounded event semantics.
