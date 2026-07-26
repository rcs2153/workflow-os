# Authoritative Proportional-Governance Route Dispatcher Report

## 1. Executive Summary

Workflow OS now has one additive local dispatcher where the complete
source-bound proportional-governance assessment selects the accepted
authoritative `DocsCheck` executor route.

The dispatcher prepares authority once and returns a route-specific typed
result. Callers supply facts and explicit visible-delivery dependencies; they
do not select quiet, visible, approval-required, or denied posture.

## 2. Scope Completed

- Added `route_authoritative_docs_check_governance(...)`.
- Added one explicit visible-route dependency bundle.
- Added one typed four-variant route result.
- Refactored the four existing public route APIs to use private
  post-preparation consumers.
- Preserved exact route-specific enforcement as defense in depth.
- Added focused all-route, one-check, dependency, event, and privacy tests.

## 3. Scope Explicitly Not Completed

This phase did not add:

- default or automatic executor routing;
- CLI, UI, schema, or example exposure;
- providers, OpenShell, sandbox lifecycle, or credentials;
- SideEffect execution or writes;
- retry, resume, or existing-run routing;
- asynchronous or durable disclosure delivery;
- report or artifact generation;
- hosted behavior, enterprise administration, or reasoning lineage; or
- release posture changes.

## 4. API Summary

The additive dispatcher accepts:

- the existing authoritative `DocsCheck` request;
- immutable run bundle store;
- explicit `DocsCheckLocalHandler`; and
- optional `LocalExecutionAuthoritativeVisibleGovernanceDependencies`.

The dependency bundle contains bounded disclosure inputs plus a borrowed
`GovernanceDisclosureDeliveryHandler`. It contains no route enum.

The result is
`LocalExecutionWithAuthoritativeGovernanceRouteResult`:

- `QuietProceed`;
- `VisibleProceed`;
- `ApprovalRequired`; or
- `Denied`.

Each variant contains the corresponding existing accepted route result.

## 5. One-Pass Authority Boundary

The dispatcher invokes authoritative preparation exactly once:

1. require a fresh run;
2. resolve and validate the execution plan;
3. build and create-only claim the immutable run bundle;
4. execute the canonical `DocsCheck`;
5. derive the complete source-bound assessment; and
6. select one private route consumer.

The dispatcher does not call another public route API after preparation.
Therefore it does not rerun the check, rebuild the bundle, or reconstruct
governance authority.

## 6. Route Semantics

The exact accepted mapping is:

```text
Proceed + Quiet            -> QuietProceed
Proceed + Visible          -> VisibleProceed
RequireApproval + Visible  -> ApprovalRequired
Denied + Visible           -> Denied
```

Incomplete, source-unbound, or invalid pairs fail closed.

Visible proceed requires explicit delivery dependencies. Supplying those
dependencies to quiet, approval-required, or denied routes is rejected rather
than ignored.

## 7. Event And Workflow Semantics

Existing route behavior is unchanged:

- quiet proceed executes without disclosure delivery;
- visible proceed delivers exactly once before ordinary run events and skill
  execution;
- approval-required pauses before workflow step scheduling; and
- denied terminates with `PolicyDenied` before step scheduling.

The dispatcher adds no workflow event kind and does not change ordinary
executor defaults.

## 8. Privacy And Error Posture

Dispatcher `Debug` exposes only route posture, run status, and local-check
result count.

Stable route errors do not echo:

- run, workflow, delivery, or assessment identifiers;
- source or spec contents;
- local-check output;
- paths, commands, arguments, or environment values;
- disclosure or approval prose; or
- credentials, authorization headers, private keys, or tokens.

## 9. Test Coverage

Focused tests prove:

- all four authoritative routes are selected correctly;
- exactly one canonical check executes per route;
- visible delivery occurs once before skills;
- approval and denial schedule no workflow steps;
- denial retains `PolicyDenied`;
- missing visible dependencies fail closed;
- unused visible dependencies fail closed without handler invocation; and
- dispatcher `Debug` does not expose run identity.

Existing route-local tests remain unchanged and continue to protect detailed
event, approval, denial, disclosure, and source-binding behavior.

## 10. Validation

- focused dispatcher tests: passed, 4 tests
- `cargo fmt --all --check`: passed
- `cargo clippy --workspace --all-targets -- -D warnings`: passed
- `cargo test --workspace`: passed
- `npm run check:docs`: passed
- `git diff --check`: passed

## 11. Known Limitations

- Route-specific dependency rejection occurs after the authoritative route is
  known and may leave bounded create-only immutable bundle residue.
- Visible delivery receipt remains in memory.
- Dispatcher operation is fresh-run-only.
- No durable dispatcher decision receipt exists beyond the accepted
  assessment binding and existing route events.
- No operator-facing default invokes the dispatcher yet.

## 12. Product Feedback Alignment

External evaluation identifies reduced ceremony for low-risk work as the next
product problem. The dispatcher closes the prerequisite composition gap:
later UX can project the route derived by Core rather than asking a caller to
select governance posture.

The dispatcher does not turn Workflow OS into an execution platform. Existing
handler and provider boundaries remain explicit.

## 13. Governed Implementation Record

- workflow: `dg/runtime-composition`
- run: `run-1785054463353555000-2`
- approval:
  `approval/run-1785054463353555000-2/composition-approved`
- presentation: `presentation/64141ca98a0c59ef`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- out-of-kernel work: Rust implementation, tests, documentation, validation,
  and later git/PR work
- missing coverage: the kernel coordinated governance only; it did not edit
  files, execute checks, create a WorkReport artifact, or perform git actions

## 14. Recommended Next Phase

Perform a focused maintainer review of the dispatcher implementation before
default executor integration, operator UX, or execution-provider work.
