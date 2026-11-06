# Governed Obligation Plan Reconciliation Plan

## Status

Accepted after focused maintainer/security review. Phase 1 architecture
reconciliation is authorized; later phases remain contingent on its evidence
and separate focused review.

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
11. Static obligation derivation and runtime satisfaction are separate. A
    derived plan may state what must be established, but only existing Core
    policy, authority, approval, evidence, SideEffect, event, and completion
    paths may establish whether it is satisfied.
12. Any run-bound plan commitment must cover the exact validated canonical
    input closure and derivation algorithm version. Runtime code must not
    rederive an accepted plan from a mutable checkout.
13. Differential validation must use independently specified expected
    obligations and canonical runtime outcomes. Reusing the same derivation
    helper on both sides is not evidence of semantic agreement.
14. Existing proportional-governance workflow derivation is a canonical source
    candidate that must be reconciled, not wrapped or copied into a competing
    obligation evaluator.

## Phased Evaluation

### Phase 1: Architecture Reconciliation

Produce a concept-to-existing-Core matrix covering proposed contract, typed
resource, effect, obligation, evidence, check, approval, program, evaluation,
and completion concepts.

The matrix must explicitly include the existing validated project bundle,
workflow and step definitions, resolved skill and policy closure, typed policy
effects, required-context contracts and consumption, canonical local-check
declaration sets and result references, proportional-governance workflow
derivation and immutable-bundle assessment, approval presentation and decision
proof, SideEffect records and discovery, immutable run bundles, durable events,
WorkReport contracts and artifacts, and terminal run projection.

For every concept, identify:

- the existing canonical Core type or runtime path;
- whether the existing contract is enforced, validated, disclosed, advisory, or
  deferred;
- any genuine semantic gap;
- the authoritative source of truth;
- compatibility and migration implications; and
- whether the concept should be rejected as duplication.

The matrix must also classify each fact as one of:

- authored requirement;
- derived static requirement;
- runtime fact;
- runtime satisfaction or denial;
- durable evidence/reference;
- disclosure-only posture; or
- unsupported/deferred.

It must not infer an authored SideEffect obligation merely because a runtime
SideEffect record or write-capable skill exists. Where no canonical authored
source exists, the matrix must record a gap or reject the proposed mapping.

Phase 1 must answer one concrete user/runtime failure: whether governance
requirements distributed across canonical contracts cannot currently be
inspected as one deterministic pre-execution obligation inventory and compared
with terminal evidence without weakening or duplicating runtime authority. If
the existing projection and report surfaces already solve that failure, the
lane closes without a new model.

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

The model may contain requirement identity and canonical source references. It
must not contain approval-granted, evidence-passed, SideEffect-completed,
policy-allowed, or run-completed state. Those are runtime outcomes projected by
existing authoritative paths.

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

Expected obligations and expected runtime outcomes must be specified
independently from the implementation under test. At least one negative fixture
must prove that removing a canonical gate from the derived plan is detected even
when runtime execution still correctly denies or waits.

This phase does not create a public evaluator with competing runtime semantics.

### Phase 4: One Opt-In Runtime Composition

Only after focused review of differential evidence, bind one derived plan to an
existing immutable run bundle and consume it in one narrow opt-in runtime path.

The immutable binding must commit to the derivation algorithm version and the
complete canonical source closure used by the plan. A changed workflow, skill,
policy, required-context contract, local-check declaration, report contract, or
other included canonical source invalidates the binding before consumption.

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
