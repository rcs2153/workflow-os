# Private Trusted-Host Production Timer Review

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups. Proceed to planning one explicit
local production caller.**

The implementation is a narrow, production-quality blocking primitive over
the accepted bounded repeated scheduling driver. It does not become a
scheduler, create authority, discover runs, or call the executor outside the
reviewed Core path.

## 2. Scope Verification

The phase stayed within the approved private implementation scope. It added a
crate-private synchronous timer, private cancellation state and handle, one
thin repeated-driver wrapper, focused tests, roadmap status, and an
implementation report.

It did not add a production caller, daemon, queue, detached worker, run
discovery, durable timer job, public API, CLI, SDK, schema, runtime
configuration, automatic approval, provider mutation, OpenShell, nested
harnesses, hosted scheduling, report artifact, or release change.

## 3. Host And Core Boundary Assessment

The host timer owns only blocking and cancellation. It receives an inert
absolute deadline and returns `Woke` or `Canceled` through the existing waiter
contract. It does not inspect workflow state or claim that a wait condition is
satisfied.

After every wake, the accepted schedule-once and repeated-driver path reloads
current durable state, reads Core-owned trusted time, validates the ticket and
handoff, and controls executor admission. This preserves the governing
boundary under delayed, early, duplicate, or competing host wakeups.

## 4. Timer Primitive Assessment

`Mutex` plus `Condvar::wait_timeout` is an appropriate standard-library
implementation. The internal loop checks cancellation, reads host time,
computes a checked duration, and absorbs spurious notification without
polling Core.

The timer uses no detached thread, periodic polling interval, external crate,
filesystem state, or hidden global. The wrapper delegates exactly once to the
accepted bounded repeated driver and adds no second scheduling loop.

## 5. Cancellation Assessment

Cancellation is explicit, shared, prompt, and idempotent. Signaling sets one
boolean and wakes current waiters. A canceled wait returns the existing typed
stop without changing workflow disposition, appending an event, or entering
the executor.

Dropped handles intentionally do not cancel while the synchronous caller
continues to own the wait. That posture is acceptable only for this private
synchronous slice; any detached owner requires separately reviewed owner-loss
semantics.

## 6. Clock And Deadline Assessment

Host wall time is used only to calculate a blocking duration. A timeout never
grants authority. Forward clock movement may delay a `Condvar` timeout until
the originally calculated host duration, and backward movement may produce
another bounded wait. Core reassessment prevents either case from creating
eligibility.

Checked conversion and elapsed-deadline behavior fail closed. The production
timer has no direct deterministic forward/backward clock-jump test; this is a
non-blocking coverage follow-up because the authority boundary is independently
enforced after wake.

## 7. Restart And Recovery Assessment

The timer persists no callback, capability, ticket, deadline job, remaining
budget, or cancellation state. The integrated reopen test reconstructs from a
fresh backend handle, durable locator, fresh finite budget, and caller-supplied
identity providers.

There is no automatic startup scanning or process recovery. The implementation
therefore proves restart-safe reconstruction, not automatic continuation.

## 8. Identity And Concurrency Assessment

Wake and redispatch identities remain explicit caller inputs. Timer mechanics
do not mint execution authority.

The duplicate-identity test proves that reusing one wake identity across
different attempts fails closed with the existing replay-conflict code. The
competing-wrapper test proves aggregate executor entry remains at most one,
while allowing the losing caller to observe any valid stale, blocked,
terminal, or replay posture.

## 9. Failure And Privacy Assessment

Poisoned synchronization state and invalid duration conversion become fixed
waiter failure rather than panic, retry, workflow failure, or completion.
Cancellation-state errors use a stable code and fixed message.

The new private types do not derive serialization. Their bounded `Debug`
implementations omit deadlines, identifiers, paths, commands, payloads,
credentials, source contents, provider data, and synchronization internals.

## 10. Test Quality Assessment

Thirteen focused tests cover elapsed and future waits, cancellation before and
during a wait, repeated cancellation, spurious notification, poisoned state,
bounded Debug, integrated executor entry, sequential waits, zero-write
cancellation, backend reopen, duplicate identity, and competing callers.

The tests assert workflow state and aggregate executor entry rather than only
constructing types. Existing workspace tests also passed.

Non-blocking gaps:

- forward and backward host-clock movement are not directly simulated at the
  timer boundary;
- impossible positive duration conversion is not directly exercised; and
- several integration tests use real 500 millisecond deadlines and should not
  become the default pattern for a broader timer suite.

## 11. Documentation Assessment

The plan, report, and roadmap accurately describe the result as a private
timer primitive. They explicitly deny automatic scheduling, public exposure,
provider mutation, hosted scheduling, and production caller support.

The clock discussion should continue to distinguish wall-clock movement from
the monotonic host timeout used by `Condvar`; delayed host wake is lawful
because Core performs the authoritative reassessment.

## 12. Blockers

None.

## 13. Non-Blocking Follow-Ups

- Add deterministic host-clock movement tests if the timer gains a production
  caller or broader platform support.
- Keep most future scheduling tests on injected waiters and reserve real-time
  waits for a small smoke boundary.
- Define identity generation and owner-loss semantics before any detached or
  automatic caller.
- Preserve fixed non-leaking synchronization errors.

## 14. Recommended Next Phase

Plan one explicit local production caller that selects this private timer from
an already-authorized trusted-host continuation path. The plan must define
caller ownership, cancellation source, fresh identity generation, restart
entry, finite budget selection, and operator-visible failure posture.

Do not implement a daemon, general scheduler, run discovery, public runtime
configuration, provider mutation, OpenShell, nested harnesses, automatic
approval, CLI, SDK, schema, hosted scheduling, or release change.

## 15. Governed Review Record

- workflow: `dg/review`
- run: `run-1791360933495437000-2`
- approval: `approval/run-1791360933495437000-2/review-scope-approved`
- presentation: `presentation/fdc26480d5c4db72`
- presentation hash:
  `fdc26480d5c4db7293679bdd480938ea965a60ca5072f6ae2f6988f16b36a9f4`
- approval outcome: granted by delegated maintainer through proof enforcement
- reviewed merge commit: `cff7af28cb8c2be1f05e5dbc7c7bd6cec45ba58c`
- approved boundary: focused maintainer/security implementation review only
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval-presentation enforcement: proof enforced with one persisted record
- validation summary: formatting, 13 focused timer tests, workspace clippy,
  workspace tests, docs checks, and diff checks passed
- out-of-kernel work: source and test inspection, review authoring, shell
  validation, and later git and pull-request actions
