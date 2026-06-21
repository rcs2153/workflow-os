# Executor SideEffect Discovery Opt-In Report

## 1. Executive Summary

The executor SideEffect discovery opt-in phase is implemented.

This phase adds an explicit, in-memory executor-adjacent helper that runs a local workflow, then opts into SideEffect discovery for terminal WorkReport generation through a caller-supplied `SideEffectRecordStore`. Existing `LocalExecutor::execute(...)` and `LocalExecutor::execute_with_report(...)` behavior remains unchanged.

## 2. Scope Completed

Implemented:

- `LocalExecutionSideEffectDiscoveryInputs`;
- `LocalExecutionWithReportAndSideEffectDiscoveryRequest`;
- `execute_with_report_and_side_effect_discovery(...)`;
- export of the new helper and request/config types from `workflow-core`;
- reuse of `LocalExecutor::execute(...)`;
- reuse of the accepted WorkReport-side `generate_terminal_local_work_report_with_side_effect_discovery(...)` helper;
- reuse of `LocalExecutionWithReportResult` semantics;
- focused local executor tests;
- documentation and roadmap updates.

## 3. Scope Explicitly Not Completed

This phase does not implement:

- automatic SideEffect discovery in `LocalExecutor::execute_with_report(...)`;
- automatic SideEffect discovery for every report;
- report artifact writing;
- persistence changes;
- workflow schema fields;
- CLI commands or rendering;
- example updates;
- EvidenceReference side-effect attachment;
- approval-side-effect linkage enforcement;
- runtime side-effect execution;
- attempted/completed/failed executor behavior;
- write-capable adapters;
- provider mutation;
- hosted/distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 4. Helper API Summary

The new API is:

```rust
pub struct LocalExecutionSideEffectDiscoveryInputs {
    pub include_workflow_events: bool,
    pub include_store_records: bool,
    pub require_records: bool,
}

pub struct LocalExecutionWithReportAndSideEffectDiscoveryRequest {
    pub execution: LocalExecutionRequest,
    pub report: LocalExecutionReportInputs,
    pub side_effect_discovery: LocalExecutionSideEffectDiscoveryInputs,
}

pub fn execute_with_report_and_side_effect_discovery<B>(
    executor: &LocalExecutor<'_, B>,
    store: &impl SideEffectRecordStore,
    request: &LocalExecutionWithReportAndSideEffectDiscoveryRequest,
) -> Result<LocalExecutionWithReportResult, WorkflowOsError>
where
    B: StateBackend;
```

The helper is intentionally a free function. That keeps the `SideEffectRecordStore` dependency explicit and avoids changing `LocalExecutor` bounds or existing executor methods.

## 5. Execution And Result Semantics

The helper:

1. calls `LocalExecutor::execute(&request.execution)`;
2. returns execution errors unchanged when no run exists;
3. returns a non-terminal run with no report and stable report-generation error `work_report_generation.status.not_terminal`;
4. runs the existing explicit `BeforeReport` hook behavior for terminal runs, matching `execute_with_report(...)`;
5. builds terminal report input from the run and explicit report inputs;
6. calls `generate_terminal_local_work_report_with_side_effect_discovery(...)`;
7. returns run plus report on success;
8. returns run plus report-generation error on discovery/report failure.

Workflow pass/fail semantics are unchanged.

## 6. Discovery Boundary Summary

Discovery is explicitly opt-in.

Rules:

- event discovery uses only events already present on the returned run;
- store discovery reads only through the caller-supplied `SideEffectRecordStore`;
- supported event-derived lifecycle states remain proposed, denied, and skipped;
- attempted, completed, and failed lifecycle behavior remains unsupported and uncited;
- missing records fail report generation only when `require_records` is true;
- discovery does not create or mutate records.

## 7. Runtime And Mutation Boundary

The helper does not:

- mutate `WorkflowRun`;
- mutate `WorkflowRunSnapshot`;
- append post-terminal events;
- append audit events;
- emit observability events;
- create, repair, update, or lifecycle-transition SideEffect records;
- execute side effects;
- call adapters or providers;
- write report artifacts;
- create files;
- emit CLI output.

## 8. Privacy And Redaction Summary

The helper remains reference-only.

It does not copy into reports, errors, Debug output, or serialization:

- SideEffect target references;
- SideEffect summaries;
- reason codes;
- authority context;
- lifecycle payload details;
- idempotency details;
- raw record JSON;
- provider payloads;
- command output;
- CI logs;
- Jira/GitHub bodies or file contents;
- spec contents;
- parser payloads;
- environment variable values;
- credentials;
- token-like values;
- local filesystem paths.

Reports may serialize stable `SideEffectId` citations, matching existing WorkReport citation behavior.

## 9. Test Coverage Summary

Added focused local executor tests covering:

- completed terminal run with store-discovered SideEffect citation;
- explicit store boundary using a `SideEffectRecordStore` distinct from the executor backend;
- no-source discovery policy returning a report-generation error after execution;
- non-terminal run returning `work_report_generation.status.not_terminal` without discovery;
- no report artifact writes;
- event log preservation;
- Debug non-leakage for SideEffect IDs and target references;
- serialized report non-leakage for target references and summaries.

Existing `execute_with_report(...)` tests continue to cover explicit SideEffect ID propagation without discovery.

## 10. Commands Run And Results

- `cargo fmt --all` - passed.
- `cargo fmt --all --check` - passed.
- `cargo test -p workflow-core --test local_executor execute_with_report_and_side_effect_discovery` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

## 11. Remaining Known Limitations

- Automatic executor SideEffect discovery is not implemented.
- `LocalExecutor::execute_with_report(...)` remains explicit-ID-only.
- Report artifact referential integrity is not implemented.
- Runtime side-effect execution is not implemented.
- Attempted/completed/failed lifecycle events remain unsupported in executor behavior.
- Write-capable adapters remain unsupported.
- EvidenceReference side-effect attachment is not implemented.
- Approval-side-effect linkage enforcement is not implemented.

## 12. Recommended Next Phase

Recommended next phase: **executor SideEffect discovery opt-in helper review**.

After review, the next roadmap decision should stay before write-capable adapters and runtime side-effect execution. Good candidates are report artifact referential integrity planning for cited SideEffect IDs or approval-side-effect linkage planning, depending on maintainer priority.
