# Trusted-Host Project-Validation Profile Plan Blocker Fix Report

## 1. Executive Summary

The trusted-host project-validation profile plan now closes both blockers from
focused maintainer/security review. It defines truthful adaptation of
non-passing local checks before trusted-host lifecycle terminalization and an
exact profile-selection compatibility rule. This is a documentation-only fix;
no runtime behavior changes in this phase.

## 2. Blockers Fixed

1. A failed or timed-out `LocalCheckResult` could otherwise be converted into
   `Ok(SkillOutput)` and complete the trusted-host workflow.
2. The plan required explicit profile selection while also promising
   compatibility with the current implicit docs-check invocation.

## 3. Truthful Result Adaptation

The corrected plan requires one private Core-owned trusted-host adapter. It
invokes the exact resolved handler once, maps only `Passed` to bounded skill
output, and maps `Failed`, `TimedOut`, or internal handler failure to stable
non-leaking execution failure before terminalization.

The general `LocalCheckResult` model remains unchanged. Authoritative
proportional-governance APIs may continue to consume non-passing results as
data. The correction applies only at the trusted-host skill-attempt boundary
and covers both docs-check and project-validation.

## 4. Selector Compatibility

The corrected plan requires:

- explicit `--profile workflow-os-project-validation` for the new path;
- accepted explicit `--profile docs-check` for the existing path; and
- a temporary no-profile compatibility alias for docs-check only.

Unknown selectors, mixed argument families, and project-validation arguments
without the explicit selector fail before backend opening or state creation.
No profile authority is inferred from ambient files or argument coincidence.

## 5. Test Boundary

The implementation tests must prove truthful failed and timed-out lifecycle
events for both profiles, single execution, non-leaking errors, non-executing
failed-run replay, explicit selector behavior, the docs-check compatibility
alias, and pre-state rejection of invalid selection.

## 6. Scope Explicitly Not Completed

- No Rust implementation.
- No generic local-command host.
- No new profile family.
- No provider, network, source-write, SideEffect, report-artifact, schema,
  hosted, or released CLI behavior.
- No change to automatic or delegated approval posture.

## 7. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.
- Rust checks are not required for this documentation-only correction.

## 8. Governed Phase Evidence

- Dogfood workflow: `dg/blocker`.
- Run: `run-1792661900036384000-2`.
- Approval: `approval/run-1792661900036384000-2/fix-approved`.
- Approval presentation: `presentation/e6792bed36203d7f`.
- Approval outcome: granted under delegated-maintainer authority with
  persisted presentation proof.
- Phase status: `Completed`.
- Event summary: 39 events, one approval, zero retries, zero escalations, and
  proof-enforced approval presentation.
- Validation summary: documentation and diff checks passed; Rust checks were
  not required because this phase changed documentation only.
- Out-of-kernel work: plan/report/roadmap edits, validation, git, and later PR
  operations are performed by the delegated maintainer while Workflow OS
  governs the bounded fix.

## 9. Recommended Next Phase

Perform focused maintainer/security re-review of the corrected plan. If
accepted, implement one bounded foreground-binary vertical slice.
