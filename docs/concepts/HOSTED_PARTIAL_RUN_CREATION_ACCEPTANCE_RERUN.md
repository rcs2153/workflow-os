# Hosted Partial-Run Creation Acceptance Rerun

## Verdict

The interrupted-prefix backup and restore gate passes on exact `main` commit
`ecf0b8cc3cd6fd4c7ecd972bc7a2a6e5ba30740a`. A bounded blocker fix on the
acceptance branch also carries 25 distinct simultaneous hosted create requests
to lawful queued posture while preserving serializable isolation.

An isolated PostgreSQL 18.6 rehearsal preserved a `RunCreated`-only hosted
creation prefix through `pg_dump` and `pg_restore`. The restored backend
appended the exact missing suffix once, rehydrated the run as `Running`, and
appended nothing on exact replay.

A new permanent contention fixture then released 25 distinct hosted create
requests simultaneously against the same isolated PostgreSQL backend. Three
requests reached their lawful queued posture on the first attempt. Twenty-two
returned `postgres_state.transaction.retryable`. The result is bounded and
safe, but it does not satisfy the accepted product requirement that ordinary
hosted create traffic at the externally reported scale reaches a lawful
continuation without exposing internal transaction contention as an
unresolved caller burden.

The hosted partial-run recovery P0 remains open through focused review,
exact-head CI, and an independent clean-environment rerun of the corrected
contention path. Cross-harness implementation remains blocked until those
gates pass.

## Exact Source And Environment

- source commit: `ecf0b8cc3cd6fd4c7ecd972bc7a2a6e5ba30740a`
- source subject: `Recover partial hosted run creation (#783)`
- PostgreSQL: isolated native PostgreSQL 18.6 cluster under `/private/tmp`
- source database: isolated `workflow_os_test`
- restored database: isolated `workflow_os_restore`
- production source changes during initial rerun: none
- bounded blocker fix: hosted no-write admission retries around only the
  immutable-bundle, creation-prefix, and dispatch storage operations
- acceptance changes: permanent contention, exhaustion, and restore fixtures
  plus this evidence record

Exact-head GitHub CI for PR #783 was terminal green across nine jobs,
including the Rust workspace, shared PostgreSQL state, hosted alpha recovery,
fresh-checkout validation, packaging, integrations, docs, and dependency
security lanes.

## Restore Evidence

The recovery rehearsal now performs this sequence:

1. create or reconcile the existing collaborative approval receipt fixture;
2. seed one exact hosted creation prefix containing only `RunCreated`;
3. dump the source database with `pg_dump`;
4. restore it into a new database with `pg_restore`;
5. verify the interrupted prefix is still exactly one event;
6. reconcile the prefix to the complete scheduled sequence;
7. verify the repaired snapshot is `Running`; and
8. repeat reconciliation and prove no additional event is appended.

The rehearsal passed. This closes the accepted backup/restore requirement for
one interrupted early hosted prefix. It does not establish tamper-evident
storage or universal disaster-recovery guarantees.

## Contention Reproduction And Fix Evidence

The permanent fixture constructs 25 distinct immutable hosted create
requests, synchronizes their first execution behind one barrier, and executes
them through the public Core hosted no-write dispatch path against one shared
PostgreSQL backend.

Observed first-attempt outcomes:

- successful lawful queued posture: 3;
- `postgres_state.transaction.retryable`: 22;
- duplicate accepted work: not observed before the test stopped;
- unsafe widening or conflicting authority: not observed.

The error was intentionally classified as retryable, but the pre-fix hosted
boundary did not absorb or durably coordinate those retries for ordinary
callers. Treating the 22 failures as test success would merely have moved the
continuity burden back to the caller and contradicted the product invariant
that accepted work reaches one lawful executable posture.

Temporary stage diagnostics classified the 22 exhausted requests:

- immutable-bundle publication: 16;
- creation-prefix reconciliation: 5;
- hosted dispatch: 1.

The evidence ruled out a single-table fix. The hosted no-write admission seam
now owns a bounded retry contract around exactly those three idempotent
storage operations. It does not rerun policy evaluation, provider execution,
or arbitrary PostgreSQL closures. Each operation retains the existing
serializable transaction and exact replay contract.

The hosted admission wrapper:

- accepts only `postgres_state.transaction.retryable` for retry;
- allows at most eight admission attempts per storage stage;
- uses a short deterministic exponential delay salted by the run identity;
- returns a stage-specific bounded exhaustion code; and
- returns every non-retryable error immediately.

After the fix, the same 25-request barrier fixture completed all 25 distinct
requests successfully and created 25 additional queued work items. The
focused shared-state milestone passed in 2.05 seconds on the isolated local
database.

## Exhaustion And Recovery Evidence

PostgreSQL triggers forced each hosted admission storage stage to fail with
SQLSTATE `40001`. The hosted path exhausted its bounded budget with the
corresponding stage-specific code:

```text
executor.hosted_no_write.immutable_bundle_retry_exhausted
executor.hosted_no_write.creation_prefix_retry_exhausted
executor.hosted_no_write.dispatch_retry_exhausted
```

Immutable-bundle exhaustion left zero events, no immutable manifest, and no
hosted work item. Prefix exhaustion preserved the already committed immutable
bundle but left zero run events and no work item. Dispatch exhaustion preserved
the complete scheduled prefix but committed no invocation request/start event
and no work item. After each fault was removed, the exact request reached one
queued posture and exact replay did not add another work item.

## Implemented Blocker Fix

The blocker fix satisfies the accepted requirements to:

- preserve exact immutable request identity;
- retry only errors explicitly classified as retryable;
- use a bounded attempt and timing policy;
- re-enter the existing Core-owned reconciliation seam rather than inventing
  a second creation or dispatch path;
- expose exhaustion as a bounded, inspectable wait or failure posture;
- prevent duplicate events, work items, invocations, approvals, SideEffects,
  receipts, and reports; and
- prove at least 25 distinct concurrent requests reach lawful posture under
  the declared retry contract.

The Core hosted no-write admission seam is the selected owner. Generic
PostgreSQL storage operations retain their existing retry contract and do not
silently retry arbitrary application closures beyond that established
boundary.

## Validation

- 25-distinct-job PostgreSQL shared-state milestone: passed.
- all three stage-specific forced retry exhaustion, bounded-state assertion,
  recovery, and replay cases: passed.
- interrupted-prefix backup and restore rehearsal: passed.
- `cargo clippy -p workflow-core --all-targets -- -D warnings`: passed.
- `cargo test -p workflow-core --test local_executor`: 351 passed, one opt-in
  live test ignored.
- `npm run check:docs`: passed.
- `cargo fmt --all --check`: passed.
- `git diff --check`: passed.

## Governed Phase

- workflow: `dg/review`
- run: `run-1794016202169853000-2`
- approval: `approval/run-1794016202169853000-2/review-scope-approved`
- presentation: `presentation/912a7f54aaadd36b`
- approval outcome: granted under delegated-maintainer authority
- out-of-kernel work: isolated PostgreSQL lifecycle, test execution, fixture
  edits, evidence analysis, and this report

The subsequent blocker phase is governed by `dg/blocker` run
`run-1794016741450425000-2`, approval
`approval/run-1794016741450425000-2/fix-approved`, and presentation
`presentation/83cfdc8bf65aee25`.

## Non-Goals

This rerun does not authorize provider mutation broadening, cross-harness
implementation, nested agents, release work, hosted production claims,
unbounded retries, caller-authored repair events, or weakened transaction
isolation.
