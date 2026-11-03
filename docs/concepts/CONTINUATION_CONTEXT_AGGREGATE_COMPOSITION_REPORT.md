# Continuation Context Aggregate Composition Report

## Status

Implemented as a private reference, non-consuming, non-authorizing Core
boundary. Focused maintainer/security review is required before any integrated
source-set composition, projection delivery, or runtime integration may begin.

## Governed Scope

- workflow: `dg/runtime-composition`
- run: `run-1793711989695174000-2`
- approval: `approval/run-1793711989695174000-2/composition-approved`
- presentation: `presentation/e361751e2a2d8067`
- approval outcome: granted by the delegated maintainer for the bounded Phase E
  composition scope

The approved scope was to compose every closed continuation collector under
stable pre- and post-collection anchors into one claim-level non-authorizing
decision and one complete source-accounting proof. The scope also allowed
focused deterministic, drift, completeness, non-consumption, provenance, and
redaction tests plus roadmap and report updates.

## Implementation

The private continuation collector registry now has a single aggregate
composition path. That path:

1. invokes exactly one collector for each of the 18 Core-owned collector kinds;
2. rejects collector-kind substitution before composition;
3. sorts and commits the complete observation set independently of registration
   order;
4. verifies the exact post-collection anchor matches the request's
   pre-collection anchor;
5. converts the complete observation set into the accepted decision input;
6. invokes the existing pure `derive_continuation_context` decision function in
   the same private call;
7. verifies the bundle and understanding remain non-authorizing; and
8. returns the decision with a redaction-safe aggregate proof chain.

The proof binds:

- the exact collection request commitment;
- the exact closed collector contract, including each collector kind, source
  class, and authority domain;
- the pre- and post-collection anchor commitments;
- the canonical complete observation-set commitment;
- the purpose-bound source-coverage commitment;
- the claim-level understanding commitment;
- the complete decision commitment;
- the number of closed collector kinds accounted for;
- whether collector accounting is complete;
- whether required source coverage is currently complete; and
- explicit non-consuming and non-authorizing posture.

Complete collector accounting and complete required-source coverage are kept
separate. A collector may be correctly accounted for as unavailable,
unsupported, denied, or stale. In that case the aggregate proof remains
complete about what happened while the derived decision remains blocked.

## Security Posture

The aggregate result is private and payload-free outside its embedded bounded
decision. Custom debug output redacts request, anchor, observation, coverage,
understanding, decision, and aggregate commitments. It reports only bounded
counts and postures.

The composition contract does not grant approval, capability, policy
permission, execution authority, or a lawful next action. It does not consume
required context, approval, capability, or idempotency state. The collector
trait exposes only collection, and the aggregate function has no consumption
callback. Existing store-backed conformance tests separately prove that the
accepted read contracts do not mutate event, SideEffect, or WorkReport state.

`non_consuming: true` is a property of this closed Core composition API, not a
cryptographic attestation that an arbitrary external producer performed no side
effect. No arbitrary external collector registration or runtime consumer is
available.

## Tests

Focused continuation tests prove:

- all 18 collector classes are bound into the aggregate proof;
- the closed collector contract commitment is bound into the aggregate proof;
- the coverage, understanding, and decision commitments in the proof exactly
  match the derived decision;
- collector registration order does not change the aggregate result or proof;
- a changed collector posture changes observation, coverage, and aggregate
  commitments;
- post-collection anchor drift rejects the whole aggregate before a decision is
  returned;
- current accepted intent reaches the claim-level projection with bounded
  source provenance;
- the result remains non-authorizing; and
- debug output does not expose request or decision commitments.

The existing collection conformance suite continues to prove missing,
duplicate, substituted, stale, unsupported, denied, failed, and empty postures
remain distinct and fail closed where required.

## Explicit Boundaries

This phase does not implement:

- a public CLI, schema, SDK, hook, or report projection;
- post-compaction delivery or acknowledgment;
- a runtime or replacement-executor consumer;
- authority resolution or operation admission;
- required-context, approval, capability, or idempotency consumption;
- a production collector registry;
- a production accepted-intent persistence backend;
- host directive ingestion or raw conversation storage;
- provider reads or writes;
- hosted or distributed behavior;
- nested harnesses, agent teams, or memory infrastructure; or
- Salsa or another incremental computation dependency.

The existing `InMemoryContinuationCollectorRegistry` remains a private
reference composition boundary. Store-backed, accepted-intent, and bounded
local-observer contracts remain independently tested private source sets. They
have not yet been composed together through one integrated private aggregate
path. This phase therefore does not claim an operational Phase E exit or that a
production host can register and execute their combined aggregate path.

## Validation

- focused continuation-context tests: passed, 79 tests;
- full workspace tests: passed, including 718 Core unit tests and 176 CLI
  tests;
- strict `workflow-core` Clippy: passed with dead-code allowance limited to the
  existing private-future-contract posture;
- warnings-denied rustdoc: passed;
- formatting: passed;
- documentation checks: passed;
- diff checks: passed.

## Next Review

Focused maintainer/security review challenged whether the proof chain is
complete, whether required-source availability is distinguished from complete
collector accounting, whether any bypass path can produce an unbound decision,
whether non-consuming posture is overstated, and whether the private boundary
is narrow enough to precede projection delivery safely. The review required one
blocking correction: bind the exact closed collector contract into the proof so
collector-to-source or collector-to-authority remapping cannot silently preserve
an aggregate commitment. The review accepts this reference composition after
that correction, but requires integrated private source-set conformance before
projection delivery.
