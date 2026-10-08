# Explicit Zero Required-Context Contract Plan Report

## 1. Executive Summary

Planning is complete for the canonical prerequisite discovered during
trusted-host local application admission implementation. The plan defines an
explicit zero-required-context posture that is distinguishable from missing,
unknown, incomplete, or caller-supplied empty data.

## 2. Scope Completed

- Inspected the canonical required-context construction, validation,
  deserialization, consumption, and current-authority resolution boundaries.
- Defined a compatibility-conscious explicit-zero contract posture.
- Defined private closed docs-check issuance.
- Defined hashing, serde, consumption, privacy, and registered-authority
  requirements.
- Defined focused and regression tests.
- Sequenced review and prerequisite implementation before admission resumes.

## 3. Scope Explicitly Not Completed

- No Rust implementation.
- No trusted-host binary or Core admission path.
- No public empty-contract constructor.
- No provider access, writes, CLI, SDK, schema, example, hosted runtime,
  OpenShell, nested harness, or release posture change.

## 4. Key Decision

Zero context must be an explicit, hash-bound contract posture. It must never be
inferred from absence or an arbitrary empty vector. Existing public non-empty
construction remains fail closed for empty requirements.

## 5. Compatibility Posture

Legacy non-empty contracts should retain their wire and hash behavior where
possible. Explicit zero must use an unambiguous marker and a domain-separated
hash. Any required compatibility break must return to review before coding.

## 6. Validation Boundary

- `DeclaredRequirements` requires at least one canonical requirement.
- `ExplicitNone` requires exactly zero requirements.
- Empty legacy data without an explicit marker remains invalid.
- Zero projections are accepted only for `ExplicitNone`.
- Any projection supplied to `ExplicitNone` is invalid.

## 7. Governed Phase Record

- workflow: `dg/d`
- run: `run-1791391563788979000-2`
- approval: `approval/run-1791391563788979000-2/planning-approved`
- presentation: `presentation/a156d4bb42f3ecfe`
- presentation hash:
  `a156d4bb42f3ecfef5cd9b9a5ab5b2216124065a42bf0798d6ceffd412cc20f3`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: documentation-only prerequisite planning
- phase status: completed
- out-of-kernel work: source inspection, plan authoring, documentation
  validation, and later git or pull-request work

## 8. Recommended Next Phase

Focused maintainer/security review of the explicit zero required-context plan.
Trusted-host local application admission remains blocked until the prerequisite
model is implemented and reviewed.
