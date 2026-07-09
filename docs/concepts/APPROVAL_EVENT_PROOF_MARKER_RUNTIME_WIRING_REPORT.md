# Approval Event Proof Marker Runtime Wiring Report

## 1. Executive Summary

This phase wired approval decision proof markers into the existing opt-in approval-presentation decision path.

When callers use `LocalExecutor::decide_approval_with_presentation(...)`, Workflow OS now records an `ApprovalDecisionProofMarker` on the resulting `ApprovalGranted` or `ApprovalDenied` event payload after approval-presentation proof has been loaded and validated.

Default `LocalExecutor::decide_approval(...)` behavior remains unchanged and does not attach proof markers.

## 2. Scope Completed

- Added an optional `proof_marker` field to `ApprovalDecision`.
- Kept the field backward-compatible with serde defaulting and skip-when-absent serialization.
- Constructed `ApprovalDecisionProofMarker` only in `decide_approval_with_presentation(...)`.
- Attached proof markers to both granted and denied approval decision events in the proof-enforced path.
- Preserved default approval decisions without proof markers.
- Preserved older marker-free approval decision payload compatibility.
- Added focused tests for proof marker attachment, persistence/rehydration, default-path non-attachment, and marker-free serde compatibility.
- Updated `ROADMAP.md`.

## 3. Scope Explicitly Not Completed

- No default approval behavior changes.
- No automatic approvals.
- No hidden approvals.
- No approval-card UI.
- No inspect/projection changes.
- No workflow schema changes.
- No examples.
- No provider writes.
- No side-effect execution.
- No report artifact writes.
- No hosted or distributed runtime behavior.
- No reasoning lineage.
- No release posture changes.

## 4. Runtime Behavior Summary

The opt-in approval-presentation path now follows this sequence:

1. Rehydrate the waiting run.
2. Resolve the pending approval request.
3. Build the approval decision.
4. Resolve the durable approval-presentation proof.
5. Validate the proof against the pending approval request.
6. Construct an `ApprovalDecisionProofMarker`.
7. Append the approval decision event with the marker.
8. Resume or fail closed using existing approval semantics.

If proof resolution, proof validation, or marker construction fails, no approval decision event is appended and downstream runtime mutation does not occur.

## 5. Compatibility Summary

`ApprovalDecision.proof_marker` is optional.

Default approvals serialize without `proof_marker`, and existing marker-free approval decision payloads deserialize with `proof_marker: None`.

Existing event logs without proof markers remain valid and do not retroactively imply invalid approval decisions.

## 6. Privacy And Redaction Summary

Proof markers store stable references and bounded metadata only:

- presentation ID;
- presentation content hash;
- proof validation timestamp;
- validation policy;
- proof record sensitivity;
- redaction metadata.

They do not copy:

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
- source or spec contents;
- credentials, tokens, authorization headers, private keys, or secret-like values.

Proof age and freshness-limit metadata are recorded only when the caller supplied `max_presentation_age`. When no freshness limit is requested, the marker remains reference-only and does not make old-but-valid presentation records fail through marker construction.

## 7. Test Coverage Summary

Added or updated focused tests proving:

- proof-enforced grants attach a marker;
- proof-enforced denials attach a marker;
- proof markers survive local state rehydration;
- default approval decisions do not attach proof markers;
- marker-free approval decision event payloads remain serde-compatible.

Existing approval-presentation model tests continue to pass.

## 8. Commands Run

- passed: `cargo fmt --all`
- passed: `cargo test -p workflow-core --test approval_presentation`
- passed: `cargo test -p workflow-core --test local_executor approval_with_presentation`
- passed: `cargo test -p workflow-core --test runtime_events approval_decision_without_proof_marker_remains_compatible`
- passed: `cargo fmt --all --check`
- passed: `cargo clippy --workspace --all-targets -- -D warnings`
- passed: `cargo test --workspace`
- passed: `npm run check:docs`
- passed: `git diff --check`
- passed: `npm run dogfood:benchmark -- phase-close run-1783608280477579000-2 --phase implementation`

Governed phase-close reported:

- dogfood workflow ID: `dg/implement`;
- run ID: `run-1783608280477579000-2`;
- approval ID: `approval/run-1783608280477579000-2/implementation-approved`;
- status: `Completed`;
- events total: 39;
- approvals: 1;
- approval-presentation enforcement: `proof_record_present_granted_approval_seen`;
- approval-presentation ID: `presentation/2c164f6ed0dfe1f9`;
- approval-presentation content hash: `2c164f6ed0dfe1f9b3eff3449892c5a1a58decf13fba815011bab7934f49caa9`;
- approval-presentation event marker: `not_available` through current inspect output.

## 9. Remaining Known Limitations

- Inspect/projection output does not yet intentionally summarize proof markers.
- Dogfood `phase-close` may still report `approval_presentation_event_marker: not_available` until inspect/projection learns to expose marker posture.
- WorkReport and audit citations to approval proof markers remain future work.
- Marker-specific redaction metadata secret-like failures still reuse the broader approval-presentation secret-like validation code.

## 10. Recommended Next Phase

Recommended next phase: approval-event proof marker inspect/projection planning or implementation.

Why: approval decision events now carry proof markers on the opt-in path, but current inspect/projection surfaces do not yet expose that posture in a bounded operator-friendly way. The next narrow slice should make proof marker presence visible without dumping approval-presentation payloads or changing default approval semantics.
