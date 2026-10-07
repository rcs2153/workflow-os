# Private Trusted-Host Production Timer Implementation Report

## 1. Executive Summary

Workflow OS now has one crate-private synchronous local timer primitive for
the accepted trusted-host `TimeWindow` scheduling boundary. The timer blocks
only the caller's thread until an absolute deadline or explicit cancellation,
then returns control to the existing bounded repeated driver and Core-owned
readiness assessment.

This is a production-shaped private timer primitive, not automatic scheduling.
It does not discover runs, persist jobs, create background threads, mint
identity, grant authority, approve work, or invoke the executor outside the
accepted Core path.

## 2. Scope Completed

- Added a private `Condvar`-backed absolute-deadline waiter.
- Added private shared cancellation state and an explicit cancellation handle.
- Made cancellation prompt, idempotent, and non-authorizing.
- Added a thin synchronous wrapper around the accepted bounded repeated driver.
- Kept wake and redispatch identity providers explicit and caller-supplied.
- Added fixed non-leaking failure behavior for poisoned synchronization state.
- Added focused timer, cancellation, restart, duplicate-identity, concurrency,
  privacy, and integrated Core tests.

## 3. Scope Explicitly Not Completed

This phase did not add:

- an explicit production caller;
- a daemon, queue, worker pool, detached thread, or run discovery;
- durable timer jobs, startup scanning, or automatic process recovery;
- automatic model turns or automatic approval;
- public API, CLI, SDK, schema, or runtime configuration;
- provider reads or mutations;
- OpenShell or nested harness execution;
- hosted or PostgreSQL scheduling;
- report artifacts, filesystem output, or release changes.

## 4. Implementation Summary

`TrustedHostLocalDeadlineWaiter` implements the existing private
`TrustedHostDeadlineWaiter` contract. It uses a `Mutex` and `Condvar` to wait
without polling Core. After spurious notification it checks only cancellation
and host time, then continues waiting.

`run_trusted_host_local_timer` constructs that waiter and delegates once to
`run_bounded_trusted_host_repeated_scheduling`. It adds no second scheduling
loop, state write, event append, workflow-status interpretation, or direct
executor call.

## 5. Cancellation And Ownership

The caller creates a private cancellation receiver and handle. Signaling the
handle sets one shared boolean and notifies current waiters. Repeated signals
are harmless. Cancellation returns the existing typed `Canceled` stop and does
not mutate, fail, or complete the workflow.

Dropping the handle does not cancel the synchronous wait. The caller's thread
remains the execution owner in this slice. Any future detached owner requires
separate owner-loss semantics and review.

## 6. Clock And Authority Boundary

The timer uses wall time only to calculate a host blocking duration. Timeout
does not prove eligibility. After every wake, the existing schedule-once path
reloads durable state, reads Core-owned trusted time, validates the current
ticket and handoff, and decides whether execution may be admitted.

Impossible duration conversion and synchronization poison fail closed. The
timer does not create authority or identity.

## 7. Restart And Concurrency

The timer stores no durable job or remaining budget. Restart reconstruction
uses a reopened backend, durable locator, fresh finite budget, and fresh
identity providers.

Competing timer integrations remain safe through the existing Core one-winner
transition. Focused tests prove aggregate executor entry remains at most one.
Reusing a wake identity across different waits fails closed with the existing
operation replay conflict.

## 8. Privacy And Redaction

The new private types do not derive serialization. Their `Debug` output omits
deadlines, identifiers, paths, commands, payloads, credentials, source
contents, and synchronization internals. Timer and cancellation failures use
stable codes and fixed messages.

## 9. Test Coverage

Thirteen focused tests cover:

- elapsed and future deadlines;
- real timeout behavior;
- cancellation before and during a wait;
- repeated cancellation;
- spurious notification;
- poisoned synchronization state;
- bounded redacted Debug output;
- one integrated deadline and executor entry;
- two sequential integrated deadlines;
- cancellation without continuity mutation;
- reconstruction after backend reopen;
- duplicate wake identity rejection; and
- competing integrations with aggregate at-most-once executor entry.

## 10. Validation

All required validation passed:

- `cargo fmt --all --check`
- 13 focused `workflow-core` local-timer tests
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `git diff --check`

## 11. Remaining Limitations

- No caller selects or starts this private timer automatically.
- Timer and cancellation state are process-local and intentionally ephemeral.
- Handle loss is a no-op while the synchronous caller retains ownership.
- Host wall-clock movement may lengthen or shorten blocking, but cannot grant
  authority because Core reassesses trusted time.
- Production identity generation remains caller-owned and unstandardized.
- No operator-visible timer status or durable scheduling metrics exist.

## 12. Recommended Next Phase

Perform a focused maintainer/security review of this implementation. Review
the host/Core boundary, cancellation, synchronization failure, clock movement,
restart, identity reuse, competing callers, privacy, tests, and scope.

Do not add a production caller, general scheduler, public configuration,
provider mutation, OpenShell, nested harnesses, automatic approval, CLI, SDK,
schema, hosted scheduling, or release changes during that review.

## 13. Governed Phase Record

- workflow: `dg/implement`
- run: `run-1791359797992546000-2`
- approval:
  `approval/run-1791359797992546000-2/implementation-approved`
- presentation: `presentation/5eee2d9fa025afe1`
- presentation hash:
  `5eee2d9fa025afe13042c3847fe8522611c00109fedfeca17d142b941d4b6022`
- approval outcome: granted by delegated maintainer through proof enforcement
- approved boundary: private synchronous local timer implementation only
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval-presentation enforcement: proof enforced with one persisted record
- validation summary: formatting, 13 focused tests, workspace clippy,
  workspace tests, docs checks, and diff checks passed
- out-of-kernel work: source and test edits, shell validation, documentation,
  and later git and pull-request actions
