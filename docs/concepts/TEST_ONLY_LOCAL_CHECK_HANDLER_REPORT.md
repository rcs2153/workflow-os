# Test-Only Local Check Handler Report

Report date: 2026-06-14

## 1. Executive Summary

Workflow OS now has a test-only local check handler for `WorkflowOsValidateDogfood`.

The handler is explicitly named `TestOnlyWorkflowOsValidateDogfoodHandler`, is never registered by default, and executes only the canonical dogfood validation command through direct process invocation. It is intended to prove the handler boundary in focused tests before any production local check execution, CLI exposure, workflow schema changes, automatic dogfood execution, or broader command families are considered.

## 2. Governance Run

This phase was governed by the self-governance dogfood workflow before implementation.

- State directory: `/tmp/workflow-os-self-governance-state.wHbJJc`
- Run ID: `run-1781498595052678000-2`
- Workflow ID: `dg/d`
- Approval ID: `approval/run-1781498595052678000-2/d`
- Final status: `Completed`

Inspection confirmed event history through `RunCompleted`.

## 3. Scope Completed

- Added `TestOnlyWorkflowOsValidateDogfoodHandler`.
- Required explicit construction with:
  - a validated `LocalCheckCommandContract`;
  - an existing local `workflow-os` binary path;
  - an explicit repository root.
- Required `LocalCheckCommandKind::WorkflowOsValidateDogfood`.
- Required repository-root working-directory policy.
- Required disabled network policy.
- Required no-source-writes side-effect classification.
- Executed the existing canonical argument vector with no shell.
- Used `env_clear()` and a minimal PATH.
- Captured bounded stdout/stderr summaries.
- Rejected secret-like command output before returning skill output.
- Returned local check status, exit code, duration, bounded summaries, and truncation flags as normal `SkillOutput` values.
- Added focused tests for unsupported-kind rejection, Debug redaction, and explicit local executor registration/execution.

## 4. Scope Explicitly Not Completed

- No production local check handler.
- No default handler registration.
- No broad handler discovery.
- No arbitrary shell execution.
- No user-supplied command text.
- No CLI exposure.
- No workflow schema changes.
- No automatic check execution.
- No automatic report generation.
- No report artifact writing.
- No evidence attachment.
- No side-effect boundary implementation.
- No source writes.
- No provider calls or live adapter execution.
- No recursive agents or agent swarms.
- No hosted or distributed runtime behavior.
- No production self-hosting claim.
- No release posture change.

## 5. Handler API Summary

The handler API is intentionally narrow:

- `TestOnlyWorkflowOsValidateDogfoodHandler::new(contract, workflow_os_binary, repository_root)`
- implements the existing `SkillHandler` trait;
- returns existing `SkillOutput`;
- uses the existing local executor event path when registered in tests.

The handler keeps the serialized/local-check contract model at `ModelOnly`; execution authority comes from the explicit test-only handler type and explicit test registration, not from broadening `LocalCheckExecutionPosture`.

## 6. Command Boundary Summary

The handler:

- uses `std::process::Command` with executable path and argument vector;
- does not invoke a shell;
- does not concatenate command strings;
- does not accept caller-supplied extra arguments;
- validates the contract before execution;
- only supports the canonical dogfood validation command;
- uses an explicit repository-root working directory;
- clears the environment and sets a minimal PATH;
- rejects unsupported command kinds before execution.

## 7. Output And Redaction Summary

The handler captures bounded stdout and stderr summaries according to the contract output policy. It does not persist raw output or command transcripts.

If bounded output contains secret-like terms, the handler fails closed with stable code `local_check.output.secret_like` and does not return the output text.

Debug output for the handler redacts local binary and repository paths.

## 8. Runtime And Event Summary

The handler is used only by explicit test registration through `LocalSkillRegistry`.

When used by the local executor, it emits only the normal existing workflow events for a local skill invocation. It does not add new event kinds, append post-terminal events, mutate runtime state outside the normal workflow run, write report artifacts, or expose CLI output.

## 9. Test Coverage Summary

Added focused tests for:

- unsupported command kind rejection without leaking local paths;
- handler Debug redaction of local paths;
- explicit local executor registration for `local/check-dogfood`;
- successful dogfood validation through the normal local executor skill path;
- persisted event log matching the returned run;
- no work report artifacts written by the handler path.

Existing local-check tests continue to cover canonical template binding, model-only execution posture rejection, shell metacharacter rejection, secret-like value rejection, output capture bounds, serde behavior, and Debug non-leakage.

## 10. Commands Run And Results

- Self-governance dogfood run and approval:
  - Passed; final status `Completed`.
- `cargo test -p workflow-core --test local_check`
  - Passed.
- Focused executor test:
  - `test_only_local_check_handler_executes_dogfood_validate_through_executor` passed.

Full validation commands for the phase:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`

Results are recorded in the final implementation report.

## 11. Remaining Known Limitations

- The handler is test-only.
- It is not registered by default.
- It is not exposed through CLI.
- It has no workflow schema surface.
- It has no production sandbox.
- Timeout behavior is implemented with local process polling, not an OS-level sandbox.
- Environment sanitization is intentionally minimal for the first slice.
- Output redaction is secret-like rejection, not a full redaction engine.
- Failed dogfood validation is not yet covered through a controlled invalid dogfood fixture.
- Additional command kinds remain deferred.

## 12. Recommended Next Phase

Recommended next phase: **test-only local check handler review**.

The review should verify that the handler is explicit, narrow, non-shell, non-default, redaction-safe, and does not introduce CLI exposure, schema changes, automatic execution, report artifacts, source writes, side-effect boundary implementation, recursive agents, agent swarms, production self-hosting, or release posture changes.
