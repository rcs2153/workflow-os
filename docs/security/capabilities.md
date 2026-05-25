# Capabilities

Capabilities describe what an action needs permission to do. Workflow OS uses capabilities so policy can reason about side effects before they happen.

## v0 Capability Set

The v0 model includes:

- `local.read`
- `local.write`
- `external.read`
- `external.write`
- `approval.request`
- `workflow.cancel`
- `workflow.resume`
- `adapter.invoke`
- `secret.read`
- `audit.write`

Unknown capabilities are denied.

## Conservative Rules

`external.write` is denied in v0 because write-capable adapters are not implemented.

`adapter.invoke` is denied in v0 because adapter execution is deferred.

`secret.read` is denied unless future explicit configuration enables it.

Local deterministic skills receive local read/write and audit capabilities by default. Skills that declare capabilities are evaluated against those declared capabilities.

## Security Boundary

Capabilities are not RBAC roles and do not prove identity. They are explicit runtime requirements that the policy engine can allow or deny.

Future secret providers, identity systems, and external adapters must map into this capability model rather than bypass it.
