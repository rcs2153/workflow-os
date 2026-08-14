# Project Approval Route Authority And PostgreSQL Integration Plan

Status: implemented and accepted; live PostgreSQL CI remains a merge gate

## 1. Executive Summary

The project-scoped approval route Core record and specialized store contract are
implemented and accepted. They preserve immutable, payload-free routing history,
but no hosted component yet derives the authority commitment from the same
deployment registry used for authentication, and no PostgreSQL adapter persists
route records.

The next implementation milestone should close both gaps as one bounded vertical
slice. It should establish one canonical deployment authority snapshot, derive
the sanitized Core registry and authority commitment from the credential-bearing
hosted registry, add a schema-v2 PostgreSQL route table and store implementation,
and prove create/read/list, concurrency, restart, migration, corruption, and
backup/restore behavior.

This plan does not authorize a hosted approval inbox, route-based approval
decisions, notifications, provider writes, public schemas, CLI behavior,
examples, dynamic identity, or production claims.

## 2. Goals

- make the credential-bearing hosted registry the sole deployment-owned source
  for the sanitized Core authority view;
- prevent authentication bindings and authority commitments from being supplied
  or constructed independently at the hosted boundary;
- bind one explicit deployment authority revision to one canonical commitment;
- detect authority rollback or same-revision content conflict across restarts;
- implement `ProjectApprovalRouteStore` for `PostgresStateBackend`;
- migrate the managed PostgreSQL schema from v1 to v2 under the existing
  advisory-lock and recovery-required posture;
- store explicit bounded index columns plus one canonical record payload and
  cross-check both on every read;
- preserve exact-retry reconciliation and first timestamps;
- fail closed for conflicting writers, stale authority, corrupt rows, and
  cross-project queries;
- extend the existing PostgreSQL recovery rehearsal to include route records;
- retain routes as historical evidence rather than current approval authority.

## 3. Non-Goals

This milestone must not add:

- a hosted approval inbox, route list endpoint, or UI;
- approval decisions authorized by a route record;
- external notification delivery or notification `SideEffect` execution;
- route update, deletion, reassignment, repair, or supersession;
- automatic route creation on every approval request;
- dynamic registry reload, OIDC, OAuth, SSO, SCIM, groups, invitations, roles,
  or enterprise administration;
- multiple organization trust domains or hostile-tenant isolation claims;
- provider mutations, new write families, or OpenShell behavior;
- workflow YAML, public schema, SDK, CLI, or example changes;
- local-filesystem or SQLite route persistence;
- production TLS, pooling, HA, PITR, retention, or SLO claims;
- release posture changes.

## 4. Existing Contracts To Reuse

Implementation must reuse:

- Core `HostedPrincipalBinding`, `HostedPrincipalRegistry`, and
  `ProjectApprovalAuthorityViewCommitment`;
- `ProjectApprovalRouteRecord`, `ProjectApprovalRouteStore`, and
  `ProjectApprovalRouteCreateResult`;
- `ProjectApprovalRouteRecord::is_decision_equivalent` for timestamp-independent
  replay;
- `HostedProjectRegistry`, credential-bearing principal configuration, and
  `CollaborativeHostedApiState` from `workflow-hosted`;
- `PostgresStateBackend`, `PostgresConnectionFactory`, and the existing bounded
  serializable retry helper;
- PostgreSQL event history and active project-resource bindings as the mutable
  facts rechecked by a later authenticated composer;
- schema metadata, advisory migration locking, recovery-required posture, and
  the current logical backup/restore rehearsal.

The generic PostgreSQL `records` table and its upsert helper must not be used for
route storage. They do not provide the required relational indexes and permit
replacement semantics that are incompatible with immutable route history.

## 5. Canonical Deployment Authority Source

The hosted crate currently owns credential digests and their
`HostedPrincipalBinding` values. Core owns a sanitized complete
`HostedPrincipalRegistry`. Those two views must not remain independently
constructible in collaborative hosted state.

The hosted credential registry constructor should:

1. validate every credential, actor, organization, project grant, capability,
   duplicate actor, and duplicate credential digest;
2. derive exactly one sanitized Core `HostedPrincipalRegistry` from every
   credential binding in the accepted hosted registry;
3. derive exactly one `ProjectApprovalAuthorityViewCommitment` from that Core
   registry;
4. bind both to an explicit deployment-owned authority revision; and
5. expose the resulting snapshot only through a read-only internal interface.

Rename the private hosted type or introduce an explicit wrapper such as
`HostedCredentialRegistry` so it cannot be confused with Core's sanitized
registry.

`CollaborativeHostedApiState` must accept one canonical credential/authority
source. It must not accept one credential registry for authentication and a
separately supplied Core registry or authority commitment for routing.

## 6. Authority Revision And Freshness

Add a bounded monotonic `HostedAuthorityRegistryRevision` and a read-only
snapshot concept equivalent to:

```text
HostedDeploymentAuthoritySnapshot
  organization_id
  authority_revision
  sanitized_registry
  authority_commitment
```

Add a source-level `ProjectApprovalAuthoritySnapshotCommitment` or equivalent
that contains the validated revision and the existing content-derived authority
view commitment. `ProjectApprovalRouteSourceCommitment` must commit that complete
snapshot commitment. The route record and PostgreSQL row must therefore prove
both which authority content was used and which deployment revision declared it
current. A content fingerprint without the revision is insufficient provenance.

The revision is deployment configuration, not an HTTP input. The snapshot must
be immutable for the lifetime of one collaborative hosted state instance.

PostgreSQL should preserve a create-or-advance high-watermark record containing
only organization, revision, commitment algorithm, and commitment fingerprint.
Initialization must fail closed when:

- a lower revision is presented after a higher accepted revision;
- the same revision carries a different commitment;
- the organization does not match the deployment scope; or
- the registry or commitment is invalid.

An exact revision and commitment replay is valid. A higher revision may advance
the high watermark atomically. Credential rotation that leaves actors and grants
unchanged does not alter the authority commitment; whether it advances the
authority revision is an operator decision, not a Core requirement.

Use a positive `u64` Core revision with checked conversion to positive signed
`BIGINT` storage. The deployment must run an explicit authority-initialization
operation after all project, credential, grant, and snapshot validation and
before constructing or serving the collaborative router. Constructors must not
perform hidden I/O, and the first route request must not lazily advance the high
watermark. This keeps invalid or rolled-back deployments from becoming
partially available.

For this immutable startup posture, freshness means that route composition uses
the currently active state snapshot and rechecks the same revision and
commitment immediately before persistence. No wall-clock TTL grants authority.
A later dynamic registry requires a separately reviewed refresh and revocation
protocol.

## 7. PostgreSQL Schema V2

Introduce managed schema v2. Do not append new DDL to the v1 schema while
retaining the v1 checksum: current initialization applies DDL before checking
metadata, which would mutate an incompatible database before rejecting it.

Refactor initialization into:

1. bootstrap-only schema metadata DDL;
2. exact empty-database installation at v2;
3. exact v1 checksum recognition;
4. one transactional v1-to-v2 migration under the existing advisory lock;
5. metadata advancement only after all v2 DDL succeeds; and
6. fail-closed handling for unknown, newer, checksum-mismatched, or
   recovery-required schemas.

The migration should create:

```text
workflow_os.project_approval_routes
  logical_subject_id          primary key
  route_id                    unique
  organization_id
  project_id
  run_id
  approval_id
  routing_reason
  escalation_id               nullable
  route_status
  recipient_actor_id          nullable
  notification_posture
  authority_revision
  source_algorithm
  source_route_id
  source_fingerprint
  record_version
  resolved_at
  record_created_at
  canonical_payload
  canonical_payload_hash
  inserted_at

workflow_os.hosted_authority_registry_high_watermarks
  organization_id             primary key
  authority_revision
  commitment_algorithm
  commitment_fingerprint
  accepted_at
```

Use relational constraints to require:

- `source_route_id = route_id`;
- routed rows have a recipient and the routed notification posture;
- unresolved rows have no recipient and use the non-delivery posture;
- escalation routing carries an escalation ID, while ordinary routing does not;
- revisions and timestamps are valid and non-empty.

`canonical_payload_hash` is a versioned SHA-256 commitment over the exact
canonical payload bytes. It detects storage drift; it is not a signature or
proof against a trusted database operator.

## 8. Required Indexes And Query Boundary

Use:

- primary key on `logical_subject_id`;
- unique index on `route_id`;
- partial routed-recipient index on organization, project, recipient, and
  logical subject;
- exact approval index on organization, project, run, approval, and logical
  subject;
- bounded reconciliation support for routing reason and optional escalation
  identity where needed by exact reads.

Every list query must apply organization, project, recipient or approval, and
strict limit predicates in SQL. Results must use deterministic logical-subject
ordering. Do not globally enumerate or decode rows before filtering.

## 9. PostgreSQL Store Semantics

Implement `ProjectApprovalRouteStore for PostgresStateBackend` behind dedicated
row helpers.

Create behavior inside the existing bounded serializable transaction helper:

1. validate and canonically encode the candidate;
2. lock the logical subject when it exists;
3. decode and integrity-check an existing row;
4. return `ReconciledExisting` when the stored record is decision-equivalent,
   preserving its first `resolved_at` and `created_at`;
5. return the stable route-store conflict when route content or authenticated
   provenance differs;
6. otherwise insert once, resolving an insertion race by rereading and applying
   the same reconciliation rule; and
7. retry only bounded PostgreSQL serialization/deadlock failures through the
   existing retry policy.

No path may update or delete a route row. Concurrent identical writers must
produce one creation and one reconciliation. Concurrent conflicting writers
must not both commit.

## 10. Row Integrity And Corruption Posture

One decoder must own all row reads. It must:

- deserialize `canonical_payload` into `ProjectApprovalRouteRecord`;
- reserialize it and require exact canonical equality;
- recompute and compare `canonical_payload_hash`;
- cross-check every explicit column against the decoded route, source
  commitment, logical subject, and timestamps; and
- return one stable non-leaking corruption error for any mismatch.

A list operation fails as a whole when any selected row is corrupt. It must not
skip the row, partially return results, repair storage, or echo stored content.

## 11. Authenticated Composition Boundary

The PostgreSQL adapter remains a storage primitive. It must not authenticate
route sources or confer authority.

The same implementation milestone should add one internal authenticated
composition request that accepts stable lookup subjects and trusted store
interfaces, never caller-authored route inputs. It should:

1. reconstruct pending approval state and the exact `ApprovalRequested` event
   from durable run history;
2. load and validate the coherent immutable run bundle;
3. derive ownership from the frozen workflow definition;
4. load an exact escalation event when escalation routing is requested;
5. require the active exact-project run binding;
6. obtain the active canonical deployment authority snapshot;
7. resolve through the accepted Core route resolver;
8. build the complete source commitment and route record; and
9. enter one PostgreSQL serializable operation that rechecks pending approval,
   active project binding, authority revision/commitment, and create-only route
   persistence.

The immutable bundle is immutable by contract. Approval state, project binding,
and authority high watermark are mutable facts and must be rechecked in the
transaction. A changed source yields a stable stale-source or conflict result,
never exact reconciliation.

No HTTP route should be added in this milestone. A later hosted consumer may
call this internal service only after separate review.

Core owns the provider-neutral reconstruction, validation, route resolution,
source commitment, and persistence orchestration contract. `workflow-hosted`
owns credential validation, canonical authority snapshot construction, explicit
startup activation, and wiring of the accepted Core service to deployment
stores. This preserves the dependency direction: Core must not depend on the
hosted crate, and hosted code must not duplicate Core route semantics.

## 12. Route History Is Not Current Authority

The route record and authority commitment prove historical resolution context.
They do not grant `ApprovalRead` or `ApprovalDecide`.

Future inbox and decision paths must independently re-evaluate exact project
binding, authenticated actor, current grant, pending approval,
approval-presentation proof, resolved-context integrity, and immutable run
binding. Revocation must take effect even when an older route remains stored.

No compatibility path may fall back to ownership metadata, requester identity,
an administrator role, or a prior route when current authority is unavailable.

## 13. Privacy And Errors

- persist identifiers, revisions, and cryptographic commitments only;
- exclude bearer digests, tokens, approval reasons, escalation messages,
  contact details, workflow contents, evidence, command output, provider
  payloads, and authority grant inventories;
- keep credential-bearing and sanitized registries redacted in `Debug`;
- use stable errors that omit organization, project, actor, run, approval,
  escalation, database, stored payload, and connection values;
- preserve non-disclosing unknown-project and unauthorized hosted posture;
- do not expose registry or route-store internals through HTTP or CLI.

## 14. Test Plan

Implementation tests must prove:

1. the hosted credential registry derives the complete sanitized Core registry;
2. authentication and authority commitment cannot use different principal sets;
3. credential digests never enter the sanitized registry or commitment;
4. authority commitment is deterministic across input ordering;
5. grant changes alter the commitment;
6. exact authority revision replay succeeds;
7. lower-revision rollback and same-revision commitment conflict fail closed;
8. clean PostgreSQL v2 initialization and idempotent reinitialization;
9. exact v1-to-v2 migration preserves all existing v1 state;
10. interrupted, newer, unknown, or checksum-mismatched schema fails closed;
11. route create, exact read, recipient list, and approval list;
12. exact replay preserves first timestamps;
13. route or source conflict never overwrites the first record;
14. concurrent identical writers create once and reconcile once;
15. concurrent conflicting writers cannot both commit;
16. routed and unresolved records remain distinguishable;
17. unresolved records never enter recipient enumeration;
18. cross-project, cross-recipient, and cross-approval reads are excluded at the
    SQL predicate boundary;
19. tampering with payload, payload hash, or every duplicated index column fails
    closed without leaking inserted secret-like test material;
20. authenticated ordinary and escalation route composition from durable facts;
21. approval projections without an approval event cannot create a route;
22. live workflow drift cannot change frozen ownership;
23. stale approval, project binding, or authority revision fails before create;
24. restart through a new backend instance returns the same canonical record;
25. backup/restore preserves route rows, indexes, authority high watermark, and
    integrity checks;
26. existing PostgreSQL, collaborative project, approval-presentation,
    immutable-run, escalation, routing, recovery, and workspace tests pass.

## 15. Operational And Recovery Posture

Extend the current PostgreSQL recovery rehearsal with at least one routed and
one unresolved record plus the authority high watermark. After restore, verify:

- exact reads and both bounded list paths;
- route payload/index/hash integrity;
- exact replay reconciliation;
- authority rollback rejection; and
- existing run, bundle, binding, projection, and report recovery checks.

This remains a logical local/CI rehearsal. It does not establish production RPO,
RTO, HA, PITR, replication, or managed-provider compatibility.

## 16. Proposed Implementation Sequence

Complete the following inside one governed implementation milestone, with
review checkpoints where the authority or migration boundary changes:

1. add the canonical hosted credential-to-authority snapshot and revision model;
2. bind the authority revision and view commitment into every route source
   commitment;
3. add schema-v2 bootstrap and exact v1-to-v2 migration;
4. implement explicit pre-serve authority high-watermark activation;
5. implement the PostgreSQL route store and shared conformance suite;
6. add the Core-owned authenticated composer and same-transaction mutable-fact
   rechecks;
7. extend restart, concurrency, corruption, and backup/restore proof;
8. update the runtime guide and focused threat model honestly;
9. run full validation and a phase-level maintainer/security review.

Do not split these into automatic model-only micro-phases unless review finds a
specific unresolved authority, migration, concurrency, or recovery blocker.

## 17. Implementation Validation

The future implementation must run:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- the live PostgreSQL state and collaborative project test targets;
- the PostgreSQL backup/restore rehearsal;
- `npm run check:docs`;
- `git diff --check`.

## 18. Review Decisions And Deferred Questions

The focused review resolved the implementation-critical questions:

- authority revision is a positive monotonic `u64` with checked PostgreSQL
  conversion;
- revision and content commitment are both bound into route source provenance;
- authority high-watermark activation is an explicit pre-serve operation after
  complete configuration validation;
- the authenticated composer is Core-owned and provider-neutral, while
  `workflow-hosted` supplies the canonical deployment authority source.

Unresolved-route operator visibility remains audit-only. Automatic route
creation at `ApprovalRequested` remains deferred until a separately reviewed
runtime consumer and operational metrics exist. These deferred questions do not
block the bounded implementation and do not authorize broader behavior.

## 19. Final Recommendation

The focused maintainer and security review is accepted after fixing authority
revision provenance, activation timing, and composer ownership. Proceed with
the canonical authority snapshot, schema-v2 migration,
PostgreSQL route store, authenticated internal composer, and recovery proof as
one governed vertical slice.

Do not implement an approval inbox, route-authorized decisions, notifications,
provider writes, workflow schemas, CLI behavior, examples, dynamic identity, or
release changes.
