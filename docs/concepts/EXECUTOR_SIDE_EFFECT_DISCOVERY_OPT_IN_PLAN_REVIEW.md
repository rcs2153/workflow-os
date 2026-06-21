# Executor SideEffect Discovery Opt-In Plan Review

## 1. Executive Verdict

Plan accepted; proceed to executor SideEffect discovery opt-in helper implementation.

The plan defines the right next step after the accepted WorkReport-side SideEffect discovery helper: an additive, explicit, executor-adjacent opt-in path that can read SideEffect records for report generation without changing existing executor semantics. It keeps discovery local, deterministic, in-memory, reference-only, and separate from runtime side-effect execution.

## 2. Scope Verification

The plan stayed within planning-only scope.

It does not authorize:

- executor discovery implementation in the planning phase;
- automatic SideEffect discovery in `LocalExecutor::execute_with_report(...)`;
- automatic SideEffect discovery for every report;
- report artifact writing;
- persistence changes;
- workflow schema changes;
- CLI behavior;
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

No accidental broadening was found.

## 3. Current Baseline Assessment

The plan accurately describes the implemented baseline:

- `LocalExecutionReportInputs::side_effect_ids` already forwards explicit SideEffect IDs into reports.
- `LocalExecutor::execute_with_report(...)` remains explicit-reference-only for SideEffect IDs.
- explicit proposed/denied/skipped SideEffect workflow events are implemented.
- attempted/completed/failed SideEffect event inputs fail closed.
- `SideEffectRecordStore` is separate from `StateBackend`.
- `LocalStateBackend` implements `SideEffectRecordStore`.
- `generate_terminal_local_work_report_with_side_effect_discovery(...)` is implemented and accepted as the WorkReport-side discovery boundary.

That baseline makes executor opt-in discovery a reasonable next implementation slice.

## 4. API Boundary Assessment

The recommended API shape is appropriately narrow.

The preferred free helper function is the right first implementation direction because it:

- avoids changing `LocalExecutor::execute(...)`;
- avoids changing `LocalExecutor::execute_with_report(...)`;
- avoids adding `SideEffectRecordStore` as a `StateBackend` supertrait;
- makes the store dependency explicit;
- keeps report discovery opt-in and visible in the call site;
- can reuse existing `LocalExecutionWithReportResult` semantics.

The alternative inherent-method design with `B: StateBackend + SideEffectRecordStore` is acceptable as a later hardening option, but the free helper is a cleaner first slice.

## 5. Executor Semantics Assessment

The plan preserves executor semantics:

- execution errors before a run exists return unchanged;
- terminal run plus successful discovery/report returns run plus report;
- terminal run plus discovery/report failure returns run plus report-generation error;
- non-terminal run returns no report and a stable non-leaking report-generation error;
- workflow pass/fail semantics do not change;
- existing executor methods and return types remain unchanged.

This is the correct boundary. SideEffect discovery should enrich the report path only when explicitly requested, not alter the workflow result.

## 6. Discovery Policy Assessment

The plan reuses the reviewed WorkReport-side discovery policy:

- `include_workflow_events`;
- `include_store_records`;
- `require_records`.

The policy is small enough to be testable and explicit enough to avoid ambient discovery. It correctly requires at least one discovery source, keeps proposed/denied/skipped as the only supported event-derived lifecycle states, and leaves attempted/completed/failed behavior deferred until runtime side-effect execution is designed.

One implementation detail to preserve: if `include_workflow_events` is true, the source must be the events already present on the returned run, not reloaded or inferred events.

## 7. Store And Contract Boundary Assessment

The plan correctly keeps `SideEffectRecordStore` separate from `StateBackend`.

This matters because making every state backend also be a SideEffect record store would broaden the persistence contract and create pressure for automatic discovery. The plan's explicit store parameter keeps the implementation honest: discovery is a read-only report-generation dependency, not a universal executor dependency.

The plan also correctly rejects:

- creating or repairing SideEffect records;
- lifecycle transitions;
- provider calls;
- report artifact writes;
- schema or runtime config changes.

## 8. Error Handling Assessment

The error policy is conservative and compatible with existing result semantics.

Verified planning requirements:

- no discovery source should produce `work_report_generation.side_effect_discovery.source_required`;
- non-terminal report generation should use `work_report_generation.status.not_terminal`;
- lower-level `side_effect_discovery.*` errors should remain stable and non-leaking;
- discovery failure should be report-generation failure, not workflow execution failure;
- no partial `WorkReport` should be returned after discovery failure.

The plan properly forbids leaking SideEffect IDs, workflow IDs, run IDs, store paths, target references, raw record JSON, provider payloads, command output, tokens, credentials, private keys, or secret-like values in errors.

## 9. Privacy And Redaction Assessment

The plan remains reference-only.

It correctly forbids copying:

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
- environment variables;
- credentials or token-like values;
- local filesystem paths.

Serialized reports may include stable `SideEffectId` citations, which is consistent with the existing WorkReport citation model.

## 10. Test Plan Assessment

The future test plan is strong and implementation-ready.

It covers:

- existing `execute(...)` and `execute_with_report(...)` non-regression;
- completed and failed terminal report generation with store-discovered SideEffect citations;
- canceled terminal run coverage if fixtures make it practical;
- non-terminal handling;
- no-source failure;
- missing required records;
- corrupt records and identity mismatch;
- deterministic merge and dedupe;
- event discovery for proposed/denied/skipped;
- unsupported attempted/completed/failed behavior;
- reference-only report output;
- no mutation, no post-terminal events, no artifacts, and no CLI output;
- redaction-safe Debug behavior;
- broader existing WorkReport, SideEffect, executor, runtime, adapter, evidence, hook, local-check, and docs regressions.

Non-blocking addition: the implementation should explicitly test that the free helper can use a store value distinct from the executor backend, if practical. That would prove the store boundary is genuinely explicit.

## 11. Documentation Review

The plan and roadmap documentation are honest.

Verified:

- executor SideEffect discovery opt-in is documented as planning, not implemented;
- automatic executor discovery remains unimplemented;
- `execute_with_report(...)` remains unchanged;
- report artifacts are not written;
- persistence behavior is not broadened;
- CLI rendering is not implemented;
- workflow schemas are not changed;
- examples are not updated;
- runtime side-effect execution is not implemented;
- writes remain unsupported.

## 12. Planning Blockers

No planning blockers.

## 13. Non-Blocking Follow-Ups

- In implementation, prefer the free helper function unless Rust visibility or ergonomics make that awkward.
- Add a test proving an explicit store boundary can be supplied independently from the executor backend, if practical.
- Keep the existing `execute_with_report(...)` tests as non-regression tests for explicit SideEffect ID propagation only.
- Consider a later documentation cleanup to split the long WorkReport contract status paragraph into shorter phase bullets.

## 14. Recommended Next Phase

Recommended next phase: **executor SideEffect discovery opt-in helper implementation, in-memory only**.

The implementation should add the explicit request/config type and free helper function, reuse `LocalExecutor::execute(...)`, reuse the reviewed WorkReport-side discovery helper, and preserve `LocalExecutionWithReportResult` semantics.

It must still not add automatic discovery, report artifact writing, runtime side-effect execution, attempted/completed/failed executor behavior, write-capable adapters, provider mutation, schemas, CLI behavior, examples, hosted behavior, reasoning lineage, or release posture changes.

## 15. Validation

Review validation:

- `git diff --check` - passed.
- `npm run check:docs` - passed.
