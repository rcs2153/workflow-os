# Executor-Integrated Live Provider Write Blocker Fix Review

## 1. Executive Verdict

Blocker fixed; proceed to provider write event append planning.

The blocker fix correctly preserves classified provider response context when local SideEffect lifecycle transition fails after a provider call, and it no longer silently drops reconciliation construction failures on the normal success path. The phase remained within the approved blocker-fix scope and did not broaden live write behavior.

## 2. Scope Verification

The fix stayed within the approved blocker-fix scope.

No accidental scope expansion was found:

- no automatic provider writes;
- no default executor write behavior;
- no workflow event append after provider call;
- no audit or observability emission;
- no report artifact writing;
- no report persistence;
- no hidden auth loading;
- no provider lookup or query reconciliation;
- no automatic retry behavior;
- no CLI behavior;
- no schema or example updates;
- no hosted runtime behavior;
- no broad write-capable adapters;
- no release posture changes.

## 3. Original Blocker Restatement

The implementation review found two blockers:

- post-provider local transition failures could be misclassified as `ProviderNotCalled`;
- reconciliation construction failures on the success path were converted to `None` with `.ok()`, which could silently drop an invalid reconciliation candidate.

The concrete risk was that the executor-integrated helper could call a provider, receive a classified success or failure response, fail while transitioning local SideEffect state, and then return a result that looked as if no provider call happened. That is unsafe because retry posture and operator action depend on distinguishing "provider not called" from "provider called but local state did not reconcile."

## 4. Fix Approach Assessment

The selected approach is minimal and appropriate.

The fix adds a bounded `GitHubPullRequestCommentProviderCallOrchestrationError` that wraps the structured `WorkflowOsError` and optionally carries a classified `GitHubPullRequestCommentWriteResponse`. This keeps pre-call failures, ambiguous provider-call failures, and post-provider local transition failures distinct without adding event append, persistence, provider lookup, retry, or broader runtime write semantics.

The error type is redaction-safe and exposes only bounded accessors:

- stable error code;
- underlying structured error by reference;
- optional classified provider response;
- provider-call-attempted boolean;
- owned parts for executor reconciliation.

The provider-call orchestration helper now returns this bounded error type, and the executor wrapper consumes it to preserve the provider response where appropriate.

## 5. Validation Boundary Assessment

The validation boundary is now correct for this phase:

- pre-call validation and store-gate failures remain provider-not-called;
- unclassified provider-call failures remain ambiguous provider-call failures;
- classified provider success plus local transition failure maps to `ProviderSucceededLocalTransitionFailed`;
- classified provider failure plus local transition failure maps to `ProviderFailedLocalTransitionFailed`;
- both post-provider local transition failure states block retry and require operator/future reconciliation;
- normal provider response plus local transition success still attempts reconciliation candidate construction;
- reconciliation construction failure now returns a structured provider-write error instead of being silently dropped.

The fix does not convert provider/local failures into misleading user project diagnostics.

## 6. Retry And Operator Posture Assessment

The retry posture is materially improved.

The new regression tests verify that both provider-success/local-transition-failure and provider-failure/local-transition-failure produce reconciliation candidates with:

- provider-call-performed posture;
- retry blocked;
- operator action required.

That is the right conservative behavior for write-adjacent work. The helper still does not implement automatic retry or provider lookup reconciliation.

## 7. Privacy And Redaction Assessment

The fix remains redaction-safe.

The new orchestration error Debug output does not include:

- auth material;
- GitHub comment body;
- provider comment reference;
- side-effect IDs;
- idempotency keys;
- redaction metadata values;
- caller-supplied summaries;
- raw provider payloads.

Provider-write errors continue to use stable bounded codes and messages. The added reconciliation-construction-failure test verifies that secret-like redaction metadata does not leak through the new result path.

## 8. Test Quality Assessment

The blocker is covered by focused tests:

- provider success followed by local transition failure;
- provider failure followed by local transition failure;
- retry blocked and operator action required for both post-provider local transition failures;
- reconciliation construction failure surfaced as provider-write error;
- non-leakage of secret-like reconciliation metadata in Debug output.

Existing provider-write tests continue to cover:

- pre-call gates;
- injected provider behavior;
- injected transport provider client behavior;
- provider response validation;
- reconciliation candidate validation;
- serialization;
- redaction safety.

No blocker-level test gaps were found.

Non-blocking test follow-up: when the next event-append phase exists, add tests proving post-provider local transition failures are not later projected as successful appendable events.

## 9. Documentation Review

Documentation is honest and current:

- the implementation report has a fix-forward note;
- the implementation review keeps the original blocker finding and records that a fix-forward report exists;
- the blocker-fix report describes the issue, approach, validation boundary, privacy posture, tests, validation commands, dogfood run, and remaining limitations;
- the roadmap and executor-integrated plan point to the blocker fix.

Docs continue to state that event append, report artifact writing, hidden auth loading, retries, default writes, CLI behavior, schemas, examples, hosted behavior, and broad write-capable adapters remain unimplemented.

## 10. Blockers

No blockers remain for this fix.

## 11. Non-Blocking Follow-Ups

- Keep the orchestration error as a narrow provider-call boundary; do not generalize it before another provider write path exists.
- In the future event-append phase, ensure only reconciled provider outcomes can be projected into workflow events.
- In the future provider lookup/reconciliation phase, define how an operator can resolve post-provider local transition failures without replaying unsafe writes.

## 12. Recommended Next Phase

Recommended next phase: provider write event append planning.

Reason: provider-call execution and reconciliation now have a reviewed safe boundary. The next runtime-composition gap is deciding how, when, and under what reconciliation gates classified provider outcomes may produce workflow event/audit projections without implying automatic writes, hidden auth loading, retries, artifacts, CLI behavior, schemas, examples, hosted behavior, or broader adapter support.

## 13. Validation

Validated before and during this review:

- `cargo fmt --all --check` - passed.
- `cargo test -p workflow-core --test local_executor execute_with_github_pr_comment_provider_write` - passed, 8 focused tests.
- `cargo test -p workflow-core --test provider_write` - passed, 96 tests.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.

## 14. Governed Dogfood Summary

- workflow: `dg/review`;
- run: `run-1783290630411210000-2`;
- approval: `approval/run-1783290630411210000-2/review-scope-approved`;
- approval reason: `delegated-maintainer-approved-executor-provider-write-blocker-fix-review`;
- approval outcome: granted by delegated maintainer.
- phase close status: completed.
- event summary: 39 total events; 1 approval; 0 retries; 0 escalations.
- event kinds: `ApprovalGranted`, `ApprovalRequested`, `PolicyDecisionRecorded`, `RunCompleted`, `RunCreated`, `RunResumed`, `RunStarted`, `RunValidated`, `SkillInvocationRequested`, `SkillInvocationStarted`, `SkillInvocationSucceeded`, and `StepScheduled`.

Out-of-kernel work included review, documentation updates, validation commands, git/PR actions, and report posture. No default writes, hidden auth loading, automatic retries, event append, report artifact writing, persistence, CLI behavior, schemas, examples, hosted behavior, broad writes, or release posture changes were performed.
