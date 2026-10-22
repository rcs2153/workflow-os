# Authoritative Collaborative Identity Source Boundary Plan Report

## 1. Executive Summary

Planning is complete for the authoritative collaborative identity source
boundary. The plan closes the gap between validated caller-assembled identity
claims and trusted current authority by defining one Core-owned PostgreSQL
source, complete exact queries, database time, monotonic authority revisions,
rollback protection, and same-call consumption at one approval-decision
boundary.

No implementation was added.

## 2. Scope Completed

- Defined authoritative source ownership and explicit caller boundaries.
- Reused the existing generic current-authority source model.
- Defined persistent collaborative identity fact families.
- Defined canonical record and exact replay/conflict behavior.
- Defined organization revision high watermark and rollback protection.
- Defined PostgreSQL database-time semantics.
- Defined exact-query completeness and complete-negative behavior.
- Defined same-call `ApprovalDecide` consumption.
- Defined concurrency, privacy, recovery, migration, failure, and test posture.
- Sequenced a persistent source slice before runtime consumption.

## 3. Scope Explicitly Not Completed

- No Rust implementation.
- No PostgreSQL schema migration.
- No runtime authority consumer.
- No OIDC, OAuth, SAML, SCIM, groups, invitations, or directory sync.
- No hosted authentication cutover or session issuance.
- No workflow schema or CLI behavior.
- No provider writes or external side effects.
- No production identity or enterprise-readiness claim.

## 4. Core Architectural Decision

Collaborative identity will not create a parallel authority framework. It will
specialize the accepted `CurrentAuthoritySource` contracts and source trust
rules. The public non-authorizing collaborative claim remains descriptive and
cannot be promoted into runtime authority.

Review preparation found that the current V1 source request is specifically
bound to required-context execution contracts and that its fact-family
vocabulary cannot represent all collaborative identity prerequisites. The plan
now requires a minimal versioned, closed, Core-constructed collaborative query
binding and domain-neutral fact-family extension inside the persistent-source
vertical slice. It explicitly forbids disguising identity subjects as required-
context hashes or counting all identity facts as capability grants.

## 5. First Implementation Boundary

The recommended first code phase is a PostgreSQL persistent fact store and
Core-owned exact-query source read. It persists provider, binding, principal,
grant, and revocation facts; activates monotonic organization revisions; reads
at database time; and proves complete exact-query source snapshots.

The approval-decision consumer follows as a separate phase after the source is
reviewed.

## 6. Security And Privacy Posture

The plan requires opaque external identity, no stored credentials or provider
assertions, non-leaking errors, redacted Debug, payload-free authority
commitments, exact-project denial behavior, and fail-closed handling for source
failure, corruption, ambiguity, rollback, staleness, and concurrent revocation.

## 7. Validation

- Repository architecture and current authority foundations inspected.
- Collaborative identity, approval routing, PostgreSQL recovery, and threat
  model boundaries reconciled.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 8. Remaining Limitations

- The plan does not select a provider protocol or authentication mechanism.
- The exact existing hosted approval-decision integration function remains an
  implementation-review decision.
- Revision manifest representation and freshness duration remain open.
- The dedicated draft-PR live sandbox proof remains dependency-blocked and
  unrelated to this no-write planning lane.

## 9. Recommended Next Phase

Focused maintainer/security review of the Authoritative Collaborative Identity
Source Boundary Plan. If accepted, proceed to the PostgreSQL persistent fact
store and Core-owned exact-query source implementation.

## 10. Governed Phase Evidence

- Dogfood workflow: `dg/d`.
- Run: `run-1792647479158775000-2`.
- Approval: `approval/run-1792647479158775000-2/planning-approved`.
- Approval presentation: `presentation/02426ad157178ecd`.
- Approval outcome: granted under delegated-maintainer authority with persisted
  presentation proof.
- Phase status: completed.
- Event summary: 39 events, including one approval request, one approval grant,
  six governed skill steps, zero retries, and zero escalations.
- Out-of-kernel work: repository inspection, architecture analysis, plan
  writing, documentation validation, and later git/PR actions are performed by
  the delegated maintainer; the kernel governs scope and records the approval
  and event trail.
