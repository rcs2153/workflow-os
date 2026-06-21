# Executor SideEffect Discovery Opt-In Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation adds the intended explicit, in-memory executor-adjacent SideEffect discovery path without changing existing executor behavior. It preserves `LocalExecutor::execute(...)`, preserves `LocalExecutor::execute_with_report(...)`, keeps discovery opt-in through a separate helper, and keeps SideEffect records as read-only report-generation inputs rather than runtime side-effect execution.

Recommended next phase: report artifact referential integrity planning for cited SideEffect IDs.

## 2. Scope Verification

The phase stayed within the approved opt-in helper scope.

Implemented scope:

- `LocalExecutionSideEffectDiscoveryInputs`;
- `LocalExecutionWithReportAndSideEffectDiscoveryRequest`;
- `execute_with_report_and_side_effect_discovery(...)`;
- explicit `SideEffectRecordStore` parameter;
- reuse of existing local execution;
- reuse of the accepted WorkReport-side SideEffect discovery helper;
- reuse of `LocalExecutionWithReportResult`;
- focused tests and documentation updates.

No accidental implementation found for:

- automatic SideEffect discovery in `LocalExecutor::execute_with_report(...)`;
- automatic SideEffect discovery for every report;
- report artifact writing;
- persistence contract broadening;
- workflow schema changes;
- CLI behavior;
- example updates;
- EvidenceReference side-effect attachment;
- approval-side-effect linkage enforcement;
- runtime side-effect execution;
- attempted/completed/failed executor side-effect behavior;
- write-capable adapters;
- provider mutation;
- hosted or distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 3. API Boundary Assessment

The API boundary is appropriately narrow.

The implementation chose the preferred free helper:

```rust
pub fn execute_with_report_and_side_effect_discovery<B>(
    executor: &LocalExecutor<'_, B>,
    store: &impl SideEffectRecordStore,
    request: &LocalExecutionWithReportAndSideEffectDiscoveryRequest,
) -> Result<LocalExecutionWithReportResult, WorkflowOsError>
where
    B: StateBackend;
```

This preserves the existing executor method surface while making discovery explicit at the call site. The helper does not add `SideEffectRecordStore` as a `StateBackend` supertrait, does not read hidden global state, and does not make store contents ambient for existing report-bearing execution.

The request type is also narrow: it wraps the existing execution request, existing report inputs, and a small boolean discovery policy. Debug output for the request redacts execution identity and report internals while exposing only safe discovery booleans.

Non-blocking API follow-up: the helper currently accepts `LocalExecutor<'_, B>` with default sink generics. If custom audit, observability, or logger sink callers need this path later, the helper can be generalized over the executor sink type parameters without changing semantics.

## 4. Executor Semantics Assessment

Executor semantics are preserved.

Verified behavior:

- `LocalExecutor::execute(...)` remains unchanged and still returns `Result<WorkflowRun, WorkflowOsError>`.
- `LocalExecutor::execute_with_report(...)` remains unchanged and explicit-ID-only for SideEffect report citations.
- execution errors before a run exists are returned unchanged;
- terminal runs can return a generated report;
- discovery/report-generation failures after a run exists return the run plus `report_generation_error`;
- non-terminal runs return the run, no report, and stable error code `work_report_generation.status.not_terminal`;
- report-generation failure does not retroactively fail or change the workflow run result.

The helper also reuses the existing in-memory `BeforeReport` behavior for terminal runs, keeping report-path hook propagation consistent with `execute_with_report(...)`.

Non-blocking maintainability follow-up: the `BeforeReport` hook handling is now duplicated between `execute_with_report(...)` and the opt-in discovery helper. This is acceptable for the small first slice, but a tiny internal refactor may be useful if another report-bearing executor path is added.

## 5. Discovery Policy Assessment

The discovery policy matches the accepted WorkReport-side boundary:

- `include_workflow_events`;
- `include_store_records`;
- `require_records`.

Discovery remains explicit and deterministic:

- workflow-event discovery uses only events already present on the returned run;
- store discovery reads only through the caller-supplied `SideEffectRecordStore`;
- the helper can use a store distinct from the executor backend;
- proposed, denied, and skipped lifecycle states remain the supported discovery vocabulary;
- attempted, completed, and failed lifecycle behavior remains deferred;
- missing records affect report generation only when the caller requests `require_records`;
- no fake IDs or new `EvidenceReference` values are created.

The implementation correctly treats missing discovery sources as report-generation failure, not workflow execution failure.

## 6. Store And Persistence Boundary Assessment

The store boundary is clean.

The implementation does not make `SideEffectRecordStore` mandatory for all state backends and does not alter normal run rehydration. Store-backed discovery is a read-only dependency passed into the helper explicitly. Tests prove the discovery store can be separate from the executor backend, which protects the intended boundary.

No persistence behavior was broadened:

- no SideEffect records are created;
- no SideEffect records are repaired or lifecycle-transitioned;
- no WorkReport artifacts are written;
- no filesystem report outputs are created;
- no normal executor path begins reading SideEffect records automatically.

## 7. Report And Citation Assessment

Report construction remains centralized through existing WorkReport constructors.

The helper delegates to `generate_terminal_local_work_report_with_side_effect_discovery(...)`, which delegates final report construction to `generate_terminal_local_work_report(...)`. That preserves:

- existing WorkReport validation;
- existing WorkReport citation construction;
- existing redaction metadata handling;
- existing terminal report section population;
- existing SideEffect citation target vocabulary.

Generated reports cite stable `SideEffectId` values only. SideEffect target references, summaries, reason details, authority context, idempotency details, raw records, provider payloads, command output, and file contents are not copied into report sections or serialization.

## 8. Runtime And Mutation Boundary Assessment

The helper does not mutate runtime state.

Verified boundaries:

- no `WorkflowRun` mutation;
- no `WorkflowRunSnapshot` mutation;
- no post-terminal workflow events;
- no audit events for report discovery;
- no observability events for report discovery;
- no adapter or provider calls;
- no side-effect execution;
- no StateBackend writes beyond the existing execution path;
- no report artifact writes;
- no CLI output.

This preserves the v0 local executor event model: terminal runs remain terminal, and all post-run report behavior stays out of the workflow event stream.

## 9. Error Handling Assessment

Error handling is conservative and non-leaking.

Verified:

- execution errors before a run exists return unchanged;
- non-terminal report generation uses `work_report_generation.status.not_terminal`;
- no-source discovery uses `work_report_generation.side_effect_discovery.source_required`;
- lower-level discovery and WorkReport validation errors remain structured and stable;
- no partial WorkReport is returned after discovery/report failure;
- errors do not include SideEffect IDs, target references, store paths, raw records, provider output, command output, tokens, credentials, or secret-like values.

The implementation correctly treats discovery errors as report-generation errors after execution rather than misleading user project diagnostics.

## 10. Privacy And Redaction Assessment

The phase remains reference-only.

Verified:

- Debug output for the new request/result path does not leak SideEffect IDs or target references;
- serialized reports include stable SideEffect IDs but not SideEffect target references or summaries;
- SideEffect authority context, lifecycle payload details, idempotency details, reason codes, raw record JSON, provider payloads, command output, CI logs, Jira/GitHub bodies, spec contents, parser payloads, environment values, credentials, and token-like values are not copied by the helper;
- existing WorkReport redaction behavior remains the report construction gate.

No redaction blocker was found.

## 11. Test Quality Assessment

The focused tests cover the core first-slice behaviors:

- terminal completed execution can generate a report with store-discovered SideEffect citation;
- discovery can use a store distinct from the executor backend;
- missing discovery source returns a report-generation error after execution;
- non-terminal execution short-circuits discovery and returns `work_report_generation.status.not_terminal`;
- generated reports cite SideEffect IDs without copying record targets or summaries;
- execution backend event history matches the returned run;
- report artifacts are not written;
- Debug output avoids SideEffect IDs and target references for the new request/result path.

Existing surrounding tests continue to cover:

- explicit SideEffect ID forwarding through `execute_with_report(...)`;
- WorkReport-side discovery behavior;
- store-backed discovery behavior;
- SideEffect record validation;
- executor, runtime, EvidenceReference, WorkReport, hook, local-check, adapter, and docs regressions through workspace validation.

Non-blocking test gaps:

- add executor-level failed terminal discovery coverage;
- add executor-level canceled terminal discovery coverage if fixture construction remains practical;
- add executor-level workflow-event discovery coverage, not only store discovery;
- add executor-level missing-required-record coverage;
- add executor-level corrupt-record or identity-mismatch coverage if it can stay small;
- add a direct assertion that the opt-in helper emits no report-specific audit or observability records when custom sinks are supported by the helper signature.

These are not blockers because the helper delegates discovery and report construction to reviewed lower-level helpers, and the first executor-facing store boundary is directly tested.

## 12. Documentation Review

Documentation is honest about implemented and deferred behavior.

Verified docs state:

- executor SideEffect discovery opt-in is implemented;
- existing `execute_with_report(...)` remains unchanged;
- automatic SideEffect discovery is not implemented;
- report artifacts are not written;
- persistence behavior is not broadened;
- CLI rendering is not implemented;
- workflow schemas are not changed;
- examples are not updated;
- EvidenceReference side-effect attachment is not implemented;
- approval-side-effect linkage is not implemented;
- runtime side-effect execution is not implemented;
- attempted/completed/failed executor side-effect behavior is not implemented;
- writes and provider mutations remain unsupported;
- hosted behavior, reasoning lineage, and release posture changes remain out of scope.

Non-blocking docs follow-up: `docs/runtime/local-executor.md` still describes `execute_with_report(...)` but does not yet mention the new free opt-in helper. That omission does not overclaim behavior, but the runtime docs should be updated when this API is promoted as part of the public executor surface.

## 13. Blockers

No blockers.

## 14. Non-Blocking Follow-Ups

- Add failed terminal executor-level discovery coverage.
- Add canceled terminal executor-level discovery coverage if practical.
- Add executor-level event-discovery coverage.
- Add executor-level missing-required-record coverage.
- Consider generalizing the free helper over custom executor sink generics.
- Consider factoring shared `BeforeReport` hook handling if another report-bearing executor path is added.
- Update `docs/runtime/local-executor.md` to mention the opt-in helper when the public API surface is ready for broader operator documentation.

## 15. Recommended Next Phase

Recommended next phase: **report artifact referential integrity planning for cited SideEffect IDs**.

The executor can now explicitly discover SideEffect IDs and include them in in-memory WorkReports. Before moving toward automatic discovery, runtime side-effect execution, or write-capable adapters, the roadmap should decide how report artifacts will preserve or validate references to SideEffect IDs when reports are explicitly stored.

This next phase should still not implement automatic executor discovery, runtime side-effect execution, attempted/completed/failed lifecycle execution behavior, EvidenceReference side-effect attachment, approval-side-effect enforcement, writes, provider mutation, schemas, CLI behavior, examples, hosted behavior, reasoning lineage, or release posture changes.

## 16. Validation

Review validation:

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
