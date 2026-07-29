# Shared PostgreSQL State Plan

Status: Planning accepted. Implementation is not started.

Related foundations:

- [ADR 0003: Stateless Workers Over Durable State](../adr/0003-stateless-workers-over-durable-state.md)
- [ADR 0012: Compatible SQLite And PostgreSQL Durable State Adapters](../adr/0012-compatible-sqlite-postgresql-durable-state-adapters.md)
- [Durable State Semantic Contract Report](../concepts/DURABLE_STATE_SEMANTIC_CONTRACT_REPORT.md)
- [Operational Embedded Durable State Review](../concepts/OPERATIONAL_EMBEDDED_DURABLE_STATE_REVIEW.md)
- [Roadmap Vertical-Slice Acceleration Plan](roadmap-vertical-slice-acceleration-plan.md)

## 1. Executive Summary

Workflow OS has a complete local opt-in embedded-state slice. The next larger
runtime milestone is a shared PostgreSQL backend that lets multiple stateless
workers coordinate over one durable governance record without weakening the
Core state contract.

This milestone should implement one production-shaped vertical boundary:

- an explicit PostgreSQL adapter;
- managed schema compatibility;
- all accepted Core transaction families;
- compare-and-set revisions;
- expiring fenced leases;
- concurrent-worker conformance;
- projection rebuild and integrity checks;
- backup, restore, and recovery posture;
- one explicit shared run consumer.

The work establishes shared durable state. It does not establish a hosted
product, multi-tenancy, enterprise identity, provider mutation expansion, or
production readiness.

## 2. Goals

- Implement the accepted Core durable-state semantic contract against
  PostgreSQL.
- Preserve authoritative-event, immutable-bundle, approval, SideEffect,
  evidence, report, and projection boundaries.
- Support multiple stateless workers without duplicate authority or stale
  lease holders committing work.
- Make transaction conflicts explicit, bounded, retryable, and observable.
- Prove every claimed transaction family with executable concurrent tests.
- Provide managed schema identity and compatibility checks.
- Provide deterministic projection rebuild and health verification.
- Define an operator-rehearsed backup, restore, and recovery boundary.
- Exercise one existing run path through explicit shared-state selection.
- Keep connection credentials and database diagnostics out of Debug, events,
  reports, and errors.

## 3. Non-Goals

This milestone does not authorize:

- a hosted API or control plane;
- multi-tenancy, row-level tenant isolation, or organization administration;
- OIDC, RBAC, IdP integration, or enterprise stewardship;
- automatic migration from filesystem or SQLite state;
- automatic PostgreSQL selection for local users;
- provider mutation expansion;
- new workflow schemas, SDK surfaces, examples, or domain packs;
- a general job queue or replacement for PostgreSQL;
- distributed transactions across PostgreSQL and external providers;
- exactly-once provider execution;
- cross-region replication or high-availability claims;
- production SLOs or a production-readiness declaration.

## 4. Existing Contract Boundary

The Core semantic contract owns:

- ordered event append;
- immutable run identity;
- create-only idempotency;
- explicit transaction-family support;
- deterministic reads;
- conflict classification;
- revisions;
- lease semantics;
- schema posture;
- health posture.

PostgreSQL may implement these semantics with MVCC, constraints, row locks,
sequences, and serializable transactions. Those mechanisms remain adapter
details. Domain behavior must not become PostgreSQL-specific.

Authoritative records remain:

- workflow events for run state;
- immutable run bundles for resolved run inputs;
- approval decisions and presentation proof for authority;
- SideEffect records for proposed and observed external effects.

Snapshots, audit projections, discovery records, telemetry, and WorkReports
remain projections or governed handoff artifacts.

## 5. Dependency And Runtime Boundary

Use the maintained synchronous `postgres` crate because the existing Core
store interfaces are synchronous. The first implementation should pin and
review the selected release and enable only required type features.

Do not embed a raw connection URL in `PostgresStateBackend`. Add an explicit
connection-factory boundary that returns a connected `postgres::Client`.
Connection establishment, credential retrieval, TLS roots, channel binding,
timeouts, and future pooling remain outside persisted backend state.

The adapter may provide a clearly named local/test `NoTls` factory for
loopback CI. It must not present `NoTls` as the production default. A caller
must opt into the connection factory explicitly.

`Debug` for the backend and factory must omit:

- host, port, database, and user;
- connection strings;
- TLS material;
- passwords, tokens, and secret references.

Connection pooling is not required to prove the first correctness slice.
Opening one bounded connection per operation is acceptable initially. Pooling
may be added later behind the same factory after correctness and recycling
semantics are reviewed.

## 6. Managed Schema

Schema version one should use an explicit Workflow OS namespace and contain:

- schema metadata and migration history;
- run events and run snapshots;
- idempotency intent and result records;
- approval decisions and approval-presentation records;
- projects and policy audit records;
- adapter audit and observability records;
- WorkReport artifacts;
- SideEffect records;
- immutable run manifests and canonical definition records;
- revisioned authoritative records;
- worker leases and fencing-token sequence;
- projection checkpoints and rebuild metadata.

Canonical validated JSON remains the compatibility envelope. Relational
columns provide identity, ordering, revision, lookup, foreign-key, and lease
constraints. They must be checked against canonical payload identity on reads
and health checks.

Schema initialization and upgrades must:

- run under one database-scoped migration lock;
- record adapter version, schema version, checksum, and state;
- be transactional where PostgreSQL permits;
- reject newer or checksum-mismatched schemas;
- leave interrupted or incomplete migration in recovery-required posture;
- never auto-downgrade.

## 7. Transaction Families

The milestone must implement and prove all seven Core transaction families.

### 7.1 Append Run Event

Atomically validate immutable run identity, enforce contiguous sequence and
unique event identity, append the event, and update the corresponding snapshot
or projection checkpoint.

### 7.2 Reserve Idempotency And Pre-Effect Intent

Atomically reserve an idempotency key and persist the proposed or attempted
SideEffect plus its authoritative event before any external provider call.
Concurrent workers must produce one reservation owner.

### 7.3 Record External Outcome

After the provider call, atomically record the bounded observed outcome,
SideEffect transition, authoritative event, and reconciliation posture.
Ambiguous outcomes remain explicit and cannot be converted to success by
retry.

### 7.4 Record Approval Decision

Atomically validate current resolved-context commitment and presentation proof,
record the approval decision, append its event, and advance the run only when
the decision permits it.

### 7.5 Transition SideEffect

Atomically enforce the accepted SideEffect lifecycle, expected revision,
approval linkage where required, and corresponding authoritative workflow
event.

### 7.6 Publish Immutable Run Bundle

Resolve all referenced canonical records, publish create-only records, and
commit one run manifest only after every reference is available and validated.
Exact retries are idempotent; rebinding fails closed.

### 7.7 Commit Authoritative Result Before Projection

Commit the authoritative record and its event before or with the dependent
projection. Projection failure must be recoverable through deterministic
rebuild and must not erase the authoritative commit.

Each family needs a Core request/result type. Do not expose a generic SQL
transaction escape hatch.

## 8. Isolation, Conflict, And Retry Policy

Use serializable transactions for mutation families whose correctness depends
on a multi-read decision. Use explicit row locks and unique constraints where
they provide a smaller deterministic boundary.

The adapter must classify:

- serialization failure;
- deadlock;
- unique/identity conflict;
- stale revision;
- lease contention;
- stale fencing token;
- schema incompatibility;
- unavailable connection;
- corrupt or identity-mismatched record.

Only serialization and deadlock conflicts are retryable automatically, with a
small bounded attempt count and deterministic backoff posture. The complete
transaction is retried from the beginning. Domain conflicts, stale revisions,
stale fences, and ambiguous provider outcomes are not hidden retries.

Errors must use stable Workflow OS codes and omit SQL, relation names when
sensitive, connection details, canonical payloads, and caller values.

## 9. Compare-And-Set Revisions

Revisioned records use a nonzero monotonically increasing
`DurableRevision`. Mutation requires the exact expected revision and returns
the committed next revision.

At minimum, CAS must cover:

- run snapshot/projection replacement;
- approval state where mutation is allowed;
- SideEffect lifecycle records;
- project/catalog records used by the shared consumer;
- lease ownership metadata.

No caller may perform an unconditional overwrite of an existing revisioned
authoritative record.

## 10. Expiring Fenced Leases

PostgreSQL worker leases must include:

- bounded lease key;
- owner actor;
- lease generation;
- database-issued fencing token;
- acquired-at and expires-at database timestamps;
- last renewal time;
- optional bounded work identity.

Acquisition and renewal use database time, not worker wall clocks. An expired
lease may be taken over with a strictly greater fencing token. Every guarded
commit must include the active token and reject stale holders even if they
continue running after expiry.

Session advisory locks may coordinate schema work, but they are not sufficient
as durable worker leases because they do not provide durable expiry and fencing
semantics.

## 11. Shared-Worker Consumer

Add one explicit opt-in shared run consumer that:

1. acquires a fenced lease for one run or schedulable unit;
2. reads the exact immutable run bundle and current authoritative state;
3. executes one existing local/mock-safe runtime step through current
   interfaces;
4. commits the resulting authoritative transition with the fence;
5. releases or allows expiry of the lease;
6. leaves complete bounded events and projection state.

The first consumer proves state coordination, not hosted execution. It must not
call live providers, add a daemon, expose a remote API, or become the default
local executor.

## 12. Projection Rebuild And Integrity

Provide an explicit read-only plan and mutating rebuild operation for derived
run snapshots and selected shared projections.

Rebuild must:

- read authoritative records in deterministic order;
- validate immutable identity and canonical payload identity;
- write projections with expected revisions;
- record source checkpoint and aggregate digest;
- be idempotent for unchanged authority;
- fail on gaps, duplicate authority, or unsupported versions;
- never rewrite authoritative history.

Backend health must check PostgreSQL connectivity, schema metadata, canonical
identity, foreign keys, event continuity, lease invariants, and projection
checkpoint posture without returning raw stored values.

## 13. Backup, Restore, And Recovery

The first milestone does not implement a backup engine. It must define and
rehearse an operator contract using maintained PostgreSQL tools:

- consistent logical backup of the Workflow OS namespace;
- restore into a separate destination;
- schema and health verification;
- authoritative count and canonical digest comparison;
- projection rebuild;
- explicit destination activation;
- source retention and rollback posture.

The repository should provide a runbook and CI/local rehearsal script that
accept explicit connection references. It must not persist credentials or
pretend that one successful logical restore proves high availability,
point-in-time recovery, or production disaster recovery.

## 14. Test Infrastructure

Do not require Docker libraries in Core production dependencies.

Add a PostgreSQL integration-test binary that reads one explicit test
connection reference. Ordinary local workspace tests may skip only when the
reference is absent and the required-test flag is false. CI must start the
official PostgreSQL service container, set the required-test flag, and fail if
the database tests do not run.

The shared-state suite must prove:

- all common conformance scenarios;
- every claimed transaction family;
- concurrent next-event writers;
- one-winner idempotency reservation;
- approval decision race;
- SideEffect transition race;
- immutable-bundle publication race;
- CAS success and stale-revision rejection;
- lease contention, renewal, expiry, takeover, and stale-fence rejection;
- worker crash/connection loss followed by safe takeover;
- serialization and deadlock retry classification;
- deterministic projection rebuild;
- schema initialization, compatibility, and interrupted-migration posture;
- backup/restore rehearsal and canonical verification;
- connection, path, payload, SQL, and secret non-leakage.

## 15. Privacy And Credential Posture

Database credentials must enter through an explicit caller-owned connection
factory or secret-reference resolution boundary. They must never be stored in:

- workflow specifications;
- runtime events;
- audit records;
- evidence references;
- WorkReports;
- migration receipts;
- test fixtures committed to the repository.

SQL statement logging is disabled by default at Workflow OS boundaries.
Database/server logging remains an operator responsibility and must be called
out in the runbook because canonical records may be sensitive even when they
contain only bounded governed data.

## 16. Implementation Sequence

This is one milestone with internal implementation slices, not a sequence of
standalone planning phases.

1. Add the reviewed connection-factory boundary and PostgreSQL dependency.
2. Implement managed schema and existing store traits.
3. Add Core transaction-family request/result APIs and PostgreSQL
   implementations.
4. Add CAS revisions and expiring fenced leases.
5. Expand executable shared-worker conformance.
6. Add the explicit shared run consumer.
7. Add projection rebuild, health, and backup/restore rehearsal.
8. Run one phase-level security/correctness review and fix blockers before
   merge.

Intermediate commits may be used for reviewability. The milestone should not
return to separate plan/report/review cycles for every internal slice.

## 17. Validation

The implementation phase must run:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- required PostgreSQL integration and concurrent-worker tests;
- schema/restore rehearsal;
- `npm run check`;
- `npm run check:integrations`;
- `npm run check:docs`;
- `cargo audit`;
- `npm audit --audit-level=moderate`;
- `git diff --check`.

## 18. Acceptance Criteria

- `PostgresStateBackend` implements existing store interfaces.
- The backend claims only executable conformance it passes.
- All seven transaction families have explicit APIs and concurrent tests.
- CAS revisions reject stale writers.
- expired lease takeover issues a greater fence and stale fences cannot commit.
- one explicit shared consumer coordinates safely across worker processes.
- schema version and compatibility checks fail closed.
- projection rebuild is deterministic and authority-preserving.
- backup/restore rehearsal verifies canonical authority and projections.
- connection data, SQL, payloads, and secrets do not leak.
- existing filesystem and SQLite behavior remains unchanged.
- no hosted, multi-tenant, provider-expansion, or production-readiness claim is
  introduced.

## 19. Open Questions

- Which current project/catalog record should be the first revisioned shared
  collaboration consumer after the run consumer?
- Should transaction retries live entirely inside the adapter or accept a
  caller-supplied bounded retry policy?
- Which PostgreSQL major versions should CI support initially?
- Should immutable canonical records share one table or retain family-specific
  tables with common constraints?
- Which backup tool versions and server settings are required for a credible
  first rehearsal?
- When should a reviewed TLS connection factory move into a runtime-facing
  crate rather than Core?

## 20. Final Recommendation

Proceed directly to the Shared PostgreSQL State implementation milestone.

Begin with the connection factory, schema, and executable integration-test
environment, then continue through transaction families, revisions, leases,
the shared consumer, and recovery proof on the same governed branch. Do not
broaden provider mutations or begin hosted/collaborative product surfaces
before this milestone passes its phase-level review.

## 21. Reference Sources

- PostgreSQL transaction isolation and serialization failure handling:
  <https://www.postgresql.org/docs/current/transaction-iso.html>
- PostgreSQL explicit and advisory locking:
  <https://www.postgresql.org/docs/current/explicit-locking.html>
- Rust `postgres` synchronous client:
  <https://docs.rs/postgres/latest/postgres/>
