# Local Check Result Reference Model Review

Review date: 2026-06-15

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The local check result reference model is narrowly scoped, reference-only, validated, serde-safe, and redaction-safe. No blocker was found.

The next recommended phase is WorkReport citation target planning for local check results.

## 2. Scope Verification

The phase stayed within the approved model-only scope.

Confirmed in scope:

- `LocalCheckResultId`;
- `LocalCheckResultReference`;
- `LocalCheckResultReferenceDefinition`;
- validated constructors;
- read-only accessors;
- `LocalCheckResultReference::from_result(...)`;
- serde validation;
- redaction-safe `Debug`;
- focused tests;
- docs and implementation report.

No accidental implementation was found for:

- WorkReport citation target for local check results;
- automatic WorkReport citation wiring;
- EvidenceReference attachment;
- `EvidenceKind::CommandOutput`;
- local check result persistence;
- local check result artifact writing;
- default `DocsCheck` registration;
- CLI behavior;
- workflow schema fields;
- automatic local check execution;
- `AllowlistedHandlerOnly`;
- side-effect boundary implementation;
- source writes;
- release posture changes.

## 3. Governance Check

This review was governed by the self-governance dogfood workflow.

- State directory: `/tmp/workflow-os-local-check-result-reference-review`
- Run ID: `run-1781507401038298000-2`
- Workflow ID: `dg/d`
- Approval ID: `approval/run-1781507401038298000-2/d`
- Final status: `Completed`

## 4. Model Assessment

The implemented model is appropriately minimal.

`LocalCheckResultId` follows the local check identifier pattern and redacts in `Debug`.

`LocalCheckResultReference` captures:

- result ID;
- command ID;
- command kind;
- result status;
- workflow ID;
- run ID;
- optional workflow event ID;
- optional audit event ID;
- optional stable output reference;
- redaction metadata;
- sensitivity.

This is enough to support future citation planning without forcing a report, evidence, persistence, or artifact model now.

## 5. Reference-Only Boundary Assessment

The reference model does not duplicate local check result summaries.

Verified:

- no stdout summary field;
- no stderr summary field;
- no raw process output;
- no command transcript;
- no environment values;
- no provider payloads;
- no docs/spec contents.

`LocalCheckResultReference::from_result(...)` copies only identity/status fields from `LocalCheckResult`, plus explicit caller-supplied workflow/run/event context. It does not copy `stdout_summary` or `stderr_summary`.

## 6. Validation Assessment

Validation is deterministic and fail-closed.

Verified:

- `LocalCheckResultId` rejects empty, malformed, oversized, and secret-like values through the existing local check identifier validator;
- optional output references are bounded safe identifiers;
- secret-like output references are rejected;
- redaction metadata entry counts are bounded;
- redaction field names are bounded and validated;
- redaction reasons are bounded and secret-like values are rejected;
- serde deserialization goes through the validated constructor;
- invalid serialized references fail closed;
- errors use stable codes and do not echo raw caller values.

Non-blocking follow-up: consider dedicated `local_check_result_reference.*` validation codes in a future polishing phase if the project wants errors to distinguish reference failures from broader local-check failures more precisely.

## 7. Privacy And Redaction Assessment

The privacy posture is sound for this phase.

Verified:

- `Debug` redacts result ID, command ID, workflow ID, run ID, event IDs, output reference, and redaction metadata;
- serialization carries stable reference fields but not raw command output;
- secret-like result IDs, output references, redaction field names, and redaction reasons are rejected;
- deserialization errors do not leak secret-like payloads;
- no raw stdout/stderr, command transcript, environment value, provider payload, parser payload, CI log, token, authorization header, private key, or docs/spec content is stored by the reference model.

Serialization of stable reference IDs remains expected model behavior. This is compatible with future citation and persistence planning because the stored values are references, not payloads.

## 8. Relationship To LocalCheckResult

The model complements `LocalCheckResult` without changing existing local check behavior.

Verified:

- existing `LocalCheckResult` construction remains unchanged;
- existing handler output mapping remains unchanged;
- `SkillOutput::output_ref` remains the existing derived string and is not replaced in this phase;
- no handler automatically creates `LocalCheckResultReference`;
- no executor path is modified to consume the new reference type.

This preserves local executor and DocsCheck behavior.

## 9. Relationship To WorkReport And EvidenceReference

The phase preserves the intended separation.

Verified:

- no `WorkReportCitationTarget::LocalCheckResult` variant was added;
- terminal report helpers do not consume local check result references;
- WorkReport artifacts are unchanged;
- no `EvidenceReference` is created;
- no `EvidenceKind::CommandOutput` usage was added;
- command-output evidence policy remains deferred.

Future WorkReport citation target planning can now build on the reference model without using raw command output.

## 10. Test Quality Assessment

The tests are focused and adequate for the model-only phase.

Covered:

- valid local check result reference construction;
- `from_result(...)` preserves command ID, command kind, and status;
- invalid and secret-like result IDs are rejected without leaking values;
- secret-like output references are rejected without leaking values;
- secret-like redaction metadata fields and reasons are rejected without leaking values;
- `Debug` does not leak stable IDs, output references, or summaries;
- serialization does not copy raw output markers;
- valid serde round trip;
- invalid serialized references fail closed without leaking values;
- existing local check, executor, WorkReport, EvidenceReference, Diagnostic, adapter telemetry, and runtime tests pass.

Non-blocking follow-up: add a direct test for oversized redaction fields/reasons if this model becomes persistence-facing.

## 11. Documentation Review

Docs were updated honestly.

Confirmed:

- local check result reference model is documented as implemented;
- WorkReport citation wiring remains unimplemented;
- EvidenceReference attachment remains unimplemented;
- command-output evidence remains unimplemented;
- persistence remains unimplemented;
- report artifact writing remains unimplemented;
- default registration remains unimplemented;
- CLI exposure remains unimplemented;
- workflow schema fields remain unimplemented;
- side-effect boundary modeling remains unimplemented;
- writes remain unsupported.

A small numbering correction was made in the citation plan's later implementation sequence during this review.

## 12. Blockers

None.

## 13. Non-Blocking Follow-Ups

- Consider dedicated `local_check_result_reference.*` error codes if future consumers need more precise error classification.
- Add oversized redaction metadata tests if references become serialized public/persistence-facing contracts.
- Plan an explicit `WorkReportCitationTarget` variant for local check result references before integrating them into generated reports.
- Keep command-output evidence policy separate from local check result reference modeling.
- Decide later whether `SkillOutput::output_ref` should be derived from `LocalCheckResultReference`.

## 14. Recommended Next Phase

Recommended next phase: **WorkReport citation target planning for local check results**.

The reference model gives future report work a stable object to cite. The next phase should decide whether to add a dedicated `WorkReportCitationTarget::LocalCheckResult`, how it should represent missing local check citations, and how terminal report helpers should eventually consume supplied references without creating evidence, persistence, artifacts, CLI behavior, schema fields, default registration, side-effect modeling, or writes.

## 15. Validation

Validation commands run for this review:

- `cargo fmt --all --check`
  - Passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Passed.
- `cargo test --workspace`
  - Passed.
- `npm run check:docs`
  - Passed.
