# GitHub PR Comment Provider Write Reconciliation Plan

Status: Accepted plan; reviewed in [GitHub PR Comment Provider Write Reconciliation Plan Review](../concepts/GITHUB_PR_COMMENT_PROVIDER_WRITE_RECONCILIATION_PLAN_REVIEW.md). This follows the accepted [GitHub PR Comment Provider Client/Auth Loading Implementation Review](../concepts/GITHUB_PR_COMMENT_PROVIDER_CLIENT_AUTH_LOADING_IMPLEMENTATION_REVIEW.md). It defines the reconciliation boundary required before any executor-integrated live GitHub pull request comment write path.

This plan does not implement reconciliation, executor writes, provider calls, hidden auth loading, automatic event append, report artifact writes, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes.

## 1. Executive Summary

Workflow OS now has a concrete GitHub PR comment provider client with explicit caller-supplied auth and injected transport only.

That client can classify bounded provider responses, but executor-integrated live writes still need a reconciliation design before they are safe. The hard case is not ordinary success or ordinary failure. The hard case is ambiguity: GitHub may have created a comment, but Workflow OS may fail before local side-effect lifecycle state, workflow events, audit candidates, or report citations are updated.

This plan defines how Workflow OS should treat remote-success/local-transition-failure and other ambiguous write outcomes. It keeps provider writes explicit, idempotency local, evidence/reference-first, and fail-closed when reconciliation state is insufficient.

## 2. Goals

- Define the reconciliation boundary before executor-integrated live writes.
- Preserve local deterministic state as the Workflow OS source of truth.
- Preserve provider references as external outcome evidence, not hidden state mutations.
- Avoid duplicate GitHub comments after ambiguous outcomes.
- Define remote-success/local-transition-failure behavior.
- Define transport ambiguity behavior.
- Define retry posture and idempotency limits.
- Define report, audit, and WorkReport disclosure obligations.
- Keep errors stable, bounded, and redaction-safe.
- Prepare a small implementation prompt for model/helper-only reconciliation.

## 3. Non-Goals

Do not implement or authorize:

- implementation in this planning phase;
- executor-integrated live writes;
- automatic provider calls;
- automatic retries;
- hidden auth loading from environment variables, keychains, GitHub CLI state, git remotes, config files, OAuth, or secret managers;
- automatic workflow event append;
- automatic audit or observability emission;
- report artifact writing;
- CLI mutation commands or flags;
- workflow schema fields;
- examples;
- hosted or distributed runtime behavior;
- broad GitHub write support;
- non-comment GitHub mutations;
- Jira or other provider writes;
- provider-native idempotency claims that GitHub does not guarantee;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- release posture changes.

## 4. Current Baseline

Implemented and reviewed:

- GitHub PR comment request and response models.
- Write preflight validation.
- Proposed `SideEffectRecord` composition and persistence.
- Approval-side-effect linkage.
- Store-backed attempted/completed/failed lifecycle transitions.
- Executor lifecycle event append helpers.
- No-provider attempted and completed/failed orchestration helpers.
- Provider-call request model and injected provider trait.
- Injected-provider orchestration helper.
- Concrete `GitHubPullRequestCommentHttpProvider` with injected transport.

Still missing:

- reconciliation model for ambiguous provider outcomes;
- durable reconciliation record or candidate model;
- safe retry policy;
- executor-integrated live write path;
- automatic workflow event/audit/report projection from provider outcomes;
- operator-facing recovery guidance;
- provider-native lookup/query helper.

## 5. Reconciliation Problem Statement

Provider writes create a split-brain risk:

- GitHub can accept the write and return a comment ID.
- Workflow OS can then fail while transitioning the local `SideEffectRecord` to `Completed`.
- Workflow OS can transition local state but fail before appending a workflow event.
- Workflow OS can lose a transport response after GitHub created a comment.
- A retry can create a duplicate comment if provider-native idempotency is unavailable.

The system must never hide this ambiguity, fabricate success, or retry blindly.

## 6. Outcome Classes

Future reconciliation should classify provider-write outcomes into explicit classes:

- `provider_not_called`: no provider call was attempted.
- `provider_succeeded_local_completed`: provider success and local completed transition both succeeded.
- `provider_failed_local_failed`: provider returned classified failure and local failed transition succeeded.
- `provider_succeeded_local_transition_failed`: provider success was observed but local completed transition failed.
- `provider_failed_local_transition_failed`: classified provider failure was observed but local failed transition failed.
- `provider_response_ambiguous`: provider call returned no reliable success/failure classification.
- `local_state_ambiguous`: local state cannot prove whether a transition was applied.
- `reconciliation_required`: operator or future helper must resolve before retry or completion.

These classes should be represented by stable vocabulary, not raw provider payloads.

## 7. Source Of Truth Boundaries

Workflow OS should maintain clear boundaries:

- `SideEffectRecordStore` remains the local lifecycle source of truth.
- Provider references are external outcome references, not local lifecycle state by themselves.
- Workflow events are append-only audit/history projections, not provider call execution.
- Report artifacts cite side effects and provider references, but do not reconcile state by themselves.
- Adapter telemetry can disclose attempted calls and classified outcomes, but does not mutate lifecycle state directly.

If these disagree, the disagreement must be explicit and inspectable.

## 8. Remote-Success Local-Failure Policy

If GitHub returns a success response with a bounded provider comment reference but the local completed transition fails:

- do not retry the provider call automatically;
- do not create another comment;
- return a structured reconciliation-required result;
- preserve the provider reference in a bounded reconciliation candidate if the model supports it;
- disclose that the remote provider may have succeeded while local state did not complete;
- require a later explicit reconciliation helper or operator action to complete local state from the known provider reference;
- avoid raw provider response bodies, headers, tokens, comment bodies, or URLs in errors.

The recommended posture is fail-closed for subsequent automatic writes with the same idempotency key until reconciliation is resolved.

## 9. Provider-Failure Local-Failure Policy

If GitHub returns a classified provider failure but the local failed transition cannot be persisted:

- do not retry automatically;
- return a reconciliation-required result;
- preserve only stable provider error code and bounded reference metadata;
- disclose that provider failure was observed but local lifecycle closure failed;
- require a later explicit reconciliation path before a new attempt.

This prevents the system from treating a known provider failure as if no call happened.

## 10. Transport Ambiguity Policy

If the transport returns an ambiguous error and no reliable provider success/failure response exists:

- do not transition to completed;
- do not fabricate failed provider outcome unless the failure is classified;
- do not retry automatically;
- mark the outcome as `provider_response_ambiguous`;
- require explicit operator or future lookup/reconciliation before another provider call with the same idempotency key;
- provide stable non-leaking error code such as `github_pr_comment_reconciliation.provider_response_ambiguous`.

The first implementation should not query GitHub to resolve ambiguity unless a separate provider lookup plan is accepted.

## 11. Retry And Idempotency Policy

GitHub PR comment creation should be treated as lacking reliable provider-native idempotency for v0.

Rules:

- local idempotency key remains mandatory;
- one live provider call is allowed per attempted side-effect/idempotency key unless reconciliation clears it;
- automatic retries are not allowed in the first reconciliation implementation;
- manual retry requires explicit reconciliation status and should not reuse ambiguous local state silently;
- retry attempts must be auditable and cite the prior ambiguous result.

## 12. Reconciliation Model Recommendation

Recommended first implementation: a model/helper-only reconciliation candidate, not executor integration.

Candidate types may include:

- `ProviderWriteReconciliationStatus`;
- `ProviderWriteReconciliationOutcome`;
- `ProviderWriteReconciliationCandidate`;
- `ProviderWriteReconciliationReference`;
- `ProviderWriteReconciliationErrorCode`.

The model should capture:

- side-effect ID;
- idempotency key;
- provider kind;
- target kind;
- local lifecycle state observed;
- provider outcome class;
- bounded provider reference when known;
- bounded provider error code when known;
- whether retry is blocked;
- whether operator action is required;
- sensitivity;
- redaction metadata.

It must not store raw provider payloads, comment bodies, auth material, headers, command output, environment values, or token-like values.

## 13. Error Handling

Errors must use stable codes and avoid raw values.

Candidate stable codes:

- `github_pr_comment_reconciliation.provider_not_called`;
- `github_pr_comment_reconciliation.remote_success_local_transition_failed`;
- `github_pr_comment_reconciliation.remote_failure_local_transition_failed`;
- `github_pr_comment_reconciliation.provider_response_ambiguous`;
- `github_pr_comment_reconciliation.local_state_ambiguous`;
- `github_pr_comment_reconciliation.retry_blocked`;
- `github_pr_comment_reconciliation.invalid_provider_reference`;
- `github_pr_comment_reconciliation.invalid_reconciliation_input`.

Errors must not include:

- provider response bodies;
- comment bodies;
- URLs containing private repository identity when avoidable;
- auth values;
- headers;
- request bodies;
- raw spec contents;
- command output;
- stack traces;
- environment variable values;
- token-like strings.

## 14. Report, Audit, And Event Disclosure

Future reconciliation should be disclosed without pretending it is a normal success/failure.

Report/audit candidates should cite:

- side-effect ID;
- idempotency key reference;
- provider outcome class;
- bounded provider reference if known;
- bounded provider error code if known;
- local lifecycle state observed;
- reconciliation requirement;
- retry blocked/unblocked posture;
- operator action required.

Workflow event append remains a separate reviewed boundary. The reconciliation helper may return event candidates, but should not append events in the first implementation.

## 15. Privacy And Redaction

Reconciliation must stay reference-first.

Do not store or output:

- raw GitHub responses;
- raw request bodies;
- comment bodies;
- provider headers;
- authorization headers;
- tokens or credentials;
- private keys;
- environment values;
- CI logs;
- command output;
- parser payloads;
- raw spec contents;
- provider payloads.

Debug, serialization, deserialization errors, WorkReport candidates, audit candidates, and validation errors must remain redaction-safe.

## 16. Test Plan

Future implementation tests should cover:

- remote success plus local completed transition success is normal success;
- remote success plus local transition failure returns reconciliation-required result;
- classified remote failure plus local failed transition success is normal failure;
- classified remote failure plus local transition failure returns reconciliation-required result;
- transport ambiguity blocks automatic retry;
- retry is blocked for ambiguous idempotency key;
- known provider reference is bounded and validated;
- provider error code is stable and bounded;
- no raw provider payload is copied;
- no comment body is copied;
- no auth/header/token-like value leaks through Debug, serialization, deserialization errors, or validation errors;
- helper does not append workflow events;
- helper does not write report artifacts;
- helper does not call providers;
- existing provider-write tests continue to pass.

## 17. Proposed Implementation Sequence

Recommended small phases:

1. Provider write reconciliation plan review.
2. Model/helper-only reconciliation candidate implementation.
3. Reconciliation model review.
4. Event/audit projection planning for reconciliation-required outcomes.
5. Report/WorkReport disclosure planning for reconciliation-required outcomes.
6. Executor-integrated live write planning only after reconciliation model review.
7. Executor-integrated live write implementation, opt-in only, after planning review.

## 18. Open Questions

- Should the reconciliation candidate be persisted in the `SideEffectRecordStore`, or remain a returned model until a later store-backed phase?
- Should a future GitHub lookup helper query for comments by known provider reference or search marker?
- Should comment bodies include hidden local idempotency markers, or is that too invasive for provider output?
- Should ambiguous provider responses block all later writes to the same PR, or only the same side-effect/idempotency key?
- Should report artifacts be allowed when reconciliation is required, or only after reconciliation is resolved?
- What minimum operator recovery command is needed before CLI mutation behavior is allowed?

## 19. Final Recommendation

Proceed next to **provider write reconciliation plan review**.

After review, the next implementation should be a model/helper-only reconciliation candidate. It should not call GitHub, integrate with the executor, append workflow events, write report artifacts, load hidden auth, add CLI behavior, add schemas or examples, or change release posture.
