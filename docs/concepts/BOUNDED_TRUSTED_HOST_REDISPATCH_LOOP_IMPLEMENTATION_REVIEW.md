# Bounded Trusted-Host Redispatch Loop Implementation Review

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups.**

The private local SQLite helper composes the accepted continuation primitives
without creating a parallel source of authority. It continues only from a
fresh Core-derived `ResumeNow`, consumes one current directive, crosses one
atomic dispatch reservation before every executor entry, and preserves the
exact immutable invocation and executor commitments fixed by the opened
window. It returns normally only for `AwaitCondition`, `Blocked`, or
execution-window `Terminal`; inconsistent exhausted-attempt posture fails
closed.

No blocker was found. The helper remains private and unused by a public or
provider path, which is the correct boundary for this first composition
slice.

## 2. Scope Verification

The phase stayed within the approved private implementation boundary. It
added:

- one crate-private SQLite redispatch module;
- one crate-private non-authorizing identity-provider trait;
- one bounded input and outcome model;
- one private supervisor-binding snapshot helper;
- focused composition tests;
- roadmap and phase documentation.

It did not add a scheduler, daemon, model-turn creation, automatic approval,
provider execution or mutation, OpenShell, nested harnesses, public runtime
configuration, CLI, SDK, schemas, filesystem or PostgreSQL parity, hosted
behavior, or release-posture changes.

## 3. Source-Of-Truth Assessment

The implementation preserves the accepted source-of-truth boundaries:

- durable Core state supplies continuation disposition;
- the window record supplies the finite attempt limit;
- projected directive consumption supplies one-use resumed authority;
- atomic dispatch reservation supplies executor-entry authority;
- the immutable operation and executor commitments govern invocation;
- durable run rehydration supplies the cursor used by the resumed supervisor;
- execution-window terminal posture remains distinct from workflow terminal
  posture.

The helper does not treat its loop counter, identity provider, output, or host
return value as workflow authority.

## 4. Fresh Continuation Assessment

After every one-shot supervisor result, the helper first handles a closed
disposition. When the returned posture is `ResumeNow`, it re-reads
`continuation_disposition` from SQLite before requesting another iteration
identity or consuming another directive.

That read is advisory only. A race after the read cannot authorize execution:
the projected consume operation still validates the current window revision,
cursor, active yield generation, wait revisions, actor, authority commitment,
and window binding. Stale or competing callers therefore fail before receiving
a resumed capability.

The helper has no normal host-preemption or budget-exhausted return. It cannot
translate a remaining `ResumeNow` into completion, wait, approval, or success.

## 5. Directive And Reservation Assessment

Every resumed iteration:

1. loads the current active yield and canonical wait revisions;
2. requires exactly one available directive for the bound window and
   generation;
3. consumes that directive through the projected compare-and-set operation;
4. treats security rejection and exact replay as structured errors;
5. rehydrates the current run cursor; and
6. enters the existing one-shot supervisor, which must atomically reserve the
   exact attempt before invoking the executor.

The loop does not reconstruct capabilities from rows or receipts. Durable
receipts remain evidence, not authority. Existing directive and reservation
concurrency tests prove one-winner behavior on independent SQLite
transactions, and the composition reuses those exact operations.

## 6. Finite-Bound And Liveness Assessment

The authoritative window `next_attempt_number` and `maximum_attempts` posture
is checked before requesting the next host identity and again before directive
consumption. Zero, overflow, or exhaustion while Core still derives
`ResumeNow` returns the stable
`trusted_host_redispatch.attempt_limit_inconsistent` error.

The loop counter uses checked arithmetic and records admitted executor entries
only after the prior one-shot call returned successfully. The counter is
report posture, not an authorization input. This preserves the corrected plan:
the durable window is the finite bound and a host-selected budget cannot
create a false stall.

## 7. Immutable Invocation Assessment

The exact `SkillInput` is owned once by the loop input and reused unchanged for
each supervisor call. The same injected executor supplies the executor
commitment on every iteration. The identity provider can supply only bounded
operation, receipt, attempt, and yield-generation identifiers; it cannot
receive or return invocation data, commitments, capabilities, or durable
state.

The one-shot supervisor recomputes the invocation commitment before
reservation and executor entry. The focused substitution test proves that a
changed secret-like input fails with
`trusted_host_supervisor.invocation_binding_mismatch` before executor entry or
an identity-provider request.

## 8. Stop And Workflow-Semantics Assessment

- `AwaitCondition` returns immediately without polling or another directive.
- `Blocked` returns immediately without retry or fabricated approval.
- `Terminal` returns immediately without another redispatch.
- security, storage, replay, ambiguity, binding, projection, and liveness
  failures remain structured errors.

`Terminal` describes the execution window only. The focused success tests
confirm that a successful skill output does not mark the workflow run
completed; the rehydrated run remains `Running`.

## 9. Failure, Ambiguity, And Restart Assessment

The helper delegates reservation, outcome, yield, and ambiguous-attempt
persistence to the accepted one-shot supervisor. A failed or ambiguous
reservation cannot reach the executor. A persistence or reconciliation error
returns from the helper and is not caught as a retryable loop result.

No loop-local capability is serializable or persisted. A process restart must
rehydrate durable state and obtain fresh current authority through a separately
reviewed entry path. The phase does not overclaim scheduler, lease, failover,
or automatic restart behavior.

## 10. Privacy And Redaction Assessment

- input, identity, and outcome Debug implementations redact bound values;
- the outcome does not Debug-format `SkillOutput` contents or references;
- errors use stable codes and static bounded messages;
- secret-like substituted input does not appear in errors or Debug output;
- the new durable path contains only existing identifiers, commitments,
  revisions, cursors, trusted-time facts, and closed result vocabulary; and
- no prompt, transcript, source content, command output, environment value,
  credential, token, authorization header, provider payload, or raw skill
  output is persisted.

## 11. Test Quality Assessment

Direct composition tests cover:

- yield followed by one lawful redispatch and success;
- two consecutive redispatches using two distinct identity sets;
- authoritative attempt exhaustion before another identity or executor call;
- immutable-input substitution before executor entry;
- redaction-safe Debug and errors; and
- separation of execution-window terminal posture from workflow completion.

The complete workspace suite also preserves the accepted primitive proofs for
typed waits, blocked and terminal dispositions, one-winner directive
consumption, exact replay, stale cursor/revision rejection, dispatch
reservation concurrency, ambiguous commit reconciliation, executor outcomes,
event ordering, restart persistence, adapters, reports, and runtime behavior.

The tests are sufficient for a private composition slice. Direct loop-level
race and fault-injection tests would improve diagnosis and regression locality,
but their absence does not weaken the transactional enforcement already tested
at the reused primitive boundaries.

## 12. Documentation Assessment

The implementation plan, report, and roadmap accurately describe the current
boundary. They distinguish execution-window posture from workflow lifecycle,
state that the helper is private and SQLite-only, and do not claim scheduling,
provider execution, public runtime integration, or production readiness.

## 13. Blockers

None.

## 14. Non-Blocking Follow-Ups

- Add a direct two-loop race test around one yielded window so the
  composition-level loser posture is locally documented.
- Add direct loop tests for `AwaitCondition`, `Blocked`, and initial
  `Terminal` proving no identity-provider or additional executor call.
- Add loop-level fault injection for ambiguous directive, reservation,
  outcome, and yield commits while retaining the accepted no-retry posture.
- Define a separately reviewed durable host-entry and restart handoff before
  any operational caller invokes this private loop.
- Keep filesystem, PostgreSQL, multi-host leases, fairness, failover, provider
  execution, OpenShell, and nested harnesses deferred.

## 15. Recommended Next Phase

Plan the smallest **trusted-host operational entry boundary** that can invoke
this accepted private loop from freshly rehydrated durable continuation state.
The plan must define how a process obtains the initial one-use supervisor
capability, how process restart resumes without reconstructing authority, and
how a genuine typed wait or blocked posture is surfaced without creating a
manual conversational restart.

That phase should remain local, injected, private, and provider-free. It must
not add model-turn creation, background scheduling, automatic approval,
OpenShell, provider mutation, nested harnesses, public configuration, CLI,
SDK, schemas, or hosted execution.

## 16. Governed Review Record

- workflow: `dg/review`
- run: `run-1790985668540551000-2`
- approval: `approval/run-1790985668540551000-2/review-scope-approved`
- presentation: `presentation/6fe4b86262ebbc17`
- presentation hash:
  `6fe4b86262ebbc17e64b628641e374e49470a39ddb5503ef99c76207c6862039`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- review verdict: phase accepted with non-blocking follow-ups
- phase status: completed
- event summary: 39 events, including 1 approval request, 1 approval grant,
  8 policy decisions, 6 scheduled steps, 6 successful skill invocations, 0
  retries, and 0 escalations
- validation summary: the exact reviewed implementation commit passed
  `cargo fmt --all --check`, focused loop tests, Core library tests, workspace
  clippy with warnings denied, the complete workspace test suite,
  `npm run check:docs`, and `git diff --check`; the review-only documentation
  passed the docs and diff checks again
- out-of-kernel work: source inspection, security review, documentation,
  validation, and git actions are performed by the delegated maintainer;
  Workflow OS governs scope and approval but does not inspect code, edit files,
  or run shell commands
