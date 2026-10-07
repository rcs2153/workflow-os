# Trusted-Host Private Schedule-Once Helper Report

## 1. Executive Summary

Workflow OS now has one crate-private, injected schedule-once composition for
an exact local SQLite `TimeWindow` wait. Core derives the current inert handoff
and absolute-UTC ticket, gives only the bounded instant to an injected waiter,
reassesses authoritative readiness once after wake, and invokes the accepted
explicit reinvocation path at most once.

The helper is not a scheduler loop. It does not poll, re-arm, persist a host
job, create a model turn, approve work, or expose public runtime configuration.
A timer wake remains only a request for fresh Core verification.

## 2. Scope Completed

- Added a crate-private injected deadline-wait interface.
- Added bounded `Woke`, `Canceled`, `Unavailable`, and `Failed` host outcomes.
- Added a crate-private schedule-once input and closed result vocabulary.
- Composed the accepted coherent observation, readiness assessment, and
  explicit reinvocation boundaries.
- Added focused elapsed, early, cancellation, host-failure, restart, and
  concurrent-callback tests.
- Updated the accepted plan and roadmap honestly.

## 3. Scope Explicitly Not Completed

This phase does not add repeated scheduling, polling, automatic re-arming, a
scheduler store, queue, daemon, timer driver, worker pool, automatic model or
assistant turns, automatic approval, another wake family, public Rust API,
CLI, SDK, schema, workflow configuration, provider execution, OpenShell,
nested harnesses, hosted behavior, or release changes.

## 4. Private Helper Boundary

`schedule_trusted_host_time_window_once` accepts explicit backend, immutable
operational locator, reinvocation operation and receipt identities, injected
deadline waiter, executor, skill input, and redispatch identity provider.

It internally derives the current handoff and ticket. Callers cannot supply a
timestamp, handoff, condition identity, or condition version. The waiter sees
only the exact absolute-UTC scheduling instant and receives no authority or
access to Core state.

## 5. One-Shot Algorithm

The helper performs this closed sequence:

1. derive one coherent authoritative scheduling observation;
2. reject non-actionable posture before entering the host wait;
3. call the injected waiter exactly once with the private ticket instant;
4. return typed cancellation or bounded host failure without mutation;
5. after wake, perform one fresh read-only readiness assessment;
6. return refreshed inert `NotYetEligible` posture on an early wake; or
7. call the accepted explicit reinvocation path exactly once when eligible.

There is no loop, sleep implementation, retry, polling call, or automatic
re-registration in the helper.

## 6. Authority And Atomicity

Neither the ticket, waiter wake, nor readiness result grants authority. The
accepted reinvocation path still verifies the exact handoff and dependency,
obtains trusted time inside the mutation transaction, commits or exactly
replays the wake, reloads current state, consumes fresh one-use authority, and
reserves executor dispatch atomically.

Concurrent scheduled callbacks therefore admit at most one executor entry.
The loser receives a bounded terminal zero-entry result or a fail-closed stale
or replay rejection from the accepted lower boundary.

## 7. Cancellation And Failure Behavior

- `Canceled` returns a typed non-terminal host outcome.
- `Unavailable` returns
  `trusted_host_time_window_scheduling.deadline_wait_unavailable`.
- `Failed` returns
  `trusted_host_time_window_scheduling.deadline_wait_failed`.
- None of these outcomes mutates continuity state, invokes the executor, or
  fabricates workflow failure or completion.
- `NotYetEligible` also performs no mutation and does not re-arm itself.

## 8. Restart Behavior

The helper persists no ticket, timer registration, or authority. After SQLite
reopen, the caller supplies only the immutable operational locator and Core
derives a fresh ticket and handoff from authoritative state before waiting and
reinvoking. The restart proof reaches one terminal executor entry without
reconstructing a capability.

## 9. Privacy And Redaction

- Helper input Debug output redacts all bindings.
- Early-wake output Debug redacts refreshed scheduling posture.
- Host failures map to stable static codes and messages.
- No timestamp, workflow/run/step identifier, path, commitment, payload,
  command, prompt, credential, or provider value is echoed in errors.
- No serde, public output, workflow schema, CLI, or SDK surface is added.

## 10. Test Coverage

Focused tests prove:

- one elapsed wake produces one reinvocation and one executor entry;
- one early wake returns refreshed `AwaitCondition` posture with no write and
  no re-arm;
- cancellation and both host-failure categories preserve byte-equivalent
  continuity state and zero executor entry;
- SQLite restart derives fresh scheduling state and reinvokes once;
- two concurrent callbacks call their injected waiters once each but admit
  one aggregate executor entry; and
- the durable disposition is terminal after the winning callback.

Existing lower-level observation, readiness, explicit reinvocation, authority,
continuity, runtime, adapter, and report tests remain part of workspace
validation.

## 11. Commands Run And Results

- Focused schedule-once tests: passed, 5 tests.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 12. Remaining Limitations

- The caller must invoke this private helper; no runtime automatically finds
  or schedules waits.
- The injected waiter has no production timer implementation in Core.
- Only one exact local SQLite `TimeWindow` wait is supported.
- There is no repeated scheduler, durable host-job record, recovery daemon,
  automatic agent continuation, or hosted/multi-host parity.
- Other wait families remain explicit and unscheduled.

## 13. Recommended Next Phase

Perform focused maintainer/security review of this schedule-once composition.
Verify the one-call boundary, zero-write early/cancel/failure behavior,
fresh-state restart behavior, concurrent at-most-once entry, privacy, and the
retained atomic authority boundary.

Do not broaden into repeated scheduling, automatic model continuation,
additional wake families, public configuration, provider execution,
OpenShell, nested harnesses, or hosted behavior during that review.

## 14. Governed Phase Record

- workflow: `dg/implement`
- run: `run-1791353944724097000-2`
- approval:
  `approval/run-1791353944724097000-2/implementation-approved`
- presentation: `presentation/4feaffbace7d48d7`
- approval outcome: granted by delegated maintainer through proof enforcement
- approved boundary: private waiter/input/outcome models, one-shot composition,
  focused tests, roadmap, accepted plan, and implementation report only
- phase status: `Completed`
- event summary: 39 events, one approval, zero retries, zero escalations;
  approval-presentation proof enforcement present
- validation summary: focused tests, formatting, strict workspace clippy, full
  workspace tests, docs checks, and diff checks passed
- out-of-kernel work: source inspection, Rust implementation, tests,
  documentation, validation, and later git and pull-request actions
