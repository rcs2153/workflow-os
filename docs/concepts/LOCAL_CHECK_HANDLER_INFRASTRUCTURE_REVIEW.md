# Local Check Handler Infrastructure Review

Review date: 2026-06-15

## 1. Executive Verdict

Needs blocker fixes.

The local check handler infrastructure is otherwise well scoped and aligned with the approved infrastructure-only phase, but one redaction-boundary blocker must be fixed before the project proceeds to non-dogfood local check handlers:

- `LocalCheckProcessOutput` is exported and derives `Debug` while it stores raw pre-redaction `stdout` and `stderr` bytes.

That means secret-like process output can leak through debug formatting before `LocalCheckResult` has a chance to bound, validate, and redact it. The fix should be small: either keep `LocalCheckProcessOutput` internal, or implement a custom redaction-safe `Debug` for it and add focused regression tests.

## 2. Scope Verification

The phase stayed within the approved infrastructure-only scope.

Confirmed in scope:

- validated `LocalCheckResult` model;
- process request/output model;
- injectable `LocalCheckProcessRunner` boundary;
- deterministic runner tests;
- refactoring the test-only dogfood handler to use shared result construction;
- bounded output capture before creating public local check results;
- documentation and phase report.

No accidental implementation was found for:

- production local check handlers;
- broader command handlers such as docs, cargo, TypeScript, contracts, or integrations;
- default handler registration;
- CLI behavior;
- workflow schema fields;
- automatic local check execution;
- automatic report generation;
- report artifacts;
- persistence;
- evidence attachment;
- side-effect boundary modeling;
- source writes;
- live provider calls;
- recursive agents or agent swarms;
- hosted/distributed runtime claims;
- release posture changes.

## 3. Dogfood Governance Check

This review was itself run under the self-governance dogfood workflow before the review artifact was written.

- State directory: `/tmp/workflow-os-local-check-infra-review.3x2CFJ`
- Run ID: `run-1781500755340014000-2`
- Workflow ID: `dg/d`
- Approval ID: `approval/run-1781500755340014000-2/d`
- Final status: `Completed`

Inspection confirmed the expected event history through `RunCompleted`.

## 4. Model Assessment

`LocalCheckResult` is the right model boundary for reviewed local check outcomes.

Positive findings:

- result construction validates bounded stdout and stderr summaries;
- secret-like summaries fail closed with stable code `local_check.output.secret_like`;
- result status, exit code, duration, truncation flags, and error code are explicit;
- `LocalCheckResult` has redaction-safe `Debug`;
- serde deserialization reuses validated construction;
- invalid serialized summaries fail closed.

The model does not store raw command transcripts, raw environment values, or raw full output.

## 5. Process Runner Assessment

The process-runner abstraction is a good fit for this phase.

Positive findings:

- `LocalCheckProcessRunner` allows deterministic tests for success, failure, timeout, and runner errors;
- `LocalCheckProcessRequest` validates arguments, environment values, and timeout;
- `LocalCheckProcessRequest` has redaction-safe `Debug`;
- the standard runner uses executable plus argument vector and does not invoke a shell;
- the dogfood handler constructs a sanitized environment instead of passing ambient process environment through.

Blocker:

- `LocalCheckProcessOutput` derives `Debug` while storing raw `stdout` and `stderr` bytes. This leaks the raw pre-redaction process boundary if the output is debug-formatted by tests, diagnostics, or future handler code.

Recommended blocker fix:

- replace derived `Debug` on `LocalCheckProcessOutput` with a custom implementation that redacts stdout/stderr and only reports safe metadata such as byte counts, exit code, success flag, timeout flag, and duration; or make the type private and unexported if public construction is not required.

The smaller likely fix is custom `Debug`, because integration tests currently construct injected process outputs.

## 6. Handler Integration Assessment

The test-only dogfood handler remains properly constrained.

Positive findings:

- it is not registered by default;
- it accepts only the canonical dogfood validation command kind;
- it requires repository-root working directory policy;
- it requires disabled network policy;
- it requires no-source-writes classification;
- it validates required local paths;
- it uses the injected process runner and converts process output through `LocalCheckResult`;
- skill output is derived from the validated result model.

No broader command execution path was introduced.

## 7. Output And Redaction Assessment

The reviewed result boundary is mostly redaction-safe.

Positive findings:

- bounded stdout and stderr summaries are created before `LocalCheckResult` storage;
- secret-like stdout and stderr are rejected;
- error messages use stable codes and do not echo raw output;
- `LocalCheckResult` debug formatting redacts output summaries;
- serialized `LocalCheckResult` cannot carry secret-like summaries silently because deserialization validates.

Blocker:

- raw stdout/stderr can leak via `LocalCheckProcessOutput` debug formatting before the result boundary.

Non-blocking note:

- future production handlers will need a broader output policy for very large, binary, structured, or tool-specific output. That remains outside this infrastructure slice.

## 8. Runtime And Event Boundary

The phase preserved the runtime boundary.

Confirmed:

- no new runtime event kinds;
- no post-terminal event appending;
- no report artifact writes;
- no `StateBackend` writes for local check results;
- no CLI rendering;
- no automatic execution;
- existing executor event behavior is unchanged.

The test-only handler still runs only where explicitly registered by tests.

## 9. Test Quality Assessment

The tests are strong for the implemented infrastructure.

Covered:

- valid local check result construction;
- failed result construction;
- secret-like result summary rejection;
- unbounded summary rejection;
- `LocalCheckResult` debug and serialization non-leakage;
- valid and invalid serde behavior;
- injected runner success, non-zero exit, timeout, runner failure, and secret-like output;
- sanitized environment construction;
- process request environment rejection;
- existing dogfood executor path.

Missing blocker regression:

- `LocalCheckProcessOutput` debug formatting should be tested to ensure stdout/stderr bytes do not leak.

Non-blocking follow-up tests after the blocker fix:

- standard runner timeout behavior may deserve a focused integration-style test if it can be kept deterministic;
- public API exposure for process-runner types should be reviewed before production handler expansion.

## 10. Documentation Review

The planning and report docs accurately say:

- local check result/process-runner infrastructure is implemented;
- the executable handler path remains dogfood-only and test-only;
- no broader command handlers are implemented;
- no default registration, CLI behavior, schemas, persistence, report artifacts, evidence attachment, side-effect modeling, writes, or release posture changes were introduced.

One documentation follow-up is needed after the blocker fix:

- update `LOCAL_CHECK_HANDLER_INFRASTRUCTURE_REPORT.md` or add a fix report noting that raw process output debug formatting is redaction-safe.

## 11. Blockers

One blocker:

- `LocalCheckProcessOutput` derives `Debug` and stores raw pre-redaction stdout/stderr bytes. This can leak secret-like output through debug formatting before the validated `LocalCheckResult` boundary.

Required fix:

- make `LocalCheckProcessOutput` debug output redaction-safe or keep the type private/unexported;
- add regression tests proving secret-like stdout/stderr do not appear in `format!("{output:?}")`;
- rerun full validation.

## 12. Non-Blocking Follow-Ups

- Reassess whether `LocalCheckProcessRunner`, `LocalCheckProcessRequest`, and `LocalCheckProcessOutput` should remain public exports before production handler APIs are stabilized.
- Consider tightening executable path validation if process request construction is used outside the dogfood handler.
- Define a production sandbox and cache/write policy before enabling cargo, npm, or integration command handlers.
- Decide how `LocalCheckResult` should later cite evidence or work reports without storing raw command output.
- Keep `DocsCheck` as the likely first non-dogfood handler candidate only after the blocker fix and review.

## 13. Recommended Next Phase

Recommended next phase: **local check handler infrastructure blocker fix**.

The blocker fix should be narrow: make `LocalCheckProcessOutput` redaction-safe at debug/output boundaries and add focused tests. After that review passes, the project can move to a first non-dogfood local check handler plan, likely `DocsCheck`.

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

These checks confirm the current tree builds and tests cleanly. They do not remove the blocker because the blocker is a review finding about an untested debug/redaction boundary.
