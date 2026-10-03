# Trusted-Host Explicit Reinvocation Vertical Slice Plan Review

## 1. Executive Verdict

**Plan accepted with in-review corrections; proceed to one private local
SQLite implementation slice.**

The plan identifies the correct next P0 composition: validate an inert opaque
handoff, apply the exact source-specific `TimeWindow` transition, and enter
through the accepted operational entry helper. It does not create scheduling
authority or make the handoff executable.

Focused review found one durable replay gap and two API/error ambiguities. The
plan now resolves all three before implementation: persist a payload-free
handoff commitment in the transition replay envelope, remove irrelevant
opening inputs from the existing-window reinvocation path, and return no
current disposition with a security rejection.

## 2. Scope Verification

The plan remains within one crate-private local SQLite vertical slice. It does
not authorize a scheduler, polling, timers, background execution, model-turn
creation, automatic approval, provider or sandbox execution, OpenShell,
nested harnesses, public configuration, CLI, SDK, workflow schema, hosted or
distributed execution, writes, or release changes.

## 3. Boundary Assessment

The proposed composition uses the correct existing boundaries:

- coherent read-only handoff observation for stale detection;
- the accepted source-specific trusted-time verifier for satisfaction;
- durable operation replay for commit ambiguity;
- `enter_trusted_host_operation` for fresh directive consumption, dispatch
  reservation, authority reassessment, and executor entry.

No new caller is allowed to construct revision, cursor, generation, wait,
directive, dispatch, or authority values.

## 4. Handoff Authority Assessment

The handoff remains inert. Equality with a fresh projection proves only that
the caller is referring to the same bounded posture. It does not prove the
deadline elapsed and cannot create a wake or execution capability.

The implementation must keep the handoff private and non-serializable. No
mutating API may accept only a handoff ID.

## 5. Durable Replay Finding

The original plan left open whether the existing transition operation could
bind exact replay to the handoff. Inspection shows it cannot do so completely.
The transition commitment covers exact window, revision, cursor, generation,
condition, and wait-revision facts. The persisted request envelope carries a
bounded subset. Neither durably records the opaque handoff identity.

This matters after an ambiguously committed success: the current wait is then
satisfied, so the old `AwaitCondition` handoff cannot be freshly re-derived
before replay. Accepting the operation based only on caller-held handoff memory
would leave the composition underbound.

The corrected plan requires a Core-derived, payload-free handoff commitment in
both the transition request commitment and the durable private replay
envelope. Exact replay must compare it before continuing. Existing direct wake
operations without that binding remain a distinct path and cannot be treated
as handoff-based reinvocation.

## 6. Operational Entry Assessment

`TrustedHostOperationalEntryInput` currently carries opening context and
opening-persistence values even when a window already exists. A reinvocation
caller should not manufacture irrelevant placeholders or retain initial-open
authority solely to satisfy that shape.

The corrected plan requires an existing-window-only wrapper or split input
that still delegates to the accepted operational entry logic. This is API
narrowing inside the crate, not a new entry implementation.

## 7. Classification And Transition Assessment

The plan correctly avoids requiring a generic `AwaitCondition`
classification at the elapsed deadline. Generic semantics may conservatively
classify an elapsed unsatisfied wait as blocked. Only the source-specific
trusted-time verifier can decide whether the exact transition is lawful.

Before transition, the helper must validate the supplied handoff against one
coherent authoritative snapshot. After transition or exact replay, it must
reload and enter from fresh authoritative state.

## 8. Concurrency And Idempotency Assessment

The accepted operations already provide the right correctness boundaries:
durable operation replay, one-winner directive consumption, and atomic
dispatch reservation. The implementation must compose those operations and
must not add an in-memory lock as authority.

Focused tests must prove that two callers with the same inert handoff produce
at most one executor entry and that all losing outcomes are recoverable from
durable state after restart.

## 9. Failure And Error Assessment

The corrected posture is appropriate:

- stale or substituted handoff input fails before mutation;
- source unavailability cannot fabricate satisfaction;
- blocked and terminal state never enters the executor;
- an ordinary coherent state change may return a bounded current outcome;
- a security rejection returns only a stable non-leaking error; and
- no error becomes completion, approval, or a fake external wait.

Returning a disposition alongside a security rejection would create an
unnecessary state oracle and is not authorized.

## 10. Restart And Ambiguity Assessment

The plan preserves fresh-process restart and exact transition replay. It also
correctly defers uncertain executor-attempt handling to existing durable
attempt semantics. The new helper may not silently retry an attempt merely
because the host did not receive a response.

The implementation must test crashes after wait transition, after directive
consumption, and around executor entry without inventing authority or outcome.

## 11. Privacy Assessment

The plan preserves custom bounded Debug, stable error codes, and no public
serde. The required replay addition is a domain-separated payload-free
commitment. It must not persist deadlines, prompts, commands, provider data,
evidence bodies, credentials, or authority internals beyond the already
accepted continuity records.

## 12. Test Plan Assessment

The planned matrix covers normal transition, unelapsed waits, stale handoffs,
binding substitution, trusted-time failure, restart, exact replay, crash
boundaries, concurrency, remaining waits, current-authority reassessment,
privacy, and regression suites.

Implementation should add a direct test proving that a committed transition
with a different handoff commitment cannot replay, even when operation,
receipt, window, and condition identities otherwise match.

## 13. Blockers

None after the in-review corrections.

## 14. Non-Blocking Follow-Ups

- Keep the first implementation limited to one already-registered exact
  `TimeWindow` wait.
- Retain direct wake callers as a distinguishable lower-level boundary.
- Revisit a host scheduling API only after implementation review accepts this
  one-shot composition.

## 15. Recommended Next Phase

Implement one crate-private local SQLite transition-and-entry helper with the
required durable handoff commitment, existing-window-only entry shape, restart
and concurrency tests, and no public or scheduling surface.

## 16. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.
- Architecture inspection covered the accepted handoff, `TimeWindow` caller,
  transition commitment/envelope, operational entry, redispatch, and durable
  continuity boundaries.

## 17. Governed Review Record

- workflow: `dg/review`
- run: `run-1791067044459395000-2`
- approval:
  `approval/run-1791067044459395000-2/review-scope-approved`
- presentation: `presentation/aacb66c6d1ffd2a7`
- approval outcome: granted by delegated maintainer through proof enforcement
- approved boundary: focused documentation-only maintainer/security review
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- approval-presentation enforcement: proof enforced with one persisted
  presentation record and event marker
- validation summary: documentation and diff checks passed
- out-of-kernel work: source and plan inspection, review authoring,
  documentation validation, and later git and pull-request work
- missing coverage: the kernel coordinates governance only; it did not inspect
  source, author this review, execute checks, implement runtime behavior,
  create a WorkReport artifact, or perform git actions
