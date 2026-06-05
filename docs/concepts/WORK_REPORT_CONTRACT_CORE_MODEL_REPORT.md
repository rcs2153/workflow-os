# WorkReportContract Core Model Phase Report

Report date: 2026-06-05

## 1. Executive Summary

The WorkReportContract core model phase is implemented.

This phase adds a domain-neutral `WorkReportContract` model to `workflow-core` with typed contract identity, contract versioning, v1 required section kinds, citation requirements, redaction policy, sensitivity, disclosure requirement flags, validation, serde support, redaction-safe Debug behavior, and focused tests.

This phase does not implement generated work reports, terminal report artifacts, runtime generation, persistence, CLI rendering, examples, schemas, reasoning lineage, approval evidence attachment, side-effect boundary modeling, writes, domain packs, or release posture changes.

## 2. Scope Completed

Completed scope:

- added `WorkReportContract` core model;
- added `WorkReportContractId`;
- added `WorkReportContractVersion`;
- added `WorkReportContractDefinition`;
- added `WorkReportSectionRequirement`;
- added `WorkReportSectionKind`;
- added `WorkReportCitationRequirement`;
- added `WorkReportCitationKind`;
- added `WorkReportSensitivity`;
- added `WorkReportRedactionPolicy`;
- added `WorkReportDisclosureKind`;
- added `WorkReportDisclosureRequirements`;
- added deterministic validation for required contract fields and disclosure consistency;
- added custom deserialization that validates contract payloads before construction;
- added redaction-safe `Debug` behavior for the contract;
- added focused Rust tests for validation, serde, section kinds, citation requirements, disclosure flags, and non-leakage;
- updated planning and concept docs.

## 3. Scope Explicitly Not Completed

Not implemented:

- `WorkReport` core model;
- generated work report artifacts;
- runtime report generation;
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

The phase adds:

| Type | Purpose |
| --- | --- |
| `WorkReportContract` | Domain-neutral contract for future governed work reports. |
| `WorkReportContractDefinition` | Named construction input for validated contract creation. |
| `WorkReportContractId` | Validated identifier for a report contract. |
| `WorkReportContractVersion` | Validated version identifier for a report contract. |
| `WorkReportSectionRequirement` | Required report section declaration. |
| `WorkReportSectionKind` | Minimal v1 domain-neutral report section taxonomy. |
| `WorkReportCitationRequirement` | Citation expectation for future report sections. |
| `WorkReportCitationKind` | Citation target kind taxonomy. |
| `WorkReportSensitivity` | Sensitivity classification for contracts and future reports. |
| `WorkReportRedactionPolicy` | Contract redaction posture for future reports. |
| `WorkReportDisclosureKind` | Disclosure category taxonomy for incomplete/deferred work, known limitations, risks, and side effects. |
| `WorkReportDisclosureRequirements` | Typed disclosure requirement collection that avoids boolean-heavy contract construction. |

The implementation does not add `WorkReport`, `WorkReportId`, report sections, concrete report citations, or report generation context.

## 5. Validation Boundary Summary

Validation is enforced at construction and deserialization.

Validation checks:

- contract ID must be valid;
- contract version must be valid;
- schema version must be valid through the existing `SchemaVersion` type;
- required section list must not be empty;
- duplicate section requirements are rejected;
- duplicate citation requirements are rejected;
- disclosure requirements for incomplete/deferred work, known limitations, risks, and side effects require the matching section kind;
- side-effect disclosure can be required even though write support does not exist;
- obvious secret-like text is rejected from new contract ID/version values and schema version values at the contract boundary.

Validation errors use stable codes and avoid echoing raw invalid values.

## 6. Redaction/Privacy Summary

The model remains contract-only and does not store report body text or payloads.

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

`WorkReportContract` Debug output reports identity and shape metadata without printing contract ID or version values. Serialization includes only contract fields and does not include raw payload fields.

## 7. Test Coverage Summary

Focused tests cover:

- valid minimal contract;
- required identity accessors;
- invalid contract ID;
- invalid contract version;
- invalid serialized schema version;
- empty required sections;
- duplicate section requirements;
- v1 required section taxonomy;
- absence of domain-specific core sections;
- citation requirement validation;
- duplicate citation requirements;
- redaction policy serde;
- sensitivity serde;
- incomplete/deferred disclosure section consistency;
- known limitations disclosure section consistency;
- risk disclosure section consistency;
- side-effect disclosure without write support;
- serde round trip;
- invalid serialized payload fail-closed behavior;
- redaction-safe Debug behavior;
- serialization non-leakage for forbidden raw payload field names.

Existing EvidenceReference, Diagnostic, validation, adapter, runtime, and workspace tests are covered by the full workspace validation commands.

## 8. Commands Run And Results

| Command | Result |
| --- | --- |
| `cargo test -p workflow-core --test work_report_contract` | Passed |
| `cargo fmt --all --check` | Passed |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed |
| `cargo test --workspace` | Passed |
| `npm run check:docs` | Passed |

## 9. Remaining Known Limitations

- `WorkReport` is not implemented.
- Terminal report artifact generation is not implemented.
- Runtime report generation is not implemented.
- Report persistence is not implemented.
- CLI report rendering is not implemented.
- Workflow specs cannot declare report contracts yet.
- Examples are not updated to use report contracts.
- Approval evidence attachment is not implemented.
- Reasoning Lineage / Claim Graph remains proposed future direction.
- Side-effect boundary modeling remains future work before writes.
- Writes remain unsupported.

## 10. Recommended Next Phase

Recommended next phase: **WorkReportContract phase review**.

The model should be reviewed before proceeding to the `WorkReport` core model. Do not start report artifact generation, persistence, CLI rendering, examples, schemas, reasoning lineage, side-effect modeling, or writes until the contract model is reviewed and a separate scoped phase is approved.
