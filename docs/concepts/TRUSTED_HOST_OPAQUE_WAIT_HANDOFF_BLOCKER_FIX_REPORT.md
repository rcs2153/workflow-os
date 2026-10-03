# Trusted Host Opaque Wait Handoff Blocker Fix Report

## 1. Executive Summary

The two blockers from the focused trusted-host opaque wait handoff review are
fixed. Observation now opens only an existing SQLite database through a
read-only connection, and the focused security suite now proves the accepted
identity-movement, non-handoff, corruption, and missing-storage boundaries.

The handoff remains private, inert, and non-authorizing. This phase does not
implement reinvocation or any scheduler behavior.

## 2. Blockers Fixed

### Existing Read-Only Observation

The handoff observer no longer calls the create-capable SQLite connection
helper. It uses a dedicated existing read-only connection that configures only
the bounded busy timeout. It does not initialize schema or change journal and
durability pragmas.

A regression test removes the database and related WAL/SHM files, attempts an
observation, and proves the operation fails with a stable error while every
file remains absent.

### Complete Focused Security Matrix

Focused tests now prove:

- unchanged coherent state derives deterministic identity;
- restart over unchanged state preserves identity;
- revision, cursor, generation, and condition movement each changes identity;
- closed, revoked, superseded, and validly expired windows return terminal
  posture without a handoff;
- satisfied waits return `ResumeNow` without a handoff;
- canceled, expired, and superseded waits return blocked posture without a
  handoff;
- unsupported dependencies and locator mismatches fail closed;
- relational projection corruption fails closed without a handoff or leaked
  marker;
- observation mutates no continuity state; and
- a concurrent transition yields a coherent old or new result, never a mixed
  handoff.

## 3. Implementation Approach

`SqliteStateBackend` now has a private existing read-only connection helper.
The handoff observer uses it for the existing deferred read transaction. No
public storage API or reusable capability was added.

Test-only derivation exposes the same internal classifier and constructor to
focused conformance tests. It does not change production visibility or add a
serialization surface.

## 4. Security And Authority Boundary

- Missing storage is not recreated by observation.
- Handoff identity responds to every accepted authoritative correlation
  dimension.
- Non-actionable or terminal state cannot produce an actionable handoff.
- Corrupt projections fail before handoff construction.
- No mutating method accepts a handoff or handoff ID.
- `RequestFreshClassification` remains the only next-operation vocabulary.

## 5. Privacy And Redaction

The new connection error remains stable and path-free. Missing-storage and
corrupt-projection tests assert that private filesystem and workflow markers
do not appear in Debug or Display output. No raw deadline, cursor, dependency,
authority, provider, command, payload, or secret value is exposed.

## 6. Scope Explicitly Not Added

This fix does not add wait satisfaction, polling, timers, queues, scheduling,
automatic reinvocation, executor calls, public APIs, serde, schemas, CLI, SDK,
providers, sandboxes, nested harnesses, writes, hosted behavior, or release
changes.

## 7. Test Coverage

The focused `trusted_host_wait_` suite contains 12 passing tests. Existing
continuity and workspace tests remain part of required validation.

## 8. Commands Run And Results

- `cargo test -p workflow-core trusted_host_wait_`: 12 passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 9. Remaining Limitations

- The handoff remains crate-private and has no production consumer.
- Only `TimeWindow` posture is represented.
- No scheduler, trusted-host loop, or explicit reinvocation path exists.
- The handoff has no public or transport representation.

## 10. Recommended Next Phase

Perform focused maintainer/security review of this blocker fix. Only after an
accepting verdict should Workflow OS plan explicit reinvocation from fresh
authoritative classification.

## 11. Governed Phase Record

- workflow: `dg/blocker`
- run: `run-1791065057026480000-2`
- approval: `approval/run-1791065057026480000-2/fix-approved`
- presentation: `presentation/7ad79e04eaf7eee5`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- validation summary: focused tests, formatting, workspace clippy, workspace
  tests, documentation checks, and diff checks passed
- approved boundary: existing read-only observation and focused security tests
- out-of-kernel work: source inspection, Rust edits, tests, documentation,
  validation, and later git and pull-request work
- missing coverage: the kernel coordinated governance only; it did not edit
  files, execute checks, create a WorkReport artifact, or perform git actions
