# Executor Report Artifact SideEffect Gates Review

Review date: 2026-06-21

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The phase adds a narrow, explicit executor-adjacent artifact-writing path that composes local execution, in-memory report generation, `SideEffect` referential integrity validation, and store-backed approval-linkage validation before writing a `WorkReportArtifactRecord`.

The implementation is a real runtime step forward without changing existing executor semantics. Existing `LocalExecutor::execute(...)`, `LocalExecutor::execute_with_report(...)`, and `execute_with_report_and_side_effect_discovery(...)` paths remain unchanged and do not write artifacts automatically.

## 2. Scope Verification

The phase stayed within the approved explicit artifact-gate scope.

Implemented:

- `LocalExecutionReportArtifactInputs`
- `LocalExecutionWithReportArtifactRequest`
- `LocalExecutionWithReportArtifactResult`
- `LocalExecutionWithReportArtifactParts`
- `WorkReportArtifactGovernedWriteInput`
- `WorkReportArtifactGovernedWriteResult`
- `write_work_report_artifact_with_side_effect_integrity_and_approval_linkage(...)`
- `execute_with_report_artifact_and_side_effect_gates(...)`
- crate-root exports for the new helper/types
- focused executor tests for successful artifact write, discovered `SideEffect` integrity, missing `SideEffect` failure, duplicate artifact write, event-history preservation, and redaction-safe debug/serialization boundaries
- documentation and end-of-phase report updates

No accidental implementation was found for:

- automatic report artifact writing from existing executor methods;
- automatic runtime report generation for every run;
- runtime side-effect execution;
- provider mutation;
- write-capable adapters;
- workflow schema changes;
- CLI rendering or export behavior;
- examples;
- hosted or distributed runtime behavior;
- reasoning lineage;
- approval evidence attachment;
- approval/cancellation report-bearing artifact methods;
- release posture changes.

## 3. Executor API Assessment

The executor API is explicit and additive.

`execute_with_report_artifact_and_side_effect_gates(...)` accepts:

- a caller-supplied `LocalExecutor`;
- a caller-supplied `WorkReportArtifactStore`;
- a caller-supplied `SideEffectRecordStore`;
- an explicit execution/report/artifact request.

It does not construct stores, infer filesystem paths, add runtime configuration, read hidden global state, call live adapters, emit CLI output, or alter existing executor APIs.

The result wrapper separates the important outcomes:

- workflow run;
- generated report;
- report-generation error;
- written artifact;
- artifact-write error;
- `SideEffect` integrity counts;
- approval-linkage counts.

That separation is important because report/artifact failure after a run exists must not retroactively change workflow pass/fail semantics.

## 4. Runtime Semantics Assessment

Runtime semantics are preserved.

Verified behavior:

- execution failures before a run exists still return `Err` from the underlying executor path;
- non-terminal report generation remains a report-generation error rather than a workflow execution error;
- report-generation failure after a run exists returns the run with no report/artifact;
- artifact-gate or artifact-write failure after report generation returns the run and report with an artifact error;
- duplicate artifact writes preserve the rehydrated run and existing artifact;
- helper paths do not append workflow events;
- helper paths do not mutate `WorkflowRun` or `WorkflowRunSnapshot`;
- helper paths do not emit audit or observability events;
- helper paths do not execute side effects;
- helper paths do not call providers or adapters.

The only artifact persistence introduced is the explicit `WorkReportArtifactStore::write_work_report_artifact(...)` call made after gates pass on the new opt-in helper path.

## 5. Artifact Gate Assessment

The governed write helper composes the expected gates in the correct order:

1. validate the `WorkReportArtifactRecord`;
2. verify artifact/report identity matches the supplied terminal run;
3. validate cited `SideEffect` IDs against the supplied `SideEffectRecordStore`;
4. validate approval linkage for cited `SideEffect` records;
5. write the artifact only after validation succeeds.

This is the right boundary for this phase. It does not change the base artifact store trait, and it does not force every artifact store implementation to know about `SideEffect` records.

One non-blocking hardening point remains: add direct tests for the governed write helper itself, independent of the executor wrapper, especially artifact/run identity mismatch and non-terminal run rejection. Current coverage exercises the helper through the executor path.

## 6. SideEffect And Approval-Linkage Assessment

The implementation correctly treats `SideEffect` citations as stable references, not payload copies.

Positive behavior:

- reports without `SideEffect` citations can write successfully;
- strict mode fails before artifact write when a cited `SideEffect` record is missing;
- discovered store-backed `SideEffect` records can be cited and validated;
- approval linkage is skipped when no `SideEffect` citations exist;
- approval linkage is delegated to the accepted store-backed helper when citations exist;
- approval-linkage result exposure is bounded to counts.

The phase does not claim that a cited side effect was executed, completed, provider-verified, or safe to mutate. It validates only reference integrity and approval-linkage posture for cited records.

## 7. Error Handling And Privacy Assessment

Error handling remains stable and non-leaking.

The new helper maps artifact validation and identity mismatch to stable governed-write codes:

- `work_report_artifact.governed_write.invalid_artifact`
- `work_report_artifact.governed_write.identity_mismatch`
- `work_report_artifact.governed_write.failed`

SideEffect integrity and approval-linkage errors preserve their existing stable code families. Tests verify the missing SideEffect ID does not leak through the artifact error debug path.

`Debug` output for the new request/result types exposes booleans, statuses, counts, and error codes. It does not expose report text, run IDs, report IDs, correlation IDs, side-effect IDs, approval IDs, target references, paths, raw provider payloads, command output, parser payloads, raw spec contents, credentials, or token-like values.

## 8. Persistence And Artifact Posture Assessment

The phase intentionally introduces one explicit artifact-write path.

This is acceptable because report artifact storage already existed as an explicit store boundary, and this phase writes only through a caller-supplied `WorkReportArtifactStore` after validation gates pass.

The implementation does not:

- write artifacts from existing executor methods;
- create hidden local artifact stores;
- add CLI export;
- add public schemas;
- write side-effect records;
- mutate workflow state;
- append artifact events.

## 9. Test Quality Assessment

Tests cover:

- successful explicit executor artifact write after report generation;
- discovered `SideEffect` record citation before artifact write;
- missing cited `SideEffect` record failure before artifact write;
- duplicate artifact write preserving the existing artifact and run events;
- no event-history mutation from artifact paths;
- redaction-safe request/result debug output;
- no raw provider or command-output marker copying into artifact serialization;
- existing report-only executor path still does not write artifacts;
- full existing `WorkReport`, `WorkReportContract`, `EvidenceReference`, Diagnostic, validation, adapter telemetry, runtime event, SideEffect, discovery, and artifact tests.

No blocker-level test gaps were found.

Non-blocking test gaps:

- Add direct governed-write helper tests for artifact/run identity mismatch.
- Add direct governed-write helper tests for non-terminal supplied run behavior.
- Add executor artifact tests for failed and canceled terminal runs, not only completed runs.
- Add an explicit artifact-gate test where approval linkage fails after SideEffect integrity succeeds.
- Add a direct permissive-mode test where missing `SideEffect` citations are counted and artifact writing is intentionally allowed.
- Add a direct assertion that `execute(...)`, `execute_with_report(...)`, and `execute_with_report_and_side_effect_discovery(...)` still leave artifact stores empty.

## 10. Documentation Review

Documentation accurately says:

- explicit executor report artifact writing with `SideEffect` integrity and approval-linkage gates is implemented;
- existing executor methods do not write artifacts automatically;
- automatic runtime report generation for every run is not implemented;
- automatic SideEffect discovery is not implemented;
- runtime side-effect execution is not implemented;
- provider writes and write-capable adapters are not implemented;
- schemas, CLI behavior, examples, hosted behavior, reasoning lineage, approval evidence attachment, and release posture changes remain unimplemented.

The phase report is accurate and recommends this review before broader runtime side-effect execution, provider writes, automatic artifact writing, or write-capable adapter readiness work.

One non-blocking documentation improvement: after this review is accepted, link this review from the roadmap status paragraph so the roadmap points to the accepted review, not only the implementation report.

## 11. Blockers

No blockers.

## 12. Non-Blocking Follow-Ups

- Add direct governed-write helper tests for artifact/run identity mismatch and non-terminal run behavior.
- Add executor artifact tests for failed and canceled terminal runs.
- Add approval-linkage failure-after-integrity-success coverage.
- Add permissive missing-citation artifact-write coverage.
- Add explicit artifact-store-empty assertions for existing non-artifact executor paths.
- Carry forward store-backed approval linkage review follow-ups:
  - explicit returned-record ID equality guard/test;
  - `ExplicitIdsAndAllRecordsForRun` overlap/count test;
  - list failure mapping test;
  - store-backed run identity mismatch test;
  - documented unique loaded-record count semantics.

## 13. Recommended Next Phase

Recommended next phase: **deterministic hook checkpoint enforcement expansion**.

This phase proves the first explicit artifact-backed governance composition around reports, `SideEffect` records, and approval linkage. The next useful step should continue runtime composition of existing primitives rather than introduce another new primitive family. Hook checkpoints are the best next target because they move agent-harness governance from orientation/prose and helper-level vocabulary toward deterministic executor-enforced checkpoints.

The immediate runtime gap is hook enforcement, not write readiness. Workflow OS already has hook vocabulary, invocation helpers, failed-closed paths, disclosure semantics, and some executor integration. The next phase should connect those primitives into deterministic executor checkpoints that enforce something real while preserving local, explicit, non-write behavior.

Write-capable adapter readiness should remain downstream, after deterministic hook checkpoint enforcement and high-assurance approval controls are planned, implemented, and reviewed. Runtime side-effect execution and provider mutation should remain unimplemented.

## 14. Validation

Commands run:

- `cargo fmt --all --check` - passed
- `cargo clippy --workspace --all-targets -- -D warnings` - passed
- `cargo test --workspace` - passed
- `npm run check:docs` - passed
