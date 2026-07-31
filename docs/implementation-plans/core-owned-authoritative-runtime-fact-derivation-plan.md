# Core-Owned Authoritative Runtime-Fact Derivation Plan

Status: Implemented and accepted.

Related foundations:

- [Engineering Standard](../ENGINEERING_STANDARD.md)
- [Proportional Governance And Quiet Success Plan](proportional-governance-quiet-success-plan.md)
- [Authoritative Local-Check Reassessment Binding Plan](authoritative-local-check-reassessment-binding-plan.md)
- [Authoritative Proportional-Governance Route Dispatcher Plan](authoritative-proportional-governance-route-dispatcher-plan.md)
- [Local Project Authority Source Runtime Composition Review](../concepts/LOCAL_PROJECT_AUTHORITY_SOURCE_RUNTIME_COMPOSITION_REVIEW.md)
- [Authoritative Governance Scaffold Default Activation Plan](authoritative-governance-scaffold-default-activation-plan.md)

## 1. Executive Summary

Workflow OS has one closed authoritative execution profile:

```text
observe_and_report
+ workflow_os_project_validation
+ project-declared local authority
+ same-call local-check attestation
+ proportional route selection
```

Core already binds project activation from the immutable run bundle, rejects
caller-preclassified authority, executes the canonical local check, and derives
the selected step's evidence/check posture from the resulting attestation.

Two caller-owned decisions remain:

1. the CLI builds facts for every workflow step and classifies non-selected
   steps as evidence/check `Satisfied`; and
2. the CLI predicts visible disclosure from optimistic facts before the actual
   check result exists so it can decide whether to pass visible-delivery
   dependencies.

This phase removes those decisions for the closed profile. It constrains that
profile to one immutable workflow step, constructs one unresolved fact inside
Core, derives authority, evidence/check, and side-effect posture from canonical
sources, and conditionally consumes visible-delivery capability only when the
actual complete assessment selects visible proceed.

This is a runtime-composition fix. It does not change scaffold defaults.

## 2. Goals

The implementation must:

1. make Core the sole constructor of runtime facts for the closed authoritative
   project-validation route;
2. require the immutable workflow to contain exactly one step;
3. require that step to be the selected canonical project-validation step;
4. reject caller-supplied runtime facts on the new closed request surface;
5. derive authority from validated immutable project activation;
6. derive evidence/check posture only from the same-call accepted local-check
   result;
7. derive side-effect posture from the immutable skill capability declaration;
8. let the actual complete assessment select quiet, visible, approval-required,
   or denied behavior;
9. consume visible-delivery dependencies only for visible proceed without
   requiring the CLI to predict the route;
10. preserve authored step approvals as a separate later gate;
11. fail before workflow execution when the immutable workflow has multiple
    steps or the selected declaration is not exact; and
12. keep errors stable, bounded, and non-leaking.

## 3. Strict Non-Goals

This phase does not authorize:

- authoritative multi-step governance;
- caller-supplied `Satisfied` facts for unobserved steps;
- scaffold default activation or migration;
- additional local-check profiles;
- inferred repository commands or arbitrary shell execution;
- automatic or delegated approval;
- new governance profiles or decision semantics;
- OpenShell or another execution provider;
- provider credentials, sandbox execution, or network access;
- SideEffect execution, external writes, or write-capable adapters;
- hosted administration or enterprise stewardship;
- schemas, SDK changes, examples, or release posture changes; or
- recursive agents, agent swarms, or Level 3/4 autonomy.

## 4. Trust Boundary

The closed request should carry execution identity, selected step identity,
profile, immutable-bundle inputs, optional expected assessment fingerprint,
project activation, and optional visible-delivery capability.

It must not carry authoritative workload facts.

Core derives the one neutral fact only after:

- loading and validating the project;
- building the immutable run bundle;
- resolving exactly one immutable workflow step;
- confirming the canonical local-check declaration;
- confirming the fixed profile and project activation; and
- preflighting the local-check contract.

No process starts before the complete structural preflight passes.

## 5. One-Step Boundary

The first accepted profile is intentionally one-step.

The immutable workflow must contain exactly one step, and that step must:

- equal the request's selected step;
- carry exactly the canonical `workflow-os validate` requirement;
- resolve one immutable skill and its relevant policies; and
- remain covered by the existing immutable-run binding.

A multi-step workflow fails with a stable
`executor.authoritative_local_check.workflow_shape_unsupported` error before
local process execution, run creation, or bundle persistence.

This is not a permanent product limitation. It prevents false governance until
each step can receive independently sourced authority, evidence/check, and
side-effect facts.

## 6. Runtime-Fact Derivation

Core creates one neutral fact:

```text
authority: unresolved
evidence_and_checks: unresolved
side_effect: unresolved
prior_execution: none
prior_disclosure: none
steward_minimum: none
```

Then:

- immutable project activation supplies the accepted local authority posture;
- same-call local-check composition supplies evidence/check posture;
- immutable skill capabilities derive side-effect posture through the existing
  proportional-governance derivation;
- immutable workflow and policy declarations derive minima and runtime
  escalation;
- prior and steward facts remain absent for this fresh-run-only route.

The CLI must not duplicate this derivation.

## 7. Conditional Visible Delivery

The route accepts an optional bounded visible-delivery capability. Core may
consume it only after the complete assessment selects:

```text
Proceed + Visible
```

Behavior:

- visible proceed without capability: fail closed before workflow execution;
- visible proceed with capability: deliver once and persist the existing
  receipt;
- quiet proceed with capability: do not invoke the handler;
- approval-required with capability: do not invoke the handler;
- denied with capability: do not invoke the handler.

Unused availability is not an execution or disclosure event. Tests must prove
zero handler calls on non-visible routes.

## 8. CLI Integration

The CLI should:

- resolve the exact selected project-validation step identifier;
- stop constructing `Vec<StepGovernanceRuntimeFacts>`;
- remove `authoritative_visible_disclosure_required(...)`;
- make one bounded local terminal disclosure handler available to Core;
- pass the fixed profile and immutable execution inputs; and
- render the route returned by Core.

The CLI does not decide the route and does not assert check success.

## 9. Compatibility

Existing lower-level proportional-governance APIs may retain explicit runtime
facts for test, embedding, and future independently sourced callers.

The closed authoritative project-validation request should become a separate
fact-free boundary. Existing public request fields should not be silently
reinterpreted. Prefer a new request type or a private conversion that makes
fact ownership explicit and prevents ordinary callers from supplying facts.

The current opt-in scaffold remains opt-in throughout this phase.

## 10. Error And Privacy Posture

Errors must not include:

- workflow or step payloads;
- file paths;
- command output;
- environment values;
- policy bodies;
- credentials or tokens; or
- disclosure content.

New errors use stable `executor.authoritative_local_check.*` or
`cli.authoritative_governance.*` codes.

Debug output reports counts and posture only. It does not render execution
inputs, identities, runtime facts, check output, or delivery text.

## 11. Test Plan

Focused tests must prove:

1. one-step authoritative execution reaches quiet proceed;
2. one-step visible execution invokes the disclosure handler exactly once;
3. quiet, approval-required, and denied routes do not invoke an available
   disclosure handler;
4. visible execution without delivery capability fails closed;
5. multi-step workflows fail before process use and run creation;
6. callers cannot provide authority or evidence/check facts through the closed
   request;
7. selected evidence/check posture comes from the actual check result;
8. failed checks cannot be converted into visible or quiet success by CLI
   prediction;
9. immutable skill capability derives side-effect posture;
10. authored workflow approvals remain separate;
11. approval reassessment repeats the exact structural and same-call checks;
12. existing lower-level explicit-fact APIs remain compatible;
13. Debug and errors are non-leaking;
14. focused CLI tests cover the removal of route prediction; and
15. workspace Rust, integration, and documentation checks pass.

## 12. Implementation Sequence

1. Add the fact-free closed authoritative request boundary.
2. Add pure one-step immutable workflow-shape preflight.
3. Derive the neutral fact inside Core.
4. Bind project-declared authority and same-call check posture.
5. change visible-delivery input from caller route prediction to conditional
   Core consumption.
6. Migrate the CLI closed path and remove duplicate prediction helpers.
7. Add focused Core and CLI regression tests.
8. Update roadmap, reports, and current-product documentation.
9. Run full validation and a disposable external-repository proof.
10. Perform focused maintainer review before returning to default activation.

## 13. Final Recommendation

The bounded prerequisite is implemented. Core now owns the neutral runtime fact,
one-step immutable shape validation, current-authority binding, same-call check
posture, side-effect derivation, and final route selection for the closed
project-validation profile. The CLI no longer constructs runtime facts or
predicts visible disclosure.

Return to authoritative scaffold default activation after the implementation
report and review.
Do not broaden to multi-step authoritative governance, providers, OpenShell,
SideEffects, or writes in this phase.
