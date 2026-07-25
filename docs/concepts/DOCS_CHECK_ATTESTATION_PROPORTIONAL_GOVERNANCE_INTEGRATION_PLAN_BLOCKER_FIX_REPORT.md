# DocsCheck Attestation Proportional-Governance Integration Plan Blocker Fix Report

## 1. Executive Summary

The plan no longer lets one leaf `DocsCheck` overwrite the aggregate
evidence/check workload fact. The corrected first implementation stops at an
exact requirement-scoped contribution. Aggregate reassessment remains blocked
until an authoritative complete obligation set and fail-closed aggregator
exist.

## 2. Blocker Fixed

The original plan could map one satisfied check directly to aggregate
`Satisfied`, erasing another failed, unavailable, or unknown obligation. The
correction removes that replacement behavior and its claimed reassessment
consumer.

## 3. Corrected Boundary

The first future helper will execute the same-call gate and return one private
contribution bound to a Core-derived fingerprint over exact immutable run,
step, and requirement identity. The contribution carries only its leaf posture.
It grants no aggregate authority and invokes no selector.

## 4. Complete-Coverage Requirement

Later reassessment requires an authoritative exact set of all evidence/check
obligations for the immutable step. Missing, duplicate, unexpected,
mismatched, ambiguous, or unsupported coverage must fail closed. Only complete
satisfied coverage may become aggregate `Satisfied`.

Current schemas and runtime models do not provide that complete set. The plan
does not fabricate it.

## 5. Privacy And Compatibility

The correction remains private, payload-free, in-memory, and non-serializing.
It adds no runtime behavior, state, events, schemas, CLI, providers,
SideEffects, writes, hosted behavior, or release change.

## 6. Test Posture

Corrected tests cover exact contribution mapping, identity substitution, no
imported outcomes, determinism, privacy, and absence of aggregate assessment.
Future aggregation tests must cover partial and duplicate coverage.

## 7. Governed Phase

- workflow: `dg/blocker`
- run: `run-1784955155878644000-2`
- approval: `approval/run-1784955155878644000-2/fix-approved`
- presentation: `presentation/635095689626cfa6`
- approval outcome: granted by delegated maintainer through proof enforcement
- kernel boundary: governance coordination only; documentation and validation
  ran outside the kernel

## 8. Validation

- `npm run check:docs` - passed.
- `git diff --check` - passed.

## 9. Remaining Limitations

- no contribution wrapper is implemented;
- no authoritative aggregate obligation set exists;
- no aggregate reassessment is authorized;
- no executor checkpoint exists; and
- handler implementation provenance remains registered-unattested.

## 10. Recommended Next Phase

Focused re-review accepts the correction with a dedicated private leaf-posture
type. Implement only the requirement-scoped contribution wrapper. Plan
complete-coverage aggregation separately before proportional-governance
reassessment.
