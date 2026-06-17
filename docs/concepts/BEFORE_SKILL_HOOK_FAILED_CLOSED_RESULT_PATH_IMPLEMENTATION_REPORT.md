# BeforeSkillInvocation Failed-Closed Result Path Implementation Report

## 1. Executive Summary

The first narrow `BeforeSkillInvocation` failed-closed result path is implemented. Workflow OS can now construct a validated in-memory `FailedClosed` agent harness hook result from explicit inputs, append bounded `HookInvocationRequested` and `HookInvocationEvaluated` events for an explicitly supplied `BeforeSkillInvocation` checkpoint, and fail the run before `SkillInvocationRequested`.

`Passed` remains the only continuing hook status. `FailedClosed` is a blocking status only. Warning continuation, skipped-with-disclosure continuation, blocked runtime behavior, automatic hook invocation, workflow-declared hook configuration, runtime hook configuration, persistence, CLI behavior, schemas, local check execution, command execution, adapter invocation, approvals, evidence attachment, side effects, writes, reasoning lineage, hosted behavior, and release posture changes remain unimplemented.

## 2. Scope Completed

- Added an explicit failed-closed in-memory hook invocation helper.
- Added an explicit failed-closed runtime hook helper that returns a hook invocation result plus model-only hook audit record.
- Added `LocalExecutionBeforeSkillInvocationHookInput::result_status` so explicit test/request inputs can choose `Passed` or `FailedClosed`.
- Updated `LocalExecutor::execute(...)` to handle explicit `FailedClosed` hook input for the active `BeforeSkillInvocation` target.
- Constructed requested/evaluated hook workflow event payloads before appending either event.
- Appended requested/evaluated hook events before failing the scoped run on `FailedClosed`.
- Prevented `SkillInvocationRequested`, skill attempts, retries, handler invocation, and report artifact creation for the failed-closed path.
- Preserved idempotent replay behavior for duplicate failed-closed runs.
- Kept warning, skipped-with-disclosure, and blocked statuses unsupported in this executor phase.
- Added focused model/runtime helper tests and local executor tests.
- Updated roadmap and concept documentation.

## 3. Scope Explicitly Not Completed

- No warning continuation.
- No skipped-with-disclosure continuation.
- No blocked runtime behavior.
- No automatic hook invocation.
- No workflow-declared hook configuration.
- No runtime hook configuration.
- No post-terminal workflow events.
- No dedicated hook audit sink emission.
- No hook persistence.
- No hook observability metrics.
- No WorkReport hook event citation targets.
- No CLI behavior.
- No workflow schema fields.
- No automatic local check execution.
- No command execution.
- No adapter invocation.
- No external provider calls.
- No approval request or approval decision creation.
- No approval evidence attachment.
- No `EvidenceReference` creation or attachment.
- No report artifact writes.
- No reasoning lineage.
- No side-effect boundary implementation.
- No writes.
- No recursive agents or agent swarms.
- No hosted or distributed runtime claims.
- No release posture changes.

## 4. Model And Helper API Summary

The hook helper layer now exposes:

- `invoke_agent_harness_hook(...)` for validated `Passed` results.
- `invoke_agent_harness_hook_failed_closed(...)` for validated `FailedClosed` results.
- `execute_runtime_agent_harness_hook(...)` for an in-memory passed runtime result plus model-only audit record.
- `execute_runtime_agent_harness_hook_failed_closed(...)` for an in-memory failed-closed runtime result plus model-only audit record.

Both failed-closed helpers reuse the existing validation boundary for contract identity, hook kind, workflow/run/schema/spec identity, target step context, required references, disclosures, redaction metadata, and side-effect rejection. Unsupported helper statuses remain rejected with stable non-leaking errors.

## 5. Executor Behavior Summary

`LocalExecutor::execute(...)` now supports explicit `BeforeSkillInvocation` hook inputs with:

- `Passed`: append `HookInvocationRequested`, append `HookInvocationEvaluated(Passed)`, then continue to skill invocation.
- `FailedClosed`: construct a safe failed-closed runtime result, construct both hook event payloads, append requested/evaluated events, then fail the run with `executor.hook.before_skill_invocation.failed_closed` before skill invocation.
- `Warning`, `SkippedWithDisclosure`, and `Blocked`: reject as unsupported for this phase with `executor.hook.before_skill_invocation.unsupported_status` and append no hook or skill events.

The failed-closed failure message is stable and generic:

```text
before-skill-invocation hook failed closed before skill invocation
```

## 6. Event Ordering And Idempotency Summary

The failed-closed event order is:

1. policy decision event;
2. `HookInvocationRequested`;
3. `HookInvocationEvaluated(FailedClosed)`;
4. `RunFailed`.

No `SkillInvocationRequested`, `SkillInvocationStarted`, `SkillInvocationSucceeded`, `SkillInvocationFailed`, or `RetryScheduled` events are appended for the scoped step. Duplicate execution with the same run ID rehydrates the existing failed run and does not duplicate hook events.

## 7. Redaction And Privacy Summary

The implementation does not copy raw provider payloads, raw command output, raw CI logs, raw Jira/GitHub bodies, raw spec contents, raw parser payloads, environment values, credentials, authorization headers, private keys, token-like values, evidence payloads, local check results, approval decisions, or WorkReport content.

Failed-closed executor errors use stable codes and generic messages. Tests assert that failure messages do not leak hook references or scoped checkpoint identifiers.

## 8. Test Coverage Summary

Added focused tests for:

- valid failed-closed hook invocation helper results;
- failed-closed helper side-effect rejection without leakage;
- failed-closed runtime helper results and audit records;
- failed-closed runtime helper side-effect rejection without leakage;
- failed-closed executor event order and run failure before skill invocation;
- no skill handler invocation for failed-closed hooks;
- no report artifact creation for failed-closed hooks;
- duplicate failed-closed run replay without duplicate hook events;
- unsupported warning status producing no hook or skill events.

Existing boundary coverage continues to cover later-step targeting, missing handlers, policy denial, duplicate run replay, no-hook execution, and debug redaction.

## 9. Commands Run And Results

- `cargo test -p workflow-core --test agent_harness_hook_invocation failed_closed`: passed.
- `cargo test -p workflow-core --test local_executor before_skill_hook`: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.

## 10. Remaining Known Limitations

- `Passed` remains the only continuing hook status.
- `FailedClosed` is blocking-only and fails the run before skill invocation.
- Warning/skipped continuation requires separate disclosure, report, and policy semantics.
- `Blocked` requires a separately accepted runtime status or escalation model.
- Retry-aware hook execution remains deferred.
- Dedicated hook audit sink emission remains deferred.
- WorkReport hook event citation targets remain deferred.
- Automatic hook configuration remains deferred.

## 11. Recommended Next Phase

Recommended next phase at implementation close: **BeforeSkillInvocation failed-closed result path review**.

The review should verify scope cleanliness, event ordering, idempotency, redaction safety, unsupported status behavior, and the absence of automatic hook configuration, persistence, CLI behavior, schemas, command execution, adapter invocation, approvals, evidence creation, side effects, writes, reasoning lineage, hosted behavior, and release posture changes.

Fix-forward note: that review is complete, and warning/skipped disclosure semantics planning is now documented in [BeforeSkillInvocation Warning And Skipped Disclosure Semantics Plan](../implementation-plans/before-skill-hook-warning-skipped-disclosure-plan.md).
