# Executor Hook Checkpoint Plan Report

Report date: 2026-06-17

## 1. Executive Summary

Created the executor hook checkpoint planning document for future agent harness hook integration with `LocalExecutor`.

The plan recommends a conservative first executor-integrated checkpoint: explicit `BeforeReport` hook execution on the report-bearing path only, in-memory only, after a terminal run exists and before terminal report generation. It does not implement executor hooks or change runtime behavior.

## 2. Scope Completed

- Added [Executor Hook Checkpoint Plan](../implementation-plans/executor-hook-checkpoint-plan.md).
- Inventoried the current `LocalExecutor` lifecycle and event order.
- Identified candidate checkpoint locations.
- Recommended `BeforeReport` as the smallest first implementation target.
- Defined event ordering, failure semantics, idempotency, policy/approval, local check, report citation, privacy, and test posture.
- Updated roadmap and concept docs to link the plan.

## 3. Scope Explicitly Not Completed

- No automatic executor hook invocation.
- No hook workflow events.
- No audit sink emission for hooks.
- No hook persistence.
- No report artifact writes.
- No workflow schema fields.
- No workflow-declared hook configuration.
- No runtime hook config.
- No CLI hook commands.
- No automatic local check execution.
- No command execution.
- No adapter invocation.
- No `EvidenceReference` creation or attachment.
- No approval request or approval decision creation.
- No reasoning lineage.
- No side-effect boundary implementation.
- No writes.
- No recursive agents or agent swarms.
- No hosted or distributed runtime claims.
- No release posture change.

## 4. Checkpoint Recommendation

The plan recommends `AgentHarnessHookKind::BeforeReport` as the first executor-integrated checkpoint.

The checkpoint should run only on explicit report-bearing execution paths, after `LocalExecutor::execute(...)` returns a terminal run and before `expose_terminal_local_work_report_result(...)` constructs the report.

This avoids post-terminal workflow event mutation, avoids side-effect-adjacent skill gating, and uses the existing report error boundary.

## 5. Runtime Semantics Summary

The plan preserves current runtime semantics.

First implementation expectations:

- `execute(...)` remains unchanged.
- `execute_with_report(...)` remains explicit.
- hook execution failure does not change run status;
- hook execution failure does not append events;
- hook execution failure does not emit audit or observability records;
- hook execution failure does not write artifacts;
- hook execution failure returns a non-leaking report-path error beside the run.

## 6. Privacy Summary

The plan keeps hook checkpoint inputs reference-first and bounded. It prohibits raw prompts, raw spec contents, raw command output, provider payloads, parser payloads, environment values, credentials, tokens, unbounded summaries, and hook execution transcripts.

## 7. Validation Commands Run

- `npm run check:docs`
  - Passed.
- `git diff --check`
  - Passed.

## 8. Remaining Known Limitations

- No executor hook behavior is implemented.
- Hook event/audit persistence semantics remain unplanned beyond the first in-memory checkpoint recommendation.
- Pre-skill, approval, retry, escalation, and step-level hook checkpoints remain deferred.
- Workflow schema and CLI hook surfaces remain deferred.

## 9. Recommended Next Phase

Recommended next phase: **executor hook checkpoint plan review**.

After review, the next implementation should be a tightly scoped explicit `BeforeReport` executor hook integration, report-path-only and in-memory-only.
