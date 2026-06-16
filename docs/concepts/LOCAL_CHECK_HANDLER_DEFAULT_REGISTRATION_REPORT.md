# Local Check Handler Default-Registration Report

Report date: 2026-06-16

## 1. Executive Summary

Workflow OS now has an explicit local check registration profile/helper while true ambient default registration remains deferred.

The implemented boundary adds `LocalCheckRegistrationProfile`, `LocalCheckRegistrationMode`, safe planned-handler metadata, and `LocalSkillRegistry::register_local_check_profile(...)`. It supports only two modes: `None` and explicit caller-supplied `DocsCheck`. `LocalSkillRegistry::new()` remains empty/default-safe.

This phase does not add CLI exposure, workflow schema fields, automatic local check execution, true default handler registration, command-output evidence, local check evidence attachment, side-effect boundary modeling, writes, or release posture changes.

## 2. Governance Run

This implementation phase was governed by the self-governance dogfood workflow before implementation.

- State directory: `/private/tmp/workflow-os-default-registration-plan-state-20260616`
- Run ID: `run-1781593992165176000-2`
- Workflow ID: `dg/d`
- Approval ID: `approval/run-1781593992165176000-2/planning-approved`
- Approval reason: `local-check-default-registration-planning`
- Final status: `Completed`

Inspection confirmed event history through `RunCompleted` with 34 durable events.

## 3. Scope Completed

- Added `LocalCheckRegistrationMode`.
- Added `LocalCheckRegisteredHandler`.
- Added `LocalCheckRegistrationProfile`.
- Added `LocalSkillRegistry::register_local_check_profile(...)`.
- Added safe planned-handler inspection for the explicit docs-check profile.
- Preserved `LocalSkillRegistry::new()` as default-safe.
- Added tests for `None` profile behavior.
- Added tests for explicit `DocsCheck` profile metadata.
- Added executor tests proving the explicit profile can register a supplied `DocsCheckLocalHandler` and execute through the existing injected-runner path.
- Updated roadmap and planning documentation.

## 4. Scope Explicitly Not Completed

- No true ambient default registration.
- No automatic local check execution.
- No CLI exposure.
- No CLI commands.
- No workflow schema fields.
- No workflow-declared local check registration.
- No `AllowlistedHandlerOnly` enablement.
- No broad handler discovery.
- No arbitrary shell execution.
- No cargo, TypeScript, contract, integration, or provider check handler registration.
- No command-output evidence.
- No local check evidence attachment.
- No report artifact writing.
- No persistence changes.
- No side-effect boundary implementation.
- No source writes.
- No write-capable adapters.
- No hosted or distributed runtime behavior.
- No recursive agents or agent swarms.
- No release posture change.

## 5. Registration Model/Helper Summary

New model/helper surface:

- `LocalCheckRegistrationMode::None`
- `LocalCheckRegistrationMode::ExplicitDocsCheck`
- `LocalCheckRegisteredHandler`
- `LocalCheckRegistrationProfile::none()`
- `LocalCheckRegistrationProfile::explicit_docs_check(...)`
- `LocalCheckRegistrationProfile::planned_handlers()`
- `LocalSkillRegistry::register_local_check_profile(...)`

`ExplicitDocsCheck` requires a prebuilt `DocsCheckLocalHandler`. The profile does not construct handlers, discover executables, resolve `PATH`, infer repository roots, create cache directories, read environment variables, or run commands.

## 6. Default-Safe Behavior Summary

`LocalSkillRegistry::new()` remains empty/default-safe.

`LocalCheckRegistrationProfile::none()` registers no handlers and preserves existing missing-handler behavior. A docs-check workflow still fails closed with `executor.skill_handler.missing` unless a caller explicitly registers a handler.

## 7. Authorization Boundary Summary

Execution authority remains explicit Rust construction/registration.

Serialized workflow specs cannot activate local command execution. `AllowlistedHandlerOnly` remains unsupported. Registered handlers still execute only canonical command templates through the existing skill-handler path.

The profile exposes only safe metadata:

- command kind;
- canonical skill ID;
- canonical skill version.

It does not expose executable paths, cache paths, environment values, command output, or user-controlled command text.

## 8. Runtime/Event Boundary Summary

The phase uses existing runtime behavior only.

- No runtime event kinds were added.
- No automatic check execution was added.
- No post-terminal events are appended by the helper.
- No report artifacts are written.
- No `StateBackend` writes occur outside existing executor behavior.
- No observability or audit noise was introduced.
- No CLI output was added.

## 9. Privacy/Redaction Summary

`LocalCheckRegistrationProfile` has a custom redaction-safe `Debug` implementation. Debug output includes the mode and planned-handler count, while redacting the supplied handler.

The profile does not store or copy:

- raw command output;
- docs contents;
- parser payloads;
- provider payloads;
- environment values;
- npm tokens;
- registry credentials;
- authorization headers;
- private keys;
- token-like strings;
- unbounded paths or command arguments.

Errors use stable codes and do not include raw paths, command output, environment values, or secret-like payloads.

## 10. Test Coverage Summary

Focused tests cover:

- `None` registration profile registers no handlers.
- `ExplicitDocsCheck` exposes only safe metadata for `local/check-docs` `v0`.
- `LocalSkillRegistry::new()` remains default-safe when given the `None` profile.
- Explicit profile registration can execute a docs-check workflow through `LocalExecutor` with an injected runner.
- Process requests still use canonical `npm run check:docs` arguments.
- No report artifacts are written by the explicit profile execution path.
- Debug output does not leak local paths, cache names, or command arguments.

Existing local check, executor, report, evidence, validation, adapter telemetry, dogfood, and runtime tests remain part of workspace validation.

## 11. Commands Run And Results

Implementation-time focused checks:

- `workflow-os --project-dir dogfood/workflow-os-self-governance validate`
  - Passed with expected experimental lifecycle warnings.
- `workflow-os --project-dir dogfood/workflow-os-self-governance --state-dir /private/tmp/workflow-os-default-registration-plan-state-20260616 --mock-all-local-skills run dg/d`
  - Paused at `WaitingForApproval`.
- `workflow-os --project-dir dogfood/workflow-os-self-governance --state-dir /private/tmp/workflow-os-default-registration-plan-state-20260616 --mock-all-local-skills approve approval/run-1781593992165176000-2/planning-approved --reason local-check-default-registration-planning`
  - Completed.
- `cargo test -p workflow-core --test local_check local_check_registration`
  - Passed.
- `cargo test -p workflow-core --test local_executor local_check_registration`
  - Passed.
- `cargo test -p workflow-core --test local_executor explicit_docs_check_registration_profile`
  - Passed.

Full validation commands for this phase:

- `cargo fmt --all --check`
  - Passed after applying rustfmt to the implementation edits.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Passed.
- `cargo test --workspace`
  - Passed.
- `npm run check:docs`
  - Passed.

## 12. Remaining Known Limitations

- True ambient default registration remains deferred.
- CLI exposure remains deferred.
- Workflow schema fields remain deferred.
- `AllowlistedHandlerOnly` remains unsupported.
- No production cache/write sandbox exists.
- No command-output evidence policy is implemented.
- Local check evidence attachment remains deferred.
- Broader cargo, TypeScript, contract, integration, and live-provider checks remain deferred.
- Duplicate registration behavior remains inherited from existing registry semantics and should be reviewed before any true default-registration posture.

## 13. Recommended Next Phase

Recommended next phase: **local check handler default-registration review**.

The review should verify that the explicit registration profile/helper is narrow, non-default, non-CLI, redaction-safe, and compatible with the existing local executor boundary. It should also confirm that true default registration, CLI exposure, workflow schema changes, automatic check execution, command-output evidence, side-effect boundary implementation, writes, and release posture changes remain unimplemented.
