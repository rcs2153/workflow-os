# Explicit-Profile Authoritative Approval-Resume Report Completion Review

## 1. Executive Verdict

**Phase accepted; proceed to the authoritative quiet-success CLI preview.**

The implementation closes the cross-prerequisite authority mismatch identified
by the CLI prerequisite re-review. A run started through a resolved explicit
project-validation profile can now resume approval through that same closed
profile and generate terminal report evidence from the exact decision-time
check.

No blocker was found.

## 2. Scope Verification

The phase stayed within the approved Core-only bridge scope.

It did not add:

- CLI behavior or defaults;
- inferred profile selection;
- public arbitrary handler authority;
- shell strings or repository command discovery;
- automatic approval;
- schemas or runtime configuration;
- persistence or report artifacts;
- providers or OpenShell integration;
- SideEffect execution or writes;
- examples, hosted behavior, or release changes.

## 3. API Assessment

The new public helper accepts `ResolvedExplicitLocalCheckProfile`, not a trait
object or caller-constructed command contract.

The existing DocsCheck helper remains unchanged and delegates to the same
private implementation. The shared `AuthoritativeLocalCheckHandler` trait
remains crate-private, and `ResolvedExplicitLocalCheckProfile::handler()`
remains crate-private.

This is the correct authority boundary for the first CLI consumer.

## 4. Approval And Reassessment Assessment

The shared private path preserves the accepted order:

1. prepare the approval decision without mutation;
2. read and verify the immutable run bundle;
3. compare request, run, and bundle identity;
4. read and compare the durable assessment binding;
5. resolve the canonical stored declaration against the selected handler;
6. run one fresh decision-time check;
7. compose and compare the reassessed binding;
8. validate the approval assessment binding;
9. validate durable approval-presentation proof;
10. apply the approval decision and resume.

Handler contract mismatch returns before approval or skill events are
appended.

## 5. Report And Evidence Assessment

The decision-time check is not rerun for reporting.

The same bounded result that supports reassessment is converted into a
payload-free `LocalCheckResultReference` and cited in the terminal report. A
non-terminal later approval remains deferred, and report-generation failure
remains separate from workflow status through the existing result type.

## 6. Compatibility Assessment

- Existing DocsCheck approval-resume behavior remains unchanged.
- Existing public request and result types remain unchanged.
- Existing error codes and fail-closed validation paths remain in use.
- No workflow schema or serialized runtime shape changed.
- No default executor path changed.

## 7. Privacy And Security Assessment

- The helper accepts no raw command output or provider payload.
- Public callers cannot supply an arbitrary handler through this API.
- Contract substitution fails with a stable, non-leaking error.
- Debug and report behavior continue to use the existing bounded result types.
- No credentials, shell strings, raw paths, or new write authority are stored.

## 8. Test Quality Assessment

Focused tests prove:

- project-validation approval can resume and complete;
- the terminal report is generated;
- the reference identifies the project-validation command kind;
- there is one request-time and one decision-time check;
- durable events equal returned run events;
- a DocsCheck-bound run cannot resume through the project-validation profile;
- the substituted profile is not executed;
- no skill invocation occurs on substitution failure; and
- event history remains unchanged on substitution failure.

The full workspace suite also exercises existing grant, denial, later
approval, report failure, presentation proof, immutable bundle, resolved
context, provider, SideEffect, capability, adapter, and report behavior.

One naming debt remains non-blocking: shared request and reference types retain
`DocsCheck` in their names even when the closed project-validation profile uses
them. Renaming those types now would add churn without changing authority or
behavior.

## 9. Validation

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Consider neutral names for shared authoritative local-check request and
  reference types during a later compatibility-focused cleanup.
- Keep the first CLI profile limited to Workflow OS project validation.
- Use the accepted proportional-governance decision and disclosure axes
  without collapsing visible disclosure into blocking approval.
- Continue toward quiet success for low-risk work, as current external
  evaluation recommends, while preserving inspectable evidence.

## 12. Governed Review Record

- Workflow: `dg/review`
- Run: `run-1785110178531619000-2`
- Approval:
  `approval/run-1785110178531619000-2/review-scope-approved`
- Presentation: `presentation/25806ff663b61054`
- Approval outcome: granted by delegated maintainer
- Validation: full required suite passed
- Out-of-kernel work: source inspection, documentation authoring, validation
  commands, and this review were performed by the maintainer outside kernel
  execution; the kernel governed scope and approval only.

## 13. Recommended Next Phase

Implement the explicit authoritative quiet-success CLI preview.

The CLI phase should consume only the reviewed closed project-validation
profile, remain opt-in, and render bounded human and JSON outcomes for quiet,
visible, approval-required, and denied routes. It must not add arbitrary
commands, automatic approvals, artifacts, providers, OpenShell integration,
SideEffect execution, or writes.
