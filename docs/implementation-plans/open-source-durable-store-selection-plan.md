# Open-Source Durable Store Selection Plan

Status: Accepted after focused maintainer review in
[Open-Source Durable Store Selection Plan Review](../concepts/OPEN_SOURCE_DURABLE_STORE_SELECTION_PLAN_REVIEW.md).

Related decision:
[ADR 0012: Compatible SQLite And PostgreSQL Durable State Adapters](../adr/0012-compatible-sqlite-postgresql-durable-state-adapters.md).

Implementation status: the first database-free semantic contract and
executable local-filesystem conformance harness are implemented in
[Durable State Semantic Contract Report](../concepts/DURABLE_STATE_SEMANTIC_CONTRACT_REPORT.md).
Focused review in the
[Durable State Semantic Contract Review](../concepts/DURABLE_STATE_SEMANTIC_CONTRACT_REVIEW.md)
accepts the phase after two bounded harness fixes. The first opt-in SQLite
embedded adapter is implemented in
[SQLite Embedded Durable State Adapter Report](../concepts/SQLITE_EMBEDDED_DURABLE_STATE_ADAPTER_REPORT.md).
Focused review in
[SQLite Embedded Durable State Adapter Review](../concepts/SQLITE_EMBEDDED_DURABLE_STATE_ADAPTER_REVIEW.md)
accepts the adapter after adding authoritative read-time relational identity
enforcement. It adds a reviewed bundled SQLite dependency, managed schema version one,
existing state-store behavior, and expanded conformance tests. It remains
local, explicit, and not selected by the runtime or CLI. Automatic migration,
backup/restore acceptance, shared-worker guarantees, PostgreSQL, and
collaborative consumers remain unimplemented.

## 1. Executive Summary

Workflow OS already externalizes durable state through Rust interfaces and
ships a local filesystem backend. That is sufficient for preview dogfooding,
but it is not a responsible collaboration backend.

This plan selected:

- SQLite for embedded local durable state;
- PostgreSQL for shared collaborative durable state;
- one Core-owned semantic contract and conformance suite for both adapters.

The database-free contract and first embedded adapter slice are now
implemented. This document remains the governing selection and sequencing
record; it does not authorize automatic adoption of the adapter or later
shared-state phases.

## 2. Goals

- Preserve the local-first, no-daemon first-run experience.
- Provide a credible path to concurrent stateless workers and collaborating
  users.
- Keep Workflow OS domain invariants independent of database-specific behavior.
- Define transaction boundaries before implementing SQL schemas.
- Cover events, immutable records, idempotency, approvals, authority,
  SideEffects, evidence, reports, artifacts, telemetry, and projections.
- Define deterministic conflict, retry, migration, backup, restore, recovery,
  and inspection behavior.
- Reuse mature open-source databases rather than inventing one.
- Make filesystem migration explicit and verifiable.

## 3. Non-Goals

This plan does not by itself authorize:

- PostgreSQL adapter code;
- automatic SQLite selection or existing-state migration;
- migration tooling beyond schema-readiness detection;
- automatic conversion of existing user state;
- a hosted Workflow OS service;
- tenant administration or enterprise control-plane behavior;
- workflow schema or SDK changes;
- provider mutation expansion;
- OpenShell or sandbox integration;
- secrets storage;
- report export;
- collaborative UI;
- replacement of Git for authored workflow definitions;
- a bespoke database;
- release posture changes.

## 4. Current State And Gaps

Current Core traits cover:

- `EventLogStore`;
- `RunSnapshotStore`;
- `IdempotencyStore`;
- `LockStore`;
- `ApprovalStore`;
- `ApprovalPresentationRecordStore`;
- `ProjectStateStore`;
- `PolicyAuditStore`;
- `AdapterTelemetryStore`;
- `WorkReportArtifactStore`;
- `SideEffectRecordStore`.

Immutable run bundles have a separate create-only local store. The aggregate
`StateBackend` includes several but not all newer store contracts.

The filesystem implementation has useful local invariants:

- create-only event and record files;
- contiguous event sequence validation;
- immutable run-identity checks;
- idempotency records;
- local lock files;
- projection validation;
- local integrity inspection.

It does not define or provide a general atomic transaction across event,
approval, authority, SideEffect, idempotency, report, and projection records.
The Rust traits also do not yet define:

- transactional mutation families;
- compare-and-set revisions;
- lease expiry and fencing;
- conflict retry categories;
- migration metadata;
- backend capability negotiation;
- cross-backend conformance tests.

These gaps must be resolved before schema design.

## 5. Required Storage Invariants

### Ordered Append-Only Events

- A run event sequence is contiguous and starts at the canonical first value.
- Event IDs are globally or scope-appropriately unique.
- A duplicate sequence or event ID fails closed.
- Immutable workflow/run identity cannot change.
- Concurrent append attempts produce one deterministic winner or a typed
  retryable conflict.
- Reads return canonical sequence order.

### Create-Only And Immutable Records

- Immutable run definitions and manifests are create-only.
- Content-addressed records verify their address.
- A duplicate identical idempotent record may return the original result only
  where the contract explicitly permits it.
- A different value at the same identity fails closed.
- Partial publication cannot make an unresolved bundle authoritative.

### Idempotency

- Reservation, pre-effect intent, and post-effect outcome cannot produce
  contradictory records.
- Duplicate keys return the prior bounded result.
- In-progress, completed, failed, and operator-recovery postures must be
  explicit where needed.
- Retention and reuse rules must be versioned.

### Approval And Authority

- Approval decisions bind to current immutable run context.
- Required presentation proof is current and matches the approval.
- Scoped authority records cannot be substituted across actor, workflow, run,
  step, resource, or time.
- Decisions and the authoritative events that expose them must commit
  atomically where separation would create an execution bypass.

### SideEffects

- Lifecycle transitions use compare-and-set or an equivalent transaction.
- Attempted/completed/failed records and authoritative workflow events cannot
  diverge silently.
- Provider reconciliation and retry posture remain deterministic after worker
  failure.
- Approval and capability references are validated in the same transaction
  when they are required to authorize an attempt.
- No database transaction spans an external provider call. Attempt intent must
  be durable before the call, and observed outcome or ambiguity must be
  recorded and reconciled afterward.

### Projections And Reports

- Snapshots and indexes are rebuildable from authoritative records where
  documented.
- Projection freshness is explicit.
- Report artifacts and evidence citations preserve referential integrity.
- Projection failure must not rewrite workflow success/failure semantics unless
  an accepted contract explicitly says otherwise.

## 6. Candidate Evaluation

| Criterion | SQLite | PostgreSQL | FoundationDB | CockroachDB comparator |
| --- | --- | --- | --- | --- |
| Embedded local operation | Excellent | Poor | Poor | Poor |
| Shared multi-host operation | Not appropriate | Excellent | Excellent | Excellent |
| Transactional integrity | Strong, one database file | Strong | Strict serializable | Serializable by default |
| Concurrent writers | One writer at a time | Mature MVCC | Optimistic distributed | Distributed with retries |
| Relational inspection | Strong | Excellent | Requires a layer | Excellent |
| Migration ecosystem | Mature but application-owned | Mature | Application layer required | Mature SQL tooling |
| Backup/recovery | Backup API, file/WAL discipline | Mature backup and PITR | Mature but specialized | Mature distributed tooling |
| Local adoption friction | Minimal | Service/container required | Cluster required | Cluster/service required |
| Operational burden | Low | Moderate and familiar | High and specialized | High and distributed |
| License posture | Public domain | PostgreSQL License | Apache 2.0 | Current releases use CockroachDB Software License |
| Fit for first Workflow OS role | Embedded adapter | Shared adapter | Not selected | Excluded from OSS candidate set |

The table evaluates suitability, not absolute database quality.

## 7. Selected Architecture

```text
Workflow OS domain and executor
            |
Core durable-state semantic contract
            |
Backend conformance suite
       /                 \
SQLite embedded      PostgreSQL shared
       |
explicit import from legacy filesystem state
```

The two adapters must provide equivalent observable domain behavior. They do
not need identical physical schemas or lock mechanisms.

### Why One Database Is Not Recommended

PostgreSQL everywhere would make local onboarding depend on a service or
container. SQLite everywhere would misuse a same-host, single-writer embedded
database as a distributed collaboration store.

Compatible adapters preserve both requirements without treating either
database as the domain model.

## 8. Semantic Contract Design

The first code phase should introduce internal contract vocabulary only where
it removes ambiguity. Candidate concepts include:

- `DurableStateCapabilities`;
- `DurableStateTransactionKind`;
- `DurableWriteConflict`;
- `DurableRevision`;
- `DurableLease`;
- `SchemaVersionRecord`;
- `MigrationState`;
- `BackendConformanceScenario`.

Names are illustrative. The implementation should add only types required by
executable conformance tests.

The contract should classify errors as:

- invalid input;
- invariant violation;
- duplicate/idempotent replay;
- retryable write conflict;
- lock or lease contention;
- unavailable backend;
- incompatible schema;
- corrupt state;
- migration required;
- recovery required.

Error messages must remain stable and non-leaking.

## 9. Transactional Mutation Families

The implementation plan must enumerate transaction families rather than expose
an unrestricted generic transaction object.

Initial families should include:

1. `append_run_event`
   - validates current tail, immutable identity, and transition;
   - appends exactly one next event.
2. `reserve_idempotency_and_record_intent`
   - reserves one operation identity and records durable pre-effect intent;
   - prevents an unrecorded effect attempt.
3. `record_external_operation_outcome`
   - records observed outcome in a transaction after an external call;
   - requires reconciliation before an ambiguous operation can be retried.
4. `record_approval_decision`
   - validates current request, context, and presentation proof;
   - appends the decision event and updates projections atomically.
5. `transition_side_effect`
   - compares the current lifecycle revision;
   - writes the new lifecycle record and event atomically.
6. `publish_immutable_run_bundle`
   - verifies all referenced records;
   - creates the run manifest exactly once.
7. `commit_authoritative_result_and_projections`
   - commits authority first;
   - treats rebuildable projection handling according to an explicit failure
     policy.

The contract should avoid a single catch-all transaction API that lets adapters
invent new state semantics.

The transaction families do not create a distributed transaction with GitHub,
Jira, OpenShell, or another provider. External effects remain outside the
database transaction and require attempted-state persistence, idempotency,
outcome recording, and reconciliation.

## 10. Concurrency And Worker Semantics

- Workers remain stateless.
- Every mutating operation carries expected identity and revision context.
- Retryable conflicts are explicit and bounded.
- Retrying must re-read current authority and immutable context.
- Locks use expiry and fencing before they are shared across workers.
- A crashed worker cannot retain ambient authority indefinitely.
- Long human approval waits never hold a database transaction open.
- Approval resume performs a fresh transaction against current durable facts.

SQLite tests should use multiple connections on one host. PostgreSQL tests
should use independent connections and processes where practical.

## 11. Schema And Migration Posture

Every database adapter needs:

- an adapter schema version;
- ordered, checksum-bound migrations;
- detection of newer unsupported schemas;
- fail-closed partial migration detection;
- migration locking;
- bounded migration diagnostics;
- forward-only production migration posture unless an explicit rollback is
  proven safe;
- compatibility tests across supported previous versions.

Database schema versions are not workflow spec schema versions.

Core records should retain canonical serialized or hash-bound representations
where exact compatibility matters. SQL columns and indexes may project fields
for constraints and queries, but projections must not change canonical hashes.

## 12. Backup, Restore, And Recovery

### SQLite

- use the SQLite backup API or a correctly coordinated snapshot;
- include the database, WAL, and shared-memory state as required by the chosen
  method;
- validate integrity after restore;
- document checkpoint and synchronous settings;
- reject unsupported network-filesystem placement.

### PostgreSQL

- document logical and physical backup boundaries;
- define supported point-in-time recovery posture;
- validate schema and event-chain integrity after restore;
- test restoration, not only backup creation;
- keep operational credentials outside governed records.

For both adapters, recovery must distinguish:

- rebuilding projections;
- replaying events;
- restoring authoritative records;
- reconciling ambiguous external SideEffects.

Database restoration must not imply that an external provider mutation was
rolled back.

## 13. Security And Privacy

- Store only validated bounded model data.
- Preserve existing redaction metadata and sensitivity posture.
- Never log connection strings, passwords, tokens, raw provider payloads,
  unrestricted command output, or raw source content.
- Use least-privilege database roles in the shared adapter.
- Separate migration authority from ordinary worker authority.
- Make destructive maintenance commands explicit and auditable.
- Treat database compromise as a governance-record integrity threat.

Encryption, tenant isolation, row-level security, external key management, and
enterprise identity are later threat-modeling phases.

## 14. Filesystem Migration

Migration from `.workflow-os/state` must be an explicit later command or API.
It should:

- default to dry-run;
- require a healthy source inspection;
- stop or exclude concurrent writers;
- preserve the source;
- import canonical records deterministically;
- verify counts, hashes, ordering, and referential integrity;
- rebuild projections from authoritative state where possible;
- produce a bounded migration report;
- require explicit activation of the destination backend.

No upgrade should silently move user state.

## 15. Proposed Implementation Sequence

1. **Durable state semantic contract and conformance harness**
   - no database dependency;
   - encode transaction families, conflict classes, ordering, and capability
     requirements;
   - run applicable scenarios against `LocalStateBackend` to document which
     guarantees it can and cannot satisfy.
2. **SQLite embedded adapter**
   - add one justified Rust database dependency;
   - implement schema, transactions, health, and conformance tests;
   - keep opt-in.
3. **Filesystem-to-SQLite migration**
   - explicit dry-run and verified import;
   - no automatic upgrade.
4. **PostgreSQL shared adapter**
   - implement the same semantic contract;
   - add concurrency, restart, and recovery integration tests.
5. **Collaborative state consumers**
   - only after adapter review and operations runbooks;
   - ownership, catalog, and stewardship remain separately scoped.
6. **Hosted and enterprise posture**
   - only after separate product, security, tenancy, and operations decisions.

## 16. Test Plan

The future conformance suite must cover:

- contiguous event append;
- duplicate event ID and sequence rejection;
- concurrent next-event conflict;
- deterministic event reads;
- immutable run-identity mismatch;
- create-only bundle publication;
- missing bundle reference rejection;
- idempotency first-write and replay;
- conflicting idempotency outcome rejection;
- approval decision with fresh matching presentation proof;
- stale or mismatched approval context rejection;
- SideEffect lifecycle compare-and-set;
- atomic SideEffect record and event behavior;
- lease contention, expiry, fencing, and crash recovery;
- deterministic list ordering;
- projection rebuild;
- schema version mismatch;
- interrupted migration;
- corruption detection;
- backup and restore verification;
- redaction-safe errors;
- no secret-like values in Debug or diagnostics.

Backend-specific tests must additionally cover:

- SQLite busy handling, checkpoint posture, multiple connections, crash reopen,
  and unsupported network-filesystem posture where testable;
- PostgreSQL serializable conflicts, deadlock handling, independent workers,
  connection loss, migration locking, backup/restore integration, and
  least-privilege role behavior.

## 17. Operational Readiness Gates

An adapter is not accepted merely because CRUD tests pass. Acceptance requires:

- full common conformance suite;
- crash/restart proof;
- concurrent writer proof;
- migration rehearsal;
- restore rehearsal;
- integrity inspection;
- bounded performance baseline;
- dependency and license review;
- security review;
- operator runbook;
- explicit supported-version policy.

## 18. Open Questions

- Which exact transaction families belong in the first conformance contract?
- Should report artifacts remain in the database or move to a future blob store
  with database metadata and hashes?
- Which canonical records should retain full serialized envelopes versus
  normalized columns?
- What lease duration and fencing model is appropriate for local and shared
  workers?
- Which PostgreSQL versions will be supported?
- Which SQLite version and compile options will be pinned?
- How long must idempotency and audit records be retained?
- Which projections are synchronously required versus asynchronously
  rebuildable?
- What is the first collaborative consumer that proves the shared adapter
  without prematurely creating a hosted product?

These questions should be resolved incrementally through the semantic contract
and adapter reviews, not by expanding this planning phase.

## 19. Final Recommendation

The semantic contract and first bounded SQLite adapter are accepted after
focused review. Plan explicit filesystem-to-SQLite migration separately. Do not
make SQLite a runtime or CLI default, begin PostgreSQL, or claim collaborative
state, managed migration, verified backup/restore, or shared-worker readiness
from this implementation slice.
