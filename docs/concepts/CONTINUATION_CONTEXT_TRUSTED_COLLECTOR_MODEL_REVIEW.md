# Continuation Context Trusted Collector Model Review

## Decision

Accepted after one blocking negative-knowledge finding was fixed. Phase A is a
private, non-authorizing prerequisite for authoritative post-compaction
understanding. It does not claim that trusted stores, accepted intent,
understanding projections, delivery acknowledgments, or runtime consumers are
implemented.

## Governed Review

- workflow: `dg/review`
- run ID: `run-1793704321127948000-2`
- approval ID: `approval/run-1793704321127948000-2/review-scope-approved`
- presentation ID: `presentation/206b752337d33c2f`
- approval outcome: granted under delegated-maintainer authority after review
  of the complete persisted handoff

## Finding

### Fixed: empty results did not require complete typed accounting

The initial implementation allowed an `Empty` collector result with an
incomplete or absent negative-knowledge accounting set. Later semantic
derivation remained conservative, but the collector boundary had not proven
that "nothing found" covered every claim kind for which that source class is
the Core-owned accounting source.

The boundary now derives the exact expected accounting set from the closed
semantic profile and requires exact equality. Missing, extra, or duplicate
accounting fails closed before conversion. A focused regression test covers
both rejection and acceptance.

## Accepted Properties

- Core owns the closed 18-kind collector registry and every source-class and
  authority-domain mapping.
- Requests bind purpose, scope, actor, sensitivity ceiling, materialization
  budget, and the complete pre-collection anchor.
- Every independently mutable source has an exact revision fence.
- Collector kind, request, revision, or aggregate-anchor substitution rejects
  the complete batch.
- Empty negative knowledge requires exact Core-owned typed accounting.
- Unavailable, stale, denied, failed, unsupported, omitted, and inapplicable
  postures remain distinct.
- Accepted intent remains unsupported until its separately reviewed durable
  capture boundary exists.
- Output conversion is deterministic, private, redaction-safe, and incapable
  of authorizing execution.

## Compatibility With The P0 Understanding Invariant

This slice supplies trusted, bounded observations and coverage posture. It does
not by itself prove equivalent post-compaction understanding. The authoritative
P0 still requires durable accepted intent, private complete source coverage,
bounded context materialization, claim-level understanding with provenance,
explicit replacement-executor delivery acknowledgment, and fresh authority
resolution at every material boundary.

No collector output may grant approval, policy satisfaction, capability,
required-context consumption, or execution authority.

## Validation

- focused continuation collector tests: 15 passed
- strict `workflow-core` clippy with warnings denied: passed
- full workspace tests: passed
- rustdoc with warnings denied: passed
- formatting: passed
- documentation checks: passed
- diff and whitespace checks: passed

## Next Boundary

Phase B may implement store-backed kernel collectors only through existing
read interfaces and source-specific stable revision fences. It must remain
non-consuming and non-authorizing. Git and operating-document observers,
accepted-intent persistence, aggregate composition, runtime consumers, and
post-compaction delivery remain later separately reviewed phases.

