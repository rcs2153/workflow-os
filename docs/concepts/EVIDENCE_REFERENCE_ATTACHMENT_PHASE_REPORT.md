# EvidenceReference Attachment Phase Report

Report date: 2026-06-04

## 1. Executive Summary

The EvidenceReference Attachment Phase is complete within the approved scope.

This phase preserved adapter telemetry evidence attachment safety after the public mutable collection blocker fix, implemented validated `EvidenceReference` attachment for the core `Diagnostic` model only, and kept all broader attachment/runtime surfaces deferred.

No persistence, CLI rendering, example integration, automatic loader/validator attachment, aggregate validation result evidence, approval attachment, work reports, reasoning lineage, side-effect modeling, writes, schemas, or release posture changes were introduced.

## 2. Scope Completed

Completed:

- Verified adapter telemetry evidence attachment remains private, validated, deserialization-checked, and non-regressed.
- Implemented `Diagnostic` evidence attachment APIs:
  - `attach_evidence_reference(...)`
  - `attach_evidence_references(...)`
  - `with_evidence_references(...)`
  - `evidence_references() -> &[EvidenceReference]`
- Added a private `Diagnostic` evidence collection.
- Added diagnostic-specific evidence validation.
- Added validated deserialization for evidence-bearing diagnostics.
- Preserved empty-diagnostic serialization shape by omitting `evidence_references` when no evidence is attached.
- Added focused diagnostic evidence attachment tests.
- Updated EvidenceReference docs and implementation plans.

## 3. Scope Explicitly Not Completed

Not implemented:

- local persistence;
- CLI inspection or report rendering;
- example integration;
- automatic loader evidence attachment;
- automatic semantic validator evidence attachment;
- `ValidationReport`;
- `ValidationSummary`;
- aggregate validation result wrapper;
- aggregate `ValidationResult` evidence;
- validation success evidence;
- approval request/decision evidence attachment;
- `WorkReportContract`;
- `WorkReport` artifacts;
- Reasoning Lineage / Claim Graph;
- side-effect boundary model;
- write-capable adapters;
- generic live adapter execution;
- domain packs;
- production evidence store;
- DLP;
- access-control systems;
- schema changes;
- release posture changes.

## 4. Adapter Telemetry Attachment Status

Adapter telemetry evidence attachment remains safe after the blocker fix.

Verified status:

- `AdapterInvocationRecord` does not expose a public mutable evidence vector.
- `AdapterRuntimeAuditRecord` does not expose a public mutable evidence vector.
- Both expose read-only `evidence_references()` accessors.
- Evidence can be added only through validated APIs.
- Deserialization validates evidence-bearing payloads.
- Invalid attached evidence fails closed.
- Runtime audit mapping preserves invocation evidence safely.
- Observability evidence attachment remains unimplemented.
- No persistence or CLI behavior was added.

Existing adapter telemetry attachment and integration tests pass.

## 5. Diagnostic Attachment Status

Diagnostic evidence attachment is implemented for the core `Diagnostic` model only.

Supported diagnostic evidence kinds:

- `EvidenceKind::ValidationResult`
- `EvidenceKind::SpecFile`

Supported diagnostic evidence scopes:

- `EvidenceScope::Validation`
- `EvidenceScope::Project`
- `EvidenceScope::Workflow`

Rejected for diagnostic attachment:

- `EvidenceKind::CommandOutput`
- `EvidenceKind::AdapterInvocation`
- `EvidenceKind::AdapterResponseSummary`
- `EvidenceKind::ApprovalDecision`
- `EvidenceKind::PolicyDecision`
- `EvidenceKind::ExternalReference`
- `EvidenceKind::ReleaseReview`
- `EvidenceKind::LiveSmokeEvidence`
- unsupported scopes such as release or run-scoped evidence.

Existing diagnostic behavior remains unchanged when no evidence is attached.

## 6. Validation Boundary Summary

Diagnostic attachment follows the adapter telemetry pattern:

- attachment APIs validate internally;
- attachment stores sanitized, validated clones;
- invalid evidence fails closed;
- invalid evidence is not silently dropped;
- multiple attachment is atomic;
- public direct mutation is impossible through the public API;
- read-only accessors return slices;
- deserialization validates attached evidence before constructing an evidence-bearing diagnostic;
- deserialization errors use stable, non-secret codes and messages.

`EvidenceReference::validate()` remains the shared model validation boundary. The diagnostic-specific attachment validator then restricts the accepted evidence kinds and scopes.

## 7. Redaction/Privacy Summary

This phase preserves the reference-first evidence posture.

Diagnostic attachment does not copy:

- raw spec file contents;
- raw command transcripts;
- parser payloads;
- environment variable values;
- provider payloads;
- secret-like YAML parser output;
- token-like values;
- authorization headers;
- private keys;
- raw CI logs;
- raw Jira descriptions/comments;
- raw GitHub file contents.

`Diagnostic.message` is not copied into `EvidenceReference.summary` by default. `SourceLocation` remains the source of truth for file path, line, column, and document path. Evidence summaries do not duplicate source location by default.

File paths remain potentially sensitive and should continue to follow the repository redaction posture.

## 8. Test Coverage Summary

Added focused tests for:

- diagnostic behavior without evidence;
- single valid evidence attachment;
- multiple valid evidence attachment;
- invalid evidence rejection;
- atomic batch failure;
- read-only accessor behavior;
- validated serialization/deserialization;
- invalid deserialization non-leakage;
- diagnostic field preservation;
- source-location source-of-truth behavior;
- accepted diagnostic evidence kinds;
- rejected command output, adapter, approval, policy, external, release, and live-smoke evidence kinds;
- unsupported scope rejection;
- secret-like title, target, summary, and metadata non-leakage through Debug and serialization.

Existing adapter telemetry, validation, loader, project-spec, read-only adapter, runtime, CLI, and integration tests still pass.

## 9. Commands Run And Results

Commands run from the repository root:

| Command | Result |
| --- | --- |
| `cargo fmt --all --check` | Passed |
| `cargo test -p workflow-core --test diagnostic_evidence` | Passed |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed |
| `cargo test --workspace` | Passed |
| `npm run check:docs` | Passed |
| `npm run check:integrations` | Passed |

## 10. Remaining Known Limitations

- No automatic evidence references are generated by loader or semantic validator call sites.
- `ValidationResult` remains a lightweight wrapper around diagnostics.
- Validation success evidence is not represented.
- Evidence-bearing diagnostics are not persisted through a dedicated evidence store.
- CLI inspect/status does not render evidence references.
- Examples do not demonstrate diagnostic evidence yet.
- Approval evidence attachment is not implemented.
- Work reports are not implemented.
- Reasoning Lineage / Claim Graph is not implemented.
- EvidenceReference remains preview redaction infrastructure, not DLP or access control.
- No writes, generic live adapter execution, schemas, or release posture changes are included.

## 11. Recommended Next Phase

Recommended next phase: **approval evidence attachment planning**.

Rationale:

- Adapter telemetry attachment is implemented and reviewed.
- Diagnostic evidence attachment is implemented and validated.
- Loader/validator automatic call-site attachment should remain deferred until there is a clear consumer for validation evidence.
- WorkReportContract planning will be stronger after approval evidence boundaries are understood, because work reports need to cite both validation evidence and approval decisions.

Still do not build writes, persistence, CLI rendering, examples, WorkReport artifacts, Reasoning Lineage, side-effect boundaries, schemas, or domain packs without separate scoped approval.

