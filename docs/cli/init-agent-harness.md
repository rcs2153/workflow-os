# `workflow-os init-agent-harness`

`workflow-os init-agent-harness` creates local documentation scaffolding for using a coding agent with Workflow OS as the governing layer.

```sh
workflow-os init-agent-harness
```

It creates or updates:

- `AGENTS.md`
- `.workflow-os/agent-harness-prompt.md`

The command is scaffold-only. It does not validate projects, run workflows, approve checkpoints, execute local checks, register handlers, create reports, write runtime state, call providers, or change workflow schemas.

Generated downstream `AGENTS.md` instructions are portable. They ask agents to read the target repository's own engineering standard or contribution guide if one exists, plus `.workflow-os/README.md` and `.workflow-os/agent-harness-prompt.md`. They do not assume the downstream repository has Workflow OS's internal `docs/ENGINEERING_STANDARD.md`.

## Options

```text
workflow-os init-agent-harness [--output-dir <path>] [--agent generic|codex|claude] [--force] [--dry-run]
```

- `--output-dir <path>`: write scaffold files under the given directory. Defaults to `--project-dir`.
- `--agent generic|codex|claude`: tune the generated prompt label. Defaults to `generic`.
- `--force`: replace existing unmanaged scaffold files.
- `--dry-run`: show the files that would be written without writing them.

## File Safety

Generated files contain a managed block:

```text
<!-- BEGIN WORKFLOW OS AGENT HARNESS -->
...
<!-- END WORKFLOW OS AGENT HARNESS -->
```

If a target file exists with that managed block, the command updates only that block and preserves surrounding user content. If a target file exists without the managed block, the command fails closed unless `--force` is supplied.

Errors use stable codes and avoid echoing file contents, prompt text, or secret-like values.

## Boundary

The generated text centers the local adoption model:

```text
Agent executes. Workflow OS governs.
```

This command does not implement runtime harness auto-generation, nested harness execution, hosted orchestration, write-capable adapters, automatic local check execution, recursive agents, agent swarms, or Level 3/4 autonomy.
