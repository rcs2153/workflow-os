# EvidenceReference Adapter Telemetry Attachment Fix Review

Review date: 2026-06-04

Reviewed materials:

- `docs/ENGINEERING_STANDARD.md`
- `docs/concepts/EVIDENCE_REFERENCE_ADAPTER_TELEMETRY_ATTACHMENT_REVIEW.md`
- `docs/concepts/EVIDENCE_REFERENCE_ADAPTER_TELEMETRY_ATTACHMENT_FIX.md`
- `docs/implementation-plans/evidence-reference-attachment-plan.md`
- `docs/concepts/EVIDENCE_REFERENCE_PHASE_1_REVIEW.md`
- `docs/concepts/evidence-reference.md`
- `docs/adr/0009-evidence-reference-core-model.md`
- `crates/workflow-core/src/evidence.rs`
- `crates/workflow-core/src/adapters.rs`
- `crates/workflow-core/tests/adapters.rs`

This review verifies only the blocker fix for adapter telemetry evidence attachment. It does not approve persistence, CLI inspection, example integration, validation diagnostic/result attachment, approval attachment, work reports, reasoning lineage, side-effect boundary modeling, writes, domain packs, schema exposure, generic live adapter execution, production evidence storage, DLP, access control, or release posture changes.

## 1. Executive Verdict

**Blocker fixed; proceed to validation attachment planning.**

The public mutable evidence vector blocker is fixed. `AdapterInvocationRecord` and `AdapterRuntimeAuditRecord` no longer expose public mutable `Vec<EvidenceReference>` fields. Both records expose read-only `evidence_references()` accessors and retain validated attachment APIs. Deserialization validates attached evidence before constructing records, so invalid evidence-bearing serialized payloads fail closed.

No remaining blocker prevents moving to validation diagnostics/results attachment planning.

## 2. Blocker Verification

| Requirement | Evidence | Result |
| --- | --- | --- |
| `AdapterInvocationRecord` no longer exposes a public mutable evidence vector. | `evidence_references: Vec<EvidenceReference>` is private in `crates/workflow-core/src/adapters.rs`. | Pass |
| `AdapterRuntimeAuditRecord` no longer exposes a public mutable evidence vector. | `evidence_references: Vec<EvidenceReference>` is private in `crates/workflow-core/src/adapters.rs`. | Pass |
| Evidence can be added only through validated APIs. | Public mutation APIs are `attach_evidence_reference`, `attach_evidence_references`, and `with_evidence_references`; each routes through `validate_adapter_telemetry_evidence`. | Pass |
| Read-only accessors do not allow mutation. | `evidence_references()` returns `&[EvidenceReference]`, not `&mut Vec<_>`. | Pass |
| Direct mutation bypass is impossible through public API. | The evidence vectors are private to the module. External callers cannot push directly. Tests document compile-time privacy through accessor-only behavior. | Pass |

The original bypass path is closed by Rust privacy plus read-only slice accessors.

## 3. Deserialization Verification

| Requirement | Evidence | Result |
| --- | --- | --- |
| Valid evidence-bearing `AdapterInvocationRecord` deserializes successfully. | `adapter_invocation_evidence_accessor_is_read_only_and_serializes` serializes and deserializes a valid evidence-bearing invocation record. | Pass |
| Invalid evidence-bearing `AdapterInvocationRecord` fails deserialization clearly. | `invalid_adapter_invocation_evidence_payload_fails_deserialization_without_leaking_values` nulls `adapter_id` and asserts rejection with `evidence.scope.adapter_id_required`. | Pass |
| Valid evidence-bearing `AdapterRuntimeAuditRecord` deserializes successfully. | `adapter_runtime_audit_evidence_accessor_is_read_only_and_serializes` serializes and deserializes valid runtime audit evidence. | Pass |
| Invalid evidence-bearing `AdapterRuntimeAuditRecord` fails deserialization clearly. | `invalid_adapter_runtime_audit_evidence_payload_fails_deserialization_without_leaking_values` nulls `adapter_kind` and asserts rejection with `evidence.scope.adapter_kind_required`. | Pass |
| Deserialization sanitizes secret-like values before storage. | Deserialization uses `deserialize_adapter_evidence_references`, which routes through `validate_evidence_references` and `sanitized_for_attachment`. | Pass |
| Deserialization errors do not leak secret-like values. | Invalid deserialization tests assert token/header/raw-payload-like fixture strings are absent from error text. | Pass |

Implementation detail: the field remains serialized as `evidence_references`, preserving the sensible public wire name while preventing unchecked in-memory mutation.

## 4. Attachment API Regression Check

| Behavior | Evidence | Result |
| --- | --- | --- |
| Single valid evidence attachment still works. | `adapter_invocation_record_attaches_one_valid_evidence_reference` and `adapter_runtime_audit_record_attaches_valid_evidence_reference`. | Pass |
| Multiple valid evidence attachment still works. | `adapter_invocation_record_attaches_multiple_evidence_references_atomically`. | Pass |
| Multiple attachment remains atomic. | `adapter_invocation_multiple_attachment_fails_atomically_when_one_reference_is_invalid` proves invalid batch leaves the record empty. | Pass |
| Invalid evidence fails closed. | Invocation and runtime audit invalid attachment tests both assert failure and no evidence attached. | Pass |
| Unsupported evidence kinds remain rejected. | `adapter_invocation_rejects_non_adapter_evidence_kind` rejects `EvidenceKind::ValidationResult` with `adapter.evidence.kind_unsupported`. | Pass |
| `AdapterRuntimeAuditRecord::from_invocation` preserves evidence. | `adapter_runtime_audit_mapping_preserves_invocation_evidence` proves invocation evidence is copied into runtime audit telemetry. | Pass |
| Existing runtime audit mapping behavior remains intact. | Existing adapter telemetry and integration tests still pass. | Pass |

No attachment API regression was found.

## 5. Redaction/Security Check

Verified protections:

- Debug output for evidence-bearing adapter telemetry does not leak token-like titles, target values, summaries, authorization headers, or raw-provider-payload-like values.
- Serialization stores sanitized evidence values and does not leak the tested secret-like strings.
- Deserialization errors do not leak secret-like title or target values.
- Metadata remains bounded and redaction-aware through the EvidenceReference model.
- No token, authorization header, private key, raw CI log, raw Jira body, raw large GitHub file content, or raw provider payload storage path was introduced.

The deterministic preview redaction model remains a preview safeguard, not enterprise DLP. That limitation is unchanged and acceptable for this scoped fix.

## 6. Scope Check

No accidental scope expansion was found.

Not added:

- local persistence;
- CLI behavior;
- example integration;
- validation diagnostic/result attachment;
- approval request/decision attachment;
- work report behavior;
- reasoning lineage;
- side-effect boundary modeling;
- write behavior;
- domain packs;
- schema exposure;
- generic live adapter execution;
- release posture changes.

The fix is limited to adapter telemetry evidence collection privacy, validated attachment, and validated deserialization.

## 7. Test Quality

Test coverage is strong enough for the blocker fix.

Covered:

- privacy/private-field behavior by compile-time API shape and read-only accessor tests;
- read-only accessors for invocation and runtime audit records;
- valid serialization/deserialization for both record types;
- invalid deserialization for both record types;
- mutation bypass prevention for source evidence mutation before attachment;
- runtime audit preservation through `from_invocation`;
- non-leakage through Debug, serialization, and deserialization error text;
- existing write-denial and integration gates.

Accepted limitation:

- Rust compile-time privacy cannot be asserted at runtime except by the code compiling through accessors and no longer compiling through direct field access. The tests document this. This is acceptable because privacy is enforced by the Rust type system.

Non-blocking gap:

- A UI-style compile-fail test would be stronger but would add test harness complexity. It is not necessary before validation attachment planning.

## 8. Remaining Blockers

No remaining blockers.

## 9. Non-Blocking Follow-Ups

- Add fuller parent/child identity assertions before persistence or CLI rendering.
- Consider a validated evidence collection newtype before schema exposure if the model grows more attachment targets.
- Consider defensive revalidation in future mapping code if adapter telemetry records are accepted from untrusted external deserialization paths.
- Keep observability evidence attachment deferred unless a concrete operator-facing need appears.

## 10. Final Recommendation

Proceed to **validation diagnostics/results attachment planning**.

Do not proceed directly to broad attachment implementation. The next step should define the validation/result attachment boundary with the same rules now proven here:

- private collections or validated collection wrappers;
- read-only accessors;
- validated attachment APIs;
- validated deserialization if serialized;
- no raw payload storage;
- no persistence, CLI, examples, approvals, work reports, reasoning lineage, writes, schemas, or release posture changes unless separately scoped.

## Validation

Commands run from the repository root:

| Command | Result |
| --- | --- |
| `cargo fmt --all --check` | Passed |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed |
| `cargo test --workspace` | Passed |
| `npm run check:docs` | Passed |
| `npm run check:integrations` | Passed |

