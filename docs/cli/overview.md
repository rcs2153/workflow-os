# CLI Overview

`workflow-os` is the primary v0 user interface for local Workflow OS projects.

The CLI is local-first. It loads files from a project directory, validates them through the Rust core, runs the minimal local executor, and stores runtime state in the local state backend. It does not require external services.

## Global Options

```text
workflow-os [--project-dir <path>] [--state-dir <path>] [--json] [--mock-all-local-skills] <command>
```

- `--project-dir <path>`: project root containing `workflow-os.yml`. Defaults to the current directory.
- `--state-dir <path>`: local state backend root. Defaults to `.workflow-os/state` under the project directory.
- `--json`: emits experimental preview JSON where implemented. In `0.1.0-preview.1`, JSON output is not a versioned stable machine-output contract.
- `--mock-all-local-skills`: registers deterministic mock handlers for eligible `local/*` skills. This is a local development/example convenience, not proof that declared skills have real implementations.

## Commands

- `workflow-os validate`
- `workflow-os run <workflow-id>`
- `workflow-os status <run-id>`
- `workflow-os approve <run-id> <approval-id>` to grant
- `workflow-os approve <run-id> <approval-id> --deny --reason <reason>` to deny
- `workflow-os inspect <run-id>`
- `workflow-os doctor`

## v0 Runtime Scope

`run` and `approve` use the v0 local executor. They support single-step local workflows and explicitly registered local skill handlers only. Approval denial fails the run closed and does not execute the gated skill. The CLI does not implement real GitHub, Jira, CI, hosted, or external adapter integrations.

## JSON Output Compatibility

`--json` exists for local preview automation, smoke tests, and contract checks. Its response shapes are intentionally documented as experimental in `0.1.0-preview.1`; fields, enum formatting, and envelopes may change in later v0 releases with release notes. Public users should not treat CLI JSON as a stable integration contract until a future release introduces a versioned response envelope and compatibility tests for every command.
