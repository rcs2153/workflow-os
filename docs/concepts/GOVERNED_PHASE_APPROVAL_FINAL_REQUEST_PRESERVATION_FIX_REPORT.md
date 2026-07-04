# Governed Phase Approval Final Request Preservation Fix Report

## 1. Executive Summary

The governed phase approval final-request preservation blocker is fixed.

The repo-local governed phase runner already emitted complete approval handoff data, but a repeated dogfood failure showed that an agent could display the handoff in commentary and still end the turn with a vague final approval request. The fix adds a dedicated copy-safe final approval request artifact to `phase-start`, updates repo instructions and the runbook, and adds focused regression coverage.

## 2. Blocker Fixed

Original failure:

- `phase-start` emitted a complete `approval_handoff`.
- The agent relayed that handoff in an intermediate update.
- The agent final response still said only that it was waiting for approval.
- The final approval request did not preserve the governed approval context.

Fixed behavior:

- `phase-start` now emits `copy_safe_approval_request_required: true`.
- `phase-start` now emits a `copy_safe_approval_request` block intended to be used as the final user-facing approval request.
- `AGENTS.md` requires agents to use that block when the turn ends waiting for approval.

## 3. Implementation Approach

Implemented:

- Added `printCopySafeApprovalRequest(...)` to the repo-local benchmark helper.
- Kept the existing `approval_handoff` output intact.
- Added a second copy-safe request block that repeats the handoff inside fenced YAML and includes a direct approval ask.
- Updated focused helper tests to verify copy-safe request output.
- Updated repo agent instructions to require the copy-safe final approval request.
- Updated the self-governed build benchmark runbook.
- Added a fix-forward bug record.
- Updated the roadmap.

## 4. Scope Explicitly Not Completed

This fix did not implement:

- runtime approval semantic changes;
- automatic approvals;
- hidden approvals;
- repository automation;
- git or PR automation;
- shell execution by the kernel;
- persistence;
- report artifacts;
- schemas;
- side-effect modeling;
- writes;
- hosted behavior;
- release posture changes.

## 5. Validation Boundary Summary

The helper still only coordinates governance. It does not approve the run, execute local commands through the kernel, write reports, mutate git state, or perform PR actions.

The new copy-safe request is output text only. It strengthens the agent-to-maintainer handoff boundary without changing Workflow OS runtime approval semantics.

## 6. Test Coverage Summary

Focused tests now cover:

- dry-run phase start emits `approval_handoff_required: true`;
- dry-run phase start emits `copy_safe_approval_request_required: true`;
- copy-safe request includes workflow ID, run ID, approval ID, approval reason, work summary, scope, non-goals, validation expectations, and final approval ask;
- copy-safe request redacts approval command reason;
- repo `AGENTS.md` requires preserving approval handoff blocks;
- repo `AGENTS.md` requires using the copy-safe approval request for the final approval ask;
- existing helper behavior remains intact.

## 7. Commands Run And Results

- `npm run test:dogfood-helper` - passed
- `npm run check:docs` - passed
- `git diff --check` - passed

## 8. Dogfood Governance Summary

- Dogfood workflow ID: `dg/blocker`
- Run ID: `run-1783194464325785000-2`
- Approval ID: `approval/run-1783194464325785000-2/fix-approved`
- Approval outcome: granted
- Final run status: `Completed`
- Terminal: true
- Events total: 39
- Approvals: 1
- Retries: 0
- Escalations: 0
- Event summary: `ApprovalGranted:1`, `ApprovalRequested:1`, `PolicyDecisionRecorded:8`, `RunCompleted:1`, `RunCreated:1`, `RunResumed:1`, `RunStarted:1`, `RunValidated:1`, `SkillInvocationRequested:6`, `SkillInvocationStarted:6`, `SkillInvocationSucceeded:6`, `StepScheduled:6`

Out-of-kernel work disclosed: repository file edits, helper test execution, documentation updates, and validation commands were performed by the agent outside kernel execution. No git or PR action was performed by the kernel.

## 9. Remaining Known Limitations

- The helper cannot technically force a chat agent to paste the copy-safe block; it emits a specific final-request artifact and pins the rule in repo instructions and tests.
- A future stronger enforcement layer could add a typed approval handoff model or agent response linter if this becomes product surface beyond repo-local dogfood tooling.
- Session-injected agent instructions can be stale; agents should read current `AGENTS.md` from disk before asking for governed approval.

## 10. Recommended Next Phase

Recommended next phase: governed phase approval final request preservation blocker fix review.

The review should verify helper output, tests, docs, and roadmap posture before returning to first provider write candidate planning.
