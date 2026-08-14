# Project-Scoped Approval Route Persistence Model And Store Report

## 1. Executive Summary

Workflow OS now has the bounded Core model and storage contract required to
preserve one project-scoped approval route as immutable, payload-free routing
history. The implementation separates the logical route slot from the route
decision identity, commits the complete authenticated source posture, preserves
the first timestamps on exact retries, and fails closed on conflicting content
or provenance.

This phase does not add a database, hosted composer, approval inbox, route-based
approval authority, notification delivery, provider writes, schemas, CLI
behavior, examples, or release changes.

## 2. Scope Completed

- added a content-derived logical route-subject identity;
- added versioned route-record, source-commitment, and authority-view models;
- added a closed, validated, deployment-owned principal registry and derived
  the authority commitment only from that complete registry;
- committed schema, workflow, spec, resolved-context, run, approval subject,
  approval event, one coherent validated immutable bundle manifest, active
  project binding, optional
  escalation event, routing reason, and authority view;
- committed the complete aggregate `GovernanceApprovalBinding` serialization
  when the approval uses an aggregate governance subject;
- added an immutable route record with validated deserialization and redacted
  `Debug` behavior;
- bound every source commitment to the exact route decision identity and
  reject mismatched route/source envelopes;
- added named decision-equivalent replay semantics that preserve the first
  `resolved_at` and `created_at` values;
- added the specialized create/read/exact-scope list
  `ProjectApprovalRouteStore` contract;
- added an in-memory contract fixture with atomic create-only reconciliation;
- added an accessor for the project resource binding timestamp so its complete
  typed commitment can be derived;
- added focused model, concurrency, isolation, tamper, and non-leakage tests.

## 3. Scope Explicitly Not Completed

This phase did not add a PostgreSQL table or migration, local filesystem or
SQLite route persistence, authenticated event/bundle composition, hosted HTTP
behavior, automatic route creation, a project approval inbox, approval
decisions, route reassignment or supersession, external notifications, dynamic
identity, provider writes, workflow schemas, CLI commands, examples, or release
posture changes.

## 4. Model And Store Summary

`ProjectApprovalRouteLogicalSubjectId` identifies one immutable route slot from
the exact project, run, approval, routing reason, and optional escalation
subject. It intentionally excludes recipient, status, authority, and resolution
time.

`ProjectApprovalRouteSourceCommitment` binds the route to the complete typed
source posture. `ProjectApprovalRouteRecord` then combines the existing route,
logical subject, source commitment, record version, and first creation time.

`ProjectApprovalRouteStore` exposes only create, exact read, exact
project/recipient listing, and typed exact project/run/approval listing. It exposes no
update, delete, global enumeration, or arbitrary payload query. The trait is a
storage primitive, not an authentication boundary.

## 5. Reconciliation And Concurrency

The first successful record is canonical. A later candidate with the same
logical subject, route ID, and complete source commitment reconciles to that
stored record even if its `resolved_at` or `created_at` is later. Any route or
source change for the same logical subject returns a stable conflict and never
overwrites the first record.

The in-memory fixture performs the comparison and insertion while holding one
store lock. Focused concurrent tests prove that identical writers produce one
creation and one reconciliation, while conflicting writers cannot both commit.

## 6. Authority Boundary

The authority-view commitment can only be created from a validated
`HostedPrincipalRegistry`. The registry fixes one organization and a complete,
sorted, duplicate-free principal, project-grant, and capability view.
Credential digests cannot enter the model. The persisted route remains
historical routing evidence only. It does not grant or preserve current
approval authority.

The model constructor accepts trusted typed source records because this phase
defines the Core commitment boundary. A later hosted composer must reconstruct
those records from the event log, immutable bundle store, active project
binding, and deployment authority source. HTTP callers must never be allowed to
construct durable route truth directly.

## 7. Privacy And Redaction

The durable model stores identifiers and cryptographic commitments only. It
does not copy approval reasons, workflow contents, escalation messages,
evidence, command output, provider payloads, credentials, contact details, or
principal grant inventories. `Debug` redacts route, subject, commitment, actor,
project, and timestamp identities. Validation, conflict, store, and serde
errors use stable messages without caller values.

## 8. Test Coverage

Focused tests cover:

- valid construction, creation, exact read, serde round trip, and record
  integrity;
- exact retry reconciliation with first timestamp preservation;
- conflicting source provenance for one logical subject;
- mismatched route decision and source-commitment identity rejection;
- concurrent identical and conflicting writers;
- deterministic authority commitments independent of input ordering;
- authority commitment changes when project grants change;
- duplicate and cross-organization authority-registry rejection;
- coherent immutable-bundle and exact approval-context matching;
- aggregate approval-subject commitment to nested assessment provenance;
- missing resolved context and inactive run-binding rejection;
- exact project/recipient bounded enumeration;
- unresolved-route exclusion from recipient enumeration;
- exact approval enumeration without cross-project visibility;
- typed bounded approval-reference enumeration input;
- tampered logical-subject deserialization failure;
- `Debug`, error, and serialization non-leakage.

## 9. Validation Commands

Completed during implementation:

- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed;
- `npm run check:docs`: passed;
- `git diff --check`: passed.

## 10. Remaining Limitations

- no durable production adapter exists;
- no PostgreSQL serializable transaction rechecks mutable approval and project
  binding facts;
- no authenticated composer reconstructs route inputs from accepted stores;
- no restart, backup/restore, indexed-column corruption, or live PostgreSQL
  proof exists for route records;
- no principal-filtered hosted inbox consumes the store;
- no decision path treats a route as current authority;
- re-evaluation, append-only supersession, and reassignment remain undefined.

## 11. Recommended Next Phase

The focused maintainer and security review is accepted after fix-forward
hardening. Proceed to canonical deployment-authority commitment integration
and PostgreSQL route-store planning before one create/read adapter with its
internal migration and concurrency/restart/integrity tests. Do not begin a
hosted inbox or route-based approval decisions first.

## 12. Governed Phase Record

- dogfood workflow: `dg/implement`;
- run ID: `run-1786682956270894000-2`;
- approval ID:
  `approval/run-1786682956270894000-2/implementation-approved`;
- approval outcome: granted under delegated-maintainer authority with persisted
  presentation proof `presentation/99e79e8767a66d07`;
- phase status: `Completed`;
- validation summary: all required Rust, documentation, and diff checks passed;
- event summary: 39 events, one approval, zero retries, zero escalations;
- proof enforcement: one persisted presentation record matched the granted
  approval and the approval event trail contains its proof marker.

Repository inspection, implementation, tests, documentation, validation, and
git operations are executor work performed outside the kernel. The kernel
governed the approved phase scope and approval; it did not execute those
actions.
