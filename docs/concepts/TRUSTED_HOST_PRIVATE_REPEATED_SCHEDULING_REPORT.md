# Private Trusted-Host Repeated Scheduling Implementation Report

## 1. Executive Summary

Workflow OS now has one crate-private bounded driver that can continue one
local SQLite operational window across a finite sequence of exact
`TimeWindow` waits without requiring a new agent turn. The driver exclusively
composes the accepted schedule-once helper. It does not become a scheduler,
grant authority, approve work, create model turns, or expose a public runtime
surface.

## 2. Scope Completed

- Added a private validated wake budget with a compile-time maximum of eight.
- Added a private per-wake operation/receipt identity provider contract.
- Added a private repeated scheduling input, outcome, and explicit stop
  taxonomy.
- Added bounded composition through the accepted schedule-once helper.
- Added checked aggregate wake and executor-entry accounting.
- Added focused sequential-wait, early-wake, cancellation, host-failure,
  unsupported-wait, budget, identity, and redaction proofs.

## 3. Scope Explicitly Not Completed

This phase did not add a production timer, daemon, queue, worker pool, general
scheduler, durable host job, polling loop, public API, CLI, SDK, schema,
workflow configuration, automatic model turn, automatic approval, provider
execution, provider mutation, OpenShell, nested harness execution,
PostgreSQL parity, hosted scheduling, filesystem output, or release change.

## 4. Private API Summary

The private model adds:

- `TrustedHostRepeatedWakeBudget`;
- `TrustedHostScheduleWakeIdentity`;
- `TrustedHostScheduleWakeIdentityProvider`;
- `TrustedHostRepeatedSchedulingInput`;
- `TrustedHostRepeatedSchedulingStopReason`;
- `TrustedHostRepeatedSchedulingOutcome`; and
- `run_bounded_trusted_host_repeated_scheduling`.

None of these types is publicly exported or serializable.

## 5. Boundedness

The caller must supply a positive private wake budget no greater than eight.
Every successful call to the injected deadline waiter consumes one wake. An
early waiter that returns immediately therefore cannot produce an unbounded
loop. Exhaustion returns `WakeBudgetExhausted` with authoritative
`AwaitCondition` posture; it does not fail or complete the workflow.

Wake and executor-entry counters use checked arithmetic. Host wait failure and
identity-provider failure return structured errors and are not retried.

## 6. Fresh-State And Identity Behavior

Every iteration obtains a fresh operation/receipt identity and calls the
accepted schedule-once helper. A prior ticket or handoff is never reused.
After an early wake or an executor yield, the next iteration asks Core to
derive current scheduling posture again.

Sequential time waits use distinct durable wait-condition identities. A wait
condition version does not make one durable wait identity reusable.

## 7. Stop Behavior

The driver stops explicitly on:

- host cancellation;
- authoritative blocked posture;
- authoritative terminal posture;
- unsupported or non-time wait posture; and
- wake-budget exhaustion.

Waiter unavailable/failed results, stale or invalid security bindings,
corruption, ambiguity, replay failure, and identity-provider failure remain
structured non-retried errors.

## 8. Workflow Semantics

The helper neither changes workflow pass/fail semantics nor fabricates a
terminal run. Cancellation and budget exhaustion are host-level stops.
Authority, wait satisfaction, current context, and executor admission remain
inside the accepted Core transitions.

## 9. Restart And Concurrency

The driver persists no ticket, callback, remaining budget, capability, or
authority. A later process may reconstruct a new bounded call from the durable
locator and fresh identity providers. Accepted lower schedule-once and
operational-entry tests continue to prove SQLite reopen and concurrent
callback one-winner behavior. This phase adds no leader election or lease.

## 10. Privacy And Redaction

Inputs and outcomes are private and count-oriented. Custom `Debug`
implementations redact operation, receipt, run, workflow, step, actor, bundle,
and payload bindings. Errors use stable codes and fixed messages. The aggregate
outcome intentionally retains no skill output.

## 11. Test Coverage

Focused tests cover:

- zero, maximum, and above-maximum wake budgets;
- two sequential exact time waits in one driver call;
- terminal stop after two accepted executor entries;
- repeated early wakes consuming the full budget without Core mutation;
- cancellation without re-arm;
- unavailable and failed host waits without retry;
- unsupported no-wait posture without waiter or executor entry;
- fresh wake identity allocation per attempted iteration; and
- bounded redaction-safe model output.

The focused sequential test uses short real trusted-time intervals because the
production schedule-once helper intentionally reads the Core-owned trusted
clock; the injected waiter itself performs no polling. Existing restart,
stale-binding, transition replay, and concurrent-callback suites remain part
of full workspace validation.

## 12. Validation

- focused repeated scheduling tests: passed, 5 tests;
- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed;
- `npm run check:docs`: passed;
- `git diff --check`: passed.

## 13. Remaining Limitations

- No production host timer calls this driver.
- Only exact single `TimeWindow` waits are supported.
- The wake budget is process-local and resets after restart.
- Host unavailability remains an error rather than a typed stop outcome.
- No durable scheduling metrics or job correlation exists.
- No public or hosted scheduling contract exists.

## 14. Recommended Next Phase

Perform a focused maintainer/security review of the private repeated
scheduling driver. Review finite construction, fresh-state derivation,
identity uniqueness, cancellation, error non-retry, restart, concurrency,
privacy, and the absence of public scheduling or execution broadening.

## 15. Governed Phase Record

- workflow: `dg/implement`
- run: `run-1791356587320883000-2`
- approval:
  `approval/run-1791356587320883000-2/implementation-approved`
- presentation: `presentation/5fa0b6039df4e15a`
- presentation hash:
  `5fa0b6039df4e15ae4dba7583fe15b4e9f49512056b5efa1af16883087a7045f`
- approval outcome: granted by delegated maintainer through proof enforcement
- approved boundary: crate-private bounded repeated scheduling only
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations;
  approval presentation proof marker present
- out-of-kernel work: source inspection, implementation, test authoring,
  documentation, validation, and later git and pull-request actions
