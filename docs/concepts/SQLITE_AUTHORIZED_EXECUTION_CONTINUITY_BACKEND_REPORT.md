# SQLite Authorized Execution Continuity Backend Report

Report date: 2026-08-15

Status: implementation complete; focused review blockers fixed; focused
blocker-fix review accepted

## 1. Executive Summary

Workflow OS now has its first durable authorized-execution continuity backend
as an explicit local-only SQLite semantic V2 path. The implementation adds
schema V2, a deliberate V1-to-V2 upgrade, the five accepted atomic operation
families, database-wide trusted time, restart-safe replay and reconciliation,
and private backend-parametric test adapters for the reference and SQLite
stores.

The implementation and repository validation are complete. SQLite now declares
the five V2 operations supported only under the `local_live_state_only` scope,
while each mutation still requires live-state instance eligibility. Focused
backend review remains mandatory before the phase is accepted. In particular,
the implementation exposes one shared conformance interface but does not yet
express the complete accepted matrix as one set of shared named scenario
functions instantiated unchanged for both backends. The review must decide
whether that proof-shape gap blocks the support declaration.

## 2. Scope And Current Status

- **Implemented:** additive SQLite schema V2, managed
  schema metadata updates, exact-V1 upgrade detection, explicit atomic
  V1-to-V2 upgrade entrypoint, fresh V2 initialization, trusted-time singleton
  initialization, physical schema-manifest validation, and fail-closed V1
  `upgrade_required` behavior on ordinary open.
- **Implemented:** all five SQLite operation families, trusted-time mutation
  and quarantine behavior, exact replay and read-only reconciliation, restart,
  contention, corruption, commit-fault, and explicit-upgrade tests.
- **Implemented:** shared semantic helpers and a private conformance adapter
  interface implemented by both the reference and SQLite stores.
- **Pending review:** proof that the backend test shape satisfies the plan's
  stronger requirement for the same named scenario matrix against both
  backends, plus final support-declaration and security acceptance.

No production-readiness, arbitrary-restore, scheduling, or external-effect
claim is made by this report.

## 3. Schema V2 And Explicit Upgrade

SQLite adapter schema V2 is additive to the existing V1 record families. It
adds continuity windows, yields, waits, directives, attempts, committed
operations and receipts, plus the database-wide trusted-time singleton and
their required relational constraints and indexes.

Ordinary `SqliteStateBackend::open` does not silently upgrade V1. An exact,
ready V1 database returns the stable `state.sqlite.schema.upgrade_required`
error. The separately named
`upgrade_authorized_execution_continuity_v1_to_v2` operation acquires an
immediate transaction, validates exact V1 metadata and physical shape, applies
the additive schema, initializes trusted time, updates schema metadata and
`user_version`, and commits once. Exact V2 is an idempotent success; unknown,
incomplete, drifted, or unmanaged shapes fail closed.

The upgrade does not select SQLite automatically, make it the default backend,
or establish arbitrary backup/restore safety.

## 4. Five Atomic Operation Families

The accepted SQLite semantic V2 surface contains exactly these operation
families:

- `RegisterYield`: atomically records a lawful executor yield and its receipt
  against the exact window, cursor, attempt, and generation binding.
- `TransitionWait`: atomically records the exact typed wait owned by the active
  yield generation.
- `ConsumeDirective`: consumes one non-cloneable authority capability by value
  and atomically creates the single durable started attempt.
- `RecordAttemptOutcome`: atomically closes or advances the exact started
  attempt and records its replayable outcome.
- `RecoverAmbiguousAttempt`: atomically records capability-free recovery for an
  attempt that may have started but has no durable outcome.

Every family uses one immediate transaction, persists a globally unique
operation and receipt for committed dispositions, preserves exact replay across
restart, reject changed-content replay, and validate the accepted
owner-to-target relationships. Commit faults before or during commit roll back;
an after-commit acknowledgement fault returns the bounded ambiguous result and
is resolved through read-only reconciliation.

## 5. Trusted Time, Replay, And Reconciliation

The V2 design uses one Core-owned injected clock plus a durable database-wide
epoch, provenance commitment, watermark, posture, eligibility, and revision.
Each window also binds its trusted-time epoch and watermark. Exact committed
replay is checked before observing time.

Clock regression, incompatible provenance, epoch mismatch, and expiry use the
accepted committed-security-rejection semantics when durable security posture
changes. Clock unavailability and failures before commit remain rolled back.
Quarantine cannot clear itself, arbitrary restored state is not eligible for
execution, and replay never reconstructs consumed bearer authority.

Commit-return ambiguity is resolved through the private read-only reconciler,
which returns durably committed, confirmed absent, or state unreadable without
performing a write or minting authority. Focused tests cover time
unavailability, regression, provenance mismatch, epoch mismatch, expiry,
quarantine, committed replay, and reconciliation.

## 6. Conformance And Support Declarations

The private `ContinuityConformanceBackend` boundary is implemented by the
reference and SQLite stores. It normalizes snapshots, reopen behavior, trusted
time controls, and deterministic commit-fault injection. Shared semantic
helpers are also used by both stores.

The complete accepted semantic matrix is now expressed as reusable named
scenario functions and instantiated unchanged for the reference and SQLite
stores. The shared suite covers replay and changed-content conflicts for every
operation family, global receipt reuse, concurrency and attempt budgets, wait
binding, trusted-time rejection and replay, restart postures, capability burn,
authority and governance binding changes, terminal races, canonical ordering,
and before/during/after commit faults for all five operation families.

SQLite-specific tests complement that shared semantic proof with one physical
database path, WAL mode, separate-process crash/reopen behavior, independent
verification processes, committed-operation reconciliation, and restart from
every durable attempt posture. These fixes satisfy the three findings from the
first focused review but remain pending independent blocker-fix re-review.

Support declaration and instance eligibility remain separate. SQLite now
declares the five operations supported only under semantic V2's
`local_live_state_only` scope; each mutation also observes
`live_state_eligible`. Local filesystem and PostgreSQL remain unsupported. The
focused review must require a blocker fix or withdrawal of the declaration if
the present conformance evidence is insufficient.

## 7. Scope Explicitly Not Included

This phase does not add runtime event or audit projection, execution-window
opening from the executor, scheduling or agent redispatch, automatic approval
or evidence satisfaction, provider mutation, another write family, workflow
or CLI schema exposure, default backend selection, distributed leases,
multi-host workers, hosted operation, exactly-once external effects, arbitrary
restore certification, recursive agents, or agent swarms.

It also does not weaken the existing roadmap handoffs for approval-presentation
proof, proportional governance, scoped runtime authority, or authoritative
continuation rehydration. Those remain independent prerequisites and authority
boundaries; durable continuity does not substitute for any of them.

## 8. Tests And Validation

Implementation-owner validation completed on 2026-08-15:

- focused public continuity and SQLite backend tests: **passed**, 7 and 14;
- focused SQLite operation tests: **passed**, including all five operation
  families, replay, contention, trusted time, corruption, and commit faults;
- `cargo fmt --all --check`: **passed**;
- `cargo clippy --workspace --all-targets -- -D warnings`: **passed**;
- `cargo test --workspace`: **passed**;
- `npm run check`: **passed**;
- `npm run check:integrations`: **passed**;
- `npm run check:docs`: **passed**; and
- `git diff --check`: **passed**.

Focused blocker-fix validation additionally passed 40 shared conformance tests,
3 SQLite subprocess tests, 311 `workflow-core` library tests, 7 public
continuity contract tests, and 14 SQLite backend tests. Full repository
validation is rerun before the blocker-fix phase closes.

## 9. Governed Phase Record

- workflow: `dg/implement`;
- run: `run-1786845193659160000-2`;
- approval:
  `approval/run-1786845193659160000-2/implementation-approved`;
- presentation: `presentation/08cbd893c4855d36`;
- approval outcome: implementation scope approved with persisted presentation
  proof;
- phase status: completed;
- event summary: 39 events, 1 approval, 0 retries, 0 escalations;
- approval presentation enforcement: proof-enforced with one persisted
  presentation record and event marker; and
- out-of-kernel work: source inspection, repository edits, tests, validation,
  documentation, and command execution are performed by external executors
  under the governed scope. The kernel governs the phase but does not edit
  files, execute commands, schedule an agent, or mutate a provider.

## 10. Contracts And Compatibility

Semantic V2 remains additive. The existing exhaustive V1 contract enum and
provider API remain source compatible. Existing V1 SQLite data requires the
explicit upgrade; old readers reject V2. Existing filesystem and PostgreSQL
behavior remains unchanged, and no public workflow, CLI, SDK, or runtime-event
contract is widened by this phase.

Private bearer capabilities are not serialized. Durable records are limited to
bounded identifiers, commitments, revisions, enums, timestamps, and stable
references. Errors must remain bounded and must not expose database paths, SQL,
payloads, source content, command output, provider responses, credentials, or
secret-like values.

## 11. Remaining Limitations

- The original focused review remains a historical `Needs blocker fixes`
  verdict. The focused blocker-fix review accepts the corrected proof surface.
- Arbitrary database, filesystem, or VM snapshot restore remains unsupported
  without a separately governed external rollback-resistant epoch anchor.
- No runtime executor or supervisor consumes this backend yet.
- Exactly-once continuity-state mutation does not imply exactly-once external
  execution.

## 12. Recommended Next Phase

Proceed to atomic authorized-execution continuity event/state projection. Keep
host supervision and redispatch outside that projection phase.

Do not begin runtime event/state projection or an injected-supervisor
redispatch slice until that focused backend review accepts the durable SQLite
implementation.

## 13. Review Outcome

Focused maintainer/security review found blocker fixes are required. The
implementation has not yet supplied the accepted complete shared named
scenario matrix, all-five-operation commit-fault proof, or subprocess
crash/WAL recovery proof, while the code advertises complete scoped V2
support. See [SQLite Authorized Execution Continuity Backend
Review](SQLITE_AUTHORIZED_EXECUTION_CONTINUITY_BACKEND_REVIEW.md).

This forward status does not erase the implementation evidence above. It
prevents that evidence from being mistaken for final support acceptance.

## 14. Blocker Fix Forward Status

The three review blockers are now implemented:

1. one reusable named scenario matrix runs unchanged against reference and
   SQLite adapters;
2. before-, during-, and after-commit faults are exercised for every operation
   family, including consume capability-burn and reconciliation semantics; and
3. SQLite tests use one real WAL database across separate processes to prove
   crash rollback, committed ambiguity recovery, restart, and every attempt
   posture.

See [SQLite Authorized Execution Continuity Backend Blocker Fix
Report](SQLITE_AUTHORIZED_EXECUTION_CONTINUITY_BACKEND_BLOCKER_FIX_REPORT.md).
The original review verdict remains part of the historical record. Focused
re-review accepts the new evidence in [SQLite Authorized Execution Continuity
Backend Blocker Fix
Review](SQLITE_AUTHORIZED_EXECUTION_CONTINUITY_BACKEND_BLOCKER_FIX_REVIEW.md).
