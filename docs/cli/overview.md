# CLI Overview

`workflow-os` is the primary v0 user interface for local Workflow OS projects.

The CLI is local-first. It loads files from a project directory, validates them through the Rust core, runs the minimal local executor, and stores runtime state in the local state backend. It does not require external services.

## Global Options

```text
workflow-os [--project-dir <path>] [--state-dir <path>] [--json] <command>
```

- `--project-dir <path>`: project root containing `workflow-os.yml`. Defaults to the current directory.
- `--state-dir <path>`: local state backend root. Defaults to `.workflow-os/state` under the project directory.
- `--json`: emits machine-readable output where implemented.

## Commands

- `workflow-os validate`
- `workflow-os run <workflow-id>`
- `workflow-os status <run-id>`
- `workflow-os approve <run-id> <approval-id>`
- `workflow-os inspect <run-id>`
- `workflow-os doctor`

## v0 Runtime Scope

`run` and `approve` use the v0 local executor. They support single-step local workflows and local skill handlers only. The CLI does not implement real GitHub, Jira, CI, hosted, or external adapter integrations.
