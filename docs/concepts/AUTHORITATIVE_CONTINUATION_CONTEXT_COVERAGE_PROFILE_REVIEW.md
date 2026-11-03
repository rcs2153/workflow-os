# Authoritative Continuation Context Coverage Profile Review

## Decision

Accepted after one fix-forward correction.

The implementation closes the reviewed collector-expectation substitution
path without adding source collection, persistence, runtime behavior, public
surfaces, or execution authority. It remains a private deterministic model.

## Review Finding And Correction

The initial implementation compared every caller-carried expectation with the
Core-owned profile, but it did not reject every expectation/outcome semantic
contradiction. A caller could leave an optional expectation unchanged while
reporting its outcome as `Inapplicable`, or report a required class as
`SkippedOptional`. This did not make a required collector complete, but it
could blur the trusted matrix and suppress accurate degraded posture.

The correction adds a stable payload-free compatibility check before source
and outcome evaluation:

- required collection rejects `SkippedOptional` and `Inapplicable`;
- optional collection rejects `Inapplicable`; and
- an inapplicable class requires the exact `Inapplicable` outcome.

The correction returns
`continuation_context.coverage.outcome_expectation_mismatch` without exposing a
source class, identity, revision, value, path, or provider detail. Two focused
regressions cover required/optional relabeling and inapplicable-class outcome
substitution.

## Accepted Properties

- Scope shape is validated before profile derivation.
- Profile posture follows exact run binding and cannot be caller selected.
- All six purposes have distinct pre-run and run-bound profile identities.
- Every closed source class receives exactly one Core-owned expectation.
- Caller expectation substitution fails before outcome evaluation.
- Caller outcome relabeling cannot contradict the Core-owned expectation.
- Required explicit empty remains complete and distinguishable from failure,
  denial, unsupported collection, or omission.
- Optional collector failure remains visible as degraded without blocking
  completeness.
- Provider observation is required for run-bound provider reconciliation and
  optional for the other reviewed postures.
- Profile identity, run-binding posture, expectations, and outcomes are bound
  into the coverage commitment.
- Coverage input order does not change the decision or commitment.
- Errors and debug output remain bounded and non-leaking.
- The result remains non-authorizing.

## Remaining P0 Boundary

This acceptance does not make the continuation decision authoritative for an
operational consumer. The caller still supplies:

- `required_claims`; and
- each source definition's `required_for_purpose` posture.

Core must own those semantic obligations next. Until then, a caller could omit
a purpose-required claim or mark a supplied source non-required even though it
cannot weaken collector classification. No collector, projection, compaction
adapter, runtime consumer, nested harness, or provider mutation may rely on
this model as complete authoritative understanding.

Required explicit empty also proves only that collection accounted for a
class. It does not prove a purpose-required semantic value exists. The next
profile must make such claims required and preserve unknown or conflicting
posture as blocking.

## Validation

- focused continuation-context tests: 31 passed
- strict `workflow-core` clippy: passed with warnings denied and the documented
  narrow pre-existing `dead_code` allowance
- full workspace tests: passed after the fix-forward change
- rustdoc with warnings denied: passed after the fix-forward change
- formatting: passed
- documentation checks: passed
- diff checks: passed

## Governed Review

- workflow: `dg/review`
- run ID: `run-1793697794519572000-2`
- approval ID:
  `approval/run-1793697794519572000-2/review-scope-approved`
- presentation ID: `presentation/70336a1983974adc`
- approval outcome: granted under delegated-maintainer authority after review
  of the complete persisted handoff

## Out-Of-Kernel Work

The delegated maintainer inspected the implementation and tests, identified
and corrected one outcome-relabeling defect, reran focused and broad
validation, updated roadmap/report posture, and authored this review outside
the kernel. The kernel governed scope and approval but did not edit files, run
validation, create commits, or mutate GitHub.
