# WorkReport Core Model Review

## 1. Executive verdict

Needs blocker fixes.

The WorkReport core model phase stayed within the approved model-only scope and implements a useful, domain-neutral generated report vocabulary. The model is not ready to proceed to terminal local report generation planning because `WorkReport` and `WorkReportCitation` currently carry `RedactionMetadata` in ways that are not validated or redaction-safe enough for Debug/serialization guarantees. `RedactionMetadata` contains public caller-supplied strings, so a secret-like field name or reason can enter a report and appear in routine output unless blocked or redacted at the WorkReport boundary.

## 2. Scope verification

The phase stayed within approved model-only scope.

Verified no accidental implementation of:

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
- release posture changes.

The implementation adds Rust core model types, validation, serde behavior, tests, documentation updates, and a phase report only.

## 3. Model assessment

The model is domain-neutral and appropriately minimal for a generated terminal work report model.

Verified modeled concepts:

- `WorkReport`;
- `WorkReportId`;
- terminal report status through `WorkReportStatus`;
- report identity through `WorkReportGenerationContext` and contract identity fields;
- report sections through `WorkReportSection`;
- report citations through `WorkReportCitation`;
- citation targets through `WorkReportCitationTarget`;
- generation context;
- incomplete/deferred work disclosure;
- known limitations;
- risks;
- handoff notes;
- sensitivity;
- redaction metadata.

The model reuses existing primitives and WorkReportContract vocabulary where appropriate. It does not create domain-specific report templates or report generation behavior.

## 4. Identity assessment

`WorkReport` captures the required identity fields:

- `report_id`;
- `report_contract_id`;
- `report_contract_version`;
- `workflow_id`;
- `workflow_version`;
- `schema_version`;
- `spec_hash`;
- `run_id`;
- terminal run status;
- `generated_at`;
- `generated_by`;
- optional `correlation_id`;
- sensitivity;
- redaction metadata.

The identity fields are represented using existing typed primitives where available. Debug output redacts the high-risk identity values in the generation context.

## 5. Section assessment

The minimal v1 section kinds are representable:

- work performed;
- evidence considered;
- decisions made;
- policy gates evaluated;
- approvals;
- validation and quality checks;
- side effects;
- incomplete or deferred work;
- known limitations;
- risks;
- operator handoff notes.

Core validation requires the v1 section set and rejects duplicates. Domain-specific sections are not required by the core model. The model does not yet provide an extension mechanism for domain-specific section kinds; that is acceptable for this phase and should be revisited before domain packs or schema exposure.

## 6. Citation assessment

Citations support stable references to:

- `EvidenceReference` by `EvidenceReferenceId`;
- workflow events;
- audit events;
- adapter telemetry records;
- validation diagnostics;
- approval decisions;
- policy decisions;
- future reasoning lineage nodes as vocabulary only.

The citation model does not copy evidence payloads and does not create `EvidenceReference` values implicitly. Citation summaries are bounded and validated for secret-like content. Missing citations are explicitly representable through the citation `missing` flag. Citation failures do not create fake evidence because invalid references fail construction/deserialization.

The future reasoning lineage target is vocabulary only; no reasoning lineage model or graph behavior is implemented.

## 7. Validation assessment

Validation covers the main model constraints:

- report ID validity;
- contract ID/version validity;
- workflow identity validity through typed primitives and deserialization;
- schema version validity at the report boundary;
- spec hash presence and validity through typed primitives;
- run ID presence and validity through typed primitives;
- terminal status representation;
- generated timestamp presence;
- generated actor presence and validity;
- required v1 sections;
- duplicate section rejection;
- bounded citations;
- incomplete/deferred work disclosure representation;
- known limitation representation;
- risk representation;
- side-effect section representation before write support;
- sensitivity representation;
- stable non-secret validation error codes.

Blocker: redaction metadata is accepted without WorkReport-specific validation or redacted formatting. Because `RedactionMetadata` contains public strings, the report boundary does not yet prove that redaction field names or reasons are non-secret, bounded, or hidden from Debug/serialization.

## 8. Privacy/redaction assessment

The model does not include fields for raw provider payloads, raw CI logs, raw Jira bodies/comments, raw GitHub file contents, raw command output, raw spec contents, environment variable values, credentials, authorization headers, private keys, or token-like values.

The body-bearing report fields are bounded and reject secret-like values:

- section summaries;
- citation summaries;
- incomplete/deferred work disclosures;
- known limitations;
- risks;
- handoff notes;
- stable reference strings.

Blocker: Debug output for `WorkReport` and `WorkReportCitation` includes `RedactionMetadata` directly. `RedactionMetadata` derives Debug/Serialize/Deserialize and has public caller-supplied string fields. A secret-like redaction field name or reason can therefore leak through Debug or default serialization unless the WorkReport model validates or redacts that metadata before storage/output. This violates the phase requirement that Debug output and serialization not leak secret-like test values.

Deserialization errors for the validated report fields are stable and non-secret. The same redaction metadata gap applies to invalid or malicious serialized redaction metadata.

## 9. Serde and compatibility assessment

Valid reports serialize and deserialize. Invalid serialized reports fail closed for the covered validation paths, including duplicate sections and invalid typed identity fields. Field names are sensible for future schema integration, and no workflow spec schema changes were introduced.

Compatibility posture is appropriate for model-only work, with one caveat: before exposing reports through persistence, CLI JSON, schemas, or generated artifacts, the redaction metadata shape must be validated, wrapped, or rendered through a report-safe representation.

## 10. Relationship to WorkReportContract

`WorkReport` uses and aligns with WorkReportContract vocabulary:

- contract ID;
- contract version;
- section kinds;
- citation kinds;
- sensitivity.

The model does not enforce a specific `WorkReportContract` instance beyond local v1 report-shape validation. That is acceptable for this phase because contract-based generation and runtime enforcement are not implemented. Future generation planning should define when a report is validated against a declared contract.

## 11. Relationship to EvidenceReference

`WorkReport` cites evidence by stable `EvidenceReferenceId`. It does not copy evidence payloads, does not create `EvidenceReference` values implicitly, and does not require persistence or CLI rendering.

This preserves the EvidenceReference source-of-truth boundary: evidence references remain citation pointers, and reports remain handoff artifacts.

## 12. Test quality assessment

The WorkReport tests cover:

- valid minimal report construction;
- required identity fields;
- invalid report ID;
- invalid contract ID/version;
- invalid workflow/run identity through deserialization;
- invalid schema version;
- missing spec hash;
- missing generated timestamp;
- missing generated actor;
- terminal statuses;
- required v1 section kinds;
- duplicate sections;
- domain-specific sections not being required;
- EvidenceReference citation target;
- adapter telemetry citation target;
- validation diagnostic citation target;
- approval/policy citation vocabulary;
- future reasoning lineage vocabulary without implementation;
- incomplete/deferred disclosure;
- known limitation;
- risk;
- side-effect section without write support;
- serde round trip;
- invalid serde failure for duplicate sections;
- Debug non-leakage for body text and identity fields;
- serialization non-leakage for forbidden raw payload field names;
- existing WorkReportContract, EvidenceReference, Diagnostic, and validation tests through workspace validation.

Missing blocker-level tests:

- redaction metadata containing secret-like field names or reasons is rejected, sanitized, or redacted;
- `WorkReport` Debug does not leak secret-like redaction metadata;
- `WorkReportCitation` Debug does not leak secret-like redaction metadata;
- serialized WorkReport payloads cannot silently carry secret-like redaction metadata;
- invalid serialized redaction metadata fails closed without leaking values.

These tests should be added with the blocker fix.

## 13. Documentation review

The documentation is honest about the implemented and deferred scope.

Verified docs state:

- WorkReport core model is implemented;
- WorkReportContract core model is implemented;
- runtime report generation is not implemented;
- report artifacts are not implemented;
- persistence is not implemented;
- CLI rendering is not implemented;
- examples are not updated;
- reasoning lineage is not implemented;
- side-effect boundary is not implemented;
- writes remain unsupported.

The docs should remain unchanged until the blocker fix is implemented, except for any future fix note or review link.

## 14. Blockers

1. `WorkReport` and `WorkReportCitation` accept and Debug-format `RedactionMetadata` directly, while `RedactionMetadata` contains public caller-supplied strings and derives Debug/Serialize/Deserialize. This can leak secret-like redaction field names or reasons through Debug or serialized report payloads.

Required fix:

- add WorkReport-boundary validation for `RedactionMetadata`, or introduce a report-safe redaction metadata wrapper;
- ensure redaction metadata strings are bounded and reject secret-like values, or ensure they are never printed/serialized raw in report contexts;
- add tests proving Debug, serialization, and deserialization errors do not leak secret-like redaction metadata.

## 15. Non-blocking follow-ups

- Define contract-instance enforcement before runtime report generation.
- Clarify how `WorkReportStatus` maps to terminal `WorkflowRunStatus` values before reports are generated from runtime state.
- Decide whether and how domain-specific report sections become extensible before domain packs or schemas.
- Decide whether future reasoning lineage citation vocabulary should remain in the first persisted/schema-exposed report shape.
- Add compatibility notes before exposing reports through persistence, CLI JSON, or workflow spec schemas.

## 16. Recommended next phase

WorkReport blocker fix.

The model-only scope is otherwise sound, but terminal local report generation planning should wait until the report redaction metadata boundary is safe. The next prompt should fix the blocker without adding runtime report generation, artifacts, persistence, CLI rendering, examples, schemas, reasoning lineage, side-effect behavior, writes, or release posture changes.

## Fix-forward note

The blocker identified in this review is addressed by the follow-up fix documented in [WorkReport Core Model Blocker Fix Report](WORK_REPORT_CORE_MODEL_BLOCKER_FIX_REPORT.md). This review remains the historical phase review and should not be read as approval for terminal report generation until the blocker fix itself is reviewed.

## Validation

| Command | Result |
| --- | --- |
| `cargo fmt --all --check` | Passed |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed |
| `cargo test --workspace` | Passed |
| `npm run check:docs` | Passed |
