# Opt-In Live DocsCheck Smoke Plan

Status: Planning only. Implementation is not done. This document does not add live execution, default registration, CLI behavior, workflow schema fields, example activation, evidence attachment, report artifact writing, source writes, network access, or release posture changes.

## 1. Executive Summary

Workflow OS has a model-only local check command contract, a fine-grained local check side-effect boundary, a production-shaped `DocsCheckLocalHandler`, and focused injected-runner tests for `npm run check:docs` behavior.

The next narrow step is to plan an opt-in live DocsCheck smoke that can prove the `DocsCheck` handler against the repository's real docs check command without making local command execution ambient.

This plan defines the boundary for that future smoke. The smoke must be explicit, test-scoped, disabled-network, source-read-only, and bound to an explicit npm executable plus explicit npm cache policy. It must not become default runtime behavior, CLI behavior, schema behavior, example behavior, evidence attachment, or a general npm/local-command facility.

Implementation is not done.

## 2. Goals

- Define the smallest acceptable future live DocsCheck smoke boundary.
- Preserve `Agent executes. Workflow OS governs.` as the operating model.
- Exercise only `LocalCheckCommandKind::DocsCheck`.
- Use the canonical `npm run check:docs` argument vector.
- Require an explicit npm executable supplied by the caller or test harness.
- Require an explicit npm cache directory before live execution.
- Use sanitized, allowlisted, non-secret environment variables only.
- Keep `LocalCheckNetworkPolicy::Disabled`.
- Prohibit repository source writes and generated source writes.
- Keep the smoke opt-in and non-default.
- Preserve bounded output capture and redaction-safe result handling.
- Keep failures explicit, stable, and non-leaking.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- default local check registration;
- automatic local check execution;
- CLI flags or commands for live local checks;
- workflow schema fields;
- example activation;
- broad npm, cargo, TypeScript, contract, integration, or provider check handlers;
- arbitrary user command execution;
- `npm ci`, dependency installation, package updates, or lockfile changes;
- network access;
- source writes;
- command-output evidence attachment;
- local check evidence attachment;
- local check result persistence;
- automatic report artifact writing;
- generic side-effect records;
- write-capable adapters;
- recursive agents, agent swarms, hosted runtime, or distributed runtime behavior;
- release posture changes.

## 4. Live Smoke Boundary

The future smoke should be a focused opt-in test or explicitly named local developer check. It should construct a `DocsCheckLocalHandler` with:

- `LocalCheckCommandContract::docs_check_model_only()`;
- an explicit existing npm executable path;
- the Workflow OS repository root;
- an explicit npm cache directory;
- the standard process runner only in the opt-in live test path.

The smoke must invoke only the canonical command:

```text
npm run check:docs
```

It must not discover handlers, discover npm from ambient `PATH`, add command arguments, run package installation, run package lifecycle commands beyond the canonical script, or execute any other local check kind.

The smoke should be disabled by default in normal Rust tests and normal docs checks. A future implementation may use an explicit ignored test, feature-gated test, environment opt-in, or manually invoked test target, but it must require an intentional operator action.

## 5. Explicit npm Executable, Cache, And Environment Policy

The future smoke must require the caller to supply the npm executable path. In this repository, the expected local candidate is the bundled npm under `.tools/node-v20.19.5-darwin-arm64/bin/npm` when present, but the handler must validate the supplied path rather than search for it.

The future smoke must require an explicit npm cache directory. The recommended repository-local candidate is `.tools/npm-cache` because it is already the established project cache location. A future implementation must decide whether the side-effect boundary should model this as a validated relative cache directory, an explicit absolute runtime path, or both. In all cases, it must not use npm's default user cache, home directory, registry credentials, or ambient npm configuration.

The live environment should be constructed from an empty baseline plus only the required non-secret values:

- `NPM_CONFIG_CACHE`, set to the explicit cache directory;
- a minimal `PATH` only if needed for the supplied npm executable to find the bundled Node runtime.

If `PATH` is needed, it should be deterministic and narrow, preferably including only the bundled Node directory plus system directories required by the platform. It must not inherit the user's ambient environment. Secret-like variable names and values must fail closed.

## 6. Disabled Network Posture

The smoke must keep `LocalCheckNetworkPolicy::Disabled`.

The future implementation must not pass registry credentials, provider tokens, npm auth tokens, authorization headers, private keys, or broad npm configuration. It must not run `npm ci`, `npm install`, `npm audit`, `npm fund`, package resolution, package download, or any command expected to contact a registry.

If `npm run check:docs` attempts network access, the smoke should fail and report that the docs check is not acceptable for the disabled-network local check boundary. Network-enabled smokes require a separate plan and review.

## 7. No Source Writes

The smoke must be treated as source-read-only.

It must not modify:

- docs, source files, scripts, examples, specs, schemas, manifests, lockfiles, or generated source;
- Workflow OS state files;
- report artifact directories;
- provider fixture files;
- repository metadata.

Cache writes are permitted only in the explicit npm cache directory once the future implementation accepts that side-effect classification. The smoke should include a source-tree protection check before and after execution, such as `git status --short` comparison scoped to tracked and untracked source paths, while preserving unrelated pre-existing user changes.

The implementation must never clean, delete, rewrite, or normalize unrelated files to make the smoke pass.

## 8. No Default Registration

The smoke must not change the default handler registry.

`LocalCheckRegistrationProfile::none()` must remain the default posture. `LocalCheckRegistrationProfile::explicit_docs_check(...)` may be used only by the opt-in smoke path with a caller-supplied handler. No default constructor, executor path, project validation path, or report path should register `DocsCheckLocalHandler` automatically.

## 9. No CLI, Schema, Or Example Activation

The future smoke must not add:

- CLI commands or flags for live DocsCheck execution;
- workflow schema fields for local checks;
- example workflows that imply live local checks are default or production-supported;
- docs that tell users local checks run automatically;
- generated schema updates;
- public release notes claiming live local check support.

Any CLI, schema, or example activation requires a separate implementation plan and review.

## 10. Error Handling

Errors must fail closed with stable, non-leaking codes.

Required failure cases include:

- missing or non-file npm executable;
- missing repository root markers;
- missing explicit npm cache directory;
- secret-like cache path, executable path, environment name, environment value, stdout, or stderr;
- command template mismatch;
- unsupported command kind;
- unsupported working directory, network, environment, or side-effect posture;
- timeout;
- non-zero exit status;
- output truncation;
- redaction failure;
- source-tree mutation detection.

Error messages must not echo local absolute paths, rejected secret-like values, command output, environment values, source snippets, npm config payloads, registry metadata, tokens, credentials, or private paths.

## 11. Privacy And Redaction

The smoke may produce only bounded redacted summaries in memory.

It must not persist raw stdout, raw stderr, full command transcripts, npm logs, npm debug files, source contents, parser payloads, environment values, local absolute paths, provider payloads, or credentials.

`Debug` output for handlers, requests, results, and registration profiles must continue to redact local paths, command details where applicable, cache paths, output, and environment values. Any future serialized result must follow the existing `LocalCheckResult` and `LocalCheckResultReference` posture: structured status and bounded summaries only, with secret-like content rejected.

## 12. Tests

A future implementation should add focused tests for:

- opt-in live smoke is skipped unless explicitly enabled;
- default registration still registers no local check handlers;
- explicit docs-check registration remains safe metadata only;
- supplied npm executable must exist and is not discovered from ambient `PATH`;
- npm cache directory is required for the live smoke path;
- secret-like cache paths fail without leaking;
- environment contains only the accepted non-secret variables;
- network policy remains disabled;
- command arguments remain exactly `run` and `check:docs`;
- non-zero exit maps to `LocalCheckResultStatus::Failed`;
- timeout maps to `LocalCheckResultStatus::TimedOut`;
- secret-like stdout or stderr fails without leaking;
- source-tree changes fail the smoke without reverting user changes;
- normal docs, schema, CLI, and example checks do not activate live local checks.

The future live smoke should be excluded from normal CI unless CI explicitly opts in with the required local npm path, cache path, and disabled-network posture.

## 13. Proposed Sequence

1. Review and accept this planning document.
2. Decide the exact opt-in mechanism: ignored Rust test, feature-gated Rust test, environment-gated live smoke, or manually invoked local test target.
3. Decide the cache directory representation: validated relative model directory, explicit absolute runtime path, or both.
4. Add tests that preserve default non-registration and opt-in-only behavior.
5. Add the live smoke path using `DocsCheckLocalHandler` and the standard process runner.
6. Add source-tree mutation detection that reports failures without reverting or cleaning user files.
7. Run focused Rust local check tests and `npm run check:docs`.
8. Produce an implementation report that states live DocsCheck smoke remains opt-in and that CLI/schema/example activation remains unimplemented.

## 14. Final Recommendation

Proceed next with a narrow opt-in live DocsCheck smoke implementation only after this plan is reviewed.

The implementation should prove one thing: a caller that explicitly supplies a reviewed npm executable, explicit npm cache directory, repository root, and opt-in execution path can run the canonical docs check through `DocsCheckLocalHandler` without source writes, network posture broadening, default registration, CLI/schema/example activation, evidence attachment, report artifact writing, or release posture changes.

Implementation is not done.
