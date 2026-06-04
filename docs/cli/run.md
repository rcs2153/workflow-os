# `workflow-os run`

Validates and runs a local workflow through the v0 local executor.

```text
workflow-os run <workflow-id>
workflow-os run <workflow-id> --run-id <run-id>
workflow-os --mock-all-local-skills run <workflow-id>
workflow-os --json run <workflow-id>
```

The command:

- loads the project
- runs semantic validation
- creates a local run unless `--run-id` points at an existing durable run
- persists runtime events to the local backend
- prints the run ID and current status

## v0 Local Skill Handler

The CLI does not treat every declared `local/*` skill as implemented. By default, an executable workflow step must have a registered local handler. If no handler is registered, the run fails closed with `executor.skill_handler.missing`.

For local examples and development-only smoke tests, `--mock-all-local-skills` registers deterministic mock handlers for specs whose skill ID starts with `local/` and that do not declare adapter requirements. Mock output references use the `mock-local-cli-output/...` prefix so event history and audit projections do not look like production skill output.

This flag is a convenience for proving the v0 kernel path. It is not a plugin system, real skill implementation, AI model call, or adapter execution.

Commands do not bypass validation, policy, audit, or durable runtime events.

`--json` output remains experimental through `0.2.0-preview.1`. It is useful for preview automation, but it is not yet a versioned stable machine-output contract.
