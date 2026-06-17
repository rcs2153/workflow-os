# Opt-In Live DocsCheck Smoke Review

Review date: 2026-06-17

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation adds the intended ignored, explicitly opted-in live DocsCheck smoke without changing default registration, executor behavior, CLI behavior, workflow schemas, examples, persistence, evidence attachment, report artifact behavior, source-write posture, write-capable adapters, or release posture.

The smoke usefully proves that the real repository `npm run check:docs` command can be invoked through `DocsCheckLocalHandler` and the standard process runner when the caller supplies the npm executable, npm cache directory, repository root, and opt-in environment variable.

## 2. Scope Verification

The phase stayed within the approved implementation scope.

Implemented:

- ignored opt-in live smoke for `DocsCheckLocalHandler`;
- explicit `WORKFLOW_OS_LIVE_DOCSCHECK_SMOKE=1` requirement;
- explicit npm executable requirement;
- explicit npm cache directory requirement;
- canonical `npm run check:docs` invocation through the existing handler/process-runner boundary;
- source-tree status comparison before and after the smoke;
- sanitized environment update that exposes only the explicit npm executable directory plus minimal system paths;
- roadmap, plan, and end-of-phase report updates.

No accidental implementation was found for:

- default local check registration;
- automatic local check execution;
- non-ignored live local check execution;
- CLI flags or commands;
- workflow schema fields;
- example activation;
- broad npm, cargo, TypeScript, contract, integration, or provider handlers;
- arbitrary user command execution;
- dependency installation or lockfile mutation;
- command-output evidence attachment;
- local check evidence attachment;
- local check result persistence;
- automatic report artifact writing;
- generic side-effect records;
- write-capable adapters;
- recursive agents, agent swarms, hosted runtime, or distributed runtime behavior;
- release posture changes.

## 3. Live Smoke Boundary Assessment

The live smoke boundary is narrow and appropriate.

The smoke is limited to:

- `LocalCheckCommandKind::DocsCheck`;
- `LocalCheckCommandContract::docs_check_model_only()`;
- caller-supplied npm executable path;
- caller-supplied npm cache directory;
- repository root discovered from the test crate location;
- the canonical `run`, `check:docs` npm argument vector;
- the standard local check process runner.

It does not discover npm from ambient `PATH`, add command arguments, run package installation, resolve dependencies, activate unrelated lifecycle commands, or execute other local check kinds.

## 4. Opt-In And Registration Assessment

The opt-in posture is correct.

The smoke is marked `#[ignore]` and returns without action unless `WORKFLOW_OS_LIVE_DOCSCHECK_SMOKE=1`. That gives two separate operator intent gates: ignored-test selection and environment opt-in.

Default registration remains unchanged. `LocalCheckRegistrationProfile::none()` still registers no handlers, and explicit docs-check registration remains caller supplied. No executor path, project validation path, CLI path, schema path, or example path now registers or executes `DocsCheckLocalHandler` automatically.

## 5. Executable, Cache, And Environment Assessment

The executable and cache handling is consistent with the accepted plan.

`DocsCheckLocalHandler::new` still validates that the supplied npm executable exists and that the supplied repository root has expected Workflow OS markers. The live smoke requires `WORKFLOW_OS_LIVE_DOCSCHECK_NPM` and `WORKFLOW_OS_LIVE_DOCSCHECK_NPM_CACHE` rather than discovering these from ambient user state.

The `docs_check_environment` update is appropriate: it prepends the supplied npm executable's parent directory to a minimal system path so bundled npm can find its adjacent node runtime without inheriting the caller's ambient `PATH`.

The environment remains allowlisted:

- `PATH`;
- `NPM_CONFIG_CACHE`, when explicitly supplied.

Existing environment validation still rejects secret-like names or values.

## 6. Network And Source-Write Assessment

The implementation preserves the current disabled-network contract posture and does not add network-enabled local checks.

Important boundary: this phase does not introduce OS-level network sandboxing for child processes. That is acceptable for this smoke because the command is the repository's docs checker and does not require network access, but future broader live local-check phases should keep this distinction explicit.

The source-write posture is sound for this phase. The smoke captures `git status --short` before and after execution and fails if the working-tree status changes. This detects source-tree mutation while respecting pre-existing user changes because it compares before/after status instead of requiring a clean tree.

The implementation does not clean, revert, normalize, delete, or rewrite user files to make the smoke pass.

## 7. Error Handling Assessment

The implementation uses existing stable local-check construction and process-runner errors.

Covered failure paths include:

- missing npm executable;
- invalid repository root;
- invalid or secret-like environment values;
- unsupported command contract posture;
- process-runner failure;
- non-zero exit status mapped into a bounded local check result;
- timeout mapped into a bounded local check result;
- secret-like stdout/stderr rejection through existing result boundaries.

The live smoke's own `expect(...)` messages are test-only and do not echo supplied path values or secret-like inputs. Production-facing handler errors remain stable and non-leaking.

## 8. Privacy And Redaction Assessment

The privacy boundary remains intact.

The implementation does not persist:

- raw stdout;
- raw stderr;
- full command transcripts;
- npm logs;
- npm debug files;
- source contents;
- parser payloads;
- environment values;
- provider payloads;
- credentials.

`DocsCheckLocalHandler` `Debug` output continues to redact npm executable, repository root, cache directory, and process runner. Process requests and local check results continue to use existing bounded/redacted behavior.

## 9. Test Quality Assessment

The tests cover the important phase behavior:

- injected-runner docs-check success behavior;
- command argument preservation;
- allowlisted environment shape;
- non-zero exit behavior;
- timeout behavior;
- secret-like output handling;
- explicit registration remains non-default;
- ignored live smoke runs only when explicitly selected and opted in;
- live smoke invokes the real docs check through `DocsCheckLocalHandler`;
- live smoke uses explicit npm and cache inputs;
- live smoke checks source-tree non-mutation.

Existing workspace tests also continue to cover local check result references, WorkReport local check citations, terminal report integration, dogfood benchmark behavior, and runtime behavior.

Non-blocking test gaps:

- add a focused assertion that `PATH` begins with the supplied npm executable's parent directory;
- add a small direct test that the live smoke returns early when the environment opt-in is absent, if the team wants explicit coverage beyond the ignored-test gate;
- keep OS-level network sandboxing deferred and documented separately if future local-check live smokes broaden beyond docs.

## 10. Documentation Review

The documentation is honest about current capability.

The updated plan, report, and roadmap state that the ignored opt-in live DocsCheck smoke is implemented.

They also state that the following remain unimplemented:

- default local check registration;
- automatic local check execution;
- non-ignored live local check execution;
- CLI behavior;
- workflow schema fields;
- example activation;
- broad check handler families;
- arbitrary shell execution;
- network-enabled checks;
- source writes;
- command-output evidence;
- local check evidence attachment;
- local check result persistence;
- report artifact auto-writing;
- write-capable adapters;
- release posture changes.

No documentation correction is required for acceptance.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Add a direct unit assertion that the docs-check environment `PATH` includes the explicit npm executable directory first.
- Consider a targeted skipped-by-default assertion for the opt-in environment gate.
- Preserve the distinction between disabled-network contract posture and OS-level network sandbox enforcement in future local-check planning.
- Keep broader cargo, TypeScript, contract, integration, and provider live handlers deferred until separately planned and reviewed.

## 13. Recommended Next Phase

Recommended next phase: side-effect boundary ADR work, high-assurance approval controls planning, or write-adapter readiness prerequisite planning.

The live DocsCheck smoke proves the first real command path is viable under explicit local authority. The next roadmap work should avoid broadening command execution and instead strengthen the governance prerequisites needed before write-capable adapters: durable side-effect modeling, stronger approval controls, evidence attachment boundaries, and report/audit integration.

## 14. Validation

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
