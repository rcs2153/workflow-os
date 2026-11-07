# Hosted Admission Contention Recurrence

## Evidence

The Shared PostgreSQL State job failed twice on the fixture bridge's reviewed
head `5e153bfb2076d8f4e21aed6c3707098633cc0417`. Both attempts failed the
25-distinct-job admission assertion. The second attempt exposed both a raw
serialization conflict and bounded creation-prefix retry exhaustion. The exact
base `8aa0077a872bd3de7a49c1630609dea6734f0feb` had one green CI run, which
did not establish that the defect was absent on base.

The exact CI conformance test ran in alternating order on archives of both
revisions, each trial using a fresh database in the same disposable PostgreSQL
18.6 cluster. Both revisions passed two of three trials and failed one at the
same assertion with `executor.hosted_no_write.dispatch_retry_exhausted`. This
proves recurrence on base and head, not that every CI failure had the same
transaction-level cause. GitHub CI uses PostgreSQL 17.5 on Linux.

## Repair Boundary

Hosted no-write creation now acquires one database-local PostgreSQL advisory
lock before entering admission. The lock uses a separate transaction from all
business writes and releases on normal return, errors, or connection loss. A
30-second lock wait budget reports `postgres_state.hosted_admission.busy`
without entering admission; the caller can retry the exact request. This
coordinates cooperating processes without changing serializable isolation,
stage-specific transaction retry policy, durable validation, or replay identity.
The lock is not an authority grant and is not required for transaction safety.

This intentionally serializes hosted no-write creates sharing a database
and uses one additional connection per contender. It does not serialize workers
or approval-resume dispatches, and cannot eliminate conflicts from those
operations, unrelated state operations, or other non-participating clients.
This is not a throughput claim.
No provider execution, connector, or fixture-bridge behavior changes.

## Local Validation

On PostgreSQL 18.6, all five post-fix trials passed the unchanged shared-state
milestone with fresh databases, including 25 simultaneous admissions and
exact replay without queue growth. The real two-connection lock test proved
timeout without invoking the waiting closure and lock release after error and
success. The hosted collaborative project boundary passed. Formatting,
warnings-denied Clippy for Core and Hosted, docs, and diff checks passed. The
disposable database was stopped after validation.

## Delivery Gate

Keep the original 25-job conformance and exact replay assertions. Verify the
new guard rejects timed-out contenders before their closure runs and releases
the lock after both failure and success. Before resuming the fixture PR, require
exact-head CI for this repair and then revalidate the fixture against updated
`main`. The governed blocker run is `run-1794032628302399000-2`; shell tests
and database lifecycle are performed outside the dogfood kernel.
