# Report Artifact Store Report

Report date: 2026-06-07

## 1. Executive Summary

This phase implemented the first report artifact boundary for Workflow OS.

The implementation adds a validated `WorkReportArtifactRecord`, redaction-safe artifact metadata, and an explicit `WorkReportArtifactStore` with local backend write/read/list support. Artifacts are stored only when callers explicitly use the artifact store. Existing executor paths do not write artifacts automatically.

## 2. Scope Completed

- Added `WorkReportArtifactMetadata`.
- Added `WorkReportArtifactRecord`.
- Added `WorkReportArtifactStore`.
- Added `LocalStateBackend` support for explicit report artifact write/read/list operations.
- Stored artifacts under the local state root using typed IDs.
- Reused validated `WorkReport` serialization/deserialization.
- Rejected duplicate artifact writes deterministically.
- Failed closed on corrupt artifact reads.
- Added focused tests.
- Updated planning, roadmap, concept, and runtime docs.

## 3. Scope Explicitly Not Completed

- No automatic artifact writing from `LocalExecutor::execute_with_report(...)`.
- No automatic runtime report generation for every run.
- No approval/cancellation report-bearing APIs.
- No CLI rendering.
- No CLI export.
- No example updates.
- No workflow spec schema changes.
- No workflow-declared report contracts.
- No runtime config.
- No report signing or notarization.
- No reasoning lineage.
- No side-effect boundary modeling.
- No write behavior.
- No approval evidence attachment.
- No DLP or access-control system.
- No release posture change.

## 4. Model Types Added

- `WorkReportArtifactMetadata`: redaction-safe artifact metadata derived from the contained report.
- `WorkReportArtifactRecord`: validated wrapper around one `WorkReport` and matching artifact metadata.

The implementation reuses `WorkReportId` as the artifact identity for this first boundary. No separate `WorkReportArtifactId` was added.

## 5. Store API Summary

`WorkReportArtifactStore` exposes:

- `write_work_report_artifact(...)`;
- `read_work_report_artifact(...)`;
- `list_work_report_artifacts(...)`.

The local backend stores artifacts under:

```text
work_reports/<run-id>/<report-id>.json
```

Path components are derived from validated typed IDs and encoded by the local backend.

## 6. Validation Boundary Summary

Artifact construction validates the contained `WorkReport` and derives artifact metadata from it.

Artifact deserialization validates:

- contained report shape;
- redaction metadata;
- report ID match;
- workflow identity match;
- schema version match;
- spec hash match;
- run ID match;
- terminal report status match;
- generation timestamp match;
- sensitivity match.

Invalid serialized artifacts fail closed with stable non-leaking errors.

## 7. Runtime Boundary Summary

Report artifacts are not workflow state.

The artifact store does not:

- append workflow events;
- mutate `WorkflowRun`;
- mutate `WorkflowRunSnapshot`;
- alter terminal status;
- run during normal rehydration;
- write through executor paths automatically.

The event log remains authoritative for workflow state.

## 8. Redaction And Privacy Summary

Artifacts store validated `WorkReport` values, which remain reference-first and bounded.

The implementation does not store raw provider payloads, raw CI logs, raw Jira bodies/comments, raw GitHub file contents, raw command output, raw spec contents, raw parser payloads, environment variable values, credentials, authorization headers, private keys, or token-like values.

Artifact Debug output redacts the contained report and redaction metadata details. Duplicate and corrupt-read errors do not include report IDs, run IDs, report text, secret-like payloads, or artifact paths.

## 9. Test Coverage Summary

Added or updated tests cover:

- artifact metadata binds report ID and run ID;
- artifact serde round trip;
- metadata/report identity mismatch rejection;
- artifact Debug non-leakage;
- local backend write/read/list;
- duplicate write rejection;
- corrupt artifact read failure without payload leakage;
- artifact write does not mutate events or snapshots;
- `execute_with_report(...)` does not write report artifacts automatically;
- existing report, executor, and state behavior remains intact.

## 10. Commands Run And Results

- `cargo test -p workflow-core --test work_report` - passed.
- `cargo test -p workflow-core --test local_executor execute_with_report` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

## 11. Remaining Known Limitations

- `WorkReportArtifactStore` is separate from the aggregate `StateBackend`.
- Local backend health checks do not inspect report artifact corruption yet.
- Duplicate artifact writes are rejected rather than idempotent.
- No integrity hash is stored beyond the validated report content.
- Artifact writes are not wired into executor APIs.
- No CLI inspect/export behavior exists.
- No public schema exists for report artifacts.

## 12. Recommended Next Phase

Recommended next phase: report artifact store implementation review.

Review should happen before adding executor artifact-writing options, CLI rendering/export, schemas, examples, automatic artifact generation, approval/cancellation report-bearing methods, reasoning lineage, side-effect modeling, writes, DLP, access control, or release posture changes.
