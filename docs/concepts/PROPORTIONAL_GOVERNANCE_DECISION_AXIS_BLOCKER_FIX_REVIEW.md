# Proportional Governance Decision Axis Blocker Fix Review

## 1. Executive Verdict

**Blocker fixed; proportional-governance decision-axis correction accepted.**

The accepted model now separates execution disposition from disclosure while
enforcing the necessary cross-axis invariant for approval and denial.

## 2. Scope Verification

The fix stayed within the approved blocker scope. It added one normalization
boundary, focused regressions, and truthful documentation updates. It did not
add workload inference, runtime behavior, persistence, events, CLI or UI,
schemas, provider mutations, automatic approvals, hosted administration, or
release changes.

## 3. Fix Assessment

`normalize_cross_axis_posture` preserves quiet or visible disclosure for
`Proceed`. It monotonically promotes disclosure to `Visible` for
`RequireApproval` and `Denied`. The helper runs for source requirements and
again after prior-decision minima, covering direct declarations and runtime
reassessment.

This is the correct narrow fix. It does not reintroduce `VisibleDisclosure` as
an execution mode. It recognizes only that approval presentation and denial
diagnostics cannot be satisfied by an accepted quiet posture.

## 4. Serde Assessment

Validated decision and projection deserialization call the same cross-axis
consistency predicate. Serialized approval/quiet and denial/quiet combinations
fail closed with fixed non-leaking errors. Consistent decisions retain round-trip
behavior.

## 5. Test Assessment

The new regressions directly cover:

- approval plus quiet normalization;
- denial plus quiet normalization;
- serialized approval plus quiet rejection;
- serialized denial plus quiet rejection.

The full focused suite continues to cover axis independence, monotonicity,
profiles, steward minima, reasons, projection consistency, payload-free output,
and unknown-value non-leakage.

## 6. Validation

All required validation passed:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `git diff --check`

## 7. Remaining Blockers

None for the decision-axis correction.

## 8. Non-Blocking Follow-Ups

- Review direct input-enum deserialization before schema exposure.
- Implement deterministic workload assessment and input fingerprinting only in
  its separately governed phase.
- Define future presentation surfaces without making them policy engines.

## 9. Recommended Next Phase

Resume the immutable run-bundle in-memory builder, which supplies canonical
definition roots needed by later workload-assessment invalidation. After that
boundary is accepted, implement the model-only workload assessment and
fingerprint helper. Runtime proportional-governance integration remains later.

## 10. Governed Review Evidence

- Dogfood workflow: `dg/review`.
- Run ID: `run-1783921769374641000-2`.
- Approval ID:
  `approval/run-1783921769374641000-2/review-scope-approved`.
- Approval presentation: `presentation/7adafdb8afb970e8`.
- Approval outcome: granted with persisted presentation proof.
- Phase status: completed.
- Out-of-kernel work: Codex reviewed source, tests, reports, and validation
  results and authored this document. The kernel coordinated governance only.
- Report posture: no WorkReport artifact was generated or persisted.
