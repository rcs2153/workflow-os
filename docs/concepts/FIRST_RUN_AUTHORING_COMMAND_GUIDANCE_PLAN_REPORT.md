# First-Run Authoring Command Guidance Plan Report

## 1. Executive Summary

This planning phase documents a small P0 UX bridge from `workflow-os first-run`
recommendations to the existing non-mutating workflow authoring surfaces.

Real-repository testing showed that `first-run` is useful, but users still have
to infer the next commands for inspecting a recommendation and previewing a
draft workflow. The plan recommends showing bounded, copyable command guidance
from default first-run output while preserving the current review-only,
non-mutating boundary.

## 2. Scope Completed

- Created
  `docs/implementation-plans/first-run-authoring-command-guidance-plan.md`.
- Updated `ROADMAP.md` to link the planned command-guidance phase.
- Kept the plan limited to user-facing guidance for existing non-mutating
  commands.
- Captured explicit non-goals around workflow generation, file writes,
  promotion, runtime execution, provider calls, local checks, schemas, examples,
  hosted behavior, writes, and release posture changes.

## 3. Scope Not Completed

This phase did not implement:

- command guidance in `workflow-os first-run`;
- JSON output changes;
- workflow generation;
- draft file writing by default;
- workflow promotion;
- runtime execution;
- local check execution;
- provider calls;
- schema changes;
- examples;
- hosted or distributed behavior;
- write-capable adapters.

## 4. Plan Summary

The plan recommends selecting one existing first-run recommendation and showing
two non-mutating commands:

```sh
workflow-os first-run --recommendation <id>
workflow-os author workflow --from-recommendation <id> --dry-run
```

The selected recommendation must already exist in the computed recommendation
set. The implementation must not fabricate recommendation IDs or suggest
commands that write, promote, run, approve, execute checks, call providers, or
infer shell commands from repository metadata.

## 5. Dogfood Governance

- Workflow: `dg/planning`
- Run ID: `run-1783737524091214000-2`
- Approval ID: `approval/run-1783737524091214000-2/planning-approved`
- Approval presentation ID: `presentation/8fd2615c8b343d39`
- Approval presentation hash:
  `8fd2615c8b343d39e069d292602466644323ffcdb888b6a0d489d7f8cfd5ccd8`
- Approval outcome: delegated maintainer approved.

## 6. Validation

Planned validation for this phase:

```sh
npm run check:docs
git diff --check
```

## 7. Remaining Limitations

- The default `first-run` output still does not show the command guidance until
  the implementation phase lands.
- Preview JSON behavior remains an open implementation decision.
- The plan intentionally does not make authoring automatic.

## 8. Recommended Next Phase

Proceed to first-run authoring command guidance implementation.

The implementation should add the smallest bounded output change: show the
recommendation detail command and authoring dry-run command for one existing
recommendation, with focused tests proving the guidance remains non-mutating and
does not suggest file writes, promotion, workflow execution, provider calls,
local checks, or unsupported automation.
