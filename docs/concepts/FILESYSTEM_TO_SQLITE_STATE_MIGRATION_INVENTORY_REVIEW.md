# Filesystem-To-SQLite State Migration Inventory Review

Review date: 2026-07-28

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups; proceed to migration plan and
staging-destination core-model implementation.**

The implementation establishes a deterministic, read-only compatibility
boundary over the current filesystem state layout. It does not create or write
SQLite, mutate the source, select a backend, or imply that migration is ready.

## 2. Scope Verification

The phase stayed within the approved inventory-only scope.

It added:

- a versioned, domain-neutral inventory model;
- explicit record-family dispositions;
- bounded compatibility findings;
- deterministic family digests and a source fingerprint;
- a read-only `LocalStateBackend` inspection entry point;
- focused model, corruption, index, privacy, and source non-mutation tests.

It did not add:

- a migration importer or destination writes;
- SQLite creation, staging, activation, or rollback;
- backend selection or default changes;
- a migration CLI;
- source repair, archival, deletion, or lock clearing;
- PostgreSQL, collaborative state, schemas, SDKs, examples, providers, or
  release-posture changes.

## 3. Model Assessment

The public model is appropriately small and migration-specific:

- `StateMigrationInventoryVersion`;
- `StateMigrationInventory`;
- `StateMigrationRecordFamily`;
- `StateMigrationRecordCount`;
- `StateMigrationDisposition`;
- `StateMigrationCompatibilityFinding`;
- `StateMigrationFindingSeverity`;
- `StateMigrationFindingCode`;
- `StateMigrationDigest`.

All 16 known state families are represented exactly once and assigned a stable
disposition. Canonical records, rebuildable projections, ephemeral locks, and
companion immutable bundles remain distinct.

The model does not expose raw records, filesystem paths, arbitrary diagnostic
text, destination configuration, or importer state.

## 4. Source Inventory Assessment

`LocalStateBackend::inspect_migration_inventory()` uses the inspection-only
backend posture and enumerates only known state-root families.

The scanner:

- validates canonical records through existing typed serde boundaries;
- checks storage-address shape and record identity;
- rejects duplicate event identities;
- validates event, approval-presentation, and SideEffect lookup indexes where
  present;
- rejects corrupt records, malformed addresses, unexpected file types, and
  symlinks;
- treats unknown empty directories as warnings and unknown non-empty state as
  blockers;
- counts locks as excluded state while blocking compatibility when any lock is
  present;
- inventories recognized immutable run-bundle files as companion state.

Unknown or unreadable state suppresses the source fingerprint rather than
being silently ignored.

## 5. Fingerprint And Compatibility Assessment

The source fingerprint is semantic and path-independent. It binds:

- inventory version;
- quiescence-required posture;
- stable family names;
- family dispositions;
- record counts;
- validated family digests.

Family digests include canonical serialized records. For records whose
canonical key is carried only by their filesystem address, the digest also
binds a bounded relative storage-address hash. The review added a regression
test proving that changing an idempotency key changes the source fingerprint
even when the stored result body is unchanged.

The filesystem root path is never included. Semantically identical state moved
to another root therefore keeps the same fingerprint.

Migration compatibility requires:

- no blocker findings;
- a digest for every known family;
- a derived source fingerprint.

The inventory continues to state that writer quiescence is required. An absent
lock does not claim cross-process quiescence.

## 6. Read-Only And Source-Safety Assessment

Inspection does not:

- create an absent source root;
- create a SQLite database;
- write, rename, remove, repair, or archive source records;
- acquire, release, or clear locks;
- rebuild projections;
- create runtime events or reports.

Focused tests compare the source tree before and after inspection and verify
that an absent source remains absent.

## 7. Error And Privacy Assessment

Compatibility findings are typed and payload-free. Errors use stable
`state.migration.*` codes and bounded messages.

The scanner does not return:

- source-root or record paths;
- filenames or unhashed record keys;
- raw JSON records;
- snippets, provider payloads, command output, logs, or environment values;
- credentials, authorization headers, private keys, or token-like values.

Corrupt-record, serde, `Debug`, and serialization tests verify that
secret-like fixture values and source paths do not appear in output.

Deserialization denies unknown fields and reconstructs derived posture through
validated constructors, so serialized callers cannot assert a false healthy or
compatible state.

## 8. Index And Companion-State Assessment

Event-ID indexes are checked during event inventory. SideEffect and
approval-presentation indexes are validated against their authoritative
records. Dangling or mismatched indexes block compatibility.

Indexes remain projection-rebuild families and are not treated as canonical
import payloads.

Immutable run-bundle files are recognized, counted, and hashed as opaque
companion records. This is an honest preservation boundary, not a claim that
SQLite schema v1 stores or semantically validates those bundles.

## 9. Test Quality Assessment

The 13 focused tests cover:

- absent-source read-only behavior;
- complete known-family coverage;
- deterministic path-independent fingerprints;
- canonical storage-key participation in fingerprint identity;
- source non-mutation and no destination creation;
- unknown empty and unknown non-empty state;
- corrupt-record non-leakage;
- live-lock exclusion and blocking;
- dangling SideEffect index rejection;
- immutable companion classification;
- serde round trip and derived-posture tamper rejection;
- invalid digest, disposition, and family coverage;
- missing-digest incompatibility;
- symlink rejection.

The workspace suite preserves existing filesystem, SQLite, runtime,
EvidenceReference, WorkReport, SideEffect, approval, adapter, and CLI behavior.

## 10. Documentation Assessment

The roadmap, migration plan, and implementation report consistently state:

- the read-only inventory is implemented;
- the filesystem backend remains the preview default;
- SQLite remains explicit and opt-in;
- no importer or destination write exists;
- no migration command or activation path exists;
- no source mutation exists;
- PostgreSQL and collaborative state remain later.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Define a cross-process writer-quiescence protocol before any importer.
- Decide whether missing rebuildable index entries should produce warnings or
  blockers in a future inventory version; current v1 blocks dangling and
  mismatched indexes and rebuilds projections later.
- Add semantic validation for immutable run bundles only when a compatible
  durable bundle-store boundary is planned.
- Bind a future migration plan to source fingerprint, destination identity,
  destination schema, and plan version before any destination write.
- Preserve a second source-fingerprint check immediately before a future
  import begins.

## 13. Recommended Next Phase

Implement the **migration plan and staging-destination core model only**.

That phase should model:

- migration identity and plan version;
- source fingerprint binding;
- explicit filesystem source and SQLite staging destination identity;
- deterministic family ordering and dispositions;
- empty/staging destination requirements;
- interruption and resume posture;
- verification obligations before activation.

It must still not import records, create or write SQLite, mutate filesystem
state, add a migration CLI, activate a destination, or change the default
backend.

## 14. Governed Review Record

- workflow: `dg/review`;
- run ID: `run-1785255879123303000-2`;
- approval ID:
  `approval/run-1785255879123303000-2/review-scope-approved`;
- presentation ID: `presentation/b1244419d6658b1a`;
- approval outcome: granted by delegated maintainer with persisted proof;
- phase status: completed;
- event summary: 39 events, one approval, zero retries, zero escalations;
- approval-presentation enforcement: proof enforced with an event marker;
- validation expectation: formatting, workspace lint/tests, docs, dependency
  audit, and diff checks;
- validation outcome: all expected checks passed;
- out-of-kernel work: source inspection, blocker fix, review authoring,
  validation, git, and PR operations.

The kernel coordinates and records governance. It does not perform repository
edits, tests, commits, pushes, or merges.
