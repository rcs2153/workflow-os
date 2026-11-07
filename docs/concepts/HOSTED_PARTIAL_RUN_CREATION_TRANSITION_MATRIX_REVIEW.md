# Hosted Partial-Run Creation Transition Matrix Review

## Verdict

Accepted after four boundary corrections. The exact-main reproduction is
credible and sufficient to authorize one bounded Core/PostgreSQL recovery
implementation phase.

The implementation may repair only valid prefixes of the exact immutable
hosted creation request. It may not infer completion from snapshot status,
invent events from current projections, or broaden hosted/provider behavior.

## Findings And Corrections

### 1. The Four-Event Matrix Is A Fixture, Not The General Grammar

The reproduced `postgres/build` fixture emits `RunCreated`, `RunValidated`,
`RunStarted`, and `StepScheduled`. The general `append_run_start` path may also
emit `GovernanceAssessmentBound` and
`GovernanceDisclosureSurfaceAccepted` before validation.

The reproduction report now distinguishes its observed sequence from the
general creation grammar. Implementation must derive the complete expected
sequence from the exact prepared plan and immutable inputs, then classify by
semantic prefix equality. Hard-coded event counts or sequence numbers are not
acceptable.

### 2. Stable Event Identity Needs A Concrete Recipe

The current `EventBuilder` generates a new event ID and current timestamp on
every call. Reusing it during recovery would not satisfy the plan's requirement
to rederive the same missing transition.

The implementation must introduce a bounded deterministic bootstrap-event
factory for this hosted creation path. IDs, timestamps, sequence positions,
correlation, actor, event payloads, and idempotency identity must derive from
the exact immutable creation inputs. An existing event is accepted only when
its complete authoritative identity and semantic payload match the expected
event at that prefix position.

This requirement is limited to creation-prefix recovery. It does not authorize
a repository-wide event-identity migration.

### 3. Serialization Must Cover Classification And Repair

A lease acquired before independent ordinary appends would still expose
intermediate prefixes and would not stop an unfenced append path from racing.

The first implementation should add one PostgreSQL-backed operation that:

1. enters a serializable transaction;
2. acquires a transaction-scoped lock for the exact run;
3. reads and validates the complete durable event prefix and immutable bundle;
4. appends all missing deterministic creation events in that transaction;
5. commits the corresponding authoritative snapshot projection; and
6. returns an explicit classification plus the events actually appended.

The existing dispatch transaction remains the sole queue/work-item seam. A
failure between repaired creation-prefix commit and dispatch is safe only when
the next exact retry observes the complete scheduled prefix and enters that
already-reviewed dispatch reconciliation.

### 4. Durable Events And Runtime Signals Must Stay Distinct

The ordinary executor appends an event, then emits audit, observability, and
structured-log signals. A batch repair transaction cannot claim those external
signals were atomically delivered with event persistence.

The recovery operation must return newly committed events for post-commit
signal emission. If signal emission fails, durable history remains authority;
exact retry must not duplicate events. Tests and reports must disclose the
signal-delivery posture rather than representing a signal as durable event
proof.

## Accepted Implementation Boundary

The next implementation phase is authorized to change only the hosted
PostgreSQL creation and exact-retry path needed to:

- classify the exact durable prefix;
- deterministically construct the expected bootstrap events;
- repair a valid missing suffix under one transaction-scoped lock;
- project the repaired snapshot;
- emit post-commit runtime signals without duplicate history;
- continue through the existing approval or dispatch seam; and
- expose bounded repaired, existing, conflict, and recovery-required outcomes.

The implementation must preserve:

- immutable bundle and resolved-context equality;
- actor, correlation, project/resource, policy, and current-fact bindings;
- create-only event semantics;
- exact second-retry idempotency;
- the existing approval and queue transaction boundaries; and
- payload-free external diagnostics.

## Required Permanent Evidence

Before acceptance, tests must prove:

- every possible creation-prefix interruption, including optional governance
  binding events;
- exact retry and second-retry behavior;
- concurrent exact retries with one transition winner;
- changed immutable inputs fail before new work;
- restart and PostgreSQL backup/restore recovery;
- post-commit signal failure does not duplicate durable events;
- no duplicate work item, invocation, approval, SideEffect, receipt, or report;
- hosted API mappings and inspectable recovery posture; and
- the existing at-least-25-job contention case.

Focused tests must inspect event payloads and relational rows, not only status
codes or projections.

## Governed Review

- workflow: `dg/review`
- run: `run-1794010920702092000-2`
- approval: `approval/run-1794010920702092000-2/review-scope-approved`
- presentation: `presentation/a59acde488251b78`
- approval outcome: granted under delegated-maintainer authority
- production changes: none

## Non-Goals

This acceptance does not authorize:

- persisted schema changes unless a later blocker proves them necessary;
- a generic repair API or caller-authored events;
- provider mutation broadening;
- nested harness or agent-team execution;
- hosted production-readiness or universal exactly-once claims;
- release or publication; or
- Governed Obligation Plan runtime composition.

## Next Authorized Phase

Implement the bounded deterministic creation-prefix reconciliation seam and
its permanent PostgreSQL/Core tests. Stop for a new focused review if the work
requires schema changes, changes to generic event semantics, or a second
dispatch path.
