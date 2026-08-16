# SQLite Authorized Execution Continuity Backend Blocker Fix Report

## 1. Executive Summary

The focused SQLite continuity review found three proof blockers: the absence
of one reusable named backend-parametric scenario matrix, commit-fault coverage
limited to register-yield, and restart tests that did not exercise a real WAL
database across separate processes. This bounded phase fixes all three without
adding runtime scheduling, host supervision, automatic approval, provider
mutation, schema exposure, or another backend.

SQLite semantic V2 acceptance remains pending focused blocker-fix review.

## 2. Blockers Fixed

1. The complete semantic matrix is now reusable and instantiated unchanged for
   both reference and SQLite stores.
2. Every operation family is tested at before-, during-, and after-commit fault
   boundaries, including consume capability-burn and read-only reconciliation.
3. SQLite restart proof now uses one physical WAL database across independent
   processes, including abrupt uncommitted termination and every durable
   attempt posture.

## 3. Shared Conformance Approach

`ContinuityConformanceBackend` is a private test boundary that supplies store
construction, same-store reopen, trusted-time control, commit-fault injection,
authoritative snapshot reads, and the five accepted operation families. Named
scenario functions operate only through that boundary. A single macro
instantiates each scenario for the reference and SQLite implementations.

The matrix covers operation replay and changed-content conflicts, global
receipt uniqueness, one-winner contention, attempt budgets, wait binding,
trusted-time regression/provenance/epoch/expiry/unavailability, exact replay
after later security state, restart and reconciliation dispositions,
capability burn, authority and governance binding changes, terminal and cursor
races, canonical wait ordering, and commit faults.

## 4. All-Five Fault Proof

Register-yield, transition-wait, consume-directive, record-attempt-outcome, and
recover-ambiguous-attempt each run through before-, during-, and after-commit
fault scenarios. Before/during failures prove rollback. After-commit
acknowledgement loss proves a bounded ambiguous error plus authoritative
read-only reconciliation to `DurablyCommitted`.

Consume proof additionally establishes that authority is reusable after a
confirmed rollback and cannot be reconstructed or reused after a durably
committed ambiguous return.

## 5. Subprocess, WAL, And Restart Proof

The SQLite subprocess harness opens one real database path in WAL mode. An
independent child starts an immediate transaction, mutates continuity state,
and exits abruptly before commit; the parent and another process reopen the
same database and prove rollback. A separate child commits an operation and an
independent verifier proves exact state and replay after process loss.

Additional subprocess cases reopen the same path and verify `started`,
`yielded`, `succeeded`, `retryable_failure`, `terminal_failure`, and
`ambiguous_may_have_started` attempt postures without relying on process-local
memory.

## 6. Privacy And Non-Leakage

The fix adds only test adapters, bounded fixtures, stable identifiers, and
subprocess mode selectors. Bearer capability material remains private and is
not serialized. Errors and assertions do not expose database paths, SQL,
payloads, source contents, command output, provider responses, credentials, or
secret-like values.

## 7. Tests And Validation

Focused validation passed:

- shared conformance matrix: 40 tests passed;
- SQLite subprocess proof: 3 tests passed;
- `workflow-core` library suite: 311 tests passed;
- authorized-execution continuity public contract: 7 tests passed;
- SQLite backend contract: 14 tests passed; and
- focused `workflow-core` Clippy with warnings denied: passed.

Full validation also passed:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check`;
- `npm run check:integrations`;
- `npm run check:docs`; and
- `git diff --check`.

## 8. Governed Phase Record

- workflow: `dg/blocker`;
- run: `run-1786852453157834000-2`;
- approval: `approval/run-1786852453157834000-2/fix-approved`;
- presentation: `presentation/07eaaed4951b3ace`;
- approval outcome: granted under delegated-maintainer authority after the
  complete proof-enforced handoff was presented;
- kernel run status: completed with 39 events, 1 approval, 0 retries, and 0
  escalations; approval-presentation enforcement was proof-enforced with one
  persisted presentation record and event marker; and
- out-of-kernel work: source inspection, repository edits, tests,
  documentation, and command execution were performed by the external
  executor under the governed scope. The kernel did not edit files, run
  commands, schedule an agent, or mutate a provider.

## 9. Remaining Known Limitations

- Focused blocker-fix review has not yet accepted the new evidence.
- Arbitrary database, filesystem, or VM rollback remains unsupported without
  a separately governed rollback-resistant external epoch anchor.
- No runtime executor or trusted host supervisor consumes this backend yet.
- Exactly-once continuity mutation does not imply exactly-once external work.
- Runtime event/state projection, automatic redispatch, provider mutation
  broadening, nested harnesses, CLI/schema exposure, and hosted execution
  remain out of scope.

## 10. Recommended Next Phase

Perform a focused maintainer/security blocker-fix review. If accepted, proceed
to atomic authorized-execution continuity event/state projection before the
first injected trusted-host supervisor vertical slice.
