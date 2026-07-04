# Governed Phase Approval Work Summary Report

## 1. Executive Summary

The governed phase approval work-summary fix is implemented for the repo-local self-governed build helper.

The fix makes material phase approvals answer "what work am I approving?" before an agent asks a maintainer to approve. `phase-start` now supports bounded work-context fields, includes those fields in the emitted `approval_handoff`, and fails closed for live material phase starts when required work context is missing.

This phase was performed as a narrow fix to the dogfood approval boundary itself. The previously emitted underspecified handoff was not treated as sufficient approval for broader runtime work.

## 2. Scope Completed

- Added bounded work-context inputs to `scripts/self-governed-benchmark.mjs`.
- Added `work_summary`, `approved_scope`, `strict_non_goals`, `expected_touched_surfaces`, `validation_required`, and `why_now` to approval handoff output.
- Added live material phase fail-closed behavior for missing required work context.
- Added stable non-leaking error code `dogfood.helper.work_context_missing`.
- Added redaction and bounding checks for work-context input values.
- Updated `AGENTS.md` so agents must not ask for approval from underspecified handoffs.
- Updated roadmap, implementation plan, bug record, and self-governed build benchmark docs.
- Added focused dogfood helper regression tests.

## 3. Scope Explicitly Not Completed

- Runtime approval semantics were not changed.
- Automatic approvals were not added.
- Hidden approvals were not added.
- Repository edits, git operations, PR operations, shell execution, or file writes by the kernel were not added.
- Persistence, report artifacts, workflow schema changes, side-effect modeling, writes, hosted behavior, enterprise approval UI, RBAC, IdP, quorum approval, revocation, and release posture changes were not added.

## 4. Helper API Summary

`phase-start` now accepts the following optional flags:

- `--work-summary`
- `--approved-scope`
- `--strict-non-goals`
- `--expected-touched-surfaces`
- `--validation-required`
- `--why-now`

Dry-run mode may print placeholders for missing work context so agents can see the required shape. Live material phase starts fail closed until the required work context is supplied.

## 5. Missing-Context Behavior

When live `phase-start` is run for a material phase without required work context, the helper exits before printing an approval handoff and reports `dogfood.helper.work_context_missing`.

This prevents a maintainer from seeing a generic approval request that looks ready while failing to describe concrete scope, non-goals, touched surfaces, validation, and why-now rationale.

## 6. Redaction And Privacy Summary

Work-context values are bounded to one line and rejected when they look secret-like. Rejection errors use stable text and do not echo the rejected value.

The helper still treats approval command display as redaction-sensitive and does not paste approval reasons into command text in the handoff display.

## 7. Test Coverage Summary

Focused tests cover:

- dry-run approval handoff field output;
- AGENTS instruction preservation;
- live missing-context fail-closed behavior;
- supplied bounded work context in handoff output;
- secret-like work-context rejection without leakage;
- existing phase mapping, phase-close, command display, and helper behavior.

## 8. Commands Run And Results

Passed:

- `npm run test:dogfood-helper`
- `npm run check:docs`
- `git diff --check`

## 9. Remaining Known Limitations

- Work context is supplied through CLI flags, not a structured phase manifest.
- The helper coordinates governance but does not perform implementation work, shell execution, repository mutation, git operations, or PR operations.
- Existing underspecified governed runs should not be treated as valid approvals for future phase work.
- A maintainer review is still required before relying on this fix for all long-running governed phases.

## 10. Recommended Next Phase

Recommended next phase: governed phase approval work-summary handoff review.

The review should verify that material phase handoffs are specific, bounded, redaction-safe, and fail closed when work context is missing, while preserving the existing runtime approval boundary.
