# Authoritative Proportional-Governance Route Dispatcher Plan

Status: Implemented and accepted.

Related foundations:

- [Proportional Governance And Quiet Success Plan](proportional-governance-quiet-success-plan.md)
- [Authoritative Proportional-Governance Executor Routing Plan](authoritative-proportional-governance-executor-routing-plan.md)
- [Authoritative Proportional Governance Routing Review](../concepts/AUTHORITATIVE_PROPORTIONAL_GOVERNANCE_ROUTING_REVIEW.md)
- [Authoritative Proportional-Governance Route Dispatcher Plan Review](../concepts/AUTHORITATIVE_PROPORTIONAL_GOVERNANCE_ROUTE_DISPATCHER_PLAN_REVIEW.md)
- [Authoritative Proportional-Governance Route Dispatcher Report](../concepts/AUTHORITATIVE_PROPORTIONAL_GOVERNANCE_ROUTE_DISPATCHER_REPORT.md)
- [Authoritative Proportional-Governance Route Dispatcher Review](../concepts/AUTHORITATIVE_PROPORTIONAL_GOVERNANCE_ROUTE_DISPATCHER_REVIEW.md)
- [Authoritative Local-Check Executor Consumer Plan](authoritative-local-check-executor-consumer-plan.md)

## 1. Executive Summary

Workflow OS now has four accepted explicit local executor routes for complete,
source-bound proportional-governance assessments:

- `Proceed + Quiet`;
- `Proceed + Visible`;
- `RequireApproval + Visible`; and
- `Denied + Visible`.

Each route recomputes the same-call authoritative assessment and rejects any
other result. That is safe, but callers still select which route API to call.
The desired product boundary is one explicit dispatcher where the derived
assessment selects the accepted route.

The first dispatcher must remain local, fresh-run-only, `DocsCheck`-only, and
additive. It must prepare the assessment once, match the exact normalized
outcome, and reuse the accepted route behavior without weakening the existing
per-route checks.

This plan does not implement Rust, CLI or UI behavior, schemas, providers,
OpenShell, SideEffect execution, writes, hosted behavior, enterprise
administration, reasoning lineage, or release changes.

## 2. Product Decision

The caller may supply workflow inputs, current typed runtime facts, and
explicit route dependencies. The caller may not select the governance result.

The authoritative assessment chooses:

```text
Proceed + Quiet            -> execute quietly
Proceed + Visible          -> deliver bounded disclosure, then execute
RequireApproval + Visible  -> pause at proof-enforced approval
Denied + Visible           -> fail before step scheduling
```

Incomplete or invalid normalized outcomes fail closed.

This converts proportional governance from four integration-specific entry
points into one routing boundary while keeping the existing entry points
available for compatibility and focused testing.

## 3. Goals

- Prepare one authoritative assessment per dispatcher invocation.
- Route from the complete source-bound assessment, not a caller enum.
- Reuse accepted route behavior and event ordering.
- Require visible-proceed delivery dependencies only when that route is
  selected.
- Preserve exact per-route validation as defense in depth.
- Return a typed outcome that cannot misstate approval or denial as execution
  success.
- Keep all existing executor APIs unchanged.

## 4. Non-Goals

The phase must not add:

- default or automatic invocation from `LocalExecutor::execute(...)`;
- CLI, UI, workflow-schema, or example exposure;
- a new governance mode or proportional decision input;
- a second approval or denial subsystem;
- automatic approvers or model self-approval;
- retry, resume, or existing-run support;
- asynchronous or durable disclosure delivery;
- report or artifact generation;
- provider execution, OpenShell, sandbox lifecycle, or credentials;
- SideEffect execution or provider mutation;
- hosted behavior, enterprise policy administration, reasoning lineage, or
  release changes.

## 5. Candidate API Shape

Add the smallest public additive API consistent with the existing explicit
executor surfaces:

```text
route_authoritative_docs_check_governance(...)
    -> Result<LocalExecutionWithAuthoritativeGovernanceRouteResult, WorkflowOsError>
```

Candidate inputs:

- `LocalExecutionWithAuthoritativeDocsCheckGovernanceRequest`;
- immutable run bundle store;
- explicit `DocsCheckLocalHandler`;
- optional visible-proceed delivery input; and
- optional injected `GovernanceDisclosureDeliveryHandler`.

The implementation may use one borrowed dependency struct to keep the
function signature bounded. That struct must not contain a route enum.

The typed result should be an enum with distinct variants:

- `QuietProceed`;
- `VisibleProceed`;
- `ApprovalRequired`; and
- `Denied`.

Each variant should reuse or contain the corresponding existing result type.
Common read-only accessors may expose run, bundle binding, assessment binding,
and local-check results without erasing the route-specific result.

## 6. One-Pass Preparation Boundary

The dispatcher must invoke the existing authoritative preparation exactly
once:

1. require a fresh run;
2. resolve and validate the execution plan;
3. build and create-only claim the immutable run bundle;
4. run the canonical `DocsCheck`;
5. derive the complete source-bound aggregate assessment; and
6. return one prepared internal value.

It must not call one of the existing public route functions after preparation,
because doing so would rebuild the bundle and rerun the check.

Refactor route-specific post-preparation behavior into private consumers:

- consume prepared quiet proceed;
- consume prepared visible proceed;
- consume prepared approval required; and
- consume prepared denial.

The existing public APIs should prepare once and call their corresponding
private consumer. The dispatcher should prepare once, inspect the assessment,
and call the selected private consumer.

## 7. Route Dependency Boundary

Only `Proceed + Visible` needs an injected non-blocking disclosure surface in
the accepted first slice.

The dispatcher should therefore accept an optional visible-proceed dependency
bundle containing:

- bounded delivery input; and
- injected delivery handler.

Rules:

- visible proceed without both dependencies fails before run events or skill
  execution;
- partial dependency configuration fails closed;
- quiet proceed must not invoke the handler;
- approval required must use approval-presentation proof, not the
  non-blocking disclosure handler;
- denial must not invoke a surface that claims successful observation;
- unused visible dependencies on another selected route should be rejected,
  not silently ignored.

This avoids ambient execution capabilities and keeps presentation boundaries
truthful.

## 8. Typed Result Semantics

The dispatcher result must preserve route truth:

- quiet and visible proceed variants may contain completed, failed, canceled,
  or non-terminal ordinary executor results according to existing semantics;
- approval-required returns the waiting-for-approval result and aggregate
  approval binding;
- denied returns the terminal failed result with `PolicyDenied`;
- a route-selection or dependency error returns `Err` and no fabricated run
  result.

`Debug` must disclose only bounded route posture, run status, and counts. It
must not expose identifiers, fingerprints, paths, check output, disclosure
metadata, approval context, or payloads.

## 9. Monotonicity And Defense In Depth

The dispatcher must match only normalized complete pairs.

It must not:

- turn incomplete facts into quiet proceed;
- turn visible into quiet;
- turn approval or denial into proceed;
- infer delivery from an available handler;
- infer approval from an actor or profile;
- accept `RequireApproval + Quiet` or `Denied + Quiet`; or
- reconstruct governance from a caller projection.

The existing route enforcement helpers should remain and execute inside each
private consumer. This provides a second fail-closed boundary if dispatcher
matching is changed incorrectly later.

## 10. Event And Failure Semantics

The dispatcher must preserve accepted event ordering:

- quiet proceed: binding, ordinary run events, execution;
- visible proceed: surface acceptance before ordinary run events, then
  binding and execution;
- approval required: binding, ordinary start, approval request, pause before
  step scheduling;
- denied: binding, ordinary start, denial-specific `RunFailed` before step
  scheduling.

The dispatcher should add no new workflow event kind.

Route dependency errors should use stable non-leaking
`executor.authoritative_local_check.route.*` codes. Existing route-specific
errors and terminal failure codes should remain stable.

## 11. Crash And Recovery Posture

The first dispatcher inherits the accepted route limitations:

- create-only immutable bundle or assessment residue may remain after a later
  failure;
- visible delivery receipt is in memory;
- fresh-run-only APIs do not recover or resume dispatcher state; and
- route dependency failure after preparation may leave bounded immutable
  residue but no execution authority.

The implementation report must state exactly when route dependencies are
validated relative to immutable bundle claiming. Cheap structural dependency
validation should happen before canonical check execution when possible.

No recovery or retry behavior may be invented in this phase.

## 12. Privacy And Redaction

Inputs, results, errors, `Debug`, serde, events, and reports must not copy:

- raw source or spec contents;
- local-check output;
- commands, arguments, environment values, or paths;
- assessment reason payloads or fingerprints;
- disclosure or approval prose;
- provider payloads or logs; or
- credentials, authorization headers, private keys, or tokens.

Unknown or inconsistent values must fail without echoing caller data.

## 13. Compatibility

Keep the four existing public route APIs and their behavior unchanged.

The dispatcher is additive. It must not change:

- `LocalExecutor::execute(...)`;
- step-level approvals;
- hooks;
- reports or artifacts;
- provider or SideEffect paths;
- state rehydration;
- CLI behavior; or
- schemas and examples.

Route-local tests remain mandatory even after dispatcher tests are added.

## 14. Test Plan

Future tests must prove:

1. quiet assessment selects quiet proceed;
2. visible proceed selects delivery and invokes the surface exactly once;
3. approval assessment selects the existing aggregate approval pause;
4. denied assessment selects terminal `PolicyDenied`;
5. one canonical check execution occurs per route;
6. no caller route enum exists;
7. visible dependency absence or partial configuration fails closed;
8. visible dependencies supplied to quiet, approval, or denial fail closed;
9. the dispatcher cannot downgrade visible, approval, or denial;
10. incomplete and invalid normalized states fail closed;
11. event ordering matches route-local behavior;
12. no extra step, skill, approval, provider, or SideEffect activity appears;
13. typed result `Debug` is non-leaking;
14. route errors do not echo IDs, paths, checks, or secret-like values;
15. all existing route-local tests still pass;
16. existing approval, immutable bundle, local-check, report, provider,
    SideEffect, event, runtime, and CLI tests still pass; and
17. `cargo test --workspace` passes.

## 15. Implementation Sequence

Use one bounded implementation phase:

1. Introduce the typed dispatcher result and minimal optional dependency
   input.
2. Refactor accepted post-preparation behavior into private route consumers
   without changing public APIs.
3. Add the dispatcher match over the exact assessment pair.
4. Add focused all-route, one-check, dependency, event, and privacy tests.
5. Run full validation.
6. Create an implementation report.
7. Perform a focused maintainer review before operator UX or provider work.

If the refactor changes existing route behavior or requires new persistence,
split a blocker fix rather than broadening this phase.

## 16. Open Questions

- Should unused visible dependencies be rejected before immutable bundle
  claiming or after the authoritative route is known?
- Should the dependency input own the handler and delivery metadata together,
  or should the handler remain a separate borrowed parameter?
- Which common result accessors are useful without erasing route truth?
- Should the dispatcher remain a free function or become an explicit
  `LocalExecutor` method after the first review?
- When should a later default executor path opt into this dispatcher?
- What durable receipt boundary is required before retry, resume, or hosted
  operation?

## 17. Relationship To Operator UX

The dispatcher is the prerequisite for quiet-success product UX.

A concise local UI may:

- show quiet evidence without changing governance;
- display visible disclosures as they are delivered;
- render approval cards from durable presentation context; and
- show denials from durable assessment and failure state.

UI code must project the authoritative route. It must not select or weaken it.

## 18. Relationship To OpenShell

OpenShell remains a later optional execution substrate.

A future adapter may receive only an invocation already authorized by this
routing and scoped-capability boundary. It may return bounded sandbox,
effective-policy, exit, denial, log, and artifact references for Workflow OS
evidence and reports.

Do not fork OpenShell or add an execution-provider integration in this phase.
Provider containment cannot substitute for Workflow OS policy, approval,
evidence, or authority decisions.

## 19. Final Recommendation

Implement the authoritative local dispatcher exactly as one additive,
fresh-run-only `DocsCheck` composition boundary.

Do not add CLI/UI behavior, schemas, providers, OpenShell, SideEffect
execution, writes, hosted administration, or new governance modes.

The focused planning review accepted this recommendation without blockers.

## 20. Governed Planning Record

- workflow: `dg/d`
- run: `run-1785054113048952000-2`
- approval:
  `approval/run-1785054113048952000-2/planning-approved`
- presentation: `presentation/334a44b57fcc4784`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- approval-presentation event marker: present
- out-of-kernel work: architecture inspection, plan authoring, documentation,
  validation, and later git/PR work
- missing coverage: the kernel coordinated governance only; it did not edit
  files, run checks, create a WorkReport artifact, or perform git actions
