# External Action Fixture Lifecycle Phase 5A Report

Status: implemented and accepted after focused maintainer/security review
corrected deterministic replay.

## Scope

This phase implements only the source and fixture contracts accepted by the
[External Action Fixture Pull-Request Lifecycle Proof
Plan](../implementation-plans/external-action-fixture-pr-lifecycle-proof-plan.md).
It does not execute the pull-request lifecycle or enter a connector.

## Implementation

### Closed source coverage

The new Core-private fixture-lifecycle module defines ten required source
classes:

- immutable run;
- approval presentation;
- approval decision;
- current capability;
- policy and governance;
- execution window;
- revocation;
- provider resource;
- provider evidence; and
- transition lifecycle.

Coverage creation requires exactly one record for every closed class. Duplicate
or missing classes fail closed. Required composition rejects every non-
available posture, including missing, unsupported, denied, failed, stale, and
drifted sources. The canonical coverage commitment binds all source revisions
and postures.

### Source anchor

The source anchor binds the complete coverage commitment to one exact fixture
provider revision. Pre- and post-composition anchors must be equal. Provider or
source revision drift rejects before later execution composition.

This is a proof contract only. It does not yet collect from the production
approval, policy, continuity, capability, or provider sources.

### Revision-fenced provider contract

The private provider contract exposes bounded observation and atomic
compare-and-operate interfaces. The conformance fixture proves:

- an operation carries the exact observed provider revision;
- provider revision drift rejects without mutation;
- an exact operation replay returns the prior receipt;
- a different operation cannot reuse a stale revision; and
- two concurrent operations against one revision have exactly one successful
  mutation.

The fixture records payload-free operation receipts. It is not a GitHub
transport, provider emulator, or write-capable adapter.

### Core-owned successor binding

The private in-memory successor store validates every binding against the exact
immutable projection before persistence. The first exact write inserts, exact
replay returns without replacement, and conflicting PR identity for the same
projection fails closed. Reads revalidate the binding against the projection.

The store is deliberately create-only rather than replace-capable. Replacing a
successor identity would weaken the projection binding; conflict requires
reconciliation or a separately governed successor authorization.

The provider fixture does not own or authorize successor bindings.

### Shared test projection

A test-only projection builder now derives the accepted three-transition
authorization shape through the production constructors. It exists only under
`cfg(test)` and does not add a runtime API.

## Focused Tests

Five focused tests prove:

1. closed source coverage and non-available posture rejection;
2. stable source anchors and provider-revision drift rejection;
3. provider atomic compare-and-operate plus exact replay;
4. concurrent provider one-winner behavior; and
5. Core-owned successor insertion, replay, conflict rejection, and reread.

## Validation

Passed:

- focused `external_action_fixture_lifecycle` tests;
- `cargo test -p workflow-core --all-features --lib` with 743 tests;
- `cargo clippy -p workflow-core --all-targets --all-features -- -D warnings`;
- `cargo fmt --all`; and
- the planning-phase documentation and whitespace checks before implementation.

Final documentation and whitespace checks are rerun after this report.

Focused review found that an exact operation replay returned the provider's
latest global revision rather than the original operation result revision. The
fixture receipt now retains and returns its original resulting revision, binds
both pre- and post-operation revisions, and remains stable after unrelated
later provider operations.

## Limits

This phase does not implement:

- authoritative production-source collection;
- complete source composition around a transition;
- connector dispatch;
- draft creation, ready transition, or merge behavior;
- successor recovery after provider-success/local-write ambiguity;
- wait registration or resume;
- outcome reconciliation;
- live GitHub, network, or access-material behavior;
- production provider writes;
- public connector API, schema, SDK, or CLI mutation command;
- scheduler or automatic continuation;
- broader provider mutation; or
- Salsa.

## Next Review Question

Focused review accepts the corrected source coverage and provider compare-and-
operate contracts as sufficiently closed and non-authorizing for Phase 5B to
compose draft creation and successor binding. Phase 5B is the next authorized
increment, subject to a separately governed implementation phase.
