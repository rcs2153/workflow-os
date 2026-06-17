# Executor Hook Disclosure Report Input Propagation Report

## 1. Executive Summary

Executor-integrated report-bearing execution can now forward explicitly supplied `AgentHarnessHookDisclosureId` values into generated terminal local WorkReports.

This is input propagation only. It does not execute hooks, discover disclosures automatically, create disclosures, append workflow events, emit audit sink records, persist hook disclosures, write report artifacts automatically, expose CLI output, change schemas, implement side effects, add writes, implement reasoning lineage, add hosted behavior, or change release posture.

## 2. Scope Completed

- Added `agent_harness_hook_disclosure_ids` to `LocalExecutionReportInputs`.
- Updated `LocalExecutionReportInputs` `Debug` to expose only `agent_harness_hook_disclosure_count`.
- Forwarded supplied hook disclosure IDs from `LocalExecutionReportInputs` into `TerminalLocalWorkReportInput`.
- Preserved existing executor behavior when no hook disclosure IDs are supplied.
- Added focused executor tests for hook disclosure propagation, coexistence with hook invocation IDs, redaction-safe debug output, report citation behavior, non-mutation, and no automatic report artifacts.
- Updated roadmap and concept/planning docs to reflect implemented executor propagation for explicitly supplied hook disclosure IDs.

## 3. Scope Explicitly Not Completed

- Runtime hook execution changes are not implemented.
- Automatic hook disclosure discovery is not implemented.
- Hook disclosure creation from reports is not implemented.
- Hook invocation result creation from reports is not implemented.
- Hook audit record creation or persistence is not implemented.
- Workflow event append behavior is not implemented for hook disclosures.
- Audit sink emission is not implemented.
- Warning, skipped, blocked, optional, or policy-controlled continuation behavior is not implemented.
- Context-aware disclosure section routing by kind or severity is not implemented.
- `EvidenceReference` creation or attachment is not implemented.
- Approval evidence attachment is not implemented.
- Report artifact behavior changes are not implemented.
- CLI hook commands, report rendering, workflow schema fields, side effects, writes, reasoning lineage, recursive agents, agent swarms, hosted behavior, and release posture changes are not implemented.

## 4. API Summary

`LocalExecutionReportInputs` now includes:

```rust
pub agent_harness_hook_disclosure_ids: Vec<AgentHarnessHookDisclosureId>
```

The field accepts already-validated typed IDs only. It does not accept raw strings, full `AgentHarnessHookDisclosure` values, disclosure title, disclosure summary, references, redaction metadata, hook context, hook audit records, workflow events, or persistence handles.

## 5. Propagation Summary

`terminal_report_input_for_run(...)` now forwards:

```rust
agent_harness_hook_disclosure_ids: report.agent_harness_hook_disclosure_ids.clone()
```

The terminal report helper then uses the already implemented `WorkReportCitationTarget::AgentHarnessHookDisclosure` path to place hook disclosure citations in `ValidationAndQualityChecks`.

## 6. Workflow Semantics Summary

The executor still:

- calls `execute(&request.execution)` first;
- returns execution errors unchanged;
- preserves the run when report generation fails after execution;
- does not mutate workflow state for report generation;
- does not append events for report generation;
- does not invoke hooks because disclosure IDs were supplied;
- does not create hook disclosures, hook invocation results, or hook audit records;
- does not write report artifacts automatically;
- does not expose CLI output.

## 7. Redaction And Privacy Summary

`LocalExecutionReportInputs` `Debug` exposes only a disclosure count, not the IDs themselves.

The implementation does not copy:

- hook disclosure title or summary;
- hook disclosure references;
- hook disclosure redaction metadata;
- hook input/output references;
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

Generated WorkReports serialize stable disclosure IDs as citation targets only, which matches the existing reference-first report model.

## 8. Test Coverage Summary

Focused tests cover:

- `execute_with_report(...)` forwards supplied hook disclosure IDs into generated report citations;
- generated citation target is `WorkReportCitationTarget::AgentHarnessHookDisclosure`;
- generated citation kind is `WorkReportCitationKind::AgentHarnessHookDisclosure`;
- hook disclosure citations appear in `ValidationAndQualityChecks`;
- hook invocation IDs and hook disclosure IDs can coexist in the same generated report;
- absent hook disclosure IDs preserve no-disclosure behavior in the hook invocation test;
- `LocalExecutionReportInputs` debug reports count-only behavior and does not leak hook invocation or hook disclosure IDs;
- generated report serialization does not copy disclosure title, summary, hook audit record markers, or raw provider payload markers;
- run event history is preserved;
- no report artifacts are written automatically.

Existing executor, WorkReport, hook disclosure, hook invocation, EvidenceReference, Diagnostic, validation, adapter telemetry, local-check, and runtime tests are covered by the workspace test suite.

## 9. Commands Run And Results

- `cargo test -p workflow-core --test local_executor` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

## 10. Remaining Known Limitations

- Supplied hook disclosure IDs are not validated against a durable disclosure store.
- Reports do not automatically discover disclosures from hook invocation results, audit records, workflow events, or persistence.
- Disclosure kind and severity do not influence section placement.
- Warning, skipped, blocked, and optional hook continuation semantics remain deferred.
- Dedicated hook audit sink emission and hook persistence remain deferred.
- Approval-resume and cancellation report-bearing APIs do not have hook disclosure input propagation.

## 11. Recommended Next Phase

Recommended next phase: **executor hook disclosure report input propagation review**.

The implementation should be reviewed before any later phase considers automatic discovery, durable disclosure records, context-aware section placement, or warning/skipped semantics.
