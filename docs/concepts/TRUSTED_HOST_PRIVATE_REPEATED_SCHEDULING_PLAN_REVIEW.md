# Private Trusted-Host Repeated Scheduling Plan Review

## 1. Executive Verdict

**Plan accepted with non-blocking implementation follow-ups. Proceed to the
crate-private bounded repeated scheduling driver.**

The plan closes the remaining local false-stall gap without turning Workflow
OS into a general scheduler. It composes the accepted schedule-once helper for
every wake, bounds repetition explicitly, and leaves authority, trusted time,
wait transition, current context, and executor admission in their existing
Core-owned boundaries.

## 2. Scope Verification

The plan stays within the approved planning-only scope. It defines:

- one local SQLite operational-window driver;
- one finite wake budget;
- one fresh operation/receipt identity pair per attempted wake;
- repeated composition through the existing schedule-once helper;
- explicit early-wake, cancellation, shutdown, restart, concurrency,
  unsupported-wait, privacy, and test posture; and
- a focused implementation and review sequence.

It does not authorize implementation in the planning phase, a general
scheduler, public API, CLI, SDK, schema, runtime config, automatic model turns,
automatic approval, provider mutation, OpenShell, nested harnesses, hosted or
distributed scheduling, PostgreSQL parity, writes, or release changes.

## 3. Product And Architecture Assessment

The boundary matches Workflow OS's role as the governing kernel. The host may
wait and call back, but it does not decide that a wait is satisfied or that
execution is authorized. Core still determines every material continuation
outcome from durable current state.

The proposed driver is intentionally narrower than a workflow scheduler. It
handles one exact local operational window and only exact `TimeWindow` waits.
It neither discovers runs nor coordinates unrelated work.

## 4. Wake-Budget Assessment

Finite construction is the key liveness and resource control. Counting every
waiter call, including early wakes and cancellation, prevents a waiter that
returns immediately from creating an unbounded CPU loop. A small private
maximum prevents a caller from turning the driver into an effectively
unbounded service.

Budget exhaustion correctly remains a host-level non-terminal stop. It does
not fail or complete the workflow and grants no authority.

The exact private maximum can be selected during implementation and tested at
zero, one, the maximum, and above the maximum. It does not need another
planning phase.

## 5. Fresh-State And Identity Assessment

Every iteration calls the accepted schedule-once helper, so the next ticket,
handoff, wait identity, trusted-time binding, and disposition are derived from
fresh authoritative state. Prior scheduling material is not reused.

Fresh operation and receipt identities are required for every wake attempt.
The identity provider cannot inspect Core state or grant authority. An unused
identity after an ineligible observation or canceled host wait is harmless and
must not be treated as a durable operation.

The implementation should preserve the plan's ordering: identity-provider
failure must occur before the corresponding waiter call, and lower-boundary
security or replay failure must never be retried automatically.

## 6. Re-Arm And Non-Polling Assessment

The only direct re-arm path without executor entry is a benign early wake.
That path must discard prior inert scheduling posture and invoke schedule-once
again to obtain a fresh observation. Repeated immediate wakes consume budget
and stop explicitly.

An executor outcome with `AwaitCondition` may also proceed to another
iteration, but schedule-once must classify the current condition again. A
non-`TimeWindow`, multi-wait, blocked, terminal, stale, corrupt, or ambiguous
posture stops the driver.

This is event/deadline-driven repetition, not Core polling. The driver does
not need a direct disposition polling loop.

## 7. Cancellation And Failure Assessment

Cancellation and host shutdown remain explicit waiter outcomes. They stop the
driver without mutating, failing, or completing the workflow. A later process
may start a new bounded driver from durable state with fresh identities.

The accepted schedule-once helper currently represents waiter unavailability
and failure as structured errors. The first repeated implementation should
propagate those errors without retry rather than add a new result taxonomy.
Changing that taxonomy may be considered later, but is not required for this
slice.

## 8. Restart And Recovery Assessment

The plan correctly persists no ticket, handoff, callback, remaining budget,
capability, or authority. Restart starts a new bounded call and rehydrates
current Core state. Existing transition replay and operational-entry recovery
remain responsible for an already-committed wake.

Resetting a non-authoritative host wake budget after process restart is an
explicit limitation, not an authority bypass. Durable scheduler jobs or
budgets require separate planning.

## 9. Concurrency And Idempotency Assessment

The plan does not introduce host leader election or leases. Competing drivers
remain safe because exact wait transition, directive consumption, and dispatch
reservation are already one-winner Core operations.

The future test must measure aggregate executor entries across competing
drivers, not merely require one specific loser error. A loser may lawfully
observe bounded terminal or blocked posture after the winner advances state.

## 10. Privacy And Error Assessment

The planned input and outcome surfaces are closed, private, and count-oriented.
Debug and error rules prohibit identifiers, deadlines, paths, commands,
prompts, payloads, credentials, source values, and provider data. No serde or
public inspection surface is introduced.

The plan correctly keeps any skill output under the accepted lower redaction
posture. The smallest implementation may omit skill output entirely if the
driver's callers do not need it.

## 11. Test Plan Assessment

The proposed tests cover the required implementation risk:

- sequential exact time waits in one driver call;
- finite early-wake re-arm and budget exhaustion;
- cancellation, unavailable, and failed waiter posture;
- unsupported conditions without waiter entry;
- unique identities and identity-provider failure ordering;
- restart without persisted authority;
- stale authority, context, wait, trusted time, and locator rejection;
- competing-driver one-winner behavior;
- checked counters and budget bounds;
- Debug/error redaction; and
- absence of public, provider, hosted, and write behavior.

The implementation report should state the selected private maximum and show
that no test relies on real wall-clock sleeps where an injected waiter can
provide deterministic behavior.

## 12. Blockers

None.

## 13. Non-Blocking Follow-Ups

- Select and document the smallest private maximum wake count during
  implementation.
- Preserve schedule-once waiter unavailable/failed outcomes as structured
  non-retried errors in the first slice.
- Omit retained skill output unless a concrete private caller needs it.
- Keep production timer integration and durable host-job correlation in
  separate reviewed phases.

## 14. Recommended Next Phase

Implement the crate-private bounded repeated scheduling driver exactly as
planned. Add only the private wake-budget and wake-identity models, the driver
composition, focused tests, roadmap update, and implementation report.

Do not add a production timer, general scheduler, public configuration,
automatic model turns, automatic approval, another wake family, provider
execution, OpenShell, nested harnesses, hosted scheduling, schemas, CLI, SDK,
writes, or release changes.

## 15. Governed Review Record

- workflow: `dg/review`
- run: `run-1791356235950940000-2`
- approval:
  `approval/run-1791356235950940000-2/review-scope-approved`
- presentation: `presentation/f71168c1d43b9c88`
- presentation hash:
  `f71168c1d43b9c8857448d8aedd003a2009b9fc4f61ca018c1152973b0dbf868`
- approval outcome: granted by delegated maintainer through proof enforcement
- reviewed merge commit: `48c34fb1d61633370f944eb453ea728c97003ce5`
- approved boundary: focused maintainer/security review only
- phase status: `Completed`
- event summary: 39 events; one approval; zero retries; zero escalations;
  approval-presentation proof enforced with one persisted presentation record
- validation summary: documentation and diff checks passed
- out-of-kernel work: source and plan inspection, security analysis, review
  authoring, validation, and later git and pull-request actions
