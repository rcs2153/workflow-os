# GitHub PR Comment Provider Client and Auth Loading Plan

Status: Accepted plan; concrete injected-transport provider client implemented in [GitHub PR Comment Provider Client/Auth Loading Implementation Report](../concepts/GITHUB_PR_COMMENT_PROVIDER_CLIENT_AUTH_LOADING_IMPLEMENTATION_REPORT.md) and accepted with non-blocking follow-ups in [GitHub PR Comment Provider Client/Auth Loading Implementation Review](../concepts/GITHUB_PR_COMMENT_PROVIDER_CLIENT_AUTH_LOADING_IMPLEMENTATION_REVIEW.md). Provider write reconciliation planning is documented in [GitHub PR Comment Provider Write Reconciliation Plan](github-pr-comment-provider-write-reconciliation-plan.md). This follows the accepted [GitHub PR Comment Provider-Call Orchestration Helper Review](../concepts/GITHUB_PR_COMMENT_PROVIDER_CALL_ORCHESTRATION_HELPER_REVIEW.md). It defines how Workflow OS adds a concrete GitHub pull request comment provider client while keeping auth loading explicit and deferred.

The first implementation adds a concrete provider client with injected transport only. It does not implement hidden auth loading, automatic live writes, executor integration, workflow event append, report artifact writes, CLI mutation behavior, schemas, examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes.

## 1. Executive Summary

Workflow OS now has:

- no-provider GitHub PR comment write orchestration;
- store-backed proposed/attempted/completed/failed side-effect lifecycle helpers;
- provider-call trait/input/request models;
- an injected-provider orchestration helper that maps classified provider success/failure responses to side-effect lifecycle transitions.

The concrete provider client is now introduced without turning writes on by default or hiding credential discovery inside the runtime.

The first concrete client remains local, explicit, opt-in, and testable. It implements the existing `GitHubPullRequestCommentProvider` trait and accepts explicit caller-supplied auth material plus an injected transport. It does not read environment variables, keychains, GitHub CLI state, git remotes, configuration files, or hidden global state.

## 2. Goals

- Define the smallest concrete GitHub PR comment provider client boundary.
- Preserve explicit opt-in behavior.
- Preserve provider writes denied by default outside this explicit helper path.
- Keep auth loading outside hidden runtime behavior.
- Classify GitHub provider responses into bounded success/failure outcomes.
- Return stable provider references without copying raw provider payloads.
- Keep workflow event append and report artifact writes as separate reviewed boundaries.
- Preserve side-effect idempotency expectations and document duplicate-call limitations.
- Keep errors stable and non-leaking.
- Prepare a small implementation prompt.

## 3. Non-Goals

Do not implement or authorize:

- implementation in this planning phase;
- automatic provider writes;
- default executor write behavior;
- executor integration;
- approval-resume write behavior;
- cancellation write behavior;
- automatic workflow event append;
- automatic audit or observability emission;
- automatic report artifact writing;
- CLI mutation commands or flags;
- workflow schema fields;
- examples;
- hosted or distributed runtime behavior;
- auth discovery from environment variables, keychains, GitHub CLI state, git remotes, config files, or hidden global state;
- OAuth app behavior;
- webhook ingestion;
- provider retries beyond a separately reviewed policy;
- broad GitHub write support;
- non-comment GitHub mutations;
- Jira or other provider writes;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- release posture changes.

## 4. Current Preconditions

A future concrete client must be called only after the existing provider-call orchestration preconditions are satisfied:

- `SideEffectRecord` exists in attempted state.
- Target matches the GitHub PR comment target.
- Idempotency key matches the attempted side-effect record.
- Request mode is `LiveSandbox`.
- Live call is explicitly enabled.
- Provider call is explicitly enabled.
- Auth wrapper is present and validated.
- Policy and approval posture were already represented before attempted state.
- Comment body, summary, sensitivity, and redaction metadata validate.

The concrete client must not weaken those preconditions.

## 5. Provider Client Boundary

The first concrete client should implement `GitHubPullRequestCommentProvider`.

Recommended shape:

- `GitHubPullRequestCommentHttpProvider`
- constructor accepts:
  - explicit API base URL, defaulting to `https://api.github.com` only when supplied through a safe constructor;
  - explicit validated auth wrapper;
  - optional injected HTTP transport for tests;
  - bounded timeout value;
  - redaction metadata;
  - sensitivity.

The provider should:

- construct a single GitHub create-comment request for the validated target;
- include only the required GitHub API fields;
- not log or return raw request/response bodies;
- classify HTTP status codes into stable provider error codes;
- return `ProviderSucceeded` with a bounded provider comment reference when GitHub returns success;
- return `ProviderFailed` with a bounded provider error code when GitHub returns a classified failure;
- return an error only for unclassified local/transport failures that cannot be converted into a provider response.

## 6. Auth Loading Boundary

Auth must remain explicit.

Allowed first implementation:

- caller passes `GitHubPullRequestCommentProviderAuth` directly to the concrete provider constructor;
- the auth wrapper remains non-serializable and redaction-safe;
- tests use explicit fake auth values that are never logged or serialized.

Deferred:

- environment variable loading;
- GitHub CLI token discovery;
- keychain discovery;
- config-file loading;
- OAuth app installation tokens;
- secret manager integration;
- hosted credential brokering.

If a later phase adds auth loading, it must be a separate explicit helper with its own reviewed plan and tests. It must not happen inside executor default behavior.

## 7. Request Execution Policy

The provider client may perform one provider call per validated request.

It must not:

- retry automatically unless a reviewed retry policy exists;
- create multiple comments for one request;
- mutate workflow state;
- write side-effect records;
- append workflow events;
- emit audit events directly;
- write report artifacts;
- print CLI output;
- read repository files;
- infer target identity from git remotes or branch state.

The injected orchestration helper remains responsible for invoking the provider and transitioning the local `SideEffectRecord` from the classified provider response.

## 8. Idempotency and Reconciliation

GitHub issue/comment creation does not provide a simple generic idempotency key that Workflow OS can rely on for all cases.

Conservative v1 posture:

- local idempotency gates prevent duplicate calls through repeated Workflow OS execution paths;
- provider-native idempotency remains not implemented unless explicitly proven;
- if the provider succeeds but the local lifecycle transition fails, the caller must reconcile manually from the provider reference or run logs;
- the concrete client must not silently retry after an ambiguous response;
- ambiguous transport failures should return a stable unclassified error rather than fabricating success/failure.

Before any executor-integrated write path, Workflow OS should add an explicit reconciliation plan for remote-success/local-transition-failure cases.

## 9. Provider Response Classification

Recommended error code vocabulary:

- `github.auth_failed`
- `github.forbidden`
- `github.not_found`
- `github.rate_limited`
- `github.validation_failed`
- `github.conflict`
- `github.server_error`
- `github.timeout`
- `github.transport_unclassified`

Rules:

- codes must be bounded and stable;
- no raw provider messages in error codes;
- bounded summaries may state the class of failure without copying provider payloads;
- raw GitHub response bodies must not be stored;
- provider comment reference must be stable and bounded, such as `github/pr-comment/{owner}/{repo}/{pull_number}/{comment_id}` or another reviewed reference shape.

## 10. Privacy and Redaction

The concrete client must not store or output:

- tokens;
- authorization headers;
- raw request bodies;
- raw response bodies;
- raw provider payloads;
- GitHub file contents;
- environment variable values;
- credentials;
- private keys;
- command output;
- CI logs;
- parser payloads;
- secret-like values.

Debug output must redact:

- auth;
- comment body;
- provider reference if it contains repository identity;
- target identity unless explicitly represented through safe accessors;
- redaction metadata values.

## 11. Runtime and Executor Boundary

The concrete client should not be wired into `LocalExecutor` in the first implementation.

First implementation should be:

- model/core only;
- explicit provider object;
- direct unit tests;
- optional injected fake HTTP transport tests;
- no executor path;
- no CLI command.

Executor integration should remain a later reviewed phase after the concrete client is reviewed.

## 12. Test Plan

Future implementation tests should cover:

- concrete provider implements `GitHubPullRequestCommentProvider`;
- success response maps to `ProviderSucceeded`;
- success response returns only a bounded provider reference;
- auth failure maps to classified provider failure;
- forbidden maps to classified provider failure;
- not found maps to classified provider failure;
- rate limit maps to classified provider failure;
- validation failure maps to classified provider failure;
- timeout/transport ambiguity returns stable non-leaking error or classified failure according to the chosen policy;
- no raw provider request or response body is stored;
- Debug output does not leak auth, comment body, target identity, response body, or token-like values;
- no workflow events are appended;
- no side-effect records are written by the provider client itself;
- no report artifacts are written;
- no CLI output is emitted;
- no environment, keychain, GitHub CLI, git remote, or config lookup occurs;
- existing provider-call orchestration tests still pass;
- workspace tests pass.

## 13. Proposed Implementation Sequence

1. Add a concrete provider client type with injected transport, no executor wiring.
2. Add explicit request construction and response classification tests using fake transport.
3. Add redaction and non-leakage tests.
4. Add no-hidden-auth-discovery tests.
5. Review the concrete provider client.
6. Review [GitHub PR Comment Provider Write Reconciliation Plan](github-pr-comment-provider-write-reconciliation-plan.md) for remote-success/local-transition-failure handling.
7. Only after review, plan executor integration for explicit live sandbox calls.

## 14. Open Questions

- What exact provider comment reference shape should be persisted for GitHub PR comments?
- Should ambiguous transport failures be represented as `ProviderFailed` with `github.transport_unclassified`, or as a returned `WorkflowOsError` with no lifecycle transition?
- Should the first client use an internal minimal HTTP abstraction or a dependency already present in the workspace?
- Should provider response headers such as request ID or rate-limit metadata be cited later as evidence references?
- What minimum live sandbox smoke test is acceptable without making CI depend on live credentials?
- What reconciliation UX should exist when remote success occurs but local lifecycle transition fails?

## 15. Final Recommendation

Proceed next to concrete GitHub PR comment provider client implementation, explicit injected transport only.

Do not add executor integration, CLI behavior, auth discovery, schemas, examples, hosted behavior, report artifact writes, workflow event append, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes.
