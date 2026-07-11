# GitHub PR Comment Live Sandbox Runtime Composition Plan

Status: Accepted in
[GitHub PR Comment Live Sandbox Runtime Composition Plan Review](../concepts/GITHUB_PR_COMMENT_LIVE_SANDBOX_RUNTIME_COMPOSITION_PLAN_REVIEW.md).
The next implementation should add the smallest explicit helper-level
composition path. Default provider writes, CLI mutation commands, hidden auth
loading, schemas, examples, hosted behavior, broad adapters, automatic retry or
repair, reasoning lineage, and release posture changes remain unimplemented.

## 1. Executive Summary

Workflow OS has an accepted GitHub pull request comment live sandbox validation
helper. That helper validates a bounded sandbox target proof, explicit
caller-supplied auth posture, sandbox readiness, provider-call input, and
attempted SideEffect lifecycle state before invoking an injected provider.

The next question is how that helper should compose with the existing explicit
provider-write runtime composition path. This plan defines that future bridge.

This plan is planning only. It does not implement provider mutation, run live
network calls, add hidden auth loading, add default executor writes, add CLI
mutation commands, add workflow schemas, update examples, add hosted behavior,
broaden adapters, add automatic recovery, implement reasoning lineage, or
change release posture.

## 2. Goals

- Compose the accepted live sandbox validation helper into an explicit runtime
  composition boundary.
- Preserve the default write-denied executor posture.
- Preserve explicit caller-supplied provider and auth inputs.
- Preserve sandbox target proof and readiness gates before provider transport.
- Reuse existing SideEffect lifecycle, provider reconciliation, workflow event
  proof, WorkReport disclosure, artifact gate, and recovery posture primitives.
- Make live sandbox composition inspectable through bounded gate posture.
- Keep provider-call failures, local transition failures, event-proof posture,
  and artifact posture distinct.
- Prepare a small implementation prompt without authorizing broader writes.

## 3. Non-Goals

Do not implement in the next phase unless separately approved:

- production provider writes;
- default provider writes;
- automatic executor provider writes;
- CLI mutation commands;
- hidden auth loading;
- auth loading from environment, keychain, GitHub CLI, git credentials,
  browser sessions, config files, OAuth state, or secret managers;
- automatic retries;
- automatic repair or recovery mutation;
- automatic report generation;
- automatic report artifact writing;
- workflow schema fields;
- SDK changes;
- examples;
- hosted or distributed runtime behavior;
- write-capable Jira, CI, filesystem, HTTP, or arbitrary adapters;
- reasoning lineage;
- recursive agents, agent swarms, or Level 3/4 autonomy;
- release posture changes.

## 4. Current Implemented Boundary

The live sandbox helper is implemented as:

- `GitHubPullRequestCommentLiveSandboxValidationInput`;
- `GitHubPullRequestCommentLiveSandboxValidationResult`;
- `validate_and_orchestrate_github_pr_comment_live_sandbox(...)`.

The explicit provider-write runtime composition helper is implemented as:

- `GitHubPrCommentProviderWriteRuntimeCompositionRequest`;
- `GitHubPrCommentProviderWriteRuntimeCompositionResult`;
- `compose_github_pr_comment_provider_write_runtime(...)`.

Artifact-gated provider-write composition is implemented separately as:

- `GitHubPrCommentProviderWriteArtifactGatedCompositionRequest`;
- `GitHubPrCommentProviderWriteArtifactGatedCompositionResult`;
- `compose_github_pr_comment_provider_write_with_artifact_gates(...)`.

These paths remain explicit, local, caller-supplied, and non-default.

## 5. Composition Problem

The live sandbox helper proves that one sandbox target can be validated and an
injected provider can be invoked only after strict sandbox gates pass.

The provider-write runtime composition helper proves that provider-write
execution can be wrapped with local executor execution, approval-presentation
proof, reconciliation, eligible event proof, and bounded WorkReport disclosure.

These two paths should not remain permanently parallel. A future runtime path
needs to say, explicitly:

- use live sandbox validation as the provider-call boundary;
- preserve all existing runtime composition gate clarity;
- preserve no-default-write posture;
- preserve artifact and recovery gates as separate explicit choices.

## 6. Recommended First Implementation Boundary

Add a new explicit helper-level composition path. Do not change
`LocalExecutor::execute(...)`.

Candidate helper:

```rust
compose_github_pr_comment_live_sandbox_runtime(...)
```

Candidate request/result names:

```rust
GitHubPrCommentLiveSandboxRuntimeCompositionRequest
GitHubPrCommentLiveSandboxRuntimeCompositionResult
```

The helper should accept:

- explicit local execution/report context where needed;
- a caller-supplied `SideEffectRecordStore`;
- an injected `GitHubPullRequestCommentProvider`;
- `GitHubPullRequestCommentLiveSandboxValidationInput`;
- approval-presentation gate policy and proof inputs where required;
- provider reconciliation/report disclosure inputs where required;
- event append policy where explicitly requested.

It should return:

- terminal run posture if local execution is part of the request;
- live sandbox validation result;
- bounded provider/local reconciliation posture;
- bounded event-proof posture;
- bounded report disclosure posture;
- explicit artifact-write eligibility posture, without writing artifacts;
- stable non-leaking error posture.

## 7. Gate Order

Recommended deterministic order:

1. Validate explicit runtime-composition request shape.
2. Execute or validate the local terminal run context through existing
   explicit paths.
3. Validate approval-presentation proof when configured.
4. Validate sandbox target proof.
5. Validate sandbox readiness.
6. Validate provider-call input and idempotency binding.
7. Invoke only the injected provider through
   `validate_and_orchestrate_github_pr_comment_live_sandbox(...)`.
8. Reconcile provider/local lifecycle posture through existing reconciliation
   helpers.
9. Append completed/failed SideEffect workflow events only when existing
   eligibility rules and caller policy allow it.
10. Build bounded WorkReport side-effect disclosure posture.
11. Return artifact gate eligibility only as posture unless the separately
    reviewed artifact-gated helper is explicitly used.
12. Return recovery posture for ambiguous outcomes without retrying or repair.

## 8. Failure Semantics

Before provider invocation, fail closed when:

- local execution context is invalid;
- approval-presentation proof is missing, stale, mismatched, or unresolved;
- sandbox target proof is missing, invalid, production-like, or ambiguous;
- sandbox readiness is denied or deferred;
- auth posture is hidden, ambient, unknown, missing, or mismatched;
- attempted SideEffect posture is missing or invalid;
- provider-call mode is unsupported;
- idempotency or correlation binding is invalid.

After provider invocation:

- do not retry automatically;
- do not repair provider/local ambiguity automatically;
- do not fabricate workflow event proof;
- do not write report artifacts automatically;
- preserve the workflow result and surface bounded composition posture.

## 9. Privacy And Redaction

The composition helper must not store or copy:

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
- browser/session state;
- approval-presentation payload text;
- secret-like values.

Errors must use stable codes and bounded messages. Debug output must be
redaction-safe.

## 10. Test Plan

Future implementation tests should cover:

- all gates satisfied invokes the injected provider exactly once;
- approval-presentation proof failure blocks provider invocation;
- sandbox target proof failure blocks provider invocation;
- readiness denied/deferred blocks provider invocation;
- hidden/ambient auth posture blocks provider invocation;
- provider success returns completed lifecycle posture;
- provider classified failure returns failed lifecycle posture;
- provider/local ambiguity blocks retry and returns recovery posture;
- event append occurs only when eligibility rules pass;
- report disclosure remains bounded and non-leaking;
- artifact writing is not performed by the runtime-composition helper;
- default `LocalExecutor::execute(...)` remains unchanged;
- no CLI output or filesystem artifacts are created;
- Debug output does not leak tokens, target strings, comment bodies, or
  approval-presentation content;
- existing provider-write, SideEffect, WorkReport, approval, and runtime tests
  still pass.

## 11. Documentation Updates For Future Implementation

Future implementation must update:

- this plan;
- [GitHub PR Comment Live Sandbox Validation Plan](github-pr-comment-live-sandbox-validation-plan.md);
- [Provider-Write Runtime Composition Plan](provider-write-runtime-composition-plan.md);
- [Roadmap](../../ROADMAP.md);
- an end-of-phase report under `docs/concepts/`.

Docs must state that the path is explicit, local, injected, and non-default.
They must also state that default writes, CLI mutation commands, hidden auth
loading, schemas, examples, hosted behavior, broad adapters, automatic
recovery, and release posture changes remain unimplemented.

## 12. Open Questions

- Should the first helper wrap local execution or accept an already-terminal
  provider-write runtime context?
- Should event append be included in the first live sandbox composition helper
  or remain a follow-on explicit input?
- Should artifact gate eligibility be returned as posture only, or should the
  artifact-gated helper be composed in the same future implementation?
- What is the smallest useful live sandbox test that proves the bridge without
  making network mutation feel like normal product behavior?
- Should any future live sandbox integration test be ignored by default and
  gated by explicit local test inputs?

## 13. Final Recommendation

Proceed next with a narrow explicit helper implementation that composes the
accepted live sandbox validation helper into the existing provider-write
runtime composition posture.

Do not build default provider writes, CLI mutation commands, hidden auth
loading, schemas, examples, hosted behavior, broad adapters, automatic retry or
repair, reasoning lineage, or release posture changes.
