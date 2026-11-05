# Project Resource Authorization Continuity Review

## Verdict

Accepted for exact-head CI and independent parent-versus-fix evaluation.

The Gate 3 candidate binds the exact current GitHub repository-metadata project
restriction into the existing approval resolved-context commitment and checks
that commitment before approval acceptance and immediately before provider
admission. The review found no authorization bypass, access-before-check path,
or payload disclosure blocker.

This verdict does not close Gate 3. Exact-head CI and an independent rerun that
reproduces the old drift behavior on the parent and rejection on the fix remain
required.

## Security Review

The reviewed commitment is domain-separated and binds:

- stable project identity;
- exact run and approval identity;
- provider family, operation, resource kind, enabled posture, endpoint, and
  non-secret access-reference identity;
- the sorted exact repository resource set;
- current workflow, skill, and referenced-policy content hashes; and
- the provider fixture commitment when fixture mode is used.

Resource ordering is canonical. Duplicate resources remain invalid. Unrelated
display metadata is intentionally outside the commitment.

Approval acceptance reconstructs the commitment before `ApprovalGranted` is
persisted. Atomic fixture and opt-in live-read admission reconstruct it before
transient access resolution, provider invocation, adapter telemetry, or any
SideEffect creation. Both checks require the current commitment to match the
immutable run bundle and exact approval request.

The atomic path resolves the approval attached to the exact step. A missing,
ungranted, replaced, or later incompatible approval fails closed. Because the
approval identity participates in the commitment, another approval for the
same step cannot silently inherit the original authority.

## Review Finding And Correction

The implementation committed endpoint, access-reference identity, and stable
project identity, but the first public regression matrix did not independently
exercise those fields. The review added three cases:

- an unsupported endpoint substitution fails project validation before
  approval;
- an unsupported access-reference substitution fails project validation before
  approval; and
- stable project-identity drift fails the resolved-context comparison before
  approval.

The corrected GitHub read-only target passes all 15 tests. No production logic
changed as a result of this evidence correction.

## Validation

Passed:

- full Core all-targets/all-features suite, including 785 unit tests;
- full CLI all-targets suite, including 179 primary CLI tests and all
  provider/example targets;
- GitHub read-only target, 15 passed;
- atomic opt-in live-read drift regression;
- warnings-denied workspace Clippy;
- warnings-denied workspace rustdoc;
- workspace formatting;
- documentation checks;
- release-state consistency for `0.4.0-preview.1`; and
- diff-integrity checks.

Opt-in credential-backed live transport was not run and is not claimed by this
review.

## Governed Review Record

- workflow: `dg/review`;
- run: `run-1793888533325941000-2`;
- approval: `approval/run-1793888533325941000-2/review-scope-approved`;
- approval presentation: `presentation/b5dc974800364672`;
- presentation content hash:
  `b5dc97480036467290225c73026b4843e0487b28d9c1e71ab0f1db4269703c7c`;
- decision: granted under delegated-maintainer authority after the complete
  handoff was presented; and
- state root: `/private/tmp/workflow-os-gate3-review-state`.

Phase close reported `Completed` with 39 events, one approval, zero retries,
and zero escalations. The approval event exposes the matching persisted
presentation proof marker.

Repository inspection, test execution, the evidence correction, and this
review were performed outside the kernel by the delegated maintainer. The
kernel coordinated approval and phase state; it did not execute or fabricate
the review evidence.

## Remaining Gate 3 Evidence

Before Gate 3 closes:

1. merge only the reviewed exact head after required GitHub checks pass;
2. run the independent drift scenario against the parent commit and preserve
   the old incorrect behavior as negative evidence;
3. run the same scenario against the fix and preserve fail-closed behavior; and
4. reconcile the roadmap and product contract only after those results are
   durable and reviewable.
