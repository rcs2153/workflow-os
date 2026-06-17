# Self-Governed Build Benchmark CLI/Dev-Helper Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The repo-local `npm run dogfood:benchmark` helper improves the self-governed build benchmark operating loop without adding stable public Workflow OS CLI behavior or changing runtime semantics. It remains a thin development helper around existing `workflow-os` CLI commands and preserves the governing boundary:

```text
Agent executes. Workflow OS governs.
```

No blocker was found.

## 2. Scope Verification

The phase stayed within the approved repo-local helper scope.

No accidental implementation was found for:

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

## 3. Helper Surface Assessment

The helper surface is appropriately small and repo-local.

Implemented subcommands:

- `commands`;
- `validate`;
- `start`;
- `status`;
- `inspect`;
- `approve`;
- `prompt`.

The helper is exposed through `npm run dogfood:benchmark`, not a new Rust CLI subcommand. That is the right maturity level: it improves Workflow OS development ergonomics without turning the dogfood benchmark into core product surface area.

The helper wraps existing generic CLI commands:

- `workflow-os validate`;
- `workflow-os run`;
- `workflow-os status`;
- `workflow-os approve`;
- `workflow-os inspect`.

It does not bypass the kernel, synthesize runtime state, or introduce a parallel execution path.

## 4. Approval Boundary Assessment

Approval remains explicit.

The helper requires the caller to provide:

- run ID;
- approval ID;
- approval reason;
- optional actor.

It does not auto-discover approval IDs, approve after `start`, provide a hidden approval reason, or implement a one-shot command that validates, starts, approves, and inspects in a single ambient flow.

That is important because the benchmark is only valuable if approval remains a real governance checkpoint rather than a ceremony hidden inside convenience tooling.

## 5. Runtime And State Boundary Assessment

The helper preserves runtime semantics.

It mutates workflow state only by invoking existing CLI commands that already mutate state for the requested operation. It does not reset local state, delete state, write report artifacts, create reports, register handlers, or append extra events.

State directory handling is explicit enough for this phase:

- callers may pass `--state-dir`;
- omitted state uses the documented OS temp default;
- the helper prints the resolved state directory;
- the helper does not hide session state in a repo dotfile.

## 6. Build And Toolchain Assessment

The helper may build `target/debug/workflow-os` only when the binary is missing.

That is consistent with the plan because it uses the existing Rust build command and does not install toolchains, run `npm ci`, modify lockfiles, or broaden into arbitrary command execution. `--no-build` provides a fail-closed path for callers that want to require an existing binary.

Build success is not represented as benchmark validation.

## 7. Privacy And Redaction Assessment

The helper takes a good first privacy posture:

- approval reasons are redacted in displayed commands;
- repo-root paths are displayed as repo-relative paths;
- secret-like approval metadata is rejected before command construction;
- helper errors use stable helper labels for supported validation failures;
- the helper does not create evidence, WorkReports, report artifacts, or command-output evidence.

No privacy blocker was found.

One non-blocking hardening opportunity remains: unsupported command errors echo the unsupported command token. That is normal usage behavior, but a future hardening pass could avoid echoing arbitrary unknown command text if this helper is ever used in less trusted contexts.

## 8. Test Quality Assessment

The focused tests are appropriate for this phase.

Covered:

- dry-run `start` command shape;
- explicit approval reason requirement;
- approval reason redaction in displayed commands;
- secret-like approval metadata rejection without value leakage;
- helper boundary text in `commands` output;
- display command repo-relative path behavior;
- secret-like value detector behavior.

Missing or shallow coverage, all non-blocking:

- no end-to-end `validate/start/approve/inspect` helper smoke test;
- no missing-binary `--no-build` test;
- no `status` or `inspect` dry-run command-shape tests;
- no `prompt` output test;
- no explicit unsupported-command non-leakage hardening test.

The current test set is enough for the approved helper implementation because the helper remains a thin wrapper and existing CLI behavior is already covered elsewhere.

## 9. Documentation Review

Documentation is aligned with the implementation.

Docs now state that:

- `npm run dogfood:benchmark` is implemented as repo-local development tooling;
- the helper wraps existing CLI commands;
- the helper is not a stable public product CLI command;
- approval remains explicit;
- automatic runtime report generation is not implemented;
- report rendering and report artifact writing are not implemented;
- automatic local check execution is not implemented;
- default `DocsCheckLocalHandler` registration is not implemented;
- arbitrary command execution is not implemented;
- schemas, writes, reasoning lineage, recursive agents, hosted execution, production self-hosting, and Level 3/4 autonomy are not implemented.

## 10. Blockers

No blockers.

## 11. Non-Blocking Follow-Ups

- Add a missing-binary `--no-build` test.
- Add dry-run command-shape tests for `status` and `inspect`.
- Add a `prompt` output test.
- Harden unsupported-command errors to avoid echoing arbitrary unknown command text.
- Consider a bounded end-to-end helper smoke test after the current uncommitted dogfood benchmark stack is reviewed and committed.

## 12. Recommended Next Phase

Recommended next phase: **commit the self-governed benchmark stack**.

This series now includes the benchmark plan review, runbook, runbook review, behavior tests, behavior-test review, CLI/dev-helper plan, implementation, and review. The next useful step is to commit the coherent self-governance benchmark stack before starting another implementation phase.

After commit, the next roadmap phase should be a focused helper hardening phase or hook disclosure model implementation, depending on whether maintainers want one more polish pass on dogfood ergonomics before returning to kernel primitives.

## 13. Validation

- `npm run dogfood:benchmark -- commands`: passed.
- `npm run dogfood:benchmark -- start --dry-run --no-build --run-id run/dogfood-helper-check`: passed.
- `npm run test:dogfood-helper`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

Full Rust workspace validation was not rerun for this review because the review only added documentation and the implementation under review changed JavaScript/docs helper behavior, not Rust runtime code.
