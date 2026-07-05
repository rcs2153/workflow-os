# GitHub PR Comment SideEffect Event Append Helper Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The GitHub PR comment SideEffect event append helper is appropriately narrow, local, explicit, and reference-only. It bridges a persisted proposed GitHub pull request comment `SideEffectRecord` into the existing `LocalExecutionSideEffectEventInput` shape without calling GitHub, mutating providers, appending events by itself, writing artifacts, adding CLI behavior, changing schemas, or broadening release posture.

The implementation is safe to proceed to the next focused phase after the follow-ups below are tracked. The most important next proof is an end-to-end executor test that uses the GitHub-specific helper output as a supplied `LocalExecutionRequest.side_effect_events` input and verifies the existing executor append/audit path.

## 2. Scope Verification

The phase stayed within the approved helper-only scope.

Implemented:

- `GitHubPullRequestCommentSideEffectAppendInput`.
- `load_github_pr_comment_proposed_side_effect_event_input(...)`.
- Reuse of `load_github_pr_comment_proposed_side_effect_event(...)`.
- Reuse of `LocalExecutionSideEffectEventInput`.
- Stable non-leaking helper-boundary error mapping.
- Focused provider-write tests.
- Roadmap, integration, plan, and implementation-report updates.

No accidental scope expansion was found:

- No GitHub provider calls.
- No GitHub PR comment mutation.
- No live sandbox write behavior.
- No runtime side-effect execution.
- No attempted/completed/failed lifecycle support.
- No automatic event append after record persistence.
- No new executor method.
- No report artifact writing.
- No CLI behavior.
- No workflow schema fields.
- No examples.
- No hosted behavior.
- No reasoning lineage.
- No autonomy expansion.
- No release posture change.

## 3. Helper API Assessment

The helper API is appropriately small and explicit.

`GitHubPullRequestCommentSideEffectAppendInput` carries only the values needed to load a persisted proposed record, verify the expected context, target the existing local executor append input, and optionally check correlation identity. It does not read hidden global state, infer runtime configuration, touch live adapters, require credentials, or perform provider work.

The public helper returns `LocalExecutionSideEffectEventInput`, which is the right boundary for this phase because the executor already owns event append semantics, ordering, idempotency, and audit projection. Adding a separate executor method would have been broader than necessary.

## 4. Event Input Mapping Assessment

The helper correctly:

- loads the record by explicit `SideEffectId`;
- delegates persisted-record validation and proposed-event construction to the existing GitHub PR comment proposed-event helper;
- maps lower-level GitHub proposed-event errors to helper-specific stable codes;
- verifies target step/skill/version when those values are present on the event;
- verifies correlation mismatch when an expected correlation ID is provided and the event has a correlation ID;
- returns a `LocalExecutionSideEffectEventInput` without appending it.

This preserves the planned source-of-truth split:

- `SideEffectRecordStore` remains the durable proposed write-intent source.
- The GitHub proposed-event helper remains the reference-only event payload constructor.
- `LocalExecutionSideEffectEventInput` remains the explicit executor append input.
- The executor remains the only append boundary.

## 5. Validation And Error Handling Assessment

Validation behavior is deterministic and fail-closed at the helper boundary.

Stable codes are present for:

- missing record;
- store read failure;
- identity mismatch;
- invalid record conversion;
- target mismatch;
- correlation mismatch.

Errors are intentionally generic and do not include side-effect IDs, workflow IDs, run IDs, repository names, pull request numbers, target references, summaries, comment bodies, spec hashes, provider references, redaction metadata, or token-like values.

One non-blocking gap remains: focused tests cover missing record, target mismatch, and correlation mismatch through the helper, while some invalid-record categories are covered at the lower proposed-event helper layer rather than through the new helper boundary. That is acceptable for this phase, but the next regression pass should add at least one helper-level invalid-record mapping test for a non-proposed lifecycle or unsupported target/capability.

## 6. Privacy And Redaction Assessment

The implementation remains reference-only and redaction-safe.

It does not copy or expose:

- raw provider payloads;
- generated comment bodies;
- pull request bodies or diffs;
- CI logs;
- command output;
- file contents;
- spec contents;
- environment values;
- credentials;
- authorization headers;
- token-like values.

`GitHubPullRequestCommentSideEffectAppendInput` implements redaction-safe `Debug`, and the returned executor input uses the existing redaction-safe `LocalExecutionSideEffectEventInput` `Debug` implementation. Focused tests assert the relevant non-leakage behavior.

## 7. Executor Boundary Assessment

The phase correctly stops before automatic event append.

The helper creates an input value that may be supplied to `LocalExecutionRequest.side_effect_events`; it does not mutate workflow state, append workflow events, emit audit records, write reports, or execute side effects on its own.

Existing `local_executor` tests already cover the generic explicit SideEffect proposed event append path, ordering before `SkillInvocationRequested`, and generic audit projection. However, this phase does not yet prove the GitHub-specific helper output through an end-to-end executor call. That is the right next focused implementation or test-hardening phase.

## 8. Test Quality Assessment

Focused tests cover:

- persisted proposed GitHub PR comment record loading into executor append input;
- returned target step/skill/version preservation;
- proposed lifecycle preservation;
- side-effect ID and reference preservation;
- missing record mapping without leakage;
- step mismatch without leakage;
- correlation mismatch without leakage;
- redaction-safe `Debug`;
- existing provider-write and local executor test suites.

Tests are meaningful and behavior-oriented, not just construction checks.

Shallow or missing coverage:

- No direct helper-level test for invalid-record mapping from non-proposed lifecycle or unsupported target/capability.
- No end-to-end executor test that supplies the GitHub-specific helper output into `LocalExecutionRequest.side_effect_events`.
- No dedicated assertion in this phase that generic audit projection observes the accepted event generated from the GitHub helper output, though the generic executor append path already has coverage.

These are non-blocking because the phase explicitly implemented the helper bridge, not automatic append or broader runtime execution.

## 9. Documentation Review

Documentation is honest about current behavior.

Docs state that the first bridge helper is implemented and that the following remain unimplemented:

- GitHub provider write calls;
- runtime write execution;
- live sandbox writes;
- automatic event append;
- attempted/completed/failed lifecycle behavior;
- report artifact writes;
- CLI write commands;
- workflow schema support;
- examples;
- hosted behavior;
- reasoning lineage;
- release posture changes.

The roadmap and GitHub integration docs preserve the fixture-first, no-provider-mutation posture.

## 10. Blockers

No blockers.

## 11. Non-Blocking Follow-Ups

- Add a helper-level invalid-record mapping regression for at least one non-proposed or unsupported persisted record.
- Add an end-to-end local executor test that persists a proposed GitHub PR comment record, loads it through the new helper, supplies it through `LocalExecutionRequest.side_effect_events`, and verifies `SideEffectProposed` appears before the targeted skill invocation.
- Verify generic audit projection on the event produced through the GitHub helper output.
- Keep automatic append after persistence deferred until the explicit helper path is reviewed and accepted.

## 12. Recommended Next Phase

Recommended next phase: GitHub PR comment SideEffect append helper executor-path proof.

That phase should remain local, fixture-first, opt-in, and reference-only. It should use the accepted helper output in the existing executor append path and verify event ordering plus audit projection. It must still not implement provider mutation, live sandbox writes, attempted/completed/failed lifecycle transitions, automatic append, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, or release posture changes.

## 13. Validation

Review validation:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test -p workflow-core --test local_executor`
- `cargo test -p workflow-core --test provider_write`
- `npm run check:docs`
- `git diff --check`

Dogfood governance:

- Workflow: `dg/review`.
- Run ID: `run-1783215218169372000-2`.
- Approval ID: `approval/run-1783215218169372000-2/review-scope-approved`.
- Approval outcome: granted by delegated maintainer.
- Kernel role: governance boundary and approval/event trail.
- Executor role: Codex performed repository review, documentation edit, validation, git, and PR actions outside the kernel.
