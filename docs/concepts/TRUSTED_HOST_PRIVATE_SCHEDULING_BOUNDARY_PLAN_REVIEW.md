# Private Trusted-Host Scheduling Boundary Plan Review

## 1. Executive Verdict

**Needs planning blocker fixes.**

Fix-forward status: the planning blockers are addressed in the corrected plan
and documented in the [planning blocker-fix
report](TRUSTED_HOST_PRIVATE_SCHEDULING_BOUNDARY_PLAN_BLOCKER_FIX_REPORT.md).
This original verdict is preserved pending focused blocker-fix re-review.

The plan has the correct architectural boundary: Core remains authoritative,
the host owns waiting only, the ticket and callback are inert, and one host
wake may request only one source-specific reinvocation. The planned boundary
also stays private, local, single-host, and schedule-once.

Implementation is not yet safe to authorize. The plan promises that an early
or spurious callback preserves wait posture, but the accepted
`TimeWindow` transition currently records an unelapsed deadline as a security
rejection and the explicit reinvocation helper returns a stable security
error. The plan also leaves the clock-domain representation unresolved while
proposing an absolute instant or delay as input to the host waiter.

The blocker fix must define one Core-owned, non-authorizing readiness
assessment that can return `NotYetEligible` without writing a security
rejection, choose the first host scheduling representation, and preserve the
atomic transition's fresh trusted-time recheck as the only satisfaction proof.

## 2. Scope Verification

The plan remains within the approved planning-only scope.

It does not authorize:

- scheduler, timer-driver, queue, daemon, worker-pool, or polling code;
- repeated scheduling;
- model, assistant, agent, or conversation-turn creation;
- automatic approval or authority inferred from delegated prose;
- provider or sandbox execution, OpenShell, or nested harnesses;
- public runtime configuration, workflow fields, CLI, SDK, schema, UI, or
  examples;
- hosted, distributed, multi-host, lease, heartbeat, or failover behavior;
- provider writes or another mutation family; or
- release or production-readiness changes.

No accidental runtime implementation was found.

## 3. Boundary Assessment

The proposed schedule-once boundary is appropriately narrow.

- Core derives scheduling posture from authoritative wait state.
- The host receives no execution authority.
- The injected deadline waiter cannot inspect Core state.
- A timer callback is treated as a request for verification, not proof.
- Operational entry continues through the accepted directive, reservation,
  authority, and required-context boundaries.
- The helper does not loop or re-arm itself in the first slice.

This preserves the product boundary: Workflow OS governs a local injected
executor but does not claim to create a new turn in Codex, ChatGPT, or another
conversational host.

## 4. Source-Of-Truth Assessment

The source-of-truth table is sound.

- SQLite continuity state owns the exact wait and dependency binding.
- Core-owned trusted time owns wake eligibility.
- Fresh continuation classification owns continue, wait, block, and stop.
- One-use capabilities and atomic reservation own executor admission.
- Durable terminal transitions own completion.
- Restart rehydrates state rather than deserializing authority.

The plan correctly prevents a schedule ticket, callback, host wall clock,
durable receipt, or assistant response from substituting for these sources.

## 5. Scheduling Ticket Assessment

An additional private scheduling projection is justified because the existing
opaque handoff intentionally exposes no deadline. Adding a test-only or
host-only accessor to the handoff would blur orientation and scheduling
semantics.

The ticket design is safe in principle if it remains:

- derived coherently with the handoff;
- bound by payload-free commitments;
- private and non-serializable as authority;
- limited to one `TimeWindow` source;
- useful only as a liveness hint; and
- revalidated against current state after wake.

The plan should not call the ticket a capability. It does not authorize any
Core mutation or executor entry.

## 6. Clock-Domain Assessment

The current accepted trusted-time source is `CoreInjectedClockV1`, implemented
by a Core-owned observation of system UTC. The durable wait stores an absolute
UTC deadline plus trusted-time source, provenance, and epoch binding.

The plan currently leaves open whether the host receives an absolute instant,
a delay, or both. That is too late to resolve during implementation because
the choice determines restart behavior, clock-jump handling, early wake
behavior, and the injected waiter contract.

The smallest first-slice correction is:

1. expose one private absolute UTC scheduling instant as an inert hint because
   it matches the accepted durable deadline and current Core clock domain;
2. permit the host waiter to wake early, late, or spuriously without assigning
   authority to its clock;
3. perform a fresh Core-owned readiness observation after wake; and
4. retain the atomic transition's own trusted-time observation as the only
   satisfaction proof.

A relative delay may be computed inside the injected host for timer APIs, but
it must not become durable state or a Core satisfaction input. Supporting a
different trusted-time source or clock domain requires a later reviewed
contract.

## 7. Early-Wake Blocker

The plan says an early or spurious wake should preserve an unsatisfied wait and
return refreshed wait posture or a stable not-yet-actionable result. The
accepted code does not currently provide that behavior.

`transition_time_window_wait` treats `observed_at < deadline` as
`wait.time_window_unsatisfied` with security kind. The transactional boundary
can persist that as a committed security rejection. The explicit
`reinvoke_after_time_window_wait` helper then converts
`SecurityRejected` into `trusted_host_time_window_reinvocation.wake_rejected`.

Therefore the planned algorithm cannot safely call the accepted reinvocation
helper after any host wake and still promise benign early-wake behavior.
Treating normal timer jitter as a security rejection would create noisy audit
state and can poison a specific operation identity for exact replay.

The blocker fix must introduce a private Core-owned readiness assessment that:

- obtains a fresh trusted-time observation through the accepted source;
- verifies source, provenance, epoch, exact dependency, and current wait
  binding;
- returns `Eligible`, `NotYetEligible`, or a structured security/corruption
  error;
- writes no wait transition, rejection, directive, reservation, or event;
- grants no wake or execution capability; and
- does not replace the transition transaction's fresh trusted-time check.

Only `Eligible` may proceed to the accepted explicit reinvocation helper.
`NotYetEligible` returns fresh inert scheduling posture with zero mutation and
zero executor entry.

## 8. Concurrency And Crash Prerequisites

The plan correctly blocks scheduling implementation until two proofs land and
receive focused review:

1. two callers racing through the complete explicit reinvocation composition
   admit at most one executor entry; and
2. a crash after committed transition but before operational entry recovers
   through exact replay after backend reopen without duplicate entry or
   reconstructed authority.

These are implementation prerequisites, not substitutes for the planning
blocker fix above. The readiness assessment must also be covered by duplicate
and stale-observation tests when later implemented.

## 9. Restart And Liveness Assessment

The restart posture is conservative and correct.

- The host persists no private authority.
- A restarted host obtains fresh scheduling posture from authoritative state.
- A satisfied wait, changed authority, blocker, terminal state, or corrupt
  record cannot be resumed from a stale ticket.
- Ending a host callback or assistant turn does not become workflow
  completion.

The first slice should not add a scheduler database. Reconstruction from
authoritative waits is the smallest useful local proof.

## 10. Duplicate And Cancellation Assessment

The plan correctly requires one-winner behavior for duplicate callbacks and
does not translate cancellation or host failure into workflow failure.

The blocker fix should clarify that:

- cancellation before wake returns a host-level canceled outcome with no Core
  mutation;
- cancellation racing after the readiness assessment cannot cancel a
  transition already committed by Core;
- a duplicate callback receives `NotYetEligible`, exact replay, current-state
  rejection, blocked posture, or terminal posture as derived by Core; and
- host retry policy is not introduced by the first schedule-once helper.

## 11. Security And Privacy Assessment

The planned boundary preserves the required posture:

- no callback or ticket is authority;
- no caller supplies deadline satisfaction;
- no capability is serialized or reconstructed;
- no raw payload, prompt, command, source content, approval reason, evidence
  body, check output, provider data, credential, or token is exposed;
- security rejection remains non-oracular; and
- Debug and error output stays bounded.

An absolute UTC scheduling hint is acceptable only on this private boundary.
Public exposure, persistence outside authoritative continuity state, or use
with another time source requires separate review.

## 12. Test Plan Assessment

The proposed test plan is broad enough after adding focused readiness tests:

- `NotYetEligible` writes no operation or security rejection;
- eligible readiness does not grant authority;
- a clock change between readiness and transition is caught by the atomic
  transition recheck;
- wrong source, provenance, epoch, dependency, or wait revision fails closed;
- early and spurious callbacks return wait posture with zero executor entry;
- duplicate callbacks remain one-winner;
- cancellation and host failure preserve workflow posture; and
- restart derives a fresh ticket rather than reusing capability.

Full workspace regression remains required for later implementation.

## 13. Blockers

1. **Early wake semantics are incompatible with the accepted transition.**
   Define a non-mutating Core readiness assessment so ordinary early timer
   behavior does not create a durable security rejection.
2. **The first clock-domain contract is unresolved.** Select the current
   durable absolute UTC deadline as the private host scheduling hint, with
   fresh Core verification after wake and atomic recheck at transition.

These are planning blockers. Scheduler implementation must not begin until the
plan is corrected and re-reviewed.

## 14. Non-Blocking Follow-Ups

- Decide later whether repeated scheduling needs a durable non-authoritative
  host job record.
- Define bounded wake-latency and false-stall metrics without workflow identity
  leakage.
- Consider relative-delay ergonomics only inside a future host adapter.
- Add another wake-source family only through a separate source-specific plan.

## 15. Recommended Next Phase

Perform a focused planning blocker fix. Update the plan to:

- choose the private absolute UTC scheduling hint for the first slice;
- add the Core-owned non-mutating readiness assessment;
- make `NotYetEligible` the only benign early-wake result;
- keep the atomic transition as the only satisfaction proof;
- clarify cancellation and duplicate callback races; and
- extend the future test plan accordingly.

After the blocker fix is reviewed and accepted, implement and review the two
pre-scheduler reinvocation proofs. Scheduler code remains deferred until both
sets of gates pass.

## 16. Governed Review Record

- workflow: `dg/review`
- run: `run-1791070395291821000-2`
- approval: `approval/run-1791070395291821000-2/review-scope-approved`
- presentation: `presentation/fee7736f2edbfe9b`
- approval outcome: granted by delegated maintainer through proof enforcement
- reviewed plan commit: `c0cc2a5`
- approved boundary: focused review only; no runtime implementation
- phase status: `Completed`
- event summary: 39 events, including one approval request, one approval
  grant, eight policy decisions, six scheduled steps, six successful skill
  invocations, zero retries, and zero escalations; approval-presentation proof
  enforced with one matching record and event marker
- validation summary: documentation and diff checks passed
- out-of-kernel work: source inspection, security analysis, review authoring,
  documentation validation, and later git and pull-request actions
