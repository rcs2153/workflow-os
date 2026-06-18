# Executor SideEffect Report Input Propagation Report

Report date: 2026-06-17

## 1. Executive Summary

Executor SideEffect report input propagation is implemented as a narrow, additive local API change.

`LocalExecutionReportInputs` now accepts explicitly supplied `SideEffectId` values. `LocalExecutor::execute_with_report(...)` forwards those IDs into `TerminalLocalWorkReportInput`, allowing the existing terminal report helper to cite SideEffect records in the required WorkReport side-effects section.

This phase does not implement automatic SideEffect discovery, SideEffect record creation or resolution, SideEffect persistence, side-effect workflow events, audit projections, EvidenceReference side-effect attachment, runtime side-effect execution, write-capable adapters, provider mutations, report artifact behavior changes, schemas, CLI behavior, examples, hosted behavior, reasoning lineage, or release posture changes.

## 2. Scope Completed

- Added `side_effect_ids: Vec<SideEffectId>` to `LocalExecutionReportInputs`.
- Added redaction-safe Debug count output for supplied SideEffect IDs.
- Forwarded `report.side_effect_ids.clone()` in `terminal_report_input_for_run(...)`.
- Preserved existing `LocalExecutor::execute(...)`, `decide_approval(...)`, and `cancel_run(...)` behavior.
- Added focused executor tests proving supplied SideEffect IDs are cited in generated reports.
- Updated roadmap and planning docs.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- automatic SideEffect citation discovery;
- SideEffect record creation;
- SideEffect record resolution;
- SideEffect persistence;
- side-effect workflow events;
- side-effect audit projections;
- EvidenceReference side-effect attachment;
- approval-side-effect linkage;
- runtime side-effect execution;
- write-capable adapters;
- provider mutations;
- rollback or compensation behavior;
- report artifact behavior changes;
- workflow schema fields;
- CLI rendering or export;
- example updates;
- hosted or distributed runtime claims;
- reasoning lineage implementation;
- release posture changes.

## 4. API Summary

`LocalExecutionReportInputs` now includes:

```rust
pub side_effect_ids: Vec<SideEffectId>
```

The field accepts only already-validated stable SideEffect IDs. It does not accept `SideEffectRecord` values, raw target references, summaries, reason codes, authority context, lifecycle state, outcome references, idempotency details, or redaction metadata.

## 5. Propagation Summary

Executor report input conversion now forwards:

```rust
side_effect_ids: report.side_effect_ids.clone()
```

The terminal report helper remains responsible for citation construction through existing `WorkReportCitation` validation. The executor does not create citations directly and does not create or resolve SideEffect records.

## 6. Report Behavior Summary

When explicit SideEffect IDs are supplied to `execute_with_report(...)`:

- generated reports include `WorkReportCitationTarget::SideEffect` citations;
- citations appear only in `WorkReportSectionKind::SideEffects`;
- citation kind is `WorkReportCitationKind::SideEffect`;
- the side-effects section uses the existing bounded supplied-reference summary;
- no SideEffect record payload fields are copied.

When no SideEffect IDs are supplied, generated reports preserve existing none/skipped/unsupported side-effects section behavior.

## 7. Workflow Semantics Summary

The implementation does not:

- mutate `WorkflowRun`;
- mutate `WorkflowRunSnapshot`;
- append workflow events;
- emit audit events;
- emit observability events;
- create or resolve SideEffect records;
- persist side effects;
- execute side effects;
- invoke adapters because a SideEffect ID was supplied;
- write report artifacts automatically;
- expose CLI output;
- change workflow pass/fail behavior.

Report generation failure remains separate from workflow execution failure.

## 8. Redaction And Privacy Summary

Executor report inputs remain reference-only.

The implementation does not copy:

- side-effect target references;
- side-effect summaries;
- side-effect reason codes;
- side-effect authority context;
- side-effect lifecycle payloads;
- side-effect outcome references;
- side-effect idempotency details;
- side-effect redaction metadata;
- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira or GitHub bodies;
- raw spec contents;
- parser payloads;
- environment variable values;
- credentials, authorization headers, private keys, or token-like values.

`LocalExecutionReportInputs` Debug output reports only `side_effect_count` and does not expose supplied SideEffect IDs.

## 9. Test Coverage Summary

Added focused executor coverage for:

- `execute_with_report(...)` forwarding supplied SideEffect IDs into generated report citations;
- generated citation target and kind;
- SideEffect citations appearing only in the side-effects section;
- redaction-safe executor report input Debug output;
- redaction-safe result Debug output;
- no side-effect payload markers copied into serialized reports;
- no event-history mutation;
- no automatic report artifact writes.

Existing WorkReport, SideEffect, executor, evidence, diagnostic, validation, adapter, hook, local check, and runtime tests continue to pass through workspace validation.

## 10. Commands Run And Results

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.

## 11. Remaining Known Limitations

- SideEffect IDs are supplied explicitly by callers; there is no automatic discovery from workflow events, audit projections, stores, adapter telemetry, local checks, hooks, disclosures, typed handoffs, or report notes.
- SideEffect records are not persisted.
- SideEffect workflow events and audit projections are not implemented.
- EvidenceReference side-effect attachment is not implemented.
- Runtime side-effect execution and write-capable adapters are not implemented.
- Report artifacts do not perform SideEffect referential integrity checks.
- Schemas, CLI behavior, examples, hosted behavior, and release posture are unchanged.

## 12. Recommended Next Phase

Recommended next phase: executor SideEffect report input propagation review.

Review this implementation before planning automatic discovery, side-effect workflow events, audit projections, persistence, EvidenceReference side-effect attachment, runtime side-effect execution, write-capable adapters, schemas, CLI behavior, examples, hosted behavior, reasoning lineage, or release posture changes.
