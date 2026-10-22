# Collaborative ApprovalDecide Consumer Review

Status: needs blocker fixes

## 1. Executive Verdict

**Needs blocker fixes.**

The same-call PostgreSQL decision transaction has the right core shape: it
uses database time, reconstructs the complete current authority revision,
requires exactly one active exact-project `ApprovalDecide` grant, serializes
against revision activation, and atomically records the decision event,
approval projection, and payload-free receipt.

The phase cannot yet be accepted as an executor-integrated consumer. Focused
tests call only the backend transaction. No test invokes
`decide_collaborative_project_approval_with_presentation(...)`, so the claimed
rehydration, restart continuation, post-commit recovery, signal-failure
recovery, and absence of handler invocation after denied authority are not
executable evidence. The transaction also reads the exact run binding without
the row lock required by the accepted plan.

## 2. Scope Verification

The implementation stayed within the approved single-capability scope. It did
not add a hosted route, authentication protocol, identity administration,
second capability, provider mutation, workflow schema, CLI, SDK, example,
nested harness execution, reasoning lineage, or production identity claim.

## 3. Consumer Boundary Assessment

The public request fixes the capability in Core and accepts only trusted exact
project scope, authenticated principal, and the existing proof-enforced
approval request. It rejects actor disagreement before opening PostgreSQL and
does not accept provider, binding, principal, grant, revocation, trusted time,
or caller-authored source evidence.

The executor prepares presentation proof and resume inputs before persistence,
calls one atomic backend operation, rehydrates committed state, and continues
without deliberately appending the approval decision twice. This is the right
composition design, but it is currently verified only by inspection.

## 4. Transaction And Authority Assessment

The backend acquires the same organization-scoped advisory lock used by
authority activation and then runs a serializable transaction. It locks the
active watermark, obtains database time, validates the complete revision,
requires exactly one active matching grant, records the approval decision,
rechecks the watermark, and creates the receipt before commit.

The deterministic database-trigger test correctly proves one lawful serial
order between decision and revocation. It does not rely on timing-only sleeps
to place the revision change inside the authorization interval.

One accepted lock requirement is missed: the consumer calls
`require_project_resource_binding_tx(...)` rather than the existing
`require_locked_project_resource_binding_tx(...)`. Even though no deactivation
surface is currently exposed, the security-sensitive transaction should match
the reviewed lock contract rather than depend on that incidental limitation.

## 5. Replay And Recovery Assessment

The backend reconciles exact replay only when the durable approval projection,
decision event, and receipt agree. Concurrent duplicate calls produce one
write and one exact replay in live PostgreSQL conformance.

The executor recovery code requires one matching durable decision event and
one receipt, then continues from rehydrated state. However, no test calls this
path. A backend-only replay test cannot prove that executor restart avoids a
duplicate resume event or handler invocation, nor that recovery succeeds after
a post-commit runtime-signal failure.

## 6. Gate Preservation Assessment

Presentation proof, actor agreement, current exact-project authority, active
run binding, immutable run inputs used for resume preparation, and existing
continuation policy evaluation remain present. The implementation correctly
does not claim that every high-assurance, check, evidence, SideEffect, or
report gate is universally composed.

That honesty is necessary but does not replace runtime-boundary tests. Failed
authority must be shown to produce no decision, receipt, resume, invocation,
or side effect through the public consumer, not only through the backend
operation.

## 7. Privacy And Error Assessment

The private authorizing view is non-serializable. Public receipts contain
closed vocabulary and commitments rather than provider subjects, identity
assertions, presentation text, or decision reasons. Public Debug output
redacts the receipt and request context, and stable errors do not echo tested
secret-like values.

No privacy blocker was found in the reviewed model or persistence shape.

## 8. Test Quality Assessment

Strong coverage exists for backend success, exact replay, concurrent duplicate
decisions, deterministic revocation serialization, revoked denial, rollback,
receipt recovery, and actor-mismatch non-leakage.

Blocking omissions:

- no test invokes the public executor-facing collaborative decision function;
- no test proves successful grant continuation through that function;
- no test proves exact restart continuation without duplicate decision or
  resume events;
- no test proves recovery after commit but before normal continuation returns;
- no test proves denied or revoked authority reaches no skill handler through
  the executor path; and
- required live PostgreSQL CI therefore proves the store boundary, not the
  complete claimed consumer.

## 9. Validation Assessment

Local validation passed:

- `cargo check -p workflow-core`;
- focused actor-mismatch test;
- PostgreSQL conformance target compilation;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`; and
- `git diff --check`.

Live PostgreSQL execution was unavailable locally because
`WORKFLOW_OS_TEST_POSTGRES_URL` is not configured. Required Shared PostgreSQL
State CI must execute the corrected end-to-end proof.

## 10. Blockers

1. Replace the non-locking run-binding check with the accepted locking helper.
2. Add live PostgreSQL tests that invoke the public executor consumer and prove
   success, exact replay/restart, post-commit recovery, and failed-authority
   non-invocation without duplicate decision or resume events.
3. Require the Shared PostgreSQL State CI job to execute those tests before
   re-review acceptance.

## 11. Non-Blocking Follow-Ups

- Consider making post-commit runtime-signal delivery posture explicit after
  the recovery behavior is executable evidence.
- Measure advisory-lock contention only after the first consumer is accepted.
- Keep broader gate composition and another capability in separate phases.

## 12. Recommended Next Phase

Implement a **Collaborative `ApprovalDecide` executor-boundary blocker fix**.
The phase should change only the binding lock call and focused live PostgreSQL
tests. It must not add another capability or broaden hosted/runtime behavior.

## 13. Governed Review Evidence

- Dogfood workflow: `dg/review`.
- Run: `run-1792658534817671000-2`.
- Approval: `approval/run-1792658534817671000-2/review-scope-approved`.
- Approval presentation: `presentation/483447053e91ae1d`.
- Approval outcome: granted under delegated-maintainer authority with persisted
  presentation proof.
- Out-of-kernel work: repository inspection, review documentation, validation,
  git, and later PR operations are performed by the delegated maintainer;
  Workflow OS governs scope and records the review trail.

## 14. Fix-Forward Note

The narrow blocker fix now uses the locking exact run-binding helper and adds
live PostgreSQL tests through the public executor consumer. The tests cover
successful continuation, exact replay, recovery after the decision commits but
runtime signal delivery fails, and revoked-authority non-invocation. This note
does not erase the original blocker verdict. Acceptance requires a focused
re-review after the required Shared PostgreSQL State CI job executes the new
live path.

During focused re-review, restart recovery was also found to ignore the
request's project scope after the durable decision committed. The fix now
requires the scope/principal query commitment derived on replay to match the
trusted receipt before continuation, with a live PostgreSQL regression test.
