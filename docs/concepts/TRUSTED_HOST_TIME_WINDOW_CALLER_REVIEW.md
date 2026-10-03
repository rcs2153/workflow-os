# Trusted Host TimeWindow Caller Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups; proceed to the private typed-wait
registration slice.

The first crate-private caller preserves the accepted trust boundary. It
derives mutable transition expectations from canonical SQLite state, delegates
trusted-time satisfaction to the transaction-owned verifier, returns bounded
orientation rather than authority, and admits exact restart replay without
opening a scheduler or executor path. No blocker was found in the approved
scope.

## 2. Scope Verification

The phase stayed within one explicit, synchronous `TimeWindow` call. It added
no scheduler, loop, sleep, poller, queue, executor invocation, typed-wait
registration, opaque host handoff, provider or sandbox integration, public API,
CLI, SDK, schema, hosted behavior, or additional wake-source family.

The caller remains crate-private and is not reachable through current runtime
configuration. The implementation report describes those boundaries honestly.

## 3. Caller Boundary Assessment

`TrustedHostTimeWindowWakeInput` accepts an existing private operational
locator, condition identity and version, and fresh operation and receipt
identities. It cannot supply window revision, cursor, yield generation, wait
revision, governance commitment, authority commitment, deadline, trusted-time
observation, or wake capability.

The caller loads a validated canonical snapshot, binds the locator to the
current window, obtains mutable expectations from that state, and calls the
accepted verifier. This is the appropriate minimum private boundary: the host
may identify the intended operation but cannot author the facts that make the
transition valid.

## 4. Authority And Trusted-Time Assessment

The caller does not construct `WakeAssessmentCapability`. The SQLite verifier
continues to obtain exactly one trusted-time observation after entering the
immediate transaction, validates the exact durable dependency, and creates any
private one-use authority in that same call.

The result contains only transition status and current
`AuthoritativeContinuationDisposition`. It contains no directive, capability,
attempt authority, or executor callback. A satisfied wait may still return
`Blocked`; timer satisfaction is therefore not confused with authorization to
execute.

## 5. Binding And Current-State Assessment

The locator check covers workflow, run, step, window, subject actor, and
immutable run bundle. The transition request additionally derives current
window revision, governance and authority commitments, cursor, active yield
generation, condition version, and wait revision from canonical state.

The verifier revalidates all of those values under transaction serialization
and recomputes the exact `TimeWindow` dependency commitment. A stale snapshot
cannot authorize a transition because the transactional compare-and-set checks
remain authoritative.

## 6. Replay And Restart Assessment

Before requiring current wait eligibility, the caller recognizes an existing
operation and checks its durable operation kind, receipt, window, condition
identity, and condition version. Canonical snapshot loading has already
cross-checked the serialized operation record, request envelope, normalized
columns, operation commitment, receipt commitment, trusted-time commitment,
and success or rejection commitment.

This supports recovery after restart even though a committed transition has
changed the current wait state. Changed receipt or stable request identity is
rejected. The operation and receipt remain caller-supplied idempotency
identities; they do not become reusable execution authority.

## 7. Failure, Corruption, And Concurrency Assessment

- Missing canonical state, malformed operation projection, or unexpected
  result shape fails with a stable corruption error.
- Wrong locator binding and conflicting operation reuse fail closed.
- Ineligible, non-deadline, unbound, or already-transitioned waits fail before
  verifier success.
- Trusted-time security rejection is reduced to `SecurityRejected`; source
  observations and rejection payloads are not returned.
- Distinct competing operations retain one-winner behavior through SQLite
  immediate transactions and exact revisions.
- Ambiguous post-commit recovery remains available through durable operation
  replay.

No state surgery, retry loop, or automatic executor action was introduced.

## 8. Privacy And Redaction Assessment

Caller Debug output exposes only a redacted binding marker. Stable errors do
not echo identifiers, deadlines, timestamps, paths, provenance, epochs,
commands, source contents, credentials, tokens, or provider payloads. The
caller creates no new persistence family and reuses canonical payload-free
continuity records.

## 9. Test Quality Assessment

Focused caller tests cover successful state-derived transition, restart replay,
conflicting replay, stale locator rejection, concurrent one-winner behavior,
and redaction-safe Debug output. Accepted verifier tests cover one transaction
observation, ambiguous-commit replay, conflicting replay, competing
transitions, exact dependency binding, and legacy unbound-wait rejection.

The tests are sufficient for acceptance. Two useful caller-level cases remain
non-blocking follow-ups:

- map a fresh trusted-time security rejection to bounded
  `SecurityRejected`; and
- recover the same security-rejected operation after reopening the backend.

These cases are already covered at the verifier and canonical-codec layers,
but direct caller tests would make its complete result vocabulary explicit.

## 10. Documentation Assessment

The roadmap, implementation plan, and implementation report accurately state
that the caller is private, synchronous, one-shot, and non-authoritative. They
do not claim typed-wait registration, opaque handoff, scheduling, autonomous
continuation, executor invocation, provider work, or public availability.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Add direct caller tests for fresh and replayed security rejection when that
  test fixture can vary trusted time without broadening production behavior.
- Keep operation and receipt generation outside the caller but require fresh,
  bounded identities at every future host integration point.
- Preserve canonical snapshot validation before any replay shortcut.
- Keep additional wake-source families behind separate source-specific plans
  and verifiers.
- Local SQLite remains non-tamper-resistant against a privileged local actor.

## 13. Recommended Next Phase

Implement the private typed-wait registration slice for `TimeWindow` only.
Allow the exact authorized executor result to return one bounded wait
declaration; let Core validate and derive all run, window, attempt, generation,
cursor, authority, and governance bindings; and register the yield and wait
atomically.

Do not add opaque wait handoff until a genuine typed wait can be registered by
the accepted executor path. Do not begin scheduling, polling, provider,
sandbox, nested-harness, public configuration, or additional wake-source work.

## 14. Validation

- `cargo test -p workflow-core --lib trusted_host_time_window_caller --no-fail-fast`:
  passed, 5 tests.
- `cargo fmt --all --check`: passed.
- `cargo clippy -p workflow-core --lib --tests -- -D warnings`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.
- Implementation-phase `cargo clippy --workspace --all-targets -- -D warnings`:
  passed.
- Implementation-phase `cargo test --workspace`: passed.
- Pull request 505: all 7 required GitHub checks passed; squash merge
  completed.

## 15. Governed Review Record

- workflow: `dg/review`
- run: `run-1791050996701692000-2`
- approval: `approval/run-1791050996701692000-2/review-scope-approved`
- presentation: `presentation/848bb1ffdcfa9d7d`
- presentation hash:
  `848bb1ffdcfa9d7dc6ece569a763770b6990b32b92184157b013faaae01144dc`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- review status: accepted with non-blocking follow-ups
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations;
  approval-presentation proof enforced
- validation summary: 5 focused caller tests, formatting, workflow-core clippy,
  documentation checks, and diff hygiene passed; the implementation-phase
  workspace clippy, workspace tests, and 7 required GitHub checks were reviewed
  as supporting evidence
- out-of-kernel work: code reading, security analysis, focused validation, and
  review documentation were performed by the delegated trusted host; Workflow
  OS governed scope and approval but did not perform those actions
