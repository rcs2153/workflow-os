# State Backends

Workflow OS runtime state is designed for stateless workers over durable state.
The state backend layer defines the contracts used by the local executor and
the explicit shared-state preview. It does not itself execute workflows, run a
hosted worker service, call adapters, or claim production storage readiness.

## Contracts

The Rust core defines:

- `EventLogStore`
- `RunSnapshotStore`
- `IdempotencyStore`
- `LockStore`
- `ApprovalStore`
- `ApprovalPresentationRecordStore`
- `ProjectStateStore`
- `PolicyAuditStore`
- `WorkReportArtifactStore`
- `SideEffectRecordStore`
- `StateBackend`
- `BackendHealthCheck`

`StateBackend` combines the individual stores and exposes a health check plus `rehydrate_run`, which reads durable events and replays them through the event-sourced run model.

`WorkReportArtifactStore` is deliberately separate from the aggregate `StateBackend` in the first implementation. It stores explicit `WorkReportArtifactRecord` values for already-generated reports, but report artifacts are not workflow events, are not run snapshots, and are not read during normal run rehydration.

`SideEffectRecordStore` is also deliberately separate from the aggregate `StateBackend` in the first implementation. It stores explicit validated `SideEffectRecord` values, but side-effect records are not workflow events, are not run snapshots, are not report artifacts, and are not read during normal run rehydration. Side-effect record persistence does not execute side effects, discover side effects automatically, call adapters, mutate providers, or enable writes.

## Event Log

The event log is the source of truth.

Backend implementations must:

- Append events without mutating prior events.
- Validate expected next sequence number before append.
- Validate state transition rules before append.
- Reject events after terminal states before append.
- Reject run identity mismatches before append, including workflow ID, schema version, workflow version, and spec hash.
- Reject duplicate event IDs.
- Reject duplicate sequence numbers for the same run.
- Return events ordered by sequence number.
- Preserve event metadata including run ID, workflow ID, schema version, workflow version, spec hash, timestamp, event ID, correlation ID, actor, and idempotency key where present.

`append_event` is the safe append boundary. Normal runtime code must write workflow run events through this API. Rehydration repeats the same validation defensively, but invalid streams should be rejected before persistence during normal backend/runtime usage.

## Snapshots

Snapshots are projections. They may be stored for faster reads, but they must remain replaceable by replaying the event log.

If snapshot state and event history disagree, event history is authoritative.

## Local Filesystem Backend

The v0 local development backend stores JSON files under a configured state root:

```text
events/
event_ids/
snapshots/
idempotency/
locks/
approvals/
projects/
work_reports/
side_effects/
```

Approval projections under `approvals/` are not authoritative. They are written after `ApprovalRequested` is appended and can be rebuilt from the event-derived run snapshot. Runtime approval decisions must validate against the event-backed approval request. A projection without a matching event-backed request cannot authorize an approval decision.

Event append uses local filesystem creation semantics and a local lock directory to reject duplicate IDs and duplicate run sequence numbers. Event JSON files are written to temporary files, synced, and then published into place so readers do not observe partially written completed event files.

The local backend treats per-run sequence event files as the source of truth and `event_ids/` as a required consistency index. Append preflights existing event/index consistency and replays current durable history before accepting a new event. That replay enforces the expected next sequence number, immutable run identity, state transition rules, terminal-state rejection, and idempotency-key requirements. Reads verify that every returned event has a matching event ID index. Health checks scan both directions:

- event file without event ID index is reported as corrupt local state
- event ID index without event file is reported as corrupt local state
- event ID index pointing to the wrong run or sequence is reported as corrupt local state
- malformed event or index JSON is reported as corrupt local state

If an append is interrupted between publishing the index and publishing the event file, the next append or health check reports the dangling index clearly. Duplicate event IDs and duplicate run sequence numbers are rejected deterministically. v0 does not attempt automatic repair because silent repair could hide write-order failures during local kernel development.

This is suitable for local development and contract tests, not distributed production coordination. It is not equivalent to transactional database durability, multi-host locking, replication, or crash-consistent storage across arbitrary filesystems.

Local event records must include schema version. v0 does not silently default schema version for legacy event JSON because that would weaken the immutable run identity contract.

Report artifacts under `work_reports/` are explicit local handoff artifacts. They are written only through `WorkReportArtifactStore`, not by `LocalExecutor::execute(...)` or `LocalExecutor::execute_with_report(...)`. They do not append workflow events, mutate snapshots, change terminal status, or add CLI rendering/export behavior.

Side-effect records under `side_effects/` are explicit local governance records. They are written only through `SideEffectRecordStore`, not by normal workflow execution or report generation. They do not append workflow events, mutate snapshots, change terminal status, call providers, invoke adapters, write external systems, or provide automatic discovery for WorkReports or evidence references.

## In-Memory Test Backend

The in-memory backend exists only under Rust tests. It is not exported as a runtime backend and must not be used as a source of truth for real execution.

## Embedded SQLite Backend

The explicit embedded `SQLite` adapter implements the accepted durable-state
semantic contract for one local machine. It supports atomic state families,
revisions, schema posture, health and integrity checks, and guarded
filesystem-to-SQLite staging with exact-receipt activation.

It remains an opt-in local backend. Activation marks a verified destination
ready but does not automatically select it for runtime use or remove the
filesystem source. `SQLite` does not provide shared-worker coordination across
machines.

## Shared PostgreSQL Backend

`PostgresStateBackend` is an explicit opt-in shared-state preview. Callers
provide a `PostgresConnectionFactory`; the backend does not store or render a
connection URL. `PostgresNoTlsConnectionFactory` is named and documented for
loopback local/CI use only. Production callers must supply reviewed TLS,
credential, timeout, and pooling behavior behind the factory.

The backend provides:

- ordered authoritative event append and deterministic reads;
- existing approval, presentation, project, policy, telemetry, artifact,
  SideEffect, snapshot, idempotency, and lock store contracts;
- all seven Core transaction families;
- serializable bounded retries for serialization/deadlock conflicts;
- compare-and-set revisions;
- expiring database-time leases with fencing tokens;
- immutable run-bundle publication and verified reads;
- one explicit shared run-event consumer;
- deterministic projection rebuild and bounded health posture;
- CI conformance against `PostgreSQL` 17 and a logical backup/restore
  integrity rehearsal.

The shared consumer is an explicit library path, not an automatic worker,
daemon, scheduler, hosted API, or default executor. A failed consumer commit
does not release its lease early; the lease expires according to database time
so a later worker can take over with a higher fencing token.

The recovery rehearsal uses maintained `pg_dump`, `pg_restore`, and `psql`
tools to restore a separate database, validate schema health, rebuild
projections, and read an immutable run bundle. See
[PostgreSQL State Recovery](postgresql-state-recovery.md).

## Production Backends And Operations

The explicit `PostgreSQL` adapter is not a production deployment claim.
Workflow OS does not yet provide reviewed production TLS wiring, connection
pooling, replication, high availability, point-in-time recovery, capacity
testing, production SLOs, hosted workers, multi-tenancy, or tenant isolation.
Redis, SQS, NATS, and a general distributed queue are not implemented. Future
operational and hosted layers must preserve the same event-sourced invariants
and fail closed when required state capabilities are unavailable.
