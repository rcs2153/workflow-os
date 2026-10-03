# Trusted Host TimeWindow Wait Registration Review

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups.**

The crate-private trusted-host executor can request zero or one bounded
`TimeWindow` wait without gaining authority over trusted time, wake
satisfaction, continuation, or redispatch. Core derives the dependency
binding from canonical SQLite state, and the existing SQLite continuity
transaction revalidates every mutable security fact before atomically
persisting the yield, wait, directive, attempt transition, window transition,
receipt, and event projection.

No blocker was found. The implementation remains private, SQLite-only, and
non-scheduling. Direct registration-path coverage for an already elapsed
deadline and for trusted-time drift between derivation and transaction would
strengthen the suite but does not replace or weaken the transaction-level
checks already covered by the accepted continuity backend tests.

## 2. Scope Verification

The phase stayed within its approved boundary:

- one crate-private `TimeWindow` declaration is supported;
- the existing zero-wait yield path remains available;
- no scheduler, timer service, poller, queue, or wake bus was added;
- no automatic reinvocation or conversational recirculation was added;
- no approval, evidence, check, external-event, capability, authority-refresh,
  or conflict wait source was added;
- no provider, OpenShell, sandbox, or nested-harness execution was added;
- no public API, workflow schema, CLI, SDK, UI, example, hosted, distributed,
  or release surface changed; and
- no persistence schema changed in this phase.

## 3. Declaration Boundary Assessment

`TrustedHostYieldRequest` and its `TimeWindow` declaration remain
crate-private. The executor supplies only a validated condition identity,
positive condition version, deadline, and existing yield reason. It cannot
supply workflow, run, step, actor, window, attempt, cursor, generation,
governance, authority, trusted-time source, provenance, epoch, wake trigger,
or dependency commitment.

The declaration is therefore a bounded liveness request. It is not an
approval, policy decision, wake capability, or continuation capability.

## 4. Core-Derived Binding Assessment

`derive_time_window_wait_binding` reads the canonical continuity snapshot and
requires live eligible trusted-time state, a non-quarantined clock, an
executing exact window, and a deadline after the current watermark but no
later than window expiry. Core then derives the domain-separated commitment
from deadline, source kind, provenance commitment, and epoch identity.

The executor cannot substitute those fields. The later transactional check
prevents the preflight snapshot from becoming an authority source.

## 5. Atomicity And Race Assessment

The existing SQLite `register_yield` transaction remains authoritative. Its
single trusted-time observation revalidates:

- exact window state, revision, binding, and cursor;
- exact attempt state and one-use capability bindings;
- condition version and identity uniqueness;
- `DeadlineReached` trigger compatibility;
- dependency-commitment recomputation;
- current trusted-time source, provenance, and epoch; and
- deadline freshness and authorized-window expiry.

Only after those checks does one transaction persist the yield, wait,
directive, attempt and window transitions, receipt, and projection. If
trusted-time or window state changes after binding derivation, registration
fails closed and no partial yield or wait is committed.

## 6. Replay And Continuation Assessment

The registration request commitment covers the wait identity, version,
trigger, and exact dependency binding. Existing operation replay,
reconciliation, receipt, cursor, generation, directive, and dispatch
reservation semantics remain unchanged.

Successful registration returns `AwaitCondition` only after reading committed
authoritative continuation state. It does not satisfy the wait, consume the
directive, reconstruct a capability, or enter another executor attempt.

## 7. Trusted-Time Assessment

The first wake source correctly reuses the accepted Core-injected trusted-time
boundary. Registration requires the deadline to remain future relative to the
transaction observation and within window expiry. Later satisfaction remains
owned by the separately accepted `TimeWindow` verifier, which requires a
fresh matching source, provenance, and epoch.

No caller-authored timestamp or boolean can satisfy the wait.

## 8. Privacy And Error Assessment

The private request Debug implementation exposes only the yield reason, wait
count, and a redacted binding marker. Stable errors omit condition values,
deadlines, timestamps, paths, provenance, epochs, prompts, commands, provider
payloads, credentials, and tokens.

Persistence uses the already reviewed bounded identifiers, timestamps, and
commitments. It does not store source contents, raw executor input, or wake
payloads.

## 9. Test Quality Assessment

Focused registration tests prove:

- existing zero-wait yields remain valid;
- one `TimeWindow` wait is registered atomically;
- committed continuation becomes `AwaitCondition`;
- the wait carries the exact generation, deadline, source, provenance, and
  epoch;
- condition version zero fails without a partial yield or wait;
- a deadline beyond window expiry fails without a partial yield or wait; and
- Debug and errors do not expose declaration values.

The broader continuity suite separately covers dependency-commitment
validation, deadline enforcement inside the transaction, exact replay,
competing transitions, wrong trusted-time posture, and restart behavior.

## 10. Documentation Assessment

The roadmap, implementation plan, and report accurately describe the slice as
private, local, SQLite-only, and non-scheduling. They do not claim automatic
wake observation, reinvocation, provider or sandbox execution, public
configuration, hosted behavior, or production readiness.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Add a direct supervisor-registration test for a deadline that is already
  elapsed but still inside window expiry.
- Add a focused derivation-to-transaction drift test when the backend exposes
  a deterministic fault-injection seam for trusted-time epoch or provenance
  change.
- Keep every non-time wake source separately planned and reviewed.
- Preserve SQLite-only scope until a second backend proves equivalent atomic
  dependency validation.

## 13. Recommended Next Phase

Plan the smallest opaque, non-authoritative wait handoff for the already
registered exact wait. The handoff may orient a trusted host to current wait
posture and the closed next operation, but it must not satisfy the wait,
reconstruct authority, schedule work, poll, sleep, invoke an executor, or
expose a public runtime API.

Do not begin scheduling, additional wake-source families, provider or sandbox
integration, nested harness execution, public configuration, CLI, SDK,
hosted behavior, or release changes first.

## 14. Validation

The reviewed merged tree passed:

- `cargo test -p workflow-core --lib operational_opening_store --no-fail-fast`
  with 36 tests;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`;
- `git diff --check`; and
- all seven required GitHub CI jobs on pull request 507.

## 15. Governed Review Record

- workflow: `dg/review`
- run: `run-1791061765713711000-2`
- approval: `approval/run-1791061765713711000-2/review-scope-approved`
- presentation: `presentation/08082800763a78bc`
- presentation hash:
  `08082800763a78bcb886afee8eae881135c7288f92942d3583961c8068aa0f6f`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: focused maintainer/security review only
- phase status: completed
- event summary: 39 events; one approval; zero retries; zero escalations;
  approval-presentation proof enforced
- validation summary: exact merged implementation validation and all seven CI
  jobs passed; documentation and diff checks passed after review authoring
- out-of-kernel work: source inspection, security review, documentation,
  validation, and git actions were performed by the delegated trusted host;
  Workflow OS governed scope and approval but did not perform those actions
