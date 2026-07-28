# SQLite Embedded Durable State Adapter Report

Report date: 2026-07-28

## 1. Executive Summary

Workflow OS now has an explicit, opt-in embedded `SQLite` durable-state
backend behind the accepted Core semantic contract.

The adapter preserves the existing state-store APIs, uses a managed version-one
schema, stores validated canonical JSON envelopes with relational identity and
ordering indexes, and passes the applicable common conformance scenarios. It
is local-only and is not selected by the runtime, CLI, workflow specs, or
configuration.

This is an implementation slice, not production durable-state acceptance.
Migration, backup/restore proof, shared-worker guarantees, PostgreSQL, and
collaborative consumers remain future governed phases.

## 2. Scope Completed

- Added `rusqlite` 0.40.1 with the bundled `SQLite` feature.
- Added public `SqliteStateBackend::open` and an explicit bounded-timeout
  constructor.
- Added managed adapter schema version one and fail-closed schema readiness
  checks.
- Implemented existing aggregate `StateBackend` stores plus current specialized
  WorkReport artifact and SideEffect record stores.
- Added transactional validated single-event append using
  `BEGIN IMMEDIATE`.
- Added canonical JSON envelopes and relational indexes for identity, ordering,
  and lookup.
- Added health-time verification that relational identity/index columns still
  match their canonical record envelopes.
- Added read-time verification that authoritative lookup/index identity still
  matches canonical records, including full event identity and sequence.
- Added WAL, foreign-key, full-synchronous, and bounded busy-timeout
  configuration for every connection.
- Extended the common conformance harness with managed-schema readiness.
- Added focused reopen, multiple-connection contention, schema mismatch,
  incomplete-schema, corruption, Debug, and error non-leakage tests.

## 3. Scope Explicitly Not Completed

The phase did not add:

- automatic runtime or CLI backend selection;
- workflow specification or runtime configuration fields;
- filesystem-to-SQLite migration;
- automatic upgrade of existing state;
- ordered schema migration tooling beyond version-one initialization and
  readiness detection;
- verified backup or restore;
- PostgreSQL;
- collaborative or multi-host state;
- shared-worker leases or fencing;
- compare-and-set revisions;
- cross-record transaction families;
- provider writes, sandbox integration, or new mutation families;
- hosted behavior, examples, SDK changes, or release posture changes.

## 4. Adapter API Summary

`SqliteStateBackend::open(path)` opens or initializes one explicit database
path. `open_with_busy_timeout(path, timeout)` exists for deterministic local
contention testing and bounded operator tuning.

The adapter opens a fresh connection per operation. It does not read hidden
global state, select itself automatically, require a daemon, or expose
connection strings through `Debug`.

## 5. Schema And Durability Boundary

Schema version one contains:

- schema metadata;
- run events and snapshots;
- idempotency results and local locks;
- approvals and approval-presentation records;
- project state and policy audit;
- adapter audit and observability records;
- WorkReport artifact records;
- SideEffect records.

Canonical validated JSON remains the compatibility envelope. Selected identity,
sequence, timestamp, and run columns provide constraints and deterministic
queries without becoming a second domain model.

Every connection enables foreign keys, WAL, full synchronous durability, and a
bounded busy timeout. A newer schema fails with
`state.sqlite.schema.incompatible`. Incomplete or checksum-mismatched metadata
fails with `state.sqlite.schema.recovery_required`.

The adapter initializes only an empty version-zero database. It does not run
general migrations.

## 6. Contract And Transaction Posture

The adapter declares:

- ordered event append;
- immutable run-identity validation;
- idempotency replay;
- process-local exclusive lock behavior;
- managed schema version one in `ready` posture;
- `AppendRunEvent` as the only supported Core transaction family.

It explicitly does not claim:

- cross-record atomic commit;
- compare-and-set revision;
- expiring fenced leases;
- managed schema migration;
- verified backup/restore;
- shared-worker concurrency;
- the six remaining Core transaction families.

Successful CRUD operations do not promote those unsupported guarantees.

## 7. Error And Privacy Posture

Errors use stable `state.sqlite.*` or existing state-contract codes. Busy and
locked database conditions collapse to `state.sqlite.busy`. Corrupt payloads,
newer schemas, incomplete schemas, failed writes, and failed reads do not
include database paths, SQL text, stored payloads, provider data, credentials,
or secret-like test markers.

`Debug` reports only the backend kind and adapter schema version. It does not
include the database path or stored values.

## 8. Test Coverage

Focused tests prove:

- all applicable common conformance scenarios;
- explicit supported and unsupported contract posture;
- managed schema readiness;
- reopen with ordered durable events;
- WAL posture;
- deterministic single-winner behavior for two concurrent next-event
  attempts;
- newer-schema rejection;
- incomplete-schema recovery requirement;
- corrupt-record detection and unhealthy health posture;
- relational identity drift detection;
- authoritative read-time rejection of relational identity drift;
- path and secret-like value non-leakage.

Existing filesystem conformance remains unchanged and continues to declare its
unmanaged schema posture.

## 9. Dependency Review

The phase uses `rusqlite` 0.40.1 with only the `bundled` feature. That compiles
the maintained SQLite C library through `libsqlite3-sys` and avoids dependence
on a host-installed SQLite version.

No pooling, extension loading, backup, tracing, user-function, encryption, or
provider-specific feature was enabled. `rusqlite` is MIT-licensed; SQLite is
public domain. Dependency and vulnerability checks remain required in phase
validation.

## 10. Commands And Results

Completed successfully:

- `cargo fmt --all --check`;
- focused durable-state and SQLite adapter tests;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs` under the repository's pinned Node 20 toolchain;
- dependency vulnerability audit;
- `git diff --check`.

## 11. Governed Phase Record

- dogfood workflow: `dg/implement`;
- run ID: `run-1785231889836444000-2`;
- approval ID:
  `approval/run-1785231889836444000-2/implementation-approved`;
- approval outcome: granted with persisted presentation proof;
- phase status: completed;
- event summary: 39 events, one approval, zero retries, zero escalations.

Repository edits, dependency resolution, shell commands, tests, documentation,
and git work occurred outside the kernel under the approved phase scope. The
kernel coordinated governance and recorded the phase; it did not execute those
operations. Live provider checks, migration rehearsal, backup/restore, process
kill fault injection, PostgreSQL, and shared-worker tests were skipped because
they are outside this phase or not implemented. No result was simulated.

## 12. Remaining Known Limitations

- Version-one initialization is not a migration framework.
- No filesystem-state import exists.
- Backup and restore are not implemented or rehearsed.
- Crash behavior is covered by reopen persistence, not process-kill fault
  injection.
- Busy handling returns a bounded retry signal but the kernel does not add a
  new retry policy in this phase.
- Locks remain process-local and unfenced by contract.
- The aggregate `StateBackend` still omits some newer specialized stores.
- There is no performance baseline, network-filesystem posture check, or
  collaborative multi-worker proof.

## 13. Recommended Next Phase

Focused maintainer review is complete in
[SQLite Embedded Durable State Adapter Review](SQLITE_EMBEDDED_DURABLE_STATE_ADAPTER_REVIEW.md).
The phase was accepted after adding read-time relational identity enforcement.

Plan explicit filesystem-to-SQLite migration with dry-run, source
preservation, deterministic import, integrity verification, and explicit
destination activation. Do not make SQLite the default or begin PostgreSQL,
collaboration consumers, hosted behavior, or broader mutation families first.
