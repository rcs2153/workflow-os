# Governed Obligation Plan Reconciliation Plan

## Status

Proposed future lane after the active delegated-authority external-connector
work. This plan does not preempt the Active Delivery Queue and does not authorize
implementation.

## Context

The proposed Work Compiler research identifies a useful missing product
property: authored governance should be reducible to a deterministic,
inspectable account of the obligations that must be satisfied before work may
proceed or complete. Its strongest distinctions are:

- recompilation when authored governance changes;
- reevaluation when relevant evidence or facts change; and
- reauthorization when current policy, approval, or authority must be proven at
  the material action boundary.

Workflow OS already owns workflow definitions, policy decisions, required
context, capability and current-authority resolution, local checks, approvals,
SideEffects, immutable run bundles, durable events, proportional governance,
and WorkReports. Introducing a separately authored work language, independent
runtime semantics, or parallel completion model before reconciling those
contracts would create competing sources of governance truth.

## Decision Boundary

The future lane explores a **Governed Obligation Plan** derived from existing
canonical Workflow OS contracts.

The plan is:

- deterministic and inspectable;
- non-authorizing;
- rebuildable from canonical versioned inputs;
- bound to exact input commitments when used by a run;
- incapable of granting authority, satisfying evidence, or completing work;
- subordinate to current Core policy, authority, evidence, and completion
  decisions; and
- rejected when its semantics disagree with the canonical runtime path.

Workflow OS has one canonical governance semantics. A derived plan may explain,
index, or accelerate those semantics, but it may not create a second policy
authority, approval system, evidence truth store, run ledger, or completion
authority.

## Required Invariants

1. Core remains the only authority for policy, approval, capability, current
   authority, material actions, and terminal completion.
2. Existing workflows, policies, required-context contracts, check
   declarations, approval requirements, SideEffect requirements, and report
   contracts are the initial source language.
3. Derived plans are cache or projection data, never durable governance truth.
4. Every derived obligation identifies its canonical source definition and
   exact commitment.
5. Agent assertions cannot satisfy independent evidence or authority
   obligations.
6. Recompilation, reevaluation, and reauthorization remain distinct operations.
7. Changed canonical inputs require deterministic rederivation; changed facts
   require reevaluation; current authority is always resolved at the protected
   time of use.
8. Unknown, stale, unsupported, ambiguous, or conflicting obligations fail
   closed for any mandatory path.
9. Existing runtime behavior and persistence remain compatible unless a later
   reviewed contract change explicitly says otherwise.
10. Human-readable explanation must not overstate modeled, fixture-backed, or
    non-enforcing behavior.

## Phased Evaluation

### Phase 1: Architecture Reconciliation

Produce a concept-to-existing-Core matrix covering proposed contract, typed
resource, effect, obligation, evidence, check, approval, program, evaluation,
and completion concepts.

For every concept, identify:

- the existing canonical Core type or runtime path;
- whether the existing contract is enforced, validated, disclosed, advisory, or
  deferred;
- any genuine semantic gap;
- the authoritative source of truth;
- compatibility and migration implications; and
- whether the concept should be rejected as duplication.

This phase changes no runtime, public schema, dependency, MSRV, or CLI behavior.

### Phase 2: Model-Only Derived Plan

If reconciliation demonstrates a real gap, define the smallest private,
model-only obligation-plan representation derived from existing validated
contracts.

Acceptance requires:

- deterministic ordering and commitments;
- stable references to canonical source contracts;
- explicit mandatory, optional, unsupported, and unknown posture;
- no authority or satisfaction constructor exposed to callers;
- redaction-safe diagnostics and debug output;
- serialization only if a concrete immutable-run use requires it; and
- tests proving that malformed or incomplete source contracts cannot produce an
  executable or authorizing claim.

No separate `WorkContract` YAML language is introduced.

### Phase 3: Explanation And Differential Validation

Add a private deterministic explanation projection and compare the derived plan
against existing executor and validation decisions for representative
workflows.

The phase must prove:

- every derived obligation traces to canonical source input;
- no existing mandatory gate disappears;
- unsupported or deferred fields remain disclosed;
- plan and runtime decisions agree for accepted fixtures and negative cases;
- disagreement fails review rather than selecting whichever result is more
  permissive; and
- clean rederivation produces identical semantic output for identical inputs.

This phase does not create a public evaluator with competing runtime semantics.

### Phase 4: One Opt-In Runtime Composition

Only after focused review of differential evidence, bind one derived plan to an
existing immutable run bundle and consume it in one narrow opt-in runtime path.

The composition must preserve current policy evaluation, current-authority
resolution, approval proof, evidence verification, SideEffect reconciliation,
event history, and WorkReport completion. The derived plan may organize or
explain those requirements; it may not replace their authoritative checks.

### Phase 5: Incrementality Decision Gate

Measure clean derivation and repeated fact-driven reevaluation on realistic
Workflow OS workloads. Record contract counts, obligation counts, latency,
memory, invalidation frequency, and the proportion of time spent outside plan
derivation.

An incremental query engine may be proposed only when evidence shows that clean
derivation is a material product bottleneck and that selective invalidation
would improve an actual supported workflow. Any proposal must include clean
rebuild equivalence, restart behavior, dependency and supply-chain review,
toolchain compatibility, and rollback posture.

No specific engine is selected by this plan. Salsa is explicitly deferred. It
must not be added as a dependency, require an MSRV change, or become an
architecture invariant without a later evidence-backed ADR and focused
maintainer/security review.

## Explicit Deferrals

This plan does not authorize:

- a separately authored `WorkContract` source language;
- a public Governed Work Program bytecode or ABI;
- a standalone evaluator that competes with Core semantics;
- Salsa or another incremental query dependency;
- an MSRV or edition change;
- optimizer passes over governance obligations;
- compiler-specific public CLI or SDK surfaces;
- new provider reads or writes;
- automatic local command execution;
- workflow, policy, approval, evidence, or report schema changes;
- hosted or distributed compiler services; or
- preemption of the active delegated-authority external-connector lane.

## Exit Criteria

The lane may advance beyond reconciliation only when maintainers can answer all
of the following with repository evidence:

1. What user or runtime failure does the derived plan solve?
2. Why can the failure not be corrected directly in existing canonical
   contracts?
3. Which existing Core types remain authoritative for every derived field?
4. How is semantic disagreement detected and failed closed?
5. How does the plan reduce ceremony or improve explainability without adding a
   second authored contract?
6. What measured evidence, if any, justifies incremental computation?

If these questions cannot be answered, the correct outcome is to retain the
existing runtime model and close the exploratory lane without implementation.
