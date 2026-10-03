# Trusted Host TimeWindow Caller Implementation Report

## 1. Executive Summary

Workflow OS now has its first crate-private trusted-host wake caller. The
caller applies one already-registered exact `TimeWindow` wait through the
accepted SQLite verifier and returns bounded transition status plus the
kernel's current continuation posture.

The caller does not schedule, sleep, poll, invoke an executor, create wake
authority, or expose a public API. Satisfying a timer does not imply that work
may execute: the returned posture can remain `Blocked` when current resume
authority is unavailable.

## 2. Scope Completed

- Added one crate-private synchronous `TimeWindow` caller.
- Reused the existing private operational-entry locator for exact workflow,
  run, step, window, actor, and immutable-bundle binding.
- Derived window revision, cursor, generation, wait revision, governance
  commitment, and authority commitment from authoritative SQLite state.
- Invoked the accepted transaction-owned trusted-time verifier.
- Returned only `Transitioned`, `ExactReplay`, or `SecurityRejected` and the
  current authoritative continuation disposition.
- Recovered an exact operation replay after reopening the SQLite backend.
- Rejected conflicting operation reuse and stale locator binding.
- Preserved one-winner behavior under competing transitions.

## 3. Scope Explicitly Not Completed

- No scheduler, daemon, loop, sleep, timer service, poller, queue, or wake bus.
- No executor invocation or automatic reinvocation after transition.
- No supervisor support for executor-authored typed wait declarations.
- No opaque host wait handoff.
- No approval, evidence, check, external-event, capability, authority-refresh,
  or conflict wake source.
- No provider execution or mutation, OpenShell, or sandbox integration.
- No nested harnesses, recursive agents, or agent swarms.
- No public API, workflow schema, CLI, SDK, UI, example, hosted, or distributed
  surface.
- No release-posture change.

## 4. Caller API Summary

`TrustedHostTimeWindowWakeInput` accepts the private operational locator, one
condition identity and version, and fresh operation and receipt identities.
It does not accept revisions, cursors, generation identity, deadline claims,
timestamps, boolean satisfaction assertions, or wake capability values.

`apply_trusted_host_time_window_wake` loads validated canonical SQLite state,
checks the locator against the current window, derives the exact transition
request, invokes `transition_time_window_wait`, and then asks Core for current
continuation disposition.

## 5. Authority And Trusted-Time Boundary

The caller does not construct `WakeAssessmentCapability`. The accepted
verifier obtains one trusted-time observation inside the SQLite transaction,
checks the exact durable dependency binding, and constructs any private
one-use wake authority in that same boundary.

The caller's result is orientation, not execution authority. A successful
timer transition cannot directly call the executor or consume a resume
directive.

## 6. Replay And Restart Behavior

For a fresh operation, mutable expectations come from canonical current state.
For an existing operation, the caller cross-checks durable operation kind,
receipt, window, condition identity, and condition version before returning a
bounded replay result. This permits exact recovery after process restart while
rejecting changed receipt or binding reuse.

The canonical snapshot loader continues to cross-check normalized operation
columns and serialized operation state before the caller trusts them.

## 7. Failure And Concurrency Posture

- Wrong locator binding fails with a stable security error.
- Missing, non-deadline, unbound, transitioned, or otherwise ineligible waits
  fail closed.
- Conflicting operation replay fails with a stable security error.
- Durable trusted-time rejection is returned as bounded
  `SecurityRejected`; raw source details are not exposed.
- Competing distinct transition operations retain one-winner behavior through
  SQLite transaction serialization and exact revisions.

## 8. Privacy And Redaction

The caller persists no new payload family. It uses bounded identifiers,
revisions, commitments, and canonical continuity records. Debug for caller
input emits only a redacted binding marker. Errors do not include deadlines,
paths, timestamps, provenance values, epochs, commands, source content,
credentials, tokens, or provider payloads.

## 9. Test Coverage

Focused tests cover:

- authoritative-state-derived successful transition;
- exact replay after reopening the backend;
- conflicting replay rejection;
- stale locator rejection;
- one winner under competing caller transitions; and
- redaction-safe Debug output.

Existing direct-verifier tests continue to cover one transaction observation,
ambiguous-commit replay, conflicting replay, competing transitions, and legacy
unbound wait rejection.

## 10. Commands Run And Results

- `cargo test -p workflow-core --lib trusted_host_time_window_caller --no-fail-fast`:
  passed, 5 tests.
- `cargo fmt --all --check`: passed after formatting.
- `cargo clippy -p workflow-core --lib --tests -- -D warnings`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed; all default workspace tests completed with
  only explicitly opt-in live integration tests ignored.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 11. Remaining Known Limitations

- No accepted runtime path yet registers a genuine executor-declared
  `TimeWindow` wait through the trusted-host supervisor.
- No host-facing opaque wait handoff exists.
- The caller performs one explicit operation only; it does not provide
  autonomous continuation.
- SQLite remains local and non-tamper-resistant against a privileged local
  actor.
- Additional wake-source families require separate source-specific plans and
  verifiers.

## 12. Recommended Next Phase

Perform a focused maintainer/security review of the first private trusted-host
`TimeWindow` caller. Verify durable replay identity, locator binding, bounded
result semantics, corruption posture, and the absence of execution authority.

Only after acceptance should the roadmap choose between the private typed-wait
registration slice and the opaque wait-handoff composition slice. Do not begin
provider, sandbox, nested-harness, public scheduling, or additional wake-source
work first.

## 13. Governed Phase Record

- workflow: `dg/runtime-composition`
- run: `run-1791042657246477000-2`
- approval: `approval/run-1791042657246477000-2/composition-approved`
- presentation: `presentation/ed94671d46868c93`
- presentation hash:
  `ed94671d46868c931ade1dfdf7a169604a536357b9725f340af86b2c61aef55c`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval presentation enforcement: proof enforced
- out-of-kernel work: code edits, tests, documentation, git operations, and
  validation were performed by the delegated trusted host; Workflow OS
  governed scope and approval but did not execute those actions
