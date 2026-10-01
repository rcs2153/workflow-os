# Operational Execution Window Opening Review

## 1. Executive Verdict

**Needs blocker fixes.**

The opening-only implementation remains private, local, SQLite-only, and
separate from trusted-host invocation. Its immediate transaction creates one
window, first attempt, event, snapshot, operation record, and projection
binding without an intermediate schedulable posture. Those are strong
foundations.

The phase cannot yet be accepted as an execution-authority boundary. The
request commitment does not bind all decision-relevant opening input, the
public serialized projection does not revalidate its derived commitments, and
required contention and commit-fault proofs are missing. Do not implement the
trusted-host supervisor until these blockers are fixed and reviewed.

## 2. Scope Verification

The implementation stayed within the approved opening-only scope. It did not
add:

- a trusted-host supervisor, scheduler, daemon, or polling loop;
- skill, tool, provider, sandbox, or OpenShell invocation;
- automatic or delegated approval;
- new provider mutations;
- CLI, SDK, workflow schema, runtime config, or examples;
- filesystem or PostgreSQL opening support;
- nested harness execution, Reasoning Lineage, hosted runtime, or release
  posture changes; or
- changes to the accepted closed five-operation continuity contract.

## 3. Authority Boundary Assessment

Core rehydrates the durable run and enters the registered current-authority
source's same-call use before constructing the private opening authorization.
Blocked, stale, missing, or failed current authority cannot call the store.
The authorization and returned attempt-use capability are private,
non-serializable, and redaction-safe.

The exact operation vocabulary is correctly closed to
`invoke_current_step_skill`. Caller-authored readiness or serialized authority
is not accepted as permission.

This boundary is directionally correct, but the durable request binding is
not yet complete enough to prove exactly what the private authority opened.

## 4. Atomic Persistence Assessment

The SQLite implementation uses one immediate transaction and does not persist
an intermediate `assessment_required` posture. On the ordinary success path it
atomically:

1. validates the request;
2. rehydrates the current run and cursor;
3. rejects an active-window conflict;
4. advances trusted time;
5. creates the `executing` window;
6. creates the first `started` opening attempt;
7. appends the bounded runtime event and derives the snapshot;
8. persists operation, attempt, and projection records; and
9. commits once.

The transaction shape is appropriate. The implementation returns a private
attempt-use capability only on a newly opened result, not on replay or
reconciliation.

## 5. Blocker: Incomplete Request Commitment

`opening_request_commitment` binds operation, workflow, run, step, window,
attempt, and operation-binding identities. It does not bind the receipt,
expected cursor or snapshot, subject actor, immutable run bundle, source
authority, Core governance commitment, expiry, trusted-time observation and
epoch, or maximum-attempt budget.

This is blocker-level because exact replay is evaluated before the current run
and request are revalidated. Reusing an operation ID and receipt with changed
omitted input can therefore return `ExactReplay` rather than fail as
same-key/different-content. Replay returns no capability, which limits the
immediate blast radius, but the operation receipt and reconciliation boundary
no longer prove the exact request that was made.

The fix must introduce a new domain-separated request-commitment version that
binds every decision-relevant bounded input or its canonical commitment. It
must add substitution tests for each field family and prove that exact replay
requires exact content.

## 6. Blocker: Missing Snapshot Binding

The reviewed plan requires both the expected runtime cursor and expected
snapshot commitment. The implemented request carries only the cursor. The
transaction rehydrates a run from event history, but it does not compare the
current derived snapshot to a caller/Core-bound expected snapshot commitment.

The fix must add a canonical expected snapshot commitment derived by Core,
persist and bind it in the opening request/operation, and reject snapshot
substitution before mutation.

## 7. Blocker: Serialized Projection Integrity

`OperationalExecutionWindowOpeningProjectionEvent` derives `Deserialize`.
Deserialization validates nested identifier types and unknown fields, but it
does not recompute the request commitment, operation-binding commitment,
projection commitment, cursor relationship, fixed attempt number, or fixed
window revision. The constructor also accepts supplied commitment values
without checking their canonical relationship.

A serialized event can therefore be internally inconsistent while still
deserializing. Because the event participates in durable rehydration and
snapshot projection, this is not merely a display concern.

The fix must use validated deserialization or an explicit `validate` boundary
that is guaranteed on every durable read. Canonical projection commitments
must bind all disclosed fields, including cursor sequence numbers, request
commitment, attempt number, and window revision. Tampering tests must cover
each derived relationship without leaking supplied values.

## 8. Replay And Reconciliation Assessment

Exact replay creates no additional event or attempt, and fresh-connection
reconciliation can recover a complete committed result. Receipt mismatch is
rejected. These are useful proofs.

Acceptance remains blocked until the request commitment is complete and
before/during/after commit-fault behavior is demonstrated. In particular, the
implementation maps commit failure to ambiguity but has no opening-specific
fault injection proving:

- pre-commit failure leaves zero opening, continuity, event, or snapshot
  writes;
- an ambiguous acknowledgement returns no attempt capability; and
- a fresh connection distinguishes durable commit from confirmed absence.

## 9. Concurrency Assessment

`TransactionBehavior::Immediate`, the active-window query, unique constraints,
and shared event cursor are sensible mechanisms. The required proof is absent.

Before acceptance, tests must demonstrate:

- two concurrent openers at the same cursor produce exactly one winner;
- a generic event append and opening projection contend for one next cursor;
- the losing path returns a stable non-leaking error and writes nothing; and
- reopening the database preserves the one-winner result.

## 10. Migration Assessment

Schema V4 is separately versioned and does not widen the closed continuity
operation-kind constraint. V3 upgrade is explicit and refuses continuity
window/attempt state rather than inventing opening authority. Existing schema
verification and migration tests continue to cover explicit upgrade and
tamper refusal.

The blocker fix should add opening-specific interruption/rollback coverage if
the commit-fault harness touches schema setup. No migration should synthesize
an opening operation or attempt for pre-opening state.

## 11. Privacy And Redaction Assessment

The new records contain bounded identities, timestamps, revisions, and
commitments rather than prompts, transcripts, source/spec bodies, command
output, provider payloads, environment values, paths, credentials, approval
reasons, or evidence bodies. Debug implementations redact binding material,
and secret-like opening identifiers are rejected without echoing values.

Privacy posture is acceptable for the implemented fields. The blocker fix
must preserve payload-free commitments and stable non-leaking errors while
adding the missing bindings.

## 12. Test Quality Assessment

Existing focused tests cover ordinary atomic open, exact replay, receipt
conflict, stale cursor, operation substitution, fresh-connection
reconciliation, identifier privacy, serialization shape, and V3 migration
refusal.

Missing blocker-level tests are:

- all omitted request-field substitutions on replay;
- expected snapshot commitment mismatch;
- actor, immutable-bundle, authority/governance, expiry, trusted-time, and
  attempt-budget mismatch;
- canonical projection deserialization tampering;
- concurrent opening with exactly one winner;
- generic-event/opening cursor contention; and
- before/during/after commit fault and no-capability-on-ambiguity behavior.

## 13. Validation Reviewed

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- Operational opening model/store tests: passed.
- `cargo test -p workflow-core --test sqlite_state_backend`: passed, 17
  tests.
- `npm run check:docs`: passed.
- `npm run check:integrations`: passed.
- `git diff --check`: passed.
- `cargo test --workspace -j 2`: compiled the full workspace and passed the
  CLI/core unit suites and multiple integration suites before being stopped
  because this host imposed multi-minute launch latency per test binary. No
  failure was observed; a complete workspace pass is not claimed.

## 14. Blockers

1. Bind every decision-relevant opening input in a new request-commitment
   version and reject changed omitted content as conflict before replay.
2. Add and enforce the expected durable snapshot commitment.
3. Make opening projection deserialization and derived commitments fail closed
   on all internal inconsistency.
4. Add concurrent-opener and generic-event cursor-contention proof.
5. Add opening-specific before/during/after commit-fault proof and demonstrate
   no attempt capability on ambiguity.

## 15. Non-Blocking Follow-Ups

- Keep the first opening attempt separate from accepted continuity attempts
  until the outcome/yield bridge is separately reviewed.
- Consider reducing internal test-binary count or documenting the macOS host
  launch-latency issue; it affects validation time, not kernel semantics.
- Add filesystem and PostgreSQL support only after the SQLite capability and
  host boundary are accepted.

## 16. Recommended Next Phase

**Operational execution-window opening blocker fix.**

Fix commitment completeness, snapshot binding, serialized projection
integrity, concurrency proof, and commit-fault proof. Then perform a focused
blocker-fix review. Do not implement the trusted-host supervisor, dispatch a
skill, or broaden mutation capabilities in the blocker phase.

## 17. Governed Review Record

- workflow: `dg/review`;
- run: `run-1790841393542308000-2`;
- approval: `approval/run-1790841393542308000-2/review-scope-approved`;
- presentation: `presentation/116c32d602e1b034`;
- presentation hash:
  `116c32d602e1b03479d95b1a56a16701fb7a559b72788669b8314396e5203d5a`;
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted review handoff was presented; and
- phase close: `Completed`, 39 events, 1 approval, 0 retries, and 0
  escalations, with the approval-presentation proof marker present; and
- out-of-kernel work: the external executor inspected implementation, tests,
  plans, and validation results and authored this review. The kernel did not
  edit files, run checks, commit source, invoke a skill, or mutate a provider.

## Fix-Forward Note

The blocker remediation is implemented and documented in
[Operational Execution Window Opening Blocker Fix Report](OPERATIONAL_EXECUTION_WINDOW_OPENING_BLOCKER_FIX_REPORT.md).
This note does not erase or revise the original findings. The repaired boundary
still requires a focused blocker-fix review before trusted-host invocation may
begin.
