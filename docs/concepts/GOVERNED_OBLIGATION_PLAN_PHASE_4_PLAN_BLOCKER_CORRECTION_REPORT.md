# Governed Obligation Plan Phase 4 Plan Blocker Correction Report

## Status

The three focused-review blockers are corrected in the Phase 4 plan. Runtime
implementation remains unauthorized pending a second focused
maintainer/security review.

## Complete Canonical Input Closure

The corrected plan no longer calls `ImmutableRunBundleBuildResult` the sole
obligation source. The immutable bundle remains the mandatory execution base,
but required context enters only through existing content-addressed
`RequiredContextContractBinding` and `RequiredContextExecutionBinding` values
validated against the exact stored bundle, step, actor, and sensitivity
posture.

Every step requires one explicit bound contract, including an existing
canonical explicit-none contract. Missing input never means no required
context. Contract and execution-binding commitments join the static source
closure, and omitted, substituted, duplicate, stale, wrong-step, wrong-actor,
or wrong-bundle inputs fail closed.

## Subordinate Projection Binding

The proposed private value is now explicitly a
`GovernedObligationPlanProjectionBinding`. It proves static inventory identity
only and contains no execution disposition, disclosure, completeness, current
facts, authority, approval, evidence, or satisfaction.

The existing durable `GovernanceAssessmentBinding` remains the sole accepted
proportional-governance assessment binding. Any future runtime composition must
use its current-fact reassessment lifecycle and reproduce both bindings
independently. The plan projection cannot substitute for or weaken it.

## Truthful Persistence Boundary

All deterministic extraction and projection checks must complete before
immutable bundle persistence. After create-only persistence, a read-back or
rederivation failure can leave an unactivated immutable bundle. The corrected
plan therefore promises no runtime event, approval, invocation, WorkReport,
`SideEffect`, or external action after such failure; it does not claim the
bundle can be atomically erased.

Exact retry must validate and reuse the identical bundle or fail closed.
Overwrite, deletion, silent replacement, and automatic orphan cleanup remain
outside Phase 4.

## Shared Invariants And Consumer Gate

The correction requires reuse of one crate-private immutable-bundle bound and
existing store record-resolution/bijection validation. It distinguishes
workflow-authored report artifact requirements from runtime WorkReport
contracts and artifacts.

The first implementation is narrowed to the private canonical-input closure,
extractor, and projection binding. Executor composition remains blocked until
a separately reviewed amendment names one exact path that already owns the
stored bundle, required-context bindings, durable governance reassessment, and
all selected runtime gates. A generic wrapper is not authorized.

## Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.

## Governed Phase

- workflow: `dg/blocker`
- run: `run-1793968438239031000-2`
- approval: `approval/run-1793968438239031000-2/fix-approved`
- presentation: `presentation/1266c01c0af9b599`
- approval outcome: granted with persisted presentation proof
- phase status: `Completed`
- event summary: 39 events, one approval, zero retries, zero escalations
- presentation enforcement: `proof_enforced`

Workflow OS governed the correction scope and approval. Documentation edits,
validation, Git, PR, and merge work occur outside the kernel and remain
disclosed. No runtime implementation, provider action, release, publication,
or independent third-party assurance is simulated.

## Next Authorized Phase

Perform a second focused maintainer/security review of the corrected plan.
Runtime implementation remains unauthorized until that review accepts the
complete canonical input closure, subordinate projection binding, persistence
boundary, shared invariants, and consumer gate.

