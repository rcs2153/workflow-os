# Authoritative Local-Check Reassessment Binding Plan Report

## 1. Executive Summary

Workflow OS now has an accepted private path from canonical local-check
declarations through observed checks and exact coverage to a
provenance-bearing aggregate fact.

This planning phase defines the next boundary: one private same-call wrapper
that carries the complete aggregate fact and its fingerprint into immutable-
bundle proportional-governance reassessment. It remains planning only.

## 2. Scope Completed

- Inspected the accepted local-check composition, aggregate fact, runtime-fact,
  immutable-bundle reassessment, and assessment-set fingerprint boundaries.
- Identified that copying only `fact.posture()` would lose authoritative fact
  identity.
- Defined a private same-call composition boundary that invokes the accepted
  local-check helper itself.
- Defined selected-step runtime-fact injection and ambiguity rejection.
- Defined a versioned fact-to-assessment binding fingerprint.
- Defined monotonicity, freshness, failure, privacy, test, and review
  requirements.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- Rust model or helper code;
- executor integration or automatic checks;
- runtime quiet-success activation;
- persistence, events, evidence, reports, or artifacts;
- schemas, CLI, UI, onboarding, or examples;
- providers, OpenShell, SideEffects, or writes;
- hosted behavior; or
- release changes.

## 4. Key Architecture Decision

The first implementation should not accept a detached aggregate fact or
posture. It should invoke authoritative local-check composition inside the
reassessment wrapper, then:

1. reject selected-step caller evidence/check posture;
2. inject the Core-derived aggregate posture;
3. run existing immutable-bundle reassessment; and
4. bind local-check fact identity to selected and complete assessment identity.

This is stronger than a convenience adapter and preserves the current same-
call freshness posture.

## 5. Product Relationship

Fresh-pull evaluation recommends reducing ceremony for eligible low-risk work
while preserving evidence. This plan advances that goal without activating
quiet success prematurely.

Workflow OS will be able to distinguish:

- a caller asserting that checks passed; and
- Core deriving complete check posture from immutable declarations and observed
  results.

Only the second path is eligible for future authoritative runtime consumption.

## 6. Validation

Completed successfully:

- `npm run check:docs`; and
- `git diff --check`.

Source inspection covered the current local-check fact, same-call composition,
runtime-fact, immutable-bundle reassessment, assessment-set fingerprint, and
executor consumption boundaries.

## 7. Governed Planning Record

- workflow: `dg/d`
- run: `run-1785019583344911000-2`
- approval:
  `approval/run-1785019583344911000-2/planning-approved`
- presentation: `presentation/31cba9eb462ce513`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- out-of-kernel work: source inspection, plan authoring, roadmap updates, and
  documentation validation
- missing coverage: the kernel coordinated governance only; it did not perform
  architecture analysis, author documentation, or generate a WorkReport
  artifact

## 8. Remaining Limitations

- No reassessment binding is implemented.
- No executor consumes authoritative local-check posture.
- Runtime facts for other steps remain under the existing explicit contract.
- The same-call path is not durable freshness or replay protection.
- Quiet success remains limited to separately reviewed existing paths.

## 9. Recommended Next Phase

Perform a phase-level maintainer review of the
[Authoritative Local-Check Reassessment Binding Plan](../implementation-plans/authoritative-local-check-reassessment-binding-plan.md).

Do not begin implementation until that review accepts the identity,
monotonicity, freshness, and non-scope boundaries.

Phase-level review found two focused planning blockers. Both are corrected in
the
[blocker fix report](AUTHORITATIVE_LOCAL_CHECK_REASSESSMENT_BINDING_PLAN_BLOCKER_FIX_REPORT.md);
focused re-review accepted both corrected boundaries in the
[blocker fix review](AUTHORITATIVE_LOCAL_CHECK_REASSESSMENT_BINDING_PLAN_BLOCKER_FIX_REVIEW.md).
The next phase is the private implementation only.
