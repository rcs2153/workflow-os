# Trusted-Host Local Application Topology Plan Report

## 1. Executive Summary

The explicit local host application topology is now planned. Repository
inspection confirms that no current application can safely adopt the private
SQLite trusted-host process owner. The accepted direction is an unpublished
local-host package and a narrow non-default Core application SPI built around
opaque, one-shot, authority-bound sessions.

This phase changes documentation only. It does not implement the SPI, create
the application package, expose a command, or adopt the owner.

## 2. Scope Completed

- Assessed CLI, hosted, and Core application boundaries.
- Chose a separate unpublished local-host package as the future lifecycle
  owner.
- Defined a feature-gated unstable Core SPI and opaque session boundary.
- Defined session issuance, cancellation custody, outcome, failure, privacy,
  test, and review requirements.
- Sequenced implementation from model/visibility through one later explicit
  source and operational caller.

## 3. Scope Explicitly Not Completed

No Rust code, application crate, binary, CLI command, SDK, workflow schema,
runtime setting, discovery, scanning, polling, background work, state bridge,
hosted parity, signal handling, provider behavior, writes, OpenShell, nested
harness execution, or release posture changed.

## 4. Topology Decision

The future `workflow-local-host` package will be unpublished and local/SQLite
specific. Core will remain the authority boundary and issue one opaque session
through a non-default unstable SPI. The local host will own only synchronous
foreground execution and cooperative cancellation-handle custody.

The CLI and hosted worker are rejected as adoption sites because their state
and lifecycle contracts do not match the accepted SQLite owner.

## 5. Visibility Decision

Cross-crate Rust visibility is treated as packaging, not authority. The SPI
must therefore combine a non-default feature with opaque constructors,
one-shot non-serde values, current-state validation, and redacted outcomes.
Private locators and capabilities must not be exported for caller assembly.

## 6. Remaining Limitation

No production component currently supplies the full current opening
authorization, trusted time, executor, and exact skill input needed to issue a
session. The first implementation is consequently model/visibility only. A
separate source plan and review are required before operational adoption.

## 7. Validation

- `npm run check:docs`: passed
- `git diff --check`: passed

## 8. Recommended Next Phase

Perform a focused maintainer/security review of the topology plan. If accepted,
implement the Core SPI model and visibility slice only.

## 9. Governed Planning Record

- workflow: `dg/d`
- run: `run-1791376780678682000-2`
- approval: `approval/run-1791376780678682000-2/planning-approved`
- presentation: `presentation/5e1db2aeb5f5c287`
- presentation hash:
  `5e1db2aeb5f5c287925f2987766074e8f5557c4ef649108706453dce8a6c063b`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: planning and documentation only
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations;
  approval-presentation proof enforced with one persisted presentation record
- out-of-kernel work: repository inspection, architecture analysis,
  documentation edits, validation, and later git or pull-request work
