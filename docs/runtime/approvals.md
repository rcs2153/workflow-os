# Approvals

Workflow OS approvals are event-sourced runtime gates. A step with an approval policy must not invoke its skill until an approval is granted.

## Approval Request

The local executor creates an `ApprovalRequested` event before skill execution for approval-gated steps.

An approval request records:

- approval ID
- workflow run ID
- workflow ID
- workflow version
- spec content hash
- step ID
- skill ID
- requested timestamp
- non-secret reason
- optional expiration duration from the workflow definition
- optional concrete expiration timestamp when known
- decision, once one exists

The approval request is also saved as a projection in the approval store. The event log remains the source of truth.

## Waiting State

After `ApprovalRequested`, the run enters `WaitingForApproval`.

While in this state:

- the gated skill is not invoked
- no local handler is called
- restart and rehydration preserve the waiting state
- approval decisions must be submitted through the local approval decision API

## Approval Decisions

An approval decision records:

- approval ID
- actor
- decision timestamp
- decision kind: granted or denied
- non-secret reason
- correlation ID

`ApprovalGranted` records the decision and keeps the run in `WaitingForApproval`. `RunResumed` then moves the run back to `Running`, after which the local executor may request and start the skill invocation.

`ApprovalDenied` records the decision and the v0 local executor immediately emits `RunFailed`. Denial fails closed and does not continue execution.

## Duplicate Decisions

Duplicate decisions are deterministically rejected when a run is still waiting and the approval already has a decision. Decisions after terminal states are rejected.

The v0 local executor resumes granted approvals synchronously, so a second grant normally sees a terminal completed run and is rejected as an approval after terminal state.

## Expiration

The v0 model stores approval expiration metadata but does not run background timers.

If a workflow declares an approval requirement with `expires_after`, the local executor copies that duration onto the approval request. Future timer or worker behavior must emit explicit events for expiration and must fail closed, cancel, or escalate according to documented policy. Silent expiration is not allowed.

## Non-Goals

v0 approvals do not implement:

- identity provider integration
- UI approval screens
- external approval systems
- background expiration timers
- escalation on expiration
- delegated approval groups

These features require future scoped implementation and must preserve the event-sourced approval invariants.
