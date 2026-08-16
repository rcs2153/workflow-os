# Authorized Execution Continuity Semantic V2 Blocker Fix Report

## 1. Executive Summary

The four blockers from the focused semantic V2 review are fixed in the
test-only reference semantics and public continuity contract wire boundary.
Historical expiry rejection replay now accepts legal monotonic successor
state, persisted terminal windows classify as terminal before live execution
checks, successful-operation replay verifies authoritative embedded identities
and ownership, and V1 wire compatibility is restored without weakening V2.

This remains a semantic and conformance-oracle phase. It does not implement a
durable backend, runtime scheduling, automatic approval, provider mutation,
CLI behavior, schemas, hosted execution, or nested harnesses.

## 2. Blockers Fixed

1. Historical expiry rejection replay no longer requires mutable global
   trusted-time state to remain frozen at the historical result.
2. Validated `closed`, `expired`, `revoked`, and `superseded` windows return
   `terminal` before current clock and live-execution eligibility checks.
3. Successful-operation replay now rejects map-key, embedded-record, committed
   result, cursor, subject, authority, and ownership mismatches.
4. V1 preserves valid caller operation order and accepts previously tolerated
   unknown outer and entry fields. V2 remains strict through a private V2 wire.

## 3. Implementation Approach

Historical security rejection validation recomputes the original legal
transition, then validates the current trusted-time snapshot as a legal
successor. Non-expiry security rejections remain exact because quarantine
prevents later lawful mutation. Expiry allows only monotonic revision and
watermark advancement with a valid healthy/live or quarantined posture; the
rejected window's expiry and resulting security snapshot remain exact.

Continuation classification first loads and validates the authoritative
window identity. Persisted terminal states are then classified independently
of whether the trusted clock is currently unavailable, past expiry, or the
live instance is quarantined. Non-terminal states retain the conservative
live-execution checks.

Successful-result validation compares authoritative records' embedded IDs
with their map keys and committed result IDs, then verifies cursor, subject,
authority commitment, and owning-window relationships where applicable.

## 4. Compatibility Summary

V1 source types and exhaustive version vocabulary remain unchanged. Valid V1
operation vectors are no longer reordered, and unknown outer or entry fields
remain accepted and ignored as before semantic V2. Invalid required fields and
secret-like enum values still fail with bounded errors.

V2 retains canonical operation ordering and rejects unknown outer and nested
entry fields. A private strict V2 entry wire prevents restored V1 tolerance
from widening the V2 contract.

## 5. Validation And Privacy

All new validation returns the existing stable, non-leaking corruption or
contract errors. No rejected identifier, field name, payload, credential,
source content, command output, or secret-like value is included in an error.
The fix adds no new stored payload fields or serializable authority.

## 6. Test Coverage

Focused regressions cover:

- exact replay of an expiry rejection after a valid sibling-window operation
  advances global trusted time and revision;
- every persisted terminal window state under unavailable, expired, and
  quarantined live execution posture;
- authoritative embedded window, directive, and attempt identity corruption;
- V1 caller-order preservation and unknown-field acceptance;
- V2 outer and nested unknown-field rejection; and
- the complete existing reference continuity and public contract suites.

## 7. Governed Phase Record

- workflow: `dg/blocker`
- run: `run-1786837876281993000-2`
- approval: `approval/run-1786837876281993000-2/fix-approved`
- presentation: `presentation/cca4ed70cb494ba3`
- approval outcome: granted under delegated-maintainer authority after the
  complete proof-enforced handoff was evaluated
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- out-of-kernel work: source inspection, repository edits, tests, validation,
  documentation, and command execution were performed by the external
  executor under the governed scope

The kernel did not edit files, run checks automatically, commit, push, open a
pull request, schedule an agent, or claim production backend support. No
required check was skipped.

## 8. Commands And Results

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check`: passed.
- `npm run check:integrations`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 9. Remaining Known Limitations

- No production backend advertises semantic V2 continuity support.
- No durable continuity state survives restart.
- No runtime executor opens windows or consumes resume directives.
- No supervisor redispatches an executor after a lawful yield.
- No live attempt lease proves ownership of an executing attempt.
- No approval automation or reusable delegated authority is introduced.
- SQLite implementation remains blocked pending focused review of this fix.

## 10. Recommended Next Phase

Perform a focused maintainer/security review of this blocker fix. Proceed to
the SQLite semantic V2 backend only if the review accepts historical replay,
terminal classification, identity integrity, and compatibility behavior.
