# WorkReport Agent Harness Hook Citation Target Plan Report

Report date: 2026-06-16

## 1. Executive Summary

Created the planning document for future WorkReport citation vocabulary for Agent Harness Hook checkpoints. The plan recommends adding a narrow model-only citation kind and target that cite `AgentHarnessHookInvocationId` values without embedding hook audit records or implying runtime hook execution.

This phase does not implement WorkReport hook citation vocabulary, terminal report helper wiring, runtime hook execution, executor integration, workflow events, audit sink emission, persistence, CLI behavior, schema fields, side-effect modeling, writes, recursive agents, agent swarms, or release posture changes.

## 2. Scope Completed

- Added [WorkReport Agent Harness Hook Citation Target Plan](../implementation-plans/work-report-hook-citation-target-plan.md).
- Recommended `WorkReportCitationKind::AgentHarnessHook`.
- Recommended `WorkReportCitationTarget::AgentHarnessHook { hook_invocation_id: AgentHarnessHookInvocationId }`.
- Clarified why reports should cite hook invocation IDs rather than embed `AgentHarnessHookAuditRecord`.
- Clarified why existing `AuditEvent` and `WorkflowEvent` citation targets should not be reused for hook records yet.
- Updated `ROADMAP.md` with planning status.
- Updated [Governed Work Pattern](governed-work-pattern.md) with planning linkage.
- Updated [WorkReportContract Planning Document](../implementation-plans/work-report-contract-plan.md) with planning linkage and non-implemented status.

## 3. Scope Explicitly Not Completed

- No WorkReport hook citation target implementation.
- No terminal report helper integration.
- No automatic hook citation wiring.
- No runtime hook execution.
- No executor-integrated hook invocation.
- No workflow event kinds.
- No workflow event append behavior.
- No audit sink emission.
- No hook audit record persistence.
- No report artifact behavior changes.
- No CLI behavior.
- No workflow schema fields.
- No workflow-declared hook configuration.
- No automatic local check execution.
- No default local check handler registration.
- No command-output evidence.
- No side-effect modeling.
- No writes.
- No approval evidence attachment.
- No reasoning lineage implementation.
- No recursive agents.
- No agent swarms.
- No release posture change.

## 4. Planning Summary

The plan recommends citing hook checkpoints through stable hook invocation IDs:

```rust
WorkReportCitationKind::AgentHarnessHook
WorkReportCitationTarget::AgentHarnessHook {
    hook_invocation_id: AgentHarnessHookInvocationId,
}
```

The plan rejects embedding full hook audit records in WorkReports and rejects reusing `AuditEvent` or `WorkflowEvent` citation targets before hook audit/event runtime semantics exist.

## 5. Safety Boundary

The plan preserves the current safety boundary:

- WorkReports cite stable references instead of copying payloads.
- `AgentHarnessHookAuditRecord` remains model-only.
- `WorkflowRunEvent` remains the runtime state source of truth.
- `AuditEvent` remains a projection or audit-shaped record, not an alias for hook audit records.
- Hook citations do not imply hook execution, audit sink emission, persistence, side effects, writes, or runtime state mutation.

## 6. Recommended Next Phase

Recommended next phase: **WorkReport hook citation target vocabulary implementation, model-only**.

That implementation should add the citation kind/target and focused tests only. It must not implement runtime hook execution, executor integration, terminal report helper wiring, workflow events, audit sink emission, persistence, report artifact behavior changes, CLI behavior, schema fields, side-effect modeling, writes, recursive agents, agent swarms, or release posture changes.

## 7. Validation Summary

Validation for this planning phase:

- `npm run check:docs`
  - Passed.
- `git diff --check`
  - Passed.

## 8. Remaining Known Limitations

- WorkReport hook citation target vocabulary is not implemented.
- Terminal report helper support for supplied hook invocation IDs is not implemented.
- Runtime hook execution is not implemented.
- Executor integration is not implemented.
- Hook workflow events are not implemented.
- Audit sink emission for hook records is not implemented.
- Hook audit records are not persisted.
- Workflow schema support for hooks is not implemented.
- CLI hook commands are not implemented.
- Side effects and writes remain unsupported.
