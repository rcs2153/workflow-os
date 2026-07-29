# Filesystem-To-SQLite Writer Quiescence And Import Transaction Plan Review

Review date: 2026-07-28

## 1. Executive Verdict

**Plan accepted with one inline correction; proceed to the writer guard and
compatibility capability model.**

The plan defines a conservative, local-only safety boundary before any
filesystem-to-SQLite destination write. It correctly rejects current per-key
lock records as proof of quiescence, limits its guarantee to cooperating
writers, keeps the destination unreachable, uses one atomic v1 import
transaction, and separates import, verification, and activation.

Review found one planning-level binding omission: exact resume did not bind the
writer, guard, and importer-transaction protocol versions. The plan now
requires one immutable migration-attempt fingerprint across authority,
staging, resume, verification, and receipt boundaries.

## 2. Scope Verification

The phase stayed planning-only.

It added no:

- writer lock or process coordination;
- importer, projection rebuild, or destination write;
- SQLite schema or staging constructor;
- source mutation, repair, archive, rename, or deletion;
- verification receipt or activation;
- backend selection, CLI, schema, SDK, or example behavior;
- provider, hosted, collaborative, or release behavior.

## 3. Current-State Assessment

The plan accurately describes `LocalStateBackend`:

- logical locks are keyed directories;
- mutations do not all acquire one root-wide lock;
- leases are unfenced and unexpired;
- an empty `locks/` directory cannot prove source quiescence.

It also correctly identifies that ordinary `SqliteStateBackend::open` creates
`ready` schema metadata and is therefore not a safe importer entrypoint.

## 4. Writer Exclusion Assessment

The shared-writer/exclusive-migration protocol is appropriate for the local
preview boundary.

The plan requires coverage for every current filesystem mutation entrypoint and
retains the exclusive guard through source recheck and receipt creation. It
rejects marker polling and check-then-create protocols as racy.

The guarantee is stated honestly: it covers cooperating local Workflow OS
writers, not older binaries, hostile processes, distributed workers, or
network filesystems.

## 5. Authority And Attempt-Binding Assessment

Migration authority is narrower than activation authority and binds the exact
source, destination, and plan.

The inline correction adds the missing immutable attempt binding:

- plan fingerprint;
- source and destination;
- writer-protocol version;
- guard-protocol version;
- importer-transaction version;
- adapter schema version.

This prevents exact resume from reinterpreting staging state after a protocol
change.

## 6. Transaction Assessment

One SQLite `BEGIN IMMEDIATE` transaction is the smallest safe v1 import
boundary.

It provides:

- no committed partial record-family state;
- restart from the first family after pre-commit interruption;
- one committed `imported_unverified` staging state;
- post-commit verification without runtime visibility.

Per-family commits and row-count-derived resume are correctly excluded.

## 7. Interruption And Recovery Assessment

Every identified interruption point has a deterministic disposition.

Exact empty staging may be reused. An uncommitted transaction restarts.
Committed unverified staging resumes verification without re-import.
Conflicting or unknown staging state requires explicit operator recovery.
Automatic staging deletion is not allowed.

## 8. Verification And Activation Assessment

The plan carries forward all accepted verification obligations and runs them
after atomic import commit while source quiescence remains held.

A failed verification leaves staging inactive and emits no successful receipt.
Receipt creation does not activate the backend. Activation remains a separate
future authority, source recheck, destination recheck, and auditable decision.

## 9. Privacy And Error Assessment

The plan is payload-free and path-redacted. It prohibits raw record JSON,
workflow contents, provider payloads, command output, environment values,
credentials, authorization headers, private keys, and tokens.

The proposed error taxonomy is stable and does not require sensitive caller
values.

## 10. Test Assessment

The future test plan covers:

- every mutation entrypoint;
- real cross-process exclusion rather than thread-only proof;
- process termination;
- source change detection;
- atomic rollback and post-commit resume;
- protocol-version mismatch;
- verification and activation separation;
- source and companion preservation;
- non-leaking output.

No additional blocker-level test category is missing.

## 11. Blockers

None after the migration-attempt binding correction.

## 12. Non-Blocking Follow-Ups

- Select and justify the cross-platform advisory-lock dependency in the
  implementation phase.
- Decide whether preview writers use shared guards or one simpler exclusive
  root-wide mutex after measuring complexity and contention.
- Specify how an operator attests that incompatible older writers are stopped.
- Keep lock acquisition helpers non-reentrant or explicitly structure guarded
  and unguarded internal write methods.

## 13. Recommended Next Phase

Implement the **writer guard and compatibility capability model only**.

The model should represent:

- writer, guard, and importer-transaction protocol versions;
- shared-writer and exclusive-migration modes;
- bounded acquisition outcomes;
- compatibility posture;
- immutable migration-attempt fingerprint;
- redaction-safe validation and serde.

It must not acquire a filesystem lock, create SQLite, import records, mutate
source, verify a destination, activate a backend, or expose CLI behavior.

## 14. Governed Review Record

- workflow: `dg/review`;
- run ID: `run-1785305333555857000-2`;
- approval ID:
  `approval/run-1785305333555857000-2/review-scope-approved`;
- presentation ID: `presentation/dbdaeb4e21b0e3df`;
- approval outcome: granted by delegated maintainer with persisted proof;
- phase status: `Completed`;
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations;
- approval presentation enforcement: `proof_enforced`.

Review, correction, validation, git, and PR work occurred outside the kernel.
The kernel governed scope and approval sequencing; it did not execute those
operations.
