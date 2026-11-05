# Hosted Accepted Means Dispatched Implementation Report

## 1. Executive Summary

The first continuity-of-authority release gate is implemented for the
single-tenant hosted alpha path. A hosted create or exact retry can no longer
acknowledge a nonempty event stream as successful merely because the run
exists. The runtime now validates the immutable run request and derives success
from one bounded durable posture:

- the exact run is waiting for approval;
- the exact run is terminal; or
- the exact `Running` run has one matching queued work item and matching
  invocation-requested and invocation-started evidence.

When a prior attempt persisted the run prefix but failed before queue insertion,
an exact retry reconstructs the reviewed no-write dispatch from the immutable
run bundle and atomically repairs it. Changed requests, changed dispatch intent,
incomplete durable dispatch, and unsupported run postures fail closed.

This closes the implementation portion of **Accepted Means Dispatched**. It
does not close the release gate until exact-head PostgreSQL CI and an independent
fault-injection rerun pass.

## 2. Scope Completed

- Replaced the hosted retry shortcut based on a nonempty event stream with
  immutable-request and durable-posture reconciliation.
- Revalidated the stored immutable run bundle against the exact retry request
  and prepared execution plan before acknowledging an existing run.
- Bound catalog, policy identity/hash, budget, and optional project scope into
  the immutable resolved execution context used by create, approval resume,
  and every replay.
- Reused an exact durable invoke-policy decision during repair rather than
  appending a duplicate policy event.
- Required existing work items to match every immutable dispatch-intent field.
- Allowed only lifecycle fields such as queue timestamps, status timestamps,
  attempt count, and generated attempt identity to differ across exact retries.
- Required an existing work item to match a rehydrated `Running` run and one
  exact pending hosted invocation, including the deterministic invocation,
  step, skill, and skill-version tuple expected by the current plan.
- Made proof-backed approval decisions exactly replayable after a durable grant
  and resume but failed queue insertion, without appending another grant.
- Required terminal granted or denied approval replay to reconstruct and
  validate the originally approved dispatch context from durable immutable
  bundle records before returning the durable terminal run.
- Changed PostgreSQL dispatch replay to validate semantic intent plus durable
  run evidence instead of comparing regenerated lifecycle event objects.
- Required the supplied replay projection to preserve invocation, step, skill,
  and skill-version identity and required project scope presence to match the
  durable work-item binding.
- Bound both legacy and collaborative hosted create idempotency to the complete
  serialized request intent rather than run identity alone.
- Added PostgreSQL fault, restart, changed-intent, replay, and concurrent-repair
  coverage.

## 3. Closed Retry Postures

The current no-write hosted entry point recognizes these success postures:

1. `WaitingForApproval`: the exact immutable run exists and is still waiting;
2. terminal: the exact immutable run is already durably complete or failed; or
3. `Running`: the exact durable work item and pending invocation evidence exist,
   or an exact retry atomically creates them.

Any other existing run posture returns
`executor.hosted_no_write.retry_posture.invalid`. A changed immutable request
returns the existing immutable-bundle binding error. A changed work-item intent
returns `hosted.dispatch.intent_mismatch`. PostgreSQL transaction-local replay
maps inconsistent durable state to
`postgres_state.hosted_dispatch.replay_conflict`.

The implementation does not yet introduce a separate public
`recovery_blocked` API disposition. A repairable pre-dispatch prefix is repaired
inside the exact retry. Unrepairable or ambiguous state fails closed and is not
reported as accepted.

## 4. Fault And Concurrency Proof

The PostgreSQL integration test installs a transaction-local database trigger
that rejects only `hosted_work_item` insertion. The first submission therefore
persists the run prefix and invoke-policy decision but fails before durable
dispatch. The test then proves:

- the stranded snapshot is `Running`;
- no queue record was added;
- removal of the fault followed by an exact retry adds exactly one queue item;
- another exact retry adds no queue item;
- a fresh backend instance reconciles the same dispatch after restart;
- changed catalog intent is rejected; and
- two concurrent exact repair attempts both converge on one durable queue
  outcome through serializable transaction retry.

Separate PostgreSQL replay coverage proves that regenerated queue timestamps and
attempt identities do not create false conflicts, while changed catalog intent
does.

The approval-path fault test applies the same queue-insert failure after one
proof-backed grant and `RunResumed`. Exact approval replay repairs the dispatch,
preserves one grant event, creates one queue item, remains idempotent on another
retry, and rejects changed dispatch configuration. Terminal denial replay also
rejects changed dispatch configuration instead of treating terminal status as
authority to bypass the approved context. Exact terminal replay remains
available after the mutable project checkout is removed because replay derives
its comparison from the stored canonical workflow, skill, policy, checkpoint,
and execution-posture records.

Post-merge independent source review found that the executor's existing-item
shortcut validated only that *some* durable invocation was pending, while the
PostgreSQL transaction path validated the exact supplied semantic tuple. The
shortcut now performs the same exact invocation, step, skill, and skill-version
comparison against the deterministic plan before acknowledging the retry.

## 5. Hosted API Idempotency

Hosted run-create idempotency now commits the complete request shape, including
workflow, immutable bundle, correlation, sensitivity, redaction, and request
identity. Collaborative project run creation includes the same request
commitment in its durable result reference.

An idempotency key can therefore replay only the exact request. Reusing it with
a changed immutable bundle version or other request field conflicts before the
runtime can acknowledge the changed work.

## 6. Validation

Passed locally:

- `cargo test -p workflow-core --all-features`;
- `cargo test -p workflow-hosted --all-features`;
- focused hosted Core tests (`17 passed`);
- hosted library tests (`38 passed`);
- `cargo clippy -p workflow-core -p workflow-hosted --all-targets --all-features -- -D warnings`;
- warnings-denied Rust documentation for Core and hosted;
- PostgreSQL integration-test compilation with all features;
- `cargo fmt --all --check`; and
- `git diff --check`.

The local environment does not provide `WORKFLOW_OS_TEST_POSTGRES_URL`, so the
live PostgreSQL fault and concurrency body was compiled but skipped locally.
Exact-head pull-request CI must execute that test against PostgreSQL before
merge. An independent rerun must reproduce the parent failure and pass against
the merged fix before the complete Gate 1 acceptance claim is closed.

## 7. Explicit Non-Goals

This phase does not add:

- another provider mutation family;
- provider writes or broader OpenShell execution;
- hosted multi-tenancy or production distributed scheduling;
- automatic approval or weakened policy/evidence gates;
- WorkReport identity changes, which belong to Gate 2;
- provider-resource authorization reassessment, which belongs to Gate 3;
- a public recovery operator API; or
- a v1, reliable-hosted, or unattended-execution readiness claim.

## 8. Residual Limits And Next Decision

Hosted alpha remains no-write and limited. PostgreSQL CI and independent fault
injection are still release-gate evidence, not optional follow-ups. The broader
continuity-of-authority preemption remains active until Gates 1 through 3 and
their cross-gate review close.

After focused maintainer/security review, exact-head CI, and independent Gate 1
rerun, the next implementation phase is **Verified Means Correctly Bound**:
exact WorkReport storage-key, run, report, workflow, and immutable execution
identity validation across every backend and replay consumer.

## 9. Governed Phase Record

- workflow: `dg/implement`
- run: `run-1793867386408656000-2`
- approval: `approval/run-1793867386408656000-2/implementation-approved`
- presentation: `presentation/f0461d65737f6c67`
- approval outcome: granted by the delegated maintainer with persisted
  presentation proof
- phase status: completed
- event summary: 39 events, including one approval request, one approval grant,
  eight policy decisions, six scheduled steps, and six successful skill
  invocations; no retries or escalations
- validation summary: Core and hosted tests, focused hosted tests,
  warnings-denied Clippy and rustdoc, formatting, docs checks, PostgreSQL test
  compilation, and diff hygiene passed; the live PostgreSQL body was skipped
  locally because no test database URL is configured
- approved boundary: hosted exact-retry reconciliation and complete create
  idempotency binding only
- out-of-kernel execution: the trusted local development host edited files and
  ran validation; Workflow OS governed scope and approval but did not execute
  repository writes or shell commands

The focused and post-merge independent reviews and their seven corrected
blockers are recorded in [Hosted
Accepted Means Dispatched Implementation
Review](HOSTED_ACCEPTED_MEANS_DISPATCHED_IMPLEMENTATION_REVIEW.md).

## 10. Governed Independent-Review Blocker Record

- workflow: `dg/blocker`
- run: `run-1793873603202986000-2`
- approval: `approval/run-1793873603202986000-2/fix-approved`
- presentation: `presentation/b8047fe8bae61f6c`
- approval outcome: granted by the delegated maintainer with persisted
  presentation proof
- phase status: completed
- event summary: `39` events, including `1` approval request, `1` approval
  grant, `8` policy decisions, `6` scheduled steps, `6` requested skill
  invocations, `6` started skill invocations, `6` successful skill
  invocations, `1` run resume, and `1` run completion; `0` retries and `0`
  escalations
- presentation proof: enforced as `presentation/b8047fe8bae61f6c` with the
  approval event marker present
- approved boundary: exact durable invocation-plan comparison and terminal
  approval-replay context validation only
- non-goals: Gate 2, provider mutation, scheduling expansion, schema changes,
  release work, and unrelated refactoring
- validation summary: the complete Core all-features suite, focused hosted and
  durable-state tests, warnings-denied Core/hosted Clippy and rustdoc,
  formatting, documentation checks, PostgreSQL test-target compilation, and
  diff hygiene passed; the live PostgreSQL body remained skipped locally
  because no test database URL is configured
- out-of-kernel execution: source inspection, edits, and validation were
  performed by the trusted development host; Workflow OS governed the phase
  but did not execute repository writes or shell commands
