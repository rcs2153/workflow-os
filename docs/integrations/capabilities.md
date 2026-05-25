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

## Phase 2 Read-Only Capabilities

Phase 2 reserves provider-specific read capabilities:

- `github.read`
- `jira.read`
- `ci.read`

Read-only adapter requests must declare the relevant read capability. The runtime policy layer must still allow the adapter action before the adapter call is made.

These capabilities are intentionally narrower than generic `external_read`. The GitHub read-only adapter requests `github.read`; the Jira read-only adapter requests `jira.read`; the GitHub Actions read-only adapter requests `ci.read`.

Adapter requests must carry explicit policy pre-check provenance. Runtime paths must use runtime policy or approval-decision provenance. Fixture examples and offline contract tests may use fixture/test provenance, but that provenance must be visible on the request and must not be described as production authorization.

## Phase 2 Denied Capabilities

Phase 2 keeps these capabilities denied or unsupported:

- `github.write`
- `jira.write`
- `ci.write`
- `ci.rerun`
- `adapter.write`

These names are reserved so future phases can discuss them precisely. They do not enable writes in Phase 2.

## External Writes

External writes are the highest-risk integration boundary. They must be:

- declared by capability
- policy-gated
- approval-gated where required
- idempotency-keyed
- auditable
- represented through references and summaries

Adapters must not treat possession of a credential as permission to act. Workflow OS policy and runtime state remain authoritative.
