# Trusted Host TimeWindow Wait Binding Blocker Fix Review

## 1. Executive Verdict

Blockers fixed; proceed to the first private trusted-host caller integration.

The merged fix makes the exact deadline check and authoritative transition use
one transaction-owned trusted-time observation. It also restores exact replay
after an ambiguous successful commit without weakening stale or conflicting
request rejection. No remaining blocker was found in the approved fix scope.

## 2. Scope Verification

The fix stayed within the private SQLite verifier boundary. It changed the
transaction helper's private mutation protocol, added a stable private request
commitment, specialized the `TimeWindow` transition, and added focused tests and
reports.

It did not add a caller, scheduler, poller, executor recirculation, provider or
sandbox integration, public configuration, CLI, SDK, hosted behavior, another
wake-source family, or release-posture change.

## 3. Atomic Trusted-Time Assessment

The detached wrapper clock read is removed. For a new operation, the SQLite
transaction helper obtains one trusted-time observation after acquiring the
immediate transaction. Security classification and commit mutation both use
that observation.

The specialized commit phase checks the durable `TimeWindow` binding and
requires the transaction observation to be at or after the exact bound
deadline. It then derives the private wake capability's source commitment and
source revision from the same state and observation before atomically updating
the wait and window.

Preflight runs against a cloned snapshot and does not claim deadline
satisfaction. It validates deterministic structure only. A caller-supplied time
assertion cannot authorize the transition.

## 4. Replay Assessment

The specialized request now has a stable domain-separated commitment covering:

- operation and receipt identity;
- window identity, expected revision, and exact binding commitment;
- cursor sequence and event identity;
- wait condition identity and version;
- active generation identity;
- expected wait revision; and
- the fixed `Satisfied` transition target.

The transaction checks an existing operation record before evaluating current
wait eligibility. An exact request therefore recovers the committed disposition
after an ambiguous acknowledgement. A changed receipt or request commitment is
rejected as a replay conflict. A distinct stale operation reaches normal
revision and state validation and fails closed.

The commitment intentionally does not include a time observation supplied by a
caller. The authoritative observation is generated inside the transaction and
is committed by the operation record.

## 5. Concurrency Assessment

SQLite `Immediate` transaction serialization, exact expected revisions, and
operation-record uniqueness preserve one-winner semantics. The direct
private-verifier concurrency test proves that two distinct competing operations
cannot both transition the same wait. The losing request does not create a
second successful transition.

## 6. Binding And Corruption Assessment

The verifier recomputes the durable dependency commitment from the exact
deadline, trusted-time source, provenance commitment, and epoch before use. It
also requires those fields to match the current trusted-time root and requires
the exact window, cursor, generation, condition version, and wait revision.

Legacy unbound deadline waits remain readable but cannot transition. No
migration inference or synthetic authority was introduced.

## 7. Privacy And Error Assessment

The fix stores and compares typed identifiers, timestamps, revisions, and
commitments. Stable errors do not include deadlines, provenance values, epoch
values, paths, tokens, commands, source contents, or provider payloads. Debug
and serialization posture is unchanged and bounded.

## 8. Test Quality Assessment

Focused tests now cover:

- exactly one transaction observation for a new transition;
- exact replay after an injected post-commit acknowledgement failure;
- conflicting replay rejection;
- one winner under two competing private-verifier calls; and
- continued rejection of legacy unbound deadline waits.

The complete workspace suite passed before merge, including 365 core unit tests,
343 local-executor tests, persistence and migration tests, provider-write tests,
and report tests. All seven required GitHub checks passed on pull request 503.

## 9. Blockers

None in the approved blocker-fix scope.

## 10. Non-Blocking Follow-Ups

- Keep the first caller private and limited to exact `TimeWindow` waits.
- Preserve the verifier as the only authority-construction boundary; the caller
  must not construct wake capability or time evidence.
- Add caller-level restart and stale-binding tests when the caller exists.
- Keep additional wake-source families separate.
- Local SQLite remains non-tamper-resistant against a privileged local actor.

## 11. Recommended Next Phase

Implement the first private trusted-host caller integration for exact
`TimeWindow` waits. The caller should obtain current durable identity and
revision inputs, invoke the accepted verifier, handle exact replay and stable
failure without state surgery, and expose no public scheduler or configuration
surface.

Provider mutation broadening, sandbox integration, nested harness work, hosted
scheduling, and additional wake-source families remain out of scope.

## 12. Validation

- `cargo test -p workflow-core --lib sqlite_time_window_verifier`: passed,
  5 tests.
- `cargo fmt --all --check`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.
- Pre-merge `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- Pre-merge `cargo test --workspace`: passed.
- Pull request 503: all 7 required GitHub checks passed; squash merge completed.

## 13. Governed Review Record

- workflow: `dg/review`
- run: `run-1791041572822656000-2`
- approval: `approval/run-1791041572822656000-2/review-scope-approved`
- presentation: `presentation/30d1f561213666e1`
- presentation hash:
  `30d1f561213666e1e0a512e8fed1e261fb1c1fe85ac3d5280e1b11e80b288bbf`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- review status: accepted
- phase status: completed
- event summary: 39 events, including 1 approval request, 1 approval grant,
  8 policy decisions, 6 scheduled steps, 6 successful skill invocations,
  0 retries, and 0 escalations; approval-presentation proof enforced
- validation summary: 5 focused verifier tests, formatting, documentation
  checks, and diff hygiene passed; the prior full workspace clippy, workspace
  tests, and 7 required GitHub checks were reviewed as supporting evidence
- out-of-kernel work: code reading, security analysis, focused validation, and
  review documentation were performed by the delegated trusted host; Workflow
  OS governed scope and approval but did not perform those actions
