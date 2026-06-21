# Dogfood Workflow Suite Phase 1 Report

## 1. Executive summary

Dogfood Workflow Suite Phase 1 is implemented. Workflow OS now has separate self-governance workflows for accepted implementation phases, phase-level maintainer reviews, and PR hygiene/conflict avoidance. The suite is designed to make the local kernel govern the recurring shapes of Workflow OS development while Codex or a human still executes repository edits, validation commands, git operations, and PR actions outside the kernel.

## 2. Scope completed

- Added `dg/implement` for bounded implementation phases.
- Added `dg/review` for maintainer reviews and blocker-fix reviews.
- Kept `dg/pr` as the PR hygiene/conflict-avoidance workflow.
- Documented when to use each workflow in the dogfood README and self-governed build benchmark guide.
- Updated the agent harness quickstart with direct commands for implementation and review workflows.
- Updated roadmap language to describe the dogfood workflow suite.

## 3. Scope explicitly not completed

- No automatic workflow generation.
- No automatic workflow selection.
- No automatic local check execution.
- No arbitrary shell execution.
- No git automation.
- No GitHub inspection, PR creation, branch creation, merge, rebase, or push behavior.
- No report artifact writing.
- No workflow schema changes.
- No hosted or collaboration registry behavior.
- No recursive agents, agent swarms, write-capable adapters, or Level 3/4 autonomy.

## 4. Workflow API summary

The implemented dogfood workflows are:

- `dg/d`: legacy planning/docs benchmark workflow.
- `dg/implement`: implementation-phase workflow governing scope confirmation, context readiness, implementation approval, implementation handoff, validation disclosure, and final implementation report posture.
- `dg/review`: maintainer-review workflow governing review context, review approval, scope verification, validation assessment, findings classification, and review report posture.
- `dg/pr`: PR hygiene workflow governing main-sync disclosure, hot-file conflict-risk scoping, PR-scope approval, implementation handoff, validation disclosure, conflict-resolution disclosure, and PR-readiness reporting.

All workflows are local, manual-triggered, Level 2, approval-gated, and compatible with `--mock-all-local-skills`.

## 5. Governance boundary

The suite governs the lifecycle of Workflow OS work. It does not execute the work. Codex, Claude Code, or a human remains responsible for reading context, changing files, running commands, resolving conflicts, pushing branches, opening PRs, and writing reports.

The core operating rule remains:

```text
Agent executes. Workflow OS governs.
```

## 6. Validation and test summary

The new workflows are covered by dogfood project validation. Dedicated `.test.yml` specs for dogfood workflows remain deferred until test execution semantics are scoped. Existing executor tests continue to cover the underlying sequential multi-step, approval, cancellation, and report-bearing behavior.

## 7. Commands run and results

- `npm run dogfood:benchmark -- validate --no-build` passed. The dogfood project is valid; the new workflows emit only expected experimental lifecycle warnings.
- `npm run check:docs` passed.

## 8. Remaining limitations

- The repo-local `npm run dogfood:benchmark` helper still defaults to `dg/d`.
- `dg/implement`, `dg/review`, and `dg/pr` currently run through the generic CLI path.
- Workflow selection is manual.
- The suite does not yet include `dg/workflow-discovery`, `dg/security`, `dg/release`, or `dg/runtime-composition`.
- The workflows do not yet produce automatic WorkReports or report artifacts.

## 9. Recommended next phase

Dogfood Workflow Suite Phase 1 review.

After review, the next implementation phase should either wire the repo-local helper to select dogfood workflow IDs explicitly or add the recommendation-only `dg/workflow-discovery` workflow so the kernel begins governing which dogfood workflows should exist next.
