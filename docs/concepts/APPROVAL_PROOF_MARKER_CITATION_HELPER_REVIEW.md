# Approval Proof Marker Citation Helper Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The approval proof marker citation helper is a narrow, pure, in-memory API that prepares future WorkReport and audit citation integration without changing approval semantics, report generation, artifact writing, CLI behavior, schemas, provider writes, hosted behavior, reasoning lineage, or release posture.

## 2. Scope Verification

The phase stayed within the approved helper-only scope.

Implemented:

- `ApprovalProofMarkerCitationInput`;
- `ApprovalProofMarkerCitationResult`;
- `derive_approval_proof_marker_report_citations(...)`;
- `workflow-core` exports;
- focused WorkReport tests;
- implementation report and roadmap/planning documentation updates.

No accidental implementation was found for:

- automatic report generation;
- terminal report helper integration;
- executor report input propagation;
- audit projection persistence;
- report artifact writing;
- workflow schema changes;
- CLI rendering;
- examples;
- provider writes;
- side-effect execution;
- hosted behavior;
- reasoning lineage;
- release posture changes.

## 3. Helper/API Assessment

The helper is appropriately bounded and domain-specific. It accepts an explicit borrowed `WorkflowRun`, caller policy for whether proof markers are required, caller policy for optional workflow-event citations, sensitivity, and redaction metadata.

The result exposes read-only citation slices and bounded counts. `Debug` reports counts only, which is appropriate because citation values may include stable references that should not become casual debug output.

The API aligns with the plan's recommendation to derive citations from workflow event history without creating evidence references or mutating runtime state.

## 4. Citation Behavior Assessment

For proof-marked `ApprovalGranted` and `ApprovalDenied` events, the helper emits:

- `WorkReportCitationTarget::ApprovalDecision`;
- optional `WorkReportCitationTarget::WorkflowEvent`.

For marker-free approval decisions, the helper preserves backward compatibility when proof markers are not required and fails closed with `approval_proof_marker_citation.marker_missing` when required by caller policy.

This matches the plan's boundary: existing marker-free approvals remain valid unless a future caller explicitly requires proof-marker posture.

## 5. Validation And Error Handling Assessment

Validation is deterministic and uses existing `WorkReportCitation` constructors and report redaction metadata validation.

Stable non-leaking error codes are used for:

- missing required marker;
- invalid approval reference;
- citation construction failure.

The missing-marker path does not include raw approval IDs, presentation IDs, content hashes, payloads, command output, provider output, or secret-like values in errors.

## 6. Privacy And Redaction Assessment

The helper does not copy:

- approval-presentation payloads;
- approval handoff text;
- work summaries;
- approved scope;
- strict non-goals;
- validation expectations;
- command output;
- provider payloads;
- source/spec contents;
- credentials, tokens, authorization headers, private keys, or secret-like values.

Citation summaries are short posture strings. Redaction metadata is validated before citation construction. Result `Debug` is count-only.

## 7. Compatibility Assessment

The helper is additive and exported through `workflow-core`.

It does not alter:

- `WorkflowRun` rehydration;
- approval request or decision semantics;
- marker-free approval compatibility;
- terminal report generation;
- executor report result behavior;
- audit projection;
- report artifacts;
- schemas.

## 8. Test Quality Assessment

Focused tests cover:

- proof-marker approval decision citation derivation;
- optional workflow event citation derivation;
- marker-free compatibility when markers are not required;
- fail-closed required-marker behavior;
- stable non-leaking missing-marker errors.

Existing workspace tests passed during implementation, including WorkReport, approval presentation, local executor, runtime event, EvidenceReference, Diagnostic, adapter telemetry, provider write, and catalog tests.

Non-blocking test follow-ups:

- add a focused test for denied approval decisions carrying proof markers;
- add a focused test for invalid approval reference mapping to `approval_proof_marker_citation.reference_invalid`;
- add a focused test that result `Debug` does not expose approval or event identifiers.

These are useful hardening tests, but the accepted behavior is covered well enough for the first helper slice.

## 9. Documentation Review

Documentation now states that the first pure approval proof marker citation derivation helper is implemented.

The docs continue to state that the following remain unimplemented:

- automatic runtime report generation;
- automatic artifact writing from executor paths;
- terminal report helper integration for proof-marker citations;
- audit projection persistence;
- CLI rendering;
- schema exposure;
- examples;
- provider writes;
- hosted behavior;
- reasoning lineage;
- release posture changes.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Add denied-decision focused test coverage.
- Add invalid-reference focused test coverage.
- Add explicit result `Debug` non-leakage test coverage.
- Plan terminal report helper integration using this citation helper.
- Plan audit projection separately before adding any persisted audit record behavior.

## 12. Validation

Implementation validation completed before this review:

- passed: `cargo fmt --all --check`;
- passed: `cargo clippy --workspace --all-targets -- -D warnings`;
- passed: `cargo test --workspace`;
- passed: `npm run check:docs`;
- passed: `git diff --check`.

Review validation:

- passed: `npm run check:docs`;
- passed: `git diff --check`.

Governed review phase:

- workflow ID: `dg/review`;
- run ID: `run-1783614581083551000-2`;
- approval ID: `approval/run-1783614581083551000-2/review-scope-approved`;
- approval-presentation ID: `presentation/e8a675aacc45f0b4`;
- approval-presentation content hash: `e8a675aacc45f0b474838065b98d98f759182d33c5114a343139f3373c6227ad`;
- approval outcome: granted.

## 13. Recommended Next Phase

Recommended next phase: terminal report proof-marker citation integration planning.

Why: the helper is accepted as a safe citation derivation primitive. The next phase should plan how terminal report generation can opt in to citing proof-marked approval decisions without making report generation automatic, changing approval semantics, writing artifacts, adding CLI rendering, changing schemas, or broadening runtime behavior.
