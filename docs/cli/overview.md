# CLI Overview

`workflow-os` is the primary v0 user interface for local Workflow OS projects.

The CLI is local-first. It loads files from a project directory, validates them through the Rust core, runs the minimal local executor, and stores runtime state in the local state backend. It does not require external services.

## Global Options

```text
workflow-os [--project-dir <path>] [--state-dir <path>] [--json] [--mock-all-local-skills] <command>
```

- `--project-dir <path>`: project root containing `workflow-os.yml`. Defaults to the current directory.
- `--state-dir <path>`: local state backend root. Defaults to `.workflow-os/state` under the project directory.
- `--json`: emits experimental preview JSON where implemented. Through `0.2.0-preview.1`, JSON output is not a versioned stable machine-output contract.
- `--mock-all-local-skills`: registers deterministic mock handlers for eligible `local/*` skills. This is a local development/example convenience, not proof that declared skills have real implementations.

## Commands

- `workflow-os validate`
- `workflow-os run <workflow-id>`
- `workflow-os status <run-id>`
- `workflow-os approve <run-id> <approval-id>` to grant
- `workflow-os approve <run-id> <approval-id> --deny --reason <reason>` to deny
- `workflow-os inspect <run-id>`
- `workflow-os doctor`
- `workflow-os init-agent-harness`

## v0 Runtime Scope

`run` and `approve` use the v0 local executor. They support sequential local workflows and explicitly registered local skill handlers only. Approval denial fails the run closed and does not execute the gated skill. The CLI does not expose generic live adapter execution commands; the GitHub reference example uses an explicit fixture-only local handler. Branching, parallelism, CI, hosted, distributed, and write-capable adapter workflows are not implemented.

`init-agent-harness` is documentation/scaffold-only. It writes `AGENTS.md` and `.workflow-os/agent-harness-prompt.md` with Workflow OS managed blocks so users can point Codex, Claude Code, or another coding agent at the local kernel as the governing layer. It does not run workflows, approve checkpoints, execute local checks, register handlers, write runtime state, create report artifacts, or change schemas.

## JSON Output Compatibility

`--json` exists for local preview automation, smoke tests, and contract checks. Its response shapes remain experimental through `0.2.0-preview.1`; fields, enum formatting, and envelopes may change in later v0 releases with release notes. Public users should not treat CLI JSON as a stable integration contract until a future release introduces a versioned response envelope and compatibility tests for every command.
