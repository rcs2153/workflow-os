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

## Future High-Assurance Approval Controls

High-assurance approval controls are a future roadmap capability for sensitive or irreversible actions. The intended direction is multi-party, role-bound approval with explicit evidence, policy, and audit requirements before a protected action can proceed.

Future controls may include:

- multi-party approval or quorum rules;
- separation of requester and approver;
- role-bound approval authority;
- prevention of self-approval for sensitive actions;
- approval expiry, revocation, and escalation semantics;
- evidence-required approval contexts;
- immutable approval audit trails;
- final work-report disclosure of approvals requested, granted, denied, expired, skipped, or deferred.

This is not a current v0 capability and is not a safety-critical certification claim. It must be planned and reviewed before write-capable adapters or high-risk external actions depend on it.

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
