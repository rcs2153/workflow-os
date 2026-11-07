# Hosted Partial-Run Creation Final Acceptance

## Verdict

Accepted. The hosted partial-run creation recovery P0 is closed for the
declared local Core/PostgreSQL boundary.

The first exact-head CI run was correctly blocked after 24 of 25 concurrent
creates completed and one request exposed a raw retryable conflict from final
post-dispatch rehydration. That result was preserved as evidence, not retried
as a flake. A governed blocker correction bounded only that idempotent durable
read. Renewed exact-head CI then passed all nine jobs, including Shared
PostgreSQL State and the full Rust suite.

An additional clean-environment rerun from the exact merged commit passed the
25-job contention and exhaustion milestone, PostgreSQL backup/restore repair,
and a real database-process restart followed by a second complete milestone.

## Exact Source

- merged commit: `2668b619db4dc63774895e31d84d5b4619295514`
- merged PR: `#784`
- clean checkout: disposable detached clone under `/private/tmp`
- inherited build artifacts: none; the rerun used a separate Cargo target
- PostgreSQL: fresh native PostgreSQL 18.6 cluster and new source/restore
  databases under `/private/tmp`
- live provider credentials or writes: not used

## Exact-Head CI

The first PR head `e271e3d0881f72b59f19d73c36491b652f178999`
failed Shared PostgreSQL State after returning one raw
`postgres_state.transaction.retryable` result. The failure identified the
unguarded final durable rehydration read.

The amended head `d4d52174b7d66990921072c38b16addb94486e80`
passed all nine required jobs in GitHub Actions run `42330287305`:

- Rust;
- TypeScript And Docs;
- Schema SDK And Example Contracts;
- Phase 2 Read-Only Integration Contracts;
- Dependency And Security;
- Shared PostgreSQL State;
- Hosted Alpha Recovery;
- Fresh Checkout Aggregate Check; and
- Release Packaging Rehearsal.

The PR was squash-merged only after the exact head remained unchanged and all
jobs were terminal green.

## Clean-Environment Evidence

The disposable checkout compiled `workflow-core` into a separate target
directory and ran the permanent PostgreSQL shared-state milestone. It proved:

- 25 simultaneous distinct hosted creates reached lawful queued posture;
- each run recorded one invocation request and one invocation start;
- exact replay added no second work item;
- immutable publication, creation-prefix reconciliation, dispatch, and final
  rehydration use bounded stage-specific retry contracts;
- forced persistent write-stage conflicts left the declared partial posture;
- removing each fault allowed one exact recovery and idempotent replay; and
- unrelated non-retryable errors remained immediate failures.

The backup/restore rehearsal then proved:

- collaborative approval receipt recovery remained exact;
- a `RunCreated`-only hosted prefix survived `pg_dump` and `pg_restore`;
- the restored backend appended the missing suffix once; and
- replay appended nothing.

Finally, the PostgreSQL server process was stopped and restarted on the same
isolated data directory. The full shared-state milestone passed again after
restart.

## Boundary

This acceptance closes only the declared hosted partial-creation continuity
gap. It does not claim:

- exactly-once behavior beyond the tested Core-owned identity and idempotency
  boundary;
- cryptographic proof against a malicious host or storage owner;
- live provider mutation support;
- packaged cross-harness interoperability;
- hosted production readiness; or
- broader multi-tenant or distributed guarantees.

## Governed Acceptance

- workflow: `dg/review`
- run: `run-1794019995627512000-2`
- approval: `approval/run-1794019995627512000-2/review-scope-approved`
- presentation: `presentation/67cf3aef0107ceb6`
- approval outcome: granted under delegated-maintainer authority
- out-of-kernel work: disposable clone and Cargo target creation, isolated
  PostgreSQL lifecycle, test execution, evidence analysis, and this report

## Resulting Sequence

The hosted partial-run recovery preemption is complete. The separately queued
P0 public-positioning consistency correction remains next. After that bounded
documentation and repository-metadata repair, the active delivery queue may
advance to the cross-harness authoritative continuation milestone.
