# First-Run Recommendation Detail Planning Report

## 1. Executive Summary

This planning phase defines a bounded future surface for inspecting individual `workflow-os first-run` recommendations.

The plan keeps recommendations review-only. It does not authorize automatic workflow generation, workflow registration, command execution, local check execution, provider calls, source-content inspection, schemas, examples, writes, hosted behavior, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes.

## 2. Scope Completed

- Created [First-Run Recommendation Detail Plan](../implementation-plans/first-run-recommendation-detail-plan.md).
- Updated [Roadmap](../../ROADMAP.md) to link the planned detail surface after the accepted recommendation next-action phase.
- Updated [first-run CLI documentation](../cli/first-run.md) to state that recommendation detail output is planned, not implemented.
- Preserved the existing `first-run` boundary: local, deterministic, report-ready context; no runtime state; no command execution; no workflow generation.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- a new CLI command or flag;
- workflow generation;
- workflow registration;
- local check registration;
- command execution;
- provider calls;
- source-content inspection;
- report artifacts;
- persistence;
- schema changes;
- examples;
- side-effect execution;
- writes;
- hosted behavior;
- reasoning lineage;
- recursive agents, agent swarms, or Level 3/4 autonomy.

## 4. Plan Summary

The plan recommends a small future implementation slice that exposes a bounded detail view for already-computed first-run recommendations.

The detail view should explain:

- recommendation id, kind, status, and review-only posture;
- rationale codes and safe metadata signals;
- ownership/escalation dependencies;
- relevant spec-field posture codes;
- suggested next action;
- what must be authored or reviewed before the recommendation can become active;
- what Workflow OS did not do.

The recommended first implementation shape is a filter-style detail view attached to `workflow-os first-run`, such as `workflow-os first-run --recommendation <id>`, if that fits current CLI conventions.

## 5. Governed Dogfood Summary

- Workflow: `dg/d`.
- Run ID: `run-1783392902451366000-2`.
- Approval ID: `approval/run-1783392902451366000-2/planning-approved`.
- Approval outcome: granted by delegated maintainer after full approval handoff review.
- Scope: first-run recommendation detail planning.

## 6. Validation Commands Run

- `npm run check:docs`: passed.
- `git diff --check`: passed.
- `npm run dogfood:benchmark -- phase-close run-1783392902451366000-2 --phase planning`: passed with 39 events, 1 approval, 0 retries, and 0 escalations.

## 7. Remaining Limitations

- No recommendation detail CLI surface exists yet.
- No detail JSON shape exists yet.
- Recommendation ids remain preview-era CLI vocabulary.
- Automatic governed workflow authoring remains unplanned and unimplemented.

## 8. Recommended Next Phase

Recommended next phase: first-run recommendation detail implementation.

The implementation should remain local, read-only, and non-mutating. It should reuse existing first-run validation and safe metadata collection, expose detail for already-computed recommendations, and preserve review-only semantics.
