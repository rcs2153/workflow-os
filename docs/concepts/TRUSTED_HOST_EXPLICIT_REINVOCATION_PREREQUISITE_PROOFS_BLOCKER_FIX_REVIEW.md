# Trusted-Host Explicit Reinvocation Prerequisite Proofs Blocker Fix Review

## 1. Executive Verdict

**Blocker fixed; proceed to the private trusted-host scheduling-boundary
implementation.**

The corrected full-composition concurrency proof now covers every currently
lawful losing-caller class exposed by the accepted operational-entry path. It
continues to require exactly one underlying executor call and exactly one
aggregate executor entry. Production runtime behavior is unchanged.

Together with the accepted transition-to-entry crash proof, the two explicit
preconditions in the private scheduling plan are now satisfied.

## 2. Scope Verification

The fix stayed within the approved blocker boundary:

- one focused test assertion was changed;
- the prerequisite proof report was corrected;
- one blocker-fix report and roadmap update were added;
- no production function, type, visibility, state transition, or error changed;
- no scheduler, readiness assessment, ticket, waiter, timer, polling, or host
  loop was added; and
- no provider, OpenShell, public API, CLI, SDK, schema, hosted, write, or
  release behavior changed.

## 3. One-Entry Invariant Assessment

The corrected test independently requires:

- the shared executor call count equals one;
- successful outcomes report exactly one aggregate executor entry; and
- exactly one successful outcome reports one executor entry.

These assertions prevent duplicate executor admission even when both callers
return successful bounded outcomes. They do not infer executor use from error
shape or thread order.

## 4. Losing-Caller Classification Assessment

Every caller result must now match one closed class.

Successful outcomes must:

- report `Transitioned` or `ExactReplay` wake posture;
- stop as `Terminal`; and
- report at most one executor entry.

Because the aggregate and one-entry-count assertions are separate, any second
successful caller must report zero entries. That precisely covers a caller
that observes terminal durable posture after the winner completes.

Error outcomes must use one of the two accepted fail-closed race codes:

- `trusted_host_redispatch.directive_replayed`; or
- `trusted_host_redispatch.attempt_limit_inconsistent`.

An unclassified error, non-terminal success, unsupported wake status, or
duplicate entry fails the proof.

## 5. Durable State Assessment

The test still requires authoritative continuation posture to be `Terminal`
after both callers join. The loser is therefore explainable through either a
bounded fail-closed error or current terminal state; it is not silently
dropped and cannot retain a resumable or ambiguous durable posture.

## 6. Restart And Authority Assessment

The blocker fix does not alter the accepted crash-recovery proof. A failure
after committed transition and before entry still recovers after SQLite
reopen through exact replay. Recovery obtains current one-use authority from
durable state and admits one executor entry. A later terminal replay admits
none.

No capability, directive, or authority is cached, serialized, or reconstructed
by either proof.

## 7. Security And Privacy Assessment

- The terminal zero-entry outcome grants no authority.
- Error classification remains a static bounded allowlist.
- Debug, serialization, storage, and runtime outputs are unchanged.
- No payload, command, path, credential, provider data, evidence body, or
  secret-like value was added.
- The test-only crash seam remains private, payload-free, and `cfg(test)`.

## 8. Test Quality Assessment

The proof no longer overfits a particular scheduler interleaving. Five
consecutive focused race runs passed locally, all reinvocation tests passed,
the full workspace passed, and all seven required GitHub checks for PR #525
passed.

The test does not force the terminal zero-entry interleaving deterministically,
but that is not a blocker: its assertions now accept and validate that lawful
outcome whenever the scheduler produces it while rejecting every unbounded
class. A future deterministic interleaving hook would add complexity without
strengthening the production invariant.

## 9. Documentation Assessment

The prerequisite proof report now accurately states that the loser may be a
bounded fail-closed error or a successful terminal zero-entry observation.
The blocker-fix report clearly states that no production behavior changed and
keeps all scheduling surfaces deferred pending this review.

## 10. Blockers

None.

The prerequisite proof gate in the private scheduling plan is satisfied.

## 11. Non-Blocking Follow-Ups

- Keep aggregate entry count and per-success classification separate in future
  duplicate-callback tests.
- Preserve the terminal zero-entry path as a normal bounded observation, not
  an error that needs retry.
- Keep readiness results, scheduling tickets, and host callbacks
  non-authoritative in the next implementation.

## 12. Recommended Next Phase

Implement the first private trusted-host scheduling-boundary slice defined by
the accepted plan:

1. one coherent Core-owned scheduling observation;
2. one inert absolute-UTC scheduling ticket;
3. one non-mutating source-specific readiness assessment; and
4. focused early, stale, binding, zero-write, and privacy tests.

Do not add the injected deadline waiter or schedule-once host helper until the
observation, ticket, and readiness slice receives focused review. Do not add
repeated scheduling, another wake family, provider execution, OpenShell,
public runtime configuration, CLI, SDK, schema, hosted behavior, or writes.

## 13. Governed Review Record

- workflow: `dg/review`
- run: `run-1791352027302271000-2`
- approval:
  `approval/run-1791352027302271000-2/review-scope-approved`
- presentation: `presentation/b170ab23060e35f6`
- presentation hash:
  `b170ab23060e35f6824bab0a053426d28642661522116b5a087f8ea34865c492`
- approval outcome: granted by delegated maintainer through proof enforcement
- reviewed fix commit: `3c65b1d`
- approved boundary: focused blocker-fix review only
- phase status: `Completed`
- event summary: 39 events; one approval; zero retries; zero escalations;
  approval presentation proof enforced with one persisted presentation record
- validation summary: focused concurrency and reinvocation tests, formatting,
  strict workspace clippy, documentation checks, diff checks, and all seven
  required GitHub checks passed
- out-of-kernel work: source inspection, security analysis, review authoring,
  validation, and later git and pull-request actions
