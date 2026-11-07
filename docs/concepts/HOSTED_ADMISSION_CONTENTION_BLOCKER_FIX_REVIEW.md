# Hosted Admission Contention Blocker Fix Review

## Verdict

Accepted after two evidence corrections, then amended after exact-head CI
found one remaining unbounded admission read. The retry owner is correctly bounded
to Core's hosted no-write admission seam, the storage operations are already
idempotent and reconciliation-aware, and the fix does not retry policy
evaluation or external execution.

This amended review accepts the branch for renewed exact-head CI and independent
clean-environment acceptance. It does not close the hosted partial-run recovery
P0 or authorize cross-harness implementation.

## Retry Ownership

The initial 25-job trace showed exhausted serializable conflicts in three
places: immutable-bundle publication, creation-prefix reconciliation, and
hosted dispatch. Assigning retry ownership to only one table operation would
therefore leave the other admission stages exposed.

`execute_with_hosted_no_write_dispatch` now wraps those three storage
operations plus the final post-dispatch durable rehydration with one
hosted-specific retry helper. It does not wrap:

- pre-run policy evaluation;
- invoke-policy evaluation;
- runtime signal emission;
- provider execution;
- arbitrary state-backend calls; or
- generic PostgreSQL transaction closures.

The immutable publication is create-only with exact manifest validation. The
prefix reconciliation is transactionally locked and exact-request bound. The
dispatch operation is atomic and validates exact replay. Those existing
contracts make each selected retry safe without creating a second execution or
dispatch path. Final rehydration is a read of the already committed result; it
cannot create another event, work item, or provider invocation.

## Attempt And Timing Bound

Each selected storage operation retains its existing three-attempt
serializable transaction loop. The hosted admission seam allows at most eight
outer attempts for errors carrying the exact
`postgres_state.transaction.retryable` code.

Between attempts it sleeps for a deterministic exponential delay of 1, 2, 4,
8, 16, 32, and 64 milliseconds plus a run-identity salt from 0 through 6
milliseconds. The maximum outer sleep budget for one stage is therefore 169
milliseconds. Every non-retryable error returns immediately.

Exhaustion maps to a stable stage-specific code and a payload-free message. It
does not expose database details, silently widen authority, or report success.

## Stable Dispatch Identity

The hosted dispatch projection is constructed once before the bounded retry
loop. Invocation identity, attempt identity, event identity, timestamp,
idempotency key, immutable bundle binding, and project scope therefore remain
stable across retries. The retry closure invokes only the existing atomic
`dispatch_hosted_skill` operation.

## Review Corrections

The first blocker test proved aggregate queue growth for 25 simultaneous
requests but did not inspect each run. Review added per-run assertions for
exactly one invocation-request event and one invocation-start event, then
replayed all 25 requests and proved the queue count remained unchanged.

The first exhaustion test covered only immutable-bundle publication. Review
added forced SQLSTATE `40001` exhaustion at creation-prefix and dispatch stages
and verified each stage's durable partial posture before exact recovery and
replay.

The first exact-head CI run then completed 24 of 25 distinct requests and
returned one raw `postgres_state.transaction.retryable` error from the final
post-dispatch rehydration. The amended implementation applies the same bounded
contract to that idempotent read and adds direct success-after-conflict and
stage-specific exhaustion tests. CI was not rerun as a blind flake retry.

## Accepted Evidence

- 25 distinct simultaneous hosted creates reached lawful queued posture.
- Each run recorded exactly one invocation request and one invocation start.
- Replaying all 25 requests added no work item.
- Immutable publication exhaustion left no run, bundle, or work item.
- Prefix exhaustion preserved the bundle but left no run events or work item.
- Dispatch exhaustion preserved the scheduled prefix but left no invocation
  event or work item.
- Removing each fault allowed one exact recovery; replay remained idempotent.
- Final rehydration retries only the exact PostgreSQL retryable classification
  and maps persistent conflict to
  `executor.hosted_no_write.rehydrate_retry_exhausted`.
- An interrupted prefix survived PostgreSQL backup and restore, repaired once,
  and replayed without another append.
- Focused PostgreSQL, Clippy, local-executor, docs, formatting, and diff checks
  passed.

## Remaining Acceptance

- required CI must be terminal green for the exact reviewed head;
- an independent clean-environment rerun must repeat the 25-job, exhaustion,
  restart, and restore evidence; and
- the final acceptance report must retain the storage-owner and post-commit
  signal-delivery limitations.

## Governed Review

- workflow: `dg/review`
- run: `run-1794017448291203000-2`
- approval: `approval/run-1794017448291203000-2/review-scope-approved`
- presentation: `presentation/76548fe20784adb9`
- approval outcome: granted under delegated-maintainer authority
- out-of-kernel work: diff review, test corrections, focused validation, and
  this review record

## Non-Goals

This review does not authorize unbounded retries, weaker PostgreSQL isolation,
generic automatic transaction replay, provider mutation broadening,
cross-harness implementation, release work, or hosted production-readiness
claims.
