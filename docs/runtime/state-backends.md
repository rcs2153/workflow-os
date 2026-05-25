# State Backends

Workflow OS runtime state is designed for stateless workers over durable state. The v0 state backend layer defines the contracts used by the current local executor and needed by future workers. It does not execute workflows, run distributed workers, call adapters, or implement production storage.

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

## In-Memory Test Backend

The in-memory backend exists only under Rust tests. It is not exported as a runtime backend and must not be used as a source of truth for real execution.

## Production Backends

Postgres, Redis, SQS, NATS, and distributed locking are intentionally not implemented in v0. Future production backends must pass the same contract tests and preserve the same event-sourced invariants.
