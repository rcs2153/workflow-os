# Audit Redaction

Audit and observability records must not expose secrets.

## Rules

- Store references and summaries instead of raw sensitive payloads.
- Mark audit fields as safe metadata, reference-only, or redacted.
- Redact sensitive-looking decision summaries and output references before local audit recording.
- Preserve redaction metadata so operators can see that a field was intentionally hidden or intentionally stored as a reference.
- Do not include raw skill input values in audit records.
- Do not include raw secrets in structured logs.

## v0 Enforcement

The v0 local executor emits audit records from workflow events. Skill input values are not copied into audit records; audit stores invocation input references instead.

Output references and decision summaries that look like secrets, tokens, passwords, credentials, or API keys are replaced with `[REDACTED]` in audit records. The matching is deterministic and conservative for v0.

Redaction metadata records:

- `safe` for non-secret summaries.
- `reference_only` for input/output references.
- `redacted` for fields replaced with `[REDACTED]`.

Structured runtime logs record event metadata, not raw payloads.

The v0 implementation is local and schema-aware at the contract boundary: sensitive contract fields are rejected or required to declare redaction during validation, and runtime audit avoids raw payloads. Deep payload inspection and enterprise data-loss-prevention integrations are deferred.

## Future Adapter Boundary

Future adapters and secret providers must enforce the same rules:

- capability-gate secret access
- policy-gate secret access
- audit the access using references
- avoid raw sensitive payloads in logs and audit records
- prove redaction behavior with tests
