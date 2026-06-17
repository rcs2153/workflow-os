# Dogfood Real DocsCheck Implementation Review

Review date: 2026-06-16

## 1. Executive Verdict

Phase accepted; proceed to local check side-effect boundary planning.

The implementation proves a narrow dogfood path for real `DocsCheckLocalHandler` execution through explicit profile registration and an injected runner. It preserves the safe-by-default posture: no ambient default registration, no CLI activation of real local checks, no workflow schema activation, no automatic check execution, no command-output evidence, no local check evidence attachment, no report artifact auto-writing, no side-effect boundary implementation, no writes, and no release posture change.

## 2. Scope Verification

The phase stayed within the approved explicit dogfood scope.

Implemented scope:

- added dogfood skill `local/check-docs`;
- added dogfood checkpoint `docs-check`;
- kept the checkpoint behind existing local executor handler registration;
- added deterministic mock output support for local check skills under `--mock-all-local-skills`;
- added fail-closed coverage when no docs-check handler is registered;
- added explicit-profile/injected-runner coverage for `DocsCheckLocalHandler`;
- updated roadmap, known limitations, dogfood docs, planning docs, and implementation report.

No accidental scope expansion was found:

- no true ambient default registration;
- no `LocalSkillRegistry::new()` behavior change;
- no automatic local check execution;
- no CLI flag or command for real local checks;
- no workflow schema field for local check registration;
- no workflow-declared handler activation;
- no runtime config for local checks;
- no `AllowlistedHandlerOnly` enablement;
- no broad handler discovery;
- no arbitrary shell execution;
- no cargo, TypeScript, contract, integration, or live-provider handler;
- no command-output evidence;
- no local check evidence attachment;
- no automatic report artifact writing;
- no persistence change;
- no side-effect boundary implementation;
- no source writes;
- no release posture change.

## 3. Dogfood Workflow Assessment

The dogfood workflow now has six ordered checkpoints:

1. `scope-requested`
2. `planning-approved`
3. `implementation-handoff`
4. `validation-disclosure`
5. `docs-check`
6. `review-and-report-posture`

The new `docs-check` step uses `local/check-docs`, maps only a bounded literal request marker, and preserves sequential behavior after the planning approval gate. The short step ID avoids exceeding existing derived idempotency-key length limits, which is preferable to loosening identity validation.

The workflow remains kernel-governed and Codex/human-executed. The docs-check checkpoint is explicit-handler-only and does not imply broad build-command execution.

## 4. Handler And Registration Assessment

The explicit real docs-check test uses the intended boundary:

- `LocalCheckCommandContract::docs_check_model_only()`;
- `DocsCheckLocalHandler::new_with_process_runner(...)`;
- explicit executable path;
- explicit repository root;
- explicit npm cache directory;
- `LocalCheckRegistrationProfile::explicit_docs_check(...)`;
- `LocalSkillRegistry::register_local_check_profile(...)`;
- existing `LocalExecutor` behavior.

This is minimal and idiomatic for the current codebase. The implementation does not discover tools, search `PATH`, infer repository roots, read ambient environment, register handlers from specs, or make local checks available by default.

## 5. Fail-Closed Behavior Assessment

Fail-closed behavior is adequately covered.

The new `dogfood_docs_check_step_fails_closed_without_explicit_docs_handler` test runs the real dogfood workflow with only the governance placeholder handler. After approval, the workflow reaches the docs-check boundary and fails with stable code `executor.skill_handler.missing`.

The test also verifies:

- the docs-check step does not succeed without a handler;
- downstream review/report posture does not run;
- the failure message does not leak the canonical command text `check:docs`.

This preserves the safe-by-default runtime posture.

## 6. Runtime And Event Boundary Assessment

The phase uses existing runtime mechanics only.

The implementation:

- preserves the `planning-approved` approval gate;
- preserves sequential ordered execution;
- preserves duplicate run-id rehydration coverage;
- preserves report-bearing dogfood execution through existing explicit APIs;
- appends no post-terminal events;
- writes no report artifacts automatically;
- introduces no runtime event kinds;
- introduces no out-of-band `StateBackend` writes.

The CLI mock handler change is bounded to deterministic mock output for `local/check-*` skills. It does not execute real commands and exists so mock dogfood runs satisfy the new local check skill output contract.

## 7. Process Request And Output Assessment

The explicit-profile test verifies the generated process request uses:

- the supplied executable path;
- canonical arguments `run`, `check:docs`;
- the supplied repository root;
- sanitized environment entries including `PATH` and `NPM_CONFIG_CACHE`.

The test uses an injected process runner and bounded fixture output. It does not invoke live npm. The successful output reference is a bounded local check result reference beginning with `local-check-result/local-check/docs/passed`.

This is the right dogfood implementation shape before any live smoke posture is considered.

## 8. Privacy And Redaction Assessment

The privacy posture is acceptable.

The phase does not store or copy:

- raw command output;
- raw docs contents;
- parser payloads;
- provider payloads;
- environment values beyond sanitized request construction;
- npm tokens;
- registry credentials;
- authorization headers;
- private keys;
- token-like strings;
- unbounded local paths;
- user-supplied command text.

Errors remain stable and non-leaking in the reviewed paths. Existing `DocsCheckLocalHandler`, `LocalCheckProcessRequest`, `LocalCheckResult`, and local check result redaction tests continue to cover the deeper model boundary.

## 9. Documentation Review

Documentation was updated honestly.

Docs now state:

- dogfood real DocsCheck execution through explicit profile registration is implemented;
- the self-governance dogfood workflow has an explicit docs-check checkpoint;
- default registration is not implemented;
- CLI exposure is not implemented;
- workflow schema activation is not implemented;
- automatic local check execution is not implemented;
- command-output evidence is not implemented;
- local check evidence attachment is not implemented;
- report artifacts are not automatically written;
- side-effect boundary and writes remain unsupported.

One stale plan subsection still listed dogfood execution as not implemented; it was corrected during this review as a tiny documentation fix to avoid a false current-state claim. Historical reports and reviews that described earlier five-step behavior were left intact as historical records.

## 10. Test Quality Assessment

Test coverage is focused and adequate for this phase.

Covered:

- dogfood cancellation still stops downstream steps before docs-check;
- dogfood fails closed at docs-check without explicit handler;
- dogfood completes through explicit docs-check profile with injected runner;
- canonical process request arguments are preserved;
- explicit executable and repository root are used;
- sanitized environment keys are present;
- no automatic report artifacts are written;
- duplicate run-id rehydration includes the new checkpoint;
- report-bearing dogfood execution still works through existing explicit APIs;
- CLI mock dogfood approval completes all six steps deterministically.

Remaining test limitations:

- No live npm smoke test is included. This is intentional and non-blocking because live/local process posture needs a separate decision.
- The test checks sanitized environment keys are present, but does not exhaustively assert that all forbidden environment variables are absent in this dogfood path. Existing local check environment tests cover that lower-level boundary.

## 11. Blockers

No blockers.

## 12. Non-Blocking Follow-Ups

- Plan the local check side-effect/cache/write boundary before any live npm smoke or broader cargo/npm handlers.
- Decide whether a live docs-check dogfood smoke should be opt-in, local-only, or deferred until side-effect policy exists.
- Consider adding dogfood-specific assertions for absence of known credential environment variable names if future handler setup broadens environment construction.
- Review duplicate registration/overwrite semantics before any schema-driven, CLI-driven, or default registration posture.

## 13. Recommended Next Phase

Recommended next phase: **local check side-effect boundary planning**.

The explicit dogfood docs-check path is now proven through injected-runner tests. Before running live npm as part of dogfood or broadening to cargo/clippy/test handlers, Workflow OS needs a clearer model for local command side effects: source writes versus cache writes, permitted directories, cleanup posture, network expectations, and how those constraints should be represented without pretending checks are side-effect-free.

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
- `git diff --check`
  - Passed.
