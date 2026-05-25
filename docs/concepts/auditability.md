# Auditability

Workflow OS treats auditability as part of runtime correctness. A meaningful runtime action must leave enough non-secret evidence for an operator to answer what happened, who or what caused it, which immutable workflow definition was used, and which policy context applied.

## v0 Model

v0 formalizes run-scoped audit as a projection of `WorkflowRunEvent` and policy-decision audit as a durable `PolicyAuditRecord` ledger.

The workflow event log remains the source of truth. `AuditEvent` is emitted from those events for audit sinks, logs, and future integrations. This avoids a second incompatible history while still giving operators a stable audit-shaped record.

Audit event order follows runtime authorization order. For approval-gated steps, audit sinks see `StepScheduled` and `ApprovalRequested` before approval, and do not see `SkillInvocationRequested` until approval has been granted and the run has resumed.

Policy decisions that happen before `RunCreated` do not have a workflow event stream yet. Those decisions are written to the durable policy audit ledger and emitted to audit sinks as policy audit records. Denied starts therefore leave an operator-visible audit record without creating a fake workflow run.

Each audit event includes:

- source event ID and timestamp
- event type
- workflow ID, schema version, and workflow version
- workflow run ID
- spec content hash
- step ID where the event carries one
- skill ID where the event carries one
- skill version where the event carries one
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

Audit events must store references, summaries, and policy context, not raw sensitive payloads. v0 records field-level redaction metadata that identifies fields as safe metadata, reference-only, or redacted.

The local executor does not place raw skill input values into audit records. It emits invocation input references instead. Skill output references are recorded as references; sensitive-looking references are deterministically replaced with `[REDACTED]`.

Future adapters and secret providers must continue this rule at their boundaries.

## Sink Failure

The local executor surfaces audit sink failures. In v0, the workflow event may already be durably appended when an audit sink rejects the derived audit event. That behavior is intentional and documented: runtime callers see a structured error instead of a hidden audit gap, and operators can reconcile from the event log.

For pre-run policy decisions, the executor writes the durable policy audit record before sending it to the configured audit sink. If the sink fails, execution returns a structured audit error and no workflow run event stream is created.
