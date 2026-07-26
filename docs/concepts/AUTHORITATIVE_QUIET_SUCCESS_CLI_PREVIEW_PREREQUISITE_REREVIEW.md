# Authoritative Quiet-Success CLI Preview Prerequisite Re-Review

## 1. Executive Verdict

**Needs one prerequisite fix before CLI implementation.**

The original approval-resume report-completion and generic explicit
local-check profile prerequisites are each implemented and reviewed. They do
not yet compose through the same closed handler boundary.

Fresh-run authoritative report generation accepts
`ResolvedExplicitLocalCheckProfile`. Proof-enforced approval-resume report
completion still accepts only `DocsCheckLocalHandler`. Exposing the CLI now
would either leave the generic profile's approval route incomplete or require
the CLI to resume through a different handler contract.

The next phase should add one narrow explicit-profile approval-resume
report-completion bridge. Do not add the CLI flag until that bridge is
implemented and reviewed.

## 2. Review Scope

This focused re-review inspected:

- both accepted prerequisite reports and reviews;
- the current quiet-success CLI plan;
- the ordinary CLI `run` and `approve` paths;
- explicit profile resolution and canonical-contract enforcement;
- fresh-run authoritative route and report helpers;
- proof-enforced approval reassessment and report completion;
- route and report result types;
- report input requirements; and
- current roadmap claims.

The review did not implement CLI behavior, runtime defaults, schemas,
providers, OpenShell, SideEffect execution, writes, artifacts, persistence,
or hosted behavior.

## 3. Accepted Prerequisites

The generic explicit local-check profile is accepted:

- it selects only `workflow_os_project_validation`;
- it constructs the complete canonical `workflow-os validate` contract;
- it requires an explicit executable and project root;
- it exposes the exact immutable declaration inventory;
- it cannot infer repository commands or accept shell strings; and
- public handler construction rejects any non-canonical contract.

The approval-resume report-completion path is also accepted:

- it performs one fresh decision-time canonical check;
- it verifies immutable run input and resolved context;
- it requires approval-presentation proof;
- it cites the exact decision-time result;
- it does not execute a second report-only check; and
- report failure does not rewrite workflow truth.

Ordinary CLI `run` and `approve` behavior remains unchanged.

## 4. Composition Blocker

The fresh-run generic path is:

```text
execute_with_authoritative_explicit_local_check_profile_governance_report(
    ...,
    profile: &ResolvedExplicitLocalCheckProfile,
    ...
)
```

The accepted approval-resume report path is:

```text
decide_approval_with_authoritative_docs_check_governance_report(
    ...,
    docs_check_handler: &DocsCheckLocalHandler,
    ...
)
```

Its private reassessment helper is also typed directly to
`DocsCheckLocalHandler`.

This means the generic profile can request an aggregate approval, but no
public closed profile path can perform the required fresh decision-time
reassessment and complete the report after that approval.

The generic profile implementation report says the profile can enter approval
and report paths. That is true for fresh route selection and deferred report
posture, but not for proof-enforced approval-resume report completion. The
roadmap's statement that CLI implementation is next is therefore premature.

## 5. Why This Is Blocking

The CLI contract requires all authoritative routes to remain truthful:

- quiet work completes with bounded report posture;
- visible work discloses without blocking;
- approval-required work presents a complete handoff and can later complete
  through the same authority boundary;
- denied work terminates without execution; and
- report errors remain separate from run truth.

Starting with `workflow_os_project_validation` and resuming with
`DocsCheckLocalHandler` would violate canonical handler identity, immutable
declaration binding, and decision-time authority. Omitting terminal report
completion would violate the accepted CLI contract.

The correct response is to close the Core composition gap, not to weaken the
CLI or special-case approval output.

## 6. Required Fix Boundary

Add one public closed helper analogous to:

```text
decide_approval_with_authoritative_explicit_local_check_profile_governance_report(...)
```

The helper should:

- accept `ResolvedExplicitLocalCheckProfile`;
- reuse the existing proof-enforced approval decision request and report
  inputs;
- run the profile's canonical handler exactly once at decision time;
- verify the durable assessment against that fresh result;
- preserve presentation-proof and immutable/resolved-context ordering;
- cite the exact decision-time result in a terminal report;
- preserve deferred posture for later step approvals;
- keep post-decision report errors separate from run status; and
- retain stable non-leaking errors and bounded `Debug`.

The implementation should private-generalize the existing Core reassessment
and report-completion logic over the already private
`AuthoritativeLocalCheckHandler` boundary. It must not expose arbitrary
handlers, commands, arguments, or public trait implementations.

## 7. Explicit Non-Goals

Do not add:

- CLI or UI behavior;
- default profile registration;
- profile or command inference;
- arbitrary shell execution;
- automatic approval;
- workflow schema or runtime configuration;
- report artifacts or report persistence;
- providers, OpenShell, containers, or credentials;
- SideEffect execution or writes;
- examples, scaffolds, hosted behavior, or release changes.

## 8. Resolved CLI Decisions

The prerequisite review confirms:

- the first profile is `workflow_os_project_validation`;
- the future surface remains
  `workflow-os run <workflow-id> --authoritative-governance`;
- ordinary `run` remains unchanged;
- the CLI supplies its own executable identity explicitly rather than using
  PATH discovery;
- project root is the validated invocation project directory;
- route selection remains inside Core;
- approval resume should remain on the existing `approve` command when the
  operator integration is implemented; and
- in-memory reports are rendered only as bounded posture and identity, not
  full report contents.

The implementation phase must still select and document one built-in preview
report contract identity.

## 9. Test Requirements For The Fix

The prerequisite fix should prove:

1. an explicit-profile approval grant completes with a generated report;
2. an explicit-profile denial completes with a generated report and no skill
   invocation;
3. a later step approval remains deferred;
4. the decision-time project-validation check runs exactly once;
5. the terminal report cites that exact result;
6. a non-canonical or substituted handler cannot enter the path;
7. presentation-proof, immutable-input, resolved-context, and durable
   assessment mismatches fail before decision mutation;
8. report failure preserves run and event truth;
9. errors and `Debug` do not expose paths, output, environment values, or
   secret-like report inputs; and
10. existing DocsCheck approval-report behavior remains unchanged.

## 10. Documentation Assessment

The plan and roadmap required a fix-forward correction. They now state that
the two original prerequisites are individually accepted but need one closed
composition bridge before CLI exposure.

No current runtime capability was removed. The correction narrows an
overclaim and preserves the project's honest preview posture.

## 11. Blockers

One:

- implement and review explicit-profile proof-enforced approval-resume report
  completion.

## 12. Non-Blocking Follow-Ups

- Decide whether project validation should later become a first-class
  in-process check rather than a child CLI invocation.
- Keep the future human output smaller than verbose or JSON output.
- Preserve request-time and decision-time evidence roles if both later appear
  in one report.
- Diagnose broader Node-version integration behavior separately.

## 13. Recommended Next Phase

Implement the explicit-profile authoritative approval-resume report-completion
bridge.

After focused review accepts that bridge, proceed directly to the additive
authoritative quiet-success CLI preview. Do not add another general primitive
family first.

## 14. Governed Review Record

- workflow: `dg/review`
- run: `run-1785107149512985000-2`
- approval:
  `approval/run-1785107149512985000-2/review-scope-approved`
- presentation: `presentation/51cee20e73ab265b`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- approval-presentation enforcement: proof enforced
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- validation:
  - `npm run check:docs` passed
  - `git diff --check` passed
- out-of-kernel work: prerequisite source inspection, architecture review,
  documentation correction, and validation
- missing coverage: the kernel coordinates governance only; it did not inspect
  source, edit files, run validation, or perform git and PR actions
