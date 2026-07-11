# GitHub PR Comment Live Sandbox Runtime Composition Helper Report

## 1. Executive Summary

The first GitHub PR comment live sandbox runtime composition helper is
implemented.

The new helper composes the accepted live sandbox validation boundary into an
executor-adjacent runtime composition surface. It accepts explicit caller
inputs, uses an injected `GitHubPullRequestCommentProvider`, delegates the
proof/readiness/provider-call gates to the existing live sandbox validation
helper, and returns an in-memory bounded result.

This phase does not make provider writes automatic. It does not load hidden
auth, wrap `LocalExecutor`, append workflow events, write report artifacts,
expose CLI output, add schemas, update examples, broaden providers, retry,
repair, implement hosted behavior, implement reasoning lineage, or change
release posture.

## 2. Scope Completed

- Added `GitHubPrCommentLiveSandboxRuntimeCompositionRequest`.
- Added `GitHubPrCommentLiveSandboxRuntimeCompositionResult`.
- Added `GitHubPrCommentLiveSandboxRuntimeCompositionParts`.
- Added `compose_github_pr_comment_live_sandbox_runtime(...)`.
- Exported the new helper API from `workflow-core`.
- Added focused tests for successful injected-provider invocation after
  explicit sandbox gates.
- Added focused tests proving readiness failure blocks provider invocation.
- Updated roadmap and implementation-plan documentation.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- default provider writes;
- automatic executor provider writes;
- `LocalExecutor::execute(...)` behavior changes;
- local execution wrapping;
- hidden auth loading;
- auth loading from environment, keychain, GitHub CLI, git credentials,
  browser sessions, config files, OAuth state, or secret managers;
- CLI mutation commands;
- workflow event append from this helper;
- report artifact writes;
- artifact gate eligibility output;
- lookup, retry, repair, or recovery mutation;
- workflow schemas;
- SDK changes;
- examples;
- hosted or distributed runtime behavior;
- broad adapter write support;
- reasoning lineage;
- release posture changes.

## 4. Helper API Summary

The implemented request is:

```rust
GitHubPrCommentLiveSandboxRuntimeCompositionRequest {
    live_sandbox: GitHubPullRequestCommentLiveSandboxValidationInput<'a>,
}
```

The implemented helper is:

```rust
compose_github_pr_comment_live_sandbox_runtime(
    store,
    provider,
    request,
)
```

The helper requires a caller-supplied `SideEffectRecordStore`, an injected
`GitHubPullRequestCommentProvider`, and explicit live-sandbox validation input.
It returns `GitHubPrCommentLiveSandboxRuntimeCompositionResult` on success or
the existing non-leaking
`GitHubPullRequestCommentProviderCallOrchestrationError` on gate/provider-call
failure.

## 5. Gate And Failure Summary

The helper delegates gate behavior to
`validate_and_orchestrate_github_pr_comment_live_sandbox(...)`.

Provider invocation remains blocked unless:

- sandbox target proof validates;
- readiness is `AllowedForSandbox`;
- caller-supplied auth posture is explicit;
- attempted `SideEffect` lifecycle state is valid;
- provider-call input validates;
- idempotency binding matches the attempted record.

Readiness failure returns a stable non-leaking error and does not invoke the
provider.

## 6. Runtime Boundary Summary

The helper is runtime composition only. It does not execute workflows, mutate a
`WorkflowRun`, append workflow events, write report artifacts, emit CLI output,
read hidden state, or call external systems except through the injected provider
given by the caller.

The result exposes:

- sandbox readiness decision;
- provider-call orchestration result;
- provider-call-performed posture;
- workflow-event-appended posture, fixed to false;
- report-artifact-written posture, fixed to false.

## 7. Redaction And Privacy Summary

The helper does not store or copy provider auth tokens, authorization headers,
raw provider payloads, raw pull request bodies, raw issue/review comments,
repository file contents, CI logs, command output, raw spec contents,
environment variables, browser/session state, approval-presentation payload
text, or secret-like values.

Debug output for request/result types is bounded and does not expose token-like
test values, comment body text, provider comment references, or side-effect
record identifiers.

## 8. Test Coverage Summary

Focused tests were added for:

- successful live sandbox runtime composition invoking the injected provider
  exactly once after explicit gates pass;
- completed lifecycle transition from a successful injected provider response;
- readiness failure blocking provider invocation;
- attempted lifecycle state remaining unchanged when readiness blocks;
- no workflow event append;
- no report artifact write;
- Debug non-leakage for request/result/error paths.

Existing provider-write and live-sandbox validation tests remain the deeper
coverage for target proof, auth posture, provider-call validation, classified
provider failure, and idempotency behavior.

## 9. Commands Run And Results

Dogfood implementation run:

- workflow: `dg/implement`
- run: `run-1783779934862744000-2`
- approval: `approval/run-1783779934862744000-2/implementation-approved`
- approval-presentation proof: `presentation/6163657427eb8d1b`
- outcome: approved and completed

Validation commands:

```sh
cargo fmt --all
cargo test -p workflow-core --test local_executor live_sandbox_runtime_composition
```

Result: passed.

Full workspace validation is expected before PR merge:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run check:docs
git diff --check
```

## 10. Remaining Known Limitations

- The helper does not wrap local workflow execution.
- The helper does not append eligible completed/failed workflow events.
- The helper does not return artifact gate eligibility.
- The helper does not perform lookup, recovery, retry, or repair.
- The helper does not expose CLI behavior.
- The helper remains GitHub PR comment specific.
- Live network use remains caller/injected-provider controlled and non-default.

## 11. Recommended Next Phase

Recommended next phase: live sandbox runtime composition helper review.

The review should verify that the helper stayed explicit, local,
caller-supplied, injected-provider only, and non-default; that readiness and
idempotency gates fail closed before provider invocation; and that no workflow
events, artifacts, CLI behavior, schemas, examples, hidden auth loading,
automatic retries, broad writes, hosted behavior, reasoning lineage, or release
posture changes were introduced.
