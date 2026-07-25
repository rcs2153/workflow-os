# Evidence And Check Obligation-Set Aggregation Plan Report

## 1. Executive Summary

Planning now defines the missing complete-coverage boundary between one
verified evidence/check contribution and the aggregate workload fact consumed
by proportional governance.

The plan is model-first and fails closed on the current lack of authoritative
canonical obligation declarations.

## 2. Scope Completed

- Defined the source-of-truth boundary.
- Proposed the smallest private obligation-set model.
- Defined immutable set and obligation identity.
- Defined required and optional requirement levels.
- Defined deterministic complete-coverage aggregation.
- Defined bounded aggregate output and privacy posture.
- Defined configuration, onboarding, invalidation, and sequencing boundaries.
- Defined future tests and implementation phases.

## 3. Key Decision

Safe repository metadata and model inference may recommend likely obligations,
but only validated declarations accepted through governance and frozen into the
immutable run bundle may become authoritative enforcement inputs.

A caller-supplied list, count, completeness flag, or opaque hash is not proof
of complete coverage.

## 4. Aggregate Safety

Aggregate `Satisfied` requires exact coverage of every required obligation.
Missing, duplicate, unexpected, mismatched, stale, unsupported, or ambiguous
coverage fails closed or preserves the appropriate unavailable/failed posture.

The first model implementation remains unwired because current schemas do not
provide a canonical complete declaration set.

## 5. Product Boundary

Onboarding should derive and present concrete recommended obligations for most
repositories. Recommendations do not become enforcement until governed
acceptance and canonical declaration.

Execution disposition and disclosure obligation remain independent. Visible
operator presentation is not a separate source of execution authority.

## 6. Scope Explicitly Not Completed

- no model or aggregator implementation;
- no schema or YAML changes;
- no immutable-bundle derivation;
- no proportional-governance reassessment;
- no executor checkpoint or automatic check;
- no persistence, events, evidence, reports, artifacts, CLI, UI, or examples;
- no provider calls, SideEffects, writes, hosted behavior, or release changes.

## 7. Governed Planning

- workflow: `dg/d`
- run: `run-1784959209868516000-2`
- approval: `approval/run-1784959209868516000-2/planning-approved`
- presentation: `presentation/b8e84a2023e1927d`
- approval outcome: granted by delegated maintainer through proof enforcement
- kernel boundary: governance coordination only; analysis, documentation, and
  validation ran outside the kernel

## 8. Validation

- `npm run check:docs` - passed;
- `git diff --check` - passed.

## 9. Recommended Next Phase

Focused review found planning blockers in source authority and v1 obligation
scope. See the
[Evidence And Check Obligation-Set Aggregation Plan Review](EVIDENCE_CHECK_OBLIGATION_SET_AGGREGATION_PLAN_REVIEW.md).

The focused correction is documented in the
[Evidence And Check Obligation-Set Aggregation Plan Blocker Fix Report](EVIDENCE_CHECK_OBLIGATION_SET_AGGREGATION_PLAN_BLOCKER_FIX_REPORT.md).

Perform focused re-review before implementing any model.
