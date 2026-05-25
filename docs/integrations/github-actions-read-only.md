# GitHub Actions Read-Only Adapter

The GitHub Actions read-only adapter is the first concrete Phase 2 CI read-only adapter on the development branch. It proves the generic CI read-only contract against GitHub Actions without introducing rerun, dispatch, cancellation, or check mutation behavior.

It is not part of the `0.1.0-preview.1` local kernel release contract, and it is not a public read-only integration preview until a follow-up maintainer review approves that posture.

## Scope

Supported read-only operations:

- read workflow run metadata
- read workflow jobs
- read check run status summaries for a commit ref
- read log download references
- read limited, redacted job log excerpts when explicitly requested
- produce normalized CI failure context summaries

## Explicit Non-Goals

The GitHub Actions read-only adapter does not support:

- rerunning workflows
- rerunning failed jobs
- canceling workflow runs
- dispatching workflows
- uploading artifacts
- deleting logs
- modifying checks
- webhook receiver
- OAuth app flow

Write-capable GitHub Actions actions fail closed in Phase 2.

## Configuration

Live read-only mode loads credentials from environment variables only:

- `WORKFLOW_OS_GITHUB_ACTIONS_TOKEN`
- fallback: `GITHUB_TOKEN`

Specs must not contain tokens.

Fixture mode requires no credentials and is the normal CI test path. Live read-only tests are opt-in and skipped by default.

## Response Handling

The adapter stores normalized summaries and external references by default.

Full logs are not stored in audit by default. Log references are preferred. Explicit log excerpts are bounded and redacted before they become adapter summaries.

## Error Classification

GitHub Actions errors are classified into the generic adapter error model:

- `401`: authentication failure
- `403`: permission failure, or rate limited when rate-limit headers indicate exhaustion
- `404`: not found
- `408` or `504`: timeout
- `429`: rate limited
- `400..499`: validation failure, except the cases above
- `5xx`: transient network failure

Malformed JSON responses are classified as malformed responses.

## Audit And Observability

Each successful read can produce:

- normalized adapter response
- audit-safe adapter invocation record
- adapter observability record

These records include adapter ID, adapter kind, operation mode, action, capability, actor, correlation ID, latency, status, redaction metadata, and run identity when run-scoped.

## Live Testing

Live tests are ignored by default. To run live GitHub Actions reads, set:

```sh
WORKFLOW_OS_LIVE_GITHUB_ACTIONS_TESTS=1
WORKFLOW_OS_GITHUB_ACTIONS_TOKEN=<read-only token>
WORKFLOW_OS_GITHUB_ACTIONS_TEST_OWNER=acme
WORKFLOW_OS_GITHUB_ACTIONS_TEST_REPO=widgets
WORKFLOW_OS_GITHUB_ACTIONS_TEST_RUN_ID=12345
```

CI must not require live credentials.
