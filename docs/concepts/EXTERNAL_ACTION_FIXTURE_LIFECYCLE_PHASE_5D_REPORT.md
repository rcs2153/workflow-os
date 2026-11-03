# External Action Fixture Lifecycle Phase 5D Report

Status: implemented and accepted after focused maintainer/security review and
blocker correction.

## Scope

This phase implements only exact unchanged-head squash merge, terminal replay,
and the accepted ambiguity-recovery matrix for fixture-backed create, ready,
and merge transitions. It remains Core-private, local, deterministic,
credential-free, and non-production.

## Implementation

### Exact unchanged-head squash merge

The fixture merge entrypoint loads the exact Core-owned successor binding and
revalidates the complete approval, policy, authority, execution-window,
resource, and evidence boundary. The provider then checks the same pull-request
number, reviewed-head commitment, ready posture, terminal-green required
checks, and expected provider revision under one lock before mutation.

Success records a payload-free exact merge result containing distinct merge-SHA
and merge-result commitments. The provider receipt binds the stable operation,
prior and resulting revisions, successor, reviewed head, and exact merge
result. A changed head, stale provider revision, missing successor, draft pull
request, or non-green checks rejects before merge.

### Terminal replay and terminal-state distinction

The durable lifecycle distinguishes successful terminal state from denied and
revoked terminal state. Only successful terminal state may resolve the exact
provider receipt. Exact replay returns the existing merge result and never
redispatches the provider mutation. Denied or revoked state cannot be
interpreted as a successful receipt.

### Typed ambiguity reconciliation

Provider lookup now returns a closed typed posture:

- `ExactFound` carries an operation-bound result and coverage proof;
- `AuthoritativeNotFound` carries exact negative-lookup coverage;
- `Inconclusive` preserves ambiguity with bounded coverage; and
- `Unavailable` preserves ambiguity without claiming a result.

An ambiguous transition remains `OutcomeAmbiguous` and cannot redispatch.
Exact-found reconciliation validates coverage and result binding before
recording success. Inconclusive and unavailable lookup leave the transition
ambiguous. A substituted operation result fails closed.

Focused review corrected authoritative absence so it may become known failure
only when the coverage binds the exact operation and the provider revision is
still the operation's expected pre-dispatch revision. Revision drift therefore
cannot turn an incomplete negative lookup into permission to retry. After a
proven absence, a lawful retry uses a distinct lifecycle attempt while keeping
the stable provider-operation identity.

Draft-create reconciliation still repairs the exact successor binding
idempotently. Ready and merge reconciliation retain the existing successor and
reject result substitution.

## Focused Tests

The Phase 5D tests prove:

1. one exact ready successor with terminal-green checks merges once;
2. terminal replay returns the same payload-free result without redispatch;
3. ambiguous create, ready, and merge reconcile from exact provider receipts
   without duplicate mutation;
4. authoritative not-found requires unchanged attempted-revision coverage;
5. proven absence permits a fresh attempt with stable provider-operation
   identity, while lifecycle tests prove distinct retry-attempt identity;
6. inconclusive and unavailable lookup preserve ambiguity and block
   redispatch;
7. substituted lookup results fail before provider redispatch; and
8. denied and revoked terminal states are not successful provider receipts.

The Phase 5A through 5C tests continue to cover complete source composition,
provider revision fencing, successor persistence and repair, evidence and
connector waits, exact ready replay, changed-head rejection, and current
authority revalidation.

## Validation

Passed after focused review correction:

- all 46 focused external-action tests;
- twenty consecutive repetitions of the corrected concurrent explicit
  TimeWindow reinvocation test;
- all 762 all-feature `workflow-core` library tests;
- strict all-target, all-feature `workflow-core` Clippy;
- `cargo fmt --all -- --check`;
- `npm run check:docs`; and
- `git diff --check`.

## Governed Phase Record

Implementation:

- dogfood workflow: `dg/implement`;
- run: `run-1793735631488090000-2`;
- approval: `approval/run-1793735631488090000-2/implementation-approved`;
- approval outcome: granted with persisted presentation proof; and
- event summary: 39 events, one approval, zero retries, zero escalations.

Focused review:

- dogfood workflow: `dg/review`;
- run: `run-1793737452665670000-2`;
- approval: `approval/run-1793737452665670000-2/review-scope-approved`;
- approval outcome: granted with persisted presentation proof; and
- event summary: 39 events, one approval, zero retries, zero escalations.

Repository edits, subagent implementation and review, shell validation, Git
commits, and the later PR lifecycle are performed by the delegated maintainer
outside the kernel. The runner coordinates governance and records the approved
scope; it does not silently execute these actions. No missing handler, check,
or report coverage was simulated.

## Limits

This phase does not provide one public end-to-end lifecycle entrypoint, live
GitHub, network access, credentials, branch deletion, automatic polling,
background scheduling, public APIs, schemas, SDKs, CLI mutation, broader
provider mutations, nested harness execution, Salsa, release, or publication.

Successful terminal replay still performs a bounded provider receipt lookup;
provider unavailability can therefore block replay even though lifecycle
success is durable. Phase 5E must document and test that boundary in the full
conformance path before any live connector plan.

The owner-conducted focused review is repository assurance, not independent
external security review.

See the [focused review](EXTERNAL_ACTION_FIXTURE_LIFECYCLE_PHASE_5D_REVIEW.md).
