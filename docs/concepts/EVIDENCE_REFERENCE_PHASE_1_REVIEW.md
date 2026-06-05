# EvidenceReference Phase 1 Review

Review date: 2026-06-04

Reviewed materials:

- `docs/adr/0009-evidence-reference-core-model.md`
- `docs/concepts/evidence-reference.md`
- `docs/implementation-plans/evidence-reference-mvp.md`
- `docs/concepts/EVIDENCE_REFERENCE_REVIEW.md`
- `crates/workflow-core/src/evidence.rs`
- `crates/workflow-core/src/lib.rs`
- `crates/workflow-core/tests/evidence_reference.rs`
- Workflow OS core primitives under `crates/workflow-core/src/`
- security and redaction documentation under `docs/security/`

This review is limited to EvidenceReference Phase 1. It does not approve persistence, CLI inspection, runtime attachment, examples, work reports, reasoning lineage, writes, domain packs, production evidence storage, DLP, access control, or release posture changes.

## 1. Executive Verdict

**Ready for Phase 2 attachment planning.**

The Phase 1 implementation adds a domain-neutral core Rust type model with serialization/deserialization, scope-specific validation, bounded fields, conservative sensitivity behavior, and redaction-safe Debug/Display behavior. The implementation stays inside the approved Phase 1 scope and is backed by focused tests plus full workspace validation.

Phase 2 should remain planning-only until maintainers decide the validation boundary for attachment points. The model currently exposes public fields and a `validate()` method, which is acceptable for Phase 1 but means future adapter/validation/approval attachment code must explicitly validate references before persistence, audit projection, or CLI display.

## 2. Scope Verification

The implementation stayed within Phase 1.

Implemented:

- core `EvidenceReference` Rust type model;
- evidence ID, kind, scope, target, source, sensitivity, retention, redaction metadata, and bounded metadata types;
- typed approval and validation reference IDs for Phase 1 citation needs;
- serialization/deserialization;
- redaction-safe Debug and Display;
- scope-specific validation;
- bounded title, summary, target, and metadata behavior;
- focused safety tests;
- documentation updates stating Phase 1 is implemented and later phases remain proposed.

Not implemented:

- local persistence;
- CLI inspection or new CLI behavior;
- example integration;
- attachment to adapter telemetry;
- attachment to validation results;
- attachment to approval decisions;
- WorkReportContract or WorkReport artifacts;
- Reasoning Lineage / Claim Graph;
- side-effect boundary model;
- write-capable adapters;
- generic live adapter execution;
- domain packs;
- production evidence store;
- DLP;
- access-control systems;
- release posture changes.

No accidental persistence, CLI behavior, runtime attachment, example behavior, write support, domain pack behavior, or release posture change was found.

## 3. Core Model Assessment

| Area | Assessment | Result |
| --- | --- | --- |
| `EvidenceReference` | Captures required and contextual fields without overfitting to GitHub, Jira, CI, or software engineering. Uses existing workflow, run, skill, adapter, event, actor, correlation, timestamp, and hash primitives where available. | Pass |
| `EvidenceReferenceId` | Validated typed string wrapper with generated IDs. It is domain-neutral and consistent with local ID patterns. | Pass |
| `EvidenceKind` | Implements the approved v1 taxonomy with snake_case serialization. Kinds are domain-neutral and broad enough for Phase 1. | Pass |
| `EvidenceScope` | Implements the required scope taxonomy with snake_case serialization. Scope validation covers the high-risk contextual scopes. | Pass |
| `EvidenceReferenceTarget` | Replaces vague `uri_or_reference` with a typed target enum for URI, internal, external, file, opaque, and command-output summary references. This is a good improvement over a free-form field. | Pass |
| `EvidenceSourceComponent` | Domain-neutral source taxonomy covers validator, runtime, adapter, CLI, operator, skill, release review, test, and external. | Pass |
| `EvidenceSensitivity` | Provides public/internal/confidential/regulated/secret/unknown with conservative defaulting to confidential. | Pass |
| `EvidenceRetentionHint` | Non-binding enum is safe because docs do not imply enforcement. | Pass |
| `EvidenceRedactionMetadata` | Reuses core redaction metadata and requires at least one field state. This makes reference-only/no-payload posture explicit. | Pass |
| `EvidenceMetadata` | Bounded map with secret-like value sanitization and redacted Debug. It is appropriately constrained for Phase 1. | Pass |
| validation errors | Use existing `WorkflowOsError` validation/security kinds and avoid including raw payloads in messages. | Pass |

Compatibility note: `EvidenceReference` is exported from `workflow-core`, but it is not yet persisted, exposed through schemas, or surfaced through CLI contracts. Compatibility pressure is therefore limited but should be treated carefully before Phase 2 attachment or persistence.

## 4. Field Behavior Assessment

Required fields are present:

| Field | Assessment |
| --- | --- |
| `id` | Required via `EvidenceReferenceRequiredFields`. |
| `kind` | Required. |
| `title` | Required, bounded, and secret-like values are sanitized. |
| reference target | Required through `EvidenceReferenceTarget`. |
| `source_component` | Required. |
| `scope` | Required. |
| `created_at` | Required. |
| `redaction_metadata` | Required and non-empty. |
| `sensitivity` | Required at stored-model level, with construction defaulting when omitted. |

Optional/contextual fields are present:

- workflow/run identity fields: `workflow_id`, `workflow_version`, `schema_version`, `spec_hash`, `run_id`;
- step/skill fields: `step_id`, `skill_id`, `skill_version`;
- adapter fields: `adapter_id`, `adapter_kind`;
- audit/workflow/approval/validation references: `audit_event_id`, `workflow_event_id`, `approval_id`, `validation_result_id`;
- correlation and actor fields: `correlation_id`, `actor`, `system_actor`;
- content/summary fields: `summary`, `content_hash`, `provider_etag_or_version`;
- lifecycle fields: `retention_hint`, `metadata`.

The constructor uses `EvidenceReferenceRequiredFields`, which avoids a long positional helper and makes required construction input explicit.

## 5. Scope-Specific Validation Assessment

| Requirement | Evidence | Result |
| --- | --- | --- |
| Run-scoped evidence requires immutable run identity. | `validate()` calls `require_run_identity()` for `EvidenceScope::Run`; tests cover missing run identity and valid run evidence. | Pass |
| Step-scoped evidence requires run identity and `step_id`. | `EvidenceScope::Step` requires run identity plus `step_id`; tests cover missing and valid step evidence. | Pass |
| Skill-scoped evidence requires run identity and skill ID/version. | `EvidenceScope::Skill` requires run identity, `skill_id`, and `skill_version`; tests cover missing version and valid skill evidence. | Pass |
| Adapter-scoped evidence requires adapter ID/kind. | `EvidenceScope::Adapter` requires `adapter_id` and `adapter_kind`; tests cover missing and valid adapter evidence. | Pass |
| Audit-scoped evidence requires audit event ID. | `EvidenceScope::Audit` requires `audit_event_id`; tests cover missing audit event ID. | Pass |
| Workflow-event evidence requires event ID. | `EvidenceKind::WorkflowEvent` requires `workflow_event_id`; tests cover missing workflow event ID. | Pass |
| Approval-scoped evidence requires approval reference. | `EvidenceScope::Approval` requires `approval_id`; tests cover missing approval ID. | Pass |
| Validation-scoped evidence requires validation reference. | `EvidenceScope::Validation` requires `validation_result_id`; tests cover missing validation reference. | Pass |
| External reference sensitivity defaults conservatively. | `EvidenceKind::ExternalReference` with public/internal/omitted sensitivity is forced to confidential; tests cover public input defaulting to confidential. | Pass |
| Command output cannot store raw output by default. | Command output requires the command-output target variant and redaction metadata that is not safe-only; secret-like output is sanitized. | Pass with caveat |

Caveat: `CommandOutput` cannot semantically prove that a non-secret-looking string is a summary rather than raw output. The Phase 1 safeguards are type naming, bounded output summary, secret-like sanitization, and required reference-only/redacted metadata. Attachment planning should preserve that posture and avoid passing full command logs into `output_summary`.

## 6. Redaction/Privacy Assessment

| Area | Assessment | Result |
| --- | --- | --- |
| Title bounds | 160-byte bound enforced; test covers oversized title. | Pass |
| Summary bounds | 2,000-byte bound enforced; test covers oversized summary. | Pass |
| Metadata bounds | 32 entries, 64-byte keys, 256-byte values; tests cover oversized value and entry count. | Pass |
| Nested metadata non-leakage | Metadata values are sanitized on construction/deserialization; Debug redacts entire metadata. Tests cover secret-like metadata. | Pass |
| Debug redaction | `EvidenceReference`, `EvidenceReferenceTarget`, and `EvidenceMetadata` Debug paths redact sensitive fields. Tests cover title, summary, metadata, and target non-leakage. | Pass |
| Display redaction | `EvidenceReference` Display uses ID, redacted title, kind, and scope. Test covers secret-like title/target non-leakage. | Pass |
| Serialization safety | Serialization includes sanitized values, not raw secret-like values. Deserialization sanitizes secret-like title, target, summary, provider version, and metadata before reserialization. | Pass |
| Error-message non-leakage | Error paths use generic field names and codes; tests verify secret-like redaction reason does not appear in errors. | Pass |
| `content_hash` and provider version | Provider version setter sanitizes and escalates public/internal sensitivity. Debug redacts provider version. Test covers this. | Pass |
| Raw provider payloads | No raw provider payload storage path was introduced. | Pass |

Important limitation: the secret-like detector is deterministic preview protection, not enterprise DLP. This is appropriate for Phase 1 and aligned with existing redaction posture.

## 7. Test Quality Assessment

The test suite is meaningful and behavior-focused. It covers:

- serialization/deserialization round trip;
- `EvidenceKind` and `EvidenceScope` snake_case taxonomy round trips;
- valid project and run evidence;
- invalid run, step, skill, adapter, audit, validation, approval, and workflow-event combinations;
- conservative external-reference sensitivity defaulting;
- command-output target and redaction restrictions;
- title, summary, metadata value, and metadata count bounds;
- Debug and Display non-leakage;
- serialization non-leakage for secret-like metadata values;
- provider version sensitivity treatment;
- construction redaction metadata requirement and sensitivity defaulting;
- error messages not leaking secret-like payloads;
- deserialization sanitization before reserialization.

No fake or placeholder tests were found. The tests use fixture strings rather than real credentials and do not require live providers.

Minor gap: tests do not cover every `EvidenceReferenceTarget` variant independently for serialization/deserialization. This is not a blocker for attachment planning, but it is a useful follow-up before persistence or schema exposure.

## 8. Documentation Assessment

`docs/concepts/evidence-reference.md` and `docs/implementation-plans/evidence-reference-mvp.md` correctly state:

- Phase 1 core model is implemented;
- persistence is not implemented;
- CLI inspection is not implemented;
- attachment points are not implemented;
- example integration is not implemented;
- WorkReportContract and WorkReport artifacts are not implemented;
- Reasoning Lineage is not implemented;
- side-effect boundary modeling is not implemented;
- writes are not implemented;
- production evidence storage, DLP, and access control are not implemented.

Documentation remains honest about the current implementation boundary. ADR 0009 still reads as proposed and includes an explicit implementation statement saying no feature is implemented by the ADR. That is acceptable because the ADR itself did not implement the feature, but maintainers may want a later status/alignment pass if ADR 0009 is accepted after this review.

## 9. Blockers

No blockers before Phase 2 attachment planning.

Do not begin attachment implementation until the attachment plan defines:

- where `EvidenceReference::validate()` is enforced;
- whether attachment APIs accept only pre-validated references or validate internally;
- whether public-field mutability remains acceptable before persistence;
- how command-output summaries are prevented from becoming raw log payloads;
- what compatibility expectations apply once evidence references are attached to persisted records.

These are planning requirements, not Phase 1 blockers.

## 10. Non-Blocking Follow-Ups

- Add targeted tests for all `EvidenceReferenceTarget` variants before persistence or schema exposure.
- Consider whether `EvidenceReference` fields should remain public or move toward private fields/builders before persistence.
- Consider a dedicated validation/newtype for provider `ETag` or version references if provider metadata becomes more prominent.
- Decide whether `EvidenceScope::Policy` needs a dedicated policy decision reference before policy attachment.
- Decide whether ADR 0009 should move from Proposed to Accepted after this implementation review.
- In Phase 2 planning, split attachment work into adapter telemetry first, validation diagnostics second, and approval/audit citations only after boundaries are clear.

## 11. Final Recommendation

Proceed with an **attachment planning ADR or implementation plan**, not direct attachment implementation.

The next prompt should scope EvidenceReference Phase 2 attachment planning for:

1. adapter telemetry references;
2. validation diagnostic/result references;
3. approval decision references if the first two boundaries are clean.

Still do not build:

- local persistence;
- CLI inspection;
- example integration;
- WorkReportContract or WorkReport artifacts;
- Reasoning Lineage / Claim Graph;
- side-effect boundary model;
- write-capable adapters;
- generic live adapter execution;
- domain packs;
- production evidence store;
- DLP;
- access-control systems;
- release posture changes.

## Validation

Commands run:

| Command | Result |
| --- | --- |
| `cargo fmt --all --check` | Passed |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed |
| `cargo test --workspace` | Passed |
| `npm run check:docs` | Passed |

