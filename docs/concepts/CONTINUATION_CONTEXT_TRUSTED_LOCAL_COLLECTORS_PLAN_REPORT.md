# Continuation Context Trusted Local Collectors Planning Report

## Status

P0 collector-boundary planning complete. Focused architecture/security review
is required before implementation.

## Decision

The next authoritative continuation-context increment will use a closed,
Core-owned collector registry. Collectors return bounded typed observations;
they do not choose source authority, coverage expectations, semantic
obligations, conflicts, or readiness.

The plan separates five reviewed implementation phases:

1. private collector model and reference composition;
2. store-backed kernel collectors;
3. bounded Git, operating-document, and typed-handoff observers;
4. separately reviewed accepted-intent capture; and
5. aggregate stable-anchor composition.

No preview or runtime consumer is authorized until aggregate composition is
accepted.

Focused review later identified and corrected two planning blockers. Event
cursor and immutable-bundle stability alone cannot prove a coherent aggregate
across independently mutable stores, so every authoritative collector now
requires a source-specific revision fence or exact snapshot proof. Collection
is also explicitly non-consuming: required context, capabilities, approval
authority, and idempotency state remain untouched until a later fresh same-call
material boundary.

## Important Boundaries

- Run-bound collection is anchored to the immutable bundle and event cursor,
  then verifies those anchors again before derivation.
- Pre-run collection is anchored to validated declarations and an injected
  Git/worktree revision, then rejects drift.
- `Empty` is typed complete negative knowledge. Unsupported, denied, failed,
  stale, and omitted results cannot masquerade as empty.
- Structured kernel sources may emit only contract-permitted typed claims.
- Git and operating-document collectors begin as bounded inventory/reference
  observations, not natural-language authority.
- Existing stores remain authoritative; no parallel persistence is introduced.
- Accepted intent stays unsupported until its separate capture boundary exists.
- Every result remains private and non-authorizing.

## Governed Planning

- workflow: `dg/d`
- run ID: `run-1793702360406708000-2`
- approval ID:
  `approval/run-1793702360406708000-2/planning-approved`
- presentation ID: `presentation/c8a636a6141d12f7`
- approval outcome: granted under delegated-maintainer authority after review
  of the complete persisted handoff

## Validation

- documentation checks: passed
- diff and whitespace checks: passed

## Out-Of-Kernel Work

The delegated maintainer inspected the accepted model, semantic profiles,
authoritative rehydration sequence, and existing store boundaries, then authored
the plan and roadmap updates outside the kernel. The kernel governed scope and
approval but did not read files, edit documentation, execute checks, create a
commit, or mutate GitHub.
