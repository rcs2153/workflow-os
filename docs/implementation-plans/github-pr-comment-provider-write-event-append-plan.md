# GitHub PR Comment Provider Write Event Append Plan

Status: Planning accepted and first helper implemented in [GitHub PR Comment Provider Write Event Append Helper Report](../concepts/GITHUB_PR_COMMENT_PROVIDER_WRITE_EVENT_APPEND_HELPER_REPORT.md). Reconciliation-aware report/artifact disclosure after provider-write event append is planned in [GitHub PR Comment Provider Reconciliation Report Artifact Disclosure Plan](github-pr-comment-provider-reconciliation-report-artifact-disclosure-plan.md). This follows the accepted [Executor-Integrated Live Provider Write Blocker Fix Review](../concepts/EXECUTOR_INTEGRATED_LIVE_PROVIDER_WRITE_BLOCKER_FIX_REVIEW.md) and the implemented explicit [Executor SideEffect Lifecycle Event Append Plan](executor-side-effect-lifecycle-event-append-plan.md). It defines how reviewed GitHub pull request comment provider-write outcomes may be projected into SideEffect lifecycle workflow events. The implemented helper is explicit, local, GitHub PR comment-only, and appends completed/failed SideEffect workflow events only for reconciled provider outcomes. It does not implement default provider writes, hidden auth loading, automatic retries, report artifacts, CLI behavior, schemas, examples, hosted behavior, broader adapter support, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes.

## 1. Executive Summary

Workflow OS now has a reviewed executor-integrated GitHub PR comment provider-write helper. It can invoke an explicitly supplied provider, preserve classified provider responses, transition local SideEffect lifecycle state, and produce reconciliation candidates when provider and local state disagree.

Workflow OS also has an explicit local executor path for appending attempted, completed, and failed SideEffect lifecycle workflow events from validated lifecycle transition results.

The first helper composes those two boundaries narrowly. It appends a completed or failed SideEffect lifecycle workflow event only after a classified provider outcome, a successful local lifecycle transition, and an eligible reconciliation posture agree.

The implementation remains opt-in and does not make default executor provider writes automatic.

## 2. Goals

- Append provider-write completed/failed workflow events only after provider outcome, local lifecycle transition, and reconciliation posture agree.
- Preserve `SideEffectRecordStore` lifecycle state as the local source of truth.
- Preserve provider references as bounded external outcome references, not local lifecycle state by themselves.
- Preserve existing `LocalExecutor::execute(...)` behavior.
- Keep the event append path explicit and opt-in.
- Use stable identifiers and existing SideEffect lifecycle event constructors.
- Avoid raw provider payloads, comment bodies, command output, parser payloads, paths, credentials, tokens, and secret-like strings.
- Keep retry posture fail-closed when provider/local state is ambiguous.
- Prepare WorkReport and report-artifact citation paths without writing artifacts in this phase.

## 3. Non-Goals

Do not implement or authorize:

- implementation in this planning phase;
- default executor provider writes;
- automatic provider calls;
- hidden auth loading from environment, keychains, GitHub CLI, git remotes, config files, OAuth, or secret managers;
- automatic retries;
- provider lookup/query reconciliation;
- broad GitHub write support;
- non-comment GitHub mutations;
- Jira, CI, or other provider writes;
- report artifact writing;
- report persistence;
- CLI mutation commands, rendering, or export;
- workflow schema fields;
- examples;
- hosted or distributed runtime behavior;
- enterprise RBAC, IdP, quorum approval, or revocation enforcement;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- release posture changes.

## 4. Current Baseline

Implemented and reviewed:

- GitHub PR comment write request/response model;
- write-adapter preflight helpers;
- proposed GitHub PR comment SideEffect record composition and persistence;
- approval-side-effect linkage;
- store-backed attempted/completed/failed lifecycle transitions;
- explicit executor SideEffect lifecycle event append input for attempted/completed/failed events;
- provider-call trait/input and injected-provider orchestration helper;
- concrete injected-transport GitHub PR comment provider client;
- provider write reconciliation model/helper;
- executor-integrated live provider-write request/result/helper;
- blocker fix preserving classified provider response context when local transition fails after provider call.

Still missing:

- audit projection wiring for provider-write event outcomes beyond existing generic SideEffect event projection;
- reconciliation-aware report/artifact disclosure from provider outcomes;
- operator recovery workflow for ambiguous provider/local states;
- hidden auth loading, retries, CLI behavior, schemas, examples, hosted behavior, and broader adapters.

## 5. Event Append Boundary

Provider-write event append should be a separate explicit composition boundary.

The implementation does not make `LocalExecutor::execute(...)` append provider outcome events automatically. It extends the explicit provider-write helper path and appends a lifecycle event only when the outcome is eligible.

Event append must not perform provider calls. It must not load auth. It must not infer provider success from local state alone. It must not convert ambiguous provider/local states into completed or failed lifecycle events.

## 6. Eligible Outcomes

Only two outcome classes are eligible for provider-write lifecycle event append:

- `provider_succeeded_local_completed`: provider returned classified success, local SideEffect lifecycle transition to completed succeeded, and reconciliation candidate construction succeeded.
- `provider_failed_local_failed`: provider returned classified failure, local SideEffect lifecycle transition to failed succeeded, and reconciliation candidate construction succeeded.

All other outcomes are ineligible for completed/failed append:

- provider not called;
- provider response ambiguous;
- provider succeeded but local completed transition failed;
- provider failed but local failed transition failed;
- local transition failure without classified provider response;
- reconciliation construction failure;
- missing provider response;
- missing transition result;
- mismatched SideEffect, run, step, skill, or idempotency identity.

Ineligible outcomes should return explicit structured posture and append no lifecycle event.

## 7. Ordering Policy

The future implementation should preserve this order:

1. Execute the explicit local workflow path selected by the caller.
2. Persist the proposed SideEffect record through the reviewed boundary.
3. Append proposed and attempted events only through existing explicit append paths when selected by the helper.
4. Invoke only the caller-supplied provider.
5. Classify provider response or provider-write error.
6. Persist the local completed or failed SideEffect lifecycle transition when a classified provider response exists.
7. Build reconciliation candidate from provider response and local transition result.
8. Append completed or failed lifecycle event only when the reconciliation status is eligible.
9. Return an in-memory result disclosing provider, transition, reconciliation, and event append posture.

Do not append a completed/failed provider-write lifecycle event before provider classification, local transition success, and reconciliation classification.

## 8. Idempotency And Replay

Provider-write event append must be deterministic and idempotent.

Recommended idempotency key ingredients:

- workflow run ID;
- workflow ID and version;
- schema version;
- spec hash;
- step ID;
- skill ID and version;
- SideEffect ID;
- SideEffect lifecycle state;
- provider kind;
- target kind;
- provider-write idempotency key;
- append checkpoint name.

Duplicate helper execution must not append duplicate lifecycle events. If the event already exists for the same governed checkpoint and same lifecycle transition, the future implementation should return a stable already-appended posture or a bounded no-op result rather than creating another event.

Replay semantics for provider calls remain separate. Event append idempotency must not authorize a second provider call.

## 9. Failure Behavior

Conservative failure rules:

- Pre-provider failures append no provider outcome event.
- Provider-call ambiguity appends no completed/failed event.
- Local transition failure after provider success/failure appends no completed/failed event.
- Reconciliation construction failure appends no completed/failed event.
- Event append failure after provider/local/reconciliation success must not re-call the provider.
- Event append failure must return a structured non-leaking error and disclose that local lifecycle state and workflow event history may be out of sync.
- Report/artifact generation or writing must not be triggered by this path.

Errors must not leak raw provider payloads, comment bodies, URLs with private context, auth material, headers, idempotency keys, side-effect IDs, paths, command output, parser payloads, or secret-like values.

## 10. Result Model Considerations

The future result shape should distinguish:

- workflow execution status;
- provider-call status;
- local SideEffect lifecycle status;
- reconciliation status;
- lifecycle event append status;
- retry blocked posture;
- operator action required posture.

The result should expose bounded accessors and an `into_parts()` shape consistent with existing executor result types. Debug output must redact report text, provider references, auth context, side-effect IDs, idempotency keys, paths, and caller-supplied metadata.

## 11. Relationship To Audit And Reports

Workflow events are append-only history projections. Audit projection should continue to derive from validated workflow events, not from provider responses directly.

WorkReports and report artifacts should cite:

- SideEffect IDs;
- workflow event IDs;
- provider references when bounded and validated;
- reconciliation status when relevant.

They should not copy provider payloads or comment bodies.

This phase does not authorize report artifact writing. A later artifact phase may validate that completed/failed SideEffect lifecycle records have matching workflow events before writing an artifact.

## 12. Privacy And Redaction

The implementation must preserve the existing redaction posture:

- no raw GitHub response bodies;
- no raw comment bodies;
- no authorization headers;
- no tokens;
- no private keys;
- no environment values;
- no command output;
- no parser payloads;
- no raw spec contents;
- no unbounded provider references;
- no secret-like redaction metadata.

Provider references must be bounded, validated, and treated as sensitive. Error messages should use stable codes and generic messages.

## 13. Test Plan

Implemented tests cover:

- provider success plus completed local transition appends exactly one completed lifecycle event;
- provider failure plus failed local transition appends exactly one failed lifecycle event;
- provider not called appends no provider outcome event;
- provider response ambiguity appends no provider outcome event;
- provider success plus local completed transition failure appends no completed event and blocks retry;
- provider failure plus local failed transition failure appends no failed event and blocks retry;
- reconciliation construction failure appends no event and returns non-leaking error;
- event append result carries bounded event posture;
- default `LocalExecutor::execute(...)` remains unchanged;
- no raw provider payload, comment body, auth value, command output, parser payload, path, token, or secret-like value appears in Debug output, serialization, or errors;
- existing provider-write, SideEffect lifecycle append, report artifact, WorkReport, approval, validation, adapter telemetry, runtime, and CLI tests continue to pass.

Deferred follow-up tests should cover:

- event append failure does not re-call provider;
- duplicate helper replay does not append duplicate events.

## 14. Proposed Implementation Sequence

Recommended future phases:

1. Provider write event append plan review.
2. Add a narrow explicit provider-write event append helper or extend the existing executor-integrated provider-write helper result path. Completed in [GitHub PR Comment Provider Write Event Append Helper Report](../concepts/GITHUB_PR_COMMENT_PROVIDER_WRITE_EVENT_APPEND_HELPER_REPORT.md).
3. Append completed/failed lifecycle events only for eligible reconciled outcomes.
4. Add focused idempotency, failure, non-leakage, and default-executor regression tests.
5. Review the implementation before artifact/report integration.
6. Plan reconciliation-aware report artifact disclosure only after event append is accepted.

## 15. Deferred Work

Deferred:

- provider lookup/query reconciliation;
- automatic retry policy;
- operator recovery workflow;
- hidden auth loading;
- automatic event append from default execution;
- report artifact writing;
- report persistence;
- CLI rendering or mutation commands;
- workflow schema fields;
- examples;
- hosted runtime behavior;
- broader provider write adapters;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 16. Final Recommendation

Next implementation phase should be: **explicit GitHub PR comment provider-write event append helper, local and opt-in only**.

It should append completed/failed SideEffect lifecycle workflow events only for reconciled provider outcomes, preserve retry-blocked posture for ambiguity, and keep report artifacts, CLI behavior, schemas, examples, hosted behavior, hidden auth loading, automatic retries, broader writes, and release posture changes out of scope.

## 17. Governed Dogfood Summary

- workflow: `dg/d`;
- phase: planning;
- run ID: `run-1783291068869685000-2`;
- approval ID: `approval/run-1783291068869685000-2/planning-approved`;
- approval reason: `delegated-maintainer-approved-provider-write-event-append-planning`;
- approval outcome: granted by delegated maintainer.
- phase close status: completed.
- event summary: 39 total events; 1 approval; 0 retries; 0 escalations.
- event kinds: `ApprovalGranted`, `ApprovalRequested`, `PolicyDecisionRecorded`, `RunCompleted`, `RunCreated`, `RunResumed`, `RunStarted`, `RunValidated`, `SkillInvocationRequested`, `SkillInvocationStarted`, `SkillInvocationSucceeded`, and `StepScheduled`.

Approved scope: create a planning document for explicit event append from reconciled provider outcomes without implementation.

Strict non-goals: no implementation, provider writes, hidden auth, retries, artifact writes, CLI behavior, schemas, examples, hosted behavior, or release posture changes.

Out-of-kernel work disclosed: repository document inspection, planning document creation, roadmap/status updates, docs validation, and git/PR actions are performed by the agent outside kernel execution. No provider write, hidden auth loading, retry, event append implementation, artifact write, CLI behavior, schema/example update, hosted behavior, or release posture change is performed by the kernel.
