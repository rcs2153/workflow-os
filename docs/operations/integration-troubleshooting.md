# Integration Troubleshooting

The Workflow OS development branch implements Phase 2 GitHub, Jira, and CI/GitHub Actions read-only adapters for internal review. They are not part of the `0.1.0-preview.1` local kernel release contract, and they are not a public read-only integration preview until a follow-up maintainer review approves that posture.

OAuth, webhook, production integrations, and write-capable adapters remain future work. This runbook defines the troubleshooting posture read-only adapters must support.

## Contract Gate

Run the offline integration gate before debugging live providers:

```sh
npm run check:integrations
```

This command runs the focused GitHub, Jira, and CI adapter tests plus the fixture-backed reference examples. It should pass without live credentials. If it fails, fix the fixture, adapter contract, policy, redaction, audit, or example issue before investigating provider-specific live behavior.

See [integration testing](../integrations/testing.md) for the full gate posture.

## First Checks

For any adapter issue, collect:

- adapter ID
- adapter kind
- operation mode
- action
- declared capability
- actor or system actor
- correlation ID
- workflow run ID if run-scoped
- adapter invocation record
- observability record
- classified adapter error

Do not collect raw credentials, authorization headers, provider tokens, private keys, or raw sensitive provider payloads.

## Operation Mode

Confirm the adapter mode:

- `fixture`: check fixture file path and fixture validity.
- `mock`: check deterministic mock setup.
- `local`: check local configuration.
- `live-read-only`: check credential presence, network reachability, provider permissions, and rate limits.
- `live-write-capable`: unsupported in Phase 2 and expected to fail closed.

CI should use fixture, mock, or local modes by default. Live read-only tests must be opt-in.

## Common Failure Classes

Adapter errors must use one of the standard classifications:

- authentication failure
- permission failure
- not found
- rate limited
- timeout
- validation failure
- malformed response
- transient network failure
- unsupported operation
- policy denied
- unknown

Unknown errors should be treated as unsafe until classified more precisely.

## Credential Issues

Health output may say credentials are present or absent. It must not reveal credential values.

If credentials are missing:

- Confirm the documented environment variable or local secret reference is configured.
- Confirm the spec does not contain credential values.
- Confirm the adapter health check reports absence without leaking secrets.

For Jira live read-only mode:

- Atlassian Cloud uses Basic auth from `WORKFLOW_OS_JIRA_EMAIL` plus `WORKFLOW_OS_JIRA_API_TOKEN`.
- `JIRA_EMAIL` plus `JIRA_API_TOKEN` are fallback variable names.
- `WORKFLOW_OS_JIRA_BEARER_TOKEN` is supported only for Jira deployments that explicitly accept bearer tokens.
- Complete Basic auth takes precedence over bearer auth.
- Partial Basic auth is reported as a health warning and does not authorize live reads by itself.

If credentials are present but calls fail:

- Check provider permissions.
- Check whether the credential is read-only.
- Check rate limits and provider status.
- Check policy decision records for denial.

## Policy Denials

Policy denial is expected when:

- capability is missing
- capability is unknown
- write capability is requested in Phase 2
- action is unknown
- actor context is missing
- adapter mode is unsupported

Policy denials must be visible in audit records and should not be retried as transient failures.

## Redaction Problems

If audit or observability output contains raw sensitive payloads, treat it as a security bug.

Expected behavior:

- summaries are non-secret
- external references are preferred
- sensitive-looking summaries are redacted
- health output never includes credential values
- request metadata is non-secret

## Live Read-Only Failures

For opt-in live read-only tests:

- Complete the [Phase 2 live smoke environment checklist](../integrations/PHASE_2_LIVE_SMOKE_ENVIRONMENT_CHECKLIST.md) before running maintainer live smoke commands.
- Confirm the live-test environment variable is explicitly set.
- Confirm credentials are read-only.
- Confirm tests do not mutate provider state.
- Confirm CI is not configured to require live credentials.
- Confirm captured fixtures do not contain secrets.
- Confirm the provider-specific smoke-test environment variables match [maintainer live smoke tests](../integrations/live-smoke-tests.md).
- Confirm the test repository, issue, or workflow run is maintainer-approved and non-sensitive.

Live read-only failures should not block offline contract tests unless the adapter contract itself is broken.

If live smoke output contains a token, token prefix, authorization header, private issue body, private repository content, or raw sensitive CI log, stop troubleshooting and treat the incident as a credential or sensitive-data exposure. Rotate exposed credentials before retrying.

## Unsupported Phase 2 Behavior

These failures are expected in Phase 2:

- creating branches
- opening pull requests
- posting comments
- updating Jira issues
- changing Jira status
- rerunning CI
- workflow dispatch
- write-capable adapter mode
- webhook ingestion
- OAuth app behavior

Do not work around these denials in adapter code. They are intentional product boundaries.
