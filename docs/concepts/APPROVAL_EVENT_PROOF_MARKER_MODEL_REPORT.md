# Approval Event Proof Marker Model Report

## 1. Executive Summary

This phase implemented the model-only approval decision proof marker vocabulary.

Workflow OS can now represent bounded, redaction-safe proof-use metadata for future approval decision event payloads. The model records which approval-presentation proof a future approval decision can cite without copying approval handoff text or changing runtime approval behavior.

This phase did not wire proof markers into runtime events, change approval semantics, add inspect/projection behavior, add approval-card UI, add schemas, add examples, enable writes, add hosted behavior, implement reasoning lineage, or change release posture.

## 2. Scope Completed

- Added `ApprovalDecisionProofMarker`.
- Added `ApprovalDecisionProofMarkerDefinition`.
- Added `ApprovalDecisionProofEnforcementMode`.
- Added `ApprovalDecisionProofValidationPolicy`.
- Added bounded marker validation for freshness metadata and redaction metadata.
- Added redaction-safe Debug behavior.
- Added serde support that validates on deserialization.
- Exported the model types from `workflow-core`.
- Added focused tests.
- Updated `ROADMAP.md`.

## 3. Scope Explicitly Not Completed

- No runtime approval event wiring.
- No default approval behavior changes.
- No public approval enforcement changes.
- No automatic approvals.
- No hidden approvals.
- No inspect/projection changes.
- No approval-card UI.
- No CLI rendering changes.
- No workflow schema changes.
- No examples.
- No provider writes.
- No side-effect execution.
- No report artifact writes.
- No hosted or distributed runtime behavior.
- No reasoning lineage.
- No release posture changes.

## 4. Model Types Added

The model-only vocabulary is implemented in `workflow-core`:

- `ApprovalDecisionProofMarker`
- `ApprovalDecisionProofMarkerDefinition`
- `ApprovalDecisionProofEnforcementMode`
- `ApprovalDecisionProofValidationPolicy`

The first enforcement mode is `approval_presentation_required`.

The first validation policy is `approval_presentation_request_match`.

## 5. Validation Boundary Summary

The marker constructor validates:

- bounded proof freshness metadata;
- non-zero freshness limits when supplied;
- proof age not exceeding freshness limit when both are supplied;
- redaction metadata entry counts;
- redaction field bounds;
- redaction reason bounds;
- secret-like redaction fields and reasons.

Invalid serialized marker payloads fail closed through the same constructor validation path.

## 6. Redaction And Privacy Summary

The model stores stable proof references and bounded metadata only.

It does not store:

- approval handoff text;
- work summary;
- approved scope;
- strict non-goals;
- validation expectations;
- why-now text;
- chat transcripts;
- screenshots;
- local file paths;
- provider payloads;
- command output;
- source/spec contents;
- credentials, tokens, authorization headers, private keys, or secret-like values.

Debug output redacts presentation IDs and redaction metadata contents. Serialization carries stable IDs and hashes, but not approval-presentation payload text.

## 7. Test Coverage Summary

Added focused tests for:

- valid marker construction;
- enum vocabulary serialization;
- freshness mismatch rejection;
- secret-like redaction metadata rejection;
- serde round trip;
- invalid serialized marker failure;
- Debug redaction behavior.

Focused approval-presentation tests passed.

## 8. Commands Run

- passed: `cargo fmt --all`
- passed: `cargo test -p workflow-core --test approval_presentation`
- passed: `cargo fmt --all --check`
- passed: `cargo clippy --workspace --all-targets -- -D warnings`
- passed: `cargo test --workspace`
- passed: `npm run check:docs`
- passed: `git diff --check`
- passed: `npm run dogfood:benchmark -- phase-close run-1783606281320157000-2 --phase implementation`

Governed phase-close reported:

- dogfood workflow ID: `dg/implement`
- run ID: `run-1783606281320157000-2`
- approval ID: `approval/run-1783606281320157000-2/implementation-approved`
- status: `Completed`
- events total: 39
- approvals: 1
- approval-presentation enforcement: `proof_record_present_granted_approval_seen`
- approval-presentation event marker: `not_available`

## 9. Remaining Known Limitations

- Approval decision events do not yet carry proof markers.
- Default public approval behavior remains unchanged.
- Inspect/projection output does not yet expose proof marker posture.
- Dogfood `phase-close` still reports `approval_presentation_event_marker: not_available` for current runs.
- WorkReport and audit citations to proof markers remain future work.

## 10. Recommended Next Phase

Recommended next phase: approval-event proof marker model review.

The review should verify model scope, validation behavior, redaction posture, serde compatibility, test coverage, and that runtime event wiring remains deferred.
