# Future CI Adapter

Workflow OS does not implement a CI adapter in v0.

When introduced, CI integrations must remain adapters around the core kernel. Workflow OS Core must not become a replacement for GitHub Actions, Buildkite, Jenkins, CircleCI, or other CI systems.

## Future Requirements

A CI adapter must:

- declare read and write capabilities explicitly
- require policy allow or approval before triggering jobs, canceling jobs, or writing statuses
- use idempotency keys for side-effecting CI actions
- store run IDs, job IDs, URLs, and summaries rather than raw logs by default
- classify authentication, permission, rate limit, not found, validation, transient, and unknown failures
- report outcomes through core runtime interfaces

## Deferred Behavior

No CI API calls, job dispatch, status writes, log ingestion, webhook handling, or external queue integration are implemented in v0.
