# Shared PostgreSQL State Report

Report date: 2026-07-29

## 1. Executive Summary

Workflow OS now has an explicit opt-in `PostgreSQL` shared-state backend that
implements the accepted Core durable-state semantic contract for concurrent
stateless workers.

The milestone adds managed schema identity, all seven Core transaction
families, compare-and-set revisions, expiring fenced leases, one shared
run-event consumer, deterministic projection rebuild, concurrent conformance,
and a logical backup/restore integrity rehearsal.

This is a shared-state correctness slice, not a hosted product or production
database declaration.

## 2. Scope Completed

- Added the synchronous `postgres` client dependency behind Core store traits.
- Added a caller-owned connection-factory boundary.
- Added an explicitly named local/test-only `NoTls` factory.
- Implemented existing durable state stores over `PostgresStateBackend`.
- Implemented all seven accepted transaction-family APIs.
- Added serializable transaction retries for serialization and deadlock
  conflicts.
- Added revisioned records and stale-writer rejection.
- Added database-time expiring leases and fencing tokens.
- Added an explicit shared run-event consumer.
- Added immutable run-bundle publication and verified reads.
- Added projection planning, rebuild, health, and schema compatibility posture.
- Added `PostgreSQL` 17 CI conformance and logical backup/restore rehearsal.
- Bound each pre-effect reservation key exactly to both the persisted
  `SideEffect` idempotency binding and its authoritative event before opening a
  database connection.

## 3. Scope Explicitly Not Completed

The phase did not add:

- a hosted API, daemon, scheduler, or automatic distributed worker service;
- multi-tenancy, tenant isolation, enterprise identity, or administration;
- automatic backend selection or filesystem/SQLite migration;
- production TLS, pooling, replication, high availability, point-in-time
  recovery, capacity proof, or SLOs;
- a general queue or Redis, SQS, or NATS integration;
- provider mutation expansion;
- workflow schemas, SDK fields, examples, or domain packs;
- a production-readiness claim.

## 4. Backend And Connection API

`PostgresStateBackend` accepts an `Arc<dyn PostgresConnectionFactory>`. The
factory owns credential retrieval, TLS, timeouts, and future pooling. Backend
state and `Debug` output contain no connection URL.

`PostgresNoTlsConnectionFactory` exists only for explicit local and CI use.
Its `Debug` output redacts connection configuration, and documentation does
not present it as a production transport.

The first implementation opens one bounded connection per operation. This is
acceptable for correctness proof and deliberately leaves pooling semantics
outside Core.

## 5. Schema And Store Coverage

The managed `workflow_os` schema stores:

- append-only run events;
- revisioned records for snapshots, approvals, approval presentation proof,
  projects, policy audit, adapter telemetry, WorkReport artifacts, and
  SideEffects;
- idempotency reservations;
- local lock compatibility records;
- expiring fenced worker leases;
- content-addressed immutable definitions and run manifests;
- schema version, checksum, and recovery-required posture.

Initialization runs under a transaction-scoped advisory migration lock.
Newer, checksum-mismatched, and recovery-required schema posture fails closed.
Reads validate canonical payload identity against relational identity.

## 6. Transaction Families

The backend implements the seven Core families:

1. ordered run-event append;
2. idempotency reservation plus pre-effect intent;
3. external outcome plus SideEffect transition;
4. approval decision plus presentation proof and event;
5. SideEffect lifecycle transition plus event;
6. immutable run-bundle publication;
7. authoritative event plus snapshot projection.

Approval decisions lock the pending projection, require an undecided durable
request, validate the supplied presentation against that request, require the
same durable presentation record, require a matching proof marker, append the
decision event, and update the approval by expected revision in one
transaction.

External provider mutation still occurs outside a database transaction.
Workflow OS records intent before the call and observed outcome afterward; it
does not claim distributed rollback or exactly-once provider execution.

## 7. Concurrency, Revisions, And Retry

Mutating multi-record operations use serializable transactions. Serialization
and deadlock conflicts are retried from the full transaction boundary with a
bounded attempt count. Domain conflicts, stale revisions, stale fences, and
identity mismatch are not hidden retries.

Executable tests race:

- competing next-event append;
- equal idempotency intent reservation;
- approval decision;
- SideEffect transition;
- immutable run-bundle publication.

The accepted result is one authoritative winner, deterministic replay where
defined, and fail-closed conflict otherwise.

The pre-effect intent boundary also rejects a reservation unless the request
key, `SideEffect` idempotency binding, and event idempotency key are identical.
This prevents a transaction from recording an event under idempotency context
that differs from the durable intent.

## 8. Lease And Shared Consumer Behavior

Leases use database time, bounded TTL, an owner, and a monotonically increasing
fencing token. Live competing owners are rejected. Renewal advances the token.
Expired lease takeover advances it again, and stale or expired holders cannot
commit an authoritative event or projection.

The explicit shared consumer acquires a run lease, validates current event
history, rehydrates the run with one supplied event, commits the event and
snapshot under the active fence, and releases the lease after success.

If the operation fails before release, the lease is allowed to expire. The
consumer is a library API, not an automatically running worker.

## 9. Integrity And Recovery

Projection planning discovers authoritative run streams in deterministic
order. Rebuild rehydrates each run from events and updates snapshots with
expected revisions. It never rewrites event history.

The recovery script creates a protected logical dump, restores an isolated
database, verifies schema health, rebuilds projections, verifies record counts
through the rebuild plan, and reads a restored immutable run bundle. It
removes the temporary dump on exit and does not print connection references.

This proves a bounded CI recovery rehearsal, not production disaster recovery.

## 10. Privacy And Error Posture

Errors use stable codes and do not include SQL, connection configuration,
stored canonical payloads, raw idempotency keys, paths, provider data,
credentials, or secret-like test values.

Focused corruption tests place secret-like values in event payload and schema
metadata, then verify deserialization and compatibility errors do not echo
them. Backend and connection-factory `Debug` output is redaction-safe.

## 11. Test Coverage

Focused coverage includes:

- existing store interface behavior;
- ordered and competing event append;
- idempotent replay and concurrent reservation;
- approval presentation/decision atomicity and decision race;
- SideEffect intent, attempt, outcome, stale revision, and transition race;
- immutable bundle publication race and exact verified read;
- lease contention, renewal, expiry, takeover, and stale-fence rejection;
- shared consumer commit;
- deterministic projection rebuild;
- schema mismatch and recovery-required failure;
- corrupt-payload and secret non-leakage;
- restored database integrity.

Ordinary local runs skip live database operations only when the explicit URL is
absent and the required-test flag is not set. CI sets the required flag.

## 12. Commands And Results

Completed locally:

- focused request-validation regressions proving invalid lease TTL and
  mismatched intent idempotency fail before connection;
- focused `PostgreSQL` test compile;
- focused strict Clippy;
- focused no-database skip-path tests;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check`;
- `npm run check:integrations`;
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`;
- `cargo metadata --locked --format-version 1`;
- `cargo audit`;
- `npm audit --audit-level=moderate`;
- `git diff --check`.

Completed in GitHub Actions CI run 949:

- `Shared PostgreSQL State / PostgreSQL state conformance`: passed against
  `PostgreSQL` 17.5;
- concurrent event, idempotency, approval, SideEffect, and immutable-bundle
  races: passed;
- fenced lease contention, expiry, takeover, and stale commit checks: passed;
- `Shared PostgreSQL State / PostgreSQL backup and restore rehearsal`: passed
  using the matching `PostgreSQL` 17 client toolchain;
- isolated restore, schema health, projection rebuild, and immutable run-bundle
  read: passed.

## 13. Governed Phase Record

- dogfood workflow: `dg/implement`;
- run ID: `run-1785333001187423000-2`;
- approval ID:
  `approval/run-1785333001187423000-2/implementation-approved`;
- presentation ID: `presentation/61b58d2267eaf6f9`;
- approval outcome: granted with persisted presentation proof;
- phase status: `Completed`;
- event summary: 39 events, one approval, zero retries, zero
  escalations.

Repository edits, shell commands, tests, documentation, and git work occurred
outside the kernel under the approved scope. The kernel coordinated governance;
it did not execute those operations. Local live-database behavior was skipped
because no PostgreSQL service was available. GitHub Actions CI run 949 supplied
the mandatory `PostgreSQL` 17.5 service and completed the executable conformance
and recovery proof.

## 14. Remaining Known Limitations

- Connection pooling and production TLS factories are caller concerns.
- Schema version one has no nontrivial forward migration yet.
- Lease failure relies on expiry rather than an out-of-band revocation channel.
- The shared consumer handles one explicit supplied event, not scheduling.
- Backup/restore uses logical tools and one CI server version.
- Performance, capacity, replication, failover, and PITR are unproven.
- No runtime configuration or workflow schema selects this backend.

## 15. Recommended Next Phase

Proceed to **single-tenant hosted alpha planning** without broadening provider
mutation families first.

That planning must keep hosted API, worker lifecycle, credential delivery,
observability, recovery, and deployment boundaries explicit. It must not infer
multi-tenancy or enterprise readiness from this shared-state slice.
