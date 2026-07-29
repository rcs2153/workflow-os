# Filesystem-To-SQLite Writer Quiescence And Import Transaction Plan

Status: Planning complete; implementation not authorized until focused
maintainer review.

Related foundations:

- [Filesystem-To-SQLite State Migration Plan](filesystem-to-sqlite-state-migration-plan.md)
- [Filesystem-To-SQLite State Migration Plan Model Blocker Fix Review](../concepts/FILESYSTEM_TO_SQLITE_STATE_MIGRATION_PLAN_MODEL_BLOCKER_FIX_REVIEW.md)
- [Durable State Semantic Contract Report](../concepts/DURABLE_STATE_SEMANTIC_CONTRACT_REPORT.md)
- [SQLite Embedded Durable State Adapter Report](../concepts/SQLITE_EMBEDDED_DURABLE_STATE_ADAPTER_REPORT.md)

## 1. Executive Summary

The accepted migration plan model requires local-filesystem writer quiescence,
but the current runtime has no root-wide cross-process writer exclusion
protocol. Existing lock records are per-key, unfenced, and are not acquired by
every mutating store method. An empty `locks/` directory therefore cannot prove
that the source is stable.

Before any importer can write SQLite, every cooperating filesystem mutation
must participate in one root-wide writer guard. Ordinary mutations acquire a
shared guard for their complete write operation. Migration acquires the
exclusive guard and retains it through source re-inventory, export, import,
verification, and verification-receipt creation.

The first importer should write only an unreachable staging database. It should
import all canonical records and rebuild projections in one SQLite
`IMMEDIATE` transaction. Interruption before commit rolls the transaction back
and restarts the exact plan from the beginning. A committed staging database
remains inactive until post-commit verification succeeds. Backend activation
is a separate future decision.

This plan adds no lock implementation, importer, SQLite write, receipt,
activation, CLI command, backend selector, or schema change.

## 2. Goals

- Define the authority required to begin migration.
- Exclude all cooperating filesystem writers across local processes.
- Prove source stability while the exclusion guard is held.
- Preserve the source state root and companion stores.
- Define one atomic SQLite import transaction.
- Make every interruption state deterministic and fail closed.
- Separate import commit, verification receipt, and backend activation.
- Define bounded errors, evidence, and tests for the future implementation.
- Avoid implying protection against non-cooperating or older binaries.

## 3. Non-Goals

This plan does not authorize:

- a writer guard implementation;
- destination creation or SQLite writes;
- canonical record import or projection rebuild;
- source mutation, repair, archival, rename, or deletion;
- verification-receipt or activation model implementation;
- automatic migration, startup migration, or backend selection;
- migration CLI behavior;
- partial per-family checkpoint resume;
- PostgreSQL, shared workers, distributed leases, or network filesystems;
- workflow schema, SDK, example, or agent-harness changes;
- provider calls, provider writes, or adapter expansion;
- hosted runtime, tenant administration, or enterprise identity;
- release-posture changes.

## 4. Current Lock Boundary

`LocalStateBackend` currently exposes logical locks through
`LockStore::acquire_lock`. The filesystem implementation creates a
key-specific directory and removes it on release.

Those locks do not prove migration quiescence:

- store mutations outside `LockStore` do not all acquire one common lock;
- different keys do not exclude one another;
- a lock lease has no fencing token or expiry;
- release does not establish a cross-process ownership proof;
- a crashed process can leave a stale directory;
- an older or non-cooperating binary can write without observing a future
  migration marker.

The migration protocol must not reinterpret existing lock records as a
root-wide writer barrier. Existing live or stale logical locks remain
compatibility findings that require operator recovery before migration.

## 5. Authority Boundary

A future migration attempt must receive explicit, bounded authority that binds:

- migration ID;
- exact migration-plan fingerprint;
- source fingerprint;
- destination identity;
- actor or system actor;
- issuance and expiry time;
- approval or policy-decision reference when required;
- authorization for import only.

Import authority must not authorize:

- source repair or deletion;
- stale-lock removal;
- destination activation;
- backend-default changes;
- provider operations;
- migration under a different plan or source fingerprint.

Authority is checked before acquiring the guard and again before opening the
import transaction. Expired, revoked, mismatched, or ambiguous authority fails
before a destination write.

## 6. Cooperating Writer Protocol

The future local filesystem backend should use one stable coordination object
derived from the canonical state-root identity but stored outside the source
state root. Creating or updating coordination metadata must not change the
source fingerprint.

Every mutating filesystem store operation must:

1. acquire a shared root-wide writer guard;
2. perform all validation and mutation while retaining that guard;
3. release the guard only after the durable operation is complete or has
   failed;
4. return a stable busy/quiescing error when an exclusive migration guard is
   held.

This includes:

- event and event-index append;
- snapshot replacement;
- idempotency recording;
- logical lock acquire and release;
- approval projection save and delete;
- project-state replacement;
- policy-audit append;
- adapter audit and observability append;
- approval-presentation record and index append;
- SideEffect record and index writes;
- WorkReport artifact writes;
- any future mutating companion-store operation included in migration.

Migration must:

1. acquire the exclusive root-wide guard;
2. fail if bounded acquisition cannot succeed;
3. retain the same live guard object through final source recheck and
   verification receipt creation;
4. release it without modifying source records.

The reviewed implementation should prefer an operating-system advisory
shared/exclusive file lock whose ownership is released on process death.
Directory-existence polling or a check-then-create marker is insufficient
because it leaves a race between writers and migration.

The guard remains a local-filesystem protocol. It does not claim distributed
lease, fencing, network-filesystem, or hostile-process protection.

## 7. Compatibility Epoch

Cross-process exclusion is only valid when every writer uses the guard.
Migration cannot prove that an older binary or unrelated process is not
writing merely by acquiring a new advisory lock.

Before importer implementation, the filesystem backend must expose a bounded
writer-protocol version or equivalent capability marker. Migration requires:

- the source declares the supported writer protocol;
- the current importer supports that exact protocol;
- the operator explicitly confirms incompatible older writers are stopped;
- the source fingerprint remains stable while the exclusive guard is held.

The receipt must disclose that the guarantee covers cooperating local
Workflow OS writers. It must not claim protection from arbitrary filesystem
mutation.

## 8. Quiescence Acquisition Sequence

The future importer sequence is:

1. validate migration authority and the immutable plan;
2. inspect that no blocking logical-lock or unknown-state finding exists;
3. verify compatible writer-protocol posture;
4. acquire the exclusive source guard with a bounded wait;
5. re-run complete source inventory while holding the guard;
6. require the resulting fingerprint to equal the plan source fingerprint;
7. open or resume only the exact unreachable staging destination;
8. recheck authority;
9. execute the import transaction;
10. verify committed staging state and unchanged source under the same guard;
11. create one bounded verification receipt;
12. release the guard.

No destination write occurs before steps one through six succeed.

## 9. Source Stability And Preservation

The accepted source fingerprint covers known record-family counts, canonical
digests, dispositions, findings, and quiescence posture. Under the exclusive
guard, the importer must:

- inventory before export;
- export only validated known canonical records;
- inventory after import and before receipt;
- require both fingerprints to equal the plan fingerprint binding;
- reject new files, changed records, unknown families, symlinks, and path
  escapes;
- leave source records byte-for-byte unchanged;
- retain immutable run bundles and other recognized companion state;
- import no logical lock record.

A changed source always invalidates the attempt. The importer must not repair,
merge, overwrite, or silently create a replacement plan.

## 10. Staging Destination Boundary

The destination must be:

- a new explicitly identified SQLite staging database;
- created in the same filesystem location required for any future atomic
  promotion;
- inaccessible to ordinary runtime backend selection;
- opened only through a migration-specific internal constructor;
- initialized with migration state and exact plan/source/destination bindings;
- rejected when pre-existing content or incompatible metadata is present.

The ordinary `SqliteStateBackend::open` currently initializes metadata as
`ready`. It must not be used by the importer until a reviewed migration-only
opening boundary can create or validate non-ready staging metadata.

Candidate staging states are:

- `importing_empty`;
- `imported_unverified`;
- `verified_inactive`;
- `ready` only after a separately authorized activation phase.

The final names and schema are implementation details, but the state machine
and fail-closed visibility boundary are required.

## 11. Import Transaction

The v1 importer should use one SQLite `BEGIN IMMEDIATE` transaction for:

1. canonical event import in run and sequence order;
2. snapshot rehydration and projection rebuild;
3. pending approval projection rebuild;
4. approval-presentation import and index rebuild;
5. idempotency, project, policy-audit, and telemetry import;
6. SideEffect import and index rebuild;
7. WorkReport artifact import after referential checks;
8. in-transaction counts, identities, ordering, and referential validation;
9. transition of migration metadata to `imported_unverified`.

All writes must go through migration-specific validated operations that enforce
the same record and identity invariants as ordinary store APIs. Direct
unvalidated SQL payload insertion is prohibited.

The transaction commits once. Any error before commit rolls back every imported
record and rebuilt projection. The importer must not commit after each record
family in v1.

## 12. Interruption And Exact-Plan Resume

The v1 restart policy is deliberately small:

| Interruption point | Required recovery |
| --- | --- |
| Before staging creation | retry exact plan after revalidation |
| After empty staging initialization, before import | reuse only exact empty staging metadata |
| During uncommitted import transaction | SQLite rollback; restart import from the first family |
| After import commit, before verification | verify exact committed staging state; do not re-import |
| After verification, before activation | reuse exact verification receipt; activation remains separate |
| Unknown or conflicting state | fail for explicit operator recovery |

The implementation must not infer progress from row counts or skip individual
families. Exact resume requires matching migration ID, plan fingerprint, source
fingerprint, destination identity, adapter schema, and migration state.

Automatic deletion or replacement of a partial staging database is not
allowed.

## 13. Verification Boundary

After import commit and while source quiescence is still held, verification
must satisfy every `StateMigrationVerificationRequirement`, including:

- unchanged source fingerprint;
- initially empty destination proof;
- canonical counts and digests;
- event identity, sequence, and ordering;
- successful run rehydration;
- projection rebuild consistency;
- approval, SideEffect, WorkReport, telemetry, project, and audit identity;
- no imported locks;
- retained companion state;
- no unknown destination records;
- schema metadata and SQLite `quick_check`.

Verification failure leaves the destination non-ready and emits no successful
receipt. It does not roll back the already atomic import commit, delete
staging, change source, or activate the destination.

## 14. Verification Receipt And Activation Separation

A future verification receipt should be payload-free and bind:

- migration ID and plan version;
- plan, source, and destination fingerprints;
- adapter schema version;
- verification requirement outcomes;
- verification time and actor;
- destination content digest;
- retained companion-state posture;
- writer-protocol version;
- sensitivity and redaction metadata.

Receipt creation is not activation. Activation must later:

- consume the exact verified receipt;
- reacquire source quiescence;
- recheck source and destination fingerprints;
- make one explicit auditable backend-selection decision;
- preserve the filesystem source;
- define rollback only before post-activation destination writes.

## 15. Error Taxonomy

Future stable errors should include:

- `state.migration.authority.invalid`;
- `state.migration.authority.expired`;
- `state.migration.source.writer_protocol_incompatible`;
- `state.migration.source.quiescence_contended`;
- `state.migration.source.changed`;
- `state.migration.destination.not_empty`;
- `state.migration.destination.state_invalid`;
- `state.migration.destination.plan_mismatch`;
- `state.migration.transaction.begin_failed`;
- `state.migration.transaction.import_failed`;
- `state.migration.transaction.commit_failed`;
- `state.migration.resume.mismatch`;
- `state.migration.verification.failed`.

Messages must not contain paths, record payloads, raw command output,
credentials, tokens, source contents, or secret-like caller values.

## 16. Privacy And Evidence

The protocol stores only bounded identities, versions, counts, digests,
postures, and references. It must not log or serialize:

- filesystem paths;
- canonical record JSON;
- workflow inputs or outputs;
- provider payloads;
- command output;
- environment values;
- credentials or authorization headers;
- private keys or tokens.

The event/audit trail should record guard acquisition posture, transaction
posture, verification disposition, and stable error codes without copying
source or destination payloads.

## 17. Test Plan

Future implementation tests must prove:

- every filesystem mutation entrypoint participates in the writer guard;
- concurrent cooperating writer and migration attempts cannot overlap;
- migration fails when a logical lock or incompatible writer protocol exists;
- guard acquisition is released on normal failure and process termination;
- source inventory remains stable under the exclusive guard;
- changed source rejects before destination write and before receipt;
- destination is new, non-ready, and unreachable to ordinary runtime open;
- all canonical imports and projection rebuilds are one transaction;
- failure before commit leaves no imported rows;
- interruption after commit resumes verification without duplicate import;
- mismatched plan, source, destination, or schema rejects resume;
- unknown staging state requires explicit recovery;
- verification failure leaves staging inactive;
- receipt cannot exist before all verification obligations pass;
- source and companion state remain unchanged;
- no lock record is imported;
- SQLite `quick_check` and relational validation run after commit;
- errors, `Debug`, serialization, and audit records do not leak paths or
  payload markers;
- existing filesystem and SQLite conformance suites remain green.

Cross-process tests should use separate processes, not threads alone.

## 18. Proposed Implementation Sequence

1. **Writer guard and compatibility capability model**
   - model guard mode, writer-protocol version, and bounded acquisition
     outcomes;
   - no filesystem lock acquisition yet.
2. **Local filesystem cooperating writer guard**
   - add shared guard acquisition to every mutation path;
   - add an exclusive read-only inspection proof;
   - review before importer work.
3. **Migration-only staging constructor and metadata**
   - create only non-ready unreachable SQLite staging state;
   - no canonical import.
4. **Atomic importer helper**
   - one `IMMEDIATE` transaction;
   - canonical imports and projection rebuilds;
   - interruption tests.
5. **Post-commit verification and receipt**
   - complete verification obligations;
   - destination remains inactive.
6. **Activation planning and review**
   - exact receipt consumption, source recheck, companion posture, rollback
     boundary.
7. **CLI and operational rehearsal**
   - only after all helper phases pass maintainer review.

## 19. Open Questions

- Which reviewed cross-platform advisory-lock implementation should back the
  local guard?
- Should ordinary writers acquire shared guards concurrently or use one
  simpler exclusive root-wide writer mutex for preview?
- Where should the path-independent writer-protocol marker live?
- What bounded operator assertion is sufficient to declare incompatible older
  writers stopped?
- Should post-commit verification and receipt persistence share one SQLite
  transaction?
- What canonical destination digest binds the receipt without exposing
  payloads?
- Should staging activation use atomic same-filesystem rename or explicit
  configuration selection?
- How should immutable run bundles remain available after activation?

## 20. Final Recommendation

Implement the **writer guard and compatibility capability model only** next,
after focused review of this plan.

Do not begin canonical import, staging writes, verification receipts, CLI
migration behavior, or activation until the cross-process guard protocol is
implemented across every filesystem mutation and independently reviewed.
