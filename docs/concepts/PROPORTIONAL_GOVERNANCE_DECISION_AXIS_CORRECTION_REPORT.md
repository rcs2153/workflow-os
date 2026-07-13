# Proportional Governance Decision Axis Correction Report

## 1. Executive Summary

The proportional-governance core model and read-only projection now separate
execution disposition from operator disclosure. Permitted work can require
visible disclosure without becoming a blocking approval. Evidence, audit, and
report obligations remain outside both presentation and interruption choices.

This is a model-only correction. No executor, persistence, event, CLI, UI,
schema, approval-default, provider-write, or hosted behavior changed.

## 2. Scope Completed

- Replaced the one-dimensional interaction mode with an ordered execution
  disposition: `Proceed`, `RequireApproval`, or `Denied`.
- Added an independent disclosure requirement: `Quiet` or `Visible`.
- Added explicit two-axis requirement vocabulary for workflow, policy,
  authority, evidence/check, sensitivity, SideEffect, runtime, prior-decision,
  and steward inputs.
- Preserved independent monotonic escalation for both axes.
- Updated risk classification to derive from the accepted pair.
- Updated the read-only projection to expose both axes.
- Made blocking operator action derive only from execution disposition.
- Preserved validated, non-leaking decision and projection deserialization.
- Added focused tests for composition, monotonicity, serde, projection
  consistency, and non-leakage.

## 3. Scope Explicitly Not Completed

- deterministic workload assessment or inference;
- decision-input fingerprinting or invalidation;
- executor or approval integration;
- durable decision events or persistence;
- local UI, CLI rendering, notifications, or inspect projection;
- workflow or policy schema configuration;
- provider mutation expansion, including PR or Jira creation;
- automatic approval, hosted administration, or release changes.

## 4. Model Boundary

`GovernanceExecutionDisposition` answers whether work may proceed, must await
approval, or is denied. `GovernanceDisclosureRequirement` answers whether
immediate bounded disclosure is required. `GovernancePostureRequirement`
combines independent execution and disclosure contributions from one validated
governance concern.

The selector computes maxima independently. Prior accepted execution and
disclosure posture prevent downgrade during an active governed action.

## 5. Projection Boundary

The projection exposes execution, disclosure, risk class, stable reasons,
blocking action requirement, and explicit assessed-not-enforced/not-persisted
posture. `Proceed + Visible` maps to no blocking action plus required visible
disclosure. The projection neither enforces nor renders that obligation.

## 6. Privacy And Error Handling

Decision and projection payloads contain closed enum vocabulary and stable
reason codes only. Custom safe-wire deserialization rejects unknown values with
fixed messages and does not echo caller-supplied content. Contradictory risk,
action, or posture combinations fail closed.

## 7. Tests Added Or Updated

Focused tests cover:

- quiet proceed;
- visible disclosure without blocking;
- independent execution/disclosure composition;
- profile and steward minima;
- prior-decision monotonicity;
- denial and unsupported requirements;
- stable reason coverage;
- decision and projection serde round trips;
- inconsistent serialized values;
- unknown-value non-leakage;
- payload-free Debug and serialization.

## 8. Governed Phase

- Dogfood workflow: `dg/implement`.
- Run ID: `run-1783920371091601000-2`.
- Approval ID:
  `approval/run-1783920371091601000-2/implementation-approved`.
- Approval presentation: `presentation/d0695b4f4ff3f84a`.
- Approval outcome: granted with persisted presentation proof.
- Phase status: completed.
- Kernel boundary: governance coordination only; Codex performed source edits,
  tests, docs changes, and later git/PR actions outside the kernel.

## 9. Validation

All required validation passed:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `git diff --check`

Focused review found one cross-axis consistency blocker. The follow-up
[blocker-fix report](PROPORTIONAL_GOVERNANCE_DECISION_AXIS_BLOCKER_FIX_REPORT.md)
documents deterministic normalization of blocking and denied decisions to
visible disclosure.

## 10. Remaining Limitations

- Callers still construct already-classified decision inputs explicitly.
- No input fingerprint binds a decision to validated workload facts.
- No runtime path enforces either axis.
- No presentation surface satisfies visible disclosure.
- No durable event or WorkReport cites the decision.

## 11. Recommended Next Phase

Perform a focused maintainer review of this correction. If accepted, resume the
immutable run-bundle builder before implementing deterministic workload
assessment and input fingerprinting. Do not integrate proportional governance
into runtime behavior until both boundaries are reviewed.
