# Local Check Handler Infrastructure Blocker Fix Report

Report date: 2026-06-15

## 1. Executive Summary

The local check handler infrastructure blocker is fixed.

The review found that `LocalCheckProcessOutput` was exported and derived `Debug` while holding raw pre-redaction stdout and stderr bytes. This could leak secret-like process output before the validated `LocalCheckResult` boundary.

This fix replaces derived debug formatting with a custom redaction-safe `Debug` implementation and adds regression tests for completed and timed-out process output.

## 2. Blocker Fixed

Original blocker:

- `LocalCheckProcessOutput` derived `Debug`;
- it stores raw `stdout` and `stderr` bytes;
- those bytes exist before `LocalCheckResult` bounds and validates output summaries;
- debug formatting could therefore expose raw process output.

Fix:

- removed derived `Debug` from `LocalCheckProcessOutput`;
- added custom `Debug` that reports only safe metadata:
  - exit code;
  - success flag;
  - duration;
  - stdout byte count;
  - stderr byte count;
  - timeout flag;
  - redacted stdout/stderr markers.

## 3. Scope Completed

- Made `LocalCheckProcessOutput` debug output redaction-safe.
- Added tests for completed process output with secret-like stdout/stderr.
- Added tests for timed-out process output with secret-like stdout/stderr.
- Preserved existing local check result behavior.
- Preserved test-only dogfood handler behavior.
- Preserved process-runner injection behavior.
- Preserved the dogfood-only executable local check posture.

## 4. Scope Explicitly Not Completed

- No production local check handlers.
- No broader command handlers.
- No default handler registration.
- No CLI behavior.
- No workflow schema fields.
- No automatic local check execution.
- No report artifacts.
- No persistence.
- No evidence attachment.
- No side-effect boundary modeling.
- No source writes.
- No live provider calls.
- No release posture changes.

## 5. Governance Run

This blocker fix was governed by the self-governance dogfood workflow before implementation.

- State directory: `/tmp/workflow-os-local-check-output-debug-fix`
- Run ID: `run-1781501031111144000-2`
- Workflow ID: `dg/d`
- Approval ID: `approval/run-1781501031111144000-2/d`
- Final status: `Completed`

Inspection confirmed event history through `RunCompleted`.

## 6. Validation Boundary Summary

The redaction boundary now has two layers:

- `LocalCheckProcessOutput` debug formatting cannot expose raw pre-redaction stdout/stderr;
- `LocalCheckResult` construction still bounds, validates, and rejects secret-like stdout/stderr summaries before storing reviewed result data.

The process output type remains a raw transport boundary between the runner and the result model, but it no longer leaks raw output through `Debug`.

## 7. Redaction And Privacy Summary

The fix prevents debug leakage of:

- raw provider payload markers;
- command output;
- parser output;
- token-like values;
- authorization headers;
- private-key markers;
- secret-like stdout/stderr text.

The implementation does not persist raw output, write files, emit CLI output, or attach command output to evidence.

## 8. Test Coverage Summary

Added focused tests:

- `local_check_process_output_debug_does_not_leak_raw_output`;
- `local_check_process_timeout_output_debug_does_not_leak_raw_output`.

Existing local check tests continue to cover:

- valid and invalid `LocalCheckResult` construction;
- result debug and serialization behavior;
- injected runner success, failure, timeout, and secret-like output behavior;
- process request environment validation;
- test-only dogfood handler behavior.

## 9. Commands Run And Results

Validation commands for this fix:

- `cargo test -p workflow-core --test local_check`
  - Passed.
- `cargo fmt --all --check`
  - Passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Passed.
- `cargo test --workspace`
  - Passed.
- `npm run check:docs`
  - Passed.

## 10. Remaining Known Limitations

- The executable local check path remains test-only.
- No production local check handler exists.
- No production sandbox exists.
- Timeout behavior still uses local process polling in the standard runner.
- Environment sanitization remains minimal.
- Output redaction remains secret-like rejection, not a comprehensive DLP engine.
- `LocalCheckResult` is not yet attached to evidence or work reports.

## 11. Recommended Next Phase

Recommended next phase: **local check handler infrastructure blocker fix review**.

After the fix review passes, the project can proceed to first non-dogfood local check handler planning, with `DocsCheck` remaining the likely first candidate.
