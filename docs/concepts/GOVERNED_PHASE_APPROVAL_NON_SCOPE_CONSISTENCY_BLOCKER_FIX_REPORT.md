# Governed Phase Approval Non-Scope Consistency Blocker Fix Report

## 1. Executive Summary

The repo-local governed phase runner could persist one explicit
`strict_non_goals` value while rendering a different hard-coded
`approval_does_not_allow` value in the approval handoff. That created a
contradictory runtime-composition gate: the approved scope authorized report
artifact persistence while the rendered non-scope prohibited report artifacts
and persistence.

The contradictory gate was denied. No report artifact implementation began
under that approval.

The runner now uses the validated and persisted `strict_non_goals` as the
authoritative `approval_does_not_allow` value. Existing phase-specific defaults
remain the fallback only when explicit non-goals are omitted.

## 2. Blocker Fixed

Before the fix:

- presentation persistence received explicit bounded non-goals;
- the handoff printed those non-goals under `strict_non_goals`;
- the handoff independently printed a phase-wide default under
  `approval_does_not_allow`; and
- the two fields could contradict each other.

After the fix:

- one validated work-context value drives presentation persistence,
  `strict_non_goals`, `approval_does_not_allow`, and the copy-safe approval
  request;
- phase-specific defaults still populate that value when the caller omits
  explicit non-goals; and
- explicit phases can authorize narrowly scoped persistence, schema, or other
  work without an obsolete blanket prohibition.

## 3. Implementation

Changed:

- `scripts/self-governed-benchmark.mjs`
  - `printApprovalHandoff(...)` now derives `approvalNonScope` from
    `workContext.strictNonGoals`.
- `scripts/self-governed-benchmark.test.mjs`
  - focused regression coverage proves an explicit non-goal appears in
    `approval_does_not_allow`;
  - the same handoff does not fall back to the unrelated phase-wide default.

No Rust runtime, approval semantics, artifact store, provider, schema, or
release behavior changed.

## 4. Failure And Privacy Posture

The original runtime-composition approval was explicitly denied:

- workflow ID: `dg/runtime-composition`;
- run ID: `run-1785190616925765000-2`;
- approval ID:
  `approval/run-1785190616925765000-2/composition-approved`;
- presentation ID: `presentation/eee0efe705c12c42`;
- outcome: denied;
- terminal status: failed.

The fix does not relax validation. Work-context fields remain bounded and
secret-like values remain rejected without echo.

## 5. Tests

Added or retained coverage for:

- explicit non-goals mirrored into approval non-scope;
- default non-goals when explicit context is absent in dry-run;
- spec-field-specific fallback;
- complete copy-safe handoff;
- missing context failure;
- secret-like context non-leakage; and
- approval command reason redaction.

## 6. Validation

- `npm run test:dogfood-helper`: passed, 30 tests;
- `git diff --check`: passed.

Broader repository validation follows in the resumed runtime-composition phase.

## 7. Governed Fix

- workflow ID: `dg/blocker`;
- run ID: `run-1785190670993100000-2`;
- approval ID: `approval/run-1785190670993100000-2/fix-approved`;
- presentation ID: `presentation/0adcee31ec48dacb`;
- approval outcome: granted through proof-enforced presentation;
- retries: none;
- escalations: none.

The kernel coordinated validation, approval, and event history. Repo edits and
test commands were executed outside the kernel and are disclosed here.

## 8. Recommended Next Phase

Restart the denied authoritative WorkReport artifact persistence
runtime-composition phase. Confirm its new handoff renders the explicit phase
non-goals identically in both `strict_non_goals` and
`approval_does_not_allow` before approving.
