# Project-Scoped Approval Route Persistence Plan Review

## 1. Executive Verdict

Plan accepted; proceed to route persistence model/store contract
implementation.

The initial maintainer review found three planning blockers. The plan now
resolves each one without broadening into a database migration, hosted inbox,
approval decision integration, or notification delivery in the first
implementation slice.

## 2. Scope Assessment

The plan remains bounded to authenticated project-scoped approval-route
persistence design. It defines durable identity, source commitments, store
semantics, backend posture, privacy, tests, and implementation sequencing.

It does not authorize a hosted inbox, approval decisions, route reassignment,
external notifications, workflow/public schemas, provider writes, dynamic
identity, enterprise administration, CLI behavior, examples, or broader
mutation families.

## 3. Authenticated Source Assessment

The plan no longer claims that independent event, immutable-bundle, authority,
and PostgreSQL stores participate in one literal transaction. It defines a
two-boundary protocol:

1. the hosted composer authenticates immutable event, run-bundle, and
   deployment-authority snapshots and creates a short-lived trusted write
   request; and
2. the serializable PostgreSQL transaction rechecks the mutable pending-
   approval and active-project-binding facts it owns before insertion.

The store interface is explicitly a storage primitive rather than an
authentication boundary. It must not be exposed directly through hosted HTTP
or accept caller-authored domain records as provenance.

## 4. Identity And Commitment Assessment

The logical subject identity is appropriately independent of route outcome,
recipient, authority, and `resolved_at`. A changed decision for one logical
slot is a conflict rather than a parallel record.

The complete source commitment now includes:

- exact workflow, schema, version, spec, resolved context, run, and approval;
- the approval-request event;
- the immutable run bundle and project binding;
- optional durable escalation provenance;
- routing reason;
- the complete canonical approval subject; and
- a versioned canonical deployment-authority commitment.

Aggregate subjects commit the complete `GovernanceApprovalBinding`, including
its exact `GovernanceAssessmentBinding`, rather than only a binding ID. The
authority commitment hashes the immutable registry's organization, principal
bindings, and capability grants in deterministic order while excluding token
digests and grant inventories from the route record.

## 5. Persistence And Replay Assessment

Create-only persistence distinguishes first creation, exact reconciliation,
and conflict. The first stored `resolved_at` and `created_at` remain canonical;
later decision-equivalent retries return the stored record.

Concurrent identical writers must converge on one creation. Conflicting
writers cannot both commit. Indexed columns and canonical payloads are
cross-checked on reads, and corruption fails closed.

## 6. Backend Assessment

PostgreSQL-first scope is appropriate because collaborative project routing is
a hosted capability. The plan does not force local-filesystem or SQLite parity.

The parent plan now distinguishes private storage migrations from public or
workflow schemas. The later PostgreSQL adapter slice may add one bounded
internal route table and indexes; the first accepted implementation slice
remains Core models, the store contract, and an in-memory contract fixture
only.

## 7. Authority Assessment

A durable route records historical routing posture, not current authority.
Future reads and decisions must independently recheck exact project binding,
current `ApprovalRead` or `ApprovalDecide`, pending approval state,
presentation proof, and immutable resolved-context integrity.

The current registry is immutable for the deployment lifetime. Any future
dynamic authority registry must add explicit revision and revalidation
semantics before replacing that posture.

## 8. Privacy And Error Assessment

The planned record contains bounded identifiers and commitments only. It does
not persist approval reasons, workflow contents, escalation messages,
evidence, command output, provider payloads, credentials, contact details,
token digests, or principal grant inventories.

Errors and `Debug` remain stable and non-leaking. Project-not-found and
unauthorized behavior preserves hosted non-disclosure semantics.

## 9. Test Assessment

The plan covers authenticated reconstruction, exact aggregate-subject
commitment, authority commitment, source-change races, duplicate replay,
conflicts, concurrency, restart, backup/restore, corruption, bounded
enumeration, unresolved posture, cross-project isolation, authority revocation,
and non-leakage.

Inbox consumption and authority-revocation behavior remain later integration
tests because the accepted first slice does not implement an inbox consumer.

## 10. Initial Blockers And Resolution

The initial review found:

1. ambiguous authentication and transaction ownership;
2. an incomplete, non-constructible authority and aggregate-subject
   commitment; and
3. a contradiction between requiring PostgreSQL storage and prohibiting the
   private migration it needs.

The two-boundary protocol, canonical authority/aggregate commitments, and
private-versus-public schema distinction resolve these blockers. Independent
fix-forward review found no remaining blockers.

## 11. Non-Blocking Follow-Ups

- Decide later whether unresolved-route operator visibility requires an
  explicit capability or remains audit-only.
- Design append-only supersession separately before ownership or authority
  changes may produce a new evaluation subject.
- Keep route creation explicit until automatic hosted composition is reviewed.
- Define inbox list limits and cursor identity in the inbox phase, not the
  store-contract phase.

## 12. Recommended Next Phase

Implement the project-scoped approval route persistence model and store
contract only:

- bounded source-commitment model;
- logical-subject identity;
- durable route record;
- create result and stable errors;
- `ProjectApprovalRouteStore`; and
- an in-memory contract fixture and focused tests.

Do not add a PostgreSQL migration, hosted composer, inbox, approval decision
path, external notification, provider write, or public schema in that phase.

## 13. Validation

- `npm run check:docs`: passed;
- `git diff --check`: passed.

No Rust checks were required because the review and blocker fixes changed only
planning documentation.

## 14. Governed Review Record

- workflow: `dg/review`;
- run: `run-1786681982615013000-2`;
- approval:
  `approval/run-1786681982615013000-2/review-scope-approved`;
- presentation: `presentation/36efd31921ea697e`;
- approval outcome: granted under delegated-maintainer authority with
  persisted presentation proof;
- review posture: independent review found three blockers; fix-forward review
  accepted all resolutions;
- terminal status: `Completed`;
- event summary: 39 events, one approval, zero retries, zero escalations;
- proof enforcement: one persisted presentation record matched the granted
  approval. Inspect output does not yet expose a proof-use event marker.

Repository inspection, independent review, documentation edits, validation,
and future git or pull-request actions are executor work outside the kernel.
The kernel governed the approved review scope and recorded the approval; it did
not perform those actions.
