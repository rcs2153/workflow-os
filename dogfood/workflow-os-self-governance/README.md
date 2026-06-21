# Workflow OS Self-Governance Dogfood Project

This project is the first Workflow OS dogfooding slice for building Workflow OS itself and the project backing the [Self-Governed Build Benchmark](../../docs/user-guide/self-governed-build-benchmark.md).

It uses the local Workflow OS kernel as a sequential multi-step governance wrapper for a planning/docs task. The kernel validates the dogfood specs, creates a durable local run, records a scope checkpoint, pauses for human approval, records policy and approval events, resumes the run, completes downstream checkpoints, and leaves inspectable event history.

This is **kernel-governed, Codex-executed** dogfooding. Codex or a human still performs the actual repository edits outside the kernel. The dogfood workflow can exercise the docs check only when a caller explicitly supplies and registers `DocsCheckLocalHandler`; it is not default, CLI-enabled, schema-driven, or automatic. The dogfood workflow does not execute arbitrary build commands, mutate repository files, call external systems, run recursive agents, or replace human review.

Use this project as the reference pattern for kernel-governed agent work:

```text
Agent executes. Workflow OS governs.
```

The agent should use the kernel to validate, start or resume the governed workflow, respect approval checkpoints, execute only the scoped repository work, run required validation commands outside the kernel unless an explicit handler exists, and report completed/deferred work honestly.

## What It Demonstrates

- Workflow OS can govern Workflow OS planning/docs work through its own local kernel.
- The workflow is Level 2 and approval-gated.
- The workflow uses sequential local multi-step execution.
- The run is local, deterministic, auditable, and inspectable.
- The dogfood checkpoints separate scope, planning approval, implementation handoff, validation disclosure, explicit docs-check execution, and review/report posture.
- The implementation workflow governs bounded implementation phases from context readiness through report handoff.
- The review workflow governs phase-level maintainer reviews from context inspection through blocker/follow-up classification.
- The PR hygiene workflow governs main-sync disclosure, hot-file conflict risk, approval before PR staging, validation disclosure, conflict-resolution disclosure, and PR readiness reporting.

The conversion is documented in [Self-Governance Dogfood Multi-Step Conversion Plan](../../docs/implementation-plans/self-governance-dogfood-multi-step-conversion-plan.md).

## What It Does Not Do

- It does not execute arbitrary shell commands.
- It does not run Rust or npm checks from inside the kernel unless a reviewed explicit local check handler is supplied by the caller.
- It does not perform code or documentation edits.
- It does not use live adapters or external services.
- It does not execute branching, parallel, or nested harness behavior.
- It does not implement recursive agents, agent swarms, or generic orchestration.
- It does not claim production self-hosting.

## Benchmark Runbook

Use this runbook for material Workflow OS roadmap phases. The dogfood workflow governs the phase; Codex, Claude Code, or a human performs the repository work.

Before work:

1. Read `docs/ENGINEERING_STANDARD.md`, `ROADMAP.md`, and the relevant plan/report/review docs.
2. Validate this dogfood project.
3. Start or resume a governed dogfood run.
4. Treat the planning approval checkpoint as mandatory.
5. Keep the task inside the approved phase scope.

During work:

1. Perform edits outside the kernel.
2. Run required validation commands outside the kernel unless an explicit reviewed local handler exists.
3. Do not claim manual commands were kernel-executed.
4. Do not invent run IDs, approvals, evidence references, audit events, local check results, WorkReports, or command output.
5. Do not claim automatic local checks, write-capable adapters, recursive agents, hosted execution, production self-hosting, or Level 3/4 autonomy.

Before handoff:

1. Inspect the governed run when applicable.
2. Report the run status, approval/checkpoint context, validation commands, failures, deferred scope, and next phase.
3. Create or update the structured implementation/review report required by the phase.
4. Stop for a blocker-fix or planning phase if validation, approval, report generation, or scope boundaries fail.

The benchmark plan is documented in [Self-Governed Build Benchmark Plan](../../docs/implementation-plans/self-governed-build-benchmark-plan.md) and accepted in [Self-Governed Build Benchmark Plan Review](../../docs/concepts/SELF_GOVERNED_BUILD_BENCHMARK_PLAN_REVIEW.md).

## Run It

From the repository root:

```sh
npm run dogfood:benchmark -- commands
npm run dogfood:benchmark -- validate
npm run dogfood:benchmark -- start
npm run dogfood:benchmark -- approve <run-id> <approval-id> --reason reviewed-governance-task
npm run dogfood:benchmark -- inspect <run-id>
```

This repo-local helper wraps the generic `workflow-os` CLI commands below. It is development tooling only: it does not approve automatically, register default local check handlers, run arbitrary commands, write report artifacts, render reports, or change runtime semantics.

## Dogfood Workflow Suite

Use the dogfood workflow that matches the work shape:

| Workflow | Use for | Boundary |
| --- | --- | --- |
| `dg/d` | Planning/docs benchmark work and legacy self-governed phases | Generic benchmark workflow |
| `dg/implement` | Accepted implementation phases | Governs scope, context, approval, implementation handoff, validation disclosure, and implementation report posture |
| `dg/review` | Maintainer reviews and blocker-fix reviews | Governs review context, review approval, scope verification, validation assessment, findings classification, and review report posture |
| `dg/pr` | PR preparation and conflict avoidance | Governs main-sync disclosure, hot-file risk, validation disclosure, conflict resolution, and PR readiness |

These workflows are kernel-governed and Codex/human-executed. They do not perform code edits, run arbitrary shell commands, inspect GitHub, create branches, open PRs, push commits, mutate Workflow OS state by hand, approve themselves, or replace maintainer judgment.

Start implementation phase governance:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-implementation-state \
  --mock-all-local-skills \
  run dg/implement
```

Start maintainer review governance:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-review-state \
  --mock-all-local-skills \
  run dg/review
```

## PR Hygiene Workflow

The `dg/pr` workflow is the maintained dogfood wrapper for PR preparation and conflict avoidance. It exists because public branch protection and frequent roadmap/doc updates make branch drift a real governance risk.

Use it before staging a PR or after a branch has fallen behind `main`. It governs these checkpoints:

- disclose that `main` was fetched and integrated before PR work;
- scope hot-file conflict risk, especially `ROADMAP.md`, concept docs, executor/lib files, and local executor tests;
- require approval before branch work or PR staging proceeds;
- disclose that Codex or a human performed git operations outside the kernel;
- disclose validation commands and any skipped checks;
- disclose merge or rebase results and any conflict resolution before push;
- report branch, commit, PR URL, mergeability, and validation status before handoff.

The workflow does not run `git`, install GitHub tooling, merge, rebase, push, open PRs, inspect GitHub, resolve conflicts, mutate repository files, append post-terminal events outside the run, persist reports, or execute CLI output on behalf of the agent. It is a governed checklist with durable run identity and approval checkpoints; Codex or a human still performs repository operations.

Manual command sequence:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-pr-hygiene-state \
  --mock-all-local-skills \
  run dg/pr
```

Approve the explicit checkpoint when the PR scope is understood:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-pr-hygiene-state \
  --mock-all-local-skills \
  approve <run-id> <approval-id> \
  --actor user/dogfood-reviewer \
  --reason reviewed-pr-hygiene-scope
```

Manual command sequence:

```sh
cargo build -p workflow-cli --bin workflow-os
```

Validate the dogfood project:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  validate
```

Start the governance workflow using an external state directory:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-self-governance-state \
  --mock-all-local-skills \
  run dg/d
```

The run should execute the `scope-requested` checkpoint, pause at `planning-approved`, and print a `run_id` plus `approval_id`.

Approve and resume the downstream placeholder checkpoints:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-self-governance-state \
  --mock-all-local-skills \
  approve <run-id> <approval-id> \
  --actor user/dogfood-reviewer \
  --reason reviewed-governance-task
```

Inspect the completed multi-step governance run:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-self-governance-state \
  inspect <run-id>
```

## Use It With Codex Or Claude Code

After starting the governed run, paste the setup prompt from [Agent Harness Quickstart](../../docs/user-guide/agent-harness-quickstart.md) into Codex or Claude Code.

The agent should:

- read the required engineering and roadmap context;
- treat the dogfood run as the governing wrapper for the task;
- ask for approval when the kernel pauses;
- stay inside the approved phase scope;
- run validation commands requested by the phase outside the kernel unless a real local handler is explicitly registered;
- avoid inventing run state, approvals, evidence, audit events, or reports;
- produce the structured implementation or review report expected by the phase.

This dogfood path is not automatic build execution, recursive agent orchestration, agent swarming, production self-hosting, or Level 3/4 autonomy.

## Operating Boundary

Use this project to prove governance around Workflow OS work, not automation ownership of Workflow OS work. The docs-check checkpoint is explicit-handler-only: ordinary registry construction still fails closed, CLI mock runs still use deterministic mocks, and broader validation/build commands remain outside the kernel until separately scoped.
