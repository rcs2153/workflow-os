# Self-Governance Dogfood Hardening Test Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The self-governance dogfood hardening test phase closes the main non-blocking gaps from the dogfood multi-step conversion review. It adds focused core executor tests for cancellation at the planning approval checkpoint, duplicate run-id rehydration after completion, and explicit in-memory report-bearing execution against the real dogfood project.

No blocker was found.

## 2. Scope Verification

The phase stayed within the approved test-only scope.

Completed scope:

- added dogfood cancellation coverage while waiting at `planning-approved`;
- added duplicate run-id rehydration coverage for a completed dogfood run;
- added report-bearing dogfood execution coverage through existing `execute_with_report(...)` APIs;
- updated roadmap and dogfood test documentation;
- created an implementation report.

No accidental scope expansion was found for:

- runtime implementation changes;
- dogfood workflow spec changes;
- skill or policy changes;
- real local validation/check command execution;
- default local check handler registration;
- arbitrary shell execution;
- automatic runtime report generation;
- automatic report artifact writing;
- report CLI rendering;
- workflow schema changes;
- examples outside documentation wording;
- branching, parallel, or DAG execution;
- nested harness runtime behavior;
- Composable Harness Contract runtime behavior;
- typed handoff runtime behavior;
- command-output evidence attachment;
- approval evidence attachment;
- reasoning lineage or claim graph implementation;
- side-effect boundary implementation;
- write behavior;
- hosted or distributed runtime behavior;
- Level 3 or Level 4 autonomy;
- release posture changes.

## 3. Cancellation Coverage Assessment

`dogfood_cancellation_while_waiting_on_planning_approval_stops_downstream_steps` is appropriate and meaningful.

The test uses the real dogfood project through the core local executor and an explicit deterministic placeholder handler for `local/d`. It verifies:

- the run reaches `WaitingForApproval`;
- the approval ID is scoped to `planning-approved`;
- exactly one placeholder invocation occurs before cancellation;
- cancellation transitions the run to `Canceled`;
- scheduled steps stop at `scope-requested` and `planning-approved`;
- only `scope-requested` is requested for skill invocation;
- downstream placeholder checkpoints are not invoked.

This closes the dogfood-specific cancellation gap without adding CLI cancellation behavior.

## 4. Duplicate Run-ID / Replay Assessment

`dogfood_duplicate_run_id_rehydrates_completed_run_without_reinvoking_steps` covers the strongest currently practical dogfood replay path.

The test uses an explicit run ID, completes the dogfood run through approval, then calls `execute(...)` again with the same run ID. It verifies:

- the duplicate call rehydrates the completed run;
- the event history is unchanged;
- the placeholder handler call count does not increase;
- all five dogfood steps succeeded exactly once.

This is aligned with the existing executor idempotency behavior and avoids adding CLI flags purely for testing.

## 5. Report-Bearing Execution Assessment

`dogfood_report_bearing_execution_uses_existing_explicit_api_without_artifacts` is within scope.

The test completes the real dogfood workflow first, then calls `execute_with_report(...)` with the same explicit run ID and bounded report inputs. It verifies:

- report generation uses the existing explicit executor-integrated report path;
- the completed run is preserved;
- an in-memory `WorkReport` is returned;
- all required v1 report sections are present;
- missing validation/check references remain explicit not-available section text;
- the side-effects section remains unsupported;
- no report artifact is written.

This proves the dogfood workflow can participate in report-bearing execution without making reports automatic or persistent.

## 6. Governance Boundary Assessment

The tests preserve the current dogfood boundary.

Workflow OS governs:

- project loading and validation through existing executor paths;
- run identity;
- event history;
- sequential step scheduling;
- policy decisions;
- approval pause/resume;
- cancellation;
- run rehydration;
- explicit in-memory report construction.

Codex or a human still performs repository edits, real validation/check command execution, review judgment, and final implementation reporting outside the kernel.

The tests do not claim or implement production self-hosting, automatic Codex control, real build-command execution, recursive agents, agent swarms, or nested harness runtime behavior.

## 7. Test Quality Assessment

The tests are behavior-oriented and inspect state/event outcomes rather than only object construction.

Strengths:

- use the real dogfood project path;
- use isolated temporary state;
- assert run status, approval identity, step scheduling, skill invocation boundaries, and call counts;
- verify explicit report-bearing behavior through existing APIs;
- verify no report artifacts are created;
- avoid new CLI behavior;
- avoid real command execution.

Non-blocking gaps:

- the report-bearing dogfood test does not separately assert Debug/serialization non-leakage for dogfood-specific report inputs; this is already covered by the broader WorkReport and executor report tests.
- there is still no executable dogfood `.test.yml` spec, which remains deferred until test execution semantics are scoped.

These are not blockers.

## 8. Documentation Review

Documentation is accurate for the phase:

- `docs/implementation-plans/self-governance-dogfood-hardening-test-plan.md` is marked implemented.
- `docs/concepts/SELF_GOVERNANCE_DOGFOOD_HARDENING_TEST_REPORT.md` documents completed scope, non-scope, coverage, validation, limitations, and next phase.
- `dogfood/workflow-os-self-governance/tests/README.md` correctly states that no executable dogfood `.test.yml` specs exist yet and that coverage currently lives in repository-level CLI/core tests.
- `ROADMAP.md` states that the test-only phase is implemented and that real command execution, default handler registration, command-output evidence, side effects, writes, and nested harness runtime behavior remain deferred.

No documentation overclaims current capabilities.

## 9. Privacy And Redaction Assessment

The phase uses bounded non-secret values for actors, approval reasons, report limitations, risks, incomplete-work disclosures, and handoff notes.

No raw provider payloads, raw command output, raw CI logs, raw GitHub/Jira bodies, raw spec contents, parser payloads, environment variable values, credentials, authorization headers, private keys, token-like values, or unbounded natural-language payloads were introduced.

The tests do not add command-output evidence or report artifact persistence.

## 10. Blockers

No blockers.

## 11. Non-Blocking Follow-Ups

- Keep dogfood `.test.yml` specs deferred until test execution semantics are separately scoped.
- Consider dogfood report Debug/serialization assertions only if future dogfood-specific report inputs grow beyond the current bounded values.
- Consider local check handler default-registration planning as the next practical governed-kernel lane.
- Keep real command execution, command-output evidence, side-effect boundary implementation, writes, and nested harness runtime behavior deferred.

## 12. Recommended Next Phase

Recommended next phase: local check handler default-registration planning.

The dogfood multi-step path is now converted, cleaned up, and hardened around the most important lifecycle edges. The next useful governed-kernel step is to plan whether and how explicitly implemented local check handlers, beginning with `DocsCheck`, can become safely registered by default without opening arbitrary command execution, shell access, CLI report rendering, command-output evidence, side effects, or writes.

## 13. Validation

- `cargo fmt --all --check` - pass.
- `cargo clippy --workspace --all-targets -- -D warnings` - pass.
- `cargo test --workspace` - pass.
- `npm run check:docs` - pass.
