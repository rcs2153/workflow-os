# Workflow OS Self-Governance Dogfood Project

This project is the first Workflow OS dogfooding slice for building Workflow OS itself and the project backing the [Self-Governed Build Benchmark](../../docs/user-guide/self-governed-build-benchmark.md).

It uses the local Workflow OS kernel as a sequential multi-step governance wrapper for a planning/docs task. The kernel validates the dogfood specs, creates a durable local run, records a scope checkpoint, pauses for human approval, records policy and approval events, resumes the run, completes downstream checkpoints, and leaves inspectable event history.

This project is **not** the default workflow pack for Workflow OS users. The `dg/*` workflows are Workflow OS's own self-governance workflows, shaped around this repository's roadmap, branch, PR, release, blocker-fix, and workflow-discovery needs. Treat them as reference patterns for kernel-governed work, not as community-required workflows or portable templates that every user should adopt unchanged.

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
- The runtime composition workflow governs phases that connect existing primitives into explicit runtime paths without creating new primitive families.
- The blocker-fix workflow governs focused blocker remediation from original finding restatement through regression validation.
- The release hygiene workflow governs public-preview readiness checks and release handoff without publishing from inside the kernel.
- The branch cleanup workflow governs merged-branch inventory, deletion-candidate review, explicit cleanup approval, post-cleanup validation, and cleanup reporting without deleting branches from inside the kernel.
- The workflow discovery workflow governs recommendation-only discovery of repeated work patterns, workflow gaps, overlap/conflict risk, and next workflow candidates without generating workflow files automatically.
- The spec field operationalization workflow governs work that turns scaffolded YAML fields into explicit validation, first-run disclosure, checks, or reviewed runtime behavior without letting rich fields become decorative metadata.

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
| `dg/runtime-composition` | Runtime-composition implementation phases | Governs primitive inventory, explicit integration path selection, approval, validation, and composition report posture |
| `dg/blocker` | Focused blocker fixes | Governs blocker restatement, narrow fix boundary, approval, regression validation, and fix report posture |
| `dg/release` | Release hygiene and public-preview readiness | Governs release scope, public docs checks, validation disclosure, release handoff, and readiness reporting |
| `dg/branch-cleanup` | Merged branch cleanup readiness | Governs repo-state readiness, main-sync disclosure, merged branch inventory, delete-candidate review, cleanup approval, cleanup handoff, post-cleanup validation, and cleanup reporting |
| `dg/workflow-discovery` | Recommendation-only workflow discovery | Governs signal-source inventory, repeated-pattern analysis, overlap/conflict review, recommendation drafting, stewardship approval, and discovery reporting |
| `dg/spec-field-operationalization` | Scaffold/spec field operationalization | Governs field inventory, posture classification, ownership/escalation review, deterministic check design, implementation-scope approval, output review, and final reporting |

These workflows are kernel-governed and Codex/human-executed. They do not perform code edits, run arbitrary shell commands, inspect GitHub, create branches, open PRs, push commits, generate workflow files, register workflows, mutate Workflow OS state by hand, approve themselves, or replace maintainer judgment.

They are not user starter assets. Existing-repo onboarding should use future scaffolds/templates, not these `dg/*` files.

The Phase 2 dogfood workflows are intentionally narrower than general-purpose automation:

- `dg/runtime-composition` is for connecting already-reviewed primitives into runtime paths, such as opt-in executor report/artifact composition. It should not be used to invent new model families or authorize writes.
- `dg/blocker` is for blocker fixes only. It should preserve the original review finding and avoid feature expansion.
- `dg/release` is for readiness and handoff. It does not tag releases, publish packages, change GitHub settings, or alter release posture by itself.
- `dg/branch-cleanup` is for branch cleanup governance only. It does not delete local branches, delete remote branches, prune remotes, force-push, inspect GitHub, or bypass branch protection. It requires explicit approval before a human or agent performs any branch deletion outside the kernel.
- `dg/workflow-discovery` is for recommendation-only workflow catalog stewardship. It does not generate workflow files, register workflows, mutate roadmap state, approve its own recommendations, or resolve workflow conflicts automatically.
- `dg/spec-field-operationalization` is for converting rich scaffold/spec fields into explicit enforcement, validation, disclosure, checks, or deferred posture. It does not change schemas, execute commands, create RBAC, notify escalation contacts, register workflows, generate workflows automatically, or authorize writes.

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

Start runtime composition governance:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-runtime-composition-state \
  --mock-all-local-skills \
  run dg/runtime-composition
```

Start blocker-fix governance:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-blocker-fix-state \
  --mock-all-local-skills \
  run dg/blocker
```

Start release hygiene governance:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-release-hygiene-state \
  --mock-all-local-skills \
  run dg/release
```

Start branch cleanup governance:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-branch-cleanup-state \
  --mock-all-local-skills \
  run dg/branch-cleanup
```

Start workflow discovery governance:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-workflow-discovery-state \
  --mock-all-local-skills \
  run dg/workflow-discovery
```

Start spec field operationalization governance:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-spec-field-operationalization-state \
  --mock-all-local-skills \
  run dg/spec-field-operationalization
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

## Branch Cleanup Workflow

The `dg/branch-cleanup` workflow is the maintained dogfood wrapper for merged-branch cleanup. It exists because branch cleanup has real repository effects and should not happen from casual agent judgment.

Use it when local or remote feature branches have likely been merged and should be reviewed for cleanup. It governs these checkpoints:

- disclose the current branch and working-tree state before cleanup;
- disclose whether `main` was fetched and whether local `main` is current;
- inventory local and remote branches that appear merged to `main`;
- present delete candidates and non-candidates before any deletion;
- require explicit approval before local or remote branch deletion;
- disclose that Codex or a human performs approved git cleanup outside the kernel;
- disclose post-cleanup branch state and skipped deletions;
- report candidates, actions, skips, validation, and the next step.

The workflow does not run `git`, delete local branches, delete remote branches, prune remotes, inspect GitHub, force-push, mutate repository files, persist reports, or bypass branch protection. It is a governed cleanup checklist with durable run identity and approval checkpoints; Codex or a human still performs repository operations.

Manual command sequence:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-branch-cleanup-state \
  --mock-all-local-skills \
  run dg/branch-cleanup
```

Approve only after the deletion candidates are understood:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-branch-cleanup-state \
  --mock-all-local-skills \
  approve <run-id> <approval-id> \
  --actor user/dogfood-reviewer \
  --reason reviewed-branch-cleanup-candidates
```

## Workflow Discovery Workflow

The `dg/workflow-discovery` workflow is the maintained dogfood wrapper for discovering new workflow needs from repeated Workflow OS build friction. It exists because the kernel should eventually recommend governed workflows when patterns become durable enough, instead of requiring humans to hand-create every workflow.

Use it after repeated manual handoffs, review findings, branch hygiene issues, conflict patterns, validation/check gaps, report limitations, or recurring roadmap pivots. It governs these checkpoints:

- disclose the discovery window, source set, and recommendation-only boundary;
- inventory recent reports, roadmap updates, dogfood runs, and repeated manual handoffs;
- identify repeated work patterns, missing gates, missing evidence, unclear handoffs, and manual workarounds;
- flag overlap, conflict, duplicate authority, lifecycle risk, and stale workflow candidates;
- draft bounded recommendations with rationale, evidence references where available, and explicit non-goals;
- require stewardship approval before recommendations become roadmap or implementation handoff;
- hand off accepted, rejected, and deferred recommendations;
- report signals, recommendations, conflicts, deferrals, and the next step.

The workflow does not generate workflow files, register workflows, mutate specs, modify roadmap state, inspect GitHub, execute commands, approve itself, or resolve workflow conflicts automatically. It is a governed discovery checklist with durable run identity and approval checkpoints; Codex or a human still performs repository operations and any accepted follow-up implementation.

Manual command sequence:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-workflow-discovery-state \
  --mock-all-local-skills \
  run dg/workflow-discovery
```

Approve only after the recommendation scope is understood:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-workflow-discovery-state \
  --mock-all-local-skills \
  approve <run-id> <approval-id> \
  --actor user/dogfood-reviewer \
  --reason reviewed-workflow-discovery-recommendations
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
