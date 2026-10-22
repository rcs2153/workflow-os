# Collaborative ApprovalDecide Consumer Blocker Fix Review

Status: required Shared PostgreSQL State CI pending

## 1. Executive Verdict

**Review in progress; required live PostgreSQL evidence is pending.**

The original run-binding lock and executor-boundary test blockers are fixed.
Focused re-review also identified and fixed a project-scope substitution gap in
restart recovery. The first required live CI run then exposed and the branch
fixed a time-varying receipt replay defect. Local validation is green. Final
acceptance is intentionally withheld until required Shared PostgreSQL State CI
reruns the complete live consumer path.

## 2. Scope Verification

The fix remains within the accepted single-consumer boundary. It adds no
hosted endpoint, second capability, provider mutation, identity administration,
schema, CLI, SDK, example, nested harness runtime, reasoning lineage, or
unrelated refactor.

## 3. Original Blocker Fix Assessment

The transaction now calls
`require_locked_project_resource_binding_tx(...)`. This matches the accepted
lock contract and prevents the exact active run binding from changing across
the decision transaction.

The live PostgreSQL milestone now invokes the public
`decide_collaborative_project_approval_with_presentation(...)` boundary. The
test proves success, exact replay, recovery after a post-commit audit-signal
failure, and revoked-authority non-invocation.

## 4. Additional Replay Binding Finding

Initial restart recovery validated actor, decision, presentation, event, and
receipt identity, but did not use the request's project scope. A caller could
substitute another project scope after the decision committed and reach
continuation under the old durable receipt.

The corrected recovery path derives the fixed `ApprovalDecide` authority query
from the supplied organization, project, and authenticated principal. Its
query commitment must equal the commitment in the trusted persisted receipt
before continuation. A substituted scope now fails with the stable
`executor.collaborative_approval.replay_scope_mismatch` code and invokes no
handler.

## 5. Replay And Recovery Assessment

The public path now distinguishes:

- first commit under current same-call authority;
- post-commit runtime-signal failure with no handler invocation;
- exact retry that recovers the receipt and continues once;
- terminal exact replay that returns without another handler invocation; and
- conflicting actor, decision, presentation, event, receipt, or project-scope
  replay that fails closed.

Exactly one `ApprovalGranted` and one `RunResumed` event are asserted for the
successful recovery path.

The low-level exact replay path also preserves the original database-time
authority observation instead of minting a new point-in-time receipt. It does
so only after re-deriving the same query and re-proving current authority. Any
query, authority-revision, source-snapshot, or active-grant commitment drift
continues to fail as a durable-state conflict.

## 6. Failed-Authority Assessment

After an authority revision containing an effective revocation is activated,
the public consumer returns the stable authority-denied error. The run remains
`WaitingForApproval`; no approval-granted, run-resumed, or skill-invocation
event is present; and the handler invocation count does not change.

## 7. Privacy And Error Assessment

The replay check compares payload-free commitments. It does not expose project
scope, actor identity, provider subjects, presentation text, reasons, paths,
tokens, or database payloads. The new error is stable and non-leaking. Receipt
claims remain evidence-only and non-authorizing.

## 8. Test Quality Assessment

The public-boundary test now covers every original blocker plus project-scope
substitution. It uses the actual PostgreSQL backend, durable approval
presentation, project binding, authority revisions, receipt lookup, and local
executor continuation. A deterministic failing audit sink establishes the
post-commit recovery point without timing guesses.

Local environments without `WORKFLOW_OS_TEST_POSTGRES_URL` compile but skip the
live path. The required CI job explicitly sets
`WORKFLOW_OS_REQUIRE_POSTGRES_TESTS=1` and invokes the exact shared-state
milestone containing this proof.

## 9. Validation Assessment

Local validation after the complete fix:

- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- focused actor-mismatch test: passed;
- PostgreSQL conformance target compilation: passed;
- `cargo test --workspace`: passed;
- `npm run check:docs`: passed;
- `git diff --check`: passed; and
- first live PostgreSQL consumer proof: failed on time-varying exact receipt
  replay and produced the bounded fix described above; and
- corrected live PostgreSQL consumer proof: pending required Shared PostgreSQL
  State CI rerun.

## 10. Remaining Blockers

1. Required Shared PostgreSQL State CI must execute the live public-consumer
   proof successfully on the review commit.

No additional code blocker is currently identified.

## 11. Recommended Completion

Push the bounded phase branch, require all repository checks, and update this
review with the exact Shared PostgreSQL State result. If the live job passes,
accept the same-call collaborative `ApprovalDecide` consumer and proceed to
the next roadmap item. If it fails, treat the failure as a blocker fix rather
than weakening the test.

## 12. Governed Review Evidence

- Dogfood workflow: `dg/review`.
- Run: `run-1792659103699832000-2`.
- Approval: `approval/run-1792659103699832000-2/review-scope-approved`.
- Approval presentation: `presentation/3a9de9219b4ab0d1`.
- Approval outcome: granted under delegated-maintainer authority with persisted
  presentation proof.
- Out-of-kernel work: repository inspection, code hardening, validation, git,
  CI, and PR operations are executed by the delegated maintainer; Workflow OS
  governs scope and records the review trail.
