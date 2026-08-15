# SQLite Authorized Execution Continuity Backend Plan Blocker Fix 3 Report

## 1. Executive Summary

The third focused correction addresses every blocker from the second security
re-review. The plan now makes replay self-contained, reconciliation receipt-
aware, operation/domain ownership relationally enforceable, validation
precedence deterministic, wait identity reference-equivalent, and trusted-time
posture internally consistent.

No implementation is authorized until focused re-review accepts this plan.

## 2. Canonical Replay Material

Every committed operation retains a bounded canonical request envelope.
Successful and security-rejected operations retain mutually exclusive bounded
result or rejection envelopes. Those envelopes contain the historical target,
revision, observation, and state-delta material needed to independently
recompute request, result/rejection, operation, receipt, and relational
commitments after later state changes.

## 3. Reconciliation Identity

Private read-only reconciliation now requires expected operation ID, request
commitment, and receipt ID. `ConfirmedAbsent` therefore proves absence of both
independent identities without deriving or reconstructing authority.

## 4. Relational Ownership

An attempt's consume operation now has a deferred composite foreign key to an
operation constrained to kind `consume_directive`. Missing and wrong-kind
operation ownership cannot commit. The wait table now enforces the current
reference model's globally unique condition ID.

## 5. Deterministic Error Precedence

For new operations, authoritative identity, revision, immutable binding, and
static capability validation precede trusted-time observation. A stale or
mismatched request rolls back and cannot be masked by a simultaneous clock
regression. Time-dependent authority and expiry checks occur only after the
request is otherwise legal.

## 6. Trusted-State Integrity

The singleton DDL cross-constrains quarantined time posture with quarantined
instance eligibility. Mutation preflight must validate both fields, and
corruption tests cover contradictory combinations.

## 7. Scope Preserved

No Rust, SQLite, executor, scheduler, event, approval, provider, Postgres, CLI,
workflow schema, nested harness, external write, or release behavior changed.

## 8. Governed Record

- workflow: `dg/blocker`;
- run: `run-1786816955154143000-2`;
- approval: `approval/run-1786816955154143000-2/fix-approved`;
- presentation: `presentation/a054de3ecae78fbb`;
- approval outcome: granted under standing delegated-maintainer authority after
  the complete handoff was presented; and
- out-of-kernel work: documentation analysis and edits were performed by the
  external executor; the kernel recorded governance only.

## 9. Validation

Required before phase close:

- `npm run check:docs`;
- canonical SQLite DDL parse and `foreign_key_check`; and
- `git diff --check`.

## 10. Recommended Next Phase

Perform focused maintainer/security re-review. Implement nothing unless the
plan is accepted without blockers.
