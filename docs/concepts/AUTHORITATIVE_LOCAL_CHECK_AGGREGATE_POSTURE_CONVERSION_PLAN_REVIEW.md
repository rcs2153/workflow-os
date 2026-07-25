# Authoritative Local-Check Aggregate Posture Conversion Plan Review

## 1. Executive Verdict

**Plan accepted; proceed to the private aggregate-fact model and pure
conversion helper.**

The plan defines the smallest safe bridge from accepted canonical local-check
structural coverage to the existing proportional-governance evidence/check
vocabulary. It does not authorize runtime consumption or quiet-success
enforcement.

## 2. Scope Verification

The plan stays within planning scope. It does not authorize:

- check execution or handler defaults;
- executor checkpoints or automatic runtime integration;
- proportional-governance selector invocation;
- workflow or policy schema changes;
- CLI, UI, onboarding, or example behavior;
- persistence, events, evidence, reports, or artifacts;
- providers, OpenShell, SideEffect execution, or writes; or
- automatic approvals or release changes.

The recommended implementation is private, pure, non-serialized, and unwired.

## 3. Authority Boundary Assessment

The plan accepts only
`LocalCheckGovernanceStructuralCoverageCandidate` with
`CanonicalStoredBundle` source posture. Unresolved caller-supplied candidates
cannot convert, including unresolved empty sets.

That source restriction is appropriate because canonical candidates are
derived from validated stored immutable-run-bundle declaration records, not
mutable project files, inferred commands, repository metadata, or caller
counts.

The plan also requires the canonical candidate-set and structural-coverage
fingerprints to remain in the converted fact. This prevents a later consumer
from treating a caller-selected posture enum as equivalent authority.

## 4. Aggregate Semantics Assessment

The exact mapping preserves the accepted structural precedence:

- `Satisfied` maps to `Satisfied`;
- `OptionalUnavailable` maps to `OptionalUnavailable`;
- `RequiredUnavailable` maps to `RequiredUnavailable`; and
- `Failed` maps to `Failed`.

An executed optional failure remains failed. A canonical empty declaration set
may be satisfied, while a missing or unresolved declaration source cannot.
These are the correct semantics.

The converter does not manufacture `Unknown`. Unknown posture remains the
explicit consumer behavior when no authoritative fact exists.

## 5. Supported-Universe Assessment

The plan correctly narrows v1 authority to the complete canonical local-check
obligation universe currently supported by the kernel. It does not claim
arbitrary evidence generation or satisfaction of future obligation families.

This boundary matters because
`GovernanceWorkloadEvidenceCheckPosture` is semantically broader than the first
authoritative family. The plan handles that risk by:

- documenting the local-check-only v1 meaning;
- versioning the conversion algorithm;
- requiring a new reviewed aggregation algorithm when another authoritative
  family is added; and
- forbidding future runtime use of the mapped enum without its fact
  fingerprint.

No blocker remains, but implementation and later runtime reviews must continue
to enforce this wording.

## 6. Quiet-Success Assessment

The plan does not equate passing checks with quiet execution. It accurately
states that evidence/check posture is one selector input and that policy,
authority, sensitivity, SideEffect, profile, runtime-escalation, prior, and
steward minima may still require visibility, approval, or denial.

This matches the product direction confirmed by fresh-pull evaluation:
low-risk work should become less interruptive without weakening evidence,
audit, disclosure, or reporting.

## 7. Identity And Invalidation Assessment

The proposed fact fingerprint binds:

- algorithm identity;
- mapped posture;
- every bounded count;
- candidate-set fingerprint; and
- structural-coverage fingerprint.

Fixed-width framing, a known vector, and delimiter-collision tests are required.
That is sufficient for the private first phase and preserves deterministic
invalidation.

## 8. Validation And Error Assessment

The plan adds defense-in-depth consistency checks at the authority-changing
boundary, even though the current structural evaluator already constructs
consistent results.

Stable
`local_check_attestation.aggregate_posture.*` error codes and static messages
are appropriate. Errors and `Debug` are forbidden from exposing identities,
fingerprints, commands, paths, output, provider data, or credentials.

## 9. Test Assessment

The planned coverage is appropriately behavioral. It includes all four mapped
postures, canonical empty versus unresolved empty, optional executed failure,
caller-selection prevention, deterministic identity, known-vector framing,
privacy, and regression of the accepted contribution, coverage, adapter, and
proportional-governance layers.

No additional test is required before implementation.

## 10. Planning Blockers

None.

## 11. Non-Blocking Follow-Ups

- Keep the local-check-only v1 authority scope explicit anywhere the broader
  evidence/check enum appears.
- Do not expose the private fact publicly merely to make future executor
  integration convenient.
- During later runtime planning, require fact fingerprint binding and fresh
  same-call derivation rather than accepting the mapped enum alone.
- Treat another evidence/check obligation family as an aggregation-algorithm
  change, not an invisible extension of v1.

## 12. Governed Review Record

- workflow: `dg/review`
- run: `run-1785007259977707000-2`
- approval:
  `approval/run-1785007259977707000-2/review-scope-approved`
- presentation: `presentation/c24d62104c478844`
- approval outcome: granted by delegated maintainer through proof enforcement
- review status: completed
- event summary: 39 events, one approval, zero retries, zero escalations
- validation: `npm run check:docs` and `git diff --check` passed
- out-of-kernel work: source and plan inspection, review authoring,
  documentation validation, and diff validation
- missing coverage: the kernel coordinated governance only; it did not perform
  the maintainer analysis, author the review, or generate a WorkReport artifact

## 13. Recommended Next Phase

Implement the crate-private provenance-bearing aggregate fact and pure
authoritative conversion helper only.

Do not combine that phase with executor integration, automatic check execution,
schema exposure, proportional-governance invocation, or quiet-success
enforcement.
