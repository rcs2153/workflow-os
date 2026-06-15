# Terminal Report Local Check Citation Integration Plan

Status: Implemented. The terminal report helper accepts supplied local check result references and cites them in generated in-memory WorkReports. Automatic local check execution, local check reference creation, EvidenceReference attachment, command-output evidence, persistence, artifacts, CLI behavior, workflow schema fields, default handler registration, side-effect modeling, writes, and release posture changes remain unimplemented.

## 1. Executive Summary

Workflow OS now has:

- validated local check result references;
- WorkReport citation vocabulary for local check results;
- an in-memory terminal local WorkReport generation helper;
- executor-integrated report-bearing execution.

The next narrow integration question is how the terminal report helper should accept already-existing local check result references and cite them in generated reports.

This plan recommended adding explicit supplied local check result reference inputs to the terminal report helper and emitting `WorkReportCitationTarget::LocalCheckResult` citations in the `validation and quality checks` section. That integration is now implemented for explicit supplied references only. It does not authorize automatic local check execution, local check reference creation, EvidenceReference attachment, command-output evidence, persistence, artifacts, CLI behavior, schema changes, default handler registration, side-effect modeling, writes, or release posture changes.

## 2. Goals

- Allow generated terminal local WorkReports to cite supplied local check result references.
- Keep local check citations distinct from validation diagnostics.
- Place local check citations in the `validation and quality checks` section.
- Preserve current terminal report generation behavior when no local check references are supplied.
- Preserve workflow semantics and executor result behavior.
- Cite stable local check references only.
- Avoid copying raw stdout, stderr, command transcripts, parser payloads, provider payloads, environment values, or secrets.
- Keep report construction routed through `WorkReportCitation::new(...)`.
- Keep local check execution and report generation separate.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- automatic local check execution;
- local check result reference creation from handlers, executor paths, or report helpers;
- local check result persistence;
- local check result artifact writing;
- EvidenceReference attachment;
- `EvidenceKind::CommandOutput` usage;
- command-output evidence policy changes;
- default `DocsCheck` registration;
- CLI exposure;
- workflow schema fields;
- automatic report generation for every run;
- report artifact writing from executor paths;
- side-effect boundary implementation;
- source writes;
- write-capable adapters;
- hosted or distributed runtime claims;
- release posture changes.

## 4. Governance Check

This planning phase was governed by the self-governance dogfood workflow before documentation edits.

- State directory: `/tmp/workflow-os-terminal-report-local-check-citation-plan`
- Run ID: `run-1781541805633523000-2`
- Workflow ID: `dg/d`
- Approval ID: `approval/run-1781541805633523000-2/d`
- Final status: `Completed`

## 5. Current Baseline

Implemented:

- `LocalCheckResult`;
- `LocalCheckResultId`;
- `LocalCheckResultReference`;
- `LocalCheckResultReferenceDefinition`;
- `WorkReportCitationKind::LocalCheckResult`;
- `WorkReportCitationTarget::LocalCheckResult`;
- `TerminalLocalWorkReportInput`;
- `generate_terminal_local_work_report(...)`;
- `expose_terminal_local_work_report_result(...)`;
- `LocalExecutor::execute_with_report(...)`;
- explicit local report artifact store.

Implemented after this plan:

- `TerminalLocalWorkReportInput` accepts supplied local check result references.
- `terminal_report_citations(...)` builds local check result citations.
- `ValidationAndQualityChecks` receives validation diagnostic citations and local check result citations.
- Generated reports can cite supplied local check outcomes without creating references or copying output.

## 6. First Implementation Target

Implemented first integration: added supplied local check result references to `TerminalLocalWorkReportInput`.

Candidate field:

```rust
pub local_check_result_references: Vec<WorkReportStableReference>
```

Rationale:

- The existing `WorkReportCitationTarget::LocalCheckResult` stores a `WorkReportStableReference`.
- The report helper should not depend on the full `LocalCheckResultReference` model yet.
- Supplied stable references preserve the existing explicit-input posture.
- The helper remains local, deterministic, and in-memory.

The helper does not create local check references. Callers must supply references that already exist.

## 7. Citation Construction Rules

The implementation:

1. accept supplied local check result stable references;
2. construct `WorkReportCitationTarget::LocalCheckResult { reference }`;
3. construct citations through `WorkReportCitation::new(...)`;
4. use bounded generic summary text such as `Local check result reference considered.`;
5. add those citations to the `validation and quality checks` section;
6. preserve existing validation diagnostic citations in the same section.

Rules:

- Do not create `EvidenceReference` values.
- Do not create `LocalCheckResultReference` values.
- Do not fabricate references.
- Do not copy `LocalCheckResult` stdout or stderr summaries.
- Do not copy command transcripts.
- Do not copy environment values.
- Do not copy parser or provider payloads.
- Do not copy docs contents.
- Do not copy CLI output.
- Do not cite raw process output.

## 8. Section Population Policy

Local check result citations belong in `WorkReportSectionKind::ValidationAndQualityChecks`.

When local check references are supplied:

- the section should include validation diagnostic citations and local check citations;
- the section summary should acknowledge validation diagnostics or local check references without enumerating raw values;
- ordering should be deterministic.

When no validation diagnostics or local check references are supplied:

- preserve current explicit not-available text.

Recommended summary behavior:

| Input State | Section Summary |
| --- | --- |
| no validation references and no local check references | `No validation diagnostic or local check result references were supplied.` |
| validation references only | `Validation diagnostic references were supplied.` |
| local check references only | `Local check result references were supplied.` |
| both supplied | `Validation diagnostic and local check result references were supplied.` |

The exact text may differ if the implementation keeps summaries shorter, but it must remain bounded, non-marketing, and non-payload-bearing.

## 9. API Compatibility

Adding a field to `TerminalLocalWorkReportInput` is an API surface change.

Recommended approach:

- add the field in the explicit input struct;
- update all in-repo construction sites and tests;
- keep existing `LocalExecutor::execute_with_report(...)` behavior unchanged unless its report-input wrapper must thread the new field;
- preserve behavior when the new vector is empty;
- do not add workflow schema fields;
- do not add runtime config.

If the implementation wants to avoid broad call-site churn, a helper/default constructor may be considered only if it follows existing repository patterns and does not hide report inputs or introduce runtime config.

## 10. Failure Semantics

Invalid local check result references should fail report construction through existing WorkReport citation validation.

Rules:

- invalid reference fails closed;
- errors use stable codes;
- errors do not leak the rejected reference;
- report generation failure must not change workflow run status;
- executor-integrated report-bearing execution should continue returning the run plus report-generation error if report construction fails after execution;
- citation failure must not append runtime events;
- citation failure must not persist artifacts;
- citation failure must not become a user project diagnostic.

## 11. Privacy And Redaction

The future implementation must preserve current WorkReport privacy boundaries.

It must not store or copy:

- raw stdout;
- raw stderr;
- raw command transcripts;
- process environment values;
- raw CI logs;
- raw docs contents;
- raw spec contents;
- raw parser payloads;
- provider payloads;
- credentials;
- tokens;
- authorization headers;
- private keys;
- secret-like reference values.

Debug output must not leak local check references. Serialization may contain valid stable references because WorkReport citations are explicit reference pointers, but it must not contain raw output or secret-like payloads.

## 12. Relationship To Local Check Execution

This integration must not execute local checks.

The terminal report helper should consume references supplied by a caller. It should not:

- call `DocsCheckLocalHandler`;
- call any process runner;
- read local files;
- run `npm`, `cargo`, or `workflow-os`;
- create local check results;
- create local check result references;
- register local check handlers;
- enable `AllowlistedHandlerOnly`.

That keeps execution authority separate from report construction.

## 13. Relationship To EvidenceReference

This integration must not attach EvidenceReference values.

Local check result citations are report citations, not evidence records. Evidence work remains deferred until a separate policy decides whether local check outcomes should use `ValidationResult`, a new evidence kind, or carefully bounded `CommandOutput` evidence.

## 14. Test Plan For Future Implementation

The implementation added or updated focused tests for:

- terminal report input accepts local check result references;
- generated report cites local check result references in `ValidationAndQualityChecks`;
- generated report preserves validation diagnostic citations when local check citations are also supplied;
- generated report with no local check references behaves as before;
- local check citations use `WorkReportCitationKind::LocalCheckResult`;
- local check citations use `WorkReportCitationTarget::LocalCheckResult`;
- local check citation references are stable references only;
- invalid/secret-like local check references fail without leaking values;
- report generation failure from invalid local check references preserves workflow run status;
- executor-integrated report-bearing execution returns the run plus a non-leaking report error when supplied local check references are invalid;
- no local check execution occurs;
- no local check reference is fabricated;
- no EvidenceReference is created;
- no raw stdout, stderr, command transcript, parser payload, provider payload, environment value, or docs content is copied;
- `Debug` output does not leak local check references or raw output;
- serialization does not contain raw output markers;
- existing WorkReport, LocalCheckResultReference, EvidenceReference, Diagnostic, adapter telemetry, executor, and runtime tests still pass.

## 15. Documentation Requirements For Future Implementation

Docs must say:

- terminal report helper integration for supplied local check result references is implemented, if implemented;
- automatic local check execution is not implemented;
- automatic local check citation wiring is not implemented;
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

## 16. Implementation Summary

Completed implementation:

1. Added a `local_check_result_references` field to `TerminalLocalWorkReportInput`.
2. Added local check citation construction in the terminal report citation helper.
3. Added those citations to the `ValidationAndQualityChecks` section.
4. Updated executor report input threading to pass through supplied stable references.
5. Added focused tests.
6. Updated docs and created an implementation report.
7. Ran validation.

Remaining sequence:

1. Maintainer review of terminal report helper integration.
2. Command-output evidence policy planning only if evidence attachment is needed.
3. Persistence/artifact integration only after separate planning.
4. CLI/schema/default registration only after authority and side-effect posture are reviewed.

## 17. Open Questions

- Should the input field use `WorkReportStableReference` or a newtyped local check report reference in the future?
- Should local check citations be ordered after validation diagnostic citations or before them?
- Should a section with local check references but no diagnostics say `validation and local check references` or `quality check references`?
- Should executor-integrated report inputs expose local check references immediately, or should only the lower-level terminal helper support them first?
- Should missing local check references become explicit missing citations later?
- Should local check result references eventually become persisted artifacts?
- Should local check result references ever become EvidenceReference values?
- Should a future default report contract require local check citations when local checks are run?

## 18. Final Recommendation

Proceed next with **terminal report local check citation integration review**.

The review should verify the implementation remains in-memory, explicit-input only, and reference-only. It should not execute checks, create references, attach evidence, persist results, write artifacts, expose CLI behavior, add schema fields, register handlers by default, model side effects, add writes, or change release posture.
