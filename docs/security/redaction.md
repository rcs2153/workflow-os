# Redaction

Workflow OS must not accidentally expose secrets through logs, debug output, diagnostics, audit events, or serialized records.

## Redacted Values

The Rust core provides a sensitive-value wrapper named `RedactedValue`.

`RedactedValue`:

- Displays as `[REDACTED]`.
- Debug-formats as `RedactedValue([REDACTED])`.
- Serializes as `[REDACTED]`.
- Requires explicit access to expose the inner value.

This wrapper is intended for values that may need to move through core APIs but must not appear in routine logs or CLI output.

## Rules

- Do not store secrets in workflow specs.
- Do not include raw secrets in diagnostics.
- Do not include raw secrets in audit events.
- Do not include raw secrets in structured logs.
- Prefer secret provider references or environment variable references.
- Prefer summaries and references over full sensitive payloads.

## Explicit Exposure

Calling code may explicitly expose a wrapped secret only at a boundary that is designed to handle sensitive values, such as a future secret provider or adapter implementation.

Any such boundary must be:

- Capability-gated.
- Policy-gated.
- Auditable.
- Tested for redaction behavior.

## Future Requirements

Future adapter and runtime implementations must prove that sensitive values are not exposed through `Display`, `Debug`, logs, diagnostics, audit events, or default serialization.

See [audit redaction](audit-redaction.md) for the v0 audit-specific rules.
