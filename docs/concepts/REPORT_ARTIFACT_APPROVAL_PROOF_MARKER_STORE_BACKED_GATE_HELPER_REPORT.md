# Report Artifact Approval Proof Marker Store-Backed Gate Helper Report

## 1. Executive Summary

This phase implemented the first explicit store-backed report artifact approval proof-marker gate helper.

The helper reads bounded approval proof-marker audit projection records from a caller-supplied `LocalApprovalProofMarkerAuditProjectionStore` and delegates validation semantics to the accepted in-memory `WorkReportArtifactApprovalProofMarkerGate` helper. It remains validation-only and local. It does not write report artifacts, persist projection records automatically, change executor defaults, append events, mutate workflow state, expose CLI behavior, add schemas, update examples, call providers, execute writes, implement hosted behavior, or change release posture.

## 2. Scope Completed

- Added `WorkReportArtifactApprovalProofMarkerStoreGateInput`.
- Added `validate_work_report_artifact_approval_proof_marker_gate_from_store(...)`.
- Reused `LocalApprovalProofMarkerAuditProjectionStore::list()` as the explicit durable projection read boundary.
- Delegated all approval citation and marker policy semantics to `validate_work_report_artifact_approval_proof_marker_gate(...)`.
- Mapped store read, corrupt-record, and identity mismatch failures to stable, non-leaking artifact gate errors.
- Exported the helper API from `workflow-core`.
- Added focused tests for matching persisted projections, missing projections, permissive missing projections, marker-free policy, corrupt store data, and redaction-safe `Debug`.
- Updated planning and roadmap documentation honestly.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- automatic report artifact writing;
- executor default proof-marker gate enforcement;
- automatic proof-marker projection persistence;
- workflow-declared proof-marker artifact requirements;
- CLI rendering or commands;
- schemas;
- examples;
- public approval cards;
- dedicated proof-marker audit sink records;
- new workflow event kinds;
- mutation of approval decision events;
- EvidenceReference creation;
- approval evidence attachment;
- provider writes;
- runtime side-effect execution;
- hosted or distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 4. Helper API Summary

The new input type is:

```rust
pub struct WorkReportArtifactApprovalProofMarkerStoreGateInput<'a> {
    pub artifact: &'a WorkReportArtifactRecord,
    pub projection_store: &'a LocalApprovalProofMarkerAuditProjectionStore,
    pub policy: WorkReportArtifactApprovalProofMarkerGatePolicy,
}
```

The new helper is:

```rust
pub fn validate_work_report_artifact_approval_proof_marker_gate_from_store(
    input: WorkReportArtifactApprovalProofMarkerStoreGateInput<'_>,
) -> Result<WorkReportArtifactApprovalProofMarkerGateResult, WorkflowOsError>
```

The helper validates the artifact, lists records from the explicit local store, maps store failures into artifact-gate error codes, and calls the accepted in-memory gate helper with the loaded records.

## 5. Validation Boundary Summary

The helper:

- requires a validated `WorkReportArtifactRecord`;
- reads only from the caller-supplied `LocalApprovalProofMarkerAuditProjectionStore`;
- does not infer store roots or hidden global state;
- delegates strict, marker-free, and permissive policy behavior to the existing gate;
- fails closed when strict policy requires a missing projection;
- counts missing projections only when policy explicitly allows missing projections;
- rejects marker-free projections unless policy explicitly allows them;
- maps corrupt store records to `work_report_artifact.approval_proof_marker_gate.record_corrupt`;
- maps store identity mismatch to `work_report_artifact.approval_proof_marker_gate.identity_mismatch`;
- maps other store read failures to `work_report_artifact.approval_proof_marker_gate.store_read_failed`.

## 6. Privacy And Redaction Summary

The helper does not copy:

- approval-presentation payloads;
- approval handoff text;
- approval reasons;
- command output;
- provider payloads;
- report text;
- local store paths;
- source/spec contents;
- credentials, authorization headers, private keys, token-like values, or secret-like metadata.

`Debug` output redacts the artifact and projection store and exposes only policy posture. Result `Debug` output remains count-only through the existing accepted gate result.

Errors use stable codes and bounded messages. Tests assert that approval IDs, run IDs, projection IDs, store paths, authorization markers, bearer markers, and provider payload markers do not appear in store-backed gate errors or `Debug` output.

## 7. Test Coverage Summary

Added focused tests covering:

- matching persisted projection records satisfy strict policy;
- missing persisted projection fails strict policy without leaking approval/run/store details;
- missing persisted projection is counted in permissive mode;
- marker-free persisted projection fails strict policy;
- marker-free persisted projection passes explicit marker-free policy;
- corrupt store data maps to a non-leaking artifact gate error;
- store-backed input and result `Debug` output are bounded and non-leaking.

Existing `WorkReport`, `WorkReportContract`, `EvidenceReference`, `Diagnostic`, validation, adapter telemetry, artifact, SideEffect, and runtime tests remain covered by workspace validation.

## 8. Commands Run And Results

- `cargo fmt --all` passed during implementation.
- `cargo test -p workflow-core --test work_report` passed with 210 tests.
- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `npm run check:docs` passed.
- `git diff --check` passed.
- `npm run dogfood:benchmark -- phase-close run-1783641619016777000-2 --phase implementation` passed.

Dogfood governance summary:

- workflow: `dg/implement`
- run: `run-1783641619016777000-2`
- approval: `approval/run-1783641619016777000-2/implementation-approved`
- approval presentation: `presentation/3df6f31ef18ed47f`
- approval outcome: granted
- event summary: 39 events, one approval, zero retries, zero escalations
- approval-presentation enforcement: `proof_enforced`
- out-of-kernel work: repo edits, shell validation commands, and future git/PR actions are performed by Codex/human executor outside the kernel and disclosed here.

## 9. Remaining Known Limitations

- The helper accepts `LocalApprovalProofMarkerAuditProjectionStore` directly; a generic projection store trait remains deferred.
- The helper reads the explicit local store and delegates filtering/identity checks to the existing in-memory gate; narrower store queries can be added later if the store grows query APIs.
- The helper does not write artifacts or compose with artifact writes automatically.
- Executor paths do not enforce proof-marker artifact gates by default.
- Workflow-declared proof-marker artifact requirements are not implemented.
- CLI inspection, schema exposure, examples, and hosted behavior remain unimplemented.

## 10. Recommended Next Phase

Recommended next phase: store-backed proof-marker gate helper review.

This helper is security- and audit-adjacent because it composes durable local projection records with artifact write-readiness checks. It should receive a focused maintainer review before any artifact-write composition phase uses it.
