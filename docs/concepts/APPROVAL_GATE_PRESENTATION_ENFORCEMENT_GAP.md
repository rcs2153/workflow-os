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
The explicit opt-in enforcement path is implemented in
[Approval Gate Presentation Opt-In Enforcement Plan](../implementation-plans/approval-gate-presentation-opt-in-enforcement-plan.md)
and reported in
[Approval Gate Presentation Opt-In Enforcement Implementation Report](APPROVAL_GATE_PRESENTATION_OPT_IN_ENFORCEMENT_IMPLEMENTATION_REPORT.md).
Default/public enforcement planning is documented in
[Approval Gate Presentation Default Enforcement Plan](../implementation-plans/approval-gate-presentation-default-enforcement-plan.md).
The plan review is documented in
[Approval Gate Presentation Default Enforcement Plan Review](APPROVAL_GATE_PRESENTATION_DEFAULT_ENFORCEMENT_PLAN_REVIEW.md).
The policy model/helper implementation is reported in
[Approval Gate Presentation Default Enforcement Implementation Report](APPROVAL_GATE_PRESENTATION_DEFAULT_ENFORCEMENT_IMPLEMENTATION_REPORT.md).
The implementation review is documented in
[Approval Gate Presentation Default Enforcement Implementation Review](APPROVAL_GATE_PRESENTATION_DEFAULT_ENFORCEMENT_IMPLEMENTATION_REVIEW.md).
Selected high-assurance/write-adjacent adoption planning is documented in
[Approval-Presentation Sensitive Adoption Plan](../implementation-plans/approval-presentation-sensitive-adoption-plan.md).
The first selected high-assurance adoption path is implemented in
[High-Assurance Approval-Presentation Adoption Report](HIGH_ASSURANCE_APPROVAL_PRESENTATION_ADOPTION_REPORT.md).
Provider-write/write-adjacent approval-presentation adoption is planned in
[Provider-Write Approval-Presentation Adoption Plan](../implementation-plans/provider-write-approval-presentation-adoption-plan.md).
The first selected GitHub PR comment provider-write proof gate is implemented
in
[Provider-Write Approval-Presentation Gate Implementation Report](PROVIDER_WRITE_APPROVAL_PRESENTATION_GATE_IMPLEMENTATION_REPORT.md),
accepted in
[Provider-Write Approval-Presentation Gate Review](PROVIDER_WRITE_APPROVAL_PRESENTATION_GATE_REVIEW.md),
and edge-hardened in
[Provider Write Approval Presentation Edge Hardening Report](PROVIDER_WRITE_APPROVAL_PRESENTATION_EDGE_HARDENING_REPORT.md)
and
[Provider Write Approval Presentation Edge Hardening Review](PROVIDER_WRITE_APPROVAL_PRESENTATION_EDGE_HARDENING_REVIEW.md).
The repo-local dogfood runner now persists proof during material phase starts,
and dogfood approval enforcement is implemented in
[Dogfood Runner Approval-Presentation Enforcement Plan](../implementation-plans/dogfood-runner-approval-presentation-enforcement-plan.md)
and reported in
[Dogfood Runner Approval-Presentation Enforcement Implementation Report](DOGFOOD_RUNNER_APPROVAL_PRESENTATION_ENFORCEMENT_IMPLEMENTATION_REPORT.md).

## Summary

Workflow OS now emits detailed governed approval handoffs for material dogfood phases. The repo-local phase runner prints bounded work summary, approved scope, strict non-goals, expected touched surfaces, validation expectations, why-now context, run ID, approval ID, and a copy-safe approval request.

The remaining gap is default and dogfood-integrated proof of presentation.

The kernel can emit the correct approval details, and core now provides a typed
approval-presentation record, local persistence, pure validation/hash helpers,
and an explicit opt-in local executor approval path that requires matching
presentation proof. The default approval path does not yet require those
records. The repo-local dogfood runner now persists proof and prints a
proof-enforced approval command for material phase starts, but ordinary public
approval behavior remains unchanged. Selected high-assurance and selected
GitHub PR comment provider-write callers can opt into proof enforcement, but
those paths do not change the default approval behavior or authorize provider
writes by default.

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
- `LocalExecutor::decide_approval_with_presentation(...)` validates matching
  durable proof before appending approval decision events for explicit callers.
- Optional presentation freshness/staleness checks are available for explicit
  callers.
- Selected high-assurance approval decisions can use an explicit opt-in
  approval-presentation policy path.
- The selected GitHub PR comment provider-write proof gate can require matching
  write-adjacent approval-presentation proof before provider invocation.
- Provider-write approval-presentation edge hardening covers wrong posture,
  missing approval references, and stale proof before provider invocation.
- Material dogfood `phase-start` runs persist approval-presentation proof and
  print a proof-enforced dogfood approval command that passes `presentation_id`
  into the existing opt-in enforcement path.

Not implemented:

- default executor enforcement that validates a durable record of the exact
  approval text/card shown to the human before approval;
- validation that every public/default approval was granted only after a
  presentation record exists;
- default executor/runtime enforcement that vague approvals fail closed;
- broad adoption of the default-enforcement policy helper by all
  write-adjacent/provider callers;
- UI/card rendering for ordinary human approval review;
- default provider-write approval-presentation enforcement;
- write-capable adapter defaults.

Implemented after the first gap slices:

- selected high-assurance approval decisions can use an explicit opt-in
  approval-presentation policy path;
- high-assurance control validation and presentation-proof validation both run
  before approval decision events are appended;
- the high-assurance disclosure-returning path can also require presentation
  proof and return report-safe disclosure.
- the selected GitHub PR comment provider-write path can use an explicit
  write-adjacent approval-presentation proof gate before provider invocation;
- provider-write proof-gate edge hardening is implemented for stale proof,
  wrong sensitive-action posture, and missing approval references.

## Implemented Dogfood Capability

The repo-local dogfood runner now persists approval-presentation proof for
material `phase-start` runs and prints a proof-enforced approval command. That
command requires a matching `presentation_id` before the approval decision is
accepted.

The persisted record includes:

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

Default public approval behavior remains unchanged. Broader approval surfaces
now have a conservative default-enforcement plan, but still need review and a
separate implementation before they can require presentation proof.

## Non-Goals

This gap record does not implement:

- public/default approval-presentation enforcement;
- UI approval cards;
- broad/default provider-write/write-adjacent approval-presentation
  enforcement;
- write-capable adapters;
- hosted identity or RBAC;
- CLI mutation behavior;
- schemas;
- examples;
- recursive agents, agent swarms, or Level 3/4 autonomy.

## Recommended Next Phase

Continue selected caller adoption only through separately scoped plans and
reviews. Keep default public approval behavior unchanged, preserve the selected
provider-write proof gate as explicit opt-in only, and keep UI/cards, default
provider writes, schemas, examples, hosted behavior, and release posture changes
out of scope until separately planned and reviewed.
