# Proportional Governance Workflow Declaration Derivation Blocker Fix Report

## 1. Executive Summary

The workflow-declaration derivation invalidation blocker is fixed. Workflow-level
retry and escalation policy definitions now participate in the selected step's
relevant-definition root alongside step-level policy references.

## 2. Blocker Fixed

Initial review found that changing a workflow-level referenced policy could
leave the definition root unchanged. That made the future reassessment boundary
incomplete even though the workflow declared those policies as part of its
governed path.

## 3. Implementation Approach

The fix passes the resolved workflow into deterministic policy resolution and
adds `retry_policy_refs` and `escalation_policy_refs` to the existing deduplicated
`PolicyId` set. Existing resolution, stable errors, sorting, and root framing are
reused. No new model or public API was added.

## 4. Invalidation Boundary

The relevant-definition root now binds:

- workflow content hash;
- selected step ID;
- resolved skill content hash;
- step requirement, approval, retry, and escalation policy hashes;
- workflow-level retry and escalation policy hashes.

Unrelated policies remain excluded.

## 5. Privacy And Scope

The fix adds no payloads, paths, free-form text, persistence, runtime behavior,
schemas, CLI output, provider calls, or mutations. Missing workflow-level policy
resolution continues to use the same stable non-leaking error boundary.

## 6. Test Coverage

The focused invalidation test now proves:

- resolved skill changes invalidate;
- workflow-level retry policy changes invalidate;
- workflow-level escalation policy changes invalidate;
- unrelated policy changes do not invalidate.

## 7. Validation

- Focused derivation tests: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 8. Governed Evidence

- Dogfood workflow: `dg/blocker`.
- Run ID: `run-1783936076097131000-2`.
- Approval ID: `approval/run-1783936076097131000-2/fix-approved`.
- Approval presentation: `presentation/056ad0408d32fd60`.
- Approval outcome: granted with persisted presentation proof.
- Phase status: completed.
- Out-of-kernel work: Codex implemented and tested the fix; the kernel
  coordinated governance only.
- Report posture: this document is repository phase evidence, not a generated
  or persisted runtime WorkReport artifact.

## 9. Recommended Next Phase

Focused re-review accepted the fix. Proceed to one explicit read-only first-run
recommendation path. Do not add runtime enforcement or broaden provider
mutations.
