# Collaborative Approval Authority Receipt PostgreSQL Blocker Fix Review

## 1. Executive Verdict

**Blocker fixed; proceed to the same-call collaborative `ApprovalDecide`
consumer.**

The fix supplies the missing required-CI, restart, and backup/restore evidence
without broadening the receipt model or authorizing runtime consumption.

## 2. Scope Verification

The phase stayed within the approved verification scope. It changed focused
PostgreSQL tests, the existing recovery rehearsal, replay error ordering, and
honest roadmap/report status.

It did not add an approval consumer, executor/resume integration, authority
locks, hosted authentication, identity administration, provider writes,
workflow schemas, CLI/SDK behavior, examples, or release posture changes.

## 3. Original Blocker Restatement

The required Shared PostgreSQL State check previously proved the v4 migration
and receipt table but did not execute receipt write, exact replay, decision
recovery, conflict, or corruption behavior. The focused test could silently
skip outside a configured database, did not reopen the backend before
recovery, and the logical backup/restore rehearsal did not prove receipt
preservation.

## 4. Required CI Assessment

The recovery script already invoked by the required Shared PostgreSQL State
job now runs the exact focused receipt test against the configured source
database immediately before `pg_dump`. This makes receipt behavior and backup
seeding part of one required path rather than two loosely related checks.

`WORKFLOW_OS_REQUIRE_POSTGRES_TESTS=1` makes missing source test configuration a
hard failure. A focused local probe confirmed the exact test fails instead of
skipping when that required configuration is absent.

## 5. Restart Recovery Assessment

The focused test writes and exactly replays the receipt, drops the original
backend, creates a new backend from the same connection configuration, and
uses the reopened backend for exact-ID and decision-binding recovery. This is
an appropriate process-boundary approximation for the connection-factory
backend, which retains no database payload cache in memory.

## 6. Backup And Restore Assessment

The focused conformance test leaves one valid receipt in the source database.
The existing rehearsal dumps that database, recreates an isolated restore
database, restores the dump, and invokes the exact restore-integrity test.

The restore test reads the stable receipt, approval, and decision-event
bindings from the restored relational table and then recovers the receipt
through the public decision lookup. Required Shared PostgreSQL State CI passed
this complete path on PR #593.

## 7. Conflict And Corruption Assessment

Exact canonical replay remains idempotent, while approval/event rebinding
fails closed. Replay now decodes and validates the durable row before comparing
raw canonical bytes. A corrupted row therefore returns
`postgres_state.collaborative_approval_receipt.corrupt`, not an ordinary
conflict, and the regression test verifies the injected secret-like value is
absent from `Debug` and display output.

## 8. Privacy And Trust Assessment

No raw authority, provider, identity, token, reason, presentation, or workflow
payload was added. Persistence still accepts only the trusted in-memory receipt
and reads still return explicitly unverified, non-authorizing records. The
restore test uses stable references only and does not recreate producer trust.

## 9. Regression And Validation Assessment

Local validation passed:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `cargo test -p workflow-core --test postgres_state_backend --no-run`;
- the explicit required-configuration fail-closed probe;
- `npm run check:docs`; and
- `git diff --check`.

PR #593's Shared PostgreSQL State check passed the live receipt conformance and
backup/restore rehearsal. The remaining required checks were either green or
still running when this review text was prepared; merge remains conditional on
all seven required checks passing.

## 10. Documentation Assessment

The report, roadmap, and consumer plan now distinguish implementation from
acceptance and identify the required CI proof accurately. The original blocker
finding remains intact with a fix-forward note.

## 11. Blockers

None in the PostgreSQL receipt verification scope.

## 12. Non-Blocking Follow-Ups

- Add representative v3 authority records to the direct v3-to-v4 fixture if a
  future migration makes that fixture the primary retained-data proof.
- Keep trusted receipt construction private until the same-call consumer can
  supply decision-transaction proof.

## 13. Recommended Next Phase

Proceed to Phase 3 of the accepted consumer plan: one same-call collaborative
`ApprovalDecide` consumer with the reviewed advisory-lock and watermark-lock
order, outer-transaction database time, fresh authority resolution, atomic
decision/event/projection/receipt composition, and race evidence.

Do not broaden to another identity consumer, provider write, hosted identity
administration, OIDC/SCIM, dynamic groups, schema exposure, or release changes.

## 14. Governed Review Evidence

- Dogfood workflow: `dg/review`.
- Run: `run-1792656875316171000-2`.
- Approval: `approval/run-1792656875316171000-2/review-scope-approved`.
- Approval presentation: `presentation/132fa84bc87b1ad8`.
- Approval outcome: granted under delegated-maintainer authority.
- Phase status: `Completed`.
- Event summary: 39 events, one approval, zero retries, and zero escalations.
- Out-of-kernel work: code/diff inspection, CI evidence inspection,
  documentation, validation, git, and PR actions were performed by the
  delegated maintainer; Workflow OS governed review scope and recorded the
  phase trail.
