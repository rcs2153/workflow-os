# Trusted-Host Explicit Local Process Owner Report

## 1. Executive Summary

Workflow OS now has the smallest accepted process-owned boundary for one
explicitly selected trusted-host local SQLite operation. The new crate-private,
one-shot owner creates the existing process-local cancellation pair, gives the
handle to process control, atomically decides whether cancellation or initial
entry wins, and delegates admitted work once to the accepted explicit local
operation composition.

The owner is not automatic continuation. It does not discover runs, start a
background worker, install signal handling, expose public configuration, or
create durable ownership. Core remains the only authority for admission,
continuation, reservation, replay, execution, and terminal state.

## 2. Scope Completed

- Added `TrustedHostExplicitLocalProcessOwner` as a crate-private one-shot
  synchronous owner.
- Added a constructor that returns the owner and the existing cancellation
  handle without reading or writing Core state.
- Added a consuming `run` method that delegates admitted work at most once.
- Added a bounded owner outcome distinguishing cancellation before entry from
  an admitted operation stop.
- Added one mutex-backed cancellation-versus-entry linearization decision to
  the existing local timer cancellation state.
- Added focused tests for pre-entry cancellation, active-attempt cancellation,
  competing owners, idempotent post-return cancellation, and redacted Debug.

## 3. Scope Explicitly Not Completed

This phase did not add application adoption, run or workflow discovery,
startup scanning, state polling, automatic invocation, automatic restart,
automatic approval, a daemon, background thread, worker pool, async runtime,
signal handling, durable owner state, host-job persistence, leases, leader
election, public API, CLI, SDK, schema, runtime configuration, provider reads
or writes, OpenShell integration, nested harnesses, hosted scheduling,
PostgreSQL scheduling parity, new wait families, reports, artifacts, or release
posture changes.

## 4. Private Owner API And Lifecycle

`TrustedHostExplicitLocalProcessOwner::new` accepts one exact
`TrustedHostOperationalEntryInput` and returns the owner with a
`TrustedHostLocalTimerCancellationHandle`. Construction is in-memory only.

`run(self)` consumes the owner. If cancellation won before entry, it returns
`CanceledBeforeEntry`. If entry won, it transfers the exact operational input
and cancellation receiver to `run_explicit_trusted_host_local_operation` and
returns its bounded outcome as `OperationStopped`.

The consumed owner cannot be restarted or cloned. The implementation adds no
serialization or persistence surface and does not translate a host stop into
workflow success, failure, cancellation, retry, or completion.

## 5. Cancellation Linearization

The existing private cancellation state now contains `entry_started`.
`begin_entry` and `cancel` use the same mutex:

- cancellation winning first produces `CanceledBeforeEntry` before any Core
  read, write, identity generation, event append, or executor entry;
- entry winning first admits the existing operational boundary and later
  cancellation cannot revoke consumed authority or interrupt the active
  attempt; and
- later cancellation remains recorded and is observed if admitted work next
  reaches the local timer wait.

Calling `begin_entry` twice fails with the stable code
`trusted_host_local_timer.entry_already_started`. The consuming owner makes
that state defensive rather than an ordinary caller path.

## 6. Outcome And Stop Semantics

The owner returns only `CanceledBeforeEntry` or
`OperationStopped(TrustedHostExplicitLocalOperationOutcome)`. It preserves the
accepted lower-level entry and continuation stop vocabulary without inventing
workflow lifecycle facts.

Cancellation is cooperative process-local control. It is not a general
shutdown token, cannot interrupt an executor attempt, creates no authority,
does not satisfy a wait, and appends no workflow event. Application-level
shutdown and operator notification remain deferred.

## 7. Authority, Privacy, And Debug

The owner accepts no caller-authored capability, approval, eligibility,
wait-satisfaction claim, or continuation disposition. It receives only the
already-selected exact operation input and delegates to existing reviewed Core
boundaries.

Custom Debug output exposes only a redacted binding marker. The owner does not
serialize workflow, run, step, window, actor, immutable-bundle, skill input,
path, deadline, credential, token, or provider values. Existing structured
errors remain unchanged and no raw state is copied into new errors.

## 8. Test Coverage

Focused coverage proves:

- cancellation before entry is zero-write and performs no identity generation
  or executor entry;
- cancellation after admission does not interrupt the active attempt and is
  observed at the subsequent timer wait;
- cancellation remains idempotent after owner return;
- two competing owners admit at most one executor entry through existing Core
  reservation and replay boundaries;
- cancellation and entry have one mutex-linearized decision;
- duplicate entry fails with a stable non-leaking error; and
- owner Debug output does not disclose bound run identity.

The unchanged workspace suites continue to cover exact-input substitution,
restart and rehydration, terminal and blocked entry, lawful timer wakes,
unsupported waits, wake-budget exhaustion, reservation races, timer wakeup,
and runtime privacy at the lower boundaries composed by the owner.

## 9. Validation Commands And Results

- focused process-owner tests: passed
- focused cancellation-linearization test: passed
- `cargo fmt --all --check`: passed
- `cargo clippy --workspace --all-targets -- -D warnings`: passed
- `cargo test --workspace`: passed
- `npm run check:docs`: passed
- `git diff --check`: passed

The default incremental Rust target stopped making observable compilation
progress in this desktop environment. Warning-denied clippy and full workspace
tests were rerun with `CARGO_INCREMENTAL=0` and a clean target directory at
`/private/tmp/workflow-os-owner-target`; both completed successfully. This was
a local build-cache recovery, not a source or product workaround.

## 10. Remaining Known Limitations

- No application constructs or invokes the owner automatically.
- No discovery, daemon, startup recovery, detached execution, durable owner,
  owner-loss detection, signal integration, or stuck-work detection exists.
- Active executor attempts remain non-interruptible by this cancellation
  handle.
- A new explicit process call receives a fresh fixed wake budget.
- Multi-process ownership and hosted/PostgreSQL scheduling remain deferred.
- Operator notification, WorkReport projection, and durable host-job evidence
  are not part of this private slice.

## 11. Recommended Next Phase

Perform a focused maintainer/security review of this implementation. The
review should verify the cancellation-versus-entry linearization, zero-write
pre-entry outcome, active-attempt limitation, competing-owner posture,
one-shot ownership, privacy, and strict absence of public or automatic runtime
behavior.

Do not begin application adoption, automatic continuation, discovery, signal
orchestration, provider mutation broadening, OpenShell, nested harnesses,
hosted scheduling, or public runtime surfaces before that review accepts the
implementation.

## 12. Governed Implementation Record

- workflow: `dg/implement`
- run: `run-1791371080612186000-2`
- approval: `approval/run-1791371080612186000-2/implementation-approved`
- presentation: `presentation/6bbfdc9335714c92`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: crate-private one-shot trusted-host local process owner
- phase status: completed
- event summary: 39 events, including 1 approval request, 1 approval grant, 0
  retries, and 0 escalations
- approval-presentation enforcement: proof enforced through the persisted
  presentation record
- out-of-kernel execution: source inspection, code and documentation edits,
  command execution, validation, and later git and pull-request actions are
  performed by the delegated trusted host; Workflow OS governs scope and
  approval but does not edit files or execute shell commands
