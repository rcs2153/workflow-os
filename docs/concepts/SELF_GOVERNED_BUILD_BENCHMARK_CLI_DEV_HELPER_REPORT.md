# Self-Governed Build Benchmark CLI/Dev-Helper Report

## 1. Executive Summary

The repo-local self-governed build benchmark dev-helper phase is complete.

The implementation adds `scripts/self-governed-benchmark.mjs` and npm alias `dogfood:benchmark`. The helper wraps existing `workflow-os` CLI commands for the dogfood benchmark and makes the local governance loop easier to operate without adding a stable public Rust CLI command or changing runtime behavior.

The boundary remains:

```text
Agent executes. Workflow OS governs.
```

## 2. Scope Completed

- Added repo-local helper script `scripts/self-governed-benchmark.mjs`.
- Added npm alias `dogfood:benchmark`.
- Added focused Node tests in `scripts/self-governed-benchmark.test.mjs`.
- Added npm alias `test:dogfood-helper`.
- Implemented helper subcommands:
  - `commands`;
  - `validate`;
  - `start`;
  - `status`;
  - `inspect`;
  - `approve`;
  - `prompt`.
- Added `--dry-run`, `--no-build`, `--state-dir`, `--run-id`, `--actor`, `--reason`, and transparent `--json` passthrough where applicable.
- Kept approval explicit: `approve` requires run ID, approval ID, and reason.
- Redacted approval reasons in displayed command output.
- Rejected secret-like approval metadata without echoing raw values.
- Updated benchmark docs, dogfood README, roadmap, and planning status.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- stable public Workflow OS CLI benchmark commands;
- automatic runtime report generation;
- runtime result exposure changes;
- CLI report rendering;
- report artifact writing;
- automatic report artifact writing;
- automatic local check execution;
- default `DocsCheckLocalHandler` registration;
- arbitrary shell execution;
- command-output evidence attachment;
- workflow schema changes;
- workflow-declared benchmark behavior;
- workflow-declared hooks;
- runtime hook configuration;
- hook warning/skipped continuation;
- approval evidence attachment;
- reasoning lineage or claim graph;
- side-effect boundary enforcement;
- write-capable adapters;
- repository writes from inside the kernel;
- recursive agents;
- agent swarms;
- hosted or distributed runtime claims;
- production self-hosting claims;
- Level 3 or Level 4 autonomy claims;
- release posture changes.

## 4. Helper API Summary

The helper is invoked through:

```sh
npm run dogfood:benchmark -- <command>
```

Supported commands:

```sh
npm run dogfood:benchmark -- commands
npm run dogfood:benchmark -- validate
npm run dogfood:benchmark -- start [--state-dir <path>] [--run-id <run-id>]
npm run dogfood:benchmark -- status <run-id> [--state-dir <path>]
npm run dogfood:benchmark -- inspect <run-id> [--state-dir <path>]
npm run dogfood:benchmark -- approve <run-id> <approval-id> --reason <reason> [--actor <actor>] [--state-dir <path>]
npm run dogfood:benchmark -- prompt
```

The helper uses the dogfood project at `dogfood/workflow-os-self-governance` and workflow ID `dg/d`.

If `target/debug/workflow-os` is missing, the helper may build it with:

```sh
cargo build -p workflow-cli --bin workflow-os
```

`--no-build` fails closed when the binary is missing. `--dry-run` prints the bounded command shape without executing it.

## 5. Approval And State Behavior

Approval remains explicit and operator-controlled.

The helper does not auto-discover approval IDs, approve immediately after start, or infer an approval reason. The operator must pass:

- run ID;
- approval ID;
- `--reason`;
- optional actor.

The helper accepts explicit `--state-dir`. When omitted, it uses the documented OS temp path default. It does not delete, reset, or hide state.

## 6. Runtime And CLI Boundary Summary

The helper is not new Workflow OS product CLI behavior. It is repository-local development tooling.

It calls existing generic CLI commands:

- `workflow-os validate`;
- `workflow-os run`;
- `workflow-os status`;
- `workflow-os approve`;
- `workflow-os inspect`.

It does not bypass validation, policy, audit, approvals, or event-sourced runtime state. It does not mutate runtime state except by invoking the existing command explicitly requested by the helper subcommand.

## 7. Privacy And Redaction Summary

The helper avoids leaking operator-provided approval context in displayed commands:

- displayed `--reason` values are replaced with `<redacted-reason>`;
- secret-like approval metadata is rejected before command construction;
- helper errors use stable helper labels and do not echo raw secret-like values.

The helper does not store raw command output, create evidence, create reports, write artifacts, inspect raw state files, call providers, or attach command-output evidence.

## 8. Test Coverage Summary

Focused tests cover:

- dry-run `start` command shape;
- explicit approval reason requirement;
- approval reason redaction in displayed commands;
- secret-like approval metadata rejection without value leakage;
- helper boundary text in `commands` output;
- direct display-command reason redaction;
- secret-like value detector behavior.

## 9. Commands Run And Results

- `npm run dogfood:benchmark -- commands`: passed.
- `npm run dogfood:benchmark -- start --dry-run --no-build --run-id run/dogfood-helper-check`: passed.
- `npm run test:dogfood-helper`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

Full Rust workspace validation was not required for this repo-local JavaScript/docs helper phase because no Rust code or runtime behavior changed.

## 10. Remaining Known Limitations

- The helper is repo-local development tooling, not a stable public CLI command.
- It still uses `--mock-all-local-skills` for the dogfood run path.
- It does not run real local checks by default.
- It does not register `DocsCheckLocalHandler`.
- It does not generate, render, or write WorkReports.
- It does not automatically propagate local check, typed handoff, or hook references into reports.
- It does not create named benchmark sessions.
- It does not implement command-output evidence.

## 11. Recommended Next Phase

Recommended next phase: **self-governed build benchmark CLI/dev-helper review**.

The review should verify that the helper improves local benchmark usability without becoming a product CLI command, bypassing approvals, registering default handlers, running arbitrary commands, writing reports/artifacts, adding schemas, enabling writes, or changing release posture.
