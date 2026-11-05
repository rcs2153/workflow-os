# Hosted Accepted Means Dispatched Implementation Review

## 1. Verdict

**Accepted for pull-request validation after five in-scope blockers were fixed.**

The implementation now closes the known source-level paths that could
acknowledge or replay hosted work without the same immutable dispatch context,
proof-backed approval decision, semantic invocation, and project scope. No
remaining maintainer-review blocker was found.

Gate 1 is not fully closed by this review. Exact-head PostgreSQL CI and an
independent fault-injection rerun remain mandatory acceptance evidence.

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
- retain the generated invocation ID only as the invocation-event identity; and
- continue validating the complete work-item and dispatch intent when the
  stable identity resolves an existing item.

Exact retries now reconcile the original durable item, while changed catalog,
policy, budget, scope, or request intent still fails closed through the semantic
intent validators.

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
- repeated approval replay without duplicate queue state; and
- changed approval dispatch context rejection.

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

- focused Core hosted tests (`17 passed`);
- hosted crate tests (`38 passed`);
- complete Core all-features suite (`783` unit tests plus the full integration
  and documentation-test matrix; only explicitly opt-in live tests ignored);
- warnings-denied Clippy for Core and hosted;
- warnings-denied Rust documentation for Core and hosted;
- PostgreSQL integration-test compilation with all features; and
- formatting, repository documentation checks, and diff hygiene.

PostgreSQL execution remains delegated to exact-head CI, followed by the
required independent fault-injection rerun.

## 7. Recommendation

Commit the reviewed Gate 1 patch, run the governed PR phase, and require the
exact-head PostgreSQL and hosted checks before merge. Do not mark Accepted Means
Dispatched fully closed until the independent rerun also passes. Do not begin
Gate 2 on a failing or ambiguous Gate 1 head.

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
