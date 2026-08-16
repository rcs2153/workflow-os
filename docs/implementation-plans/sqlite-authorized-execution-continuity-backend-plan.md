# SQLite Authorized Execution Continuity Backend Plan

Status: implemented and accepted after focused blocker-fix review

## 1. Executive Summary

Workflow OS now has an accepted atomic authorized-execution continuity contract
and a test-only in-memory reference implementation. No durable backend currently
implements that contract. A process restart therefore cannot yet preserve the
kernel-owned distinction between runnable work, a genuine wait, an executor
yield, a consumed resume directive, and an ambiguous attempt.

This plan defines the first durable implementation: an explicit, local-only
SQLite continuity backend. It adds additive, schema-versioned continuity state;
implements the five accepted atomic operation families with `BEGIN IMMEDIATE`;
extracts one backend-parametric conformance harness; and advertises SQLite
support only after restart, contention, replay, trusted-time, corruption, and
commit-ambiguity tests pass.

This phase does not add runtime events, open execution windows from the
executor, schedule or resume an agent, approve gates automatically, execute
tools, mutate providers, expose workflow or CLI schema, or make SQLite the
default backend.

The shared semantic V2 prerequisite is implemented as an additive Core
contract type and in the test-only reference store. The existing exhaustive
V1 contract enum and provider API remain source compatible. V2 adds committed
security-rejection dispositions, epoch-bound trusted-time observations,
consume-by-value authority, read-only reconciliation, exact wait identity,
private instance eligibility, and kernel-owned `resume_now`,
`await_condition`, `blocked`, and `terminal` continuation classification.
Focused maintainer/security review accepted the final owner-to-target blocker
correction and authorized this bounded SQLite phase in [Authorized Execution
Continuity Semantic V2 Owner-Target Blocker Fix
Review](../concepts/AUTHORIZED_EXECUTION_CONTINUITY_SEMANTIC_V2_OWNER_TARGET_BLOCKER_FIX_REVIEW.md).

SQLite implementation and repository validation are complete. Schema V2, the
explicit V1-to-V2 upgrade, all five operation transactions, trusted-time state,
replay and reconciliation, support declarations, and focused backend tests are
present. The first focused review required one complete shared scenario matrix,
all-five-operation fault proof, and same-path subprocess crash/WAL and restart
proof. Those fixes are implemented and focused re-review accepts the bounded
SQLite semantic V2 support claim. This acceptance does not make SQLite the
default or production-certified, and local filesystem and PostgreSQL remain
unsupported for semantic V2 continuity. Historical
planning and blocker findings below are retained as the design and review
record that constrained this implementation.

## 2. Problem Statement

Authorized work can currently survive only inside the reference semantics. The
existing embedded SQLite state adapter stores normal run, approval, audit,
report, and SideEffect records, but it explicitly reports every authorized
execution continuity operation as unsupported.

The product invariant is stronger than session continuity:

- an executor turn ending is not workflow completion;
- only the kernel may classify the governed run as runnable, waiting, blocked,
  or terminal;
- a resume directive has one durable winner;
- an attempt that may have started but lacks a durable outcome is ambiguous,
  never silently retryable;
- delegated authority is scoped capability, not model self-approval; and
- restart must preserve the same disposition and replay result.

SQLite is the first appropriate durable proof because Workflow OS already has
an opt-in local SQLite adapter with WAL, full synchronous durability, bounded
busy handling, schema checksums, and immediate transactions. It does not imply
multi-host or shared-worker support.

## 3. Goals

- Implement all five continuity operation families under the shared V2
  continuity contract for embedded SQLite.
- Preserve exact replay and conflict semantics across process restart.
- Make directive consumption and attempt start one atomic transaction.
- Make outcome recording, ambiguity recovery, wait transition, and yield
  registration atomic with their operation receipt.
- Add durable database-wide trusted-time state that fails closed across wall
  clock rollback, provenance mismatch, and ordinary process restart.
- Declare and enforce a local-live-state-only eligibility posture separately
  from operation availability.
- Extract one private backend-parametric conformance harness and run every
  applicable scenario against both reference and SQLite implementations.
- Add explicit, atomic SQLite schema upgrade behavior.
- Keep local filesystem and PostgreSQL continuity support explicitly
  unsupported.
- Preserve existing executor, workflow, approval, report, provider, and CLI
  semantics.

## 4. Non-Goals

This phase does not implement:

- runtime event or audit projection for continuity transitions;
- operational execution-window opening;
- executor, scheduler, supervisor, or agent redispatch integration;
- automatic gate approval or evidence satisfaction;
- delegated-authority policy changes;
- provider writes or another mutation family;
- PostgreSQL continuity support;
- local-filesystem continuity emulation;
- nested harness execution or agent teams;
- workflow-spec, public schema, SDK, or CLI exposure;
- automatic SQLite selection or default-backend changes;
- distributed leases, multi-host workers, or hosted operation;
- exactly-once external effects;
- general SQLite migration tooling unrelated to this additive upgrade; or
- production backup/restore certification.

## 5. Existing Boundary

The accepted state contract defines:

- `RegisterYield`;
- `TransitionWait`;
- `ConsumeDirective`;
- `RecordAttemptOutcome`; and
- `RecoverAmbiguousAttempt`.

It also defines private authoritative records for windows, yields, waits,
directives, attempts, committed operations, and receipts; private one-use
authority, wake, and attempt capabilities; and a test-only reference store.

The current SQLite adapter:

- opens a fresh connection per operation;
- uses WAL, foreign keys, `synchronous=FULL`, and a bounded busy timeout;
- serializes writers through `BEGIN IMMEDIATE`;
- maps busy/locked states to a stable reread-before-retry error;
- uses `PRAGMA user_version` plus checksummed schema metadata; and
- has no general V1-to-V2 upgrade path.

The reusable harness and durable trusted-time design are mandatory blockers,
not optional follow-ups.

## 6. Schema Version And Upgrade Posture

Introduce embedded SQLite adapter schema version 2. Version 2 is additive to
the accepted version-1 record families and adds continuity tables plus trusted
time state.

The upgrade must be explicit. `SqliteStateBackend::open` must not silently
upgrade version 1. Opening a valid version-1 database returns a stable
`upgrade_required` error. A separately named Core library operation performs
the exact V1-to-V2 upgrade after the caller has selected the path deliberately.

The upgrade operation must:

1. open one configured connection;
2. begin `BEGIN IMMEDIATE`;
3. verify exact V1 `user_version`, ready migration posture, and checksum;
4. create every additive continuity table, index, and constraint;
5. create and initialize the trusted-time singleton in an unobserved posture;
6. update schema metadata and `user_version` together; and
7. commit once.

Failure before commit leaves a complete V1 database. Success leaves a complete
V2 database. A partial V2 shape, mismatched checksum, unknown version, or stale
filesystem-to-SQLite migration plan fails closed. Existing migration plan
fingerprints that name adapter schema version 1 are intentionally invalidated
and must be regenerated rather than silently rewritten.

Version-2 open remains initialization-only for empty version-zero databases or
validation-only for exact V2 databases. Old readers reject V2.

## 7. Candidate SQLite Record Families

Use relational identity, state, revision, and ownership columns with bounded
canonical encoded envelopes where that avoids a second domain model.

Required tables:

- `continuity_windows`;
- `continuity_yields`;
- `continuity_waits`;
- `continuity_directives`;
- `continuity_attempts`;
- `continuity_operations` containing the operation commitment, result
  commitment, receipt, committed trusted-time observation, and exact replay
  material; and
- `continuity_trusted_time`, a single database-wide epoch/watermark record.

Required relational guarantees include:

- globally unique operation IDs and receipt IDs;
- unique window, yield-generation, directive, wait-condition/version, and
  attempt identities;
- unique `(window_id, attempt_number)` allocation;
- one yield per attempt;
- positive revisions, generations, and attempt numbers;
- foreign-key ownership from child records to the exact window/generation;
- one active yield generation for an eligible exact window binding;
- one-way consumed directive state;
- exact cursor, immutable bundle, governance, authority, subject, and scope
  bindings; and
- checked Rust-to-SQLite numeric conversions and checked revision increments.

Private bearer capabilities are never serialized. The database stores their
validated commitments and resulting records, not reconstructable authority.

The shared semantic amendment must remove the borrowed capability from
`ConsumeDirectiveRequest<'_>`. The request owns one non-cloneable
`AuthorityUseCapability`, and `consume_directive` takes the request by value.
Crossing the store boundary burns the capability whether the operation commits,
rolls back, or returns commit ambiguity. No error path returns it to the caller.

## 8. Trusted-Time Design

SQLite and Rust wall clocks are not monotonic across restart. Rust `Instant`
cannot be serialized, and SQLite time functions still read host wall time. The
backend therefore uses a Core-owned injected clock plus durable database-wide
security state.

`continuity_trusted_time` records:

- fixed source kind;
- provenance commitment;
- epoch ID;
- last observed UTC seconds and nanoseconds;
- `unobserved`, `healthy`, or `quarantined` posture; and
- positive revision.

Every window also stores the trusted-time epoch and its own watermark. For a
new operation, the backend:

1. acquires the immediate write transaction;
2. checks committed operation history first;
3. returns an exact replay without consulting the clock;
4. obtains one coherent observation from the store-owned clock;
5. validates source, provenance, and epoch;
6. requires `observed_now` to be at least both global and window watermarks;
7. requires `observed_now < expires_at`;
8. updates the global and window watermarks with the domain transaction; and
9. commits the receipt and exact replay material atomically.

Clock behavior is fail closed:

- unavailable time writes nothing and returns `time.unavailable`;
- a regressed clock atomically quarantines the trusted-time epoch without
  mutating continuity domain records, then returns `time.regressed`;
- incompatible provenance quarantines the epoch and returns `time.untrusted`;
- equality at expiry advances durable security time, marks the exact window
  expired when safe in the same transaction, and returns `time.expired`;
- a quarantined or mismatched epoch rejects new mutations while read-only
  inspection and exact replay remain available; and
- forward jumps may expire authority early but cannot extend authority.

Security-only time/quarantine writes are not successful continuity-domain
mutations. The shared conformance contract must distinguish those writes from
window, wait, directive, attempt, operation, and receipt changes.

Do not auto-clear quarantine when wall time catches up. Recovery into a new
epoch requires a separately governed future operation backed by trusted time
evidence, fences old windows, and remains outside this implementation phase.

Copied or restored databases cannot prove elapsed real time with database-local
state and a wall clock. This phase therefore does **not** claim continuity
support after arbitrary backup restore, database replacement, coordinated
rollback of the database and its host state, or VM/filesystem snapshot restore.
Operators must not use restored continuity state for execution. The future
runtime integration must require an external, non-copied epoch anchor before a
restored database can become eligible. Until that separate contract exists,
restore is unsupported rather than described as automatically detected or
quarantined.

Ordinary close/reopen and process or machine crash without durable-state
rollback remain in scope. A rollback observed against the still-current
database-wide watermark quarantines the current epoch.

### 8.1 Transaction Dispositions

The shared semantic contract must be amended before SQLite implementation to
distinguish:

- `CommittedSuccess`, with the normal operation result and receipt;
- `CommittedSecurityRejection`, with a stable bounded rejection kind, rejection
  commitment, trusted-time observation, and receipt; and
- `RolledBackFailure`, with no operation, receipt, trusted-time, or domain
  mutation.

Clock regression, incompatible provenance, epoch mismatch discovered during an
otherwise well-formed new operation, and expiry are committed security
rejections when they change durable security posture. Clock unavailability,
malformed input, stale capability, binding mismatch, storage busy, and SQL
failure before commit are rolled-back failures.

Every committed security rejection inserts the operation record and globally
unique receipt in the same transaction as the quarantine, watermark, or window
expiry change. Exact replay checks this record before the clock and returns the
same rejection disposition without re-observing time. The operation commitment
includes epoch ID and trusted-time observation. A committed rejection never
returns bearer authority and never authorizes executor entry.

This is a shared reference-contract amendment, not a SQLite-only exception.
The reference implementation and backend-parametric conformance suite must
adopt it before any SQLite schema or support declaration changes.

For committed success, `result_commitment` is the existing domain-separated
commitment to the bounded result. For committed security rejection,
`rejection_commitment` is a domain-separated hash over the stable rejection
kind, trusted-time epoch and observation, and exact durable security-state
delta. The common `operation_commitment` is a domain-separated hash over the
request commitment, receipt ID, disposition, trusted-time commitment and
observation, plus exactly one of the result or rejection commitments. The
persisted receipt binds that exact operation commitment, trusted-time
commitment, operation kind, receipt ID, and commit timestamp. Receipt
`committed_at` is exactly the trusted-time observation used by the operation;
the DDL stores both projections only for canonical decoding and requires them
to be equal. Replay and health validation recompute every commitment from
persisted bounded fields.

Each committed operation retains canonical relational request-target columns
plus a bounded `request_json` envelope. The envelope excludes bearer capability
and excludes the relational target duplicated in columns; together they include
every identity, expected revision, binding commitment, cursor, receipt ID, and
operation-specific input used by the request commitment. A committed success
must repeat the exact relational request target in success-target columns and
retains one canonical
`result_json` envelope containing the operation-specific target identities,
result state, and resulting revisions. A committed rejection instead retains
one canonical `rejection_json` envelope containing:

- rejection kind;
- trusted-time source kind, provenance commitment, epoch, and observation;
- affected window identity when applicable;
- exact prior and resulting trusted-time posture, eligibility, watermarks, and
  revisions; and
- exact prior and resulting window expiry/state/watermark/revision delta when
  applicable.

Success and rejection envelopes are mutually exclusive. Historical
commitments are recomputed from these immutable envelopes, never inferred from
later mutable rows.

Every committed operation also stores immutable trusted-time source kind and
provenance commitment alongside epoch and observation. Those columns are the
historical source of truth for recomputing `trusted_time_commitment` for both
success and rejection; replay never consults the later singleton value.

### 8.2 Live-State Eligibility

Operation availability and database-instance eligibility are separate. The
shared amendment introduces an additive V2 contract type with an explicit
`local_live_state_only` support scope, plus a private stateful eligibility read
returning exactly `live_state_eligible`, `restore_unverified`, or
`quarantined`. A backend operation requires both V2 operation support and
`live_state_eligible` before mutation. The existing V1 contract enum remains
unchanged so downstream exhaustive matches do not break; future V2 backends
implement the additive declaration boundary instead of widening that enum.

Fresh V2 initialization and a validated normal reopen retain
`live_state_eligible`. A separately named managed import/restore entrypoint
must set `restore_unverified` before restored continuity rows can be opened for
mutation. No operation may clear that posture in this phase. Arbitrary
out-of-band replacement of both database and host state is outside the claimed
threat model because it is not detectable locally. Future executor integration
is prohibited until an external rollback-resistant epoch-anchor contract can
establish eligibility after restore.

## 9. Atomic Operation Protocol

Each of the five operations uses one fresh configured connection and one
`BEGIN IMMEDIATE` transaction.

The common order and error precedence are:

1. validate bounded request shape before SQL;
2. begin the immediate transaction;
3. query the operation ID;
4. return exact persisted replay or reject changed-content replay;
5. load exact authoritative records and verify relational identity, revisions,
   immutable bindings, and the capability's static commitments;
6. reject stale, mismatched, or already-consumed authority as a rolled-back
   failure before any time/security-state mutation;
7. obtain and validate trusted time for the otherwise legal new operation;
8. evaluate time-dependent authority validity and expiry;
9. update the full domain or committed-security-rejection write set;
10. insert the globally unique receipt and canonical operation replay record
    last; and
11. commit once.

This precedence is normative. A stale capability or binding mismatch cannot be
masked by a simultaneous clock regression. Clock/security rejection is
committed only for a request that first passes request, identity, revision,
binding, capability-ownership, and one-use validation.

No internal retry loop hides `SQLITE_BUSY`. The caller receives a stable,
non-leaking retryable storage conflict and must reread authoritative state
before retrying the same operation identity.

A commit return error is reported as stable
`state.sqlite.commit_ambiguous`. The backend discards the transaction and
connection. The caller reconciles on a fresh connection through a private
production `AuthorizedExecutionContinuityReconciler` boundary. Its only method
accepts the operation ID, expected request commitment, and expected receipt ID,
performs no write, and validates the full persisted record. It is not fixture/
test bootstrap and does not mint or return authority.

Reconciliation has exactly three outcomes:

- `DurablyCommitted`, returning the persisted success or security-rejection
  disposition without bearer capability;
- `ConfirmedAbsent`, proving no operation or receipt exists after a successful
  authoritative read; or
- `StateUnreadable`, preserving ambiguity and prohibiting retry or execution.

`ConsumeDirectiveRequest` owns its `AuthorityUseCapability`, and the store
method consumes the request by value before any database outcome is known.
For `ConfirmedAbsent`, the consumed in-memory capability is not reconstructed.
The caller must obtain fresh current authority and issue a fresh operation
identity. For a durably committed `ConsumeDirective`, reconciliation never
enters the executor because replay cannot reproduce the original one-use
attempt capability; the started attempt is routed to the accepted ambiguity
recovery posture. The backend must never infer rollback from a commit-return
error or reuse the original capability.

Exact replay returns the persisted result and persisted time observation. A
replayed `ConsumeDirective` never recreates an `AttemptUseCapability`.

## 10. Exact SQLite V2 Schema Specification

Implementation must encode an exact equivalent of the following DDL. Names may
change only during focused review; columns, constraints, and relationships may
not be weakened implicitly.

```sql
CREATE TABLE continuity_trusted_time (
  singleton_id INTEGER PRIMARY KEY CHECK (singleton_id = 1),
  source_kind TEXT NOT NULL CHECK (source_kind = 'core_injected_clock_v1'),
  provenance_commitment TEXT NOT NULL CHECK (length(provenance_commitment) BETWEEN 1 AND 256),
  epoch_id TEXT NOT NULL CHECK (length(epoch_id) BETWEEN 1 AND 128),
  observed_seconds INTEGER,
  observed_nanos INTEGER CHECK (observed_nanos BETWEEN 0 AND 999999999),
  posture TEXT NOT NULL CHECK (posture IN ('unobserved','healthy','quarantined')),
  eligibility TEXT NOT NULL CHECK (eligibility IN ('live_state_eligible','restore_unverified','quarantined')),
  revision INTEGER NOT NULL CHECK (revision > 0),
  CHECK ((posture = 'unobserved' AND observed_seconds IS NULL AND observed_nanos IS NULL)
      OR (posture <> 'unobserved' AND observed_seconds IS NOT NULL AND observed_nanos IS NOT NULL)),
  CHECK ((posture = 'quarantined' AND eligibility = 'quarantined')
      OR (posture <> 'quarantined' AND eligibility <> 'quarantined'))
);

CREATE TABLE continuity_windows (
  window_id TEXT PRIMARY KEY CHECK (length(window_id) BETWEEN 1 AND 128),
  workflow_id TEXT NOT NULL CHECK (length(workflow_id) BETWEEN 1 AND 128),
  run_id TEXT NOT NULL CHECK (length(run_id) BETWEEN 1 AND 128),
  step_id TEXT NOT NULL CHECK (length(step_id) BETWEEN 1 AND 128),
  window_binding_commitment TEXT NOT NULL CHECK (length(window_binding_commitment) BETWEEN 1 AND 256),
  subject_actor_id TEXT NOT NULL CHECK (length(subject_actor_id) BETWEEN 1 AND 128),
  immutable_bundle_commitment TEXT NOT NULL CHECK (length(immutable_bundle_commitment) BETWEEN 1 AND 256),
  governance_commitment TEXT NOT NULL CHECK (length(governance_commitment) BETWEEN 1 AND 256),
  authority_commitment TEXT NOT NULL CHECK (length(authority_commitment) BETWEEN 1 AND 256),
  cursor_sequence INTEGER NOT NULL CHECK (cursor_sequence > 0),
  cursor_event_id TEXT NOT NULL CHECK (length(cursor_event_id) BETWEEN 1 AND 128),
  state TEXT NOT NULL CHECK (state IN ('assessment_required','executing','yielded','closed','recovery_required','expired','revoked','superseded')),
  maximum_attempts INTEGER NOT NULL CHECK (maximum_attempts > 0),
  next_attempt_number INTEGER NOT NULL CHECK (next_attempt_number > 0),
  expires_seconds INTEGER NOT NULL,
  expires_nanos INTEGER NOT NULL CHECK (expires_nanos BETWEEN 0 AND 999999999),
  watermark_seconds INTEGER NOT NULL,
  watermark_nanos INTEGER NOT NULL CHECK (watermark_nanos BETWEEN 0 AND 999999999),
  trusted_time_epoch_id TEXT NOT NULL CHECK (length(trusted_time_epoch_id) BETWEEN 1 AND 128),
  revision INTEGER NOT NULL CHECK (revision > 0),
  active_yield_generation_id TEXT,
  record_json TEXT NOT NULL CHECK (length(record_json) BETWEEN 2 AND 16384),
  UNIQUE (window_id, window_binding_commitment),
  FOREIGN KEY (active_yield_generation_id, window_id)
    REFERENCES continuity_yields(generation_id, window_id)
    ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED
);

CREATE UNIQUE INDEX continuity_one_active_window
ON continuity_windows(workflow_id, run_id, step_id, window_binding_commitment)
WHERE state IN ('executing','yielded','recovery_required');

CREATE TABLE continuity_attempts (
  attempt_id TEXT PRIMARY KEY CHECK (length(attempt_id) BETWEEN 1 AND 128),
  window_id TEXT NOT NULL REFERENCES continuity_windows(window_id) ON DELETE RESTRICT,
  attempt_number INTEGER NOT NULL CHECK (attempt_number > 0),
  subject_actor_id TEXT NOT NULL CHECK (length(subject_actor_id) BETWEEN 1 AND 128),
  cursor_sequence INTEGER NOT NULL CHECK (cursor_sequence > 0),
  cursor_event_id TEXT NOT NULL CHECK (length(cursor_event_id) BETWEEN 1 AND 128),
  authority_commitment TEXT NOT NULL CHECK (length(authority_commitment) BETWEEN 1 AND 256),
  consume_operation_id TEXT NOT NULL UNIQUE CHECK (length(consume_operation_id) BETWEEN 1 AND 128),
  consume_operation_kind TEXT NOT NULL DEFAULT 'consume_directive' CHECK (consume_operation_kind = 'consume_directive'),
  consume_operation_disposition TEXT NOT NULL DEFAULT 'committed_success' CHECK (consume_operation_disposition = 'committed_success'),
  state TEXT NOT NULL CHECK (state IN ('started','yielded','succeeded','retryable_failure','terminal_failure','ambiguous_may_have_started')),
  revision INTEGER NOT NULL CHECK (revision > 0),
  record_json TEXT NOT NULL CHECK (length(record_json) BETWEEN 2 AND 16384),
  UNIQUE (window_id, attempt_number),
  UNIQUE (attempt_id, window_id),
  UNIQUE (attempt_id, window_id, consume_operation_id),
  FOREIGN KEY (consume_operation_id, consume_operation_kind, consume_operation_disposition)
    REFERENCES continuity_operations(operation_id, operation_kind, disposition)
    ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED
);

CREATE TABLE continuity_yields (
  generation_id TEXT PRIMARY KEY CHECK (length(generation_id) BETWEEN 1 AND 128),
  window_id TEXT NOT NULL REFERENCES continuity_windows(window_id) ON DELETE RESTRICT,
  attempt_id TEXT NOT NULL UNIQUE,
  cursor_sequence INTEGER NOT NULL CHECK (cursor_sequence > 0),
  cursor_event_id TEXT NOT NULL CHECK (length(cursor_event_id) BETWEEN 1 AND 128),
  reason TEXT NOT NULL CHECK (reason IN ('turn_boundary','context_budget','host_preemption','voluntary_checkpoint','transient_executor_failure')),
  registered_seconds INTEGER NOT NULL,
  registered_nanos INTEGER NOT NULL CHECK (registered_nanos BETWEEN 0 AND 999999999),
  record_json TEXT NOT NULL CHECK (length(record_json) BETWEEN 2 AND 16384),
  UNIQUE (generation_id, window_id),
  FOREIGN KEY (attempt_id, window_id)
    REFERENCES continuity_attempts(attempt_id, window_id) ON DELETE RESTRICT
);

CREATE TABLE continuity_waits (
  condition_id TEXT NOT NULL CHECK (length(condition_id) BETWEEN 1 AND 128),
  condition_version INTEGER NOT NULL CHECK (condition_version > 0),
  window_id TEXT NOT NULL REFERENCES continuity_windows(window_id) ON DELETE RESTRICT,
  generation_id TEXT NOT NULL,
  wake_trigger TEXT NOT NULL CHECK (wake_trigger IN ('approval_decision_recorded','evidence_accepted','check_accepted','external_event_recorded','capability_availability_changed','deadline_reached','authority_source_changed','conflict_resolved')),
  state TEXT NOT NULL CHECK (state IN ('unsatisfied','satisfied','expired','superseded','canceled')),
  source_commitment TEXT CHECK (source_commitment IS NULL OR length(source_commitment) BETWEEN 1 AND 256),
  source_revision INTEGER CHECK (source_revision IS NULL OR source_revision > 0),
  revision INTEGER NOT NULL CHECK (revision > 0),
  record_json TEXT NOT NULL CHECK (length(record_json) BETWEEN 2 AND 16384),
  PRIMARY KEY (condition_id, condition_version),
  UNIQUE (condition_id),
  UNIQUE (condition_id, condition_version, window_id),
  UNIQUE (condition_id, condition_version, window_id, generation_id),
  FOREIGN KEY (generation_id, window_id)
    REFERENCES continuity_yields(generation_id, window_id) ON DELETE RESTRICT
);

CREATE TABLE continuity_directives (
  directive_id TEXT PRIMARY KEY CHECK (length(directive_id) BETWEEN 1 AND 128),
  window_id TEXT NOT NULL REFERENCES continuity_windows(window_id) ON DELETE RESTRICT,
  generation_id TEXT NOT NULL,
  cursor_sequence INTEGER NOT NULL CHECK (cursor_sequence > 0),
  cursor_event_id TEXT NOT NULL CHECK (length(cursor_event_id) BETWEEN 1 AND 128),
  authority_commitment TEXT NOT NULL CHECK (length(authority_commitment) BETWEEN 1 AND 256),
  state TEXT NOT NULL CHECK (state IN ('available','consumed','invalidated','expired')),
  revision INTEGER NOT NULL CHECK (revision > 0),
  record_json TEXT NOT NULL CHECK (length(record_json) BETWEEN 2 AND 16384),
  UNIQUE (directive_id, window_id, generation_id),
  FOREIGN KEY (generation_id, window_id)
    REFERENCES continuity_yields(generation_id, window_id) ON DELETE RESTRICT
);

CREATE TABLE continuity_operations (
  operation_id TEXT PRIMARY KEY CHECK (length(operation_id) BETWEEN 1 AND 128),
  receipt_id TEXT NOT NULL UNIQUE CHECK (length(receipt_id) BETWEEN 1 AND 128),
  operation_kind TEXT NOT NULL CHECK (operation_kind IN ('register_yield','transition_wait','consume_directive','record_attempt_outcome','recover_ambiguous_attempt')),
  request_commitment TEXT NOT NULL CHECK (length(request_commitment) BETWEEN 1 AND 256),
  request_json TEXT NOT NULL CHECK (length(request_json) BETWEEN 2 AND 16384),
  operation_commitment TEXT NOT NULL CHECK (length(operation_commitment) BETWEEN 1 AND 256),
  disposition TEXT NOT NULL CHECK (disposition IN ('committed_success','committed_security_rejection')),
  request_window_id TEXT NOT NULL CHECK (length(request_window_id) BETWEEN 1 AND 128),
  request_yield_generation_id TEXT CHECK (request_yield_generation_id IS NULL OR length(request_yield_generation_id) BETWEEN 1 AND 128),
  request_wait_condition_id TEXT CHECK (request_wait_condition_id IS NULL OR length(request_wait_condition_id) BETWEEN 1 AND 128),
  request_wait_condition_version INTEGER CHECK (request_wait_condition_version IS NULL OR request_wait_condition_version > 0),
  request_attempt_id TEXT CHECK (request_attempt_id IS NULL OR length(request_attempt_id) BETWEEN 1 AND 128),
  success_yield_generation_id TEXT CHECK (success_yield_generation_id IS NULL OR length(success_yield_generation_id) BETWEEN 1 AND 128),
  success_wait_condition_id TEXT CHECK (success_wait_condition_id IS NULL OR length(success_wait_condition_id) BETWEEN 1 AND 128),
  success_wait_condition_version INTEGER CHECK (success_wait_condition_version IS NULL OR success_wait_condition_version > 0),
  success_attempt_id TEXT CHECK (success_attempt_id IS NULL OR length(success_attempt_id) BETWEEN 1 AND 128),
  success_consume_operation_id TEXT CHECK (success_consume_operation_id IS NULL OR length(success_consume_operation_id) BETWEEN 1 AND 128),
  result_commitment TEXT CHECK (result_commitment IS NULL OR length(result_commitment) BETWEEN 1 AND 256),
  rejection_commitment TEXT CHECK (rejection_commitment IS NULL OR length(rejection_commitment) BETWEEN 1 AND 256),
  rejection_kind TEXT CHECK ((disposition = 'committed_success' AND rejection_kind IS NULL) OR
                              (disposition = 'committed_security_rejection' AND rejection_kind IN ('time_regressed','time_untrusted','time_epoch_mismatch','time_expired'))),
  trusted_time_source_kind TEXT NOT NULL CHECK (trusted_time_source_kind = 'core_injected_clock_v1'),
  trusted_time_provenance_commitment TEXT NOT NULL CHECK (length(trusted_time_provenance_commitment) BETWEEN 1 AND 256),
  trusted_time_epoch_id TEXT NOT NULL CHECK (length(trusted_time_epoch_id) BETWEEN 1 AND 128),
  observed_seconds INTEGER NOT NULL,
  observed_nanos INTEGER NOT NULL CHECK (observed_nanos BETWEEN 0 AND 999999999),
  trusted_time_commitment TEXT NOT NULL CHECK (length(trusted_time_commitment) BETWEEN 1 AND 256),
  result_json TEXT CHECK (result_json IS NULL OR length(result_json) BETWEEN 2 AND 16384),
  rejection_json TEXT CHECK (rejection_json IS NULL OR length(rejection_json) BETWEEN 2 AND 16384),
  committed_seconds INTEGER NOT NULL,
  committed_nanos INTEGER NOT NULL CHECK (committed_nanos BETWEEN 0 AND 999999999),
  UNIQUE (operation_id, operation_kind),
  UNIQUE (operation_id, operation_kind, disposition),
  FOREIGN KEY (request_window_id)
    REFERENCES continuity_windows(window_id) ON DELETE RESTRICT,
  FOREIGN KEY (success_yield_generation_id, request_window_id)
    REFERENCES continuity_yields(generation_id, window_id) ON DELETE RESTRICT,
  FOREIGN KEY (success_wait_condition_id, success_wait_condition_version, request_window_id)
    REFERENCES continuity_waits(condition_id, condition_version, window_id) ON DELETE RESTRICT,
  FOREIGN KEY (success_attempt_id, request_window_id)
    REFERENCES continuity_attempts(attempt_id, window_id) ON DELETE RESTRICT,
  FOREIGN KEY (success_attempt_id, request_window_id, success_consume_operation_id)
    REFERENCES continuity_attempts(attempt_id, window_id, consume_operation_id)
    ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED,
  CHECK (committed_seconds = observed_seconds AND committed_nanos = observed_nanos),
  CHECK ((operation_kind = 'register_yield'
          AND request_yield_generation_id IS NOT NULL
          AND request_wait_condition_id IS NULL
          AND request_wait_condition_version IS NULL
          AND request_attempt_id IS NULL)
      OR (operation_kind = 'transition_wait'
          AND request_yield_generation_id IS NULL
          AND request_wait_condition_id IS NOT NULL
          AND request_wait_condition_version IS NOT NULL
          AND request_attempt_id IS NULL)
      OR (operation_kind IN ('consume_directive','record_attempt_outcome','recover_ambiguous_attempt')
          AND request_yield_generation_id IS NULL
          AND request_wait_condition_id IS NULL
          AND request_wait_condition_version IS NULL
          AND request_attempt_id IS NOT NULL)),
  CHECK ((disposition = 'committed_success'
          AND result_commitment IS NOT NULL
          AND result_json IS NOT NULL
          AND rejection_kind IS NULL
          AND rejection_commitment IS NULL
          AND rejection_json IS NULL
          AND ((operation_kind = 'register_yield'
                AND success_yield_generation_id IS NOT NULL
                AND success_yield_generation_id = request_yield_generation_id
                AND success_wait_condition_id IS NULL
                AND success_wait_condition_version IS NULL
                AND success_attempt_id IS NULL
                AND success_consume_operation_id IS NULL)
            OR (operation_kind = 'transition_wait'
                AND success_yield_generation_id IS NULL
                AND success_wait_condition_id IS NOT NULL
                AND success_wait_condition_version IS NOT NULL
                AND success_wait_condition_id = request_wait_condition_id
                AND success_wait_condition_version = request_wait_condition_version
                AND success_attempt_id IS NULL
                AND success_consume_operation_id IS NULL)
            OR (operation_kind = 'consume_directive'
                AND success_yield_generation_id IS NULL
                AND success_wait_condition_id IS NULL
                AND success_wait_condition_version IS NULL
                AND success_attempt_id IS NOT NULL
                AND success_attempt_id = request_attempt_id
                AND success_consume_operation_id IS NOT NULL
                AND success_consume_operation_id = operation_id)
            OR (operation_kind IN ('record_attempt_outcome','recover_ambiguous_attempt')
                AND success_yield_generation_id IS NULL
                AND success_wait_condition_id IS NULL
                AND success_wait_condition_version IS NULL
                AND success_attempt_id IS NOT NULL
                AND success_attempt_id = request_attempt_id
                AND success_consume_operation_id IS NULL)))
      OR (disposition = 'committed_security_rejection'
          AND success_yield_generation_id IS NULL
          AND success_wait_condition_id IS NULL
          AND success_wait_condition_version IS NULL
          AND success_attempt_id IS NULL
          AND success_consume_operation_id IS NULL
          AND result_commitment IS NULL
          AND result_json IS NULL
          AND rejection_kind IS NOT NULL
          AND rejection_commitment IS NOT NULL
          AND rejection_json IS NOT NULL))
);
```

SQLite permits a forward table reference in a foreign-key declaration. The
active-yield relationship is therefore declared in the canonical window DDL,
and its cycle is checked at commit with `DEFERRABLE INITIALLY DEFERRED`. No
application-only substitute is accepted.

`window_binding_commitment` is the existing domain-separated commitment over
`ExpectedWindowBinding`: workflow, run, step, subject actor, immutable run
bundle, governance, authority, and cursor. The discarded `scope_commitment`
and `scope_generation` columns are not owned by the accepted model and must not
be inferred by SQLite.

The attempt-to-operation relationship is also deferred because a successful
consume creates the attempt before inserting its operation record last in the
same transaction. A successful consume cannot commit without a matching
successful `consume_directive` operation. A security-rejected consume creates
no attempt.

The operation row is the canonical relational request/target binding. Its
bounded request-target columns and `request_window_id` are part of request
commitment input. A successful operation must copy that exact identity into the
matching non-null success-target columns. Composite foreign keys require the
domain row to exist in the exact request window. Rejections have no success
target.

For consume, `success_consume_operation_id` must equal the operation row's own
`operation_id`. Its deferred composite foreign key then requires the target
attempt's `(attempt_id, window_id, consume_operation_id)` to equal that exact
triple. The attempt's reverse composite foreign key independently requires the
operation to be a successful consume. Pair-swapped, cross-window, cross-run,
null, or merely category-correct targets therefore fail at statement or commit.

The V2 checksum is a fixed Core constant computed from the canonical DDL bytes
at development time and verified against exact expected schema objects on open.
Health checks run `PRAGMA quick_check`, `PRAGMA foreign_key_check`, inspect the
required `sqlite_master` object names/types/SQL commitments, and verify every
relational identity against bounded canonical envelopes.

Only a database with no non-system objects and `user_version = 0` is eligible
for fresh V2 initialization. Explicit upgrade accepts exact ready V1 only.
Staging, importing, verifying, failed, rollback-required, checksum-mismatched,
or structurally incomplete V1 state is rejected. Two concurrent upgraders
serialize: the winner upgrades; the second re-reads exact V2 and returns an
idempotent already-upgraded result. A V1 filesystem migration plan remains
stale and must be regenerated for V2.

## 11. Backend-Parametric Conformance Harness

Extract the existing private reference-store scenarios into a crate-private,
`#[cfg(test)]` harness. Do not publish authoritative state vocabulary or test
bootstrap APIs.

Keep three distinct interfaces:

- the production `AuthorizedExecutionContinuityStore`, containing only the
  five semantic operations; and
- the private production `AuthorizedExecutionContinuityReconciler`, containing
  only the read-only operation/receipt reconciliation lookup; and
- a test-only `ContinuityConformanceBackend`, providing fresh seeded state,
  read-only snapshots, reopen/restart, injected trusted time, and operation/
  commit-phase fault injection.

Instantiate each named scenario independently for the reference store and
SQLite rather than hiding the suite behind one opaque aggregate test. Fixture
bootstrap remains unavailable through the production trait and is not evidence
that a legal runtime execution window was opened.

SQLite may report `Supported` for the five operations only under contract V2's
`local_live_state_only` scope and only when all applicable shared scenarios and
SQLite-specific durable scenarios pass. Every mutation still requires the
private instance eligibility read to return `live_state_eligible`.

## 12. Required Conformance Matrix

The shared matrix must cover:

- concurrent one-winner directive consumption;
- concurrent one-winner yield registration;
- exact replay and same-key/different-content conflict for all operations;
- global receipt uniqueness and cross-operation receipt reuse rejection;
- one-use capability behavior;
- consume-by-value authority burn on success, rollback, and ambiguous commit;
- consume replay without capability reconstruction;
- attempt-budget allocation and exhaustion under contention;
- wait satisfaction contention, stale source/revision rejection, and fresh
  authority after wake;
- yield and wait races with cursor advance, close, cancellation, expiry,
  revocation, and supersession;
- succeeded, retryable-failure, terminal-failure, yielded, and ambiguous
  attempt postures;
- capability-free ambiguity recovery and cross-run/stale-cursor rejection;
- restart from every attempt posture, especially orphaned `started`;
- trusted-time equality, unavailability, regression, provenance mismatch,
  quarantine, epoch mismatch, and forward-jump expiry;
- authority/governance changes before consume and after consume but before
  executor entry;
- before-, during-, and after-commit fault posture for all five operations;
- exact replay after an ambiguous commit return;
- read-only reconciliation returning committed, confirmed-absent, and
  unreadable outcomes without authority reconstruction;
- confirmed absence requiring both expected operation and receipt identity;
- successful replay after trusted-time singleton advance, quarantine, and
  epoch/provenance change;
- canonical wait ordering in the operation commitment;
- oversized/malicious identifiers and safe decode/error behavior; and
- no capability, path, SQL, payload, or secret-like leakage.

Each scenario defines the exact expected delta across windows, yields, waits,
directives, attempts, trusted-time state, operations, and receipts. Rolled-back
failure expects no delta. Committed security rejection expects only the named
security-state, operation, and receipt delta. Successful mutation expects its
complete domain and replay delta. Exact replay expects no new delta.

Replay validation must compare operation kind, request commitment, operation
commitment, disposition, the exactly applicable result or rejection
commitment, receipt identity and binding, epoch, trusted-time commitment and
historical source/provenance/observation, bounded decoding, and relational
identity before returning persisted output. Receipt committed time must equal
the persisted trusted observation.

SQLite-specific proof must add:

- close/reopen and subprocess restart with no process-local state;
- two independent backend instances and both writer lock orders;
- V1-to-V2 successful upgrade and interrupted rollback;
- old-reader rejection and stale migration-plan rejection;
- corrupt relational identity, envelope, clock singleton, enum, revision, and
  operation/receipt commitment rejection;
- missing or wrong-kind consume-operation ownership rejection;
- rejected-consume attempt ownership rejection;
- missing or wrong-shape operation-target ownership rejection for every
  successful operation kind;
- contradictory trusted-time posture/eligibility rejection;
- composite cross-window attempt/yield/wait/directive corruption rejection;
- managed restore posture remaining ineligible without an external anchor;
- `quick_check` plus continuity relational health validation;
- WAL/full-synchronous/busy-timeout preservation; and
- explicit proof that filesystem and PostgreSQL support remain unchanged.

The SQLite suite also covers exact replay while time is unavailable or the
epoch is quarantined, both lock orders for concurrent time observations,
concurrent upgrades, malformed schema objects, `foreign_key_check`, and
subprocess crash/WAL recovery.

## 13. Error, Privacy, And Retention Posture

Use stable `authorized_execution_continuity_state.*` semantic codes and bounded
`state.sqlite.*` backend codes. Errors must not contain database paths, SQL,
record values, clock provenance, workflow content, prompts, command output,
provider payloads, credentials, tokens, or secret-like identifiers.

`Debug` must omit the database path and redact bindings. Deserialization and
corruption errors fail closed without echoing persisted values. Continuity
records contain identifiers, commitments, revisions, enums, timestamps, and
stable references only.

Retention and deletion continue to inherit the local backend posture. This
phase does not add hosted retention guarantees.

The durable storage whitelist is limited to the columns and bounded canonical
envelopes in Section 10. It explicitly excludes private capabilities, prompts,
transcripts, source/spec contents, command output, provider payloads,
environment values, credentials, authorization material, private keys, and
token-like values. Tests scan database bytes, error strings, serialized test
views, and `Debug` output with canary values.

## 14. Compatibility And Existing Behavior

- Existing filesystem and PostgreSQL behavior remains unchanged.
- Local filesystem and PostgreSQL continuity declarations remain unsupported.
- Existing SQLite V1 data is preserved by explicit atomic upgrade.
- Existing SQLite APIs do not silently migrate or select continuity behavior.
- The durable-state V1 transaction support vocabulary is not silently widened;
  continuity remains separately versioned.
- Existing executor and approval APIs do not call the new store.
- No new run status, event kind, workflow field, CLI flag, or SDK shape is
  introduced.

## 15. Implementation Sequence

Current phase status: the semantic prerequisite and owner-target review are
accepted, the SQLite implementation and validation are complete, and the
first focused review's three blockers have been fixed under the governed record
documented in [SQLite Authorized Execution Continuity Backend Blocker Fix
Report](../concepts/SQLITE_AUTHORIZED_EXECUTION_CONTINUITY_BACKEND_BLOCKER_FIX_REPORT.md).
Focused blocker-fix re-review accepts the sequence evidence.

1. Amend the shared internal contract with committed security rejection,
   epoch-bound observations, consume-by-value authority, private read-only
   reconciliation, V2 local-live-state-only support scope, and private instance
   eligibility. Also align wait identity and enrich recorded replay results with
   target identity. Review this semantic amendment before SQLite schema work.
2. Extract private reference-store mechanics and the backend-parametric
   conformance harness without changing support declarations.
3. Add checked revision/attempt arithmetic and private backend accessors.
4. Add exact SQLite V2 schema vocabulary and explicit atomic V1-to-V2 upgrade.
5. Add the trusted-time singleton, store-owned clock injection, and health
   validation.
6. Implement the five SQLite operation transactions.
7. Run the shared suite against reference and SQLite plus SQLite-specific
   restart, upgrade, corruption, and contention tests.
8. Change only SQLite V2 operation declarations to supported under the
   local-live-state-only scope; keep non-live instances ineligible.
9. Run full validation and focused maintainer/security review.

Do not split support declaration from proof. If any operation cannot meet the
contract, all SQLite continuity operations remain unsupported until the phase
is re-scoped and reviewed.

## 16. Validation Plan

Future implementation validation must include:

- focused reference and SQLite continuity tests;
- existing SQLite backend and migration tests;
- existing public continuity contract tests;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check`;
- `npm run check:integrations`;
- `npm run check:docs`;
- dependency/security checks required by repository tooling; and
- `git diff --check`.

## 17. Acceptance Criteria

- SQLite schema V2 is explicit, checksummed, and opt-in.
- V1-to-V2 upgrade is deliberate, atomic, interruption-safe, and preserves
  existing records.
- All five continuity operations commit their entire write set once.
- Exact replay and conflicting replay survive restart.
- One directive consumer creates one durable started attempt.
- An orphaned started attempt remains ambiguity/recovery-required.
- Database-wide trusted time cannot silently regress or cross epochs.
- Quarantine cannot be cleared implicitly.
- Commit ambiguity cannot retain or reconstruct consumed authority.
- Every persisted relationship enforces same-window ownership in SQLite.
- Success and rejection receipts bind recomputable operation commitments.
- The same named conformance scenarios pass for reference and SQLite stores.
- SQLite-specific crash, corruption, contention, and upgrade tests pass.
- Arbitrary restored/rolled-back databases are explicitly unsupported until an
  external rollback-resistant epoch anchor is separately implemented.
- SQLite advertises scoped V2 support only after all required proof passes,
  and mutations additionally require live-state instance eligibility.
- Filesystem and PostgreSQL remain unsupported.
- No runtime scheduling, automatic approval, provider mutation, CLI/schema
  exposure, or default-backend change is introduced.

## 18. Resolved Decisions And Deferred Questions

- The explicit V1-to-V2 upgrade is a narrowly named associated operation on
  `SqliteStateBackend`; it is not part of `open` and does not select a backend.
- Clock quarantine and expiry return one internal committed-security-rejection
  disposition with distinct stable rejection kinds.
- Commit ambiguity uses `state.sqlite.commit_ambiguous` plus fresh-connection
  reconciliation; replayed consume never enters the executor.
- Arbitrary restore safety is not claimed in this phase. A future external
  anchor or signed backup receipt requires separate planning.
- Managed restore marks the database `restore_unverified`; arbitrary
  out-of-band coordinated replacement remains outside the claimed threat
  model.
- The implementation phase must select and document a portable subprocess
  crash harness before support is advertised.

None of these decisions authorizes runtime integration or weakens fail-closed
behavior.

## 19. Final Recommendation

The focused semantic V2 and owner-target blocker-fix reviews are accepted. The
SQLite V2 schema, operation transactions, trusted-time, upgrade, complete
shared backend-parametric conformance matrix, all-five fault matrix, and
same-path subprocess restart proof are implemented and accepted in focused
blocker-fix review.

Do not proceed directly to executor or supervisor integration. After SQLite is
accepted, add atomic continuity event/state projection. Only then implement one
local injected-supervisor vertical slice that can redispatch an external
executor while lawful work remains.
