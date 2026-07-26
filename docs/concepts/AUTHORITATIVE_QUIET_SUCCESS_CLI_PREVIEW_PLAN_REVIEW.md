# Authoritative Quiet-Success CLI Preview Plan Review

## 1. Executive Verdict

Plan accepted; proceed to authoritative approval-resume report completion
planning.

The future CLI contract is useful and appropriately additive, but immediate
implementation would be misleading. The accepted authoritative path currently
depends on a Workflow OS repository-specific `DocsCheckLocalHandler`, and the
approval-required route cannot yet complete its deferred report after resume.

The plan correctly treats those as runtime prerequisites rather than hiding
them behind CLI flags or ambient discovery.

## 2. Scope Verification

The planning phase stayed within its approved documentation-only scope.

It did not implement:

- CLI behavior;
- default executor behavior;
- automatic approval;
- handler registration or command discovery;
- runtime configuration;
- schemas, scaffolds, or examples;
- report artifacts or persistence;
- providers or OpenShell;
- SideEffect execution or writes;
- hosted behavior or release changes.

## 3. Product Boundary Assessment

Extending the existing `run` command with an explicit future flag is preferable
to introducing a second orchestration command. The plan keeps ordinary
`workflow-os run` behavior unchanged and reserves route selection for the
authoritative dispatcher.

The proposed operator contract reflects the product thesis:

```text
Agent or handler executes.
Workflow OS selects and records the required governance posture.
```

It does not ask the CLI to become a sandbox, policy author, or arbitrary command
runner.

## 4. Current Runtime Constraint Assessment

The plan accurately describes the current CLI:

- it builds the existing local executor path;
- it registers only explicit mock handlers when requested;
- it does not register local checks by default; and
- it has no generic source of executable check authority.

The accepted `DocsCheckLocalHandler` is intentionally bound to the Workflow OS
repository and `npm run check:docs`. It cannot truthfully stand in for a
general existing-repository check profile.

This is a blocking prerequisite for general CLI exposure, not a blocker in the
plan itself.

## 5. Approval-Resume Prerequisite Assessment

The accepted report consumer correctly returns `DeferredNonTerminal` for an
approval-required route. A CLI preview must not leave that route without a
reviewed terminal report-completion path.

The plan requires the next runtime phase to:

- retain the original authoritative assessment and local-check reference;
- reuse immutable and resolved-context validation;
- require approval-presentation proof;
- perform the accepted decision-time canonical check reassessment exactly once;
- preserve the fresh reassessment result for terminal report citation instead
  of treating the request-time result as current authorization evidence;
- avoid a second report-only check execution;
- resume through the existing accepted executor path; and
- generate the WorkReport only after terminal completion.

That is the right next implementation boundary.

This wording is a fix-forward clarification discovered during the next planning
phase. The accepted approval path already reruns the canonical check before
decision mutation to prevent stale authorization; the earlier shorthand about
avoiding a rerun was therefore too broad.

## 6. Check-Profile Prerequisite Assessment

The plan correctly rejects:

- raw executable and argument flags;
- arbitrary shell strings;
- PATH-based guessing as authority;
- repository script discovery as execution permission; and
- conversion of model-only check vocabulary into executable behavior.

The future profile source must bind an implemented handler to a validated
canonical contract. Whether the first generic source is project validation, an
embedding-caller profile, or later schema vocabulary remains open and should be
planned separately after approval-resume report completion.

## 7. Route And Output Assessment

The proposed output semantics preserve the accepted decision axes:

- quiet proceed is concise and non-interrupting;
- visible proceed is disclosed but not presented as approval;
- approval required renders the complete handoff;
- denial remains terminal and cannot be downgraded; and
- report failure remains separate from workflow truth.

The default output is appropriately bounded. JSON remains experimental and
payload-free.

## 8. Privacy And Security Assessment

The plan keeps raw output, command transcripts, paths, environment values,
policy payloads, approval presentation text, and provider data out of default
rendering.

It requires validated Core constructors and forbids the CLI from accepting
caller-selected routes, check statuses, or approval outcomes. It also avoids
claiming sandbox behavior before a sandbox substrate is accepted.

## 9. Compatibility Assessment

The plan preserves:

- existing `workflow-os run` behavior;
- existing local handler defaults;
- existing executor APIs;
- current schemas;
- report artifact and persistence posture;
- provider and SideEffect boundaries; and
- local-first preview positioning.

The future flag remains experimental and additive.

## 10. Test Plan Assessment

The proposed tests cover:

- unchanged ordinary run behavior;
- fail-closed missing profile behavior;
- all four route outcomes;
- single check execution;
- same-call report citation;
- approval-resume completion;
- report failure separation;
- concise and JSON rendering;
- privacy;
- no artifact or provider behavior; and
- full regression coverage.

No blocking test gap was found in the plan.

## 11. Relationship To External Evaluation

The plan responds correctly to the evaluator's main recommendation: reduce
ceremony for low-risk work without weakening evidence.

It also protects the evaluator's strongest trust signal by refusing to present
repository-specific execution as a generic feature.

The previously reported Node 24 integration-check failure and duplicated
missing-manifest diagnostic are fixed on current `main`. They require
regression awareness, not a roadmap diversion.

## 12. Relationship To OpenShell

The plan correctly keeps OpenShell out of scope.

An optional provider-neutral sandbox substrate may later satisfy part of the
explicit check-profile requirement, but it must not become policy authority or
justify CLI exposure before its own request, result, evidence, degradation, and
security-maintenance boundaries are reviewed.

Forking OpenShell is not justified by this plan.

## 13. Planning Blockers

None.

The two prerequisites block CLI implementation, not acceptance of the plan:

1. authoritative approval-resume report completion; and
2. one generic explicit local-check profile source.

## 14. Non-Blocking Follow-Ups

- Decide whether project validation should become a first-class local-check
  result rather than a child process.
- Decide the built-in preview report contract identity.
- Keep default human output smaller than verbose or JSON output.
- Add a direct consumer `BeforeReport` regression when the approval-resume path
  touches shared report construction.

## 15. Recommended Next Phase

Authoritative approval-resume report completion planning.

Why:

- it closes the only route that the accepted report consumer must defer;
- it composes existing immutable-input, resolved-context, approval-presentation,
  local-check-reference, and WorkReport primitives;
- it is runtime work rather than a new primitive family;
- it is required before complete operator UX; and
- it does not require CLI, schemas, providers, OpenShell, artifacts, or writes.

## 16. Governed Review Record

- workflow: `dg/review`
- run: `run-1785093622107068000-2`
- approval:
  `approval/run-1785093622107068000-2/review-scope-approved`
- presentation: `presentation/4195bc6f1489fee9`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval presentation enforcement: proof enforced
- validation: `npm run check:docs` and `git diff --check` passed
- out-of-kernel work: plan inspection, architecture assessment, review
  authoring, and documentation validation
- missing coverage: the kernel coordinates governance only; it did not inspect
  code, edit files, execute validation, or perform git and PR actions
