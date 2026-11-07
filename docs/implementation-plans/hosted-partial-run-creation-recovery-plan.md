# Hosted Partial-Run Creation Recovery Plan

Status: P0 planning complete; implementation not started

## 1. Decision

Workflow OS must repair the hosted creation state machine before further
private obligation-plan composition, provider-mutation broadening,
nested-harness work, or broader hosted claims.

The already-closed **Accepted Means Dispatched** gate proves recovery when a
known durable run prefix exists but queue insertion failed. New external fault
injection describes an earlier gap: failures while appending the creation
prefix can leave a run that the API exposes but an exact retry cannot advance
to an approval wait, queued work, or terminal report. This is a distinct P0
continuity defect, not a reason to invalidate the accepted queue-insert repair.

The implementation must remain Core-owned, transactionally fenced, and bound
to the exact immutable request. The caller must not reconstruct or repair run
state by guessing from HTTP status, current projections, or partial event
counts.

## 2. Evidence To Reproduce

Before changing production behavior, reproduce the report on exact current
`main` in an isolated PostgreSQL and hosted-service environment.

The supplied evidence reports:

- one contention trial completed 25 distinct jobs;
- two later trials stopped when an exact retry returned HTTP `409`;
- seven event-write boundaries were faulted independently; and
- at three boundaries, before validation, start, or step scheduling, removing
  the fault and retrying twice still produced no queued job and no report.

This is externally supplied evidence until Workflow OS preserves its own exact
fixture, commands, event prefixes, database rows, API responses, and retry
results. The reproduction must distinguish a product defect from test-harness,
schema, migration, or request-identity error.

## 3. Required Invariant

For one exact immutable hosted create request, every durable prefix must have
one deterministic disposition:

1. it is an unaccepted attempt with no governed run identity;
2. it is a valid repairable prefix that exact retry can advance exactly once;
3. it is a lawful approval wait;
4. it has exactly one durable queued work item;
5. it is terminal with its exactly bound report posture; or
6. it is corrupt, conflicting, or ambiguous and fails closed with a durable,
   inspectable recovery requirement.

No accepted API response may describe a run that lacks a lawful continuation
or explicit terminal/wait posture. No exact retry may duplicate an event, work
item, invocation attempt, approval decision, SideEffect, receipt, or report.

## 4. State-Machine Reconciliation

The first implementation phase must create an explicit matrix over every
creation boundary, including at minimum:

| Durable boundary | Valid next posture | Exact-retry responsibility |
| --- | --- | --- |
| no run record | create from the immutable request | reserve once or reject conflict |
| `RunCreated` only | validated prefix | verify immutable identity and append missing transition once |
| through `RunValidated` | started prefix | append the exact missing start once |
| through `RunStarted` | scheduled prefix | append the exact missing schedule once |
| through `StepScheduled` without approval/work | approval wait or dispatch | derive the same authoritative route and commit once |
| lawful approval wait | unchanged wait | return the exact pending approval without redispatch |
| dispatchable prefix without work item | queued work | atomically repair through the accepted dispatch seam |
| queued/running/terminal | existing posture | reconcile without duplicate work |
| invalid or conflicting prefix | none | fail closed with bounded diagnostics |

The matrix must use actual event and persistence invariants. Event count alone
is not authority. A prefix is repairable only when its immutable bundle,
request identity, project scope, current authority obligations, event ordering,
snapshot posture, and any approval facts agree with the exact retry.

## 5. Implementation Boundary

The implementation should prefer one Core-owned recovery decision and one
transactional transition seam over API-specific repair logic.

Required properties:

- exact-request idempotency and immutable-bundle equality;
- database-time locking or equivalent serialization before repair;
- rehydration and semantic validation of the complete durable prefix;
- current-authority reassessment wherever the existing route requires it;
- atomic append of only the missing lawful transitions;
- deterministic rederivation of every missing transition from the exact
  immutable request, with stable event and idempotency identities rather than
  caller-selected or projection-inferred events;
- reuse of the accepted queue/work-item transaction rather than a second
  dispatch mechanism;
- explicit outcome vocabulary for existing wait, repaired dispatch, existing
  dispatch, terminal replay, conflict, and unrecoverable corruption;
- bounded, payload-free diagnostics and telemetry; and
- no success response until a lawful recoverable posture is durable.

The HTTP layer may map these outcomes, but it must not become the owner of
recovery semantics.

Recovery must not turn a current snapshot into invented history. If the exact
missing transition cannot be rederived and create-only appended with the same
identity and causal inputs the original route would have used, the prefix is
not automatically repairable and must fail closed for bounded operator
resolution.

## 6. Fault And Concurrency Tests

Permanent tests must inject failure before or after every durable transition
used by hosted create, including:

- immutable run reservation and bundle persistence;
- `RunCreated`;
- `RunValidated`;
- `RunStarted`;
- `StepScheduled`;
- approval-request persistence when applicable;
- invocation projection;
- queue/work-item insertion; and
- the final response boundary.

For every boundary, tests must prove:

- removal of the fault plus exact retry reaches one lawful posture;
- a second exact retry is idempotent;
- changed workflow, project, resource, actor, policy, approval, or execution
  context fails closed;
- no duplicate event, queue item, invocation, approval, SideEffect, receipt, or
  report exists;
- restart and a new API process do not change the result;
- concurrent exact retries elect one transition winner; and
- conflicting retries do not consume or weaken the exact request's authority.

Repeat the contention test at or above the externally reported 25-job case.
Exercise PostgreSQL backup/restore over at least one interrupted prefix. Tests
must inspect events and relational state, not only HTTP status.

## 7. API And Operator Experience

An exact retry must either complete recovery or return a specific bounded
diagnostic that explains why recovery is unsafe. A generic `409` is not enough
when the server owns a valid partial run.

Inspection should show:

- the current durable prefix classification;
- whether repair is available, completed, waiting, or denied;
- the exact immutable request identity used for comparison;
- the current queue, approval, invocation, and report posture; and
- the lawful next action, if any.

This does not authorize arbitrary repair commands or hand-edited state.

## 8. Acceptance

This P0 closes only when:

- every valid early creation prefix is repaired or lawfully classified;
- every invalid prefix fails closed before new work;
- API exact retry no longer strands the reproduced cases;
- fault-by-fault, concurrency, restart, and restore tests pass;
- focused maintainer/security review accepts the transition matrix and code;
- exact-head required CI is terminal green; and
- an independent clean-environment rerun reproduces the original failures on
  the parent and verifies the corrections on the accepted commit.

## 9. Non-Goals

This plan does not add:

- a new workflow language or state store;
- caller-authored repair state;
- arbitrary event skipping or synthesis;
- provider-mutation broadening;
- multi-tenancy or production hosted claims;
- nested harness execution, agent teams, or recursive agents;
- new release or publication authority;
- obligation-plan runtime composition; or
- exactly-once claims beyond the explicitly tested Core-owned identity and
  idempotency boundary.

## 10. Next Phase

The next phase is an independent reproduction and exact state-transition
matrix over current code. Production implementation begins only after focused
review accepts that matrix and names the single Core-owned recovery seam.
