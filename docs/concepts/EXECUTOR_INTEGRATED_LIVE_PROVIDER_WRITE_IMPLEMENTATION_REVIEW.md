# Executor-Integrated Live Provider Write Implementation Review

## 1. Executive Verdict

Needs blocker fixes.

The implementation is broadly aligned with the approved explicit, local, injected-provider-only scope, but the review found one write-adjacent reconciliation blocker: post-provider local transition failures can be reported as if the provider was not called. That can produce unsafe retry posture and misleading operator guidance after a provider call may already have created or attempted an external write.

Fix-forward note: the blocker is fixed in [Executor-Integrated Live Provider Write Blocker Fix Report](EXECUTOR_INTEGRATED_LIVE_PROVIDER_WRITE_BLOCKER_FIX_REPORT.md). This review keeps the original blocker finding intact.

## 2. Scope Verification

The phase stayed within the approved implementation scope.

Implemented scope:

- explicit executor-adjacent GitHub PR comment provider-write request inputs;
- explicit result wrapper with run, provider response, transition, reconciliation, and error posture;
- explicit helper around existing `LocalExecutor::execute(...)`;
- injected provider trait only;
- completed/failed/canceled terminal run support through existing terminal status handling;
- provider-call/store gates before provider invocation;
- focused executor tests;
- documentation and implementation report.

No accidental implementation was found for:

- default executor writes;
- automatic provider calls;
- hidden auth loading;
- automatic retries;
- workflow event append from provider outcomes;
- audit or observability emission;
- report artifact writes;
- persistence;
- CLI behavior;
- schema or example updates;
- hosted/distributed behavior;
- broader write-capable adapters;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 3. API Assessment

The new API is appropriately explicit and additive.

`execute_with_github_pr_comment_provider_write(...)` accepts a `LocalExecutor`, explicit `SideEffectRecordStore`, injected `GitHubPullRequestCommentProvider`, and explicit `LocalExecutionWithGitHubPrCommentProviderWriteRequest`. Existing `LocalExecutor::execute(...)` remains unchanged.

The request/result types are narrow and testable. Debug implementations are intentionally bounded and do not expose auth values, comment bodies, provider references, side-effect IDs, idempotency keys, or redaction metadata.

The result accessors make the local-only posture visible:

- provider response;
- outcome transition;
- reconciliation candidate;
- provider-write error;
- provider-call-performed posture;
- retry-blocked posture;
- operator-action-required posture;
- workflow-event-appended posture, always false;
- report-artifact-written posture, always false.

## 4. Provider Boundary Assessment

The helper correctly calls only a caller-supplied provider trait. It does not create provider clients, load credentials, inspect environment variables, call GitHub CLI, read git remotes, or use hidden runtime config.

Provider invocation is gated by the existing provider-call orchestration boundary:

- validated provider-call request;
- attempted SideEffect record;
- store-backed record read;
- target/idempotency/lifecycle matching;
- explicit live-call and provider-call flags.

The implementation correctly preserves workflow execution semantics: execution failure before a run exists returns `Err`, while provider-write failures after a run exists are carried in the in-memory result instead of rewriting workflow pass/fail status.

## 5. Reconciliation Assessment

Normal classified paths are handled correctly:

- provider success plus local completed transition returns `ProviderSucceededLocalCompleted`;
- provider failure plus local failed transition returns `ProviderFailedLocalFailed`;
- pre-call gate failures avoid provider invocation and return provider-write error posture;
- unclassified provider call failures return ambiguous reconciliation with retry blocked and operator action required.

However, the implementation has a blocker for post-provider local transition failures.

The provider-call orchestration helper documents that store-backed transition failures can occur after provider invocation. In `crates/workflow-core/src/provider_write.rs`, the provider is called before the completed/failed store transition. If that store transition fails, `orchestrate_github_pr_comment_provider_call(...)` returns an error after provider invocation.

The executor wrapper currently treats only `github_pr_comment_provider.call_unclassified` as provider-call-attempted:

- `crates/workflow-core/src/executor.rs` lines 2991-2998 compute `provider_call_attempted` by comparing the error code to `github_pr_comment_provider.call_unclassified`;
- `crates/workflow-core/src/executor.rs` lines 3015-3023 then build reconciliation with no provider response, no local transition, and `provider_call_attempted` set from that boolean.

That means a provider success followed by a local completed-transition failure, or a provider failure followed by a local failed-transition failure, can be reconciled as `ProviderNotCalled`. This is unsafe because it can make retry posture look less constrained than it should after a possible external write attempt.

Related issue: on the success path, `crates/workflow-core/src/executor.rs` lines 2966-2982 silently drops reconciliation construction failures with `.ok()`. If reconciliation metadata or candidate construction fails after a provider response and local transition, the result can return no reconciliation candidate and no provider-write error. That weakens the inspectability promised by this phase.

## 6. Workflow Semantics Assessment

Default `LocalExecutor::execute(...)` behavior is preserved.

The explicit helper does not mutate workflow events beyond normal execution. It does update the SideEffect record through the already-reviewed provider-call orchestration path when a provider response is classified. That is within scope.

The helper does not append post-terminal workflow events, emit audit records, emit observability records, write report artifacts, write reports, or create CLI output.

The blocker above is not a workflow pass/fail semantic regression, but it is a provider-write reconciliation semantic issue.

## 7. Privacy And Redaction Assessment

No raw provider payload copying was found.

Debug output for request/result types is redaction-safe for:

- auth values;
- comment bodies;
- provider comment references;
- side-effect IDs;
- idempotency keys;
- redaction metadata;
- caller summaries.

Errors use stable codes and bounded messages. The tests assert non-leakage for the new request/result Debug surfaces.

No hidden credential source was introduced.

## 8. Test Quality Assessment

The added tests cover:

- completed workflow run plus provider success;
- completed workflow run plus classified provider failure;
- pre-call provider gate rejection without invocation;
- unclassified provider error mapped to ambiguous reconciliation;
- request/result Debug non-leakage.

The test suite also preserves existing provider-write, WorkReport, SideEffect, Diagnostic, validation, and runtime tests.

Missing blocker tests:

- provider success followed by local completed-transition store failure must not become `ProviderNotCalled`;
- provider failure followed by local failed-transition store failure must not become `ProviderNotCalled`;
- post-provider local transition failure should produce retry-blocked/operator-action-required reconciliation;
- reconciliation construction failure on the success path should produce a structured non-leaking error or be prevented by pre-provider validation.

## 9. Documentation Review

Documentation is mostly honest:

- first executor-integrated explicit helper is implemented;
- default executor writes remain unsupported;
- hidden auth loading is not implemented;
- automatic retries are not implemented;
- event append is not implemented;
- report artifact writing is not implemented;
- CLI behavior, schemas, examples, hosted behavior, broad writes, reasoning lineage, recursive agents, agent swarms, and release posture changes remain unsupported.

The implementation report should be amended during the blocker fix to note the corrected handling of post-provider local transition failures.

## 10. Validation

Validation reviewed from the implementation phase:

- `cargo test -p workflow-core --test provider_write` - passed;
- `cargo test -p workflow-core --test local_executor execute_with_github_pr_comment_provider_write` - passed;
- `cargo fmt --all --check` - passed;
- `cargo clippy --workspace --all-targets -- -D warnings` - passed;
- `cargo test --workspace` - passed;
- `npm run check:docs` - passed;
- `git diff --check` - passed.

Review-only documentation checks:

- `npm run check:docs` - required after this review file is added;
- `git diff --check` - required after this review file is added.

## 11. Blockers

1. Post-provider local transition failures can be misclassified as provider-not-called.

   Action required: preserve enough provider-call outcome context from `orchestrate_github_pr_comment_provider_call(...)` or add a bounded error/result shape so the executor wrapper can distinguish:

   - pre-call failures where provider was not invoked;
   - unclassified provider-call failures where provider may have been invoked;
   - provider success plus local transition failure;
   - provider failure plus local transition failure.

   The blocker fix must ensure retry is blocked and operator/future reconciliation is required whenever a provider may have been called but local transition did not complete.

2. Reconciliation construction failures are silently swallowed on the success path.

   Action required: validate reconciliation metadata before provider invocation or return a structured non-leaking provider-write error if reconciliation cannot be constructed. Do not silently return a provider response with no reconciliation candidate and no error.

## 12. Non-Blocking Follow-Ups

- Add accessor or result detail that distinguishes "provider call definitely performed" from "provider call may have been performed" if future UI/reporting needs that nuance.
- Consider a dedicated executor-level error code namespace for provider-write reconciliation construction failures.
- Add a small review note to the executor-integrated plan after the blocker fix lands so future event/artifact phases know which reconciliation shapes are available.

## 13. Recommended Next Phase

Recommended next phase: executor-integrated live provider write blocker fix.

Reason: this is write-adjacent runtime composition. The next phase should fix reconciliation posture before adding workflow event append, report artifact write, hidden auth loading, automatic retries, CLI behavior, schemas, examples, hosted behavior, or broader write-capable adapters.

## 14. Governed Dogfood Summary

- workflow: `dg/review`;
- run: `run-1783288670089855000-2`;
- approval: `approval/run-1783288670089855000-2/review-scope-approved`;
- approval reason: `delegated-maintainer-approved-executor-integrated-live-provider-write-review`;
- approval outcome: granted by delegated maintainer.
- phase close status: completed;
- event summary: 39 total events, 1 approval, 0 retries, 0 escalations.

The review remained inside the approved review scope. Out-of-kernel work included implementation inspection, test inspection, documentation review, validation review, and this review report. No code fixes, automatic writes, hidden auth loading, retries, event append, report artifact writing, persistence, CLI behavior, schemas, examples, hosted behavior, broad writes, or release posture changes were performed.
