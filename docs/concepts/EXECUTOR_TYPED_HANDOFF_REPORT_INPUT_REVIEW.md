# Executor Typed Handoff Report Input Propagation Review

Review date: 2026-06-15

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation adds explicit typed handoff ID propagation to executor-integrated report inputs. `LocalExecutionReportInputs` now accepts `typed_handoff_ids: Vec<TypedHandoffId>`, its Debug output reports only a count, and `terminal_report_input_for_run(...)` forwards supplied IDs into terminal report generation. Generated reports then cite those IDs through the already-reviewed terminal report helper path.

The phase stayed within the approved narrow propagation scope. No blockers were found.

## 2. Scope Verification

The implementation stayed within approved scope.

Implemented:

- added typed handoff ID input to `LocalExecutionReportInputs`;
- forwarded supplied IDs into `TerminalLocalWorkReportInput`;
- preserved `LocalExecutor::execute(...)` behavior;
- preserved explicit `LocalExecutor::execute_with_report(...)` semantics;
- preserved report-generation error behavior;
- added focused executor tests;
- updated docs and created the phase report.

No accidental implementation was found for:

- runtime handoff generation;
- automatic typed handoff discovery;
- automatic citation from workflow events;
- nested harness execution;
- typed handoff persistence;
- typed handoff artifact storage;
- report artifact behavior changes;
- automatic artifact writing from executor paths;
- workflow schema fields;
- CLI rendering or export;
- example updates;
- EvidenceReference creation from typed handoffs;
- approval evidence attachment;
- reasoning lineage;
- side-effect boundary;
- writes;
- domain packs;
- release posture changes.

## 3. API Assessment

The API change is appropriately small and explicit.

`LocalExecutionReportInputs` now includes:

```rust
pub typed_handoff_ids: Vec<TypedHandoffId>
```

This reuses the existing typed ID model rather than accepting raw strings or full `TypedHandoff` values. The executor input remains an explicit caller-provided report input structure. It does not read hidden state, inspect workflow events for handoffs, validate referential existence, or invent typed handoff IDs.

Existing executor methods remain unchanged:

- `execute(...)`;
- `decide_approval(...)`;
- `cancel_run(...)`.

The only report-bearing executor method remains `execute_with_report(...)`.

## 4. Propagation Assessment

Propagation is correct and bounded.

`terminal_report_input_for_run(...)` now forwards:

```rust
typed_handoff_ids: report.typed_handoff_ids.clone()
```

That is the intended single behavior change. The terminal report helper then handles citation construction through the already-reviewed `TypedHandoffId` to `WorkReportCitationTarget::TypedHandoff` path.

No full `TypedHandoff` values are created, loaded, mutated, resolved, or copied by the executor.

## 5. Citation Assessment

Generated reports cite typed handoffs by stable typed handoff ID.

Verified by tests:

- typed handoff citations appear in `OperatorHandoffNotes`;
- citation target is `WorkReportCitationTarget::TypedHandoff`;
- citation kind is `WorkReportCitationKind::TypedHandoff`;
- supplied typed handoff IDs are not recreated as `EvidenceReference` values;
- absent typed handoff IDs preserve empty handoff citation behavior.

The citation path remains reference-first. It does not validate typed handoff existence or imply a runtime handoff occurred.

## 6. Workflow Semantics Assessment

Workflow semantics are preserved.

Verified:

- existing `execute(...)` still returns only `WorkflowRun`;
- `execute_with_report(...)` still calls normal execution first;
- execution errors are not changed into report errors;
- report-generation failures remain in `report_generation_error`;
- event history is unchanged by typed handoff propagation;
- no post-terminal events are appended for report citation propagation;
- no report artifacts are written automatically;
- no runtime state mutation beyond existing execution behavior was introduced.

This remains an explicit, local, in-memory report-input propagation path.

## 7. Privacy And Redaction Assessment

The implementation preserves the redaction boundary.

Verified:

- `LocalExecutionReportInputs` Debug includes `typed_handoff_count` only;
- Debug output does not expose typed handoff IDs;
- `LocalExecutionWithReportRequest` Debug remains redaction-safe;
- `LocalExecutionWithReportResult` Debug remains redaction-safe;
- generated report serialization includes the stable typed handoff citation reference but not typed handoff payload content;
- tests check that handoff payload markers such as obligations, disclosures, and raw provider payload markers are not copied.

Typed handoff IDs remain stable references and should be treated as potentially sensitive in future CLI, schema, artifact, and SDK surfaces.

## 8. Error Handling Assessment

Error handling remains stable and non-leaking.

Because the executor input field uses `TypedHandoffId`, invalid raw ID rejection occurs before this propagation boundary. Report generation still returns structured `WorkflowOsError` values through existing WorkReport and citation validation.

No path was found that converts typed handoff citation errors into workflow diagnostics, mutates the run, or appends compensating events.

## 9. Test Quality Assessment

Test coverage is focused and appropriate.

Covered:

- completed executor report path produces a report with typed handoff citation;
- typed handoff citation target and kind are asserted;
- typed handoff citations are placed in `OperatorHandoffNotes`;
- absent typed handoff IDs preserve empty handoff citation behavior;
- executor report input Debug output does not leak typed handoff IDs;
- generated result Debug output does not leak typed handoff IDs;
- generated report serialization does not copy typed handoff payload markers;
- event history is unchanged;
- no report artifacts are written automatically;
- existing executor, WorkReport, TypedHandoff, WorkReportContract, EvidenceReference, Diagnostic, validation, adapter telemetry, artifact, CLI, example, and runtime tests pass.

Non-blocking gap: there is no helper-level invalid raw typed handoff ID test in the executor path because the API accepts `TypedHandoffId` rather than raw strings. That is acceptable for this phase; lower-level `TypedHandoffId` and serialized citation tests cover the fail-closed boundary.

## 10. Documentation Review

Documentation now states:

- executor-integrated typed handoff report input propagation is implemented;
- terminal report helper typed handoff citation integration is implemented;
- runtime handoff generation is not implemented;
- automatic typed handoff discovery is not implemented;
- nested harness execution is not implemented;
- typed handoff persistence is not implemented;
- report artifact behavior changes are not implemented;
- CLI rendering/export is not implemented;
- workflow schema fields are not implemented;
- EvidenceReference creation from typed handoffs is not implemented;
- reasoning lineage, side effects, writes, domain packs, and release posture changes remain unimplemented.

One small false-current-state statement in the implementation plan was corrected during review: the old baseline text still said `LocalExecutionReportInputs` lacked `typed_handoff_ids` even though the phase had implemented it.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Consider artifact round-trip tests for full reports containing typed handoff citations during the next report artifact review.
- Decide later whether report artifacts should validate typed handoff referential integrity once typed handoff persistence exists.
- Keep typed handoff ID exposure out of CLI/schema/SDK surfaces until those surfaces are separately planned and reviewed.
- Consider whether future approval-resume or cancellation report-bearing APIs should accept typed handoff IDs if those APIs are ever introduced.

## 13. Recommended Next Phase

Recommended next phase: report/audit/missing-citation semantics planning.

Reason: typed handoff citation propagation is now implemented through the explicit executor report path. The next unresolved report foundation question is how unavailable references, missing citations, audit references, and report completeness should be represented consistently before broadening report behavior, artifact semantics, CLI surfaces, or future schema exposure.

This next phase should remain planning-only and must not add runtime handoff generation, nested harness execution, persistence changes, CLI behavior, schema changes, reasoning lineage, side effects, writes, or release posture changes.

## 14. Validation

Validation commands for this review:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`.

The review final response records the commands run and results.
