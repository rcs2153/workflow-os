# Authorized Execution Continuity Event And State Projection Plan Review

## 1. Executive Verdict

Needs planning blocker fixes.

The plan selects the correct next P0 boundary and preserves the essential rule
that continuity state grants authority while runtime events disclose committed
facts. It is not implementation-ready because several atomicity, cursor,
replay, and compatibility decisions remain underspecified or conflict with the
current storage contracts.

## 2. Scope Verification

The plan stays within planning scope. It does not authorize runtime code,
SQLite schema changes, a supervisor, redispatch, automatic approval, executor
wiring, provider mutations, CLI behavior, workflow schema exposure, nested
harness execution, or release changes.

## 3. Existing-System Assessment

The plan correctly identifies a real split boundary:

- `AuthorizedExecutionContinuityStore` operations own continuity transactions;
- `EventLogStore::append_event` owns a separate SQLite event transaction;
- `RunSnapshotStore::save_snapshot` owns a third independent write; and
- the current SQLite event append validates history but does not derive or save
  the run snapshot.

The next implementation must therefore compose existing transaction bodies
under one write transaction and harden all competing snapshot writers. Merely
adding a new helper around the three existing public methods cannot provide the
claimed invariant.

## 4. Accepted Planning Decisions

The following decisions are sound:

- continuity state remains the execution-authority source of truth;
- runtime events are bounded operational history and never bearer authority;
- snapshots remain derived projections rather than caller-authored truth;
- exact replay must not append a duplicate event;
- commit ambiguity requires fresh-connection reconciliation;
- filesystem and PostgreSQL remain unsupported;
- no supervisor or external execution belongs in this phase; and
- privacy excludes payloads, transcripts, commands, provider data, secrets,
  and capability material.

## 5. Blocker One: Global Event/Snapshot Atomicity Is Not Closed

The proposed projected-operation transaction can write an event and snapshot
together, but the current generic paths remain able to append events and save
snapshots independently. In particular, `save_snapshot` is an unconditional
upsert with no event-cursor compare-and-swap. A stale caller can overwrite a
newer snapshot after a projected operation commits.

The corrected plan must choose and specify one enforceable posture:

- make every SQLite event append derive and persist its snapshot atomically and
  reject independent stale snapshot writes; or
- treat snapshots as explicitly non-authoritative caches, add monotonic
  cursor-checked writes, and define deterministic repair from event history.

The implementation cannot claim event/snapshot atomicity while an existing
public store method can clobber the result.

## 6. Blocker Two: Pre-Operation And Post-Operation Cursor Semantics Conflict

Continuity requests and records already bind `ContinuityCursor` to an
`EventSequenceNumber` and `EventId`. Public window, yield, and wait vocabulary
also carries event identities. The plan proposes that the backend allocate a
new event and resulting cursor but does not define:

- whether request cursor means the last event before mutation or the event
  representing the mutation;
- which continuity records advance to the new cursor;
- how wait identities seeded during yield bind to the newly allocated event;
- how later operations prove they observed the projected cursor; or
- how exact replay remains valid after lawful later events and revisions.

The correction must define separate expected-input and committed-result
cursors. Each operation family needs an exact record-by-record cursor write
set. Replay must validate the historical event and lawful successor lineage,
not require the current snapshot to equal the historical result cursor.

## 7. Blocker Three: Committed Security Rejections Are Missing

Semantic V2 can durably commit a security-rejection operation result. The plan
says every committed operation has exactly one event, but the candidate event
vocabulary describes only ordinary five-family success outcomes.

The correction must decide whether committed security rejections receive a
bounded event. The recommended answer is yes: one closed event payload should
distinguish `applied` from `security_rejected`, contain only a stable rejection
class, and preserve the fact that rejection grants no authority and changes no
workflow status. Exact rejection replay must return the same event binding
without a duplicate event.

If rejection events are intentionally excluded, the invariant and acceptance
criteria must be narrowed consistently. The current mixed posture is unsafe.

## 8. Blocker Four: Runtime Transition Matrix Is Undefined

Adding event variants requires an exact `StateTransition` posture. The plan
does not state from which `WorkflowRunStatus` values each event may be appended,
whether any event changes run status, or how continuity terminal disposition
relates to `RunCompleted`, `RunFailed`, and `RunCanceled`.

The correction must provide a closed matrix. Continuity events should normally
be status-preserving and must never independently mark the run terminal.
Events after an already terminal workflow run must remain forbidden unless a
separately justified audit-only stream is introduced, which is out of scope.
The matrix must also define whether `WaitingForApproval`,
`WaitingForExternalEvent`, and `Escalated` can lawfully receive each outcome.

## 9. Blocker Five: Transaction Reuse Is Not Implementable As Written

The current SQLite continuity methods each acquire a connection and begin
their own immediate transaction. The generic event append does the same. The
new outer transaction cannot safely call those methods without nested or split
transactions.

The correction must require private transaction-scoped operation functions
that accept `&Transaction`, reuse the existing semantic V2 functions, and are
called by both the existing continuity API and the new projected API. Tests
must prove the refactor leaves all accepted standalone continuity semantics
unchanged. The plan must prohibit copying or reimplementing the five semantic
state machines in the projection layer.

## 10. Blocker Six: Projection Commitment And Schema Version Are Unresolved

The plan requires a `resulting snapshot commitment` but does not define its
canonical material, algorithm, version, or whether the commitment covers the
event history prefix, serialized snapshot, or both. This makes relational
verification and replay ambiguous.

The plan also leaves schema versioning as an open question even though a new
durable projection-binding table and constraints necessarily change the
adapter schema. The correction must select an explicit additive SQLite adapter
schema V3, define upgrade and old-writer behavior, and define a domain-separated
projection commitment over canonical bounded fields. These are implementation
prerequisites, not review-time options.

## 11. Error And Privacy Assessment

The proposed privacy boundary is appropriate. The correction must additionally
ensure that corruption errors do not expose event IDs, operation IDs, database
paths, SQL, serialized rows, or commitments. Stable error precedence must be
specified for simultaneous cursor, revision, replay, trusted-time, and
projection-binding failures.

No raw provider payload, source/spec content, command output, environment
value, credential, token, transcript, or capability material is authorized.

## 12. Test Assessment

The planned matrix is strong but must add explicit tests for:

- stale independent snapshot overwrite rejection;
- generic event append racing a projected continuity operation;
- exact rejection-event replay;
- each operation family's expected-input to committed-result cursor write set;
- lawful later event and revision successors during historical replay;
- all runtime-status/event combinations in the closed transition matrix;
- transaction-scoped semantic reuse parity with standalone continuity APIs;
- canonical projection-commitment determinism and domain separation; and
- adapter schema V2-to-V3 interruption, old-writer rejection, and restore
  posture.

## 13. Documentation Assessment

The roadmap correctly positions projection before supervisor work, but it must
state that focused review found blockers. The plan status must not imply
implementation readiness until the six blockers above are corrected and
re-reviewed.

## 14. Validation

- `npm run check:docs`: passed;
- `git diff --check`: passed;
- current runtime event, snapshot, generic state-store, continuity-state, and
  SQLite transaction boundaries: inspected directly.

## 15. Governed Review Record

- workflow: `dg/review`;
- run: `run-1786867308633056000-2`;
- approval:
  `approval/run-1786867308633056000-2/review-scope-approved`;
- presentation: `presentation/ab7fe3c3a0770d05`;
- presentation hash:
  `ab7fe3c3a0770d05b6b7497454140bebd959fbdd42fb6dbe60d4264226654f19`;
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted review handoff was presented and assessed;
- governed status: completed; and
- out-of-kernel work: source inspection, review writing, and validation were
  performed by the external executor. The kernel did not edit files, execute
  tests, schedule an agent, or mutate a provider.

## 16. Blockers

1. Close global event/snapshot atomicity and stale-writer behavior.
2. Define expected-input and committed-result cursor semantics per operation.
3. Include or consistently exclude committed security-rejection events.
4. Define the exact runtime status-transition matrix.
5. Require transaction-scoped reuse of accepted continuity semantics.
6. Resolve canonical projection commitment and SQLite schema V3 now.

## 17. Non-Blocking Follow-Ups

- Decide whether operator inspection later needs a concise continuity event
  renderer.
- Consider report citations only after event projection is accepted.
- Keep supervisor scheduling policy separate from event projection.

## 18. Recommended Next Phase

Correct the plan blockers, then perform focused maintainer/security re-review.
Do not begin implementation or the injected-supervisor vertical slice until
the corrected plan is accepted.

Fix-forward: the six blockers are addressed in [Authorized Execution
Continuity Event And State Projection Plan Blocker Fix
Report](AUTHORIZED_EXECUTION_CONTINUITY_EVENT_STATE_PROJECTION_PLAN_BLOCKER_FIX_REPORT.md).
The original verdict remains the historical review result until focused
re-review accepts the corrected plan.
