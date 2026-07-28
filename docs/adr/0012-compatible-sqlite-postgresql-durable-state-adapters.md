# ADR 0012: Compatible SQLite And PostgreSQL Durable State Adapters

## Status

Accepted

## Acceptance Note

Accepted after focused review in
[Open-Source Durable Store Selection Plan Review](../concepts/OPEN_SOURCE_DURABLE_STORE_SELECTION_PLAN_REVIEW.md).

The review corrected two architecture claims before acceptance:

- current CockroachDB releases are treated as an excluded source-available
  comparator, not an open-source candidate;
- database transactions cannot make an external provider mutation atomic with
  Workflow OS state, so the contract requires durable pre-effect intent and
  post-effect outcome/reconciliation boundaries.

Acceptance authorizes only the durable-state semantic contract and backend
conformance harness as the next implementation phase. It does not authorize a
database dependency or adapter.

## Context

Workflow OS is local-first today. Its `LocalStateBackend` stores run events,
snapshots, idempotency results, approvals, approval-presentation proof records,
policy audit records, adapter telemetry, report artifacts, and SideEffect
records as local files. Immutable run bundles use a separate create-only local
store.

That backend is useful for one operator and honest preview dogfooding, but it is
not the eventual collaboration substrate. Collaborative workers need
transactional ordering, conflict detection, atomic authority-to-effect
boundaries, migrations, recovery, backup, and inspectability that are difficult
to provide safely through independent filesystem records.

The existing Rust store traits are the correct architectural direction, but
they currently specify individual operations more precisely than
cross-operation transactions. Selecting a database without first specifying
those semantic units would move correctness assumptions into one adapter.

The local-first product boundary creates two materially different deployment
needs:

- an embedded store that requires no daemon for one user on one machine; and
- a shared store that supports concurrent stateless workers and collaborating
  users across machines.

One database could technically be forced into both roles, but doing so would
either weaken local-first adoption or overstate an embedded database's
collaborative guarantees.

## Decision

Workflow OS will use compatible durable-state adapters rather than one physical
database for every deployment posture:

- **SQLite** is the selected embedded local durable store.
- **PostgreSQL** is the selected shared collaborative durable store.
- Existing filesystem state remains a preview/local compatibility backend until
  an explicit migration phase is implemented.

Both database adapters must implement one Core-owned semantic contract and pass
the same backend conformance suite. Workflow OS domain semantics must not
depend on SQLite- or PostgreSQL-specific behavior.

Before either database adapter is implemented, Core must define:

- authoritative versus projection records;
- append-only ordered event semantics;
- create-only immutable record semantics;
- idempotency reservation and replay semantics;
- compare-and-set revision semantics;
- lock or lease ownership and expiry semantics;
- required transactional mutation families;
- conflict and retry classifications;
- deterministic read ordering;
- schema and migration metadata;
- health, integrity, backup, restore, and recovery expectations.

The contract must identify which operations require one transaction. At
minimum, it must cover:

- event append with sequence and immutable run-identity validation;
- event append with any authoritative transition record that would otherwise
  permit an authority-to-effect split;
- idempotency reservation with durable pre-effect intent, and a separate
  transaction for the observed outcome after an external operation;
- approval decision with presentation-proof and current-context validation;
- SideEffect lifecycle transition with its authoritative workflow event;
- immutable run-bundle publication after all referenced records resolve;
- projection replacement only when the corresponding authoritative commit is
  durable.

No database can atomically commit Workflow OS state and an external provider
mutation. Provider execution must therefore use a durable protocol:

1. transactionally reserve idempotency and record proposed/attempted intent;
2. perform the external operation outside the database transaction;
3. transactionally record the observed outcome and authoritative event;
4. reconcile ambiguous outcomes before retry.

The database adapter must not claim distributed rollback or exactly-once
provider execution.

PostgreSQL must use an isolation and retry strategy that preserves these
contracts under concurrent workers. SQLite must use transactions and a
durability configuration appropriate for local governed state. Adapter-specific
optimizations may not weaken the common contract.

## Why SQLite For Embedded Local State

SQLite provides transactions, crash recovery, a stable file format, mature
backup APIs, broad platform support, and low operational friction. WAL mode
allows readers and a writer to operate concurrently on one host.

Its limits are also explicit and compatible with the intended role:

- WAL requires processes to share one host and does not work over a network
  filesystem;
- WAL has one writer at a time;
- checkpointing and `SQLITE_BUSY` handling require deliberate configuration;
- durability depends on synchronous and checkpoint settings;
- the runtime must pin a SQLite version containing applicable corruption fixes.

These boundaries make SQLite an embedded local store, not the shared
collaboration store.

## Why PostgreSQL For Shared Collaborative State

PostgreSQL provides mature ACID transactions, MVCC, serializable isolation,
row and advisory locking, constraints, indexes, schema migrations, backup,
point-in-time recovery, observability, and a large self-hosting ecosystem.

It is appropriate for concurrent stateless workers and shared workflow/catalog
state. It is not the default local store because requiring a service or
container would violate the low-friction local-first posture.

PostgreSQL compatibility does not authorize a hosted Workflow OS service. A
self-hosted adapter and a hosted control plane remain separate product phases.

## Common Semantic Contract

The contract is stronger than method signatures. It must define observable
behavior for:

- duplicate IDs and idempotent replays;
- non-contiguous or conflicting event sequence numbers;
- immutable identity mismatch;
- concurrent writers;
- transaction retries;
- stale leases and lease takeover;
- corrupted or partially migrated data;
- projection rebuild;
- deterministic list order;
- schema-version compatibility;
- interrupted backup, restore, and migration;
- redaction-safe errors and diagnostics.

Adapters must expose typed capabilities and health posture. A backend that
cannot satisfy a required transaction or durability guarantee must fail closed;
it must not silently downgrade.

## Source-Of-Truth Boundaries

- Workflow events remain authoritative for run state.
- Immutable run bundles remain authoritative for the exact authored and
  resolved inputs of a run.
- SideEffect records remain authoritative for SideEffect intent and lifecycle.
- Approval decisions and presentation proof retain their existing authority
  boundaries.
- Audit, telemetry, snapshots, discovery records, and reports remain
  projections or governed handoff artifacts as currently documented.
- Git remains appropriate for authored workflow specifications; it is not the
  runtime collaboration database.

Database schemas must preserve these distinctions. A convenient relational
join must not turn a projection into authority.

## Migration And Compatibility

No automatic migration is authorized by this ADR.

A future migration phase must provide:

- explicit source and destination selection;
- preflight health and schema checks;
- a consistent read boundary or stopped-writer requirement;
- deterministic import order;
- content and count verification;
- event replay and projection reconciliation;
- resumable or restart-safe execution;
- a dry run;
- a rollback or source-preservation posture;
- a bounded migration report.

The filesystem backend must not be silently converted when a user upgrades the
CLI.

## Security And Privacy

Database adoption does not relax existing redaction rules. Durable stores must
not add raw provider payloads, credentials, authorization headers, environment
values, unrestricted command output, raw source content, or secret-like
metadata to governed records.

Connection credentials must be supplied through existing secret-reference
boundaries. Database errors, migration errors, diagnostics, query logging, and
Debug output must not leak connection strings or stored sensitive values.

Encryption at rest, tenant isolation, enterprise identity, row-level security,
and hosted key management require separate threat models. They are not implied
by selecting a database.

## Consequences

Positive:

- local use remains installation-light;
- collaborative state gets a mature transactional database;
- Core semantics remain portable and testable;
- the project avoids inventing a database;
- migration can be explicit rather than hidden;
- SQLite and PostgreSQL adapters can share one behavioral conformance suite.

Tradeoffs:

- two adapters increase testing and migration work;
- the common contract must avoid a lowest-common-denominator design;
- SQLite and PostgreSQL need different operational guidance;
- filesystem compatibility remains until users have an explicit migration
  path;
- transaction boundaries may require changes to current store interfaces before
  either adapter can be correct.

## Alternatives Considered

### PostgreSQL Everywhere

Rejected as the default local posture. It could satisfy both roles
functionally, but requiring a service or container for first-run local
governance would weaken the local-first product.

### SQLite Everywhere

Rejected for shared collaboration. SQLite WAL is same-host and single-writer;
placing the database on a network filesystem is not a supported collaboration
architecture.

### FoundationDB

Not selected. Its strict-serializable transactions and ordered key-value model
are strong, but Workflow OS would need to build and operate more indexing,
schema, access-control, migration, and inspection machinery. FoundationDB also
documents five-second transaction and value-size limits and does not provide a
user-level access-control boundary. Those tradeoffs are disproportionate for
the first shared backend.

### CockroachDB

Excluded from the open-source candidate set for current releases. Its
serializable distributed SQL model is technically attractive, but current
releases use the CockroachDB Software License rather than an OSI open-source
license and introduce distributed operational complexity unnecessary before
Workflow OS demonstrates the PostgreSQL collaborative path.

### A Bespoke Workflow OS Database

Rejected. Database implementation and replication are outside the Core product
boundary and would divert effort from governance semantics.

### Continue With Files And Git

Rejected as the eventual collaboration architecture. Independent files do not
provide the cross-record transactions, concurrent-worker conflict handling, or
operational recovery needed for shared governed state.

## Evidence Sources

- [SQLite write-ahead logging](https://www.sqlite.org/wal.html)
- [SQLite atomic commit](https://www.sqlite.org/atomiccommit.html)
- [SQLite backup API](https://www.sqlite.org/backup.html)
- [PostgreSQL concurrency control](https://www.postgresql.org/docs/current/mvcc.html)
- [PostgreSQL transaction isolation](https://www.postgresql.org/docs/current/transaction-iso.html)
- [PostgreSQL explicit locking](https://www.postgresql.org/docs/current/explicit-locking.html)
- [PostgreSQL point-in-time recovery](https://www.postgresql.org/docs/current/continuous-archiving.html)
- [FoundationDB transaction model](https://apple.github.io/foundationdb/developer-guide.html)
- [FoundationDB known limitations](https://apple.github.io/foundationdb/known-limitations.html)
- [CockroachDB release licensing](https://www.cockroachlabs.com/docs/releases)

## Implementation Timing

The first implementation phase after ADR acceptance should define the durable
state semantic contract and backend conformance harness only. It should not add
a database dependency.

Later separately reviewed phases may add:

1. the SQLite embedded adapter;
2. explicit filesystem-to-SQLite migration tooling;
3. the PostgreSQL shared adapter;
4. collaborative workflow/catalog consumers;
5. production operations and hosted deployment posture.

## Explicit Implementation Statement

This ADR does not add a database dependency, adapter implementation, schema,
migration, hosted service, collaborative UI, enterprise administration,
provider mutation, OpenShell integration, or release posture change.
