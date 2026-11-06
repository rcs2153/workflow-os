# Governed Obligation Plan Phase 4 Runtime Composition Plan

## Status

Proposed for focused maintainer/security review. This plan authorizes no
runtime implementation until that review accepts the source-extraction,
immutable-binding, cardinality, provenance, consumption, and fail-closed
boundaries below.

## Purpose

Phase 3 proved that a private derived obligation plan can explain a bounded
set of static governance requirements and can be compared with existing Core
decision types without selecting a result. Phase 4 must prove one further
property: the plan can be composed into one existing opt-in local runtime path
without becoming a second policy engine, evidence verifier, approval system,
run ledger, or completion authority.

The smallest credible slice binds one private plan to the exact immutable run
bundle used by one local execution. The plan is consumed only as a fail-closed
pre-execution inventory and redaction-safe explanation. Every decision about
whether work may start, resume, invoke a handler, accept evidence, reconcile a
`SideEffect`, persist a WorkReport, or complete a run remains with the existing
authoritative Core path.

## User And Runtime Failure

Core currently distributes static governance requirements across validated
workflow, skill, policy, required-context, local-check, and report contracts.
The immutable run bundle preserves their canonical execution closure, and the
executor enforces the relevant runtime paths, but no run-bound projection
proves before execution that:

- the complete supported static source closure was inspected;
- every supported source produced its required obligation or explicit
  unsupported posture;
- the resulting inventory is bound to the exact run bundle; and
- later explanation refers to the same inventory rather than a mutable
  checkout or caller-authored reconstruction.

Phase 4 addresses only that inspectability and binding failure. It does not
change what Core requires or how Core decides whether a requirement is
satisfied.

## Chosen Runtime Seam

The implementation must use the existing immutable-run-bundle construction
and opt-in local execution path.

The canonical source boundary is
`ImmutableRunBundleBuildResult`, specifically:

- `ImmutableRunBundleManifest` for run identity, resolved execution-context
  commitment, ordered definition references, canonical local-check references,
  handler posture, execution posture, and bundle integrity root;
- `ImmutableRunBundleDefinitionRecord` for canonical validated typed workflow,
  skill, and policy definitions; and
- `CanonicalLocalCheckDeclarationSetRecord` for authoritative per-step
  local-check declarations.

The extractor must operate on the already-built in-memory bundle result before
persistence. It must not reload YAML, inspect the mutable project checkout,
accept an independently assembled `ProjectBundle`, or accept caller-supplied
obligations. This ensures that bundle construction and obligation extraction
share one resolved canonical closure rather than two resolution algorithms.

After the bundle is persisted and read back through
`ImmutableRunBundleStore`, the consumer must revalidate the exact
`ImmutableRunBundleBinding` before any run event is appended. Continuation or
replay must use only the stored immutable records and binding already attached
to the run.

## Private Binding Model

Add one crate-private, non-serializing binding value containing only:

- `GovernedObligationPlanAlgorithm`;
- the immutable run bundle ID, version, and root hash or the existing
  `ImmutableRunBundleBinding` value;
- the bound `WorkflowRunId`;
- the plan source-closure commitment;
- the plan inventory commitment; and
- the obligation count.

The binding is projection data. It is not a capability, approval, policy
decision, evidence receipt, event, WorkReport, or terminal-state record. It is
not separately persisted in Phase 4. The exact bundle remains durable truth;
the plan and binding are deterministically rederived from the stored bundle
when needed and must reproduce the same commitments.

The binding constructor is module-owned. Callers cannot provide a bundle root,
source commitment, inventory commitment, run identity, or obligation count
independently. Construction succeeds only from one validated bundle result and
one plan derived from that same result.

## Canonical Extraction Rules

Create one crate-private extractor that accepts
`&ImmutableRunBundleBuildResult` and returns one plan plus its private binding.
The extractor must:

1. Validate the bundle manifest and every referenced canonical record through
   existing immutable-bundle invariants.
2. Require exactly one workflow record and resolve every workflow step to the
   exact skill record and canonical local-check declaration-set record already
   referenced by the manifest.
3. Resolve referenced policies only from canonical policy records included in
   the bundle.
4. Reuse existing proportional-governance derivation and existing typed policy
   effects where they are the canonical source. It must not copy their
   selection logic into the obligation module.
5. Derive required-context obligations only from existing canonical
   required-context contracts attached to the immutable workflow/skill
   closure.
6. Derive local-check obligations only from canonical local-check declaration
   sets. An absent declaration remains absence; runtime check observations do
   not manufacture a static obligation.
7. Derive report obligations only where the immutable canonical workflow
   contract already requires the existing report-artifact path. Advisory or
   unsupported posture must remain explicit.
8. Reject unknown fields, unsupported mandatory contracts, duplicate sources,
   unresolved references, conflicting commitments, missing records, extra
   records that would alter the supported closure, or any source that cannot
   be represented without widening the closed Phase 2 vocabulary.
9. Produce deterministic ordering independent of filesystem order, map order,
   insertion order, or host architecture.

No runtime `SideEffect` record may create a static obligation. Existing
authored side-effect requirements remain outside this first composition unless
they already have a canonical supported source in the accepted private model.

## Cardinality And Resource Bounds

The first runtime composition must be bounded before allocation and before run
creation.

- Reuse the immutable-bundle maximum of 256 definition or reference records as
  the outer canonical-source bound.
- Permit at most 256 canonical obligation sources.
- Permit at most 256 derived obligations total.
- Permit at most one obligation for one source/kind pair.
- Retain the existing 256-byte source-identity bound.
- Reject arithmetic overflow and any input that exceeds a bound before
  constructing the complete plan.

These are conservative Phase 4 limits, not a new public contract. Increasing
them requires measured evidence and focused review. Truncation is forbidden.

## One Opt-In Consumer

Add one crate-private opt-in wrapper around the existing immutable-bundle local
execution entry point. Existing public and crate-visible execution APIs remain
unchanged.

For a new run, the wrapper must:

1. Prepare the existing execution plan and evaluate the existing pre-run
   policy exactly as today.
2. Load and validate the project using the existing capability.
3. Build the immutable run bundle with the existing builder.
4. Derive the governed obligation plan from that exact build result.
5. Construct and validate the private bundle/plan binding.
6. Persist and read back the immutable bundle through the existing store.
7. Prove the stored manifest reproduces the exact binding and deterministically
   rederive the same plan commitments from the stored records.
8. Fail closed before `RunCreated`, `RunValidated`, `RunStarted`, or any other
   runtime event if extraction, bounds, binding, or rederivation fails.
9. Delegate unchanged to the existing executor path.

For an existing run, continuation must:

1. rehydrate the run through the existing backend;
2. require its existing immutable-run-bundle binding;
3. read the exact stored bundle;
4. validate run, request, and bundle identity through existing checks;
5. rederive and revalidate the private plan binding from stored immutable
   records only; and
6. delegate unchanged to the existing continuation path.

The wrapper may return a private redaction-safe explanation alongside the
ordinary execution result for test inspection. The explanation cannot be used
to select or override the execution result.

## Authoritative Runtime Boundaries

The composition must preserve, without proxying or reinterpretation:

- conservative policy evaluation;
- proportional-governance workload assessment;
- current-authority and capability resolution at the protected time of use;
- approval request, presentation proof, decision, expiry, revocation, and
  consumption semantics;
- required-context resolution and consumption;
- local-check declaration resolution, handler execution, result validation,
  evidence assurance, and citation;
- handler availability and missing-handler failure;
- `SideEffect` authorization, lifecycle, persistence, reconciliation, and
  disclosure;
- append-only runtime events and durable rehydration;
- retry, idempotency, executor-yield, wait, denial, and failure behavior;
- WorkReport creation, referential integrity, artifact persistence, and
  replay; and
- terminal completion projection.

An obligation entry is never proof that one of these runtime boundaries has
passed. Phase 3's `PolicyDecision`, workload assessment, and validation error
fixtures remain differential-review evidence only and must not be accepted as
runtime-satisfaction input.

## Fail-Closed Errors

Add a small private stable error family under
`governed_obligation_plan.runtime_composition.*` for:

- source closure incomplete;
- unsupported canonical source;
- source or obligation cardinality exceeded;
- bundle binding mismatch;
- plan binding mismatch;
- stored rederivation mismatch;
- run identity mismatch; and
- obligation inventory disagreement.

Messages must be payload-free and redaction-safe. The errors must not include
definition IDs, source identities, hashes, policy contents, check commands,
approval reasons, evidence contents, or local paths.

## Required Tests

### Source extraction

- identical bundle inputs produce identical source and inventory commitments;
- filesystem, map, and insertion-order changes do not affect output;
- every supported workflow step, policy, required-context contract,
  declaration set, and report contract is represented exactly once;
- absent authored requirements do not become obligations;
- runtime `SideEffect` facts do not become obligations;
- missing, extra, duplicate, conflicting, unsupported, and unresolved records
  fail closed;
- every cardinality boundary accepts the maximum and rejects maximum plus one;
  and
- Debug and error output reveal no protected identity or payload.

### Immutable binding

- the plan binds to the exact bundle root, version, run ID, algorithm, source
  closure, inventory, and count;
- changing workflow, skill, policy, required context, local checks, report
  posture, resolved context, handler posture, execution posture, or any
  included canonical record invalidates the binding;
- a plan from run or bundle A cannot be consumed for run or bundle B;
- an arbitrary caller-supplied `PolicyDecision`, assessment, obligation ID, or
  commitment cannot establish provenance; and
- stored-bundle rederivation matches clean in-memory derivation.

### Runtime composition

- one approved representative workflow follows the same wait/resume/terminal
  result and event sequence with the private wrapper as with the existing
  executor;
- one denied policy remains denied;
- one missing approval remains waiting;
- one missing or failed local check remains blocked by the existing verifier;
- one missing handler still fails through the existing handler boundary;
- one report requirement still relies on the existing artifact path;
- one SideEffect path remains governed exclusively by existing SideEffect
  checks;
- exact retry and continuation rederive the same binding without duplicate
  events or work;
- mutable-checkout changes after run creation do not affect continuation;
- corrupted stored bundle material fails before continuation; and
- extraction or binding failure on a new run creates no run events, approval,
  handler invocation, artifact, or external effect.

### Differential non-regression

- the accepted Phase 3 oracle tests remain unchanged and passing;
- the runtime wrapper never calls the differential matcher to authorize work;
  and
- removing a derived gate causes binding/inventory failure while the existing
  runtime independently continues to deny or wait.

## Validation Expectations

Implementation acceptance requires at minimum:

- focused obligation-plan and immutable-bundle tests;
- focused executor integration tests;
- `cargo test -p workflow-core`;
- warnings-denied Clippy for `workflow-core` with all targets and features;
- formatting;
- documentation checks;
- `git diff --check`; and
- focused maintainer/security review of the exact implementation and tests.

Broader workspace validation may be required by changed surfaces or CI.

## Sequencing And Review Gates

1. Focused maintainer/security review of this plan.
2. One separately governed private source-extractor and binding implementation.
3. Focused review of extraction completeness, cardinality, binding, and
   redaction before executor composition.
4. One separately governed opt-in executor composition using the accepted
   extractor and binding.
5. Focused review and exact-head CI before Phase 4 acceptance.
6. Phase 5 measurement planning only after Phase 4 is accepted.

If review cannot establish one canonical extraction path from the immutable
bundle, the correct outcome is to stop Phase 4 rather than introduce a second
resolver.

## Explicit Non-Goals

This plan does not authorize:

- public schema, SDK, CLI, or serialization changes;
- persistence of the plan or its explanation;
- a new event, state transition, approval, evidence, report, or completion
  model;
- accepting caller-authored decisions or evidence as authoritative;
- changing default local execution behavior;
- provider reads or writes;
- automatic local command execution;
- new `SideEffect` semantics;
- hosted or distributed execution;
- nested harness execution;
- workflow or policy language changes;
- a `WorkContract` source language or GWP ABI;
- incremental recomputation, Salsa, a new dependency, or an MSRV change;
- release, publication, or product-readiness claims; or
- independent third-party assurance.

## Acceptance

Phase 4 is complete only when one opt-in local execution proves all of the
following:

- its private plan was derived from the exact immutable canonical bundle
  closure;
- its algorithm and complete source/inventory commitments are bound to the
  exact run and bundle root;
- new-run and continuation paths fail closed before action on any mismatch;
- authoritative runtime decisions and evidence paths are unchanged;
- event, wait, retry, denial, report, SideEffect, and completion behavior are
  unchanged;
- the plan adds inspectability without granting authority or satisfying work;
  and
- focused review plus exact-head CI accept the implementation.

