# Audit Log

The v0 local runtime emits `AuditEvent` records through an `AuditSink`.

## Local Sink

`LocalAuditSink` stores audit events in process memory for local development and tests. It is useful for verifying runtime behavior but is not a production audit store.

Production-shaped backends must provide their own sink implementation that preserves:

- append-only audit semantics where practical
- correlation ID
- actor or system actor
- workflow/run identity
- spec hash
- policy decision context
- idempotency key where relevant
- redaction metadata

## Failure Behavior

Audit sink failures are not hidden. The executor returns a structured error when the sink rejects an audit event. Since the workflow event log is the source of truth, operators can reconcile local audit records from durable workflow events if a sink fails after an event append.

## Operator Use

Operators should review audit records when:

- a policy decision denies execution
- a run waits for, grants, or denies approval
- retry exhaustion escalates a run
- cancellation occurs
- an adapter-capability action is introduced in future versions

Audit records must not be used to store raw payloads or secrets.
