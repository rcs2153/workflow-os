# CI Log Redaction

CI logs are sensitive by default. They may contain tokens, credentials, internal hostnames, deployment details, stack traces, customer identifiers, or private source paths.

## Default Posture

Workflow OS CI read-only adapters should:

- store log references rather than raw logs
- avoid storing full logs in audit by default
- allow log excerpts only through explicit read actions
- bound excerpt size
- redact sensitive-looking lines before summaries are produced
- treat any leaked token in logs, audit, diagnostics, observability, or health output as a security bug

## Implemented v0 Behavior

The GitHub Actions read-only adapter:

- returns log download references without downloading logs for the default log-reference operation
- supports explicit bounded job log excerpts
- redacts lines containing sensitive-looking terms such as token, secret, password, credential, API key, or authorization
- truncates excerpts before they are stored in adapter summaries

This is a local preview safety layer, not a complete data-loss-prevention system.

## Forbidden Behavior

CI read-only adapters must not:

- store full raw logs in audit records by default
- print credentials in health output
- include authorization headers in diagnostics
- request write, rerun, or dispatch permissions
- treat log access as permission to mutate CI state
