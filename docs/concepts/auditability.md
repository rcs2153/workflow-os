# Auditability

Workflow OS treats auditability as part of runtime correctness. A meaningful runtime action must leave enough non-secret evidence for an operator to answer what happened, who or what caused it, which immutable workflow definition was used, and which policy context applied.

## v0 Model

v0 formalizes audit as a projection of `WorkflowRunEvent`.

The workflow event log remains the source of truth. `AuditEvent` is emitted from those events for audit sinks, logs, and future integrations. This avoids a second incompatible history while still giving operators a stable audit-shaped record.

Each audit event includes:

- source event ID and timestamp
- event type
- workflow ID and version
- workflow run ID
- spec content hash
- step ID where the event carries one
- skill ID where the event carries one
- actor or system actor
- action where derivable
- decision context summary
- input and output references
- policy decision reference for policy audit events
- correlation ID
- idempotency key where relevant
- redaction metadata
- source component

## Redaction

Audit events must store references, summaries, and policy context, not raw sensitive payloads. v0 redacts sensitive-looking output references before local audit recording. Future adapters and secret providers must continue this rule at their boundaries.

## Sink Failure

The local executor surfaces audit sink failures. In v0, the workflow event may already be durably appended when an audit sink rejects the derived audit event. That behavior is intentional and documented: runtime callers see a structured error instead of a hidden audit gap, and operators can reconcile from the event log.
