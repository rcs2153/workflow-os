# Integration Capabilities

Adapter capabilities describe what an adapter operation is allowed to request. They are not permissions by themselves. Runtime policy must still allow the action before side effects.

## v0 Capability Concepts

- `external_read`: read from an external system.
- `external_write`: write to an external system.
- `local_read`: read deterministic local data.
- `local_write`: write deterministic local data.
- `event_source`: poll or receive events.
- `capability_discovery`: report supported capabilities.
- `dry_run`: plan without side effects.

Unknown capabilities fail closed.

## External Writes

External writes are the highest-risk integration boundary. They must be:

- declared by capability
- policy-gated
- approval-gated where required
- idempotency-keyed
- auditable
- represented through references and summaries

Adapters must not treat possession of a credential as permission to act. Workflow OS policy and runtime state remain authoritative.
