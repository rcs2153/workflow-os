# DocsCheck Attestation Proportional-Governance Integration Plan Review

## 1. Executive Verdict

Needs planning blocker fixes.

## 2. Scope Verification

The plan stays within planning-only scope. It does not authorize Rust
implementation, executor integration, automatic checks, persistence, events,
evidence records, runtime reports, artifacts, schemas, CLI behavior, provider
operations, SideEffects, writes, hosted behavior, or release changes.

## 3. Mapping Assessment

The leaf mapping is semantically sound:

- satisfied gate to `Satisfied`;
- unaccepted deterministic result to `Failed`; and
- stale required proof to `RequiredUnavailable`.

The distinction between check failure and unavailable current proof is useful
and aligns with the existing selector. Both fail closed. Execution disposition
and disclosure presentation also remain correctly independent.

## 4. Aggregate-Fact Blocker

The proposed helper would replace the complete
`ProportionalGovernanceWorkloadAssessmentInput.evidence_and_checks` fact with
the posture of one `DocsCheck` gate.

That field is an aggregate workload fact. It may summarize more than one
evidence or deterministic-check obligation. A satisfied `DocsCheck` proves only
its exact independently verified requirement and invocation. It cannot prove
that every other evidence or check obligation is satisfied.

Under the proposed replacement behavior, a caller could provide an assessment
whose aggregate posture is `Failed`, `RequiredUnavailable`, or `Unknown` for
another obligation and have one satisfied DocsCheck replace it with
`Satisfied`. The existing selector would then legitimately lower check-driven
strictness because it was given an incorrectly broadened aggregate fact.

This is a blocker because it could turn partial proof into aggregate
satisfaction.

## 5. Required Planning Correction

The corrected plan must introduce an explicit aggregation boundary before
reassessment. It must not infer aggregate satisfaction from one leaf gate.

The smallest safe correction should:

1. map the gate to a typed evidence/check contribution scoped to the exact
   requirement;
2. combine that contribution with the posture of every other required evidence
   and check obligation through a deterministic fail-closed aggregator;
3. require complete obligation coverage before aggregate `Satisfied` is
   possible;
4. treat missing, duplicate, ambiguous, or unsupported obligation coverage as
   `RequiredUnavailable` or a stable internal error, never satisfaction;
5. derive obligation identity and expected coverage from validated immutable
   context rather than caller prose or a free-form count; and
6. only then replace the aggregate assessment fact and invoke the existing
   workload selector.

If the current models cannot establish complete obligation coverage, the first
implementation must stop at a leaf contribution mapper. It must not claim to
be a proportional-governance reassessment consumer.

## 6. Semantic Preservation Assessment

Apart from the aggregate-fact blocker, the plan correctly preserves profile,
workflow, policy, authority, sensitivity, SideEffect, runtime escalation,
prior decision, and steward minima. It also correctly relies on the existing
immutable definition root and assessment fingerprint for invalidation.

The correction must retain those properties and must define how the aggregate
evidence/check contribution participates in fingerprint invalidation.

## 7. Privacy And Compatibility Assessment

The planned boundary remains payload-free and crate-private. It introduces no
raw output, paths, environment values, source content, credentials, provider
payloads, public serde, or compatibility change. The blocker fix must preserve
that posture.

## 8. Test Plan Assessment

The proposed tests cover leaf mapping, monotonicity, invalidation, and privacy,
but they do not cover partial-proof substitution across multiple obligations.

The corrected plan must add tests for:

- one satisfied DocsCheck plus another failed obligation cannot satisfy;
- one satisfied DocsCheck plus another unavailable obligation cannot satisfy;
- incomplete expected-obligation coverage cannot satisfy;
- duplicate or mismatched requirement identity cannot satisfy;
- aggregation order does not change the result; and
- only complete satisfied coverage can produce aggregate `Satisfied`.

## 9. Planning Blockers

1. One leaf check outcome currently overwrites an aggregate evidence/check
   fact without proving complete obligation coverage.

## 10. Non-Blocking Follow-Ups

- Persisted or asynchronous contribution aggregation will require one-time
  claim and replay semantics.
- Handler implementation provenance remains registered-unattested.
- A future UI may render disclosure live without changing execution authority.

## 11. Validation

- Inspected the gate implementation and visibility boundary.
- Inspected workload assessment input, evidence/check mapping, monotonic
  selector behavior, and fingerprint tests.
- `npm run check:docs` - passed.
- `git diff --check` - passed.
- governed phase status - `Completed` with 39 events, one approval, zero
  retries, and zero escalations.

Phase close hit the known 250-record approval-presentation reader cap and
reported `proof_record_read_error`; the approval itself was granted through
persisted presentation-proof enforcement.

## 12. Governed Review

- workflow: `dg/review`
- run: `run-1784954916974157000-2`
- approval: `approval/run-1784954916974157000-2/review-scope-approved`
- presentation: `presentation/d1c90fbcf804b59f`
- approval outcome: granted by delegated maintainer through proof enforcement
- kernel boundary: governance coordination only; review and validation ran
  outside the kernel

## 13. Recommended Next Phase

Perform a focused planning blocker fix for evidence/check obligation
aggregation and complete-coverage semantics. Do not implement the adapter until
that correction is reviewed and accepted.
