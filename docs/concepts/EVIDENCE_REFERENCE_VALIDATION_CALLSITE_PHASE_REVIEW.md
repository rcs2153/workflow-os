# EvidenceReference Validation Call-Site Phase Review

Review date: 2026-06-04

Reviewed materials:

- `docs/ENGINEERING_STANDARD.md`
- `docs/adr/0009-evidence-reference-core-model.md`
- `docs/concepts/evidence-reference.md`
- `docs/implementation-plans/evidence-reference-mvp.md`
- `docs/implementation-plans/evidence-reference-attachment-plan.md`
- `docs/implementation-plans/evidence-reference-validation-attachment-plan.md`
- `docs/implementation-plans/evidence-reference-validation-callsite-plan.md`
- `docs/concepts/EVIDENCE_REFERENCE_PHASE_1_REVIEW.md`
- `docs/concepts/EVIDENCE_REFERENCE_ATTACHMENT_PHASE_REPORT.md`
- `docs/concepts/EVIDENCE_REFERENCE_ATTACHMENT_PHASE_REVIEW.md`
- `docs/concepts/EVIDENCE_REFERENCE_VALIDATION_CALLSITE_PHASE_REPORT.md`
- EvidenceReference implementation code
- Diagnostic evidence attachment code
- validation call-site attachment code
- related tests under `crates/workflow-core/tests/`
- security and redaction documentation under `docs/security/`

This is a phase-level maintainer review. It does not implement persistence, CLI rendering, examples, approval attachment, work reports, reasoning lineage, side-effect modeling, writes, schemas, or release posture changes.

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups.**

The phase stayed within the approved scope. Schema-version diagnostics with existing source/spec context now attach validated `EvidenceKind::SpecFile` references through the `Diagnostic` attachment API. Validation semantics remain unchanged, and tests cover selected-family evidence plus evidence-free behavior for adjacent diagnostic families.

No blockers were found.

## 2. Scope Verification

The phase stayed within approved scope.

Implemented:

- selected schema-version diagnostics attach `EvidenceReference` values;
- evidence is attached through the validated `Diagnostic` APIs;
- diagnostic code, severity, message, source location, and ordering are preserved;
- YAML parser diagnostics remain evidence-free;
- secret-in-spec diagnostics remain evidence-free;
- semantic diagnostics outside the selected family remain evidence-free;
- docs and the phase report were updated.

No accidental scope expansion was found.

Not added:

- `ValidationReport`;
- `ValidationSummary`;
- aggregate validation result wrapper;
- validation success evidence;
- automatic attachment to every diagnostic;
- persistence;
- CLI rendering;
- example updates;
- approval attachment;
- work reports;
- reasoning lineage;
- side-effect modeling;
- writes;
- schemas;
- release posture changes.

## 3. Selected Call-Site Assessment

The selected diagnostic family is schema-version diagnostics:

- loader-produced `schema_version.missing`;
- loader-produced `schema_version.unsupported`;
- semantic validator `validation.schema_version.unsupported`.

This was an appropriate low-risk first target. These diagnostics already have stable codes, known document paths, and source/spec context. They do not require reading spec contents, copying parser payloads, invoking adapters, using command output, or creating a new validation aggregate model.

`SpecFile` evidence is appropriate because the evidence being cited is the spec file that produced the schema-version diagnostic. This should be useful for future work reports because a report can cite the validation failure source without copying the spec contents.

Review note: semantic `validation.schema_version.unsupported` is wired to attach evidence, but normal public loading usually rejects unsupported schema versions at parser/loader time before semantic validation sees a parsed bundle. That is acceptable and non-blocking; the lower-level wiring is still scoped to the intended family.

## 4. Validation Semantics Assessment

Validation semantics are preserved.

| Requirement | Review result |
| --- | --- |
| Pass/fail behavior unchanged | Pass. Evidence attachment does not change validation decisions. |
| Diagnostic code preserved | Pass. Tests assert schema diagnostic codes and semantic diagnostic ordering. |
| Diagnostic severity preserved | Pass. Tests assert selected schema-version severity remains `Error`. |
| Diagnostic message preserved | Pass. Tests assert the selected message remains the unsupported schema-version message. |
| Source location preserved | Pass. Tests assert `$.schema_version` source document path is preserved. |
| Diagnostic ordering preserved | Pass. Semantic validation ordering test includes existing loader warning and validation diagnostics in stable order. |
| Diagnostics outside selected family unchanged | Pass. YAML parse, secret-in-spec, and unrelated semantic diagnostics remain evidence-free. |
| Existing validation tests pass | Pass. Full workspace test validation passed during review. |

## 5. Evidence Construction Assessment

Evidence construction is appropriate for this phase.

| Area | Review result |
| --- | --- |
| Evidence kind | Pass. Uses `EvidenceKind::SpecFile`. |
| Evidence scope | Pass. Uses `EvidenceScope::Project`, which avoids requiring a validation reference ID or invented run/workflow context. |
| Source/spec target | Pass. Uses `EvidenceReferenceTarget::File` from existing `SourceLocation`. |
| SourceLocation authority | Pass. `SourceLocation` remains the source of truth for file path, line, column, and document path. Evidence summary does not duplicate it. |
| Diagnostic message handling | Pass. `Diagnostic.message` is not copied into `EvidenceReference.summary` by default. |
| Raw file contents | Pass. The helper does not read or copy file contents. |
| YAML/parser payloads | Pass. Parser payloads are not copied, and parser diagnostics remain evidence-free. |
| Command output | Pass. No command-output evidence is used. |
| Sensitivity | Pass. Path-backed evidence uses conservative confidential sensitivity. |
| Redaction metadata | Pass. Uses reference-only redaction metadata for the target. |

The implementation returns the original diagnostic unchanged if source-location evidence construction or attachment fails. That preserves validation semantics and avoids introducing misleading user project diagnostics.

## 6. Error-Handling Assessment

Error handling is conservative and appropriate for optional evidence generation.

Verified:

- evidence construction failure does not become a user project diagnostic;
- invalid evidence is not attached;
- partial evidence is not emitted;
- helper errors are not surfaced into normal validation output;
- error messages from the EvidenceReference model use stable codes and generic messages;
- no path snippets, parser payloads, source excerpts, or secret-like values are copied into evidence errors in this call-site path.

Accepted limitation: because failures leave the original diagnostic unchanged, evidence attachment failures are silent at runtime. This is acceptable for a first optional call-site phase that must preserve validation behavior. Before persistence or report enforcement, maintainers should decide whether internal evidence-construction failures need non-user-facing observability.

## 7. Privacy/Redaction Assessment

The phase preserves the required privacy posture.

Verified:

- no raw spec file contents are copied;
- no raw command transcripts are copied;
- no parser payloads are copied;
- no environment variable values are read or copied;
- no provider payloads are copied;
- no token, authorization header, or private-key leakage path was introduced;
- YAML parse diagnostics remain evidence-free;
- secret-in-spec diagnostics remain evidence-free;
- file-path evidence is classified as confidential;
- redaction metadata marks the target as reference-only;
- Debug, serialization, and deserialization safety continue to be covered by the core EvidenceReference and Diagnostic evidence tests.

The implementation still relies on deterministic preview redaction, not enterprise DLP or access control. That limitation remains documented.

## 8. Test Quality Assessment

Test coverage is sufficient for phase acceptance.

Covered:

- selected schema-version diagnostic family receives evidence;
- evidence kind is `SpecFile`;
- evidence scope is `Project`;
- target references the source/spec file;
- diagnostic code is preserved;
- diagnostic severity is preserved;
- diagnostic message is preserved;
- source location is preserved;
- diagnostic ordering is preserved for an unrelated semantic validation path;
- diagnostics outside the selected family remain unchanged;
- raw schema value is not copied into the evidence target;
- `Diagnostic.message` is not copied into evidence summary by default;
- file-path evidence uses confidential sensitivity;
- redaction metadata is present;
- existing validation, loader, project, EvidenceReference, Diagnostic evidence, adapter telemetry, runtime, CLI, and integration tests pass.

Non-blocking gaps:

- The semantic `validation.schema_version.unsupported` branch is wired but not directly exercised through public validation because loader parsing normally rejects unsupported schema versions earlier.
- There is no explicit test for evidence construction helper failure, because the helper is private and intentionally preserves the original diagnostic on failure.
- There is no direct assertion against evidence Debug output in the call-site test; existing EvidenceReference and Diagnostic evidence tests cover non-leakage paths.

These gaps are not blockers for the narrow phase.

## 9. Documentation Review

Docs are honest and aligned.

Docs state:

- first validation call-site attachment is implemented;
- scope is limited to selected schema-version diagnostics;
- automatic attachment to all diagnostics is not implemented;
- aggregate `ValidationResult` evidence is not implemented;
- validation success evidence is not implemented;
- persistence is not implemented;
- CLI rendering is not implemented;
- examples are not updated;
- approvals, work reports, reasoning lineage, and writes remain unsupported.

Minor wording follow-up: the top of `docs/concepts/evidence-reference.md` still says EvidenceReference is not implemented as “validation-result attachment,” which is true for aggregate `ValidationResult` but can be read quickly as broader validation attachment. The later sections clarify the distinction. This is not a blocker.

## 10. Blockers

No blockers.

## 11. Non-Blocking Follow-Ups

- Clarify the top-level EvidenceReference concept wording so “validation-result attachment” clearly means aggregate `ValidationResult`, not `Diagnostic` or schema-version call-site evidence.
- Add direct test coverage for semantic `validation.schema_version.unsupported` if a future unit-level validator fixture can bypass parser-level schema rejection without weakening public behavior.
- Add a helper-failure unit test only if the helper becomes public to the crate test surface or if future call sites make evidence attachment required.
- Consider project-relative path targets before persistence or CLI rendering so absolute local paths do not become stable public output.
- Review whether the next evidence phase should attach approval evidence or plan `WorkReportContract` first.

## 12. Recommended Next Phase

Recommended next phase: **WorkReportContract planning**.

Reasoning:

- EvidenceReference core model exists.
- Adapter telemetry evidence attachment is implemented and reviewed.
- Diagnostic evidence attachment is implemented and reviewed.
- A low-risk validation call-site now produces evidence.
- The next architectural question is how terminal work reports should cite validation evidence, adapter telemetry evidence, future approval decisions, known limitations, and incomplete work disclosures.

Additional validation call-site expansion and approval evidence attachment should remain available follow-ups, but WorkReportContract planning is now timely because it can define the report citation contract before evidence generation spreads to more call sites.

Still do not build persistence, CLI rendering, examples, approvals, work reports, reasoning lineage, side-effect boundaries, writes, schemas, or release posture changes without separate scoped approval.

## Validation

Commands run from the repository root:

| Command | Result |
| --- | --- |
| `cargo fmt --all --check` | Passed |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed |
| `cargo test --workspace` | Passed |
| `npm run check:docs` | Passed |
| `npm run check:integrations` | Passed |
