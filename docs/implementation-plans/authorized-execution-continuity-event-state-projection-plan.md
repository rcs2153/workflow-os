# Authorized Execution Continuity Event And State Projection Plan

Status: accepted after focused maintainer and security blocker-fix review

## 1. Executive Summary

Workflow OS now has an accepted scoped semantic V2 continuity backend for
embedded SQLite. The backend atomically preserves execution windows, executor
yields, typed waits, one-winner resume directives, started attempts, terminal
attempt outcomes, ambiguity recovery, trusted time, replay commitments, and
operation receipts. Runtime events and run snapshots remain a separate state
contract.

The next P0 phase must bind those two truths. One accepted continuity mutation
must commit its bounded runtime event and run-snapshot projection in the same
SQLite transaction. A continuity receipt without its event, or an event that
claims a continuity transition that did not commit, is forbidden. Exact replay
must return the original projection binding without appending another event.

This plan defines that atomic projection boundary. It does not implement a host
supervisor, redispatch an agent, open windows automatically, approve gates,
execute tools, broaden provider mutations, or expose new workflow or CLI
schema.

## 2. Goals

- Define a bounded continuity runtime-event vocabulary for all five accepted
  continuity operation families.
- Make continuity mutation, operation receipt, event append, and snapshot
  projection one atomic SQLite commit.
- Preserve the existing event stream as the operational-history source of
  truth and continuity records as the execution-authority source of truth.
- Bind every committed continuity operation to exactly one event identity,
  sequence number, resulting snapshot cursor, and operation receipt.
- Preserve exact replay without duplicate events or snapshot advancement.
- Reject same-key/different-content replay and stale event cursors before any
  write.
- Define post-commit reconciliation that returns the original result after an
  ambiguous acknowledgement.
- Keep stored and emitted data bounded, payload-free, and redaction-safe.
- Provide one backend-parametric projection conformance suite and durable
  same-path SQLite restart proof.
- Preserve all existing runtime, executor, approval, report, SideEffect,
  provider, migration, and continuity behavior.

## 3. Non-Goals

This phase does not implement:

- a trusted-host supervisor, scheduler, executor redispatch, or model-turn
  creation;
- automatic creation, extension, revocation, or supersession of execution
  windows;
- automatic gate approval, delegated self-approval, evidence satisfaction, or
  policy inference;
- tool execution, local command execution, external effects, or another
  provider mutation family;
- filesystem or PostgreSQL continuity support;
- default SQLite selection or production certification;
- workflow-declared continuity configuration, public schema, SDK, CLI, or
  report rendering changes;
- nested harness execution, agent teams, hosted runtime, or distributed
  coordination;
- exactly-once external execution; or
- Reasoning Lineage or claim-graph implementation.

## 4. Source-Of-Truth Boundary

The implementation must preserve separate authoritative domains:

- continuity state decides whether lawful work is runnable, awaiting a typed
  condition, blocked, recovery-required, or terminal;
- the workflow event stream records ordered operational history;
- the workflow run snapshot is a deterministic projection of that event
  stream; and
- an external executor or final assistant response cannot alter any of those
  states by assertion.

A projected runtime event describes a committed continuity fact. It does not
grant authority, carry a bearer capability, satisfy evidence, or itself make a
run runnable. Readers needing the current continuation disposition must query
the continuity store rather than infer authority from event history.

## 5. Closed Event Vocabulary

Add one top-level `AuthorizedExecutionContinuityProjected` runtime event with a
closed, versioned payload. The payload contains:

- the continuity operation kind;
- an outcome discriminator of `applied` or `security_rejected`;
- for `applied`, one operation-specific result enum covering registered yield,
  transitioned wait, consumed directive plus started attempt, ordinary attempt
  outcome, or ambiguity recovery;
- for `security_rejected`, only the accepted stable committed rejection class;
- the operation ID, receipt ID, projection commitment, expected-input cursor,
  and committed-result cursor; and
- bounded target identity and revision fields required to verify the exact
  operation target.

Both applied and durably committed security-rejection operations receive
exactly one event. A rejection event grants no authority, changes no workflow
status, and carries no rejected input value. Exact rejection replay returns the
same event binding without appending another event.

Each event should carry only validated IDs, revisions, posture enums, bounded
counts, trusted timestamps, and commitments already accepted by the continuity
contract. It must not carry raw work summaries, prompts, transcripts, command
output, provider payloads, paths, environment values, credentials, capability
secrets, or arbitrary failure text.

The five operation outcomes and the two committed dispositions remain
distinguishable and stable inside the closed payload. New operation kinds or
dispositions require an explicit contract version rather than an open string.

## 6. Projection Binding Model

Introduce a private validated projection binding containing:

- continuity operation kind and operation ID;
- canonical request commitment;
- committed operation receipt identity;
- workflow and run identity;
- execution window identity and relevant yield, wait, directive, or attempt
  identity;
- event ID and event sequence number;
- expected-input cursor and committed-result cursor;
- resulting continuity revision or exact target revision; and
- resulting snapshot commitment.

The expected-input cursor is the exact runtime event cursor observed before the
continuity operation. The committed-result cursor is the newly appended
projection event. They must be distinct and contiguous. The binding is
backend-owned. Callers may supply bounded operation inputs but must not choose
the result event identity or sequence. Event ID allocation and sequence
advancement occur while holding the same SQLite write transaction that
validates the input cursor.

Per-operation cursor write sets are closed:

- register-yield validates the request and attempt against the input cursor,
  creates the yield and seeded waits against the result cursor, and advances
  the owning window cursor to the result cursor;
- transition-wait validates the condition and window at the input cursor,
  writes the resulting wait revision at the result cursor, and advances the
  owning window cursor;
- consume-directive validates the directive, waits, authority, and window at
  the input cursor, writes the consumed directive and started attempt at the
  result cursor, clears active-yield ownership, and advances the window cursor;
- record-attempt-outcome validates the attempt capability and window at the
  input cursor, writes the attempt and closed-window result at the result
  cursor; and
- recover-ambiguous-attempt validates the attempt and window at the input
  cursor, writes recovery-required attempt/window state at the result cursor.

A committed security rejection leaves domain target state unchanged except for
the already accepted trusted-time/security bookkeeping, but its operation
record binds the input cursor and its rejection event becomes the result
cursor. The owning window cursor advances only when the accepted semantic V2
rejection rules identify that successor as lawful.

The projection binding is not authority. It contains no capability material
and cannot be consumed to execute work.

## 7. Atomic API Boundary

Add a separate internal capability, tentatively
`AuthorizedExecutionContinuityProjectionStore`, rather than widening the
existing generic `EventLogStore::append_event` contract.

The API should expose one typed operation per continuity mutation, or one
closed request enum, and return the existing continuity result plus its exact
projection binding. It must:

1. validate backend support and live-instance eligibility;
2. load and validate the immutable run identity and current event cursor;
3. execute the accepted semantic V2 continuity transition;
4. allocate and append the bounded runtime event;
5. deterministically rehydrate and write the resulting run snapshot;
6. persist the operation-to-event projection binding; and
7. commit all writes together.

No executor-facing convenience wrapper may compose a continuity mutation with
`append_event` and `save_snapshot` as independent calls.

The implementation must extract private transaction-scoped forms of all five
continuity operations and event append/rehydration. These functions accept one
existing `&Transaction` and reuse the accepted semantic V2 functions. Existing
standalone continuity methods call the same transaction-scoped functions in
their own transaction. The projection layer must not copy or reimplement the
five state machines.

## 8. SQLite Transaction Contract

For the accepted SQLite backend, every projected operation uses one
`BEGIN IMMEDIATE` transaction against the same database that owns continuity,
events, and snapshots.

Within that transaction the implementation must:

- verify schema version, live-instance eligibility, trusted-time posture, and
  immutable run identity;
- verify the expected event cursor and continuity revisions;
- apply existing deterministic continuity semantic functions;
- allocate the next contiguous event sequence and globally unique event ID;
- append the event and validate the complete ordered run history;
- derive the snapshot from history rather than accepting a caller-authored
  snapshot;
- save the derived snapshot with an exact prior-cursor compare-and-swap;
- persist the exact projection binding and operation receipt; and
- commit once.

Any validation, serialization, relational, or projection failure rolls back
the complete transaction. A busy/locked result preserves the existing stable
reread-before-retry posture.

All SQLite event writers must use one transaction-scoped append primitive that
derives and persists the next snapshot before commit. The public
`RunSnapshotStore::save_snapshot` path remains source compatible but becomes a
monotonic compare-and-swap projection write: it accepts an identical snapshot
or the exact next event-derived cursor and rejects stale, skipped, or
history-inconsistent snapshots. It cannot overwrite a newer cursor. A repair
helper may rebuild a snapshot from durable event history, but no caller-authored
snapshot becomes authoritative.

## 9. Replay And Reconciliation

Exact replay is read-equivalent:

- the same operation ID and canonical request commitment return the original
  continuity result and projection binding;
- no new event is appended;
- the snapshot cursor does not advance; and
- the binding is revalidated against the durable historical event and operation
  rows. The current snapshot may be a lawful later successor and need not equal
  the historical result cursor.

The same operation ID with different canonical content fails closed before a
write. A projection binding whose event, sequence, snapshot cursor, operation
target, or receipt relationship is missing or mismatched is corruption, not an
ordinary retry.

After an ambiguous commit acknowledgement, reconciliation must open a fresh
connection, query by operation ID and request commitment, verify the complete
binding, and return the original result only when every relationship is exact.
Absence permits a fresh retry under the original operation identity. Partial
presence or conflicting presence is recovery-required and must never append a
replacement event. Lawful successor validation requires the bound event to
remain at its exact sequence and identity, the current event stream to contain
that prefix, and the current snapshot cursor to be equal to or later than the
bound result cursor after deterministic rehydration.

## 10. Snapshot Projection Rules

The run snapshot remains derived exclusively from ordered events. Add one
optional `last_continuity_projection` field containing only event-safe
operation kind, disposition, receipt reference, projection commitment, and
result cursor. The field is an inspection cache, not current authority or
current disposition. The future supervisor must resolve continuity state
directly.

The snapshot must not serialize execution capabilities, authoritative wait
tokens, private attempt-use capabilities, resource values, or an independently
mutable runnable flag. A historical projection remains valid after later
events; only the snapshot's last projection cache changes.

The projection commitment uses a new domain-separated versioned digest over
canonical length-prefixed fields: contract version, workflow ID, run ID,
operation kind, operation ID, request commitment, receipt ID, disposition,
stable rejection class or applied-result target identity and revision,
expected-input sequence and event ID, committed-result sequence and event ID,
and resulting snapshot event cursor. It does not hash raw event JSON or
caller-controlled display text.

## 11. Compatibility And Schema Posture

The event additions are additive. Existing events and snapshots must remain
readable. Any new snapshot field must use a safe serde default, and unknown
future event variants must continue to fail according to the repository's
current strict runtime-event posture.

SQLite adapter schema V3 is required. It adds the projection-binding table,
operation-to-event and event-to-operation uniqueness, result-cursor ownership,
snapshot cursor checks, and required indexes and relational constraints. The
V2-to-V3 upgrade is explicit, atomic, checksummed, interruption-safe, and
compatible with the prior V1-to-V2 continuity upgrade. V2 readers may inspect
pre-upgrade databases only; V2 writers fail closed after V3 activation.

Filesystem and PostgreSQL must declare the new atomic projection capability
unsupported and perform zero writes. Existing generic event and snapshot APIs
remain source compatible and do not imply continuity projection support.

## 12. Failure Semantics

- Before commit: all continuity, event, snapshot, and binding writes roll back.
- During commit: return a stable ambiguous-commit error requiring fresh
  reconciliation.
- After commit but before acknowledgement: reconciliation returns the original
  result and binding without a duplicate event.
- Event cursor stale: reject before mutation.
- Continuity revision stale: preserve existing deterministic error precedence.
- Snapshot derivation failure: roll back; never persist a caller-provided
  substitute.
- Projection corruption: fail closed with a stable code and no raw stored
  value.
- Unsupported backend: return the explicit capability error with zero writes.

An atomic projection failure does not invent a workflow diagnostic, mark the
run complete, or authorize executor entry.

## 13. Runtime Transition Matrix

`AuthorizedExecutionContinuityProjected` is status-preserving. It is legal only
for non-terminal runs and never creates `Completed`, `Failed`, or `Canceled`.

| Continuity result | Created | Validated | Running | WaitingForApproval | WaitingForExternalEvent | Retrying | Escalated | Terminal |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| yield registered | reject | reject | allow | reject | reject | allow | reject | reject |
| wait transitioned | reject | reject | allow | reject | allow | allow | allow | reject |
| directive consumed / attempt started | reject | reject | allow | reject | allow | allow | allow | reject |
| ordinary attempt outcome | reject | reject | allow | reject | allow | allow | allow | reject |
| ambiguity recovery | reject | reject | allow | reject | allow | allow | allow | reject |
| committed security rejection | reject | reject | allow | allow | allow | allow | allow | reject |

`WaitingForApproval` permits only a committed security-rejection disclosure;
an applied continuity operation cannot bypass an unsatisfied approval. A
directive can leave a genuine external wait only after its wake transition has
committed. Runtime status changes, including resume or terminal events, remain
separate governed events outside this phase.

## 14. Privacy And Redaction

Projection models, events, Debug output, serde errors, SQL mapping errors, and
reconciliation errors must remain bounded and non-leaking. They may contain
validated opaque IDs, enums, revisions, counts, timestamps, and cryptographic
commitments only.

They must not contain raw source or spec contents, prompts, model transcripts,
local command output, CI logs, provider bodies, environment values, paths,
credentials, authorization headers, private keys, tokens, or bearer authority.
Stable errors must not echo rejected identifiers or persisted payloads.

## 15. Test Plan

The implementation phase must add:

- all five operation families committing continuity state, one event, one
  snapshot advancement, one receipt, and one projection binding atomically;
- exact replay producing no additional event;
- exact committed security-rejection replay producing no additional event;
- same-key/different-content rejection;
- stale event cursor and stale continuity revision rejection with zero writes;
- concurrent writers producing one winner and contiguous event history;
- generic event append racing a projected operation while preserving one
  contiguous history and monotonic snapshot;
- stale independent snapshot overwrite rejection;
- every expected-input to committed-result cursor write set;
- historical replay after lawful later event and continuity revisions;
- every cell of the closed runtime transition matrix;
- transaction-scoped semantic parity between standalone and projected APIs;
- canonical projection commitment determinism and domain separation;
- before-, during-, and after-commit fault injection for all five families;
- fresh-connection reconciliation for success, absence, conflict, and partial
  corruption;
- event-to-operation, event-to-target, receipt-to-operation, snapshot-to-event,
  workflow/run identity, and sequence relational corruption probes;
- deterministic event rehydration and snapshot equality after restart;
- subprocess same-path WAL crash/reopen proof;
- unsupported filesystem and PostgreSQL zero-write behavior;
- V2-to-V3 schema upgrade interruption, old-writer rejection, and restore
  posture;
- bounded serde, Debug, SQL, and error non-leakage;
- final assistant response and host delivery remaining unable to append
  completion; and
- all existing runtime, state, continuity, executor, approval, report,
  provider, migration, and integration tests unchanged.

## 16. Candidate Implementation Sequence

1. Add the closed event payload, private projection-binding and commitment
   models, stable errors, transition matrix, and explicit backend support.
2. Extract transaction-scoped continuity and event/snapshot primitives while
   preserving standalone API behavior.
3. Extend the in-memory reference store with an atomic projected-operation
   adapter and backend-parametric conformance scenarios.
4. Add SQLite adapter schema V3 and relational binding constraints.
5. Implement all five SQLite projected-operation transactions by reusing the
   accepted semantic V2 functions.
6. Add fault injection, fresh-connection reconciliation, restart, corruption,
   and migration tests.
7. Run focused maintainer/security review before any supervisor integration.
8. Only after acceptance, implement one local injected trusted-host supervisor
   vertical slice.

## 17. Validation

The implementation phase must run:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check`;
- `npm run check:integrations`;
- `npm run check:docs`; and
- `git diff --check`.

## 18. Acceptance Criteria

- Every accepted SQLite continuity mutation and its runtime event/snapshot
  projection commit or roll back together.
- Exact replay never appends another event or advances the snapshot.
- Commit ambiguity is reconciled from durable operation and projection
  bindings.
- Runtime events disclose committed facts without becoming authority.
- Generic event and snapshot paths cannot split or regress projected history.
- Both applied and committed security-rejection results receive one bounded
  event under the closed transition matrix.
- Per-operation input/result cursor write sets are enforced.
- SQLite adapter schema V3 and old-writer posture are explicit.
- No supervisor, scheduling, automatic approval, execution, provider write,
  public schema, or default-backend behavior is introduced.
- Filesystem and PostgreSQL remain explicitly unsupported.
- Privacy, compatibility, and existing workflow semantics remain intact.
- Focused maintainer/security review accepts the phase before supervisor work.

## 19. Resolved Review Decisions

- Use one closed top-level event payload, not five unrelated open surfaces.
- Project only the last bounded non-authoritative continuity receipt in the
  snapshot; resolve current disposition from continuity state.
- Allocate event IDs with the existing kernel-owned generator inside the
  projected operation and bind them relationally; do not derive IDs from a
  public commitment.
- Use explicit SQLite adapter schema V3.
- Use `state.continuity_projection.corrupt` for partial or conflicting durable
  projection presence and retain the existing ambiguous-commit code only when
  durable presence cannot yet be classified.

## 20. Final Recommendation

Implement the atomic event/state projection contract and SQLite V3 proof only.
The corrected plan is accepted in [Authorized Execution Continuity Event And
State Projection Plan Blocker Fix
Review](../concepts/AUTHORIZED_EXECUTION_CONTINUITY_EVENT_STATE_PROJECTION_PLAN_BLOCKER_FIX_REVIEW.md).
Keep the local injected supervisor, automatic resume, operational window
opening, provider mutations, nested harness execution, and public
configuration out of that implementation.
