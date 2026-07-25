# Authoritative Local-Check Aggregate Posture Conversion Report

## 1. Executive Summary

Workflow OS now has a crate-private, provenance-bearing fact that converts
complete canonical local-check structural coverage into the existing bounded
evidence/check posture vocabulary.

The conversion is pure and unwired. It does not execute checks, invoke
proportional governance, change executor behavior, suppress records, or make
quiet-success decisions. It creates one trustworthy input for a later runtime
composition phase.

This phase directly supports the current product priority: reduce ceremony for
low-risk work only when the kernel can derive authoritative check posture while
preserving evidence, audit, disclosure, and reporting.

## 2. Scope Completed

The phase added:

- `AuthoritativeLocalCheckEvidenceCheckFactAlgorithm`;
- `AuthoritativeLocalCheckEvidenceCheckFact`;
- `convert_authoritative_local_check_coverage`;
- defense-in-depth coverage consistency validation;
- a versioned, deterministic aggregate-fact fingerprint;
- stable, non-leaking aggregate-posture errors;
- redaction-safe `Debug`; and
- focused behavioral and privacy tests.

All additions remain crate-private in
`local_check_attestation::structural_coverage`.

## 3. Scope Explicitly Not Completed

The phase did not add:

- local-check execution or handler defaults;
- executor or approval-resume integration;
- proportional-governance selector invocation;
- automatic quiet success or visible disclosure;
- public APIs or serialization;
- schemas, CLI output, onboarding, or examples;
- persistence, events, evidence, reports, or artifacts;
- providers, OpenShell, SideEffect execution, or writes; or
- hosted behavior or release-posture changes.

## 4. Model And Helper Summary

The private fact retains:

- conversion algorithm identity;
- mapped `GovernanceWorkloadEvidenceCheckPosture`;
- expected, satisfied, failed, required-unavailable,
  optional-unavailable, and missing counts;
- canonical candidate-set fingerprint;
- structural-coverage fingerprint; and
- aggregate-fact fingerprint.

The helper accepts only
`LocalCheckGovernanceStructuralCoverageCandidate` values with
`CanonicalStoredBundle` source posture. Unresolved caller-created candidates
cannot convert, including unresolved empty candidates.

## 5. Mapping Summary

The exact conversion is:

| Canonical structural disposition | Aggregate evidence/check posture |
| --- | --- |
| `Satisfied` | `Satisfied` |
| `OptionalUnavailable` | `OptionalUnavailable` |
| `RequiredUnavailable` | `RequiredUnavailable` |
| `Failed` | `Failed` |

The helper does not manufacture `Unknown`. A later consumer must represent
absence of an authoritative fact explicitly.

A canonical stored empty declaration set remains valid `Satisfied` coverage.
A missing, legacy, or unresolved source is not equivalent and cannot convert.

## 6. Authority And Consistency Boundary

Conversion fails closed unless:

- the source is canonical stored-bundle coverage;
- terminal outcome counts sum to the expected count;
- missing coverage does not exceed unavailable coverage; and
- the disposition matches the strictest recorded outcome.

These checks repeat invariants already produced by the structural evaluator at
the authority-changing conversion boundary. Callers cannot supply the mapped
posture independently.

## 7. Fingerprint And Invalidation

The versioned fact fingerprint uses the repository's fixed-width field framing
and binds:

- algorithm;
- local-check-only authority scope;
- mapped posture;
- every bounded count;
- candidate-set fingerprint; and
- structural-coverage fingerprint.

Identical accepted facts are deterministic. Changes to posture, counts,
candidate identity, structural coverage, or algorithm invalidate the
fingerprint. A stable known vector and delimiter-framing regression are pinned
in tests.

Future runtime composition must bind this fact fingerprint. Passing only the
mapped posture enum would discard provenance and recreate the caller-assertion
gap this phase closes.

## 8. Quiet-Success Boundary

Authoritative satisfied checks do not independently authorize quiet execution.
Policy, authority, sensitivity, SideEffect posture, profile and steward
minimums, runtime escalation, and other unknown facts may still require visible
disclosure, approval, or denial.

Quiet work must still retain evidence, audit, disclosure, and report posture.
This phase changes none of those behaviors.

## 9. Privacy And Error Posture

The fact contains bounded counts, typed posture, and opaque commitments only.
It does not retain command text, paths, check output, source contents, provider
payloads, environment values, credentials, or natural-language summaries.

`Debug` redacts all fingerprints. Stable
`local_check_attestation.aggregate_posture.*` errors use static messages and do
not expose workflow, run, bundle, step, command, path, fingerprint, output, or
secret-like values.

## 10. Test Coverage

Focused tests cover:

- all four exact posture mappings;
- optional executed failure;
- canonical empty coverage;
- rejection of unresolved populated and empty candidates;
- prevention of source relabeling;
- fact count preservation;
- contradictory counts, missing counts, and dispositions;
- deterministic identity;
- posture, every bounded count, candidate, and structural-coverage
  invalidation;
- a stable known fingerprint vector;
- fixed-width framing; and
- `Debug` and error non-leakage.

The focused structural-coverage module suite passes with 22 tests. Phase-level
review found missing direct invalidation coverage; the focused blocker fix now
proves the identity boundary at both valid conversion and one-field hash-input
layers. See the
[blocker fix report](AUTHORITATIVE_LOCAL_CHECK_AGGREGATE_POSTURE_CONVERSION_BLOCKER_FIX_REPORT.md).

## 11. Validation Commands And Results

All required validation passed:

- focused structural-coverage module tests: passed, 21 tests;
- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed, with explicitly opt-in live tests ignored;
- `npm run check:docs`: passed; and
- `git diff --check`: passed.

## 12. Governed Implementation Record

- workflow: `dg/implement`
- run: `run-1785007769584004000-2`
- approval:
  `approval/run-1785007769584004000-2/implementation-approved`
- presentation: `presentation/d89bb02a949a2118`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: completed with repository validation passed
- out-of-kernel work: implementation, tests, documentation, and validation
- missing coverage: the kernel governed the phase but did not execute checks,
  author files, or generate a WorkReport artifact

## 13. Remaining Limitations

- The fact covers only the current complete canonical local-check obligation
  universe.
- It does not establish authenticity of check execution beyond the accepted
  attestation boundary.
- It is not consumed by the executor or proportional-governance selector.
- It is not persisted, serialized, emitted, or shown to users.
- Another evidence/check family requires a separately reviewed aggregation
  algorithm.

## 14. Recommended Next Phase

Focused re-review accepts the aggregate-fact fingerprint invalidation fix. See
the
[blocker fix review](AUTHORITATIVE_LOCAL_CHECK_AGGREGATE_POSTURE_CONVERSION_BLOCKER_FIX_REVIEW.md).

Proceed to same-call runtime composition planning that derives the fact from
the stored immutable bundle and fresh check contributions, binds its
fingerprint into proportional-governance reassessment, and does not trust a
caller-selected posture enum.
