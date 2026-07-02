# High-Assurance Approval Runtime Enforcement Plan

Status: First pure validation helper implemented; executor integration planned separately.

## 1. Executive Summary

The high-assurance approval control core model now exists and its nested deserialization blocker is fixed.

The next question is how Workflow OS should enforce a narrow subset of those controls at runtime without changing the default local approval path or implying enterprise identity. This plan defines a conservative, opt-in, local-only enforcement path for high-assurance approval decisions.

The first implementation does not make high-assurance approval automatic. It adds an explicit pure validation helper that validates selected high-assurance approval controls against existing event-sourced approval requests and proposed approval decisions before a future protected approval grant path can proceed.

The next executor integration boundary is planned in [High-Assurance Approval Executor Enforcement Plan](high-assurance-approval-executor-enforcement-plan.md). That plan keeps enforcement opt-in and additive, and it leaves the default `LocalExecutor::decide_approval(...)` path unchanged.

This plan does not implement executor-integrated enforcement, write-capable adapters, provider mutations, workflow schema fields, CLI behavior, examples, RBAC, IdP integration, quorum approval, hosted behavior, side-effect execution, reasoning lineage, or release posture changes.

## 2. Goals

- Enforce selected high-assurance approval controls only when explicitly supplied.
- Preserve existing approval behavior for workflows that do not opt in.
- Preserve event-log source-of-truth behavior.
- Preserve approval projection rebuildability.
- Preserve existing `LocalExecutor::decide_approval(...)` behavior unless a separate opt-in API is implemented.
- Prevent self-approval where the control requires requester/approver separation.
- Validate required governance references by stable ID presence where available.
- Validate approval freshness where existing request expiration metadata supports it.
- Fail closed for unsupported high-assurance control requirements.
- Keep errors stable and non-leaking.
- Keep the first implementation local and deterministic.
- Prepare for future WorkReport disclosure of high-assurance approval posture.
- Keep write-capable adapters blocked until this enforcement path is reviewed.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- automatic enforcement for all approvals;
- changes to existing `decide_approval(...)` semantics;
- workflow-declared high-assurance approval controls;
- workflow schema fields;
- CLI behavior;
- examples;
- write-capable adapters;
- provider mutations;
- runtime side-effect execution;
- approval evidence attachment;
- RBAC, IdP, SSO, SCIM, teams, groups, or external directory integration;
- quorum or multi-party approval enforcement;
- role-bound approval authority;
- approval UI;
- hosted or distributed runtime behavior;
- background expiration timers;
- approval revocation events;
- safety-critical certification claims;
- Level 3/4 autonomy enablement;
- reasoning lineage;
- release posture changes.

## 4. Current Runtime Baseline

Implemented today:

- approval-gated steps pause before skill invocation;
- `ApprovalRequested` is appended before approval projection storage;
- `ApprovalRequested` records request actor or system actor, run identity, step identity, skill identity, reason, correlation ID, idempotency key, and optional expiration metadata;
- `ApprovalGranted` appends a decision and `RunResumed` resumes execution;
- `ApprovalDenied` appends a decision and fails closed;
- duplicate approval decisions are rejected;
- decisions after terminal states are rejected;
- approval decisions validate against event-derived approval requests, not only projection state;
- approval projections can be rebuilt from event history;
- SideEffect approval linkage can validate approval references against workflow events;
- WorkReports can cite approval decisions by stable reference.
- a pure high-assurance approval decision validation helper can validate explicit controls and supplied stable references without mutating runtime state.

Not implemented today:

- executor-integrated high-assurance approval runtime enforcement;
- automatic self-approval prevention in `decide_approval(...)`;
- automatic requester/approver separation enforcement in the default approval path;
- automatic required evidence/policy/side-effect/reference enforcement on approval grants;
- minimum approval counts above one;
- quorum;
- external identity or role authority;
- approval revocation;
- expiration timers;
- approval evidence attachment;
- workflow-declared high-assurance controls.

## 5. First Enforcement Boundary

The first runtime enforcement boundary should be an explicit local decision-time helper or executor-adjacent method.

Implemented first shape:

- `validate_high_assurance_approval_decision(...)`.

Deferred possible shape:

- `LocalExecutor::decide_approval_with_high_assurance(...)`, if executor integration is approved later.

The first implementation should take explicit inputs:

- rehydrated `WorkflowRun`;
- approval request ID;
- proposed `LocalApprovalDecisionRequest`;
- one or more `HighAssuranceApprovalControl` values;
- supplied stable reference IDs available to the approval packet;
- current timestamp when expiration-at-decision is checked;
- optional report-disclosure capture structure, if already model-safe.

It should not:

- read hidden global state;
- invent runtime config;
- read workflow schema fields;
- call external identity providers;
- call adapters;
- write side-effect records;
- mutate state by itself unless implemented as an explicit executor method;
- append events before validation succeeds;
- create report artifacts;
- emit CLI output.

The implemented helper follows this boundary. It is pure and in-memory; it returns a `WorkflowOsError` on failure and does not append approval events or mutate state.

## 6. First Supported Control Subset

The first implementation should support only the controls that can be enforced with existing local data.

Supported first:

- requester/approver separation:
  - `SameActorAllowed`;
  - `MustDiffer`;
  - `HumanApproverMustDiffer` as local actor-difference only, with explicit documentation that human identity proof is not implemented;
- minimum approvals equal to `1`;
- required references by stable ID presence for:
  - EvidenceReference;
  - PolicyDecision event;
  - SideEffect;
  - ValidationReference;
  - LocalCheckResult;
  - WorkflowEvent;
  - AuditEvent;
  - WorkReport;
  - AdapterTelemetry;
- expiration policy where existing approval request metadata and supplied current time support decision-time validation;
- denial behavior vocabulary only when it maps to existing fail-closed behavior.

Rejected until separately implemented:

- `minimum_approvals > 1`;
- quorum;
- role-bound authority;
- external human identity proof;
- revocation enforcement;
- expiration-at-use if there is no reviewed protected-use checkpoint;
- background timers;
- evidence sufficiency beyond stable reference presence;
- automatic report disclosure population;
- side-effect execution;
- write-capable adapter authorization.

## 7. Control Source

For the first implementation, controls should be explicit inputs.

Do not add workflow schema fields yet.

Do not add runtime config yet.

Do not infer high-assurance controls from policy effect strings yet.

Rationale:

- explicit inputs are testable;
- they avoid false governance from decorative YAML;
- they avoid schema stability questions;
- they let the runtime enforcement semantics be reviewed before exposure through workflow authoring.

Future sources may include governance profiles, policy effects, workflow-local config, steward-managed profile config, or harness contracts, but those should be planned after the opt-in runtime boundary is reviewed.

## 8. Requester/Approver Separation Policy

The first implementation should compare the approval request actor or system actor against the proposed decision actor.

Rules:

- if `SameActorAllowed`, do not reject same actor;
- if `MustDiffer`, reject when requester actor and decision actor are the same stable actor ID;
- if `HumanApproverMustDiffer`, reject same actor and document that v0 does not prove human identity beyond local actor metadata;
- if the request lacks a comparable requester identity, fail closed for separation-required controls;
- errors must not include raw actor IDs.

This enforces a meaningful local invariant without claiming enterprise identity assurance.

## 9. Required Reference Policy

Required references should be evaluated by stable IDs supplied to the high-assurance approval packet.

Rules:

- required references must be present by name and kind;
- the supplied reference kind must match the control target kind;
- duplicate supplied reference names should fail closed;
- missing required references should fail closed;
- optional references may be absent;
- reference validation should not read payloads;
- reference validation should not create `EvidenceReference` values;
- reference validation should not fabricate IDs.

The first implementation may validate reference presence and type only. It should not claim evidence sufficiency, semantic correctness, or external proof.

## 10. Expiration Policy

Current approval requests can carry expiration metadata, but there are no background timers.

The first implementation should support only decision-time checks that can be performed deterministically:

- `NotRequired`: no expiration check;
- `RequiredOnRequest`: approval request must include expiration metadata;
- `MustBeUnexpiredAtDecision`: supplied current time must be before the request expiration timestamp;
- `MustBeUnexpiredAtUse`: reject unless a reviewed protected-use checkpoint exists, or treat as unsupported in the first implementation.

No silent expiration is allowed. If later expiration events are added, they must be explicit workflow events with reviewed state transitions.

## 11. Revocation Policy

Revocation enforcement should remain deferred.

The first implementation should:

- accept `Unsupported`;
- reject `ExplicitEventBeforeUse` as unsupported until revocation events exist;
- reject or treat `ReportOnlyAfterUse` as model-only until report disclosure behavior is implemented.

This avoids pretending revocation exists when the runtime has no source-of-truth event for it.

## 12. Failure Behavior

High-assurance enforcement must fail closed.

Recommended behavior:

- if enforcement is called as a helper, return a structured `WorkflowOsError` and do not mutate runtime state;
- if enforcement is called through an executor-adjacent approval method, reject the approval decision before appending `ApprovalGranted`, `ApprovalDenied`, or `RunResumed`;
- unsupported high-assurance control requirements must fail closed;
- validation errors must use stable codes and avoid raw actor IDs, reference IDs, side-effect IDs, paths, snippets, payloads, tokens, or secret-like values.

Report-generation or disclosure failures should not be mixed into approval enforcement until separately planned.

## 13. Event And Audit Semantics

For the first implementation, high-assurance validation should not create new workflow events.

If implemented as a helper, it should be pure validation.

If implemented as an executor-adjacent method:

- failed high-assurance validation should append no approval decision events;
- successful validation may allow the existing approval decision path to append the same existing events;
- no new audit event type should be added unless separately planned;
- existing approval audit projection should remain unchanged unless a later report/disclosure phase requires more detail.

Future phases may add explicit high-assurance validation events, but only after state-machine and audit semantics are planned.

## 14. WorkReport Disclosure

The first enforcement implementation should prepare data for WorkReport disclosure but should not automatically generate or mutate reports.

Future reports should be able to disclose:

- high-assurance control ID and version;
- whether requester/approver separation was required;
- whether it was satisfied;
- which required reference kinds were present or missing;
- expiration posture;
- unsupported requirements that blocked the decision;
- final approval decision.

Disclosure should cite stable references and bounded summaries only.

## 15. Relationship To SideEffects And Writes

High-assurance approval enforcement remains a prerequisite for write-capable adapters, not an authorization to add writes.

The first implementation should not authorize provider mutations.

Future write-capable adapter work must still compose:

- policy effect enforcement;
- high-assurance approval enforcement where required;
- SideEffect proposed/attempted/completed lifecycle;
- side-effect approval linkage;
- idempotency;
- audit;
- evidence references;
- WorkReport disclosure.

Credentials alone must never imply authority.

## 16. Test Plan

Future implementation tests should cover:

- valid high-assurance approval decision with `SameActorAllowed`;
- valid approval decision with `MustDiffer` and different actors;
- same actor rejected for `MustDiffer`;
- same actor rejected for `HumanApproverMustDiffer`;
- missing requester identity fails closed for separation-required controls;
- minimum approval count of `1` accepted;
- minimum approval count greater than `1` rejected as unsupported;
- required EvidenceReference ID present succeeds;
- required policy decision event ID present succeeds;
- required SideEffect ID present succeeds;
- required validation/local-check/workflow/audit/report/adapter-telemetry references present succeed;
- missing required reference fails closed;
- mismatched reference kind fails closed;
- duplicate supplied reference names fail closed;
- expiration required on request succeeds when metadata is present;
- expiration required on request fails when metadata is absent;
- unexpired-at-decision succeeds before expiration;
- unexpired-at-decision fails after expiration;
- unexpired-at-use rejected or deferred until protected-use checkpoint exists;
- revocation-required policies reject as unsupported;
- helper returns no events and mutates no runtime state;
- executor-adjacent API, if implemented, appends no approval events on high-assurance failure;
- existing `decide_approval(...)` behavior remains unchanged;
- no raw payloads, actor IDs, reference IDs, paths, tokens, or secret-like values leak in errors or `Debug`;
- existing approval, SideEffect, WorkReport, policy, validation, adapter, and runtime tests still pass.

## 17. Proposed Implementation Sequence

Recommended small phases:

1. Add a pure in-memory high-assurance approval decision validation helper.
2. Support requester/approver separation and required reference presence only.
3. Add focused tests for supported and unsupported control requirements.
4. Review.
5. Add an explicit executor-adjacent method that calls the helper before approval grant, without changing `decide_approval(...)`.
6. Review.
7. Plan WorkReport disclosure of high-assurance approval posture.
8. Only after review, consider policy/profile/workflow-declared control sources.
9. Only after enforcement and disclosure are reviewed, revisit write-capable adapter readiness.

## 18. Open Questions

- Should the first enforcement helper live in `high_assurance_approval.rs` or executor-adjacent code?
- Should the first executor path be `decide_approval_with_high_assurance(...)` or a wrapper around `LocalApprovalDecisionRequest`?
- Should enforcement failures be audit-projected later, or remain pure validation failures for the first slice?
- How should WorkReports cite high-assurance validation results without creating a new report model family?
- Should governance profiles become the first source of controls, or should policy effects do that later?
- How should local actor metadata distinguish human, agent, and system actor without IdP claims?
- What is the smallest useful protected-use checkpoint for expiration-at-use?
- When should revocation events enter the state machine?

## 19. Final Recommendation

Next implementation phase: **executor-integrated high-assurance approval enforcement, opt-in method only**.

The pure helper is implemented and reviewed. The next implementation should add a narrow explicit executor method that calls the helper before appending approval decision events, without changing the default `decide_approval(...)` path.

Do not build write-capable adapters, workflow schema fields, runtime config, CLI behavior, examples, RBAC/IdP, quorum approval, revocation events, side-effect execution, hosted behavior, reasoning lineage, or release posture changes in that implementation.
