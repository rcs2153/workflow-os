# Workflow Run Event Model

Workflow OS runtime state is event-sourced. This document describes the v0 Rust event model. It is a state model only; it does not execute workflows, persist events, invoke skills, call adapters, or implement workers.

## Event Stream

Every workflow run has an append-only event stream. Events must have:

- Sequence number.
- Event ID.
- Timestamp.
- Run ID.
- Workflow ID.
- Workflow version.
- Spec content hash.
- Correlation ID where relevant.
- Actor or system actor where relevant.
- Idempotency key where relevant.
- Typed event kind.

Sequence numbers start at `1` and must be contiguous. `RunCreated` must be the first event.

## Immutable Run Identity

`RunCreated` binds the run to:

- Workflow run ID.
- Workflow ID.
- Workflow version.
- Spec content hash.

Every later event must carry the same identity. Rehydration fails if any event references a different workflow ID, version, run ID, or spec hash.

## Event Kinds

v0 defines:

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
- `RetryScheduled`
- `RetryStarted`
- `RetryExhausted`
- `EscalationTriggered`
- `ExternalEventReceived`
- `RunPaused`
- `RunResumed`
- `RunCompleted`
- `RunFailed`
- `RunCanceled`
- `PolicyDecisionRecorded`

Skill invocation and retry events require idempotency keys.

Approval events carry auditable approval context. `ApprovalRequested` records the run, workflow, workflow version, spec hash, step, skill, reason, requested timestamp, and expiration metadata where declared. `ApprovalGranted` and `ApprovalDenied` record actor, decision timestamp, decision kind, reason, and correlation ID.

In v0, `ApprovalGranted` records the decision while the run remains in `WaitingForApproval`; `RunResumed` must follow before the runtime can continue. `ApprovalDenied` records the decision and the local executor fails the run closed.

Retry events record attempt number, maximum attempts, last error, failure class, and suggested next action. `RetryExhausted` must be followed by either `EscalationTriggered` or a terminal failure.

Escalation events include run, step, skill, attempts, last error, failure class, suggested next action, reason, and optional contact.

Cancellation events include run, actor, timestamp, reason, and correlation ID.

Policy decision events are audit records emitted before meaningful runtime actions when a run exists. They include action, capabilities, actor, workflow/run context, reason codes, approval requirement, and violations.

## Audit Relationship

v0 intentionally keeps `WorkflowRunEvent` as the source of truth and emits `AuditEvent` as a structured projection of those events.

`AuditEvent` adds an audit-shaped envelope around runtime events:

- source event ID and timestamp
- workflow ID, workflow version, run ID, and spec hash
- step and skill references where the source event carries them
- actor or system actor
- action and decision context where derivable
- input and output references
- policy decision reference for `PolicyDecisionRecorded`
- correlation ID
- idempotency key
- redaction metadata
- source component

Audit projections must not store raw sensitive payloads by default.

## Snapshot Projection

`WorkflowRunSnapshot` is derived from events. It stores:

- Immutable run identity.
- Current run status.
- Last sequence number.
- Last event ID.
- Skill invocations.
- Approval requests and decisions.
- Retry records.
- Escalation records.
- Cancellation record.
- Failure record.

The snapshot is a projection, not an independent source of truth.

## Terminal States

`Completed`, `Failed`, and `Canceled` are terminal. v0 does not define metadata-only events after terminal states, so all events after terminal status are rejected.

## Non-Goals

The event model does not:

- Execute workflows.
- Persist events.
- Dispatch skills.
- Evaluate policy.
- Call adapters.
- Implement worker scheduling.
- Implement CLI behavior.
- Provide vendor-specific audit, SIEM, or observability export.
