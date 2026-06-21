# WorkReport SideEffect Discovery Integration Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation adds the intended explicit WorkReport-side SideEffect discovery helper and keeps the boundary narrow, in-memory, deterministic, and reference-only. It does not broaden executor behavior, persistence, artifacts, CLI behavior, schemas, examples, runtime side-effect execution, writes, hosted behavior, reasoning lineage, or release posture.

The recommended next phase is executor SideEffect discovery opt-in planning.

## 2. Scope Verification

The phase stayed within approved scope.

Implemented scope:

- explicit `TerminalLocalWorkReportSideEffectDiscoveryInput`;
- explicit `generate_terminal_local_work_report_with_side_effect_discovery(...)`;
- use of `discover_side_effect_references_from_store(...)` for store-backed discovery;
- optional use of workflow events already present on the borrowed run;
- deterministic merge into `TerminalLocalWorkReportInput::side_effect_ids`;
- final WorkReport construction through `generate_terminal_local_work_report(...)`;
- focused tests and documentation updates.

No accidental implementation found for:

- automatic SideEffect discovery in executor paths;
- changes to `LocalExecutor::execute_with_report(...)`;
- automatic report generation;
- report artifact writing;
- report artifact referential integrity;
- EvidenceReference side-effect attachment;
- approval-side-effect linkage;
- runtime side-effect execution;
- attempted/completed/failed executor behavior;
- provider mutation;
- write-capable adapters;
- workflow schema fields;
- CLI commands or rendering;
- example updates;
- hosted/distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 3. Helper API Assessment

The helper API is appropriately explicit:

```rust
pub fn generate_terminal_local_work_report_with_side_effect_discovery(
    store: &impl SideEffectRecordStore,
    input: TerminalLocalWorkReportInput<'_>,
    discovery: TerminalLocalWorkReportSideEffectDiscoveryInput,
) -> Result<WorkReport, WorkflowOsError>
```

This is the correct level for this phase. It avoids adding `SideEffectRecordStore` to `StateBackend`, avoids changing executor generics, and avoids hidden global state.

The policy type is intentionally small:

- `include_workflow_events`;
- `include_store_records`;
- `require_records`.

The helper fails closed when no discovery source is enabled with stable code `work_report_generation.side_effect_discovery.source_required`.

## 4. Discovery Boundary Assessment

The helper derives immutable run identity from the supplied `WorkflowRun` inside `TerminalLocalWorkReportInput`.

Verified identity fields passed into discovery:

- workflow ID;
- workflow version;
- schema version;
- spec content hash;
- run ID.

The helper does not trust report prose, filesystem paths, adapter telemetry payloads, audit projections, natural-language summaries, or caller-supplied duplicate identity fields.

Store-backed discovery delegates to `discover_side_effect_references_from_store(...)`. Event-only discovery delegates to `discover_side_effect_references(...)`. That reuse is good: validation, identity checks, record-required behavior, unsupported-event filtering, and deterministic ordering stay centralized.

## 5. Citation And Merge Assessment

The helper replaces `input.side_effect_ids` with discovered references from `SideEffectDiscoveryResult::references()` and then calls the existing terminal report generator.

This preserves the existing WorkReport citation path:

- citations are built through `WorkReportCitation` constructors;
- citations appear in `WorkReportSectionKind::SideEffects`;
- `EvidenceReference` values are not created or recreated;
- SideEffect records remain source-of-truth records, while WorkReports cite stable IDs.

The deterministic merge behavior is acceptable for this phase. The tests show caller-supplied and store-discovered IDs deduplicate and serialize into stable report citations.

## 6. Runtime And State Boundary Assessment

The implementation does not mutate workflow state.

Verified boundaries:

- no `WorkflowRun` mutation;
- no `WorkflowRunSnapshot` mutation;
- no workflow event append;
- no audit or observability emission;
- no SideEffect record creation, update, repair, or lifecycle transition;
- no adapter/provider calls;
- no side-effect execution;
- no WorkReport artifact writes;
- no filesystem artifact creation;
- no CLI output;
- no workflow pass/fail semantic changes.

The only state interaction is read-only access through the explicitly supplied `SideEffectRecordStore`.

## 7. Error Handling Assessment

The helper uses stable non-leaking errors.

Verified:

- no discovery source returns `work_report_generation.side_effect_discovery.source_required`;
- missing required records return the lower-level stable code `side_effect_discovery.record_missing`;
- no partial `WorkReport` is returned on discovery failure;
- errors do not include SideEffect IDs, target references, record payloads, provider output, command output, paths, tokens, or secret-like values.

The error posture is consistent with the plan: discovery failure remains a report-generation failure, not a workflow execution failure.

## 8. Privacy And Redaction Assessment

The phase remains reference-only.

Verified:

- generated reports cite `SideEffectId` values only;
- SideEffect target references are not copied into report serialization;
- SideEffect summaries are not copied into report serialization;
- record authority context, reason codes, lifecycle payloads, idempotency details, raw record JSON, provider payloads, command output, spec contents, and secret-like values are not copied by the helper;
- Debug behavior for the helper policy is safe because it contains only booleans;
- final report Debug/serialization continues to use existing WorkReport redaction behavior.

## 9. Test Quality Assessment

Tests cover the important first-slice behaviors:

- store-backed records are cited in the WorkReport side-effects section;
- explicit and store-backed IDs merge deterministically without duplicates;
- missing discovery source fails closed without leaking;
- missing required records fail closed without leaking;
- helper does not mutate the borrowed run;
- helper does not write WorkReport artifacts;
- serialized report output does not copy SideEffect target references or summaries;
- existing WorkReport, SideEffect discovery, state, executor, runtime, adapter, evidence, hook, and local-check tests continue to pass through workspace validation.

Non-blocking test gaps:

- add a direct WorkReport-helper test for `include_workflow_events: true` without store discovery;
- add a direct WorkReport-helper test showing attempted/completed/failed SideEffect events remain unsupported and uncited;
- add direct WorkReport-helper tests for identity mismatch and corrupt store records, even though lower-level discovery tests already cover those cases;
- add a direct Debug assertion for `TerminalLocalWorkReportSideEffectDiscoveryInput`, mostly to preserve the current safe shape if the policy later gains fields.

These are follow-ups, not blockers, because the delegated lower-level discovery helpers already cover the key identity, corrupt-record, and unsupported-event semantics.

## 10. Documentation Review

Documentation is honest and scoped.

Verified docs state:

- explicit WorkReport SideEffect discovery helper integration is implemented;
- automatic executor SideEffect discovery is not implemented;
- `LocalExecutor::execute_with_report(...)` remains explicit-ID-only;
- report artifacts and artifact referential integrity are not implemented in this phase;
- EvidenceReference side-effect attachment is not implemented;
- approval-side-effect linkage is not implemented;
- runtime side-effect execution is not implemented;
- attempted/completed/failed executor behavior is not implemented;
- writes, provider mutation, schemas, CLI rendering, examples, hosted behavior, reasoning lineage, and release posture changes remain unsupported.

One minor documentation follow-up: the very long WorkReport contract status paragraph remains hard to review, though it is factually acceptable.

## 11. Blockers

No blockers.

## 12. Non-Blocking Follow-Ups

- Add direct WorkReport-helper coverage for event-only discovery.
- Add direct WorkReport-helper coverage for unsupported attempted/completed/failed SideEffect events.
- Add direct WorkReport-helper coverage for corrupt store records and identity mismatch.
- Add a Debug non-leakage assertion for `TerminalLocalWorkReportSideEffectDiscoveryInput`.
- Consider splitting the long WorkReport contract plan status paragraph into shorter status/update bullets in a future docs cleanup.

## 13. Recommended Next Phase

Recommended next phase: **executor SideEffect discovery opt-in planning**.

The WorkReport-side helper is now the reviewed boundary that an executor-facing phase can plan around. The next phase should decide whether an additive executor API should opt into SideEffect discovery from a `SideEffectRecordStore`, how that API avoids changing existing executor semantics, and how report-generation errors remain separate from workflow execution results.

Do not implement executor discovery until that plan is written and reviewed. Do not add automatic discovery, report artifact writing, runtime side-effect execution, attempted/completed/failed side-effect behavior, write-capable adapters, provider mutation, schemas, CLI behavior, examples, hosted behavior, reasoning lineage, or release posture changes.

## 14. Validation

Implementation report validation:

- `cargo fmt --all` - passed.
- `cargo test -p workflow-core --test work_report` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

Review validation:

- `npm run check:docs` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
