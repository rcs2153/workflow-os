# Deterministic Hook Checkpoint Enforcement Report

## 1. Executive Summary

This phase expands deterministic hook checkpoint enforcement for the explicit local report-bearing executor paths.

Workflow OS can now require an explicit `BeforeReport` hook checkpoint before report generation. The requirement is opt-in through report inputs, remains local and in-memory, and preserves workflow execution semantics. If the checkpoint is required but no `BeforeReport` hook input is supplied, report generation fails closed with a stable non-leaking report-path error while the completed workflow run and event history remain unchanged.

This phase does not implement broad automatic hook invocation, workflow-declared hook configuration, runtime hook configuration, hook persistence, dedicated hook audit sink emission, schemas, CLI behavior, local check execution, adapter invocation, side effects, writes, hosted execution, recursive agents, agent swarms, or release posture changes.

## 2. Scope Completed

- Added `LocalExecutionHookCheckpointInputs`.
- Added `LocalExecutionReportInputs::hook_checkpoints`.
- Implemented `require_before_report` for explicit report-bearing executor paths.
- Reused existing `BeforeReport` hook execution and validation behavior.
- Factored shared `BeforeReport` report-path checkpoint handling for:
  - `LocalExecutor::execute_with_report(...)`;
  - `execute_with_report_and_side_effect_discovery(...)`.
- Preserved optional behavior for existing report-bearing callers.
- Preserved `LocalExecutor::execute(...)` behavior.
- Added focused tests for required checkpoint success and failure behavior.
- Updated roadmap and concept documentation.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- automatic hook invocation for every executor path;
- workflow-declared hook configuration;
- runtime hook configuration;
- broad required `BeforeSkillInvocation` checkpoint policy;
- warning, skipped, or blocked hook continuation;
- hook persistence;
- dedicated hook audit sink emission;
- discovery from workflow events or audit projections;
- post-terminal hook workflow events;
- local check execution;
- command execution;
- adapter invocation;
- side-effect modeling or runtime side-effect execution;
- provider writes;
- CLI behavior;
- workflow schema changes;
- example updates;
- hosted or distributed runtime behavior;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 4. API Summary

The new policy type is:

```rust
pub struct LocalExecutionHookCheckpointInputs {
    pub require_before_report: bool,
}
```

`LocalExecutionReportInputs` now carries:

```rust
pub hook_checkpoints: LocalExecutionHookCheckpointInputs
```

When `require_before_report` is `false`, existing behavior remains unchanged: a supplied `before_report_hook` is executed before report generation, and no hook is required.

When `require_before_report` is `true`, terminal report generation requires a valid `before_report_hook`. If absent, the report-bearing executor path returns the run with no report and `report_generation_error` code:

```text
executor.hook.before_report.required
```

## 5. Runtime Semantics Summary

The enforcement boundary is report-path-only.

The executor still:

- calls existing workflow execution first;
- returns execution failures unchanged when no run exists;
- skips report generation for non-terminal runs through the existing terminal report status error;
- runs or requires `BeforeReport` only after a terminal run exists;
- returns hook requirement or hook validation failures through the report-generation error channel;
- preserves the workflow run status, snapshot, and event history;
- does not append hook events for `BeforeReport`;
- does not emit hook audit sink records;
- does not write report artifacts automatically.

## 6. Privacy And Redaction Summary

The phase reuses existing hook and report constructors and does not copy raw hook payloads into reports.

The required-checkpoint error is stable and non-leaking. It does not include workflow IDs, run IDs, hook IDs, paths, raw specs, command output, provider payloads, parser payloads, credentials, tokens, private keys, or secret-like values.

Debug output continues to redact report and hook context. Serialization behavior for generated reports is unchanged.

## 7. Test Coverage Summary

Added focused tests covering:

- required `BeforeReport` checkpoint missing on `execute_with_report(...)` fails report generation only;
- required `BeforeReport` checkpoint supplied on `execute_with_report(...)` produces a report and hook citation;
- required `BeforeReport` checkpoint missing on `execute_with_report_and_side_effect_discovery(...)` fails report generation only;
- event history is unchanged when a required checkpoint is missing;
- no `BeforeReport` hook workflow events are appended;
- no report artifacts are written;
- existing `BeforeReport` optional behavior remains intact;
- existing `BeforeSkillInvocation`, report, SideEffect, approval, local check, dogfood, and runtime tests continue to pass in the focused local executor suite.

## 8. Commands Run And Results

- `cargo test -p workflow-core --test local_executor` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

## 9. Remaining Known Limitations

- Required checkpoint enforcement is limited to explicit `BeforeReport` report paths.
- `BeforeSkillInvocation` still requires explicit caller-supplied hook input and does not yet have a general required-checkpoint policy.
- `BeforeReport` remains in-memory and non-mutating.
- `BeforeReport` does not append workflow events or emit dedicated hook audit sink records.
- Hook disclosure discovery remains limited to already-validated in-memory `BeforeReport` hook results in explicit report-bearing paths.
- Workflow-declared and runtime-declared hook configuration remain deferred.

## 10. Recommended Next Phase

Recommended next phase: **deterministic hook checkpoint enforcement review**.

This phase is small but security-adjacent because it turns an optional hook into an explicit fail-closed report gate. A focused maintainer review should verify that the new enforcement remains opt-in, deterministic, redaction-safe, and limited to report-bearing executor paths before expanding enforcement to `BeforeSkillInvocation` or broader runtime checkpoints.
