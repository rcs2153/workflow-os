# Hosted Accepted Means Dispatched Implementation Review

## 1. Verdict

**Accepted and closed on exact main after eight in-scope source blockers and
one load-sensitive test-reliability blocker were fixed.**

The implementation now closes the reviewed source-level paths that could
acknowledge or replay hosted work without the same immutable dispatch context,
proof-backed approval decision, semantic invocation, and project scope. No
remaining maintainer-review blocker was found.

PR #739 passed all nine required checks on exact reviewed head `c2f3640d`, and
independent exact-main Actions run `42019866185` passed all nine jobs. Gate 1
is closed; this verdict does not broaden the hosted alpha product boundary.

## 2. Reviewed Boundary

The review covered:

- hosted create and exact-retry reconciliation;
- proof-backed approval and post-resume dispatch recovery;
- immutable run and dispatch-context binding;
- PostgreSQL transaction replay and concurrency;
- hosted API idempotency commitments;
- project-scoped replay;
- redaction and stable error behavior;
- adversarial tests; and
- roadmap, plan, product-contract, and implementation-report accuracy.

## 3. Blocking Findings Fixed

### 3.1 Dispatch context was not part of the immutable run

The first implementation validated workflow and immutable bundle inputs, but
server-owned catalog, policy, budget, and project scope were not included in
the resolved execution-context commitment. An approval-waiting or terminal
Core retry could therefore accept changed dispatch configuration.

Fix:

- derive a hosted dispatch-context commitment from the original resolved
  execution context plus exact catalog, policy identity/hash, budget, and
  optional project scope;
- store that commitment in the immutable run manifest; and
- recompute it before create retry and approval resume/replay.

Changed dispatch configuration now fails with the existing immutable-bundle
binding error before acknowledgement or dispatch.

### 3.2 Approval replay could not repair a post-resume dispatch failure

The approval path durably appended `ApprovalGranted` and `RunResumed` before
the atomic dispatch transaction. If queue insertion then failed, replaying the
same approval re-entered a path that accepted only `WaitingForApproval`.

Fix:

- detect an existing durable decision before new approval processing;
- validate the exact actor, decision, reason, correlation, presentation record,
  proof marker, and unique grant event;
- validate the immutable run and dispatch context;
- return an exact terminal decision replay when already terminal; and
- for a valid `Running` grant, repair or reconcile dispatch without appending a
  second approval grant.

The legacy hosted API already allows an exact idempotency replay. The
collaborative API now allows exact replay only for approval decisions; all
other scoped mutations retain their existing duplicate-rejection behavior.

### 3.3 PostgreSQL replay ignored supplied invocation semantics

The transaction replay path validated the durable work item and durable
invocation, but did not compare the invocation, step, skill, and skill-version
identity supplied by the replay projection.

Fix:

- compare the supplied semantic invocation tuple with the exact pending
  durable invocation;
- continue to ignore only regenerated event timestamps and attempt identity;
  and
- map substitution to the stable PostgreSQL replay-conflict result.

### 3.4 Project scope could be omitted on replay

An existing project-scoped work item could be replayed with `project_scope:
None`, bypassing the conditional binding check.

Fix:

- read the durable work-item scope binding under the transaction lock; and
- require exact `Some`/`Some` scope equality and active status, or exact
  unscoped `None`/`None` posture.

Adding, removing, or changing scope now fails closed.

### 3.5 Hosted work-item identity changed across exact retries

Exact-head PostgreSQL CI found that a successful repair could not acknowledge a
following exact retry. Hosted work-item identity included the plan's freshly
generated invocation ID, so rebuilding the same deterministic plan searched for
a different work item and then failed when it encountered the already durable
invocation.

Fix:

- derive hosted work-item identity from run, step, and the deterministic
  invocation idempotency key;
- derive the hosted invocation ID from the same exact dispatch intent so
  concurrent repair workers cannot propose conflicting semantic invocations;
- regenerate only lifecycle timestamps and execution-attempt identity; and
- continue validating the complete work-item and dispatch intent when the
  stable identity resolves an existing item.

Exact retries now reconcile the original durable item, while changed catalog,
policy, budget, scope, or request intent still fails closed through the semantic
intent validators.

### 3.6 Executor retry acknowledged a substituted durable invocation

Post-merge independent review found that the executor shortcut for an existing
work item validated the work-item intent and the existence of a self-consistent
pending invocation, but did not compare that durable invocation with the exact
deterministic invocation, step, skill, and skill version expected by the current
plan. The PostgreSQL transaction replay path already performed the stronger
comparison, so the executor could bypass that protection by returning before
the transaction call.

Fix:

- derive the deterministic invocation identity before the existing-item
  shortcut;
- compare the complete durable semantic invocation tuple with the current
  plan; and
- reject substitution with the existing payload-free hosted replay-intent
  diagnostic.

### 3.7 Terminal approval replay bypassed dispatch-context validation

Post-merge independent review also found that approval replay validated the
durable approval decision and presentation proof, then returned a denied or
terminal run before reconstructing the originally approved dispatch context.
Changed catalog, policy, budget, or scope could therefore be accepted on an
otherwise exact terminal replay.

Fix:

- reconstruct and validate the approval resume plan before every terminal or
  denial replay return;
- preserve terminal replay idempotency only when the dispatch context still
  matches the approved commitment; and
- add a terminal-denial regression that changes catalog identity and requires
  `executor.approval.resume_context_mismatch`.

### 3.8 First terminal-replay fix depended on the mutable checkout

Independent re-review confirmed the two original bypasses were closed, then
found that the first terminal-replay correction rebuilt the execution plan from
the current project checkout. Exact terminal replay could therefore fail after
the project was moved, deleted, or legitimately changed, even though all
decision-relevant run inputs were already durable.

Fix:

- reconstruct the pre-dispatch context from the durable immutable bundle's
  canonical workflow, skill, policy, checkpoint, and execution-posture records;
- apply the supplied dispatch context to that durable base commitment;
- compare the result with both the immutable manifest and the approval's
  original resolved-context commitment; and
- prove that exact denial replay still succeeds after removing the mutable
  project checkout, while changed dispatch context remains rejected.

## 4. Adversarial Coverage

The reviewed PostgreSQL suite now covers:

- queue insertion failure after durable run prefix;
- changed dispatch configuration before repair;
- exact create retry repair;
- replay after backend restart;
- concurrent repair converging on one queue outcome;
- changed work-item intent;
- regenerated lifecycle timestamp and attempt identity;
- changed invocation/skill projection;
- proof-backed approval dispatch failure after one durable grant and resume;
- exact approval replay repair without a second grant;
- repeated approval replay without duplicate queue state;
- changed approval dispatch context rejection;
- deterministic-plan rejection of a substituted durable semantic invocation;
  and
- terminal approval denial replay rejection after dispatch-context drift; and
- exact terminal denial replay after mutable project removal.

The full project-scope transaction match is enforced in code. Exact-head
PostgreSQL CI remains the authoritative execution of database-backed fault and
concurrency coverage because no local test database URL is configured.

## 5. Security And Product Assessment

- No new provider mutation or execution family was added.
- No caller can reconstruct dispatch authority from a work-item receipt.
- No raw provider payload, credential, path, or source content was added to
  durable state or errors.
- Request and dispatch conflicts return stable payload-free errors.
- Approval replay uses the prior proof-backed durable decision; it does not
  create automatic approval authority.
- The fix does not make the hosted alpha production-ready, multi-tenant, or a
  general scheduler.

## 6. Validation

Passed during review:

- focused Core hosted tests (`18 passed`);
- hosted crate tests (`38 passed`);
- complete Core all-features suite (`784` unit tests plus the full integration
  and documentation-test matrix; only explicitly opt-in live tests ignored);
- warnings-denied Clippy for Core and hosted;
- warnings-denied Rust documentation for Core and hosted;
- PostgreSQL integration-test compilation with all features; and
- formatting, repository documentation checks, and diff hygiene.

PostgreSQL execution remains delegated to exact-head CI, followed by the
required independent fault-injection rerun.

## 7. Recommendation

Commit the independently reviewed Gate 1 blocker patch, run a fresh governed
review and PR phase, and require exact-head PostgreSQL and hosted checks before
merge. Do not mark Accepted Means Dispatched fully closed until an independent
rerun of the final merged head also passes. Do not begin Gate 2 on a failing or
ambiguous Gate 1 head.

## 8. Governed Review Record

- workflow: `dg/review`
- run: `run-1793868587767291000-2`
- approval: `approval/run-1793868587767291000-2/review-scope-approved`
- presentation: `presentation/5c63fafc6cc31f3b`
- approval outcome: granted by delegated maintainer with persisted
  presentation proof
- phase status: completed
- event summary: `39` events, including `1` approval request, `1` approval
  grant, `8` policy decisions, `6` scheduled steps, `6` requested skill
  invocations, `6` started skill invocations, `6` successful skill
  invocations, `1` run resume, and `1` run completion; `0` retries and `0`
  escalations
- presentation proof: enforced as
  `presentation/5c63fafc6cc31f3b` with the approval event marker present
- approved boundary: Gate 1 implementation and documentation review only
- out-of-kernel execution: source inspection, edits, and validation were
  performed by the trusted development host; Workflow OS governed the phase
  but did not execute repository writes or shell commands

## 9. Governed Independent-Review Blocker Record

- workflow: `dg/blocker`
- run: `run-1793873603202986000-2`
- approval: `approval/run-1793873603202986000-2/fix-approved`
- presentation: `presentation/b8047fe8bae61f6c`
- approval outcome: granted by delegated maintainer with persisted presentation
  proof
- approved boundary: exact durable invocation-plan comparison and terminal
  approval-replay context validation only
- phase status: completed
- event summary: `39` events, including `1` approval request, `1` approval
  grant, `8` policy decisions, `6` scheduled steps, `6` requested skill
  invocations, `6` started skill invocations, `6` successful skill
  invocations, `1` run resume, and `1` run completion; `0` retries and `0`
  escalations
- presentation proof: enforced as `presentation/b8047fe8bae61f6c` with the
  approval event marker present
- validation summary: the complete Core all-features suite, focused hosted and
  durable-state tests, warnings-denied Core/hosted Clippy and rustdoc,
  formatting, documentation checks, PostgreSQL test-target compilation, and
  diff hygiene passed; the live PostgreSQL body remained skipped locally
  because no test database URL is configured
- out-of-kernel execution: source inspection, edits, and validation were
  performed by the trusted development host; Workflow OS governed the phase
  but did not execute repository writes or shell commands

## 10. Post-Merge CI Follow-Up

The merged review head passed pull-request CI, but exact-main CI exposed a
load-sensitive timing assumption in one operational scheduling test. The test
allowed only `500 ms` between deadline creation and persistence of the first
time-window yield. A loaded runner could lawfully cross that deadline, causing
the production security boundary to reject the stale wait.

The bounded follow-up changes test timing only: it provides a five-second
registration margin and waits for the exact remaining interval before wake
scheduling. Deadline eligibility and every behavioral assertion remain intact.
The complete Core all-features suite and repeated focused runs pass locally.

This follow-up passed its own exact-head and post-merge CI evidence. Gate 1 is
closed.

The fresh governed review found no blocker. The patch does not alter production
deadline validation or scheduling behavior, and it preserves the real elapsed
deadline, two scheduled wakes, exact executor-entry counts, bounded wake budget,
and terminal disposition assertions.

Governed review phase:

- workflow: `dg/review`
- run: `run-1793877689851739000-2`
- approval: `approval/run-1793877689851739000-2/review-scope-approved`
- presentation: `presentation/3994e760112b2cd6`
- approval outcome: granted by the delegated maintainer with persisted
  presentation proof
- phase status: completed
- event summary: `39` events, including `1` approval request, `1` approval
  grant, `8` policy decisions, `6` scheduled steps, `6` requested skill
  invocations, `6` started skill invocations, `6` successful skill
  invocations, `1` run resume, and `1` run completion; `0` retries and `0`
  escalations
- review verdict: accepted with no blocking finding; subsequent exact-head and
  exact-main evidence closed Gate 1
- out-of-kernel execution: commit inspection, source review, evidence review,
  and validation-result comparison were performed by the trusted development
  host; Workflow OS governed scope and approval but did not execute repository
  reads, shell commands, or PR actions

## 11. Final Gate 1 Acceptance Evidence

- PR: [#739](https://github.com/rcs2153/workflow-os/pull/739)
- exact reviewed head: `c2f3640d3035482d8451b4fc5c359dc070e76149`
- exact merged main: `61de8882f8b8dd2a2e2bbb81f5668ece581053b9`
- exact-head PR Actions run: `42018593801`, all nine jobs passed
- independent exact-main Actions run: `42019866185`, all nine jobs passed
- next authorized implementation lane: Gate 2, **Verified Means Correctly
  Bound**

## 12. Governed Gate Closeout Review Record

- workflow: `dg/review`
- run: `run-1793879129394606000-2`
- approval: `approval/run-1793879129394606000-2/review-scope-approved`
- presentation: `presentation/9735bd59dec78b21`
- approval outcome: granted by the delegated maintainer with persisted
  presentation proof
- phase status: completed
- event summary: `39` events, including `1` approval request, `1` approval
  grant, `8` policy decisions, `6` scheduled steps, `6` requested skill
  invocations, `6` started skill invocations, `6` successful skill
  invocations, `1` run resume, and `1` run completion; `0` retries and `0`
  escalations
- review verdict: no blocking finding; the closeout claims match the exact PR
  and exact-main Actions evidence and preserve the bounded hosted-alpha posture
- validation summary: repository documentation checks, diff hygiene, commit
  inspection, local conflict inspection, and public GitHub Actions evidence
  comparison passed
- out-of-kernel execution: commit inspection, GitHub Actions API reads,
  documentation validation, git conflict inspection, repository edits, and
  subsequent PR actions were or will be performed by the trusted development
  host; Workflow OS governed scope, approval, and phase closure but did not
  execute repository reads, network calls, shell commands, writes, git
  operations, or PR actions
