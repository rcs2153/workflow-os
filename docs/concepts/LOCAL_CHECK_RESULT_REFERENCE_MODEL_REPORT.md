# Local Check Result Reference Model Report

Report date: 2026-06-15

## 1. Executive Summary

Workflow OS now has a model-only local check result reference layer.

The phase adds stable, validated references for local check outcomes so future WorkReport and evidence planning can cite check results without copying raw command output. The implementation does not add WorkReport citation wiring, evidence attachment, command-output evidence, persistence, artifacts, CLI behavior, workflow schema fields, default handler registration, side-effect modeling, source writes, or release posture changes.

## 2. Governance Run

This implementation phase was governed by the self-governance dogfood workflow before code changes.

- State directory: `/tmp/workflow-os-local-check-result-reference-model`
- Run ID: `run-1781506921714862000-2`
- Workflow ID: `dg/d`
- Approval ID: `approval/run-1781506921714862000-2/d`
- Final status: `Completed`

## 3. Scope Completed

- Added `LocalCheckResultId`.
- Added `LocalCheckResultReference`.
- Added `LocalCheckResultReferenceDefinition`.
- Added validated construction and read-only accessors.
- Added `LocalCheckResultReference::from_result(...)` for explicit conversion from an existing `LocalCheckResult` plus workflow/run/event context.
- Added serde validation for local check result references.
- Added redaction-safe `Debug` output.
- Exported the new model types from `workflow-core`.
- Added focused tests.
- Updated roadmap and planning documentation.

## 4. Scope Explicitly Not Completed

- No WorkReport citation target for local check results.
- No automatic WorkReport citation wiring.
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

## 5. Model Types Added

New model types:

- `LocalCheckResultId`
- `LocalCheckResultReference`
- `LocalCheckResultReferenceDefinition`

`LocalCheckResultReference` captures:

- result ID;
- local check command ID;
- local check command kind;
- local check result status;
- workflow ID;
- run ID;
- optional workflow event ID;
- optional audit event ID;
- optional stable output reference;
- redaction metadata;
- sensitivity.

The reference intentionally does not store stdout summaries, stderr summaries, raw process output, command transcripts, environment values, or provider payloads.

## 6. Validation Boundary Summary

Validation ensures:

- result IDs are bounded safe identifiers;
- output references are bounded safe identifiers;
- secret-like IDs and output references are rejected;
- redaction metadata field names and reasons are bounded;
- secret-like redaction metadata is rejected;
- serde deserialization reuses the validated constructor;
- invalid serialized references fail closed;
- errors use stable codes and do not echo raw caller values.

## 7. Redaction And Privacy Summary

The model is reference-only.

It does not copy:

- raw stdout;
- raw stderr;
- complete command transcripts;
- docs contents;
- parser payloads;
- provider payloads;
- CI logs;
- environment values;
- npm tokens or registry credentials;
- authorization headers;
- private keys;
- token-like values.

`Debug` redacts IDs, event references, output references, and redaction metadata. Serialization can carry stable references, but tests verify it does not silently copy raw output or secret-like payload markers.

## 8. Test Coverage Summary

Added focused tests for:

- valid local check result reference construction;
- deriving a reference from an existing `LocalCheckResult`;
- invalid and secret-like result ID rejection;
- secret-like output reference rejection;
- secret-like redaction metadata rejection;
- debug output non-leakage;
- serialization does not copy raw output;
- serde round trip for valid references;
- invalid serialized reference failure without leaking secret-like values.

Existing local check tests continue to cover local check contracts, bounded local check results, process output redaction, handler behavior, and DocsCheck explicit handler behavior.

## 9. Commands Run And Results

Validation commands for this phase:

- `cargo fmt --all --check`
  - Passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Passed.
- `cargo test -p workflow-core --test local_check`
  - Passed.
- `cargo test --workspace`
  - Passed.
- `npm run check:docs`
  - Passed.

## 10. Remaining Known Limitations

- Local check result references are not yet a `WorkReportCitationTarget`.
- Terminal report generation does not consume local check result references.
- Evidence attachment remains deferred.
- Command-output evidence remains deferred.
- References are not persisted in a local check result store.
- Default DocsCheck registration remains deferred.
- CLI/schema exposure remains deferred.
- Side-effect modeling remains deferred.

## 11. Recommended Next Phase

Recommended next phase: **local check result reference model review**.

The review should verify the model is reference-only, redaction-safe, serde-safe, and does not introduce WorkReport citation wiring, EvidenceReference attachment, persistence, CLI behavior, default handler registration, side-effect modeling, source writes, or release posture changes.
