# Hosted Partial-Run Creation Recovery Plan

Status: Complete; exact-head CI and clean-environment acceptance passed

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

## 10. Final Acceptance

Renewed exact-head CI and an independent clean-environment rerun are complete.
The first exact-head run completed 24 of 25 concurrent
creates and exposed a raw retryable conflict from the final post-dispatch
rehydration read. The blocker correction extends the bounded hosted-admission
contract to that idempotent read and adds stage-specific exhaustion coverage.
Focused maintainer/security review otherwise accepted the retry contract after
adding per-run replay assertions and stage-specific exhaustion tests. The
restore gate is closed: an interrupted
`RunCreated`-only prefix survived backup and restore, repaired once, and
replayed without another append. The corrected contention fixture carries all
25 simultaneous distinct requests to lawful queued posture, while a forced
persistent serialization fault proves bounded exhaustion, stage-appropriate
partial state, exact recovery, and idempotent replay.

The amended exact head passed all nine required CI jobs. A disposable clone of
the merged commit then passed the 25-job and exhaustion milestone, backup and
restore rehearsal, and a real PostgreSQL process restart followed by another
full milestone. See the [acceptance rerun](../concepts/HOSTED_PARTIAL_RUN_CREATION_ACCEPTANCE_RERUN.md),
[focused review](../concepts/HOSTED_ADMISSION_CONTENTION_BLOCKER_FIX_REVIEW.md),
and [final acceptance](../concepts/HOSTED_PARTIAL_RUN_CREATION_FINAL_ACCEPTANCE.md).

## 11. Remaining Pilot Gate: Conflicting Legacy Histories

The accepted automatic repair applies only to semantically matching history.
An operator disposition is a separate decision, not a missing creation event
or a replacement for the original run. For cases the operator API does
not classify, record dispositions externally as described in the
[operator runbook](../runtime/single-tenant-hosted-alpha.md#conflicting-creation-history).

For a typed, grammar-valid history that diverges from the expected creation
prefix, Core records one create-only observation per run, observed history
revision and expected prefix revision. It records the first mismatch and
database observation time, not raw events or an operator decision. The
existing conflict response still stops creation. Other invalid histories
remain fail-closed and are not classified by this observation path.

Core storage accepts investigation and administrative abandonment against
an exact conflict observation. Decisions and their current pointer are
written atomically outside the run history; replacing a decision requires
its current identity. Exact replay returns the original historical record,
not a claim that it is still current. The trusted caller must authenticate
and authorize the actor. The opt-in single-tenant operator API provides that
boundary using a distinct deployment credential and actor. It does not
confirm effects; continue to use the operator runbook for external evidence.

The runtime slice must bind an authenticated, authorized operator decision to
the original run, immutable request and exact observed history revision.
Persist the decision separately from workflow events, with decision time,
reason, bounded evidence references and follow-up ownership. Accept exact
decision replay without duplicating the record; reject stale history, changed
decision content under the same decision identity and evidence from another
run. If a later review supersedes a decision, retain the prior record and link
the replacement. Missing or contradictory evidence must remain unresolved
unless the operator explicitly abandons further investigation, which makes no
claim about whether the effect happened.

Prove the boundary through an authenticated hosted API and disposable
PostgreSQL fixture seeded with conflicting legacy history. Check that an exact
create retry returns the original conflict before and after disposition, that
history remains readable without repairing its projection, and that recording
and replaying the decision never changes events, approval, work items,
invocations, effects or reports. Include authorization, stale revision,
cross-run evidence, concurrent decision and restart tests. A completed
disposition does not admit the pilot or authorize new work.

[PostgreSQL CI for PR #955](https://github.com/rcs2153/workflow-os/actions/runs/43446700911/job/131957907789)
verified the grammar-valid fixture through the operator API, including
restart, unchanged execution records and visibility of stale evidence.
The grammar-invalid inspection fixture is distinct: reading its events does
not create a conflict observation or permit a disposition.
[PostgreSQL CI for PR #957](https://github.com/rcs2153/workflow-os/actions/runs/43451657709/job/131975071001)
verified the modeled missing-schedule case. The fixture preserves the
original HTTP request identity, models a policy event in place of the
missing schedule, and records an investigation without repairing the
history, producing a report, or dispatching work. It does not use the
original evaluator's database or certify production legacy histories.

### Independent Pilot Rerun

An evaluator other than the implementer or operator should use a clean
checkout of the candidate revision, an empty disposable PostgreSQL database,
and no live provider configuration. Build the hosted binary from that
checkout and run the same rehearsal as the Shared PostgreSQL State CI job:

```sh
cargo build --locked -p workflow-hosted --bin workflow-os-hosted
WORKFLOW_OS_RESULT_REHEARSAL_DATABASE_URL="$DISPOSABLE_DATABASE_URL" \
  node scripts/rehearse-hosted-result-interruption.mjs
```

Record the exact commit, PostgreSQL version, and sanitized success markers
for contention, exact retry, restart replay, current-decision freshness,
invalid-history inspection, and missing-schedule disposition. Verify the
fixture's unchanged history, absent report and work item, and idle worker
assertions. Keep any deviation open with its observed diagnostic rather
than declaring a pass from CI alone. Record the evaluator and verdict
separately from the maintainer evidence. No production access, repair,
redispatch, or pilot admission follows from this rerun.
