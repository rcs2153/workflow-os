# Authoritative Collaborative Identity Source Implementation Review

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups; proceed to planning one
same-call collaborative `ApprovalDecide` consumer.**

The implementation now provides a credible persistent source boundary. Review
found four blocking integrity gaps; all four were corrected and covered before
acceptance. No runtime decision consumer or hosted identity cutover was added.

## 2. Scope Verification

The phase stayed within the approved source-only boundary. It added a closed
collaborative query, identity fact-family vocabulary, PostgreSQL persistence,
revision activation, exact reads, tests, and documentation.

It did not add approval-decision consumption, OIDC, OAuth, SAML, SCIM, groups,
directories, invitations, hosted sessions, identity administration, workflow
schema, CLI behavior, examples, provider mutations, or production identity
claims.

## 3. Source Ownership Assessment

The source is owned by `PostgresStateBackend`, not supplied as an arbitrary
runtime trait object. Callers can construct validated revision candidates for
trusted ingestion and exact query subjects for reads, but neither candidate nor
read authorizes an action. No runtime API consumes the result as permission.

The activation API must remain confined to trusted composition and deployment
setup until an independently reviewed administration boundary exists.

## 4. Query And Fact-Family Assessment

The implementation preserves the existing required-context V1 request. A
separate closed V1 collaborative query commits organization, project,
principal, and capability without hiding identity subjects in unrelated
context hashes.

Provider registration, principal binding, principal lifecycle, capability
grant, and grant-revocation families are represented and counted separately.
The emitted counts are now limited to the exact query slice rather than
disclosing organization-wide inventory size.

## 5. Candidate Integrity Assessment

Revision candidates are canonicalized and bounded. They reject missing core
identity families, duplicate logical identities, cross-organization facts,
dangling bindings, missing grant principals, and incoherent revocations.
Commitments cover the complete canonical revision.

The candidate remains validated ingestion input, not proof of current
authority. Its caller-supplied evaluation helper is explicitly
non-authorizing.

## 6. Persistence And Revision Assessment

PostgreSQL V3 stores immutable canonical rows keyed by organization, revision,
family, and logical identity. Payload hashes and duplicated query columns are
verified during reconstruction. The high watermark binds revision,
commitment, source-model version, and database acceptance time.

Activation rejects rollback and conflicting same-revision reuse. Exact replay
reconstructs and compares durable content. Higher revision insertion and
watermark advance occur in one serializable transaction.

## 7. Concurrency Blocker And Fix

The initial implementation locked an existing watermark row with `FOR UPDATE`.
For an organization's first activation there was no row to lock, so concurrent
identical writers could race and one could fail on immutable fact insertion
instead of reconciling.

The fix acquires an organization-scoped transaction advisory lock before
reading the watermark. The PostgreSQL conformance scenario now launches two
concurrent first activations and requires exactly one `Activated` and one
`ReconciledExisting` result.

## 8. Source-Version Blocker And Fix

The accepted plan required the high watermark to bind a source model version.
The initial schema omitted it. V3 now persists `source_model_version`; both
activation and reads reject unknown durable versions as corruption. The
conformance scenario covers version tampering.

## 9. Exact-Read And Ambiguity Blocker And Fix

The source captures PostgreSQL time once inside the serializable read, verifies
the active revision commitment, reconstructs validated facts, and evaluates
provider, binding, principal, grant, expiry, and revocation posture at that
single time.

The initial match posture treated any positive matching-grant count as present.
That could hide duplicate active authority. The fixed boundary accepts exactly
zero as a complete negative, exactly one as present, and fails closed for more
than one active matching grant.

## 10. Privacy And Disclosure Blocker And Fix

Canonical payloads remain inside the source. Read evidence contains
commitments, revision, match posture, exact-query counts, and source metadata;
it does not copy external subjects or provider assertions. Debug output redacts
identity and commitment values, and errors are stable and non-leaking.

The initial snapshot counts described the full organization inventory. Counts
are now derived only from bindings, providers, principal records, grants, and
revocations relevant to the exact query.

## 11. Recovery Assessment

Restart reads, payload-hash verification, duplicated-column checks, and
revision commitment verification are present. Review extended the restored-
database rehearsal to require collaborative exact replay, a revoked negative
read, and rollback rejection.

Local execution compiles these PostgreSQL paths but skips live operations when
the configured PostgreSQL fixtures are absent. CI remains responsible for the
live conformance and recovery runs.

## 12. Compatibility Assessment

The required-context source request and existing source snapshot semantics are
preserved. Schema V1-to-V3 and V2-to-V3 upgrades are additive. No workflow spec
or external serialization contract was changed.

The implementation uses one family-tagged canonical fact table rather than
five physical fact tables. This is acceptable for the bounded first slice
because every family is decoded into its closed type, duplicated columns are
cross-checked, family counts remain distinct, and revision commitment covers
the complete inventory.

## 13. Test Quality Assessment

Coverage now includes canonical matching, expiry, revocation, dangling facts,
deterministic commitments, safe Debug and serialization, migration, concurrent
first activation, exact replay, conflict, higher-revision revocation, rollback,
restart, source-version corruption, restored-state equivalence, and existing
workspace regressions.

The principal residual limitation is environmental: local PostgreSQL tests
returned through the documented skip path because no live fixture was
configured. This is disclosed rather than treated as live database evidence.

## 14. Blockers

None after the concurrency, source-version, ambiguity, and count-disclosure
fixes.

## 15. Non-Blocking Follow-Ups

- Keep revision activation restricted to trusted composition/deployment setup;
  do not expose it as general identity administration.
- Plan an indexed exact-read optimization only if it preserves complete
  provider-subject and binding-ambiguity detection.
- Run and retain CI evidence for live PostgreSQL conformance and recovery.
- Define the later same-call consumer's final watermark recheck and atomic
  approval-decision persistence before implementation.

## 16. Recommended Next Phase

Plan one **same-call collaborative `ApprovalDecide` consumer**. It must derive
the exact query from trusted route and principal context, consume the Core-owned
source in the same call, require exactly one active grant, revalidate the
revision immediately before decision persistence, and persist only bounded
source commitments.

The next phase must still not add hosted authentication cutover, identity
protocols, directories, general administration, another capability consumer,
provider writes, or production identity claims.

## 17. Validation

- `cargo fmt --all --check`: passed after blocker fixes.
- Focused collaborative source and PostgreSQL targets: passed; live PostgreSQL
  operations were environment-skipped locally.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed; one explicitly opt-in live GitHub sandbox
  test remained ignored as designed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 18. Governed Phase Evidence

- Dogfood workflow: `dg/review`.
- Run: `run-1792648911147314000-2`.
- Approval: `approval/run-1792648911147314000-2/review-scope-approved`.
- Approval presentation: `presentation/b1ea417f92aad77f`.
- Approval outcome: granted under delegated-maintainer authority with persisted
  presentation proof.
- Phase status: completed.
- Event summary: 39 events, including one approval request, one approval grant,
  six scheduled and successful review steps, no retries, and no escalations.
- Out-of-kernel work: repository inspection, blocker fixes, validation,
  review writing, git, and later PR actions are performed by the delegated
  maintainer; Workflow OS governs scope and records the phase trail.
