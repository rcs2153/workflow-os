# Governed Phase Approval Handoff Context Bug

Status: Fixed. The original bug is preserved here as the problem statement and fix-forward record. The repo-local governed phase runner now emits a structured `approval_handoff` block from `phase-start` so agents can relay the full approval context without compressing it into vague prose.

## Summary

During a governed `dg/review` phase, the repo-local governed phase runner correctly printed the structured approval context:

- workflow ID;
- run ID;
- approval ID;
- approval outcome;
- approval required flag;
- approval reason;
- next action;
- explicit approval command with redacted reason in the command display.

The executor then summarized the pause to the maintainer as "Waiting on the governed review approval before proceeding." That message omitted the approval ID, approval reason, workflow ID, run ID, scope, and non-scope.

This is a product bug for Workflow OS dogfooding because approval checkpoints must be visible, inspectable, and specific. If an agent can collapse a governed approval into vague prose, the kernel is not yet passing enough durable approval-handoff instructions to prevent approval-context loss.

## Impact

The current behavior can make a real governed approval feel like a generic chat permission.

Risks:

- the maintainer cannot tell what is being approved without asking a follow-up question;
- the approval reason and scope can be lost between kernel output and agent response;
- non-scope constraints may not be visible at the approval moment;
- the dogfood loop becomes less trustworthy even though the underlying run state is correct;
- future users may mistake vague agent text for the intended Workflow OS approval UX.

## Root Cause

This was not a runtime state or approval projection failure.

The `phase-start` helper already emits bounded approval fields. The gap is that the kernel/helper does not emit an explicit agent-handoff instruction block that says how the executor must present approval requests to a human.

The runner output is machine-readable enough for scripts and human-readable enough for terminal review, but it does not yet provide a required copyable approval handoff template or contract. That leaves room for the agent to compress away critical approval context.

## Required Fix

The governed phase runner should emit a bounded approval handoff block whenever a phase pauses for approval.

The block should include at least:

- workflow ID;
- run ID;
- approval ID;
- status;
- approval reason;
- what approval allows;
- what approval does not allow;
- next action after approval;
- redaction/safety note for the command.

The block should be suitable for an agent to relay directly to a maintainer without interpretation.

Recommended output shape:

```text
approval_handoff_required: true
approval_handoff:
  workflow_id: dg/review
  run_id: <run-id>
  approval_id: <approval-id>
  status: WaitingForApproval
  approval_reason: approved-review-phase
  approval_allows: proceed with the maintainer review phase only
  approval_does_not_allow: implementation fixes, runtime derivation, artifact writes, CLI behavior, schema broadening, git, or PR actions
  next_action_after_approval: run the explicit approval command, perform the review, create the review document, run validation, and close the governed phase
```

The exact format can be refined, but it must be deterministic, bounded, and easy for an agent to preserve.

## Acceptance Criteria

- `phase-start` live output includes a structured approval handoff block when approval is required.
- `phase-start --dry-run` includes the same handoff shape with placeholder run/approval IDs or an explicit not-requested posture.
- Tests prove the handoff block contains workflow ID, run ID, approval ID, status, approval reason, allowed scope, disallowed scope, and next action.
- Tests prove secret-like values are rejected or redacted and do not appear in the handoff block.
- The self-governed build benchmark runbook instructs agents to preserve the approval handoff block when asking for human approval.
- The fix does not approve automatically.
- The fix does not add hidden approvals, repo edits, git actions, PR actions, shell execution, file writes, runtime mutation, or broader workflow execution authority.

## Current Workaround

Fixed behavior: agents must relay the `approval_handoff` block emitted by `phase-start` before asking for approval.

Historical workaround before the fix: agents manually surfaced approval context from `phase-start`, `status`, and `inspect` before asking for approval.

Minimum required manual handoff:

```text
Governed approval required

Workflow: <workflow_id>
Run ID: <run_id>
Approval ID: <approval_id>
Status: <status>
Approval reason: <approval_reason>

What approval allows: <plain-English scope>
What approval does not allow: <explicit non-scope>
Next action after approval: <bounded next action>
```

## Recommended Next Phase

Recommended next phase: governed phase approval handoff instruction fix review.

The fix should be reviewed as a narrow helper/runbook/test change. It should not change runtime approval semantics or implement any new approval authority model.
