# Approval Gate Presentation Opt-In Enforcement Plan

Status: Implemented as an explicit opt-in executor path.

Related work:

- [Approval Gate Presentation Enforcement Gap](../concepts/APPROVAL_GATE_PRESENTATION_ENFORCEMENT_GAP.md)
- [Approval Gate Presentation Enforcement Plan](approval-gate-presentation-enforcement-plan.md)
- [Approval Gate Presentation Persistence And Enforcement Plan](approval-gate-presentation-persistence-enforcement-plan.md)
- [Approval Gate Presentation Core Model Report](../concepts/APPROVAL_GATE_PRESENTATION_CORE_MODEL_REPORT.md)
- [Approval Gate Presentation Core Model Review](../concepts/APPROVAL_GATE_PRESENTATION_CORE_MODEL_REVIEW.md)
- [Approval Gate Presentation Persistence Report](../concepts/APPROVAL_GATE_PRESENTATION_PERSISTENCE_REPORT.md)
- [Approval Gate Presentation Persistence Review](../concepts/APPROVAL_GATE_PRESENTATION_PERSISTENCE_REVIEW.md)
- [Approval Gate Presentation Opt-In Enforcement Implementation Report](../concepts/APPROVAL_GATE_PRESENTATION_OPT_IN_ENFORCEMENT_IMPLEMENTATION_REPORT.md)
- [Approval Gate Presentation Opt-In Enforcement Review](../concepts/APPROVAL_GATE_PRESENTATION_OPT_IN_ENFORCEMENT_REVIEW.md)
- [Dogfood Runner Approval-Presentation Persistence Plan](dogfood-runner-approval-presentation-persistence-plan.md)

## 1. Executive Summary

Workflow OS can now model approval-presentation proof and persist validated
`ApprovalPresentationRecord` values locally. The remaining P0 gap is that
approval decisions do not yet require a matching presentation record.

The implementation adds an explicit opt-in enforcement path for approval
decisions. That path validates durable presentation proof before an approval
decision is accepted, while keeping existing approval APIs and default runtime
behavior unchanged.

This plan is implemented for explicit local executor callers. Dogfood runner
persistence of presented handoff text is planned in
[Dogfood Runner Approval-Presentation Persistence Plan](dogfood-runner-approval-presentation-persistence-plan.md).
CLI approval cards, high-assurance integration, and default approval
enforcement remain deferred.

## 2. Goals

- Add a narrow opt-in approval decision path that requires presentation proof.
- Preserve existing `LocalExecutor::decide_approval(...)` behavior.
- Validate that proof matches the pending `ApprovalRequest`.
- Validate that proof was presented before the approval decision.
- Support optional freshness/staleness policy.
- Fail closed when proof is missing, stale, mismatched, corrupt, ambiguous, or
  unsafe.
- Avoid partial approval events when proof validation fails.
- Return stable non-leaking errors.
- Keep the path local, deterministic, and testable.
- Prepare future high-assurance approval and write-capable adapter gates without
  implementing them here.

## 3. Non-Goals

This phase must not implement:

- default approval behavior changes;
- automatic approval;
- hidden approval;
- approval UI or hosted approval cards;
- CLI mutation behavior;
- workflow schema fields;
- examples;
- high-assurance approval integration;
- WorkReport citation changes;
- provider writes;
- side effects;
- hosted or distributed runtime behavior;
- reasoning lineage;
- recursive agents, agent swarms, or Level 3/4 autonomy;
- release posture changes.

## 4. Current Baseline

Implemented:

- bounded `ApprovalPresentationRecord` model;
- deterministic approval-presentation content hash;
- request matching helper;
- local `ApprovalPresentationRecordStore`;
- local lookup by presentation ID, run ID, and approval ID;
- maintainer review accepting persistence with non-blocking follow-ups.

Not implemented:

- default approval enforcement;
- dogfood runner persistence of presented handoff text;
- approval presentation citation in WorkReports;
- CLI approval card rendering.

## 5. Recommended First Implementation

Implement a small explicit helper or executor-adjacent method, tentatively:

```rust
decide_approval_with_presentation(...)
```

or a narrowly named helper if that better matches current code boundaries.

The implementation should:

1. Load the pending approval request through existing approval/run state.
2. Load exactly one candidate `ApprovalPresentationRecord` by supplied
   presentation ID, or derive candidates by run ID and approval ID only when
   unambiguous.
3. Validate the presentation record against the pending approval request.
4. Validate `presented_at <= decision.decided_at`.
5. Apply an optional max-age freshness policy when supplied.
6. Only after proof validation succeeds, delegate to the existing approval
   decision path.

The default `decide_approval(...)` method must remain unchanged.

## 6. Input Model

Use an explicit input model if useful, such as:

- `ApprovalDecisionWithPresentationInput`;
- `ApprovalPresentationEnforcementInput`;
- `ApprovalPresentationFreshnessPolicy`.

Required inputs should include:

- run ID;
- approval ID;
- approval decision fields already required by the existing approval path;
- presentation ID, or an explicit request to resolve by run ID and approval ID;
- optional freshness policy;
- enforcement mode, initially fail-closed only.

Do not infer presentation proof from chat history, browser state, terminal
memory, agent memory, or model self-review.

## 7. Proof Matching Rules

The enforcement path must verify:

- presentation record exists;
- presentation record validates after deserialization;
- run ID matches the approval request;
- approval ID matches the approval request;
- workflow ID matches the approval request;
- workflow version matches when present;
- schema version matches when present;
- step ID matches when present;
- content hash matches the stored bounded presentation content;
- presentation channel and sensitivity remain valid;
- redaction metadata remains valid.

Missing or mismatched proof must fail closed before any approval decision event
is appended.

## 8. Ambiguity Rules

If the caller supplies a presentation ID, the path should load and validate that
specific record.

If the caller asks the helper to resolve by run ID and approval ID:

- zero matching records fails closed as missing proof;
- one matching record may proceed to validation;
- more than one matching record fails closed as ambiguous proof unless the
  records are provably identical and a future reviewed phase allows that.

The first implementation should prefer explicit presentation ID input to keep
the enforcement boundary reviewable.

## 9. Freshness And Staleness Policy

Freshness must be explicit.

Recommended first policy:

- no freshness check unless a max age is supplied;
- if supplied, `decision.decided_at - presentation.presented_at` must be within
  the max age;
- `presented_at` after the decision timestamp fails closed;
- stale proof fails with a stable non-leaking error code.

High-assurance approval controls can require stricter freshness later.

## 10. Error Handling

Errors must use stable codes and must not leak:

- approval IDs;
- presentation IDs;
- raw handoff text;
- actor IDs;
- local filesystem paths;
- corrupt payloads;
- command output;
- provider payloads;
- token-like values;
- secret-like metadata.

Recommended code families:

- `approval_presentation_enforcement.proof_missing`;
- `approval_presentation_enforcement.proof_ambiguous`;
- `approval_presentation_enforcement.proof_mismatch`;
- `approval_presentation_enforcement.proof_stale`;
- `approval_presentation_enforcement.proof_corrupt`;
- `approval_presentation_enforcement.decision_time_invalid`;

## 11. Runtime Semantics

The opt-in path must preserve existing workflow semantics.

On proof validation failure:

- no approval decision should be appended;
- no run resume should occur;
- no skill invocation should occur;
- no side effects should occur;
- no report artifact should be written;
- existing run and approval state should remain unchanged.

On proof validation success, the path should reuse the existing approval
decision behavior rather than duplicating approval transition logic.

## 12. Dogfood Runner Relationship

The repo-local dogfood runner currently emits the exact approval handoff but
does not persist it as proof.

After opt-in enforcement planning is accepted, dogfood integration should be a
separate phase:

- persist a record for the emitted approval handoff;
- surface presentation ID and content hash;
- call the opt-in approval path using that presentation ID;
- disclose any missing persistence or enforcement coverage.

Do not combine dogfood runner integration with the first enforcement helper
unless the implementation remains small and reviewable.

## 13. WorkReport Relationship

WorkReports may later cite approval-presentation proof by stable reference.

Deferred:

- citation target changes;
- report section population changes;
- report artifact proof gates;
- disclosure of missing/stale presentation proof.

The enforcement helper should not copy approval handoff text into WorkReports.

## 14. High-Assurance Approval Relationship

Approval-presentation enforcement is a prerequisite for stronger high-assurance
approval controls, but this phase should not implement those controls.

Future high-assurance phases may require:

- requester/approver separation;
- evidence requirements;
- expiry/revocation semantics;
- quorum approval;
- authority scopes;
- admin/steward policy.

This phase should only create the local proof gate that those future controls
can depend on.

## 15. Test Plan

Future implementation tests should cover:

- existing `decide_approval(...)` remains unchanged;
- opt-in approval with matching presentation proof succeeds;
- missing presentation proof fails closed before approval events;
- mismatched approval ID fails without leakage;
- mismatched run/workflow/step identity fails without leakage;
- corrupt stored proof fails without leakage;
- ambiguous proof fails without choosing one silently;
- proof presented after decision time fails closed;
- stale proof fails when a freshness policy is supplied;
- fresh proof succeeds when policy is satisfied;
- denied approvals also require proof when using the opt-in path;
- no partial approval or resume events are appended on proof failure;
- successful opt-in approval reuses existing approval transition behavior;
- Debug and error output do not leak handoff text or identifiers;
- existing approval, high-assurance approval, local executor, and dogfood tests
  still pass.

## 16. Proposed Implementation Sequence

1. Add the explicit input/freshness model and pure proof validation helper.
2. Add an executor-adjacent opt-in approval path that validates proof before
   calling existing approval decision logic.
3. Add focused failure-mode and non-mutation tests.
4. Review the enforcement helper.
5. Plan dogfood runner integration separately.
6. Only after review, consider WorkReport citation and high-assurance
   integration.

## 17. Open Questions

- Should the first path require explicit presentation ID only?
- Should denied approvals require proof in the same way granted approvals do?
- What freshness default should high-assurance approvals eventually require?
- Should ambiguous multiple records ever be accepted if their hashes match?
- Should presentation proof eventually become a workflow event, remain a sidecar
  state record, or both?
- How should CLI approval commands receive presentation IDs without becoming
  noisy for ordinary users?
- When should dogfood runner presentation persistence become mandatory?

## 18. Final Recommendation

Proceed next to a small implementation phase: opt-in approval-presentation
enforcement helper/path.

The implementation must keep existing approval behavior unchanged by default,
must not introduce UI, schemas, examples, provider writes, side effects, hosted
runtime behavior, high-assurance controls, WorkReport citation changes, or
release posture changes, and must fail closed before approval events when proof
is missing or invalid.
