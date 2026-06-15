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
- before Phase 2 GitHub, Jira, and GitHub Actions read-only adapter actions

Every policy decision is written to the durable policy audit ledger before the runtime proceeds with the action or fails closed.

For decisions after a run exists, allowed and denied decisions are also recorded with `PolicyDecisionRecorded` in the workflow event stream before the action. The executor fails closed for denied skill or adapter invocation.

Phase 2 allows the explicitly supported `symbolic/github-read-only`, `symbolic/jira-read-only`, `symbolic/ci-read-only`, and `symbolic/github-actions-read-only` adapter paths when the action is `InvokeAdapter`, the declared capability is read-only (`external.read`), and no write, secret, or unknown capability is present. Other adapter invocation remains denied by default.

Adapter request construction must preserve policy-precheck provenance. Runtime paths must pass a runtime policy decision or approval-decision precheck into the adapter request. Fixture/test helpers may use fixture/test provenance only for offline examples and contract tests. Public adapter helpers must not silently authorize requests.

For decisions before `RunCreated`, including denied workflow starts, the executor writes a pre-run `PolicyAuditRecord` instead of creating a misleading workflow run. The record may include the pending run ID, workflow ID, schema version, workflow version, spec hash, actor, correlation ID, decision, reason codes, and policy context.

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

For run-scoped policy decisions, the workflow event log remains the source of truth and the durable policy audit ledger provides an audit-shaped index. For pre-run decisions, the durable policy audit ledger is the source of truth because no workflow event stream exists yet.

The audit foundation also emits an `AuditEvent` projection for this runtime event. The projection records the policy decision reference, non-secret decision summary, correlation ID, actor, and workflow/run identity.

Pre-run policy decisions are emitted to audit sinks as `PolicyAuditRecord` values. Audit sink failures are returned as structured errors after the durable policy audit record has been written.
