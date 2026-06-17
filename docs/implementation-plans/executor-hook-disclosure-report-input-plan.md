# Executor Hook Disclosure Report Input Propagation Plan

Status: Implemented. Terminal report helper hook disclosure citation integration is implemented for explicitly supplied `AgentHarnessHookDisclosureId` values, and executor-integrated report-bearing execution now accepts and forwards caller-supplied hook disclosure IDs into terminal reports. This plan defines the smallest safe implementation for explicit executor report input propagation only. It does not implement runtime hook behavior, automatic disclosure discovery, warning/skipped continuation, blocked behavior, hook optionality, workflow event append behavior, audit sink emission, persistence, report artifacts, CLI rendering, workflow schemas, side effects, writes, reasoning lineage, recursive agents, agent swarms, hosted behavior, or release posture changes.

## 1. Executive Summary

WorkReport citation vocabulary for hook disclosures is implemented and reviewed. The terminal local WorkReport helper can now cite explicitly supplied `AgentHarnessHookDisclosureId` values in generated in-memory reports.

The executor gap was narrow: `LocalExecutionReportInputs` did not accept hook disclosure IDs, and the executor adapter path passed an empty disclosure ID list into `TerminalLocalWorkReportInput`.

This plan defined the smallest safe implementation for explicit executor-integrated hook disclosure report input propagation. That implementation is complete. It does not implement automatic disclosure discovery, runtime hook behavior, event append behavior, audit sink emission, persistence, CLI behavior, report artifact behavior changes, schemas, side effects, writes, reasoning lineage, recursive agents, agent swarms, hosted behavior, or release posture changes.

## 2. Goals

- Let callers of `LocalExecutor::execute_with_report(...)` supply existing `AgentHarnessHookDisclosureId` values.
- Forward those IDs from `LocalExecutionReportInputs` to `TerminalLocalWorkReportInput`.
- Preserve executor semantics and existing `execute(...)` behavior.
- Keep hook disclosure citations explicit, local, deterministic, and in-memory.
- Use existing `AgentHarnessHookDisclosureId`, `WorkReportCitation`, and terminal report helper validation boundaries.
- Avoid copying hook disclosure title, summary, references, redaction metadata, hook context, audit records, event payloads, provider payloads, command output, parser output, file contents, paths, tokens, or unbounded prose into executor inputs, report sections, citations, debug output, or serialization.
- Preserve current behavior when no hook disclosure IDs are supplied.
- Prepare for later warning/skipped disclosure semantics without implementing them here.

## 3. Non-Goals

This plan did not authorize:
- runtime hook execution;
- executor-integrated hook invocation changes;
- automatic hook disclosure discovery;
- hook disclosure creation from reports;
- hook invocation result creation from reports;
- hook audit record creation from reports;
- hook audit record persistence;
- workflow event append behavior;
- audit sink emission;
- warning continuation;
- skipped-with-disclosure continuation;
- blocked runtime behavior;
- hook optionality;
- policy-controlled continuation;
- context-aware disclosure section routing by disclosure kind or severity;
- `EvidenceReference` creation or attachment;
- approval request or approval decision creation;
- approval evidence attachment;
- report artifact behavior changes;
- CLI hook commands or report rendering;
- workflow schema fields;
- automatic local check execution;
- command execution;
- adapter invocation;
- reasoning lineage;
- side-effect boundary implementation;
- writes;
- recursive agents or agent swarms;
- hosted or distributed runtime claims;
- release posture changes.

## 4. Current Baseline

Implemented:

- `AgentHarnessHookDisclosureId`;
- `AgentHarnessHookDisclosure`;
- hook disclosure validation, serde, and redaction-safe `Debug`;
- `WorkReportCitationKind::AgentHarnessHookDisclosure`;
- `WorkReportCitationTarget::AgentHarnessHookDisclosure`;
- terminal local WorkReport generation helper;
- terminal report helper support for supplied `agent_harness_hook_disclosure_ids`;
- executor-integrated report-bearing execution through `LocalExecutor::execute_with_report(...)`;
- executor report input propagation for supplied hook invocation IDs.

Closed gap:

- `LocalExecutionReportInputs` now has an `agent_harness_hook_disclosure_ids` field.
- `terminal_report_input_for_run(...)` now forwards `report.agent_harness_hook_disclosure_ids.clone()`.
- Therefore executor-integrated report-bearing execution can forward caller-supplied hook disclosure references into generated reports.

This gap is input propagation only. It is not runtime hook execution, disclosure discovery, event modeling, or audit persistence.

## 5. API Change

The implementation added one field to `LocalExecutionReportInputs`:

```rust
pub agent_harness_hook_disclosure_ids: Vec<AgentHarnessHookDisclosureId>
```

Rules:

- accept only already-validated `AgentHarnessHookDisclosureId` values;
- do not accept raw strings;
- do not accept `AgentHarnessHookDisclosure` values;
- do not accept disclosure kind, severity, title, summary, references, redaction metadata, or sensitivity;
- do not accept hook input/output references, supplemental hook references, workflow IDs, run IDs, actor IDs, hook audit records, or event payload fields;
- do not read hook disclosures from storage;
- do not validate disclosure existence;
- do not infer disclosure IDs from workflow events, hook audit records, hook invocation results, report notes, local check results, typed handoffs, or validation diagnostics;
- do not fabricate missing hook disclosure IDs.

This is an additive local Rust API change. It must preserve existing executor methods and should update tests and call sites that construct `LocalExecutionReportInputs`.

## 6. Propagation Boundary

The implementation updates `terminal_report_input_for_run(...)` so:

```rust
agent_harness_hook_disclosure_ids: report.agent_harness_hook_disclosure_ids.clone()
```

This should be the only intended behavior change.

The executor should still:

- call `execute(&request.execution)` first;
- return execution errors unchanged;
- preserve `run` when report generation fails after execution;
- never mutate workflow state for report generation;
- never append events for report generation;
- never invoke hooks because disclosure IDs were supplied;
- never create hook disclosures;
- never create hook invocation results;
- never create hook audit records;
- never write report artifacts automatically;
- never expose CLI output.

## 7. Debug And Redaction Policy

The implementation updates `LocalExecutionReportInputs` `Debug` to include only a count:

```rust
.field(
    "agent_harness_hook_disclosure_count",
    &self.agent_harness_hook_disclosure_ids.len(),
)
```

Debug output must not include hook disclosure IDs, report IDs, paths, tokens, notes, limitations, risks, handoff text, hook invocation IDs, hook disclosure titles, summaries, references, hook output summaries, or payload-like strings.

Serialization is not currently the primary surface for executor report inputs. If serialization is added later, hook disclosure IDs must be treated as potentially sensitive stable references and must not be mixed with disclosure payload fields.

## 8. Report Behavior

When hook disclosure IDs are supplied:

- generated reports should include hook disclosure citations in `ValidationAndQualityChecks`;
- citation target should be `WorkReportCitationTarget::AgentHarnessHookDisclosure`;
- citation kind should be `WorkReportCitationKind::AgentHarnessHookDisclosure`;
- citation summary should remain bounded and generic;
- no `AgentHarnessHookDisclosure` value should be created, resolved, or copied;
- no `AgentHarnessHookInvocationResult` value should be created, resolved, or copied;
- no `AgentHarnessHookAuditRecord` value should be created, resolved, or copied.

When no hook disclosure IDs are supplied:

- generated reports should preserve current behavior;
- no hook disclosure citation should be added;
- no missing hook disclosure citation should be invented by default.

## 9. Workflow Semantics Boundary

The implementation must not:

- mutate `WorkflowRun`;
- mutate `WorkflowRunSnapshot`;
- append workflow events;
- emit audit events;
- emit observability events;
- invoke hooks;
- call `execute_runtime_agent_harness_hook(...)`;
- create hook disclosures;
- create hook invocation results;
- create hook audit records;
- touch `StateBackend` beyond existing executor behavior;
- write report artifacts;
- persist hook disclosures;
- read hook disclosures from storage;
- expose CLI output;
- change workflow pass/fail behavior;
- change existing `execute(...)`, `decide_approval(...)`, or `cancel_run(...)` behavior.

Report generation failure must remain separate from workflow execution failure in `LocalExecutionWithReportResult`.

## 10. Error Handling

Because the new input field should use `AgentHarnessHookDisclosureId`, invalid raw ID values fail before the executor propagation boundary.

If report generation fails after hook disclosure IDs are supplied:

- preserve the run;
- return `work_report: None`;
- return `report_generation_error: Some(...)`;
- avoid leaking hook disclosure IDs, notes, paths, tokens, disclosure title, disclosure summary, hook output summaries, raw payloads, command output, parser output, or provider data in errors;
- do not convert report-generation errors into workflow diagnostics;
- do not append events or audit records for the report-generation failure.

## 11. Privacy And Payload Policy

Executor report inputs must remain reference-only.

The implementation must not copy:

- hook disclosure title or summary;
- hook disclosure references;
- hook disclosure redaction metadata;
- hook input references;
- hook output references;
- supplemental hook references;
- hook audit records;
- hook invocation results;
- hook contracts;
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

Hook disclosure IDs are stable references. Future artifact, schema, CLI, or SDK exposure must treat them as sensitive enough to redact in `Debug` and review before public contract exposure.

## 12. Relationship To Runtime Hook Execution

This propagation phase must not execute hooks.

The executor should consume IDs supplied by a caller. It should not:

- call `execute_runtime_agent_harness_hook(...)`;
- evaluate hook contracts because a disclosure ID exists;
- create disclosures;
- create invocation results;
- create audit records;
- register local check handlers;
- run command hooks;
- read local files;
- call external systems;
- enable any default hook execution profile.

That keeps execution authority separate from report construction.

## 13. Relationship To Audit And Workflow Events

Hook disclosure IDs should not be modeled as `AuditEvent` or `WorkflowEvent` citations in this phase.

Reasons:

- hook workflow event vocabulary is implemented, but disclosure event append behavior is not implemented;
- hook audit sink emission is not implemented;
- `AgentHarnessHookAuditRecord` remains model-only and is not a persisted audit ledger entry;
- using audit or workflow event citations for disclosure IDs would fabricate runtime history.

Future hook event phases should separately define disclosure event kinds, ordering, idempotency, state projection, and audit sink semantics.

## 14. Relationship To Warning And Skipped Semantics

This propagation phase should not interpret hook disclosures as warning, skipped, blocked, optional, or policy-controlled continuation behavior.

Reasons:

- the executor input would carry IDs only;
- disclosure kind and severity are not supplied;
- hook optionality is not implemented;
- warning/skipped continuation semantics remain planning-only;
- context-aware section routing remains deferred.

The report should continue to place forwarded disclosure citations in `ValidationAndQualityChecks` through the terminal report helper.

## 15. Test Plan

Implementation tests cover:

- `LocalExecutionReportInputs` accepts `agent_harness_hook_disclosure_ids`;
- `execute_with_report(...)` forwards supplied hook disclosure IDs into generated report citations;
- generated citation target is `WorkReportCitationTarget::AgentHarnessHookDisclosure`;
- generated citation kind is `WorkReportCitationKind::AgentHarnessHookDisclosure`;
- hook disclosure citations appear in `ValidationAndQualityChecks`;
- supplied hook invocation IDs and hook disclosure IDs can coexist in the same generated report;
- absent hook disclosure IDs preserve current no-disclosure behavior;
- `Debug` for `LocalExecutionReportInputs` reports only count and does not leak hook disclosure IDs;
- `Debug` for `LocalExecutionWithReportRequest` remains redaction-safe;
- report-generation failure preserves run and event history;
- no runtime events are appended for hook disclosure input propagation;
- no `StateBackend` write beyond existing executor execution behavior;
- no report artifacts are written automatically;
- no full `AgentHarnessHookDisclosure` value is created, resolved, or copied;
- no hook disclosure payload markers appear in `Debug` or serialized generated reports;
- executor still does not automatically discover disclosures from hook events, hook audit records, or persistence;
- existing WorkReport, hook disclosure, hook invocation, executor, artifact, EvidenceReference, Diagnostic, validation, adapter telemetry, local-check, and runtime tests still pass.

## 16. Implementation Sequence

1. Added `agent_harness_hook_disclosure_ids: Vec<AgentHarnessHookDisclosureId>` to `LocalExecutionReportInputs`.
2. Imported `AgentHarnessHookDisclosureId` in executor code.
3. Updated `LocalExecutionReportInputs` `Debug` with `agent_harness_hook_disclosure_count`.
4. Forwarded `report.agent_harness_hook_disclosure_ids.clone()` in `terminal_report_input_for_run(...)`.
5. Updated existing test helpers that construct `LocalExecutionReportInputs`.
6. Added focused executor tests for propagation, redaction-safe `Debug`, absence behavior, coexistence with hook invocation IDs, and non-mutation.
7. Updated docs and created an end-of-phase report.
8. Ran full validation.

## 17. Open Questions

- Should executor-integrated hook disclosure propagation eventually validate that supplied IDs exist in a future disclosure store?
- Should supplied disclosure IDs require corresponding hook invocation IDs once durable hook audit/event semantics exist?
- Should missing hook disclosure references ever become explicit `missing=true` citations, or remain section text unless contract requirements change?
- Should future report artifacts verify referential integrity for hook disclosure citations?
- Should hook disclosure IDs appear in CLI JSON only after schema and redaction posture are separately reviewed?
- Should approval-resume and cancellation report-bearing APIs receive the same hook disclosure input later?
- Should warning/skipped disclosure semantics move some disclosures out of `ValidationAndQualityChecks` in a later phase?

## 18. Final Recommendation

Recommended next phase: executor hook disclosure report input propagation review.

The implementation added only the explicit input field, redaction-safe `Debug` count, and forwarding behavior described here. It did not implement runtime hook execution, automatic disclosure discovery, warning/skipped continuation, blocked behavior, hook optionality, event append behavior, audit sink emission, persistence, report artifacts, CLI rendering, workflow schema changes, `EvidenceReference` creation, reasoning lineage, side-effect modeling, writes, recursive agents, agent swarms, hosted behavior, or release posture changes.
