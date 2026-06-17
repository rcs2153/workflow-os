# Executor Hook Report Input Propagation Plan

Status: Implemented. Terminal report helper hook citation integration is implemented for explicitly supplied hook invocation IDs, and executor-integrated hook report input propagation now forwards caller-supplied `AgentHarnessHookInvocationId` values into terminal reports. The explicit in-memory runtime hook execution helper is implemented. The explicit `BeforeReport` executor checkpoint is implemented for `execute_with_report(...)` only, event/audit semantics planning is documented in [Executor Hook Event And Audit Semantics Plan](executor-hook-event-audit-semantics-plan.md), model-only hook workflow event vocabulary is implemented, and generic hook workflow event audit projection is implemented as projection-only in [Hook Event Audit Projection Plan](hook-event-audit-projection-plan.md). Broader automatic executor hook invocation, executor hook event append behavior, dedicated hook audit sink emission, persistence, CLI behavior, workflow schema fields, side-effect modeling, writes, recursive agents, agent swarms, and release posture changes are not implemented.

## 1. Executive Summary

WorkReport citation vocabulary for agent harness hook invocation checkpoints is implemented and reviewed. The terminal local report helper can now cite explicitly supplied `AgentHarnessHookInvocationId` values in generated in-memory reports.

The executor gap was narrow: `LocalExecutionReportInputs` did not accept hook invocation IDs, and the executor adapter path passed an empty hook invocation list into `TerminalLocalWorkReportInput`.

This plan defined the smallest safe implementation for explicit executor-integrated hook report input propagation. That implementation is complete. Later bounded phases implemented the explicit in-memory runtime hook execution helper, model-only hook workflow event vocabulary, and projection-only generic hook workflow event audit projection. This plan does not implement automatic hook discovery, automatic executor hook invocation, executor hook event append behavior, dedicated hook audit sink emission, persistence, schemas, CLI behavior, report artifacts, side effects, writes, recursive agents, agent swarms, or release posture changes.

## 2. Goals

- Let callers of `LocalExecutor::execute_with_report(...)` supply existing `AgentHarnessHookInvocationId` values.
- Forward those IDs from `LocalExecutionReportInputs` to `TerminalLocalWorkReportInput`.
- Preserve executor semantics and existing `execute(...)` behavior.
- Keep hook citations explicit, local, deterministic, and in-memory.
- Use existing `AgentHarnessHookInvocationId`, `WorkReportCitation`, and terminal report helper validation boundaries.
- Avoid copying hook audit records, hook invocation results, hook context, hook disclosures, or hook payloads into executor inputs, report sections, citations, debug output, or serialization.
- Preserve current behavior when no hook invocation IDs are supplied.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- runtime hook execution;
- executor-integrated hook invocation;
- automatic hook citation discovery;
- automatic hook invocation from reports;
- hook invocation ID creation;
- hook invocation result creation;
- hook audit record creation;
- hook audit record persistence;
- hook workflow event kinds;
- hook workflow event append behavior;
- audit sink emission;
- report artifact behavior changes;
- workflow schema fields;
- workflow-declared hook configuration;
- CLI rendering or export;
- CLI hook commands;
- example updates;
- automatic local check execution;
- default local check handler registration;
- command-output evidence;
- `EvidenceReference` creation or attachment;
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
- `WorkReportCitationKind::AgentHarnessHook`;
- `WorkReportCitationTarget::AgentHarnessHook`;
- terminal local WorkReport generation helper;
- terminal report helper support for supplied `agent_harness_hook_invocation_ids`;
- executor-integrated report-bearing execution through `LocalExecutor::execute_with_report(...)`.

Closed gap:

- `LocalExecutionReportInputs` now has an `agent_harness_hook_invocation_ids` field.
- `terminal_report_input_for_run(...)` now forwards `report.agent_harness_hook_invocation_ids.clone()`.
- Therefore executor-integrated report-bearing execution can forward caller-supplied hook references into generated reports.

This closed gap is input propagation only. It is not runtime execution.

## 5. Proposed API Change

The implementation added one field to `LocalExecutionReportInputs`:

```rust
pub agent_harness_hook_invocation_ids: Vec<AgentHarnessHookInvocationId>
```

Rules:

- accept only already-validated `AgentHarnessHookInvocationId` values;
- do not accept raw strings;
- do not accept `AgentHarnessHookInvocationResult` values;
- do not accept `AgentHarnessHookAuditRecord` values;
- do not accept hook contracts, hook disclosures, hook input/output references, supplemental references, output summaries, workflow/run/actor context, or payload fields;
- do not read hook records from storage;
- do not validate hook record existence;
- do not infer IDs from workflow events, audit events, report notes, local check results, or typed handoffs;
- do not fabricate missing hook invocation IDs.

This is an additive local Rust API change. It must preserve existing executor methods and should update tests and call sites that construct `LocalExecutionReportInputs`.

## 6. Propagation Boundary

The implementation updated `terminal_report_input_for_run(...)` so:

```rust
agent_harness_hook_invocation_ids: report.agent_harness_hook_invocation_ids.clone()
```

This is the only intended behavior change.

The executor should still:

- call `execute(&request.execution)` first;
- return execution errors unchanged;
- preserve `run` when report generation fails after execution;
- never mutate workflow state for report generation;
- never append events for report generation;
- never invoke hooks;
- never create hook invocation results;
- never create hook audit records;
- never write report artifacts automatically;
- never expose CLI output.

## 7. Debug And Redaction Policy

The implementation updated `LocalExecutionReportInputs` Debug to include only a count:

```rust
.field("agent_harness_hook_count", &self.agent_harness_hook_invocation_ids.len())
```

Debug output must not include hook invocation IDs, report IDs, paths, tokens, notes, limitations, risks, handoff text, hook disclosures, hook output summaries, or payload-like strings.

Serialization is not currently the primary surface for executor report inputs. If serialization is added later, hook invocation IDs must be treated as potentially sensitive stable references and must not be mixed with hook payload fields.

## 8. Report Behavior

When hook invocation IDs are supplied:

- generated reports should include hook citations in `ValidationAndQualityChecks`;
- citation target should be `WorkReportCitationTarget::AgentHarnessHook`;
- citation kind should be `WorkReportCitationKind::AgentHarnessHook`;
- citation summary should remain bounded and generic;
- no `AgentHarnessHookInvocationResult` value should be created, resolved, or copied;
- no `AgentHarnessHookAuditRecord` value should be created, resolved, or copied.

When no hook invocation IDs are supplied:

- generated reports should preserve current behavior;
- no hook citation should be added;
- no missing hook citation should be invented by default.

## 9. Workflow Semantics Boundary

The implementation must not:

- mutate `WorkflowRun`;
- mutate `WorkflowRunSnapshot`;
- append workflow events;
- emit audit events;
- emit observability events;
- invoke hooks;
- call `invoke_agent_harness_hook(...)`;
- touch `StateBackend` beyond existing executor behavior;
- write report artifacts;
- persist hook records;
- read hook records from storage;
- expose CLI output;
- change workflow pass/fail behavior;
- change existing `execute(...)`, `decide_approval(...)`, or `cancel_run(...)` behavior.

Report generation failure must remain separate from workflow execution failure in `LocalExecutionWithReportResult`.

## 10. Error Handling

Because the new input field should use `AgentHarnessHookInvocationId`, invalid raw ID values fail before the executor propagation boundary.

If report generation fails after hook invocation IDs are supplied:

- preserve the run;
- return `work_report: None`;
- return `report_generation_error: Some(...)`;
- avoid leaking hook invocation IDs, notes, paths, tokens, hook disclosures, hook output summaries, raw payloads, command output, parser output, or provider data in errors;
- do not convert report-generation errors into workflow diagnostics;
- do not append events or audit records for the report-generation failure.

## 11. Privacy And Payload Policy

Executor report inputs must remain reference-only.

The implementation must not copy:

- hook audit records;
- hook invocation results;
- hook contracts;
- hook disclosures;
- hook input references;
- hook output references;
- supplemental hook references;
- workflow/run/actor context from hook records;
- hook output summaries;
- raw prompts;
- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira or GitHub bodies;
- raw spec contents;
- parser payloads;
- environment variable values;
- credentials, authorization headers, private keys, or token-like values.

Hook invocation IDs are stable references. Future artifact, schema, CLI, or SDK exposure must treat them as sensitive enough to redact in Debug and review before public contract exposure.

## 12. Relationship To Runtime Hook Execution

This propagation phase must not execute hooks.

The executor should consume IDs supplied by a caller. It should not:

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

Hook invocation IDs should not be modeled as `AuditEvent` or `WorkflowEvent` citations in this phase.

Reasons:

- hook workflow event vocabulary is implemented, but executor append behavior is not implemented;
- hook audit sink emission is not implemented;
- `AgentHarnessHookAuditRecord` is model-only and not a persisted audit ledger entry;
- using audit or workflow event citations would fabricate runtime history.

Future hook event phases should separately define event kinds, ordering, idempotency, state projection, and audit sink semantics.

## 14. Test Plan

Implementation tests cover:

- `LocalExecutionReportInputs` accepts `agent_harness_hook_invocation_ids`;
- `execute_with_report(...)` forwards supplied hook invocation IDs into generated report citations;
- generated citation target is `WorkReportCitationTarget::AgentHarnessHook`;
- generated citation kind is `WorkReportCitationKind::AgentHarnessHook`;
- hook citations appear in `ValidationAndQualityChecks`;
- absent hook invocation IDs preserve current no-hook behavior;
- Debug for `LocalExecutionReportInputs` reports only count and does not leak hook invocation IDs;
- Debug for `LocalExecutionWithReportRequest` remains redaction-safe;
- report-generation failure preserves run and event history;
- no runtime events are appended for hook input propagation;
- no `StateBackend` write beyond existing executor execution behavior;
- no report artifacts are written automatically;
- no hook invocation result is created, resolved, or copied;
- no hook audit record is created, resolved, or copied;
- no hook payload markers appear in Debug or serialized generated reports;
- existing WorkReport, hook, executor, artifact, evidence, diagnostic, validation, adapter telemetry, and runtime tests still pass.

## 15. Implementation Sequence

Completed:

1. Added `agent_harness_hook_invocation_ids: Vec<AgentHarnessHookInvocationId>` to `LocalExecutionReportInputs`.
2. Imported `AgentHarnessHookInvocationId` in executor code.
3. Updated `LocalExecutionReportInputs` Debug with `agent_harness_hook_count`.
4. Forwarded `report.agent_harness_hook_invocation_ids.clone()` in `terminal_report_input_for_run(...)`.
5. Updated existing test helpers that construct `LocalExecutionReportInputs`.
6. Added focused executor tests for propagation, redaction-safe Debug, absence behavior, and non-mutation.
7. Updated docs and created an end-of-phase report.
8. Ran full validation.

## 16. Open Questions

- Should executor hook IDs eventually come from explicit report input only, or from separately accepted hook workflow events?
- Should hook citations remain in `ValidationAndQualityChecks` until hook kind/status context is available?
- Should report contracts eventually require hook citations for selected workflows?
- Should missing hook citations become explicit missing-citation records after contract enforcement exists?
- How should future runtime hook execution preserve event ordering and idempotency?
- Should hook audit records be persisted directly, projected from workflow events, or both?
- What public compatibility guarantees should apply before hook citation inputs are exposed through schema, SDK, or CLI surfaces?

## 17. Final Recommendation

Proceed next with **executor hook report input propagation review**.

That review should verify the additive executor input, forwarding behavior, Debug redaction, report citation placement, absence behavior, non-mutation guarantees, and documentation honesty before runtime hook execution or hook audit/event integration is considered.
