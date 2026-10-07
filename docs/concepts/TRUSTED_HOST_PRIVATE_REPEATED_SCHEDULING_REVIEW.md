# Private Trusted-Host Repeated Scheduling Review

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups. Proceed to planning for one
private production trusted-host timer integration.**

The merged driver is finite, crate-private, and authority-free. Every attempted
wake composes the accepted schedule-once boundary, so trusted time, current
wait posture, transition, authority consumption, and executor admission remain
inside Core. No finding permits unbounded repetition, stale-authority reuse, or
public scheduling behavior.

## 2. Scope Verification

The phase stayed within its approved boundary. It added one private wake
budget, one private wake-identity provider contract, one private repeated
driver, focused tests, and phase documentation.

It did not add a production timer, daemon, queue, general scheduler, public
API, CLI, SDK, schema, runtime configuration, automatic model turns,
automatic approval, provider mutation, OpenShell execution, nested harnesses,
hosted scheduling, PostgreSQL scheduling, filesystem artifacts, or release
changes.

## 3. Finite Construction Assessment

`TrustedHostRepeatedWakeBudget` accepts only values from one through eight.
The driver uses a finite `for` range and counts every successful waiter return,
including cancellation and early wake. A waiter that returns immediately can
therefore consume at most eight attempts in one call.

Budget exhaustion returns non-terminal `AwaitCondition` posture with an
explicit `WakeBudgetExhausted` stop reason. It does not fail or complete the
workflow and does not grant authority.

Checked wake and executor-entry accounting prevents numeric wrap, although the
private maximum already keeps ordinary counts far below numeric limits.

## 4. Fresh-State Assessment

Every iteration calls `schedule_trusted_host_time_window_once`. That helper
derives the current handoff and inert ticket from durable Core state, waits
once, reassesses readiness, and enters the accepted reinvocation boundary only
when Core proves eligibility.

The repeated driver does not retain a ticket, handoff, capability, wait
binding, or skill output between iterations. Early wake and executor yield
both return to the same fresh schedule-once derivation path.

## 5. Identity Assessment

The driver requests a new operation and receipt identity before every
schedule-once attempt. The deterministic test provider emits distinct
identities and proves one provider call per attempted wake.

The private provider contract does not itself prove uniqueness. A faulty
provider that repeats an identity is still fail-closed at the lower operation
and receipt binding boundary, but a direct duplicate-identity regression test
should be added before production timer integration.

## 6. Cancellation And Failure Assessment

Cancellation stops after one waiter call and produces a typed host-level stop.
Waiter unavailable and failed outcomes propagate as structured errors and are
not retried. Lower stale-binding, trusted-time, corruption, ambiguity, replay,
and identity-provider failures also return immediately.

The driver translates two private schedule-once posture error codes into
bounded stop outcomes, then reads the current durable disposition. This is
safe inside the current private module, but matching error-code strings is a
maintenance-sensitive seam. A typed private classification should be
considered before broader integration.

## 7. Restart Assessment

The driver persists no host job, callback, ticket, remaining budget,
capability, or authority. A restarted process must construct a new bounded
call and rehydrate from durable Core state. Existing schedule-once restart and
transition-replay tests prove the lower boundary used by every iteration.

There is no direct repeated-driver restart test. This is non-blocking for the
private composition, but should be added before a production host owns timer
recovery. The host wake budget intentionally resets after restart and is not a
durable governance limit.

## 8. Concurrency Assessment

The driver adds no shared scheduler state, leader election, or lease.
Competing callers contend at the accepted exact wait transition, directive
consumption, and dispatch-reservation boundaries, which already prove one
winner and at-most-once executor entry.

There is no direct test running two repeated drivers against one window and
asserting aggregate executor entries. The lower proofs are sufficient for this
crate-private phase, but that composition-level test should precede production
timer integration.

## 9. Timing And Test Determinism Assessment

The sequential two-wait test uses short real-time intervals because the
production schedule-once helper reads the Core-owned trusted clock. The
injected waiter performs no polling, and the test passed locally and in all
required CI checks.

Real sleeps remain a flake and latency risk. A reviewed trusted-time test seam,
or another deterministic way to advance the Core-owned observation, should be
preferred before this path becomes a production integration dependency.

## 10. Privacy And Redaction Assessment

The private input, identity, and outcome types do not derive serialization.
Custom `Debug` output redacts operation, receipt, run, workflow, step, actor,
bundle, and input bindings. The returned outcome contains only disposition,
bounded counts, and a stop reason. Skill output is not retained.

Errors use stable codes and fixed messages. No paths, deadlines, commands,
payloads, credentials, provider data, or source contents are exposed.

## 11. Test Quality Assessment

Focused tests cover:

- zero, maximum, and above-maximum budgets;
- two sequential exact time waits and terminal stop;
- early-wake budget exhaustion without Core mutation;
- cancellation without re-arm;
- waiter unavailable and failed outcomes without retry;
- unsupported posture without waiter or executor entry;
- fresh identity-provider calls; and
- redaction-safe bounded models.

The full workspace suite preserves accepted lower restart, stale-binding,
replay, and concurrent-callback proofs. Missing direct repeated-driver
restart, competing-driver, duplicate-identity, and deterministic-clock tests
are non-blocking follow-ups, not hidden coverage claims.

## 12. Documentation Assessment

The roadmap, plan, and implementation report accurately describe the private
scope, maximum wake count, stop behavior, non-authorizing posture, test
coverage, and remaining limitations. They do not claim a production timer,
public scheduler, automatic continuation service, or hosted behavior.

## 13. Blockers

None.

## 14. Non-Blocking Follow-Ups

- Add a direct competing repeated-driver test that measures aggregate executor
  entries.
- Add a direct restart/reconstruction test at the repeated-driver boundary.
- Add a duplicate wake-identity provider regression test.
- Replace private error-code matching with a typed classification if the
  boundary grows.
- Reduce or remove short wall-clock sleeps through a reviewed deterministic
  trusted-time test seam.
- Keep wake-budget reset, host-job durability, and job correlation explicit in
  any production timer plan.

## 15. Recommended Next Phase

Plan one private production trusted-host timer integration that calls the
accepted repeated driver without gaining authority. The plan must define
process ownership, cancellation, restart, bounded reconstruction, timer
failure, duplicate callback handling, job correlation, and operator
disclosure before implementation.

Do not expose public scheduling configuration, add a general scheduler, grant
automatic approval, create model turns, broaden provider mutation, add
OpenShell execution, implement nested harnesses, or change release posture.

## 16. Governed Review Record

- workflow: `dg/review`
- run: `run-1791358113068199000-2`
- approval:
  `approval/run-1791358113068199000-2/review-scope-approved`
- presentation: `presentation/33e0ee7f56d9d3fb`
- presentation hash:
  `33e0ee7f56d9d3fbe2be3fab513dceb7e5dfcc8152b5c35fa9b7ad8c552da648`
- approval outcome: granted by delegated maintainer through proof enforcement
- reviewed merge commit: `15dc4a02cc7a6a23623f98ec789f3c843dc9311f`
- approved boundary: focused maintainer/security review only
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations;
  approval presentation proof marker present
- validation summary: 5 focused tests passed; formatting, strict clippy,
  workspace tests, documentation checks, and diff checks passed
- out-of-kernel work: source, test, and documentation inspection; review
  authoring; validation; and later git and pull-request actions
