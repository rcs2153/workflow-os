# `workflow-os inspect`

Shows a run history summary from the local backend.

```text
workflow-os inspect <run-id>
workflow-os --json inspect <run-id>
```

Text output includes:

- workflow ID
- schema version
- workflow version
- spec hash
- run status
- event sequence, event ID, and event kind summary
- approval, retry, and escalation counts

Sensitive-looking output references are redacted. The command does not print raw skill input values.

The command does not mutate state.

`--json` output is experimental in `0.1.0-preview.1`. It is useful for preview automation, but it is not yet a versioned stable machine-output contract.
