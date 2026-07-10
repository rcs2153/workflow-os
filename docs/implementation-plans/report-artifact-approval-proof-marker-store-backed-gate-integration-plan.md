# Report Artifact Approval Proof Marker Store-Backed Gate Integration Plan

Status: first explicit store-backed helper implemented in [Report Artifact Approval Proof Marker Store-Backed Gate Helper Report](../concepts/REPORT_ARTIFACT_APPROVAL_PROOF_MARKER_STORE_BACKED_GATE_HELPER_REPORT.md). This follows the accepted pure in-memory helper in [Report Artifact Approval Proof Marker Gate Helper Review](../concepts/REPORT_ARTIFACT_APPROVAL_PROOF_MARKER_GATE_HELPER_REVIEW.md).

This plan defined the smallest implementation slice for reading durable local approval proof-marker projection records and applying the accepted in-memory gate before an explicit report artifact write path. The implemented helper remains validation-only. Automatic report artifact writing, executor default enforcement, automatic proof-marker projection persistence, CLI behavior, schemas, examples, provider writes, hosted behavior, reasoning lineage, and release posture changes remain unimplemented.

## 1. Executive Summary

Workflow OS now has two reviewed foundations for approval proof-marker artifact gating:

- durable local approval proof-marker projection persistence;
- a pure in-memory report artifact approval proof-marker gate helper.

The next runtime-composition question is how an explicit artifact-capable path should supply projection records from `LocalApprovalProofMarkerAuditProjectionStore` to the accepted in-memory gate.

The first implementation must remain explicit and opt-in. It must not make report artifact writing automatic, change default executor behavior, persist projection records automatically, add CLI behavior, add schemas, update examples, call providers, execute writes, implement hosted behavior, or change release posture.

## 2. Goals

- Compose the accepted local projection store with the accepted in-memory artifact proof-marker gate.
- Keep projection store reads explicit and caller-owned.
- Validate `WorkReportArtifactRecord` approval citations against persisted projection records before artifact write when requested.
- Preserve current workflow execution and report generation semantics.
- Preserve existing `WorkReportArtifactStore` base behavior.
- Avoid copying approval-presentation payloads, approval reasons, command output, provider payloads, source contents, report text, or secret-like values.
- Return stable, bounded, non-leaking errors.
- Keep gate result exposure count-only.
- Prepare for later executor artifact composition without changing executor defaults now.

## 3. Non-Goals

This phase must not implement:

- automatic report artifact writing;
- automatic proof-marker projection persistence;
- executor default proof-marker gate enforcement;
- default proof-marker citation behavior for all reports;
- workflow-declared proof-marker artifact requirements;
- workflow schema changes;
- CLI rendering or commands;
- examples;
- public approval cards;
- dedicated proof-marker audit sink events;
- new workflow event kinds;
- mutation of approval decision events;
- EvidenceReference creation;
- approval evidence attachment;
- provider writes;
- runtime side-effect execution;
- hosted or distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 4. Current Foundation

Implemented and reviewed:

- `ApprovalDecisionProofMarker` on opt-in proof-enforced approval decisions.
- Bounded inspect/projection output for approval proof markers.
- WorkReport proof-marker citation derivation helper.
- Terminal report opt-in proof-marker citation integration.
- Executor report input propagation for proof-marker citation policy.
- Pure proof-marker audit projection helper.
- `LocalApprovalProofMarkerAuditProjectionStore`.
- `ApprovalProofMarkerAuditProjectionStoreRecord`.
- Pure in-memory `validate_work_report_artifact_approval_proof_marker_gate(...)`.
- Explicit local report artifact store.
- Explicit artifact write paths with SideEffect integrity and approval-linkage gates.

Still not implemented:

- store-backed report artifact proof-marker gate integration;
- automatic durable proof-marker projection persistence from executor paths;
- automatic artifact writing;
- workflow-declared proof-marker artifact requirements;
- CLI report artifact inspection;
- schemas or examples for proof-marker artifact policy.

## 5. Source Of Truth Boundary

Approval decision workflow events remain the source of truth for approval decisions.

Approval proof-marker projection records remain bounded durable projections. The store-backed gate must treat them as proof posture records only when their immutable run identity and stable references match the report artifact.

The store-backed integration must not infer proof-marker posture from:

- report prose;
- approval reason text;
- actor names;
- policy strings;
- approval handoff text;
- presentation payloads;
- command output;
- provider payloads;
- source/spec contents;
- chat transcripts.

## 6. Recommended First Integration Boundary

The first implementation should be a helper adjacent to existing artifact gate helpers, not a change to `WorkReportArtifactStore` itself.

Recommended flow:

1. Caller constructs a validated `WorkReportArtifactRecord`.
2. Caller supplies an explicit `LocalApprovalProofMarkerAuditProjectionStore`.
3. Caller supplies an explicit gate policy.
4. Helper reads projection records for the artifact's immutable run identity from the store.
5. Helper calls `validate_work_report_artifact_approval_proof_marker_gate(...)`.
6. Caller writes the artifact only if the gate succeeds and any other requested artifact gates also succeed.

The helper should not:

- derive or persist projection records;
- generate WorkReports;
- write report artifacts by itself unless explicitly composed in a separate governed-write helper;
- append workflow events;
- emit audit or observability events;
- mutate workflow state;
- call providers;
- execute side effects.

## 7. Candidate API Shape

The implementation may choose more idiomatic names, but the first helper should remain small.

Candidate input:

```rust
pub struct WorkReportArtifactApprovalProofMarkerStoreGateInput<'a, S>
where
    S: ApprovalProofMarkerAuditProjectionStore,
{
    pub artifact: &'a WorkReportArtifactRecord,
    pub projection_store: &'a S,
    pub policy: WorkReportArtifactApprovalProofMarkerGatePolicy,
}
```

Candidate helper:

```rust
pub fn validate_work_report_artifact_approval_proof_marker_gate_from_store<S>(
    input: WorkReportArtifactApprovalProofMarkerStoreGateInput<'_, S>,
) -> Result<WorkReportArtifactApprovalProofMarkerGateResult, WorkflowOsError>
where
    S: ApprovalProofMarkerAuditProjectionStore,
```

If the existing local store trait is not yet abstracted, the first implementation may accept `&LocalApprovalProofMarkerAuditProjectionStore` directly and document that trait generalization is deferred.

## 8. Store Read Policy

The helper should read only records for the artifact's immutable run identity.

Preferred read behavior:

- list projection records by workflow ID, workflow version, schema version, spec hash, and run ID if the store supports that query;
- otherwise list all local projection records and filter by artifact identity in memory, while keeping the helper explicit and local-only.

Store failures must fail closed with stable non-leaking errors. Corrupt or identity-mismatched records must not be silently ignored when they are returned for the target run.

The helper must not read arbitrary directories, infer store roots, create stores, or discover hidden global state.

## 9. Gate Policy Semantics

Use the accepted policy:

- strict default: every approval citation must resolve to one projection with a present proof marker;
- explicit marker-free mode: every approval citation must resolve to one projection, but marker-free projections are allowed and counted;
- explicit permissive mode: missing projections may be counted only when `require_all_approval_citations_projected` is false.

The store-backed helper should not introduce new policy semantics in the first implementation. It should only supply store-backed projection records to the reviewed in-memory gate.

## 10. Error Handling

Recommended stable error families:

- `work_report_artifact.approval_proof_marker_gate.store_read_failed`
- `work_report_artifact.approval_proof_marker_gate.store_identity_mismatch`
- existing in-memory helper codes for missing, ambiguous, mismatched, marker-free-disallowed, corrupt, and invalid-artifact paths.

Errors must not include:

- approval IDs;
- event IDs;
- projection IDs;
- presentation IDs;
- content hashes;
- report IDs;
- run IDs;
- local paths;
- report text;
- approval handoff text;
- approval reasons;
- command output;
- provider payloads;
- source/spec contents;
- credentials, tokens, authorization headers, private keys, or secret-like values.

## 11. Relationship To Artifact Writes

The first store-backed integration helper should remain validation-only.

A later composition helper may call it before writing artifacts, alongside existing SideEffect integrity and approval-linkage gates. That later helper must define failure behavior explicitly and preserve workflow pass/fail semantics.

This phase must not change existing executor methods:

- `LocalExecutor::execute(...)`;
- `LocalExecutor::execute_with_report(...)`;
- `execute_with_report_artifact_and_side_effect_gates(...)`;
- existing provider-write helper paths.

## 12. Relationship To Existing Gates

The store-backed proof-marker gate should compose with existing artifact gates without changing their meaning:

- `SideEffect` referential integrity still proves cited side-effect records match the artifact.
- approval-side-effect linkage still proves side effects are linked to the expected approval posture.
- high-assurance disclosure gates still prove required disclosure posture.
- proof-marker gate proves approval citations have durable proof-marker projection posture.

Composition order should be explicit and deterministic. A reasonable future order is:

1. artifact validation and immutable run identity validation;
2. SideEffect referential integrity gate;
3. approval-side-effect linkage gate;
4. high-assurance disclosure gate;
5. approval proof-marker projection gate;
6. artifact write.

The order may be adjusted by implementation if tests prove a clearer fail-closed boundary.

## 13. Privacy And Redaction

The helper must preserve the accepted privacy posture:

- no approval-presentation payload storage;
- no approval handoff text;
- no approval reasons;
- no command output;
- no provider payloads;
- no report text in errors or debug output;
- no local store paths in errors or debug output;
- no raw source/spec contents;
- no credentials, authorization headers, private keys, token-like values, or secret-like metadata.

`Debug` output should expose booleans, counts, policy posture, and error codes only.

## 14. Test Plan

Future implementation tests should cover:

- matching persisted projection records satisfy strict gate policy;
- missing persisted projection fails closed under strict policy;
- missing persisted projection is counted under permissive policy;
- marker-free persisted projection fails under strict policy;
- marker-free persisted projection passes under explicit marker-free policy;
- corrupt store record maps to non-leaking gate error;
- store read failure maps to non-leaking gate error;
- identity-mismatched store record fails closed without leaking values;
- duplicate matching projection records fail closed as ambiguous;
- no approval citations produce bounded zero-count result;
- helper does not write artifacts;
- helper does not append workflow events;
- helper does not mutate workflow state;
- helper does not persist projection records;
- helper does not call providers or execute side effects;
- debug output does not leak IDs, paths, hashes, report text, or payloads;
- existing artifact, projection, WorkReport, executor, SideEffect, approval, and docs tests still pass.

## 15. Proposed Implementation Sequence

Recommended small future phases:

1. Implement a store-backed validation-only helper that reads caller-supplied local projection store records and delegates to the accepted in-memory gate.
2. Add focused tests for store read success, missing record, corrupt record, identity mismatch, marker-free policy, and non-mutation.
3. Review the helper.
4. Plan explicit artifact-write composition with SideEffect integrity, approval-linkage, high-assurance disclosure, and proof-marker gates.
5. Only after review, implement the explicit artifact-write composition path.

Do not add automatic executor defaults or workflow-declared proof-marker artifact requirements in the first store-backed integration.

## 16. Deferred Work

Deferred:

- automatic projection persistence from executor paths;
- automatic report artifact writing;
- default executor proof-marker artifact gates;
- workflow-declared proof-marker artifact requirements;
- schema exposure;
- CLI report artifact inspection;
- public approval cards;
- examples;
- provider writes;
- runtime side-effect execution;
- hosted or distributed runtime;
- reasoning lineage;
- release posture changes.

## 17. Final Recommendation

Proceed next to a store-backed report artifact approval proof-marker gate helper implementation, validation-only and explicit.

The implementation should read caller-supplied local projection store records, delegate to the accepted in-memory gate, expose bounded count-only results, and fail closed without changing artifact store behavior or executor defaults.
