# WorkReport Agent Harness Hook Citation Target Plan

Status: Implemented, model-only. WorkReport hook citation target vocabulary is implemented. Later bounded phases implemented terminal report helper integration, executor hook report input propagation, explicit in-memory runtime hook execution for supplied hook invocation IDs, and explicit `BeforeReport` executor report-path integration. Follow-on event/audit semantics planning is documented in [Executor Hook Event And Audit Semantics Plan](executor-hook-event-audit-semantics-plan.md). Automatic executor hook invocation, workflow events, audit sink emission, persistence, CLI behavior, schemas, side effects, writes, recursive agents, agent swarms, and release posture changes are not implemented.

## 1. Executive Summary

The Agent Harness Hook audit record core model is implemented and reviewed. Workflow OS now has a stable `AgentHarnessHookInvocationId`, a model-only `AgentHarnessHookAuditRecord`, and WorkReport citation vocabulary for citing hook invocation checkpoints by stable ID.

The next question is how terminal reports should cite governed hook checkpoints without copying hook context, disclosures, raw payloads, or implying runtime hook execution.

This plan recommended adding WorkReport citation vocabulary for agent harness hook invocation IDs as a narrow model-only implementation. That citation vocabulary is now implemented. Later phases implemented explicit terminal report helper wiring, executor report input propagation, and explicit in-memory runtime hook execution. This plan does not implement automatic executor hook invocation, workflow events, audit sink emission, persistence, CLI behavior, workflow schema fields, side effects, writes, recursive agents, agent swarms, or release posture changes.

## 2. Goals

- Let future WorkReports cite governed hook checkpoints by stable reference.
- Preserve `WorkReport` as a governed handoff artifact.
- Preserve `AgentHarnessHookAuditRecord` as a model-only audit record until persistence/event semantics are separately accepted.
- Cite stable hook invocation IDs rather than copying hook audit record payloads.
- Preserve existing WorkReport citation validation, redaction, serde, and debug behavior.
- Keep terminal report generation behavior unchanged in the first implementation.
- Avoid creating hook records, evidence references, workflow events, audit records, approvals, local check results, or typed handoffs implicitly.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- runtime hook execution;
- executor-integrated hook invocation;
- automatic hook invocation from reports;
- terminal report helper integration;
- automatic report citation wiring;
- workflow event kinds;
- workflow event append behavior;
- audit sink emission;
- hook audit record persistence;
- report artifact behavior changes;
- CLI hook commands or report rendering;
- workflow schema fields;
- workflow-declared hook configuration;
- automatic local check execution;
- default local check handler registration;
- command-output evidence;
- side-effect modeling;
- writes;
- approval evidence attachment;
- reasoning lineage implementation;
- recursive agents;
- agent swarms;
- hosted or distributed runtime claims;
- release posture changes.

## 4. Current Baseline

Implemented:

- `WorkReport` core model;
- `WorkReportCitation`;
- `WorkReportCitationKind`;
- `WorkReportCitationTarget`;
- `WorkReportStableReference`;
- WorkReport citation targets for EvidenceReference, workflow event, audit event, adapter telemetry, validation diagnostic, local check result, typed handoff, agent harness hook invocation, approval decision, policy decision, and future reasoning lineage vocabulary;
- terminal local WorkReport generation helper;
- runtime result report exposure helper;
- executor-integrated report-bearing local execution;
- explicit local report artifact store;
- `AgentHarnessHookContract`;
- `AgentHarnessHookInvocationResult`;
- `AgentHarnessHookInvocationId`;
- `AgentHarnessHookAuditRecord`.
- terminal report helper consumption of explicitly supplied hook invocation IDs.

Not implemented:

- hook audit record persistence;
- hook workflow events;
- audit sink emission for hook records;
- runtime hook execution;
- executor hook integration.

## 5. Citation Target Decision

The first implementation added a dedicated WorkReport citation target for agent harness hook invocation IDs.

Recommended kind:

```rust
WorkReportCitationKind::AgentHarnessHook
```

Recommended target:

```rust
WorkReportCitationTarget::AgentHarnessHook {
    hook_invocation_id: AgentHarnessHookInvocationId,
}
```

Rationale:

- `AgentHarnessHookInvocationId` is now the stable reviewed identifier.
- The citation can point at a hook checkpoint without embedding the full audit record.
- It avoids deciding whether hook audit records are later persisted directly or projected from workflow events.
- It keeps WorkReport citation vocabulary aligned with existing typed ID patterns such as `TypedHandoffId`.
- It does not imply that a hook executed at runtime or that an audit sink received a record.

## 6. Why Not Embed AgentHarnessHookAuditRecord

WorkReports should not embed full hook audit records.

Reasons:

- reports should cite governed records, not copy bounded audit payloads;
- hook records include workflow/run/actor/reference/disclosure context that may be sensitive;
- embedding records would couple reports to hook audit record storage shape;
- future persistence and event semantics are not settled;
- citation target vocabulary should remain small and stable.

## 7. Why Not Use AuditEvent Yet

`AuditEvent` currently projects workflow events. Hook audit records are not workflow events and are not audit sink emissions.

Using `WorkReportCitationTarget::AuditEvent` for hook records would overclaim that a hook audit event exists. That should wait until hook workflow events or hook audit sink emission are explicitly implemented and reviewed.

## 8. Why Not Use WorkflowEvent Yet

Hook workflow events are not implemented.

Using workflow event citations would fabricate runtime history. Future executor integration may later add hook workflow event kinds, but that requires separate planning for event order, idempotency, failure semantics, snapshot projection, and terminal-state behavior.

## 9. Source-Of-Truth Rules

- `AgentHarnessHookInvocationId` is the stable hook checkpoint citation identity.
- `AgentHarnessHookAuditRecord` is the model-only record shape for hook audit context.
- `WorkReport` cites hook checkpoints; it does not reproduce hook audit record payloads.
- `WorkflowRunEvent` remains the source of truth for runtime state.
- `AuditEvent` remains a projection or audit-shaped record, not an alias for hook records.
- `EvidenceReference` remains evidence citation substrate, not the hook record itself.

## 10. Section Placement Policy

Future report helpers should likely place hook citations in these sections:

- `ValidationAndQualityChecks` for validation/review hooks;
- `DecisionsMade` for hooks that evaluate decision checkpoints;
- `EvidenceConsidered` when hooks cite evidence references;
- `IncompleteOrDeferredWork` when hooks are skipped, blocked, or failed closed;
- `OperatorHandoffNotes` only for bounded operator-facing checkpoint notes.

The first implementation should add citation vocabulary only. It should not change section population or terminal report helper behavior.

## 11. Citation Construction Rules

Future call sites should construct hook citations by:

1. receiving a validated `AgentHarnessHookInvocationId`;
2. constructing `WorkReportCitationTarget::AgentHarnessHook`;
3. constructing a `WorkReportCitation` through `WorkReportCitation::new(...)`;
4. supplying bounded, non-secret summary text only when useful.

Rules:

- Do not create hook invocation IDs inside `WorkReportCitation`.
- Do not fabricate hook IDs.
- Do not create hook audit records inside reports.
- Do not copy hook disclosures, reference names, workflow IDs, run IDs, actor IDs, or raw context into citation summaries by default.
- Do not copy raw prompts, raw spec contents, raw command output, provider payloads, parser payloads, environment values, credentials, tokens, or unbounded text.
- Missing hook references should remain explicit not-available section text until missing-citation policy is separately designed.

## 12. Privacy And Redaction

The citation target must inherit WorkReport citation privacy rules.

Rules:

- Use `WorkReportCitation::new(...)`.
- Use `AgentHarnessHookInvocationId::new(...)`.
- Reject secret-like hook invocation IDs.
- Keep citation summaries bounded and redacted.
- Keep redaction metadata validated.
- Keep `Debug` redaction-safe.
- Keep serde fail-closed.
- Do not serialize raw hook disclosures, hook reference names, workflow/run context, command output, provider payloads, parser payloads, environment values, tokens, credentials, authorization headers, private keys, or token-like strings.

## 13. Failure Semantics

Citation construction failures should fail citation construction, not hook invocation or workflow execution.

Rules:

- invalid hook citation IDs fail closed;
- citation failures must not fabricate hook IDs;
- citation failures must not change workflow run status;
- citation failures must not append runtime events;
- citation failures must not emit audit records;
- citation failures must not become misleading project diagnostics;
- errors must use stable, non-leaking messages.

## 14. Test Plan For Future Implementation

Future implementation should test:

- `WorkReportCitationKind::AgentHarnessHook` is representable;
- `WorkReportCitationTarget::AgentHarnessHook` validates with a safe `AgentHarnessHookInvocationId`;
- citation kind mapping returns `AgentHarnessHook`;
- serde round trip for hook citation target;
- invalid hook invocation ID/reference fails closed;
- secret-like hook invocation ID fails without leaking values;
- WorkReport citation debug output does not leak hook invocation IDs or summaries;
- WorkReport serialization does not copy hook audit record fields;
- WorkReport serialization does not contain raw hook payload markers;
- existing WorkReport tests still pass;
- existing Agent Harness Hook audit record and invocation tests still pass;
- no report generation helper behavior changes;
- no report artifact behavior changes;
- no CLI, schema, persistence, automatic executor hook invocation, side-effect, or write behavior is introduced.

## 15. Documentation Requirements For Future Implementation

Docs must say:

- WorkReport citation vocabulary for agent harness hooks is implemented;
- terminal report helper integration is not implemented;
- automatic hook citation wiring is not implemented;
- automatic executor hook invocation is not implemented;
- workflow event emission is not implemented;
- audit sink emission is not implemented;
- hook audit record persistence is not implemented;
- CLI exposure is not implemented;
- workflow schema fields are not implemented;
- side-effect boundary modeling is not implemented;
- writes remain unsupported;
- recursive agents and agent swarms remain non-goals.

## 16. Proposed Implementation Sequence

Recommended sequence:

1. WorkReport hook citation target vocabulary only. Implemented.
2. Focused WorkReport citation tests. Implemented.
3. Docs update and implementation report. Implemented.
4. Maintainer review.
5. Only after review, plan terminal report helper support for supplied hook invocation IDs.
6. Only after separate planning, consider hook persistence or executor integration.

## 17. Open Questions

- Should the citation kind be named `AgentHarnessHook`, `HookInvocation`, or `HookAuditRecord`?
- Should reports cite `AgentHarnessHookInvocationId` directly or a future hook audit record ID?
- Should `AgentHarnessHookInvocationId` and future hook audit record IDs remain the same identity?
- Should hook citations appear in `ValidationAndQualityChecks` by default once helper integration exists?
- Should skipped, warning, blocked, or failed-closed hooks be cited differently?
- Should WorkReportContract default citation requirements include hook citations, or keep them optional?
- Should hook audit records be persisted before terminal report helper integration?

## 18. Final Recommendation

Proceed next with **WorkReport hook citation target vocabulary review**.

That review should verify the citation kind/target and focused tests only. It must not implement runtime hook execution, executor integration, terminal report helper wiring, workflow events, audit sink emission, persistence, report artifact behavior changes, CLI behavior, schema fields, side-effect modeling, writes, recursive agents, agent swarms, or release posture changes.
