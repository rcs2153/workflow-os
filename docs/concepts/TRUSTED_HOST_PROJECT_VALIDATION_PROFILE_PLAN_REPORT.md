# Trusted-Host Project-Validation Profile Plan Report

## 1. Executive Summary

Planning selects `WorkflowOsProjectValidation` as the second closed no-write
profile for the unpublished trusted-host local application. The plan reuses the
existing canonical profile and accepted trusted-host lifecycle while requiring
profile-specific current-authority proof, exact governed-root binding, actual
foreground-binary tests, and explicit disclosure that no WorkReport artifact
is produced.

## 2. Scope Completed

- Selected the existing closed project-validation profile.
- Defined its fixed command, skill, and execution identity.
- Defined host selection and profile-specific argument posture.
- Defined exact governed-project root binding.
- Defined explicit-zero required-context and current-authority requirements.
- Defined policy, approval, evidence/check, lifecycle, replay, and privacy
  behavior.
- Defined terminal report posture and implementation tests.
- Defined one bounded implementation sequence and acceptance criteria.

## 3. Scope Explicitly Not Completed

- No Rust implementation.
- No released CLI or default activation.
- No arbitrary command or handler surface.
- No automatic approval or scheduler.
- No provider, network, credential, SideEffect, or write behavior.
- No WorkReport or report artifact.
- No OpenShell, nested harness, hosted, PostgreSQL, schema, SDK, example, or
  release change.

## 4. Selection Rationale

Project validation applies across Workflow OS projects, already has a reviewed
canonical contract and handler, and introduces no language toolchain, provider,
network, or mutation authority. It is the smallest meaningful generalization
of the accepted docs-check trusted-host proof.

## 5. Core Boundary

Core continues to own immutable admission, exhaustive profile matching,
required-context and current-authority proof, process identity, operational
preparation, workflow lifecycle events, terminalization, and replay. The host
selects a closed profile and supplies an explicit executable path; it cannot
assemble authority, commands, arguments, events, or terminal state.

## 6. Report Posture

The selected implementation will preserve terminal run truth and bounded
check output references. It will not generate or persist a `WorkReport` or
report artifact. That missing product surface is disclosed and remains a
separate future composition phase before released or broader trusted-host use.

## 7. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.
- Rust validation was not required because this phase changes documentation
  only.

## 8. Governed Planning Evidence

- Dogfood workflow: `dg/d`.
- Run: `run-1792661696858002000-2`.
- Approval: `approval/run-1792661696858002000-2/planning-approved`.
- Approval presentation: `presentation/ec00b7ae527e899e`.
- Approval outcome: granted under delegated-maintainer authority with
  persisted presentation proof.
- Phase status: completed.
- Event summary: 39 ordered events, one approval, zero retries, and zero
  escalations; approval-presentation proof was enforced with one persisted
  presentation record.
- Validation summary: documentation and diff checks passed; Rust checks were
  not run because the governed phase changed documentation only.
- Out-of-kernel work: repository inspection, plan and report authoring,
  validation, git, and later pull-request operations are performed by the
  delegated maintainer; Workflow OS governs scope and records the phase trail.

## 9. Recommended Next Phase

The focused maintainer/security
[re-review](TRUSTED_HOST_PROJECT_VALIDATION_PROFILE_PLAN_BLOCKER_FIX_REVIEW.md)
accepts both corrections. Implement one bounded foreground-binary vertical
slice next.

## 10. Blocker-Fix Status

The focused review's two blockers are corrected in
[Trusted-Host Project-Validation Profile Plan Blocker Fix
Report](TRUSTED_HOST_PROJECT_VALIDATION_PROFILE_PLAN_BLOCKER_FIX_REPORT.md).
The corrected plan defines one private Core-owned trusted-host result adapter
that accepts only passing checks as skill success, and an exact selector rule
with a temporary no-profile compatibility alias for docs-check only.
