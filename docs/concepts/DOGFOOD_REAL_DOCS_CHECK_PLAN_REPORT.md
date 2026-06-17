# Dogfood Real DocsCheck Plan Report

Report date: 2026-06-16

Implementation update: the follow-on implementation phase is now complete and documented in [Dogfood Real DocsCheck Implementation Report](DOGFOOD_REAL_DOCS_CHECK_IMPLEMENTATION_REPORT.md). This planning report preserves the original planning findings below.

## 1. Executive Summary

Created a planning document for the next local-check dogfood phase: using the existing explicit `DocsCheckLocalHandler` through explicit local check registration in the self-governance dogfood workflow.

The plan keeps the phase narrow. It recommends an explicit-profile, injected-runner-first implementation and keeps true default registration, CLI exposure, workflow schema fields, automatic check execution, command-output evidence, side-effect modeling, writes, report artifact auto-writing, and release posture changes out of scope.

## 2. Scope Completed

- Added [Dogfood Real DocsCheck Plan](../implementation-plans/dogfood-real-docs-check-plan.md).
- Positioned real DocsCheck dogfooding after the explicit local check registration profile/helper review.
- Defined the handler construction and registration boundary.
- Defined workflow integration boundaries for the self-governance dogfood path.
- Defined side-effect, result, evidence, report, privacy, and error-handling boundaries.
- Defined future test coverage and implementation sequence.
- Updated roadmap and local-check planning documents to link the plan.

## 3. Scope Explicitly Not Completed

- No real DocsCheck dogfood implementation.
- No true ambient default registration.
- No `LocalSkillRegistry::new()` behavior change.
- No automatic local check execution.
- No CLI exposure.
- No workflow schema fields.
- No workflow-declared local check registration.
- No runtime config for local checks.
- No `AllowlistedHandlerOnly` enablement.
- No broad handler discovery.
- No arbitrary shell execution.
- No cargo, TypeScript, contract, integration, or live-provider handlers.
- No command-output evidence.
- No local check evidence attachment.
- No automatic report artifact writing.
- No persistence changes.
- No side-effect boundary implementation.
- No source writes.
- No write-capable adapters.
- No release posture change.

## 4. Planning Boundary Summary

The plan recommends the next implementation phase prove real DocsCheck dogfooding with explicit caller-owned inputs:

- `LocalCheckCommandContract::docs_check_model_only()`;
- explicit `DocsCheckLocalHandler`;
- explicit npm executable path;
- explicit repository root;
- explicit npm cache directory;
- `LocalCheckRegistrationProfile::explicit_docs_check(...)`;
- `LocalSkillRegistry::register_local_check_profile(...)`;
- existing local executor behavior.

It rejects ambient discovery, `PATH` search, CLI activation, schema activation, handler defaults, and fabricated IDs.

## 5. Privacy And Redaction Summary

The plan keeps command output and local paths sensitive by default.

Future implementation must not store or copy raw command output, raw docs contents, parser payloads, provider payloads, environment values, npm tokens, registry credentials, authorization headers, private keys, token-like strings, unbounded local paths, or non-canonical command text.

Errors must use stable codes and avoid raw paths, snippets, output, environment names, and secret-like values.

## 6. Test Coverage Plan Summary

The future implementation test plan covers:

- explicit profile dogfood execution with injected runner;
- default-safe registry behavior;
- canonical `npm run check:docs` process request construction;
- explicit executable, repository root, cache, and sanitized environment behavior;
- bounded local check result mapping;
- secret-like output rejection;
- non-leaking errors;
- no automatic artifact writes;
- no CLI output;
- existing workspace regressions.

## 7. Commands Run And Results

- `npm run check:docs`
  - Passed.
- `git diff --check`
  - Passed.

## 8. Remaining Known Limitations

- The real DocsCheck dogfood run is not implemented yet.
- A real npm smoke test posture remains undecided.
- Side-effect/cache/write sandbox policy remains deferred.
- `AllowlistedHandlerOnly` remains unsupported.
- Command-output evidence remains planning-only.
- Local check evidence attachment remains deferred.
- True default registration remains deferred.

## 9. Recommended Next Phase

Recommended next phase: **dogfood real DocsCheck implementation, explicit-profile and injected-runner first**.

That phase should add the smallest test/helper path proving the self-governance workflow can use `DocsCheckLocalHandler` through explicit registration while preserving safe-by-default runtime behavior.
