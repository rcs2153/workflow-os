# Continuation Context Trusted Collector Model Report

## Status

Phase A implemented as a Core-private, read-only reference model. Focused
maintainer/security review is required before any store-backed collector work.

## Delivered

Core now contains one closed continuation collector vocabulary covering all 18
accepted source classes. Every collector kind maps inside Core to exactly one
source class and one authority domain.

The private model adds:

- a complete source-revision anchor with optional run-bound immutable-bundle
  and event-cursor bindings;
- a Core-built purpose, scope, actor, sensitivity, budget, and anchor-bound
  collection request;
- nine closed result postures: observed, empty, unavailable, denied, failed,
  unsupported, stale, omitted optional, and inapplicable;
- bounded observed-source and typed empty-accounting forms;
- a private collector trait and in-memory reference registry;
- exact collector completeness and duplicate validation;
- request, collector-kind, revision, and anchor-drift checks;
- Core-owned source-class and authority-domain assignment;
- deterministic conversion into the accepted pure
  `ContinuationContextDecisionInput`; and
- redacted debug surfaces for request, anchor, revision, source, and observation
  types.

The accepted coverage model now preserves `Unavailable` and `Stale` as distinct
outcomes. Stale posture requires an explicit source with non-current freshness;
unavailable posture carries no source and cannot masquerade as generic failure
or empty collection.

## Security Properties

- The module is crate-private and marked as future-facing dead code.
- No production caller can register a collector.
- Private ephemeral collector types implement serialization for canonical
  commitments but not deserialization.
- Registry construction requires every closed collector kind exactly once.
- Collector output cannot substitute another collector kind, request, or source
  revision.
- The pre- and post-collection anchors must match exactly.
- Source class and authority domain are assigned by Core, not collector output.
- Result posture must match the Core-owned coverage expectation.
- Empty posture must carry the exact complete Core-owned typed accounting set.
- Observed and stale results require source payloads; failure and omission
  results reject them.
- Failure and omission postures cannot carry typed empty accounting.
- Accepted-intent collection remains forced to `Unsupported` until its separate
  capture boundary exists.
- The converted decision remains non-authorizing and blocked when mandatory
  collectors are unsupported.
- Debug output redacts request bindings, revisions, source identifiers, actor,
  scope, and semantic payloads.

## Focused Coverage

Fifteen focused tests prove:

- complete bijective collector/source/authority mapping;
- missing and duplicate registry rejection;
- deterministic output under reversed registration order;
- non-authorizing blocked decision composition;
- collector-kind substitution rejection;
- complete-batch invalidation after source-fence drift;
- Core-owned class and authority assignment;
- distinct unavailable and stale conversion;
- source-revision substitution rejection;
- inapplicable-profile enforcement;
- accepted-intent support remains unavailable;
- failure cannot carry empty accounting;
- request-binding substitution rejection; and
- debug redaction; and
- complete typed negative-knowledge accounting for empty results.

## Governed Implementation

- workflow: `dg/implement`
- run ID: `run-1793703420384645000-2`
- approval ID:
  `approval/run-1793703420384645000-2/implementation-approved`
- presentation ID: `presentation/4c2d071e63f5c179`
- approval outcome: granted under delegated-maintainer authority after review
  of the complete persisted handoff

## Validation

- focused continuation collector tests: 15 passed
- strict `workflow-core` clippy with warnings denied: passed with the existing
  narrow dead-code allowance for private future-facing models
- full workspace tests: passed
- rustdoc with warnings denied: passed
- formatting: passed
- documentation checks: passed
- diff and whitespace checks: passed

## Boundary Preserved

This phase did not read a real store, add persistence, inspect Git or operating
documents, capture accepted intent, expose CLI/schema/SDK behavior, consume
required context or authority, invoke a runtime consumer, perform provider
work, add nested harness behavior, add Salsa, or expand hosted behavior.

## Out-Of-Kernel Work

The delegated maintainer implemented the private Core model, fixture registry,
conversion path, focused tests, roadmap update, and this report; ran validation;
and reviewed diagnostics outside the kernel. The kernel governed scope and
approval but did not edit files, execute tests, create commits, or mutate
GitHub.
