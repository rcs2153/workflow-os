# Project-Scoped Approval Route Persistence Plan Report

## 1. Executive Summary

Project-scoped approval route persistence is now planned. The plan closes the
gap between the accepted pure resolver and durable hosted use by requiring
authenticated reconstruction from event history, immutable run bundles, active
project binding, and deployment-owned authority before one create-only route
record may be written.

This phase is planning only. No persistence code, migration, hosted inbox,
notification delivery, approval enforcement, provider write, schema, CLI, or
runtime behavior was added.

## 2. Scope Completed

- defined durable source authentication order;
- defined a complete approval-subject and provenance commitment;
- separated logical route-subject identity from route decision identity;
- defined create-only, exact-retry, conflict, concurrency, and timestamp rules;
- chose a store interface with a PostgreSQL-first production adapter boundary;
- defined routed and unresolved persistence posture;
- preserved route history as distinct from current authority;
- defined a two-boundary authenticated composition and transactional recheck
  protocol;
- fixed the canonical immutable deployment-authority commitment posture;
- defined bounded indexes, integrity checks, privacy, and test requirements;
- updated the parent routing plan and roadmap sequence.

## 3. Scope Explicitly Not Completed

No Core model, store trait, backend implementation, database table, migration,
composition helper, hosted endpoint, approval decision integration, route
reassignment, notification adapter, public schema, CLI command, example,
provider write, or release change was implemented.

## 4. Primary Planning Decisions

The plan requires callers to supply stable lookup subjects rather than
caller-authored approval, ownership, escalation, principal, or route records.
The future composer must rehydrate the run from its event history, recover
ownership from the exact immutable run bundle, require a matching approval event
and optional escalation event, verify the active exact-project binding, and use
the deployment-owned authority view before invoking the accepted resolver.

The logical persistence subject is exact project, run, approval, routing reason,
and optional escalation. Route outcome and recipient are not part of that slot;
a changed outcome for the same subject is a conflict until append-only
supersession is separately designed.

## 5. Backend And Authority Posture

The first production adapter is planned for PostgreSQL because project-scoped
collaboration is a hosted boundary. The store remains interface-backed. Local
filesystem and SQLite migrations are not required for backends that do not
claim the collaborative routing capability.

The hosted composer authenticates immutable event, bundle, and deployment
authority snapshots before insertion. The serializable PostgreSQL transaction
then rechecks the mutable pending-approval and active-project-binding facts it
owns. The plan does not claim that independent stores and the immutable
in-memory authority registry participate in one database transaction.

The deployment registry supplies a versioned canonical content commitment over
its organization, principal bindings, and grants while excluding credential
digests. Aggregate approval subjects commit the complete canonical
`GovernanceApprovalBinding`, including its assessment.

A persisted route is routing history, not authority. Future reads and decisions
must independently enforce current exact-project capability and pending-
approval state.

## 6. Privacy And Security Posture

The planned record stores stable identifiers and source commitments only. It
does not copy approval reasons, workflow contents, escalation messages,
evidence, command output, provider payloads, credentials, contact details, or
principal grant inventories. Errors and `Debug` remain stable and non-leaking.

## 7. Test Plan Summary

Future tests cover source reconstruction, orphan projection rejection,
immutable-definition drift, complete subject commitments, duplicate
reconciliation, timestamp replay, conflicts, concurrency, PostgreSQL restart,
backup/restore, tamper detection, bounded enumeration, cross-project isolation,
unresolved posture, authority revocation, and non-leakage.

## 8. Validation

Completed:

- `npm run check:docs`: passed;
- `git diff --check`: passed.

No Rust checks were required for this documentation-only planning phase. The
accepted Core implementation and full CI remained the starting evidence; no
code or runtime contract changed in this phase.

## 9. Remaining Limitations

- open automatic versus explicit route-creation trigger;
- no append-only supersession model;
- no unresolved-route operator capability;
- no route persistence or hosted consumption implementation.

## 10. Recommended Next Phase

Project-scoped approval route persistence plan review. If accepted, implement
the route persistence record and store contract only, with an in-memory contract
fixture and no database migration.

## 11. Governed Phase Record

- dogfood workflow: `dg/d`;
- run ID: `run-1786681586992357000-2`;
- approval ID:
  `approval/run-1786681586992357000-2/planning-approved`;
- approval outcome: granted under delegated-maintainer authority with persisted
  presentation proof `presentation/c1ce2888fdbd56be`;
- terminal status: `Completed`;
- event summary: 39 events, one approval, zero retries, zero escalations;
- proof enforcement: one persisted presentation record matched the granted
  approval. Inspect output does not yet expose a proof-use event marker.

Repository inspection, parallel planning review, documentation edits,
validation, and future git or pull-request actions are executor work outside the
kernel. The kernel governed the approved planning scope and recorded the
approval; it did not perform those actions.
