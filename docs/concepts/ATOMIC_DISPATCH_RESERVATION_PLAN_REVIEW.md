# Atomic Dispatch Reservation Plan Review

## 1. Executive Verdict

**Needs one planning blocker fix before implementation.**

The one-winner transaction, non-reconstructable authority, replay posture,
ambiguous-commit posture, and SQLite-only boundary are security-sound and fit
the current trusted-host supervisor. The plan is not yet implementation-ready
because it leaves the durable event/audit representation of dispatch admission
open while Workflow OS requires meaningful runtime transitions to remain
append-only and inspectable.

## 2. Scope Verification

The plan stays within the approved planning boundary. It does not authorize a
scheduler, repeated loop, model turn, provider execution, OpenShell, nested
harness execution, automatic approval, public runtime configuration, CLI,
SDK, workflow schema, hosted behavior, or another mutation family.

The proposed implementation remains private, local, SQLite-only, and limited
to admitting one exact started attempt before one executor entry.

## 3. Threat And Invariant Assessment

The plan identifies the current race correctly. The supervisor performs a
read-only `attempt_dispatch_is_current(...)` check before executor entry. Two
concurrent in-crate claimants can both pass that read before either persists an
outcome. Outcome idempotency is too late because external work may already
have happened twice.

The following planned invariants are accepted:

- one durable winner per exact attempt;
- no executor entry before committed admission;
- no authority reconstructed from durable state;
- no capability on exact replay;
- no capability after ambiguous commit reconciliation;
- exact invocation, executor, actor, authority, governance, cursor, revision,
  expiry, epoch, window, and attempt binding;
- no inference that reservation means attempt or workflow completion; and
- fail-closed process-loss behavior after admission and before executor entry.

## 4. Transaction Boundary Assessment

An immediate SQLite transaction with an attempt-identity uniqueness
constraint is the correct first implementation boundary. It composes with the
existing operational-opening and continuity stores and addresses the race at
the only safe point: before the executor callback can run.

The transaction must use the injected trusted-time source and must perform all
current-state, binding, and uniqueness checks inside the same transaction that
inserts the reservation. A pre-transaction observation may be carried into the
request, but it must be validated and monotonically committed under the
transaction rather than treated as sufficient on its own.

## 5. Capability And Replay Assessment

The proposed `ReservedAttemptDispatchCapability` is appropriate only if its
fields and constructor remain private to the reservation module. Crate-private
type visibility alone is insufficient if sibling modules can construct an
equivalent value directly. The implementation must expose only a consuming
supervisor entrypoint or a constructor that cannot be called without the
transaction's first-delivery result.

Exact replay returning a receipt without authority is correct. A reconciled
after-commit result returning `CommittedButCapabilityUnavailable` is also
correct. Neither path may be convertible into supervisor input.

## 6. Supervisor And Persistence Assessment

Replacing `attempt_dispatch_is_current(...)` with a consumed reservation
capability is the correct integration. Invocation and executor bindings should
still be checked immediately before callback entry as defense in depth.

The reservation binding must also be carried into:

- attempt outcome persistence;
- yield registration; and
- ambiguous-attempt persistence.

Those operations must verify the exact reservation receipt and commitment in
the same transaction that changes attempt posture. Merely possessing the old
attempt capability after reservation is not sufficient.

## 7. Blocking Finding

### P0: Dispatch admission has no resolved append-only event/audit contract

The plan proposes a durable reservation record and projection, but section 17
still asks which event should represent reservation. Dispatch admission is a
meaningful runtime transition because it is the durable decision that permits
external execution. Storing it only in a private table would make the event
history incomplete and conflict with the engineering standard's append-only
runtime-state requirement.

The blocker fix must choose and document one exact representation before code:

1. add a bounded payload-free dispatch-admission event/projection vocabulary;
2. bind it atomically to the reservation insert and run snapshot projection;
3. define exact replay and ambiguous-commit inspection semantics;
4. keep it distinct from attempt completion, yield, and workflow completion;
5. define migration and compatibility posture honestly; and
6. explain why this event does not create a sixth caller-visible continuity
   operation or authorize execution by itself.

The fix may add private or runtime-event vocabulary as required, but it must
not expose a workflow-spec field, CLI command, SDK contract, or provider
execution surface.

## 8. Non-Blocking Hardening Requirements

- Localize capability construction to the reservation module and test that
  replay/reconciliation outcomes cannot reach the executor API.
- Name the exact reservation receipt and commitment fields added to outcome,
  yield, and ambiguity requests so the binding is not deferred to coding
  judgment.
- Keep the admitted reservation immutable; derive settlement from the bound
  attempt outcome rather than rewriting the admission record.
- Add a test where two independent SQLite connections contend for the same
  attempt and a counter proves that only one executor callback runs.
- Add crash/fault tests at before-commit, after-commit-before-return, and
  admitted-before-executor boundaries.

## 9. Privacy Assessment

The payload-free posture is appropriate. Reservation persistence and events
should contain bounded identities, commitments, revisions, trusted-time facts,
and disposition only. Debug and error output must not expose caller values,
paths, prompts, transcripts, commands, environment values, credentials, or
provider payloads.

## 10. Test Plan Assessment

The proposed tests cover concurrency, replay, substitution, faults, privacy,
migration, and existing-runtime regression. The blocker fix must add explicit
event-ledger assertions:

- one admission produces one ordered dispatch-admission event;
- a losing claimant produces no admission event;
- exact replay produces no duplicate event;
- ambiguous reconciliation returns the already-committed event binding;
- corrupt or missing reservation/event pairs fail closed; and
- rehydration and snapshot projection agree with the reservation relation.

## 11. Blockers

One blocker remains: resolve the append-only dispatch-admission event and
projection contract before implementation.

## 12. Recommended Next Phase

Run a **bounded atomic dispatch-reservation planning blocker fix**. Update the
plan to define the exact payload-free admission event, atomic projection,
reservation-to-outcome binding fields, and module-private capability
construction boundary. Then repeat this focused review before implementation.

## 13. Governed Review Record

- workflow: `dg/review`
- run: `run-1790909016313961000-2`
- approval: `approval/run-1790909016313961000-2/review-scope-approved`
- presentation: `presentation/41a38db2407503b8`
- presentation hash:
  `41a38db2407503b8193ac132a1c9d018936c7a4432320317d3d978e0479191fb`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: completed
- event summary: 39 events, including one approval request, one approval
  grant, eight policy decisions, six scheduled steps, and six successful skill
  invocations; no retries or escalations
- validation summary: `npm run check:docs` and `git diff --check` passed
- reviewed boundary: planning artifacts and existing private supervisor/store
  integration surfaces only
- out-of-kernel work: source inspection, review authoring, validation, and git
  are performed by the delegated maintainer; Workflow OS governs the phase but
  does not edit repository files or run shell commands
