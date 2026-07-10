# Workflow-Declared Proof-Marker Artifact Runtime Derivation Plan

Status: pure runtime derivation helper implemented in [Workflow-Declared Proof-Marker Artifact Runtime Derivation Helper Report](../concepts/WORKFLOW_DECLARED_PROOF_MARKER_ARTIFACT_RUNTIME_DERIVATION_HELPER_REPORT.md). Executor integration, artifact writing, projection persistence, CLI behavior, examples, provider writes, hosted behavior, reasoning lineage, and release behavior remain unimplemented.

## 1. Executive Summary

Workflow specs can now declare `report_artifact_requirements.approval_proof_markers` as bounded vocabulary. Default semantic validation rejects enforceable postures because no runtime path currently derives and enforces those declarations.

The next implementation question is how to derive an effective approval proof-marker artifact gate policy from a selected workflow declaration and a caller-supplied artifact policy. The helper should be pure, deterministic, local, and side-effect free. It should prepare executor artifact-path integration without itself writing artifacts or changing executor behavior.

This plan does not implement the helper.

## 2. Goals

- Derive `WorkReportArtifactApprovalProofMarkerGatePolicy` from a selected workflow's `approval_proof_markers` declaration.
- Compose workflow-declared policy with caller-supplied policy without allowing callers to weaken workflow requirements.
- Preserve caller ability to request stricter proof-marker policy than the workflow declares.
- Keep derivation scoped to one selected workflow.
- Preserve default validation and executor behavior.
- Avoid artifact writes, projection persistence, state mutation, event emission, and CLI behavior.
- Return stable, non-leaking errors for invalid or unsupported derivation inputs.
- Prepare a small implementation prompt that can be reviewed before executor artifact-path integration.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- executor artifact-path integration;
- automatic report artifact writing;
- automatic report generation;
- automatic approval proof-marker projection persistence;
- default executor proof-marker enforcement;
- changing `LocalExecutor::execute(...)`;
- changing `LocalExecutor::execute_with_report(...)`;
- CLI rendering or artifact commands;
- examples;
- public approval cards;
- provider writes;
- side-effect execution;
- hosted or distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 4. Current Baseline

Implemented:

- approval decision proof markers for proof-enforced approvals;
- approval proof-marker inspect/projection helpers;
- WorkReport approval proof-marker citation helpers;
- terminal report opt-in proof-marker citation integration;
- local durable approval proof-marker projection persistence helper;
- in-memory and store-backed report artifact proof-marker gate helpers;
- explicit executor artifact path proof-marker gate integration for caller-supplied policy;
- internal `WorkReportArtifactApprovalProofMarkerRequirement` model and mapping;
- workflow YAML/schema/SDK vocabulary for `approval_proof_markers`;
- default semantic validation rejecting enforceable workflow declarations until runtime enforcement exists.

Not implemented:

- runtime derivation from workflow declarations;
- composition between workflow-declared and caller-supplied proof-marker policies;
- artifact-capable validation posture for workflow-declared proof-marker requirements;
- executor artifact-path derivation from selected workflow.

## 5. Proposed Helper Boundary

Add a small pure helper in a future implementation phase, likely near existing report artifact requirement and gate policy helpers.

Candidate names:

- `derive_workflow_approval_proof_marker_artifact_gate_policy`
- `derive_report_artifact_approval_proof_marker_gate_policy`
- `compose_workflow_approval_proof_marker_artifact_requirement`

Candidate input:

- selected workflow identifier or loaded workflow reference;
- workflow-declared `WorkReportArtifactApprovalProofMarkerRequirement`;
- caller-supplied `WorkReportArtifactApprovalProofMarkerGatePolicy`;
- derivation posture indicating whether workflow-declared enforcement is allowed for this artifact-capable path.

Candidate output:

- effective `WorkReportArtifactApprovalProofMarkerGatePolicy`;
- bounded derivation metadata for tests/debugging, if useful;
- stable error when derivation is requested in a non-artifact-capable posture.

The helper must not read workflow specs from disk, inspect stores, write files, append events, or derive from all workflows in a project.

## 6. Composition Rules

The effective policy must be the stricter of workflow-declared and caller-supplied requirements.

Recommended ordering from least to most strict:

1. no proof-marker gate required;
2. projection required, marker-free compatible;
3. marker required.

Rules:

- workflow `not_required` plus caller disabled -> disabled;
- workflow `not_required` plus caller projection required -> projection required;
- workflow `not_required` plus caller marker required -> marker required;
- workflow `projection_required` plus caller disabled -> projection required;
- workflow `projection_required` plus caller marker-free compatible -> projection required;
- workflow `projection_required` plus caller marker required -> marker required;
- workflow `marker_required` plus any weaker caller policy -> marker required;
- workflow `marker_required` plus caller marker required -> marker required.

The caller may strengthen but must not weaken workflow-declared requirements.

## 7. Artifact-Capable Posture

Default validation rejects enforceable declarations because default paths do not enforce them. The derivation helper should therefore require an explicit artifact-capable posture before accepting `projection_required` or `marker_required`.

Rules:

- non-artifact-capable derivation with `not_required` may return caller policy unchanged;
- non-artifact-capable derivation with `projection_required` or `marker_required` must fail closed;
- artifact-capable derivation may compose enforceable workflow declarations with caller policy;
- errors must use stable codes and not leak workflow paths, raw spec fragments, approval payloads, local paths, or caller metadata.

Candidate error code:

- `work_report_artifact.approval_proof_marker.derivation.runtime_not_artifact_capable`

## 8. Relationship To Validation

This helper does not by itself change `workflow-os validate`.

Future validation behavior may use the helper only in an explicitly artifact-capable validation path. Default project validation should continue rejecting enforceable declarations until the selected runtime path can prove it will enforce them.

Validation must remain:

- deterministic;
- selected-workflow scoped when artifact-capable;
- read-only;
- non-leaking;
- free of artifact writes, projection persistence, state mutation, and event emission.

## 9. Relationship To Executor Artifact Path

Executor artifact integration remains deferred.

When implemented later, the explicit artifact path should:

- load or receive the selected workflow declaration;
- derive the workflow proof-marker artifact gate policy;
- compose it with caller-supplied artifact policy;
- validate approval proof-marker projections before artifact write;
- fail before writing when the effective gate is not satisfied;
- preserve existing workflow pass/fail semantics;
- avoid weakening workflow-declared requirements through caller inputs.

## 10. Privacy And Redaction

The derivation helper must operate on bounded enum posture and gate policy only.

It must not store, copy, log, debug-print, or serialize:

- approval presentation payloads;
- approval reasons;
- approval IDs;
- event IDs;
- projection IDs;
- presentation IDs;
- content hashes;
- report text;
- local paths;
- provider payloads;
- command output;
- source or spec contents;
- environment variable values;
- credentials, tokens, authorization headers, private keys, or secret-like metadata.

Debug output must be bounded and must not include workflow paths or raw spec fragments.

## 11. Error Handling

Errors should be structured and stable.

Recommended behavior:

- unsupported derivation posture fails closed;
- unknown future enum value, if representable, fails closed;
- contradictory policy inputs fail closed;
- errors do not include raw workflow YAML, local paths, approval payloads, projection payloads, or caller-supplied free text;
- no partial policy should be returned on failure.

## 12. Test Plan

Future implementation tests should cover:

- `not_required` plus disabled caller policy returns disabled;
- `not_required` plus stricter caller policy preserves stricter caller policy;
- `projection_required` strengthens disabled caller policy;
- `projection_required` plus marker-required caller policy returns marker required;
- `marker_required` strengthens all weaker caller policies;
- caller cannot weaken workflow-declared requirement;
- non-artifact-capable derivation rejects enforceable workflow declarations;
- artifact-capable derivation accepts enforceable workflow declarations;
- derivation is selected-workflow scoped;
- helper does not read files, write files, append events, mutate runtime state, or access stores;
- errors use stable codes;
- debug output is bounded and non-leaking;
- existing WorkReport artifact gate tests continue to pass;
- existing workflow schema/parser/SDK tests continue to pass.

Required validation for future implementation:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`

Run `npm run check:contracts` only if schema or SDK contract surfaces change; the intended helper implementation should not require contract changes.

## 13. Proposed Implementation Sequence

1. Add the pure derivation/composition helper and focused tests.
2. Review derivation helper.
3. Plan executor artifact-path integration using the helper.
4. Implement explicit executor artifact-path policy derivation and pre-write gate composition.
5. Review executor artifact-path integration.
6. Only after review, consider automatic projection persistence planning.

## 14. Deferred Work

Deferred:

- executor artifact-path integration;
- artifact-capable validation path;
- automatic approval proof-marker projection persistence;
- automatic report artifact writing;
- automatic report generation;
- default executor enforcement;
- CLI rendering or artifact commands;
- examples;
- public approval cards;
- provider writes;
- side-effect execution;
- hosted or distributed runtime;
- reasoning lineage;
- release posture changes.

## 15. Final Recommendation

Proceed next with a pure runtime derivation helper implementation only.

Do not implement executor integration, artifact writes, projection persistence, CLI behavior, examples, provider writes, hosted behavior, reasoning lineage, or release posture changes in that implementation phase.
