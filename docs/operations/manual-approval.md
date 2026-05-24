# Manual Approval

Manual approval in v0 is a local runtime operation. It is intended for a future CLI command, not for UI or external approval integrations.

## Operator Flow

1. Start a local run.
2. If a step requires approval, the run stops in `WaitingForApproval`.
3. Inspect the approval request ID, workflow ID, run ID, step ID, skill ID, reason, and expiration metadata.
4. Submit a grant or denial with actor, reason, and correlation ID.
5. Rehydrate the run from durable state to verify the result.

## Grant Behavior

A grant appends:

- `ApprovalGranted`
- `RunResumed`
- normal skill invocation events
- `RunCompleted` or `RunFailed`

The skill must not execute before the grant.

## Denial Behavior

A denial appends:

- `ApprovalDenied`
- `RunFailed`

The run fails closed. It must not invoke the skill after denial.

## Restart Safety

Approval waits are durable. A local process may stop after `ApprovalRequested`; a later process can rehydrate the run and submit the decision against the same event history.

## Security Notes

Approval reasons and event metadata must be non-secret. Sensitive payloads must be represented by references or summaries according to the redaction rules.

v0 does not authenticate actors through an identity provider. The actor field is an auditable local identifier, not proof of external identity.
