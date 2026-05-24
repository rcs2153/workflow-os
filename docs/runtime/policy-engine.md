# Policy Engine

The v0 local runtime uses a deterministic conservative policy engine before meaningful actions.

## Runtime Actions

The current action model includes:

- `StartWorkflow`
- `RequestApproval`
- `ResumeWorkflow`
- `InvokeSkill`
- `InvokeAdapter`
- `CancelWorkflow`
- `InspectWorkflow`
- `Unknown`

Unknown actions are denied.

## Runtime Enforcement

The local executor checks policy:

- before new execution starts
- before approval requests
- before approval resume
- before local skill invocation
- before cancellation
- before adapter-capability actions, even though adapters are not implemented

Allowed policy decisions are recorded with `PolicyDecisionRecorded` before the action. Denied decisions are also recorded when a run exists; the executor fails closed for denied skill or adapter invocation.

## Kill Switch

The kill switch denies new execution and non-terminal mutating actions. Safe cancellation and inspection remain allowed.

The kill switch is local runtime configuration in v0. It is not a distributed control plane.

## Audit Event

`PolicyDecisionRecorded` captures:

- action
- capabilities
- actor
- workflow ID
- run ID when available
- correlation ID
- allow/deny result
- approval requirement flag
- reason codes
- violations

The event log remains the source of truth for policy audit.

The audit foundation also emits an `AuditEvent` projection for this runtime event. The projection records the policy decision reference, non-secret decision summary, correlation ID, actor, and workflow/run identity.
