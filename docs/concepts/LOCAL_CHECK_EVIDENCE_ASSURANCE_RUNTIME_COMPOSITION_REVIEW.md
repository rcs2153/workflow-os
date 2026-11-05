# Local Check Evidence Assurance Runtime Composition Review

## Decision

Accepted for the narrow closed trusted-host project-validation boundary.

The implementation establishes Core-owned construction provenance by creating
the projection inside the same crate-private call that builds and dispatches
the exact process request. It does not accept a projection as an input, and it
does not treat a serialized projection or deterministic fingerprint as proof of
Core observation.

## Reviewed Properties

The review confirmed:

1. The resolved-execution fingerprint is computed from the exact request passed
   to `run_process`, including executable bytes, arguments, working directory,
   sanitized environment, timeout, and selected executable content hash.
2. The projection binds the immutable execution, handler selection, result,
   workflow, run, step, and immutable bundle already held by Core.
3. The projection remains weak and unattested even though the existing
   governance contribution may separately accept its crate-private local
   process proof.
4. Every authoritative route exposes the projection through a read-only
   accessor while keeping result fields private.
5. Existing-terminal reassessment constructs a fresh projection rather than
   replaying or upgrading serialized assurance.
6. No minimum-assurance comparison participates in execution, policy,
   approval, reporting, or persistence.
7. Debug output remains payload-free and commitment-redacted.

## Residual Risks

This phase proves only that Core constructed the weak projection from one
trusted-host call. It does not authenticate the operator-selected executable,
make the host independent, persist assurance, protect storage-owner edits,
anchor a WorkReport body, or provide external verifiability.

The public projection constructor remains intentionally available as model
vocabulary. Therefore consumers must continue to distinguish a projection
returned inside a Core-owned route from a caller-created or deserialized
projection. No future gate may infer source provenance from the projection
fingerprint alone.

Changing the existing `into_parts` tuple contracts was intentionally avoided;
callers that need assurance must use the route's explicit read-only projection
accessor before consuming legacy tuple parts. A future API version may add a
new owned-parts shape, but that is not required for this closed slice.

## Validation Reviewed

- Focused fresh-route and terminal-reassessment behavior tests passed.
- Warnings-denied all-target/all-feature Core Clippy passed.
- The complete Core all-feature test suite and doc tests passed.
- Formatting and diff checks passed.
- Declared opt-in live credential and external-tool tests remained outside this
  local phase.

## Next Phase

Proceed to the model-only versioned complete WorkReport body commitment. Do not
add persistence anchors, read/replay enforcement, product projection, signing,
provider mutation, or publication in that first slice.
