# Self-Governance Dogfood Hardening Test Report

## 1. Executive Summary

The self-governance dogfood hardening test phase is complete.

The phase added focused core executor tests against the real dogfood project for cancellation while waiting on the planning approval checkpoint, duplicate run-id rehydration after completion, and report-bearing dogfood execution through existing explicit APIs.

No runtime implementation, dogfood spec, skill, policy, CLI, schema, artifact, command-execution, side-effect, write, or release posture changes were introduced.

## 2. Scope Completed

- Added dogfood cancellation coverage while the run waits at `planning-approved`.
- Added dogfood duplicate run-id rehydration coverage after a completed run.
- Added dogfood report-bearing execution coverage through `execute_with_report(...)` using explicit report inputs.
- Updated dogfood test documentation and roadmap status.
- Marked the hardening test plan as implemented.

## 3. Scope Explicitly Not Completed

- No runtime implementation changes.
- No dogfood workflow spec changes.
- No skill or policy changes.
- No real local validation/check command execution.
- No default local check handler registration.
- No arbitrary shell execution.
- No automatic runtime report generation.
- No automatic report artifact writing.
- No report CLI rendering.
- No workflow schema changes.
- No examples outside documentation wording.
- No branching, parallel, or DAG execution.
- No nested harness runtime behavior.
- No Composable Harness Contract runtime behavior.
- No typed handoff runtime behavior.
- No command-output evidence attachment.
- No approval evidence attachment.
- No reasoning lineage or claim graph implementation.
- No side-effect boundary implementation.
- No write behavior.
- No hosted or distributed runtime behavior.
- No Level 3 or Level 4 autonomy.
- No release posture changes.

## 4. Cancellation Test Summary

`dogfood_cancellation_while_waiting_on_planning_approval_stops_downstream_steps` runs the real dogfood project through the core local executor with deterministic placeholder local skill registration.

The test verifies:

- the run waits for approval at `planning-approved`;
- only `scope-requested` is invoked before cancellation;
- cancellation transitions the run to `Canceled`;
- downstream steps are not invoked after cancellation;
- no post-cancellation downstream invocation events are appended.

## 5. Duplicate Run-ID / Replay Test Summary

`dogfood_duplicate_run_id_rehydrates_completed_run_without_reinvoking_steps` executes the real dogfood workflow with an explicit run ID, grants the planning approval, and then calls `execute(...)` again with the same run ID.

The test verifies:

- the completed run rehydrates deterministically;
- event history is unchanged;
- the placeholder handler is not invoked again;
- all five dogfood steps succeeded exactly once.

## 6. Report-Bearing Dogfood Execution Test Summary

`dogfood_report_bearing_execution_uses_existing_explicit_api_without_artifacts` first completes the real dogfood workflow, then calls `execute_with_report(...)` with the same run ID and explicit report inputs.

The test verifies:

- the completed run rehydrates through the existing explicit report-bearing API;
- an in-memory report is returned;
- all required v1 report sections are present;
- absent validation/check references remain explicit not-available section text;
- the side-effects section remains explicit unsupported text;
- no report artifacts are written.

## 7. Dogfood Governance Boundary Summary

The tests preserve the current dogfood boundary:

- Workflow OS governs validation, run identity, event history, sequential step scheduling, policy decisions, approval pause/resume, cancellation, rehydration, and explicit in-memory report construction.
- Codex or a human still performs repository edits, real validation/check command execution, review judgment, and final implementation reporting outside the kernel.
- The tests use deterministic placeholder local skill behavior and do not execute real local commands.

## 8. Privacy And Redaction Summary

The tests use bounded non-secret actor IDs, approval reasons, report notes, limitations, risks, and handoff notes.

No raw provider payloads, raw command output, raw CI logs, raw GitHub/Jira bodies, raw spec contents, parser payloads, environment variable values, credentials, authorization headers, private keys, token-like values, or unbounded natural-language payloads were introduced.

## 9. Commands Run And Results

- `cargo test -p workflow-core --test local_executor dogfood_` - pass: 4 tests.
- `cargo fmt --all --check` - pass.
- `cargo clippy --workspace --all-targets -- -D warnings` - pass.
- `cargo test --workspace` - pass.
- `npm run check:docs` - pass.

## 10. Remaining Known Limitations

- The dogfood workflow still uses deterministic placeholder local skill behavior.
- Real local validation/check command execution remains separately scoped.
- Default local check handler registration remains unimplemented.
- Command-output evidence attachment remains deferred.
- Typed handoff runtime behavior remains unimplemented for the dogfood workflow.
- Side-effect boundary and writes remain unsupported.
- No automatic runtime report generation or automatic report artifact writing is implemented.

## 11. Recommended Next Phase

Recommended next phase: self-governance dogfood hardening test review.

The review should verify that the new dogfood tests cover the intended cancellation, rehydration, and explicit report-bearing paths while preserving the kernel-governed, Codex/human-executed boundary and avoiding runtime broadening.
