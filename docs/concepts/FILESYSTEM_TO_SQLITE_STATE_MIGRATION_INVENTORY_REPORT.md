# Filesystem-To-SQLite State Migration Inventory Report

Review status: accepted with non-blocking follow-ups in
[Filesystem-To-SQLite State Migration Inventory Review](FILESYSTEM_TO_SQLITE_STATE_MIGRATION_INVENTORY_REVIEW.md).

## 1. Executive Summary

Workflow OS now has a read-only filesystem state migration inventory and
compatibility model. `LocalStateBackend::inspect_migration_inventory` inspects
the preview filesystem state boundary, validates known storage shapes, assigns
an explicit migration disposition to every known record family, and returns a
bounded inventory with a path-independent semantic fingerprint when all source
state is accounted for.

This phase does not migrate anything. It does not open or create SQLite,
construct a destination, import records, rebuild projections, establish writer
quiescence, activate a backend, expose CLI behavior, or change the filesystem
source.

## 2. Scope Completed

- Added a versioned migration inventory model.
- Added stable record-family, disposition, finding-severity, and finding-code
  vocabulary.
- Added bounded per-family counts and SHA-256 digests.
- Added a deterministic aggregate source fingerprint that excludes filesystem
  paths.
- Added read-only filesystem inspection through
  `LocalStateBackend::inspect_migration_inventory`.
- Added complete known-family coverage for events, projections, idempotency,
  approvals, policy and adapter records, reports, SideEffects, locks, and
  immutable run-bundle companion files.
- Added fail-closed checks for corrupt records, malformed addresses, duplicate
  identities, dangling projection indexes, symlinks, unexpected file types,
  unknown non-empty state, and unreadable source state.
- Resolved unknown empty directories as warnings; unknown non-empty entries are
  blockers.

## 3. Scope Explicitly Not Completed

- No SQLite destination is opened, created, or written.
- No filesystem record is imported, rewritten, repaired, deleted, or archived.
- No projection is rebuilt.
- No writer-quiescence mechanism or migration lock is implemented.
- No migration plan, resume identity, importer, verification receipt, rollback,
  activation, or backend-selection behavior is implemented.
- No CLI, schema, SDK, example, provider, hosted, PostgreSQL, or release-posture
  behavior changes are introduced.

## 4. Model And API Summary

The core model adds:

- `StateMigrationInventoryVersion`
- `StateMigrationRecordFamily`
- `StateMigrationDisposition`
- `StateMigrationDigest`
- `StateMigrationRecordCount`
- `StateMigrationFindingSeverity`
- `StateMigrationFindingCode`
- `StateMigrationCompatibilityFinding`
- `StateMigrationInventory`

`LocalStateBackend::for_inspection` can be paired with
`inspect_migration_inventory` to inspect an absent or existing source without
creating the state layout. The returned model exposes only typed counts,
digests, dispositions, findings, derived posture, and a source fingerprint. It
does not expose the source root, record addresses, or payloads.

## 5. State Classification

Canonical import candidates:

- workflow events;
- idempotency outcomes;
- approval-presentation proof records;
- project state;
- policy audit records;
- adapter audit and observability records;
- WorkReport artifacts;
- SideEffect records.

Projection rebuild candidates:

- event-ID indexes;
- run snapshots;
- pending approval projections;
- approval-presentation ID indexes;
- SideEffect ID indexes.

Ephemeral exclusions:

- local lock leases.

Companion preservation:

- recognized immutable run-bundle files, retained outside the SQLite schema-v1
  boundary.

## 6. Compatibility And Fingerprint Boundary

The inventory is migration-compatible only when it has no blocker finding and
every known family has a deterministic digest. A fingerprint is withheld when
state is corrupt, structurally ambiguous, unknown and non-empty, unreadable, or
referentially inconsistent.

The aggregate fingerprint binds the inventory version, quiescence requirement,
stable family names, dispositions, counts, and canonicalized family digests.
Family digests also bind bounded relative storage-address hashes where the
record body does not contain its canonical key, including idempotency and lookup
records. They never bind filesystem root paths. Moving semantically identical
source state therefore does not change the fingerprint, while changing an
idempotency key does.

Locks are counted and classified as excluded, but any present lock blocks
compatibility. Inventory v1 still reports that a future migration requires
explicit writer quiescence; it does not claim that an absent lock proves
quiescence.

## 7. Privacy And Redaction

Inspection parses validated records only to derive canonical hashes and storage
consistency. It does not return raw records, filenames, directory paths,
snippets, provider payloads, logs, command output, environment values, or
credentials.

Findings are typed and payload-free. Invalid serialized inventory state fails
closed with bounded errors. `Debug` and serialization tests verify that source
paths and secret-like corrupt payload markers are not exposed.

## 8. Test Coverage

Focused tests cover:

- absent source inspection without directory creation;
- complete family representation;
- populated event and idempotency inventory;
- path-independent deterministic fingerprints;
- canonical key/address changes affecting source fingerprints;
- source-tree non-mutation and no destination creation;
- unknown empty warnings and unknown non-empty blocking;
- corrupt known record non-leakage;
- live-lock exclusion and blocking;
- dangling SideEffect index rejection;
- immutable run-bundle companion classification;
- serde round trip and derived-posture tamper rejection;
- invalid digest, disposition, and family-coverage errors;
- missing-digest incompatibility;
- symlink rejection.

## 9. Validation Commands And Results

The following commands passed:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo test -p workflow-core --test state_migration_inventory` with 13 focused
  tests passing
- `npm run check:docs`
- `cargo audit`
- `git diff --check`

Opt-in live provider tests remained ignored by the workspace suite as designed;
this phase does not use provider access.

## 10. Remaining Known Limitations

- Companion immutable run-bundle files are structurally inventoried and hashed
  as opaque recognized files; importer-ready semantic validation remains later.
- Inventory does not establish cross-process writer quiescence.
- Inventory does not export canonical records to an importer boundary.
- No destination identity, plan identity, interruption model, or resume binding
  exists.
- Pending approval and snapshot reconstruction rules remain deferred to the
  importer phase.
- The filesystem backend remains the preview default and SQLite remains
  explicit opt-in.

## 11. Recommended Next Phase

Perform a focused maintainer review of the read-only migration inventory and
compatibility model.

The review should verify state-family coverage, canonical/projection
classification, filesystem grammar handling, corruption/index/unknown-state
failure behavior, semantic fingerprint stability, privacy, and strict source
non-mutation. It must not implement an importer or destination writes.
