# Trusted Host TimeWindow Wait Binding Maintainer And Security Review

## 1. Executive Verdict

Needs blocker fixes.

The implementation establishes the correct exact dependency shape, preserves
legacy rows without inventing authority, and keeps the verifier private. Two
runtime-correctness gaps must be fixed before caller integration: the trusted
time fact checked by the verifier is not the same observation used by the
atomic transition, and the private wrapper prevents exact replay after an
ambiguous successful commit.

## 2. Scope Verification

The phase stayed within its approved prerequisite scope. It added exact
`TimeWindow` dependency binding, SQLite v6 persistence and migration, a private
verifier, tests, and honest documentation.

It did not add a caller, scheduler, poller, executor recirculation, another
wake-source family, provider or sandbox work, public API or configuration,
hosted behavior, or release-posture changes.

## 3. Binding And Commitment Assessment

The durable binding is appropriately narrow and covers the exact deadline,
trusted-time source, provenance commitment, epoch identifier, and a
domain-separated commitment over those fields. Registration recomputes the
commitment and rejects missing, forged, source-mismatched, stale, or
out-of-window bindings. Register-yield and transition-wait request commitments
cover the new dependency fields.

The binding is private and Debug output redacts its values. No raw dependency
payload is introduced.

## 4. Persistence And Migration Assessment

SQLite schema v6 is additive and uses an explicit v5-to-v6 upgrade. The v5
checksum remains preserved rather than being redefined. Existing rows receive
null dependency columns and remain explicitly unbound; no deadline, source,
provenance, epoch, or authority is inferred during migration.

Current-schema validation accepts only an all-null legacy shape or a complete
`TimeWindow` relational shape. Snapshot loading cross-checks the canonical JSON
record against that relational projection. Legacy unbound deadline waits are
representable for compatibility but rejected by the private verifier.

## 5. Trusted-Time Freshness Blocker

The private verifier currently performs two trusted-time observations:

1. `transition_time_window_wait` obtains an observation and checks it against
   the exact deadline, source, provenance, epoch, watermark, and expiry; then
2. `transition_wait` opens the atomic transaction and obtains another
   observation for the committed operation.

The transition request carries a private wake capability committed to the
first observation, but the transaction does not prove that the second
observation is the same fact or independently recheck that the second
`observed_at` is at or after the bound deadline. A second reading may move
backward below the deadline while remaining at or above the prior window
watermark. In that case the generic trusted-time rejection logic need not
classify a regression, and the wait can commit as satisfied earlier than its
exact dependency permits.

This violates the plan's same-call verifier invariant. The blocker fix must
make deadline assessment and the authoritative transition share one atomic
trusted-time observation, or revalidate the exact bound deadline inside the
transaction against the observation that is actually committed. A
caller-supplied time assertion is not an acceptable fix.

## 6. Replay And Ambiguous-Commit Blocker

The underlying continuity store supports exact operation replay before
reapplying a mutation. The private wrapper, however, loads current state and
requires the wait to remain `Unsatisfied` before it calls `transition_wait`.

If the first transition commits but its result is lost, retrying the exact same
`TimeWindowTransitionRequest` finds the wait already satisfied and returns
`wait.revision_stale`. It never reaches the existing operation-record replay
path. The caller therefore cannot distinguish an accepted prior commit from a
genuinely stale request through the intended exact replay contract.

The blocker fix must preserve exact replay for the same operation,
commitment, receipt, and expected revisions while continuing to reject
conflicting replays and stale new operations. It must include ambiguous-commit
fault coverage through the private verifier, not only through the generic
store method.

## 7. Concurrency Assessment

The generic transition remains one-winner because it uses an immediate SQLite
transaction, expected revisions, current window and generation bindings, and
the atomic operation record. Competing distinct operations should leave one
winner and one stale or already-transitioned loser.

The implementation does not yet include a direct private-verifier concurrency
test. That is non-blocking only if the blocker fix adds coverage proving:

- two competing verifier calls cannot both record different transitions;
- an exact retry returns the committed result; and
- a conflicting replay fails closed.

## 8. Privacy And Error Assessment

The new model stores bounded timestamps and commitments rather than dependency
payloads. Debug output for the binding is redacted. Stable errors identify
invalid binding, unbound legacy state, unsatisfied time, stale revision, and
wake mismatch without including raw deadlines, provenance, epoch, paths,
tokens, commands, or provider payloads.

No privacy blocker was found.

## 9. Test Quality Assessment

Existing focused tests correctly cover binding persistence, early rejection,
out-of-window deadlines, forged dependency commitments, legacy unbound rows,
and the explicit schema-upgrade chain.

Missing blocker-level coverage:

- the committed transaction observation moving below the deadline after an
  earlier verifier observation passed;
- exact replay through the private verifier after an ambiguous commit;
- conflicting replay through the private verifier; and
- two competing private verifier calls.

## 10. Documentation Assessment

The implementation report accurately states that no caller or scheduler is
implemented and that the verifier is local SQLite only. Its recommendation to
review before caller integration was correct. The roadmap must continue to
block caller integration until both findings above are fixed and re-reviewed.

## 11. Blockers

1. Use the atomic transition's trusted-time observation to enforce the exact
   bound deadline; do not authorize satisfaction from an earlier observation
   that is detached from the committed transaction.
2. Preserve exact replay and ambiguous-commit recovery through the private
   `TimeWindow` verifier before performing current-state eligibility checks
   that would reject the already-committed result.

## 12. Non-Blocking Follow-Ups

- Recompute and validate dependency commitments on authoritative snapshot load
  as additional corruption defense, while recognizing that local database
  tamper resistance is not provided by this preview backend.
- Add direct private-verifier competing-caller coverage even after the generic
  store's one-winner behavior is reused.
- Keep other wake-source families and caller integration deferred.

## 13. Recommended Next Phase

Implement a focused blocker fix for atomic trusted-time deadline enforcement
and exact verifier replay. Then perform a blocker-fix review. Do not begin the
private trusted-host caller until that review accepts both corrections.

## 14. Validation

- Focused private TimeWindow verifier tests: passed, 4 tests.
- Explicit SQLite schema-upgrade-through-v6 test: passed, 1 test.
- `npm run check:docs`: passed.
- `git diff --check`: passed.
- PR 502 CI before merge: all 7 required GitHub checks passed.

The existing tests confirm the implemented happy and rejection paths. They do
not cover the two blocker scenarios identified by this review; those tests are
required in the blocker fix.

## 15. Governed Review Record

- workflow: `dg/review`
- run: `run-1791033347935325000-2`
- approval: `approval/run-1791033347935325000-2/review-scope-approved`
- presentation: `presentation/a3bfa362eeaf5c69`
- presentation hash:
  `a3bfa362eeaf5c69eb7cada84cc04f3d99e235f8b81e61a7f5d3e91f2222bb8e`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- review status: needs blocker fixes
- phase status: completed
- event summary: 39 events, including 1 approval request, 1 approval grant,
  8 policy decisions, 6 scheduled steps, 6 successful skill invocations,
  0 retries, and 0 escalations; approval-presentation proof enforced
- validation summary: 4 focused verifier tests, the schema-v6 upgrade test,
  documentation checks, diff hygiene, and all 7 required pre-merge CI checks
  passed; the blocker scenarios remain untested because their behavior is not
  yet correct
- out-of-kernel work: code reading, security analysis, focused validation, and
  review documentation were performed by the delegated trusted host; Workflow
  OS governed scope and approval but did not perform those actions

## 16. Fix-Forward Note

The two blockers identified above are addressed in [Trusted Host TimeWindow
Wait Binding Blocker Fix
Report](TRUSTED_HOST_TIME_WINDOW_WAIT_BINDING_BLOCKER_FIX_REPORT.md). The
original findings and verdict remain part of the durable review record. Caller
integration stays blocked until a focused blocker-fix review independently
accepts the transaction-bound time check and exact-replay behavior.
