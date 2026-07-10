# High-Assurance Approval-Resume Artifact Projection Plan

Status: implemented for the explicit local grant path and accepted with
non-blocking follow-ups in [High-Assurance Approval-Resume Artifact Projection
Review](../concepts/HIGH_ASSURANCE_APPROVAL_RESUME_ARTIFACT_PROJECTION_REVIEW.md).

This plan follows the accepted [Approval-Resume Artifact Projection Composition Review](../concepts/APPROVAL_RESUME_ARTIFACT_PROJECTION_COMPOSITION_REVIEW.md). It defines the next bounded implementation slice for composing high-assurance approval resume with report disclosure, proof-marker projection persistence, artifact gates, and local report artifact writing.

## 1. Executive Summary

Workflow OS now has an explicit approval-resume artifact/projection helper for the proof-enforced approval-presentation path.

High-assurance approval is the adjacent sensitive-action boundary. It already has a core model, runtime validation, opt-in executor enforcement, WorkReport disclosure, artifact disclosure gates, and workflow-declared artifact requirement derivation.

The next question is whether approval-resume should gain a matching explicit high-assurance artifact/projection composition path. This plan says yes, but only as a separate opt-in API. It must not change `LocalExecutor::decide_approval(...)`, `LocalExecutor::decide_approval_with_high_assurance(...)`, `LocalExecutor::decide_approval_with_high_assurance_disclosure(...)`, or default artifact behavior.

The first implementation is complete as an explicit local helper. It requires
durable approval-presentation proof plus high-assurance approval validation
before applying the approval decision, then composes proof-marker projection,
WorkReport generation, high-assurance disclosure gates, and local report
artifact writing.

## 2. Goals

- Define a narrow high-assurance approval-resume artifact/projection composition boundary.
- Preserve default approval behavior.
- Preserve existing high-assurance approval decision semantics.
- Preserve workflow pass/fail semantics.
- Compose existing reviewed primitives rather than creating a new governance path.
- Require explicit high-assurance controls and supplied references.
- Require explicit caller-supplied stores and artifact policies.
- Carry bounded high-assurance disclosure into terminal WorkReport generation.
- Preserve workflow-declared high-assurance and proof-marker artifact requirements in explicit artifact-capable paths.
- Persist approval proof-marker projections only through the explicit projection store.
- Write local report artifacts only after all requested gates pass.
- Keep errors stable and non-leaking.

## 3. Non-Goals

This plan does not authorize:

- additional implementation outside the completed explicit helper;
- changing default `decide_approval(...)`;
- changing default `decide_approval_with_high_assurance(...)`;
- changing default `decide_approval_with_high_assurance_disclosure(...)`;
- automatic high-assurance approval enforcement;
- automatic report generation;
- automatic projection persistence;
- automatic artifact writing;
- CLI behavior;
- workflow schema changes;
- examples;
- runtime config;
- provider calls;
- provider writes;
- side-effect execution;
- approval evidence attachment;
- RBAC, IdP, SSO, SCIM, teams, groups, or external directory integration;
- quorum approval;
- role-bound approval authority;
- revocation enforcement;
- hosted or distributed runtime behavior;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy changes;
- release posture changes.

## 4. Current Foundation

Implemented and reviewed foundations:

- `LocalExecutor::decide_approval_with_high_assurance(...)`;
- `LocalExecutor::decide_approval_with_high_assurance_disclosure(...)`;
- `WorkReportHighAssuranceApprovalDisclosure`;
- terminal report input propagation for high-assurance disclosure;
- report artifact high-assurance disclosure gate;
- workflow-declared `report_artifact_requirements.high_assurance_approval`;
- pure runtime derivation of workflow-declared high-assurance artifact policy;
- explicit executor artifact path enforcement for workflow-declared high-assurance artifact requirements;
- approval proof-marker decision markers;
- approval proof-marker citation and audit projection helpers;
- durable local approval proof-marker projection store;
- store-backed proof-marker artifact gate;
- approval-resume artifact/projection composition for the proof-enforced presentation path.

Still not implemented:

- automatic high-assurance artifact behavior for approval resume;
- CLI exposure;
- workflow-declared high-assurance approval controls;
- RBAC/IdP/quorum/revocation enforcement.

## 5. Problem Statement

Before this slice, callers could resume a high-assurance approval and
separately compose:

1. high-assurance validation and approval decision;
2. high-assurance disclosure;
3. approval proof-marker projection persistence;
4. terminal WorkReport generation;
5. high-assurance disclosure artifact gates;
6. proof-marker artifact gates;
7. local report artifact writing.

That hand-composition was too easy to get wrong. It could also produce false confidence if a caller wrote a report artifact without carrying high-assurance disclosure or without proof-marker projection coverage.

Workflow OS now provides one explicit composition path for maintainer review before broader runtime exposure.

## 6. Recommended First Boundary

Add a new executor-adjacent helper/API rather than modifying existing approval methods.

Possible name:

```text
decide_approval_with_high_assurance_report_artifact_and_projected_proof_markers(...)
```

The implementation may choose a shorter name if it matches repository conventions, but the name must make clear that the path is:

- approval-resume;
- high-assurance;
- report-artifact capable;
- proof-marker projection capable;
- explicit and opt-in.

The implemented helper accepts:

- `LocalExecutor`;
- explicit `WorkReportArtifactStore`;
- explicit `SideEffectRecordStore`;
- explicit `LocalApprovalProofMarkerAuditProjectionStore` through
  `LocalExecutionProjectedProofMarkerArtifactInputs`;
- `LocalHighAssuranceApprovalDecisionRequest`;
- durable approval presentation proof and optional maximum presentation age;
- report inputs;
- optional side-effect discovery inputs;
- artifact inputs;
- projection persistence policy;
- selected approval references;
- projection sensitivity and redaction metadata;
- proof-marker artifact gate policy.

## 7. Result Shape

Prefer reusing or extending existing result wrappers only if the resulting API remains clear.

The result should expose:

- resumed `WorkflowRun`;
- optional `WorkReport`;
- optional report-generation error;
- optional `WorkReportArtifactRecord`;
- optional artifact-write error;
- optional high-assurance disclosure;
- optional high-assurance disclosure gate posture;
- optional approval proof-marker projection persistence result;
- optional projection persistence error;
- proof-marker artifact gate posture where available;
- read-only accessors;
- `into_parts()`.

The result must distinguish approval-resume failures from post-resume report, projection, or artifact failures.

## 8. Composition Order

Recommended order:

1. Rehydrate and validate the waiting run.
2. Resolve and validate durable approval-presentation proof.
3. Validate high-assurance controls before approval decision events are appended.
4. Apply the approval decision only if presentation proof and high-assurance validation succeed.
5. Produce bounded high-assurance disclosure from the validated decision path.
6. Derive workflow-declared high-assurance and proof-marker artifact requirements from immutable workflow/run identity.
7. If the resumed run is non-terminal, return the run plus report status posture and no projection persistence.
8. If the resumed run is terminal, persist approval proof-marker projections from the resumed run.
9. If projection persistence fails, return the run plus projection error and write no artifact.
10. Generate a terminal `WorkReport` carrying the high-assurance disclosure.
11. Construct the `WorkReportArtifactRecord`.
12. Evaluate side-effect integrity, approval linkage, high-assurance disclosure, provider-candidate, and proof-marker artifact gates.
13. Write the artifact only if all requested gates pass.

## 9. High-Assurance Disclosure Policy

The helper reuses the existing disclosure construction path rather than storing raw high-assurance controls.

The generated report may disclose:

- high-assurance validation was used;
- validation passed before approval decision events were appended;
- decision posture: granted or denied;
- requester/approver separation posture without actor IDs;
- required reference posture by stable references or bounded counts;
- expiration posture;
- revocation posture as unsupported/deferred unless separately implemented;
- denial behavior posture limited to existing fail-closed behavior;
- known limitations such as no RBAC, IdP, quorum, or revocation enforcement.

The report must not claim enterprise identity assurance, role-bound authority, quorum, revocation enforcement, protected-use expiration enforcement, write authorization, provider mutation approval, safety-critical certification, or Level 3/4 autonomy.

## 10. Artifact Gate Policy

The effective artifact policy should combine caller-supplied and workflow-declared requirements by strictness:

- high-assurance disclosure gate policy;
- approval proof-marker gate policy;
- side-effect integrity gate policy;
- approval linkage policy;
- provider-candidate event-proof policy where applicable.

Callers may strengthen authored requirements but must not weaken them.

Artifact write must fail closed before persistence when:

- required high-assurance disclosure is missing or insufficient;
- required proof-marker projection records are missing or mismatched;
- required side-effect integrity records are missing or mismatched;
- required approval linkage is missing or mismatched;
- provider-candidate event proof is required and absent.

## 11. Workflow Semantics

The helper must preserve existing semantics:

- high-assurance validation failure before decision event append returns the existing structured error and appends no invalid events;
- approval denial still fails closed according to existing approval behavior;
- approval grant resumes exactly once;
- duplicate approval decisions remain rejected after terminal state;
- projection/report/artifact failure after a run exists must not change workflow pass/fail status;
- no failure may append extra workflow events;
- no artifact may be partially written when requested gates fail.

Projection persistence may succeed before a later artifact gate fails. That is acceptable if the result clearly discloses projection success and artifact failure separately.

## 12. Store And Runtime Boundaries

The helper must receive all stores explicitly:

- state backend through the existing executor;
- `WorkReportArtifactStore`;
- `SideEffectRecordStore`;
- `LocalApprovalProofMarkerAuditProjectionStore`.

It must not:

- discover hidden stores;
- infer store paths from specs;
- create runtime config;
- call providers;
- execute side effects;
- write projection records to unrelated stores;
- add workflow schema fields for store locations.

## 13. Privacy And Redaction

The helper must not copy or persist:

- high-assurance control payloads;
- approval presentation payloads;
- approval handoff text;
- approval reasons;
- actor IDs outside already reviewed bounded posture;
- command output;
- provider payloads;
- raw source or spec contents;
- environment values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- secret-like values.

Debug output must redact stores, approval references, projection references, high-assurance disclosure payloads, report/artifact payloads, and redaction metadata.

Errors must use stable codes and non-leaking messages.

## 14. Test Plan

Implementation tests cover:

- high-assurance approval grant resumes and writes a gated artifact;
- missing required high-assurance reference fails before decision events;
- projection persistence succeeds before artifact write;
- successful artifact write requires both disclosure and proof-marker gates when requested;
- report carries bounded high-assurance disclosure without raw control payloads;
- debug output does not leak approval, presentation, actor, store, or report payloads;
- default approval paths remain unchanged;
- existing high-assurance, WorkReport, artifact, projection, and local executor tests still pass.

Deferred tests and behavior:

- high-assurance approval denial artifact support;
- same-actor rejection coverage through this composition helper;
- projection persistence failure through this composition helper;
- missing high-assurance disclosure gate failure through this composition helper;
- broader workflow-declared policy combinations beyond existing shared artifact-gate tests.

## 15. Proposed Implementation Sequence

Recommended small PR sequence:

1. Add request/result shape for high-assurance approval-resume artifact/projection composition.
2. Compose existing high-assurance disclosure result into terminal report inputs.
3. Reuse existing projection persistence and artifact finish path where possible.
4. Add focused success and failure tests.
5. Update docs and create an end-of-phase report.
6. Run maintainer review before any CLI, default executor, or provider-write expansion.

## 16. Open Questions

- Should the helper require disclosure-producing high-assurance approval resume, or can it accept a precomputed disclosure from the caller?
- Should denial-result artifacts be supported in the first implementation, or should the first slice focus on granted terminal completion?
- Can the existing `LocalExecutionWithProjectedProofMarkerArtifactResult` remain clear enough, or should the high-assurance path introduce a dedicated result wrapper?
- Should high-assurance disclosure gate failures preserve the generated report in the result, matching existing artifact behavior?
- How should future workflow-declared high-assurance approval controls relate to workflow-declared report artifact requirements?

## 17. Final Recommendation

Implemented phase: high-assurance approval-resume artifact/projection composition helper, grant path first.

The first implementation should stay explicit, local, caller-supplied-store bounded, and proof/report/artifact capable. It must not build CLI behavior, automatic artifact writing, default approval behavior changes, workflow-declared high-assurance approval controls, provider writes, hosted behavior, reasoning lineage, side-effect execution, or release posture changes.

Recommended next phase: high-assurance approval-resume artifact/projection composition review.

## 18. Governed Planning Record

- Dogfood workflow: `dg/d`.
- Run ID: `run-1783694836190044000-2`.
- Approval ID: `approval/run-1783694836190044000-2/planning-approved`.
- Approval outcome: granted by delegated maintainer after proof-enforced handoff.
- Approval presentation ID: `presentation/4c4db4aeb0b958f8`.
- Approval presentation hash: `4c4db4aeb0b958f8c8302d002fcb0df26cb9b9f08b832111ce00845aedc669c7`.

## 19. Validation

- `npm run check:docs` - passed.
- `git diff --check` - passed.
- `npm run dogfood:benchmark -- phase-close run-1783694836190044000-2 --phase planning` - passed.
