# Approval Gate Presentation Enforcement Gap

Status: Partially implemented P0 hardening gap. Planning is documented in
[Approval Gate Presentation Enforcement Plan](../implementation-plans/approval-gate-presentation-enforcement-plan.md),
and the first model/helper slice is reported in
[Approval Gate Presentation Core Model Report](APPROVAL_GATE_PRESENTATION_CORE_MODEL_REPORT.md).
The model review is documented in
[Approval Gate Presentation Core Model Review](APPROVAL_GATE_PRESENTATION_CORE_MODEL_REVIEW.md),
and follow-on persistence/enforcement planning is documented in
[Approval Gate Presentation Persistence And Enforcement Plan](../implementation-plans/approval-gate-presentation-persistence-enforcement-plan.md).
The local persistence helper is reported in
[Approval Gate Presentation Persistence Report](APPROVAL_GATE_PRESENTATION_PERSISTENCE_REPORT.md),
and reviewed in
[Approval Gate Presentation Persistence Review](APPROVAL_GATE_PRESENTATION_PERSISTENCE_REVIEW.md).
The explicit opt-in enforcement path is planned in
[Approval Gate Presentation Opt-In Enforcement Plan](../implementation-plans/approval-gate-presentation-opt-in-enforcement-plan.md).

## Summary

Workflow OS now emits detailed governed approval handoffs for material dogfood phases. The repo-local phase runner prints bounded work summary, approved scope, strict non-goals, expected touched surfaces, validation expectations, why-now context, run ID, approval ID, and a copy-safe approval request.

The remaining gap is enforced proof of presentation.

The kernel can emit the correct approval details, and core now provides a typed
approval-presentation record plus pure validation/hash helpers. Runtime approval
decisions do not yet require those records. An agent can still treat tool output
as "visible enough," summarize the approval vaguely, or approve under delegated
authority without producing and attaching a durable approval-presentation record.

That weakens approval gates. The approval may be technically recorded, but the human-review boundary is not yet enforceable.

## Why This Matters

Workflow OS should not rely on agent obedience for approval review quality.

For low-risk local dogfood work, a missing presentation record is a UX and audit defect. For future high-assurance approvals and write-capable adapters, it becomes a safety and governance blocker. If the system cannot prove what scope, non-goals, touched surfaces, validation obligations, and next action were shown before approval, then approval can become theater.

This gap belongs before serious write-capable adapter work and before broader autonomous dogfooding.

## Current Implemented Boundary

Implemented:

- `phase-start` emits `approval_handoff_required: true`.
- `phase-start` emits a structured `approval_handoff` block.
- `phase-start` emits `copy_safe_approval_request_required: true`.
- `phase-start` emits a copy-safe final approval request block.
- Repo instructions require agents to preserve and present the full approval handoff.
- Material phase starts fail closed when bounded work-context fields are missing.
- `ApprovalPresentationRecord` models bounded presentation proof.
- `ApprovalPresentationContentHash` binds proof to canonical presentation
  content.
- `validate_approval_presentation_for_request(...)` validates presentation proof
  against a supplied approval request identity.
- `ApprovalPresentationRecordStore` can locally persist, read, and list
  validated presentation proof records.

Not implemented:

- executor enforcement that validates a durable record of the exact approval
  text/card shown to the human before approval;
- validation that approval was granted only after a presentation record exists;
- executor/runtime enforcement that vague approvals fail closed;
- UI/card rendering for ordinary human approval review;
- integration with high-assurance approval controls.

## Future Capability

Add an approval gate presentation enforcement layer.

The layer should eventually record:

- `run_id`;
- `approval_id`;
- workflow/phase;
- requested action;
- planned work;
- approved scope;
- strict non-goals;
- expected touched surfaces;
- validation/check requirements;
- why-now context;
- next action after approval;
- redaction metadata;
- presentation timestamp;
- presented-by actor/system actor;
- presentation channel or surface;
- stable hash of the presented approval content.

Approval decisions for material phases should be able to require a matching approval-presentation record. Missing or stale presentation records should fail closed.

## Non-Goals

This gap record does not implement:

- runtime approval-presentation records;
- UI approval cards;
- approval decision enforcement changes;
- high-assurance approval integration;
- write-capable adapters;
- hosted identity or RBAC;
- CLI mutation behavior;
- schemas;
- examples;
- recursive agents, agent swarms, or Level 3/4 autonomy.

## Recommended Next Phase

Implement the explicit opt-in approval-presentation enforcement helper/path
next. Keep default approval behavior unchanged until that path is implemented
and reviewed. Keep UI, high-assurance integration, and write-capable adapters
deferred until presentation proof persistence and opt-in enforcement are both
reviewed.
