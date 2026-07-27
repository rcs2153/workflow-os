# Explicit-Profile Authoritative Approval-Resume Report Completion Plan

Status: Implemented and accepted.

Related foundations:

- [Authoritative Quiet-Success CLI Preview Plan](authoritative-quiet-success-cli-preview-plan.md)
- [Authoritative Quiet-Success CLI Preview Prerequisite Re-Review](../concepts/AUTHORITATIVE_QUIET_SUCCESS_CLI_PREVIEW_PREREQUISITE_REREVIEW.md)
- [Authoritative Approval-Resume Report Completion Plan](authoritative-approval-resume-report-completion-plan.md)
- [Generic Explicit Local-Check Profile Source Plan](generic-explicit-local-check-profile-source-plan.md)
- [Phase Review](../concepts/EXPLICIT_PROFILE_AUTHORITATIVE_APPROVAL_RESUME_REPORT_COMPLETION_REVIEW.md)

## 1. Executive Summary

Fresh authoritative execution can use a
`ResolvedExplicitLocalCheckProfile`, including the closed Workflow OS project
validation profile. Proof-enforced approval-resume report completion was still
restricted to `DocsCheckLocalHandler`.

This phase adds one closed bridge so an approval-required run started with a
resolved explicit profile can be reassessed and reported through that same
profile after approval. The bridge reuses the existing immutable run binding,
presentation proof, fresh decision-time check, approval mutation, and
in-memory report behavior.

It does not implement CLI behavior, automatic profile selection, arbitrary
handlers, persistence, artifacts, providers, OpenShell integration,
SideEffect execution, or writes.

## 2. Goals

- Preserve one handler authority across request-time and decision-time checks.
- Reuse the accepted proof-enforced approval and report path.
- Run the canonical decision-time check exactly once.
- Cite that exact result in the terminal report.
- Preserve approval denial, later approval, and report-failure truth.
- Fail closed before approval mutation when the profile contract differs from
  the immutable declaration.
- Keep the polymorphic handler boundary private to `workflow-core`.

## 3. Non-Goals

This phase does not authorize:

- an authoritative CLI flag;
- implicit or inferred profile selection;
- a public arbitrary local-check handler API;
- raw executable paths, arguments, or shell strings;
- automatic approval or model self-approval;
- workflow schema changes;
- report artifacts or persistence;
- provider execution or OpenShell integration;
- SideEffect execution or writes;
- examples, hosted behavior, or release changes.

## 4. API

Add:

```text
decide_approval_with_authoritative_explicit_local_check_profile_governance_report
```

The helper accepts:

- `LocalExecutor`;
- `LocalImmutableRunBundleStore`;
- one previously resolved `ResolvedExplicitLocalCheckProfile`; and
- the existing
  `LocalAuthoritativeGovernanceApprovalReportDecisionRequest`.

It returns the existing
`LocalAuthoritativeGovernanceApprovalReportDecisionResult`.

The existing DocsCheck-specific public helper remains unchanged.

## 5. Private Composition

The two public helpers delegate to one private function that accepts the
crate-owned `AuthoritativeLocalCheckHandler` trait. Approval reassessment also
uses that private trait.

This keeps polymorphism inside Core. Public callers can select only a profile
represented by the closed `ExplicitLocalCheckProfileSelection` vocabulary.

## 6. Validation Boundary

Before approval mutation, the helper must preserve:

- immutable run identity and bundle binding checks;
- durable assessment binding equality;
- selected declaration and handler contract equality;
- expected aggregate fingerprint checks;
- fresh decision-time reassessment;
- approval assessment binding equality;
- durable presentation proof validation; and
- resolved execution context checks in the existing approval path.

Contract substitution must fail before a decision or skill event is appended.

## 7. Report Boundary

The accepted decision-time local-check result is the only result used to:

- authorize the fresh reassessment;
- construct the payload-free `LocalCheckResultReference`; and
- cite validation in the generated terminal `WorkReport`.

No extra check runs only for reporting. Report construction failure remains
separate from workflow status.

## 8. Test Plan

Focused tests cover:

1. project-validation profile approval grant;
2. generated terminal report;
3. project-validation command kind in the result reference;
4. exactly one request-time and one decision-time check;
5. durable run events equal the returned run;
6. handler substitution rejected before approval mutation;
7. no substituted handler execution;
8. no skill execution on substitution failure; and
9. existing DocsCheck approval-report behavior unchanged.

Workspace validation must include:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`; and
- `git diff --check`.

## 9. Final Recommendation

Focused maintainer review accepted this phase. Proceed directly to the
explicit authoritative quiet-success CLI preview. The CLI must remain opt-in
and use only the closed Workflow OS project-validation profile.
