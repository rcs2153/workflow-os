# High-Assurance Approval-Presentation Adoption Report

## 1. Executive Summary

The first selected approval-presentation adoption path is implemented for
explicit high-assurance local approval decisions.

Workflow OS can now compose high-assurance approval validation with
approval-presentation proof validation before any approval decision event is
appended. The implementation is opt-in, local, and explicit. Ordinary approval
behavior remains unchanged.

## 2. Scope Completed

- Added an explicit high-assurance approval-presentation request type.
- Added an executor method for high-assurance approval decisions with an
  approval-presentation policy.
- Added a disclosure-returning executor method for the same proof-enforced
  high-assurance path.
- Refactored default approval-presentation policy handling through a shared
  pre-mutation helper.
- Preserved existing ordinary approval behavior and existing high-assurance
  approval behavior when presentation proof is not required.
- Added focused tests for proof attachment, missing proof, wrong posture,
  validation ordering, disclosure behavior, and Debug redaction.
- Updated roadmap and planning docs honestly.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- default public approval-presentation enforcement;
- automatic approvals;
- hidden approvals;
- approval-card UI;
- workflow schema fields;
- CLI mutation behavior;
- provider writes;
- provider-write/write-adjacent approval-presentation adoption;
- side-effect execution;
- automatic report artifact writing;
- hosted or distributed runtime behavior;
- reasoning lineage;
- examples;
- release posture changes.

## 4. API Summary

Added:

- `LocalHighAssuranceApprovalPresentationDecisionRequest`;
- `LocalExecutor::decide_approval_with_high_assurance_presentation_policy(...)`;
- `LocalExecutor::decide_approval_with_high_assurance_presentation_policy_disclosure(...)`.

The request composes:

- an existing `LocalHighAssuranceApprovalDecisionRequest`;
- an explicit `ApprovalPresentationDefaultEnforcementPolicy`.

The disclosure method returns the existing report-safe high-assurance approval
disclosure result shape.

## 5. Enforcement And Composition Summary

The high-assurance approval-presentation path validates in this order:

1. rehydrate the run and resolve the pending approval;
2. construct the approval decision in memory;
3. validate high-assurance controls and supplied references;
4. evaluate the approval-presentation policy;
5. require high-assurance sensitive posture when the policy uses
   `RequiredForSensitiveAction`;
6. resolve and validate durable presentation proof when proof is required;
7. attach a proof marker to the approval decision when proof is required;
8. construct report-safe disclosure when requested;
9. append the approval decision exactly once.

Failures before approval append do not resume the run, append approval decision
events, invoke skills, append side-effect events, write artifacts, call
providers, or mutate unrelated runtime state.

## 6. Privacy And Redaction Summary

The implementation stores and cites proof by bounded record identity and proof
marker metadata. It does not copy approval-card text, raw approval reasons,
provider payloads, command output, source contents, transcript text, tokens, or
secret-like strings into errors or Debug output.

Debug output for the new request redacts the nested approval request and does
not expose run IDs, approval IDs, presentation IDs, evidence IDs, or
secret-like reason text.

## 7. Tests Added

Focused tests cover:

- `NotRequired` preserving existing high-assurance approval behavior;
- required high-assurance presentation proof attaching a proof marker;
- missing proof failing before approval events;
- wrong sensitive posture failing before approval events;
- high-assurance validation failure occurring before proof attachment;
- disclosure-returning path preserving report-safe disclosure and proof marker
  behavior;
- Debug redaction for approval IDs, run IDs, presentation IDs, evidence IDs, and
  secret-like reason text.

Existing default approval-presentation, ordinary approval, proof-enforced
approval, and high-assurance approval tests remain in place.

## 8. Governed Dogfood Run

- workflow: `dg/implement`
- run ID: `run-1783714826496218000-2`
- approval ID: `approval/run-1783714826496218000-2/implementation-approved`
- approval-presentation ID: `presentation/4df6a3fc267caeba`
- approval outcome: granted
- governed phase: implementation

## 9. Validation Commands Run

Completed during implementation:

- `cargo fmt --all --check` passed.
- `cargo test -p workflow-core --test local_executor high_assurance_presentation_policy`
  passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `npm run check:docs` passed.
- `git diff --check` passed.

## 10. Remaining Known Limitations

- Default public approval behavior is still unchanged.
- Provider-write/write-adjacent approval-presentation adoption remains
  unimplemented.
- Approval-card UI remains unimplemented.
- Workflow-declared approval-presentation requirements remain unimplemented.
- The implementation is explicit and local; it does not make approval proof
  automatic for every approval surface.

## 11. Recommended Next Phase

Recommended next phase: high-assurance approval-presentation adoption review.

This path is safety-sensitive because it composes two approval gates before
runtime mutation. It should be reviewed before expanding adoption to
provider-write/write-adjacent paths.
