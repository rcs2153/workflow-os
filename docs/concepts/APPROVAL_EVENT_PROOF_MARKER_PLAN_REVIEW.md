# Approval Event Proof Marker Plan Review

## 1. Executive Verdict

Plan accepted with non-blocking follow-ups.

Proceed to model-only approval decision proof marker implementation.

## 2. Scope Verification

The plan stayed within planning-only scope.

It did not authorize or implement:

- runtime approval behavior changes;
- default approval enforcement changes;
- automatic approvals;
- hidden approvals;
- approval-card UI;
- CLI rendering changes;
- workflow schema changes;
- examples;
- provider writes;
- side-effect execution;
- report artifact writes;
- hosted or distributed runtime behavior;
- reasoning lineage;
- recursive agents, agent swarms, or Level 3/4 autonomy;
- release posture changes.

The plan is appropriately framed as event/audit semantics planning for the existing opt-in approval-presentation path.

## 3. Current Boundary Assessment

The plan correctly identifies the implemented boundary:

- `ApprovalPresentationRecord` exists.
- local approval-presentation persistence exists.
- `LocalExecutor::decide_approval_with_presentation(...)` exists as an opt-in proof-enforced approval path.
- dogfood `phase-start` persists bounded presentation proof.
- dogfood approval uses the opt-in proof-enforced path.
- dogfood `phase-close` can disclose matching proof records.

The plan also correctly identifies the remaining gap. The proof-enforced path validates the presentation record, then appends the same `ApprovalGranted` or `ApprovalDenied` event payload as the default approval path. The durable event trail therefore proves that an approval decision occurred, but does not yet prove from the approval event itself which presentation proof was used.

## 4. Marker Concept Assessment

The proposed `ApprovalDecisionProofMarker` concept is appropriate and bounded.

The recommended fields are useful:

- enforcement mode;
- presentation ID;
- presentation content hash;
- proof validation timestamp;
- proof validation policy;
- optional proof age/freshness metadata;
- proof record sensitivity;
- redaction metadata.

The concept is appropriately reference-based. It avoids copying the approval handoff text, approved scope, strict non-goals, validation expectations, chat transcript, screenshots, local paths, provider payloads, command output, source/spec contents, tokens, credentials, private keys, or secret-like values.

Non-blocking follow-up: during implementation, prefer the smallest marker surface that closes the event-level proof gap. Freshness metadata can remain optional if adding it would broaden the first model slice.

## 5. Event Placement Assessment

The plan's recommended event placement is correct.

Approval proof-use metadata belongs on approval decision event payloads:

- `ApprovalGranted`;
- `ApprovalDenied`.

This is the right level because the approval decision is the audited action that must prove which presented scope was accepted or denied.

The plan appropriately keeps default `decide_approval(...)` unchanged and limits marker attachment to the opt-in proof-enforced approval path unless a future phase explicitly changes public approval semantics.

## 6. Runtime Sequencing Assessment

The proposed sequencing is sound:

1. Load pending approval request.
2. Resolve approval-presentation proof.
3. Validate proof against the pending request.
4. Validate decision timestamp and optional freshness.
5. Construct the approval decision event payload with proof marker.
6. Append the approval decision event.
7. Resume or fail the run through existing approval semantics.

The key invariant is preserved: proof validation and marker construction must fail before approval decision events, `RunResumed`, downstream skill invocation, side-effect events, report artifacts, provider calls, or other runtime mutation.

This preserves deterministic approval behavior while adding the missing durable proof-use record.

## 7. Projection And Inspect Assessment

The plan correctly separates marker implementation from projection/inspect ergonomics.

It recommends a bounded inspect posture:

- approval ID;
- decision;
- proof enforcement;
- presentation ID;
- presentation content hash;
- proof validation timestamp.

That is enough for dogfood `phase-close`, audit projection, future WorkReport citations, and high-assurance approval composition without dumping full presentation records.

Non-blocking follow-up: if inspect exposure is not included in the first implementation, add a small local projection helper immediately after marker implementation. Otherwise `phase-close` will still need fallback record-level disclosure.

## 8. Privacy And Redaction Assessment

The privacy posture is appropriate.

Allowed marker data is stable, bounded, and reference-oriented:

- approval ID if already present;
- presentation ID;
- content hash;
- enforcement mode;
- timestamp/freshness metadata;
- sensitivity and redaction metadata.

Forbidden data is clearly listed and includes the right high-risk surfaces:

- raw approval handoff text;
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
- tokens, credentials, private keys, authorization headers, and secret-like values.

The implementation must keep Debug, serialization, deserialization, and error strings non-leaking.

## 9. Error-Handling Assessment

The plan defines fail-closed behavior at the right boundary.

Marker construction should fail before event append when:

- proof validation succeeds but marker construction fails;
- marker fields are invalid;
- marker serialization would carry forbidden values;
- freshness metadata is inconsistent;
- marker proof identity does not match the validated proof.

The candidate error codes are stable enough for the first implementation:

- `approval_event_proof_marker.invalid`;
- `approval_event_proof_marker.mismatch`;
- `approval_event_proof_marker.serialization_failed`;
- `approval_event_proof_marker.redaction_failed`.

Non-blocking follow-up: implementation should avoid returning raw IDs in error messages even if IDs are otherwise stable, because this path is specifically about approval evidence and should remain conservative.

## 10. Compatibility Assessment

The compatibility posture is correct.

Existing event logs without proof markers must remain readable. Missing markers must not retroactively invalidate old approval decisions or default approval paths.

The plan's `proof_marker: not_available` posture is the right compatibility behavior unless a specific workflow, runtime path, or approval API required proof markers at the time of decision.

## 11. Test Plan Assessment

The planned tests cover the important behavior:

- proof-enforced grant appends marker;
- proof-enforced denial appends marker and still fails closed;
- default approval path remains unchanged;
- missing proof fails before decision event;
- mismatched proof fails before decision event;
- marker references validated presentation ID and content hash;
- marker does not copy presentation payload text;
- marker Debug/serialization is redaction-safe;
- old logs without markers remain readable;
- inspect/projection exposes bounded marker posture if included;
- dogfood `phase-close` can report `proof_enforced` when marker exposure exists;
- existing approval-presentation, executor, dogfood helper, and workspace tests continue to pass.

No blocking test gaps were found.

Non-blocking follow-up: include a regression proving the proof-enforced path does not append `ApprovalGranted`, `ApprovalDenied`, `RunResumed`, or downstream events when marker construction fails.

## 12. Documentation Review

The plan and report accurately state:

- approval-event proof markers are planned, not implemented;
- existing approval-presentation proof records and dogfood phase-close disclosure exist;
- approval decision events do not yet expose proof-use markers;
- default public approval behavior remains unchanged;
- approval-card UI is not implemented;
- schemas are not changed;
- examples are not updated;
- provider writes, hosted behavior, reasoning lineage, side-effect execution, and release posture changes remain out of scope.

## 13. Blockers

None.

## 14. Non-Blocking Follow-Ups

- Keep the first model implementation minimal; defer optional freshness detail if needed.
- Add a follow-on inspect/projection helper if marker exposure is not part of the first implementation.
- Avoid raw IDs in marker-construction error messages.
- Add a regression proving marker construction failure prevents approval decision events and downstream runtime mutation.

## 15. Recommended Next Phase

Recommended next phase: model-only approval decision proof marker implementation.

Why: the plan is bounded, compatible, and directly closes the event-level proof gap identified by dogfood phase-close disclosure. The next slice should add the marker vocabulary and validation without public approval UI, default approval enforcement, schemas, examples, writes, hosted behavior, reasoning lineage, report artifact changes, or high-assurance approval expansion.

## 16. Validation

- passed: `npm run check:docs`
- passed: `git diff --check`
- passed: `npm run dogfood:benchmark -- phase-close run-1783605609908902000-2 --phase review`

Governed phase-close reported:

- dogfood workflow ID: `dg/review`
- run ID: `run-1783605609908902000-2`
- approval ID: `approval/run-1783605609908902000-2/review-scope-approved`
- status: `Completed`
- events total: 39
- approvals: 1
- approval-presentation enforcement: `proof_record_present_granted_approval_seen`
- approval-presentation event marker: `not_available`
