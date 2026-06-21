# PR Hygiene Dogfood Workflow Report

## 1. Executive summary

The PR hygiene dogfood workflow is implemented as a bounded Workflow OS workflow in the self-governance dogfood project. It governs the repository conflict-prevention and PR-readiness loop while leaving git operations, GitHub operations, edits, validation commands, and PR creation outside the kernel.

## 2. Scope completed

- Added `dg/pr`, a local approval-gated dogfood workflow for PR hygiene.
- Added checkpoints for main-sync disclosure, hot-file conflict-risk scoping, PR-scope approval, implementation handoff, validation disclosure, conflict-resolution disclosure, and PR-readiness reporting.
- Updated dogfood and user-facing docs to describe when and how to use the workflow.
- Updated roadmap state to reflect the workflow as implemented dogfood governance.

## 3. Scope explicitly not completed

- No git automation.
- No automatic merge, rebase, push, branch creation, or conflict resolution.
- No GitHub API or `gh` integration.
- No PR creation or PR inspection from inside the kernel.
- No repository mutation by the workflow.
- No automatic local check execution.
- No default local check handler registration.
- No report artifact writing.
- No persistence beyond normal local workflow state.
- No recursive agents, agent swarms, hosted behavior, write-capable adapters, or Level 3/4 autonomy.

## 4. Workflow summary

The new workflow is `dg/pr` in the self-governance dogfood project. It is a sequential, manual-triggered, Level 2 workflow with approval policy checkpoints. The workflow records that PR preparation should include:

- integration of current `main` before phase or PR work;
- explicit conflict-risk scoping for hot files;
- approval before branch work or PR staging;
- disclosure that implementation and git commands were executed outside the kernel;
- validation disclosure before PR handoff;
- conflict-resolution disclosure before push;
- final PR-readiness reporting.

## 5. Governance boundary

The workflow governs the handoff, not the repository machinery. It intentionally avoids pretending Workflow OS can perform git operations or inspect GitHub state automatically. Agents and humans remain responsible for executing commands and reporting results honestly.

## 6. Validation and test summary

The new workflow is covered by dogfood project validation. Dedicated `.test.yml` workflow tests remain deferred because the dogfood test semantics are not yet scoped. Existing runtime and executor tests continue to cover the underlying multi-step approval and report-bearing behavior.

## 7. Commands run and results

- `npm run dogfood:benchmark -- validate --no-build` passed.
- `npm run check:docs` passed.

## 8. Remaining limitations

- The repo-local helper still defaults to the main dogfood workflow, `dg/d`.
- `dg/pr` is run through the generic CLI path.
- The workflow does not prove GitHub mergeability or CI status; the agent or maintainer must inspect and disclose those externally.
- Conflict prevention still depends on human/agent execution of git commands outside the kernel.

## 9. Recommended next phase

PR hygiene dogfood workflow review.

The workflow is intentionally small, but it affects the project operating model. A focused review should confirm the boundary is honest and that it does not overclaim automation.
