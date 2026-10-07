# Trusted-Host Explicit Local Caller Adoption Report

## 1. Executive Summary

Workflow OS now has one additive crate-private composition that adopts the
accepted synchronous trusted-host production caller for an explicitly selected
local SQLite operation. The composition enters the existing operational
boundary exactly once and routes only an authoritative `AwaitCondition` stop
into the existing production caller. Every other bounded entry result returns
unchanged.

The implementation does not discover work, create authority, reconstruct a
capability, start a background owner, or expose scheduling through a public
surface. It remains synchronous, process-owned, local, injected, and private.

## 2. Scope Completed

- Added `TrustedHostExplicitLocalOperationInput` around the exact existing
  operational-entry input and an owner-created cancellation receiver.
- Added a closed outcome that distinguishes an initial entry stop from a
  production-caller continuation stop.
- Added `run_explicit_trusted_host_local_operation` as a private composition.
- Preserved the same backend, locator binding, executor, and `SkillInput`
  across initial entry and continuation.
- Routed only Core-derived `AwaitCondition` into the accepted production
  caller.
- Added focused composition tests for terminal bypass, one and two lawful
  wakes, no-wait yield handling, cancellation, substitution failure, restart,
  and redacted Debug output.

## 3. Scope Explicitly Not Completed

This phase did not add run discovery, startup scanning, a daemon, a detached
task, a background thread, a durable host job, public API, CLI, SDK, schema,
workflow configuration, LocalExecutor integration, automatic approval,
provider reads or mutations, OpenShell, nested harness execution, hosted or
PostgreSQL scheduling, new wait families, a configurable wake budget, or a
release posture change.

## 4. Private Composition Summary

`run_explicit_trusted_host_local_operation` destructures one exact
`TrustedHostOperationalEntryInput`, retaining only bounded clones of its
locator and in-memory `SkillInput` for a possible lawful continuation. It calls
`enter_trusted_host_operation` exactly once.

When entry returns anything other than `AwaitCondition`, the complete entry
outcome is returned as `EntryStopped`. When entry returns `AwaitCondition`, the
composition calls `run_trusted_host_local_production_caller` once with the same
backend, locator binding, executor, skill input, and owner cancellation
receiver. The caller's complete bounded outcome returns as
`ContinuationStopped`.

The routing result is not authority. The production caller reloads durable Core
state and relies on the existing reservation, directive, supervisor, and
reinvocation boundaries before every executor entry.

## 5. Authority And Binding

The implementation clones no capability and accepts no caller-authored
eligibility, wait satisfaction, approval, wake budget, or resume claim. The
only cloned values are inert identity/binding data in the private locator and
the exact in-memory `SkillInput` required by the already-reviewed caller.

Changed invocation input fails at the operational-entry commitment check
before identity generation or executor entry. Current Core state remains the
source of truth for authority, continuation disposition, reservation, replay,
and terminal posture.

## 6. Stop, Output, And Workflow Semantics

Immediate terminal, blocked, and other non-awaiting entry postures bypass the
timer and return unchanged. A no-wait turn-boundary yield derives
`ResumeNow` and remains inside the operational-entry redispatch path; it is not
misclassified as an external scheduled wait.

The accepted production caller preserves terminal, blocked, canceled,
unsupported-wait, and wake-budget-exhausted stop reasons. The composition does
not convert errors into normal outcomes, invent retry advice, or claim workflow
completion. An initial `AwaitCondition` is produced by a yielded attempt and
therefore carries no successful `SkillOutput` for this routing to discard.

## 7. Cancellation, Restart, And Concurrency

The local owner creates the existing cancellation pair and retains the handle.
Cancellation wakes a production wait and returns bounded canceled posture
without granting authority or changing workflow lifecycle semantics.

Restart uses a reopened backend, the exact durable locator and immutable run
binding, a fresh cancellation pair, and the production caller's fresh
identities. Existing atomic reservation and replay boundaries remain the
authority for competing owners. This composition adds no retry loop around a
losing or stale caller.

## 8. Privacy And Debug

The new input has custom bounded Debug output that exposes only a redacted
binding marker. The outcome delegates to the already-bounded entry and
repeated-scheduling outcomes. Focused tests confirm that run identity, skill
output markers, and secret-like substituted input do not appear in input,
outcome, or error Debug output.

The composition adds no serde surface, workflow event, audit event, report
artifact, metric, host record, payload persistence, or operator log.

## 9. Test Coverage

Direct composition tests cover:

- terminal entry bypass with no scheduling or identity generation;
- one durable `TimeWindow` yield routed through one lawful wake;
- two durable `TimeWindow` yields within the fixed production wake budget;
- no-wait `ResumeNow` yield retained in operational entry;
- owner cancellation after the initial yielded attempt;
- secret-like substituted `SkillInput` rejected before executor entry;
- reopened-backend continuation with exact binding; and
- bounded input, outcome, and error Debug behavior.

The unchanged lower-layer suites continue to cover unsupported waits,
wake-budget exhaustion, early wakes, deadline failures, replay conflicts,
competing reservation winners, stale bindings, identity generation, and
zero-write cancellation for the exact primitives composed here. Those
guarantees are inherited, not reimplemented by this wrapper.

## 10. Validation

- focused explicit-local-operation tests: passed
- `cargo fmt --all --check`: passed
- `cargo clippy --workspace --all-targets -- -D warnings`: passed
- `cargo test --workspace`: passed
- `npm run check:docs`: passed
- `git diff --check`: passed

## 11. Remaining Limitations

- The composition has no public or automatic caller.
- There is no discovery, detached ownership, owner-loss detection, operator
  notification, durable host-job model, or startup recovery loop.
- Terminal output retrieval is not widened beyond existing private outcomes.
- The fixed two-wake production budget remains unchanged and is not restored
  across a new explicit owner call.
- Composition-level competing-owner fault injection remains proven by the
  unchanged reservation and replay primitives rather than duplicated here.
- The implementation is local SQLite only and makes no hosted parity claim.

## 12. Recommended Next Phase

Perform focused maintainer/security review of this implementation. Do not add a
public owner, run discovery, automatic scheduling, provider mutation,
OpenShell, nested harnesses, hosted scheduling, CLI, SDK, schemas, or release
changes until that review accepts the composition.

## 13. Governed Implementation Record

- workflow: `dg/implement`
- run: `run-1791367158708914000-2`
- approval: `approval/run-1791367158708914000-2/implementation-approved`
- presentation: `presentation/68a54c1095c1fd04`
- presentation hash:
  `68a54c1095c1fd04fa3cc2e3c091ee9cc85b38473a9d1dfad215e4d84fd9bad7`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: one private explicit local operational-entry composition
- phase status: completed
- event summary: 39 events, including 1 approval request, 1 approval grant,
  8 policy decisions, 6 scheduled steps, 6 successful skill invocations, 0
  retries, and 0 escalations
- approval-presentation enforcement: proof enforced with one matching record
  and event marker
- validation summary: 7 focused tests, formatting, warning-denied workspace
  clippy, full workspace tests, docs checks, and diff hygiene passed
- out-of-kernel execution: source inspection, code edits, tests,
  documentation, and later git and pull-request actions are performed by the
  delegated trusted host; Workflow OS governs scope and approval but does not
  edit files or execute shell commands
