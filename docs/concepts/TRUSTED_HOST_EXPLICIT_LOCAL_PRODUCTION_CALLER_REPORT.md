# Explicit Local Trusted-Host Production Caller Report

## 1. Executive Summary

Workflow OS now has one crate-private synchronous local production caller for
an explicitly selected trusted-host run. The caller composes the accepted local
timer, bounded repeated scheduler, operational-entry path, reservation boundary,
and executor without creating a parallel scheduling system.

The implementation also adds a private production identity source backed by a
direct `getrandom` 0.2.17 dependency. It constructs each redispatch or wake
identity set from one all-or-nothing entropy fill and returns the existing
bounded scheduling outcome unchanged.

## 2. Scope Completed

- Added one private caller input and synchronous caller function.
- Accepted an owner-created cancellation receiver while leaving the handle
  with the owner.
- Added production redispatch and wake identity providers.
- Fixed the caller wake budget at exactly two.
- Added deterministic test-only byte filling.
- Added focused identity, cancellation, restart, wake-budget, and execution
  tests.

## 3. Scope Explicitly Not Completed

This phase did not add a caller adoption site, run discovery, startup scanning,
a daemon, background thread, queue, public API, CLI, SDK, schema, workflow
field, runtime configuration, provider mutation, OpenShell, nested harnesses,
hosted scheduling, automatic approval, or release posture change.

## 4. Caller API Summary

`TrustedHostLocalProductionCallerInput` contains only the SQLite backend,
durable operational locator, owner-supplied cancellation receiver, exact
executor, and exact skill input. `run_trusted_host_local_production_caller`
creates private identity-provider views, selects the fixed budget, enters the
accepted timer once, and returns `TrustedHostRepeatedSchedulingOutcome`.

All types and functions remain crate-private.

## 5. Identity Construction

The production source uses `getrandom::getrandom` directly. A redispatch identity
set uses one 96-byte fill partitioned into six independent 128-bit values. A
wake identity set uses one 32-byte fill partitioned into two independent
128-bit values. Values use fixed operation-family prefixes and lowercase
hexadecimal encoding before existing typed constructors validate them.

Entropy failure returns
`trusted_host_local_production_caller.identity_generation_failed`. The error
contains no random material, binding, path, payload, or partial identity.

## 6. Cancellation And Restart

The owner creates the existing private cancellation pair, retains the handle,
and passes the receiver into the caller. Cancellation remains idempotent and
non-authorizing. It wakes a blocked call without mutating continuity state.

Restart uses a reopened backend, the same durable locator and immutable input
binding, a fresh cancellation pair, fresh identity source, and a fresh fixed
budget. No process-local timer or identity becomes durable authority.

## 7. Workflow Semantics

The caller does not change workflow semantics. Core remains authoritative for
eligibility, current authority, wait satisfaction, reservation, dispatch,
execution, and terminal state. Identity values provide collision-resistant
idempotency material only. The caller does not retry replay conflicts or infer
resume advice.

## 8. Privacy And Security

Debug output redacts identity material and caller bindings. Structured errors
are stable and non-leaking. The bounded outcome contains only authoritative
disposition, wake count, executor-entry count, and stop reason. No prompt,
command, credential, payload, path, approval reason, evidence body, or reusable
authority is returned.

## 9. Test Coverage

Focused tests cover:

- one all-or-nothing fill per typed identity set;
- independent, bounded, domain-separated identifier families;
- deterministic test-only byte filling;
- stable non-leaking entropy failure;
- redaction-safe identity-source Debug output;
- one eligible caller entry;
- owner-reachable cancellation with zero continuity mutation;
- backend reopen and reconstruction;
- exact two-wake behavior; and
- preservation of the existing bounded outcome.

## 10. Validation

- `cargo fmt --all --check`: passed
- focused `workflow-core` tests: 8 passed
- `cargo clippy --workspace --all-targets -- -D warnings`: passed
- `cargo test --workspace`: passed
- `npm run check:docs`: passed
- `git diff --check`: passed
- locked dependency metadata check: direct `workflow-core` requirement is
  `getrandom ^0.2.17`; no Rust 1.85 direct dependency was introduced

## 11. Remaining Limitations

- No runtime path adopts the caller yet.
- There is no run discovery, detached ownership, operator notification, or
  durable host-job model.
- Cancellation provenance and owner-loss detection remain deferred.
- The implementation is local SQLite only and makes no hosted parity claim.
- `getrandom` 0.4 was rejected during final audit because its Rust 1.85 floor
  conflicts with the repository's Rust 1.78 contract; the direct 0.2.17 line
  preserves the required entropy semantics and compatibility floor.

## 12. Recommended Next Phase

Perform focused maintainer/security review of this implementation. Only after
acceptance should Workflow OS plan one explicit internal adoption site.

## 13. Governed Implementation Record

- workflow: `dg/implement`
- run: `run-1791363703619472000-2`
- approval: `approval/run-1791363703619472000-2/implementation-approved`
- presentation: `presentation/e8ed98514fc3707e`
- approval outcome: granted by delegated maintainer through proof enforcement
- approved boundary: private caller and identity-source implementation only
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations;
  approval-presentation proof enforced with one matching record and event marker
- out-of-kernel work: repository edits, Rust and documentation validation,
  dependency metadata update, and later git and pull-request actions

## 14. Governed MSRV Blocker-Fix Record

- workflow: `dg/blocker`
- run: `run-1791364628588898000-2`
- approval: `approval/run-1791364628588898000-2/fix-approved`
- presentation: `presentation/3f83e6bf45f553cb`
- approval outcome: granted by delegated maintainer through proof enforcement
- blocker: reviewed `getrandom` 0.4 requires Rust 1.85 while Workflow OS
  declares Rust 1.78
- resolution: direct dependency and caller API changed to `getrandom` 0.2.17;
  the repository MSRV was not widened
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations;
  approval-presentation proof enforced with one matching record and event marker
