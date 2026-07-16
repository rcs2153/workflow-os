# Runtime Proportional-Governance Reassessment Helper Blocker Fix Report

## 1. Executive Summary

The pure immutable-bundle reassessment helper now includes explicit validated
runtime escalation in each exact step fact. Static and live escalation posture
compose monotonically on both execution and disclosure axes before assessment.
The missing immutable-definition boundary tests are also present.

## 2. Blocker Fixed

The original helper could derive escalation only from static workflow and step
policy declarations. A live runtime escalation could not raise posture or
invalidate the fingerprint. `StepGovernanceRuntimeFacts` now supports an
explicit runtime escalation through `with_runtime_escalation`.

## 3. Composition Boundary

The shared resolved-definition derivation selects the stricter execution and
disclosure requirement independently between static declarations and explicit
runtime escalation. Quiet posture cannot weaken static requirements. Visible,
approval, denied, and unsupported posture can only hold or raise the result
through the accepted selector.

The existing step-fact constructor remains source-compatible; callers opt into
an explicit runtime escalation with the builder method.

## 4. Fingerprint Behavior

Runtime escalation is part of the accepted workload-assessment input, so every
decision-relevant escalation alters the per-step input fingerprint and the
aggregate set fingerprint. Explicit quiet escalation produces the same result
as no additional escalation.

Tests also prove:

- changing a referenced immutable workflow definition changes the aggregate
  fingerprint;
- adding an unreferenced policy outside the bundle does not change it.

## 5. Privacy And Scope

The fix adds no raw values, payloads, paths, or provider data. Existing
redaction-safe `Debug` and stable bounded errors remain unchanged. No executor,
durable binding, event, persistence, schema, CLI, UI, provider, write, approval,
enterprise, or default behavior was added.

## 6. Test Coverage

Seven focused integration tests and the framing unit test pass. New coverage
includes runtime-escalation monotonicity, escalation fingerprint invalidation,
relevant-definition invalidation, and unreferenced-definition stability.

## 7. Validation Commands

- Focused reassessment tests: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 8. Remaining Limitations

- The helper does not prove runtime-fact freshness.
- Assessment fingerprints are not durably bound.
- No executor path consumes or enforces the result.
- Retry and resume reassessment remain future work.

## 9. Recommended Next Phase

Perform a focused re-review of the blocker fix. If accepted, proceed to the
durable assessment-binding model and event vocabulary only.

## 10. Governed Phase Record

- Dogfood workflow: `dg/blocker`
- Run ID: `run-1784185642868236000-2`
- Approval ID: `approval/run-1784185642868236000-2/fix-approved`
- Approval outcome: granted with persisted presentation proof
- Out-of-kernel work: Rust and test edits, focused validation, documentation
  updates, diff inspection, and report drafting
