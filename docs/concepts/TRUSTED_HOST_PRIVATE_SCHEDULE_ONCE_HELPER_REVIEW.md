# Trusted-Host Private Schedule-Once Helper Review

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups.**

The merged helper preserves the accepted trusted-host boundary. It derives one
current inert scheduling observation, exposes only an absolute UTC deadline to
an injected waiter, performs one fresh readiness assessment after wake, and
enters the accepted explicit reinvocation path at most once. It does not add a
scheduler loop, polling, automatic re-arming, authority-bearing timer state, or
public runtime behavior.

## 2. Scope Verification

The phase stayed within its approved private implementation scope:

- one crate-private deadline-wait trait;
- closed wake, cancellation, unavailable, and failure vocabulary;
- one crate-private schedule-once composition;
- focused elapsed, early, cancellation, failure, restart, and concurrency
  proofs; and
- roadmap, plan, and implementation-report updates.

It did not add repeated scheduling, a daemon, queue, worker pool, scheduler
store, automatic model continuation, automatic approval, another wake family,
public Rust API, CLI, SDK, schema, workflow configuration, provider execution,
OpenShell, nested harnesses, hosted behavior, or release changes.

## 3. Helper Boundary Assessment

`schedule_trusted_host_time_window_once` accepts explicit injected
dependencies and immutable operational identities. It derives the handoff,
condition identity, condition version, and scheduling ticket internally rather
than trusting caller-provided scheduling posture.

The injected waiter receives only the exact `Timestamp`. It receives no
backend, run identity, condition identity, handoff, ticket, authority,
approval, evidence, or executor capability. This is an appropriately narrow
host boundary and remains easy to replace in tests without making host timing
an authority source.

## 4. One-Shot Composition Assessment

The helper follows the accepted closed sequence:

1. observe one coherent current scheduling posture;
2. reject non-actionable posture before entering the host wait;
3. invoke the injected waiter exactly once;
4. return cancellation or bounded host failure without Core mutation;
5. assess current readiness exactly once after wake;
6. return refreshed inert posture for an early wake; or
7. invoke the accepted explicit reinvocation boundary exactly once.

There is no loop, poll, sleep implementation, retry, callback registration,
or automatic re-arm in this composition.

## 5. Authority And Freshness Assessment

A timer wake is not treated as satisfaction, approval, or authority. The
readiness assessment reloads current authoritative state and trusted time. The
reinvocation path then performs the accepted atomic transition and current
authority checks before executor entry.

The schedule ticket remains private and non-serializable. The helper does not
persist it or reconstruct authority after restart. Copying, delaying, or
duplicating a host callback cannot itself authorize work.

## 6. Cancellation, Failure, And Early-Wake Assessment

- `Canceled` returns a typed non-terminal host outcome.
- `Unavailable` and `Failed` map to stable bounded error codes.
- An early wake returns refreshed `NotYetEligible` scheduling posture.
- All three paths avoid executor and identity-provider calls.
- Focused tests compare continuity snapshots before and after these paths and
  prove byte-equivalent zero-write behavior.

No path fabricates workflow completion or failure, appends a durable security
rejection for ordinary timer jitter, or silently schedules another attempt.

## 7. Restart And Concurrency Assessment

The restart proof reopens SQLite and derives fresh scheduling state from the
authoritative store before one successful reinvocation. No timer registration,
ticket, or capability must be reconstructed from process memory.

The concurrent-callback proof synchronizes two callbacks after each has
observed the same ticket. The accepted lower boundaries admit exactly one
aggregate executor entry. The loser is bounded to a terminal zero-entry
outcome or a documented fail-closed stale/replay posture. Durable disposition
is terminal after the winner completes.

## 8. Privacy And Error Assessment

- Helper input Debug output exposes only a redacted binding marker.
- Early-wake output Debug redacts refreshed scheduling state.
- The reinvoked variant delegates only to existing bounded outcome Debug
  implementations.
- Host failures use static messages and stable codes without echoing timestamps,
  identifiers, paths, payloads, credentials, or provider values.
- No serde, CLI, schema, SDK, or public display surface was added.

The implementation preserves the repository's payload-free scheduling and
error posture.

## 9. Test Quality Assessment

The focused tests prove:

- elapsed wake produces one waiter call, one reinvocation, and one executor
  entry;
- early wake produces no write, executor entry, identity allocation, or
  automatic re-arm;
- cancellation and both host-failure categories produce no write or executor
  entry;
- restart performs fresh rehydration and one executor entry; and
- concurrent callbacks produce one aggregate executor entry and a terminal
  durable disposition.

The full workspace suite retains the deeper stale-ticket, trusted-time,
authority, idempotency, crash-recovery, and operational-entry proofs beneath
this composition.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Add a direct regression assertion for the new schedule-once input and
  outcome Debug wrappers when this private surface next changes.
- Add a composition-level stale-authority or replaced-wait test if the helper
  gains any broader host integration; current lower-boundary tests already
  enforce those failures.
- Keep any production timer implementation outside Core authority and require
  a separate reviewed integration boundary.

## 12. Recommended Next Phase

Plan the smallest private trusted-host repeated scheduling boundary needed to
resume lawful local work without an agent turn. The plan must preserve
schedule-once semantics per callback, derive every next wait from fresh Core
state, bound cancellation and shutdown, and prevent busy polling or automatic
approval.

Do not add public runtime configuration, CLI or schema exposure, provider
mutation, OpenShell, nested harness execution, hosted scheduling, or a general
workflow scheduler in that planning phase.

## 13. Governed Review Record

- workflow: `dg/review`
- run: `run-1791355016670122000-2`
- approval:
  `approval/run-1791355016670122000-2/review-scope-approved`
- presentation: `presentation/1e20d99a695b55b4`
- presentation hash:
  `1e20d99a695b55b4612bb06995429aed2b82647f3a5b634a2f955eed1f2565f1`
- approval outcome: granted by delegated maintainer through proof enforcement
- reviewed merge commit: `49b97009d0a2691912c752afb0978872a8082645`
- approved boundary: focused maintainer/security review only
- phase status: `Completed`
- event summary: 39 events; one approval; zero retries; zero escalations;
  approval-presentation proof enforced with one persisted presentation record
- validation summary: five focused schedule-once tests, formatting, strict
  workspace clippy, full workspace tests, docs checks, and diff checks passed
- out-of-kernel work: source inspection, security analysis, review authoring,
  validation, and later git and pull-request actions
