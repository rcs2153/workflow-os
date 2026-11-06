# Governed Obligation Plan Reconciliation Plan Review

Status: **accepted after blocker correction**

## Scope

This owner-conducted focused architecture and security review evaluates the
[Governed Obligation Plan Reconciliation
Plan](../implementation-plans/governed-obligation-plan-reconciliation-plan.md)
against current Workflow OS Core. It considers validated project definitions,
typed policy effects, required context, local checks, proportional governance,
approval proof, SideEffects, immutable run bundles, durable events, WorkReport
contracts and artifacts, and terminal completion.

This is repository assurance, not independent external security review. No
runtime, schema, SDK, CLI, dependency, provider, or package behavior changed.

## Findings

### Blocker: static requirements and runtime satisfaction were blurred

The draft described a derived plan as containing obligations while also
proposing later reevaluation and runtime consumption. Without a hard boundary,
the model could evolve into a second source of truth for whether approval,
evidence, policy, SideEffects, or completion had been satisfied.

Correction: the plan now permits only static requirement identity and canonical
source references in the derived model. Existing Core paths remain solely
authoritative for runtime fact resolution, satisfaction, denial, waiting, and
terminal completion.

### Blocker: immutable input closure was incomplete

The draft required exact input commitments when a plan is used by a run, but it
did not require the derivation algorithm version and complete canonical source
closure. A plan could otherwise be rederived from a changed checkout while
retaining a superficially stable workflow identity.

Correction: any run-bound commitment must cover the derivation algorithm and
every included canonical workflow, skill, policy, required-context, check,
report, and other source commitment. Runtime consumption may not rederive the
accepted plan from mutable files.

### Blocker: differential validation could be circular

The draft required plan/runtime agreement but did not prevent tests from using
the same derivation helper as both implementation and expected oracle. Such a
test would preserve a shared omission.

Correction: representative expected obligations and runtime outcomes must be
specified independently. Negative coverage must prove that a missing derived
gate is detected even when the canonical runtime still denies or waits.

### Important: existing proportional-governance derivation is close prior art

`proportional_governance_workflow_derivation` already resolves one validated
workflow step, skill, policy set, definition root, static action class,
sensitivity, declared minima, and explicit unknown runtime facts. The new lane
must reconcile and reuse that canonical semantics rather than wrap it in a
parallel evaluator.

Correction: Phase 1 names this boundary explicitly and may conclude that no new
model is justified.

### Important: SideEffect observations are not authored obligations

Current SideEffect records and discovery describe governed runtime effects.
Write-capable skill declarations and inferred side-effect posture do not by
themselves constitute a canonical authored SideEffect obligation contract.

Correction: the Phase 1 matrix must record the absence of an authored source as
a gap or reject the proposed mapping. It may not manufacture an obligation from
runtime observation.

## Accepted Architecture

The corrected plan preserves one governance system:

- validated Workflow OS contracts are the source language;
- the derived plan, if justified, is private, deterministic, non-authorizing,
  and rebuildable;
- Core policy, authority, approval, evidence, SideEffect, event, and completion
  paths remain authoritative;
- immutable use commits the complete source closure and algorithm version;
- semantic disagreement fails closed against the canonical runtime; and
- incremental computation remains an evidence-gated optimization decision,
  not an architectural premise.

The product problem is bounded to inspectability and explainability: can the
requirements distributed across existing contracts be presented as one
deterministic pre-execution inventory and reconciled with terminal evidence?
If existing projections already solve that problem, the correct outcome is to
close the lane without creating a new model.

## Explicitly Rejected In This Review

- a new `WorkContract` YAML language;
- a public bytecode or Governed Work Program ABI;
- a competing evaluator or completion engine;
- caller-authored satisfaction state;
- a second evidence, approval, or policy store;
- Salsa or another incremental query dependency;
- MSRV or edition changes;
- provider behavior, command execution, or hosted compilation; and
- public schema, SDK, or CLI surfaces.

## Validation

- source-reference inspection against current Core definitions, policy,
  required-context, local-check, proportional-governance, immutable-bundle,
  approval, SideEffect, WorkReport, event, and executor paths;
- `npm run check:docs`;
- `git diff --check`.

Runtime tests are intentionally not required because the review changes
documentation only.

## Governed Review Record

- Workflow: `dg/review`
- Run: `run-1793950016896002000-2`
- Approval: `approval/run-1793950016896002000-2/review-scope-approved`
- Approval presentation: `presentation/679e862da30b6b29`
- Presentation content hash:
  `679e862da30b6b29b223b3e2b4410be2e3b43ef2301636b05006261add60f8f9`
- Approval outcome: granted under delegated-maintainer authority after the
  complete proof-backed handoff was presented
- Phase result: completed
- Event summary: 39 ordered events, one approval request, one approval grant,
  eight policy decisions, six scheduled steps, six requested, started, and
  succeeded skill invocations, zero retries, and zero escalations
- Approval presentation enforcement: `proof_enforced`, one persisted record,
  matching event marker present

Workflow OS governed the review scope and approval. Source inspection,
documentation edits, validation, Git, PR, and merge work occur outside the
kernel and must remain disclosed. No runtime behavior, check, WorkReport,
provider action, or independent review was simulated by the governed run.

## Verdict And Next Phase

Accepted after correction. Phase 1 architecture reconciliation is the next
authorized increment. It must produce the concrete concept-to-Core matrix and
may close the exploratory lane without implementation if no genuine semantic
gap remains. Phase 2 and later work remain unauthorized pending separate review.
