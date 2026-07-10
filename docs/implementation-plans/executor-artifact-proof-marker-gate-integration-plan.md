# Executor Artifact Proof-Marker Gate Integration Plan

Status: implemented in [Executor Artifact Proof-Marker Gate Integration Report](../concepts/EXECUTOR_ARTIFACT_PROOF_MARKER_GATE_INTEGRATION_REPORT.md). This follows the accepted helper-level composition in [Report Artifact Proof-Marker Write Composition Helper Review](../concepts/REPORT_ARTIFACT_PROOF_MARKER_WRITE_COMPOSITION_HELPER_REVIEW.md).

This plan does not implement Rust code, executor default behavior, automatic artifact writing, automatic approval proof-marker projection persistence, CLI behavior, schemas, examples, provider writes, hosted behavior, reasoning lineage, writes, or release posture changes.

## 1. Executive Summary

Workflow OS now has an explicit helper that can validate report artifact identity, `SideEffect` referential integrity, approval-side-effect linkage, high-assurance disclosure posture, and store-backed approval proof-marker projection posture before writing a `WorkReportArtifactRecord`.

The next runtime-composition question is whether the existing explicit artifact-capable executor path should be able to use that helper when the caller supplies a proof-marker projection store and policy.

The answer should remain narrow: add an opt-in executor artifact path extension that accepts explicit proof-marker gate inputs and delegates to the accepted helper. Default executor behavior must not change.

## 2. Goals

- Let the explicit artifact-capable executor path require approval proof-marker projection validation before artifact write when requested.
- Preserve existing `LocalExecutor::execute(...)`, `LocalExecutor::execute_with_report(...)`, and default artifact behavior.
- Keep proof-marker projection stores caller-supplied.
- Keep proof-marker policy caller-supplied or explicitly disabled.
- Reuse `write_work_report_artifact_with_governance_gates(...)`.
- Preserve workflow-declared high-assurance artifact policy derivation and strictness composition.
- Preserve workflow pass/fail semantics when report or artifact generation fails after a run exists.
- Keep all errors stable, bounded, and non-leaking.
- Avoid hidden persistence, hidden runtime config, hidden store roots, and hidden approvals.

## 3. Non-Goals

Do not implement:

- code changes in this planning phase;
- automatic report artifact writing;
- default executor proof-marker enforcement;
- automatic approval proof-marker projection persistence;
- workflow-declared proof-marker artifact requirements;
- CLI rendering or commands;
- schemas;
- examples;
- provider writes;
- runtime side-effect execution;
- hosted or distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 4. Current Baseline

Implemented foundations:

- explicit artifact-capable executor path: `execute_with_report_artifact_and_side_effect_gates(...)`;
- explicit artifact request/result types;
- workflow-declared high-assurance artifact gate derivation;
- caller/workflow high-assurance policy strictness composition;
- generic artifact write integration helper;
- helper-level proof-marker artifact write composition: `write_work_report_artifact_with_governance_gates(...)`;
- local proof-marker projection store: `LocalApprovalProofMarkerAuditProjectionStore`;
- store-backed proof-marker gate helper.

Not implemented:

- executor artifact path inputs for proof-marker projection store/policy;
- automatic proof-marker projection persistence;
- default executor proof-marker gate enforcement.

## 5. Problem Statement

`WorkReport` artifacts can cite approval proof-marker posture. The accepted helper can prove those citations against persisted projection records before artifact write. The explicit artifact-capable executor path still writes artifacts through the generic integration helper that does not require proof-marker projection validation.

This is safe because proof-marker validation is not claimed by the current executor artifact path. But it leaves a runtime-composition gap: callers that have proof-marker projection records cannot yet ask the executor artifact path to enforce that gate before writing the artifact.

The solution should compose existing primitives rather than introduce new proof concepts.

## 6. Recommended First Implementation Boundary

Add an optional proof-marker gate input to the explicit artifact-capable executor path only.

Do not change:

- `LocalExecutor::execute(...)`;
- `LocalExecutor::execute_with_report(...)`;
- `execute_with_report_and_side_effect_discovery(...)`;
- CLI commands;
- workflow schemas;
- examples;
- default validation behavior.

The integration should run only when a caller explicitly invokes the artifact-capable executor API and supplies proof-marker gate inputs.

## 7. Candidate API Shape

Prefer a small optional field on `LocalExecutionReportArtifactInputs`, or an adjacent wrapper, such as:

```rust
pub proof_marker_gate: Option<LocalExecutionReportArtifactProofMarkerGateInputs<'a>>
```

Candidate input:

```rust
pub struct LocalExecutionReportArtifactProofMarkerGateInputs<'a> {
    pub projection_store: &'a LocalApprovalProofMarkerAuditProjectionStore,
    pub policy: WorkReportArtifactApprovalProofMarkerGatePolicy,
}
```

If lifetimes on the existing request shape make this awkward, prefer an executor method variant that accepts a request wrapper with borrowed stores rather than hiding a store root in runtime config.

Do not infer projection stores from `state_dir`.

## 8. Gate Composition

When proof-marker gate inputs are absent, preserve current behavior exactly.

When proof-marker gate inputs are present:

1. execute the workflow through the existing explicit artifact-capable path;
2. generate the terminal report as today;
3. build the report artifact record as today;
4. derive and compose high-assurance artifact policy as today;
5. call `write_work_report_artifact_with_governance_gates(...)`;
6. return existing result shape, extended only if necessary to expose bounded proof-marker posture.

The proof-marker gate should execute before artifact write and after existing artifact identity, side-effect, approval-linkage, and high-assurance gates.

## 9. Result Shape

Prefer preserving `LocalExecutionWithReportArtifactResult`.

If proof-marker posture must be exposed, add an optional bounded field that contains only:

- approval citation count;
- projected count;
- marker-present count;
- marker-free count;
- missing projection count;
- duplicate approval citation count.

Do not expose approval IDs, projection IDs, presentation IDs, content hashes, local paths, approval reasons, report text, or payloads.

## 10. Failure Semantics

Proof-marker gate failure must remain an artifact error, not a workflow execution error, once a run/report exists.

Rules:

- execution failure before a run exists still returns `Err`;
- report generation failure after a run exists remains inside the result;
- artifact gate failure after report generation remains inside the result;
- workflow run status does not change because artifact proof-marker validation failed;
- no workflow events are appended;
- no audit or observability events are emitted;
- no projection records are created;
- no side-effect records are created or repaired;
- no partial artifact is written when strict proof-marker policy fails.

## 11. Store And Persistence Boundary

All proof-marker projection stores must be caller-supplied.

The executor must not:

- infer projection store roots;
- persist projection records;
- repair projection records;
- read hidden stores;
- create default projection stores;
- write artifacts unless the caller already selected the explicit artifact-capable path.

Automatic proof-marker projection persistence remains a separate concern.

## 12. Privacy And Redaction

The integration must not inspect or copy:

- approval presentation text;
- approval reasons;
- report section text beyond existing validated report construction;
- raw provider payloads;
- command output;
- CI logs;
- spec or source contents;
- parser payloads;
- environment variable values;
- credentials, authorization headers, private keys, tokens, or secret-like values.

Errors must use stable codes and avoid raw identifiers, paths, snippets, and payloads.

## 13. Test Plan

Future implementation tests should cover:

- absent proof-marker gate inputs preserve current artifact path behavior;
- present proof-marker gate inputs call the governance-gated helper before write;
- successful persisted projection allows artifact write;
- missing required projection returns artifact error and writes no artifact;
- marker-free projection fails under marker-required policy and writes no artifact;
- proof-marker gate failure preserves run/report status and event history;
- workflow-derived high-assurance policy strictness still composes correctly;
- side-effect integrity and approval-linkage still run when side-effect citations exist;
- result/debug output does not leak approval IDs, projection IDs, presentation IDs, content hashes, paths, report text, or approval reasons;
- default `execute(...)` and `execute_with_report(...)` remain unchanged;
- existing executor, WorkReport, SideEffect, approval proof-marker, and docs tests continue to pass.

## 14. Proposed Implementation Sequence

1. Add a small optional proof-marker gate input surface for the explicit artifact-capable executor path.
2. Preserve existing behavior when the field is absent.
3. Route the artifact write through `write_work_report_artifact_with_governance_gates(...)` when the field is present.
4. Preserve or minimally extend the existing result shape with bounded proof-marker posture.
5. Add focused executor tests.
6. Run full validation.
7. Review before considering workflow-declared proof-marker artifact requirements.

## 15. Deferred Work

- Workflow-declared proof-marker artifact requirements.
- Automatic proof-marker projection persistence.
- Default executor proof-marker enforcement.
- CLI artifact rendering or export.
- Provider writes.
- Runtime side-effect execution.
- Hosted or distributed runtime behavior.
- Reasoning lineage.
- Examples and schemas.

## 16. Final Recommendation

Proceed next with **executor artifact proof-marker gate integration implementation**, but keep it opt-in and explicit.

The first implementation should add only caller-supplied proof-marker projection store/policy inputs to the explicit artifact-capable executor path. It must not make artifact writing automatic, infer stores, persist projections, change default executor behavior, add CLI behavior, add schemas, call providers, or enable writes.
