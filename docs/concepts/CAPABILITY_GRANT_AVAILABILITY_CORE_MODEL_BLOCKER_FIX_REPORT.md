# Capability Grant And Availability Core Model Blocker Fix Report

## 1. Executive Summary

The capability availability source-of-truth blocker is fixed. Availability
records now describe inventory and connectivity posture only. They cannot
assert that a capability is authorized, denied, expired, or revoked.

## 2. Blocker Fixed

The original model allowed a caller or serialized payload to construct an
availability record with `available_and_authorized`, `denied`, or
`expired_or_revoked` despite carrying no grant or decision proof. That blurred
the mandatory boundary between capability inventory and scoped authority.

## 3. Implementation Approach

`CapabilityAvailability` now contains only:

- `available`;
- `declared_not_connected`;
- `known_unsupported`;
- `unknown`.

The authority-bearing states were removed. A future pure resolver must derive
authority, denial, expiry, and revocation from explicit validated grants and
independent policy, approval, evidence, and check inputs.

## 4. Validation Boundary

Availability record constructors and deserialization continue to validate the
capability reference, bounded resource, timestamp type, and redaction metadata.
Serde now rejects the removed authority-bearing values as unknown variants.
Unknown and unsupported inventory posture remain explicit fail-closed inputs
for future resolution.

## 5. Privacy And Redaction

The fix adds no new payload fields. Availability records still contain only a
stable capability reference, bounded resource reference, inventory posture,
observation timestamp, and validated redaction metadata. Debug and serde error
paths remain non-leaking.

## 6. Test Coverage

A focused regression test proves:

- `available` remains a valid inventory observation;
- `available_and_authorized` is rejected;
- `available_not_authorized` is rejected;
- `expired_or_revoked` is rejected;
- `denied` is rejected.

The focused capability-authority suite passes with 16 tests.

## 7. Commands And Results

- `cargo fmt --all --check`: passed with the bundled toolchain.
- `cargo test -p workflow-core --test capability_authority`: 16 passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed; explicit live provider tests remained ignored
  by design.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 8. Governed Phase Record

- workflow: `dg/blocker`
- run ID: `run-1784163172968637000-2`
- approval ID: `approval/run-1784163172968637000-2/fix-approved`
- approval presentation ID: `presentation/2947c8ee3e193000`
- approval outcome: granted under delegated maintainer authority
- status: completed before repository phase close

## 9. Remaining Known Limitations

- No pure runtime resolver exists.
- Availability freshness is represented by `observed_at` but not evaluated.
- Grants and prerequisite references are not consumed at runtime.
- Capability requests, projections, receipts, tools, connectors, and writes
  remain unimplemented.

## 10. Recommended Next Phase

Perform a focused blocker-fix maintainer review. If accepted, proceed to the
pure capability resolution helper. Do not add runtime mutation, tool execution,
provider behavior, schemas, CLI behavior, or hosted features during review.
