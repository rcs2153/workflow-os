# Hosted Recovery And Cross-Harness Queue Reconciliation Review

## Verdict

Accepted after three documentation-level boundary corrections. No runtime
implementation is authorized by this review.

The revised queue responds to externally reproduced correctness evidence and
moves the project toward an externally testable product milestone without
weakening the local-first kernel boundary. Hosted partial-run recovery is a
proper P0 because a server-owned immutable run prefix must not become
unrecoverable through an ordinary exact retry. The cross-harness milestone is
the right follow-on because it tests whether the authoritative record is usable
across execution hosts rather than only inside Workflow OS's private machinery.

## Findings And Corrections

### 1. Repaired History Must Be Deterministic

The initial recovery plan correctly assigned repair to Core, but it did not
explicitly prohibit constructing missing history from a current projection.
That would let recovery invent causality.

The plan now requires every missing transition to be rederived from the exact
immutable request with stable event and idempotency identities. If Core cannot
recreate the exact transition the original route would have emitted, the state
fails closed for bounded operator resolution.

### 2. Replacement Harness Authority Must Be Explicit

The initial cross-harness plan prohibited conversational authority inference
but did not require the second harness to prove its actor and delegation
posture. A valid continuation record is not permission for every host or actor
to act.

The plan now separates same-human, delegated-agent, service-actor, and
unauthorized substitution cases. A checkpoint remains non-authorizing.

### 3. Exactly-Once Language Must Stay Bounded

The requested milestone should verify no duplicate effect, but Workflow OS
must not imply a universal exactly-once guarantee across a remote provider.

The plan now scopes the assertion to one accepted attempt and one reconciled
outcome at the selected existing idempotency and provider-observation boundary,
with no duplicate Workflow OS SideEffect or completion claim.

## Accepted Architecture Boundaries

- Core owns classification and repair of durable creation prefixes.
- The hosted API maps outcomes but does not infer or repair governance state.
- Exact retry is bound to immutable request and current required facts.
- Recovery reuses the accepted dispatch seam instead of adding another queue.
- The two harnesses use documented public integration surfaces.
- The selected consequential effect must already have a reviewed path.
- Independent review distinguishes kernel proof, host assertions, and provider
  observations.
- Governed Obligation Plan executor composition remains deferred.

## Governed Review

- workflow: `dg/review`
- run: `run-1794009394897022000-2`
- approval: `approval/run-1794009394897022000-2/review-scope-approved`
- presentation: `presentation/9565eb537e7962a3`
- approval outcome: granted under delegated-maintainer authority
- review boundary: documentation and planning only

## Validation

- `npm run check:docs`
- `git diff --check`

Both checks passed before review close.

## Next Authorized Phase

Independently reproduce the hosted partial-run creation defect and write the
exact durable-prefix transition matrix. Do not change production code until a
focused review accepts the reproduction, source-of-truth analysis, and one
Core-owned recovery seam.
