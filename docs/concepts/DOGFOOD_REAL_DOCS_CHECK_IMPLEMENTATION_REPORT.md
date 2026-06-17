# Dogfood Real DocsCheck Implementation Report

Report date: 2026-06-16

## 1. Executive Summary

The self-governance dogfood workflow can now exercise the real `DocsCheckLocalHandler` through explicit local check profile registration.

This phase adds an explicit `local/check-docs` dogfood skill and a `docs-check` workflow checkpoint. Focused tests prove that the dogfood workflow fails closed without an explicitly registered docs-check handler, and completes when a caller supplies `DocsCheckLocalHandler` through `LocalCheckRegistrationProfile::explicit_docs_check(...)` with an injected process runner.

The implementation remains local, explicit, non-default, non-CLI, non-schema, and non-artifact. It does not add automatic check execution, command-output evidence, local check evidence attachment, side-effect boundary modeling, writes, or release posture changes.

## 2. Scope Completed

- Added dogfood skill `local/check-docs`.
- Added dogfood workflow checkpoint `docs-check`.
- Preserved the existing planning approval checkpoint and sequential execution behavior.
- Added deterministic placeholder docs-check behavior for existing mock dogfood tests.
- Added a fail-closed test proving the dogfood docs-check step fails without explicit handler registration.
- Added an explicit-profile test proving `DocsCheckLocalHandler` runs through the dogfood workflow with an injected runner.
- Verified canonical process request construction for `npm run check:docs`.
- Verified the process request uses explicit executable, repository root, and sanitized environment inputs.
- Verified no report artifacts are written by the dogfood docs-check execution path.
- Updated roadmap, local-check planning docs, dogfood docs, and known limitations.

## 3. Scope Explicitly Not Completed

- No true ambient default registration.
- No `LocalSkillRegistry::new()` behavior change.
- No automatic local check execution.
- No CLI flags or commands for real local checks.
- No workflow schema fields for local check registration.
- No workflow-declared local check handlers.
- No runtime config for local checks.
- No `AllowlistedHandlerOnly` enablement.
- No broad handler discovery.
- No arbitrary shell execution.
- No user-supplied command text.
- No cargo, TypeScript, contract, integration, or live-provider check handlers.
- No command-output evidence attachment.
- No local check evidence attachment.
- No raw command transcript storage.
- No automatic report artifact writing.
- No persistence changes.
- No side-effect boundary implementation.
- No source writes.
- No write-capable adapters.
- No release posture change.

## 4. Dogfood Workflow Summary

The dogfood workflow now has six ordered checkpoints:

1. `scope-requested`
2. `planning-approved`
3. `implementation-handoff`
4. `validation-disclosure`
5. `docs-check`
6. `review-and-report-posture`

The `docs-check` checkpoint references `local/check-docs`. It is explicit-handler-only: ordinary default registry construction still fails closed, and CLI mock runs still use deterministic mock local skill behavior.

## 5. Handler And Registration Summary

The real docs-check dogfood test constructs:

- `LocalCheckCommandContract::docs_check_model_only()`;
- `DocsCheckLocalHandler::new_with_process_runner(...)`;
- explicit executable path;
- explicit repository root;
- explicit npm cache directory;
- `LocalCheckRegistrationProfile::explicit_docs_check(...)`;
- `LocalSkillRegistry::register_local_check_profile(...)`.

No handler discovery, `PATH` search, CLI activation, schema activation, or ambient runtime config was added.

## 6. Runtime And Event Boundary Summary

The implementation uses existing local executor behavior only.

- The dogfood run still pauses at `planning-approved`.
- Approval grant resumes downstream ordered steps.
- Missing docs-check handler fails closed with `executor.skill_handler.missing`.
- Explicit docs-check handler success completes the run.
- No post-terminal events are appended.
- No report artifacts are written automatically.
- No out-of-band `StateBackend` writes are introduced.

## 7. Privacy And Redaction Summary

The implementation stores only bounded local check result output produced through existing `LocalCheckResult` behavior.

It does not store or copy raw command output, raw docs contents, parser payloads, provider payloads, environment values, npm tokens, registry credentials, authorization headers, private keys, token-like strings, unbounded local paths, or user-supplied command text.

The new tests use an injected runner and bounded fixture output, not live npm execution.

## 8. Test Coverage Summary

Added and updated tests cover:

- dogfood docs-check fails closed without an explicit docs-check handler;
- dogfood real docs-check completes through explicit profile registration with injected runner;
- canonical `npm run check:docs` process request arguments;
- explicit executable and repository root inputs;
- sanitized environment presence;
- no automatic report artifacts;
- dogfood duplicate run-id rehydration with the new checkpoint;
- dogfood report-bearing execution with the new checkpoint;
- CLI dogfood approval completion through deterministic mock local skills.

Existing local check, local executor, CLI dogfood, report, evidence, validation, adapter telemetry, and runtime tests remain covered by workspace validation.

## 9. Commands Run And Results

- `cargo test -p workflow-core --test local_executor dogfood`
  - Passed after shortening the new checkpoint ID to keep derived idempotency keys within existing limits.
- `cargo test -p workflow-cli --test cli dogfood_multi_step`
  - Passed after updating expected dogfood step order.
- `cargo fmt --all --check`
  - Passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Passed.
- `cargo test --workspace`
  - Passed.
- `npm run check:docs`
  - Passed.
- `git diff --check`
  - Passed.

## 10. Remaining Known Limitations

- The dogfood real DocsCheck path uses injected-runner tests; a live npm smoke posture remains deferred.
- True ambient default registration remains deferred.
- CLI exposure remains deferred.
- Workflow schema activation remains deferred.
- `AllowlistedHandlerOnly` remains unsupported.
- Side-effect/cache/write sandbox policy remains deferred.
- Command-output evidence remains planning-only.
- Local check evidence attachment remains deferred.
- Broader cargo, TypeScript, contract, integration, and live-provider checks remain deferred.

## 11. Recommended Next Phase

Recommended next phase: **dogfood real DocsCheck implementation review**.

The review should verify that the explicit docs-check dogfood path is narrow, fail-closed, non-default, non-CLI, non-schema, non-artifact, redaction-safe, and compatible with existing local executor semantics before any real npm smoke posture, side-effect boundary, or broader local check handler work is considered.
