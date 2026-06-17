# BeforeSkillInvocation Warning And Skipped Disclosure Semantics Plan

Status: Planning complete; unsupported-status hardening tests implemented; hook disclosure model planning complete in [Hook Disclosure Model Plan](hook-disclosure-model-plan.md). This plan follows the accepted explicit `BeforeSkillInvocation` executor hook event append path, boundary hardening review, and failed-closed result path review. It does not implement warning continuation, skipped-with-disclosure continuation, blocked runtime behavior, automatic hook invocation, workflow-declared hook configuration, runtime hook configuration, dedicated hook audit sink emission, hook persistence, observability metrics, CLI behavior, schemas, local check execution, command execution, adapter invocation, approvals, evidence attachment, side effects, writes, recursive agents, agent swarms, hosted behavior, or release posture changes.

## 1. Executive Summary

Workflow OS now has a bounded explicit `BeforeSkillInvocation` hook path:

- `Passed` can continue execution.
- explicit `FailedClosed` can append requested/evaluated hook events and fail the run before `SkillInvocationRequested`.
- `Warning`, `SkippedWithDisclosure`, and `Blocked` remain unsupported in the executor path.

The next question is whether warning or skipped hook statuses should ever continue execution. This plan recommends **not** implementing continuation yet. Warning and skipped statuses need a disclosure model, optionality model, policy decision boundary, report/audit semantics, and replay behavior before any runtime continuation can be safe.

The immediate next implementation should be model/test hardening only: keep warning and skipped unsupported, add focused tests for non-leaking rejection and no event append behavior where gaps remain, and define future disclosure requirements before runtime behavior broadens.

## 2. Goals

- Define safe semantics for future warning and skipped hook statuses.
- Preserve deterministic workflow execution.
- Prevent non-passed statuses from silently continuing.
- Avoid fake evidence, fake approvals, fake policy decisions, fake local check results, fake WorkReports, and fake side effects.
- Require bounded, redaction-safe disclosures before warning/skipped status can become durable.
- Require policy control before warning/skipped continuation can affect execution.
- Preserve `Passed` behavior.
- Preserve `FailedClosed` blocking behavior.
- Preserve no-hook, policy-denial, missing-handler, later-step targeting, and replay behavior.
- Prepare a small future implementation prompt.

## 3. Non-Goals

Do not implement in this phase:

- runtime code changes;
- warning continuation;
- skipped-with-disclosure continuation;
- blocked runtime behavior;
- automatic hook invocation;
- broad executor hook checkpoints;
- workflow-declared hook configuration;
- runtime hook configuration;
- post-terminal workflow events;
- conversion of `BeforeReport` into workflow events;
- dedicated hook audit sink emission;
- hook audit store or persistence;
- hook observability metrics;
- WorkReport hook event citation targets;
- CLI hook commands or rendering;
- workflow schema fields;
- automatic local check execution;
- command execution;
- adapter invocation;
- external provider calls;
- `EvidenceReference` creation or attachment;
- approval request or approval decision creation;
- approval evidence attachment;
- report artifact writes;
- reasoning lineage;
- side-effect boundary implementation;
- writes;
- recursive agents or agent swarms;
- hosted or distributed runtime claims;
- release posture changes.

## 4. Current Baseline

Current implemented behavior:

- `LocalExecutionRequest` can carry one explicit `BeforeSkillInvocation` hook input.
- The executor considers that input only when the active step ID matches.
- Policy is evaluated before hook append.
- Local skill handler lookup happens before hook append.
- `Passed` appends `HookInvocationRequested`, appends `HookInvocationEvaluated(Passed)`, and continues to skill invocation.
- explicit `FailedClosed` appends `HookInvocationRequested`, appends `HookInvocationEvaluated(FailedClosed)`, and fails the run before `SkillInvocationRequested`.
- `Warning`, `SkippedWithDisclosure`, and `Blocked` are rejected as unsupported and append no hook or skill events.
- Duplicate execution with the same run ID rehydrates existing events and does not duplicate hook events.

## 5. Status Semantics Recommendation

### Warning

Recommended current behavior: continue rejecting as unsupported.

Future warning continuation may become safe only if all of the following exist:

- bounded warning disclosure text;
- disclosure redaction validation;
- a durable event representation for warning status;
- a policy decision that explicitly allows warning continuation;
- WorkReport disclosure rules;
- replay/idempotency behavior;
- tests proving warnings do not bypass approval, validation, local checks, policy gates, or side-effect boundaries.

Warning must never mean "the model had a concern but execution continued because the agent said so." It must be a governed runtime status with explicit policy and disclosure.

### SkippedWithDisclosure

Recommended current behavior: continue rejecting as unsupported.

Future skipped-with-disclosure continuation may become safe only for optional hooks and only if all of the following exist:

- explicit hook optionality;
- bounded skip disclosure text;
- disclosure redaction validation;
- durable event representation for skipped status;
- policy decision that allows skipping that hook;
- WorkReport disclosure rules;
- replay/idempotency behavior;
- tests proving skip does not fabricate a passed hook.

Skipped must not mean "the hook failed to run but execution continued invisibly." A skip must be explicit, policy-allowed, and reportable.

### Blocked

Recommended current behavior: defer.

`Blocked` implies either a new runtime status or a policy-approved mapping to existing failure, waiting-for-approval, or escalation behavior. That should remain separate from warning/skipped disclosure planning.

## 6. Disclosure Model Requirements

Future warning/skipped disclosures must be:

- bounded in length;
- redaction-safe;
- free of secret-like values;
- free of raw provider payloads;
- free of raw command output;
- free of raw CI logs;
- free of raw Jira/GitHub bodies;
- free of raw spec contents;
- free of raw parser payloads;
- free of environment variable values;
- free of credentials, authorization headers, private keys, and token-like values;
- excluded from stable executor error messages;
- represented as structured disclosure, not unbounded free-form logs.

Disclosure text must not be copied from diagnostic messages, command output, adapter payloads, provider output, or raw spec files by default.

## 7. Optionality Requirements For Skips

`SkippedWithDisclosure` requires an explicit optionality model before it can continue execution.

Future optionality should define:

- which hook checkpoint is optional;
- who or what declares optionality;
- whether policy can override optionality;
- whether a skipped hook still counts as unmet governance;
- whether skipped hooks affect WorkReport completeness;
- whether skipped hooks affect terminal status;
- how replay treats skipped events.

Until optionality exists, skipped-with-disclosure should remain rejected or fail closed.

## 8. Policy Relationship

Warning/skipped continuation must be policy-controlled.

Rules:

- policy still runs before the hook for the scoped action;
- hook status does not create a policy decision;
- hook status does not infer approval;
- warning/skipped continuation must require an explicit policy allow decision before the scoped skill invocation can continue;
- warning/skipped must not bypass approval gates;
- warning/skipped must not downgrade sensitive actions;
- warning/skipped must not authorize side effects.

If no policy semantics exist for warning/skipped continuation, the executor must reject the status or fail closed.

## 9. Event Semantics

Future warning/skipped events should follow the same safe-result-before-append posture:

1. validate hook input;
2. construct a safe warning or skipped invocation result;
3. construct requested/evaluated event payloads;
4. append requested/evaluated events together;
5. continue only if policy allows continuation and disclosure semantics are satisfied.

Do not append a requested event before a safe evaluated event can be constructed unless a separate requested-without-evaluated recovery policy exists.

## 10. WorkReport And Audit Relationship

Warnings and skips should become WorkReport disclosures before they are allowed to continue execution.

Future WorkReport behavior should:

- disclose warning/skipped statuses in a bounded report section;
- cite stable hook invocation or hook event references if citation targets exist;
- never copy raw hook context or raw payloads;
- distinguish `Passed`, `FailedClosed`, `Warning`, and `SkippedWithDisclosure`;
- make skipped required hooks visible as incomplete or policy-exceptional work.

Generic audit projection may be sufficient for initial warning/skipped event visibility, but dedicated hook audit sink emission remains deferred.

## 11. Retry And Replay Policy

Warning/skipped behavior must be deterministic across retries and duplicate runs.

Recommended defaults:

- do not move hooks inside the retry loop without separate retry-aware hook planning;
- warning/skipped status should not consume a skill retry attempt before `SkillInvocationRequested`;
- duplicate runs must not duplicate warning/skipped hook events;
- replay must preserve whether continuation was policy-allowed or rejected;
- changing policy after a warning/skipped run must not rewrite historical event meaning.

## 12. Error Handling

Until warning/skipped continuation is implemented, unsupported statuses should return stable non-leaking errors and append no hook or skill events.

Current stable executor error:

```text
executor.hook.before_skill_invocation.unsupported_status
```

Errors must not include raw hook IDs, input references, output references, phase IDs, source paths, payloads, command output, provider output, parser output, disclosures, tokens, or secret-like values.

## 13. Recommended First Implementation

Implemented next phase: **warning/skipped unsupported-status hardening tests**.

That implementation remained test-only. It proves:

- `Warning` appends no hook or skill events;
- `SkippedWithDisclosure` appends no hook or skill events;
- `Blocked` appends no hook or skill events if not already directly covered;
- errors use stable codes;
- errors do not leak hook references or disclosure text;
- passed and failed-closed behavior remain unchanged through existing coverage;
- no report artifacts are created.

It did not implement warning/skipped continuation.

## 14. Future Implementation Sequence

1. Add warning/skipped/blocked unsupported-status hardening tests.
2. Review.
3. Plan a bounded hook disclosure model if continuation remains desired. Complete: [Hook Disclosure Model Plan](hook-disclosure-model-plan.md).
4. Add disclosure model types and validation only.
5. Review.
6. Plan policy-controlled warning continuation.
7. Plan optionality-controlled skipped continuation.
8. Defer blocked status until runtime status or escalation semantics are accepted.

## 15. Test Plan

Future tests should cover:

- warning unsupported status appends no hook events;
- warning unsupported status appends no skill events;
- warning unsupported status error is stable and non-leaking;
- skipped-with-disclosure unsupported status appends no hook events;
- skipped-with-disclosure unsupported status appends no skill events;
- skipped-with-disclosure unsupported status error is stable and non-leaking;
- blocked unsupported status remains rejected or deferred;
- disclosure text is not leaked in errors;
- passed behavior remains unchanged;
- failed-closed behavior remains unchanged;
- duplicate run replay does not duplicate events;
- policy denial still appends no hook events;
- missing handler still appends no hook events;
- later-step targeting remains deterministic;
- no WorkReports, report artifacts, approvals, evidence references, local check results, adapter calls, command execution, side effects, or writes are created.

## 16. Deferred Work

- Warning continuation.
- Skipped-with-disclosure continuation.
- Blocked runtime status support.
- Hook optionality model.
- Hook disclosure model implementation.
- Policy-controlled continuation.
- WorkReport hook event citation targets.
- Dedicated hook audit sink/store.
- Hook observability metrics.
- Retry-aware hook execution.
- Multiple hook checkpoints.
- Workflow-declared hook configuration.
- Runtime hook configuration.
- Automatic local check execution.
- Command execution.
- Adapter invocation.
- Approval evidence attachment.
- EvidenceReference creation/attachment.
- Report artifact auto-writing.
- CLI rendering.
- Schema changes.
- Side-effect boundary implementation.
- Writes.
- Reasoning lineage.

## 17. Open Questions

- Should warnings ever continue by default, or should continuation always require explicit policy?
- Should skipped-with-disclosure be allowed only for optional hooks declared in workflow specs, or can runtime requests mark hooks optional?
- Should warning/skipped disclosures appear in WorkReport sections before they become executable continuation statuses?
- Should warning/skipped hook events become WorkReport citation targets, or should reports cite hook invocation IDs only?
- Should blocked map to failure, waiting-for-approval, escalation, or a future blocked terminal/intermediate status?
- Should policy decisions for warning/skipped continuation be separate events or reuse existing policy decision events?

## 18. Final Recommendation

Recommended next phase: **BeforeSkillInvocation unsupported status hardening review**.

The review should verify that unsupported status behavior remains fail-closed and non-leaking without enabling warning continuation, skipped-with-disclosure continuation, blocked runtime behavior, automatic hook invocation, workflow-declared hook configuration, runtime hook configuration, dedicated hook audit sinks, persistence, CLI behavior, schemas, local check execution, command execution, adapter invocation, approvals, evidence attachment, side effects, writes, reasoning lineage, hosted behavior, or release posture changes.
