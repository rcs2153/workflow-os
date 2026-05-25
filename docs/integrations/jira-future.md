# Future Jira Adapter Work

Workflow OS implements a Jira read-only adapter in Phase 2. Write-capable Jira behavior remains future work.

Any future Jira expansion must remain a generic adapter implementation. It must not turn Workflow OS Core into a ticketing automation product.

## Future Requirements

A future write-capable Jira adapter must:

- declare read and write capabilities explicitly
- require policy allow or approval before writes
- use idempotency keys for issue mutation or comment creation
- store issue keys, URLs, and summaries rather than raw sensitive payloads
- classify authentication, permission, rate limit, not found, validation, transient, and unknown failures
- report outcomes through core runtime interfaces

## Deferred Behavior

Jira issue creation, issue updates, comments, status transitions, assignment changes, label changes, OAuth flows, webhook handling, and event ingestion are not implemented.
