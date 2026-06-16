# Governed Multi-Step Workflow Execution Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The first sequential local multi-step executor slice satisfies the accepted P0 boundary: it runs ordered local steps, preserves existing public executor APIs, keeps governance at each step boundary, rejects branch declarations fail-closed, and does not introduce branching, parallelism, nested harness runtime behavior, writes, schemas, CLI changes, examples, automatic report generation, or release posture changes.

The implementation should be hardened with later-step approval, retry, failure, and cancellation coverage before the roadmap broadens to branching, handoffs, nested harnesses, or side-effect modeling.

## 2. Scope Verification

The phase stayed within the approved sequential local executor scope.

Implemented scope:

- Ordered local workflow steps are executed sequentially.
- Existing single-step execution behavior remains compatible.
- Existing `LocalExecutor::execute(...)` remains unchanged as public API.
- Existing `LocalExecutor::execute_with_report(...)` remains explicit and in-memory.
- Per-step `StepScheduled`, policy, invocation, retry, approval, failure, escalation, and terminal events use the existing event model.
- Branch declarations fail closed before run events are appended.
- Report-bearing execution remains compatible with completed multi-step runs.

No accidental scope expansion was found for:

- branching execution;
- parallel or DAG scheduling;
- nested harness execution;
- Composable Harness Contract runtime behavior;
- side-effect modeling;
- write behavior;
- automatic report generation;
- automatic report artifact writing;
- CLI behavior;
- workflow spec schema changes;
- examples;
- reasoning lineage or claim graph behavior;
- hosted or distributed runtime behavior;
- release posture changes.

## 3. Implementation Assessment

The executor now builds an internal ordered `StepExecutionPlan` vector from the existing workflow `steps` array. The public runtime model is unchanged; no new event kind, snapshot field, schema field, CLI command, or artifact behavior was introduced.

The internal cursor is appropriately conservative:

- it starts at the first declared step;
- it schedules the current step before governance checks;
- it evaluates approval and invocation policy at the current step;
- it invokes only registered local handlers;
- it advances only when the current step succeeds and declares `terminal_behavior: continue`;
- it completes the run when a successful step does not request continuation;
- it fails closed if a step requests continuation without a next step;
- it rejects branch declarations with `executor.workflow.multistep.unsupported_branching`.

The implementation is compatible with current validation, which already requires at least one step and prevents the last step from declaring `continue`.

## 4. Runtime Semantics Assessment

Single-step compatibility is preserved. Existing local executor tests still pass, including event count/order for the prior single-step happy path.

Multi-step event behavior is deterministic for the covered cases:

- two-step workflows schedule and succeed both steps in order;
- three-step workflows emit ordered `StepScheduled`, policy, invocation request, and success boundaries for each step;
- duplicate execution with an existing explicit run ID rehydrates the durable completed run and does not repeat handler calls;
- report-bearing execution returns a completed run plus a validated report for a completed multi-step run.

Approval behavior remains correct for existing single-step approval tests. The resume path now selects the approval step by ID and marks it as already scheduled/approved so approval grant does not re-request approval. That is the right shape for later-step approval, although later-step approval still needs direct tests.

Retry and escalation behavior remain correct for existing covered retry paths. Later-step retry and retry-exhaustion coverage should be added before expanding execution topology.

## 5. Governance Boundary Assessment

Per-step governance remains intact:

- `StepScheduled` is emitted before the step's policy/invocation path.
- Policy evaluation happens before skill invocation.
- Approval-gated steps pause before invocation.
- Denied invocation policy fails the run closed through the existing path.
- Retry and escalation remain step-scoped through existing retry and escalation records.
- Missing local handlers fail closed.

No evidence was found that the implementation bypasses policy, approval, audit, observability, idempotency, or output contract validation.

## 6. Idempotency And Replay Assessment

The duplicate explicit run-id path remains safe: existing runs are rehydrated before a new execution plan is prepared.

Per-step invocation idempotency keys still include:

- run ID;
- workflow ID;
- workflow version;
- step ID;
- skill ID;
- skill version.

This preserves the key property needed for multi-step idempotency: different steps in the same run do not share invocation keys.

The implementation does not yet implement partial-run resume after process interruption between steps. That is consistent with the current local executor model and should be handled in a later restart-safety phase if needed.

## 7. Report Compatibility Assessment

The existing explicit report-bearing execution path remains compatible.

The implementation does not:

- generate reports automatically;
- append report events;
- write report artifacts;
- add report CLI rendering;
- create step-by-step report sections;
- change report citation semantics.

The added report-bearing multi-step test verifies that a completed multi-step run can produce a validated in-memory `WorkReport`, preserves run events, and creates no report artifacts.

## 8. Error Handling Assessment

The new error boundary is stable and non-leaking:

- `executor.workflow.multistep.unsupported_branching` rejects branch declarations before run events are appended.
- `executor.workflow.multistep.no_next_step` protects against an impossible continuation cursor state.
- `executor.workflow.multistep.resume_step_missing` protects approval resume if the workflow definition no longer contains the approved step.
- `executor.workflow.multistep.step_index_invalid` protects internal cursor misuse.

The error messages are stable summaries and do not include raw inputs, outputs, provider payloads, command output, parser payloads, secrets, tokens, or credentials.

## 9. Test Quality Assessment

Strong coverage added:

- existing single-step workflow still passes unchanged;
- two-step sequential workflow completes both steps;
- three-step workflow emits ordered step boundaries;
- per-step policy decision appears before invocation request;
- duplicate run ID does not repeat completed multi-step handler calls;
- completed multi-step run works with explicit report-bearing execution;
- unsupported branch declarations fail closed before events;
- full local executor regression suite still passes;
- full workspace tests still pass.

Important remaining gaps:

- later-step approval pause/grant/denial should be directly tested;
- later-step retry success and retry exhaustion should be directly tested;
- policy denial on a later step should verify later steps are not invoked;
- cancellation during a multi-step run should be covered where a non-terminal pause state is available;
- report-generation failure after a multi-step run should be directly covered, although the generic report failure behavior is already tested.

These are non-blocking for the first sequential slice because the implemented happy-path and fail-closed branch behavior are correct, existing approval/retry primitives still pass, and no scope-broadening behavior was introduced.

## 10. Documentation Review

Living docs were updated to reflect the current implementation boundary:

- `README.md` now describes a sequential local workflow path.
- `ROADMAP.md` says the first sequential local executor slice is implemented and review/hardening is next.
- `docs/runtime/local-executor.md` documents sequential ordered local steps and keeps branches/parallelism/nested harness behavior deferred.
- `docs/release/V0_KNOWN_LIMITATIONS.md` no longer claims the local executor is single-step.
- CLI, runtime, concepts, specs, and user-guide docs were updated to avoid stale single-step claims.
- The implementation report exists at `docs/concepts/GOVERNED_MULTI_STEP_WORKFLOW_EXECUTION_REPORT.md`.

Historical review/report artifacts still contain old single-step wording where they describe prior project state. That is acceptable as historical record.

## 11. Blockers

No blockers.

## 12. Non-Blocking Follow-Ups

- Add direct later-step approval tests: step one completes, step two pauses, grant resumes step two without re-running step one, denial fails without invoking step two.
- Add later-step retry and retry-exhaustion tests.
- Add policy denial on a later step.
- Add multi-step cancellation coverage where the run can be paused/non-terminal.
- Add report-generation failure coverage for a completed multi-step run.
- Consider converting the dogfood workflow to a small multi-step governed workflow after the hardening tests land.
- Consider an explicit restart-safety plan for partial multi-step progress if the runtime needs to resume after interruption between steps.

## 13. Recommended Next Phase

Recommended next phase: governed multi-step workflow execution hardening tests.

The implementation should not move to branching, parallelism, nested harness runtime behavior, write-capable adapters, side-effect modeling, or reasoning lineage until later-step approval/retry/failure/cancellation behavior is directly tested.

After hardening tests pass, the next practical dogfood phase should be converting the self-governance workflow from one approval-gated step into a small sequential governed workflow.

## 14. Validation

- `cargo fmt --all --check` - pass.
- `cargo clippy --workspace --all-targets -- -D warnings` - pass.
- `cargo test --workspace` - pass.
- `npm run check:docs` - pass.
