# Governed Obligation Plan Phase 4 Source-Extraction Correction Report

## Status

Implemented for focused review. Executor composition remains unauthorized.

The private Phase 4 source closure now describes what it can prove: exact
immutable commitment identity. It does not claim that the extractor owns or
authorizes the actor, sensitivity ceiling, or binding time selected by a
future runtime consumer.

## Implemented Correction

`GovernedObligationCanonicalInputClosure` is replaced by the private
`GovernedObligationCanonicalCommitmentClosure`. Its documentation states that
the closure proves the exact immutable sources supplied to extraction but does
not prove that a future consumer selected or authorized authority-sensitive
binding fields.

The constructor continues to fail closed on:

- missing per-step required-context posture;
- duplicate step posture;
- wrong immutable bundle, workflow, run, or step;
- wrong contract identity, version, or content hash;
- internally invalid execution bindings;
- unknown sensitivity;
- pre-bundle binding time; and
- pair, source, obligation, or arithmetic bounds.

The correction deliberately does not reject a different actor, known
sensitivity ceiling, or non-pre-bundle time merely because it differs from
another internally valid binding. Each alternate value forms a different
valid commitment and deterministically changes the projection commitment.
Only a future consumer-owned comparison may decide that such a commitment is
semantically wrong for an operation.

## Bound And Determinism Hardening

Cardinality checks now use three private helpers:

- one shared required-context pair bound;
- one independent source and obligation count bound; and
- one checked-addition helper that fails closed on overflow.

All limits continue to derive from
`immutable_run_bundle_reference_max_count()`. The obligation module does not
copy the underlying literal. Duplicate step posture is detected as a binding
mismatch rather than being hidden behind a generic closure-length failure.

The focused suite now proves:

- missing and duplicate per-step posture;
- wrong bundle, workflow, run, step, contract identity, contract version, and
  contract hash;
- unknown sensitivity and pre-bundle time;
- alternate actor, known sensitivity, and later time as distinct valid
  commitments;
- payload-safe errors and Debug output;
- multi-step insertion-order independence;
- exact pair, source, and obligation maxima;
- independent maximum-plus-one rejection for those families;
- checked arithmetic overflow rejection;
- repeated deterministic extraction;
- explicit-none source posture; and
- no authority, satisfaction, evidence, or execution semantics in the plan or
  projection.

The existing required-context execution-binding tests continue to pin
content-hash integrity and serialization validation.

## Runtime Boundary

No executor or provider imports the corrected closure or projection. The
implementation does not add persistence, a public contract, current-authority
resolution, required-context satisfaction, approval, evidence, checks,
`SideEffect` authorization, events, reports, completion, or external actions.

Executor composition remains blocked until a separately governed plan names
one exact runtime consumer that independently owns expected actor, sensitivity,
trusted time, freshness, the complete per-step closure, and the existing
durable governance-assessment lifecycle.

## Validation

- focused `workflow-core` obligation-plan unit tests: 36 passed;
- warnings-denied `workflow-core` Clippy with all targets and features: passed;
- full `workflow-core` test suite: passed, including 784 unit tests, all
  applicable integration targets, and the crate doc test; explicitly opt-in
  live tests remained skipped by their declared gates;
- `cargo fmt --all --check`: passed;
- documentation checks: passed; and
- `git diff --check`: passed.

## Governed Implementation

- workflow: `dg/implement`
- run: `run-1793978312802726000-2`
- approval: `approval/run-1793978312802726000-2/implementation-approved`
- presentation: `presentation/52d98c31e15c0b39`
- approval outcome: granted with persisted presentation proof
- presentation enforcement: `proof_enforced`
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations

Source inspection, edits, tests, documentation, Git, PR, and merge work occur
outside the kernel and remain disclosed. Workflow OS governs the phase scope
and approval; it does not simulate execution or independent review.

## Next Required Gate

Perform one focused maintainer/security review of the exact correction and its
tests. No executor composition, persistence, public schema/SDK/CLI, provider,
dependency, Salsa, release, or publication work is authorized by this report.
