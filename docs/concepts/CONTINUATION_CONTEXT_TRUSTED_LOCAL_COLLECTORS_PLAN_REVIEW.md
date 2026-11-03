# Continuation Context Trusted Local Collectors Plan Review

## Decision

Accepted after two blocking corrections.

The plan now preserves existing source ownership, keeps collection read-only,
and provides a credible staged boundary between the accepted pure semantic
model and future operational composition. Phase A, the private collector model
and reference conformance layer, may proceed. Store-backed collectors, local
observers, accepted-intent capture, aggregate composition, previews, and
runtime consumers still require their own governed phases and reviews.

## Blocking Finding 1: Aggregate Cursor Was Incomplete

The first draft captured the immutable run bundle and event cursor before
collection and reread them afterward. That protects run and event posture but
does not freeze independently mutable approval, capability, required-context,
evidence, check, SideEffect, WorkReport, handoff, Git, or provider sources.

Combining those sources under only the event cursor could produce a canonical
but temporally incoherent understanding.

The correction requires every independently mutable authoritative collector to
provide a source-specific revision fence or equivalent exact snapshot proof.
Core must reread all required fences after collection. Any drift invalidates the
entire observation set. A store without a stable read contract remains typed
`Unsupported`; queryability alone is not enough to advertise collector support.

## Blocking Finding 2: Observation Could Spend Authority

The first draft described aggregate composition through required-context
consumption. That conflicts with the declared read-only and non-authorizing
collector boundary. A preview or orientation build must not spend one-use
required context, capability, approval authority, or idempotency state.

The correction separates non-consuming observation from material consumption.
Collectors may observe current posture only. A later same-call material
consumer must freshly recollect and revalidate context before using the existing
atomic consumption boundary. Stores that expose only consuming reads remain
unsupported in collection.

## Accepted Properties

- Core owns the closed collector registry and exact source mappings.
- Collector kind fixes authority domain, allowed claims, sensitivity, and
  result shape.
- Coverage and semantic requirements remain Core-derived.
- Empty is explicit complete negative knowledge, not missing output.
- Failure, denial, unsupported, stale, and omission remain distinct.
- Every authoritative source needs a stable revision fence or snapshot proof.
- Aggregate drift invalidates the complete set rather than mixing revisions.
- Collection is read-only, ephemeral, deterministic, and non-authorizing.
- Structured sources emit only contract-permitted typed claims.
- Git and operating documents begin as bounded reference observations.
- Operating documents cannot grant permission or establish check/evidence
  success.
- Existing stores remain the sole owners of their facts.
- Accepted intent remains unsupported until its separate durable capture
  boundary is accepted.
- Public projection, material consumers, and execution stay deferred.

## Phase A Boundary

The next implementation may add only:

- private closed collector kinds and mappings;
- private request, anchor, result, observation, and composition types;
- fixture collectors and an in-memory reference registry;
- contract validation;
- deterministic conversion into the accepted pure decision input; and
- conformance tests for completeness, substitution, result posture, ordering,
  anchor drift, redaction, and non-authorizing behavior.

It may not read real stores, persist collector state, inspect Git or documents,
capture accepted intent, expose CLI/schema/SDK surfaces, consume authority,
invoke a runtime consumer, or add Salsa.

## Governed Review

- workflow: `dg/review`
- run ID: `run-1793702499409797000-2`
- approval ID:
  `approval/run-1793702499409797000-2/review-scope-approved`
- presentation ID: `presentation/609a8486b8aad101`
- approval outcome: granted under delegated-maintainer authority after review
  of the complete persisted handoff

## Validation

- documentation checks: passed after correction
- diff and whitespace checks: passed after correction

## Out-Of-Kernel Work

The delegated maintainer reviewed collector trust, source mappings, stable
anchors, negative knowledge, structured claims, document boundaries, source
reuse, and phase sequencing; corrected the two blocking issues; and authored
this review outside the kernel. The kernel governed scope and approval but did
not inspect files, edit documentation, run checks, create commits, or mutate
GitHub.
