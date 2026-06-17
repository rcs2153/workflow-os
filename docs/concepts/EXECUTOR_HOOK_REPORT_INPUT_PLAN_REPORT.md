# Executor Hook Report Input Propagation Plan Report

Report date: 2026-06-16

## 1. Executive Summary

Created the executor hook report input propagation plan. The plan defines a narrow future implementation that would let `LocalExecutor::execute_with_report(...)` callers supply existing `AgentHarnessHookInvocationId` values through `LocalExecutionReportInputs`, then forward those IDs into `TerminalLocalWorkReportInput` for existing terminal report helper citation construction.

Fix-forward note: the planned propagation has since been implemented in the bounded executor hook report input propagation phase. Runtime hook execution, hook workflow events, audit sink emission, persistence, CLI behavior, schemas, side effects, writes, recursive agents, agent swarms, and release posture changes remain unimplemented.

This is planning only. No executor API changes, runtime hook execution, hook invocation result creation, hook audit record creation, workflow events, audit sink emission, persistence, CLI behavior, schemas, side effects, writes, recursive agents, agent swarms, or release posture changes were implemented.

## 2. Scope Completed

- Documented the current gap between terminal report helper hook citation support and executor report input propagation.
- Proposed the additive `LocalExecutionReportInputs::agent_harness_hook_invocation_ids` field.
- Defined the intended forwarding boundary in `terminal_report_input_for_run(...)`.
- Defined Debug/redaction expectations for executor report inputs.
- Defined report behavior for supplied and absent hook invocation IDs.
- Defined workflow semantics boundaries.
- Defined privacy and payload restrictions.
- Defined relationship boundaries with runtime hook execution, audit events, and workflow events.
- Defined future implementation tests and sequence.

## 3. Scope Explicitly Not Completed

- No implementation.
- No executor API change.
- No runtime hook execution.
- No executor-integrated hook invocation.
- No hook invocation ID creation.
- No hook invocation result creation.
- No hook audit record creation.
- No workflow event kinds.
- No workflow event append behavior.
- No audit sink emission.
- No hook persistence.
- No report artifact behavior changes.
- No CLI behavior.
- No workflow schema fields.
- No automatic local check execution.
- No command-output evidence.
- No `EvidenceReference` creation or attachment.
- No approval evidence attachment.
- No reasoning lineage implementation.
- No side-effect boundary implementation.
- No writes.
- No recursive agents.
- No agent swarms.
- No release posture change.

## 4. Planning Summary

The plan recommends one future additive Rust API field:

```rust
pub agent_harness_hook_invocation_ids: Vec<AgentHarnessHookInvocationId>
```

The future implementation should forward that field into the already-implemented terminal report helper:

```rust
agent_harness_hook_invocation_ids: report.agent_harness_hook_invocation_ids.clone()
```

The executor must remain a propagation boundary only. It must not execute hooks, discover hook records, create hook IDs, create hook invocation results, create hook audit records, append events, write artifacts, or change workflow semantics.

## 5. Validation Boundary Summary

The plan requires the future API to accept typed `AgentHarnessHookInvocationId` values rather than raw strings. Invalid or secret-like hook invocation IDs should fail before executor propagation.

If report generation fails after a run exists, `LocalExecutionWithReportResult` should preserve the run and expose a structured non-leaking report-generation error, matching existing report-bearing execution behavior.

## 6. Redaction And Privacy Summary

The plan keeps executor report inputs reference-only. It explicitly forbids copying hook audit records, hook invocation results, hook disclosures, hook input/output references, hook output summaries, raw prompts, raw provider payloads, command output, raw spec contents, parser payloads, environment values, credentials, authorization headers, private keys, or token-like values.

Debug output should expose only a count for supplied hook invocation IDs.

## 7. Test Coverage Plan Summary

The future implementation should test:

- executor input acceptance of hook invocation IDs;
- forwarding through `execute_with_report(...)`;
- citation target and kind correctness;
- placement in `ValidationAndQualityChecks`;
- absence behavior;
- Debug non-leakage;
- report-generation failure preserving run/event history;
- no extra state writes, events, artifacts, or hook execution;
- no hook payload copying;
- existing workspace test coverage.

## 8. Commands Run And Results

- `npm run check:docs`
  - Passed.
- `git diff --check`
  - Passed.

## 9. Remaining Known Limitations

- Executor hook report input propagation has since been implemented in a bounded follow-up phase.
- Runtime hook execution is not implemented.
- Hook workflow events are not implemented.
- Hook audit sink emission is not implemented.
- Hook audit record persistence is not implemented.
- Workflow schema support for hooks is not implemented.
- CLI hook commands are not implemented.
- Side effects and writes remain unsupported.

## 10. Recommended Next Phase

Recommended next phase at planning time: **executor hook report input propagation implementation**.

That phase added only the explicit executor report input field and forwarding behavior, with focused tests and documentation. Runtime hook execution and hook audit/event semantics remain separate future phases.
