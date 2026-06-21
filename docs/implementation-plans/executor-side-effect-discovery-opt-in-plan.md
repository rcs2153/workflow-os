# Executor SideEffect Discovery Opt-In Plan

Status: Implemented in [Executor SideEffect Discovery Opt-In Report](../concepts/EXECUTOR_SIDE_EFFECT_DISCOVERY_OPT_IN_REPORT.md) and accepted with non-blocking follow-ups in [Executor SideEffect Discovery Opt-In Review](../concepts/EXECUTOR_SIDE_EFFECT_DISCOVERY_OPT_IN_REVIEW.md). Planning was accepted in [Executor SideEffect Discovery Opt-In Plan Review](../concepts/EXECUTOR_SIDE_EFFECT_DISCOVERY_OPT_IN_PLAN_REVIEW.md). The explicit WorkReport-side SideEffect discovery helper is implemented and accepted in [WorkReport SideEffect Discovery Integration Review](../concepts/WORK_REPORT_SIDE_EFFECT_DISCOVERY_INTEGRATION_REVIEW.md). This plan defines the conservative executor-facing step: an additive local executor helper that can opt into SideEffect discovery for report-bearing execution without changing existing executor semantics.

This phase implements explicit executor opt-in discovery only. It does not implement automatic report discovery, report artifacts, persistence changes, CLI behavior, schemas, examples, hosted behavior, runtime side-effect execution, attempted/completed/failed executor behavior, write-capable adapters, provider mutation, EvidenceReference side-effect attachment, approval-side-effect linkage enforcement, reasoning lineage, or release posture changes.

## 1. Executive Summary

Workflow OS now supports:

- explicit SideEffect IDs in `LocalExecutionReportInputs`;
- explicit proposed/denied/skipped SideEffect workflow events in local execution;
- persisted `SideEffectRecord` values through `SideEffectRecordStore`;
- explicit in-memory SideEffect discovery;
- explicit store-backed SideEffect discovery;
- explicit WorkReport-side SideEffect discovery before terminal report construction.

The next question is how local executor report-bearing execution should opt into that discovery.

The answer should not be automatic discovery inside `LocalExecutor::execute_with_report(...)`. Existing executor behavior is already reviewed: callers supply stable SideEffect IDs explicitly, and the executor forwards those IDs into the report. That path should remain unchanged.

This plan recommends a new additive executor-adjacent API or helper that accepts explicit discovery policy plus a `SideEffectRecordStore` boundary, executes the existing local workflow, and then generates the in-memory report through the reviewed WorkReport-side discovery helper.

## 2. Goals

- Add a future executor-facing opt-in path for SideEffect discovery.
- Preserve `LocalExecutor::execute(...)` exactly.
- Preserve `LocalExecutor::execute_with_report(...)` exactly unless a later review explicitly approves a small refactor.
- Keep discovery local, deterministic, explicit, and in-memory.
- Use `SideEffectRecordStore` only when the caller opts in.
- Reuse `generate_terminal_local_work_report_with_side_effect_discovery(...)`.
- Preserve workflow execution pass/fail semantics.
- Keep report-generation errors separate from workflow execution errors.
- Cite stable `SideEffectId` values only.
- Avoid recreating `EvidenceReference` values.
- Avoid copying SideEffect record payloads.
- Keep all errors stable and non-leaking.
- Prepare for future report artifact integrity planning without implementing it now.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- automatic SideEffect discovery in `execute_with_report(...)`;
- automatic SideEffect discovery for every report;
- adding `SideEffectRecordStore` as a supertrait of `StateBackend`;
- changing existing executor return types;
- changing existing workflow pass/fail behavior;
- report artifact creation or persistence;
- artifact referential integrity checks;
- EvidenceReference side-effect attachment;
- approval-side-effect linkage enforcement;
- runtime side-effect execution;
- attempted/completed/failed executor behavior;
- write-capable GitHub, Jira, CI, HTTP, local filesystem, or generic adapters;
- provider mutation;
- creating, updating, repairing, or lifecycle-transitioning SideEffect records;
- workflow-declared SideEffect discovery config;
- runtime default discovery config;
- CLI commands or rendering;
- workflow schema fields;
- examples;
- hosted or distributed runtime behavior;
- reasoning lineage;
- rollback or compensation behavior;
- Level 3/4 autonomy enablement;
- release posture changes.

## 4. Current Executor Baseline

Current local executor report-bearing behavior:

- `LocalExecutor::execute(...) -> Result<WorkflowRun, WorkflowOsError>`;
- `LocalExecutor::execute_with_report(...) -> Result<LocalExecutionWithReportResult, WorkflowOsError>`;
- `LocalExecutionReportInputs::side_effect_ids` accepts explicit stable SideEffect IDs;
- `terminal_report_input_for_run(...)` forwards explicit SideEffect IDs;
- `execute_with_report(...)` calls `expose_terminal_local_work_report_result(...)`;
- report-generation failure returns the run plus `report_generation_error`;
- workflow execution failure before a run exists returns the execution error unchanged.

Current local executor SideEffect event behavior:

- explicit `LocalExecutionSideEffectEventInput` can append proposed/denied/skipped SideEffect workflow events before local skill invocation;
- attempted/completed/failed event inputs fail closed;
- SideEffect events preserve workflow run event semantics and do not execute provider writes.

Current store baseline:

- `SideEffectRecordStore` is separate from `StateBackend`;
- `LocalStateBackend` implements `SideEffectRecordStore`;
- store-backed discovery reads through `SideEffectRecordStore::list_side_effect_records_for_workflow_run(...)`;
- normal run rehydration does not read SideEffect records.

## 5. Recommended API Shape

The first implementation should add an explicit, additive executor-side API. The exact names may vary, but the recommended shape is:

```rust
pub struct LocalExecutionWithReportAndSideEffectDiscoveryRequest {
    pub execution: LocalExecutionRequest,
    pub report: LocalExecutionReportInputs,
    pub side_effect_discovery: LocalExecutionSideEffectDiscoveryInputs,
}

pub struct LocalExecutionSideEffectDiscoveryInputs {
    pub include_workflow_events: bool,
    pub include_store_records: bool,
    pub require_records: bool,
}

pub fn execute_with_report_and_side_effect_discovery(
    executor: &LocalExecutor<'_, impl StateBackend>,
    store: &impl SideEffectRecordStore,
    request: &LocalExecutionWithReportAndSideEffectDiscoveryRequest,
) -> Result<LocalExecutionWithReportResult, WorkflowOsError>
```

An inherent `LocalExecutor` method may be acceptable only if Rust bounds stay narrow and do not disturb existing methods. For example:

```rust
impl<'a, B> LocalExecutor<'a, B>
where
    B: StateBackend + SideEffectRecordStore,
{
    pub fn execute_with_report_and_side_effect_discovery(...)
}
```

However, the safer first implementation is likely a free helper function that takes:

- `&LocalExecutor`;
- `&impl SideEffectRecordStore`;
- explicit request inputs.

That avoids broadening the existing `LocalExecutor` implementation surface and makes the opt-in boundary obvious.

## 6. Why Not Modify `execute_with_report(...)`

`execute_with_report(...)` is already reviewed and tested as explicit-reference propagation. Changing it to discover records would alter caller expectations and make report content depend on local store contents.

Reasons to leave it unchanged:

- discovery would become ambient for existing callers;
- report output would depend on store state even when not requested;
- report-generation failure modes would expand for existing users;
- `SideEffectRecordStore` would pressure the existing executor generic bounds;
- automatic discovery could be mistaken for proof of side-effect completeness;
- absence of records must not imply that no side effects happened.

The new path should advertise discovery in its name and request type.

## 7. Execution Flow

Recommended future flow:

1. Call existing `LocalExecutor::execute(&request.execution)`.
2. If execution returns `Err`, return that error unchanged.
3. If execution returns a non-terminal run, return `LocalExecutionWithReportResult` with:
   - the run;
   - no report;
   - stable non-leaking report error from the existing report generation path or discovery policy.
4. If execution returns a terminal run, build `TerminalLocalWorkReportInput` from `LocalExecutionReportInputs`.
5. Call `generate_terminal_local_work_report_with_side_effect_discovery(...)`.
6. If report generation succeeds, return run plus report.
7. If discovery or report generation fails, return run plus `report_generation_error`.

Workflow execution result semantics must not change based on report discovery success or failure.

## 8. Discovery Policy

The executor opt-in request should mirror the reviewed WorkReport-side discovery policy:

- `include_workflow_events`;
- `include_store_records`;
- `require_records`.

Rules:

- at least one discovery source must be enabled;
- workflow events must be the events already present on the returned run;
- store records must be loaded only through the supplied `SideEffectRecordStore`;
- supported event discovery remains proposed/denied/skipped only;
- attempted/completed/failed remain unsupported and uncited until runtime execution semantics are planned;
- `require_records` should fail report generation, not workflow execution, when discovered IDs have no matching records;
- missing optional records should not fabricate citations.

## 9. Contract And Store Boundary

The implementation should not make `StateBackend: SideEffectRecordStore` globally.

Acceptable options:

1. Free helper function with explicit `store: &impl SideEffectRecordStore`.
2. Separate inherent implementation for `LocalExecutor<B>` where `B: StateBackend + SideEffectRecordStore`.
3. New narrow wrapper type that owns references to executor and store.

Preferred first implementation: **free helper function**.

Rationale:

- no existing public method changes;
- no new trait inheritance;
- explicit store dependency;
- easier to test with existing `LocalStateBackend`;
- keeps future executor API hardening separate.

## 10. Result Semantics

The result should reuse `LocalExecutionWithReportResult`.

Rules:

- successful execution plus successful discovery/report returns `Some(work_report)`;
- successful execution plus discovery/report failure returns `None` report and `Some(report_generation_error)`;
- execution failure before a run exists returns `Err` unchanged;
- non-terminal run returns no report and a stable report-generation error;
- report-generation failure must not append events, emit audit records, emit observability records, persist artifacts, or mutate the run.

No new report result type should be added unless the implementation cannot express discovery error semantics with the existing result.

## 11. Error Handling

Errors must remain stable and non-leaking.

Expected codes:

- `work_report_generation.status.not_terminal` for non-terminal report generation;
- `work_report_generation.side_effect_discovery.source_required` when discovery config has no source;
- existing `side_effect_discovery.*` codes for identity mismatch, missing required records, corrupt records, and store read failures;
- existing `work_report.*` or related validation codes for report construction failures.

Errors must not include:

- SideEffect IDs;
- workflow IDs;
- run IDs;
- store paths;
- target references;
- record JSON;
- provider payloads;
- command output;
- spec contents;
- tokens, credentials, private keys, or secret-like values.

## 12. Runtime And Mutation Boundary

The future executor opt-in path must not:

- mutate `WorkflowRun`;
- mutate `WorkflowRunSnapshot`;
- append workflow events after terminal state;
- append audit events for report discovery;
- emit observability events for report discovery;
- create, update, repair, or transition SideEffect records;
- execute side effects;
- call adapters or providers;
- write report artifacts;
- create files;
- emit CLI output;
- change workflow pass/fail semantics;
- change idempotency replay behavior.

The only extra operation beyond existing execution is optional read-only SideEffect discovery through the supplied store.

## 13. Privacy And Redaction

The implementation must remain reference-only.

The executor opt-in path must not copy into reports, errors, Debug output, or serialization:

- SideEffect target references;
- SideEffect summaries;
- SideEffect reason codes;
- authority context;
- lifecycle payload details;
- idempotency details;
- raw record JSON;
- provider payloads;
- command output;
- CI logs;
- Jira issue/comment bodies;
- GitHub file contents;
- spec contents;
- parser payloads;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- local filesystem paths.

Serialized reports may include valid stable `SideEffectId` citations, matching existing WorkReport citation behavior.

## 14. Test Plan

Future implementation tests should cover:

- existing `execute_with_report(...)` remains explicit-ID-only;
- existing `execute(...)` remains unchanged;
- new opt-in path returns completed run plus report with store-discovered SideEffect citation;
- new opt-in path returns failed run plus report with store-discovered SideEffect citation;
- canceled terminal run is supported if existing executor fixtures make it convenient;
- non-terminal run returns run plus non-leaking report-generation error and no report;
- no discovery source returns run plus `work_report_generation.side_effect_discovery.source_required`;
- missing required records returns run plus `side_effect_discovery.record_missing`;
- corrupt store record returns run plus non-leaking report-generation error;
- identity mismatch returns run plus non-leaking report-generation error;
- explicit SideEffect IDs and discovered IDs merge deterministically without duplicates;
- proposed/denied/skipped workflow events can be discovered when event discovery is enabled;
- attempted/completed/failed events remain unsupported and uncited;
- generated SideEffect citations appear only in the side-effects section;
- generated reports do not copy SideEffect target references, summaries, reason codes, authority context, idempotency details, raw record JSON, provider payloads, command output, or spec contents;
- discovery failure does not mutate the run, snapshot, event history, or state backend;
- discovery failure does not append post-terminal events;
- helper can be used without writing report artifacts;
- no filesystem artifacts are created;
- no CLI output is emitted;
- Debug output for new request/config/result paths does not leak IDs, paths, raw payloads, or secret-like values;
- existing WorkReport, SideEffect, SideEffect discovery, SideEffect record store, executor, runtime, adapter, evidence, hook, local-check, and docs tests continue to pass.

## 15. Documentation Updates For Implementation

Future implementation should update:

- this plan;
- `ROADMAP.md`;
- [WorkReport SideEffect Discovery Integration Plan](work-report-side-effect-discovery-integration-plan.md);
- [WorkReportContract Planning Document](work-report-contract-plan.md), if needed;
- an end-of-phase report under `docs/concepts/`.

Docs must say:

- executor SideEffect discovery opt-in path is implemented;
- existing `execute_with_report(...)` remains unchanged unless explicitly scoped otherwise;
- automatic executor discovery is not implemented;
- report artifacts are not written;
- persistence behavior is not broadened;
- CLI rendering is not implemented;
- workflow schemas are not changed;
- examples are not updated;
- runtime side-effect execution is not implemented;
- writes remain unsupported.

## 16. Proposed Implementation Sequence

Recommended small implementation sequence:

1. Add executor opt-in request/config type.
2. Add free helper function using an explicit `SideEffectRecordStore`.
3. Reuse existing `LocalExecutor::execute(...)`.
4. Reuse `terminal_report_input_for_run(...)` or a small shared extraction if visibility requires it.
5. Call `generate_terminal_local_work_report_with_side_effect_discovery(...)`.
6. Preserve `LocalExecutionWithReportResult` semantics.
7. Add focused tests.
8. Update docs and create implementation report.
9. Review before any automatic executor discovery, artifact integrity, or write-capable adapter planning.

## 17. Open Questions

- Should the first implementation be a free helper function or an inherent `LocalExecutor` method with extra bounds?
- Should the opt-in request wrap `LocalExecutionWithReportRequest` or duplicate `execution` and `report` fields for clarity?
- Should event discovery default to false even in the opt-in path?
- Should store discovery default to true in the opt-in path, or require explicit true?
- Should `require_records` be allowed when `include_store_records` is false?
- Should discovery metadata counts ever surface in WorkReport section text?
- Should report artifact storage later validate cited SideEffect IDs against the store?
- Should approval-side-effect linkage happen before or after executor discovery opt-in?
- Should attempted/completed/failed lifecycle discovery wait until write-capable adapter attempts exist?

## 18. Final Recommendation

Implemented phase: **executor SideEffect discovery opt-in helper, in-memory only**.

Prefer a free helper function that accepts an explicit `SideEffectRecordStore` rather than changing `LocalExecutor::execute_with_report(...)`. The implementation must still not add automatic discovery, report artifact writing, runtime side-effect execution, attempted/completed/failed executor behavior, write-capable adapters, provider mutation, schemas, CLI behavior, examples, hosted behavior, reasoning lineage, or release posture changes.

The implementation has been accepted with non-blocking follow-ups in [Executor SideEffect Discovery Opt-In Review](../concepts/EXECUTOR_SIDE_EFFECT_DISCOVERY_OPT_IN_REVIEW.md). Report artifact referential integrity planning for cited SideEffect IDs is documented in [Report Artifact SideEffect Referential Integrity Plan](report-artifact-side-effect-referential-integrity-plan.md), and the validation-only helper is implemented in [Report Artifact SideEffect Referential Integrity Report](../concepts/REPORT_ARTIFACT_SIDE_EFFECT_REFERENTIAL_INTEGRITY_REPORT.md) and accepted with non-blocking follow-ups in [Report Artifact SideEffect Referential Integrity Review](../concepts/REPORT_ARTIFACT_SIDE_EFFECT_REFERENTIAL_INTEGRITY_REVIEW.md). Approval-side-effect linkage planning is documented in [Approval SideEffect Linkage Plan](approval-side-effect-linkage-plan.md), and the validation-only helper is implemented in [SideEffect Approval Linkage Report](../concepts/SIDE_EFFECT_APPROVAL_LINKAGE_REPORT.md). The recommended next phase is SideEffect approval linkage validation helper review.
