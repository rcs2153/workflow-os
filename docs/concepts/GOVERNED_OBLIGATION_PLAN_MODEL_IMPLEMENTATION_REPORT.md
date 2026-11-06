# Governed Obligation Plan Model Implementation Report

## Status

Phase 2 private model-only implementation complete. Focused
maintainer/security review is the next authorized increment. Persistence,
runtime composition, and every public surface remain unauthorized.

## Implemented Boundary

This phase adds one crate-private deterministic inventory model for static
governance requirements. The model is intentionally disconnected from project
loading, execution, persistence, schemas, SDKs, CLI output, providers, reports,
and release behavior.

The model contains closed vocabulary for:

- canonical source classes: workflow step, policy rule, required-context
  contract, local-check declaration set, and WorkReport contract;
- static obligation classes: policy, approval, required context, local check,
  and WorkReport artifact;
- static posture: mandatory, optional, unsupported, and unknown; and
- references to existing evidence classes without creating evidence or
  satisfaction state.

Runtime `SideEffects` are intentionally absent. An observed runtime effect
cannot manufacture an authored static obligation.

## Deterministic Identity And Completeness

The builder receives two independent inputs:

1. the complete canonical source closure; and
2. the derived obligation entries.

It produces:

- source-derived obligation identities;
- deterministic entry ordering;
- a source-closure commitment over the complete canonical source set; and
- an independent inventory commitment over the complete sorted derived entry
  set.

Keeping the commitments independent is essential. Removing an obligation while
retaining its canonical source changes the inventory commitment without
changing the source-closure commitment. Removing a canonical source changes the
source-closure commitment. This permits a later independent validator to detect
both derivation omission and source-set drift without treating the model as its
own oracle.

The builder fails closed for:

- an empty source closure;
- an empty obligation inventory;
- malformed source identities;
- conflicting content commitments for one canonical source identity;
- an obligation derived from a source outside the canonical closure; and
- duplicate derived obligations.

## Safety And Privacy

The model is private to `workflow-core` and is not re-exported. It has no Serde
surface, persistence contract, runtime mutation, or executor path. Constructors
for canonical sources and derived obligations remain module-owned so callers
cannot supply arbitrary obligation IDs, kinds, or evidence-class strings.

Debug output redacts source identities, source commitments, aggregate
commitments, and derived obligation IDs. The model stores no source payload,
policy prose, evidence payload, approval decision, runtime status, or provider
value.

## Focused Tests

Ten focused unit tests prove:

- deterministic inventory and commitments across input order;
- omitted obligation detection independent of source closure;
- omitted canonical source detection;
- source-content changes invalidate derived identity and both roots;
- conflicting source commitments fail closed;
- duplicate obligations fail closed;
- empty and malformed inputs fail closed;
- obligations outside the independent source closure fail closed;
- Debug output is redaction-safe; and
- the closed static posture and evidence vocabularies are preserved.

## Validation

- `cargo test -p workflow-core --lib governed_obligation_plan`: 10 passed;
- `cargo test -p workflow-core`: passed, including 758 library unit tests, all
  package integration binaries, and doc tests; opt-in live credential tests
  remained intentionally ignored;
- `cargo clippy -p workflow-core --all-targets -- -D warnings`: passed;
- `cargo fmt --all --check`: passed;
- documentation and diff checks: required before phase close.

## Governance

- workflow: `dg/implement`;
- run: `run-1793952881330573000-2`;
- approval: `approval/run-1793952881330573000-2/implementation-approved`;
- presentation proof: `presentation/fdbcd9c5c2a7ffd3`;
- approval content hash:
  `fdbcd9c5c2a7ffd30c434f7c1098f27c6ce8fc3ddc39e86395268bcd31fd3515`;
- approval outcome: granted under delegated maintainer authority;
- phase status: `Completed`;
- event summary: 39 events, one approval, zero retries, zero escalations;
- approval-presentation enforcement: `proof_enforced` with one persisted
  presentation record.

Repository edits, shell commands, validation commands, and this report were
performed by the delegated maintainer outside the kernel executor. The kernel
governed the phase boundary and approval; it did not execute edits, tests, git,
or provider actions. No handler, check, or report coverage was simulated.

## Non-Goals

This phase does not add:

- project-loader composition or canonical-source extraction;
- persistence, serialization, schemas, SDKs, or CLI output;
- executor, approval, policy, evidence, report, or completion authority;
- runtime satisfaction state or a second evaluator;
- SideEffect inference;
- provider reads or writes;
- incremental computation or Salsa;
- release or publication behavior.

## Next Step

Perform one focused maintainer/security review of the private model. The review
must verify independent source-closure completeness, source-derived identity,
closed vocabulary, commitment framing, omission detection, redaction, and the
absence of runtime/public behavior. Phase 3 deterministic explanation and
differential validation remain unauthorized until that review accepts this
implementation.
