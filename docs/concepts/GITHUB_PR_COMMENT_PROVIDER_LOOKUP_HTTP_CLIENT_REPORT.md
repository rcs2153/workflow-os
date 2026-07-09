# GitHub PR Comment Provider Lookup HTTP Client Report

## 1. Executive Summary

The GitHub PR comment provider lookup HTTP client phase implemented the first concrete, explicit, injected-transport lookup client for provider-side PR comment observations.

The implementation remains local model/helper infrastructure. It does not make provider lookup automatic, does not load credentials from hidden state, does not write provider data, does not append workflow events, does not repair state, and does not write report artifacts.

## 2. Scope Completed

- Added `GitHubPullRequestCommentLookupHttpClient`.
- Added `GitHubPullRequestCommentLookupHttpRequest`.
- Added `GitHubPullRequestCommentLookupHttpResponse`.
- Added `GitHubPullRequestCommentLookupHttpTransport`.
- Implemented `GitHubPullRequestCommentProviderLookupClient` for the concrete lookup HTTP client.
- Reused caller-supplied `GitHubPullRequestCommentProviderAuth`.
- Reused the existing bounded lookup response and reconciliation model.
- Added status classification for lookup HTTP responses.
- Exported the new lookup HTTP types from `workflow-core`.
- Added focused fake-transport tests.

## 3. Scope Explicitly Not Completed

- No automatic provider lookup.
- No hidden auth loading.
- No GitHub provider writes.
- No retries.
- No workflow event append.
- No state repair.
- No report artifact writes.
- No CLI lookup or recovery command.
- No workflow schema changes.
- No examples.
- No hosted or distributed behavior.
- No reasoning lineage.
- No approval-presentation enforcement.
- No release posture changes.

## 4. API Summary

`GitHubPullRequestCommentLookupHttpClient<T>` accepts:

- injected transport;
- explicit API base URL;
- caller-supplied provider auth;
- sensitivity;
- redaction metadata.

It implements `GitHubPullRequestCommentProviderLookupClient` and accepts already-validated `GitHubPullRequestCommentProviderLookupRequest` values. The transport receives a redaction-safe lookup HTTP request with:

- method;
- URL;
- authorization header for transport only;
- idempotency key;
- optional expected provider reference;
- optional expected managed marker.

The transport returns a bounded lookup HTTP response containing only:

- HTTP status;
- bounded provider-side lookup observations.

## 5. HTTP Status Behavior

The first implementation maps status codes conservatively:

- `2xx`: `Completed`;
- `401`: `NotAuthorized` with `github.auth_failed`;
- `403`: `NotAuthorized` with `github.forbidden`;
- `404`: `Unavailable` with `github.not_found`;
- `408`: `Unavailable` with `github.timeout`;
- `429`: `RateLimited` with `github.rate_limited`;
- `5xx`: `Unavailable` with `github.server_error`;
- other status codes: `ResponseUntrusted` with `github.response_untrusted`.

`404` is treated as unavailable rather than remote absence because the comments endpoint alone does not prove that the target exists with no matching comments.

## 6. Citation And Reconciliation Summary

The lookup HTTP client does not create `EvidenceReference` values and does not write report citations.

It returns bounded lookup observations that the existing lookup reconciliation helper can classify as:

- remote comment observed;
- remote comment absent;
- remote comment ambiguous;
- lookup not authorized;
- lookup unavailable;
- lookup rate-limited;
- lookup response untrusted.

Provider lookup remains observation only. Durable workflow event proof remains distinct and is still required by strict report artifact gates.

## 7. Redaction And Privacy Summary

The implementation:

- does not store raw GitHub JSON;
- does not store raw comment bodies;
- does not store PR bodies, diffs, review threads, source files, CI logs, command output, or provider payloads;
- does not serialize auth;
- redacts URL, auth, idempotency key, provider references, managed markers, transport, and redaction metadata from Debug output;
- maps unclassified transport failures to stable non-leaking errors;
- uses existing bounded validation for provider references, managed markers, redaction metadata, and observations.

## 8. Test Coverage Summary

Added focused tests for:

- successful lookup request construction and bounded observation mapping;
- reuse of the existing lookup reconciliation helper;
- status classification for auth failure, forbidden, not found, timeout, rate limit, server error, and untrusted response;
- transport failure non-leakage;
- auth mismatch rejection before transport invocation;
- lookup client/request Debug non-leakage;
- no state, workflow event, artifact, or CLI side effects.

Existing provider write and lookup reconciliation tests continue to pass.

## 9. Commands Run And Results

- `cargo test -p workflow-core --test provider_write` - passed.
- `cargo fmt --all --check` - passed after formatting.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed after the final report update.

## 10. Remaining Known Limitations

- No live HTTP transport implementation exists in core.
- No automatic lookup path exists.
- No executor integration exists.
- No CLI lookup/recovery command exists.
- No hidden auth loading exists.
- No provider lookup event append exists.
- No manual repair helper exists.
- No report artifact write can be justified by provider lookup alone when durable event proof is missing.
- Pagination remains bounded to a single request URL with `per_page=100`; no unbounded pagination is implemented.

## 11. Recommended Next Phase

Recommended next phase: **GitHub PR comment provider lookup HTTP client review**.

This phase is close to provider-observation and write-readiness surfaces, so it should receive a maintainer review before any executor integration, CLI recovery, manual repair, or artifact-composition phase.
