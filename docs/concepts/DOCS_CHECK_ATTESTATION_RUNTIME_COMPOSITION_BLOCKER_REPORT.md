# DocsCheck Attestation Runtime Composition Blocker Report

## 1. Executive Summary

The runtime-composition implementation stopped before code changes after
inspection found an identity-relabelling gap in the prerequisite verifier.

The verifier derives and validates the stored immutable bundle root, then
requires the execution binding, candidate, and Core-owned observation to cite
that root. It also requires those three supplied inputs to agree on workflow
and run identity. It does not require their workflow and run identities to
equal `StoredImmutableRunBundle::manifest().workflow_id()` and
`manifest().run_id()`.

Consistent relabelling could therefore produce accepted proof for a different
workflow/run identity while citing a valid stored bundle root. Runtime
composition must not be built on that incomplete source-of-truth check.

## 2. Blocker

Before any composition helper is implemented, the verifier must:

- compare execution-binding workflow identity to the stored manifest workflow
  identity;
- compare execution-binding run identity to the stored manifest run identity;
- continue requiring candidate and observation identities to equal the
  execution binding; and
- fail with a stable non-leaking bundle/identity mismatch error and no partial
  accepted proof.

Focused regression coverage must prove that changing workflow and run identity
consistently across execution binding, candidate, and observation still fails
when those values do not match the stored manifest.

## 3. Why Existing Root Validation Is Insufficient

The stored manifest root commits workflow and run identity, but
`ImmutableRunBundleBinding` exposes only bundle ID, bundle version, and root
hash. An execution binding can cite that exact root while carrying separately
supplied workflow and run IDs.

Agreement among caller-supplied execution, candidate, and observation context
is not evidence that those identities came from the stored manifest. The
verifier must derive and compare them explicitly.

## 4. Scope Preserved

No runtime-composition code, process execution, executor change, default
registration, persistence, events, evidence, reports, artifacts, schemas, CLI,
providers, SideEffects, writes, hosted behavior, or release changes were added.

## 5. Governed Phase

- workflow: `dg/runtime-composition`
- run: `run-1784567358916370000-2`
- approval: `approval/run-1784567358916370000-2/composition-approved`
- presentation: `presentation/0791265c3ed1112d`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase outcome: implementation halted at a prerequisite integrity blocker
- kernel boundary: governance coordination only; repository inspection and
  blocker documentation ran outside the kernel

## 6. Validation

- `npm run check:docs` - passed.
- `git diff --check` - passed.
- governed phase close - completed with 39 events, one proof-backed approval,
  zero retries, and zero escalations.

The close helper reported the known presentation-record read cap after proof
had already been persisted and enforced.

## 7. Recommended Next Phase

Run a focused verifier blocker fix and maintainer review. Resume the accepted
DocsCheck runtime-composition plan only after the stored-manifest workflow/run
identity invariant is proven.
