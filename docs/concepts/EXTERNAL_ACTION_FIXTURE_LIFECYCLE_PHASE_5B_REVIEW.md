# External Action Fixture Lifecycle Phase 5B Review

Status: accepted after two blocker corrections.

## Scope

This owner-conducted focused maintainer/security review evaluates the private
Phase 5B fixture draft-creation composition, same-call authority admission,
provider revision fence, exact successor persistence, terminal replay, and
provider-success/local-persistence repair. It is repository assurance, not
independent external security review.

## Findings

### Blocker: entrypoint accepted a caller-assembled fact aggregate

The first implementation wrapped the accepted connector-admission helper but
still accepted `ExternalActionConnectorFixtureFacts` directly. That aggregate
contained caller-selected current resource and evidence plus boolean window
and revocation posture.

The lower layer validated those values, but the Phase 5 proof architecture
explicitly prohibits a caller-constructed aggregate of current facts. Keeping
that shape would have made the new end-to-end boundary appear more
authoritative than it was.

Correction:

- the Phase 5B entrypoint now accepts only stable bindings and source
  interfaces;
- one closed in-memory fixture source returns a typed, revision-fenced
  observation for facts without an accepted non-consuming production source;
- Core derives all ten coverage records from the immutable projection,
  execution binding, observed approval and policy facts, window and revocation
  posture, provider observation, evidence, and exact transition operation;
- Core derives the lower admission facts in the same call; and
- a focused test substitutes the fixture resource and proves rejection before
  provider mutation.

This remains fixture authority, not production source assurance. The source is
named and bounded rather than relabeled as a production store.

### Blocker: post-provider read failure lost ambiguity classification

Provider creation can succeed before either the initial successor read, the
successor write, or the verification reread completes. The first
implementation mapped write failure and missing reread to typed ambiguity, but
an unavailable store read propagated as a generic store error.

That weakened the required recovery contract and made the observable failure
depend on which local persistence operation failed.

Correction:

- unavailable successor reads after provider success map to
  `draft.successor_persistence_ambiguous`;
- security conflicts and validation errors retain their specific fail-closed
  classification;
- durable transition success continues to block provider redispatch;
- re-entry looks up the stable provider receipt and repairs the exact binding;
  and
- a focused read-failure test proves one provider operation and successful
  repair.

## Additional Review Conclusions

- Atomic provider revision comparison remains the mutation linearization
  boundary; equal pre-admission anchors do not replace it.
- Only create-draft results can contain a successor observation.
- Provider receipts bind operation identity, transition, prior and resulting
  revisions, and the created successor.
- Exact terminal replay does not re-enter the provider.
- A locally valid successor is compared with the provider-derived successor;
  projection validity alone is insufficient.
- Conflicting successor state is never replaced.
- Provider success with unresolved local persistence never becomes a known
  provider failure and cannot trigger blind create retry.
- Debug surfaces redact provider revisions and receipt commitments.

## Validation

Passed after corrections:

- five focused Phase 5B tests;
- all 748 all-feature `workflow-core` library tests;
- strict all-target, all-feature Clippy;
- formatting;
- documentation checks; and
- whitespace checks.

## Remaining Limits

No mark-ready, merge, evidence wait, connector-refusal wait, general provider
ambiguity injection, live provider, network, external write, access-material
path, public contract, scheduler, or automatic continuation exists in this
slice. The closed fixture source is not evidence that equivalent production
sources exist.

## Verdict

Accepted after correction. Phase 5C may implement only exact-head check
observation, evidence/refusal waits, explicit resume, and mark-ready for the
persisted successor. Merge and broader ambiguity work remain outside Phase 5C.
