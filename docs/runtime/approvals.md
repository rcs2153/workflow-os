# Approvals

Workflow OS approvals are event-sourced runtime gates. A step with an approval policy must not invoke its skill, or emit an event that implies invocation authorization, until an approval is granted.

## Approval Request

The local executor emits `StepScheduled`, records the approval policy decision, and then creates an `ApprovalRequested` event before skill execution for approval-gated steps. `SkillInvocationRequested` is not emitted while the run is waiting for approval.

An approval request records:

- approval ID
- workflow run ID
- workflow ID
- schema version
- workflow version
- spec content hash
- step ID
- skill ID
- skill version
- actor or system actor that requested approval
- requested timestamp
- correlation ID
- idempotency key for the gated invocation where relevant
- non-secret reason
- optional expiration duration from the workflow definition
- optional concrete expiration timestamp when known
- decision, once one exists

The local executor appends `ApprovalRequested` before saving the approval projection. The approval store is a rebuildable cache for lookup convenience; the event log remains the source of truth.

## Waiting State

After `ApprovalRequested`, the run enters `WaitingForApproval`.

While in this state:

- the gated skill is not invoked
- no local handler is called
- restart and rehydration preserve the waiting state
- approval decisions must be submitted through the local approval decision API
- a missing approval projection can be rebuilt from the event-derived run snapshot

## Approval Decisions

An approval decision records:

- approval ID
- actor
- decision timestamp
- decision kind: granted or denied
- non-secret reason
- correlation ID

`ApprovalGranted` records the decision and keeps the run in `WaitingForApproval`. `RunResumed` then moves the run back to `Running`, after which the local executor may emit `SkillInvocationRequested` and start the skill invocation.

`ApprovalDenied` records the decision and the v0 local executor immediately emits `RunFailed`. Denial fails closed and does not continue execution.

Approval decisions validate against the rehydrated event-derived approval request, not only the approval projection. A projection without a matching `ApprovalRequested` event cannot authorize a grant or denial.

The v0 CLI exposes both decisions through `workflow-os approve <run-id> <approval-id>` for grants and `workflow-os approve <run-id> <approval-id> --deny --reason <reason>` for denials. Denial requires an explicit reason in the CLI path.

## Duplicate Decisions

Duplicate decisions are deterministically rejected when a run is still waiting and the approval already has a decision. Decisions after terminal states are rejected.

The v0 local executor resumes granted approvals synchronously, so a second grant normally sees a terminal completed run and is rejected as an approval after terminal state.

## Expiration

The v0 model stores approval expiration metadata but does not run background timers.

If a workflow declares an approval requirement with `expires_after`, the local executor copies that duration onto the approval request. Future timer or worker behavior must emit explicit events for expiration and must fail closed, cancel, or escalate according to documented policy. Silent expiration is not allowed.

## High-Assurance Approval Controls

High-assurance approval controls are planned in [High-Assurance Approval Controls Plan](../implementation-plans/high-assurance-approval-controls-plan.md), and the core model is implemented as model vocabulary and validation in [High-Assurance Approval Control Core Model Report](../concepts/HIGH_ASSURANCE_APPROVAL_CONTROL_CORE_MODEL_REPORT.md). The model blocker fix is accepted in [High-Assurance Approval Control Core Model Blocker Fix Review](../concepts/HIGH_ASSURANCE_APPROVAL_CONTROL_CORE_MODEL_BLOCKER_FIX_REVIEW.md), opt-in runtime enforcement is planned in [High-Assurance Approval Runtime Enforcement Plan](../implementation-plans/high-assurance-approval-runtime-enforcement-plan.md), the first pure decision-validation helper is implemented in [High-Assurance Approval Runtime Validation Helper Report](../concepts/HIGH_ASSURANCE_APPROVAL_RUNTIME_VALIDATION_HELPER_REPORT.md), and the first opt-in executor boundary is implemented in [High-Assurance Approval Executor Enforcement Report](../concepts/HIGH_ASSURANCE_APPROVAL_EXECUTOR_ENFORCEMENT_REPORT.md). The intended direction is approval with explicit requester/approver posture, evidence, policy, audit, expiration/revocation, and report-disclosure requirements before a protected action can proceed.

Future controls may include:

- multi-party approval or quorum rules;
- separation of requester and approver;
- role-bound approval authority;
- prevention of self-approval for sensitive actions;
- approval expiry, revocation, and escalation semantics;
- evidence-required approval contexts;
- immutable approval audit trails;
- final work-report disclosure of approvals requested, granted, denied, expired, skipped, or deferred.

The implemented helper is explicit, local, and in-memory. It can validate selected controls and supplied stable references without mutating workflow state. The opt-in executor method validates before appending approval decision events while leaving the default `decide_approval(...)` path unchanged. This is not a safety-critical certification claim. Later scoped runtime phases must be implemented and reviewed before write-capable adapters or high-risk external actions depend on it.

## Non-Goals

v0 approvals do not implement:

- identity provider integration
- UI approval screens
- external approval systems
- background expiration timers
- escalation on expiration
- delegated approval groups
- multi-party or quorum approvals
- role-based approval authority
- approval revocation
- safety-critical certification controls

These features require future scoped implementation and must preserve the event-sourced approval invariants.
