# GitHub Read-Only Adapter

The GitHub read-only adapter is a Phase 2 read-only integration preview. It proves the generic adapter contract against GitHub without introducing write behavior.

It is part of the `0.2.0-preview.1` public read-only integration preview. It is not production GitHub automation and does not support GitHub writes.

## Scope

Supported read-only operations:

- read repository metadata
- read repository default branch
- read file contents metadata and references by path/ref
- read pull request metadata
- read pull request diff summary
- read pull request changed files
- read pull request comments as read-only data
- read check run status summaries

The adapter normalizes GitHub responses into `AdapterResponse`, `AdapterInvocationRecord`, and `AdapterObservabilityRecord` values.

See [the GitHub read-only review context example](../../examples/github-read-only-review-context/README.md) for a fixture-backed workflow that reads pull request context without writing to GitHub.

## Explicit Non-Goals

The GitHub read-only adapter does not support:

- creating branches
- committing files
- opening pull requests
- posting comments
- requesting reviews
- changing labels
- merging
- closing pull requests
- rerunning checks
- workflow dispatch
- webhook receiver
- OAuth app flow

Write-capable GitHub actions fail closed in Phase 2.

## Configuration

Live read-only mode loads credentials from environment variables only:

- `WORKFLOW_OS_GITHUB_TOKEN`
- fallback: `GITHUB_TOKEN`

Specs must not contain tokens.

Fixture mode requires no credentials and is the normal CI test path. Live read-only tests are opt-in and skipped by default.

## Operation Modes

- `fixture`: static offline GitHub responses for tests.
- `mock`: deterministic test doubles.
- `live-read-only`: real GitHub REST API reads.
- `live-write-capable`: represented by the generic adapter contract but denied in Phase 2.

## Response Handling

The adapter stores normalized summaries and external references by default.

File contents and pull request comment bodies are not stored raw in adapter summaries. They are represented as reference-only data. Private repository data should be treated as sensitive even when the adapter can read it.

## Error Classification

GitHub errors are classified into the generic adapter error model:

- `401`: authentication failure
- `403`: permission failure, or rate limited when rate-limit headers indicate exhaustion
- `404`: not found
- `408` or `504`: timeout
- `422`: validation failure
- `429`: rate limited
- `5xx`: transient network failure

Malformed JSON responses are classified as malformed responses.

## Audit And Observability

Each successful read can produce:

- normalized adapter response
- audit-safe adapter invocation record
- adapter observability record

These records include adapter ID, adapter kind, operation mode, action, capability, actor, correlation ID, latency, status, redaction metadata, and run identity when run-scoped.

## Live Testing

Live tests are ignored by default. To run live GitHub reads, set:

```sh
WORKFLOW_OS_LIVE_GITHUB_TESTS=1
WORKFLOW_OS_GITHUB_TOKEN=<read-only token>
```

CI must not require live credentials.

Recorded `0.2.0-preview.1` live smoke evidence exercised repository metadata for `octocat/Hello-World`. Broader GitHub operation coverage is fixture-tested and should not be described as live-proven in this preview.
