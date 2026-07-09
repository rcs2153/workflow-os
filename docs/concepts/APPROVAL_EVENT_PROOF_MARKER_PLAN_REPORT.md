# Approval Event Proof Marker Plan Report

## 1. Executive Summary

This phase created the approval-event proof marker plan.

The plan addresses the remaining gap after dogfood phase-close proof disclosure: Workflow OS can persist approval-presentation records and enforce proof through an opt-in approval path, but approval decision events do not yet durably expose which presentation proof was used.

The plan defines a future bounded event/audit semantics change. It does not implement the marker, change public approval behavior, add approval-card UI, add schemas, enable writes, add hosted behavior, implement reasoning lineage, or change release posture.

## 2. Scope Completed

- Created [Approval Event Proof Marker Plan](../implementation-plans/approval-event-proof-marker-plan.md).
- Defined the current proof-record/event-marker boundary.
- Proposed bounded `ApprovalDecisionProofMarker` vocabulary.
- Defined event placement for `ApprovalGranted` and `ApprovalDenied`.
- Defined runtime sequencing and fail-closed behavior.
- Defined inspect/projection and dogfood phase-close implications.
- Defined privacy/redaction rules.
- Defined future test plan and implementation sequence.
- Updated `ROADMAP.md` to link the plan.

## 3. Scope Explicitly Not Completed

- No implementation.
- No public approval behavior changes.
- No automatic approvals.
- No hidden approvals.
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

## 4. Planning Boundary Summary

The plan keeps the next implementation as a model/event semantics slice, not a user-facing approval redesign.

It recommends adding bounded proof-use metadata to approval decision event payloads only when an opt-in proof-enforced approval path validates approval-presentation proof.

Default approval paths should remain unchanged.

## 5. Privacy And Redaction Summary

The planned marker may include stable IDs, hashes, timestamps, freshness metadata, sensitivity, and redaction metadata.

It must not include approval handoff text, work summaries, approved scope, strict non-goals, validation expectations, why-now text, chat transcripts, screenshots, local file paths, provider payloads, command output, source/spec contents, credentials, tokens, private keys, or secret-like values.

## 6. Test Coverage Planned

The plan calls for tests covering:

- proof-enforced grants with event proof markers;
- proof-enforced denials with event proof markers;
- unchanged default approval behavior;
- missing/mismatched proof failing before decision events;
- marker identity and content hash correctness;
- redaction-safe Debug/serialization;
- backward compatibility with old event logs;
- inspect/projection behavior if included;
- dogfood `phase-close` reporting `proof_enforced` when marker exposure exists.

## 7. Governed Phase Summary

- dogfood workflow ID: `dg/d`
- run ID: `run-1783604719373395000-2`
- approval ID: `approval/run-1783604719373395000-2/planning-approved`
- approval-presentation proof: persisted before approval
- presentation ID: `presentation/7ba387cb71dcd9fc`
- approval outcome: granted by delegated maintainer through the proof-enforced dogfood approval command

The dogfood runner coordinated governance only. Planning, documentation edits, validation commands, git operations, PR actions, and this report were performed by the executor outside the kernel and are disclosed here.

## 8. Commands Run

- passed: `npm run check:docs`
- passed: `git diff --check`
- passed: `npm run dogfood:benchmark -- phase-close run-1783604719373395000-2 --phase planning`

Governed phase-close reported:

- status: `Completed`
- events total: 39
- approvals: 1
- approval-presentation enforcement: `proof_record_present_granted_approval_seen`
- approval-presentation event marker: `not_available`

## 9. Remaining Known Limitations

- The approval-event proof marker is planned, not implemented.
- Existing approval decision events do not yet expose proof-use markers.
- Dogfood `phase-close` still reports record-level proof posture for current runs.
- Public approval behavior remains unchanged.

## 10. Recommended Next Phase

Recommended next phase: approval-event proof marker plan review.

The review should verify event placement, default approval compatibility, redaction posture, backwards compatibility, and implementation sequencing before any model or event payload changes are implemented.
