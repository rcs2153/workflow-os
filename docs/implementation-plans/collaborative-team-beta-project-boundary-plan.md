# Collaborative Team Beta Project Boundary Plan

Status: accepted planning boundary; implementation not started

## 1. Executive Summary

Workflow OS has accepted shared PostgreSQL state and a single-tenant hosted
alpha. The hosted alpha intentionally authenticates one deployment token as one
actor and exposes one project root. It proves a remote governed-run path, but it
does not prove that multiple people can collaborate inside one project or that
one project cannot read or mutate another project's governed state.

The first collaborative team beta implementation should close that exact gap.
It should add one deployment organization, multiple explicitly registered
projects, multiple pre-provisioned principals, project-scoped hosted run access,
and a shared project-scoped workflow catalog over PostgreSQL. Two principals
must be able to collaborate inside one project, while an authenticated principal
without access to another project must fail closed before any state read or
mutation.

This is a runnable vertical slice, not a model-only phase. It reuses the existing
`ProjectId`, `ActorId`, immutable-run, approval-presentation, capability,
PostgreSQL, hosted, workflow-catalog, ownership, escalation, and stewardship
contracts. It does not claim production multi-tenancy, enterprise identity,
hosted SaaS readiness, or complete collaborative administration.

## 2. Why This Is Next

The second closed authoritative local-check profile and its review are complete.
The next authoritative roadmap milestone is collaborative team beta foundation.
The repository already contains most of the semantic pieces, but they remain
disconnected:

- `ProjectId` and `ProjectStateRecord` exist in Core and PostgreSQL;
- the single-tenant hosted API authenticates exactly one token as one `ActorId`;
- hosted routes read run-owned resources directly by global identifiers;
- the hosted deployment owns exactly one configured project root;
- workflow catalog, stewardship, ownership, escalation, conflict, and promotion
  contracts exist for local filesystem-backed operation;
- no hosted caller is durably bound to an organization, project, or scoped
  action set;
- no run, work item, receipt, report, approval, or catalog record is protected
  by a hosted project boundary.

Adding more identity vocabulary without a consuming path would repeat the
project's earlier primitives-before-product problem. The first phase must prove
the complete authorization and isolation path through an actual hosted API and
shared store.

## 3. Product Outcome

The first accepted slice should support this bounded scenario:

1. One Workflow OS deployment is configured for one organization.
2. Two projects are registered by deployment configuration with fixed server-
   owned project roots.
3. Two human or service principals are pre-provisioned with different scoped
   capabilities.
4. A runner creates a governed run in project A.
5. A reviewer in project A reads the approval context and grants or denies the
   approval with presentation proof.
6. Both authorized principals can read project A's run, events, report posture,
   and project catalog according to their capabilities.
7. Neither principal can use project A authority to inspect or mutate project B.
8. PostgreSQL preserves project binding across API and worker restarts.
9. Audit output identifies the actor, organization, project, action, decision,
   and target reference without storing bearer material or raw payloads.

The product claim is narrow:

```text
One deployment can safely host multiple explicitly registered projects for a
small pre-provisioned team under one administrative trust domain.
```

It is not a claim of hostile-tenant isolation or general enterprise access
management.

## 4. Goals

- reuse canonical Core identifiers and governance contracts;
- authenticate more than one pre-provisioned principal;
- bind every hosted principal to one organization and explicit project/action
  grants;
- expose project-scoped hosted routes for run creation and run-owned reads and
  mutations;
- persist create-only project bindings for runs and hosted derivative resources;
- persist and read versioned workflow catalog records by project scope;
- preserve actor identity through approvals, cancellation, events, audit, and
  reports;
- make same-project collaboration and cross-project isolation deterministic and
  testable;
- keep project roots deployment-owned rather than caller-selected;
- fail closed with stable, non-leaking errors;
- preserve current single-tenant API compatibility until an explicit migration
  or removal decision is reviewed;
- prove restart behavior against PostgreSQL.

## 5. Non-Goals

This milestone does not authorize:

- production multi-tenancy or hostile-tenant isolation claims;
- multiple independent organization trust domains in one deployment;
- OIDC, OAuth, SSO, SCIM, external IdP, or directory integration;
- dynamic user invitation, account recovery, password management, or token
  issuance APIs;
- a general RBAC engine or administrator UI;
- organization policy administration or remote policy synchronization;
- notification delivery;
- enterprise retention, quotas, billing, abuse controls, SLOs, HA, or disaster
  recovery claims;
- arbitrary project paths supplied by API callers;
- broad workflow catalog auto-generation or auto-promotion;
- automatic workflow conflict resolution;
- cross-project workflows or cross-project evidence access;
- provider mutation expansion, OpenShell wiring, or new execution providers;
- recursive agents, agent swarms, nested harness runtime, or reasoning lineage;
- public schema changes in workflow project YAML;
- weakening approval-presentation, immutable-run, capability, SideEffect,
  evidence, audit, or WorkReport boundaries;
- a release-posture change.

## 6. Existing Boundaries To Reuse

### 6.1 Identity

- Reuse `ActorId` for the authenticated human or service actor.
- Reuse `ProjectId`; do not create a hosted-specific duplicate project ID.
- Add a bounded `OrganizationId` only because organization identity is not
  currently represented and must be included in every hosted scope commitment.
- Do not use `ProjectStateRecord.metadata` as an authority source. It is a
  non-secret local project summary, not a validated access-control contract.

### 6.2 Governance

Reuse existing:

- immutable run bundles and resolved-context integrity;
- approval request and approval-presentation proof;
- scoped runtime capability and current-authority resolution;
- proportional-governance decision posture;
- SideEffect lifecycle, approval linkage, reconciliation, and report gates;
- evidence, audit, and WorkReport references;
- workflow catalog, stewardship, ownership, escalation, conflict, and lifecycle
  vocabulary.

The collaborative boundary must not create a parallel authorization language
that bypasses those contracts.

### 6.3 Durable State

Reuse the accepted PostgreSQL record, revision, serializable transaction,
idempotency, lease, event, snapshot, immutable bundle, hosted work-item,
receipt, report artifact, and health-check foundations.

## 7. Candidate Core Model

The implementation should add the smallest validated model set that the hosted
and PostgreSQL consumers require:

- `OrganizationId`;
- `HostedProjectScope` containing `organization_id` and `project_id`;
- `HostedPrincipalBinding` containing one `ActorId`, one organization, and an
  explicit bounded set of project grants;
- `HostedProjectGrant` containing `ProjectId` and explicit hosted capabilities;
- `HostedProjectCapability`, initially limited to:
  - `catalog_read`;
  - `catalog_publish_version`;
  - `run_create`;
  - `run_read`;
  - `approval_read`;
  - `approval_decide`;
  - `run_cancel`;
  - `report_read`;
- `HostedProjectResourceKind` for run, work item, execution receipt, report, and
  catalog record families;
- `HostedProjectResourceBinding`, a payload-free create-only commitment between
  one resource identifier and one exact `HostedProjectScope`;
- `HostedProjectAccessDecision`, a bounded allowed/denied result suitable for
  audit projection.

The capability list is closed for this phase. It must not become free-form
role text. Convenience roles may be deployment-configuration helpers, but Core
authorization must operate on explicit capabilities.

All constructors and deserializers must validate length, character set,
duplicates, empty grants, cross-organization ambiguity, unsupported
capabilities, and secret-like values. Debug and errors must redact token
digests, project roots, and caller-supplied metadata.

## 8. Authentication And Principal Configuration

### 8.1 Deployment-Owned Registry

Replace the one-token `HostedApiAuth` assumption with a deployment-owned,
immutable `HostedPrincipalRegistry` assembled at process startup. Each entry
contains:

- a token digest;
- canonical `ActorId`;
- one `OrganizationId` matching the deployment organization;
- explicit project grants and capabilities;
- optional bounded principal kind (`human` or `service`) for audit posture.

Bearer tokens remain external deployment secrets. Only digests are retained.
Duplicate token digests, duplicate principal identities with conflicting grants,
unknown projects, empty grants, and organization mismatches fail startup.

There is no runtime token-creation or grant-management API in this milestone.
Configuration changes require controlled deployment restart and are outside the
workflow event log.

### 8.2 Authorization Result

Authentication returns an immutable `HostedAuthenticatedPrincipal`, not only an
`ActorId`. Route authorization then requires:

1. the route organization equals the deployment organization;
2. the route project is registered;
3. the principal has the required capability for that exact project;
4. the target resource binding matches that exact scope when a resource already
   exists.

Unknown or mismatched organization/project/resource scope returns a bounded
`404` posture to avoid confirming cross-project resource existence. A known
project with an authenticated principal lacking the required action returns a
bounded `403`. Invalid credentials return `401`. No response includes token,
grant, path, or hidden-resource detail.

## 9. Project Registry

`HostedApiState` should own an immutable `HostedProjectRegistry` mapping each
registered `ProjectId` to a server-owned project root and bounded project
posture. Construction must:

- canonicalize and validate each root;
- reject duplicate project IDs and duplicate/conflicting roots;
- ensure roots are directories;
- ensure no project root is selected by a request body;
- avoid rendering roots in Debug or API errors;
- ensure every principal grant references a registered project.

One `OrganizationId` remains deployment-wide for this milestone. Supporting
multiple organizations is deferred until stronger tenant isolation exists.

## 10. Hosted API Boundary

Add project-scoped routes under:

```text
/api/v0alpha1/organizations/:organization_id/projects/:project_id/...
```

The first route set should cover:

- `POST /runs`;
- `GET /runs/:run_id`;
- `GET /runs/:run_id/events`;
- `GET /runs/:run_id/report`;
- `GET|POST /runs/:run_id/approvals/:approval_id`;
- `POST /runs/:run_id/cancel`;
- `GET /runs/:run_id/reports/:report_id`;
- `GET /runs/:run_id/work-items/:work_item_id` or an equivalent run-owned
  work-item path;
- `GET /runs/:run_id/work-items/:work_item_id/executions/:execution_id`;
- `GET /catalog`;
- `GET /catalog/:workflow_id/versions/:version`;
- `POST /catalog/:workflow_id/versions` for an explicitly authorized steward
  publication of an already validated version record.

The publication endpoint stores catalog metadata and immutable content
references. It must not accept an arbitrary filesystem path, write workflow
files, activate a workflow automatically, or bypass stewardship evidence.

Existing unscoped alpha routes should remain available only behind an explicit
single-tenant compatibility posture during this milestone. They must not be
silently remapped to an arbitrary default project. The final review should
decide whether they are deprecated, test-only, or retained for the alpha.

## 11. PostgreSQL Project Binding

### 11.1 Create-Only Resource Binding

Before a project-scoped run becomes reachable through the hosted API, persist a
create-only `HostedProjectResourceBinding` for its run ID. Exact replay under
the same scope is allowed. Reuse under another scope fails closed.

The current executor creates run state through multiple accepted transactions.
This phase should not falsely claim one transaction around the entire executor.
Instead, use a fail-safe reservation protocol:

1. validate principal, project, request, and immutable inputs;
2. create-only reserve the run ID for the exact project scope;
3. bind the idempotency intent to organization, project, actor, operation, and
   request identity;
4. execute the existing Core-owned run creation path;
5. bind derivative work-item, execution, report, and receipt identities before
   they are exposed through project routes;
6. leave a failed reservation as a non-authorizing tombstone if downstream
   creation fails; never reassign it to another project.

All project-scoped reads require the binding first. An orphaned run without a
binding is unreachable through the collaborative API. A reservation without a
run is diagnosable by bounded administrative health output but grants no access.

If implementation discovers that a derivative resource can become externally
visible before binding, that is a blocker requiring a stronger atomic Core/
PostgreSQL operation before the phase can be accepted.

### 11.2 Project-Scoped Catalog Store

Add a PostgreSQL-backed project workflow catalog store whose complete storage
identity includes organization, project, workflow ID, and version/record ID.
Catalog writes are create-only for immutable version records. Exact replay is
idempotent; conflicting replay fails closed.

Promotion or active-version pointers require revision-checked serializable
updates and an accepted stewardship decision reference. This milestone may
publish and read versions, but must not auto-promote recommendations or drafts.

### 11.3 Existing Project State

`ProjectStateRecord` remains local project metadata. It may be referenced for
known project posture, but it is not sufficient proof of organization
membership, principal authorization, resource scope, or catalog ownership.

## 12. Worker And Derivative Resource Scope

The stateless worker is a deployment service actor. It may claim queued work
across registered projects only through an explicit deployment capability.
Before provider invocation or terminal projection, it must load and validate the
work item's run binding and preserve that exact scope through:

- execution attempt;
- provider request and receipt reference;
- terminal run projection;
- report artifact;
- reconciliation posture;
- cancellation observation.

The provider request still receives only its existing bounded execution input.
Project scope is governance metadata, not provider credential material.

No project-owned API caller can claim work or impersonate the worker service
actor.

## 13. Idempotency And Concurrency

Every collaborative hosted mutation intent must bind:

- organization ID;
- project ID;
- authenticated actor ID;
- operation kind;
- target resource identity;
- canonical request fingerprint.

An idempotency key reused with a different organization, project, actor,
operation, or request fails closed. Concurrent create, approval, cancellation,
catalog publication, and promotion attempts use PostgreSQL serializable
transactions and revision checks already established by the shared-state
adapter.

Run IDs remain globally unique canonical identifiers. Project binding does not
weaken global uniqueness and does not make identical run IDs legal across
projects.

## 14. Ownership, Escalation, And Approval Routing

The first slice reuses workflow/catalog owner and escalation metadata and proves
that a project-scoped reviewer can decide an approval. It does not yet implement
general routing or notification delivery.

For this phase:

- catalog publication requires an owner, escalation contact, lifecycle posture,
  and stewardship decision reference;
- approval reads/decisions require explicit project capabilities;
- presentation proof remains bound to the exact actor and approval context;
- requester and reviewer are distinct in the collaborative acceptance test;
- the system records who requested, presented, and decided the approval;
- missing routing metadata is disclosed and blocks any operation whose accepted
  contract requires it.

General owner-based routing, escalation timers, delegation, substitution,
quorum, and notification channels remain the next collaborative milestone.

## 15. Audit, Evidence, And Reporting

Project-scoped hosted access and authorization decisions should project bounded
audit records containing:

- actor ID;
- principal kind;
- organization ID;
- project ID;
- action/capability;
- allowed or denied posture and stable reason code;
- target kind and payload-free target reference;
- correlation and idempotency references where relevant;
- timestamp.

Audit, event, evidence, and report reads must themselves pass project access
checks. Cross-project denial must not reveal whether a run, approval, report,
catalog version, work item, or receipt exists.

Raw bearer tokens, token digests, project roots, provider payloads, command
output, raw source, parser payloads, environment values, or secret-like values
must not enter audit or WorkReport data.

## 16. Error Handling

Use stable non-leaking codes for:

- invalid deployment principal/project configuration;
- unauthenticated request;
- unknown project scope;
- missing project capability;
- resource-scope mismatch;
- conflicting project binding;
- project-scoped idempotency conflict;
- missing or conflicting catalog version;
- invalid stewardship reference;
- derivative-resource binding gap;
- PostgreSQL unavailability or serialization exhaustion.

Errors must not include credentials, token digests, hidden organization/project
membership, paths, workflow bodies, provider payloads, or secret-like caller
values.

## 17. Compatibility And Migration

- Keep current local filesystem and SQLite behavior unchanged.
- Keep existing `ProjectId` serialization unchanged.
- Add PostgreSQL records additively under a new reviewed durable schema version
  or additive record families with updated checksum metadata.
- Do not reinterpret existing single-tenant records as project-scoped records
  without an explicit migration or deployment binding.
- Existing unscoped hosted alpha routes remain explicitly single-tenant and
  cannot read collaborative records unless a reviewed compatibility mapping is
  configured.
- Document rollback: disabling collaborative routes does not delete bindings or
  catalog state; re-enabling with the same organization/project registry
  restores access.

## 18. Test Plan

### 18.1 Model And Configuration

- valid organization/project/principal bindings;
- duplicate and conflicting principal bindings rejected;
- unknown project grants rejected;
- empty or unsupported capability sets rejected;
- safe Debug/serialization and non-leaking deserialization failures;
- duplicate project roots and IDs rejected;
- caller cannot select a project root.

### 18.2 Hosted Authorization

- unauthenticated request returns `401`;
- authenticated principal without capability returns bounded `403`;
- wrong organization/project/resource scope returns bounded `404`;
- no cross-project existence, status, event-count, approval, report, work-item,
  receipt, or catalog leakage;
- service actor and human actor capabilities remain distinct;
- route actor is preserved in approval/cancellation/audit records.

### 18.3 Collaboration

- runner in project A creates a run;
- separate reviewer in project A reads presentation context and decides it;
- both authorized actors can inspect the resulting event/report posture;
- project B principal cannot read or mutate project A;
- project A principal cannot read or mutate project B;
- same actor with grants in both projects must still use the exact route scope.

### 18.4 Durable Binding And Idempotency

- exact binding replay succeeds;
- cross-project binding replay fails;
- idempotency reuse across project, actor, operation, or payload fails;
- failed run creation leaves a non-authorizing reservation;
- unbound resource cannot be exposed;
- worker restart preserves scope;
- API restart preserves principal-independent durable scope;
- concurrent run creation, approval, cancellation, catalog publication, and
  promotion remain deterministic.

### 18.5 Catalog

- project-scoped version publication and read;
- immutable version conflict rejection;
- stewardship/owner/escalation requirements;
- revision-safe active pointer update if promotion is included;
- no cross-project catalog list, version, owner, conflict, or lifecycle leakage;
- catalog record contains references and metadata, not raw repository payloads.

### 18.6 Regression

- existing hosted alpha tests;
- existing PostgreSQL conformance and live recovery tests;
- existing workflow catalog and stewardship tests;
- existing immutable-run, approval-presentation, proportional-governance,
  SideEffect, evidence, report, and audit tests;
- local filesystem and SQLite state tests;
- full workspace and docs checks.

## 19. Validation Commands

The implementation milestone should run at minimum:

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run check
npm run check:integrations
```

It must also run the repository's live PostgreSQL conformance/recovery suite and
a focused two-project/two-principal hosted integration test. Skipped live checks
must be disclosed and block acceptance when they are the only proof of project
isolation or restart behavior.

## 20. Implementation Sequence

Execute this as one governed vertical milestone, with internal checkpoints:

1. add validated organization, scope, principal, capability, resource-binding,
   and access-decision models;
2. add PostgreSQL project-resource binding and project catalog record families;
3. replace single-actor hosted authentication internally with the immutable
   principal/project registries while retaining explicit alpha compatibility;
4. add project-scoped routes and capability checks;
5. bind run and derivative hosted resources before exposure;
6. add project-scoped catalog publication/read;
7. preserve scope through worker, receipt, report, and reconciliation paths;
8. add bounded authorization audit projection;
9. prove two-actor same-project collaboration, two-project isolation,
   idempotency, concurrency, restart, and regression behavior;
10. update runtime/security docs and create implementation report;
11. run focused maintainer review before any notification, dynamic identity,
    or provider expansion work.

If a real blocker is found in authentication, cross-project isolation, atomic
resource exposure, worker scope, idempotency, migration, or audit leakage, stop
that internal lane for a focused blocker fix. Do not split every ordinary model,
store, route, and test step into separate roadmap phases.

## 21. Acceptance Criteria

- one deployment organization and at least two registered projects are
  representable;
- at least two pre-provisioned principals can hold different project
  capabilities;
- project A run and catalog operations are inaccessible from project B;
- one project A actor can create a run and another can decide its approval;
- all hosted run-owned resources require exact durable project binding;
- project roots remain server-owned;
- catalog versions are immutable and project-scoped in PostgreSQL;
- mutation idempotency binds organization, project, actor, operation, and
  request;
- worker, receipt, report, and reconciliation paths preserve scope;
- errors, Debug, audit, and serialization are non-leaking;
- restart and concurrent behavior are proven against live PostgreSQL;
- current local and single-tenant behavior remains compatible or has explicit
  reviewed migration posture;
- no production multi-tenancy, enterprise identity, notification, provider
  expansion, or hosted-SaaS claim is introduced.

## 22. Recommended Next Phase

Proceed to **collaborative team beta project-boundary implementation** as one
governed vertical slice.

After that implementation and review, the next collaborative milestone should
compose ownership/escalation metadata into approval routing and bounded
notifications. Dynamic OIDC identity and enterprise stewardship remain later.
