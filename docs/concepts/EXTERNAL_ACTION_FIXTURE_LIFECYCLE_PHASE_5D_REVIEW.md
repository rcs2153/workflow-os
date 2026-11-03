# External Action Fixture Lifecycle Phase 5D Review

Status: accepted after focused maintainer/security review and blocker
correction.

## Decision

Accept Phase 5D. The exact unchanged-head merge, terminal replay, and typed
ambiguity-recovery slice is sufficiently bounded for the local Core-private
conformance fixture. Phase 5E may implement only the accepted end-to-end
conformance and review scope. This decision does not authorize live GitHub,
credentials, network access, branch deletion, a scheduler, public connector
APIs, broader mutation families, nested harnesses, release, or publication.

This is owner-conducted repository assurance, not independent external
security review.

## Findings And Corrections

### 1. Negative lookup coverage could reopen provider mutation

The initial implementation allowed `AuthoritativeNotFound` to reconcile an
ambiguous attempt to known failure. Known failure is lawfully retryable, but
the initial coverage model proved only the operation commitment and an
observed provider revision. It did not require that the observed revision
still matched the exact attempted pre-dispatch revision. A weak or incomplete
negative lookup could therefore have reopened provider mutation.

Corrected:

- authoritative absence validates the exact operation-bound coverage;
- the coverage revision must equal the operation's expected pre-dispatch
  provider revision;
- revision-drifted negative coverage fails closed;
- inconclusive and unavailable lookup continue to preserve ambiguity;
- focused tests execute the post-reconciliation call and prove that retry is
  possible only after accepted authoritative absence; and
- the retry retains stable provider-operation identity while the lifecycle
  allocates a distinct attempt identity.

### 2. Strict Clippy rejected a test-only panic

The first strict all-target Clippy run rejected a direct `panic!` in the exact
merge receipt-lookup test. Production behavior was unaffected, but repository
quality policy applies to test targets as well.

Corrected:

- the test now asserts the expected typed lookup variant before extracting
  the result without a direct panic; and
- strict all-target, all-feature Clippy passes.

### 3. Full-suite concurrency validation exposed a stale wake-status assumption

The first full-suite rerun preserved the runtime's at-most-once guarantees but
failed an existing concurrent explicit TimeWindow reinvocation assertion. One
caller exact-replayed the committed wake transition and then won the separate
one-shot executor directive, completing terminal with one executor entry. The
test incorrectly assumed that `ExactReplay` wake posture always implied zero
executor entries.

Corrected:

- wake transition replay and executor-directive consumption are asserted as
  separate atomic boundaries;
- the sole caller with one executor entry must be terminal and cannot have a
  security-rejected wake;
- a zero-entry successful caller may be blocked or terminal and cannot have a
  security-rejected wake;
- the accepted bounded loser errors remain explicit; and
- aggregate one-entry and final durable-terminal assertions remain mandatory.

## Accepted Boundaries

- Merge requires the exact persisted successor and unchanged reviewed head.
- Ready posture and terminal-green checks are revalidated at the provider
  compare-and-operate boundary.
- Provider revision drift fails before mutation.
- Exact merge results and receipts remain payload-free and operation-bound.
- Exact terminal replay never redispatches the mutation.
- Denied and revoked terminal states cannot become successful receipts.
- Ambiguity blocks redispatch until typed reconciliation.
- Exact found, authoritative not-found, inconclusive, and unavailable lookup
  have distinct fail-closed behavior.
- The fixture remains local, deterministic, credential-free, and
  non-production.

## Validation

Passed after correction:

- all 46 focused external-action tests;
- twenty consecutive repetitions of the corrected concurrent explicit
  TimeWindow reinvocation test;
- all 762 all-feature `workflow-core` library tests;
- strict all-target, all-feature `workflow-core` Clippy;
- `cargo fmt --all -- --check`;
- `npm run check:docs`; and
- `git diff --check`.

## Residual Risks

- Successful terminal replay still depends on a bounded provider receipt
  lookup, so provider unavailability can block replay despite durable lifecycle
  success.
- Provider mutation atomicity and lifecycle cursor fencing are tested, but a
  full create-ready-merge connector-admission concurrency proof remains part of
  Phase 5E conformance.
- Provider operation identity intentionally excludes expected provider
  revision so logical idempotency survives later provider revision movement;
  the expected revision remains a compare-and-operate precondition and an
  authoritative-negative-coverage boundary. This distinction must stay
  explicit before live adapters.

## Next Authorized Increment

Phase 5E may implement one complete create-draft to mark-ready to squash-merge
fixture proof under one complete authorization projection, inspect durable
lifecycle and successor state, and close the conformance review. Live connector
work remains unauthorized until Phase 5E is implemented and focused-review
accepted.
