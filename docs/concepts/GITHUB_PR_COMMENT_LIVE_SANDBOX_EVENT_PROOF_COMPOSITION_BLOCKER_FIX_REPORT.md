# GitHub PR Comment Live Sandbox Event-Proof Composition Blocker Fix Report

## 1. Executive Summary

The live sandbox event-proof identity blockers are fixed.

The helper now derives one canonical event idempotency key from accepted
SideEffect/provider outcome identity. Callers can no longer select the durable
proof-event key. The helper also requires the explicit event correlation ID to
match both the accepted SideEffect record and transition event before append.

The phase remains a narrow, explicit post-provider event-proof helper. It does
not call providers, enable default writes, write artifacts, expose CLI
behavior, add schemas or examples, retry or repair, broaden adapters, add
hosted behavior, implement reasoning lineage, or change release posture.

## 2. Blockers Fixed

### Deterministic event identity

Before the fix, the first event key was caller-selected. Sequential conflict
checks prevented ordinary replay duplicates, but distinct concurrent callers
could choose different keys before either append became visible.

The request no longer contains an event idempotency-key field. The helper
derives a canonical SHA-256-backed key from:

- SideEffect ID;
- original provider-write idempotency key;
- GitHub PR comment provider kind;
- SideEffect target kind;
- classified provider outcome;
- completed/failed lifecycle state.

Equivalent callers therefore reach the backend with the same event key, so the
existing exact-key idempotency boundary can prevent duplicate durable proof.

### Correlation identity

Before the fix, the workflow event envelope could use a caller correlation ID
that disagreed with the accepted transition.

The helper now requires the supplied correlation ID to match both the
store-backed SideEffect record and the bounded transition event payload. A
mismatch returns `Conflict` with stable
`github_pr_comment_live_sandbox_event_proof.identity_mismatch` posture before
event append.

## 3. API Change

`GitHubPrCommentLiveSandboxEventProofCompositionRequest` retains:

- accepted live-sandbox runtime result;
- caller-supplied run context;
- explicit append policy;
- expected correlation ID;
- event actor.

It no longer accepts a caller-selected event idempotency key. This API was
introduced as an experimental, unreviewed helper in the immediately preceding
phase and was corrected before further runtime composition.

## 4. Runtime Semantics

The helper still:

- rehydrates authoritative run state before eligibility and replay checks;
- accepts only terminal runs;
- maps provider success to `SideEffectCompleted`;
- maps classified provider failure to `SideEffectFailed`;
- rejects fixture and dry-run outcomes;
- appends through the existing local event path only when explicitly enabled;
- returns `AlreadyPresent` for matching replay;
- appends no provider call, artifact, CLI output, retry, or repair behavior.

The deterministic key is derived before existing-event inspection and append.
Matching replay derives the same key and returns `AlreadyPresent` after
authoritative rehydration.

## 5. Privacy And Redaction

The fix does not persist or copy provider auth, authorization headers, raw
provider payloads, comment bodies, repository contents, CI logs, command
output, spec contents, environment values, browser/session state,
approval-presentation text, or secret-like values.

The derived key is hashed and remains redacted from Debug output. Correlation
values and SideEffect identifiers remain redacted. Errors continue to use
stable codes and bounded messages.

## 6. Test Coverage

Focused regressions cover:

- completed outcome event append;
- failed outcome event append;
- disabled append policy;
- canonical key prefix and stable matching replay;
- one completed proof event after replay;
- explicit correlation mismatch rejection before append;
- non-terminal blocking;
- Debug and error non-leakage;
- no report artifact write;
- no provider recall.

Existing event runtime tests continue to prove exact-key idempotency and that
completed/failed SideEffect outcome events are the only lifecycle events
allowed after terminal run state.

## 7. Validation

Required validation:

```sh
cargo test -p workflow-core --test local_executor live_sandbox_event_proof
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run check:docs
git diff --check
```

Results:

- Focused event-proof regressions: passed, 6 tests.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 8. Governed Phase Evidence

- Dogfood workflow: `dg/blocker`.
- Run ID: `run-1783791232276986000-2`.
- Approval ID: `approval/run-1783791232276986000-2/fix-approved`.
- Approval outcome: granted through proof-enforced approval presentation.
- Approval presentation ID: `presentation/5f8a6a6ab5f8112b`.
- Event summary: 39 ordered events, including one approval request, one approval
  grant, eight policy decisions, six scheduled steps, six successful skill
  invocations, and one completed run; no retries or escalations.
- Validation summary: all focused and required repository validation commands
  passed.
- Out-of-kernel work: Codex performed source edits, tests, documentation,
  validation, and git/PR operations. The kernel governed scope and approval but
  did not execute shell commands, edit files, call providers, write artifacts,
  or perform git/PR actions.
- Report posture: this document is the phase report; no runtime `WorkReport`
  artifact is generated or persisted.

## 9. Remaining Known Limitations

- The helper remains explicit and is not invoked automatically by ordinary
  executor paths.
- Append-success/rehydration-failure remains conservatively reported as
  `Failed` without explicit ambiguous-outcome vocabulary.
- Report artifact gates remain a separate helper path.
- Lookup and recovery remain explicit and deferred.
- Live sandbox transport remains caller-injected and opt-in.
- Broader provider support remains deferred.

## 10. Recommended Next Phase

Recommended next phase: live sandbox event-proof composition blocker-fix
review.

The review should verify canonical key derivation, correlation binding,
matching replay, non-leakage, and unchanged provider/artifact boundaries before
any further runtime composition.
