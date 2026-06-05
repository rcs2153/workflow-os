# WorkReportContract Core Model Phase Review

Review date: 2026-06-05

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The WorkReportContract core model phase stayed within the approved model-only scope. The implementation is domain-neutral, deterministic, serde-backed, redaction-conscious, and does not introduce report generation, runtime behavior, persistence, CLI rendering, workflow spec schema changes, writes, reasoning lineage, side-effect modeling, or release posture changes.

There are no blockers before the next scoped phase.

## 2. Scope Verification

The phase remained within the approved WorkReportContract model-only scope.

Implemented:

- `WorkReportContract`
- `WorkReportContractId`
- `WorkReportContractVersion`
- `WorkReportContractDefinition`
- `WorkReportSectionKind`
- `WorkReportSectionRequirement`
- `WorkReportCitationKind`
- `WorkReportCitationRequirement`
- `WorkReportDisclosureKind`
- `WorkReportDisclosureRequirements`
- `WorkReportSensitivity`
- `WorkReportRedactionPolicy`
- validation and serde support
- focused tests
- documentation updates
- phase report

Not implemented:

- `WorkReport` model
- report generation
- report artifacts
- runtime report generation
- persistence
- CLI behavior
- example updates
- workflow spec schema changes
- reasoning lineage
- approval attachment
- side-effect boundary
- writes
- release posture changes

No accidental scope expansion was found.

## 3. Model Assessment

The model is appropriately minimal and domain-neutral. It defines a contract for future governed report shape without modeling generated report bodies.

Verified:

- WorkReportContract identity is modeled through `WorkReportContractId`.
- Contract version is modeled through `WorkReportContractVersion`.
- Schema version is modeled with the existing `SchemaVersion` primitive.
- Required section requirements are modeled with `WorkReportSectionRequirement`.
- Citation requirements are modeled with `WorkReportCitationRequirement`.
- Redaction policy is modeled with `WorkReportRedactionPolicy`.
- Sensitivity is modeled with `WorkReportSensitivity`.
- Incomplete/deferred disclosure, known limitations disclosure, risk disclosure, and side-effect disclosure are modeled through `WorkReportDisclosureRequirements`.
- Side-effect section requirements are representable without implying write support.

The implementation does not create report sections, concrete citations, generated report status, report identity, or report generation context. That separation is correct for this phase.

## 4. Section Requirement Assessment

All minimal v1 section kinds are representable:

- work performed
- evidence considered
- decisions made
- policy gates evaluated
- approvals
- validation and quality checks
- side effects
- incomplete or deferred work
- known limitations
- risks
- operator handoff notes

The core contract does not require domain-specific sections. Tests verify that engineering or domain-specific names such as pull request, Jira, legal clause, and finance exception are not required by the core contract.

## 5. Validation Assessment

Validation is enforced at construction and deserialization.

Verified validation behavior:

- contract ID is validated
- contract version is validated
- schema version is validated at the contract boundary
- required sections must not be empty
- duplicate section requirements are rejected
- duplicate citation requirements are rejected
- disclosure requirements must be backed by matching section kinds
- side-effect disclosure can be required before write support exists
- validation errors use stable `work_report_contract.*` codes
- validation errors avoid echoing secret-like raw values

The validation shape is conservative and matches the engineering standard's fail-closed posture.

## 6. Privacy And Redaction Assessment

The contract model does not store report bodies or payload-bearing fields.

Verified:

- no raw provider payloads are stored
- no raw CI logs are stored
- no raw Jira bodies are stored
- no raw GitHub file contents are stored
- no raw command output is stored
- no raw spec contents are stored
- no environment variable values are stored
- no credentials, authorization headers, private keys, or token-like values are stored
- `WorkReportContract` Debug output redacts contract ID and version values
- serialization contains contract structure only
- deserialization errors reviewed through invalid payload tests do not echo raw invalid schema-version values

This is appropriate for a contract model. Generated report privacy rules still need their own future review before any report artifact exists.

## 7. Serde And Compatibility Assessment

Serde support is suitable for the current model-only phase.

Verified:

- valid contracts serialize and deserialize
- invalid serialized contracts fail closed through validated deserialization
- field names are stable and readable
- enum variants use snake_case
- private `WorkReportContract` fields prevent unchecked mutation after construction
- no workflow spec schema changes were introduced

Compatibility pressure will increase when report contracts become spec-declared or persisted. That should be handled in a separate schema or persistence phase, not folded into this model review.

## 8. Relationship To EvidenceReference

The model correctly depends on citation requirements without copying evidence payloads.

Verified:

- `WorkReportCitationKind::EvidenceReference` is represented as a citation kind.
- The contract does not create `EvidenceReference` values implicitly.
- The contract does not require persistence or report generation.
- The contract remains compatible with future concrete report citations to evidence references, workflow events, audit events, adapter telemetry, validation diagnostics, approval decisions, policy decisions, and future reasoning lineage nodes.

`WorkReportCitationKind::ReasoningLineageNode` is a future citation vocabulary entry only. It does not implement reasoning lineage.

## 9. Test Quality Assessment

The test coverage is focused and meaningful for the approved scope.

Covered:

- valid minimal contract
- required identity fields
- invalid contract ID
- invalid version
- invalid schema version through serialized payload
- empty required sections
- duplicate section kinds
- all v1 section kinds
- domain-specific sections not required
- citation requirement validation
- duplicate citation requirements
- redaction policy serde
- sensitivity serde
- disclosure requirements
- side-effect section requirement without write support
- serde round trip
- invalid serde failure
- Debug non-leakage
- serialization non-leakage
- existing EvidenceReference, Diagnostic, validation, adapter, runtime, and workspace tests through full workspace validation

No shallow blocker tests were found. Non-blocking test expansion is listed below.

## 10. Documentation Review

Documentation is aligned with the implementation.

Verified docs state:

- WorkReportContract core model is implemented
- WorkReport generation is not implemented
- WorkReport artifacts are not implemented
- runtime generation is not implemented
- persistence is not implemented
- CLI rendering is not implemented
- examples are not updated
- reasoning lineage is not implemented
- side-effect boundary is not implemented
- writes remain unsupported

The concept docs and implementation plan keep the architecture direction separate from runtime authorization.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Add direct serde tests for secret-like `contract_id`, `contract_version`, and `schema_version` payloads to keep error non-leakage obvious at the deserialization boundary.
- Before any schema exposure, decide whether `WorkReportDisclosureRequirements` should serialize as the current `{ "required": [...] }` object or a flatter compatibility shape.
- Before implementing `WorkReport`, review whether `WorkReportCitationKind::ReasoningLineageNode` should stay in v1 citation taxonomy or remain a future-only variant until reasoning lineage planning resumes.
- Consider a small compatibility note before report contracts become workflow-spec-declared or persisted.

These follow-ups do not block the next scoped model phase.

## 13. Recommended Next Phase

Recommended next phase: WorkReport core model.

The next implementation should remain model-only unless a separate plan authorizes more. It should define generated report identity, section shape, citation shape, terminal status binding, redaction/sensitivity posture, and validation without adding runtime generation, persistence, CLI rendering, examples, schemas, reasoning lineage, side-effect modeling, writes, or release posture changes.

## Validation Results

| Command | Result |
| --- | --- |
| `cargo fmt --all --check` | Passed |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed |
| `cargo test --workspace` | Passed |
| `npm run check:docs` | Passed |
