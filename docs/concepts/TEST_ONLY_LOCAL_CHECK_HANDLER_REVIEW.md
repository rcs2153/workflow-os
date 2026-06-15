# Test-Only Local Check Handler Review

Review date: 2026-06-15

## 1. Executive Verdict

Blocker fixed; proceed to broader local check handler planning.

The test-only `WorkflowOsValidateDogfood` local check handler is explicit, narrow, non-shell, non-default, bounded, and redaction-conscious. It proves the first local check handler boundary without authorizing production command execution, CLI exposure, workflow schema changes, automatic check execution, report artifacts, side-effect boundary implementation, writes, recursive agents, agent swarms, production self-hosting, or release posture changes.

## 2. Scope Verification

The phase stayed within the approved test-only handler scope.

Confirmed in scope:

- `TestOnlyWorkflowOsValidateDogfoodHandler` exists.
- The handler supports only `LocalCheckCommandKind::WorkflowOsValidateDogfood`.
- The handler requires an explicit `LocalCheckCommandContract`.
- The handler is registered only in focused tests.
- The handler returns normal `SkillOutput` through the existing `SkillHandler` interface.
- The handler is exercised through existing local executor skill invocation behavior.

No accidental scope expansion found:

- No production local check handler registration.
- No broad handler discovery.
- No arbitrary shell execution.
- No user-supplied command text.
- No CLI exposure.
- No workflow schema changes.
- No automatic check execution.
- No automatic runtime report generation.
- No report artifact writing from the handler path.
- No evidence attachment.
- No side-effect boundary implementation.
- No source writes.
- No write-capable adapters.
- No provider calls or live adapter execution.
- No recursive agent, agent swarm, hosted runtime, or production self-hosting claim.
- No release posture change.

## 3. Dogfood Governance Check

This review was also governed by the Workflow OS dogfood workflow before review writing.

- State directory: `/tmp/workflow-os-self-governance-review.fO3eUB`
- Run ID: `run-1781499234647925000-2`
- Approval ID: `approval/run-1781499234647925000-2/d`
- Final status: `Completed`

Inspection confirmed ordered events from `RunCreated` through `RunCompleted`.

## 4. Handler Design Assessment

The handler design fits the approved boundary.

- Construction validates the contract before storing it.
- Construction rejects unsupported command kinds with stable code `local_check.handler.unsupported_kind`.
- Construction requires repository-root working directory policy, disabled network policy, and no-source-writes side-effect classification.
- Construction requires an existing workflow-os binary path and a repository root that contains `Cargo.toml` plus the dogfood project.
- `Debug` output redacts local binary and repository paths.
- The handler keeps the serialized contract posture as `ModelOnly`; execution authority comes from the explicit test-only handler type and explicit test registration, not from broadening the contract model.

This is the right shape for a first handler slice. It proves a reviewed boundary without making local checks ambient runtime behavior.

## 5. Command Authority Assessment

The command authority boundary is acceptable for test-only scope.

- The handler uses `std::process::Command` with an executable path plus a fixed argument vector.
- The handler does not invoke a shell.
- The handler does not concatenate command strings.
- The handler does not accept caller-supplied extra arguments.
- The contract model still validates canonical command templates.
- The handler executes from an explicit repository root.
- The handler clears the environment and restores only a minimal `PATH`.
- Timeout is bounded by the contract timeout.

No arbitrary local command execution path was introduced.

## 6. Output And Redaction Assessment

Output handling is acceptable for the test-only phase.

- Stdout and stderr are bounded by the existing `LocalCheckOutputCapturePolicy`.
- Raw output is not persisted.
- Full command transcripts are not stored.
- Secret-like bounded output fails closed with stable code `local_check.output.secret_like`.
- Error messages avoid echoing raw command output, local paths, command arguments, or secret-like text.
- The skill output stores bounded summaries, truncation flags, local check status, exit code, duration, and kind.

The approach is intentionally simple. It is not a full redaction engine, and the report documents that limitation honestly.

## 7. Runtime And Event Boundary Assessment

The handler preserves the local executor boundary.

- The handler is used through explicit `LocalSkillRegistry` test registration.
- The executor emits only existing local skill invocation events.
- The focused executor test verifies returned events match persisted backend events.
- The focused executor test verifies no work report artifacts are written.
- No new runtime event type is added.
- No post-terminal event append behavior is added.
- No automatic report generation or artifact writing is added.

This satisfies the event-boundary requirement for the phase.

## 8. Privacy And Security Assessment

The implementation avoids the primary privacy and security failures for this phase.

- No shell invocation.
- No provider credentials.
- No ambient environment propagation.
- No raw output persistence.
- No command transcript persistence.
- No provider payload copying.
- No parser payload copying.
- No spec content copying.
- No token or credential values in structured errors.
- No local path leakage through handler `Debug`.

The remaining security limitations are documented: there is no production sandbox, timeout is local process polling, and output redaction is secret-like rejection rather than a comprehensive redaction engine.

## 9. Test Quality Assessment

Tests cover the core test-only boundary:

- local check command contract validation and canonical template binding;
- model-only execution posture rejection;
- shell metacharacter and whitespace rejection;
- secret-like argument and environment name rejection;
- raw output persistence rejection;
- invalid serde failure without leaking secret-like payloads;
- handler rejection of unsupported command kinds without leaking local paths;
- handler `Debug` redaction of local paths;
- explicit executor registration and execution of the dogfood validation handler;
- event-log preservation;
- no work report artifact writes.

Missing or shallow tests are non-blocking for this phase:

- No controlled failed-dogfood fixture proves non-zero exit maps to `LocalCheckResultStatus::Failed`.
- No deterministic timeout test exists; adding one would likely benefit from an injectable process runner.
- No direct test asserts the child process receives a sanitized environment.
- No direct test asserts secret-like stdout/stderr failure using a controlled child process.
- No direct test snapshots repository source files before and after handler execution; current coverage instead verifies no report artifacts and relies on the selected dogfood validation command being no-source-writes.

These gaps should be addressed before adding broader command kinds.

## 10. Documentation Review

Documentation is honest about current capability.

Confirmed:

- The implementation report states the handler is test-only.
- The plan states production local command execution is not implemented.
- The plan and roadmap state no CLI exposure, no workflow schema changes, no automatic check execution, no report artifacts, no evidence attachment, no side-effect boundary implementation, no writes, and no production self-hosting.
- The self-governed validation/check plan positions this as the first test-only handler boundary and keeps broader handler execution future-scoped.

One minor wording issue remains in `self-governed-validation-check-plan.md`: the status line still says real local validation/check skill handlers are not implemented. That remains defensible if "real" means production handlers, but a future cleanup should distinguish "test-only handler implemented" from "production handlers not implemented" more directly.

## 11. Blockers

No blockers.

## 12. Non-Blocking Follow-Ups

- Add an injectable process-runner boundary before expanding beyond `WorkflowOsValidateDogfood`.
- Add deterministic tests for failed validation, timeout behavior, secret-like stdout/stderr rejection, and environment sanitization.
- Tighten documentation wording around "real handlers" versus "test-only handler."
- Decide whether a local check result type should replace skill-output map fields before report/evidence integration.
- Keep additional command kinds deferred until side-effect policy, output policy, and sandbox posture are reviewed.

## 13. Recommended Next Phase

Recommended next phase: broader local check handler planning.

The next plan should decide whether to add an injectable process runner and a local check result model before introducing additional command kinds such as docs check, cargo fmt, clippy, or cargo test. It must keep production registration, CLI exposure, workflow schema changes, automatic execution, report artifacts, evidence attachment, side-effect modeling, writes, recursive agents, agent swarms, hosted execution, and release posture changes out of scope unless separately approved.

## 14. Validation

Validation run for this review:

- `cargo fmt --all --check`
  - Passed with the repository toolchain path.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Passed.
- `cargo test --workspace`
  - Passed.
- `npm run check:docs`
  - Passed.
