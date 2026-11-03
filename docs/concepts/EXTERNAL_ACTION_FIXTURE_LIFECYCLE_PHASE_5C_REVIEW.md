# External Action Fixture Lifecycle Phase 5C Review

Status: accepted after focused maintainer/security review and blocker
correction.

## Decision

Accept Phase 5C. The evidence-wait, connector-refusal-wait, explicit-resume,
and exact-successor mark-ready slice is sufficiently bounded for the local
Core-private conformance fixture. Phase 5D may implement only the accepted
merge, replay, and ambiguity-recovery scope. This decision does not authorize
live GitHub, credentials, network access, a scheduler, public connector APIs,
broader mutation families, nested harnesses, release, or publication.

This is owner-conducted repository assurance, not independent external
security review.

## Findings And Corrections

### 1. Wait resolution trusted an opaque satisfaction commitment

The initial implementation persisted rich wait fields but resolved a wait with
only its condition identity, terminal posture, and an opaque commitment. The
generic resume helper then treated any final satisfied-wait event as resumable.
That made the reusable lifecycle primitive weaker than the fixture wrapper
that happened to call it.

Corrected:

- wait resolution now persists a typed payload-free proof;
- the proof binds condition version, expected sequence, expected event,
  required reference, wake trigger, source observation, terminal posture, and
  resolution time;
- construction and rehydration recompute and validate the proof commitment;
- wait registration validates the exact prior cursor and event commitment;
- required-reference and condition identities must share the exact derived
  suffix; and
- generic admission receives a resume attempt only from a structurally
  verified satisfied-wait event while still re-resolving current authority.

### 2. Provider replay did not bind the exact successor

The initial fixture receipt compared only operation identity and transition
kind. A different successor number attached to the same ready operation could
therefore be mistaken for exact replay.

Corrected:

- every fixture provider receipt stores a commitment over the immutable
  provider operation content, including the exact successor identity;
- both compare-and-apply replay and receipt lookup reject operation-content
  substitution; and
- focused tests reject a different successor during direct replay and
  reconciliation lookup.

### 3. Dispatch-time refusal and check regression became ordinary failure

The initial implementation registered waits only from pre-dispatch
observation. A connector refusal or required-check regression discovered
inside the provider compare-and-operate boundary produced `OutcomeFailed` and
could strand the next call outside the intended wait path.

Corrected:

- the fixture connector projects these two pre-mutation provider refusals into
  a closed typed refusal vocabulary;
- the failed attempt is recorded honestly;
- Core rereads provider posture, reserves a distinct retry attempt, and
  registers the exact `CapabilityUnavailable` or `CheckRequired` wait in the
  same call; and
- explicit re-entry resolves that wait and proceeds only after fresh static
  and current-authority validation.

### 4. Resume drift tests exercised only initial admission

Changed-head and revoked-authority tests initially mutated state before any
wait existed. They did not prove that re-entry after suspension remained
fail-closed.

Corrected:

- both paths now register a durable evidence wait first;
- the head or revocation posture changes before re-entry; and
- re-entry rejects before ready connector mutation.

## Accepted Boundaries

- The projection contains create, ready, and later merge scope before the
  first connector entry, while this phase executes only create and ready.
- Mark-ready requires the exact Core-owned successor binding and unchanged
  reviewed head.
- Pending or failed checks and proven connector unavailability create durable
  waits rather than fabricated completion.
- Dispatch-time pre-mutation refusal is distinguishable from ambiguous or
  started provider execution.
- Every resumed connector attempt revalidates current authority and evidence.
- Exact ready replay and exact receipt lookup do not redispatch.
- The fixture remains local, deterministic, credential-free, and
  non-production.

## Validation

Passed after correction:

- focused external-action tests, including wait-proof substitution,
  successor-replay substitution, post-wait head/revocation drift, and
  dispatch-time refusal/check regression;
- all 754 all-feature `workflow-core` library tests;
- strict all-target, all-feature `workflow-core` Clippy;
- `cargo fmt --all -- --check`;
- `npm run check:docs`; and
- `git diff --check`.

## Remaining Limits

Phase 5C does not implement squash merge, branch deletion, live GitHub,
network access, credentials, provider ambiguity recovery, automatic polling,
background continuation, schemas, SDKs, CLI mutation commands, broader
provider mutation, nested harness execution, release, or publication.

The typed resolution proof establishes exact structural and provenance
binding to the fixture observation commitment. It does not make an observation
authoritative by itself; authority remains in the source composition and
same-call admission boundary.

## Next Authorized Increment

Phase 5D may implement only exact unchanged-head squash merge, terminal replay,
and the accepted ambiguity/reconciliation matrix for create, ready, and merge.
Phase 5E and any live connector work remain unauthorized until Phase 5D is
implemented and focused-review accepted.
