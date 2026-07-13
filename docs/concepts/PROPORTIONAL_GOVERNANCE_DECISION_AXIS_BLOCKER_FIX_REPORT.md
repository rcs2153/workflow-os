# Proportional Governance Decision Axis Blocker Fix Report

## 1. Executive Summary

The cross-axis consistency blocker is fixed. Accepted proportional-governance
decisions can no longer pair blocking approval or denial with quiet disclosure.
The selector monotonically normalizes those execution dispositions to visible
disclosure, and deserialization rejects contradictory accepted decisions.

## 2. Blocker Fixed

The public two-axis requirement constructor intentionally permits independent
inputs, including `RequireApproval + QuietAllowed` and `Denied + QuietAllowed`.
The selector previously preserved those pairs. That could make a future
consumer interpret a blocking or denied decision as not requiring visible
presentation.

## 3. Implementation Approach

- Added one bounded cross-axis normalization helper.
- `RequireApproval` and `Denied` normalize disclosure to `Visible`.
- `Proceed` preserves either `Quiet` or `Visible` independently.
- Final selected posture is normalized after prior-decision minima are applied.
- Validated deserialization rejects blocking/denied decisions carrying `Quiet`.

Normalization is preferable to rejecting source requirements because it is a
deterministic monotonic escalation and preserves the stricter declared
execution requirement.

## 4. Validation Boundary

The accepted invariants are now:

```text
Proceed          -> Quiet | Visible
RequireApproval  -> Visible
Denied           -> Visible
```

The projection continues to derive blocking operator action only from execution
disposition. Visible disclosure alone never creates an approval requirement.

## 5. Tests

Focused regressions prove:

- approval plus quiet source input normalizes to visible;
- denial plus quiet source input normalizes to visible;
- serialized approval plus quiet fails closed;
- serialized denial plus quiet fails closed;
- all prior two-axis selection, projection, serde, and non-leakage tests pass.

## 6. Governed Phase

- Dogfood workflow: `dg/blocker`.
- Run ID: `run-1783921429722720000-2`.
- Approval ID: `approval/run-1783921429722720000-2/fix-approved`.
- Approval presentation: `presentation/bdd30cca415b8def`.
- Approval outcome: granted with persisted presentation proof.
- Phase status: completed.
- Out-of-kernel work: Codex edited source, tests, and documentation and ran
  engineering validation. The kernel coordinated governance only.

## 7. Validation

All required validation passed:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `git diff --check`

## 8. Remaining Limitations

- Workload assessment and input fingerprinting remain unimplemented.
- No runtime path enforces execution or disclosure decisions.
- No UI or CLI surface satisfies visible disclosure.
- No durable event or report cites the decision.

## 9. Recommended Next Phase

Perform a focused blocker-fix re-review. If accepted, resume the immutable
run-bundle builder before implementing workload assessment or runtime
integration.
