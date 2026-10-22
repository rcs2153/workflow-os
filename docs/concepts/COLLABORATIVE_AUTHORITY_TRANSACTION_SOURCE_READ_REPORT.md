# Collaborative Authority Transaction Source Read Report

## 1. Executive Summary

Workflow OS now has a private transaction-scoped PostgreSQL helper for reading
the complete current collaborative identity authority source. The existing
public read delegates to this helper inside the same retrying serializable
transaction boundary, preserving its non-authorizing behavior and payload-free
result.

This phase creates only the internal composition seam required by the reviewed
collaborative `ApprovalDecide` consumer plan. It does not lock the authority
watermark for a consequential action, persist an authority receipt, authorize
an approval decision, resume a run, or add a new public API.

## 2. Scope Completed

- Extracted `read_collaborative_identity_authority_tx(...)` as a private helper
  accepting a caller-owned PostgreSQL transaction and exact validated query.
- Preserved database-time evaluation, source-model version checking, complete
  active-revision reconstruction, revision commitment verification, exact
  query evaluation, and payload-free evidence construction.
- Preserved the public
  `PostgresStateBackend::read_collaborative_identity_authority(...)` method and
  its retrying serializable transaction behavior.
- Documented that the helper is evidence-only and does not establish the lock
  order required by the future consequential consumer.
- Extended PostgreSQL conformance coverage for repeat-read equivalence and
  Debug/serialization non-leakage.

## 3. Scope Explicitly Not Completed

- No collaborative approval-decision consumer.
- No authority receipt model, table, or persistence.
- No authority-watermark locking for consequential actions.
- No executor-facing approval callsite or run resumption.
- No changes to legacy approval behavior.
- No hosted authentication, identity administration, OIDC, SCIM, groups, or
  directories.
- No workflow schema, CLI, SDK, example, or provider-write changes.

## 4. Implementation Boundary

The private helper reads database time and the active organization authority
watermark through the caller's transaction, reconstructs and validates the
complete revision, confirms the durable revision commitment, and builds the
existing payload-free source evidence.

The public method still opens and commits through the repository's established
serializable retry wrapper. The helper is private, returns no reusable permit,
and cannot be called across process or persistence boundaries.

## 5. Validation Boundary

The source still fails closed for missing authority, unsupported source model
version, corrupt facts, invalid revision commitment, ambiguous matching grants,
or invalid exact-query posture. Stable error codes remain unchanged.

The helper deliberately does not use `FOR UPDATE` or acquire the organization
advisory lock. Those operations belong to the reviewed same-call decision
transaction and must be implemented only after this phase passes focused
review.

## 6. Privacy And Redaction

The returned model contains commitments, revision metadata, bounded match
posture, count, and payload-free source snapshot evidence. Focused assertions
confirm that Debug and serialized output do not expose organization, project,
principal, provider, binding, grant, issuer, audience, or subject values.

No provider assertion, credential, token, source payload, or database error
detail is added by this refactor.

## 7. Test Coverage

The existing collaborative source conformance path continues to cover
activation, exact replay, current positive and negative reads, restart,
revocation, rollback rejection, and source-model corruption. This phase adds a
repeat public read and compares query commitment, authority revision, match
posture, and active grant count, proving the extracted helper preserves the
observable evidence contract. It also adds explicit Debug and serialization
non-leakage assertions.

## 8. Validation Commands

- `cargo fmt --all --check`: passed after applying canonical formatting.
- `cargo test -p workflow-core --test collaborative_identity_authority_source
  --test postgres_state_backend --no-fail-fast`: passed; 8 tests passed and
  PostgreSQL operations remained environment-gated without a local fixture.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 9. Remaining Limitations

- The helper is not an authorization consumer.
- It does not serialize revision activation against an approval decision.
- No payload-free authority receipt exists yet.
- Local PostgreSQL conformance execution remains dependent on the configured
  test fixture; CI provides the required live database path.
- The later consumer still needs one executor-facing callsite, exact project
  binding proof, durable presentation proof, atomic receipt persistence, and
  duplicate-event-safe run continuation.

## 10. Recommended Next Phase

Perform a focused maintainer/security review of the transaction-scoped source
refactor. Only after acceptance should Phase 2 add the minimal payload-free
authority receipt model and persistence. Do not combine that work with the
Phase 3 consequential consumer.

## 11. Governed Phase Evidence

- Workflow: `dg/implement`.
- Run: `run-1792651793320774000-2`.
- Approval: `approval/run-1792651793320774000-2/implementation-approved`.
- Approval presentation: `presentation/378e7c0403c936c1`.
- Approval outcome: granted under delegated-maintainer authority with persisted
  presentation proof.
- Phase status: completed.
- Event summary: 39 events, 1 approval, 0 retries, and 0 escalations.
- Out-of-kernel work: repository edits, tests, documentation, git, and later PR
  actions are executed by the delegated maintainer; Workflow OS governs scope
  and records the phase trail.
