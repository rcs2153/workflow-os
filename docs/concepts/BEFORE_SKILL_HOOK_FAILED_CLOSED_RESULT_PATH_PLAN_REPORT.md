# BeforeSkillInvocation Failed-Closed Result Path Plan Report

## 1. Executive Summary

BeforeSkillInvocation failed-closed result path planning is complete. The plan recommends the smallest safe next implementation: add a validated failed-closed hook invocation result path, append requested/evaluated hook events only after safe result construction, and fail the run before `SkillInvocationRequested`.

The plan keeps `Passed` as the only continuing hook status. It does not implement warning continuation, skipped-with-disclosure continuation, blocked runtime behavior, automatic hook invocation, workflow-declared hook configuration, runtime hook configuration, persistence, CLI behavior, schemas, local check execution, command execution, adapter invocation, approvals, evidence attachment, side effects, writes, reasoning lineage, hosted behavior, or release posture changes.

Fix-forward note: the narrow failed-closed result path implementation is now complete and documented in [BeforeSkillInvocation Failed-Closed Result Path Implementation Report](BEFORE_SKILL_HOOK_FAILED_CLOSED_RESULT_PATH_IMPLEMENTATION_REPORT.md). The original planning scope remains accurate for the planning phase.

## 2. Scope Completed

- Created [BeforeSkillInvocation Failed-Closed Result Path Plan](../implementation-plans/before-skill-hook-failed-closed-result-plan.md).
- Defined the recommended safe-result-before-append model.
- Defined failed-closed runtime failure code and non-leaking failure message posture.
- Defined failed-closed result construction requirements.
- Defined event ordering for requested/evaluated/failed run behavior.
- Defined idempotency and replay expectations.
- Defined policy, approval, audit, report, privacy, and redaction boundaries.
- Defined future implementation test requirements.
- Recommended the next implementation phase.

## 3. Scope Explicitly Not Completed

- No runtime code changes.
- No failed-closed event append implementation during the planning phase.
- No warning continuation.
- No skipped-with-disclosure continuation.
- No blocked runtime status support.
- No automatic hook invocation.
- No workflow-declared hook configuration.
- No runtime hook configuration.
- No post-terminal workflow events.
- No dedicated hook audit sink/store.
- No hook observability metrics.
- No WorkReport hook event citation targets.
- No CLI behavior.
- No workflow schema fields.
- No automatic local check execution.
- No command execution.
- No adapter invocation.
- No approvals.
- No `EvidenceReference` creation or attachment.
- No report artifact writes.
- No reasoning lineage.
- No side-effect boundary implementation.
- No writes.
- No recursive agents or agent swarms.
- No hosted/distributed runtime claims.
- No release posture changes.

## 4. Recommendation Summary

Recommended next implementation phase at the time of planning: **BeforeSkillInvocation failed-closed result path implementation**. That implementation is now complete and ready for review.

The implementation should remain narrow:

- construct a safe failed-closed hook invocation result;
- validate requested/evaluated hook workflow event payloads before appending;
- append requested/evaluated events together;
- append `RunFailed`;
- prevent `SkillInvocationRequested`;
- preserve existing `Passed`, no-hook, policy-denial, missing-handler, later-step, and replay behavior.

## 5. Boundary Summary

The plan preserves these source-of-truth boundaries:

- `WorkflowRunEvent` remains the runtime source of truth.
- `WorkflowRunSnapshot` remains replay-derived.
- Generic audit projection remains sufficient for the first failed-closed implementation.
- `AgentHarnessHookAuditRecord` remains model-only.
- `WorkReport` remains a governed handoff artifact and should not become a hidden hook execution log.
- Hooks remain downstream of policy and do not replace approvals.

## 6. Privacy And Redaction Summary

The plan forbids raw provider payloads, raw command output, raw CI logs, raw Jira/GitHub bodies, raw spec contents, raw parser payloads, environment values, credentials, authorization headers, private keys, token-like values, unbounded hook context, unbounded disclosures, and evidence payloads.

Failed-closed errors should use stable codes and generic non-leaking messages. Disclosure text should either remain absent in the first implementation or be bounded, validated, redaction-aware, and excluded from runtime failure messages.

## 7. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 8. Remaining Known Limitations

- The planning phase did not implement failed-closed runtime behavior.
- `Passed` remains the only continuing hook status today.
- Warning/skipped continuation requires separate report disclosure and policy semantics.
- Blocked semantics require a separately accepted runtime status or escalation mapping.
- Retry-aware hook execution remains deferred.
- Dedicated hook audit sink emission and WorkReport hook event citations remain deferred.

## 9. Recommended Next Phase

Recommended next phase after implementation: **BeforeSkillInvocation failed-closed result path review**.

That review should confirm the code-bearing phase stayed tightly scoped. It must not accept warning continuation, skipped continuation, blocked behavior, automatic hook invocation, workflow-declared hook configuration, runtime hook configuration, dedicated hook audit sinks, persistence, CLI behavior, schemas, local check execution, command execution, adapter invocation, approvals, evidence attachment, side effects, writes, recursive agents, agent swarms, hosted behavior, or release posture changes.
