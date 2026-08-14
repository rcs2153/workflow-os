# Collaborative Team Beta Project Boundary Report

## 1. Executive Summary

The first collaborative team beta project boundary is implemented as one
vertical slice. Workflow OS can represent one deployment organization, multiple
server-registered projects, and multiple pre-provisioned principals with exact
closed capabilities. The project-scoped hosted router binds runs and hosted
derivative resources durably in PostgreSQL, supports two-actor approval
collaboration, stores immutable project catalog versions, and fails closed on
cross-project access.

This is a local evaluation milestone. It does not claim production
multi-tenancy, enterprise identity, hosted SaaS readiness, or provider expansion.

## 2. Scope Completed

- `OrganizationId` and validated project scope, principal, capability, resource
  binding, access decision, and catalog-version models;
- immutable deployment project and principal registries;
- project-scoped run, event, approval, cancellation, report, work-item, receipt,
  and catalog routes;
- create-only PostgreSQL resource reservations and active bindings;
- atomic run activation plus work-item publication;
- atomic receipt/report binding with terminal projection;
- explicit collaborative worker posture requiring bindings;
- payload-free allowed/denied authorization records;
- project catalog publication with complete approved stewardship proof;
- safe hosted filesystem loading and registered manifest/project matching;
- focused Core, loader, hosted, and live PostgreSQL CI tests;
- runtime and threat-model documentation.

## 3. Scope Explicitly Not Completed

No production tenant isolation, multiple organization trust domains, OIDC,
OAuth, SSO, SCIM, invitations, dynamic grants, admin UI, notifications, quotas,
billing, retention, production TLS/pooling/HA/PITR, cross-project workflows,
provider expansion, caller-submitted hosted work, schema changes, or release
posture changes were introduced.

## 4. API And Authorization Summary

The collaborative API is a separate router under organization/project paths.
Authentication resolves bearer digests to canonical principal bindings.
Authorization requires the route organization, registered project, exact
capability, and active resource scope. Invalid authentication returns `401`, a
known project without capability returns `403`, and unknown/mismatched scope
returns bounded `404` posture. Existing unscoped alpha routes retain their
legacy contract for unbound resources but now fail closed with `404` when a
requested run belongs to the collaborative project namespace.

## 5. Durable Boundary Summary

A run identity is reserved before execution. Dispatch atomically activates that
reservation before the first work item becomes claimable. Work-item bindings
commit with dispatch. Execution receipt and report bindings commit in the same
serializable transaction as terminal work-item state, receipt, report artifact,
events, and run snapshot. Exact reservation replay tolerates retry timestamps;
cross-project identity reuse fails closed.

## 6. Catalog And Stewardship Summary

Catalog storage identity includes organization, project, workflow, and version.
Publication requires owner and escalation metadata, the authenticated publisher,
and a matching complete `ApprovedForPromotion` stewardship record. The
stewardship record and immutable version are persisted together. Publication
does not write or activate workflow files.

## 7. Security Hardening

Maintainer review during implementation found and fixed:

- manifest layout and symlink path escape;
- reserved-run dispatch race;
- implicit legacy-worker binding bypass;
- receipt route hierarchy substitution;
- self-asserted catalog stewardship;
- catalog full-family listing coupling;
- missing authenticated scope-denial audit;
- project/scope Debug leakage;
- concurrent duplicate mutation execution posture.
- collaborative-resource exposure through legacy alpha routes.

Duplicate collaborative mutations currently fail closed after the first durable
intent rather than replaying a stored response. This is safe but intentionally
less ergonomic than a future completed-result protocol.

## 8. Test Coverage

Focused tests cover model validation, duplicate rejection, serde fail-closed
behavior, redacted Debug, unsafe layout/symlink rejection, hosted regressions,
resource reservation replay, cross-project binding conflict, restart recovery,
project-scoped catalog publication/replay/list isolation, allowed/denied access
decision persistence, two-actor project-A approval collaboration, project-B
non-disclosing denial, and legacy-route collaborative-resource rejection.

## 9. Validation

Completed locally:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `cargo test -p workflow-hosted tests::collaborative_project_boundary -- --exact`;
- focused `project_loader`, `hosted_project`, PostgreSQL, and hosted tests;
- `npm run check` under the repository-pinned Node 20 toolchain;
- `npm run check:integrations` under the repository-pinned Node 20 toolchain;
- `git diff --check`.

The laptop has no Docker/PostgreSQL runtime, so live database execution was
delegated to required CI run 1113. All required jobs passed, including the
collaborative two-project/two-principal boundary, shared-state conformance,
PostgreSQL backup/restore integrity rehearsal, and hosted restart recovery.
These live paths are reported as CI evidence, not as locally executed checks.

## 10. Remaining Limitations

- pre-provisioned bearer credentials only;
- one organization trust domain;
- no general approval routing or notification delivery;
- no production operations posture;
- duplicate mutation responses are not replayed;
- compatibility alpha surfaces remain unscoped for legacy records and are
  explicitly barred from project-bound records.

## 11. Recommended Next Phase

Proceed to ownership, escalation, approval routing, and bounded notification
planning/implementation. Do not broaden providers or dynamic identity first.

## 12. Governed Phase Record

- dogfood workflow: `dg/implement`;
- run ID: `run-1786657696432925000-2`;
- approval ID:
  `approval/run-1786657696432925000-2/implementation-approved`;
- approval outcome: granted with persisted presentation proof
  `presentation/dbb370238fa88c13`;
- terminal status: `Completed`;
- event summary: 39 events, one approval, zero retries, zero escalations;
- event kinds: one run creation, validation, start, resume, and completion;
  six scheduled and successful skill invocations; eight policy decisions; one
  approval request and grant.

Repository edits, shell commands, validation commands, and the forthcoming git
and pull-request operations are execution work performed outside the kernel.
The kernel governed scope and approval, persisted the approval-presentation
proof, and recorded the phase event trail; it did not edit files, execute the
validation suite, write git state, or operate GitHub. Live PostgreSQL execution
was performed by required CI and was not simulated.
