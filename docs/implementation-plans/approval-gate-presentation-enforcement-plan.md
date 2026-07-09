# Approval Gate Presentation Enforcement Plan

Status: Planned. This plan follows the open P0 hardening gap documented in
[Approval Gate Presentation Enforcement Gap](../concepts/APPROVAL_GATE_PRESENTATION_ENFORCEMENT_GAP.md).

## 1. Executive Summary

Workflow OS now emits detailed approval handoff content for governed dogfood
phases, including work summary, approved scope, strict non-goals, expected
touched surfaces, validation expectations, why-now context, run ID, approval ID,
and copy-safe approval request text.

The remaining problem is proof of presentation. The kernel can emit the right
approval content, but it does not yet durably prove that the exact approval
scope was presented before the approval was granted.

This plan defines a bounded P0 hardening path. The first implementation should
add a typed, redaction-safe approval-presentation record and pure validation
helper. It should not change default approval semantics yet. Runtime enforcement
should come only after the model/helper boundary is reviewed.

This plan does not implement anything.

## 2. Goals

- Model approval-presentation proof as a first-class local governance record.
- Bind presentation proof to stable run and approval identity.
- Capture bounded approval scope, non-goals, touched surfaces, validation
  expectations, and next action.
- Record who or what presented the approval context.
- Record when and where the approval context was presented.
- Store a stable hash of the presented approval content.
- Keep presentation content redaction-safe.
- Reject missing, vague, unbounded, or secret-like presentation metadata.
- Prepare future approval decisions to fail closed when presentation proof is
  missing or stale.
- Keep the first implementation deterministic, local, and model/helper-only.
- Preserve existing approval behavior until an explicit enforcement path is
  separately implemented and reviewed.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- automatic approval;
- hidden approval;
- changes to default `LocalExecutor::decide_approval(...)` behavior;
- runtime approval-presentation enforcement in the first model/helper slice;
- approval UI or hosted approval cards;
- high-assurance approval integration;
- write-capable adapters;
- provider mutations;
- CLI mutation behavior;
- workflow schema fields;
- examples;
- report artifact writes;
- persistence beyond a future explicitly scoped local record helper;
- RBAC, IdP, SSO, teams, groups, quorum, revocation, or external identity;
- hosted or distributed runtime behavior;
- reasoning lineage;
- recursive agents, agent swarms, or Level 3/4 autonomy;
- release posture changes.

## 4. Current Baseline

Implemented today:

- approval-gated workflow steps pause before protected work;
- approval request and decision events are durable workflow events;
- approval projections can be rebuilt from event history;
- the repo-local dogfood phase runner emits `approval_handoff`;
- the runner emits `copy_safe_approval_request`;
- the runner requires bounded work context for material phase starts;
- repo instructions require agents to present the complete handoff;
- phase reports can disclose run ID, approval ID, event summary, validation
  summary, and out-of-kernel work.

Not implemented today:

- typed approval-presentation record;
- durable proof of the exact approval content presented;
- stable content hash validation for approval presentations;
- freshness/staleness validation between presentation and approval decision;
- enforcement that material approval decisions require matching presentation
  proof;
- general approval card rendering;
- high-assurance approval presentation integration.

## 5. Candidate Core Model

The first implementation should add the smallest model set needed for local
presentation proof:

- `ApprovalPresentationRecord`;
- `ApprovalPresentationId`;
- `ApprovalPresentationChannel`;
- `ApprovalPresentationContentHash`;
- `ApprovalPresentationValidationInput`;
- `ApprovalPresentationValidationResult`, only if the repository's existing
  validation patterns require a result wrapper;
- `ApprovalPresentationRequirement`, only if needed to express required fields
  without introducing runtime enforcement.

The model should remain domain-neutral and reusable beyond dogfood phases.

## 6. Required Record Fields

An approval-presentation record should capture:

- presentation ID;
- run ID;
- approval ID;
- workflow ID;
- workflow version when available;
- phase or step identity when available;
- requested action;
- planned work or work summary;
- approved scope;
- strict non-goals;
- expected touched surfaces;
- validation/check expectations;
- why-now context;
- next action after approval;
- presented-at timestamp;
- presented-by actor or system actor;
- presentation channel or surface;
- stable hash of the presented content;
- redaction metadata;
- sensitivity.

The record should store bounded summaries and stable references, not raw chats,
screenshots, terminal transcripts, provider payloads, command output, or full
source/spec contents.

## 7. Content Hash Policy

The content hash should bind the record to a canonical presentation payload.

Rules:

- use deterministic field ordering;
- include the approval identity and scope fields;
- include the next action and non-goals;
- include redaction/sensitivity posture where appropriate;
- do not include raw secret-like values;
- do not include volatile display-only formatting unless explicitly canonical;
- reject missing or malformed hashes;
- make hash mismatch a validation failure in future enforcement helpers.

The first implementation may expose a pure helper for computing or validating
the hash from explicit presentation fields.

## 8. Validation Rules

Validation should ensure:

- presentation ID is valid;
- run ID is valid;
- approval ID is valid;
- workflow identity is valid when present;
- requested action is present and bounded;
- planned work/work summary is present and bounded;
- approved scope is present and bounded;
- strict non-goals are present and bounded;
- expected touched surfaces are present and bounded;
- validation/check expectations are present and bounded;
- why-now context is present and bounded;
- next action is present and bounded;
- presented-at timestamp is present;
- presented-by actor or system actor is present and valid;
- presentation channel is valid;
- content hash is present and valid;
- redaction metadata is present and valid;
- sensitivity is present and valid;
- duplicate or conflicting field entries are rejected where collection fields
  are used;
- secret-like values fail closed without leakage.

Validation errors must use stable codes and must not echo raw values.

## 9. Privacy And Redaction

Approval-presentation proof must be safe to inspect.

The model must not store or print:

- raw provider payloads;
- raw command output;
- raw CI logs;
- raw spec contents;
- raw source contents;
- raw chat transcripts;
- screenshots;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- local filesystem paths unless separately bounded and justified.

Debug output must be redaction-safe.

Serialization must not silently carry secret-like values.

Deserialization must fail closed or sanitize according to an explicit policy,
and errors must not leak rejected values.

## 10. First Implementation Boundary

The first implementation should be model/helper-only:

1. Add approval-presentation record types.
2. Add constructors and validation.
3. Add redaction-safe Debug/serde behavior.
4. Add a pure helper to validate a presentation record against an approval
   request identity when the caller supplies both.
5. Add focused tests.
6. Update docs and create an implementation report.

It should not:

- require approval-presentation records in default approval decisions;
- mutate runtime state;
- append workflow events;
- write presentation records to disk;
- add CLI commands;
- add workflow schema fields;
- integrate high-assurance approvals;
- authorize writes.

## 11. Future Enforcement Boundary

After model/helper review, a future opt-in enforcement phase can add:

- local presentation record store helper;
- explicit executor-adjacent approval decision method that requires matching
  presentation proof;
- freshness/staleness checks;
- presentation hash matching;
- report disclosure of presentation-proof posture;
- high-assurance approval integration.

Default approval behavior should remain unchanged until the opt-in enforcement
path is reviewed.

## 12. Relationship To High-Assurance Approvals

Approval-presentation proof is a prerequisite for trustworthy high-assurance
approval controls.

High-assurance controls can enforce requester/approver separation and required
references, but they still need proof that the approver saw the correct
bounded scope. Presentation proof should eventually become one of the required
references or gates for sensitive approvals.

Do not merge the two concepts in the first implementation. Keep presentation
proof model/helper-only, then integrate after review.

## 13. Relationship To Write-Capable Adapters

This belongs before serious write-capable adapter readiness.

Write-capable adapters should not rely on approvals unless Workflow OS can show:

- what action was requested;
- what authority was granted;
- what remained forbidden;
- what evidence/check/report obligations were presented;
- who saw the request;
- when it was presented;
- what exact content was presented.

The first implementation does not authorize provider writes.

## 14. Test Plan

Future tests should cover:

- valid minimal approval-presentation record;
- required identity fields;
- invalid presentation ID rejected;
- invalid run ID rejected;
- invalid approval ID rejected;
- missing requested action rejected;
- missing approved scope rejected;
- missing strict non-goals rejected;
- missing touched surfaces rejected;
- missing validation expectations rejected;
- missing next action rejected;
- missing presented-at rejected;
- missing presented-by rejected;
- invalid channel rejected;
- missing or malformed content hash rejected;
- content hash helper is deterministic;
- secret-like work summary rejected without leakage;
- secret-like scope/non-goals/touched surfaces rejected without leakage;
- Debug output does not leak values;
- serialization does not leak forbidden raw payload markers;
- invalid serialized records fail closed;
- deserialization errors do not leak rejected values;
- helper validates matching run and approval identity;
- helper rejects stale or mismatched identity without leaking IDs;
- existing approval and high-assurance approval tests still pass;
- docs check passes.

## 15. Proposed Implementation Sequence

1. Implement core approval-presentation record model.
2. Implement content hash helper.
3. Implement pure validation helper against supplied approval identity.
4. Add focused tests for validation, hashing, serde, and non-leakage.
5. Update roadmap and gap docs.
6. Create implementation report.
7. Run full Rust and docs validation.
8. Review before any runtime enforcement or persistence work.

## 16. Open Questions

- Should presentation records become workflow events or separate local records?
- Should content hash include exact display formatting or canonical fields only?
- How should presentation proof relate to future approval cards?
- Should presentation proof be required for all material approvals or only
  high-assurance approvals?
- How long may a presentation record remain fresh before approval?
- Should delegated-maintainer approvals require the same presentation proof?
- How should future hosted identity prove the presenter and approver?
- Should WorkReports cite presentation proof directly?

## 17. Final Recommendation

Recommended next implementation phase: approval gate presentation core model and
pure validation helper.

Do not implement runtime enforcement, CLI commands, local persistence,
high-assurance approval integration, provider writes, schemas, examples,
hosted behavior, reasoning lineage, side effects, writes, or release posture
changes in that first implementation.
