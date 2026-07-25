# DocsCheck Attestation Runtime Composition Plan Blocker Fix Review

## 1. Executive Verdict

Planning blockers fixed; proceed to one explicit in-memory `DocsCheck`
attestation runtime-composition helper.

## 2. Scope Verification

The fix stayed within documentation-only planning scope. No Rust runtime,
process execution, executor default, persistence, event, evidence, report,
artifact, schema, CLI, provider, SideEffect, write, hosted, or release behavior
changed.

## 3. Clock Authority Assessment

The corrected plan removes all caller-supplied binding, observation, and
evaluation timestamps. One crate-private injected clock is owned by the helper
and sampled for binding creation, process start, process completion, and
verifier evaluation.

The runner cannot supply or override these facts. Clock errors and impossible
ordering fail closed. Scripted clock tests preserve determinism without making
the caller authoritative.

This resolves the first blocker.

## 4. Typed Eligibility Assessment

The corrected plan uses
`LocalCheckAttestationRequirement::accepted_statuses()` before invoking the
verifier.

- Ineligible typed status returns the honest structured result and no proof.
- Eligible typed status always invokes the verifier.
- Every verifier error propagates as an integrity failure.

No error-code or error-string matching is allowed. An integrity failure cannot
be downgraded to an ordinary check failure.

This resolves the second blocker.

## 5. Implementation Readiness

The plan now defines a complete bounded implementation contract:

- exact pre-execution ordering;
- one explicit `DocsCheck` command;
- Core-owned binding, timing, observation, candidate, and verification;
- honest passed/no-proof/internal-failure outcomes;
- no `SkillOutput` parsing;
- no change to empty default registration or executor behavior;
- bounded privacy and redaction rules; and
- focused ordering, authority, failure, and regression tests.

## 6. Blockers

None.

## 7. Non-Blocking Follow-Ups

- Decide during implementation whether the clock trait returns `Timestamp` or
  `Result<Timestamp, WorkflowOsError>`; explicit failure is preferred.
- Freshness must be reevaluated at later consumption boundaries.
- Stronger handler implementation provenance remains future work.
- Persistence, events, evidence/reports, and proportional-governance
  integration remain separately governed phases.
- The dogfood presentation-record close cap remains an open defect.

## 8. Validation

- `npm run check:docs` - passed before review close.
- `git diff --check` - passed before review close.

## 9. Governed Review

- workflow: `dg/review`
- run: `run-1784564547097177000-2`
- approval: `approval/run-1784564547097177000-2/review-scope-approved`
- presentation: `presentation/8b82015998c081e1`
- approval outcome: granted by delegated maintainer through proof enforcement
- kernel boundary: governance coordination only; review, documentation, and
  validation ran outside the kernel

## 10. Recommended Next Phase

Implement one explicit in-memory `DocsCheck` attestation runtime-composition
helper according to the corrected plan.

Do not add automatic checks, executor defaults, persistence, events, evidence,
reports, artifacts, schemas, CLI, additional check kinds, providers,
SideEffects, writes, hosted behavior, or release changes.
