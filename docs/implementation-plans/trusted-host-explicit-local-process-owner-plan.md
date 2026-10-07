# Trusted-Host Explicit Local Process Owner Plan

Status: focused maintainer/security review found planning blockers in the
pre-entry cancellation, process-shutdown, canceled-result, and cancellation
race contracts. The documentation-only blocker fix below defines an atomic
pre-entry cancellation linearization point, a truthful owner-level canceled
outcome, and the non-interruptible active-attempt boundary. Focused re-review
is required before implementation. This plan does not implement runtime
behavior. See the
[focused review](../concepts/TRUSTED_HOST_EXPLICIT_LOCAL_PROCESS_OWNER_PLAN_REVIEW.md)
and [blocker-fix report](../concepts/TRUSTED_HOST_EXPLICIT_LOCAL_PROCESS_OWNER_PLAN_BLOCKER_FIX_REPORT.md).

## 1. Executive Summary

Workflow OS now has an accepted crate-private composition that enters one
already-selected trusted-host operation and, only when Core returns an
authoritative `AwaitCondition`, invokes the accepted bounded local production
caller. The composition remains unowned: no process boundary creates its
cancellation pair, retains the cancellation handle, invokes it exactly once,
and takes responsibility for the returned stop posture.

The next implementation should add that smallest owner. It should be a
crate-private, one-shot, synchronous object created with one exact operational
entry input. Construction returns the owner plus the existing cancellation
handle. The invoking process may retain or clone that handle for cooperative
pre-entry and timer-wait cancellation, then consume the owner through one
blocking `run` call. The owner atomically linearizes cancellation against
entry, delegates admitted work entirely to
`run_explicit_trusted_host_local_operation`, and returns one bounded owner
outcome.

This boundary is not run discovery, automatic continuation, a daemon, a
background worker, or a public runtime feature.

## 2. Goals

- Give the accepted private operation composition one explicit process-owned
  lifecycle.
- Create the existing cancellation pair before synchronous execution starts.
- Return a cancellation handle to process control while the owner retains the
  receiver.
- Require one exact, already-selected operational-entry input.
- Make owner execution one-shot by consuming the owner.
- Preserve Core as the only source of authority, eligibility, wait posture,
  transition, reservation, and terminal truth.
- Return the accepted operation outcome without translating it into workflow
  completion, success, retry, or operator advice.
- Define bounded cooperative cancellation, process-shutdown limitations,
  restart, concurrency, error, privacy, and test posture before implementation.

## 3. Non-Goals

This phase must not add:

- runtime implementation in the planning prompt;
- run, workflow, step, window, or wait discovery;
- startup scanning, directory scanning, queue polling, or state polling;
- a daemon, service, detached task, background thread, worker pool, or async
  runtime;
- automatic invocation, automatic restart, or automatic approval;
- a public API, CLI command, SDK method, schema, workflow field, runtime
  configuration, or environment-variable switch;
- owner persistence, host-job persistence, callback registration, leases,
  leader election, or multi-process coordination;
- LocalExecutor or filesystem-state bridging;
- provider reads or mutations, OpenShell integration, nested harnesses, or
  agent teams;
- additional wait-source families or a changed wake budget;
- hosted or PostgreSQL scheduling parity;
- signal-handler installation or operating-system service integration;
- durable skill-output retrieval; or
- release posture changes.

## 4. Selected Boundary

Add one crate-private sibling module under SQLite trusted-host state, likely:

```text
trusted_host_explicit_local_process_owner
```

The module should depend on the accepted private types only:

- `TrustedHostOperationalEntryInput`;
- `TrustedHostLocalTimerCancellation` and its handle;
- `TrustedHostExplicitLocalOperationInput`;
- `TrustedHostExplicitLocalOperationOutcome`; and
- `run_explicit_trusted_host_local_operation`.

It must not query durable state to find work. The exact backend, locator,
opening context, executor, skill input, opening persistence, and initial
identity provider must already be selected by the caller.

## 5. Candidate Private API

The smallest candidate shape is:

```rust
pub(crate) struct TrustedHostExplicitLocalProcessOwner<'a> {
    operational_entry: TrustedHostOperationalEntryInput<'a>,
    cancellation: TrustedHostLocalTimerCancellation,
}

pub(crate) enum TrustedHostExplicitLocalProcessOwnerOutcome {
    CanceledBeforeEntry,
    OperationStopped(TrustedHostExplicitLocalOperationOutcome),
}

impl<'a> TrustedHostExplicitLocalProcessOwner<'a> {
    pub(crate) fn new(
        operational_entry: TrustedHostOperationalEntryInput<'a>,
    ) -> (
        Self,
        TrustedHostLocalTimerCancellationHandle,
    );

    pub(crate) fn run(
        self,
    ) -> Result<TrustedHostExplicitLocalProcessOwnerOutcome, WorkflowOsError>;
}
```

Names may follow final repository conventions, but the semantics must remain
this narrow. `new` only creates process-local cancellation state. `run`
consumes the owner, atomically competes with pre-entry cancellation, and calls
the accepted composition at most once.

The owner must not implement `Clone`, `Serialize`, or `Deserialize`. It should
not expose its receiver or synthesize a second cancellation pair.

## 6. Owner Lifecycle

The v1 lifecycle is intentionally process-local and finite:

1. **Constructed**: the caller already possesses one exact operational input;
   owner construction creates no durable state and returns a cancellation
   handle.
2. **Entry decision**: consuming `run` atomically marks entry started unless
   cancellation already won. Pre-entry cancellation returns
   `CanceledBeforeEntry` without Core reads, writes, or executor admission.
3. **Running**: after entry wins, `run` transfers the exact input and receiver
   into the accepted explicit local operation. The call remains synchronous on
   the invoking thread. Later cancellation cannot revoke admission or
   interrupt an active attempt; it is observed at a subsequent timer wait.
4. **Closed**: `run` returns one bounded owner outcome or a structured error.
   The owner is consumed and cannot be restarted.

There is no detached `Started` state and no durable owner record. A dropped
owner that was never run performs no Core mutation. Dropping a cancellation
handle does not claim cancellation. Process death does not convert a run into
failure or completion; durable Core state remains authoritative for a later
explicit reconstruction.

## 7. Authority And Exact-Input Boundary

The owner accepts no capability, approval decision, wait-satisfaction claim,
or caller-authored continuation disposition. It cannot reconstruct authority.

The operational input retains the same backend object, locator, immutable run
bundle, opening context, executor, skill input, persistence identities, and
identity provider already required by the accepted entry boundary. The owner
must not clone or rewrite those bindings merely to simplify ownership.

All admission, current-authority reassessment, invocation commitment,
directive consumption, dispatch reservation, trusted-time evaluation, and
executor entry remain inside the existing Core operations.

## 8. Cooperative Cancellation And Process Shutdown Limits

Owner construction must create the existing cancellation receiver/handle pair
before `run` starts. The owner retains the receiver. The invoking process
retains the handle and may clone it for one explicit cooperative-control path.

The shared private cancellation state should add an `entry_started` marker and
one zero-write `begin_entry` operation. `begin_entry` and `cancel` use the same
mutex as their linearization boundary:

- if `cancel` commits first, `begin_entry` returns canceled and `run` returns
  `CanceledBeforeEntry` without invoking the operational boundary;
- if `begin_entry` commits first, initial admission may proceed and later
  cancellation cannot revoke consumed authority or interrupt an active
  executor attempt; and
- the cancellation flag remains set so the next local timer wait returns
  canceled promptly.

This state is process-local, non-authorizing, non-serializable, and
non-durable. It must not create a workflow event or mutate Core state.

Cooperative cancellation must:

- prevent all Core reads, writes, and executor entry when it wins before the
  `begin_entry` linearization point;
- wake an active local deadline wait promptly;
- return a truthful bounded owner or timer-canceled posture;
- avoid claiming workflow cancellation, failure, or completion;
- avoid creating authority or satisfying a wait condition; and
- remain idempotent through the existing handle.

The handle is not a general process-shutdown or executor-interruption token.
The first implementation must not install signal handlers, spawn a monitor,
interrupt an active attempt, or map process shutdown into a workflow event. A
later adoption site must decide how an application obtains and invokes the
handle and how it waits for an admitted attempt to return during shutdown.

## 9. Result And Stop Semantics

The owner should return the minimal owner-level outcome shown above.
`CanceledBeforeEntry` states that no operational entry occurred.
`OperationStopped` contains the exact accepted operation outcome without
translation and preserves `EntryStopped` output.

The caller remains responsible for distinguishing a pre-entry cancellation
from an admitted operation that stopped terminal, blocked, canceled while
waiting, awaiting an unsupported condition, or exhausted its fixed wake
budget. None of those host-level stops implicitly changes workflow terminal
status.

No logging, metrics, audit event, WorkReport, report artifact, operator card,
or resume recommendation is added in this slice. The planning report and
future implementation report must disclose that operator notification remains
manual and outside the kernel owner.

## 10. Restart And Reconstruction

The owner is not durable. After process loss, a new explicit caller may create
a new owner only after independently reconstructing the same exact operational
input from accepted sources. The new call then rehydrates current Core state
through the accepted entry boundary.

The owner must not persist or recover:

- cancellation state;
- identity-source state;
- wake-budget remainder;
- capabilities;
- in-memory executor objects;
- skill input payloads; or
- skill output.

The fixed wake budget intentionally resets for a new explicit process call;
it is a bounded host-loop limit, not a durable governance quota.

## 11. Concurrency And Single-Owner Posture

The owner does not establish global singleton ownership. Two explicitly
constructed owners may race. Existing transactional opening, directive,
reservation, replay, and attempt boundaries must still admit at most one
executor entry.

Because this is the first process-owned wrapper, implementation acceptance
requires a direct competing-owner test at this exact boundary. The test must
prove aggregate executor admission is at most one and that the losing owner
returns existing bounded posture or a stable non-leaking error. The owner must
not add a process-local mutex that hides Core concurrency behavior.

Multi-process ownership, leader election, leases, and owner-loss detection
remain deferred.

## 12. Failure And Privacy Posture

Construction should be infallible except for memory allocation behavior
outside the model: creating the current cancellation pair does not read or
write Core state.

`run` returns existing structured errors unchanged. A poisoned cancellation
mutex returns the existing stable timer-state error before entry. The owner
must not retry
entropy, timer, SQLite, trusted-time, authority, replay, reservation, or
executor failures.

Custom `Debug` must expose only a bounded redacted marker. It must not expose
workflow, run, step, window, actor, immutable-bundle, input, output, path,
deadline, identity, credential, token, or provider values. The owner adds no
serialization or persistence surface.

## 13. Required Tests

The first implementation should add focused tests proving:

1. construction creates no durable state;
2. `run` invokes the accepted explicit local composition once;
3. terminal and blocked entry outcomes return unchanged without waiting;
4. one lawful `TimeWindow` wait returns the accepted continuation outcome;
5. cancellation before `run` returns `CanceledBeforeEntry` with no Core read,
   write, event, identity generation, or executor entry;
6. a deterministic cancel-versus-entry race proves exactly one linearized
   result: no entry when cancellation wins, or admitted non-interruptible work
   when entry wins;
7. cancellation while the attempt executor is active does not claim
   interruption, revoke admission, or duplicate entry, and is observed if the
   operation later reaches a timer wait;
8. a retained handle cancels an active wait promptly from a scoped process
   control thread;
9. cancellation after owner return is harmless and idempotent;
10. cancellation does not mark the workflow failed or completed;
11. dropping an unrun owner creates no state or events;
12. a reopened backend plus newly constructed owner rehydrates current state;
13. substituted input remains rejected before executor admission;
14. two competing owners admit at most one executor entry;
15. unsupported wait and wake-budget exhaustion remain bounded and are not
    retried;
16. errors and `Debug` do not leak secret-like bindings; and
17. no public export, CLI, schema, provider, artifact, or hosted behavior is
    introduced.

Tests should reuse real accepted constructors and SQLite state rather than
fabricating capabilities or editing state by hand.

## 14. Candidate Implementation Sequence

1. Focused maintainer/security review of this plan.
2. Add the one-shot private owner type and constructor.
3. Add the shared private pre-entry linearization operation and minimal
   owner-level outcome.
4. Implement consuming `run` as at most one delegation to the accepted
   composition.
5. Add direct lifecycle, cancellation, restart, concurrency, and privacy
   tests.
6. Run focused and workspace validation.
7. Perform a focused implementation/security review.
8. Only after acceptance, plan one explicit application adoption site.

The implementation must start and end with the private owner boundary. It
must not combine owner implementation with CLI, discovery, background work,
or automatic continuation.

## 15. Open Questions Deferred To Later Phases

- Which application or process will first construct the owner?
- How will that application select an exact operation without scanning?
- How will process signals or shutdown orchestration invoke cooperative
  cancellation and await admitted attempts?
- Where will bounded owner outcomes be surfaced to an operator?
- How will owner loss or stuck work be detected?
- Is durable terminal-output retrieval required before public adoption?
- When is a durable host-job identity justified?
- What parity is required for PostgreSQL or hosted workers?

None of these questions may be answered implicitly by the first private owner
implementation.

## 16. Validation For This Planning Phase

Planning validation is limited to:

- `npm run check:docs`; and
- `git diff --check`.

No Rust behavior changes in this phase.

## 17. Final Recommendation

Proceed next to a focused maintainer/security re-review of the corrected plan.

Do not implement the owner until review confirms that cancellation ownership,
one-shot lifecycle, exact-input retention, direct competing-owner proof, and
manual operator boundary are sufficiently explicit.
