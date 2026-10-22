# First-Use CLI And Integration Check Verification Report

## 1. Executive Summary

The active delivery queue asked for implementation of two reported first-use
defects: duplicate missing-manifest output from `workflow-os validate` and an
opaque `check:integrations` child-process failure under Node 24. Current-main
verification found that both defects were already fixed, tested, reviewed, and
merged in PR #344.

No implementation change was justified. This phase closes the stale queue item
with direct regression and cross-version execution evidence instead of
manufacturing duplicate code changes.

## 2. Scope Completed

This phase:

- traced both reported symptoms to the accepted Fresh-Pull Evaluator UX fix;
- inspected the current CLI diagnostic and integration-helper implementations;
- confirmed focused regressions remain present;
- ran the complete integration contract gate under Node 20 and Node 24;
- corrected the authoritative active delivery queue; and
- preserved the dedicated draft-pull-request sandbox proof as dependency-
  tracked work.

## 3. Prior Fix Provenance

PR #344, merged as `88e38664`, already:

- stopped `validate` from reattaching diagnostics that it had rendered;
- required exactly one `loader.manifest_missing` occurrence across stdout and
  stderr;
- added an explicit 16 MiB child-process output bound;
- distinguished spawn and output exhaustion from nonzero exits;
- included bounded stdout and stderr for actionable command failures; and
- added focused integration-helper success, failure, and overflow tests.

The accepted implementation and review are documented in:

- [Fresh-Pull Evaluator UX And Tooling Fix Report](FRESH_PULL_EVALUATOR_UX_AND_TOOLING_FIX_REPORT.md)
- [Fresh-Pull Evaluator UX And Tooling Fix Review](FRESH_PULL_EVALUATOR_UX_AND_TOOLING_FIX_REVIEW.md)

## 4. Verification Results

Current main was verified with:

- the CLI regression that requires one missing-manifest diagnostic;
- the integration-helper unit tests;
- `npm run check:integrations` under Node 20.19.5; and
- `npm run check:integrations` under Node 24.18.0.

Both complete integration runs passed. The Node 24 failure was not
reproducible because the bounded child-process correction is already present.

## 5. Scope Explicitly Not Changed

This verification did not change:

- CLI validation or exit semantics;
- supported Node declarations;
- workflow schemas or SDK contracts;
- runtime authority, approvals, evidence, reports, or SideEffects;
- provider calls or mutations;
- persistence or hosted behavior; or
- release posture.

## 6. Governed Execution

- workflow ID: `dg/implement`
- run ID: `run-1792687745805119000-2`
- approval ID: `approval/run-1792687745805119000-2/implementation-approved`
- presentation ID: `presentation/937795e9e3c6c173`
- approval outcome: granted by delegated maintainer with persisted presentation
  proof
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- approval-presentation enforcement: proof enforced with one matching persisted
  presentation record and event marker
- kernel boundary: governance coordination only; verification, documentation,
  and Git operations ran outside the kernel

## 7. Validation

The phase ran:

- focused CLI missing-manifest regression;
- integration-helper unit tests under Node 20 and Node 24;
- complete `check:integrations` under Node 20 and Node 24;
- `npm run check:docs`; and
- `git diff --check`.

All listed checks passed. No Rust implementation changed, so workspace format,
clippy, and full workspace test repetition were not required by the bounded
verification outcome. The focused Rust CLI regression was run directly.

## 8. Remaining Limitations

- Node 20 remains the required CI toolchain; newer declared-compatible Node
  versions are not a required CI matrix.
- Child-process output remains intentionally bounded.
- The dedicated draft-pull-request live sandbox proof still lacks verified
  least-privilege access and remains dependency-tracked.

## 9. Recommended Next Phase

Run governed active-queue selection using current accepted evidence. Do not
select another historical `next phase` statement automatically, and do not
broaden provider mutations while the dedicated sandbox proof remains blocked.
