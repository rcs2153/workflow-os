# Governed Phase Approval Handoff Preservation Bug

Status: Fixed. The original bug is preserved here as the problem statement and fix-forward record.

## Summary

The governed phase runner correctly emits a structured `approval_handoff` block, but the executor/agent can still collapse that block into vague user-facing prose.

This was observed during a governed `dg/review` phase after the prior handoff-context fix. The runner printed the complete block, including workflow ID, run ID, approval ID, approval reason, allowed scope, disallowed scope, next action, redaction note, approval command, and agent instruction. The agent then ended the turn with a compressed final message:

```text
Waiting on your explicit approval for the governed review phase before I proceed.
```

That final message did not preserve the approval handoff details. The kernel/helper output was correct; the failure was at the agent-facing relay boundary.

## Why This Matters

Approval checkpoints are one of the most important dogfood surfaces for Workflow OS. The maintainer must be able to see exactly what is being approved, what is not being approved, and what happens next.

If the agent can summarize away the handoff block, the governance boundary becomes less inspectable even when runtime state is correct.

This is a blocker because the project has already fixed approval handoff emission. The remaining gap is preserving that handoff in the user-facing approval request.

## Evidence

The runner emits the required block:

- `scripts/self-governed-benchmark.mjs` prints `approval_handoff_required: true`.
- `scripts/self-governed-benchmark.mjs` prints `approval_handoff`.
- `scripts/self-governed-benchmark.mjs` prints `agent_instruction: relay this complete approval_handoff block to the maintainer before asking for approval`.

The prior fix review accepted the runner-side fix:

- [Governed Phase Approval Handoff Context Blocker Fix Review](GOVERNED_PHASE_APPROVAL_HANDOFF_CONTEXT_BLOCKER_FIX_REVIEW.md)

The runbook already states the intent:

- [Self-Governed Build Benchmark](../user-guide/self-governed-build-benchmark.md)

The missed behavior shows that helper output and runbook text are not enough. The repo-level agent instructions and future enforcement checks must make handoff preservation mandatory.

## Root Cause

The previous fix solved the helper output problem, not the agent preservation problem.

Current state:

- the runner emits a complete `approval_handoff` block;
- tests prove the helper output shape;
- documentation says agents must relay it;
- `AGENTS.md` did not explicitly require approval pause responses to include the complete block;
- no checker verifies that a user-facing approval request preserved the emitted block.

The bug is therefore at the approval handoff preservation boundary:

```text
kernel emits complete handoff -> agent receives complete handoff -> agent may still summarize it away
```

## Fix Implemented

The fix makes approval handoff preservation explicit in the repo-level agent instructions and pins that instruction with a focused regression test.

Implemented behavior:

- `AGENTS.md` requires agents to preserve and present the complete `approval_handoff` block when `phase-start` emits `approval_handoff_required: true`.
- `AGENTS.md` explicitly prohibits replacing the handoff with vague prose such as "waiting for approval."
- `AGENTS.md` requires any final response while waiting for approval to include the complete handoff block or a verbatim copy-safe equivalent.
- `scripts/self-governed-benchmark.test.mjs` now verifies the repo-level agent instructions keep the preservation rule.
- The self-governed build benchmark runbook now distinguishes runner-side handoff emission from agent-side handoff preservation.

## Required Behavior

Approval handoff preservation must remain explicit and hard to miss.

Required behavior:

- Any agent-facing approval request after `phase-start` emits `approval_handoff_required: true` must include the complete `approval_handoff` block.
- The approval request must not be replaced by generic prose such as "waiting for approval".
- If the agent sends a final message while waiting for approval, that final message must include the complete handoff block or a verbatim copy-safe equivalent.
- The agent must not omit workflow ID, run ID, approval ID, status, approval reason, allowed scope, disallowed scope, next action, redaction note, or approval command posture.
- The agent may add a short human-readable sentence before the block, but the block itself must remain intact.

## Acceptance Criteria

- `AGENTS.md` contains a mandatory approval handoff preservation rule.
- The self-governed build benchmark runbook keeps the same rule.
- A focused regression check exists for the repo-local helper/instructions boundary, if feasible.
- Future governed phase reports disclose whether the approval handoff was preserved.
- The fix does not change runtime approval semantics.
- The fix does not approve automatically.
- The fix does not add hidden approvals, repository actions, git/PR automation, shell execution, file writes, persistence, artifacts, schemas, side effects, writes, or release posture changes.

## Current Required Manual Behavior

Until a stronger checker exists, agents working in this repository must preserve the handoff manually.

When `phase-start` prints:

```text
approval_handoff_required: true
approval_handoff:
  ...
```

the user-facing approval request must include:

```text
Governed approval required

approval_handoff:
  workflow_id: <workflow_id>
  phase: <phase>
  run_id: <run_id>
  approval_id: <approval_id>
  status: <status>
  approval_reason: <approval_reason>
  approval_allows: <allowed scope>
  approval_does_not_allow: <disallowed scope>
  next_action_after_approval: <next action>
  redaction_note: <redaction note>
  approval_command: <redacted approval command, when present>
  agent_instruction: relay this complete approval_handoff block to the maintainer before asking for approval
```

## Recommended Next Phase

Recommended next phase: governed phase approval handoff preservation blocker fix.

That phase should update repo-level agent instructions, add focused validation where feasible, and create a blocker-fix report/review before returning to feature roadmap work.
