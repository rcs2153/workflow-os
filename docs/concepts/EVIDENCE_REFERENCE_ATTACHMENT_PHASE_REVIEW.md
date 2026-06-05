# EvidenceReference Attachment Phase Review

Review date: 2026-06-04

Reviewed materials:

- `docs/ENGINEERING_STANDARD.md`
- `docs/concepts/EVIDENCE_REFERENCE_ATTACHMENT_PHASE_REPORT.md`
- `docs/concepts/EVIDENCE_REFERENCE_PHASE_1_REVIEW.md`
- `docs/concepts/EVIDENCE_REFERENCE_ADAPTER_TELEMETRY_ATTACHMENT_REVIEW.md`
- `docs/concepts/EVIDENCE_REFERENCE_ADAPTER_TELEMETRY_ATTACHMENT_FIX_REVIEW.md`
- `docs/implementation-plans/evidence-reference-attachment-plan.md`
- `docs/implementation-plans/evidence-reference-validation-attachment-plan.md`
- `docs/concepts/evidence-reference.md`
- `docs/adr/0009-evidence-reference-core-model.md`
- EvidenceReference implementation code
- adapter telemetry evidence attachment code
- Diagnostic evidence attachment code
- related tests under `crates/workflow-core/tests/`
- core redaction and security documentation

This is a phase-level maintainer review. It does not implement persistence, CLI rendering, example integration, automatic loader/validator attachment, approval attachment, work reports, reasoning lineage, side-effect modeling, writes, schemas, or release posture changes.

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups.**

The EvidenceReference Attachment Phase stayed within the approved scope. Adapter telemetry evidence attachment remains safe after the prior blocker fix, and `Diagnostic` now supports private, validated, atomic, redaction-safe evidence attachment.

No blockers were found before the next planning phase.

## 2. Scope Verification

The phase stayed within approved scope.

Implemented:

- adapter telemetry evidence attachment safety was preserved;
- `Diagnostic` evidence attachment was implemented;
- attachment boundaries validate internally;
- evidence collections are private with read-only accessors;
- multiple attachment is atomic;
- evidence-bearing deserialization validates attached evidence;
- redaction-safe behavior is tested;
- docs and the phase report were updated.

No accidental scope expansion was found.

Not added:

- local persistence;
- CLI behavior or rendering;
- example updates;
- automatic loader evidence attachment;
- automatic semantic validator evidence attachment;
- `ValidationResult` aggregate evidence;
- validation success evidence;
- approval attachment;
- work reports;
- reasoning lineage;
- side-effect modeling;
- writes;
- schemas;
- release posture changes.

## 3. Adapter Telemetry Attachment Regression Check

Adapter telemetry evidence attachment remains sound.

| Requirement | Review result |
| --- | --- |
| No public mutable evidence vectors | Pass. `AdapterInvocationRecord` and `AdapterRuntimeAuditRecord` keep `evidence_references` private. |
| Read-only accessors | Pass. Both expose `evidence_references() -> &[EvidenceReference]`. |
| Validated APIs remain intact | Pass. `attach_evidence_reference`, `attach_evidence_references`, and `with_evidence_references` remain available. |
| Invalid evidence fails closed | Pass. Invalid adapter evidence returns stable validation errors and does not attach. |
| Multiple attachment remains atomic | Pass. Batch validation completes before extending the record. |
| Deserialization validates attached evidence | Pass. Serde uses `deserialize_adapter_evidence_references`. |
| Runtime audit mapping preserves evidence safely | Pass. `AdapterRuntimeAuditRecord::from_invocation` copies evidence from the validated private invocation record. |
| No new observability evidence noise | Pass. Observability evidence attachment remains unimplemented. |

No adapter telemetry regression was found.

## 4. Diagnostic Attachment Review

Diagnostic evidence attachment is correctly scoped and implemented.

| Requirement | Review result |
| --- | --- |
| Private or validated evidence collection | Pass. `Diagnostic.evidence_references` is private. |
| Read-only accessor exists | Pass. `Diagnostic::evidence_references()` returns a slice. |
| Attachment APIs validate internally | Pass. Attachment routes through `validate_diagnostic_evidence`. |
| Multiple attachment is atomic | Pass. Batch references are validated before extending the collection. |
| Invalid evidence fails closed | Pass. Invalid references return errors and do not attach. |
| Deserialization validates attached evidence | Pass. Serde uses `deserialize_diagnostic_evidence_references`. |
| No-evidence behavior preserved | Pass. Empty diagnostics omit `evidence_references` from serialization and retain existing display behavior. |
| Code/severity/message/source location preserved | Pass. Tests verify these fields remain unchanged after attachment. |
| `SourceLocation` remains source of truth | Pass. Source location stays on `Diagnostic`; evidence summary is not populated by default. |
| `Diagnostic.message` not copied to evidence summary | Pass. No implementation path copies the message into attached evidence. |

Compatibility note: `Diagnostic` keeps a manual `Hash` implementation over the original diagnostic fields. Evidence-bearing diagnostics with different evidence can share a hash even though they are not equal. This is acceptable for Rust hash contracts, but should be revisited before evidence-bearing diagnostics become persisted or used as stable identity keys.

## 5. Evidence Kind/Scope Review

Diagnostic attachment accepts only the intended evidence kinds:

- `EvidenceKind::ValidationResult`
- `EvidenceKind::SpecFile`

Diagnostic attachment accepts only the intended scopes:

- `EvidenceScope::Validation`
- `EvidenceScope::Project`
- `EvidenceScope::Workflow`

Diagnostic attachment rejects:

- `EvidenceKind::CommandOutput`
- `EvidenceKind::AdapterInvocation`
- `EvidenceKind::AdapterResponseSummary`
- `EvidenceKind::ApprovalDecision`
- `EvidenceKind::PolicyDecision`
- `EvidenceKind::LiveSmokeEvidence`
- `EvidenceKind::ReleaseReview`
- `EvidenceKind::ExternalReference`
- unsupported scopes, including release and run-scoped evidence.

The diagnostic-specific validator is the right boundary. It lets the general EvidenceReference model remain reusable while keeping diagnostics from becoming a command log, adapter payload, approval packet, release artifact, or external-reference store.

## 6. Redaction/Privacy Review

The phase preserves the reference-first privacy posture.

Verified:

- raw spec file contents are not copied;
- raw command transcripts are not accepted for diagnostic attachment;
- parser payloads are not copied;
- environment variable values are not copied;
- provider payloads are not copied;
- token-like, authorization-header-like, and private-key-like values are sanitized or rejected by EvidenceReference before attachment;
- raw CI/Jira/GitHub payload-like fixtures do not leak through tested Debug, serialization, or deserialization error paths;
- metadata remains bounded and redaction-aware through `EvidenceMetadata`;
- `Diagnostic.message` is not copied into evidence summaries;
- `SourceLocation` is not duplicated into evidence summaries by default.

Accepted limitation: deterministic preview redaction is still not enterprise DLP or access control. This remains correctly documented.

## 7. Test Quality Review

Test coverage is strong enough for phase acceptance.

Covered:

- adapter telemetry non-regression;
- no public mutable adapter evidence vectors;
- adapter telemetry read-only accessors;
- adapter telemetry valid/invalid attachment;
- adapter telemetry atomic attachment;
- adapter telemetry valid/invalid deserialization;
- adapter runtime audit evidence preservation;
- diagnostic no-evidence behavior;
- single and multiple diagnostic evidence attachment;
- diagnostic atomic failure;
- diagnostic invalid evidence rejection;
- diagnostic read-only accessor behavior;
- diagnostic valid/invalid serde;
- diagnostic field and source-location preservation;
- accepted diagnostic kinds;
- rejected command output, adapter, approval, policy, external, release, and live-smoke kinds;
- unsupported diagnostic scopes;
- non-leakage through Debug, serialization, and deserialization errors;
- existing loader, validator, project-spec, adapter, runtime, CLI, and integration gates.

No fake or shallow tests were found.

Non-blocking test follow-ups:

- Add compile-fail tests only if the repository later adopts a compile-test harness.
- Add an explicit diagnostic hash behavior test before evidence-bearing diagnostics are used as stable identity keys.
- Add more parent/child identity tests before persistence or CLI rendering.

## 8. Documentation Review

Documentation is honest and aligned.

Docs state:

- adapter telemetry evidence attachment is implemented;
- `Diagnostic` evidence attachment is implemented;
- automatic loader/validator attachment is not implemented;
- aggregate `ValidationResult` evidence is not implemented;
- validation success evidence is not implemented;
- approval attachment is not implemented;
- persistence is not implemented;
- CLI rendering is not implemented;
- examples are not updated;
- work reports are not implemented;
- reasoning lineage is not implemented;
- writes remain unsupported.

No overclaim was found.

## 9. Blockers

No blockers.

## 10. Non-Blocking Follow-Ups

- Revisit `Diagnostic` hash semantics before evidence-bearing diagnostics become persisted, schema-exposed, or used as stable identity keys.
- Add compile-fail tests for private evidence collections if a compile-test harness is introduced.
- Decide whether future validation call-site generation should attach evidence automatically or remain operator/report-driven.
- Plan approval evidence attachment before WorkReportContract so report citations can cover both validation evidence and approval decisions.
- Keep command-output evidence deferred for diagnostics until a scoped bounded-command-summary plan exists.

## 11. Recommended Next Phase

Recommended next phase: **approval evidence attachment planning**.

Reasoning:

- Adapter telemetry attachment is implemented, reviewed, fixed, and non-regressed.
- Diagnostic evidence attachment is implemented and phase-reviewed.
- Automatic validation call-site attachment can wait until there is a clear consumer for generated validation evidence.
- WorkReportContract planning will be stronger after approval evidence boundaries are understood, because future work reports need to cite validation evidence and approval decisions without copying raw payloads.

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

