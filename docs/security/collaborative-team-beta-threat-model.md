# Collaborative Team Beta Threat Model

Status: focused threat model for the first project-boundary implementation

This document covers one deployment organization with multiple explicitly
registered projects and pre-provisioned principals. It does not claim hostile-
tenant isolation, production multi-tenancy, enterprise identity, or security
certification.

## Protected Assets

- project-scoped run, event, approval, work-item, receipt, and report metadata;
- immutable project workflow catalog versions and stewardship records;
- server-owned project roots;
- bearer credentials and credential digests;
- bounded authorization decisions.
- immutable project approval route history and canonical deployment authority
  commitments.

## Trust Boundaries And Controls

| Threat | Control |
| --- | --- |
| Caller selects another filesystem root | Roots come only from the immutable deployment registry |
| Manifest redirects discovery outside its root | Absolute/parent layouts and symlinked manifests, directories, and specs fail closed |
| Caller guesses another project's resource ID | Every collaborative read requires an active exact project binding; mismatches return `404` |
| Caller uses the legacy alpha API to bypass project routing | Legacy run-owned reads and mutations reject resources carrying any collaborative project binding |
| Run ID is reused across projects | Create-only global binding allows exact replay only under the original scope |
| Worker processes unbound collaborative work | Collaborative worker posture requires active run and work-item bindings before invocation |
| Receipt or report becomes visible before scope binding | Terminal bindings commit atomically with receipt, report, work-item, and run projection |
| Receipt is substituted under another run path | Receipt reads verify the bound work item belongs to the route run |
| Caller self-asserts catalog approval | Publication requires and atomically persists a matching approved stewardship record |
| Bearer or project root leaks through errors/Debug | Digests and roots are not serialized; Debug and errors use redacted or stable bounded values |
| Cross-project denial reveals existence | Scope mismatch and unknown scope return the same bounded `404` posture |
| Authentication bindings and approval-routing authority diverge | The hosted credential registry derives the complete sanitized Core registry and one revision-bound authority snapshot |
| Rolled-back or conflicting deployment authority serves traffic | Explicit pre-serve PostgreSQL high-watermark activation rejects rollback and same-revision content conflict |
| Stale approval, project binding, or authority creates a route | One serializable route-create transaction rechecks pending approval history, active exact-project run binding, and exact current authority |
| Database columns drift from canonical route content | Every read recomputes the canonical payload hash and cross-checks all duplicated index columns |

Authenticated authorization decisions are stored as payload-free records with
actor, principal kind, scope, capability, allowed/denied posture, stable reason,
target kind/reference, and timestamp. They do not contain bearer material,
project roots, provider payloads, source contents, or command output.

## Residual Risk

- Pre-provisioned bearer authentication is an evaluation mechanism, not
  enterprise identity lifecycle management.
- The deployment remains one administrative trust domain; PostgreSQL operators
  and process operators are trusted.
- The compatibility single-tenant router and worker remain legacy surfaces for
  unbound records; they reject project-bound records but are not themselves a
  project-scoped API.
- Mutation retries currently fail closed for duplicate intent rather than
  replaying a stored response. This preserves single execution but is not a
  complete ergonomic idempotent-response protocol.
- General ownership routing, escalation timers, delegation, notifications,
  quotas, abuse controls, and production operations remain unimplemented.
- Approval route records are historical evidence, not current authority. No
  inbox, decision endpoint, notification delivery, or automatic route creation
  consumes them yet.
