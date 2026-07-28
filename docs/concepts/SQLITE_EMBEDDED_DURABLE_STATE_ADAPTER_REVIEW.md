# SQLite Embedded Durable State Adapter Review

Review date: 2026-07-28

## 1. Executive Verdict

**Phase accepted after blocker fix. Proceed to explicit
filesystem-to-SQLite migration planning.**

The opt-in `SqliteStateBackend` is a bounded and credible embedded implementation
of the accepted durable-state semantic contract. It provides managed schema
readiness, validated ordered event append, local contention behavior, canonical
record envelopes, stable failures, and honest capability declarations without
becoming a runtime or CLI default.

The review found one blocker: relational identity drift was detected by backend
health checks but not by every authoritative read path. A tampered event
`run_id` index could therefore route a valid canonical payload through the
wrong lookup before a health check ran. The fix validates relational identity
against canonical records during reads, with the event stream checking event
ID, run ID, requested run, and sequence. Focused regression coverage proves the
read fails closed.

No blocker remains.

## 2. Scope Verification

The phase stayed within the approved embedded-adapter scope.

It added no:

- automatic runtime or CLI backend selection;
- workflow schema or runtime configuration;
- filesystem-state migration;
- PostgreSQL or collaborative state;
- hosted or distributed runtime;
- managed schema upgrade framework;
- backup or restore claim;
- shared-worker lease or fencing model;
- provider mutation or sandbox expansion;
- example, SDK, or release posture change.

## 3. Dependency Assessment

`rusqlite` 0.40.1 is justified for the first embedded adapter. Only the
`bundled` feature is enabled, avoiding reliance on an unknown host SQLite
version while excluding extension loading, user functions, pooling, backup,
tracing, encryption, and unrelated optional surfaces.

`rusqlite` is MIT licensed and SQLite is public domain. The dependency audit
passes. The bundled C dependency increases the maintenance and vulnerability
surface compared with the filesystem backend, but it is proportionate to the
selected embedded-state requirement.

## 4. API And Architecture Assessment

The public surface is appropriately small:

- `SqliteStateBackend::open(path)`;
- `SqliteStateBackend::open_with_busy_timeout(path, timeout)`;
- implementations of existing Core store traits;
- an explicit durable-state contract declaration.

The adapter opens one connection per operation and reads no hidden global
state. It does not require a daemon, live adapter, state backend selector, or
workflow configuration. Its custom `Debug` omits the database path and stored
records.

Canonical JSON records remain the compatibility envelope. Relational columns
provide identity, ordering, and lookup constraints rather than creating a
second domain model.

## 5. Schema And Readiness Assessment

Schema version one covers the current aggregate `StateBackend` stores plus
WorkReport artifacts and SideEffect records. Empty version-zero databases are
initialized transactionally and marked ready. Newer versions,
metadata/checksum disagreement, and incomplete initialization fail closed.

Each connection enables:

- foreign keys;
- WAL journal mode;
- full synchronous durability;
- a bounded busy timeout.

This is schema initialization and readiness detection, not managed migration.
There is no automatic upgrade path, downgrade path, import, backup, restore, or
network-filesystem acceptance.

## 6. Transaction And Concurrency Assessment

Validated event append uses `BEGIN IMMEDIATE`, reads current history, enforces
next-event invariants, and inserts the event before commit. Two concurrent
connections attempting the same next sequence produce one success and one
stable duplicate-sequence failure.

Idempotency first-write and replay are transactional. Local locks provide
single-owner contention and owner-checked release.

The contract correctly declares only `AppendRunEvent` as a supported Core
transaction family. It does not claim:

- cross-record atomic commit;
- compare-and-set revision;
- expiring fenced leases;
- managed schema migration;
- verified backup and restore;
- shared-worker concurrency.

WAL and multiple connections do not promote the adapter into a collaborative
or distributed backend.

## 7. Integrity And Corruption Assessment

Decoded records pass their existing validated deserialization boundaries.
Corrupt JSON fails with `state.sqlite.record.corrupt` without echoing payloads
or paths. Backend health runs SQLite `quick_check`, validates all typed
payloads, and verifies relational identity columns against canonical records.

The review blocker showed that health-only identity verification was
insufficient for authoritative reads. The accepted implementation now also:

- verifies event ID, run ID, requested run, and sequence during event reads;
- verifies requested snapshot, approval, approval-presentation, project,
  WorkReport artifact, and SideEffect identities;
- verifies run identity for listed approval presentations, WorkReport
  artifacts, and SideEffect records.

Health remains defense in depth for full-table identity drift, including
projection rows not selected by a particular lookup.

## 8. Privacy And Error Assessment

SQLite failures collapse to stable `state.sqlite.*` or existing state-contract
codes. Busy/locked conditions use `state.sqlite.busy`. Schema, read, write,
corruption, and identity failures do not include:

- database paths or SQL text;
- canonical payloads;
- provider data or command output;
- credentials, tokens, or secret-like test markers.

`Debug` exposes only the backend kind and adapter schema version. No persistence
or CLI surface was added that would expose raw database values.

## 9. Conformance And Test Assessment

The common conformance harness reports 10 passed and 12 explicitly unsupported
scenarios for SQLite. Focused adapter tests cover:

- managed schema readiness;
- ordered append and reopen;
- WAL posture;
- multiple-connection next-event contention;
- idempotency and lock behavior through the shared harness;
- newer and incomplete schema rejection;
- corrupt-payload non-leakage;
- health-time relational identity detection;
- read-time relational identity rejection;
- path-safe `Debug` and errors.

The full workspace suite preserves the filesystem backend, runtime, approvals,
evidence, reports, SideEffects, provider sandboxes, onboarding, and dogfood
behavior.

Remaining test gaps are correctly outside this phase: process-kill fault
injection, coordinated backup/restore, network-filesystem detection,
performance baselines, shared workers, and migration rehearsal.

## 10. Documentation Assessment

The roadmap, accepted store-selection plan, implementation report, and this
review consistently state:

- SQLite is implemented only as an explicit local adapter;
- PostgreSQL remains the selected future shared adapter;
- the Core semantic contract governs both;
- the filesystem backend remains the current preview default;
- migration, automatic selection, backup/restore, collaboration, and
  production readiness are not implemented.

## 11. Blocker Fixed

**Authoritative reads did not always verify relational index identity against
the canonical record.**

Impact: a corrupted event relational key could misroute an otherwise valid
event payload before an operator invoked backend health.

Resolution: authoritative read paths now fail closed with
`state.sqlite.record.identity_mismatch`, and focused regression coverage
exercises the event-stream case.

## 12. Remaining Blockers

None.

## 13. Non-Blocking Follow-Ups

- Replace the schema checksum marker with a generated digest if later schema
  evolution needs stronger drift detection.
- Add process-kill and WAL recovery tests before production durability claims.
- Define backup, checkpoint, restore, and unsupported-filesystem posture before
  operator adoption.
- Keep aggregate transaction families unsupported until they have explicit
  transactional APIs and executable conformance scenarios.
- Preserve the product priority emerging from external feedback: reduce
  low-risk ceremony through deterministic proportional governance while
  retaining evidence and audit posture.

## 14. Recommended Next Phase

Plan explicit filesystem-to-SQLite migration.

The plan should require dry-run inventory, source preservation, deterministic
record ordering, canonical validation, destination integrity checks,
idempotent restart behavior, failure recovery, and explicit activation. It
must not make SQLite the default, mutate source state during import, begin
PostgreSQL, claim collaborative readiness, or broaden provider writes.

## 15. Validation

Completed successfully:

- `cargo fmt --all --check`;
- focused SQLite clippy and tests after the blocker fix;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs` under the pinned Node 20 toolchain;
- dependency vulnerability audit;
- `git diff --check`.

Governed review:

- workflow: `dg/review`;
- run ID: `run-1785236538067207000-2`;
- approval ID:
  `approval/run-1785236538067207000-2/review-scope-approved`;
- presentation ID: `presentation/c163f292e3bade7c`;
- outcome: granted with persisted presentation proof;
- phase status: completed;
- event summary: 39 events, one approval, zero retries, zero escalations.

Code inspection, blocker correction, tests, documentation, and git work occurred
outside the kernel under the approved review scope. The kernel coordinated and
recorded governance; it did not execute those operations.
