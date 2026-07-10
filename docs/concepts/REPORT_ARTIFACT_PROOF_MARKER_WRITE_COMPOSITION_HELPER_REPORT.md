# Report Artifact Proof-Marker Write Composition Helper Report

## 1. Executive Summary

The report artifact proof-marker write composition helper is implemented as an explicit, local, helper-level path.

The new helper composes existing report artifact validation, terminal run identity validation, `SideEffect` referential integrity, approval-side-effect linkage, high-assurance disclosure validation, and store-backed approval proof-marker projection validation before writing a `WorkReportArtifactRecord`.

This is an opt-in composition helper. It does not change executor defaults, make artifact writing automatic, persist proof-marker projections, expose CLI behavior, add schemas, update examples, call providers, enable writes, implement hosted behavior, implement reasoning lineage, or change release posture.

## 2. Scope Completed

- Added `WorkReportArtifactProofMarkerGovernedWriteInput`.
- Added `WorkReportArtifactProofMarkerGovernedWriteResult`.
- Added `write_work_report_artifact_with_governance_gates(...)`.
- Exported the helper and bounded types from `workflow-core`.
- Composed the accepted store-backed proof-marker gate with the existing artifact-write gates.
- Added focused tests for success and fail-closed pre-write behavior.
- Updated roadmap and implementation-plan status.

## 3. Scope Explicitly Not Completed

- No executor default behavior changed.
- No automatic report artifact writing was added.
- No automatic approval proof-marker projection persistence was added.
- No workflow-declared proof-marker artifact requirements were added.
- No CLI rendering or commands were added.
- No schemas or examples were added.
- No provider writes were added.
- No runtime side-effect execution was added.
- No hosted or distributed runtime behavior was added.
- No reasoning lineage was implemented.
- No release posture changed.

## 4. Helper API Summary

The new helper is:

```rust
write_work_report_artifact_with_governance_gates(...)
```

It accepts:

- a caller-supplied `WorkReportArtifactStore`;
- a caller-supplied `SideEffectRecordStore`;
- a `WorkReportArtifactProofMarkerGovernedWriteInput`;
- an existing `WorkReportArtifactGovernedWriteInput`;
- a caller-supplied `LocalApprovalProofMarkerAuditProjectionStore`;
- an explicit `WorkReportArtifactApprovalProofMarkerGatePolicy`.

It returns a bounded `WorkReportArtifactProofMarkerGovernedWriteResult` with:

- side-effect integrity counts;
- approval-linkage posture when side-effect citations are present;
- high-assurance disclosure posture when configured;
- approval proof-marker gate counts.

The result does not expose report text, run IDs, side-effect IDs, approval IDs, projection IDs, presentation IDs, content hashes, local paths, payloads, or provider data.

## 5. Gate Order

The helper uses deterministic pre-write gate order:

1. Validate the `WorkReportArtifactRecord`.
2. Validate artifact identity against the supplied terminal `WorkflowRun`.
3. Validate `SideEffect` referential integrity.
4. Validate approval-side-effect linkage when side-effect citations exist.
5. Validate high-assurance approval disclosure when configured.
6. Validate approval proof-marker projection posture from the caller-supplied store.
7. Write the artifact only after all requested gates pass.

This keeps proof-marker citation claims from being treated as durable proof unless the caller supplies a projection store that validates them.

## 6. Failure Semantics

Gate failure returns a structured `WorkflowOsError` and does not write the artifact.

The helper does not mutate the workflow run, append workflow events, emit audit records, create projection records, repair citations, call providers, execute side effects, or change workflow pass/fail semantics.

Strict proof-marker policy fails closed before artifact write when:

- cited approvals do not have persisted projection records;
- marker-free projections are present but marker-free approvals are not allowed;
- projection identity does not match the artifact's immutable run identity;
- projection store records are corrupt or unreadable.

## 7. Privacy And Redaction Summary

The helper reuses existing redaction-safe model constructors and validation gates.

Errors and debug output are bounded and do not expose:

- report text;
- approval handoff text;
- approval reasons;
- run IDs;
- approval IDs;
- presentation IDs;
- projection IDs;
- local paths;
- command output;
- provider payloads;
- source or spec contents;
- credentials, tokens, authorization headers, private keys, or secret-like values.

Proof-marker gate results remain count-only.

## 8. Test Coverage Summary

Added focused tests for:

- successful artifact write when a matching approval proof-marker projection is persisted;
- strict missing-projection failure before artifact write;
- strict marker-free projection failure before artifact write;
- run preservation during helper execution;
- artifact-store emptiness on gate failures;
- bounded proof-marker gate counts in the success result.

Existing work-report, artifact, side-effect, approval-linkage, high-assurance disclosure, and proof-marker tests continue to cover the lower-level primitives.

## 9. Commands Run And Results

- `cargo test -p workflow-core --test work_report governance_gated_artifact_write` - passed.
- `cargo fmt --all` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.

## 10. Remaining Known Limitations

- Existing executor default paths do not require proof-marker projection validation before artifact writes.
- The existing explicit artifact-capable executor path does not automatically use this helper.
- Approval proof-marker projection persistence remains explicit and caller-supplied.
- Workflow-declared proof-marker artifact requirements are not implemented.
- CLI artifact inspection/export is not implemented.
- No provider writes, hosted behavior, reasoning lineage, schemas, examples, or release posture changes are included.

## 11. Recommended Next Phase

Recommended next phase: **Report artifact proof-marker write composition helper review**.

The helper is security-sensitive because it decides whether approval proof-marker citations are durable enough to allow artifact persistence. A maintainer review should verify gate order, fail-closed behavior, unchanged executor defaults, privacy posture, and test quality before any executor-adjacent integration is considered.
