# Approval Event Proof Marker Inspect Projection Report

## 1. Executive Summary

This phase exposes opt-in approval decision proof markers through bounded inspect/projection output.

Approval decision events created through `LocalExecutor::decide_approval_with_presentation(...)` already carry `ApprovalDecisionProofMarker` values. `workflow-os inspect` now projects marker presence and stable marker references for `ApprovalGranted` and `ApprovalDenied` events without dumping approval-presentation payloads. The repo-local dogfood `phase-close` helper now recognizes the projected marker and can report `approval_presentation_enforcement: proof_enforced`.

Default approval behavior remains unchanged.

## 2. Scope Completed

- Added bounded `approval_proof_marker` JSON projection for approval decision events with proof markers.
- Added human inspect text marker posture: `approval_proof_marker=present`.
- Updated the dogfood benchmark helper to detect projected proof markers from inspect JSON.
- Added focused CLI coverage for proof-enforced approval inspect output.
- Added focused dogfood helper coverage for `proof_enforced` phase-close marker detection.
- Updated roadmap and self-governed build documentation.

## 3. Scope Explicitly Not Completed

- No approval-card UI.
- No workflow schema changes.
- No examples.
- No provider writes.
- No side-effect execution.
- No report artifact writes.
- No hosted or distributed runtime behavior.
- No reasoning lineage.
- No release posture changes.
- No default approval behavior changes.
- No automatic approvals.
- No hidden approvals.

## 4. Projection Behavior Summary

For approval decision events carrying a proof marker, `workflow-os --json inspect <run-id>` now includes:

- marker status;
- enforcement mode;
- presentation ID;
- presentation content hash;
- proof validation timestamp;
- validation policy;
- optional proof age;
- optional freshness limit;
- proof record sensitivity.

Marker-free approval decisions remain marker-free in inspect JSON.

Human inspect output keeps the projection compact by appending `approval_proof_marker=present` to the approval decision event line.

## 5. Privacy And Redaction Summary

The projection only exposes bounded proof marker fields that were already validated by the `ApprovalDecisionProofMarker` constructor.

The projection does not copy:

- approval handoff text;
- work summaries;
- approved scope;
- strict non-goals;
- validation expectations;
- chat transcripts;
- command output;
- provider payloads;
- source contents;
- spec contents;
- credentials, tokens, authorization headers, private keys, or secret-like values.

Redaction metadata itself is not projected.

## 6. Dogfood Phase-Close Summary

`scripts/self-governed-benchmark.mjs` now treats inspect events with `approval_proof_marker.status == "present"` as proof marker evidence.

When a matching approval-presentation proof record exists and inspect exposes the marker, phase-close reports:

- `approval_presentation_enforcement: proof_enforced`;
- `approval_presentation_event_marker: present`;
- bounded presentation proof IDs and content hashes.

When old inspect output or marker-free events are encountered, the helper keeps the previous bounded `not_available` posture instead of fabricating marker evidence.

## 7. Test Coverage Summary

Added focused coverage proving:

- proof-enforced approval through the dogfood approval-presentation path projects a marker in JSON inspect output;
- JSON projection includes bounded marker references and stable policy labels;
- JSON projection does not include approval handoff text;
- human inspect output indicates marker presence;
- phase-close proof discovery reports `proof_enforced` when inspect exposes the marker;
- phase-close proof discovery still handles marker-free inspect output conservatively.

## 8. Commands Run

- passed: `cargo fmt --all`
- passed: `node --test scripts/self-governed-benchmark.test.mjs`
- passed: `cargo test -p workflow-cli --test cli dogfood_approval_presentation_approval_projects_proof_marker_in_inspect`
- passed: `cargo fmt --all --check`
- passed: `cargo clippy --workspace --all-targets -- -D warnings`
- passed: `cargo test --workspace`
- passed: `npm run check:docs`
- passed: `git diff --check`
- passed: `npm run dogfood:benchmark -- phase-close run-1783610591590298000-2 --phase implementation`

Governed phase-close reported:

- workflow ID: `dg/implement`;
- run ID: `run-1783610591590298000-2`;
- approval ID: `approval/run-1783610591590298000-2/implementation-approved`;
- approval-presentation ID: `presentation/e294baf716187ae0`;
- approval-presentation content hash: `e294baf716187ae07bcf0c6acf123e5be4bfd1ea9d4e6fa1e0bb0f3dd070cb4c`;
- status: `Completed`;
- events total: 39;
- approvals: 1;
- approval-presentation enforcement: `proof_enforced`;
- approval-presentation event marker: `present`.

## 9. Remaining Known Limitations

- Inspect projection is intentionally bounded and does not render a human approval card.
- WorkReport and audit citations to proof markers remain future work.
- Marker-free approvals still do not imply proof-enforced approval.
- Full schema exposure remains deferred.

## 10. Recommended Next Phase

Recommended next phase: approval-event proof marker inspect/projection review.

Why: the runtime event payload and inspect projection now compose into the dogfood phase-close path. A focused review should verify bounded disclosure, default-path compatibility, tests, and documentation before building additional approval-card, WorkReport citation, or high-assurance approval integrations.
