# SQLite Authorized Execution Continuity Backend Plan Blocker Fix 2 Review

## 1. Executive Verdict

**Needs additional planning blocker fixes.**

The second correction resolves the previously identified categories in
principle, and the proposed composite/cyclic DDL is implementable. Durable
replay completeness, operation/domain identity, error precedence, and two
remaining relational-state constraints still block implementation.

## 2. Accepted Corrections

- Success and security-rejection operation commitments are distinguished.
- Same-window attempt, yield, wait, directive, and active-yield ownership is
  expressed through valid SQLite composite and deferred foreign keys.
- `ConsumeDirectiveRequest` owns and burns authority by value.
- Fresh-connection reconciliation is private and read-only.
- Restore claims are narrowed to local live state, with future executor
  integration gated on an external epoch anchor.
- The conformance architecture, sequencing, scope, and privacy posture remain
  appropriately conservative.

## 3. Blocker: Replay Material Is Incomplete

The rejection commitment includes an exact historical security-state delta,
but the DDL persists no canonical rejection envelope or equivalent before/after
fields. Later state cannot independently reconstruct that commitment.

Successful replay also lacks a canonical request/target envelope. Several
bounded result variants omit the wait, attempt, or window identity needed for
the plan's promised relational validation.

Required fix: persist mutually exclusive bounded success and rejection replay
envelopes containing all material needed to recompute request, result or
rejection, operation, receipt, and relational identity commitments without
consulting later mutable state.

## 4. Blocker: Reconciliation Identity Is Incomplete

The private reconciler accepts operation ID and request commitment, but
`ConfirmedAbsent` claims both operation and receipt absence. Receipt ID is
independently supplied and is not recoverable from its hash.

Required fix: reconciliation must also accept the expected receipt ID, or the
shared model must first make receipt identity deterministic from the operation.

## 5. Blocker: Operation/Attempt Ownership Is Incomplete

`continuity_attempts.consume_operation_id` is unique but does not reference a
persisted `consume_directive` operation. Missing or wrong-kind operation
ownership can evade `foreign_key_check`.

Required fix: add a deferred composite relationship to an
`(operation_id, operation_kind)` candidate key, with the attempt row constrained
to `consume_directive`, and test missing/wrong-kind corruption.

## 6. Blocker: Error Precedence Is Contradictory

The common protocol observes trusted time before verifying authoritative
identity and private capability. The disposition table says stale capability
and binding mismatch roll back, but a coincident clock regression could commit
a security rejection first.

Required fix: define one deterministic precedence. Validate bounded shape,
authoritative identity, revisions, bindings, and capability before any
security-state mutation; only then observe trusted time for an otherwise legal
new operation.

## 7. Blocker: Wait Identity Differs From Reference Semantics

SQLite currently permits one condition ID across multiple versions, while the
reference store keys waits by condition ID and rejects reuse.

Required fix: add `UNIQUE(condition_id)` or amend and review the shared
reference identity model before SQLite work.

## 8. Blocker: Trusted-Time State Can Contradict Itself

Independent posture and eligibility checks permit a quarantined epoch to remain
`live_state_eligible`.

Required fix: add a cross-column constraint and require mutation preflight to
validate both values. Add direct-corruption tests for contradictory pairs.

## 9. Independent Review And Validation

Two independent read-only reviewers assessed the corrected plan and current
Core model. Together they verified:

- canonical DDL parses under SQLite;
- deferred forward/cyclic foreign keys are supported;
- same-window composite ownership works; and
- docs and diff checks pass.

Both reviewers returned a blocking verdict.

## 10. Governed Review Record

- workflow: `dg/review`;
- run: `run-1786816583001817000-2`;
- approval: `approval/run-1786816583001817000-2/review-scope-approved`;
- presentation: `presentation/6d6c9fe7e51ec5f4`;
- approval outcome: granted under standing delegated-maintainer authority after
  the complete handoff was presented; and
- out-of-kernel work: independent review and documentation were performed by
  external executors; the kernel recorded governance only.

## 11. Recommended Next Phase

Run one final focused planning correction for these exact blockers, then repeat
security re-review. Do not implement Rust or SQLite behavior until accepted.
