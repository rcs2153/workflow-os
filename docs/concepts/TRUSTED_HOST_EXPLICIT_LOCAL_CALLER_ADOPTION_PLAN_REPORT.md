# Trusted-Host Explicit Local Caller Adoption Plan Report

## 1. Executive Summary

Planning is complete for one explicit internal adoption site for the accepted
trusted-host local production caller. The selected boundary is an additive
crate-private operational-entry composition owned synchronously by an already
authorized local caller.

No runtime behavior was implemented.

## 2. Scope Completed

- Inspected current trusted-host operational entry, timer, caller, SQLite,
  CLI, LocalExecutor, and hosted boundaries.
- Selected one additive private operational-entry composition.
- Defined exact input ownership and a closed two-variant outcome.
- Defined routing from initial operational entry to bounded continuation only
  after Core returns `AwaitCondition`.
- Defined cancellation, stop, error, restart, concurrency, privacy, and test
  posture.
- Rejected unsafe or premature CLI, LocalExecutor, hosted, startup-scan, and
  background-loop adoption sites.

## 3. Scope Explicitly Not Completed

The phase did not add caller wiring, runtime behavior, discovery, a daemon,
threads, public APIs, CLI, SDKs, schemas, runtime config, provider mutations,
OpenShell, nested harnesses, hosted scheduling, automatic approval, or release
changes.

## 4. Selected Adoption Boundary

The plan selects one crate-private function beside trusted-host operational
entry. An explicit local owner supplies the already-authorized opening input
and cancellation receiver. Existing operational entry runs once. Only an
authoritative `AwaitCondition` disposition routes to the accepted production
caller with the same exact binding.

This is the smallest site that exercises real composition without inventing a
state bridge or broadening product behavior.

## 5. Alternatives Rejected

- LocalExecutor does not carry the exact SQLite continuity bindings.
- CLI exposure would prematurely publish private contracts and permit unsafe
  reconstruction pressure.
- Hosted adoption lacks PostgreSQL parity and durable owner semantics.
- Startup scanning and background loops are general scheduling and discovery.

## 6. Authority And Workflow Semantics

The plan gives no authority to the host result. Core reobserves durable state
before continuation. Existing reservation, replay, trusted-time, immutable
bundle, actor, executor, and skill-input bindings remain mandatory.

Cancellation and host stop never become workflow failure or completion.

## 7. Test Posture

The future implementation must directly prove routing, unsupported waits,
cancellation during wait, exact budget behavior, restart, competing owners,
binding substitution rejection, no hidden replay retry, Debug redaction, and
absence of public or automatic surfaces.

## 8. Validation

- `npm run check:docs`: passed
- `git diff --check`: passed

## 9. Remaining Limitations

- The selected site is still crate-private and synchronous.
- No product-facing local owner invokes it yet.
- There is no discovery, durable host job, shutdown integration, operator
  notification, or hosted parity.
- The workspace dependency/MSRV follow-up remains separate.

## 10. Recommended Next Phase

Perform focused maintainer/security review of the adoption plan. Do not
implement the composition until that review accepts the boundary.

## 11. Governed Planning Record

- workflow: `dg/d`
- run: `run-1791366137284272000-2`
- approval: `approval/run-1791366137284272000-2/planning-approved`
- presentation: `presentation/993a53682738bd52`
- presentation hash:
  `993a53682738bd5204700d4fba29b30e08999d9aa5e45270ad38d7b5ea7d1446`
- approval outcome: granted by delegated maintainer through proof enforcement
- approved boundary: documentation-only internal adoption planning
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval-presentation enforcement: proof enforced with one persisted record
- out-of-kernel work: source inspection, plan and report authoring, docs
  validation, and later git and pull-request actions

