# Governed Obligation Plan Phase 4 Boundary Test Fix Report

## Status

Implemented for focused review. Production semantics are unchanged and
executor composition remains unauthorized.

## Corrected Evidence

The three valid alternate-binding tests now compare the private
required-context closure commitment directly. Alternate actor, known
sensitivity ceiling, and non-pre-bundle binding time each change that
commitment and continue to change the final projection commitment.

A generated valid-project fixture now exercises the real canonical extraction
path at its resource boundary. It proves:

- exact shared source maximum succeeds;
- the highest obligation count reachable through the current valid authored
  contract succeeds at `maximum - 1`;
- source maximum plus one fails closed with
  `governed_obligation_plan.runtime_composition.cardinality_exceeded` before a
  complete plan is returned; and
- all counts derive from `immutable_run_bundle_reference_max_count()` rather
  than a copied literal.

The one-count difference is an authored-model property, not a relaxed bound.
Every valid extraction contains one report source. Under the currently valid
fixture contract that source is disclosed but non-obligating, because report
enforcement is not authorable at this boundary. Manufacturing a report
obligation would require an unsupported project shape that semantic validation
correctly rejects.

The independent obligation maximum-plus-one branch remains covered at the
private helper boundary. A real fixture cannot reach it while source count
remains valid because canonical extraction derives at most one obligation per
source.

## Scope Boundary

This fix changes tests and acceptance documentation only. It does not change
the extractor, plan, projection, immutable bundle, required-context binding,
executor, persistence, schema, SDK, CLI, provider, dependency, Salsa, release,
or publication behavior.

## Validation

- focused governed-obligation-plan tests: 38 passed;
- warnings-denied `workflow-core` Clippy with all targets and features: passed;
- full `workflow-core` rerun: passed, including 786 unit tests, all applicable
  integration targets, and the crate doc test; explicitly opt-in live tests
  remained skipped by their declared gates;
- `cargo fmt --all --check`: passed;
- documentation and release-state checks: passed; and
- `git diff --check`: passed.

The first full-suite attempt passed 785 of 786 unit tests but encountered the
pre-existing timing-sensitive
`competing_local_timer_wrappers_enter_executor_at_most_once` contention case.
That unrelated test passed immediately on focused rerun, passed 10 consecutive
focused repetitions, and passed in the clean full-suite rerun. No production
change was made to suppress or bypass it.

## Governed Blocker Fix

- workflow: `dg/blocker`
- run: `run-1793984871947140000-2`
- approval: `approval/run-1793984871947140000-2/fix-approved`
- presentation: `presentation/38e78fc03aa25415`
- approval outcome: granted with persisted presentation proof
- presentation enforcement: `proof_enforced`
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations

Repository inspection, fixture design, edits, tests, documentation, Git, PR,
and merge work occur outside the kernel and remain disclosed. Workflow OS
governs the approved blocker-fix scope; it does not simulate execution or
independent review.

## Next Required Gate

After validation and governed phase close, perform one focused
maintainer/security review of this exact test-only fix. Do not begin executor
composition or broaden runtime authority from this report.
