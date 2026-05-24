# Future GitHub Adapter

Workflow OS does not implement a GitHub adapter in v0.

When introduced, a GitHub adapter must implement the generic adapter contracts rather than special-case workflow state. It must not make Workflow OS a GitHub automation tool.

## Future Requirements

A GitHub adapter must:

- declare read and write capabilities explicitly
- require policy allow or approval before writes
- use idempotency keys for write operations
- store references to repositories, issues, pull requests, commits, or checks rather than raw sensitive payloads
- classify authentication, permission, rate limit, not found, validation, transient, and unknown failures
- report outcomes through core runtime interfaces

## Deferred Behavior

No GitHub API calls, OAuth flows, webhook handling, Checks API writes, issue writes, pull request writes, or Actions API calls are implemented in v0.
