# Jira Token Scopes

The Jira read-only adapter must use least-privilege credentials.

## Recommended Token Posture

For Atlassian Cloud, use email plus API-token Basic auth with an account that can only read the Jira projects and issues Workflow OS must inspect.

Workflow OS also supports bearer auth for Jira deployments that explicitly accept bearer tokens. Do not use bearer tokens for Atlassian Cloud API-token auth unless your Jira deployment documents that mode.

Recommended read-only permissions:

- issue metadata: read-only
- issue descriptions: read-only
- issue comments: read-only
- project metadata: read-only

Do not grant write permissions for Phase 2.

## Forbidden Permissions For Phase 2

Do not request or use permissions that allow:

- issue creation
- issue updates
- comment creation
- status transitions
- assignment changes
- label changes
- link creation
- administration

## Storage Rules

Tokens must never be stored in:

- `workflow-os.yml`
- workflow specs
- skill specs
- policy specs
- test specs
- audit records
- observability records
- diagnostics
- logs

Use `WORKFLOW_OS_JIRA_EMAIL` plus `WORKFLOW_OS_JIRA_API_TOKEN` for Atlassian Cloud local live read-only testing. `JIRA_EMAIL` plus `JIRA_API_TOKEN` are recognized as fallback names. Use `WORKFLOW_OS_JIRA_BEARER_TOKEN` only for Jira deployments that explicitly support bearer auth.

## Redaction Rules

Health checks may report whether a token is present. They must not expose:

- token value
- token prefix
- email address
- authorization header
- private key material
- secret provider payloads

Issue descriptions and comments should be treated as sensitive. Adapter summaries should store references and counts, not raw bodies.

If any token appears in logs, audit, observability, health output, or diagnostics, treat it as a security bug.
