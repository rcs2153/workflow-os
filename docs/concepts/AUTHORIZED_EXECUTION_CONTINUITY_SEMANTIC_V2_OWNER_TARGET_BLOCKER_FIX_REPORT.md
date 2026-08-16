# Authorized Execution Continuity Semantic V2 Owner-Target Blocker Fix Report

## 1. Executive Summary

The remaining semantic V2 owner-to-target replay blocker is fixed in the
test-only reference continuity state. A committed successful operation now
proves that the authoritative window still owns the committed target when the
window remains at that exact revision. Legitimate later window revisions
remain valid replay successors.

This phase does not implement SQLite continuity storage, runtime scheduling,
executor redispatch, automatic approval, provider mutation, CLI behavior,
schemas, hosted execution, or nested harnesses.

## 2. Blocker Fixed

Focused review showed that exact replay of a committed yield registration
still succeeded after the owning window's `active_yield` was detached from the
committed generation. The yield, attempt, and operation records remained
individually well-shaped, but the authoritative ownership graph was broken.

The same class of corruption could affect the other successful operation
families if replay validated target rows without validating the owning
window's exact-revision relationship to them.

## 3. Implementation Approach

`validate_success_target` now enforces these exact-revision relationships:

- `YieldRegistered`: the window is yielded and `active_yield` names the
  committed generation;
- `WaitTransitioned`: the window remains yielded and `active_yield` names the
  generation that owns the committed wait;
- `DirectiveConsumed`: the window is executing and `active_yield` is clear;
- `RecordAttemptOutcome`: the closed window has no active yield; and
- `RecoverAmbiguousAttempt`: the recovery-required window has no active yield.

Yield, wait, and directive replay continue to permit a current window revision
greater than the committed revision. This preserves exact replay after lawful
successor operations instead of requiring mutable current state to remain
frozen forever. Outcome and recovery records retain their existing terminal
or recovery-state exactness.

## 4. Integrity Boundary

The correction validates the authoritative window-to-target link in addition
to the existing map-key, embedded-ID, cursor, subject, authority commitment,
wait membership, directive ownership, attempt ownership, and consume-operation
bindings. Corruption returns the existing stable
`authorized_execution_continuity_state.state.corrupt` code without exposing
identifiers or stored values.

The phase does not claim protection against arbitrary rollback of an entire
state snapshot. It does not add an external epoch anchor, durable database,
lease service, scheduler, or supervisor.

## 5. Test Coverage

The new operation-family corruption matrix commits and then corrupts each of
the five successful operation families:

- registered yield ownership detached;
- transitioned wait ownership detached;
- consumed directive given a stale active yield;
- recorded outcome given a stale active yield; and
- recovered ambiguous attempt given a stale active yield.

Every exact replay fails closed. Additional regressions prove that a committed
wait remains replayable after a lawful directive consumption and a committed
directive consumption remains replayable after a lawful attempt outcome.

The complete 25-test reference continuity suite and the 6-test public
continuity contract suite pass.

## 6. Privacy And Error Posture

The implementation adds no payload-bearing fields, reusable authority,
credentials, source content, command output, provider response, or secret-like
metadata. New validation uses existing bounded errors and does not echo the
corrupt target, window, operation, or identifier.

## 7. Governed Phase Record

- workflow: `dg/blocker`
- run: `run-1786842944168882000-2`
- approval: `approval/run-1786842944168882000-2/fix-approved`
- presentation: `presentation/ef5358506e0b0fa9`
- approval outcome: granted under delegated-maintainer authority after the
  complete proof-enforced handoff was evaluated
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- out-of-kernel work: source inspection, repository edits, tests, validation,
  documentation, and command execution were performed by the external
  executor under the governed scope

The kernel did not edit files, execute checks automatically, persist
continuity state, schedule or redispatch an executor, approve a user-facing
gate automatically, or mutate a provider.

## 8. Commands And Results

- the 25 focused reference continuity tests: passed;
- the 6 public continuity contract tests: passed;
- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed using an isolated target directory under
  `/private/tmp`;
- `npm run check`: passed;
- `npm run check:integrations`: passed;
- `npm run check:docs`: passed; and
- `git diff --check`: passed.

## 9. Remaining Known Limitations

- No production backend advertises semantic V2 continuity support.
- No durable continuity state survives restart.
- No runtime executor opens windows, consumes directives, or records yields.
- No supervisor interprets continuation disposition and redispatches lawful
  work after an executor turn boundary.
- No final-response guard prevents an external executor from terminating while
  the authoritative run remains non-terminal.
- No automatic approval or reusable delegated authority is introduced.
- SQLite implementation remains blocked pending focused review of this fix.

## 10. Recommended Next Phase

Perform a focused maintainer/security review of the owner-to-target correction
and its operation-family matrix. Proceed to SQLite semantic V2 implementation
only if that review accepts exact-revision ownership and lawful-successor
replay behavior.
