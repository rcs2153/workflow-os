# Authorized Execution Continuity Event And State Projection Plan Report

## 1. Executive Summary

The next P0 continuity phase is now planned. It binds each accepted SQLite
continuity mutation to one bounded runtime event and derived run-snapshot
projection in the same transaction. Planning does not implement runtime
projection or scheduling.

## 2. Scope Completed

- Defined the source-of-truth boundary among continuity state, events, and
  snapshots.
- Proposed bounded event vocabulary for all five continuity operations.
- Defined a private operation, receipt, event, and snapshot projection binding.
- Defined one atomic capability boundary rather than independent state/event
  calls.
- Defined SQLite transaction, replay, commit reconciliation, compatibility,
  privacy, migration, and test requirements.
- Positioned one injected trusted-host supervisor vertical slice after focused
  review and acceptance of this projection phase.

## 3. Scope Explicitly Not Completed

No runtime event type, snapshot field, projection store, SQLite schema change,
executor wiring, supervisor, redispatch, automatic approval, tool execution,
provider mutation, CLI, workflow schema, nested harness runtime, or release
posture change is implemented.

## 4. Key Planning Decision

Continuity state remains the source of execution authority. Runtime events are
bounded operational history, and snapshots remain deterministic event
projections. One transaction must commit all three representations plus their
exact binding, or none of them.

## 5. Replay And Recovery Posture

Exact replay returns the original continuity result and projection binding
without a second event. Commit ambiguity is reconciled on a fresh connection.
Partial or conflicting durable presence is corruption and fails closed.

## 6. Privacy Posture

Only validated IDs, enums, revisions, counts, timestamps, and commitments may
cross the projection boundary. Raw work, source, spec, command, provider,
environment, credential, token, and bearer-authority material remains
forbidden.

## 7. Governed Planning Record

- workflow: `dg/d`;
- run: `run-1786867019502784000-2`;
- approval: `approval/run-1786867019502784000-2/planning-approved`;
- presentation: `presentation/a1d8eac838b4621f`;
- presentation hash:
  `a1d8eac838b4621fadcdea9c48a41af9829e822c69e4f920eb4768c49bc890bb`;
- approval outcome: granted under standing delegated-maintainer authority
  after the complete persisted planning handoff was presented and assessed;
- governed status: completed with 39 events, 1 approval, 0 retries, and 0
  escalations; and
- out-of-kernel work: repository inspection, plan authoring, and validation
  were performed by the external executor. The kernel did not edit files, run
  commands, schedule an agent, or mutate a provider.

## 8. Validation

- `npm run check:docs`: passed;
- `git diff --check`: passed.

## 9. Remaining Limitations

- Focused maintainer/security review found six planning blockers in global
  event/snapshot atomicity, cursor semantics, committed security-rejection
  projection, runtime transitions, transaction-scoped semantic reuse, and
  commitment/schema decisions.
- Continuity mutations and runtime event/snapshot projection remain separate.
- No trusted-host supervisor exists.
- SQLite is scoped local support, not default or production-certified.
- Filesystem and PostgreSQL remain unsupported for continuity semantic V2.

## 10. Recommended Next Phase

Correct the planning blockers and perform focused maintainer/security
re-review. Do not implement the atomic projection contract or local supervisor
until the corrected plan is accepted.
