# Proportional Governance Core Model Blocker Fix Report

## 1. Executive Summary

The two blockers from the proportional-governance core model review are fixed.
Serialized decisions now fail closed when derived fields disagree, and profile
evaluation no longer invents unconditional interruption behavior.

## 2. Blockers Fixed

1. `ProportionalGovernanceDecision` previously derived deserialization and
   accepted inconsistent mode/risk-class pairs.
2. Profile mapping forced disclosure for agent-assisted work and approval for
   strict-enterprise work without the evidence or steward minimum required by
   the accepted plan.

## 3. Implementation Approach

- Replaced derived decision deserialization with a validated wire boundary.
- Required the serialized risk class to match the class derived from mode.
- Required the stable profile-minimum reason to be present.
- Changed agent-assisted profile minimum to quiet capture; concrete evidence,
  policy, authority, sensitivity, and SideEffect requirements still escalate.
- Added an explicit optional steward-minimum input.
- Required strict-enterprise evaluation to supply that steward minimum or fail
  closed with `governance.proportional.steward_minimum.required`.
- Added a stable steward-minimum reason code.

## 4. Validation Boundary

The selector remains pure and model-only. Unsupported requirements still return
the existing stable unsupported error. Strict-enterprise evaluation without a
minimum now returns a stable invalid-state error. Neither error includes input
values or caller payloads.

## 5. Privacy And Redaction

The added field and reason remain enum-only. Custom deserialization errors are
fixed text and do not reproduce serialized input. Debug and serialization still
cannot contain raw evidence, source content, commands, provider payloads,
approval reasons, or credentials.

## 6. Test Coverage

Focused tests now cover:

- agent-assisted quiet eligibility;
- strict-enterprise missing-minimum failure;
- strict-enterprise explicit-minimum selection;
- inconsistent mode/risk-class wire rejection;
- missing validation-reason wire rejection;
- all original deterministic, monotonic, denial, unsupported, and serde paths.

## 7. Commands And Results

- `cargo test -p workflow-core --test proportional_governance`: passed, 14
  tests.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace --quiet`: passed; existing opt-in live tests remained
  ignored behind their environment gates.
- `npm run check:docs`: passed.

## 8. Scope Not Added

No runtime integration, executor behavior, CLI, schema, persistence, events,
automatic approval, metrics collection, provider writes, examples, hosted
administration, or release changes were added.

## 9. Recommended Next Phase

Perform a focused blocker-fix review. Only after acceptance should the roadmap
consider a read-only decision projection.

## 10. Governed Fix Evidence

- Workflow ID: `dg/blocker`.
- Run ID: `run-1783806875134422000-2`.
- Approval ID: `approval/run-1783806875134422000-2/fix-approved`.
- Approval outcome: granted with persisted presentation proof
  `presentation/e412d98e79507e31`.
- Out-of-kernel work: Codex performed code/docs edits and validation; the kernel
  governed scope and approval but did not edit files or run checks.
- Event and terminal summary: completed with 39 ordered events, one approval,
  zero retries, and zero escalations; presentation proof enforcement and the
  approval event marker were present.
