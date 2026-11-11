# Hosted Invocation Result Interruption Report

## Scope And Status

Candidate correction for the independently reported worker-death boundary,
separate from creation-prefix recovery and live provider continuation.
Final maintainer recheck accepts the bounded correction; exact-head CI remains
required. This is not an unattended
hosted admission or a claim that an unknown provider outcome was recovered.

## Reproduction

On base `361f4216`, the disposable PostgreSQL 18.6 rehearsal created one
no-write hosted run through HTTP. A database trigger blocked insertion of
`SkillInvocationSucceeded` inside the result transaction. The rehearsal
killed the worker and terminated its database connection, causing rollback,
removed the trigger, waited for the real lease to expire, and launched a
replacement process. No kernel records were edited to force recovery.

The attempt remained `invoking` without a success event. The replacement
failed with `hosted.execution_attempt.transition.invalid`: it tried to mark
the already-invoking attempt as invoking again. The original worker could
have executed; neither repeating it nor claiming completion was justified.

## Correction

- The worker validates the existing exact attempt binding, then projects a
  reclaimed `Invoking` attempt through the existing fenced, atomic unreceipted
  result path instead of calling the provider again.
- That transaction records an ambiguous work item, a reconciliation-required
  attempt and one escalation event, and releases the lease. It does not create
  a receipt, successful invocation, completion event or WorkReport.
- Exact create replay acknowledges the existing escalated posture without
  dispatch. Later workers see no eligible work rather than repeatedly crashing
  on the same attempt.
- Worker output preserves its actual outcome. Reconciliation, rejection and
  cancellation are no longer collapsed into the misleading `completed` label.
  A unit test covers the non-receipt outcome labels.

## Validation Contract

`scripts/rehearse-hosted-result-interruption.mjs` requires an explicitly
selected empty disposable database and an already-built hosted binary. It
uses a no-write provider and loopback API only. Its assertions cover worker
death inside the transaction, rollback, real lease expiry, replacement,
exact replay, one attempt and escalation, idle subsequent worker, stable
event count, and absence of success events and report artifacts. It refuses
an existing Workflow OS schema rather than overwriting user state. The CI
PostgreSQL job runs it in a separate disposable database.

The final PostgreSQL 18.6 process-death/replacement/replay rehearsal passed:
`hosted_result_interruption: passed; durable reconciliation, no repeated
invocation or fabricated report`. Both earlier failures were observed before
their corrections: invalid repeated invoking transition, then misleading CLI
completion output. A further rerun caught the escalated create-replay rejection
before the final replay correction. One fresh startup attempt timed out before
the fault stage and was discarded; no recovery claim is based on that attempt.
The final rehearsal used a fresh database and real lease expiry. Local docs,
JavaScript syntax and whitespace checks passed. The final candidate passed
`cargo test -p workflow-hosted` (38 library tests, one binary test, and zero
documentation tests), `cargo clippy -p workflow-hosted --all-targets -- -D
warnings`, and `cargo fmt --all --check`. The optional collaborative database
case in the library suite was not activated; the new standalone rehearsal
did exercise a real local PostgreSQL database. The disposable API and workers
were cleaned up and the local cluster stopped, retaining diagnostic data.
Full exact-head CI, including shared PostgreSQL and hosted recovery, remains
the delivery boundary. No live-provider, installation, custody or production
deployment testing is implied by this no-write regression.

## Remaining Boundary

The generic provider interface has no authoritative recovery lookup. This
change classifies uncertain execution into a durable, inspectable escalation;
it does not recover an actual missing receipt or authorize another invocation.
Supervised reconciliation remains necessary. A missing report in this posture
is disclosed uncertainty, not successful execution evidence. Provider-specific
recovery must separately prove the exact outcome before completion.

## Focused Maintainer Review

Code-path review found no new blocking authority defect in the worker change,
but real PostgreSQL conformance exposed a blocking timestamp conversion defect.
`prepare_hosted_execution_attempt_tx` validates the exact invocation identity,
request fingerprint and provider configuration before returning the prior
attempt. Recovery is selected only for its `Invoking` posture. The existing
unreceipted projection transaction checks the work-item revision, current
lease fence and attempt revision, then commits the escalation, snapshot,
ambiguous queue item and reconciliation-required attempt together and releases
the lease. A stale holder cannot commit through that path.

Acknowledging `Escalated` on exact create replay does not resume execution:
the immutable bundle and creation prefix still validate, and only the
`Running` match arm can dispatch. The rehearsal asserts no additional events
on replay and no eligible job for the subsequent worker. Success remains
receipt-bound; the CLI now distinguishes processed work from completed work.

The new CI rehearsal uses its own empty PostgreSQL database and does not
alter the shared conformance or backup/restore database. It runs with Node 20;
the recorded local crash test used Node 24.18.0 and PostgreSQL 18.6. The
PostgreSQL 17.5 CI run is still required before delivery.

This owner-conducted review is not independent security assurance. Review run
`run-1794351881039926000-2` has proof-backed approval
`approval/run-1794351881039926000-2/review-scope-approved`. Broader local
PostgreSQL conformance did run, with `WORKFLOW_OS_REQUIRE_POSTGRES_TESTS=1`,
and failed at the unreceipted projection with
`hosted.execution_attempt.timestamp.regressed`. Do not report that suite as
passed or accept this candidate for delivery yet.

`database_timestamp_tx` converts extracted milliseconds with a PostgreSQL
`bigint` cast, which rounds rather than truncates. A deterministic synthetic
query at `2026-11-10T00:00:00.0006Z` returned `1794268800001` with the current
cast versus `1794268800000` with `floor`. That can put a database attempt time
fractionally ahead of a subsequent local-clock outcome. The helper predates
this change; no allowance for timestamp regression should be added.

Required blocker correction: floor database epoch milliseconds before integer
conversion, add a deterministic fractional-millisecond regression against real
PostgreSQL, and rerun the mandatory shared-state conformance and crash/replay
rehearsal. Preserve the existing time-regression rejection, exact bindings,
revision checks and lease fences. Broader cross-host clock-skew handling remains
outside this precision correction.

The precision correction candidate floors the shared database epoch-millisecond
expression before integer conversion. The deterministic PostgreSQL test uses
that same expression with sub-millisecond, second-boundary, pre-epoch and
current-era samples, then checks a production database clock sample against
the local after-sample time. CI explicitly runs the real-database test with
database absence treated as failure. No timestamp ordering check was relaxed.
This correction is governed by `run-1794352222301163000-2`, proof-approved at
`approval/run-1794352222301163000-2/fix-approved` under presentation
`presentation/b2a685122c9192ac`. The mandatory real-PostgreSQL precision test
and shared-state conformance both passed after the correction. The rebuilt
worker also passed a fresh Node 20 crash/replacement/replay rehearsal. The
Core library suite passed with 886 tests and 20 intentionally ignored tests;
database-specific cases in that general run were not activated, so its count
does not replace the separately required real-PostgreSQL runs.
`cargo clippy -p workflow-core -p workflow-hosted --all-targets -- -D warnings`,
format, docs and whitespace checks also passed. Delivery CI remains required.

### Final Recheck

Final owner-conducted recheck accepts the bounded correction with no remaining
blocking finding for classification and replay. The shared SQL expression uses
floor before conversion, including for negative epochs; no historical timestamp
or record is rewritten. The prior conformance failure and its correction remain
visible above rather than being replaced by a success-only account. The worker
still requires exact invocation/provider binding and an active fence, and the
atomic projection preserves uncertainty rather than issuing another invocation.

Recheck run `run-1794358886216292000-2` is proof-approved at
`approval/run-1794358886216292000-2/review-scope-approved`. The precision blocker
phase closed Completed with 39 events, one proof-backed approval, zero retries
and zero escalations. Final review and delivery are separate phases. Exact-head
PostgreSQL 17.5 CI and the remaining required jobs must be terminal green before
merge. Independent rerun, actual provider-outcome reconciliation and unattended
hosted admission are not supplied by this maintainer recheck.

## Governance

- Workflow: `dg/blocker`
- Run: `run-1794350459951714000-2`
- Approval: `approval/run-1794350459951714000-2/fix-approved`
- Presentation: `presentation/9b1032892eccf92c`
- Approval outcome: proof-backed grant under standing delegated maintenance
- Phase close: Completed, 39 events, one approval, zero retries and zero
  escalations. These are dogfood coordination events, distinct from the
  deliberately escalated disposable hosted run.
- Executor work: code and documentation edits, builds, local database fault
  injection, process interruption, HTTP checks and validation outside kernel
  handlers. Preview helper skills do not attest those operations.
- No live access, deployment, permission broadening or provider mutation.
