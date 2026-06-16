# Governed Multi-Step Workflow P0 Pivot Report

## 1. Executive Summary

Kernel dogfooding surfaced a P0 blocker: Workflow OS can currently govern a single executable step, but realistic governed work needs multiple ordered governance checkpoints.

This pivot moves governed multi-step workflow execution to the top of the roadmap. The current report/audit/missing-citation semantics phase was closed with a review artifact, and the multi-step direction was captured in a proposed ADR plus a bounded implementation plan.

## 2. Triggering Feedback

User feedback from kernel use:

- the kernel is promising;
- the current limitation is multi-step workflows;
- today only one governance check works in a workflow;
- the framework becomes more valuable at scale.

The feedback maps directly to the current executor boundary: `LocalExecutor` still supports exactly one local step and rejects multi-step workflows with `executor.workflow.multistep_unsupported`.

## 3. Scope Completed

- Completed the report/audit/missing-citation semantics phase by adding a review document.
- Added [ADR 0010: Governed Multi-Step Workflow Execution](../adr/0010-governed-multi-step-workflow-execution.md) with `Status: Proposed`.
- Added [Governed Multi-Step Workflow Execution Plan](../implementation-plans/governed-multi-step-workflow-execution-plan.md).
- Updated `ROADMAP.md` to mark governed multi-step workflow execution as the P0 blocker.
- Updated Governed Work Pattern docs to explain the relationship between multi-step execution, governed work, and future harness contracts.
- Updated runtime and release limitation docs to state that multi-step execution is planned but not implemented.
- Ran the self-governance dogfood workflow for this pivot.

## 4. Scope Explicitly Not Completed

This pivot did not implement:

- multi-step executor behavior;
- branching or DAG execution;
- parallel step execution;
- nested harness runtime behavior;
- recursive agents, agent swarms, or arbitrary multi-agent orchestration;
- write-capable adapters;
- side-effect boundary implementation;
- hosted or distributed runtime;
- workflow schema changes;
- CLI behavior;
- example updates;
- automatic report generation;
- automatic report artifact writing;
- reasoning lineage;
- Level 3/4 autonomy enablement;
- release posture changes.

## 5. ADR Summary

ADR 0010 proposes governed multi-step workflow execution as a P0 kernel capability.

Key decisions:

- execute authored workflow steps under deterministic governance;
- make each step boundary explicit, auditable, and reconstructable;
- preserve policy, approval, retry, escalation, cancellation, idempotency, evidence, typed handoff, and report boundaries;
- start with sequential local execution before branching, parallelism, nested harnesses, writes, or reasoning lineage.

## 6. Implementation Plan Summary

The implementation plan recommends the first slice:

1. Accept/review ADR 0010.
2. Add failing tests for desired sequential multi-step behavior.
3. Refactor the local executor from `single_step(...)` to an ordered step cursor.
4. Implement two-step sequential local execution.
5. Extend to N ordered local steps.
6. Add approval, retry/escalation, idempotency, replay, cancellation, and report-bearing tests.
7. Review before branching, parallelism, harness runtime, or side-effect work.

## 7. Dogfood Governance Run

This pivot was governed by the self-governance dogfood workflow.

Commands and results:

- `cargo build -p workflow-cli --bin workflow-os` - passed.
- `target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance validate` - passed with expected experimental lifecycle warnings.
- `target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance --state-dir /private/tmp/workflow-os-multistep-pivot-state --mock-all-local-skills run dg/d` - produced `WaitingForApproval`.
- `target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance --state-dir /private/tmp/workflow-os-multistep-pivot-state --mock-all-local-skills approve run-1781585621328709000-2 approval/run-1781585621328709000-2/d --actor codex --reason governed-multi-step-p0-pivot` - completed.
- `target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance --state-dir /private/tmp/workflow-os-multistep-pivot-state inspect run-1781585621328709000-2` - confirmed `Completed` with 14 durable events.

## 8. Validation Commands

Validation for the prior semantics phase had already passed:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`

Additional validation after the pivot docs:

- `npm run check:docs` - passed.

## 9. Remaining Known Limitations

- The local executor still supports exactly one executable step.
- Multi-step execution is planned but not implemented.
- Branching and parallelism remain out of scope.
- Typed handoff runtime generation remains unimplemented.
- Automatic report generation remains unimplemented.
- Side-effect boundary modeling and writes remain unsupported.

## 10. Recommended Next Phase

Recommended next phase: ADR 0010 review, followed immediately by governed multi-step workflow execution tests and local sequential executor implementation.

The first implementation should be sequential, local, deterministic, and test-heavy. It should not introduce branching, parallel execution, nested harness execution, writes, schemas, examples, CLI behavior, automatic report generation, or reasoning lineage.
