# Next Roadmap Sprint Plan

Status: Planning accepted for sprint execution. This sprint plan uses the self-governed dogfood workflow as the governing wrapper for the phase. The dogfood project was validated, a governed run was started as `run/roadmap-next-sprint-plan`, the approval checkpoint `approval/run/roadmap-next-sprint-plan/planning-approved` was granted, and the governed run completed. This plan does not implement code, add runtime behavior, persist reports, expose CLI behavior, add schemas, enable writes, implement reasoning lineage, or change release posture.

## 1. Executive Summary

The project is not stuck in an endless planning loop, but it is carrying too many serial planning/review gates. The next sprint should split the work into:

1. **Immediate implementation lane**: hook disclosure discovery from already-validated in-memory hook results.
2. **Short review gate lane**: close the local-check side-effect boundary blocker-fix review.
3. **Parallel planning lane**: side-effect boundary, high-assurance approvals, and write-adapter readiness planning in parallel, not sequentially.

The exit from planning is explicit:

- after hook disclosure discovery plan review, move directly to code;
- after local-check boundary blocker-fix review, move directly to opt-in live DocsCheck smoke implementation;
- broader write-capable adapter work still requires side-effect boundary, high-assurance approval, evidence/report citation, and first write-adapter readiness planning, but those planning phases should run in parallel where possible.

## 2. Kernel Dogfood Context

The sprint planning phase was governed by the local dogfood workflow.

Kernel actions completed:

- `npm run dogfood:benchmark -- validate` passed with expected experimental lifecycle warnings.
- `npm run dogfood:benchmark -- start --run-id run/roadmap-next-sprint-plan` started the governed run.
- `npm run dogfood:benchmark -- approve run/roadmap-next-sprint-plan approval/run/roadmap-next-sprint-plan/planning-approved --reason reviewed-next-roadmap-sprint-plan` granted the approval.
- Current dogfood run status: `Completed`.
- Approval checkpoint: `approval/run/roadmap-next-sprint-plan/planning-approved` was granted.

Boundary:

- The kernel validated and paused the governed planning workflow.
- Codex performed repository inspection and plan drafting outside the kernel.
- The approval checkpoint was approved after human review.
- The plan authorizes the next bounded review and implementation phases only through the scoped prompts and reviews described below.

## 3. Sprint Goals

- Stop serializing every roadmap question through one planning/review/implementation chain.
- Identify the next code-bearing phases.
- Preserve safety boundaries before writes and broader runtime behavior.
- Use subagents for independent review, planning, and bounded implementation lanes.
- Keep the local kernel as the governing wrapper for material roadmap work.
- Make clear when the project returns to implementation-heavy work.

## 4. Non-Goals

This sprint plan does not authorize:

- automatic runtime hook discovery;
- warning/skipped continuation;
- broad executor hook checkpoints;
- workflow-declared hook configuration;
- runtime hook configuration;
- default local check registration;
- automatic local check execution;
- arbitrary shell execution;
- command-output evidence attachment;
- write-capable adapters;
- generic runtime adapter execution;
- side-effect execution;
- high-autonomy behavior;
- recursive agents or agent swarms;
- nested harness runtime execution;
- reasoning lineage implementation;
- CLI behavior;
- schemas;
- hosted or distributed runtime behavior;
- release posture changes.

## 5. Immediate Phase Queue

### Phase A: Commit/Review Current Planning Artifacts

Type: housekeeping plus review.

Current uncommitted artifacts include:

- executor hook disclosure report input propagation review;
- hook disclosure discovery plan;
- roadmap/concept links to hook disclosure discovery planning;
- this sprint plan.

Recommended action:

1. Maintainer reviews this sprint plan.
2. Commit the planning/review artifacts.
3. Approve the dogfood checkpoint only if the plan is accepted.

### Phase B: Hook Disclosure Discovery Plan Review

Type: short review phase.

Why:

- The hook disclosure discovery plan is already precise and narrow.
- It defines allowed sources, rejected sources, merge/deduplication, report placement, workflow semantics, privacy, tests, and non-goals.
- A review should be quick and should not expand scope.

Exit criterion:

- Review verdict accepts the plan or identifies blockers.

If accepted:

- Immediately execute Phase C.

### Phase C: In-Memory Hook Disclosure Discovery Implementation

Type: code-bearing implementation.

Scope:

- add pure extraction helper for `AgentHarnessHookDisclosureId` values from already-validated in-memory hook results;
- add deterministic merge/deduplication of caller-supplied and discovered disclosure IDs;
- wire only the explicit `BeforeReport` report-bearing path;
- keep citations in `ValidationAndQualityChecks`;
- preserve workflow semantics and report-generation error separation;
- add focused tests and implementation report.

Strict non-goals:

- no event/audit discovery;
- no persistence;
- no disclosure store;
- no workflow event append changes;
- no dedicated hook audit sink;
- no warning/skipped continuation;
- no schemas;
- no CLI;
- no artifact writes;
- no side effects or writes.

This is the first immediate exit from planning into code.

### Phase D: Hook Disclosure Discovery Implementation Review

Type: review.

Why:

- Confirms the implementation did not accidentally infer from workflow events, generic audit projections, text, diagnostics, local checks, or adapter telemetry.

Exit criterion:

- If accepted, stop the hook-disclosure micro-chain unless a concrete user-facing need requires section routing or warning/skipped semantics.

## 6. Local Check Lane

### Phase E: Local Check Side-Effect Boundary Blocker-Fix Review

Type: review gate.

Why:

- The local check boundary model has a blocker-fix report.
- The review is a prerequisite before opt-in live DocsCheck smoke or broader local command execution.

Exit criterion:

- Review accepts the blocker fix.

### Phase F: Opt-In Live DocsCheck Smoke Planning Or Implementation

Type: preferably implementation after a tiny plan update if needed.

Recommended implementation boundary:

- explicit npm executable;
- explicit repository root;
- explicit npm cache directory;
- disabled network posture unless separately approved;
- no ambient environment;
- no source writes;
- no default registration;
- no CLI activation;
- no schema fields;
- no arbitrary commands.

This is the second implementation-heavy exit from planning.

## 7. Write-Adapter Prerequisite Lane

Write-capable adapters should not start yet. The roadmap and engineering standard require side effects to be capability-gated, policy-gated, auditable, idempotent, approval-aware, and redaction-safe before provider mutation.

Planning-only gates before serious write-adapter implementation:

1. Generic Side-Effect Boundary ADR/core-model plan.
2. High-Assurance Approval Controls ADR/plan.
3. Evidence/report/approval/side-effect citation plan.
4. Write-Adapter Readiness and first provider write plan.

These should not run serially unless a dependency truly requires it.

Recommended parallel execution:

- side-effect boundary ADR planning and high-assurance approval planning can run in parallel;
- evidence/report citation planning can run in parallel after both drafts identify shared terms;
- first write-adapter readiness planning should wait for the previous outputs, but can be prepared as a skeleton in parallel.

After those gates, the first write-adapter implementation can be scoped.

## 8. What Subagents Can Run

Recommended subagent use:

| Workstream | Subagent type | Parallel? | Write scope |
| --- | --- | --- | --- |
| Hook disclosure discovery implementation | worker | Yes, but only if main agent owns integration review | `crates/workflow-core/src/work_report.rs`, `crates/workflow-core/src/executor.rs`, focused tests |
| Hook discovery implementation tests | worker or explorer | Yes | focused tests only, disjoint from code worker if split |
| Local check boundary blocker-fix review | explorer | Yes | no edits |
| Opt-in DocsCheck smoke plan | explorer or worker after review | Yes | docs first; code later in local-check modules |
| Generic side-effect ADR | explorer/worker docs | Yes | docs/adr and docs/implementation-plans only |
| High-assurance approval controls plan | explorer/worker docs | Yes | docs/adr, docs/runtime, docs/implementation-plans only |
| Evidence/report side-effect citation plan | explorer/worker docs | Yes | docs/implementation-plans and concepts only |
| First write-adapter readiness plan | explorer | Yes, skeleton only | docs/integrations and implementation plan |

Subagent rules:

- Do not give two coding agents overlapping files.
- Do not let subagents implement write-capable adapters.
- Do not let subagents add schemas, CLI behavior, persistence, or side-effecting behavior without a reviewed plan.
- Use explorers for review/planning and workers only for bounded code changes with disjoint ownership.

## 9. Planning Exit Timeline

This is the shortest honest route out of planning:

1. **Now**: review this sprint plan and commit current planning artifacts.
2. **Next phase**: review hook disclosure discovery plan.
3. **Following phase**: implement in-memory hook disclosure discovery. This is code.
4. **Parallel**: review local-check boundary blocker fix.
5. **Following local-check phase**: implement opt-in live DocsCheck smoke. This is code.
6. **Parallel planning**: side-effect boundary ADR, high-assurance approval controls, evidence/report citation plan.
7. **After parallel planning**: first write-adapter readiness plan.
8. **After readiness review**: first write-adapter implementation slice.

Answer to "when do we get out of planning":

- **For hook/report work: after one short plan review, immediately.**
- **For local check execution: after one blocker-fix review, immediately.**
- **For write adapters: after four planning-only gates plus reviews, but those gates should run in parallel instead of as a long serial crawl.**

## 10. Recommended Next Phase

Recommended next phase: **hook disclosure discovery plan review**, with a parallel subagent reviewing the local-check side-effect boundary blocker fix.

That gives the project one quick governance check before the next code phase, while also clearing the local-check lane for implementation-heavy work.

## 11. Validation

- `npm run dogfood:benchmark -- validate` - passed with expected experimental lifecycle warnings.
- `npm run dogfood:benchmark -- start --run-id run/roadmap-next-sprint-plan` - started governed run and paused at approval.
- `npm run dogfood:benchmark -- approve run/roadmap-next-sprint-plan approval/run/roadmap-next-sprint-plan/planning-approved --reason reviewed-next-roadmap-sprint-plan` - granted approval and completed the governed run.
- `npm run dogfood:benchmark -- inspect run/roadmap-next-sprint-plan` - confirmed run status `Completed`.
- `npm run check:docs` - passed.
