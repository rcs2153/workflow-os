# Shared PostgreSQL State Plan Review

Review date: 2026-07-29

## 1. Executive Verdict

**Plan accepted; proceed to Shared PostgreSQL State implementation.**

The plan is sufficiently concrete to begin one larger vertical build. It
preserves the Core semantic contract, requires executable concurrency proof,
and avoids turning PostgreSQL selection into hosted or production claims.

## 2. Scope Assessment

The plan stays within shared durable state. It does not authorize hosted APIs,
multi-tenancy, identity, enterprise administration, provider expansion,
schemas, SDKs, examples, or release changes.

## 3. Architecture Assessment

The injectable connection factory is the correct boundary. It prevents raw
connection strings and local `NoTls` behavior from becoming stored adapter
state or an accidental production default.

Canonical JSON plus constrained relational identity remains compatible with
the accepted SQLite boundary while PostgreSQL-specific mechanisms stay behind
the adapter.

## 4. Transaction Assessment

All seven accepted transaction families have explicit intended behavior. The
plan correctly rejects a generic SQL transaction escape hatch and recognizes
that PostgreSQL cannot atomically commit external provider operations.

Serializable transactions, row constraints, explicit conflict
classification, and whole-transaction retries are appropriate. Retry must
remain bounded and must not absorb stale revisions, stale fences, or ambiguous
external outcomes.

## 5. Revision And Lease Assessment

Expected-revision CAS prevents silent shared overwrites. Database-time expiry
and monotonically increasing fencing tokens are required for safe worker
takeover.

The plan correctly distinguishes session advisory locks used for schema
coordination from durable worker leases.

## 6. Test And Operations Assessment

Required CI service-container testing avoids adding Docker libraries to Core
while ensuring the shared claims are executable. The required-test flag
prevents CI from silently skipping PostgreSQL coverage.

The backup/restore boundary is honest: rehearse maintained PostgreSQL tooling,
verify authority and projections, and retain the source without claiming high
availability or disaster recovery.

## 7. Privacy Assessment

The plan excludes connection strings, credentials, SQL, raw canonical
payloads, and secret-like values from backend state, Debug, errors, events,
reports, and fixtures. Database-side logging remains an explicit operator
consideration.

## 8. Planning Blockers

None.

## 9. Non-Blocking Follow-Ups

- Choose initial PostgreSQL major-version coverage during implementation.
- Decide whether the first TLS factory belongs in Core or a runtime-facing
  crate after dependency review.
- Keep pooling deferred until correctness and connection recycling are proven.
- Select the first revisioned catalog consumer after the run consumer.

## 10. Recommended Next Phase

Implement the complete Shared PostgreSQL State milestone on the current branch.
Use internal code slices for reviewability, but perform one phase-level
maintainer review after adapter, transactions, leases, shared consumer, and
recovery proof are complete.

## 11. Validation

Completed successfully:

- `npm run check:docs`;
- `git diff --check`.

Governed planning:

- workflow: `dg/d`;
- run ID: `run-1785332666615321000-2`;
- approval ID: `approval/run-1785332666615321000-2/planning-approved`;
- presentation ID: `presentation/e48e76580e9bd7c7`;
- outcome: granted with persisted presentation proof;
- phase status: completed;
- event summary: 39 events, one approval, zero retries, zero escalations.

Research, documentation edits, validation, and git work occurred outside the
kernel under the approved planning scope. The kernel coordinated governance and
did not execute those operations.
