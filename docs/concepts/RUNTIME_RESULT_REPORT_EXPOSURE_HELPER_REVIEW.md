# Runtime Result Report Exposure Helper Review

Review date: 2026-06-05

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The runtime result exposure helper phase adds a narrow in-memory result wrapper and helper for pairing a terminal `WorkflowRun` with a generated `WorkReport`. The implementation stays within the accepted boundary: it does not change existing executor return types, does not automatically generate reports, does not mutate runtime state, does not append events, does not persist reports, does not write artifacts, and does not expose CLI output.

No blockers were found before the next planning phase.

## 2. Scope Verification

The phase stayed within the approved in-memory helper scope.

Verified in scope:

- `TerminalLocalWorkReportResult` owns a `WorkflowRun` and `WorkReport`.
- `expose_terminal_local_work_report_result(...)` returns the wrapper from explicit `TerminalLocalWorkReportInput`.
- Existing `LocalExecutor::execute(...)`, approval, and cancellation methods still return `WorkflowRun`.
- The helper reuses `generate_terminal_local_work_report(...)`.
- Non-terminal run input fails through `work_report_generation.status.not_terminal`.
- The implementation is additive and local.

No accidental implementation was found for:

- automatic runtime report generation;
- executor-integrated automatic result exposure;
- report artifacts;
- persistence;
- filesystem writing;
- CLI behavior;
- examples;
- workflow spec schema changes;
- workflow-declared report contracts;
- runtime config;
- reasoning lineage;
- side-effect boundary;
- writes;
- approval evidence attachment;
- release posture changes.

## 3. API Assessment

The API is intentionally small.

`TerminalLocalWorkReportResult` exposes:

- `run() -> &WorkflowRun`;
- `work_report() -> &WorkReport`;
- `into_parts() -> (WorkflowRun, WorkReport)`.

`expose_terminal_local_work_report_result(...)` accepts the existing explicit report-generation input and returns `Result<TerminalLocalWorkReportResult, WorkflowOsError>`.

The helper does not read hidden global state, does not require a `StateBackend`, does not require live adapters, and does not call external systems. It clones the supplied run before returning the wrapper. That is acceptable for a model/helper phase because `WorkflowRun` is the current runtime result object and report exposure remains in memory only.

Non-blocking follow-up: before broader public compatibility expectations attach, decide whether this helper should remain exported from `workflow-core`, move behind a more explicit experimental namespace, or become an executor-adjacent API.

## 4. Runtime Semantics Assessment

Verified:

- The existing executor result shape remains unchanged.
- The helper does not execute workflows.
- The helper does not mutate the borrowed `WorkflowRun`.
- The helper does not mutate `WorkflowRunSnapshot`.
- The helper does not mutate event history.
- The helper does not append post-terminal events.
- The helper does not emit audit or observability events.
- The helper does not touch a `StateBackend`.
- The helper does not alter terminal status semantics.

The result wrapper is a derived in-memory value, not event-sourced runtime state.

## 5. Terminal Status Assessment

The helper inherits terminal status behavior from `generate_terminal_local_work_report(...)`.

Verified supported terminal statuses:

- completed;
- failed;
- canceled.

Verified rejected status:

- running/non-terminal input returns `work_report_generation.status.not_terminal`.

Runtime `Escalated` remains non-terminal, and `Blocked` remains report vocabulary only. The phase does not change runtime terminal semantics.

## 6. Report Construction Assessment

The exposure helper delegates report construction to the existing terminal report helper. This is the right boundary because it keeps validation and redaction gates centralized.

Verified:

- `WorkReport` construction still flows through `WorkReport::new(...)`.
- `WorkReportSection` construction still flows through `WorkReportSection::new(...)`.
- `WorkReportCitation` construction still flows through `WorkReportCitation::new(...)`.
- The exposure helper does not create a parallel section or citation path.
- The exposure helper does not recreate `EvidenceReference` values.
- The exposure helper does not fabricate IDs.

## 7. Error Handling Assessment

Report exposure errors remain structured and non-leaking.

Verified:

- non-terminal input returns `work_report_generation.status.not_terminal`;
- secret-like supplied text is rejected by existing report model validation;
- errors do not include raw secret-like test values;
- failure returns `Err` from the explicit helper path and does not mutate the run.

This matches the conservative plan. Future executor integration planning should decide whether report-bearing executor APIs return `Err` on report generation failure or return a run plus a separate report error field.

## 8. Privacy And Redaction Assessment

The helper preserves the report privacy posture.

Verified:

- `TerminalLocalWorkReportResult` has a manual redaction-safe `Debug` implementation.
- Debug output includes run status and event count, but not run ID, workflow ID, adapter references, report text, or secret-like values.
- The result wrapper is not serialized and does not create a schema surface.
- Underlying `WorkReport` serialization and Debug protections remain intact.
- No raw provider payloads, raw command output, raw CI logs, raw Jira bodies/comments, raw GitHub file contents, raw spec contents, raw parser payloads, environment variable values, credentials, authorization headers, private keys, or token-like values are introduced.

Non-blocking follow-up: if `TerminalLocalWorkReportResult` is ever serialized or exposed through CLI/API output, it needs a separate schema/output posture review.

## 9. Test Quality Assessment

The tests are focused and appropriate for this phase.

Covered:

- in-memory result exposure for completed terminal runs;
- owned run/report extraction through `into_parts()`;
- non-terminal rejection without run mutation;
- redaction-safe result Debug output;
- existing completed, failed, and canceled report generation;
- required v1 report sections;
- stable citation behavior;
- no filesystem artifacts;
- no CLI output;
- secret-like input rejection;
- existing WorkReport, WorkReportContract, EvidenceReference, Diagnostic, validation, adapter telemetry, and runtime tests through `cargo test --workspace`.

No blocker-level missing tests were found.

Non-blocking follow-up: add direct tests around future executor-adjacent report-bearing methods when such APIs are introduced. The current helper intentionally does not exercise `LocalExecutor` integration.

## 10. Documentation Review

Docs now state:

- in-memory runtime result exposure helper is implemented;
- automatic runtime report generation is not implemented;
- executor-integrated automatic result exposure is not implemented;
- report artifacts are not implemented;
- persistence is not implemented;
- CLI rendering is not implemented;
- examples are not updated;
- workflow schema changes are not implemented;
- reasoning lineage is not implemented;
- side-effect boundary is not implemented;
- writes remain unsupported.

The phase report documents the remaining limitations and recommends review before any broader runtime integration.

## 11. Blockers

No blockers found.

## 12. Non-Blocking Follow-Ups

- Decide whether `TerminalLocalWorkReportResult` should remain part of the public `workflow-core` export surface before broader compatibility promises attach.
- Decide whether future executor-integrated report APIs should return `Err` on report generation failure or return a successful run plus a report error field.
- Decide whether missing citations should remain section text or become explicit missing-citation records before report artifacts or CLI output.
- Add direct tests for executor-adjacent report-bearing methods if a later phase introduces them.
- Review serialization/schema posture before exposing report-bearing results through CLI, JSON, persistence, or public schemas.

## 13. Recommended Next Phase

Recommended next phase: executor-integrated report result planning.

The current helper proves the report-bearing in-memory result shape, but existing executor methods still return `WorkflowRun` only. The next planning phase should decide whether to add an explicit executor method or request type that returns a report-bearing result while preserving existing methods, avoiding automatic generation for every run, avoiding post-terminal events, avoiding persistence/artifacts/CLI/schema exposure, and keeping workflow semantics unchanged.

## Validation

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
