# Approval Event Proof Marker Plan

Status: planning only.

## 1. Executive Summary

Workflow OS can now persist approval-presentation records and use them through an opt-in approval decision path. Dogfood phase approvals use that path, and phase-close can disclose matching proof-record posture.

The remaining gap is event-level proof. The workflow event trail records `ApprovalGranted` or `ApprovalDenied`, but it does not yet expose a durable marker showing which approval-presentation proof was used for that decision.

This plan defines a future bounded implementation for approval-event proof markers. It does not implement the marker, change public approval behavior, add approval-card UI, add schemas, enable writes, add hosted behavior, implement reasoning lineage, or change release posture.

## 2. Goals

- Record durable proof-use metadata on approval decision events when an approval path validates approval-presentation proof.
- Preserve existing default approval behavior.
- Preserve existing approval pass/fail semantics.
- Keep proof markers bounded and redaction-safe.
- Let `phase-close`, inspect surfaces, audit projections, and future reports distinguish "proof record exists" from "this approval decision used this proof."
- Avoid copying approval handoff text or presentation payloads into approval events.
- Prepare for later high-assurance approval and WorkReport citation composition.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- default public approval behavior changes;
- automatic approvals;
- hidden approvals;
- approval-card UI;
- CLI rendering changes beyond a separately scoped inspect/projection update;
- workflow schema changes;
- examples;
- provider writes;
- side-effect execution;
- report artifact writes;
- hosted or distributed runtime behavior;
- reasoning lineage;
- recursive agents, agent swarms, or Level 3/4 autonomy;
- release posture changes.

## 4. Current Boundary

Implemented foundations:

- `ApprovalPresentationRecord` model.
- Local approval-presentation persistence.
- Opt-in `LocalExecutor::decide_approval_with_presentation(...)`.
- Dogfood `phase-start` approval-presentation persistence.
- Dogfood hidden proof-enforced approval command.
- Dogfood `phase-close` proof-record disclosure.

Current limitation:

- approval decision events do not expose `presentation_id`, content hash, enforcement mode, or proof validation posture;
- CLI inspect output exposes event kind counts but not approval decision proof-use details;
- dogfood phase-close can report matching proof records, but cannot prove from the event log alone that the approval decision recorded proof use.

## 5. Proposed Marker Concept

Add a bounded marker to approval decision event payloads for opt-in proof-enforced approval paths only.

Candidate model:

```text
ApprovalDecisionProofMarker
- enforcement_mode
- presentation_id
- presentation_content_hash
- proof_validated_at
- proof_validation_policy
- proof_age_ms, optional
- proof_freshness_limit_ms, optional
- proof_record_sensitivity
- redaction metadata
```

Recommended `enforcement_mode` values:

- `none` or absent for existing/default approval paths;
- `approval_presentation_required` for proof-enforced paths;
- future values only after separate planning.

The marker should reference proof by stable identifiers and hashes. It must not copy approval handoff text, work summary, approved scope, non-goals, validation expectations, chat transcripts, screenshots, local paths, provider payloads, command output, or raw source/spec contents.

## 6. Event Placement

The marker belongs on approval decision event payloads:

- `ApprovalGranted`;
- `ApprovalDenied`.

The marker should be appended only after proof validation succeeds and only through opt-in proof-enforced approval paths.

Default `decide_approval(...)` should remain unchanged unless a later phase explicitly changes public approval semantics.

## 7. Runtime Sequencing

Future implementation should preserve the current sequence:

1. Load pending approval request.
2. Resolve approval-presentation proof.
3. Validate proof against pending request.
4. Validate decision timestamp and optional freshness.
5. Construct approval decision event payload with proof marker.
6. Append approval decision event.
7. Resume or fail the run according to the existing approval decision semantics.

Proof validation failures must still occur before approval decision events, `RunResumed`, downstream skill invocation, side-effect events, report artifacts, provider calls, or other runtime mutation.

## 8. Projection And Inspect Behavior

Future inspect/projection work should expose bounded proof marker posture without dumping full presentation records.

Recommended inspect fields:

```text
approval_decisions:
  - approval_id
    decision
    proof_enforcement
    presentation_id
    presentation_content_hash
    proof_validated_at
```

If inspect remains event-kind-only, `phase-close` can still read raw event payloads or approval projections in a separately scoped helper phase, but the preferred path is to make inspect output intentionally expose the bounded proof marker.

## 9. Phase-Close Behavior

Once approval events carry proof markers, dogfood `phase-close` should distinguish:

- `proof_enforced`: approval event explicitly records proof marker;
- `proof_record_present_granted_approval_seen`: proof record exists and approval occurred, but event marker is unavailable;
- `no_proof_record_found`;
- `proof_record_ambiguous`;
- `proof_record_read_error`;
- `no_proof_record_store`.

That preserves backward compatibility with current state while improving newer runs.

## 10. Audit And WorkReport Relationship

Approval-event proof markers should be future-citable by:

- audit projections;
- WorkReport citations;
- high-assurance approval disclosures;
- report artifacts;
- dogfood phase-close summaries.

Those future citations should reference event IDs, approval IDs, presentation IDs, and content hashes. They should not copy presentation payloads.

## 11. Privacy And Redaction

The marker must be redaction-safe by construction.

Allowed:

- stable approval ID if already present in the event payload;
- stable presentation ID;
- presentation content hash;
- bounded enforcement mode;
- bounded timestamp/freshness metadata;
- sensitivity/redaction metadata.

Forbidden:

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
- tokens, credentials, private keys, authorization headers, or secret-like values.

Debug, serialization, and deserialization errors must not leak forbidden values.

## 12. Error Handling

Marker construction should fail closed before appending approval events when:

- proof validation succeeds but marker construction fails;
- marker fields are invalid;
- marker serialization would carry forbidden values;
- marker freshness metadata is inconsistent;
- marker proof identity does not match the validated proof.

Errors should use stable non-leaking codes and should not include raw IDs, paths, payloads, handoff text, snippets, command output, provider output, or secret-like values.

Candidate error codes:

- `approval_event_proof_marker.invalid`;
- `approval_event_proof_marker.mismatch`;
- `approval_event_proof_marker.serialization_failed`;
- `approval_event_proof_marker.redaction_failed`.

## 13. Compatibility

Existing event logs without proof markers must remain readable.

The absence of a proof marker must not retroactively imply an invalid approval because existing default approval behavior and older dogfood runs did not record markers.

Consumers should treat missing markers as:

```text
proof_marker: not_available
```

unless the specific workflow, runtime config, or approval path required proof markers at the time of decision.

## 14. Test Plan

Future implementation tests should cover:

- proof-enforced grant appends approval event with proof marker;
- proof-enforced denial appends approval event with proof marker and still fails closed;
- default approval path appends no proof marker and remains unchanged;
- missing proof still fails before decision event;
- mismatched proof still fails before decision event;
- marker references the validated presentation ID and content hash;
- marker does not copy presentation payload text;
- marker Debug/serialization are redaction-safe;
- old event logs without markers remain readable;
- inspect/projection exposes bounded marker posture if included in scope;
- dogfood phase-close reports `proof_enforced` when inspect/event output exposes marker;
- existing approval-presentation, executor, dogfood helper, and workspace tests still pass.

## 15. Proposed Implementation Sequence

Recommended small phases:

1. Add model-only approval decision proof marker vocabulary.
2. Add marker construction inside opt-in approval-presentation decision path.
3. Add event serialization/deserialization compatibility tests.
4. Expose bounded marker posture through inspect or a local projection helper.
5. Update dogfood `phase-close` to report `proof_enforced` from event marker.
6. Review before any public approval UX, high-assurance approval composition, or WorkReport citation expansion.

## 16. Open Questions

- Should the proof marker live directly on approval decision event payloads or in a linked audit projection?
- Should inspect expose marker details by default or only behind JSON/verbose output?
- Should default public approvals ever require proof markers?
- Should dogfood approvals configure a freshness/max-age policy before marker implementation?
- How should marker records interact with future high-assurance approval controls?
- Should WorkReports cite the approval event, the presentation record, or both?

## 17. Final Recommendation

Proceed next with model-only approval decision proof marker implementation.

Do not build public approval-card UI, default approval enforcement, schemas, examples, writes, hosted behavior, reasoning lineage, report artifact changes, or high-assurance approval expansion in that phase.
