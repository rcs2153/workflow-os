# Executor Typed Handoff Report Input Propagation Plan Report

## 1. Executive Summary

This planning phase defines the next narrow WorkReport/TypedHandoff integration step: allow explicit executor-integrated report inputs to carry supplied `TypedHandoffId` values into generated in-memory reports.

The plan does not implement the behavior. It documents a small additive Rust API change to `LocalExecutionReportInputs` and a single propagation change in `terminal_report_input_for_run(...)`.

## 2. Scope Completed

Completed:

- documented the current executor propagation gap;
- defined the proposed `typed_handoff_ids: Vec<TypedHandoffId>` field;
- defined the propagation boundary from `LocalExecutionReportInputs` to `TerminalLocalWorkReportInput`;
- defined redaction-safe Debug requirements;
- defined report behavior when typed handoff IDs are supplied or absent;
- defined workflow semantics boundaries;
- defined privacy and payload exclusions;
- defined future implementation tests;
- updated roadmap and concept/planning docs.

## 3. Scope Explicitly Not Completed

Not implemented:

- executor-integrated typed handoff propagation;
- runtime handoff generation;
- automatic typed handoff discovery;
- nested harness execution;
- typed handoff persistence;
- report artifact behavior changes;
- CLI rendering or export;
- workflow schema fields;
- EvidenceReference creation from typed handoffs;
- reasoning lineage;
- side-effect boundary;
- writes;
- domain packs;
- release posture changes.

## 4. Current Baseline Summary

The WorkReport typed handoff citation target is implemented and reviewed. The terminal local report helper can cite supplied typed handoff IDs. Executor-integrated report-bearing execution is implemented, but its report input type cannot yet carry typed handoff IDs and currently forwards an empty list.

## 5. Planned Implementation Summary

The next implementation should:

- add `typed_handoff_ids: Vec<TypedHandoffId>` to `LocalExecutionReportInputs`;
- add `typed_handoff_count` to redaction-safe Debug output;
- forward the field into `TerminalLocalWorkReportInput`;
- update tests and docs.

No full `TypedHandoff` values should be accepted, resolved, created, copied, or persisted.

## 6. Workflow Semantics Summary

The plan preserves existing workflow semantics:

- `execute(...)` remains unchanged;
- `execute_with_report(...)` remains explicit and additive;
- report generation errors remain report-generation errors;
- no post-terminal events are appended;
- no runtime state is mutated for typed handoff propagation;
- no automatic report artifacts are written.

## 7. Redaction And Privacy Summary

Typed handoff IDs are stable references and should be treated as sensitive enough for Debug redaction. The implementation should expose only counts in executor input Debug output and must not copy typed handoff payload fields into reports, citations, errors, or docs.

## 8. Test Coverage Plan Summary

The future implementation should add tests for:

- executor report input accepts typed handoff IDs;
- `execute_with_report(...)` propagates typed handoff IDs into generated report citations;
- typed handoff citations appear in `OperatorHandoffNotes`;
- no IDs leak through Debug;
- absent typed handoff IDs preserve current behavior;
- no state mutation, event append, artifact write, or payload copying occurs;
- existing WorkReport, TypedHandoff, executor, artifact, EvidenceReference, Diagnostic, validation, adapter telemetry, and runtime tests still pass.

## 9. Commands Run And Results

Planning validation command:

- `npm run check:docs` - result recorded in the final response for this phase.

## 10. Remaining Known Limitations

- Executor-integrated typed handoff propagation is planned but not implemented.
- Typed handoff persistence is not implemented.
- Runtime handoff generation is not implemented.
- Nested harness execution is not implemented.
- Report artifacts do not validate typed handoff referential integrity.
- CLI, schemas, examples, reasoning lineage, side effects, and writes remain unimplemented.

## 11. Recommended Next Phase

Recommended next phase: executor-integrated typed handoff report input propagation implementation.

That phase should remain narrowly scoped to adding the explicit input field, forwarding it into terminal report generation, adding tests, and updating docs.
