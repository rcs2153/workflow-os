# Local Check Handler Infrastructure Blocker Fix Review

Review date: 2026-06-15

## 1. Executive Verdict

Blocker fixed; proceed to first non-dogfood local check handler planning.

The fix directly addresses the blocker from the infrastructure review. `LocalCheckProcessOutput` no longer derives `Debug`, and its custom debug implementation redacts raw stdout/stderr bytes while exposing only safe metadata.

No additional blocker was found.

## 2. Scope Verification

The fix stayed within the approved blocker-fix scope.

Confirmed in scope:

- removed raw-output debug formatting from `LocalCheckProcessOutput`;
- added a custom redaction-safe `Debug` implementation;
- added focused regression tests for completed and timed-out process output;
- added a blocker-fix report;
- preserved existing local check behavior and test-only dogfood execution.

No accidental implementation was found for:

- production local check handlers;
- broader command handlers;
- default handler registration;
- CLI behavior;
- workflow schema fields;
- automatic local check execution;
- report artifacts;
- persistence;
- evidence attachment;
- side-effect boundary modeling;
- source writes;
- live provider calls;
- release posture changes.

## 3. Governance Check

This review was governed by the self-governance dogfood workflow.

- State directory: `/tmp/workflow-os-local-check-output-debug-fix-review`
- Run ID: `run-1781501271242105000-2`
- Workflow ID: `dg/d`
- Approval ID: `approval/run-1781501271242105000-2/d`
- Final status: `Completed`

Inspection confirmed the expected event history through `RunCompleted`.

## 4. Original Blocker Restatement

The original blocker was:

- `LocalCheckProcessOutput` was public/exported;
- it stored raw pre-redaction stdout and stderr bytes;
- it derived `Debug`;
- debug formatting could expose secret-like command output before `LocalCheckResult` performed bounded summary validation and secret-like rejection.

This violated the project’s redaction-boundary pattern.

## 5. Fix Approach Assessment

The selected fix is minimal and idiomatic.

Implemented approach:

- keep `LocalCheckProcessOutput` available for deterministic injected-runner tests;
- remove derived `Debug`;
- add custom `Debug` that reports:
  - exit code;
  - success flag;
  - duration;
  - stdout byte count;
  - stderr byte count;
  - timeout flag;
  - redacted stdout/stderr markers.

This preserves the existing process-runner test boundary while removing the raw-output debug leak.

## 6. Debug And Redaction Assessment

Verified:

- raw stdout bytes are not debug-formatted;
- raw stderr bytes are not debug-formatted;
- completed process output debug formatting redacts secret-like stdout/stderr;
- timed-out process output debug formatting redacts secret-like stdout/stderr;
- safe process metadata remains available for debugging;
- `LocalCheckResult` remains the validated storage boundary for bounded output summaries.

No serialization exposure was introduced because `LocalCheckProcessOutput` does not implement serde.

## 7. Validation Boundary Assessment

The fix restores the intended two-stage boundary:

1. `LocalCheckProcessOutput` can carry raw process bytes from a runner to the local result builder, but debug formatting cannot expose those bytes.
2. `LocalCheckResult` bounds, validates, and rejects secret-like summaries before reviewed result storage or skill-output conversion.

Validation errors remain stable and non-leaking. No raw output, command transcript, environment value, provider payload, parser payload, token, credential, authorization header, or private-key marker is echoed by the fix.

## 8. Regression Assessment

Existing behavior remains intact:

- valid local check results still construct;
- failed local check results remain representable;
- injected runner success maps to `passed`;
- non-zero exit maps to `failed`;
- timeout maps to `timed_out`;
- secret-like stdout/stderr still fail closed at `LocalCheckResult` construction;
- runner failures still return stable non-leaking errors;
- test-only dogfood handler behavior is unchanged;
- no new command handlers were added.

## 9. Test Quality Assessment

New focused tests:

- `local_check_process_output_debug_does_not_leak_raw_output`;
- `local_check_process_timeout_output_debug_does_not_leak_raw_output`.

These tests cover:

- completed process output with secret-like stdout and stderr;
- timed-out process output with secret-like stdout and stderr;
- presence of safe byte-count/debug metadata;
- absence of token-like and private-key-like strings in debug output.

The broader local check test suite now has 35 tests and continues to cover the process request, result model, injected runner, test-only dogfood handler, and serde boundaries.

## 10. Documentation Review

Documentation was updated honestly.

Confirmed:

- `LOCAL_CHECK_HANDLER_INFRASTRUCTURE_BLOCKER_FIX_REPORT.md` records the blocker and fix;
- `LOCAL_CHECK_HANDLER_INFRASTRUCTURE_REPORT.md` preserves the original phase record and adds a fix-forward note;
- docs continue to state that production local check handlers, CLI behavior, schema fields, persistence, report artifacts, evidence attachment, side-effect modeling, writes, and release posture changes are not implemented.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Reassess public export posture for `LocalCheckProcessRunner`, `LocalCheckProcessRequest`, and `LocalCheckProcessOutput` before stabilizing production handler APIs.
- Consider tightening executable path validation before broader handler exposure.
- Define production sandbox/cache/write policy before cargo, npm, or integration command handlers execute beyond the dogfood test path.
- Plan how `LocalCheckResult` should later cite evidence or work reports without storing raw command output.

## 13. Recommended Next Phase

Recommended next phase: **first non-dogfood local check handler planning**.

`DocsCheck` remains the strongest first candidate because it is project-owned and narrow, but it still needs explicit planning around npm/tooling paths, environment policy, cache/write posture, timeout, output capture, registration posture, and docs check result reporting.

## 14. Validation

Validation commands run for this review:

- `cargo fmt --all --check`
  - Passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Passed.
- `cargo test --workspace`
  - Passed.
- `npm run check:docs`
  - Passed.
