# SQLite Authorized Execution Continuity Backend Plan Report

## 1. Executive Summary

The first durable authorized-execution continuity phase is now planned. The
plan defines an explicit SQLite schema-version upgrade, five atomic continuity
operations, database-wide trusted-time quarantine, and one reusable private
conformance harness shared with the reference store.

No runtime continuity behavior was implemented in this planning phase.

## 2. Scope Completed

- Inspected the accepted atomic continuity contract and phase review.
- Inspected current SQLite schema, transaction, concurrency, migration, and
  health-check behavior.
- Defined the V1-to-V2 upgrade and compatibility posture.
- Defined candidate durable record families and relational constraints.
- Defined transaction ordering, replay, conflict, and commit-ambiguity rules.
- Defined database-wide trusted-time epoch, watermark, and quarantine rules.
- Defined the private backend-parametric conformance architecture.
- Defined shared and SQLite-specific test matrices.
- Defined implementation sequence, acceptance criteria, and strict non-goals.

## 3. Scope Explicitly Not Completed

- No Rust implementation or schema change.
- No SQLite continuity support declaration.
- No event or audit projection.
- No executor, scheduler, supervisor, or redispatch path.
- No automatic approval or evidence satisfaction.
- No provider write, PostgreSQL support, CLI/schema change, or default-backend
  change.

## 4. Key Decisions

- SQLite is the first durable continuity backend and remains local-only.
- Schema upgrade is explicit rather than automatic on open.
- All five operations use `BEGIN IMMEDIATE` and durable operation receipts.
- Exact replay is checked before trusted-time observation.
- Trusted time uses a database-wide durable epoch and watermark in addition to
  per-window watermarks.
- Clock regression or incompatible provenance quarantines the epoch and cannot
  be auto-cleared.
- Security-only quarantine writes are distinguished from successful domain
  mutation.
- SQLite support is advertised only when one named conformance suite passes
  against both reference and SQLite stores.

## 5. Security And Privacy

The plan keeps bearer capabilities private and non-serializable. Durable state
contains bounded identity, commitment, revision, enum, timestamp, and stable
reference data only. It requires fail-closed corruption and time behavior plus
non-leaking Debug, SQL, decode, contention, and replay errors.

## 6. Parallel Review Inputs

Three independent read-only analyses covered:

- SQLite schema/transaction integration and migration risks;
- restart-safe trusted-time semantics; and
- extraction of the reusable conformance harness.

Their recommendations were reconciled into the plan. No subagent edited files.

## 7. Governed Phase Record

- workflow: `dg/d`;
- run: `run-1786815195336102000-2`;
- approval: `approval/run-1786815195336102000-2/planning-approved`;
- presentation: `presentation/ec68de9809422104`;
- approval outcome: granted by delegated maintainer after complete handoff;
- phase status: completed before documentation authoring; and
- out-of-kernel work: source inspection, parallel analysis, documentation
  authoring, validation, and git work were performed by external executors.

## 8. Validation

The planning phase requires:

- `npm run check:docs`; and
- `git diff --check`.

Results:

- `npm run check:docs`: passed; and
- `git diff --check`: passed.

## 9. Remaining Limitations

- The plan does not prove a working durable backend.
- A local wall clock cannot independently prove real elapsed time after an
  arbitrary database restore; quarantine and future governed epoch recovery
  remain necessary.
- Process-level commit ambiguity testing needs a portable CI design.
- Runtime event/state projection and supervisor integration remain later
  phases.

## 10. Recommended Next Phase

Perform a focused maintainer/security review of the plan. If accepted, execute
the bounded SQLite backend and reusable-conformance implementation phase.
