# Executor Typed Handoff Report Input Propagation Plan

Status: Executor-integrated typed handoff report input propagation is implemented. Runtime handoff generation, automatic typed handoff discovery, nested harness execution, typed handoff persistence, report artifact behavior changes, CLI behavior, workflow schema fields, side-effect modeling, writes, reasoning lineage, domain packs, and release posture changes are not implemented.

## 1. Executive Summary

WorkReport typed handoff citation target vocabulary is implemented and reviewed. The terminal local report helper can now cite explicitly supplied `TypedHandoffId` values in generated in-memory reports.

The remaining executor gap is narrow: `LocalExecutionReportInputs` does not yet accept typed handoff IDs, and the executor adapter path currently passes an empty typed handoff list into `TerminalLocalWorkReportInput`.

This plan defines the smallest safe implementation for explicit executor-integrated typed handoff report input propagation. It does not implement runtime handoff generation, automatic discovery, persistence, schemas, CLI behavior, report artifacts, nested harness execution, side effects, writes, reasoning lineage, or release posture changes.

## 2. Goals

- Let callers of `LocalExecutor::execute_with_report(...)` supply existing `TypedHandoffId` values.
- Forward those IDs from `LocalExecutionReportInputs` to `TerminalLocalWorkReportInput`.
- Preserve executor semantics and existing `execute(...)` behavior.
- Keep typed handoff citations explicit, local, deterministic, and in-memory.
- Use existing `TypedHandoffId`, `WorkReportCitation`, and terminal report helper validation boundaries.
- Avoid copying full typed handoff payloads into executor inputs, report sections, citations, debug output, or serialization.
- Preserve current behavior when no typed handoff IDs are supplied.

## 3. Non-Goals

This plan does not authorize:

- runtime handoff generation;
- automatic typed handoff discovery;
- automatic citation from workflow events;
- nested harness execution;
- typed handoff persistence;
- typed handoff artifact storage;
- report artifact behavior changes;
- workflow schema fields;
- CLI rendering or export;
- example updates;
- EvidenceReference creation from typed handoffs;
- approval evidence attachment;
- reasoning lineage implementation;
- side-effect boundary implementation;
- write-capable adapters;
- domain packs;
- hosted or distributed runtime claims;
- release posture changes.

## 4. Current Baseline

Implemented:

- `TypedHandoffId`;
- typed handoff core model;
- WorkReport typed handoff citation kind and target vocabulary;
- terminal local report helper support for supplied `typed_handoff_ids`;
- executor-integrated report-bearing execution through `LocalExecutor::execute_with_report(...)`.
- executor-integrated report input propagation for supplied typed handoff IDs.

Closed gap:

- `LocalExecutionReportInputs` now has a `typed_handoff_ids` field.
- `terminal_report_input_for_run(...)` now forwards `report.typed_handoff_ids.clone()`.
- Therefore executor-integrated report-bearing execution can forward caller-supplied typed handoff references into generated reports.

This closed gap remains intentionally small and does not change runtime behavior.

## 5. Proposed API Change

Add one field to `LocalExecutionReportInputs`:

```rust
pub typed_handoff_ids: Vec<TypedHandoffId>
```

Rules:

- accept only already-validated `TypedHandoffId` values;
- do not accept raw strings;
- do not accept `TypedHandoff` values;
- do not accept handoff obligations, disclosures, risk text, notes, endpoint details, or payload fields;
- do not read typed handoffs from storage;
- do not validate typed handoff existence;
- do not infer IDs from workflow events or report notes;
- do not fabricate missing typed handoff IDs.

This is an additive local Rust API change. It must preserve existing executor methods and should update tests and call sites that construct `LocalExecutionReportInputs`.

## 6. Propagation Boundary

Update `terminal_report_input_for_run(...)` so:

```rust
typed_handoff_ids: report.typed_handoff_ids.clone()
```

This is the only intended behavior change.

The executor should still:

- call `execute(&request.execution)` first;
- return execution errors unchanged;
- preserve `run` when report generation fails after execution;
- never mutate workflow state for report generation;
- never append events for report generation;
- never write report artifacts automatically;
- never expose CLI output.

## 7. Debug And Redaction Policy

Update `LocalExecutionReportInputs` Debug to include only a count:

```rust
.field("typed_handoff_count", &self.typed_handoff_ids.len())
```

Debug output must not include typed handoff IDs, report IDs, paths, tokens, notes, limitations, risks, handoff text, or payload-like strings.

Serialization is not currently the primary surface for executor report inputs. If serialization is added later, typed handoff IDs must be treated as potentially sensitive stable references and must not be mixed with handoff payload fields.

## 8. Report Behavior

When typed handoff IDs are supplied:

- generated reports should include typed handoff citations in `OperatorHandoffNotes`;
- citation target should be `WorkReportCitationTarget::TypedHandoff`;
- citation kind should be `WorkReportCitationKind::TypedHandoff`;
- citation summary should remain bounded and generic;
- no `TypedHandoff` value should be created, resolved, or copied.

When no typed handoff IDs are supplied:

- generated reports should preserve current behavior;
- no typed handoff citation should be added;
- no missing typed handoff citation should be invented by default.

## 9. Workflow Semantics Boundary

The implementation must not:

- mutate `WorkflowRun`;
- mutate `WorkflowRunSnapshot`;
- append workflow events;
- emit audit events;
- emit observability events;
- touch `StateBackend` beyond existing executor behavior;
- write report artifacts;
- persist typed handoffs;
- read typed handoffs from storage;
- expose CLI output;
- change workflow pass/fail behavior;
- change existing `execute(...)`, `decide_approval(...)`, or `cancel_run(...)` behavior.

Report generation failure must remain separate from workflow execution failure in `LocalExecutionWithReportResult`.

## 10. Error Handling

Because the new input field should use `TypedHandoffId`, invalid raw ID values fail before the executor propagation boundary.

If report generation fails after typed handoff IDs are supplied:

- preserve the run;
- return `work_report: None`;
- return `report_generation_error: Some(...)`;
- avoid leaking typed handoff IDs, notes, paths, tokens, raw payloads, command output, parser output, or provider data in errors;
- do not convert report-generation errors into workflow diagnostics.

## 11. Privacy And Payload Policy

Executor report inputs must remain reference-only.

The implementation must not copy:

- typed handoff obligations;
- typed handoff disclosures;
- typed handoff risks;
- typed handoff notes;
- typed handoff endpoint details;
- typed handoff reference labels;
- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira or GitHub bodies;
- raw spec contents;
- parser payloads;
- environment variable values;
- credentials, authorization headers, private keys, or token-like values.

Typed handoff IDs are stable references. Future artifact, schema, CLI, or SDK exposure must treat them as sensitive enough to redact in Debug and review before public contract exposure.

## 12. Test Plan

Future implementation tests should cover:

- `LocalExecutionReportInputs` accepts `typed_handoff_ids`;
- `execute_with_report(...)` forwards supplied typed handoff IDs into generated report citations;
- generated citation target is `WorkReportCitationTarget::TypedHandoff`;
- generated citation kind is `WorkReportCitationKind::TypedHandoff`;
- typed handoff citations appear in `OperatorHandoffNotes`;
- absent typed handoff IDs preserve current no-handoff behavior;
- Debug for `LocalExecutionReportInputs` reports only count and does not leak typed handoff IDs;
- Debug for `LocalExecutionWithReportRequest` remains redaction-safe;
- report-generation failure preserves run and event history;
- no runtime events are appended for typed handoff propagation;
- no `StateBackend` write beyond existing executor execution behavior;
- no report artifacts are written automatically;
- no full `TypedHandoff` value is created, resolved, or copied;
- no typed handoff payload markers appear in Debug or serialized generated reports;
- existing WorkReport, typed handoff, executor, artifact, evidence, diagnostic, validation, adapter telemetry, and runtime tests still pass.

## 13. Proposed Implementation Sequence

1. Add `typed_handoff_ids: Vec<TypedHandoffId>` to `LocalExecutionReportInputs`.
2. Import/export `TypedHandoffId` in executor code as needed.
3. Update `LocalExecutionReportInputs` Debug with `typed_handoff_count`.
4. Forward `report.typed_handoff_ids.clone()` in `terminal_report_input_for_run(...)`.
5. Update existing test helpers that construct `LocalExecutionReportInputs`.
6. Add focused executor tests for propagation, redaction-safe Debug, absence behavior, and non-mutation.
7. Update docs and create an end-of-phase report.
8. Run full validation.

## 14. Open Questions

- Should executor-integrated typed handoff propagation eventually validate that supplied IDs exist in a future typed handoff store?
- Should missing typed handoff references ever become explicit `missing=true` citations, or remain section text unless contract requirements change?
- Should future report artifacts verify referential integrity for typed handoff citations?
- Should typed handoff IDs appear in CLI JSON only after schema and redaction posture are separately reviewed?
- Should approval-resume and cancellation report-bearing APIs receive the same typed handoff input later?

## 15. Final Recommendation

Recommended next implementation phase: executor-integrated typed handoff report input propagation.

The implementation added only the explicit input field and forwarding behavior described here. It does not implement runtime handoff generation, automatic discovery, nested harness execution, typed handoff persistence, report artifact behavior changes, CLI rendering, workflow schema changes, EvidenceReference creation, reasoning lineage, side-effect modeling, writes, domain packs, or release posture changes.
