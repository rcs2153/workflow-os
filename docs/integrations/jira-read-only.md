# Jira Read-Only Adapter

The Jira read-only adapter is a Phase 2 read-only integration preview. It proves the generic adapter contract against Jira issue data without introducing write behavior.

It is part of the `0.2.0-preview.1` public read-only integration preview. It is not production Jira automation and does not support Jira writes.

## Scope

Supported read-only operations:

- read issue metadata
- read issue summary
- read issue description as reference-only data
- read issue comments as reference-only data
- read issue status
- read issue priority
- read issue labels
- read assignee and reporter display metadata where available
- read project metadata

The adapter normalizes Jira responses into `AdapterResponse`, `AdapterInvocationRecord`, and `AdapterObservabilityRecord` values.

See [the Jira read-only intake quality example](../../examples/jira-read-only-intake-quality/README.md) for a fixture-backed workflow that reads issue context without writing to Jira.

## Explicit Non-Goals

The Jira read-only adapter does not support:

- creating issues
- updating issues
- adding comments
- changing status
- assigning issues
- changing labels
- creating links
- webhook receiver
- OAuth app flow

Write-capable Jira actions fail closed in Phase 2.

## Configuration

Live read-only mode loads configuration from environment variables only:

- `WORKFLOW_OS_JIRA_BASE_URL`
- Atlassian Cloud Basic auth, preferred:
  - `WORKFLOW_OS_JIRA_EMAIL`
  - `WORKFLOW_OS_JIRA_API_TOKEN`
- Basic auth fallback variables:
  - `JIRA_EMAIL`
  - `JIRA_API_TOKEN`
- Bearer auth, only for Jira deployments that explicitly support bearer tokens:
  - `WORKFLOW_OS_JIRA_BEARER_TOKEN`
  - legacy fallback: `WORKFLOW_OS_JIRA_TOKEN`

Specs must not contain tokens or Jira credentials.

Fixture mode requires no credentials and is the normal CI test path. Live read-only tests are opt-in and skipped by default.

When complete Basic auth and bearer auth are both configured, Basic auth takes precedence. A partial Basic auth configuration, such as email without API token, is treated as incomplete and reported in health warnings.

## Operation Modes

- `fixture`: static offline Jira responses for tests.
- `mock`: deterministic test doubles.
- `live-read-only`: real Jira REST API reads.
- `live-write-capable`: represented by the generic adapter contract but denied in Phase 2.

## Response Handling

The adapter stores normalized summaries and external references by default.

Issue descriptions and comment bodies are treated as sensitive and represented as reference-only data in adapter summaries. Private Jira issue data should be treated as sensitive even when the adapter can read it.

## Error Classification

Jira errors are classified into the generic adapter error model:

- `401`: authentication failure
- `403`: permission failure
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

Live tests are ignored by default. To run live Jira reads, set:

```sh
WORKFLOW_OS_LIVE_JIRA_TESTS=1
WORKFLOW_OS_JIRA_BASE_URL=https://example.atlassian.net
WORKFLOW_OS_JIRA_EMAIL=person@example.com
WORKFLOW_OS_JIRA_API_TOKEN=<read-only-api-token>
WORKFLOW_OS_JIRA_TEST_ISSUE_KEY=OPS-42
```

CI must not require live credentials.

Recorded `0.2.0-preview.1` live smoke evidence exercised Jira issue metadata only against a sandbox issue. Broader Jira operation coverage is fixture-tested and should not be described as live-proven in this preview. Rotate the sandbox API token used during smoke testing because a sandbox token was pasted into the local evaluation thread.
