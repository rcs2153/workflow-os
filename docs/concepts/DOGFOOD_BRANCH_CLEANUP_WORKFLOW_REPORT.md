# Dogfood Branch Cleanup Workflow Report

## 1. Executive Summary

The dogfood suite now includes `dg/branch-cleanup`, a local, approval-gated Workflow OS workflow for governing merged-branch cleanup readiness.

The workflow turns branch cleanup into a governed repository hygiene path: check repo state, disclose main sync, inventory merged local and remote branches, review deletion candidates, require approval before deletion, disclose cleanup handoff, validate post-cleanup state, and report the result.

This phase does not delete branches or automate git operations. Codex or a human still performs approved repository operations outside the kernel.

## 2. Scope Completed

- Added `dogfood/workflow-os-self-governance/workflows/branch-cleanup.workflow.yml`.
- Added the `dg/branch-cleanup` workflow to the self-governance dogfood suite.
- Documented the workflow in the dogfood README.
- Documented the workflow in the self-governed build benchmark runbook.
- Documented the workflow in the agent harness quickstart.
- Updated the roadmap to include branch cleanup governance.
- Updated dogfood test documentation to include branch cleanup project validation coverage.

## 3. Scope Explicitly Not Completed

- No local branch deletion.
- No remote branch deletion.
- No remote pruning.
- No git automation.
- No GitHub inspection.
- No PR creation.
- No branch protection changes.
- No automatic cleanup recommendations.
- No runtime config.
- No schemas.
- No persistence or report artifacts.
- No CLI output beyond existing Workflow OS commands.
- No write-capable adapter behavior.
- No recursive agents, agent swarms, hosted behavior, or Level 3/4 autonomy.

## 4. Workflow Summary

`dg/branch-cleanup` is a Level 2 manual workflow with the same local/d skill and approval/d policy posture as the existing dogfood workflows.

It contains these checkpoints:

- `repo-state-readiness`
- `main-sync-disclosure`
- `merged-pr-inventory`
- `delete-candidate-review`
- `cleanup-approved`
- `cleanup-handoff`
- `post-cleanup-validation`
- `cleanup-report`

The `cleanup-approved` checkpoint requires explicit approval before a human or agent performs branch deletion outside the kernel.

## 5. Governance Boundary

The workflow governs branch cleanup decision quality. It does not perform branch cleanup itself.

The kernel records the governed path, approval checkpoint, and report posture. Codex or a human executes any approved git operation outside the kernel and must disclose the result back into the handoff/report.

## 6. Validation Summary

Validation commands run:

- `npm run dogfood:benchmark -- validate --no-build`
- `npm run check:docs`
- `target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance --state-dir /tmp/workflow-os-branch-cleanup-smoke --mock-all-local-skills run dg/branch-cleanup --run-id run/bc`
- `target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance --state-dir /tmp/workflow-os-branch-cleanup-smoke --mock-all-local-skills approve run/bc approval/run/bc/cleanup-approved --actor user/dogfood-reviewer --reason smoke-approved-branch-cleanup-workflow`

Results:

- Dogfood project validation passed with expected experimental lifecycle warnings.
- Docs check passed.
- Smoke run reached `WaitingForApproval` at `approval/run/bc/cleanup-approved`.
- Smoke approval completed the run with status `Completed`.

## 7. Privacy And Safety Posture

The workflow uses bounded literal checkpoint disclosures. It does not store raw git logs, GitHub payloads, tokens, branch protection details, credentials, or command output as evidence. Any branch inventory remains a human/agent disclosure unless a future scoped evidence or report artifact path is implemented.

## 8. Remaining Limitations

- Branch cleanup candidate discovery is manual.
- Actual branch deletion remains outside the kernel.
- Remote branch cleanup still requires explicit human/agent git operations.
- No automatic GitHub merge-state inspection is implemented.
- No branch cleanup report artifact is persisted.
- No workflow discovery integration recommends branch cleanup automatically.

## 9. Recommended Next Phase

Run `dg/branch-cleanup` before deleting merged local or remote branches. After approval, perform only the approved cleanup outside the kernel, then disclose post-cleanup branch state and skipped deletions.
