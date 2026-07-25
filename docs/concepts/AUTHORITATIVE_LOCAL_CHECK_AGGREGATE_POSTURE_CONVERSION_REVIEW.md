# Authoritative Local-Check Aggregate Posture Conversion Review

## 1. Executive Verdict

**Needs one focused blocker fix.**

The private model, conversion semantics, source authority, consistency checks,
privacy posture, and scope boundary are sound. The blocker is missing direct
behavioral proof that each decision-relevant input invalidates the aggregate
fact fingerprint.

## 2. Scope Verification

The phase stayed within the approved private, pure, unwired scope.

It did not add:

- local-check execution or handler defaults;
- executor or approval-resume integration;
- proportional-governance selector invocation;
- public exports or serialization;
- schemas, CLI behavior, onboarding, or examples;
- persistence, events, reports, artifacts, providers, SideEffects, or writes;
  or
- hosted behavior or release-posture changes.

## 3. Authority Boundary Assessment

The converter accepts only structural coverage whose source posture is
`CanonicalStoredBundle`. Unresolved caller-created candidates fail before a
fact is constructed, including unresolved empty sets.

Production code can obtain that source posture only through the private stored
immutable-bundle adapter. The mapped posture is derived internally from the
accepted structural disposition; no caller can supply it independently.

This boundary is accepted.

## 4. Mapping Assessment

The exact mapping is correct:

- `Satisfied` to `Satisfied`;
- `OptionalUnavailable` to `OptionalUnavailable`;
- `RequiredUnavailable` to `RequiredUnavailable`; and
- `Failed` to `Failed`.

An optional check that executes and fails remains `Failed`. A canonical empty
declaration set may convert to `Satisfied`; missing and unresolved sources may
not.

The converter does not manufacture `Unknown`. A future consumer must represent
absence of an authoritative fact explicitly.

This boundary is accepted.

## 5. Consistency And Failure Assessment

The converter defensively verifies:

- checked terminal-count addition;
- exact expected-count equality;
- bounded missing coverage;
- strictest-outcome disposition consistency; and
- canonical source posture.

Errors use stable
`local_check_attestation.aggregate_posture.*` codes with static messages. They
do not expose identities, fingerprints, paths, commands, output, or secret-like
test values.

This boundary is accepted.

## 6. Identity Assessment

The v1 fact fingerprint binds:

- versioned algorithm identity;
- explicit `canonical_local_checks` scope;
- mapped posture;
- every bounded coverage count;
- candidate-set fingerprint; and
- structural-coverage fingerprint.

Fixed-width framing and a stable known vector are present.

The implementation shape is correct. However, the focused tests prove only
same-input determinism, the known vector, and generic framing. They do not
directly prove that changing posture, counts, candidate identity, or structural
coverage changes the aggregate fact fingerprint.

Because this fingerprint is intended to become the provenance commitment
consumed by proportional-governance reassessment, that missing behavioral
proof is a blocker.

## 7. Privacy Assessment

The fact stores bounded posture, counts, and opaque fingerprints only.
`Debug` redacts candidate, coverage, and fact fingerprints. No raw command,
path, output, source, provider, environment, credential, or natural-language
payload is retained.

This boundary is accepted.

## 8. Quiet-Success Assessment

The implementation does not equate satisfied checks with quiet execution. It
does not call the proportional-governance selector or suppress evidence,
audit, disclosure, approval, or report obligations.

This is the correct product boundary. The current user review supports the
direction: reduce low-risk ceremony only through authoritative facts while
preserving the evidence trail.

## 9. Test Assessment

Passing coverage includes:

- all four posture mappings;
- optional executed failure;
- canonical versus unresolved empty sets;
- unresolved populated source rejection;
- contradictory count, missing, and disposition rejection;
- deterministic identity and a known vector;
- fixed-width framing; and
- `Debug` and error non-leakage.

The full workspace test suite, strict clippy, formatting, docs, and diff checks
pass.

Missing blocker coverage:

- posture invalidation;
- count invalidation;
- candidate-set fingerprint invalidation; and
- structural-coverage fingerprint invalidation.

Algorithm identity is pinned by the known vector and versioned enum. A future
algorithm variant must add its own known vector and cross-version invalidation
test when introduced.

## 10. Blocker

Add one focused deterministic test matrix that constructs internally
consistent canonical coverage variants and proves that each currently
changeable decision-relevant input changes the aggregate fact fingerprint.

The test must not weaken production privacy or expose fingerprints through
`Debug` or errors.

## 11. Non-Blocking Follow-Ups

- Keep the fact crate-private through first runtime composition.
- Bind the fact fingerprint, not only the mapped posture, in future
  reassessment.
- Require a new reviewed algorithm when another evidence/check family enters
  the authoritative universe.
- Preserve `Unknown` as explicit consumer behavior when no fact exists.

## 12. Governed Review Record

- workflow: `dg/review`
- run: `run-1785010629287782000-2`
- approval:
  `approval/run-1785010629287782000-2/review-scope-approved`
- presentation: `presentation/289645e5a9f31c87`
- approval outcome: granted by delegated maintainer through proof enforcement
- review status: completed with one blocker
- validation: formatting, strict clippy, workspace tests, docs check, and diff
  check passed before review
- out-of-kernel work: source inspection, test inspection, review authoring,
  documentation correction, and validation
- missing coverage: the kernel coordinated governance only; it did not perform
  maintainer analysis, execute checks, or generate a WorkReport artifact

## 13. Recommended Next Phase

Run a focused blocker-fix phase for aggregate-fact fingerprint invalidation
tests, then re-review.

Do not begin runtime composition, executor integration, or quiet-success
enforcement before that re-review accepts the fact identity boundary.

## 14. Fix-Forward Status

The focused invalidation regression is implemented, all required validation
passes, and re-review accepts the fix in the
[Authoritative Local-Check Aggregate Posture Conversion Blocker Fix Review](AUTHORITATIVE_LOCAL_CHECK_AGGREGATE_POSTURE_CONVERSION_BLOCKER_FIX_REVIEW.md).

The original blocker finding remains preserved above.
