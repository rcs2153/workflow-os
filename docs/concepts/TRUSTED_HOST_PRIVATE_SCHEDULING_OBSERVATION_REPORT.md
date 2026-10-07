# Trusted-Host Private Scheduling Observation Report

## 1. Executive Summary

The first private trusted-host scheduling-boundary slice is implemented. Core
can now derive one coherent `TimeWindow` scheduling observation containing the
current inert handoff and a matching absolute-UTC schedule ticket. After a
host wake, Core can assess the exact current binding as `Eligible` or
`NotYetEligible` through a read-only transaction.

The ticket and readiness result grant no authority. They do not transition a
wait, reserve dispatch, invoke an executor, or prove that a deadline elapsed.
The accepted transition transaction remains the only deadline-satisfaction
boundary and obtains trusted time again atomically.

## 2. Scope Completed

- Added a crate-private coherent scheduling observation.
- Added a crate-private inert absolute-UTC schedule ticket.
- Bound the ticket to the current handoff, window revision, yield generation,
  wait identity and revision, dependency commitment, trusted-time source
  binding, and closed next operation.
- Added a crate-private non-mutating readiness assessment.
- Added deterministic early, elapsed, stale, trusted-time-binding, restart,
  zero-write, and redaction tests.
- Updated the accepted plan and roadmap honestly.

## 3. Scope Explicitly Not Completed

This phase does not add a deadline waiter, timer, sleep, polling, callback,
schedule-once host helper, automatic re-arming, repeated scheduling, executor
entry, wait transition, workflow mutation, provider execution, OpenShell,
nested harnesses, automatic approval, public Rust API, CLI, SDK, workflow
schema, hosted behavior, provider write, or release change.

## 4. Coherent Observation Boundary

`observe_trusted_host_time_window_scheduling` opens an existing SQLite database
read-only, loads one authoritative continuity snapshot, obtains one Core-owned
trusted-time observation, and derives the continuation disposition, inert
handoff, and matching ticket from that same snapshot.

Only an exact single unsatisfied `TimeWindow` wait may produce a ticket.
Blocked, terminal, unsupported, ambiguous, stale, or corrupt posture produces
no actionable ticket or fails closed with a stable bounded error.

## 5. Inert Ticket Model

The private ticket contains the exact absolute UTC scheduling instant needed
by a future injected waiter, but its commitment binds all scheduling-relevant
state. It is not serializable, public, or a capability. Its Debug output
redacts the instant and all identifiers and commitments.

Copying, delaying, substituting, or firing the ticket cannot satisfy a wait or
admit executor entry. The only closed next-operation vocabulary is
`WaitOnceThenRequestFreshVerification`.

## 6. Readiness Assessment

`assess_trusted_host_time_window_readiness` uses an existing read-only SQLite
connection and fresh Core-owned trusted time. It validates current locator,
handoff, ticket, wait, dependency, revision, source, provenance, and epoch
bindings.

- `NotYetEligible` returns refreshed inert scheduling posture.
- `Eligible` returns no capability and only permits a future host composition
  to request the accepted explicit reinvocation once.
- Stale or substituted tickets and invalid trusted-time bindings fail closed.

The assessment writes no operation, security rejection, event, directive,
reservation, receipt, wait state, or workflow transition.

## 7. Authority And Atomicity

Readiness is deliberately advisory. Even after `Eligible`, the accepted
reinvocation path must validate the handoff, obtain trusted time again inside
the transition transaction, commit or exactly replay the source-specific
transition, reload current state, consume fresh one-use authority, reserve
dispatch atomically, and reassess current context before entry.

The new slice therefore improves liveness classification without weakening
the existing one-winner or authority boundaries.

## 8. Privacy And Redaction

- Debug output omits timestamps, workflow/run/step identifiers, paths,
  commitments, payloads, commands, prompts, credentials, and provider data.
- Errors use stable static codes and messages and do not echo supplied values.
- The ticket uses payload-free commitments for source and state binding.
- No new serde, public output, schema, CLI, SDK, or persistence surface exists.

## 9. Test Coverage

Focused tests prove:

- deterministic coherent observation and ticket derivation;
- exact absolute-UTC scheduling instant inside the private model;
- restart derives the same inert ticket from authoritative SQLite state;
- early readiness returns `NotYetEligible` and refreshed posture;
- elapsed readiness returns `Eligible` without authority;
- observation and assessment leave authoritative state byte-for-byte
  equivalent at the model level;
- substituted tickets fail closed;
- invalid trusted-time provenance fails closed without leakage; and
- Debug output does not expose the deadline, commitment, or run identity.

## 10. Commands Run And Results

- Focused scheduling observation tests: passed.
- Focused readiness tests: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 11. Remaining Limitations

- There is no deadline waiter or schedule-once host composition.
- No code sleeps, polls, registers a callback, or invokes reinvocation.
- Only one exact local SQLite `TimeWindow` wait is representable.
- Filesystem, PostgreSQL, multi-host, public configuration, and hosted parity
  remain deferred.
- The slice cannot create a new model or assistant turn.

## 12. Recommended Next Phase

Perform focused maintainer/security review of the coherent observation,
ticket, and readiness assessment. Verify zero-write behavior, source and stale
binding rejection, non-authority semantics, privacy, and the retained atomic
transition boundary.

Only after acceptance should the project consider one injected deadline
waiter and one private schedule-once host helper. Repeated scheduling,
additional wake families, provider execution, OpenShell, public host
integration, and hosted behavior remain deferred.

## 13. Governed Phase Record

- workflow: `dg/implement`
- run: `run-1791352444316096000-2`
- approval:
  `approval/run-1791352444316096000-2/implementation-approved`
- presentation: `presentation/84c71f67d28b4b13`
- approval outcome: granted by delegated maintainer through proof enforcement
- approved boundary: coherent observation, inert ticket, non-mutating
  readiness assessment, focused tests, and honest documentation only
- phase status: `Completed`
- event summary: 39 events, one approval, zero retries, zero escalations;
  approval-presentation proof enforcement present
- validation summary: focused tests, formatting, strict workspace clippy, full
  workspace tests, docs checks, and diff checks passed
- out-of-kernel work: source inspection, Rust implementation, tests,
  documentation, validation, and later git and pull-request actions
