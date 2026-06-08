# Executor-Integrated Report Result Report

Report date: 2026-06-07

## 1. Executive Summary

This phase implemented an explicit executor-integrated report-bearing execution path for local runs.

`LocalExecutor::execute_with_report(...)` wraps the existing `execute(...)` method and returns an in-memory `LocalExecutionWithReportResult`. Existing executor methods and workflow semantics remain unchanged.

The implementation is additive and opt-in. It does not make report generation automatic for every run.

## 2. Scope Completed

- Added `LocalExecutionReportInputs`.
- Added `LocalExecutionWithReportRequest`.
- Added `LocalExecutionWithReportResult`.
- Added `LocalExecutor::execute_with_report(...)`.
- Reused the existing terminal local report generation and runtime result exposure helpers.
- Returned completed and failed local runs with generated reports when report generation succeeds.
- Returned non-terminal runs with no report and a structured report-generation error.
- Returned terminal runs with no report and a structured report-generation error when report construction fails after execution returns a run.
- Added focused local executor tests.
- Updated roadmap and report planning docs.

## 3. Scope Explicitly Not Completed

- Existing `execute(...)`, `decide_approval(...)`, and `cancel_run(...)` return types are unchanged.
- Approval-resume report-bearing methods are not implemented.
- Cancellation report-bearing methods are not implemented.
- Automatic runtime report generation for every run is not implemented.
- Report artifacts are not implemented.
- Persistence is not implemented.
- CLI rendering is not implemented.
- Examples are not updated.
- Workflow schema changes are not implemented.
- Runtime report configuration is not implemented.
- Reasoning lineage is not implemented.
- Side-effect boundary modeling is not implemented.
- Writes remain unsupported.
- Approval evidence attachment is not implemented.
- Release posture is unchanged.

## 4. API Summary

`LocalExecutionWithReportRequest` contains:

- an existing `LocalExecutionRequest`;
- explicit `LocalExecutionReportInputs`.

`LocalExecutionReportInputs` contains explicit report identity, contract identity/version, generated timestamp/actor, sensitivity, redaction metadata, optional correlation ID, supplied stable references, and bounded disclosure/note text.

`LocalExecutionWithReportResult` contains:

- `run() -> &WorkflowRun`;
- `work_report() -> Option<&WorkReport>`;
- `report_generation_error() -> Option<&WorkflowOsError>`;
- `into_parts() -> (WorkflowRun, Option<WorkReport>, Option<WorkflowOsError>)`.

`LocalExecutor::execute_with_report(...)` calls existing `execute(...)` and never duplicates executor state-transition logic.

## 5. Terminal Status Behavior

Completed and failed local executions can return a generated report.

Waiting-for-approval and other non-terminal outcomes return the run, no report, and a structured report-generation error with code `work_report_generation.status.not_terminal`.

The implementation does not change `WorkflowRunStatus::is_terminal()`. Runtime `Escalated` remains non-terminal, and `Blocked` remains report model vocabulary only.

## 6. Error Policy

If workflow execution fails before producing a run, `execute_with_report(...)` returns the execution error unchanged.

If workflow execution produces a run but report generation fails, `execute_with_report(...)` returns:

- the run;
- `work_report: None`;
- `report_generation_error: Some(error)`.

This preserves workflow pass/fail semantics and keeps report generation failure visible without retroactively failing the workflow.

## 7. Citation And Section Summary

The executor-integrated path accepts supplied stable references and passes them through existing report generation:

- `EvidenceReferenceId`;
- workflow event IDs;
- audit event IDs;
- adapter telemetry stable references;
- validation reference IDs;
- policy event IDs;
- approval reference IDs where stable.

It does not discover references from stores, recreate `EvidenceReference` values, fabricate IDs, or copy raw payloads. Missing optional references remain explicit not-available section text for this phase.

Generated reports continue to include all required v1 report sections through existing `WorkReport` construction.

## 8. Workflow Semantics Summary

The implementation does not:

- mutate `WorkflowRun`;
- mutate `WorkflowRunSnapshot`;
- mutate event history;
- append post-terminal events;
- emit report-generation audit events;
- emit report-generation observability events;
- write report state to `StateBackend`;
- create filesystem artifacts;
- emit CLI output.

The report-bearing result is derived in memory.

## 9. Redaction And Privacy Summary

The implementation reuses existing `WorkReport`, `WorkReportSection`, `WorkReportCitation`, and terminal report helper validation.

Report generation errors remain structured. The new result `Debug` output includes run status, event count, report presence, and report error code only. It does not include report text, report IDs, workflow IDs, correlation IDs, paths, raw payloads, or secret-like input values.

The implementation does not copy raw provider payloads, raw command output, raw CI logs, raw Jira bodies/comments, raw GitHub file contents, raw spec contents, raw parser payloads, environment variable values, credentials, authorization headers, private keys, or token-like values.

## 10. Test Coverage Summary

Focused tests cover:

- existing `execute(...)` still returns `WorkflowRun`;
- completed execution returns run plus report;
- failed execution returns run plus report;
- waiting-for-approval execution returns run plus non-terminal report error and no report;
- report generation failure from secret-like input preserves run/events and returns no report;
- all required v1 sections are present;
- supplied evidence and adapter telemetry references are cited;
- absent optional references become explicit section text;
- no report artifact file is created by the executor-integrated path;
- result `Debug` output is redaction-safe.

## 11. Commands Run And Results

- `cargo test -p workflow-core --test local_executor execute_with_report -- --nocapture` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

## 12. Remaining Known Limitations

- Approval-resume and cancellation report-bearing APIs are not implemented.
- Report inputs still use explicit contract identity/version rather than a full declared contract instance.
- Missing citations remain section text rather than explicit missing-citation records.
- Reports are not persisted, rendered, exported, or exposed through CLI.
- Executor-integrated report-bearing execution is local and in-memory only.

## 13. Recommended Next Phase

Recommended next phase: executor-integrated report result implementation review.

The review should verify that the additive executor path preserves workflow semantics, keeps report generation failure separate from execution failure, avoids event/state mutation, and does not broaden scope into artifacts, persistence, CLI, schemas, examples, reasoning lineage, side effects, writes, approval evidence attachment, or release posture changes.
