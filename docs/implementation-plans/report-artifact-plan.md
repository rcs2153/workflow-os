# Report Artifact Plan

Status: Core/local artifact store implemented. SideEffect citation referential integrity validation is implemented as an explicit helper in [Report Artifact SideEffect Referential Integrity Report](../concepts/REPORT_ARTIFACT_SIDE_EFFECT_REFERENTIAL_INTEGRITY_REPORT.md), following [Report Artifact SideEffect Referential Integrity Plan](report-artifact-side-effect-referential-integrity-plan.md), and accepted in [Report Artifact SideEffect Referential Integrity Review](../concepts/REPORT_ARTIFACT_SIDE_EFFECT_REFERENTIAL_INTEGRITY_REVIEW.md). Explicit high-assurance approval disclosure artifact gating is implemented in [Report Artifact High-Assurance Approval Disclosure Gate Plan](report-artifact-high-assurance-disclosure-gate-plan.md). Workflow-declared high-assurance artifact requirements are planned in [Workflow-Declared High-Assurance Artifact Requirement Plan](workflow-declared-high-assurance-artifact-requirement-plan.md), the first internal model/policy-mapping slice is implemented in [Workflow-Declared High-Assurance Artifact Requirement Model Report](../concepts/WORKFLOW_DECLARED_HIGH_ASSURANCE_ARTIFACT_REQUIREMENT_MODEL_REPORT.md), the first schema/parser/SDK/validation field is implemented in [Workflow-Declared High-Assurance Artifact Requirement Schema Report](../concepts/WORKFLOW_DECLARED_HIGH_ASSURANCE_ARTIFACT_REQUIREMENT_SCHEMA_REPORT.md), the pure runtime derivation helper is implemented in [Workflow-Declared High-Assurance Artifact Requirement Runtime Derivation Report](../concepts/WORKFLOW_DECLARED_HIGH_ASSURANCE_ARTIFACT_REQUIREMENT_RUNTIME_DERIVATION_REPORT.md), and explicit executor artifact-path integration is implemented in [Workflow-Declared High-Assurance Artifact Requirement Executor Integration Plan](workflow-declared-high-assurance-artifact-requirement-executor-integration-plan.md). Automatic artifact writing from default executor paths, CLI rendering, examples, signing, notarization, DLP, access control, and release posture changes are not implemented.


## 1. Executive Summary

`WorkReportContract`, `WorkReport`, terminal local report generation, in-memory runtime result exposure, and explicit executor-integrated report-bearing execution are implemented and reviewed. Workflow OS can now produce a validated in-memory report for explicit local execution paths without changing existing executor semantics.

The first artifact-store implementation adds a validated `WorkReportArtifactRecord`, a separate `WorkReportArtifactStore` trait, and local backend read/write/list support for explicit report artifact persistence. This makes a generated `WorkReport` durable only when artifact writing is called explicitly.

This plan and implementation do not add executor wiring, automatic artifact writing, CLI rendering, schemas, examples, automatic runtime report generation, approval/cancellation report-bearing APIs, reasoning lineage, side-effect modeling, writes, signing, notarization, DLP, access control, or release posture changes.

## 2. Goals

- Define the narrow first durable artifact boundary for generated `WorkReport` values.
- Preserve existing workflow execution semantics.
- Preserve event-sourced workflow state as the source of truth for runtime state.
- Keep report artifacts separate from workflow events and snapshots.
- Store validated `WorkReport` values only.
- Avoid raw provider, spec, command, parser, log, or credential payloads.
- Preserve redaction-safe serialization and deserialization behavior.
- Make artifact write failures explicit and non-leaking.
- Prepare for future CLI inspection/export without implementing CLI behavior now.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- automatic report generation for every run;
- changing `LocalExecutor::execute(...)`, `decide_approval(...)`, or `cancel_run(...)`;
- approval/cancellation report-bearing APIs;
- appending report-created workflow events;
- mutating `WorkflowRun` or `WorkflowRunSnapshot`;
- changing terminal status semantics;
- filesystem report writing in this planning phase;
- local backend schema changes in this planning phase;
- CLI rendering or commands;
- JSON export commands;
- example updates;
- workflow spec schema changes;
- workflow-declared report contracts;
- runtime config for reports;
- reasoning lineage;
- side-effect boundary modeling;
- write behavior;
- approval evidence attachment;
- production reporting;
- report signing or notarization;
- SIEM or OpenTelemetry integration;
- DLP or access-control systems;
- release posture changes.

## 4. Current Foundation

Implemented foundation:

- `WorkReportContract` core model.
- `WorkReport` core model.
- Redaction-safe report validation and serialization.
- In-memory terminal local report generation helper.
- In-memory runtime result exposure helper.
- `LocalExecutor::execute_with_report(...)` as an explicit additive local execution path.

Current implementation boundary:

- generated reports are persisted only through explicit `WorkReportArtifactStore` calls;
- local report artifacts are stored under `work_reports/<run-id>/<report-id>.json`;
- no executor path writes report artifacts automatically;
- no CLI command reads or renders reports;
- no workflow spec requests report artifacts;
- no runtime automatically generates reports for every terminal run.

## 5. Artifact Boundary

A v1 report artifact should be a durable representation of one validated `WorkReport` for one workflow run.

It should not be workflow state. It should not be an event. It should not be an audit record. It should not be an evidence payload store.

Recommended boundary:

- artifact identity is derived from or bound to `WorkReportId`;
- artifact metadata includes `WorkflowRunId`, `WorkflowId`, workflow version, schema version, spec hash, terminal status, generation timestamp, sensitivity, and redaction metadata;
- artifact content is the validated serialized `WorkReport`;
- artifact storage is append-once or create-if-absent for a given report ID;
- duplicate report IDs fail closed unless the stored artifact is byte-for-byte identical and the final implementation explicitly accepts idempotent replay.

## 6. Store Placement Options

| Option | Assessment |
| --- | --- |
| Add `WorkReportArtifactStore` as a separate state-backend capability | Preferred first implementation. It keeps report artifacts adjacent to local state without making them workflow events. |
| Store reports as workflow events | Reject for v1. Current terminal-state rules reject post-terminal mutation, and reports are derived handoff artifacts, not runtime state transitions. |
| Add report field to `WorkflowRunSnapshot` | Reject. A report is not projection state and should not mutate rehydrated run shape. |
| Write arbitrary files from executor path | Reject. It couples runtime execution to filesystem output and bypasses backend abstraction. |
| CLI-only export from in-memory result | Defer. CLI rendering/export should follow artifact and compatibility planning. |

## 7. Recommended First Implementation Boundary

The first implementation added a minimal artifact store abstraction and local backend support, without wiring it automatically into executor execution.

Implemented first boundary:

1. Added a `WorkReportArtifactStore` trait.
2. Added a `WorkReportArtifactRecord` model that wraps a validated `WorkReport` plus artifact metadata.
3. Added local backend write/read/list support under the existing local state root.
4. Kept artifact writing explicit and separate from report generation and executor execution.
5. Added tests for validation, duplicate handling, non-leakage, corruption handling, and runtime non-mutation.
6. Confirmed `execute_with_report(...)` does not write artifacts.

This keeps artifact persistence reviewable before it becomes part of any executor or CLI surface.

## 8. Artifact Identity And Path Policy

Artifact identity should be stable and typed.

Implemented model:

- `WorkReportId` is reused as artifact identity for this first boundary.
- `WorkReportArtifactMetadata`.
- `WorkReportArtifactRecord`.
- `WorkReportArtifactStore`.

Local path policy is deterministic and non-secret:

- `<state-root>/work_reports/<run-id>/<report-id>.json`

Path rules:

- path components must use validated typed IDs only;
- no workflow names, project paths, operator notes, titles, summaries, actor text, or untrusted strings in artifact paths;
- no path traversal;
- no raw provider IDs unless already validated as safe typed references;
- local file permissions should follow existing local state behavior, with stricter posture considered before storing regulated or secret reports.

## 9. Serialization And Compatibility

The artifact should serialize the existing `WorkReport` model shape through its validated serde path.

Rules:

- only validated `WorkReport` values may be stored;
- deserialization must validate before returning a report;
- invalid serialized artifacts fail closed;
- deserialization errors must not leak report text, raw citation values, secret-like metadata, or local paths;
- field names should remain stable and sensible for future schema exposure;
- no public schema should be added in the first artifact implementation;
- compatibility posture should be documented as local artifact preview until schema/export planning is accepted.

## 10. Artifact Write Semantics

Artifact writes should be explicit and separate from workflow execution.

Recommended v1 semantics:

- write only after a valid `WorkReport` exists;
- do not generate a report during artifact write;
- do not append workflow events;
- do not mutate snapshots;
- do not alter workflow terminal status;
- create parent directories only within the local backend's state root;
- write atomically using the repository's existing local-state style where feasible;
- fail duplicate report IDs deterministically;
- return structured non-leaking errors.

If artifact write fails, the workflow run and in-memory report remain unchanged.

## 11. Artifact Read Semantics

Reads should be explicit.

Recommended v1 behavior:

- read by `WorkflowRunId` plus `WorkReportId`, or list report artifact metadata by `WorkflowRunId`;
- validate deserialized records before returning them;
- fail closed on corrupt JSON, missing required fields, invalid report shape, or mismatched run/report identity;
- do not read artifacts during normal run rehydration;
- do not merge artifacts into `WorkflowRun`;
- do not render reports for CLI in this phase.

## 12. Privacy And Redaction

Report artifacts are sensitive by default.

Rules:

- store references and bounded summaries only;
- never store raw provider payloads;
- never store raw CI logs;
- never store raw Jira bodies or comments;
- never store raw GitHub file contents;
- never store raw command output;
- never store raw spec contents;
- never store raw parser payloads;
- never store environment variable values;
- never store credentials, authorization headers, private keys, or token-like values;
- preserve `WorkReport` redaction metadata validation;
- Debug and error output must be redaction-safe;
- local artifact paths must not include sensitive report text or project paths beyond the existing state-root choice.

This artifact plan is not a DLP or access-control system. Those remain separate future work.

## 13. Relationship To Existing State Backend

The event log remains authoritative for workflow state.

Report artifacts should be a separate durable projection-like store, not part of event replay. Existing `StateBackend` currently combines event, snapshot, idempotency, lock, approval, project state, policy audit, and adapter telemetry stores. The first artifact implementation keeps `WorkReportArtifactStore` separate from the aggregate `StateBackend`.

A future artifact store may either:

- extend the aggregate `StateBackend` after review; or
- remain a separate trait implemented by `LocalStateBackend`.

The first implementation chose the smallest interface that does not force report artifact behavior into normal runtime execution paths.

## 14. Relationship To CLI And Examples

CLI rendering and examples remain deferred.

A future CLI phase may inspect or export report artifacts only after:

- artifact format is reviewed;
- read semantics are stable;
- redaction behavior is tested;
- operator-facing output rules are defined;
- compatibility posture is documented.

Examples should not be updated until report artifacts and CLI behavior can be explained without implying production reporting, hosted persistence, or write support.

## 15. Failure Handling

Artifact failures must be explicit and non-leaking.

Future error codes should be stable, for example:

- `work_report_artifact.write.failed`;
- `work_report_artifact.write.duplicate`;
- `work_report_artifact.read.not_found`;
- `work_report_artifact.read.corrupt`;
- `work_report_artifact.identity.mismatch`;
- `work_report_artifact.path.invalid`.

Error messages must not include report section text, citation values, redaction metadata values, raw payloads, tokens, local source snippets, or untrusted path fragments.

## 16. Test Plan

Future implementation tests should cover:

- valid `WorkReport` artifact can be written explicitly;
- valid artifact can be read and validated;
- read artifact equals stored report model;
- artifact metadata binds report ID and run ID;
- invalid report artifact fails deserialization/validation;
- corrupt JSON fails clearly;
- duplicate artifact write is rejected or idempotent only by explicit rule;
- path traversal is impossible through typed IDs;
- artifact write does not append workflow events;
- artifact write does not mutate `WorkflowRun` or `WorkflowRunSnapshot`;
- artifact write does not change terminal status;
- artifact write is not called by `execute(...)`;
- artifact write is not called by `execute_with_report(...)`;
- raw provider/spec/command/parser/log payload markers are not copied;
- secret-like report text and redaction metadata do not leak through Debug, serialization errors, or read/write errors;
- existing report, executor, state backend, EvidenceReference, Diagnostic, validation, adapter telemetry, and runtime tests still pass;
- docs check passes.

Deferred tests:

- local backend health behavior for corrupt report artifacts if artifact health checks are added;
- executor opt-in artifact writing behavior;
- CLI inspection/export behavior.

## 17. Proposed Implementation Sequence

Recommended future phases:

1. Implement report artifact core/local store model only. Completed.
2. Review artifact store implementation.
3. Plan explicit executor or helper opt-in artifact writing.
4. Implement explicit artifact writing only after review.
5. Review artifact write integration.
6. Plan CLI inspect/export behavior.
7. Plan examples only after CLI and artifact posture are reviewed.

Do not begin with automatic artifact generation from every terminal run.

## 18. Open Questions

- Should a later lifecycle require a separate `WorkReportArtifactId`, or is reused `WorkReportId` sufficient?
- Should artifact storage eventually extend `StateBackend`, or remain a separate trait?
- Should duplicate artifact writes remain rejected, or become idempotent when bytes match?
- Should artifact records gain an integrity hash before signing/notarization exists?
- Should local backend health checks inspect report artifacts in the first implementation?
- Should artifacts be stored for failed report generation attempts, or only successful reports?
- Should artifact writes ever be part of workflow success criteria?
- Should artifact paths be considered confidential even when IDs are safe?
- What is the smallest CLI inspection surface that would be useful after artifacts exist?

## 19. Final Recommendation

The report artifact store implementation is complete and reviewed. The validation-only report artifact SideEffect referential integrity helper described in [Report Artifact SideEffect Referential Integrity Plan](report-artifact-side-effect-referential-integrity-plan.md) is implemented and accepted. Approval-side-effect linkage planning is documented in [Approval SideEffect Linkage Plan](approval-side-effect-linkage-plan.md), and the validation-only helper is implemented in [SideEffect Approval Linkage Report](../concepts/SIDE_EFFECT_APPROVAL_LINKAGE_REPORT.md). Explicit high-assurance approval disclosure artifact gating is implemented in [Report Artifact High-Assurance Approval Disclosure Gate Plan](report-artifact-high-assurance-disclosure-gate-plan.md). Proceed next with a bounded review of the explicit high-assurance disclosure artifact gate before any CLI artifact inspection, automatic discovery, EvidenceReference side-effect attachment, runtime side-effect execution, write-capable adapters, or automatic artifact writes from existing executor paths.

The implementation added a minimal explicit `WorkReportArtifactStore` and local backend read/write/list support for validated `WorkReport` artifacts, without executor wiring, automatic generation, CLI rendering, schemas, examples, reasoning lineage, writes, approval/cancellation report-bearing APIs, signing, notarization, DLP, access control, or release posture changes. The next integrity helper must remain explicit, reference-only, and non-mutating.
