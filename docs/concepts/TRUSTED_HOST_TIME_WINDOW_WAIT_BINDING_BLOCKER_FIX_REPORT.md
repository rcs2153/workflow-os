# Trusted Host TimeWindow Wait Binding Blocker Fix Report

## 1. Executive Summary

The two blockers from the initial maintainer/security review are fixed. The
private SQLite `TimeWindow` verifier now enforces the exact bound deadline from
the same trusted-time observation used by the atomic transition. Its stable
request commitment also permits exact replay to recover an ambiguous successful
commit before current-state eligibility checks reject the already-satisfied
wait.

This remains private, local SQLite prerequisite work. No caller, scheduler,
poller, executor recirculation, provider integration, sandbox integration,
public configuration, CLI, SDK, hosted behavior, or new wake-source family was
added.

## 2. Blockers Fixed

1. **Detached time observation:** removed the wrapper's pre-transaction clock
   read. The specialized transition now receives the one transaction-owned
   observation and checks `observed_at >= deadline` during the commit phase.
2. **Replay blocked by wrapper preflight:** introduced a stable private
   `TimeWindow` transition request commitment and routes the specialized
   transition through operation replay before evaluating current wait state.
   Exact retry returns the committed result; changed receipt or request identity
   fails closed.

## 3. Implementation Approach

The SQLite transaction helper now identifies preflight and commit mutation
phases. Preflight still validates deterministic structure against a cloned
snapshot without authorizing time satisfaction. Commit receives the actual
trusted-time observation acquired after the immediate transaction starts and
performs the deadline check before mutating the wait and window.

The specialized verifier remains one auditable transaction boundary. It
validates the exact window revision and binding, cursor, generation, wait
revision and state, trusted-time source, provenance, epoch, and recomputed
dependency commitment. It then records the source commitment and source
revision derived from the same committed observation.

The stable request commitment binds the operation and receipt identities,
window binding and revision, cursor, exact wait identity and version,
generation, expected wait revision, and the fixed `Satisfied` target. It does
not bind a caller-supplied or pre-transaction time assertion.

## 4. Atomicity And Replay Boundary

- One trusted-time observation is made for a new transition operation.
- Deadline satisfaction and the recorded trusted-time commitment use that same
  observation.
- SQLite `Immediate` transaction serialization and expected revisions preserve
  one-winner behavior for competing operations.
- Exact operation replay occurs before current-state eligibility checks and does
  not re-observe time.
- Conflicting replay and stale competing operations fail closed.

## 5. Privacy And Error Posture

The fix adds no raw timestamps, provenance values, epoch values, provider
payloads, paths, tokens, command output, or source contents to errors or Debug
output. Existing bounded stable error codes remain in use. The request
commitment and operation record remain payload-free hashes and typed identity
references.

## 6. Test Coverage

Focused regression tests prove:

- the private verifier performs one transaction observation;
- a second hypothetical regressed observation is never consulted;
- exact retry recovers the committed result after an injected post-commit
  acknowledgement failure;
- changed replay identity is rejected; and
- two competing private-verifier transitions produce exactly one winner.

The earlier binding, early-time, forged-commitment, out-of-window, and legacy
unbound-row tests continue to pass.

## 7. Validation

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 8. Remaining Known Limitations

- The verifier is private and SQLite-only.
- No trusted-host caller, scheduler, poller, executor recirculation, or automatic
  wake path exists.
- No other wake-source family is implemented.
- Local SQLite does not provide tamper resistance against a privileged local
  actor.
- Caller integration remains blocked until focused review accepts this fix.

## 9. Recommended Next Phase

Perform a focused maintainer/security blocker-fix review. Confirm the
transaction-owned time observation, stable commitment coverage, exact replay,
conflicting replay, concurrency behavior, and privacy posture before beginning
the first private trusted-host caller integration.

## 10. Governed Phase Record

- workflow: `dg/blocker`
- run: `run-1791033803075624000-2`
- approval: `approval/run-1791033803075624000-2/fix-approved`
- presentation: `presentation/5bb716bc815496ad`
- presentation hash:
  `5bb716bc815496ad7c740bee56b8299bce6b89b858bca39beabd14cd72582699`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- phase status: completed
- event summary: 39 events, including 1 approval request, 1 approval grant,
  8 policy decisions, 6 scheduled steps, 6 successful skill invocations,
  0 retries, and 0 escalations; approval-presentation proof enforced
- validation summary: formatting, workspace clippy with warnings denied, the
  complete Rust workspace test suite, documentation checks, and diff hygiene
  passed
- out-of-kernel work: implementation, tests, review, and validation were
  performed by the delegated trusted host; Workflow OS governed scope and
  approval but did not execute those actions
