# Governed Obligation Plan Phase 3 Review

Status: **changes requested; Phase 4 remains unauthorized**

## Scope

This owner-conducted focused maintainer and security review evaluates the
private Phase 3 governed-obligation explanation and differential-validation
slice. It reviews independent-oracle integrity, omission detection,
source-trace redaction, outcome disagreement, unsupported disclosure,
`SideEffect` non-inference, stale-closure rejection, ordering determinism,
duplicate handling, and the absence of runtime or public behavior.

This is repository assurance, not independent external security review. The
review changes documentation and roadmap authorization only. It does not add
runtime composition, persistence, a source extractor, public contracts,
provider behavior, or release behavior.

## Findings

### Blocker: observed outcomes are not authoritative Core decisions

The accepted Phase 3 plan requires the derived plan to be compared against
existing executor and validation decisions for representative workflows. Its
negative case must prove that removing a canonical gate is detected while the
existing runtime still denies or waits.

The implementation instead defines
`GovernedObligationCanonicalOutcome` and manually supplies both the expected
outcome and the observed outcome in module-local fixtures. The comparator
correctly detects disagreement between those two inputs, but neither input is
produced by an existing executor, policy evaluator, required-context consumer,
local-check validator, or report validator. The tests therefore prove that the
comparator compares records; they do not prove agreement with authoritative
Core behavior.

This is not accepted as differential runtime evidence. The new outcome enum is
also a parallel semantic vocabulary whose variants can drift from the
authoritative decision types. That is exactly the second-evaluator risk this
lane is required to avoid.

Required correction:

- remove the module-local canonical-outcome vocabulary as the claimed runtime
  oracle;
- obtain representative observed decisions from existing authoritative Core
  paths or their returned canonical types;
- keep expected obligations independently authored and fixed;
- prove the omitted-gate negative case while the authoritative path still
  returns its blocking or waiting decision; and
- make any unsupported mapping fail closed rather than translating it into a
  permissive generic outcome.

The correction remains private and test-focused. It must not enter executor
control flow or create a new public evaluator.

### Important: duplicate and unexpected-outcome guards lack direct tests

The comparator contains fail-closed branches for duplicate expectations,
duplicate observed outcomes, and outcomes outside the derived plan. The Phase
3 suite does not exercise those branches directly. This does not weaken the
current private implementation path, but the blocker correction must add
focused regression coverage before review can accept the differential
boundary.

### Accepted: deterministic explanation and omission commitments

The explanation remains bound to the plan algorithm, complete source-closure
commitment, complete inventory commitment, and canonically ordered obligation
identities. Source traces use versioned fixed-width framing, raw source
identities and commitments are omitted from `Debug`, stale explanations fail
closed, and independent fixed commitments detect a removed obligation even
when the source closure is unchanged.

These properties are accepted as a private explanation boundary. They do not
authorize runtime use.

### Accepted: unsupported posture and SideEffect non-inference

Unsupported authored requirements remain explicit in the explanation. Runtime
`SideEffect` observations are counted only as review facts and cannot add a
static obligation or alter the plan. The implementation does not infer
authored requirements from runtime observations.

### Accepted: private and non-authorizing boundary

The module remains crate-private and non-serializing. It has no loader,
persistence, executor, CLI, schema, SDK, provider, release, or publication
integration. It cannot grant approval, satisfy evidence, select policy,
reconcile a `SideEffect`, or complete a run.

## Governed Review

- workflow: `dg/review`
- run: `run-1793961269386554000-2`
- approval: `approval/run-1793961269386554000-2/review-scope-approved`
- presentation: `presentation/f2536427078a4fd5`
- approval outcome: granted with persisted presentation proof

Workflow OS governed the review scope and approval. Source inspection,
analysis, documentation edits, validation, Git, PR, and merge work occur
outside the kernel and remain disclosed. No runtime execution, provider call,
independent third-party review, or release behavior is simulated.

## Verdict And Next Phase

Changes requested. Phase 3's deterministic explanation boundary is accepted,
but its differential evidence is not. Phase 4 runtime composition remains
unauthorized.

The next authorized increment is one separately governed blocker correction
that replaces manually authored observed outcomes with evidence from existing
authoritative Core decisions and adds direct duplicate/unexpected-outcome
coverage. A second focused review is required before Phase 3 can be accepted.
Salsa, incremental dependencies, runtime composition, persistence, public
schema/SDK/CLI, providers, release behavior, and publication remain
unauthorized.
