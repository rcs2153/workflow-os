# High-Assurance Approval-Presentation Adoption Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The high-assurance approval-presentation adoption phase stayed within the
approved explicit, opt-in runtime path. It composes high-assurance approval
validation with durable approval-presentation proof validation before approval
decision mutation, preserves ordinary approval behavior, and keeps
provider-write/write-adjacent adoption deferred.

Recommended next phase: provider-write/write-adjacent approval-presentation
adoption planning.

## 2. Scope Verification

The phase stayed within the approved high-assurance approval-presentation
adoption scope.

Implemented scope:

- explicit high-assurance approval-presentation request type;
- explicit local executor approval method for high-assurance decisions with an
  approval-presentation policy;
- disclosure-returning method for the same proof-enforced high-assurance path;
- shared pre-mutation approval-presentation policy helper;
- focused tests and documentation/report updates.

No accidental implementation was found for:

- default public approval behavior changes;
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

## 3. API And Model Assessment

The model addition is appropriately narrow.

`LocalHighAssuranceApprovalPresentationDecisionRequest` composes the existing
`LocalHighAssuranceApprovalDecisionRequest` with an explicit
`ApprovalPresentationDefaultEnforcementPolicy`. This keeps high-assurance
controls, supplied references, and presentation proof policy explicit at the
call site.

The executor methods are additive:

- `decide_approval_with_high_assurance_presentation_policy(...)`;
- `decide_approval_with_high_assurance_presentation_policy_disclosure(...)`.

They do not replace or alter:

- `decide_approval(...)`;
- `decide_approval_with_presentation(...)`;
- `decide_approval_with_default_presentation_policy(...)`;
- `decide_approval_with_high_assurance(...)`.

Debug behavior for the request redacts the nested approval request and avoids
printing run IDs, approval IDs, presentation IDs, evidence IDs, approval
reasons, or secret-like values.

## 4. Composition Order Assessment

The implementation follows the required pre-mutation order.

Observed order:

1. rehydrate the run and resolve the pending approval;
2. construct the approval decision in memory;
3. validate high-assurance controls and supplied references;
4. evaluate the approval-presentation policy;
5. require high-assurance posture for the high-assurance sensitive path;
6. resolve and validate durable presentation proof when required;
7. attach a proof marker when proof is required;
8. construct report-safe disclosure when requested;
9. append the approval decision exactly once.

The implementation does not simply call one public approval method and then
another. It uses a shared helper before `apply_approval_decision(...)`, which
keeps failure behavior pre-mutation.

## 5. Policy Assessment

The policy behavior is appropriate for the first high-assurance adoption path.

Verified behavior:

- `NotRequired` preserves existing high-assurance approval behavior.
- Required presentation proof attaches a proof marker on success.
- Missing proof fails closed before approval events.
- `RequiredForSensitiveAction` requires
  `ApprovalPresentationSensitiveActionPosture::HighAssurance` on the
  high-assurance path.
- `WriteAdjacent` posture is rejected on the high-assurance-specific path.
- High-assurance validation failure occurs before proof attachment or approval
  mutation.

This is the right boundary: the path proves high-assurance approval presentation
without broadening provider-write, side-effect, or default approval behavior.

## 6. Error-Handling Assessment

Failure behavior is structured and non-leaking.

Covered errors include:

- `approval_presentation_default_enforcement.proof_missing`;
- `approval_presentation_default_enforcement.sensitive_posture_mismatch`;
- existing approval-presentation proof validation errors;
- existing high-assurance validation errors such as
  `high_assurance_approval.enforcement.reference.missing`.

Tests verify that missing proof, wrong sensitive posture, and high-assurance
validation failures do not append approval decision events. Error assertions
also check that approval IDs and posture details are not leaked in selected
failure paths.

## 7. Privacy And Redaction Assessment

The implementation uses references and proof markers rather than copying
approval-presentation payloads.

Verified privacy posture:

- no raw provider payloads are introduced;
- no command output is copied;
- no source contents are copied;
- no approval-card text is copied into errors or Debug output;
- no raw approval reasons are copied into Debug output;
- proof is represented by bounded presentation identity and proof-marker
  metadata;
- disclosure output remains report-safe.

The Debug regression test for the request covers secret-like reason text,
approval IDs, run IDs, presentation IDs, and evidence IDs.

## 8. Runtime Semantics Assessment

Existing approval semantics remain intact.

The new path is explicit and opt-in. Ordinary approval calls and existing
high-assurance approval calls continue to behave as before. The implementation
does not introduce automatic report generation, artifact writes, provider
calls, side-effect execution, CLI output, or schema behavior.

On failure before approval append, the run is not resumed, no approval decision
event is appended, and skill invocation does not proceed.

## 9. Test Quality Assessment

The focused tests cover the important safety and compatibility paths:

- `NotRequired` compatibility;
- proof-required success and proof-marker attachment;
- missing proof fail-closed behavior;
- wrong sensitive posture fail-closed behavior;
- high-assurance validation before proof attachment;
- disclosure-returning proof path;
- Debug redaction.

The test set is strong enough for this phase.

Non-blocking test follow-ups:

- Add a high-assurance-specific stale or mismatched proof regression, even
  though shared proof validation is already covered elsewhere.
- Add a denial-path test with required high-assurance presentation proof, to
  prove denied decisions also preserve proof requirements when configured.
- Add a one-event-count assertion for the success path to make the
  "approval decision appended exactly once" contract even more explicit.

## 10. Documentation Review

Documentation is honest about current capabilities.

Docs now state that the selected high-assurance approval-presentation adoption
path is implemented, while the following remain unimplemented:

- default public approval-presentation enforcement;
- provider-write approval-presentation adoption;
- approval-card UI;
- workflow schema fields;
- CLI mutation behavior;
- provider writes;
- side-effect execution;
- hosted runtime;
- reasoning lineage;
- examples;
- release posture changes.

No dangerous overclaim was found.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Add path-specific stale/mismatched proof tests for the high-assurance
  approval-presentation method.
- Add a denial-path regression with required presentation proof.
- Add success-path event-count assertions for exactly-one decision append.
- Plan provider-write/write-adjacent approval-presentation adoption before any
  implementation touches provider mutation paths.

## 13. Recommended Next Phase

Recommended next phase: provider-write/write-adjacent approval-presentation
adoption planning.

The high-assurance composition path is now implemented and accepted. The next
work should decide where presentation proof becomes mandatory for
write-adjacent or provider-call approval surfaces without enabling provider
writes prematurely and without changing default public approval behavior.

## 14. Governed Dogfood Run

- workflow: `dg/review`
- run ID: `run-1783716422134438000-2`
- approval ID: `approval/run-1783716422134438000-2/review-scope-approved`
- approval-presentation ID: `presentation/0f1296d6449fe5f0`
- approval-presentation hash:
  `0f1296d6449fe5f05c1c381040ccb476c2912b830b9393c36483fe6148649d72`
- approval outcome: granted by delegated maintainer
- approval reason: `approved-high-assurance-approval-presentation-review`

Workflow OS governed the review approval boundary. Codex performed repository
inspection, review writing, validation, git, and PR work outside the kernel.

## 15. Validation

Review validation:

- `npm run check:docs` - passed.
- `git diff --check` - passed.
