# GitHub PR Comment Provider-Call Orchestration Helper Report

## 1. Executive Summary

The injected GitHub PR comment provider-call orchestration helper is implemented as a narrow local boundary. It validates the existing provider-call request gates, invokes only a caller-supplied provider trait, and transitions the attempted `SideEffectRecord` to completed or failed from a classified provider response.

This phase does not add a concrete GitHub client, auth loading, executor-integrated writes, automatic workflow event append, report artifact writes, CLI mutation behavior, schemas, examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes.

## 2. Scope Completed

- Added `GitHubPullRequestCommentProviderCallOrchestrationInput`.
- Added `GitHubPullRequestCommentProviderCallOrchestrationResult`.
- Added `orchestrate_github_pr_comment_provider_call`.
- Validated transition references before provider invocation.
- Reused `GitHubPullRequestCommentProviderCallRequest::new` so explicit live/provider-call gates remain active.
- Loaded and verified the attempted `SideEffectRecord` from the store before calling the injected provider.
- Invoked only the supplied `GitHubPullRequestCommentProvider` trait.
- Mapped provider success to a completed side-effect lifecycle transition.
- Mapped classified provider failure to a failed side-effect lifecycle transition.
- Rejected unclassified provider errors without lifecycle transition.
- Rejected fixture/dry-run responses from the provider-call path without lifecycle transition.
- Kept workflow event append and report artifact write permissions false.
- Added focused provider-write tests.
- Updated the live provider-call plan and roadmap.

## 3. Scope Explicitly Not Completed

- No concrete GitHub HTTP client.
- No auth loading from environment variables, config, keychains, GitHub CLI state, git remotes, or hidden global state.
- No automatic provider writes.
- No executor-integrated write path.
- No workflow event append.
- No audit event emission.
- No report artifact write.
- No CLI mutation command.
- No workflow schema fields.
- No examples.
- No hosted or distributed runtime behavior.
- No reasoning lineage implementation.
- No recursive agents or agent swarms.
- No Level 3/4 autonomy expansion.
- No release posture change.

## 4. API Summary

`GitHubPullRequestCommentProviderCallOrchestrationInput` contains:

- the existing `GitHubPullRequestCommentProviderCallInput`;
- transition timestamp;
- additional transition references;
- evidence reference count.

`orchestrate_github_pr_comment_provider_call` accepts:

- a `SideEffectRecordStore`;
- an injected `GitHubPullRequestCommentProvider`;
- the orchestration input.

It returns `GitHubPullRequestCommentProviderCallOrchestrationResult`, which exposes:

- the validated provider response;
- the store-backed lifecycle transition result;
- explicit `false` accessors for workflow event append and report artifact write.

## 5. Provider-Call Behavior

The helper only calls the injected provider after:

- provider-call input validates;
- live call is explicitly enabled;
- provider call is explicitly enabled;
- mode is `LiveSandbox`;
- auth material is present through the explicit auth wrapper;
- the request target matches the attempted side-effect record;
- idempotency matches the attempted side-effect record;
- the store record still exists and remains in the attempted lifecycle state.

Unclassified provider trait errors are mapped to `github_pr_comment_provider.call_unclassified` and do not transition the record.

## 6. Lifecycle Transition Behavior

Provider success:

- requires a provider comment reference;
- transitions the attempted record to completed;
- stores the provider reference as an outcome reference;
- preserves bounded response summary and supplied transition references.

Provider failure:

- requires a bounded provider error code;
- transitions the attempted record to failed;
- stores the provider error code as the failure reason;
- preserves bounded response summary and supplied transition references.

Fixture and dry-run responses are rejected because this path is specifically for classified provider outcomes.

## 7. Workflow, Event, and Report Boundary

The helper does not append workflow events, emit audit records, write report artifacts, mutate workflow runs, or expose CLI output.

Those remain separate reviewed boundaries. This phase only connects an injected provider response to the existing `SideEffectRecord` lifecycle store.

## 8. Redaction and Privacy

The helper uses existing validated constructors for provider-call requests, provider responses, outcome references, side-effect references, and lifecycle transitions.

The implementation does not store or output raw provider payloads, raw command output, raw CI logs, raw GitHub file contents, raw spec contents, raw parser payloads, environment variable values, credentials, authorization headers, private keys, token-like values, or secret-like values.

Debug output for the orchestration input and result redacts sensitive request, response, provider reference, comment body, auth, side-effect ID, and idempotency details.

## 9. Test Coverage Summary

Focused tests cover:

- provider success transitions attempted records to completed;
- classified provider failure transitions attempted records to failed;
- unclassified provider errors do not transition;
- fixture/dry-run responses do not transition;
- pre-call gate failures do not invoke the provider;
- request idempotency mismatch is rejected before provider invocation;
- Debug output does not leak auth, comment body, provider references, or stable side-effect identifiers;
- existing provider-call request tests continue to pass.

## 10. Commands Run and Results

- `cargo fmt --all` - passed.
- `cargo test -p workflow-core --test provider_write` - passed, 78 tests.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

## 11. Dogfood Summary

Governed workflow:

- workflow: `dg/implement`;
- run: `run-1783279890996298000-2`;
- approval: `approval/run-1783279890996298000-2/implementation-approved`;
- approval reason: `delegated-maintainer-approved-injected-provider-orchestration`;
- approved scope: injected provider-call orchestration helper only;
- strict non-goals: no concrete network auth loading, executor integration, CLI behavior, schemas, or examples.

Phase close summary:

- status: `Completed`;
- terminal: true;
- events total: 39;
- approvals: 1;
- retries: 0;
- escalations: 0;
- event kinds: `ApprovalGranted:1`, `ApprovalRequested:1`, `PolicyDecisionRecorded:8`, `RunCompleted:1`, `RunCreated:1`, `RunResumed:1`, `RunStarted:1`, `RunValidated:1`, `SkillInvocationRequested:6`, `SkillInvocationStarted:6`, `SkillInvocationSucceeded:6`, `StepScheduled:6`.

Out-of-kernel work disclosed:

- repo edits to Rust source, tests, and docs;
- shell validation commands;
- no skipped required checks;
- no git or PR action before phase validation;
- no report artifact was written by the kernel for this phase.

## 12. Remaining Known Limitations

- The provider trait is injected by the caller; there is no built-in GitHub network client.
- Auth must be passed explicitly; no credential discovery exists.
- Provider-native idempotency is not implemented.
- If a provider succeeds but the subsequent local lifecycle transition fails, the caller must handle reconciliation. This phase does not add remote rollback, duplicate prevention beyond local idempotency, or artifact/event reconciliation.
- Executor integration remains future work.
- Event append and report artifact writes remain separate reviewed boundaries.

## 13. Recommended Next Phase

Recommended next phase: GitHub PR comment provider-call orchestration helper review.

The review should assess the helper’s safety boundary, lifecycle transition behavior, error handling, privacy posture, and whether any additional blocker fixes are needed before planning concrete provider-client/auth loading or executor integration.
