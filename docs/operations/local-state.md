# Local State

The v0 local state backend is for development and future local execution. It is not a production database.

## Location

Callers choose the local state root when constructing `LocalStateBackend`. The backend creates:

```text
events/
event_ids/
snapshots/
idempotency/
locks/
approvals/
projects/
```

## What Is Stored

The backend stores:

- Workflow run events as JSON files.
- Event ID index records.
- Snapshot projections.
- Idempotency result references.
- Local lock records.
- Approval request projections.
- Non-secret project metadata.

Workflow specs remain project files. Secrets must not be stored in specs or local state result summaries.

## Recovery

Current run state should be recovered by reading events and rehydrating the run snapshot. Snapshots are projections and may be rebuilt from events.

If stored JSON is corrupted, the backend returns a structured `state.corrupt` error rather than silently ignoring the file.

## Backup And Restore

For local development, back up the entire configured state root while no `workflow-os run` or `workflow-os approve` command is active. The state root contains event history, event ID indexes, snapshots, idempotency records, locks, approvals, and project metadata projections.

Restore by placing the full directory back at the configured state path. After restore, use `workflow-os inspect <run-id>` or `workflow-os status <run-id>` to confirm event rehydration succeeds.

Do not restore partial event directories unless you are intentionally performing manual forensic recovery. Partial restores can cause rehydration to fail because event sequences must remain contiguous.

## Cleanup

For local development, state can be removed by deleting the configured local state root. This deletes event history, projections, idempotency records, and locks.

If a process exits while holding a local lock, remove the stale lock directory only after confirming no active process is using that state root.

## Production Boundary

The local backend does not provide distributed durability, distributed locks, replication, backups, or multi-host coordination. Production backends must be implemented separately and pass the shared backend contract tests.
