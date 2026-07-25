# Authoritative Local-Check Reassessment Binding Plan Blocker Fix Report

## 1. Executive Summary

The two planning blockers found in the authoritative local-check reassessment
binding plan are corrected.

The plan now requires:

1. complete deterministic wrapper preflight before any clock or local process
   use; and
2. one private bound-assessment value that makes local-check fact identity
   inseparable from reassessment authority.

No Rust implementation or runtime behavior was added.

## 2. Original Blockers

The first plan could be read to invoke local-check composition before proving
that runtime-fact shape, selected-step ambiguity, and immutable reassessment
context were valid.

It also proposed returning a raw assessment set and a separate binding
fingerprint. A future crate-internal caller could ignore that fingerprint and
consume the unbound assessment set.

## 3. Corrected Preflight Boundary

The plan now requires a pure preflight before local-check composition. It
validates:

- exact stored bundle and selected step;
- canonical selected-step declarations;
- immutable workflow, skill, and policy resolution;
- exact runtime-fact count and membership;
- duplicate, missing, extra, and mismatched runtime facts; and
- absence of caller-selected evidence/check posture for the selected step.

Only after preflight succeeds may the existing local-check helper perform its
own full requirement/command preflight and process execution.

## 4. Corrected Authority Surface

The plan now defines:

- `AuthoritativeLocalCheckBoundAssessment`, which privately owns the
  authoritative fact, complete assessment set, and binding fingerprint; and
- `AuthoritativeLocalCheckReassessmentOutcome`, which owns bounded results and
  that bound value.

The raw assessment set is not exposed as independently reusable authority.
Future consumers must accept the bound value or a separately reviewed durable
projection derived from it.

## 5. Test Corrections

The future implementation must prove:

- every deterministic wrapper mismatch fails before clock or process use;
- the selected-step caller posture cannot enter the authoritative path;
- the bound value retains fact and assessment identity;
- no intended accessor returns a raw unbound assessment set as authority; and
- all original invalidation, monotonicity, failure, and privacy tests.

## 6. Governed Fix Record

- workflow: `dg/blocker`
- run: `run-1785019863029372000-2`
- approval: `approval/run-1785019863029372000-2/fix-approved`
- presentation: `presentation/f6f5969e5cfa3cad`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- out-of-kernel work: planning-document correction, fix-report authoring, and
  documentation validation
- missing coverage: the kernel coordinated governance only; no implementation
  or WorkReport artifact was created

## 7. Validation

Completed successfully:

- `npm run check:docs`; and
- `git diff --check`.

## 8. Remaining Limitations

- The corrected plan is not implemented.
- Focused re-review remains required.
- No executor or durable consumer exists.
- No automatic checks or runtime quiet-success behavior was added.

## 9. Recommended Next Phase

Perform focused re-review of the two corrected planning boundaries.

Only after acceptance should the private implementation phase begin.
