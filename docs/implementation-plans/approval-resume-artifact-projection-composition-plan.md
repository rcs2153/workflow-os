# Approval-Resume Artifact Projection Composition Plan

Status: implemented and reviewed for the proof-enforced approval-presentation resume path.

This plan follows the accepted [Explicit Projected Proof-Marker Artifact Composition Review](../concepts/EXPLICIT_PROJECTED_PROOF_MARKER_ARTIFACT_COMPOSITION_REVIEW.md).

The implementation is documented in [Approval-Resume Artifact Projection Composition Report](../concepts/APPROVAL_RESUME_ARTIFACT_PROJECTION_COMPOSITION_REPORT.md) and accepted in [Approval-Resume Artifact Projection Composition Review](../concepts/APPROVAL_RESUME_ARTIFACT_PROJECTION_COMPOSITION_REVIEW.md).

## 1. Executive Summary

Workflow OS now has an explicit local artifact path that can execute or rehydrate a terminal run, persist approval proof-marker projections, generate a WorkReport, evaluate artifact gates, and write a local report artifact.

Approval-resume remains separate. Existing approval APIs resume a waiting run and return `WorkflowRun`. They do not return report-bearing results, persist proof-marker projections for artifact gates, or write artifacts.

The next question is whether approval-resume should gain its own explicit artifact/projection composition path. This plan says yes, but only as a separate opt-in API. It must not change `decide_approval(...)`, `decide_approval_with_presentation(...)`, or existing approval semantics.

The first implementation is complete as the explicit
`decide_approval_with_report_artifact_and_projected_proof_markers(...)`
helper. It remains local, opt-in, caller-supplied-store bounded, and
executor-adjacent.

## 2. Goals

- Define a narrow approval-resume artifact/projection composition boundary.
- Preserve existing approval decision semantics.
- Preserve existing approval event ordering and idempotency behavior.
- Preserve existing `decide_approval(...)` and `decide_approval_with_presentation(...)` return types.
- Compose approval-resume, proof-marker projection persistence, report generation, artifact gates, and local artifact write only through an explicit API.
- Require explicit caller-supplied stores and policies.
- Ensure projection persistence and artifact failures do not retroactively change workflow pass/fail status.
- Keep errors stable and non-leaking.
- Avoid hand-populated projection stores after approval-resume completion.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- changes to default `decide_approval(...)`;
- changes to default `decide_approval_with_presentation(...)`;
- automatic report generation;
- automatic projection persistence;
- automatic artifact writing;
- CLI approval-resume artifact behavior;
- workflow schema changes;
- examples;
- runtime config;
- provider calls;
- provider writes;
- side-effect execution;
- hosted or distributed runtime behavior;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy changes;
- release posture changes.

## 4. Current Baseline

Implemented and reviewed:

- `LocalExecutor::decide_approval(...)`;
- `LocalExecutor::decide_approval_with_presentation(...)`;
- `LocalExecutor::decide_approval_with_high_assurance(...)`;
- `LocalExecutor::decide_approval_with_high_assurance_disclosure(...)`;
- terminal report generation helpers;
- explicit report artifact write helpers;
- store-backed approval proof-marker artifact gate helper;
- executor-adjacent approval proof-marker projection persistence helper;
- explicit projected proof-marker artifact composition helper for execution/rehydration paths.

Still not implemented:

- CLI surfaces for approval-resume artifacts;
- workflow-declared approval-resume artifact behavior.

Implemented in the first slice:

- proof-enforced approval-presentation resume plus terminal report generation;
- approval-resume projection persistence composition;
- approval-resume artifact write composition through existing artifact gates.

## 5. Problem Statement

Approval-gated workflows commonly become terminal only after an approval decision resumes execution. Today callers can:

1. call an approval API to resume the run;
2. separately persist approval proof-marker projections;
3. separately generate a report;
4. separately evaluate artifact gates;
5. separately write a report artifact.

That hand-composition is error-prone. It can also tempt callers to hand-populate projection stores or write artifacts without the same explicit ordering used by the accepted projected proof-marker artifact path.

Workflow OS should provide a reviewed, explicit approval-resume composition path before broader runtime exposure.

## 6. Recommended First Boundary

Add a new executor-adjacent helper/API rather than modifying existing approval methods.

Possible names:

```text
decide_approval_with_report_artifact_and_projected_proof_markers(...)
```

or:

```text
resume_approval_with_report_artifact_and_projected_proof_markers(...)
```

The implementation should choose the name that best matches existing repository conventions.

The helper should accept:

- `LocalExecutor`;
- explicit approval decision request;
- optional approval-presentation proof input when proof-enforced approval is required;
- optional high-assurance control/disclosure inputs only if existing explicit APIs can be reused without broadening scope;
- explicit `WorkReportArtifactStore`;
- explicit `SideEffectRecordStore`;
- explicit `LocalApprovalProofMarkerAuditProjectionStore`;
- existing `LocalExecutionReportInputs`;
- existing `LocalExecutionReportArtifactInputs`;
- projection persistence policy;
- proof-marker artifact gate policy;
- selected approval references.

## 7. Result Shape

Prefer a new result wrapper instead of changing existing approval return types.

Possible shape:

```text
LocalApprovalResumeWithProjectedProofMarkerArtifactResult
```

It should expose:

- the resumed `WorkflowRun`;
- optional `WorkReport`;
- optional report-generation error;
- optional `WorkReportArtifactRecord`;
- optional artifact-write error;
- optional projection persistence result;
- optional projection persistence error;
- existing artifact gate posture fields where available;
- read-only accessors;
- `into_parts()`.

If reuse of `LocalExecutionWithProjectedProofMarkerArtifactResult` is clean and does not distort naming, implementation may reuse it. The result should still clearly communicate that the run came from approval-resume, not a fresh execute path.

## 8. Approval Decision Variants

The first implementation should support only one approval-resume path if that keeps scope small.

Preferred first target:

- approval-presentation enforced resume, because proof markers are central to artifact projection.

Acceptable alternative:

- default approval resume plus marker-free projection policy, only if the helper clearly cannot satisfy `marker_required` artifact gates without presentation proof.

The implementation must not silently upgrade default approval behavior. If a proof-marker artifact gate requires proof markers, marker-free approval decisions must fail projection or artifact gating rather than pass.

## 9. Composition Order

Recommended order:

1. Rehydrate and validate the waiting run through the existing approval-resume logic.
2. Apply the approval decision through the selected explicit approval API.
3. If approval validation fails before events are appended, return the existing approval error unchanged.
4. If the resumed run is non-terminal, return the run with report/artifact status posture and no projection persistence.
5. If the resumed run is terminal, persist approval proof-marker projections from the resumed run into the caller-supplied projection store.
6. If projection persistence fails, return the run plus projection error and write no artifact.
7. Generate the terminal `WorkReport`.
8. Construct the `WorkReportArtifactRecord`.
9. Evaluate side-effect integrity, approval linkage, high-assurance disclosure, provider-candidate, and proof-marker artifact gates.
10. Write the artifact only if all requested gates pass.

This mirrors the accepted explicit execution artifact path while preserving approval-resume semantics.

## 10. Workflow Semantics

The helper must preserve workflow semantics:

- approval denial still fails closed according to existing approval behavior;
- approval grant still resumes exactly once;
- duplicate approval decisions remain rejected after terminal state;
- validation failure before approval event append must append no events;
- report/projection/artifact failure after a run exists must not change workflow pass/fail status;
- no failure may append extra workflow events;
- no failure may repair missing approval proof;
- no artifact may be partially written when requested gates fail.

Projection persistence may write bounded projection records before a later artifact gate fails. That is acceptable if the result clearly discloses that projection persistence succeeded but artifact writing did not.

## 11. Store And Runtime Boundaries

The helper must receive all stores explicitly:

- state backend through the existing executor;
- `WorkReportArtifactStore`;
- `SideEffectRecordStore`;
- `LocalApprovalProofMarkerAuditProjectionStore`.

It must not:

- discover hidden stores;
- infer store paths from specs;
- create runtime config;
- write projection records to the workflow state backend unless that backend is explicitly supplied through the reviewed store type;
- use report artifact stores as projection stores;
- add workflow schema fields for store locations.

## 12. Privacy And Redaction

The helper must not copy or persist:

- approval-presentation payloads;
- approval handoff text;
- approval reasons;
- report prose outside the already validated report artifact;
- command output;
- provider payloads;
- raw source or spec contents;
- environment values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- secret-like values.

Debug output must redact:

- stores;
- approval references;
- projection references;
- presentation references;
- report/artifact payloads;
- redaction metadata.

Errors must use stable codes and non-leaking messages.

## 13. Relationship To Workflow-Declared Requirements

The approval-resume composition path should preserve existing workflow-declared proof-marker artifact behavior:

- `not_required` does not force proof-marker gates;
- `projection_required` requires matching durable projection records when the explicit proof-marker-capable path is selected;
- `marker_required` requires present proof markers;
- default executor and approval paths still reject or avoid enforceable artifact requirements unless an explicit artifact-capable path is used.

Callers must not weaken authored workflow requirements.

## 14. Failure Semantics

Failure categories should remain distinct:

- approval validation failure: return the approval error and append no new invalid events;
- non-terminal resumed run: return run plus report/artifact status posture, no projection persistence;
- projection failure: return run plus projection error, no report and no artifact;
- report generation failure: return run plus report error, no artifact;
- artifact construction/gate/write failure: return run plus report and artifact error, no artifact record when write fails before persistence.

No category should be represented as a misleading user project diagnostic.

## 15. Test Plan

Future implementation tests should cover:

- approval-presentation grant resumes to terminal run, persists projection, and writes artifact;
- approval-presentation denial resumes to failed terminal run and handles report/artifact posture according to existing terminal report behavior;
- marker-free approval fails projection when selected marker is required;
- non-terminal approval-resume returns run with no projection persistence and no artifact;
- duplicate approval after terminal state remains rejected;
- approval proof mismatch fails before events and before projection/artifact work;
- projection failure writes no artifact and does not generate a report if that is the chosen failure ordering;
- artifact gate failure after projection writes no artifact and preserves projection posture;
- workflow-declared `projection_required` succeeds when projection persistence creates required records;
- workflow-declared `marker_required` fails when approval decision is marker-free;
- event history is unchanged by report/projection/artifact post-processing;
- no provider payloads, command output, approval reasons, presentation payloads, raw source/spec contents, tokens, or secret-like values leak through errors, debug, or serialization;
- existing `decide_approval(...)` behavior remains unchanged;
- existing `decide_approval_with_presentation(...)` behavior remains unchanged;
- existing execution artifact helper behavior remains unchanged;
- workspace tests continue to pass.

## 16. Documentation Updates For Future Implementation

The implementation phase should update:

- this plan;
- `ROADMAP.md`;
- relevant approval proof-marker and WorkReport artifact concept docs;
- an end-of-phase report.

Docs must clearly say:

- approval-resume artifact/projection composition is implemented only if the future implementation completes it;
- default approval behavior remains unchanged;
- automatic report artifact writing is not implemented;
- automatic projection persistence for all approvals is not implemented;
- CLI behavior is not implemented;
- schemas and examples are not updated;
- provider writes, side-effect execution, hosted behavior, reasoning lineage, and release posture changes remain unsupported.

## 17. Proposed Implementation Sequence

1. Add explicit approval-resume artifact/projection input and result types.
2. Reuse one existing approval decision path first, preferably approval-presentation enforced resume.
3. Call projection persistence after the resumed run exists and is terminal.
4. Reuse existing terminal report generation and artifact gate/write helpers.
5. Add focused tests for successful proof-enforced resume, projection failure, non-terminal posture, and unchanged default approval APIs.
6. Run full validation.
7. Create an implementation report.
8. Review before any CLI, default behavior, or automatic artifact expansion.

## 18. Open Questions

- Should the first implementation support only `decide_approval_with_presentation(...)`, or should it also support high-assurance disclosure resume?
- Should approval denial produce report artifacts in the first slice, or should failed terminal artifact writing remain separate until failure-report posture is reviewed?
- Should selected approval references default from the approval decision just made, report input approval references, or explicit projection input? This plan recommends explicit projection input first.
- Should one result wrapper be shared with execution artifact composition, or should approval-resume use a distinct wrapper for clarity?

## 19. Final Recommendation

Recommended next implementation phase: approval-resume artifact/projection composition helper, proof-enforced approval path first.

The implementation should remain local, explicit, opt-in, caller-supplied-store bounded, and executor-adjacent. It must not implement default approval behavior changes, automatic projection persistence, automatic artifact writing, CLI behavior, schemas, examples, provider writes, side-effect execution, hosted behavior, reasoning lineage, or release posture changes.

## 20. Governed Planning Record

- Dogfood workflow: `dg/d`
- Run ID: `run-1783690733983543000-2`
- Approval ID: `approval/run-1783690733983543000-2/planning-approved`
- Approval outcome: granted by delegated maintainer
- Approval presentation ID: `presentation/5fb7eb646e2453e7`
- Approval presentation hash: `5fb7eb646e2453e7498c6f435b9ed0ec31069fc375e68634ac986fb2d60edea2`
- Event summary: 39 events; `ApprovalGranted:1`, `ApprovalRequested:1`, `PolicyDecisionRecorded:8`, `RunCompleted:1`, `RunCreated:1`, `RunResumed:1`, `RunStarted:1`, `RunValidated:1`, `SkillInvocationRequested:6`, `SkillInvocationStarted:6`, `SkillInvocationSucceeded:6`, `StepScheduled:6`.
- Validation summary: `npm run check:docs` and `git diff --check` passed.
- Out-of-kernel work: planning documentation edits, documentation validation, git/PR actions, and GitHub merge actions remain performed by the delegated maintainer execution layer outside the Workflow OS runtime.
- Missing coverage disclosure: no implementation, runtime behavior, artifact writes, projection persistence, CLI behavior, schemas, examples, provider calls, provider writes, hosted behavior, or reasoning lineage were added by this planning phase.

## 21. Implementation Record

The proof-enforced approval-presentation resume helper is implemented and
reported in [Approval-Resume Artifact Projection Composition Report](../concepts/APPROVAL_RESUME_ARTIFACT_PROJECTION_COMPOSITION_REPORT.md).
Default approval behavior, automatic projection persistence, automatic report
artifact writing, CLI behavior, schemas, examples, provider writes,
side-effect execution, hosted behavior, reasoning lineage, and release posture
changes remain unimplemented.
