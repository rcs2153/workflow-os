# Evidence And Check Obligation-Set Aggregation Plan Blocker Fix Report

## 1. Executive Summary

The planning blockers are corrected. The first implementation now produces
only a non-authoritative structural coverage candidate for local-check
attestation obligations.

It cannot create or convert to aggregate proportional-governance workload
posture.

## 2. Blockers Fixed

- Split structural exactness from authoritative aggregate posture.
- Narrowed v1 to the accepted local-check attestation family.
- Removed speculative generic obligation-kind vocabulary from v1.
- Defined unresolved declaration provenance as non-authoritative.
- Defined empty-set, optional-failure, and private leaf-adaptation semantics.

## 3. Authority Boundary

An explicitly supplied candidate set can prove only that contributions cover
that candidate set. The v1 structural result exposes no conversion to
`GovernanceWorkloadEvidenceCheckPosture`.

A future authoritative adapter requires canonical declarations frozen into the
immutable run bundle and a separately reviewed mapping.

## 4. Semantics Clarified

- A canonical authoritative empty set may eventually be vacuously satisfied.
- An absent, caller-asserted, or unresolved set is not that authoritative empty
  set and cannot produce satisfaction.
- Optional absence may become `OptionalUnavailable`.
- An optional check that executes and fails remains `Failed`.
- The current `DocsCheck` contribution is adapted privately in the same call
  stack and is not serialized or recreated.

## 5. Scope Explicitly Not Completed

- no model or code implementation;
- no schema or immutable declaration derivation;
- no authoritative aggregate posture;
- no reassessment or executor integration;
- no persistence, events, reports, artifacts, CLI, UI, providers, SideEffects,
  or writes.

## 6. Governed Fix

- workflow: `dg/blocker`
- run: `run-1784959651908773000-2`
- approval: `approval/run-1784959651908773000-2/fix-approved`
- presentation: `presentation/53a35c18a44386e4`
- approval outcome: granted by delegated maintainer through proof enforcement
- kernel boundary: governance coordination only; documentation and validation
  ran outside the kernel

## 7. Validation

- `npm run check:docs` - passed;
- `git diff --check` - passed.

## 8. Recommended Next Phase

Focused re-review accepts the correction in the
[Evidence And Check Obligation-Set Aggregation Plan Blocker Fix Review](EVIDENCE_CHECK_OBLIGATION_SET_AGGREGATION_PLAN_BLOCKER_FIX_REVIEW.md).

Implement only the private local-check candidate model and structural
evaluator.
