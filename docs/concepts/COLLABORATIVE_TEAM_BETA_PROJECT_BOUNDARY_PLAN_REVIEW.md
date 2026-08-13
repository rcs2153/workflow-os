# Collaborative Team Beta Project Boundary Plan Review

## 1. Executive Verdict

**Plan accepted; proceed to collaborative team beta project-boundary
implementation.**

The plan converts the roadmap's broad collaborative-team milestone into one
runnable, reviewable slice: multiple pre-provisioned principals collaborate
inside one project while PostgreSQL-backed hosted and catalog paths fail closed
across projects. It reuses accepted identity, authority, immutable-run,
approval, state, catalog, audit, and report foundations rather than adding a
disconnected identity model.

The implementation is security-sensitive. Acceptance depends on proving the
complete resource-binding and authorization path against live PostgreSQL, not
merely constructing the new types or testing HTTP status codes with mocks.

## 2. Scope Verification

The plan stays within the approved planning scope. It defines one
organization-level deployment, multiple projects, multiple pre-provisioned
principals, project-scoped hosted run access, and a project-scoped PostgreSQL
workflow catalog.

It does not authorize:

- runtime code in the planning phase;
- production multi-tenancy or hostile-tenant isolation claims;
- OIDC, SSO, SCIM, external IdP, account-management, or dynamic invitation;
- enterprise administrator UI or policy sync;
- notification delivery;
- provider mutation expansion or OpenShell wiring;
- broad workflow auto-generation or auto-promotion;
- cross-project workflows or evidence sharing;
- hosted SaaS, HA, DR, quota, billing, or SLO claims;
- workflow project schema changes;
- recursive agents, agent swarms, nested harness runtime, or lineage;
- release posture changes.

## 3. Existing-Foundation Assessment

The plan accurately identifies and reuses the existing substrate:

- canonical `ProjectId` already exists and must not be duplicated;
- `ActorId` remains the principal identity projected into run decisions;
- `ProjectStateRecord` exists across local, SQLite, and PostgreSQL stores but is
  correctly rejected as an authorization source;
- PostgreSQL already provides revisioned records, serializable transactions,
  idempotency, leases, hosted work items, receipts, events, reports, and recovery
  proof;
- the hosted alpha already proves authenticated run, approval, cancellation,
  report, work-item, and receipt paths inside one trust domain;
- workflow catalog, stewardship, ownership, escalation, conflict, archive, and
  promotion contracts already exist locally;
- scoped capabilities, approval-presentation proof, immutable run bindings,
  SideEffects, evidence, audit, and WorkReports are established governance
  primitives.

The only new identifier justified at this boundary is `OrganizationId`, because
organization scope is absent and cannot be inferred safely from project or
deployment names.

## 4. Product Boundary Assessment

The selected product claim is appropriately narrow. A small team can use one
deployment to govern multiple explicitly registered projects under one
administrative trust domain. This is materially more useful than the current
single-token alpha while remaining honest about what is not proven.

The plan does not describe the result as enterprise-ready, generally
multi-tenant, or safe against hostile co-tenants. Pre-provisioned token digests
and immutable startup configuration are acceptable for this beta foundation,
provided the docs remain explicit that dynamic identity lifecycle and external
identity providers are deferred.

## 5. Authentication And Authorization Assessment

The move from `HostedApiAuth -> ActorId` to an immutable principal registry that
returns actor, organization, project grants, and explicit capabilities is the
right boundary.

The plan correctly avoids free-form roles as the enforcement source. Explicit
closed capabilities make authorization decisions deterministic and allow
convenience roles to remain configuration sugar rather than a parallel policy
language.

The proposed response posture is sound:

- `401` for missing or invalid authentication;
- bounded `403` for a known project and missing action capability;
- bounded `404` for unknown or mismatched organization/project/resource scope
  to avoid cross-project existence disclosure.

Implementation review must verify that no route authorizes only by actor or
resource ID. Exact organization, project, capability, and resource binding are
all required.

## 6. Project Registry Assessment

Server-owned project roots are essential. Allowing API callers to supply paths
would turn a collaboration feature into a filesystem capability escalation.

The startup registry validation requirements are adequate: canonical roots,
unique IDs, no conflicting roots, known project grants, and redacted Debug/
errors. The implementation should also test symlink/canonicalization behavior
and reject a configuration where one project root aliases or contains another
in a way that defeats isolation.

That containment check is a required implementation detail even though it does
not require a planning blocker.

## 7. Durable Resource-Binding Assessment

The plan is appropriately candid that current executor run creation spans
multiple transactions. It does not falsely claim atomic creation of all run
state and project binding.

The create-only reservation protocol is acceptable for the first beta slice if
all of these invariants hold:

1. reservation happens before any collaborative route can expose the resource;
2. every project-scoped read checks the reservation/binding first;
3. exact replay under the same scope is idempotent;
4. reuse under another scope is permanently rejected;
5. a failed reservation/run sequence leaves only a non-authorizing tombstone;
6. derivative resources are bound before exposure;
7. administrative diagnostics remain bounded and do not reveal hidden project
   membership.

The plan correctly declares a blocker if any derivative resource can become
externally visible before binding. That is the most important runtime invariant
in the milestone.

## 8. Worker And Derived-State Assessment

Project isolation cannot end at the API router. The plan correctly carries scope
through work-item claim, provider attempt, receipt, terminal projection, report,
reconciliation, and cancellation.

The deployment worker may operate across projects only as an explicitly
configured service actor. A project principal must never gain work-claim or
worker authority through ordinary project grants.

Implementation review should verify that metrics and health endpoints do not
become cross-project data channels. Deployment-global operational metrics may
remain administrative, but project callers must not infer another project's
queue depth, failures, or activity.

## 9. Catalog Assessment

Project-scoped immutable catalog version records are a reasonable first shared
catalog capability. Requiring owner, escalation contact, lifecycle posture, and
stewardship reference preserves the governed catalog thesis.

The plan correctly excludes arbitrary filesystem paths, automatic promotion,
and automatic workflow-file writes. Catalog metadata and immutable content
references are enough to prove shared versioning without prematurely turning
the hosted service into a source-control system.

If active-version promotion is included, it must be a revision-checked
serializable update with a stewardship decision. If the implementation cannot
complete that safely in the same milestone, version publication/read may ship
first, but the report must not claim promotion is implemented.

## 10. Idempotency And Concurrency Assessment

Binding organization, project, actor, operation, resource, and request
fingerprint into every collaborative mutation intent is required and correctly
specified. Global run-ID uniqueness remains intact.

The test plan appropriately includes cross-project and cross-actor idempotency
reuse, concurrent approval/cancellation/catalog writes, exact replay, and
restart behavior. These are behavioral proofs, not construction tests.

## 11. Approval, Ownership, And Escalation Assessment

The first slice proves collaboration with separate runner and reviewer actors,
preserves approval-presentation proof, and requires ownership/escalation
metadata where the catalog contract needs it. That is useful without claiming a
general routing system.

The plan correctly defers notification channels, escalation timers,
substitution, delegation, and quorum. Those should be the next collaborative
vertical slice after project isolation is accepted.

## 12. Privacy And Audit Assessment

The proposed access-decision audit fields are bounded and useful. They retain
actor, organization, project, capability, decision, target reference,
correlation, idempotency, and timestamp while excluding bearer material, token
digests, project roots, provider payloads, and raw repository content.

The implementation must test denial paths as aggressively as success paths.
An error body, Debug string, serialized access decision, metric label, or audit
record must not reveal hidden resource existence or sensitive configuration.

## 13. Compatibility And Migration Assessment

The plan does not silently reinterpret existing single-tenant records as
collaborative records. That is correct.

The explicit compatibility posture for existing unscoped alpha routes is still
an implementation decision. The recommended default is:

- retain them behind explicit single-tenant alpha construction;
- expose project-scoped routes only from collaborative construction;
- do not let one router mode read the other's records through an implicit
  default project mapping.

This avoids a hidden migration and keeps current tests and evaluators stable.

## 14. Test Quality Assessment

The planned matrix covers the high-risk boundaries:

- principal and project registry validation;
- project-root control and redaction;
- `401`/`403`/`404` distinctions;
- two-actor same-project collaboration;
- two-project read and mutation isolation;
- durable binding and tombstone behavior;
- idempotency, concurrency, worker restart, and API restart;
- project-scoped catalog versioning and stewardship;
- audit/error/Debug/serialization non-leakage;
- existing hosted, PostgreSQL, catalog, approval, SideEffect, report, local,
  SQLite, and workspace regressions.

Live PostgreSQL tests are mandatory for acceptance. An in-memory mock cannot
prove transaction, restart, or cross-process scope behavior.

## 15. Planning Blockers

None.

The plan resolves the major design questions sufficiently to begin one governed
vertical implementation milestone. Any discovered resource-exposure,
transaction, worker-scope, migration, or leakage defect should become a focused
implementation blocker rather than another broad planning cycle.

## 16. Non-Blocking Follow-Ups

- Decide whether nested/aliased project roots are always forbidden or can be
  admitted under a stronger filesystem boundary; default to forbidden now.
- Decide whether project catalog promotion fits safely in the first milestone
  after publication/read is complete.
- Add owner/escalation-based approval routing and bounded notifications after
  isolation review.
- Add OIDC-backed identity only after the static principal/project contract is
  accepted.
- Evaluate organization policy minimums and enterprise stewardship later.

## 17. Recommended Next Phase

**Collaborative team beta project-boundary implementation.**

Implement the accepted plan as one vertical slice spanning Core models,
PostgreSQL project/resource/catalog records, hosted principal/project
authorization, project-scoped routes, worker/report scope preservation, audit,
live integration tests, runtime/security documentation, implementation report,
and focused review.

Do not start provider expansion, dynamic identity, notifications, or enterprise
administration first. Project isolation and same-project collaboration are the
load-bearing boundary.
