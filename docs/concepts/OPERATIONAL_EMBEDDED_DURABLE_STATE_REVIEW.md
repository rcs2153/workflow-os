# Operational Embedded Durable State Review

Review date: 2026-07-29

## 1. Executive Verdict

**Phase accepted; proceed to Shared PostgreSQL State.**

The implementation closes the bounded local operational-state milestone:
filesystem records can be staged into SQLite under writer exclusion, imported
atomically, verified by canonical content and referential integrity, and
activated through an exact receipt without mutating the source or changing
runtime selection.

No blocker remains. The result is intentionally an opt-in local migration
capability, not collaborative state or a production migration system.

## 2. Scope Verification

The phase stayed within the approved operational embedded-state scope.

It added no:

- automatic migration or backend selection;
- source deletion, replacement, or cleanup;
- workflow schema or runtime configuration;
- PostgreSQL, hosted behavior, or shared-worker claims;
- backup, restore, replication, or production-readiness claim;
- SDK or example changes;
- provider writes or new mutation families;
- tenant, identity, or enterprise administration features.

## 3. Writer Exclusion Assessment

The migration acquires the filesystem exclusive migration guard before
exporting state and keeps it through the staging operation. The source
inventory is recalculated under the guard and compared with the accepted plan,
so a stale dry-run cannot silently authorize a changed source.

This is sufficient for cooperating current writers. It cannot stop an older
binary that does not honor the guard, so the explicit older-writer shutdown
assertion remains necessary and honestly documented.

## 4. Transaction And Resume Assessment

The supported record families are staged in one immediate SQLite transaction.
Injected failure before commit proves that imported tables are rolled back
rather than left partially populated.

Migration attempt metadata binds source inventory, destination identity,
adapter schema, and migration identity. Exact verified retries can resume
deterministically; changed attempts and changed source state fail closed.

## 5. Projection And Referential Integrity Assessment

Run snapshots are rebuilt from authoritative events instead of being copied as
independent truth. The import validates that run-scoped approval presentation,
adapter, SideEffect, and WorkReport records refer to exported runs.

WorkReport SideEffect citations are checked against the exported SideEffect
set. Process-local locks are excluded because transferring them would falsely
transfer ownership.

The supported-family boundary is explicit. Future durable record families must
join both the export and canonical verification contract before being claimed
as migratable.

## 6. Verification Assessment

Verification combines SQLite health, exact inventories, relational identity,
validated deserialization, referential integrity, and canonical
source/destination digest equality.

The digest compares payload content, not merely counts. The same-count tamper
regression demonstrates that modified destination data fails verification and
remains inactive.

This is a credible local verification boundary. It is not a cryptographic
notarization system, backup proof, or shared-store consistency protocol.

## 7. Activation And Recovery Assessment

Verified staging remains inactive until an exact receipt is supplied.
Activation rechecks the destination digest before marking it ready.

Activation does not:

- select the destination for runtime use;
- mutate or delete filesystem source state;
- claim rollback automation;
- authorize external writes.

On import or verification failure, the source remains readable and the
destination remains non-ready. That is the correct conservative recovery
posture for this phase.

## 8. Privacy And Error Assessment

Migration inputs, receipts, errors, and `Debug` surfaces are bounded. They do
not expose database or filesystem paths, raw payloads, SQL, raw idempotency
keys, provider data, command output, credentials, tokens, or secret-like test
values.

Stable error codes distinguish stale plan, writer posture, import,
verification, receipt, and activation failures without echoing sensitive
inputs.

## 9. CLI Assessment

The CLI requires explicit destination and migration identity rather than
inventing hidden runtime configuration. Staging and activation are separate
commands, and staging does not imply runtime selection.

This is appropriate for a maintainer-only local operational slice. General
user onboarding, automated backend selection, source retirement, and migration
progress UI remain deferred.

## 10. Test Quality Assessment

Focused tests cover the important correctness properties:

- source guard and stale-plan rejection;
- one-transaction rollback;
- verified inactive staging;
- exact receipt activation;
- source retention;
- event and idempotency behavior;
- projection rebuild;
- exact resume;
- changed-receipt rejection;
- older-writer assertion;
- content tamper detection;
- privacy and non-leakage;
- CLI staging and activation.

The workspace suite protects existing filesystem and SQLite state contracts,
runtime semantics, approvals, evidence, reports, SideEffects, adapters, and
CLI behavior.

Remaining test gaps are non-blocking for this local slice: process-kill fault
injection, backup/restore rehearsal, automated runtime selection, older-writer
process discovery, performance baselines, PostgreSQL, and shared workers.

## 11. Documentation Assessment

The roadmap, migration plans, product contract, evaluation guide, report, and
review consistently state:

- operational local filesystem-to-SQLite staging is implemented;
- activation is explicit and receipt-bound;
- source state remains retained;
- runtime backend selection is not changed;
- PostgreSQL and collaborative state remain future work;
- production durable-state readiness is not claimed.

## 12. Blockers

None.

## 13. Non-Blocking Follow-Ups

- Add process-kill and WAL recovery tests before broader durability claims.
- Define explicit source retirement and rollback only after operational usage
  proves the retained-source boundary.
- Expand all-family migration fixtures as specialized durable stores grow.
- Keep immutable run-bundle migration explicit; do not hide companion
  filesystem dependencies.
- Preserve the external-feedback priority: low-risk work should become quieter
  through deterministic proportional governance without weakening evidence.

## 14. Recommended Next Phase

Proceed to **Shared PostgreSQL State** as the next larger implementation slice.

The work should implement the accepted Core state contract against PostgreSQL,
including real shared-worker transaction and concurrency behavior. It must not
infer shared guarantees from SQLite, begin hosted product behavior, introduce
tenant administration prematurely, or broaden provider mutations.

## 15. Validation

Completed successfully:

- `cargo fmt --all --check`;
- focused migration tests and strict clippy;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check`;
- `npm run check:integrations`;
- `npm run check:docs`;
- `cargo audit`;
- `npm audit --audit-level=moderate`;
- `git diff --check`.

Governed review:

- workflow: `dg/review`;
- run ID: `run-1785332005095805000-2`;
- approval ID:
  `approval/run-1785332005095805000-2/review-scope-approved`;
- presentation ID: `presentation/505a04aa31e8ab13`;
- outcome: granted with persisted presentation proof;
- phase status: completed;
- event summary: 39 events, one approval, zero retries, zero escalations.

Code inspection, tests, documentation, and git work occurred outside the
kernel under the approved review scope. The kernel coordinated and recorded
governance; it did not execute those operations.
