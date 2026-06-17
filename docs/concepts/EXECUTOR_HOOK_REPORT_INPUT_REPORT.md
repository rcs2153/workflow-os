# Executor Hook Report Input Propagation Report

Report date: 2026-06-16

## 1. Executive Summary

Implemented explicit executor-integrated hook report input propagation. `LocalExecutionReportInputs` now accepts caller-supplied `AgentHarnessHookInvocationId` values, and `terminal_report_input_for_run(...)` forwards those IDs into `TerminalLocalWorkReportInput` so the existing terminal report helper can cite them in generated in-memory WorkReports.

The implementation is input propagation only. It does not execute hooks, discover hook records automatically, create hook invocation IDs, create hook invocation results, create hook audit records, append workflow events, emit audit sink records, persist hook records, change report artifact behavior, expose CLI behavior, add workflow schema fields, model side effects, add writes, introduce recursive agents, introduce agent swarms, or change release posture.

## 2. Scope Completed

- Added `agent_harness_hook_invocation_ids` to `LocalExecutionReportInputs`.
- Added count-only `agent_harness_hook_count` to `LocalExecutionReportInputs` Debug output.
- Forwarded caller-supplied hook invocation IDs from `LocalExecutionReportInputs` into `TerminalLocalWorkReportInput`.
- Preserved behavior when no hook invocation IDs are supplied.
- Added focused executor tests proving forwarded hook IDs appear as WorkReport hook citations.
- Added tests confirming Debug output does not leak hook invocation IDs.
- Added tests confirming generated reports do not copy hook payload markers.
- Added tests confirming event history and report artifact behavior remain unchanged.

## 3. Scope Explicitly Not Completed

- No runtime hook execution.
- No executor-integrated hook invocation.
- No automatic hook citation discovery.
- No hook invocation ID creation.
- No hook invocation result creation.
- No hook audit record creation.
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
- No `EvidenceReference` creation or attachment.
- No approval evidence attachment.
- No reasoning lineage implementation.
- No side-effect boundary implementation.
- No writes.
- No recursive agents.
- No agent swarms.
- No release posture change.

## 4. API Summary

`LocalExecutionReportInputs` now includes:

```rust
pub agent_harness_hook_invocation_ids: Vec<AgentHarnessHookInvocationId>
```

The executor accepts already-validated hook invocation IDs only. It does not accept raw string IDs, hook invocation results, hook audit records, hook contracts, hook disclosures, hook input/output references, output summaries, or hook payloads.

## 5. Propagation Boundary Summary

`terminal_report_input_for_run(...)` now forwards:

```rust
agent_harness_hook_invocation_ids: report.agent_harness_hook_invocation_ids.clone()
```

The executor remains a reference propagation boundary. It does not execute hooks, read hook records, validate hook record existence, infer hook IDs from workflow or audit events, or fabricate missing IDs.

## 6. Report Behavior Summary

When hook invocation IDs are supplied to `LocalExecutionReportInputs`, generated reports include `WorkReportCitationTarget::AgentHarnessHook` citations in `ValidationAndQualityChecks` through the already-implemented terminal report helper.

When no hook invocation IDs are supplied, generated reports preserve the existing explicit not-available behavior and do not invent missing hook citations.

## 7. Workflow Semantics Summary

The implementation preserves existing workflow semantics:

- `execute(...)` still returns `WorkflowRun`.
- `execute_with_report(...)` still calls existing execution first.
- Report generation failure remains separate from workflow execution failure.
- No workflow events are appended for hook input propagation.
- No audit or observability events are emitted for hook input propagation.
- No report artifacts are written automatically.
- No `StateBackend` writes are added beyond existing executor behavior.

## 8. Redaction And Privacy Summary

`LocalExecutionReportInputs` Debug output reports only `agent_harness_hook_count` and does not include hook invocation IDs.

Generated report serialization includes valid hook invocation IDs as stable citation references, but it does not include hook audit record fields, hook disclosures, hook input/output payloads, output summaries, provider payloads, command output, parser payloads, raw spec contents, environment values, credentials, authorization headers, private keys, or token-like values.

## 9. Test Coverage Summary

Added focused executor coverage for:

- forwarding supplied hook invocation IDs through `execute_with_report(...)`;
- citation target `WorkReportCitationTarget::AgentHarnessHook`;
- citation kind `WorkReportCitationKind::AgentHarnessHook`;
- placement in `ValidationAndQualityChecks`;
- count-only `LocalExecutionReportInputs` Debug behavior;
- generated report serialization not copying hook payload markers;
- event history preservation;
- absence of automatic report artifacts.

Existing WorkReport, hook, executor, artifact, evidence, diagnostic, validation, adapter telemetry, runtime, CLI, and docs tests remain covered by the validation commands below.

## 10. Commands Run And Results

- `cargo fmt --all --check`
  - Passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Passed.
- `cargo test --workspace`
  - Passed.
- `npm run check:docs`
  - Passed.
- `git diff --check`
  - Passed.

## 11. Remaining Known Limitations

- Runtime hook execution is not implemented.
- Hook workflow events are not implemented.
- Hook audit sink emission is not implemented.
- Hook audit records are not persisted.
- Workflow schema support for hooks is not implemented.
- CLI hook commands are not implemented.
- Hook citation routing remains fixed to `ValidationAndQualityChecks` until hook kind/status context is represented.
- Side effects and writes remain unsupported.

## 12. Recommended Next Phase

Recommended next phase: **executor hook report input propagation review**.

That review should verify the additive API field, forwarding behavior, Debug redaction, report citation placement, non-mutation guarantees, tests, and documentation honesty before any runtime hook execution or hook audit/event work is planned.
