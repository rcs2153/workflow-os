# Governed Obligation Plan Phase 4 Runtime Composition Plan

## Status

Corrected after focused maintainer/security review requested changes. Runtime
implementation remains unauthorized pending a second focused review of the
complete source closure, projection binding, persistence boundary, shared
invariants, and consumer gate below.

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

## Corrected Canonical Input Closure

The immutable run bundle is the mandatory execution base, but it is not the
complete obligation source closure.

The bundle-owned portion consists of:

- `ImmutableRunBundleManifest` for run identity, resolved execution-context
  commitment, ordered definition references, canonical local-check references,
  handler posture, execution posture, and bundle integrity root;
- `ImmutableRunBundleDefinitionRecord` for canonical validated typed workflow,
  skill, and policy definitions; and
- `CanonicalLocalCheckDeclarationSetRecord` for authoritative per-step
  local-check declarations.

Required context is not embedded in those definition records. The complete
private extraction input must therefore be one module-owned
`GovernedObligationCanonicalInputClosure` assembled only from:

- one `StoredImmutableRunBundle` already validated by
  `ImmutableRunBundleStore`;
- for every workflow step, exactly one existing
  `RequiredContextContractBinding` and matching
  `RequiredContextExecutionBinding` validated against that same stored bundle,
  step, actor, and sensitivity posture; and
- no other caller-defined source family.

An explicit-none required-context contract is a real canonical input, not the
absence of input. Missing per-step posture is unknown or unsupported and fails
closed. The extractor must not infer zero required context from a missing
argument.

The closure constructor must be module-owned. It must reuse existing immutable
store resolution and record-bijection validation, then validate each required-
context execution binding through its existing Core contract. It must not
reload YAML, inspect the mutable project checkout, accept an independently
assembled `ProjectBundle`, or accept caller-supplied obligations, decisions,
evidence, or satisfaction state.

Workflow-authored `ReportArtifactRequirements` are available in the canonical
workflow record and may contribute static obligations. A runtime
`WorkReportContract`, report-generation request, or stored artifact is not part
of this first static source closure and cannot be inferred or substituted.

## Private Projection Binding Model

Add one crate-private, non-serializing
`GovernedObligationPlanProjectionBinding` containing only:

- `GovernedObligationPlanAlgorithm`;
- the immutable run bundle ID, version, and root hash or the existing
  `ImmutableRunBundleBinding` value;
- the bound `WorkflowRunId`;
- sorted commitments for every accepted required-context contract and
  execution binding;
- the plan source-closure commitment;
- the plan inventory commitment; and
- the obligation count.

The projection binding is static inventory identity only. It is not a
`GovernanceAssessmentBinding`, capability, approval, policy
decision, evidence receipt, event, WorkReport, or terminal-state record. It is
not separately persisted in Phase 4. The exact bundle remains durable truth;
required-context contracts and execution bindings remain their own canonical
inputs; and the existing durable `GovernanceAssessmentBinding` remains the
only accepted proportional-governance assessment binding. The projection must
not contain execution disposition, disclosure, completeness, current facts,
authority, approval, evidence, or satisfaction.

The projection-binding constructor is module-owned. Callers cannot provide a
bundle root, required-context commitment, source commitment, inventory
commitment, run identity, or obligation count independently. Construction
succeeds only from one validated canonical input closure and one plan derived
from that exact closure.

## Canonical Extraction Rules

Create one crate-private extractor that accepts
`&GovernedObligationCanonicalInputClosure` and returns one plan plus its
private projection binding. The extractor must:

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
5. Derive required-context obligations only from the exact validated per-step
   contract and execution-binding pairs in the closure. Omitted, substituted,
   stale, wrong-step, wrong-actor, wrong-bundle, and duplicate bindings fail
   closed. Explicit none remains an explicit canonical source.
6. Derive local-check obligations only from canonical local-check declaration
   sets. An absent declaration remains absence; runtime check observations do
   not manufacture a static obligation.
7. Derive report obligations only from workflow-authored
   `ReportArtifactRequirements` in the canonical workflow record. It must not
   treat a runtime `WorkReportContract`, generation request, or artifact as an
   authored static source.
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

The first extraction and any later runtime composition must be bounded before
allocation and before run creation.

- Expose or reuse one crate-private immutable-bundle reference bound; do not
  copy the literal into the obligation module.
- Derive the canonical-source and obligation bounds from that shared bound.
- Bound required-context pairs by the validated workflow step count and the
  same shared outer bound.
- Permit at most one obligation for one source/kind pair.
- Retain the existing 256-byte source-identity bound.
- Reject arithmetic overflow and any input that exceeds a bound before
  constructing the complete plan.

These are conservative Phase 4 limits, not a new public contract. Increasing
them requires measured evidence and focused review. Truncation is forbidden.

## Runtime Consumer Gate

The first corrected implementation stops at the private canonical-input
closure, extractor, and projection binding. It does not add an executor
wrapper.

A later separately governed plan amendment must name one exact existing
runtime path that already owns:

- the exact stored immutable run bundle;
- complete per-step required-context contracts and execution bindings;
- the existing durable `GovernanceAssessmentBinding` and its current-fact
  reassessment lifecycle; and
- all existing approval, evidence/check, `SideEffect`, event, report, retry,
  wait, replay, and completion behavior required by the selected workflow.

If an existing path does not own that complete closure, executor composition
remains blocked. A generic wrapper may not add parallel caller-injected inputs
or bypass established reassessment. Any future composition must use
`execute_with_governance_assessment_binding`,
`execute_with_current_runtime_facts_governance_assessment_binding`, or a
focused-review-accepted common internal stage factored from those paths. The
private plan projection must be evaluated alongside the durable governance
assessment; it cannot replace, satisfy, or weaken it.

For a future new run, all deterministic source extraction, bounds, and
projection-binding checks must complete in memory before bundle persistence.
After persistence, read-back and rederivation must complete before any runtime
event. A post-persistence failure may leave the create-only immutable bundle as
an unactivated record, but it creates no run event, approval, invocation,
WorkReport, `SideEffect`, or external action.

For a future existing run, continuation must use only the stored immutable
bundle and exact existing required-context execution bindings. It must
independently reproduce both the durable governance assessment and private
plan projection before delegating unchanged to the selected continuation path.
An exact retry may validate and reuse an identical unactivated bundle; it must
not overwrite, delete, or silently replace it. Automatic orphan cleanup is
outside Phase 4.

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
- run identity mismatch;
- obligation inventory disagreement;
- required-context binding missing, substituted, stale, or mismatched; and
- plan projection and governance-assessment lifecycle disagreement.

Messages must be payload-free and redaction-safe. The errors must not include
definition IDs, source identities, hashes, policy contents, check commands,
approval reasons, evidence contents, or local paths.

## Required Tests

### Source extraction

- identical canonical input closures produce identical source and inventory
  commitments;
- filesystem, map, and insertion-order changes do not affect output;
- every supported workflow step, policy, required-context contract,
  declaration set, and report contract is represented exactly once;
- explicit-none required context is represented as a canonical source;
- missing, substituted, duplicated, stale, wrong-step, wrong-actor, and
  wrong-bundle required-context bindings fail closed;
- absent authored requirements do not become obligations;
- runtime `SideEffect` facts do not become obligations;
- missing, extra, duplicate, conflicting, unsupported, and unresolved records
  fail closed;
- every cardinality boundary accepts the maximum and rejects maximum plus one;
  and
- Debug and error output reveal no protected identity or payload.

### Immutable binding

- the plan projection binds to the exact bundle root, version, run ID,
  required-context closure, algorithm, source closure, inventory, and count;
- changing workflow, skill, policy, required context, local checks, report
  posture, resolved context, handler posture, execution posture, or any
  included canonical record invalidates the binding;
- a plan from run or bundle A cannot be consumed for run or bundle B;
- an arbitrary caller-supplied `PolicyDecision`, assessment, obligation ID, or
  commitment cannot establish provenance;
- stored-bundle rederivation matches clean in-memory derivation; and
- the plan projection cannot substitute for, weaken, or satisfy a missing,
  stale, incomplete, denied, or changed `GovernanceAssessmentBinding`.

### Future runtime composition

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
- pre-persistence extraction or binding failure creates no immutable bundle,
  run event, approval, handler invocation, artifact, or external effect;
- post-persistence read-back or rederivation failure may retain only the exact
  unactivated immutable bundle and creates no run event, approval, invocation,
  WorkReport, `SideEffect`, or external action; and
- exact retry validates and reuses an identical unactivated bundle or fails
  closed without overwrite or deletion.

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
2. One separately governed private canonical-input-closure, source-extractor,
   and projection-binding implementation only.
3. Focused review of extraction completeness, cardinality, binding, and
   redaction before executor composition.
4. One separately governed plan amendment naming an exact consumer that owns
   required context and durable governance reassessment; if no such path
   exists, stop Phase 4 runtime composition.
5. One separately governed opt-in executor composition using the accepted
   extractor, projection, exact consumer, and unchanged authoritative gates.
6. Focused review and exact-head CI before Phase 4 acceptance.
7. Phase 5 measurement planning only after Phase 4 is accepted.

If review cannot establish one canonical extraction path from the immutable
bundle plus existing exact required-context bindings, or cannot name a
consumer that already owns the complete closure, the correct outcome is to
stop Phase 4 rather than introduce a second resolver or assessment lifecycle.

## Explicit Non-Goals

This plan does not authorize:

- public schema, SDK, CLI, or serialization changes;
- persistence of the plan, projection binding, or explanation;
- modification or replacement of `GovernanceAssessmentBinding`;
- automatic cleanup of unactivated immutable bundles;
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

- its private plan was derived from the exact immutable bundle plus complete
  validated per-step required-context closure;
- its algorithm and complete source/inventory commitments are bound to the
  exact run and bundle root;
- new-run and continuation paths fail closed before action on any mismatch;
- the private projection remains subordinate to the existing durable
  governance-assessment lifecycle;
- authoritative runtime decisions and evidence paths are unchanged;
- event, wait, retry, denial, report, SideEffect, and completion behavior are
  unchanged;
- the plan adds inspectability without granting authority or satisfying work;
  and
- focused review plus exact-head CI accept the implementation.

