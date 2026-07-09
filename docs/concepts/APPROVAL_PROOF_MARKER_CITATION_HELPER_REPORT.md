# Approval Proof Marker Citation Helper Report

## 1. Executive Summary

This phase implemented a pure in-memory helper for deriving bounded WorkReport citations from approval decision events that carry approval-presentation proof markers.

The helper consumes an explicit `WorkflowRun` and returns validated `WorkReportCitation` values. It does not mutate runtime state, append events, create evidence references, persist reports, write report artifacts, emit audit records, call providers, or change approval behavior.

## 2. Scope Completed

- Added `ApprovalProofMarkerCitationInput`.
- Added `ApprovalProofMarkerCitationResult`.
- Added `derive_approval_proof_marker_report_citations(...)`.
- Exported the helper API from `workflow-core`.
- Added focused WorkReport tests for proof-marker citation derivation, marker-free compatibility, and fail-closed required-marker behavior.

## 3. Scope Explicitly Not Completed

- No automatic report generation.
- No terminal report helper integration.
- No executor report input propagation.
- No audit projection persistence.
- No report artifact writing.
- No workflow schema changes.
- No CLI rendering changes.
- No examples.
- No provider writes.
- No side-effect execution.
- No hosted behavior.
- No reasoning lineage.
- No release posture changes.

## 4. Helper API Summary

`derive_approval_proof_marker_report_citations(...)` accepts:

- a borrowed `WorkflowRun`;
- whether proof markers are required for all approval decisions;
- whether workflow event citations should be emitted alongside approval decision citations;
- citation sensitivity;
- citation redaction metadata.

It returns:

- approval decision citations for proof-marker approval decisions;
- optional workflow event citations for the events carrying those decisions;
- proof-marker decision count;
- marker-free decision count.

## 5. Behavior Summary

For approval decisions with proof markers, the helper emits `WorkReportCitationTarget::ApprovalDecision` citations. If requested, it also emits `WorkReportCitationTarget::WorkflowEvent` citations for the approval decision event.

For marker-free approval decisions:

- if markers are not required, the helper records marker-free count and emits no proof-marker citations;
- if markers are required, the helper fails closed with `approval_proof_marker_citation.marker_missing`.

The helper does not create `EvidenceReference` values and does not fabricate missing IDs.

## 6. Privacy And Redaction Summary

The helper uses existing `WorkReportCitation` constructors and existing report redaction validation.

It does not copy:

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

Errors are stable and non-leaking.

## 7. Test Coverage Summary

Focused tests cover:

- proof-marker approval decision produces approval and workflow event citations;
- marker-free approval decision remains compatible when proof markers are not required;
- marker-free approval decision fails closed when proof markers are required;
- errors do not leak approval IDs or presentation identifiers.

## 8. Commands Run

- passed: `cargo fmt --all`
- passed: `cargo test -p workflow-core --test work_report approval_proof_marker_citation`
- passed: `cargo fmt --all --check`
- passed: `cargo clippy --workspace --all-targets -- -D warnings`
- passed: `cargo test --workspace`
- passed: `npm run check:docs`
- passed: `git diff --check`
- passed: `npm run dogfood:benchmark -- phase-close run-1783613012645586000-2 --phase implementation`

Governed implementation phase:

- workflow ID: `dg/implement`
- run ID: `run-1783613012645586000-2`
- approval ID: `approval/run-1783613012645586000-2/implementation-approved`
- approval-presentation ID: `presentation/298bfbd3992a5a1a`
- approval-presentation content hash: `298bfbd3992a5a1a8e12d873c7126181c3f3b52493bfff4390d16eefefe2e108`
- approval outcome: granted

## 9. Remaining Known Limitations

- Terminal report helper integration remains future work.
- Audit projection remains future work.
- Report artifact citation behavior remains future work.
- Missing citation records are not added in this helper; missing required markers fail closed when requested.
- Approval-card UI is not implemented.

## 10. Recommended Next Phase

Recommended next phase: approval proof marker citation helper review.

Why: the helper is a new core API adjacent to WorkReport and approval governance. A focused maintainer review should verify scope, privacy, compatibility, and test coverage before integrating these citations into terminal report generation or audit projection paths.
