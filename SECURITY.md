# Security Policy

Security is part of the Workflow OS core contract.

## Supported Versions

Workflow OS has not made a stable release yet. Until a first supported release exists, security fixes are handled on the main development line.

## Reporting A Vulnerability

Do not open a public issue for a suspected vulnerability.

Report security concerns by contacting the maintainers listed in [MAINTAINERS.md](MAINTAINERS.md). Include:

- A description of the issue.
- Steps to reproduce, if available.
- Affected files, commands, or workflows.
- Any known impact.
- Whether the report includes sensitive information.

Maintainers should acknowledge receipt, assess severity, coordinate a fix, and document disclosure timing before public discussion.

## Security Expectations

- Secrets must not be committed to workflow specs, examples, tests, or documentation.
- Logs and audit events must redact sensitive fields.
- External writes must be capability-gated, policy-gated, auditable, and idempotent.
- Unknown or unsafe actions must fail closed.
