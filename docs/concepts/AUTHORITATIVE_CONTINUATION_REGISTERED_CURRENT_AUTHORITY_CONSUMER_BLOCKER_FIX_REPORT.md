# Authoritative Continuation Registered Current-Authority Consumer Blocker Fix Report

## 1. Executive Summary

Two focused maintainer/security reviews required direct executor-composition
evidence before accepting the crate-private source-backed continuation
boundary. The final blocker fix now exercises the real approval-resume path,
fresh registered current-authority source, durable duplicate and stale-cursor
claim outcomes, exact binding substitutions, and local handler together rather
than relying only on separately tested primitives.

## 2. Blocker Fixed

The implementation previously proved one positive approval-resume path but
did not directly prove failure-before-claim, failure-before-handler, claim and
event ordering, replay behavior, or fresh reassessment at the exact executor
composition boundary.

The first blocker fix added direct durable-state, event-log, and handler-counter
assertions for most properties. Focused review correctly found that terminal
replay stopped before the claim boundary, stale cursor was still proved only
below the composed path, and several exact identities were not substituted at
the executor entry. The final fix replaces that indirect evidence with a
test-only state-backend control and valid recomputed test bindings. No public
or operational behavior was broadened.

## 3. Test Implementation

The composition fixture creates a real approval-gated immutable run, registers
one bounded local skill handler, constructs the exact harness contract and
binding, and uses the private source-backed executor builder. Tests inspect
durable idempotency records with structured JSON parsing and inspect the run's
ordered events.

Coverage now proves:

- a continuation claim is durable before handler entry;
- approval grant, resume, policy, invocation request, invocation attempt,
  invocation success, and terminal completion remain ordered;
- unmet authority and stale source posture produce zero claims and zero
  handler calls;
- immutable bundle, workflow, run, step, actor, harness ID, harness version,
  and contract-hash substitutions all fail closed without a claim or handler
  call;
- an exact duplicate claim after fresh authority resolution fails closed with
  zero handler calls;
- a legitimate durable cursor advance after claim selection burns the claim
  and blocks the handler; and
- the same registered source is freshly reassessed at each executor use time.

The duplicate and stale tests wrap the real state backend at the idempotency
boundary. The wrapper is compiled only for tests and forwards every other
state operation unchanged. Binding substitutions use a test-only helper that
recomputes the binding commitment after changing exactly one field, ensuring
the executor receives a valid but wrong binding rather than malformed input.

## 4. Runtime Behavior

The final tests did not expose a production behavior defect. The source-backed path
continues to resolve current authority before every attempted continuation,
bind the resulting payload-free commitment into the existing cursor-bound
claim, and invoke the existing handler path only after the first durable
claim.

The earlier approval-resume integrity fixes remain unchanged: approval does
not replace the original execution actor, and resume retains the immutable
run bundle.

## 5. Privacy And Error Safety

The tests use bounded synthetic identifiers and parse only local test-state
records. Negative errors remain stable and do not include substituted binding
values, source records, context targets, paths, prompts, command output,
provider payloads, environment values, or credentials.

## 6. Scope Explicitly Not Added

- No public authority or continuation API.
- No runtime authority-source configuration.
- No provider mutation, OpenShell integration, sandbox execution, nested
  harness runtime, SideEffect execution, schema, SDK, CLI, hosted, or release
  behavior.
- No claim of external transactional atomicity or crash-safe consumer outcome
  recovery.

## 7. Focused Validation

- `cargo test -p workflow-core --lib source_backed_executor`: passed, 7 tests
- `cargo test -p workflow-core --lib same_registered_source_is_reassessed_at_each_executor_use_time`: passed, 1 test
- `cargo fmt --all --check`: passed
- `cargo clippy --workspace --all-targets -- -D warnings`: passed
- `cargo test --workspace`: passed
- `npm run check`: passed
- `npm run check:integrations`: passed
- `npm run check:docs`: passed
- `git diff --check`: passed

## 8. Governed Fix Record

- workflow ID: `dg/blocker`
- initial run ID: `run-1786783836561730000-2`
- initial approval ID: `approval/run-1786783836561730000-2/fix-approved`
- initial approval presentation ID: `presentation/d2825cde4479c76a`
- final run ID: `run-1786790396422164000-2`
- final approval ID: `approval/run-1786790396422164000-2/fix-approved`
- final approval presentation ID: `presentation/650b96ce6cb09efd`
- approval outcomes: both granted through their emitted proof-enforced approval commands
- final phase status: `Completed`
- final event summary: 39 events, 1 approval, 0 retries, 0 escalations
- final approval-presentation enforcement: proof enforced with the persisted
  presentation record and approval event marker present
- out-of-kernel work: test implementation, code inspection, command execution,
  and report authorship were performed by the delegated maintainer; the
  kernel governed scope and approval but did not edit files or run checks

## 9. Remaining Limitations

Crash-after-claim ambiguity remains conservative. The final blocker fix proves
the existing duplicate and stale-cursor behavior at the exact composed path
without creating a public way to forge durable cursor or binding state.

## 10. Recommended Next Phase

Perform one focused maintainer/security blocker-fix review. If accepted, close
the private source-backed continuation proof and begin the separately scoped
P0 continuity lane for authorized execution windows, executor yield, typed
waits, and scoped delegated authority.
