# External Action Fixture Lifecycle Phase 5C Report

Status: implemented and accepted after focused maintainer/security review and
blocker correction.

## Scope

This phase implements only the evidence-wait, connector-refusal-wait,
explicit-resume, and exact-successor mark-ready increment accepted by the
[External Action Fixture Pull-Request Lifecycle Proof
Plan](../implementation-plans/external-action-fixture-pr-lifecycle-proof-plan.md).
It remains Core-private and credential-free.

## Implementation

### Complete projected lifecycle scope

The conformance fixture now binds create-draft, mark-ready, and later
squash-merge transitions into the original proof-backed projection before the
first connector entry. Phase 5C executes only create and mark-ready. The merge
transition remains projected but unimplemented, preventing later transition
discovery from silently widening the approved lifecycle.

### Exact provider observation

Fixture observations now carry revision-bound repository, base-target,
reviewed-head, required-check, pull-request-state, and connector-availability
commitments. Mark-ready compares the observed repository, base, and head with
the immutable projection before registering a wait or entering the connector.
Every fixture mutation still atomically compares the exact observed provider
revision.

### Evidence and connector waits

Pending and failed required checks create one durable `CheckRequired` wait for
the exact transition attempt. A proven pre-dispatch connector refusal creates
one durable `CapabilityUnavailable` wait. Both conditions bind the run,
projection operation, execution window, reserved attempt, successor,
observation commitment, and expiry without carrying a reusable capability or
provider payload.

Re-entry is the explicit local resume call. It reloads the projection, source
facts, exact successor, and provider observation. If the dependency is still
unsatisfied, the existing wait is returned without another lifecycle event or
connector entry. If satisfied, Core resolves the exact active wait and resumes
the same reserved attempt through fresh current-authority and static admission
validation.

Focused review strengthened this boundary with a typed resolution proof that
binds the wait cursor, expected event, dependency reference, wake trigger,
source observation, status, and resolution time. Rehydration recomputes that
proof before a resume attempt is exposed to generic admission.

### Exact successor mark-ready

Mark-ready requires the Core-owned successor binding created in Phase 5B. The
fixture provider independently checks the same pull-request number, reviewed
head, draft posture, terminal-green checks, and expected provider revision.
Success records one stable provider receipt. Exact replay returns that receipt
without another provider operation.

Focused review corrected receipt identity so compare-and-apply replay and
receipt lookup both bind the exact immutable provider operation, including the
successor number. A substituted successor can no longer reuse the prior ready
receipt.

## Focused Tests

The focused Phase 5C tests prove:

1. pending checks register a durable evidence wait, later terminal-green
   checks resolve it, and only the exact successor is marked ready;
2. failed checks remain durably waiting and never enter the ready connector;
3. proven pre-dispatch connector refusal registers a durable wait and resumes
   without a second approval when availability returns; and
4. changed-head and revoked-source paths fail after wait registration and
   before ready connector entry;
5. connector refusal discovered inside dispatch becomes a resumable wait;
6. required-check regression discovered inside dispatch becomes a resumable
   wait;
7. successor substitution fails during replay and receipt lookup; and
8. a mismatched wait-resolution proof fails before resume.

The Phase 5A and 5B tests continue to cover source drift, atomic provider
revision comparison, exact draft replay, concurrent admission, successor
conflict, and local successor-persistence repair.

## Validation

Passed before focused review:

- focused Phase 5C and lifecycle security tests;
- all 754 all-feature `workflow-core` library tests;
- strict all-target, all-feature `workflow-core` Clippy; and
- formatting;
- documentation checks; and
- diff-hygiene checks.

## Limits

This phase does not implement squash merge, branch deletion, live GitHub,
network access, access material, provider ambiguity broadening, automatic
polling or scheduling, background continuation, public APIs, schemas, SDKs,
CLI mutation commands, broader mutation families, nested harness execution,
Salsa, release, or publication.

The owner-conducted focused review is repository assurance, not independent
external security review.

See the [focused review](EXTERNAL_ACTION_FIXTURE_LIFECYCLE_PHASE_5C_REVIEW.md).
