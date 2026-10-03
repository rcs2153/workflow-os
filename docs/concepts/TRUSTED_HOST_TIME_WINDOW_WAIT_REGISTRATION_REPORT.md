# Trusted Host TimeWindow Wait Registration Report

## 1. Executive Summary

Workflow OS now lets the exact crate-private trusted-host executor return zero
or one bounded `TimeWindow` wait declaration. Core derives the trusted-time
dependency binding from canonical SQLite state and uses the existing
continuity transaction to register the attempt yield and durable wait
atomically.

This does not add a scheduler, timer service, opaque host handoff, automatic
reinvocation, provider execution, sandbox integration, or public runtime
surface.

## 2. Scope Completed

- Replaced the supervisor's bare yielded reason with a private bounded yield
  request.
- Preserved the existing empty-wait turn-boundary path.
- Added one private `TimeWindow` declaration constructor accepting condition
  identity, positive version, deadline, and yield reason.
- Derived wake-trigger class, trusted-time source, provenance commitment,
  epoch, and domain-separated dependency commitment inside Core.
- Reused the exact authorized attempt capability for run, window, attempt,
  cursor, authority, and immutable binding.
- Passed the derived wait seed to the accepted atomic yield-registration
  transaction.
- Returned `AwaitCondition` only from committed authoritative state.

## 3. Scope Explicitly Not Completed

- No opaque wait handoff or host-facing wait token.
- No scheduler, loop, sleep, poller, timer service, queue, or wake bus.
- No automatic executor reinvocation or conversational recirculation.
- No approval, evidence, check, external-event, capability, authority-refresh,
  or conflict wait declarations.
- No provider execution or mutation, OpenShell, or sandbox integration.
- No nested harness runtime, recursive agents, or agent swarms.
- No public API, workflow schema, CLI, SDK, UI, example, hosted, or distributed
  surface.
- No persistence schema or release-posture change.

## 4. Private Model And API Summary

`TrustedHostYieldRequest` is crate-private. `without_wait(reason)` preserves
the ordinary zero-wait path for every existing yield reason.
`with_time_window(...)` accepts one typed declaration but cannot provide
workflow, run, step, window, attempt, generation, cursor, authority,
governance, trusted-time source, provenance, epoch, wake trigger, or dependency
commitment.

`TrustedHostAttemptExecutionResult::Yielded` now carries that private request.
No public executor, handler, CLI, or workflow-spec contract changed.

## 5. Atomic Registration Boundary

The supervisor validates the condition version and asks the SQLite backend to
derive a binding from a canonically decoded continuity snapshot. The backend
requires live eligible trusted-time state, an executing window, and a deadline
after the trusted watermark but no later than the authorized window expiry.

The existing `register_yield` transaction remains authoritative. It obtains
fresh trusted time, revalidates the exact attempt capability and window
binding, recomputes the dependency commitment, rejects stale or invalid
deadlines, and writes the yield, wait, directive, attempt state, window state,
receipt, and event projection atomically.

## 6. Authority And Governance Binding

The executor declaration is a liveness request, not authority. Core continues
to derive or validate:

- workflow, run, step, actor, and immutable run bundle;
- window and attempt identities and revisions;
- continuity cursor and yield generation;
- governance and authority commitments;
- dispatch reservation and exact executor/invocation commitments; and
- trusted-time source, provenance, epoch, and deadline dependency commitment.

The declaration cannot satisfy its own wait or create a wake capability.

## 7. Failure And Replay Posture

- Condition version zero fails with a stable validation code.
- Missing generation identity retains the existing fail-closed behavior.
- A deadline outside the authorized window fails with a stable security code.
- No invalid declaration persists a yield or wait.
- A state change between binding derivation and registration is rechecked in
  the transaction and fails closed.
- Existing register-yield operation replay and reconciliation semantics remain
  unchanged.

## 8. Privacy And Redaction

The private yield request Debug implementation exposes only yield reason,
wait count, and a redacted binding marker. Errors omit condition values,
deadlines, timestamps, paths, source provenance, epochs, commands, prompts,
credentials, tokens, and provider payloads. Persistence contains only the
already-accepted bounded identifiers, timestamps, and commitments.

## 9. Test Coverage

Focused tests prove:

- existing zero-wait turn-boundary behavior remains valid;
- one executor-declared `TimeWindow` wait is registered atomically;
- authoritative continuation becomes `AwaitCondition`;
- the stored wait is bound to the exact generation and canonical trusted-time
  source, provenance, epoch, and deadline;
- condition version zero fails without a partial yield or wait;
- a deadline outside the authorized window fails without a partial yield or
  wait; and
- Debug and errors do not expose declaration values.

Existing operational-entry, dispatch, supervisor, redispatch, continuity,
SQLite, runtime, adapter, and report tests remain part of workspace validation.

## 10. Commands Run And Results

- `cargo test -p workflow-core --lib operational_opening_store --no-fail-fast`:
  passed, 36 tests.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 11. Remaining Known Limitations

- Only `TimeWindow` is accepted and only one declaration may be returned.
- No opaque wait handoff exists for a host to orient around current waits.
- No source observes or schedules the deadline automatically.
- The existing private caller must still be invoked explicitly after the
  deadline to assess and transition the wait.
- SQLite remains local and non-tamper-resistant against a privileged local
  actor.

## 12. Recommended Next Phase

Perform a focused maintainer/security review of the private `TimeWindow`
registration boundary. Verify declaration bounds, Core-derived authority and
trusted-time fields, atomicity, race handling, replay posture, privacy, and the
absence of scheduling or public runtime behavior.

Only after acceptance should Workflow OS add an opaque, non-authoritative wait
handoff. Do not begin scheduling, provider, sandbox, nested-harness, public
configuration, or additional wake-source work first.

## 13. Governed Phase Record

- workflow: `dg/runtime-composition`
- run: `run-1791051995351493000-2`
- approval: `approval/run-1791051995351493000-2/composition-approved`
- presentation: `presentation/8377fcc7447d6119`
- presentation hash:
  `8377fcc7447d6119bddfd1a1b6894b80368e7d997a3285d9e0986545a45fbb38`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- phase status: completed
- event summary: 39 events; one approval; zero retries; zero escalations;
  approval-presentation proof enforced
- out-of-kernel work: code edits, tests, documentation, git operations, and
  validation were performed by the delegated trusted host; Workflow OS
  governed scope and approval but did not execute those actions
