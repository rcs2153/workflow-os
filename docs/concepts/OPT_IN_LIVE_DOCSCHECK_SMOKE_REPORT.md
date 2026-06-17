# Opt-In Live DocsCheck Smoke Report

## 1. Executive Summary

The opt-in live DocsCheck smoke is implemented as an ignored Rust test.

The smoke proves that an explicit caller can run the real repository docs check through `DocsCheckLocalHandler` and the standard process runner by supplying:

- an explicit npm executable;
- an explicit npm cache directory;
- the Workflow OS repository root;
- an explicit opt-in environment variable.

The smoke is not automatic runtime behavior. It is not default local check registration, CLI behavior, schema behavior, example activation, evidence attachment, persistence, report artifact writing, source writing, network-enabled execution, arbitrary command execution, or release posture change.

## 2. Scope Completed

Completed:

- added an ignored opt-in live smoke test for `DocsCheckLocalHandler`;
- required `WORKFLOW_OS_LIVE_DOCSCHECK_SMOKE=1` before the smoke runs;
- required explicit `WORKFLOW_OS_LIVE_DOCSCHECK_NPM`;
- required explicit `WORKFLOW_OS_LIVE_DOCSCHECK_NPM_CACHE`;
- ran the canonical `npm run check:docs` command through the existing handler and standard process runner;
- kept `LocalCheckRegistrationProfile::none()` unchanged as the default;
- kept explicit docs-check registration non-default;
- included source-tree mutation detection using before/after `git status --short`;
- kept outputs in bounded in-memory local check summaries;
- adjusted the docs-check sanitized environment so the explicit npm executable's directory is available without inheriting ambient `PATH`;
- updated roadmap and implementation-plan status docs.

## 3. Scope Explicitly Not Completed

Not implemented:

- default local check registration;
- automatic local check execution;
- non-ignored live local check execution;
- CLI flags or commands;
- workflow schema fields;
- example activation;
- broad npm, cargo, TypeScript, contract, integration, or provider check handlers;
- arbitrary user command execution;
- `npm ci`, dependency installation, package updates, or lockfile changes;
- network-enabled local checks;
- source writes;
- command-output evidence attachment;
- local check evidence attachment;
- local check result persistence;
- automatic report artifact writing;
- generic side-effect records;
- write-capable adapters;
- recursive agents, agent swarms, hosted runtime, or distributed runtime behavior;
- release posture changes.

## 4. Implementation Summary

The implementation uses an ignored test:

```text
opt_in_live_docs_check_smoke_runs_real_docs_check_without_source_mutation
```

The test returns without action unless `WORKFLOW_OS_LIVE_DOCSCHECK_SMOKE=1`.

When opted in, the test constructs `DocsCheckLocalHandler` with:

- `LocalCheckCommandContract::docs_check_model_only()`;
- the supplied npm executable path;
- the repository root;
- the supplied npm cache directory;
- the standard process runner.

It then invokes the handler through the existing `SkillHandler` boundary and asserts the local check status is `passed`.

## 5. Environment And Cache Summary

The docs-check environment remains sanitized and allowlisted.

The implementation sets:

- `PATH` to the explicit npm executable's parent directory plus the existing minimal system path;
- `NPM_CONFIG_CACHE` to the explicit cache directory.

It does not inherit the caller's ambient environment. Existing process environment validation still rejects secret-like variable names or values.

## 6. Source-Tree Protection Summary

The live smoke captures `git status --short` before and after the handler invocation and fails if the source-tree status changes.

This reports source mutation without cleaning, reverting, normalizing, or otherwise modifying unrelated user changes.

## 7. Privacy And Redaction Summary

The smoke uses existing `DocsCheckLocalHandler`, `LocalCheckProcessRequest`, `LocalCheckProcessOutput`, and `LocalCheckResult` redaction boundaries.

It does not persist raw stdout, raw stderr, full command transcripts, npm logs, npm debug files, source contents, parser payloads, environment values, local absolute paths, provider payloads, or credentials.

Debug output remains redaction-safe for handlers and process requests.

## 8. Test Coverage Summary

Existing tests continue to cover:

- default registration registers no local check handlers;
- explicit docs-check registration is caller-supplied;
- handler debug output redacts paths and cache values;
- injected-runner success, non-zero exit, timeout, and secret-like output behavior;
- command arguments remain `run` and `check:docs`;
- `NPM_CONFIG_CACHE` is present for docs check invocation.

The new ignored live smoke covers:

- explicit opt-in requirement;
- real `DocsCheckLocalHandler` execution with the standard process runner;
- explicit npm executable and npm cache inputs;
- canonical docs command execution;
- source-tree non-mutation.

## 9. Commands Run And Results

- `cargo fmt --all --check` - passed.
- `cargo test -p workflow-core --test local_check docs_check -- --nocapture` - passed, with the live smoke ignored by default.
- `cargo test -p workflow-core --test local_check opt_in_live_docs_check_smoke -- --ignored --nocapture` with explicit bundled npm/cache environment - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

## 10. Remaining Known Limitations

- The live smoke is an ignored test, not normal CI behavior.
- The smoke validates the docs check command only.
- The smoke does not add default registration or automatic execution.
- The smoke does not implement command-output evidence.
- The smoke does not persist local check results.
- The smoke does not expose CLI behavior or schema fields.
- The smoke does not broaden side-effect policy beyond the explicit npm cache path.
- Other local check families remain deferred.

## 11. Recommended Next Phase

Recommended next phase: opt-in live DocsCheck smoke implementation review.

After review, the roadmap can move to parallel planning for side-effect boundary ADR work, high-assurance approval controls, and write-adapter readiness prerequisites.
