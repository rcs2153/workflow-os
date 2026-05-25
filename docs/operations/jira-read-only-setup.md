# Jira Read-Only Setup

The Jira read-only adapter is optional. Workflow OS local kernel use does not require Jira credentials.

This runbook is for Phase 2 development-branch read-only adapter evaluation. Jira read-only live mode is not part of the `0.1.0-preview.1` local kernel release contract, and it is not a public read-only integration preview until a follow-up maintainer review approves that posture.

## Fixture Mode

Fixture mode is used by normal tests and CI. It does not call Jira and does not require credentials.

Use fixture mode for:

- unit tests
- contract tests
- offline development
- CI validation

## Live Read-Only Mode

Live read-only mode calls Jira REST read APIs. It is opt-in.

Set the Jira base URL:

```sh
export WORKFLOW_OS_JIRA_BASE_URL=https://example.atlassian.net
```

For Atlassian Cloud, use email plus API-token Basic auth:

```sh
export WORKFLOW_OS_JIRA_EMAIL=person@example.com
export WORKFLOW_OS_JIRA_API_TOKEN=...
```

Fallback variable names are also recognized:

```sh
export JIRA_EMAIL=person@example.com
export JIRA_API_TOKEN=...
```

For Jira deployments that explicitly support bearer tokens, use:

```sh
export WORKFLOW_OS_JIRA_BEARER_TOKEN=...
```

The legacy `WORKFLOW_OS_JIRA_TOKEN` variable is treated as a bearer token fallback. When complete Basic auth and bearer auth are both configured, Basic auth takes precedence.

Partial Basic auth is not accepted. Set both email and API token, or use a supported bearer token.

Do not put tokens in Workflow OS specs.

## Health Checks

Jira adapter health reports:

- adapter ID
- adapter kind
- operation mode
- configured/unconfigured
- credential present or absent
- last checked timestamp
- warnings

Health output must never print the token value or token prefix.
Health output reports partial Basic auth as a warning without printing the email or token.

## Live Test Opt-In

Live tests are skipped by default. To run them manually:

```sh
export WORKFLOW_OS_LIVE_JIRA_TESTS=1
export WORKFLOW_OS_JIRA_BASE_URL=https://example.atlassian.net
export WORKFLOW_OS_JIRA_EMAIL=person@example.com
export WORKFLOW_OS_JIRA_API_TOKEN=...
export WORKFLOW_OS_JIRA_TEST_ISSUE_KEY=OPS-42
cargo test -p workflow-core --test jira_adapter -- --ignored
```

Use a token with the least privilege needed for the issue being read.

Maintainers can also run the documented smoke wrapper:

```sh
npm run smoke:jira-live
```

Use only maintainer-approved non-sensitive Jira projects and issues for live smoke testing.

## Troubleshooting

Common failures:

- `auth_failure`: token missing, expired, revoked, or invalid
- `permission_failure`: token lacks access to the issue or project
- `not_found`: issue or project does not exist or is not visible to the token
- `rate_limited`: Jira rate limit exhausted
- `timeout`: Jira did not respond within the adapter timeout
- `malformed_response`: response shape was not valid JSON for JSON endpoints

See [integration troubleshooting](integration-troubleshooting.md) for generic adapter diagnostics.
