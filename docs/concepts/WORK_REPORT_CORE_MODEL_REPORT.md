# WorkReport Core Model Phase Report

Report date: 2026-06-05

## 1. Executive Summary

The WorkReport core model phase is implemented.

This phase adds a domain-neutral `WorkReport` model to `workflow-core` with typed report identity, contract binding, immutable workflow/run identity, terminal report status, generation context, report sections, citation targets, incomplete/deferred work disclosures, known limitations, risks, handoff notes, sensitivity, redaction metadata, deterministic validation, serde support, redaction-safe Debug behavior, and focused tests.

This phase does not implement runtime report generation, report artifact writing, persistence, CLI rendering, examples, workflow spec schema changes, reasoning lineage, approval evidence attachment, side-effect boundary modeling, writes, domain packs, or release posture changes.

## 2. Scope Completed

Completed scope:

- added `WorkReport` core model;
- added `WorkReportId`;
- added `WorkReportStatus`;
- added `WorkReportSection`;
- added `WorkReportCitation`;
- added `WorkReportCitationTarget`;
- added `WorkReportStableReference`;
- added `WorkReportGenerationContext`;
- added `WorkReportIncompleteWorkDisclosure`;
- added `WorkReportKnownLimitation`;
- added `WorkReportRisk`;
- added `WorkReportHandoffNote`;
- added deterministic validation for required v1 sections, duplicate sections, bounded summaries, citation targets, disclosure summaries, and identity binding;
- added custom deserialization that validates report payloads before construction;
- added redaction-safe `Debug` behavior for report structures;
- added focused Rust tests for validation, serde, citation targets, disclosure types, terminal status vocabulary, and non-leakage;
- updated planning and concept docs.

## 3. Scope Explicitly Not Completed

Not implemented:

- runtime report generation;
- report artifact writing;
- report persistence;
- CLI rendering or report command behavior;
- example updates;
- workflow spec schema changes;
- reasoning lineage;
- approval evidence attachment;
- side-effect boundary model;
- write-capable adapters;
- domain packs;
- production evidence store;
- DLP or access-control system;
- release posture changes.

## 4. Model Types Added

| Type | Purpose |
| --- | --- |
| `WorkReport` | Domain-neutral generated report model for a terminal governed handoff artifact. |
| `WorkReportId` | Validated identifier for a work report. |
| `WorkReportStatus` | Terminal report status vocabulary: completed, failed, canceled, escalated, or blocked. |
| `WorkReportSection` | One bounded domain-neutral report section. |
| `WorkReportCitation` | Citation from a report section to a stable referenced artifact. |
| `WorkReportCitationTarget` | Typed citation target for evidence references, workflow events, audit events, adapter telemetry, validation diagnostics, approval decisions, policy decisions, and future reasoning lineage vocabulary. |
| `WorkReportStableReference` | Bounded stable reference for citation targets without a dedicated core ID yet. |
| `WorkReportGenerationContext` | Workflow/run identity, terminal status, generation actor, generation timestamp, and correlation context. |
| `WorkReportIncompleteWorkDisclosure` | Bounded incomplete/deferred work disclosure. |
| `WorkReportKnownLimitation` | Bounded known limitation disclosure. |
| `WorkReportRisk` | Bounded risk disclosure. |
| `WorkReportHandoffNote` | Bounded operator handoff note. |

The implementation reuses `WorkReportContractId`, `WorkReportContractVersion`, `WorkReportSectionKind`, `WorkReportCitationKind`, `WorkReportSensitivity`, existing workflow identity primitives, `Timestamp`, and `RedactionMetadata`.

## 5. Validation Boundary Summary

Validation is enforced at construction and deserialization.

Validation checks:

- report ID must be valid;
- contract ID and version must be valid;
- workflow ID, workflow version, schema version, spec hash, run ID, generated timestamp, generated actor, and optional correlation ID use existing typed primitives;
- schema version is checked for secret-like values at the report boundary;
- all v1 core section kinds must be present;
- duplicate core sections are rejected;
- section summaries are bounded and secret-like values are rejected;
- citation summaries are bounded and secret-like values are rejected;
- citation targets use stable typed references only;
- report and citation redaction metadata field names and reasons are bounded and secret-like values are rejected;
- disclosure, limitation, risk, and handoff-note summaries are bounded and secret-like values are rejected;
- side-effect sections are representable even though write support does not exist.

Validation errors use stable codes and avoid echoing raw invalid values.

## 6. Citation Model Summary

`WorkReportCitationTarget` supports stable references to:

- `EvidenceReference`;
- workflow events;
- audit events;
- adapter telemetry records;
- validation diagnostics;
- approval decisions;
- policy decisions;
- future reasoning lineage nodes as vocabulary only.

Citations do not copy raw payloads. Missing citations are represented explicitly through the `missing` field on `WorkReportCitation`; the model does not fabricate replacement evidence.

Reasoning lineage citation vocabulary does not implement reasoning lineage, claims, edges, confidence, derivation, or graph storage.

## 7. Redaction/Privacy Summary

The model stores bounded summaries and stable references, not raw evidence payloads.

It does not store:

- raw provider payloads;
- raw CI logs;
- raw Jira descriptions or comments;
- raw GitHub file contents;
- raw command output;
- raw spec contents;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values.

`WorkReport`, section, citation, disclosure, limitation, risk, handoff-note, and generation-context Debug output avoids printing report body text, sensitive identity fields, and caller-supplied redaction metadata values. Serialization includes only validated fields; secret-like summaries, references, and redaction metadata strings are rejected before construction or deserialization succeeds.

## 8. Test Coverage Summary

Focused tests cover:

- valid minimal report;
- required identity accessors;
- invalid report ID;
- invalid contract ID and version;
- invalid workflow/run identity through deserialization;
- invalid schema version through deserialization;
- missing spec hash;
- missing generated timestamp;
- missing generated actor;
- terminal status vocabulary;
- required v1 section taxonomy;
- duplicate section rejection;
- absence of domain-specific required core sections;
- evidence-reference citation target;
- adapter telemetry citation target;
- validation diagnostic citation target;
- approval and policy citation vocabulary without attachment implementation;
- future reasoning lineage citation vocabulary without lineage implementation;
- incomplete/deferred work disclosure;
- known limitation disclosure;
- risk disclosure;
- side-effect section without write support;
- serde round trip;
- invalid serialized payload fail-closed behavior;
- redaction-safe Debug behavior;
- redaction metadata field and reason validation;
- redaction metadata Debug non-leakage;
- invalid serialized redaction metadata fail-closed behavior;
- serialization non-leakage for forbidden raw payload field names.

Existing WorkReportContract, EvidenceReference, Diagnostic, validation, adapter, runtime, and workspace tests are covered by the full workspace validation commands.

## 9. Commands Run And Results

| Command | Result |
| --- | --- |
| `cargo test -p workflow-core --test work_report` | Passed |
| `cargo fmt --all --check` | Passed |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed |
| `cargo test --workspace` | Passed |
| `npm run check:docs` | Passed |

## 10. Remaining Known Limitations

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

## 11. Recommended Next Phase

Recommended next phase: **WorkReport core model blocker fix review**.

The model should be reviewed after the blocker fix before planning runtime terminal local report generation. Do not start report generation, persistence, CLI rendering, examples, schemas, reasoning lineage, side-effect modeling, or writes until the blocker fix is reviewed and a separate scoped phase is approved.
