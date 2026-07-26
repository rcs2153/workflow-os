# Authoritative Proportional-Governance Route Dispatcher Plan Review

## 1. Executive Verdict

Plan accepted; proceed to authoritative route dispatcher implementation.

The plan defines the missing composition boundary without weakening any of the
four accepted route-specific executor slices. The authoritative assessment,
not caller preference, selects quiet proceed, visible proceed,
approval-required, or denied behavior.

## 2. Scope Verification

The plan remains within an additive local `DocsCheck` dispatcher boundary.

It does not authorize:

- default or automatic executor integration;
- CLI, UI, schema, or example exposure;
- providers, OpenShell, sandbox lifecycle, or credentials;
- SideEffect execution or writes;
- retry, resume, or existing-run support;
- hosted behavior or enterprise administration;
- reasoning lineage; or
- release posture changes.

## 3. Dispatcher Boundary Assessment

The proposed boundary is appropriately narrow and removes the remaining
caller-selected route gap. Callers provide facts and explicit dependencies;
they do not provide a route enum.

The exact normalized assessment pairs are complete:

- `Proceed + Quiet`;
- `Proceed + Visible`;
- `RequireApproval + Visible`; and
- `Denied + Visible`.

Incomplete or invalid pairs fail closed. This is monotonic: visible,
approval-required, and denied outcomes cannot become quiet proceed.

## 4. One-Pass Preparation Assessment

Preparing the immutable run bundle, executing the canonical `DocsCheck`, and
deriving the complete source-bound assessment exactly once is the correct
authority boundary.

The planned private post-preparation consumers avoid rerunning checks or
reclaiming bundles. Keeping the existing route enforcement helpers inside
those consumers provides useful defense in depth without creating a second
decision model.

## 5. Dependency Assessment

Only visible proceed requires the injected non-blocking disclosure surface.
The plan correctly requires complete visible dependencies and rejects unused
visible dependencies on other routes instead of silently accepting ambient
capability.

Structural dependency validation should happen before canonical check
execution when possible. Route-specific unused-dependency rejection can occur
only after the authoritative assessment is known; bounded create-only residue
at that point is an accepted first-slice limitation and must remain disclosed.

## 6. Result And Event Assessment

Distinct typed result variants preserve route truth. Common accessors must not
erase approval-required or denied semantics.

The planned event ordering matches the accepted route-local behavior and does
not add event vocabulary:

- quiet execution remains quiet;
- visible delivery occurs before ordinary execution events;
- approval pauses before step scheduling; and
- denial terminates before step scheduling.

## 7. Privacy And Failure Assessment

The plan maintains the existing non-leaking boundary for source contents,
check output, paths, commands, assessment reasons, presentation prose,
provider data, and credentials.

Stable route-dependency errors are appropriate. Invalid route state returns an
error without fabricating an execution result.

## 8. Compatibility Assessment

The dispatcher is additive. Existing route APIs, ordinary executor behavior,
step approvals, hooks, reports, providers, SideEffects, persistence, CLI
behavior, schemas, and examples remain unchanged.

Route-local tests remain mandatory after dispatcher tests are introduced.

## 9. Test Plan Assessment

The planned tests cover:

- all four route selections;
- exactly one canonical check execution;
- visible dependency completeness and unused-dependency rejection;
- monotonic route behavior;
- route-local event ordering;
- absence of unintended execution activity;
- typed-result and error non-leakage; and
- full existing workspace regression coverage.

The implementation should also assert that malformed optional dependency
bundles fail before handler invocation and that common result accessors cannot
misrepresent approval-required or denied variants.

## 10. Product Feedback Alignment

Current external evaluation describes Workflow OS as a coherent and honest
local-first governance kernel and identifies ceremony reduction for low-risk
work as the next product problem.

This dispatcher is the correct near-term response. It composes already
accepted proportional-governance routes so later operator UX can project the
derived route rather than asking callers to choose it. It does not broaden
into a general execution platform.

The separate Node-version integration-check sharpness and duplicate
missing-manifest diagnostic are legitimate product follow-ups, but they are
not blockers for this runtime composition phase.

## 11. Planning Blockers

None.

## 12. Non-Blocking Follow-Ups

- Validate visible dependency structure as early as possible without
  evaluating governance twice.
- Keep convenience accessors route-aware.
- Define durable disclosure receipt recovery before retry, resume, or hosted
  operation.
- Address Node-version integration-check diagnostics and the duplicated
  missing-manifest message in separate focused UX/tooling work.

## 13. Recommended Next Phase

Implement the authoritative route dispatcher as one additive, fresh-run-only
local `DocsCheck` composition phase, followed by a focused maintainer review.

Do not add CLI or UI behavior, schemas, providers, OpenShell, SideEffect
execution, writes, hosted administration, or new governance modes.

## 14. Governed Review Record

- workflow: `dg/review`
- run: `run-1785054381662359000-2`
- approval:
  `approval/run-1785054381662359000-2/review-scope-approved`
- presentation: `presentation/3f362fe30042950d`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- approval-presentation proof: persisted and freshness checked
- out-of-kernel work: plan inspection, review authoring, documentation
  validation, and later git/PR work
- missing coverage: the kernel coordinated governance only; it did not edit
  files, run checks, create a WorkReport artifact, or perform git actions

