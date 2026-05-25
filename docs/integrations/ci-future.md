# Future CI Adapter Work

Workflow OS implements a GitHub Actions read-only adapter in Phase 2. Write-capable CI behavior remains future work.

Any future CI expansion must remain an adapter around the core kernel. Workflow OS Core must not become a replacement for GitHub Actions, Buildkite, Jenkins, CircleCI, or other CI systems.

## Future Requirements

A future write-capable CI adapter must:

- declare read and write capabilities explicitly
- require policy allow or approval before triggering jobs, canceling jobs, or writing statuses
- use idempotency keys for side-effecting CI actions
- store run IDs, job IDs, URLs, and summaries rather than raw logs by default
- classify authentication, permission, rate limit, not found, validation, transient, and unknown failures
- report outcomes through core runtime interfaces

## Deferred Behavior

CI reruns, workflow dispatch, cancellation, status writes, artifact writes, webhook handling, and external queue integration are not implemented. The GitHub Actions read-only adapter can read run metadata, job summaries, check summaries, log references, and bounded redacted excerpts.
