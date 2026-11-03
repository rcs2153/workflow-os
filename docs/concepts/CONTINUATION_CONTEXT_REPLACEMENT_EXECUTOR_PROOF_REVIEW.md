# Continuation Context Replacement-Executor Proof Review

## Decision

Blocked pending one bounded P0 correction phase. The private proof establishes
exact aggregate and projection equality, typed gap delivery, superseded-intent
exclusion, distinct recipient identity, and a non-authorizing orientation
boundary. It does not yet establish safe replacement-executor continuation.

This is an owner-conducted focused maintainer/security review, supplemented by
a second repository-local agent review. It is not independent external
security assurance.

The bounded blockers recorded here were subsequently corrected and accepted in
the [hardening re-review](CONTINUATION_CONTEXT_REPLACEMENT_EXECUTOR_PROOF_HARDENING_REVIEW.md).
This document remains the historical first-review record.

## Governed Review

- workflow: `dg/review`
- run: `run-1793719145994628000-2`
- approval: `approval/run-1793719145994628000-2/review-scope-approved`
- presentation: `presentation/e3c348df5c40451c`
- approval outcome: granted by the delegated maintainer for the bounded review
  scope

## Findings

### Blocking: control intent loses its disposition

Current `pause`, `stop`, and `resume` accepted intents all project as
`LawfulNextOption`. A complete aggregate containing a current pause or stop can
therefore reach the proof's fresh-authority callback as if it were resumable.
The correction must preserve a typed current control disposition through the
private aggregate, delivery, and replacement orientation. Pause and stop must
remain non-actionable and must not invoke fresh authority resolution.

### Blocking: fresh authority is not bound to the exact proposed operation

The injected resolver currently receives only scope and actor. It is not bound
to the accepted aggregate proof, decision, immutable continuation facts, or
exact lawful-next operation. The correction must create a payload-free private
request commitment covering the exact aggregate, decision, recipient, and
stable operation identity. All proof construction and validation must complete
before the resolver boundary is invoked. This remains conformance-only and
must not create a reusable capability or approval token.

### Blocking: completed-work exclusion has no stable identity

The proof currently rejects only an exact canonical text value shared by a
`CompletedObligation` and `LawfulNextOption`. Rephrasing the same completed work
can bypass that check. The correction must compare stable typed obligation and
operation identities or commitments, including completed and superseded
disposition. Until then the implementation proves only one exact-value replay
fixture, not that completed work cannot be repeated generally.

## Narrow Review Correction

Review also found that the proof accepted the same recipient identity for the
original and replacement orientations despite documentation claiming distinct
recipients. The private proof now rejects recipient identity reuse before any
authority resolution, with a focused regression test. Wording around the
existing completed-work check now says exact canonical value rather than
semantic equality.

## Verified Strengths

- The exact aggregate proof commitment must match before delivery.
- Claims, typed gaps, decision posture, and projection commitment must match.
- Gaps are bound into the delivery commitment and cannot silently disappear.
- Superseded accepted intent is excluded from the projected understanding.
- Delivery and receipt remain explicit non-authorizing boundaries.
- Distinct original and replacement recipient identities are now required.
- Debug surfaces do not expose semantic payloads or stable bindings.

## Validation

- focused continuation collection tests after the review correction: passed,
  49 tests;
- strict `workflow-core` Clippy with all targets and features: passed;
- formatting check: passed;
- documentation checks: passed; and
- the implementation phase's full workspace suite and warnings-denied rustdoc:
  passed before the narrow review correction.

## Next Authorized Phase

Perform one bounded `dg/blocker` phase that preserves typed pause/stop/resume
disposition, binds fresh authority resolution to the exact aggregate and stable
operation identity after proof validation, and replaces text-equality replay
protection with typed obligation/action identity. Add adversarial pause, stop,
superseded-resume, rephrased-completed-work, operation-substitution, and
post-proof resolver-order tests. Do not expose a public API, runtime consumer,
provider operation, persistence contract, schema, hosted behavior, nested
harness, agent memory system, or Salsa.
