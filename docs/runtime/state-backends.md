# State Backends

Workflow OS runtime state is designed for stateless workers over durable state. The v0 state backend layer defines the contracts needed for restart-safe local execution later. It does not execute workflows, run workers, call adapters, or implement production storage.

## Contracts

The Rust core defines:

- `EventLogStore`
- `RunSnapshotStore`
- `IdempotencyStore`
- `LockStore`
- `ApprovalStore`
- `ProjectStateStore`
- `StateBackend`
- `BackendHealthCheck`

`StateBackend` combines the individual stores and exposes a health check plus `rehydrate_run`, which reads durable events and replays them through the event-sourced run model.

## Event Log

The event log is the source of truth.

Backend implementations must:

- Append events without mutating prior events.
- Reject duplicate event IDs.
- Reject duplicate sequence numbers for the same run.
- Return events ordered by sequence number.
- Preserve event metadata including run ID, workflow ID, workflow version, spec hash, timestamp, event ID, correlation ID, actor, and idempotency key where present.

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
```

Event append uses local filesystem creation semantics and a local lock directory to reject duplicate IDs and duplicate run sequence numbers. This is suitable for local development and contract tests, not distributed production coordination.

## In-Memory Test Backend

The in-memory backend exists only under Rust tests. It is not exported as a runtime backend and must not be used as a source of truth for real execution.

## Production Backends

Postgres, Redis, SQS, NATS, and distributed locking are intentionally not implemented in v0. Future production backends must pass the same contract tests and preserve the same event-sourced invariants.
