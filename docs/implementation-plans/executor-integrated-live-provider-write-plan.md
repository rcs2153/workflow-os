# Executor-Integrated Live Provider Write Plan

Status: Planning accepted with non-blocking follow-ups in [Executor-Integrated Live Provider Write Plan Review](../concepts/EXECUTOR_INTEGRATED_LIVE_PROVIDER_WRITE_PLAN_REVIEW.md). This follows the accepted [GitHub PR Comment Provider Write Reconciliation Model Review](../concepts/GITHUB_PR_COMMENT_PROVIDER_WRITE_RECONCILIATION_MODEL_REVIEW.md). It plans the smallest future executor-integrated live provider write path for GitHub pull request comments. It does not implement executor writes, automatic provider calls, hidden auth loading, automatic retries, workflow event append, report artifact writing, CLI behavior, schemas, examples, hosted behavior, broad write support, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes.

## 1. Executive Summary

Workflow OS now has the pieces needed to plan the first executor-integrated live provider write path:

- write preflight validation;
- GitHub PR comment request/response models;
- proposed SideEffect record composition and persistence;
- approval-side-effect linkage;
- store-backed attempted/completed/failed lifecycle transitions;
- executor SideEffect lifecycle event append helpers;
- injected provider-call helper and concrete injected-transport GitHub PR comment provider client;
- provider write reconciliation model/helper;
- report artifact SideEffect referential integrity gates.

The next question is how the executor should compose these reviewed primitives into one explicit, opt-in, local live write path without making writes automatic.

The recommended first implementation is an executor-adjacent helper/API for GitHub PR comments only. It should accept explicit inputs, explicit caller-supplied auth/provider, and explicit side-effect/report context. It should return a structured in-memory result that includes the run, provider response or reconciliation candidate, lifecycle transition/event append status, and report/artifact status. It must not become default execution behavior.

## 2. Goals

- Compose existing primitives into the smallest executor-integrated live write path.
- Keep provider writes denied by default.
- Require explicit opt-in at the API boundary.
- Require explicit caller-supplied auth or caller-supplied provider.
- Preserve policy, approval, high-assurance, idempotency, SideEffect, event, and report/artifact gates.
- Preserve workflow pass/fail semantics unless explicitly stated by the new path.
- Prevent duplicate GitHub comments after ambiguous provider outcomes.
- Return reconciliation status when provider/local state disagree.
- Keep errors stable, bounded, and redaction-safe.
- Avoid raw payload copying.
- Keep the implementation local and in-memory except for already-reviewed explicit store/event/report artifact boundaries.

## 3. Non-Goals

Do not implement or authorize in the first executor-integrated live write phase:

- implementation in this planning phase;
- default `LocalExecutor::execute(...)` provider writes;
- automatic provider calls for all runs;
- hidden auth loading from environment variables, keychains, GitHub CLI, git remote, config files, OAuth, or secret managers;
- automatic retries;
- provider-native idempotency claims not guaranteed by GitHub;
- provider lookup/query reconciliation;
- broad GitHub write support;
- non-comment GitHub mutations;
- Jira, CI, or other provider writes;
- CLI mutation commands or flags;
- workflow schema fields;
- examples;
- hosted or distributed runtime behavior;
- production credential management;
- enterprise RBAC, IdP, quorum approval, or revocation enforcement;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- release posture changes.

## 4. Current Baseline

Implemented and reviewed:

- adapter write preflight model/helper;
- GitHub PR comment write request/response model;
- no-provider fixture/dry-run validation;
- proposed `SideEffectRecord` composition and persistence;
- proposed SideEffect event construction;
- approval-side-effect linkage;
- store-backed attempted/completed/failed lifecycle transitions;
- executor attempted/completed/failed lifecycle event append helpers;
- no-provider attempted and outcome orchestration helpers;
- provider-call request model;
- injected provider-call trait/helper;
- concrete injected-transport GitHub PR comment provider client;
- provider write reconciliation model/helper;
- report artifact SideEffect referential integrity and GitHub PR comment provider candidate gates.

Still missing:

- executor-integrated live write API;
- explicit result type for provider-write executor integration;
- event append composition after provider call;
- reconciliation-aware report/artifact disclosure path;
- operator recovery guidance after ambiguous provider outcomes;
- hidden auth loading, which remains deferred;
- CLI behavior, which remains deferred.

## 5. First Implementation Recommendation

Recommended first implementation: **explicit executor-adjacent GitHub PR comment live write helper, local only**.

Possible API shape:

- `LocalExecutionWithGitHubPrCommentWriteRequest`;
- `LocalExecutionGitHubPrCommentWriteInputs`;
- `LocalExecutionWithGitHubPrCommentWriteResult`;
- `execute_with_github_pr_comment_write(...)`; or
- a free helper wrapping `LocalExecutor::execute(...)` if that better matches current patterns.

The first implementation should remain one lane only:

- one workflow execution;
- one explicit GitHub PR comment write candidate;
- one explicit provider call;
- caller-supplied auth/provider only;
- local state backend/store supplied by the caller;
- no automatic retry;
- no broad adapter registry.

## 6. Required Pre-Call Gates

The executor-integrated helper must fail before provider invocation unless all required gates pass:

1. Workflow execution completed the pre-write local governance path required by the helper.
2. The write request has passed adapter write preflight.
3. Policy references are present and valid.
4. Approval references are present when authority requires approval.
5. Approval-side-effect linkage is valid when required.
6. High-assurance approval disclosure is present when requested by the explicit policy.
7. Proposed SideEffect record is persisted.
8. Proposed SideEffect event append is completed when required by the helper.
9. Attempted SideEffect transition is persisted.
10. Attempted SideEffect event append is completed when required by the helper.
11. Idempotency key is present and bound to the side-effect.
12. Prior local lifecycle state does not already prove completion/failure.
13. Provider call is explicitly enabled for this request.
14. Caller-supplied auth/provider is present.
15. Auth/provider input passes redaction-safe validation.

If any gate fails, no provider call occurs.

## 7. Execution Sequence

The first implementation should use this ordering:

1. Execute or accept an existing local run through an explicit executor request.
2. Compose and persist the proposed GitHub PR comment SideEffect record.
3. Append the proposed SideEffect event only through the explicit executor append boundary.
4. Validate approval linkage and high-assurance disclosure when required.
5. Transition the record to attempted through the store-backed transition helper.
6. Append the attempted SideEffect event only through the explicit executor append boundary.
7. Build the provider-call request from the attempted record and explicit provider inputs.
8. Invoke only the caller-supplied provider or explicitly supplied concrete injected-transport provider.
9. Transition the attempted record to completed or failed when a classified provider response is available.
10. If local transition fails after provider response, create a reconciliation candidate and block retry.
11. Append completed/failed lifecycle event only after local transition success.
12. Validate report/artifact SideEffect integrity and approval linkage when artifact writing is explicitly part of the selected path.
13. Return an in-memory result that discloses all completed, skipped, failed, and ambiguous steps.

The helper must not mutate state outside these reviewed boundaries.

## 8. Result Model Recommendation

The first result type should expose:

- owned `WorkflowRun`;
- optional proposed `SideEffectRecord`;
- optional attempted transition result;
- optional provider response;
- optional completed/failed transition result;
- optional reconciliation candidate;
- optional lifecycle event append IDs;
- optional report artifact result if explicitly requested by a reviewed artifact path;
- optional structured provider-write error;
- booleans/counts for provider call performed, event append performed, artifact write performed, retry blocked, operator action required;
- safe accessors and `into_parts()`.

Debug output must redact report text, provider references, auth context, side-effect IDs, idempotency keys, paths, and any caller-supplied metadata.

## 9. Provider Call And Auth Boundary

The first implementation should prefer a caller-supplied provider trait for tests and a caller-constructed concrete provider for real calls.

Rules:

- no hidden auth loading;
- no environment reads;
- no keychain reads;
- no GitHub CLI reads;
- no git remote inference;
- no OAuth or secret-manager integration;
- no storing auth in records, events, reports, artifacts, logs, or errors;
- no Debug/serialization of auth values;
- no automatic provider construction from global state.

Production auth loading requires separate planning and review.

## 10. Reconciliation Policy

The executor-integrated helper must call `reconcile_github_pr_comment_provider_write(...)` whenever provider/local state may disagree.

Required handling:

- provider success plus completed transition: continue to completed event append;
- provider failure plus failed transition: continue to failed event append;
- provider success plus failed/missing local transition: return reconciliation candidate, block retry, require operator action;
- provider failure plus failed/missing local transition: return reconciliation candidate, block retry, require operator action;
- provider response ambiguity: return reconciliation candidate, block retry, require operator action;
- local state ambiguity: return reconciliation candidate, block retry, require operator action;
- provider not called: disclose no provider call occurred.

Do not retry a provider call automatically after any ambiguous state.

## 11. Workflow Semantics

The executor-integrated live write path must be explicit and separate from default execution.

Rules:

- existing `LocalExecutor::execute(...)` behavior remains unchanged;
- existing report-bearing executor paths remain unchanged unless explicitly selected;
- report/artifact failure after a run exists must be represented separately from workflow execution failure;
- provider-write failure must not silently rewrite completed workflow state unless the selected API explicitly returns a failed provider-write result;
- ambiguous provider outcomes must be visible and must not be hidden behind a successful workflow result.

The result should distinguish:

- workflow execution status;
- provider write status;
- side-effect lifecycle status;
- event append status;
- report/artifact status;
- reconciliation status.

## 12. Event, Audit, And Report Artifact Boundary

Workflow event append remains a reviewed executor boundary, not provider behavior.

Audit/observability emission remains deferred unless explicitly planned for this path.

Report artifact writing may be included only if the implementation selects an already-reviewed explicit report artifact helper and keeps it opt-in. If included, it must validate:

- SideEffect referential integrity;
- GitHub PR comment provider candidate/event requirements;
- approval linkage when required;
- high-assurance disclosure when required.

The first implementation may defer report artifact writes and return report/artifact obligations instead. That is the recommended conservative v1.

## 13. Failure Handling

Errors must be stable and non-leaking.

Recommended categories:

- `executor_github_pr_comment_write.preflight_failed`;
- `executor_github_pr_comment_write.proposed_record_failed`;
- `executor_github_pr_comment_write.proposed_event_failed`;
- `executor_github_pr_comment_write.approval_linkage_failed`;
- `executor_github_pr_comment_write.attempted_transition_failed`;
- `executor_github_pr_comment_write.attempted_event_failed`;
- `executor_github_pr_comment_write.provider_call_failed`;
- `executor_github_pr_comment_write.provider_response_ambiguous`;
- `executor_github_pr_comment_write.local_transition_failed`;
- `executor_github_pr_comment_write.lifecycle_event_failed`;
- `executor_github_pr_comment_write.reconciliation_required`;
- `executor_github_pr_comment_write.report_artifact_failed`.

Errors must not include:

- raw provider payloads;
- request bodies;
- comment bodies;
- headers;
- auth values;
- private repository URLs;
- local paths;
- command output;
- raw specs;
- parser payloads;
- stack traces;
- token-like strings.

## 14. Privacy And Redaction

The path must remain reference-first.

Do not store or output:

- raw GitHub responses;
- raw HTTP request/response bodies;
- comment bodies beyond already validated write request boundaries;
- authorization headers;
- tokens or credentials;
- private keys;
- environment values;
- CI logs;
- command output;
- parser payloads;
- raw spec contents;
- raw provider payloads.

All Debug, serialization, error, report, and event paths must stay redaction-safe.

## 15. Test Plan

Future implementation tests should cover:

- existing `LocalExecutor::execute(...)` remains unchanged;
- default executor path never performs provider writes;
- explicit executor-integrated live write path rejects missing opt-in before provider call;
- rejects missing auth/provider before provider call;
- rejects missing attempted record before provider call;
- rejects missing approval linkage before provider call when required;
- rejects prior completed/failed state without duplicate provider call;
- successful injected provider call transitions attempted record to completed;
- classified provider failure transitions attempted record to failed;
- provider success plus local completed transition appends completed lifecycle event when selected;
- provider failure plus local failed transition appends failed lifecycle event when selected;
- provider success plus local transition failure returns reconciliation candidate and blocks retry;
- provider failure plus local transition failure returns reconciliation candidate and blocks retry;
- provider response ambiguity returns reconciliation candidate and blocks retry;
- no raw provider payload, auth, command output, spec content, or token-like values leak through Debug, serialization, or errors;
- no report artifact is written unless explicitly selected;
- no CLI output is emitted;
- no schemas or examples are changed;
- existing provider-write, executor, report artifact, SideEffect, approval, and validation tests still pass.

## 16. Proposed Implementation Sequence

Recommended small phases:

1. Add explicit executor-integrated live write request/result types for GitHub PR comments.
2. Add an injected-provider-only executor helper that stops before concrete auth/provider construction.
3. Add event append composition for proposed/attempted/completed/failed lifecycle events through existing executor boundaries.
4. Add reconciliation candidate return path for remote/local ambiguity.
5. Add focused tests for success, failure, ambiguity, no duplicate provider call, non-leakage, and unchanged default executor behavior.
6. Review.
7. Only after review, consider optional concrete provider construction with explicit caller-supplied auth.
8. Only after separate planning, consider hidden auth loading or CLI mutation behavior.

## 17. Open Questions

- Should the first implementation execute the workflow itself, or accept an already-created terminal/paused run plus explicit side-effect context?
- Should report artifact writing be deferred entirely and represented only as obligations in the first executor-integrated live write slice?
- How should provider-write failure affect the returned workflow result when the workflow execution itself completed?
- Should event append failure after provider success produce a dedicated reconciliation candidate or a separate event-append mismatch result?
- Should the result type include audit/report candidates, or should those remain separate future paths?
- What is the smallest live sandbox test that proves the provider call without relying on public network availability in normal CI?
- Should the path support exactly one write candidate per request, or allow a bounded list later?

## 18. Final Recommendation

Next implementation phase: explicit executor-integrated GitHub PR comment live write request/result model and injected-provider-only helper.

Still do not build:

- default provider writes;
- hidden auth loading;
- automatic retries;
- broad GitHub writes;
- non-comment provider writes;
- CLI mutation commands;
- schemas;
- examples;
- hosted behavior;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- release posture changes.

## 19. Governed Dogfood Summary

- workflow: `dg/d`;
- run: `run-1783286362614076000-2`;
- approval: `approval/run-1783286362614076000-2/planning-approved`;
- approval reason: `delegated-maintainer-approved-executor-integrated-live-provider-write-planning`;
- approval outcome: granted by delegated maintainer.

- validation summary: `npm run check:docs` passed; `git diff --check` passed;
- phase close status: completed;
- events total: 39;
- approvals: 1;
- retries: 0;
- escalations: 0;
- event kinds: `ApprovalGranted:1`, `ApprovalRequested:1`, `PolicyDecisionRecorded:8`, `RunCompleted:1`, `RunCreated:1`, `RunResumed:1`, `RunStarted:1`, `RunValidated:1`, `SkillInvocationRequested:6`, `SkillInvocationStarted:6`, `SkillInvocationSucceeded:6`, `StepScheduled:6`.

Out-of-kernel work performed by the executor included planning documentation, roadmap/status updates, validation commands, git/PR actions, and report posture. No implementation, hidden auth loading, automatic provider calls, automatic retries, broad write support, CLI behavior, schemas, examples, hosted behavior, or release posture changes were performed.
