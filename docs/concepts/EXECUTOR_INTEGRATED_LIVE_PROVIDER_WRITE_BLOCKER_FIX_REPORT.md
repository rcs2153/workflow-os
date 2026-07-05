# Executor-Integrated Live Provider Write Blocker Fix Report

## 1. Executive Summary

The executor-integrated live provider write blocker is fixed.

The implementation review found that post-provider local transition failures could be misclassified as `ProviderNotCalled`, and that reconciliation construction failures on the success path could be silently dropped. This fix preserves provider-call outcome context across the provider-call orchestration boundary and surfaces reconciliation construction failures as structured non-leaking provider-write errors.

The phase remains explicit, local, GitHub PR comment-only, injected-provider-only, and write-adjacent. It does not make provider writes automatic.

## 2. Blocker Fixed

Original blocker:

- provider-call orchestration could call the provider, receive a classified provider response, then fail during the local completed/failed SideEffect transition;
- the executor wrapper only treated `github_pr_comment_provider.call_unclassified` as provider-call-attempted;
- local transition failures after a provider response could therefore be reconciled as `ProviderNotCalled`;
- the success path used `.ok()` and could silently drop reconciliation construction failures.

Fixed behavior:

- provider-call orchestration now returns a bounded `GitHubPullRequestCommentProviderCallOrchestrationError`;
- the error can carry a classified provider response when local transition fails after provider invocation;
- the executor wrapper uses that context to build `ProviderSucceededLocalTransitionFailed` or `ProviderFailedLocalTransitionFailed` reconciliation candidates;
- those candidates block retry and require operator/future reconciliation action;
- reconciliation construction failures after normal provider response plus local transition are surfaced as `provider_write_error` instead of being silently dropped.

## 3. Implementation Approach

Added:

- `GitHubPullRequestCommentProviderCallOrchestrationError`;
- redaction-safe Debug for the bounded orchestration error;
- `code()`, `error()`, `provider_response()`, `provider_call_attempted()`, and `into_parts()` accessors.

Updated:

- `orchestrate_github_pr_comment_provider_call(...)` to return the bounded orchestration error type;
- provider response validation and store-backed transition failures to preserve classified provider responses;
- `execute_with_github_pr_comment_provider_write(...)` to reconcile with provider response context when available;
- success-path reconciliation handling to surface construction failure as `provider_write_error`.

## 4. Validation Boundary Summary

The fix keeps pre-call failures distinct from post-provider failures:

- pre-call validation/store-gate failures remain provider-not-called;
- unclassified provider-call failures remain provider-response-ambiguous;
- provider success plus local transition failure maps to provider-succeeded/local-transition-failed;
- provider failure plus local transition failure maps to provider-failed/local-transition-failed;
- invalid reconciliation metadata after an otherwise successful provider/local transition produces a structured provider-write error.

No automatic retry behavior was added.

## 5. Redaction And Privacy Summary

The new orchestration error Debug output is bounded and does not include:

- auth values;
- comment bodies;
- provider references;
- side-effect IDs;
- idempotency keys;
- redaction metadata;
- caller-supplied summaries.

Provider-write errors continue to use stable codes and bounded messages. The new tests assert that secret-like reconciliation metadata does not leak through result Debug output.

## 6. Test Coverage Summary

Added focused executor regression tests for:

- provider success followed by local transition failure;
- provider failure followed by local transition failure;
- retry-blocked and operator-action-required posture after post-provider local transition failure;
- reconciliation construction failure surfacing as provider-write error;
- non-leakage of secret-like reconciliation metadata.

Existing provider-write tests continue to cover provider-call pre-gates, injected provider behavior, concrete injected-transport provider behavior, reconciliation candidate validation, serialization, and redaction safety.

## 7. Commands Run And Results

- `cargo fmt --all --check` - passed.
- `cargo test -p workflow-core --test local_executor execute_with_github_pr_comment_provider_write` - passed.
- `cargo test -p workflow-core --test provider_write` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.

## 8. Remaining Known Limitations

- Event append after provider call remains unimplemented.
- Report artifact writing after provider call remains unimplemented.
- Provider lookup/query reconciliation remains unimplemented.
- Hidden auth loading remains unimplemented.
- Automatic retries remain unimplemented.
- Default executor write behavior remains unsupported.
- The helper remains GitHub PR comment-only.

## 9. Recommended Next Phase

Recommended next phase: executor-integrated live provider write blocker fix review.

Reason: this fix touches write-adjacent reconciliation semantics and should be reviewed before adding event append, report artifact writing, provider lookup reconciliation, hidden auth loading, automatic retries, CLI behavior, schemas, examples, hosted behavior, or broader write-capable adapters.

## 10. Governed Dogfood Summary

- workflow: `dg/blocker`;
- run: `run-1783289017824773000-2`;
- approval: `approval/run-1783289017824773000-2/fix-approved`;
- approval reason: `delegated-maintainer-approved-executor-provider-write-reconciliation-blocker-fix`;
- approval outcome: granted by delegated maintainer.
- phase close status: completed.
- event summary: 39 total events; 1 approval; 0 retries; 0 escalations.
- event kinds: `ApprovalGranted`, `ApprovalRequested`, `PolicyDecisionRecorded`, `RunCompleted`, `RunCreated`, `RunResumed`, `RunStarted`, `RunValidated`, `SkillInvocationRequested`, `SkillInvocationStarted`, `SkillInvocationSucceeded`, and `StepScheduled`.

Out-of-kernel work included code edits, focused tests, documentation updates, validation commands, git/PR actions, and report posture. No default writes, hidden auth loading, automatic retries, event append, report artifact writing, persistence, CLI behavior, schemas, examples, hosted behavior, broad writes, or release posture changes were performed.
