# External Action Fixture Lifecycle Phase 5A Review

Status: accepted after blocker correction.

## Scope

This owner-conducted focused maintainer/security review evaluates the dark
Core-private Phase 5A source coverage, source anchor, revision-fenced provider,
and successor-binding contracts. It is repository assurance, not independent
external security review.

## Finding

### Blocker: exact replay returned mutable global revision

The first conformance fixture stored only the provider receipt commitment for
an operation. Exact replay returned that receipt with the provider's current
global revision. After another operation advanced provider state, replay of the
first operation could therefore return a different result revision.

That violated deterministic replay and could make a later caller bind an old
operation receipt to unrelated newer provider state.

Correction:

- each operation receipt now retains its exact resulting provider revision;
- the resulting revision commits to the prior revision, operation identity,
  transition kind, and revision sequence;
- the receipt commits to both prior and resulting revisions;
- exact replay returns the original receipt and original resulting revision;
  and
- a focused test advances provider state with another operation and proves the
  first replay remains byte-equivalent.

## Additional Review Conclusions

- Closed coverage correctly requires exactly one record for all ten required
  source classes.
- Non-available required postures fail closed.
- Source-anchor equality binds coverage to provider revision but remains non-
  authorizing.
- Atomic provider revision comparison closes concurrent stale-read mutation;
  the test proves exactly one of two different operations can mutate one
  revision.
- Operation replay is checked before revision drift only for the exact stable
  operation identity and transition kind.
- The successor store is Core-owned, validates against the immutable
  projection, accepts exact replay, and rejects conflicting identity.
- Create-only exact persistence is safer than replacement and better matches
  later reconciliation requirements.
- Debug output for provider application results redacts provider revision and
  receipt commitments.

## Validation

Passed after correction:

- focused `external_action_fixture_lifecycle` tests;
- all 743 all-feature `workflow-core` library tests;
- strict all-target, all-feature Clippy;
- formatting;
- documentation checks; and
- whitespace checks.

## Remaining Limits

No complete lifecycle, connector dispatch, live provider, network, external
write, access-material path, public contract, scheduler, or automatic
continuation exists in this slice. Phase 5A contracts do not themselves prove
that production sources can satisfy the coverage model.

## Verdict

Accepted after correction. Phase 5B may compose fixture draft creation and
Core-owned successor binding through these contracts. Mark-ready, merge, waits,
and ambiguity recovery beyond create/persistence remain outside Phase 5B.
