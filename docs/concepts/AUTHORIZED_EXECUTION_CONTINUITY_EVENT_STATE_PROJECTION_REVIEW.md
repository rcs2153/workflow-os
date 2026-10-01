# Authorized Execution Continuity Event And State Projection Review

## 1. Executive Verdict

**Needs blocker fixes.**

The implementation establishes a credible SQLite V3 atomic projection path:
accepted continuity mutations, receipts, bounded runtime events, derived
snapshots, and relational bindings share one immediate transaction. Exact
replay also avoids duplicate events. However, the accepted plan requires
fresh-connection reconciliation of the complete projected result and
backend-parametric projected-operation conformance. Those guarantees are not
yet implemented or proved.

No trusted-host supervisor integration should begin until the blockers below
are fixed and re-reviewed.

## 2. Scope Verification

The phase stayed within the approved atomic projection boundary. It did not
add a supervisor, scheduler, executor redispatch, automatic approval, command
execution, provider mutation, filesystem or PostgreSQL projection support,
CLI behavior, public workflow schema, hosted runtime, nested harness
execution, or Reasoning Lineage.

The changes are limited to continuity projection vocabulary, runtime event and
snapshot behavior, SQLite schema and transactions, tests, the roadmap, and
phase documentation.

## 3. Atomicity Assessment

The projected SQLite path uses one `BEGIN IMMEDIATE` transaction and reuses
the existing continuity semantics. Within that transaction it persists the
continuity operation and receipt, appends the bounded projection event,
derives the snapshot from event history, and stores the relational projection
binding. Validation or persistence failures before commit roll back the
transaction.

Generic SQLite event append now derives and persists the corresponding
snapshot in the same transaction. Independent snapshot writes reject stale,
skipped, and event-inconsistent cursors. These are material improvements over
caller-authored snapshot authority.

## 4. Event And Snapshot Assessment

`AuthorizedExecutionContinuityProjected` is closed, versioned, bounded, and
status-preserving. The payload carries operation kind, disposition, stable
result or rejection class, receipt and operation identity, exact cursors,
bounded target identity and revision, and a projection commitment. It does
not carry bearer capability material.

The runtime transition matrix correctly prevents an applied continuity
operation from bypassing `WaitingForApproval`; only a committed security
rejection may be disclosed there. Created, validated, and terminal histories
reject the projection event. `last_continuity_projection` remains an
inspection cache rather than current authority.

## 5. Replay Assessment

Exact projected replay reloads the durable operation and projection binding,
returns the original bounded result, and appends no event. Conflicting replay
fails before mutation. A relational-versus-JSON binding mismatch fails closed
as SQLite record corruption.

The replay path is stronger than the separate reconciliation path. The two
must not be described as equivalent until reconciliation returns and verifies
the same complete projected result.

## 6. Reconciliation Blocker

The accepted plan requires ambiguous-acknowledgement reconciliation on a fresh
connection to verify and return the original continuity result **and** its
projection binding. The current `AuthorizedExecutionContinuityReconciler`
returns only `CommittedOperationDisposition`. Its SQLite implementation loads
continuity state and validates the operation record, but it does not load or
verify `continuity_projection_bindings`, the bound runtime event, the
historical result cursor, or the derived snapshot commitment.

Consequently, a committed operation with a missing or corrupted projection
binding can still be reported as `DurablyCommitted`. That is unsafe for a
future supervisor because it cannot distinguish complete atomic projection
from partial or conflicting durable presence.

Required fix:

- add a projected reconciliation result that returns the original bounded
  disposition and validated projection binding;
- query it through a fresh connection after ambiguous acknowledgement;
- classify complete absence as retryable absence;
- classify partial or conflicting operation/event/snapshot/binding presence as
  `state.continuity_projection.corrupt`; and
- prove lawful later event successors by validating the bound historical
  prefix and deterministically derived snapshot-at-result cursor.

## 7. Fault And Conformance Blocker

The reference-store fault matrix covers all five continuity operation
families, but it exercises standalone continuity semantics rather than the
event/snapshot/binding projection contract. The SQLite projected path proves
all five successful operation families, while SQLite fault injection and
reconciliation cover only a narrower register-yield case.

The accepted plan requires one backend-parametric **projected-operation**
conformance suite plus before-, during-, and after-commit fault posture for all
five families. It also requires a generic event append racing a projected
operation and fresh-connection success, absence, conflict, and partial-
corruption reconciliation. Those proofs are not present as a reusable matrix.

Required fix:

- extract a projected-operation conformance harness used by an in-memory
  reference adapter and SQLite;
- run all five families through exact replay and before/during/after fault
  cases;
- model during-commit uncertainty as ambiguous rather than as a known
  pre-commit write failure;
- add generic-event-versus-projected-operation contention; and
- add unsupported filesystem and PostgreSQL zero-write assertions.

## 8. Schema And Migration Assessment

SQLite schema V3 adds the expected event, cursor, window identity, and
operation-binding constraints. V2 continuity history is rejected rather than
receiving fabricated historical projection events. V1-to-V2 and V2-to-V3
upgrades are explicit and checksummed, and newer or drifted schemas fail
closed.

The migration posture is acceptable for this phase, subject to the projection
reconciliation blocker. A future supervisor must not infer complete
projection solely from successful schema migration.

## 9. Backend Support Assessment

SQLite is the only backend that implements the private projection store.
Filesystem and PostgreSQL return the stable unsupported capability error. This
is the correct compatibility posture and does not overclaim generic event or
snapshot APIs as atomic continuity projection support.

Focused zero-write assertions for the unsupported projected methods remain
part of the blocker-fix test matrix.

## 10. Privacy And Redaction Assessment

The public payloads, private bindings, commitments, Debug output, serde
failures, and SQLite mapping errors remain bounded and non-leaking. They do not
store prompts, transcripts, source or spec contents, command output, provider
payloads, environment values, paths, credentials, tokens, or bearer
authority.

The newly required reconciliation result must preserve this posture. In
particular, partial-presence and conflicting-presence errors must not echo raw
stored JSON, identifiers rejected as secret-like, or database values.

## 11. Test Quality Assessment

Existing tests provide strong coverage of:

- all five successful SQLite projected operation families;
- exact replay without duplicate events;
- runtime transition and serde validation;
- relational binding drift;
- atomic generic event and snapshot persistence;
- stale snapshot rejection;
- concurrency in the continuity layer;
- V1-to-V2-to-V3 migration and refusal to fabricate projections; and
- broad workspace regression behavior.

Missing blocker-level coverage is:

- complete projected reconciliation returning the binding;
- partial binding absence and conflicting durable presence on a fresh
  connection;
- all-five-family projected fault injection;
- backend-parametric projection parity;
- generic append racing a projected operation; and
- explicit unsupported-backend zero-write behavior.

## 12. Documentation Assessment

The plan and roadmap preserve the product boundary and correctly defer the
supervisor. The implementation report accurately describes successful exact
replay, but its test summary should not be read as satisfying the plan's full
fault and fresh-connection reconciliation matrix. This review records the
remaining distinction rather than erasing the implementation work.

## 13. Blockers

1. Fresh-connection reconciliation does not verify or return the durable
   projection binding and can accept partial projection presence.
2. The required backend-parametric projected-operation conformance and
   all-five-family projected fault matrix are missing.
3. During-commit fault posture is currently modeled as a known pre-commit
   write failure instead of an ambiguous outcome requiring reconciliation.

## 14. Non-Blocking Follow-Ups

- Keep SQLite opt-in and non-production-certified.
- Preserve the non-authoritative snapshot cache boundary.
- Consider persisted historical snapshot checkpoints only if deterministic
  event-prefix derivation proves insufficient; do not add them by default.
- Continue to keep filesystem and PostgreSQL projection unsupported until each
  backend independently earns the capability.

## 15. Validation

- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed;
- `npm run check`: passed;
- `npm run check:integrations`: passed;
- `npm run check:docs`: passed;
- `git diff --check`: passed; and
- focused maintainer/security review: blocker findings recorded above.

## 16. Governed Review Record

- workflow: `dg/review`;
- run: `run-1786888322131610000-2`;
- approval:
  `approval/run-1786888322131610000-2/review-scope-approved`;
- presentation: `presentation/aa5f2605ab8fc737`;
- presentation hash:
  `aa5f2605ab8fc737f0cb2e10816825cc61bbc2b6c95b46aede6a2d6254ef47ad`;
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted handoff was presented;
- governed run status: completed; and
- out-of-kernel work: the external executor inspected source and tests,
  authored this review, and ran validation. The kernel did not edit files,
  execute checks, commit, push, merge, schedule an executor, or mutate a
  provider.

## 17. Recommended Next Phase

Implement an **authorized execution continuity event/state projection blocker
fix** limited to complete projected reconciliation and reusable projected
conformance proof.

Do not begin the local trusted-host supervisor vertical slice until that fix
passes focused maintainer/security re-review. The supervisor must consume a
projection guarantee that survives ambiguous acknowledgement and fresh-process
recovery, not only a successful continuity mutation.
