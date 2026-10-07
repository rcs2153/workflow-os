# Private Trusted-Host Repeated Scheduling Plan Report

## 1. Executive Summary

Planning is complete for the smallest private repeated scheduling boundary
after acceptance of the schedule-once helper. The proposed future driver is
bounded, local SQLite, single-window, and crate-private. It composes the
accepted schedule-once helper for every wake instead of duplicating authority
or transition logic.

No runtime behavior is implemented by this phase.

## 2. Scope Completed

- Defined the false-stall gap remaining after one-shot scheduling.
- Defined a finite wake-budget model.
- Defined fresh per-wake operation and receipt identity requirements.
- Defined the bounded repeated scheduling algorithm.
- Defined early-wake re-arm, cancellation, shutdown, restart, concurrency,
  unsupported-wait, privacy, and observability posture.
- Defined focused future tests and a small implementation sequence.
- Updated the roadmap to identify focused plan review as the next phase.

## 3. Scope Explicitly Not Completed

This phase does not implement repeated scheduling, a production timer, a
scheduler daemon, queue, worker pool, host-job persistence, automatic model
turns, automatic approval, public API, CLI, SDK, schema, workflow config,
provider mutation, OpenShell, nested harnesses, hosted scheduling, additional
wake families, writes, or release changes.

## 4. Proposed Boundary

The future driver remains private and accepts one immutable operational
locator, one injected deadline waiter, one executor, immutable skill input,
one redispatch identity provider, a new per-wake operation/receipt identity
provider, and a bounded wake budget.

It does not discover runs, inspect repositories, coordinate multiple windows,
or retain authority. Every iteration calls the accepted schedule-once helper,
which rehydrates current Core state and retains all accepted trusted-time,
authority, context, transition, replay, and dispatch gates.

## 5. Repetition And Stop Semantics

Repetition is allowed only after:

- a benign early wake that returns fresh inert scheduling posture; or
- a reinvoked executor attempt that lawfully yields to another exact
  `TimeWindow`.

The driver stops on terminal, blocked, cancellation, host failure, unsupported
wait, stale/security/corrupt/ambiguous posture, identity failure, or wake-budget
exhaustion. It never polls Core and never retries a fail-closed result.

## 6. Authority And Safety Summary

- The wake budget is a resource bound, not authority.
- Every wake receives fresh operation and receipt identities.
- Every next ticket and handoff is freshly derived by schedule-once.
- Timer wake remains a request for Core verification.
- Current authority and required context remain checked at operational entry.
- Competing drivers rely on accepted one-winner transitions and atomic dispatch
  reservation rather than host coordination.
- No ticket, callback, outcome, or restart state grants execution authority.

## 7. Restart And Cancellation Summary

The future driver persists nothing. Process restart begins a new bounded call
from durable Core state. Cancellation and shutdown return explicit host-level
posture without mutating or completing the workflow. Durable host jobs,
leases, and recovery daemons remain deferred.

## 8. Privacy Summary

The planned models carry typed identities, closed enums, and bounded counters
only. Debug and errors must redact bindings and omit deadlines, paths,
commands, prompts, payloads, credentials, source values, and provider data.
No public or serialized scheduling surface is authorized.

## 9. Future Test Summary

The plan requires proofs for sequential time waits, early-wake re-arm and
budget exhaustion, cancellation, host failure, unsupported waits, unique
per-wake identities, restart, stale-state rejection, competing drivers,
checked counters, redaction, and absence of public or external behavior.

## 10. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 11. Remaining Limitations

- No repeated driver exists yet.
- No production timer implementation exists.
- Only exact local SQLite `TimeWindow` waits are planned.
- No process-level automatic agent continuation is claimed.
- No public configuration or hosted parity is planned in this slice.
- Genuine approval, evidence, external-event, capability, conflict, check, and
  authority-refresh waits remain unscheduled.

## 12. Recommended Next Phase

Perform focused maintainer/security review of the private repeated scheduling
plan. Verify the finite wake budget, fresh per-wake identities, no-polling
algorithm, early-wake re-arm boundary, cancellation and restart posture,
one-winner concurrency reliance, privacy, and strict non-goals.

Do not implement the driver during that review.

## 13. Governed Planning Record

- workflow: `dg/d`
- run: `run-1791355794329596000-2`
- approval: `approval/run-1791355794329596000-2/planning-approved`
- presentation: `presentation/205f347fce1def90`
- presentation hash:
  `205f347fce1def90f49d16cf8db57fd444844f1f15216c95f6dbd7e1e980e48c`
- approval outcome: granted by delegated maintainer through proof enforcement
- approved boundary: planning and documentation only
- phase status: `Completed`
- event summary: 39 events; one approval; zero retries; zero escalations;
  approval-presentation proof enforced with one persisted presentation record
- validation summary: documentation and diff checks passed
- out-of-kernel work: source inspection, architecture analysis, plan and report
  authoring, validation, and later git and pull-request actions
