# Authorized Execution Continuity Semantic V2 Owner-Target Blocker Fix Review

## 1. Executive Verdict

**Blocker fixed. Proceed to the SQLite semantic V2 continuity backend.**

The focused correction closes the remaining owner-to-target replay gap in the
test-only semantic oracle. Exact-revision replay now proves the owning
window's active-yield relationship for yield and wait operations and proves
the absence of stale yield ownership for consume, outcome, and recovery
operations. Lawful later window revisions remain replayable.

This acceptance is limited to shared reference semantics. It does not claim
durable continuity, executor redispatch, host scheduling, automatic approval,
provider mutation, CLI behavior, hosted execution, or nested harness runtime.

## 2. Scope Verification

The fix stayed within the approved blocker scope. It changed the private
test-only reference validator, added focused corruption and lawful-successor
regressions, updated the roadmap, and added an implementation report.

It did not add SQLite schema or transactions, runtime events, executor or
supervisor integration, delegated approval automation, provider writes,
workflow-schema fields, SDK behavior, CLI behavior, hosted execution, nested
harnesses, or release-posture changes.

## 3. Owner-To-Target Integrity Assessment

The exact-revision relationships are now explicit for every successful
operation family:

- `YieldRegistered` requires a yielded owning window whose `active_yield`
  names the committed generation;
- `WaitTransitioned` requires the same yielded owner and active generation;
- `DirectiveConsumed` requires the committed executing window with no active
  yield;
- `RecordAttemptOutcome` requires the committed closed window with no active
  yield; and
- `RecoverAmbiguousAttempt` requires the committed recovery-required window
  with no active yield.

These checks complement the existing operation kind, map-key, embedded
identity, cursor, subject, authority commitment, wait membership, directive
ownership, attempt ownership, and consume-operation linkage checks. A
detached or stale owner now fails with the stable
`authorized_execution_continuity_state.state.corrupt` code.

## 4. Historical Replay And Lawful Successors

The correction deliberately distinguishes an exact committed revision from a
lawful later revision. Exact replay validates the relationship that must hold
at the committed revision. Yield, wait, and directive results may still be
replayed after a later valid operation advances the owning window.

Focused regressions prove:

- a committed wait remains replayable after the same generation is satisfied
  and a directive is lawfully consumed; and
- a committed directive consumption remains replayable after the resulting
  attempt records a lawful terminal outcome.

This is the correct monotonic replay posture. Requiring mutable current state
to equal every historical result would create false corruption after lawful
progress; accepting a broken relationship at the exact revision would permit
disconnected authoritative state.

## 5. Operation-Family Corruption Matrix

The new matrix exercises all five successful operation families by first
committing a valid operation and then corrupting only the owning window's
active-yield relationship:

- registered yield ownership detached;
- transitioned wait ownership detached;
- consumed directive given a stale active yield;
- recorded outcome given a stale active yield; and
- recovered ambiguous attempt given a stale active yield.

Every exact replay fails closed. Together with the earlier semantic V2 suites,
this closes the specific defect identified by the prior review and provides a
clear conformance case for the durable backend.

## 6. Compatibility, Privacy, And Error Assessment

The correction changes no public contract or serialized wire shape. V1
ordering and unknown-field compatibility remain unchanged, and the additive
V2 declaration remains capability vocabulary rather than a production-backend
claim.

No payload-bearing field, source content, command output, provider response,
credential, token, or reusable authority was added. Corruption errors remain
bounded and do not echo window, yield, attempt, operation, or identifier
values.

## 7. Test Quality Assessment

The focused tests are behavioral rather than construction-only. They prove
that a valid commit succeeds, a precise authoritative-ownership mutation is
detected on replay, and legitimate successor revisions remain accepted. The
complete reference suite and public contract suite continue to pass.

The next test-architecture requirement is not another semantic-model test
family. It is a backend-parametric conformance harness that runs these exact
semantic V2 scenarios against both the reference oracle and SQLite before the
SQLite backend advertises support.

## 8. Remaining Operational Limitations

- No production backend advertises semantic V2 continuity support.
- No durable continuity state survives process restart.
- No runtime executor opens windows, consumes directives, or records yields.
- No host supervisor consumes `ResumeNow` and redispatches lawful work.
- No final-response guard prevents a host from ending an executor turn while
  a run remains non-terminal.
- No automatic approval or reusable delegated authority is introduced.
- The model does not provide an external rollback-resistant epoch anchor.

These are accepted next-phase boundaries, not reasons to weaken the false-stall
invariant. The kernel remains the only authority for classifying a run as
resumable, waiting, blocked, or terminal; a host turn ending is not workflow
completion.

## 9. Governed Review Record

- workflow: `dg/review`;
- run: `run-1786844430765850000-2`;
- approval: `approval/run-1786844430765850000-2/review-scope-approved`;
- presentation: `presentation/9c875aca675aeef0`;
- presentation hash:
  `9c875aca675aeef01fda19002afd13ae816d7504abe90292cb1a5ebd02bf5024`;
- approval outcome: granted under standing delegated-maintainer authority
  after the complete proof-enforced handoff was evaluated;
- governed run status: completed; and
- out-of-kernel work: source inspection, review, documentation, validation,
  git operations, and PR operations were performed by the external executor.
  The kernel did not edit files, run checks, schedule or redispatch an agent,
  persist semantic V2 continuity state, or approve a user-facing gate
  automatically.

## 10. Validation Evidence

The implementation phase passed:

- the 25 focused reference continuity tests;
- the 6 public continuity contract tests;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace` with an isolated target directory;
- `npm run check`;
- `npm run check:integrations`;
- `npm run check:docs`; and
- `git diff --check`.

The review also inspected the exact implementation diff, the five-family
corruption matrix, and the lawful-successor regressions. Repository CI must
remain green before merge.

## 11. Blockers

None for beginning the bounded SQLite semantic V2 backend implementation.

The SQLite backend must not advertise V2 until it passes the accepted
backend-parametric conformance suite, including owner-to-target corruption,
trusted-time, restart, fault, replay, wait, authority, and attempt semantics.

## 12. Non-Blocking Follow-Ups

- Keep V2 support additive until a deliberate public contract migration is
  justified.
- Preserve fixed non-leaking corruption errors in the durable backend.
- Keep arbitrary restore support deferred until an external epoch anchor can
  prove rollback resistance.
- Do not confuse exactly-once continuity-state mutation with exactly-once
  external execution.

## 13. Recommended Next Phase

Implement the bounded SQLite semantic V2 continuity backend and reusable
backend-parametric conformance harness from the accepted plan. Runtime
event/state projection and one injected-supervisor redispatch vertical slice
follow only after the durable backend is accepted.

Provider mutation broadening, nested harness runtime, automatic approval,
hosted execution, CLI/schema exposure, and additional capability families
remain deferred.
