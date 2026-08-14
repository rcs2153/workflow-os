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
