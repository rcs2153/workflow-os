# Approval Gate Presentation Default Enforcement Plan Review

## 1. Executive Verdict

Plan accepted; proceed to default approval-presentation enforcement policy
model/helper implementation.

The plan defines the right next boundary: a small explicit policy/helper layer
that can route selected approval decisions through existing proof-enforced
approval behavior without flipping default public approval behavior globally.

## 2. Scope Verification

The planning phase stayed within approved planning-only scope.

It created a default approval-presentation enforcement plan, linked it from the
roadmap and approval-presentation gap docs, and produced a governed phase
report.

No accidental authorization was found for:

- runtime implementation during the planning phase;
- global default approval behavior changes;
- automatic approvals;
- hidden approvals;
- approval-card UI;
- workflow schema fields;
- CLI mutation behavior;
- provider writes;
- side effects;
- automatic report artifact writing;
- hosted or distributed runtime behavior;
- reasoning lineage;
- recursive agents, agent swarms, or Level 3/4 autonomy;
- release posture changes.

## 3. Boundary Assessment

The plan correctly rejects a global change to
`LocalExecutor::decide_approval(...)` as the first default-enforcement step.

The recommended boundary is explicit and reviewable:

- proof not required;
- proof required for this approval decision;
- proof required only when the caller already knows the approval is
  sensitive/write-adjacent.

That preserves compatibility while creating the missing bridge between
dogfood/opt-in proof enforcement and future public default enforcement.

## 4. Model And Helper Assessment

The proposed model/helper vocabulary is appropriate:

- `ApprovalPresentationDefaultEnforcementPolicy`;
- `ApprovalPresentationDefaultEnforcementMode`;
- `ApprovalPresentationDefaultEnforcementContext`;
- an executor-adjacent helper only if it is the smallest idiomatic shape.

The plan correctly says the helper should delegate to existing
`LocalExecutor::decide_approval_with_presentation(...)` when proof is required
and preserve the existing approval path when proof is not required.

## 5. Enforcement Mode Assessment

The proposed initial modes are acceptable:

- `NotRequired`;
- `Required`;
- `RequiredForSensitiveAction`.

The plan correctly excludes `Automatic` and `InferFromText` modes. The kernel
must not infer approval criticality from raw chat text, source contents,
provider payloads, approval reasons, or model opinion.

## 6. Fail-Closed Assessment

The fail-closed rules are sufficiently concrete for implementation.

When proof is required, the plan requires approval failure before mutation if
proof is missing, stale, mismatched, ambiguous, corrupt, invalid, or unsafe to
validate.

The plan also preserves the critical no-mutation boundary: proof failure must
not append approval events, resume runs, invoke skills, append side-effect
events, write artifacts, or perform provider calls.

## 7. Compatibility Assessment

The compatibility posture is strong.

The plan keeps default public approval behavior unchanged until:

- the explicit policy boundary is implemented and reviewed;
- dogfood usage proves proof-enforced grants and denials;
- high-assurance approval integration is planned;
- write-adjacent approval surfaces have clear evidence and authority
  requirements;
- public docs explain proof-required approval behavior;
- tests prove non-proof paths remain unchanged when proof is not required.

This is the right migration shape. It prevents sudden approval friction while
still closing the path toward enforceable approval presentation proof.

## 8. Relationship Assessment

The plan correctly relates default enforcement to existing foundations:

- dogfood remains the benchmark, not public default behavior;
- high-assurance approvals remain future integration;
- provider writes must not proceed on ordinary approval events alone;
- write-adjacent paths should eventually require proof as part of pre-provider
  gate posture.

It does not overclaim readiness for write-capable adapters or high-assurance
approval controls.

## 9. Privacy And Error Assessment

The error-handling plan is appropriately conservative.

Candidate error families are stable and bounded. The plan explicitly forbids
errors from including raw approval IDs, presentation IDs, paths, handoff text,
approval reasons, command output, provider payloads, source snippets, tokens,
credentials, or secret-like values.

The implementation should preserve that standard by using fixed messages and
redaction-safe Debug output for all new policy/helper types.

## 10. Test Plan Assessment

The future test plan covers the important proof and compatibility boundaries:

- `NotRequired` preserves existing approval behavior;
- `Required` delegates to the proof-enforced path;
- missing, stale, mismatched, and corrupt proof fail closed;
- policy Debug/serialization are bounded and redaction-safe;
- grant and denial paths honor required proof;
- no resume, skill invocation, side-effect event, provider call, or artifact
  write occurs after proof failure;
- existing `decide_approval(...)` tests continue to pass unchanged.

Non-blocking follow-up: the implementation should include direct tests that
prove `RequiredForSensitiveAction` does not infer sensitivity from free-form
text and fails closed when the caller does not provide an explicit bounded
sensitive/write-adjacent posture.

## 11. Documentation Review

The plan and phase report state that:

- default approval-presentation enforcement is planned, not implemented;
- current default approval behavior remains unchanged;
- the next implementation should be a policy model/helper only;
- approval-card UI is not implemented;
- workflow schema fields are not implemented;
- CLI mutation behavior is not implemented;
- high-assurance approval integration is not implemented;
- provider writes, side effects, hosted runtime, reasoning lineage, recursive
  agents, agent swarms, Level 3/4 autonomy, examples, and release posture
  changes remain unsupported.

## 12. Governed Dogfood Review Run

- workflow_id: `dg/review`
- run_id: `run-1783709404368483000-2`
- approval_id: `approval/run-1783709404368483000-2/review-scope-approved`
- approval presentation: `presentation/fdb609eb2c1ad66b`
- approval presentation hash:
  `fdb609eb2c1ad66bbe2ec7b2746d07669adf85a1128b8265dd9489edc2433e8f`
- approval outcome: granted by delegated maintainer
- approval reason: `approved-review-phase`

Workflow OS governed the review approval boundary. Codex performed repository
inspection, documentation authoring, validation, git, and PR work outside the
kernel.

## 13. Validation

Review validation:

- `npm run check:docs` - passed.
- `git diff --check` - passed.

Rust validation is not required for this review-only phase because no runtime
code changed.

## 14. Blockers

No blockers.

## 15. Non-Blocking Follow-Ups

- During implementation, add focused tests for `RequiredForSensitiveAction`
  explicit-posture behavior.
- Keep default public approval behavior unchanged until the policy/helper is
  implemented, reviewed, and used in enough dogfood paths to justify broader
  migration.
- Later, decide whether proof should be required for denial decisions as well
  as grant decisions in public default paths.

## 16. Recommended Next Phase

Recommended next phase: default approval-presentation enforcement policy
model/helper implementation.

The next phase should add model/helper vocabulary and tests only. It must not
flip public default approval behavior, add UI approval cards, add workflow
schemas, add CLI mutation behavior, integrate high-assurance approvals, enable
provider writes, model side effects, add hosted behavior, add reasoning
lineage, update examples, or change release posture.
