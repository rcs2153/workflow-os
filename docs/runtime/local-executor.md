# Local Executor

The v0 local executor is the first minimal runtime kernel path. It proves that Workflow OS can load a project, validate it, create a workflow run, schedule one step, invoke one local skill handler, persist events, rehydrate state, and complete or fail the run.

It is intentionally narrow. It is not a general executor, distributed worker, adapter runtime, policy engine, approval engine, retry engine, or branch interpreter.

## Supported Scope

The local executor supports:

- Loading a project from `workflow-os.yml`.
- Running deterministic project validation before execution.
- Executing exactly one workflow step.
- Invoking exactly one registered local `SkillHandler`.
- Pausing before approval-gated local skill execution.
- Resuming approved local runs through the approval decision API.
- Applying bounded retry around the single local skill step.
- Escalating or failing closed after retry exhaustion.
- Canceling non-terminal local runs.
- Representing timeout policy without active background timers.
- Persisting events through a `StateBackend`.
- Enforcing conservative runtime policy before meaningful actions.
- Rehydrating current run state from durable events.
- Recording completed and failed terminal runs.
- Propagating a caller-provided correlation ID onto emitted events.
- Reusing an existing durable run when the same explicit run ID is requested again.
- Emitting audit, observability, and structured log records from runtime events.

The executor emits:

- `RunCreated`
- `RunValidated`
- `RunStarted`
- `StepScheduled`
- `SkillInvocationRequested`
- `SkillInvocationStarted`
- `SkillInvocationSucceeded`
- `SkillInvocationFailed`
- `ApprovalRequested`
- `ApprovalGranted`
- `ApprovalDenied`
- `RunResumed`
- `RetryScheduled`
- `RetryStarted`
- `RetryExhausted`
- `EscalationTriggered`
- `RunCompleted`
- `RunFailed`
- `RunCanceled`
- `PolicyDecisionRecorded`

## Audit And Observability

The local executor emits audit, observability, and structured log records at the same append boundary used for workflow run events.

In v0:

- `AuditSink` receives `AuditEvent` projections of workflow events.
- `ObservabilitySink` receives local metric-style events for run, skill, retry, escalation, approval, and policy paths.
- `StructuredLogger` receives metadata-only structured log records.
- Sink failures are returned as structured errors instead of being hidden.
- Local sink implementations are development and test utilities, not production stores.

## Conservative Policy Boundary

The v0 executor uses the conservative policy engine before meaningful actions. Adapter invocation, `external.write`, unknown capabilities, missing context, and Level 3/4 execution are denied by default.

Denied decisions are audited when a run exists. A denied skill or adapter invocation fails the run closed.

## Idempotency

Skill invocation events use a derived idempotency key based on:

- run ID
- workflow ID
- workflow version
- step ID
- skill ID
- skill version

The executor records the invocation in the backend `IdempotencyStore`. If an execution request reuses an existing run ID, the executor rehydrates and returns the durable run instead of invoking the handler again.

## Output Contract Check

The local executor checks that required output fields declared by the skill output contract are present in the handler result.

Full type checking, nested object validation, field-level redaction enforcement at runtime, and schema-driven contract validation are deferred. Future contract validation must remain Rust-owned and deterministic.

## Approval Gates

If the single step declares an approval policy, the local executor emits `ApprovalRequested` and stops in `WaitingForApproval` before any skill invocation event is emitted. The local handler is not called while waiting.

`LocalApprovalDecisionRequest` grants or denies the approval:

- grant appends `ApprovalGranted`, `RunResumed`, then proceeds with local skill invocation
- denial appends `ApprovalDenied` and `RunFailed`

Approval expiration metadata is stored on the approval request when declared, but v0 does not run background timers.

## Retries, Escalation, And Cancellation

If the step declares a retry policy, the local executor enforces a bounded attempt count. Retry attempts emit `RetryScheduled` and `RetryStarted`. Exhaustion emits `RetryExhausted` and then either `EscalationTriggered` when the step has an escalation policy or `RunFailed` when terminal failure is the declared behavior.

Cancellation emits `RunCanceled` from non-terminal states and is rejected after terminal states.

Timeout policy is parsed and represented for runtime classification. Active timeout scheduling is deferred.

## Unsupported In v0

The local executor does not implement:

- multi-step workflows
- conditional branches
- external adapters
- distributed workers
- production databases
- real trigger processing
- enterprise RBAC or IdP-backed policy
- full input/output schema validation

Unsupported behavior must fail closed and must not be documented as production behavior.
