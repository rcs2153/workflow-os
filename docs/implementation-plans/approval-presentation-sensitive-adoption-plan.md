# Approval-Presentation Sensitive Adoption Plan

## 1. Executive Summary

Workflow OS now has an explicit approval-presentation default-enforcement
policy/helper. The helper lets selected local callers preserve ordinary approval
behavior, require presentation proof, or require proof for a caller-declared
sensitive/write-adjacent approval posture.

The next question is which sensitive approval surfaces should adopt the helper
first.

This plan recommends a narrow adoption sequence:

1. add a high-assurance approval composition path that validates
   high-assurance controls and approval-presentation proof before any approval
   event is appended;
2. defer provider-write/write-adjacent adoption until the high-assurance path
   is implemented and reviewed;
3. keep public default approval behavior unchanged.

This plan does not implement runtime adoption.

## 2. Goals

- Select the first high-assurance/write-adjacent caller surfaces for
  approval-presentation proof adoption.
- Preserve deterministic approval semantics.
- Ensure proof-required approvals fail closed before approval mutation.
- Avoid duplicate approval events or double-application of approval decisions.
- Preserve existing ordinary approval behavior.
- Keep errors stable and non-leaking.
- Prepare provider-write gates without enabling provider writes.
- Keep default public approval behavior unchanged until a separate reviewed
  migration phase.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
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
- examples;
- release posture changes.

## 4. Current Implemented Baseline

Implemented foundations:

- `ApprovalPresentationRecord`;
- `ApprovalPresentationRecordStore`;
- approval-presentation content hashing;
- explicit opt-in proof-enforced approval path;
- optional proof freshness validation;
- approval decision proof markers;
- proof marker inspect/report/audit projection foundations;
- dogfood approval-presentation proof persistence and proof-enforced approval;
- `ApprovalPresentationDefaultEnforcementPolicy`;
- `ApprovalPresentationDefaultEnforcementMode`;
- `ApprovalPresentationSensitiveActionPosture`;
- `LocalExecutor::decide_approval_with_default_presentation_policy(...)`.

Still not implemented:

- public/default approval-presentation enforcement;
- high-assurance approval proof integration;
- provider-write approval-presentation proof integration;
- approval-card UI;
- workflow-declared approval-presentation requirements.

## 5. Candidate Adoption Surfaces

### Ordinary Approval Decisions

Classification: defer.

Reason: default public approval behavior should not change until selected
sensitive paths have proved the migration shape.

### Dogfood Material Phase Approvals

Classification: already adopted through the dogfood runner.

Reason: material dogfood phases already persist presentation proof and use the
proof-enforced approval path. Dogfood remains the benchmark, not the public
default.

### High-Assurance Approval Decisions

Classification: first implementation target.

Reason: these approvals already represent explicit sensitivity, authority,
required references, requester/approver separation, and report disclosure. They
are the safest first public-ish caller surface for proof adoption because the
caller already declares high-assurance intent.

### High-Assurance Approval With Disclosure

Classification: first implementation target, paired with high-assurance
approval decisions.

Reason: this path returns report-safe disclosure and is the most useful
high-assurance bridge for later work reports and artifacts.

### Approval-Resume Artifact/Projection Composition

Classification: defer until high-assurance proof integration is reviewed.

Reason: artifact/projection composition is write-adjacent. It should consume
proof markers after approval proof integration exists, not define the first
approval-presentation adoption surface itself.

### Provider-Write Preflight And Provider Calls

Classification: reject for first adoption, plan later.

Reason: provider-write gates are write-adjacent and security-sensitive. They
must not be wired to presentation-proof policy until high-assurance adoption is
implemented and reviewed. This plan does not enable provider writes.

### Report Artifact Writes

Classification: defer.

Reason: report artifacts already have explicit proof-marker gate helpers. The
approval decision path should produce durable proof markers before artifact
paths rely on them more broadly.

## 6. First Implementation Target Recommendation

The first implementation should add an explicit high-assurance
approval-presentation composition path.

Recommended shape:

- add a request type that combines:
  - existing `LocalHighAssuranceApprovalDecisionRequest`;
  - `ApprovalPresentationDefaultEnforcementPolicy`;
- add a local executor method such as
  `decide_approval_with_high_assurance_and_presentation_policy(...)`;
- add a matching disclosure-returning method only if it can share the same
  pre-mutation helper without duplicating logic.

The implementation must validate both high-assurance controls and
approval-presentation proof before any approval event is appended.

## 7. Required Composition Order

The implementation must not simply call one public approval method and then the
other.

Required order:

1. rehydrate the run;
2. resolve the pending approval request;
3. construct the approval decision;
4. validate high-assurance approval controls;
5. evaluate the approval-presentation policy;
6. if proof is required, resolve and validate presentation proof;
7. attach a proof marker to the approval decision when proof is required;
8. construct report-safe high-assurance disclosure if requested;
9. append the approval decision exactly once;
10. resume or fail closed according to the approval decision.

Failure before step 9 must append no approval decision events, resume no run,
invoke no skills, append no side-effect events, write no artifacts, and call no
provider.

## 8. Enforcement Policy Rules

For high-assurance adoption:

- `NotRequired` may remain available only for explicit compatibility tests and
  must preserve existing high-assurance behavior.
- `Required` should require matching durable presentation proof before
  approval mutation.
- `RequiredForSensitiveAction` should require explicit
  `ApprovalPresentationSensitiveActionPosture::HighAssurance`.
- `WriteAdjacent` and `SideEffect` posture should be rejected for the
  high-assurance-specific path unless a later implementation explicitly scopes
  those paths.

Do not infer sensitivity from free-form approval reasons, report notes, command
output, provider payloads, source snippets, or model opinion.

## 9. Error Handling

Errors must use stable codes and fixed messages.

Required error families:

- missing proof when proof is required;
- stale or mismatched proof, delegated to existing proof validation;
- missing sensitive posture when required;
- mismatched sensitive posture for high-assurance-only adoption;
- high-assurance validation failure, preserving existing high-assurance codes.

Errors must not include raw approval IDs, presentation IDs, run IDs, actor IDs,
approval reasons, handoff text, touched surfaces, command output, provider
payloads, source snippets, paths, tokens, credentials, or secret-like values.

## 10. Privacy And Redaction

The adoption path must:

- use existing validated proof records;
- use existing approval-presentation proof validation;
- use existing high-assurance validation;
- avoid storing or copying raw approval-card text;
- avoid copying provider payloads, command output, source contents, or
  transcript text;
- keep Debug output bounded and redaction-safe;
- preserve proof markers as references, not presentation payloads.

## 11. Test Plan

Future implementation tests should cover:

- `NotRequired` preserves existing high-assurance approval behavior;
- `Required` succeeds only with matching presentation proof;
- `Required` missing proof fails before approval events;
- stale/mismatched proof fails before approval events;
- `RequiredForSensitiveAction` requires explicit high-assurance posture;
- wrong posture fails before approval events;
- high-assurance validation failure occurs before proof-related mutation;
- proof validation failure occurs before approval mutation;
- approval decision is appended exactly once on success;
- denial paths preserve proof requirements where configured;
- disclosure-returning path returns report-safe disclosure without copying
  presentation payloads;
- Debug output does not leak approval IDs, presentation IDs, reasons, paths, or
  secret-like values;
- existing ordinary approval, proof-enforced approval, and high-assurance
  approval tests continue to pass;
- `cargo test --workspace` passes.

## 12. Documentation Requirements

Future docs must say:

- selected high-assurance approval-presentation adoption is implemented only
  when that phase is complete;
- default public approval behavior remains unchanged;
- provider-write approval-presentation adoption is not implemented;
- approval-card UI is not implemented;
- workflow schema fields are not implemented;
- CLI mutation behavior is not implemented;
- provider writes, side effects, hosted runtime, reasoning lineage, examples,
  and release posture changes remain unsupported.

## 13. Proposed Implementation Sequence

1. Add a shared private pre-mutation helper for high-assurance approval
   validation plus optional presentation-proof validation.
2. Add an explicit high-assurance approval-presentation request type.
3. Add one executor method for high-assurance approval with presentation policy.
4. Add disclosure-returning composition only if it reuses the same helper.
5. Add focused tests for success, fail-closed, non-leakage, and event counts.
6. Review.
7. Plan write-adjacent/provider-call adoption only after review.

## 14. Deferred Work

Deferred:

- ordinary approval default enforcement;
- workflow-declared approval-presentation requirements;
- provider-write proof enforcement;
- report artifact automatic adoption;
- approval-card UI;
- CLI approval UX;
- schemas;
- examples;
- hosted/distributed runtime;
- reasoning lineage;
- release posture changes.

## 15. Governed Dogfood Run

- workflow_id: `dg/d`
- run_id: `run-1783714044561976000-2`
- approval_id: `approval/run-1783714044561976000-2/planning-approved`
- approval presentation: `presentation/30b7d75c5667b6e5`
- approval presentation hash:
  `30b7d75c5667b6e591d54dcf68d1c659369005f122dd1ad160c75e18de031df6`
- approval outcome: granted by delegated maintainer
- approval reason: `approved-planning-phase`

Workflow OS governed the planning approval boundary. Codex performed repository
inspection, documentation authoring, validation, git, and PR work outside the
kernel.

## 16. Validation

Planning validation:

- `npm run check:docs` - passed;
- `git diff --check` - passed.

## 17. Final Recommendation

Proceed next to high-assurance approval-presentation adoption implementation.

The first implementation should be limited to an explicit local executor path
for high-assurance approvals. It must not flip public default approval
behavior, wire provider writes, add approval-card UI, add schemas, add CLI
mutation behavior, model new side effects, update examples, add hosted runtime,
add reasoning lineage, or change release posture.
