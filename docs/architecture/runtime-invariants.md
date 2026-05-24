# Runtime Invariants

This document defines non-negotiable Workflow OS v0 runtime invariants. Future implementation must enforce these invariants in code and tests.

## Run Identity

- Workflow definitions are immutable once a run starts.
- Each run must reference workflow ID, workflow version, schema version, and spec content hash.
- The run must preserve the exact workflow identity it was created from, even if local project files later change.
- A run must never execute against an implicit latest workflow definition.

## Event-Sourced State

- Every meaningful state transition emits an event.
- The event log is append-only.
- Event records must not be overwritten to hide or revise runtime history.
- Current state is derived from, or reconciled against, event history.
- State projections may exist for efficient reads, but they must not become an independent source of truth.

## Policy And Side Effects

- Policy checks occur before side effects.
- Unknown, unsafe, unsupported, or ambiguous actions fail closed.
- External side effects must only occur through adapters.
- Adapters cannot directly mutate core runtime state.
- Adapters must report outcomes through core interfaces that enforce validation, policy, audit, idempotency, and state transition rules.

## Idempotency

- Skill invocations are idempotency-keyed.
- External side-effect attempts are idempotency-keyed.
- Retries must reuse or derive stable idempotency keys according to documented retry semantics.
- Duplicate trigger events must be deduplicated.
- Restarted workers must not duplicate completed side effects.

## Approvals

- Approval-gated steps cannot execute until approval is granted.
- Approval decisions must be auditable.
- Approval records must include actor, timestamp, run ID, workflow ID, decision context, and policy context where relevant.
- Approval expiration must transition explicitly.
- Missing, expired, denied, or ambiguous approval must fail closed or escalate according to policy.

## Retry And Escalation

- Retry attempts must be counted.
- Retry delay and timeout behavior must be explicit.
- Retry exhaustion must lead to escalation or terminal failure.
- Escalation must emit an event.
- Escalation must include enough context for human review or safe diagnosis.

## Failure Safety

- Unknown state must fail closed.
- No workflow may silently terminate in an unsafe or ambiguous state.
- Runtime workers must be restart-safe.
- A worker crash must not corrupt event history.
- Partial side-effect outcomes must be represented explicitly.
- Cancellation must be explicit and auditable.

## Observability

Runtime events must support:

- Correlation IDs where relevant.
- Metrics.
- Tracing.
- Latency tracking.
- Retry counts.
- Failure counts.
- Escalation counts.
- Approval wait time.
- Stuck workflow detection.

## Required Event Metadata

Every meaningful transition event must include:

- Event ID.
- Event name.
- Timestamp.
- Workflow ID.
- Workflow version.
- Schema version.
- Spec content hash.
- Run ID.
- Previous state.
- New state.
- Actor or system actor.
- Correlation ID where relevant.
- Causation ID where relevant.
- Idempotency key where relevant.
- Policy context where relevant.
- Input references where relevant.
- Output references where relevant.
- Error or denial context where relevant.

Audit records should store references and summaries, not full sensitive payloads by default.

## Implementation Requirement

Any future implementation touching runtime state, policy, skill invocation, adapters, approvals, retries, or observability must include tests proving these invariants for expected behavior, failure modes, duplicate delivery, restart safety, and permission boundaries where applicable.
