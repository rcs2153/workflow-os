# Governed Phase Approval Final Request Preservation Bug

Status: Fixed. The original bug is preserved here as the problem statement and fix-forward record.

## Summary

A governed phase can still lose approval context if the agent relays the complete `approval_handoff` block in an intermediate update but ends the turn with a vague final approval request.

This happened during a governed planning phase after the runner emitted a complete handoff and after the agent displayed it in commentary. The final response still collapsed the approval pause into:

```text
Waiting on your explicit approval for the governed planning phase before I proceed.
```

That sentence was not enough. The final approval request is the durable user-facing ask, so it must preserve the full approval context.

## Why This Matters

Workflow OS dogfooding depends on approval checkpoints being visible, specific, and auditable. A complete handoff in terminal output or commentary is not sufficient if the final approval request hides:

- workflow ID;
- phase;
- run ID;
- approval ID;
- approval reason;
- concrete work summary;
- approved scope;
- strict non-goals;
- touched surfaces;
- validation expectations;
- why-now rationale;
- next action after approval;
- approval command posture.

The failure makes approval look like ordinary conversational permission instead of a governed checkpoint.

## Root Cause

Previous fixes addressed:

- runner-side handoff emission;
- agent instruction preservation;
- bounded work-summary context.

They did not create a dedicated copy-safe final approval request surface. The kernel helper emitted the data, but the agent still had to manually compose the final approval request and could degrade it.

The vulnerable path was:

```text
phase-start emits approval_handoff
agent includes the block in commentary
agent final response asks for approval using vague prose
maintainer sees an underspecified final approval ask
```

## Fix Implemented

The repo-local governed phase runner now emits a second approval artifact:

```text
copy_safe_approval_request_required: true
copy_safe_approval_request:
  begin: |
    Governed approval required before proceeding.
    ...
  end: copy_safe_approval_request
```

This block is formatted as the exact request an agent should use when the turn ends waiting for approval.

Implemented behavior:

- `phase-start` still emits `approval_handoff_required: true`.
- `phase-start` now emits `copy_safe_approval_request_required: true`.
- The copy-safe request repeats the governed approval handoff inside a fenced YAML block.
- The copy-safe request explicitly instructs agents not to replace it with vague prose.
- `AGENTS.md` requires agents to use the copy-safe request in the final approval request.
- Focused helper tests assert the copy-safe request is emitted and that repo instructions require it.

## Required Behavior

When `phase-start` emits `copy_safe_approval_request_required: true`, the final user-facing approval request must include the emitted copy-safe approval request or a verbatim copy-safe equivalent.

Commentary containing the handoff is not enough if the final response collapses the approval into vague prose.

## Non-Goals

This fix does not implement:

- runtime approval semantic changes;
- automatic approval;
- hidden approval;
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

## Validation

Required validation for the fix:

- `npm run test:dogfood-helper`
- `npm run check:docs`
- `git diff --check`

## Recommended Next Phase

Recommended next phase: governed phase approval final request preservation blocker fix review.

That review should verify that the helper output, repo instructions, runbook, and regression tests now protect the final approval request surface rather than only the terminal/commentary handoff.
