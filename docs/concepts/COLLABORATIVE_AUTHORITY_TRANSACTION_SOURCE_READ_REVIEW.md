# Collaborative Authority Transaction Source Read Review

## 1. Executive Verdict

Phase accepted with a mandatory precondition before the consequential
`ApprovalDecide` consumer. Proceed to the payload-free receipt core model and
persistence phase only.

The refactor preserves the existing public non-authorizing read and creates a
minimal internal transaction composition seam. It does not introduce a current
authorization bypass, public permit, receipt, approval mutation, or run
resumption path.

## 2. Scope Verification

The phase stayed within its approved internal-refactor scope:

- one module-private transaction-scoped source-read helper was extracted;
- the existing public read still owns a retrying serializable transaction;
- no public model or method was added;
- no authority receipt model or table was added;
- no approval-decision consumer or executor callsite was added;
- no watermark lock or claim of consequential authorization was added; and
- no hosted authentication, administration, provider write, schema, CLI, SDK,
  or example surface changed.

## 3. Semantic Equivalence Assessment

The public method previously performed database-time observation, active
watermark lookup, source-version validation, complete candidate reconstruction,
revision commitment checking, and payload-free evidence construction directly
inside `serializable(...)`.

It now calls `read_collaborative_identity_authority_tx(...)` from that same
closure. The helper contains the same operations and error mapping. The public
connection, serializable retry loop, commit behavior, source-model failure,
candidate reconstruction, commitment comparison, and returned type are
unchanged.

Repeat-read conformance coverage compares query commitment, active authority
revision, match posture, and matching active grant count. Existing restart,
revocation, rollback, ambiguity, source-version corruption, and restored-state
tests remain in place.

## 4. Transaction Ownership Assessment

The helper correctly accepts a caller-owned `Transaction<'_>` and does not
open, commit, or retry a nested transaction. It is module-private and therefore
cannot become a public caller-assembled authorization surface.

The helper intentionally does not lock the organization authority watermark or
acquire the organization-scoped advisory lock. Its documentation says so. This
is correct for the current evidence-only public read and insufficient for the
future consequential consumer, which remains unimplemented.

## 5. Mandatory Phase 3 Precondition

The helper currently obtains `clock_timestamp()` internally. The accepted
consumer design requires the outer approval-decision transaction to obtain one
database-time observation and use that same observation for source resolution,
receipt construction, and decision validation.

This is not a current vulnerability because no consequential consumer accepts
the helper's output. It is, however, a mandatory correction before Phase 3:

1. acquire the shared organization advisory lock;
2. start the serializable decision transaction;
3. capture one database timestamp;
4. lock and read the active watermark;
5. pass the timestamp into source resolution;
6. revalidate the locked watermark before atomic persistence; and
7. retry the entire operation on serialization conflict.

Phase 2 may add the non-authorizing receipt model and persistence without this
change. Phase 3 must not compose the current helper directly as an authorizing
decision gate.

## 6. Privacy And Redaction Assessment

The result remains the existing payload-free
`CollaborativeIdentityAuthoritySourceRead`. Debug output redacts query and
snapshot commitments. Serialization exposes only bounded commitments, revision
metadata, source posture, counts, and aggregate source evidence.

Focused PostgreSQL assertions reject exposure of organization, project,
principal, provider, binding, grant, issuer, audience, and subject fixture
values in both Debug and serialized output. Error construction remains stable
and does not include durable row contents or query subjects.

## 7. Test Quality Assessment

Coverage is proportionate to this mechanical internal refactor:

- the public method exercises the extracted helper;
- repeat reads preserve stable evidence fields;
- Debug and serialization remain payload-free;
- existing model tests cover deterministic commitments and exact-match
  behavior;
- existing PostgreSQL conformance covers activation, replay, restart,
  revocation, rollback, and corruption; and
- CI's Shared PostgreSQL State check passed for PR #589.

The test does not compare full snapshots because database evaluation time is
part of each snapshot commitment. That is correct for separate public reads.
Phase 3 needs transaction-level tests proving one captured time observation and
the reviewed lock ordering.

## 8. Validation Results

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- Focused authority-source and PostgreSQL backend tests: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.
- PR #589 required checks: all seven passed, including Shared PostgreSQL State,
  Rust, Hosted Alpha Recovery, Dependency And Security, schema/example
  contracts, read-only integration contracts, and TypeScript/docs.

## 9. Blockers

No blocker remains for Phase 2 receipt core model and persistence.

The single-database-time and lock-order requirements above are blockers for
Phase 3, the consequential approval-decision consumer.

## 10. Non-Blocking Follow-Ups

- Keep the transaction helper module-private.
- Keep its output explicitly non-authorizing.
- Add receipt persistence as a dedicated create-only table rather than a
  generic record store.
- Include recovery and conflicting-replay checks before any executor callsite
  consumes the receipt boundary.

## 11. Recommended Next Phase

Implement Phase 2 only: the minimal payload-free collaborative approval-
authority receipt core model and PostgreSQL persistence. The receipt must be
non-authorizing, create-only, uniquely bound to approval and decision-event
identity, redaction-safe, and restart-verifiable.

Do not add the same-call approval consumer, run resumption, another capability,
hosted identity administration, or provider writes in that phase.

## 12. Governed Review Evidence

- Workflow: `dg/review`.
- Run: `run-1792652726416773000-2`.
- Approval: `approval/run-1792652726416773000-2/review-scope-approved`.
- Approval presentation: `presentation/04354ae9c889f296`.
- Approval outcome: granted under delegated-maintainer authority with persisted
  presentation proof.
- Phase status: completed.
- Event summary: 39 events, 1 approval, 0 retries, and 0 escalations.
- Out-of-kernel work: source and test inspection, validation commands,
  documentation edits, git, and later PR actions are performed by the delegated
  maintainer; Workflow OS governs scope and records the review trail.
