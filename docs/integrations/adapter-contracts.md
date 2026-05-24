# Adapter Contracts

Workflow OS adapters are future integration boundaries. They are the only place external systems may be read from or written to, and they must never mutate core workflow state directly.

v0 defines contracts only. It does not implement real GitHub, Jira, CI, or generic HTTP clients.

## Concepts

- `AdapterId`: stable adapter identifier.
- `AdapterKind`: symbolic adapter kind. v0 reserves `github`, `jira`, `ci`, `local`, and `generic-http`.
- `AdapterCapability`: capability required by an adapter operation.
- `AdapterAction`: named operation plus whether it may create side effects.
- `AdapterRequest`: request envelope carrying action, capabilities, correlation ID, idempotency key where needed, redaction strategy, and policy pre-check.
- `AdapterResponse`: non-secret summary plus external reference.
- `AdapterError`: classified adapter failure.
- `AdapterEvent`: non-secret event emitted by an event-source adapter.
- `AdapterHealth`: health-check result.
- `AdapterInvocationRecord`: audit-safe record of an adapter attempt.

## Traits

The Rust core defines traits for:

- read-only operations
- write-capable operations
- event-source operations
- health checks
- capability discovery
- dry-run or plan mode

Implementations must live behind these traits. They must not call core state mutation APIs directly.

## Side-Effect Preconditions

External writes require all of the following before an adapter operation can run:

- declared `external_write` capability
- correlation ID
- idempotency key
- policy allow or approval-granted pre-check
- redaction strategy

Unknown capabilities fail closed.

## Audit And Redaction

Adapter responses should store external references and summaries, not raw payloads. Sensitive-looking summaries are redacted by the contract helpers. Future adapters must preserve this rule for logs, audit events, and runtime event payloads.

## Non-Goals

v0 adapter contracts do not include:

- OAuth flows
- API clients
- network calls
- webhook receivers
- distributed queues
- production secret providers

Those features require future scoped work after the kernel remains correct under local execution.
