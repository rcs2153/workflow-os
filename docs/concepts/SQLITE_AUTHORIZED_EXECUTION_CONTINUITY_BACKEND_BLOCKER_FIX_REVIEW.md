# SQLite Authorized Execution Continuity Backend Blocker Fix Review

## 1. Executive Verdict

**Blockers fixed; proceed to atomic continuity event/state projection.**

The bounded fix resolves all three findings from the first focused review. The
same complete named semantic matrix now runs unchanged against reference and
SQLite stores, commit faults cover every operation family at every accepted
boundary, and a real WAL database is exercised across abrupt and orderly
separate-process restart paths. SQLite has earned its scoped semantic V2
continuity declaration.

This verdict does not authorize executor integration, host scheduling,
redispatch, automatic approval, another backend, provider mutation, or hosted
operation.

## 2. Scope Verification

The fix remains inside the approved proof boundary. It adds private test-only
conformance infrastructure, shared semantic scenarios, SQLite fault adapters,
same-path subprocess restart tests, documentation, and validation evidence.

It does not add runtime events, execution-window creation, scheduler behavior,
agent redispatch, automatic gate satisfaction, external execution, provider
writes, workflow or CLI schema, default backend selection, distributed leases,
or nested harness runtime.

## 3. Shared Scenario Matrix Assessment

`ContinuityConformanceBackend` supplies only the test capabilities required to
exercise the semantic contract: normalized snapshots, same-state reopen,
trusted-time controls, deterministic commit faults, eligibility/reconciliation,
and the five store operations. It remains private and test-only.

One macro instantiates the same named scenario functions for reference and
SQLite adapters. The matrix covers:

- exact replay and changed-content conflict for every operation family;
- global operation-receipt uniqueness;
- one-winner concurrency and attempt-budget exhaustion;
- wait ownership, wake binding, and fresh-authority requirements;
- trusted-time regression, provenance, epoch, expiry, unavailability, and
  replay after later security state;
- every durable attempt posture and every reconciliation disposition;
- capability burn and authority/governance binding changes;
- closed, expired, revoked, superseded, canceled, and cursor races;
- canonical wait ordering; and
- before-, during-, and after-commit faults for every operation family.

This satisfies the accepted requirement that semantic support be proven by one
backend-parametric suite rather than parallel, potentially drifting tests.

## 4. Commit-Fault Assessment

The fault scenario invokes register-yield, transition-wait, consume-directive,
record-attempt-outcome, and recover-ambiguous-attempt under all three injected
commit boundaries.

Before/during faults require authoritative state equality with the pre-call
snapshot. After-commit acknowledgement loss requires a bounded ambiguous error
and read-only reconciliation to the expected committed operation and receipt.
The consume scenarios separately prove capability reuse after confirmed
rollback and capability non-reconstruction after durable ambiguous commit.

No partial-success or fake-evidence path was found.

## 5. Subprocess And Restart Assessment

The SQLite harness uses one physical database path in WAL mode. A child process
opens an immediate transaction, mutates the window, and aborts before commit.
The parent reopens the same path and proves rollback. A separate child commits
an operation, and a separate verifier process reopens and checks exact durable
state and replay.

Independent subprocess checks cover `started`, `yielded`, `succeeded`,
`retryable_failure`, `terminal_failure`, and
`ambiguous_may_have_started`. The proof no longer depends on exporting an
in-memory snapshot into a new database.

This satisfies the accepted crash/WAL and no-process-local-dependency boundary.

## 6. Support And Compatibility Assessment

SQLite alone advertises all five semantic V2 operations under
`local_live_state_only`, and mutation still requires live-state instance
eligibility. Existing V1 APIs remain additive and the V1-to-V2 upgrade remains
explicit. Filesystem and PostgreSQL remain unsupported.

Acceptance is scoped semantic compatibility, not default-backend selection,
arbitrary-restore certification, production hardening, or exactly-once
external execution.

## 7. Privacy And Error Assessment

The conformance and subprocess layers use bounded identifiers, stable fixture
vocabulary, commitments, and posture enums. Bearer capability material remains
private and unserialized. Errors remain code-oriented and do not expose SQL,
database paths, source contents, command output, provider payloads,
credentials, authorization headers, or token-like values.

No privacy or non-leakage blocker was found.

## 8. Test Quality Assessment

The proof is appropriately layered:

- 40 shared conformance tests exercise both adapters;
- 3 SQLite subprocess tests exercise real durable process boundaries;
- 311 `workflow-core` library tests cover integrated internal behavior;
- 7 public continuity contract tests preserve declaration compatibility;
- 14 SQLite backend tests preserve schema, migration, WAL, corruption, and
  contention behavior; and
- the complete workspace and integration gates remain green.

The private conformance module uses scoped test-only Clippy allowances for
fixtures and assertions. Production lint posture is unchanged.

## 9. Validation Evidence

The following passed on 2026-08-15:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check`;
- `npm run check:integrations`;
- `npm run check:docs`; and
- `git diff --check`.

The reviewer also inspected the shared scenario instantiation, all-five fault
helpers, SQLite conformance adapter, subprocess child modes, and same-path
parent verification directly.

## 10. Governed Review Record

- workflow: `dg/review`;
- run: `run-1786866319921413000-2`;
- approval:
  `approval/run-1786866319921413000-2/review-scope-approved`;
- presentation: `presentation/de44529693862ad3`;
- presentation hash:
  `de44529693862ad385023f2fa4bb0b518b2e945fd65be9f100e94adcd8da972b`;
- approval outcome: granted under standing delegated-maintainer authority
  after the complete persisted handoff was presented and assessed; and
- governed status: completed with 39 events, 1 approval, 0 retries, and 0
  escalations; presentation proof was enforced with one persisted record and
  event marker; and
- out-of-kernel work: source inspection, review writing, and validation were
  performed by the external executor. The kernel governed scope and approval
  but did not edit files, run commands, schedule an agent, or mutate a
  provider.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Preserve explicit upgrade and scoped live-state eligibility.
- Keep arbitrary restore unsupported until a rollback-resistant external epoch
  anchor is separately designed.
- Keep continuity-state exactly-once semantics distinct from external-effect
  exactly-once claims.
- Keep backend support conformance reusable as future backend implementations
  are proposed.

## 13. Recommended Next Phase

Implement atomic authorized-execution continuity event/state projection. The
next phase should bind durable continuity mutations and bounded runtime event
projection without yet adding a host supervisor, redispatch loop, automatic
approval, or external execution.

Only after that atomic projection is accepted should Workflow OS implement one
local injected trusted-host supervisor vertical slice.
