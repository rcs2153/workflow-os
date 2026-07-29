# Operational Embedded Durable State Report

Report date: 2026-07-29

## 1. Executive Summary

Workflow OS now has an explicit, local-only path for staging filesystem state
into the embedded SQLite backend, verifying the staged destination, and
activating that destination through an exact verification receipt.

The migration holds the filesystem migration guard while exporting the source,
rejects a stale plan, imports all supported record families in one SQLite
transaction, rebuilds run projections from authoritative events, and compares
canonical source and destination digests before issuing a payload-free receipt.
Activation marks only the verified destination as ready. It does not select the
destination for the runtime, alter or delete the source, or claim shared-state
or production readiness.

## 2. Scope Completed

- Added a guarded, typed, read-only filesystem migration export.
- Added explicit migration input and verification receipt models.
- Added resumable SQLite staging states and migration-attempt metadata.
- Imported supported record families in one immediate SQLite transaction.
- Rebuilt run snapshots from authoritative event history.
- Added pre-import referential-integrity validation.
- Added post-import SQLite health, inventory, and canonical digest checks.
- Added exact-receipt destination activation.
- Added bounded `state migrate-sqlite` and `state activate-sqlite` CLI paths.
- Added focused migration, rollback, tamper, resume, privacy, and CLI tests.

## 3. Scope Explicitly Not Completed

The phase did not add:

- automatic migration or automatic backend selection;
- source mutation, deletion, retirement, or cleanup;
- runtime configuration or workflow specification fields;
- PostgreSQL, shared state, hosted operation, or multi-worker coordination;
- backup, restore, replication, or disaster-recovery claims;
- schema, SDK, example, or release-posture changes;
- provider writes, new mutation families, or broader SideEffect execution;
- tenant isolation, enterprise identity, or production-readiness claims.

## 4. API And CLI Summary

`FilesystemToSqliteMigrationInput` binds the migration attempt to:

- an explicit migration ID;
- an explicit destination ID;
- the accepted dry-run plan;
- an operator assertion about older writers;
- a bounded verifier actor.

`SqliteStateBackend::stage_filesystem_migration` returns a
`StateMigrationVerificationReceipt` only after import and verification.
`SqliteStateBackend::activate_verified_migration` accepts that exact receipt
and marks the same unchanged destination ready.

The CLI exposes:

- `workflow-os state migrate-sqlite`;
- `workflow-os state activate-sqlite`.

Both commands require an explicit destination. Their output contains bounded
identifiers and verification posture, not source paths, stored payloads, raw
idempotency keys, SQL, or secrets.

## 5. Import Transaction And Projection Posture

The filesystem migration guard is acquired before export and retained through
staging. The source is re-inventoried under that guard and must still match the
accepted plan.

The destination starts in `importing_empty`. Supported records are written in
one immediate SQLite transaction. Injected pre-commit failure proves that the
transaction leaves no partial imported state.

Run snapshots are not copied as independent authority. They are rebuilt from
the imported authoritative event streams. Process-local lock records are
excluded because they are not durable transferable ownership.

## 6. Verification And Activation Posture

After commit, the destination enters `imported_unverified`. Verification
requires:

- SQLite `quick_check`;
- exact supported-family counts;
- zero migrated process-local locks;
- canonical source and destination digest equality;
- valid record deserialization and relational identity;
- referential integrity for run-scoped records;
- WorkReport-to-SideEffect citation integrity.

The canonical digest compares stable family framing and canonical serialized
records. It detects same-count payload tampering rather than relying only on
inventory counts.

Successful verification produces a payload-free receipt and leaves the
destination `verified_inactive`. Activation requires the exact receipt and
rechecks that the destination digest is unchanged before marking it `ready`.
Activation does not configure the runtime to use the destination.

## 7. Resume, Failure, And Recovery Behavior

An exact retry of the same migration attempt may resume verified staging and
return the same bounded receipt. Changed source inventory, changed plan,
changed migration identity, receipt mismatch, unsupported writer posture,
transaction failure, verification failure, and destination tampering fail
closed.

Failed verification leaves the destination inactive for diagnosis. The
filesystem source remains authoritative, retained, and readable throughout
the phase. No recovery path rewrites source state.

## 8. Privacy And Security Posture

The migration stores canonical existing records; it does not introduce a new
raw-payload channel. Migration errors and `Debug` output omit:

- filesystem and database paths;
- raw record payloads and command output;
- provider data;
- raw idempotency keys;
- credentials, tokens, and secret-like test values.

Receipts contain bounded identifiers, schema versions, counts, and digests.
They are verification records, not proof of runtime selection or source
retirement.

## 9. Test Coverage

Focused coverage proves:

- guarded staging remains inactive before activation;
- exact receipt activation;
- source retention and continued readability;
- event preservation and projection rebuild;
- idempotency replay after migration;
- exact verified resume;
- stale-source and mismatched-receipt rejection;
- required older-writer assertion;
- transaction rollback under injected failure;
- same-count payload tamper detection;
- path-safe and secret-safe errors and `Debug`;
- bounded CLI stage and activation behavior.

Existing filesystem, SQLite adapter, runtime, approval, evidence, WorkReport,
SideEffect, provider sandbox, and CLI tests remain in the workspace suite.

## 10. Commands And Results

Completed successfully:

- `cargo fmt --all --check`;
- focused migration library, runtime, and CLI tests;
- focused strict clippy for `workflow-core` and `workflow-cli`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check`;
- `npm run check:integrations`;
- `npm run check:docs`;
- `cargo audit`;
- `npm audit --audit-level=moderate`;
- `git diff --check`.

## 11. Governed Phase Record

- dogfood workflow: `dg/implement`;
- run ID: `run-1785325995656027000-2`;
- approval ID:
  `approval/run-1785325995656027000-2/implementation-approved`;
- presentation ID: `presentation/e10be370165dc5d2`;
- approval outcome: granted with persisted presentation proof;
- phase status: completed;
- event summary: 39 events, one approval, zero retries, zero escalations.

Repository edits, shell commands, tests, documentation, and git work occurred
outside the kernel under the approved phase scope. The kernel coordinated and
recorded governance; it did not execute those operations. PostgreSQL,
shared-worker tests, source retirement, backup/restore, and live provider work
were skipped because they remain outside this phase.

## 12. Remaining Known Limitations

- The writer guard coordinates only cooperating current writers.
- Shutdown of older writers remains an explicit operator assertion.
- Activation marks a destination ready but does not select it for runtime use.
- Source cleanup and automated rollback commands are not implemented.
- Immutable run bundles remain in their existing filesystem companion store.
- There is no backup/restore or process-kill migration rehearsal.
- There is no PostgreSQL or shared-worker concurrency proof.
- Richer all-family migration fixtures remain useful as record families grow.

## 13. Recommended Next Phase

The operational embedded durable-state maintainer review accepts the phase.
Begin the larger shared PostgreSQL state slice rather than adding another
filesystem-to-SQLite planning layer.

The next phase must preserve the accepted Core state semantics and must not
silently promote local SQLite behavior into shared-worker guarantees.
