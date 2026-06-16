# Execution Semantics

This document defines v0 execution semantics for Workflow OS. The current local executor implements a narrow subset of these semantics for sequential local runs: durable event replay, policy gates, approval pause/resume, bounded retry, escalation, cancellation, and idempotency for local skill invocation. Trigger delivery, active timeout scheduling, external event waits, distributed worker leasing, branching, parallelism, and real adapter side effects remain future work.

## At-Least-Once Execution

Workflow OS v0 should assume at-least-once execution at worker, trigger, and skill-invocation boundaries. The current local executor demonstrates this primarily through durable run rehydration and idempotency-keyed local skill invocation; it does not implement a distributed worker or trigger delivery system.

At-least-once means:

- A worker may attempt the same runtime step more than once.
- A trigger event may be delivered more than once.
- A retry may occur after uncertain worker failure.
- A side-effect attempt may need reconciliation.

Correctness must come from idempotency, durable state, event history, and policy gates, not from assuming exactly-once process execution.

## Idempotency Requirements

Idempotency is required for:

- Run creation from trigger events.
- Skill invocation.
- External side-effect attempts.
- Approval decisions.
- Retry scheduling.
- State transition application.

Idempotency keys must be stable and auditable. A repeated request with the same idempotency key must not create duplicate meaningful state transitions or duplicate completed side effects.

## Replay Expectations

Replay is the ability to inspect event history and reconstruct or reconcile current state.

Replay must:

- Use immutable workflow spec identity recorded at run creation.
- Respect schema version and spec content hash.
- Rebuild current state from meaningful transition events.
- Detect projection drift.
- Preserve original event timestamps and actors.
- Avoid re-performing external side effects unless explicitly requested through a safe recovery procedure.

Replay must not silently reinterpret a historical run against a newer workflow definition.

## Deterministic Boundaries

Deterministic boundaries include:

- Project loading order.
- Spec parsing.
- Schema validation.
- Workflow reference resolution.
- Content hashing.
- Policy evaluation for deterministic inputs.
- State transition validation.
- Event projection.

Non-deterministic boundaries include:

- Current time.
- External service responses.
- Human approval decisions.
- Skill execution that depends on model output.
- File system changes after run creation.
- Network behavior.
- Adapter outcomes.

Non-deterministic values must be captured as event data, input references, output references, or decision context so the run remains auditable.

## Timeout Expectations

Timeouts must be explicit for:

- Skill invocation.
- Approval waits.
- External event waits.
- Retry delays.
- Overall run execution where configured.

Timeouts must emit events once active timeout scheduling exists. In the current v0 implementation, timeout policies are parsed and represented, but no background timer interrupts local handlers or approval waits.

## Cancellation Expectations

Cancellation must be explicit and auditable.

Cancellation must:

- Record actor or system actor.
- Record timestamp and reason.
- Stop future runtime work for the run.
- Avoid starting new side effects.
- Preserve event history.
- Transition to Canceled only through `RunCanceled`.

If a side effect is already in flight when cancellation occurs, the runtime must record the uncertain or completed outcome when it becomes known. It must not pretend the side effect never happened.

## Approval Expiration Expectations

Approval waits may have expiration policies.

When active approval expiration handling is implemented, the runtime must:

- Emit `ApprovalExpired` or `ApprovalExpiredTerminal`.
- Include approval request ID.
- Include policy context.
- Include elapsed wait time.
- Escalate or fail according to policy.

The current v0 implementation stores approval expiration metadata but does not run background expiration timers. Approval expiration must not silently continue execution once timer behavior exists.

## Stuck Workflow Detection

The runtime must support stuck workflow detection through future worker or monitoring layers. v0 exposes observability hooks and state needed for detection, but it does not run a background detector.

A run may be stuck when:

- It remains in a non-terminal state beyond configured expectations.
- It exceeds approval wait thresholds.
- It exceeds external event wait thresholds.
- It repeatedly retries without progress.
- Its projection cannot be reconciled with event history.
- A worker lease or heartbeat model exists in a future implementation and expires.

Stuck detection must emit observability signals and may emit audit events or escalation events according to policy.

## Retry Semantics

Retry behavior must be explicit.

Retries must capture:

- Attempt number.
- Retry budget.
- Retry reason.
- Delay or backoff decision.
- Idempotency key.
- Last error category and summary.

Retry exhaustion must lead to Escalated or Failed. It must not loop forever or silently stop.

## External Events

External trigger and resume events must be deduplicated.

External events must include:

- External event key or provider reference.
- Received timestamp.
- Correlation ID where relevant.
- Source adapter identity where relevant.
- Policy context where relevant.

Real external adapters are deferred in v0, but the runtime semantics must be designed so adapters cannot bypass deduplication, policy, audit, or state transition validation.

## Side-Effect Boundary

The runtime may decide that a side effect should be attempted only after:

- The workflow spec is validated.
- The run is in a state that allows execution.
- Required approvals are granted.
- Policy permits the action.
- Capability checks pass.
- An idempotency key is allocated.
- Audit context is prepared.

External side effects must only occur through adapters. Adapters must return outcomes to the runtime; they must not mutate core runtime state directly.
