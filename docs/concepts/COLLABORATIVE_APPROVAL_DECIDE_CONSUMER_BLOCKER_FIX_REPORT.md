# Collaborative ApprovalDecide Consumer Blocker Fix Report

## 1. Executive Summary

The focused review blockers for the same-call collaborative `ApprovalDecide`
consumer are fixed. The PostgreSQL decision transaction now locks the exact
run binding, and required live PostgreSQL conformance directly invokes the
public executor consumer instead of proving only its backend transaction.

## 2. Blockers Fixed

- Replaced the non-locking run-binding check with the existing transaction-
  scoped locking helper.
- Added direct public consumer coverage for a successful grant and
  continuation.
- Added exact restart and terminal replay coverage without duplicate decision,
  resume, or handler invocation.
- Added recovery coverage for a decision that committed before audit-signal
  delivery failed.
- Added revoked-authority coverage proving no decision, resume, or handler
  invocation occurs.
- Bound restart recovery to the exact project-scope/principal authority query
  commitment recorded by the durable receipt.

## 3. Implementation Approach

The production transaction change is one helper substitution:
`require_locked_project_resource_binding_tx(...)` now protects the exact run
binding inside the existing serializable approval decision transaction.

The public collaborative decision function and its private continuation
helpers are generic over the existing audit, observability, and structured-log
sinks. The default call shape is unchanged. This permits a deterministic
failing audit sink in conformance tests so the test can prove recovery after
the durable decision commits but before ordinary continuation returns.

Focused re-review found that the initial recovery path ignored the supplied
project scope after a decision had committed. Recovery now derives the fixed
`ApprovalDecide` query from the supplied scope and authenticated principal and
requires its commitment to match the trusted persisted receipt before any
continuation.

## 4. Executor-Boundary Proof

The live PostgreSQL test creates a real approval-gated local run, persists its
exact project binding and presentation proof, and calls
`decide_collaborative_project_approval_with_presentation(...)`.

It first injects a post-commit audit failure and proves the local skill handler
was not invoked. A substituted project-scope retry then fails before
continuation and still invokes no handler. A fresh exact executor call then
recovers the already-committed
decision, resumes once, and completes once. Exact terminal replay invokes no
additional handler and leaves exactly one `ApprovalGranted` and one
`RunResumed` event.

The same test then activates a revoked authority revision and proves the public
consumer fails before continuation. The run remains waiting for approval and
contains no grant, resume, or skill-invocation event.

## 5. Scope Explicitly Not Changed

- No hosted approval endpoint or authentication surface.
- No second collaborative capability.
- No provider mutation or write-family broadening.
- No workflow schema, CLI, SDK, or example changes.
- No identity administration, nested harness runtime, or reasoning lineage.

## 6. Privacy And Error Posture

The fix adds no payload-bearing authority state. Existing stable non-leaking
errors and redacted Debug behavior remain in force. The public test checks
stable error codes and event state rather than copying identity, presentation,
provider, or skill payloads.

## 7. Test Coverage

- actor mismatch rejection before a PostgreSQL connection;
- public consumer success and continuation;
- post-commit signal failure followed by restart recovery;
- project-scope substitution rejection before restart continuation;
- exact terminal replay without duplicate handler execution;
- exactly one approval-granted and run-resumed event;
- revoked-authority non-invocation and unchanged waiting run; and
- existing workspace, receipt, revocation-race, migration, and recovery tests.

## 8. Validation Commands And Results

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test -p workflow-core --test postgres_state_backend collaborative_approval_rejects_actor_mismatch_before_connection`: passed.
- `cargo test -p workflow-core --test postgres_state_backend --no-run`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.
- Required Shared PostgreSQL State CI: passed the complete live public-consumer
  proof and corrected backup/restore rehearsal.
- All other required repository checks: passed.

## 9. Remaining Known Limitations

- The public consumer remains an explicit local/Core API, not a hosted route.
- The new end-to-end test requires the repository's live PostgreSQL CI fixture.
- Post-commit signal failure is recoverable on exact retry; this phase does not
  add a background signal-delivery service.
- Broader gate composition and additional collaborative capabilities remain
  separately reviewed work.

The first two required Shared PostgreSQL State runs exposed one exact-replay
defect in two layers: the backend recomputed the receipt with a fresh database
observation time and a fresh time-bound source-snapshot commitment, so the
deterministic decision binding collided with a different receipt identity. The
bounded fix reconstructs the historical point-in-time receipt only after the
same authority query, authority revision, revision commitment, active-grant
commitment, and decision kind are re-proven against current state. Query or
authority drift still fails closed. Required CI reran and passed the live proof.

The next required run passed the complete live consumer milestone and exposed
a stale recovery-rehearsal expectation: the new consumer proof advances the
backed-up collaborative authority watermark to revision 12, while the restore
test still expected the historical revision 8. The rehearsal now requires
exact replay of restored revision 12 and rejects revision 11 as rollback.

## 10. Recommended Next Phase

Proceed to the **next runnable no-write trusted-host profile**. Keep the
dedicated draft-pull-request live sandbox proof dependency-tracked until its
least-privilege credential is available, and do not broaden collaborative
capabilities or provider mutations through this phase.

## 11. Governed Fix Evidence

- Dogfood workflow: `dg/blocker`.
- Run: `run-1792658687978251000-2`.
- Approval: `approval/run-1792658687978251000-2/fix-approved`.
- Approval presentation: `presentation/61d2b860c66fd5a1`.
- Approval outcome: granted under delegated-maintainer authority with persisted
  presentation proof.
- Out-of-kernel work: code edits, tests, documentation, git, and later PR
  operations are executed by the delegated maintainer; Workflow OS governs
  scope and records the phase trail.
