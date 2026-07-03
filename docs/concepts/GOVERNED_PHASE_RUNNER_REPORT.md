# Governed Phase Runner Report

## 1. Executive Summary

The repo-local dogfood helper now includes a governed phase runner for material Workflow OS roadmap work.

The runner makes dogfooding the default entry point for implementation, review, blocker, PR hygiene, release, runtime-composition, workflow-discovery, and related phases. It validates the dogfood project, starts the mapped `dg/*` workflow, displays the real `run_id`, `approval_id`, status, approval posture, and next action, and closes a phase by summarizing the governed event trail.

The runner coordinates governance only. Codex, Claude Code, or a human remains the executor.

## 2. Scope Completed

- Added `phase-start` to `npm run dogfood:benchmark`.
- Added `phase-close` to `npm run dogfood:benchmark`.
- Added phase-to-workflow mappings for:
  - `planning`
  - `docs`
  - `implementation`
  - `review`
  - `blocker`
  - `pr`
  - `release`
  - `runtime-composition`
  - `branch-cleanup`
  - `workflow-discovery`
  - `spec-field-operationalization`
- `phase-start` validates the dogfood project before starting the governed run.
- `phase-start` starts the mapped `dg/*` workflow with deterministic mock local skills.
- `phase-start` prints `run_id`, `approval_id`, status, approval outcome, and the approval command.
- `phase-start` does not run the approval command.
- `phase-close` reads status and inspect output for the run.
- `phase-close` summarizes event count, approval count, retry count, escalation count, event kinds, terminal status, and required phase-report disclosures.
- Added focused helper tests.
- Updated roadmap, dogfood README, self-governed build benchmark guide, and root agent instructions.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- hidden approvals;
- automatic approvals;
- repo edits from inside the runner;
- git operations;
- PR creation;
- shell command execution beyond wrapping existing local Workflow OS CLI commands;
- local check execution;
- report artifact writing;
- WorkReport rendering;
- state mutation outside normal Workflow OS run/approval commands;
- workflow schema changes;
- runtime hook broadening;
- write-capable adapters;
- recursive agents;
- agent swarms;
- hosted execution;
- production self-hosting;
- Level 3/4 autonomy.

## 4. Helper API Summary

Start a governed phase:

```sh
npm run dogfood:benchmark -- phase-start --phase implementation
```

Close and summarize a governed phase:

```sh
npm run dogfood:benchmark -- phase-close <run-id> --phase implementation
```

The phase runner is repo-local development tooling. It is not a stable public CLI surface.

## 5. Phase Mapping Summary

| Phase | Workflow |
| --- | --- |
| `planning` | `dg/d` |
| `docs` | `dg/d` |
| `implementation` | `dg/implement` |
| `review` | `dg/review` |
| `blocker` | `dg/blocker` |
| `pr` | `dg/pr` |
| `release` | `dg/release` |
| `runtime-composition` | `dg/runtime-composition` |
| `branch-cleanup` | `dg/branch-cleanup` |
| `workflow-discovery` | `dg/workflow-discovery` |
| `spec-field-operationalization` | `dg/spec-field-operationalization` |

## 6. Approval Boundary Summary

Approval remains explicit and human-controlled.

The runner prints the approval command after a governed run pauses, but it does not execute that command. The approving human or agent must intentionally run the approval command with a bounded reason.

## 7. Event Trail Summary

This implementation phase was governed by:

- Workflow ID: `dg/implement`
- Run ID: `run-1783050656112397000-2`
- Approval ID: `approval/run-1783050656112397000-2/implementation-approved`
- Approval outcome: granted

The repository edits, tests, and documentation updates were performed outside the kernel by Codex. The kernel governed the phase boundary and recorded the run/approval event trail.

## 8. Validation Summary

Tests added or updated:

- phase-start command mapping;
- phase-start dry-run approval boundary;
- unsupported phase non-leakage;
- phase-close dry-run command posture;
- commands output including phase runner posture.
- root `npm run check` now includes `check:dogfood`, which runs the focused helper test suite.

Commands run:

- `npm run dogfood:benchmark -- validate --no-build` - passed with expected experimental lifecycle warnings.
- `target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance --state-dir /private/tmp/workflow-os-governed-phase-runner-state --mock-all-local-skills run dg/implement` - paused for approval.
- `target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance --state-dir /private/tmp/workflow-os-governed-phase-runner-state --mock-all-local-skills approve run-1783050656112397000-2 approval/run-1783050656112397000-2/implementation-approved --actor user/maintainer --reason approved-governed-phase-runner-p0` - completed.
- `npm run test:dogfood-helper` - passed.
- `npm run dogfood:benchmark -- phase-close run-1783050656112397000-2 --phase implementation --state-dir /private/tmp/workflow-os-governed-phase-runner-state --no-build` - passed and produced a bounded event summary.
- `npm run check:docs` - passed.
- `npm run check` - passed, including docs, dogfood-helper tests, TypeScript checks, SDK tests, and contract checks.
- `cargo fmt --all --check` - passed.

Full validation results are reported in the final implementation response.

## 9. Remaining Known Limitations

- The helper uses the local CLI's experimental JSON output for phase-close parsing.
- The helper is repo-local development tooling, not a stable public product API.
- The helper does not generate WorkReport artifacts.
- The helper does not execute real local checks.
- The helper does not automatically attach validation/check evidence.
- The helper does not prove production self-hosting.

## 10. Recommended Next Phase

Recommended next phase: **governed phase runner review**.

The review should verify the runner preserves explicit approval, avoids hidden execution, maps phase types correctly, produces useful close summaries, and keeps the dogfood/product boundary honest.
