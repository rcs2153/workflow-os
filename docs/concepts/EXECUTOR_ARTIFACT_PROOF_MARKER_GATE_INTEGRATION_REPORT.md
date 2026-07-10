# Executor Artifact Proof-Marker Gate Integration Report

## 1. Executive Summary

The executor artifact proof-marker gate integration is implemented as an explicit opt-in API.

The existing artifact-capable executor path still behaves as before. A new helper, `execute_with_report_artifact_and_proof_marker_gates(...)`, lets callers provide a local approval proof-marker projection store and policy. When supplied, the executor generates the terminal report, builds the artifact, preserves existing artifact gates, validates approval proof-marker projection posture, and writes the artifact only if all gates pass.

Default executor behavior does not change.

## 2. Scope Completed

- Added `LocalExecutionReportArtifactProofMarkerGateInputs`.
- Added `execute_with_report_artifact_and_proof_marker_gates(...)`.
- Preserved `execute_with_report_artifact_and_side_effect_gates(...)` behavior by delegating through a shared internal path with no proof-marker gate.
- Reused `write_work_report_artifact_with_governance_gates(...)` for opt-in proof-marker validation before artifact write.
- Preserved workflow-derived high-assurance artifact policy composition.
- Preserved provider-candidate artifact validation when the proof-marker path is used.
- Preserved the existing `LocalExecutionWithReportArtifactResult` shape.
- Added focused executor tests.
- Updated roadmap and implementation-plan status.

## 3. Scope Explicitly Not Completed

This phase does not implement:

- executor default proof-marker enforcement;
- automatic report artifact writing;
- automatic approval proof-marker projection persistence;
- workflow-declared proof-marker artifact requirements;
- CLI rendering or commands;
- schemas;
- examples;
- provider writes;
- runtime side-effect execution;
- hosted or distributed runtime behavior;
- reasoning lineage;
- write-capable adapters;
- release posture changes.

## 4. API Summary

The new API is:

```rust
execute_with_report_artifact_and_proof_marker_gates(...)
```

It accepts:

- the existing local executor;
- an explicit `WorkReportArtifactStore`;
- an explicit `SideEffectRecordStore`;
- `LocalExecutionReportArtifactProofMarkerGateInputs`;
- the existing `LocalExecutionWithReportArtifactRequest`.

The proof-marker gate inputs contain:

- a caller-supplied `LocalApprovalProofMarkerAuditProjectionStore`;
- an explicit `WorkReportArtifactApprovalProofMarkerGatePolicy`.

The API does not infer projection stores from runtime state, create stores, persist projection records, discover hidden state, append events, call providers, execute side effects, or expose CLI output.

## 5. Gate Composition Summary

When proof-marker gate inputs are supplied, artifact persistence is gated by:

1. workflow execution through the existing local executor path;
2. terminal report generation;
3. report artifact construction;
4. provider-candidate validation when provider integration inputs are supplied;
5. side-effect citation integrity;
6. approval-side-effect linkage;
7. high-assurance approval disclosure policy;
8. store-backed approval proof-marker projection policy;
9. artifact write only after every requested gate passes.

When proof-marker gate inputs are absent, existing artifact behavior is preserved.

## 6. Workflow Semantics Summary

The integration preserves workflow semantics:

- execution failure before a run exists still returns `Err`;
- report-generation failure after a run exists remains in the result;
- artifact-gate failure after report generation remains in the result;
- proof-marker gate failure does not change workflow run status;
- no workflow events are appended;
- no audit or observability events are emitted;
- no projection records are created;
- no side-effect records are created or repaired;
- no partial artifact is written when strict proof-marker policy fails.

## 7. Privacy And Redaction Summary

The new input and result surfaces are bounded and redaction-safe.

Debug output redacts the projection store and does not expose approval IDs, projection IDs, presentation IDs, content hashes, local paths, report text, approval reasons, command output, provider payloads, source contents, spec contents, credentials, tokens, authorization headers, private keys, or secret-like values.

Errors remain stable and non-leaking.

## 8. Test Coverage Summary

Focused tests cover:

- successful artifact write when a persisted approval proof-marker projection exists;
- missing projection failure before artifact write;
- preservation of existing artifact behavior when proof-marker inputs are absent;
- report and run preservation on proof-marker gate failure;
- artifact-store emptiness on proof-marker gate failure;
- provider-candidate artifact tests continuing to pass through the shared executor path;
- existing artifact high-assurance and side-effect gate behavior.

## 9. Commands Run And Results

- `cargo fmt --all` - passed.
- `cargo test -p workflow-core --test local_executor execute_with_report_artifact_proof_marker -- --nocapture` - passed.
- `cargo test -p workflow-core --test local_executor execute_with_report_artifact -- --nocapture` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed after refactoring the shared executor artifact helper to keep the implementation lint-clean.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.

## 10. Remaining Known Limitations

- The default executor does not enforce proof-marker projection gates.
- The regular artifact-capable executor path does not require proof-marker validation unless the new opt-in helper is used.
- Approval proof-marker projection persistence remains explicit and caller-supplied.
- Workflow-declared proof-marker artifact requirements are not implemented.
- CLI artifact rendering/export is not implemented.
- No provider writes, hosted behavior, reasoning lineage, schemas, examples, write-capable adapters, or release posture changes are included.

## 11. Recommended Next Phase

Recommended next phase: **executor artifact proof-marker gate integration review**.

This phase connects a security-sensitive proof boundary into an executor-adjacent artifact path. A maintainer review should verify unchanged defaults, gate order, provider-candidate preservation, fail-closed behavior, non-leaking errors, and test quality before any workflow-declared proof-marker artifact requirements are considered.
