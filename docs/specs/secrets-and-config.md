# Secrets And Config

Secrets must not be stored in Workflow OS specs.

This applies to:

- `workflow-os.yml`
- `workflows/*.workflow.yml`
- `skills/*.skill.yml`
- `policies/*.policy.yml`
- `tests/*.test.yml`

## Disallowed In Specs

Specs must not contain:

- Passwords.
- API keys.
- Access tokens.
- Private keys.
- Secret literals.
- Provider credentials.
- Environment-specific sensitive payloads.

The v0 parser rejects obvious secret-bearing field names such as `secret`, `secrets`, `password`, `token`, `api_key`, `apikey`, and `private_key`.

The parser also rejects string values with obvious secret prefixes such as `secret:`, `token:`, and `password:`.

This is a guardrail, not a complete secret scanner. Contributors must still avoid putting secrets in specs.

## Non-Secret Config

`workflow-os.yml` may contain non-secret config overlays:

```yaml
config:
  - environment: dev
    vars:
      - name: approval_timeout
        value: 1h
```

Config overlays are for non-sensitive values only.

## Secret References

Future secret handling must use explicit references such as environment variables or secret provider references. Real secret provider behavior is not implemented in v0.

Future secret references must be:

- Capability-gated.
- Policy-gated.
- Auditable.
- Redacted in diagnostics, logs, and audit events.

## Environment Overlays In v0

v0 environment overlays are declarative metadata only. They do not execute, load external secret stores, mutate runtime behavior, or select adapters.

Overlay semantics beyond non-secret variables require future documented validation and tests.
