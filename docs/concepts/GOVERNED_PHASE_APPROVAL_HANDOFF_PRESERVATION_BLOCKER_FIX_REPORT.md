# Governed Phase Approval Handoff Preservation Blocker Fix Report

## 1. Executive Summary

The governed phase approval handoff preservation blocker is fixed.

The prior runner-side fix made `phase-start` emit a complete `approval_handoff` block, but a later dogfood approval pause showed that an agent could still collapse the emitted block into vague prose in the user-facing approval request. This phase fixes the preservation boundary by making the complete handoff block mandatory in repo-level agent instructions and adding a focused regression check for that instruction.

## 2. Blocker Fixed

Original blocker:

- the kernel/helper emitted a complete `approval_handoff` block;
- the agent received the block;
- the final approval request still collapsed the checkpoint into generic prose;
- the maintainer could not see the workflow ID, run ID, approval ID, allowed scope, disallowed scope, or next action in the final approval request.

The fix treats that as an agent-relay preservation bug, not as a runtime approval-state bug.

## 3. Scope Completed

- Added a mandatory approval handoff preservation rule to `AGENTS.md`.
- Updated the self-governed build benchmark runbook to distinguish handoff emission from handoff preservation.
- Updated [Governed Phase Approval Handoff Preservation Bug](GOVERNED_PHASE_APPROVAL_HANDOFF_PRESERVATION_BUG.md) from open blocker to fixed fix-forward record.
- Added a focused helper test proving repo-level agent instructions require preserving complete `approval_handoff` blocks and prohibit vague replacement prose.

## 4. Scope Explicitly Not Completed

This phase did not implement:

- runtime approval semantic changes;
- automatic approval;
- hidden approval;
- repository actions performed by the kernel;
- git or PR automation;
- local command execution by the kernel;
- persistence;
- report artifacts;
- schema changes;
- side-effect modeling;
- writes;
- release posture changes.

## 5. Implementation Approach

The fix is intentionally small.

The runner already emits the correct structured block. The remaining gap was the executor/agent instruction boundary. The fix therefore updates the repository's primary agent instruction file:

```text
When phase-start emits approval_handoff_required: true, preserve and present the complete approval_handoff block...
```

The new regression test reads `AGENTS.md` and verifies the preservation requirements stay present:

- `approval_handoff_required: true`;
- complete `approval_handoff` block;
- no vague prose replacement;
- final response includes the complete handoff block.

## 6. Governance And Dogfood Summary

This blocker fix was governed by the local Workflow OS dogfood runner.

- workflow phase: blocker
- workflow ID: `dg/blocker`
- run ID: `run-1783141866574149000-2`
- approval ID: `approval/run-1783141866574149000-2/fix-approved`
- approval outcome: approved by the maintainer before blocker-fix work continued
- event summary: completed terminal run with 39 events, 1 approval, 0 retries, and 0 escalations
- event kinds: `ApprovalGranted:1`, `ApprovalRequested:1`, `PolicyDecisionRecorded:8`, `RunCompleted:1`, `RunCreated:1`, `RunResumed:1`, `RunStarted:1`, `RunValidated:1`, `SkillInvocationRequested:6`, `SkillInvocationStarted:6`, `SkillInvocationSucceeded:6`, `StepScheduled:6`

The approval request preserved the full emitted `approval_handoff` block in the user-facing final response before approval was granted.

The dogfood runner coordinated governance only. Repository edits, validation commands, and reporting were performed outside the kernel by the executor.

## 7. Validation Summary

Validation commands run:

- `npm run test:dogfood-helper` passed.
- `npm run check:docs` passed.
- `git diff --check` passed.

Additional validation remains to be run before commit/PR if this blocker fix is bundled with Rust implementation changes.

## 8. Remaining Known Limitations

- The preservation rule is enforced through repo-level instructions and a focused instruction-presence regression test.
- There is not yet a typed core approval handoff model.
- There is not yet a general agent-response linter that can inspect actual assistant final messages.
- The runner still depends on agents following the emitted instruction, but the instruction is now explicit at the repository entry point and test-covered.

## 9. Recommended Next Phase

Recommended next phase: governed phase approval handoff preservation blocker fix review.

Why: approval checkpoint presentation is a governance-critical dogfood surface. The fix should be reviewed before returning to workflow-declared high-assurance artifact requirement executor integration review or broader feature roadmap work.
