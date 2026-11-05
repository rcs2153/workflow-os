# WorkReport Artifact Identity Binding Post-Merge Evidence

## Purpose

This report records exact-head CI and maintainer-operated post-merge evidence
for continuity-of-authority Gate 2, **Verified Means Correctly Bound**. It does
not claim independent assurance and does not close Gate 2.

## Bound Revision

- pull request: `#741`
- reviewed head: `152717f3907cb9101e58ba4bfe855451f20ca177`
- squash-merge commit: `b23a573a5f6314373f7b1df9e136dd0fc71c4dba`
- local `main`: `b23a573a5f6314373f7b1df9e136dd0fc71c4dba`
- `origin/main`: `b23a573a5f6314373f7b1df9e136dd0fc71c4dba`

The reviewed head remained unchanged from CI through squash merge.

## Exact-Head CI

All nine checks attached to the reviewed head completed successfully:

- Dependency And Security;
- Release Packaging Rehearsal;
- Phase 2 Read-Only Integration Contracts;
- Shared PostgreSQL State;
- Hosted Alpha Recovery;
- Schema SDK And Example Contracts;
- Fresh Checkout Aggregate Check;
- Rust; and
- TypeScript And Docs.

The Shared PostgreSQL State job executed the credential-backed relational-key
misplacement and immutable-identity-plus-body-substitution scenario. This is CI
evidence for the exact reviewed head, not a local PostgreSQL claim.

## Post-Merge Exact-Main Rerun

The maintainer reran the following tests from the exact squash-merge commit:

- filesystem rejection of a valid artifact stored under the wrong run/report;
- filesystem location-before-body rejection for combined substitution;
- filesystem immutable-identity-before-body rejection;
- SQLite rejection under wrong relational keys;
- SQLite immutable-identity-before-body rejection;
- CLI inspect and replay rejection before human/JSON projection or an
  `already_persisted` posture;
- PostgreSQL state-backend test compilation; and
- the complete 38-test hosted suite.

Every executed test passed. The rerun modified no tracked repository file and
used only temporary test state.

## Governed Phase Evidence

- PR phase workflow: `dg/pr`
- PR phase run: `run-1793883690490254000-2`
- PR phase approval:
  `approval/run-1793883690490254000-2/pr-scope-approved`
- PR phase presentation: `presentation/ecfc5775c299f041`
- PR phase outcome: completed with 44 events, one approval, zero retries, and
  zero escalations
- acceptance-review workflow: `dg/review`
- acceptance-review run: `run-1793884435215821000-2`
- acceptance-review approval:
  `approval/run-1793884435215821000-2/review-scope-approved`
- acceptance-review presentation: `presentation/d8e4e44a6dd8d583`

Repository edits, Git operations, GitHub transitions, CI observation, and test
execution occurred outside the kernel under the approved phase scopes. The
kernel coordinated governance and retained the phase audit trails; it did not
perform those external actions.

## Verdict

Exact-head CI, credential-backed PostgreSQL execution, unchanged-head merge,
and maintainer-operated post-merge exact-main evidence pass.

Gate 2 remains open because its plan also requires an independent
filesystem/backend-misplacement rerun. This report must not be used to relabel
maintainer testing as independent review. Gate 3 and the cross-gate review
remain blocked until the independent rerun passes and Gate 2 is explicitly
closed.
