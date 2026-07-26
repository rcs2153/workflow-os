# Explicit-Profile Authoritative Approval-Resume Report Completion Report

## 1. Executive Summary

The explicit-profile approval-resume report bridge is implemented.

A run started through the closed Workflow OS project-validation profile can
now resume an aggregate approval through the same resolved profile, perform
the required fresh decision-time reassessment, and generate an in-memory
terminal `WorkReport` citing the exact decision-time local-check result.

The implementation remains local, explicit, and Core-owned. It does not add
CLI behavior, automatic profile selection, arbitrary command authority,
persistence, artifacts, providers, OpenShell integration, SideEffect
execution, or writes.

## 2. Scope Completed

- Added the explicit-profile approval-resume report helper.
- Preserved the existing DocsCheck-specific helper.
- Extracted shared report composition behind a private handler boundary.
- Generalized private approval reassessment over the existing private
  authoritative handler trait.
- Exported only the closed resolved-profile helper.
- Added focused success and handler-substitution tests.

## 3. Scope Explicitly Not Completed

- No CLI integration or default behavior.
- No profile inference or repository command discovery.
- No public arbitrary handler API.
- No automatic approvals.
- No schema or runtime configuration changes.
- No report artifacts or persistence.
- No providers or OpenShell integration.
- No SideEffect execution or writes.
- No examples, hosted behavior, or release changes.

## 4. API Summary

The new function is:

```text
decide_approval_with_authoritative_explicit_local_check_profile_governance_report
```

It accepts the existing executor, immutable bundle store, one
`ResolvedExplicitLocalCheckProfile`, and the existing approval-report request.
It returns the existing approval-report result type.

The private shared helper accepts the crate-owned
`AuthoritativeLocalCheckHandler`. This avoids exposing arbitrary handler
polymorphism to external callers.

## 5. Approval And Reassessment Behavior

The helper preserves:

- immutable run and bundle matching;
- durable governance assessment matching;
- canonical declaration and handler contract matching;
- fresh decision-time check reassessment;
- expected aggregate fingerprint checks;
- approval assessment binding equality;
- approval-presentation proof enforcement; and
- existing resolved-context approval behavior.

Only after those checks pass does the existing approval mutation and resume
path run.

## 6. Evidence And Report Behavior

The decision-time local-check result is reused for the fresh reassessment and
for one payload-free `LocalCheckResultReference`. A terminal report cites that
reference.

The helper does not run an additional check for reporting. Non-terminal and
report-generation failure postures remain represented by the existing result
type without rewriting workflow truth.

## 7. Privacy And Authority Posture

- No raw command output is stored in the report reference.
- Stable errors do not include caller payloads.
- Handler contract substitution fails before approval mutation.
- The polymorphic handler boundary is private to Core.
- Public profile selection remains a closed vocabulary.
- No shell strings, credentials, provider payloads, or new write authority are
  accepted.

## 8. Test Coverage

Focused coverage verifies:

- project-validation approval resumes to completion;
- the terminal report is generated;
- the result reference records
  `workflow_os_project_validation`;
- the check runs once at request time and once at decision time;
- durable events match returned run events;
- a DocsCheck-bound run cannot resume through the project-validation profile;
- the substituted profile does not execute;
- approval and skill events are not appended on substitution failure; and
- existing DocsCheck approval-report tests remain in the workspace suite.

## 9. Commands Run And Results

- Focused explicit-profile local-executor tests: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 10. Remaining Limitations

- Only Workflow OS project validation is a closed resolved profile.
- The ordinary CLI does not consume authoritative governance routes.
- Approval resume requires explicit caller-supplied profile and report inputs.
- Reports remain in memory.
- No runtime default or schema declares the profile.
- No provider or sandbox execution substrate is integrated.

## 11. Recommended Next Phase

Focused maintainer review accepted this bridge without blockers.

Recommended next phase: **explicit authoritative quiet-success CLI preview**.
That preview should stay opt-in and should not add further model or planning
lanes before proving the operator path.
