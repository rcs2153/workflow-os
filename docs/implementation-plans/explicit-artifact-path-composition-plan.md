# Explicit Artifact-Path Composition Plan

Status: implemented.

This plan follows the accepted [Executor-Adjacent Approval Proof-Marker Projection Persistence Helper Review](../concepts/EXECUTOR_ADJACENT_APPROVAL_PROOF_MARKER_PROJECTION_PERSISTENCE_HELPER_REVIEW.md).

## 1. Executive Summary

Workflow OS now has the individual primitives needed for a proof-marker-aware local report artifact path:

- explicit terminal execution with report artifact writing;
- side-effect citation integrity gates;
- approval-side-effect linkage gates;
- high-assurance approval disclosure gates;
- store-backed approval proof-marker artifact gates;
- workflow-declared proof-marker artifact requirement derivation;
- explicit executor-adjacent approval proof-marker projection persistence.

The next question is how to compose these already-reviewed primitives into one explicit, opt-in artifact-capable helper/API so callers do not hand-populate projection stores before artifact validation.

This plan is implemented by the explicit projected proof-marker artifact composition helper. The implementation does not make report artifacts automatic, change default executor behavior, add CLI behavior, add schemas, update examples, call providers, execute writes, broaden approval behavior, add hosted behavior, implement reasoning lineage, or change release posture.

## 2. Goals

- Compose accepted primitives into one explicit artifact-capable path.
- Persist bounded approval proof-marker projections from the terminal run before proof-marker artifact gates evaluate.
- Keep projection persistence local, caller-supplied-store bounded, and opt-in.
- Preserve terminal report generation and artifact gate behavior.
- Preserve workflow pass/fail semantics.
- Preserve artifact failure semantics.
- Avoid hand-populated proof-marker projection stores in explicit artifact paths.
- Keep errors stable and non-leaking.
- Add focused implementation guidance for a small future PR.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- default executor projection persistence;
- automatic report generation;
- automatic report artifact writing;
- automatic projection persistence for all approvals;
- CLI artifact rendering or commands;
- workflow schema changes;
- examples;
- runtime config;
- provider calls;
- provider writes;
- side-effect execution;
- approval-resume API changes;
- hosted or distributed runtime behavior;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy changes;
- release posture changes.

## 4. Current Baseline

Implemented and reviewed:

- `execute_with_report_artifact_and_side_effect_gates(...)`;
- `execute_with_report_artifact_and_proof_marker_gates(...)`;
- `write_work_report_artifact_with_governance_gates(...)`;
- `validate_work_report_artifact_approval_proof_marker_gate_from_store(...)`;
- `persist_approval_proof_marker_projections_for_run(...)`;
- workflow-declared `report_artifact_requirements.approval_proof_markers` derivation in the explicit proof-marker artifact path;
- local approval-presentation proof records and approval decision proof markers.

Implemented in this phase:

- one explicit executor-adjacent path that persists proof-marker projections and then writes proof-marker-gated artifacts.

Still not implemented:

- default executor behavior;
- automatic artifact writing;
- CLI artifact behavior;
- workflow-declared projection-store configuration.

## 5. Problem Statement

The proof-marker-gated artifact executor path can enforce proof-marker artifact requirements, but it currently requires the caller to supply a projection store that has already been populated.

The new projection persistence helper closes the hand-population gap, but callers still need to manually orchestrate:

1. execute or rehydrate a terminal run;
2. persist approval proof-marker projections from the run;
3. generate the terminal report;
4. validate artifact gates;
5. write the artifact.

That orchestration is exactly the kind of runtime composition Workflow OS should make explicit and testable.

## 6. Recommended First Boundary

Add a new explicit helper/API adjacent to existing artifact paths. Possible name:

```text
execute_with_report_artifact_and_projected_proof_markers(...)
```

or an equivalent name aligned with repository conventions.

The helper should accept:

- `LocalExecutor`;
- explicit `WorkReportArtifactStore`;
- explicit `SideEffectRecordStore`;
- explicit `LocalApprovalProofMarkerAuditProjectionStore`;
- `LocalExecutionWithReportArtifactRequest`;
- `ApprovalProofMarkerProjectionPersistencePolicy`;
- `WorkReportArtifactApprovalProofMarkerGatePolicy`;
- optional selected approval references for projection persistence.

The helper should return an additive result that preserves the existing artifact result and includes projection persistence posture.

## 7. Recommended Result Shape

Prefer adding a narrow result wrapper rather than changing `LocalExecutionWithReportArtifactResult`.

Possible shape:

```text
LocalExecutionWithProjectedProofMarkerArtifactResult
```

It should expose:

- the existing `LocalExecutionWithReportArtifactResult`;
- optional `ApprovalProofMarkerProjectionPersistenceResult`;
- optional projection persistence error;
- read-only accessors;
- `into_parts()`.

Projection persistence failure after a run exists should not erase the run or generated report posture if those exist.

## 8. Composition Order

The first implementation should use this order:

1. Execute through the same selected workflow and capability path used by `execute_with_report_artifact_and_proof_marker_gates(...)`.
2. If execution fails before a run exists, return the execution `Err` unchanged.
3. If the run is non-terminal, return the run with the existing terminal-report status error and no projection persistence.
4. Persist approval proof-marker projections from the run into the caller-supplied projection store.
5. If projection persistence fails, return the run with a projection error and no artifact write.
6. Generate the terminal `WorkReport`.
7. Construct `WorkReportArtifactRecord`.
8. Validate provider-candidate, side-effect integrity, approval-linkage, high-assurance disclosure, and proof-marker gates.
9. Write the artifact only if all requested gates pass.

This ordering lets the proof-marker gate validate durable projection posture without relying on hand-populated stores.

## 9. Policy Composition

The helper must compose two policy layers:

- projection persistence policy: what approval decision events should be projected;
- artifact proof-marker gate policy: what report artifact approval citations must prove.

Workflow-declared proof-marker artifact requirements must continue to strengthen caller proof-marker gate policy by strictness. Callers must not weaken authored workflow requirements.

The projection persistence policy should remain caller-supplied. Recommended default for helper convenience, if one is provided:

- persist proof-marked granted and denied approval decisions;
- skip marker-free decisions unless selected approvals are required;
- treat matching duplicates as already present;
- fail closed on conflicting duplicates.

## 10. Failure Semantics

Failure semantics must preserve the existing model:

- execution failure before a run exists returns `Err`;
- report generation failure after a run exists is report posture, not execution failure;
- projection persistence failure after a run exists is projection posture, not execution failure;
- artifact gate/write failure after report generation is artifact posture, not execution failure;
- no failure may retroactively change workflow pass/fail status;
- no failure may append workflow events;
- no failure may repair missing approval proof;
- no partial artifact may be written if any requested gate fails.

Projection persistence can write bounded projection records before a later artifact gate fails. That is acceptable if disclosed: projection persistence records observed approval decision proof posture; artifact failure records that no artifact was written.

## 11. Store Boundaries

The future helper must receive all stores explicitly:

- state backend through the existing executor;
- `WorkReportArtifactStore`;
- `SideEffectRecordStore`;
- `LocalApprovalProofMarkerAuditProjectionStore`.

It must not:

- discover projection stores from runtime state;
- infer projection-store paths from specs;
- create hidden stores;
- write projection records to the workflow state backend unless that backend is explicitly supplied as the projection store root through the reviewed store type;
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
- tokens;
- authorization headers;
- private keys;
- secret-like values.

Debug output must redact:

- stores;
- workflow/run IDs where established local patterns redact them;
- approval references;
- projection references;
- presentation references;
- report/artifact payloads.

Errors must use stable codes and non-leaking messages.

## 13. Workflow-Declared Requirement Relationship

The new helper should preserve the accepted workflow-declared proof-marker artifact requirement behavior:

- `not_required` does not force proof-marker gates;
- `projection_required` requires matching durable projection records when the explicit proof-marker-capable path is selected;
- `marker_required` requires present proof markers, not merely marker-free compatibility;
- default executor paths still reject enforceable workflow-declared proof-marker artifact requirements.

The helper must not make default validation more permissive.

## 14. Test Plan

Future implementation tests should cover:

- completed approval-gated run with proof marker persists projection and writes artifact;
- denied approval with proof marker persists projection when policy includes denied decisions and returns failed run posture without artifact generation if terminal report behavior requires it;
- marker-free selected approval fails projection before artifact write when required;
- missing selected approval returns stable projection error without artifact write;
- matching duplicate projection reports already present and still permits artifact write;
- conflicting duplicate projection fails before artifact write;
- workflow-declared `projection_required` succeeds when projection persistence creates required records;
- workflow-declared `marker_required` fails when only marker-free projection exists;
- projection persistence failure preserves run semantics and appends no events;
- artifact gate failure after projection persistence writes no artifact;
- no provider payloads, command output, raw spec/source contents, approval reasons, presentation payloads, tokens, or secret-like values leak through errors, debug, or serialization;
- existing `execute_with_report_artifact_and_side_effect_gates(...)` behavior remains unchanged;
- existing `execute_with_report_artifact_and_proof_marker_gates(...)` behavior remains unchanged;
- workspace tests continue to pass.

## 15. Documentation Updates For Future Implementation

The implementation phase should update:

- this plan;
- `ROADMAP.md`;
- relevant WorkReport/proof-marker artifact concept docs;
- an end-of-phase report.

Docs must clearly say:

- explicit projected proof-marker artifact composition is implemented;
- default executor behavior is unchanged;
- automatic report artifact writing is not implemented;
- automatic projection persistence for all approvals is not implemented;
- CLI behavior is not implemented;
- schemas and examples are not updated;
- provider writes, side-effect execution, hosted behavior, reasoning lineage, and release posture changes remain unsupported.

## 16. Proposed Implementation Sequence

1. Add explicit input/result types for projected proof-marker artifact composition.
2. Reuse the existing artifact-capable selected-workflow execution path.
3. Call `persist_approval_proof_marker_projections_for_run(...)` after a terminal run exists and before artifact gate validation.
4. Reuse existing terminal report generation and artifact gate/write helpers.
5. Add focused tests for success, projection failure, duplicate posture, workflow-declared requirement composition, and non-leakage.
6. Run full validation.
7. Create an implementation report.
8. Review before any CLI, default behavior, or approval-resume expansion.

## 17. Open Questions

- Should projection persistence run before or after report generation when both are possible? This plan recommends before artifact gate validation and before artifact write, but implementation should choose the smallest code path that preserves existing report error semantics.
- Should selected approval references default from report input approval references, artifact gate requirements, or an explicit projection input? This plan recommends explicit input first.
- Should denied proof-marker projection be included by default in artifact-capable composition? Existing projection helper default includes denied decisions, but artifact generation for failed runs must remain consistent with terminal report behavior.
- Should result wrappers expose projection persistence posture even when artifact writing later fails? This plan recommends yes.

## 18. Final Recommendation

Implemented phase: explicit projected proof-marker artifact composition helper/API.

It remains local, explicit, opt-in, and executor-adjacent. It does not implement default executor behavior, automatic projection persistence, automatic artifact writing, CLI behavior, schemas, examples, provider writes, side-effect execution, hosted behavior, reasoning lineage, or release posture changes.

## 19. Governed Planning Record

- Dogfood workflow: `dg/d`
- Run ID: `run-1783687352134614000-2`
- Approval ID: `approval/run-1783687352134614000-2/planning-approved`
- Approval outcome: granted
- Approval presentation ID: `presentation/ee8a14db73009e57`
- Approval presentation hash: `ee8a14db73009e571e83f89bcedc4a4936b892313bce36a3b1da1581847745c2`
- Event summary: 39 events, including one approval request, one approval grant, eight policy decisions, six scheduled skills, and completed terminal status.
- Validation summary: `npm run check:docs` and `git diff --check` passed before phase close.
- Out-of-kernel work: repository documentation edits, documentation validation, git commit/PR actions, and GitHub merge actions are performed by the maintainer/Codex execution layer outside the Workflow OS runtime.
- Missing coverage disclosure: no implementation, runtime checks, artifact writes, projection persistence, CLI behavior, schemas, examples, provider calls, provider writes, hosted behavior, or reasoning lineage were added by this planning phase.
