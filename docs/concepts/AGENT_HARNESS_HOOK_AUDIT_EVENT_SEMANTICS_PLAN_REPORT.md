# Agent Harness Hook Audit/Event Semantics Plan Report

Report date: 2026-06-16

## 1. Executive Summary

Created the planning document for future Agent Harness Hook audit/event semantics. The plan keeps the next implementation conservative: a model-only hook audit record and stable hook invocation identity before executor integration, workflow event kinds, audit sink emission, CLI commands, schema fields, automatic local checks, side effects, writes, or hosted behavior.

## 2. Scope Completed

- Added [Agent Harness Hook Audit/Event Semantics Plan](../implementation-plans/agent-harness-hook-audit-event-semantics-plan.md).
- Defined source-of-truth boundaries for workflow events, audit projections, hook invocation results, EvidenceReference, and WorkReports.
- Recommended a model-only hook audit record before executor integration.
- Recommended stable hook invocation/result identity before WorkReport or audit citations.
- Deferred hook workflow event kinds until a separate executor integration plan.
- Updated `ROADMAP.md` with planning status.
- Updated [Governed Work Pattern](governed-work-pattern.md) with planning linkage.
- Updated [Agent Harness Hook Runtime Invocation Plan](../implementation-plans/agent-harness-hook-runtime-invocation-plan.md) to point to this plan.
- Updated [Agent Harness Quickstart](../user-guide/agent-harness-quickstart.md) with the audit/event semantics planning link.

## 3. Scope Explicitly Not Completed

- No runtime hook execution.
- No executor-integrated hook invocation.
- No workflow event kinds.
- No audit record model implementation.
- No audit sink emission.
- No automatic workflow execution.
- No automatic local check execution.
- No default local check handler registration.
- No command-output evidence.
- No CLI hook commands.
- No workflow schema fields.
- No workflow-declared hook configuration.
- No runtime harness generation.
- No nested harness execution.
- No recursive agents.
- No agent swarms.
- No hosted or distributed execution.
- No side-effect modeling.
- No writes.
- No approval evidence attachment.
- No reasoning lineage.
- No persistence changes.
- No report artifact auto-writing.
- No examples.
- No release posture change.

## 4. Planning Summary

The plan recommends a future model-only phase that introduces:

- stable hook invocation/result identity;
- a validated hook audit record model;
- bounded hook audit reference vocabulary;
- redaction-safe Debug behavior;
- fail-closed serde validation;
- no runtime mutation or event emission.

The plan explicitly rejects adding hook workflow events until executor placement, failure semantics, idempotency, state transition effects, snapshot projection, and terminal-state behavior are designed and reviewed.

## 5. Safety Boundary

The plan preserves the current safety boundary:

- workflow events remain the source of truth for runtime state;
- snapshots remain projections;
- audit events remain projections or explicit audit-shaped ledgers;
- hook invocation results remain in-memory unless a future model/persistence phase is accepted;
- reports remain governed handoff artifacts, not audit logs;
- no metadata-only post-terminal events are introduced.

## 6. Recommended Next Phase

Recommended next phase: **agent harness hook audit record core model, model-only**.

That phase should add validated, redaction-safe hook audit record and hook invocation ID types. It must not append workflow events, emit audit records, integrate with `LocalExecutor`, run local checks, invoke adapters, write files, persist records, expose CLI behavior, add schema fields, authorize side effects, add writes, implement reasoning lineage, enable recursive agents, enable agent swarms, or change release posture.

## 7. Validation Summary

Validation for this planning phase:

- `npm run check:docs`
  - Passed.
- `git diff --check`
  - Passed.

## 8. Remaining Known Limitations

- Runtime hook execution is not implemented.
- Executor integration is not implemented.
- Hook audit record types are not implemented.
- Hook workflow events are not implemented.
- Audit sink emission for hook records is not implemented.
- Hook result IDs are not implemented.
- WorkReport hook citation vocabulary is not implemented.
- Workflow schema support for hooks is not implemented.
- CLI hook commands are not implemented.
- Side effects and writes remain unsupported.
