# Private Trusted-Host Production Timer Integration Plan

Status: implemented pending focused maintainer/security review. The
crate-private bounded repeated scheduling driver and private synchronous local
timer primitive are implemented. No explicit production caller, automatic
scheduling service, or public runtime integration is implemented.

## 1. Executive Summary

Workflow OS can now compose a finite sequence of exact `TimeWindow` waits
through the accepted schedule-once boundary, but only through an injected test
waiter. The next slice should add one real local host timer that blocks the
caller's thread until an absolute deadline or explicit cancellation, then
delegates all governance decisions back to the accepted repeated driver.

The timer is not a scheduler. It does not discover runs, persist jobs, create
threads, grant authority, approve work, evaluate wait satisfaction, or invoke
an executor directly. Core remains the only component that may prove current
eligibility and admit execution.

This plan does not implement anything.

## 2. Goals

- Add one crate-private production implementation of the accepted deadline
  waiter contract.
- Support absolute UTC deadline waiting and prompt explicit cancellation.
- Integrate that waiter only through the accepted bounded repeated driver.
- Keep the caller's thread as the execution owner.
- Reassess trusted time and current durable state after every host wake.
- Preserve finite wake budget, fresh identity, at-most-once entry, and
  fail-closed behavior.
- Define restart, duplicate signal, clock movement, and failure semantics.
- Keep inputs, Debug output, and errors bounded and non-leaking.

## 3. Non-Goals

This phase must not add:

- implementation in the planning prompt;
- a daemon, service, queue, worker pool, or general scheduler;
- detached threads or background run discovery;
- a durable host-job or timer store;
- automatic process restart or run resumption;
- public API, CLI, SDK, schema, or workflow configuration;
- automatic model turns or automatic approval;
- provider reads or mutations;
- OpenShell execution;
- nested harness execution;
- hosted or distributed scheduling;
- PostgreSQL scheduling parity;
- filesystem report or artifact output;
- release posture changes.

## 4. Architectural Boundary

The future flow should be:

```text
explicit local caller
  -> private production timer integration
  -> accepted bounded repeated scheduling driver
  -> accepted schedule-once helper
  -> Core-owned current observation and readiness assessment
  -> accepted reinvocation and executor-admission boundary
```

The host timer owns only blocking and cancellation. The repeated driver owns
only finite orchestration. Core owns trusted-time validation, wait identity,
current disposition, transition, authority consumption, dispatch reservation,
and executor admission.

## 5. Placement

Add a new private sibling module under `workflow-core` SQLite state, likely:

```text
crates/workflow-core/src/sqlite_state/trusted_host_local_timer.rs
```

The module should remain unexported outside `workflow-core`. It may use the
crate-private `TrustedHostDeadlineWaiter` and repeated-driver contracts without
widening visibility.

Do not place this first slice in the CLI or hosted crate. The CLI would create
a premature user contract, while the hosted crate would imply distributed
scheduling that this phase cannot support.

## 6. Candidate Private Model

Use the smallest private types needed, such as:

- `TrustedHostLocalTimerCancellation`;
- `TrustedHostLocalTimerCancellationHandle`;
- `TrustedHostLocalDeadlineWaiter`;
- `TrustedHostLocalTimerInput`; and
- `run_trusted_host_local_timer`.

Names may follow final repository conventions. None should derive serde or be
publicly exported.

The timer input should contain only:

- the accepted repeated scheduling input fields;
- a validated finite wake budget;
- one private cancellation receiver or shared cancellation state; and
- optional bounded, non-authorizing host correlation for diagnostics if a
  concrete need is proven during implementation.

The caller must continue to supply the executor, skill input, redispatch
identity provider, and fresh wake-identity provider explicitly. The timer must
not invent execution identity or authority.

## 7. Waiting Primitive

Use a local `Mutex` plus `Condvar` cancellation primitive rather than periodic
sleep polling.

For each supplied absolute deadline, the waiter should:

1. check cancellation under the shared lock;
2. read the current UTC time through the production clock boundary;
3. return `Woke` immediately if the deadline is already elapsed;
4. compute one bounded remaining duration;
5. wait on the condition variable with timeout;
6. return `Canceled` if cancellation was signaled;
7. recompute remaining time after a spurious wake; and
8. return `Woke` after the deadline timeout.

The internal loop handles operating-system spurious wakeups only. It must not
read Core state or poll workflow disposition.

Poisoned synchronization state or invalid time conversion should map to the
accepted fixed `Failed` waiter posture. The implementation should not expose
panic text, timestamps, paths, or synchronization details.

## 8. Cancellation Semantics

Cancellation must be explicit and idempotent:

- cancellation before `wait_until` returns `Canceled` without sleeping;
- cancellation during a wait wakes the condition variable promptly;
- repeated cancellation signals are harmless;
- cancellation after the waiter returns has no retrospective effect;
- cancellation does not mutate Core state;
- cancellation does not fail or complete the workflow; and
- a later explicit caller may reconstruct a new bounded run from durable
  state.

Dropping a handle must not silently grant continuation. The first slice may
keep cancellation false when all handles are dropped, because the caller's
thread remains the explicit owner. A later background-service design would
need stronger owner-loss semantics.

## 9. Clock And Deadline Semantics

The timer consumes the inert absolute UTC deadline produced by Core. It may
use the system clock only to determine how long the host should block. It must
not claim that the wait is satisfied.

After every return from the timer, the accepted schedule-once helper reads the
Core-owned trusted clock and reassesses readiness. Therefore:

- a forward clock movement may wake early from the host's perspective, but
  Core still decides eligibility;
- a backward clock movement may cause another bounded host wait;
- a spurious operating-system wake is absorbed by the waiter or becomes a
  bounded fresh reassessment;
- clock disagreement cannot create authority; and
- wake-budget exhaustion remains a non-terminal host stop.

The implementation must use checked duration conversion and reject impossible
or oversized durations with fixed non-leaking errors.

## 10. Integration Function

`run_trusted_host_local_timer` should be an explicit, synchronous,
crate-private call. It should:

1. construct the production deadline waiter from the supplied cancellation
   state;
2. pass that waiter and all explicit inputs to
   `run_bounded_trusted_host_repeated_scheduling`;
3. return the repeated driver's existing bounded outcome unchanged; and
4. add no second loop around the repeated driver.

The wrapper must not append workflow events, write state directly, reinterpret
workflow status, invoke a handler itself, or catch security/corruption errors
for retry.

## 11. Identity Posture

Wake and redispatch identity providers remain explicit inputs in the first
production timer slice. This is deliberate:

- timer mechanics must not become an identity minting authority;
- the accepted driver already requests a fresh pair for every attempt;
- duplicate or conflicting identity remains fail-closed in Core; and
- production identity generation needs its own collision and restart proof if
  it is later standardized.

Before production timer implementation is accepted, add a regression test
showing that a duplicate wake-identity provider cannot enter the executor
twice or advance a different wait under reused identity.

## 12. Restart And Recovery

The timer persists nothing. Process exit loses the in-memory wait and
cancellation state. Restart requires an explicit caller to construct a new
timer integration from:

- the durable operational locator;
- a fresh finite wake budget;
- fresh identity providers; and
- the same immutable execution inputs required by the accepted boundary.

Core then rehydrates current state. No prior ticket, handoff, timer duration,
callback, capability, or remaining budget may be trusted across restart.

Add a direct reconstruction test at the integrated timer boundary. Do not add
automatic run discovery or startup scanning in this phase.

## 13. Concurrency And Duplicate Signals

Multiple host callers may race only because the API remains an internal
function, not because the phase provides a coordinator. Safety remains in the
accepted Core one-winner transitions.

Before acceptance, add a direct competing integration test that proves:

- two independently constructed timer integrations may wake for one window;
- aggregate executor entry remains at most one for the governed attempt;
- the losing caller receives a bounded stop or fail-closed error; and
- neither caller receives reusable authority.

Repeated cancellation notifications and spurious condition-variable signals
must not create duplicate Core transitions.

## 14. Failure Semantics

The integration should preserve existing semantics:

- cancellation -> typed `Canceled` stop;
- wake budget exhausted -> typed non-terminal stop;
- blocked or terminal Core posture -> typed stop;
- unsupported wait -> typed stop;
- synchronization failure -> stable waiter failure error;
- stale state, trusted-time rejection, corruption, ambiguity, replay failure,
  or identity conflict -> immediate structured error;
- no automatic retry for any error; and
- no conversion of host failure into workflow failure or completion.

Use typed private classifications where practical rather than expanding the
current internal error-code matching seam.

## 15. Correlation And Observability

The first slice should return existing count and stop posture only. It must not
persist timer jobs or emit new workflow events merely because the host slept.

If implementation needs local diagnostics, allow only a bounded private
correlation value that is redacted in `Debug` and absent from serialization.
Do not record deadlines, paths, commands, payloads, credentials, or skill
outputs.

Durable host-job correlation, scheduling metrics, and operator-visible timer
status require separate planning because they create a new state family and
retention boundary.

## 16. Privacy And Redaction

The production timer must not store or expose:

- workflow inputs or skill outputs;
- prompts or model content;
- source contents;
- raw commands or command output;
- paths or environment values;
- provider payloads;
- credentials or authorization material;
- exact identifiers in `Debug`; or
- raw synchronization or panic details.

Private model Debug output should expose only bounded posture, counts, and
booleans. Errors must use stable codes and fixed messages.

## 17. Test Plan

Future focused tests must cover:

1. elapsed deadline returns immediately;
2. future deadline wakes after timeout;
3. cancellation before wait returns immediately;
4. cancellation during wait wakes promptly;
5. repeated cancellation is idempotent;
6. spurious notification does not claim eligibility;
7. synchronization poison maps to fixed non-leaking failure;
8. invalid duration conversion fails closed;
9. early host wake is reassessed by Core;
10. two sequential Core time waits complete through one integrated call;
11. budget exhaustion remains non-terminal;
12. duplicate wake identity fails closed;
13. restart reconstructs from durable state without retained authority;
14. competing integrations yield aggregate at-most-once executor entry;
15. cancellation, host failure, and security failure do not retry;
16. no direct Core state mutation by the timer;
17. no event, artifact, filesystem, provider, CLI, or schema behavior;
18. bounded Debug and error output; and
19. existing schedule-once, repeated-driver, continuity, executor, and
    workspace suites remain green.

Real-time tests should use generous bounds and remain few. Most behavior
should use an injected private clock or synchronization seam. The production
`Condvar` path still needs at least one focused real-time smoke test.

## 18. Implementation Sequence

1. Add private cancellation state and handle.
2. Add the private local deadline waiter with `Condvar` semantics.
3. Add the explicit synchronous integration wrapper around the accepted
   repeated driver.
4. Add deterministic unit tests plus one bounded real-time smoke test.
5. Add direct duplicate-identity, restart, and competing-integration proofs.
6. Update the roadmap and create an implementation report.
7. Perform focused maintainer/security review.
8. Only after acceptance, plan an explicit private caller integration. Do not
   expose public scheduling in that phase by default.

## 19. Open Questions

- Should owner-handle loss cancel a future background timer, or remain a
  no-op while execution is synchronous?
- What private clock seam preserves production behavior without letting tests
  replace Core's trusted-time decision?
- Should synchronization poison map to `Unavailable` or `Failed` under the
  existing waiter taxonomy?
- Is one optional private host correlation identifier useful enough to justify
  adding it before durable timer jobs exist?
- Which explicit private caller should own this synchronous wait after the
  timer itself is accepted?

## 20. Final Recommendation

After focused plan review, implement only the crate-private synchronous local
timer, explicit cancellation primitive, repeated-driver wrapper, and focused
proofs described here. Keep wake identities injected and keep all authority in
Core.

Do not build a scheduler service, daemon, background discovery loop, public
configuration, automatic approval, model-turn automation, provider mutation,
OpenShell execution, nested harness runtime, hosted scheduling, CLI, SDK,
schema, or release changes.

## 21. Governed Planning Record

- workflow: `dg/d`
- run: `run-1791358837134650000-2`
- approval: `approval/run-1791358837134650000-2/planning-approved`
- presentation: `presentation/4cfed9373d561517`
- presentation hash:
  `4cfed9373d5615171445cc49e5630df134933a08baedda6014781d7f7ac76a58`
- approval outcome: granted by delegated maintainer through proof enforcement
- approved boundary: private production trusted-host timer planning only
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations;
  approval presentation proof marker present
- validation summary: documentation and diff checks passed
- out-of-kernel work: architecture inspection, plan authoring, documentation
  validation, and later git and pull-request actions
