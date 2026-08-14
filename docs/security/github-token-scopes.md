# GitHub Token Scopes

GitHub access must use least-privilege credentials. The ordinary adapter and
default runtime posture remain read-only. One plan-reviewed, explicit,
ignored sandbox smoke may use pull-request write permission to prove the
managed draft-pull-request transport; the concrete implementation is accepted
only for that compile-time allowlisted sandbox. That exception does not make GitHub
writes a default adapter capability.

## Recommended Token Posture

For public repositories, prefer unauthenticated fixture tests or a token with no write permissions.

For private repositories, use a fine-grained GitHub token limited to the specific repositories that Workflow OS must read.

Recommended read-only permissions:

- repository metadata: read-only
- contents: read-only
- pull requests: read-only
- issues: read-only, only if pull request issue comments must be read
- checks: read-only

Do not grant write permissions to Phase 2 read-only adapter tokens.

## Explicit Draft Pull Request Sandbox Token

The ignored draft-pull-request HTTP smoke requires a distinct fine-grained
token limited to the exact allowlisted sandbox repository
`rcs2153/workflow-os-sandbox`. Grant only:

- repository metadata: read-only
- contents: read-only, for exact branch-ref observation
- pull requests: read and write, for lookup and one managed draft creation

Do not reuse this token for ordinary read-only adapters. The smoke reads it
only from `WORKFLOW_OS_GITHUB_DRAFT_PR_SANDBOX_TOKEN` at the test-harness
boundary and immediately wraps it in the existing non-serializable auth type.
Core does not discover environment variables or other credential stores.

The smoke is ignored by default. It additionally requires the exact opt-in
flag `WORKFLOW_OS_GITHUB_DRAFT_PR_SANDBOX_SMOKE=1`, pre-existing base and head
branches, and their full observed SHAs. Repository owner and name are compiled
into the ignored test and cannot be selected by environment input.

## Forbidden Write Permissions

Do not request or use scopes that allow:

- repository contents write
- pull request write, except on the distinct allowlisted sandbox token above
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

Use `WORKFLOW_OS_GITHUB_TOKEN` or `GITHUB_TOKEN` only for local live read-only
testing. Never place the sandbox write token in those generic variables.

## Redaction Rules

Health checks may report whether a token is present. They must not expose:

- token value
- token prefix
- authorization header
- private key material
- secret provider payloads

If any token appears in logs, audit, observability, health output, or diagnostics, treat it as a security bug.

The concrete draft-pull-request provider does not log request URLs, request or
response bodies, headers, branch names, SHAs, repository identity, managed
markers, or provider messages. Any create transport uncertainty after the HTTP
call boundary is classified as may-have-started ambiguity and is never retried
automatically.
