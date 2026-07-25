# DocsCheck Attestation Runtime Composition Plan Review

## 1. Executive Verdict

Needs planning blocker fixes.

The plan selects the correct narrow runtime boundary, but two authority and
failure-semantics details must be resolved before implementation.

## 2. Scope Verification

The plan stays within planning-only scope. It authorizes one explicit,
in-memory, opt-in `DocsCheck` helper and explicitly excludes automatic checks,
executor-default changes, persistence, events, evidence, reports, artifacts,
schemas, CLI behavior, providers, SideEffects, writes, hosted behavior, and
release changes.

No implementation was added during planning or review.

## 3. Integration Boundary Assessment

Using an additive internal helper is appropriate. The current handler reduces
`LocalCheckResult` to `SkillOutput`, so reconstructing proof from an output
reference would incorrectly make presentation data authoritative. Direct
composition around the bounded process runner and structured result is the
smallest safe path.

Keeping `LocalSkillRegistry::new()` empty and leaving
`LocalExecutor::execute` unchanged preserves current runtime semantics.

## 4. Pre-Execution Binding Assessment

The planned ordering correctly requires validation and creation of
`ImmutableLocalCheckExecutionBinding` before process launch. The binding
commits the validated stored bundle, canonical command contract, explicit
registered-handler selection, and effective execution policy.

The process cannot start after a binding failure, and accepted proof cannot
exist before a structured result. This matches the accepted binding and
verifier contracts.

## 5. Observation Authority Assessment

The plan correctly prohibits caller-created observations and assigns result,
candidate, and observation construction to Core. However, its proposed input
still accepts observation start, completion, and evaluation timestamps from a
"Core-owned caller boundary."

That is underspecified. The composition helper itself owns process invocation,
so the implementation must use one explicit injected Core clock/time source and
sample it immediately before runner invocation, immediately after runner
completion, and at verifier evaluation. Public or general callers must not
supply observed timestamps as facts.

This is a planning blocker because timing participates in observation
integrity, ordering, and freshness proof.

## 6. Result And Proof Semantics Assessment

The planned distinction is correct:

- a passed result may produce accepted proof;
- a failed or timed-out process remains an honest structured result but does
  not satisfy a passed-only requirement; and
- internal binding, execution, redaction, or integrity failure returns no
  partial outcome.

The mechanism remains unresolved. The plan allows either mapping a verifier
rejection or using an eligibility predicate and warns against broad error-code
matching, but does not decide the contract.

The fix must define a typed requirement-eligibility check before verifier
invocation. If the observed result status is not in the requirement's accepted
status set, return the structured no-proof outcome. If it is eligible, invoke
the verifier and propagate every verifier error as an integrity failure. Do not
interpret verifier error strings or codes as normal check outcomes.

This is a planning blocker because implementation otherwise risks converting
integrity failures into benign no-proof results.

## 7. Privacy And Redaction Assessment

The plan preserves existing bounded output capture, redaction, safe Debug, and
stable error rules. It does not add raw command output, arguments, paths,
environment values, source content, credentials, or provider payloads to the
binding, observation, candidate, accepted proof, or errors.

Accepted proof remains payload-free and non-deserializable.

## 8. Test Plan Assessment

The planned tests cover ordering, exact binding, Core-owned construction,
passed proof, honest failed/timed-out no-proof outcomes, internal errors,
redaction, mismatch handling, privacy, empty default registration, and
workspace regressions.

The blocker fix should add explicit tests proving:

- the injected clock is sampled in pre-run, post-run, and evaluation order;
- callers cannot supply observation timestamps;
- ineligible status bypasses verifier invocation through a typed predicate;
- eligible status always invokes the verifier; and
- verifier integrity errors are never converted to a no-proof outcome.

## 9. Blockers

1. Replace caller-supplied observation/evaluation timestamps with a narrow
   injected Core-owned clock sampled by the helper around process execution.
2. Define typed status eligibility before verification and require all verifier
   failures for eligible results to propagate as integrity errors.

## 10. Non-Blocking Follow-Ups

- Freshness must be reevaluated at later use boundaries.
- Handler implementation provenance remains registered-unattested.
- Attestation persistence, event/audit projection, evidence/report use, and
  proportional-governance fact composition remain separate phases.
- The dogfood phase-close presentation-record cap defect remains open.

## 11. Governed Review

- workflow: `dg/review`
- run: `run-1784522094505421000-2`
- approval: `approval/run-1784522094505421000-2/review-scope-approved`
- presentation: `presentation/9a7a2076fcf8538f`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: completed
- kernel boundary: governance coordination only; review, repository
  inspection, documentation, and validation ran outside the kernel

## 12. Recommended Next Phase

Perform a planning blocker fix only. Resolve clock ownership and typed
eligibility semantics, update the plan and report, then perform a focused
re-review.

Do not implement runtime composition until the corrected plan is accepted.
