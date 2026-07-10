# Report Artifact Proof-Marker Write Composition Plan

Status: implemented as a helper-level, explicit artifact-write composition path in [Report Artifact Proof-Marker Write Composition Helper Report](../concepts/REPORT_ARTIFACT_PROOF_MARKER_WRITE_COMPOSITION_HELPER_REPORT.md). This follows the accepted store-backed validation helper in [Report Artifact Approval Proof Marker Store-Backed Gate Helper Review](../concepts/REPORT_ARTIFACT_APPROVAL_PROOF_MARKER_STORE_BACKED_GATE_HELPER_REVIEW.md).

This plan implemented only the explicit helper-level Rust path. It does not implement executor default behavior, automatic persistence, automatic artifact writing, CLI behavior, schemas, examples, provider writes, hosted behavior, reasoning lineage, or release posture changes.

## 1. Executive Summary

Workflow OS already has an explicit artifact-capable executor path that can compose terminal local report generation, `SideEffect` referential integrity, approval-side-effect linkage, high-assurance disclosure posture, and a caller-supplied report artifact store.

Workflow OS also now has an accepted store-backed approval proof-marker gate helper. That helper reads bounded projection records from a caller-supplied `LocalApprovalProofMarkerAuditProjectionStore` and validates proof-marker citation posture without writing artifacts.

The next implementation question is how to compose these foundations before an explicit report artifact write. The first implementation should remain additive and opt-in. It should not make artifact writing automatic, change existing executor methods, infer hidden stores, persist proof-marker projections, add CLI rendering, add schemas, update examples, call providers, enable writes, implement hosted behavior, or change release posture.

## 2. Goals

- Add a future explicit artifact-write composition path that can require store-backed approval proof-marker coverage before writing a `WorkReportArtifactRecord`.
- Reuse `validate_work_report_artifact_approval_proof_marker_gate_from_store(...)`.
- Preserve existing workflow execution semantics.
- Preserve existing report-generation and artifact-write semantics for callers that do not opt in to the proof-marker gate.
- Keep all stores caller-supplied.
- Keep gate results bounded and count-only.
- Fail closed before artifact write when strict proof-marker policy is requested and not satisfied.
- Return stable, non-leaking errors.
- Avoid copying approval-presentation payloads, approval reasons, report text, command output, provider payloads, source contents, local paths, credentials, tokens, or secret-like values.

## 3. Non-Goals

This phase must not implement:

- code changes in this planning prompt;
- automatic report artifact writing;
- executor default proof-marker gate enforcement;
- automatic approval proof-marker projection persistence;
- workflow-declared proof-marker artifact requirements;
- CLI rendering or commands;
- schemas;
- examples;
- public approval cards;
- provider writes;
- runtime side-effect execution;
- hosted or distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 4. Current Baseline

Implemented and reviewed foundations include:

- explicit `WorkReportArtifactStore` writes;
- `write_work_report_artifact_with_side_effect_integrity_and_approval_linkage(...)`;
- `execute_with_report_artifact_and_side_effect_gates(...)`;
- `SideEffect` referential integrity validation for cited records;
- store-backed approval-side-effect linkage validation;
- high-assurance disclosure gate composition in the existing artifact path;
- terminal report proof-marker citation propagation as explicit caller input;
- `LocalApprovalProofMarkerAuditProjectionStore`;
- `validate_work_report_artifact_approval_proof_marker_gate_from_store(...)`.

The missing composition is a single explicit path that validates store-backed proof-marker posture before a report artifact write.

## 5. Composition Problem

`WorkReport` and `WorkReportArtifactRecord` can cite approval proof-marker posture by stable reference, but citation alone should not be treated as proof that durable projection records exist.

Without an explicit store-backed gate in the artifact-write path, a future operator could over-trust an artifact that cites approval proof markers but was never checked against the durable local projection store.

The fix should be runtime composition of existing primitives, not a new primitive family. The artifact write path should be able to prove:

- the artifact and terminal run identity match;
- cited `SideEffect` records exist when required;
- approval-side-effect linkage is valid when required;
- high-assurance disclosures are satisfied when required;
- approval proof-marker citations resolve to persisted projection records when required;
- the artifact is written only after requested gates pass.

## 6. Recommended First Implementation Boundary

The first implementation should add a new explicit helper adjacent to existing governed artifact-write helpers.

Prefer an additive helper over changing existing `write_work_report_artifact_with_side_effect_integrity_and_approval_linkage(...)` behavior. Existing callers should not suddenly require a proof-marker projection store.

Recommended helper posture:

- accepts a validated report artifact or enough input to build one through existing constructors;
- accepts the terminal `WorkflowRun`;
- accepts caller-supplied artifact, `SideEffect`, approval-linkage, and proof-marker projection stores;
- accepts explicit gate policies;
- validates all requested gates in deterministic order;
- writes the artifact only if every strict requested gate passes;
- returns bounded gate counts and artifact write posture.

## 7. Candidate API Shape

Implementation may choose more idiomatic names, but the model should stay small.

Candidate input:

```rust
pub struct WorkReportArtifactGovernanceGatedWriteInput<'a> {
    pub artifact: &'a WorkReportArtifactRecord,
    pub terminal_run: &'a WorkflowRun,
    pub artifact_store: &'a WorkReportArtifactStore,
    pub side_effect_store: &'a SideEffectRecordStore,
    pub approval_linkage_store: &'a LocalApprovalSideEffectLinkageStore,
    pub approval_proof_marker_projection_store: &'a LocalApprovalProofMarkerAuditProjectionStore,
    pub side_effect_integrity_policy: WorkReportArtifactSideEffectIntegrityPolicy,
    pub approval_linkage_policy: WorkReportArtifactApprovalLinkagePolicy,
    pub high_assurance_disclosure_policy: WorkReportArtifactHighAssuranceDisclosurePolicy,
    pub approval_proof_marker_policy: WorkReportArtifactApprovalProofMarkerGatePolicy,
}
```

Candidate helper:

```rust
pub fn write_work_report_artifact_with_governance_gates(
    input: WorkReportArtifactGovernanceGatedWriteInput<'_>,
) -> Result<WorkReportArtifactGovernanceGatedWriteResult, WorkflowOsError>
```

If existing concrete store types differ from these placeholder names, the implementation should use the repository's current names and avoid broad trait generalization unless already present.

## 8. Gate Order

The first implementation should use a deterministic order that fails before artifact write:

1. validate the `WorkReportArtifactRecord`;
2. verify artifact/report identity against the supplied terminal run;
3. validate `SideEffect` referential integrity;
4. validate approval-side-effect linkage;
5. validate high-assurance disclosure posture when requested;
6. validate approval proof-marker projection posture from the caller-supplied store;
7. write the artifact.

This order keeps cheap local validation first and prevents artifact writes when proof-marker projection requirements are not satisfied.

## 9. Failure Semantics

Gate failure must not retroactively change workflow pass/fail semantics.

If a workflow run and report already exist, artifact gate failure should return the run/report context with a structured artifact error in the explicit caller result path. It must not append workflow events, mutate runtime state, fabricate evidence, or create partial artifacts.

Errors must use stable codes and must not leak:

- report IDs;
- run IDs;
- approval IDs;
- presentation IDs;
- projection IDs;
- local paths;
- report text;
- approval handoff text;
- approval reasons;
- command output;
- provider payloads;
- source or spec contents;
- credentials, tokens, authorization headers, private keys, or secret-like values.

## 10. Store And Persistence Boundary

All stores must be caller-supplied.

The helper must not:

- infer artifact store roots;
- infer projection store roots;
- create stores;
- persist projection records;
- persist side-effect records;
- append workflow events;
- emit audit or observability events;
- mutate workflow state.

The proof-marker gate should read only from the supplied `LocalApprovalProofMarkerAuditProjectionStore` through the accepted store-backed helper.

## 11. Privacy And Redaction

The composition path must preserve the strictest privacy posture of the existing gates:

- report artifacts store references and bounded summaries, not raw payloads;
- proof-marker gate results remain count-only;
- approval proof-marker projection records are treated as bounded proof posture, not approval payloads;
- local paths and store internals are not exposed through public errors or `Debug`;
- serialized results do not include raw provider payloads, command output, parser payloads, source contents, approval reasons, or secret-like values.

## 12. Executor And API Posture

Do not change existing methods:

- `LocalExecutor::execute(...)`;
- `LocalExecutor::execute_with_report(...)`;
- `execute_with_report_artifact_and_side_effect_gates(...)`;
- existing provider-candidate report artifact helpers.

If an executor-adjacent path is added later, it should be new and explicit, for example:

- `execute_with_report_artifact_and_governance_gates(...)`; or
- `execute_with_report_artifact_and_proof_marker_gate(...)`.

The first implementation should start helper-level if that is smaller. Executor integration should follow only after helper tests prove the gate order and failure posture.

## 13. Test Plan

Future implementation tests should cover:

- successful artifact write when all gates pass;
- strict proof-marker gate failure before artifact write when projection records are missing;
- strict proof-marker gate failure before artifact write when projections are marker-free;
- explicit marker-free policy behavior;
- explicit permissive missing-projection behavior;
- `SideEffect` integrity failure before approval-linkage, proof-marker validation, or artifact write;
- approval-linkage failure before proof-marker validation or artifact write;
- high-assurance disclosure failure before proof-marker validation or artifact write;
- artifact/run identity mismatch failure with no artifact write;
- non-terminal run rejection with no artifact write;
- corrupt projection store data maps to non-leaking artifact gate errors;
- artifact store remains empty on every gate failure;
- existing artifact path without proof-marker policy remains unchanged;
- existing executor methods still do not write artifacts automatically;
- `Debug` and serialization do not leak report text, approval reasons, IDs, paths, provider payloads, command output, or secret-like values.

## 14. Proposed Implementation Sequence

1. Add a helper-level governance-gated artifact write input/result type.
2. Compose existing artifact validation, terminal-run identity, `SideEffect` integrity, approval-linkage, high-assurance disclosure, and store-backed proof-marker gates.
3. Add focused helper tests for success and fail-closed ordering.
4. Review the helper phase.
5. Add a separate explicit executor-adjacent API only after helper review.
6. Defer CLI, schemas, examples, automatic artifact writing, workflow-declared requirements, hosted behavior, provider writes, and reasoning lineage.

## 15. Deferred Work

- Automatic approval proof-marker projection persistence from runtime paths.
- Workflow-declared proof-marker artifact requirements.
- Automatic artifact writing for all terminal runs.
- CLI artifact inspection or export.
- Public schemas or examples.
- Hosted/distributed artifact stores.
- Provider writes.
- Reasoning lineage.
- Release posture changes.

## 16. Final Recommendation

The next implementation phase should add a helper-level, explicit governance-gated report artifact write path that composes the accepted store-backed approval proof-marker gate with the existing artifact gates.

It must remain opt-in, local, caller-supplied-store based, and fail-closed before artifact write. It must not change executor defaults, generate reports automatically, persist projections automatically, expose CLI behavior, add schemas/examples, enable provider writes, or change release posture.
