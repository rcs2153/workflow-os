# Collaborative Identity Lifecycle Plan

Status: planning only; no identity runtime or persistence is implemented by this plan

## 1. Executive Summary

Workflow OS already proves one bounded collaborative trust domain with
pre-provisioned principals, exact project grants, project-scoped approval
routing, durable authority commitments, and PostgreSQL rollback protection.
That foundation is deliberately static. It does not yet authenticate an
external identity provider, model a principal lifecycle, revoke a live grant,
or re-evaluate revocation at an authority-consuming boundary.

The next collaborative identity milestone should establish a domain-neutral
identity and revocable-grant contract before any OIDC transport or hosted
administration is added. The future invariant is:

```text
Authentication proves one current external identity assertion.
An explicit local binding maps that assertion to one Workflow OS principal.
Current, scoped, revocable grants determine authority at time of use.
Historical routes, approvals, events, and reports remain evidence, not authority.
```

The first implementation after review should be model-only. It should define
trusted identity-provider registration, opaque external-subject binding,
principal lifecycle, immutable project-grant issuance, create-only revocation,
and current-authority projection vocabulary with deterministic validation and
redaction-safe serialization. It must not parse tokens, call an identity
provider, migrate PostgreSQL, change hosted authentication, expose a UI or CLI,
or claim multi-tenant or enterprise readiness.

## 2. Why This Belongs In Workflow OS

Workflow OS cannot safely route approvals, delegate capabilities, or govern
collaborative work if authentication and authority are ambient deployment
assumptions. A bearer token that maps to one configured actor is sufficient for
the current single-trust-domain evaluation, but it cannot answer:

- which external identity was authenticated;
- which local principal that identity is bound to;
- whether the binding is current, disabled, or superseded;
- which exact organization and project grants are current;
- whether a grant expired or was revoked after a session began;
- which authority revision was used for a decision;
- whether an approval route recipient still has decision authority; or
- whether historical evidence is being mistaken for present authority.

These are governance questions, not merely web authentication concerns. Core
must own the domain semantics while hosted adapters own protocol validation and
deployment integration.

## 3. Product Invariant

The collaborative identity boundary must preserve these separations:

| Concept | Source of truth | Does not prove |
| --- | --- | --- |
| External identity assertion | configured identity-provider adapter | local principal authority |
| Local principal binding | Workflow OS identity binding | project access or capability grant |
| Principal lifecycle | current principal record | any particular project grant |
| Project grant | immutable scoped grant plus current lifecycle | authentication freshness |
| Revocation | create-only revocation fact and current authority projection | deletion of history |
| Hosted session | bounded authenticated request context | reusable workflow authority |
| Approval route | historical routing evidence | current decision authority |
| Capability grant | exact runtime capability authority | identity proof by itself |
| Authority receipt | evidence of a point-in-time governed decision | future reusable authority |
| Audit event or WorkReport | durable evidence and handoff | permission to act |

No route, prior approval, report citation, session, cached projection, or model
opinion may replace a current grant check at an authority-consuming boundary.

## 4. Goals

- Define one domain-neutral external identity-provider trust registration.
- Bind an opaque external subject to exactly one local Workflow OS principal in
  one organization trust domain.
- Model principal activation, disablement, and supersession without deleting
  historical identity evidence.
- Model immutable, exact-project grants with bounded capability sets,
  effective time, expiry posture, issuer, and authority revision.
- Model create-only grant revocation with deterministic conflict and replay
  semantics.
- Require fresh current-authority projection before approval decisions,
  capability use, provider mutation, or other consequential actions.
- Preserve the existing canonical hosted authority snapshot and rollback
  protection as the deployment commitment boundary.
- Keep approval routing as historical evidence and require an independent
  current-authority check at read and decision time.
- Make privacy, redaction, retention, and non-leaking failure behavior explicit.
- Preserve local-first single-user operation without requiring OIDC.
- Prepare a later principal-filtered collaborative approval inbox without
  authorizing it in this plan.

## 5. Non-Goals

This plan does not authorize:

- Rust implementation in this planning phase;
- JWT or ID-token parsing, signature verification, JWKS retrieval, discovery,
  login redirects, logout, refresh, or session cookies;
- OIDC, OAuth, SAML, SCIM, LDAP, directory, group, invitation, or role-provider
  integration;
- PostgreSQL schema or migration changes;
- dynamic hosted configuration reload;
- an administrator UI, user UI, CLI, SDK, or workflow schema change;
- automatic user creation or just-in-time grant issuance;
- deriving authority from email domains, display names, groups, or model
  inference;
- tenant isolation, hostile-tenant claims, row-level security, or enterprise
  production readiness;
- quorum approval, separation-of-duties policy, break-glass operation, or
  emergency access;
- credential storage, access-material resolution, provider writes, OpenShell,
  nested harness execution, or new mutation families;
- cryptographic authority receipts or portable cross-deployment identity;
- retroactive deletion or rewriting of historical audit, approval, route, or
  report records; or
- release posture changes.

## 6. Existing Foundations To Reuse

The future implementation should compose, not replace:

- `HostedPrincipalBinding`, `HostedProjectGrant`, and the exact project
  capability vocabulary;
- `HostedPrincipalRegistry` and the canonical deployment-owned sanitized
  authority view;
- `HostedAuthorityRegistryRevision`, the deployment authority commitment, and
  PostgreSQL high-watermark rollback protection;
- `ProjectApprovalRouteRecord` as immutable historical routing evidence;
- the authenticated hosted composer that reconstructs routes from durable
  events and immutable run bundles;
- capability grants, availability, resolution, requests, step-scoped
  projection, and authority-receipt vocabulary;
- immutable run bundles and resolved-context commitments;
- approval-presentation proof and decision-time reassessment;
- audit, EvidenceReference, SideEffect, and WorkReport boundaries; and
- the selected PostgreSQL shared-state semantic contract.

The current deployment bearer token remains an evaluation-only compatibility
path until an OIDC-backed path is separately implemented, reviewed, migrated,
and explicitly selected. This plan must not silently reinterpret it as an OIDC
session or production authority.

## 7. Trust-Domain Boundary

The first collaborative identity implementation should remain inside one
configured organization trust domain. The deployment must pre-register the
accepted identity-provider issuer and audiences. An authenticated external
subject may bind only to a principal in that same organization.

The first boundary must reject:

- an unregistered issuer or audience;
- a missing, empty, malformed, or unbounded subject;
- one external subject bound to multiple active principals;
- one active principal ambiguously bound to multiple external subjects unless
  a later explicit multi-identity policy is designed;
- cross-organization bindings or grants;
- caller-selected organization or project scope that conflicts with the
  authenticated binding;
- unknown capability values;
- stale, expired, disabled, revoked, or rollback-conflicting authority; and
- any attempt to infer grants from external claims not explicitly mapped by a
  reviewed deployment policy.

Multiple hostile tenants, federation between organizations, and cross-domain
delegation require a separate threat model and are out of scope.

## 8. Candidate Core Model

The first model-only implementation should add only types with concrete
validation rules. Names may follow repository conventions, but the semantic
roles must remain distinct.

### 8.1 Identity Provider Trust Registration

Candidate vocabulary:

- `CollaborativeIdentityProviderId`;
- `CollaborativeIdentityProviderRegistration`;
- registration version and positive revision;
- exact organization scope;
- canonical issuer identity;
- bounded accepted audience set;
- allowed protocol/profile version;
- subject-commitment algorithm;
- lifecycle posture: `Active` or `Disabled`; and
- redaction/sensitivity posture.

The model should not store signing keys, JWKS documents, discovery payloads,
client credentials, raw tokens, arbitrary claims, or network configuration.
Protocol adapters may later validate those inputs and provide a trusted opaque
assertion to Core.

### 8.2 Opaque External Subject Binding

Candidate vocabulary:

- `ExternalIdentitySubjectCommitment`;
- `PrincipalIdentityBindingId`;
- provider registration identity and revision;
- exact organization and local principal identity;
- opaque subject commitment rather than raw subject text;
- issued-at and optional supersession posture;
- binding lifecycle: `Active`, `Disabled`, or `Superseded`; and
- source and actor references for the governed administrative decision.

Email address, display name, username, group list, full subject claim, and raw
provider attributes must not be required durable fields. A future operator UI
may maintain separate display metadata under a separately reviewed privacy
policy; it is not authority.

### 8.3 Principal Lifecycle

Candidate vocabulary:

- `CollaborativePrincipalRecord`;
- exact organization and actor/principal identity;
- lifecycle: `Active` or `Disabled`;
- positive principal revision;
- effective time;
- optional payload-free supersession or disablement reference; and
- create-only historical record identity.

Disabling a principal immediately prevents new authority projection but does
not erase old events, approvals, routes, receipts, or reports.

### 8.4 Revocable Project Grant

Candidate vocabulary:

- `CollaborativeProjectGrantId`;
- exact organization, project, and principal identity;
- ordered bounded capability set using existing hosted project capabilities;
- issuer/administrative actor reference;
- grant revision and authority-registry revision;
- effective-at and optional expires-at;
- optional delegation provenance, initially non-delegable;
- sensitivity ceiling where applicable; and
- create-only canonical grant commitment.

The grant must not contain credentials, raw OIDC claims, policy payloads,
approval text, evidence payloads, provider output, or unrestricted metadata.

### 8.5 Grant Revocation

Candidate vocabulary:

- `CollaborativeProjectGrantRevocationId`;
- exact grant identity and commitment;
- exact organization, project, and principal binding;
- revoked-at using trusted deployment/database time;
- bounded enumerated reason category;
- revoking actor/authority reference; and
- create-only revocation commitment.

Revocation is additive. It never updates or deletes the original grant. Exact
replay reconciles; conflicting second revocation fails closed. A revocation
must become visible to every later authority projection even when a previously
authenticated session remains otherwise valid.

### 8.6 Current Collaborative Authority Projection

A payload-free projection should bind:

- provider registration and revision;
- opaque subject-binding identity and revision;
- principal identity and current lifecycle revision;
- organization and project;
- exact active grant identities and commitments;
- authority-registry revision and commitment;
- evaluation time and trusted-time provenance;
- resulting ordered capability set;
- expiry/revocation posture; and
- a versioned aggregate commitment.

The projection is current evidence for one bounded decision call. It is not a
reusable bearer capability and must not authorize a later call without fresh
re-evaluation.

## 9. Authentication Adapter Boundary

OIDC protocol handling belongs in a hosted adapter, not in generic Core. A
future adapter should use a mature standards implementation and must validate,
at minimum:

- configured issuer and audience;
- signature and allowed algorithm;
- expiry and not-before with bounded clock skew;
- subject presence and bounds;
- provider-registration lifecycle and revision;
- token-use/profile constraints appropriate to the selected flow; and
- replay/nonce posture where the selected flow requires it.

After validation, the adapter should pass only a private, non-serializable,
same-call authenticated assertion into Core. Core then resolves the opaque
subject binding, principal lifecycle, and current grants. Neither serialized
claims nor an HTTP header may construct trusted Core identity directly.

This plan does not choose an OIDC crate, flow, or hosted endpoint. Dependency,
flow, protocol, and threat-model review are required before adapter work.

## 10. Grant Issuance And Revocation Semantics

Future grant issuance must be an explicit governed administrative operation:

1. authenticate the administrative actor;
2. prove current administration authority through a separately accepted
   policy boundary;
3. validate the exact organization, project, principal, and capability set;
4. bind the current identity and authority-registry revisions;
5. create the immutable grant and its audit event atomically;
6. advance the canonical authority snapshot/high watermark; and
7. return only stable references and bounded posture.

Future revocation follows the same shape but creates a revocation record and
new authority revision. Revocation must not depend on deleting sessions or
waiting for their natural expiry. Every consequential operation must obtain a
fresh authority projection or compare against a freshly resolved revision.

The model-only first phase must not implement either operation.

## 11. Time-Of-Use Enforcement

Authentication and authorization occur at different times. A valid session may
outlive a grant or principal lifecycle change. Therefore later runtime paths
must re-evaluate current authority:

- before presenting or deciding an approval;
- before consuming a scoped capability;
- before provider invocation or SideEffect transition;
- before exposing project-scoped inbox records;
- before issuing any future delegated authority; and
- after restart or retry before resuming a consequential operation.

Immutable run bundles preserve what a run was authorized to request. They do
not freeze current actor authority forever. Historical approval routes remain
discoverable evidence but must be filtered and decision-checked against current
authority.

## 12. Routing And Notification Relationship

The existing project approval route remains immutable historical evidence. An
identity lifecycle does not mutate old routes when a recipient is disabled or
a grant is revoked.

A future principal-filtered inbox must:

- authenticate the requesting principal through the accepted identity path;
- resolve current exact-project `ApprovalDecide` authority;
- query only exact organization/project/principal route indexes;
- label stale historical routes as unavailable when current authority no
  longer permits action;
- independently validate current authority again when a decision is submitted;
- preserve approval-presentation proof and resolved-context integrity; and
- avoid exposing whether principals or routes exist in another project.

External notification delivery remains a separate SideEffect family and is not
authorized by this plan.

## 13. Durable State And Revision Posture

The first model-only phase has no persistence. A later PostgreSQL phase should
use append-only/create-only tables for provider registrations, subject
bindings, principal revisions, grants, and revocations, plus a canonical
organization authority high watermark.

That phase must define:

- transactional grant/revocation plus audit/event projection;
- exact replay and conflict semantics;
- monotonic organization authority revision;
- rollback and same-revision conflict rejection;
- deterministic current-grant queries;
- bounded, indexed exact-scope reads;
- restart, concurrency, backup/restore, migration, and corruption proof;
- trusted database-time semantics for effective/expiry/revocation checks; and
- compatibility with existing hosted principal and route records.

No implementation may store raw tokens, complete provider claims, signing
material, access material, email addresses by default, or identity-provider
response payloads.

## 14. Failure Behavior

The boundary must fail closed with stable, non-leaking codes for:

- unknown or disabled provider registration;
- invalid or stale assertion;
- missing, ambiguous, disabled, or superseded subject binding;
- disabled principal;
- missing exact-project grant;
- expired or revoked grant;
- capability absent from the active grant set;
- organization/project mismatch;
- authority rollback or revision conflict;
- corrupt or incomplete durable identity state; and
- inability to obtain fresh current authority.

Errors and Debug output must not echo issuer URLs where sensitive, raw subject
values, emails, claim values, grant contents, directory/group membership,
credentials, headers, tokens, database values, or provider payloads.

## 15. Privacy And Retention

- Persist opaque subject commitments instead of raw subject values where
  practical.
- Keep display metadata separate from authority records.
- Require explicit retention and deletion posture before storing optional
  human-readable identity data.
- Preserve append-only governance evidence even after access is revoked,
  subject to future lawful retention policy.
- Do not place identity-provider claims in audit events, EvidenceReference
  summaries, WorkReports, diagnostics, or logs.
- Treat issuer, subject-binding, principal, project, and grant relationships as
  sensitive even when all records are payload-free.
- Ensure serialized untrusted claims never deserialize into a trusted
  authenticated assertion or current-authority projection.

## 16. Compatibility And Migration Posture

The existing single-actor hosted alpha and static collaborative registry are
preview compatibility surfaces, not production identity. A later adoption plan
must decide whether to:

- retain the bearer path only for explicit local evaluation;
- add OIDC as an opt-in parallel path during migration; or
- cut over one hosted deployment after its grants are imported and verified.

There must be no silent conversion of bearer identities into external subject
bindings or revocable grants. Migration requires an explicit dry run,
principal/grant mapping review, count and commitment verification, rollback
posture, and bounded report.

Local single-user Core and CLI operation must continue without OIDC. Local
actors remain explicit local identities under existing policy and do not become
federated principals implicitly.

## 17. Candidate Implementation Sequence

1. Implement the Core identity-provider, opaque subject-binding, principal,
   immutable grant, revocation, and current-authority projection model only.
2. Perform focused maintainer and security review of validation, trust
   boundaries, wire posture, privacy, and compatibility.
3. Plan and implement create-only PostgreSQL identity/grant/revocation storage
   with authority-revision and recovery conformance.
4. Review and select one OIDC protocol/library profile and write a dedicated
   threat model before implementing the adapter.
5. Add one explicit, opt-in hosted authentication path for read-only and
   no-write evaluation behavior only.
6. Enforce fresh revocable authority at one approval-inbox read and one
   approval-decision boundary.
7. Prove restart, revocation-during-session, expiry, rollback, concurrency,
   cross-project isolation, and privacy behavior.
8. Only after review, consider replacing the evaluation bearer path or using
   identity for additional capability/provider boundaries.

Provider mutations, enterprise administration, hostile multi-tenancy, dynamic
groups, and automatic delegated authority remain later separately approved
milestones.

## 18. Test Plan

The model phase should prove:

- valid provider registration and canonical serialization;
- invalid issuer/audience/profile/lifecycle input fails closed;
- opaque subject-binding identity is deterministic and bounded;
- duplicate active subject or principal bindings are rejected by the future
  aggregate helper rather than silently selected;
- principal disablement is representable without deleting history;
- grants require exact organization, project, principal, capabilities,
  revision, and effective-time posture;
- empty, duplicate, unknown, unbounded, expired, or reversed-time grant input
  fails closed;
- revocation binds the exact grant and cannot silently replace it;
- exact revocation replay is distinguishable from conflicting revocation;
- current-authority projection excludes disabled, expired, and revoked grants;
- projection is deterministic for ordered equivalent input;
- serialized untrusted input does not become a trusted authenticated assertion
  or current authority;
- Debug and serialization do not expose secret-like or PII-like fixture values;
- validation errors use stable codes and do not echo caller values;
- existing hosted principal, routing, capability, approval, audit, and runtime
  tests remain green; and
- documentation remains honest about the model-only boundary.

Later persistence and adapter phases need separate tests for migration,
concurrency, restart, rollback, token validation, key rotation, revocation
during an active session, cross-project isolation, and backup/restore.

## 19. Open Questions

- Should one principal initially support exactly one active external subject
  binding, or should multiple providers be modeled before implementation?
- Which OIDC flow and token profile should the first hosted evaluation accept?
- Should provider registration revision and organization authority revision be
  one atomic deployment revision or separately committed domains?
- Which administrative capability may issue and revoke project grants?
- How should emergency access be modeled without creating ambient bypass?
- What maximum session age is acceptable when every consequential action still
  rechecks authority?
- Which optional display metadata may be stored, where, and under what
  retention/deletion policy?
- How should an existing static `HostedPrincipalBinding` migrate to a
  principal plus opaque external subject binding?
- Should a revoked recipient's pending route remain visible as historical
  evidence to administrators while hidden from the former recipient?
- When is SCIM provisioning justified, and how can it remain separate from
  decision authority?

These questions do not block the model-only first phase where the conservative
answer is explicit single-provider binding, no automatic provisioning, no
delegation, and fresh authority at every consequential use.

## 20. Final Recommendation

Proceed next with the collaborative identity lifecycle Core model only, after
focused review of this plan. The model should establish opaque identity
binding, immutable scoped grants, additive revocation, and point-in-time
authority projection without protocol or persistence behavior.

Do not build OIDC transport, PostgreSQL identity tables, hosted cutover,
principal inboxes, administration, groups, provider writes, OpenShell, nested
harnesses, or production identity claims in that first implementation.

## 21. Implementation Status

The collaborative identity lifecycle Core model is now implemented in
`workflow-core`. It provides validated provider registrations, opaque external
subject bindings, principal lifecycle records, immutable exact-project grants,
create-only revocations, aggregate active-binding ambiguity checks, and a
validated point-in-time authority projection claim.

The phase review found that the original public projection API described
caller-assembled facts as trusted current authority. The blocker fix demotes
that result to an explicitly non-authorizing claim. It proves internal
coherence only; authoritative provenance, complete current fact coverage, and
trusted time remain prerequisites for any future trusted authority resolver.

The implementation remains model-only. It does not validate external identity
protocols, persist identity or grant state, change hosted authentication,
expose a principal inbox, add directory synchronization or groups, execute
provider writes, change workflow schemas, add CLI behavior, or claim production
identity readiness. The blocker fix and focused review are complete.

Authoritative source-boundary planning is now documented in
[Authoritative Collaborative Identity Source Boundary Plan](authoritative-collaborative-identity-source-boundary-plan.md).
The first persistent source slice is implemented and awaits phase-level review;
it does not yet authorize approval decisions or replace hosted authentication.
It reuses the existing generic current-authority source contracts and recommends
a PostgreSQL persistent fact-store and Core-owned exact-query source as the
first implementation. No authoritative source or runtime identity consumer is
implemented yet.
