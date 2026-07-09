# Approval Event Proof Marker Runtime Wiring Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

Proceed to approval-event proof marker inspect/projection implementation.

The runtime wiring is narrow, opt-in, and compatible with existing approval behavior. `LocalExecutor::decide_approval_with_presentation(...)` now records bounded proof markers on approval decision events after durable presentation proof validation, while default `LocalExecutor::decide_approval(...)` continues to emit marker-free approval decisions.

## 2. Scope Verification

The phase stayed within the approved runtime wiring scope.

Implemented scope verified:

- optional `ApprovalDecision.proof_marker`;
- marker attachment for opt-in approval-presentation grants;
- marker attachment for opt-in approval-presentation denials;
- default approval path remains marker-free;
- marker-free event payload compatibility is preserved;
- focused tests and documentation report exist.

No accidental scope expansion found:

- no default approval behavior change;
- no automatic approvals;
- no hidden approvals;
- no approval-card UI;
- no inspect/projection behavior;
- no workflow schema changes;
- no examples;
- no provider writes;
- no side-effect execution;
- no report artifact writes;
- no hosted or distributed runtime behavior;
- no reasoning lineage;
- no release posture changes.

## 3. Runtime Wiring Assessment

The implementation correctly keeps proof marker wiring behind `LocalExecutor::decide_approval_with_presentation(...)`.

The opt-in path:

1. prepares the approval decision through the same existing decision preparation path;
2. resolves the durable approval-presentation proof;
3. validates the proof against the pending approval request and decision;
4. constructs an `ApprovalDecisionProofMarker`;
5. appends the approval decision event with the marker.

This ordering is appropriate. Proof resolution, proof validation, and marker construction happen before the approval decision event is appended and before downstream resume/fail-closed behavior proceeds.

The default `LocalExecutor::decide_approval(...)` path still constructs `ApprovalDecision { proof_marker: None, ... }` and applies the existing approval decision behavior unchanged.

## 4. Event Payload And Compatibility Assessment

`ApprovalDecision.proof_marker` is optional and serde-compatible:

- `#[serde(default)]` allows old marker-free approval decision payloads to deserialize;
- `skip_serializing_if = "Option::is_none"` keeps marker-free approvals from serializing a new null field;
- focused compatibility coverage proves marker-free approval decision events serialize without `proof_marker` and deserialize as `None`.

This is the right compatibility posture for an event-log payload change. Existing event logs remain valid and do not retroactively imply proof-enforced approval.

## 5. Privacy And Redaction Assessment

The proof marker stores bounded references and metadata only:

- presentation ID;
- presentation content hash;
- proof validation timestamp;
- validation policy;
- presentation proof sensitivity;
- redaction metadata.

The wiring does not copy approval handoff text, approved scope, non-goals, validation expectations, chat transcripts, command output, provider payloads, source contents, spec contents, credentials, tokens, private keys, or secret-like values into approval decision events.

The marker uses the existing `ApprovalDecisionProofMarker` constructor, so model validation remains the redaction and shape boundary.

## 6. Validation And Error Handling Assessment

The opt-in path fails closed before appending an approval decision event when proof resolution, proof validation, or marker construction fails.

Validation errors remain structured and stable. The phase does not convert proof construction failures into misleading user project diagnostics, and it does not emit partial proof markers.

The decision to record `proof_age_ms` and `proof_freshness_limit_ms` only when `max_presentation_age` is supplied is conservative and compatible with existing presentation records. It avoids making old-but-valid records fail marker construction when the caller did not request freshness enforcement.

## 7. Test Quality Assessment

Focused tests cover the important behavior:

- proof-enforced grants attach proof markers;
- proof-enforced denials attach proof markers;
- proof markers survive local state rehydration;
- default approval decisions do not attach proof markers;
- marker-free approval decision payloads remain serde-compatible.

Existing approval-presentation, runtime event, and local executor tests were run during implementation.

Non-blocking test follow-up: add a narrow regression proving marker construction failure prevents event append if a public test path can produce invalid marker metadata without hand-editing state.

## 8. Documentation Review

The phase report is accurate and appropriately bounded.

Docs correctly say:

- proof markers are wired into the opt-in approval-presentation decision path;
- default approval behavior remains unchanged;
- inspect/projection output does not yet intentionally summarize proof markers;
- WorkReport and audit citations to approval proof markers remain future work;
- UI/cards, schemas, examples, writes, hosted behavior, reasoning lineage, and release posture changes are not implemented.

## 9. Blockers

None.

## 10. Non-Blocking Follow-Ups

- Expose approval decision proof marker posture in inspect/projection output without dumping approval-presentation payloads.
- Teach dogfood `phase-close` to report proof marker presence once inspect/projection exposes it.
- Add WorkReport/audit citation behavior for approval decision proof markers after projection is reviewed.
- Consider marker-specific redaction metadata regression coverage if a safe public invalid-input path is available.
- Consider richer freshness metadata tests when callers provide `max_presentation_age`.

## 11. Recommended Next Phase

Recommended next phase: approval-event proof marker inspect/projection implementation.

Why: approval decision events now carry proof markers on the opt-in path, but operator-facing inspect/projection surfaces still cannot summarize that proof posture. The next narrow runtime-composition slice should expose marker presence and stable marker references without changing default approval semantics, adding UI/cards, introducing schemas, or copying approval-presentation payloads.

## 12. Validation

Review governed by:

- workflow ID: `dg/review`;
- run ID: `run-1783609911834109000-2`;
- approval ID: `approval/run-1783609911834109000-2/review-scope-approved`;
- approval-presentation ID: `presentation/d8b1cae1759b7532`;
- approval-presentation content hash: `d8b1cae1759b7532b704c3ff02ede96b88e3e0ed64d56bb8255c8115e784698e`;
- approval outcome: granted.

Commands:

- passed: `npm run check:docs`;
- passed: `git diff --check`;
- passed: `npm run dogfood:benchmark -- phase-close run-1783609911834109000-2 --phase review`.

Governed phase-close reported:

- workflow ID: `dg/review`;
- status: `Completed`;
- events total: 39;
- approvals: 1;
- retries: 0;
- escalations: 0;
- approval-presentation enforcement: `proof_record_present_granted_approval_seen`;
- approval-presentation event marker: `not_available`;
- note: proof record matched the run; inspect output does not yet expose a presentation proof-use marker.
