# WorkReport Core Model Blocker Fix Report

Report date: 2026-06-05

## 1. Executive Summary

The WorkReport redaction metadata blocker is fixed.

The fix keeps the WorkReport core model in model-only scope. It adds WorkReport-boundary validation for `RedactionMetadata` used by `WorkReport` and `WorkReportCitation`, prevents secret-like redaction field names or reasons from being stored through construction or deserialization, and changes Debug output so caller-supplied redaction metadata values are not printed.

This fix does not implement runtime report generation, report artifacts, persistence, CLI rendering, examples, workflow spec schema changes, reasoning lineage, side-effect boundary modeling, writes, approval evidence attachment, domain packs, or release posture changes.

## 2. Blocker Fixed

The phase review found that `WorkReport` and `WorkReportCitation` accepted and Debug-formatted `RedactionMetadata` directly. `RedactionMetadata` contains public caller-supplied strings, so secret-like field names or reasons could leak through Debug or serialized report payloads.

Fixed behavior:

- redaction metadata field names are bounded;
- redaction metadata reasons are bounded;
- secret-like redaction field names are rejected;
- secret-like redaction reasons are rejected;
- `WorkReport` Debug output shows redaction counts, not redaction values;
- `WorkReportCitation` Debug output shows redaction counts, not redaction values;
- deserialization validates redaction metadata before constructing report values;
- invalid serialized redaction metadata fails closed without echoing secret-like values.

## 3. Implementation Approach

The fix uses the smallest repository-consistent approach: WorkReport-boundary validation for the existing `RedactionMetadata` shape.

No new report schema, persistence model, CLI behavior, or wrapper exposed as public API was added.

Implementation details:

- added bounded validation for `RedactionMetadata.redacted_fields`;
- added bounded validation for `RedactionMetadata.field_states[].field`;
- added bounded validation for `RedactionMetadata.field_states[].reason`;
- reused the existing WorkReport secret-like string detection path;
- called redaction metadata validation from `WorkReport::validate()`;
- called redaction metadata validation from `WorkReportCitation::validate()`;
- changed WorkReport and citation Debug formatting to print only redaction field/state counts.

## 4. Validation Boundary Summary

Validation is enforced at construction and deserialization.

Invalid redaction metadata cannot produce a valid `WorkReport` or `WorkReportCitation`. Deserialization continues to use the validated constructors, so serialized payloads with secret-like redaction metadata fail closed.

Validation errors use stable error codes and do not include raw redaction field names, reasons, token-like values, provider payloads, command output, file contents, paths, or snippets.

## 5. Redaction/Privacy Summary

The fix preserves the WorkReport privacy posture:

- reports store references and bounded summaries, not raw payloads;
- redaction metadata remains allowed when bounded and non-secret;
- caller-supplied redaction metadata values are not printed in Debug output;
- secret-like redaction metadata is rejected before storage;
- invalid serialized redaction metadata fails without leaking the offending value.

The model still does not store raw provider payloads, raw CI logs, raw Jira bodies/comments, raw GitHub file contents, raw command output, raw spec contents, environment variable values, credentials, authorization headers, private keys, or token-like values.

## 6. Test Coverage Summary

Focused blocker regression tests cover:

- `WorkReport` rejecting secret-like redaction metadata field names;
- `WorkReport` rejecting secret-like redaction metadata reasons;
- `WorkReportCitation` rejecting secret-like redaction metadata field names;
- `WorkReportCitation` rejecting secret-like redaction metadata reasons;
- `WorkReport` Debug non-leakage for redaction metadata values;
- `WorkReportCitation` Debug non-leakage for redaction metadata values;
- serialized WorkReport payloads not silently carrying secret-like redaction metadata;
- invalid serialized redaction metadata reasons failing closed without leaking values;
- valid redaction metadata continuing to work for reports and citations.

Existing WorkReport, WorkReportContract, EvidenceReference, Diagnostic, validation, adapter telemetry, runtime, and workspace tests remain covered by the full workspace validation commands.

## 7. Commands Run And Results

| Command | Result |
| --- | --- |
| `cargo test -p workflow-core --test work_report` | Passed |
| `cargo fmt --all --check` | Passed |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed |
| `cargo test --workspace` | Passed |
| `npm run check:docs` | Passed |

## 8. Remaining Known Limitations

- Runtime report generation is not implemented.
- Report artifact writing is not implemented.
- Report persistence is not implemented.
- CLI report rendering is not implemented.
- Workflow specs cannot declare report contracts yet.
- Examples are not updated to use report contracts or reports.
- Approval evidence attachment is not implemented.
- Reasoning Lineage / Claim Graph remains proposed future direction.
- Side-effect boundary modeling remains future work before writes.
- Writes remain unsupported.
- WorkReport validation is not yet enforced against a specific declared `WorkReportContract` instance.

## 9. Recommended Next Phase

Recommended next phase: **WorkReport core model blocker fix review**.

The blocker fix should be reviewed before terminal local report generation planning. Do not start runtime report generation, report artifacts, persistence, CLI rendering, examples, schemas, reasoning lineage, side-effect modeling, writes, or release posture changes until that review approves moving forward.
