# Governed Multi-Step Workflow Execution Report

## 1. Executive Summary

The first governed multi-step workflow execution slice is implemented for the local executor.

The implementation is intentionally narrow: ordered local workflow steps execute sequentially inside the existing event-sourced local runtime. Existing single-step behavior remains compatible, and existing executor APIs are unchanged.

## 2. Scope Completed

- Replaced the single-step executor preparation path with an ordered internal step cursor.
- Added sequential local execution for one or more declared workflow steps.
- Preserved the existing `LocalExecutor::execute(...)` API.
- Preserved the existing `LocalExecutor::execute_with_report(...)` API.
- Emitted `StepScheduled`, policy, invocation, retry, approval, failure, escalation, and completion events through the existing event model.
- Advanced to the next declared step only when the current step succeeds with `terminal_behavior: continue`.
- Completed the run after the final successful step.
- Preserved approval pause/resume behavior without re-running already completed steps.
- Preserved bounded retry and escalation behavior for the current step.
- Preserved explicit run-id idempotency: duplicate execution requests for a completed multi-step run rehydrate the durable run instead of repeating handlers.
- Rejected branch declarations at executor preparation with a stable unsupported error code.
- Added focused local executor tests for two-step execution, three-step event ordering, duplicate run-id replay, and report-bearing multi-step completion.
- Updated living docs and roadmap posture to reflect sequential local multi-step support.

## 3. Scope Explicitly Not Completed

- No branching execution.
- No parallel execution.
- No DAG scheduler.
- No nested harness runtime behavior.
- No Composable Harness Contract runtime execution.
- No side-effect boundary model.
- No write behavior.
- No automatic report generation.
- No automatic report artifact writing.
- No CLI behavior changes.
- No workflow spec schema changes.
- No example updates.
- No reasoning lineage or claim graph implementation.
- No hosted or distributed runtime behavior.
- No release posture change.

## 4. Implementation Summary

The local executor now prepares an ordered vector of per-step execution plans from the existing workflow `steps` array. Each step plan contains the resolved skill, skill version, invocation ID, idempotency key, retry policy, approval sensitivity, adapter identity, and required capabilities.

Execution uses an internal cursor over that vector. For each step, the executor schedules the step, evaluates policy, pauses for approval when required, invokes the registered local handler, records success or failure, and either advances to the next step or emits a terminal run event.

The implementation does not add public runtime state or new workflow events.

## 5. Governance Semantics

Per-step governance is preserved:

- `StepScheduled` is emitted for each executed step.
- Policy is evaluated before skill invocation for each step.
- Approval-gated steps pause before invocation.
- Approval grant resumes the approved step without re-scheduling or re-running prior steps.
- Approval denial fails closed.
- Retry remains bounded and step-scoped.
- Retry exhaustion escalates or fails according to existing step policy.
- Later steps are not invoked after terminal failure or escalation.

## 6. Replay And Idempotency Summary

The duplicate explicit run-id path remains durable and idempotent. If a caller reuses a run ID that already has events, the executor rehydrates and returns the existing run.

Each skill invocation still uses a stable idempotency key derived from run ID, workflow ID, workflow version, step ID, skill ID, and skill version.

## 7. Report Compatibility Summary

`LocalExecutor::execute_with_report(...)` continues to work after multi-step execution. The report-bearing path remains explicit and in-memory. A completed multi-step run can produce a validated `WorkReport` through the existing report result path.

No automatic report generation or artifact writing was added.

## 8. Test Coverage Summary

Added or extended local executor coverage for:

- two-step sequential execution;
- three-step sequential event ordering;
- duplicate run-id replay for a multi-step run;
- report-bearing multi-step completion;
- unsupported branch declarations failing closed without run events;
- preservation of existing single-step, approval, retry, escalation, report, audit, observability, and runtime behavior through the local executor suite.

Remaining useful coverage includes later-step approval, later-step retry, policy denial on later steps, and cancellation during multi-step runs.

## 9. Commands Run And Results

- `cargo fmt --all` - pass.
- `cargo test -p workflow-core --test local_executor` - pass: 68 tests.
- `cargo fmt --all --check` - pass.
- `cargo clippy --workspace --all-targets -- -D warnings` - pass after refactoring the executor preparation helper and step-result enum to satisfy clippy.
- `cargo test --workspace` - pass.
- `npm run check:docs` - pass.

## 10. Remaining Known Limitations

- Branch declarations are not executed.
- Parallel and DAG execution are not implemented.
- The executor does not derive a public step cursor or expose per-step progress beyond existing events and snapshots.
- `terminal_behavior: continue` is the only behavior that advances to a next step.
- Report sections remain generic; no step-by-step report summaries are generated automatically.
- The self-governance dogfood workflow has not yet been converted to a multi-step workflow.

## 11. Recommended Next Phase

Recommended next phase: governed multi-step workflow execution review.

The review should focus on scope control, single-step compatibility, event ordering, approval resume semantics, retry/failure behavior, replay/idempotency, report compatibility, and documentation honesty before any branching, parallelism, nested harness behavior, or side-effect modeling is considered.
