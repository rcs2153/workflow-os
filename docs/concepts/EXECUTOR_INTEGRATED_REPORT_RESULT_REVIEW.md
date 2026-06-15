# Executor-Integrated Report Result Review

Review date: 2026-06-07

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation adds an explicit, additive `LocalExecutor::execute_with_report(...)` path that wraps the existing local executor, returns an in-memory report-bearing result, and preserves existing executor methods and workflow semantics. It uses the existing terminal local report generation and runtime result exposure helpers, keeps report generation failure separate from workflow execution failure after a run exists, and does not add automatic report generation, persistence, artifacts, CLI rendering, schema changes, examples, reasoning lineage, side-effect modeling, writes, approval evidence attachment, runtime config, or release posture changes.

No blockers were found before the next planning phase.

## 2. Scope Verification

The phase stayed within the approved executor-adjacent, in-memory result scope.

Verified in scope:

- `LocalExecutionReportInputs` captures explicit report-generation inputs.
- `LocalExecutionWithReportRequest` composes an existing `LocalExecutionRequest` with explicit report inputs.
- `LocalExecutionWithReportResult` returns the owned `WorkflowRun`, optional `WorkReport`, and optional report-generation error through read-only accessors and `into_parts()`.
- `LocalExecutor::execute_with_report(...)` calls the existing `execute(...)` path and then attempts in-memory report exposure.
- Existing `execute(...)`, `decide_approval(...)`, and `cancel_run(...)` signatures remain unchanged.
- Report generation failures after a run exists are returned as report errors rather than retroactively failing workflow execution.
- Focused tests and documentation updates were added.
- The end-of-phase report exists.

No accidental implementation was found for:

- automatic runtime report generation;
- report artifacts;
- persistence;
- CLI behavior;
- example updates;
- workflow spec schema changes;
- runtime config creation;
- approval/cancellation report-bearing methods;
- reasoning lineage implementation;
- side-effect boundary;
- writes;
- approval evidence attachment;
- release posture changes.

## 3. API Assessment

The new API is narrow, explicit, and compatible with existing local execution callers.

`LocalExecutionWithReportRequest` does not replace `LocalExecutionRequest`; it wraps it. That keeps existing callers on `execute(...) -> Result<WorkflowRun, WorkflowOsError>` unchanged while giving report-aware callers an opt-in path.

`LocalExecutionReportInputs` is explicit rather than runtime-config-driven. It accepts contract identity/version, generation identity, sensitivity, redaction metadata, optional correlation override, stable citation/reference IDs, and bounded report note text. The implementation does not invent workflow-declared report contracts or hidden configuration.

`LocalExecutionWithReportResult` keeps its fields private and provides read-only accessors plus `into_parts()`. Its manual `Debug` implementation reports only status, event count, report presence, and report error code. That is appropriate for a report-bearing wrapper that may contain sensitive report text or identifiers.

The new types are exported from `workflow-core` consistently with the existing executor/report APIs. That is acceptable for this phase, with a compatibility follow-up before schema, artifact, or CLI exposure.

## 4. Executor Integration Assessment

The integration point is appropriately small.

`execute_with_report(...)` calls `self.execute(&request.execution)` and does not duplicate executor state-transition logic. If execution fails before producing a run, the original execution error is returned unchanged. If execution returns a run, the implementation derives report-generation input from the run plus explicit report inputs and calls the existing in-memory report exposure helper.

This preserves the current executor lifecycle and avoids adding report generation into the lower-level state machine, event builder, backend, audit sink, observability sink, or CLI layer.

The implementation does not alter `execute(...)`, `decide_approval(...)`, or `cancel_run(...)`. Approval-resume and cancellation report-bearing APIs remain future work.

## 5. Terminal Status Behavior

Verified:

- completed local execution returns a completed `WorkflowRun` plus a generated report;
- failed local execution returns a failed `WorkflowRun` plus a generated report;
- waiting-for-approval/non-terminal execution returns a `WorkflowRun`, no report, and `work_report_generation.status.not_terminal`;
- non-terminal handling does not change the run status or fail the executor call;
- escalated and blocked are not introduced as runtime terminal statuses;
- existing runtime terminal semantics remain unchanged.

Cancellation report generation is not directly exposed through this method because cancellation currently uses `cancel_run(...)`, and a cancellation-specific report-bearing API is explicitly out of scope. That is acceptable for this phase because the executor-integrated path is limited to `execute_with_report(...)`; cancellation report-bearing behavior should be planned separately before claiming full terminal lifecycle report coverage.

## 6. Report Generation Error Policy

The report error policy matches the accepted plan.

Verified:

- execution errors before a run exists remain normal `Err` results;
- report generation errors after a run exists are returned in `LocalExecutionWithReportResult::report_generation_error()`;
- report generation errors do not cause the workflow result to change;
- secret-like report input causes report generation to fail without leaking the secret-like value through result or error Debug;
- non-terminal report generation failure uses stable code `work_report_generation.status.not_terminal`;
- no fake report is produced when report generation fails.

This is the correct conservative behavior for in-memory report-bearing execution.

## 7. Runtime State and Event Boundary

The implementation preserves the runtime state boundary.

Verified:

- `execute_with_report(...)` does not mutate the returned `WorkflowRun` after execution beyond the existing `execute(...)` behavior;
- tests compare backend events with the returned run events after report generation;
- report generation failure does not append events;
- no post-terminal workflow events are introduced;
- no report state is written to the backend;
- no filesystem report artifact is created;
- no CLI output path is added.

The current tests prove backend event equality for success, non-terminal report error, and report generation failure paths. A future custom-sink test could make no-audit/no-observability-report-emission guarantees even more explicit, but no regression was found.

## 8. Citation and Report Construction Assessment

Report construction is delegated to existing validated helpers.

Verified:

- `execute_with_report(...)` uses `expose_terminal_local_work_report_result(...)`;
- report construction flows through the existing terminal local report generation helper;
- `WorkReport`, `WorkReportSection`, and `WorkReportCitation` constructor validation remains active;
- supplied `EvidenceReferenceId` values are cited by stable reference and no `EvidenceReference` values are recreated;
- supplied validation references and adapter telemetry stable references are cited;
- unavailable references remain explicit none/not-available section text;
- generated reports contain the v1 required section set;
- side-effect content remains none/skipped/unsupported and does not imply write support.

The implementation keeps missing-citation records deferred; this matches the accepted plan for the current phase.

## 9. Privacy and Redaction Assessment

The privacy posture remains sound.

Verified:

- request and result wrapper `Debug` output is redaction-safe;
- report input Debug redacts IDs, actor, correlation details, redaction metadata, and note text;
- report result Debug does not expose report text, IDs, paths, tokens, or secret-like inputs;
- report generation uses existing WorkReport redaction and note validation;
- raw provider payloads are not copied;
- raw spec contents are not copied;
- raw command output is not copied;
- raw parser payloads are not copied;
- environment values and credentials are not copied;
- report generation errors use stable codes and avoid raw payload values.

The wrapper still exposes the generated `WorkReport` through an accessor when generation succeeds. That is intended: the caller explicitly requested a report-bearing execution path. The safe-output requirement is met by model validation and Debug redaction, not by hiding the report value from the caller.

## 10. Compatibility Assessment

The change is additive.

Verified:

- existing `execute(...) -> Result<WorkflowRun, WorkflowOsError>` remains unchanged;
- existing approval and cancellation APIs remain unchanged;
- no workflow spec schema changes were introduced;
- no runtime config was introduced;
- no persistence or artifact compatibility surface was introduced;
- no CLI JSON or display contract was introduced;
- existing report, evidence, diagnostic, validation, adapter telemetry, runtime, and docs checks pass.

The public export of the new request/result types is appropriate for local in-memory usage. Before persistence, CLI, artifact, or schema exposure, maintainers should decide whether these APIs need experimental language or compatibility hardening.

## 11. Test Quality Assessment

The tests are focused and cover the important behavior.

Covered:

- existing `execute(...)` still returns `WorkflowRun`;
- `execute_with_report(...)` returns completed run plus report;
- `execute_with_report(...)` returns failed run plus report;
- waiting-for-approval/non-terminal execution returns run plus non-leaking report error and no report;
- secret-like report input returns run plus non-leaking report error;
- report generation failure does not append backend events;
- no report artifact file is created;
- generated report includes all v1 sections;
- supplied `EvidenceReferenceId` and adapter telemetry references are cited;
- unavailable references become explicit section text;
- side effects section remains unsupported/no-write;
- wrapper Debug output is redaction-safe;
- existing executor tests still cover baseline runtime behavior.

Missing or shallow areas, all non-blocking for this phase:

- no cancellation-specific `execute_with_report` test exists because cancellation report-bearing APIs are intentionally not implemented;
- no direct custom audit/observability sink assertion proves report generation emits no sink events, though the implementation path does not call those sinks;
- `into_parts()` is available but not directly tested;
- no serialization test exists for the executor result wrapper, which is acceptable because no schema, CLI, artifact, or persistence exposure was added.

## 12. Documentation Review

Docs state:

- executor-integrated report-bearing execution is implemented for local runs;
- existing executor APIs remain unchanged;
- automatic runtime report generation is not implemented;
- approval/cancellation report-bearing methods are not implemented;
- report artifacts are not implemented;
- persistence is not implemented;
- CLI rendering is not implemented;
- examples are not updated;
- workflow schema changes are not implemented;
- reasoning lineage is not implemented;
- side-effect boundary is not implemented;
- writes remain unsupported;
- release posture is unchanged.

The docs do not overclaim production report artifacts or automatic reporting.

## 13. Blockers

No blockers found.

## 14. Non-Blocking Follow-Ups

- Plan cancellation and approval-resume report-bearing APIs separately before claiming full terminal lifecycle report coverage.
- Decide whether the new public executor report APIs need explicit experimental/compatibility language before CLI, artifact, persistence, or schema exposure.
- Add direct custom sink tests if future report integration gets close to audit or observability emission points.
- Add a direct `into_parts()` test for `LocalExecutionWithReportResult`.
- Revisit missing-citation records before report artifacts or CLI rendering; section text is acceptable for this phase.
- Decide whether future report-bearing execution should accept a full `WorkReportContract` instance once workflow-declared contracts are planned.

## 15. Recommended Next Phase

Recommended next phase: report artifact planning.

The executor-integrated in-memory path is now sufficient to prove local execution can return a report-bearing result without changing existing executor semantics. The next roadmap question should be how, when, and whether generated reports become durable artifacts. That planning should keep persistence, CLI rendering, schemas, examples, approval/cancellation report-bearing APIs, reasoning lineage, side effects, writes, and release posture changes out of scope unless explicitly approved.

## 16. Validation

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
