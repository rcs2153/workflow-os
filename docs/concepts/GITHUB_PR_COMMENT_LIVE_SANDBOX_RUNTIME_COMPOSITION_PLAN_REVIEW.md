# GitHub PR Comment Live Sandbox Runtime Composition Plan Review

## 1. Executive Verdict

Plan accepted; proceed to live sandbox runtime composition helper
implementation.

The plan defines the right bridge between the accepted live sandbox validation
helper and the existing explicit provider-write runtime composition path. It
keeps the future implementation helper-level, local, caller-supplied, injected,
and non-default. It does not authorize default writes, CLI mutation behavior,
hidden auth loading, schemas, examples, hosted behavior, broad adapters,
automatic recovery, reasoning lineage, or release posture changes.

## 2. Scope Verification

The planning phase stayed within planning-only scope.

It added:

- [GitHub PR Comment Live Sandbox Runtime Composition Plan](../implementation-plans/github-pr-comment-live-sandbox-runtime-composition-plan.md);
- [GitHub PR Comment Live Sandbox Runtime Composition Plan Report](GITHUB_PR_COMMENT_LIVE_SANDBOX_RUNTIME_COMPOSITION_PLAN_REPORT.md);
- links from the roadmap, live sandbox validation plan, and provider-write
  runtime composition plan.

It did not implement or authorize:

- provider mutation;
- live network calls;
- default executor writes;
- CLI mutation commands;
- hidden auth loading;
- workflow schema fields;
- SDK changes;
- examples;
- hosted or distributed runtime behavior;
- broad provider writes;
- automatic retry;
- automatic repair or recovery mutation;
- automatic report generation;
- automatic report artifact writing;
- reasoning lineage;
- release posture changes.

## 3. Boundary Assessment

The plan correctly identifies the composition gap.

The live sandbox validation helper already validates sandbox target proof,
sandbox readiness, explicit caller-supplied auth posture, provider-call input,
and attempted SideEffect lifecycle state before invoking an injected provider.

The provider-write runtime composition helper already wraps explicit
provider-write execution with local executor execution, approval-presentation
proof, reconciliation posture, eligible event proof, and bounded WorkReport
disclosure.

The plan does not collapse those boundaries into default behavior. It proposes
a new explicit helper-level bridge that reuses both paths while keeping
`LocalExecutor::execute(...)` unchanged.

## 4. API And Integration Assessment

The candidate API shape is appropriately narrow:

- `compose_github_pr_comment_live_sandbox_runtime(...)`;
- `GitHubPrCommentLiveSandboxRuntimeCompositionRequest`;
- `GitHubPrCommentLiveSandboxRuntimeCompositionResult`.

The future request should accept explicit context, a caller-supplied
`SideEffectRecordStore`, an injected `GitHubPullRequestCommentProvider`,
`GitHubPullRequestCommentLiveSandboxValidationInput`, approval-presentation
policy/proof inputs, report disclosure inputs, and optional event append policy.

That shape is compatible with the existing codebase because it composes
existing helper inputs rather than inventing new provider-write vocabulary.

## 5. Gate Order Assessment

The proposed gate order is conservative and correct.

Provider transport remains blocked until:

- runtime composition request shape is valid;
- local terminal context is valid where applicable;
- approval-presentation proof passes when configured;
- sandbox target proof validates;
- sandbox readiness allows the request;
- provider-call input and idempotency binding validate.

Only then should the future helper invoke
`validate_and_orchestrate_github_pr_comment_live_sandbox(...)`.

This preserves the important safety property proven by the existing live
sandbox tests: provider invocation count stays zero when local proof/readiness
gates fail.

## 6. Failure Semantics Assessment

The plan keeps failure behavior fail-closed and non-retrying.

Before provider invocation, missing or invalid local execution context,
approval-presentation proof, target proof, readiness, auth posture, attempted
SideEffect posture, provider-call mode, idempotency, or correlation binding
must block transport.

After provider invocation, the plan correctly requires:

- no automatic retry;
- no automatic repair;
- no fabricated workflow event proof;
- no automatic artifact write;
- bounded provider/local reconciliation posture.

This is the right posture for a write-adjacent bridge.

## 7. Artifact And Recovery Assessment

The plan correctly keeps artifact writing and recovery automation separate.

The future helper may return artifact-write eligibility posture, but it should
not write artifacts. Artifact writing belongs in the separately reviewed
artifact-gated composition path.

The future helper may return recovery posture for ambiguous provider/local
outcomes, but it must not perform lookup, retry, repair, or recovery mutation
unless a later plan explicitly scopes that behavior.

## 8. Privacy And Redaction Assessment

The plan preserves the existing redaction posture.

The future helper must not store or copy:

- provider auth tokens;
- authorization headers;
- raw provider payloads;
- raw pull request bodies;
- raw issue or review comments;
- repository file contents;
- CI logs;
- command output;
- raw spec contents;
- environment values;
- browser/session state;
- approval-presentation payload text;
- secret-like values.

Errors and Debug output must remain stable, bounded, and non-leaking.

## 9. Test Plan Assessment

The proposed test plan covers the right risks:

- successful path invokes the injected provider exactly once;
- approval-presentation failure blocks provider invocation;
- target proof failure blocks provider invocation;
- readiness denial/defer blocks provider invocation;
- hidden/ambient auth posture blocks provider invocation;
- provider success and classified failure map to lifecycle posture;
- provider/local ambiguity blocks retry and returns recovery posture;
- event append occurs only when existing eligibility rules pass;
- report disclosure remains bounded;
- artifact writing is not performed by this helper;
- default executor behavior remains unchanged;
- Debug output does not leak sensitive values.

The implementation phase should prefer focused helper-level tests plus existing
provider-write/runtime regression coverage.

## 10. Documentation Review

The plan and report are honest about current capability.

They state that live sandbox runtime composition is planned, not implemented,
and that default writes, CLI mutation commands, hidden auth loading, schemas,
examples, hosted behavior, broad adapters, automatic recovery, reasoning
lineage, and release posture changes remain unimplemented.

## 11. Blockers

No planning blockers.

## 12. Non-Blocking Follow-Ups

- In the implementation phase, keep event append optional or explicitly
  policy-driven; do not silently append events from the new helper.
- Keep artifact writing out of the first live sandbox runtime composition
  helper unless the implementation prompt explicitly scopes artifact-gated
  composition.
- Add a focused regression proving `LocalExecutor::execute(...)` remains
  unchanged.
- Keep any real network integration test ignored by default and driven only by
  explicit local test inputs.

## 13. Recommended Next Phase

Recommended next phase: live sandbox runtime composition helper implementation.

The implementation should add the smallest explicit helper that composes the
accepted live sandbox validation helper with the existing provider-write
runtime composition posture. It must remain local, injected, caller-supplied,
non-default, and non-CLI.

Do not build default provider writes, CLI mutation commands, hidden auth
loading, schemas, examples, hosted behavior, broad adapters, automatic retry or
repair, reasoning lineage, or release posture changes.

## 14. Validation

Validation commands for this review:

```sh
npm run check:docs
git diff --check
```

Result: passed.

Dogfood review run:

- workflow: `dg/review`
- run ID: `run-1783779165667205000-2`
- approval ID: `approval/run-1783779165667205000-2/review-scope-approved`
- presentation ID: `presentation/6e8abd135b55ef40`
- approval outcome: delegated maintainer approved
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval-presentation enforcement: proof-enforced, with proof marker present
  on the approval event
