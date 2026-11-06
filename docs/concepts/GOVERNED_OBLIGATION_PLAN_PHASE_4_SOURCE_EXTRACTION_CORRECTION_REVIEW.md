# Governed Obligation Plan Phase 4 Source-Extraction Correction Review

## Verdict

Changes requested before the correction is accepted.

The private implementation now states the correct security boundary: it
commits to immutable inputs but does not authorize the actor, sensitivity
ceiling, or binding time selected by a future runtime consumer. No executor or
provider consumes the private closure or projection, so this review found no
runtime-authority widening.

Two acceptance-evidence gaps remain. They do not justify reverting the
commitment-only correction, but they block the roadmap from treating its
required 26-case matrix as complete and continue to block executor
composition.

## Findings

### P1: Alternate-binding tests do not directly prove the closure commitment

The correction plan requires alternate actor, known sensitivity, and later
non-pre-bundle time to change both the required-context closure commitment and
the final projection commitment. The three tests construct valid alternates
but assert only that the final projection commitments differ.

That final hash also commits to the extracted source closure and inventory, so
its difference does not isolate the required-context closure commitment. The
private closure stores the required commitment, but the tests do not read or
compare it.

Required correction:

- add a private test-only accessor or compare the private field directly from
  the nested test module;
- assert that each alternate actor, sensitivity, and time changes the
  required-context closure commitment; and
- retain the existing final projection-commitment assertions and redaction
  checks.

### P1: Reachable exact-bound extraction is not proven end to end

`shared_cardinality_bounds_accept_exact_maxima_and_reject_max_plus_one`
invokes the private count-bound helper directly. It proves the comparison
operator, but it does not prove that a real canonical source closure is counted
correctly at the exact source maximum, that source maximum-plus-one is rejected
before complete plan construction, or that a reachable exact-obligation
extraction succeeds.

This distinction matters because the pre-allocation counter and the extraction
loops are separate code. A future change could make them diverge: the terminal
length check would still fail closed, but only after an unexpectedly larger
allocation and without proving the planned pre-construction resource bound.

The extractor derives at most one obligation from each canonical source.
Therefore every reachable extraction preserves `obligation_count <=
source_count`. An obligation-maximum-plus-one fixture whose source count remains
in bounds is structurally impossible. The helper-level independent obligation
overflow test is the correct defense-in-depth proof for that otherwise
unreachable branch; requiring a real fixture for it would manufacture an
invalid product shape.

Required correction:

- build bounded generated fixtures whose real extraction reaches exactly the
  canonical-source maximum and a reachable exact obligation maximum;
- prove those extractions succeed;
- add a real fixture at source maximum plus one and prove rejection occurs
  before a complete plan is returned;
- retain the helper-level independent obligation maximum-plus-one rejection and
  document why it is unreachable through the one-source-per-obligation
  extraction invariant; and
- continue deriving every size from
  `immutable_run_bundle_reference_max_count()` rather than copying its literal.

## Accepted Properties

Subject to the two test blockers above, the review accepts these properties of
the implementation:

- the private type and documentation consistently describe commitment identity
  rather than consumer-owned authority;
- exact immutable bundle, workflow, run, step, contract identity, contract
  version, and contract content mismatches fail closed;
- missing and duplicate per-step posture fail closed with bounded errors;
- unknown sensitivity and pre-bundle time fail at the execution-binding
  boundary;
- checked cardinality arithmetic rejects overflow;
- multi-step input ordering is canonical;
- errors and Debug output remain payload-safe;
- no external module imports the private commitment closure or extractor; and
- executor composition, persistence, public contracts, providers, writes,
  dependencies, Salsa, release behavior, and publication remain absent.

The existing required-context execution-binding suite separately proves
binding-hash integrity and validated deserialization. The full implementation
suite and exact-head CI were green before this review.

## Validation

- focused governed-obligation-plan tests: 36 passed;
- warnings-denied `workflow-core` Clippy with all targets and features: passed;
- external consumer search for the private closure and extractor: none found;
- documentation checks: passed; and
- `git diff --check`: passed.

## Governed Review

- workflow: `dg/review`
- run: `run-1793981641686515000-2`
- approval: `approval/run-1793981641686515000-2/review-scope-approved`
- presentation: `presentation/54fd4a09ade33bf9`
- approval outcome: granted with persisted presentation proof
- presentation enforcement: `proof_enforced`
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations

Repository inspection, tests, documentation, Git, and review judgment occurred
outside the kernel. This is an owner-conducted maintainer/security review, not
independent external assurance.

## Review-Artifact Correction

Read-only follow-up found that the first draft of this review requested an
obligation-only over-bound extraction fixture that cannot exist under the
one-source-per-obligation invariant. Draft PR #777 was closed without merge.
The corrected review preserves reachable end-to-end boundary requirements and
retains helper-level proof for the structurally unreachable independent
obligation overflow branch.

- workflow: `dg/blocker`
- run: `run-1793982071027613000-2`
- approval: `approval/run-1793982071027613000-2/fix-approved`
- presentation: `presentation/e9d7e24dccfa87c8`
- approval outcome: granted with persisted presentation proof
- presentation enforcement: `proof_enforced`
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations

## Next Required Gate

Run one separately governed blocker-fix phase limited to the two missing test
families. The fix must not change production semantics unless the reachable
real boundary fixtures reveal an implementation defect. Do not construct an
impossible obligation-only overflow shape merely to satisfy a test. Repeat
focused tests, warnings-denied Clippy, the full `workflow-core` suite if
production code changes, docs checks, and diff checks, then perform one focused
review of the exact fix before reconsidering acceptance.
