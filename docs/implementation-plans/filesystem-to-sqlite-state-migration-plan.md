# Filesystem-To-SQLite State Migration Plan

Status: Accepted after focused maintainer review in
[Filesystem-To-SQLite State Migration Plan Review](../concepts/FILESYSTEM_TO_SQLITE_STATE_MIGRATION_PLAN_REVIEW.md).
The first read-only inventory and compatibility model is implemented and
documented in
[Filesystem-To-SQLite State Migration Inventory Report](../concepts/FILESYSTEM_TO_SQLITE_STATE_MIGRATION_INVENTORY_REPORT.md).
It is accepted with non-blocking follow-ups in
[Filesystem-To-SQLite State Migration Inventory Review](../concepts/FILESYSTEM_TO_SQLITE_STATE_MIGRATION_INVENTORY_REVIEW.md).
The migration plan and unreachable SQLite staging-destination core model is
implemented and documented in
[Filesystem-To-SQLite State Migration Plan Model Report](../concepts/FILESYSTEM_TO_SQLITE_STATE_MIGRATION_PLAN_MODEL_REPORT.md).
No importer, destination creation or write, migration command, backend
selector, activation path, verification receipt, or automatic state conversion
is implemented.

Related decisions and accepted foundations:

- [ADR 0012: Compatible SQLite And PostgreSQL Durable State Adapters](../adr/0012-compatible-sqlite-postgresql-durable-state-adapters.md)
- [Open-Source Durable Store Selection Plan](open-source-durable-store-selection-plan.md)
- [Durable State Semantic Contract Report](../concepts/DURABLE_STATE_SEMANTIC_CONTRACT_REPORT.md)
- [SQLite Embedded Durable State Adapter Report](../concepts/SQLITE_EMBEDDED_DURABLE_STATE_ADAPTER_REPORT.md)
- [SQLite Embedded Durable State Adapter Review](../concepts/SQLITE_EMBEDDED_DURABLE_STATE_ADAPTER_REVIEW.md)

## 1. Executive Summary

Workflow OS now has an accepted opt-in SQLite embedded-state adapter, while the
local filesystem backend remains the preview default. Existing local state must
not be moved merely because a new adapter exists.

This plan defines an explicit, fail-closed filesystem-to-SQLite migration
boundary. Migration must begin with read-only inventory and source health,
preserve the source, import canonical records in deterministic dependency
order, rebuild projections rather than copy indexes, verify the destination,
and require a separate explicit activation decision.

The first implementation phase should add only a typed, read-only migration
inventory and compatibility assessment. It must not write SQLite, alter the
filesystem source, select a backend, or expose a migration CLI command.

## 2. Goals

- Make every source record family and unsupported companion store visible
  before migration.
- Refuse migration from unhealthy, changing, ambiguous, or unsupported state.
- Preserve canonical validated records without copying filesystem path
  conventions into the database contract.
- Make deterministic import order and referential dependencies explicit.
- Keep event history authoritative and rebuild derivable projections.
- Support safe restart after interruption without accepting partial state.
- Verify record counts, canonical digests, run rehydration, ordering, identity,
  and referential integrity before activation.
- Preserve the original filesystem state until an operator separately chooses
  archival or deletion in a later phase.
- Produce bounded, redaction-safe migration plans and receipts.

## 3. Non-Goals

This plan does not authorize:

- migration implementation in the planning phase;
- automatic migration during upgrade, startup, validation, or execution;
- automatic SQLite selection or a new default backend;
- mutation, deletion, repair, or archival of filesystem source state;
- import into a non-empty or already-active SQLite database;
- migration of live locks or transfer of lock ownership;
- SQLite backup, restore, or disaster-recovery claims;
- PostgreSQL, shared workers, distributed leases, or collaborative state;
- schema, SDK, example, or workflow-spec changes;
- hosted behavior, tenant administration, or enterprise identity;
- provider calls, provider writes, or broader adapter mutation;
- migration of raw logs, source contents, credentials, or secret stores;
- release-posture changes.

## 4. Current State Boundary

The filesystem backend currently stores these Core record families:

- ordered workflow events and event-ID indexes;
- workflow snapshots;
- idempotency results;
- local lock records;
- pending approval projections;
- approval-presentation records and presentation-ID indexes;
- project-state records;
- policy audit records;
- adapter audit and observability records;
- WorkReport artifact records;
- SideEffect records and SideEffect-ID indexes.

SQLite schema version one stores equivalent canonical record families except
that it must not be treated as a replacement for every file under the state
root.

`LocalImmutableRunBundleStore` uses the companion
`immutable-run-bundles/` directory under the CLI state root. It is not part of
`StateBackend`, and SQLite schema version one does not store immutable run
bundles. The first migration must preserve that companion directory and report
it as retained filesystem state. Activation must fail if the runtime would lose
access to required bundle records.

The aggregate `StateBackend` also omits some newer specialized store traits.
Migration cannot assume that implementing `StateBackend` proves complete state
coverage.

## 5. Source-Of-Truth Classification

Every discovered source file must be classified before import.

### Authoritative Records

- workflow run events;
- idempotency results;
- approval-presentation records;
- policy audit records;
- adapter audit and observability records;
- WorkReport artifacts where the artifact contract treats the record as
  durable truth;
- SideEffect records;
- project metadata where no stronger source exists.

### Rebuildable Projections

- workflow snapshots, rebuilt by rehydrating ordered events;
- pending approval projections, rebuilt or checked against event history;
- event-ID indexes;
- approval-presentation ID indexes;
- SideEffect ID indexes;
- relational ordering and lookup columns in SQLite.

Indexes must never be copied as canonical payloads. They are checked against
their authoritative records during inventory and recreated by destination
write paths.

### Ephemeral State

- local lock files.

Locks must not migrate. A migration requires a quiescent source and acquires a
dedicated migration exclusion boundary in the implementation phase. A
destination starts without inherited locks.

### Companion State

- immutable run bundles;
- any future directory or record family not recognized by the migration
  version.

Recognized companion state is retained and reported. Unknown state is a
blocking compatibility finding, not something to ignore or copy.

## 6. Candidate Migration Model

The first model set should remain small and domain-neutral:

- `StateMigrationId`;
- `StateMigrationPlanVersion`;
- `StateMigrationSource`;
- `StateMigrationDestination`;
- `StateMigrationInventory`;
- `StateMigrationRecordFamily`;
- `StateMigrationRecordCount`;
- `StateMigrationCompatibilityFinding`;
- `StateMigrationDisposition`;
- `StateMigrationPlan`;
- `StateMigrationVerification`;
- `StateMigrationReceipt`;
- `StateMigrationActivationDecision`.

Only the inventory, compatibility, disposition, and plan types are justified
for the first read-only implementation. Receipt and activation types should be
added only when an importer and activation boundary exist.

The model must distinguish:

- canonical import;
- projection rebuild;
- ephemeral exclusion;
- companion preservation;
- unsupported blocker.

It must never contain raw record payloads, source paths in `Debug`, credentials,
or unbounded diagnostic text.

## 7. Read-Only Inventory Boundary

The current public store traits do not provide complete enumeration:

- idempotency, approvals, projects, and locks have no list API;
- adapter telemetry lists require known run IDs;
- WorkReport and SideEffect lists require known run IDs;
- filesystem index files need independent consistency inspection;
- immutable run bundles use another store.

The first implementation should therefore add a migration-specific read-only
inventory/export boundary owned by Core. It should be implemented for
`LocalStateBackend` without widening ordinary runtime store traits merely for
migration.

The inventory must:

1. open the source through `LocalStateBackend::for_inspection` so inspection
   creates nothing;
2. run the existing local state inspection;
3. enumerate only known versioned directories and JSON files;
4. reject symlinks, unexpected file types, unknown non-empty directories,
   malformed names, duplicate identities, dangling indexes, and corrupt
   records;
5. deserialize every canonical record through its validated type boundary;
6. calculate deterministic bounded counts and canonical SHA-256 digests;
7. classify projections, locks, companion stores, and blockers;
8. report whether the source is empty, healthy, quiescence-required, and
   migration-compatible;
9. avoid reading arbitrary repository files or following paths outside the
   state root.

Dry-run must be the only initial user-visible posture. Inventory must not
create a destination database.

## 8. Quiescence And Authority

Migration cannot safely infer that no writer exists merely because lock files
are absent. The implementation plan must define a source quiescence protocol.

Minimum requirements:

- the caller supplies explicit migration authority;
- the source root is inspected before and after export;
- a versioned source fingerprint is stable across both inspections;
- ordinary executor writers are excluded by a dedicated migration lock or an
  equivalent reviewed boundary;
- no active run is silently reclassified or completed;
- a stale local lock is a blocking recovery condition, not silently discarded;
- no lock record is imported.

The first read-only inventory phase may report `quiescence_required` without
implementing exclusion.

## 9. Deterministic Import Order

The future importer should use the destination's validated write APIs and this
dependency order:

1. verify an empty destination with compatible adapter schema;
2. import events grouped by run and ordered by sequence;
3. rehydrate each run and rebuild snapshots from imported event truth;
4. rebuild pending approval projections from event-backed run state;
5. import approval-presentation records after their run and approval identities
   are known;
6. import idempotency results;
7. import project metadata;
8. import policy audit records in stable timestamp/identity order;
9. import adapter audit and observability records in stable
   run/timestamp/identity order;
10. import SideEffect records in stable run/identity order while preserving
    the final validated lifecycle record;
11. import WorkReport artifacts after cited run and SideEffect identities can
    be checked;
12. run destination health and cross-record verification.

Filesystem indexes are rebuilt implicitly. Locks are excluded. Immutable run
bundles remain in their companion store until a separately reviewed durable
bundle adapter exists.

The importer must not bypass duplicate, transition, or referential checks by
writing SQL directly unless a separately reviewed migration transaction API
preserves equivalent validation.

## 10. Destination Preparation

Migration must target a new, explicitly named SQLite file. Existing non-empty
databases are rejected.

Current `SqliteStateBackend::open` initializes schema metadata as `ready`.
That behavior is correct for an empty opt-in adapter but is insufficient to
represent an in-progress import. The implementation phase must choose one
reviewed approach:

- a migration-only constructor that creates schema metadata in `importing`
  state and changes it to `ready` only after verification; or
- a staging database that remains unreachable by runtime selection and is
  atomically promoted only after verification.

The plan recommends a staging database plus an internal migration marker. No
runtime may open the staging destination as authoritative state.

## 11. Restart And Idempotency

An interrupted migration must never make partial destination state active.

Required behavior:

- migration identity binds source fingerprint, destination identity, plan
  version, and adapter schema version;
- exact replay of an interrupted migration may resume or restart
  deterministically;
- changed source fingerprint rejects resume and requires a new plan;
- a different source for the same migration ID fails closed;
- duplicate identical canonical records may be recognized only through
  explicitly idempotent migration semantics;
- conflicting destination records fail closed;
- completion writes one bounded verification receipt only after every check
  passes;
- activation requires the exact completed receipt.

Deleting a partial destination is an explicit operator recovery action, not an
automatic cleanup side effect.

## 12. Verification And Referential Integrity

Verification must compare source inventory with destination truth:

- per-family canonical record counts;
- per-family deterministic digests;
- event IDs, run IDs, contiguous sequence numbers, and ordered event digests;
- successful rehydration of every run;
- rebuilt snapshot equality with rehydrated run state;
- pending approval projection consistency;
- approval-presentation run and approval linkage;
- SideEffect workflow/run identity and lifecycle validity;
- WorkReport-to-run and WorkReport-to-SideEffect references;
- adapter telemetry run identity;
- project and audit record identity;
- no migrated locks;
- no unknown destination records;
- healthy SQLite schema metadata and `quick_check`.

Projection count differences are allowed only where the plan explicitly
rebuilds a projection from authoritative truth and records that disposition.

## 13. Activation And Rollback Posture

Import completion and backend activation are separate decisions.

Activation must:

- require a completed verification receipt;
- bind to the exact destination database identity and digest;
- recheck source and destination fingerprints;
- state which companion stores remain filesystem-backed;
- be explicit and auditable;
- refuse active writers or changed source state;
- preserve the source root as rollback evidence.

The first migration implementation must not delete or rename source state.
Rollback means selecting the preserved source before any new writes occur on
the destination. Bidirectional merge or rollback after both backends accept
writes is not supported.

## 14. Privacy And Redaction

Migration artifacts may be sensitive even when they contain only references.

The migration boundary must:

- store counts, digests, typed identities, and bounded findings rather than raw
  payloads;
- redact source and destination paths from `Debug`, errors, and reports;
- reject secret-like migration labels and unbounded operator reasons;
- never copy environment values, credentials, provider payloads, command
  output, source code, or arbitrary repository files;
- preserve existing record sensitivity and redaction metadata;
- avoid emitting canonical record JSON into logs;
- use stable non-leaking error codes.

## 15. Error Taxonomy

Candidate stable error families:

- `state.migration.input.invalid`;
- `state.migration.source.unhealthy`;
- `state.migration.source.changed`;
- `state.migration.source.busy`;
- `state.migration.source.unknown_record_family`;
- `state.migration.source.unsupported_state`;
- `state.migration.destination.not_empty`;
- `state.migration.destination.incompatible`;
- `state.migration.record.invalid`;
- `state.migration.record.conflict`;
- `state.migration.verification.failed`;
- `state.migration.resume.mismatch`;
- `state.migration.activation.not_ready`.

Messages must not echo paths, record payloads, credentials, raw identities
classified as sensitive, or secret-like caller input.

## 16. Test Plan

Future tests should cover:

- healthy empty and populated source inventory;
- every known filesystem record family;
- deterministic count and digest ordering;
- source inspection creates no files;
- symlink and path-escape rejection;
- corrupt record and dangling/mismatched index rejection;
- unknown state-family blocking;
- live and stale lock posture;
- immutable run-bundle companion preservation;
- canonical events imported in sequence;
- snapshots and approval projections rebuilt from events;
- indexes and locks not copied;
- destination must be new and empty;
- interruption before and after each record family;
- exact restart and changed-source rejection;
- duplicate and conflicting destination behavior;
- complete count, digest, rehydration, and referential verification;
- activation rejection before verification;
- source remains byte-for-byte unchanged;
- `Debug`, serialization, and errors do not leak paths or payload markers;
- filesystem and SQLite conformance suites remain green;
- no runtime, provider, artifact-export, or CLI behavior changes in model-only
  phases.

## 17. Proposed Implementation Sequence

1. **Read-only migration inventory and compatibility model (implemented and
   accepted)**
   - typed counts, dispositions, blockers, and source fingerprint;
   - `LocalStateBackend` inventory/export helper;
   - no destination creation or writes.
2. **Migration plan and staging-destination model (implemented)**
   - bind source fingerprint, destination identity, plan version, and schema;
   - derive deterministic family ordering, exact-plan resume posture, and
     pre-activation verification obligations;
   - no runtime backend selection.
   - focused review found one blocker: serialized local-filesystem source
     posture could weaken `quiescence_required`; the focused correction is
     implemented and must be re-reviewed before step 3.
3. **Verified importer helper**
   - deterministic canonical import into unreachable staging SQLite;
   - projection rebuild and interruption tests.
4. **Verification receipt**
   - counts, digests, rehydration, identity, and referential checks;
   - destination remains inactive.
5. **Explicit local activation planning and review**
   - decide configuration and companion-store behavior separately.
6. **CLI dry-run and execution planning**
   - only after helper review;
   - default dry-run, explicit execution and activation.
7. **Operational migration rehearsal**
   - real disposable filesystem state, interruption, recovery, and retained
     source proof before changing any default.

## 18. Open Questions

- Resolved for inventory v1: unknown empty directories warn, while unknown
  non-empty entries block compatibility and suppress the source fingerprint.
- What exact mechanism proves filesystem writer quiescence across processes?
- Should the importer rebuild all snapshots or preserve only an independently
  verified source snapshot?
- How should idempotency records be enumerated without widening ordinary
  runtime traits?
- Should pending approval projections be rebuilt exclusively from event truth?
- What stable database identity should bind the verification receipt?
- When should immutable run bundles gain a database-backed store?
- What constitutes byte-for-byte source preservation when unrelated temporary
  files are present?
- Should activation be a local configuration file, explicit CLI selection, or
  caller-supplied API only?
- What backup and checkpoint posture is required before SQLite becomes
  recommendable beyond disposable/local evaluation?

## 19. Final Recommendation

Re-review the fixed **migration plan local-filesystem quiescence
deserialization boundary** next.

The re-review should verify that public construction and serde both require
writer quiescence for the fixed local-filesystem source and that the exact
tamper regression fails closed. It must not create or write SQLite, import
records, add CLI behavior, activate a destination, select a backend, or alter
filesystem source state.
