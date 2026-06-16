# Local Check Handler Default-Registration Review

Review date: 2026-06-16

## 1. Executive Verdict

Phase accepted; proceed to dogfood real DocsCheck planning.

The implementation adds an explicit, non-default local check registration profile/helper and preserves the current safe-by-default runtime posture. It does not introduce ambient default registration, CLI exposure, workflow schema behavior, automatic check execution, command-output evidence, side-effect modeling, writes, or release posture changes.

## 2. Scope Verification

The phase stayed within the approved explicit-profile scope.

Implemented scope:

- `LocalCheckRegistrationMode`;
- `LocalCheckRegisteredHandler`;
- `LocalCheckRegistrationProfile`;
- `LocalSkillRegistry::register_local_check_profile(...)`;
- safe planned-handler metadata for the explicit `DocsCheck` profile;
- focused local-check and executor tests;
- roadmap/planning documentation updates;
- phase report.

No accidental scope expansion was found:

- no true ambient default registration;
- no automatic local check execution;
- no CLI behavior;
- no workflow schema fields;
- no workflow-declared local check registration;
- no `AllowlistedHandlerOnly` enablement;
- no broad handler discovery;
- no arbitrary shell execution;
- no additional cargo, TypeScript, contract, integration, or provider handlers;
- no command-output evidence;
- no local check evidence attachment;
- no report artifact writing;
- no persistence changes;
- no side-effect boundary implementation;
- no source writes;
- no write-capable adapters;
- no release posture change.

## 3. Registration Model Assessment

The model is narrow and appropriate for this phase.

`LocalCheckRegistrationMode` supports only:

- `None`;
- `ExplicitDocsCheck`.

`LocalCheckRegistrationProfile::none()` carries no handler and exposes no planned handlers. `LocalCheckRegistrationProfile::explicit_docs_check(...)` requires a caller-supplied `DocsCheckLocalHandler` and exposes only safe planned-handler metadata.

The model does not represent rejected modes such as `All`, `FromWorkflowSpec`, `FromEnvironment`, `FromPath`, `AllowlistedHandlerOnly`, broader npm/cargo families, or arbitrary commands.

## 4. Default-Safe Behavior Assessment

`LocalSkillRegistry::new()` remains empty/default-safe.

The `None` profile registers nothing and preserves missing-handler behavior for docs-check workflows. The existing and new tests verify that a docs-check workflow fails closed with `executor.skill_handler.missing` unless a handler is explicitly registered.

This is the right distinction: the phase improves explicit wiring ergonomics without changing the authority profile of ordinary local executor construction.

## 5. Authorization Boundary Assessment

Execution authority remains explicit.

The profile does not construct a handler, infer a repository root, resolve executable paths, search `PATH`, create cache directories, read ambient environment, or run commands. `ExplicitDocsCheck` consumes a prebuilt `DocsCheckLocalHandler`, so executable/cache/toolchain decisions remain at the existing explicit handler-construction boundary.

Serialized workflow specs still cannot activate local command execution. `AllowlistedHandlerOnly` remains unsupported. Registered handlers continue to execute through existing skill-handler policy and invocation flow.

## 6. Runtime And Event Boundary Assessment

The implementation uses existing runtime mechanics only.

The explicit profile path can register `DocsCheck` and then run through normal `LocalExecutor` skill invocation. No new runtime event kinds were introduced. No automatic check execution was added. No post-terminal events, audit events, observability events, report artifact writes, or out-of-band `StateBackend` writes were introduced by the helper.

The executor test verifies explicit profile registration can execute with an injected runner and leaves the report artifact list empty.

## 7. Privacy And Redaction Assessment

The privacy posture is acceptable for this phase.

`LocalCheckRegistrationProfile` has a custom `Debug` implementation that reports only the mode, planned-handler count, and a redacted handler marker. Tests verify Debug output does not leak repository paths, cache names, or canonical command arguments.

The profile stores a supplied handler internally, but does not expose it through accessors or Debug. Planned-handler metadata is limited to command kind, canonical skill ID, and canonical skill version.

No raw command output, docs contents, parser payloads, provider payloads, environment values, npm tokens, registry credentials, authorization headers, private keys, token-like strings, unbounded paths, or command arguments are copied into new reportable fields.

## 8. Error Handling Assessment

The only explicit registration-profile error path is an internally inconsistent `ExplicitDocsCheck` profile without a handler, returning stable code `local_check.registration.docs_check_missing`.

That state is not constructible through the public constructors. The error message is stable and does not include handler details, paths, command arguments, environment values, output, or secret-like payloads.

Existing missing-handler behavior remains the stable executor failure path for unregistered local skills.

## 9. Test Quality Assessment

Test coverage is focused and adequate for this phase.

Covered:

- `None` profile metadata and Debug behavior;
- `ExplicitDocsCheck` profile safe metadata;
- redaction-safe profile Debug;
- default registry remains safe when registering `None`;
- docs-check workflow still fails closed without explicit handler;
- explicit profile registration executes through `LocalExecutor` with injected runner;
- canonical docs-check process arguments are preserved;
- no report artifacts are written by the explicit profile execution path;
- full existing workspace tests pass.

Test limitations:

- Duplicate registration behavior is not tested specifically for profiles. It currently inherits existing registry overwrite semantics. This is acceptable for this phase because true default registration is still deferred, but should be reviewed before any ambient or schema-driven registration posture.

## 10. Documentation Review

Documentation accurately states:

- explicit local check registration profile/helper is implemented;
- true ambient default registration is not implemented;
- `LocalSkillRegistry::new()` remains default-safe;
- CLI exposure is not implemented;
- workflow schema exposure is not implemented;
- automatic local check execution is not implemented;
- command-output evidence is not implemented;
- side-effect boundary and writes remain unsupported.

The new report clearly records the implemented scope, non-scope, validation boundary, dogfood governance run, validation commands, limitations, and recommended next phase.

## 11. Blockers

No blockers.

## 12. Non-Blocking Follow-Ups

- Review duplicate registration semantics before any true default-registration, workflow-declared registration, or broader handler-registration posture.
- Consider whether future profile metadata should include a stable handler registration category if additional handler families are introduced.
- Keep true default registration blocked until side-effect/cache/write sandbox policy, CLI/schema posture, and command-output evidence policy are reviewed.

## 13. Recommended Next Phase

Recommended next phase: dogfood real DocsCheck planning.

The explicit profile/helper has closed the safe wiring gap without making local checks ambient. The next useful planning phase should decide whether the self-governance dogfood workflow can use a real `DocsCheckLocalHandler` through explicit registration, while still avoiding CLI exposure, workflow schema changes, true default registration, command-output evidence, side-effect modeling, writes, report artifact auto-writing, and release posture changes.

## 14. Validation

Validation commands for this review:

- `cargo fmt --all --check`
  - Passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Passed.
- `cargo test --workspace`
  - Passed.
- `npm run check:docs`
  - Passed.
