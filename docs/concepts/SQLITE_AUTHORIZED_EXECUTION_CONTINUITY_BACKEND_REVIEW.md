# SQLite Authorized Execution Continuity Backend Review

## 1. Executive Verdict

**Needs blocker fixes.**

The SQLite semantic V2 implementation is substantial and the repository is
green, but the backend has not yet earned its advertised support declaration.
The accepted plan requires one complete set of named backend-parametric
scenarios to pass unchanged against the reference and SQLite stores, plus
SQLite subprocess crash/WAL recovery and all-five-operation commit-fault
proof. The implementation currently supplies a shared adapter interface and a
smaller SQLite-specific suite instead.

Do not proceed to runtime event/state projection or supervisor redispatch
until the conformance proof is completed or SQLite V2 support is withdrawn.

## 2. Scope Verification

The implementation stayed within the approved durable-backend scope. It adds
SQLite schema V2, an explicit V1-to-V2 upgrade, trusted-time state, the five
continuity transactions, replay and reconciliation, test adapters, focused
tests, and documentation.

It does not add runtime event projection, execution-window opening, host
scheduling, agent redispatch, automatic approval, provider writes, CLI or
workflow-schema exposure, default-backend selection, hosted execution,
distributed leases, nested harnesses, or agent teams.

## 3. Schema And Upgrade Assessment

Schema V2 is additive and physically validated through a stable manifest
digest. Fresh databases initialize V2 explicitly. Existing exact V1 databases
fail ordinary open with `state.sqlite.schema.upgrade_required` and may be
upgraded only through the named
`upgrade_authorized_execution_continuity_v1_to_v2` operation.

The upgrade uses an immediate transaction, validates the exact V1 metadata and
manifest, applies the continuity schema, initializes trusted time, updates
managed metadata and `user_version`, revalidates V2, and commits once.
Concurrent upgrades serialize and an already-valid V2 database is idempotent.
Unknown, incomplete, drifted, or unmanaged shapes fail closed.

## 4. Atomic Operation Assessment

All five operation families are implemented behind immediate SQLite
transactions:

- register yield;
- transition wait;
- consume directive and start attempt;
- record attempt outcome; and
- recover ambiguous attempt.

The transactions use shared semantic preparation helpers, write the domain
records and operation receipt together, and reload the normalized snapshot
before commit. Exact replay is checked before trusted-time observation.
Changed-content replay, stale revisions, identity mismatches, ineligible
instances, and corrupted projections fail closed with bounded errors.

## 5. Trusted Time, Replay, And Reconciliation

The backend persists one trusted-time singleton with source kind, provenance,
epoch, watermark, posture, eligibility, and revision. Regression, provenance
mismatch, and epoch mismatch quarantine the instance through committed
security rejection. Expiry closes the affected window without globally
quarantining healthy time. Unavailable time rolls back.

Read-only reconciliation distinguishes durably committed, confirmed absent,
and unreadable state while requiring both expected operation commitment and
receipt identity. It does not write or reconstruct consumed authority.

## 6. Conformance Blockers

### Blocker 1: advertised support precedes the accepted shared scenario proof

The public V2 provider currently returns `Supported` for every operation in
`LocalLiveStateOnly`. The provider contract itself says a backend implements
that trait only after the scoped V2 conformance suite passes.

The new `ContinuityConformanceBackend` module defines only an adapter trait.
It contains no reusable named scenario functions. Reference-store scenarios
remain in `authorized_execution_continuity_state.rs`, while the SQLite
scenarios are independently authored in `sqlite_state/continuity_store.rs`.
Consequently, the same semantic matrix is not instantiated unchanged against
both stores as required by the accepted plan.

Required fix: extract the accepted matrix into reusable named scenario
functions and instantiate every applicable scenario independently for the
reference and SQLite adapters. Until that passes, either withdraw the SQLite
V2 support declaration or keep the blocker branch unmerged.

### Blocker 2: commit-fault proof covers only register-yield

The SQLite fault test injects before-, during-, and after-commit faults only
for `RegisterYield`. The accepted matrix requires those postures for all five
operation families, including consume-by-value authority burn, wait state,
attempt outcomes, and ambiguity recovery.

Required fix: run the shared fault scenarios for every operation and prove the
exact state delta, rollback posture, ambiguous-commit result, and read-only
reconciliation outcome. Consume tests must prove authority is neither reused
after a committed ambiguous return nor lost after a confirmed rollback.

### Blocker 3: restart proof is not a subprocess crash/WAL test

The current continuity restart test exports a normalized in-memory snapshot
and creates a new temporary SQLite database from it. That tests codec parity,
but it does not reopen the same durable database after process loss and cannot
prove WAL recovery or the absence of process-local dependencies.

The accepted plan explicitly requires close/reopen, subprocess restart,
subprocess crash/WAL recovery, and restart from every attempt posture.

Required fix: add a portable subprocess harness that operates on one real
database path, terminates at deterministic transaction boundaries, reopens
through a separate process, and verifies authoritative state, replay,
reconciliation, trusted time, and orphaned-started-attempt posture.

## 7. Privacy And Error Assessment

The implementation stores bounded identifiers, commitments, revisions,
timestamps, enums, and canonical record envelopes. Bearer capabilities remain
private and are not serialized. SQLite errors are mapped to stable bounded
codes and messages. Corruption tests use canaries and verify that persisted
values are not echoed.

No raw provider payloads, command output, source contents, prompts,
environment values, credentials, authorization headers, private keys, or
token-like values are added by this phase.

## 8. Compatibility Assessment

The V1 contract enum and provider API remain source compatible. Semantic V2 is
additive. Existing V1 SQLite databases require deliberate upgrade, old readers
reject V2, and filesystem/PostgreSQL V2 continuity remains unsupported. No
workflow, CLI, SDK, event, or report schema is widened.

The support declaration is the only unacceptable compatibility posture: it
communicates earned semantic support before the accepted proof suite exists.

## 9. Test Quality Assessment

Existing focused tests provide useful evidence for:

- exact V1-to-V2 upgrade and concurrent upgrader serialization;
- schema drift, unmanaged state, and corruption rejection;
- all five happy-path operation families;
- exact register-yield and consume replay;
- register-yield contention;
- trusted-time unavailability, quarantine, epoch/provenance mismatch, and
  expiry;
- register-yield commit faults and reconciliation;
- managed-restore ineligibility; and
- public support-contract shape.

They do not satisfy the complete matrix in the accepted plan. Missing shared
proof includes same-key conflict for every operation, global receipt reuse,
attempt-budget contention, wait/cursor/terminal races, all attempt outcomes,
all-five-operation fault stages, every reconciliation disposition, replay
after later trusted-time states, both writer lock orders, malicious decode
coverage across record families, and real subprocess crash/WAL recovery.

## 10. Validation Evidence

Implementation-owner validation passed on 2026-08-15:

- focused public continuity contract tests: 7 passed;
- focused SQLite backend tests: 14 passed;
- focused SQLite operation tests passed;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check`;
- `npm run check:integrations`;
- `npm run check:docs`; and
- `git diff --check`.

Green validation demonstrates implementation health but does not substitute
for missing required security scenarios.

## 11. Governed Review Record

- workflow: `dg/review`;
- run: `run-1786852231048910000-2`;
- approval:
  `approval/run-1786852231048910000-2/review-scope-approved`;
- presentation: `presentation/407dd7fa36524f83`;
- presentation hash:
  `407dd7fa36524f83095365623a85ec3d63d624febca510628977096e8f50ab24`;
- approval outcome: granted under standing delegated-maintainer authority
  after the complete proof-enforced handoff was presented and assessed; and
- governed status: completed with 39 events, 1 approval, 0 retries, and 0
  escalations; approval presentation proof was persisted and marked in the
  event trail; and
- out-of-kernel work: source inspection, review, documentation, validation,
  git, and PR actions are performed by the external executor. The kernel does
  not edit files, run tests, open a process, schedule an agent, or mutate a
  provider.

## 12. Blockers

1. Build and run the complete shared named backend-parametric conformance
   matrix against reference and SQLite.
2. Cover before-, during-, and after-commit fault posture for all five
   operations, including consume authority-burn semantics.
3. Add real close/reopen and subprocess crash/WAL recovery proof from every
   relevant attempt posture.
4. Do not advertise SQLite semantic V2 support until those proofs pass and the
   blocker-fix review accepts them.

## 13. Non-Blocking Follow-Ups

- Preserve explicit V1-to-V2 upgrade rather than adding silent migration.
- Keep arbitrary restored state ineligible until an external rollback-resistant
  epoch anchor is separately designed.
- Keep exactly-once continuity-state mutation distinct from exactly-once
  external effects.
- Keep runtime event projection and host supervision out of the blocker fix.

## 14. Recommended Next Phase

Execute a focused SQLite continuity conformance blocker fix. Refactor the
accepted semantic scenarios into one reusable named suite, instantiate it for
reference and SQLite, add the missing SQLite subprocess and fault proofs, and
re-run full validation.

Only after focused blocker-fix review accepts the durable backend should the
roadmap proceed to atomic continuity event/state projection and one local
injected-supervisor vertical slice. Provider mutation broadening, nested
harness runtime, automatic approval, CLI/schema exposure, and hosted execution
remain deferred.

## 15. Fix-Forward Note

The bounded blocker fix is now implemented and documented in [SQLite
Authorized Execution Continuity Backend Blocker Fix
Report](SQLITE_AUTHORIZED_EXECUTION_CONTINUITY_BACKEND_BLOCKER_FIX_REPORT.md).
It adds the complete shared named matrix for reference and SQLite, all-five
before/during/after commit-fault proof, and same-path subprocess crash/WAL and
restart proof for every durable attempt posture.

This note does not erase or retroactively change the original **Needs blocker
fixes** verdict. A separate focused blocker-fix review must independently
assess the new evidence before SQLite semantic V2 is accepted and before
runtime event/state projection begins.
