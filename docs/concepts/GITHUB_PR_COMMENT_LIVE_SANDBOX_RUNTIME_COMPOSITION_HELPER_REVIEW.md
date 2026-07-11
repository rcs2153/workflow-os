# GitHub PR Comment Live Sandbox Runtime Composition Helper Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The live sandbox runtime composition helper stayed within the approved narrow
runtime-composition scope. It adds an explicit, injected-provider helper that
delegates gate behavior to the accepted live sandbox validation boundary and
returns an in-memory result. It does not make provider writes automatic, does
not wrap `LocalExecutor`, does not load hidden auth, does not append workflow
events, does not write report artifacts, and does not expose CLI behavior.

## 2. Scope Verification

The phase stayed within the approved helper-only scope.

Implemented:

- `GitHubPrCommentLiveSandboxRuntimeCompositionRequest`;
- `GitHubPrCommentLiveSandboxRuntimeCompositionResult`;
- `GitHubPrCommentLiveSandboxRuntimeCompositionParts`;
- `compose_github_pr_comment_live_sandbox_runtime(...)`;
- focused success and readiness-blocking tests;
- roadmap, plan, and phase-report documentation updates.

No accidental implementation was found for:

- default provider writes;
- automatic executor writes;
- `LocalExecutor::execute(...)` behavior changes;
- local execution wrapping;
- hidden auth loading;
- auth loading from environment, keychain, GitHub CLI, git credentials,
  browser sessions, config files, OAuth state, or secret managers;
- CLI mutation commands;
- workflow event append;
- report artifact writes;
- artifact gate eligibility output;
- lookup, retry, repair, or recovery mutation;
- workflow schemas;
- SDK changes;
- examples;
- hosted or distributed runtime behavior;
- broad adapter write support;
- reasoning lineage;
- release posture changes.

## 3. Helper API Assessment

The API is appropriately small and explicit.

The request contains only the accepted
`GitHubPullRequestCommentLiveSandboxValidationInput`. The helper requires a
caller-supplied `SideEffectRecordStore`, an injected
`GitHubPullRequestCommentProvider`, and the explicit request. It returns a
bounded result or the existing non-leaking provider-call orchestration error.

This is the right shape for this phase because it composes an already-reviewed
validation boundary without inventing new runtime config, hidden state, or
ambient authorization.

## 4. Gate Assessment

Gate behavior is delegated to
`validate_and_orchestrate_github_pr_comment_live_sandbox(...)`, which preserves
the accepted order of operations.

Provider invocation remains blocked unless:

- sandbox target proof validates;
- readiness is `AllowedForSandbox`;
- caller-supplied auth posture is explicit;
- the attempted `SideEffect` lifecycle state is valid;
- provider-call input validates;
- idempotency binding matches the attempted record.

The readiness-blocking test proves that a denied readiness posture returns a
stable error, leaves provider invocation count at zero, and keeps the attempted
side-effect record in the attempted lifecycle state.

## 5. Runtime Boundary Assessment

The helper does not execute workflows, mutate `WorkflowRun`, append workflow
events, write report artifacts, emit CLI output, read hidden auth, or call
external systems except through the injected provider supplied by the caller.

The result explicitly reports:

- readiness decision;
- provider-call result;
- provider-call-performed posture;
- workflow-event-appended posture fixed to false;
- report-artifact-written posture fixed to false.

The helper can update side-effect lifecycle state through the accepted
provider-call orchestration path after gates pass. That behavior is expected
for this lane and is documented by the phase report and tests.

## 6. Evidence, Event, And Artifact Assessment

The helper does not fabricate event proof or artifact proof.

It does not append workflow events, does not produce report artifacts, and does
not return artifact eligibility. Those boundaries remain deferred to explicit
follow-on phases.

This is important because the helper is write-adjacent: a provider call may
occur only after explicit sandbox gates, but durable workflow event projection
and artifact referential integrity should remain separately reviewed before
they become part of the live sandbox path.

## 7. Privacy And Redaction Assessment

The redaction posture is acceptable.

The helper does not store or copy:

- provider auth tokens;
- authorization headers;
- raw provider payloads;
- raw pull request bodies;
- raw issue or review comments;
- repository file contents;
- CI logs;
- command output;
- raw spec contents;
- environment variable values;
- browser or session state;
- approval-presentation payload text;
- secret-like values.

Debug output is bounded. Focused tests verify that request/result/error Debug
paths do not leak token-like values, comment body text, provider comment
references, or side-effect record identifiers.

## 8. Failure Semantics Assessment

Failure behavior remains fail-closed and non-retrying.

Gate failures return the existing structured orchestration error and do not
invoke the injected provider. Provider-call failures remain classified by the
accepted provider-call orchestration layer. The helper does not retry, repair,
recover, fabricate IDs, fabricate workflow events, or write artifacts after a
failure.

Errors use stable codes and avoid leaking raw payloads, token-like values,
approval IDs, or record identifiers in the reviewed test paths.

## 9. Test Quality Assessment

The focused tests cover the most important phase risks:

- successful live sandbox runtime composition invokes the injected provider
  exactly once after explicit gates pass;
- successful provider response completes the side-effect lifecycle;
- readiness failure blocks provider invocation;
- readiness failure leaves attempted lifecycle state unchanged;
- no workflow event append;
- no report artifact write;
- Debug non-leakage for request, result, and error paths.

Existing provider-write and live-sandbox validation tests continue to provide
the deeper coverage for target proof, auth posture, provider-call validation,
classified provider failure, and idempotency behavior.

Non-blocking additional tests would be useful if this helper expands: one
composition-level target-proof failure test, one idempotency-mismatch test, and
one classified-provider-failure test. They are not blockers because those gates
are delegated to an accepted helper with existing coverage, and this phase did
not alter that logic.

## 10. Documentation Review

Documentation is honest about current capability.

The phase report, roadmap, and implementation plan state that the helper is
implemented and that default writes, automatic executor behavior, hidden auth,
CLI mutation commands, workflow event append, report artifact writes, schemas,
examples, hosted behavior, broad adapter writes, reasoning lineage, and release
posture changes remain unimplemented.

## 11. Blockers

No blockers.

## 12. Non-Blocking Follow-Ups

- Add composition-level regressions for target-proof failure, idempotency
  mismatch, and classified provider failure if this helper grows.
- Plan the smallest event-proof composition bridge before appending workflow
  events from live sandbox outcomes.
- Keep artifact eligibility and artifact writing in separate explicit phases.
- Keep real network tests opt-in and injected-provider based.

## 13. Recommended Next Phase

Recommended next phase: live sandbox event-proof composition planning.

The helper now proves that explicit sandbox readiness can reach an injected
provider without default writes or hidden auth. The next runtime-composition
gap is event proof: deciding when and how live sandbox outcomes may be
projected into durable workflow events without fabricating evidence, changing
executor semantics, or making provider writes automatic.

Do not build default writes, CLI mutation behavior, hidden auth loading,
schemas, examples, hosted behavior, broad adapters, automatic retry or repair,
reasoning lineage, or release posture changes.

## 14. Validation

Validation commands for this review:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run check:docs
git diff --check
```

Result: passed.

Dogfood review run:

- workflow: `dg/review`
- run: `run-1783782846308055000-2`
- approval: `approval/run-1783782846308055000-2/review-scope-approved`
- approval-presentation proof: `presentation/9b182dc5f73a21e2`
- outcome: approved and completed
