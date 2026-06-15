# WorkReport Local Check Citation Target Report

Report date: 2026-06-15

## 1. Executive Summary

Workflow OS now has WorkReport citation vocabulary for local check result references.

The phase adds `WorkReportCitationKind::LocalCheckResult` and `WorkReportCitationTarget::LocalCheckResult`, using `WorkReportStableReference` to cite an existing local check result reference without copying raw command output. The implementation does not add terminal report helper integration, automatic citation wiring, EvidenceReference attachment, command-output evidence, persistence, artifacts, CLI behavior, workflow schema fields, default handler registration, side-effect modeling, source writes, or release posture changes.

## 2. Governance Run

This implementation phase was governed by the self-governance dogfood workflow before code changes.

- State directory: `/tmp/workflow-os-work-report-local-check-citation-target-impl`
- Run ID: `run-1781540673680117000-2`
- Workflow ID: `dg/d`
- Approval ID: `approval/run-1781540673680117000-2/d`
- Final status: `Completed`

## 3. Scope Completed

- Added `WorkReportCitationKind::LocalCheckResult`.
- Added `WorkReportCitationTarget::LocalCheckResult`.
- Updated citation-kind mapping.
- Preserved redaction-safe citation target `Debug`.
- Preserved serde tagging with `snake_case`.
- Added focused WorkReport tests for local check result citation vocabulary.
- Updated roadmap and planning docs.

## 4. Scope Explicitly Not Completed

- No terminal report helper integration.
- No automatic local check citation wiring.
- No local check result reference creation from handlers or executor paths.
- No EvidenceReference attachment.
- No `EvidenceKind::CommandOutput` usage.
- No local check result persistence.
- No report artifact writing.
- No default `DocsCheck` registration.
- No CLI exposure.
- No workflow schema fields.
- No automatic local check execution.
- No `AllowlistedHandlerOnly` enablement.
- No side-effect boundary implementation.
- No source writes.
- No release posture change.

## 5. Model Summary

New citation vocabulary:

- `WorkReportCitationKind::LocalCheckResult`
- `WorkReportCitationTarget::LocalCheckResult { reference: WorkReportStableReference }`

The target follows the existing stable-reference pattern used by adapter telemetry and future reasoning lineage citations. It avoids embedding a full `LocalCheckResultReference` in `WorkReportCitationTarget` while still allowing future report sections to cite local check outcomes by stable reference.

## 6. Citation Boundary Summary

Local check result citations cite stable references only.

They do not copy:

- raw stdout;
- raw stderr;
- command transcripts;
- environment values;
- docs contents;
- parser payloads;
- provider payloads;
- CI logs;
- credentials;
- tokens;
- authorization headers;
- private keys.

`WorkReportCitation::new(...)` remains the validation boundary for bounded summaries, redaction metadata, sensitivity, and missing-citation flags.

## 7. Privacy And Redaction Summary

The new target inherits existing WorkReport citation redaction behavior:

- `WorkReportStableReference::new(...)` rejects secret-like references.
- `WorkReportCitationTarget` debug output redacts target references.
- `WorkReportCitation` debug output redacts summaries and redaction metadata.
- serde deserialization reuses existing validated constructors.

Serialization may contain stable local check result references, but it must not contain raw command output or secret-like payload markers.

## 8. Test Coverage Summary

Added focused tests for:

- valid local check result citation target construction;
- citation kind mapping to `LocalCheckResult`;
- serde round trip with `local_check_result` tag;
- secret-like stable reference rejection without leaking values;
- debug output non-leakage;
- serialization not copying raw command-output markers.

Existing WorkReport, WorkReportContract, LocalCheckResultReference, EvidenceReference, Diagnostic, adapter telemetry, executor, and runtime tests remain part of full validation.

## 9. Commands Run And Results

Validation commands for this phase:

- `cargo fmt --all --check`
  - Passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Passed.
- `cargo test -p workflow-core --test work_report`
  - Passed.
- `cargo test --workspace`
  - Passed.
- `npm run check:docs`
  - Passed.

## 10. Remaining Known Limitations

- Terminal report generation helpers do not consume local check result references.
- Local check citations are not automatically wired into generated reports.
- Local check result references are not persisted.
- Evidence attachment remains deferred.
- Command-output evidence remains deferred.
- Default `DocsCheck` registration remains deferred.
- CLI/schema exposure remains deferred.
- Side-effect modeling remains deferred.

## 11. Recommended Next Phase

Recommended next phase: **WorkReport local check citation target review**.

The review should verify that the implementation is vocabulary-only, redaction-safe, serde-safe, and does not introduce terminal report helper integration, automatic citation wiring, EvidenceReference attachment, command-output evidence, persistence, artifacts, CLI behavior, workflow schema fields, default handler registration, side-effect modeling, source writes, or release posture changes.
