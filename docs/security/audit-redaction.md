# Audit Redaction

Audit and observability records must not expose secrets.

## Rules

- Store references and summaries instead of raw sensitive payloads.
- Redact sensitive-looking output references before local audit recording.
- Preserve redaction metadata so operators can see that a field was intentionally hidden.
- Do not include raw skill input values in audit records.
- Do not include raw secrets in structured logs.

## v0 Enforcement

The v0 local executor emits audit records from workflow events. Output references that look like secrets, tokens, passwords, credentials, or API keys are replaced with `[REDACTED]` in audit records.

Structured runtime logs record event metadata, not raw payloads.

## Future Adapter Boundary

Future adapters and secret providers must enforce the same rules:

- capability-gate secret access
- policy-gate secret access
- audit the access using references
- avoid raw sensitive payloads in logs and audit records
- prove redaction behavior with tests
