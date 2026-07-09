# Approval Gate Presentation Persistence And Enforcement Plan

Status: First persistence helper slice implemented and reviewed. The explicit
opt-in executor enforcement path is implemented. Approval-presentation core
model and validation helpers are implemented and reviewed in
[Approval Gate Presentation Core Model Review](../concepts/APPROVAL_GATE_PRESENTATION_CORE_MODEL_REVIEW.md).
The local persistence helper is reported in
[Approval Gate Presentation Persistence Report](../concepts/APPROVAL_GATE_PRESENTATION_PERSISTENCE_REPORT.md)
and accepted in
[Approval Gate Presentation Persistence Review](../concepts/APPROVAL_GATE_PRESENTATION_PERSISTENCE_REVIEW.md).
The explicit opt-in enforcement path is implemented in
[Approval Gate Presentation Opt-In Enforcement Plan](approval-gate-presentation-opt-in-enforcement-plan.md).

## 1. Executive Summary

Workflow OS can now model bounded approval-presentation proof and validate that
the proof matches an approval request identity. The next question is how that
proof becomes durable and enforceable without changing default approval
semantics prematurely.

This plan defines a conservative two-step path:

1. Add a local approval-presentation record persistence helper.
2. Add an explicit opt-in approval decision path that requires matching
   presentation proof.

The first implementation should start with local persistence only. Runtime
enforcement should follow only after that persistence boundary is implemented
and reviewed.

This plan does not implement anything.

## 2. Goals

- Persist validated `ApprovalPresentationRecord` values locally.
- Preserve deterministic approval behavior for existing callers.
- Provide stable local lookup by presentation ID, run ID, and approval ID.
- Prevent duplicate or conflicting presentation records.
- Keep persisted records redaction-safe and bounded.
- Prepare an explicit opt-in approval enforcement path.
- Allow future material approvals to fail closed when matching proof is missing,
  stale, mismatched, or unsafe.
- Preserve WorkReport and audit compatibility by using stable references rather
  than copying approval handoff payloads.
- Keep default `LocalExecutor::decide_approval(...)` behavior unchanged until an
  enforcement path is explicitly added and reviewed.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- default approval behavior changes;
- automatic approval;
- hidden approval;
- runtime enforcement in the persistence-only implementation;
- approval UI or hosted approval cards;
- workflow schema fields;
- CLI mutation behavior;
- examples;
- high-assurance approval integration;
- provider writes;
- side effects;
- hosted or distributed runtime behavior;
- reasoning lineage;
- recursive agents, agent swarms, or Level 3/4 autonomy;
- release posture changes.

## 4. Current Baseline

Implemented:

- `ApprovalPresentationRecord`;
- `ApprovalPresentationId`;
- `ApprovalPresentationContentHash`;
- `ApprovalPresentationChannel`;
- `ApprovalPresentationSensitivity`;
- `compute_approval_presentation_content_hash(...)`;
- `validate_approval_presentation_for_request(...)`;
- redaction-safe Debug and serde validation;
- model/helper tests;
- maintainer review accepting the model/helper boundary.

Implemented after this plan:

- local presentation record store;
- durable lookup by presentation ID;
- listing by run ID;
- listing by run ID and approval ID.

Implemented after this plan:

- explicit executor-adjacent opt-in enforcement;
- proof lookup by presentation ID or unambiguous run/approval lookup;
- presentation freshness or staleness policy for callers that supply a max age;

Not implemented:

- default approval enforcement;
- WorkReport citation of presentation proof;
- CLI approval card rendering.

## 5. Recommended Split

The next work should be split into two implementation phases.

Phase A: local record persistence helper.

- Add a store-backed helper for writing, reading, and listing
  `ApprovalPresentationRecord` values.
- Reuse the existing local backend pattern where appropriate.
- Reject duplicate writes.
- Validate record identity before persistence.
- Do not change approval decisions.

Phase B: opt-in enforcement path.

- Add an explicit executor-adjacent approval decision method or helper that
  requires a matching presentation record.
- Validate presentation record identity against the pending `ApprovalRequest`.
- Enforce freshness/staleness if configured.
- Return structured non-leaking errors when proof is missing or mismatched.
- Preserve existing approval APIs unchanged.

The phases should not be collapsed unless the persistence implementation is
small enough to review safely and the enforcement behavior remains explicitly
opt-in.

## 6. Persistence Model

The persistence helper should store validated approval-presentation records by
stable identity.

Recommended lookup surfaces:

- by `ApprovalPresentationId`;
- by `WorkflowRunId`;
- by approval ID;
- list records for a run;
- list records for a run and approval ID.

Recommended local layout should follow existing store patterns and safe file
name encoding. The file path must not include unbounded or raw approval text.

The helper should return owned validated records and must not expose mutable
collections that bypass validation.

## 7. Persistence Validation Rules

Persistence should fail closed when:

- the record fails constructor validation;
- record identity and file identity disagree;
- a duplicate record would overwrite an existing record;
- a read record fails deserialization validation;
- a record references secret-like redaction metadata;
- store corruption is encountered;
- lookup input is invalid, unbounded, or secret-like.

Errors must use stable codes and must not leak approval IDs, raw handoff text,
local filesystem paths, source snippets, command output, provider payloads, or
secret-like values.

## 8. Enforcement Boundary

Opt-in enforcement should validate a presentation record before granting or
denying approval through a new explicit path.

The enforcement path should:

- load or accept a candidate `ApprovalPresentationRecord`;
- validate the record against the pending `ApprovalRequest`;
- ensure the record was presented before the approval decision timestamp;
- reject stale presentation proof if a freshness policy is supplied;
- reject content hash mismatch through existing constructor validation;
- reject missing proof in fail-closed mode;
- reject ambiguous multiple candidate records unless the caller supplies a
  precise presentation ID;
- return the original run plus a structured approval error only where existing
  executor architecture supports that safely.

The enforcement path must not:

- silently grant approval;
- change default `decide_approval(...)`;
- mutate workflow state before proof validation passes;
- append partial approval events on validation failure;
- treat model self-review as presentation proof.

## 9. Freshness And Staleness Policy

Freshness should be explicit and conservative.

Recommended first policy:

- no default freshness enforcement in persistence-only phase;
- opt-in enforcement may accept an optional max age duration;
- approval decision time must be greater than or equal to `presented_at`;
- stale proof fails closed with a stable non-leaking code;
- future high-assurance approvals can require stricter freshness.

Do not infer freshness from chat recency, browser state, or agent memory.

## 10. Relationship To Existing Approval APIs

Existing APIs should remain unchanged:

- `LocalExecutor::decide_approval(...)`;
- existing approval request and decision event shapes;
- existing CLI approval behavior unless separately scoped.

New enforcement should be explicit, such as:

- a helper that validates presentation proof before a caller invokes approval;
- an executor-adjacent `decide_approval_with_presentation(...)` path;
- or a local dogfood runner path that requires presentation record identity.

The implementation should choose the smallest shape that matches current
runtime patterns.

## 11. Relationship To Dogfood Runner

The repo-local dogfood runner currently prints the full approval handoff and
copy-safe approval request. Future dogfood integration should use the
persistence helper to record the presented approval handoff before approval.

Initial dogfood integration should remain explicit:

- record proof only when the runner can compute the same bounded content hash;
- surface presentation ID and content hash in phase output;
- disclose missing proof instead of simulating it;
- keep human/delegated maintainer approval explicit.

## 12. Relationship To WorkReports

WorkReports should eventually cite presentation proof by stable reference.

Deferred work:

- add a WorkReport citation target for approval-presentation records if needed;
- disclose whether proof was present, missing, stale, or not required;
- avoid copying raw approval text into report summaries.

Do not add WorkReport changes in the persistence-only implementation unless
separately scoped.

## 13. Relationship To High-Assurance Approvals

High-assurance approvals should eventually require presentation proof as one of
their required references. That integration should wait until:

- records can be persisted;
- enforcement can validate matching proof;
- stale/missing proof behavior is reviewed;
- report disclosure posture is defined.

Do not couple high-assurance controls to this first persistence phase.

## 14. Privacy And Redaction

Persistence and enforcement must preserve the existing model privacy posture.

They must not store or print:

- raw provider payloads;
- raw command output;
- raw CI logs;
- raw source contents;
- raw spec contents;
- raw chat transcripts;
- screenshots;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- unbounded local filesystem paths.

Debug output, serialization, deserialization, store errors, and enforcement
errors must remain bounded and non-leaking.

## 15. Test Plan

Future persistence tests should cover:

- valid record writes, reads, and lists;
- duplicate write rejection;
- lookup by presentation ID;
- lookup by run ID;
- lookup by run ID and approval ID;
- corrupt record read fails closed without leaking payload;
- invalid file identity mismatch fails closed;
- secret-like lookup input rejected without leakage;
- Debug output redacts local paths and approval text;
- persistence does not mutate workflow events or approval decisions.

Future enforcement tests should cover:

- matching presentation proof allows explicit opt-in approval path;
- missing proof fails closed before approval event append;
- mismatched run ID fails closed;
- mismatched approval ID fails closed;
- mismatched workflow ID/version/schema/step fails closed;
- stale proof fails closed when freshness policy is enabled;
- multiple ambiguous records fail closed unless one is selected explicitly;
- existing default approval path remains unchanged;
- errors do not leak IDs, raw handoff text, paths, payloads, or secret-like
  values;
- existing approval, high-assurance approval, WorkReport, and runtime tests
  still pass;
- docs check passes.

## 16. Proposed Implementation Sequence

1. Implement local approval-presentation record store/helper.
2. Add persistence tests and docs/report.
3. Review persistence boundary.
4. Plan or implement explicit opt-in enforcement path.
5. Add enforcement tests.
6. Review enforcement before any default approval behavior changes.
7. Later, integrate dogfood runner proof recording.
8. Later, integrate WorkReport citation/disclosure.
9. Later, integrate high-assurance approvals.

## 17. Open Questions

- Should approval-presentation records live in the same state backend namespace
  as side-effect and report artifact records?
- Should approval-presentation proof become a workflow event, a sidecar record,
  or both?
- What freshness window should high-assurance approvals require?
- Should delegated-maintainer approvals require proof by default in dogfood
  flows before generic runtime flows do?
- Should missing proof block only grants, or also denials?
- Should approval-presentation records be cited in WorkReports before executor
  enforcement exists?
- Should CLI approval display be implemented before or after enforcement?

## 18. Final Recommendation

Recommended next implementation phase: local approval-presentation record
persistence helper only.

Do not implement opt-in executor enforcement, default approval behavior changes,
CLI mutation behavior, UI/cards, workflow schemas, examples, high-assurance
integration, WorkReport citation changes, provider writes, side effects, hosted
behavior, reasoning lineage, or release posture changes in that first
persistence implementation.
