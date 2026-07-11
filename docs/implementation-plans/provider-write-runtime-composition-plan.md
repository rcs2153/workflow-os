# Provider-Write Runtime Composition Plan

## 1. Executive Summary

Workflow OS now has many provider-write-adjacent primitives, but they are intentionally exposed as narrow helpers and explicit paths. The next question is how those primitives should compose into one clear, explicit runtime path for the GitHub PR comment provider-write lane without changing default executor behavior.

This plan has now produced the first explicit in-memory runtime composition
helper for the GitHub PR comment provider-write lane. The implemented helper is
still opt-in and local: it composes existing executor, approval-presentation,
provider-call, reconciliation, workflow-event proof, and report-disclosure
helpers without changing default executor behavior.

Artifact-gated provider-write composition is planned separately in
[Artifact-Gated Provider-Write Composition Plan](artifact-gated-provider-write-composition-plan.md).
That follow-on lane is implemented as an explicit in-memory helper that
composes the provider-write composition result with existing report artifact
gates without making provider writes or artifact writes automatic.

The accepted live sandbox validation helper remains a separate explicit
boundary, and the first helper-level composition path for that boundary is
implemented as an explicit, injected-provider-only runtime helper. Planning is
documented in
[GitHub PR Comment Live Sandbox Runtime Composition Plan](github-pr-comment-live-sandbox-runtime-composition-plan.md)
and implementation is reported in
[GitHub PR Comment Live Sandbox Runtime Composition Helper Report](../concepts/GITHUB_PR_COMMENT_LIVE_SANDBOX_RUNTIME_COMPOSITION_HELPER_REPORT.md).
The helper review is documented in
[GitHub PR Comment Live Sandbox Runtime Composition Helper Review](../concepts/GITHUB_PR_COMMENT_LIVE_SANDBOX_RUNTIME_COMPOSITION_HELPER_REVIEW.md),
and follow-on event-proof composition planning is documented in
[GitHub PR Comment Live Sandbox Event-Proof Composition Plan](github-pr-comment-live-sandbox-event-proof-composition-plan.md).

Automatic provider writes, default executor writes, hidden auth loading, CLI
mutation commands, workflow schema changes, examples, hosted behavior,
reasoning lineage, broad side-effect execution, report artifacts, persistence,
lookup/recovery automation, and release posture changes remain unimplemented.

## 2. Goals

- Define the smallest explicit provider-write runtime composition boundary.
- Compose existing reviewed primitives instead of inventing new governance vocabulary.
- Preserve default local executor behavior.
- Require explicit caller opt-in.
- Keep provider writes behind stable side-effect, approval, and report evidence gates.
- Make provider-call, local-transition, event-proof, report-disclosure, artifact-gate, and recovery posture inspectable.
- Preserve non-leaking, bounded errors and Debug output.
- Prepare the next implementation prompt without authorizing automatic writes.

## 3. Non-Goals

- Further implementation beyond the first explicit in-memory composition helper.
- Automatic provider writes.
- Default executor provider writes.
- Hidden provider or auth loading.
- CLI mutation commands.
- Workflow-declared provider-write config.
- Runtime config.
- Schema or SDK changes.
- Examples.
- Hosted or distributed runtime.
- Generic side-effect execution expansion.
- Write-capable Jira, CI, filesystem, HTTP, or arbitrary adapters.
- Reasoning lineage.
- Recursive agents, agent swarms, or Level 3/4 autonomy.
- Release posture changes.

## 4. Existing Building Blocks

The composition path should use existing reviewed pieces:

- `SideEffectRecord` persistence and store-backed discovery.
- Approval-side-effect linkage validation.
- Explicit no-provider-call write-adapter orchestration for proposed and attempted records.
- Provider-call trait/input model.
- Injected provider-call orchestration helper.
- Concrete injected-transport GitHub PR comment provider client.
- Provider write reconciliation model/helper.
- Executor-integrated live provider-write helper.
- Provider-write completed/failed workflow event append helper.
- Provider reconciliation disclosure helper for WorkReport side-effect posture.
- Report artifact event-proof gate.
- Provider event-proof recovery classifier.
- Provider lookup/query reconciliation helper.
- Provider lookup recovery integration helper.
- Provider lookup operator recovery summary helper and local summary-input CLI.
- Provider-call orchestration gate clarity.
- Approval-presentation opt-in enforcement.
- Provider-write approval-presentation gate and edge hardening tests.

The next implementation should not duplicate these concepts. It should compose them.

## 5. Proposed Runtime Composition Boundary

Add an explicit provider-write composition helper/service that accepts all required inputs and returns an in-memory composition result.

The helper should be local, additive, and opt-in. It should not be called by `LocalExecutor::execute(...)` and should not be reachable from default workflow execution.

Candidate name:

- `compose_github_pr_comment_provider_write_runtime(...)`

Candidate request/result names:

- `GitHubPrCommentProviderWriteRuntimeCompositionRequest`
- `GitHubPrCommentProviderWriteRuntimeCompositionResult`

The implementation should choose repository conventions when scoped.

## 6. Inputs

Required explicit inputs should include:

- local executor or completed/terminal run context, depending on the selected integration point;
- side-effect record store;
- attempted side-effect record or explicit proposed-to-attempted orchestration inputs;
- injected provider implementation;
- provider call input;
- approval linkage policy;
- approval-presentation enforcement policy;
- provider reconciliation policy;
- workflow event append policy;
- WorkReport disclosure inputs or report-generation inputs;
- event-proof/artifact-gate policy if artifact writing remains part of the explicit composition;
- lookup/recovery policy if post-provider reconciliation is requested.

The helper must not discover hidden auth, load tokens, infer provider clients, or fabricate missing stable identifiers.

## 7. Composition Sequence

Recommended conservative sequence:

1. Validate explicit provider-write request shape.
2. Validate or construct the proposed/attempted `SideEffectRecord` through existing helpers.
3. Validate approval-side-effect linkage.
4. Validate approval-presentation proof when the caller requests the write-adjacent gate.
5. Invoke the injected provider only after all pre-provider gates pass.
6. Transition attempted side-effect lifecycle using existing provider-call orchestration.
7. Build reconciliation posture for provider/local agreement or ambiguity.
8. Append completed/failed SideEffect workflow events only when existing eligibility rules allow it.
9. Build bounded WorkReport disclosure posture.
10. Apply event-proof/report-artifact gates only when explicitly requested.
11. If provider/local or event-proof posture is ambiguous, derive recovery posture and next action.

Every step should expose bounded gate clarity.

## 8. Failure Semantics

The helper should fail closed before provider invocation when:

- preflight context is invalid;
- attempted record is missing or invalid;
- approval-side-effect linkage fails;
- approval-presentation proof is missing, mismatched, stale, wrong-posture, or unresolved;
- policy gate denies provider call;
- idempotency binding is invalid;
- provider target/auth shape is invalid.

After provider invocation, failures must not be retried automatically. Post-provider local transition failure must produce explicit ambiguous reconciliation posture and block automatic retry.

Report/artifact failures must not rewrite the original workflow result. They should surface as composition errors or report/artifact posture, depending on the selected result model.

## 9. Gate Clarity

The result should expose gate posture for:

- preflight context;
- attempted record;
- approval linkage;
- approval presentation;
- attempted lifecycle;
- provider call;
- provider response;
- post-provider local transition;
- reconciliation;
- workflow event proof;
- report disclosure;
- artifact event-proof gate;
- operator recovery.

Gate states should remain bounded vocabulary such as `Satisfied`, `Blocked`, `NotEvaluated`, and `NotRequired`.

## 10. Report And Artifact Posture

The first implementation should prefer in-memory report disclosure over artifact writing unless the artifact path is separately scoped.

If artifact writing is included, it must remain explicit and must require:

- valid WorkReport;
- validated SideEffect referential integrity;
- approval linkage where required;
- durable workflow event proof where required;
- proof-marker gate satisfaction where configured;
- non-leaking artifact-store errors.

No automatic report generation or automatic artifact writing should be introduced.

## 11. Provider Lookup And Recovery

Lookup/recovery should remain explicit and caller-supplied.

If the composition result finds ambiguous provider/local agreement, it may return recovery posture and recommended operator action. It must not:

- perform automatic lookup unless an explicit injected lookup client is supplied;
- perform automatic repair;
- append recovery workflow events;
- retry provider writes;
- mutate side-effect records outside reviewed transition helpers.

## 12. Privacy And Redaction

The composition helper must not store or copy:

- provider auth tokens;
- raw provider payloads;
- raw GitHub issue or PR bodies;
- raw command output;
- raw source contents;
- raw spec contents;
- environment variable values;
- approval-presentation payload text;
- secret-like values.

Errors must use stable codes and non-leaking messages. Debug output must remain bounded and redaction-safe.

## 13. Test Plan

Future implementation should add focused tests for:

- all pre-provider gates satisfied invokes injected provider exactly once;
- approval-side-effect linkage failure blocks provider call;
- approval-presentation proof failure blocks provider call;
- stale proof blocks provider call;
- invalid attempted record blocks provider call;
- provider success transitions attempted record to completed;
- provider failure transitions attempted record to failed where eligible;
- post-provider local transition failure produces ambiguous reconciliation and blocks retry;
- completed/failed workflow event append occurs only when eligibility rules pass;
- missing event proof blocks artifact writing when configured;
- WorkReport disclosure is bounded and non-leaking;
- lookup/recovery posture is explicit when provider/local state is ambiguous;
- no hidden auth loading occurs;
- default executor behavior remains unchanged;
- no CLI output or filesystem artifacts are created unless explicitly scoped;
- Debug/serialization do not leak token-like or payload-like values;
- existing provider-write, WorkReport, SideEffect, approval, and runtime tests still pass.

## 14. Proposed Implementation Sequence

1. Add a pure composition request/result model around existing helpers, with no new provider behavior. Implemented by `compose_github_pr_comment_provider_write_runtime(...)` and `GitHubPrCommentProviderWriteRuntimeCompositionRequest` / `GitHubPrCommentProviderWriteRuntimeCompositionResult`.
2. Compose pre-provider gates, injected provider invocation, reconciliation, eligible lifecycle event append, and bounded WorkReport disclosure. Implemented through delegation to the existing proof-gated provider-write helper.
3. Defer artifact writing, lookup/recovery composition, CLI behavior, schema behavior, examples, default executor behavior, hosted behavior, and broader provider writes unless separately approved.
4. Review the composition helper before widening runtime exposure.
5. Keep automatic/default provider writes deferred until approval, side-effect, artifact, and recovery posture are materially stronger.

## 15. Open Questions

- Should the first implementation accept a pre-existing terminal run, or should it wrap executor execution like the current provider-write helper?
- Should artifact writing stay out of the first composition slice?
- Should lookup/recovery be returned as optional posture only, or composed when an injected lookup client is supplied?
- Should provider-write composition expose one result type or nested sub-results from each helper?
- How much gate clarity should be duplicated versus referenced from existing result types?
- What is the smallest slice that proves runtime composition without making provider writes feel default?

## 16. Final Recommendation

Proceed next with a narrow explicit provider-write runtime composition helper, in memory first, using injected provider/client inputs and existing reviewed helpers.

Do not build automatic provider writes, default executor integration, hidden auth loading, CLI mutation behavior, schemas, examples, hosted behavior, generic write-capable adapters, reasoning lineage, or release posture changes in that implementation phase.
