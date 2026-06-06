# WorkReport Core Model Blocker Fix Review

Review date: 2026-06-05

## 1. Executive Verdict

Blocker fixed with non-blocking follow-ups.

The WorkReport redaction metadata blocker is fixed. The implementation adds WorkReport-boundary validation for `RedactionMetadata`, rejects secret-like redaction field names and reasons before storage, validates redaction metadata during deserialization, and changes `WorkReport` and `WorkReportCitation` Debug output to expose only redaction counts rather than caller-supplied metadata strings.

The fix remains model-only. It does not introduce runtime report generation, report artifacts, persistence, CLI behavior, examples, schema changes, reasoning lineage, side-effect modeling, writes, approval evidence attachment, release posture changes, or a broad WorkReport redesign.

## 2. Scope Verification

The fix stayed within the approved blocker-fix scope.

Implemented:

- WorkReport-boundary validation for `RedactionMetadata`;
- bounded redaction field-name validation;
- bounded redaction reason validation;
- secret-like redaction field-name and reason rejection;
- redaction-safe Debug formatting for report-level and citation-level redaction metadata;
- validated deserialization for report/citation redaction metadata through existing constructors;
- focused blocker regression tests;
- fix-forward documentation and a blocker fix report.

No accidental implementation found for:

- runtime report generation;
- report artifacts;
- persistence;
- CLI behavior;
- example updates;
- workflow spec schema changes;
- reasoning lineage implementation;
- side-effect boundary;
- writes;
- approval evidence attachment;
- release posture changes;
- broad WorkReport redesign.

## 3. Original Blocker Restatement

The original blocker was concrete:

- `WorkReport` and `WorkReportCitation` stored `RedactionMetadata` directly.
- `RedactionMetadata` contains public caller-supplied `redacted_fields`, `field_states[].field`, and `field_states[].reason` strings.
- `RedactionMetadata` derives `Debug`, `Serialize`, and `Deserialize`.
- `WorkReport` and `WorkReportCitation` Debug output previously formatted `RedactionMetadata` directly.
- Secret-like redaction field names or reasons could therefore leak through Debug output.
- Serialized report payloads could silently carry secret-like redaction metadata unless the report boundary rejected or sanitized it.
- Deserialization errors could be risky if invalid metadata handling echoed raw values.

## 4. Fix Approach Assessment

The selected approach is WorkReport-boundary validation for the existing `RedactionMetadata` shape.

This is minimal and idiomatic for the current repository:

- it avoids introducing a new public wrapper or schema shape during a blocker fix;
- it keeps valid `RedactionMetadata` serialization compatible for model-only use;
- it preserves existing valid WorkReport and WorkReportCitation construction paths;
- it routes deserialization through the already validated constructors;
- it keeps the fix local to the WorkReport boundary.

The approach is compatible with future persistence/schema exposure as a safe near-term posture. Before public schema exposure, maintainers should still decide whether `RedactionMetadata` should remain the serialized report shape or be wrapped in a dedicated report-safe metadata type.

## 5. Validation Boundary Assessment

Verified validation behavior:

- redaction metadata field names are bounded;
- redaction metadata reasons are bounded;
- too many redacted fields are rejected;
- too many field states are rejected;
- secret-like field names are rejected before storage/output;
- secret-like reasons are rejected before storage/output;
- invalid serialized redaction metadata fails closed through validated deserialization;
- validation errors use stable codes;
- validation errors do not include raw metadata values;
- valid redaction metadata still works for WorkReport and WorkReportCitation.

The fix reuses the existing WorkReport secret-like detection path, so error codes currently use `work_report_contract.secret_like_identifier`. That is stable and non-leaking. A future cleanup may split WorkReport and WorkReportContract error-code namespaces more finely, but this does not block the fix.

## 6. Debug And Serialization Assessment

Verified:

- `WorkReport` Debug does not print caller-supplied redaction metadata field names or reasons.
- `WorkReportCitation` Debug does not print caller-supplied redaction metadata field names or reasons.
- Debug output prints redaction field/state counts only.
- Serialized `WorkReport` cannot silently carry secret-like top-level redaction metadata because construction and deserialization reject it.
- Serialized citation metadata cannot silently carry secret-like redaction metadata because `WorkReportCitation` deserialization uses the validated constructor.
- Deserialization errors do not leak the tested secret-like metadata values.
- Valid redaction metadata remains serializable.

The redaction metadata policy is explicit and testable: bounded, non-secret redaction metadata is allowed; secret-like metadata fails closed; Debug is count-only.

## 7. Privacy/Redaction Assessment

The fix preserves the report privacy boundary.

Verified no storage or introduction of:

- raw provider payloads;
- raw CI logs;
- raw Jira bodies or comments;
- raw GitHub file contents;
- raw command output;
- raw spec contents;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values.

No secret-like redaction field names or reasons leak through WorkReport or WorkReportCitation Debug output. Secret-like redaction metadata is rejected before it can become a valid report/citation value.

## 8. Regression Assessment

Existing behavior remains intact for:

- valid WorkReport construction;
- valid WorkReportCitation construction;
- WorkReportContract model behavior;
- EvidenceReference model behavior;
- Diagnostic evidence attachment;
- validation call-site evidence attachment;
- adapter telemetry evidence attachment;
- runtime tests.

The full workspace test suite passes after the fix.

## 9. Test Quality Assessment

Tests cover the core blocker cases:

- WorkReport rejects secret-like redaction metadata field names;
- WorkReport rejects secret-like redaction metadata reasons;
- WorkReportCitation rejects secret-like redaction metadata field names;
- WorkReportCitation rejects secret-like redaction metadata reasons;
- WorkReport Debug does not leak redaction metadata values;
- WorkReportCitation Debug does not leak redaction metadata values;
- serialized WorkReport payloads do not silently carry secret-like redaction metadata;
- invalid serialized report redaction metadata fails closed;
- deserialization errors do not leak the tested secret-like metadata values;
- valid redaction metadata still works;
- existing WorkReport tests still pass;
- existing WorkReportContract, EvidenceReference, Diagnostic, validation, adapter telemetry, and runtime tests still pass through workspace validation.

Non-blocking test hardening opportunities:

- add a direct invalid serialized `WorkReportCitation` redaction metadata test, separate from full WorkReport payload behavior;
- add explicit too-long redaction field/reason tests;
- add a nested WorkReport section citation deserialization test with invalid citation redaction metadata.

These are not blockers because the code path is visibly validated through `WorkReportCitation::deserialize` and `WorkReportCitation::new`, and the full workspace tests pass.

## 10. Documentation Review

Documentation is aligned with the fix.

Verified docs state:

- WorkReport redaction metadata blocker is fixed;
- WorkReport core model remains model-only;
- runtime report generation is not implemented;
- report artifacts are not implemented;
- persistence is not implemented;
- CLI rendering is not implemented;
- examples are not updated;
- reasoning lineage is not implemented;
- side-effect boundary is not implemented;
- writes remain unsupported.

The historical phase review remains intact and now links forward to the blocker fix report rather than erasing the original finding.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Add direct invalid serialized `WorkReportCitation` redaction metadata coverage.
- Add explicit too-long redaction field-name and reason tests.
- Before persistence or schema exposure, decide whether report redaction metadata should keep the generic `RedactionMetadata` JSON shape or use a dedicated report-safe wrapper.
- Consider splitting the shared `work_report_contract.secret_like_identifier` error code into more specific WorkReport report/citation error codes in a compatibility-reviewed cleanup.
- Continue to defer runtime report generation until terminal local report generation planning is accepted.

## 13. Recommended Next Phase

Recommended next phase: terminal local report generation planning.

The blocker is fixed, the model remains scoped, and no remaining blocker prevents planning the runtime/report-artifact phase. Planning should remain conservative and must not implement runtime report generation, persistence, CLI rendering, examples, schemas, reasoning lineage, side-effect modeling, writes, or release posture changes without separate scoped approval.

## Validation

| Command | Result |
| --- | --- |
| `cargo fmt --all --check` | Passed |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed |
| `cargo test --workspace` | Passed |
| `npm run check:docs` | Passed |
