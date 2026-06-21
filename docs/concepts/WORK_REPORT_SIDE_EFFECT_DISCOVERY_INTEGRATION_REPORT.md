# WorkReport SideEffect Discovery Integration Report

## 1. Executive Summary

The WorkReport SideEffect discovery integration phase is implemented as an explicit, in-memory helper. Report-generation callers can now opt into bounded SideEffect discovery from an existing `SideEffectRecordStore` before constructing a terminal local `WorkReport`.

The implementation remains reference-only. It cites stable `SideEffectId` values in the WorkReport side-effects section and does not copy SideEffect record payloads, target references, summaries, authority context, reason codes, idempotency details, provider payloads, command output, or raw record JSON.

## 2. Scope Completed

- Added `TerminalLocalWorkReportSideEffectDiscoveryInput`.
- Added `generate_terminal_local_work_report_with_side_effect_discovery(...)`.
- Reused `discover_side_effect_references_from_store(...)` for store-backed discovery.
- Supported explicit inclusion of already-loaded workflow events from the borrowed terminal run.
- Merged caller-supplied SideEffect IDs with discovered IDs deterministically through the existing discovery helper.
- Delegated final report construction to `generate_terminal_local_work_report(...)`.
- Exported the helper API from `workflow-core`.
- Added focused WorkReport tests for discovery integration, deterministic merging, missing-source failure, missing-record failure, non-mutation, artifact non-writing, and payload non-copying.
- Updated roadmap and planning documents to mark the helper integration implemented.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- automatic SideEffect discovery in executor report paths;
- changes to `LocalExecutor::execute_with_report(...)`;
- report artifact integration;
- automatic artifact writing;
- EvidenceReference side-effect attachment;
- approval-side-effect linkage;
- runtime side-effect execution;
- attempted/completed/failed executor behavior;
- write-capable adapters;
- provider mutation;
- workflow schema fields;
- CLI commands or rendering;
- example updates;
- hosted or distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 4. Helper API Summary

The new helper is:

```rust
pub fn generate_terminal_local_work_report_with_side_effect_discovery(
    store: &impl SideEffectRecordStore,
    input: TerminalLocalWorkReportInput<'_>,
    discovery: TerminalLocalWorkReportSideEffectDiscoveryInput,
) -> Result<WorkReport, WorkflowOsError>
```

The discovery policy contains:

- `include_workflow_events`;
- `include_store_records`;
- `require_records`.

At least one discovery source must be enabled. If no source is enabled, the helper fails with stable code `work_report_generation.side_effect_discovery.source_required`.

## 5. Discovery Policy Summary

The helper derives immutable identity from the supplied terminal `WorkflowRun`:

- workflow ID;
- workflow version;
- schema version;
- spec hash;
- run ID.

When store discovery is enabled, it reads through the caller-supplied `SideEffectRecordStore` and delegates validation to `discover_side_effect_references_from_store(...)`.

When workflow-event discovery is enabled, it considers only the events already present on the borrowed run. Proposed, denied, and skipped SideEffect events remain the supported discovery states. Attempted, completed, and failed remain future lifecycle vocabulary until runtime execution semantics are planned.

## 6. Citation And Merge Behavior

The helper seeds discovery with `TerminalLocalWorkReportInput::side_effect_ids`, merges discovered references deterministically, and replaces the input SideEffect ID vector before calling the existing terminal report generator.

Final WorkReport citation construction still goes through existing `WorkReportCitation` constructors. SideEffect citations appear only in `WorkReportSectionKind::SideEffects`.

No `EvidenceReference` values are created or recreated by this helper.

## 7. Runtime Boundary Summary

The helper does not:

- mutate `WorkflowRun`;
- mutate `WorkflowRunSnapshot`;
- append workflow events;
- create, repair, update, or transition SideEffect records;
- emit audit or observability events;
- call adapters or providers;
- execute side effects;
- write report artifacts;
- create files;
- expose CLI output;
- change workflow pass/fail behavior.

The only state interaction is optional read-only access through an explicitly supplied `SideEffectRecordStore`.

## 8. Redaction And Privacy Summary

The integration is redaction-safe and reference-only. Tests cover that generated reports do not copy SideEffect target references or SideEffect record summaries into serialized report payloads.

Errors use stable codes and do not include SideEffect IDs, workflow IDs, run IDs, target references, paths, raw record payloads, provider payloads, command output, tokens, or secret-like values.

## 9. Test Coverage Summary

Added focused tests covering:

- store-backed SideEffect records are cited in WorkReports;
- caller-supplied IDs and store-backed records merge deterministically without duplicates;
- helper fails closed when no discovery source is enabled;
- required missing records fail closed without leaking IDs;
- helper does not mutate the borrowed run or append events;
- helper does not write WorkReport artifacts;
- serialized reports do not copy SideEffect target references or summaries.

Existing WorkReport tests continue to cover explicit SideEffect citation behavior, unsupported side-effects section text, redaction-safe Debug output, serialization non-leakage, and terminal report behavior.

## 10. Commands Run And Results

- `cargo fmt --all` - passed.
- `cargo test -p workflow-core --test work_report` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

## 11. Remaining Known Limitations

- Executor report paths still propagate only explicitly supplied SideEffect IDs.
- No automatic executor SideEffect discovery exists.
- No report artifact referential integrity checks exist for SideEffect citations.
- EvidenceReference side-effect attachment remains unimplemented.
- Attempted/completed/failed runtime side-effect execution remains unimplemented.
- Write-capable adapters remain unimplemented.

## 12. Recommended Next Phase

Recommended next phase: **WorkReport SideEffect discovery helper integration review**.

Review should verify scope, helper API shape, source-of-truth boundaries, deterministic merge behavior, privacy/redaction posture, tests, and documentation before any executor-level opt-in discovery or write-capable adapter planning.
