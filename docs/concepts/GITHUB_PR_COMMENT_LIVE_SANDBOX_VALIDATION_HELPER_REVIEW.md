# GitHub PR Comment Live Sandbox Validation Helper Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The live sandbox validation helper is an appropriately narrow write-adjacent
composition boundary. It validates sandbox target proof, sandbox readiness,
caller-supplied provider-call input, and existing provider-call request gates
before invoking the caller-supplied injected provider. It preserves the current
no-default-write product posture.

Recommended next phase: defer executor-facing, CLI-facing, or real-network
sandbox validation until separately planned. Continue with the next reviewed
roadmap lane only if it composes existing primitives through explicit,
non-default runtime paths.

## 2. Scope Verification

The phase stayed within the approved explicit injected helper scope.

It added:

- `GitHubPullRequestCommentLiveSandboxValidationInput`;
- `GitHubPullRequestCommentLiveSandboxValidationResult`;
- `validate_and_orchestrate_github_pr_comment_live_sandbox`;
- target-proof/readiness/provider-call consistency checks;
- explicit `AllowedForSandbox` readiness gating;
- reuse of existing injected provider-call orchestration;
- store-backed attempted-to-completed or attempted-to-failed SideEffect
  transition through the existing provider-call boundary;
- focused tests;
- roadmap, plan, and phase-report documentation.

It did not add:

- production writes;
- default provider writes;
- automatic executor writes;
- hidden auth loading from environment variables, keychains, GitHub CLI, git
  credentials, browser sessions, config files, OAuth state, or secret managers;
- CLI mutation commands;
- workflow schema fields;
- example updates;
- hosted or distributed runtime behavior;
- broad GitHub mutations beyond the explicit injected PR comment lane;
- Jira, CI, filesystem, HTTP, or arbitrary provider writes;
- automatic retries, repair, or recovery mutation;
- workflow event append;
- report artifact writes;
- reasoning lineage;
- release posture changes.

## 3. Helper API Assessment

The helper API is minimal and idiomatic for this phase.

`GitHubPullRequestCommentLiveSandboxValidationInput` composes only existing
reviewed inputs:

- `ProviderWriteSandboxTargetProof`;
- `ProviderWriteSandboxReadinessInput`;
- `GitHubPullRequestCommentProviderCallInput`;
- transition timestamp;
- bounded transition references;
- evidence reference count.

`GitHubPullRequestCommentLiveSandboxValidationResult` exposes the bounded
readiness result and the existing provider-call orchestration result. It also
has explicit `workflow_event_appended()` and `report_artifact_written()`
accessors returning `false`, which makes the non-event and non-artifact
boundary inspectable in tests.

The helper accepts an injected `SideEffectRecordStore` and an injected
`GitHubPullRequestCommentProvider`. It does not construct clients, discover
auth, read global state, or create a parallel provider-write model.

## 4. Gate Sequence Assessment

The gate sequence is appropriately fail-closed before provider transport.

The helper validates:

1. target proof validates;
2. transition references validate;
3. target proof target matches provider-call target;
4. target proof capability matches readiness capability;
5. target proof adapter target matches readiness target;
6. target proof target posture matches readiness target posture;
7. target proof idempotency key matches provider-call idempotency key;
8. readiness auth posture is explicit caller-supplied;
9. provider-call mode is `LiveSandbox`;
10. sandbox readiness decision is `AllowedForSandbox`;
11. existing provider-call request validation passes inside the reused
    orchestration boundary.

Only after those gates pass does the helper invoke the injected provider.

This is the right order. The helper checks local proof/readiness consistency
before the readiness decision and before the lower-level provider-call
orchestration. The lower-level orchestration remains responsible for validating
attempted SideEffect state, live-call opt-in, provider-call opt-in, target,
comment body, auth wrapper, summary, and redaction metadata.

## 5. Provider Boundary Assessment

The provider boundary remains explicit and injected.

The helper invokes only the supplied `GitHubPullRequestCommentProvider`. It
does not:

- create a network client;
- load credentials;
- read environment variables;
- read keychains;
- read GitHub CLI state;
- read git credentials;
- read browser state;
- use repository config;
- use global runtime configuration.

Tests verify that provider invocation count remains zero for target-proof
mismatch, denied readiness, and hidden/ambient auth posture. The success test
verifies exactly one provider invocation after gates pass.

## 6. SideEffect Lifecycle Assessment

The helper correctly delegates SideEffect lifecycle mutation to the existing
provider-call orchestration boundary.

On classified provider success, the attempted SideEffect transitions to
`Completed`. On classified provider failure, the existing orchestration helper
can transition to `Failed`. Unclassified provider errors remain bounded
orchestration errors.

The helper itself does not append workflow events, emit audit records, write
report artifacts, mutate `WorkflowRun`, emit CLI output, or change default
executor behavior.

## 7. Error Handling And Redaction Assessment

Error handling is stable and non-leaking.

Pre-provider gate failures use stable codes such as:

- `github_pr_comment_live_sandbox_validation.target_proof.invalid`;
- `github_pr_comment_live_sandbox_validation.target.mismatch`;
- `github_pr_comment_live_sandbox_validation.capability.mismatch`;
- `github_pr_comment_live_sandbox_validation.readiness_target.mismatch`;
- `github_pr_comment_live_sandbox_validation.target_posture.mismatch`;
- `github_pr_comment_live_sandbox_validation.idempotency.mismatch`;
- `github_pr_comment_live_sandbox_validation.auth_posture.not_explicit`;
- `github_pr_comment_live_sandbox_validation.mode.unsupported`;
- `github_pr_comment_live_sandbox_validation.readiness_not_allowed`.

The errors do not include raw owner/repository strings, PR comment body,
auth material, target-proof statement, correlation ID, idempotency key, provider
payloads, command output, paths, or secret-like values.

Debug output for the helper input redacts the provider-call payload and the
target-proof sensitive fields through the existing redaction-safe Debug
implementations. The added debug regression test checks that comment body,
auth marker, target-proof statement, correlation ID, and idempotency key are not
rendered.

## 8. Test Quality Assessment

The focused tests cover the important phase behavior:

- successful live sandbox validation invokes the injected provider once;
- successful provider response transitions the attempted SideEffect to
  completed;
- target-proof mismatch prevents provider invocation;
- denied readiness prevents provider invocation;
- hidden/ambient auth posture prevents provider invocation;
- Debug output does not leak comment body, auth marker, target-proof statement,
  correlation ID, or idempotency key.

Existing lower-level provider-write tests continue to cover provider-call
request validation, injected provider orchestration, concrete injected HTTP
provider auth matching, SideEffect lifecycle transitions, lookup,
reconciliation, recovery, and redaction boundaries.

Missing or shallow coverage:

- The helper-specific test block does not directly exercise a classified
  provider failure transitioning the attempted record to failed through this
  exact helper.
- The helper-specific test block does not directly exercise every individual
  mismatch gate, such as capability mismatch or target-posture mismatch.

These are non-blocking. The implementation is simple, the lower-level
orchestration already covers provider failure behavior, and the most dangerous
helper-level risk is provider invocation before gates pass. That risk is
covered by the negative invocation-count tests.

## 9. Documentation Review

Documentation is honest about current behavior.

The plan and report state that this phase implements an explicit injected
helper only. They do not claim:

- production writes;
- default provider writes;
- automatic executor writes;
- CLI mutation behavior;
- hidden auth loading;
- workflow event append;
- report artifact writes;
- schemas;
- examples;
- hosted behavior;
- reasoning lineage;
- release posture changes.

The roadmap should link this review next to the helper report so future readers
can see that the write-adjacent boundary was reviewed before any broader live
sandbox exposure.

## 10. Blockers

No blockers.

## 11. Non-Blocking Follow-Ups

- Add helper-specific coverage for classified provider failure transitioning
  the attempted record to failed through
  `validate_and_orchestrate_github_pr_comment_live_sandbox`.
- Add one small matrix test for capability, target-posture, and readiness-target
  mismatch codes if this helper grows.
- Keep any real network sandbox validation behind a separately planned,
  explicit, non-default, caller-supplied-auth boundary.
- Keep executor-facing and CLI-facing write paths blocked until approval
  presentation, sandbox target proof, readiness, provider event proof, report
  disclosure, and recovery posture are composed at that boundary.

## 12. Recommended Next Phase

Recommended next phase: continue with the next explicit runtime-composition
roadmap lane, not broad provider-write expansion.

This helper is accepted as a local write-adjacent building block. It should not
be treated as approval for production writes, automatic executor writes, CLI
mutation commands, hidden auth loading, or live network validation by default.
