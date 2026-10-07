# Private Trusted-Host Scheduling Boundary Plan

Status: accepted plan with the first two private implementation slices
complete. The coherent observation slice is accepted by focused
maintainer/security review; the schedule-once helper awaits focused review.
The crate-private coherent scheduling observation, inert absolute-UTC ticket,
non-mutating `TimeWindow` readiness assessment, injected deadline-wait
interface, and one-shot host composition are implemented. No repeated
scheduler, timer driver, background worker, automatic model turn, public
configuration, or hosted runtime is implemented.

## 1. Executive Summary

Workflow OS can durably register one exact `TimeWindow` wait, project an inert
opaque handoff, verify the exact deadline against trusted time, transition the
wait atomically, and explicitly re-enter the accepted local execution path.
The host must still decide when to call that explicit operation.

The next boundary should be one private, injected, schedule-once composition.
Core derives a bounded inert scheduling ticket from the same authoritative
wait state as the handoff. A trusted host registers that ticket with an
injected deadline waiter without polling. When the waiter wakes, the host
invokes the accepted source-specific reinvocation exactly once. Core treats
the wake as a request for fresh verification, never as proof that the deadline
elapsed or authority remains valid.

This boundary addresses false stalls for already-authorized local work without
turning Workflow OS into a model-turn scheduler or a general orchestration
service. Genuine waits, blocks, security rejections, and terminal posture
remain explicit Core-derived outcomes.

Implementation may not begin until the accepted explicit reinvocation helper
has both a full-composition concurrent-caller proof and a
transition-to-entry crash-recovery proof.

## 2. Current Accepted Foundation

The future boundary may compose, but must not weaken:

- exact durable `TimeWindow` dependency binding;
- one coherent read-only opaque wait handoff observation;
- source-specific trusted-time verification inside Core;
- handoff-bound, domain-separated transition replay;
- one-use resume-directive consumption;
- atomic one-winner dispatch reservation;
- current-authority and required-context reassessment at entry;
- bounded trusted-host redispatch while Core derives `ResumeNow`; and
- typed `AwaitCondition`, `Blocked`, and `Terminal` outcomes.

The accepted handoff is orientation and stale-detection input only. It does
not expose the deadline, reconstruct authority, satisfy a wait, reserve a
dispatch, or invoke an executor.

## 3. Problem Statement

The explicit reinvocation helper closes the security-sensitive composition
between an inert handoff, exact wait transition, and operational entry. It
does not close the liveness gap between observing a future `TimeWindow` and
calling that helper at an appropriate time.

A host can currently fill that gap only with ad hoc behavior such as sleeping,
polling, retaining unbounded state, or depending on a human or conversation to
restart work. That behavior is outside the kernel contract and can create:

- false stalls even though a lawful future action is already known;
- busy polling against authoritative state;
- stale handoff reuse;
- duplicated wake attempts;
- host-authored deadline satisfaction;
- accidental treatment of a timer callback as authority; or
- misleading completion when the executor merely yielded.

The safe next step is not a general scheduler. It is a narrow contract that
lets an injected host wait once for one Core-derived `TimeWindow` deadline and
then request one fresh source-specific reinvocation.

## 4. Goals

- Define one private local schedule-once boundary for one exact registered
  `TimeWindow` wait.
- Derive scheduling posture from authoritative Core state rather than caller
  prose, cached workflow state, or a model decision.
- Wait without busy polling through an injected host deadline-wait interface.
- Keep the scheduling ticket and opaque handoff inert and non-authoritative.
- Invoke the accepted `reinvoke_after_time_window_wait` helper at most once per
  admitted scheduled wake.
- Recheck exact wait binding, trusted time, current authority, required
  context, and dispatch eligibility after wake.
- Preserve genuine wait, blocked, terminal, corrupt, ambiguous, and security
  posture without requiring a manual conversational restart.
- Support restart through fresh authoritative rehydration, not serialized
  capabilities or caller memory.
- Keep errors, Debug output, receipts, and future host telemetry bounded and
  non-leaking.

## 5. Non-Goals

This plan does not authorize:

- implementation during this planning phase;
- a scheduler daemon, queue, worker pool, polling loop, wake bus, or hosted
  control plane;
- repeated scheduling or an unbounded host run loop;
- automatic model, assistant, agent, or conversation turns;
- automatic approval, agent self-approval, evidence bypass, check bypass, or
  authority inferred from delegated prose;
- approval, evidence, check, external-event, capability, conflict, or
  authority-refresh wake sources;
- provider execution or mutation, OpenShell, sandbox lifecycle, or live
  connectors;
- nested harness execution, recursive agents, or agent swarms;
- public Rust APIs, runtime configuration, workflow fields, CLI, SDK, schema,
  UI, or examples;
- filesystem, PostgreSQL, multi-host, lease, heartbeat, failover, or
  distributed scheduling parity;
- persistent bearer capabilities, serializable authority, or capability
  reconstruction;
- provider writes, additional mutation families, release changes, or
  production-readiness claims.

## 6. Required Pre-Implementation Proofs

Scheduler implementation is blocked until focused tests prove both:

1. two callers racing through `reinvoke_after_time_window_wait` admit at most
   one executor entry and leave explainable durable state for the loser; and
2. a fault after committed wait transition but before operational entry can be
   recovered by exact replay after backend reopen without duplicate executor
   entry or reconstructed authority.

These proofs belong to the existing explicit reinvocation composition. They
must land and receive focused maintainer/security review before any scheduling
type or interface is implemented.

## 7. Source-Of-Truth Boundaries

| Concern | Source of truth | Host must not substitute |
| --- | --- | --- |
| Current wait | Authoritative SQLite continuity state | Cached handoff alone |
| Deadline identity | Exact durable `TimeWindow` dependency binding | Caller-selected timestamp |
| Wake eligibility | Same-call trusted-time verifier | Timer callback or wall clock |
| Continue/wait/block/stop | Fresh Core continuation disposition | Host or model judgment |
| Execution authority | Fresh one-use Core capability | Ticket, handoff, receipt, or callback |
| Executor admission | Atomic dispatch reservation | Scheduled job identity |
| Workflow completion | Durable terminal transition | Successful wake or executor return |
| Restart posture | Fresh state rehydration | Serialized in-memory capability |

## 8. Candidate Private Model

After the prerequisite proofs pass, the smallest implementation should
consider crate-private types adjacent to the existing handoff and
reinvocation modules:

- `TrustedHostTimeWindowScheduleTicket`
  - payload-free ticket commitment;
  - the exact private absolute UTC scheduling instant derived from the durable
    deadline;
  - trusted-time source category and epoch commitment where required;
  - handoff commitment; and
  - closed next operation: `wait_once_then_request_fresh_verification`.
- `TrustedHostScheduleOnceInput`
  - backend and immutable operational locator;
  - current inert handoff and matching schedule ticket;
  - exact reinvocation identities and immutable executor binding;
  - injected deadline waiter, executor, and identity provider.
- `TrustedHostDeadlineWaiter`
  - one injected call that accepts only the bounded schedule instant;
  - returns `Woke`, `Canceled`, or a structured host error;
  - cannot inspect authoritative state or receive authority.
- `TrustedHostScheduledReinvocationOutcome`
  - accepted explicit reinvocation outcome;
  - refreshed inert wait posture when no transition occurs;
  - blocked or terminal posture; or
  - structured error for corruption, ambiguity, security rejection, or host
    failure.

Names are provisional. No model should be public, serializable, cloneable as
authority, or added to workflow schemas in the first slice.

## 9. Scheduling Ticket Semantics

The opaque handoff intentionally does not expose a deadline. Scheduling needs
one additional private inert projection rather than a new handoff accessor or
caller-authored timestamp.

Core should derive the ticket and handoff from one coherent authoritative
observation. The ticket must bind:

- the same workflow, run, step, window, generation, and condition posture as
  the handoff through payload-free commitments;
- the exact `TimeWindow` dependency commitment;
- the trusted-time source category and epoch rules needed to interpret the
  scheduling hint; and
- one bounded scheduling instant or delay suitable for an injected host.

The scheduling instant is a liveness hint, not evidence that the wait is
satisfied. Copying, replaying, delaying, or firing the ticket early cannot
authorize transition or entry. The accepted source-specific verifier performs
the authoritative check after wake.

The first slice uses an absolute UTC instant because the accepted durable
deadline and `CoreInjectedClockV1` observation share that domain. A host timer
API may compute a process-local relative delay, but that delay is neither
persisted nor returned to Core and cannot satisfy the wait. A different
trusted-time source or clock domain requires a separate reviewed contract.

The ticket must not expose prompts, commands, source contents, raw provider
payloads, credentials, approval reasons, evidence bodies, check output, or
private authority material.

## 10. Core-Owned Readiness Assessment

The accepted transition records an unelapsed deadline as a security
rejection. Normal host timer jitter must not enter that mutation boundary.
Before reinvocation, Core therefore needs one private, non-mutating
source-specific readiness assessment.

The assessment must:

1. load the current authoritative wait and exact dependency binding;
2. obtain a fresh trusted-time observation through the accepted Core-owned
   source;
3. validate source, provenance, epoch, condition, generation, revision,
   window, and handoff/ticket commitments;
4. return `Eligible`, `NotYetEligible`, or a structured security/corruption
   error;
5. write no operation, security rejection, event, directive, reservation,
   receipt, or workflow transition; and
6. grant no wake, authority, or dispatch capability.

`NotYetEligible` is the only benign early-wake result. It returns refreshed
inert scheduling posture with zero executor entry. `Eligible` permits the host
to call the accepted explicit reinvocation helper once, but it is still not
proof of satisfaction. The transition transaction obtains and checks trusted
time again atomically. A clock or state change between readiness and transition
therefore fails closed inside the accepted mutation boundary.

## 11. Schedule-Once Algorithm

The future private helper should follow this closed sequence:

1. Open one coherent read transaction and derive the current wait observation,
   inert handoff, and matching schedule ticket.
2. Reject missing, stale, unsupported, multi-wait, blocked, terminal, corrupt,
   or non-`TimeWindow` posture before registering a host wait.
3. Give the injected deadline waiter only the bounded scheduling instant.
4. Wait once without polling Core or mutating workflow state.
5. On cancellation or host failure, return a structured non-terminal outcome;
   do not transition the wait or fabricate workflow failure.
6. On wake, call the Core-owned readiness assessment exactly once.
7. For `NotYetEligible`, derive refreshed inert scheduling posture and return
   without mutation, automatic re-arming, or executor entry.
8. For `Eligible`, call `reinvoke_after_time_window_wait` exactly once with the
   original inert handoff and exact immutable invocation binding.
9. Let Core freshly verify handoff commitment, trusted time, exact dependency,
   replay posture, authority, required context, directive, and dispatch.
10. Return the accepted execution, refreshed wait, blocked, terminal, or
   structured error posture.

The helper must not loop, re-arm itself, sleep again, or call another wake
family in the first slice.

## 12. Early, Late, Spurious, And Duplicate Wakes

- An early or spurious host wake is not deadline satisfaction. The readiness
  assessment returns `NotYetEligible`, writes no durable rejection, and leaves
  the wait unsatisfied with zero executor entry.
- A late wake is acceptable only if the exact wait and immutable bindings are
  still current when Core verifies them.
- Duplicate host callbacks may race, but the prerequisite full-composition
  concurrency proof and existing one-winner transitions must admit at most one
  executor entry.
- A callback after cancellation, blocking, terminal transition, authority
  change, or handoff replacement must fail closed or return current bounded
  posture without executor entry.
- The host must not automatically retry a security rejection, corrupt state,
  or ambiguous outcome.

Cancellation before wake returns a host-level canceled outcome with no Core
mutation. Cancellation racing after `Eligible` cannot revoke a transition that
Core has already committed. The first helper does not add host retry,
automatic re-arming, or cancellation-driven workflow mutation.

## 13. Restart And Recovery

The first implementation may remain local SQLite and single host, but it must
be restart-safe.

No private capability may be persisted by the host. After restart, the host
must reopen SQLite and request a fresh authoritative scheduling observation.
Core may then:

- return a new inert handoff and schedule ticket for the same unsatisfied
  exact wait;
- recover an exactly committed transition through durable replay;
- classify a currently blocked or terminal run; or
- reject stale or corrupt state.

The host may persist or reconstruct non-authoritative job correlation only in
a later separately reviewed phase. The first slice should prove restart by
recreating the injected host around durable Core state, not by adding a new
scheduler store.

## 14. Genuine Wait And False-Stall Posture

The boundary must distinguish:

- `ResumeNow`: existing bounded redispatch continues inside the accepted
  operational entry path;
- exact future `TimeWindow`: the host may wait once using the inert ticket;
- early or refreshed `TimeWindow`: return wait posture for fresh scheduling
  classification;
- `Blocked`: stop and preserve the blocker without approval fabrication;
- `Terminal`: stop permanently for this operation; and
- ambiguity or corruption: fail closed for operator resolution.

An assistant response ending, a host callback returning, or a process restart
is not workflow completion. Conversely, this boundary does not guarantee that
the Codex desktop, ChatGPT, or another conversational product will create a
new model turn. It governs a local injected executor process only.

## 15. Security And Privacy

- A schedule ticket and callback are never authority.
- Only Core may obtain trusted time and create the private wake capability.
- Exact handoff and ticket commitments must be checked before fresh mutation.
- Security rejection must not expose current disposition as an oracle.
- Debug and error text must omit IDs, timestamps where sensitive, paths,
  commands, prompts, payloads, credentials, and secret-like test markers.
- Durable records should add only payload-free commitments if a new binding is
  required.
- The deadline waiter receives no workflow state, executor input, capability,
  approval context, or provider data.

## 16. Test Plan

Before scheduler implementation:

1. full-composition concurrent reinvocation admits at most one executor entry;
2. transition-to-entry crash recovery survives backend reopen without a
   duplicate entry.

For the later private scheduling slice, focused tests must prove:

3. a future exact deadline registers one injected wait and performs no Core
   polling;
4. one elapsed wake requests one source-specific reinvocation;
5. early wake returns `NotYetEligible`, writes no operation or security
   rejection, and leaves the wait unsatisfied with zero executor entries;
6. late wake succeeds only while exact bindings remain current;
7. canceled host wait preserves non-terminal workflow posture;
8. host failure does not fabricate workflow failure or completion;
9. duplicate callbacks admit at most one transition and executor entry;
10. stale handoff or ticket substitution fails before fresh mutation;
11. authority or required-context change after scheduling prevents entry;
12. blocked and terminal changes after scheduling prevent entry;
13. restart derives a fresh ticket from authoritative state without persisted
    capability;
14. exact replay after ambiguous transition remains domain-separated from
    direct wake;
15. one scheduled call never loops or re-arms itself;
16. `Eligible` grants no authority and a clock or state change before the
    transition is rejected by the transaction's fresh trusted-time check;
17. wrong source, provenance, epoch, dependency, or wait revision fails closed
    in readiness assessment;
18. Debug and errors do not leak schedule, binding, or test-secret values;
19. no public API, serde, CLI, SDK, schema, provider, OpenShell, nested-harness,
    hosted, or write behavior appears; and
20. existing continuity, wait, reinvocation, operational-entry, runtime,
    adapter, report, and workspace tests remain green.

## 17. Proposed Implementation Sequence

1. Implement the full-composition concurrent-caller proof in the existing
   reinvocation test boundary.
2. Implement transition-to-entry crash fault injection and recovery proof.
3. Perform focused maintainer/security review of those prerequisite proofs.
4. Add the crate-private coherent scheduling observation, absolute UTC inert
   ticket, and non-mutating readiness assessment. **Implemented.**
5. Add one injected deadline-wait interface and schedule-once helper.
   **Implemented.**
6. Add early, late, cancellation, duplicate, restart, binding, privacy, and
   non-polling tests. **Implemented for the schedule-once boundary.**
7. Create an implementation report. **Implemented.**
8. Perform focused maintainer/security review. **Next phase.**
9. Only after acceptance, consider repeated private scheduling, another wake
   family, or a public host integration as separate phases.

## 18. Open Questions

- Should early wake return a refreshed scheduling ticket immediately or force
  a separate fresh observation call?
- What minimum fault hook proves the transition-to-entry crash seam without
  adding production test controls?
- Does a future repeated scheduler need a durable non-authoritative job record,
  or can it reconstruct all local jobs from authoritative waits after restart?
- Which metrics can disclose false stalls, wake latency, duplicate callbacks,
  and scheduler failures without leaking workflow identities or payloads?

## 19. Final Recommendation

Perform focused maintainer/security review of the implemented crate-private
local SQLite schedule-once boundary, injected deadline waiter, and inert
Core-derived ticket composition.

Do not implement a scheduler daemon, repeated scheduling, automatic model
turns, automatic approval, provider or sandbox execution, OpenShell, nested
harnesses, public configuration, CLI, SDK, schemas, hosted execution, provider
writes, or release changes.
