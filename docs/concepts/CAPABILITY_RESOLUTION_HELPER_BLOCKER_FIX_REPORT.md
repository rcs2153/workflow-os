# Capability Resolution Helper Blocker Fix Report

## 1. Executive Summary

The capability resolution wire-invariant blocker is fixed.
`CapabilityResolution::validate` now binds every `NotAuthorized` reason shape
to its matching availability posture, so custom deserialization fails closed
for contradictory denial explanations.

The helper remains pure, in-memory, non-mutating, and unused by runtime
execution.

## 2. Blocker Fixed

The review found that `NotAuthorized` validation rejected authorization and
prerequisite reasons but did not prove that the remaining reasons agreed with
the recorded availability value. Impossible wire states could therefore pass
deserialization.

The fix enforces this matrix:

| Availability | Permitted denial reasons |
| --- | --- |
| absent | `availability_record_missing` only |
| `declared_not_connected` | `capability_not_connected` only |
| `known_unsupported` | `capability_unsupported` only |
| `unknown` | `capability_availability_unknown` only |
| `available` | `no_matching_grant` alone, or one or more ordered grant rejection reasons |

Grant rejection reasons are limited to revoked, expired, and sensitivity
insufficient. `no_matching_grant` cannot be mixed with rejected-grant reasons.

## 3. Implementation Approach

The fix adds one private predicate used by the existing validated result
boundary. It does not create a second constructor or a parallel validation
path. Runtime-created results, serde round trips, and future persisted or
schema-fed results therefore share the same invariant.

The earlier specificity hardening remains in place: only the highest-specificity
matching grant tier is evaluated, so broader grants cannot bypass narrower
prerequisites.

## 4. Scope Explicitly Not Added

This fix adds no executor integration, capability request, tool projection,
authority receipt, policy or approval evaluation, persistence, event, schema,
CLI behavior, connector, provider write, hosted administration, RBAC, IdP
integration, or release change.

## 5. Privacy And Error Behavior

The new validation compares bounded enums only. It adds no payload field and
does not echo capability, resource, actor, workflow, run, step, harness, or
grant identifiers in errors. The existing stable
`capability_authority.resolution.inconsistent` error remains the public failure
boundary.

## 6. Test Coverage

The focused suite now includes 30 tests. New invalid-wire coverage proves that:

- available inventory cannot claim a not-connected reason;
- an unavailable enum cannot carry a different unavailable reason;
- absent inventory cannot claim a grant-matching result;
- `no_matching_grant` cannot be mixed with grant rejection reasons;
- deserialization errors remain non-leaking.

Existing authorization, specificity, prerequisite, lifecycle, sensitivity,
serde, and privacy tests remain green.

## 7. Validation

- `cargo test -p workflow-core --test capability_authority --quiet`: passed,
  30 tests.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace --quiet`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 8. Remaining Limitations

- Availability freshness has no caller-defined maximum-age policy.
- Independent policy, approval, evidence, and check evaluation remains future
  composition work.
- The resolver is not consumed by executor or immutable run-bundle paths.
- Capability requests, step-scoped projection, and authority receipts do not
  exist yet.

## 9. Recommended Next Phase

Perform a focused blocker-fix review. If accepted, proceed to the capability
request model and review-only projection. Do not begin tool invocation or a new
provider mutation family first.

## 10. Governed Phase Evidence

- Workflow: `dg/blocker`.
- Run ID: `run-1784170923180844000-2`.
- Approval ID: `approval/run-1784170923180844000-2/fix-approved`.
- Approval presentation: `presentation/7593425564a494bc`.
- Approval outcome: granted with persisted presentation proof under delegated
  maintainer authority.
- Out-of-kernel work: Codex changed Rust validation and tests, authored this
  report, and ran validation commands. The kernel coordinated governance only.
- Report posture: no runtime WorkReport artifact was created or persisted.
