# Authoritative Local-Check Aggregate Posture Conversion Blocker Fix Report

## 1. Executive Summary

The aggregate-fact identity test blocker is fixed.

Workflow OS now directly proves that semantic posture and count changes,
candidate-set identity changes, structural-coverage identity changes, and each
individual bounded fingerprint input invalidate the v1 aggregate fact
fingerprint.

The fix changes tests and documentation only. Production conversion semantics,
runtime behavior, and public surfaces are unchanged.

## 2. Blocker Fixed

Phase-level review found that the implementation visibly bound all required
fields into the fingerprint but did not directly test their invalidation
behavior.

That gap mattered because future proportional-governance reassessment will
depend on this fingerprint as the provenance commitment for authoritative
check posture.

## 3. Fix Approach

The focused regression uses one internally consistent canonical baseline and
proves that valid converted variants change identity when:

- aggregate posture changes;
- aggregate counts change;
- candidate-set fingerprint changes; or
- structural-coverage fingerprint changes.

The same test then exercises the private fingerprint function with one-field
mutations to prove independent binding of:

- expected count;
- satisfied count;
- failed count;
- required-unavailable count;
- optional-unavailable count;
- missing count;
- candidate-set fingerprint;
- structural-coverage fingerprint; and
- mapped posture.

Algorithm identity remains pinned by the versioned enum, domain separator, and
stable known vector. A future algorithm variant must add cross-version
invalidation coverage when introduced.

## 4. Scope And Semantics

The fix does not change:

- canonical source requirements;
- disposition-to-posture mapping;
- consistency validation;
- fact fields or fingerprint construction;
- executor behavior;
- proportional-governance behavior;
- public APIs, schemas, CLI, persistence, events, or reports; or
- providers, SideEffects, writes, hosted behavior, or release posture.

## 5. Privacy Posture

The tests use bounded synthetic hashes. They do not add raw commands, paths,
output, source contents, provider data, environment values, credentials, or
natural-language payloads.

Production `Debug` and error behavior remains redaction-safe and unchanged.

## 6. Test Coverage

Added:

- `aggregate_fact_identity_binds_every_decision_relevant_input`.

Focused results:

- aggregate fact identity tests: passed, 2 tests;
- complete structural-coverage module: passed, 22 tests;
- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed, with explicitly opt-in live tests ignored;
- `npm run check:docs`: passed; and
- `git diff --check`: passed.

## 7. Governed Fix Record

- workflow: `dg/blocker`
- run: `run-1785010771472343000-2`
- approval: `approval/run-1785010771472343000-2/fix-approved`
- presentation: `presentation/9102db11c7695c38`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: completed with final validation passed
- out-of-kernel work: test authoring, documentation, and validation
- missing coverage: the kernel governed the phase but did not author files,
  execute checks, or generate a WorkReport artifact

## 8. Recommended Next Phase

Focused blocker-fix re-review accepts the fix. See the
[blocker fix review](AUTHORITATIVE_LOCAL_CHECK_AGGREGATE_POSTURE_CONVERSION_BLOCKER_FIX_REVIEW.md).

Proceed to same-call authoritative local-check composition planning only.
