# Bounded Trusted-Host Redispatch Loop Implementation Report

## 1. Executive Summary

The private local SQLite bounded trusted-host redispatch loop is implemented.
One Core-owned helper now composes the accepted execution-window opening,
one-shot trusted-host supervisor, continuation-disposition, directive
consumption, and atomic dispatch-reservation boundaries.

The helper invokes one injected executor for the initial admitted attempt and
continues only while a fresh durable read derives `ResumeNow`. Every resumed
iteration consumes one distinct directive, admits one exact attempt through
the existing atomic reservation boundary, and reuses the exact immutable
`SkillInput` and executor commitment fixed by the opening operation. It stops
only on `AwaitCondition`, `Blocked`, `Terminal`, or a structured error. It
cannot return a normal outcome while immediate lawful continuation remains.

This remains private, local, injected, SQLite-only, and review-pending. It does
not add a scheduler, create model turns, approve gates, execute providers,
mutate external systems, integrate OpenShell, run nested harnesses, or expose
public configuration, CLI, SDK, schemas, hosted behavior, or release changes.

## 2. Scope Completed

- Added a crate-private `TrustedHostRedispatchLoopInput` carrying one initial
  capability, one immutable invocation, one injected executor, initial
  persistence identities, and one narrow identity provider.
- Added a crate-private identity provider that can supply only fresh bounded
  operation, receipt, attempt, and yield-generation identifiers.
- Added a redaction-safe loop outcome with final Core disposition, admitted
  executor-entry count, closed stop reason, and optional in-memory output.
- Added one bounded loop that freshly re-derives continuation state before
  every resumed attempt.
- Added exact durable-window binding and authoritative attempt-limit checks.
- Added deterministic current-yield, wait-revision, and available-directive
  selection before projected directive consumption.
- Rehydrated the current run cursor after directive consumption before the
  resumed supervisor call.
- Reused the accepted one-shot supervisor so every executor entry still
  requires one atomic dispatch reservation.
- Added focused tests for one and two consecutive redispatches, exhausted
  authority, immutable input binding, privacy, and workflow-status posture.

## 3. Scope Explicitly Not Completed

This phase did not add:

- a scheduler, daemon, queue, background worker, poller, or timer;
- model conversation creation, model-turn resumption, or agent recirculation;
- automatic approval, delegated-authority expansion, evidence synthesis,
  check synthesis, or policy bypass;
- provider execution, provider mutation, a live adapter, or OpenShell;
- nested harness execution, recursive agents, or agent swarms;
- public runtime configuration, workflow fields, CLI, SDK, UI, or examples;
- filesystem or PostgreSQL loop parity;
- multi-host leases, heartbeat, failover, fairness, or reservation stealing;
- a caller-selected iteration budget or successful host-preemption return;
- automatic retry after ambiguous executor entry or commit acknowledgement;
- new runtime-event, SideEffect, approval, report-artifact, or reasoning-lineage
  vocabulary; or
- hosted, distributed, production, or release-readiness claims.

## 4. Private API Summary

The implementation is isolated in
`sqlite_state::trusted_host_redispatch_loop` and is not exported from
`workflow-core`.

The private boundary consists of:

- `TrustedHostRedispatchIterationIdentity`;
- `TrustedHostRedispatchIdentityProvider`;
- `TrustedHostRedispatchLoopInput`;
- `TrustedHostRedispatchStopReason`;
- `TrustedHostRedispatchLoopOutcome`; and
- `run_bounded_trusted_host_redispatch_loop`.

The provider is deliberately non-authorizing. It cannot receive, construct,
or replace invocation input, executor binding, durable state, capabilities, or
commitments. All authority remains owned by Core operations.

## 5. Iteration And Authority Boundary

The helper follows this sequence:

1. Snapshot the immutable supervisor binding from the one-use initial
   capability before consuming it.
2. Invoke the accepted one-shot supervisor for the initial attempt.
3. Return immediately if the resulting Core disposition is
   `AwaitCondition`, `Blocked`, or `Terminal`.
4. When the result is `ResumeNow`, derive the disposition again from durable
   state.
5. Validate the exact durable window binding and prove another attempt is
   available under the authoritative maximum-attempt count.
6. Obtain fresh non-authorizing identifiers from the injected provider.
7. Load the active yield, canonical wait revisions, and exactly one available
   directive.
8. Consume that directive through the existing projected compare-and-set
   operation, producing one fresh attempt capability.
9. Rehydrate the run cursor and invoke the existing one-shot supervisor with
   the unchanged input and executor.
10. Repeat only while a fresh Core disposition remains `ResumeNow`.

Every resumed attempt still crosses the atomic reservation operation before
executor entry. Directive or reservation replay cannot reissue authority.

## 6. Liveness And Stop Semantics

The durable execution-window attempt limit is the only finite loop bound. The
helper accepts no host-selected budget. If Core derives `ResumeNow` after the
window has no lawful next attempt, the helper returns
`trusted_host_redispatch.attempt_limit_inconsistent` rather than fabricating
completion, wait, block, approval, or success.

Normal stop reasons are closed:

- `AwaitCondition`;
- `Blocked`; and
- `Terminal`.

`Terminal` is execution-window posture, not proof that the workflow run
completed. Tests preserve the workflow run in `Running` after successful local
attempt completion.

## 7. Invocation Integrity

The exact `SkillInput` is carried once in the loop input and cloned only to
pass identical owned values into successive one-shot calls. The executor
binding is supplied by the same injected executor and checked against the
opening operation commitment before every reservation and executor entry.

The helper cannot accept replacement invocation material per iteration. A
changed input fails with the existing stable
`trusted_host_supervisor.invocation_binding_mismatch` error before the first
executor call or identity-provider request.

## 8. Privacy And Redaction

- Loop input, iteration identity, and loop outcome Debug output is bounded and
  redacted.
- In-memory `SkillOutput` remains outside durable events and continuity rows.
- Stable redispatch errors contain no caller identifiers, invocation values,
  paths, output, metadata, credentials, tokens, or provider payloads.
- Durable state remains limited to the already-accepted identifiers,
  commitments, revisions, cursors, trusted-time facts, and closed result
  vocabulary.
- No prompt, transcript, source content, command output, environment value,
  credential, authorization header, token, or provider payload is added.

## 9. Test Coverage

Focused tests prove:

- one opened attempt yields and succeeds after one redispatch;
- two consecutive `ResumeNow` postures consume two distinct identity sets and
  produce three admitted executor entries;
- a `ResumeNow` posture with exhausted authoritative attempts fails closed
  before requesting another identity;
- substituted invocation input is rejected before executor entry;
- loop Debug output omits output references and output-map keys;
- stable errors omit run identifiers and secret-like input markers;
- successful execution-window closure does not complete the workflow run; and
- the complete Core library regression suite remains green.

The existing continuity, directive, reservation, one-shot supervisor,
concurrency, replay, ambiguity, persistence-fault, restart, event-ordering,
adapter, and runtime tests continue to cover the primitives composed by this
private helper.

## 10. Validation

Passed during implementation:

- `cargo fmt --all --check`
- focused Core loop tests (`4 passed`)
- `cargo clippy -p workflow-core --all-targets -- -D warnings`
- `cargo test -p workflow-core --lib` (`352 passed`)
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `git diff --check`

The complete local workspace suite passed. Pull-request CI remains an
independent repository proof rather than a substitute for local validation.

## 11. Remaining Known Limitations

- The helper is private, local, injected, and SQLite-only.
- No public runtime path invokes it yet.
- Process loss during an active `ResumeNow` loop still requires a separately
  reviewed durable continuation handoff.
- A lost admitted capability is not leased or reassigned.
- There is no filesystem, PostgreSQL, multi-host, scheduler, or provider
  integration.
- The loop outcome does not itself create evidence or a WorkReport artifact.
- Typed wait wake-up registration remains a separate governed operation.

## 12. Recommended Next Phase

Run a focused maintainer/security implementation review. The review must
verify fresh disposition derivation, exact authority consumption, one-winner
reservation reuse, finite-bound behavior, immutable invocation, non-leaking
errors and Debug output, and absence of public/provider/scheduler broadening.

Do not begin provider execution, OpenShell, nested harnesses, public
configuration, CLI, SDK, hosted execution, or additional provider mutation
until this private slice is accepted.

## 13. Governed Phase Record

- workflow: `dg/implement`
- run: `run-1790953001521591000-2`
- approval: `approval/run-1790953001521591000-2/implementation-approved`
- presentation: `presentation/6540ea25b9b9a44a`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: private local SQLite bounded trusted-host redispatch loop
  only
- phase status: completed
- event summary: 39 events, including 1 approval request, 1 approval grant,
  8 policy decisions, 6 scheduled steps, 6 successful skill invocations, 0
  retries, and 0 escalations
- out-of-kernel execution: source inspection, code edits, tests, documentation,
  and git actions are performed by the delegated trusted host; Workflow OS
  governs scope and approval but does not edit files or execute shell commands
