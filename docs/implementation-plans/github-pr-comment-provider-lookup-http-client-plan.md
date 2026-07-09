# GitHub PR Comment Provider Lookup HTTP Client Plan

Status: Planned. This follows the accepted [GitHub PR Comment Provider Lookup Reconciliation Model Review](../concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_RECONCILIATION_MODEL_REVIEW.md).

This plan defines the next narrow phase: a concrete injected-transport GitHub PR comment lookup HTTP client that can return the existing bounded lookup response model. It does not implement the client.

## 1. Executive Summary

Workflow OS now has a reviewed lookup reconciliation model/helper for GitHub PR comments.

The next question is how a future implementation should create a concrete HTTP-backed lookup client without changing runtime behavior. The client should implement `GitHubPullRequestCommentProviderLookupClient`, use caller-supplied auth, use injected transport, and return bounded observations that the existing reconciliation helper can classify.

This plan does not implement automatic lookup, hidden auth loading, provider writes, retries, event append, state repair, report artifact writes, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, approval-presentation enforcement, or release posture changes.

## 2. Goals

- Add a concrete, explicit GitHub PR comment lookup HTTP client in a future implementation.
- Reuse the injected-transport pattern already used by `GitHubPullRequestCommentHttpProvider`.
- Preserve caller-supplied auth only.
- Preserve lookup as bounded observation, not durable event proof.
- Return only bounded provider-side observations.
- Classify HTTP status and transport outcomes into stable lookup outcomes.
- Avoid raw provider payload, comment body, PR body, diff, review-thread, source-file, command-output, CI-log, token, and credential storage.
- Keep errors stable and non-leaking.
- Keep all runtime, artifact, event, and repair behavior separately scoped.

## 3. Non-Goals

Do not implement or authorize:

- implementation in this planning phase;
- automatic provider lookup;
- hidden auth loading;
- environment, keychain, GitHub CLI, git remote, config-file, OAuth app, or hosted credential discovery;
- GitHub comment creation, update, deletion, reaction, or edit behavior;
- automatic retries;
- provider writes;
- workflow event append;
- workflow event repair;
- side-effect record mutation;
- manual state repair;
- report artifact writes;
- automatic report generation;
- CLI recovery or lookup commands;
- workflow schema changes;
- examples;
- hosted or distributed runtime behavior;
- broad write-capable adapter expansion;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- approval-presentation enforcement;
- release posture changes.

## 4. Current Baseline

Implemented and reviewed:

- concrete injected-transport GitHub PR comment provider write client;
- explicit caller-supplied provider auth wrapper;
- provider-call request/response model;
- provider-write reconciliation helper;
- executor-integrated live provider-write helper;
- provider write event append helper;
- provider reconciliation disclosure helper;
- strict report artifact event-proof gates;
- provider event-proof recovery classifier;
- provider lookup reconciliation model/helper.

Still missing:

- concrete lookup HTTP transport/request/response model;
- concrete lookup HTTP client implementing `GitHubPullRequestCommentProviderLookupClient`;
- opt-in live lookup smoke test;
- executor/runtime lookup integration;
- CLI lookup/recovery command;
- manual repair helper.

## 5. Client Boundary

The future client should be explicit and injectable.

Recommended shape:

- `GitHubPullRequestCommentLookupHttpProvider<T>`;
- `GitHubPullRequestCommentLookupHttpTransport`;
- `GitHubPullRequestCommentLookupHttpRequest`;
- `GitHubPullRequestCommentLookupHttpResponse`.

The provider should implement:

```text
GitHubPullRequestCommentProviderLookupClient
```

The constructor should accept:

- injected transport;
- explicit API base URL;
- explicit `GitHubPullRequestCommentProviderAuth`;
- sensitivity;
- redaction metadata.

The client must not discover credentials, inspect git remotes, read repo files, create runtime state, append events, mutate side-effect records, write artifacts, or emit CLI output.

## 6. Lookup Request Construction

The concrete client should construct a single bounded GitHub API request from `GitHubPullRequestCommentProviderLookupRequest`.

Recommended endpoint:

```text
GET /repos/{owner}/{repo}/issues/{pull_number}/comments
```

The request should include:

- method;
- fully constructed URL;
- authorization header for transport only;
- optional bounded pagination controls if implemented;
- no request body;
- no serialized auth.

The request debug output must redact:

- URL;
- authorization header;
- side-effect ID;
- idempotency key;
- expected provider reference;
- expected managed marker;
- redaction metadata.

## 7. Provider Response Parsing

The injected transport should return a bounded parsed HTTP response, not a raw GitHub response body.

Allowed parsed data:

- HTTP status;
- a bounded list of observed comment IDs;
- optional bounded Workflow OS managed markers if the parser can extract them safely;
- stable provider error code.

Disallowed data:

- raw comment bodies;
- PR bodies;
- diffs;
- review comments;
- file contents;
- raw GitHub JSON;
- response headers unless separately planned as bounded metadata;
- rate-limit header values unless separately planned;
- raw error messages.

The parser/transport boundary may inspect provider JSON to extract only bounded comment IDs and exact managed marker values, then discard the raw payload.

## 8. Managed Marker Handling

The lookup client may support marker extraction only for Workflow OS managed markers that were intentionally embedded by an approved provider-write path.

Marker rules:

- bounded by the existing marker limit;
- ASCII-safe according to the lookup model;
- rejected if secret-like;
- exact-match only;
- no fuzzy matching;
- no natural-language interpretation;
- no raw comment body storage.

If marker extraction cannot be performed safely, the client should return observations with provider comment references only or return `ResponseUntrusted` when parsing shape is unsafe.

## 9. HTTP Status Classification

The future client should map HTTP/provider conditions to existing lookup outcomes:

| Condition | Lookup outcome | Provider error code |
| --- | --- | --- |
| `2xx` with bounded parsed response | `Completed` | optional none |
| `401` or bad auth | `NotAuthorized` | `github.auth_failed` |
| `403` forbidden | `NotAuthorized` | `github.forbidden` |
| `404` target not found | `Completed` with no observations or `Unavailable` depending on final policy | `github.not_found` if non-completed |
| `429` or rate-limit condition | `RateLimited` | `github.rate_limited` |
| `5xx` | `Unavailable` | `github.server_error` |
| malformed safe parser result | `ResponseUntrusted` | `github.response_untrusted` |
| transport timeout | `Unavailable` | `github.timeout` |
| unclassified transport failure | returned stable `WorkflowOsError` before bounded response | `github.transport_unclassified` if represented |

The implementation must choose and document whether `404` means no comments were observed for an otherwise valid target or lookup unavailable due to target ambiguity. The first implementation should bias conservative if target existence cannot be proven from the comments endpoint alone.

## 10. Pagination Policy

The first implementation should avoid broad pagination unless it is explicitly bounded.

Acceptable v1 options:

- request a single bounded page size;
- return `RemoteCommentAmbiguous` or `LookupUnavailable` posture through the existing helper if the response indicates more pages are required;
- support a small bounded page limit if implemented with deterministic request count and no automatic retry.

The client must not perform unbounded pagination, background crawling, search queries, or broad repository scans.

## 11. Auth Boundary

Auth remains explicit.

Allowed:

- caller passes `GitHubPullRequestCommentProviderAuth` directly to the lookup HTTP provider constructor;
- request validates that client auth matches the caller-supplied lookup request auth;
- auth is redaction-safe in `Debug`;
- tests use fake auth values and injected transports.

Deferred:

- environment variable loading;
- keychain lookup;
- GitHub CLI token discovery;
- app installation tokens;
- OAuth flows;
- secret manager integration;
- hosted credential broker.

## 12. Relationship To Lookup Reconciliation Helper

The concrete client should produce `GitHubPullRequestCommentProviderLookupResponse`.

The existing helper remains responsible for:

- validating attempted record and target match;
- building the validated lookup request;
- invoking the supplied lookup client;
- classifying observations into lookup reconciliation posture;
- preserving artifact-write blocking;
- preserving event-proof separation.

The concrete client must not duplicate reconciliation policy or decide retry/artifact semantics.

## 13. Workflow Semantics

The concrete lookup client must not change workflow pass/fail status.

It must not:

- mutate `WorkflowRun`;
- append workflow events;
- emit audit events;
- emit observability events;
- mutate side-effect records;
- write report artifacts;
- retry provider writes;
- create filesystem artifacts;
- expose CLI output.

Any future executor integration must be separately planned and reviewed.

## 14. Error Handling

Errors must use stable non-leaking codes.

Candidate codes:

- `github_pr_comment_lookup_http.base_url.invalid`;
- `github_pr_comment_lookup_http.auth.mismatch`;
- `github_pr_comment_lookup_http.transport_unclassified`;
- `github_pr_comment_lookup_http.response.invalid`;
- `github_pr_comment_lookup_http.response_untrusted`;
- `github_pr_comment_lookup_http.pagination.unsupported`;

Errors must not include:

- raw URLs containing private repository identity;
- authorization headers;
- tokens;
- provider JSON;
- comment bodies;
- PR bodies;
- diffs;
- review thread text;
- CI logs;
- command output;
- parser payloads;
- raw specs;
- secret-like IDs or metadata.

## 15. Privacy And Redaction

The client and transport model must be redaction-safe.

Required:

- no serialization for request objects that carry auth;
- debug redacts URL, auth, body/parser data, expected markers, side-effect IDs, idempotency keys, and redaction metadata;
- parsed response stores only bounded IDs/markers/outcome codes;
- no raw provider payload storage;
- no raw body persistence;
- no raw error propagation from transport;
- deserialization, if any response model is deserializable, must fail closed through constructors.

## 16. Test Plan

Future implementation tests should cover:

- concrete lookup provider implements `GitHubPullRequestCommentProviderLookupClient`;
- request URL and method are constructed through injected transport without leaking debug output;
- auth mismatch fails before transport invocation;
- missing/invalid auth fails before transport invocation;
- `2xx` with one parsed comment ID returns `Completed` response with bounded observation;
- `2xx` with bounded managed marker returns an observation with marker only when safe;
- `401` maps to `NotAuthorized`;
- `403` maps to `NotAuthorized`;
- `404` maps according to the selected conservative policy;
- `429` maps to `RateLimited`;
- `5xx` maps to `Unavailable`;
- malformed parser output maps to `ResponseUntrusted`;
- transport failure maps to stable non-leaking error;
- pagination beyond supported bound fails closed or returns a bounded unavailable/untrusted posture;
- debug output does not leak auth, URL, marker, provider comment ID, raw body, token-like values, or redaction metadata;
- no workflow events are appended;
- no side-effect records are written;
- no report artifacts are written;
- no CLI output is emitted;
- existing provider lookup reconciliation tests still pass;
- workspace tests pass.

## 17. Proposed Implementation Sequence

1. Add lookup HTTP request/response/transport types with redaction-safe debug behavior.
2. Add concrete lookup HTTP provider using injected transport and explicit auth.
3. Map HTTP status/parser outcomes into the existing lookup response model.
4. Add focused unit tests with fake transport.
5. Add non-leakage tests for auth, URL, markers, IDs, parser payloads, and transport errors.
6. Update docs and create an implementation report.
7. Run full validation.
8. Review before any executor, CLI, artifact, event-repair, or manual-state-repair phase.

## 18. Deferred Work

- Hidden auth loading.
- Automatic provider lookup.
- Executor integration.
- CLI lookup/recovery command.
- Manual state repair helper.
- Workflow event repair.
- Report artifact composition using lookup observation.
- Live smoke test.
- Pagination beyond the first bounded policy.
- EvidenceReference attachment for provider lookup observations.
- Hosted/runtime provider lookup services.

## 19. Final Recommendation

Proceed next to a small implementation phase: **GitHub PR comment provider lookup HTTP client, injected transport only**.

The implementation must reuse the reviewed lookup reconciliation model/helper and the existing explicit-auth/injected-transport provider pattern. It must not implement automatic lookup, hidden auth, provider writes, retries, event append, state repair, report artifact writes, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, approval-presentation enforcement, or release posture changes.

