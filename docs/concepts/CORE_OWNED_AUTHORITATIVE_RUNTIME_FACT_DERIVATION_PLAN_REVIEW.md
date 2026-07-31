# Core-Owned Authoritative Runtime-Fact Derivation Plan Review

## 1. Executive Verdict

Plan accepted; proceed to the bounded runtime-composition implementation.

## 2. Problem Verification

The problem is confirmed in current code:

- the CLI constructs `StepGovernanceRuntimeFacts` for the selected workflow;
- non-selected steps are marked evidence/check `Satisfied`;
- the CLI predicts visible disclosure from optimistic authority, check, and
  side-effect facts before the same-call check result exists; and
- the executor request still accepts caller-supplied facts even when project
  activation is intended to be authoritative.

Core correctly rejects caller-preclassified authority for project-declared
execution and correctly replaces the selected step's evidence/check posture
with the same-call local-check result. Those protections do not justify the
remaining non-selected step classification or route prediction.

## 3. Scope Assessment

The plan is appropriately narrow:

- one closed project-validation profile;
- one immutable workflow step;
- one Core-derived neutral fact;
- existing authority and local-check sources;
- existing proportional-governance decision model;
- existing disclosure handler and receipt;
- no scaffold default change.

The one-step restriction matches the only generated and externally exercised
authoritative scaffold. It is more honest than implying multi-step coverage
whose independent facts do not exist.

## 4. Trust Assessment

The proposed boundary restores the intended ownership:

- declarations come from the immutable bundle;
- authority comes from validated project activation;
- evidence/check posture comes from the same-call attestation;
- side-effect posture comes from immutable capability declarations;
- Core selects the route; and
- the CLI only supplies execution inputs and an optional delivery capability.

The caller no longer supplies the conclusion it asks Core to enforce.

## 5. Visible Delivery Assessment

Making delivery capability available without invoking it is acceptable.
Capability availability is not evidence that delivery happened. The handler
must be called only after the complete assessment selects visible proceed, and
the existing receipt remains the proof of actual delivery.

Tests requiring zero calls for quiet, approval, and denial routes are a
necessary boundary.

## 6. Compatibility Assessment

Lower-level explicit-fact APIs serve legitimate embedding and testing use
cases. The plan should not remove them broadly.

The closed authoritative path needs a fact-free request type or equivalent
private boundary so callers cannot accidentally regain fact ownership.

## 7. Privacy And Failure Assessment

The proposed error vocabulary is bounded. Structural rejection occurs before
process use, run creation, and immutable-bundle persistence. No new payload
surface is introduced.

Multi-step rejection must not enumerate step identifiers or definitions.
Visible-delivery absence must not render disclosure content.

## 8. Test Assessment

The test plan covers the essential regressions:

- multi-step false satisfaction;
- optimistic visible-route prediction;
- handler invocation isolation;
- failed-check route integrity;
- authored approval separation;
- same-call reassessment; and
- compatibility of lower-level APIs.

A disposable scaffold proof remains necessary before the later default change,
but it is not a substitute for the focused Core tests.

## 9. Blockers

None.

## 10. Non-Blocking Follow-Ups

- General multi-step authoritative fact aggregation needs separate planning
  after per-step authority, checks, and SideEffect facts have real sources.
- Default scaffold activation should retain its explicit opt-out once this
  prerequisite passes.
- OpenShell should remain a later optional execution provider behind a
  provider-neutral execution and receipt contract.

## 11. Recommended Next Phase

Implement Core-owned authoritative runtime-fact derivation and conditional
visible-delivery consumption for the closed one-step project-validation route.

## 12. Governed Planning Record

- workflow: `dg/d`
- run: `run-1785468401630252000-2`
- approval: `approval/run-1785468401630252000-2/planning-approved`
- presentation: `presentation/caeca7fe4c85adb9`
- approval outcome: granted under delegated-maintainer authority
- approval proof: persisted before decision
- planning boundary: documentation and implementation design only
