# Local Check Handler Infrastructure Report

Report date: 2026-06-15

Fix-forward note: the subsequent infrastructure review found that
`LocalCheckProcessOutput` derived `Debug` while holding raw pre-redaction
stdout/stderr bytes. That blocker was fixed in the local check handler
infrastructure blocker-fix phase. The original phase scope and validation
record are preserved below.

## 1. Executive Summary

Workflow OS now has reusable local check handler infrastructure while remaining dogfood-only for executable local checks.

This phase added a structured `LocalCheckResult` model, an injectable `LocalCheckProcessRunner` boundary, validated process request/output types, shared bounded output capture, and deterministic tests for success, failure, timeout, secret-like output, runner failure, and environment rejection.

No broader command handlers were added. `WorkflowOsValidateDogfood` remains the only executable local check path, and it remains explicitly test-only.

## 2. Governance Run

This phase was governed by the self-governance dogfood workflow before implementation.

- State directory: `/tmp/workflow-os-local-check-infra.pWlWxa`
- Run ID: `run-1781499735680258000-2`
- Workflow ID: `dg/d`
- Approval ID: `approval/run-1781499735680258000-2/d`
- Final status: `Completed`

Inspection confirmed event history through `RunCompleted`.

## 3. Scope Completed

- Added validated `LocalCheckResult`.
- Added `LocalCheckResultDefinition`.
- Added `LocalCheckProcessRequest`.
- Added `LocalCheckProcessOutput`.
- Added `LocalCheckProcessRunner`.
- Added an internal standard process runner used by the test-only dogfood handler.
- Added `TestOnlyWorkflowOsValidateDogfoodHandler::new_with_process_runner(...)` for deterministic tests.
- Refactored the test-only dogfood handler to derive `SkillOutput` from `LocalCheckResult`.
- Preserved non-shell process execution.
- Preserved explicit test-only handler registration.
- Preserved model-only serialized command contracts.
- Added deterministic tests for passed, failed, timed-out, secret-like output, runner failure, and environment rejection paths.

## 4. Scope Explicitly Not Completed

- No production local check handlers.
- No additional command handlers.
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
- No command-output evidence policy.
- No side-effect boundary implementation.
- No source writes.
- No write-capable adapters.
- No provider calls or live adapter execution.
- No recursive agents or agent swarms.
- No hosted or distributed runtime behavior.
- No production self-hosting claim.
- No release posture change.

## 5. Model And API Summary

New local check infrastructure:

- `LocalCheckResult`
- `LocalCheckResultDefinition`
- `LocalCheckProcessRequest`
- `LocalCheckProcessOutput`
- `LocalCheckProcessRunner`

`LocalCheckResult` captures:

- command ID;
- command kind;
- result status;
- optional exit code;
- duration in milliseconds;
- bounded stdout summary;
- bounded stderr summary;
- truncation flags;
- optional stable error code.

`LocalCheckProcessRunner` allows tests to inject deterministic process outcomes while the standard runner still executes an explicit executable plus argument vector without invoking a shell.

## 6. Validation Boundary Summary

Validation now occurs before local check results or process requests are accepted.

- Result summaries are bounded.
- Secret-like result summaries are rejected.
- Result error codes are stable and bounded.
- Process request arguments use existing command token validation.
- Process request environment keys and values are validated.
- Secret-like environment keys or values are rejected.
- Process requests require a non-zero timeout.

Errors use stable codes and avoid echoing raw output, command text, paths, environment values, or secret-like payloads.

## 7. Output And Redaction Summary

The handler captures only bounded stdout and stderr summaries. Raw output is not persisted, and full command transcripts are not stored.

Secret-like stdout or stderr fails closed with stable code `local_check.output.secret_like`. The failure does not include the raw output value.

`LocalCheckResult` `Debug` redacts stdout and stderr summaries.

## 8. Runtime And Event Summary

The test-only handler still runs through the existing local executor skill path when explicitly registered in tests.

No new runtime event kinds were added. No post-terminal events are appended. No report artifacts are written. No CLI behavior or workflow schema surface was added.

## 9. Test Coverage Summary

Added or preserved focused coverage for:

- valid structured local check result;
- failed local check result;
- secret-like stdout summary rejection;
- secret-like stderr summary rejection;
- unbounded summary rejection;
- redaction-safe `Debug`;
- serde round trip and invalid serde failure for `LocalCheckResult`;
- injected runner success mapping to `passed`;
- injected runner non-zero exit mapping to `failed`;
- injected runner timeout mapping to `timed_out`;
- injected runner failure returning a stable non-leaking error;
- secret-like stdout/stderr process output rejection;
- sanitized environment construction with only `PATH` for dogfood handler execution;
- secret-like environment key rejection;
- existing dogfood handler executor path still passes.

## 10. Commands Run And Results

- Self-governance dogfood run and approval:
  - Passed; final status `Completed`.
- `cargo test -p workflow-core --test local_check`
  - Passed.
- Focused executor test:
  - `test_only_local_check_handler_executes_dogfood_validate_through_executor` passed.

Full validation commands for the phase:

- `cargo fmt --all --check`
  - Passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Passed.
- `cargo test --workspace`
  - Passed.
- `npm run check:docs`
  - Passed.

## 11. Remaining Known Limitations

- The executable local check path remains test-only.
- No non-dogfood command handler is implemented.
- No production sandbox exists.
- Timeout behavior still uses local process polling in the standard runner.
- Environment sanitization is minimal and currently only supplies `PATH`.
- Output redaction is secret-like rejection, not a comprehensive redaction engine.
- No source-file snapshot test exists for broader commands because broader commands are still deferred.
- `LocalCheckResult` is not yet attached to evidence or work reports.

## 12. Recommended Next Phase

Recommended next phase: **local check handler infrastructure review**.

The review should verify the structured result model, process-runner abstraction, redaction behavior, environment validation, dogfood-only execution posture, tests, and documentation before any broader local check command handlers are planned or implemented.
