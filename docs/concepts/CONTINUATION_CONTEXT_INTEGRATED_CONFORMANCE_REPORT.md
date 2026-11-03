# Continuation Context Integrated Conformance Report

## Status

Implemented and accepted after focused maintainer/security review as a private,
read-only, non-consuming, non-authorizing Phase E conformance boundary.
Projection delivery and replacement-executor work remain separate phases.

## Governed Scope

- workflow: `dg/runtime-composition`
- run: `run-1793714571817652000-2`
- approval: `approval/run-1793714571817652000-2/composition-approved`
- presentation: `presentation/e33beca28e1ad782`
- approval outcome: granted by the delegated maintainer for the bounded
  integrated private conformance scope

The phase was authorized to compose the already accepted store-backed,
bounded-local-observer, and accepted-intent contracts under one stable anchor
and the accepted aggregate proof. Provider observation and agent advisory were
required to remain explicit closed postures rather than fabricated data.

## Implementation

`IntegratedContinuationSourceSet` is a private Core composition boundary. It:

1. captures the accepted store-backed anchor;
2. layers the accepted bounded-local-observer revisions over that anchor;
3. layers the accepted-intent revision over the combined anchor;
4. constructs one request bound to the complete layered anchor;
5. invokes all 12 store-backed collectors, all three bounded local collectors,
   and the accepted-intent collector through their existing validation paths;
6. invokes two explicit closed-posture collectors for provider observation and
   agent advisory;
7. recaptures the complete layered anchor after collection; and
8. invokes the accepted aggregate composition and proof only when the complete
   pre- and post-collection anchors are equal.

The child source-set contracts expose only the minimum `pub(super)` surface
needed for their parent private composition. No type becomes public outside the
crate-private continuation collection module.

## Closed Postures

Provider observation and agent advisory have no accepted producer in this
phase. Their collectors therefore return only Core-owned `Unsupported` or
`Inapplicable` posture, depending on the existing purpose/scope expectation.
Their revisions are derived from the same deterministic outside-batch contract
used by the initial store-backed anchor. They cannot return source payloads,
claims, authority, or readiness.

## Conformance Proof

The stable-state test composes one real private source set using:

- the existing local state and immutable-bundle stores through their read-only
  contracts;
- a bounded injected local observer that returns request-bound Git and exact
  operating-document observations;
- an accepted-intent store containing one attributable current objective; and
- the two explicit closed postures.

The result accounts for all 18 collector kinds, carries the accepted aggregate
proof, preserves the accepted-intent objective with provenance, and remains
non-authorizing.

The drift test changes the Git/worktree observation only during post-collection
anchor capture. Initial anchor capture and collection agree, but the complete
post-collection anchor differs. The entire aggregate is rejected with
`continuation_collection.anchor.drifted`; no decision is returned.

## Explicit Boundaries

This phase does not implement:

- a production collector registry;
- a public API, CLI, schema, SDK, hook, or report projection;
- post-compaction projection delivery or acknowledgment;
- a runtime or replacement-executor consumer;
- authority resolution, operation admission, or lawful next-action selection;
- approval, capability, required-context, evidence, or idempotency consumption;
- provider reads or writes;
- host directive ingestion or raw conversation storage;
- production accepted-intent persistence;
- hosted or distributed behavior;
- nested harnesses, agent teams, or memory infrastructure; or
- Salsa or another incremental computation dependency.

## Validation

- focused continuation collection tests: passed, 41 tests;
- strict `workflow-core` Clippy: passed;
- full workspace tests with all features: passed;
- warnings-denied workspace rustdoc: passed;
- formatting check: passed;
- documentation checks: passed; and
- diff hygiene check: passed.

## Review Outcome

Focused review accepted the integrated composition without a blocking finding.
The review confirmed that the layered anchor preserves child revisions and
run-bound anchors, every accepted collector retains its own validation path,
closed postures cannot manufacture source data or authority, cross-source drift
rejects the complete aggregate, and visibility remains limited to the private
parent module. See the [focused review](CONTINUATION_CONTEXT_INTEGRATED_CONFORMANCE_REVIEW.md).
