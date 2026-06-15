# Report Artifact Store Review

Review date: 2026-06-07

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation adds a narrow explicit artifact boundary for generated `WorkReport` values. `WorkReportArtifactRecord` binds one validated report to derived artifact metadata, `WorkReportArtifactStore` exposes explicit write/read/list operations, and the local backend stores artifacts under a separate `work_reports/` tree. The implementation does not make report artifacts workflow events, does not write artifacts from executor paths, does not add CLI rendering or export behavior, and does not change workflow execution semantics.

No blockers were found before the next planning phase.

## 2. Scope Verification

The phase stayed within the approved explicit local artifact-store scope.

Verified in scope:

- `WorkReportArtifactMetadata` derives report/run identity from a contained `WorkReport`.
- `WorkReportArtifactRecord` wraps one validated `WorkReport` plus matching metadata.
- `WorkReportArtifactStore` exposes explicit `write`, `read`, and `list` operations.
- `LocalStateBackend` implements the artifact store using local JSON files.
- Duplicate artifact writes fail closed.
- Corrupt artifact reads fail closed with stable non-leaking errors.
- Artifact writes do not mutate workflow events or snapshots.
- Documentation and the end-of-phase report were added.

No accidental implementation was found for:

- executor artifact auto-writing;
- automatic runtime report generation;
- runtime result exposure beyond the already reviewed in-memory result path;
- CLI behavior;
- report export commands;
- example updates;
- workflow spec schema changes;
- approval/cancellation report-bearing APIs;
- event append from artifact writes;
- run or snapshot mutation from artifact writes;
- terminal status changes;
- reasoning lineage implementation;
- side-effect boundary modeling;
- write-capable adapters;
- release posture changes.

## 3. Artifact Model Assessment

The artifact model is intentionally small and appropriate for the first durable handoff boundary.

`WorkReportArtifactMetadata` captures the report ID, workflow identity, schema version, spec hash, run ID, terminal status, generated timestamp, sensitivity, and redaction metadata derived from the contained report. `WorkReportArtifactRecord::new(...)` validates the report, derives metadata, and validates that the artifact metadata matches the contained report.

The implementation reuses `WorkReportId` as the artifact identity rather than introducing a separate artifact ID. That is acceptable for this first phase because the store path is keyed by run ID plus report ID and duplicate writes are rejected.

The model keeps fields private, exposes read-only accessors, and uses redaction-safe `Debug` output. Deserialization validates the metadata/report relationship and fails closed on identity mismatch.

## 4. Artifact Store API Assessment

The store API is narrow and explicit.

`WorkReportArtifactStore` exposes:

- `write_work_report_artifact(...)`;
- `read_work_report_artifact(...)`;
- `list_work_report_artifacts(...)`.

The trait is deliberately separate from the aggregate `StateBackend`. That fits the current architecture: report artifacts are durable handoff records, not workflow events, run snapshots, idempotency records, locks, approvals, policy audit records, or adapter telemetry. Keeping the trait separate avoids implying that artifacts participate in normal event replay or workflow rehydration.

The API does not create artifacts implicitly and does not make artifact reads part of executor behavior.

## 5. Local Backend and Path Assessment

The local backend stores report artifacts under:

```text
work_reports/<encoded-run-id>/<encoded-report-id>.json
```

The implementation uses typed `WorkflowRunId` and `WorkReportId` values, then encodes keys before constructing filesystem paths. That avoids raw caller-controlled path fragments. Writes use create-new atomic JSON behavior, so duplicate artifact writes fail instead of overwriting existing artifacts.

This is an appropriate v0 local backend posture. It is not a production storage contract, signing mechanism, export format, or schema guarantee.

## 6. Validation and Deserialization Assessment

Validation is defensive at the artifact boundary.

Verified:

- `WorkReportArtifactRecord::new(...)` validates the contained report.
- Artifact metadata is derived from the report rather than accepted from callers during normal construction.
- `validate_against_report(...)` rejects metadata/report identity mismatch.
- artifact metadata redaction metadata is validated.
- artifact deserialization validates both metadata and the contained report.
- invalid serialized artifacts fail closed.
- duplicate writes return `work_report_artifact.write.duplicate`.
- corrupt artifact reads return `work_report_artifact.read.corrupt` without leaking corrupt payload text.

Mapping all malformed local artifact read failures to `work_report_artifact.read.corrupt` is conservative and safe for this phase.

## 7. Runtime and Event Boundary Assessment

The implementation preserves the runtime boundary.

Verified:

- report artifacts are not workflow events;
- artifact writes do not append workflow events;
- artifact writes do not mutate run snapshots;
- artifact writes do not change terminal status;
- artifacts are not read during normal run rehydration;
- `LocalExecutor::execute(...)` does not write artifacts;
- `LocalExecutor::execute_with_report(...)` does not write artifacts;
- no CLI or export path was added.

This keeps report artifact persistence an explicit caller action.

## 8. Privacy and Redaction Assessment

The privacy posture is sound for a local explicit artifact store.

Verified:

- artifact `Debug` redacts the contained `WorkReport`;
- artifact metadata `Debug` redacts IDs, spec hash, and redaction metadata values;
- artifact identity mismatch errors use stable codes and do not include mismatched raw IDs;
- corrupt artifact read errors do not include corrupt payload values;
- the artifact wrapper does not introduce raw provider payloads, raw CI logs, raw Jira/GitHub bodies, raw command output, raw spec contents, parser payloads, environment values, credentials, authorization headers, private keys, or token-like values.

Artifacts contain serialized `WorkReport` values by design. That means the artifact store is sensitive even when reports contain only bounded summaries and stable references. This is documented and acceptable for local explicit persistence.

## 9. Compatibility and API Surface Assessment

The new public API surface is additive.

Verified:

- `WorkReportArtifactMetadata` and `WorkReportArtifactRecord` are exported from `workflow-core`.
- `WorkReportArtifactStore` is exported from `workflow-core`.
- existing executor APIs remain unchanged.
- no workflow spec schema changes were introduced.
- no CLI JSON/display contract was introduced.
- no examples were updated.
- no production backend contract was claimed.

Before artifact export, schema exposure, or production backends, the project should decide whether artifact JSON shape is an internal local format or a versioned external compatibility surface.

## 10. Test Quality Assessment

The tests are focused and meaningful.

Covered:

- artifact record binds report and run identity;
- artifact record serializes and deserializes;
- artifact metadata/report identity mismatch is rejected;
- artifact `Debug` does not leak report text or redaction values;
- local backend writes, reads, and lists artifacts;
- duplicate artifact writes are rejected;
- corrupt artifact reads fail without leaking payload text;
- artifact writes do not mutate runtime events or snapshots;
- `execute_with_report(...)` does not automatically write artifacts;
- existing report, contract, evidence, diagnostic, validation, adapter telemetry, executor, and state tests remain part of the workspace validation suite.

Missing or shallow areas, all non-blocking for this phase:

- multiple-artifact list ordering is not directly asserted;
- artifact JSON compatibility is not versioned as an external schema;
- no health-check scan currently reports corrupt work report artifacts;
- no explicit test distinguishes missing artifact reads from corrupt reads beyond the existing `Ok(None)` path implied by the store API;
- no production backend contract tests exist, which is expected because production backends remain out of scope.

## 11. Documentation Review

Docs now state:

- explicit local report artifact storage is implemented;
- `WorkReportArtifactStore` is separate from aggregate `StateBackend`;
- report artifacts are not workflow events or run snapshots;
- `LocalExecutor::execute(...)` and `LocalExecutor::execute_with_report(...)` do not write artifacts automatically;
- report artifacts do not append events, mutate snapshots, change terminal status, or add CLI rendering/export behavior;
- persistence remains local and explicit only;
- schemas, examples, reasoning lineage, side effects, writes, production stores, signing, notarization, DLP, access control, and release posture changes remain unimplemented.

The docs do not overclaim automatic reporting, production durability, or CLI/report export support.

## 12. Blockers

No blockers found.

## 13. Non-Blocking Follow-Ups

- Decide whether artifact JSON should remain an internal local format or become a versioned external schema before export/CLI exposure.
- Decide whether `WorkReportArtifactStore` should remain separate from the aggregate `StateBackend` before production backend planning.
- Consider a health-check extension that can report corrupt local work report artifacts without making artifacts part of workflow rehydration.
- Decide whether duplicate artifact writes should always fail or whether a future idempotent write mode is needed for replay-safe callers.
- Consider whether a separate `WorkReportArtifactId` is needed before supporting multiple report artifacts per report or export lifecycle metadata.
- Add multiple-artifact list ordering tests if callers begin depending on list order.
- Consider integrity metadata before signing, notarization, export, or cross-machine artifact transfer.

## 14. Recommended Next Phase

Recommended next phase: explicit executor/helper artifact-writing planning.

The artifact store is ready as an explicit local persistence boundary, but no executor or report helper path writes artifacts yet. The next planning phase should decide whether and how report-aware execution or report generation helpers may optionally call `WorkReportArtifactStore` without making artifact generation automatic, changing workflow semantics, appending events, adding CLI behavior, or exposing schemas.

## 15. Validation

Passed:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
