# Collaborative Approval Authority Receipt PostgreSQL Blocker Fix Report

## 1. Executive Summary

The PostgreSQL verification blocker identified by focused receipt review is
fixed in a narrow evidence-only phase. Required Shared PostgreSQL State CI now
executes receipt persistence behavior, restart recovery uses a reopened
backend, and the existing logical backup/restore rehearsal proves that a
receipt remains recoverable by its approval-decision bindings.

The trusted receipt model, approval semantics, and runtime consumer boundary
are unchanged. Phase 3 remains blocked pending focused review of this fix.

## 2. Blocker Fixed

The previous required CI path proved PostgreSQL v4 migration and table
creation but did not execute receipt write, exact replay, decision recovery,
conflict, or corruption behavior. It also did not preserve a receipt through
restart or backup/restore.

The required PostgreSQL recovery rehearsal now invokes the exact focused
receipt conformance test immediately before taking its logical backup. The
receipt written by that test is therefore included in the existing dump and
verified after restore.

## 3. Implementation Approach

- Added one exact receipt-conformance command to the recovery script already
  invoked by the required Shared PostgreSQL State job.
- Made missing `WORKFLOW_OS_TEST_POSTGRES_URL` fail when required PostgreSQL
  testing is enabled, while preserving local opt-in skip behavior.
- Reopened `PostgresStateBackend` after the initial write and exact replay,
  then performed receipt-ID and decision-binding recovery through the reopened
  backend.
- Extended the established restore-integrity test to locate the restored
  receipt bindings and recover the receipt through the public decision lookup.
- Decoded an existing receipt row before raw replay comparison so corrupted
  durable content reports the stable corruption error rather than an ordinary
  conflict.

## 4. Validation Boundary Summary

The production write boundary still accepts only the trusted receipt type.
Reads still return explicitly unverified persisted records. Exact byte replay
remains idempotent, conflicting bindings fail closed, and corrupted durable
content fails before replay can be accepted.

The test-only database lookup in the recovery rehearsal reads stable binding
columns solely to invoke the public recovery API. It does not restore producer
trust or authorize an action.

## 5. Privacy And Redaction Summary

No payload-bearing fields were added. Receipt tests continue to use references
and commitments only. The corruption scenario verifies that a secret-like
stored value is absent from both `Debug` and display errors, including the
corrupted replay path.

## 6. Test Coverage Summary

The required PostgreSQL lane now covers:

- first create and exact byte replay;
- restart recovery through a reopened backend;
- exact receipt and decision-binding lookup;
- binding conflict rejection;
- corrupted read and corrupted replay rejection without leakage;
- required-database configuration failure; and
- logical dump/restore preservation and decision-binding recovery.

## 7. Commands Run And Results

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed; local opt-in PostgreSQL tests preserved
  their documented skip behavior without configured database URLs.
- `cargo test -p workflow-core --test postgres_state_backend --no-run`: passed.
- the exact receipt test with `WORKFLOW_OS_REQUIRE_POSTGRES_TESTS=1` and no test
  URL: failed closed with `WORKFLOW_OS_TEST_POSTGRES_URL is required`, as
  expected.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

Required live PostgreSQL behavior and logical backup/restore are executed by
the Shared PostgreSQL State PR check; its result is recorded in the focused
blocker-fix review.

## 8. Remaining Known Limitations

- The receipt remains unsigned, local evidence only.
- No approval consumer accepts the receipt.
- No executor/resume integration is added.
- The direct v3-to-v4 fixture still contains metadata rather than
  representative v3 authority records; existing broader migration and
  backup/restore proofs cover retained data for this blocker.
- Local live PostgreSQL execution requires explicit test database URLs.

## 9. Recommended Next Phase

Perform a focused **collaborative approval authority receipt PostgreSQL blocker
fix review**. Only after acceptance should Phase 3 begin the same-call
collaborative `ApprovalDecide` consumer.

## 10. Governed Phase Evidence

- Dogfood workflow: `dg/blocker`.
- Run: `run-1792656330203334000-2`.
- Approval: `approval/run-1792656330203334000-2/fix-approved`.
- Approval presentation: `presentation/2264930b39bbe9e4`.
- Approval outcome: granted under delegated-maintainer authority.
- Phase status: `Completed`.
- Event summary: 39 events, one approval, zero retries, and zero escalations.
- Out-of-kernel work: repository edits and validation are performed by the
  delegated maintainer; Workflow OS governs scope and records the phase trail.
