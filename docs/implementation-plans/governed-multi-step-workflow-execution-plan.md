# Governed Multi-Step Workflow Execution Plan

Status: First sequential local executor slice implemented and reviewed. Governed multi-step workflow execution is the P0 blocker surfaced by kernel dogfooding. The architecture direction is accepted in [ADR 0010: Governed Multi-Step Workflow Execution](../adr/0010-governed-multi-step-workflow-execution.md), and the implementation review is [Governed Multi-Step Workflow Execution Review](../concepts/GOVERNED_MULTI_STEP_WORKFLOW_EXECUTION_REVIEW.md). This plan does not authorize branching, parallelism, nested harness runtime behavior, writes, schemas, examples, CLI changes, or automatic report generation.

## 1. Executive Summary

Workflow OS specs and validation already model ordered workflow steps. The first local executor implementation supported exactly one executable step; dogfooding showed that one governance check is not enough. The first sequential local executor slice now runs ordered local steps while keeping the implementation bounded and deterministic.

This plan defines the first bounded implementation path for governed multi-step workflows: sequential local execution of ordered steps, with deterministic event history, per-step policy checks, approval behavior, retry/escalation handling, idempotency, and final report compatibility.

This plan does not authorize parallel execution, branching execution, nested harness execution, write behavior, hosted/distributed workers, CLI changes, schemas, examples, reasoning lineage, side-effect modeling, automatic report generation, or agent orchestration.

## 2. Goals

- Execute more than one ordered local step in a single workflow run.
- Preserve deterministic local kernel behavior.
- Preserve existing single-step behavior.
- Apply governance per step before skill invocation.
- Preserve approval pauses and resumes.
- Preserve retry and escalation semantics.
- Preserve durable event history and replay correctness.
- Preserve idempotency for each skill invocation.
- Preserve existing report-generation behavior and allow reports to summarize multi-step runs later.
- Keep implementation small enough for focused review.

## 3. Non-Goals

This plan does not authorize:

- arbitrary branching execution;
- parallel step execution;
- DAG scheduling;
- nested harness execution;
- Composable Harness Contract runtime execution;
- recursive agents, agent swarms, or multi-agent orchestration;
- write-capable adapters;
- side-effect boundary implementation;
- hosted or distributed runtime behavior;
- worker leasing or heartbeat behavior;
- workflow spec schema changes;
- CLI rendering or new CLI commands;
- example updates;
- automatic report generation for every run;
- automatic artifact writing;
- reasoning lineage or claim graph implementation;
- Level 3/4 autonomy enablement;
- release posture changes.

## 4. Current State

Current relevant behavior:

- Workflow specs define ordered `steps`.
- Validation requires at least one step, unique step IDs, valid skill references, explicit terminal behavior, bounded retries, and approval policy for sensitive steps.
- Runtime events already include step-scoped scheduling, policy, approval, invocation, retry, escalation, failure, and terminal events.
- `WorkflowRunSnapshot` can retain step invocation state and approval state.
- The local executor now builds an ordered step cursor over local workflow steps and executes them sequentially.
- Conditional branches remain unsupported by execution and fail closed at executor preparation.

The implementation gap is executor orchestration, not basic schema representation.

## 5. First Implementation Boundary

The first implementation should support only sequential ordered steps in the existing `steps` array.

Recommended v1 behavior:

- Execute steps in declared order.
- Treat `terminal_behavior: continue` as continue to the next step.
- Treat terminal behaviors such as fail/escalate/complete according to existing model semantics.
- Complete the run after the last successfully executed step.
- Pause the run when a step requires approval.
- Resume from the approved step without replaying completed skill invocations.
- Retry only the failing step according to existing retry policy.
- Escalate or fail according to existing retry/escalation policy.
- Reject or defer branch-target execution until a separate branch/DAG plan exists.

## 6. Step Cursor And Replay Model

The implementation should introduce the smallest internal execution cursor needed to resume deterministically.

Candidate approach:

- derive completed step IDs from `SkillInvocationSucceeded` events;
- derive failed/retry/escalation state from existing step-scoped events;
- derive waiting approval step from snapshot approval state;
- choose the next executable step by declared order and existing event history;
- avoid adding new public runtime state unless replay cannot remain unambiguous.

If a new internal cursor event or snapshot field becomes necessary, it must be separately justified and tested for replay compatibility.

## 7. Per-Step Governance

Each step must preserve the existing governance sequence:

1. `StepScheduled`.
2. Policy decision evaluation.
3. Approval request if required.
4. `SkillInvocationRequested` only after policy and approval gates clear.
5. Skill invocation through registered local handler.
6. Skill success/failure event.
7. Retry, escalation, failure, or next-step selection.

Policy denial for any step must fail closed without invoking that step or later steps.

## 8. Approval Semantics

Approval behavior must remain step-scoped.

For a workflow paused on step N:

- prior completed steps must not be re-invoked after approval;
- approval grant resumes at step N;
- approval denial fails the run;
- duplicate approval decisions remain rejected;
- approval after terminal state remains rejected;
- approval audit records preserve step identity.

No new approval evidence attachment is authorized by this plan.

## 9. Retry And Escalation Semantics

Retry behavior must remain step-scoped.

For a failing step:

- retry should reattempt only that step;
- retry events must preserve step ID, skill ID, attempt number, retry budget, idempotency context, and failure summary;
- exhausted retry policy should escalate or fail exactly as currently modeled;
- later steps must not execute after terminal failure or escalation.

## 10. Idempotency Semantics

Each skill invocation must retain a stable idempotency key.

Multi-step execution must ensure:

- repeated executor calls with the same run ID do not repeat completed steps;
- a completed step is not re-executed after approval for a later step;
- retry attempts remain explicit and step-scoped;
- event sequence numbers remain contiguous;
- duplicate event IDs and duplicate sequence numbers remain rejected.

## 11. Terminal Behavior Semantics

The first implementation must define how existing `terminal_behavior` values affect sequential execution.

Recommended conservative policy:

- `continue`: advance to the next declared step.
- `complete`: complete the run after the current step succeeds, even if later steps are declared.
- `fail`: fail the run if the step cannot complete successfully.
- `escalate`: escalate according to current escalation semantics when configured.

If the existing model has additional or ambiguous terminal behavior variants, the implementation should preserve current validation rules and defer broadening until a focused terminal-behavior review.

## 12. Event And Audit Requirements

Multi-step execution must remain event-sourced and auditable.

Required behavior:

- every executed step emits step-scoped runtime events;
- policy decisions remain durable and auditable per step;
- skill invocation audit records include step ID and skill identity;
- observability records remain derived from workflow events;
- no post-terminal event append is introduced;
- report generation remains separate from execution.

## 13. Report Compatibility

Existing `execute_with_report(...)` should continue to work after multi-step execution.

The first implementation does not need to create step-by-step report sections. It should ensure:

- generated reports bind to the final run identity and terminal status;
- event/audit citations remain stable where supplied;
- optional absent refs remain section text;
- no automatic report artifact writing is introduced.

## 14. Error Handling

Errors must be stable and non-leaking.

Implementation should use stable codes such as:

- `executor.workflow.multistep.unsupported_branching`
- `executor.workflow.multistep.no_next_step`
- `executor.workflow.multistep.replay_ambiguous`
- `executor.workflow.multistep.terminal_behavior_unsupported`

Error messages must not leak raw skill inputs, outputs, paths, provider payloads, command output, parser payloads, secrets, tokens, or credentials.

## 15. Test Plan

Future implementation tests must cover:

- existing single-step workflows still pass unchanged;
- two-step sequential workflow completes both steps;
- three-step sequential workflow emits ordered events for each step;
- per-step policy decisions occur before each skill invocation;
- policy denial on step two prevents step two invocation and prevents later steps;
- approval-gated step two pauses after step one completes;
- approval grant resumes at step two without re-running step one;
- approval denial at step two fails the run without invoking step two;
- retry on step two retries only step two;
- retry exhaustion on step two escalates or fails without invoking later steps;
- duplicate execution with the same run ID does not repeat completed steps;
- event replay reconstructs multi-step completed state;
- cancellation prevents remaining steps;
- report-bearing execution returns a report for completed multi-step runs;
- report-generation failure does not alter multi-step run status;
- unsupported branching or ambiguous terminal behavior fails closed;
- no writes, artifacts, CLI output, schemas, examples, or side effects are introduced;
- `cargo test --workspace` passes.

## 16. Proposed Implementation Sequence

Recommended small phases:

1. Accept the governed multi-step workflow execution ADR.
2. Add failing tests documenting current `multistep_unsupported` behavior and desired sequential behavior.
3. Refactor executor planning from `single_step(...)` to an ordered step cursor without changing public APIs.
4. Implement sequential execution for two local steps with `continue`.
5. Extend to N ordered local steps.
6. Add approval resume tests for a later step.
7. Add retry/escalation tests for a later step.
8. Add report-bearing execution tests for completed multi-step runs.
9. Review before any branching, parallelism, harness runtime, or side-effect work.

## 17. Documentation Updates Required

Future implementation should update:

- `docs/runtime/execution-semantics.md`
- `docs/runtime/state-machine.md`
- `docs/runtime/event-model.md`
- `docs/specs/workflows.md`
- `docs/specs/validation.md`
- `docs/concepts/governed-work-pattern.md`
- `ROADMAP.md`

Docs must clearly state what is implemented and what remains deferred.

## 18. Open Questions

- Should `terminal_behavior: complete` allow later declared steps to remain unreachable, or should validation warn?
- Should branch declarations remain validation-only until a branch execution ADR exists?
- Should the executor expose current step cursor state explicitly, or derive it entirely from events?
- Should approval resume always continue the same executor method, or should later resume APIs gain report-bearing variants first?
- Should multi-step report summaries remain generic initially, or include bounded step counts?
- What is the smallest dogfood workflow that proves multi-step governance value?

## 19. Final Recommendation

Recommended next phase: governed multi-step workflow execution implementation review.

Do not implement branching, parallel execution, nested harness execution, writes, side effects, schemas, examples, CLI behavior, automatic report generation, or reasoning lineage until the first multi-step execution slice is reviewed and hardened.
