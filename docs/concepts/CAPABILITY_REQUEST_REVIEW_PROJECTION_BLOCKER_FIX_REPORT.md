# Capability Request Review Projection Blocker Fix Report

## 1. Executive Summary

The capability request and review-projection semantic-integrity blockers are
fixed. Resolutions now retain the exact identity and scope context used by the
pure resolver, requests require exact equality with that context, and review
projections reuse canonical posture/reason validation.

The fix remains model-only. It adds no grant issuance, runtime authority,
connector activation, tool visibility, invocation, persistence, events,
schemas, CLI behavior, provider writes, hosted behavior, or release changes.

## 2. Blockers Fixed

### Resolution Context Substitution

Previously, a valid resolution from one actor, resource, or run could be
attached to a request describing another. `CapabilityResolutionContext` now
records capability, bounded resource, actor, workflow, run, step, optional
harness contract, and requested sensitivity.

`resolve_capability_authority` derives this context directly from its explicit
input. `CapabilityRequest::new` rejects any mismatch with stable code
`capability_authority.request.resolution_context_mismatch`.

### Projection Posture/Reason Mismatch

Previously, a wire projection could pair a non-authorized posture with an
authorized reason if its action matched that reason. Resolution and projection
validation now share one canonical posture/reason invariant. Invalid pairs fail
with `capability_authority.request_projection.reasons_inconsistent`.

## 3. Implementation Approach

- Added public read-only `CapabilityResolutionContext` vocabulary.
- Embedded validated context in each serialized `CapabilityResolution`.
- Kept context Debug output redacted.
- Required request fields and sensitivity to equal resolution context.
- Extracted shared deterministic posture/reason validation.
- Preserved stricter availability and selected-grant checks in full resolution
  validation.
- Preserved reason/action recomputation in projection validation.

## 4. Security And Privacy Boundary

The fix prevents review-context substitution; it does not claim freshness,
source attestation, or authority. Stable context references are serialized as
model data, while Debug output redacts them. Existing identifier and resource
constructors reject unsafe or secret-like values. Errors describe only the
failure class and do not echo context values.

Any future grant issuance, tool projection, or invocation must still re-resolve
authority using current availability, grants, prerequisites, scope,
sensitivity, and time. A context-bound request remains non-authoritative.

## 5. Tests Added

- actor mismatch fails closed;
- resource mismatch fails closed;
- run mismatch fails closed;
- requested-sensitivity mismatch fails closed;
- non-authorized posture with active-grant reason fails closed;
- independent-evaluation posture with denial reason fails closed;
- existing resolution serde tests now include validated context;
- existing request, projection, grant, availability, and resolution tests pass.

## 6. Validation

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test -p workflow-core --test capability_authority --quiet`: passed,
  44 tests.
- `cargo test --workspace --quiet`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 7. Dogfood Governance

- workflow: `dg/blocker`
- run ID: `run-1784176033254308000-2`
- approval ID: `approval/run-1784176033254308000-2/fix-approved`
- presentation ID: `presentation/eccc2d9487a1399f`
- approval outcome: granted under delegated-maintainer authority after complete
  handoff presentation.

## 8. Remaining Limitations

- Context binding is not an immutable-run-bundle commitment.
- Availability and grants may change after request creation.
- No persistence or stale-request behavior exists.
- No grant issuance or runtime consumer exists.
- No tool/context projection or authority receipt exists.

## 9. Recommended Next Phase

Perform a focused blocker-fix maintainer review. Verify exact context binding,
canonical reason validation, wire compatibility posture, privacy, and full
regression coverage before beginning tool or context projection.
