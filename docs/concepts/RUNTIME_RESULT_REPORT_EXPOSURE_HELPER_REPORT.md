# Runtime Result Report Exposure Helper Report

Report date: 2026-06-05

## 1. Executive Summary

The runtime result report exposure phase is implemented as an in-memory helper only.

The phase adds a `TerminalLocalWorkReportResult` wrapper and `expose_terminal_local_work_report_result(...)` helper. The helper pairs an already-terminal `WorkflowRun` with a generated, validated `WorkReport` without changing existing executor return types, mutating runtime state, appending events, writing files, persisting reports, or emitting CLI output.

Automatic runtime report generation, executor-integrated automatic result exposure, report artifacts, persistence, CLI rendering, workflow schema changes, examples, reasoning lineage, side-effect modeling, writes, approval evidence attachment, runtime config, and release posture changes remain unimplemented.

## 2. Scope Completed

- Added `TerminalLocalWorkReportResult`.
- Added `expose_terminal_local_work_report_result(...)`.
- Reused `generate_terminal_local_work_report(...)`.
- Preserved existing `LocalExecutor::execute(...) -> WorkflowRun` behavior.
- Preserved existing approval and cancellation executor result behavior.
- Returned a report-bearing in-memory result from explicit inputs.
- Kept completed, failed, and canceled status support through the existing terminal report helper.
- Rejected non-terminal input through the existing `work_report_generation.status.not_terminal` boundary.
- Added focused tests for result exposure, ownership, non-mutation, non-terminal rejection, and Debug redaction safety.

## 3. Scope Explicitly Not Completed

- Automatic runtime report generation is not implemented.
- Executor-integrated automatic report result exposure is not implemented.
- Report artifacts are not implemented.
- Persistence is not implemented.
- Filesystem report writing is not implemented.
- CLI rendering is not implemented.
- CLI commands are not implemented.
- Examples are not updated.
- Workflow spec schema changes are not implemented.
- Workflow-declared report contracts are not implemented.
- Runtime config for reports is not implemented.
- Reasoning lineage is not implemented.
- Side-effect boundary modeling is not implemented.
- Write behavior is not implemented.
- Approval evidence attachment is not implemented.
- Release posture is unchanged.

## 4. Helper API Summary

The new result type is:

- `TerminalLocalWorkReportResult`.

It owns:

- `WorkflowRun`;
- `WorkReport`.

The new helper is:

- `expose_terminal_local_work_report_result(input) -> Result<TerminalLocalWorkReportResult, WorkflowOsError>`.

The helper clones the supplied terminal run, generates a report through the existing terminal report helper, and returns both values in memory. It exposes read-only accessors and `into_parts()` for owned extraction.

## 5. Runtime Semantics Summary

The helper preserves workflow semantics:

- it does not execute workflows;
- it does not mutate a `WorkflowRun`;
- it does not mutate a `WorkflowRunSnapshot`;
- it does not mutate event history;
- it does not append post-terminal events;
- it does not emit audit or observability events;
- it does not touch a `StateBackend`;
- it does not alter terminal status semantics;
- it does not change existing executor method signatures.

Report exposure failure returns a structured error from the explicit helper path and does not change the borrowed run.

## 6. Citation And Report Construction Summary

The helper does not construct report sections or citations directly. It delegates to `generate_terminal_local_work_report(...)`, which uses existing `WorkReport`, `WorkReportSection`, and `WorkReportCitation` constructors.

This preserves the existing policy:

- cite stable IDs only;
- do not recreate `EvidenceReference` values;
- do not fabricate IDs;
- use not-available section text for unavailable optional references;
- do not copy raw payloads.

## 7. Redaction And Privacy Summary

The result wrapper implements redaction-safe `Debug` output. It reports only the run status, run event count, and whether a report exists.

The helper does not serialize the result wrapper and does not create a schema surface. The underlying `WorkReport` model continues to enforce bounded text, redaction metadata validation, and secret-like value rejection.

The phase does not introduce storage for:

- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira bodies/comments;
- raw GitHub file contents;
- raw spec contents;
- raw parser payloads;
- environment variable values;
- credentials, authorization headers, private keys, or token-like values.

## 8. Test Coverage Summary

Added tests cover:

- in-memory result exposure for completed terminal runs;
- owned `WorkflowRun` and `WorkReport` extraction;
- non-terminal rejection without run mutation;
- redaction-safe result Debug output;
- continued terminal helper behavior for completed, failed, and canceled runs;
- existing no-filesystem-artifact and no-CLI-output behavior;
- existing WorkReport, WorkReportContract, EvidenceReference, Diagnostic, validation, adapter telemetry, and runtime tests through the workspace test suite.

## 9. Commands Run And Results

- `cargo fmt --all --check` - passed after applying formatter line wrapping.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

## 10. Remaining Known Limitations

- Existing executor methods still return `WorkflowRun` only.
- The result wrapper is not returned by `LocalExecutor`.
- Reports are not automatically generated for every terminal run.
- Reports are not persisted, written as artifacts, exported, rendered, or exposed through CLI.
- Missing citation targets still use section text in the helper path.
- Runtime result exposure still accepts explicit inputs; workflow-declared report contracts remain deferred.
- Approval evidence attachment remains unimplemented.

## 11. Recommended Next Phase

Recommended next phase: runtime result exposure helper review.

Review should happen before adding executor-integrated report-bearing methods, automatic report generation, report artifacts, persistence, CLI rendering, workflow schema changes, examples, reasoning lineage, side-effect modeling, writes, or approval evidence attachment.
