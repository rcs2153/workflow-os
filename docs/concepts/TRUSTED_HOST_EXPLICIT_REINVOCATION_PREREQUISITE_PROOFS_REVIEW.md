# Trusted-Host Explicit Reinvocation Prerequisite Proofs Review

## 1. Executive Verdict

**Needs one focused blocker fix before private scheduling-boundary
implementation.**

The transition-to-entry crash proof is accepted. It places a test-only fault
seam after the durable wait transition and before operational entry, reopens
SQLite, recovers through exact transition replay, obtains fresh one-use
authority from current state, and admits the executor exactly once.

The concurrent-caller proof demonstrates one executor entry in the observed
race, but its loser assertion is narrower than the production contract. A
second caller can lawfully reach existing-window entry after the winning
caller has already made the window terminal. In that interleaving the second
caller receives a successful terminal outcome with zero executor entries,
not one of the two asserted errors. The proof and report must cover that
bounded terminal loser before scheduling work begins.

## 2. Scope Verification

The implementation stayed within the approved prerequisite-proof scope:

- it added one full-composition concurrent-caller test;
- it added one `cfg(test)` transition-to-entry fault seam and restart test;
- the production reinvocation function retains its existing signature;
- no scheduler, readiness assessment, scheduling ticket, deadline waiter,
  timer, polling loop, queue, worker, or host execution loop was added;
- no provider execution, OpenShell integration, nested harness behavior, or
  provider mutation was added; and
- no public API, CLI, SDK, schema, hosted behavior, or release posture changed.

## 3. Concurrent Admission Assessment

The test races two callers through `reinvoke_after_time_window_wait` with the
same handoff, transition operation, and receipt. It correctly asserts the
load-bearing invariant that the shared executor is entered once and that
authoritative continuation posture ends `Terminal`.

The production composition retains multiple one-winner boundaries:

- the exact wait transition records one transition and exact-replays the
  duplicate operation;
- resume-directive consumption grants one caller the fresh one-use
  capability; and
- dispatch reservation remains the final atomic admission boundary.

No process-local mutex or caller-owned winner decision was introduced.

## 4. Concurrent Loser Blocker

The test currently requires exactly one successful caller and exactly one
error whose code is either:

- `trusted_host_redispatch.directive_replayed`; or
- `trusted_host_redispatch.attempt_limit_inconsistent`.

Those are valid fail-closed loser observations, but they are not exhaustive.
`enter_existing_trusted_host_operation` first reloads authoritative
continuation posture. If the winning caller reaches terminal state before the
second caller performs that read, `closed_outcome(Terminal)` returns a lawful
successful result with:

- `stop_reason: Terminal`;
- `executor_entries: 0`; and
- no identity-provider or authority consumption attempt.

The current proof would fail under that safe scheduler interleaving even
though the one-entry invariant holds. Repeating the test and observing the two
error outcomes does not prove that the terminal zero-entry path is excluded.

Before scheduling implementation, the focused proof must accept and assert
all bounded loser classes while still requiring:

- exactly one aggregate executor entry;
- exactly one underlying executor call;
- every successful zero-entry loser to be terminal;
- every error loser to use an explicitly accepted bounded code; and
- terminal durable continuation posture.

The implementation report must describe terminal zero-entry as a lawful loser
outcome rather than claiming that every loser is an error.

## 5. Crash-Seam Placement Assessment

The test seam is correctly placed after
`apply_trusted_host_time_window_wake` returns `Transitioned` or `ExactReplay`
and before `enter_existing_trusted_host_operation` is called. The injected
failure therefore models the intended committed-transition/pre-entry crash,
not a pre-commit failure or a failure after executor admission.

The seam is private and `cfg(test)` only. Its callback receives no input and
cannot inspect or retain workflow data, handoff data, authority, identities,
paths, commands, provider payloads, or secrets. Production execution uses a
no-op callback through the unchanged private entrypoint.

## 6. Restart And Replay Assessment

The crash proof verifies zero executor calls and zero identity-provider calls
before reopening the backend. Recovery reuses the exact transition operation
and receipt, observes `ExactReplay`, reloads current SQLite state, and enters
the existing-window path exactly once.

A later replay after terminal completion returns zero executor entries and
does not request a new identity. This demonstrates that transition replay is
restart-safe and that terminal replay cannot duplicate work.

## 7. Authority Assessment

No authority is serialized or reconstructed through the crash seam. Recovery
does not receive a cached directive or capability. Existing-window entry:

1. reloads the exact durable window;
2. validates workflow, run, step, window, actor, immutable bundle, invocation,
   and executor binding;
3. observes current continuation posture;
4. consumes a fresh available resume directive; and
5. reaches the executor only through the accepted supervisor and dispatch
   reservation path.

The inert handoff commitment remains stale-detection and replay-binding input,
not execution authority.

## 8. Failure And Privacy Assessment

- Security-rejected wake posture remains a bounded reinvocation error and does
  not disclose current continuation disposition.
- Concurrent error outcomes use stable static codes and messages.
- The test-only crash error contains no runtime values.
- Debug and serialization surfaces are unchanged and remain redacted.
- No raw workflow input, provider payload, source content, command output,
  credential, approval reason, or authority material is added to storage or
  output.

## 9. Test Quality Assessment

Accepted coverage:

- complete explicit-reinvocation composition is exercised rather than only
  isolated transition or reservation primitives;
- the observed concurrent race admits one executor entry;
- the transition-to-entry crash occurs at the required seam;
- backend reopen and exact replay recover one executor entry;
- post-terminal replay adds no entry; and
- existing reinvocation coverage remains green.

Blocking gap:

- the concurrency assertion does not cover the lawful terminal zero-entry
  loser produced by the existing operational-entry contract.

Non-blocking future coverage remains unchanged: direct composition-level
early-wake, trusted-time failure, current-authority denial, and remaining-wait
cases belong with the later representable scheduling/readiness surface.

## 10. Documentation Assessment

The report accurately describes the private scope, fault seam, restart path,
and deferred scheduler surface. Its concurrent-loser wording is too narrow:
it says the loser receives one of two errors, while production can also return
a terminal zero-entry outcome. That claim must be corrected with the focused
test fix.

## 11. Blockers

1. Harden the full-composition concurrency proof to accept and verify the
   lawful terminal zero-entry loser interleaving in addition to the two
   bounded fail-closed errors.
2. Correct the implementation report so it does not claim that every losing
   caller must receive an error.

No production runtime change is required by this review finding.

## 12. Non-Blocking Follow-Ups

- Keep the transition-to-entry seam test-only and payload-free.
- Preserve the exact-replay recovery path without cached authority.
- Keep scheduling types and behavior blocked until the focused proof fix is
  reviewed.
- Add readiness and early-wake coverage only with the later private scheduling
  slice.

## 13. Recommended Next Phase

Implement a focused prerequisite-proof blocker fix. Change only the concurrent
test assertions and the phase report so all lawful loser outcomes are bounded
and explainable while the aggregate executor-entry invariant remains exactly
one. Then perform a focused blocker-fix review.

Do not implement the readiness assessment, schedule ticket, deadline waiter,
or private scheduling helper yet.

## 14. Governed Review Record

- workflow: `dg/review`
- run: `run-1791350427708875000-2`
- approval:
  `approval/run-1791350427708875000-2/review-scope-approved`
- presentation: `presentation/9f2c484d4bea458d`
- presentation hash:
  `9f2c484d4bea458d01c9d523ee2fefb62e3154a315f0488be537e0f0fc40cef5`
- approval outcome: granted by delegated maintainer through proof enforcement
- reviewed implementation commit: `292d918`
- approved boundary: focused maintainer/security review only
- phase status: `Completed`
- event summary: 39 events, including one approval request, one approval
  grant, eight policy decisions, six scheduled steps, six successful skill
  invocations, zero retries, and zero escalations; approval-presentation proof
  enforced with one matching record and event marker
- validation summary: focused reinvocation tests, formatting, strict workspace
  clippy, full workspace tests, documentation checks, and diff checks passed
- out-of-kernel work: source inspection, security analysis, review authoring,
  validation, and later git and pull-request actions
