# Authoritative Proportional-Governance Route Dispatcher Review

## 1. Executive Verdict

Phase accepted; proceed to planning the first explicit consumer of the
authoritative dispatcher.

The implementation closes the caller-selected route gap for the bounded local
`DocsCheck` slice without changing default executor behavior or broadening
execution authority.

## 2. Scope Verification

The phase stayed within approved runtime-composition scope.

It added:

- one explicit dispatcher;
- one visible-route dependency bundle;
- one route-truth-preserving result enum;
- private post-preparation consumers; and
- focused tests and documentation.

It did not add CLI/UI/schema/example exposure, provider or OpenShell
integration, SideEffect execution, writes, retry/resume, hosted behavior,
reasoning lineage, or release changes.

## 3. Authority Boundary Assessment

The dispatcher invokes authoritative preparation exactly once. It does not
call another public route API after preparation and therefore does not rerun
the canonical check or reclaim the immutable bundle.

The existing public route APIs now reuse the same private consumers. Their
accepted route checks remain inside those consumers as defense in depth.

## 4. Route Correctness Assessment

The dispatcher accepts only:

- `Proceed + Quiet`;
- `Proceed + Visible`;
- `RequireApproval + Visible`; and
- `Denied + Visible`.

The assessment must be complete and source-bound. Invalid or incomplete
combinations fail closed. The caller supplies no route enum, so visible,
approval-required, and denied posture cannot be downgraded by API choice.

## 5. Dependency Assessment

Visible proceed requires one structurally complete dependency bundle
containing bounded delivery inputs and the injected disclosure handler.
Partial dependencies are unrepresentable.

Other routes reject supplied visible dependencies. Quiet, approval-required,
and denied paths do not invoke the disclosure handler.

Because route selection follows authoritative preparation, unused or missing
dependency rejection may leave bounded create-only immutable bundle residue.
No run event, skill, approval, provider call, or SideEffect authority is
created by that failure. This limitation is documented and non-blocking for
the fresh-run-only slice.

## 6. Result Assessment

The four enum variants contain the existing route-specific result types.
Common accessors expose only run, assessment, and bounded local-check results;
they do not convert approval-required or denied variants into success.

Custom `Debug` exposes route posture, run status, and result count only.

## 7. Event And Workflow Semantics Assessment

The dispatcher reuses accepted route consumers:

- quiet proceed executes normally;
- visible proceed delivers before ordinary run events and skills;
- approval-required pauses before step scheduling; and
- denied fails with `PolicyDenied` before step scheduling.

No new event kind or default executor path was introduced.

## 8. Privacy And Error Assessment

The dependency and result `Debug` implementations remain bounded.

Route errors are stable and do not include caller identifiers, paths, check
output, disclosure prose, assessment fingerprints, provider data, or secret
material.

## 9. Compatibility Assessment

The four existing route APIs remain public and behaviorally unchanged.
Ordinary executor methods, step approvals, hooks, reports, artifacts,
providers, SideEffects, persistence, CLI behavior, schemas, and examples are
unchanged.

## 10. Test Assessment

Focused tests prove:

- all four route selections;
- exactly one canonical check per route;
- one visible delivery before skill execution;
- no step scheduling for approval or denial;
- `PolicyDenied` terminal denial;
- missing and unused visible-dependency failure; and
- non-leaking dispatcher `Debug` and errors.

The existing detailed route-local tests and full workspace suite passed.

## 11. Documentation Assessment

The plan, implementation report, and roadmap accurately distinguish:

- implemented explicit dispatcher behavior;
- unchanged default executor behavior;
- in-memory visible delivery;
- fresh-run-only operation; and
- deferred CLI/UI, provider, OpenShell, SideEffect, write, hosted, and
  recovery work.

## 12. Blockers

None.

## 13. Non-Blocking Follow-Ups

- Decide which explicit local runtime path should first consume the dispatcher
  without making it a universal default.
- Preserve route-specific result matching in future convenience APIs.
- Add durable visible-delivery receipts before retry, resume, or hosted use.
- Revisit pre-claim dependency validation if future route inputs permit safe
  structural checks before authoritative assessment.

## 14. Recommended Next Phase

Plan one narrow explicit consumer of the dispatcher before any broad default
or operator-facing integration.

That plan should decide:

- the exact opt-in caller;
- how it supplies visible-delivery dependencies;
- how route-specific results are exposed without semantic flattening; and
- which event, report, and recovery limitations remain visible.

OpenShell remains a later optional execution substrate behind Workflow OS
authorization. It is not the next implementation phase.

## 15. Governed Review Record

- workflow: `dg/review`
- run: `run-1785057124777553000-2`
- approval:
  `approval/run-1785057124777553000-2/review-scope-approved`
- presentation: `presentation/39c7b347eac2c259`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- validation: focused tests, formatting, full workspace clippy, full workspace
  tests, docs checks, and diff checks passed
- out-of-kernel work: code inspection, review authoring, validation, and later
  git/PR work
- missing coverage: the kernel coordinated governance only; it did not edit
  files, execute checks, create a WorkReport artifact, or perform git actions

