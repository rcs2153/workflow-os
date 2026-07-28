# Filesystem-To-SQLite State Migration Plan Review

Review date: 2026-07-28

## 1. Executive Verdict

**Plan accepted; proceed to read-only migration inventory and compatibility
model implementation.**

The plan is appropriately conservative for governance state. It treats
migration as a sequence of separately reviewable inventory, import,
verification, and activation boundaries rather than a file-copy operation.

## 2. Scope Verification

The planning phase stayed within documentation-only scope.

It added no:

- migration model or helper code;
- source or destination writes;
- SQLite import or automatic selection;
- CLI migration surface;
- PostgreSQL or collaborative state;
- backup or restore behavior;
- runtime, schema, SDK, or example change;
- provider call or mutation;
- release posture change.

## 3. Source Boundary Assessment

The plan correctly inventories the actual filesystem backend rather than
assuming the aggregate `StateBackend` trait exposes every record.

It identifies that:

- several stores lack list APIs;
- adapter telemetry, reports, and SideEffects require run-aware enumeration;
- filesystem index files are projections;
- locks are ephemeral;
- immutable run bundles use a companion filesystem store not represented in
  SQLite schema version one;
- unknown non-empty state must block rather than disappear.

A migration-specific read-only inventory/export boundary is preferable to
widening ordinary runtime traits solely for migration.

## 4. Source-Of-Truth Assessment

The canonical/projection/ephemeral/companion classification is sound.

In particular:

- events remain authoritative;
- snapshots are rebuilt from rehydrated events;
- pending approvals are reconciled with event-backed run state;
- event, presentation, and SideEffect indexes are checked but not imported;
- locks are excluded;
- immutable run bundles remain explicitly retained companion state.

This avoids treating filesystem layout as the durable domain contract.

## 5. Quiescence Assessment

The plan correctly refuses to equate an absent lock file with writer
quiescence. It requires a dedicated exclusion boundary and a stable source
fingerprint before and after export.

The implementation must define the cross-process writer protocol before any
importer exists. The first read-only inventory phase may report that
quiescence is unresolved; it must not claim the source is safe to migrate.

## 6. Import And Restart Assessment

The proposed import order respects the important dependencies:

- events precede run projections;
- approvals and presentations follow event identity;
- SideEffects precede WorkReport reference verification;
- indexes are rebuilt through destination behavior;
- destination health and referential verification finish the import.

The staging-database posture is preferable to making a partially imported
database appear `ready`. Migration identity binds source fingerprint,
destination, plan version, and schema version, which is sufficient groundwork
for deterministic restart.

No implementation should rely on arbitrary direct SQL inserts merely to make
bulk import easier. If a migration-only transactional API becomes necessary,
it must preserve the same validated model and identity boundaries.

## 7. Verification Assessment

The plan requires more than record counts. It includes:

- canonical per-family digests;
- event sequence and identity checks;
- run rehydration;
- rebuilt snapshot comparison;
- approval and presentation linkage;
- SideEffect identity and lifecycle validity;
- WorkReport referential integrity;
- no unknown destination records;
- schema metadata and SQLite integrity checks.

This is the right acceptance threshold before a destination receipt can exist.

## 8. Activation And Recovery Assessment

Import completion and activation are correctly separate.

The source remains preserved, and rollback is intentionally limited to
selecting the untouched source before divergent destination writes. The plan
does not imply bidirectional reconciliation.

The exact activation configuration remains an open question and is properly
deferred beyond inventory and importer helpers.

## 9. Privacy And Error Assessment

The proposed migration artifacts are payload-free: counts, typed dispositions,
digests, bounded findings, and references.

The plan rejects path and payload leakage through `Debug`, errors, reports, and
logs. Candidate error families are stable and do not require echoing rejected
values.

## 10. Test Plan Assessment

The future test plan covers:

- all known record families;
- deterministic inventory;
- source non-mutation;
- symlink and path escape;
- corruption and index drift;
- unknown state;
- lock posture;
- companion state;
- ordered import and projection rebuild;
- interruption and restart;
- complete verification and activation gating;
- redaction-safe output;
- existing backend conformance.

The first implementation should focus only on inventory behavior and explicitly
mark importer, verification-receipt, and activation tests deferred.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Define whether unknown empty directories warn or block before implementation.
- Specify the canonical serialization used for record-family digests.
- Keep source fingerprinting separate from path identity so moving an unchanged
  source does not silently change semantic content.
- Decide whether stale lock evidence is always a blocker or can be cleared only
  through a separate recovery workflow.
- Add database identity vocabulary only when a staging destination is in scope.
- Keep immutable run-bundle migration separate until a compatible durable store
  exists.

## 13. Documentation Assessment

The migration plan, planning report, durable-store selection plan, and roadmap
consistently state:

- migration is planned, not implemented;
- the filesystem backend remains the preview default;
- SQLite remains explicit and opt-in;
- source preservation and explicit activation are mandatory;
- PostgreSQL and collaborative state remain later;
- the first implementation is read-only inventory only.

## 14. Validation

Passed:

- `npm run check:docs` under the pinned Node 20 toolchain;
- `git diff --check`;
- plan review against current filesystem and SQLite backend code.

## 15. Governed Review Record

- workflow: `dg/review`;
- run ID: `run-1785240865491565000-2`;
- approval ID:
  `approval/run-1785240865491565000-2/review-scope-approved`;
- presentation ID: `presentation/b3a7d6fa7dd4b699`;
- approval outcome: granted by delegated maintainer with persisted proof;
- phase status: completed;
- event summary: 39 events, one approval, zero retries, zero escalations.

Plan inspection, review authoring, validation, git, and later PR work occur
outside the kernel under the approved scope. The kernel coordinates and records
governance; it does not perform those operations.

## 16. Recommended Next Phase

Implement the **read-only migration inventory and compatibility model**.

The phase should enumerate known state deterministically, validate canonical
records, classify projections/locks/companion state, produce bounded blockers
and a payload-free source fingerprint, and prove source non-mutation. It must
not create SQLite, import records, add CLI behavior, or alter runtime backend
selection.
