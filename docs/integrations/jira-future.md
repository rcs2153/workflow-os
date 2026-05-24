# Future Jira Adapter

Workflow OS does not implement a Jira adapter in v0.

When introduced, a Jira adapter must remain a generic adapter implementation. It must not turn Workflow OS Core into a ticketing automation product.

## Future Requirements

A Jira adapter must:

- declare read and write capabilities explicitly
- require policy allow or approval before writes
- use idempotency keys for issue mutation or comment creation
- store issue keys, URLs, and summaries rather than raw sensitive payloads
- classify authentication, permission, rate limit, not found, validation, transient, and unknown failures
- report outcomes through core runtime interfaces

## Deferred Behavior

No Jira API calls, OAuth flows, webhook handling, issue creation, issue updates, transitions, or comments are implemented in v0.
