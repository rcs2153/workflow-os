# Approval Gate Presentation Default Enforcement Implementation Report

## 1. Executive Summary

Default approval-presentation enforcement now has an explicit model/helper
boundary.

The implementation adds a local policy request that lets callers choose whether
approval-presentation proof is not required, required, or required for an
explicitly declared sensitive/write-adjacent approval posture. The helper
delegates to existing ordinary approval behavior or the existing
proof-enforced approval path. It does not flip public default approval
behavior.

## 2. Scope Completed

Completed:

- added `ApprovalPresentationDefaultEnforcementMode`;
- added `ApprovalPresentationSensitiveActionPosture`;
- added `ApprovalPresentationDefaultEnforcementPolicy`;
- added `LocalApprovalPresentationDefaultDecisionRequest`;
- added
  `LocalExecutor::decide_approval_with_default_presentation_policy(...)`;
- exported the new public model/helper types from `workflow-core`;
- added focused local executor tests for ordinary, required, and
  sensitive-posture behavior;
- added redaction/non-leakage tests;
- updated docs and roadmap status.

## 3. Scope Explicitly Not Completed

Not implemented:

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

## 4. API Summary

The new API is additive:

- `ApprovalPresentationDefaultEnforcementPolicy::not_required()` preserves
  existing approval behavior when no proof-only fields are supplied.
- `ApprovalPresentationDefaultEnforcementPolicy::required(...)` requires
  durable approval-presentation proof and delegates to the existing
  proof-enforced approval path.
- `ApprovalPresentationDefaultEnforcementPolicy::required_for_sensitive_action(...)`
  requires both explicit bounded sensitive/write-adjacent posture and durable
  presentation proof.
- `LocalExecutor::decide_approval_with_default_presentation_policy(...)`
  routes the request according to the explicit policy.

Existing `LocalExecutor::decide_approval(...)` and
`LocalExecutor::decide_approval_with_presentation(...)` remain unchanged.

## 5. Enforcement Summary

`NotRequired` preserves ordinary approval behavior only when the policy contains
no proof, freshness, or sensitive-posture fields. If proof-only fields are
provided while proof is not required, the helper fails closed with a stable
policy error.

`Required` fails closed when proof is missing. When proof is supplied, the
helper delegates to the existing approval-presentation proof validation path.

`RequiredForSensitiveAction` does not infer sensitivity from approval reasons,
chat text, source contents, provider payloads, or model opinion. It fails closed
unless the caller supplies explicit bounded posture.

## 6. Error And Privacy Summary

New stable error codes:

- `approval_presentation_default_enforcement.proof_not_required`;
- `approval_presentation_default_enforcement.proof_missing`;
- `approval_presentation_default_enforcement.sensitive_posture_missing`.

Errors use fixed messages and do not include raw approval IDs, presentation
IDs, run IDs, approval reasons, handoff text, paths, command output, provider
payloads, source snippets, tokens, credentials, or secret-like values.

Debug output for the new request and policy redacts nested approval request
details and proof identifiers while preserving bounded policy posture.

## 7. Test Coverage Summary

Focused tests cover:

- `NotRequired` preserves existing approval behavior;
- `Required` delegates to the proof-enforced path and produces proof markers;
- missing proof fails closed before new events are appended;
- proof fields on `NotRequired` fail closed before mutation;
- `RequiredForSensitiveAction` requires explicit posture and does not infer
  from secret-like approval reason text;
- sensitive posture with proof succeeds;
- Debug output does not leak approval IDs, presentation IDs, or secret-like
  approval reason text.

## 8. Governed Dogfood Run

- workflow_id: `dg/implement`
- run_id: `run-1783710089650124000-2`
- approval_id: `approval/run-1783710089650124000-2/implementation-approved`
- approval presentation: `presentation/d878e719999a71ce`
- approval presentation hash:
  `d878e719999a71ce4382142724a858eccfca321ecd0577dd6708e7c3468d12b2`
- approval outcome: granted by delegated maintainer
- approval reason: `approved-implementation-phase`

Workflow OS governed the implementation approval boundary. Codex performed
repository edits, shell validation, git, and PR work outside the kernel.

## 9. Validation Commands And Results

Passed:

- `cargo fmt --all --check`;
- `cargo test -p workflow-core --test local_executor default_presentation_policy`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`.
- `git diff --check`.

## 10. Remaining Known Limitations

- Default public approval behavior remains unchanged.
- The helper is not wired into high-assurance approval controls.
- The helper is not wired into provider-write orchestration.
- Workflow-declared approval-presentation requirements remain unimplemented.
- Approval-card UI remains unimplemented.

## 11. Recommended Next Phase

Recommended next phase: default approval-presentation enforcement policy
model/helper review.

After review, the next implementation should decide whether selected
high-assurance or write-adjacent callers should adopt this helper. Do not flip
public default approval behavior without a separate reviewed migration phase.
