# Trusted Host Opaque Wait Handoff Implementation Report

## 1. Executive Summary

Workflow OS now has a crate-private, observation-only handoff for one exact
authorized execution window that is durably waiting on an unsatisfied
`TimeWindow` condition. Core derives the handoff from one coherent SQLite read
transaction and returns it only when the existing authoritative continuity
semantics classify that same snapshot as `AwaitCondition`.

The handoff is inert. It is not durable state, a bearer capability, a wake
assessment, a dispatch reservation, or executor authority.

## 2. Scope Completed

- Added dedicated crate-private handoff, handoff-ID, condition, dependency,
  condition-state, next-operation, and observation-outcome types.
- Added deterministic domain-separated handoff and cursor commitments.
- Added an observation path that validates the exact private operational-entry
  locator and reads the continuity snapshot inside an explicit SQLite read
  transaction.
- Reused the existing authoritative continuation classifier.
- Added `TimeWindow` as the only accepted dependency kind.
- Added `RequestFreshClassification` as the only next operation.
- Added focused deterministic, restart, blocked-posture, no-write, redaction,
  and concurrent snapshot tests.

## 3. Scope Explicitly Not Completed

This phase did not add wait satisfaction, sleeping, polling, timers, queues,
scheduling, wake transport, automatic reinvocation, agent-turn management,
dispatch reservation, directive consumption, provider execution, OpenShell,
sandbox execution, nested harnesses, public Rust APIs, serde, workflow schema,
CLI, SDK, UI, hosted behavior, writes, or release changes.

## 4. Model Summary

`TrustedHostWaitHandoffId` is a dedicated private type backed by a
domain-separated commitment. `TrustedHostWaitHandoff` retains the exact private
selectors required for correlation, but custom Debug output exposes only the
authoritative disposition, closed next-operation vocabulary, condition count,
and a redacted binding marker.

The handoff contains a commitment to the continuation cursor, not the raw
cursor. No handoff type implements serde or is exported from `workflow-core`.

## 5. Coherent Observation Boundary

The observation path opens an explicit deferred SQLite transaction, loads and
projection-validates the complete continuity snapshot, obtains a trusted-time
observation, validates its source, provenance, and epoch against that snapshot,
runs the existing continuation classifier, and derives any handoff before
ending the same transaction.

It does not call `continuation_disposition` and then independently reload rows.
The concurrency test commits a wait transition after the read snapshot is
loaded and proves the observer returns the coherent old handoff; a subsequent
fresh observation returns `ResumeNow` and no handoff.

## 6. Authority And Security Summary

- Possession or equality of a handoff grants no permission.
- No mutating method accepts a handoff or handoff ID.
- Unsupported or unbound dependencies fail closed.
- Locator mismatches return stable, value-free security errors.
- The existing source-specific transition remains the only wait-satisfaction
  boundary.
- Future callers must request fresh classification from Core.

## 7. Privacy And Redaction Summary

Debug output omits window, generation, condition, cursor, deadline, trusted
time, dependency, authority, capability, receipt, command, payload, and secret
values. Stable errors disclose only bounded failure classes. The handoff is
not serializable and no public accessor or API was added.

## 8. Test Coverage Summary

Focused tests cover:

- deterministic identity across repeated coherent reads;
- stable identity after reopening the SQLite backend;
- exactly one bounded condition and closed next operation;
- no durable state mutation during observation;
- no handoff for blocked posture;
- fail-closed unsupported dependencies and mismatched private bindings;
- redaction-safe Debug output; and
- old-or-new coherent results across a concurrent transition.

## 9. Commands Run And Results

- `cargo check -p workflow-core` with an isolated target directory: passed.
- `cargo test -p workflow-core trusted_host_wait_ -- --nocapture` with an
  isolated target directory: 5 focused tests passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings` with an isolated
  target directory: passed.
- `cargo test --workspace` with an isolated target directory: passed; live
  opt-in tests remained ignored by design.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

The ordinary shared Cargo target stalled without diagnostics during the first
probe. The same source compiled and tested normally with an isolated target
directory, indicating a local shared-target process or lock issue rather than
a source failure.

## 10. Workflow Semantics

Observation does not modify workflow status, continuity state, event history,
or runtime results. Blocked, terminal, and resumed posture remain ordinary
kernel dispositions without fabricated handoffs.

## 11. Remaining Limitations

- Only the private `TimeWindow` dependency family is supported.
- The handoff has no transport or public representation.
- No scheduler or trusted host consumes the handoff yet.
- No explicit reinvocation path exists.
- The first caller supports a tightly bounded wait shape; broader condition
  sets require separate review.

## 12. Recommended Next Phase

Perform a focused maintainer/security review of this implementation. Only after
acceptance should Workflow OS plan explicit reinvocation from freshly
reclassified authoritative state.

## 13. Governed Phase Record

- workflow: `dg/runtime-composition`
- run: `run-1791063269883483000-2`
- approval:
  `approval/run-1791063269883483000-2/composition-approved`
- presentation: `presentation/1abcbf0a43be1bb2`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approved boundary: private handoff model and coherent observation only
- validation summary: focused and workspace Rust tests, formatting, clippy,
  documentation checks, and diff checks passed
- out-of-kernel work: source inspection, Rust edits, tests, documentation,
  validation, and later git/PR work
- missing coverage: the kernel coordinated governance only; it did not edit
  files, run tests, create a WorkReport artifact, or perform git actions
