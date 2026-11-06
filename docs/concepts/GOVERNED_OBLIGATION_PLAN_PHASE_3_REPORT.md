# Governed Obligation Plan Phase 3 Report

## Status

Private deterministic explanation and differential-validation implementation
complete. Focused maintainer/security review is the next authorized increment.
Runtime composition, persistence, and every public surface remain unauthorized.

## Implemented Scope

Phase 3 extends the accepted private obligation-plan model with two bounded
capabilities:

1. a deterministic, redaction-safe explanation projection; and
2. a fail-closed differential comparator whose expected obligations and
   observed canonical outcomes must be supplied independently.

The implementation remains crate-private. It does not serialize a plan, write
state, load project contracts, enter executor control flow, decide policy,
grant approval, satisfy evidence, infer authority, reconcile `SideEffects`, or
complete a run.

## Explanation Projection

Each explanation entry contains:

- the existing source-derived obligation identity;
- the closed canonical source kind;
- a versioned, redaction-safe source trace commitment;
- the closed obligation kind;
- mandatory, optional, unsupported, or unknown posture; and
- the existing authoritative evidence class, when one exists.

The projection carries the accepted algorithm version, complete source-closure
commitment, complete inventory commitment, and canonically ordered entries. It
does not expose raw source identities through `Debug` output. Rebuilding the
same model from reordered inputs produces the same explanation.

The source trace is a deterministic commitment to source kind, source identity,
and source content commitment. It permits exact private differential matching
without turning raw source identity into a diagnostic payload.

## Differential Boundary

The comparator accepts three separately supplied records:

- the derived plan and its explanation;
- independently authored expected obligations, source traces, postures,
  evidence classes, and canonical outcomes; and
- independently observed canonical runtime or validation outcomes.

The comparator does not compute a preferred runtime result. It fails closed
when:

- the explanation is stale relative to the plan;
- the source-closure or inventory commitment differs from the independent
  expectation;
- explanation ordering is not canonical;
- expected obligations are missing, duplicated, or unexpected;
- explanation fields disagree with independent expectations;
- canonical outcomes are missing, duplicated, unexpected, or disagree with
  independent expectations.

Disagreement therefore cannot select the more permissive result.

## Independent Fixtures

The representative fixture expectations use fixed, externally specified
commitment vectors rather than values copied from the implementation result.
They cover:

- one mandatory approval gate;
- one mandatory local-check gate;
- approval-wait and local-check-required canonical outcomes;
- an omitted local-check gate while the independently observed outcomes remain
  complete;
- unsupported local-check disclosure;
- a runtime `SideEffect` observation that is counted but cannot manufacture a
  static obligation;
- stale source closure;
- deliberately unstable explanation order; and
- canonical-outcome disagreement.

The fixture uses the existing closed obligation and evidence vocabulary. It
does not add a separate workflow language or runtime evaluator.

## SideEffect Boundary

A runtime `SideEffect` observation is represented only as a differential-review
fact. The validation result may count that fact, but it cannot convert it into
a static obligation or alter the plan. Authored `SideEffect` requirements still
require an existing canonical source before they can enter a future inventory.

## Validation

- `cargo test -p workflow-core --lib governed_obligation_plan`: 18 passed;
  zero failed.
- `cargo test -p workflow-core`: passed, including 766 library tests, the full
  integration suite, and doc tests; opt-in live credential and smoke tests
  remained ignored by design.
- `cargo clippy -p workflow-core --all-targets --all-features -- -D warnings`:
  passed.
- `cargo fmt --all --check`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## Governed Phase

- workflow: `dg/implement`
- run: `run-1793958224023836000-2`
- approval:
  `approval/run-1793958224023836000-2/implementation-approved`
- presentation: `presentation/b6d837a58af33889`
- approval outcome: granted with persisted presentation proof

The kernel governed phase scope and approval. Repository edits, test commands,
and documentation were performed by the delegated maintainer outside the kernel
executor. No runtime behavior, provider call, independent third-party review,
or public product behavior was simulated.

## Residual Risks

- The Phase 3 oracle rows and canonical outcomes are manually authored review
  fixtures. They are intentionally independent from plan derivation, but human
  fixture mistakes remain possible and require focused review.
- This phase does not invoke a live executor. It compares private explanation
  output against independently authored representations of existing canonical
  outcomes. Phase 4 remains responsible for one reviewed opt-in runtime
  composition if this evidence is accepted.
- The private model still has no canonical source extractor and no reviewed
  cardinality bounds. Neither may be introduced implicitly during runtime
  composition.
- V1 commitment framing and known vectors are locked. Any semantic change
  requires a new algorithm version.
- Runtime `SideEffects` remain observations only. This phase does not create an
  authored `SideEffect` requirement source.

## Next Authorized Phase

Perform one focused maintainer/security review of this Phase 3 implementation.
The review must verify independent-oracle integrity, omission detection,
source-trace redaction, outcome disagreement behavior, unsupported disclosure,
`SideEffect` non-inference, stale-closure rejection, ordering determinism, and
the absence of runtime or public behavior.

Phase 4 runtime composition remains unauthorized until that review accepts the
implementation. Salsa, incremental dependencies, schemas, SDKs, CLI output,
providers, release behavior, and publication remain unauthorized.
