# Executor Hook Event Append Plan Report

## 1. Executive Summary

Executor hook event append planning is complete. The plan defines the first safe future runtime boundary for appending hook workflow events after the hook event vocabulary and generic audit projection have been implemented and reviewed.

Fix-forward note: the recommended explicit `BeforeSkillInvocation` append implementation has since been completed in a bounded phase. See [Executor Before-Skill Hook Event Append Report](EXECUTOR_BEFORE_SKILL_HOOK_EVENT_APPEND_REPORT.md).

The plan recommends a future implementation for one explicit pre-terminal `BeforeSkillInvocation` checkpoint. It deliberately does not convert the existing post-terminal `BeforeReport` hook into workflow events, because current terminal-state rules reject post-terminal mutation.

## 2. Scope Completed

- Created [Executor Hook Event Append Plan](../implementation-plans/executor-hook-event-append-plan.md).
- Documented current executor, hook event, audit projection, and `BeforeReport` boundaries.
- Selected a future first append target: explicit pre-terminal `BeforeSkillInvocation`.
- Defined event ordering.
- Defined explicit input boundary.
- Defined conservative hook status semantics.
- Defined failure behavior.
- Defined idempotency and replay requirements.
- Defined audit and observability posture.
- Defined privacy/redaction requirements.
- Defined relationship to policy, approval, local checks, adapters, and `BeforeReport`.
- Defined future test plan and implementation sequence.
- Updated roadmap and related concept/planning docs.

## 3. Scope Explicitly Not Completed

- No executor event append behavior.
- No automatic hook invocation.
- No broad executor hook checkpoint support.
- No workflow-declared hook configuration.
- No runtime hook configuration.
- No post-terminal workflow event model.
- No conversion of `BeforeReport` into workflow events.
- No dedicated hook audit sink emission.
- No hook audit store or persistence.
- No hook observability metrics.
- No WorkReport hook event citation target.
- No CLI behavior.
- No workflow schema fields.
- No automatic local check execution.
- No command execution.
- No adapter invocation.
- No external provider calls.
- No `EvidenceReference` creation or attachment.
- No approval request or approval decision creation.
- No approval evidence attachment.
- No report artifact writes.
- No reasoning lineage.
- No side-effect boundary implementation.
- No writes.
- No recursive agents or agent swarms.
- No hosted/distributed runtime claims.
- No release posture changes.

## 4. Recommendation Summary

Recommended next implementation phase: **executor `BeforeSkillInvocation` hook event append, explicit input only**.

The first implementation should support `Passed` first, append `HookInvocationRequested` and `HookInvocationEvaluated` through the existing executor append pipeline, rely on existing generic audit projection, and preserve existing executor behavior when no explicit hook input is supplied.

## 5. Boundary Summary

The plan keeps boundaries clean:

- `WorkflowRunEvent` remains the source of truth for runtime state.
- `AuditEvent` remains a projection of accepted workflow events.
- `AgentHarnessHookAuditRecord` remains model-only.
- `BeforeReport` remains post-terminal, in-memory-only, and report-path-only.
- Hooks do not replace policy gates, approvals, local checks, adapters, evidence, or reports.

## 6. Privacy And Redaction Summary

The future append path must remain reference-first and bounded. It must not store raw provider payloads, raw command output, raw CI logs, raw Jira/GitHub bodies, raw spec contents, raw parser payloads, environment values, credentials, authorization headers, private keys, token-like values, unbounded hook context, unbounded disclosures, or evidence payloads.

## 7. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 8. Remaining Known Limitations

- The first explicit `BeforeSkillInvocation` executor hook event append path is implemented; broader hook append behavior remains unimplemented.
- Only the first recommended checkpoint is selected.
- Warning, skipped, failed-closed, and blocked behavior need implementation-level review.
- Hook observability metrics remain deferred.
- Dedicated hook audit sink/store semantics remain deferred.
- `BeforeReport` remains in-memory-only and not state-visible.
- WorkReport hook event citation targets remain deferred.

## 9. Recommended Next Phase

Recommended next phase: **executor `BeforeSkillInvocation` hook event append implementation, explicit input only**.

That implementation must remain narrow, support `Passed` first, avoid automatic hook broadening, avoid post-terminal events, avoid local check/command/adapter execution, and preserve existing executor semantics when no explicit hook input is supplied.
