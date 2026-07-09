# Approval Event Proof Marker Inspect Projection Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The inspect/projection slice is narrow, bounded, and aligned with the approved phase boundary. Approval decision events that already carry proof markers through the opt-in approval-presentation path are now visible through `workflow-os inspect` without exposing approval handoff text, presentation payloads, raw command output, or secret-like values. Default approval behavior remains unchanged.

Proceed next to approval proof marker WorkReport/audit citation planning.

## 2. Scope Verification

The phase stayed within approved inspect/projection scope.

Confirmed not introduced:

- approval-card UI;
- workflow schema changes;
- examples;
- provider writes;
- side-effect execution;
- report artifact writes;
- hosted or distributed runtime behavior;
- reasoning lineage;
- release posture changes;
- default approval behavior changes;
- automatic approvals;
- hidden approvals.

The implementation only changes bounded CLI inspect projection and repo-local dogfood phase-close proof detection.

## 3. Projection Behavior Assessment

The selected projection surface is appropriate.

`workflow-os --json inspect <run-id>` now emits `approval_proof_marker` only for `ApprovalGranted` or `ApprovalDenied` events whose decision payload already contains a proof marker. Human inspect output appends the compact `approval_proof_marker=present` marker to those event lines.

This is the right operator-facing boundary because it makes proof-enforced approvals inspectable without turning approval-presentation records into a broad public payload surface.

## 4. Default Approval Compatibility Assessment

Default approval behavior is preserved.

Marker-free approval decisions remain marker-free in inspect output. Existing event logs without proof markers remain readable and do not become invalid. The implementation does not require proof markers for the default `decide_approval` path and does not retroactively reinterpret old approvals.

## 5. Privacy And Redaction Assessment

The projection is bounded and redaction-safe.

Projected fields are limited to:

- marker status;
- enforcement mode;
- presentation ID;
- presentation content hash;
- proof validation timestamp;
- validation policy;
- optional proof age;
- optional freshness limit;
- proof record sensitivity.

Confirmed not projected:

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
- credentials, tokens, authorization headers, private keys, or secret-like values;
- redaction metadata.

## 6. Dogfood Phase-Close Assessment

The dogfood helper now correctly treats inspect events with `approval_proof_marker.status == "present"` as proof marker evidence.

When a matching approval-presentation proof record exists and inspect exposes the marker, phase-close can report `approval_presentation_enforcement: proof_enforced` and `approval_presentation_event_marker: present`. Marker-free inspect output remains conservative and does not fabricate enforcement.

This closes the previously documented gap where phase-close could see proof records but could not prove the approval event itself referenced the proof.

## 7. Test Quality Assessment

Focused tests cover the important behavior:

- proof-enforced dogfood approval projects a marker in JSON inspect output;
- JSON inspect includes bounded marker references and stable labels;
- JSON inspect does not include approval handoff text;
- human inspect output includes compact marker presence;
- phase-close proof discovery reports `proof_enforced` when inspect exposes the marker;
- marker-free inspect output remains conservative.

Existing full-suite validation from the implementation phase passed. This review changed documentation only, so Rust tests were not rerun during review.

## 8. Documentation Review

Documentation is honest about implemented and deferred scope.

Updated or verified documentation says bounded proof-marker inspect/projection is implemented and that default approval behavior, automatic approvals, approval-card UI, schemas, examples, writes, hosted behavior, and release posture changes remain unimplemented.

## 9. Blockers

None.

## 10. Non-Blocking Follow-Ups

- Plan WorkReport and audit citation behavior for approval decision proof markers.
- Consider a future human approval-card display that reads the bounded marker and proof record without exposing presentation payloads.
- Keep old marker-free approval events explicitly compatible in any future schema or CLI stability work.

## 11. Recommended Next Phase

Recommended next phase: approval proof marker WorkReport/audit citation planning.

Why: proof markers are now modeled, wired into the opt-in approval-presentation decision path, and visible through bounded inspect/projection. The next useful integration is citation planning so WorkReports and audit summaries can reference proof-enforced approvals without copying approval-presentation payloads.

## 12. Validation

- passed: `npm run check:docs`
- not run: Rust tests during review, because this review changed documentation only and the implementation phase already ran `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace`.

Governed review phase:

- workflow ID: `dg/review`
- run ID: `run-1783611923251950000-2`
- approval ID: `approval/run-1783611923251950000-2/review-scope-approved`
- approval-presentation ID: `presentation/4c1ea7b36e38d4c7`
- approval-presentation content hash: `4c1ea7b36e38d4c7692f6c249acd97e30d46d432c21ed88a93fd960037fc2737`
- approval outcome: granted
