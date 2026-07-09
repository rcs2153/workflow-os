# Approval Proof Marker Audit Projection Helper Report

## 1. Executive Summary

The first approval proof-marker audit projection helper is implemented as a pure in-memory helper in `workflow-core`.

The helper derives bounded proof-marker audit posture from explicit `WorkflowRun` input. It records source event identity, approval reference identity, granted/denied decision vocabulary, proof-marker posture, bounded marker-presence booleans, redaction metadata, and sensitivity. It does not persist audit records, add a dedicated audit sink, change executor defaults, write report artifacts, add schemas, expose CLI output, perform writes, or change release posture.

## 2. Scope Completed

Completed:

- Added `derive_approval_proof_marker_audit_projection(...)`.
- Added explicit input type `ApprovalProofMarkerAuditProjectionInput`.
- Added bounded result type `ApprovalProofMarkerAuditProjectionResult`.
- Added bounded record type `ApprovalProofMarkerAuditProjectionRecord`.
- Added decision vocabulary `ApprovalProofMarkerAuditDecision`.
- Added status vocabulary `ApprovalProofMarkerAuditStatus`.
- Exported the helper and types from `workflow-core`.
- Added focused tests for granted, denied, marker-free, missing-required, identity, and non-leakage behavior.
- Updated roadmap and planning docs.

## 3. Scope Explicitly Not Completed

Not implemented:

- durable audit projection persistence;
- dedicated proof-marker audit sink records;
- executor default proof-marker citation behavior;
- automatic approval proof-marker enforcement;
- automatic report generation for every run;
- report artifact proof-marker gates;
- report artifact writes;
- workflow schema changes;
- CLI rendering;
- examples;
- provider writes;
- side-effect execution;
- hosted or distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 4. Helper API Summary

The helper accepts:

- a borrowed `WorkflowRun`;
- `require_proof_markers`;
- sensitivity;
- redaction metadata.

It returns an in-memory `ApprovalProofMarkerAuditProjectionResult` containing bounded projection records.

Each record exposes:

- source workflow event ID;
- approval reference ID;
- granted/denied decision vocabulary;
- proof-marker status: `present` or `not_required`;
- whether a presentation ID was present;
- whether a presentation content hash was present;
- sensitivity;
- redaction metadata.

## 5. Validation Boundary Summary

The helper:

- validates supplied redaction metadata through existing WorkReport redaction validation;
- rejects invalid approval references with a stable non-leaking code;
- fails closed when proof markers are required but an approval decision is marker-free;
- preserves marker-free compatibility when proof markers are not required;
- preserves run state and event history.

Stable error namespace:

- `approval_proof_marker_audit_projection.reference_invalid`;
- `approval_proof_marker_audit_projection.marker_missing`.

## 6. Redaction And Privacy Summary

The helper does not copy:

- approval-presentation payloads;
- approval handoff text;
- work summaries, approved scopes, strict non-goals, validation expectations, or why-now text;
- command output;
- provider payloads;
- CI logs;
- GitHub or Jira bodies;
- source or spec contents;
- environment variable values;
- credentials, tokens, authorization headers, private keys, or secret-like values.

Custom `Debug` implementations redact source event IDs and approval reference IDs from debug output while preserving safe counts and posture vocabulary.

Serialization includes stable source event and approval references because the helper models audit projection posture, but it does not serialize presentation IDs, presentation content hashes, handoff text, approval reasons, or approval-presentation payloads.

## 7. Workflow Semantics Summary

The helper is pure and in-memory.

It does not:

- mutate `WorkflowRun`;
- append workflow events;
- emit audit events;
- write audit records;
- write report artifacts;
- call providers;
- touch a `StateBackend`;
- emit CLI output;
- change workflow pass/fail semantics.

## 8. Test Coverage Summary

Focused tests cover:

- proof-enforced granted approval projection;
- proof-enforced denied approval projection;
- marker-free approval compatibility when proof markers are not required;
- missing required proof marker failure with non-leaking error;
- source workflow event and approval reference identity preservation;
- proof-marker posture vocabulary;
- presentation ID/hash presence booleans;
- Debug non-leakage;
- serialization non-leakage for presentation payloads and raw forbidden markers;
- run state and event-history preservation on failure.

## 9. Commands Run And Results

- `cargo test -p workflow-core --test work_report approval_proof_marker_audit_projection`: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 10. Dogfood Governance

- workflow_id: `dg/implement`
- run_id: `run-1783624139252300000-2`
- approval_id: `approval/run-1783624139252300000-2/implementation-approved`
- presentation_id: `presentation/a773bfb5b06ef203`
- approval_outcome: granted

## 11. Remaining Known Limitations

- The helper does not persist audit projection records.
- The helper does not emit dedicated audit sink records.
- The helper does not integrate with executor defaults.
- The helper does not gate report artifacts.
- Workflow-declared proof-marker requirements remain unimplemented.
- CLI rendering remains unimplemented.

## 12. Recommended Next Phase

Recommended next phase: approval proof-marker audit projection helper review.

Reason: the helper is now implemented, and the next decision should verify the bounded API, compatibility, non-leakage posture, and whether a later durable persistence phase is justified.
