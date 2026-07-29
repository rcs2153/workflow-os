# Shared PostgreSQL State Plan Report

Report date: 2026-07-29

## 1. Executive Summary

The Shared PostgreSQL State milestone is now phase-ready. The plan defines one
larger runtime build covering the adapter, managed schema, transaction
families, revisions, fenced leases, shared-worker conformance, one shared run
consumer, projection rebuild, and recovery rehearsal.

No PostgreSQL implementation was added during planning.

## 2. Scope Completed

- Defined the Core/adapter and connection-factory boundaries.
- Defined the managed PostgreSQL schema families and compatibility posture.
- Defined all seven transaction-family behaviors.
- Defined serializable conflict and retry classification.
- Defined CAS revisions and database-time fenced leases.
- Defined one explicit shared run consumer.
- Defined projection rebuild, health, backup, restore, and recovery posture.
- Defined required CI PostgreSQL integration tests.
- Defined privacy, credential, non-goal, and acceptance boundaries.
- Consolidated implementation into one milestone rather than many tiny phases.

## 3. Scope Explicitly Not Completed

Planning added no:

- PostgreSQL dependency, adapter, schema, or migration;
- runtime or CLI backend selection;
- hosted API, worker service, or daemon;
- multi-tenancy, enterprise identity, or stewardship;
- provider mutation expansion;
- public schemas, SDK changes, examples, or release changes;
- production-readiness claim.

## 4. Key Architecture Decisions

- PostgreSQL implements a Core-owned semantic contract.
- Connection establishment is injectable and secret-safe.
- Local/test `NoTls` is explicit and never a production default.
- All transaction families use typed request/result boundaries.
- Serializable/deadlock retry is bounded; domain conflicts are not hidden.
- leases use database time and monotonically increasing fencing tokens.
- CI must execute real PostgreSQL concurrency tests.
- backup/restore is operator-rehearsed, not falsely productized.

## 5. Product Boundary

This milestone establishes shared state for stateless workers. It does not
establish the hosted Workflow OS product, collaboration UI, tenant isolation,
enterprise policy administration, or production operations.

## 6. Validation

Completed successfully:

- `npm run check:docs`;
- `git diff --check`.

## 7. Governed Phase Record

- dogfood workflow: `dg/d`;
- run ID: `run-1785332666615321000-2`;
- approval ID: `approval/run-1785332666615321000-2/planning-approved`;
- presentation ID: `presentation/e48e76580e9bd7c7`;
- approval outcome: granted with persisted presentation proof;
- phase status: completed;
- event summary: 39 events, one approval, zero retries, zero escalations.

Documentation research, editing, validation, and git work occurred outside the
kernel under the approved scope. The kernel coordinated governance and did not
execute those operations.

## 8. Remaining Limitations

- Dependency review is planning-level until the crate is added and audited.
- No PostgreSQL runtime is installed in the current local workspace.
- TLS factory implementation remains a build decision behind the accepted
  injectable boundary.
- Exact backup tooling and supported PostgreSQL major versions remain open.
- Transaction-family API details require implementation review.

## 9. Recommended Next Phase

Proceed directly to the Shared PostgreSQL State implementation milestone on
the current branch. Do not split each internal slice into a separate planning
cycle.
