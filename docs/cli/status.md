# `workflow-os status`

Reads a run from the local backend, rehydrates it from durable events, and prints the current projection.

```text
workflow-os status <run-id>
workflow-os --json status <run-id>
```

Text output includes:

- run ID
- schema version
- status
- current step where available
- terminal marker for completed, failed, or canceled runs
- last event ID
- last event timestamp

The command does not mutate state.

`--json` output is experimental in `0.1.0-preview.1`. It is useful for preview automation, but it is not yet a versioned stable machine-output contract.
