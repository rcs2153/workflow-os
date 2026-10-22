# Collaborative ApprovalDecide Consumer Plan

Status: planning and focused maintainer/security review accepted; implementation
not started

Related foundations:

- [Authoritative Collaborative Identity Source Boundary Plan](authoritative-collaborative-identity-source-boundary-plan.md)
- [Authoritative Collaborative Identity Source Implementation Review](../concepts/AUTHORITATIVE_COLLABORATIVE_IDENTITY_SOURCE_IMPLEMENTATION_REVIEW.md)
- [Collaborative Identity Lifecycle Plan](collaborative-identity-lifecycle-plan.md)
- [Collaborative Team Beta Threat Model](../security/collaborative-team-beta-threat-model.md)
- [Project-Scoped Approval Route Persistence Plan](project-scoped-approval-route-persistence-plan.md)
- [Collaborative ApprovalDecide Consumer Plan Review](../concepts/COLLABORATIVE_APPROVAL_DECIDE_CONSUMER_PLAN_REVIEW.md)

## 1. Executive Summary

Workflow OS now has a reviewed Core-owned PostgreSQL source for complete,
current collaborative identity authority. The source can answer one exact
organization/project/principal/capability query at database time and emit
payload-free evidence, but no consequential runtime action consumes it.

The first consumer should be exactly one collaborative project approval-
decision boundary for `HostedProjectCapability::ApprovalDecide`. The boundary
must derive its query from trusted project scope and authenticated principal
context, resolve current authority inside the same PostgreSQL call that records
the decision, and fail before any decision event, approval projection update,
or run resumption when authority is absent, ambiguous, stale, revoked, or
unavailable.

The governing invariant is:

```text
An approval decision may commit only in the same serializable transaction that
proves current exact-project ApprovalDecide authority.
```

This plan does not implement the consumer. It does not change hosted
authentication, create an identity administration surface, broaden approval
APIs, or authorize any provider write.

## 2. Why This Phase Is Next

The persistent source deliberately remains non-consuming. That is safe, but it
leaves collaborative authority disconnected from the decision it is intended
to govern. Adding another source model, projection, or standalone receipt would
not close that gap.

The existing PostgreSQL approval transaction already:

- locks the pending approval projection;
- proves the decision matches the pending request;
- verifies durable approval-presentation proof;
- appends the authoritative approval event; and
- updates the approval projection atomically.

The smallest correct next step is to add current collaborative authority to
that transaction. Performing a source read before calling
`record_approval_decision(...)` would introduce a time-of-check/time-of-use gap
and is therefore not acceptable.

## 3. Goals

- Add one explicit opt-in collaborative `ApprovalDecide` composition boundary.
- Accept authenticated principal identity and exact project scope only from a
  trusted hosted composition root.
- Derive a closed `CollaborativeIdentityAuthorityQuery` in Core.
- Resolve authority with PostgreSQL database time.
- Require one active provider, one active unambiguous binding, one active
  principal, and exactly one active exact-project grant containing
  `ApprovalDecide`.
- Revalidate the active organization authority watermark in the decision
  transaction.
- Lock or otherwise serialize the watermark so a concurrent revision cannot
  produce a decision committed under stale authority.
- Preserve existing approval-presentation, evidence, policy, immutable-run,
  required-check, high-assurance, and requester/approver-separation gates.
- Persist one bounded payload-free authority receipt atomically with the
  approval decision event and approval projection.
- Fail before decision mutation and run resumption when authority cannot be
  proven.
- Use stable, non-leaking errors and redaction-safe Debug behavior.
- Prove restart, replay, concurrency, revocation, and rollback behavior with
  live PostgreSQL tests.

## 4. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- changes to legacy `LocalExecutor::decide_approval(...)` behavior;
- automatic migration of existing approval callers;
- hosted authentication, login, sessions, or bearer-token cutover;
- OIDC, OAuth, SAML, WebAuthn, SCIM, groups, invitations, or directories;
- identity-provider protocol validation or assertion processing;
- identity, grant, or revocation administration APIs;
- a generic capability consumer or reusable authority permit;
- a second capability beyond `ApprovalDecide`;
- another provider mutation or adapter write;
- workflow schema, CLI, SDK, or example changes;
- caches, background refresh, or eventually consistent authorization;
- hosted multi-tenant production claims;
- cryptographic receipts, external notarization, or compliance certification;
- enterprise stewardship UI; or
- reasoning lineage, nested harness execution, or Level 3/4 autonomy changes.

## 5. Selected Consumer Boundary

The first consumer must include an executor-facing composition function with
the conceptual shape:

```text
decide_collaborative_project_approval_with_presentation(request)
    -> CollaborativeApprovalDecisionResult
```

That function should use a PostgreSQL-backed atomic operation with the narrower
conceptual shape:

```text
record_collaborative_approval_decision(request)
    -> CollaborativeApprovalDecisionCommit
```

The backend operation alone is not the accepted runtime consumer. The existing
`PostgresStateBackend::record_approval_decision(...)` is currently a low-level
primitive exercised by persistence tests, not an executor-integrated approval
callsite. The implementation must wire the collaborative operation through one
explicit proof-enforced executor path before claiming a consequential consumer.

The request should contain only already-validated decision material and trusted
subjects required to derive the exact query:

- exact `HostedProjectScope`;
- authenticated local `ActorId` supplied by the trusted composition root;
- updated `ApprovalRequest` containing the proposed decision;
- durable `ApprovalPresentationRecord`;
- authoritative approval-decision `WorkflowRunEvent`; and
- any existing high-assurance or policy proof required by the selected path.

It must not accept provider, binding, principal, grant, or revocation records;
a caller-authored source snapshot or watermark; a caller-assembled
`CollaborativeAuthorityProjectionClaim`; a caller-selected capability; an
observation timestamp; a completeness assertion; or a reusable authority
token. Core fixes the capability to `ApprovalDecide`.

## 6. Exact Query Derivation

The consumer must derive:

```text
organization_id = trusted project scope organization
project_id      = trusted project scope project
principal_id    = authenticated local principal
capability      = ApprovalDecide
```

Before the source read, Core must also prove that:

- the pending approval belongs to the exact durable run and workflow;
- the run has an active exact project binding for the same project scope;
- the authenticated principal is the actor recorded by the proposed decision;
- the approval remains pending;
- the presentation proof matches the pending request; and
- any selected high-assurance requester/approver-separation rule is satisfied.

Caller-provided organization, project, actor, or capability values must not be
allowed to widen or substitute these durable subjects.

## 7. Transaction And Concurrency Model

The implementation must extend the existing PostgreSQL approval-decision
transaction rather than compose two public calls.

One serializable transaction should:

1. read and lock the active collaborative identity authority watermark for the
   organization;
2. obtain one PostgreSQL database timestamp;
3. reconstruct and verify the complete active authority revision;
4. resolve the exact `ApprovalDecide` query at that database timestamp;
5. require complete, fresh, current, unambiguous positive authority;
6. read and lock the pending approval projection;
7. verify the event, presentation, project binding, policy, and selected
   high-assurance controls;
8. re-read or compare the locked watermark immediately before persistence;
9. append the approval-decision event;
10. update the approval projection; and
11. create the payload-free collaborative authority receipt.

The watermark row must be locked in a mode that prevents revision activation
from crossing the authorization-to-commit interval. Serializable retry alone
must not be assumed to detect every logically stale read. If activation and
decision race, PostgreSQL must establish one serial order: a decision
serialized first may commit before activation, while activation serialized
first forces a full reread and reassessment.

No decision may observe an old revision and commit after a newer revision is
already authoritative in the transaction's serial order. Retry must rerun the
entire query, approval validation, and receipt construction. It must never
reuse a prior positive source result.

## 8. Source Read Reuse

The existing public
`PostgresStateBackend::read_collaborative_identity_authority(...)` opens its
own transaction and remains a non-authorizing evidence read. The consumer must
not call it before decision persistence.

Implementation should extract a private transaction-scoped helper that accepts
the active transaction, the Core-derived exact query, and the transaction's
database timestamp. It should return a private non-serializable authorizing
view plus the existing payload-free source evidence. The private view must not
be public, persistable, cloneable into a later permit, or constructible from
serialized evidence.

## 9. Authority Requirements

The same-call source resolution must require:

- one supported Core-owned PostgreSQL source registration;
- one active authority watermark with the expected source-model version;
- a revision commitment matching the complete reconstructed inventory;
- one active provider registration relevant to the binding;
- one active and unambiguous binding to the authenticated local principal;
- one active principal lifecycle record;
- exactly one active exact-project grant containing `ApprovalDecide`;
- no effective revocation for that grant at database time;
- complete exact-query fact families and counts;
- a current, internally consistent source snapshot; and
- exact organization, project, principal, and capability binding.

Zero matching grants is a complete denial. More than one active matching grant
is ambiguity and fails closed even if both grants contain the capability.

## 10. Payload-Free Authority Receipt

The transaction should persist one immutable receipt bound to the approval
decision. A suitable concept is `CollaborativeApprovalAuthorityReceipt`; the
exact name is a review decision.

The receipt should contain only:

- receipt format version;
- approval reference and approval-decision event reference;
- fixed capability `ApprovalDecide`;
- source identifier and source-model version;
- exact-query commitment;
- organization authority revision and revision commitment;
- source snapshot or watermark commitment;
- active grant commitment;
- database observation timestamp;
- decision kind; and
- canonical receipt commitment.

It must not contain external issuer, audience, or subject values; provider
payloads or tokens; raw identity labels when commitments suffice; grant or
revocation payloads; presentation or reason text; workflow contents; or caller
summaries. The receipt is evidence for one decision, not reusable authority.

## 11. Decision Persistence And Run Resumption

The selected boundary authorizes and records one approval decision. It does not
invent a second run-resume mechanism. The ordering remains:

```text
authority proof + approval decision persistence -> existing governed resume
```

If authority resolution or atomic persistence fails, no decision event,
approval mutation, authority receipt, resume event, handler invocation, or
side effect may occur. Retry starts from current durable state.

After a successful commit, the executor-facing composition function must
rehydrate the newly committed approval event and continue through a private
resume-from-committed-decision path. It must not call an existing helper that
appends `ApprovalGranted` or `ApprovalDenied` a second time. A crash after the
decision commit but before resumption leaves an inspectable committed decision
that a restart-safe worker can rehydrate; it does not roll back or duplicate
the decision.

The private source view must not survive the transaction. Subsequent workflow
execution relies on the durable decision and receipt, not on a reusable
identity-authority object.

## 12. Existing Gates Remain Mandatory

Collaborative identity authority is necessary but not sufficient. The consumer
must not bypass or weaken approval-presentation proof, approval evidence,
policy, immutable run context, required checks, proportional governance,
high-assurance requester/approver separation, approval expiry/revocation,
SideEffect disclosure/linkage, or terminal WorkReport obligations.

Implementation should reuse the narrowest existing validated approval path or
factor shared private validation. It must not duplicate a weaker parallel
approval implementation.

## 13. Failure Taxonomy

Stable non-leaking errors should distinguish trusted context mismatch;
project/run binding failure; authenticated-principal/decision-actor mismatch;
source missing or unavailable; unsupported source version; corrupt or changed
watermark; revision commitment mismatch; incomplete/corrupt exact query;
inactive provider; missing/inactive/ambiguous binding; inactive principal;
absent/ambiguous/not-yet-active/expired/revoked grant; non-pending approval;
presentation mismatch; policy or high-assurance failure; transaction retry
exhaustion; receipt conflict/corruption; and event/projection mismatch.

Errors must not echo identifiers, commitments, database values, provider
subjects, grant contents, presentation content, reasons, paths, tokens, or
payloads.

## 14. Privacy And Redaction

- The private authorizing view must have redacted Debug or no Debug.
- Public source evidence and receipts remain payload-free.
- Receipt serialization exposes only documented commitments and closed terms.
- Deserialization validates the canonical commitment and uses generic errors.
- Database failures map to stable Workflow OS error codes.
- Fact counts stay query-minimized and do not expose unrelated inventory.
- No credential, identity assertion, provider response, raw event payload, or
  workflow content may enter the receipt or error path.

## 15. Compatibility Posture

- Existing local approval APIs remain unchanged.
- Existing PostgreSQL `record_approval_decision(...)` remains available for
  accepted non-collaborative paths.
- The new consumer is explicit and opt-in.
- The first implementation changes no workflow spec, public schema, CLI, SDK,
  or example.
- No serialized `CollaborativeAuthorityProjectionClaim` becomes authority.
- Any receipt storage migration must be additive and migration-tested.

## 16. Test Plan

Future implementation tests must cover:

- successful same-call decision and payload-free receipt;
- exact trusted query derivation and actor agreement;
- one active grant and complete no-grant denial;
- wrong project/capability, inactive provider, missing/inactive/ambiguous
  binding, inactive principal, not-yet-active/expired grant, and revocation;
- revision, revocation, binding, and principal lifecycle races;
- retry rereads authority instead of reusing a positive result;
- one decision under concurrent duplicate calls;
- presentation, policy, evidence, and high-assurance gates remain enforced;
- failed authority creates no event, mutation, receipt, resume, invocation, or
  side effect;
- replay/restart-safe receipt reconciliation and corruption failure;
- rejection of caller projections and capability widening;
- Debug, serde, and error non-leakage;
- PostgreSQL migration, conformance, backup/restore, and recovery rehearsal;
- existing identity source, approval persistence/routing, executor, hosted,
  and workspace tests; and
- documentation honesty.

Concurrency tests must use controllable transaction barriers or database hooks
that place a higher revision between initial read and attempted commit.
Timing-only sleeps are not sufficient evidence.

## 17. Implementation Sequence

### Phase 1: Transaction-Scoped Source Refactor

- Extract a private transaction-scoped collaborative source read.
- Preserve the public non-authorizing read unchanged.
- Add focused equivalence and non-leakage tests.

### Phase 2: Receipt Core Model And Persistence

- Add the minimal payload-free receipt model.
- Add additive PostgreSQL persistence and corruption/replay checks.
- Keep the receipt non-authorizing.

### Phase 3: Same-Call ApprovalDecide Consumer

- Add one explicit proof-enforced executor-facing collaborative decision
  request/result boundary.
- Derive the query from trusted scope and principal context.
- Resolve and lock current authority in the approval transaction.
- Preserve all existing gates.
- Atomically persist event, projection, and receipt.
- Rehydrate the committed event and continue without appending a duplicate
  decision event.
- Prove races cannot commit stale authority.

### Phase 4: Focused Maintainer/Security Review

- Review transaction ordering, locking, retries, source completeness, receipt
  privacy, non-bypass, recovery, and regression evidence.
- Do not add another identity consumer before acceptance.

## 18. Documentation Updates For Implementation

Implementation should update this plan, the collaborative identity status,
collaborative runtime and threat-model boundaries, the active roadmap queue,
and an end-of-phase report. Documentation must keep hosted authentication,
identity administration, additional consumers, provider writes, and production
enterprise identity explicitly unimplemented.

## 19. Focused Review Resolutions

The focused maintainer/security review resolved the implementation choices as
follows:

1. Use a dedicated create-only collaborative approval-authority receipt table
   with unique approval and decision-event bindings. Generic records storage
   would obscure the security boundary and exact recovery query.
2. Acquire the same organization-scoped session advisory lock used by revision
   activation before starting the serializable decision transaction, then read
   the watermark row `FOR UPDATE`. The shared advisory lock covers the missing-
   row/first-activation case; the row lock makes the in-transaction comparison
   explicit. Lock acquisition and release must use one reviewed helper and a
   single global order.
3. Reuse the existing PostgreSQL project resource binding table through a
   transaction-scoped exact run-binding read with locking. Do not add a caller-
   asserted project binding.
4. The first executor-facing callsite must require durable approval-
   presentation proof and reuse existing decision preparation. It must not
   claim high-assurance approval unless the selected request also satisfies the
   accepted high-assurance controls. Receipt/report-artifact and provider-write
   behavior remain outside this consumer.
5. Granted and denied decisions both require current `ApprovalDecide`
   authority because either decision changes durable project workflow state.
6. A decision serialized immediately before revocation activation is valid
   only when the shared lock and PostgreSQL transaction establish that order.
   Wall-clock overlap is not enough; a decision that loses that order must
   retry and reassess.
7. The low-level backend transaction is not sufficient product integration.
   Phase 3 must include one explicit executor-facing callsite and a private
   resume-from-committed-decision path that never duplicates the approval
   event.

These resolutions do not authorize broader implementation.

## 20. Final Recommendation

Proceed next to a focused maintainer/security review of this plan. If accepted,
implement Phase 1 only: the private transaction-scoped source read with public
read equivalence tests. Do not combine source refactoring, receipt persistence,
and the consequential approval consumer into one unreviewed code phase.

This is the right next runtime composition step because it turns the reviewed
persistent source into enforceable governance at exactly one security-sensitive
boundary while keeping the product surface narrow.
