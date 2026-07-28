# Open-Source Durable Store Selection Plan Report

Report date: 2026-07-28

## 1. Executive Summary

The open-source durable-store planning phase is complete and accepted after
focused maintainer review.

The phase recommends compatible adapters:

- SQLite for embedded local durable state;
- PostgreSQL for shared collaborative durable state;
- one Core-owned semantic contract and backend conformance suite.

No database dependency or runtime behavior was added.

## 2. Scope Completed

- Inspected existing Core store interfaces and local filesystem behavior.
- Identified missing cross-record transaction, revision, conflict, migration,
  and conformance semantics.
- Evaluated SQLite, PostgreSQL, and FoundationDB as open-source candidates and
  CockroachDB as an excluded source-available comparator against roadmap
  requirements.
- Distinguished embedded local state from shared collaborative state.
- Accepted ADR 0012 after focused review.
- Defined a phased implementation and test strategy.

## 3. Scope Explicitly Not Completed

The phase did not add:

- database dependencies;
- database schemas;
- SQLite or PostgreSQL adapters;
- migrations;
- automatic user-state conversion;
- hosted or collaborative runtime behavior;
- enterprise administration;
- provider mutation expansion;
- OpenShell integration;
- workflow schema or SDK changes;
- release posture changes.

## 4. Current-State Findings

Workflow OS has mature individual store contracts for events, snapshots,
idempotency, locks, approvals, presentation proof, project metadata, audit,
telemetry, report artifacts, and SideEffects. Immutable run bundles use a
separate create-only store.

The current filesystem backend provides valuable local validation and durable
records, but it cannot serve as the eventual multi-host collaboration
substrate. Existing traits do not fully specify the cross-record transactional
units required for authority, approval, SideEffect, event, and idempotency
integrity.

## 5. Candidate Decision

SQLite is selected for the embedded role because it preserves low-friction
local-first use while adding transactional storage, recovery, inspection, and
backup support.

PostgreSQL is selected for the shared role because it provides mature
transactions, concurrency, locking, migration, recovery, observability, and
self-hosting support.

FoundationDB was not selected because its low-level layer and operational model
would move substantial database design into Workflow OS. Current CockroachDB
releases were excluded from the open-source candidate set because they use the
CockroachDB Software License; their distributed operational complexity is also
not justified before the PostgreSQL path is proven.

## 6. Architecture Summary

Core owns storage semantics. Adapters own physical persistence.

The plan rejects:

- PostgreSQL as a mandatory local dependency;
- SQLite as a multi-host shared database;
- a bespoke Workflow OS database;
- continued Git/filesystem use as the eventual collaboration backend.

## 7. Recommended First Implementation

Implement a durable-state semantic contract and backend conformance harness
without adding a database dependency.

That phase should define:

- transactional mutation families;
- conflict and retry categories;
- revisions and compare-and-set;
- lease/fencing semantics;
- deterministic ordering;
- migration metadata;
- backend capabilities;
- executable conformance scenarios.

## 8. Privacy And Security

The decision preserves current data-minimization and redaction boundaries.
Database credentials, raw provider payloads, raw source contents, unrestricted
command output, and secret-like metadata remain excluded.

Hosted tenancy, encryption, enterprise identity, row-level security, and key
management require separate threat models.

## 9. Evidence Reviewed

- `docs/ENGINEERING_STANDARD.md`
- ADR 0003 and ADR 0004
- current `StateBackend` and store traits
- `LocalStateBackend`
- `LocalImmutableRunBundleStore`
- current roadmap sequencing
- official SQLite, PostgreSQL, FoundationDB, and CockroachDB documentation

## 10. Validation

| Command | Result |
| --- | --- |
| `npm run check:docs` with pinned Node 20 | Passed |
| `git diff --check` | Passed |
| Current store-interface inspection | Passed |
| Primary-source database documentation review | Completed |

## 11. Remaining Limitations

- No conformance harness exists yet.
- No cross-store transactional API exists yet.
- No database adapter exists.
- No migration path exists.
- PostgreSQL operations and tenancy are not designed.
- SQLite durability settings and supported versions are not pinned.
- Collaborative consumers remain future work.

## 12. Recommended Next Phase

Implement the durable-state semantic contract and backend conformance harness
only. Do not begin with SQL schema or adapter code.

## 13. Governed Planning Record

- workflow ID: `dg/d`
- run ID: `run-1785222620853198000-2`
- approval ID:
  `approval/run-1785222620853198000-2/planning-approved`
- presentation ID: `presentation/0baa372fde1e01e8`
- approval outcome: granted
- terminal status: completed
- events: 39
- retries: 0
- escalations: 0
- out-of-kernel work: source inspection, primary-source database research,
  documentation edits, and validation commands
