# Collaborative Identity Lifecycle Plan Review

## 1. Executive Verdict

Plan accepted; proceed to the collaborative identity lifecycle Core model only.

The plan closes the correct next conceptual gap after accepted project approval
routing and hosted authority integration. It separates external authentication,
opaque local identity binding, principal lifecycle, exact-project grants,
additive revocation, point-in-time authority, sessions, historical routes, and
evidence. That separation is required before collaborative approval surfaces or
provider mutations can safely depend on a changing population of principals.

No planning blocker was found. The first implementation must remain model-only
and must reuse the existing hosted project capability vocabulary rather than
creating a parallel permission system.

## 2. Scope Verification

The planning phase stayed within its approved documentation-only scope. It
updated the authoritative roadmap, created one implementation plan, and created
one planning report.

It did not add Rust types, identity-provider protocol handling, hosted
authentication behavior, PostgreSQL identity tables, schema migrations,
principal inboxes, directory synchronization, groups, administration, provider
writes, CLI or SDK behavior, workflow schemas, examples, or release changes.

## 3. Problem And Sequencing Assessment

The plan correctly identifies the current boundary. `HostedPrincipalRegistry`
is a complete immutable deployment-owned view, and `HostedProjectGrant` carries
an exact project plus a closed capability set. Those types do not model external
identity proof, principal disablement, grant issuance, expiry, or revocation.

Project approval route records already preserve a revision-bound authority
snapshot as historical evidence, while PostgreSQL high-watermark activation
prevents authority rollback. The next safe move is therefore to model identity
and revocable authority semantics before attempting an external identity
adapter, dynamic inbox, or wider mutation surface.

The roadmap placement is appropriate. Identity lifecycle follows accepted
routing integration and precedes enterprise stewardship, dynamic collaborative
surfaces, and provider-mutation broadening.

## 4. Source-Of-Truth Boundary Assessment

The plan defines the important negative guarantees:

- an external assertion does not grant Workflow OS authority;
- a local subject binding does not grant project access;
- a hosted session does not become reusable workflow authority;
- an approval route does not prove current decision authority;
- a capability grant does not prove identity by itself; and
- audit events, reports, receipts, and earlier approvals remain evidence rather
  than permission to act.

The requirement to resolve current authority at each consequential use is the
right invariant. Session validity and immutable run intent may survive for their
bounded purposes, but neither may preserve a revoked or expired actor grant.

## 5. Candidate Model Assessment

The candidate model is domain-neutral and appropriately conservative:

- provider registration records establish one reviewed trust configuration;
- opaque subject commitments avoid making raw external identity data part of
  the durable authority model;
- principal lifecycle records preserve history while allowing disablement;
- immutable exact-project grants bind one principal to a closed capability set;
- create-only revocations remove future authority without rewriting history;
  and
- point-in-time authority projections describe one bounded decision rather than
  a reusable bearer capability.

The model phase should keep trusted current-authority projections
constructor-only. Serialized data may deserialize into an explicitly untrusted
claim or persisted record, but must not directly become trusted current
authority.

## 6. Relationship To Existing Authority Models

The plan correctly reuses `HostedProjectCapability` for collaborative project
authority. It must not introduce a second project permission vocabulary in the
identity module.

The existing general runtime `CapabilityGrant` family serves a different
boundary: step-scoped tool and context authority during governed execution.
Future composition may require both an active collaborative project grant and a
resolved runtime capability grant. One must not silently substitute for the
other.

The model should also preserve the existing canonical hosted authority snapshot
and high-watermark contract. A later persistence phase may extend its source
facts, but must not create an independent identity revision that can authorize
actions while the deployment authority commitment disagrees.

## 7. Revocation And Time-Of-Use Assessment

Additive revocation is the correct durable posture. Exact replay may reconcile;
a conflicting second revocation must fail closed. Historical grants, routes,
approvals, events, and reports remain inspectable after revocation.

The plan also correctly requires fresh authority before approval presentation
or decision, capability consumption, provider invocation, SideEffect
transition, collaborative inbox exposure, delegated-authority issuance, and
consequential resume after restart or retry.

The model-only phase can represent effective and expiry posture, but it must not
claim trusted-time enforcement. Database-time and clock behavior belong to the
later persistence/runtime phase and require their own review.

## 8. Authentication Adapter Assessment

Keeping external identity protocol handling outside generic Core is correct.
Core should receive only a private same-call authenticated assertion from a
reviewed adapter, then resolve the local binding and current grants.

The plan correctly defers protocol flow, library selection, discovery, key
rotation, replay handling, and endpoint design. Those choices need a dedicated
threat model and dependency review. Caller-authored headers, serialized claims,
and wire records must never construct trusted identity directly.

## 9. Privacy And Failure Assessment

The privacy posture is strong. Authority records exclude raw assertions,
complete claims, email addresses, display names, group lists, signing material,
provider payloads, and unrestricted metadata. Opaque subject commitments and
separate optional display metadata reduce authority-record exposure.

The listed failure modes are fail-closed and non-enumerating. Stable errors
must not echo external subject values, human-readable identity data, claim
values, authority contents, database values, or provider responses. Debug,
serialization, audit, evidence, and WorkReport paths must preserve the same
boundary.

## 10. Compatibility And Product Assessment

The plan preserves local single-user operation without requiring federated
identity. That is necessary: collaborative identity is a hosted capability, not
a prerequisite for the local kernel.

It also treats the current deployment bearer path and static registry as
evaluation compatibility surfaces rather than production identity. Any later
parallel adoption or cutover requires an explicit migration plan, dry run,
mapping review, commitment verification, and rollback posture. Silent identity
conversion is correctly prohibited.

The one-organization trust domain is an honest first boundary. Multi-tenant
isolation, enterprise administration, directory provisioning, dynamic groups,
and cross-domain delegation remain unsupported and must not be inferred from
this plan.

## 11. Test Plan Assessment

The proposed model tests cover canonical construction, deterministic ordering,
bounded identity commitments, duplicate and ambiguous bindings, principal
disablement, exact-project grants, expiry posture, additive revocation,
projection exclusion, tamper resistance, redaction, and non-leaking errors.

The implementation phase should additionally ensure:

- the candidate model cannot manufacture `HostedProjectCapability` values
  outside the existing closed enum;
- current-authority projection cannot be trusted through ordinary
  deserialization;
- equivalent unordered grant inputs cannot produce distinct commitments;
- disabling a provider, binding, or principal dominates otherwise active
  grants; and
- no Debug or serialization fixture exposes a raw external subject even when
  the fixture resembles ordinary non-secret user data.

Runtime, persistence, migration, concurrency, clock, key-rotation, and active
session revocation tests remain correctly deferred.

## 12. Blockers

None for the model-only implementation.

OIDC transport, persistence, hosted cutover, or collaborative inbox work would
be blocked because their dedicated protocol, migration, threat-model, and
runtime enforcement phases do not yet exist.

## 13. Non-Blocking Follow-Ups

- Choose whether the first model permits exactly one active external binding
  per principal; the conservative default should be one.
- Define an explicit untrusted serialized projection type if persisted
  projection evidence is later required.
- Keep project authority and runtime capability authority distinct and require
  explicit composition when both are needed.
- Define trusted-time semantics only in the persistence phase.
- Decide the administrative capability for grant issuance and revocation before
  implementing those operations.
- Preserve historical route visibility for audit while preventing a revoked
  recipient from using the route as current authority.

## 14. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.
- Review-only scope check: passed; the phase adds only this review document.
  The repo-local `.workflow-os/` runtime directory remains untracked and is not
  part of the phase diff.

## 15. Recommended Next Phase

Implement the collaborative identity lifecycle Core model only:

- provider trust registration vocabulary;
- opaque external-subject binding;
- principal lifecycle;
- immutable exact-project grant;
- create-only grant revocation; and
- payload-free point-in-time current-authority projection.

Do not implement external identity transport, PostgreSQL identity storage,
hosted authentication cutover, principal inboxes, directory synchronization,
groups, administration, provider writes, workflow schema changes, or production
identity claims in that phase.

## 16. Governed Review Evidence

- Dogfood workflow: `dg/review`.
- Run: `run-1792538963812172000-2`.
- Approval: `approval/run-1792538963812172000-2/review-scope-approved`.
- Approval presentation: `presentation/507dfe52a13fb205`.
- Approval outcome: granted by the delegated maintainer with persisted
  presentation proof.
- Phase status: completed.
- Event summary: 39 events, including one approval request, one approval
  grant, six governed review steps, zero retries, and zero escalations.
- Out-of-kernel work: code and documentation inspection, review authoring,
  validation, and later git/PR operations are performed by the delegated
  maintainer; the kernel governs scope and records the approval/event trail.
