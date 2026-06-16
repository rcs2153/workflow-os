# Governed Multi-Step Workflow Hardening Review

## 1. Executive Verdict

Phase accepted; proceed to self-governance dogfood multi-step conversion planning.

The hardening phase directly addresses the non-blocking follow-ups from the first governed multi-step execution review. It adds focused later-step approval, retry, policy-denial, cancellation, and report-generation-failure coverage without broadening the runtime beyond sequential local execution.

No blocker was found.

## 2. Scope Verification

The phase stayed within the approved hardening-test scope.

Completed scope:

- later-step approval pause coverage;
- later-step approval grant coverage;
- later-step approval denial coverage;
- cancellation while waiting on a later-step approval;
- later-step retry success coverage;
- later-step retry exhaustion and escalation coverage;
- policy denial on a later external step before invocation;
- report-generation failure after a completed multi-step run;
- roadmap and phase-report documentation.

No accidental scope expansion was found for:

- branching execution;
- parallel or DAG execution;
- nested harness runtime behavior;
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

## 3. Later-Step Approval Assessment

The new approval tests verify the important later-step approval cases:

- a two-step workflow completes the first step, schedules the second step, and pauses for approval before invoking step two;
- approval grant resumes step two without re-scheduling or re-running step one;
- approval denial fails the run and does not invoke step two;
- cancellation from the waiting state cancels the run without invoking step two.

The tests assert both snapshot status and event behavior. They verify the handler call count, scheduled step sequence, absence of unauthorized `SkillInvocationRequested` events for step two, and resumption ordering around `RunResumed`.

This closes the approval coverage gap identified in the prior execution review.

## 4. Later-Step Retry And Escalation Assessment

The new step-aware transient handler is appropriate for test scope. It records invoked step IDs and fails only the configured step for a bounded number of attempts.

The retry success test verifies:

- step one runs once;
- step two fails once and is retried;
- only step two is retried;
- the run completes;
- a `RetryScheduled` record is associated with step two.

The retry exhaustion test verifies:

- step one runs once;
- step two exhausts retry attempts;
- the run escalates;
- the escalation record preserves step two identity.

This closes the later-step retry and retry-exhaustion gap from the prior review.

## 5. Policy Denial Assessment

The policy-denial test uses a second-step symbolic external skill to exercise the conservative runtime policy boundary after a successful first local step.

The test verifies:

- the run fails with `policy.deny.adapter_invoke_v0`;
- only the first local handler is invoked;
- both steps are scheduled in order;
- the denied external step is never requested for skill invocation.

This is the right fail-closed behavior for the current sequential local executor slice.

## 6. Cancellation Assessment

The cancellation hardening coverage uses the existing approval wait state as the available non-terminal multi-step pause point.

The test verifies:

- cancellation from `WaitingForApproval` produces `Canceled`;
- only the first step handler was called;
- the pending later step was not invoked.

This is sufficient for the current local executor model. Broader cancellation during active in-process handler execution remains outside the current synchronous executor scope.

## 7. Report Failure Assessment

The report-generation failure test verifies the explicit report-bearing path after a completed multi-step run.

It confirms:

- workflow execution still completes;
- no report is returned;
- a report-generation error is returned separately;
- the secret-like input does not leak through debug formatting;
- scheduled step order remains unchanged;
- persisted backend events match the returned run events.

This preserves the established report semantics: report-generation failure after a run exists does not retroactively change workflow pass/fail status or mutate event history.

## 8. Runtime Semantics Assessment

The hardening phase did not change the executor implementation. It adds tests around existing behavior.

The covered runtime semantics remain consistent with ADR 0010:

- ordered local steps execute sequentially;
- each step boundary is explicit in events;
- policy remains before invocation;
- approval-gated steps pause before invocation;
- retry and escalation are step-scoped;
- terminal failure, escalation, denial, or cancellation prevents later invocation;
- report-bearing execution remains explicit and in-memory.

The phase does not introduce public cursor state, branch interpretation, parallel scheduling, nested harness execution, write support, or automatic report generation.

## 9. Test Quality Assessment

The new tests are behavior-oriented and inspect state plus event history, not only return values.

Strong coverage added:

- later-step approval pause/grant/denial;
- later-step cancellation while waiting;
- later-step retry success;
- later-step retry exhaustion with escalation context;
- later-step policy denial before invocation;
- multi-step report-generation failure preservation.

The local executor suite now includes 76 tests and continues to cover single-step compatibility, duplicate run-id idempotency, event persistence, audit, observability, report paths, local check handlers, and unsupported branch fail-closed behavior.

Remaining non-blocking test gaps:

- partial-run restart after interruption between completed steps is not separately modeled or tested;
- cancellation during an actively running synchronous handler is not represented by the current executor shape;
- future typed handoff behavior between multi-step outputs remains deferred.

These are not blockers for the hardening phase because they are outside the accepted sequential-local execution slice.

## 10. Documentation Review

Documentation is accurate for the phase boundary:

- `ROADMAP.md` states that the first sequential local multi-step slice has been reviewed and hardened, and that hardening review is next.
- `docs/runtime/local-executor.md` documents ordered local step execution, later-step approval/retry/cancellation semantics, and unsupported behavior.
- `docs/concepts/GOVERNED_MULTI_STEP_WORKFLOW_HARDENING_REPORT.md` clearly states completed scope, non-scope, validation, remaining limitations, and next phase.

The docs do not claim support for branching, parallel execution, nested harness runtime behavior, writes, automatic report generation, CLI changes, schemas, examples, or hosted/distributed runtime behavior.

## 11. Blockers

No blockers.

## 12. Non-Blocking Follow-Ups

- Plan conversion of the self-governance dogfood workflow from a single approval-gated step to a small sequential governed multi-step workflow.
- Consider a later partial-run restart-safety plan for interruption between completed steps.
- Keep branching, parallelism, nested harness execution, writes, and side-effect modeling deferred until additional scoped planning and review.
- Consider future typed handoff integration once the dogfood multi-step path proves the sequential kernel shape.

## 13. Recommended Next Phase

Recommended next phase: self-governance dogfood multi-step conversion planning.

The multi-step kernel is now implemented, reviewed, and hardened enough to dogfood against the project’s own governed build process. The next phase should plan a small sequential dogfood workflow, likely separating planning/review/check/report-oriented steps without adding real build-command execution, automatic Codex control, writes, schemas, CLI behavior, examples, branching, parallelism, nested harness execution, or reasoning lineage.

## 14. Validation

- `cargo fmt --all --check` - pass.
- `cargo clippy --workspace --all-targets -- -D warnings` - pass.
- `cargo test --workspace` - pass.
- `npm run check:docs` - pass.
