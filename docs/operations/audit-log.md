# Audit Log

The v0 local runtime emits `AuditEvent` records through an `AuditSink` for workflow events and `PolicyAuditRecord` records for policy decisions.

Policy decisions are also written to the local durable policy audit ledger. This ledger covers both run-scoped policy decisions and decisions that happen before `RunCreated`, such as denied starts.

## Local Sink

`LocalAuditSink` stores audit events in process memory for local development and tests. It is useful for verifying runtime behavior but is not a production audit store.

The local state backend stores durable policy audit records under its policy audit area. These records are local development artifacts, not a production SIEM replacement.

Production-shaped backends must provide their own sink implementation that preserves:

- append-only audit semantics where practical
- correlation ID
- actor or system actor
- workflow/run identity, including schema version
- spec hash
- step, skill, and skill version where relevant
- policy decision context
- pre-run denied-start records without fake workflow runs
- idempotency key where relevant
- redaction metadata describing safe, reference-only, and redacted fields

## Adapter Telemetry

Phase 2 read-only adapters produce **contract-level adapter telemetry** as `AdapterInvocationRecord` values. Those records are audit-safe and redacted, but the fixture-backed CLI examples do not yet durably persist them as first-class runtime audit records.

Operators inspecting Phase 2 examples should treat adapter invocation records as contract-test evidence from the adapter layer. The local runtime audit log remains authoritative for workflow events, policy decisions, approvals, skill invocation events, retries, escalations, and cancellations.

## Failure Behavior

Audit sink failures are not hidden. The executor returns a structured error when the sink rejects an audit event. Since the workflow event log is the source of truth, operators can reconcile local audit records from durable workflow events if a sink fails after an event append.

For pre-run policy decisions, the durable policy audit record is written before the configured audit sink is called. If the sink rejects the record, the command fails with a structured audit error and no workflow run event stream is created.

## Operator Use

Operators should review audit records when:

- a policy decision denies execution
- a run waits for, grants, or denies approval
- retry exhaustion escalates a run
- cancellation occurs
- an adapter-capability action is introduced in future versions

Audit records must not be used to store raw payloads or secrets.

The local v0 audit path is suitable for validating kernel behavior and local development. It is not a production SIEM, retention, export, or tamper-evident audit service.
