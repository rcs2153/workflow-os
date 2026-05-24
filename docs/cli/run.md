# `workflow-os run`

Validates and runs a local workflow through the v0 local executor.

```text
workflow-os run <workflow-id>
workflow-os run <workflow-id> --run-id <run-id>
workflow-os --json run <workflow-id>
```

The command:

- loads the project
- runs semantic validation
- creates a local run unless `--run-id` points at an existing durable run
- persists runtime events to the local backend
- prints the run ID and current status

## v0 Local Skill Handler

The initial CLI registers deterministic local handlers for specs whose skill ID starts with `local/` and that do not declare adapter requirements. This is only for local v0 execution. Real adapter execution is not implemented.

Commands do not bypass validation, policy, audit, or durable runtime events.
