# Executor-Integrated Report Result Plan

Status: Implemented and reviewed. `LocalExecutor::execute_with_report(...)` is implemented as an additive, in-memory report-bearing execution path. Report artifact planning and the explicit local artifact store are documented in [Report Artifact Plan](report-artifact-plan.md). Approval-resume and cancellation report-bearing methods, automatic report generation, automatic artifact writing from executor paths, CLI rendering, schemas, examples, reasoning lineage, side-effect modeling, writes, approval evidence attachment, and release posture changes are not implemented.

## 1. Executive Summary

`WorkReportContract`, `WorkReport`, terminal local report generation, and in-memory runtime result exposure helper foundations are implemented and reviewed. `LocalExecutor::execute_with_report(...)` now provides an explicit report-bearing execution path that runs the existing executor and returns a `WorkflowRun` with an optional generated `WorkReport`.

The implemented API is additive and opt-in. It does not change existing executor method signatures and does not make report generation automatic for every run.

This phase does not implement approval/cancellation report-bearing methods, automatic report generation, persistence, report artifacts, CLI rendering, examples, workflow schema changes, runtime config, reasoning lineage, side-effect modeling, writes, approval evidence attachment, or release posture changes.

## 2. Goals

- Add an explicit executor path that can return both `WorkflowRun` and `WorkReport`. Implemented.
- Preserve existing executor methods and return types.
- Preserve workflow pass/fail semantics.
- Keep report generation local, deterministic, and opt-in.
- Use existing terminal report generation and runtime result exposure helpers.
- Avoid post-terminal workflow events.
- Avoid persistence, artifacts, CLI output, schema exposure, and runtime config.
- Keep report generation errors structured and non-leaking.
- Keep report citations based on supplied stable references, not fabricated IDs.

## 3. Non-Goals

This plan and implementation do not authorize:

- changing `LocalExecutor::execute(...)` return type;
- changing `LocalExecutor::decide_approval(...)` return type;
- changing `LocalExecutor::cancel_run(...)` return type;
- automatic report generation for every run;
- workflow-declared report contracts;
- runtime config for reports;
- persistence;
- filesystem report artifacts;
- CLI rendering or commands;
- JSON export;
- examples;
- schema changes;
- reasoning lineage;
- side-effect boundary modeling;
- writes;
- approval evidence attachment;
- production reporting;
- report signing or notarization;
- SIEM or OpenTelemetry integration;
- DLP or access-control systems;
- release posture changes.

## 4. Current Executor Boundary

Current executor methods:

- `LocalExecutor::execute(...) -> Result<WorkflowRun, WorkflowOsError>`;
- `LocalExecutor::decide_approval(...) -> Result<WorkflowRun, WorkflowOsError>`;
- `LocalExecutor::cancel_run(...) -> Result<WorkflowRun, WorkflowOsError>`.

Current behavior:

- executor methods write workflow events and projections through `StateBackend`;
- terminal statuses are completed, failed, and canceled;
- `Escalated` is not terminal in current runtime semantics;
- report generation is not part of executor state transitions;
- report generation is available only through explicit helper calls.

The first executor-integrated report result implementation should not alter existing methods. Existing callers must continue receiving `WorkflowRun`.

## 5. Candidate API Shape

Implemented API:

- `LocalExecutionWithReportRequest`;
- `LocalExecutionReportInputs`;
- `LocalExecutionWithReportResult`;
- `LocalExecutor::execute_with_report(...) -> Result<LocalExecutionWithReportResult, WorkflowOsError>`.

Candidate request fields:

- existing `LocalExecutionRequest`;
- report ID;
- report contract ID;
- report contract version;
- generated timestamp;
- generated actor/system actor;
- sensitivity;
- redaction metadata;
- optional correlation ID override;
- supplied `EvidenceReferenceId` values;
- supplied workflow event IDs;
- supplied audit event IDs;
- supplied adapter telemetry stable references;
- supplied validation reference IDs;
- supplied policy decision event IDs;
- supplied approval reference IDs where stable;
- bounded incomplete-work disclosures;
- bounded known limitations;
- bounded risks;
- bounded operator handoff notes.

Candidate result fields:

- `run: WorkflowRun`;
- `work_report: Option<WorkReport>`;
- `report_generation_error: Option<WorkflowOsError>` if warning-style behavior is selected.

## 6. Recommended Error Policy

Preferred v1 behavior: return a successful `WorkflowRun` plus a report error field when workflow execution succeeds but report generation fails.

Rationale:

- report generation is not yet part of workflow semantics;
- report construction should not retroactively fail a completed workflow;
- callers using the explicit report-bearing path still need visibility into report failure;
- the report error can remain structured and non-leaking.

Required rule:

- If workflow execution itself fails before producing a `WorkflowRun`, return `Err` as today.
- If workflow execution returns a terminal `WorkflowRun` but report generation fails, return a result with `work_report: None` and `report_generation_error: Some(error)`.
- If workflow execution returns a non-terminal `WorkflowRun`, do not generate a report; return a result with no report and a structured non-terminal report error, or reject according to the final implementation decision.

Implemented decision: the result wrapper contains a `WorkflowRun` whenever execution returns one. Non-terminal outcomes return no report plus a structured `report_generation_error`; they do not become execution failures.

## 7. Terminal Status Policy

Supported for report generation:

- completed;
- failed;
- canceled.

Deferred:

- escalated runtime reports until `WorkflowRunStatus::Escalated` is intentionally terminal;
- blocked runtime reports until a runtime blocked terminal state exists.

The executor integration must not change `WorkflowRunStatus::is_terminal()`.

## 8. Integration Boundary

Implemented sequence inside `execute_with_report(...)`:

1. Call existing `execute(...)`.
2. If execution returns `Err`, return `Err`.
3. If execution returns a terminal run, pass it to `expose_terminal_local_work_report_result(...)`.
4. If report exposure succeeds, return run plus report.
5. If report exposure fails, return run plus structured report error according to the selected policy.

Do not duplicate executor state transition logic. Do not fork `execute(...)` internals.

This keeps report integration outside the event append path and avoids creating a second runtime execution implementation.

## 9. Citation And Input Policy

The future executor API should not discover or fabricate references.

Allowed:

- supplied `EvidenceReferenceId` values;
- supplied workflow event IDs;
- supplied audit event IDs;
- supplied adapter telemetry stable references;
- supplied validation reference IDs;
- supplied policy event IDs;
- supplied approval reference IDs where stable;
- bounded disclosures, limitations, risks, and handoff notes.

Deferred:

- automatic citation discovery from stores;
- automatic validation diagnostic collection from loader/validator results;
- approval evidence attachment;
- explicit missing-citation records instead of section text;
- report contract-driven required citation enforcement.

## 10. Workflow Semantics

Executor-integrated report result exposure must not:

- change existing `execute(...)` behavior;
- change pass/fail outcome;
- append post-terminal events;
- write report data to `StateBackend`;
- mutate `WorkflowRun`;
- mutate `WorkflowRunSnapshot`;
- mutate event history;
- emit audit events for report generation;
- emit observability events for report generation;
- create filesystem artifacts;
- expose CLI output.

Generated reports are derived in-memory values, not event-sourced workflow state.

## 11. Privacy And Redaction

Future executor integration must:

- use `expose_terminal_local_work_report_result(...)`;
- use existing `WorkReport` validation and redaction boundaries;
- keep report result Debug output safe;
- reject or safely handle secret-like report inputs;
- keep report generation errors stable and non-leaking.

It must not copy:

- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira issue/comment bodies;
- raw GitHub file contents;
- raw spec contents;
- raw parser payloads;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values.

## 12. Compatibility Posture

Implementation should be additive:

- existing executor methods remain unchanged;
- new request/result types are local runtime APIs only;
- no schema exposure;
- no CLI output;
- no persisted report format;
- no examples until operator-facing behavior is separately planned.

The new API should be documented as in-memory and local. It should not promise stable report artifact compatibility.

## 13. Test Plan

Future implementation tests should cover:

- existing `execute(...)` still returns `WorkflowRun`;
- `execute_with_report(...)` returns completed run plus report;
- `execute_with_report(...)` returns failed run plus report;
- cancellation report path if a cancellation-specific method is introduced;
- waiting-for-approval/non-terminal execution does not fabricate a report;
- report generation failure preserves the run and exposes a non-leaking report error;
- report generation failure does not append events;
- report generation failure does not write backend report state;
- no filesystem artifacts are created;
- no CLI output is emitted;
- generated report contains all v1 sections;
- supplied stable references are cited;
- absent references become not-available section text;
- secret-like report inputs are rejected without leakage;
- Debug and serialization remain safe for report model values;
- existing runtime tests still pass;
- existing WorkReport and WorkReportContract tests still pass.

## 14. Implementation Sequence

Completed:

1. Added additive executor report request/result types.
2. Added `execute_with_report(...)` that wraps existing `execute(...)`.
3. Returns run plus report when report generation succeeds.
4. Returns run plus report error when report generation fails after execution returns a run.
5. Added focused tests for completed, failed, waiting-for-approval/non-terminal, non-mutation, no report persistence, and non-leakage.
6. Review is still required before considering approval/cancellation report APIs, CLI display, artifacts, schema exposure, examples, or persistence.

## 15. Open Questions

- Should approval resume paths get a separate `decide_approval_with_report(...)` method later?
- Should cancellation get a separate `cancel_run_with_report(...)` method later?
- Should report inputs be nested under a reusable `LocalReportGenerationRequest`?
- Should the API accept a full `WorkReportContract` instance before workflow-declared contracts exist?
- Should missing citations remain section text until artifact/CLI phases?
- Should report-bearing APIs remain public exports or be marked experimental in docs?

## 16. Final Recommendation

Maintainer review accepted the executor-integrated report result implementation with non-blocking follow-ups. Report artifact store planning and implementation followed, but `execute_with_report(...)` still does not write artifacts automatically. Review the artifact store before adding automatic artifact writing, CLI rendering, persistence beyond the planned artifact boundary, or schema exposure.

Future work must not add approval/cancellation report-bearing methods, generate reports automatically for every run, append events, persist reports, create artifacts, render CLI output, change schemas, update examples, implement reasoning lineage, model side effects, add writes, attach approval evidence, or change release posture unless separately scoped and reviewed.
