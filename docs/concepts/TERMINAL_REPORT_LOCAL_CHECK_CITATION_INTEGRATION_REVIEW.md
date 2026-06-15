# Terminal Report Local Check Citation Integration Review

Review date: 2026-06-15

## 1. Executive Verdict

Phase accepted; proceed to command-output evidence policy planning only if evidence attachment is needed.

The terminal report local check citation integration is appropriately narrow. It consumes caller-supplied stable local check result references, builds `WorkReportCitationTarget::LocalCheckResult` citations through existing WorkReport citation validation, and places them in the `validation and quality checks` section. It does not execute checks, create references, create EvidenceReference values, persist results, write artifacts, expose CLI behavior, add schema fields, register handlers by default, model side effects, add writes, or change release posture.

## 2. Scope Verification

The phase stayed within the approved explicit-input integration scope.

Implemented:

- `TerminalLocalWorkReportInput::local_check_result_references`;
- `LocalExecutionReportInputs::local_check_result_references`;
- executor report-input pass-through into terminal report generation;
- local check citation construction through `WorkReportCitation::new(...)`;
- `ValidationAndQualityChecks` section population with local check citations;
- summary text for validation-only, local-check-only, combined, and no-reference cases;
- focused terminal report and executor tests;
- documentation updates and implementation report.

No accidental implementation was found for:

- automatic local check execution;
- local check result reference creation from handlers, executor paths, or report helpers;
- local check result persistence;
- local check artifact writing;
- EvidenceReference attachment;
- `EvidenceKind::CommandOutput`;
- command-output evidence policy changes;
- default `DocsCheck` registration;
- CLI exposure;
- workflow schema fields;
- automatic report generation for every run;
- automatic report artifact writing from executor paths;
- side-effect boundary implementation;
- source writes;
- write-capable adapters;
- release posture changes.

## 3. Input Boundary Assessment

The input boundary is explicit and compatible with the current WorkReport generation posture.

`TerminalLocalWorkReportInput` now accepts `local_check_result_references: Vec<WorkReportStableReference>`. This matches the already-implemented `WorkReportCitationTarget::LocalCheckResult { reference: WorkReportStableReference }` shape and avoids coupling terminal report generation to the full `LocalCheckResultReference` model.

`LocalExecutionReportInputs` also accepts the same stable-reference vector and passes it through to the terminal report helper. The debug implementation reports only a count, not the reference values.

The helper does not read hidden global state, does not use runtime config, does not require a state backend for reference lookup, and does not call local check handlers or process runners.

## 4. Citation Construction Assessment

The citation path is correct.

Verified:

- supplied stable references are converted to `WorkReportCitationTarget::LocalCheckResult`;
- citations are constructed through `WorkReportCitation::new(...)`;
- bounded generic summary text is used;
- no raw output or check summaries are copied into citation summaries;
- validation diagnostic citations are preserved;
- local check citations and validation diagnostic citations share `ValidationAndQualityChecks`;
- citation ordering is deterministic: validation citations first, local check citations second.

Because `WorkReportStableReference` is already validated at construction and deserialization boundaries, the integration does not add a new raw string acceptance path.

## 5. Section Population Assessment

`ValidationAndQualityChecks` now handles the relevant input states:

- validation diagnostics only;
- local check result references only;
- both validation diagnostics and local check result references;
- neither supplied.

The section summary remains bounded and non-payload-bearing. The no-reference case is explicit: `No validation diagnostic or local check result references were supplied.`

No domain-specific sections are required, and no marketing prose was introduced.

## 6. Workflow Semantics Assessment

Workflow semantics remain unchanged.

Verified:

- `LocalExecutor::execute(...)` remains unchanged.
- `LocalExecutor::execute_with_report(...)` still returns the run plus a report or report-generation error.
- The terminal report helper still borrows the run and does not mutate it.
- The executor pass-through does not append events.
- The helper does not mutate snapshots, event history, state backends, telemetry stores, or artifact stores.
- Report generation remains in-memory and explicit.

The existing executor tests continue to verify event history equality and absence of report artifacts.

## 7. Privacy And Redaction Assessment

The integration is reference-only and does not copy local check output.

Verified:

- no raw stdout;
- no raw stderr;
- no command transcripts;
- no environment values;
- no docs contents;
- no parser payloads;
- no provider payloads;
- no CI logs;
- no credentials;
- no tokens;
- no authorization headers;
- no private keys.

`WorkReportStableReference` and `WorkReportCitation` retain existing validation and redaction-safe debug behavior. Serialization may contain valid stable references, which is acceptable for a citation pointer, but tests verify that raw command-output markers are not serialized.

## 8. Error Handling Assessment

The implementation relies on existing validation boundaries:

- `WorkReportStableReference::new(...)` rejects invalid or secret-like references before they can be supplied;
- `WorkReportCitation::new(...)` validates summaries, redaction metadata, and sensitivity;
- report construction errors remain structured `WorkflowOsError` values;
- executor-integrated report generation errors remain separate from workflow execution success/failure.

No misleading project diagnostics are introduced.

One limitation is that invalid local check references are difficult to pass into this API because the field is strongly typed as `WorkReportStableReference`. That is good API design, but a future serde-facing surface should add an explicit invalid serialized local-check-reference regression test before schema or artifact exposure.

## 9. Relationship To Local Check Execution

The integration does not execute local checks.

It does not call:

- `DocsCheckLocalHandler`;
- test-only local check handlers;
- process runners;
- shell commands;
- `npm`;
- `cargo`;
- `workflow-os`.

It also does not create `LocalCheckResult` or `LocalCheckResultReference` values. Callers must supply stable references that already exist.

## 10. Relationship To EvidenceReference

The integration does not create or attach EvidenceReference values.

This is correct. Local check report citations are not command-output evidence. `EvidenceKind::CommandOutput` remains deferred because it needs a separate policy review to avoid turning WorkReports or EvidenceReference into raw log storage.

## 11. Test Quality Assessment

The test coverage is appropriate for this phase.

Covered:

- generated reports cite local check result references by stable reference;
- local check citations use `WorkReportCitationKind::LocalCheckResult`;
- local check citations use `WorkReportCitationTarget::LocalCheckResult`;
- validation diagnostic citations are preserved alongside local check citations;
- no-reference reports retain explicit not-available section text;
- executor-integrated report-bearing execution propagates supplied local check references into the generated report;
- no raw command-output markers are serialized;
- existing WorkReport, WorkReportContract, local check, EvidenceReference, Diagnostic, adapter telemetry, executor, and runtime tests pass.

Non-blocking hardening:

- Add an executor-level report-generation failure test for a secret-like local check reference if a future API ever accepts raw strings or serialized report-input payloads.
- Add a direct helper test for the local-check-only section summary path.

## 12. Documentation Review

Documentation is accurate about the implemented state.

Verified docs say or preserve that:

- terminal report helper integration for supplied local check result references is implemented;
- automatic local check execution is not implemented;
- automatic local check citation wiring from execution is not implemented;
- local check result reference creation from handlers or executor paths is not implemented;
- local check EvidenceReference attachment is not implemented;
- command-output evidence is not implemented;
- local check result persistence is not implemented;
- report artifact writing from this integration is not implemented;
- default registration is not implemented;
- CLI exposure is not implemented;
- workflow schema fields are not implemented;
- side-effect boundary modeling is not implemented;
- writes remain unsupported.

## 13. Blockers

None.

## 14. Non-Blocking Follow-Ups

- Add a local-check-only summary test for `ValidationAndQualityChecks`.
- Add invalid serialized local check citation/reference tests before schema, artifact, or CLI exposure.
- Keep command-output evidence policy separate.
- Keep default handler registration and automatic check execution deferred.

## 15. Recommended Next Phase

Recommended next phase: command-output evidence policy planning only if evidence attachment is needed; otherwise defer and return to the broader roadmap queue.

The local check report citation path is now sufficient for WorkReports to reference supplied check outcomes without copying output. The next risky boundary is evidence treatment for command output, and that should remain planning-only until a concrete evidence need exists.

## 16. Governance Run

This review phase was governed by the self-governance dogfood workflow before the review document was written.

- State directory: `/tmp/workflow-os-terminal-report-local-check-citation-review`
- Run ID: `run-1781543055698883000-2`
- Workflow ID: `dg/d`
- Approval ID: `approval/run-1781543055698883000-2/d`
- Final status: `Completed`

## 17. Validation

Validation commands run for this review:

- `cargo fmt --all --check`
  - Passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Passed.
- `cargo test --workspace`
  - Passed.
- `npm run check:docs`
  - Passed.
