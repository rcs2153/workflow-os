# GitHub Token Scopes

The GitHub read-only adapter must use least-privilege credentials.

## Recommended Token Posture

For public repositories, prefer unauthenticated fixture tests or a token with no write permissions.

For private repositories, use a fine-grained GitHub token limited to the specific repositories that Workflow OS must read.

Recommended read-only permissions:

- repository metadata: read-only
- contents: read-only
- pull requests: read-only
- issues: read-only, only if pull request issue comments must be read
- checks: read-only

Do not grant write permissions for Phase 2.

## Forbidden Scopes For Phase 2

Do not request or use scopes that allow:

- repository contents write
- pull request write
- issue write
- checks write
- actions write
- workflow dispatch
- administration
- secrets write

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

Use `WORKFLOW_OS_GITHUB_TOKEN` or `GITHUB_TOKEN` for local live read-only testing.

## Redaction Rules

Health checks may report whether a token is present. They must not expose:

- token value
- token prefix
- authorization header
- private key material
- secret provider payloads

If any token appears in logs, audit, observability, health output, or diagnostics, treat it as a security bug.
