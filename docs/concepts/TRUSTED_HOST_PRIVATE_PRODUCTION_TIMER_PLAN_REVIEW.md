# Private Trusted-Host Production Timer Plan Review

## 1. Executive Verdict

**Plan accepted with non-blocking implementation follow-ups. Proceed to the
crate-private synchronous local timer implementation.**

The plan preserves the governing boundary: the host may block and signal a
wake, while Core remains the only authority for trusted-time validation,
current wait posture, transition, and executor admission. The proposed
`Condvar` timer is a private production-quality primitive, not a production
scheduling product or automatic continuation service.

## 2. Scope Verification

The plan stays within planning-only scope. It defines one private local timer,
explicit cancellation, a synchronous wrapper around the accepted repeated
driver, failure posture, privacy rules, focused tests, and review sequencing.

It does not authorize implementation in the planning phase, a daemon, queue,
worker pool, general scheduler, durable timer jobs, run discovery, detached
threads, public API, CLI, SDK, schema, runtime configuration, automatic model
turns, automatic approval, provider mutation, OpenShell, nested harnesses,
hosted scheduling, PostgreSQL scheduling, artifacts, or release changes.

## 3. Host And Core Boundary Assessment

The proposed flow is correct. The timer consumes an inert absolute deadline
and owns only local blocking and cancellation. It does not interpret the wait
as satisfied. Every host wake returns through the accepted repeated driver and
schedule-once helper, where Core reads current trusted time and durable state.

The integration wrapper must remain a thin construction boundary with no
second repetition loop, status reinterpretation, direct event append, direct
state mutation, or executor call.

## 4. Placement Assessment

A private sibling module in `workflow-core` SQLite state is appropriate for
the first slice because all accepted scheduling contracts are crate-private.
Placing the timer in the CLI would create a premature product contract;
placing it in the hosted crate would imply distributed scheduling.

This placement should not be mistaken for a permanent host architecture. A
later explicit caller plan may justify moving or exposing a narrower
interface, but this implementation should not anticipate that decision.

## 5. Waiting Primitive Assessment

`Mutex` plus `Condvar::wait_timeout` is the smallest standard-library boundary
that supports a real deadline and prompt cancellation without polling Core.
The waiter may loop internally after spurious operating-system wakeups, but
that loop must inspect only cancellation and host time.

Checked conversion from the Core-produced UTC deadline to a host duration is
required. Poisoned synchronization state, impossible conversion, and other
host failures must become fixed non-leaking waiter failures rather than panic
or fabricated cancellation.

## 6. Cancellation Assessment

The plan correctly makes cancellation explicit, idempotent, and
non-authorizing. Cancellation before or during a wait returns the existing
typed canceled posture and does not mutate, fail, or complete the workflow.

The implementation must choose and document handle-drop behavior. For the
first synchronous slice, dropped handles may leave cancellation false because
the calling thread still owns the wait. Any future detached or background
owner must fail closed on owner loss and requires separate review.

## 7. Clock-Movement Assessment

The timer may use wall time only to calculate host blocking duration. A
backward jump may require another bounded wait; a forward jump may delay the
host wake until the original monotonic timeout. Neither condition creates
authority because Core reassesses current trusted time after wake.

The implementation should avoid periodic clock polling. A timeout or explicit
signal is enough. Long but representable waits remain cancelable; impossible
durations fail closed.

## 8. Identity Assessment

Keeping wake and redispatch identity providers injected is the correct first
boundary. Timer mechanics must not mint governance identity. The production
timer therefore remains useful as a real wait primitive without silently
claiming a complete automatic caller.

A duplicate wake-identity regression test is required before acceptance. It
must prove that reused identity cannot enter the executor twice or advance a
different wait.

## 9. Restart And Recovery Assessment

The timer persists no deadline job, callback, cancellation state, capability,
ticket, or remaining budget. Process exit loses the wait. A later explicit
caller must reconstruct from the durable operational locator, immutable
inputs, fresh budget, and fresh identity providers.

A direct reconstruction test at the integrated timer boundary is required.
Automatic startup scanning and run discovery remain prohibited.

## 10. Concurrency Assessment

The plan appropriately delegates safety under competing callers to the
accepted Core one-winner transitions. The timer itself provides no lease or
leader election.

A direct competing-integration test is required. It should measure aggregate
executor entries rather than demand one exact loser error, because the loser
may lawfully observe blocked, terminal, stale, or replay posture after the
winner advances state.

Repeated cancellation notifications and spurious `Condvar` signals must not
produce duplicate transitions.

## 11. Failure And Privacy Assessment

The proposed failure matrix is conservative: cancellation and finite stops are
typed outcomes; synchronization, security, corruption, ambiguity, replay, and
identity failures are structured errors without retry. Host failure cannot be
converted into workflow failure or completion.

Private models should expose only bounded posture, counts, and booleans.
Deadlines, identifiers, paths, commands, payloads, credentials, source
contents, provider data, and synchronization details must remain absent from
Debug, errors, and serialization.

## 12. Test Plan Assessment

The planned tests cover the implementation risk:

- elapsed and future deadlines;
- cancellation before and during wait;
- idempotent cancellation and spurious notification;
- synchronization and conversion failure;
- Core reassessment after early wake;
- sequential waits and finite budget exhaustion;
- duplicate identity rejection;
- reconstruction after restart;
- competing integration at-most-once entry;
- no retry on cancellation, host failure, or security failure;
- no direct state, event, artifact, provider, CLI, or schema behavior; and
- bounded Debug and error output.

The split between deterministic tests and one generous real-time smoke test is
appropriate. Test seams must never replace Core's trusted-time decision.

## 13. Blockers

None.

## 14. Non-Blocking Follow-Ups

- Choose and document cancellation-handle drop behavior during implementation.
- Map synchronization poison consistently to the existing waiter taxonomy.
- Avoid adding host correlation unless a concrete diagnostic need appears.
- Describe the result as a private timer primitive, not automatic scheduling.
- Keep production identity generation and explicit caller integration in later
  reviewed phases.

## 15. Recommended Next Phase

Implement only the crate-private synchronous local timer, cancellation state
and handle, thin repeated-driver wrapper, deterministic focused tests, one
bounded real-time smoke test, and direct duplicate-identity, restart, and
competing-caller proofs.

Do not add a daemon, queue, background discovery, durable timer store, public
configuration, automatic approval, model-turn automation, provider mutation,
OpenShell, nested harnesses, hosted scheduling, CLI, SDK, schema, or release
changes.

## 16. Governed Review Record

- workflow: `dg/review`
- run: `run-1791359310667234000-2`
- approval:
  `approval/run-1791359310667234000-2/review-scope-approved`
- presentation: `presentation/872d98892ddfb94c`
- presentation hash:
  `872d98892ddfb94c09ce5ffbfc4643df51cec3bf4d030e98a7e2af3d358e36b8`
- approval outcome: granted by delegated maintainer through proof enforcement
- reviewed merge commit: `1fe355965ea55147bedb0c5d4f18818e60da1d4a`
- approved boundary: focused maintainer/security plan review only
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- validation summary: `npm run check:docs` and `git diff --check` passed
- out-of-kernel work: architecture and plan inspection, review authoring,
  documentation validation, and later git and pull-request actions
