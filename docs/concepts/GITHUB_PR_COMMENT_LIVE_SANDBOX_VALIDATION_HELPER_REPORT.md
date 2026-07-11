# GitHub PR Comment Live Sandbox Validation Helper Report

## 1. Executive Summary

This phase implements the first explicit, injected live sandbox validation
helper for the GitHub pull request comment provider-write lane.

The helper composes existing reviewed primitives:

- sandbox target proof;
- sandbox readiness;
- caller-supplied provider-call input;
- injected provider boundary;
- store-backed attempted-to-completed or attempted-to-failed SideEffect
  transition.

The helper is non-default and local. It does not make provider writes automatic,
does not add CLI mutation behavior, does not load hidden auth, does not append
workflow events, does not write report artifacts, and does not change executor
defaults.

## 2. Scope Completed

Completed scope:

- added `GitHubPullRequestCommentLiveSandboxValidationInput`;
- added `GitHubPullRequestCommentLiveSandboxValidationResult`;
- added `validate_and_orchestrate_github_pr_comment_live_sandbox`;
- validated target proof before provider transport;
- validated target proof/readiness/provider-call consistency before provider
  transport;
- required `AllowedForSandbox` readiness before provider transport;
- reused existing `GitHubPullRequestCommentProviderCallRequest` construction;
- reused existing injected provider-call orchestration;
- preserved store-backed SideEffect lifecycle transitions from classified
  provider success or failure responses;
- added focused regression tests;
- updated roadmap and planning documentation.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- production writes;
- default provider writes;
- automatic executor writes;
- hidden auth loading from environment variables, keychains, GitHub CLI, git
  credentials, browser sessions, config files, OAuth state, or secret managers;
- CLI mutation commands;
- workflow schema fields;
- example updates;
- hosted or distributed runtime behavior;
- broad GitHub mutations beyond the explicit injected PR comment lane;
- Jira, CI, filesystem, HTTP, or arbitrary provider writes;
- automatic retries, repair, or recovery mutation;
- workflow event append;
- report artifact writes;
- reasoning lineage;
- release posture changes.

## 4. Helper API Summary

The new helper is:

```rust
validate_and_orchestrate_github_pr_comment_live_sandbox(
    store,
    provider,
    input,
)
```

It accepts:

- a caller-supplied `SideEffectRecordStore`;
- a caller-supplied injected `GitHubPullRequestCommentProvider`;
- explicit `GitHubPullRequestCommentLiveSandboxValidationInput`.

The input contains:

- `ProviderWriteSandboxTargetProof`;
- `ProviderWriteSandboxReadinessInput`;
- `GitHubPullRequestCommentProviderCallInput`;
- transition timestamp;
- stable transition references;
- evidence reference count.

The result contains:

- the readiness result that allowed the sandbox call;
- the provider-call orchestration result.

## 5. Gate Sequence

The helper fails before provider transport unless all local gates pass:

1. target proof validates;
2. target proof target matches provider-call target;
3. target proof capability matches readiness capability;
4. target proof adapter target matches readiness target;
5. target proof posture matches readiness posture;
6. target proof idempotency matches provider-call idempotency;
7. readiness auth posture is explicit caller-supplied;
8. provider-call mode is `LiveSandbox`;
9. sandbox readiness decision is `AllowedForSandbox`;
10. existing provider-call request validation passes.

Only after those gates pass does the helper invoke the injected provider.

## 6. Provider Boundary Summary

The helper calls only the caller-supplied
`GitHubPullRequestCommentProvider`. It does not construct a network client, load
auth, discover credentials, read environment variables, read browser state, or
use global configuration.

Provider responses still use the existing bounded response model. Classified
provider success transitions the attempted SideEffect to completed. Classified
provider failure transitions it to failed. Unclassified provider errors remain
bounded orchestration errors.

## 7. Workflow Semantics Summary

The helper does not:

- mutate `WorkflowRun`;
- append workflow events;
- emit audit records;
- write report artifacts;
- write files;
- expose CLI output;
- change default executor behavior.

Any event append, audit projection, report artifact write, executor integration,
or CLI exposure remains a separately reviewed boundary.

## 8. Redaction And Privacy Summary

The helper preserves existing provider-write privacy posture:

- no raw provider payloads are stored;
- no raw PR bodies or issue comments are stored;
- no repository file contents are copied;
- no CI logs or command output are copied;
- no parser payloads are copied;
- no environment variable values are read or copied;
- no credentials, authorization headers, private keys, token-like values, or
  browser/session state are stored;
- Debug output redacts caller-supplied payload values;
- errors use stable codes and do not include raw target, comment, auth, or
  proof statement values.

## 9. Test Coverage Summary

Focused tests cover:

- successful live sandbox validation invokes the injected provider once;
- successful provider response transitions attempted SideEffect to completed;
- target-proof mismatch prevents provider invocation;
- denied readiness prevents provider invocation;
- hidden/ambient auth posture prevents provider invocation;
- Debug output does not leak comment body, auth, target-proof statement,
  correlation ID, or idempotency key.

Existing provider-write tests continue to cover provider-call request
validation, injected provider orchestration, concrete injected HTTP provider
auth matching, SideEffect lifecycle transitions, reconciliation, lookup, and
recovery helpers.

## 10. Commands Run And Results

Commands run during implementation and validation:

- `cargo fmt --all`
- `cargo test -p workflow-core --test provider_write live_sandbox_validation -- --nocapture`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`

All commands passed.

## 11. Dogfood Governance

Dogfood implementation phase:

- workflow: `dg/implement`;
- run ID: `run-1783762515773138000-2`;
- approval ID: `approval/run-1783762515773138000-2/implementation-approved`;
- presentation ID: `presentation/d09ab631a890d02e`;
- approval outcome: granted;
- approved scope: add non-default injected helper, tests, docs, report,
  validation, commit, PR, and merge after checks.

## 12. Remaining Known Limitations

Known limitations:

- no real live network test is run by default;
- no CLI mutation surface exists;
- no automatic executor provider-write path exists;
- no hidden auth source is supported;
- no production target validation is performed through provider lookup;
- no workflow event append occurs;
- no report artifact write occurs;
- no automatic recovery or retry mutation occurs.

## 13. Recommended Next Phase

Recommended next phase: live sandbox validation helper review.

The helper is write-adjacent and security-sensitive. It should be reviewed
before any executor-facing, CLI-facing, or real-network sandbox validation path
is considered.

Fix-forward note: the helper review is documented in
[GitHub PR Comment Live Sandbox Validation Helper Review](GITHUB_PR_COMMENT_LIVE_SANDBOX_VALIDATION_HELPER_REVIEW.md).
