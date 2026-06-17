# Executor BeforeReport Hook Integration Report

Report date: 2026-06-17

## 1. Executive Summary

Implemented the first executor-integrated agent harness hook checkpoint: explicit `BeforeReport` hook execution on the report-bearing local executor path.

The implementation is narrow, additive, in-memory only, and non-mutating. It applies only to `LocalExecutor::execute_with_report(...)` when callers provide an explicit `before_report_hook` input. Existing `execute(...)`, approval, cancellation, runtime event, audit sink, persistence, artifact, CLI, schema, local check, adapter, side-effect, write, recursive-agent, agent-swarm, and release-posture behavior remains unchanged.

## 2. Scope Completed

- Added `LocalExecutionBeforeReportHookInput`.
- Added optional `before_report_hook` to `LocalExecutionReportInputs`.
- Executed the supplied hook only after `execute(...)` returns a terminal run and before terminal report generation.
- Required the hook kind to be `AgentHarnessHookKind::BeforeReport`.
- Required the hook invocation identity fields to match the terminal run identity.
- Called `execute_runtime_agent_harness_hook(...)` through the existing hook validation boundary.
- Forwarded successful hook invocation IDs into generated `WorkReport` citations.
- Preserved the run and returned a report-path error when hook execution fails.
- Added focused executor tests.
- Updated roadmap and planning docs honestly.

## 3. Scope Explicitly Not Completed

- No hooks on `execute(...)`.
- No automatic hooks for all runs.
- No pre-skill hooks.
- No step, approval, retry, escalation, cancellation, or post-report hooks.
- No hook workflow events.
- No audit sink emission for hook records.
- No hook persistence.
- No automatic report artifact writing.
- No workflow schema fields.
- No workflow-declared hook configuration.
- No runtime hook config.
- No CLI hook commands.
- No automatic local check execution.
- No command execution.
- No adapter invocation.
- No `EvidenceReference` creation or attachment.
- No approval request or decision creation.
- No reasoning lineage.
- No side-effect boundary implementation.
- No writes.
- No recursive agents or agent swarms.
- No hosted or distributed runtime claims.
- No release posture change.

## 4. API Summary

New executor-adjacent input:

```rust
pub struct LocalExecutionBeforeReportHookInput {
    pub hook_invocation_id: AgentHarnessHookInvocationId,
    pub invocation: AgentHarnessHookInvocationInput,
}
```

`LocalExecutionReportInputs` now includes:

```rust
pub before_report_hook: Option<LocalExecutionBeforeReportHookInput>
```

This keeps hook execution explicit and report-path-scoped.

## 5. Runtime Behavior Summary

`LocalExecutor::execute_with_report(...)` now:

1. calls existing `execute(...)`;
2. skips hook execution for non-terminal runs;
3. for terminal runs with a supplied `before_report_hook`, validates kind and run identity;
4. calls `execute_runtime_agent_harness_hook(...)`;
5. forwards the successful hook invocation ID to terminal report generation;
6. returns `run + None + report_generation_error` if hook execution fails.

Hook failure does not alter the workflow run result.

## 6. Event, Audit, And Persistence Summary

The implementation does not append workflow events, emit audit sink records, emit observability records, write hook records, write report artifacts, or mutate snapshots for hook execution.

The existing workflow event log remains the source of truth. The hook execution result remains in memory and is represented in the report only by its stable hook invocation ID citation when successful.

## 7. Privacy And Redaction Summary

The implementation keeps the boundary reference-first and constructor-driven.

It does not copy raw prompts, raw spec contents, raw command output, provider payloads, parser payloads, environment values, credentials, tokens, unbounded summaries, or hook execution transcripts into executor output or reports.

Errors use stable codes and do not include raw hook IDs or reference payload values.

## 8. Test Coverage Summary

Focused tests cover:

- successful `BeforeReport` hook execution and report citation;
- failed terminal run plus hook citation;
- non-terminal run skips hook execution and returns the existing not-terminal report error;
- hook side-effect request failure preserves run and events;
- hook identity mismatch fails without leaking mismatched values;
- duplicate run/report execution does not append events or repeat skill execution;
- no report artifacts are written;
- audit and observability sinks do not receive hook records;
- existing executor report behavior still works.

## 9. Commands Run And Results

- `cargo fmt --all`
  - Passed.
- `cargo test -p workflow-core --test local_executor`
  - Passed.
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

## 10. Remaining Known Limitations

- Only `BeforeReport` is implemented.
- Hook execution is explicit and report-path-only.
- Hook audit records are not persisted or emitted to audit sinks.
- Hook workflow events are not modeled.
- Pre-skill, approval, retry, escalation, cancellation, and post-report checkpoints remain deferred.
- Workflow schema and CLI hook surfaces remain deferred.

## 11. Recommended Next Phase

Recommended next phase: **executor BeforeReport hook integration review**.

After review, future work should plan hook workflow event semantics before any state-mutating hook checkpoint is implemented.
