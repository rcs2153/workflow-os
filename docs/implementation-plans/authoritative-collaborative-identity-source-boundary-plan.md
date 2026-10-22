# Authoritative Collaborative Identity Source Boundary Plan

Status: planning, focused maintainer/security review, persistent fact-store and
Core-owned exact-query source implementation, and phase-level review accepted

Related foundations:

- [Collaborative Identity Lifecycle Plan](collaborative-identity-lifecycle-plan.md)
- [Production Current-Authority Source Boundary Plan](production-current-authority-source-boundary-plan.md)
- [Collaborative Identity Core Model Blocker Fix Review](../concepts/COLLABORATIVE_IDENTITY_CORE_MODEL_BLOCKER_FIX_REVIEW.md)
- [Collaborative Team Beta Threat Model](../security/collaborative-team-beta-threat-model.md)
- [Collaborative Team Beta](../runtime/collaborative-team-beta.md)

## 1. Executive Summary

The collaborative identity model now represents provider registrations, opaque
principal bindings, principal lifecycle, exact-project grants, and additive
revocations. Its public projection helper deliberately produces a
non-authorizing claim because caller-assembled facts cannot prove provenance,
completeness, trusted time, or current authority.

The next boundary must make those guarantees real. Workflow OS should add one
Core-owned PostgreSQL authority source that persists collaborative identity
facts, reads a complete exact query in one transaction at database time,
enforces a monotonic organization authority revision, and exposes the result
only through the existing `CurrentAuthoritySource` contracts. One consequential
approval-decision boundary should then consume that same-call result before an
`ApprovalDecide` action can proceed.

The governing invariant is:

```text
Persisted identity facts are history.
Only a complete, current, Core-owned source read may authorize an action.
```

This phase does not add OIDC, SCIM, hosted authentication cutover, groups,
administration, provider writes, workflow schema fields, or production identity
claims.

Implementation status: Workflow OS now has the closed collaborative query and
complete identity fact-family vocabulary, a managed PostgreSQL V3 schema for
complete canonical authority revisions, monotonic activation, rollback and
same-revision conflict rejection, database-time exact reads, restart-safe
reconstruction, and payload-free source snapshot evidence. The implementation
uses a separate versioned collaborative request sharing the existing source
registration and snapshot commitment machinery, preserving the V1 required-
context request unchanged. No approval-decision consumer or hosted identity
cutover is implemented.

The first consumer is now specified, but not implemented, in the
[Collaborative ApprovalDecide Consumer
Plan](collaborative-approval-decide-consumer-plan.md). It extends the existing
PostgreSQL approval-decision transaction so source resolution, watermark
serialization, decision persistence, and a payload-free authority receipt form
one atomic boundary. The first implementation phase has extracted a private
transaction-scoped source read while preserving the public non-authorizing
read. That internal seam is pending focused review and does not yet lock the
authority watermark, persist a receipt, or authorize an approval decision.

The phase-level review required and applied five integrity corrections before
acceptance: organization-scoped activation locking for the first-revision race,
durable source-model version verification, fail-closed ambiguous matching
grants, canonical closed fact-family ordering, and query-minimized emitted fact
counts. Restored-state rehearsal now also checks exact replay, negative read
posture, and rollback rejection.

## 2. Why This Phase Is Next

The accepted blocker fix removed a dangerous semantic shortcut: internally
coherent caller input is not trusted authority. Leaving the model permanently
disconnected from a source would preserve safety but provide no usable identity
governance.

The repository already has the necessary architectural foundations:

- domain-neutral `CurrentAuthoritySourceRegistration`, request, snapshot,
  watermark, completeness, freshness, and failure vocabulary;
- a Core-owned registered source proof and same-call consumption pattern;
- PostgreSQL database time through `clock_timestamp()`;
- serializable transactions and create-only immutable records;
- organization authority high-watermark activation with rollback and
  same-revision conflict rejection;
- exact project resource bindings;
- immutable approval route history and transactional current-authority checks;
  and
- the collaborative identity lifecycle models.

The correct next step is composition of those foundations, not another
parallel source abstraction or caller-supplied projection type.

## 3. Goals

- Persist collaborative provider, binding, principal, grant, and revocation
  facts as canonical, validated, create-only records.
- Reconcile exact replay and reject conflicting reuse of logical identities.
- Bind every accepted identity mutation to a monotonic organization authority
  revision.
- Reject revision rollback and same-revision content conflict.
- Resolve trusted observation time from PostgreSQL, not the caller.
- Read one complete exact organization/project/principal fact set in one
  transaction.
- Reuse the existing current-authority source registration, request, snapshot,
  watermark, completeness, freshness, and failure vocabulary.
- Establish explicit proof that the snapshot is complete for its exact query.
- Resolve revocations and lifecycle state at the database observation time.
- Consume the snapshot in the same call as one exact approval-decision
  authorization check.
- Fail closed for unknown, incomplete, stale, rolled-back, ambiguous,
  unavailable, or concurrently changed authority.
- Preserve bounded, payload-free, redaction-safe errors and Debug output.
- Remain restart-safe and compatible with PostgreSQL backup, recovery, and
  schema-health checks.

## 4. Non-Goals

This plan does not authorize:

- OIDC, OAuth, SAML, WebAuthn, or provider assertion validation;
- SCIM, groups, invitations, automatic provisioning, or directory sync;
- hosted login, session issuance, bearer replacement, or authentication
  cutover;
- a general role language or organization-wide administration UI;
- dynamic delegation, emergency access, quorum approval, or break-glass access;
- multiple hostile tenant trust domains;
- workflow schema, CLI, example, or public API configuration changes;
- general capability issuance or reusable authority leases;
- background refresh, caches, or eventual-consistency authorization;
- broad approval inboxes or notification delivery;
- provider writes, adapter mutations, or external side effects;
- a second current-authority framework;
- accepting `CollaborativeAuthorityProjectionClaim` as authority; or
- production identity, security certification, or enterprise-readiness claims.

## 5. Existing Model Reuse

The implementation should reuse, not duplicate:

| Existing primitive | Collaborative use |
| --- | --- |
| `CurrentAuthoritySourceRegistration` | Core-owned registration for the PostgreSQL collaborative identity source |
| `CurrentAuthoritySourceRequest` | Existing commitment/freshness request pattern; it requires a minimal versioned query-binding extension before it can represent collaborative identity |
| `CurrentAuthoritySourceSnapshot` | Provenance, watermark, observation window, completeness, freshness, and counts |
| `CurrentAuthoritySourceWatermark` | Opaque commitment to organization revision and source snapshot |
| `CurrentAuthoritySourceCompleteness::CompleteForExactQuery` | Required posture before authorization |
| `CurrentAuthoritySourceFailure` | Stable unavailable/incomplete/stale/conflict failure boundary |
| collaborative identity records | Canonical persisted lifecycle facts |
| hosted authority high watermark | Pattern for rollback and same-revision conflict rejection |
| project scope and capability vocabulary | Exact project and `ApprovalDecide` subject |

The current source model is not used unchanged. Its V1 request is bound to a
`RequiredContextExecutionBinding` and `RequiredContextContractBinding`, and its
fact-family vocabulary has no identity lifecycle or revocation families. Those
are meaningful type boundaries, not documentation details. The first
implementation must add the smallest versioned, domain-neutral extension that
can commit an exact Core-derived collaborative authority query and count all
authority-bearing fact families. It must preserve the existing V1 required-
context path unchanged.

The non-authorizing collaborative claim remains useful for validation and
canonicalization tests. It must not become the source response, a trusted
snapshot, or an authorization token.

### 5.1 Required Generic Source-Model Extension

The persistent-source vertical slice should include a V2 query binding rather
than forcing collaborative subjects into required-context hashes. The new
model should distinguish at least:

- the existing required-context execution query; and
- an exact collaborative project-capability authority query committed from
  organization, project, local principal, and capability.

The collaborative query value must be constructed by Core from trusted route
and authenticated-principal context. A serialized request is evidence of a
request, not permission and not a reusable authorization token.

The fact-family vocabulary should gain domain-neutral families sufficient to
prove and count the complete read, such as identity-provider registration,
principal identity binding, principal lifecycle, capability grants, and grant
revocations. Exact names are an implementation-review decision, but collapsing
all five into `CapabilityGrants` is not acceptable completeness evidence.

This extension belongs in the same reviewed persistent-source implementation.
It is not a separate caller-facing projection phase and does not authorize a
generic user-defined source query language.

## 6. Authoritative Source Ownership

The source must be constructed and registered by Core from a trusted
`PostgresStateBackend`. Callers may supply query subjects such as organization,
project, and principal identities, but they must not supply:

- provider or binding records;
- lifecycle status;
- grants or revocations;
- authority revision;
- completeness assertions;
- source watermark;
- observation time; or
- an already-resolved authority projection.

The public boundary should accept only the exact action subject and a registered
source handle owned by the trusted composition root. A raw trait object supplied
by an untrusted caller must not establish source trust.

## 7. Persistence Model

The first implementation should add managed PostgreSQL tables for:

1. collaborative identity provider registrations;
2. opaque principal identity bindings;
3. collaborative principal lifecycle records;
4. exact-project grant records;
5. additive grant revocation records; and
6. collaborative identity authority high watermarks.

Each fact table should store:

- its canonical logical identity;
- the validated canonical payload;
- a canonical payload hash;
- organization identity and index subjects required for exact reads;
- the organization authority revision that introduced the fact; and
- database insertion time.

Duplicated query columns must be cross-checked against the canonical decoded
record on every read. A mismatch is corruption, not a recoverable absence.

Create-only reconciliation rules:

- an exact canonical replay returns the existing record;
- the same logical identity with different content fails as conflict;
- historical records are never rewritten or deleted;
- lifecycle change is represented by a higher-revision record;
- revocation is additive and cannot be reversed in place; and
- a transaction may activate a new organization revision only after all facts
  belonging to that revision are durably present and coherent.

## 8. Revision And Rollback Protection

`CollaborativeAuthorityRevision` is the organization-level ordering boundary.
The source must maintain a durable high watermark containing:

- organization identity;
- accepted authority revision;
- canonical revision commitment;
- source model version; and
- database acceptance time.

Activation rules:

- a lower revision than the high watermark fails closed;
- the same revision with the same commitment reconciles;
- the same revision with a different commitment fails as conflict;
- a higher revision advances atomically only after complete fact validation;
- a snapshot may authorize only from the current accepted revision; and
- restore/recovery must preserve and verify the high watermark.

The commitment should cover the complete set of fact identities and canonical
payload hashes associated with the revision, sorted deterministically. It must
not include raw external subjects, credentials, or provider assertions.

## 9. Trusted Time

Authority time comes from PostgreSQL `clock_timestamp()` inside the same source
transaction. Callers may not choose `observed_at`, grant activation time, expiry
evaluation time, or snapshot time.

The transaction should capture one database observation value and use it for:

- principal lifecycle evaluation;
- grant `not_before` and expiry evaluation;
- revocation effectiveness;
- snapshot observation window;
- freshness classification; and
- resulting decision metadata.

Application-wall-clock values may be used only in tests of rejected caller
input or for non-authorizing display. They must not affect authorization.

## 10. Exact Query And Completeness

The first query is intentionally narrow and requires the versioned query
binding described in Section 5.1:

```text
organization + project + local principal + capability ApprovalDecide
```

Core derives the collaborative query binding and the current-authority request.
It must not repurpose a required-context contract hash as an identity query.
The source transaction must read:

- the one active provider registration referenced by the binding;
- every current binding relevant to the local principal and provider subject
  uniqueness constraint;
- the current principal lifecycle record;
- every grant for the exact organization/project/principal tuple;
- every revocation for those grants;
- the active organization authority high watermark; and
- database time.

`CompleteForExactQuery` may be asserted only when every required table and
index read succeeded under one transaction and the facts all belong to the
accepted revision domain. Empty grants are a complete negative result. Missing
tables, corrupt rows, uncertain revision, mixed revisions, or failed reads are
source failures, not negative authority.

## 11. Source Snapshot And Watermark

The source should produce a versioned `CurrentAuthoritySourceSnapshot` using
the accepted generic source model whose:

- source registration identifies the Core-owned PostgreSQL source;
- fact families explicitly cover and count provider, binding, principal,
  grant, and revocation authority through the minimal domain-neutral extension;
- request commits the exact organization/project/principal/capability query;
- watermark commits the accepted revision and canonical revision commitment;
- read window uses database time;
- completeness is `CompleteForExactQuery` only after all checks pass;
- fact counts are bounded and exact; and
- freshness is evaluated against a conservative source contract.

The collaborating resolver may additionally construct a private authorizing
view for same-call use. That view must not be serializable, persistable,
cloneable into a reusable permit, or constructible from the public
non-authorizing claim.

## 12. First Runtime Consumer

The first consumer should be one exact project approval-decision authorization
boundary for `HostedProjectCapability::ApprovalDecide`.

The boundary should:

1. receive authenticated local principal identity from the existing trusted
   hosted composition root;
2. derive the exact organization/project/principal/capability query;
3. read the Core-owned source in the same call;
4. require complete, fresh, current, unambiguous facts;
5. require active provider, active unique binding, active principal, and one
   active matching exact-project grant not defeated by revocation;
6. recheck the source watermark immediately before committing the decision;
7. record a bounded authority snapshot commitment with the approval decision;
   and
8. fail before decision persistence or run resumption when authority is not
   proven.

This does not replace approval evidence, presentation proof, policy, immutable
run context, required checks, or high-assurance controls. Identity authority is
one prerequisite, not the entire decision gate.

The implementation must not broaden legacy `decide_approval` APIs silently.
Use an explicit opt-in collaborative boundary or the existing collaborative
hosted route where project scope is already mandatory.

## 13. Concurrent Change Semantics

The authorization decision and mutable authority check must share a
serializable transaction or an equivalent compare-and-commit boundary.

Before decision persistence, Core must confirm:

- the high-watermark revision and commitment still match the snapshot;
- the principal remains active;
- the binding remains active and unambiguous;
- the grant remains active for database time;
- no effective revocation now exists; and
- the project/run/approval binding remains exact and pending.

Serialization conflict, revision advance, revocation arrival, or lifecycle
change causes a bounded retry or fail-closed reassessment. No approval decision
may commit under the old snapshot.

## 14. Failure Taxonomy

Stable failures should distinguish at least:

- source not registered;
- source unavailable;
- exact query invalid;
- incomplete fact set;
- authority revision missing;
- rollback detected;
- same-revision conflict;
- mixed-revision snapshot;
- row corruption;
- provider inactive;
- binding missing, inactive, or ambiguous;
- principal inactive;
- grant missing, inactive, expired, or not yet active;
- grant revoked;
- snapshot stale or future-dated;
- authority changed before commit; and
- approval no longer pending.

Errors must use stable codes and generic bounded messages. They must not include
provider subjects, local principal values, token material, emails, usernames,
paths, SQL, connection strings, canonical payloads, or grant metadata.

## 15. Privacy And Redaction

- External provider subjects remain opaque hashes or opaque bounded identifiers.
- Credentials and protocol assertions are never stored in identity fact tables.
- Canonical commitments are one-way identifiers, not payload containers.
- Debug output redacts source requests, fact content, watermarks, and database
  details.
- Serialization exposes persisted fact records only where already explicitly
  supported; it never exposes the private authorizing view.
- Audit and approval records cite source/snapshot/revision commitments rather
  than copying identity payloads.
- Exact-project denial responses remain non-disclosing at hosted boundaries.

## 16. Recovery And Migration

The managed schema migration must be additive and versioned. Recovery rehearsal
should verify:

- all identity fact payload hashes;
- all duplicated index columns;
- revision commitments and high watermarks;
- exact replay behavior;
- rollback and same-revision conflict rejection;
- a complete negative query;
- an active positive query; and
- post-restore approval-decision authority reassessment.

Workflow OS must not rebuild authority by selecting the highest visible fact
revision without verifying the stored revision commitment. Recovery-required
posture remains fail-closed.

## 17. Implementation Sequence

### Phase 1: Persistent Fact Store And Source Read

- Add the minimal versioned source query-binding and fact-family extension
  required by Section 5.1 while preserving existing V1 behavior.
- Add the PostgreSQL identity fact and high-watermark schema.
- Implement validated create/reconcile methods.
- Implement revision activation and rollback protection.
- Implement one private Core-owned exact-query source adapter using database
  time and the existing current-authority source model.
- Add restart, recovery, corruption, exact replay, and concurrency tests.
- Do not add a runtime consumer yet.

### Phase 2: Same-Call Approval-Decision Consumption

- Add one explicit collaborative approval-decision composition boundary.
- Resolve exact authority and require `ApprovalDecide` in the same call.
- Compare the source watermark before decision commit.
- Persist only bounded authority commitments with the decision/audit record.
- Prove revocation and revision races fail before approval commit.

### Phase 3: Focused Maintainer And Security Review

- Review source ownership, exact-query completeness, trusted time, transaction
  isolation, rollback protection, corruption checks, privacy, and non-bypass.
- Require PostgreSQL conformance and recovery evidence.
- Do not broaden identity consumers until accepted.

### Later, Separately Planned

- provider protocol validation;
- hosted authentication and session issuance;
- principal approval inbox filtering;
- invitations, groups, directories, and administration;
- additional capability consumers; and
- production operations and hostile-tenant isolation.

## 18. Test Plan

Future implementation tests must cover:

- canonical persistence and exact replay for every identity fact family;
- conflicting logical-identity reuse;
- lower revision rollback rejection;
- same-revision commitment conflict;
- higher-revision atomic activation;
- database-time grant activation and expiry;
- complete positive exact query;
- complete negative exact query;
- missing/corrupt/mixed-revision source failure;
- provider inactive;
- missing, inactive, and ambiguous binding;
- principal inactive;
- grant absent, wrong project, wrong capability, not-yet-active, and expired;
- revocation before read;
- revocation between read and decision commit;
- revision advance between read and decision commit;
- one successful `ApprovalDecide` use;
- no decision/event/run resumption on failed authority;
- no acceptance of `CollaborativeAuthorityProjectionClaim` as authority;
- no substitution of required-context hashes for collaborative identity query
  subjects;
- source-model V1 compatibility and V2 query-substitution rejection;
- restart and restored-state equivalence;
- payload-free audit commitment;
- Debug, error, and serialization non-leakage;
- PostgreSQL conformance and recovery rehearsal; and
- all existing collaborative identity, approval route, hosted, executor, and
  workspace tests.

## 19. Open Questions

- Should revision activation require one explicit manifest record listing every
  fact hash, or derive that manifest inside the activation transaction?
- Should provider and principal lifecycle changes share one organization
  revision stream with grants, or use subdomain revisions committed by one
  organization vector?
- Which existing hosted approval-decision route is narrowest without coupling
  identity authority to the legacy bearer registry?
- Should the generic source query extension be a closed enum in the existing
  request model or a new versioned request type sharing registration and
  snapshot commitments? It must remain closed and Core-constructed either way.
- Should a complete negative result be represented only by fact counts and
  bounded reason, or also by a private empty authorizing view?
- What conservative freshness duration is appropriate when the decision path
  already rereads the watermark in the commit transaction?
- When should the existing static hosted principal registry become migration
  input rather than runtime authority?

These questions must be resolved in the Phase 1 implementation prompt or its
focused plan review. They do not justify broadening this plan.

## 20. Final Recommendation

The focused maintainer/security review is accepted after clarifying the required
versioned generic source query binding and fact-family extensions. Implement
**Phase 1: Persistent Fact Store And Source Read** as the next code phase.

The first implementation must be a bounded PostgreSQL vertical slice that
persists collaborative identity facts and proves a complete Core-owned exact
query through the existing current-authority source boundary. It must not be
another model-only projection, and it must not yet change hosted authentication
or approval execution.

See [Authoritative Collaborative Identity Source Boundary Plan
Review](../concepts/AUTHORITATIVE_COLLABORATIVE_IDENTITY_SOURCE_BOUNDARY_PLAN_REVIEW.md).
