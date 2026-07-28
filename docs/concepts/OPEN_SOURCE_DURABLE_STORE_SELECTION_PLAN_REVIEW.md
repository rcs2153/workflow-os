# Open-Source Durable Store Selection Plan Review

Review date: 2026-07-28

## 1. Executive Verdict

**Plan and ADR accepted with non-blocking follow-ups. Proceed to the durable
state semantic contract and backend conformance harness.**

The SQLite embedded and PostgreSQL shared adapter split is appropriate for the
local-first and collaborative product postures. One Core-owned semantic
contract is required before either adapter is implemented.

Two review blockers were corrected before acceptance:

- current CockroachDB releases are not represented as an open-source candidate;
- external provider mutation is explicitly outside database transaction
  atomicity and requires durable attempted-state and reconciliation semantics.

No implementation blocker remains.

## 2. Scope Verification

The phase stayed within planning and architecture-decision scope.

It did not add:

- database dependencies;
- SQL schemas;
- database adapters;
- migration behavior;
- automatic user-state conversion;
- hosted or collaborative runtime behavior;
- enterprise administration;
- workflow schema or SDK changes;
- provider mutation expansion;
- OpenShell integration;
- release posture changes.

## 3. Current-State Assessment

The plan correctly inspected the existing storage boundary.

Core already defines individual interfaces for:

- event logs;
- snapshots;
- idempotency;
- locks;
- approvals;
- approval-presentation proof;
- project metadata;
- policy audit;
- adapter telemetry;
- WorkReport artifacts;
- SideEffect records.

Immutable run bundles use a separate create-only local store. The aggregate
`StateBackend` does not currently include every newer store interface.

The review agrees that the primary missing abstraction is not another CRUD
trait. It is an executable semantic contract for transaction families,
revisions, conflicts, leases, migrations, recovery, and deterministic reads.

## 4. Local And Shared Store Decision

### SQLite

SQLite is accepted for embedded local state.

It preserves:

- no-daemon local onboarding;
- one-file inspectability;
- mature transactions and recovery;
- broad platform support;
- explicit backup APIs.

The plan accurately limits it to same-host use. SQLite WAL documentation states
that WAL requires same-host shared memory, does not work over network
filesystems, and permits one writer at a time.

The implementation must pin a supported SQLite version and account for
documented WAL fixes. This is a later dependency-review requirement, not a
planning blocker.

### PostgreSQL

PostgreSQL is accepted for shared collaborative state.

Its transactions, MVCC, serializable isolation, locks, constraints, migration
ecosystem, backup, point-in-time recovery, observability, and operational
maturity fit concurrent stateless workers.

The plan correctly avoids making PostgreSQL a mandatory first-run dependency.

### Compatible Adapters

Two adapters are justified. A forced one-database solution would either:

- add unacceptable local installation friction; or
- overstate an embedded database's distributed collaboration posture.

The shared semantic contract must be strong enough to preserve invariants, not
reduced to the weakest physical implementation.

## 5. Candidate Assessment

FoundationDB was reasonably considered and rejected for the first shared
adapter. Its strict-serializable transaction model is strong, but its key-value
layer, operational model, transaction limits, value limits, and lack of
user-level access control would move disproportionate database design into
Workflow OS.

The initial plan listed CockroachDB beside open-source candidates while also
noting licensing concerns. Current CockroachDB releases use the CockroachDB
Software License. The plan and ADR now classify it as an excluded
source-available comparator rather than an open-source candidate.

That correction is required by the roadmap's explicit open-source-store
constraint.

## 6. Transaction Boundary Assessment

The plan identifies the right internal transactional families:

- ordered event append;
- immutable bundle publication;
- idempotency reservation;
- approval decision and presentation proof;
- SideEffect lifecycle and event append;
- authoritative records and projections.

The initial draft implied that idempotency reservation and external operation
outcome could be one database transaction. That is not possible across a
database and an external provider.

The corrected plan now requires:

1. durable reservation and pre-effect intent;
2. external operation outside the database transaction;
3. durable outcome or ambiguity recording;
4. reconciliation before retry.

The review accepts this correction. No adapter may claim exactly-once external
effects or distributed rollback merely because its local writes are atomic.

## 7. Concurrency Assessment

The plan addresses:

- concurrent next-event writers;
- optimistic conflict detection;
- compare-and-set lifecycle transitions;
- deterministic retry classification;
- lease expiry and fencing;
- fresh authority resolution after conflict;
- no long-lived database transaction across human approval waits.

The eventual conformance suite must require the same observable result from
SQLite and PostgreSQL while allowing different physical locking mechanisms.

## 8. Event And Projection Assessment

The source-of-truth boundaries remain correct:

- workflow events govern run state;
- immutable bundles govern exact run inputs;
- SideEffect records govern SideEffect intent and lifecycle;
- approvals and presentation proof retain their authority roles;
- snapshots, indexes, telemetry, audit projections, and reports remain
  projections or handoff artifacts as already documented.

Relational convenience must not promote projections into authority.

The plan should not automatically add every store interface to the aggregate
`StateBackend`. The conformance implementation should first decide which
interfaces require one transactional boundary and which remain independently
composable. This is a non-blocking design follow-up.

## 9. Migration And Compatibility Assessment

The plan correctly rejects automatic filesystem migration.

The required future migration posture includes:

- dry run;
- source health;
- writer exclusion or a consistent read boundary;
- deterministic order;
- count, hash, ordering, and referential-integrity verification;
- projection rebuild;
- source preservation;
- explicit destination activation;
- bounded reporting.

Database schema versions are correctly separated from workflow specification
versions.

## 10. Backup And Recovery Assessment

The plan distinguishes:

- authoritative record restore;
- event replay;
- projection rebuild;
- external SideEffect reconciliation.

That distinction is essential. Restoring database state cannot roll back an
external provider mutation.

SQLite backup must use the backup API or a correctly coordinated snapshot.
PostgreSQL operations must include an exercised restore and point-in-time
recovery posture. Merely documenting backup commands will not satisfy adapter
acceptance.

## 11. Security And Privacy Assessment

The plan preserves existing exclusions for:

- credentials and connection secrets;
- raw provider payloads;
- authorization headers;
- environment values;
- raw source contents;
- unrestricted command output;
- secret-like metadata.

It correctly defers hosted tenancy, enterprise identity, row-level security,
encryption/key-management posture, and collaborative administration to
separate threat models.

Shared database selection does not itself provide tenant isolation.

## 12. Test Plan Assessment

The proposed conformance coverage is substantive and behavior-oriented:

- ordering and duplicate rejection;
- immutable identity;
- concurrency conflicts;
- idempotency;
- approval proof;
- SideEffect compare-and-set and event atomicity;
- lease fencing;
- deterministic reads;
- migration interruption;
- corruption;
- backup and restore;
- redaction.

Backend-specific concurrency and recovery tests are also included.

The first implementation must make scenarios executable rather than introduce
model types without behavioral tests.

## 13. Documentation Assessment

The ADR, plan, report, and roadmap now state:

- SQLite is selected for embedded local state;
- PostgreSQL is selected for shared collaborative state;
- both use one Core-owned semantic contract;
- CockroachDB is not an open-source candidate under current licensing;
- external provider effects are not database-atomic;
- no database adapter or dependency is implemented;
- the next phase is the semantic contract and conformance harness.

No capability overclaim remains.

## 14. Blockers

None after the two draft corrections.

## 15. Non-Blocking Follow-Ups

- Decide whether transactional composition extends `StateBackend` or uses
  narrowly typed transaction-family interfaces.
- Decide which records retain canonical serialized envelopes and which fields
  are normalized projections.
- Pin supported SQLite versions and compile options during dependency review.
- Pin supported PostgreSQL versions during adapter planning.
- Define lease duration and fencing semantics from actual worker behavior.
- Decide whether report artifact bytes remain database-local or later move
  behind a blob/artifact interface.
- Add CockroachDB or another distributed store only if a future scale
  requirement justifies reopening the ADR.

## 16. Recommended Next Phase

Implement the **durable state semantic contract and backend conformance
harness**, with no database dependency.

That phase should:

- define only the transaction-family, conflict, revision, lease, capability,
  and schema metadata needed by executable scenarios;
- test applicable semantics against `LocalStateBackend`;
- explicitly report guarantees the filesystem backend cannot satisfy;
- avoid SQL schema, adapter, migration, hosted, or collaborative-consumer work.

After focused review, proceed to the SQLite embedded adapter.

## 17. Validation

| Validation | Result |
| --- | --- |
| `npm run check:docs` with pinned Node 20 | Passed |
| `git diff --check` | Passed |
| Current state/store interface inspection | Passed |
| SQLite primary documentation review | Completed |
| PostgreSQL primary documentation review | Completed |
| FoundationDB primary documentation review | Completed |
| CockroachDB release-license documentation review | Completed |
| Runtime/dependency change inspection | No runtime or dependency changes |

## 18. Governed Review Record

- workflow ID: `dg/review`
- run ID: `run-1785223102052656000-2`
- approval ID:
  `approval/run-1785223102052656000-2/review-scope-approved`
- presentation ID: `presentation/f18de46c2ee6a8dc`
- approval outcome: granted
- terminal status: completed
- events: 39
- retries: 0
- escalations: 0
- presentation proof marker: present
- scope: focused documentation and architecture review only
