# SQLite Authorized Execution Continuity Backend Plan Blocker Fix 2 Report

## 1. Executive Summary

The second focused correction resolves the four blockers found by the first
blocker-fix re-review. The SQLite plan now binds durable success and rejection
operations, enforces composite same-window ownership, consumes ambiguity
authority by value, exposes reconciliation only through a private read-only
boundary, and separates scoped operation support from live database-instance
eligibility.

No backend implementation is authorized by this report. Focused security
re-review remains required.

## 2. Operation Integrity Fix

The canonical operations table now stores `operation_commitment` and the
exactly applicable result or rejection commitment. The plan defines
domain-separated success, rejection, common operation, and receipt bindings.
Replay and health validation must recompute them from bounded persisted fields.

## 3. Relational Integrity Fix

The DDL now uses composite candidate keys and foreign keys to prove that yield
attempts, wait generations, directive generations, and active yields belong to
the same window. The cyclic active-yield relationship is a deferred SQLite
foreign key, not an application-only check.

The unowned `scope_commitment` and `scope_generation` fields were removed.
`window_binding_commitment` is explicitly the existing commitment over
`ExpectedWindowBinding`.

## 4. Ambiguity Authority Fix

`ConsumeDirectiveRequest` must own its non-cloneable authority capability, and
the store method consumes the request by value. No success, rollback, or
commit-ambiguous path returns that capability. A private production reconciler
performs only fresh-connection, read-only operation lookup and never returns
authority. Confirmed absence requires fresh assessment and operation identity.

## 5. Restore And Support Fix

Restore safety was removed from this phase's goals. Contract V2 declares a
`local_live_state_only` support scope, while a separate private stateful read
reports live, restore-unverified, or quarantined eligibility. Both operation
support and live eligibility are required for mutation.

Managed restore remains ineligible. Arbitrary coordinated out-of-band state
replacement is explicitly outside the claimed local threat model. Future
executor integration remains prohibited until an external rollback-resistant
epoch-anchor contract is accepted.

## 6. Scope Preserved

No Rust, SQLite, executor, scheduler, event, approval, provider, Postgres, CLI,
workflow schema, nested harness, external write, or release behavior changed.

## 7. Governed Record

- workflow: `dg/blocker`;
- run: `run-1786816415805125000-2`;
- approval: `approval/run-1786816415805125000-2/fix-approved`;
- presentation: `presentation/c89cc830d3537ed0`;
- approval outcome: granted under standing delegated-maintainer authority after
  the complete handoff was presented; and
- out-of-kernel work: documentation analysis and edits were performed by the
  external executor; the kernel recorded governance only.

## 8. Validation

Required before phase close:

- `npm run check:docs`; and
- `git diff --check`.

## 9. Recommended Next Phase

Perform one focused maintainer/security re-review. Implement nothing until the
plan is accepted without blockers.
