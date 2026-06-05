# EvidenceReference Adapter Telemetry Attachment Review

Review date: 2026-06-04

Reviewed materials:

- `docs/adr/0009-evidence-reference-core-model.md`
- `docs/concepts/evidence-reference.md`
- `docs/implementation-plans/evidence-reference-mvp.md`
- `docs/implementation-plans/evidence-reference-attachment-plan.md`
- `docs/concepts/EVIDENCE_REFERENCE_PHASE_1_REVIEW.md`
- `docs/integrations/PHASE_2_ADAPTER_TELEMETRY_MAPPING_REVIEW.md`
- `docs/integrations/adapter-contracts.md`
- `docs/concepts/auditability.md`
- `docs/concepts/observability.md`
- `docs/security/redaction.md`
- `crates/workflow-core/src/evidence.rs`
- `crates/workflow-core/src/adapters.rs`
- `crates/workflow-core/tests/adapters.rs`

This review is limited to EvidenceReference attachment for adapter telemetry records. It does not approve persistence, CLI inspection, example integration, validation diagnostic/result attachment, approval attachment, work reports, reasoning lineage, side-effect boundary modeling, writes, domain packs, schema exposure, generic live adapter execution, DLP, access control, production evidence storage, or release posture changes.

## 1. Executive Verdict

**Needs blocker fixes before next attachment phase.**

The implementation correctly adds explicit, `Result`-returning evidence attachment APIs to `AdapterInvocationRecord` and `AdapterRuntimeAuditRecord`. Those APIs validate internally, fail closed on invalid references, attach multiple references atomically, sanitize secret-like fields before storage, and preserve evidence through runtime audit mapping.

However, the attached evidence list is exposed as a public `Vec<EvidenceReference>` on both target records. That means a caller can bypass the attachment APIs and push invalid or unsanitized evidence directly into the record. This conflicts with the approved attachment rule that callers must not be able to bypass attachment validation.

This is fixable and does not invalidate the overall direction. Before moving to validation diagnostics/results attachment, the adapter telemetry attachment should either make the evidence collection private behind validated accessors, introduce a validated evidence collection/newtype, or otherwise prevent direct mutation from bypassing validation.

## 2. Scope Verification

The implementation stayed limited to adapter telemetry evidence attachment except for one necessary helper addition inside the EvidenceReference model.

Implemented:

- `AdapterInvocationRecord` can attach validated `EvidenceReference` values.
- `AdapterRuntimeAuditRecord` can attach validated `EvidenceReference` values.
- Runtime audit mapping copies invocation evidence references into runtime audit telemetry.
- Attachment accepts only adapter invocation and adapter response summary evidence kinds.
- Attachment sanitizes and validates references internally.
- Tests cover valid, invalid, atomic, redaction, and mapping behavior.
- Docs state adapter telemetry evidence attachment is implemented and later attachment targets remain unimplemented.

Not implemented:

- local persistence changes;
- CLI behavior changes;
- example integration;
- validation diagnostic/result attachment;
- approval request/decision attachment;
- work report behavior;
- reasoning lineage;
- side-effect boundary model;
- write behavior;
- domain packs;
- release posture changes;
- schema exposure;
- generic live adapter execution.

No accidental provider write path, webhook path, OAuth path, generic live adapter execution path, schema exposure, or release posture change was found.

## 3. Attachment API Review

| Area | Assessment | Result |
| --- | --- | --- |
| API naming | `attach_evidence_reference`, `attach_evidence_references`, and `with_evidence_references` are clear and consistent with the intended mutating/new-record distinction. | Pass |
| Result-returning behavior | All attachment APIs return `Result<_, WorkflowOsError>`. | Pass |
| Internal validation | Attachment APIs call `validate_adapter_telemetry_evidence`, which calls `EvidenceReference::sanitized_for_attachment()` and then restricts accepted evidence kinds. | Pass |
| Clone/take ownership after validation | Single attachment stores the sanitized validated clone. Multiple attachment builds a separate validated vector before extending the record. | Pass |
| Atomic multiple attachment | Batch attachment validates all references before extending the record. Tests prove one invalid reference leaves the record unchanged. | Pass |
| Public-field mutation of source evidence | Tests prove a caller cannot validate an `EvidenceReference`, mutate it, and then sneak it through the attachment API. | Pass |
| Bypass through target record fields | `evidence_references` is a public `Vec<EvidenceReference>`, so callers can bypass the attachment APIs by mutating the record directly. | **Blocker** |

The attachment APIs themselves are well-shaped. The blocker is the public mutable collection on the target records, not the validation logic inside the attachment APIs.

## 4. Target Record Review

### AdapterInvocationRecord

`AdapterInvocationRecord` now includes `evidence_references: Vec<EvidenceReference>` and initializes it to an empty list in both success and error constructors. Explicit attachment APIs validate and sanitize evidence before adding it.

Assessment: good model shape, but public field mutability allows bypass.

### AdapterRuntimeAuditRecord

`AdapterRuntimeAuditRecord` now includes `evidence_references: Vec<EvidenceReference>`. `from_invocation` preserves invocation evidence by cloning it into the runtime audit record. It also exposes the same explicit attachment APIs.

Assessment: good mapping behavior, but public field mutability allows bypass.

### Observability Records

`AdapterObservabilityRecord` and `AdapterRuntimeObservabilityRecord` do not carry evidence references.

Assessment: acceptable. Observability should remain concise and metric-like. Keeping evidence on invocation/audit telemetry avoids turning observability records into payload mirrors and matches the attachment plan.

## 5. Evidence Kind Review

Evidence kind handling is correct:

- `EvidenceKind::AdapterInvocation` is accepted for invocation-level evidence.
- `EvidenceKind::AdapterResponseSummary` is accepted for response-summary evidence.
- No provider-specific evidence kinds were introduced.
- Non-adapter evidence kinds are rejected with `adapter.evidence.kind_unsupported`.

Tests prove both accepted kinds attach and that `EvidenceKind::ValidationResult` is rejected when used on adapter telemetry.

## 6. Identity And Provenance Review

Identity is split between the parent telemetry record and attached evidence references.

| Identity/provenance field | Location |
| --- | --- |
| adapter ID | Parent telemetry and adapter-scoped EvidenceReference when supplied. |
| adapter kind | Parent telemetry and adapter-scoped EvidenceReference when supplied. |
| correlation ID | Parent telemetry; EvidenceReference may also carry it. Tests cover direct evidence preservation. |
| workflow ID | Parent telemetry through `run_scope`; EvidenceReference may carry run identity if constructed that way. |
| workflow version | Parent telemetry through `run_scope`; EvidenceReference may carry run identity if constructed that way. |
| schema version | Parent telemetry through `run_scope`; EvidenceReference may carry run identity if constructed that way. |
| spec hash | Parent telemetry through `run_scope`; EvidenceReference may carry run identity if constructed that way. |
| run ID | Parent telemetry through `run_scope`; EvidenceReference may carry run identity if constructed that way. |
| step ID | Runtime audit parent telemetry. |
| skill ID/version | Runtime audit parent telemetry when available. |
| actor/system actor | Parent telemetry; EvidenceReference may carry actor/system actor if constructed that way. |
| policy precheck provenance | Parent telemetry only. |

This split is acceptable for adapter telemetry. EvidenceReferences do not need to duplicate every parent field as long as the parent record remains the attachment context and is persisted/projected together.

Non-blocking gap: tests prove adapter ID/kind and correlation ID preservation on direct evidence and runtime audit mapping preserves the evidence list, but they do not exhaustively assert run/step/skill identity alongside attached evidence. That is acceptable for this phase because those fields remain on the parent telemetry record and existing telemetry mapping tests cover those parent fields.

## 7. Redaction And Privacy Review

The attachment boundary is stronger than a plain `validate()` call because it uses `sanitized_for_attachment()` before storing evidence. That helper sanitizes title, target, summary, provider version, redaction metadata, and metadata, then validates the sanitized clone.

Verified protections:

- provider-token-like strings are redacted in attached evidence debug/serialization tests;
- authorization-header-like strings are redacted;
- raw-provider-payload-like strings are redacted;
- raw-CI-log-like target strings are redacted;
- Jira-body-like summary strings are redacted;
- secret-like metadata values are redacted;
- invalid evidence errors use stable codes and do not include payload values.

Display behavior is indirectly covered by EvidenceReference Phase 1 tests. Adapter telemetry records do not implement a custom `Display`, so the main surfaces are Debug and serialization.

Important limitation: direct public mutation of `evidence_references` can bypass the sanitizing attachment boundary. That is the same blocker identified in the API review.

## 8. Test Quality Review

Coverage is strong for the explicit attachment APIs.

| Required test area | Status |
| --- | --- |
| Single valid evidence attachment | Covered. |
| Multiple valid evidence attachment | Covered. |
| Atomic failure when one reference is invalid | Covered. |
| Invalid evidence rejection | Covered. |
| `AdapterRuntimeAuditRecord` attachment | Covered. |
| Public-field mutation after prior validation | Covered for mutating the source EvidenceReference before calling the API. Not covered for direct mutation of the target record's public evidence vector. |
| Adapter ID/kind preservation | Covered on attached evidence. |
| Correlation ID preservation | Covered on attached evidence. |
| `AdapterInvocation` evidence | Covered. |
| `AdapterResponseSummary` evidence | Covered. |
| Debug/display non-leakage | Debug/serialization covered here; Display covered by Phase 1 EvidenceReference tests. |
| Serialization non-leakage | Covered. |
| Existing write-denial tests still pass | Covered in adapter tests and workspace validation. |
| Existing integration checks still pass | Covered by `npm run check:integrations`. |

Missing blocker-level test:

- A test should prove callers cannot bypass validation by directly mutating `AdapterInvocationRecord.evidence_references` or `AdapterRuntimeAuditRecord.evidence_references`. That test cannot pass while the vectors remain public.

Non-blocking test gaps:

- More exhaustive parent/child identity assertions for run/step/skill fields would be useful before persistence.
- Failure-path adapter telemetry with attached evidence is not separately tested, though constructors initialize evidence lists and the attachment APIs are independent of status.

## 9. Documentation Assessment

Docs now state the important boundaries clearly:

- adapter telemetry evidence attachment is implemented;
- validation diagnostics/results attachment is not implemented;
- approval attachment is not implemented;
- persistence behavior is not changed;
- CLI inspection is not changed;
- examples are not updated;
- work reports are not implemented;
- reasoning lineage is not implemented;
- writes remain unsupported.

The EvidenceReference concept, attachment plan, MVP plan, adapter contracts, auditability, and observability docs remain aligned with the scoped read-only posture. No public preview, production telemetry export, write support, generic runtime adapter execution, WorkReport, or Reasoning Lineage overclaim was found.

## 10. Blockers

Before moving to validation diagnostics/results attachment planning, fix this blocker:

1. **Public evidence vectors allow validation bypass.**
   `AdapterInvocationRecord.evidence_references` and `AdapterRuntimeAuditRecord.evidence_references` are public `Vec<EvidenceReference>` fields. A caller can push invalid or unsanitized evidence directly, bypassing `attach_evidence_reference` and `attach_evidence_references`.

   Required fix options:

   - make the evidence collection private and expose read-only accessors plus validated attachment methods;
   - introduce a validated evidence collection/newtype that controls mutation;
   - or otherwise preserve serialization compatibility while preventing unchecked direct mutation.

   Required tests after fix:

   - direct target-record mutation cannot bypass validation;
   - serialization/deserialization still works for valid evidence-bearing records;
   - existing fixture/integration gates still pass.

## 11. Non-Blocking Follow-Ups

- Add fuller tests showing run, workflow, schema, spec hash, step, skill, and actor identity remain available through the parent telemetry context when evidence is attached.
- Consider whether `AdapterRuntimeAuditRecord::from_invocation` should defensively revalidate copied invocation evidence if the invocation record can arrive from deserialization or external construction.
- Add an explicit failure-path evidence attachment test for `AdapterInvocationRecord::from_error`.
- Decide before persistence whether attached evidence should preserve JSON compatibility via a private field plus accessor or a validated collection wrapper.
- Keep observability evidence attachment deferred unless a concrete operator need appears.

## 12. Final Recommendation

Issue a **blocker fix prompt** before validation diagnostics/results attachment planning.

The narrow adapter telemetry evidence direction is sound, and the explicit APIs are close. The next change should close the validation-bypass gap by preventing unchecked direct mutation of attached evidence collections on adapter telemetry records.

After that fix and validation pass, proceed to validation diagnostics/results attachment planning.

Still do not build:

- local persistence;
- CLI inspection;
- example integration;
- validation attachment before the blocker fix;
- approval attachment;
- work reports;
- reasoning lineage;
- side-effect boundary modeling;
- writes;
- domain packs;
- schema exposure;
- generic live adapter execution;
- production evidence store;
- DLP or access-control systems;
- release posture changes.

## Validation

Commands run from the repository root:

| Command | Result |
| --- | --- |
| `cargo fmt --all --check` | Passed |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed |
| `cargo test --workspace` | Passed |
| `npm run check:docs` | Passed |
| `npm run check:integrations` | Passed |

