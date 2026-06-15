# Terminal Report Local Check Citation Integration Report

Report date: 2026-06-15

## 1. Executive Summary

Workflow OS terminal local WorkReport generation now accepts supplied local check result references and cites them in generated in-memory reports.

The phase adds an explicit `local_check_result_references` input vector to terminal report generation inputs and executor report inputs. Supplied references are converted into `WorkReportCitationTarget::LocalCheckResult` citations through existing `WorkReportCitation::new(...)` validation and are placed in the `validation and quality checks` section.

The implementation remains explicit-input only. It does not execute local checks, create local check result references, create EvidenceReference values, use command-output evidence, persist reports, write artifacts, expose CLI behavior, add schema fields, register handlers by default, model side effects, add writes, or change release posture.

## 2. Governance Run

This implementation phase was governed by the self-governance dogfood workflow before code changes.

- State directory: `/tmp/workflow-os-terminal-report-local-check-citation-impl`
- Run ID: `run-1781542576029571000-2`
- Workflow ID: `dg/d`
- Approval ID: `approval/run-1781542576029571000-2/d`
- Final status: `Completed`

## 3. Scope Completed

- Added `TerminalLocalWorkReportInput::local_check_result_references`.
- Added `LocalExecutionReportInputs::local_check_result_references`.
- Threaded supplied local check references through executor-integrated report input conversion.
- Added local check result citation construction for terminal local reports.
- Added local check citations to `WorkReportSectionKind::ValidationAndQualityChecks`.
- Preserved validation diagnostic citations in the same section.
- Updated validation/quality section summaries for validation-only, local-check-only, both, and neither cases.
- Added focused tests for terminal helper and executor-integrated report paths.
- Updated roadmap and planning documentation.

## 4. Scope Explicitly Not Completed

- No automatic local check execution.
- No local check result reference creation from handlers, executor paths, or report helpers.
- No local check result persistence.
- No local check artifact writing.
- No EvidenceReference attachment.
- No `EvidenceKind::CommandOutput` usage.
- No command-output evidence policy change.
- No default `DocsCheck` registration.
- No CLI exposure.
- No workflow schema fields.
- No automatic report generation for every run.
- No automatic report artifact writing from executor paths.
- No side-effect boundary implementation.
- No source writes.
- No write-capable adapters.
- No release posture change.

## 5. API Summary

New input fields:

- `TerminalLocalWorkReportInput::local_check_result_references: Vec<WorkReportStableReference>`
- `LocalExecutionReportInputs::local_check_result_references: Vec<WorkReportStableReference>`

The fields accept caller-supplied stable local check result references. They do not create references, execute checks, or read local check results from storage.

## 6. Citation Construction Summary

The terminal report helper now builds local check citations by:

1. iterating supplied stable local check references;
2. constructing `WorkReportCitationTarget::LocalCheckResult { reference }`;
3. constructing each citation through `WorkReportCitation::new(...)`;
4. using bounded generic summary text;
5. placing the citations in `ValidationAndQualityChecks`.

Invalid references fail before or during report construction through existing stable-reference and WorkReport citation validation.

## 7. Section Population Summary

`ValidationAndQualityChecks` now supports:

- validation diagnostic citations only;
- local check result citations only;
- both validation diagnostic and local check result citations;
- explicit not-available text when neither is supplied.

The section does not copy raw local check output, stdout, stderr, command transcripts, docs contents, parser payloads, provider payloads, environment values, or secrets.

## 8. Workflow Semantics Summary

The implementation does not alter workflow execution semantics.

- `LocalExecutor::execute(...)` remains unchanged.
- `LocalExecutor::execute_with_report(...)` still returns the run plus report or report-generation error.
- Report generation failure after execution does not change the workflow run status.
- The report helper does not mutate workflow runs, snapshots, event history, state backend, or artifact stores.
- The report helper does not append events or emit CLI output.

## 9. Privacy And Redaction Summary

Local check citations are reference-only.

The implementation does not store or copy:

- raw stdout;
- raw stderr;
- raw command transcripts;
- raw CI logs;
- raw docs contents;
- raw spec contents;
- raw parser payloads;
- provider payloads;
- environment values;
- credentials;
- tokens;
- authorization headers;
- private keys.

`WorkReportStableReference`, `WorkReportCitationTarget`, and `WorkReportCitation` retain existing validation and redaction-safe debug behavior.

## 10. Test Coverage Summary

Added or updated tests cover:

- generated reports cite local check result references by stable reference;
- generated reports preserve validation diagnostic citations alongside local check citations;
- generated reports without local check or validation references retain explicit not-available text;
- executor-integrated report-bearing execution propagates supplied local check references into the generated report;
- no raw command-output markers are serialized;
- existing terminal helper, WorkReport, WorkReportContract, local check, EvidenceReference, Diagnostic, adapter telemetry, executor, and runtime tests remain part of full validation.

## 11. Commands Run And Results

Validation commands for this phase:

- `cargo fmt --all --check`
  - Passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Passed.
- `cargo test -p workflow-core --test work_report`
  - Passed.
- `cargo test -p workflow-core --test local_executor execute_with_report_returns_completed_run_plus_report`
  - Passed.
- `cargo test --workspace`
  - Passed.
- `npm run check:docs`
  - Passed.

## 12. Remaining Known Limitations

- Local check result references must be supplied by callers.
- Report helpers do not create `LocalCheckResultReference` values.
- Local checks are not executed by report generation.
- Local check result references are not persisted.
- Local check result references are not attached to EvidenceReference.
- Command-output evidence remains deferred.
- Default handler registration remains deferred.
- CLI/schema exposure remains deferred.
- Side-effect modeling remains deferred.

## 13. Recommended Next Phase

Recommended next phase: **terminal report local check citation integration review**.

The review should verify that the implementation is explicit-input only, redaction-safe, reference-only, and does not introduce automatic local check execution, local check reference creation, EvidenceReference attachment, command-output evidence, persistence, artifacts, CLI behavior, workflow schema fields, default handler registration, side-effect modeling, writes, or release posture changes.
