# Collaborative Identity Lifecycle Plan Report

> Fix-forward status: the accepted model-only recommendation is now
> implemented. See
> [Collaborative Identity Core Model Report](COLLABORATIVE_IDENTITY_CORE_MODEL_REPORT.md).

## 1. Executive Summary

Workflow OS now has a phase-ready plan for moving from static collaborative
principal configuration toward OIDC-backed identity and revocable project
grants without confusing authentication, identity binding, current authority,
approval routing, or historical evidence.

This phase is planning only. It adds no identity runtime, token validation,
durable identity storage, hosted authentication behavior, schema, UI, CLI,
provider mutation, or production claim.

## 2. Scope Completed

- Inspected the accepted hosted authentication and project-routing boundaries.
- Defined the one-organization trust-domain posture for the first identity
  lifecycle.
- Separated external authentication, local principal binding, principal
  lifecycle, project grants, revocation, current authority, sessions, routes,
  capability grants, and evidence.
- Proposed the smallest model-only first implementation.
- Defined additive revocation and fresh time-of-use authority requirements.
- Defined privacy, failure, compatibility, migration, testing, and sequencing
  posture.
- Positioned the plan in the authoritative roadmap queue.

## 3. Scope Explicitly Not Completed

- No Rust model or runtime implementation.
- No OIDC or other identity-provider integration.
- No JWT, JWKS, login, session, refresh, or logout behavior.
- No PostgreSQL schema or migration.
- No hosted authentication cutover.
- No principal-filtered inbox.
- No SCIM, groups, roles, invitations, or administration.
- No tenant-isolation or production-readiness claim.
- No provider write, OpenShell, nested harness, CLI, SDK, workflow schema,
  example, or release change.

## 4. Planning Decision

The first implementation should be Core model only. It should define:

- identity-provider trust registration;
- opaque external-subject binding;
- principal lifecycle;
- immutable exact-project grants;
- create-only grant revocation; and
- payload-free current collaborative authority projection.

OIDC protocol validation belongs in a later hosted adapter using a reviewed
standards implementation. Durable grant operations belong in a later
PostgreSQL phase with monotonic authority revision, exact replay, rollback
protection, and recovery proof.

## 5. Authority Boundary Summary

Authentication identifies an external subject for one request. A local binding
maps that subject to a Workflow OS principal. An active exact-project grant
provides scoped authority. Revocation removes future authority without deleting
history. Approval routes, decisions, events, receipts, and reports remain
evidence and cannot authorize later actions.

Every consequential use must resolve current authority again. A valid session
or historical route cannot preserve a revoked grant.

## 6. Privacy Summary

The plan prefers opaque subject commitments and excludes raw tokens, full
claims, emails, display names, group lists, credentials, signing material,
provider payloads, and unrestricted metadata from authority records. Debug,
serialization, audit, diagnostics, evidence, and WorkReports must remain
payload-free and non-leaking.

## 7. Relationship To Existing Foundations

The plan reuses the canonical hosted authority snapshot, revision/high-watermark
protection, project grants, approval route records, capability vocabulary,
immutable run bundles, approval-presentation proof, audit, EvidenceReference,
and WorkReport boundaries. It does not create a parallel permission system or
turn routing metadata into authority.

## 8. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.
- Documentation-only scope check: passed; changes are limited to `ROADMAP.md`,
  the implementation plan, and this planning report. The repo-local
  `.workflow-os/` runtime directory remains untracked and is not part of the
  phase diff.

## 9. Remaining Known Limitations

- The current hosted alpha still uses one deployment bearer identity.
- Collaborative principals and grants remain pre-provisioned and static.
- No external identity assertion is currently validated.
- No grant revocation store or current-session invalidation exists.
- No principal-filtered collaborative inbox exists.
- Multi-tenancy, enterprise administration, and production identity remain
  unsupported.

## 10. Recommended Next Phase

Perform focused maintainer and security review of the Collaborative Identity
Lifecycle Plan. If accepted, implement the Core identity and revocable-grant
model only.

Do not begin OIDC transport, persistence, hosted cutover, administration,
provider mutation, or enterprise identity claims before that review.

## 11. Governed Phase Evidence

- Dogfood workflow: `dg/d`.
- Run: `run-1792538790270618000-2`.
- Approval: `approval/run-1792538790270618000-2/planning-approved`.
- Approval presentation: `presentation/01ac5c9f9aff238b`.
- Approval outcome: granted with persisted presentation proof.
- Phase status: completed.
- Event summary: 39 events, including one approval request, one approval
  grant, six governed skill steps, zero retries, and zero escalations.
- Out-of-kernel work: repository inspection, documentation edits, validation,
  and later git/PR operations are performed by the delegated maintainer; the
  kernel governs phase scope and records the approval/event trail.
