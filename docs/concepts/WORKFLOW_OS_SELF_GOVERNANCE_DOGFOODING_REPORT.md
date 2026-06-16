# Workflow OS Self-Governance Dogfooding Report

Report date: 2026-06-14

## 1. Executive Summary

Workflow OS has begun dogfooding its own local kernel.

This first dogfood slice adds a dedicated local Workflow OS project that uses the kernel as a governance wrapper for Workflow OS planning/docs work. The workflow validates as normal project specs, runs through the local executor, pauses for approval, resumes after approval, and leaves inspectable event history.

This is **kernel-governed, Codex-executed** dogfooding. Codex or a human still performs repository edits and validation outside the kernel.

## 2. Scope Completed

- Added `dogfood/workflow-os-self-governance`.
- Added a local project manifest, workflow spec, skill spec, and approval policy.
- Added a dogfood project README with validation, run, approve, and inspect commands.
- Updated public docs and roadmap status to describe the first dogfood slice.
- Kept the dogfood workflow single-step, local, approval-gated, and fixture/mock explicit.

## 3. Scope Explicitly Not Completed

- No real build-command execution skills.
- No arbitrary shell-command skills.
- No multi-step workflow execution.
- No branching.
- No automatic Codex control through the kernel.
- No recursive agents, agent swarms, or generic agent orchestration.
- No write-capable adapters.
- No side-effect boundary modeling.
- No automatic runtime report generation.
- No CLI report rendering or export.
- No workflow schema changes.
- No production self-hosting claim.
- No hosted or distributed runtime claim.
- No Level 3 or Level 4 autonomy.

## 4. Dogfood Project Summary

The dogfood project lives at:

```text
dogfood/workflow-os-self-governance/
```

It defines:

- project ID: `dogfood/workflow-os-self-governance`;
- workflow ID: `dg/d`;
- skill ID: `local/d`;
- policy ID: `approval/d`.

The workflow represents a planning/docs governance step for Workflow OS work. It is deliberately generic and does not encode a specific GitHub, Jira, CI, or code-editing workflow.

## 5. Kernel Governance Behavior

The local kernel governs the dogfood run by:

- loading and validating the project specs;
- creating a workflow run;
- recording immutable run identity;
- enforcing policy before the local skill invocation;
- pausing for approval;
- recording approval and resume events;
- completing the run through the existing single-step local executor path;
- preserving event history for inspection.

The kernel does not perform repository edits or validation commands.

## 6. Approval Behavior

The dogfood workflow is Level 2 and approval-gated.

Expected behavior:

1. `run dg/d` enters `WaitingForApproval`.
2. The operator approves with actor and reason.
3. The run resumes and completes.
4. `inspect` shows event history through `RunCompleted`.

## 7. Privacy/Security Posture

The dogfood workflow uses only non-secret literals:

- `workflow-os-planning-docs-task`;
- `kernel-governed-codex-executed`.

It does not read live provider data, copy raw payloads, use secrets, call external systems, or perform writes. It uses the explicit `--mock-all-local-skills` path, which remains a deterministic local preview mechanism and not a production skill plugin system.

## 8. Commands Run And Results

Dogfood kernel commands:

- `target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance validate`
  - Passed with expected `validation.lifecycle.experimental` warnings.
- `target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance --state-dir /tmp/workflow-os-self-governance-state.Nq1NUv --mock-all-local-skills run dg/d`
  - Created `run-1781492263903333000-2`.
  - Entered `WaitingForApproval`.
  - Created approval `approval/run-1781492263903333000-2/d`.
- `target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance --state-dir /tmp/workflow-os-self-governance-state.Nq1NUv --mock-all-local-skills approve run-1781492263903333000-2 approval/run-1781492263903333000-2/d --actor user/dogfood-reviewer --reason reviewed-governance-task`
  - Granted approval and completed the run.
- `target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance --state-dir /tmp/workflow-os-self-governance-state.Nq1NUv inspect run-1781492263903333000-2`
  - Confirmed `Completed` status and event history through `RunCompleted`.

Repository validation commands:

- `cargo fmt --all --check`
  - Passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Passed.
- `cargo test --workspace`
  - Passed.
- `npm run check:docs`
  - Passed.

## 9. Limitations

- The first dogfood workflow governs planning/docs work only.
- The workflow uses explicit deterministic mock handling.
- No real local build/check skill handler exists.
- The kernel does not control Codex directly.
- The workflow was kept single-step for the initial dogfood slice; the local executor now has a separate sequential multi-step slice pending review.
- The dogfood project is not production self-hosting.
- Approval resume idempotency keys currently include runtime identifiers, so dogfood runtime IDs are intentionally compact.

## 10. Recommended Next Phase

Recommended next phase: **self-governed validation/check planning**.

Status: documented in [Self-Governed Validation/Check Plan](../implementation-plans/self-governed-validation-check-plan.md).

The first dogfood slice proves the kernel can govern a Workflow OS planning/docs workflow. The next planning phase should decide how to introduce real local validation/check skill handlers safely, including command allowlists, side-effect boundaries, output capture limits, evidence/report integration, and failure semantics.
