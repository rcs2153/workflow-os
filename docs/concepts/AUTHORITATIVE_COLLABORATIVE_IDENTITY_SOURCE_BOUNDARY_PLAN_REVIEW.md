# Authoritative Collaborative Identity Source Boundary Plan Review

## 1. Executive Verdict

**Plan accepted with required clarifications applied; proceed to the persistent
fact-store and Core-owned exact-query source implementation.**

The plan now defines a credible boundary between persisted identity history and
authorizing current authority. It reuses the generic source architecture,
requires database time and rollback protection, and begins with a bounded
PostgreSQL source before any hosted identity cutover.

## 2. Scope Verification

The plan remains planning-only. It does not authorize identity protocol
handling, hosted authentication cutover, directories, groups, administration,
provider writes, workflow schemas, CLI behavior, examples, or production
identity claims.

The proposed implementation is narrow enough to review independently:

1. a minimal versioned generic source-model extension;
2. persistent collaborative identity facts and revision activation; and
3. one Core-owned complete exact-query read.

Approval-decision consumption remains a separate second phase after source
review.

## 3. Existing Foundation Assessment

The plan correctly builds on:

- accepted collaborative provider, binding, principal, grant, and revocation
  models;
- the fixed non-authorizing claim boundary;
- current-authority source registration, request, snapshot, freshness,
  completeness, and failure concepts;
- PostgreSQL serializable transactions and database time;
- hosted organization authority high-watermark precedent;
- exact project resource binding; and
- recovery-required and canonical-row integrity posture.

It does not duplicate the current-authority source framework or promote the
public claim into authority.

## 4. Review Correction: Query Binding

The initial plan draft said the existing `CurrentAuthoritySourceRequest` could
carry the collaborative exact query unchanged. That was not accurate. V1 is
constructed from `RequiredContextExecutionBinding` and
`RequiredContextContractBinding`; it has no organization/project/principal/
capability subject.

The plan now requires a minimal versioned, closed query-binding extension that
preserves the V1 required-context path and adds one Core-constructed
collaborative project-capability query. It explicitly rejects hiding identity
subjects inside required-context hashes.

This clarification is blocking for implementation and is now resolved in the
plan.

## 5. Review Correction: Fact Families

The current fact-family enum covers capability grants, capability availability,
and governed context references. It cannot honestly establish that provider,
binding, principal, grant, and revocation reads were all complete.

The plan now requires a minimal domain-neutral fact-family extension and exact
counts for all authority-bearing identity prerequisites. Treating all records
as `CapabilityGrants` is explicitly forbidden.

This clarification is blocking for implementation and is now resolved in the
plan.

## 6. Source Trust Assessment

The source trust boundary is appropriate:

- Core constructs and registers the source from a trusted PostgreSQL backend;
- callers provide subjects, not facts or authority results;
- serialized requests and snapshots remain evidence, not permits;
- a raw caller-supplied trait object cannot establish trusted provenance; and
- the private same-call authorizing view cannot be serialized or reused.

The implementation must preserve the existing registration proof posture when
adding the PostgreSQL source.

## 7. Persistence And Revision Assessment

Create-only canonical records, exact replay, conflicting logical-identity
rejection, additive revocation, and duplicated-column integrity checks match
the repository's established state patterns.

An organization authority high watermark with lower-revision rejection,
same-revision commitment reconciliation, same-revision conflict rejection, and
atomic higher-revision activation is the correct rollback boundary.

The implementation prompt must choose one explicit revision-manifest shape and
prove that incomplete fact insertion cannot activate a revision.

## 8. Trusted Time Assessment

Using PostgreSQL `clock_timestamp()` inside the source transaction is correct.
The caller must not choose grant activation, expiry, revocation effectiveness,
observation, or freshness time. Reusing one captured database observation
value throughout the query prevents internally inconsistent time evaluation.

## 9. Exact Query And Completeness Assessment

The first query is appropriately narrow:

```text
organization + project + local principal + ApprovalDecide
```

The required read set is complete enough to detect provider lifecycle,
binding ambiguity, principal lifecycle, positive and negative grants,
revocations, and authority revision. The plan correctly distinguishes a
complete empty grant result from source failure.

Completeness must be established by the Core-owned adapter after all table and
integrity checks, never accepted from storage or a caller as a flag.

## 10. Runtime Consumption Assessment

Deferring approval-decision use until after source implementation and review is
the safer sequence. The proposed later consumer is appropriately explicit,
exact-project, same-call, and additive to existing approval presentation,
policy, evidence, check, immutable-run, and high-assurance controls.

The consumer must not silently alter legacy local approval APIs or make a
historical approval route record current authority.

## 11. Concurrency And Revocation Assessment

The plan requires a serializable or equivalent compare-and-commit boundary,
with a high-watermark reread before decision persistence. That is sufficient as
the planned invariant: a concurrent revision, revocation, lifecycle change, or
serialization conflict prevents the old snapshot from authorizing a decision.

The source implementation phase must prove concurrent activation and read
behavior even before the decision consumer exists.

## 12. Privacy And Error Assessment

The plan retains opaque external identity, excludes credentials and provider
assertions, uses commitments rather than copied payloads, and requires stable
non-leaking errors and redacted Debug. Hosted negative responses remain
non-disclosing.

The future schema must avoid indexing raw provider subject material where an
opaque digest suffices.

## 13. Recovery Assessment

Adding collaborative identity tables and high watermarks to managed schema
health and logical recovery rehearsal is necessary. The plan correctly rejects
reconstructing authority by merely selecting the numerically highest visible
fact revision.

Recovery tests must verify revision commitments, row integrity, exact positive
and negative queries, rollback rejection, and restored source equivalence.

## 14. Blockers

None after the query-binding and fact-family clarifications were applied.

## 15. Non-Blocking Follow-Ups

- Select the exact closed V2 request representation during implementation.
- Choose an explicit revision-manifest representation and atomic activation
  protocol.
- Keep initial fact insertion available only to trusted test/deployment setup;
  no public administration mutation surface is authorized.
- Select the later approval-decision consumer only after source review.
- Define a conservative freshness bound; same-transaction watermark comparison
  remains mandatory regardless of duration.

## 16. Recommended Next Phase

Implement **Persistent Collaborative Identity Fact Store And Core-Owned Source
Read**.

That phase should include the minimal versioned source-model extension, managed
PostgreSQL tables, canonical create/reconcile operations, revision activation,
database-time exact reads, complete source snapshots, recovery coverage, and
focused tests. It must not add the approval-decision consumer, OIDC, hosted
authentication cutover, directories, administration, provider writes, schemas,
CLI behavior, or production claims.

## 17. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 18. Governed Phase Evidence

- Dogfood workflow: `dg/review`.
- Run: `run-1792647622599411000-2`.
- Approval: `approval/run-1792647622599411000-2/review-scope-approved`.
- Approval presentation: `presentation/4422236168db75e3`.
- Approval outcome: granted under delegated-maintainer authority with persisted
  presentation proof.
- Phase status: completed.
- Event summary: 39 events, including one approval request, one approval grant,
  six governed skill steps, zero retries, and zero escalations.
- Out-of-kernel work: repository inspection, plan correction, review writing,
  documentation validation, and later git/PR actions are performed by the
  delegated maintainer; the kernel governs scope and records the approval and
  event trail.
