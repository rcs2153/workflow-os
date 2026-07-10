# Approval Gate Presentation Default Enforcement Implementation Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation adds the planned explicit default-enforcement policy/helper
boundary without changing public approval defaults. It gives selected local
callers a deterministic way to route approval decisions through ordinary
approval behavior, existing proof-enforced approval behavior, or
caller-declared sensitive/write-adjacent proof enforcement.

## 2. Scope Verification

The phase stayed within the approved model/helper implementation scope.

Implemented scope:

- explicit approval-presentation default-enforcement mode vocabulary;
- explicit sensitive/write-adjacent posture vocabulary;
- explicit policy request type;
- executor-adjacent helper method;
- fail-closed policy validation before approval mutation;
- redaction-safe Debug behavior;
- focused tests;
- docs, roadmap, and implementation report updates.

No accidental implementation was found for:

- global default approval behavior changes;
- automatic approvals;
- hidden approvals;
- approval-card UI;
- workflow schema fields;
- CLI mutation behavior;
- high-assurance approval integration;
- provider writes;
- side effects;
- automatic report artifact writing;
- hosted or distributed runtime behavior;
- reasoning lineage;
- examples;
- release posture changes.

## 3. API Assessment

The API is appropriately small and explicit.

Implemented:

- `ApprovalPresentationDefaultEnforcementMode`;
- `ApprovalPresentationSensitiveActionPosture`;
- `ApprovalPresentationDefaultEnforcementPolicy`;
- `LocalApprovalPresentationDefaultDecisionRequest`;
- `LocalExecutor::decide_approval_with_default_presentation_policy(...)`.

The helper composes existing approval behavior rather than creating a parallel
approval system. `NotRequired` delegates to `decide_approval(...)`, while
proof-required paths delegate to `decide_approval_with_presentation(...)`.
That is the right compatibility shape.

The implementation did not add the planned placeholder
`ApprovalPresentationDefaultEnforcementContext` name. That is acceptable:
`LocalApprovalPresentationDefaultDecisionRequest` and
`ApprovalPresentationDefaultEnforcementPolicy` cover the needed implementation
surface without extra vocabulary.

## 4. Enforcement Mode Assessment

The implemented modes match the accepted plan:

- `NotRequired`;
- `Required`;
- `RequiredForSensitiveAction`.

No `Automatic`, `InferFromText`, or model-opinion mode was introduced.

`RequiredForSensitiveAction` requires explicit bounded posture. The helper does
not infer sensitivity from approval reasons, chat text, source contents,
provider payloads, command output, or model judgment.

## 5. Fail-Closed Assessment

The helper fails closed before approval mutation when:

- proof is required but missing;
- `RequiredForSensitiveAction` is used without explicit posture;
- proof/freshness/posture fields are supplied while the mode is `NotRequired`.

When proof is supplied, the helper delegates to the existing
approval-presentation proof-enforced path, preserving the already-reviewed
identity, freshness, mismatch, ambiguity, and proof-marker behavior.

The tests verify that missing proof and invalid policy combinations do not
append new events before failure.

## 6. Compatibility Assessment

Existing `LocalExecutor::decide_approval(...)` remains unchanged.

The new helper is additive and caller-selected. Non-proof approval behavior is
preserved for callers that do not opt into the new helper and for helper calls
that explicitly use `NotRequired` without proof-only fields.

This is compatible with the planned migration path: gather evidence through
explicit callers before any broader default change is considered.

## 7. Privacy And Error Assessment

The new error codes are stable and bounded:

- `approval_presentation_default_enforcement.proof_not_required`;
- `approval_presentation_default_enforcement.proof_missing`;
- `approval_presentation_default_enforcement.sensitive_posture_missing`.

Errors use fixed messages and do not include raw approval IDs, presentation
IDs, run IDs, approval reasons, handoff text, paths, command output, provider
payloads, source snippets, tokens, credentials, or secret-like values.

Debug output for the policy redacts proof identifiers while preserving bounded
mode/posture visibility. Debug output for the request redacts the nested
approval request.

## 8. Test Quality Assessment

Focused tests cover:

- `NotRequired` preserving existing approval behavior;
- `Required` delegating to the proof-enforced approval path;
- missing required proof failing before event mutation;
- proof fields on `NotRequired` failing before event mutation;
- sensitive/write-adjacent mode requiring explicit posture;
- sensitive/write-adjacent mode with proof succeeding;
- Debug output redacting approval IDs, presentation IDs, and secret-like
  approval reason text.

The phase also passed the full workspace test suite, preserving existing
EvidenceReference, WorkReport, approval, policy, runtime, adapter, provider,
CLI, and docs behavior.

Non-blocking follow-up: a later adoption phase should add caller-specific tests
for whichever high-assurance or write-adjacent paths begin using this helper.

## 9. Documentation Review

Docs state that:

- the policy model/helper implementation is complete;
- default public approval behavior remains unchanged;
- approval-card UI is not implemented;
- workflow schema fields are not implemented;
- CLI mutation behavior is not implemented;
- high-assurance approval integration is not implemented;
- provider writes, side effects, hosted runtime, reasoning lineage, examples,
  and release posture changes remain unsupported.

The implementation report includes governed dogfood run details and validation
results.

## 10. Governed Dogfood Review Run

- workflow_id: `dg/review`
- run_id: `run-1783713423217182000-2`
- approval_id: `approval/run-1783713423217182000-2/review-scope-approved`
- approval presentation: `presentation/ec7d5bfa8a12e9dd`
- approval presentation hash:
  `ec7d5bfa8a12e9ddb1957713fd2e967cdb669b95738f3105483238a77093281d`
- approval outcome: granted by delegated maintainer
- approval reason: `approved-review-phase`

Workflow OS governed the review approval boundary. Codex performed repository
inspection, documentation authoring, validation, git, and PR work outside the
kernel.

## 11. Validation

Implementation validation reviewed:

- `cargo fmt --all --check` - passed;
- `cargo test -p workflow-core --test local_executor default_presentation_policy`
  - passed;
- `cargo clippy --workspace --all-targets -- -D warnings` - passed;
- `cargo test --workspace` - passed;
- `npm run check:docs` - passed;
- `git diff --check` - passed.

Review validation:

- `npm run check:docs` - passed;
- `git diff --check` - passed.

## 12. Blockers

No blockers.

## 13. Non-Blocking Follow-Ups

- Plan selected high-assurance/write-adjacent adoption of the helper before
  wiring it into security-sensitive caller paths.
- Add caller-specific tests when selected paths adopt the helper.
- Keep global/default public approval behavior unchanged until a separately
  reviewed migration phase.

## 14. Recommended Next Phase

Recommended next phase: high-assurance/write-adjacent approval-presentation
policy adoption planning.

The next phase should decide which existing high-assurance or write-adjacent
approval callers should use this helper first. It must not flip public default
approval behavior, add UI approval cards, add workflow schemas, add CLI
mutation behavior, enable provider writes, model side effects, add hosted
behavior, add reasoning lineage, update examples, or change release posture.
