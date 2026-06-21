# Executor Report Artifact SideEffect Gates Report

Report date: 2026-06-21

## 1. Executive Summary

This phase adds an explicit executor-adjacent path that can execute a local workflow, generate an in-memory `WorkReport`, validate the resulting report artifact against `SideEffect` referential integrity and approval-linkage gates, and then write the artifact through a caller-supplied `WorkReportArtifactStore`.

The change closes a runtime composition gap between report artifacts, `SideEffect` records, and approval linkage without making report artifact writing automatic.

## 2. Scope Completed

- Added a governed report artifact write helper.
- Added executor-adjacent request/result types for explicit artifact-writing report execution.
- Added an explicit executor helper that preserves existing execution/report semantics.
- Added focused executor tests for successful writes, side-effect-gated writes, missing side-effect failures, and duplicate artifact writes.
- Added the store-backed approval linkage review.

## 3. Scope Explicitly Not Completed

- No automatic report artifact writing from existing executor methods.
- No runtime side-effect execution.
- No provider writes or write-capable adapters.
- No workflow schema changes.
- No CLI rendering or export behavior.
- No examples or hosted runtime behavior.
- No reasoning lineage.
- No approval evidence attachment.
- No release posture change.

## 4. API Summary

New report-layer helper:

```rust
write_work_report_artifact_with_side_effect_integrity_and_approval_linkage(...)
```

New executor-layer helper:

```rust
execute_with_report_artifact_and_side_effect_gates(...)
```

The executor helper accepts explicit execution inputs, report inputs, optional `SideEffect` discovery policy, and explicit artifact gate policy. It returns a result that separates:

- workflow run;
- generated `WorkReport`;
- report-generation error;
- written artifact;
- artifact-write error;
- `SideEffect` integrity counts;
- approval-linkage counts.

## 5. Runtime Semantics

Workflow execution failures before a run exists still return `Err`.

If a run exists, report-generation failure does not change the workflow result. If report generation succeeds but artifact gates or artifact write fail, the run and report are preserved and the artifact error is returned separately.

The helper does not mutate the run, append workflow events, emit audit or observability events, execute side effects, call providers, write side-effect records, or repair citations.

## 6. SideEffect And Approval Gates

The artifact write helper:

1. validates the report artifact;
2. verifies the artifact matches the supplied terminal run identity;
3. validates cited `SideEffect` IDs against the supplied `SideEffectRecordStore`;
4. validates approval linkage for cited `SideEffect` records when citations exist;
5. writes the artifact only after those gates pass.

Reports without `SideEffect` citations still validate and write successfully. Approval linkage is skipped for that no-citation case.

## 7. Privacy And Redaction

Errors and `Debug` output remain bounded and non-leaking. The new result types expose counts and stable error codes, not report text, run IDs, side-effect IDs, approval IDs, target references, provider payloads, command output, raw spec contents, paths, or token-like values.

The artifact serialization behavior remains the existing `WorkReportArtifactRecord` behavior.

## 8. Test Coverage

Focused tests cover:

- explicit executor artifact writing after successful report generation;
- `SideEffect` discovery plus artifact write with a matching stored record;
- missing cited `SideEffect` record failing before artifact write;
- duplicate artifact write preserving the existing artifact and event history;
- no workflow event mutation from artifact paths;
- no artifact writes from existing report-only executor paths.

Existing focused executor tests passed.

## 9. Commands Run And Results

- `cargo fmt --all` passed.
- `cargo test -p workflow-core --test local_executor` passed.
- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `npm run check:docs` passed.

## 10. Remaining Known Limitations

- Existing executor methods still do not write artifacts automatically.
- Approval-resume and cancellation report/artifact paths remain future work.
- `SideEffectAttempted`, `SideEffectCompleted`, and `SideEffectFailed` executor behavior remains unsupported.
- Write-capable adapters remain unsupported.
- High-assurance approval controls remain future work.
- CLI rendering/export and schema exposure remain deferred.

## 11. Recommended Next Phase

Recommended next phase: **executor report artifact SideEffect gates review**.

The phase should be reviewed before any broader runtime side-effect execution, provider writes, automatic artifact writing, or write-capable adapter readiness work.
