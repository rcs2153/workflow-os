# SQLite Authorized Execution Continuity Backend Plan Review

## 1. Executive Verdict

**Needs planning blocker fixes.**

The plan has the correct product boundary and durable-state direction, but it
does not yet authorize implementation. Restore detection, committed security
rejection semantics, exact SQLite schema integrity, and commit-ambiguity
reconciliation require explicit contracts first.

SQLite correctly remains unsupported.

## 2. Scope Verification

The phase stayed within planning scope. It added no Rust behavior, schema,
backend support, runtime event, executor, scheduler, automatic approval,
provider write, PostgreSQL support, CLI/schema exposure, nested execution, or
release change.

## 3. Accepted Direction

The review accepts these foundations:

- SQLite is the first durable continuity backend and remains local-only.
- All five operation families use one `BEGIN IMMEDIATE` transaction.
- Committed operation history is checked before a new clock observation.
- Exact consume replay never reconstructs attempt authority.
- One private backend-parametric conformance harness gates support.
- SQLite support is all-or-none for this first phase.
- Local filesystem and PostgreSQL remain unsupported.
- Runtime event/state projection and supervisor integration remain later.

## 4. Blocker One: Restore Detection

The plan says a restored V2 database must remain quarantined. Database-local
epoch and watermark state is copied with the database, so the database cannot
detect its own rollback or restore. A local wall clock does not supply a
rollback-resistant external anchor.

Required fix:

- either define a non-copied, rollback-resistant external anchor bound to the
  database identity and trusted-time epoch; or
- explicitly exclude arbitrary restore safety from this phase and ensure no
  continuity authority is advertised after an unverified restore.

The plan must not claim automatic restore quarantine without an implementable
detection source.

## 5. Blocker Two: Committed Security Rejection

The accepted reference contract rejects unavailable, regressed, untrusted, or
expired time without domain writes. The new plan proposes committing quarantine
or expiry security state and then returning an error. A normal error path would
roll back; committing without a durable operation/rejection record breaks exact
replay.

Required fix:

- define explicit transaction dispositions such as committed success,
  committed security rejection, and rolled-back failure;
- persist a bounded rejection commitment and exact replay material for every
  committed security rejection;
- add trusted-time epoch to the observation and operation commitment; and
- update the reference semantics and shared conformance before SQLite support.

## 6. Blocker Three: Exact V2 Schema

Candidate table names and high-level constraints are insufficient for a
security review. The plan must provide exact DDL or an equivalent complete
schema specification.

It must define:

- every column, type, nullability rule, enum check, key, foreign key, partial
  uniqueness constraint, and encoded-envelope bound;
- checksum derivation and schema-shape validation;
- `foreign_key_check` and relational identity health validation;
- exact empty-V0 detection;
- ready-V1 eligibility and staging/inactive-V1 rejection;
- behavior of new filesystem migration plans;
- concurrent open and upgrade outcomes; and
- rollback/fault tests for the migration transaction.

## 7. Blocker Four: Commit Ambiguity Protocol

The plan recognizes commit-return ambiguity but does not define an executable
caller protocol.

Required fix:

- add stable `state.sqlite.commit_ambiguous` behavior;
- discard the affected transaction and connection;
- reconcile on a fresh connection using the exact operation identity;
- distinguish durable committed result, confirmed absence, and unreadable
  state; and
- prohibit executor entry when a replayed consume reveals that the attempt was
  already durably started without returning the original one-use capability.

## 8. Conformance And Privacy Follow-Ups

Before implementation, the plan must also specify:

- expected state deltas for committed rejection versus rolled-back error;
- replay integrity checks over operation kind, request/result commitments,
  receipt identity, trusted-time observation, and relational identity;
- concurrent trusted-time and migration scenarios;
- subprocess crash/WAL recovery;
- malformed schema-object and foreign-key failures;
- an explicit storage whitelist; and
- canary scanning proving capabilities, paths, SQL, payloads, and secret-like
  values do not enter database bytes, errors, or `Debug`.

## 9. Validation Assessment

Planning validation passed:

- `npm run check:docs`; and
- `git diff --check`.

No implementation tests were expected or claimed.

## 10. Governed Review Record

- workflow: `dg/review`;
- run: `run-1786815599920956000-2`;
- approval: `approval/run-1786815599920956000-2/review-scope-approved`;
- presentation: `presentation/644ffdfac25b1a8e`;
- presentation hash:
  `644ffdfac25b1a8e2780bbd86ccb83df93f4745394f83b2d49ea85dc13e58329`;
- approval outcome: granted by delegated maintainer after complete handoff;
- independent reviewer: read-only focused security review; and
- out-of-kernel work: source inspection, review analysis, documentation, and
  validation were performed by external executors.

## 11. Recommended Next Phase

Perform a focused **SQLite continuity plan blocker-fix phase**.

That phase must resolve the four blockers in documentation and shared semantic
contract sequencing. It must not implement SQLite transactions or advertise
support. After re-review accepts the corrected plan, begin implementation with
the shared committed-rejection/trusted-time contract and backend-parametric
conformance extraction before changing SQLite schema.

## 12. Fix-Forward Status

The planning blockers are addressed in the corrected plan and documented in
the [blocker-fix
report](SQLITE_AUTHORIZED_EXECUTION_CONTINUITY_BACKEND_PLAN_BLOCKER_FIX_REPORT.md).
This does not erase the original verdict. A focused re-review is still required
before implementation.
