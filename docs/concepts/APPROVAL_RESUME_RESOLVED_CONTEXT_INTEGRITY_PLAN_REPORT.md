# Approval Resume Resolved-Context Integrity Plan Report

## 1. Executive Summary

P0 planning is complete for the confirmed approval/resume TOCTOU boundary. The
plan requires every new approval request to carry a payload-free commitment to
the resolved execution context and requires grant paths to validate it before
any durable mutation.

No runtime fix is implemented by this planning phase.

## 2. Finding Confirmed

External testing reported that approval resume reloads current project specs
without proving they match the work originally awaiting approval. Inspection of
current `main` confirms:

- grant-side events are appended before project reload;
- resume rebuilds from current workflow, skill, and policy files;
- only `step_id` is used to relocate the approved step;
- the rebuilt workflow hash is not compared before mutation;
- skill and policy content hashes are not bound to approval;
- transient hook/checkpoint and SideEffect request inputs are replaced by
  defaults.

The issue remains current even though the external evaluator referenced an
older pinned commit.

## 3. Planned Fix

- Add a versioned resolved execution-context hash to new approval requests.
- Commit workflow, resolved skills, referenced policies, and resume-sensitive
  request posture.
- Rebuild and compare a candidate plan before `ApprovalGranted`.
- Leave durable state unchanged on mismatch.
- Fail closed for legacy missing commitments on grant.
- Preserve denial behavior.

## 4. Explicit Non-Scope

No raw bundle persistence, handler attestation, provider write, CLI, schema,
hosted runtime, RBAC, automatic approval, distributed replay, or release change
is authorized.

## 5. Governed Evidence

- Workflow: `dg/d`.
- Run: `run-1783871247582345000-2`.
- Approval:
  `approval/run-1783871247582345000-2/planning-approved`.
- Presentation: `presentation/c4361eccbe68e363`.
- Approval outcome: granted through proof-enforced presentation.
- Status: completed.
- Event summary: 39 events, one approval request, one proof-marked approval
  grant, eight policy decisions, six scheduled steps, six successful mock skill
  invocations, no retries, and no escalations.

## 6. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.

Rust checks were not run because this phase changed documentation only.

## 7. Out-Of-Kernel Work

Codex inspected current code and authored the plan and roadmap updates. The
kernel governed scope and approval but did not inspect code, edit files, run
checks, create a WorkReport, or persist an artifact.

No git, PR, provider, or external-write action occurred.

## 8. Recommended Next Phase

Perform focused maintainer review, then implement the P0 fix before returning to
ordinary immutable run-bundle planning.
