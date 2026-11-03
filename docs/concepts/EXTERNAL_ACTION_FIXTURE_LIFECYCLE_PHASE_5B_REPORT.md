# External Action Fixture Lifecycle Phase 5B Report

Status: implemented and accepted after focused maintainer/security review
corrected two blockers.

## Scope

This phase implements only fixture-backed draft pull-request creation and the
Core-owned exact successor binding accepted by the [External Action Fixture
Pull-Request Lifecycle Proof
Plan](../implementation-plans/external-action-fixture-pr-lifecycle-proof-plan.md).
It composes the accepted Phase 5A source and provider contracts with the
existing same-call current-authority and durable transition-lifecycle path.

## Implementation

### Deterministic fixture provider

The Phase 5A provider contract now has one Core-private in-memory conformance
implementation. It:

- exposes bounded provider observation;
- atomically compares the exact expected provider revision while applying one
  operation;
- assigns one deterministic fixture pull-request identity only to the
  create-draft transition;
- stores one payload-free receipt keyed by stable operation identity;
- returns the original result on exact replay; and
- supports bounded receipt lookup for reconciliation.

The fixture contains no network, access material, live GitHub behavior, or
production provider write.

### Same-call draft admission

The new crate-private draft composition entrypoint accepts only stable bindings
and source interfaces. One closed in-memory fixture source returns the bounded
approval, policy, window, revocation, resource, and evidence observation. Core
derives all ten Phase 5A coverage records from the observed facts, immutable
bindings, provider observation, and transition operation; callers cannot pass
a preassembled current-fact aggregate or window/revocation booleans to the
entrypoint. Core captures equal pre-admission source anchors and then enters
the already accepted connector-admission path, which freshly verifies:

- immutable run and execution binding;
- current exact capability authority;
- approval presentation and proof-backed decision;
- policy and governance commitments;
- execution window and revocation posture;
- exact repository resource binding; and
- transition-specific evidence freshness.

Only the borrowed same-call capability reaches the fixture adapter. The
adapter converts that capability into one revision-fenced provider operation.
Provider revision drift therefore rejects atomically even if state changes
after the second source observation.

### Exact successor persistence

Successful create-draft execution returns the fixture pull-request number and
observation time. Core derives `ExternalActionSuccessorBinding` from the exact
authorization projection, writes it through the Phase 5A create-only store,
and rereads the binding before reporting success.

An already persisted binding is not accepted merely because it validates
against the projection. Core also derives the expected binding from the exact
provider receipt and requires equality. This prevents a locally valid but
provider-conflicting successor identity from being paired with an unrelated
receipt.

### Local-persistence ambiguity repair

If provider creation succeeds but successor persistence fails, the entrypoint
returns a typed unresolved-persistence error. The durable transition lifecycle
has already recorded provider success, so re-entry cannot redispatch the
provider operation.

On re-entry, Core:

1. observes the existing terminal transition;
2. looks up the stable provider receipt by exact operation identity;
3. reconstructs the same successor binding;
4. writes it idempotently through the Core-owned store; and
5. reports `ReconciledSuccessor` only after exact reread.

An inconclusive receipt lookup remains blocked. Conflicting successor state
fails closed rather than being replaced.

## Focused Tests

Five new Phase 5B tests prove:

1. one create-draft operation produces one exact successor binding and exact
   replay does not re-enter the provider;
2. provider success followed by a failed local successor write repairs from
   the receipt without duplicate creation; and
3. provider success followed by a failed local successor read remains typed
   ambiguous and repairs without duplicate creation;
4. a locally valid successor that conflicts with the provider receipt is
   rejected; and
5. a substituted fixture-source resource fails before provider mutation.

The Phase 5A tests continue to prove closed source coverage, source-anchor
drift rejection, provider revision drift rejection, exact provider replay,
concurrent one-winner behavior, and successor-store conflict rejection.

## Validation

Passed after review corrections:

- five focused Phase 5B tests;
- all 748 all-feature `workflow-core` library tests;
- strict all-target, all-feature `workflow-core` Clippy; and
- formatting;
- documentation checks; and
- whitespace checks.

The owner-conducted focused review is repository assurance, not independent
external security review.

## Limits

This phase does not implement:

- mark-ready or merge transitions;
- evidence or connector-refusal waits;
- provider ambiguity injection or general outcome reconciliation;
- automatic retry, polling, scheduling, or continuation;
- production source collectors;
- live GitHub, network, or access-material behavior;
- external provider writes;
- public API, schema, SDK, or CLI mutation commands;
- broader mutation families, nested harnesses, or Salsa; or
- independent external security review.

## Review Corrections

Focused review found and corrected two blockers:

1. the first entrypoint accepted a caller-assembled fixture-facts aggregate,
   including window and revocation booleans, contrary to the accepted proof
   architecture; and
2. a local successor-store read failure after provider success escaped as a
   generic store error rather than the required typed persistence ambiguity.

The corrected entrypoint accepts only source interfaces and derives coverage
and admission facts in the same call. Both post-provider write and read
failures now preserve typed ambiguity, terminal lifecycle state blocks
redispatch, and receipt reconciliation repairs the exact binding.

The complete findings and acceptance reasoning are in [External Action Fixture
Lifecycle Phase 5B Review](EXTERNAL_ACTION_FIXTURE_LIFECYCLE_PHASE_5B_REVIEW.md).
