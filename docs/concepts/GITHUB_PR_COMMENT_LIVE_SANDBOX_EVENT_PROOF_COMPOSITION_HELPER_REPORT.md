# GitHub PR Comment Live Sandbox Event-Proof Composition Helper Report

## 1. Executive Summary

The GitHub PR comment live sandbox event-proof composition helper is
implemented as a narrow, explicit, in-memory helper.

The helper consumes an already-composed live sandbox runtime result and a
caller-supplied terminal `WorkflowRun`, then appends durable completed/failed
`SideEffect` workflow event proof through the existing local event append path
when the caller explicitly allows append.

This phase does not make provider writes automatic. It does not call the
provider again, load hidden auth, write report artifacts, expose CLI behavior,
add schemas or examples, broaden adapters, implement retry/repair, implement
reasoning lineage, or change release posture.

## 2. Scope Completed

- Added `GitHubPrCommentLiveSandboxEventProofAppendPolicy`.
- Added `GitHubPrCommentLiveSandboxEventProofStatus`.
- Added `GitHubPrCommentLiveSandboxEventProofCompositionRequest`.
- Added `GitHubPrCommentLiveSandboxEventProofCompositionResult`.
- Added `compose_github_pr_comment_live_sandbox_event_proof(...)`.
- Exported the helper API from `workflow-core`.
- Added focused Rust tests for completed proof, failed proof, disabled policy,
  idempotent already-present proof, non-terminal blocking, and redaction-safe
  Debug/error behavior.
- Updated roadmap and implementation planning docs.

## 3. Scope Explicitly Not Completed

- No default provider writes.
- No automatic executor provider writes.
- No provider call from the event-proof helper.
- No hidden auth loading.
- No environment, keychain, GitHub CLI, git credential, browser session,
  config-file, OAuth, or secret-manager loading.
- No CLI mutation command.
- No report artifact write.
- No persistence beyond the existing workflow event append path when explicitly
  invoked.
- No workflow schema fields.
- No SDK changes.
- No examples.
- No hosted or distributed runtime behavior.
- No automatic retry, repair, lookup, or recovery mutation.
- No write-capable Jira, CI, filesystem, HTTP, or arbitrary adapters.
- No reasoning lineage.
- No recursive agents, agent swarms, or Level 3/4 autonomy.
- No release posture change.

## 4. Helper API Summary

The implemented helper is:

```rust
compose_github_pr_comment_live_sandbox_event_proof(...)
```

It accepts:

- an existing `GitHubPrCommentLiveSandboxRuntimeCompositionResult`;
- a caller-supplied `WorkflowRun`;
- explicit append policy;
- explicit idempotency key;
- explicit correlation ID;
- explicit actor.

It returns:

- the original live sandbox runtime composition result;
- the run after attempted event-proof projection;
- bounded event-proof status;
- optional stable non-leaking event append error.

## 5. Event-Proof Behavior

The helper appends durable workflow event proof only when:

- append policy is `AppendIfMissing`;
- the supplied run is terminal;
- the provider outcome is `ProviderSucceeded` or `ProviderFailed`;
- the store-backed transition lifecycle is `Completed` or `Failed`;
- the side-effect transition identity matches the workflow run identity;
- step and skill identity are present;
- the caller-supplied idempotency key is not already present.

Before append, the helper rehydrates the authoritative run from the supplied
executor backend. A matching key bound to the same SideEffect outcome returns
`AlreadyPresent`; a reused key or a second key for an existing outcome returns
`Conflict` without appending another event.

`ProviderSucceeded` maps to `SideEffectCompleted`.
`ProviderFailed` maps to `SideEffectFailed`.

Fixture and dry-run validated outcomes are not eligible for completed/failed
event proof.

## 6. Status Model

The helper exposes bounded status values:

- `NotRequested`;
- `NotEligible`;
- `Blocked`;
- `Appended`;
- `AlreadyPresent`;
- `Conflict`;
- `Failed`.

This keeps missing proof visible to future report/artifact gates instead of
silently treating a provider response as durable workflow proof.

## 7. Workflow Semantics Summary

The helper does not execute workflows and does not alter
`LocalExecutor::execute(...)`.

It appends only the explicit event proof requested by the caller, using the
existing local event append path. It does not append provider calls, approvals,
audit events, report artifacts, or CLI output.

## 8. Redaction And Privacy Summary

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
- browser/session state;
- approval-presentation payload text;
- secret-like values.

Debug output redacts run identity, idempotency key, correlation ID, actor, and
target details. Errors use stable codes and bounded messages.

## 9. Test Coverage Summary

Focused tests cover:

- provider success appends a completed `SideEffect` workflow event;
- provider failure appends a failed `SideEffect` workflow event;
- disabled append policy returns `NotRequested`;
- duplicate idempotency returns `AlreadyPresent`;
- stale caller run input is rehydrated before proof inspection;
- a different idempotency key for an existing SideEffect outcome returns
  `Conflict` without appending a duplicate event;
- non-terminal runs return `Blocked`;
- Debug/error output does not leak provider token, comment body, target, or
  side-effect IDs;
- no report artifact is written by the helper.

Existing live sandbox, provider-write, SideEffect, WorkReport, approval, and
runtime tests remain covered by the required workspace validation.

## 10. Commands Run And Results

- `cargo test -p workflow-core --test local_executor live_sandbox_event_proof`
  - Passed: 5 tests.
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

## 11. Governed Phase Evidence

- Dogfood workflow: `dg/implement`.
- Run ID: `run-1783785482954932000-2`.
- Approval ID:
  `approval/run-1783785482954932000-2/implementation-approved`.
- Approval outcome: granted through proof-enforced approval presentation.
- Approval presentation ID: `presentation/8be6d247f386c205`.
- Event summary: 39 ordered events, including one approval request, one approval
  grant, eight policy decisions, six scheduled steps, six successful skill
  invocations, and one completed run; no retries or escalations.
- Validation summary: focused tests and all required repository validation
  commands passed.
- Out-of-kernel work: Codex performed the Rust and documentation edits, test
  execution, formatting, diff review, and git/PR operations. The kernel governed
  phase scope and approval but did not execute shell commands, edit files, call
  providers, write report artifacts, or perform git/PR actions.
- Report posture: this phase report is documentation; no runtime `WorkReport`
  artifact was generated or persisted.

## 12. Remaining Known Limitations

- The helper is explicit and in-memory; no automatic runtime event-proof path
  exists.
- Report artifact gates are not invoked by this helper.
- Lookup/recovery remains deferred.
- Live sandbox provider transport remains caller-injected and opt-in.
- Broader provider support remains deferred.

## 13. Recommended Next Phase

Recommended next phase: GitHub PR comment live sandbox event-proof composition
helper review.

This helper is write-adjacent and security-sensitive enough to require a
maintainer review before any further runtime composition, artifact, recovery,
or provider-write expansion.
