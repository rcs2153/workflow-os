# Proportional Governance Decision Axis Correction Review

## 1. Executive Verdict

**Needs blocker fixes.**

The architectural correction is right: execution disposition and disclosure
are now independent, and `Proceed + Visible` no longer becomes a blocking
operator action. One cross-axis invariant is not yet enforced, however, so the
model can accept contradictory blocking or denial decisions marked `Quiet`.

Fix-forward note: the blocker is addressed in the
[Decision Axis Blocker Fix Report](PROPORTIONAL_GOVERNANCE_DECISION_AXIS_BLOCKER_FIX_REPORT.md).
The original finding remains the review record; focused re-review is required
before acceptance.

## 2. Scope Verification

The phase stayed within the approved model-only scope. It did not add workload
inference, executor integration, persistence, workflow events, CLI or UI
behavior, schemas, provider mutations, automatic approval, hosted
administration, or release changes.

## 3. Model Assessment

The replacement vocabulary is domain-neutral and appropriately small:

- `GovernanceExecutionDisposition` selects proceed, approval, or denial.
- `GovernanceDisclosureRequirement` selects quiet or visible presentation.
- source requirements contribute independent execution and disclosure minima.
- prior accepted values prevent downgrade independently.
- risk class is derived from the accepted pair.

This resolves the user-reported conflation. A policy-required visible
disclosure can coexist with `Proceed`, while an operator presentation
preference can later render quiet records without changing execution policy.

## 4. Projection Assessment

The read-only projection preserves both accepted axes and derives blocking
operator action only from execution disposition. `Proceed + Visible` correctly
projects `action_requirement: none` plus `disclosure: visible`.

The projection remains assessed-not-enforced and not-persisted. It does not
render, notify, pause, deny, append events, or alter runtime behavior.

## 5. Validation And Monotonicity

The selector computes maxima independently and preserves prior execution and
disclosure posture. Unsupported requirements fail closed. Strict-enterprise
evaluation still requires a steward minimum. Explicit denial wins.

The remaining blocker is cross-axis consistency. The public constructor allows:

```text
execution: require_approval
disclosure: quiet_allowed
```

and:

```text
execution: denied
disclosure: quiet_allowed
```

`requirement_posture` currently accepts both unchanged. A future consumer could
therefore receive an approval or denial result whose accepted disclosure says
`Quiet`, despite approval-presentation and fail-closed user-diagnostic
requirements. The selector must normalize blocking/denied execution to visible
disclosure or reject those requirement pairs deterministically. Normalization
is the smaller compatible option because stricter disclosure is monotonic.

## 6. Serde And Privacy

Accepted decisions and projections use fixed safe-wire parsers for execution,
disclosure, risk, reasons, action, and posture fields. Unknown caller-supplied
values are rejected without echo. Inconsistent risk or action combinations fail
closed. Debug and serialized outputs contain closed vocabulary only.

Input requirement enums retain their preexisting direct serde posture. A later
review may consider a safe-wire input boundary before schema exposure, but that
is non-blocking while no workflow schema or runtime integration consumes this
input model.

## 7. Test Assessment

The focused suite covers:

- quiet proceed;
- visible disclosure without blocking;
- independent axis composition;
- profile and steward minima;
- prior-decision monotonicity;
- denial and unsupported inputs;
- reason coverage;
- decision and projection serde;
- inconsistent and unknown value rejection;
- payload-free Debug and serialization.

Missing blocker regression coverage:

- approval plus quiet input becomes visible or fails closed;
- denial plus quiet input becomes visible or fails closed;
- direct deserialization of a contradictory accepted decision remains invalid.

## 8. Validation

The following passed before review:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `git diff --check`

## 9. Blockers

1. Enforce the invariant that `RequireApproval` and `Denied` accepted decisions
   require `Visible` disclosure. Add focused constructor, selector, and serde
   regression tests.

## 10. Non-Blocking Follow-Ups

- Review direct input-enum deserialization errors before workflow schema
  exposure.
- Keep deterministic workload assessment and fingerprinting in its separate
  planned phase.
- Define whether future enterprise notification obligations need more closed
  vocabulary than quiet/visible.

## 11. Recommended Next Phase

Perform one focused blocker fix for blocking/denial disclosure consistency,
then re-review this model. Do not begin workload inference or runtime
integration until the fix is accepted.

## 12. Governed Review Evidence

- Dogfood workflow: `dg/review`.
- Run ID: `run-1783921216309974000-2`.
- Approval ID:
  `approval/run-1783921216309974000-2/review-scope-approved`.
- Approval presentation: `presentation/c1d41c3990dba91f`.
- Approval outcome: granted with persisted presentation proof.
- Phase status: completed.
- Out-of-kernel work: Codex inspected source, tests, plans, and validation
  results and authored this review. The kernel did not edit files or execute
  engineering checks.
- Report posture: no WorkReport artifact was generated or persisted.
