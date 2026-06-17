# Terminal Report Agent Harness Hook Citation Integration Plan

Status: Implemented. Terminal report helper support for supplied agent harness hook invocation IDs is implemented. Executor-integrated hook report input propagation is implemented in [Executor Hook Report Input Propagation Plan](executor-hook-report-input-plan.md). Runtime hook execution, hook workflow events, audit sink emission, persistence, CLI behavior, workflow schema fields, side-effect modeling, writes, recursive agents, agent swarms, and release posture changes are not implemented.

## 1. Executive Summary

WorkReport citation vocabulary for agent harness hook invocation checkpoints is implemented and reviewed. `WorkReportCitationTarget::AgentHarnessHook` can cite a validated `AgentHarnessHookInvocationId` without embedding `AgentHarnessHookAuditRecord` values or copying hook context.

The narrow integration question was whether the terminal local report generation helper should accept explicitly supplied hook invocation IDs and include them as citations in generated in-memory reports.

This plan recommended a small implementation: add explicit supplied `AgentHarnessHookInvocationId` inputs to the in-memory terminal report helper and cite those IDs in the `ValidationAndQualityChecks` section. That implementation is now complete. It does not invoke hooks, discover hook records automatically, create hook invocation IDs, create hook audit records, persist hook audit records, append workflow events, emit audit sink records, change executor semantics, change report artifact behavior, add workflow schema fields, expose CLI behavior, model side effects, add writes, introduce recursive agents, introduce agent swarms, or change release posture.

## 2. Goals

- Allow terminal report helper callers to supply existing `AgentHarnessHookInvocationId` values.
- Cite supplied hook invocation IDs through existing `WorkReportCitation` constructors.
- Preserve terminal report generation as explicit, local, deterministic, and in-memory.
- Preserve current workflow pass/fail semantics.
- Avoid copying hook audit records, hook disclosures, hook references, or hook context into report sections.
- Keep source-of-truth boundaries clear between WorkReport, hook invocation results, hook audit records, workflow events, and audit events.
- Prepare for future executor or helper integration without enabling runtime hook execution.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- runtime hook execution;
- automatic hook invocation from terminal reports;
- automatic hook citation discovery;
- hook invocation ID creation;
- hook invocation result creation;
- hook audit record creation;
- hook audit record persistence;
- workflow event kinds;
- workflow event append behavior;
- audit sink emission;
- executor-integrated hook invocation;
- executor report input propagation;
- report artifact behavior changes;
- CLI hook commands or report rendering;
- workflow schema fields;
- workflow-declared hook configuration;
- automatic local check execution;
- default local check handler registration;
- command-output evidence;
- EvidenceReference creation or attachment;
- approval evidence attachment;
- reasoning lineage implementation;
- side-effect boundary implementation;
- write-capable adapters;
- recursive agents;
- agent swarms;
- hosted or distributed runtime claims;
- release posture changes.

## 4. Current Baseline

Implemented:

- `AgentHarnessHookInvocationId`;
- `AgentHarnessHookInvocationResult`;
- `AgentHarnessHookAuditRecord`;
- `WorkReport` core model;
- `WorkReportCitation`;
- `WorkReportCitationKind::AgentHarnessHook`;
- `WorkReportCitationTarget::AgentHarnessHook`;
- terminal local WorkReport generation helper;
- in-memory runtime result exposure helper;
- executor-integrated report-bearing execution;
- explicit local report artifact store;
- terminal report helper support for supplied local check result references;
- terminal report helper support for supplied typed handoff IDs.

Implemented after this plan:

- `TerminalLocalWorkReportInput` accepts supplied hook invocation IDs.
- `terminal_report_citations(...)` builds agent harness hook citations.
- `ValidationAndQualityChecks` receives validation diagnostic, local check result, and agent harness hook citations.
- Generated reports can cite supplied hook invocation IDs without creating hooks, audit records, events, or payload copies.

Not implemented:

- automatic hook citation wiring;
- runtime hook execution;
- executor-integrated hook invocation;
- hook workflow events;
- audit sink emission for hook records;
- hook audit record persistence;
- workflow schema support for hooks;
- CLI hook commands.

## 5. Proposed Helper Input

The implementation added one explicit optional input field to `TerminalLocalWorkReportInput`:

```rust
pub agent_harness_hook_invocation_ids: Vec<AgentHarnessHookInvocationId>
```

Rules:

- accept only already-constructed `AgentHarnessHookInvocationId` values;
- do not accept raw string hook IDs at the helper boundary;
- do not accept `AgentHarnessHookInvocationResult` values;
- do not accept `AgentHarnessHookAuditRecord` values;
- do not read hook records from storage;
- do not validate hook record existence;
- do not infer hook invocation IDs from workflow events, audit events, report notes, or local check results;
- do not fabricate missing hook invocation IDs.

Using `AgentHarnessHookInvocationId` keeps validation at the existing hook ID boundary and avoids generic string references.

## 6. Citation Construction Policy

Helper integration constructs citations with:

```rust
WorkReportCitationTarget::AgentHarnessHook {
    hook_invocation_id,
}
```

Rules:

- use `WorkReportCitation::new(...)`;
- use bounded, generic summary text such as `Agent harness hook checkpoint reference considered.`;
- use the report helper's existing sensitivity and redaction metadata;
- fail safely if citation construction fails;
- return structured non-leaking report-generation errors;
- do not create `EvidenceReference` values;
- do not create hook invocation IDs;
- do not create hook invocation results;
- do not create hook audit records;
- do not copy hook disclosures, hook reference names, workflow IDs, run IDs, actor IDs, output summaries, or raw context.

## 7. Section Placement Recommendation

Recommended first placement: `ValidationAndQualityChecks`.

Rationale:

- hook invocation IDs represent governed checkpoints;
- the future helper input would not include hook kind, hook status, or hook audit context;
- `ValidationAndQualityChecks` already hosts validation diagnostics and local check result citations;
- placing hook checkpoint citations there avoids implying that hook IDs are evidence payloads or typed handoff payloads;
- it keeps first integration deterministic without context-aware section routing.

Alternative placements:

| Section | Assessment |
| --- | --- |
| `EvidenceConsidered` | Useful when a hook itself cites evidence, but the hook citation target is not the evidence payload. Defer. |
| `DecisionsMade` | Useful for policy or approval decision hooks, but requires hook kind/status context. Defer. |
| `IncompleteOrDeferredWork` | Useful for skipped, blocked, or failed-closed hooks, but requires hook status context. Defer. |
| `OperatorHandoffNotes` | Useful for report/handoff hooks, but requires hook kind context. Defer. |
| New dedicated section | Not supported by v1 required section vocabulary; defer until report extension planning. |

## 8. Missing And Unavailable References

If no hook invocation IDs are supplied:

- preserve existing section text behavior;
- do not add missing hook citations by default;
- do not imply a hook checkpoint was expected;
- do not fabricate hook invocation IDs.

If future workflow-declared report contracts require hook citations, missing citation semantics should be handled in a separate contract-driven phase. This plan does not change missing-citation semantics.

## 9. Error Handling

Invalid hook invocation IDs should fail before report generation when callers construct `AgentHarnessHookInvocationId` values.

If citation construction fails inside report generation:

- return existing structured `WorkflowOsError` values;
- do not leak rejected IDs, raw paths, notes, risks, hook disclosures, provider payloads, command output, parser output, tokens, or secret-like values;
- do not convert helper citation failures into workflow diagnostics;
- do not mutate workflow runs;
- do not append workflow events;
- do not emit audit or observability events;
- do not persist reports or hook records;
- do not change workflow pass/fail results.

Executor-integrated report-bearing execution may later return the run plus a non-leaking report-generation error if supplied hook citation inputs fail, but that propagation is not part of this plan.

## 10. Workflow Semantics Boundary

The helper integration must not:

- mutate `WorkflowRun`;
- mutate `WorkflowRunSnapshot`;
- append workflow events;
- emit audit events;
- emit observability events;
- invoke hooks;
- call `invoke_agent_harness_hook(...)`;
- create hook invocation results;
- create hook audit records;
- read or write hook stores;
- touch a `StateBackend`;
- persist reports;
- create filesystem artifacts;
- expose CLI output;
- change executor behavior.

Executor-integrated report-bearing execution may accept hook invocation IDs only after helper integration is implemented and reviewed separately. That propagation is implemented in [Executor Hook Report Input Propagation Plan](executor-hook-report-input-plan.md).

## 11. Privacy And Redaction

The implementation preserves current WorkReport privacy boundaries.

It must not store or copy:

- hook audit records;
- hook disclosures;
- hook named references;
- hook input references;
- hook output references;
- supplemental hook references;
- workflow/run/actor context from hook records;
- raw prompts;
- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira or GitHub bodies;
- raw spec contents;
- parser payloads;
- environment variable values;
- credentials, authorization headers, private keys, or token-like values.

Debug output must not leak hook invocation IDs. Serialization may include valid hook invocation IDs as stable references, but it must not contain hook audit record payload fields or secret-like values.

## 12. Relationship To Agent Harness Hook Execution

This integration must not execute hooks.

The terminal report helper should consume IDs supplied by a caller. It should not:

- call `invoke_agent_harness_hook(...)`;
- evaluate hook contracts;
- create invocation results;
- create audit records;
- register local check handlers;
- run command hooks;
- read local files;
- call external systems;
- enable any default hook execution profile.

That keeps execution authority separate from report construction.

## 13. Relationship To Audit And Workflow Events

Hook invocation citations should not be modeled as `AuditEvent` or `WorkflowEvent` citations in this phase.

Reasons:

- hook workflow events are not implemented;
- hook audit sink emission is not implemented;
- `AgentHarnessHookAuditRecord` is model-only and not a persisted audit ledger entry;
- using event citation targets would overclaim runtime history.

Future hook event or audit sink work requires separate planning for event ordering, idempotency, persistence, projection, failure semantics, and report linkage.

## 14. Test Plan

The implementation added focused tests for:

- terminal report input accepts supplied `AgentHarnessHookInvocationId` values;
- generated report cites hook invocation IDs in `ValidationAndQualityChecks`;
- hook citations use `WorkReportCitationTarget::AgentHarnessHook`;
- hook citations map to `WorkReportCitationKind::AgentHarnessHook`;
- hook citations are deterministic and preserve existing validation/local check citations;
- absence of hook invocation IDs preserves existing not-available section behavior;
- helper does not call `invoke_agent_harness_hook(...)`;
- helper does not create hook invocation IDs;
- helper does not create hook invocation results;
- helper does not create hook audit records;
- helper does not recreate `EvidenceReference` values;
- helper does not copy hook disclosures, references, workflow/run/actor context, or payload fields;
- helper errors do not leak secret-like values;
- Debug output does not leak hook invocation IDs;
- serialization does not copy hook audit record fields or raw payload markers;
- existing terminal report helper tests still pass;
- existing executor-integrated report tests still pass;
- existing WorkReport, Agent Harness Hook, EvidenceReference, Diagnostic, validation, adapter telemetry, and runtime tests still pass.

## 15. Documentation Requirements

Docs must say:

- terminal report helper integration for supplied hook invocation IDs is implemented;
- automatic hook citation wiring is not implemented;
- runtime hook execution is not implemented;
- executor hook integration is not implemented;
- hook invocation result creation from reports is not implemented;
- hook audit record creation from reports is not implemented;
- hook audit record persistence is not implemented;
- workflow event emission is not implemented;
- audit sink emission is not implemented;
- EvidenceReference creation or attachment is not implemented;
- command-output evidence is not implemented;
- report artifact writing from this integration is not implemented;
- CLI exposure is not implemented;
- workflow schema fields are not implemented;
- side-effect boundary modeling is not implemented;
- writes remain unsupported;
- recursive agents and agent swarms remain non-goals.

## 16. Implementation Sequence

1. Add `agent_harness_hook_invocation_ids: Vec<AgentHarnessHookInvocationId>` to `TerminalLocalWorkReportInput`. Completed.
2. Add hook citation construction inside terminal report citation grouping. Completed.
3. Attach hook citations to `ValidationAndQualityChecks`. Completed.
4. Keep existing validation diagnostic and local check citations in that section. Completed.
5. Add focused helper tests. Completed.
6. Update docs and create an implementation report. Completed.
7. Review.
8. Only after review, plan executor-integrated report input propagation for hook invocation IDs.
9. Only after separate planning, consider hook workflow events, audit sink emission, or runtime hook execution.

## 17. Open Questions

- Should hook citations always appear in `ValidationAndQualityChecks`, or should future hook kind/status context route them to other sections?
- Should skipped, warning, blocked, or failed-closed hooks receive separate citation summaries?
- Should report contracts ever require hook citations by default?
- Should terminal report helper integration happen before or after hook audit record persistence planning?
- Should executor-integrated report inputs accept hook invocation IDs directly after helper support exists?
- How should hook citations relate to future reasoning lineage nodes?
- Should future hook audit records and hook invocation IDs remain the same identity boundary?

## 18. Final Recommendation

Proceed next with **terminal report helper hook citation integration review**.

That review should verify the explicit supplied hook invocation ID input, citation construction through existing WorkReport citation constructors, `ValidationAndQualityChecks` placement, focused tests, and documentation honesty. It must not implement runtime hook execution, executor hook integration, workflow events, audit sink emission, persistence, report artifact behavior changes, CLI behavior, workflow schema fields, side-effect modeling, writes, recursive agents, agent swarms, or release posture changes.
