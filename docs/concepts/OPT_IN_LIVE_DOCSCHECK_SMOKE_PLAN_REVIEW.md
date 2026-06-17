# Opt-In Live DocsCheck Smoke Plan Review

Review date: 2026-06-17

## 1. Executive Verdict

Plan accepted; proceed to opt-in live DocsCheck smoke implementation.

The plan defines a narrow and appropriate live-smoke boundary for proving `DocsCheckLocalHandler` against the repository's real docs command without making local command execution ambient. It keeps the smoke explicit, disabled by default, disabled-network, source-read-only, cache-bounded, non-CLI, non-schema, non-example, and non-persistent.

## 2. Scope Verification

The plan stayed within planning-only scope.

It does not authorize:

- implementation in the planning phase;
- default local check registration;
- automatic local check execution;
- CLI flags or commands;
- workflow schema fields;
- example activation;
- broad npm/cargo/TypeScript/contract/integration/provider check handlers;
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

No accidental scope expansion was found.

## 3. Live Smoke Boundary Assessment

The plan defines a conservative v1 boundary.

The future smoke is limited to:

- `LocalCheckCommandKind::DocsCheck`;
- `LocalCheckCommandContract::docs_check_model_only()`;
- explicit existing npm executable path;
- explicit repository root;
- explicit npm cache directory;
- canonical `npm run check:docs` only;
- standard process runner only in the opt-in live path.

The plan correctly rejects npm discovery from ambient `PATH`, extra arguments, package installation, package resolution, unrelated lifecycle commands, and other local check kinds.

## 4. Opt-In And Registration Assessment

The opt-in posture is correct.

The plan keeps `LocalCheckRegistrationProfile::none()` as the default and limits `LocalCheckRegistrationProfile::explicit_docs_check(...)` to caller-supplied explicit registration. It does not add default executor registration, automatic project validation behavior, CLI activation, schema activation, or example activation.

The future implementation should use a clearly intentional opt-in mechanism, such as an ignored Rust test or explicit environment-gated live smoke, and should remain excluded from normal CI unless CI explicitly supplies the required npm path/cache posture.

## 5. Executable, Cache, And Environment Assessment

The explicit npm executable policy is appropriate.

The future implementation must validate the supplied npm path rather than search ambient `PATH`. The plan's preferred local candidate, `.tools/node-v20.19.5-darwin-arm64/bin/npm`, is consistent with current repository tooling, but the handler should still treat it as supplied input.

The explicit npm cache requirement is also appropriate. The plan correctly identifies `.tools/npm-cache` as the current repository-local cache convention while deferring the exact representation question: validated relative model directory, explicit absolute runtime path, or both.

The environment policy is conservative:

- empty baseline;
- `NPM_CONFIG_CACHE` only;
- minimal deterministic `PATH` only if needed;
- no inherited user environment;
- secret-like variable names or values fail closed.

## 6. Disabled Network Assessment

The disabled-network posture is correct.

The plan rejects registry credentials, provider tokens, npm auth tokens, authorization headers, private keys, broad npm configuration, `npm ci`, `npm install`, `npm audit`, `npm fund`, package resolution, and package download.

If `npm run check:docs` attempts network access, the future smoke should fail rather than broadening the boundary. Any network-enabled smoke requires separate planning and review.

## 7. Source-Write Boundary Assessment

The plan treats the live smoke as source-read-only, which is the right default.

It rejects modifications to:

- docs;
- source files;
- scripts;
- examples;
- specs;
- schemas;
- manifests;
- lockfiles;
- generated source;
- Workflow OS state files;
- report artifact directories;
- provider fixtures;
- repository metadata.

The plan allows cache writes only in the explicit npm cache directory after implementation accepts that side-effect classification. It also calls for source-tree mutation detection that reports failures without cleaning or reverting user files.

## 8. Error Handling Assessment

The error posture is stable and non-leaking.

Required failure cases are concrete:

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

The plan correctly forbids error messages from echoing local absolute paths, rejected secret-like values, command output, environment values, source snippets, npm config payloads, registry metadata, tokens, credentials, or private paths.

## 9. Privacy And Redaction Assessment

The privacy boundary is appropriate.

The future smoke may produce only bounded redacted summaries in memory. It must not persist raw stdout, raw stderr, full transcripts, npm logs, npm debug files, source contents, parser payloads, environment values, local absolute paths, provider payloads, or credentials.

The plan keeps `Debug` output redacted for handlers, requests, results, and registration profiles.

## 10. Test Plan Assessment

The planned tests cover the important safety boundaries:

- opt-in live smoke skipped unless explicitly enabled;
- default registration remains empty;
- explicit docs-check registration remains safe metadata only;
- npm executable must be supplied and not ambiently discovered;
- npm cache directory is required;
- secret-like cache paths fail without leaking;
- environment is allowlisted;
- network policy remains disabled;
- command arguments stay exactly `run` and `check:docs`;
- non-zero exit maps to failed local check result;
- timeout maps to timed-out result;
- secret-like stdout/stderr fails without leaking;
- source-tree mutation detection fails the smoke without reverting user changes;
- normal docs, schema, CLI, and example checks do not activate live local checks.

Non-blocking implementation guidance:

- The source-tree mutation check should avoid treating unrelated pre-existing dirty files as smoke-created mutations.
- The implementation report should clearly state whether the opt-in mechanism is an ignored test, feature flag, environment variable, or manual target.

## 11. Documentation Review

The plan is honest about current capabilities.

It says implementation is not done. It also states that live execution, default registration, CLI behavior, workflow schema fields, examples, evidence attachment, report artifact writing, source writes, network access, and release posture changes are not added.

No documentation correction is required before implementation.

## 12. Planning Blockers

None.

## 13. Non-Blocking Follow-Ups

- Decide whether the first implementation should be an ignored Rust test or environment-gated live smoke.
- Decide whether the npm cache directory should be represented as a validated relative model directory, explicit absolute runtime path, or both.
- Document how the smoke compares source-tree state without reverting or normalizing user changes.
- Keep cargo, TypeScript, contract, integration, and provider live handlers deferred.

## 14. Recommended Next Phase

Recommended next phase: opt-in live DocsCheck smoke implementation.

The implementation should remain narrow: explicit npm path, explicit npm cache, disabled-network posture, source-read-only mutation detection, no default registration, no CLI/schema/example activation, no persistence, no evidence attachment, no report artifact writing, no side effects beyond declared cache writes, and no release posture changes.

## 15. Validation

- `npm run check:docs` - passed.
