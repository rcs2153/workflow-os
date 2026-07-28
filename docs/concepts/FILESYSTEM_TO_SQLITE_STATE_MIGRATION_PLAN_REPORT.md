# Filesystem-To-SQLite State Migration Plan Report

Report date: 2026-07-28

## 1. Executive Summary

Workflow OS now has a phase-ready plan for moving existing local filesystem
state into the accepted opt-in SQLite adapter without silently converting or
activating user state.

Focused review accepts the plan in
[Filesystem-To-SQLite State Migration Plan Review](FILESYSTEM_TO_SQLITE_STATE_MIGRATION_PLAN_REVIEW.md).

The plan makes source inspection, deterministic import, projection rebuild,
source preservation, interruption recovery, destination verification, and
explicit activation separate governed boundaries.

This phase is documentation only. It implements no migration API, importer,
SQLite write path, CLI command, backend selection, or default change.

## 2. Scope Completed

- Inspected the current filesystem state layout and SQLite schema.
- Classified canonical records, rebuildable projections, ephemeral locks, and
  companion state.
- Documented the gap between ordinary store traits and complete migration
  enumeration.
- Defined read-only inventory, source fingerprint, quiescence, deterministic
  import order, restart, verification, activation, and rollback posture.
- Defined future privacy, error, and test requirements.
- Sequenced the first implementation as read-only inventory only.
- Updated the roadmap and durable-store selection plan linkage.

## 3. Scope Explicitly Not Completed

- No migration model or helper code.
- No filesystem export.
- No SQLite import or staging database.
- No CLI migration or backend-selection command.
- No automatic migration or default change.
- No source mutation, deletion, repair, or archival.
- No backup or restore implementation.
- No PostgreSQL or collaborative state.
- No provider behavior or write expansion.

## 4. Key Architecture Findings

The filesystem root contains more than canonical records:

- event, presentation, and SideEffect index files are rebuildable projections;
- snapshots and pending approvals should be reconciled with event truth;
- local locks are ephemeral and must not migrate;
- immutable run bundles are a companion filesystem store outside SQLite schema
  version one.

The existing public traits cannot enumerate all state needed for migration.
The first implementation therefore needs a dedicated read-only
`LocalStateBackend` migration inventory/export boundary rather than weakening
ordinary runtime interfaces or scanning files ad hoc in a CLI command.

## 5. Safety Boundary

The plan requires:

- healthy and stable source state;
- explicit writer quiescence before import;
- a new empty staging destination;
- canonical deserialization and deterministic ordering;
- no direct copying of indexes or locks;
- full destination verification before a completion receipt;
- separate explicit activation;
- preserved source state.

Partial import must never become authoritative.

## 6. Privacy And Redaction

Future plans and receipts store counts, typed dispositions, bounded findings,
and deterministic digests rather than canonical payloads.

Paths, raw JSON, source contents, environment values, credentials, provider
payloads, and command output must not appear in `Debug`, errors, reports, or
migration logs.

## 7. Validation

Planning validation:

- `npm run check:docs` under the pinned Node 20 toolchain;
- `git diff --check`;
- claims reviewed against `LocalStateBackend`, `SqliteStateBackend`, the
  durable-state semantic contract, and accepted adapter review.

## 8. Governed Phase Record

- workflow: `dg/d`;
- run ID: `run-1785240590850157000-2`;
- approval ID:
  `approval/run-1785240590850157000-2/planning-approved`;
- presentation ID: `presentation/795af67d5266300b`;
- approval outcome: granted by delegated maintainer with persisted proof;
- phase status: completed;
- event summary: 39 events, one approval, zero retries, zero escalations.

Repository inspection, documentation edits, validation, git, and later PR work
occur outside the kernel under the approved scope. The kernel coordinates and
records governance; it does not perform those operations.

## 9. Remaining Limitations

- The inventory model and filesystem reader do not exist.
- Cross-process writer quiescence is not designed beyond requirements.
- SQLite schema version one has no migration-in-progress state.
- Immutable run bundles remain filesystem-only.
- Backend activation and rollback after destination writes are unresolved.
- Backup, restore, process-kill rehearsal, and network-filesystem posture remain
  unproven.

## 10. Recommended Next Phase

Implement the read-only migration inventory and compatibility model only.

It should enumerate known filesystem state deterministically, validate every
canonical record, classify projections/locks/companion state, expose blockers,
and calculate a payload-free source fingerprint. It must not create a
destination database or alter runtime behavior.
