# Governed Multi-Step Workflow Execution ADR Review

## 1. Executive Verdict

ADR accepted; proceed to governed multi-step workflow execution tests and local sequential executor implementation.

ADR 0010 correctly captures governed multi-step workflow execution as the P0 kernel blocker surfaced by dogfooding. The implementation plan is sufficiently bounded for the next phase: sequential local execution only, with no branching, parallelism, nested harness runtime behavior, writes, schemas, CLI behavior, examples, automatic report generation, or reasoning lineage.

## 2. Scope Verification

The ADR and plan stay within architecture/planning scope.

They do not authorize:

- runtime implementation in the review phase;
- arbitrary recursive agent spawning;
- agent swarms;
- agents managing agents;
- automatic agent orchestration;
- hosted or distributed runtime execution;
- production nested harness execution;
- live write integrations;
- side-effect boundary implementation;
- external write behavior;
- workflow spec schema changes;
- CLI behavior;
- example updates;
- Reasoning Lineage / Claim Graph implementation;
- Level 3/4 autonomy enablement;
- replacement of deterministic governance with model self-review;
- parallel step execution;
- conditional branch execution;
- generic adapter execution;
- report artifact auto-writing from executor paths.

## 3. Dogfood Feedback Assessment

The P0 framing is justified.

Kernel dogfooding showed that a single governance check proves the kernel shape but does not support realistic governed work. Real workflows need multiple ordered checks, approvals, validation/check boundaries, failure handling, and final reportability.

This is a kernel execution-depth problem, not an agent-orchestration problem.

## 4. Current Kernel Assessment

The review confirms the current implementation boundary:

- workflow specs already model ordered `steps`;
- validation already checks multi-step shape such as unique step IDs, references, terminal behavior, and branch targets;
- runtime events are already step-scoped for scheduling, policy, approval, skill invocation, retry, escalation, failure, and terminal transitions;
- `LocalExecutor` still calls `single_step(...)`;
- multi-step workflows currently fail closed with `executor.workflow.multistep_unsupported`.

The implementation gap is local executor orchestration and replay semantics, not basic spec modeling.

## 5. ADR Assessment

ADR 0010 is accepted because it:

- identifies the dogfooding blocker clearly;
- frames the capability as governed multi-step workflows, not recursive agents or swarms;
- preserves Workflow OS as a governed workflow kernel;
- keeps Composable Harness Contracts as a later layer;
- requires deterministic state transitions and durable event reconstruction;
- keeps policy, approval, retry, escalation, idempotency, evidence, typed handoff, audit, observability, and report boundaries intact;
- explicitly does not authorize runtime behavior by itself.

## 6. Plan Assessment

The implementation plan is appropriately conservative.

Accepted first boundary:

- sequential ordered steps only;
- local executor only;
- no schema changes;
- preserve existing single-step behavior;
- use existing event-sourced runtime model;
- derive step cursor from event history where possible;
- reject or defer branching and ambiguous terminal behavior;
- keep report generation separate from execution;
- require focused tests before broadening scope.

## 7. Governance Semantics Assessment

The plan correctly requires each step to preserve the existing governance sequence:

1. `StepScheduled`.
2. Policy decision evaluation.
3. Approval request if required.
4. `SkillInvocationRequested` only after policy and approval gates clear.
5. Skill invocation through registered local handler.
6. Skill success/failure event.
7. Retry, escalation, failure, or next-step selection.

This is the right invariant to protect before adding branching, parallelism, or harness runtime behavior.

## 8. Replay And Idempotency Assessment

The plan correctly treats replay and idempotency as first-class blockers.

The next implementation must prove:

- completed steps are not re-run on duplicate execution;
- completed prior steps are not re-run after approval for a later step;
- retry reattempts only the failing step;
- event sequence numbers remain contiguous;
- replay reconstructs the multi-step completed state;
- any ambiguous replay state fails closed with stable non-leaking errors.

## 9. Report And Handoff Assessment

The plan correctly keeps report behavior compatible without expanding report scope.

The first implementation should not add step-specific report sections, automatic report generation, artifact auto-writing, or new citation semantics. It only needs to ensure existing report-bearing execution can bind to a completed multi-step run and preserve current report/audit/missing-citation semantics.

Typed handoffs remain a key future dependency, but runtime handoff generation should not be smuggled into the first multi-step executor slice.

## 10. Relationship To Harness Contracts

The ADR correctly positions governed multi-step execution before Composable Harness Contract runtime behavior.

Composable Harness Contracts describe bounded execution envelopes. Multi-step execution proves that the kernel can sequence governed work, enforce step boundaries, preserve durable state, and produce final reportability before nested harness execution patterns are attempted.

## 11. Test Plan Review

The required future tests are appropriate and high-signal:

- single-step compatibility;
- two-step and three-step sequential completion;
- per-step policy ordering;
- policy denial prevents later execution;
- approval pause/resume on a later step;
- approval denial on a later step;
- retry and retry exhaustion on a later step;
- duplicate run ID does not repeat completed steps;
- event replay reconstructs state;
- cancellation prevents remaining steps;
- report-bearing execution works for multi-step completion;
- unsupported branching fails closed;
- no writes, artifacts, CLI, schemas, examples, side effects, or reasoning lineage.

The implementation phase should add tests before or alongside refactoring to avoid turning this into a broad executor rewrite.

## 12. Documentation Review

Docs now state:

- governed multi-step workflow execution is P0;
- ADR 0010 is accepted;
- the implementation plan is sequential/local/deterministic;
- current local executor remains single-step until implementation;
- Composable Harness Contracts remain later;
- nested harness execution, side effects, writes, schemas, CLI, examples, and reasoning lineage remain unimplemented.

## 13. Blockers

No planning blockers remain.

Implementation blockers to resolve in the next phase:

- define whether the step cursor can be fully derived from current events;
- define how `terminal_behavior: complete` should behave when later steps exist;
- decide whether branch declarations must fail closed in executor scope even if validation accepts references;
- preserve approval resume semantics without replaying earlier completed steps.

## 14. Non-Blocking Follow-Ups

- Consider a later terminal-behavior review after sequential execution is working.
- Consider step-level report summaries after multi-step execution is stable.
- Consider runtime handoff generation only after sequential step execution and replay are proven.

## 15. Validation

Review validation:

- `npm run check:docs` - passed.

The preceding pivot phase also ran the self-governance dogfood workflow successfully:

- validated dogfood project;
- started approval-gated run;
- approved run;
- inspected completed event history.

## 16. Recommended Next Phase

Recommended next phase: governed multi-step workflow execution tests and local sequential executor implementation.

Start with the smallest implementation that makes a two-step local workflow complete deterministically while preserving existing single-step behavior. Then extend to N ordered steps and add approval/retry/report-bearing coverage.
