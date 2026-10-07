# Private Trusted-Host Repeated Scheduling Plan

Status: implemented as a crate-private bounded composition. Focused
maintainer/security review is next. No production timer or public scheduling
surface is implemented.

## 1. Executive Summary

Workflow OS can now derive one exact inert `TimeWindow` ticket, wait once
through an injected host boundary, reassess authoritative readiness, and enter
the accepted reinvocation path at most once. That closes one wake operation,
but lawful local work can still stop when execution yields to a later
`TimeWindow` or when an early host wake returns refreshed scheduling posture.

The next implementation should add one crate-private, bounded repeated
scheduling driver for one local SQLite operational window. The driver should
compose the accepted schedule-once helper. It must freshly derive every next
wait from Core, consume fresh operation identities for every wake attempt, and
stop explicitly on terminal, blocked, canceled, unavailable, unsupported,
ambiguous, corrupt, or budget-exhausted posture.

This is not a general scheduler. It does not create model turns, approve work,
persist host jobs, poll Core, scan repositories, manage multiple runs, or
expose runtime configuration.

## 2. Problem Statement

The accepted one-shot helper deliberately returns after one wait and one fresh
verification request. This leaves two bounded liveness gaps:

- an early or spurious host wake returns a refreshed future ticket but does not
  register another wait; and
- a lawful executor attempt may yield to a new exact `TimeWindow`, leaving the
  same operational window in `AwaitCondition` after reinvocation.

Today a caller must notice those results and call the helper again. If that
caller is an agent turn or ad hoc host script, otherwise lawful work can stall
even though Core has already recorded the next exact condition. Conversely, a
naive loop could busy poll, reuse stale identities, hide cancellation, or turn
timer behavior into authority.

The smallest safe answer is a private bounded driver that repeats only the
already-accepted one-shot composition and treats each result as a request to
rehydrate current Core state.

## 3. Goals

- Resume one exact local operational window across a bounded sequence of
  `TimeWindow` waits without requiring a new agent turn.
- Reuse the accepted schedule-once helper rather than duplicating readiness,
  transition, authority, or executor-entry logic.
- Derive every subsequent wait from fresh authoritative state.
- Allocate unique operation and receipt identities for every attempted wake.
- Bound the total number of scheduled wakes and executor entries.
- Stop deterministically on genuine wait, blocked, terminal, canceled,
  unavailable, unsupported, ambiguous, corrupt, or exhausted posture.
- Preserve restart by reconstructing the driver from durable Core state rather
  than persisting authority or in-memory tickets.
- Make cancellation and host shutdown explicit non-terminal host outcomes.
- Prove no busy polling, no automatic approval, no duplicate executor entry,
  and no hidden workflow completion.
- Keep all models crate-private, payload-free, and redaction-safe.

## 4. Non-Goals

This plan does not authorize:

- implementation during this planning phase;
- a general workflow scheduler, daemon, queue, worker pool, wake bus, cron
  service, repository scanner, or multi-run coordinator;
- unbounded loops, periodic polling, fixed-interval retries, or automatic
  retry after security, corruption, ambiguity, or host failure;
- automatic model, assistant, agent, or conversation turns;
- automatic approval, delegated self-approval, evidence bypass, check bypass,
  or inferred authority;
- non-`TimeWindow` wake families;
- durable host-job, lease, heartbeat, or scheduler-registration records;
- public Rust API, CLI, SDK, workflow field, schema, UI, or example changes;
- provider mutation, OpenShell, sandbox lifecycle, nested harness execution,
  hosted/distributed scheduling, PostgreSQL parity, or multi-host failover;
- filesystem writes outside existing SQLite state transitions;
- provider writes, new mutation families, or release posture changes.

## 5. Accepted Foundation

The repeated driver may compose but must not weaken:

- immutable run and exact operational-window binding;
- one coherent scheduling observation and inert private ticket;
- one injected deadline wait that receives only an absolute UTC instant;
- fresh read-only readiness assessment after wake;
- exact source, provenance, epoch, condition, generation, revision, and
  dependency validation;
- trusted-time verification inside the mutation transaction;
- one-use wake transition and domain-separated exact replay;
- fresh current-authority and required-context assessment;
- one-winner atomic dispatch reservation;
- bounded synchronous redispatch while Core returns `ResumeNow`; and
- explicit `AwaitCondition`, `Blocked`, and `Terminal` stop reasons.

Neither the scheduling ticket, host callback, readiness result, driver budget,
nor scheduling identity grants authority.

## 6. Candidate Private Model

The smallest future model should be adjacent to the existing scheduling
module and remain crate-private:

- `TrustedHostRepeatedSchedulingInput`
  - backend and immutable operational locator;
  - injected deadline waiter and executor;
  - immutable skill input;
  - redispatch identity provider;
  - new schedule-wake identity provider; and
  - explicit bounded wake budget.
- `TrustedHostScheduleWakeIdentity`
  - one unique continuity operation ID; and
  - one unique continuity receipt ID.
- `TrustedHostScheduleWakeIdentityProvider`
  - returns one fresh identity pair for a numbered wake attempt;
  - cannot inspect Core state or grant authority.
- `TrustedHostRepeatedSchedulingStopReason`
  - `Canceled`;
  - `Blocked`;
  - `Terminal`;
  - `UnsupportedWait`;
  - `HostUnavailable` only if represented as a typed result rather than an
    error; and
  - `WakeBudgetExhausted`.
- `TrustedHostRepeatedSchedulingOutcome`
  - current authoritative disposition;
  - bounded scheduled-wake count;
  - bounded aggregate executor-entry count;
  - explicit stop reason; and
  - optional redacted skill-output presence only if the accepted lower outcome
    already supports it safely.

Names are provisional. The implementation review should reject any type that
is public, serializable as authority, or capable of carrying workflow payloads.

## 7. Wake Budget

Repeated scheduling must be finite by construction.

- The caller supplies a positive wake budget through a validated private
  wrapper or bounded integer.
- Core enforces a small compile-time maximum to prevent an effectively
  unbounded host loop.
- The budget counts every call to the injected deadline waiter, including
  early wakes and cancellation.
- Budget exhaustion returns an explicit non-terminal host stop. It does not
  fail or complete the workflow and does not satisfy the wait.
- Executor-entry count is accumulated with checked arithmetic from accepted
  schedule-once outcomes.
- A wake budget is a liveness/resource limit, never an authorization limit or
  policy decision.

The first implementation should choose the smallest maximum justified by
tests. It should not expose the value through workflow configuration.

## 8. Repeated Scheduling Algorithm

The future helper should perform this bounded sequence:

1. Validate the private wake budget before reading or mutating state.
2. For the next numbered wake attempt, obtain one fresh operation/receipt
   identity pair.
3. Call `schedule_trusted_host_time_window_once` exactly once.
4. On `Canceled`, stop with current non-terminal posture and no re-arm.
5. On `NotYetEligible`, discard the returned ticket as authority, count the
   wake, and begin the next iteration only by calling schedule-once again so
   Core derives a fresh coherent observation.
6. On `Reinvoked`, checked-add the lower executor-entry count.
7. If the lower stop reason is `Terminal` or `Blocked`, stop immediately.
8. If the lower stop reason is `AwaitCondition`, begin the next iteration only
   if budget remains. The next schedule-once call must classify the current
   wait; it must not reuse the prior handoff or ticket.
9. If current posture is not one exact supported `TimeWindow`, stop with a
   bounded unsupported or ineligible result rather than spinning.
10. Propagate security, corruption, ambiguity, stale-binding, host failure,
    and identity-provider failure without retry.
11. When the wake budget is consumed while lawful waiting remains, return
    `WakeBudgetExhausted` with no fabricated workflow terminal state.

The driver must not call `continuation_disposition` in a polling loop. State is
read only through each accepted schedule-once composition and its lower
boundaries.

## 9. Early Wake And Re-Arm Policy

An early wake is the only first-slice reason to re-arm without an intervening
executor entry.

- The old ticket is never reused.
- The next iteration derives a fresh ticket from current Core state.
- The waiter is called again only while budget remains.
- Repeated immediate early wakes consume budget and stop explicitly; they
  cannot produce an infinite busy loop.
- No durable security rejection is written for ordinary early wake jitter.
- A changed wait, blocked window, terminal run, source mismatch, or stale
  binding prevents re-arm.

The plan intentionally does not add delay backoff. The Core-derived deadline
is the only scheduling hint.

## 10. Cancellation And Shutdown

The injected waiter remains the only cancellation-aware boundary in the first
slice.

- Cancellation before wake returns `Canceled` and ends the driver.
- Host shutdown should cause the waiter to return `Canceled` or a bounded
  unavailable/failure result.
- Cancellation does not mutate workflow state, satisfy a wait, revoke a Core
  transition already committed, or fabricate terminal status.
- The driver does not swallow cancellation and does not automatically restart
  itself.
- A later host process may reconstruct current posture from durable Core state
  and begin a new bounded driver call with fresh identities.

No public cancellation token or scheduler control API is added.

## 11. Restart And Recovery

The first repeated driver remains process-local but restart-safe:

- it persists no ticket, handoff, wake budget, callback, capability, or
  authority;
- after restart, the host supplies the immutable locator and fresh identity
  providers;
- Core rehydrates current durable posture on the first schedule-once call;
- an unsatisfied time wait produces a fresh inert ticket;
- an already-committed wake uses accepted exact replay and operational-entry
  recovery semantics; and
- blocked or terminal posture stops without executor entry.

Losing the in-memory remaining budget on process restart is acceptable in this
private slice because the budget is not authority. A durable scheduler budget
or job record requires separate planning.

## 12. Concurrency And Idempotency

Multiple private drivers may race accidentally, but they must not both enter
the executor for the same operation:

- every driver obtains unique operation/receipt identities per wake attempt;
- exact Core transition, directive consumption, and dispatch reservation
  remain the one-winner boundaries;
- a loser may observe terminal/blocked posture or receive bounded stale/replay
  rejection;
- no loser automatically retries a security or replay rejection; and
- aggregate executor entries across competing drivers remain at most one per
  admitted one-use dispatch.

The first implementation does not elect a leader or add a scheduler lease.

## 13. Unsupported And Genuine Waits

The driver handles only one exact `TimeWindow` at a time. If fresh observation
finds an approval, evidence, external event, conflict, capability, check,
authority-refresh, multi-wait, or unknown condition, it stops. It does not
infer satisfaction, ask a model to decide, or convert that condition into a
timer.

`Blocked` and `Terminal` are final for the driver invocation. They are not
errors to retry. `AwaitCondition` is repeatable only when the next fresh
schedule-once call recognizes the exact supported `TimeWindow` posture.

## 14. Privacy And Redaction

- Input and outcome Debug implementations expose counts and closed enums only.
- Errors use stable static codes and never echo identifiers, deadlines, paths,
  commands, prompts, payloads, credentials, source values, or provider data.
- Identity-provider failures are mapped or propagated without copying raw
  source errors.
- No serialized scheduling record or public inspection payload is introduced.
- Any optional skill output remains redacted exactly as in the accepted lower
  outcome.

## 15. Observability Posture

The first slice should return bounded counters to its private caller but add no
new durable event family or metrics backend. Useful future metrics include:

- scheduled wakes attempted;
- early wakes;
- wake budget exhaustion;
- host cancellation and failure;
- elapsed-wake latency; and
- duplicate-driver rejection.

Metrics must not expose workflow identity or payloads. Durable scheduling
telemetry requires separate review.

## 16. Test Plan

The implementation and accepted lower-boundary suites prove:

1. two sequential elapsed `TimeWindow` waits are handled in one bounded driver
   call without a new agent turn;
2. terminal and blocked lower outcomes stop immediately;
3. one early wake re-arms only through a fresh schedule-once observation;
4. repeated immediate early wakes consume the budget and stop without busy
   polling or Core mutation;
5. cancellation stops without re-arm, workflow failure, or completion;
6. unavailable and failed waiters stop without retry or executor entry;
7. unsupported or multi-wait posture stops without calling the waiter;
8. every wake attempt uses a distinct operation and receipt identity;
9. identity-provider failure occurs before the corresponding waiter call;
10. restart rehydrates fresh state and continues without persisted authority;
11. two competing repeated drivers preserve one-winner transition and atomic
    executor-entry semantics;
12. stale authority, required context, wait, trusted time, or locator binding
    blocks the next entry;
13. aggregate wake and executor-entry counters use checked arithmetic;
14. budget zero and values above the private maximum fail before state access;
15. Debug and errors remain redaction-safe;
16. no public API, serde, CLI, SDK, schema, provider, OpenShell, nested-harness,
    hosted, or write behavior appears; and
17. existing continuity, supervisor, redispatch, scheduling, runtime, adapter,
    report, and workspace tests remain green.

## 17. Proposed Implementation Sequence

1. Completed: add the private bounded wake-budget and per-wake
   identity-provider models.
2. Completed: add one private repeated scheduling helper that exclusively
   composes the accepted schedule-once helper.
3. Completed: add focused sequential-wait, early-wake budget, cancellation,
   host-failure, unsupported-wait, identity, and redaction tests while retaining
   the accepted lower restart, stale-state, and concurrent-callback proofs.
4. Completed: create an implementation report and update the roadmap honestly.
5. Next: perform focused maintainer/security review.
6. Deferred: only after acceptance, consider a private production timer
   implementation or another wake family as a separate phase.

No public host integration should begin in this sequence.

## 18. Open Questions

- What smallest compile-time wake maximum proves useful repeated continuation
  without resembling a general run loop?
- Should host unavailability be a typed stop outcome or remain a structured
  error, given the accepted schedule-once contract?
- Should the driver retain the latest redacted skill-output presence, or only
  aggregate counts and stop reason?
- Does a future production timer belong in the CLI process, a dedicated local
  host crate, or an injected application boundary outside Core?
- What later phase, if any, should persist non-authoritative scheduler-job
  correlation for process recovery?

## 19. Final Recommendation

The crate-private bounded repeated scheduling driver and focused proofs are
implemented. The next phase should review the implementation for boundedness,
fresh-state derivation, identity uniqueness, cancellation, concurrency,
restart, and non-leakage before any production host integration is considered.

Do not implement a general scheduler, public configuration, model-turn
automation, automatic approval, provider execution, OpenShell, nested
harnesses, hosted scheduling, additional wake families, schemas, CLI, SDK,
writes, or release changes.
